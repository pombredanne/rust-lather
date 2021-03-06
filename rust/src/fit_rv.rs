use rulinalg::matrix::BaseMatrix;
use rulinalg::matrix::Matrix;
use std;

pub fn fit_rv(rv: &[f64], ccf: &[f64]) -> f64 {
    let (min_index, _) = ccf.iter().enumerate().fold(
        (0, std::f64::INFINITY),
        |(min_ind, min_val), (current_ind, current_val)| {
            if *current_val < min_val {
                (current_ind, *current_val)
            } else {
                (min_ind, min_val)
            }
        },
    );

    // TODO: Allocation here is very silly, should be able to avoid it
    let peak_rv = &rv[min_index - 3..min_index + 4];
    let peak_ccf = &ccf[min_index - 3..min_index + 4];
    let rv_matrix_values: Vec<f64> = std::iter::repeat(1.0)
        .take(7)
        .chain(peak_rv.iter().cloned())
        .chain(peak_rv.iter().map(|x| x.powi(2)))
        .collect();

    let rv_matrix_transpose = Matrix::new(3, 7, rv_matrix_values);
    let rv_matrix = rv_matrix_transpose.transpose();
    let ccf_matrix = Matrix::new(7, 1, peak_ccf);
    let coefficients =
        (&rv_matrix_transpose * rv_matrix).inverse().unwrap() * rv_matrix_transpose * ccf_matrix;

    -coefficients[[1, 0]] / (2.0 * coefficients[[2, 0]])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_close(actual: f64, expected: f64) -> bool {
        let precision = 5;
        let pow = 10.0f64.powi(precision + 1);
        let delta = (expected - actual).abs();
        let max_delta = 10.0f64.powi(-precision) / 2.0;
        return (delta * pow).round() / pow <= max_delta;
    }

    #[test]
    fn simple_gaussian() {
        use linspace::linspace;

        let test_len = 101;
        let rv: Vec<f64> = linspace(-1.0, 1.0, test_len).collect();
        let ccf: Vec<f64> = rv.iter().map(|x| -(-(x - 0.5).powi(2)).exp()).collect();

        let poly_rv = fit_rv(&rv, &ccf);
        assert!(is_close(poly_rv, 0.5));
    }
}
