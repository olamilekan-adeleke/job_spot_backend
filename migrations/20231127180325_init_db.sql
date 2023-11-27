-- Add migration script here
create table users (
    user_id serial primary key,
    full_name varchar(200) not null,
    email varchar(200) not null unique,
    username varchar(200) not null unique,
    hash_password varchar not null,
    phone_number varchar(11),
    bio varchar(2500),
    profile_url varchar,
    user_location varchar,
    date_of_birth varchar,
    gender varchar(10)
);