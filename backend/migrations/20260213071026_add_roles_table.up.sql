-- Add up migration script here
CREATE TABLE roles (
    role_id uuid PRIMARY key,
    name text NOT NULL,
    description text NOT NULL
);
