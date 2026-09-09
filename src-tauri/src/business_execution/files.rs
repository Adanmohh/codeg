//! Managed bytes outside scratch. Unix descriptor/no-follow port of Codeg's
//! upload_jail.rs (NOTICE); stronger source identity/link/size checks added here.
//! The original host is trusted, not an OS sandbox. Other platforms fail closed
//! until an equivalent anchored/reparse-aware implementation is verified.
use super::{common::*, types::OperationReason as R, validation::MAX_OUTPUT_BYTES};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub(crate) struct Files {
    data_dir: PathBuf,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Stamp {
    dev: u64,
    ino: u64,
    len: u64,
    links: u64,
    mode: u32,
    modified: (i64, i64),
    changed: (i64, i64),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Observation {
    pub stamp: Stamp,
    pub sha256: String,
    pub byte_size: i64,
    pub media_type: String,
}
pub(super) struct Candidate {
    pub relative: String,
    pub observation: Observation,
}
pub(super) struct Retained {
    pub object_id: String,
    pub sha256: String,
    pub byte_size: i64,
}
pub(super) struct Content {
    pub bytes: Vec<u8>,
}

#[cfg(unix)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SyncPoint {
    StagedBytes,
    SealedBytes,
    ObjectDirectory,
}

pub(super) fn media_type(name: &str) -> Option<&'static str> {
    match Path::new(name)
        .extension()?
        .to_str()?
        .to_ascii_lowercase()
        .as_str()
    {
        "txt" | "md" | "csv" => Some("text/plain"),
        "json" => Some("application/json"),
        "pdf" => Some("application/pdf"),
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "docx" => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        "pptx" => Some("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        "xlsx" => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        _ => None,
    }
}
impl Files {
    pub(crate) fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }
    pub(crate) fn supported() -> bool {
        cfg!(unix)
    }
}

#[cfg(unix)]
mod unix {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::{
        ffi::CString,
        fs::{File, Metadata},
        io::{Read, Write},
        os::unix::{
            ffi::OsStrExt,
            fs::MetadataExt,
            io::{AsRawFd, FromRawFd},
        },
    };

