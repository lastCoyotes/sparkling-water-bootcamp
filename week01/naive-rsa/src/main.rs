use num_traits::FromPrimitive;
use num_traits::{One, Zero};
use num_integer::Integer;
use num_bigint::BigInt;
use num_bigint::Sign;
use rand::Rng;
use rand::thread_rng;


/*
naive rsa approach:
choose 2 prime numbers, one for Alice (a), one for Bob (b)
    USE MILLER RABIN algo
compute a*b
compute f(a*b) = lowest common multiple (a-1, b-1)
use e (largest fermat prime, 65537) aka 2^2^4 + 1
compute modular multiplicative inverse of e
enter message (m) to encrypt and encrypt it  (em)
enter encrypted message (em) to be decrypted (dm)
assert that the message entered equals the decrypted message (m == dm)

*/

fn main() {

    let mut a = genRand();
    while !millerRabinAlgo(&a, 10) { a = genRand(); }
    let mut b = genRand();
    while !millerRabinAlgo(&b, 10) { b = genRand(); }
    println!("a: {}, b: {}", a, b);
    
    let product: BigInt = a.clone() * b.clone();
    println!("a * b: {}", product);
    
    let productRes = LCM(&(a.clone() - BigInt::one()), &(b.clone() - BigInt::one()));
    println!("λ(a*b): {}", productRes);
    
    let e = BigInt::parse_bytes(b"65537", 10).unwrap();
    println!("e: {}", e);
    
    let modInvE = mod_inv(&e.clone(), &productRes);
    println!("Modular Multiplicative Inverse of e: {}", modInvE);

    let mut m = String::new();
    println!("Enter message to encrypt: ");
    let x = std::io::stdin().read_line(&mut m).unwrap();
    println!("Message: {}", m);
    let strByte = BigInt::from_bytes_be(Sign::Plus, m.clone());
    // encrypt str_byte
    let em = modpow(strByte.clone(), e, product.clone());
    println!("Encrypted Message: {}", em);

    // decrypt em
    let dm = modpow(em, modInvE, product);
    println!("Decrypted Message: {}", dm);

}

fn genRand() -> BigInt {
    let mut num = rand::thread_rng();
    let randNum: u64 = num.gen();
    let bigNum: BigInt = FromPrimitive::from_u64(randNum).unwrap();
    let randBigUInt: BigInt = bigNum << 256;
    return randBigUInt;
}

// true if probable prime, false if composite
// var names from pseudocode algorithm on miller rabin algorithm wikipedia
fn millerRabinAlgo(num: &BigInt, k: usize) -> bool {
    if num <= &BigInt::from(1) {
        return false;
    }
    let mut d = num - BigInt::from(1);
    let mut s = 0;

    while &d.is_even() {
        d /= BigInt::from(2);
        s += 1;
    }

    let mut random = thread_rng();

    for _ in 0..k {
        let a = random.gen_range(BigInt::from(1)..num.clone());
        let mut x = a.modpow(&d, &num);

        if x == BigInt::from(1) || x == num - BigInt::from(1) {
            continue;
        }

        for _ in 0..s-1 {
            x = x.modpow(&BigInt::from(2), &n);

            if x == BigInt::from(1) {
                return false;
            }

            if x == n - BigInt::from(1) {
                break;
            }
        }

        if x != n - BigInt::from(1) {
            return false;
        }
    }    
    return true;
}

fn LCM(a: &BigInt, b: &BigInt) -> BigInt {
    if a.is_zero() || b.is_zero() {
        return BigInt::zero();
    }
    let GCD = euclideanAlgo(a.clone(), b.clone());
    return (a / &GCD) * b;
}

fn euclideanAlgo(a: BigInt, b: BigInt) -> BigInt {
    let (mut x, mut y) = (a.clone(), b.clone());
    
    while !y.is_zero() {
        let r = x % y.clone();
        x = y;
        y = r;
    }
    return x;
}

fn mod_inv(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_one() {
        return BigInt::one();
    }

    let (mut j, mut m, mut x, mut i) = (a.clone(), b.clone(), BigInt::zero(), BigInt::one());

    while j > BigInt::one() {
        let (d, r) = j.div_rem(&m);
        i -= d * &x;
        j = r;
        std::mem::swap(&mut j, &mut m);
        std::mem::swap(&mut x, &mut i);
    }

    if i < BigInt::zero() {
        i += b;
    }

    return i;
}

fn mod_pow(base: BigInt, exp: BigInt, modulus: BigInt) -> BigInt {
    if modulus.is_one() {
        return BigInt::zero();
    }

    let (mut res, mut base, mut exp) = (BigInt::one(), base % &modulus, exp.clone());

    while exp > BigInt::zero() {
        if exp.is_odd() {
            res = (&res * &base) %modulus;
        }

        exp = exp >> 1;
        base = (&base * &base) % &modulus;
    }

    return res;
}
