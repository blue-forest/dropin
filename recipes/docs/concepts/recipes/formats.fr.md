# Formats

Les Formats sont des définitions de structure de données et peuvent être composés de :

- Types natifs uniques et composés d'autres Formats
- Types personnalisés créés par les Propriétaires contenant d'autres Formats

Les Formats sont par exemple utilisés :

- dans les [Options](/fr/concepts/recipes/options) des Recettes
- dans les Variations des [Types](/fr/concepts/validations/types){.cico .cico-types}
- dans les Champs des [Collections](/fr/concepts/storage/collections){.cico .cico-collections}
- dans le [Définisseur](/fr/concepts/recipes/setters) "comparaison des Formats"

Un Format de type natif objet peut contenir l'information de tous les champs obligatoires ainsi que les valeurs par défaut à utiliser.

En utilisant un Format, il est possible de poser des Contraintes sur les données manipulées en donnant une condition de validité et une [Anomalie](/fr/concepts/validations/issues){.cico .cico-issues}. Si la condition n'est pas remplie, l'Anomalie est déclenchée et renvoie des erreurs personnalisées.

Les Formats peuvent aussi contenir des Marqueurs afin d'appliquer des sélections dynamiques avec par exemple la Étape "recherche" dans une [Fonction](/fr/concepts/automations/functions){.cico .cico-functions}.

## Types natifs

### Uniques

#### Audio
- Options : [Volume](/fr/concepts/storage/volumes){.cico .cico-volumes} à utiliser, tableaux d'extensions acceptées, taille min et max, durée min et max
- Format : Item de Fichier

#### Binaire
- Options : taille min et max
- Format : binaire

#### Booléen
- Affichage : éditer la date de réalisation
- Variations :
    - basique : booléen
    - tâche : objet avec un booléen et deux date

#### Choix
- Options : choix min et max, possibilités avec leur description
- Format : liste de choix

#### Couleur
- Variations :
    - Hexadécimal : objet avec texte et opacité
    - RGBA : objet avec des textes (rouge, vert, bleu et opacité)
    - HSLA : objet avec des textes

#### Date
- Options : min et max
- Format : timestamp
- Affichage : unités (secondes, minutes, heures, jours, mois, années)

#### Durée
- Options : min et max
- Format : durée
- Affichage : unités (secondes, minutes, heures, jours, mois, années)

#### E-mail
- Options : nom de domaines à utiliser, Expression Régulière sur le nom
- Format : texte

#### Fichier
- Options : [Volume](/fr/concepts/storage/volumes){.cico .cico-volumes} à utiliser, tableaux d'extensions acceptées, taille min et max
- Format : Item de Fichier

#### Image
- Options : [Volume](/fr/concepts/storage/volumes){.cico .cico-volumes} à utiliser, tableaux d'extensions acceptées, taille min et max, dimensions min et max
- Format : Item de Fichier

#### Item
- Options : [Collection](/fr/concepts/storage/collections){.cico .cico-collections} avec ses [Options](/fr/concepts/recipes/options)
- Format : Item d'Item

#### Localisation
- Options : pays inclus
- Format : objet
- Affichage : carte, champs (nom, nombre, rue, région, ville, pays, code postal et coordonnées GPS)

#### Mesure
- Options : décimales, min et max, table de conversions
- Affichage : unité
- Format : objet avec quantité et unité

#### Période
- Options : dates min et max
- Format : objet avec deux dates

#### Téléphone
- Options : liste de préfixes des pays acceptés
- Affichage : regroupement de nombres, séparateur
- Format : objet avec préfixe et texte avec le numéro

#### Prix
- Options : min et max, liste des devises acceptées
- Affichage : devise
- Format : mesure avec devise

#### Quantité
- Options : décimales, min et max
- Format : quantité

#### Récurrence
- Format : période et intervalle (secondes, minutes, heures, jours, mois, années)

#### Texte
- Options : taille min, taille max, Expression Régulière
- Variations :
    - texte
    - par nombre : nom pour un, nom pour plusieurs, article si applicable
    - par langue : pour chaque langue, un texte ou par nombre

#### URL
- Format : texte

#### Vidéo
- Options : [Volume](/fr/concepts/storage/volumes){.cico .cico-volumes} à utiliser, tableaux d'extensions acceptées, taille min et max, dimensions min et max
- Format : Item de Fichier


### Composés

#### Index
Clés dynamiques et Formats communs

- Options : liste de clés possibles


#### Liste
Ensemble ordonné d'un même Format

- Options : taille min et max


#### Objet
Clés fixes et Formats individuels

- Options : objet avec champs requis et valeurs par défaut
