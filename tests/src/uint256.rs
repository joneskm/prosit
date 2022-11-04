use core::str::FromStr;

use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/uint256.rs"));

#[test]
fn test_uint256_serializes_equivalent_to_string() {
    let no_opts = RequestNoOpts {
        uint256: String::from("1234335353523398888"),
    };

    let with_opts = Request {
        uint256: cosmwasm_std::Uint256::from_str("1234335353523398888").unwrap(),
    };

    check_serialize_equivalent(&no_opts, &with_opts)
}
