CREATE TABLE worklogger_service.public.worklogs (
    id UUID PRIMARY KEY,
    datetime TIMESTAMPTZ NOT NULL,
    duration INTERVAL NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}',
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT worklogs_duration_positive CHECK (duration > INTERVAL '0')
);

CREATE INDEX worklogs_datetime_idx ON worklogger_service.public.worklogs (datetime);
CREATE INDEX worklogs_tags_gin_idx ON worklogger_service.public.worklogs USING GIN (tags);