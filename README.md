# Qubit Atomic

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-atomic.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-atomic)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-atomic/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-atomic?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-atomic.svg?color=blue)](https://crates.io/crates/qubit-atomic)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

User-friendly atomic operations wrapper providing JDK-like atomic API for Rust.

## Overview

Qubit Atomic is a comprehensive atomic operations library that provides easy-to-use atomic types with reasonable default memory orderings, similar to Java's `java.util.concurrent.atomic` package. It hides the complexity of memory ordering while maintaining zero-cost abstraction and allowing advanced users to access underlying types for fine-grained control.

## Design Goals

- **Ease of Use**: Hides memory ordering complexity with reasonable defaults
- **Completeness**: Provides high-level operations similar to JDK atomic classes
- **Safety**: Guarantees memory safety and thread safety
- **Performance**: Zero-cost abstraction with no additional overhead
- **Flexibility**: Exposes underlying types via `inner()` for advanced users
- **Simplicity**: Minimal API surface without `_with_ordering` variants

## Features

### 🔢 **Atomic Integer Types**
- **Signed Integers**: `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicIsize`
- **Unsigned Integers**: `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicUsize`
- **Rich Operations**: increment, decrement, add, subtract, multiply, divide, bitwise operations, max/min
- **Functional Updates**: `fetch_update`, `fetch_accumulate`

### 🔘 **Atomic Boolean Type**
- **AtomicBool**: Boolean atomic operations
- **Special Operations**: set, clear, negate, logical AND/OR/XOR
- **Conditional CAS**: `set_if_false`, `set_if_true`

### 🔢 **Atomic Floating-Point Types**
- **AtomicF32/AtomicF64**: 32-bit and 64-bit floating-point atomics
- **Arithmetic Operations**: `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div` (via CAS loop)
- **Functional Updates**: Custom operations via closures

### 🔗 **Atomic Reference Type**
- **AtomicRef<T>**: Thread-safe atomic reference using `Arc<T>`
- **Reference Updates**: Atomic swap and CAS operations
- **Functional Updates**: Transform references atomically

### 🎯 **Trait Abstractions**
- **Atomic**: Common atomic operations trait (includes `fetch_update`)
- **AtomicNumber**: Arithmetic operations trait for numeric types (integers and floats)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-atomic = "0.7.1"
```

## Quick Start

### Basic Counter

```rust
use qubit_atomic::AtomicI32;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    // Spawn 10 threads, each increments counter 1000 times
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_inc();
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify result
    assert_eq!(counter.load(), 10000);
    println!("Final count: {}", counter.load());
}
```

### CAS Loop

```rust
use qubit_atomic::AtomicI32;

fn increment_even_only(atomic: &AtomicI32) -> Result<i32, &'static str> {
    let mut current = atomic.load();
    loop {
        // Only increment even values
        if current % 2 != 0 {
            return Err("Value is odd");
        }

        let new = current + 2;
        match atomic.compare_set(current, new) {
            Ok(_) => return Ok(new),
            Err(actual) => current = actual, // Retry
        }
    }
}

fn main() {
    let atomic = AtomicI32::new(10);
    match increment_even_only(&atomic) {
        Ok(new_value) => println!("Successfully incremented to: {}", new_value),
        Err(e) => println!("Failed: {}", e),
    }
    assert_eq!(atomic.load(), 12);
}
```

### Functional Updates

```rust
use qubit_atomic::AtomicI32;

fn main() {
    let atomic = AtomicI32::new(10);

    // Update using a function (returns old value)
    let old_value = atomic.fetch_update(|x| {
        if x < 100 {
            x * 2
        } else {
            x
        }
    });

    assert_eq!(old_value, 10);
    assert_eq!(atomic.load(), 20);
    println!("Updated value: {}", atomic.load());

    // Accumulate operation (returns old value)
    let old_result = atomic.fetch_accumulate(5, |a, b| a + b);
    assert_eq!(old_result, 20);
    assert_eq!(atomic.load(), 25);
    println!("Accumulated value: {}", atomic.load());
}
```

### Atomic Reference

```rust
use qubit_atomic::AtomicRef;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Config {
    timeout: u64,
    max_retries: u32,
}

