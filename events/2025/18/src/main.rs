use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone, Hash)]
struct Plant {
    thickness: isize,
    branches: Vec<(usize, isize)>,
}

impl Plant {
    pub fn outgoing_energy(&self, plants: &HashMap<usize, Plant>) -> isize {
        if self.branches.is_empty() {
            1
        } else {
            let branch_sum = self
                .branches
                .iter()
                .map(|(id, t)| {
                    let plant = plants.get(id).unwrap();
                    t * plant.outgoing_energy(plants)
                })
                .sum();
            if branch_sum >= self.thickness { branch_sum } else { 0 }
        }
    }

    pub fn outgoing_energy2(&self, id: usize, plants: &HashMap<usize, Plant>, active: &[isize]) -> isize {
        if self.branches.is_empty() {
            active[id - 1]
        } else {
            let branch_sum = self
                .branches
                .iter()
                .map(|(id, t)| {
                    let plant = plants.get(id).unwrap();
                    t * plant.outgoing_energy2(*id, plants, active)
                })
                .sum();
            if branch_sum >= self.thickness { branch_sum } else { 0 }
        }
    }
}

fn main() {
    // p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q18_p1.txt").unwrap();
    let plant_str = input.trim().split("\n\n").collect::<Vec<&str>>();
    let id_and_thickness = |s: &str| {
        let (f, b) = s.trim().split_once(" with thickness ").unwrap();
        let f = if let Some((_, id)) = f.split_once("Plant ") { Some(id.parse::<usize>().unwrap()) } else { None };
        let b = if b.ends_with(":") { b[..b.len() - 1].to_string() } else { b.to_string() };
        let b = b.parse::<isize>().unwrap();
        (f, b)
    };
    let plants = plant_str.iter().fold(HashMap::new(), |mut acc, s| {
        let mut lines = s.lines();
        let (id, thickness) = id_and_thickness(lines.next().unwrap());
        let branches = lines.map(|l| id_and_thickness(l)).filter(|(id, _)| id.is_some()).map(|(id, t)| (id.unwrap(), t)).collect();
        acc.insert(id.unwrap(), Plant { thickness, branches });
        acc
    });
    println!("Part 1: {:?}", plants.get(&19).unwrap().outgoing_energy(&plants));

    // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q18_p2.txt").unwrap();
    let (plant_input, activation_input) = input.trim().split_once("\n\n\n").unwrap();
    let plant_str = plant_input.trim().split("\n\n").collect::<Vec<&str>>();
    let id_and_thickness = |s: &str| {
        let (f, b) = s.trim().split_once(" with thickness ").unwrap();
        let f = if let Some((_, id)) = f.split_once("Plant ") { Some(id.parse::<usize>().unwrap()) } else { None };
        let b = if b.ends_with(":") { b[..b.len() - 1].to_string() } else { b.to_string() };
        let b = b.parse::<isize>().unwrap();
        (f, b)
    };
    let plants = plant_str.iter().fold(HashMap::new(), |mut acc, s| {
        let mut lines = s.lines();
        let (id, thickness) = id_and_thickness(lines.next().unwrap());
        let branches = lines.map(|l| id_and_thickness(l)).filter(|(id, _)| id.is_some()).map(|(id, t)| (id.unwrap(), t)).collect();
        acc.insert(id.unwrap(), Plant { thickness, branches });
        acc
    });
    let activations = activation_input.lines().fold(Vec::new(), |mut acc, l| {
        acc.push(l.trim().split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect::<Vec<_>>());
        acc
    });
    let p2 = activations.iter().map(|a| plants.get(&36).unwrap().outgoing_energy2(36, &plants, a)).sum::<isize>();
    println!("Part 2: {p2}");

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q18_p3.txt").unwrap();
    let (plant_input, activation_input) = input.trim().split_once("\n\n\n").unwrap();
    let plant_str = plant_input.trim().split("\n\n").collect::<Vec<&str>>();
    let id_and_thickness = |s: &str| {
        let (f, b) = s.trim().split_once(" with thickness ").unwrap();
        let f = if let Some((_, id)) = f.split_once("Plant ") { Some(id.parse::<usize>().unwrap()) } else { None };
        let b = if b.ends_with(":") { b[..b.len() - 1].to_string() } else { b.to_string() };
        let b = b.parse::<isize>().unwrap();
        (f, b)
    };
    let plants = plant_str.iter().fold(HashMap::new(), |mut acc, s| {
        let mut lines = s.lines();
        let (id, thickness) = id_and_thickness(lines.next().unwrap());
        let branches = lines.map(|l| id_and_thickness(l)).filter(|(id, _)| id.is_some()).map(|(id, t)| (id.unwrap(), t)).collect();
        acc.insert(id.unwrap(), Plant { thickness, branches });
        acc
    });
    // println!("plants: {:?}", plants);
    let activations = activation_input.lines().fold(Vec::new(), |mut acc, l| {
        acc.push(l.trim().split_whitespace().map(|e| e.parse::<isize>().unwrap()).collect::<Vec<_>>());
        acc
    });

    // looks like all free are used the same, so disable neg used ones and enable the pos used ones
    let num_free = activations[0].len();
    let mut optimal_activation = vec![0isize; num_free];
    for free_id in 1..=num_free {
        for id in num_free..plants.len() {
            let plant = plants.get(&id).unwrap();
            if let Some((_, bt)) = plant.branches.iter().find(|(bid, _)| *bid == free_id) {
                if *bt > 0 {
                    optimal_activation[free_id - 1] = 1;
                }
                if *bt < 0 && optimal_activation[free_id - 1] == 1 {
                    println!(
                        "should not happen, all are used the same! (free_id: {}, id: {}, bt: {})",
                        free_id, id, bt
                    );
                }
            }
        }
    }
    // println!("optimal activation: {:?}", optimal_activation);
    let last = 109;
    let optimal = plants.get(&last).unwrap().outgoing_energy2(last, &plants, &optimal_activation);
    // println!("optimal: {}", optimal);
    let p3 = activations
        .iter()
        .map(|a| {
            let energy = plants.get(&last).unwrap().outgoing_energy2(last, &plants, a);
            if energy > 0 { optimal - energy } else { 0 }
        })
        // .inspect(|e| println!("energy: {}", e))
        .sum::<isize>();
    println!("Part 3: {p3}");
}
