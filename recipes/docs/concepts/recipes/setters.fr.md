---
tags: [""]
---

# Définisseurs

Les Définisseurs permettent de déterminer et transformer des données.

Dans les différents Définisseurs ci-dessous, les valeurs utilisées sont :

- soit des données brutes
- soit issues d'un [Récupérateur](/fr/concepts/recipes/getters)

Dans un texte, il est possible d'utiliser un Définisseur pour rendre un texte dynamique.

## Opérations

### Arithmétique
- addition : liste de quantités
- soustraction : liste de quantités
- multiplication : liste de quantités
- division : liste de quantités
- puissance : quantité
- racine carrée : quantité
- exponentielle : quantité
- logarithme : quantité
- modulo : deux quantités

### Logique
- existe : valeur
- et : liste de booléens
- ou : liste de booléens
- non : booléen

### Comparaisons
- moins que (<) : deux quantités
- au plus (<=) : deux quantités
- au moins (>=) : deux quantités
- plus que (>) : deux quantités
- égal à : deux quantités
- différent de : deux quantités
- différent de tous : une valeur, une liste de valeurs
- différent d'un seul : une valeur, une liste de valeurs
- égal à tous : une valeur, une liste de valeurs
- égal à un seul : une valeur, une liste de valeurs



## Formats

### Commun
- taille en caractères d'une valeur ou d'éléments d'une liste (length) : valeur ou liste
- comparaison des Formats : valeur à vérifier, [Format](/fr/concepts/recipes/formats) à détecter

### Textes
- concaténer : valeurs, texte séparateur (" ")
- découper : valeur, texte séparateur
- chercher une chaîne de caractères (match) : texte dans lequel chercher, expression régulière
- remplacer : texte dans lequel chercher, texte à remplacer, texte de remplacement
- traduire : valeur, langue, nombre, article à utiliser
- minuscule : texte
- majuscule : texte
- majuscules des premières lettres de chaque mot  (titleCase) : texte
- tronquer les espaces de début et de fin (trim) : texte, caractères ([""]), droite (true), gauche (true)
- abréger : texte, maximum, texte de fin ("...")
- transformation d'un texte avec une Syntaxe : texte, [Syntaxe](/fr/concepts/validations/syntaxes){.cico .cico-syntaxes} et ses [Options](/fr/concepts/recipes/options)
- sélectionner une partie du texte : texte à découper, début, fin

### Quantités
- moyenne : liste de quantités
- arrondi : quantité, nombre de décimales (2), type d'arrondi (au plus proche, inférieur ou supérieur)

### Dates
- durée entre deux dates : début, fin, unité de retour ("s")
- jour de la semaine d'une date : date
- extraction (jour, semaine, mois, année, heures, ...) : date
- ajouter une période à une date : date, période (jours, mois, années, ...)

### Listes
- quantité maximale dans une liste : liste de quantités
- quantité minimale dans une liste : liste de quantités
- fusionner plusieurs listes : liste de listes
- sélectionner des éléments : liste de valeurs, index de début, index de fin

### Valeurs
- nombre aléatoire : quantité minimale (0), quantité maximale (1), nombre de décimales (2)
- pi : nombre de décimales (15)
- date actuelle : variation de retour ("timestamp")
