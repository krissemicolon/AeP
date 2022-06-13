use clap::Parser;
use rayon::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Sha256, Digest};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// amount of passwords you want to generate
    #[clap(short, long, value_parser, default_value_t = 1)]
    amount: u64,
}

fn main() {
    let args = Args::parse();

    (0..args.amount).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let chain_length: u16 = rng.gen::<u16>();
        let hash_seed_length: u8 = rng.gen::<u8>();

        // initializing password with hash seed
        let mut password: String = rng
            .sample_iter(&Alphanumeric)
            .take((hash_seed_length as usize) + 16)
            .map(char::from)
            .collect();

        // hashing process
        // chain length + 10 for 10 min length garanty
        for _ in 1..(chain_length + 10) {
            password = format!("{:X}", Sha256::digest(&password));
        }

        println!("{}", password);
    });
}