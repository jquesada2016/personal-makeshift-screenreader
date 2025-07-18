mod fut;

use std::ops::Deref;

use crate::{
    settings::{INVERT_CROSSHAIR_EVENT, subscribe_to_settings_changes},
    tauri,
};
use fut::request_animation_frame;
use leptos::{html, prelude::*};
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

use crate::settings::Settings;

#[instrument]
#[wasm_bindgen]
pub fn start_crosshair() {
    info!("starting crosshair page");

    mount_to_body(Crosshair);
}

#[component]
fn Crosshair() -> impl IntoView {
    let (settings, set_settings) = signal(None);
    let left = NodeRef::<html::Div>::new();
    let right = NodeRef::<html::Div>::new();
    let top = NodeRef::<html::Div>::new();
    let bottom = NodeRef::<html::Div>::new();

    subscribe_to_settings_changes(set_settings);

    Effect::new(move || {
        let left = left.get();
        let right = right.get();
        let top = top.get();
        let bottom = bottom.get();

        let Some(left) = left else { return };
        let Some(right) = right else { return };
        let Some(top) = top else { return };
        let Some(bottom) = bottom else { return };

        listen_for_invert_crosshair_event(
            left.deref().clone(),
            right.deref().clone(),
            top.deref().clone(),
            bottom.deref().clone(),
        );

        leptos::task::spawn_local(run_crosshair_loop(settings, left, right, top, bottom));
    });

    view! {
      <main class="h-screen">
        <div node_ref=left class="fixed bg-gradient-to-r from-black to-white" />
        <div node_ref=right class="fixed bg-gradient-to-l from-black to-white" />
        <div node_ref=top class="fixed bg-gradient-to-b from-black to-white" />
        <div node_ref=bottom class="fixed bg-gradient-to-t from-black to-white" />
      </main>
    }
}

async fn run_crosshair_loop<S>(
    settings: S,
    left: HtmlDivElement,
    right: HtmlDivElement,
    top: HtmlDivElement,
    bottom: HtmlDivElement,
) where
    S: GetUntracked<Value = Option<Settings>>,
{
    let window = tauri::window::get_current_window();

    loop {
        let Some(settings) = settings.get_untracked() else {
            request_animation_frame().await;

            continue;
        };

        let (pointer_pos, scale_factor, inner_pos) = futures::join!(
            tauri::window::get_cursor_position(),
            window.scale_factor(),
            window.inner_position()
        );

        let x = pointer_pos.x();
        let y = pointer_pos.y();

        let x_offset = inner_pos.x();
        let y_offset = inner_pos.y();

        let x_offset = x_offset / scale_factor;
        let y_offset = y_offset / scale_factor;

        let x = x / scale_factor - x_offset;
        let y = y / scale_factor - y_offset;

        draw_left(&left, &settings, x, y);
        draw_right(&right, &settings, x, y);
        draw_top(&top, &settings, x, y);
        draw_bottom(&bottom, &settings, x, y);

        request_animation_frame().await;
    }
}

macro_rules! set_styles {
    (
        $el:ident,
        {
            $($prop:literal : $value:literal),* $(,)?
        } $(,)?
    ) => {
        let styles = $el.style();

        $(
            styles.set_property($prop, &format!($value)).unwrap();
        )*
    };
}

fn draw_left(left: &web_sys::HtmlElement, settings: &Settings, x: f32, y: f32) {
    let Settings {
        line_thickness,
        pointer_gap,
        shortcuts: _,
    } = settings;

    set_styles!(
        left,
        {
            "height": "{line_thickness}rem",
            "width": "calc({x}px - {pointer_gap}rem)",
            "left": "0px",
            "top": "calc({y}px - {line_thickness}rem / 2)",
        },
    );
}

fn draw_right(right: &web_sys::HtmlElement, settings: &Settings, x: f32, y: f32) {
    let Settings {
        line_thickness,
        pointer_gap,
        shortcuts: _,
    } = settings;

    set_styles!(
        right,
        {
            "height": "{line_thickness}rem",
            "width": "calc(100vw - {x}px - {pointer_gap}rem)",
            "right": "0px",
            "top": "calc({y}px - {line_thickness}rem / 2)",
        },
    );
}

fn draw_top(top: &web_sys::HtmlElement, settings: &Settings, x: f32, y: f32) {
    let Settings {
        line_thickness,
        pointer_gap,
        shortcuts: _,
    } = settings;

    set_styles!(
        top,
        {
            "height": "calc({y}px - {pointer_gap}rem)",
            "width": "{line_thickness}rem",
            "top": "0px",
            "left": "calc({x}px - {line_thickness}rem / 2)",
        },
    );
}

fn draw_bottom(bottom: &web_sys::HtmlElement, settings: &Settings, x: f32, y: f32) {
    let Settings {
        line_thickness,
        pointer_gap,
        shortcuts: _,
    } = settings;

    set_styles!(
        bottom,
        {
            "height": "calc(100vh - {y}px - {pointer_gap}rem)",
            "width": "{line_thickness}rem",
            "bottom": "0px",
            "left": "calc({x}px - {line_thickness}rem / 2)",
        },
    );
}

fn listen_for_invert_crosshair_event(
    left: web_sys::HtmlElement,
    right: web_sys::HtmlElement,
    top: web_sys::HtmlElement,
    bottom: web_sys::HtmlElement,
) {
    let f = Closure::new(move || {
        info!("received invert crosshair event");

        invert_crosshair(&left, &right, &top, &bottom);
    });

    tauri::event::listen(INVERT_CROSSHAIR_EVENT, &f);

    f.forget();
}

fn invert_crosshair(
    left: &web_sys::HtmlElement,
    right: &web_sys::HtmlElement,
    top: &web_sys::HtmlElement,
    bottom: &web_sys::HtmlElement,
) {
    for line in [left, right, top, bottom] {
        for class in ["from-white", "from-black", "to-white", "to-black"] {
            line.class_list().toggle(class).unwrap();
        }
    }
}
