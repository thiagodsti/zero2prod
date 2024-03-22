CREATE
extension if not exists "pgcrypto";

CREATE TABLE subscriptions
(
    id            uuid        NOT NULL,
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE users
(
    id         VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL,
    password   VARCHAR(255) NOT NULL,
    roles      VARCHAR(255)[] NOT NULL DEFAULT '{BASIC}',
    created_at timestamptz  NOT NULL   DEFAULT NOW(),
    updated_at timestamptz
);

CREATE TABLE refresh_token
(
    id         VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    VARCHAR(36) NOT NULL,
    token      VARCHAR(36) NOT NULL,
    expiry_at  timestamptz NOT NULL    DEFAULT NOW(),
    updated_at timestamptz NOT NULL    DEFAULT NOW(),
    CONSTRAINT fk_refresh_token_user_id FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT uk_refresh_token_token UNIQUE (token)
);

CREATE TABLE recovery_password
(
    id         VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    VARCHAR(36) NOT NULL,
    created_at timestamptz NOT NULL    DEFAULT NOW(),
    CONSTRAINT uk_recovery_password_user_id UNIQUE (user_id),
    CONSTRAINT fk_recovery_password_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

ALTER TABLE users
    ADD CONSTRAINT uk_users_email UNIQUE (email);