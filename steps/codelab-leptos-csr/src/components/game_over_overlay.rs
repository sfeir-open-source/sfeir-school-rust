use leptos::prelude::*;

#[component]
pub fn GameOverOverlay(state: RwSignal<bool>) -> impl IntoView {
    view! {
        {move ||
             if *state.read() {
                 view! {
                     <div class="overlay">
                        <div class="overlay-container">
                            <div class="message">"Perdu"</div>
                            <button
                                on:click=move |_| {
                                    state.set(false);
                                }
                            >
                                "Rejouer"
                            </button>
                        </div>
                     </div>
                 }.into_any()
             } else {
                 view! {}.into_any()
             }
        }
    }
}
