mod app;
mod processor;

use dioxus::prelude::*;
use app::App;

fn main() {
    let cfg = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("✍  Signature BG Remover")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1100.0, 720.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 560.0))
                .with_resizable(true)
                .with_transparent(true),
        )
        .with_custom_head(
            format!("<style>{}</style>", include_str!("../assets/style.css"))
        );

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
