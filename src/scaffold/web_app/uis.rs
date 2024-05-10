use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_exists, dir_to_file_tree, file_exists, FileTree},
};

static LIT_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/lit");
static SVELTE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/svelte");
static VUE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vue");
static VANILLA_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vanilla");
static HEADLESS_TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/headless");

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
    Svelte,
    Vue,
    Other(String),
}

impl UiFramework {
    pub fn template_filetree(&self) -> ScaffoldResult<FileTree> {
        let dir = match self {
            UiFramework::Lit => &LIT_TEMPLATES,
            UiFramework::Vanilla => &VANILLA_TEMPLATES,
            UiFramework::Svelte => &SVELTE_TEMPLATES,
            UiFramework::Vue => &VUE_TEMPLATES,
            UiFramework::Other(other) if other == "headless (no ui)" => &HEADLESS_TEMPLATE,
            UiFramework::Other(t) => {
                return Err(ScaffoldError::MalformedTemplate(format!(
                    "Inbuilt template {t} not found."
                )));
            }
        };
        dir_to_file_tree(dir)
    }

    pub fn choose() -> ScaffoldResult<UiFramework> {
        let frameworks = [
            UiFramework::Lit,
            UiFramework::Svelte,
            UiFramework::Vue,
            UiFramework::Vanilla,
            UiFramework::Other("headless (no ui)".to_owned()),
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }

    pub fn choose_non_vanilla() -> ScaffoldResult<UiFramework> {
        let frameworks = [UiFramework::Lit, UiFramework::Svelte, UiFramework::Vue];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }
}

impl TryFrom<&FileTree> for UiFramework {
    type Error = ScaffoldError;

    fn try_from(app_file_tree: &FileTree) -> Result<Self, Self::Error> {
        let ui_package_json_path = PathBuf::from("ui/package.json");
        if file_exists(app_file_tree, &ui_package_json_path) {
            let v: Vec<OsString> = ui_package_json_path
                .iter()
                .map(|s| s.to_os_string())
                .collect();
            let ui_package_json = app_file_tree
                .path(&mut v.iter())
                .ok_or(ScaffoldError::PathNotFound(ui_package_json_path.clone()))?
                .file_content()
                .ok_or(ScaffoldError::PathNotFound(ui_package_json_path.clone()))?
                .clone();
            if ui_package_json.contains("lit") {
                return Ok(UiFramework::Lit);
            } else if ui_package_json.contains("svelte") {
                return Ok(UiFramework::Svelte);
            } else if ui_package_json.contains("vue") {
                return Ok(UiFramework::Vue);
            } else if !dir_exists(app_file_tree, &PathBuf::from("ui/src")) {
                return Ok(UiFramework::Vanilla);
            }
        }
        UiFramework::choose()
    }
}

impl std::fmt::Display for UiFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UiFramework::Vanilla => "vanilla".yellow(),
            UiFramework::Lit => "lit".blue(),
            UiFramework::Svelte => "svelte".red(),
            UiFramework::Vue => "vue".green(),
            UiFramework::Other(value) if value == "headless (no ui)" => value.italic(),
            UiFramework::Other(value) => value.normal(),
        };
        write!(f, "{str}")
    }
}

impl FromStr for UiFramework {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<UiFramework> {
        match s {
            "vanilla" => Ok(UiFramework::Vanilla),
            "svelte" => Ok(UiFramework::Svelte),
            "vue" => Ok(UiFramework::Vue),
            "lit" => Ok(UiFramework::Lit),
            other => Ok(UiFramework::Other(other.to_owned())),
        }
    }
}
