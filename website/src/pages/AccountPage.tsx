import {
  Button,
  Divider,
  FileInput,
  Group,
  Modal,
  PasswordInput,
  Progress,
  Stack,
  Text,
  TextInput,
  Title,
} from "@mantine/core";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import dayjs from "dayjs";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { updateAccount, deleteAccount, register as registerUser } from "../api/auth";
import { listDevices, createDevice, updateDevice } from "../api/devices";
import { listProfiles, createProfile, updateProfile } from "../api/profiles";
import { apiErrorMessage } from "../api/errors";
import { queryKeys } from "../lib/queryKeys";
import { useAuthStore } from "../store/auth";
import { isOwner } from "../types";
import type { CreateProfileRequest } from "../types";

const changeSchema = z
  .object({
    current_password: z.string().min(1),
    new_username: z.string().optional(),
    new_password: z.string().optional(),
    confirm_password: z.string().optional(),
  })
  .refine(
    (d) => !d.new_password || d.new_password === d.confirm_password,
    { message: "account.passwordsDoNotMatch", path: ["confirm_password"] }
  );

type ChangeValues = z.infer<typeof changeSchema>;

const deleteSchema = z.object({ password: z.string().min(1) });
type DeleteValues = z.infer<typeof deleteSchema>;

const subUserSchema = z.object({
  username: z.string().min(3),
  password: z.string().min(8),
});
type SubUserValues = z.infer<typeof subUserSchema>;

interface ImportData {
  version: number;
  exported_at: string;
  devices: Array<{
    name: string;
    profile_name: string | null;
    is_public: boolean;
  }>;
  profiles: Array<{
    name: string;
    timezone: string;
    sleep_start: string;
    sleep_end: string;
    min_color_temp: number;
    max_color_temp: number;
    motion_timeout_seconds: number;
    latitude: number | null;
    longitude: number | null;
    is_shared: boolean;
    night_mode_enabled: boolean;
  }>;
}

const importSchema = z.object({
  version: z.literal(1),
  exported_at: z.string(),
  devices: z.array(
    z.object({
      name: z.string(),
      profile_name: z.string().nullable().optional(),
      is_public: z.boolean(),
    })
  ),
  profiles: z.array(
    z.object({
      name: z.string(),
      timezone: z.string(),
      sleep_start: z.string(),
      sleep_end: z.string(),
      min_color_temp: z.number(),
      max_color_temp: z.number(),
      motion_timeout_seconds: z.number(),
      latitude: z.number().nullable().optional(),
      longitude: z.number().nullable().optional(),
      is_shared: z.boolean(),
      night_mode_enabled: z.boolean(),
    })
  ),
});

