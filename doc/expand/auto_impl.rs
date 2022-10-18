impl AutoImplTrait for () {
    fn test(&self, _a: i32, _b: &f32) {}
}
impl<Member0> AutoImplTrait for (Member0,)
where
    Member0: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
    }
}
impl<Member0, Member1> AutoImplTrait for (Member0, Member1)
where
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2> AutoImplTrait for (Member0, Member1, Member2)
where
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3> AutoImplTrait for (Member0, Member1, Member2, Member3)
where
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4> AutoImplTrait
    for (Member0, Member1, Member2, Member3, Member4)
where
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5> AutoImplTrait
    for (Member0, Member1, Member2, Member3, Member4, Member5)
where
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6> AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7> AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8> AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8, Member9>
    AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
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
    > AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
    Member10: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
        self.10.test(_a.clone(), _b);
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
    > AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
    Member10: AutoImplTrait,
    Member11: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
        self.10.test(_a.clone(), _b);
        self.11.test(_a.clone(), _b);
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
    > AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
    Member10: AutoImplTrait,
    Member11: AutoImplTrait,
    Member12: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
        self.10.test(_a.clone(), _b);
        self.11.test(_a.clone(), _b);
        self.12.test(_a.clone(), _b);
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
    > AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
    Member10: AutoImplTrait,
    Member11: AutoImplTrait,
    Member12: AutoImplTrait,
    Member13: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
        self.10.test(_a.clone(), _b);
        self.11.test(_a.clone(), _b);
        self.12.test(_a.clone(), _b);
        self.13.test(_a.clone(), _b);
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
    > AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
    Member10: AutoImplTrait,
    Member11: AutoImplTrait,
    Member12: AutoImplTrait,
    Member13: AutoImplTrait,
    Member14: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
        self.10.test(_a.clone(), _b);
        self.11.test(_a.clone(), _b);
        self.12.test(_a.clone(), _b);
        self.13.test(_a.clone(), _b);
        self.14.test(_a.clone(), _b);
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
    > AutoImplTrait
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
    Member0: AutoImplTrait,
    Member1: AutoImplTrait,
    Member2: AutoImplTrait,
    Member3: AutoImplTrait,
    Member4: AutoImplTrait,
    Member5: AutoImplTrait,
    Member6: AutoImplTrait,
    Member7: AutoImplTrait,
    Member8: AutoImplTrait,
    Member9: AutoImplTrait,
    Member10: AutoImplTrait,
    Member11: AutoImplTrait,
    Member12: AutoImplTrait,
    Member13: AutoImplTrait,
    Member14: AutoImplTrait,
    Member15: AutoImplTrait,
{
    fn test(&self, _a: i32, _b: &f32) {
        self.0.test(_a.clone(), _b);
        self.1.test(_a.clone(), _b);
        self.2.test(_a.clone(), _b);
        self.3.test(_a.clone(), _b);
        self.4.test(_a.clone(), _b);
        self.5.test(_a.clone(), _b);
        self.6.test(_a.clone(), _b);
        self.7.test(_a.clone(), _b);
        self.8.test(_a.clone(), _b);
        self.9.test(_a.clone(), _b);
        self.10.test(_a.clone(), _b);
        self.11.test(_a.clone(), _b);
        self.12.test(_a.clone(), _b);
        self.13.test(_a.clone(), _b);
        self.14.test(_a.clone(), _b);
        self.15.test(_a.clone(), _b);
    }
}
