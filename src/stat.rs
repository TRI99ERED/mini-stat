use std::ops::{Add, Mul};

use crate::modifier::{shared::Shared, *};

pub trait StatMarker: Clone {
    type Raw: Clone + Copy + PartialEq + Add<Output = Self::Raw> + Mul<Output = Self::Raw>;
}

#[derive(Debug, Clone)]
pub struct Stat<Marker>
where
    Marker: StatMarker,
{
    base: Marker::Raw,
    cached: Option<Marker::Raw>,
    flats: Vec<Flat<Marker, Marker::Raw>>,
    additives: Vec<Additive<Marker, Marker::Raw>>,
    multiplicatives: Vec<Multiplicative<Marker, Marker::Raw>>,
}

impl<Marker: Default> Default for Stat<Marker>
where
    Marker: StatMarker,
    Marker::Raw: Default,
{
    fn default() -> Self {
        Self {
            base: Default::default(),
            cached: Some(Marker::Raw::default()),
            flats: Default::default(),
            additives: Default::default(),
            multiplicatives: Default::default(),
        }
    }
}

impl<Marker> Stat<Marker>
where
    Marker: StatMarker,
    Flat<Marker, Marker::Raw>: Default,
    Additive<Marker, Marker::Raw>: Default,
    Multiplicative<Marker, Marker::Raw>: Default,
{
    pub fn with_base(base: Marker::Raw) -> Self
    where
        Self: Default,
    {
        Stat {
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
            let fadds = self
                .flats
                .iter()
                .fold(Flat::default(), |acc: Flat<Marker, Marker::Raw>, m| {
                    acc.combine(m)
                });
            let madds = self.additives.iter().fold(
                Additive::default(),
                |acc: Additive<Marker, Marker::Raw>, m| acc.combine(m),
            );
            let mmuls = self.multiplicatives.iter().fold(
                Multiplicative::default(),
                |acc: Multiplicative<Marker, Marker::Raw>, m| acc.combine(m),
            );
            self.cached = Some((self.base + fadds.value()) * madds.value() * mmuls.value());
        }
        self
    }

    pub fn cached(&self) -> Option<Marker::Raw> {
        self.cached
    }

    pub fn apply_flat(&mut self, flat: Flat<Marker, Marker::Raw>) -> &mut Self {
        self.flats.push(flat);
        self.cached = None;
        self
    }

    pub fn apply_flat_from_shared<T>(&mut self, flat: T) -> &mut Self
    where
        T: Shared<Marker, TargetModifier = Flat<Marker, Marker::Raw>>,
    {
        self.apply_flat(flat.share())
    }

    pub fn apply_add(&mut self, additive: Additive<Marker, Marker::Raw>) -> &mut Self {
        self.additives.push(additive);
        self.cached = None;
        self
    }

    pub fn apply_add_from_shared<T>(&mut self, additive: T) -> &mut Self
    where
        T: Shared<Marker, TargetModifier = Additive<Marker, Marker::Raw>>,
    {
        self.apply_add(additive.share())
    }

    pub fn apply_mul(&mut self, multiplicative: Multiplicative<Marker, Marker::Raw>) -> &mut Self {
        self.multiplicatives.push(multiplicative);
        self.cached = None;
        self
    }

    pub fn apply_mul_from_shared<T>(&mut self, multiplicative: T) -> &mut Self
    where
        T: Shared<Marker, TargetModifier = Multiplicative<Marker, Marker::Raw>>,
    {
        self.apply_mul(multiplicative.share())
    }

    pub fn remove_flat(&mut self, flat: Flat<Marker, Marker::Raw>) -> &mut Self {
        if let Some(i) = self.flats.iter().position(|&v| v == flat) {
            self.flats.swap_remove(i);
            self.cached = None;
        }
        self
    }

    pub fn remove_additive(&mut self, additive: Additive<Marker, Marker::Raw>) -> &mut Self {
        if let Some(i) = self.additives.iter().position(|&v| v == additive) {
            self.additives.swap_remove(i);
            self.cached = None;
        }
        self
    }

    pub fn remove_multiplicative(
        &mut self,
        multiplicative: Multiplicative<Marker, Marker::Raw>,
    ) -> &mut Self {
        if let Some(i) = self
            .multiplicatives
            .iter()
            .position(|&v| v == multiplicative)
        {
            self.multiplicatives.swap_remove(i);
            self.cached = None;
        }
        self
    }

    pub fn flats(&self) -> &impl IntoIterator<Item = Flat<Marker, Marker::Raw>> {
        &self.flats
    }

    pub fn additives(&self) -> &impl IntoIterator<Item = Additive<Marker, Marker::Raw>> {
        &self.additives
    }

    pub fn multiplicatives(
        &self,
    ) -> &impl IntoIterator<Item = Multiplicative<Marker, Marker::Raw>> {
        &self.multiplicatives
    }
}
