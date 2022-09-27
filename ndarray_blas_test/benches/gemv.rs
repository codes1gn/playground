use ndarray::linalg::general_mat_vec_mul;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::*;

use criterion::*;

fn gemv_64_64c(crit: &mut Criterion) {
    let mut bench_group = crit.benchmark_group("gemm_test");
    let mat_size = 64;
    bench_group.bench_with_input(
        BenchmarkId::new("gemv_64_64c/1", mat_size),
        &mat_size,
        |bench, msize| {
            let a = Array2::<f32>::zeros((*msize, *msize));
            let (m, n) = a.dim();
            let x = Array1::<f32>::zeros(n);
            let mut y = Array1::<f32>::zeros(m);
            bench.iter(|| {
                black_box(general_mat_vec_mul(1.0, &a, &x, 1.0, &mut y));
            });
        },
    );
}

criterion_group!(gemm_test, gemv_64_64c);
criterion_main!(gemm_test);

