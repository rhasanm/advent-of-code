#[macro_export]
macro_rules! pairs {
    ($iter:expr) => {{
        $iter.tuple_windows::<(_, _)>()
    }};
}
