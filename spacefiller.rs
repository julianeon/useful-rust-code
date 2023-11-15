extern crate itertools;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    let alphabet: Vec<char> = ('A'..='Z').collect(); // Uppercase alphabet letters

    let permutations = alphabet.iter().permutations(alphabet.len());

    let mut file = File::create("output.txt")?;
    let start_time = Instant::now();
    let mut minutes_passed = 0;

    for perm in permutations {
        let permutation_str: String = perm.into_iter().collect();
        writeln!(file, "{}", permutation_str)?;

        let elapsed_time = start_time.elapsed();
        let seconds_passed = elapsed_time.as_secs();
        
        if seconds_passed >= 60 * (minutes_passed + 1) {
            minutes_passed += 1;
            println!("{} minutes have passed", minutes_passed);
        }
    }

    println!("Text written to output.txt.");
    Ok(())
}
