import { AppShell, Burger, Button, Group, NavLink, Text, Title } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Link, Outlet, useLocation } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useAuthStore } from "../store/auth";
import { isAdmin } from "../types";

export function Layout() {
  const [opened, { toggle }] = useDisclosure();
  const { t, i18n } = useTranslation();
  const location = useLocation();
  const user = useAuthStore((s) => s.user);

  function switchLanguage(lang: string) {
    i18n.changeLanguage(lang);
    localStorage.setItem("lumirum-lang", lang);
  }

  const navLinks = [
    { to: "/devices", label: t("nav.devices") },
    { to: "/profiles", label: t("nav.profiles") },
    { to: "/account", label: t("nav.account") },
    ...(user && isAdmin(user.role)
      ? [{ to: "/admin", label: t("nav.admin") }]
      : []),
  ];

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{ width: 220, breakpoint: "sm", collapsed: { mobile: !opened } }}
      padding="md"
    >
      <AppShell.Header>
        <Group h="100%" px="md" justify="space-between">
          <Group>
            <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
            <Title order={4}>LumiRum</Title>
          </Group>
          <Group gap="xs">
            <Button
              size="xs"
              variant={i18n.language === "en" ? "filled" : "subtle"}
              onClick={() => switchLanguage("en")}
            >
              EN
            </Button>
            <Button
              size="xs"
              variant={i18n.language === "uk" ? "filled" : "subtle"}
              onClick={() => switchLanguage("uk")}
            >
              UA
            </Button>
          </Group>
        </Group>
      </AppShell.Header>

      <AppShell.Navbar p="md">
        {navLinks.map((link) => (
          <NavLink
            key={link.to}
            component={Link}
            to={link.to}
            label={<Text size="sm">{link.label}</Text>}
            active={location.pathname.startsWith(link.to)}
          />
        ))}
      </AppShell.Navbar>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}
