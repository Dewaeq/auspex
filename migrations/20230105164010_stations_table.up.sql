CREATE TABLE stations (
    id serial PRIMARY KEY,
    uid TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    hw_version INT NOT NULL,
    sw_version INT NOT NULL,
    location_id INT,
    last_online TIMESTAMPTZ NOT NULL DEFAULT CURRENT_DATE
);