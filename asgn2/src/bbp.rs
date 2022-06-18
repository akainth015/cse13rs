use crate::newton::sqrt;

pub fn pi() -> (f64, u32) {
    let mut k = 1u32;
    let mut term = 1f64;
    let mut sum = 0f64;

    while term > 1e-14 {
        sum += term;
        term /= 16f64;
        k += 1;
    }

    let numer = (k * (120 * k + 151) + 47) as f64;
    let denom = (k * (k * (k * (512 * k + 1024) + 712) + 194) + 15) as f64;
    let p2 = numer / denom;
    (sum * p2, k)
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
