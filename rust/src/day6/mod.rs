enum LexerState {
    InNum,
    InSpace,
}

struct Task {
    ops: char,
    values: Vec<u64>,
}

impl Task {
    fn new(ops: &[String], numbers: &[Vec<String>], task_idx: usize) -> Self {
        let ops = ops[task_idx].chars().next().unwrap();
        let values = numbers
            .iter()
            .map(|row| row[task_idx].parse::<u64>().unwrap())
            .collect();
        Self { ops, values }
    }
}

pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let lines: Vec<&str> = src.lines().collect();

    let mut processed_lines = Vec::new();
    let mut state = LexerState::InSpace;
    for line in lines {
        let line = line.trim();

        let mut out = Vec::new();
        let mut staging = String::new();
        for ch in line.chars() {
            match state {
                LexerState::InNum => {
                    if ch.is_whitespace() {
                        state = LexerState::InSpace;
                        out.push(std::mem::take(&mut staging));
                    } else {
                        staging.push(ch);
                    }
                }
                LexerState::InSpace => {
                    if !ch.is_whitespace() {
                        state = LexerState::InNum;
                        staging.push(ch);
                    }
                }
            }
        }

        out.push(std::mem::take(&mut staging));

        processed_lines.push(out);
    }

    let tasks_count = processed_lines[0].len();
    let tasks_height = processed_lines.len() - 1;
    let ops_idx = tasks_height;

    let numbers = &processed_lines[0..tasks_height];
    let ops = &processed_lines[ops_idx];

    let mut out = 0;
    for task_idx in 0..tasks_count {
        let task = Task::new(ops, numbers, task_idx);

        let res = match task.ops {
            '*' => task.values.iter().product::<u64>(),
            '+' => task.values.iter().sum(),
            _ => unimplemented!(),
        };

        out += res;
    }

    dbg!(out);
}
