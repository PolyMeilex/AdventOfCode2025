use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Vec3 {
    x: u64,
    y: u64,
    z: u64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Vec3 {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }
}

fn dist(a: Vec3, b: Vec3) -> f64 {
    let x = (b.x as f64 - a.x as f64).powi(2);
    let y = (b.y as f64 - a.y as f64).powi(2);
    let z = (b.z as f64 - a.z as f64).powi(2);
    (x + y + z).sqrt()
}

pub fn run(_part2: bool) {
    let src = include_str!("./input2.txt").trim();

    let junctions: Vec<_> = src
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            Vec3::new(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    #[derive(Debug, Clone, Copy)]
    struct Res {
        a: Vec3,
        b: Vec3,
        dist: f64,
    }

    let mut distance_list = Vec::new();

    for a in junctions.iter() {
        for b in junctions.iter() {
            if a == b {
                continue;
            }

            let d = dist(*a, *b);
            distance_list.push(Res {
                a: *a,
                b: *b,
                dist: d,
            });
        }
    }

    distance_list.sort_by(|a, b| a.dist.total_cmp(&b.dist));
    distance_list.dedup_by(|a, b| a.a == b.b && a.b == b.a);

    let mut world = World::new();
    for res in distance_list.iter().take(1000) {
        world.connect(res.a, res.b);
    }

    let mut circuits: Vec<_> = world
        .circuits
        .iter()
        .filter(|c| !c.is_empty())
        .map(|c| c.len())
        .collect();
    circuits.sort();
    circuits.reverse();

    let out: usize = circuits.iter().take(3).product();

    dbg!(out);
}

type CircuitId = usize;

#[derive(Debug)]
struct World {
    junction_to_circuit: HashMap<Vec3, CircuitId>,
    circuits: Vec<Vec<Vec3>>,
}

impl World {
    fn new() -> Self {
        Self {
            junction_to_circuit: HashMap::new(),
            circuits: (0..1000).map(|_| Vec::new()).collect(),
        }
    }

    fn circuit_id_for_junction(&self, a: &Vec3) -> Option<CircuitId> {
        self.junction_to_circuit.get(a).copied()
    }

    fn first_empty_circuit_id(&self) -> CircuitId {
        self.circuits
            .iter()
            .enumerate()
            .find(|(_, c)| c.is_empty())
            .map(|(id, _)| id)
            .unwrap()
    }

    fn connect(&mut self, a: Vec3, b: Vec3) {
        let a_circuit_id = self.circuit_id_for_junction(&a);
        let b_circuit_id = self.circuit_id_for_junction(&b);

        match (a_circuit_id, b_circuit_id) {
            (None, None) => {
                let id = self.first_empty_circuit_id();
                self.circuits[id].push(a);
                self.circuits[id].push(b);
                self.junction_to_circuit.insert(a, id);
                self.junction_to_circuit.insert(b, id);
            }
            (None, Some(id)) => {
                self.circuits[id].push(a);
                self.junction_to_circuit.insert(a, id);
            }
            (Some(id), None) => {
                self.circuits[id].push(b);
                self.junction_to_circuit.insert(b, id);
            }
            (Some(a_circuit_id), Some(b_circuit_id)) => {
                if a_circuit_id == b_circuit_id {
                    //
                } else {
                    let [a, b] = self
                        .circuits
                        .get_disjoint_mut([a_circuit_id, b_circuit_id])
                        .unwrap();

                    for v in b.iter() {
                        self.junction_to_circuit.insert(*v, a_circuit_id);
                    }

                    a.append(b);
                }
            }
        }
    }
}
