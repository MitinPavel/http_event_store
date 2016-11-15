macro_rules! assert_error {
    ($err:pat, $actual_error:expr) => ({
        assert!(
            match $actual_error {
                $err => true,
                _ => false
        })

    })
}

macro_rules! assert_error_status_code {
    ($code:expr, $result:expr) => ({
        match $result.unwrap_err() {
            hes::error::ApiError::Restful(response) => assert_eq!(hyper::status::StatusCode::BadRequest, response.status),
            _ =>  assert!(false)
        }
    })
}
