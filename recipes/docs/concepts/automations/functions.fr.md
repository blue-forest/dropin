---
icon: functions
tags: [ alpha ]
---
# Fonctions

Les Fonctions permettent de décrire un comportement en Étapes lorsqu'un [Déclencheur](/fr/concepts/automations/triggers){.cico .cico-triggers} est activé ou lorsqu'on les exécute manuellement.

## Étapes
Une Étape est une série d'actions exécutée par des Fonctions. Les Étapes sont liées par un système de dépendances permettant d'identifier leur ordre d'exécution. Les actions d'une Étape sont exécutées successivement.

## Actions
- requête : envoi d'une requête à un serveur (HTTP, WebSocket, ...)
- recherche : parcours une donnée avec des Manipulateurs pour extraire des informations
- transformation : permet de passer d'un texte à en un objet via une Syntaxe
- cryptographie : génère des clés de cryptage (bcrypt, JWT, UUID, RSA, ...)
- collection : effectue une opération de lecture ou d'écriture sur une [Collection](/fr/concepts/storage/collections){.cico .cico-collections}
- fonction : appel d'une autre Fonction avec ses [Options](/fr/concepts/recipes/options)
- connecteur : communication avec une service externe (API) via un [Connecteur](/fr/concepts/endpoints/connectors){.cico .cico-connectors} et ses [Options](/fr/concepts/recipes/options)
- base de données : effectue des lectures ou des écritures avec une requête
- email : permet un envoi manuel de messages
- page : permet de simuler la navigation sur une [Page](/fr/concepts/recipes/pages){.cico .cico-pages}
