import { apiClient } from "./client";
import type {
  AuthResponse,
  LoginRequest,
  RegisterRequest,
  ChangeAccountRequest,
  DeleteRequest,
} from "../types";

export async function login(data: LoginRequest): Promise<AuthResponse> {
  const res = await apiClient.post<AuthResponse>("/auth/login", data);
  return res.data;
}

export async function register(data: RegisterRequest): Promise<AuthResponse> {
  const res = await apiClient.post<AuthResponse>("/auth/register", data);
  return res.data;
}

export async function getMe(): Promise<AuthResponse> {
  const res = await apiClient.get<AuthResponse>("/auth/me");
  return res.data;
}

export async function updateAccount(
  data: ChangeAccountRequest
): Promise<AuthResponse> {
  const res = await apiClient.patch<AuthResponse>("/auth/me", data);
  return res.data;
}

export async function deleteAccount(data: DeleteRequest): Promise<void> {
  await apiClient.delete("/auth/me", { data });
}
