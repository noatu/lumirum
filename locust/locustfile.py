import uuid
from locust import HttpUser, task, between


class LumirumUser(HttpUser):
    wait_time = between(0.5, 2)

    def on_start(self):
        username = f"load{uuid.uuid4().hex[:12]}"
        password = "loadtest123"

        self.client.post("/auth/register", json={
            "username": username,
            "password": password,
        })

        resp = self.client.post("/auth/login", json={
            "username": username,
            "password": password,
        })
        token = resp.json()["token"]
        self.headers = {"Authorization": f"Bearer {token}"}

        resp = self.client.post("/profiles", json={
            "name": "Load Test Profile",
            "is_shared": True,
            "timezone": "UTC",
            "sleep_start": "23:00:00",
            "sleep_end": "07:00:00",
            "night_mode_enabled": False,
            "min_color_temp": 2700,
            "max_color_temp": 6500,
            "motion_timeout_seconds": 30,
        }, headers=self.headers)
        self.profile_id = resp.json()["id"]

        resp = self.client.post("/devices", json={
            "name": "Load Test Device",
            "profile_id": self.profile_id,
            "is_public": True,
        }, headers=self.headers)
        self.device_id = resp.json()["id"]
        self.device_key = resp.json()["secret_key"]

    def on_stop(self):
        self.client.delete("/auth/me", json={"password": "loadtest123"}, headers=self.headers)

    @task(8)
    def post_telemetry(self):
        self.client.post("/telemetry", json={
            "event_type": "motion_detected",
            "motion_detected": True,
            "light_is_on": False,
            "brightness": 75,
            "color_temp": 4000,
        }, headers={"x-api-key": self.device_key})

    @task(4)
    def get_circadian(self):
        self.client.get(
            "/profiles/circadian/" + str(self.profile_id),
            headers=self.headers,
            name="/profiles/circadian/[id]",
        )

    @task(2)
    def list_devices(self):
        self.client.get("/devices", headers=self.headers)

    @task(2)
    def list_profiles(self):
        self.client.get("/profiles", headers=self.headers)

    @task(1)
    def list_telemetry(self):
        self.client.get(
            "/telemetry?start=2020-01-01T00:00:00Z&end=2030-01-01T00:00:00Z",
            headers=self.headers,
            name="/telemetry",
        )

    @task(1)
    def get_me(self):
        self.client.get("/auth/me", headers=self.headers)
