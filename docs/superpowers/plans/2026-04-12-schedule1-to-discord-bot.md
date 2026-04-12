# Schedule1 UI Migration to Discord Bot Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate all schedule1 UI components and domain logic into discord_bot as a new `/schedule1` route, then delete the schedule1 package.

**Architecture:** Direct migration - copy domain logic (`sellable.rs` → `domain.rs`) and all 15 components into new`discord_bot/src/schedule1/` module. Create new view and route. Add dependencies. No refactoring of behavior.

**Tech Stack:** Dioxus 0.7, kinetic_ui, dioxus-free-icons, dioxus-primitives

---

## Task 1: Add Dependencies to discord_bot

**Files:**
- Modify: `discord_bot/Cargo.toml`

- [ ] **Step 1: Add dioxus-free-icons and dioxus-primitives dependencies**

Add to `discord_bot/Cargo.toml` in the `[dependencies]` section:

```toml
dioxus-free-icons = { workspace = true, features = ["octicons"] }
dioxus-primitives = { workspace = true }
```

---

## Task 2: Create schedule1 Module Structure

**Files:**
- Create: `discord_bot/src/schedule1/mod.rs`
- Create: `discord_bot/src/schedule1/domain.rs`
- Create: `discord_bot/src/schedule1/components/mod.rs`

- [ ] **Step 1: Create schedule1 module directory**

Run: `mkdir -p discord_bot/src/schedule1/components`

- [ ] **Step 2: Create schedule1/mod.rs**

Create `discord_bot/src/schedule1/mod.rs`:

```rust
mod components;
pubmod domain;

pub use components::*;
pub use domain::*;
```

- [ ] **Step 3: Create schedule1/components/mod.rs**

Create `discord_bot/src/schedule1/components/mod.rs`:

```rust
mod add_ingredients;
pub use add_ingredients::*;

mod base_products;
pub use base_products::*;

mod saved_recipes;
pub use saved_recipes::*;

mod price_per_unit;
pub use price_per_unit::*;

mod soil_options;
pub use soil_options::*;

mod sell_prices;
pub use sell_prices::*;

mod save_product;
pub use save_product::*;

mod expenses;
pub use expenses::*;

mod yield_options;
pub use yield_options::*;

mod pseudo_options;
pub use pseudo_options::*;

mod total_price;
pub use total_price::*;

mod addictiveness;
pub use addictiveness::*;

mod causes;
pub use causes::*;

mod mix_map;
pub use mix_map::*;
```

---

## Task 3: Copy Domain Logic (sellable.rs → domain.rs)

**Files:**
- Create: `discord_bot/src/schedule1/domain.rs`

- [ ] **Step 1: Copy sellable.rs content to domain.rs**

Copy the entire content of `schedule1/src/sellable.rs` to `discord_bot/src/schedule1/domain.rs`. This file contains:
- `OneTimeIngredient` enum
- `MixState` struct
- `Sellable` struct with all methods
- `Quality` enum
- `Product` enum
- `Ingredient` enum
- `Effect` enum with all methods
- All tests

The file is approximately 1300+ lines. No changes needed - the file has no imports that reference `crate::` paths that need updating.

---

## Task 4: Copy All Components (15 files)

**Files:**
- Create: `discord_bot/src/schedule1/components/add_ingredients.rs`
- Create: `discord_bot/src/schedule1/components/base_products.rs`
- Create: `discord_bot/src/schedule1/components/saved_recipes.rs`
- Create: `discord_bot/src/schedule1/components/price_per_unit.rs`
- Create: `discord_bot/src/schedule1/components/soil_options.rs`
- Create: `discord_bot/src/schedule1/components/sell_prices.rs`
- Create: `discord_bot/src/schedule1/components/save_product.rs`
- Create: `discord_bot/src/schedule1/components/expenses.rs`
- Create: `discord_bot/src/schedule1/components/yield_options.rs`
- Create: `discord_bot/src/schedule1/components/pseudo_options.rs`
- Create: `discord_bot/src/schedule1/components/total_price.rs`
- Create: `discord_bot/src/schedule1/components/addictiveness.rs`
- Create: `discord_bot/src/schedule1/components/causes.rs`
- Create: `discord_bot/src/schedule1/components/mix_map.rs`

- [ ] **Step 1: Copy each component file**

For each component file, copy from `schedule1/src/components/<name>.rs` to `discord_bot/src/schedule1/components/<name>.rs`.

**Important:** Update the import in each file from `use crate::sellable::` to `use crate::schedule1::domain::`.

Files and their required import changes:

1. `add_ingredients.rs` - Change `use crate::sellable::Ingredient;` to `use crate::schedule1::domain::Ingredient;`

