use crate::io::{NamedNbtRef, UnnamedNbtRef};

pub struct NamedNbtRefIterator<'a>(NamedNbtRef<'a>);

pub struct UnnamedNbtRefIterator<'a>(UnnamedNbtRef<'a>);
