-- LumiRum test data
-- All users have password: lumirum!

-- Owner users
INSERT INTO users (username, role, password_hash) VALUES
    ('alice', 'owner', '$argon2id$v=19$m=32768,t=3,p=1$dGVzdHNhbHQ$phr/Oj1wInJyuWLDue7DYBqDIfIHWLgDY8W1iXQ61g8'),
    ('bob',   'owner', '$argon2id$v=19$m=32768,t=3,p=1$dGVzdHNhbHQ$phr/Oj1wInJyuWLDue7DYBqDIfIHWLgDY8W1iXQ61g8');

-- Sub-users for alice
INSERT INTO users (username, role, parent_id, password_hash)
SELECT 'alicechild1', 'user', id, '$argon2id$v=19$m=32768,t=3,p=1$dGVzdHNhbHQ$phr/Oj1wInJyuWLDue7DYBqDIfIHWLgDY8W1iXQ61g8'
FROM users WHERE username = 'alice';

INSERT INTO users (username, role, parent_id, password_hash)
SELECT 'alicechild2', 'user', id, '$argon2id$v=19$m=32768,t=3,p=1$dGVzdHNhbHQ$phr/Oj1wInJyuWLDue7DYBqDIfIHWLgDY8W1iXQ61g8'
FROM users WHERE username = 'alice';

-- Profiles for alice
INSERT INTO profiles (name, owner_id, is_shared, latitude, longitude, timezone, sleep_start, sleep_end, night_mode_enabled, min_color_temp, max_color_temp, motion_timeout_seconds)
SELECT 'Home Default', id, true, 50.4501, 30.5234, 'Europe/Kyiv', '22:30', '07:00', true, 2200, 6500, 300
FROM users WHERE username = 'alice';

INSERT INTO profiles (name, owner_id, is_shared, latitude, longitude, timezone, sleep_start, sleep_end, night_mode_enabled, min_color_temp, max_color_temp, motion_timeout_seconds)
SELECT 'Office', id, false, 50.4501, 30.5234, 'Europe/Kyiv', '23:00', '06:30', false, 4000, 6500, 180
FROM users WHERE username = 'alice';

-- Profile for bob
INSERT INTO profiles (name, owner_id, is_shared, timezone, sleep_start, sleep_end, night_mode_enabled, min_color_temp, max_color_temp, motion_timeout_seconds)
SELECT 'Bob Home', id, true, 'UTC', '23:00', '07:30', true, 2000, 5500, 600
FROM users WHERE username = 'bob';

-- Devices for alice
INSERT INTO devices (name, secret_key, owner_id, is_public, firmware_version, profile_id)
SELECT 'Living Room Light', 'sk_alice_living_room_abc123', u.id, true, '1.2.0', p.id
FROM users u, profiles p
WHERE u.username = 'alice' AND p.name = 'Home Default' AND p.owner_id = u.id;

INSERT INTO devices (name, secret_key, owner_id, is_public, firmware_version, profile_id)
SELECT 'Bedroom Light', 'sk_alice_bedroom_def456', u.id, false, '1.1.5', p.id
FROM users u, profiles p
WHERE u.username = 'alice' AND p.name = 'Home Default' AND p.owner_id = u.id;

INSERT INTO devices (name, secret_key, owner_id, is_public, firmware_version)
SELECT 'Kitchen Light', 'sk_alice_kitchen_ghi789', id, true, '1.2.0'
FROM users WHERE username = 'alice';

-- Device for bob
INSERT INTO devices (name, secret_key, owner_id, is_public, firmware_version, profile_id)
SELECT 'Bob Living Room', 'sk_bob_living_jkl012', u.id, true, '1.0.3', p.id
FROM users u, profiles p
WHERE u.username = 'bob' AND p.name = 'Bob Home' AND p.owner_id = u.id;

