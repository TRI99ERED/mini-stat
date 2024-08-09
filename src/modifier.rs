use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{sealed::Sealed, stat::StatMarker};

pub mod shared;
use shared::{All, Shared};

/// Trait containing common modifier interface.
pub trait Modifier: Sealed {
    /// Target [stat marker][StatMarker] for which this modifier is applicable.
    ///
    /// See [shared] for ways to apply modifier to groups of stat markers.
    type Target: StatMarker;

    /// Raw modifier type. Currently either `f32` or `f64` are allowed.
    type Raw: Copy + PartialEq;

    /// Additional metadata, you may want to store with your modifiers (e.g. description).
    ///
    /// This is normally defined at [StatMarker] level and propagated to modifiers.
    type Metadata: Copy;

    /// Create a modifier value from value of [Raw][Modifier::Raw] type.
    ///
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mini_stat::prelude::{Modifier, FlatAll};
    ///
    /// let modifier = FlatAll::<f32, ()>::from_raw(1.);
    /// #   Ok(())
    /// # }
    /// ```
    fn from_raw(raw: Self::Raw) -> Self;

    /// Get value of underlying [Raw][Modifier::Raw] type.
    ///
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mini_stat::prelude::{Modifier, FlatAll};
    ///
    /// let modifier = FlatAll::<f32, ()>::from_raw(1.);
    ///
    /// assert_eq!(modifier.raw(), 1.);
    /// #   Ok(())
    /// # }
    /// ```
    fn raw(&self) -> Self::Raw;
}

/// Flat modifier (e.g. "+1", "-10"). Applied first to the base value.
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
#[derive(Debug)]
pub struct Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    raw: R,
    metadata: Option<M>,
    _target: PhantomData<S>,
}

impl<S, R, M> PartialEq for Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.metadata == other.metadata
    }
}

impl<S, R, M> Sealed for Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
}

impl<S, R, M> Modifier for Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    type Target = S;

    type Raw = R;

    type Metadata = M;

    fn from_raw(raw: Self::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _target: PhantomData,
        }
    }

    fn raw(&self) -> R {
        self.raw
    }
}

impl<To, R, M> Shared<To> for Flat<All<R, M>, R, M>
where
    To: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    type TargetModifier = Flat<To, R, M>;

    fn share(self) -> Self::TargetModifier {
        Flat::<To, R, M> {
            raw: self.raw,
            metadata: self.metadata,
            _target: PhantomData,
        }
    }
}

impl<S, R, M> Clone for Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, R, M> Copy for Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
}

impl<S, R, M> From<R> for Flat<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn from(value: R) -> Self {
        Self::from_raw(value)
    }
}

impl<S, M> Default for Flat<S, f32, M>
where
    S: StatMarker,
    M: Copy + PartialEq,
{
    fn default() -> Self {
        Self::from_raw(0.)
    }
}

impl<S, M> Default for Flat<S, f64, M>
where
    S: StatMarker,
    M: Copy + PartialEq,
{
    fn default() -> Self {
        Self::from_raw(0.)
    }
}

/// Additive multiplier modifier (e.g. "+1%", "-10%"). Applied second to the base value.
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use mini_stat::prelude::{Modifier, Additive, All};
///
/// let modifier = Additive::<All<f32, ()>, f32, ()>::from_raw(1.);
///
/// assert_eq!(modifier.raw(), 1.);
/// #   Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    raw: R,
    metadata: Option<M>,
    _target: PhantomData<S>,
}

impl<S, R, M> PartialEq for Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.metadata == other.metadata
    }
}

impl<S, R, M> Sealed for Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
}

impl<S, R, M> Modifier for Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    type Target = S;

    type Raw = R;

    type Metadata = M;

    fn from_raw(raw: Self::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _target: PhantomData,
        }
    }

    fn raw(&self) -> R {
        self.raw
    }
}

impl<To, R, M> Shared<To> for Additive<All<R, M>, R, M>
where
    To: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    type TargetModifier = Additive<To, R, M>;

    fn share(self) -> Self::TargetModifier {
        Additive::<To, R, M> {
            raw: self.raw,
            metadata: self.metadata,
            _target: PhantomData,
        }
    }
}

impl<S, R, M> Clone for Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, R, M> Copy for Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
}

impl<S, R, M> From<R> for Additive<S, R, M>
where
    S: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn from(value: R) -> Self {
        Self::from_raw(value)
    }
}

impl<S, M> Default for Additive<S, f32, M>
where
    S: StatMarker,
    M: Copy + PartialEq,
{
    fn default() -> Self {
        Self::from_raw(1.)
    }
}

impl<S, M> Default for Additive<S, f64, M>
where
    S: StatMarker,
    M: Copy + PartialEq,
{
    fn default() -> Self {
        Self::from_raw(1.)
    }
}

/// Multiplicative multiplier modifier (e.g. "1%", "x10"). Applied third to the base value.
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use mini_stat::prelude::{Modifier, Multiplicative, All};
///
/// let modifier = Multiplicative::<All<f32, ()>, f32, ()>::from_raw(1.);
///
/// assert_eq!(modifier.raw(), 1.);
/// #   Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    raw: R,
    metadata: Option<M>,
    _target: PhantomData<S>,
}

impl<S, R, M> PartialEq for Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.metadata == other.metadata
    }
}

impl<S, R, M> Sealed for Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
}

impl<S, R, M> Modifier for Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    type Target = S;

    type Raw = R;

    type Metadata = M;

    fn from_raw(raw: Self::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _target: PhantomData,
        }
    }

    fn raw(&self) -> R {
        self.raw
    }
}

impl<To, R, M> Shared<To> for Multiplicative<All<R, M>, R, M>
where
    To: StatMarker,
    R: Copy + Add<Output = R> + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    type TargetModifier = Multiplicative<To, R, M>;

    fn share(self) -> Self::TargetModifier {
        Multiplicative::<To, R, M> {
            raw: self.raw,
            metadata: self.metadata,
            _target: PhantomData,
        }
    }
}

impl<S, R, M> Clone for Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, R, M> Copy for Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
}

impl<S, R, M> From<R> for Multiplicative<S, R, M>
where
    S: StatMarker,
    R: Copy + Mul<Output = R> + PartialEq,
    M: Copy + PartialEq,
{
    fn from(value: R) -> Self {
        Self::from_raw(value)
    }
}

impl<S, M> Default for Multiplicative<S, f32, M>
where
    S: StatMarker,
    M: Copy + PartialEq,
{
    fn default() -> Self {
        Self::from_raw(1.)
    }
}

impl<S, M> Default for Multiplicative<S, f64, M>
where
    S: StatMarker,
    M: Copy + PartialEq,
{
    fn default() -> Self {
        Self::from_raw(1.)
    }
}
