use mini_stat::{modifier::shared::Shared, prelude::*};

#[derive(Debug, Default, Clone, PartialEq)]
struct Dummy;

impl StatMarker for Dummy {
    type Raw = f64;

    type Metadata = &'static str;
}

#[test]
fn specs() {
    let mut stat = Stat::<Dummy>::with_base(3.);
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_flat(Flat::from_raw(1.))
        .apply_flat(Flat::from_raw(2.))
        .apply_flat(Flat::from_raw(3.))
        .cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    let mut stat = Stat::<Dummy>::with_base(6.);
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_add(Additive::from_raw(0.1))
        .apply_add(Additive::from_raw(0.1))
        .apply_add(Additive::from_raw(0.3))
        .cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    let mut stat = Stat::<Dummy>::with_base(1.);
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_mul(Multiplicative::from_raw(0.5))
        .apply_mul(Multiplicative::from_raw(3.))
        .apply_mul(Multiplicative::from_raw(6.))
        .cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    let mut stat = Stat::<Dummy>::with_base(0.);
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_flat(Flat::from_raw(1.))
        .apply_add(Additive::from_raw(0.5))
        .apply_mul(Multiplicative::from_raw(6.))
        .cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    stat.remove_add(Additive::from_raw(1.)).cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");
    assert_eq!(stat.cached().unwrap().round(), 9.);

    stat.remove_add(Additive::from_raw(0.5)).cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");
}

#[derive(Debug, Default, Clone, PartialEq)]
struct A;

impl StatMarker for A {
    type Raw = f64;

    type Metadata = &'static str;
}

#[derive(Debug, Default, Clone, PartialEq)]
struct B;

impl StatMarker for B {
    type Raw = f64;

    type Metadata = &'static str;
}

#[test]
fn two_stats_shared_mod() {
    let mut a = Stat::<A>::with_base(1.);
    let mut b = Stat::<B>::with_base(3.);

    let modifier = FlatAll::from_raw(2.);

    a.apply_flat(modifier.share());

    assert_eq!(a.cache_value().cached(), Some(3.));

    b.apply_flat(modifier.share());

    assert_eq!(b.cache_value().cached(), Some(5.));

    a.apply_flat_from_shared(modifier);

    assert_eq!(a.cache_value().cached(), Some(5.));

    b.apply_flat_from_shared(modifier);

    assert_eq!(b.cache_value().cached(), Some(7.));
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SomeGroup;

impl StatMarker for SomeGroup {
    type Raw = f64;

    type Metadata = &'static str;
}

impl Shared<A> for Flat<SomeGroup, f64, &'static str> {
    type TargetModifier = Flat<A, f64, &'static str>;

    fn share(self) -> Self::TargetModifier {
        Flat::<A, f64, &'static str>::from_raw(self.raw())
    }
}

impl Shared<B> for Flat<SomeGroup, f64, &'static str> {
    type TargetModifier = Flat<B, f64, &'static str>;

    fn share(self) -> Self::TargetModifier {
        Flat::<B, f64, &'static str>::from_raw(self.raw())
    }
}

#[test]
fn custom_group() {
    let mut a = Stat::<A>::with_base(1.);
    let mut b = Stat::<B>::with_base(3.);

    let modifier = Flat::<SomeGroup, f64, &str>::from_raw(2.);

    a.apply_flat(Shared::<A>::share(modifier));

    assert_eq!(a.cache_value().cached(), Some(3.));

    b.apply_flat(Shared::<B>::share(modifier));

    assert_eq!(b.cache_value().cached(), Some(5.));

    a.apply_flat_from_shared(modifier);

    assert_eq!(a.cache_value().cached(), Some(5.));

    b.apply_flat_from_shared(modifier);

    assert_eq!(b.cache_value().cached(), Some(7.));
}
