# Qubit Atomic

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-atomic.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-atomic)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-atomic/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-atomic?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-atomic.svg?color=blue)](https://crates.io/crates/qubit-atomic)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 提供类似 JDK 的用户友好原子操作封装。

## 概述

Qubit Atomic 是一个全面的原子操作库，提供易于使用的原子类型和合理的默认内存序，类似于 Java 的 `java.util.concurrent.atomic` 包。它隐藏了内存序的复杂性，同时保持零成本抽象，并允许高级用户访问底层类型以进行细粒度控制。

## 设计目标

- **易用性**：通过合理的默认值隐藏内存序复杂性
- **完整性**：提供类似 JDK atomic 类的高级操作
- **安全性**：保证内存安全和线程安全
- **性能**：零成本抽象，无额外开销
- **灵活性**：通过 `inner()` 方法暴露底层类型供高级用户使用
- **简洁性**：最小化 API 表面积，不提供 `_with_ordering` 变体

## 特性

### 🔢 **泛型原子基础类型**
- **整数特例**：`Atomic<i8>`、`Atomic<u8>`、`Atomic<i16>`、`Atomic<u16>`、`Atomic<i32>`、`Atomic<u32>`、`Atomic<i64>`、`Atomic<u64>`、`Atomic<i128>`、`Atomic<u128>`、`Atomic<isize>`、`Atomic<usize>`
- **布尔特例**：`Atomic<bool>`，支持设置、清除、取反、逻辑与/或/异或和条件 CAS
- **浮点特例**：`Atomic<f32>` 和 `Atomic<f64>`，通过 CAS 循环实现算术操作
- **丰富的操作**：自增、自减、加法、减法、乘法、除法、位运算、最大值/最小值
- **函数式更新**：`fetch_update`、`try_update`、`fetch_accumulate`

### 🔢 **`AtomicCount` 与 `AtomicSignedCount`**
- **`AtomicCount`**：活跃任务数、进行中请求数、资源使用数等非负计数
- **`AtomicSignedCount`**：差值、余额、积压量、偏移量等有符号计数
- **无环绕语义**：检查式更新，出错时 panic 或返回 `None`，不会发生 wrap
- **归零判断友好**：`inc`、`dec`、`add`、`sub` 返回操作后的新值

### 🔗 **原子引用类型**
- **AtomicRef<T>**：使用 `Arc<T>` 的线程安全原子引用
- **引用更新**：原子交换和 CAS 操作
- **Guard 读取**：`load_guard()` 适用于短生命周期读取，快路径不克隆 `Arc`
- **函数式更新**：用 `fetch_update` 原子地转换引用，或用 `try_update` 条件更新

### 🤝 **共享所有权便利封装**
- **`ArcAtomic<T>`**：`Arc<Atomic<T>>` 的便利 newtype
- **`ArcAtomicRef<T>`**：`Arc<AtomicRef<T>>` 的便利 newtype
- **`ArcAtomicCount` / `ArcAtomicSignedCount`**：计数类型的共享所有权封装
- **共享容器 clone**：克隆 `ArcAtomic*` 值时共享同一个原子容器

### 🎯 **聚焦的公开 API**
- **Atomic<T>**：基础原子值的泛型入口
- **AtomicRef<T>**：原子的 `Arc<T>` 引用封装
- **`AtomicCount` / `AtomicSignedCount`**：带检查语义、面向状态表达（无静默环绕）
- **`ArcAtomic*` 封装**：不必在每个使用点手写 `Arc<...>` 的共享所有权 API

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-atomic = "0.10.2"
```

## 快速开始

### 指定值类型 `T`

`Atomic<T>` 对基础值类型 `T` 泛型。多数情况下，编译器会根据传给 [`Atomic::new`](https://docs.rs/qubit-atomic/latest/qubit_atomic/struct.Atomic.html#method.new) 的实参推断 `T`，但像 `0` 这样的整型字面量可能在不同位宽之间产生歧义。

此时应显式指定 `T`：在构造函数上使用 [turbofish](https://doc.rust-lang.org/book/appendix-02-operators.html#the-turbofish) 写成 `Atomic::<T>::new(...)`，或为变量添加类型注解：

```rust
use qubit_atomic::Atomic;

let wide: Atomic<u64> = Atomic::new(0);
assert_eq!(wide.load(), 0u64);

let narrow = Atomic::<i16>::new(0);
assert_eq!(narrow.load(), 0i16);
```

### 示例：并发 `Atomic<i32>`

```rust
use qubit_atomic::Atomic;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Atomic::<i32>::new(0));
    let mut handles = vec![];

    // 启动 10 个线程，每个线程递增计数器 1000 次
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_inc();
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 验证结果
    assert_eq!(counter.load(), 10000);
    println!("最终计数：{}", counter.load());
}
```

### `AtomicCount` 与 `AtomicSignedCount`

纯指标统计使用 `Atomic<T>`。计数值本身属于并发状态（例如活跃任务数或
终止判断）时，使用 `AtomicCount`。

```rust
use qubit_atomic::{
    AtomicCount,
    AtomicSignedCount,
};

