package com.lumirum.lumirumapp.data.api

import com.google.gson.Gson
import com.google.gson.JsonObject
import retrofit2.Response

private val gson = Gson()

fun <T> Response<T>.apiErrorMessage(): String {
    return try {
        val httpCode = code()
        if (httpCode >= 500) return "Server error, please try again later (HTTP $httpCode)"

        val body = errorBody()?.string()
            ?: return "HTTP $httpCode: ${message()}"
        val json = gson.fromJson(body, JsonObject::class.java)
        val errorObj = json?.getAsJsonObject("error")
        val errorCode = errorObj?.get("code")?.asString
        val errorMessage = errorObj?.get("message")?.asString
            ?: return "HTTP $httpCode: ${message()}"

        when (errorCode) {
            "ProfileNotFound" -> "Profile not found — it may have been deleted or isn't accessible"
            "DeviceNotFound" -> "Device not found — it may have been deleted or isn't accessible"
            "UserNotFound" -> "Invalid credentials"
            "TelemetryNotFound" -> "No telemetry found"
            else -> errorMessage
        }
    } catch (e: Exception) {
        "HTTP ${code()}: ${message()}"
    }
}

suspend fun <T : Any> safeApiCall(block: suspend () -> Response<T>): Result<T> =
    runCatching {
        val response = block()
        if (response.isSuccessful) response.body()!! else error(response.apiErrorMessage())
    }

suspend fun safeCall(block: suspend () -> Response<*>): Result<Unit> =
    runCatching {
        val response = block()
        if (response.isSuccessful) Unit else error(response.apiErrorMessage())
    }
