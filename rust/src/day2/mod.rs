use std::ops::RangeInclusive;

pub fn run(part2: bool) {
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
        if part2 {
            check_range_part2(range, &mut out);
        } else {
            check_range(range, &mut out);
        }
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

// 12345 12345             : Source
// 12345 12345 12345 12345 : Double Source
//  2345 12345 12345 1234  : Trim start-end to avoid false positives
//       XXXXXXXXXXX       : Matched source
//
// 12 12 12 12             : Source
// 12 12 12 12 12 12 12 12 : Double Source
//  2 12 12 12 12 12 12 1  : Trim start-end to avoid false positives
//    XXXXXXXXXXX          : Matched source
fn check_range_part2(range: RangeInclusive<u64>, out: &mut u64) {
    for i in range {
        let chars = i.to_string();

        let work_area = format!("{chars}{chars}");
        let trimmed = work_area.get(1..work_area.len() - 1).unwrap();

        if trimmed.contains(&chars) {
            *out += i;
        }
    }
}
