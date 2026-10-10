Bloomery manages external libraries (JARs) through a simple and flexible dependency system. Dependencies can either be downloaded and managed automatically through Maven coordinates, or provided as local `.jar` files.

### Managed dependencies

Managed dependencies are declared under the 'dependencies' section of bloomery.toml. Bloomery resolves them from Maven Central, downloads the JARs into lib/ when needed, and adds them to the classpath when compiling and running your project.

```toml
[dependencies]
gson = "2.11.0"
guava = "33.3.1-jre"
```

Each dependency is specified using its name and version.

#### CLI

```bash
# Add a managed dependency
blm deps add <name> <version>
# or without a specified version
# bloomery will search up the newest version and use it.
blm deps add <name>

# Remove a managed dependency
blm deps remove <name>

# List all managed dependencies
blm deps list
```

### Local JARs

If you already have JAR files locally, you can list them under `[dependencies.local]`. The `jars` list contains paths relative to your project directory.

```toml
[dependencies.local]
jars = [
    "lib/gson.jar",
    "lib/lombok.jar",
]
```

Bloomery automatically adds these JARs to the classpath both when compiling.

You can use managed dependencies and local JARs together:

```toml
[dependencies]
gson = "2.11.0"

[dependencies.local]
jars = [
    "lib/custom-library.jar",
]
```

#### CLI

```bash
# Add a local JAR path
blm deps add-local <path>

# Remove a local JAR path from the config
blm deps remove-local <path>

# List all local JAR paths
blm deps list-local
```

If you don't need any external libraries, you can remove both sections or simply leave them empty:

The `[dependencies]` section is intended for dependencies managed by Bloomery, while `[dependencies.local]` is specifically for JAR files that are already present in your project.
