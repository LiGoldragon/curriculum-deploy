# curriculum-deploy

Curriculum data deployment runtime. Projects a Curriculum data checkout into
a consumer workspace: generates skill companions, role packets, and cleanup
inventories.

## Version

0.6.2

## Usage

The CLI accepts one inline datom value and no flags:

```
curriculum-deploy 'Generate.{ «/path/to/curriculum» «/path/to/workspace» }'
curriculum-deploy 'Check.{ «/path/to/curriculum» «/path/to/workspace» }'
curriculum-deploy 'Visualize.{ «/path/to/curriculum» «/path/to/workspace» }'
```


Output is printed as datom on stdout. Faults are printed as datom on stderr.

## Dependencies

| Crate | Version | Rev |
|---|---|---|
| protos | 0.29.1 | b543678cfc86 |
| datom-codec | 0.25.6 | f2cc06858d38 |
| ethos-zero | 6.1.4 (dev)| 79e51c0f9103 |

## Ethos declaration

The request, output, and role-packet types are declared in
`curriculum-deploy.ethos` as an ethos Library. The generated Rust module
`src/generated.rs` is committed and verified fresh by a test that reads
the ethos file through the ethos-zero library and compares the emitted
output.
