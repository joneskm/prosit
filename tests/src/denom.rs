use core::convert::TryFrom;

use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/denom.rs"));

#[test]
fn test_denom_serializes_equivalent_to_string() {
    let no_opts = RequestNoOpts {
        denom: String::from("atom2"),
    };

    let with_opts = Request {
        denom: proto_types::Denom::try_from(String::from("atom2")).unwrap(),
    };

    check_serialize_equivalent(&no_opts, &with_opts)
}
