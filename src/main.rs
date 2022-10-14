#![allow(dead_code, unused_mut, unused_variables)]

macro_rules! transition {
    ($($str:literal => $state: literal), +) => {
        vec![$(Transition {
            events: $str,
            next_state: $state,
        }), +]
    };
}

// For every ascii char we have a number that tells us
// If we found that char to which state we should transition
struct State {
    ts: [usize; 127],
}

impl State {
    fn new(settings: Option<Vec<Transition>>) -> Self {
        let mut state = Self { ts: [0; 127] };
        match settings {
            Some(str) => {
                state.set(str);
                state
            }
            None => state,
        }
    }

    fn set(&mut self, transitions: Vec<Transition>) -> () {
        for tr in transitions {
            let mut chars: Vec<usize> = tr.events.chars().map(|c| c as usize).collect();
            for c in chars.into_iter() {
                self.ts[c] = tr.next_state;
            }
        }
    }

    fn print_pattern(&self) -> () {
        for s in self.ts {
            print!("{}", s);
        }
        println!();
    }
}

type FSM = Vec<State>;

struct Transition<'str> {
    events: &'str str,
    next_state: usize,
}

fn match_str(string: &str, fsm: &FSM) -> (isize, isize) {
    let mut current_state = 0;
    let final_state = fsm.len() - 1;
    let mut start_index = 0;
    let mut end_index = 0;
    let mut chars: Vec<usize> = string.chars().map(|c| c as usize).collect();
    for c in chars.into_iter() {
        current_state = fsm.get(current_state).unwrap().ts[c];
        if current_state == final_state {
            return (start_index, end_index);
        }

        if current_state == 0 {
            start_index += 1;
        }

        end_index += 1;
    }

    return (-1, -1);
}

fn main() {
    let mut fsm = FSM::new();

    // This will match a(f)*bc

    fsm.push(State::new(Some(transition!("a" => 1))));
    fsm.push(State::new(Some(transition!("f" => 1, "b" => 2))));
    fsm.push(State::new(Some(transition!("c" => 3))));
    fsm.push(State::new(None)); // This is the final state

    println!("Pattern is \"a(f)*bc\"");
    println!("Masks: ");
    println!("=================================");
    for state in &fsm {
        state.print_pattern();
    }
    println!("=================================");

    let str = "dddabcdd";
    let (s, e) = match_str(str, &fsm);
    if s > 0 && e > 0 {
        println!("Pattern {:?} is starting at {} and ending at {}", str, s, e);
    } else {
        println!("Pattern not found");
    }

    let str = "dddaffbcdd";
    let (s, e) = match_str(str, &fsm);
    if s > 0 && e > 0 {
        println!("Pattern {:?} is starting at {} and ending at {}", str, s, e);
    } else {
        println!("Pattern not found");
    }

    let str = "gghhhlll";
    let (s, e) = match_str(str, &fsm);
    if s > 0 && e > 0 {
        println!("Pattern {:?} is starting at {} and ending at {}", str, s, e);
    } else {
        println!("Pattern not found");
    }
}
