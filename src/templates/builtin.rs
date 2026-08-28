use std::collections::HashMap;

// struct to describe the project template
pub struct FileTemplate {
    pub path: &'static str,
    pub content: &'static str,
}

pub struct ProjectTemplate {
    pub name: &'static str,
    pub dirs: &'static [&'static str],
    pub files: &'static [FileTemplate],
    pub main_class: &'static str,
}

// Two templates; an advanced and a default one
pub fn get_templates() -> HashMap<&'static str, ProjectTemplate> {
    let mut map = HashMap::new();

    /*
     * This is a minimal template, the default
     */
    map.insert(
        "default",
        ProjectTemplate {
            name: "default",
            dirs: &["src", "bin"],
            main_class: "Main",
            files: &[
                FileTemplate {
                    path: "src/Main.java",
                    content: "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}\n",
                },
                FileTemplate {
                    path: ".gitignore",
                    content: "/bin/\n*.class\n",
                },
            ],
        },
    );

    /*
     * This is an advanced, maven like template
     */
    map.insert(
        "advanced",
        ProjectTemplate {
            name: "advanced",
            dirs: &["src/main/java", "target/classes"],
            main_class: "Main",
            files: &[
                FileTemplate {
                    path: "src/main/java/Main.java",
                    content: "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}\n",
                },
                FileTemplate {
                    path: ".gitignore",
                    content: "/target/\n*.class\n",
                },
            ],
        },
    );

    map
}
