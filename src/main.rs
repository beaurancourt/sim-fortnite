use rand::{thread_rng, Rng};
use std::collections::HashMap;
use rand::distributions::{Normal, Distribution};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
struct Score {
    points: u32,
    placement: u32,
    elims: u32
}

#[derive(Debug, Clone, Copy)]
struct Player {
    id: u32,
    player_elo: f64,
    current_elo: f64
}

#[derive(Debug, Clone, Copy)]
struct FightResult {
    winner: Player,
    loser: Player
}

fn fight(player1: Player, player2: Player) -> FightResult {
    let transformed_rating_1 = 10f64.powf(player1.player_elo as f64 / 400.0);
    let transformed_rating_2 = 10f64.powf(player2.player_elo as f64 / 400.0);
    let player1_winrate = transformed_rating_1 / (transformed_rating_1 + transformed_rating_2);
    let random_number: f64 = thread_rng().gen();
    if random_number <= player1_winrate {
        FightResult{winner: player1, loser: player2}
    } else {
        FightResult{winner: player2, loser: player1}
    }
}

fn play_match(players: Vec<Player>) -> Vec<FightResult> {
    let mut alive_players = players.clone();
    let mut match_history: Vec<FightResult> = Vec::new();
    while alive_players.len() > 1 {
        let player1index = thread_rng().gen_range(0, alive_players.len());
        let player1 = alive_players.swap_remove(player1index);
        let player2index = thread_rng().gen_range(0, alive_players.len());
        let player2 = alive_players.swap_remove(player2index);
        let result = fight(player1, player2);
        alive_players.push(result.winner);
        match_history.push(result);
    }
    match_history
}

fn points_for_placement(placement: u32) -> u32 {
    if placement == 1 {
        5
    } else if placement <= 3 {
        2
    } else if placement <= 10 {
        1
    } else {
        0
    }
}

fn points_for_elims(elims: u32) -> u32 {
    if elims >= 7 {
        3
    } else if elims >= 5 {
        2
    } else if elims >= 3 {
        1
    } else {
        0
    }
}

fn score_match(history: Vec<FightResult>) -> Vec<(Player, Score)> {
    let mut scores: HashMap<u32, Score> = HashMap::new();
    let mut players: HashMap<u32, Player> = HashMap::new();
    for (index, fight_result) in history.iter().enumerate() {
        match scores.get(&fight_result.winner.id) {
            Some(score) => {
                let new_score = Score{ elims: score.elims + 1, ..*score };
                scores.insert(fight_result.winner.id, new_score);
                players.insert(fight_result.winner.id, fight_result.winner);
            },
            None => {
                let score = Score{ elims: 1, points: 0, placement: 0 };
                scores.insert(fight_result.winner.id, score);
                players.insert(fight_result.winner.id, fight_result.winner);
            }
        }
        match scores.get(&fight_result.loser.id) {
            Some(score) => {
                let placement = history.len() as u32 - index as u32 + 1;
                let points = points_for_elims(score.elims) + points_for_placement(placement);
                let new_score = Score{placement: placement, points: points, elims: score.elims};
                scores.insert(fight_result.loser.id, new_score);
                players.insert(fight_result.loser.id, fight_result.loser);
            },
            None => {
                let placement = history.len() as u32 - index as u32 + 1;
                let points = points_for_placement(placement);
                let score = Score{placement: placement, elims: 0, points: points};
                scores.insert(fight_result.loser.id, score);
                players.insert(fight_result.loser.id, fight_result.loser);
            }
        }
    }
    if history.len() > 0 {
        let last_match = history[history.len() - 1];
        match scores.get(&last_match.winner.id) {
            Some(score) => {
                let points = points_for_elims(score.elims) + points_for_placement(1);
                let new_score = Score{placement: 1,  points: points, ..*score};
                scores.insert(last_match.winner.id, new_score);
            },
            None => {}
        }
    }
    let mut results = Vec::new();
    for (player_id, score) in scores {
        match players.get(&player_id) {
            Some(player) => {
                results.push((*player, score));
            },
            None => {}
        }
    };
    results.sort_by(|a, b| b.1.points.cmp(&a.1.points));
    results
}

fn play_tournament(players: Vec<Player>) {
    let mut special_points = 0;
    let n = 1000;
    for _ in 0..n {
        let history = play_match(players.clone());
        let score = score_match(history);
        for (player, player_score) in score {
            if (player.id == 100) {
                special_points = special_points + player_score.points;
            }
        }
    }
    println!("{}", special_points as f64 / n as f64);
}

fn sample_from_population(population: &mut Vec<Player>, n: usize) -> Vec<Player> {
    let mut rng = &mut thread_rng();
    population.choose_multiple(&mut rng, n).cloned().collect()
}

fn main() {
    let normal = Normal::new(1200.0, 0.0);
    let population_size = 10000;
    let mut population: Vec<Player> = (1..population_size).map(|x| {
        let player_elo = normal.sample(&mut thread_rng());
        Player{id: x, player_elo, current_elo: 1000f64}
    }).collect();
    let mut players = sample_from_population(&mut population, 99);
    players.push(Player{id: 100, player_elo: 1600f64, current_elo: 0f64});
    play_tournament(players);
}
