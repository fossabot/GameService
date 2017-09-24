#[derive(Serialize, Deserialize)]
pub struct Game {
    bet: u64,
    result: Option<bool>,
    computer: String,
    player: String,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    bet: u64,
    msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    status_code: u16,
    status: Result<Game, Error>,
}

impl Response {
    pub fn win(bet: u64, player: String, computer: String) -> Self {
        let bet: u64 = (bet as f64 * 1.25) as u64;
        Self {
            status_code: 200,
            status: Ok(Game {
                bet,
                result: Some(true),
                player,
                computer,
            }),
        }
    }

    pub fn lose(player: String, computer: String) -> Self {
        Self {
            status_code: 200,
            status: Ok(Game {
                bet: 0,
                result: Some(false),
                player,
                computer,
            }),
        }
    }

    pub fn draw(bet: u64, player: String, computer: String) -> Self {
        Self {
            status_code: 200,
            status: Ok(Game {
                bet,
                result: Some(false),
                player,
                computer,
            }),
        }
    }

    pub fn error(bet: u64, msg: String) -> Self {
        Self {
            status_code: 501,
            status: Err(Error { bet, msg }),
        }
    }
}
