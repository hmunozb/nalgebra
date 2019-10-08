use alga::general::{DynamicIdentity, AbstractSemigroup, AbstractGroupAbelian, RingCommutative, ClosedAdd, ClosedNeg, AbstractLoop, AbstractQuasigroup};

use alga::general::{AbstractMonoid, AbstractGroup};
use alga::general::{DynamicMonoid, DynamicLoop, DynamicGroup, DynamicGroupAbelian, DynamicRing,
DynamicRingCommutative, DynamicAbstractModule, DynamicModule};
use alga::general::{Additive, Multiplicative};
use num::{One, Zero};

use crate::base::{DMatrix, Scalar};

impl<N> DynamicIdentity<Additive> for DMatrix<N>
where N: Scalar+Zero{
    fn identity(&self) -> Self{
        self.map(|_| N::zero())
    }
}

impl<N> AbstractSemigroup<Additive> for DMatrix<N>
where N: Scalar+AbstractSemigroup<Additive> + ClosedAdd {

}

impl<N> AbstractQuasigroup<Additive> for DMatrix<N>
where N: Scalar + AbstractQuasigroup<Additive> + ClosedAdd + ClosedNeg
{

}
impl<N> DynamicLoop<Additive> for DMatrix<N>
where N: Scalar + DynamicLoop<Additive> + ClosedAdd + Zero + ClosedNeg
{ }

impl<N> DynamicMonoid<Additive> for DMatrix<N>
where N: Scalar+AbstractMonoid<Additive> + Zero + ClosedAdd{ }

impl<N> DynamicGroup<Additive> for DMatrix<N>
where N: Scalar+AbstractGroup<Additive> + Zero + ClosedAdd + ClosedNeg{ }

impl<N> DynamicGroupAbelian<Additive> for DMatrix<N>
where N: Scalar+AbstractGroupAbelian<Additive> + Zero + ClosedAdd + ClosedNeg{ }

impl<N> DynamicAbstractModule for DMatrix<N>
where N:Scalar + RingCommutative
{
    type AbstractRing = N;

    #[inline]
    fn multiply_by(&self, n: N) -> Self{ self * n}
}

impl<N> DynamicModule for DMatrix<N>
where N:Scalar+RingCommutative
{
    type Ring = N;
}