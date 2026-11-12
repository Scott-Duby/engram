---
status: active
project: meta
type: index
---
# Vault Map

What each of the owner's folders holds, so the assistant knows where to look and where new notes go. One row per top-level folder; each folder's notes are listed in its index in `Agent/Indexes/`. Update this map in the same pass as any folder created, renamed, or removed.

| Folder | Holds | Index |
|---|---|---|
| {{Inbox}} | Quick captures, sorted later | [[{{Inbox}} Index]] |
| {{Projects}} | One subfolder per active project | [[{{Projects}} Index]] |
| {{Resources}} | Reference material used across projects | [[{{Resources}} Index]] |
| {{Archive}} | Finished projects and old notes | [[{{Archive}} Index]] |
| Agent | The assistant's folder | [[Agent]] |

Optional rows, added at setup when chosen: Daily Notes, Templates, Tags.

## Placing a new note
1. If it belongs to an existing project, it goes in that project's folder.
2. Otherwise pick the folder whose "Holds" fits; if none fits, it goes in the inbox.
3. Assistant notes (runbooks, logs, indexes) always go in `Agent/`, never in the owner's folders.
