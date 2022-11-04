use core::str::FromStr;

use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/dec256.rs"));

#[test]
fn test_dec256_serializes_equivalent_to_string() {
    let no_opts = RequestNoOpts {
        dec256: String::from("12343353535233988880000000000000000"),
    };

    let with_opts = Request {
        dec256: math::Decimal256::from_str("12343353535233988.88").unwrap(),
    };

    check_serialize_equivalent(&no_opts, &with_opts)
}
