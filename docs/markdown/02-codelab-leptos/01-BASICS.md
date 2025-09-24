<!-- .slide: class="sfeir-bg-blue-1" sfeir-level="2" sfeir-techno="rust" -->

# **Leptos**

## **Les bases**

##==##

<!-- .slide: class="with-code" -->

# Comment ça marche ?

##==##

<!-- .slide: class="with-code" -->

# Creation d'un projet

```bash
cargo new leptos-minesweeper

# global install
cargo install	trunk  `# gère le build et sert le binaire en local` \
	leptosfmt `# formatteur adapter à Leptos`
rustup target add wasm32-unknown-unknown # ajoute la possibilité de génerer du wasm

cd leptos-minesweeper
cargo add leptos --features=csr # project's deps install 

# active le formatage de la macro view 
echo "[rustfmt]
overrideCommand = [\"leptosfmt\", \"--stdin\", \"--rustfmt\"]" > rust-analyzer.toml
```
##==##

<!-- .slide: class="with-code" -->

# La première page
index.html
```html 
<!DOCTYPE html>
<html lang="fr">
<head>
  <title>Démineur</title>
  <link data-trunk rel="css" href="app.css" />
  <meta charset="utf-8">
</head>
<body></body>
</html>
```
##==##

<!-- .slide: class="with-code" -->

# Un simple composant
src/components/app.rs
```rust 
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
  view! {
        <div></div>
    }
}
```
##==##

<!-- .slide: class="with-code" -->

# qu'on monte

```rust src/main.rs
mod components;
mod model;

use leptos::prelude::*;
use crate::components::app::App;

fn main() {
  mount_to_body(App);
}
```
##==##

<!-- .slide: class="with-code" -->

# et qu'on sert 

```bash
trunk serve
```
##==##

<!-- .slide: class="with-code" -->

# Commençons par la fin
src/components/app.rs
```rust 
pub fn App() -> impl IntoView {
  let is_game_over = true;
  if is_game_over {
      view! {
        <div class="overlay">
        </div>
      }.into_any()
    } else {
      view! {}.into_any()
  }
}
```
##==##

<!-- .slide: class="with-code" -->

# Commençons par la fin
src/components/app.rs
```rust 
pub fn App() -> impl IntoView {
  let is_game_over = true;
  if is_game_over {
      view! {
        <div class="overlay">
          <div class="overlay-container">
            <div class="message">"Perdu"</div>
            <button >
                "Rejouer"
            </button>
          </div>
        </div>
      }.into_any()
    } else {
      view! {}.into_any()
  }
}
```

##==##

<!-- .slide: class="with-code" -->

# Démarrons une partie
src/components/App.rs
```rust
//[...]
  <button on:click=move |_| {is_game_over = false} >
    "Rejouer" 
  </button>
