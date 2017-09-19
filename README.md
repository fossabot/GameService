[![Build Status](https://travis-ci.org/Mikibot/GameService.svg?branch=master)](https://travis-ci.org/Mikibot/GameService)

## Install Requirements:
 - [libpq](https://www.postgresql.org)
 - [rust](https://rustup.rs)
 
  
## Setup:
Set the environment variables:

	DATABASE_URL
	TEST_DATABASE_URL
And configure [rocket](https://rocket.rs/guide/configuration/)

###### .env is supported, if you use it, you need to configure [Rocket.toml](https://rocket.rs/guide/configuration/#rockettoml)

## Routes:
- `/blackjack`
	- `GET: /` - Active Sessions (where game isnt in a completed state).
	- `GET: /<uid>` - Information about `<uid>`'s current game.
	- `POST: /<uid>/<bet>` - Creates a new game for `<uid>` with `<bet>` at stake. 
	- `POST: /<uid>/hit` - Draw another card for `<uid>`.
	- `POST: /<uid>/stay` - Tell the dealer you are done, dealer will make its play.
	- `POST: /<uid>/claim` - Returns the amount bet + reward, either 0 or 2x the `bet`
- `/slot_machine`
	- `GET: /<bet>` - returns the bet mutiplied by 0/1.5/2
