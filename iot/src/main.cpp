// LumiRum IoT Client for ESP32-C3 with Arduino Framework

#include "config.h"
#include <Adafruit_NeoPixel.h>
#include <ArduinoJson.h>
#include <HTTPClient.h>
#include <Preferences.h>
#include <WebServer.h>
#include <WiFi.h>

const time_t MIN_VALID_EPOCH_SEC = 1735693200; // January 1, 2025 (Ensures NTP sync)

// Algorithm Constants
const float KELVIN_DIVISOR = 100.0; // Used in Tanner Helland's algorithm

Adafruit_NeoPixel strip(LED_COUNT, PIN_LED_RING, NEO_GRB + NEO_KHZ800);
Preferences preferences; // Non-volatile storage handler
WebServer server(WEB_SERVER_PORT);

String currentApiKey;        // Stores the active API Key (from NVS or Secrets)
bool isInConfigMode = false; // Flag to stop normal operation and serve Web UI

struct DeviceState {
  bool modeAuto = true;
  bool lightIsOn = false;
  int currentBrightnessPercent = 0;
  int currentColorTemp = DEFAULT_COLOR_TEMP_K;
  unsigned long motionLastSeenMs = 0;
  time_t lastKnownTimeSeconds = 0;
  bool scheduleLoaded = false;
  bool scheduleExpiredWarned = false;
} state;

struct LightingSchedule {
  long profileId = 0;
  uint32_t sleepStartUtcSeconds = 0;
  uint32_t sleepEndUtcSeconds = 0;
  int minColorTemp = DEFAULT_COLOR_TEMP_K;
  int maxColorTemp = 6500; // a sane default
  bool nightModeEnabled = false;
  int motionTimeoutSeconds = 300; // 5 minutes
  time_t generatedAt = 0;
  time_t validUntil = 0;

  struct Point {
    time_t timestamp;
    int colorTemp;
  };
  Point points[API_MAX_SCHEDULE_SIZE];
  int pointCount = 0;
} schedule;

// Function declarations
void setupWiFi();
void setupTime();
void loadApiKey();
void fetchSchedule();
void sendTelemetry(const char *eventType, bool motionDetected);
int getCurrentColorTemp();
void updateLighting();
void handleButton();
void handleMotion();
void handleBrightnessPot();
void handleTimeJump();
void convertColorTempToRGB(int kelvin, uint8_t *r, uint8_t *g, uint8_t *b);
bool isNightTime();
void setSerialCommands();
void enterConfigMode();
void handleConfigPortal();

void setup() {
  Serial.begin(115200);

  Serial.println("LumiRum IoT Client v1.0");

  // Hardware Init
  pinMode(PIN_PIR_SENSOR, INPUT);
  pinMode(PIN_BUTTON, INPUT_PULLUP);
  pinMode(PIN_POTENTIOMETER, INPUT);

  strip.begin();
  strip.setBrightness(DEFAULT_BRIGHTNESS_LIMIT_PERCENT);
  strip.show();
  Serial.println("[INIT] LED strip initialized");

  // Load Preferences (NVS)
  loadApiKey();

  setupWiFi();
  setupTime();
  fetchSchedule();

  if (isInConfigMode) {
    Serial.println("\n[!] AUTHENTICATION FAILED");
    Serial.println("[!] Device is in CONFIGURATION MODE");
    Serial.println("[!] Open your browser at: http://localhost:8180");
  } else {
    state.lastKnownTimeSeconds = time(nullptr);
    Serial.println("\n[READY] Device is ready!");
    Serial.println(
        "Commands: 'status', 'reset_key', 'fetch', 'time YYYY-MM-DD HH:MM:SS'");
  }
}

unsigned long lastScheduleCheckMs = 0;

