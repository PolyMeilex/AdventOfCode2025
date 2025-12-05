pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let mut lines = src.lines();

    let mut fresh = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut split = line.split("-");
        let a: u64 = split.next().unwrap().parse().unwrap();
        let b: u64 = split.next().unwrap().parse().unwrap();

        let range = a..=b;

        fresh.push(range);
    }

    let mut out = 0;
    for line in lines {
        let id: u64 = line.parse().unwrap();
        if fresh.iter().any(|r| r.contains(&id)) {
            out += 1;
        }
    }

    dbg!(out);
}
