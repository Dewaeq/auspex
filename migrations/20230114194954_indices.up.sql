-- Add up migration script here
CREATE INDEX token_idx ON stations(token);
CREATE INDEX station_id_idx ON readings(station_id);