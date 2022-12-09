---
icon: functions
tags: [ alpha ]
---
# Functions

Functions are used to describe behavior in Steps when a [Trigger](/concepts/automations/triggers){.cico .cico-triggers} is activated or when executed manually.

## Steps
A Step is a series of actions performed by Functions. The Steps are linked by a system of dependencies allowing to identify their execution order. The actions of a Step are executed successively.

## Actions
- request: send a request to a server (HTTP, WS, ...)
- lookup: browse a data with Manipulators to extract information
- parse: allows to pass from a text to an object via a Syntax
- cryptography: generates encryption keys (bcrypt, JWT, UUID, RSA, ...)
- collection: performs a read or write operation on a [Collection](/concepts/storage/collections){.cico .cico-collections}
- function: call another Function with its [Options](/concepts/recipes/options)
- connector: communication with an external service (API) via a [Connector](/concepts/endpoints/connectors){.cico .cico-connectors} and its [Options](/concepts/recipes/options)
- database: performs reads or writes with a query
- email: allows manual sending of messages
- page: allows to simulate the navigation on a [Page](/concepts/recipes/pages){.cico .cico-pages}
