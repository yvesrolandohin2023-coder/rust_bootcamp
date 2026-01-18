use clap::{Parser, Subcommand};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Paramètres DH imposés par le sujet (Screenshot 15)
const P: u64 = 0xD87FA3E291B4C7F3;
const G: u64 = 2;

#[derive(Parser)]
#[command(name = "streamchat")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server { port: u16 },
    Client { addr: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Server { port } => start_server(port),
        Commands::Client { addr } => start_client(addr),
    }
}

// Implémentation de l'exponentiation modulaire (Square-and-multiply)
fn mod_exp(mut base: u128, mut exp: u64, modulus: u128) -> u64 {
    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    res as u64
}

fn handle_connection(mut stream: TcpStream) {
    // 1. Génération clé privée aléatoire (64-bit)
    let private_key: u64 = rand::random::<u64>() % (P - 2) + 1;
    let public_key = mod_exp(G as u128, private_key, P as u128);

    // 2. Échange de clés publiques
    let mut buffer = [0; 8];
    stream.write_all(&public_key.to_be_bytes()).unwrap();
    stream.read_exact(&mut buffer).unwrap();
    let other_public = u64::from_be_bytes(buffer);

    // 3. Calcul du secret partagé
    let shared_secret = mod_exp(other_public as u128, private_key, P as u128);
    println!("Secure channel established! Secret: {:X}", shared_secret);

    // 4. Logique de chat simple (XOR avec le secret)
    // Ici vous ajouteriez une boucle de lecture/écriture avec chiffrement XOR
}

fn start_server(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("[SERVER] Listening on 0.0.0.0:{}", port);
    if let Ok((stream, _)) = listener.accept() {
        handle_connection(stream);
    }
}

fn start_client(addr: String) {
    let stream = TcpStream::connect(addr).expect("Connection failed");
    println!("[CLIENT] Connected!");
    handle_connection(stream);
}
