use degit::{degit, validate_src};
use temp_dir::TempDir;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_content, load_directory_into_memory, FileTree},
};

use super::{choose_or_get_template, templates_path};

pub fn get_template(
    template_url: &str,
    template: &Option<String>,
) -> ScaffoldResult<(String, FileTree)> {
    validate_src(template_url.to_owned()).map_err(|s| ScaffoldError::DegitError(s))?;

    let tempdir = TempDir::new().unwrap();

    let tempdir_path = tempdir.path().to_path_buf();
    degit(template_url, tempdir_path.to_str().unwrap());

    let file_tree = load_directory_into_memory(&tempdir_path)?;

    let template_name = choose_or_get_template(&file_tree, template).map_err(|e| match e {
        ScaffoldError::NoTemplatesFound => ScaffoldError::NoTemplatesFoundInGitRepo,
        _ => e,
    })?;
    Ok((
        template_name.clone(),
        FileTree::Directory(dir_content(
            &file_tree,
            &templates_path().join(template_name),
        )?),
    ))
}
