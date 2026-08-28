# bloomery

Bloomery is a easy and minimalist build system for Java projects written in Rust.
It is designed to make creating, building and running simple Java projects easy and just working.

I started Bloomery because I am learning Java at school and wanted something similar to Maven, but much simpler.

I don't need millions of features, complicated configuration or a huge build system just to compile and run a small Java project. I also don't want to use a heavy IDE like IntelliJ just for basic Java development.

Bloomery is my attempt at making a small tool that does what I need and nothing more.

## Usage

Create a new project:

```bash
blm init <my-new-java-project>
```

Build the project:

```bash
blm build
```

Run the project:

```bash
blm run
```

If no project name is given, `blm init` creates a project called `bloomery-project`.

## Project Structure

A new project looks like this:

```text
my-project/
├── bloomery.toml
├── src/
│   └── main/
│       └── java/
│           └── Main.java
└── target/
    └── classes/
```

At this point, the project structure is still hard-coded and there is only one template. It should be reworked into selectable and custom templates.

## Configuration

The project configuration is stored in `bloomery.toml`:

```toml
name = "my-project"
version = "0.1.0"
main_class = "Main"
```

## Requirements

- Rust
- JDK
- `javac` and `java` in your PATH

Bloomery is still a work in progress.
