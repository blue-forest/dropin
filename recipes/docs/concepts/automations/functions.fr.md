---
icon: functions
tags: [ alpha ]
---
# Fonctions

Les Fonctions permettent de décrire un ensemble d'Actions à effectuer qui peuvent être exécutées lorsqu'un [Déclencheur](/fr/concepts/automations/triggers){.cico .cico-triggers} est activé ou par appel d'un [Composant](/fr/concepts/interfaces/components){.cico .cico-components} ou d'une autre [Fonction](/fr/concepts/automations/functions){.cico .cico-functions}.


## Actions

### Générales
- connecteur : communication avec une service externe (API) via un [Connecteur](/fr/concepts/endpoints/connectors){.cico .cico-connectors} et ses [Options](/fr/concepts/recipes/options)
- collection : effectue une opération de lecture ou d'écriture sur une [Collection](/fr/concepts/storage/collections){.cico .cico-collections}
- recherche : parcours une donnée pour extraire des informations
- fonction : appel d'une autre Fonction avec ses [Options](/fr/concepts/recipes/options)
- cryptographie : génère des clés de cryptage (bcrypt, JWT, UUID, RSA, ...)
- e-mail : permet un envoi manuel de messages
- page : permet de simuler la navigation sur une [Page](/fr/concepts/recipes/pages){.cico .cico-pages}
- transformation : permet de passer d'un texte à en un objet via une [Syntaxe](/fr/concepts/validations/syntaxes){.cico .cico-syntaxes}
- base de données : effectue des lectures ou des écritures avec une requête sur une [Base de données](/fr/concepts/endpoints/databases){.cico .cico-databases}

### Uniquement accessibles depuis un Composant
- session : permet de gérer une Session
- navigation : permet de manipuler l'historique des Pages
- imprimer : si disponible, déclenche une impression
- notification : permet de gérer les notifications de l'appareil
- vibrer : si disponible, faire vibrer l'appareil
- presse-papiers : permet de gérer la copie et le collage

### Uniquement dans les itérations et les intervalles
- continuer (continue)
- sortir (break)

## Manipulateurs
- assignation : valeur à définir, nouvelle valeur
- suppression : valeur à supprimer
- condition : déclencheur, alors [Actions](#actions), sinon [Actions](#actions)
- itération (iterate) : liste ou objet, [Actions](#actions)
- intervalle (range) : valeur de début, valeur de fin, décalages (quantités, jours, …), [Actions](#actions)
- tant que (while) : condition, [Actions](#actions)
- cas : valeur à tester, [Actions](#actions) pour chaque combinaison possible
- contrainte : condition, [Anomalie](/fr/concepts/validations/issues){.cico .cico-issues} à déclencher
