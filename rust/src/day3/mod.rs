pub fn run(part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let banks: Vec<Vec<u32>> = src
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut out = 0;

    for bank in banks {
        let res = process_bank(&bank, if part2 { 12 } else { 2 });
        out += res;
    }

    dbg!(out);
}

fn pass(bank_section: &[u32], num_place: u32) -> Option<(u64, &[u32])> {
    let mut first_pass_res = None;
    for (id, v) in bank_section
        .iter()
        .enumerate()
        // Reserve sapce at the end for the remaining digits
        .take(bank_section.len() - num_place as usize)
    {
        if *v > first_pass_res.map(|(_, v)| v).unwrap_or(0) {
            first_pass_res = Some((id, *v));
        }
    }

    let (idx, value) = first_pass_res?;
    let bank = &bank_section[idx + 1..];

    let value = 10u64.pow(num_place) * value as u64;

    Some((value, bank))
}

fn process_bank(mut bank: &[u32], len: u32) -> u64 {
    let mut out = 0;
    for n in (0..len).rev() {
        let res = pass(bank, n).unwrap();
        bank = res.1;
        out += res.0;
    }
    out
}
