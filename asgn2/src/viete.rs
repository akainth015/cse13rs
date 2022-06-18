pub fn pi() -> (f64, u32) {
    let mut product = 1f64;
    let mut k = 1;
    let mut numerator = 2f64.sqrt();

    while (2f64 + numerator).sqrt() - numerator > 1e-14 {
        product *= numerator / 2f64;
        numerator = (2f64 + numerator).sqrt();
        k += 1;
    }

    (product / 2f64, k)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI};
    use crate::madhava::pi;

    #[test]
    fn correct_value() {
        let (pi_actual, _) = pi();
        let difference = (PI - pi_actual).abs();
        assert!(difference < 1e-7, "actual: {}, expected: {}", pi_actual, PI);
    }
}
