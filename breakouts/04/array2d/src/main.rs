use std::env;
use std::process::ExitCode;

type Matrix = Box<Vec<Box<Vec<f64>>>>;

fn create_array(m: usize, n: usize) -> Matrix {
    let mut mat: Matrix = Box::new(Vec::with_capacity(m));
    for _ in 0..m {
        mat.push(Box::new(vec![0.0; n]));
    }
    mat
}

fn init(mat: &mut Matrix, m: usize, n: usize) {
    for i in 0..m {
        mat[i][0] = 100.0;
        mat[i][n - 1] = 100.0;
    }

    for i in 0..n {
        mat[0][i] = 100.0;
        mat[m - 1][i] = 100.0;
    }
}

fn print_array(mat: &Matrix, m: usize, n: usize) {
    for i in 0..m {
        for j in 0..n {
            print!("{:8.2}", mat[i][j]);
        }
        println!();
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Error: Must include two arguments for matrix size.");
        return ExitCode::FAILURE;
    }

    let m = args[1].parse::<usize>().unwrap();
    let n = args[2].parse::<usize>().unwrap();

    if m < 1 || n < 1 {
        println!("Command line arguments must be positive integers.");
        return ExitCode::FAILURE;
    }

    let mut mat = create_array(m, n);
    init(&mut mat, m, n);
    print_array(&mat, m, n);

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_values() {
        let m = 7;
        let n = 11;

        let mut mat = create_array(m, n);
        init(&mut mat, m, n);

        for i in 0..m {
            for j in 0..n {
                match i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    true => assert_eq!(mat[i][j], 100.0, "Checking index {i},{j}"),
                    false => assert_eq!(mat[i][j], 0.0, "Checking index {i},{j}"),
                }
            }
        }
    }
}
