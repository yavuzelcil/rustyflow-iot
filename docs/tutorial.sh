#!/bin/bash

# 🎓 RustyFlow Interactive Tutorial
# Her adımı interaktif olarak öğret

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
cat << "EOF"
 ____            _         _____ _               
|  _ \ _   _ ___| |_ _   _|  ___| | _____      __
| |_) | | | / __| __| | | | |_  | |/ _ \ \ /\ / /
|  _ <| |_| \__ \ |_| |_| |  _| | | (_) \ V  V / 
|_| \_\\__,_|___/\__|\__, |_|   |_|\___/ \_/\_/  
                     |___/                        
     Interactive Learning Tutorial 🚀
EOF
echo -e "${NC}"

# Fonksiyonlar
ask_continue() {
    echo -e "\n${YELLOW}Devam etmek için Enter'a bas...${NC}"
    read
}

run_command() {
    local cmd="$1"
    local desc="$2"
    
    echo -e "\n${GREEN}📝 $desc${NC}"
    echo -e "${BLUE}$ $cmd${NC}"
    ask_continue
    eval "$cmd"
}

# Adım 1: MQTT Temelleri
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Adım 1: MQTT Temellerini Öğrenelim${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}MQTT nedir?${NC}"
echo "MQTT, IoT cihazlar için hafif bir mesajlaşma protokolüdür."
echo "3 ana parçası var:"
echo "  1. Publisher (Yayıncı) - Mesaj gönderen"
echo "  2. Broker (Aracı) - Mesajları ileten sunucu"
echo "  3. Subscriber (Abone) - Mesaj alan"

ask_continue

echo -e "\n${YELLOW}Topic nedir?${NC}"
echo "Topic, mesajların gönderildiği adrestir."
echo "Örnekler:"
echo "  sensors/temperature     → Tek bir sensör"
echo "  sensors/#               → Tüm sensörler (wildcard)"
echo "  devices/+/status        → Tüm cihazların durumu"

ask_continue

# Docker'ı başlat
run_command \
    "docker-compose up -d" \
    "MQTT broker'ı başlatıyoruz (Mosquitto)"

# Terminal 1: Subscribe
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Terminal Deneyi: MQTT'yi Görelim${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}Şimdi 2 terminal açacağız:${NC}"
echo "Terminal 1: Mesajları dinleyecek (subscriber)"
echo "Terminal 2: Mesaj gönderecek (publisher)"

ask_continue

echo -e "\n${GREEN}Terminal 1'de şunu çalıştır:${NC}"
echo -e "${BLUE}mosquitto_sub -h localhost -t 'tutorial/#' -v${NC}"
echo ""
echo "Bu komut 'tutorial/' ile başlayan tüm mesajları dinler"

ask_continue

echo -e "\n${GREEN}Terminal 2'de şunları dene:${NC}"
echo -e "${BLUE}mosquitto_pub -h localhost -t 'tutorial/hello' -m 'Merhaba MQTT!'${NC}"
echo -e "${BLUE}mosquitto_pub -h localhost -t 'tutorial/temp' -m '23.5'${NC}"
echo -e "${BLUE}mosquitto_pub -h localhost -t 'tutorial/data' -m '{\"sensor\": \"temp\", \"value\": 25}'${NC}"

echo -e "\n${YELLOW}Terminal 1'de mesajları göreceksin!${NC}"

ask_continue

# Adım 2: RustyFlow Bileşenleri
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Adım 2: RustyFlow Bileşenleri${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}Projede 4 ana bileşen var:${NC}"
echo ""
echo "1. 📱 Edge Agent (edge-agent/)"
echo "   - Sensörleri okur (temperature, humidity, motion)"
echo "   - MQTT'ye veri gönderir"
echo "   - Raspberry Pi'de çalışacak"
echo ""
echo "2. 🌉 MQTT Gateway (mqtt-gateway/)"
echo "   - MQTT mesajlarını dinler"
echo "   - API server'a HTTP ile iletir"
echo "   - Köprü görevi yapar"
echo ""
echo "3. 🔌 API Server (api-server/)"
echo "   - REST API sunar (/api/sensors)"
echo "   - Verileri cache'ler (in-memory)"
echo "   - Database'e yazabilir"
echo ""
echo "4. 🎨 Web Dashboard (web-dashboard/)"
echo "   - Leptos + WASM ile yazılmış"
echo "   - Real-time sensor gösterimi"
echo "   - Her 2 saniyede güncellenir"

ask_continue

# Her servisi başlat
echo -e "\n${GREEN}Servisleri tek tek başlatalım:${NC}"

