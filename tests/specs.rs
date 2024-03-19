use mini_stat::{modifier::shared::Shared, prelude::*};

#[derive(Debug, Default, Clone)]
struct Dummy;

impl StatMarker for Dummy {
    type Raw = f64;
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

    stat.remove_additive(Additive::from_raw(1.)).cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");
    assert_eq!(stat.cached().unwrap().round(), 9.);

    stat.remove_additive(Additive::from_raw(0.5)).cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");
}

#[derive(Debug, Default, Clone)]
struct A;

impl StatMarker for A {
    type Raw = f64;
}

#[derive(Debug, Default, Clone)]
struct B;

impl StatMarker for B {
    type Raw = f64;
}

#[test]
fn two_stats_shared_mod() {
    let mut a = Stat::<A>::with_base(1.);
    let mut b = Stat::<B>::with_base(3.);

    let modifier = FlatAllF64::from_raw(2.);

    a.apply_flat(modifier.share());

    b.apply_flat(modifier.share());

    a.apply_flat_from_shared(modifier);

    b.apply_flat_from_shared(modifier);
}

#[derive(Debug, Default, Clone)]
pub struct SomeGroup;

impl StatMarker for SomeGroup {
    type Raw = f64;
}

impl Shared<A> for FlatF64<SomeGroup> {
    type TargetModifier = Flat<A, f64>;

    fn share(self) -> Self::TargetModifier {
        Flat::<A, f64>::from_raw(self.value())
    }
}

impl Shared<B> for FlatF64<SomeGroup> {
    type TargetModifier = Flat<B, f64>;

    fn share(self) -> Self::TargetModifier {
        Flat::<B, f64>::from_raw(self.value())
    }
}

#[test]
fn custom_group() {
    let mut a = Stat::<A>::with_base(1.);
    let mut b = Stat::<B>::with_base(3.);

    let modifier = FlatF64::<SomeGroup>::from_raw(2.);

    a.apply_flat(Shared::<A>::share(modifier));

    b.apply_flat(Shared::<B>::share(modifier));

    a.apply_flat_from_shared(modifier);

    b.apply_flat_from_shared(modifier);
}
