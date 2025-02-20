use std::ops::Range;
use std::path::Path;

pub fn ex1() {
    // let o = OsStr::assert_from_raw_bytes()
    let p = Path::new("arst/arst");
    let pb = p.to_path_buf();
    let _p2 = pb.as_path();
}

pub fn ex2<T, K, M>(t: T) -> M
where
    K: From<T>,
    M: From<K>,
{
    M::from(K::from(t))
}

pub fn ex3(_r: Range<u32>) {}
