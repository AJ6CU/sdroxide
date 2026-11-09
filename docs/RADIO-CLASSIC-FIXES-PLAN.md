# Radio Classic — correctif (plan validé par l'opérateur, implémenté)

Branche neuve depuis `origin/main` (d2844aa, PROGRAMS déjà renommé par #303). Une PR, non mergée, non suivie.
Pas de changement de protocole prévu : tout passe par des commandes existantes et par `UiSettings` (non wire).

| # | Demande | Cause trouvée dans le code | Correction | Test (échoue avant) |
|---|---|---|---|---|
| 2 | RADIO = exclusivité | RADIO s'ajoute aux autres receveurs et décodeurs | **Décision opérateur** : une fois RADIO lancé, tous les autres modes sont coupés et leurs panneaux fermés (ex. SSTV → RADIO : SSTV se ferme, le flux et la fréquence passent aussitôt sur ceux de Radio Classic). À l'activation : mode numérique/DAB/SSTV… → WFM (ou dernier mode Radio Classic), fréquence Radio Classic (voir 5) appliquée immédiatement, `SetSubRx(false)`, `SetScanning(false)`, ISM/ADS-B décodage off, fenêtres de décodeurs, AIRCRAFT, ISM fermées. Choisir ensuite un autre mode/receveur désactive RADIO | test de trame : depuis SSTV, RADIO on → mode voix, fenêtre SSTV fermée, fréquence = celle de Radio Classic, sous-RX off |
| 3 | Fréquence de droite centrée | `freq_display::show` dans un layout centré mais le widget prend toute la largeur : chiffres collés à gauche | Mesurer la largeur du widget, l'allouer dans un rect centré | rect du widget : centre = centre de la colonne (±1 px) |
| 4 | PROGRAMS ouvre le vrai panneau | le chip ouvre un petit popup interne (`rc_schedule_popup`), pas la fenêtre PROGRAMS (`self.schedule.show`) | Le chip bascule `schedule.show`, lit comme celui de la barre du haut ; le popup et `schedule_open` disparaissent | clic simulé → `schedule.show == true` |
| 5 | 100 MHz au premier lancement, puis dernière fréquence | rien n'est mémorisé | `UiSettings.radio_classic_last: Option<(f64, Mode)>` : vide → `SetMode Wfm` + `SetVfo 100 MHz` ; sinon rappel. Mis à jour à chaque changement de fréquence tant que RADIO est actif (MEM, PROGRAMS, molette) | premier lancement = 100 MHz ; après un changement, la réouverture rend la dernière |
| 6 | Stéréo réelle en WFM 64–108 MHz, faux stéréo ailleurs | le chip STEREO n'apparaît que si le pilote est verrouillé (pas de chip éteint) ; le VU lit `rms_db` L/R du moteur, qui reste mono si `stereo_allowed` est faux (NR ou notch actifs, ou `wfm_stereo` coupé) | (a) chip STEREO toujours présent en 64–108, allumé seulement si `meters.stereo`, absent ailleurs ; (b) à l'entrée en WFM : `SetWfmStereo(true)`, et le chip dit pourquoi si NR/notch bloquent ; (c) hors 64–108 : VU « faux stéréo » — **affichage seul**, le son n'est pas touché (décision opérateur) | test moteur sur le signal stéréo synthétique existant (`wfm_stereo_iq`) : L ≠ R dans `AudioLevel` ; test UI du chip allumé/éteint |
| 7 | Chip RADIO dans WIDEBAND, après ISM | n'existe que dans EXTRAS | Chip dans `wideband_chip_grid`, même bascule que EXTRAS (une seule fonction, état identique) | les deux chips restent d'accord |
| 8 | Scope 2× plus lent, 10× plus de lignes | une ligne de cascade par trame, `WF_ROWS = 40`, un rect peint par case | Une ligne sur deux, `WF_ROWS = 400` ; cascade dessinée en **texture** (96×400) au lieu de 38 000 rects | cadence : N trames → N/2 lignes ; 400 lignes max |
| 9 | BW / STEP / AGC / SQL actifs | simples cadres peints, aucun clic | BW : clic = préréglage suivant du mode, clic droit = précédent, molette ; STEP : idem sur `next_tune_step` ; AGC : `SetAgc` suivant ; SQL : molette ±, clic = ouvert/dernier seuil ; `SetFilter` / `SetSquelch` | un test par case : clic → commande attendue |
| 10 | Aperçus visuels | — | Rendu avant/après avec le test `radio_classic_preview` existant, un PNG par point visible (3, 4, 6, 7, 8, 9) + capture de la liste WIDEBAND | PNG regardés, pas seulement les nombres |

## Déroulé après ton OK
1. Branche `claude/radio-classic-fixes` depuis `origin/main` (la PR #293 est mergée).
2. Un commit par point, test d'abord vérifié en échec sur le code non corrigé.
3. Contrôles : `cargo check --workspace --all-targets` silencieux, tests `sdroxide-ui` (+ moteur pour le 6), `cargo check --release --target wasm32-unknown-unknown -p sdroxide-ui`, `rustfmt` sur les hunks touchés.
4. Aperçus envoyés ici, puis PR ouverte, ni mergée ni suivie.

## Pas testable ici
Vrai signal FM stéréo, RDS, clics sur une vraie station : banc uniquement. Le point 6 est prouvé sur signal synthétique, pas sur l'antenne.

## Décisions de l'opérateur (prises)
- Point 2 : exclusivité totale, comme décrit ci-dessus.
- Point 6 : faux stéréo = VU seulement, aucun traitement du son.
- Branche neuve (pas de force-push) pour un revert propre : `claude/radio-classic-fixes`.
