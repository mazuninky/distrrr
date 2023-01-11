CREATE TYPE distrrr.job_status AS ENUM ('waiting', 'processing', 'competed', 'error', 'canceled');

CREATE TABLE distrrr.job
(
    id          uuid PRIMARY KEY,
    name        varchar(128),
    data        bytea,
    status      distrrr.job_status not null default 'waiting',
    retry_count numeric(4, 0)      not null default 0,
    created_at  timestamp          not null default now()
)
