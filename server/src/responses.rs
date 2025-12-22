#![allow(unused)]
use crate::features::{
    auth::AuthResponse,
    devices::Device,
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

    #[derive(IntoResponses)]
    ValidInternalAuth := Validation || InternalServerError || Unauthorized

    // AUTH

    #[derive(IntoResponses)]
    UsernameTaken {
        /// Username is taken
        #[response(status = CONFLICT)]
        UsernameTaken(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Register := ValidInternalAuth || UsernameTaken || {
        /// User registered successfully
        #[response(status = CREATED)]
        Created(AuthResponse),
        /// User role cannot create a user
        #[response(status = FORBIDDEN)]
        UserCantUser(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    Login := ValidInternalAuth || {
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
    PatchMe := ValidInternalAuth || UsernameTaken || {
        /// Account updated successfully
        #[response(status = OK)]
        Success(AuthResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    DeleteMe := ValidInternalAuth || {
        /// Account deleted successfully
        #[response(status = NO_CONTENT)]
        Success,
        /// Cannot delete administrator account
        #[response(status = CONFLICT)]
        LastAdmin(ErrorResponse),
    }


    // PROFILES

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfile := InternalServerError || Unauthorized || {
        /// Get profile information
        #[response(status = OK)]
        Success(Profile),
        /// Profile does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfiles := InternalServerError || Unauthorized || {
        /// Get all profiles information
        #[response(status = OK)]
        Success(Vec<Profile>),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PostProfile := ValidInternalAuth || {
        /// Profile created successfully
        #[response(status = CREATED)]
        Created(Profile),
        /// Profile name is taken
        #[response(status = CONFLICT)]
        ProfileNameTaken(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PutProfile := ValidInternalAuth || {
        /// Profile updated successfully
        #[response(status = OK)]
        Success(Profile),
        /// Cannot set others' profile private
        #[response(status = FORBIDDEN)]
        CantProfilePrivate(ErrorResponse),
        /// Profile name is taken
        #[response(status = CONFLICT)]
        ProfileNameTaken(ErrorResponse),
        /// Profile not found
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    DeleteProfile := ValidInternalAuth || {
        /// Profile deleted successfully
        #[response(status = NO_CONTENT)]
        Success,
        /// Cannot delete a parent profile
        #[response(status = FORBIDDEN)]
        CantParentProfile(ErrorResponse),
        /// Profile not found
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }


    // DEVICES

    #[derive(IntoResponses)]
    DeviceNotFound {
        /// Device does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetDevice := InternalServerError || Unauthorized || DeviceNotFound || {
        /// Get device information
        #[response(status = OK)]
        Success(Device),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetDevices := InternalServerError || Unauthorized || {
        /// Get all devices
        #[response(status = OK)]
        Success(Vec<Device>),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PostDevice := ValidInternalAuth || {
        /// Device created successfully
        #[response(status = CREATED)]
        Success(Device),
        /// Device name is taken
        #[response(status = CONFLICT)]
        DeviceNameTaken(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PutDevice := ValidInternalAuth || DeviceNotFound || {
        /// Device updated successfully
        #[response(status = OK)]
        Success(Device),
        /// Device name is taken
        #[response(status = CONFLICT)]
        DeviceNameTaken(ErrorResponse),
        /// Cannot set others' device private
        #[response(status = FORBIDDEN)]
        CantDevicePrivate(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    DeleteDevice := InternalServerError || Unauthorized || DeviceNotFound || {
        /// Device deleted successfully
        #[response(status = NO_CONTENT)]
        Success,
        /// Cannot delete this device
        #[response(status = FORBIDDEN)]
        Forbidden(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    RegenerateDeviceKey := InternalServerError || Unauthorized || DeviceNotFound || {
        /// Key regenerated successfully returns the updated device
        #[response(status = OK)]
        Success(Device),
        /// Users cannot regenerate an Owner's device key
        #[response(status = FORBIDDEN)]
        Forbidden(ErrorResponse),
    }
}
