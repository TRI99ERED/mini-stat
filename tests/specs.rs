use mini_stat::{
    modifier::{Additive, Flat, Modifier, Multiplicative},
    stat::{Stat, StatMarker}, shared::{All, Shared},
};

#[derive(Debug, Default)]
struct Dummy;

impl StatMarker for Dummy {
    type Raw = f64;
}

#[test]
fn specs() {
    let mut stat = Stat::<Dummy>::with_base(3.);
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_flat(Flat::from_raw(1.));
    stat.apply_flat(Flat::from_raw(2.));
    stat.apply_flat(Flat::from_raw(3.));
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    let mut stat = Stat::<Dummy>::with_base(6.);
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_additive(Additive::from_raw(0.1));
    stat.apply_additive(Additive::from_raw(0.1));
    stat.apply_additive(Additive::from_raw(0.3));
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    let mut stat = Stat::<Dummy>::with_base(1.);
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_multiplicative(Multiplicative::from_raw(0.5));
    stat.apply_multiplicative(Multiplicative::from_raw(3.));
    stat.apply_multiplicative(Multiplicative::from_raw(6.));
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    let mut stat = Stat::<Dummy>::with_base(0.);
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());

    stat.apply_flat(Flat::from_raw(1.));
    stat.apply_additive(Additive::from_raw(0.5));
    stat.apply_multiplicative(Multiplicative::from_raw(6.));
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");

    assert_eq!(stat.cached().unwrap().round(), 9.);

    stat.remove_additive(Additive::from_raw(1.));
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");
    assert_eq!(stat.cached().unwrap().round(), 9.);

    stat.remove_additive(Additive::from_raw(0.5));
    stat.cache_value();
    println!("{:.0}", stat.cached().unwrap());
    println!("{stat:#?}");
}

#[derive(Debug, Default)]
struct A;

impl StatMarker for A {
    type Raw = f64;
}

#[derive(Debug, Default)]
struct B;

impl StatMarker for B {
    type Raw = f64;
}

#[test]
fn two_stats_shared_mod() {
    let mut a = Stat::<A>::with_base(1.);
    let mut b = Stat::<B>::with_base(3.);

    let modifier = Flat::<All<f64>, f64>::from_raw(2.);

    a.apply_flat(modifier.share());

    b.apply_flat(modifier.share());

    a.apply_flat_shared(modifier);

    b.apply_flat_shared(modifier);
}

pub struct SomeGroup;

impl StatMarker for SomeGroup
{
    type Raw = f64;
}

impl<R> Shared<A> for Flat<SomeGroup, R>
where
    R: Copy + std::ops::Add<Output = R> + std::ops::Mul<Output = R> + PartialEq,
{
    type TargetModifier = Flat<A, R>;
    
    fn share(self) -> Self::TargetModifier {
        Flat::<A, R>::from_raw(self.value())
    }
}

impl<R> Shared<B> for Flat<SomeGroup, R>
where
    R: Copy + std::ops::Add<Output = R> + std::ops::Mul<Output = R> + PartialEq,
{
    type TargetModifier = Flat<B, R>;
    
    fn share(self) -> Self::TargetModifier {
        Flat::<B, R>::from_raw(self.value())
    }
}

#[test]
fn custom_group() {
    let mut a = Stat::<A>::with_base(1.);
    let mut b = Stat::<B>::with_base(3.);

    let modifier = Flat::<SomeGroup, f64>::from_raw(2.);

    a.apply_flat(Shared::<A>::share(modifier));

    b.apply_flat(Shared::<B>::share(modifier));

    a.apply_flat_shared(modifier);

    b.apply_flat_shared(modifier);
}
