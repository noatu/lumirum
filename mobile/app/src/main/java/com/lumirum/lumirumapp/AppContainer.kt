package com.lumirum.lumirumapp

import android.content.Context
import androidx.compose.runtime.staticCompositionLocalOf
import com.lumirum.lumirumapp.data.api.ApiClient
import com.lumirum.lumirumapp.data.api.dto.Role
import com.lumirum.lumirumapp.data.local.AppDataStore
import com.lumirum.lumirumapp.data.repository.AuthRepository
import com.lumirum.lumirumapp.data.repository.DeviceRepository
import com.lumirum.lumirumapp.data.repository.ProfileRepository
import com.lumirum.lumirumapp.data.repository.TelemetryRepository
import com.lumirum.lumirumapp.data.repository.UserRepository
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

class AppContainer(context: Context) {

    val dataStore = AppDataStore(context)

    var token: String? = null

    private val _userRole = MutableStateFlow<Role?>(null)
    val userRole: StateFlow<Role?> = _userRole.asStateFlow()

    fun setUserRole(role: Role?) {
        _userRole.value = role
    }

    private val _unauthorizedEvent = MutableSharedFlow<Unit>(extraBufferCapacity = 1)
    val unauthorizedEvent: SharedFlow<Unit> = _unauthorizedEvent

    private val apiClient = ApiClient(
        baseUrl = AppDataStore.DEFAULT_BASE_URL,
        tokenProvider = { token },
        onUnauthorized = { _unauthorizedEvent.tryEmit(Unit) },
    )

    val authRepository = AuthRepository(apiClient.api, dataStore)
    val deviceRepository = DeviceRepository(apiClient.api)
    val telemetryRepository = TelemetryRepository(apiClient.api)
    val profileRepository = ProfileRepository(apiClient.api)
    val userRepository = UserRepository(apiClient.api)

    suspend fun initialize() {
        token = dataStore.getToken()
    }

    suspend fun logout() {
        token = null
        _userRole.value = null
        authRepository.clearSession()
    }
}

val LocalAppContainer = staticCompositionLocalOf<AppContainer> {
    error("AppContainer not provided")
}
