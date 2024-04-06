use rand::Rng;

use crate::{L, R};

pub fn get_rand_array() -> [i32; L] {
    let mut arr: [i32; L] = [1; L];
    let mut index = 0;
    let mut rng = rand::thread_rng();

    loop {
        if index > L - 1 {
            break;
        }
        arr[index] = rng.gen_range(0..R as i32 + 1);
        index += 1;
    }
    arr
}
