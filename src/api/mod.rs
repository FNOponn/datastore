use std::env;

use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

pub mod test_data;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    #[serde(rename = "sport_key")]
    pub sport_key: String,
    #[serde(rename = "sport_title")]
    pub sport_title: String,
    #[serde(rename = "commence_time")]
    pub commence_time: String,
    #[serde(rename = "home_team")]
    pub home_team: String,
    #[serde(rename = "away_team")]
    pub away_team: String,
    pub bookmakers: Vec<Bookmaker>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmaker {
    pub key: String,
    pub title: String,
    #[serde(rename = "last_update")]
    pub last_update: String,
    pub markets: Vec<Market>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub key: String,
    #[serde(rename = "last_update")]
    pub last_update: String,
    pub outcomes: Vec<Outcome>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outcome {
    pub name: String,
    pub price: i64,
}

impl Game {
    pub async fn get_upcoming_games() -> Result<Vec<Game>> {
        dotenv().ok();

        let odds_base_url = "https://api.the-odds-api.com";
        let api_key =
            env::var("ODDS_API_KEY").expect("Please set the ODDS API Key environment var!");
        let markets = "h2h";
        let odds_format = "american";
        let upcoming_games_url = format!(
            "{}/v4/sports/basketball_nba/odds/?regions=us&markets={}&apiKey={}&oddsFormat={}",
            odds_base_url, markets, api_key, odds_format
        );

        let response = reqwest::get(&upcoming_games_url)
            .await?
            .json::<Vec<Game>>()
            .await?;

        println!("{:#?}", upcoming_games_url);
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_01_get_req() {
        let response = reqwest::get("https://www.example.com").await.unwrap();

        println!("Status: {}", response.status());
    }

    #[tokio::test]
    async fn test_02_get_upcoming_games() {
        let response = Game::get_upcoming_games().await.unwrap();
        println!("{:#?}", response);
    }
}
