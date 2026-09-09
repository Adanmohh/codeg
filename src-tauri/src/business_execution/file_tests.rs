//! Real temporary-file checks adapted from Apache upload_jail tests in NOTICE.
use super::{common::*, files::Files, types::OperationReason, validation::MAX_OUTPUT_BYTES};
use std::{
    fs,
    os::unix::fs::{symlink, PermissionsExt},
};

#[test]
fn execution_assets_retained_bytes_survive_scratch_removal_and_same_object_recovery() {
    let temp = tempfile::TempDir::new().unwrap();
    let files = Files::new(temp.path().into());
    let admission = id();
    let workspace = files.workspace(&admission).unwrap();
    fs::write(workspace.join("brief.md"), b"# Exact reviewed brief\n").unwrap();
    let source = files.scan(&admission).unwrap().remove(0);
    let object_id = id();
    let retained = files
        .stage(
            &admission,
            &source.relative,
            &source.observation,
            &object_id,
        )
        .unwrap();
    fs::remove_dir_all(&workspace).unwrap();
    let replay = files
        .stage(
            &admission,
            &source.relative,
            &source.observation,
            &object_id,
        )
        .unwrap();
    assert_eq!(replay.object_id, object_id);
    assert_eq!(replay.sha256, retained.sha256);
    assert_eq!(
        files
            .content(&object_id, &retained.sha256, retained.byte_size)
            .unwrap()
            .bytes,
        b"# Exact reviewed brief\n"
    );
    assert_eq!(
        fs::read_dir(temp.path().join("business-execution/objects"))
            .unwrap()
            .count(),
        1
    );
    assert_eq!(
        fs::metadata(
            temp.path()
                .join("business-execution/objects")
                .join(&object_id)
        )
        .unwrap()
        .permissions()
        .mode()
            & 0o777,
        0o400
    );
}

#[test]
fn execution_assets_path_symlink_hardlink_and_profile_hints_never_become_candidates() {
    let temp = tempfile::TempDir::new().unwrap();
    let files = Files::new(temp.path().into());
    let admission = id();
    let workspace = files.workspace(&admission).unwrap();
    let outside = temp.path().join("outside.txt");
    fs::write(&outside, b"outside synthetic sentinel").unwrap();
    fs::write(workspace.join("brief.md"), b"allowed").unwrap();
    symlink(&outside, workspace.join("alias.txt")).unwrap();
    fs::hard_link(&outside, workspace.join("hard.txt")).unwrap();
    fs::create_dir(workspace.join("config")).unwrap();
    fs::write(
        workspace.join("config/client.json"),
        b"private synthetic config",
    )
    .unwrap();
    fs::write(workspace.join("auth.json"), b"private synthetic auth").unwrap();
    symlink(temp.path(), workspace.join("foreign")).unwrap();
    let candidates = files.scan(&admission).unwrap();
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].relative, "brief.md");
    for relative in [
        "../outside.txt",
        "/outside.txt",
        "foreign/outside.txt",
        "alias.txt",
        "hard.txt",
        "config/client.json",
        "auth.json",
    ] {
        assert!(
            files
                .stage(&admission, relative, &candidates[0].observation, &id())
                .is_err(),
            "{relative}"
        );
    }
    assert_eq!(fs::read(outside).unwrap(), b"outside synthetic sentinel");
}

#[test]
fn execution_assets_changed_observation_and_corrupt_retained_object_fail_closed() {
    let temp = tempfile::TempDir::new().unwrap();
    let files = Files::new(temp.path().into());
    let admission = id();
    let workspace = files.workspace(&admission).unwrap();
    let original = workspace.join("brief.md");
    fs::write(&original, b"original").unwrap();
    let observed = files.scan(&admission).unwrap().remove(0);
    fs::write(&original, b"modified").unwrap();
    assert!(matches!(
        files.stage(&admission, "brief.md", &observed.observation, &id()),
        Err(Error(OperationReason::ContentChanged))
    ));
    let current = files.scan(&admission).unwrap().remove(0);
    let object = id();
    let kept = files
        .stage(&admission, "brief.md", &current.observation, &object)
        .unwrap();
    let path = temp.path().join("business-execution/objects").join(&object);
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(&path, b"tampered").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o400)).unwrap();
    assert!(files
        .content(&object, &kept.sha256, kept.byte_size)
        .is_err());
    assert!(files
        .stage(&admission, "brief.md", &current.observation, &object)
        .is_err());
    assert_eq!(fs::read(path).unwrap(), b"tampered");
}

#[test]
fn execution_assets_oversize_and_replaced_storage_parent_do_not_escape() {
    let temp = tempfile::TempDir::new().unwrap();
    let files = Files::new(temp.path().into());
    let admission = id();
    let workspace = files.workspace(&admission).unwrap();
    fs::write(
        workspace.join("large.txt"),
        vec![0u8; MAX_OUTPUT_BYTES as usize + 1],
    )
    .unwrap();
    fs::write(workspace.join("ok.md"), b"ok").unwrap();
    let found = files.scan(&admission).unwrap();
    assert_eq!(found.len(), 1);
    let outside = temp.path().join("other-storage");
    fs::create_dir(&outside).unwrap();
    symlink(&outside, temp.path().join("business-execution/objects")).unwrap();
    assert!(files
        .stage(&admission, "ok.md", &found[0].observation, &id())
        .is_err());
    assert_eq!(fs::read_dir(outside).unwrap().count(), 0);
}
