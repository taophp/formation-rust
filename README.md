**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# Calc

L'objectif est créer une librairie qui permette d'évaluer des expressions mathématiques.

## Fonctions à développer

3 fonctions doivent être proposées :

`check`
: vérifie si l'expression passée en argument peut-être évaluée avec meval ou, sinon, eval.

`calc`
: évalue l'expression passée en argument avec meval ou, sinon, eval. Le résultat est renvoyé doit être du type serde_json::Value (le même que celui renvoyé par `eval::eval`).

`are_equal`
: vérifie si deux expressions sont égales, inférieures ou supérieures. Renvoie 0 s'ils sont égaux, -1 si le premier est inférieur, ou 1 si le premier est supérieur.

### En option

`generate_24_challenge`
: renvoie un tableau de 4 chiffres comme défi pour le [jeu du 24](https://en.wikipedia.org/wiki/24_(puzzle))

`check_24_challenge`
: reçoit deux arguments : un tableau de 4 chiffres et une expression mathématique résolvant le jeu du 24. Vérifie que l'expression ne contient que les 4 chiffres donnés dans le tableau, une seule fois chacun, sinon, renvoie l'erreur `CalcError::ChallengeFalseItems`. Vérifie que l'expression résout bien le jeu du 24, sinon l'erreur `CalcError::UnsolvedChallenge`. Si tout est vérifié, renvoie `Ok(())`.

## Dépendances
- meval
- eval
- serde_json
