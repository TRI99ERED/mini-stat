use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{modifier::Modifier, stat::StatMarker};

pub struct All<Raw>(PhantomData<Raw>)
where
    Raw: Copy + PartialEq + Add<Output = Raw> + Mul<Output = Raw>;

impl<Raw> StatMarker for All<Raw>
where
    Raw: Copy + PartialEq + Add<Output = Raw> + Mul<Output = Raw>,
{
    type Raw = Raw;
}

pub trait Shared<To: StatMarker>: Modifier {
    type TargetModifier: Modifier<Target = To>;

    fn share(self) -> Self::TargetModifier;
}
