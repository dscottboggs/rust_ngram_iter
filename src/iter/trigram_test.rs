use crate::{Iter, WORD_JOINER};

#[test]
fn empty_iter() {
    let mut subj: Iter<char, _, 3> = "".chars().into();
    assert!(subj.next().is_none());
}

#[test]
fn one_item() {
    let mut subj: Iter<char, _, 3> = "1".chars().into();
    assert_eq!(subj.next(), Some([WORD_JOINER, '1', WORD_JOINER]));
    assert!(subj.next().is_none());
}

#[test]
fn two_items() {
    let mut subj: Iter<char, _, 3> = "12".chars().into();
    assert_eq!(subj.next(), Some([WORD_JOINER, '1', '2']));
    // Should this 👇 be None instead?
    assert_eq!(subj.next(), Some(['2', WORD_JOINER, WORD_JOINER]));
    assert!(subj.next().is_none());
}

#[test]
fn three_items() {
    let mut subj: Iter<char, _, 3> = "123".chars().into();
    assert_eq!(subj.next(), Some([WORD_JOINER, '1', '2']));
    assert_eq!(subj.next(), Some(['2', '3', WORD_JOINER]));
    assert!(subj.next().is_none());
}

#[test]
fn four_items() {
    let mut subj: Iter<char, _, 3> = "1234".chars().into();
    assert_eq!(subj.next(), Some([WORD_JOINER, '1', '2']));
    assert_eq!(subj.next(), Some(['2', '3', '4']));
    assert_eq!(subj.next(), Some(['4', WORD_JOINER, WORD_JOINER]));
    assert!(subj.next().is_none());
}

#[test]
fn five_items() {
    let mut subj: Iter<char, _, 3> = "12345".chars().into();
    assert_eq!(subj.next(), Some([WORD_JOINER, '1', '2']));
    assert_eq!(subj.next(), Some(['2', '3', '4']));
    assert_eq!(subj.next(), Some(['4', '5', WORD_JOINER]));
    assert!(subj.next().is_none());
}
