pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut factors_vector: Vec<u32> = Vec::new();
    let mut counter = 1;
    for item in factors {
        if *item < 1 {
            break;
        }
        while counter * item < limit {
            if !factors_vector.contains(&(item * counter)) {
                factors_vector.push(counter * item);
            }
            counter += 1;
        }
        counter = 1;
    }
    factors_vector.sort();
    factors_vector
        .iter()
        .copied()
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}
