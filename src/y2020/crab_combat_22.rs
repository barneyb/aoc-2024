use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let decks = parse(input);
    tx.send(Part::Parse(format!(
        "{}, {}",
        decks[0].len(),
        decks[1].len()
    )))
    .unwrap();
    tx.send(Part::A(part_one(&decks[0], &decks[1]).to_string()))
        .unwrap();
    tx.send(Part::B(part_two(&decks[0], &decks[1]).to_string()))
        .unwrap();
}

#[derive(Clone, Debug)]
enum Player {
    A(VecDeque<usize>),
    B(VecDeque<usize>),
}

impl Player {
    fn score(&mut self) -> usize {
        calc_score(
            match self {
                Player::A(d) => d,
                Player::B(d) => d,
            }
            .make_contiguous(),
        )
    }
}

fn parse(input: &str) -> [Vec<usize>; 2] {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut switch = false;
    for l in input.lines() {
        if l == "" {
            switch = true
        } else if let Some('P') = l.chars().next() {
        } else {
            let n = l.parse().unwrap();
            if switch {
                b.push(n)
            } else {
                a.push(n)
            }
        }
    }
    [a, b]
}

fn part_one(deck_a: &[usize], deck_b: &[usize]) -> usize {
    let mut da: VecDeque<usize> = deck_a.iter().map(|&n| n).collect();
    let mut db: VecDeque<usize> = deck_b.iter().map(|&n| n).collect();
    let to_win = da.len() + db.len();
    loop {
        let a = da.pop_front().unwrap();
        let b = db.pop_front().unwrap();
        if a > b {
            da.push_back(a);
            da.push_back(b);
            if da.len() == to_win {
                break Player::A(da);
            }
        } else {
            db.push_back(b);
            db.push_back(a);
            if db.len() == to_win {
                break Player::B(db);
            }
        }
    }
    .score()
}

fn calc_score(deck: &[usize]) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

fn part_two(deck_a: &[usize], deck_b: &[usize]) -> usize {
    let mut counter = 0;
    let mut winner = play_a_game(deck_a, deck_b, 1, &mut counter);
    println!("\n\n== Post-game results ==\n{counter} games played",);
    match &winner {
        Player::A(d) => println!("Player 1's deck: {d:?}\nPlayer 2's deck: ..."),
        Player::B(d) => println!("Player 1's deck: ...\nPlayer 2's deck: {d:?}"),
    }
    winner.score()
}

fn play_a_game(deck_a: &[usize], deck_b: &[usize], depth: usize, counter: &mut usize) -> Player {
    *counter += 1;
    // let game = *counter;
    // println!("=== Game {game} (depth {depth}) ===\n");
    let mut da: VecDeque<usize> = deck_a.iter().map(|&n| n).collect();
    let mut db: VecDeque<usize> = deck_b.iter().map(|&n| n).collect();
    let to_win = da.len() + db.len();
    let mut visited = HashSet::new();
    // let mut round = 0;
    loop {
        // round += 1;
        // println!("-- Round {round} (Game {game}) --");
        // println!("Player 1's deck: {da:?}");
        // println!("Player 2's deck: {db:?}");
        if !visited.insert(build_state_id(&da, &db)) {
            // println!("State revisited after round {round} of game {game}; player 1 wins!");
            let winner = Player::A(da);
            return winner;
        }
        let a = da.pop_front().unwrap();
        let b = db.pop_front().unwrap();
        // println!("Player 1 plays: {a}");
        // println!("Player 2 plays: {b}");
        if if a <= da.len() && b <= db.len() {
            // println!("Playing a sub-game to determine the winner...\n");
            let suba = &da.make_contiguous()[0..a];
            let subb = &db.make_contiguous()[0..b];
            let did_a_win = match play_a_game(suba, subb, depth + 1, counter) {
                Player::A(_) => true,
                Player::B(_) => false,
            };
            // println!("\n...anyway, back to game {game}.");
            did_a_win
        } else {
            a > b
        } {
            // println!("Player 1 wins round {round} of game {game}!");
            da.push_back(a);
            da.push_back(b);
            if da.len() == to_win {
                // println!("The winner of game {game} is player 1!");
                return Player::A(da);
            }
        } else {
            // println!("Player 2 wins round {round} of game {game}!");
            db.push_back(b);
            db.push_back(a);
            if db.len() == to_win {
                // println!("The winner of game {game} is player 2!");
                return Player::B(db);
            }
        }
        // println!();
    }
}

fn build_state_id(da: &VecDeque<usize>, db: &VecDeque<usize>) -> Vec<usize> {
    let mut id = Vec::with_capacity(da.len() + db.len() + 1);
    id.extend(da);
    id.push(99999); // semaphore
    id.extend(db);
    id
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    const EXAMPLE_2: &str = r#"Player 1:
43
19

Player 2:
2
29
14"#;

    lazy_static! {
        static ref DECK_A: Vec<usize> = vec![9, 2, 6, 3, 1];
        static ref DECK_B: Vec<usize> = vec![5, 8, 4, 7, 10];
    }

    #[test]
    fn parse_example_1() {
        let decks = parse(EXAMPLE_1);
        assert_eq!(*DECK_A, decks[0]);
        assert_eq!(*DECK_B, decks[1]);
    }

    #[test]
    fn test_score_calculation() {
        assert_eq!(306, calc_score(&[3, 2, 10, 6, 8, 5, 9, 4, 7, 1]));
    }

    #[test]
    fn example_1() {
        assert_eq!(306, part_one(&DECK_A, &DECK_B));
        assert_eq!(291, part_two(&DECK_A, &DECK_B));
    }

    #[test]
    fn example_2() {
        let decks = parse(EXAMPLE_2);
        assert_eq!(105, part_two(&decks[0], &decks[1]));
    }

    /// This is the first game which cycles, but WITHOUT the initial state as
    /// part of it.
    #[ignore]
    #[test]
    fn game_17() {
        assert_eq!(272, part_two(&[1, 11, 6, 46], &[30, 25, 34]));
    }

    /// This is the super-expensive game, comprising over 80% of the total
    /// runtime. It's implicitly tested by the real input.
    #[ignore]
    #[test]
    fn game_851() {
        /*
        State revisited after round 193362; player 1 wins!
        15628 games played (14460 unique)
        */
        part_two(
            &[11, 49, 6, 38, 9],
            &[
                15, 2, 29, 4, 46, 27, 35, 10, 47, 16, 42, 25, 40, 8, 36, 20, 37, 22, 48, 24, 31,
                21, 41, 26, 44, 12, 45, 17,
            ],
        );
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2020, 22, do_solve).unwrap();
    }
}
