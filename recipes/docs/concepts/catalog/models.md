---
icon: models
tags: [ alpha ]
---
# Models

A Model is a group of [Recipes](/concepts/catalog/recipes/){.cico .cico-recipes} with a Versioning system.

The elements defined in a Model are:

- an icon and a description
- the [Users](/concepts/owners/users/){.cico .cico-users} authors
- the [Format](/concepts/recipes/formats/) of its [Options](/concepts/recipes/options/)
- the default configuration of the hosts used for the [Connectors](/concepts/endpoints/connectors/){.cico .cico-connectors}
- the stages possible for a Task of the [Community](/cloud/#community)

Each Version is isolated from the others but it is possible to link [Recipes](/concepts/catalog/recipes/){.cico .cico-recipes} between them.

[Recipes](/concepts/catalog/recipes/){.cico .cico-recipes} that can be found in a Model are :

- for [Interfaces](/concepts/interfaces/){.cico .cico-interfaces} :
    - [Components](/concepts/interfaces/components/){.cico .cico-components}
    - [Styles](/concepts/interfaces/styles/){.cico .cico-styles}
    - [Pages](/concepts/interfaces/pages/){.cico .cico-pages}
    - [Sessions](/concepts/interfaces/sessions/){.cico .cico-sessions}

- for [Storage](/concepts/storage/){.cico .cico-storage} :
    - [Collections](/concepts/storage/collections/){.cico .cico-collections}
    - [Metrics](/concepts/storage/metrics/){.cico .cico-metrics}
    - [Logs](/concepts/storage/logs/){.cico .cico-logs}
    - [Values](/concepts/storage/values/){.cico .cico-values}

- for [Validations](/concepts/validations/){.cico .cico-validations} :
    - [Types](/concepts/validations/types/){.cico .cico-types}
    - [Anomalies](/concepts/validations/issues/){.cico .cico-issues}
    - [Syntaxes](/concepts/validations/syntaxes/){.cico .cico-syntaxes}

- for [Automation](/concepts/automations/){.cico .cico-automations}
    - [Functions](/concepts/automations/functions/){.cico .cico-functions}
    - [Triggers](/concepts/automations/triggers/){.cico .cico-triggers}
    - [Migrations](/concepts/automations/migrations/){.cico .cico-migrations}
    - [Deployments](/concepts/automations/deployments/){.cico .cico-deployments}

- for [Authentication](/concepts/auth/){.cico .cico-auth} :
    - [Permissions](/concepts/auth/permissions/){.cico .cico-permissions}

- for [Endpoints](/concepts/endpoints/){.cico .cico-endpoints} :
    - [Connectors](/concepts/endpoints/connectors/){.cico .cico-connectors}

- for [Editor](/concepts/editor/){.cico .cico-editor} :
    - [Sections](/concepts/editor/sections/){.cico .cico-sections}
    - [Visualizations](/concepts/editor/visualizations/){.cico .cico-visualizations}
    - [Forms](/concepts/editor/forms/){.cico .cico-forms}
    - [Maps](/concepts/editor/maps/){.cico .cico-maps}
