#![cfg(windows)]

use mipcmanager_patch::infra::powershell::run_powershell;
use std::env;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn preserves_unicode_in_powershell_errors() {
    let _guard = ENV_LOCK.lock().unwrap();
    let error = run_powershell("throw '编码测试'").unwrap_err();
    let message = format!("{error:#}");

    assert!(
        message.contains("编码测试"),
        "unexpected error text: {message}"
    );
}

#[test]
fn starts_system_powershell_without_using_path() {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_path = env::var_os("PATH");
    // SAFETY: ENV_LOCK 串行化此测试进程中的环境变量访问。
    unsafe { env::set_var("PATH", "") };

    let result = run_powershell("'system-powershell'");

    match original_path {
        // SAFETY: 在断言前恢复本测试保存的原始进程环境。
        Some(path) => unsafe { env::set_var("PATH", path) },
        // SAFETY: PATH 原先不存在时恢复为不存在。
        None => unsafe { env::remove_var("PATH") },
    }

    assert_eq!(result.unwrap().trim(), "system-powershell");
}
