# 🗺️ RustyFlow IoT - Kod Bağlantı Haritası

## 📊 Servisler ve Portlar

```
┌─────────────────────────────────────────────────────────┐
│ Docker Compose (docker-compose.yml)                     │
├─────────────────────────────────────────────────────────┤
│ • PostgreSQL    → localhost:5432                        │
│ • Mosquitto     → localhost:1883 (MQTT)                 │
│                   localhost:9001 (WebSocket)            │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ Edge Agent (edge-agent/)          Port: N/A             │
├─────────────────────────────────────────────────────────┤
│ Bağlantılar:                                            │
│ → MQTT Broker (localhost:1883)                          │
│   Topics: sensors/edge-agent/{type}                     │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ MQTT Gateway (mqtt-gateway/)      Port: N/A             │
├─────────────────────────────────────────────────────────┤
│ Bağlantılar:                                            │
│ ← MQTT Broker (localhost:1883)                          │
│   Subscribe: sensors/#, devices/#                       │
│ → API Server (localhost:3000)                           │
│   POST /api/sensors                                     │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ API Server (api-server/)          Port: 3000            │
├─────────────────────────────────────────────────────────┤
│ Bağlantılar:                                            │
│ ← PostgreSQL (localhost:5432)                           │
│ ← MQTT Gateway (HTTP POST)                              │
│ ← Web Dashboard (HTTP GET)                              │
│                                                          │
│ Endpoints:                                              │
│ • GET  /                                                │
│ • GET  /health                                          │
│ • GET  /ready                                           │
│ • GET  /v1/config                                       │
│ • GET  /api/sensors                                     │
│ • POST /api/sensors                                     │
│ • POST /v1/media                                        │
│ • GET  /v1/media                                        │
│ • GET  /v1/media/{id}                                   │
│ • PUT  /v1/media/{id}                                   │
│ • DELETE /v1/media/{id}                                 │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ Web Dashboard (web-dashboard/)    Port: 8080            │
├─────────────────────────────────────────────────────────┤
│ Bağlantılar:                                            │
│ → API Server (localhost:3000)                           │
│   GET /api/sensors (every 2 seconds)                    │
└─────────────────────────────────────────────────────────┘
```

---

## 📂 Dosya Bağlantıları (Import/Export)

### 1. shared-types/ (Tüm servislerde kullanılır)

```
shared-types/src/lib.rs
├── pub mod media;      → Media, NewMedia, UpdateMedia
├── pub mod error;      → Error enum
├── pub mod sensor;     → Sensor, SensorReading
└── pub mod messages;   → MqttMessage, DeviceMessage, DeviceCommand

Kullanan Servisler:
├── api-server/         (sqlx-support = true)
├── mqtt-gateway/       (sqlx-support = true)
├── edge-agent/         (sqlx-support = true)
└── web-dashboard/      (sqlx-support = false, WASM için)
```

**Bağımlılıklar:**
```toml
# shared-types/Cargo.toml
uuid = { features = ["v4", "serde", "js"] }  # js = WASM uyumlu
sqlx = { optional = true }                    # Web için devre dışı
```

---

### 2. edge-agent/ → MQTT Broker

**Ana Dosya:** `edge-agent/src/main.rs`

```rust
Bağlantılar:
├── config.rs           // .env'den ayarları oku
├── sensors.rs          // Mock sensör implementasyonu
└── shared_types        // MqttMessage, SensorReading

Veri Akışı:
1. sensors::SensorController::read_all()
   ├── TemperatureSensor::read()
   ├── HumiditySensor::read()
   └── MotionSensor::read()
   
2. Her sensör için SensorData {
     sensor_type: String,
     reading: SensorReading,
   }

3. MqttMessage oluştur {
     message_type: "temperature_reading",
     payload: SensorReading (JSON),
     device_id: UUID,
     timestamp: DateTime<Utc>,
   }

4. MQTT'ye publish et:
   Topic: "sensors/edge-agent/{sensor_type}"
   Payload: JSON string
```

**Config:**
```rust
edge-agent/src/config.rs
└── .env'den okur:
    ├── MQTT_BROKER_HOST=localhost
    ├── MQTT_BROKER_PORT=1883
    ├── DEVICE_NAME=edge-agent
    ├── DEVICE_INTERVAL_SECS=5
    └── RUST_LOG=info
```

**Sensörler:**
```rust
edge-agent/src/sensors.rs
├── TemperatureSensor
│   └── range: 18.0 - 30.0°C
├── HumiditySensor
│   └── range: 30.0 - 80.0%
└── MotionSensor
    └── 20% detection probability
```

---

### 3. MQTT Broker (Mosquitto) - Merkez Hub

**Config:** `docker/mosquitto/mosquitto.conf`

