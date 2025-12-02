use std::ops::RangeInclusive;

pub fn run() {
    let src = include_str!("./input2.txt").trim();
    let ranges: Vec<_> = src
        .split(",")
        .map(|item| {
            let mut iter = item.split("-").map(|num| num.parse::<u64>().unwrap());
            iter.next().unwrap()..=iter.next().unwrap()
        })
        .collect();

    let mut out = 0;
    for range in ranges {
        check_range(range, &mut out);
    }

    dbg!(out);
}

fn check_range(range: RangeInclusive<u64>, out: &mut u64) {
    for i in range {
        let v = i.to_string();

        let (a, b) = v.split_at(v.len() / 2);

        if a == b {
            *out += i;
        }
    }
}
