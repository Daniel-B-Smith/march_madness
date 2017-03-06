extern crate rand;

use std::io::BufReader;
use std::io::BufRead;
use std::fmt;
use std::fs::File;
use std::mem;

use std::collections::HashMap;

use itertools::Itertools;
use self::rand::distributions::{IndependentSample, Range};

use tournament::Tournament;

pub struct OwnedTournament {
    pub first_round: Vec<String>,
    pub second_round: Vec<String>,
    pub sweet_sixteen: Vec<String>,
    pub elite_eight: Vec<String>,
    pub final_four: Vec<String>,
    pub championship: Vec<String>,
    pub champion: String,
}

impl OwnedTournament {
    pub fn new() -> OwnedTournament {
        OwnedTournament {
            first_round: vec![],
            second_round: vec![],
            sweet_sixteen: vec![],
            elite_eight: vec![],
            final_four: vec![],
            championship: vec![],
            champion: "".to_string(),
        }
    }
}

pub fn run_public_tournament() -> OwnedTournament {
    let teams = ["Kansas",
                 "Austin Peay",
                 "Colorado",
                 "UConn",
                 "Maryland",
                 "South Dakota St",
                 "Cal",
                 "Hawaii",
                 "Arizona",
                 "Wichita St.",
                 "Miami",
                 "Buffalo",
                 "Iowa",
                 "Temple",
                 "Villanova",
                 "UNC Asheville",
                 "Oregon",
                 "Holy Cross",
                 "Saint Joe's",
                 "Cincinnati",
                 "Baylor",
                 "Yale",
                 "Duke",
                 "UNC Wilmington",
                 "Texas",
                 "Northern Iowa",
                 "Texas A&M",
                 "Green Bay",
                 "Oregon State",
                 "VCU",
                 "Oklahoma",
                 "CSU Bakersfield",
                 "UNC",
                 "FGCU",
                 "USC",
                 "Providence",
                 "Indiana",
                 "Chattanooga",
                 "Kentucky",
                 "Stony Brook",
                 "Notre Dame",
                 "Michigan",
                 "West Virginia",
                 "SF Austin",
                 "Wisconsin",
                 "Pitt",
                 "Xavier",
                 "Weber State",
                 "UVA",
                 "Hampton",
                 "Texas Tech",
                 "Butler",
                 "Purdue",
                 "AR-Little Rock",
                 "Iowa State",
                 "Iona",
                 "Seton Hall",
                 "Gonzaga",
                 "Utah",
                 "Fresno State",
                 "Dayton",
                 "Syracuse",
                 "Michigan State",
                 "Mid Tennessee"]
        .iter()
        .map(|team_name| team_name.to_string())
        .collect::<Vec<String>>();
    let round_pools = RoundPools::new(&teams);

    owned_tournament_from_round_pools(&round_pools)
}


#[derive(Clone, Debug)]
struct RoundWinProbability<'a> {
    team_name: &'a String,
    cum_win_prob: f64,
}

fn owned_tournament_from_round_pools(round_pools: &RoundPools) -> OwnedTournament {
    let mut tournament = OwnedTournament::new();
    let mut rng = rand::thread_rng();

    let mut winners = Vec::new();
    winners = choose_winners(&round_pools.championship, winners, &mut rng);
    tournament.champion = winners[0].1.to_string();

    tournament.championship = run_round(&round_pools.final_four, &mut winners, &mut rng);
    tournament.final_four = run_round(&round_pools.elite_eight, &mut winners, &mut rng);
    tournament.elite_eight = run_round(&round_pools.sweet_sixteen, &mut winners, &mut rng);
    tournament.sweet_sixteen = run_round(&round_pools.second_round, &mut winners, &mut rng);
    tournament.second_round = run_round(&round_pools.first_round, &mut winners, &mut rng);

    tournament
}