2. `base_products.rs` - Change `use crate::sellable::{Effect, Product};` to `use crate::schedule1::domain::{Effect, Product};`

3. `saved_recipes.rs` - Change `use crate::sellable::Sellable;` to `use crate::schedule1::domain::Sellable;`

4. `price_per_unit.rs` - Change `use crate::sellable::{MixState, Sellable};` to `use crate::schedule1::domain::{MixState, Sellable};`

5. `soil_options.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

6. `sell_prices.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

7. `save_product.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

8. `expenses.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

9. `yield_options.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

10. `pseudo_options.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

11. `total_price.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

12. `addictiveness.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

13. `causes.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

14. `mix_map.rs` - Change imports from `crate::sellable` to `crate::schedule1::domain`

---

## Task 5: Create Schedule1 View

**Files:**
- Create: `discord_bot/src/views/schedule1.rs`
- Modify: `discord_bot/src/views/mod.rs`

- [ ] **Step 1: Create views/schedule1.rs**

Create `discord_bot/src/views/schedule1.rs`:

```rust
use crate::schedule1::{Effect, Ingredient, MixState, Product, Sellable};
use crate::schedule1::components::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Schedule1() -> Element {
    let mut working_product =
        use_signal(|| Sellable::from_product(Product::Marijuana(Effect::Calming)));
    let mut previous_working_product =
        use_signal(|| Sellable::from_product(Product::Marijuana(Effect::Calming)));
    let mut saved_recipes = use_signal(HashMap::<String, Sellable>::new);
    let mut mix_state = use_signal(MixState::default);
    let mut added_effect = use_signal(|| None);

    rsx! {
        div {
            display: "flex",
            justify_content: "flex-end",
            padding: "8px",
        }
        div {
            display: "grid",
            gap: "1rem",
            grid_template_columns: "minmax(365px, 1fr) minmax(240px, 1fr) minmax(150px, 1fr)",
            div {
                display: "grid",
                grid_template_columns: "repeat(3, 1fr)",
                gap: "1rem",
                align_content: "start",
                BaseProducts { set_working_product: move |product| {
                    working_product.set(Sellable::from_product(product));
                    added_effect.set(None);
                }}
                AddIngredients { add_ingredient: move |ingredient| {
                    previous_working_product.set(working_product());
                    working_product.set(working_product().add_ingredient(ingredient));
                    added_effect.set(Some(ingredient.effect()));
                }}
                SavedRecipes {
                    set_working_product: move |recipe| {
                        working_product.set(recipe);
                        added_effect.set(None);
                    },
                    working_product: working_product(),
                    saved_recipes: saved_recipes(),
                }
            }
            div {
                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                gap: "1rem",
                align_content: "start",
                SaveProduct {
                    working_product: working_product(),
                    saved_recipes: saved_recipes(),
                    set_working_product: move |recipe| working_product.set(recipe),
                    toggle_save: move |recipe: Sellable| {
                        let key = recipe.key();
                        if saved_recipes.read().contains_key(&key) {
                            saved_recipes.write().remove(&key);
                        } else {
                            saved_recipes.write().insert(key, recipe);
                        }
                    },
                }
                Expenses {
                    mix_state: mix_state(),
                    working_product: working_product(),
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                match working_product.read().base {
                    Product::Meth => rsx! {
                        PseudoOptions {
                            mix_state: mix_state(),
                            set_pseudo_quality: move |quality| mix_state.write().pseudo_quality = quality,
                        }
                    },
                    _ => rsx! {
                        YieldOptions {
                            mix_state: mix_state(),
                            working_product: working_product(),
                            toggle_ingredient: move |ingredient| {
                                if mix_state.read().ingredients.contains(&ingredient) {
                                    mix_state.write().ingredients.remove(&ingredient);
                                } else {
                                    mix_state.write().ingredients.insert(ingredient);
                                }
                            },
                            set_use_pot: move |use_pot| mix_state.write().use_pot = use_pot,
                        }
                        SoilOptions {
                            mix_state: mix_state(),
                            set_soil_quality: move |quality| mix_state.write().soil_quality = quality,
                            toggle_ingredient: move |ingredient| {
                                if mix_state.read().ingredients.contains(&ingredient) {
                                    mix_state.write().ingredients.remove(&ingredient);
                                } else {
                                    mix_state.write().ingredients.insert(ingredient);
                                }
                            },
                        }
                    }
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                PricePerUnit {
                    working_product: working_product(),
                    mix_state: mix_state(),
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                TotalPrice { working_product: working_product(), mix_state: mix_state() }
            }
            div {
                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                gap: "1rem",
                align_content: "start",
                div { style: "grid-column: 1 / -1; border: 1px solid var(--primary-color-6); padding: 0.5rem;", "Warning: Column in progress, has inaccuracies"}
                Addictiveness { working_product: working_product() }
                if !working_product.read().effects.is_empty() {
                    div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                    Causes { working_product: working_product() }
                }
                MixMap {
                    added_effect: added_effect,
                    previous_working_product: previous_working_product,
                    working_product: working_product,
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                SellPrices { working_product: working_product() }
            }
        }
    }
}
```



- [ ] **Step 2: Update views/mod.rs to export Schedule1**

Modify `discord_bot/src/views/mod.rs` to add:

```rust
mod schedule1;
pub use schedule1::*;
```

---

## Task 6: Update Route Enum

**Files:**
- Modify: `discord_bot/src/app.rs`

- [ ] **Step 1: Add Schedule1 route variant**

Modify `discord_bot/src/app.rs`:

Update the imports at the top:
```rust
use {
    crate::{
        components::Navbar,
        views::{Home, Modpack, PrivacyPolicy, Schedule1, TermsOfService},
    },
    dioxus::prelude::*,kinetic_ui::KineticTheme,
};
```

Add the Schedule1 route to the Route enum:
```rust
#[route("/schedule1")]
Schedule1 {},
```

The full Route enum should be:
```rust
#[derive(Debug, Clone, Routable, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/modpack")]
    Modpack {},
    #[route("/schedule1")]
    Schedule1 {},
    #[route("/terms-of-service")]
    TermsOfService {},
    #[route("/privacy-policy")]
    PrivacyPolicy {},
}
```

---

## Task 7: Update Navbar

**Files:**
- Modify: `discord_bot/src/components/navbar.rs`

- [ ] **Step 1: Add Schedule1 link to navbar**

Modify `discord_bot/src/components/navbar.rs` to add a link to the Schedule1 route:

```rust
use {crate::app::Route, dioxus::prelude::*};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Modpack {},
                "Minecraft Modpack"
            }
            Link {
                to: Route::Schedule1 {},
                "Schedule1"
            }
            Link {
                to: Route::TermsOfService {},
                "Terms of Service"
            }
            Link {
                to: Route::PrivacyPolicy {},
                "Privacy Policy"
            }
        }

        Outlet::<Route> {}
    }
}
```

---

## Task 8: Update lib.rs to Include schedule1 Module

**Files:**
- Modify: `discord_bot/src/lib.rs`

- [ ] **Step 1: Add schedule1 module to lib.rs**

Modify `discord_bot/src/lib.rs`:

```rust
pub mod app;
pub mod components;
#[cfg(feature = "server")]
pub mod discord;
pub mod error;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod queries;
#[cfg(feature = "server")]
pub mod schema;
pub mod schedule1;
pub mod state;
pub mod views;

