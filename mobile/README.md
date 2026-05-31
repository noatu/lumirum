# LumiRum Android

Mobile client for the LumiRum smart lighting management system. Connects to the LumiRum backend over REST and lets owners manage IoT lighting devices, configure circadian lighting schedules, and inspect device telemetry.

**Minimum Android version:** 8.0 (API 26) — **Target:** API 36

---

## Features

- **Authentication** — login and self-registration for Owner accounts; automatic JWT refresh and redirect to login on session expiry
- **Devices** — list, create, edit, and delete IoT lighting devices; shows firmware version, last-seen time, and creation date
- **Profiles** — manage lighting schedule profiles with geographic coordinates, time zone, and access flag
- **Circadian schedule** — view the full 24-hour colour-temperature schedule for a profile (96 × 15-minute points) grouped into contiguous equal-temperature ranges
- **Telemetry** — interactive line chart (brightness %, colour temperature K, ambient light lux) with per-series Y-axis scaling; grouped list view; date/time range filter; bulk delete by range
- **Account management** — change username and password, delete account, create sub-users (Owner only)
- **Admin panel** — server and database health status, system-wide stats, searchable user list with deletion (Admin role only)
- **Localisation** — English (default) and Ukrainian (`values-uk/strings.xml`)
- **Material You** — dynamic colour theming on Android 12+

---

## Stack

| Layer | Technology |
|---|---|
| Language | Kotlin with coroutines |
| UI | Jetpack Compose + Material 3 |
| HTTP | Retrofit 2 + OkHttp |
| JSON | Gson with a custom deserialiser for the `Role` discriminated union |
| Navigation | Jetpack Navigation Compose |
| Persistence | Jetpack DataStore Preferences (JWT token) |

---

## Architecture

The app follows **MVVM** with a strict unidirectional dependency rule: Screens → ViewModels → Repositories → API. Nothing points upward.

```
ui/
  screen/<feature>/
    <Feature>Screen.kt      — Compose UI, reads StateFlow, no business logic
    <Feature>ViewModel.kt   — coroutine scope, maps Result<T> into UiState<T>
  components/               — shared composables (TelemetryChart, etc.)
  navigation/AppNavigation  — single NavHost, listens for 401 logout events

data/
  api/
    LumiRumApi.kt           — Retrofit interface, one function per endpoint
    ApiClient.kt            — OkHttp setup: JWT interceptor, 401 SharedFlow
    ApiUtils.kt             — safeApiCall: turns HTTP responses and server
                              error codes into Result<T> in one place
    Dtos.kt                 — all request/response data classes
  repository/               — typed facades; callers never see a raw Response
  local/AppDataStore.kt     — DataStore wrapper for the JWT token

AppContainer.kt             — manual DI: constructs all singletons once,
                              shared via CompositionLocal
```

**Dependency injection** is done by hand in `AppContainer`, created once in the `Application` class and provided to the Compose tree via `CompositionLocal`. All repositories share the same `LumiRumApi` instance — Kotlin variables hold references, not copies, so there is no duplication.

**Error handling** is centralised in `ApiUtils.safeApiCall`. It parses the server's structured `{ "code": "..." }` error body and maps known codes (`DeviceNotFound`, `InvalidCredentials`, etc.) to localised messages, falling back to the HTTP status for unknown errors. Repositories return `Result<T>`; ViewModels never deal with raw exceptions.

**List refresh without flicker** uses `repeatOnLifecycle(RESUMED)` so lists reload on navigation return. While reloading, cached data stays visible and the loading indicator in the FAB is delayed by one second — it only appears if the request takes longer than that.

---

## Backend API

The mobile app is built on top of a well-designed REST API, and that quality made a real difference during development.

The API returns structured JSON errors with a `code` field (`ProfileNotFound`, `InvalidCredentials`, `DeviceNotFound`, …) alongside the HTTP status. This made it possible to build `ApiUtils` as a single, exhaustive error-mapping layer rather than scattering ad-hoc string parsing across every repository. Users see actionable messages; the code stays clean.

Input validation on the server side meant the client could stay thin — no need to duplicate constraint checks in the app. If something is wrong, the server says so precisely and the app surfaces it.

Comprehensive API documentation meant every endpoint's shape, required fields, and possible error codes were known up front. Screen and ViewModel implementations could be written against a clear contract without exploratory trial-and-error against a live server.

The combination — validated inputs, structured errors, documented contract — compressed what would otherwise be a long debugging cycle into a straightforward mapping exercise.
