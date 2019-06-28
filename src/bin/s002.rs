use project_euler::test_solution;

static INPUT: u32 = 4000000;

fn solution() -> u32 {
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    loop {
        if c % 2 == 0 {
            sum += c;
        }
        if c > INPUT {
            break;
        }
        c = a + b;
        a = b;
        b = c;
    }
    sum
}

test_solution!("4613732", solution);