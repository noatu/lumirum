export type Role = "admin" | "owner" | { user: number };

export interface User {
  id: number;
  username: string;
  role: Role;
  created_at: string;
}

export interface AuthResponse extends User {
  token: string;
}

export interface Device {
  id: number;
  name: string;
  secret_key: string;
  owner_id: number;
  is_public: boolean;
  created_at: string;
  firmware_version: string | null;
  last_seen: string | null;
  profile_id: number | null;
}

export interface Profile {
  id: number;
  name: string;
  owner_id: number;
  is_shared: boolean;
  timezone: string;
  sleep_start: string;
  sleep_end: string;
  night_mode_enabled: boolean;
  min_color_temp: number;
  max_color_temp: number;
  motion_timeout_seconds: number;
  created_at: string;
  latitude: number | null;
  longitude: number | null;
}

export interface LightingPoint {
  utc: string;
  temp: number;
}

export interface LightingSchedule {
  profile_id: number;
  sleep_start_utc_seconds: number;
  sleep_end_utc_seconds: number;
  min_color_temp: number;
  max_color_temp: number;
  night_mode_enabled: boolean;
  motion_timeout_seconds: number;
  generated_at: string;
  valid_until: string;
  schedule: LightingPoint[];
}

export interface Telemetry {
  id: number;
  device_id: number;
  event_type: string;
  created_at: string;
  ambient_light: number | null;
  brightness: number | null;
  color_temp: number | null;
  light_is_on: boolean | null;
  motion_detected: boolean | null;
}

export interface Stats {
  users: number;
  profiles: number;
  devices: number;
  telemetry: number;
  timestamp: string;
}

export interface HealthResponse {
  status: "healthy" | "no_database_connection";
  timestamp: string;
}

export interface ErrorResponseInner {
  code: string;
  message: string;
}

export interface ErrorResponse {
  error: ErrorResponseInner;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  password: string;
}

export interface ChangeAccountRequest {
  password: string;
  new_username?: string | null;
  new_password?: string | null;
}

export interface DeleteRequest {
  password: string;
}

export interface CreateDeviceRequest {
  name: string;
  is_public: boolean;
  profile_id?: number | null;
}

export interface CreateProfileRequest {
  name: string;
  is_shared: boolean;
  timezone: string;
  sleep_start: string;
  sleep_end: string;
  night_mode_enabled: boolean;
  min_color_temp: number;
  max_color_temp: number;
  motion_timeout_seconds: number;
  latitude?: number | null;
  longitude?: number | null;
}

export interface CreateTelemetryRequest {
  event_type: string;
  ambient_light?: number | null;
  brightness?: number | null;
  color_temp?: number | null;
  light_is_on?: boolean | null;
  motion_detected?: boolean | null;
}

export function isAdmin(role: Role): boolean {
  return role === "admin";
}

export function isOwner(role: Role): boolean {
  return role === "owner";
}

export function isSubUser(role: Role): role is { user: number } {
  return typeof role === "object" && role !== null && "user" in role;
}
