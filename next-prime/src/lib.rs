pub fn next_prime(nbr: u64) -> u64 {
    if nbr <= 1 {
        return 2
    } else {
        return prime(nbr)
    }
}

pub fn prime(nbr: u64) -> u64 {
    let mut n = nbr;
    let mut is_prime;

    loop {
        is_prime = true;
        for i in 2..(n/2+1) {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            break;
        }
        n += 1;
    }
    n
}

// pub fn next_prime(nbr: u64) -> u64 {
//     let mut next = nbr + 1;
//     if is_prime(next) {
//         return next;
//     } else {
//         while !is_prime(next) {
//             next += 1;
//         }
//     }

//     next
// }

// pub fn is_prime(nbr: u64) -> bool {
//     if nbr <= 1 {
//         return false;
//     }
//     for i in 2..=(nbr as f64).sqrt() as u64 {
//         if nbr % i == 0 {
//             return false;
//         }
//     }
//     true
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = next_prime(2);
        assert_eq!(result, 2);
    }
}
