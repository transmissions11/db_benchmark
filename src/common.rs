use rand::Rng;

pub const DATA_SIZE: usize = 10_000;

// Generate a large initial dataset (10x the normal size)
pub fn generate_large_initial_data() -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    for _ in 0..(DATA_SIZE * 100) {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }
    data
}

// Generate mixed update/insert data: 50% updates to existing keys, 50% new keys
pub fn generate_mixed_update_data(existing_keys: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    // 50% updates: select random keys from existing data with new values
    let update_count = DATA_SIZE / 2;
    for _ in 0..update_count {
        let idx = rng.gen_range(0..existing_keys.len());
        let key = existing_keys[idx].0;
        let new_value = rng.r#gen::<u64>();
        data.push((key, new_value));
    }

    // 50% inserts: generate completely new keys
    let insert_count = DATA_SIZE - update_count;
    for _ in 0..insert_count {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }

    data
}
