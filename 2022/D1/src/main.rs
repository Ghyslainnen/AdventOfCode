fn main() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("input.txt")?;
    let vec = sum_elf_calories(&contents);

    part_a(vec.clone());
    part_b(vec);
    Ok(())
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
    let max = vec.iter().max().expect("Expected a vector");
    println!("Part A = {max}");
}

fn part_b(vec: Vec<i32>) {
    let mut top_one = 0;
    let mut top_two = 0;
    let mut top_three = 0;

    for n in vec {
        if n > top_one {
            top_three = top_two;
            top_two = top_one;
            top_one = n;
        } else if n > top_two {
            top_three = top_two;
            top_two = n;
        } else if n > top_three {
            top_three = n;
        }
    }

    println!("Part B = {}", top_one + top_two + top_three);
}
