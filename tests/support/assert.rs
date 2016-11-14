extern crate http_event_store;

macro_rules! assert_error {
    ($err:pat, $actual_error:expr) => ({
        assert!(
            match $actual_error {
                $err => true,
                _ => false
        })

    })
}
