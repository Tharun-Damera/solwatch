-- Add migration script here
CREATE TABLE accounts (
    address VARCHAR(50) PRIMARY KEY,
    lamports BIGINT,
    owner VARCHAR(50),
    executable BOOLEAN,
    data_length BIGINT,
    rent_epoch BIGINT,
    indexed_at TIMESTAMPTZ DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ DEFAULT NOW()
);