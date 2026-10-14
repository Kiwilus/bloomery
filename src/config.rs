use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/*
 * Part for the bloomery.toml configuration
 */

// struct for the bloomery.toml sections
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // toml sections
    pub project: Project,
    pub paths: Paths,
    #[serde(default)]
    pub dependencies: Dependencies,
    #[serde(default)]
    pub build: Build,
}

// 'Project' section from the sections struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
}

// 'Paths' section from the sections struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Paths {
    pub main_class: String,
    #[serde(default = "default_class_dir")]
    pub class_dir: String,
    #[serde(default)]
    pub clean_dirs: Vec<String>,
}

// 'Dependencies' section from the sections struct
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dependencies {
    // Local JARs
    // [dependencies.local]
    #[serde(default)]
    pub local: LocalDependencies,

    // Online Maven JARs
    // [dependencies]
    #[serde(flatten)]
    pub managed: HashMap<String, String>,
}

// list for local JARs
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalDependencies {
    #[serde(default)]
    pub jars: Vec<String>,
}

// 'build section from the sections struct'
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Build {
    // release version like 17 or 21
    #[serde(default)]
    pub release: Option<String>,
    // optional path to exact JDK to use
    #[serde(default)]
    pub jdk_path: Option<String>,
}

impl Build {
    // path to a tool like java/javac or jar
    pub fn tool(&self, name: &str) -> String {
        if let Some(jdk_path) = &self.jdk_path {
            let bin = Path::new(jdk_path).join("bin").join(name);
            // attach .exe on windows if not already present
            #[cfg(windows)]
            {
                let with_exe = bin.with_extension("exe");
                if with_exe.exists() {
                    return with_exe.to_string_lossy().into_owned();
                }
            }
            if bin.exists() {
                return bin.to_string_lossy().into_owned();
            }
        }
        // fallback: from path
        name.to_string()
    }

    pub fn javac(&self) -> String {
        self.tool("javac")
    }
    pub fn java(&self) -> String {
        self.tool("java")
    }
    pub fn jar(&self) -> String {
        self.tool("jar")
    }
}

// default class directory
fn default_class_dir() -> String {
    "target/classes".to_string()
}

pub fn load_config() -> Result<Config> {
    let content = fs::read_to_string("bloomery.toml")
        .map_err(|_e| error!("bloomery.toml not found. Are you in the project directory?"))?;
    let config: Config =
        toml::from_str(&content).map_err(|_e| error!("bloomery.toml could not be read"))?;
    Ok(config)
}

// Part for global configuration of the default template in bloomery/config.toml
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default = "default_template")]
    pub default_template: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            default_template: "default".to_string(),
        }
    }
}

fn default_template() -> String {
    "default".to_string()
}

// Return the path to the global config file
pub fn global_config_path() -> anyhow::Result<PathBuf> {
    let proj = ProjectDirs::from("", "", "bloomery")
        .ok_or_else(|| error!("Could not determine config directory"))?;
    Ok(proj.config_dir().join("config.toml"))
}

// load config.toml
pub fn load_global_config() -> anyhow::Result<GlobalConfig> {
    let path = global_config_path()?;
    if !path.exists() {
        return Ok(GlobalConfig::default());
    }
    let content = fs::read_to_string(&path)?;
    Ok(toml::from_str(&content)?)
}

/*
// write global config to the disk
pub fn save_global_config(cfg: &GlobalConfig) -> anyhow::Result<()> {
    let path = global_config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, toml::to_string_pretty(cfg)?)?;
    Ok(())
}
*/

/// write_config for both functions init and new
// write a fresh bloomery.toml to 'root'
pub fn write_config(
    root: &Path,
    name: &str,
    version: &str,
    main_class: &str,
    class_dir: &str,
    clean_dirs: Vec<String>,
    build: Build,
) -> Result<()> {
    let config = Config {
        project: Project {
            name: name.to_string(),
            version: version.to_string(),
        },
        paths: Paths {
            main_class: main_class.to_string(),
            class_dir: class_dir.to_string(),
            clean_dirs,
        },
        dependencies: Dependencies::default(),
        build,
    };

    let mut content = toml::to_string_pretty(&config)?;

    // Ensure [dependencies] exists before [dependencies.local]
    if let Some(pos) = content.find("[dependencies.local]") {
        content.insert_str(pos, "[dependencies]\n\n");
    } else {
        content.push_str("\n[dependencies]\n");
    }

    fs::write(root.join("bloomery.toml"), content)?;
    Ok(())
}

// save_config function here now
pub fn save_config(config: &Config) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write("bloomery.toml", content)?;
    Ok(())
}
