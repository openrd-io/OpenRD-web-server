-- Your SQL goes here
-- 聊天组表
CREATE TABLE chat_groups (
    id INT PRIMARY KEY AUTO_INCREMENT,
    biz_id VARCHAR(64) NOT NULL UNIQUE COMMENT '业务ID',
    user_id INT NOT NULL COMMENT '创建者ID',
    title VARCHAR(255) NOT NULL COMMENT '聊天组标题',
    description TEXT COMMENT '描述',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    deleted_flag BOOLEAN NOT NULL DEFAULT FALSE COMMENT '删除标记',
    INDEX idx_user_id (user_id),
    INDEX idx_biz_id (biz_id),
    INDEX idx_created_at (created_at),
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='聊天组表';

-- 聊天消息表
CREATE TABLE chat_messages (
    id INT PRIMARY KEY AUTO_INCREMENT,
    biz_id VARCHAR(64) NOT NULL UNIQUE COMMENT '业务ID',
    group_id INT NOT NULL COMMENT '聊天组ID',
    role VARCHAR(20) NOT NULL COMMENT '角色(system/user/assistant)',
    content TEXT NOT NULL COMMENT '消息内容',
    tokens INT NOT NULL DEFAULT 0 COMMENT 'token数量',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    deleted_flag BOOLEAN NOT NULL DEFAULT FALSE COMMENT '删除标记',
    INDEX idx_group_id (group_id),
    INDEX idx_biz_id (biz_id),
    INDEX idx_created_at (created_at),
    FOREIGN KEY (group_id) REFERENCES chat_groups(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='聊天消息表';
