# Radio Classic — noms longs et collage de couleur (correctif)

Suite de la PR #305, deux remarques d'opérateurs. Aucun changement de protocole.

| # | Constat | Correction | Test |
|---|---------|------------|------|
| 1 | Un nom de station long (« Belgien Klara (Sint-Pieters-Leeuw Nahe Brüssel) ») est tracé sur une ligne et déborde sous le panneau voisin | Le grand nom garde sa taille ; ce qui est entre parenthèses passe dessous en petite police (15, comme le RadioText), même couleur ; un nom encore trop long passe sur 2 lignes. Le petit nom de l'en-tête est coupé par « … » avant la plage de bande | `a_long_station_name_keeps_its_size_and_the_bracket_goes_small_under_it` (échoue avant) |
| 2 | Le sélecteur de couleur des mémoires copie le code mais ne sait pas le recoller ; Cmd+V ne fait rien | Module `colour_edit` : bouton **PST** à côté de la pastille (demande le presse-papiers, `ViewportCommand::RequestPaste`) et Cmd/Ctrl+V pris au survol de la pastille ou picker ouvert. Formats : `#rrggbb`, `rrggbb`, `r, g, b` (ce que le bouton copie), `rgb(r,g,b)`. Appliqué à la couleur de chaque mémoire et à la couleur globale | `parse_colour` (valides/invalides), `cmd_v_over_the_swatch_pastes_the_colour`, `a_paste_that_is_not_a_colour_changes_nothing` |

Hors périmètre : coller dans le picker lui-même (fenêtre interne d'egui, non modifiable).
Retour arrière : revert de la PR.
