package com.lumirum.lumirumapp.ui.screen.admin

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.repeatOnLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.HealthResponse
import com.lumirum.lumirumapp.data.api.dto.Stats
import com.lumirum.lumirumapp.data.api.dto.User
import com.lumirum.lumirumapp.ui.components.ConfirmDeleteDialog
import com.lumirum.lumirumapp.ui.components.ErrorView
import com.lumirum.lumirumapp.ui.components.LoadingView
import com.lumirum.lumirumapp.ui.components.UiState
import androidx.lifecycle.Lifecycle

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AdminScreen() {
    val container = LocalAppContainer.current
    val viewModel: AdminViewModel = viewModel { AdminViewModel(container.userRepository) }
    val uiState by viewModel.uiState.collectAsState()

    val lifecycleOwner = LocalLifecycleOwner.current
    LaunchedEffect(lifecycleOwner) {
        lifecycleOwner.lifecycle.repeatOnLifecycle(Lifecycle.State.RESUMED) {
            viewModel.load()
        }
    }

    var userToDelete by remember { mutableStateOf<User?>(null) }

    uiState.deleteError?.let { err ->
        LaunchedEffect(err) {
            viewModel.clearDeleteError()
        }
    }

    Scaffold(
        topBar = { TopAppBar(title = { Text(stringResource(R.string.admin)) }) },
    ) { padding ->
        LazyColumn(contentPadding = padding) {
            item {
                when (val health = uiState.health) {
                    is UiState.Loading -> {}
                    is UiState.Error -> HealthBanner(null, health.message)
                    is UiState.Success -> HealthBanner(health.data, null)
                }
            }

            item {
                when (val stats = uiState.stats) {
                    is UiState.Loading -> {}
                    is UiState.Error -> {}
                    is UiState.Success -> StatsCard(stats.data)
                }
            }

            item {
                OutlinedTextField(
                    value = uiState.searchQuery,
                    onValueChange = { viewModel.setSearch(it) },
                    label = { Text(stringResource(R.string.search_users)) },
                    leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
                    modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 8.dp),
                    singleLine = true,
                )
            }

            when (val users = uiState.users) {
                is UiState.Loading -> item { LoadingView(Modifier.height(200.dp)) }
                is UiState.Error -> item { ErrorView(users.message, onRetry = { viewModel.load() }) }
                is UiState.Success -> {
                    val filtered = users.data.filter {
                        uiState.searchQuery.isBlank() || it.username.contains(uiState.searchQuery, ignoreCase = true)
                    }
                    items(filtered) { user ->
                        UserListItem(user = user, onDelete = { userToDelete = user })
                    }
                }
            }
        }
    }

    userToDelete?.let { user ->
        ConfirmDeleteDialog(
            title = stringResource(R.string.delete_user),
            text = stringResource(R.string.delete_user_confirm, user.username),
            onConfirm = {
                viewModel.deleteUser(user.id)
                userToDelete = null
            },
            onDismiss = { userToDelete = null },
        )
    }
}

@Composable
private fun HealthBanner(health: HealthResponse?, error: String?) {
    val isHealthy = health?.isHealthy == true
    val containerColor = if (isHealthy) MaterialTheme.colorScheme.primaryContainer
                         else MaterialTheme.colorScheme.errorContainer
    val contentColor = if (isHealthy) MaterialTheme.colorScheme.onPrimaryContainer
                       else MaterialTheme.colorScheme.onErrorContainer

    Card(
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 8.dp),
        colors = CardDefaults.cardColors(containerColor = containerColor),
    ) {
        Row(Modifier.padding(12.dp), verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            val statusText = when {
                error != null -> "Server unreachable: $error"
                health == null -> "Checking..."
                health.isHealthy -> "Server: healthy"
                else -> "Database disconnected — ${health.status}"
            }
            Text(statusText, style = MaterialTheme.typography.bodySmall, color = contentColor)
        }
    }
}

@Composable
private fun StatsCard(stats: Stats) {
    Card(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
        Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            Text(stringResource(R.string.server_stats), style = MaterialTheme.typography.titleMedium)
            Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceEvenly) {
                StatItem(stringResource(R.string.users), stats.users.toString())
                StatItem(stringResource(R.string.devices), stats.devices.toString())
                StatItem(stringResource(R.string.profiles), stats.profiles.toString())
                StatItem(stringResource(R.string.telemetry), stats.telemetry.toString())
            }
        }
    }
}

@Composable
private fun StatItem(label: String, value: String) {
    Column(horizontalAlignment = androidx.compose.ui.Alignment.CenterHorizontally) {
        Text(value, style = MaterialTheme.typography.headlineSmall, color = MaterialTheme.colorScheme.primary)
        Text(label, style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
    }
}

@Composable
private fun UserListItem(user: User, onDelete: () -> Unit) {
    ListItem(
        headlineContent = { Text(user.username) },
        supportingContent = {
            Text("${user.role.label()} · ID: ${user.id} · ${user.createdAt.take(10)}", style = MaterialTheme.typography.bodySmall)
        },
        trailingContent = {
            IconButton(onClick = onDelete) {
                Icon(Icons.Default.Delete, contentDescription = stringResource(R.string.delete), tint = MaterialTheme.colorScheme.error)
            }
        },
    )
    HorizontalDivider()
}

