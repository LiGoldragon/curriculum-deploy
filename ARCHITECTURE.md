# Architecture

`curriculum-deploy` is a pure runtime repository. Its package source contains only Rust, templates, tests, Nix packaging, and repository documentation.

The public process contract is one inline Datom object. The root operation owns all execution configuration: `data_root`, `workspace_root`, and mode. The process never supplements that object from the environment, current directory, flags, or a configuration file.

Curriculum is external data. The runtime discovers runtime skill Markdown files by their presence under its external data root and realizes the typed `Roles` Datom record from that root. The record owns role modules, model/effort selections, permission policy, descriptions, aliases, and target insertions; the runtime only formats those decisions for each harness surface.

Eight described role cross-products project to Claude, Codex, and Pi packets.
The three typed aliases target Codex, yielding 27 role packets. The consumer
inventory is `skills/generated-role-outputs.datom`; each generation realizes
the previous inventory and removes only its listed stale role files before
writing the current typed inventory.

Parent-child flow identity is data, not runtime state. `main-flow` emits the
contract that the parent claims its normalized hexadecimal alias and lane with
the installed `flow-id` helper before its first artifact, then carries
`FLOW_ID` and `FLOW_DIRECTORY` in each child brief. `child-flow` preserves
both values for nested children and obtains its own `THREAD_ID` after launch.
The runtime has no vendor-harness identity hook and must not invoke or invent
one.

The generated harness trees are consumer state. They are not source or package input here.
Generation removes stale immediate skill directories from `.agents/skills` and
`.claude/skills`, so removed Curriculum sources leave no parallel generated
contract. Every `user-only: true` source also projects Codex's
`allow_implicit_invocation: false` companion beneath its current skill
directory.

The flake pins Curriculum as a non-flake input exclusively for its
`external-data` check. That check passes the input root through the typed CLI
request and runs on remote builders. The engine package uses its own Cargo-only
source and has no Curriculum input or closure edge.