-- Telemetry for alice's Living Room Light (last 7 days, every 2 hours)
INSERT INTO telemetry (device_id, event_type, brightness, color_temp, ambient_light, light_is_on, created_at)
SELECT
    d.id,
    CASE WHEN s.n % 3 = 0 THEN 'motion_detected' WHEN s.n % 3 = 1 THEN 'light_on' ELSE 'light_off' END,
    CASE WHEN s.hour BETWEEN 8 AND 22 THEN 60 + (s.n % 30) ELSE 20 + (s.n % 15) END,
    CASE WHEN s.hour BETWEEN 8 AND 18 THEN 5500 + (s.n % 500) ELSE 2500 + (s.n % 300) END,
    CASE WHEN s.hour BETWEEN 6 AND 20 THEN 400 + (s.n * 10) ELSE 50 + s.n END,
    s.hour BETWEEN 7 AND 23,
    NOW() - (s.n || ' hours')::INTERVAL
FROM devices d,
     (SELECT generate_series(0, 83) AS n, (generate_series(0, 83) % 24) AS hour) s
WHERE d.name = 'Living Room Light'
  AND d.owner_id = (SELECT id FROM users WHERE username = 'alice');

-- last_seen: reflect realistic device activity
UPDATE devices SET last_seen = NOW() - INTERVAL '3 minutes'  WHERE name = 'Living Room Light';
UPDATE devices SET last_seen = NOW() - INTERVAL '2 hours'    WHERE name = 'Bedroom Light';
UPDATE devices SET last_seen = NOW() - INTERVAL '4 days'     WHERE name = 'Kitchen Light';
UPDATE devices SET last_seen = NOW() - INTERVAL '30 minutes' WHERE name = 'Bob Living Room';

-- Telemetry for alice's Bedroom Light (last 3 days, every 4 hours)
INSERT INTO telemetry (device_id, event_type, brightness, color_temp, ambient_light, motion_detected, light_is_on, created_at)
SELECT
    d.id,
    CASE WHEN s.n % 2 = 0 THEN 'motion_detected' ELSE 'light_on' END,
    CASE WHEN s.hour BETWEEN 7 AND 22 THEN 40 + (s.n % 40) ELSE 10 + (s.n % 10) END,
    CASE WHEN s.hour BETWEEN 7 AND 18 THEN 4800 + (s.n % 400) ELSE 2200 + (s.n % 200) END,
    CASE WHEN s.hour BETWEEN 6 AND 21 THEN 200 + (s.n * 5) ELSE 20 + s.n END,
    s.n % 3 = 0,
    s.hour BETWEEN 7 AND 23,
    NOW() - (s.n * 4 || ' hours')::INTERVAL
FROM devices d,
     (SELECT generate_series(0, 17) AS n, (generate_series(0, 17) * 4 % 24) AS hour) s
WHERE d.name = 'Bedroom Light'
  AND d.owner_id = (SELECT id FROM users WHERE username = 'alice');

-- Telemetry for bob's Living Room Light (last 2 days, every 3 hours)
INSERT INTO telemetry (device_id, event_type, brightness, color_temp, ambient_light, motion_detected, light_is_on, created_at)
SELECT
    d.id,
    CASE WHEN s.n % 2 = 0 THEN 'light_on' ELSE 'motion_detected' END,
    CASE WHEN s.hour BETWEEN 8 AND 23 THEN 50 + (s.n % 35) ELSE 15 + (s.n % 10) END,
    CASE WHEN s.hour BETWEEN 8 AND 18 THEN 5000 + (s.n % 600) ELSE 2700 + (s.n % 200) END,
    CASE WHEN s.hour BETWEEN 7 AND 21 THEN 300 + (s.n * 8) ELSE 30 + s.n END,
    s.n % 4 = 0,
    s.hour BETWEEN 8 AND 23,
    NOW() - (s.n * 3 || ' hours')::INTERVAL
FROM devices d,
     (SELECT generate_series(0, 15) AS n, (generate_series(0, 15) * 3 % 24) AS hour) s
WHERE d.name = 'Bob Living Room'
  AND d.owner_id = (SELECT id FROM users WHERE username = 'bob');
