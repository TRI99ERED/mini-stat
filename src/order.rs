use crate::sealed::Sealed;

pub trait Order: Sealed {}

pub struct At<const INDEX: isize>;

impl<const INDEX: isize> Order for At<INDEX> {}

impl<const INDEX: isize> Sealed for At<INDEX> {}

pub type First = At<{ isize::MIN }>;
pub type Last = At<{ isize::MAX }>;
pub type Middle = At<0>;
