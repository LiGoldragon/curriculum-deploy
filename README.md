# curriculum-deploy

Curriculum data deployment runtime. Projects a Curriculum data checkout into
a consumer workspace: generates skill companions, role packets, and cleanup
inventories.

## Version

0.3.0 (ProtoformStack train)

## Usage

The CLI accepts one inline datom value and no flags:

```
curriculum-deploy 'Generate.{ /path/to/curriculum /path/to/workspace }'
curriculum-deploy 'Check.{ /path/to/curriculum /path/to/workspace }'
curriculum-deploy 'Visualize.{ /path/to/curriculum /path/to/workspace }'
```

The legacy `CurriculumRequest.{ Generate.{ ... } }` wrapper is accepted
for backward compatibility.

Output is printed as datom on stdout. Faults are printed as datom on stderr.

## Dependencies

| Crate | Version | Rev |
|---|---|---|
| protos | 0.15.0 | 56c683ec8d1e |
| datomic | 0.8.0 | a27f9b8e7789 |
| ethos-zero | 1.1.0 (dev) | 31c5984c7fda |

## Ethos declaration

The request, output, and role-packet types are declared in
`curriculum-deploy.ethos` as an ethos Library. The generated Rust module
`src/generated.rs` is committed and verified fresh by a test that reads
the ethos file through the ethos-zero library and compares the emitted
output.