export function AccountPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const user = useAuthStore((s) => s.user);
  const setUser = useAuthStore((s) => s.setUser);
  const logout = useAuthStore((s) => s.logout);

  const [subUserModalOpen, setSubUserModalOpen] = useState(false);
  const [importData, setImportData] = useState<ImportData | null>(null);
  const [importProgress, setImportProgress] = useState<number | null>(null);
  const [importResult, setImportResult] = useState<string | null>(null);

  const canCreateSubUsers = user && isOwner(user.role);

  const {
    register: regChange,
    handleSubmit: handleChange,
    formState: { errors: changeErrors },
  } = useForm<ChangeValues>({
    resolver: zodResolver(changeSchema),
  });

  const {
    register: regDelete,
    handleSubmit: handleDeleteForm,
    formState: { errors: deleteErrors },
  } = useForm<DeleteValues>({
    resolver: zodResolver(deleteSchema),
  });

  const {
    register: regSubUser,
    handleSubmit: handleSubUser,
    reset: resetSubUser,
    formState: { errors: subUserErrors },
  } = useForm<SubUserValues>({
    resolver: zodResolver(subUserSchema),
  });

  const changeMutation = useMutation({
    mutationFn: (values: ChangeValues) =>
      updateAccount({
        password: values.current_password,
        new_username: values.new_username || null,
        new_password: values.new_password || null,
      }),
    onSuccess: (res) => {
      setUser(res);
      notifications.show({ color: "green", message: t("account.saveSuccess") });
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (values: DeleteValues) => deleteAccount({ password: values.password }),
    onSuccess: () => {
      logout();
      navigate("/login");
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });


  const createSubUserMutation = useMutation({
    mutationFn: (values: SubUserValues) =>
      registerUser({ username: values.username, password: values.password }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.users() });
      notifications.show({ color: "green", message: t("account.subUserCreated") });
      setSubUserModalOpen(false);
      resetSubUser();
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  function confirmDeleteAccount(values: DeleteValues) {
    modals.openConfirmModal({
      title: t("account.deleteAccount"),
      children: t("account.deleteConfirm"),
      labels: { confirm: t("common.delete"), cancel: t("common.cancel") },
      confirmProps: { color: "red" },
      onConfirm: () => deleteMutation.mutate(values),
    });
  }

  async function handleExport() {
    const [devices, profiles] = await Promise.all([listDevices(), listProfiles()]);

    const profileMap = new Map(profiles.map((p) => [p.id, p.name]));

    const exportObj = {
      version: 1,
      exported_at: new Date().toISOString(),
      devices: devices.map((d) => ({
        name: d.name,
        profile_name: d.profile_id != null ? (profileMap.get(d.profile_id) ?? null) : null,
        is_public: d.is_public,
      })),
      profiles: profiles.map((p) => ({
        name: p.name,
        timezone: p.timezone,
        sleep_start: p.sleep_start,
        sleep_end: p.sleep_end,
        min_color_temp: p.min_color_temp,
        max_color_temp: p.max_color_temp,
        motion_timeout_seconds: p.motion_timeout_seconds,
        latitude: p.latitude,
        longitude: p.longitude,
        is_shared: p.is_shared,
        night_mode_enabled: p.night_mode_enabled,
      })),
    };

    const blob = new Blob([JSON.stringify(exportObj, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `lumirum-export-${dayjs().format("YYYY-MM-DD")}.json`;
    a.click();
    URL.revokeObjectURL(url);
    notifications.show({ color: "green", message: t("account.exportSuccess") });
  }

  function handleFileSelect(file: File | null) {
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const json = JSON.parse(e.target?.result as string);
        const parsed = importSchema.safeParse(json);
        if (!parsed.success) {
          notifications.show({ color: "red", message: t("account.invalidFile") });
          return;
        }
        setImportData(parsed.data as ImportData);
        setImportResult(null);
      } catch {
        notifications.show({ color: "red", message: t("account.invalidFile") });
      }
    };
    reader.readAsText(file);
  }

  async function executeImport() {
    if (!importData) return;

    setImportProgress(0);
    setImportResult(null);

    let created = 0;
    let updated = 0;
    let failed = 0;

    const existingProfiles = await listProfiles();
    const profileNameToId = new Map(existingProfiles.map((p) => [p.name, p.id]));

    const totalSteps = importData.profiles.length + importData.devices.length;
    let step = 0;

    for (const prof of importData.profiles) {
      const payload: CreateProfileRequest = {
        name: prof.name,
        timezone: prof.timezone,
        sleep_start: prof.sleep_start,
        sleep_end: prof.sleep_end,
        min_color_temp: prof.min_color_temp,
        max_color_temp: prof.max_color_temp,
        motion_timeout_seconds: prof.motion_timeout_seconds,
        latitude: prof.latitude ?? null,
        longitude: prof.longitude ?? null,
        is_shared: prof.is_shared,
        night_mode_enabled: prof.night_mode_enabled,
      };

      try {
        const existingId = profileNameToId.get(prof.name);
        if (existingId != null) {
          await updateProfile(existingId, payload);
          updated++;
        } else {
          const newProf = await createProfile(payload);
          profileNameToId.set(newProf.name, newProf.id);
          created++;
        }
      } catch {
        failed++;
      }

      step++;
      setImportProgress(Math.round((step / totalSteps) * 100));
    }

    const existingDevices = await listDevices();
    const deviceNameToId = new Map(existingDevices.map((d) => [d.name, d.id]));

    for (const dev of importData.devices) {
      const resolvedProfileId =
        dev.profile_name != null ? (profileNameToId.get(dev.profile_name) ?? null) : null;

      const payload = {
        name: dev.name,
        is_public: dev.is_public,
        profile_id: resolvedProfileId,
      };

      try {
        const existingId = deviceNameToId.get(dev.name);
        if (existingId != null) {
          await updateDevice(existingId, payload);
          updated++;
        } else {
          await createDevice(payload);
          created++;
        }
      } catch {
        failed++;
      }

      step++;
      setImportProgress(Math.round((step / totalSteps) * 100));
    }

    queryClient.invalidateQueries({ queryKey: queryKeys.devices() });
    queryClient.invalidateQueries({ queryKey: queryKeys.profiles() });

    setImportProgress(null);
    setImportData(null);
    setImportResult(t("account.importSuccess", { created, updated, failed }));
  }

  return (
    <Stack maw={560}>
      <Group justify="space-between">
        <Title order={2}>{t("account.title")}</Title>
        <Button variant="subtle" color="red" onClick={() => { logout(); navigate("/login"); }}>
          {t("auth.logout")}
        </Button>
      </Group>

      <Title order={4}>{t("account.changeInfo")}</Title>
      <form onSubmit={handleChange((v) => changeMutation.mutate(v))}>
        <Stack>
          <PasswordInput
            label={t("account.currentPassword")}
            error={changeErrors.current_password?.message}
            {...regChange("current_password")}
          />
          <TextInput
            label={t("account.newUsername")}
            error={changeErrors.new_username?.message}
            {...regChange("new_username")}
          />
          <PasswordInput
            label={t("account.newPassword")}
            error={changeErrors.new_password?.message}
            {...regChange("new_password")}
          />
          <PasswordInput
            label={t("account.confirmNewPassword")}
            error={changeErrors.confirm_password?.message}
            {...regChange("confirm_password")}
          />
          <Button type="submit" loading={changeMutation.isPending}>
            {t("account.saveChanges")}
          </Button>
        </Stack>
      </form>

      <Divider />

      <Title order={4}>{t("account.deleteAccount")}</Title>
      <Text size="sm" c="dimmed">{t("account.deleteAccountDesc")}</Text>
      <form onSubmit={handleDeleteForm(confirmDeleteAccount)}>
        <Stack>
          <PasswordInput
            label={t("account.currentPassword")}
            error={deleteErrors.password?.message}
            {...regDelete("password")}
          />
          <Button type="submit" color="red" loading={deleteMutation.isPending}>
            {t("account.deleteAccount")}
          </Button>
        </Stack>
      </form>

      {canCreateSubUsers && (
        <>
          <Divider />
          <Title order={4}>{t("account.subUsers")}</Title>
          <Button variant="outline" onClick={() => setSubUserModalOpen(true)}>
            {t("account.addSubUser")}
          </Button>

          <Modal
            opened={subUserModalOpen}
            onClose={() => setSubUserModalOpen(false)}
            title={t("account.addSubUser")}
          >
            <form onSubmit={handleSubUser((v) => createSubUserMutation.mutate(v))}>
              <Stack>
                <TextInput
                  label={t("auth.username")}
                  error={subUserErrors.username?.message}
                  {...regSubUser("username")}
                />
                <PasswordInput
                  label={t("auth.password")}
                  error={subUserErrors.password?.message}
                  {...regSubUser("password")}
                />
                <Button type="submit" loading={createSubUserMutation.isPending}>
                  {t("common.create")}
                </Button>
              </Stack>
            </form>
          </Modal>
        </>
      )}

      <Divider />

      <Title order={4}>{t("account.dataExport")}</Title>

      <Button variant="outline" onClick={handleExport}>
        {t("account.exportJson")}
      </Button>

      <FileInput
        label={t("account.importFile")}
        accept=".json"
        onChange={handleFileSelect}
      />

      {importData && (
        <Stack gap="xs">
          <Text size="sm">
            {t("account.importPreview", {
              profiles: importData.profiles.length,
              devices: importData.devices.length,
            })}
          </Text>
          <Button onClick={executeImport} loading={importProgress !== null}>
            {t("account.importBtn")}
          </Button>
        </Stack>
      )}

      {importProgress !== null && (
        <Progress value={importProgress} animated />
      )}

      {importResult && (
        <Text size="sm" c="green">{importResult}</Text>
      )}
    </Stack>
  );
}
