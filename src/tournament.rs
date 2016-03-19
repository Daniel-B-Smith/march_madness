extern crate rand;

use std::collections::HashMap;
use std::fmt;

use itertools::Itertools;

pub struct Tournament {
    rounds_completed: u8,
    win_probabilities: HashMap<(&'static str, &'static str), f64>,
    first_round: Vec<&'static str>,
    second_round: Option<Vec<&'static str>>,
    sweet_sixteen: Option<Vec<&'static str>>,
    elite_eight: Option<Vec<&'static str>>,
    final_four: Option<Vec<&'static str>>,
    championship: Option<Vec<&'static str>>,
    champion: Option<&'static str>,
}

// The maximum number of rounds a tournament can go.
const MAX_ROUNDS: u8 = 6;

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
        let mut win_probabilities = HashMap::new();
        for (team1, team2) in iproduct!(&teams, &teams) {
            win_probabilities.insert((*team1, *team2), 0.5);
        }
        Tournament {
            rounds_completed: 0,
            win_probabilities: win_probabilities,
            first_round: teams,
            second_round: None,
            sweet_sixteen: None,
            elite_eight: None,
            final_four: None,
            championship: None,
            champion: None,
        }
    }

    pub fn run_tournament(&mut self) {
        // Don't try to run the full tournament if it's already started.
        if self.rounds_completed > 0 {
            return;
        }
        while self.rounds_completed < MAX_ROUNDS {
            self.run_next_round();
        }
    }

    fn run_next_round(&mut self) {
        match self.rounds_completed {
            0 => {
                self.second_round = Some(advance_round(&self.first_round, &self.win_probabilities))
            }
            1 => {
                self.sweet_sixteen = Some(advance_round(self.second_round.as_ref().unwrap(),
                                                        &self.win_probabilities))
            }
            2 => {
                self.elite_eight = Some(advance_round(self.sweet_sixteen.as_ref().unwrap(),
                                                      &self.win_probabilities))
            }
            3 => {
                self.final_four = Some(advance_round(self.elite_eight.as_ref().unwrap(),
                                                     &self.win_probabilities))
            }
            4 => {
                self.championship = Some(advance_round(self.final_four.as_ref().unwrap(),
                                                       &self.win_probabilities))
            }
            5 => {
                self.champion =
                    Some(advance_round(self.championship.as_ref().unwrap(),
                                       &self.win_probabilities)[0])
            }
            _ => panic!("Tried to advance the round of a completed tournament"),
        }
        self.rounds_completed += 1;
    }
}

fn advance_round<'a>(round: &Vec<&'a str>,
                     win_probabilities: &HashMap<(&'a str, &'a str), f64>)
                     -> Vec<&'a str> {
    let mut next_round = Vec::new();
    for mut chunk in &round.iter().chunks_lazy(2) {
        let team1 = chunk.next().unwrap();
        let team2 = chunk.next().unwrap();
        if *win_probabilities.get(&(*team1, *team2)).unwrap() > rand::random() {
            next_round.push(*team1)
        } else {
            next_round.push(*team2)
        }
    }
    next_round
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
        match $round_data {
            Some(ref round_data) => try!(print_round($f, &round_data)),
            None => try!(writeln!($f, "Round not started.")),
        }
        try!(writeln!($f, "--------------"));)
}

impl fmt::Display for Tournament {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "--------------"));
        try!(writeln!(f, "First Round:"));
        try!(writeln!(f, "--------------"));
        try!(print_round(f, &self.first_round));
        try!(writeln!(f, "--------------"));
        print_round!(f, "Second Round:", self.second_round);
        print_round!(f, "Sweet Sixteen:", self.sweet_sixteen);
        print_round!(f, "Elite Eight:", self.elite_eight);
        print_round!(f, "Final Four:", self.final_four);
        print_round!(f, "Championship:", self.championship);
        match self.champion {
            Some(ref champion) => try!(writeln!(f, "Your champion is: {}.", champion)),
            None => try!(writeln!(f, "No champion crowned yet.")),
        }
        Ok(())
    }
}
