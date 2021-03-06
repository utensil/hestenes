use std::fmt;
use std::ops::BitXor;
use dimension::{CountBits, Dimension};
use ops::OuterProduct;
use num::Real;
use unit_basis_blade::UnitBasisBlade;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ScaledBasisBlade<R: Real, D: Dimension> {
    scale: R,
    unit_basis_blade: UnitBasisBlade<D>,
}

impl<R: Real, D: Dimension> ScaledBasisBlade<R, D> {
    pub fn zero() -> Self {
        ScaledBasisBlade {
            scale: R::zero(),
            unit_basis_blade: UnitBasisBlade::zero(),
        }
    }

    pub fn new(scale: R, unit_basis_blade: UnitBasisBlade<D>) -> Self {
        if unit_basis_blade.is_zero() || scale.is_zero() {
            Self::zero()
        } else {
            Self {
                scale,
                unit_basis_blade,
            }
        }
    }

    pub fn scale(&self) -> R {
        self.scale
    }

    pub fn unit_basis_blade(&self) -> &UnitBasisBlade<D> {
        &self.unit_basis_blade
    }

    pub fn is_zero(&self) -> bool {
        self.scale.is_zero()
    }
}

impl<R: Real, D: Dimension> From<UnitBasisBlade<D>> for ScaledBasisBlade<R, D> {
    fn from(unit_basis_blade: UnitBasisBlade<D>) -> Self {
        ScaledBasisBlade::new(R::one(), unit_basis_blade)
    }
}

impl<R: Real, D: Dimension> From<(R, UnitBasisBlade<D>)> for ScaledBasisBlade<R, D> {
    fn from(tuple: (R, UnitBasisBlade<D>)) -> Self {
        let (scale, unit_basis_blade) = tuple;

        ScaledBasisBlade::new(scale, unit_basis_blade)
    }
}

impl<R: Real, D: Dimension> fmt::Display for ScaledBasisBlade<R, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
        write!(f, "{}", self.scale())?;
        for (i, has_base) in self.unit_basis_blade().basis_vectors().iter().enumerate() {
            if *has_base {
                write!(f, "\\boldsymbol{{e}}_{}", i + 1)?;
            }
        }

        Ok(())
    }
}

impl_operator_outer! {
    inline: [false];
    generics: [R: Real, D: Dimension];
    header: (ScaledBasisBlade<R, D>, ScaledBasisBlade<R, D>) -> ScaledBasisBlade<R, D>;
    |&lhs, &rhs| {
        let mut lbs = lhs.unit_basis_blade().bitset();
        let rbs = rhs.unit_basis_blade().bitset();

        // Check for linear dependency
        if lbs & rbs != 0 {
            // If two blades are linearly dependent, the result is 0.
            return ScaledBasisBlade::zero();
        }

        let mut scale = lhs.scale() * rhs.scale();

        if scale.is_zero() {
            return ScaledBasisBlade::zero();
        }

        let resulting_bitset = lbs | rbs;
        let mut total_swaps = 0;

        while lbs > 1 {
            lbs >>= 1;
            total_swaps += (lbs & rbs).count_bits();
        }

        // Negate the scale if the number of swaps was odd
        scale = if total_swaps % 2 == 0 { scale } else { scale.neg() };

        ScaledBasisBlade::new(scale, UnitBasisBlade::new(resulting_bitset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::U3;

    #[test]
    fn outer_product_1() {
        let a: ScaledBasisBlade<f32, U3> = (2.0, 0b110.into()).into();
        let b: ScaledBasisBlade<f32, U3> = (3.0, 0b001.into()).into();

        assert_eq!(a^b, ScaledBasisBlade::new(6.0, 0b111.into()));
    }

    #[test]
    fn outer_product_2() {
        let a: ScaledBasisBlade<f32, U3> = (5.0, 0b110.into()).into();
        let b: ScaledBasisBlade<f32, U3> = (7.0, 0b110.into()).into();

        assert_eq!(a^b, ScaledBasisBlade::new(35.0, 0b000.into()));
    }
}
