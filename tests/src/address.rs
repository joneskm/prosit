use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/address.rs"));

#[test]
fn test_address_serializes_equivalent_to_string() {
    let no_opts = RequestNoOpts {
        address: String::from("cosmos18kkrrrc0j7hkz0fzhaka7p99dz5v3weyx2kxwd"),
    };

    let with_opts = Request {
        address: proto_types::AccAddress::from_bech32(
            "cosmos18kkrrrc0j7hkz0fzhaka7p99dz5v3weyx2kxwd",
        )
        .unwrap(),
    };

    check_serialize_equivalent(&no_opts, &with_opts)
}
