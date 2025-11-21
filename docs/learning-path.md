# 🎓 RustyFlow IoT Learning Path

> Adım adım MQTT, Leptos ve IoT geliştirmeyi öğrenme rehberi

## 📖 İçindekiler
1. [MQTT Temelleri](#mqtt-temelleri)
2. [Rust Async Programlama](#rust-async-programlama)
3. [Leptos ve WASM](#leptos-ve-wasm)
4. [IoT Mimarisi](#iot-mimarisi)
5. [Pratik Projeler](#pratik-projeler)

---

## 1️⃣ MQTT Temelleri

### MQTT Nedir?
**Message Queue Telemetry Transport** - IoT cihazlar için hafif bir mesajlaşma protokolü

### Temel Kavramlar

#### Publisher (Yayıncı)
```rust
// Mesaj gönderen taraf
client.publish("sensors/temperature", "23.5", QoS::AtMostOnce)
```

#### Subscriber (Abone)
```rust
// Mesaj alan taraf
client.subscribe("sensors/#", QoS::AtMostOnce)
// # = wildcard (tüm alt topic'ler)
```

#### Broker (Aracı)
```
Mosquitto, EMQX, HiveMQ gibi sunucular
- Mesajları alır
- Abonemanlara göre dağıtır
- Mesaj geçmişini tutabilir (retained messages)
```

#### Topic (Konu)
```
Hiyerarşik yapı (URL gibi):
sensors/temperature          → Tek sensör
sensors/+/temperature        → Tüm cihazların sıcaklık sensörleri
sensors/#                    → Sensörler altındaki her şey
devices/rpi-01/status        → Belirli cihaz durumu
```

#### QoS (Quality of Service)
```
0 = At most once (En fazla bir kez) - Fire and forget
1 = At least once (En az bir kez) - Onay ile
2 = Exactly once (Tam bir kez) - Garantili
```

### Pratik Deney 1: Manuel MQTT Test

```bash
# Terminal 1: Subscribe (dinle)
mosquitto_sub -h localhost -t "sensors/#" -v

# Terminal 2: Publish (gönder)
mosquitto_pub -h localhost -t "sensors/temperature" -m "25.3"
mosquitto_pub -h localhost -t "sensors/humidity" -m "60.0"
mosquitto_pub -h localhost -t "devices/status" -m "online"

# Terminal 1'de mesajları göreceksin!
```

### Pratik Deney 2: Kendi MQTT Client'ını Yaz

Minimal örnek:
```rust
use rumqttc::{AsyncClient, MqttOptions, QoS};

#[tokio::main]
async fn main() {
    // Bağlantı ayarları
    let mut mqttoptions = MqttOptions::new("test-client", "localhost", 1883);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    // Client oluştur
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Subscribe ol
    client.subscribe("test/topic", QoS::AtMostOnce).await.unwrap();

    // Mesaj gönder
    client.publish("test/topic", QoS::AtMostOnce, false, "Hello MQTT!").await.unwrap();

    // Mesajları dinle
    loop {
        match eventloop.poll().await {
            Ok(notification) => println!("Event: {:?}", notification),
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

**Öğren:**
- `MqttOptions`: Bağlantı parametreleri
- `AsyncClient`: Mesaj gönderme
- `eventloop.poll()`: Mesaj alma
- `QoS`: Mesaj güvenilirliği

### Alıştırma 1: Basit Sıcaklık Yayıncısı
```rust
// Görev: Her 3 saniyede rastgele sıcaklık değeri gönder
// Topic: "home/living-room/temperature"
// Mesaj formatı: JSON {"value": 23.5, "unit": "C"}
```

---

## 2️⃣ Rust Async Programlama

### Async/Await Nedir?

**Senkron (Blocking):**
```rust
// Her adım sırayla, biri bitene kadar bekle
let data1 = read_file("a.txt");      // 2 saniye
let data2 = read_file("b.txt");      // 2 saniye
// Toplam: 4 saniye
```

**Asenkron (Non-blocking):**
```rust
// Paralel çalışabilir
let data1 = read_file("a.txt").await;  // Başlat
let data2 = read_file("b.txt").await;  // Paralel başlat
// Toplam: ~2 saniye (en yavaş olan kadar)
```

### Tokio Nedir?

Rust'un async runtime'ı (çalışma zamanı motoru):
```rust
#[tokio::main]  // Bu macro Tokio'yu başlatır
async fn main() {
    // Burada async fonksiyonlar çalışabilir
}
```

### Önemli Kavramlar

#### Future
```rust
// Future = "gelecekte tamamlanacak bir iş"
async fn fetch_data() -> String {
    // Asenkron işlem
    "data".to_string()
}

// Kullanım:
let result = fetch_data().await;
```

#### Task (Görev)
```rust
// Paralel görevler başlat
tokio::spawn(async {
    // Bu ayrı bir thread'de çalışır
    println!("Task 1");
});

tokio::spawn(async {
    println!("Task 2");
});
```

#### Channel (Kanal)
```rust
// Görevler arası haberleşme
let (tx, mut rx) = tokio::sync::mpsc::channel(100);

// Gönderici
tx.send("mesaj").await.unwrap();

// Alıcı
let msg = rx.recv().await;
```

### Alıştırma 2: Async Timer
```rust
// Görev: 3 farklı timer paralel çalıştır
// Timer 1: Her 1 saniyede "Tick"
// Timer 2: Her 2 saniyede "Tock"
// Timer 3: Her 5 saniyede "Boom"
```

---

## 3️⃣ Leptos ve WASM

### WebAssembly (WASM) Nedir?

**Analoji:**
```
JavaScript: Tarayıcının ana dili (yorumlanır, yavaş)
WASM: Tarayıcı için makine kodu (derlenmiş, hızlı)

Rust → WASM → Browser
      ^^^^
    Native hıza yakın!
```

### Leptos Nedir?

React/Vue benzeri bir Rust web framework'ü, ama WASM ile çalışır.

**Temel Özellikler:**

#### 1. Reactive Signals
```rust
// State oluştur (signal)
let (count, set_count) = create_signal(0);

// Oku
println!("Count: {}", count.get());

// Yaz
set_count.set(count.get() + 1);

// UI otomatik güncellenir!
```

#### 2. Components
```rust
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Increment"
            </button>
        </div>
    }
}
```

#### 3. Effects (Yan Etkiler)
```rust
// Signal değişince çalışır
create_effect(move |_| {
    println!("Count changed to: {}", count.get());
});
```

### RustyFlow Dashboard Analizi

**Adım adım nasıl çalışıyor:**

```rust
// 1. State oluştur
let (sensor_data, set_sensor_data) = create_signal(Vec::new());

// 2. API'den veri çek
let fetch_sensors = move || {
    spawn_local(async move {
        let data = api::fetch_sensor_data().await?;
        set_sensor_data.set(data);  // Signal'i güncelle
    });
};

// 3. Component mount olunca çalıştır
create_effect(move |_| {
    fetch_sensors();
});

// 4. Her 2 saniyede tekrarla
set_interval(fetch_sensors, Duration::from_secs(2));

// 5. UI render et
view! {
    <For
        each=move || sensor_data.get()
        key=|sensor| sensor.id.clone()
        children=|sensor| view! { <SensorCard sensor=sensor/> }
    />
}
```

### Alıştırma 3: Basit Counter App
```rust
// Görev: Leptos ile sayaç uygulaması
// - Sayıyı göster
// - + butonu (artır)
// - - butonu (azalt)
// - Reset butonu (sıfırla)
```

---

## 4️⃣ IoT Mimarisi

### Katmanlar

```
┌──────────────────────────────────┐
│  Presentation Layer (Dashboard)  │  ← Leptos/WASM
│  - Web interface                 │
│  - Mobile apps                   │
└──────────────────────────────────┘
               ↕ HTTP/REST
┌──────────────────────────────────┐
│  Application Layer (API Server)  │  ← Axum
│  - Business logic                │
│  - Data aggregation              │
│  - Authentication                │
└──────────────────────────────────┘
               ↕ Database
┌──────────────────────────────────┐
│  Data Layer (PostgreSQL)         │
│  - Persistent storage            │
│  - Historical data               │
└──────────────────────────────────┘
               ↕ MQTT
┌──────────────────────────────────┐
│  Message Layer (MQTT Gateway)    │  ← rumqttc
│  - Protocol translation          │
│  - Message routing               │
└──────────────────────────────────┘
               ↕ MQTT
┌──────────────────────────────────┐
│  Device Layer (Edge Agents)      │  ← Raspberry Pi
│  - Sensor reading                │
│  - Local processing              │
│  - Actuator control              │
└──────────────────────────────────┘
```

### Design Patterns

#### 1. Publisher-Subscriber Pattern
```
Edge Agent (Publisher)
    ↓ publish("sensors/temp", 23.5)
MQTT Broker
    ↓ forward to subscribers
Gateway (Subscriber)
```

#### 2. Request-Response via HTTP
```
Dashboard
    → GET /api/sensors
API Server
    ← JSON response
```

#### 3. Observer Pattern (Leptos)
```
Signal değişir
    → Effect tetiklenir
    → UI güncellenir
```

---

## 5️⃣ Pratik Projeler

### Proje 1: Oda Sıcaklık Monitörü (Temel)
**Öğreneceklerin:** MQTT basics, basit subscriber

```rust
// Görev:
// 1. MQTT'ye bağlan
// 2. "home/temperature" topic'ini dinle
// 3. Gelen değerleri terminale yazdır
// 4. Eğer >30°C ise uyarı ver
```

### Proje 2: Multi-Sensor Dashboard (Orta)
**Öğreneceklerin:** MQTT patterns, JSON parsing, Leptos basics

```rust
// Görev:
// 1. Birden fazla sensör topic'i dinle (temperature, humidity, light)
// 2. Verileri JSON olarak parse et
// 3. Leptos ile basit dashboard yap
// 4. Her sensör için ayrı kart göster
```

### Proje 3: Smart Home Controller (İleri)
**Öğreneceklerin:** Bi-directional MQTT, commands, state management

```rust
// Görev:
// 1. Sensörlerden veri al (temperature, motion)
// 2. Akıllı kurallar yaz (if temp > 30, turn on fan)
// 3. Dashboard'dan komut gönder (light on/off)
// 4. Cihaz durumlarını takip et
```

### Proje 4: RustyFlow'u Genişlet (Uzman)
**Öğreneceklerin:** Production patterns, scalability

```
Görev seçenekleri:
[ ] Redis cache entegrasyonu
[ ] WebSocket ile real-time updates
[ ] Grafana dashboard
[ ] Alarm sistemi (kritik değerlerde bildirim)
[ ] Time-series database (historical data)
```

---

## 📚 Önerilen Öğrenme Sırası

### Hafta 1-2: Temeller
- [ ] MQTT kavramlarını öğren (video: MQTT Essentials)
- [ ] mosquitto_pub/sub ile deney yap
- [ ] Basit Rust MQTT client yaz
- [ ] Pratik Deney 1 ve 2'yi tamamla

### Hafta 3-4: Rust Async
- [ ] Tokio dokumentasyonunu oku
- [ ] async/await öğren
- [ ] Basit async programlar yaz (timer, http request)
- [ ] Alıştırma 2'yi tamamla

### Hafta 5-6: Leptos
- [ ] Leptos Book'u oku (https://leptos-rs.github.io/leptos/)
- [ ] Counter örneğini yap
- [ ] Todo list uygulaması yap
- [ ] Alıştırma 3'ü tamamla

### Hafta 7-8: IoT Projesi
- [ ] Proje 1'i tamamla
- [ ] Proje 2'yi tamamla
- [ ] RustyFlow kodunu detaylı incele
- [ ] Her modülü tek tek çalıştırıp test et

### Hafta 9-10: Genişletme
- [ ] Kendi sensörünü ekle
- [ ] Dashboard'a yeni özellik ekle
- [ ] Raspberry Pi'ye deploy et
- [ ] Proje 3 veya 4'ü dene

---

## 🛠️ Geliştirme Araçları

### Gerekli Kurulumlar
```bash
# Rust toolchain
rustup target add wasm32-unknown-unknown

# Leptos CLI
cargo install trunk

# MQTT tools
brew install mosquitto  # macOS
sudo apt install mosquitto-clients  # Linux

# Database tools
cargo install sqlx-cli
```

### Faydalı VS Code Extensions
- rust-analyzer
- Even Better TOML
- Error Lens
- REST Client (API test için)

### Debug Araçları
```bash
# MQTT mesajlarını izle
mosquitto_sub -h localhost -t "#" -v

# HTTP API test
curl -X GET http://localhost:3000/api/sensors | jq

# Database sorguları
docker exec -it rustyflow_postgres psql -U postgres -d rustyflow
```

---

## 📖 Önerilen Kaynaklar

### MQTT
- 🎥 [MQTT Essentials (HiveMQ)](https://www.hivemq.com/mqtt-essentials/)
- 📚 [MQTT Specification](https://mqtt.org/mqtt-specification/)

### Rust Async
- 📚 [Asynchronous Programming in Rust Book](https://rust-lang.github.io/async-book/)
- 📚 [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Leptos
- 📚 [Leptos Book](https://leptos-rs.github.io/leptos/)
- 🎥 [Leptos Tutorial Videos](https://www.youtube.com/c/chrisbiscardi)
- 💬 [Leptos Discord](https://discord.gg/leptos)

### IoT with Rust
- 📚 "Rust for the IoT" (Packt Publishing)
- 🔗 [Embedded Rust Book](https://rust-embedded.github.io/book/)

---

## 💡 Öğrenme İpuçları

1. **Küçük adımlarla başla**: Önce MQTT'yi anla, sonra Leptos'a geç
2. **Kod yaz, oku değil**: Her kavramı mutlaka dene
3. **Hata mesajlarını oku**: Rust compiler'ı çok yardımcıdır
4. **Dokumentasyonu kullan**: `cargo doc --open` ile kendi projeni görüntüle
5. **Topluluk desteği**: Discord/Reddit'te soru sor
6. **Incremental development**: Her zaman çalışan bir versiyon tut

---

## 🎯 Başarı Kriterleri

Her seviye sonunda şunları yapabiliyor olmalısın:

### Temel Seviye ✅
- [ ] MQTT publish/subscribe yapabiliyorum
- [ ] Async fonksiyon yazabiliyorum
- [ ] Basit Leptos component oluşturabiliyorum

### Orta Seviye ✅
- [ ] Multi-topic MQTT dinliyorum
- [ ] JSON parse edip API'ye gönderiyorum
- [ ] Reactive state yönetiyorum

### İleri Seviye ✅
- [ ] Bi-directional MQTT iletişimi yapıyorum
- [ ] Karmaşık state yönetimi yapıyorum
- [ ] Production-ready kod yazabiliyorum

### Uzman Seviye 🚀
- [ ] Kendi IoT platformumu tasarlayabiliyorum
- [ ] Scalability sorunlarını çözebiliyorum
- [ ] Open source contribute edebiliyorum

---

**Not:** Bu yol haritası 2-3 aylık bir süreç. Acele etme, her adımı sindire sindire ilerle!

---

**Sorular?** RustyFlow Discord'da veya GitHub Issues'da sor!
