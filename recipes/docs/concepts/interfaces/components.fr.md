---
icon: components
tags: [ alpha ]
---
# Composants

Les Composants permettent de créer une interface graphique décomposée en éléments réutilisables et imbriquables. L'ensemble du contenu d'un Composant est défini par des Blocs qui sont découpées en deux catégories : les Blocs statiques et les Blocs dynamiques.

Chaque Bloc possède des Options qui permettent de définir son comportement. Il est par exemple possible de définir le déclenchement d'une [Fonction](/fr/concepts/automations/functions/){.cico .cico-functions} lors d'un appui sur un Bloc, ou encore en tant que condition pour déterminer si un Bloc doit être affiché ou non.

Les Blocs peuvent aussi être liés à des [Styles](/fr/concepts/interfaces/styles/){.cico .cico-styles} qui permettent de définir leur apparence. L'application d'un Style peut être conditionné par une condition exécutée par une [Fonction](/fr/concepts/automations/functions/){.cico .cico-functions}.

### Blocs statiques
- Zone : permet le regroupement Blocs
- Texte : affiche du texte potentiellement dans plusieurs langues
- Image : affiche une image contenue dans un [Volume](/fr/concepts/storage/volumes/){.cico .cico-volumes} ou depuis une URL externe
- Lien : lien vers une [Page](/fr/concepts/interfaces/pages/){.cico .cico-pages} ou une URL externe
- Bouton : déclenche une action lors d'un appui
- Saisie : entrée d'une valeur par l'utilisateur
- Case à cocher : permet de définir un état binaire
- Vidéo : affiche une vidéo contenue dans un [Volume](/fr/concepts/storage/volumes/){.cico .cico-volumes} ou depuis une URL externe
- Diviseur : sépare dynamiquement deux Blocs horizontalement ou verticalement
- Déplaçable : permet de déplacer ce Bloc dans un Emplacement
- Emplacement : regroupe des Blocs Déplaçables

### Blocs dynamiques
- Composant : permet d'appeler une autre Recette Composant
- Référence : permet d'afficher la valeur d'une [Option](/fr/concepts/recipes/options/) d'un autre Bloc
- Itération : permet d'afficher autant de Blocs qu'il y a d'éléments retournés par une [Fonction](/fr/concepts/automations/functions/){.cico .cico-functions}
