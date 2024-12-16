-- Your SQL goes here

ALTER TABLE chat_groups DROP FOREIGN KEY chat_groups_ibfk_1;
ALTER TABLE chat_messages DROP FOREIGN KEY chat_messages_ibfk_1;

ALTER TABLE chat_groups modify COLUMN user_id VARCHAR(64) NOT NULL comment 'User ID';

