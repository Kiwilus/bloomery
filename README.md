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

## Install and use a custom template

In **Bloomery**, you can turn any project directory into a reusable system-wide template.

### Step 1: Create your template directory structure

Set up a directory with the exact folder layout and files you want your template to have:

```text
my-custom-template/
|__ src/
|   |__ com/
|       |__ example/
|           |__ App.java
|__ README.md
|__ .gitignore
```

**Note on Packages & main_class:**

If your main method is located inside a Java package (e.g., `package com.example;` in `src/com/example/App.java`), you must specify the fully qualified class name including the package path using dot notation (`com.example.App`).

Include a `bloomery.toml` file inside your template directory to pre-define project defaults:

```toml
# bloomery.toml inside your template folder
name = "template-project"
version = "0.1.0"
main_class = "com.example.App"
```

### Step 2: Install the template system-wide

Navigate into your template directory and register it with Bloomery by assigning it a name:

```bash
cd my-custom-template
blm install --name my-custom-template
```

Bloomery serializes the folder structure and files, saving it to your global configuration directory:

```text
~/.config/bloomery/templates/my-custom-template.toml
```

### Step 3: Spawn a new project from your template

You can now initialize a new Java project anywhere on your system using the `-t` / `--template` flag:

```bash
blm init my-new-project --template my-custom-template
```

and your project structure will be like the directory you turn into a template

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
