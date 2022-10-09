use std::env;
use std::ops::Index;
use std::ops::IndexMut;
use std::process::ExitCode;
use std::fmt::Debug;

#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    fn new (rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!( rows * cols, data.len());
        Self { rows, cols, data }
    }
    fn print(&self) {
       for m in 0..self.rows {
            for n in 0..self.cols {
                print!("{:5}", self[(m,n)]);
            }
            println!();
        } 
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

fn create_array(m: usize, n: usize) -> Matrix {
    let data: Vec<f64> = vec![0.0; m*n];

    let mut arr: Matrix = Matrix::new(m, n, data);

    for i in 0..m {
        arr[(i,0)] = 100.0;
        arr[(i,n-1)] = 100.0;
    }

    for i in 0..n {
        arr[(0,i)] = 100.0;
        arr[(m-1,i)] = 100.0;
    }
    arr 
} 


fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return ExitCode::FAILURE
    }

    let m = args[1].parse::<usize>().unwrap();
    let n = args[2].parse::<usize>().unwrap();

    if m < 1 || n < 1 {
        println!("Command line arguments must be positive integers.");
        return ExitCode::FAILURE
    }

    let arr = create_array(m, n);
    arr.print();

    ExitCode::SUCCESS
}
