#[fortuples::auto_impl]
trait SimpleTest {}

impl SimpleTest for i32 {}
impl SimpleTest for f32 {}

#[fortuples::auto_impl]
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
#[tuples::max_size(2)]
trait Args {
    fn bias(&self, out: &mut i32);

    fn sum(&mut self, arg: i32, arg_ref: &i32, out: &mut i32) {
        self.bias(out);
        *out += arg + arg_ref;
    }

    fn diff(arg_ref: &i32, arg: i32, out: &mut i32) {
        *out -= arg_ref - arg;
    }
}

impl Args for i32 {
    fn bias(&self, out: &mut i32) {
        *out += self;
    }
}

#[fortuples::auto_impl]
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
    <() as NoArgs>::foo();
    <() as NoArgs>::bar();

    <(i32,) as NoArgs>::foo();
    <(i32,) as NoArgs>::bar();

    <(i32, char) as NoArgs>::foo();
    <(i32, char) as NoArgs>::bar();
}

#[test]
fn test_auto_impl_only_self() {
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

#[test]
fn test_auto_impl_args() {

    let a = 42;
    let b = 112;

    {
        let mut out = 0;
        ().sum(a, &b, &mut out);

        assert_eq!(out, 0);
    }

    {
        let mut out = 0;
        <() as Args>::diff(&a, b, &mut out);

        assert_eq!(out, 0);
    }

    {
        let mut out = 0;
        let num = 10;

        (num,).sum(a, &b, &mut out);

        assert_eq!(out, a + b + num);
    }

    {
        let mut out = 0;

        <(i32,) as Args>::diff(&a, a, &mut out);

        assert_eq!(out, 0);
    }

    {
        let mut out = 0;
        let num = 10;

        (num, ()).sum(a, &b, &mut out);

        assert_eq!(out, a + b + num);
    }

    {
        let mut out = 0;

        <(i32, ()) as Args>::diff(&a, a, &mut out);

        assert_eq!(out, 0);
    }
}
