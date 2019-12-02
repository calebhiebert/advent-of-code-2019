use std::io::{Read, stdin};

enum Command {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    End,
    Unknown(i32),
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = parse_program(&buf);
            program[1] = noun;
            program[2] = verb;

            execute(&mut program);

            if program[0] == 19690720 {
                println!("Result {}", 100 * noun + verb);
                return
            }
        }
    }
}

fn parse_program(input: &str) -> Vec<i32> {
    input
        .split(",")
        .filter_map(|ln| {
            match ln.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_e) => None
            }
        })
        .collect()
}

fn execute(program: &mut Vec<i32>) {
    let mut i = 0usize;

    loop {
        let command = match program[i] {
            1 => Command::Add(program[program[i + 1] as usize], program[program[i + 2] as usize], program[i + 3] as usize),
            2 => Command::Multiply(program[program[i + 1] as usize], program[program[i + 2] as usize], program[i + 3] as usize),
            99 => Command::End,
            (c) => Command::Unknown(c)
        };

        match command {
            Command::Add(a, b, i) => {
                program[i] = a + b
            }
            Command::Multiply(a, b, i) => {
                program[i] = a * b
            }
            Command::End => {
                break;
            }
            Command::Unknown(c) => {
                println!("Unknown Command {}", c);
                break;
            }
        }

        i += 4;

        if i >= program.len() {
            break;
        }
    }
}

#[test]
fn test_execute() {
    let test_programs: Vec<(&str, &[i32])> = vec![
        ("1,0,0,0,99", &[2, 0, 0, 0, 99]),
        ("2,3,0,3,99", &[2, 3, 0, 6, 99]),
        ("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801]),
        ("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99])
    ];

    for (program, result) in test_programs {
        let mut prog = parse_program(program);
        execute(&mut prog);

        assert_eq!(prog, result)
    }
}
