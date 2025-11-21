# 🦀 RustyFlow IoT Platform

> A modern, full-stack IoT platform built entirely in Rust - from edge devices to cloud services with integrated machine learning capabilities.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

## 🎯 Project Vision

RustyFlow is a comprehensive IoT platform that demonstrates the power of Rust across the entire stack:
- **Edge Computing**: Raspberry Pi agents collecting sensor data
- **Message Broker**: MQTT gateway for reliable device communication  
- **Backend Services**: Microservices architecture for data processing
- **Machine Learning**: Rust-native ML models for anomaly detection and predictions
- **API Layer**: Modern GraphQL and REST APIs
- **Web Dashboard**: Real-time monitoring and control interface

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Cloud Platform                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  API Server  │  │  ML Service  │  │ Data Store   │      │
│  │   (Axum)     │  │  (Burn/Linfa)│  │ (PostgreSQL) │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│           │                 │                  │             │
│           └─────────────────┴──────────────────┘             │
│                           │                                  │
│                  ┌────────▼────────┐                         │
│                  │  MQTT Gateway   │                         │
│                  │   (Rumqttc)     │                         │
│                  └────────┬────────┘                         │
└───────────────────────────┼──────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
    ┌─────────▼────────┐      ┌──────────▼─────────┐
    │  Edge Agent #1   │      │   Edge Agent #2    │
    │  (Raspberry Pi)  │      │  (Raspberry Pi)    │
    │  • Camera        │      │  • Sensors         │
    │  • Sensors       │      │  • GPIO Controls   │
    └──────────────────┘      └────────────────────┘
```

## 🛠️ Tech Stack

### Core Technologies
- **Language**: Rust 1.75+ (Edition 2021)
- **Async Runtime**: Tokio 1.40
- **Error Handling**: thiserror + anyhow

### Edge Computing
- **GPIO**: rppal 0.18
- **Camera**: opencv-rust 0.92
- **Sensors**: embedded-hal compatible drivers
- **Communication**: rumqttc 0.24

### Backend Services
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL with SQLx 0.8
- **Message Queue**: MQTT (Mosquitto/EMQX)
- **GraphQL**: async-graphql 7.0

### Machine Learning
- **Framework**: Burn 0.14 (Deep Learning)
- **Classical ML**: Linfa 0.7
- **Data Processing**: ndarray, polars

### DevOps
- **Containerization**: Docker + Docker Compose
- **Orchestration**: Kubernetes (optional)
- **Monitoring**: Prometheus + Grafana
- **Logging**: tracing + tracing-subscriber

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies (Ubuntu/Debian)
sudo apt-get install -y build-essential pkg-config libssl-dev \
    postgresql-client libopencv-dev
```

### Development Setup
```bash
# Clone repository
git clone https://github.com/yourusername/rustyflow-iot.git
cd rustyflow-iot

# Start infrastructure (PostgreSQL, MQTT broker)
docker-compose up -d

# Run database migrations
cd api-server
sqlx migrate run

# Start API server
cargo run --bin api-server

# In another terminal, start MQTT gateway
cargo run --bin mqtt-gateway
```

## 📦 Project Structure

```
rustyflow-iot/
├── shared-types/          # Common data models and traits
├── edge-agent/            # Raspberry Pi edge application
├── mqtt-gateway/          # MQTT message broker service
├── api-server/            # REST and GraphQL API
├── ml-service/            # Machine learning inference service
├── web-dashboard/         # Frontend (future)
├── docker/                # Dockerfiles for each service
├── k8s/                   # Kubernetes manifests
└── docs/                  # Additional documentation
```

## 🎓 Learning Resources

