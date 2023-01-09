use crate::file::File;
use std::cmp::Ordering;

// -t
pub fn time(f1: &&File, f2: &&File) -> Ordering {
    f1.modified().cmp(&f2.modified())
}

// -S
pub fn size(f1: &&File, f2: &&File) -> Ordering {
    f1.size().cmp(&f2.size())
}

// -S
pub fn no_order(_: &&File, _: &&File) -> Ordering {
    Ordering::Equal
}
