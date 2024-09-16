**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# StringTools

L'objectif est de créer une librairie qui permette de manipuler des chaînes de caractères avec différentes transformations.

## Fonctions à développer

3 fonctions principales doivent être proposées :

`reverse`
: renverse la chaîne de caractères passée en argument. Exemple : `"hello"` devient `"olleh"`.

`is_palindrome`
: vérifie si la chaîne de caractères est un palindrome (se lit de la même manière dans les deux sens). Exemple : `"radar"` renvoie `true`, mais `"hello"` renvoie `false`.

`remove_vowels`
: supprime toutes les voyelles (a, e, i, o, u) de la chaîne de caractères. Exemple : `"hello"` devient `"hll"`.

### En option

`rle_encode`
: applique la compression par encodage Run-Length (RLE) à la chaîne de caractères passée en argument. Exemple : `"aaabbc"` devient `"a3b2c1"`.

`to_case`
: convertit une chaîne en fonction du format choisi parmi `snake_case`, `CamelCase` ou `kebab-case`. Exemple : `"Hello World"` devient `"hello_world"` (snake_case), `"helloWorld"` (CamelCase) ou `"hello-world"` (kebab-case), selon le paramètre passé.

## Dépendances

Aucune dépendance externe n'est nécessaire pour cette librairie, tout est réalisé avec les fonctionnalités standard de Rust.
