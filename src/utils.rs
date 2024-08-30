use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref GENERATED_IDS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

pub fn generate_unique_id() -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        let id = rng.gen_range(1..9999999999); // 生成一个 10 位随机数
        let mut ids = GENERATED_IDS.lock().unwrap();
        if !ids.contains(&id) {
            ids.insert(id);
            return id;
        }
    }
}
