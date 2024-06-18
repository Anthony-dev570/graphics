use crate::a_or_b::AOrB;

impl<A, B> AOrB<A, B> {
    pub fn is_a(&self) -> bool {
        match self {
            AOrB::A(_) => true,
            AOrB::B(_) => false
        }
    }

    pub fn is_b(&self) -> bool {
        !self.is_a()
    }

    pub fn to_b<AInto: Fn(&A) -> B>(self, a_to_b: AInto) -> Self {
        match self {
            AOrB::A(a) => Self::B(a_to_b(&a)),
            AOrB::B(b) => Self::B(b)
        }
    }

    pub fn b(&self) -> Option<&B> {
        match self {
            AOrB::A(_) => None,
            AOrB::B(b) => Some(b)
        }
    }
}