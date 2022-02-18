-- Add up migration script here

CREATE TABLE auth_two(
    user_id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    pkce VARCHAR(128) NOT NULL,
    access_token VARCHAR(128) NOT NULL,
    refresh_token VARCHAR(128) NOT NULL
);
