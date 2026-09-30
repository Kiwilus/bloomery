### Create a new project:

```bash
blm init <my-new-java-project>
```

This will create a folder with a Java template. You have 2 builtin templates. A classic/easy one and an advanced maven like template. You can install and use a custom template.
You can lookup how to install and use a custom template [here.](docs/custom-template.md)

> If no project name is given, `blm init` creates a project called `bloomery-project`.

### Build the project:

```bash
blm build
```

> it will build all java files in your source directory


### Run the project:

```bash
blm run
```

> it will build all java files in your source directory and run the .class, set in your bloomery.toml as main_class

### Run or build just a single Java file

If you want to build just 1 file directly you can use:

```bash
blm build-file <file_to_build>
```

If you want to run just 1 file directly you can use:

```bash
blm run-file <file_to_run>
```

> for these operations you don't need a bloomery.toml. Bloomery will create a `bin` folder and store there your compiled files.

### Clean your project

```bash
blm clean
```

this will remove the folder, given in class_dir in your bloomery.toml

> when no bloomery.toml file is found, it will automaticly delete the bin folder.

### Configuration

The project configuration is stored in `bloomery.toml`:

```toml
name = "my-project"
version = "0.1.0"
main_class = "Main"
class_dir = "out"
```

If you want to run an other main class, change your compilation folder, change your projects version or rename your project, do it in the `bloomery.toml`.
