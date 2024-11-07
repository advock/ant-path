use rand::seq::IteratorRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Colony {
    name: String,
    connections: HashMap<String, String>, // Direction -> Colony name
}

// Structure to represent an ant with an ID and current position
#[derive(Clone, Debug)]
struct Ant {
    id: usize,
    position: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ant_mania <map_file> <num_ants>");
        return;
    }

    let filename = &args[1];
    let num_ants: usize = args[2].parse().expect("Invalid number of ants");
    match parse_map("./hiveum_map_small.txt") {
        Ok(map) => {
            // Print each colony and its connections for verification
            simulate_ants(map, num_ants, 10_000)
        }
        Err(e) => eprintln!("Error reading map file: {}", e),
    }
}

fn parse_map(filename: &str) -> io::Result<HashMap<String, Colony>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let colony_name = parts[0].to_string();
        let mut connections = HashMap::new();

        for i in 1..parts.len() {
            let pair: Vec<&str> = parts[i].split('=').collect();
            if pair.len() == 2 {
                let direction = pair[0].to_string();
                let connected_colony = pair[1].to_string();
                connections.insert(direction, connected_colony);
            }
        }

        map.insert(
            colony_name.clone(),
            Colony {
                name: colony_name,
                connections,
            },
        );
    }
    Ok(map)
}

// Move the ant randomly to an adjacent colony
fn move_ant(ant: &mut Ant, map: &HashMap<String, Colony>, rng: &mut impl Rng) -> bool {
    if let Some(colony) = map.get(&ant.position) {
        if colony.connections.is_empty() {
            return false; // Ant is trapped
        }

        // Choose a random direction to move to
        if let Some((_, next_colony)) = colony.connections.iter().choose(rng) {
            ant.position = next_colony.clone();
            return true;
        }
    }
    false
}

// Function to simulate the ant invasion
fn simulate_ants(mut map: HashMap<String, Colony>, num_ants: usize, max_moves: usize) {
    let mut rng = rand::thread_rng();
    let mut ants: Vec<Ant> = (0..num_ants)
        .map(|id| {
            let start_colony = map.keys().choose(&mut rng).unwrap().clone();
            Ant {
                id,
                position: start_colony,
            }
        })
        .collect();

    let mut moves = 0;
    let mut destroyed_colonies = HashSet::new();

    while moves < max_moves && ants.len() > 1 {
        let mut positions: HashMap<String, Vec<usize>> = HashMap::new();

        // Move each ant and record their new positions
        for ant in &mut ants {
            if move_ant(ant, &map, &mut rng) {
                positions
                    .entry(ant.position.clone())
                    .or_default()
                    .push(ant.id);
            }
        }

        // Check for fights and destroy colonies
        for (colony, ant_ids) in positions {
            if ant_ids.len() > 1 {
                println!("{} has been destroyed by ants {:?}", colony, ant_ids);
                destroyed_colonies.insert(colony.clone());
                ants.retain(|ant| ant.position != colony);

                // Remove the destroyed colony and its connections
                map.remove(&colony);
                for col in map.values_mut() {
                    col.connections.retain(|_, dest| dest != &colony);
                }
            }
        }

        moves += 1;
    }

    // Print the final state of the map
    println!("\nFinal Map:");
    for (name, colony) in &map {
        if !destroyed_colonies.contains(name) {
            let connections: Vec<String> = colony
                .connections
                .iter()
                .map(|(dir, target)| format!("{}={}", dir, target))
                .collect();
            println!("{} {}", name, connections.join(" "));
        }
    }
}
