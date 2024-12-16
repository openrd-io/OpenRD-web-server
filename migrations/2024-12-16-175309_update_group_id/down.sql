-- This file should undo anything in `up.sql`
ALTER TABLE chat_messages modify COLUMN group_id VARCHAR(64) NOT NULL comment 'Group ID';