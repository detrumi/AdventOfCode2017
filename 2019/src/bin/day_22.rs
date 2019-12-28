use mod_exp::mod_exp;
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

    part1(&techniques);

    let num_cards = 10_007;
    let b = work_forward(num_cards, &techniques, 0);
    let a = work_forward(num_cards, &techniques, 1) - b;
    eprintln!(
        "(a, b, calculate(part1 as i128, a, b, num_cards, 1)) = {:?}",
        (a, b, calculate(part1 as i128, a, b, num_cards, 1))
    );
    assert!(calculate(part1 as i128, a, b, num_cards, 1) == 2019);

    part2(&techniques);
}

fn part1(techniques: &Vec<Technique>) {
    let num_cards: i128 = 10_007;
    let mut cards: Vec<usize> = (0..num_cards as usize).collect();
    cards = shuffle(cards, &techniques);
    let part1 = cards.iter().position(|n| *n == 2019).unwrap();
    eprintln!("Part 1 = {}", part1);
}

fn part2(techniques: &Vec<Technique>) {
    let num_cards: i128 = 119_315_717_514_047;
    let b = work_forward(num_cards, &techniques, 0); // 5_113_249_733_551
    let a = work_forward(num_cards, &techniques, 1) - b; // 48_116_552_563_827
    assert!((a * 2020 + b) % num_cards == work_forward(num_cards, &techniques, 2020));

    println!(
        "Part 2 = {}",
        calculate(2020, a, b, num_cards, 101_741_582_076_661)
    );
}

// Applying ax+b n times = a^n * x + b * (a^n - 1) / (a - 1)
fn calculate(x: i128, a: i128, b: i128, p: i128, n: i128) -> i128 {
    let a_to_n = mod_exp(a, n, p);
    let left = x * a_to_n;
    let right = b * moddiv(a_to_n - 1, a - 1, p);
    (left + right) % p
}

// (a/b) % p = ((a mod p) * (b^(p-2) mod p)) mod p
fn moddiv(a: i128, b: i128, p: i128) -> i128 {
    let right = mod_exp(b, p - 2, p);
    (a * right) % p
}

fn work_forward(num_cards: i128, techniques: &[Technique], mut position: i128) -> i128 {
    for technique in techniques {
        match technique {
            Technique::Deal => position = num_cards - position - 1,
            Technique::Cut(cut) if *cut >= 0 => {
                if position >= *cut as i128 {
                    position -= *cut as i128;
                } else {
                    position += num_cards - *cut as i128;
                }
            }
            Technique::Cut(cut) => {
                let cut = -cut as i128;
                if position >= num_cards - cut {
                    position -= num_cards - cut;
                } else {
                    position += cut;
                }
            }
            Technique::DealWithIncrement(increment) => {
                position *= *increment as i128;
                position %= num_cards;
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