fn main() {
    let config = Arc::new(Config {
        timeout: 1000,
        max_retries: 3,
    });

    let atomic_config = AtomicRef::new(config);

    // Update configuration
    let new_config = Arc::new(Config {
        timeout: 2000,
        max_retries: 5,
    });

    let old_config = atomic_config.swap(new_config);
    println!("Old config: {:?}", old_config);
    println!("New config: {:?}", atomic_config.load());

    // Update using a function (returns old value)
    let old = atomic_config.fetch_update(|current| {
        Arc::new(Config {
            timeout: current.timeout * 2,
            max_retries: current.max_retries + 1,
        })
    });

    println!("Previous config: {:?}", old);
    println!("Updated config: {:?}", atomic_config.load());
}
```

### Boolean Flag

```rust
use qubit_atomic::AtomicBool;
use std::sync::Arc;

struct Service {
    running: Arc<AtomicBool>,
}

impl Service {
    fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn start(&self) {
        // Only start if not already running
        if self.running.set_if_false(true).is_ok() {
            println!("Service started successfully");
        } else {
            println!("Service is already running");
        }
    }

    fn stop(&self) {
        // Only stop if currently running
        if self.running.set_if_true(false).is_ok() {
            println!("Service stopped successfully");
        } else {
            println!("Service is already stopped");
        }
    }

    fn is_running(&self) -> bool {
        self.running.load()
    }
}

fn main() {
    let service = Service::new();

    service.start();
    assert!(service.is_running());

    service.start(); // Duplicate start will fail

    service.stop();
    assert!(!service.is_running());

    service.stop(); // Duplicate stop will fail
}
```

### Floating-Point Atomics

```rust
use qubit_atomic::AtomicF32;
use std::sync::Arc;
use std::thread;

