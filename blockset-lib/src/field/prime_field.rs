use core::fmt;
use std::marker::PhantomData;

use crate::uint::{
    u256x::{self, U256},
    u512x,
};

use super::prime::Prime;

pub struct PrimeField<P: Prime>(pub U256, PhantomData<P>);

impl<P: Prime> Clone for PrimeField<P> {
    fn clone(&self) -> Self {
        Self::unchecked_new(self.0)
    }
}

impl<P: Prime> PartialEq for PrimeField<P> {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl<P: Prime> fmt::Debug for PrimeField<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<P: Prime> Copy for PrimeField<P> {}

impl<P: Prime> PrimeField<P> {
    pub const P: U256 = P::P;
    pub const OFFSET: u32 = u256x::leading_zeros(Self::P);
    pub const OFFSET8: u32 = Self::OFFSET >> 3 << 3;
    pub const _0: Self = Self::n(0);
    pub const _1: Self = Self::n(1);
    pub const MAX: Self = Self::new(u256x::wsub(Self::P, [1, 0]));
    pub const MIDDLE: Self = Self::new(u256x::shr(&Self::P, 1));
    // (P+1)/4
    const SQRT_K: Self = Self::new(u256x::shr(&u256x::wadd(Self::P, [1, 0]), 2));
    const fn is_valid(key: U256) -> bool {
        u256x::less(&key, &Self::P)
    }
    //
    #[inline(always)]
    pub const fn unchecked_new(num: U256) -> Self {
        Self(num, PhantomData)
    }
    #[inline(always)]
    pub const fn new(mut num: U256) -> Self {
        num = if u256x::less(&num, &Self::P) {
            num
        } else {
            u256x::wsub(num, Self::P)
        };
        Self::unchecked_new(num)
    }
    #[inline(always)]
    pub const fn n(num: u128) -> Self {
        Self::new([num, 0])
    }
    #[inline(always)]
    pub const fn eq(&self, b: &Self) -> bool {
        u256x::eq(&self.0, &b.0)
    }
    //
    pub const fn sub(self, b: Self) -> Self {
        let (mut result, b) = u256x::osub(self.0, b.0);
        if b {
            result = u256x::wadd(result, Self::P)
        }
        Self::unchecked_new(result)
    }
    #[inline(always)]
    pub const fn neg(self) -> Self {
        Self::_0.sub(self)
    }
    pub const fn add(self, b: Self) -> Self {
        self.sub(b.neg())
    }
    #[inline(always)]
    const fn is_neg(&self) -> bool {
        u256x::less(&Self::MIDDLE.0, &self.0)
    }
    #[inline(always)]
    pub const fn abs(self) -> Self {
        if self.is_neg() {
            self.neg()
        } else {
            self
        }
    }
    //
    pub const fn mul(self, b: Self) -> Self {
        Self::unchecked_new(u512x::div_rem(u256x::mul(self.0, b.0), [Self::P, u256x::_0])[1][0])
    }
    pub const fn reciprocal(mut self) -> Self {
        assert!(!Self::_0.eq(&self));
        let mut a0 = Self::P;
        let mut f0 = Self::_0;
        let mut f1 = Self::_1;
        loop {
            if Self::_1.eq(&self) {
                return f1;
            }
            let [q, a2] = u256x::div_rem(a0, self.0);
            a0 = self.0;
            self = Self::unchecked_new(a2);
            let f2 = f0.sub(f1.mul(Self::unchecked_new(q)));
            f0 = f1;
            f1 = f2;
        }
    }
    pub const fn div(self, b: Self) -> Self {
        self.mul(b.reciprocal())
    }
    pub const fn pow(mut self, mut n: Self) -> Self {
        let mut result = Self::_1;
        loop {
            if n.0[0] & 1 == 1 {
                result = result.mul(self);
            }
            n.0 = u256x::shr(&n.0, 1);
            if Self::_0.eq(&n) {
                break;
            }
            self = self.mul(self);
        }
        result
    }
    pub const fn sqrt(self) -> Option<Self> {
        assert!(Self::P[0] & 3 == 3);
        let result = self.pow(Self::SQRT_K);
        if result.mul(result).eq(&self) {
            Some(result)
        } else {
            None
        }
    }
}
