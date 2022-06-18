use crate::newton::sqrt;

pub fn pi() -> (f64, u32) {
    let mut sum = 0f64;
    let mut terms = 1;
    let mut term: f64 = 1.0;

    let mut p1 = 1.0;

    while term.abs() > 1e-14 {
        sum += term;

        p1 /= -3 as f64;
        term = p1 / ((2 * terms + 1) as f64);
        terms += 1;
    }

    let (sqrt12, _) = sqrt(12f64);
    (sum * sqrt12, terms)
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
