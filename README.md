# Tauri + SvelteKit

This template should help get you started developing with Tauri and SvelteKit in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

.
.
# PACKAGE INSTALLED FRONTEND
## SVELTE 
 - [tailwind](https://tailwindcss.com/docs/installation/using-vite)
 - [Flowbite Svelte](https://flowbite-svelte.com/docs/components/accordion)


# Package Installed BACKEND RUST
- [databases => sqlx-cli](https://docs.rs/sqlx_wasi/latest/sqlx/)

## SQLX
- make migaration table
``` zsh 
sqlx migrate add create_users_table
```
- migrate 
```zsh
sqlx migrate run
```
- rollback migartion
```zsh
sqlx migrate revert

```