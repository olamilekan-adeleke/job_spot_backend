-- Add up migration script here
ALTER TABLE
    users
ADD
    COLUMN created_at timestamptz DEFAULT NOW();

ALTER TABLE
    users
ALTER COLUMN
    created_at
SET
    DEFAULT NOW();

ALTER TABLE
    users
ADD
    COLUMN updated_at timestamptz DEFAULT NOW();

ALTER TABLE
    users
ALTER COLUMN
    updated_at
SET
    DEFAULT NOW();

