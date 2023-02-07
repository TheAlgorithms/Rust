use num::pow;

pub fn n_pow2_plus_n_pow2(num: f32) -> f32 {
    let mut x = 1.0;
    let mut y = 1.0;
    let mut result = 1.0;
    let mut gap = 3.0;
    while y < num {
        x += gap;
        gap += 2.0;
        y += 1.0;
        result += x;
    }
    return result;
}

pub fn sum_n_pow2(num: f32) -> f32 {
    let result = ((num / 2.0) + 0.5) * num;
    return pow(result, 2);
}

pub fn solution(num: f32) -> f32 {
    let r_n_pow2_plus_n_pow2 = n_pow2_plus_n_pow2(num);
    let r_sum_n_pow2 = sum_n_pow2(num);
    return r_sum_n_pow2 - r_n_pow2_plus_n_pow2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ten() {
        let res = solution(10.0);
        assert_eq!(res, 2640.0);
    }
    
    #[test]
    fn test_fifteen() {
        let res = solution(15.0);
        assert_eq!(res, 13160.0);
    }
    
    #[test]
    fn test_twenty() {
        let res = solution(20.0);
        assert_eq!(res, 41230.0);
    }
    
    #[test]
    fn test_fifty() {
        let res = solution(50.0);
        assert_eq!(res, 1582700.0);
    }    
}


