# curriculum-deploy

Curriculum data deployment runtime. Projects a Curriculum data checkout into
a consumer workspace: generates skill companions, role packets, and cleanup
inventories.

## Version

0.6.3

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
| protos | 0.30.1 | 171b21f65337 |
| datom-codec | 0.27.0 | 6dccc76b7591 |
| ethos-zero | 9.0.0 (dev)| b232d35e0301 |

## Ethos declaration

The request, output, and role-packet types are declared in
`curriculum-deploy.ethos` as an ethos Library. The generated Rust module
`src/generated.rs` is committed and verified fresh by a test that reads
the ethos file through the ethos-zero library and compares the emitted
output.
