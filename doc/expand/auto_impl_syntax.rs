impl Trait for () {
    fn no_args() {}
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {}
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {}
}
impl<Member0> Trait for (Member0,)
where
    Member0: Trait,
{
    fn no_args() {
        Member0::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1> Trait for (Member0, Member1)
where
    Member0: Trait,
    Member1: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2> Trait for (Member0, Member1, Member2)
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3> Trait for (Member0, Member1, Member2, Member3)
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3, Member4> Trait
    for (Member0, Member1, Member2, Member3, Member4)
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5> Trait
    for (Member0, Member1, Member2, Member3, Member4, Member5)
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8, Member9> Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
    Member10: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
        Member10::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member10::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.10.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
    Member10: Trait,
    Member11: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
        Member10::no_args();
        Member11::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member10::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member11::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.10.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.11.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
    Member10: Trait,
    Member11: Trait,
    Member12: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
        Member10::no_args();
        Member11::no_args();
        Member12::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member10::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member11::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member12::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.10.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.11.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.12.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
    Member10: Trait,
    Member11: Trait,
    Member12: Trait,
    Member13: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
        Member10::no_args();
        Member11::no_args();
        Member12::no_args();
        Member13::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member10::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member11::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member12::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member13::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.10.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.11.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.12.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.13.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
    Member10: Trait,
    Member11: Trait,
    Member12: Trait,
    Member13: Trait,
    Member14: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
        Member10::no_args();
        Member11::no_args();
        Member12::no_args();
        Member13::no_args();
        Member14::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member10::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member11::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member12::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member13::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member14::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.10.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.11.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.12.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.13.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.14.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
impl<
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
        Member15,
    > Trait
    for (
        Member0,
        Member1,
        Member2,
        Member3,
        Member4,
        Member5,
        Member6,
        Member7,
        Member8,
        Member9,
        Member10,
        Member11,
        Member12,
        Member13,
        Member14,
        Member15,
    )
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
    Member4: Trait,
    Member5: Trait,
    Member6: Trait,
    Member7: Trait,
    Member8: Trait,
    Member9: Trait,
    Member10: Trait,
    Member11: Trait,
    Member12: Trait,
    Member13: Trait,
    Member14: Trait,
    Member15: Trait,
{
    fn no_args() {
        Member0::no_args();
        Member1::no_args();
        Member2::no_args();
        Member3::no_args();
        Member4::no_args();
        Member5::no_args();
        Member6::no_args();
        Member7::no_args();
        Member8::no_args();
        Member9::no_args();
        Member10::no_args();
        Member11::no_args();
        Member12::no_args();
        Member13::no_args();
        Member14::no_args();
        Member15::no_args();
    }
    fn assoc_fn_args<T: Clone, U>(_non_ref_arg: T, _ref_arg: &U) {
        Member0::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member1::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member2::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member3::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member4::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member5::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member6::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member7::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member8::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member9::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member10::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member11::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member12::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member13::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member14::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
        Member15::assoc_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
    fn self_fn_args<T: Clone, U>(self, _non_ref_arg: T, _ref_arg: &U) {
        self.0.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.1.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.2.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.3.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.4.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.5.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.6.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.7.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.8.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.9.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.10.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.11.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.12.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.13.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.14.self_fn_args(_non_ref_arg.clone(), _ref_arg);
        self.15.self_fn_args(_non_ref_arg.clone(), _ref_arg);
    }
}
