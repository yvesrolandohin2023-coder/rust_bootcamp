use clap::Parser;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Parser)]
#[command(about = "Count word frequency in text")]
struct Args {
    /// Show top N words
    #[arg(long, default_value_t = 10)]
    top: usize,

    /// Ignore words shorter than N
    #[arg(long = "min-length", default_value_t = 1)]
    min_length: usize,

    /// Case insensitive counting
    #[arg(long)]
    ignore_case: bool,

    /// Text to analyze (or use stdin)
    text: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Lire le texte (argument ou stdin)
    let mut input = String::new();
    if let Some(text) = args.text {
        input = text;
    } else {
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read stdin");
    }

    if args.ignore_case {
        input = input.to_lowercase();
    }

    let mut freq: HashMap<String, usize> = HashMap::new();

    for word in input.split_whitespace() {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());

        if clean.len() < args.min_length {
            continue;
        }

        *freq.entry(clean.to_string()).or_insert(0) += 1;
    }

    let mut items: Vec<_> = freq.into_iter().collect();

    // Trier par fréquence décroissante
    items.sort_by(|a, b| b.1.cmp(&a.1));

    if args.top < items.len() {
        items.truncate(args.top);
    }

    println!("Word frequency:");
    for (word, count) in items {
        println!("{}: {}", word, count);
    }
}
