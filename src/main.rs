/*
 * Copyright (C) 2022 Kris Huber
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

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
