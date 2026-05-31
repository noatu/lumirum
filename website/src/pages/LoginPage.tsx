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
import { useLogin } from "../hooks/useAuth";
import { apiErrorMessage } from "../api/errors";

const schema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

type FormValues = z.infer<typeof schema>;

export function LoginPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const loginMutation = useLogin();

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<FormValues>({ resolver: zodResolver(schema) });

  async function onSubmit(values: FormValues) {
    try {
      await loginMutation.mutateAsync({ username: values.username, password: values.password });
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
          {t("auth.login")}
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
              mb="lg"
              error={errors.password?.message}
              {...register("password")}
            />
            <Button type="submit" fullWidth loading={isSubmitting}>
              {t("auth.loginButton")}
            </Button>
          </form>
        </Paper>

        <Text ta="center" mt="md" size="sm">
          {t("auth.noAccount")}{" "}
          <Anchor component={Link} to="/register">
            {t("auth.register")}
          </Anchor>
        </Text>
      </Box>
    </Center>
  );
}
