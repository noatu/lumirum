#![allow(unused)]
use crate::features::auth::AuthResponse;
use error_set::error_set;
use utoipa::IntoResponses;

error_set! {
    #[derive(IntoResponses)]
    enum InternalError {
        /// Internal Server Error
        #[response(status = INTERNAL_SERVER_ERROR)]
        InternalError,
    }

    #[derive(IntoResponses)]
    enum Validation {
        /// Invalid JSON Structure
        #[response(status = BAD_REQUEST)]
        InvalidJson,
        /// Data Validation Failed
        #[response(status = UNPROCESSABLE_ENTITY)]
        InvalidData,
    }

    #[derive(IntoResponses)]
    enum WrongCredentials {
        /// Credentials are Wrong
        #[response(status = UNAUTHORIZED)]
        WrongCredentials,
    }
    #[derive(IntoResponses)]
    enum Jwt {
        /// Credentials are Missing
        #[response(status = UNAUTHORIZED)]
        MissingCredentials,
        /// Token is Invalid
        #[response(status = UNAUTHORIZED)]
        InvalidToken,
        /// Token has Expired
        #[response(status = UNAUTHORIZED)]
        TokenExpired,
    }


    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Register := Validation || InternalError || {
        /// User Registered Successfully
        #[response(status = CREATED)]
        UserCreated(AuthResponse),
        /// Username is Taken
        #[response(status = CONFLICT)]
        UsernameTaken,
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Login := Validation || InternalError ||  WrongCredentials || {
        /// Login Successful
        #[response(status = OK)]
        AuthorizationSuccess(AuthResponse),

    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetMe := InternalError || Jwt || {
        /// Login Successful
        #[response(status = OK)]
        AuthorizationSuccess(AuthResponse),
    }
}
