---
icon: components
tags: [ alpha ]
---
# Components

Components allow you to create a graphical interface made up of reusable and nestable elements. The entire content of a Component is defined by Blocks that are divided into two categories: Static Blocks and Dynamic Blocks.


## Blocks

Each Block has Options that allow you to define its behavior. For example, it is possible to define the triggering of a [Function](/concepts/automations/functions/){.cico .cico-functions} when a Block is pressed, or as a condition to determine whether a Block should be displayed or not.

Blocks can also be linked to [Styles](/concepts/interfaces/styles/){.cico .cico-styles} that allow you to define their appearance. Applying a Style can be conditioned by a condition executed by a [Function](/concepts/automations/functions/){.cico .cico-functions}.

### Static Blocks
- Zone: allows grouping Blocks
- Text: displays text potentially in multiple languages
- Image: displays an image contained in a [Volume](/concepts/storage/volumes/){.cico .cico-volumes} or from an external URL
- Link: link to a [Page](/concepts/interfaces/pages/){.cico .cico-pages} or an external URL
- Button: triggers an action when pressed
- Input: user input
- Checkbox: allows you to define a binary state
- Video: displays a video contained in a [Volume](/concepts/storage/volumes/){.cico .cico-volumes} or from an external URL
- Divider: dynamically separates two Blocks horizontally or vertically
- Draggable: allows you to move this Block to a Droppable Block
- Droppable: groups Draggable Blocks

### Dynamic Blocks

- Component: allows you to call another Component Recipe
- Reference: allows you to display the value of an [Option](/concepts/recipes/options/) of another Block
- Iteration: allows you to display as many Blocks as there are elements returned by a [Function](/concepts/automations/functions/){.cico .cico-functions}


## Schema

[Overview](https://json-schema.app/view/%23?url=https%3A%2F%2Fraw.githubusercontent.com%2Fblue-forest%2Fdropin%2Fmain%2Fschemas%2Fcomponent.json){:target="_blank" rel="noopener"}

[Source](https://github.com/blue-forest/dropin/blob/main/schemas/component.json){:target="_blank" rel="noopener"}
