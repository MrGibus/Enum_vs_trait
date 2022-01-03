use enum_vs_trait_lib::{quxit_enum, quxit_trait, quxit_generic, quxit_genenum};

fn main() {
    let xs = [3, 10, -2, -3];

    let qux_traits = quxit_trait(&xs, 10, 3);

    let qux_matches = quxit_enum(&xs, 10, 3);

    let qux_generic = quxit_generic::<i32>(&xs, 10, 3);

    let qux_genenum = quxit_genenum(&xs, 10, 3);

    assert_eq!(qux_traits, qux_matches);
    assert_eq!(qux_traits, qux_generic);
    assert_eq!(qux_traits, qux_genenum);

    println! {"{:?} vs {:?} vs {:?} vs {:?}", qux_traits, qux_matches, qux_generic, qux_genenum}
}
