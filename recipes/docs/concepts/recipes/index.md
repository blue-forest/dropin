---
icon: recipes
hide: [ toc ]
tags: [root]
---
# Recipes

A Recipe is the description of an entity, a state or a behavior.

drop'in allows the writing of 32 different types of Recipes each having a specific role.

There are two types of Recipes:

- those directly linked to an [Owner](/concepts/owners/){.cico .cico-owners} such as [Groups](/concepts/auth/groups/){.cico .cico-groups}, [Volumes](/concepts/storage/volumes/){.cico .cico-volumes}, or [Domains](/concepts/endpoints/domains/){.cico .cico-domains}

- those found in a [Model](/concepts/catalog/models/){.cico .cico-models} (such as [Components](/concepts/interfaces/components/){.cico .cico-components}, [Functions](/concepts/automations/functions/){.cico .cico-functions} or [Collections](/concepts/storage/collections/){.cico .cico-collections}) that can be used to create [Projects](/concepts/catalog/projects/){.cico .cico-projects} or be shared with other users via the [Catalog](/concepts/catalog/).

## Categories
- [Catalog](/concepts/catalog/){.cico .cico-catalog}: management of [Models](/concepts/catalog/models/){.cico .cico-models} and [Projects](/concepts/catalog/projects/){.cico .cico-projects}
- [Interfaces](/concepts/interfaces/){.cico .cico-interfaces}: creating graphical experiences
- [Storage](/concepts/storage/){.cico .cico-storage}: persistence of formatted data
- [Automations](/concepts/automations/){.cico .cico-automations}: automated actions
- [Validations](/concepts/validations/){.cico .cico-validations}: verification of data conformity
- [Auth](/concepts/auth/){.cico .cico-auth}: management of [Users](/concepts/owners/users/){.cico .cico-users} accesses
- [Endpoints](/concepts/endpoints/){.cico .cico-endpoints}: connection to external services
- [Editor](/concepts/editor/){.cico .cico-editor}: [Projects](/concepts/catalog/projects/){.cico .cico-projects} administration

## Header data
- name
- description
- terms, to define the name of the Recipe to qualify a single entity and for several entities
- labels, words allowing to create filters to select a set of Recipes
