### Create a new project:

```bash
blm new <my-new-java-project>
```

This will create a folder with a Java template. You have 3 builtin templates. An easy/minimalist one, an classic one and an advanced maven like template. If you want, you can install and use a custom template.
You can lookup how to install and use a custom template [here.](docs/custom-template.md)

> If no project name is given, `blm new` creates a project called `bloomery-project`.

If you have an existing directory e.g. with a .git directory you can Initialize a new bloomery project in your directory, without creating a new one:

```bash
blm init
```

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

This removes the folder given in 'class_dir' in your bloomery.toml and every directory listed in 'clean_dirs'.
When you don't have a bloomery.toml file, the default directory to clean will be bin.

You can still pass extra directories on the CLI:

```bash
blm clean testdir
```

This removes class_dir, all clean_dirs, and additionally testdir.

#### Manage clean_dirs via CLI

```bash
blm add clean-dir target # 1 additional folder
blm add clean-dir target tmp build # 2 additional folders
blm remove clean-dir target # remove folder from bloomery.toml list
blm list clean-dirs # list additional folders
```

### Configuration

The project configuration is stored in `bloomery.toml`:

```toml
[project]
name = "my-project"
version = "0.1.0"

[paths]
main_class = "Main"
class_dir = "out"
clean_dirs = []

[dependencies]

[dependencies.local]
jars = []
```

section and key meaning:

| Section              | Key               | Description                                  |
|----------------------|-------------------|----------------------------------------------|
| `[project]`          | `name`            | Project name                                 |
| `[project]`          | `version`         | Project version                              |
| `[paths]`            | `main_class`      | Main class used by `blm run`                 |
| `[paths]`            | `class_dir`       | Output directory for compiled `.class` files |
| `[paths]`            | `clean_dirs`      | Extra directories removed by `blm clean`    |
| `[dependencies]`     | `<name> = "<version>"` | Managed Maven dependencies              |
| `[dependencies.local]` | `jars`          | List of local JAR paths                      |

If you want to run an other main class, change your compilation folder, change your projects version, parse a dependencie, rename your project and other things, do it in the bloomery.toml or via CLI.

> If you want to add external dependencies, see the full Dependencies guide for [how to add dependencies](docs/dependencies.md).

### Set configuration values via CLI

You can also configure your project without opening the file:

```bash
blm set <key_to_change> <your_new_value>
```

You can change the name, version, main_class and class_dir:

```bash
blm set name my-project
blm set version 0.2.0
blm set main_class com.example.Main
blm set class_dir out
```

Lists such as local JARs or clean_dirs are managed with add/remove/list:

```bash
# clean_dirs
blm add clean-dir target tmp
blm remove clean-dir target
blm list clean-dirs

# managed dependencies
blm add deps gson
blm add deps gson 2.11.0
blm remove deps gson
blm remove deps gson -d
blm list deps

# local JARs
blm add local lib/custom.jar
blm remove local lib/custom.jar
blm list local
```

For details on managed and local dependencies, see [dependencies.md](docs/dependencies.md).
