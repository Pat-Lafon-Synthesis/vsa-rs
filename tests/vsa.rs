use vsa_rs::{Opt, VersionSpace, VersionTable};

#[test]
fn intersection_with_empty() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum Op {
        Neg,
        Plus,
    }
    impl Opt<i64> for Op {
        fn construct_exp(&self, args: &[i64]) -> i64 {
            match self {
                Op::Neg => -args[0],
                Op::Plus => args[0] + args[1],
            }
        }
    }

    let mut table = VersionTable::new();
    assert!(
        VersionSpace::Empty.intersection(&VersionSpace::Empty, &mut table) == VersionSpace::Empty
    );

    assert!(
        VersionSpace::Empty.intersection(&VersionSpace::VS(1), &mut table) == VersionSpace::Empty
    );

    assert!(
        VersionSpace::VS(1).intersection(&VersionSpace::Empty, &mut table) == VersionSpace::Empty
    );

    let one = table.add(VersionSpace::VS(1));

    assert!(
        VersionSpace::Empty.intersection(&VersionSpace::Union(vec![one]), &mut table)
            == VersionSpace::Empty
    );

    assert!(
        VersionSpace::Union(vec![one]).intersection(&VersionSpace::Empty, &mut table)
            == VersionSpace::Empty
    );

    assert!(
        VersionSpace::Empty.intersection(&VersionSpace::Join(Op::Neg, vec![one]), &mut table)
            == VersionSpace::Empty
    );
}
