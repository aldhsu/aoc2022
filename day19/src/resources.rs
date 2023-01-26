use derive_more::Add;
use derive_more::AddAssign;
use derive_more::Div;
use derive_more::Mul;
use derive_more::Sub;
use derive_more::SubAssign;

#[derive(
    Debug,
    Eq,
    PartialEq,
    Default,
    Copy,
    Clone,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Ord,
    PartialOrd,
    Mul,
    Div,
    Hash,
)]
pub struct Ore(pub usize);

#[derive(
    Debug,
    Eq,
    PartialEq,
    Default,
    Copy,
    Clone,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Ord,
    PartialOrd,
    Mul,
    Div,
    Hash,
)]
pub struct Clay(pub usize);

#[derive(
    Debug,
    Eq,
    PartialEq,
    Default,
    Copy,
    Clone,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Ord,
    PartialOrd,
    Mul,
    Div,
    Hash,
)]
pub struct Obsidian(pub usize);
