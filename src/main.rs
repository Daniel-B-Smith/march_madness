#[macro_use]
extern crate itertools;
extern crate threadpool;

use std::collections::HashMap;
use std::sync::mpsc::channel;

use threadpool::ThreadPool;

mod tournament;
use tournament::Tournament;

mod bracket;
use bracket::OwnedTournament;

fn run_public_tournaments(num_tourneys: u32) -> Vec<OwnedTournament> {
    let pool = ThreadPool::new(6);
    let (t_tourneys, r_tourneys) = channel();

    for _ in 0..num_tourneys {
        let t_tourneys = t_tourneys.clone();
        pool.execute(move || {
            let t = bracket::run_public_tournament();
            t_tourneys.send(t).unwrap()
        });
    }

    drop(t_tourneys);
    r_tourneys.into_iter().collect()
}

fn run_a_bunch_of_tournaments(num_tourneys: u32) -> Vec<Tournament> {
    let pool = ThreadPool::new(6);
    let (t_tourneys, r_tourneys) = channel();

    for _ in 0..num_tourneys {
        let t_tourneys = t_tourneys.clone();
        pool.execute(move || {
            let t = tournament::run_tournament();
            t_tourneys.send(t).unwrap()
        });
    }

    drop(t_tourneys);
    r_tourneys.into_iter().collect()
}

fn main() {
    let tourneys = run_public_tournaments(1000);
    let mut champion_counts = HashMap::new();
    for tourney in tourneys.iter() {
        *champion_counts.entry(&tourney.champion).or_insert(0) += 1;
    }
    for (champ, count) in champion_counts {
        println!("{} won {} times!", champ, count);
    }
    let mut sweet_sixteen_counts = HashMap::new();
    for tourney in tourneys.iter() {
        for team in tourney.sweet_sixteen.iter() {
            *sweet_sixteen_counts.entry(team).or_insert(0) += 1;
        }
    }
    for (team, count) in sweet_sixteen_counts {
        println!("{} advanced to the sweet sixteen {} times!", team, count);
    }
}
