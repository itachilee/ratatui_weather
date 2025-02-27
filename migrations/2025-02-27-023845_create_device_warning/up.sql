-- Your SQL goes here
CREATE TABLE warnings (
    id SERIAL PRIMARY KEY,
    sensor_type TEXT NOT NULL,
    dev_ip TEXT NOT NULL,
    value DOUBLE PRECISION,
    threshold DOUBLE PRECISION,
    reason INTEGER NOT NULL,
    desciption TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);