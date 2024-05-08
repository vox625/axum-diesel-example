-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  description TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE
);