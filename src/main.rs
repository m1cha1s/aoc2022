use runner::Runner;

mod runner;

mod day1;

fn main() {
    let mut r = Runner::new();

    r.add_day(
        day1::day1,
        "data/day1.txt".into(),
        "data/day1Example.txt".into(),
    );

    #[cfg(not(debug_assertions))]
    r.run();

    #[cfg(debug_assertions)]
    r.run_example();
}
