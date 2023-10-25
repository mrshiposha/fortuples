use std::{fs::File, io::Read};

use fortuples::fortuples;

#[allow(unused)]
struct Container<T>(T);

fortuples! {
    #[tuples::tuple_name(Test)]
    #[cfg(feature = "test")]
    #[tuples::member_name(TestMember)]
    #[doc = "test docs"]
    #[tuples::max_size(1)]
    #[tuples::debug_expand(path = "tests/ui/attrs_preserve.rs.output")]
    impl Container<#Test> {
        #[inline]
        #[cfg(feature = "extra-test")]
        #[doc = "test method docs"]
        fn test_method() {}
    }
}

#[test]
fn test_attr_preserve() {
    let mut f_output = File::open("tests/ui/attrs_preserve.rs.output").unwrap();
    let mut f_expected = File::open("tests/ui/attrs_preserve.rs.expected").unwrap();

    let mut output = String::new();
    f_output.read_to_string(&mut output).unwrap();

    let mut expected = String::new();
    f_expected.read_to_string(&mut expected).unwrap();

    assert_eq!(output, expected);
}
