extern crate http_event_store as hes;

macro_rules! assert_user_error {
    ($kind:pat, $actual_error:expr) => ({
        assert!(
            match $actual_error {
                hes::error::HesError::UserError(k) => {
                      match k {
                          $kind => true,
                          _ => false
                      }
                },
                _ => false
        })

    })
}
