pub mod modifier;
pub mod stat;

#[cfg(feature = "refcell")]
pub mod refcell;
#[cfg(feature = "sync")]
pub mod sync;

pub mod prelude {
    pub use crate::modifier::Additive;
    pub use crate::modifier::Flat;
    pub use crate::modifier::Modifier;
    pub use crate::modifier::Multiplicative;
    pub use crate::stat::Stat;
    pub use crate::stat::StatMarker;
    pub use crate::modifier::shared::All;

    #[cfg(feature = "refcell")]
    pub use crate::refcell::MiniStat;
    #[cfg(feature = "sync")]
    pub use crate::sync::MiniStat as MiniStatSync;


    /// Flat modifier applicable to all stats.
    pub type FlatAll<R, M> = Flat<All<R, M>, R, M>;
    /// Additive modifier applicable to all stats.
    pub type AdditiveAll<R, M> = Additive<All<R, M>, R, M>;
    /// Multiplicative modifier applicable to all stats.
    pub type MultiplicativeAll<R, M> = Multiplicative<All<R, M>, R, M>;
}

mod sealed {
    pub trait Sealed {}
}
