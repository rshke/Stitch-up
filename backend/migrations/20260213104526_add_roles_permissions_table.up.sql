-- Add up migration script here
CREATE TABLE roles_permissions (
    role_id uuid NOT NULL,
    permission_id uuid NOT NULL,
    CONSTRAINT fk_roke FOREIGN key (role_id) REFERENCES roles (role_id),
    CONSTRAINT fk_permission FOREIGN key (permission_id) REFERENCES permissions (permission_id),
    PRIMARY key (role_id, permission_id)
);
