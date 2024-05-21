#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new()
    };       // these are not argument but pattern
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($llement);
        vs
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = vecmac![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = vecmac![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}