void loop() {
  // If we hit a 401 error, enter config mode to update the key
  if (isInConfigMode) {
    server.handleClient();

    delay(10);
    return;
  }

  setSerialCommands();
  handleTimeJump();
  handleButton();
  handleMotion();
  handleBrightnessPot();
  updateLighting();

  if (millis() - lastScheduleCheckMs > SCHEDULE_REFRESH_INTERVAL_MS) {
    fetchSchedule();
    lastScheduleCheckMs = millis();
  }

  delay(LOOP_DELAY_MS);
}

void loadApiKey() {
  preferences.begin("lumirum", false); // Namespace "lumirum", Read/Write
  String savedKey = preferences.getString("apikey", "");

  if (savedKey.length() == API_KEY_LENGTH) {
    currentApiKey = savedKey;
    Serial.println("[Config] API Key loaded from NVS storage.");
  } else {
    currentApiKey = String(API_DEVICE_KEY); // From config.h
    Serial.println("[Config] Using default API Key from config.h");
  }
}

void setupWiFi() {
  Serial.print("[WiFi] Connecting to ");
  Serial.print(WIFI_SSID);
  Serial.print("...");

  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED && attempts < MAX_WIFI_ATTEMPTS) {
    delay(WIFI_RETRY_DELAY_MS);
    Serial.print(".");
    ++attempts;
  }

  if (WiFi.status() == WL_CONNECTED) {
    Serial.println(" Connected!");
    Serial.print("[WiFi] IP Address: ");
    Serial.println(WiFi.localIP());
  } else {
    Serial.println(" Failed!");
    Serial.println(
        "[ERROR] Could not connect to WiFi. Device will work in offline mode.");
  }
}

void setupTime() {
  Serial.println("[Time] Synchronizing with NTP server...");
  configTime(0, 0, "pool.ntp.org", "time.nist.gov");

  time_t now = time(nullptr);
  int attempts = 0;
  while (now < MIN_VALID_EPOCH_SEC && attempts < MAX_WIFI_ATTEMPTS) {
    delay(WIFI_RETRY_DELAY_MS);
    Serial.print(".");
    now = time(nullptr);
    ++attempts;
  }

  if (now > MIN_VALID_EPOCH_SEC) {
    Serial.println(" Synchronized!");
    Serial.print("[Time] Current UTC: ");
    Serial.println(ctime(&now));
  } else {
    Serial.println(" Failed!");
    Serial.println("[WARN] Could not sync time. Using default time.");
  }
}

