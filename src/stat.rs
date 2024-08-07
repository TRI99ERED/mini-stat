use std::ops::{Add, Mul};

use smallvec::SmallVec;

use crate::modifier::{shared::Shared, *};

pub trait StatMarker {
    type Raw: Copy + PartialEq + Add<Output = Self::Raw> + Mul<Output = Self::Raw>;
    type Metadata: Copy + PartialEq;
}

#[derive(Debug)]
pub struct Stat<Marker, const N: usize = 2>
where
    Marker: StatMarker,
{
    pub(crate) base: Marker::Raw,
    pub(crate) cached: Option<Marker::Raw>,
    pub(crate) flats: SmallVec<[Flat<Marker, Marker::Raw, Marker::Metadata>; N]>,
    pub(crate) adds: SmallVec<[Additive<Marker, Marker::Raw, Marker::Metadata>; N]>,
    pub(crate) muls: SmallVec<[Multiplicative<Marker, Marker::Raw, Marker::Metadata>; N]>,
}

impl<Marker, const N: usize> Default for Stat<Marker, N>
where
    Marker: StatMarker,
    Marker::Raw: Default,
{
    fn default() -> Self {
        Self {
            base: Default::default(),
            cached: Some(Marker::Raw::default()),
            flats: Default::default(),
            adds: Default::default(),
            muls: Default::default(),
        }
    }
}

impl<Marker, const N: usize> Stat<Marker, N>
where
    Marker: StatMarker,
    Flat<Marker, Marker::Raw, Marker::Metadata>: Default,
    Additive<Marker, Marker::Raw, Marker::Metadata>: Default,
    Multiplicative<Marker, Marker::Raw, Marker::Metadata>: Default,
{
    pub fn with_base(base: Marker::Raw) -> Self
    where
        Self: Default,
    {
        Self {
            base,
            cached: Some(base),
            ..Default::default()
        }
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn cache_value(&mut self) -> &mut Self {
        if self.cached.is_none() {
            let flats = self
                .flats
                .iter()
                .fold(Flat::default().raw(), |acc: Marker::Raw, m| acc + m.raw());
            let adds = self
                .adds
                .iter()
                .fold(Additive::default().raw(), |acc: Marker::Raw, m| {
                    acc + m.raw()
                });
            let muls = self
                .muls
                .iter()
                .fold(Multiplicative::default().raw(), |acc: Marker::Raw, m| {
                    acc * m.raw()
                });
            self.cached = Some((self.base + flats) * adds * muls);
        }
        self
    }

    pub fn cached(&self) -> Option<Marker::Raw> {
        self.cached
    }

    pub fn apply_flat(&mut self, flat: Flat<Marker, Marker::Raw, Marker::Metadata>) -> &mut Self {
        self.flats.push(flat);
        self.cached = None;
        self
    }

    pub fn apply_flat_from_shared<T>(&mut self, flat: T) -> &mut Self
    where
        T: Shared<Marker, TargetModifier = Flat<Marker, Marker::Raw, Marker::Metadata>>,
    {
        self.apply_flat(flat.share())
    }

    pub fn apply_add(
        &mut self,
        additive: Additive<Marker, Marker::Raw, Marker::Metadata>,
    ) -> &mut Self {
        self.adds.push(additive);
        self.cached = None;
        self
    }

    pub fn apply_add_from_shared<T>(&mut self, additive: T) -> &mut Self
    where
        T: Shared<Marker, TargetModifier = Additive<Marker, Marker::Raw, Marker::Metadata>>,
    {
        self.apply_add(additive.share())
    }

    pub fn apply_mul(
        &mut self,
        multiplicative: Multiplicative<Marker, Marker::Raw, Marker::Metadata>,
    ) -> &mut Self {
        self.muls.push(multiplicative);
        self.cached = None;
        self
    }

    pub fn apply_mul_from_shared<T>(&mut self, multiplicative: T) -> &mut Self
    where
        T: Shared<Marker, TargetModifier = Multiplicative<Marker, Marker::Raw, Marker::Metadata>>,
    {
        self.apply_mul(multiplicative.share())
    }

    pub fn remove_flat(&mut self, flat: Flat<Marker, Marker::Raw, Marker::Metadata>) -> &mut Self {
        if let Some(i) = self.flats.iter().position(|&v| v == flat) {
            self.flats.swap_remove(i);
            self.cached = None;
        }
        self
    }

    pub fn remove_add(
        &mut self,
        additive: Additive<Marker, Marker::Raw, Marker::Metadata>,
    ) -> &mut Self {
        if let Some(i) = self.adds.iter().position(|&v| v == additive) {
            self.adds.swap_remove(i);
            self.cached = None;
        }
        self
    }

    pub fn remove_mul(
        &mut self,
        multiplicative: Multiplicative<Marker, Marker::Raw, Marker::Metadata>,
    ) -> &mut Self {
        if let Some(i) = self.muls.iter().position(|&v| v == multiplicative) {
            self.muls.swap_remove(i);
            self.cached = None;
        }
        self
    }

    pub fn flats(
        &self,
    ) -> &SmallVec<[Flat<Marker, <Marker as StatMarker>::Raw, <Marker as StatMarker>::Metadata>; N]>
    {
        &self.flats
    }

    pub fn additives(
        &self,
    ) -> &SmallVec<
        [Additive<Marker, <Marker as StatMarker>::Raw, <Marker as StatMarker>::Metadata>; N],
    > {
        &self.adds
    }

    pub fn multiplicatives(
        &self,
    ) -> &SmallVec<
        [Multiplicative<Marker, <Marker as StatMarker>::Raw, <Marker as StatMarker>::Metadata>; N],
    > {
        &self.muls
    }
}

impl<Marker: StatMarker, const N: usize> Clone for Stat<Marker, N> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            cached: self.cached,
            flats: self.flats.clone(),
            adds: self.adds.clone(),
            muls: self.muls.clone(),
        }
    }
}
