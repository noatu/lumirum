package com.lumirum.lumirumapp.data.repository

import com.lumirum.lumirumapp.data.api.LumiRumApi
import com.lumirum.lumirumapp.data.api.safeApiCall
import com.lumirum.lumirumapp.data.api.safeCall
import com.lumirum.lumirumapp.data.api.dto.Telemetry

class TelemetryRepository(private val api: LumiRumApi) {

    suspend fun getTelemetryForDevice(deviceId: Long, start: String, end: String): Result<List<Telemetry>> =
        safeApiCall { api.getTelemetryForDevice(deviceId, start, end) }

    suspend fun deleteTelemetry(deviceId: Long, start: String, end: String): Result<Unit> =
        safeCall { api.deleteTelemetry(deviceId, start, end) }
}
