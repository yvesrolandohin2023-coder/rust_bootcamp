use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Convert to uppercase
    #[arg(long)]
    upper: bool,

    /// Repeat greeting N times 
    #[arg(long, default_value_t = 1)]
    repeat: u32,

    /// Name to greet [default:World] 
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let name = args.name.unwrap_or_else(|| "World".to_string());
    let mut message = format!("Hello, {}!", name);

    if args.upper {
        message = message.to_uppercase();
    }

    for _ in 0..args.repeat {
        println!("{}", message);
    }
}
