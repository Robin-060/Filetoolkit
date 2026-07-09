use std::process::Command;

/// 检测系统是否存在tesseract命令
pub async fn check_tesseract_installed() -> bool {
    let check = Command::new("tesseract").arg("--version").output();
    check.is_ok() && check.unwrap().status.success()
}

/// 检测poppler(pdftoppm)
pub async fn check_poppler() -> bool {
    let check = Command::new("pdftoppm").arg("-v").output();
    check.is_ok() && check.unwrap().status.success()
}

/// 检测pdftk
pub async fn check_pdftk() -> bool {
    let check = Command::new("pdftk").arg("--version").output();
    check.is_ok() && check.unwrap().status.success()
}