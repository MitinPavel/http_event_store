extern crate http_event_store;

macro_rules! assert_error {
    ($err:path, $kind:pat, $actual_error:expr) => ({
        assert!(
            match $actual_error {
                $err(k) => {
                      match k {
                          $kind => true,
                          _ => false
                      }
                },
                _ => false
        })

    })
}
