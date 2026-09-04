# Upgrades

## 0.3.0

Breaking upgrade from the retired `datom` crate and old protos API to
the ProtoformStack train: datomic 0.8.0 and protos 0.15.0.

### What changed

- Dependency `datom` (github:LiGoldragon/datom) replaced by `datomic`
  (github:LiGoldragon/datomic).
- Dependency `protos` updated from bfea114c to 56c683ec.
- All hand-written DatomRealizing/DatomTextualizing impls replaced by
  generated Datomic (Corporal + datomize) impls from an ethos Library.
- Request root changed from `CurriculumRequest.{ Generate.{ ... } }` to
  `Generate.{ ... }` (the old form is still accepted).
- Canonical print uses spaced delimiters: `{ a b }` not `{a b}`.
- generated-role-outputs.datom paths are curly-quoted when they contain
  separator characters (e.g., `.codex/agents/file.toml`).
- Curriculum input bumped to 5716f71a (child-flow -> subflow rename).

### How to deploy

1. Bump primary's `curriculum-deploy` flake input to the new rev.
2. Ensure primary's `curriculum` input is at 5716f71a or later.
3. Regenerate: `nix run .#generate-skills 'Generate.{ /path/to/Curriculum /home/li/primary }'`
   (or with the CurriculumRequest wrapper).
4. Commit and push the regenerated trees.
