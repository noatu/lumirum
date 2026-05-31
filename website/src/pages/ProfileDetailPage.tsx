import {
  Button,
  Group,
  Loader,
  NumberInput,
  Stack,
  Switch,
  TextInput,
  Title,
} from "@mantine/core";
import { TimeInput } from "@mantine/dates";
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
  createProfile,
  deleteProfile,
  getProfile,
  updateProfile,
} from "../api/profiles";
import { apiErrorMessage } from "../api/errors";
import { queryKeys } from "../lib/queryKeys";
import { useAuthStore } from "../store/auth";

const schema = z
  .object({
    name: z.string().min(1),
    timezone: z.string().min(1),
    sleep_start: z.string().min(1),
    sleep_end: z.string().min(1),
    min_color_temp: z.number().min(1800).max(10000),
    max_color_temp: z.number().min(1800).max(10000),
    motion_timeout_seconds: z.number().min(0),
    latitude: z.number().nullable().optional(),
    longitude: z.number().nullable().optional(),
    is_shared: z.boolean(),
    night_mode_enabled: z.boolean(),
  })
  .refine((d) => d.min_color_temp < d.max_color_temp, {
    message: "profiles.colorTempError",
    path: ["max_color_temp"],
  });

type FormValues = z.infer<typeof schema>;

const defaultValues: FormValues = {
  name: "",
  timezone: "UTC",
  sleep_start: "22:00",
  sleep_end: "07:00",
  min_color_temp: 2700,
  max_color_temp: 6500,
  motion_timeout_seconds: 60,
  latitude: null,
  longitude: null,
  is_shared: false,
  night_mode_enabled: false,
};