    fn unavailable(_: std::io::Error) -> Error {
        R::ContentUnavailable.into()
    }
    fn component(value: &str) -> Result<CString> {
        if value.is_empty()
            || matches!(value, "." | "..")
            || value.contains(['/', '\\'])
            || value.chars().any(char::is_control)
        {
            return Err(R::Invalid.into());
        }
        CString::new(value).map_err(|_| R::Invalid.into())
    }
    struct Directory(File);
    impl Directory {
        // data_dir is supplied by the trusted AppState, never by a wire DTO.
        // From this anchored directory every descendant is opened with openat.
        fn root(path: &Path) -> Result<Self> {
            let path = CString::new(path.as_os_str().as_bytes()).map_err(|_| R::Invalid)?;
            let fd = unsafe {
                libc::open(
                    path.as_ptr(),
                    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                )
            };
            if fd < 0 {
                return Err(R::ContentUnavailable.into());
            }
            Ok(Self(unsafe { File::from_raw_fd(fd) }))
        }
        fn child(&self, name: &str, create: bool) -> Result<Self> {
            let name = component(name)?;
            if create {
                // Open after mkdir, including EEXIST, to reject a symlink/special
                // file or a directory replaced during creation.
                unsafe {
                    libc::mkdirat(self.0.as_raw_fd(), name.as_ptr(), 0o700);
                }
            }
            let fd = unsafe {
                libc::openat(
                    self.0.as_raw_fd(),
                    name.as_ptr(),
                    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                )
            };
            if fd < 0 {
                return Err(R::ContentUnavailable.into());
            }
            Ok(Self(unsafe { File::from_raw_fd(fd) }))
        }
        fn file(&self, name: &str, create: bool) -> Result<File> {
            let name = component(name)?;
            let flags = if create {
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL
            } else {
                libc::O_RDONLY | libc::O_NONBLOCK
            };
            let fd = unsafe {
                libc::openat(
                    self.0.as_raw_fd(),
                    name.as_ptr(),
                    flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                    0o600,
                )
            };
            if fd < 0 {
                return Err(R::ContentUnavailable.into());
            }
            let file = unsafe { File::from_raw_fd(fd) };
            stamp(&file.metadata().map_err(unavailable)?)?;
            Ok(file)
        }
    }
    fn stamp(metadata: &Metadata) -> Result<Stamp> {
        if !metadata.is_file() || metadata.nlink() != 1 || metadata.len() > MAX_OUTPUT_BYTES {
            return Err(R::ContentUnavailable.into());
        }
        Ok(Stamp {
            dev: metadata.dev(),
            ino: metadata.ino(),
            len: metadata.len(),
            links: metadata.nlink(),
            mode: metadata.mode(),
            modified: (metadata.mtime(), metadata.mtime_nsec()),
            changed: (metadata.ctime(), metadata.ctime_nsec()),
        })
    }
    fn safe_relative(relative: &str) -> Result<Vec<&str>> {
        if relative.len() > 1024 {
            return Err(R::Invalid.into());
        }
        let parts = relative.split('/').collect::<Vec<_>>();
        if parts.len() > 4
            || parts.iter().any(|p| {
                p.starts_with('.')
                    || matches!(
                        *p,
                        "config"
                            | "profiles"
                            | "credentials"
                            | "node_modules"
                            | "target"
                            | "auth.json"
                            | "config.json"
                            | "settings.json"
                            | "credentials.json"
                    )
            })
        {
            return Err(R::Invalid.into());
        }
        for part in &parts {
            component(part)?;
        }
        Ok(parts)
    }
    fn source(root: &Directory, relative: &str) -> Result<File> {
        let parts = safe_relative(relative)?;
        let mut parent = root.0.try_clone().map(Directory).map_err(unavailable)?;
        for part in &parts[..parts.len() - 1] {
            parent = parent.child(part, false)?;
        }
        parent.file(parts[parts.len() - 1], false)
    }
    fn read_stable(
        mut file: File,
        mut write: impl FnMut(&[u8]) -> Result<()>,
    ) -> Result<(Stamp, String)> {
        let before = stamp(&file.metadata().map_err(unavailable)?)?;
        let mut hasher = Sha256::new();
        let mut size = 0u64;
        let mut buffer = [0u8; 64 * 1024];
        loop {
            let n = file.read(&mut buffer).map_err(unavailable)?;
            if n == 0 {
                break;
            }
            size = size.checked_add(n as u64).ok_or(R::ContentChanged)?;
            if size > MAX_OUTPUT_BYTES || size > before.len {
                return Err(R::ContentChanged.into());
            }
            hasher.update(&buffer[..n]);
            write(&buffer[..n])?;
        }
        let after = stamp(&file.metadata().map_err(unavailable)?)?;
        if before != after || size != before.len {
            return Err(R::ContentChanged.into());
        }
        Ok((before, format!("{:x}", hasher.finalize())))
    }
    fn retained(
        objects: &Directory,
        existing: File,
        object_id: &str,
        expected_hash: &str,
        expected_size: i64,
        sync: &mut impl FnMut(&File, SyncPoint) -> Result<()>,
    ) -> Result<Retained> {
        if existing.metadata().map_err(unavailable)?.mode() & 0o777 != 0o400 {
            return Err(R::ContentUnavailable.into());
        }
        let (stored, hash) = read_stable(existing.try_clone().map_err(unavailable)?, |_| Ok(()))?;
        if hash != expected_hash || stored.len != expected_size as u64 {
            return Err(R::ContentChanged.into());
        }
        // Sealed mode does not prove a preceding sync succeeded. Both metadata
        // and directory durability must succeed again on every recovery.
        sync(&existing, SyncPoint::SealedBytes)?;
        sync(&objects.0, SyncPoint::ObjectDirectory)?;
        Ok(Retained {
            object_id: object_id.into(),
            sha256: hash,
            byte_size: stored.len as i64,
        })
    }

