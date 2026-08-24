# Upgrades

## Initial runtime split

Consumers use two pinned public inputs: this runtime flake and the external
Curriculum data source. Regenerate consumer outputs with a single inline Datom
configuration carrying the pinned data root, consumer workspace root, and
requested operation.

Regenerate all consumer-owned output rather than editing generated files. The
runtime accepts no flags, request files, environment configuration, or
current-directory default; its only configuration is the inline typed object.
