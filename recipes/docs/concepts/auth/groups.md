---
icon: groups
tags: [ cloud, alpha ]
---
# Groups

Groups describe the access of a [User](/concepts/owners/users){.cico .cico-users} on the resources of an [Owner](/concepts/owners){.cico .cico-owners} such as:

- the access to an [Organization](/concepts/owners/organizations){.cico .cico-organizations} (administration, billing, access, [Tokens](/concepts/auth/tokens){.cico .cico-tokens}, [Realms](/concepts/auth/realms){.cico .cico-realms}, Payments, ...)
- [Models](/concepts/catalog/models){.cico .cico-models} (creation, modification, deletion, ...)
- [Projects](/concepts/catalog/projects){.cico .cico-projects} by specifying for each the [Permissions](/concepts/auth/permissions){.cico .cico-permissions} to use

Once created, a Group can be assigned to [Users](/concepts/owners/users){.cico .cico-users}. It is even possible to assign additional conditions to create sub-groups.
