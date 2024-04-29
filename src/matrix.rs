#![allow(dead_code)]

use anyhow::Result;
use std::ops::{Add, Mul};
use std::sync::mpsc;
use std::thread;

/// Matrix struct
pub struct Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Send + Sync + 'static,
{
    pub data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Send + Sync + 'static,
{
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        Self { data, rows, cols }
    }

    pub fn get_row(&self, row: usize) -> Vec<T> {
        let start = row * self.cols;
        let end = start + self.cols;
        self.data[start..end].to_vec()
    }

    pub fn get_col(&self, col: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            result.push(self.data[i * self.cols + col]);
        }
        result
    }
}

fn matrix_mul<T>(a: Vec<T>, b: Vec<T>) -> T
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Send + Sync + 'static,
{
    if a.len() != b.len() {
        panic!("Vector size mismatch");
    };
    // simulate a time-consuming CPU intensive operation
    thread::sleep(std::time::Duration::from_secs(1));
    a.iter()
        .zip(b.iter())
        .fold(T::default(), |acc, (a, b)| acc + *a * *b)
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Send + Sync + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.cols || self.cols != rhs.rows {
            panic!("Matrix size mismatch");
        };
        let mut result = Vec::with_capacity(self.rows * rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                result.push(matrix_mul(self.get_row(i), rhs.get_col(j)));
            }
        }
        Self::Output::new(result, self.rows, rhs.cols)
    }
}

struct MsgIn<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Send + 'static + Sync,
{
    row: Vec<T>,
    col: Vec<T>,
    index: usize,
    sender: oneshot::Sender<MsgOut<T>>,
}

struct MsgOut<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Send + 'static + Sync,
{
    value: T,
    index: usize,
}

pub fn matrix_multi_threads_mul<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Send + Sync + 'static,
{
    let senders = (0..4)
        .map(move |_| {
            let (tx, rx) = mpsc::channel::<MsgIn<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = matrix_mul(msg.row, msg.col);
                    if let Err(e) = msg.sender.send(MsgOut {
                        value,
                        index: msg.index,
                    }) {
                        eprintln!("Oneshot Send Error: {}", e);
                    }
                }
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.rows * b.cols;
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let (tx, rx) = oneshot::channel::<MsgOut<T>>();
            if let Err(e) = senders[(i + j) % senders.len()].send(MsgIn {
                row: a.get_row(i),
                col: b.get_col(j),
                index: i * b.cols + j,
                sender: tx,
            }) {
                eprintln!("Send Error: {}", e);
            }
            receivers.push(rx);
        }
    }

    let mut result = vec![T::default(); matrix_len];
    for rx in receivers {
        let msg = rx.recv()?;
        result[msg.index] = msg.value;
    }

    Ok(Matrix::new(result, a.rows, b.cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_mul() {
        // generate 3 * 3 matrix
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let m2 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let m3 = m1 * m2;
        assert_eq!(m3.data, vec![30, 36, 42, 66, 81, 96, 102, 126, 150]);
    }

    #[test]
    fn test_matrix_multi_threads_mul() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let m2 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let m3 = matrix_multi_threads_mul(m1, m2).unwrap();
        assert_eq!(m3.data, vec![30, 36, 42, 66, 81, 96, 102, 126, 150]);
    }
}
