---
icon: organizations
tags: [ beta ]
---
# Organizations

Organizations are created and owned by [Users](/concepts/owners/users){.cico .cico-users}.

Unlike [Users](/concepts/owners/users){.cico .cico-users}, Organizations do not have the ability to authenticate on drop'in interfaces. As a result, all exchanges via API must use manually generated [Tokens](/concepts/auth/tokens){.cico .cico-tokens}.

Another difference with [Users](/concepts/owners/users){.cico .cico-users} is that an Organization can create [Realms](/concepts/auth/realms){.cico .cico-realms} to allow the registration of Users, and Payments to make bank transactions.

Payments allow you to use the credit card of a [User](/concepts/owners/users){.cico .cico-users} via [Components](/concepts/interfaces/components){.cico .cico-components} in the context of a [Project](/concepts/catalog/projects){.cico .cico-projects}.
