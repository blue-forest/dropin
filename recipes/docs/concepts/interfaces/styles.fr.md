---
icon: styles
tags: [ alpha ]
---
# Styles

Les Styles permettent d'appliquer des règles de disposition d'affichage des Blocs d'un [Composant](/fr/concepts/interfaces/components/){.cico .cico-components}.

Il existe plusieurs catégories de Styles :

- Dimensions : taille d'un Bloc
- Position : positionnement d'un Bloc
- Marges : espacements entre un Bloc et ses voisins
- Gouttières : espacements à l'intérieur d'un Bloc
- Fonds : style de fond d'un Bloc
- Bords : style des bords d'un Bloc
- Ombres : ombre portée d'un Bloc
- Texte : lié au Bloc Texte
- Zone : lié au Bloc Zone
- Saisie : lié au Bloc Saisie

Un Style est décomposé en plusieurs Classes qui portent toutes un nom unique. Chaque Classe peut être appliquée à n'importe quel Bloc.

Il est possible de créer des Classes conditionnelles afin d'appliquer des Styles en fonction de conditions basées sur :

- la plateforme (web, Android, iOS, linux, windows, macOS)
- les dimensions (longueur et largeur)
- des états (survol, focus, impression)

Enfin, il est possible de créer des animations afin d'appliquer des trames de styles sur un Bloc avec une durée et une fréquence définies.

## Schéma

[Aperçu](https://json-schema.app/view/%23?url=https%3A%2F%2Fraw.githubusercontent.com%2Fblue-forest%2Fdropin%2Fmain%2Fschemas%2Fstyles.json){:target="_blank" rel="noopener"}

[Source](https://github.com/blue-forest/dropin/blob/main/schemas/styles.json){:target="_blank" rel="noopener"}
