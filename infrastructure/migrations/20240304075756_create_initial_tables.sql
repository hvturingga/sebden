-- Add migration script here
CREATE TABLE users (
                       id BIGINT PRIMARY KEY,
                       username VARCHAR(255) NOT NULL,
                       email VARCHAR(255) NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       bio TEXT
);