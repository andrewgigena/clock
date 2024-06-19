use chrono::Local;
use leptos::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlAudioElement;
use leptos_use::{use_favicon, use_interval_fn};
use leptos_use::utils::Pausable;

#[component]
fn App() -> impl IntoView {
    let (icon, set_icon) = use_favicon();
    set_icon.set(Some("favicon.ico".to_string())); // change current icon

    let (date_time, set_date_time) =
        create_signal(vec![String::new(), String::new(), String::new()]);
    let (is_playing, set_is_playing) = create_signal(false);

    let Pausable {
        pause: _,
        resume: _,
        is_active: _,
    } = use_interval_fn(
        move || {
            // Get the current time in the specified format
            let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f %:z").to_string();

            // Split the date, time, and timezone
            let time_parts: Vec<String> = now.split_whitespace().map(String::from).collect();
            set_date_time.set(time_parts);
        },
        1,
    );

    view! {
        <div class="flex flex-col justify-center items-center h-screen">
            <p class="text-5xl">{move || date_time.get()[0].clone()}</p>
            <p class="text-3xl">{move || date_time.get()[1].clone()}</p>
            <p class="text-2xl">{move || date_time.get()[2].clone()}</p>

            <div class="mt-4">
                <audio id="audioPlayer" loop preload="true">
                    <source src="ticking.mp3" muted="muted"/>
                </audio>
                <button
                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded font-mono h-12 w-12"

                    on:click=move |_| {
                        let audio = document().get_element_by_id("audioPlayer").unwrap().dyn_into::<HtmlAudioElement>().unwrap();
                        if is_playing.get() {
                            audio.pause();
                            set_is_playing.set(false);
                        } else {
                            audio.play();
                            set_is_playing.set(true);
                        }
                    }
                >

                    {move || if is_playing.get() { "⏸︎" } else { "▶" }}
                </button>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
