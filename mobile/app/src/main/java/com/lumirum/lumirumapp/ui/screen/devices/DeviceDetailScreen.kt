package com.lumirum.lumirumapp.ui.screen.devices

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Timeline
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.Role
import com.lumirum.lumirumapp.ui.components.ConfirmDeleteDialog
import com.lumirum.lumirumapp.ui.components.ErrorView
import com.lumirum.lumirumapp.ui.components.LoadingView
import com.lumirum.lumirumapp.ui.components.UiState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceDetailScreen(
    deviceId: Long?,
    onBack: () -> Unit,
    onViewTelemetry: (Long) -> Unit,
) {
    val container = LocalAppContainer.current
    val viewModel: DeviceDetailViewModel = viewModel {
        DeviceDetailViewModel(deviceId, container.deviceRepository, container.profileRepository)
    }
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(uiState.isSaved) { if (uiState.isSaved) onBack() }
    LaunchedEffect(uiState.isDeleted) { if (uiState.isDeleted) onBack() }

    var showDeleteDialog by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(if (deviceId == null) stringResource(R.string.new_device) else stringResource(R.string.device_detail)) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, null)
                    }
                },
                actions = {
                    if (deviceId != null) {
                        IconButton(onClick = { onViewTelemetry(deviceId) }) {
                            Icon(Icons.Default.Timeline, stringResource(R.string.telemetry))
                        }
                    }
                },
            )
        },
    ) { padding ->
        when (val deviceState = uiState.device) {
            is UiState.Loading -> LoadingView(Modifier.padding(padding))
            is UiState.Error -> ErrorView(deviceState.message, modifier = Modifier.padding(padding))
            is UiState.Success -> {
                val device = deviceState.data
                val userRole by container.userRole.collectAsState()
                val isAdmin = userRole is Role.Admin

                var name by remember(device.id) { mutableStateOf(device.name) }
                var isPublic by remember(device.id) { mutableStateOf(device.isPublic) }
                var selectedProfileId by remember(device.id) { mutableStateOf(device.profileId) }
                var profileDropdownExpanded by remember { mutableStateOf(false) }
                val clipboardManager = LocalClipboardManager.current

                Column(
                    modifier = Modifier
                        .padding(padding)
                        .fillMaxSize()
                        .verticalScroll(rememberScrollState())
                        .padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(16.dp),
                ) {
                    OutlinedTextField(
                        value = name,
                        onValueChange = { name = it },
                        label = { Text(stringResource(R.string.device_name)) },
                        modifier = Modifier.fillMaxWidth(),
                        singleLine = true,
                    )

                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween, verticalAlignment = Alignment.CenterVertically) {
                        Column(Modifier.weight(1f).padding(end = 12.dp)) {
                            Text(stringResource(R.string.public_label), style = MaterialTheme.typography.bodyLarge)
                            Text(stringResource(R.string.public_hint), style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                        }
                        Switch(checked = isPublic, onCheckedChange = { isPublic = it })
                    }

                    ExposedDropdownMenuBox(expanded = profileDropdownExpanded, onExpandedChange = { profileDropdownExpanded = it }) {
                        val selectedProfile = uiState.profiles.find { it.id == selectedProfileId }
                        OutlinedTextField(
                            value = selectedProfile?.name ?: stringResource(R.string.no_profile),
                            onValueChange = {},
                            readOnly = true,
                            label = { Text(stringResource(R.string.profile)) },
                            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = profileDropdownExpanded) },
                            modifier = Modifier.fillMaxWidth().menuAnchor(MenuAnchorType.PrimaryNotEditable),
                        )
                        ExposedDropdownMenu(expanded = profileDropdownExpanded, onDismissRequest = { profileDropdownExpanded = false }) {
                            DropdownMenuItem(text = { Text(stringResource(R.string.no_profile)) }, onClick = { selectedProfileId = null; profileDropdownExpanded = false })
                            uiState.profiles.forEach { profile ->
                                DropdownMenuItem(text = { Text(profile.name) }, onClick = { selectedProfileId = profile.id; profileDropdownExpanded = false })
                            }
                        }
                    }

                    if (deviceId != null) {
                        OutlinedCard(modifier = Modifier.fillMaxWidth()) {
                            Column(Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) {
                                Text(stringResource(R.string.device_info), style = MaterialTheme.typography.labelMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                                InfoRow(stringResource(R.string.created_on), device.createdAt.take(19).replace('T', ' '))
                                device.lastSeen?.let { InfoRow(stringResource(R.string.last_seen), it.take(19).replace('T', ' ')) }
                                device.firmwareVersion?.let { InfoRow(stringResource(R.string.firmware_version), it) }
                                if (isAdmin) {
                                    InfoRow("ID", device.id.toString())
                                    InfoRow(stringResource(R.string.owner_id), device.ownerId.toString())
                                }
                            }
                        }
                    }

                    if (deviceId != null && device.secretKey.isNotBlank()) {
                        OutlinedCard(modifier = Modifier.fillMaxWidth()) {
                            Column(Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                                Text(stringResource(R.string.secret_key), style = MaterialTheme.typography.labelMedium)
                                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                                    Text(device.secretKey.take(24) + "…", style = MaterialTheme.typography.bodySmall, modifier = Modifier.weight(1f))
                                    IconButton(onClick = { clipboardManager.setText(AnnotatedString(device.secretKey)) }, modifier = Modifier.size(24.dp)) {
                                        Icon(Icons.Default.ContentCopy, null, modifier = Modifier.size(16.dp))
                                    }
                                    IconButton(onClick = { viewModel.regenerateKey() }, modifier = Modifier.size(24.dp)) {
                                        Icon(Icons.Default.Refresh, stringResource(R.string.regenerate_key), modifier = Modifier.size(16.dp))
                                    }
                                }
                            }
                        }
                    }

                    uiState.saveError?.let { error ->
                        Card(colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.errorContainer)) {
                            Text(error, modifier = Modifier.padding(12.dp), color = MaterialTheme.colorScheme.onErrorContainer, style = MaterialTheme.typography.bodySmall)
                        }
                    }

                    Button(
                        onClick = { viewModel.save(name, isPublic, selectedProfileId) },
                        modifier = Modifier.fillMaxWidth(),
                        enabled = !uiState.isSaving && name.isNotBlank(),
                    ) {
                        if (uiState.isSaving) CircularProgressIndicator(Modifier.size(20.dp), strokeWidth = 2.dp, color = MaterialTheme.colorScheme.onPrimary)
                        else Text(stringResource(R.string.save))
                    }

                    if (deviceId != null) {
                        OutlinedButton(
                            onClick = { showDeleteDialog = true },
                            modifier = Modifier.fillMaxWidth(),
                            colors = ButtonDefaults.outlinedButtonColors(contentColor = MaterialTheme.colorScheme.error),
                        ) {
                            Text(stringResource(R.string.delete_device))
                        }
                    }
                }
            }
        }
    }

    if (showDeleteDialog) {
        ConfirmDeleteDialog(
            title = stringResource(R.string.delete_device),
            text = stringResource(R.string.delete_device_confirm),
            onConfirm = { showDeleteDialog = false; viewModel.delete() },
            onDismiss = { showDeleteDialog = false },
        )
    }
}

@Composable
private fun InfoRow(label: String, value: String) {
    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
        Text(label, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
        Text(value, style = MaterialTheme.typography.bodySmall)
    }
}