fn run_round<'a, 'b>(pools: &'a Vec<Vec<RoundWinProbability<'b>>>,
                     mut winners: &mut Vec<(usize, &'a String)>,
                     mut rng: &mut rand::Rng)
                     -> Vec<String> {
    let mut round = Vec::new();
    let mut new_winners = Vec::new();
    mem::swap(&mut new_winners, &mut winners);
    *winners = choose_winners(&pools, new_winners, &mut rng);
    for winner in winners {
        round.push(winner.1.to_string());
    }
    round
}

fn choose_winners<'a, 'b>(pools: &'a Vec<Vec<RoundWinProbability<'b>>>,
                          winners: Vec<(usize, &'a String)>,
                          mut rng: &mut rand::Rng)
                          -> Vec<(usize, &'a String)> {
    let mut new_winners = Vec::new();
    let mut win_iter = winners.iter();
    let mut winner = win_iter.next();
    for (index, pool) in pools.iter().enumerate() {
        if round_has_winner(&winner, index) {
            new_winners.push((index, winner.unwrap().1));
            winner = win_iter.next();
            continue;
        }
        new_winners.push((index, choose_winner(pool, &mut rng)));
    }

    new_winners
}

fn round_has_winner(optional: &Option<&(usize, &String)>, compare: usize) -> bool {
    match *optional {
        Some(value) => value.0 / 2 == compare,
        None => false,
    }
}

fn choose_winner<'a, 'b>(pool: &'a Vec<RoundWinProbability<'b>>,
                         mut rng: &mut rand::Rng)
                         -> &'b String {
    let between = Range::new(0f64, 1.);
    let win_prob = between.ind_sample(&mut rng);
    for team in pool {
        if team.cum_win_prob > win_prob {
            return team.team_name;
        }
    }
    unreachable!()
}

struct RoundPools<'a> {
    pub first_round: Vec<Vec<RoundWinProbability<'a>>>,
    pub second_round: Vec<Vec<RoundWinProbability<'a>>>,
    pub sweet_sixteen: Vec<Vec<RoundWinProbability<'a>>>,
    pub elite_eight: Vec<Vec<RoundWinProbability<'a>>>,
    pub final_four: Vec<Vec<RoundWinProbability<'a>>>,
    pub championship: Vec<Vec<RoundWinProbability<'a>>>,
}

impl<'a> RoundPools<'a> {
    pub fn new(teams: &'a [String]) -> RoundPools<'a> {
        let mut round_pools = RoundPools {
            first_round: vec![],
            second_round: vec![],
            sweet_sixteen: vec![],
            elite_eight: vec![],
            final_four: vec![],
            championship: vec![],
        };

        let round_prob_map = get_round_probabilities();
        for round in 0..6u32 {
            let mut round_data = get_mut_round(&mut round_pools, round);
            for chunk in teams.chunks(2usize.pow(round + 1)) {
                let mut total_prob = 0f64;
                for team_name in chunk {
                    total_prob += calculate_round_prob(&round_prob_map, &team_name, round);
                }
                round_data.push(Vec::new());
                let mut cum_prob = 0f64;
                for team_name in chunk {
                    cum_prob += calculate_round_prob(&round_prob_map, &team_name, round) /
                                total_prob;
                    round_data.last_mut()
                        .unwrap()
                        .push(RoundWinProbability {
                            team_name: team_name,
                            cum_win_prob: cum_prob,
                        });
                }
            }
        }
        round_pools
    }
}

fn get_mut_round<'a, 'b>(round_pools: &'a mut RoundPools<'b>,
                         round: u32)
                         -> &'a mut Vec<Vec<RoundWinProbability<'b>>> {
    match round {
        0 => &mut round_pools.first_round,
        1 => &mut round_pools.second_round,
        2 => &mut round_pools.sweet_sixteen,
        3 => &mut round_pools.elite_eight,
        4 => &mut round_pools.final_four,
        5 => &mut round_pools.championship,
        _ => unreachable!(),
    }
}

fn calculate_round_prob(round_probs: &HashMap<(String, u32), f64>,
                        team_name: &String,
                        round: u32)
                        -> f64 {
    // Get the probability of reaching the round.
    let mut prob = *round_probs.get(&(team_name.to_string(), round)).unwrap();
    // Subtract off the probability of the subsequent round to correct for sampling from the
    // champion backwards.
    if round < 5 {
        prob -= *round_probs.get(&(team_name.to_string(), round + 1)).unwrap();
    }

    prob
}

fn get_round_probabilities() -> HashMap<(String, u32), f64> {
    let mut map: HashMap<(String, u32), f64> = HashMap::new();

    let f = File::open("espn.data").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let unwrapped = line.unwrap();
        let pieces: Vec<_> = unwrapped.split(",").collect();
        map.insert((pieces[0].to_string(), pieces[1].parse::<u32>().unwrap()),
                   pieces[2].parse::<f64>().unwrap());
    }
    map
}

fn print_round(f: &mut fmt::Formatter, round: &Vec<String>) -> fmt::Result {
    for mut chunk in &round.iter().chunks(2) {
        try!(writeln!(f, "{} - {}", chunk.next().unwrap(), chunk.next().unwrap()));
    }
    Ok(())
}

macro_rules! print_round {
    ($f:expr, $round_name:tt, $round_data:expr) => (
        try!(writeln!($f, $round_name));
        try!(writeln!($f, "--------------"));
        try!(print_round($f, $round_data));
        try!(writeln!($f, "--------------"));)
}

impl fmt::Display for OwnedTournament {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "--------------"));
        print_round!(f, "First Round:", &self.first_round);
        print_round!(f, "Second Round:", &self.second_round);
        print_round!(f, "Sweet Sixteen:", &self.sweet_sixteen);
        print_round!(f, "Elite Eight:", &self.elite_eight);
        print_round!(f, "Final Four:", &self.final_four);
        print_round!(f, "Championship:", &self.championship);
        try!(writeln!(f, "Your champion is: {}.", &self.champion));
        Ok(())
    }
}
