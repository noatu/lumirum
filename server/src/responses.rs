#![allow(unused)]
use crate::features::{
    auth::AuthResponse,
    profiles::Profile,
};
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
        /// Internal server error
        #[response(status = INTERNAL_SERVER_ERROR)]
        InternalServerError(ErrorResponse),
    }
    #[derive(IntoResponses)]
    BadRequest {
        /// Invalid json structure or invalid request
        #[response(status = BAD_REQUEST)]
        BadRequest(ErrorResponse),
    }
    #[derive(IntoResponses)]
    UnprocessableEntity {
        /// Data validation failed
        #[response(status = UNPROCESSABLE_ENTITY)]
        UnprocessableEntity(ErrorResponse),
    }
    #[derive(IntoResponses)]
    Unauthorized {
        /// Credentials are invalid
        #[response(status = UNAUTHORIZED)]
        Unauthorized(ErrorResponse),
    }

    // EXTRACTORS

    #[derive(IntoResponses)]
    Validation := BadRequest || UnprocessableEntity


    // AUTH

    #[derive(IntoResponses)]
    UsernameTaken {
        /// Username is taken
        #[response(status = CONFLICT)]
        UsernameTaken(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Register := Validation || InternalServerError || UsernameTaken || {
        /// User registered successfully
        #[response(status = CREATED)]
        UserCreated(AuthResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Login := Validation || InternalServerError || Unauthorized || {
        /// Login successful
        #[response(status = OK)]
        Success(AuthResponse),

    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetMe := InternalServerError || Unauthorized || {
        /// Login successful
        #[response(status = OK)]
        Success(AuthResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PatchMe := Validation || InternalServerError || Unauthorized || UsernameTaken || {
        /// Account updated successfully
        #[response(status = OK)]
        Success(AuthResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    DeleteMe := Validation || InternalServerError || Unauthorized || {
        /// Account deleted successfully
        #[response(status = NO_CONTENT)]
        Success,
        /// Cannot delete last administrator account
        #[response(status = CONFLICT)]
        LastAdmin(ErrorResponse),
    }


    // PROFILES

    #[derive(IntoResponses)]
    ProfileNameTaken {
        /// Profile name is taken
        #[response(status = CONFLICT)]
        ProfileNameTaken(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfile := InternalServerError || Unauthorized || {
        /// Get profile information
        #[response(status = OK)]
        Success(AuthResponse),
        /// Profile does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfiles := InternalServerError || Unauthorized || {
        /// Get all profiles information
        #[response(status = OK)]
        Success(AuthResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PostProfile := Validation || InternalServerError || Unauthorized || ProfileNameTaken || {
        /// Profile created successfully
        #[response(status = CREATED)]
        Success(Profile),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PutProfile := Validation || InternalServerError || Unauthorized || ProfileNameTaken || {
        /// Profile updated successfully
        #[response(status = OK)]
        Success(Profile),
        /// Cannot update a parent profile
        #[response(status = FORBIDDEN)]
        CantParentProfile(ErrorResponse)

    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    DeleteProfile := Validation || InternalServerError || Unauthorized || {
        /// Profile deleted successfully
        #[response(status = NO_CONTENT)]
        Success,
        /// Cannot delete a parent profile
        #[response(status = FORBIDDEN)]
        CantParentProfile(ErrorResponse)
    }

}
