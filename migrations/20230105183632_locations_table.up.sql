-- Add up migration script here
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    station_token TEXT NOT NULL,
    longitude FLOAT(8) NOT NULL,
    latitude FLOAT(8) NOT NULL,
    country TEXT NOT NULL,
    province TEXT NOT NULL,
    city TEXT NOT NULL,
    street TEXT NOT NULL,
    number TEXT NOT NULL
);