    impl Files {
        fn base(&self) -> Result<Directory> {
            Directory::root(&self.data_dir)?.child("business-execution", true)
        }
        fn workspace_directory(&self, admission_id: &str, create: bool) -> Result<Directory> {
            super::super::validation::uuid(admission_id)?;
            self.base()?
                .child("workspaces", true)?
                .child(admission_id, create)
        }
        pub(in crate::business_execution) fn workspace(
            &self,
            admission_id: &str,
        ) -> Result<PathBuf> {
            self.workspace_directory(admission_id, true)?;
            Ok(self
                .data_dir
                .join("business-execution")
                .join("workspaces")
                .join(admission_id))
        }
        pub(in crate::business_execution) fn scan(
            &self,
            admission_id: &str,
        ) -> Result<Vec<Candidate>> {
            let root = self.workspace_directory(admission_id, false)?;
            let path = self
                .data_dir
                .join("business-execution")
                .join("workspaces")
                .join(admission_id);
            let mut todo = vec![String::new()];
            let mut result = vec![];
            let mut inspected = 0;
            while let Some(relative) = todo.pop() {
                // Names from path enumeration are untrusted hints only. All
                // directories and bytes below are reopened from the original FD.
                let mut anchored = root.0.try_clone().map(Directory).map_err(unavailable)?;
                if !relative.is_empty() {
                    for part in safe_relative(&relative)? {
                        anchored = anchored.child(part, false)?;
                    }
                }
                let entries = std::fs::read_dir(path.join(&relative)).map_err(unavailable)?;
                for entry in entries {
                    inspected += 1;
                    if inspected > 512 {
                        return Err(R::RateLimited.into());
                    }
                    let entry = entry.map_err(unavailable)?;
                    let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
                        continue;
                    };
                    if name.starts_with('.') || component(&name).is_err() {
                        continue;
                    }
                    let next = if relative.is_empty() {
                        name.clone()
                    } else {
                        format!("{relative}/{name}")
                    };
                    if safe_relative(&next).is_err() {
                        continue;
                    }
                    if anchored.child(&name, false).is_ok() {
                        if next.split('/').count() < 4 {
                            todo.push(next);
                        }
                        continue;
                    }
                    let Some(kind) = media_type(&name) else {
                        continue;
                    };
                    let Ok(file) = source(&root, &next) else {
                        continue;
                    };
                    let (stamp, sha256) = read_stable(file, |_| Ok(()))?;
                    result.push(Candidate {
                        relative: next,
                        observation: Observation {
                            byte_size: stamp.len as i64,
                            stamp,
                            sha256,
                            media_type: kind.into(),
                        },
                    });
                }
            }
            result.sort_by(|a, b| a.relative.cmp(&b.relative));
            Ok(result)
        }
        pub(in crate::business_execution) fn stage(
            &self,
            admission_id: &str,
            relative: &str,
            expected: &Observation,
            object_id: &str,
        ) -> Result<Retained> {
            self.stage_with_sync(admission_id, relative, expected, object_id, |file, _| {
                file.sync_all().map_err(unavailable)
            })
        }

        // Fault injection is absent from production; callers cannot replace
        // either durability operation with a successful no-op.
        #[cfg(test)]
        pub(in crate::business_execution) fn stage_with_test_sync(
            &self,
            admission_id: &str,
            relative: &str,
            expected: &Observation,
            object_id: &str,
            sync: impl FnMut(&File, SyncPoint) -> Result<()>,
        ) -> Result<Retained> {
            self.stage_with_sync(admission_id, relative, expected, object_id, sync)
        }

