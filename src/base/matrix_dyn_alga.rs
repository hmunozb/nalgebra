use alga::general::{DynamicIdentity, AbstractSemigroup, AbstractGroupAbelian, RingCommutative, ClosedAdd, ClosedNeg, AbstractLoop, AbstractQuasigroup};

use alga::general::{AbstractMagma, AbstractMonoid, AbstractGroup, TwoSidedInverse};
use alga::general::{DynamicMonoid, DynamicLoop, DynamicGroup, DynamicGroupAbelian,
 DynamicAbstractModule, DynamicModule};
use alga::general::{Additive};
use num::{Zero};

use crate::base::{ DMatrix, DVector, RowDVector,
                  Scalar};

impl<N> AbstractMagma<Additive> for DMatrix<N>
    where
        N: Scalar + ClosedAdd,
        //DefaultAllocator: Allocator<N, Dynamic, Dynamic>,
{
    #[inline]
    fn operate(&self, other: &Self) -> Self {
        self + other
    }
}
impl<N> AbstractMagma<Additive> for DVector<N>
    where
        N: Scalar + ClosedAdd,
        //DefaultAllocator: Allocator<N, Dynamic, Dynamic>,
{
    #[inline]
    fn operate(&self, other: &Self) -> Self {
        self + other
    }
}

impl<N> AbstractMagma<Additive> for RowDVector<N>
    where
        N: Scalar + ClosedAdd,
//DefaultAllocator: Allocator<N, Dynamic, Dynamic>,
{
    #[inline]
    fn operate(&self, other: &Self) -> Self {
        self + other
    }
}
impl<N> TwoSidedInverse<Additive> for DMatrix<N>
    where
        N: Scalar + ClosedNeg,
        //DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        -self
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        *self = -self.clone()
    }
}
impl<N> TwoSidedInverse<Additive> for DVector<N>
    where
        N: Scalar + ClosedNeg,
//DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        -self
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        *self = -self.clone()
    }
}

impl<N> TwoSidedInverse<Additive> for RowDVector<N>
    where
        N: Scalar + ClosedNeg,
//DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        -self
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        *self = -self.clone()
    }
}

impl<N> DynamicIdentity<Additive>
for DMatrix<N>
where N: Scalar+Zero
{
    fn identity(&self) -> Self{
        self.map(|_| N::zero())
    }
}

impl<N> DynamicIdentity<Additive>
for DVector<N>
    where N: Scalar+Zero
{
    fn identity(&self) -> Self{
        self.map(|_| N::zero())
    }
}

impl<N> DynamicIdentity<Additive>
for RowDVector<N>
    where N: Scalar+Zero
{
    fn identity(&self) -> Self{
        self.map(|_| N::zero())
    }
}



macro_rules! inherit_additive_structure(
    ($($dynmarker: ident<$operator: ident>, $marker:ident $(+ $bounds: ident)*);* $(;)*) => {$(
        impl<N> $dynmarker<$operator> for DMatrix<N>
            where N: Scalar + $marker<$operator> $(+ $bounds)*,
                  //DefaultAllocator: Allocator<N, Dynamic, Dynamic>
                   { }

        impl<N> $dynmarker<$operator> for DVector<N>
            where N: Scalar + $marker<$operator> $(+ $bounds)*,
                  //DefaultAllocator: Allocator<N, Dynamic, U1>
                  { }

        impl<N> $dynmarker<$operator> for RowDVector<N>
            where N: Scalar + $marker<$operator> $(+ $bounds)*,
                  //DefaultAllocator: Allocator<N, U1, Dynamic>
                  { }
    )*}
);

inherit_additive_structure!(
    AbstractSemigroup<Additive>, AbstractSemigroup + ClosedAdd;
    AbstractQuasigroup<Additive>, AbstractQuasigroup + ClosedAdd + ClosedNeg;
    DynamicLoop<Additive>, AbstractLoop + ClosedAdd + Zero + ClosedNeg;
    DynamicMonoid<Additive>, AbstractMonoid + Zero + ClosedAdd;
    DynamicGroup<Additive>, AbstractGroup + Zero + ClosedAdd + ClosedNeg;
    DynamicGroupAbelian<Additive>, AbstractGroupAbelian + Zero + ClosedAdd + ClosedNeg;
);

//impl<N> AbstractSemigroup<Additive> for DMatrix<N>
//where N: Scalar+AbstractSemigroup<Additive> + ClosedAdd {}
//
//impl<N> AbstractQuasigroup<Additive> for DMatrix<N>
//where N: Scalar + AbstractQuasigroup<Additive> + ClosedAdd + ClosedNeg
//{}

//impl<N> DynamicLoop<Additive> for DMatrix<N>
//where N: Scalar + DynamicLoop<Additive> + ClosedAdd + Zero + ClosedNeg
//{ }

//impl<N> DynamicMonoid<Additive> for DMatrix<N>
//where N: Scalar+AbstractMonoid<Additive> + Zero + ClosedAdd{ }

//impl<N> DynamicGroup<Additive> for DMatrix<N>
//where N: Scalar+AbstractGroup<Additive> + Zero + ClosedAdd + ClosedNeg{ }

//impl<N> DynamicGroupAbelian<Additive> for DMatrix<N>
//where N: Scalar+AbstractGroupAbelian<Additive> + Zero + ClosedAdd + ClosedNeg{ }

impl<N> DynamicAbstractModule for DMatrix<N>
    where N:Scalar + RingCommutative
{
    type AbstractRing = N;

    #[inline]
    fn multiply_by(&self, n: N) -> Self{ self * n}
}

impl<N> DynamicAbstractModule for DVector<N>
    where N:Scalar + RingCommutative
{
    type AbstractRing = N;

    #[inline]
    fn multiply_by(&self, n: N) -> Self{ self * n}
}
impl<N> DynamicAbstractModule for RowDVector<N>
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

impl<N> DynamicModule for DVector<N>
    where N:Scalar+RingCommutative
{
    type Ring = N;
}

impl<N> DynamicModule for RowDVector<N>
    where N:Scalar+RingCommutative
{
    type Ring = N;
}