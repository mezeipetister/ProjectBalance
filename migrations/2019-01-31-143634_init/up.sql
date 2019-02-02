CREATE TABLE transactions (
  id SERIAL PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  debit INTEGER NOT NULL,
  credit INTEGER NOT NULL,
  payment INTEGER NOT NULL,
  time_created timestamptz NOT NULL default current_timestamp
)
-- Your SQL goes here