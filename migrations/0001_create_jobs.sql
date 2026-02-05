CREATE TABLE jobs (
    id UUID PRIMARY KEY,
    payload JSONB NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,

    state TEXT NOT NULL CHECK (
        state IN (
            'queued',
            'running',
            'succeeded',
            'failed',
            'cancelled'
        )
    ),

    attempt INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,

    failure_type TEXT NULL CHECK (
        failure_type IN (
            'user_error',
            'system_error',
            'timeout'
        )
    ),

    failure_reason TEXT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_jobs_state_priority
    ON jobs (state, priority DESC, created_at ASC);
