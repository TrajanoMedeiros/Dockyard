# Rustainer

Rustainer is an open-source container controller and lightweight orchestration tool written in Rust.

It is designed as a learning-focused project in systems programming, containerization, and distributed systems. The project evolves from a simple Docker CLI into a minimal orchestration platform.

---

## Overview

Rustainer provides a simple way to manage and orchestrate containers through a command-line interface, configuration files, and future API and dashboard components.

It begins by controlling Docker containers and gradually expands toward features inspired by tools such as Docker Compose and Kubernetes.

---

## Features (MVP)

* List running containers
* Start, stop, and restart containers
* View container logs
* Simple command-line interface
* Docker integration

---

## Goals

Rustainer is built to:

* Learn Rust in a real-world systems project
* Understand container internals
* Practice asynchronous programming
* Explore Linux and networking concepts
* Study orchestration and distributed systems
* Build a strong backend and infrastructure portfolio project

---

## Tech Stack

* Rust
* Tokio (async runtime)
* Clap (CLI parsing)
* Bollard (Docker API)
* Serde (serialization)
* Axum (planned API layer)
* SQLx (planned persistence layer)

---

## Installation

Requirements: Rust and Docker installed

```bash id="a1b2c3"
git clone https://github.com/your-username/rustainer.git
cd rustainer
cargo build
```

Run:

```bash id="d4e5f6"
cargo run -- ps
```

---

## Usage

List containers:

```bash id="g7h8i9"
rustainer ps
```

Start a container:

```bash id="j1k2l3"
rustainer start nginx
```

Stop a container:

```bash id="m4n5o6"
rustainer stop nginx
```

View logs:

```bash id="p7q8r9"
rustainer logs nginx
```

Deploy from configuration (future feature):

```bash id="s1t2u3"
rustainer apply stack.yml
```

---

## Project Structure

```text id="v4w5x6"
rustainer/
├── cli/
├── core/
├── daemon/
├── api/
├── storage/
├── scheduler/
└── config/
```

---

## Roadmap

### Phase 1: CLI and Docker Control

* Basic CLI implementation
* Container lifecycle management
* Docker API integration

### Phase 2: Core Engine

* Internal container state model
* Improved error handling
* CLI improvements

### Phase 3: Declarative Deployments

* YAML configuration support
* Service definitions
* Multi-container stacks

### Phase 4: Daemon Mode

* Background service
* Automatic recovery of failed containers
* State reconciliation loop

### Phase 5: API Layer

* REST API using Axum
* External control interface

### Phase 6: Observability

* Logging system
* Metrics collection
* Health checks

### Phase 7: Advanced Orchestration

* Multi-node support
* Scheduling system
* Scaling and replication

---

## Vision

Rustainer is not intended to replace Docker or Kubernetes. Its purpose is to provide a minimal and educational implementation of container orchestration concepts.

The goal is to help developers understand how these systems work internally by building a simplified version from scratch.

---

## Contributing

Contributions are welcome.

To contribute:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

Please keep contributions focused, well-documented, and aligned with the educational purpose of the project.

---

## License

This project is licensed under the MIT License.
