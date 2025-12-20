#![allow(unused)]
use crate::features::auth::AuthResponse;
use error_set::error_set;
use utoipa::{
    IntoResponses,
    ToSchema,
};

error_set! {
    #[derive(ToSchema)]
    struct ErrorResponse {
        error: ErrorResponseInner
    }
    #[derive(ToSchema)]
    struct ErrorResponseInner {
        code: String,
        message: String,
    }

    #[derive(IntoResponses)]
    InternalServerError {
        /// Internal Server Error
        #[response(status = INTERNAL_SERVER_ERROR)]
        InternalServerError(ErrorResponse),
    }
    #[derive(IntoResponses)]
    BadRequest {
        /// Invalid JSON Structure or Invalid Request
        #[response(status = BAD_REQUEST)]
        BadRequest(ErrorResponse),
    }
    #[derive(IntoResponses)]
    UnprocessableEntity {
        /// Data Validation Failed
        #[response(status = UNPROCESSABLE_ENTITY)]
        UnprocessableEntity(ErrorResponse),
    }
    #[derive(IntoResponses)]
    Unauthorized {
        /// Credentials are Invalid
        #[response(status = UNAUTHORIZED)]
        Unauthorized(ErrorResponse),
    }

    // EXTRACTORS

    #[derive(IntoResponses)]
    Validation := BadRequest || UnprocessableEntity

    // AUTH

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Register := Validation || InternalServerError || {
        /// User Registered Successfully
        #[response(status = CREATED)]
        UserCreated(AuthResponse),
        /// Username is Taken
        #[response(status = CONFLICT)]
        UsernameTaken(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Login := Validation || InternalServerError || Unauthorized || {
        /// Login Successful
        #[response(status = OK)]
        Success(AuthResponse),

    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    ChangePassword := Validation || InternalServerError || Unauthorized || {
        /// Password Changed Successfully
        #[response(status = OK)]
        Success(AuthResponse),

    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetMe := InternalServerError || Unauthorized || {
        /// Login Successful
        #[response(status = OK)]
        AuthorizationSuccess(AuthResponse),
    }


    // OTHER
}
