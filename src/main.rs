use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use num_traits::Euclid;
use num_traits::ToPrimitive;

fn main() {
    
    // Finding the generators for subgroups:
    let g = BigInt::parse_bytes(b"7", 10).unwrap();
    let p: BigInt = BigInt::from(2).pow(64) - BigInt::from(2).pow(32) + BigInt::one();
    let l_1: BigInt = BigInt::parse_bytes(b"65537", 10).unwrap();
    let l_2 = BigInt::parse_bytes(b"257", 10).unwrap();

    // Checking that such subgroups exist:
    println!("Checking that such subgroups exist:");
    let div_1 = (p.clone() - BigInt::one()) % (l_1.clone() - BigInt::one());
    let div_2 = (p.clone() - BigInt::one()) % (l_2.clone() - BigInt::one());
    if div_1.is_zero() && div_2.is_zero() {
        println!("Yes, they exist");
    } else {
        println!("No, they don't exist");
    }

    // Factor of scale:
    let fact_1 = (p.clone() - BigInt::one()) / (l_1.clone() - BigInt::one());
    let fact_2 = (p.clone() - BigInt::one()) / (l_2.clone() - BigInt::one());
    println!("\nScale factor for the first case: {}", fact_1);
    println!("Scale factor for the second case: {}", fact_2);

    fn fast_modular_exponentiation(base: BigInt, exponent: BigInt, modulus: BigInt) -> BigInt {
        let mut result = BigInt::one();
        let mut base = base % modulus.clone();

        let zero = BigInt::zero();
        let one = BigInt::one();
        let two = BigInt::from(2);

        let mut exponent = exponent.clone();
        while exponent > zero {
            // If the exponent is odd:
            if exponent.clone() % two.clone() == one {
                result = (result * &base).rem_euclid(&modulus);
            }
            exponent = exponent / two.clone();
            base = (&base * &base).rem_euclid(&modulus);
        }
        result
    }

    let gen_1 = fast_modular_exponentiation(g.clone(), fact_1.clone(), p.clone());
    let gen_2 = fast_modular_exponentiation(g, fact_2, p.clone());
    println!("\nFirst generator: {}", gen_1);
    println!("Second generator: {}", gen_2);

    // Checking:
    println!("\nVerification:\n");
    for i in 1..p.to_usize().unwrap() {
        let result = fast_modular_exponentiation(gen_1.clone(), BigInt::from(i), p.clone());

        if result == BigInt::one() && BigInt::from(i) == l_1.clone() - BigInt::one() {
            println!("First Case: Verified!");
            break;
        }
    }
    for i in 1..p.to_usize().unwrap() {
        let result_2 = fast_modular_exponentiation(gen_2.clone(), BigInt::from(i), p.clone());
        if result_2 == BigInt::one() && BigInt::from(i) == l_2.clone() - BigInt::one() {
            println!("Second Case: Verified!");
            break;
        }
    }
}
