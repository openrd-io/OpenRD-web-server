use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub biz_id: String,
    pub name: String,
    pub mobile: String,
    pub email: String,
    pub mobile_verified: bool,
    pub email_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_flag: bool,
    pub password: String, // 新增的字段
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub biz_id: String,
    pub name: String,
    pub mobile: String,
    pub email: String,
    pub mobile_verified: bool,
    pub email_verified: bool,
    pub deleted_flag: bool,
    pub password: String, // 新增的字段
}

impl User {
    pub fn find_by_id(conn: &mut MysqlConnection, user_id: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users
            .filter(biz_id.eq(user_id))
            .filter(deleted_flag.eq(false))
            .first(conn)
    }

    pub fn create(conn: &mut MysqlConnection, user_dto: &UserDTO) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        let new_user = (
            biz_id.eq(Uuid::new_v4().to_string()),
            name.eq(&user_dto.name),
            mobile.eq(&user_dto.mobile),
            email.eq(&user_dto.email),
            mobile_verified.eq(user_dto.mobile_verified),
            email_verified.eq(user_dto.email_verified),
            deleted_flag.eq(false),
        );

        diesel::insert_into(users).values(new_user).execute(conn)?;

        users.order(id.desc()).first(conn)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        user_id: i32,
        user_dto: &UserDTO,
    ) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::update(users.find(user_id))
            .set((
                name.eq(&user_dto.name),
                mobile.eq(&user_dto.mobile),
                email.eq(&user_dto.email),
                mobile_verified.eq(user_dto.mobile_verified),
                email_verified.eq(user_dto.email_verified),
                updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)?;

        users.find(user_id).first(conn)
    }

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
