use std::ops::RangeInclusive;

#[derive(Debug)]
struct Task {
    ops: char,
    values: Vec<u64>,
}

impl Task {
    fn new(ops: &str, numbers: &[&str]) -> Self {
        let ops = ops.chars().next().unwrap();
        let values = numbers
            .iter()
            .map(|row| row.trim().parse::<u64>().unwrap())
            .collect();
        Self { ops, values }
    }

    fn new_part2(ops: &str, numbers: &[&str]) -> Self {
        let ops = ops.chars().next().unwrap();

        let width = numbers.iter().map(|v| v.len()).max().unwrap();

        let mut out = Vec::new();
        for col in 0..width {
            let mut digits = String::new();
            for row in numbers.iter() {
                if let Some(digit) = &row.get(col..=col) {
                    digits += digit;
                }
            }
            out.push(digits.trim().parse::<u64>().unwrap());
        }

        Self { ops, values: out }
    }
}

type ColumnSpan = RangeInclusive<usize>;

pub fn calc_column_spans(ops_row: &str) -> Vec<ColumnSpan> {
    let mut out = Vec::<RangeInclusive<usize>>::new();

    for (idx, ch) in ops_row.char_indices() {
        if ch.is_whitespace() {
            let current = out.last_mut().unwrap();
            *current = *current.start()..=idx;
        } else {
            out.push(idx..=idx);
        }
    }

    // Strip empty columns, last one does not have one
    let count = out.len() - 1;
    for range in out.iter_mut().take(count) {
        let start = *range.start();
        let end = range.end() - 1;
        *range = start..=end;
    }

    out
}

pub fn slice_columns<'a>(column_spans: &[ColumnSpan], rows: &[&'a str]) -> Vec<Vec<&'a str>> {
    column_spans
        .iter()
        .map(|span| rows.iter().map(|row| &row[span.clone()]))
        .map(|iter| iter.collect())
        .collect()
}

pub fn run(part2: bool) {
    let src = include_str!("./input2.txt");

    let lines: Vec<&str> = src.lines().collect();

    let column_spans = calc_column_spans(lines[lines.len() - 1]);

    let ops: Vec<_> = slice_columns(&column_spans, &lines[lines.len() - 1..])
        .into_iter()
        .map(|v| v[0])
        .collect();
    let numbers = slice_columns(&column_spans, &lines[..lines.len() - 1]);

    let tasks_count = column_spans.len();

    let mut out = 0;
    for task_idx in 0..tasks_count {
        let task = if part2 {
            Task::new_part2(ops[task_idx], &numbers[task_idx])
        } else {
            Task::new(ops[task_idx], &numbers[task_idx])
        };

        let res = match task.ops {
            '*' => task.values.iter().product::<u64>(),
            '+' => task.values.iter().sum(),
            _ => unimplemented!(),
        };

        out += res;
    }

    dbg!(out);
}
