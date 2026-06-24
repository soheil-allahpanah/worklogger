CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE,
    password_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    disabled_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE api_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash BYTEA NOT NULL,
    label TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX api_tokens_user_id_idx ON api_tokens (user_id);
CREATE INDEX api_tokens_token_hash_idx ON api_tokens (token_hash);

INSERT INTO users (
    id,
    name,
    email,
    password_hash,
    created_at,
    updated_at,
    disabled_at,
    deleted_at
) VALUES (
    '00000000-0000-4000-8000-000000000001',
    'legacy',
    NULL,
    NULL,
    NOW(),
    NOW(),
    NULL,
    NULL
);

ALTER TABLE worklogs
    ADD COLUMN user_id UUID REFERENCES users(id);

UPDATE worklogs
SET user_id = '00000000-0000-4000-8000-000000000001'
WHERE user_id IS NULL;

ALTER TABLE worklogs
    ALTER COLUMN user_id SET NOT NULL;

CREATE INDEX worklogs_user_id_datetime_idx ON worklogs (user_id, datetime);
