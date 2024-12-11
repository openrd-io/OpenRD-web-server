use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::sql_types::{Integer, Varchar, Bool, Timestamp};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::users::{self, biz_id};
use diesel::prelude::*;

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
}

// 用于创建和更新的 DTO
#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
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

    pub fn get_by_biz_id(conn: &mut MysqlConnection, biz_id_value: &str) -> QueryResult<User> {
        use crate::schema::users;
        use diesel::prelude::*;

        users::table
            .filter(users::biz_id.eq(biz_id_value))
            .select(User::as_select())
            .first(conn)
    }

    pub fn create(conn: &mut MysqlConnection, user_dto: &UserDTO) -> QueryResult<User> {
        use crate::schema::users;
        diesel::insert_into(users::table)
            .values(user_dto)
            .execute(conn)?;
        users::table
            .order(users::id.desc())
            .first(conn)
    }
  

    pub fn update(conn: &mut MysqlConnection, user_id: i32, updated_user: &UserDTO) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        diesel::update(users.find(user_id))
            .set((
                name.eq(&updated_user.name),
                mobile.eq(&updated_user.mobile),
                email.eq(&updated_user.email),
                mobile_verified.eq(updated_user.mobile_verified),
                email_verified.eq(updated_user.email_verified),
                deleted_flag.eq(updated_user.deleted_flag),
                updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)?;
        users.find(user_id).first::<User>(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, user_id: i32) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;
        diesel::delete(users.find(user_id)).execute(conn)
    }
}