-- Add up migration script here
-- this type user_role is associated trait derive that we created in model file
CREATE TYPE user_role AS ENUM ('admin','moderator','user');
-- this uuis-ossp is the extension for unique identifier generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "users" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        photo VARCHAR NOT NULL DEFAULT 'default.png',
        verified BOOLEAN NOT NULL DEFAULT FALSE,
        password VARCHAR(100) NOT NULL,
        role user_role NOT NULL DEFAULT 'user',
        created_at TIMESTAMP
        WITH 
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH 
            TIME ZONE DEFAULT NOW()
    );

-- this creates the index numbers on email column of the 'users' table
CREATE INDEX users_email_idx ON users (email)



