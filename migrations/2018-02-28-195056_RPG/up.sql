-- Your SQL goes here
CREATE TABLE RPGPlayer (
	id	   BIGINT NOT NULL PRIMARY KEY,
	exp	           BIGINT NOT NULL DEFAULT 0,
    damage_recieved BIGINT NOT NULL DEFAULT 0,
    gear   TEXT[] NOT NULL
)