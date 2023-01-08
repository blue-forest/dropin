---
hide: [ "toc" ]
---
# Roadmap

The development of drop'in was divided into seasons and episodes: we have been going from version to version since 2016 and we are currently at episode 2 of season 6.

During our journey we went through several technical stacks with a multitude of sandboxes which allowed us to progressively establish a set of features and define core concepts.

In previous seasons, drop'in was used as part of [Blue Forest](https://blueforest.cc){:target="_blank" rel="noopener"}'s services to create cross-platform applications for its clients. Today, we have finalized the specifications of drop'in to make it available publicly allowing anyone to build their own applications in total autonomy.


## alpha

This first phase will be dedicated to the implementation of the fundamental concepts of drop'in with all that is necessary for the creation of applications such as [Interfaces](/concepts/interfaces/){.cico .cico-interfaces} or [Automations](/concepts/automations/){.cico .cico-automations} for example. In this phase we will also propose a first version of all the concepts related to the constitution of a [Model](/concepts/catalog/models/){.cico .cico-models}, they are marked with the mention "alpha" in the documentation.

Our strategy will be to use [Visual Studio Code](https://code.visualstudio.com/){:target="_blank" rel="noopener"} as a base and integrate our extension to access all drop'in features. The projects will be hosted on the service of your choice (GitHub, GitLab, Gitea, BitBucket, ...) and the use of [dropin.cloud](/cloud/) will be possible through the extension that will guide you throughout the creation of your application. This documentation will also be integrated in the extension.

We are actively working on this phase but we do not have a release date yet. However, we have already made [this registration form](https://docs.google.com/forms/d/e/1FAIpQLSejGbv2SCbZ7xZwpdGSDTqEi3e7eg2FQNmsoZeJWaNxv27Nkw/viewform){:target="_blank" rel="noopener"} available to be informed about the alpha release and to be able to test the extension as soon as it becomes available.

Once the phase is launched, access to [dropin.cloud](/cloud/) will be completely free and limited to a specific number of resources. Access will be by invitation only, a registration form will be made available to allow anyone to request access on our final homepage.

What is or will be made open source:

- [this documentation](https://github.com/blue-forest/dropin/tree/main/recipes){:target="_blank" rel="noopener"}
- [our language schemas](https://github.com/blue-forest/dropin/tree/main/schemas){:target="_blank" rel="noopener"}
- the source code of the Visual Studio Code extension


## beta

This phase will be devoted to stabilizing all existing features and adding new ones.

Among the upcoming features that will offer a new level of possibilities, we have:

- [Migrations](/concepts/automations/migrations/){.cico .cico-migrations} which will make it easier to change the data structure between different versions of your application
- [Syntaxes](/concepts/validations/syntaxes/){.cico .cico-syntaxes} which will allow to decompose any text
- [Connectors](/concepts/endpoints/connectors/){.cico .cico-connectors} which will allow to connect your application to any third party service through APIs
- ... and all the other concepts that have the "beta" mention in the documentation

The use of Visual Studio Code will still be possible but we will also provide a web editor for those who do not wish to install anything on their machine. We will also be offering a mobile editor (web, Android and iOS) for those who want to create applications on their phone or tablet. Finally, a desktop version (Linux, Windows and MacOS) will also be available so that you do not have to use a browser.

Access to [dropin.cloud](/cloud/) will be possible without an invitation and a free version with a limited number of resources will still be available. Users will also be able to choose to upgrade to a paid subscription for unlimited resources and access to an advanced technical support. We will communicate more about the pricing and features of the paid subscription before the beta version will be released.

In addition to what is open source the alpha version, [our language compiler source code](https://github.com/blue-forest/dropin/tree/main/compiler){:target="_blank" rel="noopener"} will also be publicly available. The only services that will not be open source will be those integrated to [dropin.cloud](/cloud/) and that require creation of an account to use them.


## stable

This last phase will allow us to stabilize all the functionalities by locking the drop'in specifications while ensuring that all the functionalities are fully tested.

In addition, new features will be added to allow the creation of more complex applications. Among these, we have:

- [Containers](/concepts/storage/containers/){.cico .cico-containers} which will allow to describe the functioning of any containerized application
- [Repositories](/concepts/storage/repositories/){.cico .cico-repositories} which will allow to link a Git code repository to a [Container](/concepts/storage/containers/){.cico .cico-containers}
- [Deployments](/concepts/automations/deployments/){.cico .cico-deployments} which will allow to deploy a [Container](/concepts/storage/containers/){.cico .cico-containers} on [dropin.cloud](/cloud/)
- [Databases](/concepts/endpoints/databases/){.cico .cico-databases} which will allow you to use a specific database for use in a [Function](/concepts/automations/functions/){.cico .cico-functions}
- ... and all other concepts with the mention "stable" in the documentation


It is essential for us to be able to guarantee maximum stability and security for our users. We have therefore decided not to offer a stable version until we have achieved this goal.
