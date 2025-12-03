pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let banks: Vec<Vec<u32>> = src
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut out = 0;
    for bank in banks {
        let res = process_bank(&bank);
        out += res;
    }

    dbg!(out);
}

fn process_bank(bank: &[u32]) -> u32 {
    let mut first_pass_res = None;
    for (id, v) in bank
        .iter()
        .enumerate()
        // Skip the last value, because that would prevent us from finding a second number
        .take(bank.len() - 1)
    {
        if *v > first_pass_res.map(|(_, v)| v).unwrap_or(0) {
            first_pass_res = Some((id, *v));
        }
    }

    let (first_pass_idx, first_pas_val) = first_pass_res.unwrap();

    let bank = &bank[first_pass_idx + 1..];

    let second_pas_val = bank.iter().max().unwrap();

    first_pas_val * 10 + second_pas_val
}
