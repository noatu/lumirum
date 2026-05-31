import {
  Button,
  Group,
  Loader,
  SegmentedControl,
  Stack,
  Table,
  Text,
  Title,
} from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import dayjs from "dayjs";
import { useState } from "react";
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
import { getDevice } from "../api/devices";
import { deleteTelemetry, getTelemetry } from "../api/telemetry";
import { apiErrorMessage } from "../api/errors";
import { queryKeys } from "../lib/queryKeys";
import { formatDateTime } from "../lib/dates";
import type { Telemetry } from "../types";

function nullDash(v: number | boolean | null | undefined): string {
  if (v === null || v === undefined) return "—";
  return String(v);
}

export function TelemetryPage() {
  const { t, i18n } = useTranslation();
  const { deviceId } = useParams<{ deviceId: string }>();
  const id = parseInt(deviceId!, 10);
  const queryClient = useQueryClient();

  const [dateRange, setDateRange] = useState<[Date | null, Date | null]>([
    dayjs().subtract(7, "day").toDate(),
    dayjs().toDate(),
  ]);
  const [view, setView] = useState<"chart" | "table">("chart");

  const startStr = dateRange[0] ? dayjs(dateRange[0]).toISOString() : "";
  const endStr = dateRange[1] ? dayjs(dateRange[1]).toISOString() : "";

  const { data: device } = useQuery({
    queryKey: queryKeys.device(id),
    queryFn: () => getDevice(id),
  });

  const { data: telemetry, isLoading } = useQuery({
    queryKey: queryKeys.telemetry(id, startStr, endStr),
    queryFn: () => getTelemetry(id, startStr, endStr),
    enabled: !!startStr && !!endStr,
  });

  const deleteMutation = useMutation({
    mutationFn: () => deleteTelemetry(id, startStr, endStr),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["telemetry", id] });
      notifications.show({ color: "green", message: t("telemetry.deleteSuccess") });
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  function confirmDelete() {
    modals.openConfirmModal({
      title: t("telemetry.deleteRange"),
      children: t("telemetry.deleteConfirm"),
      labels: { confirm: t("common.delete"), cancel: t("common.cancel") },
      confirmProps: { color: "red" },
      onConfirm: () => deleteMutation.mutate(),
    });
  }

  const chartData = (telemetry ?? []).map((row: Telemetry) => ({
    time: new Date(row.created_at).getTime(),
    brightness: row.brightness,
    color_temp: row.color_temp,
    ambient_light: row.ambient_light,
  }));

  const hasData = (key: keyof typeof chartData[0]) =>
    chartData.some((d) => d[key] != null);

  return (
    <Stack>
      <Title order={2}>
        {t("telemetry.title")} {device ? `— ${device.name}` : ""}
      </Title>

      <Group>
        <DatePickerInput
          type="range"
          label={t("telemetry.dateRange")}
          value={dateRange}
          onChange={setDateRange}
          locale={i18n.language}
        />
        <SegmentedControl
          mt="auto"
          value={view}
          onChange={(v) => setView(v as "chart" | "table")}
          data={[
            { value: "chart", label: t("telemetry.chartView") },
            { value: "table", label: t("telemetry.tableView") },
          ]}
        />
        <Button mt="auto" color="red" variant="outline" onClick={confirmDelete} loading={deleteMutation.isPending}>
          {t("telemetry.deleteRange")}
        </Button>
      </Group>

      {isLoading && <Loader />}

      {!isLoading && telemetry?.length === 0 && (
        <Text c="dimmed">{t("telemetry.noData")}</Text>
      )}

      {!isLoading && telemetry && telemetry.length > 0 && view === "chart" && (
        <ResponsiveContainer width="100%" height={350}>
          <ComposedChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis
              dataKey="time"
              type="number"
              domain={["dataMin", "dataMax"]}
              tickFormatter={(v) => formatDateTime(new Date(v).toISOString(), i18n.language)}
              tick={{ fontSize: 10 }}
            />
            {hasData("brightness") && (
              <YAxis yAxisId="brightness" domain={[0, 100]} orientation="left" tickFormatter={(v) => `${v}%`} />
            )}
            {hasData("color_temp") && (
              <YAxis yAxisId="color_temp" domain={[1800, 10000]} orientation="right" tickFormatter={(v) => `${v}K`} />
            )}
            {hasData("ambient_light") && (
              <YAxis yAxisId="ambient_light" orientation="right" />
            )}
            <Tooltip
              labelFormatter={(v) => formatDateTime(new Date(v as number).toISOString(), i18n.language)}
            />
            {hasData("brightness") && (
              <Line yAxisId="brightness" type="monotone" dataKey="brightness" stroke="#40c057" dot={false} name={t("telemetry.brightness")} />
            )}
            {hasData("color_temp") && (
              <Line yAxisId="color_temp" type="monotone" dataKey="color_temp" stroke="#339af0" dot={false} name={t("telemetry.colorTemp")} />
            )}
            {hasData("ambient_light") && (
              <Line yAxisId="ambient_light" type="monotone" dataKey="ambient_light" stroke="#fab005" dot={false} name={t("telemetry.ambientLight")} />
            )}
          </ComposedChart>
        </ResponsiveContainer>
      )}

      {!isLoading && telemetry && telemetry.length > 0 && view === "table" && (
        <Table striped withTableBorder>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>{t("telemetry.time")}</Table.Th>
              <Table.Th>{t("telemetry.eventType")}</Table.Th>
              <Table.Th>{t("telemetry.motionDetected")}</Table.Th>
              <Table.Th>{t("telemetry.lightIsOn")}</Table.Th>
              <Table.Th>{t("telemetry.brightness")}</Table.Th>
              <Table.Th>{t("telemetry.colorTemp")}</Table.Th>
              <Table.Th>{t("telemetry.ambientLight")}</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {telemetry.map((row) => (
              <Table.Tr key={row.id}>
                <Table.Td>{formatDateTime(row.created_at, i18n.language)}</Table.Td>
                <Table.Td>{row.event_type}</Table.Td>
                <Table.Td>{nullDash(row.motion_detected)}</Table.Td>
                <Table.Td>{nullDash(row.light_is_on)}</Table.Td>
                <Table.Td>{nullDash(row.brightness)}</Table.Td>
                <Table.Td>{nullDash(row.color_temp)}</Table.Td>
                <Table.Td>{nullDash(row.ambient_light)}</Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      )}
    </Stack>
  );
}
