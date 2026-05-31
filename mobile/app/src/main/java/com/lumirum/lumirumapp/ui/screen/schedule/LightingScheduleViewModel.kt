package com.lumirum.lumirumapp.ui.screen.schedule

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.LightingSchedule
import com.lumirum.lumirumapp.data.repository.ProfileRepository
import com.lumirum.lumirumapp.ui.components.UiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class LightingScheduleViewModel(
    private val profileId: Long,
    private val repository: ProfileRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow<UiState<LightingSchedule>>(UiState.Loading)
    val uiState: StateFlow<UiState<LightingSchedule>> = _uiState.asStateFlow()

    init {
        load()
    }

    fun load() {
        viewModelScope.launch {
            _uiState.value = UiState.Loading
            _uiState.value = repository.getSchedule(profileId).fold(
                onSuccess = { UiState.Success(it) },
                onFailure = { UiState.Error(it.message ?: "Error") },
            )
        }
    }
}
