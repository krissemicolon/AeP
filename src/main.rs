use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Sha256, Digest};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// amount of passwords you want to generate
    #[clap(short, long, value_parser, default_value_t = 1)]
    amount: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.amount {
        let mut rng = rand::thread_rng();
        let chain_length: u16 = rng.gen::<u16>();
        let hash_seed_length: u8 = rng.gen::<u8>();

        // initializing password with hash seed
        let mut password: String = rng
            .sample_iter(&Alphanumeric)
            .take(hash_seed_length as usize)
            .map(char::from)
            .collect();

        // hashing process
        for _ in 1..chain_length {
            password = format!("{:X}", Sha256::digest(&password));
        }

        println!("{}", password);
    }
}