use std::cell::RefCell;

use crate::{
    modifier::shared::Shared,
    prelude::{Additive, Flat, Multiplicative, Stat, StatMarker},
};

pub struct MiniStat<Marker, const N: usize = 2>(RefCell<Stat<Marker, N>>)
where
    Marker: StatMarker;

impl<Marker, const N: usize> MiniStat<Marker, N>
where
    Marker: StatMarker,
    Stat<Marker, N>: Default,
    Flat<Marker, Marker::Raw, Marker::Metadata>: Default,
    Additive<Marker, Marker::Raw, Marker::Metadata>: Default,
    Multiplicative<Marker, Marker::Raw, Marker::Metadata>: Default,
{
    pub fn new(stat: Stat<Marker, N>) -> Self {
        Self(RefCell::new(stat))
    }

    pub fn with_base(base: Marker::Raw) -> Self {
        Self(RefCell::new(Stat::<Marker, N>::with_base(base)))
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.borrow().base
    }

    pub fn cached(&self) -> Marker::Raw {
        self.0.borrow_mut().cache_value();
        self.0.borrow().cached().unwrap()
    }

    pub fn apply_flat(&self, flat: Flat<Marker, Marker::Raw, Marker::Metadata>) {
        self.0.borrow_mut().apply_flat(flat);
    }

    pub fn apply_flat_from_shared<T>(&self, flat: T)
    where
        T: Shared<Marker, TargetModifier = Flat<Marker, Marker::Raw, Marker::Metadata>>,
    {
        self.0.borrow_mut().apply_flat_from_shared(flat);
    }

    pub fn apply_add(&self, add: Additive<Marker, Marker::Raw, Marker::Metadata>) {
        self.0.borrow_mut().apply_add(add);
    }

    pub fn apply_add_from_shared<T>(&self, add: T)
    where
        T: Shared<Marker, TargetModifier = Additive<Marker, Marker::Raw, Marker::Metadata>>,
    {
        self.0.borrow_mut().apply_add_from_shared(add);
    }

    pub fn apply_mul(&self, mul: Multiplicative<Marker, Marker::Raw, Marker::Metadata>) {
        self.0.borrow_mut().apply_mul(mul);
    }

    pub fn apply_mul_from_shared<T>(&self, mul: T)
    where
        T: Shared<Marker, TargetModifier = Multiplicative<Marker, Marker::Raw, Marker::Metadata>>,
    {
        self.0.borrow_mut().apply_mul_from_shared(mul);
    }

    pub fn remove_flat(&self, flat: Flat<Marker, Marker::Raw, Marker::Metadata>) {
        self.0.borrow_mut().remove_flat(flat);
    }

    pub fn remove_add(&self, additive: Additive<Marker, Marker::Raw, Marker::Metadata>) {
        self.0.borrow_mut().remove_add(additive);
    }

    pub fn remove_mul(
        &self,
        multiplicative: Multiplicative<Marker, Marker::Raw, Marker::Metadata>,
    ) {
        self.0.borrow_mut().remove_mul(multiplicative);
    }
}
