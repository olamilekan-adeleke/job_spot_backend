-- Add up migration script here
INSERT INTO
    users (
        full_name,
        email,
        username,
        hash_password,
        phone_number,
        bio,
        profile_url,
        user_location,
        date_of_birth,
        gender
    )
VALUES
    (
        'John Doe',
        'john.doe@example.com',
        'johndoe',
        'hashed_password',
        '1234567890',
        'Some bio',
        'http://example.com/profile',
        'Some location',
        '1990-01-01',
        'Male'
    );