**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# Txt-web

L'objectif est de créer une application web qui permette de manipuler des chaînes de caractères.

## Appels à développer

5 appels doivent être proposés :

`/reverse?arg=` en GET
: renvoie l'argument `arg` inversé. Si l'argument est absent ou vide, renvoie une erreur 400.

`/is_palindrome?arg=` en GET
: vérifie si l'argument `arg` est un palindrome. Renvoie `true` ou `false` en JSON. Si l'argument est absent ou vide, renvoie une erreur 400.

`/remove_vowels?arg=` en GET
: renvoie l'argument `arg` avec toutes les voyelles supprimées. Si l'argument est absent ou vide, renvoie une erreur 400.

`/rle_encode?arg=` en GET
: applique la compression Run-Length Encoding (RLE) à l'argument `arg`. Si l'argument est absent ou vide, renvoie une erreur 400.

`/to_case` en POST
: reçoit un objet JSON avec deux clés : `"text"` et `"case_type"` (valeurs possibles : `snake`, `camel`, `kebab`). Renvoie la chaîne formatée selon le `case_type`. Si `case_type` est invalide ou absent, renvoie une erreur 400.

## Dépendances
- rocket
- serde
- serde_json
