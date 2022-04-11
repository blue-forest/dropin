# Memory Management

## Mapping

```
╔════════════╗
║    null    ║
╠════════════╣
║    data    ║
╠════════════╣
║   buffers  ║
╠════════════╣
║    refs    ║
╠════════════╣
║   values   ║
╚════════════╝
```

### null

The Memory begins with the null region. It is composed of 16 bytes used to write
unused values, or values that are used directly after their creation.

### data

The data region stores all literal values. For example, in the instruction:
```dropin
print "hello world"
```
the string "hello world" resides in this region.

### buffers

A module needs fix-sized buffers to comminucate with the other processes. They
are located in the buffer region.

### refs & values

In the future, modules will be able to define high level types. Values of these
types will be pointers called refs.

For example, this recipe:

```dropin
format
  object
    key
      list
        text

data
  key
    "test1"
    "test2"
```

would produce:
```
     ...
╠═════════════════╣
║       refs      ║
║┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄║
║  0: data.key    ║
║       12        ║
║─────────────────║
║  4: data.key.0  ║
║        3        ║
║─────────────────║
║  8: data.key.1  ║
║        3        ║
╠═════════════════╣
║      values     ║
║┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄║
║ 12: length      ║
║        2        ║
║╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌║
║ 16: element     ║
║        4        ║
║╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌║
║ 20: element     ║
║        8        ║
║─────────────────║
║ 24: length      ║
║        5        ║
║╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌║
║ 28: text        ║
║     "test1"     ║
║─────────────────║
║ 32: length      ║
║        5        ║
║╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌║
║ 36: text        ║
║     "test2"     ║
╚═════════════════╝
```

