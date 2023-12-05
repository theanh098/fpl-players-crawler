mod database;
mod fpl_service;

use std::time::Duration;

use database::DFantasyDb;
use fpl_service::{get_player_by_fpl_id, get_total_players, Player};
use futures::future;
use rand::{random, Rng};

static BATCH_SIZE: u64 = 300;

#[tokio::main]
async fn main() {
    let total_players = get_total_players().await.unwrap();

    let mut i = 1;

    loop {
        println!("total players: {}", total_players);

        if i >= total_players {
            return ();
        };

        let tasks = (i..i + BATCH_SIZE).into_iter().map(|fpl_id| {
            let rand_time = rand::thread_rng().gen_range(10..30);

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(rand_time)).await;
                get_player_by_fpl_id(fpl_id).await
            })
        });

        let players = future::join_all(tasks)
            .await
            .into_iter()
            .filter(|r| r.as_ref().ok().is_some_and(|player| player.name == ""))
            .map(|r| r.unwrap())
            .collect::<Vec<Player>>();

        dbg!(&players.len());

        // let db = DFantasyDb::new().await;

        // db.save_players(players).await.unwrap();

        // tokio::time::sleep(Duration::from_secs(1)).await;

        i += BATCH_SIZE;
    }
}
