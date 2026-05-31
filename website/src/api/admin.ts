import { apiClient } from "./client";
import type { HealthResponse, Stats } from "../types";

export async function health(): Promise<HealthResponse> {
  const res = await apiClient.get<HealthResponse>("/health");
  return res.data;
}

export async function stats(): Promise<Stats> {
  const res = await apiClient.get<Stats>("/stats");
  return res.data;
}
