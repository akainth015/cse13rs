pub fn e() -> (f64, u32) {
    let mut e = 1.0;
    let mut factorial_recip = 1.0;
    let mut terms = 1;

    while factorial_recip >= 1e-14 {
        e += factorial_recip;

        terms += 1;
        factorial_recip /= terms as f64;
    }

    (e, terms)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::E;
    use crate::e::e;

    #[test]
    fn correct_value() {
        let (e_actual, _) = e();
        let difference = (E - e_actual).abs();
        assert!(difference < 1e-14);
    }
}
