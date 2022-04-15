-- Add up migration script here
CREATE TABLE user_preference(
    user_id UUID PRIMARY KEY UNIQUE NOT NULL,
    v1_active BOOLEAN NOT NULL,
    v2_active BOOLEAN NOT NULL
)