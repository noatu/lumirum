package com.lumirum.lumirumapp.ui.screen.schedule

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.remember
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.LightingSchedule
import com.lumirum.lumirumapp.ui.components.ChartSeries
import com.lumirum.lumirumapp.ui.components.ErrorView
import com.lumirum.lumirumapp.ui.components.LoadingView
import com.lumirum.lumirumapp.ui.components.TelemetryChart
import com.lumirum.lumirumapp.ui.components.UiState
import java.time.Instant
import java.time.ZoneOffset
import java.time.format.DateTimeFormatter

private val isoParser = DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss").withZone(ZoneOffset.UTC)
private val timeFormatter = DateTimeFormatter.ofPattern("HH:mm").withZone(ZoneOffset.UTC)

private fun parseTimestamp(s: String): Long? = runCatching {
    Instant.from(isoParser.parse(s.take(19))).toEpochMilli()
}.getOrNull()

private data class ScheduleGroup(val colorTemp: Int, val first: String, val last: String)

private fun groupSchedulePoints(points: List<com.lumirum.lumirumapp.data.api.dto.LightingPoint>): List<ScheduleGroup> {
    if (points.isEmpty()) return emptyList()
    val groups = mutableListOf<ScheduleGroup>()
    var start = points[0]
    var end = points[0]
    for (i in 1 until points.size) {
        val cur = points[i]
        if (cur.colorTemp == end.colorTemp) {
            end = cur
        } else {
            groups += ScheduleGroup(start.colorTemp, fmtScheduleTime(start.timestamp), fmtScheduleTime(end.timestamp))
            start = cur
            end = cur
        }
    }
    groups += ScheduleGroup(start.colorTemp, fmtScheduleTime(start.timestamp), fmtScheduleTime(end.timestamp))
    return groups
}

private fun fmtScheduleTime(iso: String): String = iso.take(19).substring(11, 16)

private fun secondsToUtcTime(seconds: Int): String {
    val h = seconds / 3600
    val m = (seconds % 3600) / 60
    return "%02d:%02d UTC".format(h, m)
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LightingScheduleScreen(profileId: Long, onBack: () -> Unit) {
    val container = LocalAppContainer.current
    val viewModel: LightingScheduleViewModel = viewModel {
        LightingScheduleViewModel(profileId, container.profileRepository)
    }
    val uiState by viewModel.uiState.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(stringResource(R.string.lighting_schedule)) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = null)
                    }
                },
            )
        },
    ) { padding ->
        when (val state = uiState) {
            is UiState.Loading -> LoadingView(Modifier.padding(padding))
            is UiState.Error -> ErrorView(state.message, onRetry = { viewModel.load() }, modifier = Modifier.padding(padding))
            is UiState.Success -> ScheduleContent(state.data, Modifier.padding(padding))
        }
    }
}

@Composable
private fun ScheduleContent(schedule: LightingSchedule, modifier: Modifier = Modifier) {
    val points = remember(schedule) {
        schedule.schedule.mapNotNull { pt -> parseTimestamp(pt.timestamp)?.let { it to pt.colorTemp.toFloat() } }
    }
    val groups = remember(schedule) { groupSchedulePoints(schedule.schedule) }

    LazyColumn(modifier = modifier, contentPadding = PaddingValues(16.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
        item {
            Card(Modifier.fillMaxWidth()) {
                Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    Text(stringResource(R.string.profile_info), style = MaterialTheme.typography.labelMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                    InfoRow(stringResource(R.string.sleep_window),
                        "${secondsToUtcTime(schedule.sleepStartUtcSeconds)} – ${secondsToUtcTime(schedule.sleepEndUtcSeconds)}")
                    InfoRow(stringResource(R.string.min_color_temp), "${schedule.minColorTemp} K")
                    InfoRow(stringResource(R.string.max_color_temp), "${schedule.maxColorTemp} K")
                    InfoRow(stringResource(R.string.night_mode), if (schedule.nightModeEnabled) "On" else "Off")
                    InfoRow(stringResource(R.string.motion_timeout), "${schedule.motionTimeoutSeconds} s")
                }
            }
        }

        if (points.isNotEmpty()) {
            item {
                TelemetryChart(
                    series = listOf(ChartSeries("Color temp K", Color(0xFF2196F3), points)),
                    modifier = Modifier.fillMaxWidth(),
                )
            }
        }

        items(groups) { group ->
            val timeLabel = if (group.first == group.last) group.first
                            else "${group.first} – ${group.last}"
            ListItem(
                headlineContent = {
                    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                        Text("${group.colorTemp} K")
                        Text(timeLabel, style = MaterialTheme.typography.bodyMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                    }
                },
            )
            HorizontalDivider()
        }
    }
}

@Composable
private fun InfoRow(label: String, value: String) {
    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
        Text(label, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
        Text(value, style = MaterialTheme.typography.bodySmall)
    }
}