fn main() {
    let active_tasks = AtomicCount::zero();

    active_tasks.inc();
    assert!(!active_tasks.is_zero());

    if active_tasks.dec() == 0 {
        println!("所有活跃任务都已完成");
    }

    let backlog_delta = AtomicSignedCount::zero();
    assert_eq!(backlog_delta.add(5), 5);
    assert_eq!(backlog_delta.sub(8), -3);
    assert!(backlog_delta.is_negative());
}
```

### 共享所有权封装

当需要在线程或组件之间共享原子容器本身时，使用 `ArcAtomic*` 封装。
它们的 `clone()` 会克隆外层 `Arc`，因此所有 clone 都观察并修改同一个
原子容器。

```rust
use qubit_atomic::{
    ArcAtomic,
    ArcAtomicCount,
    ArcAtomicRef,
    ArcAtomicSignedCount,
};
use std::sync::Arc;
use std::thread;

fn main() {
    let requests = ArcAtomic::new(0usize);
    let worker_requests = requests.clone();

    let handle = thread::spawn(move || {
        worker_requests.fetch_inc();
    });
    handle.join().expect("worker should finish");

    assert_eq!(requests.load(), 1);
    assert_eq!(requests.strong_count(), 1);

    let active_tasks = ArcAtomicCount::zero();
    let shared_tasks = active_tasks.clone();
    assert_eq!(shared_tasks.inc(), 1);
    assert_eq!(active_tasks.get(), 1);

    let backlog = ArcAtomicSignedCount::zero();
    let shared_backlog = backlog.clone();
    assert_eq!(shared_backlog.sub(3), -3);
    assert_eq!(backlog.get(), -3);

    let config = ArcAtomicRef::from_value(String::from("v1"));
    let same_config = config.clone();
    same_config.store(Arc::new(String::from("v2")));
    assert_eq!(config.load().as_str(), "v2");
}
```

### CAS 循环

```rust
use qubit_atomic::Atomic;

fn increment_even_only(atomic: &Atomic<i32>) -> Result<i32, &'static str> {
    let mut current = atomic.load();
    loop {
        // 只对偶数值进行递增
        if current % 2 != 0 {
            return Err("值为奇数");
        }

        let new = current + 2;
        match atomic.compare_set(current, new) {
            Ok(_) => return Ok(new),
            Err(actual) => current = actual, // 重试
        }
    }
}

fn main() {
    let atomic = Atomic::<i32>::new(10);
    match increment_even_only(&atomic) {
        Ok(new_value) => println!("成功递增到：{}", new_value),
        Err(e) => println!("失败：{}", e),
    }
    assert_eq!(atomic.load(), 12);
}
```

### 函数式更新

```rust
use qubit_atomic::Atomic;

