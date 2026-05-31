package com.lumirum.lumirumapp.ui.screen.profiles

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.CreateProfileRequest
import com.lumirum.lumirumapp.data.api.dto.Profile
import com.lumirum.lumirumapp.data.repository.ProfileRepository
import com.lumirum.lumirumapp.ui.components.UiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class ProfileDetailUiState(
    val profile: UiState<Profile?> = UiState.Loading,
    val isSaving: Boolean = false,
    val saveError: String? = null,
    val isSaved: Boolean = false,
    val isDeleted: Boolean = false,
)

class ProfileDetailViewModel(
    private val profileId: Long?,
    private val repository: ProfileRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow(ProfileDetailUiState())
    val uiState: StateFlow<ProfileDetailUiState> = _uiState.asStateFlow()

    init {
        if (profileId != null) loadProfile()
        else _uiState.value = _uiState.value.copy(profile = UiState.Success(null))
    }

    private fun loadProfile() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(profile = UiState.Loading)
            repository.getProfile(profileId!!)
                .onSuccess { _uiState.value = _uiState.value.copy(profile = UiState.Success(it)) }
                .onFailure { _uiState.value = _uiState.value.copy(profile = UiState.Error(it.message ?: "Error")) }
        }
    }

    fun save(
        name: String,
        isShared: Boolean,
        timezone: String,
        sleepStart: String,
        sleepEnd: String,
        nightModeEnabled: Boolean,
        minColorTemp: Int,
        maxColorTemp: Int,
        motionTimeoutSeconds: Int,
        latitude: Double?,
        longitude: Double?,
    ) {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isSaving = true, saveError = null)
            val request = CreateProfileRequest(
                name = name,
                isShared = isShared,
                timezone = timezone,
                sleepStart = sleepStart,
                sleepEnd = sleepEnd,
                nightModeEnabled = nightModeEnabled,
                minColorTemp = minColorTemp,
                maxColorTemp = maxColorTemp,
                motionTimeoutSeconds = motionTimeoutSeconds,
                latitude = latitude,
                longitude = longitude,
            )
            val result = if (profileId == null) repository.createProfile(request)
            else repository.updateProfile(profileId, request)
            result
                .onSuccess { profile ->
                    _uiState.value = _uiState.value.copy(
                        isSaving = false,
                        isSaved = true,
                        profile = UiState.Success(profile),
                    )
                }
                .onFailure { e ->
                    _uiState.value = _uiState.value.copy(isSaving = false, saveError = e.message)
                }
        }
    }

    fun delete() {
        if (profileId == null) return
        viewModelScope.launch {
            repository.deleteProfile(profileId)
                .onSuccess { _uiState.value = _uiState.value.copy(isDeleted = true) }
                .onFailure { e -> _uiState.value = _uiState.value.copy(saveError = e.message) }
        }
    }
}
