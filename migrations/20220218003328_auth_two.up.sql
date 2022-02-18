-- Add up migration script here

CREATE TYPE preference AS ENUM ('tweets', 'rts', 'likes');

CREATE DATABASE auth_two(
    id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    user_id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    pkce VARCHAR(128) NOT NULL,
    pref preference [],
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
);
