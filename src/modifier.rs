use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{
    stat::StatMarker,
    shared::{All, Shared},
};

pub trait Modifier: Copy {
    type Target: StatMarker;
    type Raw: Copy + PartialEq;

    fn from_raw(raw: Self::Raw) -> Self;

    fn value(&self) -> Self::Raw;

    fn combine(&self, other: &Self) -> Self;
}

#[derive(Debug)]
pub struct Flat<S: StatMarker, R: Copy + Add<Output = R> + PartialEq>(R, PhantomData<S>);

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> Modifier for Flat<S, R> {
    type Target = S;

    type Raw = R;

    fn from_raw(raw: Self::Raw) -> Self {
        Self(raw, PhantomData)
    }

    fn value(&self) -> R {
        self.0
    }

    fn combine(&self, other: &Self) -> Self {
        Self(self.0 + other.0, PhantomData)
    }
}

impl<To, R> Shared<To> for Flat<All<R>, R>
where
    To: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
{
    type TargetModifier = Flat<To, R>;
    
    fn share(self) -> Self::TargetModifier {
        Flat::<To, R>(self.0, PhantomData)
    }
}

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> Clone for Flat<S, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> Copy for Flat<S, R> {}

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> PartialEq for Flat<S, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: StatMarker> Default for Flat<S, f32> {
    fn default() -> Self {
        Self(0., PhantomData)
    }
}

impl<S: StatMarker> Default for Flat<S, f64> {
    fn default() -> Self {
        Self(0., PhantomData)
    }
}

#[derive(Debug)]
pub struct Additive<S: StatMarker, R: Copy + Add<Output = R> + PartialEq>(R, PhantomData<S>);

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> Modifier for Additive<S, R> {
    type Target = S;

    type Raw = R;

    fn from_raw(raw: Self::Raw) -> Self {
        Self(raw, PhantomData)
    }

    fn value(&self) -> R {
        self.0
    }

    fn combine(&self, other: &Self) -> Self {
        Self(self.0 + other.0, PhantomData)
    }
}

impl<To, R> Shared<To> for Additive<All<R>, R>
where
    To: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
{
    type TargetModifier = Additive<To, R>;
    
    fn share(self) -> Self::TargetModifier {
        Additive::<To, R>(self.0, PhantomData)
    }
}

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> Clone for Additive<S, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> Copy for Additive<S, R> {}

impl<S: StatMarker, R: Copy + Add<Output = R> + PartialEq> PartialEq for Additive<S, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: StatMarker> Default for Additive<S, f32> {
    fn default() -> Self {
        Self(1., PhantomData)
    }
}

impl<S: StatMarker> Default for Additive<S, f64> {
    fn default() -> Self {
        Self(1., PhantomData)
    }
}

#[derive(Debug)]
pub struct Multiplicative<S: StatMarker, R: Copy + Mul<Output = R> + PartialEq>(R, PhantomData<S>);

impl<S: StatMarker, R: Copy + Mul<Output = R> + PartialEq> Modifier for Multiplicative<S, R> {
    type Target = S;

    type Raw = R;

    fn from_raw(raw: Self::Raw) -> Self {
        Self(raw, PhantomData)
    }

    fn value(&self) -> R {
        self.0
    }

    fn combine(&self, other: &Self) -> Self {
        Self(self.0 * other.0, PhantomData)
    }
}

impl<To, R> Shared<To> for Multiplicative<All<R>, R>
where
    To: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
{
    type TargetModifier = Multiplicative<To, R>;
    
    fn share(self) -> Self::TargetModifier {
        Multiplicative::<To, R>(self.0, PhantomData)
    }
}

impl<S: StatMarker, R: Copy + Mul<Output = R> + PartialEq> Clone for Multiplicative<S, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: StatMarker, R: Copy + Mul<Output = R> + PartialEq> Copy for Multiplicative<S, R> {}

impl<S: StatMarker, R: Copy + Mul<Output = R> + PartialEq> PartialEq for Multiplicative<S, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: StatMarker> Default for Multiplicative<S, f32> {
    fn default() -> Self {
        Self(1., PhantomData)
    }
}

impl<S: StatMarker> Default for Multiplicative<S, f64> {
    fn default() -> Self {
        Self(1., PhantomData)
    }
}
