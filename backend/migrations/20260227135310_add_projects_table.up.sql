-- Add up migration script here
CREATE TABLE if NOT EXISTS projects (
    project_id uuid PRIMARY key DEFAULT gen_random_uuid (),
    name VARCHAR(255) NOT NULL,
    description text NOT NULL
);
