---
icon: projects
tags: [ cloud, alpha ]
---
# Projects

A Project is the use of a [Model](/concepts/catalog/models/){.cico .cico-models} : it applies the content of the [Recipes](/concepts/catalog/recipes/){.cico .cico-recipes} of a Model to work. It is therefore possible to have a single Model and several Projects with specific Options for each environment.

The elements specified in a Project are:

- a [Model](/concepts/catalog/models/){.cico .cico-models}, its Version and its [Options](/concepts/recipes/options/)
- the configuration of the hosts used for [Connectors](/concepts/endpoints/connectors/){.cico .cico-connectors}
- platform-specific informations (web, Android, iOS, Linux, Windows, macOS)

When updating a [Model](/concepts/catalog/models/){.cico .cico-models}, it will be possible to manually or automatically migrate the Project to a new version.