fn main() {
    let atomic = Atomic::<i32>::new(10);

    // 使用函数更新（返回旧值）
    let old_value = atomic.fetch_update(|x| {
        if x < 100 {
            x * 2
        } else {
            x
        }
    });

    assert_eq!(old_value, 10);
    assert_eq!(atomic.load(), 20);
    println!("更新后的值：{}", atomic.load());

    // 累积操作（返回旧值）
    let old_result = atomic.fetch_accumulate(5, |a, b| a + b);
    assert_eq!(old_result, 20);
    assert_eq!(atomic.load(), 25);
    println!("累积后的值：{}", atomic.load());
}
```

### 原子引用

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

    // 更新配置
    let new_config = Arc::new(Config {
        timeout: 2000,
        max_retries: 5,
    });

    let old_config = atomic_config.swap(new_config);
    println!("旧配置：{:?}", old_config);
    println!("新配置：{:?}", atomic_config.load());

    // 使用函数更新（返回旧值）
    let old = atomic_config.fetch_update(|current| {
        Arc::new(Config {
            timeout: current.timeout * 2,
            max_retries: current.max_retries + 1,
        })
    });

    println!("更新前的配置：{:?}", old);
    println!("更新后的配置：{:?}", atomic_config.load());

    // 短生命周期读取；快路径不会克隆 Arc
    let snapshot = atomic_config.load_guard();
    println!("快照配置：{:?}", snapshot);

    // 条件更新；拒绝时返回 None 且不修改当前值
    let accepted = atomic_config.try_update(|current| {
        (current.timeout < 10_000).then_some(Arc::new(Config {
            timeout: current.timeout + 1000,
            max_retries: current.max_retries,
        }))
    });
    assert!(accepted.is_some());
}
```

### 布尔标志

```rust
use qubit_atomic::Atomic;
use std::sync::Arc;

struct Service {
    running: Arc<Atomic<bool>>,
}

impl Service {
    fn new() -> Self {
        Self {
            running: Arc::new(Atomic::<bool>::new(false)),
        }
    }

    fn start(&self) {
        // 只有当前未运行时才启动
        if self.running.set_if_false(true).is_ok() {
            println!("服务启动成功");
        } else {
            println!("服务已经在运行");
        }
    }

    fn stop(&self) {
        // 只有当前运行时才停止
        if self.running.set_if_true(false).is_ok() {
            println!("服务停止成功");
        } else {
            println!("服务已经停止");
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

    service.start(); // 重复启动会失败

    service.stop();
    assert!(!service.is_running());

    service.stop(); // 重复停止会失败
}
```

### 浮点数原子操作

```rust
use qubit_atomic::Atomic;
use std::sync::Arc;
use std::thread;

fn main() {
    let sum = Arc::new(Atomic::<f32>::new(0.0));
    let mut handles = vec![];

    // 启动 10 个线程，每个线程累加 100 次
    for _ in 0..10 {
        let sum = sum.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                sum.fetch_add(0.01);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 注意：由于浮点数精度问题，结果可能不是精确的 10.0
    let result = sum.load();
    println!("累加结果：{:.6}", result);
    println!("误差：{:.6}", (result - 10.0).abs());
}
```

## API 参考

### 通用操作（所有类型）

| 方法 | 描述 | 内存序 |
|-----|------|--------|
| `new(value)` | 创建新的原子值 | - |
| `load()` | 加载当前值 | Acquire |
| `store(value)` | 存储新值 | Release |
| `swap(value)` | 交换值，返回旧值 | AcqRel |
| `compare_set(current, new)` | CAS 操作，返回 Result | AcqRel/Acquire |
| `compare_set_weak(current, new)` | 弱 CAS，返回 Result | AcqRel/Acquire |
| `compare_and_exchange(current, new)` | CAS 操作，返回实际值 | AcqRel/Acquire |
| `compare_and_exchange_weak(current, new)` | 弱 CAS，返回实际值 | AcqRel/Acquire |
| `fetch_update(f)` | 函数式更新，返回旧值 | AcqRel/Acquire |
| `try_update(f)` | 条件函数式更新，返回 `Option<旧值>` | AcqRel/Acquire |
| `inner()` | 访问底层后端类型 | - |

