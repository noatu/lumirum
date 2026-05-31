# LumiRum Web

React + TypeScript SPA for the LumiRum smart lighting system. Connects to the LumiRum backend REST API.

## Running

The backend must be running at `localhost:3000`. From this rectory

```bash
npm run dev
```

Open http://localhost:5173.

The dev server proxies all API routes (`/auth`, `/devices`, `/profiles`, `/telemetry`, `/users`, `/health`, `/stats`) to `localhost:3000`, so no CORS configuration is needed.

## Features

- **Devices** — list, create, edit, delete. Displays last-seen time and firmware version. Secret key management (view masked, copy, regenerate).
- **Profiles** — circadian rhythm profiles with sleep schedule, color temperature range, and optional GPS coordinates for solar-cycle calculation. Shared profiles from other users are visible but read-only.
- **Lighting schedule** — visual chart of the computed color temperature curve across the day (96 points, 15-minute intervals).
- **Telemetry** — per-device sensor history with date range filtering. Chart view (brightness / color temperature / ambient light on independent Y-axes) and table view. Bulk delete by date range.
- **Account** — change username and password, manage sub-users, delete account.
- **Admin panel** — health check, system stats, user search and management. Visible only to admin accounts.
- **Export / Import** — export all your devices and profiles as a JSON file, then import it back (or into another account). Import matches by name: existing records are updated, new ones are created.

## Data export format

Exported files follow this shape:

```json
{
  "version": 1,
  "exported_at": "<ISO timestamp>",
  "profiles": [
    {
      "name": "Morning Person",
      "timezone": "Europe/Kyiv",
      "sleep_start": "22:30:00",
      "sleep_end": "06:30:00",
      "min_color_temp": 2200,
      "max_color_temp": 6500,
      "motion_timeout_seconds": 300,
      "latitude": 50.4501,
      "longitude": 30.5234,
      "is_shared": true,
      "night_mode_enabled": true
    }
  ],
  "devices": [
    {
      "name": "Living Room Light",
      "profile_name": "Morning Person",
      "is_public": true
    }
  ]
}
```

Server-generated fields (`id`, `owner_id`, `secret_key`, `created_at`) are stripped. `profile_name` is used instead of `profile_id` so exports are portable across accounts. A sample file for testing is at `sample-import.json`.

## Internationalization

UI is available in English and Ukrainian. Language is toggled in the top-right header and persisted in `localStorage`. Date and time formatting follows the selected locale via `dayjs`.

## Stack

| Concern | Library |
|---|---|
| UI components | Mantine v7 |
| Server state / caching | TanStack Query v5 |
| Routing | React Router v6 |
| HTTP client | axios |
| Forms + validation | react-hook-form + zod |
| Charts | Recharts |
| i18n | react-i18next |
| Auth state | Zustand (persisted to localStorage) |
