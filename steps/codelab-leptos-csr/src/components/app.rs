use crate::components::cell::Cell;
use crate::components::game_over_overlay::GameOverOverlay;
use crate::model::board_store::{GameState, GameStateStoreFields};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {
    let state = Store::new(GameState::new());
    let is_game_over = RwSignal::new(false);

    Effect::new(move |_| {
        state.maybe_update(|state| {
            if is_game_over.get() {
                false
            } else {
                console_log("run only when is_game_over change to false");
                state.reset();
                true
            }
        });
    });
    view! {
        <div class="square">
            <div class="grid" style="grid-template-columns: repeat(10, 1fr); grid-template-rows: repeat(10, 1fr); ">
                <For
                    each=move ||  state.rows()
                    key=|idx_case| idx_case.get().get_key()
                    let:idx_case
                >
                    <Cell
                        case=idx_case.get().case.clone()
                        on:click = move |_| {
                            console_log(&format!("click on cell {}", idx_case.get().idx));
                            if idx_case.get().case.is_mine() {
                                is_game_over.set(true);
                            }
                            idx_case.update(|c| {
                                c.case.reveal();
                            });
                        }
                    />
                </For>
            </div>
        </div>
        <GameOverOverlay state=is_game_over />
    }
}