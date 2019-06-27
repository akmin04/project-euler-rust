use colored::*;
use std::fmt::Display;

#[macro_export]
macro_rules! test_solution {
    ($expect:expr, $solution:expr) => {
        fn main() {
            let solution = $crate::Solution::test(Some(String::from($expect)), $solution);
            println!("{}", solution);
        }
    };
}

#[macro_export]
macro_rules! run_solution {
    ($solution:expr) => {
        fn main() {
            let solution = $crate::Solution::test(None, $solution);
            println!("{}", solution);
        }
    };
}

pub enum Result {
    Pass,
    Fail,
    NoTest,
}

pub struct Solution {
    problem: String,
    result: Result,
    duration: u64,
    expect: String,
    actual: String,
}

impl Solution {
    pub fn test<Output: ToString>(expect: Option<String>, solution: fn() -> Output) -> Solution {
        let args: Vec<String> = std::env::args().collect();

        let start = time::precise_time_ns();
        let actual = (solution)().to_string();
        let end = time::precise_time_ns();

        Solution {
            problem: (&args[0])
                .replace("target/release/s", "")
                .replace("target/debug/s", ""),
            result: match &expect {
                Some(s) => {
                    if *s == actual {
                        Result::Pass
                    } else {
                        Result::Fail
                    }
                }
                None => Result::NoTest,
            },
            duration: end - start,
            expect: expect.unwrap_or(String::new()),
            actual: actual,
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{} | {} ns{}",
            self.problem.to_string().bold(),
            match &self.result {
                Result::Pass => format!(" [{}]", "PASS".green()),
                Result::Fail => format!(" [{}]", "FAIL".red()),
                Result::NoTest => String::new(),
            },
            self.duration,
            match &self.result {
                Result::Pass => String::new(),
                Result::Fail => format!(
                    " | Expect: {}, Actual: {}",
                    self.expect,
                    self.actual.to_string()
                ),
                Result::NoTest => format!(" | Actual: {}", self.actual.to_string()),
            },
        )
    }
}