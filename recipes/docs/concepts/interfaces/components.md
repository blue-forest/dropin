---
icon: components
tags: [ alpha ]
---
# Components

## Description

Components allow you to create a graphical interface made up of reusable and nestable elements.

The entire content of a Component is defined by Blocks that are divided into two categories: Static Blocks and Dynamic Blocks.

Each Block has Options that allow you to define its behavior. For example, it is possible to define the triggering of a [Function](/concepts/automations/functions/){.cico .cico-functions} when a Block is pressed, or as a condition to determine whether a Block should be displayed or not.

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
