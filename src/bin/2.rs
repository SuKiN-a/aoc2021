use std::{error::Error, fs};

fn main() {
    s1().unwrap();
    s2().unwrap();
}

fn s1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/2.input")?;
    let input = input
        .split('\n')
        .map(|f| f.split_once(' ').unwrap())
        .map(|f| (f.0, f.1.parse::<i32>().unwrap()));
    let result = input.fold(Position::default(), |mut acc, (direction, magnitude)| {
        match direction {
            "forward" => acc.horizontal += magnitude,
            "down" => acc.depth += magnitude,
            _ => acc.depth -= magnitude,
        }
        acc
    });
    println!("{}", result.horizontal * result.depth);
    Ok(())
}
#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn s2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/2.input")?;
    let input = input
        .split('\n')
        .map(|f| f.split_once(' ').unwrap())
        .map(|f| (f.0, f.1.parse::<i32>().unwrap()));
    let result = input.fold(Position::default(), |mut acc, (direction, magnitude)| {
        match direction {
            "forward" => {
                acc.horizontal += magnitude;
                acc.depth += acc.aim * magnitude
            }
            "down" => acc.aim += magnitude,
            _ => acc.aim -= magnitude,
        }
        acc
    });
    println!("{}", result.horizontal * result.depth);
    Ok(())
}
