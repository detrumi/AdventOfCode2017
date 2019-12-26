use num_bigint::BigUint;
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
    eprintln!("cards[0] = {:?}", cards[0]);
    cards = shuffle(cards, &techniques);
    eprintln!("cards[0] = {:?}", cards[0]);
    cards = shuffle(cards, &techniques);
    eprintln!("cards[0] = {:?}", cards[0]);

    // let mut value = 2019;
    // for _ in 0..10 {
    //     value = work_forward(10_007, &techniques, value);
    //     eprintln!("value = {:?}", value);
    // }

    // eprintln!("cards = {:?}", cards);

    // part2();

    let num_cards = 119_315_717_514_047;
    // b = f(0), a = f(1) - f(0)
    for i in 0..10 {
        let value = work_forward(num_cards, &techniques, i);
        eprintln!("value = {:?}", value);
    }

    part2();

    // let val = BigInt::from(41790941595554_i64);
    // let expected1 = BigInt::from(44653370793617_i64);
    // let expected2: BigInt = BigInt::from(16644877215659_i64);
    // let p = BigInt::from(num_cards);
    // for a in 2.. {
    //     if a % 1000 == 0 {
    //         eprintln!("a = {:?}", a);
    //     }
    //     let a = BigInt::from(a as i64);
    //     let ax = a.clone() * val.clone();
    //     for p_times in 0..=2 {
    //         let b: BigInt = expected1.clone() + BigInt::from(p_times) * p.clone() - ax.clone();
    //         let axb = (ax.clone() + b.clone()) % p.clone();
    //         if (a.clone() * axb + b.clone()) % p.clone() == expected2 {
    //             println!("a={},b={}", a, b);
    //             return;
    //         }
    //     }
    // }
}

// Applying ax+b n times = a^n * x + b * (a^n - 1) / (a - 1)
// f(x) = ax+b => f^-1(x) = (x-b)/a
// Applying n times => a^-n * (x - b) - b * ((a^(n+1) - 1) / (a - 1))
fn part2() {
    // let x = &BigUint::from(2020_usize);
    // let a = &BigUint::from(48116552563827_usize);
    // let b = &BigUint::from(5113249733551_usize);
    // let p = &BigUint::from(119315717514047_usize);
    // let n = &(p - BigUint::from(101741582076661_usize));
    // let n = &BigUint::from(101741582076661_usize);

    let x = &BigUint::from(0_usize);
    let a = &BigUint::from(3541_usize);
    let b = &BigUint::from(204_usize);
    let p = &BigUint::from(10007_usize);
    let n = &BigUint::from(2_usize);

    let one = &BigUint::from(1_usize);

    let a_to_n = &a.modpow(n, p);
    let division = moddiv(&(a_to_n - one), &(a - one), p);
    let result = (a_to_n * x + b * division) % p;

    // let a_to_minus_n = &moddiv(one, &a.modpow(n, p), p);
    // let a_to_n_plus_1 = &a.modpow(&(n + one), p);
    // let division = moddiv(&(a_to_n_plus_1 - one), &(a - one), p);
    // let result = (a_to_minus_n * (x + p - b) - b * division) % p;

    eprintln!("Part 2 = {}", result.to_string());
}

// (a/b) % p = ((a mod p) * (b^(p-2) mod p)) mod p
fn moddiv(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    let right = b.modpow(&(p - BigUint::from(2_usize)), p);
    (a * right) % p
}

fn work_forward(num_cards: usize, techniques: &[Technique], mut position: usize) -> usize {
    for technique in techniques {
        match technique {
            Technique::Deal => position = num_cards - position - 1,
            Technique::Cut(cut) if *cut >= 0 => {
                if position >= *cut as usize {
                    position -= *cut as usize;
                } else {
                    position += num_cards - *cut as usize;
                }
            }
            Technique::Cut(cut) => {
                let cut = -cut as usize;
                if position >= num_cards - cut {
                    position -= num_cards - cut;
                } else {
                    position += cut;
                }
            }
            Technique::DealWithIncrement(increment) => {
                position *= increment;
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
