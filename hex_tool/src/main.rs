use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Parser)]
#[command(about = "Read and write binary files in hexadecimal")]
struct Args {
    /// Target file
    #[arg(short, long)]
    file: String,

    /// Read mode (display hex)
    #[arg(short, long)]
    read: bool,

    /// Write mode (hex string to write)
    #[arg(short, long)]
    write: Option<String>,

    /// Offset in bytes (decimal or 0x...)
    #[arg(short, long, default_value = "0")]
    offset: String,

    /// Number of bytes to read
    #[arg(short, long, default_value_t = 16)]
    size: usize,
}

fn parse_offset(s: &str) -> u64 {
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).expect("Invalid hex offset")
    } else {
        s.parse().expect("Invalid decimal offset")
    }
}

fn hex_to_bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("Invalid hex"))
        .collect()
}

fn main() {
    let args = Args::parse();
    let offset = parse_offset(&args.offset);

    // ---- READ MODE ----
    if args.read {
        let mut file = File::open(&args.file).expect("Cannot open file");
        file.seek(SeekFrom::Start(offset)).unwrap();

        let mut buffer = vec![0u8; args.size];
        let read = file.read(&mut buffer).unwrap();
        buffer.truncate(read);

        for (i, chunk) in buffer.chunks(16).enumerate() {
            let addr = offset + (i * 16) as u64;
            print!("{:08x}: ", addr);

            for b in chunk {
                print!("{:02x} ", b);
            }

            for _ in 0..(16 - chunk.len()) {
                print!("   ");
            }

            print!("|");
            for b in chunk {
                let c = if b.is_ascii_graphic() {
                    *b as char
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("|");
        }
    }

    // ---- WRITE MODE ----
    if let Some(hex) = args.write {
        let bytes = hex_to_bytes(&hex);

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&args.file)
            .expect("Cannot open file");

        file.seek(SeekFrom::Start(offset)).unwrap();
        file.write_all(&bytes).unwrap();

        println!("Writing {} bytes at offset 0x{:08x}", bytes.len(), offset);

        print!("Hex: ");
        for b in &bytes {
            print!("{:02x} ", b);
        }
        println!();

        print!("ASCII: ");
        for b in &bytes {
            let c = if b.is_ascii_graphic() {
                *b as char
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("\nSuccessfully written");
    }
}
