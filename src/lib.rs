use std::ops::*;
macro_rules! impl_op {
    {$trait: ident < $( $bound: ident ),+ > => $imp: item} => {
        impl<T> $trait for Coordinates<T>
            where
                T: $trait< $($bound),*, Output=T>,
        {
            type Output = Self;
            $imp
        }
    };
    {$trait: ident => $imp: item} => {
        impl<T> $trait for Coordinates<T>
            where
                T: $trait<Output=T>,
        {
            type Output = Self;
            $imp
        }
    };

}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates<T>(T, T);

impl_op! {
    Add => fn add(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl_op! {
    Neg => fn neg(self                     ) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
impl_op! {
    Sub => fn sub(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl_op! {
    Mul => fn mul(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl_op! {
    Div => fn div(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Coordinates;

    #[test]
    fn add() {
        let pos = Coordinates(0, 0);
        let vel = Coordinates(2, 2);

        assert_eq!(Coordinates(2, 2), pos + vel);
        assert_eq!(Coordinates(0, 0), pos);
    }
    #[test]
    fn neg() {
        let pos = Coordinates(1, 0);

        assert_eq!(Coordinates(-1, 0), -pos);
        assert_eq!(Coordinates(1, 0), pos);
    }
    #[test]
    fn mul() {
        let pos = Coordinates(2, 0);
        let vel = Coordinates(2, 2);

        assert_eq!(Coordinates(4, 0), pos * vel);
        assert_eq!(Coordinates(2, 0), pos);
    }
}
