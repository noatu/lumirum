use axum::{
    Json,
    extract::{
        FromRequest,
        Request,
    },
};
use garde::{
    Unvalidated,
    Valid,
    Validate,
};
use serde::de::DeserializeOwned;

use crate::errors::Error;

pub struct Validated<T>(pub Valid<T>);

impl<S, T> FromRequest<S> for Validated<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    <T as Validate>::Context: Default,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await?;

        Ok(Self(Unvalidated::new(data).validate()?))
    }
}
