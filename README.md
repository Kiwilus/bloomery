<div>

# Bloomery

**A lightweight, minimalist build system for Java written in Rust.**

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Work%20in%20Progress-yellow.svg)](#)

</div>

---

Bloomery is a easy and minimalist build system for Java projects written in Rust.
It is designed to make creating, building and running simple Java projects easy and just working.

I started Bloomery because I am learning Java at school and wanted something similar to Maven, but much simpler.

I don't need millions of features, complicated configuration or a huge build system just to compile and run a small Java project. I also don't want to use a heavy IDE like IntelliJ just for basic Java development.

Bloomery is my attempt at making a small tool that does what I need and nothing more.

---

## Installation

To install bloomery on your system:

clone the repo and change into it:

```bash
git clone https://github.com/Kiwilus/bloomery.git && cd bloomery
```

install bloomery system wide:

```bash
cargo install --path .
```

## Documentation

- [Usage](docs/usage.md)
- [Custom Templates](docs/custom-template.md)

## How I use bloomery with zed

If you are Interessted how I use bloomery in zed

See the [Zed configuration.](docs/zed-configuration.md).

## Requirements

- Rust
- JDK
- `javac` and `java` in your PATH

Bloomery is still a work in progress.
