CREATE TABLE transactions (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  debit INTEGER NOT NULL,
  credit INTEGER NOT NULL,
  payment INTEGER NOT NULL,
  time_created TEXT DEFAULT (datetime('now', 'localtime'))
)
-- Your SQL goes here