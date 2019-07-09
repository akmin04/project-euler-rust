use project_euler::test_solution;

static INPUT: u32 = 3;

fn solution() -> u32 {

    let mut max = 0;
    let lower = 10u32.pow(INPUT - 1);
    let upper = 10u32.pow(INPUT) - 1;

    for i in (lower..=upper).rev() {
        for j in (lower..=upper).rev() {
            let num = i * j;
            let string = num.to_string();
            let reversed = string.chars().rev().collect::<String>();
            if string == reversed && i * j > max {
                max = i * j;
            }
        }
    }
    max
}

test_solution!("906609", solution);