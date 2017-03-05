extern crate rand;

use std::collections::HashMap;
use std::fmt;

use itertools::Itertools;
use self::rand::distributions::{IndependentSample, Range};

pub struct Tournament {
    pub first_round: Vec<&'static str>,
    pub second_round: Vec<&'static str>,
    pub sweet_sixteen: Vec<&'static str>,
    pub elite_eight: Vec<&'static str>,
    pub final_four: Vec<&'static str>,
    pub championship: Vec<&'static str>,
    pub champion: &'static str,
}

pub fn run_tournament() -> Tournament {
    let mut tournament = Tournament::new();
    let team_comp = TeamComp::new(&tournament.first_round);
    let mut rng = rand::thread_rng();

    tournament.second_round = advance_round(&tournament.first_round, &team_comp, &mut rng);
    tournament.sweet_sixteen = advance_round(&tournament.second_round, &team_comp, &mut rng);
    tournament.elite_eight = advance_round(&tournament.sweet_sixteen, &team_comp, &mut rng);
    tournament.final_four = advance_round(&tournament.elite_eight, &team_comp, &mut rng);
    tournament.championship = advance_round(&tournament.final_four, &team_comp, &mut rng);
    tournament.champion = advance_round(&tournament.championship, &team_comp, &mut rng)[0];

    tournament
}

fn advance_round<'a>(round: &Vec<&'a str>,
                     team_comp: &TeamComp,
                     mut rng: &mut rand::Rng)
                     -> Vec<&'a str> {
    let mut next_round = Vec::new();
    let between = Range::new(0f64, 1.);
    for mut chunk in &round.iter().chunks_lazy(2) {
        let team1 = chunk.next().unwrap();
        let team2 = chunk.next().unwrap();
        if team_comp.win_probability(*team1, *team2) > between.ind_sample(&mut rng) {
            next_round.push(*team1)
        } else {
            next_round.push(*team2)
        }
    }
    next_round
}

pub struct TeamComp {
    win_probabilities: HashMap<(&'static str, &'static str), f64>,
}

impl TeamComp {
    pub fn new(teams: &Vec<&'static str>) -> TeamComp {
        let mut win_probabilities = HashMap::new();
        for (team1, team2) in iproduct!(teams, teams) {
            win_probabilities.insert((*team1, *team2), 0.5);
        }
        TeamComp { win_probabilities: win_probabilities }
    }

    pub fn win_probability(&self, team1: &str, team2: &str) -> f64 {
        *self.win_probabilities.get(&(team1, team2)).unwrap()
    }
}

impl Tournament {
    pub fn new() -> Tournament {
        let teams = vec!["Kansas",
                         "Austin Peay",
                         "Colorado",
                         "UConn",
                         "Maryland",
                         "S Dak St",
                         "California",
                         "Hawaii",
                         "Arizona",
                         "Wichita St",
                         "Miami (Fla)",
                         "Buffalo",
                         "Iowa",
                         "Temple",
                         "Villanova",
                         "UNC-Ashville",
                         "Oregon",
                         "Holy Cross",
                         "Saint Joe's",
                         "Cincinnati",
                         "Baylor",
                         "Yale",
                         "Duke",
                         "UNC-Wilm",
                         "Texas",
                         "N Iowa",
                         "Texas A&M",
                         "Green Bay",
                         "Oregon St",
                         "VCU",
                         "Oklahoma",
                         "Cal-Baker",
                         "N Carolina",
                         "FGCU",
                         "USC",
                         "Providence",
                         "Indiana",
                         "Chattanooga",
                         "Kentucky",
                         "Stony Brook",
                         "Notre Dame",
                         "Michigan",
                         "W Virginia",
                         "SF Austin",
                         "Wisconsin",
                         "Pittsburgh",
                         "Xavier",
                         "Weber St",
                         "Virginia",
                         "Hampton",
                         "Texas Tech",
                         "Butler",
                         "Purdue",
                         "Little Rock",
                         "Iowa St",
                         "Iona",
                         "Seton Hall",
                         "Gonzaga",
                         "Utah",
                         "Fresno St",
                         "Dayton",
                         "Syracue",
                         "Michigan St",
                         "Middle Tenn"];
        Tournament {
            first_round: teams,
            second_round: vec![],
            sweet_sixteen: vec![],
            elite_eight: vec![],
            final_four: vec![],
            championship: vec![],
            champion: "",
        }
    }
}

fn print_round(f: &mut fmt::Formatter, round: &Vec<&str>) -> fmt::Result {
    for mut chunk in &round.iter().chunks_lazy(2) {
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

impl fmt::Display for Tournament {
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
