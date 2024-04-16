use std::ops::*;
macro_rules! impl_ops {
    {$($ty: ty => $imp: item);+} => {
        $(impl<T> $ty for Coordinates<T>
            where
                T:    Add<T, Output = T>
                    + Neg<   Output = T>
                    + Sub<T, Output = T>
                    + Mul<T, Output = T>
                    + Div<T, Output = T>,
        {
            type Output = Self;
            $imp
        })+
    };
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates<T>(T, T)
where
    T:    Add<T, Output = T>
        + Neg<   Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>;

impl_ops! {
    Add => fn add(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    };
    Neg => fn neg(self                     ) -> Self::Output {
        Self(-self.0, -self.1)
    };
    Sub => fn sub(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    };
    Mul => fn mul(self, rhs: Coordinates<T>) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    };
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
}
