use std::fmt::Display;
use std::io::{stdout, Write};
use std::time::Instant;

use crate::instant::BasicInstant;

pub fn run<S, R>(name: &str, solver: S)
where
    R: Display,
    S: Fn() -> R,
{
    raw_run(stdout(), Instant::now(), name, solver);
}

fn raw_run<W, N, S, R>(mut writer: W, now: N, name: &str, solver: S)
where
    W: Write,
    N: BasicInstant,
    S: Fn() -> R,
    R: Display,
{
    let solution = solver();

    writeln!(
        &mut writer,
        "{}: {} ({:.2?})",
        name,
        solution,
        now.elapsed()
    )
    .expect("Something went wrong writing the solution!")
}

#[cfg(test)]
mod tests {
    use crate::instant;

    use super::*;
    use std::time::Duration;

    #[test]
    fn test_run() {
        instant::fake::with_fake_elapsed(Duration::from_millis(10));
        let mut output = Vec::new();
        let input = [1721, 979, 366];
        fn solver(input: &[i32; 3]) -> i32 {
            input
                .iter()
                .fold(0, |accumulator, entry| accumulator + entry)
        }

        raw_run(&mut output, instant::fake::Instant::now(), "test", || {
            solver(&input)
        });
        let output = String::from_utf8(output).expect("Not UTF-8");

        assert_eq!(output, "test: 3066 (10.00ms)\n");
    }
}
