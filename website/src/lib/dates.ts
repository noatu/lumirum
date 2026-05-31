import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";

dayjs.extend(relativeTime);

export function formatRelative(date: string | null, lang: string): string {
  if (!date) return "—";
  return dayjs(date).locale(lang).fromNow();
}

export function formatDateTime(date: string | null, lang: string): string {
  if (!date) return "—";
  return dayjs(date).locale(lang).format("YYYY-MM-DD HH:mm");
}

export function formatTime(seconds: number): string {
  const h = Math.floor(seconds / 3600)
    .toString()
    .padStart(2, "0");
  const m = Math.floor((seconds % 3600) / 60)
    .toString()
    .padStart(2, "0");
  return `${h}:${m}`;
}
