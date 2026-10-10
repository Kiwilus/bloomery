use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;

const LIB_DIR: &str = "lib";

// Maven search API types
#[derive(Debug, Deserialize)]
struct MavenSearchResponse {
    response: MavenSearchBody,
}

#[derive(Debug, Deserialize)]
struct MavenSearchBody {
    docs: Vec<MavenDoc>,
}

#[derive(Debug, Deserialize)]
struct MavenDoc {
    // groupID
    g: String,
    // artifactID
    a: String,
    // version
    #[serde(default)]
    v: String,
    #[serde(default, rename = "latestVersion")]
    latest_version: String,
}

// Resolve the latest version of an artifact on Maven Central
pub fn resolve_latest_version(artifact: &str) -> Result<String> {
    let url = format!("https://search.maven.org/solrsearch/select?q=a:{artifact}&rows=5&wt=json");

    let body: MavenSearchResponse = ureq::get(&url)
        .call()
        .context("Maven Search API request failed")?
        .into_json()
        .context("Failed to parse Maven Search response")?;

    body.response
        .docs
        .into_iter()
        .find(|doc| doc.a == artifact && !doc.latest_version.is_empty())
        .map(|doc| doc.latest_version)
        .with_context(|| {
            error!("Could not resolve latest version for '{artifact}' on Maven Central")
        })
}

// Resolve the Maven groupId for an artifact
// uses the public Maven Central search API:
// https://search.maven.org/solrsearch/select
fn resolve_group_id(artifact: &str, version: &str) -> Result<String> {
    // Prefer an exact artifact and version match.
    let url = format!(
        "https://search.maven.org/solrsearch/select?q=a:{artifact}+AND+v:{version}&rows=5&wt=json"
    );

    if let Some(group) = query_group_id(&url, artifact)? {
        return Ok(group);
    }

    // Fallback: search by artifactId only
    let url = format!("https://search.maven.org/solrsearch/select?q=a:{artifact}&rows=5&wt=json");

    query_group_id(&url, artifact)?
        .with_context(|| error!("Artifact '{artifact}' not found on Maven Central"))
}

fn query_group_id(url: &str, artifact: &str) -> Result<Option<String>> {
    let body: MavenSearchResponse = ureq::get(url)
        .call()
        .context("Maven Search API request failed")?
        .into_json()
        .context("Failed to parse Maven Search response")?;

    Ok(body
        .response
        .docs
        .into_iter()
        .find(|doc| doc.a == artifact)
        .map(|doc| doc.g))
}

fn maven_jar_url(group: &str, artifact: &str, version: &str) -> String {
    let group_path = group.replace('.', "/");
    format!(
        "https://repo1.maven.org/maven2/{group_path}/{artifact}/{version}/{artifact}-{version}.jar"
    )
}

// download process
fn download_jar(url: &str, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    let response = ureq::get(url)
        .call()
        .with_context(|| error!("Download failed: {url}"))?;

    if response.status() != 200 {
        anyhow::bail!("HTTP {} while downloading {url}", response.status());
    }

    let mut reader = response.into_reader();
    let mut file =
        fs::File::create(dest).with_context(|| error!("Could not create {}", dest.display()))?;

    std::io::copy(&mut reader, &mut file)
        .with_context(|| error!("Failed to write {}", dest.display()))?;

    Ok(())
}

// Ensure a single managed dependency exists under 'lib/'.
// Downloads from Maven Central when the JAR is missing.
fn ensure_jar(name: &str, version: &str) -> Result<PathBuf> {
    let filename = format!("{name}-{version}.jar");
    let dest = PathBuf::from(LIB_DIR).join(&filename);

    if dest.exists() {
        return Ok(dest);
    }

    info!("Downloading {name}:{version} ...");

    let group = resolve_group_id(name, version)?;
    let url = maven_jar_url(&group, name, version);

    download_jar(&url, &dest).with_context(|| {
        format!("Failed to fetch {name}:{version} (groupId={group}, url={url})")
    })?;

    success!("Downloaded {}", dest.display());
    Ok(dest)
}

// Resolve all dependency JAR paths for the classpath.
pub fn resolve_classpath_jars(config: &Config) -> Result<Vec<String>> {
    let mut paths = Vec::new();

    // manual JARs
    for jar in &config.dependencies.local.jars {
        if !Path::new(jar).exists() {
            error!("Dependency JAR not found: {jar}");
        }
        paths.push(jar.clone());
    }

    // managed JARs
    if !config.dependencies.managed.is_empty() {
        fs::create_dir_all(LIB_DIR)?;
    }

    for (name, version) in &config.dependencies.managed {
        let path = ensure_jar(name, version)?;
        paths.push(path.to_string_lossy().into_owned());
    }

    Ok(paths)
}
