# ID

Here is how an ID looks like: `blueforest:examples/id:v1:my-namespace/my-type`

An ID is composed of different parts splitted by `:`.

These parts are:
 - `blueforest` the [owner](#owner) of the models
 - `examples/id` the [model](#model) containing the recipes
 - `v1` the [version](#version) of your model
 - `my-namespace/my-type` the [recipe](#recipe), which can be a Type, a
   Function, a Collection or a Pipeline

The model and recipe parts may contain Namespaces, which are groups of models
or recipes. In our example, `examples/` is a model namespace and `my-namespace`
is a recipe (in this case, Types) namespace.

## Owner

Example: `blueforest`

An Owner is the User or the Company who owns the models. An Owner's ID is only
composed of one part (no `:`).

## Model

Example: `blueforest:examples/id`

Models group together recipes which have the same goal.

## Version

Example: `blueforest:examples/id:v1`

Versions correspond to Model release. They are isolated from each other, but it
is possible to make links to an older recipe.

## Recipe

Example: `blueforest:examples/id:v1:my-namespace/my-type`

Recipes are implementations of [Modules](../Modules/SUMMARY.md). They describe
a behavior.

