// Device Configuration
#ifndef LUMIRUM_CONFIG_H
#define LUMIRUM_CONFIG_H
#include <Arduino.h> // for time_t type

// WiFi Credentials
// WARNING: the " must stay intact
#define WIFI_SSID "Wokwi-GUEST"
#define WIFI_PASSWORD ""

// Network & API Configuration
#define API_BASE_URL "http://192.168.18.103:3000"
#define API_FETCH_ROUTE "/devices/circadian"
#define API_TELEMETRY_ROUTE "/telemetry"
#define API_KEY_HEADER "x-api-key"

const int API_KEY_LENGTH = 64; // chars
#define API_DEVICE_KEY "OjEYR18aTyNiKMEP8o7XLDD3Rv47rUVVzPBIsxrdB0iOb3PMMmpcsRPiWIDJdRb2"
const bool TELEMETRY = true; // Whether to send telemetry to the API

const int MAX_WIFI_ATTEMPTS = 20;
const int WIFI_RETRY_DELAY_MS = 500; // 0.5 seconds between connection attempts
const int WEB_SERVER_PORT = 80;

const int API_MAX_SCHEDULE_SIZE = 96; // elements
const int API_MIN_TEMP_CONSTRAINT_K = 1800; // Minimum allowed by API

// Pin configuration
const int PIN_BUTTON = 3;
const int PIN_LED_RING = 8;
const int PIN_PIR_SENSOR = 1;
const int PIN_POTENTIOMETER = 0;

// Hardware configuration
const int LED_COUNT = 16;
const int ANALOG_MAX_VALUE = 4095; // ESP32-C3 ADC resolution is 12-bit
const int PWM_MAX_VALUE = 255;     // Standard 8-bit PWM limit

// Timing configuration
const unsigned long LOOP_DELAY_MS = 50;   // 20Hz refresh rate
const unsigned long BOOT_DELAY_MS = 1000; // 1 second warm-up
const unsigned long BUTTON_DEBOUNCE_MS = 200; // 0.2 seconds is enough for a button press
const unsigned long SCHEDULE_REFRESH_INTERVAL_MS = 3600000; // 1 hour
const unsigned long TELEMETRY_DEBOUNCE_MS = 2000; // 2 seconds min between sending telemetry events
const unsigned long TIME_JUMP_REFETCH_THRESHOLD_SEC =
    3600; // 1 hour change triggers schedule refetch

// Color temperature limits (Kelvin)
const int MIN_COLOR_TEMP_K = 1000;     // Candlelight/Deep Red
const int DEFAULT_COLOR_TEMP_K = 3500; // Warm White

// Brightness thresholds (0-100%)
const int DEFAULT_BRIGHTNESS_LIMIT_PERCENT = 50; // Safety limit for startup
const int BRIGHTNESS_OFF_THRESHOLD_PERCENT = 10; // Below this, lights turn off
const int BRIGHTNESS_CHANGE_THRESHOLD_PERCENT = 5; // Hysteresis to prevent flickering


#endif // LUMIRUM_CONFIG_H
