import {
  Anchor,
  Box,
  Button,
  Center,
  Paper,
  PasswordInput,
  Text,
  TextInput,
  Title,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Link, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useRegister } from "../hooks/useAuth";
import { apiErrorMessage } from "../api/errors";

const schema = z
  .object({
    username: z.string().min(3).max(25),
    password: z.string().min(8),
    confirmPassword: z.string(),
  })
  .refine((d) => d.password === d.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

type FormValues = z.infer<typeof schema>;

export function RegisterPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const registerMutation = useRegister();

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<FormValues>({ resolver: zodResolver(schema) });

  async function onSubmit(values: FormValues) {
    try {
      await registerMutation.mutateAsync({
        username: values.username,
        password: values.password,
      });
      navigate("/devices", { replace: true });
    } catch (err) {
      notifications.show({
        title: t("common.error"),
        message: apiErrorMessage(err),
        color: "red",
      });
    }
  }

  return (
    <Center mih="100vh">
      <Box w={360}>
        <Title order={2} mb="lg" ta="center">
          {t("auth.register")}
        </Title>

        <Paper withBorder shadow="sm" p="xl" radius="md">
          <form onSubmit={handleSubmit(onSubmit)}>
            <TextInput
              label={t("auth.username")}
              mb="sm"
              error={errors.username?.message}
              {...register("username")}
            />
            <PasswordInput
              label={t("auth.password")}
              mb="sm"
              error={errors.password?.message}
              {...register("password")}
            />
            <PasswordInput
              label={t("auth.confirmPassword")}
              mb="lg"
              error={errors.confirmPassword?.message}
              {...register("confirmPassword")}
            />
            <Button type="submit" fullWidth loading={isSubmitting}>
              {t("auth.registerButton")}
            </Button>
          </form>
        </Paper>

        <Text ta="center" mt="md" size="sm">
          {t("auth.hasAccount")}{" "}
          <Anchor component={Link} to="/login">
            {t("auth.login")}
          </Anchor>
        </Text>
      </Box>
    </Center>
  );
}
