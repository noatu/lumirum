package com.lumirum.lumirumapp.ui.screen.account

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lumirum.lumirumapp.data.api.dto.AuthResponse
import com.lumirum.lumirumapp.data.repository.AuthRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class AccountUiState(
    val user: AuthResponse? = null,
    val isLoading: Boolean = true,
    val isSaving: Boolean = false,
    val error: String? = null,
    val message: String? = null,
    val isLoggedOut: Boolean = false,
    val isDeleted: Boolean = false,
    val isCreatingUser: Boolean = false,
    val subUserCreated: Boolean = false,
    val createUserError: String? = null,
)

class AccountViewModel(
    private val authRepository: AuthRepository,
    private val onTokenCleared: suspend () -> Unit,
) : ViewModel() {

    private val _uiState = MutableStateFlow(AccountUiState())
    val uiState: StateFlow<AccountUiState> = _uiState.asStateFlow()

    init {
        loadUser()
    }

    private fun loadUser() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isLoading = true)
            authRepository.getCurrentUser()
                .onSuccess { _uiState.value = _uiState.value.copy(isLoading = false, user = it) }
                .onFailure { _uiState.value = _uiState.value.copy(isLoading = false, error = it.message) }
        }
    }

    fun updateAccount(currentPassword: String, newUsername: String?, newPassword: String?) {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isSaving = true, error = null, message = null)
            authRepository.updateAccount(currentPassword, newUsername?.takeIf { it.isNotBlank() }, newPassword?.takeIf { it.isNotBlank() })
                .onSuccess { user ->
                    authRepository.saveSession(user.token)
                    _uiState.value = _uiState.value.copy(isSaving = false, user = user, message = "Account updated")
                }
                .onFailure { e ->
                    _uiState.value = _uiState.value.copy(isSaving = false, error = e.message)
                }
        }
    }

    fun logout() {
        viewModelScope.launch {
            onTokenCleared()
            _uiState.value = _uiState.value.copy(isLoggedOut = true)
        }
    }

    fun deleteAccount(password: String) {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isSaving = true, error = null)
            authRepository.deleteAccount(password)
                .onSuccess {
                    onTokenCleared()
                    _uiState.value = _uiState.value.copy(isSaving = false, isDeleted = true)
                }
                .onFailure { e ->
                    _uiState.value = _uiState.value.copy(isSaving = false, error = e.message)
                }
        }
    }

    fun createSubUser(username: String, password: String) {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isCreatingUser = true, createUserError = null, message = null)
            authRepository.register(username, password)
                .onSuccess {
                    _uiState.value = _uiState.value.copy(isCreatingUser = false, subUserCreated = true)
                }
                .onFailure { e ->
                    _uiState.value = _uiState.value.copy(isCreatingUser = false, createUserError = e.message)
                }
        }
    }

    fun clearMessage() {
        _uiState.value = _uiState.value.copy(message = null, error = null, subUserCreated = false, createUserError = null)
    }
}
