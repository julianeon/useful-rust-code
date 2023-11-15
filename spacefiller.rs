extern crate itertools;

use itertools::Itertools;

fn main() {
    let alphabet: Vec<char> = ('A'..='Z').collect(); // Uppercase alphabet letters

    let permutations = alphabet.iter().permutations(alphabet.len());

    for perm in permutations {
        println!("{}", perm.into_iter().collect::<String>());
    }
}














