use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32, // 使用 i32 以匹配 Diesel 的 Integer 类型
    pub biz_id: String,
    pub name: String,
    pub mobile: String,
    pub email: String,
    pub mobile_verified: bool,
    pub email_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_flag: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserDTO {
    pub biz_id: String,
    pub name: String,
    pub mobile: String,
    pub email: String,
    pub mobile_verified: bool,
    pub email_verified: bool,
    pub deleted_flag: bool,
}

impl User {
    pub fn get_by_id(conn: &mut MysqlConnection, user_id: i32) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.find(user_id).first(conn)
    }

    pub fn create(conn: &MysqlConnection, user_dto: &UserDTO) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user_dto)
            .execute(conn)?;
        users::order(id.desc()).first(conn)
    }

    pub fn update(conn: &MysqlConnection, user_id: i32, updated_user: &UserDTO) -> QueryResult<User> {
        diesel::update(users::find(user_id))
            .set((
                users::name.eq(&updated_user.name),
                users::mobile.eq(&updated_user.mobile),
                users::email.eq(&updated_user.email),
                users::mobile_verified.eq(updated_user.mobile_verified),
                users::email_verified.eq(updated_user.email_verified),
                users::deleted_flag.eq(updated_user.deleted_flag),
                users::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)?;
        users.find(user_id).first(conn)
    }

    pub fn delete(conn: &MysqlConnection, user_id: i32) -> QueryResult<usize> {
        diesel::delete(users.find(user_id)).execute(conn)
    }
}
