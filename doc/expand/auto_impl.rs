impl Notify for () {
    fn notify(&self, _a: i32, _b: &f32) {}
}
impl<Member0> Notify for (Member0,)
where
    Member0: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1> Notify for (Member0, Member1)
where
    Member0: Notify,
    Member1: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2> Notify for (Member0, Member1, Member2)
where
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3> Notify for (Member0, Member1, Member2, Member3)
where
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4> Notify
    for (Member0, Member1, Member2, Member3, Member4)
where
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5> Notify
    for (Member0, Member1, Member2, Member3, Member4, Member5)
where
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6> Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7> Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8> Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
    }
}
impl<Member0, Member1, Member2, Member3, Member4, Member5, Member6, Member7, Member8, Member9>
    Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
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
    > Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
    Member10: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
        self.10.notify(_a.clone(), _b);
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
    > Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
    Member10: Notify,
    Member11: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
        self.10.notify(_a.clone(), _b);
        self.11.notify(_a.clone(), _b);
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
    > Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
    Member10: Notify,
    Member11: Notify,
    Member12: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
        self.10.notify(_a.clone(), _b);
        self.11.notify(_a.clone(), _b);
        self.12.notify(_a.clone(), _b);
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
    > Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
    Member10: Notify,
    Member11: Notify,
    Member12: Notify,
    Member13: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
        self.10.notify(_a.clone(), _b);
        self.11.notify(_a.clone(), _b);
        self.12.notify(_a.clone(), _b);
        self.13.notify(_a.clone(), _b);
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
    > Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
    Member10: Notify,
    Member11: Notify,
    Member12: Notify,
    Member13: Notify,
    Member14: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
        self.10.notify(_a.clone(), _b);
        self.11.notify(_a.clone(), _b);
        self.12.notify(_a.clone(), _b);
        self.13.notify(_a.clone(), _b);
        self.14.notify(_a.clone(), _b);
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
    > Notify
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
    Member0: Notify,
    Member1: Notify,
    Member2: Notify,
    Member3: Notify,
    Member4: Notify,
    Member5: Notify,
    Member6: Notify,
    Member7: Notify,
    Member8: Notify,
    Member9: Notify,
    Member10: Notify,
    Member11: Notify,
    Member12: Notify,
    Member13: Notify,
    Member14: Notify,
    Member15: Notify,
{
    fn notify(&self, _a: i32, _b: &f32) {
        self.0.notify(_a.clone(), _b);
        self.1.notify(_a.clone(), _b);
        self.2.notify(_a.clone(), _b);
        self.3.notify(_a.clone(), _b);
        self.4.notify(_a.clone(), _b);
        self.5.notify(_a.clone(), _b);
        self.6.notify(_a.clone(), _b);
        self.7.notify(_a.clone(), _b);
        self.8.notify(_a.clone(), _b);
        self.9.notify(_a.clone(), _b);
        self.10.notify(_a.clone(), _b);
        self.11.notify(_a.clone(), _b);
        self.12.notify(_a.clone(), _b);
        self.13.notify(_a.clone(), _b);
        self.14.notify(_a.clone(), _b);
        self.15.notify(_a.clone(), _b);
    }
}
