pub fn day1(data: String) {
    let mut cals: Vec<i32> = Vec::new();

    for line in data.split("\n") {
        if line == "" {
            cals.push(0);
            continue;
        }

        *cals.last_mut().unwrap() += line.parse::<i32>().unwrap();
    }
    cals.sort();

    println!(
        "Max calories: {:?}",
        cals[(cals.len() - 3)..cals.len()].iter().sum::<i32>()
    );
}
