# Setters

Setters are used to determine and transform data.

In the different Setters below, the values used are:

- either raw data
- or from a [Getter](/concepts/recipes/getters)

In a text, it is possible to use a Setter to make a text dynamic.

## Operations

### Arithmetic
- addition: list of quantities
- subtraction: list of quantities
- multiplication: list of quantities
- division: list of quantities
- power: quantity
- square root: quantity
- exponential: quantity
- logarithm: quantity
- modulo: two quantities

### Logic
- exists: value
- and: list of booleans
- or: list of booleans
- no: boolean

### Comparisons
- less than (<): two quantities
- at most (<=): two quantities
- at least ()>=): two quantities
- more than (>): two quantities
- equal to: two quantities
- different from: two quantities
- different from all: one value, a list of values
- different from one: one value, a list of values
- same to all: one value, a list of values
- same to one: a value, a list of values


## Formats

### Common
- length: size in characters of a value or elements of a list
- instance of: value to check, [Format](/concepts/recipes/formats) to use

### Texts
- concatenate: values, separator text (" ")
- split: value, separator text
- match: text to search, regular expression
- replace: text to search, text to replace, replacement text
- translate: value, language, number, item to use
- lowercase: text
- upper case: text
- capitalize first letters of each word (titleCase): text
- truncate leading and trailing spaces (trim): text, characters ([""]), right (true), left (true)
- abbreviate (summarize): text, maximum, end text ("...")
- transform a text with a Syntax (encode): text, [Syntax](/concepts/validations/syntaxes){.cico .cico-syntaxes} and its [Options](/concepts/recipes/options)
- pick: select a part of the text to cut with the beginning and the end

### Quantities
- average: list of quantities
- rounding: quantity, number of decimals (2), type of rounding (nearest, lower or higher)

### Dates
- duration between two dates: start, end, return unit ("s")
- day of the week of a date: date
- extraction (day, week, month, year, hours, ...): date
- add a period to a date: date, period (days, months, years, ...)

### Lists
- maximum quantity in a list: list of quantities
- minimum quantity in a list: list of quantities
- merge several lists: list of lists
- select elements (pick): list of values, start index, end index

### Values
- random number: minimum quantity (0), maximum quantity (1), number of decimals (2)
- pi: number of decimals (15)
- current date: return variation ("timestamp")
