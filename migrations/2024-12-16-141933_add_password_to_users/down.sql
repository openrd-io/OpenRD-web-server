-- This file should undo anything in `up.sql`
ALTER TABLE chat_groups ADD CONSTRAINT fk_user_id FOREIGN KEY (chat_groups_ibfk_1) REFERENCES users(id);
ALTER TABLE chat_messages ADD CONSTRAINT fk_group_id FOREIGN KEY (chat_messages_ibfk_1) REFERENCES chat_groups(id);
ALTER TABLE chat_groups modify COLUMN user_id INT NOT NULL comment 'User ID';