### 整数操作

| 方法 | 描述 | 内存序 |
|-----|------|--------|
| `fetch_inc()` | 后增，返回旧值 | Relaxed |
| `fetch_dec()` | 后减，返回旧值 | Relaxed |
| `fetch_add(delta)` | 后加，返回旧值 | Relaxed |
| `fetch_sub(delta)` | 后减，返回旧值 | Relaxed |
| `fetch_mul(factor)` | 后乘，返回旧值 | AcqRel（CAS 循环） |
| `fetch_div(divisor)` | 后除，返回旧值 | AcqRel（CAS 循环） |
| `fetch_and(value)` | 按位与，返回旧值 | AcqRel |
| `fetch_or(value)` | 按位或，返回旧值 | AcqRel |
| `fetch_xor(value)` | 按位异或，返回旧值 | AcqRel |
| `fetch_not()` | 按位取反，返回旧值 | AcqRel |
| `fetch_max(value)` | 原子取最大值，返回旧值 | AcqRel |
| `fetch_min(value)` | 原子取最小值，返回旧值 | AcqRel |
| `fetch_update(f)` | 函数式更新，返回旧值 | AcqRel/Acquire |
| `try_update(f)` | 条件函数式更新，返回 `Option<旧值>` | AcqRel/Acquire |
| `fetch_accumulate(x, f)` | 累积，返回旧值 | AcqRel/Acquire |

基础整数原子操作会在溢出和下溢时按 Rust 原子整数语义环绕。若业务语义要求拒绝溢出或下溢，请使用 `AtomicCount` 或 `AtomicSignedCount`。

### `AtomicCount` / `AtomicSignedCount` 的方法

| 方法 | `AtomicCount` | `AtomicSignedCount` | 内存序 | 描述 |
|-----|-----------------|-----------------------|--------|------|
| `new(value)` | `usize` | `isize` | - | 创建实例 |
| `zero()` | 支持 | 支持 | - | 创建零值实例 |
| `get()` | `usize` | `isize` | Acquire | 读取当前值 |
| `is_zero()` | 支持 | 支持 | Acquire | 判断值是否为零 |
| `is_positive()` | 支持 | 支持 | Acquire | 判断值是否为正 |
| `is_negative()` | 不支持 | 支持 | Acquire | 判断值是否为负 |
| `inc()` | 支持 | 支持 | AcqRel/Acquire | 加一，返回新值 |
| `dec()` | 下溢 panic | 允许负数 | AcqRel/Acquire | 减一，返回新值 |
| `add(delta)` | 溢出 panic | 溢出/下溢 panic | AcqRel/Acquire | 加 delta，返回新值 |
| `sub(delta)` | 下溢 panic | 溢出/下溢 panic | AcqRel/Acquire | 减 delta，返回新值 |
| `try_add(delta)` | 溢出返回 `None` | 溢出/下溢返回 `None` | AcqRel/Acquire | 检查式加法 |
| `try_dec()` | 零值返回 `None` | 不支持 | AcqRel/Acquire（仅 `AtomicCount`） | 检查式减一 |
| `try_sub(delta)` | 下溢返回 `None` | 溢出/下溢返回 `None` | AcqRel/Acquire | 检查式减法 |

### 共享所有权封装的方法

`ArcAtomic*` 封装会解引用到底层原子容器，因此可以直接在封装值上调用
`load`、`fetch_inc`、`store`、`inc`、`sub` 等操作。

