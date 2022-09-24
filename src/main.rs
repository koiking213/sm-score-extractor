use itertools::Itertools;
use roxmltree;
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::fs;
use std::str::FromStr;

const TARGET_SONG_PATH_PREFIX: &str = "Songs/stepmania_edit";

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Difficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
    Challenge,
    Edit,
}

impl FromStr for Difficulty {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Beginner" => Ok(Difficulty::Beginner),
            "Easy" => Ok(Difficulty::Easy),
            "Medium" => Ok(Difficulty::Medium),
            "Hard" => Ok(Difficulty::Hard),
            "Challenge" => Ok(Difficulty::Challenge),
            "Edit" => Ok(Difficulty::Edit),
            _ => Err(format!("{} is not supported", s)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Score {
    score: i32,
    player: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Chart {
    difficulty: Difficulty,
    scores: Vec<Score>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Song {
    name: String,
    charts: Vec<Chart>,
}


#[derive(Debug, Deserialize, Serialize)]
struct Player {
    id: String,
    name: String
}
#[derive(Debug, Deserialize, Serialize)]
struct Players {
    players: Vec<Player>
}

fn id_to_player(players: &Players, id: &str) -> String {
    if let Some(player ) = players.players.iter().find(|p| p.id == id) {
        player.name.clone()
    } else {
        "unknown player".to_string()
    }

}
fn main() {
    // open player ID file
    let str = fs::read_to_string("ids.json").expect("Unable to read ID file");
    let players = serde_json::from_str::<Players>(&str).unwrap();
    
    // create json
    let str = fs::read_to_string("Stats.xml").unwrap();
    let doc = roxmltree::Document::parse(&str).unwrap();
    let node = doc.root_element();
    let mut songs: Vec<Song> = Vec::new();
    for song in node.descendants().filter(|n| n.tag_name().name() == "Song") {
        if !song
            .attribute("Dir")
            .unwrap()
            .starts_with(TARGET_SONG_PATH_PREFIX)
        {
            continue;
        }
        let song_name = song.attribute("Dir").unwrap();
        let mut song_json = Song {
            name: song_name.to_string(),
            charts: Vec::new(),
        };
        for steps in song
            .descendants()
            .filter(|n| n.tag_name().name() == "Steps")
        {
            let difficulty_str = steps.attribute("Difficulty").unwrap();
            let mut chart_json = Chart {
                difficulty: Difficulty::from_str(difficulty_str).unwrap(),
                scores: Vec::new(),
            };
            let mut scores: Vec<Score> = Vec::new();
            for high_score in steps
                .descendants()
                .filter(|n| n.tag_name().name() == "HighScore")
            {
                let score_value = high_score
                    .children()
                    .find(|n| n.tag_name().name() == "Score")
                    .unwrap();
                let score_num: i32 = score_value.text().unwrap().parse().unwrap();
                let player = high_score
                    .children()
                    .find(|n| n.tag_name().name() == "PlayerGuid")
                    .unwrap();
                if let Some(player_guid) = player.text() {
                    let player_name = id_to_player(&players, player_guid);
                    let score = Score {
                        score: score_num,
                        player: player_name.to_string(),
                    };
                    scores.push(score);
                }
            }
            // find highest score by player
            for (key, group) in scores.iter().into_group_map_by(|s| &s.player) {
                let highest_score = group.iter().max().unwrap();
                chart_json.scores.push((*highest_score).clone());
            }
            song_json.charts.push(chart_json);
        }
        songs.push(song_json);
    }
    let json = serde_json::to_string(&songs).unwrap();
    fs::write("scores.json", json).unwrap();
}
