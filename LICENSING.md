# Licensing

This project has two licenses, split by folder, because it builds on upstream work under two different licenses.

| Folder | License | Why |
|---|---|---|
| `brain/`, `vault-template/` | CC BY-SA 4.0 (`LICENSES/CC-BY-SA-4.0.txt`) | Adapted from AI Memory Vault, which is CC BY-SA 4.0. ShareAlike requires adaptations to keep the same license. |
| Everything else (`adapters/`, `crates/`, `apps/`, `voice/`, scripts, tooling) | AGPL-3.0-or-later (`LICENSE`, `LICENSES/AGPL-3.0-or-later.txt`) | The companion app builds on backtalk and ai-visualizer, which are AGPL-3.0-or-later. |

What that means in practice:

- Anyone may use, change, and share this project, including commercially.
- Anyone who distributes a changed version, or runs a changed version of the code as a service for other people, has to share their source under the same licenses.
- Credit to the upstream authors (see `CREDITS.md`) stays in every copy.

Rules for contributors:

- Only add third-party code or text whose license is compatible with the folder it goes in. Record it in `CREDITS.md` in the same change.
- Never paste in material from a source with unknown or incompatible terms (for example, textbook content or code without a license).