void fetchSchedule() {
  if (WiFi.status() != WL_CONNECTED) {
    Serial.println("[ERROR] Cannot fetch schedule - no WiFi connection");
    return;
  }

  Serial.println("\n[API] Fetching lighting schedule...");

  HTTPClient http;
  String url = String(API_BASE_URL) + API_FETCH_ROUTE;

  http.begin(url);
  http.addHeader(API_KEY_HEADER, currentApiKey);

  int httpCode = http.GET();

  if (httpCode == HTTP_CODE_UNAUTHORIZED) {
    Serial.println("[ERROR] 401 Unauthorized. API Key invalid.");
    enterConfigMode();
    http.end();
    return;
  }

  if (httpCode == HTTP_CODE_OK) {
    String payload = http.getString();

    JsonDocument doc;
    DeserializationError error = deserializeJson(doc, payload);

    if (error) {
      Serial.print("[ERROR] JSON parsing failed: ");
      Serial.println(error.c_str());
      http.end();
      return;
    }

    schedule.profileId = doc["profile_id"];
    schedule.sleepStartUtcSeconds = doc["sleep_start_utc_seconds"];
    schedule.sleepEndUtcSeconds = doc["sleep_end_utc_seconds"];
    schedule.minColorTemp = doc["min_color_temp"];
    schedule.maxColorTemp = doc["max_color_temp"];
    schedule.nightModeEnabled = doc["night_mode_enabled"];
    schedule.motionTimeoutSeconds = doc["motion_timeout_seconds"];

    // Parse ISO8601 timestamps
    const char *generatedAtStr = doc["generated_at"];
    const char *validUntilStr = doc["valid_until"];

    struct tm tmGenerated, tmValid;
    strptime(generatedAtStr, "%Y-%m-%dT%H:%M:%S", &tmGenerated);
    strptime(validUntilStr, "%Y-%m-%dT%H:%M:%S", &tmValid);

    schedule.generatedAt = mktime(&tmGenerated);
    schedule.validUntil = mktime(&tmValid);

    JsonArray scheduleArray = doc["schedule"];
    schedule.pointCount = min((int)scheduleArray.size(), API_MAX_SCHEDULE_SIZE);

    for (int i = 0; i < schedule.pointCount; ++i) {
      JsonObject point = scheduleArray[i];

      const char *utcStr = point["utc"];
      struct tm tm;
      strptime(utcStr, "%Y-%m-%dT%H:%M:%S", &tm);
      schedule.points[i].timestamp = mktime(&tm);

      schedule.points[i].colorTemp = point["temp"];
    }

    state.scheduleLoaded = true;
    state.scheduleExpiredWarned = false;

    Serial.println("[API] Schedule loaded successfully!");
    Serial.print("[API] Profile ID: ");
    Serial.println(schedule.profileId);
    Serial.print("[API] Points loaded: ");
    Serial.println(schedule.pointCount);
    Serial.print("[API] Motion timeout: ");
    Serial.print(schedule.motionTimeoutSeconds);
    Serial.println(" seconds");
    Serial.print("[API] Night mode: ");
    Serial.println(schedule.nightModeEnabled ? "Enabled" : "Disabled");

  } else {
    Serial.print("[ERROR] HTTP request failed with code: ");
    Serial.println(httpCode);
    if (httpCode > 0)
      Serial.println(http.getString());
  }

  http.end();
}

static unsigned long lastTelemetryMs = 0;

void sendTelemetry(const char *eventType, bool motionDetected) {
  if (!TELEMETRY)
    return;

  if (millis() - lastTelemetryMs < TELEMETRY_DEBOUNCE_MS)
    return;
  lastTelemetryMs = millis();

  if (WiFi.status() != WL_CONNECTED)
    return;

  Serial.print("[Telemetry] Sending event: ");
  Serial.println(eventType);

  HTTPClient http;
  String url = String(API_BASE_URL) + API_TELEMETRY_ROUTE;

  http.begin(url);
  http.addHeader("Content-Type", "application/json");
  http.addHeader(API_KEY_HEADER, currentApiKey);

  JsonDocument doc;
  doc["event_type"] = eventType;
  doc["motion_detected"] = motionDetected;
  doc["light_is_on"] = state.lightIsOn;
  doc["brightness"] = state.currentBrightnessPercent;

  if (state.currentColorTemp >= API_MIN_TEMP_CONSTRAINT_K) {
    doc["color_temp"] = state.currentColorTemp;
  }

  String jsonPayload;
  serializeJson(doc, jsonPayload);

  int httpCode = http.POST(jsonPayload);

  if (httpCode == HTTP_CODE_UNAUTHORIZED) {
    Serial.println("[ERROR] 401 Unauthorized. API Key invalid.");
    enterConfigMode();
  } else if (httpCode == HTTP_CODE_OK || httpCode == HTTP_CODE_CREATED) {
    Serial.println("[Telemetry] Sent successfully");
  } else {
    Serial.print("[Telemetry] Failed with code: ");
    Serial.println(httpCode);
  }

  http.end();
}

