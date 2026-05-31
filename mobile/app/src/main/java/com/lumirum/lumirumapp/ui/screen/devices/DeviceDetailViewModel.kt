package com.lumirum.lumirumapp.ui.screen.devices

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.Device
import com.lumirum.lumirumapp.data.api.dto.Profile
import com.lumirum.lumirumapp.data.repository.DeviceRepository
import com.lumirum.lumirumapp.data.repository.ProfileRepository
import com.lumirum.lumirumapp.ui.components.UiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class DeviceDetailUiState(
    val device: UiState<Device> = UiState.Loading,
    val profiles: List<Profile> = emptyList(),
    val isSaving: Boolean = false,
    val saveError: String? = null,
    val isSaved: Boolean = false,
    val isDeleted: Boolean = false,
)

class DeviceDetailViewModel(
    private val deviceId: Long?,
    private val deviceRepository: DeviceRepository,
    private val profileRepository: ProfileRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow(DeviceDetailUiState())
    val uiState: StateFlow<DeviceDetailUiState> = _uiState.asStateFlow()

    init {
        loadProfiles()
        if (deviceId != null) loadDevice()
        else _uiState.value = _uiState.value.copy(device = UiState.Success(Device(0, "", "", 0, true, "", null, null, null)))
    }

    private fun loadDevice() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(device = UiState.Loading)
            deviceRepository.getDevice(deviceId!!)
                .onSuccess { _uiState.value = _uiState.value.copy(device = UiState.Success(it)) }
                .onFailure { _uiState.value = _uiState.value.copy(device = UiState.Error(it.message ?: "Error")) }
        }
    }

    private fun loadProfiles() {
        viewModelScope.launch {
            profileRepository.getProfiles()
                .onSuccess { _uiState.value = _uiState.value.copy(profiles = it) }
        }
    }

    fun save(name: String, isPublic: Boolean, profileId: Long?) {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isSaving = true, saveError = null)
            val result = if (deviceId == null) {
                deviceRepository.createDevice(name, isPublic, profileId)
            } else {
                deviceRepository.updateDevice(deviceId, name, isPublic, profileId)
            }
            result
                .onSuccess { device ->
                    _uiState.value = _uiState.value.copy(
                        isSaving = false,
                        isSaved = true,
                        device = UiState.Success(device),
                    )
                }
                .onFailure { e ->
                    _uiState.value = _uiState.value.copy(isSaving = false, saveError = e.message)
                }
        }
    }

    fun delete() {
        if (deviceId == null) return
        viewModelScope.launch {
            deviceRepository.deleteDevice(deviceId)
                .onSuccess { _uiState.value = _uiState.value.copy(isDeleted = true) }
                .onFailure { e -> _uiState.value = _uiState.value.copy(saveError = e.message) }
        }
    }

    fun regenerateKey() {
        if (deviceId == null) return
        viewModelScope.launch {
            deviceRepository.regenerateKey(deviceId)
                .onSuccess { _uiState.value = _uiState.value.copy(device = UiState.Success(it)) }
                .onFailure { e -> _uiState.value = _uiState.value.copy(saveError = e.message) }
        }
    }

    fun clearSaveError() {
        _uiState.value = _uiState.value.copy(saveError = null)
    }
}
