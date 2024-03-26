-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE verifications_code (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  user_id uuid NOT NULL,
  code varchar(6) NOT NULL,
  expiration_date timestamp NOT NULL,
  FOREIGN KEY (user_id) REFERENCES "users"(user_id) ON DELETE CASCADE
);