rust_i18n::i18n!();
```

---

## Task 9: Verify Build

**Files:**
- None (verification only)

- [ ] **Step 1: Run cargo check on discord_bot**

Run: `cargo check --package discord_bot`

Expected: Compilation succeeds with no errors

- [ ] **Step 2: Run cargo check on workspace**

Run: `cargo check --workspace`

Expected: All packages compile successfully

---

## Task 10: Test Manually

**Files:**
- None (verification only)

- [ ] **Step 1: Start the discord_bot web server**

Run: `dx serve --package discord_bot --platform web`

Expected: Server starts without errors

- [ ] **Step 2: Verify Schedule1 route works**

Navigate to `http://localhost:8080/schedule1` (or appropriate port)

Expected: Schedule1 calculator UI renders correctly

---

## Task 11: Delete schedule1 Package

**Files:**
- Delete: `schedule1/` directory
- Modify: `Cargo.toml` (workspace root)

- [ ] **Step 1: Remove schedule1 from workspace members**

Modify the root `Cargo.toml` to remove `schedule1` from the workspace members list.

- [ ] **Step 2: Delete schedule1 directory**

Run: `rm -rf schedule1/`

- [ ] **Step 3: Verify workspace still builds**

Run: `cargo check --workspace`

Expected: All packages compile successfully without schedule1

---

## Task 12: Final Verification

**Files:**
- None (verification only)

- [ ] **Step 1: Run clippy on workspace**

Run: `cargo clippy --workspace`

Expected: No clippy errors (warnings acceptable)

- [ ] **Step 2: Format code**

Run: `nix fmt` or `cargo fmt`

Expected: Code formatted successfully

- [ ] **Step 3: Commit changes**

```bash
git add .
git commit -m "feat: migrate schedule1 UI into discord_bot as /schedule1 route"
```