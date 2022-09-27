use std::arch::x86_64::{_mm_set_ps, _mm_mul_ps, _mm_add_ps};
use std::time::{Instant};

fn standard_128_f32() {
	let a_values: [f32; 8] = [8.1239412, 931.20100, 5.531, 6.030100,8.1239412, 931.20100, 5.531, 6.030100];
	let b_values: [f32; 8] = [9.0003, 20.202, 81325.20230, 195132.0099,98.1239412, 931.20100, 5.531, 6.030100];
    let start = Instant::now();
    for _i in 1..1000000 {
        let _ = [a_values[0] + b_values[0], a_values[1] + b_values[1],
                    a_values[2] + b_values[2], a_values[3] + b_values[3]];
    }
    println!("std time = {:?}", start.elapsed());
}

fn simd_128_f32() {
	unsafe{
		let a_values = _mm_set_ps(8.1239412, 931.20100, 5.531, 6.030100);
		let b_values = _mm_set_ps(9.0003, 20.202, 81325.20230, 195132.0099);
        let start = Instant::now();
        for _i in 1..1000000 {
            let _ = _mm_add_ps(a_values, b_values);
            let _ = _mm_add_ps(a_values, b_values);
        }
        println!("simd time = {:?}", start.elapsed());
	}
}

fn main() {
    standard_128_f32();
    simd_128_f32();
}