```
Ayarlar:
├── listener 1883           # MQTT port
├── allow_anonymous true    # Development için açık
└── persistence true        # Mesajları sakla
```

**Topic Pattern:**
```
sensors/edge-agent/temperature
sensors/edge-agent/humidity
sensors/edge-agent/motion
devices/+/status
devices/+/commands
```

---

### 4. mqtt-gateway/ → API Server

**Ana Dosya:** `mqtt-gateway/src/main.rs`

```rust
Bağlantılar:
├── config.rs           // MQTT broker ayarları
├── shared_types        // MqttMessage, SensorReading
└── reqwest             // HTTP client (API'ye POST için)

Veri Akışı:
1. MQTT'den subscribe:
   Topics: "sensors/#", "devices/#"

2. Mesaj gelir → handle_message() çağrılır

3. JSON parse et:
   MqttMessage → SensorReading

4. SensorData oluştur {
     device_id: String (UUID'den),
     sensor_type: String (topic'ten),
     value: f64 (String'den parse),
     unit: String (sensor_type'a göre),
     timestamp: String (RFC3339),
     metadata: Option<Value>,
   }

5. API'ye HTTP POST:
   URL: http://localhost:3000/api/sensors
   Body: JSON(SensorData)
```

**Config:**
```rust
mqtt-gateway/src/config.rs
└── .env'den okur:
    ├── MQTT_BROKER_HOST=localhost
    ├── MQTT_BROKER_PORT=1883
    ├── MQTT_CLIENT_ID=rustyflow-gateway
    ├── MQTT_TOPICS=sensors/#,devices/#
    ├── API_SERVER_URL=http://localhost:3000
    └── RUST_LOG=info
```

---

### 5. api-server/ → PostgreSQL + Cache

**Ana Dosya:** `api-server/src/main.rs`

```rust
Bağlantılar:
├── config.rs           // .env ayarları
├── state.rs            // AppState (DB + in-memory)
├── routes/
│   ├── health.rs       // Sağlık kontrol
│   ├── sys.rs          // Sistem bilgisi
│   ├── media.rs        // Media CRUD
│   └── sensors.rs      // Sensör endpoints
└── shared_types        // Media, Sensor, Error

State Yapısı:
├── AppState {
│     cfg: Config,
│     media_store: Arc<RwLock<HashMap>>,  // Fallback
│     db: Option<PgPool>,                 // PostgreSQL
│   }
└── SensorCache = Arc<RwLock<HashMap<String, SensorData>>>
    Key: "device_id:sensor_type"
    Value: SensorData (en son veri)
```

**Endpoints ve Hangi Route:**

```rust
api-server/src/routes/health.rs
├── GET  /          → root()
├── GET  /health    → health()
└── GET  /ready     → ready()

api-server/src/routes/sys.rs
└── GET  /v1/config → config()

api-server/src/routes/media.rs (Database: media_datas table)
├── POST   /v1/media       → create_media()
├── GET    /v1/media       → list_media()
├── GET    /v1/media/{id}  → get_media()
├── PUT    /v1/media/{id}  → update_media()
└── DELETE /v1/media/{id}  → delete_media()

api-server/src/routes/sensors.rs (In-Memory: SensorCache)
├── GET  /api/sensors → list_sensors()
└── POST /api/sensors → add_sensor_data()
```

**Database Migration:**
```sql
api-server/migrations/20251025205807_media_init.sql
CREATE TABLE media_datas (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type TEXT DEFAULT 'application/octet-stream',
    size_bytes BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Config:**
```rust
api-server/src/config.rs
└── .env'den okur:
    ├── APP_PORT=3000
    ├── DATABASE_URL=postgres://postgres:pass@localhost:5432/rustyflow
    └── RUST_LOG=info
```

---

### 6. web-dashboard/ → API Server

**Ana Dosya:** `web-dashboard/src/main.rs`

```rust
Bağlantılar:
├── api.rs              // HTTP client (gloo-net)
├── components/
│   └── sensor_card.rs  // Sensör kartları
└── shared_types        // SensorData (sqlx devre dışı)

Veri Akışı:
1. App component mount olur

2. create_effect → fetch_sensors() çağrılır

3. api::fetch_sensor_data() {
     URL: http://localhost:3000/api/sensors
     Method: GET
     Returns: Vec<SensorData>
   }

4. Signal güncellenir:
   set_sensor_data(data)

5. UI otomatik render:
   For loop → Her SensorData için SensorCard

6. set_interval (2 seconds):
   fetch_sensors() tekrar çağrılır
```

**Components:**
```rust
web-dashboard/src/components/sensor_card.rs

