use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/url.rs"));

#[test]
fn test_url_serializes_equivalent_to_string() {
    let no_opts = RequestNoOpts {
        url: String::from("cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux"),
    };

    let with_opts = Request {
        address: prost_types::AccAddress::from_bech32(
            "cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux",
        )
        .unwrap(),
    };

    check_serialize_equivalent(&no_opts, &with_opts)
}
