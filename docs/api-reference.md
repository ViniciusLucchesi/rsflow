# API Reference

The API provides endpoints to manage users, groups and their associations. Business rules are documented in the Swagger UI available at `/swagger` when running the application.

## Users
- `POST /users` &mdash; Creates a user after validating name and email.
- `GET /users/{id}` &mdash; Retrieves a single user.
- `GET /users/all` &mdash; Lists all users.

## Groups
- `POST /groups` &mdash; Creates a group with a name and description.
- `GET /groups/{id}` &mdash; Fetches a group by id.
- `GET /groups/all` &mdash; Lists all groups.

## User Groups
- `POST /user-groups` &mdash; Links a user to a group, ensuring both exist.
- `GET /user-groups/{id}` &mdash; Gets a specific user-group relation.
- `GET /user-groups/user/{user_id}` &mdash; Lists groups for a user.
- `GET /user-groups/group/{group_id}` &mdash; Lists users of a group.
- `GET /user-groups/all` &mdash; Lists all relations.

Return to the [index](index.md).
