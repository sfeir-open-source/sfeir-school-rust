CREATE TABLE tests
(
  id SERIAL PRIMARY KEY,
  val BIGINT NOT NULL
);

CREATE TABLE games
(
  id SERIAL PRIMARY KEY,
  turn integer DEFAULT 0,
  player character varying NOT NULL
);

CREATE TABLE cases
(
  id SERIAL PRIMARY KEY,
  position character varying DEFAULT 0,
  alive BOOLEAN default true,
  game integer NOT NULL,
  FOREIGN KEY (game) REFERENCES games (id)
);