This project is inspired by and builds upon concepts from:
- [Rust for IoT Book](https://www.packtpub.com/product/rust-for-the-iot/9781805120797)
- Modern Rust best practices (2024+)
- Microservices architecture patterns

## 🗺️ Roadmap

### ✅ Completed
- [x] Project structure and basic setup
- [x] Core shared types and error handling
  - Media, Sensor, Error types with comprehensive documentation
  - MQTT message structures (MqttMessage, DeviceMessage, DeviceCommand)
  - Common Result type and error conversions
- [x] API server with REST endpoints
  - Axum-based REST API with health checks
  - Media CRUD operations (POST, GET, PUT, DELETE)
  - System configuration endpoints
  - Comprehensive doc comments ("mala anlatır gibi")
- [x] PostgreSQL integration
  - Docker Compose setup with PostgreSQL 16
  - SQLx migrations with UUID support
  - Connection pooling and fallback to in-memory storage
  - Full 7-field Media schema with timestamps
- [x] MQTT gateway with basic pub/sub
  - rumqttc async MQTT client
  - Multi-topic subscription with wildcard support (sensors/#, devices/#)
  - Message parsing with shared-types::MqttMessage
  - Auto-reconnect with backoff
  - Mosquitto broker in Docker
- [x] Edge agent with mock sensors
  - Mock sensors: temperature, humidity, motion (PIR)
  - Realistic data generation with gradual value changes
  - Periodic MQTT publishing (configurable interval)
  - Ready for real sensor integration (rppal/embedded-hal)
  - Tested end-to-end with gateway

### ✅ Recently Completed
- [x] **Web Dashboard with Leptos + WASM**
  - Real-time sensor monitoring interface
  - Auto-refresh every 2 seconds
  - Responsive design with CSS animations
  - Temperature, humidity, motion sensor cards
  - Full Rust stack (backend + frontend)
- [x] **API Sensor Endpoints**
  - GET/POST /api/sensors for real-time data
  - In-memory cache for sensor readings
  - CORS support for web dashboard
- [x] **MQTT Gateway → API Integration**
  - Forward sensor data from MQTT to REST API
  - HTTP client with reqwest
  - Automatic sensor type and unit detection
  - Complete data flow: Edge → MQTT → Gateway → API → Dashboard

### 🚧 In Progress
- [ ] Production hardening (see roadmap below)

### 📋 Short-term Goals
- [ ] Deploy to Raspberry Pi 5 (8GB RAM)
- [ ] Real sensor integration (DHT22, PIR, camera)
- [ ] Persistent sensor data storage
- [ ] Redis integration for distributed caching

### 🎯 Long-term Vision
- [ ] Machine learning model integration
- [ ] Advanced camera features with OpenCV
- [ ] Kubernetes deployment
- [ ] Multi-tenant support
- [ ] CI/CD pipeline

## 🏭 Production Readiness Roadmap

### ⚠️ Current Status: Development/Prototype
This project is **not production-ready** yet. Below are the gaps that need to be addressed:

#### 🔐 Security
- [ ] **Authentication & Authorization**
  - JWT token-based authentication
  - Role-based access control (RBAC)
  - API key management for IoT devices
- [ ] **TLS/HTTPS**
  - SSL certificates for all services
  - Encrypted MQTT connections (MQTTS)
  - Secure WebSocket connections
- [ ] **Input Validation**
  - Rate limiting per IP/user
  - Request size limits
  - SQL injection prevention (already using SQLx with compile-time checks)

#### 📊 Observability
- [ ] **Monitoring & Metrics**
  - Prometheus metrics export
  - Grafana dashboards
  - Custom business metrics (sensor readings/min, API latency, etc.)
- [ ] **Logging**
  - Structured logging (JSON format)
  - Centralized log aggregation (ELK/Loki)
  - Log rotation and retention policies
- [ ] **Alerting**
  - Alert manager integration
  - Slack/PagerDuty notifications
  - Anomaly detection alerts

#### 💾 Data Persistence
- [ ] **Sensor Data Storage**
  - Time-series database (TimescaleDB/InfluxDB)
  - Historical data retention policies
  - Data archival strategy
- [ ] **Caching Layer**
  - Redis for distributed caching
  - Cache invalidation strategies
  - TTL policies for sensor data
- [ ] **Backups**
  - Automated database backups
  - Point-in-time recovery
  - Disaster recovery plan

#### 🚀 Scalability
- [ ] **Horizontal Scaling**
  - Load balancer (nginx/traefik)
  - Multiple API server instances
  - Session management for stateless APIs
- [ ] **Message Queue**
  - Kafka/RabbitMQ for event streaming
  - Dead letter queues for failed messages
  - Message replay capability
- [ ] **Database Optimization**
  - Connection pooling tuning
  - Database indexing strategy
  - Read replicas for scaling reads

#### 🛠️ Reliability
- [ ] **Error Handling**
  - Graceful degradation
  - Circuit breakers for external services
  - Retry mechanisms with exponential backoff
- [ ] **Health Checks**
  - Kubernetes liveness/readiness probes
  - Deep health checks (database, MQTT, cache)
  - Service mesh integration
- [ ] **Testing**
  - Integration tests for all services
  - Load testing (locust/k6)
  - Chaos engineering experiments

#### 📝 Documentation
- [ ] **API Documentation**
  - OpenAPI/Swagger specs
  - Interactive API explorer
  - Code examples for common use cases
- [ ] **Deployment Guide**
  - Step-by-step deployment instructions
  - Environment configuration guide
  - Troubleshooting documentation
- [ ] **Architecture Decision Records (ADRs)**
  - Document key technical decisions
  - Trade-offs and alternatives considered

#### 🔄 DevOps
- [ ] **CI/CD Pipeline**
  - Automated testing on PR
  - Docker image building and scanning
  - Automated deployment to staging/production
- [ ] **Infrastructure as Code**
  - Terraform/Pulumi for cloud resources
  - GitOps workflow (ArgoCD/Flux)
  - Environment parity (dev/staging/prod)

### 📈 Current Development Focus
1. **Raspberry Pi Deployment**: Test full stack on real hardware
2. **Real Sensor Integration**: Replace mock sensors with actual hardware
3. **Redis Cache**: Implement distributed caching for sensor data
4. **Time-series DB**: Add TimescaleDB for historical sensor data

---

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file.

## 🙏 Acknowledgments

- Rust community for amazing crates
- IoT and edge computing enthusiasts
- Contributors and supporters

---

**Status**: 🚧 Active Development | **Version**: 0.1.0 | **Rust**: 1.75+
