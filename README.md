**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# Txt-cli

L'objectif est de créer une application en ligne de commande qui permette de manipuler des chaînes de caractères avec différentes transformations.

## Sous-commandes à développer

En ce qui concerne les arguments : ils sont optionnels ; s'ils sont présents, il s'agit de fichiers ; s'ils sont absents, la commande lira le contenu de l'entrée standard pour trouver les données nécessaires au traitement.

Concernant les error levels supérieurs à 0 : à chaque fois qu'il s'en produit un, un message correspondant doit être envoyé vers la sortie d'erreur.

3 commandes doivent être proposées :

`reverse`
: renverse la chaîne de caractères passée en argument. Si l'argument est vide, renvoie un error level 1.

`is_palindrome`
: vérifie si la chaîne de caractères passée en argument est un palindrome. Si l'argument est vide, renvoie un error level 1.

`remove_vowels`
: supprime toutes les voyelles (a, e, i, o, u) de la chaîne de caractères passée en argument. Si l'argument est vide, renvoie un error level 1.

### En option

`rle_encode`
: applique la compression par encodage Run-Length (RLE) à la chaîne de caractères passée en argument. Si l'argument est vide, renvoie un error level 1.

`to_case`
: convertit la chaîne passée en argument en `snake_case`, `CamelCase`, ou `kebab-case`. Le type de conversion est spécifié par une option. Si l'argument est vide, renvoie un error level 1. L'option par défaut est CamelCase.

## Dépendances
- clap
