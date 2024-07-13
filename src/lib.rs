pub mod modifier;
pub mod stat;

#[cfg(feature = "refcell")]
pub mod mini_stat_single;

#[cfg(feature = "mutex")]
pub mod mini_stat_multi;

pub mod prelude {
    pub use crate::modifier::Additive;
    pub use crate::modifier::Flat;
    pub use crate::modifier::Modifier;
    pub use crate::modifier::Multiplicative;
    pub use crate::stat::Stat;
    pub use crate::stat::StatMarker;

    #[cfg(feature = "refcell")]
    pub use crate::mini_stat_single::MiniStat as MiniStatRefcell;

    #[cfg(feature = "mutex")]
    pub use crate::mini_stat_multi::MiniStat as MiniStatMutex;

    use crate::modifier::shared::All;

    pub type FlatAll<R, M> = Flat<All<R, M>, R, M>;
    pub type AdditiveAll<R, M> = Additive<All<R, M>, R, M>;
    pub type MultiplicativeAll<R, M> = Multiplicative<All<R, M>, R, M>;
}

mod sealed {
    pub trait Sealed {}
}
