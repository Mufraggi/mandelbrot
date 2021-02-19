use std::str::FromStr;
use num::Complex;

/// Check the input for s
/// You need to respect this format <left><sep><right> exemple:
///"400*600" '*'
///
fn check_pair<T:FromStr>(s: &str, seperator: char) -> Option<(T,T)> {
    match s.find(seperator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

/// Analyse 1 paire off to float séparate by a ,
fn check_complex(s: &str) -> Option<Complex<f64>> {
    match check_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_check_pair() {
    assert_eq!(check_pair::<i32>("", ','), None);
    assert_eq!(check_pair::<i32>("10,", ','), None);
    assert_eq!(check_pair::<i32>(",10", ','), None);
    assert_eq!(check_pair::<i32>("10,20", ','), Some((10,20)));
    assert_eq!(check_pair::<i32>("10,20xy", ','), None);
    assert_eq!(check_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(check_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_check_complex() {
    assert_eq!(check_complex("1.25,-0.0625"), Some(Complex{re:1.25, im:-0.0625}));
    assert_eq!(check_complex(",-0.0625"), None)
}