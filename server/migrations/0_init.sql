-- LumiRum Database Schema

CREATE TYPE user_role AS ENUM (
    'admin',  -- server access, for maintenance
    'owner',  -- full home control
    'user'    -- can control public devices and read config of other devices
);

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL CONSTRAINT users_username_key UNIQUE CHECK (length(username) >= 3),
    password_hash TEXT NOT NULL, -- argon2id
    role user_role NOT NULL DEFAULT 'owner',
    -- If role is 'user', this MUST NOT be NULL.
    parent_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

    CHECK (role != 'user' OR parent_id IS NOT NULL)
);

CREATE INDEX idx_users_parent_id ON users(parent_id) WHERE parent_id IS NOT NULL;


-- Circadian Rhythm Settings
CREATE TABLE profiles (
    id BIGSERIAL PRIMARY KEY,
    owner_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (length(name) > 0 AND length(name) <= 200),

    -- Location, for solar cycle calculation
    latitude DOUBLE PRECISION CHECK (latitude BETWEEN -90 AND 90),
    longitude DOUBLE PRECISION CHECK (longitude BETWEEN -180 AND 180),

    timezone TEXT NOT NULL DEFAULT 'UTC', -- IANA: 'Europe/Kyiv'
    -- Local time in user's timezone
    sleep_start TIME NOT NULL DEFAULT '22:00',
    sleep_end TIME NOT NULL DEFAULT '07:00',

    night_mode_enabled BOOLEAN NOT NULL DEFAULT true,

    -- Color temperature limits
    min_color_temp INTEGER NOT NULL DEFAULT 2000 CHECK (min_color_temp BETWEEN 1800 AND 10000),
    max_color_temp INTEGER NOT NULL DEFAULT 6500 CHECK (max_color_temp BETWEEN 1800 AND 10000),

    -- Timing
    motion_timeout_seconds INTEGER NOT NULL DEFAULT 300,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT profiles_owner_id_name_key UNIQUE (owner_id, name),
    CHECK (min_color_temp <= max_color_temp)
);

CREATE INDEX idx_profiles_owner_id ON profiles(owner_id);



CREATE TABLE devices (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL CHECK (length(name) > 0 AND length(name) <= 200),
    secret_key TEXT NOT NULL, -- device auth token

    profile_id BIGINT REFERENCES profiles(id) ON DELETE SET NULL,
    owner_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_public BOOLEAN NOT NULL DEFAULT true, -- Can a User role control it?

    firmware_version TEXT,
    last_seen TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT devices_owner_id_name_key UNIQUE (owner_id, name)
);

CREATE INDEX idx_devices_owner_id ON devices(owner_id);
CREATE INDEX idx_devices_profile_id ON devices(profile_id) WHERE profile_id IS NOT NULL;

-- Telemetry events from devices
CREATE TABLE telemetry (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES devices(id) ON DELETE CASCADE,

    event_type TEXT NOT NULL, -- 'motion_detected', 'light_on', 'light_off', etc.
    motion_detected BOOLEAN,
    light_is_on BOOLEAN,
    brightness INTEGER CHECK (brightness BETWEEN 0 AND 100),
    color_temp INTEGER CHECK (color_temp BETWEEN 1800 AND 10000),

    -- Sensor data
    ambient_light INTEGER, -- photoresistor ADC value

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_telemetry_device_time ON telemetry(device_id, created_at DESC);
CREATE INDEX idx_telemetry_event_type ON telemetry(event_type, created_at DESC);


-- CHANGE AFTER INITIAL DEPLOYMENT
-- Default admin user with password: 'lumirum!'
INSERT INTO users (username, role, password_hash) VALUES
    ('admin', 'admin', '$argon2id$v=19$m=32768,t=3,p=1$dGVzdHNhbHQ$phr/Oj1wInJyuWLDue7DYBqDIfIHWLgDY8W1iXQ61g8');

