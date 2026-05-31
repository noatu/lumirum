import { Badge, Card, Group, Loader, SimpleGrid, Stack, Text, Title } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router-dom";
import {
  CartesianGrid,
  ComposedChart,
  Line,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { getProfileSchedule } from "../api/profiles";
import { queryKeys } from "../lib/queryKeys";
import { formatDateTime, formatTime } from "../lib/dates";

export function LightingSchedulePage() {
  const { t, i18n } = useTranslation();
  const { id } = useParams<{ id: string }>();
  const profileId = parseInt(id!, 10);

  const { data: schedule, isLoading } = useQuery({
    queryKey: queryKeys.schedule(profileId),
    queryFn: () => getProfileSchedule(profileId),
  });

  if (isLoading) return <Loader />;
  if (!schedule) return null;

  const chartData = schedule.schedule.map((point) => ({
    time: new Date(point.utc).getTime(),
    temp: point.temp,
  }));

  return (
    <Stack>
      <Title order={2}>{t("schedule.title")}</Title>

      <SimpleGrid cols={{ base: 2, sm: 3 }}>
        <Card withBorder>
          <Text size="xs" c="dimmed">{t("schedule.sleepStart")}</Text>
          <Text fw={600}>{formatTime(schedule.sleep_start_utc_seconds)}</Text>
        </Card>
        <Card withBorder>
          <Text size="xs" c="dimmed">{t("schedule.sleepEnd")}</Text>
          <Text fw={600}>{formatTime(schedule.sleep_end_utc_seconds)}</Text>
        </Card>
        <Card withBorder>
          <Text size="xs" c="dimmed">{t("schedule.minColorTemp")}</Text>
          <Text fw={600}>{schedule.min_color_temp} {t("schedule.kelvin")}</Text>
        </Card>
        <Card withBorder>
          <Text size="xs" c="dimmed">{t("schedule.maxColorTemp")}</Text>
          <Text fw={600}>{schedule.max_color_temp} {t("schedule.kelvin")}</Text>
        </Card>
        <Card withBorder>
          <Text size="xs" c="dimmed">{t("schedule.motionTimeout")}</Text>
          <Text fw={600}>{schedule.motion_timeout_seconds} {t("schedule.seconds")}</Text>
        </Card>
        <Card withBorder>
          <Text size="xs" c="dimmed">{t("schedule.nightMode")}</Text>
          <Badge color={schedule.night_mode_enabled ? "blue" : "gray"}>
            {schedule.night_mode_enabled ? t("schedule.enabled") : t("schedule.disabled")}
          </Badge>
        </Card>
      </SimpleGrid>

      <Title order={4}>{t("schedule.chartTitle")}</Title>

      <Group justify="center" h={320}>
        <ResponsiveContainer width="100%" height={300}>
          <ComposedChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis
              dataKey="time"
              type="number"
              domain={["dataMin", "dataMax"]}
              tickFormatter={(v) => formatDateTime(new Date(v).toISOString(), i18n.language)}
              tick={{ fontSize: 11 }}
            />
            <YAxis
              domain={[schedule.min_color_temp - 200, schedule.max_color_temp + 200]}
              tickFormatter={(v) => `${v}K`}
            />
            <Tooltip
              labelFormatter={(v) => formatDateTime(new Date(v as number).toISOString(), i18n.language)}
              formatter={(value) => [`${value} K`, t("schedule.colorTemp")]}
            />
            <Line
              type="monotone"
              dataKey="temp"
              stroke="#339af0"
              dot={false}
              strokeWidth={2}
            />
          </ComposedChart>
        </ResponsiveContainer>
      </Group>
    </Stack>
  );
}
