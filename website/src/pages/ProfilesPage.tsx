import { Badge, Button, Group, Loader, Stack, Table, Text, Title } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { listProfiles } from "../api/profiles";
import { queryKeys } from "../lib/queryKeys";
import { useAuthStore } from "../store/auth";

export function ProfilesPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);

  const { data: profiles, isLoading } = useQuery({
    queryKey: queryKeys.profiles(),
    queryFn: listProfiles,
  });

  if (isLoading) return <Loader />;

  return (
    <Stack>
      <Group justify="space-between">
        <Title order={2}>{t("profiles.title")}</Title>
        <Button onClick={() => navigate("/profiles/new")}>{t("profiles.createProfile")}</Button>
      </Group>

      {!profiles?.length ? (
        <Text c="dimmed">{t("profiles.noProfiles")}</Text>
      ) : (
        <Table highlightOnHover>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>{t("profiles.name")}</Table.Th>
              <Table.Th>{t("profiles.owner")}</Table.Th>
              <Table.Th>{t("profiles.timezone")}</Table.Th>
              <Table.Th></Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {profiles.map((profile) => (
              <Table.Tr
                key={profile.id}
                style={{ cursor: "pointer" }}
                onClick={() => navigate(`/profiles/${profile.id}`)}
              >
                <Table.Td>{profile.name}</Table.Td>
                <Table.Td>
                  {profile.owner_id === user?.id ? t("profiles.own") : `#${profile.owner_id}`}
                </Table.Td>
                <Table.Td>{profile.timezone}</Table.Td>
                <Table.Td>
                  {profile.is_shared && (
                    <Badge color="teal">{t("profiles.shared")}</Badge>
                  )}
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      )}
    </Stack>
  );
}
