use super::ecpoint::ECPoint;
use num::bigint::RandBigInt;
use num::traits::Euclid;
use num::{BigInt, One, Zero};
use rand::thread_rng;

pub struct Signer {
    point: ECPoint,

    q: BigInt,
}

impl Signer {
    pub fn new(p: BigInt, a: BigInt, b: BigInt, q: BigInt, p_x: BigInt, p_y: BigInt) -> Self {
        let point = ECPoint {
            x: p_x,
            y: p_y,
            a,
            b,
            p,
        };

        Self { point, q }
    }

    pub fn gen_keys(&self) -> (BigInt, ECPoint) {
        let d = thread_rng().gen_bigint_range(&BigInt::from(1), &(self.q.clone() - 1));
        let q_point = d.clone() * self.point.clone();
        (d, q_point)
    }

    pub fn sign(&self, message: BigInt, private_key: BigInt, mut k: BigInt) -> (BigInt, BigInt) {
        let mut e = message % &self.q;
        if e.is_zero() {
            e = BigInt::one();
        }
        let e = e;

        if k.is_zero() {
            k = thread_rng().gen_bigint_range(&BigInt::from(1), &(self.q.clone() - 1));
        }

        let mut r = BigInt::zero();
        let mut s = BigInt::zero();

        while r.is_zero() || s.is_zero() {
            let c_point = k.clone() * self.point.clone();
            r = c_point.x % self.q.clone();
            s = (&r * &private_key + &k * &e) % &self.q;
        }
        (r, s)
    }

    pub fn verify(&self, message: BigInt, sign: (BigInt, BigInt), public_key: ECPoint) -> bool {
        let mut e = &message % &self.q;
        if e.is_zero() {
            e = BigInt::one();
        }
        let nu = ECPoint::mod_inverse(e, self.q.clone());
        let z1 = (&sign.1 * &nu).rem_euclid(&self.q);
        let z2 = (-&sign.0 * &nu).rem_euclid(&self.q);
        let c_point = z1 * self.point.clone() + z2 * public_key;

        c_point.x % &self.q == sign.0
    }
}

impl std::str::FromStr for Signer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let nums: Vec<BigInt> = s
            .split_ascii_whitespace()
            .map(BigInt::from_str)
            .flatten()
            .collect();

        let [p, a, b, q, p_x, p_y] = nums.try_into().map_err(|_| ())?;

        Ok(Self::new(p, a, b, q, p_x, p_y))
    }
}

impl std::fmt::Display for Signer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            point:
                ECPoint {
                    x: p_x,
                    y: p_y,
                    a,
                    b,
                    p,
                },
            q,
        } = self;

        write!(f, "{p} {a} {b} {q} {p_x} {p_y}")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_sign() {
        let big_int = |s| BigInt::from_str(s).unwrap();
        let p = big_int(
            "57896044618658097711785492504343953926634992332820282019728792003956564821041",
        );
        let a = BigInt::from(7);
        let b = big_int(
            "43308876546767276905765904595650931995942111794451039583252968842033849580414",
        );
        let x = BigInt::from(2);
        let y =
            big_int("4018974056539037503335449422937059775635739389905545080690979365213431566280");
        let q = big_int(
            "57896044618658097711785492504343953927082934583725450622380973592137631069619",
        );
        let gost = Signer::new(p, a, b, q, x, y);
        let key = big_int(
            "55441196065363246126355624130324183196576709222340016572108097750006097525544",
        );
        let message = big_int(
            "20798893674476452017134061561508270130637142515379653289952617252661468872421",
        );
        let k = big_int(
            "53854137677348463731403841147996619241504003434302020712960838528893196233395",
        );
        let sign = gost.sign(message, key, k);

        let expected = (
            big_int(
                "29700980915817952874371204983938256990422752107994319651632687982059210933395",
            ),
            big_int("574973400270084654178925310019147038455227042649098563933718999175515839552"),
        );

        assert_eq!(sign, expected);
    }

    #[test]
    fn test_verify() {
        let big_int = |s| BigInt::from_str(s).unwrap();
        let p = big_int(
            "57896044618658097711785492504343953926634992332820282019728792003956564821041",
        );
        let a = BigInt::from(7);
        let b = big_int(
            "43308876546767276905765904595650931995942111794451039583252968842033849580414",
        );
        let x = BigInt::from(2);
        let y =
            big_int("4018974056539037503335449422937059775635739389905545080690979365213431566280");
        let q = big_int(
            "57896044618658097711785492504343953927082934583725450622380973592137631069619",
        );
        let gost = Signer::new(p.clone(), a.clone(), b.clone(), q.clone(), x, y);

        let message = big_int(
            "20798893674476452017134061561508270130637142515379653289952617252661468872421",
        );

        let sign = (
            big_int(
                "29700980915817952874371204983938256990422752107994319651632687982059210933395",
            ),
            big_int("574973400270084654178925310019147038455227042649098563933718999175515839552"),
        );

        let q_x = big_int(
            "57520216126176808443631405023338071176630104906313632182896741342206604859403",
        );
        let q_y = big_int(
            "17614944419213781543809391949654080031942662045363639260709847859438286763994",
        );

        let public_key = ECPoint {
            x: q_x,
            y: q_y,
            a,
            b,
            p,
        };

        // let public_key

        assert!(gost.verify(message, sign, public_key));
    }

    // key = 55441196065363246126355624130324183196576709222340016572108097750006097525544
    // message = 20798893674476452017134061561508270130637142515379653289952617252661468872421
    // k = 53854137677348463731403841147996619241504003434302020712960838528893196233395
    // sign = gost.sign(message, key, k)
    // expected = (29700980915817952874371204983938256990422752107994319651632687982059210933395,
    //             574973400270084654178925310019147038455227042649098563933718999175515839552)
    // assert sign == expected

    // message = 20798893674476452017134061561508270130637142515379653289952617252661468872421
    // sign = (29700980915817952874371204983938256990422752107994319651632687982059210933395,
    //         574973400270084654178925310019147038455227042649098563933718999175515839552)
    // q_x = 57520216126176808443631405023338071176630104906313632182896741342206604859403
    // q_y = 17614944419213781543809391949654080031942662045363639260709847859438286763994
    // public_key = ECPoint(q_x, q_y, a, b, p)
    // assert gost.verify(message, sign, public_key) == True
}

// class DSGOST:
//     # p - int, EC module
//     # a, b - int, EC coefficients
//     # q - int, order of point P
//     # p_x, p_y - int, point P coordinates
//     def __init__(self, p, a, b, q, p_x, p_y):
//         self.p_point = ECPoint(p_x, p_y, a, b, p)
//         self.q = q
//         self.a = a
//         self.b = b
//         self.p = p

//     # generate key pair
//     def gen_keys(self):
//         d = random.randint(1, self.q - 1)
//         q_point = d * self.p_point
//         return d, q_point

//     # sign message
//     # message - int
//     # private_key - int
//     def sign(self, message, private_key, k=0):
//         e = message % self.q
//         if e == 0:
//             e = 1
//         if k == 0:
//             k = random.randint(1, self.q - 1)
//         r, s = 0, 0
//         while r == 0 or s == 0:
//             c_point = k * self.p_point
//             r = c_point.x % self.q
//             s = (r * private_key + k * e) % self.q
//         return r, s
