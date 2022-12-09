---
icon: organizations
tags: [ beta ]
---
# Organisations

Les Organisations sont créées et possédées par des [Utilisateurs](/fr/concepts/owners/users){.cico .cico-users}.

Contrairement aux [Utilisateurs](/fr/concepts/owners/users){.cico .cico-users}, les Organisations ne possèdent pas la possibilité de s'authentifier sur les interfaces drop'in. De ce fait, l'ensemble des échanges effectués par API doivent utiliser des [Jetons](/fr/concepts/auth/tokens){.cico .cico-tokens} manuellement générés.

Une autre différence avec les [Utilisateurs](/fr/concepts/owners/users){.cico .cico-users} est qu'une Organisation peut créer des [Royaumes](/fr/concepts/auth/realms){.cico .cico-realms} pour permettre l'inscription d'Utilisateurs, et des Paiements pour effectuer des transactions bancaires.

Les Paiements permettent d'utiliser via des [Composants](/fr/concepts/interfaces/components){.cico .cico-components} la carte bancaire d'un [Utilisateur](/fr/concepts/owners/users){.cico .cico-users} dans le carde d'un [Projet](/fr/concepts/catalog/projects){.cico .cico-projects}.
