use project_euler::test_solution;

static INPUT: u32 = 1000;

fn solution() -> u32 {
    (0..INPUT).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

test_solution!("233168", solution);