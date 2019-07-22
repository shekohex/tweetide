CREATE TABLE users
(
    id         INTEGER            NOT NULL PRIMARY KEY,
    username   VARCHAR(30) UNIQUE NOT NULL,
    password   VARCHAR            NOT NULL,
    created_at datetime           NOT NULL DEFAULT CURRENT_TIMESTAMP
)
