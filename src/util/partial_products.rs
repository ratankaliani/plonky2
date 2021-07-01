use std::iter::Product;
use std::ops::Sub;

use crate::util::ceil_div_usize;

pub fn partial_products<T: Product + Copy>(v: &[T], max_degree: usize) -> Vec<T> {
    let mut res = Vec::new();
    let mut remainder = v.to_vec();
    while remainder.len() >= max_degree {
        let new_partials = remainder
            .chunks(max_degree)
            .filter(|chunk| chunk.len() != 1)
            .map(|chunk| chunk.iter().copied().product())
            .collect::<Vec<_>>();
        res.extend_from_slice(&new_partials);
        let addendum = if remainder.len() % max_degree == 1 {
            vec![*remainder.last().unwrap()]
        } else {
            vec![]
        };
        remainder = new_partials;
        remainder.extend(addendum);
    }

    res
}

pub fn num_partial_products(n: usize, max_degree: usize) -> (usize, usize) {
    let mut res = 0;
    let mut remainder = n;
    while remainder >= max_degree {
        let new_partials_len = ceil_div_usize(remainder, max_degree);
        let addendum = if remainder % max_degree == 1 { 1 } else { 0 };
        res += new_partials_len - addendum;
        remainder = new_partials_len;
    }

    (res, remainder)
}

pub fn check_partial_products<T: Product + Copy + Sub<Output = T>>(
    v: &[T],
    partials: &[T],
    max_degree: usize,
) -> Vec<T> {
    let mut res = Vec::new();
    let mut remainder = v.to_vec();
    let mut partials = partials.to_vec();
    while remainder.len() >= max_degree {
        let products = remainder
            .chunks(max_degree)
            .filter(|chunk| chunk.len() != 1)
            .map(|chunk| chunk.iter().copied().product())
            .collect::<Vec<T>>();
        res.extend(products.iter().zip(&partials).map(|(&a, &b)| a - b));
        let addendum = if remainder.len() % max_degree == 1 {
            vec![*remainder.last().unwrap()]
        } else {
            vec![]
        };
        remainder = partials.drain(..products.len()).collect();
        remainder.extend(addendum)
    }

    res
}

#[cfg(test)]
mod tests {
    use num::Zero;

    use super::*;

    #[test]
    fn test_partial_products() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let p = partial_products(&v, 2);
        assert_eq!(p, vec![2, 12, 30, 24, 720]);
        assert_eq!(p.len(), num_partial_products(v.len(), 2).0);
        assert!(check_partial_products(&v, &p, 2)
            .iter()
            .all(|x| x.is_zero()));

        let v = vec![1, 2, 3, 4, 5, 6];
        let p = partial_products(&v, 3);
        assert_eq!(p, vec![6, 120]);
        assert_eq!(p.len(), num_partial_products(v.len(), 3).0);
        assert!(check_partial_products(&v, &p, 3)
            .iter()
            .all(|x| x.is_zero()));
    }
}
