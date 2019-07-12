use project_euler::test_solution;

static INPUT: u32 = 20;

fn solution() -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    (1..=INPUT).fold(1, |total, cur| lcm(total, cur as u64)) as u64
}

test_solution!("232792560", solution);