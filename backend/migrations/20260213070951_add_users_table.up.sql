-- Add up migration script here
CREATE TABLE users (
    user_id uuid PRIMARY key,
    username text NOT NULL UNIQUE,
    password_hash text NOT NULL
);
