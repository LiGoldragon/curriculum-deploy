# Architecture

## Layers

Text -> Protos -> Datom -> generated Rust values.

Realization (inbound): `Potential<T>::actualize` with an explicit parse budget.
Textualization (outbound): `datomize` -> `protosize` -> `textualize`.

## Modules

- `curriculum-deploy.ethos` -- the type declarations.
- `src/generated.rs` -- committed output of ethos-zero; freshness-tested.
- `src/runtime.rs` -- CLI dispatch, root-head convention, Deployment logic,
  skill template rendering, and the typed Datom conversion chain.
- `src/roles.rs` -- role packet assembly from the Roles data.
- `src/main.rs` -- entry point.

## Root-head convention

Standalone datom files (roles.datom, generated-role-outputs.datom) carry
a named variant head: `Roles.{ ... }`, `GeneratedRoleOutputs.{ ... }`.
The generated document enums represent these heads. Generated
`Datomizable` and `Composing` implementations handle the complete value.
