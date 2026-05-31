package com.lumirum.lumirumapp.ui.screen.devices

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.filled.LockOpen
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.repeatOnLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.Device
import com.lumirum.lumirumapp.ui.components.EmptyView
import com.lumirum.lumirumapp.ui.components.ErrorView
import com.lumirum.lumirumapp.ui.components.LoadingView
import com.lumirum.lumirumapp.ui.components.UiState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DevicesScreen(
    onDeviceClick: (Long) -> Unit,
    onCreateClick: () -> Unit,
) {
    val container = LocalAppContainer.current
    val viewModel: DevicesViewModel = viewModel { DevicesViewModel(container.deviceRepository) }
    val uiState by viewModel.uiState.collectAsState()

    val lifecycleOwner = LocalLifecycleOwner.current
    LaunchedEffect(lifecycleOwner) {
        lifecycleOwner.lifecycle.repeatOnLifecycle(Lifecycle.State.RESUMED) {
            viewModel.load()
        }
    }

    Scaffold(
        topBar = { TopAppBar(title = { Text(stringResource(R.string.devices)) }) },
        floatingActionButton = {
            FloatingActionButton(onClick = { if (!uiState.isRefreshing) onCreateClick() }) {
                if (uiState.isRefreshing) {
                    CircularProgressIndicator(Modifier.size(24.dp), strokeWidth = 2.5.dp, color = MaterialTheme.colorScheme.onPrimaryContainer)
                } else {
                    Icon(Icons.Default.Add, contentDescription = stringResource(R.string.new_device))
                }
            }
        },
    ) { padding ->
        when (val state = uiState.list) {
            is UiState.Loading -> LoadingView(Modifier.padding(padding))
            is UiState.Error -> ErrorView(state.message, onRetry = { viewModel.load() }, modifier = Modifier.padding(padding))
            is UiState.Success -> {
                if (state.data.isEmpty()) {
                    EmptyView(stringResource(R.string.no_devices), Modifier.padding(padding))
                } else {
                    LazyColumn(contentPadding = padding) {
                        items(state.data, key = { it.id }) { device ->
                            DeviceListItem(device = device, onClick = { onDeviceClick(device.id) })
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun DeviceListItem(device: Device, onClick: () -> Unit) {
    ListItem(
        headlineContent = { Text(device.name) },
        supportingContent = {
            Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                device.lastSeen?.let {
                    Text(
                        stringResource(R.string.last_seen_fmt, it.take(19).replace('T', ' ')),
                        style = MaterialTheme.typography.bodySmall,
                    )
                }
                device.firmwareVersion?.let {
                    Text("FW: $it", style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                }
            }
        },
        trailingContent = {
            Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                if (device.isPublic) {
                    Icon(Icons.Default.LockOpen, null, Modifier.size(16.dp), MaterialTheme.colorScheme.primary)
                    Text(stringResource(R.string.public_label), style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.primary)
                } else {
                    Icon(Icons.Default.Lock, null, Modifier.size(16.dp), MaterialTheme.colorScheme.onSurfaceVariant)
                    Text(stringResource(R.string.private_label), style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                }
            }
        },
        modifier = Modifier.clickable(onClick = onClick),
    )
    HorizontalDivider()
}
