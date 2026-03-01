-- Add up migration script here
CREATE TABLE permissions (
    permission_id uuid PRIMARY key,
    resource text NOT NULL,
    action text NOT NULL,
    scope text NOT NULL
);
