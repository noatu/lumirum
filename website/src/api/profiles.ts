import { apiClient } from "./client";
import type { Profile, CreateProfileRequest, LightingSchedule } from "../types";

export async function listProfiles(): Promise<Profile[]> {
  const res = await apiClient.get<Profile[]>("/profiles");
  return res.data;
}

export async function getProfile(id: number): Promise<Profile> {
  const res = await apiClient.get<Profile>(`/profiles/${id}`);
  return res.data;
}

export async function createProfile(
  data: CreateProfileRequest
): Promise<Profile> {
  const res = await apiClient.post<Profile>("/profiles", data);
  return res.data;
}

export async function updateProfile(
  id: number,
  data: CreateProfileRequest
): Promise<Profile> {
  const res = await apiClient.put<Profile>(`/profiles/${id}`, data);
  return res.data;
}

export async function deleteProfile(id: number): Promise<void> {
  await apiClient.delete(`/profiles/${id}`);
}

export async function getProfileSchedule(id: number): Promise<LightingSchedule> {
  const res = await apiClient.get<LightingSchedule>(`/profiles/circadian/${id}`);
  return res.data;
}
