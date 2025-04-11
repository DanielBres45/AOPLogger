#[macro_export]
macro_rules! flush {
    ($l:literal) => {
        log::debug!(flush = true; $l);
    };
}
