#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new()
    };       // these are not argument but pattern
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = vecmac![];
    assert!(x.is_empty());
}