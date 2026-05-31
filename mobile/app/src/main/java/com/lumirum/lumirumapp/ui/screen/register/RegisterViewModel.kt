package com.lumirum.lumirumapp.ui.screen.register

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.Role
import com.lumirum.lumirumapp.data.repository.AuthRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class RegisterUiState(
    val isLoading: Boolean = false,
    val error: String? = null,
    val success: Boolean = false,
)

class RegisterViewModel(
    private val authRepository: AuthRepository,
    private val onSessionStarted: suspend (token: String, role: Role) -> Unit,
) : ViewModel() {

    private val _uiState = MutableStateFlow(RegisterUiState())
    val uiState: StateFlow<RegisterUiState> = _uiState.asStateFlow()

    fun register(username: String, password: String, confirmPassword: String) {
        when {
            username.isBlank() || password.isBlank() ->
                _uiState.value = _uiState.value.copy(error = "All fields are required")
            username.length < 3 ->
                _uiState.value = _uiState.value.copy(error = "Username must be at least 3 characters")
            password.length < 8 ->
                _uiState.value = _uiState.value.copy(error = "Password must be at least 8 characters")
            password != confirmPassword ->
                _uiState.value = _uiState.value.copy(error = "Passwords do not match")
            else -> {
                viewModelScope.launch {
                    _uiState.value = RegisterUiState(isLoading = true)
                    authRepository.register(username, password)
                        .onSuccess { response ->
                            authRepository.saveSession(response.token)
                            onSessionStarted(response.token, response.role)
                            _uiState.value = RegisterUiState(success = true)
                        }
                        .onFailure { e ->
                            _uiState.value = RegisterUiState(error = e.message ?: "Registration failed")
                        }
                }
            }
        }
    }
}
