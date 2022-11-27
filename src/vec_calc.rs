use std::collections::HashMap;

pub fn vec_calc_get_promedio(v: &[i32]) -> f32 {
    let n: i32 = v.iter().sum();
    n as f32 / v.len() as f32
}

pub fn vec_calc_get_mediana(v: &[i32]) -> f32 {
    let mut v = Vec::from(v);
    v.sort_unstable();
    let len = v.len();
    if len % 2 == 0 {
        let a = v[len / 2 - 1];
        let b = v[len / 2];
        (a + b) as f32 / 2.0
    } else {
        v[len / 2] as f32
    }
}

pub fn vec_calc_get_modo(v: &[i32]) -> i32 {
    let mut modos = HashMap::new();
    //v.iter().for_each(|a| *(modos.entry(*a).or_insert(0)) += 1);
    v.iter().for_each(|a| {
        modos.entry(*a).and_modify(|e| *e += 1).or_insert(1);
    });

    let mut max_value = 0;
    let mut modo: Option<(&i32, &i32)> = None;
    modos.iter().for_each(|(key, value)| {
        if *value > max_value {
            max_value = *value;
            modo = Some((key, value))
        }
    });
    *modo.unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_promedio() {
        assert_eq!(vec_calc_get_promedio(&[1, 2, 3, 4]), 2.5)
    }

    #[test]
    fn test_get_mediana() {
        assert_eq!(vec_calc_get_mediana(&[5, 2, 3, 4, 1, 6]), 3.5);
        assert_eq!(vec_calc_get_mediana(&[5, 2, 3, 4, 1]), 3.0);
        assert_eq!(vec_calc_get_mediana(&[25, 12, 13, 54, 101, -5, 0]), 13.0);
    }

    #[test]
    fn test_get_modo() {
        assert_eq!(vec_calc_get_modo(&[1, 2, 3, 3, 4, 4, 4, 5, 5, 6]), 4);
        assert_eq!(vec_calc_get_modo(&[1, 2, 3, 3, 3, 4, 5, 5, 6]), 3);
    }
}
