#[macro_export]
macro_rules! match_all {
    ($value:expr, $( $pattern:pat_param )|+ $( if $guard:expr )? => $result:expr) => {
        match $value {
            $( $pattern )|+ $( if $guard )? => $result,
            _ => panic!("No pattern matched for: {:?}", $value),
        }
    };
}
