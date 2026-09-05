//! 超级小爱安装与运行所需的 `userenv.dll` 代理部署。

use crate::install;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

/// 代理 DLL 文件名。
pub const PROXY_DLL_NAME: &str = "userenv.dll";

/// 内嵌的超级小爱专用代理 DLL。
const EMBEDDED_USERENV: &[u8] = include_bytes!("dlls/userenv.dll");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchOutcome {
    Patched,
    AlreadyPatched,
}

/// 将代理部署到指定目录；已有的其他同名 DLL 会保留为 `.orig.bak`。
pub fn apply(target_dir: &Path) -> Result<PatchOutcome> {
    if current_state(target_dir) {
        ensure_absent_origin_marker(&target_dir.join(PROXY_DLL_NAME))?;
        return Ok(PatchOutcome::AlreadyPatched);
    }
    deploy_proxy(target_dir)?;
    Ok(PatchOutcome::Patched)
}

/// 将内嵌代理 DLL 释放到指定目录，必要时备份已有文件。
pub fn deploy_proxy(target_dir: &Path) -> Result<PathBuf> {
    if !target_dir.is_dir() {
        bail!("目标目录不存在：{}", target_dir.display());
    }
    let target = target_dir.join(PROXY_DLL_NAME);
    if target.exists() {
        let current =
            fs::read(&target).with_context(|| format!("无法读取现有文件 {}", target.display()))?;
        if current != EMBEDDED_USERENV {
            install::ensure_backup(&target)?;
        } else {
            ensure_absent_origin_marker(&target)?;
        }
    } else {
        ensure_absent_origin_marker(&target)?;
    }
    install::write_file_atomic(&target, EMBEDDED_USERENV)?;
    Ok(target)
}

/// 在一次操作期间临时部署代理，并在操作结束后恢复目录原状。
pub fn with_temporary_proxy<T>(target_dir: &Path, action: impl FnOnce() -> Result<T>) -> Result<T> {
    let target = target_dir.join(PROXY_DLL_NAME);
    let previous = match fs::read(&target) {
        Ok(bytes) => Some(bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error).context(format!("无法读取 {}", target.display())),
    };
    let backup = install::backup_path(&target);
    let backup_existed = backup.exists();

    if let Err(error) = deploy_proxy(target_dir) {
        let cleanup =
            restore_temporary_proxy(target_dir, previous.as_deref(), &backup, backup_existed);
        return match cleanup {
            Ok(()) => Err(error),
            Err(cleanup_error) => {
                Err(error.context(format!("恢复临时代理也失败：{cleanup_error:#}")))
            }
        };
    }

    let action_result = action();
    let cleanup_result =
        restore_temporary_proxy(target_dir, previous.as_deref(), &backup, backup_existed);
    match (action_result, cleanup_result) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), Ok(())) => Err(error),
        (Ok(_), Err(error)) => Err(error.context("无法恢复临时 userenv.dll")),
        (Err(error), Err(cleanup_error)) => {
            Err(error.context(format!("恢复临时代理也失败：{cleanup_error:#}")))
        }
    }
}

/// 还原原有 DLL；若目录原本没有该文件，则删除本工具部署的代理。
pub fn revert(target_dir: &Path) -> Result<()> {
    let target = target_dir.join(PROXY_DLL_NAME);
    let backup = install::backup_path(&target);
    if !backup.is_file() {
        bail!("未找到备份文件 {}", backup.display());
    }
    let original =
        fs::read(&backup).with_context(|| format!("无法读取备份文件 {}", backup.display()))?;
    if original.is_empty() {
        if !current_state(target_dir) {
            bail!("{} 不是本工具部署的代理，拒绝删除", target.display());
        }
        fs::remove_file(&target)
            .with_context(|| format!("无法删除超级小爱代理 {}", target.display()))?;
    } else {
        install::restore_backup(&target)?;
    }
    fs::remove_file(&backup).with_context(|| format!("无法删除备份文件 {}", backup.display()))?;
    Ok(())
}

/// 当前目录是否已部署本工具内嵌的 `userenv.dll`。
pub fn current_state(target_dir: &Path) -> bool {
    fs::read(target_dir.join(PROXY_DLL_NAME))
        .map(|bytes| bytes == EMBEDDED_USERENV)
        .unwrap_or(false)
}

fn restore_temporary_proxy(
    dir: &Path,
    previous: Option<&[u8]>,
    backup: &Path,
    backup_existed: bool,
) -> Result<()> {
    let target = dir.join(PROXY_DLL_NAME);
    if let Some(bytes) = previous {
        install::write_file_atomic(&target, bytes)
            .with_context(|| format!("无法恢复 {}", target.display()))?;
    } else if target.exists() {
        if !current_state(dir) {
            bail!("{} 已被其他程序改写，拒绝清理", target.display());
        }
        fs::remove_file(&target).with_context(|| format!("无法移除 {}", target.display()))?;
    }
    if !backup_existed && backup.exists() {
        fs::remove_file(backup).with_context(|| format!("无法移除 {}", backup.display()))?;
    }
    Ok(())
}

fn ensure_absent_origin_marker(target: &Path) -> Result<()> {
    let backup = install::backup_path(target);
    if !backup.exists() {
        fs::write(&backup, [])
            .with_context(|| format!("无法创建原始文件缺失标记 {}", backup.display()))?;
    }
    Ok(())
}
