# Formats

Formats are data structure definitions and can be composed of:

- Unique native types composed of other Formats
- Custom types created by Owners containing other Formats

Formats are for example used:

- in [Options](/concepts/recipes/options/) of Recipes
- in Variations of [Types](/concepts/validations/types/){.cico .cico-types}
- in Fields of [Collections](/concepts/storage/collections/){.cico .cico-collections}
- in [Setter](/concepts/recipes/setters/) "Format comparison"

A Format of native object type can contain the information of all required fields as well as default values to be used.

When using a Format, it is possible to place Constraints on the manipulated data by giving a validity condition and an [Issue](/concepts/validations/issues/){.cico .cico-issues}. If the condition is not met, the Issue is triggered and returns custom errors.

Formats can also contain Labels to apply dynamic selections with for example the "search" step in a [Function](/concepts/automations/functions/){.cico .cico-functions}.

## Native types

### Unique

#### Audio
- Options: [Volume](/concepts/storage/volumes/){.cico .cico-volumes} to use, list of accepted extension, min and max size, min and max duration
- Format: File Item

#### Binary
- Options: min and max size
- Format: binary

#### Boolean
- Display: edit date
- Variations:
    - basic: boolean
    - task: object with one boolean and two dates

#### Choice
- Options: min and max choices, possibilities with their description
- Format: list of choices

#### Color
- Variations:
    - Hexadecimal: object with text and opacity
    - RGBA: object with texts (red, green, blue and opacity)
    - HSLA: object with texts

#### Date
- Options: min and max
- Format: timestamp
- Display: units (seconds, minutes, hours, days, months, years)

#### Duration
- Options: min and max
- Format: duration
- Display: units (seconds, minutes, hours, days, months, years)

#### E-mail
- Options: domain name to use, Regular expression on name
- Format: text

#### File
- Options: [Volume](/concepts/storage/volumes/){.cico .cico-volumes} to use, accepted extension tables, min and max size
- Format: File Item

#### Image
- Options: [Volume](/concepts/storage/volumes/){.cico .cico-volumes} to use, supported extension tables, min and max size, min and max dimensions
- Format: File Item

#### Item
- Options: [Collection](/concepts/storage/collections/){.cico .cico-collections} with its [Options](/concepts/recipes/options/)
- Format: Item of Item

#### Location
- Options: country included
- Format: object
- Display: map, fields (name, number, street, region, city, country, postal code and GPS coordinates)

#### Measure
- Options: decimals, min and max, conversion table
- Display: unit
- Format: object with quantity and unit

#### Period
- Options: min and max dates
- Format: object with two dates

#### Telephone
- Options: list of accepted country prefixes
- Display: grouping of numbers, separator
- Format: object with prefix and text with number

#### Price
- Options: min and max, list of accepted currencies
- Display: currency
- Format: measure with currency

#### Quantity
- Options: decimal, min and max
- Format: quantity

#### Recurrence
- Format: period and interval (seconds, minutes, hours, days, months, years)

#### Text
- Options: min size, max size, Regular expression
- Variations:
    - text
    - by number: name for one, name for many, article if applicable
    - by language: for each language, a text or by number

#### URL
- Format: text

#### Video
- Options: [Volume](/concepts/storage/volumes/){.cico .cico-volumes} to use, accepted extension tables, min and max size, min and max dimensions
- Format: File Item


### Compounds

#### Index
Dynamic keys and common formats

- Options: list of possible keys


#### List
Ordered set of the same Format

- Options: min and max size


#### Object
Fixed keys and individual Formats

- Options: object with required fields and default values
