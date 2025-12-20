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
    enum InternalError {
        /// Internal Server Error
        #[response(status = INTERNAL_SERVER_ERROR)]
        InternalError(ErrorResponse),
    }

    #[derive(IntoResponses)]
    enum Validation {
        /// Invalid JSON Structure
        #[response(status = BAD_REQUEST)]
        InvalidJson(ErrorResponse),
        /// Data Validation Failed
        #[response(status = UNPROCESSABLE_ENTITY)]
        InvalidData(ErrorResponse),
    }

    #[derive(IntoResponses)]
    enum WrongCredentials {
        /// Credentials are Wrong
        #[response(status = UNAUTHORIZED)]
        WrongCredentials(ErrorResponse),
    }
    #[derive(IntoResponses)]
    enum Jwt {
        /// Credentials are Missing
        #[response(status = UNAUTHORIZED)]
        MissingCredentials(ErrorResponse),
        /// Token is Invalid
        #[response(status = UNAUTHORIZED)]
        InvalidToken(ErrorResponse),
        /// Token has Expired
        #[response(status = UNAUTHORIZED)]
        TokenExpired(ErrorResponse),
    }


    // AUTH

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Register := Validation || InternalError || {
        /// User Registered Successfully
        #[response(status = CREATED)]
        UserCreated(AuthResponse),
        /// Username is Taken
        #[response(status = CONFLICT)]
        UsernameTaken(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Login := Validation || InternalError ||  WrongCredentials || {
        /// Login Successful
        #[response(status = OK)]
        Success(AuthResponse),

    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    ChangePassword := Validation || InternalError ||  WrongCredentials ||  Jwt || {
        /// Password Changed Successfully
        #[response(status = OK)]
        Success(AuthResponse),

    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetMe := InternalError || Jwt || {
        /// Login Successful
        #[response(status = OK)]
        AuthorizationSuccess(AuthResponse),
    }


    // OTHER
}
