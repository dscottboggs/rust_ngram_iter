mod t_is_char {
    use crate::{Iter, WORD_JOINER};

    #[test]
    fn empty_iter() {
        let mut subj: Iter<char, _, 2> = "".chars().into();
        assert!(subj.next().is_none());
    }

    #[test]
    fn one_item() {
        let mut subj: Iter<char, _, 2> = "1".chars().into();
        assert_eq!(subj.next(), Some([WORD_JOINER, '1']));
        assert_eq!(subj.next(), Some(['1', WORD_JOINER]));
        assert!(subj.next().is_none());
    }

    #[test]
    fn two_items() {
        let mut subj: Iter<char, _, 2> = "12".chars().into();
        assert_eq!(subj.next(), Some([WORD_JOINER, '1']));
        assert_eq!(subj.next(), Some(['1', '2']));
        assert_eq!(subj.next(), Some(['2', WORD_JOINER]));
        assert_eq!(subj.next(), None);
    }

    #[test]
    fn four_items() {
        let mut subj: Iter<char, _, 2> = "1234".chars().into();
        assert_eq!(subj.next(), Some([WORD_JOINER, '1']));
        assert_eq!(subj.next(), Some(['1', '2']));
        assert_eq!(subj.next(), Some(['2', '3']));
        assert_eq!(subj.next(), Some(['3', '4']));
        assert_eq!(subj.next(), Some(['4', WORD_JOINER]));
        assert_eq!(subj.next(), None);
    }
}
mod t_is_f32 {
    use crate::{Iter, Iterable};

    impl Iterable for f32 {
        fn bumper_item() -> Self {
            f32::INFINITY
        }
    }

    #[test]
    fn two_items() {
        let vec = vec![1f32, 2f32];
        let mut subj: Iter<f32, _, 2> = vec.iter().map(|n| *n).into();
        assert_eq!(subj.next(), Some([f32::INFINITY, 1.0]));
        assert_eq!(subj.next(), Some([1.0, 2.0]));
        assert_eq!(subj.next(), Some([2.0, f32::INFINITY]));
        assert!(subj.next().is_none());
    }
}

mod t_is_f32_ref {
    use crate::{Iter, Iterable};

    impl Iterable for &f32 {
        fn bumper_item() -> Self {
            &f32::INFINITY
        }
    }

    #[test]
    fn two_items() {
        let vec = vec![1f32, 2f32];
        let mut subj: Iter<&f32, _, 2> = vec.iter().into();
        assert_eq!(subj.next(), Some([&f32::INFINITY, &1.0]));
        assert_eq!(subj.next(), Some([&1.0, &2.0]));
        assert_eq!(subj.next(), Some([&2.0, &f32::INFINITY]));
        assert!(subj.next().is_none());
    }
}
mod t_is_str_ref {
    use crate::{Iter, Iterable};

    #[test]
    fn two_items() {
        let vec: Vec<&str> = "one two".split(" ").collect();
        let mut subj: Iter<&str, _, 2> = vec.iter().map(|s| *s).into();
        let bumper = <&str>::bumper_item();
        assert_eq!(subj.next(), Some([bumper, "one"]));
        assert_eq!(subj.next(), Some(["one", "two"]));
        assert_eq!(subj.next(), Some(["two", bumper]));
        assert!(subj.next().is_none());
    }
}

mod t_is_option {
    use crate::Iter;

    #[test]
    fn two_items() {
        let vec = vec![Some(1), Some(2)];
        let mut subj: Iter<_, _, 2> = vec.iter().map(|it| *it).into();
        assert_eq!(subj.next(), Some([None, Some(1)]));
        assert_eq!(subj.next(), Some([Some(1), Some(2)]));
        assert_eq!(subj.next(), Some([Some(2), None]));
        assert!(subj.next().is_none());
    }
}

mod t_is_user_defined {
    use crate::{Iter, Iterable};

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum MyType {
        NoData,
        SomeData { x: i32, y: i32 },
    }
    use MyType::*;

    impl Iterable for MyType {
        fn bumper_item() -> Self {
            NoData
        }
    }

    #[test]
    fn test() {
        let values = vec![
            MyType::SomeData { x: 1, y: 2 },
            NoData,
            MyType::SomeData { x: 3, y: 4 },
            MyType::SomeData { x: 5, y: 6 },
        ];
        let mut iter: Iter<_, _, 2> = values.iter().map(|it| *it).into();
        assert_eq!(iter.next(), Some([NoData, MyType::SomeData { x: 1, y: 2 }]));
        assert_eq!(iter.next(), Some([MyType::SomeData { x: 1, y: 2 }, NoData]));
        assert_eq!(iter.next(), Some([NoData, MyType::SomeData { x: 3, y: 4 }]));
        assert_eq!(
            iter.next(),
            Some([
                MyType::SomeData { x: 3, y: 4 },
                MyType::SomeData { x: 5, y: 6 }
            ])
        );
        assert_eq!(iter.next(), Some([MyType::SomeData { x: 5, y: 6 }, NoData]));
    }
}
