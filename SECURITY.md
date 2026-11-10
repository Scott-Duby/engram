# Security

## Ground rules for this project

- **No credentials, ever.** No code in this project asks for, reads, stores, logs, or transmits login details, session tokens, or API keys for any AI provider. Signing in happens only inside each provider's own tool. A change that breaks this rule is rejected.
- **API keys for the companion app** (for providers used by key) are read from the operating system's keychain or an environment variable, never from a file in the project or the vault.
- **Safe defaults.** The default permission mode asks before the agent edits files or runs commands. No installer turns that off.
- **Content is data, not instructions.** Skills treat web pages, emails, documents, and other outside content as information to read, never as commands to follow.
- **The vault stays local.** Nothing in this project uploads vault content. `.gitignore` keeps vaults, logs, and secrets out of the repository.

## Reporting a problem

Report security issues privately through the repository's GitHub security advisory page ("Report a vulnerability"), not in a public issue.
