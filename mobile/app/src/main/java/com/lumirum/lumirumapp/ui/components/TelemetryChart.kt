package com.lumirum.lumirumapp.ui.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.*
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

data class ChartSeries(
    val label: String,
    val color: Color,
    val points: List<Pair<Long, Float>>,
)

@Composable
fun TelemetryChart(
    series: List<ChartSeries>,
    modifier: Modifier = Modifier,
) {
    val allPoints = series.flatMap { it.points }
    if (allPoints.isEmpty()) return

    val minX = allPoints.minOf { it.first }.toFloat()
    val maxX = allPoints.maxOf { it.first }.toFloat()
    val xRange = (maxX - minX).coerceAtLeast(1f)

    data class SeriesScale(val minY: Float, val yRange: Float)
    val scales = series.associate { s ->
        val lo = s.points.minOf { it.second }
        val hi = s.points.maxOf { it.second }
        s to SeriesScale(lo, (hi - lo).coerceAtLeast(1f))
    }

    val primaryColor = MaterialTheme.colorScheme.primary
    val onSurface = MaterialTheme.colorScheme.onSurfaceVariant

    Column(modifier) {
        Canvas(
            modifier = Modifier
                .fillMaxWidth()
                .height(200.dp)
                .padding(start = 8.dp, end = 8.dp, top = 8.dp, bottom = 24.dp),
        ) {
            val gridLines = 4
            for (i in 0..gridLines) {
                val y = size.height * i / gridLines
                drawLine(
                    color = onSurface.copy(alpha = 0.15f),
                    start = Offset(0f, y),
                    end = Offset(size.width, y),
                    strokeWidth = 1.dp.toPx(),
                )
            }

            for (s in series) {
                if (s.points.size < 2) continue
                val (minY, yRange) = scales[s] ?: continue
                val path = Path()
                s.points.forEachIndexed { index, (x, y) ->
                    val xPos = ((x.toFloat() - minX) / xRange) * size.width
                    val yPos = size.height - ((y - minY) / yRange) * size.height
                    if (index == 0) path.moveTo(xPos, yPos) else path.lineTo(xPos, yPos)
                }
                drawPath(path, color = s.color, style = Stroke(width = 2.dp.toPx()))

                s.points.forEach { (x, y) ->
                    val xPos = ((x.toFloat() - minX) / xRange) * size.width
                    val yPos = size.height - ((y - minY) / yRange) * size.height
                    drawCircle(color = s.color, radius = 3.dp.toPx(), center = Offset(xPos, yPos))
                }
            }
        }

        Row(
            modifier = Modifier.padding(horizontal = 8.dp),
            horizontalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            series.forEach { s ->
                Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                    Canvas(Modifier.size(12.dp)) {
                        drawCircle(color = s.color, radius = 6.dp.toPx())
                    }
                    Text(s.label, fontSize = 11.sp, color = onSurface)
                }
            }
        }
    }
}
