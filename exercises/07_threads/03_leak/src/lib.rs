// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid_point = v.len() / 2;
    let v: &'static [i32] = v.leak();
    let v1 = &v[..mid_point];
    let v2 = &v[mid_point..];
    let j1 = thread::spawn(|| -> i32 { v1.iter().sum() });
    let j2 = thread::spawn(|| -> i32 { v2.iter().sum() });
    let r1 = j1.join().unwrap();
    let r2 = j2.join().unwrap();
    r1 + r2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
