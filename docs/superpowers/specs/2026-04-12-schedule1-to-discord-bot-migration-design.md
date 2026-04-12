# Schedule1 UI Migration to Discord Bot

## Summary

Migrate all UI components and domain logic from the `schedule1` package into `discord_bot`, accessible via a new `/schedule1` route. After successful migration and verification, delete the `schedule1` package entirely.

## Goal

Consolidate the schedule1 drug mixing calculator into discord_bot to:
- Reduce monorepo maintenance overhead
- Share authentication/infrastructure infrastructure
- Single deployment for both features

## Non-Goals

- Refactoring component behavior or UI design
- Adding database persistence for recipes(localStorage only)
- Adding authentication requirements to the calculator
- Creating shared libraries between packages

## Architecture

### File Structure

```
discord_bot/src/
├── schedule1/
│   ├── mod.rs           # Re-exports domain + components
│   ├── domain.rs        # Moved from sellable.rs (MixState, Product, effects logic)
│   └── components/
│       ├── mod.rs       # Re-exports all components
│       ├── add_ingredients.rs
│       ├── addictiveness.rs
│       ├── base_products.rs
│       ├── causes.rs
│       ├── expenses.rs
│       ├── mix_map.rs
│       ├── price_per_unit.rs
│       ├── pseudo_options.rs
│       ├── saved_recipes.rs
│       ├── save_product.rs
│       ├── sell_prices.rs
│       ├── soil_options.rs
│       ├── total_price.rs
│       └── yield_options.rs
├── views/
│   ├── schedule1.rs     # NEW: Route view wrapping ScheduleApp
│   └── ...existing...
└── app.rs               # Add Schedule1 route variant
```

### Components Being Migrated

| Component | Purpose |
|-----------|---------|
| `BaseProducts` | Select base product (Marijuana strains, Meth, Cocaine) |
| `AddIngredients` | Grid of buttons to add ingredients |
| `SavedRecipes` | Display saved recipe buttons |
| `SaveProduct` | Save/load recipes with bookmark icon |
| `YieldOptions` | PGR checkbox, Tent/Pot toggle |
| `SoilOptions` | Fertilizer/Speed Grow checkboxes, soil quality buttons |
| `PseudoOptions` | Quality selector for Meth pseudo ingredients |
| `Expenses` | Cost breakdown display |
| `PricePerUnit` | Per-unit pricing display |
| `TotalPrice` | Total computed price display |
| `SellPrices` | Sell prices for baggie/jar/brick |
| `Addictiveness` | Addictiveness percentage display |
| `Causes` | Effect multipliers display |
| `MixMap` | Complex SVG circular effect visualization |

### Domain Logic (`domain.rs`)

- `Product` enum - Base products (Marijuana variants, Meth, Cocaine)
- `Ingredient` enum - Additives (Cuke, Banana, Paracetamol, etc.)
- `Effect` enum - Drug effects (CalorieDense, Caffeinated, etc.)
- `Sellable` struct - Product + ingredients + modifiers
- `MixState` struct - Reactive state for the calculator
- `Quality` enum - Quality levels
- Mix calculations, pricing formulas, addictiveness calculations

## Dependencies

Add to `discord_bot/Cargo.toml`:

```toml
dioxus-free-icons = { workspace = true, features = ["octicons"] }
dioxus-primitives = { workspace = true }
```

Both dependencies are already workspace members, just not included in discord_bot.

## Routing Changes

### `discord_bot/src/app.rs`

Add route variant:

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/modpack")]
    Modpack {},
    #[route("/schedule1")]  // NEW
    Schedule1 {},
    #[route("/privacy")]
    PrivacyPolicy {},
    #[route("/terms")]
    TermsOfService {},
}
```

### `discord_bot/src/components/navbar.rs`

Add navigation link:

```rust
// Add to navbar links
Link {
    to: Route::Schedule1 {},
    "Schedule1"
}
```

## State Management

- `MixState` remains as signal-based local state
- No server-side persistence
- LocalStorage for saved recipes (client-side only)

## Migration Steps

1. Add dependencies to discord_bot
2. Create `discord_bot/src/schedule1/` module structure
3. Copy `sellable.rs` → `domain.rs` with import path updates
4. Copy all components to `discord_bot/src/schedule1/components/`
5. Create `discord_bot/src/views/schedule1.rs` view
6. Update `Route` enum with new route
7. Update navbar with new link
8. Verify build: `cargo check --package discord_bot`
9. Test manually: `dx serve --package discord_bot --platform web`
10. Delete `schedule1/` package
11. Update workspace `Cargo.toml` to remove schedule1 member

## Testing

- Manual testing: Verify calculator functionality on `/schedule1` route
- All existing schedule1 component behavior preserved
- No automated tests exist upstream, none required

## Rollback

If issues arise:
1. Revert git commits
2. schedule1 package remains in git history

## Success Criteria

- All15 components render correctly in discord_bot
- Calculator functionality works identically to standalone schedule1
- Build passes: `cargo check --workspace`
- schedule1 package deleted from monorepo