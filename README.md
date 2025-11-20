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

### 🚧 In Progress
- None currently!

### 📋 Planned
- [ ] Machine learning model integration
- [ ] Camera and OpenCV integration
- [ ] Real-time dashboard
- [ ] Kubernetes deployment
- [ ] CI/CD pipeline

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
