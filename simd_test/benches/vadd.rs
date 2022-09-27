
use criterion::*;
use std::arch::x86_64::{_mm_set_ps, _mm_mul_ps, _mm_add_ps};
use std::time::{Instant};

fn vadd_tests(crit: &mut Criterion) {
    let mut bench_group = crit.benchmark_group("vadd_test");
    let mat_size = 64;
    bench_group.bench_with_input(
        BenchmarkId::new("scalar_add_vec/1", mat_size),
        &mat_size,
        |bench, msize| {
	        let a_values: [f32; 8] = [8.1239412, 931.20100, 5.531, 6.030100,8.1239412, 931.20100, 5.531, 6.030100];
	        let b_values: [f32; 8] = [9.0003, 20.202, 81325.20230, 195132.0099,98.1239412, 931.20100, 5.531, 6.030100];
            bench.iter(|| {
                black_box([a_values[0] + b_values[0], a_values[1] + b_values[1],
                        a_values[2] + b_values[2], a_values[3] + b_values[3]]);
            });
        },
    );
    bench_group.bench_with_input(
        BenchmarkId::new("vec_add/1", mat_size),
        &mat_size,
        |bench, msize| {
            unsafe{
		        let a_values = _mm_set_ps(8.1239412, 931.20100, 5.531, 6.030100);
		        let b_values = _mm_set_ps(9.0003, 20.202, 81325.20230, 195132.0099);
                bench.iter(|| {
                    black_box(_mm_add_ps(a_values, b_values));
                });
            }
        },
    );
}


criterion_group!(vadd_test, vadd_tests);
criterion_main!(vadd_test);

