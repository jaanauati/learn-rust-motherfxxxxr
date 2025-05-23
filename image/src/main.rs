
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use gpui::{
    actions, div, img, prelude::*, px, rgb, size, App, AppContext, AssetSource, Bounds,
    ImageSource, KeyBinding, Menu, MenuItem, Point, SharedString, SharedUri, TitlebarOptions,
    ViewContext, WindowBounds, WindowContext, WindowOptions,
};

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|e| e.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }
}

#[derive(IntoElement)]
struct ImageContainer {
    text: SharedString,
    src: ImageSource,
}

impl ImageContainer {
    pub fn new(text: impl Into<SharedString>, src: impl Into<ImageSource>) -> Self {
        Self {
            text: text.into(),
            src: src.into(),
        }
    }
}

impl RenderOnce for ImageContainer {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        div().child(
            div()
                .flex_row()
                .size_full()
                .gap_4()
                .child(self.text)
                .child(img(self.src).size(px(256.0))),
        )
    }
}

struct ImageShowcase {
    local_resource: Arc<std::path::Path>,
    remote_resource: SharedUri,
    asset_resource: SharedString,
}

impl Render for ImageShowcase {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

actions!(image, [Quit]);

fn main() {

    App::new()
        .with_assets(Assets {
            base: PathBuf::from("crates/gpui/examples"),
        })
        .run(|cx: &mut AppContext| {
            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            cx.set_menus(vec![Menu {
                name: "Image".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            }]);

            let window_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Image Example")),
                    appears_transparent: false,
                    ..Default::default()
                }),

                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    size: size(px(1100.), px(600.)),
                    origin: Point::new(px(200.), px(200.)),
                })),

                ..Default::default()
            };

            cx.open_window(window_options, |cx| {
                cx.new_view(|_cx| ImageShowcase {
                    // Relative path to your root project path
                    local_resource: PathBuf::from_str("crates/gpui/examples/image/app-icon.png")
                        .unwrap()
                        .into(),

                    remote_resource: "https://picsum.photos/512/512".into(),

                    asset_resource: "image/color.svg".into(),
                })
            })
            .unwrap();
        });
}
