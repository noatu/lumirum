package com.lumirum.lumirumapp.data.repository

import com.lumirum.lumirumapp.data.api.LumiRumApi
import com.lumirum.lumirumapp.data.api.safeApiCall
import com.lumirum.lumirumapp.data.api.safeCall
import com.lumirum.lumirumapp.data.api.dto.CreateProfileRequest
import com.lumirum.lumirumapp.data.api.dto.LightingSchedule
import com.lumirum.lumirumapp.data.api.dto.Profile

class ProfileRepository(private val api: LumiRumApi) {

    suspend fun getProfiles(): Result<List<Profile>> =
        safeApiCall { api.getProfiles() }

    suspend fun getProfile(id: Long): Result<Profile> =
        safeApiCall { api.getProfile(id) }

    suspend fun createProfile(request: CreateProfileRequest): Result<Profile> =
        safeApiCall { api.createProfile(request) }

    suspend fun updateProfile(id: Long, request: CreateProfileRequest): Result<Profile> =
        safeApiCall { api.updateProfile(id, request) }

    suspend fun deleteProfile(id: Long): Result<Unit> =
        safeCall { api.deleteProfile(id) }

    suspend fun getSchedule(id: Long): Result<LightingSchedule> =
        safeApiCall { api.getProfileSchedule(id) }
}
