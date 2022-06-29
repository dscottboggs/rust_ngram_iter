# NGram Iterator

I wasn't pleased with any of the NGram iteration crates, so I implemented my
own. This makes use of [const generics](https://rust-lang.github.io/rfcs/2000-const-generics.html)
to determine the length of the NGram at compile-time, and supports iteration
over arbitrary types.

# Usage
Out of the box, `char`, `&str`, and any `Option` type are supported:

```rust
use ngram_iter as ngram;
use ngram::WORD_JOINER;
// char
let mut chars: ngram::Iter<char, _, 3> = "1234".chars().into();
assert_eq!(chars.next(), Some([WORD_JOINER, '1', '2']));
assert_eq!(chars.next(), Some(['2', '3', '4']));
assert_eq!(chars.next(), Some(['4', WORD_JOINER, WORD_JOINER]));
drop(chars);
// str
let values = vec!["one", "two"];
let mut iter: ngram::Iter<&str, _, 2> = values.iter().map(|s| *s).into();
//                 Vec<&str>::iter yields &&str, so deref ^^^^^^ to &str
assert_eq!(iter.next(), Some(["\u{2060}", "one"]));
// Option types
let values = vec![Some(1), Some(2)];
let mut iter: ngram::Iter<_, _, 2> = values.iter().map(|v| *v).into();
assert_eq!(iter.next(), Some([None, Some(1)]));
```

## Arbitrary types
Support for arbitrary `Copy` types are supported.

```rust
use ngram_iter as ngram;
#[derive(Debug, PartialEq, Clone, Copy)]
enum MyType {
    NoData,
    SomeData {
        x: i32,
        y: i32,
    },
}
use MyType::*;

impl ngram::Iterable for MyType {
    fn bumper_item() -> Self {
        NoData
    }
}

let values = vec![MyType::SomeData{x: 1, y: 2}, NoData, MyType::SomeData{x: 3, y: 4}, MyType::SomeData{x: 5, y: 6}];
let mut iter: ngram::Iter<_, _, 2> = values.iter().map(|it| *it).into();
assert_eq!(iter.next(), Some([NoData, MyType::SomeData{x: 1, y: 2}]));
assert_eq!(iter.next(), Some([MyType::SomeData{x: 1, y: 2}, NoData]));
assert_eq!(iter.next(), Some([NoData, MyType::SomeData{x: 3, y: 4}]));
assert_eq!(iter.next(), Some([MyType::SomeData{x: 3, y: 4}, MyType::SomeData{x: 5, y: 6}]));
assert_eq!(iter.next(), Some([MyType::SomeData{x: 5, y: 6}, NoData]));
```

# Contributing
This crate doesn't yet support *completely* arbitrary types. NGrams must be
composed of `Copy` types. Advice and/or implementation of an idiomatic way to
implement this for arbitrary `Clone` types is welcome.

# Bugs
None known. Please report any issues you encounter.

# Regarding the Use of Unsafe Code
This library makes use of unsafe **and** unstable elements of the Rust
language. If you're uncomfortable with the use of unsafe code, please seek an
alternative implementation, or contribute a safe implementation which exists
only behind a non-default feature. If you're unable to make use of unstable
features, please be patient until these features are stabilized, recommend a
way to implement the relevant code in stable rust without a performance impact
(or contribute an implementation with a performance impact, but behind a non-
default feature-flag), or seek an alternative implementation.

# Authors
- [D. Scott Boggs](https://github.com/dscottboggs): Creator and maintainer

# License
This library is LGPLv3.