// #![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

//  let x: StrSplit;
// for part in x {
// }                    -> this is what iterator allows us to do
impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {          // lifetime illusion: impl<'haystack> Iterator for StrSplit<'haystack, '_>
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &'_ str {                               // until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);           // this does heap allocation
    StrSplit::new(s, &delim)
        .next()
        .expect("StrSplit always gives at least one result.")
}

#[test]
fn it_works() {
    let haystack = "a b c d e";

    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}


#[test]
fn tail() {
    let haystack = "a b c d e ";

    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}
