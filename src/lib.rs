use fnv::FnvHashMap;
use itertools::Itertools;
use std::{fmt::Display, hash::Hash};

pub mod dot;

type VersionSpacePointer = usize;

pub trait Exp: PartialEq + Clone + Hash + Eq + Display {}

impl Exp for i64 {}
impl Exp for bool {}

pub trait Opt<E>: Display + Hash + Eq + PartialEq + Clone {
    fn construct_exp(&self, _: &[E]) -> E;
}

pub struct VersionTable<O: Opt<E>, E: Exp> {
    count: VersionSpacePointer,
    map: FnvHashMap<VersionSpace<O, E>, VersionSpacePointer>,
    space: Vec<VersionSpace<O, E>>,
}

impl<O: Opt<E>, E: Exp> VersionTable<O, E> {
    pub fn new() -> Self {
        Self {
            count: 0,
            map: FnvHashMap::default(),
            space: Vec::new(),
        }
    }
    pub fn get(&self, ptr: VersionSpacePointer) -> Option<&VersionSpace<O, E>> {
        self.space.get(ptr)
    }
    pub fn add(&mut self, v: VersionSpace<O, E>) -> VersionSpacePointer {
        *self.map.entry(v.clone()).or_insert_with(|| {
            let ptr = self.count;
            self.count += 1;
            self.space.push(v);
            ptr
        })
    }
}

impl<O: Opt<E>, E: Exp> Default for VersionTable<O, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum VersionSpace<O: Opt<E>, E: PartialEq + Clone + Hash + Eq> {
    Empty,
    VS(E),
    Union(Vec<VersionSpacePointer>), // Note that the order is important here when you might not expect it to be in a general Version Space. For uniqueness, you should order all of your elements.
    Join(O, Vec<VersionSpacePointer>),
}

impl<O: Opt<E>, E: Exp> Display for VersionSpace<O, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionSpace::Empty => write!(f, "Ø"),
            VersionSpace::VS(e) => write!(f, "{e}"),
            VersionSpace::Union(_) => write!(f, "U"),
            VersionSpace::Join(o, _) => write!(f, "{o}<sub>x</sub>"),
        }
    }
}

impl<O: Opt<E>, E: Exp> VersionSpace<O, E> {
    pub fn to_exprs(&self, table: &VersionTable<O, E>) -> Vec<E> {
        match self {
            VersionSpace::Empty => Vec::new(),
            VersionSpace::VS(e) => {
                vec![e.clone()]
            }
            VersionSpace::Union(v_set) => v_set
                .iter()
                .filter_map(|e| table.get(*e).map(|v| v.to_exprs(table)))
                .flatten()
                .collect(),
            VersionSpace::Join(o, v_list) if v_list.is_empty() => {
                vec![o.construct_exp(&Vec::new())]
            }
            VersionSpace::Join(o, v_list) => v_list
                .iter()
                .map(|p| {
                    table
                        .space
                        .get(*p)
                        .unwrap()
                        .to_exprs(table)
                        .into_iter()
                        .unique()
                })
                .multi_cartesian_product()
                .map(|e| o.construct_exp(&e))
                .collect(),
        }
    }
    pub fn intersection(&self, v: &Self, table: &mut VersionTable<O, E>) -> Self {
        match (self, v) {
            (VersionSpace::Empty, _) | (_, VersionSpace::Empty) => VersionSpace::Empty,
            (VersionSpace::VS(e1), VersionSpace::VS(e2)) => {
                if e1 == e2 {
                    VersionSpace::VS(e1.clone())
                } else {
                    VersionSpace::Empty
                }
            }
            (v1 @ VersionSpace::VS(_), VersionSpace::Union(v_list)) => {
                let new_spaces: Vec<_> = v_list
                    .iter()
                    .map(|v| table.get(*v).unwrap().clone().intersection(v1, table))
                    .filter(|v| v != &VersionSpace::Empty)
                    .collect();
                if new_spaces.is_empty() {
                    VersionSpace::Empty
                } else {
                    VersionSpace::Union(new_spaces.into_iter().map(|v| table.add(v)).collect())
                }
            }
            (VersionSpace::VS(_), VersionSpace::Join(_, _)) => todo!(),
            (VersionSpace::Union(_), VersionSpace::VS(_)) => todo!(),
            (VersionSpace::Union(_), VersionSpace::Union(_)) => todo!(),
            (VersionSpace::Union(_), VersionSpace::Join(_, _)) => todo!(),
            (VersionSpace::Join(_, _), VersionSpace::VS(_)) => todo!(),
            (VersionSpace::Join(_, _), VersionSpace::Union(_)) => todo!(),
            (VersionSpace::Join(_, _), VersionSpace::Join(_, _)) => todo!(),
        }
    }
}
