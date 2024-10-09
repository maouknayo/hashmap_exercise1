use std::collections::HashMap;

fn main() {
    let number_list = vec![1, 2, 2, 3, 4, 4];

    let median = get_median_of_vec(&number_list);

    let mode = get_mode_of_num_vec(&number_list);

    println!("median: {}\n mode: {}", median, mode);
}

fn get_median_of_vec<T>(vec: &Vec<T>) -> &T {
    &vec[vec.len() / 2]
}

fn get_mode_of_num_vec(vec: &Vec<i32>) -> i32 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for number in vec {
        let entry = hash_map.entry(*number).or_insert(0);

        *entry += 1
    }

    let mut max_key: i32 = 0;
    let mut max_value: i32 = 0;

    for (k, v) in hash_map.iter() {
        if *v > max_value {
            max_key = *k;
            max_value = *v
        }
    }

    return max_key;
}
