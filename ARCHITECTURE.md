# Architecture

`curriculum-deploy` is a pure runtime repository. Its package source contains only Rust, templates, tests, Nix packaging, and repository documentation.

The public process contract is one inline Datom object. The root operation owns all execution configuration: `data_root`, `workspace_root`, and mode. The process never supplements that object from the environment, current directory, flags, or a configuration file.

Curriculum is external data. The runtime discovers runtime skill Markdown files by their presence under its external data root and realizes the typed `Roles` Datom record from that root. The record owns role modules, model/effort selections, permission policy, descriptions, aliases, and target insertions; the runtime only formats those decisions for each harness surface.

Eight described role cross-products project to Claude, Codex, and Pi packets.
The three typed aliases target Codex, yielding 27 role packets. The consumer
inventory is `skills/generated-role-outputs.datom`; each generation realizes
the previous inventory and removes only its listed stale role files before
writing the current typed inventory.

The generated harness trees are consumer state. They are not source or package input here.
