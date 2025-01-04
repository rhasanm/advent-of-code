#[macro_export]
macro_rules! measure_time {
    ($block:expr) => {{
        let start = ::std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        result
    }};
}
