-- Your SQL goes here

CREATE TABLE replies (
  id SERIAL PRIMARY KEY,
  tag TEXT UNIQUE NOT NULL,
  url TEXT NOT NULL
);

CREATE INDEX tag ON replies (tag);
