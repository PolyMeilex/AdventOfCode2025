#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

fn area(a: Point, b: Point) -> u64 {
    let w = (a.x as i64 - b.x as i64).unsigned_abs() + 1;
    let h = (a.y as i64 - b.y as i64).unsigned_abs() + 1;
    w * h
}

pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let tiles: Vec<_> = src
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            Point {
                x: split.next().unwrap().parse::<u32>().unwrap(),
                y: split.next().unwrap().parse::<u32>().unwrap(),
            }
        })
        .collect();

    let mut res = (Point::default(), Point::default(), 0);

    for a in tiles.iter() {
        for b in tiles.iter() {
            let area = area(*a, *b);

            if res.2 < area {
                res.0 = *a;
                res.1 = *b;
                res.2 = area;
            }
        }
    }

    dbg!(res);
}
