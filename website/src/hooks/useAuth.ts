import { useMutation, useQuery } from "@tanstack/react-query";
import { login, register, getMe, updateAccount, deleteAccount } from "../api/auth";
import { useAuthStore } from "../store/auth";
import type {
  LoginRequest,
  RegisterRequest,
  ChangeAccountRequest,
  DeleteRequest,
} from "../types";

export function useGetMe() {
  const token = useAuthStore((s) => s.token);
  return useQuery({
    queryKey: ["me"],
    queryFn: getMe,
    enabled: !!token,
    staleTime: 5 * 60 * 1000,
  });
}

export function useLogin() {
  const setToken = useAuthStore((s) => s.setToken);
  const setUser = useAuthStore((s) => s.setUser);

  return useMutation({
    mutationFn: (data: LoginRequest) => login(data),
    onSuccess: (auth) => {
      setToken(auth.token);
      setUser(auth);
    },
  });
}

export function useRegister() {
  const setToken = useAuthStore((s) => s.setToken);
  const setUser = useAuthStore((s) => s.setUser);

  return useMutation({
    mutationFn: (data: RegisterRequest) => register(data),
    onSuccess: (auth) => {
      setToken(auth.token);
      setUser(auth);
    },
  });
}

export function useUpdateAccount() {
  const setUser = useAuthStore((s) => s.setUser);

  return useMutation({
    mutationFn: (data: ChangeAccountRequest) => updateAccount(data),
    onSuccess: (auth) => {
      setUser(auth);
    },
  });
}

export function useDeleteAccount() {
  const logout = useAuthStore((s) => s.logout);

  return useMutation({
    mutationFn: (data: DeleteRequest) => deleteAccount(data),
    onSuccess: () => {
      logout();
    },
  });
}
