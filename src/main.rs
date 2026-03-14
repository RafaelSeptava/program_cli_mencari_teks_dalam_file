use std::{env, process};
use program_cli_mencari_teks_dalam_file::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Masalah dalam konfigurasi: {}", err);
        process::exit(1);
    });

    if let Err(e) = program_cli_mencari_teks_dalam_file::run(config) {
        println!("Program error: {}", e);
        process::exit(1);
    }
}