use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
struct Plant {
    thickness: isize,
    branches: Vec<(usize, isize)>,
}

const ALL_ACTIVE: [isize; 100] = [1; 100];

impl Plant {
    pub fn outgoing_energy_all_active(&self, id: usize, plants: &HashMap<usize, Plant>) -> isize {
        self.outgoing_energy(id, plants, &ALL_ACTIVE)
    }

    pub fn outgoing_energy(&self, id: usize, plants: &HashMap<usize, Plant>, active: &[isize]) -> isize {
        if self.branches.is_empty() {
            active[id - 1]
        } else {
            let branch_sum = self
                .branches
                .iter()
                .map(|(id, t)| {
                    let plant = plants.get(id).unwrap();
                    t * plant.outgoing_energy(*id, plants, active)
                })
                .sum();
            if branch_sum >= self.thickness { branch_sum } else { 0 }
        }
    }
}

fn main() {
    // p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q18_p1.txt").unwrap();
    let plants = parse_plants(&input);
    let p1 = plants.get(&plants.len()).unwrap().outgoing_energy_all_active(plants.len(), &plants);
    println!("Part 1: {:?}", p1);

    // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q18_p2.txt").unwrap();
    let (plant_input, activation_input) = input.trim().split_once("\n\n\n").unwrap();
    let plants = parse_plants(plant_input);
    let activations = parse_activations(activation_input);
    let p2 = activations.iter().map(|a| plants.get(&plants.len()).unwrap().outgoing_energy(plants.len(), &plants, a)).sum::<isize>();
    println!("Part 2: {p2}");

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q18_p3.txt").unwrap();
    let (plant_input, activation_input) = input.trim().split_once("\n\n\n").unwrap();
    let plants = parse_plants(plant_input);
    let activations = parse_activations(activation_input);

    // looks like all free are used the same, so disable neg used ones and enable the pos used ones
    let num_free = activations[0].len();
    let mut optimal_activation = vec![0isize; num_free];
    for free_id in 1..=num_free {
        for id in num_free..plants.len() {
            if let Some((_, bt)) = plants.get(&id).unwrap().branches.iter().find(|(bid, _)| *bid == free_id)
                && *bt > 0
            {
                optimal_activation[free_id - 1] = 1;
            }
        }
    }
    let last = plants.len();
    let optimal = plants.get(&last).unwrap().outgoing_energy(last, &plants, &optimal_activation);
    let p3 = activations
        .iter()
        .map(|a| {
            let energy = plants.get(&last).unwrap().outgoing_energy(last, &plants, a);
            if energy > 0 { optimal - energy } else { 0 }
        })
        .sum::<isize>();
    println!("Part 3: {p3}");
}

fn parse_activations(activation_input: &str) -> Vec<Vec<isize>> {
    let activations = activation_input.lines().fold(Vec::new(), |mut acc, l| {
        acc.push(l.trim().split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect::<Vec<_>>());
        acc
    });
    activations
}

fn parse_plants(input: &str) -> HashMap<usize, Plant> {
    let id_and_thickness = |s: &str| {
        let (f, b) = s.trim().split_once(" with thickness ").unwrap();
        let f = if let Some((_, id)) = f.split_once("Plant ") { Some(id.parse::<usize>().unwrap()) } else { None };
        let b = if b.ends_with(":") { b[..b.len() - 1].to_string() } else { b.to_string() };
        let b = b.parse::<isize>().unwrap();
        (f, b)
    };
    input.trim().split("\n\n").fold(HashMap::new(), |mut acc, s| {
        let mut lines = s.lines();
        let (id, thickness) = id_and_thickness(lines.next().unwrap());
        let branches = lines.map(|l| id_and_thickness(l)).filter(|(id, _)| id.is_some()).map(|(id, t)| (id.unwrap(), t)).collect();
        acc.insert(id.unwrap(), Plant { thickness, branches });
        acc
    })
}
