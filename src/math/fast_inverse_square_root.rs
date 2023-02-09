/*
A direct Rust translation of the implementation of the Fast Inverse Square Root from Quake 3 Arena found on Wikipedia.
Includes original code comments from Quake 3 Arena by id Software.
Doesn't care about the sign of the input as the Quake 3 Arena implementation doesn't either.
Makes the slight change of defining x2 later when the value is assigned.

Reference implementation: https://en.wikipedia.org/wiki/Fast_inverse_square_root#Overview_of_the_code
More info: https://en.wikipedia.org/wiki/Fast_inverse_square_root
Explanation: https://www.youtube.com/watch?v=p8u_k2LIZyo
*/

#[rustfmt::skip] // Skipping to preserve Quake 3 Arena's style and comments, fmt mangles them
pub fn q_rsqrt(number: f32) -> f32
{
    let mut i: i32;
    let mut y: f32;
    const THREEHALFS: f32 = 1.5;

    let x2: f32 = number * 0.5;
    y  = number;
    i  = y.to_bits() as i32;                    // evil floating point bit level hacking
    i  = 0x5f3759df - ( i >> 1 );               // what the fuck? 
    y  = f32::from_bits(i as u32);
    y  = y * ( THREEHALFS - ( x2 * y * y ) );   // 1st iteration
//  y  = y * ( THREEHALFS - ( x2 * y * y ) );   // 2nd iteration, this can be removed

    y
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_comparison(x: f32) {
        let inv_sqrt = 1f32 / x.sqrt();
        let fast_inv_sqrt = q_rsqrt(x);
        assert!(rougly_equals(inv_sqrt, fast_inv_sqrt));
    }

    fn rougly_equals(x: f32, y: f32) -> bool {
        (x - y).abs() < 0.01
    }

    #[test]
    fn test() {
        const RANDOM_NUMBERS: [f32; 20] = [
            0.8097895651505039,
            0.11734484640406195,
            0.05151651871265883,
            0.9885611825134174,
            0.9823145219313479,
            0.212102674174118,
            0.9687849702708267,
            0.14154802792493193,
            0.06917985357283662,
            0.11756028096053828,
            0.9638334459631667,
            0.713335801152753,
            0.9250035503617997,
            0.16620400431428484,
            0.845606100854796,
            0.7366874081096062,
            0.5187924473801349,
            0.8183728075281342,
            0.06445079031538248,
            0.5533318928422675,
        ];
        for random_number in RANDOM_NUMBERS {
            do_comparison(random_number);
        }
    }
}
