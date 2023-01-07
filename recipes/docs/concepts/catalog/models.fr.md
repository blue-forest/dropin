---
icon: models
tags: [ alpha ]
---
# Modèles

Un Modèle est un regroupement de [Recettes](/fr/concepts/catalog/recipes/){.cico .cico-recipes} avec un système de Versions.

Les éléments renseignés dans un Modèle sont :

- une icône et une description
- les [Utilisateurs](/fr/concepts/owners/users/){.cico .cico-users} auteurs
- le [Format](/fr/concepts/recipes/formats/) de ses [Options](/fr/concepts/recipes/options/)
- la configuration des hôtes utilisés pour les [Connecteurs](/fr/concepts/endpoints/connectors/){.cico .cico-connectors}
- les stades possibles pour une Tâche de la [Communauté](/fr/cloud/#communaute)

Chaque Version est isolée des autres mais il est possible de faire des liens entre les [Recettes](/fr/concepts/catalog/recipes/){.cico .cico-recipes}.

Les [Recettes](/fr/concepts/catalog/recipes/){.cico .cico-recipes} pouvant se trouver dans un Modèle sont :

- pour les [Interfaces](/fr/concepts/interfaces/){.cico .cico-interfaces} :
    - les [Composants](/fr/concepts/interfaces/components/){.cico .cico-components}
    - les [Styles](/fr/concepts/interfaces/styles/){.cico .cico-styles}
    - les [Pages](/fr/concepts/interfaces/pages/){.cico .cico-pages}
    - les [Sessions](/fr/concepts/interfaces/sessions/){.cico .cico-sessions}

- pour le [Stockage](/fr/concepts/storage/){.cico .cico-storage} :
    - les [Collections](/fr/concepts/storage/collections/){.cico .cico-collections}
    - les [Métriques](/fr/concepts/storage/metrics/){.cico .cico-metrics}
    - les [Journaux](/fr/concepts/storage/logs/){.cico .cico-logs}
    - les [Valeurs](/fr/concepts/storage/values/){.cico .cico-values}

- pour les [Validations](/fr/concepts/validations/){.cico .cico-validations} :
    - les [Types](/fr/concepts/validations/types/){.cico .cico-types}
    - les [Anomalies](/fr/concepts/validations/issues/){.cico .cico-issues}
    - les [Syntaxes](/fr/concepts/validations/syntaxes/){.cico .cico-syntaxes}

- pour l'[Automatisation](/fr/concepts/automations/){.cico .cico-automations}
    - les [Fonctions](/fr/concepts/automations/functions/){.cico .cico-functions}
    - les [Déclencheurs](/fr/concepts/automations/triggers/){.cico .cico-triggers}
    - les [Migrations](/fr/concepts/automations/migrations/){.cico .cico-migrations}
    - les [Déploiements](/fr/concepts/automations/deployments/){.cico .cico-deployments}

- pour l'[Authentification](/fr/concepts/auth/){.cico .cico-auth} :
    - les [Permissions](/fr/concepts/auth/permissions/){.cico .cico-permissions}

- pour les [Terminaisons](/fr/concepts/endpoints/){.cico .cico-endpoints} :
    - les [Connecteurs](/fr/concepts/endpoints/connectors/){.cico .cico-connectors}

- pour l'[Éditeur](/fr/concepts/editor/){.cico .cico-editor} :
    - les [Sections](/fr/concepts/editor/sections/){.cico .cico-sections}
    - les [Visualisations](/fr/concepts/editor/visualizations/){.cico .cico-visualizations}
    - les [Formulaires](/fr/concepts/editor/forms/){.cico .cico-forms}
    - les [Cartes](/fr/concepts/editor/maps/){.cico .cico-maps}


## Aperçu de Studio (dropin.cloud)

La version alpha de drop'in Studio sera basée sur l'éditeur [Visual Studio Code](https://code.visualstudio.com/){:target="_blank" rel="noopener"} avec lequel une extension a été développée. De cette manière, les fonctionnalités initiales comme le parcours de la structure des fichiers, la navigation entre les fichiers et le support du versionnement Git sont déjà disponibles.

Voici un exemple d'une Recette de Modèle dans Studio :

![](/assets/studio/model.png)


## Schéma

[Aperçu](https://json-schema.app/view/%23?url=https%3A%2F%2Fraw.githubusercontent.com%2Fblue-forest%2Fdropin%2Fmain%2Fschemas%2Fmodel.json){:target="_blank" rel="noopener"}

[Source](https://github.com/blue-forest/dropin/blob/main/schemas/model.json){:target="_blank" rel="noopener"}
