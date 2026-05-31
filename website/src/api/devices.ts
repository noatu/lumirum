import { apiClient } from "./client";
import type { Device, CreateDeviceRequest, LightingSchedule } from "../types";

export async function listDevices(): Promise<Device[]> {
  const res = await apiClient.get<Device[]>("/devices");
  return res.data;
}

export async function getDevice(id: number): Promise<Device> {
  const res = await apiClient.get<Device>(`/devices/${id}`);
  return res.data;
}

export async function createDevice(data: CreateDeviceRequest): Promise<Device> {
  const res = await apiClient.post<Device>("/devices", data);
  return res.data;
}

export async function updateDevice(
  id: number,
  data: CreateDeviceRequest
): Promise<Device> {
  const res = await apiClient.put<Device>(`/devices/${id}`, data);
  return res.data;
}

export async function deleteDevice(id: number): Promise<void> {
  await apiClient.delete(`/devices/${id}`);
}

export async function regenerateKey(id: number): Promise<Device> {
  const res = await apiClient.post<Device>(`/devices/${id}/key`);
  return res.data;
}

export async function getCircadian(): Promise<LightingSchedule | null> {
  const res = await apiClient.get<LightingSchedule | null>("/devices/circadian");
  return res.data;
}
