#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltage: (),
}

fn parse(src: &str) -> Vec<Machine> {
    src.lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();

            let lights = split[0];
            let lights = lights
                .chars()
                .skip(1)
                .take(lights.len() - 2)
                .map(|ch| ch == '#')
                .collect();

            let buttons = split[1..split.len() - 1]
                .iter()
                .copied()
                .map(|btn| {
                    let btn = &btn[1..btn.len() - 1];
                    let btn: Vec<_> = btn.split(",").map(|v| v.parse::<u32>().unwrap()).collect();
                    btn
                })
                .collect();

            Machine {
                lights,
                buttons,
                joltage: (),
            }
        })
        .collect()
}

pub fn run(part2: bool) {
    let src = include_str!("./input.txt").trim();

    let machines = parse(src);

    if part2 {
    } else {
        part1(&machines);
    }
}

fn part1(machines: &[Machine]) {
    for m in machines {
        //
    }
}
