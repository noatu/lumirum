import { useEffect } from "react";
import { BrowserRouter, Navigate, Route, Routes, useNavigate } from "react-router-dom";
import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { ModalsProvider } from "@mantine/modals";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { setOnUnauthorized } from "./api/client";
import { Layout } from "./components/Layout";
import { ProtectedRoute } from "./components/ProtectedRoute";
import { LoginPage } from "./pages/LoginPage";
import { RegisterPage } from "./pages/RegisterPage";
import { DevicesPage } from "./pages/DevicesPage";
import { DeviceDetailPage } from "./pages/DeviceDetailPage";
import { ProfilesPage } from "./pages/ProfilesPage";
import { ProfileDetailPage } from "./pages/ProfileDetailPage";
import { LightingSchedulePage } from "./pages/LightingSchedulePage";
import { TelemetryPage } from "./pages/TelemetryPage";
import { AccountPage } from "./pages/AccountPage";
import { AdminPage } from "./pages/AdminPage";
import { useAuthStore } from "./store/auth";
import { isAdmin } from "./types";

const queryClient = new QueryClient();

function AuthInterceptorSetup() {
  const navigate = useNavigate();

  useEffect(() => {
    setOnUnauthorized(() => {
      navigate("/login", { replace: true });
    });
  }, [navigate]);

  return null;
}

function AdminRoute({ children }: { children: React.ReactNode }) {
  const user = useAuthStore((s) => s.user);
  if (!user || !isAdmin(user.role)) {
    return <Navigate to="/devices" replace />;
  }
  return <>{children}</>;
}

function AppRoutes() {
  return (
    <>
      <AuthInterceptorSetup />
      <Routes>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/register" element={<RegisterPage />} />
        <Route element={<ProtectedRoute />}>
          <Route element={<Layout />}>
            <Route path="/devices" element={<DevicesPage />} />
            <Route path="/devices/new" element={<DeviceDetailPage />} />
            <Route path="/devices/:id" element={<DeviceDetailPage />} />
            <Route path="/telemetry/:deviceId" element={<TelemetryPage />} />
            <Route path="/profiles" element={<ProfilesPage />} />
            <Route path="/profiles/new" element={<ProfileDetailPage />} />
            <Route path="/profiles/:id" element={<ProfileDetailPage />} />
            <Route path="/profiles/:id/schedule" element={<LightingSchedulePage />} />
            <Route path="/account" element={<AccountPage />} />
            <Route
              path="/admin"
              element={
                <AdminRoute>
                  <AdminPage />
                </AdminRoute>
              }
            />
          </Route>
        </Route>
        <Route path="*" element={<Navigate to="/devices" replace />} />
      </Routes>
    </>
  );
}

export default function App() {
  return (
    <MantineProvider>
      <ModalsProvider>
        <Notifications />
        <QueryClientProvider client={queryClient}>
          <BrowserRouter>
            <AppRoutes />
          </BrowserRouter>
        </QueryClientProvider>
      </ModalsProvider>
    </MantineProvider>
  );
}
