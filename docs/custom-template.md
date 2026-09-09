## In **Bloomery**, you can turn any project directory into a reusable system-wide template.

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
class_dir = "bin"
```

`name` is the name of your project
`version` is the version of your project
`main_class` is the class to run with the run command
`class_dir` is the directory where your .class files are stored like bin or target/classes

### Step 2: Install the template system-wide

Navigate into your template directory and register it with Bloomery by assigning it a name:

```bash
cd my-custom-template
blm install --name my-custom-template
```

Bloomery serializes the folder structure and files, saving it to your global configuration directory:

on linux:

```text
~/.config/bloomery/templates/my-custom-template.toml
```

on windows:

```text
C:\Users\<Username>\AppData\Roaming\bloomery\config\templates
```

on macos:

```text
/Users/<Username>/Library/Application Support/bloomery/templates
```

### Step 3: Spawn a new project from your template

You can now initialize a new Java project anywhere on your system using the `-t` / `--template` flag:

```bash
blm init my-new-project --template my-custom-template
```

and your project structure will be like the directory you turn into a template
