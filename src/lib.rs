// convert map syntax into array of key value tuples
#[allow(unused_macros)]
macro_rules! kva {
    ($($key:ident : $value:expr)*) => (
        [$((stringify!($key), $value)),*]
    );
    ($($key:tt : $value:expr),*) => (
        [$(($key, $value)),*]
    );
}


/// create a HashSet from macro
#[macro_export]
macro_rules! hashset {
    ($($value:expr),*) => {
        std::collections::HashSet::from($($value)*)
    };
}


/// create a BTreeSet
#[macro_export]
macro_rules! btreeset {
    ($($value:expr),*) => {
        std::collections::BTreeSet::from($($value)*)
    };
}

/// create a auto set
#[macro_export]
macro_rules! set {
    ($($value:expr),*) => {
        [$($value)*].into()
    };
}

/// macro to create a hashmap
#[macro_export]
macro_rules! hashmap {
    ($($body:tt)*) => (
        std::collections::HashMap::from(kva!($($body)*))
    );
}

/// macro to create a hashmap

#[macro_export]
macro_rules! btreemap {
    ($($body:tt)*) => (
        std::collections::BTreeMap::from(kva!($($body)*))
    );
}

/// make a dict
/// supports standard json like format (with commas)
/// as well as yml like (without commas)
#[macro_export]
macro_rules! map {
    ($($body:tt)*) => (
        (kva!($($body)*)).into()
    );
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let a = 1;
        let b = 2;
        assert_eq!(
            hashmap![
                a: a+b
                b: b
            ],
            [("a", a + b), ("b", b)].into()
        );
        assert_eq!(hashmap![a: a, b: a + b], [(a, a), (b, a + b)].into());
    }
}
