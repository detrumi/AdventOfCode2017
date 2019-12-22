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

    techniques.reverse();

    let part2 = work_back(119_315_717_514_047, &techniques, 2020);
    eprintln!("Part 2 = {:?}", part2);
    eprintln!("cards[2020] = {:?}", cards[2020]);

    let num_cards = 10;
    let values = shuffle((0..num_cards).collect(), &techniques);
    for i in 0..num_cards {
        // eprintln!("{}: {}", i, values[i]);
        eprintln!(
            "{}: {}, {}",
            i,
            values[i],
            work_back(num_cards, &techniques, i)
        );
    }

    // for num in vec![1, 2, 10, 50, 9999, 10_006] {
    //     // for num in 1..=100 {
    //     let value = work_back(10_007, &techniques, num);
    //     println!(
    //         "{} should be {}, is {}, difference={}",
    //         num,
    //         cards[num],
    //         value,
    //         (value as i32 - cards[num] as i32)
    //     );
    // }

    // let mut cards: Vec<usize> = (0..119_315_717_514_047).collect();
}

fn work_back(num_cards: usize, techniques: &[Technique], mut position: usize) -> usize {
    for technique in techniques {
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
                if position > cut {
                    position -= cut;
                } else {
                    position += num_cards - cut;
                }
            }
            Technique::DealWithIncrement(increment) => {
                // let group = (position - 1) / increment;
                // let num_groups = f32::ceil((num_cards - 1) as f32 / *increment as f32) as usize;
                // let index = (position - 1) % increment;
                // eprintln!(
                //     "(num_groups, group, index, (increment - 1 - index)) = {:?}",
                //     (num_groups, group, index, (increment - 1 - index))
                // );
                // position = group + 1 + (increment - 1 - index) * increment;
                // position %= num_cards;
                // eprintln!("p={}, i={}", position, increment);
                // position = (position + increment * (position % increment)) / increment;
                // position = position / increment
                //     + increment * (increment - 1 - (position + increment - 1) % increment);

                position *= num_cards - increment;
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
