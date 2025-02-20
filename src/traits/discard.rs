use crate::types::outcome::Outcome;

pub trait Discard {
    type Output;

    fn discard(self) -> Self::Output;
}

impl<T> Discard for Outcome<T> {
    type Output = Outcome<()>;

    fn discard(self) -> Self::Output {
        self.map(|_| ())
    }
}
