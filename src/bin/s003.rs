use project_euler::test_solution;

static INPUT: u64 = 600851475143;

fn solution() -> u64 {

    let mut prime_factors = 0;
    let mut n = INPUT;
    let mut d = 2;

    while n > 1 {
        while n % d == 0 {
            prime_factors = d;
            n /= d;
        }
        d += 1;
    }
    prime_factors
}

test_solution!("6857", solution);