package com.lumirum.lumirumapp.data.repository

import com.lumirum.lumirumapp.data.api.LumiRumApi
import com.lumirum.lumirumapp.data.api.safeApiCall
import com.lumirum.lumirumapp.data.api.safeCall
import com.lumirum.lumirumapp.data.api.dto.HealthResponse
import com.lumirum.lumirumapp.data.api.dto.Stats
import com.lumirum.lumirumapp.data.api.dto.User

class UserRepository(private val api: LumiRumApi) {

    suspend fun getUsers(): Result<List<User>> =
        safeApiCall { api.getUsers() }

    suspend fun deleteUser(id: Long): Result<Unit> =
        safeCall { api.deleteUser(id) }

    suspend fun getStats(): Result<Stats> =
        safeApiCall { api.getStats() }

    suspend fun getHealth(): Result<HealthResponse> =
        safeApiCall { api.getHealth() }
}