export function ProfileDetailPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const isNew = !id;
  const profileId = id ? parseInt(id, 10) : null;
  const queryClient = useQueryClient();
  const user = useAuthStore((s) => s.user);

  const { data: profile, isLoading } = useQuery({
    queryKey: queryKeys.profile(profileId!),
    queryFn: () => getProfile(profileId!),
    enabled: !isNew,
  });

  const {
    register,
    handleSubmit,
    control,
    reset,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues,
  });

  useEffect(() => {
    if (profile) {
      reset({
        name: profile.name,
        timezone: profile.timezone,
        sleep_start: profile.sleep_start,
        sleep_end: profile.sleep_end,
        min_color_temp: profile.min_color_temp,
        max_color_temp: profile.max_color_temp,
        motion_timeout_seconds: profile.motion_timeout_seconds,
        latitude: profile.latitude,
        longitude: profile.longitude,
        is_shared: profile.is_shared,
        night_mode_enabled: profile.night_mode_enabled,
      });
    }
  }, [profile, reset]);

  const isOwned = isNew || (profile && user && profile.owner_id === user.id);

  const saveMutation = useMutation({
    mutationFn: (values: FormValues) => {
      const payload = {
        name: values.name,
        timezone: values.timezone,
        sleep_start: values.sleep_start,
        sleep_end: values.sleep_end,
        min_color_temp: values.min_color_temp,
        max_color_temp: values.max_color_temp,
        motion_timeout_seconds: values.motion_timeout_seconds,
        latitude: values.latitude ?? null,
        longitude: values.longitude ?? null,
        is_shared: values.is_shared,
        night_mode_enabled: values.night_mode_enabled,
      };
      return isNew ? createProfile(payload) : updateProfile(profileId!, payload);
    },
    onSuccess: (saved) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.profiles() });
      notifications.show({ color: "green", message: t("profiles.saveSuccess") });
      if (isNew) navigate(`/profiles/${saved.id}`);
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: () => deleteProfile(profileId!),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.profiles() });
      notifications.show({ color: "green", message: t("profiles.deleteSuccess") });
      navigate("/profiles");
    },
    onError: (err) => {
      notifications.show({ color: "red", message: apiErrorMessage(err) });
    },
  });

  function confirmDelete() {
    modals.openConfirmModal({
      title: t("profiles.deleteProfile"),
      children: t("profiles.deleteConfirm"),
      labels: { confirm: t("common.delete"), cancel: t("common.cancel") },
      confirmProps: { color: "red" },
      onConfirm: () => deleteMutation.mutate(),
    });
  }

  if (!isNew && isLoading) return <Loader />;

  return (
    <Stack maw={520}>
      <Title order={2}>
        {isNew ? t("profiles.createTitle") : t("profiles.editTitle")}
      </Title>

      <form onSubmit={handleSubmit((v) => saveMutation.mutate(v))}>
        <Stack>
          <TextInput
            label={t("profiles.name")}
            error={errors.name?.message}
            disabled={!isOwned}
            {...register("name")}
          />

          <TextInput
            label={t("profiles.timezone")}
            error={errors.timezone?.message}
            disabled={!isOwned}
            {...register("timezone")}
          />

          <Group grow>
            <Controller
              name="sleep_start"
              control={control}
              render={({ field }) => (
                <TimeInput
                  label={t("profiles.sleepStart")}
                  value={field.value}
                  onChange={field.onChange}
                  error={errors.sleep_start?.message}
                  disabled={!isOwned}
                />
              )}
            />
            <Controller
              name="sleep_end"
              control={control}
              render={({ field }) => (
                <TimeInput
                  label={t("profiles.sleepEnd")}
                  value={field.value}
                  onChange={field.onChange}
                  error={errors.sleep_end?.message}
                  disabled={!isOwned}
                />
              )}
            />
          </Group>

          <Group grow>
            <Controller
              name="min_color_temp"
              control={control}
              render={({ field }) => (
                <NumberInput
                  label={t("profiles.minColorTemp")}
                  min={1800}
                  max={10000}
                  value={field.value}
                  onChange={(v) => field.onChange(Number(v))}
                  error={errors.min_color_temp?.message}
                  disabled={!isOwned}
                />
              )}
            />
            <Controller
              name="max_color_temp"
              control={control}
              render={({ field }) => (
                <NumberInput
                  label={t("profiles.maxColorTemp")}
                  min={1800}
                  max={10000}
                  value={field.value}
                  onChange={(v) => field.onChange(Number(v))}
                  error={errors.max_color_temp?.message}
                  disabled={!isOwned}
                />
              )}
            />
          </Group>

          <Controller
            name="motion_timeout_seconds"
            control={control}
            render={({ field }) => (
              <NumberInput
                label={t("profiles.motionTimeout")}
                min={0}
                value={field.value}
                onChange={(v) => field.onChange(Number(v))}
                error={errors.motion_timeout_seconds?.message}
                disabled={!isOwned}
              />
            )}
          />

          <Group grow>
            <Controller
              name="latitude"
              control={control}
              render={({ field }) => (
                <NumberInput
                  label={t("profiles.latitude")}
                  value={field.value ?? ""}
                  onChange={(v) => field.onChange(v === "" ? null : Number(v))}
                  decimalScale={6}
                  disabled={!isOwned}
                />
              )}
            />
            <Controller
              name="longitude"
              control={control}
              render={({ field }) => (
                <NumberInput
                  label={t("profiles.longitude")}
                  value={field.value ?? ""}
                  onChange={(v) => field.onChange(v === "" ? null : Number(v))}
                  decimalScale={6}
                  disabled={!isOwned}
                />
              )}
            />
          </Group>

          <Controller
            name="is_shared"
            control={control}
            render={({ field }) => (
              <Switch
                label={t("profiles.isShared")}
                checked={field.value}
                onChange={(e) => field.onChange(e.currentTarget.checked)}
                disabled={!isOwned}
              />
            )}
          />

          <Controller
            name="night_mode_enabled"
            control={control}
            render={({ field }) => (
              <Switch
                label={t("profiles.nightModeEnabled")}
                checked={field.value}
                onChange={(e) => field.onChange(e.currentTarget.checked)}
                disabled={!isOwned}
              />
            )}
          />

          <Group>
            {isOwned && (
              <Button type="submit" loading={saveMutation.isPending}>
                {t("common.save")}
              </Button>
            )}

            {!isNew && (
              <Button variant="outline" onClick={() => navigate(`/profiles/${profileId}/schedule`)}>
                {t("profiles.viewSchedule")}
              </Button>
            )}

            {!isNew && isOwned && (
              <Button color="red" variant="outline" onClick={confirmDelete}>
                {t("profiles.deleteProfile")}
              </Button>
            )}
          </Group>
        </Stack>
      </form>
    </Stack>
  );
}