//[...]
```

##==##

<!-- .slide: class="with-code" -->

# Soyons plus réactif
src/components/App.rs
```rust
pub fn App() -> impl IntoView {
  let is_game_over = true;
   view! { 
     {move || if *state.read() {
//[...]
        <button on:click=move |_| {is_game_over.set(false)} >
          "Rejouer" 
        </button>
//[...]
```

##==##

<!-- .slide: class="with-code" -->

# Découpons notre code
src/components/game_over_overlay.rs
```rust
#[component]
pub fn GameOverOverlay(state: RwSignal<bool>) -> impl IntoView {
  view! {
        {move ||
             if *state.read() {
                 view! {
                     /* [...] */
                 }.into_any()
             } else {
                 view! {}.into_any()
             }
        }
    }
}
```

##==##

<!-- .slide: class="with-code" -->

# Découpons notre code
src/components/app.rs
```rust
use crate::components::game_over_overlay::GameOverOverlay;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
  let is_game_over = RwSignal::new(false);

  view! {
        <GameOverOverlay state=is_game_over />
    }
}
```

##==##

<!-- .slide: class="with-code" -->

# Notre premier effet: Initialiser le jeu
src/components/app.rs
```rust
// [...]
pub fn App() -> impl IntoView {
  let is_game_over = RwSignal::new(false);
  let (board, set_board) = signal(Board::new(10));

  Effect::new(move |_| {
    if !is_game_over.get() {
      console_log("run only when is_game_over change to false");

      set_board.set(Board::new(10))
    }
  });
  // [...]
}
```

##==##

<!-- .slide: class="with-code" -->

# Une case enfin !
src/components/cell.rs
```rust
use leptos::prelude::*;
use crate::model::board::Case;

#[component]
pub fn Cell(case: Case) -> impl IntoView {
  if case.is_revealed() && !case.is_mine() {
    view! {
            <div
                class="cell square revealed"
            >
            {case.get_mines_around()}
            </div>
        }.into_any()
  } else {
    // [...]
  }
}
```

##==##

<!-- .slide: class="with-code" -->

# Une case enfin !
src/components/cell.rs
```rust
// [...]
  } else {
    view! {
            <button
                class="cell square"
                class:revealed=move || case.is_revealed()
                class:mine=move || case.is_revealed() && case.is_mine()
                class:flag=move || case.is_flagged()
            />
        }.into_any()
  }
}
```

##==##

<!-- .slide: class="with-code" -->

# Une case enfin !
src/components/app.rs
```rust
 view! {
        <div class="square">
            <div class="grid" style="grid-template-columns: repeat(10, 1fr); grid-template-rows: repeat(10, 1fr); ">
                {board.read()
                    .iter()
                    .enumerate()
                    .map(|(idx, case)| {
                        // [...]
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
        <GameOverOverlay state=is_game_over />
    }
```

##==##

<!-- .slide: class="with-code" -->

# Une case enfin !
src/components/app.rs
```rust
.map(|(idx, case)| {
  let case_cloned = case.clone();
  view! {
    <Cell
      case=case_cloned
      on:click = move |_| {
        console_log(&format!("click on cell {}", idx));
        if case_cloned.is_mine() {
          is_game_over.set(true);
        }
        let mut new_board = board.read_untracked().clone();
        let x = idx.div_euclid(10) ;
        new_board.reveal_case(x, idx -(x*10));
        set_board.set(new_board);
      }
    />
  }
})
```

##==##

<!-- .slide: class="with-code" -->

# Evitons de cloner board à chaque fois
src/model/board_store.rs
```rust
use reactive_stores::Store;
use crate::model::board::{Board, Case};

#[derive(Store, Debug, Clone)]
pub struct GameState {
  #[store(key:String = |case| case.get_key())]
  pub rows: Vec<IndexCase>,
}

#[derive(Debug, Store, Clone)]
pub struct IndexCase {
  pub idx: usize,
  pub case: Case,
}

impl IndexCase {
  pub fn get_key(&self) -> String {
    self.idx.to_string() + &self.case.is_flagged().to_string() + &self.case.is_revealed().to_string()
  }
}
```

##==##

<!-- .slide: class="with-code" -->

# Donnons des fonctions de mise à jour à notre store
src/model/board_store.rs
```rust
use leptos::prelude::Update;

// [...]
impl Update for IndexCase {
  type Value = IndexCase;

  fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
    let (changed, res) = fun(&mut self.clone());
    if changed {
      Some(res)
    } else {
      None
    }
  }
}

```

##==##

<!-- .slide: class="with-code" -->

# utilisons le store dans App
src/model/app.rs
```rust
// [...]
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
  // [...]
}
```

##==##

<!-- .slide: class="with-code" -->

# utilisons le store dans App
src/model/app.rs
```rust
// [...]
<For
  each=move ||  state.rows()
  key=|idx_case| idx_case.get().get_key()
  let:idx_case
  >
    <Cell
      case=idx_case.get().case.clone()
      on:click = move |_| {
        // [...]
      }
    />
</For>
  // [...]
}
```

##==##

<!-- .slide: class="with-code" -->

# utilisons le store dans App
src/model/app.rs
```rust
// [...]
      on:click = move |_| {
        console_log(&format!("click on cell {}", idx_case.get().idx));
        if idx_case.get().case.is_mine() {
          is_game_over.set(true);
        }
        idx_case.update(|c| {
          c.case.reveal();
        });
      }
  // [...]
}
```

##==##

<!-- .slide: class="with-code" -->

# Et si on évitait la triche ? 
Créons un backend avec Axum. Le plus simple est d'utiliser un template.
``` bash
# on charge une série d'utilitaires pour leptos
cargo install --locked cargo-leptos

# nouveau projet avec un template Axum
cargo leptos new --git https://github.com/leptos-rs/start-axum
```

##==##

<!-- .slide: class="with-code" -->

# Et si on évitait la triche ?
peu de changement dans le code client
un nouveau fichier main qui gère le serveur

