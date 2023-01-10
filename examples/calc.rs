use std::fmt::Display;

use dot::render;
use vsa_rs::{Opt, VersionSpace, VersionTable};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Calc {
    Plus,
    Minus,
    Multiply,
    Divide,
    Neg,
}

impl Display for Calc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calc::Plus => write!(f, "plus"),
            Calc::Minus => write!(f, "minus"),
            Calc::Multiply => write!(f, "mul"),
            Calc::Divide => write!(f, "div"),
            Calc::Neg => write!(f, "neg"),
        }
    }
}

impl Opt<i64> for Calc {
    fn construct_exp(&self, args: &[i64]) -> i64 {
        match self {
            Calc::Neg => -args[0],
            Calc::Plus => args[0] + args[1],
            Calc::Minus => args[0] - args[1],
            Calc::Multiply => args[0] * args[1],
            Calc::Divide => args[0] / args[1],
        }
    }
}

fn main() {
    let mut table = VersionTable::new();
    let one = table.add(VersionSpace::VS(1));
    let zero = table.add(VersionSpace::VS(0));
    let add_o_z = table.add(VersionSpace::Join(Calc::Plus, vec![one, zero]));
    let new_one = table.add(VersionSpace::Union(vec![add_o_z, one]));
    let zero = table.add(VersionSpace::VS(0));
    let mul_z = table.add(VersionSpace::Join(Calc::Multiply, vec![zero, new_one]));
    let new_zero = table.add(VersionSpace::Union(vec![zero, mul_z]));
    render(&table, &mut std::io::stdout()).unwrap();
}
