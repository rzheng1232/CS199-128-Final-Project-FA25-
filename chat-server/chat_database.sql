-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY,               -- auto-increments
    username TEXT UNIQUE NOT NULL,
    role TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Chats table
CREATE TABLE chats (
    id INTEGER PRIMARY KEY,
    name TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Chat users table
CREATE TABLE chat_users (
    id INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    is_active BOOLEAN DEFAULT 0,
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(chat_id) REFERENCES chats(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- Messages table
CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status TEXT,
    FOREIGN KEY(chat_id) REFERENCES chats(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- Message queue table
CREATE TABLE message_queue (
    id INTEGER PRIMARY KEY,
    message_id INTEGER NOT NULL,
    direction TEXT,
    queued_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMP,
    status TEXT,
    FOREIGN KEY(message_id) REFERENCES messages(id)
);

-- Chat history cache table
CREATE TABLE chat_history_cache (
    id INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    message_history TEXT,                 -- SQLite supports JSON functions if stored as TEXT
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(chat_id) REFERENCES chats(id)
);