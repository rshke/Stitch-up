-- Add up migration script here
CREATE TABLE members (
    member_id uuid PRIMARY key,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL
)
