mod ecpoint;
mod signer;

use ecpoint::ECPoint;
use num::BigInt;
use signer::Signer;
use std::str::FromStr;

fn g_keys() {
    let big_int = |s| BigInt::from_str(s).unwrap();
    let p =
        big_int("57896044618658097711785492504343953926634992332820282019728792003956564821041");
    let a = BigInt::from(7);
    let b =
        big_int("43308876546767276905765904595650931995942111794451039583252968842033849580414");
    let x = BigInt::from(2);
    let y = big_int("4018974056539037503335449422937059775635739389905545080690979365213431566280");
    let q =
        big_int("57896044618658097711785492504343953927082934583725450622380973592137631069619");
    let gost = Signer::new(p, a, b, q, x, y);

    let (private_key, public_key) = gost.gen_keys();

    let message =
        big_int("20798893674476452017134061561508270130637142515379653289952617252661468872421");

    let k =
        big_int("53854137677348463731403841147996619241504003434302020712960838528893196233395");

    let sign = gost.sign(message.clone(), private_key, k);

    let nice = gost.verify(message, sign, public_key);

    assert!(nice);
}

fn main() {
    g_keys();
}
