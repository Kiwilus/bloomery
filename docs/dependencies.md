Bloomery manages external libraries (JARs) through a simple but effective mechanism: you place your .jar files in a folder of your choice (typically lib/) and list them in your bloomery.toml 
under the [dependencies] section. Bloomery then automatically adds them to the classpath—both when compiling (javac) and when running (java).

```toml
[dependencies]
jars = ["lib/gson.jar", "lib/lombok.jar"]
```

If you don't need any external libraries, just don't fill the `jars` list:

```toml 
[dependencies]
jars = []
```

please don't remove the [dependencies] section, this can cause problems at this moment.
