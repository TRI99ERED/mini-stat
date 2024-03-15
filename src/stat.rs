use std::ops::{Add, Mul};

use crate::{modifier::*, shared::Shared};

pub trait StatMarker {
    type Raw: Copy + PartialEq + Add<Output = Self::Raw> + Mul<Output = Self::Raw>;
}

#[derive(Debug, Default)]
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
            ..Default::default()
        }
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn cache_value(&mut self) {
        if let None = self.cached {
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
    }

    pub fn cached(&self) -> Option<Marker::Raw> {
        self.cached
    }

    pub fn apply_flat(&mut self, flat: Flat<Marker, Marker::Raw>) {
        self.flats.push(flat);
        self.cached = None;
    }

    pub fn apply_flat_shared<T>(&mut self, flat: T)
    where
        T: Shared<Marker, TargetModifier = Flat<Marker, Marker::Raw>>,
    {
        self.apply_flat(flat.share())
    }

    pub fn apply_additive(&mut self, additive: Additive<Marker, Marker::Raw>) {
        self.additives.push(additive);
        self.cached = None;
    }

    pub fn apply_additive_shared<T>(&mut self, additive: T)
    where
        T: Shared<Marker, TargetModifier = Additive<Marker, Marker::Raw>>,
    {
        self.apply_additive(additive.share())
    }

    pub fn apply_multiplicative(&mut self, multiplicative: Multiplicative<Marker, Marker::Raw>) {
        self.multiplicatives.push(multiplicative);
        self.cached = None;
    }

    pub fn apply_multiplicative_shared<T>(&mut self, multiplicative: T)
    where
        T: Shared<Marker, TargetModifier = Multiplicative<Marker, Marker::Raw>>,
    {
        self.apply_multiplicative(multiplicative.share())
    }

    pub fn remove_flat(&mut self, flat: Flat<Marker, Marker::Raw>) {
        if let Some(i) = self.flats.iter().position(|&v| v == flat) {
            self.flats.swap_remove(i);
            self.cached = None;
        }
    }

    pub fn remove_additive(&mut self, additive: Additive<Marker, Marker::Raw>) {
        if let Some(i) = self.additives.iter().position(|&v| v == additive) {
            self.additives.swap_remove(i);
            self.cached = None;
        }
    }

    pub fn remove_multiplicative(&mut self, multiplicative: Multiplicative<Marker, Marker::Raw>) {
        if let Some(i) = self
            .multiplicatives
            .iter()
            .position(|&v| v == multiplicative)
        {
            self.multiplicatives.swap_remove(i);
            self.cached = None;
        }
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
