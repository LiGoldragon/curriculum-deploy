# Architecture

## Layers

Text -> Protos -> Datom -> generated Rust values.

Realization (inbound): `Potential<T>::actualize` with an explicit parse budget.
Textualization (outbound): `datomize` -> `protosize` -> `textualize`.
Skill deployment reads the authored Markdown of every declared skill source,
each declared with its aspect (Psyche, Mind or Field), unions them into one
catalog ordered by name, refuses a skill defined by two sources, and renders
the catalog into each supported skill surface. The Curriculum checkout gives
the role data.

## Modules

- `curriculum-deploy.ethos` -- the type declarations.
- `src/generated.rs` -- committed output of ethos-zero; freshness-tested.
- `src/runtime.rs` -- CLI dispatch, root-head convention, Deployment logic,
  skill template rendering, and the typed Datom conversion chain.
- `src/catalog.rs` -- declared skill sources and the merged skill catalog.
- `src/roles.rs` -- role packet assembly from the Roles data.
- `src/main.rs` -- entry point.

## Root-head convention

Standalone datom files (roles.datom, generated-role-outputs.datom) carry
a named variant head: `Roles.{ ... }`, `GeneratedRoleOutputs.{ ... }`.
The generated document enums represent these heads. Generated
`Datomizable` and `Composing` implementations handle the complete value.
