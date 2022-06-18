pub fn sqrt(x: f64) -> (f64, u32) {
    let mut z = 0f64;
    let mut y = 1f64;

    let mut iterations = 0;
    while (y - z).abs() > 1e-14 {
        z = y;
        y = 0.5 * (z + x / z);
        iterations += 1;
    }

    (y, iterations)
}

#[cfg(test)]
mod tests {
    use crate::newton::sqrt;

    #[test]
    fn correct_value() {
        let mut i = 0f64;
        while i < 10f64 {
            i += 0.1;
            let expected = i.sqrt();
            let (actual, _) = sqrt(i);
            let difference = (expected - actual).abs();
            assert!(difference < 1e-14, "Actual: {}, Expected: {}", actual, expected);
        }
    }
}
