package com.lumirum.lumirumapp.data.api

import com.lumirum.lumirumapp.data.api.dto.*
import okhttp3.ResponseBody
import retrofit2.Response
import retrofit2.http.*

interface LumiRumApi {

    @POST("auth/login")
    suspend fun login(@Body request: LoginRequest): Response<AuthResponse>

    @GET("auth/me")
    suspend fun getCurrentUser(): Response<AuthResponse>

    @PATCH("auth/me")
    suspend fun updateAccount(@Body request: ChangeAccountRequest): Response<AuthResponse>

    @HTTP(method = "DELETE", path = "auth/me", hasBody = true)
    suspend fun deleteAccount(@Body request: DeleteAccountRequest): Response<ResponseBody>

    @POST("auth/register")
    suspend fun register(@Body request: RegisterRequest): Response<AuthResponse>

    @GET("devices")
    suspend fun getDevices(): Response<List<Device>>

    @GET("devices/{id}")
    suspend fun getDevice(@Path("id") id: Long): Response<Device>

    @POST("devices")
    suspend fun createDevice(@Body request: CreateDeviceRequest): Response<Device>

    @PUT("devices/{id}")
    suspend fun updateDevice(@Path("id") id: Long, @Body request: CreateDeviceRequest): Response<Device>

    @DELETE("devices/{id}")
    suspend fun deleteDevice(@Path("id") id: Long): Response<ResponseBody>

    @POST("devices/{id}/key")
    suspend fun regenerateKey(@Path("id") id: Long): Response<Device>

    @GET("profiles")
    suspend fun getProfiles(): Response<List<Profile>>

    @GET("profiles/{id}")
    suspend fun getProfile(@Path("id") id: Long): Response<Profile>

    @POST("profiles")
    suspend fun createProfile(@Body request: CreateProfileRequest): Response<Profile>

    @PUT("profiles/{id}")
    suspend fun updateProfile(@Path("id") id: Long, @Body request: CreateProfileRequest): Response<Profile>

    @DELETE("profiles/{id}")
    suspend fun deleteProfile(@Path("id") id: Long): Response<ResponseBody>

    @GET("profiles/circadian/{id}")
    suspend fun getProfileSchedule(@Path("id") id: Long): Response<LightingSchedule>

    @GET("telemetry/device/{deviceId}")
    suspend fun getTelemetryForDevice(
        @Path("deviceId") deviceId: Long,
        @Query("start") start: String,
        @Query("end") end: String,
    ): Response<List<Telemetry>>

    @DELETE("telemetry/device/{deviceId}")
    suspend fun deleteTelemetry(
        @Path("deviceId") deviceId: Long,
        @Query("start") start: String,
        @Query("end") end: String,
    ): Response<ResponseBody>

    @GET("health")
    suspend fun getHealth(): Response<HealthResponse>

    @GET("stats")
    suspend fun getStats(): Response<Stats>

    @GET("users")
    suspend fun getUsers(): Response<List<User>>

    @DELETE("users/{id}")
    suspend fun deleteUser(@Path("id") id: Long): Response<ResponseBody>
}
