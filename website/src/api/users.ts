import { apiClient } from "./client";
import type { User } from "../types";

export async function listUsers(): Promise<User[]> {
  const res = await apiClient.get<User[]>("/users");
  return res.data;
}

export async function getUser(id: number): Promise<User> {
  const res = await apiClient.get<User>(`/users/${id}`);
  return res.data;
}

export async function deleteUser(id: number): Promise<void> {
  await apiClient.delete(`/users/${id}`);
}
