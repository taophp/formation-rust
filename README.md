**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# Calc-web

L'objectif est créer application web qui permette d'évaluer des expressions mathématiques.

## Sous-commandes à développer

3 appels doivent être proposées :

`/check` en GET
: vérifie si l'argument `arg` est une expression évaluable avec meval ou eval, si non renvoie une erreur 400

`/calc` en GET
: évalue l'argument avec meval ou eval. Le résultat est renvoyé en html simple, si non renvoie une erreur 400

`/equal` en POST
: reçoit un tableau d'expressions en JSON, vérifie si les deux premières expressions sont égales, une par ligne de l'argument. Renvoie `true` ou `false` en JSON

### En option

`/generate24` en GET
: renvoie un tableau JSON de 4 chiffres comme défi pour le [jeu du 24](https://en.wikipedia.org/wiki/24_(puzzle))

`/check24` en POST
: reçoit un tableau d'expressions en JSON ; la première est composée d'un tableau de 4 chiffres et la seconde, une expression mathématique résolvant le jeu du 24. Vérifie que l'expression ne contient que les 4 chiffres donnés dans le tableau, une seule fois chacun, sinon, renvoie une erreur 400. Vérifie que l'expression résout bien le jeu du 24, renvoie un statut 200 si oui, une erreur 404 si non.

## Dépendances
- meval
- eval
- serde_json
- rocket
