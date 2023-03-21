#[derive(Debug)]
pub struct TestData {
    pub data_1: String,
    pub data_2: String,
}

impl TestData {
    pub fn new() -> TestData {
        TestData {
            data_1: String::from(
                r##"[
  {
    "id": "e40d079e6db5293e7e0aa22e0c857a85",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T00:10:00Z",
    "home_team": "Detroit Pistons",
    "away_team": "Washington Wizards",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 220
              },
              {
                "name": "Washington Wizards",
                "price": -260
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 215
              },
              {
                "name": "Washington Wizards",
                "price": -260
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 205
              },
              {
                "name": "Washington Wizards",
                "price": -265
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 205
              },
              {
                "name": "Washington Wizards",
                "price": -265
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 205
              },
              {
                "name": "Washington Wizards",
                "price": -265
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 205
              },
              {
                "name": "Washington Wizards",
                "price": -265
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 205
              },
              {
                "name": "Washington Wizards",
                "price": -265
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 220
              },
              {
                "name": "Washington Wizards",
                "price": -275
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 200
              },
              {
                "name": "Washington Wizards",
                "price": -240
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 206
              },
              {
                "name": "Washington Wizards",
                "price": -263
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 222
              },
              {
                "name": "Washington Wizards",
                "price": -278
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 200
              },
              {
                "name": "Washington Wizards",
                "price": -278
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 210
              },
              {
                "name": "Washington Wizards",
                "price": -250
              }
            ]
          }
        ]
      },
      {
        "key": "pointsbetus",
        "title": "PointsBet (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Detroit Pistons",
                "price": 220
              },
              {
                "name": "Washington Wizards",
                "price": -275
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "56420b74c402bfccb04db2542d901054",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T00:10:00Z",
    "home_team": "Orlando Magic",
    "away_team": "Milwaukee Bucks",
    "bookmakers": [
      {
        "key": "pointsbetus",
        "title": "PointsBet (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -300
              },
              {
                "name": "Orlando Magic",
                "price": 240
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -295
              },
              {
                "name": "Orlando Magic",
                "price": 240
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -280
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -286
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -286
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -286
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -286
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -286
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -295
              },
              {
                "name": "Orlando Magic",
                "price": 245
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -300
              },
              {
                "name": "Orlando Magic",
                "price": 240
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -290
              },
              {
                "name": "Orlando Magic",
                "price": 240
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -285
              },
              {
                "name": "Orlando Magic",
                "price": 228
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -333
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -280
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "wynnbet",
        "title": "WynnBET",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Milwaukee Bucks",
                "price": -294
              },
              {
                "name": "Orlando Magic",
                "price": 230
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "3ffbb2b93ab28e164a220a7d6d7162cb",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T00:30:00Z",
    "home_team": "Minnesota Timberwolves",
    "away_team": "Philadelphia 76ers",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": -105
              },
              {
                "name": "Philadelphia 76ers",
                "price": -115
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 108
              },
              {
                "name": "Philadelphia 76ers",
                "price": -126
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 108
              },
              {
                "name": "Philadelphia 76ers",
                "price": -130
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 108
              },
              {
                "name": "Philadelphia 76ers",
                "price": -130
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 108
              },
              {
                "name": "Philadelphia 76ers",
                "price": -130
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 108
              },
              {
                "name": "Philadelphia 76ers",
                "price": -130
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 108
              },
              {
                "name": "Philadelphia 76ers",
                "price": -130
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 105
              },
              {
                "name": "Philadelphia 76ers",
                "price": -125
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 105
              },
              {
                "name": "Philadelphia 76ers",
                "price": -125
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 102
              },
              {
                "name": "Philadelphia 76ers",
                "price": -125
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 105
              },
              {
                "name": "Philadelphia 76ers",
                "price": -125
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Minnesota Timberwolves",
                "price": 100
              },
              {
                "name": "Philadelphia 76ers",
                "price": -133
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "fdb0ebdf0ae93daaed8930abe48f95a7",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T00:40:00Z",
    "home_team": "New York Knicks",
    "away_team": "Charlotte Hornets",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 400
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 385
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 345
              },
              {
                "name": "New York Knicks",
                "price": -460
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 375
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -480
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 340
              },
              {
                "name": "New York Knicks",
                "price": -556
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Charlotte Hornets",
                "price": 360
              },
              {
                "name": "New York Knicks",
                "price": -500
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "efb4b56bd6dbe34edf174c60626c54a0",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T01:10:00Z",
    "home_team": "Houston Rockets",
    "away_team": "Brooklyn Nets",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -275
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "betonlineag",
        "title": "BetOnline.ag",
        "last_update": "2023-03-07T12:13:52Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:13:52Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -278
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "lowvig",
        "title": "LowVig.ag",
        "last_update": "2023-03-07T12:13:46Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:13:46Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -275
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -285
              },
              {
                "name": "Houston Rockets",
                "price": 228
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -275
              },
              {
                "name": "Houston Rockets",
                "price": 225
              }
            ]
          }
        ]
      },
      {
        "key": "wynnbet",
        "title": "WynnBET",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -294
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -280
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -275
              },
              {
                "name": "Houston Rockets",
                "price": 225
              }
            ]
          }
        ]
      },
      {
        "key": "superbook",
        "title": "SuperBook",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -270
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -275
              },
              {
                "name": "Houston Rockets",
                "price": 225
              }
            ]
          }
        ]
      },
      {
        "key": "pointsbetus",
        "title": "PointsBet (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -275
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -290
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -278
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -278
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -278
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -278
              },
              {
                "name": "Houston Rockets",
                "price": 225
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -278
              },
              {
                "name": "Houston Rockets",
                "price": 225
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Brooklyn Nets",
                "price": -333
              },
              {
                "name": "Houston Rockets",
                "price": 230
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "9c950da2cbab6a4e71437182846961d4",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T01:10:00Z",
    "home_team": "Oklahoma City Thunder",
    "away_team": "Golden State Warriors",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -175
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 150
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 152
              }
            ]
          }
        ]
      },
      {
        "key": "betonlineag",
        "title": "BetOnline.ag",
        "last_update": "2023-03-07T12:13:52Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:13:52Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -182
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 160
              }
            ]
          }
        ]
      },
      {
        "key": "lowvig",
        "title": "LowVig.ag",
        "last_update": "2023-03-07T12:13:46Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:13:46Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 160
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -176
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "wynnbet",
        "title": "WynnBET",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -185
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 155
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -175
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 150
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -190
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 155
              }
            ]
          }
        ]
      },
      {
        "key": "superbook",
        "title": "SuperBook",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -170
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 150
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 155
              }
            ]
          }
        ]
      },
      {
        "key": "pointsbetus",
        "title": "PointsBet (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -190
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 160
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -181
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -180
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 148
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Golden State Warriors",
                "price": -189
              },
              {
                "name": "Oklahoma City Thunder",
                "price": 140
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "faf4e0059f0d0dea9eebd49255e6ed15",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T01:40:00Z",
    "home_team": "Dallas Mavericks",
    "away_team": "Utah Jazz",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -280
              },
              {
                "name": "Utah Jazz",
                "price": 235
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -300
              },
              {
                "name": "Utah Jazz",
                "price": 245
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -275
              },
              {
                "name": "Utah Jazz",
                "price": 215
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -275
              },
              {
                "name": "Utah Jazz",
                "price": 215
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -275
              },
              {
                "name": "Utah Jazz",
                "price": 215
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -275
              },
              {
                "name": "Utah Jazz",
                "price": 215
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -275
              },
              {
                "name": "Utah Jazz",
                "price": 215
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -250
              },
              {
                "name": "Utah Jazz",
                "price": 200
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -260
              },
              {
                "name": "Utah Jazz",
                "price": 210
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -277
              },
              {
                "name": "Utah Jazz",
                "price": 219
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -278
              },
              {
                "name": "Utah Jazz",
                "price": 222
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -250
              },
              {
                "name": "Utah Jazz",
                "price": 210
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -303
              },
              {
                "name": "Utah Jazz",
                "price": 210
              }
            ]
          }
        ]
      },
      {
        "key": "pointsbetus",
        "title": "PointsBet (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Dallas Mavericks",
                "price": -275
              },
              {
                "name": "Utah Jazz",
                "price": 230
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "id": "0aa7ba9d4ef9dfacd6c1d4e545b86e87",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-08T03:00:00Z",
    "home_team": "Los Angeles Lakers",
    "away_team": "Memphis Grizzlies",
    "bookmakers": [
      {
        "key": "draftkings",
        "title": "DraftKings",
        "last_update": "2023-03-07T12:14:57Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:57Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "betonlineag",
        "title": "BetOnline.ag",
        "last_update": "2023-03-07T12:13:52Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:13:52Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -128
              },
              {
                "name": "Memphis Grizzlies",
                "price": 108
              }
            ]
          }
        ]
      },
      {
        "key": "lowvig",
        "title": "LowVig.ag",
        "last_update": "2023-03-07T12:13:46Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:13:46Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -128
              },
              {
                "name": "Memphis Grizzlies",
                "price": 108
              }
            ]
          }
        ]
      },
      {
        "key": "williamhill_us",
        "title": "William Hill (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "fanduel",
        "title": "FanDuel",
        "last_update": "2023-03-07T12:14:58Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:58Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -132
              },
              {
                "name": "Memphis Grizzlies",
                "price": 112
              }
            ]
          }
        ]
      },
      {
        "key": "wynnbet",
        "title": "WynnBET",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "bovada",
        "title": "Bovada",
        "last_update": "2023-03-07T12:11:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:11:49Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "betmgm",
        "title": "BetMGM",
        "last_update": "2023-03-07T12:15:04Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:04Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -135
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "superbook",
        "title": "SuperBook",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "betus",
        "title": "BetUS",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "pointsbetus",
        "title": "PointsBet (US)",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -130
              },
              {
                "name": "Memphis Grizzlies",
                "price": 110
              }
            ]
          }
        ]
      },
      {
        "key": "betrivers",
        "title": "BetRivers",
        "last_update": "2023-03-07T12:10:48Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:48Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -125
              },
              {
                "name": "Memphis Grizzlies",
                "price": 106
              }
            ]
          }
        ]
      },
      {
        "key": "sugarhouse",
        "title": "SugarHouse",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -125
              },
              {
                "name": "Memphis Grizzlies",
                "price": 106
              }
            ]
          }
        ]
      },
      {
        "key": "barstool",
        "title": "Barstool Sportsbook",
        "last_update": "2023-03-07T12:14:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:50Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -125
              },
              {
                "name": "Memphis Grizzlies",
                "price": 106
              }
            ]
          }
        ]
      },
      {
        "key": "twinspires",
        "title": "TwinSpires",
        "last_update": "2023-03-07T12:10:49Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:49Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -125
              },
              {
                "name": "Memphis Grizzlies",
                "price": 106
              }
            ]
          }
        ]
      },
      {
        "key": "unibet_us",
        "title": "Unibet",
        "last_update": "2023-03-07T12:10:50Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:10:50Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -125
              },
              {
                "name": "Memphis Grizzlies",
                "price": 106
              }
            ]
          }
        ]
      },
      {
        "key": "mybookieag",
        "title": "MyBookie.ag",
        "last_update": "2023-03-07T12:15:20Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:15:20Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -131
              },
              {
                "name": "Memphis Grizzlies",
                "price": 108
              }
            ]
          }
        ]
      },
      {
        "key": "foxbet",
        "title": "FOX Bet",
        "last_update": "2023-03-07T12:14:05Z",
        "markets": [
          {
            "key": "h2h",
            "last_update": "2023-03-07T12:14:05Z",
            "outcomes": [
              {
                "name": "Los Angeles Lakers",
                "price": -139
              },
              {
                "name": "Memphis Grizzlies",
                "price": 105
              }
            ]
          }
        ]
      }
    ]
  }
]
 "##,
            ),
            data_2: String::from(
                r##"
        {
    "id": "abe2c187d35b88402a28c99a113601e9",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-04T00:10:00Z",
    "home_team": "Charlotte Hornets",
    "away_team": "Orlando Magic",
    "bookmakers": [
        {
            "key": "draftkings",
            "title": "DraftKings",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -155
                        }
                    ]
                }
            ]
        },
        {
            "key": "mybookieag",
            "title": "MyBookie.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:05Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 139
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -172
                        }
                    ]
                }
            ]
        },
        {
            "key": "fanduel",
            "title": "FanDuel",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 142
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -168
                        }
                    ]
                }
            ]
        },
        {
            "key": "circasports",
            "title": "Circa Sports",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 142
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -173
                        }
                    ]
                }
            ]
        },
        {
            "key": "betrivers",
            "title": "BetRivers",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "sugarhouse",
            "title": "SugarHouse",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "barstool",
            "title": "Barstool Sportsbook",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "twinspires",
            "title": "TwinSpires",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:50:52Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "unibet_us",
            "title": "Unibet",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:40Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "foxbet",
            "title": "FOX Bet",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 138
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -182
                        }
                    ]
                }
            ]
        },
        {
            "key": "pointsbetus",
            "title": "PointsBet (US)",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        },
        {
            "key": "betus",
            "title": "BetUS",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 140
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -160
                        }
                    ]
                }
            ]
        },
        {
            "key": "williamhill_us",
            "title": "William Hill (US)",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -160
                        }
                    ]
                }
            ]
        },
        {
            "key": "bovada",
            "title": "Bovada",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        },
        {
            "key": "lowvig",
            "title": "LowVig.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:26Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 144
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -164
                        }
                    ]
                }
            ]
        },
        {
            "key": "betonlineag",
            "title": "BetOnline.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:04Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 144
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -164
                        }
                    ]
                }
            ]
        },
        {
            "key": "wynnbet",
            "title": "WynnBET",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -175
                        }
                    ]
                }
            ]
        },
        {
            "key": "betmgm",
            "title": "BetMGM",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -175
                        }
                    ]
                }
            ]
        },
        {
            "key": "superbook",
            "title": "SuperBook",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 150
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        }
    ]
}
        "##,
            ),
        }
    }
}
