# Upgrades

## Initial runtime split

Consumers use two pinned public inputs: this runtime flake and the external
Curriculum data source. Regenerate consumer outputs with a single inline Datom
configuration carrying the pinned data root, consumer workspace root, and
requested operation.

Regenerate all consumer-owned output rather than editing generated files. The
runtime accepts no flags, request files, environment configuration, or
current-directory default; its only configuration is the inline typed object.

## Parent-child flow contract

Update the pinned Curriculum input, then regenerate the consumer. The new
projection replaces `flows` and `subflows` with `main-flow`, `child-flow`, and
`flow-evidence`. The generated contract requires callers to pass `FLOW_ID`,
`FLOW_DIRECTORY`, and `THREAD_ID` in each child brief; this runtime does not
inject those values into a vendor harness.

Generation now replaces the runtime-owned generated skill trees. Do not keep
manual or retired sources below `.agents/skills` or `.claude/skills`; place
other instruction sources outside those generated roots.
