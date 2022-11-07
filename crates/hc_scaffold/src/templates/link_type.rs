use std::{ffi::OsString, path::PathBuf};

use serde::Serialize;

use crate::{error::ScaffoldResult, file_tree::FileTree};

use super::{build_handlebars, render_template_file_tree_and_merge_with_existing};

#[derive(Serialize)]
pub struct ScaffoldLinkTypeData {
    dna_role_id: String,
    coordinator_zome_name: String,
    from_entry_type: String,
    to_entry_type: Option<String>,
}
pub fn scaffold_link_type_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    dna_role_id: &String,
    coordinator_zome_name: &String,
    from_entry_type: &String,
    to_entry_type: &Option<String>,
) -> ScaffoldResult<FileTree> {
    let data = ScaffoldLinkTypeData {
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_name: coordinator_zome_name.clone(),
        from_entry_type: from_entry_type.clone(),
        to_entry_type: to_entry_type.clone(),
    };

    let h = build_handlebars(&template_file_tree)?;

    let field_types_path = PathBuf::from("link-type");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            web_app_template,
            &data,
        )?;
    }

    Ok(app_file_tree)
}