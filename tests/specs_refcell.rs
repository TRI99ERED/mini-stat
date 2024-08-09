use mini_stat::{prelude::*, refcell::MiniStat};

#[derive(Debug, Default)]
struct Dummy;

impl StatMarker for Dummy {
    type Raw = f64;

    type Metadata = &'static str;
}

#[test]
fn specs() {
    let stat = MiniStat::<Dummy>::with_base(3.);
    println!("{:.0}", stat.cached());

    stat.stat_mut()
        .apply_flat(Flat::from_raw(1.))
        .apply_flat(Flat::from_raw(2.))
        .apply_flat(Flat::from_raw(3.));
    println!("{:.0}", stat.cached());
    println!("{stat:#?}");

    assert_eq!(stat.cached().round(), 9.);

    let stat = MiniStat::<Dummy>::with_base(6.);
    println!("{:.0}", stat.cached());

    stat.stat_mut()
        .apply_add(Additive::from_raw(0.1))
        .apply_add(Additive::from_raw(0.1))
        .apply_add(Additive::from_raw(0.3));
    println!("{:.0}", stat.cached());
    println!("{stat:#?}");

    assert_eq!(stat.cached().round(), 9.);

    let stat = MiniStat::<Dummy>::with_base(1.);
    println!("{:.0}", stat.cached());

    stat.stat_mut()
        .apply_mul(Multiplicative::from_raw(0.5))
        .apply_mul(Multiplicative::from_raw(3.))
        .apply_mul(Multiplicative::from_raw(6.))
        .cache_value();
    println!("{:.0}", stat.cached());
    println!("{stat:#?}");

    assert_eq!(stat.cached().round(), 9.);

    let stat = MiniStat::<Dummy>::with_base(0.);
    println!("{:.0}", stat.cached());

    stat.stat_mut()
        .apply_flat(Flat::from_raw(1.))
        .apply_add(Additive::from_raw(0.5))
        .apply_mul(Multiplicative::from_raw(6.))
        .cache_value();
    println!("{:.0}", stat.cached());
    println!("{stat:#?}");

    assert_eq!(stat.cached().round(), 9.);

    stat.remove_add(Additive::from_raw(1.));
    println!("{:.0}", stat.cached());
    println!("{stat:#?}");
    assert_eq!(stat.cached().round(), 9.);

    stat.remove_add(Additive::from_raw(0.5));
    println!("{:.0}", stat.cached());
    println!("{stat:#?}");
}