| 方法 | 适用类型 | 描述 |
|-----|----------|------|
| `new(value)` | `ArcAtomic<T>`、`ArcAtomicCount`、`ArcAtomicSignedCount` | 从初始值创建共享封装 |
| `new(Arc<T>)` | `ArcAtomicRef<T>` | 从已有 `Arc<T>` 创建共享原子引用 |
| `from_value(value)` | `ArcAtomicRef<T>` | 从自有值创建共享原子引用 |
| `from_atomic(...)` | `ArcAtomic<T>` | 封装已有 `Atomic<T>` |
| `from_atomic_ref(...)` | `ArcAtomicRef<T>` | 封装已有 `AtomicRef<T>` |
| `from_count(...)` | `ArcAtomicCount`、`ArcAtomicSignedCount` | 封装已有计数容器 |
| `from_arc(arc)` | 所有 `ArcAtomic*` 封装 | 封装已有 `Arc<...>` 容器 |
| `as_arc()` | 所有 `ArcAtomic*` 封装 | 借用底层 `Arc<...>` |
| `into_arc()` | 所有 `ArcAtomic*` 封装 | 消耗封装并返回底层 `Arc<...>` |
| `strong_count()` | 所有 `ArcAtomic*` 封装 | 返回 `Arc` 强引用数量 |

### 布尔操作

| 方法 | 描述 | 内存序 |
|-----|------|--------|
| `fetch_set()` | 设置为 true，返回旧值 | AcqRel |
| `fetch_clear()` | 设置为 false，返回旧值 | AcqRel |
| `fetch_not()` | 取反，返回旧值 | AcqRel |
| `fetch_and(value)` | 逻辑与，返回旧值 | AcqRel |
| `fetch_or(value)` | 逻辑或，返回旧值 | AcqRel |
| `fetch_xor(value)` | 逻辑异或，返回旧值 | AcqRel |
| `set_if_false(new)` | 如果为 false 则 CAS | AcqRel/Acquire |
| `set_if_true(new)` | 如果为 true 则 CAS | AcqRel/Acquire |

### 浮点数操作

| 方法 | 描述 | 内存序 |
|-----|------|--------|
| `fetch_add(delta)` | 原子加法，返回旧值 | AcqRel（CAS 循环） |
| `fetch_sub(delta)` | 原子减法，返回旧值 | AcqRel（CAS 循环） |
| `fetch_mul(factor)` | 原子乘法，返回旧值 | AcqRel（CAS 循环） |
| `fetch_div(divisor)` | 原子除法，返回旧值 | AcqRel（CAS 循环） |
| `fetch_update(f)` | 函数式更新，返回旧值 | AcqRel/Acquire |
| `try_update(f)` | 条件函数式更新，返回 `Option<旧值>` | AcqRel/Acquire |

浮点 CAS 操作（`compare_set`、`compare_and_exchange` 及其 weak 版本）比较的是
原始 `to_bits()` 位模式，而不是 `PartialEq`。例如 `0.0` 和 `-0.0` 虽然相等，
但 CAS 不会匹配；NaN 的 payload 位也必须完全一致。需要明确成功结果时，
优先使用 `compare_set`，或自行比较 `to_bits()`。

## 内存序策略

| 操作类型 | 默认内存序 | 原因 |
|---------|-----------|------|
| **纯读操作** (`load()`) | `Acquire` | 保证读取最新值 |
| **纯写操作** (`store()`) | `Release` | 保证写入可见 |
| **读-改-写操作** (`swap()`、CAS) | `AcqRel` | 同时保证读和写的正确性 |
| **`Atomic<T>` 计数加减操作** (`fetch_inc()`、`fetch_dec()`、`fetch_add()`、`fetch_sub()`) | `Relaxed` | 纯指标场景，无需同步其他数据 |
| **基于 CAS 的算术和更新操作** (`fetch_mul()`、`fetch_div()`、`fetch_update()`、`try_update()`、`fetch_accumulate()`) | `AcqRel` / `Acquire` | CAS 循环标准语义 |
| **`AtomicCount` / `AtomicSignedCount`** (`inc()`、`dec()`) | CAS 循环 | 值作为并发状态信号 |
| **位运算操作** (`fetch_and()`、`fetch_or()`) | `AcqRel` | 通常用于标志位同步 |
| **最大/最小值操作** (`fetch_max()`、`fetch_min()`) | `AcqRel` | 常与阈值判断配合使用 |

