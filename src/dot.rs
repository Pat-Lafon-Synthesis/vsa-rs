use std::borrow::Cow;

use dot::{GraphWalk, Id, LabelText, Labeller};

use crate::{Exp, Opt, VersionSpace, VersionSpacePointer, VersionTable};

impl<'a, O: Opt<E>, E: Exp>
    GraphWalk<'a, VersionSpacePointer, (VersionSpacePointer, VersionSpacePointer)>
    for VersionTable<O, E>
{
    fn nodes(&'a self) -> dot::Nodes<'a, VersionSpacePointer> {
        Cow::Owned((0..self.count).collect())
    }

    fn edges(&'a self) -> dot::Edges<'a, (VersionSpacePointer, VersionSpacePointer)> {
        Cow::Owned(
            self.space
                .iter()
                .enumerate()
                .filter_map(|(i, v)| match v {
                    VersionSpace::Empty => None,
                    VersionSpace::VS(_) => None,
                    VersionSpace::Union(v_list) => {
                        Some(v_list.iter().map(|vs| (i, *vs)).collect::<Vec<_>>())
                    }
                    VersionSpace::Join(_, v_list) => {
                        Some(v_list.iter().map(|vs| (i, *vs)).collect())
                    }
                })
                .flatten()
                .collect(),
        )
    }

    fn source(&'a self, edge: &(VersionSpacePointer, VersionSpacePointer)) -> VersionSpacePointer {
        edge.0
    }

    fn target(&'a self, edge: &(VersionSpacePointer, VersionSpacePointer)) -> VersionSpacePointer {
        edge.1
    }
}

impl<'a, O: Opt<E>, E: Exp>
    Labeller<'a, VersionSpacePointer, (VersionSpacePointer, VersionSpacePointer)>
    for VersionTable<O, E>
{
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("g").unwrap()
    }

    fn node_id(&'a self, n: &VersionSpacePointer) -> Id<'a> {
        Id::new(format!("node{n}")).unwrap()
    }

    fn node_label(&'a self, n: &VersionSpacePointer) -> LabelText<'a> {
        let x = Cow::Owned(self.get(*n).unwrap().to_string());
        LabelText::HtmlStr(x)
    }
}
