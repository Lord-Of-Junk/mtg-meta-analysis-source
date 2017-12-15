mod player;
mod provider;
mod game;
mod interval;

use std::env;
use provider::LehmerProvider;
use interval::generate_interval;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid number of arguments!");
        eprintln!("Correct arguments: [desired interval half-width] [Seed to be used in random number generator]")
        std::process::exit(1);
    }

    let w:  f64   = args[1].parse().expect("Unable to process desired half-width, w: likely not a valid number: {}", args[1]);
    let seed: i64 = args[2].parse().expect("Unable to process seed: likely not a valid number: {}", args[2]);

    // Simple check to ensure non-negative parameters
    if w < 0.0 {
        eprintln!("Cannot have a negative half-width!");
        std::process::exit(2);
    }
    
    if seed < 0 {
        eprintln!("Cannot have a negative seed!");
        std::process::exit(2);
    }

    // Initialize needed provider and file stream
    let mut p = LehmerProvider::new(seed);

    let mut out = File::create("output.csv").expect("Could not create output file!");

    // Fill out a results table. Is it necessary to have this AND write to file?
    // Not entirely but it is nice infastructure to have should we need to process them further
    // int the future and besides, the table will take up little over three kilobytes in memory.
    let mut results = [ [ 0f64; 20]; 20];
    for p1 in 0..20 {
        for p2 in 0..20 {
            let mean = generate_interval( p1+1, p2+1, w, &mut p );
            results[p1][p2] = mean;
            out.write_all( format!( "{:.4},", mean ).as_bytes() ).expect("Could not write to output file!");
        }
        out.write_all(b"\n").expect("Could not write to output file!");
    }


}
