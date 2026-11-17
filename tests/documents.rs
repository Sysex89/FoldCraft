use marvelous_cloth::{document::atomic_write, fitting::FittingProject, garment::GarmentTemplate};
use std::{fs, path::PathBuf};

fn project() -> FittingProject {
    let mut p = FittingProject::basic(
        marvelous_cloth::garment::presets::GarmentKind::Tshirt,
        1,
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/makehuman/mannequin.obj"),
    );
    p.template = GarmentTemplate::two_panel_demo();
    p
}

#[test]
fn portable_document_and_cached_state_survive_relocation() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source.obj");
    fs::write(&source, "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n").unwrap();
    let mut p = project();
    p.avatar_path = source.clone();
    let state = p.compile().unwrap().state;
    p.state = Some(state.clone());
    p.simulation = Some(marvelous_cloth::vbd::VbdSettings {
        iterations: 27,
        dt: 1. / 120.,
        ..Default::default()
    });
    let bundle = dir.path().join("original");
    fs::create_dir(&bundle).unwrap();
    let file = bundle.join("test.json");
    p.save(&file).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    assert!(PathBuf::from(json["avatar_path"].as_str().unwrap()).is_relative());
    let moved = dir.path().join("moved");
    fs::rename(&bundle, &moved).unwrap();
    fs::remove_file(source).unwrap();
    let loaded = FittingProject::load(&moved.join("test.json")).unwrap();
    assert_eq!(loaded.simulation, p.simulation);
    let reopened = loaded.compile().unwrap();
    assert_eq!(reopened.state.x, state.x);
    assert_eq!(reopened.state.v, state.v);
    assert_eq!(reopened.state.time, state.time);
}

#[test]
fn failed_save_preserves_previous_document() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.json");
    atomic_write(&file, b"previous valid bytes").unwrap();
    let mut p = project();
    p.avatar_path = dir.path().join("missing.obj");
    assert!(p.save(&file).is_err());
    assert_eq!(fs::read(&file).unwrap(), b"previous valid bytes");
    assert!(atomic_write(dir.path(), b"cannot replace directory").is_err());
    atomic_write(&file, b"replacement").unwrap();
    assert_eq!(fs::read(file).unwrap(), b"replacement");
}

/// A new document is as accessible as any other file the process creates, and
/// replacing a document keeps the mode it already had.
#[cfg(unix)]
#[test]
fn atomic_write_keeps_ordinary_and_existing_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let mode = |path: &std::path::Path| fs::metadata(path).unwrap().permissions().mode() & 0o777;
    let dir = tempfile::tempdir().unwrap();
    // The umask cannot be read without changing it, so a plain create in the
    // same directory is the reference for a new file.
    let plain = dir.path().join("plain.json");
    fs::write(&plain, b"plain").unwrap();
    let file = dir.path().join("document.json");
    atomic_write(&file, b"first").unwrap();
    assert_eq!(mode(&file), mode(&plain));
    for existing in [0o600, 0o640, 0o664] {
        fs::set_permissions(&file, fs::Permissions::from_mode(existing)).unwrap();
        atomic_write(&file, b"replacement").unwrap();
        assert_eq!(mode(&file), existing);
        assert_eq!(fs::read(&file).unwrap(), b"replacement");
    }
}

/// A template or legacy project save replaces the file instead of rewriting it
/// in place: a second link to the previous file still holds its complete bytes,
/// which a truncating write through the shared inode would have destroyed.
#[cfg(unix)]
#[test]
fn template_and_project_saves_replace_the_previous_file() {
    let dir = tempfile::tempdir().unwrap();
    let previous = b"previous valid bytes";
    let seed = |name: &str| {
        let file = dir.path().join(name);
        fs::write(&file, previous).unwrap();
        let link = dir.path().join(format!("{name}.previous"));
        fs::hard_link(&file, &link).unwrap();
        (file, link)
    };

    let (file, link) = seed("template.json");
    let template = GarmentTemplate::two_panel_demo();
    template.save(&file).unwrap();
    assert_eq!(fs::read(&link).unwrap(), previous);
    assert_eq!(
        fs::read(&file).unwrap(),
        serde_json::to_vec_pretty(&template).unwrap()
    );
    GarmentTemplate::load(&file).unwrap();

    let (file, link) = seed("project.json");
    let project = marvelous_cloth::project::Project::default();
    project.save(&file).unwrap();
    assert_eq!(fs::read(&link).unwrap(), previous);
    assert_eq!(
        fs::read(&file).unwrap(),
        serde_json::to_vec_pretty(&project).unwrap()
    );
    marvelous_cloth::project::Project::load(&file).unwrap();

    // No temporary file is left beside the two documents and their links.
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 4);
}
