use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::entry_type::{crud::Crud, definitions::EntryDefinition},
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize, Debug)]
pub struct ScaffoldEntryTypeData {
    pub app_name: String,
    pub dna_role_name: String,
    pub coordinator_zome_manifest: ZomeManifest,
    pub entry_type: EntryDefinition,
    pub crud: Crud,
    pub link_from_original_to_each_update: bool,
}
pub fn scaffold_entry_type_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &str,
    dna_role_name: &str,
    coordinator_zome: &ZomeManifest,
    entry_type: &EntryDefinition,
    crud: &Crud,
    link_from_original_to_each_update: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldEntryTypeData {
        app_name: app_name.to_owned(),
        dna_role_name: dna_role_name.to_owned(),
        coordinator_zome_manifest: coordinator_zome.clone(),
        entry_type: entry_type.clone(),
        crud: crud.clone(),
        link_from_original_to_each_update: link_from_original_to_each_update.clone(),
    };
    let h = build_handlebars(&template_file_tree)?;

    let field_types_path = PathBuf::from("entry-type");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            web_app_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        &template_file_tree,
        &PathBuf::from("entry-type.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
