package com.lumirum.lumirumapp.ui.screen.profiles

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.LightMode
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
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
fun ProfileDetailScreen(profileId: Long?, onBack: () -> Unit, onViewSchedule: ((Long) -> Unit)? = null) {
    val container = LocalAppContainer.current
    val viewModel: ProfileDetailViewModel = viewModel {
        ProfileDetailViewModel(profileId, container.profileRepository)
    }
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(uiState.isSaved) { if (uiState.isSaved) onBack() }
    LaunchedEffect(uiState.isDeleted) { if (uiState.isDeleted) onBack() }

    var showDeleteDialog by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(if (profileId == null) stringResource(R.string.new_profile) else stringResource(R.string.profile_detail)) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, null)
                    }
                },
                actions = {
                    if (profileId != null && onViewSchedule != null) {
                        IconButton(onClick = { onViewSchedule(profileId) }) {
                            Icon(Icons.Default.LightMode, contentDescription = stringResource(R.string.schedule))
                        }
                    }
                },
            )
        },
    ) { padding ->
        when (val profileState = uiState.profile) {
            is UiState.Loading -> LoadingView(Modifier.padding(padding))
            is UiState.Error -> ErrorView(profileState.message, modifier = Modifier.padding(padding))
            is UiState.Success -> {
                val existing = profileState.data
                val userRole by container.userRole.collectAsState()
                val isAdmin = userRole is Role.Admin

                var name by remember(profileId) { mutableStateOf(existing?.name ?: "") }
                var isShared by remember(profileId) { mutableStateOf(existing?.isShared ?: true) }
                var timezone by remember(profileId) { mutableStateOf(existing?.timezone ?: "UTC") }
                var sleepStart by remember(profileId) { mutableStateOf(existing?.sleepStart ?: "22:00:00") }
                var sleepEnd by remember(profileId) { mutableStateOf(existing?.sleepEnd ?: "07:00:00") }
                var nightMode by remember(profileId) { mutableStateOf(existing?.nightModeEnabled ?: false) }
                var minTemp by remember(profileId) { mutableStateOf((existing?.minColorTemp ?: 2000).toString()) }
                var maxTemp by remember(profileId) { mutableStateOf((existing?.maxColorTemp ?: 6500).toString()) }
                var motionTimeout by remember(profileId) { mutableStateOf((existing?.motionTimeoutSeconds ?: 300).toString()) }
                var latitude by remember(profileId) { mutableStateOf(existing?.latitude?.toString() ?: "") }
                var longitude by remember(profileId) { mutableStateOf(existing?.longitude?.toString() ?: "") }

                Column(
                    modifier = Modifier
                        .padding(padding)
                        .fillMaxSize()
                        .verticalScroll(rememberScrollState())
                        .padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(16.dp),
                ) {
                    OutlinedTextField(value = name, onValueChange = { name = it }, label = { Text(stringResource(R.string.profile_name)) }, modifier = Modifier.fillMaxWidth(), singleLine = true)

                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween, verticalAlignment = Alignment.CenterVertically) {
                        Column(Modifier.weight(1f).padding(end = 12.dp)) {
                            Text(stringResource(R.string.is_shared), style = MaterialTheme.typography.bodyLarge)
                            Text(stringResource(R.string.shared_hint), style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                        }
                        Switch(checked = isShared, onCheckedChange = { isShared = it })
                    }

                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                        Text(stringResource(R.string.night_mode), style = MaterialTheme.typography.bodyLarge)
                        Switch(checked = nightMode, onCheckedChange = { nightMode = it })
                    }

                    OutlinedTextField(value = timezone, onValueChange = { timezone = it }, label = { Text(stringResource(R.string.timezone)) }, modifier = Modifier.fillMaxWidth(), singleLine = true, placeholder = { Text("Europe/Kyiv") })

                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                        OutlinedTextField(value = sleepStart, onValueChange = { sleepStart = it }, label = { Text(stringResource(R.string.sleep_start)) }, modifier = Modifier.weight(1f), singleLine = true, placeholder = { Text("22:00:00") })
                        OutlinedTextField(value = sleepEnd, onValueChange = { sleepEnd = it }, label = { Text(stringResource(R.string.sleep_end)) }, modifier = Modifier.weight(1f), singleLine = true, placeholder = { Text("07:00:00") })
                    }

                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                        OutlinedTextField(value = minTemp, onValueChange = { minTemp = it }, label = { Text(stringResource(R.string.min_color_temp)) }, modifier = Modifier.weight(1f), singleLine = true, keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number))
                        OutlinedTextField(value = maxTemp, onValueChange = { maxTemp = it }, label = { Text(stringResource(R.string.max_color_temp)) }, modifier = Modifier.weight(1f), singleLine = true, keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number))
                    }

                    OutlinedTextField(value = motionTimeout, onValueChange = { motionTimeout = it }, label = { Text(stringResource(R.string.motion_timeout)) }, modifier = Modifier.fillMaxWidth(), singleLine = true, keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number))

                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                        OutlinedTextField(value = latitude, onValueChange = { latitude = it }, label = { Text(stringResource(R.string.latitude)) }, modifier = Modifier.weight(1f), singleLine = true, keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal), placeholder = { Text("49.9808") })
                        OutlinedTextField(value = longitude, onValueChange = { longitude = it }, label = { Text(stringResource(R.string.longitude)) }, modifier = Modifier.weight(1f), singleLine = true, keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal), placeholder = { Text("36.2527") })
                    }

                    if (profileId != null && existing != null) {
                        OutlinedCard(Modifier.fillMaxWidth()) {
                            Column(Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
                                Text(stringResource(R.string.profile_info), style = MaterialTheme.typography.labelMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                                Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                                    Text(stringResource(R.string.created_on), style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                                    Text(existing.createdAt.take(10), style = MaterialTheme.typography.bodySmall)
                                }
                                if (isAdmin) Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                                    Text(stringResource(R.string.owner_id), style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                                    Text(existing.ownerId.toString(), style = MaterialTheme.typography.bodySmall)
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
                        onClick = {
                            viewModel.save(
                                name = name, isShared = isShared, timezone = timezone,
                                sleepStart = sleepStart, sleepEnd = sleepEnd,
                                nightModeEnabled = nightMode,
                                minColorTemp = minTemp.toIntOrNull() ?: 2000,
                                maxColorTemp = maxTemp.toIntOrNull() ?: 6500,
                                motionTimeoutSeconds = motionTimeout.toIntOrNull() ?: 300,
                                latitude = latitude.toDoubleOrNull(),
                                longitude = longitude.toDoubleOrNull(),
                            )
                        },
                        modifier = Modifier.fillMaxWidth(),
                        enabled = !uiState.isSaving && name.isNotBlank(),
                    ) {
                        if (uiState.isSaving) CircularProgressIndicator(Modifier.size(20.dp), strokeWidth = 2.dp, color = MaterialTheme.colorScheme.onPrimary)
                        else Text(stringResource(R.string.save))
                    }

                    if (profileId != null) {
                        OutlinedButton(
                            onClick = { showDeleteDialog = true },
                            modifier = Modifier.fillMaxWidth(),
                            colors = ButtonDefaults.outlinedButtonColors(contentColor = MaterialTheme.colorScheme.error),
                        ) { Text(stringResource(R.string.delete_profile)) }
                    }
                }
            }
        }
    }

    if (showDeleteDialog) {
        ConfirmDeleteDialog(
            title = stringResource(R.string.delete_profile),
            text = stringResource(R.string.delete_profile_confirm),
            onConfirm = { showDeleteDialog = false; viewModel.delete() },
            onDismiss = { showDeleteDialog = false },
        )
    }
}
