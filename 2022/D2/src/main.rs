// A and X for Rock, B and Y for Paper, C and Z for Scissors
fn main() -> std::io::Result<()> {
    let mut score_a = 0;
    let mut score_b = 0;
    let text = std::fs::read_to_string("input.txt")?;

    score_a += compute_choice_points_part_a(&text);

    score_a += part_a(&text);
    println!("Part A = {}", score_a);
    score_b += part_b(&text);
    println!("Part B = {}", score_b);
    Ok(())
}

fn compute_choice_points_part_a(text: &str) -> u64 {
    let mut score = 0;

    for line in text.lines() {
        if line.contains('X') {
            score += 1;
        } else if line.contains('Y') {
            score += 2;
        } else if line.contains('Z') {
            score += 3;
        }
    }

    score
}

fn part_a(text: &str) -> u64 {
    let mut score: u64 = 0;
    for line in text.lines() {
        if (line.contains('A') && line.contains('X'))
            || (line.contains('B') && line.contains('Y'))
            || (line.contains('C') && line.contains('Z'))
        {
            score += 3;
        } else if (line.contains('A') && line.contains('Y'))
            || (line.contains('B') && line.contains('Z'))
            || (line.contains('C') && line.contains('X'))
        {
            score += 6;
        }
    }
    score
}
fn part_b(text: &str) -> u64 {
    let mut score: u64 = 0;
    for line in text.lines() {
        if line.contains('A') {
            if line.contains('X') {
                score += 3;
            } else if line.contains('Y') {
                score = score + 3 + 1;
            } else {
                score = score + 6 + 2;
            }
        } else if line.contains('B') {
            if line.contains('X') {
                score += 1;
            } else if line.contains('Y') {
                score = score + 3 + 2;
            } else {
                score = score + 6 + 3;
            }
        } else if line.contains('C') {
            if line.contains('X') {
                score += 2;
            } else if line.contains('Y') {
                score = score + 3 + 3;
            } else {
                score = score + 6 + 1;
            }
        }
    }
    score
}
