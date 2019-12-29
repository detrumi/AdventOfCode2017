use mod_exp::mod_exp;
use std::fs::File;
use std::io::{self, BufRead};

enum Technique {
    Deal,
    Cut(i128),
    DealWithIncrement(i128),
}

fn main() {
    let file = File::open("input/day_22.txt").unwrap();
    let techniques: Vec<Technique> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let parts: Vec<String> = l.unwrap().split(' ').map(|s| s.to_string()).collect();
            if parts[1] == "into" {
                Technique::Deal
            } else if parts[0] == "cut" {
                Technique::Cut(parts[1].parse::<i128>().unwrap())
            } else if parts[2] == "increment" {
                Technique::DealWithIncrement(parts[3].parse::<i128>().unwrap())
            } else {
                panic!()
            }
        })
        .collect();

    let part1 = shuffle((0..10_007).collect(), &techniques)
        .iter()
        .position(|n| *n == 2019)
        .unwrap() as i128;
    println!("Part 1 = {}", part1);

    assert_eq!(calculate(part1, 10_007, &techniques, 1), 2019);

    let part2 = calculate(2020, 119_315_717_514_047, &techniques, 101_741_582_076_661);
    println!("Part 2 = {}", part2);
}

// Applying ax+b n times = a^n * x + b * (a^n - 1) / (a - 1)
fn calculate(x: i128, p: i128, techniques: &[Technique], n: i128) -> i128 {
    let (a, b) = find_formula(p, &techniques);
    let a_to_n = mod_exp(a, n, p);
    let left = x * a_to_n;
    let right = b * moddiv(a_to_n - 1, a - 1, p);
    (left + right) % p
}

// (a/b) % p = ((a mod p) * (b^(p-2) mod p)) mod p
fn moddiv(a: i128, b: i128, p: i128) -> i128 {
    (a * mod_exp(b, p - 2, p)) % p
}

// Reduces multiple shuffling techniques to (a,b) for which shuffling = a*x+b
fn find_formula(num_cards: i128, techniques: &[Technique]) -> (i128, i128) {
    let mut a = 1;
    let mut b = 0;
    for technique in techniques {
        match *technique {
            Technique::Deal => {
                a *= -1;
                a %= num_cards;

                b += a;
                b %= num_cards;
            }
            Technique::Cut(cut) => {
                b += a * cut;
                b %= num_cards;
            }
            Technique::DealWithIncrement(increment) => {
                a *= moddiv(1, increment, num_cards);
                a %= num_cards;
            }
        }
    }
    b = (b + num_cards) % num_cards;
    (a, b)
}

fn shuffle(mut cards: Vec<i128>, techniques: &[Technique]) -> Vec<i128> {
    let num_cards = cards.len();
    for technique in techniques {
        match *technique {
            Technique::Deal => cards = cards.into_iter().rev().collect(),
            Technique::Cut(cut) => {
                if cut >= 0 {
                    let mut new_cards: Vec<i128> =
                        cards.iter().skip(cut as usize).copied().collect();
                    new_cards.extend(cards.drain(0..cut as usize));
                    cards = new_cards;
                } else {
                    let mut new_cards: Vec<i128> = cards
                        .iter()
                        .skip(num_cards + cut as usize)
                        .copied()
                        .collect();
                    new_cards.extend(cards.drain(0..num_cards + cut as usize));
                    cards = new_cards;
                }
            }
            Technique::DealWithIncrement(increment) => {
                let mut new_cards = cards.clone();

                let mut current = 0;
                for i in 0..num_cards {
                    new_cards[current] = cards[i];
                    current = (current + increment as usize) % num_cards;
                }
                cards = new_cards;
            }
        }
    }
    cards
}
