use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Player {
    pub id: u64,
    pub player_first_name: String,
    pub player_last_name: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Bootstrap {
    total_players: u64,
}

pub async fn get_player_by_fpl_id(fpl_id: u64) -> Player {
    let response = reqwest::get(format!(
        "https://fantasy.premierleague.com/api/entry/{fpl_id}"
    ))
    .await;

    let Ok(response) = response else {
        return Player {
            id: fpl_id,
            ..Default::default()
        };
    };

    response.json::<Player>().await.unwrap_or(Player {
        id: fpl_id,
        ..Default::default()
    })
}

pub async fn get_total_players() -> Result<u64, reqwest::Error> {
    reqwest::get("https://fantasy.premierleague.com/api/bootstrap-static")
        .await?
        .json::<Bootstrap>()
        .await
        .map(|bootstrap| bootstrap.total_players)
}