#[component]
fn SensorCard(sensor: SensorData) {
    Props:
    ├── device_id: String
    ├── sensor_type: String ("temperature", "humidity", "motion")
    ├── value: f64
    ├── unit: String ("°C", "%", "bool")
    ├── timestamp: String
    └── metadata: Option<Value>

    Render:
    ├── sensor_type = "temperature" → Kırmızı kart
    ├── sensor_type = "humidity"    → Mavi kart
    └── sensor_type = "motion"      → Yeşil kart (animasyonlu)
}
```

**CSS:**
```
web-dashboard/style.css
├── .sensor-card           # Kart stili
├── .sensor-card.temperature  # Kırmızı renk
├── .sensor-card.humidity     # Mavi renk
├── .sensor-card.motion       # Yeşil renk + animasyon
└── @keyframes pulse       # Motion animasyonu
```

---

## 🔄 Veri Akış Diyagramı (Detaylı)

```
┌─────────────────────────────────────────────────────────────────┐
│ EDGE AGENT (edge-agent/src/main.rs)                            │
│                                                                 │
│ Timer (her 5 saniye):                                          │
│   1. sensors::SensorController::read_all()                     │
│      ├── TemperatureSensor::read() → 23.5                      │
│      ├── HumiditySensor::read() → 62.3                         │
│      └── MotionSensor::read() → 1 (detected)                   │
│                                                                 │
│   2. Her sensör için MqttMessage oluştur:                      │
│      {                                                          │
│        message_type: "temperature_reading",                    │
│        payload: { sensor_id, value, timestamp, ... },          │
│        device_id: UUID,                                         │
│        timestamp: DateTime<Utc>,                                │
│      }                                                          │
│                                                                 │
│   3. MQTT Publish:                                             │
│      Topic: "sensors/edge-agent/temperature"                   │
│      Payload: JSON string                                      │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ↓ MQTT Protocol
┌─────────────────────────────────────────────────────────────────┐
│ MOSQUITTO BROKER (Docker port 1883)                            │
│                                                                 │
│ Topics:                                                         │
│   • sensors/edge-agent/temperature                             │
│   • sensors/edge-agent/humidity                                │
│   • sensors/edge-agent/motion                                  │
│                                                                 │
│ Subscribers:                                                    │
│   • mqtt-gateway (pattern: sensors/#)                          │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ↓ MQTT Subscription
┌─────────────────────────────────────────────────────────────────┐
│ MQTT GATEWAY (mqtt-gateway/src/main.rs)                        │
│                                                                 │
│ eventloop.poll() → Incoming Message:                           │
│   1. handle_message(topic, payload)                            │
│                                                                 │
│   2. JSON parse:                                               │
│      payload → MqttMessage                                      │
│      MqttMessage.payload → SensorReading                        │
│                                                                 │
│   3. Transform:                                                │
│      SensorReading → SensorData {                              │
│        device_id: "UUID string",                               │
│        sensor_type: "temperature" (from topic),                │
│        value: 23.5 (parse from string),                        │
│        unit: "°C" (deduce from type),                          │
│        timestamp: "2025-11-21T10:30:00Z",                      │
│      }                                                          │
│                                                                 │
│   4. HTTP POST to API:                                         │
│      reqwest::post("http://localhost:3000/api/sensors")        │
│      .json(&sensor_data)                                        │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ↓ HTTP POST
┌─────────────────────────────────────────────────────────────────┐
│ API SERVER (api-server/src/main.rs)                            │
│                                                                 │
│ POST /api/sensors:                                             │
│   1. routes/sensors.rs::add_sensor_data()                      │
│                                                                 │
│   2. Cache'e kaydet:                                           │
│      Key: "device_id:sensor_type"                              │
│      Value: SensorData                                          │
│      Storage: Arc<RwLock<HashMap>>                             │
│                                                                 │
│   3. Return: 200 OK                                            │
│                                                                 │
│ GET /api/sensors:                                              │
│   1. routes/sensors.rs::list_sensors()                         │
│                                                                 │
│   2. Cache'ten oku:                                            │
│      HashMap.values() → Vec<SensorData>                        │
│                                                                 │
│   3. Return: JSON array                                        │
│      [                                                          │
│        {device_id, sensor_type, value, unit, ...},             │
│        {device_id, sensor_type, value, unit, ...},             │
│      ]                                                          │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ↓ HTTP GET (her 2 saniye)
┌─────────────────────────────────────────────────────────────────┐
│ WEB DASHBOARD (web-dashboard/src/main.rs)                      │
│                                                                 │
│ Leptos App Component:                                          │
│   1. create_signal(Vec::new()) → sensor_data                   │
│                                                                 │
│   2. create_effect:                                            │
│      fetch_sensors() {                                          │
│        api::fetch_sensor_data().await                          │
│        → Vec<SensorData>                                        │
│        set_sensor_data(data)                                    │
│      }                                                          │
│                                                                 │
│   3. set_interval(2 secs):                                     │
│      fetch_sensors() tekrar çağrılır                           │
│                                                                 │
│   4. UI Render:                                                │
│      <For each=sensor_data>                                    │
│        <SensorCard sensor={sensor}/>                           │
│      </For>                                                     │
│                                                                 │
│ Browser'da:                                                     │
│   • WASM çalışır (native hıza yakın)                          │
│   • DOM manipülasyonu (reactive)                               │
│   • CSS animasyonlar                                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔗 Dependency Graph

```
shared-types (lib)
    ↑ ↑ ↑ ↑
    │ │ │ └─────────────────┐
    │ │ │                   │
    │ │ └─────────┐         │
    │ │           │         │
edge-agent    mqtt-gateway  api-server    web-dashboard
    ↓             ↓            ↓               ↓
MQTT Broker ──────┘            │               │
    │                          │               │
    └──────────────────────────┘               │
                               │               │
                        PostgreSQL             │
                                               │
                                HTTP REST ─────┘
```

---

## 🗂️ Kritik Dosyalar Listesi

### Configuration Files
```
├── .env                           # Tüm servisler için ayarlar
├── Cargo.toml                     # Workspace tanımı
├── docker-compose.yml             # PostgreSQL + Mosquitto
└── docker/mosquitto/mosquitto.conf # MQTT broker config
```

### Shared Types
```
shared-types/
├── Cargo.toml                     # Optional SQLx, WASM features
├── src/lib.rs                     # Public exports
├── src/media.rs                   # Media, NewMedia, UpdateMedia
├── src/error.rs                   # Error enum + conversions
├── src/sensor.rs                  # Sensor, SensorReading
└── src/messages.rs                # MqttMessage, DeviceMessage, DeviceCommand
```

### Edge Agent
```
edge-agent/
├── Cargo.toml                     # rumqttc, shared-types
├── src/main.rs                    # Timer loop + MQTT publish
├── src/config.rs                  # MQTT broker config
└── src/sensors.rs                 # Mock sensors (temp, humidity, motion)
```

### MQTT Gateway
```
mqtt-gateway/
├── Cargo.toml                     # rumqttc, reqwest, shared-types
├── src/main.rs                    # Subscribe + forward to API
└── src/config.rs                  # MQTT + API config
```

### API Server
```
api-server/
├── Cargo.toml                     # axum, sqlx, tower-http
├── src/main.rs                    # Router + CORS + State
├── src/config.rs                  # App config (.env)
├── src/state.rs                   # AppState (DB + in-memory)
├── src/routes/
│   ├── mod.rs                     # Module exports
│   ├── health.rs                  # /, /health, /ready
│   ├── sys.rs                     # /v1/config
│   ├── media.rs                   # /v1/media/* (DB)
│   └── sensors.rs                 # /api/sensors (cache)
└── migrations/
    └── 20251025205807_media_init.sql # CREATE TABLE
```

### Web Dashboard
```
web-dashboard/
├── Cargo.toml                     # leptos, gloo-net, shared-types
├── index.html                     # HTML shell
├── style.css                      # Component styles
├── src/main.rs                    # App component + timer
├── src/api.rs                     # HTTP client (fetch_sensor_data)
└── src/components/
    ├── mod.rs                     # Component exports
    └── sensor_card.rs             # SensorCard component
```

---

## 📌 Port Summary

| Service        | Port | Protocol | Purpose                    |
|----------------|------|----------|----------------------------|
| PostgreSQL     | 5432 | TCP      | Database                   |
| Mosquitto      | 1883 | MQTT     | Message broker             |
| Mosquitto WS   | 9001 | WebSocket| Browser MQTT (unused)      |
| API Server     | 3000 | HTTP     | REST API                   |
| Web Dashboard  | 8080 | HTTP     | Frontend (trunk serve)     |

---

## 🎯 Data Types Mapping

```
Edge Agent (SensorReading)
    ↓
MQTT (JSON string)
    ↓
Gateway (SensorReading → SensorData)
    ↓
API (SensorData in cache)
    ↓
Dashboard (SensorData in UI)
```

### SensorReading (shared-types)
```rust
{
    sensor_id: UUID,
    value: String,          // "23.5" or "1"
    timestamp: DateTime<Utc>,
    is_valid: bool,
    metadata: Option<Value>,
}
```

### SensorData (API + Dashboard)
```rust
{
    device_id: String,      // UUID as string
    sensor_type: String,    // "temperature", "humidity", "motion"
    value: f64,             // 23.5 (parsed)
    unit: String,           // "°C", "%", "bool"
    timestamp: String,      // RFC3339 format
    metadata: Option<Value>,
}
```

---

Bu harita ile artık net bir diagram çizebilirsin! Başka detay lazım mı?
