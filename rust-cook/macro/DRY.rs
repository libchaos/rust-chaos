use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    ($a: ident, $b: ident, $func: iden, $op: tt) => (
      assert!($a.len()==$b.len(), "{:?}: dimensions mismatch: {:?} {:?} {:?}", stringify!($func),($a.len()));
    )
}