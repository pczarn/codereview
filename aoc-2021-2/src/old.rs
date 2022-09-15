use std::{
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
};

enum Action {
    F,
    D,
    U,
}

fn read_inputs() -> io_result<Vec<(Action, u32)>> {
    let input_file = File::open("inputs/2.txt")?;
    let file_reader = BufReader::new(input_file);
    let inputs = file_reader
        .lines()
        .map(|line| {
            let line_res = line.unwrap();
            let mut res = line_res.split(" ");
            let (action, value) = (res.next().unwrap(), res.next().unwrap());
            match (res.next(), res.next(), res.next()) {
                // meaningful line
                (Some(action), Some(movement), None) => {}
                // incorrect input
                _ => panic!("incorrect input line: '{}'", line_res),
            }
            return {
                (
                    {
                        if action == "forward" {
                            Action::F
                        } else {
                            if action == "down" {
                                Action::D
                            } else {
                                Action::U
                            }
                        }
                    },
                    value.parse().unwrap(),
                )
            };
        })
        .collect::<Vec<(Action, u32)>>();
    return Ok(inputs);
}