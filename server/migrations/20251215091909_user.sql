CREATE TYPE user_role AS ENUM (
    'admin', -- Server access, no personal data access
    'owner', -- Full home control
    'user'   -- Can trigger overrides
);

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE CHECK (length(username) >= 3),
    password_hash TEXT NOT NULL, -- argon2id
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- INSERT INTO users (username, password_hash, role) VALUES -- password: 'test'
--     ('test', '$argon2id$v=19$m=32768,t=3,p=1$dGVzdHNhbHQ$K9fBGHb3RAvV/eL0TXwgo2FvuXK4XOl9xau/uzCrsDw', 'user');
