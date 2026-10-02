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

your compiled file be stored in your current directory.

you can optionally parse a output directory, where your compiled file should be stored:

```bash
blm build-file <file_to_build> --output <output_directory>
```

If you want to run just 1 file , without compiling it and creating a compiled file or output directory, you can use:

```bash
blm run-file <file_to_run>
```

this will will execute `java <your_file>` and you dont need javac for this.

> for these operations you don't need a bloomery.toml.

### Clean your project

```bash
blm clean
```

this will remove the folder, given in class_dir in your bloomery.toml. When you don't have a bloomery.toml file, the default directory to clean will be `bin`.
You can clean multiple commands e.g. you have in your bloomery.toml `target` as your class dir, but you have another directory you want to remove e.g. `testdir`

You can clean both directorys in 1 CLI command with:

```bash
blm clean testdir
```

this will remove the `target` directory and additionally the `testdir` directory. You can remove as many directorys in 1 command as you want.

### Configuration

The project configuration is stored in `bloomery.toml`:

```toml
name = "my-project"
version = "0.1.0"
main_class = "Main"
class_dir = "out"
```

If you want to run an other main class, change your compilation folder, change your projects version or rename your project, do it in the `bloomery.toml`.
