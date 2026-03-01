#### Add and remove custom methods

how to manage many to many relationship? For example, roles and permissions.

key concept:
- managing resources

api design:
- add associated resources to managing resource
    `/managing-resources/{id}/associated-resources/add`
    return 409 Conflict if associated resource is already added
- remove associated resources from managing resource
    `/managing-resources/{id}/associated-resources/remove`
    return 412 Precondition Failed if associated resource is not remove
- list associated resources of managing resource
    `/managing-resources/{id}/associated-resources`
- list managing resources of associated resource
    `/associated-resources/{id}/managing-resources`

### PostgreSQL Batch Insert with Conflict Handling

When using `INSERT INTO ... VALUES (...)` for batch insertion, `ON CONFLICT DO NOTHING` works on a **per-row** basis.

- **Behavior**: It iterates through each row in the `VALUES` list. If a row violates a unique constraint, it is ignored, and the next row is processed.
*   **Syntax**:
    ```sql
    INSERT INTO table_name (col1, col2)
    VALUES (v1, v2), (v3, v4)
    ON CONFLICT (constraint_col) DO NOTHING;
    ```
- **RETURNING**: Rows that are skipped due to conflict will **not** be included in the `RETURNING` result set.
- **SQLx QueryBuilder**: You can use `.push(" ON CONFLICT (...) DO NOTHING")` at the end of a built query to apply this logic to dynamic batch inserts.

