use std::ops::RangeInclusive;

pub fn run(part2: bool) {
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
    if part2 {
        for range in merge_ranges(fresh) {
            out += range.count();
        }
    } else {
        for line in lines {
            let id: u64 = line.parse().unwrap();
            if fresh.iter().any(|r| r.contains(&id)) {
                out += 1;
            }
        }
    }

    dbg!(out);
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut out = Vec::<RangeInclusive<u64>>::new();

    // In:
    // 1, 2, 3, 4
    //    2, 3, 4, 5
    //    2, 3
    // Out:
    // 1, 2, 3, 4, 5

    ranges.sort_by_key(|r| *r.start());

    for range in ranges.iter() {
        let start = *range.start();
        let end = *range.end();

        if let Some(last) = out.last_mut() {
            let last_start = *last.start();
            let last_end = *last.end();

            if last.contains(&start) {
                *last = last_start..=last_end.max(end);
                continue;
            }

            // TODO: We could also merge neighbour non-overlapping but continouous ranges, eg.
            // 1, 2,
            //       3, 4
            // -----------
            // 1, 2, 3, 4,
        }

        out.push(range.clone());
    }

    out
}
