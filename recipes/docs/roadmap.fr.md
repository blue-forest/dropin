---
hide: [ "toc" ]
---
# Feuille de route

Le développement de drop'in a été divisé en saisons et épisodes : nous sommes passés de version en version depuis 2016 et nous sommes actuellement à l'épisode 2 de la saison 6.

Au cours de parcours, nous avons établis plusieurs stacks techniques avec une multitude d'expérimentations qui nous ont permis de progressivement établir un ensemble de fonctionnalités et de définir des Concepts clés.

Durant les saisons précédentes, drop'in a été utilisé dans le cadre des services de [Blue Forest](https://blueforest.cc){:target="_blank" rel="noopener"} pour créer des applications multi-plateformes pour ses clients. Aujourd'hui, nous avons finalisé les spécifications de drop'in pour le rendre disponible publiquement et permettant prochainement à n'importe qui de construire ses propres applications en totale autonomie.


## alpha

Cette première phase sera consacrée à la mise en place des Concepts fondamentaux de drop'in avec tout ce qui est nécessaire à la création d'applications comme les [Interfaces](/fr/concepts/interfaces/){.cico .cico-interfaces} ou les [Automatisations](/fr/concepts/automations/){.cico .cico-automations} par exemple. Dans cette phase, nous allons également proposer une première version de tous les Concepts liés à la constitution d'un [Modèle](/fr/concepts/catalog/models/){.cico .cico-models}, ils sont marqués avec la mention "alpha" dans la documentation.

Notre stratégie sera d'utiliser [Visual Studio Code](https://code.visualstudio.com/){:target="_blank" rel="noopener"} comme base et y intégrer notre extension pour accéder à toutes les fonctionnalités de drop'in. Les projets seront hébergés sur le service de votre choix (GitHub, GitLab, Gitea, BitBucket, ...) et l'utilisation de [dropin.cloud](/fr/cloud/) sera possible par le biais de l'extension qui vous guidera tout au long de la création de votre application. Cette documentation sera également disponible au travers de l'extension.

Nous travaillons activement sur cette phase mais nous n'avons pas encore de date de sortie. Nous avons cependant déjà mis à disposition [ce formulaire d'inscription](https://docs.google.com/forms/d/e/1FAIpQLSejGbv2SCbZ7xZwpdGSDTqEi3e7eg2FQNmsoZeJWaNxv27Nkw/viewform){:target="_blank" rel="noopener"} pour être informé de la sortie de la version alpha et pour pouvoir tester l'extension dès qu'elle sera disponible.

Une fois la phase lancée, l'accès à [dropin.cloud](/fr/cloud/) sera totalement gratuite et limitée à un nombre spécifique de ressources. L'accès ne sera uniquement possible que par invitation, un formulaire d'inscription sera mis à disposition pour permettre à n'importe qui de demander un accès sur notre page d'accueil finale.

Ce qui est ou sera rendu open source :

- [cette documentation](https://github.com/blue-forest/dropin/tree/main/recipes){:target="_blank" rel="noopener"}
- [les schémas de notre langage](https://github.com/blue-forest/dropin/tree/main/schemas){:target="_blank" rel="noopener"}
- le code source de l'extension Visual Studio Code


## beta

Cette phase sera consacrée à la stabilisation de l'ensemble des fonctionnalités existantes et à l'ajout de nouvelles fonctionnalités.

Parmi les fonctionnalités à venir qui offriront un nouveau niveau de possibilités, nous avons :

- les [Migrations](/fr/concepts/automations/migrations/){.cico .cico-migrations} qui permettront de faciliter le changement de structure de données entre les différentes versions de votre application
- les [Syntaxes](/fr/concepts/validations/syntaxes/){.cico .cico-syntaxes} qui permettront de décomposer n'importe quel texte
- les [Connecteurs](/fr/concepts/endpoints/connectors/){.cico .cico-connectors} qui permettront de connecter votre application à n'importe quel service tiers par le biais d'API
- ... et tous les autres Concepts possédant la mention "beta" dans la documentation

L'utilisation de Visual Studio Code sera toujours possible mais nous allons également proposer un éditeur web pour ceux qui ne souhaitent rien installer sur leur machine. Nous allons également proposer un éditeur mobile (web, Android et iOS) pour ceux qui souhaitent créer des applications sur leur téléphone. Enfin, une version bureau (Linux, Windows et MacOS) pourra aussi être proposée pour ne pas avoir à passer par un navigateur.

L'accès à [dropin.cloud](/fr/cloud/) sera possible sans invitation et une conservation d'une version gratuite avec un nombre limité de ressources sera toujours disponible. Les utilisateurs pourront également choisir de passer à un abonnement payant pour bénéficier d'un nombre illimité de ressources ainsi que l'accès à un support technique avancé. Nous communiquerons plus en détail sur les tarifs et les fonctionnalités de l'abonnement payant avant la sortie de la version beta.

En plus de ce qui est open source la version alpha, [le code source du compilateur de notre language](https://github.com/blue-forest/dropin/tree/main/compiler){:target="_blank" rel="noopener"} sera aussi disponible publiquement. Les seuls services qui ne seront pas open source seront ceux intégrés à [dropin.cloud](/fr/cloud/) et qui nécessitant la création d'un compte pour pouvoir les utiliser.


## stable

Cette dernière phase permettra de stabiliser l'ensemble des fonctionnalités en verrouillant les spécifications de drop'in tout en s'assurant que l'ensemble des fonctionnalités fonctionnelles et entièrement testées.

Par ailleurs, de nouvelles fonctionnalités seront ajoutées pour permettre de créer des applications plus complexes. Parmi celles-ci, nous avons :

- les [Conteneurs](/fr/concepts/storage/containers/){.cico .cico-containers} qui permettront de décrire le fonctionnement de n'importe quelle application conteneurisée
- les [Dépôts](/fr/concepts/storage/repositories/){.cico .cico-repositories} qui permettront de lier un dépôt de code Git à un [Conteneur](/fr/concepts/storage/containers/){.cico .cico-containers}
- les [Déploiements](/fr/concepts/automations/deployments/){.cico .cico-deployments} qui permettront de déployer un [Conteneur](/fr/concepts/storage/containers/){.cico .cico-containers} sur [dropin.cloud](/fr/cloud/)
- les [Bases de données](/fr/concepts/endpoints/databases/){.cico .cico-databases} qui permettront d'utiliser une base de données spécifique pour l'utiliser dans une [Fonction](/fr/concepts/automations/functions/){.cico .cico-functions}
- ... et tous les autres Concepts possédant la mention "stable" dans la documentation

Il est essentiel pour nous de pouvoir garantir une stabilité et une sécurité maximale pour nos utilisateurs. Nous avons donc décidé de ne pas proposer de version stable tant que nous n'aurons pas atteint cet objectif.
