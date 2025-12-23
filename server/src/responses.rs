#![allow(unused)]
use crate::features::{
    auth::{
        AuthResponse,
        User,
    },
    circadian::LightingSchedule,
    devices::Device,
    profiles::Profile,
    system::Stats,
    telemetry::Telemetry,
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


    // SYSTEM

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    StatsResponse := InternalServerError || Unauthorized || {
        /// Stats retrieved successfully
        #[response(status = OK)]
        Success(Stats),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetUser := InternalServerError || Unauthorized || {
        /// Got user information successfully
        #[response(status = OK)]
        Success(User),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetUsers := InternalServerError || Unauthorized || {
        /// Got users successfully
        #[response(status = OK)]
        Success(Vec<User>),
    }


    // ME

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
        /// Username not found
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
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
        Admin(ErrorResponse),
    }


    // PROFILES

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfile := InternalServerError || Unauthorized || {
        /// Got profile information successfully
        #[response(status = OK)]
        Success(Profile),
        /// Profile does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfiles := InternalServerError || Unauthorized || {
        /// Got all profiles information successfully
        #[response(status = OK)]
        Success(Vec<Profile>),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetProfileSchedule := InternalServerError || Unauthorized || {
        /// Got profile lighting schedule successfully
        #[response(status = OK)]
        Success(LightingSchedule),
        /// Profile does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
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
        /// Got device information successfully
        #[response(status = OK)]
        Success(Device),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetDevices := InternalServerError || Unauthorized || {
        /// Got all devices successfully
        #[response(status = OK)]
        Success(Vec<Device>),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetDeviceSchedule := InternalServerError || Unauthorized || {
        /// Got device lighting schedule successfully
        #[response(status = OK)]
        Success(Option<LightingSchedule>),
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
        // /// Cannot delete this device
        // #[response(status = FORBIDDEN)]
        // Forbidden(ErrorResponse),
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


    // TELEMETRY

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetOneTelemetry := InternalServerError || Unauthorized || {
        /// Got telemetry information successfully
        #[response(status = OK)]
        Success(Telemetry),
        /// Telemetry does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetTelemetry := InternalServerError || Unauthorized || {
        /// Got telemetry information successfully
        #[response(status = OK)]
        Success(Vec<Telemetry>),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    GetDeviceTelemetry := InternalServerError || Unauthorized || {
        /// Got telemetry information successfully
        #[response(status = OK)]
        Success(Vec<Telemetry>),
    }

    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    PostTelemetry := ValidInternalAuth || {
        /// Telemetry created successfully
        #[response(status = CREATED)]
        Success(Telemetry),
        /// Telemetry name is taken
        #[response(status = CONFLICT)]
        TelemetryNameTaken(ErrorResponse),
    }
    #[derive(IntoResponses)]
    #[skip(Error,Display,Debug)]
    DeleteTelemetry := InternalServerError || Unauthorized || {
        /// Telemetry deleted successfully
        #[response(status = OK)]
        Success(u64),
        /// Telemetry does not exist
        #[response(status = NOT_FOUND)]
        NotFound(ErrorResponse),
    }
}
