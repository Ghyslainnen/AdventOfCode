#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{fs::File, io::Read};

fn main() {
    let contents = read_file("input.txt").expect("Couldn't find file");
    let vec = sum_elf_calories(&contents);

    part_a(vec);
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn sum_elf_calories(contents: &str) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut num = 0;

    for line in contents.lines() {
        if line.is_empty() {
            vec.push(num);
            num = 0;
            continue;
        }
        let line: i32 = line.trim().parse().expect("Expected a number");
        num += line;
    }

    vec
}

fn part_a(vec: Vec<i32>) {
    let mut prev_n = 0;
    for n in vec {
        if n > prev_n {
            prev_n = n;
            continue;
        }
    }
    println!("Part A = {prev_n}");
}
