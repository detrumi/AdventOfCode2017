use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

enum Technique {
    Deal,
    Cut(i32),
    DealWithIncrement(usize),
}

fn main() {
    let file = File::open("input/day_22.txt").unwrap();
    let techniques: Vec<Technique> = io::BufReader::new(file)
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

    eprintln!(
        "Part 2 test = {} (should be 2019)",
        work_back(cards.len(), &techniques, part1)
    );

    let mut number = 2020;
    let mut found: HashSet<usize> = HashSet::new();
    for i in 0..100 {
        number = work_back(119_315_717_514_047, &techniques, number);
        // number = work_back(10_007, &techniques, number);
        let is_new = found.insert(number);
        println!("{}: number = {} {}", i, number, is_new);
        if !is_new {
            return;
        }
    }
}

fn work_back(num_cards: usize, techniques: &[Technique], mut position: usize) -> usize {
    for technique in techniques.iter().rev() {
        match technique {
            Technique::Deal => position = num_cards - position - 1,
            Technique::Cut(cut) if *cut >= 0 => {
                if position >= num_cards - *cut as usize {
                    position -= num_cards - *cut as usize;
                } else {
                    position += *cut as usize;
                }
            }
            Technique::Cut(cut) => {
                let cut = -cut as usize;
                if position >= cut {
                    position -= cut;
                } else {
                    position += num_cards - cut;
                }
            }
            Technique::DealWithIncrement(increment) => {
                let mut num = 0;
                let mut base = 0_u128;
                while num != 1 {
                    let additions = std::cmp::max((num_cards - num) / increment, 1);
                    num += additions * increment;
                    num %= num_cards;
                    base += additions as u128;
                }
                position = ((position as u128 * base) % num_cards as u128) as usize;
            }
        }
    }
    position
}

fn shuffle(mut cards: Vec<usize>, techniques: &[Technique]) -> Vec<usize> {
    let num_cards = cards.len();
    for technique in techniques {
        match technique {
            Technique::Deal => cards = cards.into_iter().rev().collect(),
            Technique::Cut(cut) => {
                if *cut >= 0 {
                    let mut new_cards: Vec<usize> =
                        cards.iter().skip(*cut as usize).copied().collect();
                    new_cards.extend(cards.drain(0..*cut as usize));
                    cards = new_cards;
                } else {
                    let mut new_cards: Vec<usize> = cards
                        .iter()
                        .skip((num_cards as i32 + cut) as usize)
                        .copied()
                        .collect();
                    new_cards.extend(cards.drain(0..(num_cards as i32 + cut) as usize));
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
