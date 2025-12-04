struct Grid {
    data: Vec<Vec<bool>>,
}

impl Grid {
    fn new(data: Vec<Vec<bool>>) -> Self {
        Self { data }
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let Some(row) = self.data.get(y) else {
            return false;
        };

        row.get(x).copied().unwrap_or(false)
    }

    fn surrounding_count(&self, x: usize, y: usize) -> usize {
        // xxx
        // x x
        // xxx

        let mut count = 0;

        let mut check = |x: usize, y: usize| {
            if self.get(x, y) {
                count += 1;
            }
        };

        // Top row
        if let Some(y) = y.checked_sub(1) {
            // Left col
            if let Some(x) = x.checked_sub(1) {
                check(x, y);
            }

            // Middle col
            check(x, y);

            // Right col
            check(x + 1, y);
        }

        // Middle row
        {
            // Left col
            if let Some(x) = x.checked_sub(1) {
                check(x, y);
            }

            // Right col
            check(x + 1, y);
        }

        // Bottom row
        if let Some(y) = y.checked_add(1) {
            // Left col
            if let Some(x) = x.checked_sub(1) {
                check(x, y);
            }

            // Middle col
            check(x, y);

            // Right col
            check(x + 1, y);
        }

        count
    }
}

pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let grid: Vec<Vec<bool>> = src
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let grid = Grid::new(grid);

    let mut out = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) {
                let count = grid.surrounding_count(x, y);
                if count < 4 {
                    out += 1;
                }
            }
        }
    }

    dbg!(out);
}
