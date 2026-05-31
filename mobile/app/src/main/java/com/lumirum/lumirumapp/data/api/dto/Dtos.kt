package com.lumirum.lumirumapp.data.api.dto

import com.google.gson.JsonDeserializationContext
import com.google.gson.JsonDeserializer
import com.google.gson.JsonElement
import com.google.gson.annotations.SerializedName
import java.lang.reflect.Type

data class LoginRequest(val username: String, val password: String)

data class RegisterRequest(val username: String, val password: String)

data class ChangeAccountRequest(
    val password: String,
    val newUsername: String?,
    val newPassword: String?,
)

data class DeleteAccountRequest(val password: String)

data class AuthResponse(
    val id: Long,
    val username: String,
    val role: Role,
    val createdAt: String,
    val token: String,
)

data class User(
    val id: Long,
    val username: String,
    val role: Role,
    val createdAt: String,
)

sealed class Role {
    data object Admin : Role()
    data object Owner : Role()
    data class SubUser(val parentId: Long) : Role()

    fun label(): String = when (this) {
        is Admin -> "Admin"
        is Owner -> "Owner"
        is SubUser -> "User"
    }
}

class RoleDeserializer : JsonDeserializer<Role> {
    override fun deserialize(json: JsonElement, typeOfT: Type, context: JsonDeserializationContext): Role {
        return when {
            json.isJsonPrimitive && json.asString == "admin" -> Role.Admin
            json.isJsonPrimitive && json.asString == "owner" -> Role.Owner
            json.isJsonObject -> Role.SubUser(json.asJsonObject["user"].asLong)
            else -> Role.Owner
        }
    }
}

data class Device(
    val id: Long,
    val name: String,
    val secretKey: String,
    val ownerId: Long,
    val isPublic: Boolean,
    val createdAt: String,
    val profileId: Long?,
    val lastSeen: String?,
    val firmwareVersion: String?,
)

data class CreateDeviceRequest(
    val name: String,
    val isPublic: Boolean,
    val profileId: Long?,
)

data class Profile(
    val id: Long,
    val name: String,
    val ownerId: Long,
    val isShared: Boolean,
    val timezone: String,
    val sleepStart: String,
    val sleepEnd: String,
    val nightModeEnabled: Boolean,
    val minColorTemp: Int,
    val maxColorTemp: Int,
    val motionTimeoutSeconds: Int,
    val latitude: Double?,
    val longitude: Double?,
    val createdAt: String,
)

data class CreateProfileRequest(
    val name: String,
    val isShared: Boolean,
    val timezone: String,
    val sleepStart: String,
    val sleepEnd: String,
    val nightModeEnabled: Boolean,
    val minColorTemp: Int,
    val maxColorTemp: Int,
    val motionTimeoutSeconds: Int,
    val latitude: Double?,
    val longitude: Double?,
)

data class Telemetry(
    val id: Long,
    val deviceId: Long,
    val eventType: String,
    val createdAt: String,
    val brightness: Int?,
    val colorTemp: Int?,
    val ambientLight: Int?,
    val motionDetected: Boolean?,
    val lightIsOn: Boolean?,
)

data class Stats(
    val users: Long,
    val profiles: Long,
    val devices: Long,
    val telemetry: Long,
    val timestamp: String,
)

data class LightingSchedule(
    val profileId: Long,
    val sleepStartUtcSeconds: Int,
    val sleepEndUtcSeconds: Int,
    val minColorTemp: Int,
    val maxColorTemp: Int,
    val nightModeEnabled: Boolean,
    val motionTimeoutSeconds: Int,
    val generatedAt: String,
    val validUntil: String,
    val schedule: List<LightingPoint>,
)

data class LightingPoint(
    @SerializedName("utc") val timestamp: String,
    @SerializedName("temp") val colorTemp: Int,
)

data class HealthResponse(
    val status: String,
    val timestamp: String,
) {
    val isHealthy: Boolean get() = status == "healthy"
}

data class ApiError(val error: ApiErrorInner)

data class ApiErrorInner(val code: String, val message: String)