### 高级用法：直接访问底层类型

对于需要精细控制内存序的场景（约 1% 的使用情况），通过 `inner()` 方法访问底层后端类型：

```rust
use std::sync::atomic::Ordering;
use qubit_atomic::Atomic;

let atomic = Atomic::<i32>::new(0);

// 99% 的场景：使用简单 API
let value = atomic.load();

// 1% 的场景：需要精细控制
let value = atomic.inner().load(Ordering::Relaxed);
atomic.inner().store(42, Ordering::Release);
```

## 与 JDK 对比

| 特性 | JDK | Qubit Atomic | 说明 |
|-----|-----|---------------|------|
| **基础类型** | 3 种类型 | `Atomic<T>` 特例 | Rust 覆盖更多整数、浮点、布尔与计数场景 |
| **内存序** | 隐式（volatile 语义） | 默认 + `inner()` 可选 | Rust 更灵活 |
| **弱 CAS** | `weakCompareAndSet` | `compare_set_weak` | 等价 |
| **引用类型** | `AtomicReference<V>` | `AtomicRef<T>` | Rust 使用 `Arc<T>` |
| **`AtomicCount` / `AtomicSignedCount`** | 手动组合 | `AtomicCount`、`AtomicSignedCount` | 状态跟踪用的非负 / 有符号计数 |
| **共享所有权** | 通常使用对象引用 | `ArcAtomic<T>`、`ArcAtomicRef<T>`、`ArcAtomicCount`、`ArcAtomicSignedCount` | 共享原子容器的便利封装 |
| **可空性** | 允许 `null` | 使用 `Option<Arc<T>>` | Rust 不允许空指针 |
| **位运算** | 部分支持 | 完整支持 | Rust 更强大 |
| **最大/最小值** | Java 9+ 支持 | 支持 | 等价 |
| **API 数量** | 约 20 个方法/类型 | 约 25 个方法/类型 | Rust 提供更多便利方法 |

## 性能考虑

### 零成本抽象

基础类型封装使用 `#[repr(transparent)]` 和 `#[inline]`，让泛型 API 编译到底层原子操作：

```rust
// 我们的封装
let atomic = Atomic::<i32>::new(0);
let value = atomic.load();

// 编译后与以下代码生成相同的机器码
let atomic = std::sync::atomic::AtomicI32::new(0);
let value = atomic.load(Ordering::Acquire);
```

### 何时使用 `inner()`

**99% 的场景**：使用默认 API，已经提供最优性能。

**1% 的场景**：只有在以下情况才使用 `inner()`：
- 极致性能优化（需要使用 `Relaxed` 内存序）
- 复杂的无锁算法（需要精确控制内存序）
- 与直接使用标准库或 `portable-atomic` 后端类型的代码互操作

**黄金法则**：默认 API 优先，`inner()` 是最后的手段。

## 测试与代码覆盖率

本项目保持全面的测试覆盖，对所有功能进行详细验证。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行基准测试
cargo bench --bench atomic_bench

# 列出基准测试场景
cargo bench --bench atomic_bench -- --list

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 运行 CI 检查（格式化、clippy、测试、覆盖率）
./ci-check.sh
```

### 覆盖率指标

详细的覆盖率统计请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 依赖项

运行时依赖保持很少：

- `arc-swap` 用于实现 `AtomicRef<T>`。
- `portable-atomic` 为 `Atomic<i128>` 和 `Atomic<u128>` 提供稳定后端。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南
- 保持全面的测试覆盖
- 为所有公共 API 编写文档和示例
- 提交 PR 前确保所有测试通过

## 作者

**胡海星** - *Qubit Co. Ltd.*

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-atomic](https://github.com/qubit-ltd/rs-atomic)
