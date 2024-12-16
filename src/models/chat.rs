use crate::schema::{chat_groups, chat_messages};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 聊天组模型
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = chat_groups)]
pub struct ChatGroup {
    pub id: i32,
    pub biz_id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_flag: bool,
}

// 聊天组 DTO
#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = chat_groups)]
pub struct ChatGroupDTO {
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub deleted_flag: bool,
}

// 聊天消息模型
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = chat_messages)]
pub struct ChatMessage {
    pub id: i32,
    pub biz_id: String,
    pub group_id: String,
    pub role: String,
    pub content: String,
    pub tokens: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_flag: bool,
}

// 聊天消息 DTO
#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = chat_messages)]
pub struct ChatMessageDTO {
    pub group_id: Option<String>,
    pub role: String,
    pub content: String,
    pub tokens: i32,
    #[serde(default)]
    pub deleted_flag: bool,
}

// ChatGroup 实现
impl ChatGroup {
    pub fn create(conn: &mut MysqlConnection, group_dto: &ChatGroupDTO) -> QueryResult<ChatGroup> {
        use crate::schema::chat_groups::dsl::*;

        // 如果没有传入 description，则设置为 默认的描述
        let descrips = match &group_dto.description {
            Some(desc) => desc,
            None => "这是一个聊天组",
        };

        let new_group = (
            biz_id.eq(Uuid::new_v4().to_string()),
            user_id.eq(group_dto.user_id.clone()),
            title.eq(&group_dto.title),
            description.eq(descrips),
            deleted_flag.eq(false),
        );

        diesel::insert_into(chat_groups)
            .values(new_group)
            .execute(conn)?;

        chat_groups.order(id.desc()).first(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, group_id: i32) -> QueryResult<ChatGroup> {
        use crate::schema::chat_groups::dsl::*;
        chat_groups
            .find(group_id)
            .filter(deleted_flag.eq(false))
            .first(conn)
    }

    pub fn find_by_user_id(
        conn: &mut MysqlConnection,
        user_id_val: String,
        page: i64,
        per_page: i64,
    ) -> QueryResult<Vec<ChatGroup>> {
        use crate::schema::chat_groups::dsl::*;
        chat_groups
            .filter(user_id.eq(user_id_val))
            .filter(deleted_flag.eq(false))
            .order(id.desc())
            .limit(per_page)
            .offset((page - 1) * per_page)
            .load(conn)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        group_id: i32,
        group_dto: &ChatGroupDTO,
    ) -> QueryResult<ChatGroup> {
        use crate::schema::chat_groups::dsl::*;

        diesel::update(chat_groups.find(group_id))
            .set((
                title.eq(&group_dto.title),
                description.eq(&group_dto.description),
                deleted_flag.eq(group_dto.deleted_flag),
            ))
            .execute(conn)?;

        chat_groups.find(group_id).first(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, group_id: i32) -> QueryResult<usize> {
        use crate::schema::chat_groups::dsl::*;
        diesel::update(chat_groups.find(group_id))
            .set(deleted_flag.eq(true))
            .execute(conn)
    }

    pub fn count_by_user(conn: &mut MysqlConnection, user_id_val: String) -> QueryResult<i64> {
        use crate::schema::chat_groups::dsl::*;
        use diesel::dsl::count;

        chat_groups
            .filter(user_id.eq(user_id_val.clone()))
            .filter(deleted_flag.eq(false))
            .select(count(id))
            .first(conn)
    }
}

// ChatMessage 实现
impl ChatMessage {
    pub fn create(
        conn: &mut MysqlConnection,
        message_dto: &ChatMessageDTO,
    ) -> QueryResult<ChatMessage> {
        use crate::schema::chat_messages::dsl::*;

        let new_message = (
            biz_id.eq(Uuid::new_v4().to_string()),
            group_id.eq(message_dto.group_id.clone().unwrap_or_default()),
            role.eq(&message_dto.role),
            content.eq(&message_dto.content),
            tokens.eq(message_dto.tokens),
            deleted_flag.eq(false),
        );

        diesel::insert_into(chat_messages)
            .values(new_message)
            .execute(conn)?;

        chat_messages.order(id.desc()).first(conn)
    }

    pub fn find_by_group_id(
        conn: &mut MysqlConnection,
        group_id_val: String,
        page: i64,
        per_page: i64,
    ) -> QueryResult<Vec<ChatMessage>> {
        use crate::schema::chat_messages::dsl::*;
        chat_messages
            .filter(group_id.eq(group_id_val))
            .filter(deleted_flag.eq(false))
            .order(id.desc())
            .limit(per_page)
            .offset((page - 1) * per_page)
            .load(conn)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        message_id: i32,
        message_dto: &ChatMessageDTO,
    ) -> QueryResult<ChatMessage> {
        use crate::schema::chat_messages::dsl::*;

        diesel::update(chat_messages.find(message_id))
            .set((
                content.eq(&message_dto.content),
                tokens.eq(message_dto.tokens),
                deleted_flag.eq(message_dto.deleted_flag),
            ))
            .execute(conn)?;

        chat_messages.find(message_id).first(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, message_id: i32) -> QueryResult<usize> {
        use crate::schema::chat_messages::dsl::*;
        diesel::update(chat_messages.find(message_id))
            .set(deleted_flag.eq(true))
            .execute(conn)
    }

    pub fn count_by_group(conn: &mut MysqlConnection, group_id_val: String) -> QueryResult<i64> {
        use crate::schema::chat_messages::dsl::*;
        use diesel::dsl::count;

        chat_messages
            .filter(group_id.eq(group_id_val))
            .filter(deleted_flag.eq(false))
            .select(count(id))
            .first(conn)
    }

    pub fn find_latest_by_group(
        conn: &mut MysqlConnection,
        group_id_val: String,
        limit: i64,
    ) -> QueryResult<Vec<ChatMessage>> {
        use crate::schema::chat_messages::dsl::*;
        chat_messages
            .filter(group_id.eq(group_id_val))
            .filter(deleted_flag.eq(false))
            .order(id.desc())
            .limit(limit)
            .load(conn)
    }

    pub fn default_messages() -> Vec<String> {
        vec![
            "首次使用：我是第一次使用，请告诉我如何开始提问？".to_string(),
            "疾病诊断：我或身边的人似乎得了某种难以诊断的疾病，我想找到病因".to_string(),
            "医疗救助：我或身边的人已确诊某种罕见病，我想求助对应的医疗机构、官方或民间组织"
                .to_string(),
            "行业支持：我是罕见病领域的从业者，我想了解学习专业详细的行业信息".to_string(),
            "初次了解：我好奇罕见病这个领域，想获取系统的知识".to_string(),
        ]
    }
}

// 用于查询的结构体
#[derive(Debug, Deserialize)]
pub struct ChatQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
