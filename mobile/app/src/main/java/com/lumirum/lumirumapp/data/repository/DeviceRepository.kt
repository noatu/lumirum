package com.lumirum.lumirumapp.data.repository

import com.lumirum.lumirumapp.data.api.LumiRumApi
import com.lumirum.lumirumapp.data.api.safeApiCall
import com.lumirum.lumirumapp.data.api.safeCall
import com.lumirum.lumirumapp.data.api.dto.CreateDeviceRequest
import com.lumirum.lumirumapp.data.api.dto.Device

class DeviceRepository(private val api: LumiRumApi) {

    suspend fun getDevices(): Result<List<Device>> =
        safeApiCall { api.getDevices() }

    suspend fun getDevice(id: Long): Result<Device> =
        safeApiCall { api.getDevice(id) }

    suspend fun createDevice(name: String, isPublic: Boolean, profileId: Long?): Result<Device> =
        safeApiCall { api.createDevice(CreateDeviceRequest(name, isPublic, profileId)) }

    suspend fun updateDevice(id: Long, name: String, isPublic: Boolean, profileId: Long?): Result<Device> =
        safeApiCall { api.updateDevice(id, CreateDeviceRequest(name, isPublic, profileId)) }

    suspend fun deleteDevice(id: Long): Result<Unit> =
        safeCall { api.deleteDevice(id) }

    suspend fun regenerateKey(id: Long): Result<Device> =
        safeApiCall { api.regenerateKey(id) }
}
