#[derive(Debug)]
struct Lock {
    value: u32,
}

impl Default for Lock {
    fn default() -> Self {
        Self { value: 50 }
    }
}

// 99, 0, 1
//  ┌─────┐
//  │     │
//  │     │
//  └─────┘
//

impl Lock {
    fn new() -> Self {
        Self::default()
    }

    fn left(&mut self) {
        if self.value == 0 {
            self.value = 99;
        } else {
            self.value -= 1;
        }
    }

    fn right(&mut self) {
        self.value = (self.value + 1) % 100;
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Command {
    value: u32,
    direction: Direction,
}

impl Command {
    fn parse_lines(src: &str) -> Vec<Self> {
        src.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(Self::parse)
            .collect()
    }

    fn parse(line: &str) -> Self {
        let (direction, value) = line.split_at(1);

        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };

        let value: u32 = value.parse().unwrap();

        Self { value, direction }
    }
}

pub fn run(part2: bool) {
    let src = include_str!("./input.txt");

    let commands = Command::parse_lines(src.trim());

    let mut lock = Lock::new();

    let mut count = 0;
    for command in commands {
        if part2 {
            for _ in 0..command.value {
                match command.direction {
                    Direction::Left => lock.left(),
                    Direction::Right => lock.right(),
                }

                if lock.value == 0 {
                    count += 1;
                }
            }
        } else {
            for _ in 0..command.value {
                match command.direction {
                    Direction::Left => lock.left(),
                    Direction::Right => lock.right(),
                }
            }

            if lock.value == 0 {
                count += 1;
            }
        }
    }

    dbg!(count);
}
