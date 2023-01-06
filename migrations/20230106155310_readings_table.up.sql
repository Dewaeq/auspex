-- Add up migration script here
CREATE TABLE readings (
    id SERIAL PRIMARY KEY,
    station_id INT NOT NULL,
    location_id INT,
    date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_DATE,
    temperature FLOAT(8) NOT NULL,
    humidity FLOAT(8) NOT NULL,
    pm10 FLOAT(8) NOT NULL,
    pm25 FLOAT(8) NOT NULL,
    co2 FLOAT(8) NOT NULL,
    voc FLOAT(8) NOT NULL
);