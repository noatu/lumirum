export const queryKeys = {
  devices: () => ["devices"] as const,
  device: (id: number) => ["devices", id] as const,
  profiles: () => ["profiles"] as const,
  profile: (id: number) => ["profiles", id] as const,
  schedule: (id: number) => ["profiles", id, "schedule"] as const,
  telemetry: (deviceId: number, start: string, end: string) =>
    ["telemetry", deviceId, start, end] as const,
  users: () => ["users"] as const,
  health: () => ["health"] as const,
  stats: () => ["stats"] as const,
};
