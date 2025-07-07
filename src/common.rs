use rand::Rng;

pub const INITIAL_DATA_SIZE: usize = 10_000_000;
pub const DATA_SIZE: usize = 100_000;

// Generate a large initial dataset (10x the normal size)
pub fn generate_large_initial_data() -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    for _ in 0..(INITIAL_DATA_SIZE) {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }
    data
}

// Generate mixed update/insert data: mostly updates to existing keys
pub fn generate_mixed_update_data(existing_keys: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    // Mostly updates: select random keys from existing data with new values
    let insert_count = (DATA_SIZE as f64 * 0.05) as usize; // 5% inserts
    let update_count = DATA_SIZE - insert_count;
    for _ in 0..update_count {
        let idx = rng.gen_range(0..existing_keys.len());
        let key = existing_keys[idx].0;
        let new_value = rng.r#gen::<u64>();
        data.push((key, new_value));
    }

    // 5% inserts: generate completely new keys
    for _ in 0..insert_count {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }

    data
}
