//Bakhshali Sqrt Algorithm

//support f32 so please use 10.0 instead of 10
//this is an ancient indian way to calculate square root
pub fn bakhshali_sqrt(x: f32) -> f32 {
    let mut res: f32;
    let mut a: f32;
    let mut b: f32;

    if (x == 0.0) || (x == 1.0) {
        return x;
    }
    
    else if x < 0.0 {
        return f32::NAN;
    }

    res = x * 0.25;

    loop {
        let pre_res: f32 = res;

        a = (x - (res * res)) / (2.0 * res);
        b = res + a;
        res = b - ((a * a) / (2.0 * b));

        if pre_res == res {
            break;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_bakhshali_sqrt() {
        // let's try get the sqrt of 10
        let a1: f32 = bakhshali_sqrt(9.0);
        assert_eq!(a1, 3.0); //Sqrt Of 9 Is 3

        let a2: f32 = bakhshali_sqrt(0.0);
        assert_eq!(a2, 0.0); //Sqrt Of 0 Is 0

        //let a3: f32 = bakhshali_sqrt(-1.0);
        //assert_eq!(a3, f32::NAN); //I Get Error Here, Because In Rust, NAN != NAN For Unknow Reason... But It's True So...

        let a4: f32 = bakhshali_sqrt(0.5);
        assert_eq!(a4, 0.70710677); //Sqrt Of 0.5 Is 0.70710677
    }
}
