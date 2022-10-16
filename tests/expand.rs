use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

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
    impl Container<#Test> {}
}

#[test]
fn test_attr_preserve() {
    assert_output("attrs_preserve.rs");
}

#[fortuples::auto_impl]
#[tuples::debug_expand(path = "tests/ui/auto_impl_simple.rs.output")]
trait SimpleTest {}

impl SimpleTest for i32 {}
impl SimpleTest for f32 {}

#[fortuples::auto_impl]
#[tuples::debug_expand(path = "tests/ui/auto_impl_no_args.rs.output")]
#[tuples::max_size(2)]
trait NoArgs {
    fn foo();

    fn bar();
}

impl NoArgs for i32 {
    fn foo() {}

    fn bar() {}
}

impl NoArgs for char {
    fn foo() {}

    fn bar() {}
}

#[fortuples::auto_impl]
#[tuples::debug_expand(path = "tests/ui/auto_impl_only_self.rs.output")]
#[tuples::min_size(1)]
#[tuples::max_size(3)]
trait OnlySelf {
    fn foo(self);

    fn bar(&self);

    fn baz(&mut self);
}

impl OnlySelf for i32 {
    fn foo(self) {}

    fn bar(&self) {}

    fn baz(&mut self) {}
}

impl OnlySelf for &'static str {
    fn foo(self) {}

    fn bar(&self) {}

    fn baz(&mut self) {}
}

#[fortuples::auto_impl]
#[tuples::debug_expand(path = "tests/ui/auto_impl_member_type.rs.output")]
#[tuples::member_type(i32)]
#[tuples::min_size(1)]
#[tuples::max_size(5)]
trait MemberType {
    fn inc(&mut self);
    fn dec(&mut self);
}

impl MemberType for i32 {
    fn inc(&mut self) {
        *self += 1;
    }

    fn dec(&mut self) {
        *self -= 1;
    }
}

#[test]
fn test_auto_impl_simple() {
    assert_output("auto_impl_simple.rs");

    fn simple<T: SimpleTest>() {}

    simple::<()>();
    simple::<((),)>();
    simple::<((), i32)>();
    simple::<((), i32, f32)>();
    simple::<((), i32, f32, i32)>();
    simple::<((), i32, f32, i32, ())>();
    simple::<((), i32, f32, i32, ())>();
    simple::<((), i32, f32, i32, (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), (), (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), (), (), (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), (), (), (), (), (), ())>();
    simple::<((), i32, f32, i32, (), (), (), (), (), (), (), (), (), ())>();
    simple::<(
        (),
        i32,
        f32,
        i32,
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    )>();
    simple::<(
        (),
        i32,
        f32,
        i32,
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    )>();
}

#[test]
fn test_auto_impl_no_args() {
    assert_output("auto_impl_no_args.rs");

    <() as NoArgs>::foo();
    <() as NoArgs>::bar();

    <(i32,) as NoArgs>::foo();
    <(i32,) as NoArgs>::bar();

    <(i32, char) as NoArgs>::foo();
    <(i32, char) as NoArgs>::bar();
}

#[test]
fn test_auto_impl_only_self() {
    assert_output("auto_impl_only_self.rs");

    fn test<T: OnlySelf>(mut t: T) {
        t.bar();
        t.baz();
        t.foo();
    }

    test((1,));
    test((1, "hi"));
    test((1, "hi", 42));
}

#[test]
fn test_auto_impl_member_type() {
    assert_output("auto_impl_member_type.rs");

    {
        let mut _1 = (0,);
        let mut _2 = (0, 10);
        let mut _3 = (0, 10, 20);
        let mut _4 = (0, 10, 20, 30);
        let mut _5 = (0, 10, 20, 30, 40);

        _1.inc();
        _2.inc();
        _3.inc();
        _4.inc();
        _5.inc();

        assert_eq!(_1, (1,));
        assert_eq!(_2, (1, 11));
        assert_eq!(_3, (1, 11, 21));
        assert_eq!(_4, (1, 11, 21, 31));
        assert_eq!(_5, (1, 11, 21, 31, 41));
    }

    {
        let mut _1 = (0,);
        let mut _2 = (0, 10);
        let mut _3 = (0, 10, 20);
        let mut _4 = (0, 10, 20, 30);
        let mut _5 = (0, 10, 20, 30, 40);

        _1.dec();
        _2.dec();
        _3.dec();
        _4.dec();
        _5.dec();

        assert_eq!(_1, (-1,));
        assert_eq!(_2, (-1, 9));
        assert_eq!(_3, (-1, 9, 19));
        assert_eq!(_4, (-1, 9, 19, 29));
        assert_eq!(_5, (-1, 9, 19, 29, 39));
    }
}

fn assert_output(test_file: &str) {
    let prefix = "tests/ui/".to_string();
    let out_ext = ".output";
    let exp_ext = ".expected";

    let output_path = PathBuf::from_str(&(prefix.clone() + test_file + out_ext)).unwrap();
    let expected_path = PathBuf::from_str(&(prefix + test_file + exp_ext)).unwrap();

    let mut f_output = File::open(output_path).unwrap();
    let mut f_expected = File::open(expected_path).unwrap();

    let mut output = String::new();
    f_output.read_to_string(&mut output).unwrap();

    let mut expected = String::new();
    f_expected.read_to_string(&mut expected).unwrap();

    assert_eq!(output, expected);
}
