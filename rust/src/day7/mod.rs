struct Grid {
    rows: Vec<Vec<char>>,
    _height: usize,
    _wigth: usize,
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Self {
        let height = rows.len();
        let wigth = rows[0].len();

        Self {
            rows,
            _height: height,
            _wigth: wigth,
        }
    }

    fn line(&self, y: usize) -> &[char] {
        &self.rows[y]
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.rows.get(y)?.get(x).copied()
    }

    #[allow(unused)]
    fn set(&mut self, x: usize, y: usize, v: char) {
        self.rows[y][x] = v;
    }

    #[allow(unused)]
    fn print(&self) {
        println!();
        for row in self.rows.iter() {
            println!("{row:?}");
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let rows: Vec<Vec<_>> = src.lines().map(|l| l.chars().collect()).collect();

    let grid = Grid::new(rows);
    // grid.print();

    let start = grid
        .line(0)
        .iter()
        .enumerate()
        .find(|(_, ch)| **ch == 'S')
        .map(|(pos, _)| pos)
        .unwrap();

    let mut scheduled = vec![Point { x: start, y: 1 }];

    let mut count = 0;
    loop {
        let tasks = std::mem::take(&mut scheduled);

        for task in tasks.iter() {
            let Some(ch) = grid.get(task.x, task.y) else {
                continue;
            };

            match ch {
                '.' => {
                    let center = Point {
                        x: task.x,
                        y: task.y + 1,
                    };
                    if !scheduled.contains(&center) {
                        scheduled.push(center);
                    }
                }
                '^' => {
                    count += 1;
                    let left = Point {
                        x: task.x - 1,
                        y: task.y + 1,
                    };
                    if !scheduled.contains(&left) {
                        scheduled.push(left);
                    }
                    let right = Point {
                        x: task.x + 1,
                        y: task.y + 1,
                    };
                    if !scheduled.contains(&right) {
                        scheduled.push(right);
                    }
                }
                _ => unreachable!(),
            }
        }

        // Debug trace path
        // for task in tasks.iter() {
        //     grid.set(task.x, task.y, 'X');
        // }
        // grid.print();

        if scheduled.is_empty() {
            break;
        }
    }

    dbg!(count);
}
