struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new(src: &str) -> Self {
        let rows = src.lines().map(|l| l.chars().collect()).collect();
        Self { rows }
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

    fn down_until_split(&self, x: usize, mut y: usize) -> Option<(Point, Point)> {
        loop {
            let ch = self.get(x, y)?;

            if ch == '^' {
                let left = Point { x: x - 1, y };
                let right = Point { x: x + 1, y };

                return Some((left, right));
            }

            y += 1;
        }
    }

    fn descent(&self, pos: Point, parent: Point, tree: &mut Tree) -> ChoicesCount {
        tree.add_edge(parent, pos);

        if let Some(cached) = tree.add_node(pos) {
            return cached;
        }

        let Some((left, right)) = self.down_until_split(pos.x, pos.y) else {
            return ChoicesCount::Leaf;
        };

        let left = self.descent(left, pos, tree);
        let right = self.descent(right, pos, tree);

        let descendants = ChoicesCount::Node {
            left: left.count(),
            right: right.count(),
        };

        tree.set_choices(pos, descendants);

        descendants
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ChoicesCount {
    Leaf,
    Node { left: usize, right: usize },
}

impl ChoicesCount {
    fn count(&self) -> usize {
        match self {
            ChoicesCount::Leaf => 1,
            ChoicesCount::Node { left, right } => left + right,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TreeNode {
    pos: Point,
    choices: ChoicesCount,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TreeEdge {
    from: Point,
    to: Point,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tree {
    nodes: Vec<TreeNode>,
    edges: Vec<TreeEdge>,
}

impl Tree {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, pos: Point) -> Option<ChoicesCount> {
        if let Some(node) = self.nodes.iter().find(|n| n.pos == pos) {
            Some(node.choices)
        } else {
            self.nodes.push(TreeNode {
                pos,
                choices: ChoicesCount::Leaf,
            });
            None
        }
    }

    fn set_choices(&mut self, pos: Point, choices: ChoicesCount) {
        self.nodes
            .iter_mut()
            .find(|node| node.pos == pos)
            .unwrap()
            .choices = choices;
    }

    fn add_edge(&mut self, from: Point, to: Point) {
        let edge = TreeEdge { from, to };
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

pub fn run(part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let grid = Grid::new(src);

    let start = grid
        .line(0)
        .iter()
        .enumerate()
        .find(|(_, ch)| **ch == 'S')
        .map(|(pos, _)| pos)
        .unwrap();

    if part2 {
        let mut tree = Tree::new();
        let count = grid.descent(Point { x: start, y: 0 }, Point { x: 0, y: 0 }, &mut tree);

        if let ChoicesCount::Node { left, right } = count {
            let count = left + right;
            dbg!(count);
        }

        return;
    }

    let mut scheduled = vec![Point { x: start, y: 0 }];

    let mut count = 0;
    while !scheduled.is_empty() {
        let tasks = std::mem::take(&mut scheduled);

        for task in tasks.iter() {
            let Some(ch) = grid.get(task.x, task.y) else {
                continue;
            };

            match ch {
                '.' | 'S' => {
                    let center = Point {
                        x: task.x,
                        y: task.y + 2,
                    };
                    if !scheduled.contains(&center) {
                        scheduled.push(center);
                    }
                }
                '^' => {
                    count += 1;
                    let left = Point {
                        x: task.x - 1,
                        y: task.y + 2,
                    };
                    if !scheduled.contains(&left) {
                        scheduled.push(left);
                    }
                    let right = Point {
                        x: task.x + 1,
                        y: task.y + 2,
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
    }

    dbg!(count);
}
