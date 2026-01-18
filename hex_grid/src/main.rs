use clap::Parser;
use rand::Rng;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::{File, read_to_string};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(name = "hexpath")]
struct Cli {
    #[arg(long)]
    generate: Option<String>,
    #[arg(long)]
    output: Option<String>,
    #[arg(index = 1)]
    map_file: Option<String>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    pos: (usize, usize),
}

// Configuration pour que le BinaryHeap soit un Min-Heap (chemin minimum)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let args = Cli::parse();

    if let Some(dims) = args.generate {
        let parts: Vec<&str> = dims.split('x').collect();
        if parts.len() == 2 {
            let w = parts[0].parse().unwrap_or(8);
            let h = parts[1].parse().unwrap_or(4);
            let grid = generate_grid(w, h);
            display_grid(&grid);
            
            if let Some(path) = args.output {
                save_to_file(&grid, &path);
            }
        }
    } else if let Some(file_path) = args.map_file {
        // Ajout de l'appel pour l'analyse du fichier sans modifier le reste
        analyze_map(&file_path);
    }
}

fn generate_grid(w: usize, h: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![0u8; w]; h];
    for r in 0..h {
        for c in 0..w {
            grid[r][c] = if r == 0 && c == 0 { 0 } 
                         else if r == h-1 && c == w-1 { 255 } 
                         else { rng.r#gen() };
        }
    }
    grid
}

fn display_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        for val in row {
            print!("{:02X} ", val);
        }
        println!();
    }
}

fn save_to_file(grid: &Vec<Vec<u8>>, path: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    for row in grid {
        let line = row.iter()
            .map(|v| format!("{:02X}", v))
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(file, "{}", line).expect("Unable to write data");
    }
    println!("Map saved to {}", path);
}

// --- AJOUT DES FONCTIONS MANQUANTES (Analyse & Dijkstra) ---

fn analyze_map(path: &str) {
    let content = read_to_string(path).expect("Failed to read file");
    let grid: Vec<Vec<u32>> = content.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace()
            .map(|h| u32::from_str_radix(h, 16).expect("Invalid hex"))
            .collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    println!("Analyzing hexadecimal grid...");
    println!("Grid size: {}x{}", cols, rows);

    // Calcul du chemin Minimum
    if let Some(min_cost) = find_path(&grid, true) {
        println!("\nMINIMUM COST PATH:");
        println!("Total cost: 0x{:X} ({} decimal)", min_cost, min_cost);
    }

    // Calcul du chemin Maximum
    if let Some(max_cost) = find_path(&grid, false) {
        println!("\nMAXIMUM COST PATH:");
        println!("Total cost: 0x{:X} ({} decimal)", max_cost, max_cost);
    }
}

fn get_neighbors(pos: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let (r, c) = (pos.0 as i32, pos.1 as i32);
    // Voisinage hexagonal standard (pointy-topped)
    let directions = if c % 2 == 0 {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1)]
    } else {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1), (1, -1), (1, 1)]
    };

    directions.into_iter()
        .map(|(dr, dc)| (r + dr, c + dc))
        .filter(|&(nr, nc)| nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32)
        .map(|(nr, nc)| (nr as usize, nc as usize))
        .collect()
}

fn find_path(grid: &Vec<Vec<u32>>, find_min: bool) -> Option<u32> {
    let rows = grid.len();
    let cols = grid[0].len();
    let start = (0, 0);
    let end = (rows - 1, cols - 1);

    let mut dists = HashMap::new();
    let mut heap = BinaryHeap::new();

    dists.insert(start, grid[0][0]);

    // Pour le MAX, on utilise un Node avec un coût inversé dans le Min-Heap
    // ou on pourrait implémenter un autre Node struct, mais voici une astuce simple :
    let initial_cost = if find_min { grid[0][0] } else { u32::MAX - grid[0][0] };
    
    heap.push(Node { cost: initial_cost, pos: start });

    while let Some(Node { cost, pos }) = heap.pop() {
        let current_actual_cost = if find_min { cost } else { u32::MAX - cost };

        if pos == end { return Some(current_actual_cost); }
        
        if let Some(&best) = dists.get(&pos) {
            if find_min && current_actual_cost > best { continue; }
            if !find_min && current_actual_cost < best { continue; }
        }

        for next in get_neighbors(pos, rows, cols) {
            let next_actual_cost = current_actual_cost + grid[next.0][next.1];
            let is_better = if find_min {
                next_actual_cost < *dists.get(&next).unwrap_or(&u32::MAX)
            } else {
                next_actual_cost > *dists.get(&next).unwrap_or(&0)
            };

            if is_better {
                dists.insert(next, next_actual_cost);
                let heap_cost = if find_min { next_actual_cost } else { u32::MAX - next_actual_cost };
                heap.push(Node { cost: heap_cost, pos: next });
            }
        }
    }
    None
}
