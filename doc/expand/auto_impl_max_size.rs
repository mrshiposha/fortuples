impl Trait for () {}
impl<Member0> Trait for (Member0,) where Member0: Trait {}
impl<Member0, Member1> Trait for (Member0, Member1)
where
    Member0: Trait,
    Member1: Trait,
{
}
impl<Member0, Member1, Member2> Trait for (Member0, Member1, Member2)
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
{
}
impl<Member0, Member1, Member2, Member3> Trait for (Member0, Member1, Member2, Member3)
where
    Member0: Trait,
    Member1: Trait,
    Member2: Trait,
    Member3: Trait,
{
}
