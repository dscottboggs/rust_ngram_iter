#[cfg(test)]
mod bigram_test;
#[cfg(test)]
mod trigram_test;

use crate::{state::State, Iterable};
use std::mem::MaybeUninit;

/// An iterator over arbitrary-N-grams of arbitrary `Copy` types `T`.
///
/// `N` must be greater than or equal to 2, and this **is not** verified at
/// compile-time.
///
/// ```
/// use ngram_iter::Iterable; // adds the `bumper_item()` function to char.
/// let letters: String = ('a'..='z').collect();
/// let mut bigrams: ngram_iter::Iter<_, _, 2> = letters.chars().into();
/// assert_eq!(bigrams.next(), Some([char::bumper_item(), 'a']));
/// assert_eq!(bigrams.next(), Some(['a', 'b']));
/// let mut trigrams: ngram_iter::Iter<_, _, 3> = letters.chars().into();
/// assert_eq!(trigrams.next(), Some([char::bumper_item(), 'a', 'b']));
/// let mut ten_grams: ngram_iter::Iter<_, _, 10> = letters.chars().into();
/// assert_eq!(ten_grams.next(), Some([char::bumper_item(), 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']));
/// assert_eq!(ten_grams.next(), Some(['i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r']));
///
/// // N < 2 is panics at runtime!
/// let mut one_gram: ngram_iter::Iter<_, _, 1> = letters.chars().into();
/// std::panic::catch_unwind(move || one_gram.next()).expect_err("ngram with N<2 panics at runtime");
/// ```
pub struct Iter<T, I, const N: usize>
where
    T: Copy + Iterable,
    I: Iterator<Item = T>,
{
    it: I,
    state: State<T>,
}

impl<T, I, const N: usize> Iterator for Iter<T, I, N>
where
    T: Iterable + Copy,
    I: Iterator<Item = T>,
{
    type Item = [T; N];

    /// Returns a list of the next N items.
    ///
    /// panics if N < 2!
    fn next(&mut self) -> Option<Self::Item> {
        if N < 2 {
            panic!("ngram must have N of 2 or more")
        }
        let mut out: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
        match self.state {
            State::Start => {
                if let Some(item) = self.it.next() {
                    // dependent iterator has a least one item
                    self.state = item.into();
                    out[0] = MaybeUninit::new(T::bumper_item());
                    out[1] = MaybeUninit::new(item);
                } else {
                    // Iterating over empty iterator, skip to the end.
                    self.state = State::End;
                    return None;
                }
                // Fill in the remaining values if N is greater than 2.
                for i in 2..N {
                    if let Some(item) = self.it.next() {
                        self.state = item.into();
                        out[i] = MaybeUninit::new(item);
                    } else {
                        self.state = State::End;
                        // Fill in N-i values with the bumper/buffer item.
                        for j in i..N {
                            out[j] = MaybeUninit::new(T::bumper_item());
                        }
                    }
                }
            }
            State::Middle(item) => {
                // first value was stored in the state.
                out[0] = MaybeUninit::new(item);
                for i in 1..N {
                    if let Some(item) = self.it.next() {
                        // store current value in state for overlap.
                        self.state = State::Middle(item);
                        out[i] = MaybeUninit::new(item);
                    } else {
                        // End of iterator has been reached. Note the state...
                        self.state = State::End;
                        // ...and fill in the remaining values of this NGram
                        // with the bumper value.
                        for j in i..N {
                            out[j] = MaybeUninit::new(T::bumper_item());
                        }
                    }
                }
            }
            State::End => return None,
        }
        Some(unsafe { MaybeUninit::array_assume_init(out) })
    }
}

impl<T, I, const N: usize> From<I> for Iter<T, I, N>
where
    T: Copy + Iterable,
    I: Iterator<Item = T>,
{
    fn from(it: I) -> Self {
        Self {
            it,
            state: State::Start,
        }
    }
}
