package com.lumirum.lumirumapp.data.repository

import com.lumirum.lumirumapp.data.api.LumiRumApi
import com.lumirum.lumirumapp.data.api.safeApiCall
import com.lumirum.lumirumapp.data.api.safeCall
import com.lumirum.lumirumapp.data.api.dto.*
import com.lumirum.lumirumapp.data.local.AppDataStore

class AuthRepository(
    private val api: LumiRumApi,
    private val dataStore: AppDataStore,
) {
    suspend fun login(username: String, password: String): Result<AuthResponse> =
        safeApiCall { api.login(LoginRequest(username, password)) }

    suspend fun register(username: String, password: String): Result<AuthResponse> =
        safeApiCall { api.register(RegisterRequest(username, password)) }

    suspend fun getCurrentUser(): Result<AuthResponse> =
        safeApiCall { api.getCurrentUser() }

    suspend fun updateAccount(
        currentPassword: String,
        newUsername: String?,
        newPassword: String?,
    ): Result<AuthResponse> =
        safeApiCall { api.updateAccount(ChangeAccountRequest(currentPassword, newUsername, newPassword)) }

    suspend fun deleteAccount(password: String): Result<Unit> =
        safeCall { api.deleteAccount(DeleteAccountRequest(password)) }

    suspend fun saveSession(token: String) = dataStore.saveToken(token)

    suspend fun clearSession() = dataStore.clearToken()
}
