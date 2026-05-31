package com.lumirum.lumirumapp.ui.screen.profiles

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Group
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.repeatOnLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.Profile
import com.lumirum.lumirumapp.ui.components.EmptyView
import com.lumirum.lumirumapp.ui.components.ErrorView
import com.lumirum.lumirumapp.ui.components.LoadingView
import com.lumirum.lumirumapp.ui.components.UiState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfilesScreen(
    onProfileClick: (Long) -> Unit,
    onCreateClick: () -> Unit,
) {
    val container = LocalAppContainer.current
    val viewModel: ProfilesViewModel = viewModel { ProfilesViewModel(container.profileRepository) }
    val uiState by viewModel.uiState.collectAsState()

    val lifecycleOwner = LocalLifecycleOwner.current
    LaunchedEffect(lifecycleOwner) {
        lifecycleOwner.lifecycle.repeatOnLifecycle(Lifecycle.State.RESUMED) {
            viewModel.load()
        }
    }

    Scaffold(
        topBar = { TopAppBar(title = { Text(stringResource(R.string.profiles)) }) },
        floatingActionButton = {
            FloatingActionButton(onClick = { if (!uiState.isRefreshing) onCreateClick() }) {
                if (uiState.isRefreshing) {
                    CircularProgressIndicator(Modifier.size(24.dp), strokeWidth = 2.5.dp, color = MaterialTheme.colorScheme.onPrimaryContainer)
                } else {
                    Icon(Icons.Default.Add, stringResource(R.string.new_profile))
                }
            }
        },
    ) { padding ->
        when (val state = uiState.list) {
            is UiState.Loading -> LoadingView(Modifier.padding(padding))
            is UiState.Error -> ErrorView(state.message, onRetry = { viewModel.load() }, modifier = Modifier.padding(padding))
            is UiState.Success -> {
                if (state.data.isEmpty()) {
                    EmptyView(stringResource(R.string.no_profiles), Modifier.padding(padding))
                } else {
                    LazyColumn(contentPadding = padding) {
                        items(state.data, key = { it.id }) { profile ->
                            ProfileListItem(profile = profile, onClick = { onProfileClick(profile.id) })
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun ProfileListItem(profile: Profile, onClick: () -> Unit) {
    ListItem(
        headlineContent = { Text(profile.name) },
        supportingContent = {
            Text("${profile.timezone} · ${profile.sleepStart.take(5)}–${profile.sleepEnd.take(5)}", style = MaterialTheme.typography.bodySmall)
        },
        trailingContent = {
            if (profile.isShared) {
                Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                    Icon(Icons.Default.Group, null, Modifier.size(16.dp), MaterialTheme.colorScheme.primary)
                    Text(stringResource(R.string.shared), style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.primary)
                }
            }
        },
        modifier = Modifier.clickable(onClick = onClick),
    )
    HorizontalDivider()
}
