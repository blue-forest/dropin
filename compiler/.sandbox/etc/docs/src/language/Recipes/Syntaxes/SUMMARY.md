# Syntaxes

Syntaxes are sets of patterns used to parse and validate a text. When a Syntax
parses a text, it produces **Expressions** that can be used into [Modules](../Modules/SUMMARY.md).

## Literals

Literals are text parts that match an exact same text value.

You can escape:
- `\"` for a non-closing `"`
- `\n` for a new line
- `\r` for a carriage return
- `\t` for a tabulation
- `\0` for a null character

For example:

```dropin
syntaxes example
================
pattern "\"hello\""
```

matches:

- ✅ `"hello"`
- ❌ `bye`
- ❌ `hello`
- ❌ `\"hello\"`
- ❌ `prefix"hello"`
- ❌ `"hello"suffix`

## Quantifiers

You can define how many times a token repeats with quantifiers. They stand after
the token to repeat.

The most common form of quantifiers is `{<min>..<max>}`, replacing `<min>` and
`<max>` with the desired values.

For example:

```dropin
syntaxes example
================
pattern "ab"{1..3}
```

matches:

- ✅ `ab`
- ✅ `abab`
- ✅ `ababab`
- ❌ `abababab`
- ❌ `<empty>`
- ❌ `aba`
- ❌ `baba`

If no max value is given, the token repeats as many times as needed. For
example:

```dropin
syntaxes example
================
pattern "ab"{2..}
```

matches:

- ❌ `ab`
- ✅ `abab`
- ✅ `ababab`
- ✅ `abababab`
- ❌ `<empty>`
- ❌ `aba`
- ❌ `baba`

If no min is given, the token can not appear at all. For example:

```dropin
syntaxes example
================
pattern "ab"{..2}
```

matches:

- ✅ `ab`
- ✅ `abab`
- ❌ `ababab`
- ❌ `abababab`
- ✅ `<empty>`
- ❌ `aba`
- ❌ `baba`

You can ommit both min and max, the number of repetition is therefore undefined.
For example:

```dropin
syntaxes example
================
pattern "ab"{..}
```

matches:

- ✅ `ab`
- ✅ `abab`
- ✅ `ababab`
- ✅ `abababab`
- ✅ `<empty>`
- ❌ `aba`
- ❌ `baba`

If both min and max are the same, you can use this useful shortcut:

```dropin
syntaxes example
================
pattern "ab"{2}
```

It matches:

- ❌ `ab`
- ✅ `abab`
- ❌ `ababab`
- ❌ `abababab`
- ❌ `<empty>`
- ❌ `aba`
- ❌ `baba`

The `|` operator skips some unacceptable repetition numbers. For example:

```dropin
syntaxes example
================
pattern "ab"{0|3..}
```

matches:

- ❌ `ab`
- ❌ `abab`
- ✅ `ababab`
- ✅ `abababab`
- ✅ `<empty>`
- ❌ `aba`
- ❌ `baba`

## Getters

You can define several patterns in a syntax. A pattern can include another one
with a [Getter](../../Concepts/Getters.md) to `patterns`.

For example:

```dropin
syntaxes example
================
includer $patterns.included{1..}
included "b"
```

is equivalent to:

```dropin
syntaxes example
================
standAlone "b"{1..}
```

In addition, there are some pre-defined patterns which you can use with a
[Getter](../../Concepts/Getters.md) to `std`.

- **std.alpha**: Any letter from the latin alphabet
  - ✅ `d`
  - ✅ `R`
  - ✅ `ó`
  - ❌ `🅿`
  - ❌ `'`
  - ❌ `1`
  - ❌ `И`
- **std.alphaI18n**: Any letter from any alphabet
  - ✅ `d`
  - ✅ `R`
  - ✅ `ó`
  - ❌ `🅿`
  - ❌ `'`
  - ❌ `1`
  - ✅ `И`
- **std.numeric**: Any digit between 0 and 9
  - ❌ `d`
  - ❌ `R`
  - ❌ `ó`
  - ❌ `🅿`
  - ❌ `'`
  - ✅ `1`
  - ❌ `И`
- **std.alphaNumeric**: Any letter from the latin alphabet or digit
  - ✅ `d`
  - ✅ `R`
  - ✅ `ó`
  - ❌ `🅿`
  - ❌ `'`
  - ✅ `1`
  - ❌ `И`
- **std.alphaI18nNumeric**: Any letter or digit
  - ✅ `d`
  - ✅ `R`
  - ✅ `ó`
  - ❌ `🅿`
  - ❌ `'`
  - ✅ `1`
  - ✅ `И`
- **std.hexadecimal**: Any hexadecimal digit between 0 and F (case-insensitive)
  - ✅ `d`
  - ✅ `D`
  - ❌ `R`
  - ❌ `ó`
  - ❌ `🅿`
  - ❌ `'`
  - ✅ `1`
  - ❌ `И`

## Concatenation

You can define several tokens in a pattern. When separated by whitespaces, it is
interpreted as "this first token, and then this second one".

For example:

```dropin
syntaxes example
================
pattern "a" "b"
```

is equivalent to:

```dropin
syntaxes example
================
pattern "ab"
```

You can define a concatenation on multiple lines, the folowing lines should be
indented (shifted by at leat one space):

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK "a"
  "b"
```

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK
  "a"
  "b"
```

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK
  "a" "b"
```

❌ **This is not OK**
```dropin
syntaxes example
================
doNotDoThis "a"
"b"
```

## Alternatives

Another combination of several tokens is alternatives. When separated by `|`, it
is interpreted as "this first token, or this seconc one"

For example:

```dropin
syntaxes example
================
pattern "a" | "b"
```

matches:

- ✅ `a`
- ✅ `b`
- ❌ `c`

As concatenations, you can define it on multiple indented lines:

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK "a"
  | "b"
```

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK "a" |
  "b"
```

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK
  "a"
  | "b"
```

✅ **This is OK**
```dropin
syntaxes example
================
thisIsOK
  "a" | "b"
```

The order matters ! The expression correspond to the first recognized pattern.

For example when this syntax:
```dropin
syntaxes example
================
letter $std.alpha | $patterns.letterA
letterA "a"
```

parses `a`, the pattern `letterA` is not recognized because it matches
`$std.alpha`.

Left recursions are not allowed. That means that if you call the pattern itself,
it has to be the last alternative.

❌ **This is not OK**
```dropin
syntaxes example
================
doNotDoThis $patterns.doNotDoThis "whatever" | "end"
```

✅ **This is OK**
```dropin
syntaxes example
================
thisIsLegit "end" | $patterns.thisIsLegit "whatever"
```

## Priorities

The priority order for all these features is:

- literal / getter
- quantifier
- concatenation
- alternatives

It means that, by default:
- quantifiers can only be applied to literals or getters
- concateration can only be applied to quantifiers, literals or getters 
- alternatives can be applied to everything

You can change this priorities with parentheses.

For example:

```dropin
syntaxes example
================
sentence ("tomatoes" | "pineapples") "are fruits"
```

is equivalent to:

```dropin
syntaxes example
================
sentence $patterns.fruits "are fruits" 
fruits  "tomatoes" | "pineapples"
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
