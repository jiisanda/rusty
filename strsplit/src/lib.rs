#![warn(rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

//  let x: StrSplit;
// for part in x {
// }                    -> this is what iterator allows us to do
impl<'haystack, D> Iterator for StrSplit<'haystack, D> where D: Delimiter {          // lifetime illusion: impl<'haystack> Iterator for StrSplit<'haystack, '_>
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))       // here s.find(self): will give start of a string in that string
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char(s: &str, c: char) -> &'_ str {                               // until_char(s: &str, c: char) -> &str {
    // let delim = format!("{}", c);           // this does heap allocation
    StrSplit::new(s, c)
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
