use template::{matrix_multi_threads_mul, Matrix};

fn main() {
    let now = std::time::Instant::now();
    let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
    let m2 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
    let m3 = matrix_multi_threads_mul(m1, m2).unwrap();
    let end = now.elapsed();
    println!("Time: {:?}", end);
    println!("{:?}", m3.data);
    let now = std::time::Instant::now();
    let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
    let m2 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
    let m3 = m1 * m2;
    let end = now.elapsed();
    println!("Time: {:?}", end);
    println!("{:?}", m3.data);
}
