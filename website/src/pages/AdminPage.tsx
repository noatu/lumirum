import {
  Button,
  Code,
  Group,
  Loader,
  SimpleGrid,
  Stack,
  Table,
  Text,
  TextInput,
  Title,
} from "@mantine/core";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { health, stats } from "../api/admin";
import { deleteUser, listUsers } from "../api/users";
import { apiErrorMessage } from "../api/errors";
import { queryKeys } from "../lib/queryKeys";
import { formatDateTime } from "../lib/dates";

function roleLabel(role: unknown): string {
  if (role === "admin") return "admin";
  if (role === "owner") return "owner";
  if (typeof role === "object" && role !== null && "user" in role) {
    return `sub-user of #${(role as { user: number }).user}`;
  }
  return String(role);
}

export function AdminPage() {
  const { t, i18n } = useTranslation();
  const queryClient = useQueryClient();
  const [search, setSearch] = useState("");

  const { data: healthData } = useQuery({
    queryKey: queryKeys.health(),
    queryFn: health,
  });

  const { data: statsData } = useQuery({
    queryKey: queryKeys.stats(),
    queryFn: stats,
  });

  const { data: users, isLoading: usersLoading } = useQuery({
    queryKey: queryKeys.users(),
    queryFn: listUsers,
  });

  const deleteMutation = useMutation({
    mutationFn: (id: number) => deleteUser(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.users() });
      queryClient.invalidateQueries({ queryKey: queryKeys.stats() });
      notifications.show({ color: "green", message: t("admin.deleteSuccess") });
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  function confirmDelete(id: number) {
    modals.openConfirmModal({
      title: t("admin.deleteUser"),
      children: t("admin.deleteUserConfirm"),
      labels: { confirm: t("common.delete"), cancel: t("common.cancel") },
      confirmProps: { color: "red" },
      onConfirm: () => deleteMutation.mutate(id),
    });
  }

  const filtered = (users ?? []).filter((u) =>
    u.username.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <Stack>
      <Title order={2}>{t("admin.title")}</Title>

      <Title order={4}>{t("admin.health")}</Title>
      {healthData ? (
        <Code block>{JSON.stringify(healthData, null, 2)}</Code>
      ) : (
        <Loader size="sm" />
      )}

      <Title order={4}>{t("admin.stats")}</Title>
      {statsData ? (
        <SimpleGrid cols={{ base: 2, sm: 4 }}>
          {[
            { label: t("admin.userCount"), value: statsData.users },
            { label: t("admin.deviceCount"), value: statsData.devices },
            { label: t("admin.profileCount"), value: statsData.profiles },
            { label: t("admin.telemetryCount"), value: statsData.telemetry },
          ].map((stat) => (
            <Stack key={stat.label} gap={2} p="md" style={{ border: "1px solid var(--mantine-color-default-border)", borderRadius: "var(--mantine-radius-default)" }}>
              <Text size="xs" c="dimmed">{stat.label}</Text>
              <Text size="xl" fw={700}>{stat.value}</Text>
            </Stack>
          ))}
        </SimpleGrid>
      ) : (
        <Loader size="sm" />
      )}

      <Title order={4}>{t("admin.users")}</Title>
      <TextInput
        placeholder={t("admin.searchUsers")}
        value={search}
        onChange={(e) => setSearch(e.currentTarget.value)}
        maw={300}
      />

      {usersLoading ? (
        <Loader />
      ) : (
        <Table striped withTableBorder>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>{t("admin.username")}</Table.Th>
              <Table.Th>{t("admin.role")}</Table.Th>
              <Table.Th>{t("admin.createdAt")}</Table.Th>
              <Table.Th></Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {filtered.map((user) => (
              <Table.Tr key={user.id}>
                <Table.Td>{user.username}</Table.Td>
                <Table.Td>{roleLabel(user.role)}</Table.Td>
                <Table.Td>{formatDateTime(user.created_at, i18n.language)}</Table.Td>
                <Table.Td>
                  <Group gap="xs">
                    <Button
                      size="xs"
                      color="red"
                      variant="outline"
                      onClick={() => confirmDelete(user.id)}
                    >
                      {t("common.delete")}
                    </Button>
                  </Group>
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      )}
    </Stack>
  );
}
