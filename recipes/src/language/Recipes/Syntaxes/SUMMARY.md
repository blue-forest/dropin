# Syntaxes

Syntaxes are sets of patterns used to parse and validate a text. When a Syntax
parses a text, it produces **Expressions** that can be used into [Modules](../Modules/SUMMARY.md).

## Literals

```dropin
syntaxes example
================
pattern "hello"
```

- ✅ `hello`
- ❌ `bye`
- ❌ `prefixhello`
- ❌ `hellosuffix`

You can escape:
- `\"`
- `\n`
- `\r`
- `\t`
- `\0`


## Quantifiers

```dropin
syntaxes example
================
pattern "ab"{1..}
```

- ✅ `ab`
- ✅ `abab`
- ✅ `ababab`
- ❌ `<empty>`
- ❌ `aba`
- ❌ `baba`

## Getters

```dropin
syntaxes example
================
outer $patterns.inner
inner $std.alphanum{1..}
```

No left recursion.

## Concatenation

```dropin
syntaxes example
================
command "print " $patterns.message
message $std.alphanum{1..}
```

## Alternatives

```dropin
command $patterns.print | $patterns.set
set     "set " $std.alphanum{1..} " \"" $std.alphanum{..} "\""
print   "print \"" $std.alphanum{..} "\""
```

## Priorities

- parentheses
- quantifier
- concatenation
- alternatives

```dropin
syntaxes example
================
forExampleThis "a" "b" "c"{..} | "d"{..} "e" "f"
isEquivalentTo ("ab" ("c"){..}) | (("d"){..} "ef")
```

## JSON example

```dropin
syntaxes json
=============

token
  $patterns.list
  | $patterns.object
  | $patterns.text
  | $patterns.quantity
  | $patterns.boolean
  | "null"

list "[" $patterns.listTokens{..1} "]"

listTokens $patterns.token ("," $patterns.token){..}

object "{" $patterns.keyValues{..1} "}"

keyValues $patterns.keyValue ("," $patterns.keyValue){..}

keyValue $patterns.key ":" $patterns.token

key "\"" $patterns.keyChar "\""

keyChar $std.alphanum | "-" | "_" | ":"

quantity "-"{..1} "1"-"9" $std.digit{..}
  $patterns.quantityDot{..1} $patterns.quantityExponent{..1}

quantityDot "." $std.digit{1..}

quantityExponent ("e" | "E") ("-" | "+"){..1} $std.digit{1..}

text "\"" (
    "\\" ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\u" $std.hex{4}
    | !"\\"
  ){..} "\""

boolean "true" | "false"
```

## Future

**None of the following propositions are guaranteed to be implemented as is, or at all.**

- unicode escapes
- ghost patterns
- stacks
- config

## Further readings

- https://en.wikipedia.org/wiki/Formal_grammar
- https://en.wikipedia.org/wiki/Parsing_expression_grammar
- https://en.wikipedia.org/wiki/Pushdown_automaton
- https://en.wikipedia.org/wiki/Yacc
- https://www.youtube.com/watch?v=yTXCPGAD3SQ
- https://pest.rs
