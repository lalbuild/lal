use common::*;

#[test]
pub fn test_remove_dependencies_no_save() {
    let state = setup();

    // "helloworld" has 1 dependency
    let component_dir = clone_component_dir("helloworld", &state);

    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(
        manifest.dependencies.get_key_value("heylib"),
        Some((&"heylib".to_string(), &1))
    );

    let save = false;
    let savedev = false;
    let r = lal::remove(
        &component_dir,
        &manifest,
        vec!["heylib".to_string()],
        save,
        savedev,
    );
    assert!(r.is_ok(), "removed heylib from INPUT");
    assert!(
        !component_dir.join("INPUT/heylib").exists(),
        "INPUT/heylib removed"
    );

    // Dependency remains in manifest
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(
        manifest.dependencies.get_key_value("heylib"),
        Some((&"heylib".to_string(), &1))
    );
}

#[test]
pub fn test_remove_dependencies_with_save() {
    let state = setup();

    // "helloworld" has 1 dependency
    let component_dir = clone_component_dir("helloworld", &state);

    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(
        manifest.dependencies.get_key_value("heylib"),
        Some((&"heylib".to_string(), &1))
    );

    let save = true;
    let savedev = false;
    let r = lal::remove(
        &component_dir,
        &manifest,
        vec!["heylib".to_string()],
        save,
        savedev,
    );
    assert!(r.is_ok(), "removed heylib from INPUT");
    assert!(
        !component_dir.join("INPUT/heylib").exists(),
        "INPUT/heylib removed"
    );

    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert_eq!(manifest.dependencies.len(), 0);
    assert_eq!(manifest.dependencies.get_key_value("heylib"), None);
}
