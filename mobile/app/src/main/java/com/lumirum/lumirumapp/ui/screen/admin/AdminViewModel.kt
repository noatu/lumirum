package com.lumirum.lumirumapp.ui.screen.admin

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.HealthResponse
import com.lumirum.lumirumapp.data.api.dto.Stats
import com.lumirum.lumirumapp.data.api.dto.User
import com.lumirum.lumirumapp.data.repository.UserRepository
import com.lumirum.lumirumapp.ui.components.UiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class AdminUiState(
    val health: UiState<HealthResponse> = UiState.Loading,
    val stats: UiState<Stats> = UiState.Loading,
    val users: UiState<List<User>> = UiState.Loading,
    val searchQuery: String = "",
    val deleteError: String? = null,
)

class AdminViewModel(private val repository: UserRepository) : ViewModel() {

    private val _uiState = MutableStateFlow(AdminUiState())
    val uiState: StateFlow<AdminUiState> = _uiState.asStateFlow()

    init {
        load()
    }

    fun load() {
        loadHealth()
        loadStats()
        loadUsers()
    }

    private fun loadHealth() {
        viewModelScope.launch {
            repository.getHealth()
                .onSuccess { _uiState.value = _uiState.value.copy(health = UiState.Success(it)) }
                .onFailure { _uiState.value = _uiState.value.copy(health = UiState.Error(it.message ?: "Error")) }
        }
    }

    private fun loadStats() {
        viewModelScope.launch {
            repository.getStats()
                .onSuccess { _uiState.value = _uiState.value.copy(stats = UiState.Success(it)) }
                .onFailure { _uiState.value = _uiState.value.copy(stats = UiState.Error(it.message ?: "Error")) }
        }
    }

    private fun loadUsers() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(users = UiState.Loading)
            repository.getUsers()
                .onSuccess { _uiState.value = _uiState.value.copy(users = UiState.Success(it)) }
                .onFailure { _uiState.value = _uiState.value.copy(users = UiState.Error(it.message ?: "Error")) }
        }
    }

    fun setSearch(query: String) {
        _uiState.value = _uiState.value.copy(searchQuery = query)
    }

    fun deleteUser(id: Long) {
        viewModelScope.launch {
            repository.deleteUser(id)
                .onSuccess { load() }
                .onFailure { e -> _uiState.value = _uiState.value.copy(deleteError = e.message) }
        }
    }

    fun clearDeleteError() {
        _uiState.value = _uiState.value.copy(deleteError = null)
    }
}
