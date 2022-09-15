use std::{
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    // str::FromStr,
};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr)]
#[display("{action} {movement}")]
struct Step {
    action: Action,
    movement: u32,
}

#[derive(Display, FromStr)]
enum Action {
    #[display("forward")]
    F,
    #[display("down")]
    D,
    #[display("up")]
    U,
}

fn read_inputs() -> io_result<Vec<Step>> {
    let input_file = File::open("inputs/2.txt")?;
    let file_reader = BufReader::new(input_file);
    let inputs = file_reader
        .lines()
        .filter_map(|line| {
            let line_res = line.ok()?;
            // empty line
            if line_res.is_empty() { return None; }
            let res: Step = line_res.parse().expect("incorrect input line");
            Some(res)
        })
        .collect::<Vec<Step>>();
    return Ok(inputs);
}

fn part1(inputs: &Vec<Step>) -> u32 {
    let (mut horizontal, mut depth): (u32, u32) = (0, 0);

    for step in inputs {
        match step.action {
            Action::F => {
                horizontal += step.movement;
            }
            Action::D => {
                depth += step.movement;
            }
            Action::U => {
                depth -= step.movement;
            }
        }
    }

    return horizontal * depth;
}

fn main() {
    let inputs = read_inputs().expect("Input read correctly");
    let answer = part1(&inputs);
    println!("Answer: {}", answer)
}
