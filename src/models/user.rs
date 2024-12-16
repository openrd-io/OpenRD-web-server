use std::{fmt::format, hash};

use crate::{handlers::error::AppError, log_info, schema::users, utils::api_response::ApiResponse};
use actix_web::cookie::time::error;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户实体，表示数据库中的 `users` 表。
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    /// 用户的唯一标识符。
    pub id: i32,
    /// 业务关联ID，用于关联特定的业务逻辑或模块。
    pub biz_id: String,
    /// 用户的姓名。
    pub name: String,
    /// 用户的手机号码。
    pub mobile: String,
    /// 用户的电子邮件地址。
    pub email: String,
    /// 手机号码是否经过验证。
    pub mobile_verified: bool,
    /// 电子邮件是否经过验证。
    pub email_verified: bool,
    /// 记录创建时间。
    pub created_at: NaiveDateTime,
    /// 记录最后更新时间。
    pub updated_at: NaiveDateTime,
    /// 标记记录是否被删除（逻辑删除）。
    pub deleted_flag: bool,
    /// 用户的密码（**注意：在实际应用中，请确保密码的安全存储和处理**）。
    pub password: String, // 新增的字段
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub name: String,
    pub mobile: String,
    pub email: String,
    pub mobile_verified: bool,
    pub email_verified: bool,
    pub deleted_flag: bool,
    pub password: String, // 新增的字段
}

/// 用户数据传输对象，用于更新用户记录，所有字段可选。
/// 用户数据传输对象，用于更新用户记录，所有字段可选。
#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDTO {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub mobile_verified: Option<bool>,
    pub email_verified: Option<bool>,
    pub password: Option<String>, // 如果提供，则需要哈希
}

impl User {
    /// 通过业务ID查找用户，且未被逻辑删除。
    ///
    /// # 参数
    ///
    /// - `conn`：数据库连接。
    /// - `user_id`：用户的业务关联ID。
    ///
    /// # 返回
    ///
    /// 返回查询结果，包含用户信息或错误。

    pub fn find_by_id(conn: &mut MysqlConnection, user_id: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users
            .filter(biz_id.eq(user_id))
            .filter(deleted_flag.eq(false))
            .first(conn)
    }

    /// 创建一个新的用户记录，并将密码进行哈希处理。
    ///
    /// # 参数
    ///
    /// - `conn`：数据库连接。
    /// - `user_dto`：包含用户创建信息的数据传输对象。
    ///
    /// # 返回
    ///
    /// 返回创建的用户信息或错误。
    pub fn create(conn: &mut MysqlConnection, user_dto: &UserDTO) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
       
        // 判断用户名或邮箱是否已经存在，如果存在则返回错误
        if users.filter(name.eq(&user_dto.name)).or_filter(email.eq(&user_dto.email)).first::<User>(conn).is_ok() {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation
                , Box::new("用户名或邮箱已存在".to_string())
            ));
        }

        let hashed_password = bcrypt::hash(&user_dto.password, bcrypt::DEFAULT_COST).unwrap();

        let new_user = (
            biz_id.eq(format!("user_{}", Uuid::new_v4().to_string())),
            name.eq(&user_dto.name),
            mobile.eq(&user_dto.mobile),
            email.eq(&user_dto.email),
            mobile_verified.eq(user_dto.mobile_verified),
            email_verified.eq(user_dto.email_verified),
            deleted_flag.eq(false),
            password.eq(hashed_password),
        );

        if let Err(e) = diesel::insert_into(users).values(new_user).execute(conn) {
            log::error!("Failed to create user: {}", e);
            return Err(e);
        };

        users.order(id.desc()).first(conn)
    }
    /// 更新现有的用户记录，并在必要时更新密码为哈希后的密码。
    ///
    /// # 参数
    ///
    /// - `conn`：数据库连接。
    /// - `user_id`：要更新的用户的业务关联ID。
    /// - `user_dto`：包含更新信息的数据传输对象。
    ///
    /// # 返回
    ///
    /// 返回更新后的用户信息或错误。
    pub fn update(
        conn: &mut MysqlConnection,
        user_id: &str,
        update_dto: &UpdateUserDTO,
    ) -> QueryResult<User> {
        let mut update_dto = update_dto.clone();
        use crate::schema::users::dsl::*;

        // 如果提供了密码，则进行哈希处理
        if let Some(ref pwd) = update_dto.password {
            match bcrypt::hash(pwd, bcrypt::DEFAULT_COST) {
                Ok(hash) => {
                    update_dto.password = Some(hash);
                }
                Err(e) => {
                    log::error!("密码哈希失败: {}", e);
                    return Err(diesel::result::Error::NotFound);
                }
            }
        }

        // 尝试更新用户，如果失败则记录错误日志并返回错误
        if let Err(e) = diesel::update(
            users
                .filter(biz_id.eq(user_id))
                .filter(deleted_flag.eq(false)),
        )
        .set(update_dto) // 传递拥有所有权的 `UpdateUserDTO`
        .execute(conn)
        {
            log::error!("更新用户失败: {}", e);
            return Err(e);
        }

        // 返回更新后的用户
        users.filter(biz_id.eq(user_id)).first(conn)
    }
    // 删除用户（逻辑删除）。
    ///
    /// # 参数
    ///
    /// - `conn`：数据库连接。
    /// - `user_id`：要删除的用户的ID。
    ///
    /// # 返回
    ///
    /// 返回受影响的行数或错误。
    pub fn delete(conn: &mut MysqlConnection, user_id: i32) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::update(users.find(user_id))
            .set(deleted_flag.eq(true))
            .execute(conn)
    }

    pub fn list(conn: &mut MysqlConnection, page: i64, per_page: i64) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        users
            .filter(deleted_flag.eq(false))
            .order(id.desc())
            .limit(per_page)
            .offset((page - 1) * per_page)
            .load(conn)
    }
}
