use colored::*;

pub struct Result<Output: ToString> {
    problem: String,
    pass: bool,
    duration: u64,
    expect: String,
    actual: Output,
}

impl<Output: ToString> Result<Output> {
    pub fn print(&self) {
        println!(
            "{} [{}] | {} ns{}",
            self.problem.to_string().bold(),
            if self.pass {
                "PASS".green()
            } else {
                "FAIL".red()
            },
            self.duration,
            if self.pass {
                String::new()
            } else {
                format!(" | Expect: {}, Actual: {}", self.expect, self.actual.to_string())
            },
        );
    }
}

pub fn test<Output: ToString>(expect: String, solution: fn() -> Output) -> Result<Output> {
    let args: Vec<String> = std::env::args().collect();

    let start = time::precise_time_ns();
    let actual = (solution)();
    let end = time::precise_time_ns();

    Result {
        problem: (&args[0])
            .replace("target/release/s", "")
            .replace("target/debug/s", ""),
        pass: actual.to_string() == expect,
        duration: end - start,
        expect: expect,
        actual: actual,
    }
}

#[macro_export]
macro_rules! test_solution {
    ($expect:expr, $solution:expr) => {
        fn main() {
            let result = $crate::test(String::from($expect), $solution);
            result.print();
        }
    };
}