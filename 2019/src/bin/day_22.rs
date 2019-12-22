use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

enum Technique {
    Deal,
    Cut(i32),
    DealWithIncrement(usize),
}

fn main() {
    let file = File::open("input/day_22.txt").unwrap();
    let mut techniques: Vec<Technique> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let parts = l
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            if parts[1] == "into" {
                Technique::Deal
            } else if parts[0] == "cut" {
                Technique::Cut(parts[1].parse::<i32>().unwrap())
            } else if parts[2] == "increment" {
                let increment = parts[3].parse::<usize>().unwrap();
                Technique::DealWithIncrement(increment)
            } else {
                panic!()
            }
        })
        .collect();

    let mut cards: Vec<usize> = (0..10_007).collect();
    cards = shuffle(cards, &techniques);
    let part1 = cards.iter().position(|n| *n == 2019).unwrap();
    eprintln!("Part 1 = {}", part1);
    // eprintln!("cards = {:?}", cards);

    techniques.reverse();
    // let part2 = work_back(119_315_717_514_047, &lines, 2020);

    // let mut cards: Vec<usize> = (0..119_315_717_514_047).collect();
    // let mut cards: Vec<usize> = (0..30_000).collect();
    // cards = shuffle(cards, &lines);
    // eprintln!("cards = {:?}", cards);
}

fn work_back(num_cards: usize, techniques: &[Technique], position: usize) -> usize {
    panic!()
}

fn shuffle(mut cards: Vec<usize>, techniques: &[Technique]) -> Vec<usize> {
    let num_cards = cards.len();
    for technique in techniques {
        match technique {
            Technique::Deal => cards = cards.into_iter().rev().collect(),
            Technique::Cut(num) => {
                if *num >= 0 {
                    let mut new_cards: Vec<usize> =
                        cards.iter().skip(*num as usize).copied().collect();
                    new_cards.extend(cards.drain(0..*num as usize));
                    cards = new_cards;
                } else {
                    let mut new_cards: Vec<usize> = cards
                        .iter()
                        .skip((num_cards as i32 + num) as usize)
                        .copied()
                        .collect();
                    new_cards.extend(cards.drain(0..(num_cards as i32 + num) as usize));
                    cards = new_cards;
                }
            }
            Technique::DealWithIncrement(increment) => {
                let mut new_cards = cards.clone();

                let mut current = 0;
                for i in 0..num_cards {
                    new_cards[current] = cards[i];
                    current = (current + increment) % num_cards;
                }
                cards = new_cards;
            }
        }
    }
    cards
}
