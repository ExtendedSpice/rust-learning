mod kata5526fc09a1bbd946250002dc;
mod kata554e4a2f232cdd87d9000038;
mod kata55f2b110f61eb01779000053;
mod kata583710ccaa6717322c000105;
mod kata586d6cefbcc21eed7a001155;

use crate::kata554e4a2f232cdd87d9000038::dna_strand;
use crate::kata586d6cefbcc21eed7a001155::longest_repetition;

fn main() {
    let _longest_repetition = longest_repetition(&"aaaabbb").unwrap();
    // println!("{} {}", longest_repetition.0, longest_repetition.1);

    let _dna_strand = dna_strand("ATTGC");
    // println!("{}", dna_strand);
}
