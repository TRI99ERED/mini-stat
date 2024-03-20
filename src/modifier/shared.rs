use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{modifier::Modifier, stat::StatMarker};

#[derive(Clone, Copy, PartialEq)]
pub struct All<Raw, M>(PhantomData<Raw>, M)
where
    Raw: Copy + PartialEq + Add<Output = Raw> + Mul<Output = Raw>,
    M: Copy + PartialEq;

impl<Raw, M> StatMarker for All<Raw, M>
where
    Raw: Copy + PartialEq + Add<Output = Raw> + Mul<Output = Raw>,
    M: Copy + PartialEq,
{
    type Raw = Raw;

    type Metadata = M;
}

pub trait Shared<To: StatMarker>: Modifier {
    type TargetModifier: Modifier<Target = To>;

    fn share(self) -> Self::TargetModifier;
}
