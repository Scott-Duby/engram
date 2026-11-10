# agent-brain

> Working name. Early development: nothing here is ready to install yet.

An open-source brain for the AI agent you already use. It gives the agent a long-term memory in an Obsidian vault, a set of operating rules, and skills for recurring jobs. Later it adds a companion app for voice, an animated face, and a chat window.

It does not replace your AI tool. It runs inside the agent tool you already have, on your own account:

| Tool | Models | How you sign in |
|---|---|---|
| Claude Code | Claude | Your Claude plan or an API key, in Claude Code itself |
| Codex CLI | GPT | Your ChatGPT plan or an API key, in Codex itself |
| Gemini CLI | Gemini | Your Google account, in Gemini CLI itself |
| OpenCode | Other providers and local models | Set up in OpenCode itself |

## Layout

| Folder | What goes there |
|---|---|
| `brain/` | The rules (`AGENTS.md`) and the skills (`SKILL.md` format), shared by every tool |
| `vault-template/` | The starter Obsidian vault: folders, indexes, daily note template |
| `adapters/` | One installer per agent tool, placing `brain/` where that tool looks for it |
| `crates/` | Rust: `core` (vault, display, and settings logic), `mcp-server` (the actions every agent tool can call), `cli` (the installer) |
| `apps/companion/` | The voice, face, and chat app (Tauri 2, React, TypeScript, Tailwind CSS, daisyUI) |
| `voice/` | Python helper for the companion: the agent session, speech-to-text, and voices |

## Your account and your data

- **This project never handles your login.** You sign in to your agent tool yourself, in that tool. Nothing here shows a login screen, asks for a password, or reads, stores, or sends your credentials or tokens.
- **The companion app uses your installed Claude Code.** When you use it with Claude, it drives the copy of Claude Code on your machine, so usage counts against your own Claude plan or API key, under your own agreement with Anthropic. Other providers use your own API key for that provider.
- **Your vault stays on your machine.** Nothing in this project uploads your notes anywhere. What your agent tool sends to its model provider is governed by that provider's terms.
- **Actions ask first by default.** The default permission setting asks before the agent changes files or runs commands. Turning that off is your choice, made in your own settings.

## Not affiliated

This is an independent community project. It is not made, endorsed, or supported by Anthropic, OpenAI, Google, Obsidian, or any other company named here. Claude, Claude Code, ChatGPT, Codex, Gemini, and Obsidian are trademarks of their owners and are used only to say what this project works with. Use of each tool is subject to that provider's own terms.

## License

- Code: GNU Affero General Public License v3.0 or later (`LICENSE`).
- The vault template and the rules and skills in `brain/`: Creative Commons Attribution-ShareAlike 4.0 (`LICENSES/CC-BY-SA-4.0.txt`), because they build on AI Memory Vault.

Details in `LICENSING.md`. Credits in `CREDITS.md`.
