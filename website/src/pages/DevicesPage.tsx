import { Badge, Button, Group, Loader, Stack, Table, Text, Title } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { listDevices } from "../api/devices";
import { queryKeys } from "../lib/queryKeys";
import { formatRelative } from "../lib/dates";

export function DevicesPage() {
  const { t, i18n } = useTranslation();
  const navigate = useNavigate();

  const { data: devices, isLoading } = useQuery({
    queryKey: queryKeys.devices(),
    queryFn: listDevices,
  });

  if (isLoading) {
    return <Loader />;
  }

  return (
    <Stack>
      <Group justify="space-between">
        <Title order={2}>{t("devices.title")}</Title>
        <Button onClick={() => navigate("/devices/new")}>{t("devices.addDevice")}</Button>
      </Group>

      {!devices?.length ? (
        <Text c="dimmed">{t("devices.noDevices")}</Text>
      ) : (
        <Table highlightOnHover>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>{t("devices.name")}</Table.Th>
              <Table.Th>{t("devices.lastSeen")}</Table.Th>
              <Table.Th>{t("devices.firmwareVersion")}</Table.Th>
              <Table.Th>{t("devices.visibility")}</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {devices.map((device) => (
              <Table.Tr
                key={device.id}
                style={{ cursor: "pointer" }}
                onClick={() => navigate(`/devices/${device.id}`)}
              >
                <Table.Td>{device.name}</Table.Td>
                <Table.Td>
                  {device.last_seen
                    ? formatRelative(device.last_seen, i18n.language)
                    : t("devices.never")}
                </Table.Td>
                <Table.Td>{device.firmware_version ?? t("common.notAvailable")}</Table.Td>
                <Table.Td>
                  <Badge color={device.is_public ? "blue" : "gray"}>
                    {device.is_public ? t("devices.public") : t("devices.private")}
                  </Badge>
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      )}
    </Stack>
  );
}
