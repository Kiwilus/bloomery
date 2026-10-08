### Create a new project:

```bash
blm init <my-new-java-project>
```

This will create a folder with a Java template. You have 3 builtin templates. An easy/minimalist one, an classic one and an advanced maven like template. If you want, you can install and use a custom template.
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

If you want to build just 1 file directly, you can use:

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

### build and run .jar files

If you want to build a jar file, you can use:

```bash
blm build-jar
```

> This will build your code into java-bytecode, extract your optional external dependencies and package it all in one .jar file

Your packaged .jar file will be in your current directory.

You can optionally parse a output directory, where your packaged .jar file will be stored:

```bash
blm build-jar --output <output_directory>
```

You can run an existing .jar file with:

```bash
blm run-jar <your_jar_file>
```

> If no jar file is parsed, bloomery will take the name of your directory, but it will not build a jar file like the run command

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
[project]
name = "my-project"
version = "0.1.0"

[paths]
main_class = "Main"
class_dir = "out"

[dependencies]
jars = []
```

If you want to run an other main class, change your compilation folder, change your projects version, parse a dependencie or rename your project, do it in the bloomery.toml.

> If you want to add external dependencies, see the full Dependencies guide for [how to add external JARs](docs/dependencies.md).

### Set configuration values via CLI

You can also configure your project without opening the file:

```bash
blm set <key_to_change> <your_new_value>
```

You can change the name, version, main_class and class_dir