void enterConfigMode() {
  if (isInConfigMode)
    return;

  isInConfigMode = true;

  // Visual cue with red light to indicate error/attention needed
  state.lightIsOn = true;
  state.currentColorTemp = MIN_COLOR_TEMP_K;
  state.currentBrightnessPercent = 50;
  updateLighting();

  Serial.println("\n!!! ENTERING CONFIGURATION MODE !!!");
  Serial.print("Please connect to: http://");
  Serial.println(WiFi.localIP());
  Serial.println("Or: http://localhost:8180");

  // Define Web Server Routes
  server.on("/", HTTP_GET, []() {
    String html = "<html><body><h1>LumiRum Device Config</h1>";
    html += "<p>Device is unauthorized. Please update API Key.</p>";
    html += "<form action='/save' method='POST'>";
    html += "API Key: <input type='text' name='apikey' size='70'><br><br>";
    html += "<input type='submit' value='Save & Reboot'>";
    html += "</form></body></html>";
    server.send(HTTP_CODE_OK, "text/html", html);
  });

  server.on("/save", HTTP_POST, []() {
    if (server.hasArg("apikey")) {
      String newKey = server.arg("apikey");
      newKey.trim();

      if (newKey.length() == API_KEY_LENGTH) {
        preferences.putString("apikey", newKey);
        server.send(HTTP_CODE_OK, "text/html",
                    "<body>Saved! Rebooting...</body>");
        delay(1000);
        ESP.restart();
      } else {
        server.send(HTTP_CODE_BAD_REQUEST, "text/html",
                    "<body>Invalid Key Length</body>");
      }
    } else {
      server.send(HTTP_CODE_BAD_REQUEST, "text/plain", "Missing apikey");
    }
  });

  server.begin();
}

int getCurrentColorTemp() {
  if (!state.scheduleLoaded || schedule.pointCount == 0)
    return DEFAULT_COLOR_TEMP_K;

  time_t now = time(nullptr);

  if (schedule.nightModeEnabled && isNightTime())
    return MIN_COLOR_TEMP_K;

  if (now > schedule.validUntil && !state.scheduleExpiredWarned) {
    Serial.println("[WARN] Schedule expired, using cyclic lookup");
    state.scheduleExpiredWarned = true;
  }

  // Get time of day for cyclic lookup
  struct tm timeinfo;
  gmtime_r(&now, &timeinfo);
  uint32_t currentDaySeconds =
      timeinfo.tm_hour * 3600 + timeinfo.tm_min * 60 + timeinfo.tm_sec;

  // Find matching points by time of day (cyclic)
  for (int i = 0; i < schedule.pointCount - 1; ++i) {
    struct tm tm1, tm2;
    gmtime_r(&schedule.points[i].timestamp, &tm1);
    gmtime_r(&schedule.points[i + 1].timestamp, &tm2);

    uint32_t daySeconds1 = tm1.tm_hour * 3600 + tm1.tm_min * 60 + tm1.tm_sec;
    uint32_t daySeconds2 = tm2.tm_hour * 3600 + tm2.tm_min * 60 + tm2.tm_sec;

    if (currentDaySeconds >= daySeconds1 && currentDaySeconds < daySeconds2) {
      // Linear interpolation
      float progress = (float)(currentDaySeconds - daySeconds1) /
                       (float)(daySeconds2 - daySeconds1);
      int temp1 = schedule.points[i].colorTemp;
      int temp2 = schedule.points[i + 1].colorTemp;

      return temp1 + (int)(progress * (temp2 - temp1));
    }
  }

  // Use last point if past all points
  return schedule.points[schedule.pointCount - 1].colorTemp;
}

bool isNightTime() {
  time_t now = time(nullptr);
  struct tm timeinfo;
  gmtime_r(&now, &timeinfo);
  uint32_t secondsSinceMidnight =
      timeinfo.tm_hour * 3600 + timeinfo.tm_min * 60 + timeinfo.tm_sec;

  if (schedule.sleepStartUtcSeconds <= schedule.sleepEndUtcSeconds) {
    // e.g. 2:00 - 10:00 a.m.
    return secondsSinceMidnight >= schedule.sleepStartUtcSeconds &&
           secondsSinceMidnight < schedule.sleepEndUtcSeconds;
  } else {
    // e.g. 20:00 - 6:00
    return secondsSinceMidnight >= schedule.sleepStartUtcSeconds ||
           secondsSinceMidnight < schedule.sleepEndUtcSeconds;
  }
}