        fn stage_with_sync(
            &self,
            admission_id: &str,
            relative: &str,
            expected: &Observation,
            object_id: &str,
            mut sync: impl FnMut(&File, SyncPoint) -> Result<()>,
        ) -> Result<Retained> {
            super::super::validation::uuid(object_id)?;
            let objects = self.base()?.child("objects", true)?;
            // Same-operation recovery can only reuse its completed immutable
            // object. It never truncates/replaces a partial or referenced object.
            if let Ok(existing) = objects.file(object_id, false) {
                return retained(
                    &objects,
                    existing,
                    object_id,
                    &expected.sha256,
                    expected.byte_size,
                    &mut sync,
                );
            }
            let root = self.workspace_directory(admission_id, false)?;
            let source = source(&root, relative)?;
            if stamp(&source.metadata().map_err(unavailable)?)? != expected.stamp {
                return Err(R::ContentChanged.into());
            }
            let mut staged = objects.file(object_id, true)?;
            let (observed, hash) =
                read_stable(source, |bytes| staged.write_all(bytes).map_err(unavailable))?;
            if observed != expected.stamp || hash != expected.sha256 {
                return Err(R::ContentChanged.into());
            }
            sync(&staged, SyncPoint::StagedBytes)?;
            if unsafe { libc::fchmod(staged.as_raw_fd(), 0o400) } != 0 {
                return Err(R::ContentUnavailable.into());
            }
            sync(&staged, SyncPoint::SealedBytes)?;
            sync(&objects.0, SyncPoint::ObjectDirectory)?;
            Ok(Retained {
                object_id: object_id.into(),
                sha256: hash,
                byte_size: observed.len as i64,
            })
        }
        /// Reconcile one durable import claim. A replay cannot fall through to
        /// another copy from the mutable original workspace.
        pub(in crate::business_execution) fn recover(
            &self,
            object_id: &str,
            expected_hash: &str,
            expected_size: i64,
        ) -> Result<Retained> {
            super::super::validation::uuid(object_id)?;
            let objects = self.base()?.child("objects", false)?;
            let existing = objects.file(object_id, false)?;
            retained(
                &objects,
                existing,
                object_id,
                expected_hash,
                expected_size,
                &mut |file: &File, _: SyncPoint| file.sync_all().map_err(unavailable),
            )
        }
        pub(in crate::business_execution) fn content(
            &self,
            object_id: &str,
            expected_hash: &str,
            expected_size: i64,
        ) -> Result<Content> {
            super::super::validation::uuid(object_id)?;
            if expected_size < 0 || expected_size as u64 > MAX_OUTPUT_BYTES {
                return Err(R::ContentUnavailable.into());
            }
            let file = self
                .base()?
                .child("objects", false)?
                .file(object_id, false)?;
            if file.metadata().map_err(unavailable)?.mode() & 0o777 != 0o400 {
                return Err(R::ContentUnavailable.into());
            }
            let mut bytes = Vec::with_capacity(expected_size as usize);
            let (metadata, hash) = read_stable(file, |chunk| {
                bytes.extend_from_slice(chunk);
                Ok(())
            })?;
            if hash != expected_hash || metadata.len != expected_size as u64 {
                return Err(R::ContentUnavailable.into());
            }
            Ok(Content { bytes })
        }
    }
}

#[cfg(not(unix))]
impl Files {
    pub(super) fn workspace(&self, _: &str) -> Result<PathBuf> {
        Err(R::Unavailable.into())
    }
    pub(super) fn scan(&self, _: &str) -> Result<Vec<Candidate>> {
        Err(R::Unavailable.into())
    }
    pub(super) fn stage(&self, _: &str, _: &str, _: &Observation, _: &str) -> Result<Retained> {
        Err(R::Unavailable.into())
    }
    pub(super) fn recover(&self, _: &str, _: &str, _: i64) -> Result<Retained> {
        Err(R::Unavailable.into())
    }
    pub(super) fn content(&self, _: &str, _: &str, _: i64) -> Result<Content> {
        Err(R::Unavailable.into())
    }
}
