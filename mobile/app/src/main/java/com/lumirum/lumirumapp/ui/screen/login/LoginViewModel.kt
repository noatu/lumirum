package com.lumirum.lumirumapp.ui.screen.login

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.Role
import com.lumirum.lumirumapp.data.repository.AuthRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class LoginUiState(
    val isLoading: Boolean = false,
    val error: String? = null,
    val loginSuccess: Boolean = false,
)

class LoginViewModel(
    private val authRepository: AuthRepository,
    private val onSessionStarted: suspend (token: String, role: Role) -> Unit,
) : ViewModel() {

    private val _uiState = MutableStateFlow(LoginUiState())
    val uiState: StateFlow<LoginUiState> = _uiState.asStateFlow()

    fun login(username: String, password: String) {
        if (username.isBlank() || password.isBlank()) {
            _uiState.value = _uiState.value.copy(error = "Username and password are required")
            return
        }
        viewModelScope.launch {
            _uiState.value = LoginUiState(isLoading = true)
            authRepository.login(username, password)
                .onSuccess { response ->
                    authRepository.saveSession(response.token)
                    onSessionStarted(response.token, response.role)
                    _uiState.value = LoginUiState(loginSuccess = true)
                }
                .onFailure { e ->
                    _uiState.value = LoginUiState(error = e.message ?: "Login failed")
                }
        }
    }

    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
}
