use md5;
use std::fs::read_to_string;

fn get_binary_hash(data: String) -> Vec<u8> {
    // OsRng OS's random number generator
    let digest = md5::compute(data);

    return digest.to_vec();
}

fn count_trailing_zeros(bytes: &[u8]) -> u32 {
    if bytes.is_empty() {
        return 0;
    }

    // Start with the least significant byte (last one)
    for (i, &byte) in bytes.iter().rev().enumerate() {
        if byte != 0 {
            // Found a non-zero byte
            // Count trailing zeros in this byte + complete bytes already processed
            return byte.trailing_zeros() + (i as u32) * 8;
        }
    }

    // All bytes are zero
    return (bytes.len() as u32) * 8;
}

fn read_input_file(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }
    return lines;
}

fn main() {
    let mut registries: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let inputs = read_input_file("inputs.txt");
    for input in &inputs {
        for index in 0..registries.len() {
            let reg_prefix: &'static str = "registry-";
            let reg_id = index.to_string();
            let val = reg_prefix.to_string() + &reg_id + &input;
            let hashed_val = get_binary_hash(val);
            let num_zeros = count_trailing_zeros(&hashed_val);
            let reg_val = registries[index];
            if num_zeros > reg_val {
                registries[index] = num_zeros;
            }
        }
    }

    let mut all_vals = 0;
    for val in registries {
        all_vals += val;
    }
    println!("registries: {:?}", registries);
    let mean_val: f32 = all_vals as f32 / registries.len() as f32;
    println!("mean_val: {:?}", mean_val);
    let approx = 1f32 / 0.5f32.powf(mean_val);
    println!("approx: {:?}", approx);
    println!("actual: {:?}", inputs.len());
}
