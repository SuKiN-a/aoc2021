use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    s1()?;
    s2()
}

fn s1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/1.input")?;
    let mut input = input.split('\n').map(|f| f.parse::<i32>().unwrap());
    let mut last = input.next().unwrap();
    let mut num_depth_incr = 0;
    for i in input {
        if i > last {
            num_depth_incr += 1;
        }
        last = i;
    }
    println!("{}", num_depth_incr);
    Ok(())
}

fn s2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/1.input")?;
    let collect = input
        .split('\n')
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut input = collect.windows(3).map(|f| f.iter().sum::<i32>());
    let mut last = input.next().unwrap();
    let mut num_depth_incr = 0;
    for i in input {
        if i > last {
            num_depth_incr += 1;
        }
        last = i;
    }
    println!("{}", num_depth_incr);
    Ok(())
}
