
# Sample API Responses

##### Game_state:
	null  => In progress
	true  => Player Won
	false => Player Lost
### POST: `/blackjack/0/create/100`
```json
{
	"status": {
		"Ok": {
			"bet": 100,
				"dealer_can_hit": true,
				"dealer_hand": [
					"SIXES"
				],
				"dealer_score": 6,
				"game_state": null,
				"player_can_hit": true,
				"player_hand": [
					"TENS",
				"FOURS"
				],
				"player_id": 0,
				"player_score": 14
		}
	},
		"status_code": 200
}
```
###### Note: Only dealer's first card is shown for the first turn
### POST: `/blackjack/0`
```json
{
	"status": {
		"Ok": {
			"bet": 100,
				"dealer_can_hit": true,
				"dealer_hand": [
					"SIXES"
				],
				"dealer_score": 6,
				"game_state": null,
				"player_can_hit": true,
				"player_hand": [
					"TENS",
				"FOURS"
				],
				"player_id": 0,
				"player_score": 14
		}
	},
		"status_code": 200
}
```
###### Note: Only dealer's first card is shown for the first turn

### POST: `/blackjack/0/hit`
```json
{
	"status": {
		"Ok": {
			"bet": 100,
				"dealer_can_hit": true,
				"dealer_hand": [
					"SIXES",
				"THREES"
				],
				"dealer_score": 9,
				"game_state": null,
				"player_can_hit": true,
				"player_hand": [
					"TENS",
				"FOURS",
				"TWOS"
				],
				"player_id": 0,
				"player_score": 16
		}
	},
		"status_code": 200
}
```
### POST: `/blackjack/0/stay`
```json
{
	"status": {
		"Ok": {
			"bet": 100,
				"dealer_can_hit": false,
				"dealer_hand": [
					"SIXES",
				"THREES"
				],
				"dealer_score": 9,
				"game_state": true,
				"player_can_hit": false,
				"player_hand": [
					"TENS",
				"FOURS",
				"TWOS"
				],
				"player_id": 0,
				"player_score": 16
		}
	},
		"status_code": 200
}
```
### POST: `/blackjack/0/claim`
###### Note: claim will return back a number, which is your bet returned from the game this wil either be 0/2x the bet depending on win/lose.
```json
{
	"status": {
		"Ok": 200
	},
		"status_code": 200
}
```
### POST: `/blackjack/0/hit` (err)
```json
{
	"status": {
		"Err": "You already won"
	},
		"status_code": 501
}
```
### POST: `/blackjack/0/100` (err)
```json
{
	"status": {
		"Err": "Failed to create, bet must be claimed before recreating a session."
	},
		"status_code": 501
}
```
### POST: `/blackjack/0/claim` (err)
```json
{
	"status": {
		"Err": "Game is not over yet"
	},
		"status_code": 501
}
```
### POST: `/blackjack/0/stay` (err)
```json
{
	"status": {
		"Err": "User does not exist"
	},
		"status_code": 501
}
```
###### NOTE: using stay more than once wont cause an error, just server side or user not existing