fn main() {
    let sum = Arc::new(AtomicF32::new(0.0));
    let mut handles = vec![];

    // Spawn 10 threads, each adds 100 times
    for _ in 0..10 {
        let sum = sum.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                sum.add(0.01);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Note: Due to floating-point precision, result may not be exactly 10.0
    let result = sum.load();
    println!("Sum: {:.6}", result);
    println!("Error: {:.6}", (result - 10.0).abs());
}
```

## API Reference

### Common Operations (All Types)

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `new(value)` | Create new atomic | - |
| `load()` | Load current value | Acquire |
| `store(value)` | Store new value | Release |
| `swap(value)` | Swap value, return old | AcqRel |
| `compare_set(current, new)` | CAS operation, return Result | AcqRel/Acquire |
| `compare_set_weak(current, new)` | Weak CAS, return Result | AcqRel/Acquire |
| `compare_and_exchange(current, new)` | CAS operation, return actual value | AcqRel/Acquire |
| `compare_and_exchange_weak(current, new)` | Weak CAS, return actual value | AcqRel/Acquire |
| `fetch_update(f)` | Functional update, return old | AcqRel/Acquire |
| `inner()` | Access underlying std type | - |

### Integer Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `fetch_inc()` | Post-increment, return old | Relaxed |
| `fetch_dec()` | Post-decrement, return old | Relaxed |
| `fetch_add(delta)` | Post-add, return old | Relaxed |
| `fetch_sub(delta)` | Post-subtract, return old | Relaxed |
| `fetch_mul(factor)` | Post-multiply, return old | AcqRel (CAS loop) |
| `fetch_div(divisor)` | Post-divide, return old | AcqRel (CAS loop) |
| `fetch_and(value)` | Bitwise AND, return old | AcqRel |
| `fetch_or(value)` | Bitwise OR, return old | AcqRel |
| `fetch_xor(value)` | Bitwise XOR, return old | AcqRel |
| `fetch_not()` | Bitwise NOT, return old | AcqRel |
| `fetch_max(value)` | Atomic max, return old | AcqRel |
| `fetch_min(value)` | Atomic min, return old | AcqRel |
| `fetch_update(f)` | Functional update, return old | AcqRel/Acquire |
| `fetch_accumulate(x, f)` | Accumulate, return old | AcqRel/Acquire |

### Boolean Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `fetch_set()` | Set to true, return old | AcqRel |
| `fetch_clear()` | Set to false, return old | AcqRel |
| `fetch_not()` | Negate, return old | AcqRel |
| `fetch_and(value)` | Logical AND, return old | AcqRel |
| `fetch_or(value)` | Logical OR, return old | AcqRel |
| `fetch_xor(value)` | Logical XOR, return old | AcqRel |
| `set_if_false(new)` | CAS if false | AcqRel/Acquire |
| `set_if_true(new)` | CAS if true | AcqRel/Acquire |

### Floating-Point Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `fetch_add(delta)` | Atomic add, return old | AcqRel (CAS loop) |
| `fetch_sub(delta)` | Atomic subtract, return old | AcqRel (CAS loop) |
| `fetch_mul(factor)` | Atomic multiply, return old | AcqRel (CAS loop) |
| `fetch_div(divisor)` | Atomic divide, return old | AcqRel (CAS loop) |
| `fetch_update(f)` | Functional update, return old | AcqRel/Acquire |

## Memory Ordering Strategy

| Operation Type | Default Ordering | Reason |
|---------------|------------------|--------|
| **Pure Read** (`load()`) | `Acquire` | Ensure reading latest value |
| **Pure Write** (`store()`) | `Release` | Ensure write visibility |
| **Read-Modify-Write** (`swap()`, CAS) | `AcqRel` | Ensure both read and write correctness |
| **Counter Operations** (`fetch_inc()`, `fetch_add()`) | `Relaxed` | Pure counting, no need to sync other data |
| **Bitwise Operations** (`fetch_and()`, `fetch_or()`) | `AcqRel` | Usually used for flag synchronization |
| **Max/Min Operations** (`fetch_max()`, `fetch_min()`) | `AcqRel` | Often used with threshold checks |
| **Functional Updates** (`fetch_update()`) | `AcqRel` / `Acquire` | CAS loop standard semantics |

### Advanced Usage: Direct Access to Underlying Types

For scenarios requiring fine-grained memory ordering control (approximately 1% of use cases), use `inner()` to access the underlying standard library type:

```rust
use std::sync::atomic::Ordering;
use qubit_atomic::AtomicI32;

let atomic = AtomicI32::new(0);

// 99% of scenarios: use simple API
let value = atomic.load();

// 1% of scenarios: need fine-grained control
let value = atomic.inner().load(Ordering::Relaxed);
atomic.inner().store(42, Ordering::Release);
```

## Comparison with JDK

| Feature | JDK | Qubit Atomic | Notes |
|---------|-----|---------------|-------|
| **Basic Types** | 3 types | 13 types | Rust supports more integer types |
| **Memory Ordering** | Implicit (volatile) | Default + `inner()` optional | Rust more flexible |
| **Weak CAS** | `weakCompareAndSet` | `compare_and_set_weak` | Equivalent |
| **Reference Type** | `AtomicReference<V>` | `AtomicRef<T>` | Rust uses `Arc<T>` |
| **Nullability** | Allows `null` | Use `Option<Arc<T>>` | Rust no null pointers |
| **Bitwise Operations** | Partial support | Full support | Rust more powerful |
| **Max/Min Operations** | Java 9+ support | Supported | Equivalent |
| **API Count** | ~20 methods/type | ~25 methods/type | Rust provides more convenience methods |

## Performance Considerations

### Zero-Cost Abstraction

All wrapper types use `#[repr(transparent)]` and `#[inline]` to ensure zero overhead after compilation:

```rust
// Our wrapper
let atomic = AtomicI32::new(0);
let value = atomic.load();

// Compiles to the same code as
let atomic = std::sync::atomic::AtomicI32::new(0);
let value = atomic.load(Ordering::Acquire);
```

### When to Use `inner()`

**99% of scenarios**: Use default API, which already provides optimal performance.

**1% of scenarios**: Use `inner()` only when:
- Extreme performance optimization (need `Relaxed` ordering)
- Complex lock-free algorithms (need precise memory ordering control)
- Interoperating with code that directly uses standard library types

**Golden Rule**: Default API first, `inner()` as last resort.

## Testing & Code Coverage

This project maintains comprehensive test coverage with detailed validation of all functionality.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Run CI checks (format, clippy, test, coverage)
./ci-check.sh
```

### Coverage Metrics

See [COVERAGE.md](COVERAGE.md) for detailed coverage statistics.

## Dependencies

This crate has **zero dependencies** for the core functionality, relying only on Rust's standard library.

## License

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines
- Maintain comprehensive test coverage
- Document all public APIs with examples
- Ensure all tests pass before submitting PR

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

## Related Projects

More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-atomic](https://github.com/qubit-ltd/rs-atomic)
