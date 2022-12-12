// A and X for Rock, B and Y for Paper, C and Z for Scissors
fn main() -> std::io::Result<()> {
    let mut score: u64 = 0;
    let text = std::fs::read_to_string("input.txt")?;

    for line in text.lines() {
        if line.contains('X') {
            score += 1;
        } else if line.contains('Y') {
            score += 2;
        } else {
            score += 3;
        }
    }
    score += compute_single_game_result(&text);
    println!("{}", score);
    Ok(())
}

/// First if statement calculates draws (3 points)
///
/// Second if statement calculates wins (6 points)
///
/// No loses calculation because you don't win any points
fn compute_single_game_result(text: &str) -> u64 {
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
