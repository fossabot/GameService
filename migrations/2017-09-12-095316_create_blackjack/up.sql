-- Your SQL goes here
CREATE TABLE BlackJack (
	id	   BIGINT NOT NULL PRIMARY KEY CONSTRAINT unq_ord_no UNIQUE,
	bet	           BIGINT,
	-- NULL - in progress, TRUE - player win - FALSE - dealer win
	status         BOOLEAN,
	deck	       TEXT[] NOT NULL,
	player_hand	   TEXT[] NOT NULL,
	dealer_hand	   TEXT[] NOT NULL,
	player_stay	   BOOLEAN      NOT NULL DEFAULT FALSE,
	dealer_stay    BOOLEAN      NOT NULL DEFAULT FALSE,
	first_turn     BOOLEAN NOT NULL DEFAULT FALSE
)
