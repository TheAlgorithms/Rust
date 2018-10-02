use std::iter::Peekable;
use std::cmp::Ordering;
struct MergeAscending<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
{
    left: Peekable<L>,
    right: Peekable<R>,
}
impl<L, R> MergeAscending<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
{
    fn new(left: L, right: R) -> Self {
        MergeAscending {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}
impl<L, R> Iterator for MergeAscending<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
    L::Item: Ord,
{
    type Item = L::Item;
    fn next(&mut self) -> Option<L::Item> {
        let which = match (self.left.peek(), self.right.peek()) {
            (Some(l), Some(r)) => Some(l.cmp(r)),
            (Some(_), None) => Some(Ordering::Less),
            (None, Some(_)) => Some(Ordering::Greater),
            (None, None) => None,
        };
        match which {
            Some(Ordering::Less) => self.left.next(),
            Some(Ordering::Equal) => self.left.next(),
            Some(Ordering::Greater) => self.right.next(),
            None => None,
        }
    }
}
fn main() {
    let left = [1, 3, 5, 7, 9];
    let right = [3, 4, 5, 6, 7];
    let result: Vec<_> = MergeAscending::new(left.iter(), right.iter()).collect();
    println!("{:?}", result);
}
