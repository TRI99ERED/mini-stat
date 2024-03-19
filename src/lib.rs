pub mod modifier;
pub mod stat;

pub mod prelude {
    pub use crate::modifier::Additive;
    pub use crate::modifier::Flat;
    pub use crate::modifier::Modifier;
    pub use crate::modifier::Multiplicative;
    pub use crate::stat::Stat;
    pub use crate::stat::StatMarker;

    use crate::modifier::shared::All;

    pub type FlatAll<R> = Flat<All<R>, R>;
    pub type AdditiveAll<R> = Additive<All<R>, R>;
    pub type MultiplicativeAll<R> = Multiplicative<All<R>, R>;

    pub type FlatF32<S> = Flat<S, f32>;
    pub type FlatF64<S> = Flat<S, f64>;
    pub type FlatAllF32 = FlatAll<f32>;
    pub type FlatAllF64 = FlatAll<f64>;

    pub type AdditiveF32<S> = Additive<S, f32>;
    pub type AdditiveF64<S> = Additive<S, f64>;
    pub type AdditiveAllF32 = AdditiveAll<f32>;
    pub type AdditiveAllF64 = AdditiveAll<f64>;

    pub type MultiplicativeF32<S> = Multiplicative<S, f32>;
    pub type MultiplicativeF64<S> = Multiplicative<S, f64>;
    pub type MultiplicativeAllF32 = MultiplicativeAll<f32>;
    pub type MultiplicativeAllF64 = MultiplicativeAll<f64>;
}
