**Nothing to see here !**

I created this repository to serve as a training resource. It probably doesn't contain anything useful for you, unless you are one of the participants.

# Mogulid

L'objectif est d'écrire une librairie puis de l'exploiter dans divers situations afin d'explorer les possibilités offertes par Rust.

Mogulid est une librairie assistant à la gestion de versions concurrentes.

À chaque élément versionné est associé une instance de la structure Mogulid. Celle-ci se compose d'un identifiant unique non-modifiable, de type `ulid`, `id`, faisant le lien entre l'instance et l'élément ; d'un état, `state`, de type `ulid` également, modifié à chaque version ; et, enfin, d'une liste des états précédents, `history`.

Lorsque deux instances d'un même élément sont considérées, la librairie aide à détecter et à résoudre les conflits, ou à s'assurer qu'il n'y en a pas. Chaque instance de l'élément présente le Mogulid qui lui est associé. L'`id` permettra de s'assurer qu'il s'agit bien du même élément, `state` de vérifier qu'il s'agit bien de deux versions concurentes, et `history` d'examiner à quel moment la divergence s'est produit et quels ont été les états successifs des deux éléments.
