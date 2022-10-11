use prost::Message;

use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/datetime.rs"));

#[test]
fn test_datetime_serializes_equivalent_to_string() {
    println!("{}", concat!(env!("OUT_DIR"), "/datetime.rs"));
    let no_opts = RequestNoOpts {
        datetime: String::from("Tue, 1 Jul 2003 10:52:37 +0200"),
    };

    let with_opts = Request {
        datetime: chrono::DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200").unwrap(),
    };

    let ser = serialize_data(&with_opts);
    println!("{:?}", ser);

    check_serialize_equivalent(&no_opts, &with_opts)
}

pub fn serialize_data(hello: &Request) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(hello.encoded_len());

    hello.encode(&mut buf).unwrap();
    buf
}
