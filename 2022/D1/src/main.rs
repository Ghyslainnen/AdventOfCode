#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut vec: Vec<i32> = Vec::new();
    let mut num = 0;
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in contents.lines() {
        if line.is_empty() {
            vec.push(num);
            num = 0;
            continue;
        }
        let line: i32 = line.trim().parse().expect("Expected a number");
        num += line;
    }
    let mut prev_n = 0;
    for n in vec {
        if n > prev_n {
            prev_n = n;
            continue;
        }
    }
    println!("Part A = {prev_n}");
    Ok(())
}
