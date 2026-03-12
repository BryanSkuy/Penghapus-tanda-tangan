#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::processor;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
struct AppState {
    image_path: String,
    original_uri: String,
    result_uri: String,
    threshold: u8,
    color_tolerance: u8,
    contrast: f32,
    status: String,
    is_processing: bool,
    original_img: Option<Arc<image::DynamicImage>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            image_path: String::new(),
            original_uri: String::new(),
            result_uri: String::new(),
            threshold: 200,
            color_tolerance: 30,
            contrast: 0.0,
            status: "Unggah foto tanda tangan untuk memulai...".to_string(),
            is_processing: false,
            original_img: None,
        }
    }
}

pub fn App() -> Element {
    let mut state = use_signal(AppState::default);

    // --- Actions ---

    let open_file = move |_| {
        spawn(async move {
            let file = rfd::AsyncFileDialog::new()
                .set_title("Pilih Gambar Tanda Tangan")
                .add_filter("Gambar", &["png", "jpg", "jpeg", "bmp", "webp", "tif", "tiff"])
                .pick_file()
                .await;

            if let Some(f) = file {
                let path = f.path().to_string_lossy().to_string();
                
                state.with_mut(|s| s.status = "Memuat gambar...".to_string());
                
                match processor::load_image(&path) {
                    Ok((img, uri)) => {
                        state.with_mut(|s| {
                            s.image_path = path.clone();
                            s.original_uri = uri;
                            s.result_uri = String::new();
                            s.original_img = Some(Arc::new(img));
                            s.status = format!("✅ Berhasil memuat: {}", path.split(['/', '\\']).last().unwrap_or(""));
                        });
                    }
                    Err(e) => {
                        state.with_mut(|s| s.status = format!("❌ {}", e));
                    }
                }
            }
        });
    };

    let process = move |_| {
        let s = state.read();
        let img_opt = s.original_img.clone();
        let threshold = s.threshold;
        let tolerance = s.color_tolerance;
        let contrast = s.contrast;
        drop(s);

        let Some(img_arc) = img_opt else {
            state.with_mut(|s| s.status = "⚠️ Pilih gambar dulu!".to_string());
            return;
        };

        state.with_mut(|s| {
            s.is_processing = true;
            s.status = "⏳ Sedang memproses...".to_string();
        });

        spawn(async move {
            let res = tokio::task::spawn_blocking(move || {
                let processed = processor::remove_background(&img_arc, threshold, tolerance, contrast);
                processor::to_data_uri(&processed)
            })
            .await;

            match res {
                Ok(uri) => {
                    state.with_mut(|s| {
                        s.result_uri = uri;
                        s.is_processing = false;
                        s.status = "✅ Selesai! Klik Simpan PNG untuk mengunduh.".to_string();
                    });
                }
                Err(_) => {
                    state.with_mut(|s| {
                        s.is_processing = false;
                        s.status = "❌ Terjadi kesalahan saat memproses.".to_string();
                    });
                }
            }
        });
    };

    let save_file = move |_| {
        let s = state.read();
        let img_opt = s.original_img.clone();
        let threshold = s.threshold;
        let tolerance = s.color_tolerance;
        let contrast = s.contrast;
        let has_res = !s.result_uri.is_empty();
        drop(s);

        if !has_res || img_opt.is_none() {
            state.with_mut(|s| s.status = "⚠️ Proses gambar dulu sebelum simpan!".to_string());
            return;
        }

        let img_arc = img_opt.unwrap();

        spawn(async move {
            let file = rfd::AsyncFileDialog::new()
                .set_title("Simpan PNG Transparan")
                .add_filter("PNG", &["png"])
                .set_file_name("tanda_tangan_transparan.png")
                .save_file()
                .await;

            if let Some(f) = file {
                let path = f.path().to_string_lossy().to_string();
                let path_clone = path.clone();

                let save_res = tokio::task::spawn_blocking(move || {
                    let processed = processor::remove_background(&img_arc, threshold, tolerance, contrast);
                    processed.save(&path).map_err(|e| e.to_string())
                })
                .await;

                match save_res {
                    Ok(Ok(_)) => {
                        state.with_mut(|s| s.status = format!("✅ Berhasil disimpan ke: {}", path_clone));
                    }
                    _ => {
                        state.with_mut(|s| s.status = "❌ Gagal menyimpan file.".to_string());
                    }
                }
            }
        });
    };

    // --- UI Values ---
    let s = state.read();
    let threshold = s.threshold;
    let tolerance = s.color_tolerance;
    let contrast = s.contrast;
    let is_processing = s.is_processing;
    let original_uri = s.original_uri.clone();
    let result_uri = s.result_uri.clone();
    let status = s.status.clone();
    let has_image = !original_uri.is_empty();
    let has_result = !result_uri.is_empty();
    drop(s);

    rsx! {
        div { class: "root",
            aside { class: "sidebar",
                div { class: "brand",
                    div { class: "brand-icon", "✍" }
                    div { class: "brand-text",
                        h1 { class: "brand-title", "Sig Remover" }
                        p { class: "brand-sub", "Signature Background Eraser" }
                    }
                }
                div { class: "divider" }
                
                button { class: "btn btn-primary", onclick: open_file,
                    span { class: "btn-icon", "📂" }
                    span { "Buka Gambar" }
                }
                
                div { class: "divider" }

                div { class: "controls",
                    div { class: "control-group",
                        div { class: "control-header",
                            label { class: "control-label", "Threshold" }
                            span { class: "control-value badge", "{threshold}" }
                        }
                        input { class: "slider", r#type: "range", min: "50", max: "255", value: "{threshold}",
                            oninput: move |e| { if let Ok(v) = e.value().parse() { state.with_mut(|s| s.threshold = v); } }
                        }
                    }
                    div { class: "control-group",
                        div { class: "control-header",
                            label { class: "control-label", "Toleransi" }
                            span { class: "control-value badge", "{tolerance}" }
                        }
                        input { class: "slider", r#type: "range", min: "0", max: "150", value: "{tolerance}",
                            oninput: move |e| { if let Ok(v) = e.value().parse() { state.with_mut(|s| s.color_tolerance = v); } }
                        }
                    }
                    div { class: "control-group",
                        div { class: "control-header",
                            label { class: "control-label", "Kontras" }
                            span { class: "control-value badge", "{contrast as i32}" }
                        }
                        input { class: "slider", r#type: "range", min: "-100", max: "100", value: "{contrast}",
                            oninput: move |e| { if let Ok(v) = e.value().parse() { state.with_mut(|s| s.contrast = v); } }
                        }
                    }
                }

                div { class: "divider" }

                button { 
                    class: if is_processing { "btn btn-accent btn-disabled" } else { "btn btn-accent" },
                    onclick: process,
                    disabled: is_processing,
                    if is_processing { span { class: "spinner" } span { "Memproses..." } }
                    else { span { class: "btn-icon", "⚡" } span { "Hapus Background" } }
                }

                button { 
                    class: if has_result { "btn btn-save" } else { "btn btn-save btn-disabled" },
                    onclick: save_file,
                    disabled: !has_result,
                    span { class: "btn-icon", "💾" }
                    span { "Simpan PNG" }
                }
            }

            main { class: "content",
                div { class: "status-bar",
                    div { class: "status-dot" }
                    span { class: "status-text", "{status}" }
                }

                div { class: "preview-area",
                    div { class: "preview-card",
                        div { class: "preview-card-header", span { class: "preview-tag tag-original", "ORIGINAL" } h3 { "Input" } }
                        div { class: "preview-frame",
                            if has_image { img { class: "preview-img", src: "{original_uri}" } }
                            else { div { class: "preview-placeholder", div { class: "placeholder-icon", "🖼" } p { "Menunggu gambar..." } } }
                        }
                    }
                    div { class: "arrow-divider", "→" }
                    div { class: "preview-card",
                        div { class: "preview-card-header", span { class: "preview-tag tag-result", "TRANSPARENT" } h3 { "Output" } }
                        div { class: "preview-frame checker",
                            if has_result { img { class: "preview-img", src: "{result_uri}" } }
                            else { div { class: "preview-placeholder", div { class: "placeholder-icon", "✨" } p { "Hasil transparan" } } }
                        }
                    }
                }
            }
        }
    }
}
