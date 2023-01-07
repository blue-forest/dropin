---
icon: styles
tags: [ alpha ]
---
# Styles

Styles allow you to apply display rules for the Blocks of a [Component](/concepts/interfaces/components/){.cico .cico-components}.

There are several categories of Styles:

- Dimensions: size of a Block
- Position: positioning of a Block
- Margins: spacing between a Block and its neighbors
- Gutters: spacing inside a Block
- Background: background style of a Block
- Borders: border style of a Block
- Shadows: shadow of a Block
- Text: related to the Text Block
- Zone: related to the Zone Block
- Input: related to the Input Block

A Style is decomposed into several Classes that all have a unique name. Each Class can be applied to any Block.

It is possible to create conditional Classes in order to apply Styles depending on conditions based on:
- the platform (web, Android, iOS, linux, windows, macOS)
- the dimensions (height and width)
- the state (hover, focus, print)

Finally, it is possible to create animations in order to use styles frames on a Block with a defined duration and frequency.

## Schema

[Overview](https://json-schema.app/view/%23?url=https%3A%2F%2Fraw.githubusercontent.com%2Fblue-forest%2Fdropin%2Fmain%2Fschemas%2Fstyles.json){:target="_blank" rel="noopener"}

[Source](https://github.com/blue-forest/dropin/blob/main/schemas/styles.json){:target="_blank" rel="noopener"}
