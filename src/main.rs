fn vec_loop_add_50() -> Vec<i32> {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50
    }
    println!("{:?}", v);
    v
}

fn fibonacci_slow(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
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

fn fibonacci_fast(n: usize) -> u64 {
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
    res[n]
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
        assert_eq!(fibonacci_slow(20), 6765);
    }
    #[test]
    fn test_fibonacci_fast() {
        assert_eq!(fibonacci_fast(93), 12200160415121876738);
    }
}
