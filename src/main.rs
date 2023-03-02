#![deny(overflowing_literals)]
use num::bigint::{BigInt, ToBigInt};

fn vec_loop_add_50() -> Vec<i32> {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50
    }
    println!("{:?}", v);
    v
}

fn fibonacci_slow(n: usize) -> BigInt {
    match n {
        0 => ToBigInt::to_bigint(&0).to_owned().unwrap(),
        1 => ToBigInt::to_bigint(&1).to_owned().unwrap(),
        _ => fibonacci_slow(n - 1).to_owned() + (fibonacci_slow(n - 2)).to_owned(),
    }
}

// def fib(n):
//   res = [0,1]
//   if n < 2:
//     return res[n]
//   else:
//     for i in range(2, n+1):
//       res.append(res[i-1] + res[i-2])
//   return res[-1]

fn fibonacci_fast(n: usize) -> BigInt {
    let mut res = Vec::new();
    res.push(0);
    res.push(1);
    match n {
        0 => res.push(0),
        1 => res.push(1),
        _ => {
            for i in 2..=n {
                res.push(res[i - 1] + res[i - 2])
            }
        }
    }
    return ToBigInt::to_bigint(&res[n]).to_owned().unwrap();
}

fn main() {
    vec_loop_add_50();
    fibonacci_slow(20);
    fibonacci_fast(2300);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vec_loop_add_50() {
        assert_eq!(vec_loop_add_50(), vec![51, 52, 53, 54, 55]);
    }
    #[test]
    fn test_fibonacci_slow() {
        assert_eq!(
            fibonacci_slow(20),
            ToBigInt::to_bigint(&6765).to_owned().unwrap()
        );
    }

    #[test]
    fn test_fibonacci_fast() {
        // 93 == 12200160415121876738
        assert_eq!(fibonacci_fast(46), 1836311903.to_bigint().unwrap());
    }
}
