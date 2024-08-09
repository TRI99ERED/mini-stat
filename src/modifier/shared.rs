use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{modifier::Modifier, stat::StatMarker};

/// A shared modifier group ([`StatMarker`]) for modifiers applicable to all user defined stat markers
/// with same [raw type][Modifier::Raw] and [metadata][Modifier::Metadata].
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use mini_stat::prelude::{Modifier, Flat, All};
///
/// let modifier = Flat::<All<f32, ()>, f32, ()>::from_raw(1.);
///
/// assert_eq!(modifier.raw(), 1.);
/// #   Ok(())
/// # }
/// ```
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

/// A trait defining a group of modifiers applicable to multiple [stat markers][StatMarker].
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #[derive(Debug, Default, Clone, PartialEq)]
/// use mini_stat::{modifier::shared::Shared, prelude::*};
/// 
/// pub struct SomeGroup;
/// 
/// impl StatMarker for SomeGroup {
///     type Raw = f64;
/// 
///     type Metadata = &'static str;
/// }
/// 
/// impl Shared<A> for Flat<SomeGroup, f64, &'static str> {
///     type TargetModifier = Flat<A, f64, &'static str>;
/// 
///     fn share(self) -> Self::TargetModifier {
///         Flat::<A, f64, &'static str>::from_raw(self.raw())
///     }
/// }
/// 
/// impl Shared<B> for Flat<SomeGroup, f64, &'static str> {
///     type TargetModifier = Flat<B, f64, &'static str>;
/// 
///     fn share(self) -> Self::TargetModifier {
///         Flat::<B, f64, &'static str>::from_raw(self.raw())
///     }
/// }
/// 
/// let mut a = Stat::<A>::with_base(1.);
/// let mut b = Stat::<B>::with_base(3.);
/// 
/// let modifier = Flat::<SomeGroup, f64, &str>::from_raw(2.);
/// 
/// a.apply_flat(Shared::<A>::share(modifier));
/// 
/// assert_eq!(a.cache_value().cached(), Some(3.));
/// 
/// b.apply_flat(Shared::<B>::share(modifier));
/// 
/// assert_eq!(b.cache_value().cached(), Some(5.));
/// #   Ok(())
/// # }
/// ```
pub trait Shared<To: StatMarker>: Modifier {
    /// Modifier, which will get shared to the new target [`StatMarker`].
    type TargetModifier: Modifier<Target = To>;

    /// Shares the modifier to the target.
    fn share(self) -> Self::TargetModifier;
}
