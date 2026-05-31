import axios from "axios";
import type { ErrorResponse } from "../types";

const errorCodeMessages: Record<string, string> = {
  DeviceNotFound: "Device not found",
  ProfileNotFound: "Profile not found",
  InvalidCredentials: "Invalid username or password",
  UserNotFound: "User not found",
  TelemetryNotFound: "Telemetry entry not found",
  InvalidCoordinates: "Invalid coordinates provided",
  WeakPassword: "Password is too weak",
  UsernameTaken: "Username is already taken",
  DeviceNameTaken: "Device name is already taken",
  ProfileNameTaken: "Profile name is already taken",
  CannotDeleteAdmin: "Cannot delete an administrator account",
  CannotDeleteParentProfile: "Cannot delete a profile that has child profiles",
  Forbidden: "You do not have permission to perform this action",
};

export function apiErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error)) {
    const data = error.response?.data as ErrorResponse | undefined;
    const code = data?.error?.code;
    if (code && errorCodeMessages[code]) {
      return errorCodeMessages[code];
    }
    if (data?.error?.message) {
      return data.error.message;
    }
  }
  return "Something went wrong";
}

export async function safeApiCall<T>(
  fn: () => Promise<T>
): Promise<{ data: T; error: null } | { data: null; error: string }> {
  try {
    const data = await fn();
    return { data, error: null };
  } catch (err) {
    return { data: null, error: apiErrorMessage(err) };
  }
}