run_command \
    "cargo run --bin api-server > /tmp/api-server.log 2>&1 &" \
    "API Server başlatılıyor (port 3000)"

sleep 3

run_command \
    "cargo run --bin mqtt-gateway > /tmp/mqtt-gateway.log 2>&1 &" \
    "MQTT Gateway başlatılıyor"

sleep 3

run_command \
    "cargo run --bin edge-agent > /tmp/edge-agent.log 2>&1 &" \
    "Edge Agent başlatılıyor (mock sensörler)"

sleep 5

# Logları göster
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Sistem Çalışıyor! Logları Görelim${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

run_command \
    "tail -5 /tmp/edge-agent.log" \
    "Edge Agent logu (sensör verileri)"

run_command \
    "tail -5 /tmp/mqtt-gateway.log" \
    "MQTT Gateway logu (mesaj iletimi)"

run_command \
    "curl -s http://localhost:3000/api/sensors | jq" \
    "API'den sensör verilerini çek"

# Dashboard'ı başlat
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Adım 3: Web Dashboard${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}Şimdi web dashboard'ı başlatalım${NC}"
echo "Leptos + WASM kullanarak tarayıcıda çalışacak"

ask_continue

run_command \
    "cd web-dashboard && trunk serve --port 8080 > /tmp/trunk.log 2>&1 &" \
    "Trunk ile dashboard'ı build edip serve et"

sleep 10

echo -e "\n${GREEN}✅ Dashboard hazır!${NC}"
echo -e "${BLUE}http://localhost:8080${NC} adresini tarayıcıda aç"
echo ""
echo "Göreceksin:"
echo "  🌡️  Temperature sensor (değişen değerler)"
echo "  💧 Humidity sensor (değişen değerler)"
echo "  🚶 Motion sensor (bazen detected)"

ask_continue

# Kod İnceleme
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Adım 4: Kod İnceleme${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}1. Edge Agent'ın sensör kodunu görelim:${NC}"
run_command \
    "cat edge-agent/src/sensors.rs | head -50" \
    "Mock sensör implementasyonu"

echo -e "\n${YELLOW}2. MQTT Gateway'in message handler'ını görelim:${NC}"
run_command \
    "grep -A 20 'async fn handle_message' mqtt-gateway/src/main.rs" \
    "MQTT mesaj işleyici"

echo -e "\n${YELLOW}3. Leptos component'ini görelim:${NC}"
run_command \
    "cat web-dashboard/src/components/sensor_card.rs | head -40" \
    "SensorCard component'i"

# Interaktif deney
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  Adım 5: Kendi Deneyin!${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}Şimdi sen dene:${NC}"
echo ""
echo "1. Manuel bir mesaj gönder:"
echo -e "   ${BLUE}mosquitto_pub -h localhost -t 'sensors/test/light' -m '{...}'${NC}"
echo ""
echo "2. Dashboard'da yeni bir sensör kartı gör"
echo ""
echo "3. Kod değiştir:"
echo "   - edge-agent/src/sensors.rs → Yeni sensör ekle"
echo "   - web-dashboard/src/components/sensor_card.rs → Görünümü değiştir"
echo ""
echo "4. Servisleri restart et:"
echo -e "   ${BLUE}pkill -f edge-agent && cargo run --bin edge-agent &${NC}"

ask_continue

# Özet
echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}  🎉 Tebrikler! Tutorial Tamamlandı${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

echo -e "\n${YELLOW}Öğrendiklerin:${NC}"
echo "✅ MQTT'nin nasıl çalıştığı (pub/sub pattern)"
echo "✅ IoT mimarisinin katmanları"
echo "✅ Rust async programlama (Tokio)"
echo "✅ Leptos ile reactive UI"
echo "✅ Docker Compose ile servis yönetimi"

echo -e "\n${YELLOW}Sıradaki adımlar:${NC}"
echo "1. 📚 docs/learning-path.md dosyasını oku"
echo "2. 💻 Her servisi ayrı ayrı incele"
echo "3. 🔧 Kendi sensörünü ekle"
echo "4. 🚀 Raspberry Pi'ye deploy et"

echo -e "\n${YELLOW}Kaynaklar:${NC}"
echo "- MQTT: https://www.hivemq.com/mqtt-essentials/"
echo "- Tokio: https://tokio.rs/tokio/tutorial"
echo "- Leptos: https://leptos-rs.github.io/leptos/"

echo -e "\n${GREEN}Servisleri durdurmak için:${NC}"
echo -e "${BLUE}pkill -f 'cargo run' && docker-compose down${NC}"

echo -e "\n${BLUE}Happy coding! 🦀${NC}\n"
