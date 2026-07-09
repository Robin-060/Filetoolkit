use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::command;

// 引用同级common公共工具
use crate::common::dependency::check_tesseract_installed;

/// OCR返回结果结构体
#[derive(Debug, serde::Serialize)]
pub struct OcrResult {
    success: bool,
    msg: String,
    output_pdf: Option<String>,
}

/// PDF OCR核心接口
/// input：原始PDF文件路径
/// languages：识别语言数组 ["chi_sim", "eng"]
/// output：生成双层可检索PDF保存路径
#[command]
pub async fn ocr_pdf(
    input: String,
    languages: Vec<String>,
    output: String,
) -> OcrResult {
    // 第一步：检测OCR工具是否安装
    if !check_tesseract_installed().await {
        return OcrResult {
            success: false,
            msg: "未检测到Tesseract识别工具，请先安装依赖".to_string(),
            output_pdf: None,
        };
    }

    let input_path = Path::new(&input);
    let output_path = Path::new(&output);
    // 临时目录存放PDF分页图片
    let temp_dir = std::env::temp_dir().join("pdf_ocr_temp");
    let _ = std::fs::create_dir_all(&temp_dir);

    // 第二步：pdftoppm(Poppler)将PDF分页导出PNG图片
    let pdf_split = Command::new("pdftoppm")
        .arg("-png")
        .arg(input_path)
        .arg(temp_dir.join("page"))
        .output();

    if pdf_split.is_err() || !pdf_split.unwrap().status.success() {
        return OcrResult {
            success: false,
            msg: "PDF分页失败，请安装poppler工具".to_string(),
            output_pdf: None,
        };
    }

    // 读取全部分页图片
    let mut img_files = Vec::new();
    if let Ok(dir_list) = std::fs::read_dir(&temp_dir) {
        for entry in dir_list.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("png") {
                img_files.push(path);
            }
        }
    }

    if img_files.is_empty() {
        return OcrResult {
            success: false,
            msg: "PDF分页图片为空，原文件损坏".to_string(),
            output_pdf: None,
        };
    }

    // 拼接识别语言参数 中文+英文
    let lang_param = languages.join("+");
    let mut single_page_pdfs = VecDeque::new();

    // 第三步：逐张图片执行Tesseract OCR，生成单页带文字PDF
    for img in img_files {
        let pdf_temp_file = temp_dir.join(img.file_stem().unwrap().to_str().unwrap());
        let tesseract_run = Command::new("tesseract")
            .arg(&img)
            .arg(&pdf_temp_file)
            .arg("-l")
            .arg(&lang_param)
            .arg("pdf")
            .output();

        if tesseract_run.is_ok() && tesseract_run.unwrap().status.success() {
            single_page_pdfs.push_back(pdf_temp_file.with_extension("pdf"));
        }
    }

    if single_page_pdfs.is_empty() {
        return OcrResult {
            success: false,
            msg: "所有页面OCR识别失败".to_string(),
            output_pdf: None,
        };
    }

    // 第四步：pdftk合并所有单页PDF为完整文档
    let mut merge_cmd = Command::new("pdftk");
    for page_pdf in single_page_pdfs {
        merge_cmd.arg(page_pdf);
    }
    merge_cmd.arg("cat").arg("output").arg(output_path);
    let merge_result = merge_cmd.output();

    // 清空临时缓存文件
    let _ = std::fs::remove_dir_all(temp_dir);

    match merge_result {
        Ok(out) if out.status.success() => OcrResult {
            success: true,
            msg: "PDF OCR识别完成，文件已生成".to_string(),
            output_pdf: Some(output_path.to_string_lossy().to_string()),
        },
        _ => OcrResult {
            success: false,
            msg: "分页PDF合并失败，请安装pdftk".to_string(),
            output_pdf: None,
        },
    }
}