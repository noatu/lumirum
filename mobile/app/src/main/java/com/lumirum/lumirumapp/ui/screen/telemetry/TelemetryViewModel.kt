package com.lumirum.lumirumapp.ui.screen.telemetry

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.Telemetry
import com.lumirum.lumirumapp.data.repository.TelemetryRepository
import com.lumirum.lumirumapp.ui.components.UiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import java.time.Instant
import java.time.ZoneOffset
import java.time.format.DateTimeFormatter

data class TelemetryUiState(
    val data: UiState<List<Telemetry>> = UiState.Loading,
    val startTime: Instant = Instant.now().minusSeconds(7 * 24 * 3600),
    val endTime: Instant = Instant.now(),
    val isDeleting: Boolean = false,
    val deleteError: String? = null,
)

class TelemetryViewModel(
    private val deviceId: Long,
    private val repository: TelemetryRepository,
) : ViewModel() {

    private val formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss'Z'").withZone(ZoneOffset.UTC)

    private val _uiState = MutableStateFlow(TelemetryUiState())
    val uiState: StateFlow<TelemetryUiState> = _uiState.asStateFlow()

    init {
        load()
    }

    fun load() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(data = UiState.Loading)
            val s = _uiState.value
            _uiState.value = s.copy(
                data = repository.getTelemetryForDevice(
                    deviceId = deviceId,
                    start = formatter.format(s.startTime),
                    end = formatter.format(s.endTime),
                ).fold(
                    onSuccess = { UiState.Success(it) },
                    onFailure = { UiState.Error(it.message ?: "Unknown error") },
                )
            )
        }
    }

    fun setRange(start: Instant, end: Instant) {
        _uiState.value = _uiState.value.copy(startTime = start, endTime = end)
        load()
    }

    fun deleteCurrentRange() {
        viewModelScope.launch {
            val s = _uiState.value
            _uiState.value = s.copy(isDeleting = true, deleteError = null)
            repository.deleteTelemetry(
                deviceId = deviceId,
                start = formatter.format(s.startTime),
                end = formatter.format(s.endTime),
            ).onSuccess {
                _uiState.value = _uiState.value.copy(isDeleting = false)
                load()
            }.onFailure { e ->
                _uiState.value = _uiState.value.copy(isDeleting = false, deleteError = e.message)
            }
        }
    }

    fun clearDeleteError() {
        _uiState.value = _uiState.value.copy(deleteError = null)
    }
}
