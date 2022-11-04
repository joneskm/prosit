use crate::check_serialize_equivalent;

include!(concat!(env!("OUT_DIR"), "/datetime.rs"));

#[test]
fn test_datetime_serializes_equivalent_to_string() {
    println!("{}", concat!(env!("OUT_DIR"), "/datetime.rs"));

    let no_opts = RequestNoOpts {
        datetime: String::from("Tue, 01 Jul 2003 11:52:37 +0200"),
    };

    let with_opts = Request {
        datetime: chrono::DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 11:52:37 +0200").unwrap(),
    };

    check_serialize_equivalent(&no_opts, &with_opts)
}
