pub fn gitignore() -> String {
    r#"# editors
/.idea
/.vscode

# system files
.DS_Store

# build
/dist/
/target/
/.cargo/

# npm 
/**/node_modules/

# generated and compiled files
*.happ
*.webhapp
*.zip
*.dna

# temporary files
.hc*
.running
.hc
"#
    .to_string()
}
