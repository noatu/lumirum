import { apiClient } from "./client";
import type { Telemetry } from "../types";

export async function listTelemetry(start: string, end: string): Promise<Telemetry[]> {
  const res = await apiClient.get<Telemetry[]>("/telemetry", {
    params: { start, end },
  });
  return res.data;
}

export async function getTelemetry(
  deviceId: number,
  start: string,
  end: string
): Promise<Telemetry[]> {
  const res = await apiClient.get<Telemetry[]>(`/telemetry/device/${deviceId}`, {
    params: { start, end },
  });
  return res.data;
}

export async function deleteTelemetry(
  deviceId: number,
  start: string,
  end: string
): Promise<number> {
  const res = await apiClient.delete<number>(`/telemetry/device/${deviceId}`, {
    params: { start, end },
  });
  return res.data;
}
