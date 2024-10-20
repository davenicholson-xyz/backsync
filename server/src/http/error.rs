use axum::response::IntoResponse;
use hyper::StatusCode;

pub struct HttpError(anyhow::Error);
impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

        if let Some(reqwest_err) = self.0.downcast_ref::<reqwest::Error>() {
            if let Some(reqwest_status) = reqwest_err.status() {
                status_code = StatusCode::from_u16(reqwest_status.as_u16())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            } else {
                status_code = StatusCode::BAD_GATEWAY;
            }
        }

        (status_code, format!("Something went wrong: {:?}", self.0)).into_response()
    }
}

impl<E> From<E> for HttpError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
