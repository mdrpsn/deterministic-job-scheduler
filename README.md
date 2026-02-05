# Deterministic Job Scheduler (Rust)

A **deterministic, crash-safe job scheduler and execution engine** implemented in Rust.

This project demonstrates how to build **infrastructure-grade background workers** with:
- Explicit state management
- Bounded concurrency
- Deterministic scheduling
- Crash-safe behavior
- Clear failure modes

There is **no UI** and no SaaS layer.  
This is a backend control-plane component.

---

## What this does (plain English)

- Watches a database table for jobs
- Selects a limited number of jobs at a time
- Executes them in the background
- Records success or failure
- Repeats continuously

Think of it as a **small engine that safely pulls work from a database and runs it in controlled batches**.

---

## Core Properties

- **Deterministic scheduling**
  - Jobs are selected in a predictable order
  - Concurrency is strictly bounded

- **Crash-safe**
  - Jobs in progress are not lost
  - Failures are explicit and recorded

- **Fail-fast configuration**
  - Missing critical config causes startup failure
  - No silent misconfiguration

- **Infrastructure-first**
  - No framework magic
  - No hidden state
  - Clear ownership boundaries

---

## Architecture Overview

- **Domain layer** – job state, transitions, failure types
- **Scheduler** – pure, deterministic job selection
- **Executor** – bounded async execution with timeouts
- **Storage** – PostgreSQL-backed persistence
- **Orchestrator** – control loop that ties everything together

This is a single-node v1 designed to be extended, not a finished product.

---

## Running locally

### Requirements
- Rust (stable)
- PostgreSQL (local or Docker)

### Environment variables

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/deterministic_scheduler
MAX_CONCURRENCY=10
SCHEDULER_TICK_MS=500
JOB_TIMEOUT_SECS=5
RUST_LOG=info
