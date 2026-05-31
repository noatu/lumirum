package com.lumirum.lumirumapp.ui.screen.devices

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.Device
import com.lumirum.lumirumapp.data.repository.DeviceRepository
import com.lumirum.lumirumapp.ui.components.UiState
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class DevicesUiState(
    val list: UiState<List<Device>> = UiState.Loading,
    val isRefreshing: Boolean = false,
)

class DevicesViewModel(private val repository: DeviceRepository) : ViewModel() {

    private val _uiState = MutableStateFlow(DevicesUiState())
    val uiState: StateFlow<DevicesUiState> = _uiState.asStateFlow()

    init {
        load()
    }

    fun load() {
        viewModelScope.launch {
            val hasData = _uiState.value.list is UiState.Success
            val spinnerJob = if (hasData) launch {
                delay(1000)
                _uiState.value = _uiState.value.copy(isRefreshing = true)
            } else {
                _uiState.value = _uiState.value.copy(list = UiState.Loading)
                null
            }
            val result = repository.getDevices().fold(
                onSuccess = { UiState.Success(it) },
                onFailure = { UiState.Error(it.message ?: "Unknown error") },
            )
            spinnerJob?.cancel()
            _uiState.value = _uiState.value.copy(list = result, isRefreshing = false)
        }
    }
}
