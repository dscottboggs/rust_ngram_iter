pub const WORD_JOINER: char = '\u{2060}';

/// A trait which implements `ngram_iter::Iterable` is able to be iterated over
/// as an NGram.
///
/// This crate implements `Iterable` for `char` and `&str`, where the bumper
/// item is the Unicode word-joiner character; and any `Option` type, where the
/// bumper item is `None`.
///
/// ```
/// use ngram_iter::Iterable;
/// assert_eq!(char::bumper_item(), '\u{2060}');
/// assert_eq!(<&str>::bumper_item(), "\u{2060}");
/// assert_eq!(Option::<u8>::bumper_item(), None);
/// ```
/// In case you need to iterate over n-grams of some custom type, you can
/// implement `Iterable` for those types:
/// ```
/// use std::result::Result;
/// use ngram_iter::Iterable;
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// enum MyError {
///     Bumper,
///     InvalidResponse,
/// }
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct ResponseData(Result<i32, MyError>);
///
/// impl Iterable for ResponseData {
///     fn bumper_item() -> Self {
///         ResponseData(Err(MyError::Bumper))
///     }
/// }
/// impl From<i32> for ResponseData {
///     fn from(n: i32) -> Self {
///         ResponseData(Ok(n))
///     }
/// }
/// let data = vec![1.into(), 2.into()];
/// let mut bigrams: ngram_iter::Iter<ResponseData, _, 2> = data.iter().map(|n| *n).into();
/// assert_eq!(bigrams.next(), Some([ResponseData(Err(MyError::Bumper)), ResponseData(Ok(1))]));
/// ```
pub trait Iterable {
    /// The value which is the first element of the first ngram and the last
    /// element of the last ngram.
    fn bumper_item() -> Self;
}

impl Iterable for char {
    fn bumper_item() -> char {
        WORD_JOINER
    }
}
impl Iterable for &str {
    fn bumper_item() -> Self {
        "\u{2060}"
    }
}

impl<T> Iterable for Option<T> {
    fn bumper_item() -> Option<T> {
        None
    }
}
