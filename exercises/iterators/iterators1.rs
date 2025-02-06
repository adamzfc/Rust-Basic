// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut fav_fruits_iterator = my_fav_fruits.iter();

    assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
    assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple"));
    assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
    assert_eq!(fav_fruits_iterator.next(), Some(&"peach"));
    assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
    assert_eq!(fav_fruits_iterator.next(), None);
}
