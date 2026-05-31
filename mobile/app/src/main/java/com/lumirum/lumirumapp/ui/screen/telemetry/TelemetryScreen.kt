package com.lumirum.lumirumapp.ui.screen.telemetry

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.List
import androidx.compose.material.icons.filled.BarChart
import androidx.compose.material.icons.filled.DateRange
import androidx.compose.material.icons.filled.Delete
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.Telemetry
import com.lumirum.lumirumapp.ui.components.ChartSeries
import com.lumirum.lumirumapp.ui.components.EmptyView
import com.lumirum.lumirumapp.ui.components.ErrorView
import com.lumirum.lumirumapp.ui.components.LoadingView
import com.lumirum.lumirumapp.ui.components.TelemetryChart
import com.lumirum.lumirumapp.ui.components.UiState
import java.time.Instant
import java.time.ZoneOffset
import java.time.format.DateTimeFormatter

private val isoParser = DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss").withZone(ZoneOffset.UTC)
private val dateLabel = DateTimeFormatter.ofPattern("MMM d").withZone(ZoneOffset.UTC)

private fun parseTimestamp(s: String): Long? = runCatching {
    Instant.from(isoParser.parse(s.take(19))).toEpochMilli()
}.getOrNull()

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TelemetryScreen(deviceId: Long, onBack: () -> Unit) {
    val container = LocalAppContainer.current
    val viewModel: TelemetryViewModel = viewModel {
        TelemetryViewModel(deviceId, container.telemetryRepository)
    }
    val uiState by viewModel.uiState.collectAsState()
    var showChart by remember { mutableStateOf(true) }
    var showDatePicker by remember { mutableStateOf(false) }
    var showDeleteConfirm by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Text(stringResource(R.string.telemetry))
                        Text(
                            "${dateLabel.format(uiState.startTime)} – ${dateLabel.format(uiState.endTime)}",
                            style = MaterialTheme.typography.labelSmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = null)
                    }
                },
                actions = {
                    IconButton(onClick = { showDatePicker = true }) {
                        Icon(Icons.Default.DateRange, contentDescription = stringResource(R.string.select_date_range))
                    }
                    IconButton(onClick = { showChart = !showChart }) {
                        Icon(
                            if (showChart) Icons.AutoMirrored.Filled.List else Icons.Default.BarChart,
                            contentDescription = null,
                        )
                    }
                    IconButton(onClick = { showDeleteConfirm = true }) {
                        Icon(Icons.Default.Delete, contentDescription = stringResource(R.string.delete_telemetry), tint = MaterialTheme.colorScheme.error)
                    }
                },
            )
        },
    ) { padding ->
        Column(Modifier.padding(padding).fillMaxSize()) {
            uiState.deleteError?.let { err ->
                Card(
                    modifier = Modifier.fillMaxWidth().padding(8.dp),
                    colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.errorContainer),
                ) {
                    Text(err, modifier = Modifier.padding(12.dp), color = MaterialTheme.colorScheme.onErrorContainer, style = MaterialTheme.typography.bodySmall)
                }
            }

            when (val state = uiState.data) {
                is UiState.Loading -> LoadingView()
                is UiState.Error -> ErrorView(state.message, onRetry = { viewModel.load() })
                is UiState.Success -> {
                    if (state.data.isEmpty()) {
                        EmptyView(stringResource(R.string.no_telemetry))
                    } else if (showChart) {
                        TelemetryChartView(state.data)
                    } else {
                        TelemetryListView(state.data)
                    }
                }
            }
        }
    }

    if (showDatePicker) {
        DateRangePickerDialog(
            initialStart = uiState.startTime,
            initialEnd = uiState.endTime,
            onConfirm = { start, end ->
                showDatePicker = false
                viewModel.setRange(start, end)
            },
            onDismiss = { showDatePicker = false },
        )
    }

    if (showDeleteConfirm) {
        AlertDialog(
            onDismissRequest = { showDeleteConfirm = false },
            title = { Text(stringResource(R.string.delete_telemetry)) },
            text = { Text(stringResource(R.string.delete_telemetry_confirm)) },
            confirmButton = {
                TextButton(
                    onClick = { showDeleteConfirm = false; viewModel.deleteCurrentRange() },
                    colors = ButtonDefaults.textButtonColors(contentColor = MaterialTheme.colorScheme.error),
                ) { Text(stringResource(R.string.delete)) }
            },
            dismissButton = { TextButton(onClick = { showDeleteConfirm = false }) { Text(stringResource(R.string.cancel)) } },
        )
    }
}

