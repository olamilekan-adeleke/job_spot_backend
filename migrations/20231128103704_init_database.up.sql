-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    user_id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    full_name varchar(200) NOT NULL,
    email varchar(200) NOT NULL UNIQUE,
    username varchar(200) NOT NULL UNIQUE,
    hash_password varchar NOT NULL,
    phone_number varchar(11),
    bio varchar(2500),
    profile_url varchar,
    user_location varchar,
    date_of_birth varchar,
    gender varchar(10)
);