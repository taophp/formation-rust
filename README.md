**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# Calc-cli

L'objectif est créer application en ligne de commande qui permette d'évaluer des expressions mathématiques.

## Sous-commandes à développer

En ce qui concerne les arguments : ils sont optionnels ; s'ils sont présents, il s'agit de fichiers ; s'ils sont absents, la commande lira le contenu de l'entrée standard pour trouver les données nécessaires au traitement.

3 commandes doivent être proposées :

`check`
: vérifie si l'expression passée en argument peut-être évaluée avec meval ou, sinon, eval, renvoie un error level de 1 si l'expression ne peut être évaluée

`calc`
: évalue l'expression passée en argument avec meval ou, sinon, eval. Le résultat est renvoyé sur la sortie standard

`are_equal`
: vérifie si deux expressions sont égales, inférieures ou supérieures. Ne revoie rien si les expressions sont égales, error level 1 si la première est inférieure, error level 2 si la seconde est inférieure.

### En option

`generate_24_challenge`
: renvoie sur la standard un list de 4 chiffres comme défi pour le [jeu du 24](https://en.wikipedia.org/wiki/24_(puzzle))

`check_24_challenge`
: reçoit un argument composé d'une liste de 4 chiffres séparés par des espaces sur la première ligne et une expression mathématique résolvant le jeu du 24 sur la seconde. Vérifie que l'expression ne contient que les 4 chiffres donnés dans le tableau, une seule fois chacun, sinon, renvoie l'error level 1. Vérifie que l'expression résout bien le jeu du 24, sinon renvoie l'error level 2.

## Dépendances
- meval
- eval
- serde_json
- clap