void handleTimeJump() {
  time_t now = time(nullptr);
  time_t timeDiff = labs(now - state.lastKnownTimeSeconds);

  if (timeDiff > schedule.motionTimeoutSeconds) {
    Serial.print("[Time] Detected time jump of ");
    Serial.print(timeDiff);
    Serial.println(" seconds");

    // Check if light timeout expired during jump
    if (state.lightIsOn && state.modeAuto) {
      unsigned long timeSinceMotion =
          (now - (state.motionLastSeenMs / 1000)) * 1000;
      if (timeSinceMotion >
          (unsigned long)schedule.motionTimeoutSeconds * 1000) {
        Serial.println("[Time] Light timeout expired during time jump");
        state.lightIsOn = false;
      }
    }

    // Trigger schedule refresh if time jumped significantly forward
    if (now > state.lastKnownTimeSeconds + TIME_JUMP_REFETCH_THRESHOLD_SEC) {
      Serial.println("[Time] Triggering schedule refresh");
      fetchSchedule();
    }
  }

  state.lastKnownTimeSeconds = now;
}

static bool lastButtonState = HIGH;
static unsigned long lastButtonPressMs = 0;

void handleButton() {
  bool currentButtonState = digitalRead(PIN_BUTTON);

  if (currentButtonState == LOW && lastButtonState == HIGH &&
      millis() - lastButtonPressMs > BUTTON_DEBOUNCE_MS) {
    state.modeAuto = !state.modeAuto;

    Serial.print("[Button] Mode switched to: ");
    Serial.println(state.modeAuto ? "AUTO" : "MANUAL");

    if (state.modeAuto) {
      state.lightIsOn = false;
    } else {
      state.lightIsOn = true;
      state.currentColorTemp = DEFAULT_COLOR_TEMP_K;
    }

    sendTelemetry("mode_change", false);
    lastButtonPressMs = millis();
  }

  lastButtonState = currentButtonState;
}

void handleMotion() {
  if (!state.modeAuto)
    return;

  bool motionDetected = digitalRead(PIN_PIR_SENSOR);

  if (motionDetected) {
    if (!state.lightIsOn) {
      Serial.println("[Motion] Detected - turning light ON");
      sendTelemetry("motion_detected", true);
    }

    state.lightIsOn = true;
    state.motionLastSeenMs = millis();
    state.currentColorTemp = getCurrentColorTemp();
    return;
  }

  if (state.lightIsOn &&
      (millis() - state.motionLastSeenMs >
       (unsigned long)schedule.motionTimeoutSeconds * 1000)) {
    Serial.println("[Motion] Timeout - turning light OFF");
    state.lightIsOn = false;
    sendTelemetry("motion_timeout", false);
  }
}

void handleBrightnessPot() {
  int potValue = analogRead(PIN_POTENTIOMETER);
  int brightness = map(potValue, 0, ANALOG_MAX_VALUE, 0, 100);

  if (brightness <= BRIGHTNESS_OFF_THRESHOLD_PERCENT) {
    if (state.lightIsOn && !state.modeAuto) {
      state.lightIsOn = false;
      Serial.println("[Brightness] Light turned OFF (pot at minimum)");
    }
    state.currentBrightnessPercent = 0;
    return;
  }

  if (!state.modeAuto && !state.lightIsOn &&
      brightness > BRIGHTNESS_OFF_THRESHOLD_PERCENT) {
    state.lightIsOn = true;
    Serial.println("[Brightness] Light turned ON (pot increased)");
  }

  if (abs(brightness - state.currentBrightnessPercent) >
      BRIGHTNESS_CHANGE_THRESHOLD_PERCENT) {
    state.currentBrightnessPercent = brightness;
  }
}

