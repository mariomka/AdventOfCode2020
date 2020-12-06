#[macro_export]
macro_rules! debug {
    ($($expression:expr),*) => {
        $(println!("{} = {:?}", stringify!($expression), $expression);)*
    };
}
