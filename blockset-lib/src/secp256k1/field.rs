use std::marker::PhantomData;

use crate::uint::{
    u256x::{self, U256},
    u512x,
};

trait U256Const {
    const VALUE: U256;
}

struct Field<P: U256Const>(U256, PhantomData<P>);

impl<P: U256Const> Clone for Field<P> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<P: U256Const> Copy for Field<P> {}

impl<P: U256Const> Field<P> {
    const _0: Self = Self::n(0);
    const _1: Self = Self::n(1);
    pub const MAX: Self = Self::new(u256x::wsub(P::VALUE, [1, 0]));
    pub const MIDDLE: Self = Self::new(u256x::shr(&P::VALUE, 1));
    const SQRT_K: Self = Self::new(u256x::shr(&u256x::wadd(P::VALUE, [1, 0]), 2));
    const fn is_valid(key: U256) -> bool {
        u256x::less(&key, &P::VALUE)
    }
    //
    const fn unchecked_new(v: U256) -> Self {
        Self(v, PhantomData)
    }
    #[inline(always)]
    const fn new(num: U256) -> Self {
        assert!(Self::is_valid(num));
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
    const fn sub(self, b: Self) -> Self {
        let (mut result, b) = u256x::osub(self.0, b.0);
        if b {
            result = u256x::wadd(result, P::VALUE)
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
        Self::unchecked_new(u512x::div_rem(u256x::mul(self.0, b.0), [P::VALUE, u256x::ZERO])[1][0])
    }
    pub const fn reciprocal(mut self) -> Self {
        assert!(!Self::_0.eq(&self));
        let mut a0 = P::VALUE;
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
    const fn pow(mut self, mut n: Self) -> Self {
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
}
