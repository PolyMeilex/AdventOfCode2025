use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    _joltage: (),
}

pub fn run(part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let machines = parse(src);

    if part2 {
        //
    } else {
        part1(&machines);
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct Node {
    nest_level: u32,
    state_bits: u32,
}

fn press_all_buttons(
    buttons_bits: &[u32],
    state_bits: u32,
    nest_level: u32,
    out: &mut VecDeque<Node>,
) {
    for btn in buttons_bits.iter() {
        out.push_back(Node {
            nest_level,
            state_bits: state_bits ^ btn,
        });
    }
}

fn solve(machine: &Machine, queue: &mut VecDeque<Node>) -> u32 {
    queue.clear();

    let state = 0;
    let nest_level = 1;
    press_all_buttons(&machine.buttons, state, nest_level, queue);

    while let Some(node) = queue.pop_front() {
        if node.state_bits == machine.lights {
            return node.nest_level;
        }

        press_all_buttons(
            &machine.buttons,
            node.state_bits,
            node.nest_level + 1,
            queue,
        );
    }

    0
}

fn part1(machines: &[Machine]) {
    let mut sum = 0;
    let mut queue = VecDeque::new();

    for m in machines.iter() {
        sum += solve(m, &mut queue);
    }

    dbg!(sum);
}

fn parse(src: &str) -> Vec<Machine> {
    src.lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();

            let lights = split[0];
            let lights_iter = lights.chars().skip(1).take(lights.len() - 2);

            let lights_len = lights_iter.clone().count();
            let lights = lights_iter
                .map(|ch| ch == '#')
                .fold(0u32, |acc, b| (acc << 1) | b as u32);

            let buttons: Vec<_> = split[1..split.len() - 1]
                .iter()
                .copied()
                .map(|btn| {
                    btn[1..btn.len() - 1]
                        .split(",")
                        .map(|v| v.parse::<u32>().unwrap())
                })
                .map(|bits| {
                    let mut out = 0u32;

                    let len = lights_len as u32 - 1;
                    for pos in bits {
                        out |= 0b1 << (len - pos);
                    }

                    out
                })
                .collect();

            Machine {
                lights,
                buttons,
                _joltage: (),
            }
        })
        .collect()
}