private fun parseHHmm(s: String): Long? {
    val parts = s.trim().split(":")
    val h = parts.getOrNull(0)?.toLongOrNull() ?: return null
    val m = parts.getOrNull(1)?.toLongOrNull() ?: return null
    if (h !in 0..23 || m !in 0..59) return null
    return (h * 3600 + m * 60) * 1000L
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun DateRangePickerDialog(
    initialStart: Instant,
    initialEnd: Instant,
    onConfirm: (Instant, Instant) -> Unit,
    onDismiss: () -> Unit,
) {
    val state = rememberDateRangePickerState(
        initialSelectedStartDateMillis = initialStart.toEpochMilli(),
        initialSelectedEndDateMillis = initialEnd.toEpochMilli(),
    )
    var startTime by remember { mutableStateOf("00:00") }
    var endTime by remember { mutableStateOf("23:59") }

    Dialog(
        onDismissRequest = onDismiss,
        properties = DialogProperties(usePlatformDefaultWidth = false),
    ) {
        Surface(
            modifier = Modifier.fillMaxWidth(0.92f).wrapContentHeight(),
            shape = MaterialTheme.shapes.extraLarge,
            tonalElevation = 6.dp,
        ) {
            Column(modifier = Modifier.padding(bottom = 8.dp)) {
                DateRangePicker(
                    state = state,
                    modifier = Modifier.weight(1f, fill = false).fillMaxWidth(),
                    title = null,
                    headline = null,
                    showModeToggle = false,
                )
                HorizontalDivider(Modifier.padding(horizontal = 16.dp))
                Row(
                    Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 8.dp),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    OutlinedTextField(
                        value = startTime,
                        onValueChange = { startTime = it },
                        label = { Text("Start") },
                        modifier = Modifier.weight(1f),
                        singleLine = true,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        isError = parseHHmm(startTime) == null,
                    )
                    OutlinedTextField(
                        value = endTime,
                        onValueChange = { endTime = it },
                        label = { Text("End") },
                        modifier = Modifier.weight(1f),
                        singleLine = true,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        isError = parseHHmm(endTime) == null,
                    )
                }
                Row(
                    Modifier.fillMaxWidth().padding(horizontal = 8.dp),
                    horizontalArrangement = Arrangement.End,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    TextButton(onClick = onDismiss) { Text(stringResource(R.string.cancel)) }
                    TextButton(
                        onClick = {
                            val s = state.selectedStartDateMillis ?: return@TextButton
                            val e = state.selectedEndDateMillis ?: return@TextButton
                            val startOffset = parseHHmm(startTime) ?: 0L
                            val endOffset = parseHHmm(endTime) ?: (86399L * 1000L)
                            onConfirm(Instant.ofEpochMilli(s + startOffset), Instant.ofEpochMilli(e + endOffset + 59_000L))
                        },
                        enabled = state.selectedStartDateMillis != null && state.selectedEndDateMillis != null,
                    ) { Text("OK") }
                }
            }
        }
    }
}

@Composable
private fun TelemetryChartView(entries: List<Telemetry>) {
    val brightnessSeries = entries
        .mapNotNull { t -> t.brightness?.let { parseTimestamp(t.createdAt)?.to(it.toFloat()) } }

    val ambientSeries = entries
        .mapNotNull { t -> t.ambientLight?.let { parseTimestamp(t.createdAt)?.to(it.toFloat()) } }

    val colorTempSeries = entries
        .mapNotNull { t -> t.colorTemp?.let { parseTimestamp(t.createdAt)?.to(it.toFloat()) } }

    val series = buildList {
        if (brightnessSeries.isNotEmpty()) add(ChartSeries("Brightness %", Color(0xFF4CAF50), brightnessSeries))
        if (ambientSeries.isNotEmpty()) add(ChartSeries("Ambient lux", Color(0xFFFF9800), ambientSeries))
        if (colorTempSeries.isNotEmpty()) add(ChartSeries("Color temp K", Color(0xFF2196F3), colorTempSeries))
    }

    if (series.isEmpty()) {
        EmptyView("No numeric telemetry in this period")
    } else {
        TelemetryChart(series = series, modifier = Modifier.fillMaxWidth().padding(8.dp))
    }
}

private data class TelemetryGroup(
    val colorTemp: Int?,
    val startTime: String,
    val endTime: String,
)

private fun groupByColorTemp(entries: List<Telemetry>): List<TelemetryGroup> {
    val sorted = entries.sortedBy { it.createdAt }
    if (sorted.isEmpty()) return emptyList()
    val groups = mutableListOf<TelemetryGroup>()
    var start = sorted[0]
    var end = sorted[0]
    for (i in 1 until sorted.size) {
        val cur = sorted[i]
        if (cur.colorTemp == end.colorTemp) {
            end = cur
        } else {
            groups += TelemetryGroup(start.colorTemp, start.createdAt, end.createdAt)
            start = cur
            end = cur
        }
    }
    groups += TelemetryGroup(start.colorTemp, start.createdAt, end.createdAt)
    return groups.reversed()
}

private fun fmtTime(iso: String): String {
    val t = iso.take(19)
    return t.substring(11, 16) // HH:mm
}

@Composable
private fun TelemetryListView(entries: List<Telemetry>) {
    val groups = remember(entries) { groupByColorTemp(entries) }
    LazyColumn {
        items(groups) { group ->
            val tempLabel = group.colorTemp?.let { "$it K" } ?: "– K"
            val timeLabel = if (group.startTime == group.endTime) fmtTime(group.startTime)
                            else "${fmtTime(group.startTime)} – ${fmtTime(group.endTime)}"
            ListItem(
                headlineContent = {
                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                        Text(tempLabel)
                        Text(timeLabel, style = MaterialTheme.typography.bodyMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                    }
                },
            )
            HorizontalDivider()
        }
    }
}
