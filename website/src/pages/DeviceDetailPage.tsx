import {
  Button,
  Group,
  Loader,
  PasswordInput,
  Select,
  Stack,
  Switch,
  TextInput,
  Title,
} from "@mantine/core";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { Controller, useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import {
  createDevice,
  deleteDevice,
  getDevice,
  regenerateKey,
  updateDevice,
} from "../api/devices";
import { listProfiles } from "../api/profiles";
import { apiErrorMessage } from "../api/errors";
import { queryKeys } from "../lib/queryKeys";

const schema = z.object({
  name: z.string().min(1),
  profile_id: z.string().nullable().optional(),
  is_public: z.boolean(),
});

type FormValues = z.infer<typeof schema>;

export function DeviceDetailPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const isNew = !id;
  const deviceId = id ? parseInt(id, 10) : null;
  const queryClient = useQueryClient();

  const { data: device, isLoading: deviceLoading } = useQuery({
    queryKey: queryKeys.device(deviceId!),
    queryFn: () => getDevice(deviceId!),
    enabled: !isNew,
  });

  const { data: profiles } = useQuery({
    queryKey: queryKeys.profiles(),
    queryFn: listProfiles,
  });

  const {
    register,
    handleSubmit,
    control,
    reset,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: { name: "", profile_id: null, is_public: false },
  });

  useEffect(() => {
    if (device) {
      reset({
        name: device.name,
        profile_id: device.profile_id != null ? String(device.profile_id) : null,
        is_public: device.is_public,
      });
    }
  }, [device, reset]);

  const saveMutation = useMutation({
    mutationFn: (values: FormValues) => {
      const payload = {
        name: values.name,
        is_public: values.is_public,
        profile_id: values.profile_id ? parseInt(values.profile_id, 10) : null,
      };
      return isNew ? createDevice(payload) : updateDevice(deviceId!, payload);
    },
    onSuccess: (saved) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.devices() });
      notifications.show({ color: "green", message: t("devices.saveSuccess") });
      if (isNew) navigate(`/devices/${saved.id}`);
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: () => deleteDevice(deviceId!),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.devices() });
      notifications.show({ color: "green", message: t("devices.deleteSuccess") });
      navigate("/devices");
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  const regenMutation = useMutation({
    mutationFn: () => regenerateKey(deviceId!),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.device(deviceId!) });
      notifications.show({ color: "green", message: t("devices.keyRegenerated") });
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  function confirmDelete() {
    modals.openConfirmModal({
      title: t("devices.deleteDevice"),
      children: t("devices.deleteConfirm"),
      labels: { confirm: t("common.delete"), cancel: t("common.cancel") },
      confirmProps: { color: "red" },
      onConfirm: () => deleteMutation.mutate(),
    });
  }

  function confirmRegen() {
    modals.openConfirmModal({
      title: t("devices.regenerateKey"),
      children: t("devices.regenerateKeyConfirm"),
      labels: { confirm: t("common.confirm"), cancel: t("common.cancel") },
      confirmProps: { color: "orange" },
      onConfirm: () => regenMutation.mutate(),
    });
  }

  if (!isNew && deviceLoading) return <Loader />;

  const profileOptions =
    profiles?.map((p) => ({ value: String(p.id), label: p.name })) ?? [];

  return (
    <Stack maw={480}>
      <Title order={2}>
        {isNew ? t("devices.createTitle") : t("devices.editTitle")}
      </Title>

      <form onSubmit={handleSubmit((v) => saveMutation.mutate(v))}>
        <Stack>
          <TextInput
            label={t("devices.name")}
            error={errors.name?.message}
            {...register("name")}
          />

          <Controller
            name="profile_id"
            control={control}
            render={({ field }) => (
              <Select
                label={t("devices.profile")}
                data={profileOptions}
                value={field.value ?? null}
                onChange={field.onChange}
                clearable
                placeholder={t("devices.noProfile")}
              />
            )}
          />

          <Controller
            name="is_public"
            control={control}
            render={({ field }) => (
              <Switch
                label={t("devices.isPublic")}
                checked={field.value}
                onChange={(e) => field.onChange(e.currentTarget.checked)}
              />
            )}
          />

          {!isNew && device && (
            <PasswordInput
              label={t("devices.secretKey")}
              value={device.secret_key}
              readOnly
              rightSection={
                <Button
                  size="xs"
                  variant="subtle"
                  onClick={() => {
                    navigator.clipboard.writeText(device.secret_key);
                    notifications.show({ message: t("common.copied") });
                  }}
                >
                  {t("common.copy")}
                </Button>
              }
              rightSectionWidth={80}
            />
          )}

          <Group>
            <Button type="submit" loading={saveMutation.isPending}>
              {t("common.save")}
            </Button>

            {!isNew && (
              <>
                <Button variant="outline" onClick={confirmRegen} loading={regenMutation.isPending}>
                  {t("devices.regenerateKey")}
                </Button>
                <Button
                  variant="outline"
                  onClick={() => navigate(`/telemetry/${deviceId}`)}
                >
                  {t("devices.viewTelemetry")}
                </Button>
                <Button color="red" variant="outline" onClick={confirmDelete}>
                  {t("devices.deleteDevice")}
                </Button>
              </>
            )}
          </Group>
        </Stack>
      </form>
    </Stack>
  );
}