void updateLighting() {
  if (!state.lightIsOn) {
    strip.clear();
    strip.show();
    return;
  }

  uint8_t r, g, b;
  convertColorTempToRGB(state.currentColorTemp, &r, &g, &b);

  int actualBrightness =
      map(state.currentBrightnessPercent, 0, 100, 0, PWM_MAX_VALUE);
  strip.setBrightness(actualBrightness);

  for (int i = 0; i < LED_COUNT; ++i)
    strip.setPixelColor(i, strip.Color(r, g, b));

  strip.show();
}

// Tanner Helland's Algorithm for RGB from Kelvin
// https://tannerhelland.com/2012/09/18/convert-temperature-rgb-algorithm-code.html
void convertColorTempToRGB(int kelvin, uint8_t *r, uint8_t *g, uint8_t *b) {
  float temp = kelvin / KELVIN_DIVISOR;
  float red, green, blue;

  // Red calculation
  if (temp <= 66) {
    red = 255;
  } else {
    red = temp - 60;
    red = 329.698727446 * pow(red, -0.1332047592);
    red = constrain(red, 0, 255);
  }

  // Green calculation
  if (temp <= 66) {
    green = temp;
    green = 99.4708025861 * log(green) - 161.1195681661;
    green = constrain(green, 0, 255);
  } else {
    green = temp - 60;
    green = 288.1221695283 * pow(green, -0.0755148492);
    green = constrain(green, 0, 255);
  }

  // Blue calculation
  if (temp >= 66) {
    blue = 255;
  } else if (temp <= 19) {
    blue = 0;
  } else {
    blue = temp - 10;
    blue = 138.5177312231 * log(blue) - 305.0447927307;
    blue = constrain(blue, 0, 255);
  }

  *r = (uint8_t)red;
  *g = (uint8_t)green;
  *b = (uint8_t)blue;
}

void setSerialCommands() {
  if (Serial.available() <= 0)
    return;

  String command = Serial.readStringUntil('\n');
  command.trim();

  if (command == "status") {
    Serial.println("\nDEVICE STATUS");
    Serial.print("Mode: ");
    Serial.println(state.modeAuto ? "AUTO" : "MANUAL");
    Serial.print("Light: ");
    Serial.println(state.lightIsOn ? "ON" : "OFF");
    Serial.print("Brightness: ");
    Serial.print(state.currentBrightnessPercent);
    Serial.println("%");
    Serial.print("Color Temp: ");
    Serial.print(state.currentColorTemp);
    Serial.println("K");
    Serial.print("Schedule loaded:    ");
    Serial.println(state.scheduleLoaded ? "Yes" : "No");
    Serial.print("Night mode enabled: ");
    Serial.println(schedule.nightModeEnabled ? "Yes" : "No");
    Serial.print("Night mode status:  ");
    Serial.println(schedule.nightModeEnabled && isNightTime() ? "Active"
                                                              : "Inactive");
    Serial.print("Telemetry: ");
    Serial.println(TELEMETRY ? "Enabled" : "Disabled");
    Serial.print("Current API Key (first 5): ");
    Serial.println(currentApiKey.substring(0, 5));
    time_t now = time(nullptr);
    Serial.print("Current time: ");
    Serial.print(ctime(&now));
    Serial.println();

  } else if (command == "fetch") {
    fetchSchedule();

  } else if (command == "reset_key") {
    preferences.putString("apikey", "");
    Serial.println("API Key cleared from NVS. Rebooting...");
    delay(500);
    ESP.restart();

  } else if (command.startsWith("time ")) {
    String timeStr = command.substring(5);
    struct tm tm;
    if (strptime(timeStr.c_str(), "%Y-%m-%d %H:%M:%S", &tm) == NULL) {
      Serial.println("[ERROR] Invalid time format. Use: YYYY-MM-DD HH:MM:SS");
      return;
    }

    time_t t = mktime(&tm);
    struct timeval tv = {t, 0};
    settimeofday(&tv, NULL);
    Serial.print("[Time] Set to: ");
    Serial.println(ctime(&t));
  }
}
