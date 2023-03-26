// https://www.codewars.com/kata/554e4a2f232cdd87d9000038

pub fn dna_strand(dna: &str) -> String {
    let mut result = String::with_capacity(dna.len());

    for char in dna.chars() {
        result.push(match char {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => panic!("genetics 101"),
        });
    }

    result
}
