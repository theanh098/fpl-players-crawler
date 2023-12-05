use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::fpl_service::Player;

pub struct DFantasyDb(Pool<Postgres>);

impl DFantasyDb {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .connect("postgresql://postgres:vitaminc@localhost:5432/fpl_mock")
            .await
            .unwrap();

        Self(pool)
    }

    pub async fn save_players(&self, players: Vec<Player>) -> Result<(), sqlx::Error> {
        let mut fpl_id_list: Vec<f64> = Vec::new();
        let mut team_name_list: Vec<String> = Vec::new();
        let mut manager_name_list: Vec<String> = Vec::new();

        players.into_iter().for_each(|player| {
            fpl_id_list.push(player.id as f64);
            team_name_list.push(player.name);
            manager_name_list.push(player.player_first_name + player.player_last_name.as_ref());

            dbg!(&fpl_id_list);
            dbg!(&team_name_list);
            dbg!(&manager_name_list);
        });

        sqlx::query(
            r#"
                INSERT INTO public.fpl_mock.fpl_users (fpl_id, team_name, manager_name)
                VALUES 
                    SELECT * FROM UNNEST ($1, $2, $3)
                ON CONFLICT (fpl_id) DO UPDATE
                SET name = EXCLUDED.name, team_name = EXCLUDED.team_name,manager_name = EXCLUDED.manager_name;
            "#,
        )
        .bind(&fpl_id_list).bind(&team_name_list).bind(&manager_name_list)
        .execute(&self.0)
        .await
        .map(|_| ())
    }
}
