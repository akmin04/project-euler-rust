use project_euler::test_solution;

static INPUT: u32 = 100;

fn solution() -> u32 {
    (INPUT * (INPUT + 1) / 2).pow(2) - (1..=INPUT).fold(0, |total, cur| cur.pow(2) + total)
}

test_solution!("25164150", solution);