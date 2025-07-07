use rand::Rng;

pub const DATA_SIZE: usize = 100_000;

pub fn generate_test_data() -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    for _ in 0..DATA_SIZE {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }
    data
}
