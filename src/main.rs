#![allow(dead_code, unused_mut, unused_variables)]

use std::io::{self, BufRead, Write};

const LOCKED: usize = 0;
const UNLOCKED: usize = 1;
const STATES_COUNT: usize = 2;
const PUSH: usize = 0;
const COIN: usize = 1;
const EVENTS_COUNT: usize = 2;

const FSM: [[usize; EVENTS_COUNT]; STATES_COUNT] = [
    // LOCKED 0
    [LOCKED, UNLOCKED],
    [LOCKED, UNLOCKED],
];

fn next_state(state: usize, event: usize) -> usize {
    FSM[state][event]
}

fn state_to_str(state: usize) -> &'static str {
    match state {
        LOCKED => "Locked",
        UNLOCKED => "Unlocked",
        _ => unreachable!(),
    }
}

fn main() {
    let mut state = 0;

    println!("State is Locked\nq => Quit\ncoin => Unlock\npush => Lock");

    print!("> ");
    io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        match line.unwrap().trim() {
            "q" => {
                println!("Quiting");
                return;
            }
            "push" => {
                if state != LOCKED {
                    state = next_state(state, PUSH);
                    println!("State {}", state_to_str(state));
                }
            }
            "coin" => {
                if state != UNLOCKED {
                    state = next_state(state, COIN);
                    println!("State {}", state_to_str(state));
                }
            }
            _ => eprintln!("ERROR: Uknown event"),
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }
}
