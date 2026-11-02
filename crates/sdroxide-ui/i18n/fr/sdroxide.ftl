# sdroxide — traduction française de l'interface. La référence est
# en-US/sdroxide.ftl ; ce qui manque ici s'affiche en anglais.

## The language's own name, as the language picker lists it.
language-name = Français

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = Automatique
layout-mode-desktop = Bureau
layout-mode-tablet = Tablette
layout-mode-small = Petit écran
layout-mode-phone = Téléphone

ui-theme-default = Par défaut
ui-theme-light = Clair
ui-theme-high-contrast = Contraste élevé
ui-theme-green-phosphor = Phosphore vert
ui-theme-amber-phosphor = Phosphore ambre
ui-theme-teal-orange = Sarcelle / orange
ui-theme-rainbow = Arc-en-ciel
ui-theme-nord = Nord
ui-theme-nord-dark = Nord sombre
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized sombre
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin mocha
ui-theme-catppuccin-latte = Catppuccin latte
ui-theme-modern-minimalist = Minimaliste moderne

chrome-style-angled = Anguleux
chrome-style-rectangular = Rectangulaire
chrome-style-rounded = Arrondi
chrome-style-gradient = Dégradé
chrome-style-bevel = Biseau 3D
chrome-style-terminal = Terminal

font-size-small = Petite
font-size-medium = Moyenne
font-size-large = Grande

bandplan-kind-ham = Amateur
bandplan-kind-cw = CW
bandplan-kind-digi = Numérique
bandplan-kind-phone = Phonie
bandplan-kind-beacon = Balise
bandplan-kind-broadcast = Radiodiffusion
bandplan-kind-am = AM / OL / PO
bandplan-kind-cb = CB

speech-verbosity-terse = Concise
speech-verbosity-normal = Normale
speech-verbosity-full = Complète

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = Cadran (14 point 074)
speech-freq-style-digits = Chaque chiffre
speech-freq-style-full = Cadran avec unités

speech-callsign-style-phonetic = Phonétique (kilo one alpha)
speech-callsign-style-letters = Lettres (kay one ay)
speech-callsign-style-as-is = Tel qu'écrit

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = Affichage
settings-ui-moved-note = La largeur du panoramique, la réaction du spectre et la vitesse de défilement de la cascade se règlent avec le bouton SPEC de la barre d'affichage — le menu DISP sur une fenêtre étroite.
settings-ui-language = Langue
    .hint = La langue de l'interface. Le manuel accessible avec F1 et les annonces vocales restent en anglais, quelle que soit la langue choisie ici.
settings-ui-layout = Disposition
settings-ui-theme = Thème
settings-ui-button-style = Style des boutons
settings-ui-frame-rate = Fréquence de mise à jour de l'écran
settings-ui-palette = Palette de la cascade
settings-ui-tuning-buttons = Boutons d'accord
settings-ui-tuning-buttons-show = Afficher sur téléphone et tablette
    .hint = Une rangée de boutons diminuer / pas / augmenter sous la barre de commandes d'un écran tactile. Il n'y a ni molette pour faire défiler un chiffre ni cadran, donc sans ces boutons le seul moyen de changer d'un pas connu est de saisir toute la fréquence. Jamais affichés sur un ordinateur de bureau.
settings-ui-tuning-step = Pas : { $step }
settings-ui-tuning-snap = Le premier appui aligne sur le pas
    .hint = Un cadran placé entre deux multiples du pas passe au multiple suivant dans le sens de l'appui — 7 074 300 monte à 7 075 000 avec un pas de 1 kHz, ou descend à 7 074 000. Ensuite les boutons avancent du pas comme d'habitude. Désactivé, chaque appui avance exactement d'un pas.
settings-ui-smoothing = Lissage de la cascade
settings-ui-smoothing-interpolate = Interpoler
    .hint = Mélange chaque pixel de l'écran avec les cases et les lignes qui l'entourent, pour qu'un signal paraisse continu quand l'affichage est plus large que la transformée. Désactivez-le pour une cascade rectangulaire — un bloc par case, une ligne par balayage — ce qui est nécessaire pour lire la forme d'un signal sur l'image : un signal interpolé ne se distingue pas d'un signal réellement plus large. Une FFT plus grande (la puce FFT) est l'autre moitié de la solution.
settings-ui-background = Fond du spectre
settings-ui-background-gradient = Dégradé
settings-ui-background-top = haut
settings-ui-background-bottom = bas
settings-ui-spot-colours = Couleurs des étiquettes de spots
    .hint = La teinte de chaque source de spots : les cases le long du bas de la cascade, les badges de la liste SPOTS et les points sur la carte du monde.
settings-ui-spot-colour = Couleur des spots { $kind }
settings-ui-spot-colours-reset = Remettre chaque couleur de spot à sa valeur par défaut
settings-ui-reset = Réinitialiser
settings-ui-bandplan-colours = Couleurs du plan de bande
    .hint = La nuance de chaque classe d'attribution, dessinée sur la bande du plan de bande le long du bas de la cascade. Les blocs sont dessinés semi-transparents par-dessus la cascade, donc ils y paraissent plus sombres que l'échantillon ici.
settings-ui-bandplan-colour = Couleur des attributions { $kind }
settings-ui-bandplan-colours-reset = Remettre chaque couleur du plan de bande à sa valeur par défaut
settings-ui-skimmer-font = Taille de police du skimmer
settings-ui-waterfall-font = Taille de police de la cascade / du spectre
settings-ui-interface-font = Taille de police de l'interface
    .hint = Met à l'échelle toute l'interface — menus, boîtes de dialogue, fenêtres, onglets radio, la barre supérieure et ses boutons. Les deux tailles ci-dessus sont relatives à celle-ci.
settings-ui-cities = Villes sur les cartes
    .hint =
        Afficher les villes du monde — un point par lieu, avec son nom à côté s'il y a la place — sur les cartes plates : FT8/WSPR, APRS, ADS-B et AIS.

        Elles indiquent surtout *où* se trouve un point sur une carte du monde entier, et ce sont aussi les éléments les plus chargés de la carte. Désactivez-les si les noms gênent la lecture des stations. Le globe 3D n'est pas concerné : ses villes sont des lumières du côté nuit, pas des repères, et rien n'y est écrit par-dessus un contact.
settings-ui-cities-show = afficher les villes et leurs noms
settings-ui-3d-view = Vue 3D
settings-ui-clouds = Rendu des nuages
settings-ui-clouds-layered = En couches
settings-ui-clouds-volumetric = Volumétrique
settings-ui-clouds-note = La façon dont la couche NUAGES de la vue 3D dessine la météo. En couches empile des tranches de la troposphère et c'est l'option économique. Volumétrique fait traverser la troposphère par un rayon, si bien que le Soleil projette les sommets des nuages sur le sol en contrebas et que la foudre rayonne à travers l'orage qui la produit, au lieu de seulement éclaircir son extérieur — à plusieurs fois le coût par pixel.

## Settings window, UI tab: voice announcements.

speech-title = Annonces vocales
speech-enabled = Annoncer les changements de la radio
    .hint = Énonce ce qui a changé, pour pouvoir utiliser la radio sans la regarder
speech-voice = Voix
speech-voice-shipped = Voix fournie
speech-speed = Vitesse
    .hint = La voix étire ou compresse sa propre diction, sans changer la hauteur. Au-delà d'environ 2 ×, elle ne se raccourcit plus.
speech-volume = Volume
speech-output = Sortie
speech-detail = Détail
speech-duck = Atténuer le récepteur
speech-duck-while = Pendant la parole
speech-test = Tester
speech-what = Ce qu'il faut annoncer
speech-cat-frequency = Fréquence
speech-cat-mode-band = Mode et bande
speech-cat-vfo-split = VFO et split
speech-cat-agc-gain = CAG et gain
speech-cat-levels = Excitation, accord et micro
speech-cat-ptt = Émission et réception
speech-cat-rit-xit = RIT et XIT
speech-cat-memory-scan = Mémoires et balayage
speech-cat-band-edge = Sortie d'une bande amateur
speech-cat-notices = Avertissements et messages
speech-cat-filters = Filtres, squelch et réduction de bruit
    .hint = Désactivé par défaut : ces réglages changent sans arrêt pendant la recherche d'un signal
speech-decoded = Messages décodés
speech-ft8-to-me = Appels FT8 qui me sont adressés
speech-ft8-cq = CQ FT8 auxquels je pourrais répondre
    .hint = Une soirée chargée sur vingt mètres en compte une centaine à la minute
speech-js8 = Messages JS8 qui me sont adressés
speech-js8-allcall = JS8 @ALLCALL aussi
speech-fsq = Messages FSQ qui me sont adressés
speech-include-snr = Inclure le rapport
    .hint = Uniquement lorsque le message n'en contient pas un — un décodage qui donne déjà un nombre n'en reçoit pas un second de notre part
speech-ft8-qso = Mon propre échange FT8
    .hint = Ce que le séquenceur est sur le point d'envoyer, et la fin du contact
speech-reading-text = Lire à voix haute le texte décodé
speech-text-cw = CW
speech-text-rtty-psk = RTTY, PSK, Olivia, THOR, FSQ
speech-text-cw-locked = CW seulement lorsque le décodeur est verrouillé
    .hint = Lire la sortie d'un décodeur non verrouillé est pire que le silence
speech-text-note = Les deux sont désactivés par défaut. Un décodeur produit du texte plus vite que la parole ne le lit, donc tout ce qui prend trop de retard sur l'audio en direct est abandonné au lieu d'être mis en file d'attente.
speech-tuning-up = Accord en cours
speech-swr-while-tuning = Annoncer le ROS pendant que TUNE est maintenu
speech-swr-every = Toutes les
speech-swr-summary = Annoncer la meilleure adaptation au relâchement
speech-swr-alarm = Avertir d'un ROS élevé pendant toute émission
speech-how-read = Comment les choses sont lues
speech-frequencies = Fréquences
speech-callsigns = Indicatifs
speech-quiet-on-tx = Rester silencieux pendant l'émission
    .hint = La parole sort par vos haut-parleurs, donc entre dans votre microphone. Les avertissements de ROS élevé passent quand même.
speech-device-note = Les annonces sont jouées sur leur propre périphérique audio, donc elles ne sont jamais enregistrées ni envoyées à quiconque écoute à distance. Les touches pour énoncer l'état, répéter la dernière annonce et interrompre une phrase se trouvent dans l'onglet Commandes.

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = En attente de la configuration réseau de la station…
settings-device-reports = tel que le périphérique l'annonce
settings-transverters = Transverteurs
settings-one-row-per-box-while = Une ligne par case. Tant que le cadran est dans la bande d'une ligne, la radio est accordée sur cadran + décalage et la règle d'émission et la limite d'excitation de cette ligne s'appliquent ; hors de toute ligne, la radio fonctionne sur ses propres bandes. Les lignes sont essayées de haut en bas.
settings-name = Nom
settings-band-low = Bas de bande
settings-band-high = Haut de bande
settings-offset = Décalage
settings-transmit = Émission
settings-max-drive = Excitation max
settings-off-takes-transverter-out-line = Désactivé retire ce transverteur de la liste sans perdre la ligne — la bande revient à ce que la radio atteint d'elle-même.
settings-t-2-m = 2 m
settings-what-you-call-shown-log = Le nom que vous lui donnez. Affiché dans le journal lorsque le cadran le sélectionne.
settings-bottom-band-transverter-works-dial = Le bas de la bande que ce transverteur couvre, sur le cadran. 144 pour le 2 m.
settings-top-band-dial-148-2 = Le haut de la bande, sur le cadran. 148 pour le 2 m dans les régions 2 et 3, 146 dans la région 1.
settings-radio-ends-up-dial-offset = La radio se retrouve sur cadran + décalage, donc un transverteur qui ramène une bande vers une F.I. a un décalage négatif : le 2 m vers une F.I. de 28 MHz vaut -116, car 144 - 116 = 28. Faites glisser pour ajuster au hertz près un oscillateur légèrement décalé.

    Par le même convertisseur : cela fonctionne dans les deux sens, et l'émission prend le même décalage.

    Votre réglage d'excitation est maintenu sous cette valeur plutôt que modifié, pour que le nombre que vous utilisez en HF revienne lorsque le cadran quitte la bande.
settings-remove = SUPPRIMER
settings-add-transverter = AJOUTER UN TRANSVERTEUR
settings-takes-effect-apply-reconnect-like = Prend effet à l'application / la reconnexion, comme le décalage du convertisseur ci-dessus.
settings-maximum-transmit-drive = Excitation maximale à l'émission
settings-hard-limit-drive-tune-controls = Une limite stricte que les commandes Excitation et TUNE ne peuvent pas dépasser, appliquée après l'étalonnage par bande ci-dessous pour que rien ne puisse la repousser au-delà. Réglez-la à la puissance nominale de la radio : toute la commande Excitation devient alors utilisable. Sur un émetteur qui module ses propres échantillons (HPSDR, LimeSDR, PlutoSDR, HackRF), l'excitation maximale signifie émetteur grand ouvert, ce qui, avec un amplificateur à fort gain, dépasse largement ce que ses étages finaux supportent. Désactivé signifie que les commandes atteignent l'excitation maximale.
settings-limit-drive = Limiter l'excitation à
settings-per-cent-full-drive-key = Pourcentage de l'excitation maximale. Émettez sur une charge fictive et baissez cette valeur jusqu'à ce que le wattmètre indique la puissance nominale de l'amplificateur.
settings-applies-immediately-every-band-tune = S'applique immédiatement, sur chaque bande et aussi à TUNE.
settings-transmit-drive-band = Excitation à l'émission par bande
settings-every-amplifier-has-different-gain = Chaque amplificateur a un gain différent sur chaque bande, donc un même réglage d'excitation donne une puissance différente sur chacune. Mesurez la sortie sur chaque bande, puis ajustez à la baisse les bandes qui sortent trop fort jusqu'à ce qu'elles correspondent toutes : réglez l'excitation pour la bande qui en demande le plus (en général la plus haute), et abaissez les autres à son niveau. Décibels de puissance de sortie, appliqués à la voix, aux modes numériques et à TUNE. Zéro partout — la valeur par défaut — ne change rien.
settings-other = Autre
settings-clear-all = TOUT EFFACER
settings-back-no-calibration-drive-setting = Retour à aucun étalonnage : le réglage d'excitation atteint chaque bande intégralement.
settings-applies-immediately-band-you-would = S'applique immédiatement, et à la bande sur laquelle vous émettriez — derrière un transverteur, c'est la bande du cadran, et la limite d'excitation propre au transverteur l'emporte toujours sur tout réglage fait ici.
settings-settings = Réglages
settings-enter-server-s-address-first = saisissez d'abord l'adresse du serveur
settings-enter-receiver-s-address-first = saisissez d'abord l'adresse du récepteur, ou choisissez-en un dans SDR publics
settings-no-radio-selected-press-discover = aucune radio sélectionnée — appuyez sur Découvrir, ou saisissez une adresse
settings-enter-radio-s-address-first = saisissez d'abord l'adresse de la radio
settings-general = Général
settings-radio = Radio
settings-ui = Interface
settings-alerts = Alertes
settings-controls = Commandes
settings-spots = Spots
settings-freedv = FreeDV
settings-uploads = Envois
settings-winlink = Winlink
settings-servers = Serveurs
settings-t-r-switch = Commutateur E/R
settings-tle = TLE
settings-profiles = Profils
settings-station = Station
settings-enter-digital-mode-ft8-sstv = Saisissez un mode numérique (FT8 / SSTV / …) une fois pour charger les valeurs enregistrées.
settings-callsign = Indicatif
settings-grid-square = Carré (locator)
settings-iaru-region = Région UIR
settings-which-radio-interface-server-uses = L'interface radio qu'utilise le serveur se règle sur la machine où il tourne.
settings-radio-interface = Interface radio
settings-waiting-machine-radio-attached-interface = En attente de la machine à laquelle la radio est reliée. Ses réglages d'interface figurent ci-dessous et peuvent être modifiés d'ici en attendant.
settings-converter = Convertisseur
settings-manual-type-offset-below = Manuel — saisissez un décalage ci-dessous
settings-what-box-reads-when-offset = Ce que cette case affiche lorsque le décalage ne fait pas partie des préréglages. Réglez-le en saisissant dans le champ Décalage ci-dessous ; il n'y a rien à sélectionner ici.
settings-frequency-converter-front-receiver-pick = Un convertisseur de fréquence placé devant le récepteur. Choisissez-en un, ou saisissez un décalage à côté.
settings-how-far-converter-moves-signal =
    De combien un convertisseur déplace le signal sur son trajet vers le récepteur, en Hz — le même nombre et le même signe que dans la documentation de chaque convertisseur et dans chaque autre programme SDR. La radio se retrouve sur cadran + décalage : positif pour un convertisseur montant (un Ham It Up vaut 125000000), négatif pour tout ce qui ramène une bande vers une F.I. — un LNB satellite, ou un transverteur 2 m sur une F.I. de 28 MHz, soit -116000000 (28 − 144). 0 = pas de convertisseur.

    Faites glisser pour ajuster au hertz près, ce qu'attend un convertisseur dont l'oscillateur est légèrement décalé.

    Ceci concerne le trajet de réception. Ce qui se trouve sur la ligne d'émission est la ligne ci-dessous.

    Prend effet à l'application.
settings-what-transmit-line-while-converter =
    Ce qui se trouve sur la ligne d'émission lorsqu'un convertisseur est défini.

    Désactivé pendant la conversion : rien n'est émis — la valeur sûre par défaut, et la bonne pour un accessoire en réception seule.

    Par le même convertisseur : une même case convertit dans les deux sens, comme un transverteur. L'émission prend le décalage ci-dessus et le suit lorsqu'il est ajusté.

    Son propre décalage : la ligne d'émission est différente de celle de réception. Zéro est le cas courant — un convertisseur (un LNB, un Ham It Up) en réception avec l'émetteur sur sa propre antenne, ce qui est la station QO-100.

    La vérification de bande amateur s'applique toujours, sur la fréquence sur laquelle vous émettez.

    Prend effet à l'application.
settings-how-far-transmit-line-moves =
    De combien la ligne d'émission déplace le signal, en Hz, avec la même règle de signe que le décalage de réception : la radio émet sur cadran + décalage.

    0 = l'émetteur est relié à sa propre antenne et travaille sur la fréquence qu'indique le cadran — la liaison montante 2,4 GHz d'une station QO-100.

    Un convertisseur d'émission qui convertit une F.I. *vers le haut* est un nombre négatif : la radio travaille en dessous du cadran.
settings-rx-range = Plage RX
settings-which-frequencies-radio-receives-mhz =
    Les fréquences que cette radio reçoit, en MHz : 144-146, 430-440. Laissez vide pour utiliser ce que le périphérique annonce de lui-même.

    Les boutons de bande hors de la plage sont grisés, et le cadran ne va pas jusque-là.

    Ce sont des fréquences de cadran. Avec un convertisseur défini, indiquez la bande que vous accordez — 144-148 pour un transverteur 2 m — et non la F.I. sur laquelle la radio est réellement réglée.

    Prend effet à l'application.
settings-tx-range = Plage TX
settings-which-frequencies-radio-transmits-mhz =
    Les fréquences sur lesquelles cette radio émet, en MHz : 144-146, 430-440. Laissez vide pour utiliser ce qu'annonce le périphérique — et s'il n'annonce rien, on fait confiance au pilote et toute fréquence est autorisée.

    Fréquences de cadran, comme la plage de réception ci-dessus.

    C'est une limite que vous fixez, pas une autorisation : l'émission hors des bandes amateur est refusée dans tous les cas, sauf si vous l'avez désactivé dans config.toml. Et cela ne donne pas un émetteur à un appareil de réception seule.

    Prend effet à l'application.
settings-antenna = L'antenne est
settings-where-radio-listens-station-antenna =
    D'où cette radio écoute.

    À la station : l'antenne est à vous, donc votre propre locator indique où elle se trouve. Valeur par défaut, et la bonne pour tout ce qui est dans la station.

    Ailleurs : un récepteur en ligne, ou votre propre installation sur une colline. Ce qu'il entend est transmis à PSK Reporter, WSPRnet et FreeDV Reporter à partir du locator à côté, jamais du vôtre — et ADS-B place aussi les avions par rapport à lui.

    Choisir un récepteur dans « SDR publics » remplit ce champ pour vous.

    Prend effet à l'application.
settings-receiver-s-maidenhead-locator-jn88ec =
    Le locator Maidenhead du récepteur — JN88ec, DO30db. Quatre ou six caractères.

    Laissez vide si vous ne savez pas où se trouve le récepteur : rien n'est alors transmis du tout, ce qui est la seule réponse honnête. Transmettre depuis votre propre carré placerait la réception de quelqu'un d'autre sur le mauvais continent.

    S'ajoute à cal_offset_db dans config.toml, qui est un chiffre unique pour toute la station.
settings-antenna-somewhere-else-no-locator = L'antenne est ailleurs et aucun locator n'est défini, donc rien de ce qu'entend cette radio n'est transmis à PSK Reporter, WSPRnet ou FreeDV Reporter. Saisissez le locator du récepteur ci-dessus pour transmettre depuis lui.
settings-ranges-are-mhz-low-high = Les plages sont en MHz, bas-haut, séparées par des virgules : 144-146, 430-440 — soit 144000000-146000000 Hz et 430000000-440000000 Hz. Ce sont des fréquences de cadran : un décalage de convertisseur ne les déplace pas. Le décalage de convertisseur ci-dessus est le champ en hertz. Laissez une plage vide pour utiliser ce que le périphérique annonce de lui-même ; un périphérique qui n'annonce rien est pris au mot.
settings-careful-rtl-sdr-blog-v4 = Attention avec un RTL-SDR : le Blog V4 convertit déjà vers le haut en dessous de 28,8 MHz ; un décalage négatif qui amène le matériel dans cette zone se décale donc deux fois.
settings-pick-radio-interface-above-configuration = Choisissez une interface radio ci-dessus (cette configuration utilisait le mode de détection automatique, aujourd'hui supprimé).
settings-configuration-used-removed-auto-detect = Cette configuration utilisait le mode de détection automatique, aujourd'hui supprimé. Choisissez une interface ci-dessus une fois que la machine à laquelle la radio est reliée a répondu.
settings-radio-has-no-interface-yet = Cette radio n'a pas encore d'interface — choisissez-en une ci-dessus et appuyez sur Appliquer / reconnecter.
settings-radio-has-no-interface-yet-2 = Cette radio n'a pas encore d'interface. On peut en choisir une ci-dessus une fois que la machine à laquelle la radio est reliée a répondu.
settings-apply-reconnect = Appliquer / reconnecter
settings-switch-interface-now-no-restart = Passer à cette interface maintenant — aucun redémarrage nécessaire
settings-switch-server-s-radio-interface = Passer tout de suite la radio du serveur à cette interface — aucun redémarrage nécessaire. L'ancienne continue de tourner si la nouvelle ne peut pas être ouverte.
settings-switches-live-radio-without-restarting = Bascule la radio en direct, sans redémarrage.
settings-everything-above-server-s-own = Tout ce qui précède est la configuration propre du serveur, et les modifications y sont enregistrées. La plupart des réglages s'appliquent dès que vous les modifiez ; ceux qui sont fixés à l'ouverture du périphérique — le taux d'échantillonnage, une adresse, l'interface elle-même — demandent ce bouton.
settings-dx-cluster-telnet = Cluster DX (telnet)
settings-enabled = Activé
settings-host = Hôte
settings-port = Port
settings-login-call = Indicatif de connexion
settings-commands = Commandes
settings-one-per-line-e-g = une par ligne, p. ex. SET/FT8
settings-one-per-line-e-g-2 = une par ligne, p. ex. set/filter cont=eu
settings-sent-after-login-place-narrow = Envoyé après la connexion. C'est là qu'il faut restreindre le flux — sans filtre, c'est chaque skimmer de la Terre.
settings-rbn-paths-are-placed-country = Les trajets RBN sont placés depuis le centre des pays, pas depuis des locators — exacts pour un petit pays, très inexacts pour un grand. Ils forment une couche séparée sur la carte de propagation, et peuvent y être désactivés.
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = Spots d'activateurs POTA
settings-sota-spots = Spots SOTA
settings-psk-reporter-current-band = PSK Reporter (bande courante)
settings-upload-my-ft8-ft4-ft2 = Envoyer mes décodages FT8/FT4/FT2/JS8
settings-antenna-2 = Antenne
settings-collector = Collecteur
settings-t-4739-live-collector-14739-test = 4739 est le collecteur en service, 14739 celui de test
settings-max-age-s = Âge max (s)
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = Envoyer mes décodages WSPR
settings-send-every-wspr-reception-wsprnet = Envoyer chaque réception WSPR à wsprnet.org. Activé par défaut : cela n'émet rien, et rapporter ce que vous entendez est ce qui fait d'un récepteur WSPR un élément du réseau plutôt qu'une simple curiosité privée. Un créneau sans aucun décodage est aussi transmis, ce qui permet au réseau de distinguer une bande fermée d'un récepteur éteint.
settings-download-who-heard-me = Télécharger qui m'a entendu
settings-ask-wsprnet-org-which-stations = Demander à wsprnet.org quelles stations ont décodé celle-ci. WSPR n'a aucun accusé de réception, donc c'est le seul moyen pour une balise émettrice d'apprendre quoi que ce soit sur sa portée.
settings-ask-every = Demander toutes les
settings-looking-back = en remontant
settings-apply = {" "}APPLIQUER{" "}
settings-persist-re-connect-feeds = Enregistrer et (re)connecter les flux
settings-broadcast-stations = Stations de radiodiffusion
settings-winlink-account = Compte Winlink
settings-password = Mot de passe
settings-winlink-account-password-not-gateway = Le mot de passe du compte Winlink, pas celui de la passerelle. Il respecte la casse — saisissez-le exactement comme il vous a été délivré.
settings-locator = Locator
settings-how-connect = Comment se connecter
settings-route = Route
settings-internet = Internet
settings-forward-cms-over-internet = Transférer via le CMS par Internet
settings-radio-packet = Radio (paquet)
settings-call-rms-gateway-air-radio = Appeler une passerelle RMS sur les ondes. La radio doit être en PACKET ou PACKET-HF.
settings-gateway = Passerelle
settings-via = Via
settings-digipeaters-order-separated-spaces-usually = Digipeaters, dans l'ordre, séparés par des espaces. Généralement vide — une passerelle que vous entendez directement est une passerelle que vous devez appeler directement.
settings-speed = Vitesse
settings-most-rms-gateways-answer-1200 = La plupart des passerelles RMS répondent à 1200. Appeler une passerelle 1200 en 9600 sonne exactement comme une passerelle hors de l'air : réglez donc cette valeur d'après ce que publie le responsable de la passerelle, plutôt que de deviner. 9600 exige aussi le port de données de la radio aux deux bouts. 300 est en HF.
settings-frequency = Fréquence
settings-clear = EFFACER
settings-zero-leaves-dial-alone-which = Zéro laisse le cadran tranquille, ce que vous voulez lorsque vous restez sur un seul canal. Toute autre valeur accorde la radio au démarrage de la session.
settings-my-gateways = Mes passerelles
settings-winlink-s-published-gateway-list = La liste publiée des passerelles de Winlink exige une clé d'API que sdroxide n'a pas, donc conservez la vôtre. Les deux ou trois passerelles joignables depuis un même lieu s'apprennent en essayant, et elles changent rarement.
settings-use-2 = UTILISER
settings-call-one-next-connect = Appeler celle-ci à la prochaine connexion
settings-current-dial = cadran actuel
settings-forget = OUBLIER
settings-forget-gateway = Oublier cette passerelle
settings-add-gateway = + AJOUTER UNE PASSERELLE
settings-save-gateway-above-list = Enregistrer la passerelle ci-dessus dans la liste
settings-internet-gateway = Passerelle Internet
settings-cms-address = Adresse du CMS
settings-client-name = Nom du client
settings-winlink-s-production-servers-only = Les serveurs de production de Winlink n'acceptent que les noms de client qu'ils connaissent, et répondent à un nom inconnu par « Unknown client types are not allowed on production servers ». Tant que sdroxide n'est pas enregistré auprès de la Winlink Development Team, la connexion exige un nom qu'ils reconnaissent.
settings-automatic-connection = Connexion automatique
settings-connect-timer = Se connecter sur minuterie
settings-every = Toutes les
settings-persist-winlink-account = Enregistrer le compte Winlink
settings-callsign-lookup = Recherche d'indicatif
settings-provider = Fournisseur
settings-auto-fill-name-qth-grid = Remplir automatiquement nom/QTH/carré au clic sur un spot et pour un QSO
settings-choose-provider-enter-login = Choisissez un fournisseur pour saisir son identifiant.
settings-qrz-user = Utilisateur QRZ
settings-qrz-pass = Mot de passe QRZ
settings-hamqth-user = Utilisateur HamQTH
settings-hamqth-pass = Mot de passe HamQTH
settings-one-hamqth-account-does-both = Un seul compte HamQTH sert aux deux — cet identifiant est celui qu'utilise l'onglet d'envoi HamQTH.
settings-upload = Envoi
settings-auto-upload-each-new-qso = Envoyer automatiquement chaque nouveau QSO
settings-master-switch-which-services-qso = L'interrupteur général. Les services vers lesquels va un QSO se règlent dans chaque onglet ci-dessous.
settings-turn-auto-upload-each-new = Activez d'abord « Envoyer automatiquement chaque nouveau QSO » ci-dessus — un service coché ici n'est pas envoyé tant que cette option n'est pas active.
settings-auto-upload-off-above-so = L'envoi automatique est désactivé ci-dessus, donc les cases des services sont inactives et rien n'est envoyé automatiquement pour l'instant — le bouton ENVOYER de chaque QSO dans le journal fonctionne toujours.
settings-qrz-log-key = Clé du journal QRZ
settings-qrz-logbook-api-key-your = La clé d'API du journal QRZ, à lire dans les réglages de votre journal QRZ — pas l'identifiant de recherche XML.
settings-eqsl-user = Utilisateur eQSL
settings-eqsl-pass = Mot de passe eQSL
settings-eqsl-qth = QTH eQSL
settings-same-login-hamqth-callsign-lookup = Le même identifiant que la recherche d'indicatif HamQTH — remplir l'un remplit les deux.
settings-club-log-email = Courriel Club Log
settings-club-log-pass = Mot de passe Club Log
settings-club-log-key = Clé Club Log
settings-wrl-api-key = Clé d'API WRL
settings-developer-api-key-world-radio = La clé d'API développeur de World Radio League → Intégrations → API développeur. Elle n'est affichée qu'une fois, au moment où vous la générez : copiez-la alors. Les contacts vont dans votre journal par défaut — définissez-en un dans WRL si vous en tenez plusieurs.
settings-confirmations-download = Confirmations (téléchargement)
settings-lotw-user = Utilisateur LoTW
settings-lotw-pass = Mot de passe LoTW
settings-lotw-upload-uses-tqsl-export = L'envoi LoTW utilise TQSL — exportez l'ADIF depuis le journal et signez-le. Les confirmations LoTW/eQSL sont téléchargées ici pour distinguer les contacts travaillés des contacts confirmés.
settings-sync-confirmations = SYNCHRONISER LES CONFIRMATIONS
settings-fetching-subscriptions = Récupération des abonnements…

    Pour l'utiliser à nouveau seule, videz la case Récepteur du panoramique sur la page de cette radio.
settings-panadapter = Panoramique
settings-give-radio-wideband-panadapter-borrowing = Donner à cette radio un panoramique à large bande en empruntant le récepteur d'une autre radio — un SDR sur la même antenne, ou alimenté depuis la sortie F.I. de cette radio. Le spectre, la cascade, le sous-récepteur, les modes numériques et le skimmer viennent tous de ce récepteur ; le cadran, le mode, le filtre et l'émetteur restent ici.
settings-receiver = Récepteur
settings-none = Aucun

    Prend effet à l'application.
settings-connected = Connecté à

    Ce n'est pas le champ Convertisseur ci-dessus, qui accorde la radio, ni le décalage du centre I/Q de l'onglet CAT, qui concerne la carte son de cette radio. Rien de ce qui est saisi ici n'est jamais envoyé à l'une ou l'autre radio.

    Prend effet à l'application.
settings-audio-2 = Audio depuis
settings-follow-dial = Suivre le cadran
settings-keep-receiver-radio-s-dial = Maintenir le récepteur sur le cadran de cette radio, et déplacer le cadran lorsque vous accordez sur le panoramique. Désactivé, le récepteur reste où il est — une écoute fixe sur un segment pendant que la radio va ailleurs.
settings-invert-spectrum = Inverser le spectre
settings-mirror-receiver-s-span-about = Symétrise la largeur du récepteur autour de son centre, pour une prise dont l'oscillateur est au-dessus du signal et qui renvoie la bande à l'envers. Laissez-le désactivé sauf si la cascade se remplit de signaux tous situés du mauvais côté du cadran.
settings-mute-transmit = Couper le son à l'émission
settings-silence-receive-audio-while-radio = Couper l'audio de réception pendant que cette radio émet. Activé par défaut : avec le récepteur sur la même antenne, ou sur la F.I. de cette radio, ce qu'il entend pendant un trafic est votre propre émetteur. Les trafics déclenchés sur la radio elle-même — le PTT du microphone, une pédale, MOX — comptent aussi, dès que la liaison CAT indique que l'appareil est passé en émission.
settings-blank-transmit = Masquer à l'émission
settings-stop-panadapter-waterfall-while-radio = Arrêter le panoramique et la cascade pendant que cette radio émet. Activé par défaut : un émetteur qui couvre toute la largeur efface la bande derrière lui. Désactivez-le pour observer la bande — ou votre propre signal — pendant un trafic, ce qui vaut la peine avec le récepteur sur une antenne séparée. Ne concerne que les trafics que sdroxide déclenche ; un trafic déclenché sur la radio est reçu sans interruption, quel que soit ce réglage, et l'affichage continue de tourner.
settings-per-mode-offsets-radio-whose = Décalages par mode, pour une radio dont la F.I. se déplace avec le mode. Un mode laissé vide utilise le décalage ci-dessus.
settings-computer = Sur cet ordinateur
settings-add-radio-computer = Ajouter une radio sur cet ordinateur
settings-radio-s-settings-are-below = Les réglages de cette radio sont ci-dessous. Faites glisser pour la déplacer le long de la barre.
settings-switch-radio-dialog-follows-drag = Passer à cette radio (la boîte suit). Faites glisser pour la déplacer le long de la barre.
settings-tx = ● TX
settings-mute-radio-s-audio = Couper l'audio de cette radio
settings-close-radio-configuration-kept = Fermer cette radio (sa configuration est conservée)
settings-close-tab-close-radio-station = Fermer cet onglet, ou fermer la radio sur sa station
settings-close-tab = Fermer cet onglet
settings-transverters-full = { $max } est le maximum que ce tableau peut contenir
settings-dialling = Composition de { $url }…
settings-region-note = L'endroit où se trouve la station. Règle tous les plans de bande : les limites des bandes, les sous-segments CW / données / phonie de la barre de la cascade, l'endroit où écoutent les skimmers, les fréquences d'appel proposées, et ce qui compte comme hors bande à l'émission. En région { $region }, cela donne pour le 70 cm { $region ->
        [1] 430–440 MHz — donc 446 est hors bande, et le 40 m s'arrête à 7.200
        [2] 420–450 MHz, et le 40 m va jusqu'à 7.300
       *[3] 430–450 MHz, et le 80 m s'arrête à 3.900
    }. Prend effet immédiatement, et s'applique à chaque radio de cette station.
settings-rx-grid-reported = Les réceptions de cette radio sont transmises depuis { $grid }, et non depuis votre propre locator.
settings-rx-grid-invalid = { $grid } n'est pas un locator Maidenhead, donc rien de ce qu'entend cette radio n'est transmis. Quatre ou six caractères : deux lettres, deux chiffres, et éventuellement deux lettres de plus.
settings-gateway-via = { " " }via { $via }
settings-auto-upload-to = Envoyer automatiquement chaque nouveau QSO à { $service }
settings-tle-updated = Mis à jour : { $n } { $n ->
        [one] abonnement
       *[other] abonnements
    }, { $total } satellites.
settings-tle-updated-failed = Mis à jour : { $ok } sur { $n } ; { $failed } en échec — voir les lignes ci-dessus.
settings-panadapter-of = Cette radio est le panoramique de { $radio }
settings-radio-n = radio { $n }
settings-on-station = Sur { $station }
settings-add-radio-on-station = Ajouter une radio sur { $station } — la station la sert immédiatement
settings-panadapter-receiver-for = Récepteur du panoramique pour { $radio }
settings-hang-up-stays-on = Raccrocher. La radio reste sur { $station }.
settings-close-on-station = Fermer { $radio } sur { $station } ? Sa configuration reste sur cette machine ; l'onglet ici se ferme avec elle, et aussi celui de tous les autres.
settings-close-it-on = La fermer sur { $station }
settings-locator-hint = locator

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = Alertes sonores
settings-alerts-sound-alarm-when-decode-matters = Déclencher une alarme lorsqu'un décodage est important
settings-alerts-plays-over-own-audio-output = Joue sur sa propre sortie audio, pour être entendue même lorsque la bande sort par un autre haut-parleur que cet écran.
settings-alerts-volume = Volume
settings-alerts-output = Sortie
settings-alerts-test = Tester
settings-alerts-what-sound = Ce qui doit sonner
settings-alerts-each-station-quiet-while-after = Chaque station reste silencieuse un moment après une alerte, pour qu'une bande chargée ne fasse pas sonner chaque créneau. Voix et Ton + voix sont lus par la voix des annonces parlées — activez-la dans Réglages → Interface.

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = Amplificateur
settings-amp-waiting-station-s-amplifier-configuration = En attente de la configuration de l'amplificateur de la station…
settings-amp-hobbypcb-hardrock-500-hardrock-50 = Un HobbyPCB Hardrock-500 ou Hardrock-50 sur son port série USB. SDRoxide lit ses indicateurs, commute sa bande, son mode et son antenne, et pilote son coupleur depuis la fenêtre AMP. L'amplificateur est toujours commandé par la ligne PTT de la radio, comme auparavant.
settings-amp-enable = Activer
settings-amp-model = Modèle
settings-amp-untested-panel-has-only-been = Non testé : ce panneau n'a été essayé qu'avec un Hardrock-500. Les limites des indicateurs pour ce modèle sont mises à l'échelle de celles du Hardrock-500 et peuvent être fausses. Surveillez l'affichage de l'amplificateur lui-même.
settings-amp-port = Port
settings-amp-choose = — choisir —
settings-amp-baud = Débit
settings-amp-must-match-amplifier-s-menu = Doit correspondre au menu de l'amplificateur. Le panneau de commande de HobbyPCB demande 115200.
settings-amp-follow-radio-s-band = Suivre la bande de la radio
settings-amp-sends-transmit-frequency-amplifier-every = Envoie la fréquence d'émission à l'amplificateur à chaque changement de bande, pour que ses filtres suivent la radio sans câble CAT propre. Jamais envoyée pendant une émission.
settings-amp-show-power-swr-tx-meter = Afficher sa puissance et son ROS sur le wattmètre TX
settings-amp-only-where-radio-does-not = Seulement lorsque la radio ne les mesure pas elle-même, et seulement lorsque l'amplificateur est en ligne.
settings-amp-off = Désactivé
settings-amp-connecting = Connexion…
settings-amp-status = État
settings-amp-apply = {" "}APPLIQUER{" "}
settings-amp-save-re-open-amplifier-s = Enregistrer et (ré)ouvrir le port de l'amplificateur

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = Clavier
settings-controls-click-shortcut-rebind-then-press = Cliquez sur un raccourci pour le réaffecter, puis appuyez sur la combinaison de touches (Échap annule). Les raccourcis sont ignorés pendant que vous saisissez dans un champ de texte.
settings-controls-shortcut = Raccourci
settings-controls-does = Action
settings-controls-step-mode = Pas / mode
settings-controls-accel = Accél.
settings-controls-on = Activé
settings-controls-press-key = appuyez sur une touche…
settings-controls-remove-binding = Supprimer cette affectation
settings-controls-add-shortcut = + Ajouter un raccourci
settings-controls-restore-defaults = Rétablir les valeurs par défaut
settings-controls-bind-hold-talk-space = Affecter la parole maintenue à Espace
settings-controls-hold-space-transmit-releasing-losing = Maintenez Espace pour émettre ; le relâcher — ou perdre le focus de la fenêtre — arrête l'émission
settings-controls-unkey-held-ptt-after = Relâcher un PTT maintenu après
settings-controls-backstop-against-stuck-key-controller = Filet de sécurité contre une touche bloquée ou une manette qui cesse d'envoyer. 0 désactive.
settings-controls-panadapter-mouse = Souris sur le panoramique
settings-controls-wheel = Molette
settings-controls-wheel-shift = Molette + Maj
settings-controls-tune-step = Pas d'accord
settings-controls-zoom-rate = Vitesse de zoom
settings-controls-click-tune-rounding = Arrondi de l'accord au clic
settings-controls-invert-wheel-direction = Inverser le sens de la molette
settings-controls-left-drag-tunes-well-pans = Le glissement gauche accorde en plus de déplacer la vue
settings-controls-off-makes-left-drag-pan = Désactivé, le glissement gauche ne fait que déplacer la vue, comme le glissement droit.
settings-controls-scroll-digit-frequency-readout-tune = Faire défiler un chiffre de l'affichage de fréquence pour l'accorder
settings-controls-both-wheel-actions-are-tune = Les deux actions de la molette sont Accorder — il ne reste aucun moyen de zoomer.
settings-controls-restore-mouse-defaults = Rétablir les valeurs par défaut de la souris
settings-controls-mouse-buttons = Boutons de souris
settings-controls-left-right-buttons-are-reserved = Les boutons gauche et droit sont réservés à l'accord et au déplacement de la vue ; les boutons du milieu et les boutons supplémentaires sont libres. Un bouton latéral maintenu pour le PTT se comporte comme une pédale.
settings-controls-add-mouse-button = + Ajouter un bouton de souris
settings-controls-f1-always-opens-manual-even = F1 ouvre toujours ce manuel, même en pleine saisie, donc cette touche n'est pas réaffectable.
settings-controls-midi-controller = Contrôleur MIDI
settings-controls-midi-controllers-need-native-app = Les contrôleurs MIDI exigent l'application native — le client navigateur n'a pas d'accès MIDI.
settings-controls-any-class-compliant-midi-surface = Toute surface MIDI conforme à la classe fonctionne : la molette de jog d'une console DJ fait un bon bouton de VFO, ses pads commandent le PTT et les boutons de bande, ses curseurs commandent les gains.
settings-controls-enable = Activer
settings-controls-controller = Contrôleur
settings-controls-feedback = Retour vers
settings-controls-rescan-ports = Rechercher les ports
settings-controls-not-connected = Non connecté.
settings-controls-move-control-see-here = Déplacez une commande pour la voir ici.
settings-controls-control = Commande
settings-controls-reads = Lue comme
settings-controls-led = LED
settings-controls-move = la déplacer…
settings-controls-click-then-move-control-you = Cliquez, puis déplacez la commande que vous voulez affecter
settings-controls-speed-sensitivity-spin-faster-tune = Sensibilité de vitesse : tournez plus vite pour accorder plus vite
settings-controls-send-current-value-back-light = Renvoyer la valeur actuelle, pour allumer une LED ou déplacer un curseur
settings-controls-add-midi-control = + Ajouter une commande MIDI
settings-controls-clear-all = Tout effacer
settings-controls-endless-jog-encoders-send-relative = Les encodeurs sans fin (jog) envoient un pas relatif plutôt qu'une position, dans l'un de trois codages qui se ressemblent pour de petits mouvements. APPRENDRE devine d'après une rotation horaire ; si la commande accorde alors dans le mauvais sens, cochez « inv. ».
settings-controls-none = — aucun —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = Valeur système
settings-general-remote-access = Accès à distance
settings-general-what-remote-client-browser-page = Ce qu'un client distant — la page du navigateur, ou une autre instance de sdroxide lancée avec --connect — doit fournir avant que cette station ne le laisse piloter la radio. S'applique en mode serveur (--server) ; la prochaine connexion prend en compte un changement, sans redémarrage.
settings-general-username = Identifiant
settings-general-password = Mot de passe
settings-general-clients-must-give-password-leaving = Les clients doivent fournir le mot de passe. Laisser l'identifiant vide ne pose pas de problème.
settings-general-clients-must-sign = Les clients doivent se connecter.
settings-general-empty-anyone-who-can-reach = ⚠ Vide : toute personne pouvant joindre le port du serveur peut piloter cette radio, sous votre indicatif. Définissez un mot de passe avant de transférer le port.
settings-general-stored-clear-config-toml-like = Stocké en clair dans config.toml, comme les autres mots de passe que sdroxide conserve.
settings-general-receive-audio-gain = Gain audio de réception
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    Gain supplémentaire sur tout ce que cette radio envoie aux haut-parleurs, en plus de la commande de volume. Laissez 0 dB, sauf si la radio est faible à plein volume : le haut de la réglette de volume est l'audio tel qu'il arrive, si bien qu'elle peut baisser une radio mais jamais la monter, et la sortie USB audio de certains transceivers se situe bien en dessous de la pleine échelle.

    Montez par paliers de 6 dB. Trop fort écrête — l'audio est limité à la pleine échelle au lieu de déborder, donc en exagérer sonne dur plutôt que fort. Les enregistrements sont pris en amont de ce gain et ne sont pas affectés.
settings-general-reload-band-plan = RECHARGER LE PLAN DE BANDE
settings-general-re-read-bandplan-json-machine = Relire bandplan.json sur la machine à laquelle la radio est reliée et l'appliquer — limites de bandes, sous-segments et fenêtres des skimmers — sans redémarrer.
settings-general-running-station-s-own-band = Fonctionne avec le plan de bande propre à la station.
settings-general-running-built-iaru-tables = Fonctionne avec les tables UIR intégrées.
settings-general-settings-file = Fichier de réglages
settings-general-settings-are-machine-radio-attached = Les réglages se trouvent sur la machine à laquelle la radio est reliée. Exportez-les depuis là.
settings-general-export = EXPORTER…
settings-general-write-every-setting-station-radios = Écrit chaque réglage de cette station — les radios, les modes, les serveurs, les mémoires, le plan de bande — dans un fichier que vous pouvez emporter vers une autre installation. Votre journal et tout mot de passe de serveur enregistré restent ici.
settings-general-import = IMPORTER…
settings-general-replace-station-s-settings-ones = Remplace les réglages de cette station par ceux d'un fichier exporté d'une autre installation. Redémarrez sdroxide ensuite.
settings-general-import-overwrites-what-here-file = Un import écrase ce qui se trouve ici, fichier par fichier, et prend effet au prochain démarrage de sdroxide — sinon les réglages déjà en mémoire seraient réécrits par-dessus. Ce que le fichier ne mentionne pas reste tel quel, si bien qu'un ensemble venu d'une station à une radio ne supprime pas une seconde radio ici. Votre journal ne figure jamais dans le fichier : exportez-le en ADIF depuis la fenêtre LOG si vous voulez le déplacer aussi.
settings-general-sdroxide-settings = Réglages de sdroxide
settings-general-swr-guard = Protection ROS
settings-general-stop-transmitting-high-swr = Arrêter l'émission en cas de ROS élevé
settings-general-trip = Déclencher à
settings-general-stops-transmission-when-radio-reports =
    Interrompt l'émission lorsque la radio annonce un ROS égal ou supérieur à cette valeur, et bloque l'émission jusqu'à ce que vous l'acquittiez. Détecte une antenne débranchée, une ligne d'alimentation défaillante ou un commutateur laissé sur le mauvais port.

    L'accord est traité différemment, car alimenter une désadaptation est le but même : un coupleur d'antenne a le double de la limite et environ cinq secondes avant que la protection ne s'applique. Un coupleur manuel qui prend plus de temps que cela exige de désactiver la protection pour la session.

    Exige une radio qui annonce le ROS par CAT. Ignore le premier cinquième de seconde de chaque émission, et n'attend pas la forte puissance.
settings-general-per-mode-settings = Réglages par mode
settings-general-reset-every-mode = RÉINITIALISER TOUS LES MODES
settings-general-forget-every-mode-s-remembered = Oublie le CAG, le squelch, la réduction de bruit, le filtre coupe-bande et les commutateurs stéréo mémorisés pour chaque mode, et rétablit les valeurs par défaut propres à chaque mode.
settings-general-selecting-mode-lays-mode-s = Choisir un mode applique les valeurs de départ propres à ce mode pour le CAG, le squelch, la réduction de bruit, le filtre coupe-bande et les commutateurs stéréo du récepteur, et modifier une de ces valeurs la mémorise pour ce seul mode. La puce en forme de flèche circulaire, à la fin de la ligne filtre/bruit du récepteur, liste ce qui a été modifié et rétablit le mode courant.
settings-general-channels = Canaux

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = Opérateur
settings-net-callsign-grid-are-set-general = L'indicatif et le carré se règlent dans l'onglet Général.
settings-net-set-your-callsign-grid-general = ⚠ Réglez votre indicatif et votre carré dans l'onglet Général.
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = Activer
settings-net-connects-whenever-enabled-your-station = Se connecte dès qu'il est activé. Votre station n'est visible des autres que lorsque la radio est en RADE — dans tout autre mode vous restez connecté mais masqué.
settings-net-station = Station
settings-net-message = Message
settings-net-receive-only-i-cannot-transmit = Réception seule (je ne peux pas émettre)
settings-net-server = Serveur
settings-net-host = Hôte
settings-net-port = Port
settings-net-tls-wss = TLS (wss://)
settings-net-not-yet-implemented-freedv-gui = Pas encore implémenté — l'interface graphique FreeDV utilise aussi le ws:// en clair.
settings-net-reporting = Rapports
settings-net-report-stations-i-decode = Transmettre les stations que je décode
settings-net-sends-rx-report-each-callsign = Envoie un rx_report pour chaque indicatif récupéré d'une trame de fin d'émission RADE.
settings-net-show-other-reporter-stations-spots = Afficher les autres stations de reporter comme spots
settings-net-adds-them-panadapter-overlay-world = Les ajoute à la superposition du panoramique, à la carte du monde et à la fenêtre SPOTS sous le filtre FREEDV.
settings-net-set-your-callsign-grid-general-2 = ⚠ Réglez votre indicatif et votre carré dans l'onglet Général. Sans les deux, la connexion est en lecture seule : vous voyez les autres stations mais n'apparaissez pas vous-même.
settings-net-apply = {" "}APPLIQUER{" "}
settings-net-persist-re-connect = Enregistrer et (re)connecter
settings-net-season = Saison
settings-net-downloading = téléchargement…
settings-net-using-built-copy = utilisation de la copie intégrée
settings-net-your-own-stations-corrections-go = Vos propres stations et corrections vont dans ce fichier. sdroxide ne l'écrit jamais, et une entrée ici remplace une station planifiée de même nom et même fréquence.
settings-net-reload = Recharger
settings-net-re-read-your-station-file = Relire votre fichier de stations après l'avoir modifié
settings-net-broadcast-stations-labelled-waterfall-come = Les stations de radiodiffusion étiquetées sur la cascade viennent de la liste intégrée à cette version. Les modifier exige l'application de bureau.
settings-net-ask-service-whether-these-credentials = Demander au service si ces identifiants fonctionnent. Applique d'abord les réglages ci-dessus, et n'enregistre rien : il ne fait que lire.
settings-net-checking = vérification…

## Settings window: messages with values filled in.

settings-amp-connected-to = Connecté au { $model }
settings-controls-last-midi = Dernier message : { $message }  valeur { $value }
settings-general-band-plan-file = Chaque limite de bande et chaque sous-segment viennent de { $path }, écrit à partir des tables UIR intégrées la première fois, et qui vous appartient à modifier ensuite — resserrez une bande à votre licence et sdroxide refusera d'émettre en dehors. Les fréquences sont en MHz ; supprimez le fichier pour obtenir une copie neuve des valeurs par défaut. Il s'agit de l'attribution régionale, pas de votre licence : vos propres conditions peuvent être plus étroites, et les plans nationaux diffèrent à l'intérieur d'une région.
settings-general-exported = { $note } exporté — choisissez où l'enregistrer.
settings-general-export-failed = Échec de l'export : { $error }
settings-general-import-failed = Échec de l'import : { $error }
settings-general-imported = { $summary } — redémarrez sdroxide pour les utiliser.
settings-general-import-skipped = { $path } ignoré : { $why }
settings-general-swr-tripped = ⚠ Déclenché actuellement à { $swr }:1 — l'émission est bloquée.
settings-net-call-grid-set-on-general = { $call } / { $grid } — réglé dans l'onglet Général
settings-net-reporting-as = Transmission sous { $call } / { $grid } — SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ Échec du téléchargement : { $error }. Utilisation du planning intégré.
settings-net-test-service = Tester { $service }

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = L'émission est désactivée pendant qu'un convertisseur est défini — indiquez ce qui se trouve sur la ligne d'émission ci-dessus pour la rétablir.
settings-transmit-goes-through-same-converter = L'émission passe par le même convertisseur, donc la radio travaille avec le même décalage hors du cadran dans les deux sens.
settings-transmit-not-converted-radio-transmits = L'émission n'est pas convertie : la radio émet sur la fréquence qu'affiche le cadran, tandis que la réception passe par le convertisseur.
settings-transmit-takes-own-offset-separate = L'émission prend son propre décalage, distinct de celui de la réception.
settings-no-enabled-subscriptions-update = Aucun abonnement activé à mettre à jour.
settings-add-radio-here-station-you = Ajouter une radio — ici, ou sur une station à laquelle vous êtes connecté

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = Chaque limite de bande et chaque sous-segment viennent de bandplan.json sur la machine à laquelle la radio est reliée. Il s'agit de l'attribution régionale, pas de votre licence : vos propres conditions peuvent être plus étroites, et les plans nationaux diffèrent à l'intérieur d'une région.

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = Images reçues → FTP
settings-picup-uploads-every-received-sstv-picture = Envoie chaque image SSTV reçue (et, si cochée, chaque carte météo) dès son enregistrement, mise en page comme le fait QSSTV, pour que les pages web écrites pour QSSTV fonctionnent sans changement.
settings-picup-upload-received-pictures = Envoyer les images reçues
settings-picup-server = Serveur
settings-picup-port = Port
settings-picup-security = Sécurité
settings-picup-user = Utilisateur
settings-picup-password = Mot de passe
settings-picup-certificate = Certificat
settings-picup-trusted-when-computer-s-operating = Approuvé lorsque le système d'exploitation de cet ordinateur lui fait confiance, comme le ferait un navigateur ici. Une AC privée peut être ajoutée ci-dessous (PEM).
settings-picup-extra-ca = AC supplémentaire
settings-picup-only-certificate-fingerprint-accepted-way = Seul le certificat portant cette empreinte est accepté — la façon de faire confiance à un serveur auto-signé. TESTER affiche celle du serveur.
settings-picup-any-certificate-accepted-link-encrypted = ⚠ Tout certificat est accepté. La liaison est chiffrée, mais quiconque se trouve sur le trajet peut se faire passer pour le serveur et lire le mot de passe.
settings-picup-plain-ftp-sends-password-pictures = ⚠ Le FTP en clair envoie le mot de passe et les images sans chiffrement.
settings-picup-sstv = SSTV
settings-picup-sstv-dir = Dossier SSTV
settings-picup-weather-fax = Fax météo
settings-picup-wefax-dir = Dossier WEFAX
settings-picup-empty-login-directory = Vide : le répertoire de connexion.
settings-picup-file-names = Noms de fichiers
settings-picup-format = Format
settings-picup-add-extension = ajouter l'extension
settings-picup-image1-png-rather-than-qsstv = image1.png plutôt que le image1 nu de QSSTV
settings-picup-caption-bar = Bandeau de légende
settings-picup-time-frequency-mode-sender-s = L'heure, la fréquence, le mode et l'identifiant FSK de l'émetteur en haut, comme le dessine QSSTV. Seule la copie envoyée est marquée.
settings-picup-fit-960-768 = Ajuster à 960 × 768
settings-picup-qsstv-s-heartbeat-tiny-picture = Le battement de QSSTV : une minuscule image dont la hauteur est la minute, envoyée à côté de chaque image, et rendue large de deux pixels à la fermeture du programme.
settings-picup-nat-workaround = Contournement NAT
settings-picup-ignore-private-address-server-s = Ignore une adresse privée dans la réponse du serveur en mode passif et utilise l'adresse propre du serveur — pour un serveur derrière un NAT qui ne connaît pas son adresse publique.
settings-picup-apply = {" "}APPLIQUER{" "}
settings-picup-test = TESTER
settings-picup-connect-settings-above-applied-not = Se connecte avec les réglages ci-dessus — appliqués ou non — se connecte, et écrit puis supprime un petit fichier dans chaque dossier d'envoi.
settings-picup-checking = vérification…
settings-picup-trust-certificate = Faire confiance à ce certificat
settings-picup-pin-switch-pinned-fingerprint-one = Épinglez-le : passez sur « Empreinte épinglée » avec celle-ci. Vérifiez-la d'abord auprès du serveur — c'est ce que le serveur a annoncé, pas une preuve de son identité. Puis APPLIQUER.
settings-picup-last-upload = Dernier envoi

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = Profils : les configurations enregistrées de la station
settings-profiles-profile-captures-whole-working-setup = Un profil capture toute la configuration de travail — les cadrans et VFO, le mode et les filtres, les gains, l'excitation et les antennes, votre indicatif et vos modèles de messages, ainsi que les registres par bande — et la réapplique en un clic. Le matériel (pilote, périphériques audio, convertisseurs) est volontairement laissé de côté : un profil décrit la façon dont vous travaillez la station, non la radio qu'elle utilise.
settings-profiles-save-current-setup = Enregistrer la configuration actuelle
settings-profiles-profile-name = nom du profil
settings-profiles-save = Enregistrer
settings-profiles-applies-whatever-sdroxide-attached-dials = S'applique à ce à quoi sdroxide est relié : cadrans et VFO, mode et filtres, niveaux, gains, excitation, antennes, identité numérique et piles de bandes.
settings-profiles-no-profiles-saved-yet-name = Aucun profil enregistré pour l'instant — nommez-en un ci-dessus et appuyez sur Enregistrer.
settings-profiles-saved-profiles = Profils enregistrés
settings-profiles-apply = Appliquer

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = Commutateur émission/réception
settings-relay-waiting-station-s-t-r = En attente de la configuration du commutateur E/R de la station…
settings-relay-closes-contact-while-station-transmits = Ferme un contact pendant que cette station émet — pour déconnecter et mettre à la masse l'entrée antenne du SDR, et pour commander en séquence un amplificateur ou un relais E/R externe. Fonctionne avec les cartes à relais USB bon marché (LCUS, KMtronic, Numato), avec une ligne série RTS/DTR vers toute interface qui attend une fermeture PTT, et avec tout autre dispositif via un crochet de commande.
settings-relay-hardware = Matériel
settings-relay-board = Carte
settings-relay-lcus-four-byte-a0-command = LCUS est la carte à commande A0 de quatre octets vendue sous une douzaine de noms ; KMtronic et Numato peuvent aussi être interrogés sur leur configuration
settings-relay-port = Port
settings-relay-choose = — choisir —
settings-relay-device = Périphérique
settings-relay-gpio-pin = Broche GPIO
settings-relay-pin-3-every-homebrew-plan = Broche 3 sur tous les plans maison et toutes les cartes du commerce — c'est celle, en bout de boîtier, à laquelle on peut souder un fil. Si le relais ne claque pas, la carte est un clone qui n'implémente pas du tout ces broches.
settings-relay-gpio-chip = Puce GPIO
settings-relay-linux-only-gpiodetect-lists-chips = Linux uniquement. `gpiodetect` liste les puces de la machine.
settings-relay-transmit = À l'émission
settings-relay-receive = À la réception
settings-relay-nothing-switched = Rien n'est commuté.
settings-relay-starting-program-costs-unpredictable-amount = Lancer un programme prend un temps imprévisible, et rien de ce temps ne compte dans l'avance — donc prévoyez ici une marge plus large que pour une carte à relais. Les nombres du tableau des contacts ne veulent rien dire pour cette liaison : vos commandes sont exécutées une fois à l'appui sur la touche et une fois à son relâchement.
settings-relay-contacts = Contacts
settings-relay-contact-1-rts-contact-2 = Le contact 1 est RTS et le contact 2 est DTR.
settings-relay-no = N°.
settings-relay-gpio = GPIO
settings-relay-name = Nom
settings-relay-job = Fonction
settings-relay-tx-closes = Émission ferme
settings-relay-lead = Avance
settings-relay-hold = Maintien
settings-relay-line-s-offset-chip-bcm = Le décalage de la ligne sur cette puce — le numéro BCM sur le connecteur d'un Raspberry Pi, qui n'est pas le numéro de broche physique
settings-relay-sdr-antenna = Antenne SDR
settings-relay-decides-default-timings-what-log = Détermine les temporisations par défaut et le nom que lui donne le journal — et, pour un décodeur de bande, que le contact suit le tableau des bandes ci-dessous
settings-relay-transmitting-energises-coil-off-transmitting =
    Activé : émettre alimente la bobine. Désactivé : émettre la relâche.

    C'est d'abord un choix de câblage avant d'être un réglage — voir la note ci-dessous.
settings-relay-how-long-before-rf-contact = Combien de temps avant la HF ce contact passe du mot RX de sa bande au mot TX. Un changement d'accord le déplace aussitôt ; ceci concerne l'appui sur la touche, lorsqu'un filtre réservé à l'émission doit être en circuit avant que la puissance ne le soit.
settings-relay-closed-long-before-rf-small = Fermé ce temps avant la HF. Un petit relais coaxial commute en 5 à 15 ms. L'émission attend le plus long de ces délais, donc une grande valeur crée un blanc audible dans l'audio de réception ; au-delà de 250 ms, la valeur est ignorée.
settings-relay-how-long-after-rf-stops = Combien de temps après l'arrêt de la HF ce contact revient du mot TX à son mot RX.
settings-relay-opened-long-after-rf-stops = Ouvert ce temps après l'arrêt de la HF. Plus long que l'avance, délibérément : rendre l'antenne un peu en retard ne coûte rien, la rendre trop tôt coûte un étage d'entrée. Zéro est une réponse valable, et la bonne pour la ligne de commande d'un amplificateur.
settings-relay-test = {" "}TESTER{" "}
settings-relay-not-while-station-air = Pas pendant que la station est en ondes
settings-relay-close-contact-half-second-so = Fermer ce contact pendant une demi-seconde, pour entendre le relais et vérifier le câblage avec l'émetteur froid
settings-relay-remove = Supprimer
settings-relay-add-contact = + ajouter un contact
settings-relay-if-will-not-answer = S'il ne répond pas
settings-relay-switch-exists-protect-receiver-not = Un commutateur destiné à protéger un récepteur ne vaut rien s'il cesse silencieusement de le protéger — donc refuser est le comportement par défaut. Choisissez l'autre option lorsqu'un câble USB détaché qui met fin à votre concours est la pire issue.
settings-relay-order-timings-longest-lead-closes = L'ordre est celui des temporisations : la plus longue avance se ferme en premier, et le plus long maintien s'ouvre en dernier. Donnez donc au relais d'antenne l'avance et le maintien les plus longs, et à l'amplificateur les plus courts de chacun — ainsi l'antenne est toujours commutée avant que l'amplificateur ne soit commandé, et l'amplificateur se désactive toujours avant que l'antenne ne revienne.
settings-relay-transmit-sense-input = Entrée de détection d'émission
settings-relay-when-transceiver-keys-itself-own = Lorsqu'un transceiver s'enclenche lui-même — son propre bouton de micro, sa pédale, son VOX ou son manipulateur — sdroxide s'en aperçoit en le demandant par CAT, et cette réponse n'arrive que quelques centaines de millisecondes après le début du trafic. Si vous reliez la ligne SEND de l'appareil, par un opto-isolateur, à une entrée de handshake de ce même port, il est vu en quelques millisecondes. Le wattmètre et le verrouillage d'émission gagnent en rapidité.
settings-relay-line = Ligne
settings-relay-transmitting = L'émission correspond à
settings-relay-high-line = une ligne haute
settings-relay-most-opto-isolated-interfaces-pull = La plupart des interfaces opto-isolées tirent la ligne vers le bas lorsque l'appareil émet, donc c'est généralement désactivé
settings-relay-belongs-radio = Appartient à la radio
settings-relay-which-radio-tab-sensed-transceiver = Quel onglet radio est le transceiver détecté. Sur une station avec une seule radio, c'est 1 et rien n'en dépend.
settings-relay-needs-serial-port-handshake-lines = Nécessite un port série — les lignes de handshake s'y trouvent.
settings-relay-wiring-only-fail-safe-survives = Le câblage est la seule sécurité qui survit à ce programme.
settings-relay-choose-tx-closes-so-de = Choisissez « Émission ferme » pour que le contact *désénergisé* soit l'état voulu lorsque la carte est débranchée, que l'ordinateur est éteint ou que sdroxide ne tourne pas. Câblé de sorte que désénergisé signifie mis à la masse, un relais mort laisse le SDR sourd — gênant, mais sûr. Câblé à l'inverse, un relais mort plus une émission donnent un étage d'entrée mort. Avec un vrai amplificateur derrière, prenez la première option.
settings-relay-cannot-protect-first-instant-over = Cela ne peut pas protéger le tout premier instant d'un trafic que vous lancez à la radio. Si vous commandez l'appareil depuis sdroxide, les contacts précèdent toujours la HF ; si vous câblez l'entrée de détection ci-dessus, ils la suivent en quelques millisecondes. Pour un étage d'entrée réellement digne d'être protégé, ajoutez aussi un commutateur E/R matériel détecté par la HF — aucun programme sur un ordinateur ne peut être la solution complète.
settings-relay-contacts-transmit = contacts en ÉMISSION
settings-relay-not-open = Non ouvert.
settings-relay-status-unknown-press-apply = État inconnu — appuyez sur APPLIQUER.
settings-relay-station-air-apply-will-take = La station est en ondes — APPLIQUER prendra effet après ce trafic.
settings-relay-band-decoder-outputs = Sorties du décodeur de bande
settings-relay-one-control-word-per-band = Un mot de commande par bande : le bit 0 correspond au contact 1, le bit 1 au contact 2, et ainsi de suite, selon la numérotation ci-dessus. RX suit la bande du cadran de réception ; TX la bande de la radio qui a déclenché l'émission. Ils sont commutés sur l'avance de chaque contact avant la HF et de nouveau après son maintien — donnez-leur la même valeur pour un filtre qui se moque du sens de la HF, et des valeurs différentes pour un contournement de préampli en réception seule ou un banc de filtres passe-bas réservé à l'émission. Les bandes laissées à 0 n'activent rien. S'applique à Appliquer / reconnecter.
settings-relay-band = Bande
settings-relay-rx = RX
settings-relay-tx = TX
settings-relay-outputs-asserted = Sorties activées
settings-relay-other = Autre
settings-relay-everywhere-outside-amateur-bands-short = Partout hors des bandes amateur — écoute en ondes courtes, et tout ce sur quoi tombe le cadran d'un transverteur et qu'aucune bande ne couvre.
settings-relay-apply = {" "}APPLIQUER{" "}
settings-relay-persist-re-open-switch = Enregistrer et (ré)ouvrir le commutateur

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = Se connecter à un serveur
settings-remote-drive-sdroxide-running-somewhere-else = Piloter un sdroxide fonctionnant ailleurs — la machine de la station, un site distant — par le réseau. Lancez-le là-bas avec --server, et saisissez son adresse ici. La radio, les décodeurs et le journal restent sur cette machine ; cet écran reçoit la cascade et l'audio, et ce sont ses haut-parleurs et son microphone que vous utilisez.
settings-remote-address = Adresse
settings-remote-host-name-ip = nom d'hôte ou IP
settings-remote-machine-running-sdroxide-server-host = La machine qui exécute sdroxide --server : un nom d'hôte, une adresse IPv4 ou IPv6, ou une URL ws:// (ou wss://) complète si le serveur se trouve derrière un proxy inverse — une URL est utilisée exactement comme saisie et ignore le port et le commutateur ci-dessous.
settings-remote-port = Port
settings-remote-port-server-listens-server-port = Le port sur lequel écoute ce serveur — server_port dans son config.toml. 4950 sauf s'il a reçu --port.
settings-remote-secure = Sécurisé
settings-remote-wss-tls = wss:// (TLS)
settings-remote-tick-where-server-sits-behind = Cochez ceci lorsque le serveur se trouve derrière un élément qui termine le HTTPS pour lui — un proxy inverse sur le port 443, qui est la façon habituelle d'atteindre une station sur Internet ouvert. Le serveur propre à sdroxide parle en ws:// simple et n'en veut pas.
settings-remote-connect = {" "}CONNECTER{" "}
settings-remote-open-station-radio-tab-own = Ouvrir cette station dans un onglet radio à part
settings-remote-client-cannot-hold-second-connection = Ce client ne peut pas tenir une seconde connexion.

    Si le serveur demande un identifiant et un mot de passe, son écran de connexion s'affiche dans le nouvel onglet. Un lien ws:// simple ne transporte rien de cela chiffré : sur Internet ouvert, faites passer la liaison par un VPN ou un tunnel SSH, ou terminez le HTTPS devant le serveur et cochez Sécurisé ci-dessus.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = Diffusion UDP WSJT-X
settings-servers-waiting-station-s-wsjt-x = En attente de la configuration de diffusion WSJT-X de la station…
settings-servers-sends-decodes-station-status-logged = Envoie les décodages, l'état de la station et les QSO journalisés comme le fait WSJT-X, pour que GridTracker, JTAlert, N1MM+ et Log4OM fonctionnent avec sdroxide sans changement. Sortie seulement — rien sur ce socket ne peut toucher la radio.
settings-servers-enable = Activer
settings-servers-send = Envoyer à
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 atteint les clients de cette machine ; une adresse du réseau local ou un groupe de multidiffusion (224.0.0.1) atteint les autres
settings-servers-port = Port
settings-servers-t-2237-port-every-client-defaults = 2237 est le port que chaque client utilise par défaut
settings-servers-identify = S'identifier comme
settings-servers-name-clients-see-some-loggers = Le nom que voient les clients. Certains logiciels de journal n'acceptent que les données qui se déclarent WSJT-X.
settings-servers-n1mm-contactinfo-broadcast = Diffusion contactinfo N1MM+
settings-servers-same-news-n1mm-s-words = Les mêmes informations dans le langage de N1MM : un datagramme XML par contact journalisé, pour les logiciels qui écoutent N1MM plutôt que WSJT-X — le pont de la World Radio League accepte l'un ou l'autre. Contacts uniquement ; ce protocole ne comporte pas de flux de décodages. Il a son propre commutateur et son propre port, car une station peut vouloir les deux.
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 atteint les clients de cette machine. Le conseil de N1MM pour un réseau de concours est l'adresse de diffusion de ce sous-réseau — 192.168.1.255 pour un réseau 192.168.1.n — qui atteint toutes les positions à la fois.
settings-servers-t-12060-port-n1mm-s-documentation = 12060 est le port recommandé par la documentation de N1MM
settings-servers-station-name = Nom de la station
settings-servers-what-n1mm-calls-stationname-name = Ce que N1MM appelle StationName : le nom de l'ordinateur qui a envoyé le paquet. Les logiciels de journal l'affichent pour distinguer un poste opérateur d'un autre.
settings-servers-cw-panel-s-copy-plain = La copie du panneau CW en texte brut, un datagramme UDP par morceau dès qu'il est stabilisé — pour un contrôleur de station, la fenêtre CW d'un logiciel de journal ou un script. Pas de protocole : un écouteur ajoute simplement ce qui arrive (essayez nc -ul 9999). Seul le texte stabilisé est envoyé, jamais la queue que le décodeur révise encore. Chaque radio a son propre commutateur.
settings-servers-apply = {" "}APPLIQUER{" "}
settings-servers-persist-re-open-broadcast-sockets = Enregistrer et (ré)ouvrir les sockets de diffusion
settings-servers-hamlib-rigctld-server = Serveur rigctld Hamlib
settings-servers-waiting-station-s-rigctld-configuration = En attente de la configuration rigctld de la station…
settings-servers-listen = Écouter sur
settings-servers-t-127-0-0-1-machine = 127.0.0.1 (cette machine)
settings-servers-t-0-0-0-0-whole = 0.0.0.0 (tout le réseau)
settings-servers-t-4532-port-every-rigctld-client = 4532 est le port que suppose chaque client rigctld
settings-servers-rig-name = Nom de la radio
settings-servers-reported-clients = annoncé aux clients
settings-servers-max-clients = Nombre max de clients
settings-servers-allow-clients-transmit = Autoriser les clients à émettre
settings-servers-off-refuses-every-key-request = Désactivé refuse chaque demande de manipulation et cesse d'annoncer une plage d'émission, si bien que Hamlib lui-même refuse de manipuler.
settings-servers-rigctld-has-no-authentication-0 = ⚠ rigctld n'a aucune authentification. Sur 0.0.0.0, quiconque peut atteindre ce port peut accorder et piloter votre émetteur.
settings-servers-t-1-client = 1 client
settings-servers-not-running = Non lancé.
settings-servers-status-unknown-press-apply = État inconnu — appuyez sur APPLIQUER.
settings-servers-persist-re-bind-server = Enregistrer et (ré)lier le serveur
settings-servers-lets-any-hamlib-capable-program = Permet à tout programme compatible Hamlib de piloter cette radio : fréquence, mode, PTT, split, RIT et puissance. Dans WSJT-X, fldigi ou CQRLOG, choisissez la radio « Hamlib NET rigctl » (modèle 2) et indiquez cette adresse ; dans GPredict et N1MM, saisissez directement l'hôte et le port. Contrairement au serveur TCI, il ne transporte que la commande — ni audio, ni IQ.
settings-servers-antenna-rotator = Rotateur d'antenne
settings-servers-waiting-station-s-rotator-configuration = En attente de la configuration du rotateur de la station…
settings-servers-host = Hôte
settings-servers-min-elevation = Élévation min
settings-servers-below-rotator-parks-instead-tracking = En dessous de cette valeur, le rotateur se met en position de repos au lieu de suivre — réglez-la sur votre horizon ou votre ligne de toit local
settings-servers-azimuth-offset = Décalage d'azimut
settings-servers-added-every-command-rotator-whose = Ajouté à chaque commande, pour un rotateur dont le nord est décalé
settings-servers-min-movement = Mouvement min
settings-servers-steps-smaller-than-are-not = Les pas plus petits que cette valeur ne sont pas envoyés — poursuivre des dixièmes de degré use les moteurs pour rien
settings-servers-park-position = Position de repos
settings-servers-drive-position-when-idle = se placer à une position à l'arrêt
settings-servers-off-leaves-antenna-wherever-last = Désactivé laisse l'antenne là où a fini la dernière passe
settings-servers-not-connected = Non connecté.
settings-servers-waiting-station-s-tci-server = En attente de la configuration du serveur TCI de la station…
settings-servers-built-tci-server = Serveur TCI intégré
settings-servers-device-name = Nom du périphérique
settings-servers-off-leaves-control-receive-streams = Désactivé laisse la commande et les flux de réception fonctionner, mais chaque demande de manipulation est refusée.
settings-servers-tci-has-no-authentication-0 = ⚠ TCI n'a aucune authentification. Sur 0.0.0.0, quiconque peut atteindre ce port peut accorder et piloter votre émetteur.

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = Satellites : jeux d'éléments et fréquences
settings-tle-waiting-station-s-satellite-configuration = En attente de la configuration satellite de la station…
settings-tle-tracker-already-fetches-celestrak-s = Le suiveur récupère déjà seul le groupe amateur de CelesTrak. Ceci sert au reste : les satellites météo NOAA, un cubesat trop récent pour figurer dans le groupe, ou un jeu d'éléments plus frais que celui reçu. Les listes sont récupérées — et conservées — là où tourne le moteur radio, donc c'est le même ensemble de satellites partout où l'application est ouverte.
settings-tle-subscriptions = Abonnements
settings-tle-listings-fetched-kept-current-same = Listes récupérées et tenues à jour, au même rythme de six heures que le jeu amateur. Rafraîchies pendant que la fenêtre solaire est ouverte, et par METTRE À JOUR MAINTENANT ici.
settings-tle-fetch-track-listing = Récupérer et suivre cette liste
settings-tle-remove-subscription = Supprimer cet abonnement
settings-tle-orbits = Orbites
settings-tle-which-satellites-listing-get-orbit = Quels satellites de cette liste reçoivent un anneau d'orbite et une étiquette. Un groupe entier veut « sélection » : quatre-vingt-dix anneaux d'un coup sont illisibles, et aucun laisse quatre-vingt-dix points anonymes.
settings-tle-nothing-listing-sdroxide-s-curated = Rien dans cette liste ne figure dans la sélection de sdroxide — cette sélection compte dix satellites amateurs, donc cela se comporterait exactement comme « aucun ».
settings-tle-all-satellites = tous les satellites
settings-tle-catalogue-numbers-keep-comma-separated = Numéros de catalogue à conserver, séparés par des virgules. Vide suit tout ce que contient la liste.
settings-tle-not-fetched-yet = pas encore récupéré
settings-tle-subscription = + Abonnement
settings-tle-update-now = {" "}METTRE À JOUR MAINTENANT{" "}
settings-tle-ask-radio-engine-fetch-every = Demander au moteur radio de récupérer maintenant chaque abonnement activé
settings-tle-celestrak-groups = Groupes CelesTrak
settings-tle-pasted-element-sets = Jeux d'éléments collés
settings-tle-one-off-these-do-not = Pour une fois. Ceux-ci ne se mettent pas à jour seuls, et SGP4 cesse de propager un jeu d'éléments une quinzaine de jours après son époque — abonnez-vous plutôt pour tout ce que vous comptez garder.
settings-tle-track-one = Suivre celui-ci
settings-tle-show-two-element-lines = Afficher les deux lignes d'éléments
settings-tle-remove = Supprimer
settings-tle-paste-two-three-line-element = Collez ici des jeux d'éléments à deux ou trois lignes
settings-tle-add-pasted = + Ajouter le collé
settings-tle-nothing-paste-box-looked-like = Rien dans la zone de collage ne ressemblait à un jeu d'éléments.
settings-tle-clear-box = Vider la zone
settings-tle-frequencies = Fréquences
settings-tle-shown-under-pass-table-solar = Affichées sous le tableau des passages dans la fenêtre solaire. Une entrée ici remplace purement et simplement celle intégrée pour ce numéro de catalogue, donc partez d'une copie de celle-ci sauf si vous voulez abandonner le reste.
settings-tle-remove-satellite-s-entry = Supprimer l'entrée de ce satellite
settings-tle-link = LIAISON
settings-tle-downlink = DESCENDANTE
settings-tle-uplink = MONTANTE
settings-tle-mode = MODE
settings-tle-note = NOTE
settings-tle-fm-repeater = Relais FM
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67,0 Hz
settings-tle-link-2 = + Liaison
settings-tle-copy-built = Copier l'intégré
settings-tle-frequency-either-one-number-145 = Une fréquence est soit un seul nombre (145.800), soit une bande passante de répéteur écrite 145.950-145.970. Laissez une direction vide pour une balise.
settings-tle-norad = NORAD
settings-tle-satellite = + Satellite
settings-tle-frequency-entry-needs-satellite-s = Une entrée de fréquence exige le numéro de catalogue NORAD du satellite.

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = Adresse
settings-wavelog-api-key = Clé d'API
settings-wavelog-your-wavelog-s-front-page = L'adresse de la page d'accueil de votre Wavelog — https://log.example.org ou http://192.168.1.10/wavelog — et une clé issue de sa page API. L'envoi exige une clé en lecture/écriture.
settings-wavelog-plain-http-address-internet-api = http:// simple vers une adresse sur Internet : la clé d'API voyage sans chiffrement. Acceptable dans votre propre réseau ; utilisez https:// sur Internet.
settings-wavelog-trusted-cert = Certificat approuvé
settings-wavelog-forget-certificate = Oublier ce certificat
settings-wavelog-go-back-ordinary-checking-against = Revenir à la vérification ordinaire auprès des autorités de certification publiques — pour un serveur qui possède désormais un vrai certificat.
settings-wavelog-connection-check-runs-machine-radio = La vérification de connexion s'exécute sur la machine à laquelle la radio est reliée — utilisez les réglages de là-bas pour la tester et lister les emplacements de station.
settings-wavelog-server-presented-certificate-sha-256 = Le serveur a présenté ce certificat (SHA-256) :
settings-wavelog-compare-your-server-s-e = Comparez-la avec celle de votre serveur — par ex. `openssl x509 -noout -fingerprint -sha256 -in cert.pem` — et ne lui faites confiance que si elles correspondent. Désormais, ce certificat exact est accepté et tout autre est refusé.
settings-wavelog-trust-certificate = Faire confiance à ce certificat
settings-wavelog-station-location = Emplacement de station
settings-wavelog-none = — aucun —
settings-wavelog-press-test-above-list-station = Appuyez sur Tester ci-dessus pour lister les emplacements de station définis dans votre Wavelog.
settings-wavelog-default = Par défaut
settings-wavelog-where-qsos-go-when-operating = Où vont les QSO lorsque le profil d'exploitation ne nomme pas d'emplacement.
settings-wavelog-profile = Ce profil
settings-wavelog-use-default = Utiliser celui par défaut
settings-wavelog-not-set = non défini
settings-wavelog-no-station-location-chosen-uploads = Aucun emplacement de station choisi — les envois échoueront tant qu'il n'y en a pas un.
settings-wavelog-qsos-not-wavelog = QSO absents de Wavelog
settings-wavelog-every-qso-logbook-wavelog = Chaque QSO du journal est dans Wavelog.
settings-wavelog-everything-logbook-wavelog-has-not = Tout ce que Wavelog n'a pas accusé réception dans le journal — y compris les contacts antérieurs à l'envoi automatique. Ceux qu'il possède déjà reviennent comme doublons et sont simplement marqués comme envoyés.
settings-wavelog-set-address-key-station-location = Renseignez d'abord l'adresse, la clé et un emplacement de station.
settings-wavelog-yes-send-them = Oui, les envoyer
settings-wavelog-cancel = Annuler
settings-wavelog-download-into-logbook = Télécharger dans ce journal
settings-wavelog-downloading-runs-where-radio-attached = Le téléchargement s'exécute là où la radio est reliée, dans le journal de cette machine — configurez-le là-bas.
settings-wavelog-download-qsos-start-every = Télécharger les QSO au démarrage puis toutes les
settings-wavelog-downloaded-qsos-join-logbook-so = Les QSO téléchargés rejoignent le journal, pour que FT8 et les listes de spots sachent ce que vous avez travaillé depuis d'autres logiciels et radios. Chaque téléchargement ne récupère que les nouveautés.
settings-wavelog-press-test-above-choose-which = Appuyez sur Tester ci-dessus pour choisir les emplacements de station à télécharger.
settings-wavelog-sync-now = Synchroniser maintenant
settings-wavelog-fetch-what-wavelog-has-added = Récupère ce que Wavelog a ajouté depuis le dernier téléchargement. Applique d'abord les réglages ci-dessus.
settings-wavelog-full-resync = Resynchronisation complète…
settings-wavelog-download-ticked-locations-whole-compare = Télécharge en entier les emplacements cochés et les compare au journal : les QSO modifiés ou supprimés dans Wavelog, ou venant d'un emplacement qui n'est plus coché, sont remis en accord. Vous voyez les changements avant que quoi que ce soit ne se produise.
settings-wavelog-full-resync-changes-make = Resynchronisation complète — modifications à effectuer
settings-wavelog-qsos-logged-here-imported-file = Les QSO saisis ici ou importés depuis un fichier ne sont jamais modifiés ni supprimés — seuls ceux téléchargés depuis Wavelog le sont.
settings-wavelog-apply = Appliquer
settings-wavelog-discard = Abandonner
settings-wavelog-live-radio-status = État de la radio en direct
settings-wavelog-send-frequency-mode-wavelog-they = Envoie la fréquence et le mode à Wavelog à mesure qu'ils changent
settings-wavelog-radio-name = Nom de la radio
settings-wavelog-pick-radio-wavelog-s-live = Choisissez cette radio dans la saisie en direct de Wavelog et son formulaire suit le cadran. Exige une clé en lecture/écriture. Depuis la radio principale de la station uniquement.

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = Empreinte SHA-256 du certificat du serveur : { $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = Remet la station sur « { $name } » : cadrans, VFO, mode, filtres, gains, excitation, antennes, identité et piles de bandes.
settings-profiles-delete = Supprimer « { $name } »

## Settings window, T/R switch: messages with values

settings-relay-not-usable = Pas encore utilisable : { $why }
settings-relay-receiving = réception
settings-relay-rx-tx-words = { $rx } / TX { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n ->
        [one] { $n } client
       *[other] { $n } clients
    }
settings-servers-listening = ● En écoute sur { $addr } — { $clients }
settings-servers-failed = Échec : { $error }
settings-servers-rotator-at = ● Connecté — antenne à { $az }° az, { $el }° el

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } satellites · { $age } d'ancienneté
settings-tle-subscribed = Abonné à { $name }. Appuyez sur METTRE À JOUR MAINTENANT pour le récupérer.
settings-tle-norad-too-stale = NORAD { $norad } · { $age } d'ancienneté — trop ancien pour propager
settings-tle-norad-age = NORAD { $norad } · { $age } d'ancienneté
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = { $n } { $n ->
        [one] jeu d'éléments ajouté
       *[other] jeux d'éléments ajoutés
    }.
settings-tle-added-refreshed = { $added } ajouté{ $added ->
        [one] {""}
       *[other] s
    } et { $r } { $r ->
        [one] jeu d'éléments actualisé
       *[other] jeux d'éléments actualisés
    }.
settings-tle-links = { $n } { $n ->
        [one] liaison
       *[other] liaisons
    }
settings-tle-replace-links = Remplacer ces liaisons par les intégrées pour { $name }

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = L'emplacement « { $location } » est { $station_call }, mais vous journalisez sous { $call } — Wavelog refusera ces QSO.
settings-wavelog-grid-mismatch = L'emplacement « { $location } » est en { $station_grid }, mais votre carré est { $grid } — Wavelog refusera ces QSO.
settings-wavelog-location-n = emplacement n° { $id }
settings-wavelog-saved-with-callsign = Enregistré avec l'indicatif ({ $call }) dans Réglages → Profils, pour qu'un profil portable puisse journaliser vers un emplacement portable.
settings-wavelog-owed = { $n } { $n ->
        [one] QSO journalisé
       *[other] QSO journalisés
    } depuis l'activation de l'envoi automatique restent à envoyer — nouvelle tentative toutes les 5 minutes.
settings-wavelog-send-unsent = Envoyer { $n } { $n ->
        [one] QSO
       *[other] QSO
    } pas encore dans Wavelog…
settings-wavelog-send-confirm = Envoyer { $n } QSO à { $location } ? Les réglages sont appliqués d'abord.
settings-wavelog-plan-add = • { $n } QSO à ajouter au journal depuis Wavelog
settings-wavelog-plan-update = • { $n } QSO téléchargés modifiés dans Wavelog à mettre à jour
settings-wavelog-plan-remove = • { $n } QSO téléchargés qui ne figurent plus dans Wavelog (ou venant d'un emplacement décoché) à supprimer
settings-wavelog-plan-mark = • { $n } de vos QSO à marquer comme présents dans Wavelog

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = Antenne
settings-radio-which-socket-back-radio-using =
    Quelle prise à l'arrière la radio utilise — sa propre commande ANT, le même réglage que le bouton ANT de la face avant.

    S'applique immédiatement, et c'est le réglage de la radio plutôt qu'une copie tenue ici : la prise sur laquelle est la radio est relue à l'ouverture de la session.

    Le choix est mémorisé par bande. Passez sur le faisceau en 2 m et sur le fil en 40 m, et chacun revient la prochaine fois que le cadran entre dans cette bande — et la prochaine fois que sdroxide démarre. Un canal mémoire enregistré ici conserve aussi sa prise.
settings-radio-receive-antenna = Antenne de réception
settings-radio-radio-s-separate-receiving-antenna =
    L'antenne de réception séparée de la radio — RX ANT à l'arrière — commutée dans le trajet de réception ou hors de celui-ci. Son propre réglage, le même que le bouton RX ANT de la radio.

    L'antenne sur la prise principale reste en émission dans tous les cas : c'est une entrée supplémentaire pour la réception, une boucle ou une Beverage, non un choix de prise.

    Contrairement à l'antenne ci-dessus, ceci n'est *pas* mémorisé ici. La radio le conserve elle-même par bande, donc sdroxide le relit après chaque changement de bande et affiche ce que dit la radio ; cocher cette case est la seule chose qui le modifie.
settings-radio-receive-path = Dans le trajet de réception
settings-radio-radio-power = Alimentation de la radio
settings-radio-switch-radio-itself-off-back =
    Éteint la radio elle-même, puis la rallume, par la liaison de commande. C'est le seul vrai marche/arrêt du programme — et non le commutateur LIAISON de sdroxide, qui ferme le côté de sdroxide et laisse la radio en marche.

    Pour que le rallumage atteigne quelque chose, l'extrémité de commande de la radio doit rester éveillée pendant qu'elle est éteinte. Sur le réseau, c'est ce que fait le contrôle réseau. Sur un câble série, c'est le port CI-V de la radio, qui reste alimenté par le secteur sur un appareil éteint par la face avant plutôt que débranché ; sdroxide envoie la séquence de réveil que demande la propre documentation d'Icom avant la mise sous tension.

    L'arrêt met fin à l'audio et aux indicateurs — il n'y a plus de radio derrière eux — et sdroxide garde la liaison de commande ouverte pour que le rallumage ait une destination.
settings-radio-on = Marche
settings-radio-off = Arrêt
settings-radio-waiting-configuration-machine-radio-attached = En attente de la configuration de la machine à laquelle la radio est reliée.
settings-radio-sound-format = Format sonore
settings-radio-invert-spectrum = Inverser le spectre
settings-radio-swap-i-q = Permuter I/Q
settings-radio-mirror-panadapter-about-tuned-frequency =
    Symétrise le panoramique autour de la fréquence accordée, pour une radio dont I et Q sont inversés sur sa carte son. Le signe est une cascade pleine de signaux convaincants tous situés du mauvais côté du cadran, avec la BLU ressortant sur la bande latérale opposée — permuter les deux câbles sur la carte son réglerait le problème tout aussi bien.

    Réception seulement : l'émission remet à la radio un vrai signal audio, qui n'a pas de bande latérale à inverser.
settings-radio-i-q-sample-rate = Taux d'échantillonnage I/Q
settings-radio-how-fast-radio-s-i =
    La vitesse à laquelle tourne la carte son I/Q de la radio — et donc la largeur de bande qu'affiche le panoramique, car un flux en quadrature couvre tout son taux d'échantillonnage : 48 kHz donne ±24 kHz de part et d'autre du cadran, 192 kHz donne ±96.

    La carte a le dernier mot. Une carte incapable du taux choisi est ouverte au plus proche qu'elle puisse faire, et le panoramique est d'autant plus étroit — le journal le signale au démarrage lorsque cela se produit.

    Une carte plus rapide demande aussi plus de travail : la machine dispose proportionnellement de moins de temps pour la vider, et une carte qui ne suit pas perd des échantillons. Cela se voit à un audio qui coupe pendant que la cascade reste parfaite, et cela aussi est journalisé. Si vous le constatez, redescendez d'un cran.
settings-radio-t-48-khz-24-khz = 48 kHz (±24 kHz)
settings-radio-t-96-khz-48-khz = 96 kHz (±48 kHz)
settings-radio-t-192-khz-96-khz = 192 kHz (±96 kHz)
settings-radio-t-384-khz-192-khz = 384 kHz (±192 kHz)
settings-radio-i-q-centre-offset = Décalage du centre I/Q
settings-radio-how-far-radio-s-i =
    De combien la sortie I/Q de la radio est centrée au-dessus de son propre cadran, pour une radio dont la F.I. de réception a été déplacée de zéro — sur un Elecraft, MENU:RX SHFT réglé sur 8.0 au lieu de NOR, ce qui éloigne le cadran de la pointe CC du mélangeur (et empêche qu'une forte station BLU/AM voisine soit détectée en AM).

    Ce n'est pas un convertisseur : la radio continue d'afficher et d'émettre sur la fréquence réelle, et rien ici ne lui est jamais envoyé. Cela indique seulement où se trouvent les échantillons de la carte son, et le flux est recalé sur le cadran à mesure qu'il arrive.

    Laissez à 0 sauf si vous avez activé une telle entrée de menu. Si les signaux tombent deux fois plus loin que le décalage, le signe est inversé.
settings-radio-iq-correction = Correction IQ
settings-radio-cancel-mirror-image-every-signal =
    Annule l'image miroir de chaque signal, et la pointe CC au milieu de la cascade.

    La sortie I/Q d'une radio se compose de deux chemins analogiques qui ne sont jamais tout à fait égaux en gain ni exactement à 90° l'un de l'autre, et il en résulte une copie de chaque signal réfléchie autour de la fréquence accordée, généralement 30 à 40 dB plus bas — assez forte pour ressembler à une station absente, et pour être décodée comme telle. Les radios dotées de trimmers d'équilibre en façade règlent exactement cela ; ici, la mesure se fait sur le bruit reçu et ne demande aucun réglage.

    Laissez-la activée. Désactivez-la si vous écoutez l'AM accordé pile sur la porteuse — la porteuse est en CC, donc elle part avec la pointe — ou pour vérifier si un signal est réel en regardant s'il survit.
settings-radio-cancel-mirror-images = Annuler les images miroirs
settings-radio-dc-notch = Filtre coupe-CC
settings-radio-widen-hole-taken-out-middle =
    Élargit le trou pratiqué au milieu de la largeur de bande, pour une radio dont le pic central est plus large que le décalage qui se trouve dessous.

    0 laisse le bloqueur ordinaire, qui fait quelques dizaines de hertz de large et supprime le décalage sans toucher au reste. Le monter fait aussi disparaître le bas de la largeur : un passe-haut du premier ordre, atténué de 3 dB à la valeur réglée ici et davantage plus bas.

    Il est centré là où est centré le I/Q de la radio, c'est-à-dire le cadran, sauf si le décalage ci-dessus l'a déplacé — donc tout ce qui est accordé là part aussi. Une note CW à 600 Hz tombe dans un réglage de 600 Hz.
settings-radio-panadapter-bw = Largeur du panoramique
settings-radio-cat-family = Famille CAT
settings-radio-rigctld-address = Adresse rigctld
settings-radio-host-port-running-hamlib-rigctld =
    hôte:port d'un rigctld Hamlib en cours d'exécution — 127.0.0.1:4532 est sa valeur par défaut, sur cette machine.

    Lancez-en un avec, par exemple, `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, où -m est le numéro de modèle Hamlib de votre radio (`rigctl -l` les liste).

    C'est la solution générale : elle atteint la fréquence, le mode, le PTT, la puissance, le S-mètre et le ROS, et rien d'autre. Lorsque l'une des familles natives ci-dessus convient à votre radio, elle en fait davantage — manipulation depuis le tampon de texte de la radio, filtre de réception, et échelles de mesure propres à chaque modèle.
settings-radio-flrig-address = Adresse flrig
settings-radio-host-port-running-flrig-127 =
    hôte:port d'un flrig en cours d'exécution — 127.0.0.1:12345 est sa valeur par défaut, sur cette machine. flrig sert en XML-RPC chaque fois qu'il tourne ; le port se trouve dans Config → Setup → Server.

    Comme l'option Hamlib, il pilote un démon plutôt que la radio, mais par le pilote propre à chaque modèle de flrig — sur plusieurs radios, sa gestion de la puissance et des filtres est la plus fidèle des deux. Il atteint la fréquence, le mode, le PTT, la puissance d'émission (en watts entiers), la largeur de bande de réception, le S-mètre, le ROS et la puissance de sortie. La manipulation CW passe par le port cwio propre à flrig, qui doit être configuré dans flrig lui-même.
settings-radio-serial-port = Port série
settings-radio-select = — sélectionner —
settings-radio-baud = Débit
settings-radio-data-bits = Bits de données
settings-radio-parity = Parité
settings-radio-stop-bits = Bits d'arrêt
settings-radio-force-rts = Forcer RTS
settings-radio-force-dtr = Forcer DTR
settings-radio-ptt-method = Méthode PTT
settings-radio-how-transmit-keyed = Comment l'émission est déclenchée.
settings-radio-how-transmit-keyed-network-link = Comment l'émission est déclenchée. Une liaison réseau n'a pas de lignes de commande, donc DTR et RTS ne déclenchent rien ici — utilisez CAT, qui demande au démon de manipuler la radio, ou VOX.
settings-radio-mode-control = Commande du mode
settings-radio-digimode-mode = Mode numérique
settings-radio-what-put-radio-mode-sdroxide =
    Le mode à donner à la radio pour un mode que sdroxide module par sa carte son. « DIGI » sélectionne la position DATA/PKT de l'appareil, qui prend l'audio d'émission sur l'entrée USB ou ACC, en retirant de ce trajet le traitement de la parole du chemin micro ; « USB » le laisse sur la bande latérale ordinaire, pour une radio déjà réglée pour moduler la BLU depuis son entrée données, ou une radio qui n'a aucune position DATA.

    Cela couvre tous les modes numériques — FT8, FT4, PSK, RTTY, SSTV et les autres — et la CW envoyée par « Carte son », et cela remplace la Commande du mode pour eux. Le SSTV est le seul dont la bande latérale suit la bande : là, cela signifie la position DATA sur cette bande latérale — USB-D au-dessus du 40 m, LSB-D à partir de là et en dessous. Le SSTV-FM n'en fait pas partie : une porteuse FM n'a pas de bande latérale à choisir.
settings-radio-cw-keying = Manipulation CW
settings-radio-how-cw-panel-s-keyer =
    Comment le manipulateur du panneau CW émet. « Manipulateur de la radio » met la radio en CW et lui remet le texte à envoyer avec son propre manipulateur. Il utilise la vitesse du manipulateur de la radio (réglée depuis le WPM du panneau), et sur Yaesu il envoie par la mémoire de manipulateur 1, en écrasant ce qui y était stocké.

    Il faut activer le break-in sur la radio, ce que sdroxide fait lui-même à chaque message sur Icom, Yaesu et Kenwood — une radio avec le break-in désactivé accepte le message et n'émet rien. Il reste activé ensuite, et un break-in complet n'est jamais ramené à semi.

    « Carte son » envoie à la place la manipulation sous forme de signal audio (MCW), un ton à cadran + tonalité — et comme une radio en CW ignorerait totalement sa carte son, le choix de CW suit alors le réglage du mode Mode numérique (USB, DATA, ou Contrôlé par la radio) au lieu de passer la radio en CW. Choisissez « Contrôlé par la radio » ici pour les radios dont la position données ne peut pas être commandée par CAT (le U-D d'un Xiegu), et mettez vous-même la radio sur cette position, comme pour le FT8.
settings-radio-poll-rate = Fréquence d'interrogation
settings-radio-how-often-radio-asked-what =
    La fréquence à laquelle la radio est interrogée sur ce qu'elle fait — son cadran, son mode et ses indicateurs. C'est la moitié de la liaison de commande qui va de la radio vers sdroxide : tournez le bouton VFO de l'appareil ou changez son mode sur la face avant, et l'affichage, la bande et le panoramique suivent en un cycle d'interrogation.

    C'est aussi tout le trafic de commande produit de ce côté, et sur un Icom moderne ce trafic n'est pas gratuit : le port CI-V et la carte son sont derrière un même hub USB à l'intérieur de la radio, donc les trames demandées ici prennent du temps de bus que l'audio n'obtient pas — on l'entend sous forme de coupures qui ressemblent à un défaut de DSP et n'en sont pas un. La valeur par défaut de 2 Hz est à une demi-seconde du bouton et assez discrète pour ne pas gêner l'audio. Augmentez-la lorsque le port de commande a son propre périphérique (un adaptateur USB-série distinct, un rigctld en réseau) ; baissez-la si l'audio de la radio coupe.

    Un Icom dont le CI-V Transceive est activé dans son propre menu n'a pas besoin de tout cela pour le cadran : il signale le bouton dès qu'il bouge, et sdroxide arrête l'interrogation du cadran dès qu'il le voit faire — et la reprend si la radio change de fréquence sans le dire.

    Le débit est celui du cadran. Le mode n'est relevé qu'une interrogation sur quatre, car c'est un réglage qui change quelques fois dans une soirée plutôt qu'un élément qui suit un bouton.
settings-radio-send-command = Commande d'envoi
settings-radio-which-transceiver-generation-keys-rig =
    La génération de transceiver qui manipule la radio, pour la méthode PTT « CAT ». Les deux ne s'accordent pas sur le sens du paramètre TX, et rien sur le fil ne permet de les distinguer, donc choisissez celle qui correspond à votre radio.

    « Style TS-2000 (TX;) » — TS-480, TS-570, TS-870, TS-2000 : l'émission ordinaire, sur la bande principale.

    « Style TS-590 (TX1;) » — TS-590S/SG, TS-890, TS-990 : DATA SEND, qui manipule avec l'entrée audio ACC2/USB en direct. Sur ces radios, l'émission ordinaire sélectionne le micro à la place et coupe l'audio que sdroxide émet.

    Mal réglé, un TS-590 n'émet que du silence — mais un TS-2000 émet sur la sous-bande, qui est une tout autre bande.
settings-radio-radio = Radio
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    Un seul profil couvre la famille : le jeu de commandes du K3, auquel répondent aussi le KX3, le KX2 et le K4. Il n'y a rien à choisir ici — le nombre de watts que couvre le curseur d'excitation (12 sur un KX2 ou KX3 nus, 110 avec un KPA3 ou un KXPA100) est lu depuis la requête du module d'option de l'appareil lorsque le port s'ouvre.

    Notez les débits ci-dessus : le K3, le K3S, le KX3 et le KX2 ne dépassent pas 38400, et une radio réglée en dessous du débit choisi ici ne répond rien du tout.
settings-radio-transmit-input = Entrée émission
settings-radio-where-radio-takes-transmit-audio =
    D'où la radio prend son audio d'émission — la commande TI de l'appareil, qui est le menu 32 « TX IN » sur la face avant. Envoyé à l'ouverture du port.

    « Audio USB » est ce qui fait fonctionner cette interface : le FDM-DUO émet ce que sdroxide met dans sa carte son USB. Une radio laissée sur « Microphone » émet à la place ce qui se dit dans la pièce, et rien à l'écran ne le signale.

    « Auto » laisse l'appareil choisir — le micro pour un appui PTT sur le micro, le port USB pour un appui CAT ou RTS.

    « Laisser comme réglé sur la radio » n'envoie aucun TI, pour un opérateur qui règle cela sur la face avant.
settings-radio-which-two-sockets-back-receiver =
    Laquelle des deux prises à l'arrière le récepteur écoute — la commande AN de l'appareil, qui est le menu 31 « ANTENNAS » sur la face avant et l'indicateur « ANT 1 2 » sur son afficheur.

    « RTX » est une antenne qui remplit les deux fonctions, sur la prise de type M qui porte aussi l'émission. « RX seul » fait passer la réception sur la seconde prise et laisse l'émission sur RTX — une antenne de réception, une boucle ou une Beverage, le faisceau restant sur l'émetteur.

    S'applique immédiatement, et est relu depuis la radio à l'ouverture du port : c'est le réglage propre à l'appareil, et non une copie tenue ici.
settings-radio-one-profile-covers-both-there =
    Un seul profil couvre les deux. Il n'y a rien à choisir ici — la radio se nomme elle-même à l'ouverture du port.

    Ceci ne concerne que la partie CAT : elle pilote le cadran, le mode, le PTT, le S-mètre, le ROS et la puissance d'émission par le port USB CAT de l'appareil, et prend l'audio sur son port USB Audio. Le troisième port USB du FDM-DUO transporte l'I/Q large bande, que cette interface ne lit pas — choisissez l'interface ELAD ci-dessus pour cela.

    Notez le débit : le menu 70 « CAT BAUD » de la radio doit correspondre, et il est livré à 38400.

    La CW est manipulée par la manipulation ou la palette propre à la radio. Le FDM-DUO n'a aucune commande qui accepte du texte, donc le panneau CW ne peut pas le manipuler par CAT.
settings-radio-one-profile-covers-range-qrp =
    Un seul profil couvre la gamme : le jeu de commandes propre à QRP Labs, qui est un sous-ensemble de celui du Kenwood TS-480 auquel on a beaucoup ajouté. Il n'y a rien à choisir ici — la radio se nomme elle-même avec la version de son micrologiciel à l'ouverture du port, et c'est cette version qui décide si la lecture de la protection ROS est demandée.

    Pas le profil Kenwood : sur un QMX, la commande PC est le wattmètre et non le réglage de puissance, donc une radio pilotée comme un Kenwood verrait sa lecture de mesure écrite comme si c'était un réglage, et MD8 — un mode Kenwood — est ici le SWR Tune.

    La puissance d'émission se règle sur la radio. Il n'existe aucune commande CAT pour cela, donc le curseur d'excitation n'atteint que le niveau de l'audio qui entre dans la carte son.

    Le filtre de réception est celui de la radio aussi : elle annonce la largeur que lui impose son mode (3,2 kHz en Digi, 300 Hz en CW) et ne propose aucun moyen de la changer.
settings-radio-baud-rate-ignored-port-usb = Le débit est ignoré — le port est USB
settings-radio-qmx-serves-own-virtual-com =
    Un QMX expose ses propres ports COM virtuels par USB, donc le débit réglé ci-dessus n'a d'effet à aucun bout. Il en propose jusqu'à trois ; si la radio ne répond rien, la cause la plus probable est une autre que celle-ci.

    Une chose compte vraiment sur le port : un retour chariot envoyé dessus fait passer la radio en mode terminal pour le reste de la session. sdroxide n'en envoie jamais — mais un programme de terminal laissé ouvert sur le même port, lui, en envoie, et la CAT cesse de fonctionner dès qu'il le fait.
settings-radio-i-q-mode-switched-radio = Le mode I/Q est activé sur la radio
settings-radio-radio-s-sound-card-carries =
    La carte son de la radio transporte soit l'audio démodulé, soit l'I/Q brut que voit son CAN, et sdroxide impose celui que demande le Format sonore ci-dessus à l'ouverture du port (la commande Q9 de la radio, l'entrée de menu « IQ mode »).

    Le décalage du centre et le taux d'échantillonnage ci-dessus ont été réglés sur ce dont a besoin un QMX : son récepteur est un superhétérodyne avec une F.I. de 12 kHz, donc le synthétiseur se trouve 12 kHz sous le cadran et tout ce qui est sur la carte est 12 kHz au-dessus du milieu de la largeur. La carte fonctionne à 48 kHz, ce qui donne un panoramique large de 48 kHz.

    En CW la radio déplace cette F.I. d'environ 700 Hz de plus, ce qu'un seul chiffre ne peut suivre — ajoutez-le à la main si vous utilisez le panoramique avec la radio en CW. En Digi, où se place d'ordinaire une radio utilisée comme front-end I/Q, il n'y a rien à ajouter.

    QRP Labs précise que le mode I/Q ne convient pas à WSJT-X et aux autres programmes qui attendent de l'audio démodulé ; ici, la démodulation se fait de ce côté, donc cela convient.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ (émetteur-récepteur HF 5 W)
settings-radio-hobbypcb-s-rs-hfiq-there =
    Le RS-HFIQ de HobbyPCB. Il n'y a rien à choisir : le profil est tout le jeu de commandes de la radio, et la version du micrologiciel est journalisée à l'ouverture du port.

    La liaison de commande ne transporte que deux choses et pas davantage — où placer l'oscillateur, et s'il faut émettre. Il n'y a aucune commande de mode, de puissance, de filtre, de squelch ou de mesure, car la radio n'en a aucune. Tout le reste relève de sdroxide : ce qui sort de la carte son est une bande de base complexe centrée sur le cadran, donc le mode, le filtre et la modulation se font tous de ce côté.

    La commande de fréquence couvre 3–30 MHz. Si vous en demandez davantage, la radio refuse et reste où elle est ; le journal le signale.

    La CW est manipulée sous forme d'audio à travers la chaîne d'émission — cette radio n'a pas de manipulateur qui accepte du texte. (Elle a un générateur CW interne ; sdroxide ne l'utilise jamais, sur le conseil du micrologiciel lui-même.)
settings-radio-sound-format-baud-ptt-are = Le format sonore, le débit et le PTT sont ceux de la radio, pas des réglages
settings-radio-all-three-have-been-filled =
    Les trois ont été renseignés, et aucun n'est une préférence :

    • Le format sonore est I/Q (stéréo) — la carte transporte la bande de base complexe et rien d'autre.
    • 57600 bauds, 8N1 — le débit unique du micrologiciel, sans menu pour le changer. Tout autre débit donne une liaison muette, donc il est fixé à l'ouverture du port.
    • PTT par CAT — la commande *X est le seul interrupteur d'émission de l'interface.

    Le décalage du centre a été mis à zéro : le synthétiseur fonctionne à quatre fois le cadran vers un détecteur en quadrature, donc le milieu de la largeur est le cadran. Réglez le taux d'échantillonnage ci-dessus sur ce que fait réellement votre carte son ; c'est lui qui donne au panoramique sa largeur.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    L'émetteur-récepteur QRP de poche de DL2MAN/PE1NNZ et le micrologiciel uSDX ouvert dont il est issu. Il émule un Kenwood TS-480, mais le sous-ensemble est maigre : il ne répond que au cadran, au mode, au PTT et à l'effacement RIT/XIT. Pas de S-mètre, pas de ROS, pas de commande de puissance, pas de VFO B, pas de split et pas de manipulateur CAT, donc le curseur d'excitation et les mesures ne font rien par CAT — le niveau audio est la seule commande d'émission qui existe.

    La CW est manipulée sous forme d'audio (MCW) à travers la bande latérale, ou avec la manipulation ou la palette sur la radio ; aucune commande ne prend du texte.
settings-radio-audio = Audio
settings-radio-tr-usdx-has-no-sound =
    Un (tr)uSDX n'a pas de carte son propre et deux façons d'être entendu, et laquelle est la bonne tient à votre station plutôt qu'à la radio.

    « Un seul câble » prend l'audio à l'intérieur même de la liaison série CAT — le flux 8 bits de la radio, sur le même câble USB que la commande, sans rien d'autre à brancher. Le revers tient au micrologiciel : il ne peut pas recevoir une commande CAT pendant que son flux tourne (une commande envoyée dans le flux ne le met pas en pause, elle le tue), donc ce mode n'interroge jamais. Le réglage et les changements de mode sont envoyés au fur et à mesure, chacun mettant le flux en pause un instant puis le relançant — attendez-vous à un bref trou à chaque changement ; rien n'est envoyé lorsque vous écoutez simplement. Le cadran et le mode propres à la radio ne sont pas suivis non plus, car il n'y a pas de lecture pour cela — pilotez-la donc depuis ici.

    « Carte son USB » prend l'audio d'une carte son externe branchée sur la prise haut-parleur/micro 3,5 mm de la radio, et se comporte comme tout autre appareil CAT : l'interrogation du cadran fonctionne, donc le bouton et le mode de la radio sont suivis. Choisissez la carte sous Audio radio ci-dessous.

    Les deux modes ouvrent le port avec DTR maintenu haut : le DTR de l'adaptateur série est relié au reset de la radio, donc ouvrir le port la redémarre, et la première seconde ou les deux suivantes sont silencieuses le temps qu'elle démarre. DTR n'est jamais utilisé pour manipuler.
settings-radio-radio-model = Modèle de radio
settings-radio-which-icom-two-things-ci =
    Quel Icom, pour les deux choses que le CI-V ne fait pas de la même façon sur tous.

    L'adresse de l'émetteur-récepteur ci-dessous est remplie à partir de là : chaque modèle en a une différente, et une trame envoyée à la mauvaise adresse est tout simplement ignorée — une radio qui ne répond rien, sans aucune erreur pour dire pourquoi.

    Cela décide aussi si sdroxide peut choisir le mode DATA. Sur le CI-V, USB et USB-DATA ont le même octet de mode, et une commande distincte les départage ; sans elle, un message de mode numérique passe par l'entrée micro, avec le traitement de la parole et le filtre BLU de la radio sur le trajet.

    « Autre » laisse l'adresse à votre charge et n'envoie aucune commande de mode DATA.
settings-radio-radio-id-hex = ID de la radio (hex)
settings-radio-show-radio-s-spectrum-scope = Afficher l'oscilloscope spectral de la radio
settings-radio-scope-span = Largeur de l'oscilloscope
settings-radio-how-wide-sweep-radio-keeps = La largeur du balayage. La radio conserve la largeur choisie en dernier sur son propre écran — souvent quelques kHz. Régler une largeur ici place aussi l'oscilloscope en mode centré, donc il suit le cadran. Cela modifie aussi l'affichage propre à la radio ; « Comme réglé sur la radio » le laisse tranquille.
settings-radio-press-apply-reconnect-switch-without = Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-radio-audio-sound-card = Audio radio (carte son)
settings-radio-devices = Périphériques
settings-radio-discover = Découvrir
settings-radio-no-devices-press-discover = aucun périphérique — appuyez sur Découvrir
settings-radio-manual-ip = IP manuelle
settings-radio-optional-e-g-192-168 = facultatif, p. ex. 192.168.1.50
settings-radio-sample-rate = Taux d'échantillonnage
settings-radio-receiver-ddc = Récepteur (DDC)
settings-radio-protocol-2-board-carries-several = Une carte Protocole 2 porte plusieurs récepteurs accordables indépendamment (DDC) sur une seule connexion — faites tourner cette radio sur DDC1 et une autre radio, à la même adresse, sur DDC2. L'émetteur appartient à la radio DDC1. Les cartes Protocole 1 n'ont que DDC1.
settings-radio-lna-gain = Gain LNA
settings-radio-front-end-gain-hermes-lite = Gain d'entrée d'un Hermes-Lite 2. Prend effet immédiatement — sans reconnexion — et est mémorisé comme le niveau auquel démarre la radio. Trop élevé, il écrête le CAN et toute la bande semble déformée ; trop bas, le récepteur devient sourd.
settings-radio-overload-protection = Protection contre la saturation
settings-radio-back-lna-gain-off-itself =
    Réduit de lui-même le gain LNA tant que la carte signale que son CAN déborde, et le remonte dès que cela cesse.

    Un Hermes-Lite 2 échantillonne toute la plage 0-38 MHz sur un seul convertisseur 12 bits, sans mélangeur ni présélecteur devant lui, si bien qu'une station de radiodiffusion située à une bande de là peut le faire saturer alors que la bande que vous regardez ne présente rien d'anormal — le bruit de fond monte, tout intermodule et les décodeurs s'arrêtent. La carte le sait, et c'est ceci qui agit là-dessus.

    Désactivé par défaut : cela modifie un réglage que vous avez choisi. Rien ne se passe pendant l'émission.
settings-radio-wind-gain-back-when-adc = Réduire le gain lorsque le CAN déborde
settings-radio-applies-apply-reconnect-main-window = S'applique à Appliquer / reconnecter. La réglette de gain de la fenêtre principale affiche le gain effectivement utilisé par la carte, donc elle suit la boucle et vous pouvez voir ce qu'elle fait ; le curseur ci-dessus est le niveau auquel démarre la radio, et la boucle ne le réécrit pas.
settings-radio-how-far-gain-moves-each = De combien le gain se déplace à chaque fois, en dB. Un dB est le pas du registre de gain propre à la carte.
settings-radio-attack-decay = {"  "}Attaque / retour
settings-radio-how-often-gain-may-come =
    La fréquence à laquelle le gain peut baisser pendant que le convertisseur déborde, et la fréquence à laquelle il peut remonter une fois la saturation terminée.

    Les deux sont volontairement séparés d'un facteur cent. Reculer immédiatement : chaque milliseconde de saturation est un récepteur plein d'intermodulation. Revenir lentement : ce qui l'a causée — un voisin qui manipule, une station de radiodiffusion qui monte au crépuscule — n'a en général pas disparu, et une boucle qui se rétablirait aussi vite qu'elle recule passerait la soirée à osciller autour du seuil. 100 ms et 10 s par décibel sont ce qu'ont utilisé à la fois le Auto S-Att de PowerSDR et le HermesIntf de N1GP.
settings-radio-range = {"  "}Plage
settings-radio-lowest-highest-gain-loop-may = Le gain le plus bas et le plus haut que la boucle peut utiliser, en dB. Le plafond empêche qu'elle décide de la sensibilité de votre récepteur ; le plancher est l'endroit où vous indiquez qu'en dessous d'un certain niveau la saturation est le problème d'un autre et que la réponse est un filtre, non vingt décibels de plus.
settings-radio-filter-board = Carte de filtres
settings-radio-accessory-board-hermes-lite-2 = Carte d'accessoires sur le connecteur J16 du Hermes-Lite 2 (ou les sorties à collecteur ouvert de toute autre carte openHPSDR — Protocole 2 compris). « N2ADR » choisit un relais par bande ; « Code de bande Alex / Hermes » met le numéro de bande sur les sorties 1-4, ce qu'attendent la carte Alex d'un ANAN, un Zeus SDR, un HiQSDR et Quisk. « Personnalisé » ouvre plus bas un tableau dans lequel vous indiquez vous-même les sept sorties, bande par bande, en réception comme en émission — pour un commutateur d'antenne, le décodeur de bande d'un amplificateur ou tout autre usage qu'aucun des deux préréglages ne couvre ; chacun des deux peut être versé dedans comme point de départ. Laissez « Aucune » à moins qu'un élément ne soit réellement installé : ces sept broches sont des sorties à collecteur ouvert à usage général, et les opérateurs les relient aussi à la commande PTT d'un amplificateur, à des relais d'antenne et à la commutation de transverteurs. Les piloter à partir des données de bande mettrait en marche ce qui est branché. S'applique à Appliquer / reconnecter.
settings-radio-io-board-rx-input = Entrée RX de la carte IO
settings-radio-where-n2adr-hl2ioboard-takes-receive = D'où une carte N2ADR HL2IOBoard prend son signal de réception. La carte elle-même est trouvée automatiquement et ne demande aucun réglage ; celui-ci n'existe que pour les opérateurs qui ont câblé ses propres prises SMA. Laissez « Entrée propre de la radio » sinon — sélectionner le J9 de la carte IO sans rien branché dessus rend le récepteur sourd. S'applique à Appliquer / reconnecter. Réglé sur l'un des choix J9, les entrées apparaissent aussi sur la commande ANT et sont mémorisées par bande.
settings-radio-power-amplifier = Amplificateur de puissance
settings-radio-use-hermes-lite-2-s = Utiliser l'ampli intégré du Hermes-Lite 2
settings-radio-default-what-you-want-unless = Activé par défaut, et c'est ce que vous voulez, sauf si un amplificateur externe est piloté depuis la sortie RF1 à faible puissance de la carte. Désactivé, la radio se déclenche toujours — le relais E/R bascule et toute carte d'accessoires suit — mais la prise d'antenne ne produit aucune puissance, et le relais est volontairement maintenu en réception. Ignoré sur les cartes autres qu'un Hermes-Lite.
settings-radio-mirror-board-s-spectrum-about = Symétrise le spectre de la carte autour de la fréquence accordée, à l'émission comme à la réception. Activé par défaut : un Hermes-Lite 2 en a besoin. Ne le désactivez que si des signaux apparaissent du mauvais côté du cadran et que rien ne se décode — le signe est une cascade pleine de traces convaincantes pendant que la BLU tombe sur la mauvaise bande latérale et que le FT8 ne donne aucun décodage.
settings-radio-frequency-correction = Correction de fréquence
settings-radio-crystal-tcxo-error-ppm-applied = Erreur du quartz/TCXO en ppm, appliquée à la réception et à l'émission. S'applique immédiatement.
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = Tampon d'émission
settings-radio-how-far-ahead-real-time = De combien d'avance sur le temps réel l'audio d'émission est envoyé à la carte par le réseau, avant que sdroxide ralentisse pour s'accorder. La carte elle-même ne possède aucun tampon de ce genre — ceci n'élargit que la marge propre à sdroxide. Augmentez-le sur une liaison WiFi ou un VPN, où la valeur par défaut basse (adaptée à une connexion filaire directe) ne laisse pas assez de marge contre la gigue et où l'audio émis ou le PTT hachent ; une valeur plus haute coûte de la latence à l'émission. S'applique à APPLIQUER, qui reconnecte la carte.
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = Prédistorsion adaptative : linéarise l'émetteur à partir d'un échantillon de ce qu'il émet réellement, pour une vingtaine de décibels de moins d'intermodulation à la même puissance. Le récepteur est la boucle de retour — la carte continue de recevoir pendant un trafic — donc un coupleur directif et un atténuateur doivent faire parvenir un échantillon de la sortie de l'amplificateur à une entrée que le commutateur E/R ne retire pas à l'émission. Sur un Hermes-Lite 2, cela signifie la prise PureSignal de la carte IO, et l'entrée de réception ci-dessus réglée en conséquence ; sur une carte qui n'en a pas, le port RX d'un ANAN, ou un relais externe qui branche le coupleur pendant la durée du trafic. C'est ce récepteur qui est lu — une carte dotée d'un second CAN ne peut pas utiliser RX2 comme prise de retour, quelle que soit la façon dont son propre micrologiciel l'aiguille. Sans rien de couplé, la boucle ne se verrouille jamais et l'émetteur reste exactement comme il aurait été. S'applique à Appliquer / reconnecter, et uniquement à la radio qui possède l'émetteur (DDC1).
settings-radio-correct-transmitter-receiver-s-own = Corriger l'émetteur à partir de la réaction du récepteur lui-même
settings-radio-how-finely-amplifier-s-curve = La finesse avec laquelle la courbe de l'amplificateur est modélisée. Plus de pas suivent un coude plus net et prennent plus de temps à remplir ; 32 est un bon point de départ.
settings-radio-how-fast-correction-follows-what = La vitesse à laquelle la correction suit ce que signale le coupleur. Lent est le bon réglage — il s'agit de moyenner la courbe d'un amplificateur, qui ne bouge pas, à partir d'une boucle de retour bruitée.
settings-radio-hold = Maintien
settings-radio-stop-adapting-keep-correction-where = Cesser l'adaptation et garder la correction où elle est — pour mesurer, et pour un opérateur satisfait de ce qu'elle a appris.
settings-radio-manual-ip-overrides-discovery-press = Une IP manuelle remplace la découverte. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-open-collector-outputs-band = Sorties à collecteur ouvert par bande
settings-radio-one-control-word-per-band = Un mot de commande par bande, comme le indique la documentation de votre matériel : le bit 0 est la sortie 1, le bit 6 la sortie 7. RX est activé pendant la réception sur cette bande et TX pendant que l'émetteur est manipulé — donnez-leur la même valeur pour un filtre, et des valeurs différentes pour tout élément qui ne doit exister que d'un côté du basculement (la ligne de manipulation d'un amplificateur, le contournement d'un préamplificateur de réception). Les bandes laissées à 00 n'activent rien. S'applique à Appliquer / reconnecter.
settings-radio-alex-band-code = CODE DE BANDE ALEX
settings-radio-replace-table-what-preset-would = Remplace le tableau par ce que ce préréglage enverrait sur chaque bande, comme point de départ à modifier. Rien d'autre dans la configuration ne change.
settings-radio-clear = EFFACER
settings-radio-every-output-off-every-band = Chaque sortie désactivée sur chaque bande, ce que fait « Aucune ».
settings-radio-band = Bande
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = Sorties activées
settings-radio-other = Autre
settings-radio-everywhere-outside-amateur-bands-short = Partout hors des bandes amateur — écoute en ondes courtes, et tout ce sur quoi tombe le cadran d'un transverteur et qu'aucune bande ne couvre.
settings-radio-dongle = Clé
settings-radio-rescan = Rechercher
settings-radio-re-list-usb-bus-no = Relister le bus USB. Aucun périphérique n'est ouvert, donc on peut appuyer dessus pendant la réception.
settings-radio-no-dongles-press-rescan = aucune clé — appuyez sur Rechercher
settings-radio-first-one-found = — la première trouvée —
settings-radio-rtl2832u-s-resampler-reaches-225 = Le rééchantillonneur du RTL2832U va de 225 à 300 kHz et de 900 kHz à 3,2 MHz, rien entre les deux. Prend effet à Appliquer.
settings-radio-often-drops-samples = {"  "}(perd souvent des échantillons)
settings-radio-agc = CAG
settings-radio-manual-setting-measurement-weak-signal = Manuel est le réglage pour la mesure et les modes numériques en signaux faibles. L'accord et le démodulateur ont des boucles automatiques indépendantes.
settings-radio-tuner-gain = Gain de l'accord
settings-radio-applies-immediately-no-reconnect-tuner = S'applique immédiatement — sans reconnexion. L'accord a 29 pas discrets, donc la valeur se cale sur le plus proche qu'il puisse produire. Ignoré pendant que le CAG de l'accord fonctionne.
settings-radio-crystal-error-parts-per-million = Erreur du quartz en parties par million. Lancez avec RUST_LOG=sdroxide_rtlsdr=debug et le journal affiche l'erreur d'horloge mesurée après environ 20 secondes — c'est ce nombre-là qu'il faut saisir. S'applique immédiatement.
settings-radio-hf-reception = Réception HF
settings-radio-tuner-itself-starts-24-mhz =
    L'accord lui-même commence à 24 MHz. Un RTL-SDR Blog V4 convertit vers le haut en dessous de cette valeur, matériellement ; les autres clés n'atteignent la HF qu'en échantillonnant directement le CAN, par le port HF du V3. Automatique confie tout ce qui est sous 24 MHz au CAN et tout ce qui est au-dessus à l'accord.

    L'échantillonnage direct couvre toutes les bandes HF, 17 m et 15 m compris — elles arrivent dans la seconde zone de Nyquist du CAN, dans le bon sens. Rien ne filtre toutefois l'entrée du CAN, donc ce qui se trouve à 28,8 MHz moins le cadran arrive avec elles : 10,7 MHz sous le 17 m, 7,726 MHz sous le 15 m.

    Changer de mode interrompt brièvement le flux.
settings-radio-removes-dongle-s-own-dc =
    Supprime le pic continu propre à la clé, au centre de la largeur, ainsi que l'image miroir que chaque signal laisse réfléchie autour de lui, en mesurant le déséquilibre dans les échantillons eux-mêmes — sans étalonnage, et cela s'applique immédiatement. L'accord n'a pas de mode de décalage d'accord, donc c'est le seul moyen de dégager le centre.

    Une porteuse AM accordée pile sur le cadran est elle aussi en continu, donc elle part avec le pic : accordez à un kilohertz à côté, ou désactivez ceci.
settings-radio-remove-centre-spike-mirror-image = Supprimer le pic central et l'image miroir
settings-radio-bias-tee = Té de polarisation
settings-radio-feed-4-5-v-dc = Alimenter ~4,5 V continus sur le câble coaxial
settings-radio-bias-tee-never-connect-transceiver = Le té de polarisation est ACTIVÉ. Ne branchez jamais un émetteur-récepteur, une antenne mise à la masse ou un préampli alimenté d'ailleurs pendant que ceci est activé — la tension continue descend directement dans la ligne d'alimentation.
settings-radio-receive-only-dongle-sample-rate = Réception seule. La clé et le taux d'échantillonnage prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez.
settings-radio-server-address = Adresse du serveur
settings-radio-where-rtl-tcp-listening-address =
    Là où rtl_tcp écoute : une adresse, ou une adresse et un port. Le port vaut 1234 par défaut, qui est la valeur propre à rtl_tcp.

    De l'autre côté, lancez-le avec `rtl_tcp -a 0.0.0.0` — lié à 127.0.0.1, ce qu'il fait sans -a, il n'accepte que les connexions venant de cette même machine.

    Prend effet à Appliquer.
settings-radio-host-host-port-e-g = hôte ou hôte:port, p. ex. raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    Demandé au serveur, avec les mêmes limites de rééchantillonnage qu'une clé locale — c'est le même silicium à l'autre bout.

    Le chiffre à côté de chaque taux est ce qu'il coûte sur la liaison : les échantillons sont envoyés sans compression, et un taux que le réseau ne peut pas porter fait que rtl_tcp coupe la connexion plutôt que de se dégrader. Prend effet à Appliquer.
settings-radio-wired-link = {"  "}(liaison filaire)
settings-radio-runs-server-s-dongle-manual = Fonctionne sur la clé du serveur. Manuel est le réglage pour la mesure et les modes numériques en signaux faibles.
settings-radio-applies-immediately-no-reconnect-sent = S'applique immédiatement — sans reconnexion. Envoyé en dixièmes de dB, et calé par le serveur sur un pas que son accord possède ; le protocole ne comporte aucune réponse, donc ce sur quoi il s'est fixé ne peut pas être relu et ce curseur continue d'afficher ce qui a été demandé. Ignoré pendant que le CAG de l'accord fonctionne.
settings-radio-crystal-error-server-s-dongle =
    Erreur du quartz de la clé *du serveur*, en parties par million — une propriété de ce matériel, donc elle se règle ici et non sur les clés de cette machine. S'applique immédiatement.

    L'erreur d'horloge mesurée qu'affiche l'interface USB n'est pas disponible ici : sur un réseau, ce que mesure cette mesure est la mise en tampon, non le quartz, et elle est fausse de plusieurs milliers de ppm. Étalonnez la clé en USB une fois et reportez le nombre, ou accordez une station de radiodiffusion de fréquence connue et ajustez jusqu'à ce qu'elle tombe sur le cadran.
settings-radio-tuner-starts-24-mhz-below =
    L'accord commence à 24 MHz ; en dessous, l'autre bout a besoin d'aide. Un Blog V4 convertit de lui-même et Automatique le laisse tranquille — ce qui est la seule chose qu'il puisse faire, puisque le protocole ne signale que la puce d'accord et rien d'autre, et qu'un V4 est indiscernable d'un simple R828D sur le fil.

    Sur un V3 ou toute autre clé, Automatique fait passer le serveur à l'échantillonnage direct sous le plancher de 24 MHz de l'accord — ce qui couvre toutes les bandes HF, 17 m et 15 m arrivant dans la seconde zone de Nyquist du CAN, avec 28,8 MHz moins le cadran replié par-dessus. Choisissez explicitement Échantillonnage direct pour un R828D simple qui n'entend rien en HF. Le changement interrompt brièvement le flux.
settings-radio-removes-dongle-s-dc-spike =
    Supprime ici le pic continu de la clé et son image miroir, à partir des échantillons au fur et à mesure de leur arrivée — ce sont des artefacts du matériel, donc ils voyagent sur le réseau avec tout le reste. S'applique immédiatement.

    Une porteuse AM accordée pile sur le cadran est elle aussi en continu, donc elle part avec le pic : accordez à un kilohertz à côté, ou désactivez ceci.
settings-radio-powers-preamp-far-end-s = Alimente un préampli depuis la clé de l'autre bout. Les anciens serveurs n'implémentent pas la commande et l'ignorent silencieusement — le protocole n'a aucun moyen de dire non — donc un té de polarisation qui ne s'allume pas n'est pas forcément de notre fait.
settings-radio-feed-4-5-v-dc-2 = Alimenter ~4,5 V continus sur le câble coaxial distant
settings-radio-bias-tee-hardware-somewhere-else = Le té de polarisation est ACTIVÉ, sur un matériel qui se trouve ailleurs. Ce qui est à l'autre bout de cette ligne d'alimentation — un émetteur-récepteur, une antenne mise à la masse, un préampli déjà alimenté — n'est pas devant vous pour être vérifié.
settings-radio-sdrplay-server-rsp-tcp = Serveur SDRplay (rsp_tcp)
settings-radio-sdrplay-server-greets-exactly-like = Un serveur SDRplay s'annonce exactement comme une clé, donc ces réglages sont toujours affichés plutôt que seulement lorsqu'un appareil est détecté. Face à un serveur rtl_tcp ordinaire, ils sont simplement ignorés — le protocole n'a aucune réponse et écarte les commandes qu'il ne connaît pas. L'onglet Périphérique désigne le serveur comme rsp_tcp dès qu'il s'est identifié.
settings-radio-input = Entrée A
settings-radio-input-b = Entrée B
settings-radio-hi-z = Haute impédance
settings-radio-lna-state = État LNA
settings-radio-step-index-not-db-figure = Un indice de pas, non un nombre de dB : ce que vaut chaque pas dépend du modèle RSP et de la bande. 0 est le gain le plus fort.
settings-radio-if-gain-reduction = Réduction du gain F.I.
settings-radio-reduction-so-more-less-signal = Une réduction, donc plus élevé signifie moins de signal. N'est respecté que lorsque le CAG du RSP est désactivé.
settings-radio-enable = Activer
settings-radio-level-rsp-s-agc-aims = Le niveau que le CAG du RSP cherche à maintenir.
settings-radio-notches = Coupe-bandes
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = Sortie de référence
settings-radio-t-24-mhz-clock-out = Sortie d'horloge 24 MHz
settings-radio-rsp2-rspduo-only = RSP2 et RSPduo uniquement.
settings-radio-receive-only-address-sample-rate = Réception seule. L'adresse et le taux d'échantillonnage prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez. Une connexion coupée est relancée d'elle-même, donc un serveur redémarré revient sans rien toucher ici.
settings-radio-where-spyserver-listening-address-address =
    Là où spyserver écoute : une adresse, ou une adresse et un port. Le port vaut 5555 par défaut, qui est la valeur propre à spyserver.

    De l'autre côté, vérifiez que son fichier de configuration lie une adresse que les autres machines peuvent atteindre — lié à 127.0.0.1, il n'accepte que les connexions venant de cette même machine.

    Prend effet à Appliquer.
settings-radio-host-host-port-e-g-2 = hôte ou hôte:port, p. ex. raspberrypi.local:5555
settings-radio-i-q-bandwidth = Largeur de bande I/Q
settings-radio-how-wide-slice-band-arrives =
    La largeur de la tranche de bande qui arrive en I/Q — et donc la part de la liaison qu'elle utilise. C'est la fenêtre *démodulée* : elle suit le cadran, et tout ce qui est plus large qu'elle est la FFT du serveur dans la bande au-dessus du panoramique.

    Automatique vise environ 96 kHz, ce qui porte tous les modes d'ici, y compris la FM large, et tient encore sur une liaison montante cellulaire. Prend effet à Appliquer.
settings-radio-which-stage-server-s-own =
    L'étage de l'échelle de taux propre au serveur à demander. Chaque récepteur a une échelle différente — son taux maximal divisé par deux à chaque étage — donc ceci est stocké comme l'étage plutôt que comme une valeur en hertz, et le même réglage garde un sens raisonnable face à un autre serveur.

    Automatique vise environ 1 Msps. Appuyez sur Tester la connexion pour voir ce que donnent les étages sur ce serveur. Prend effet à Appliquer.
settings-radio-automatic = Automatique
settings-radio-sample-format = Format d'échantillon
settings-radio-bits-per-component-wire-so =
    Bits par composante sur le fil, et donc ce que cela coûte sur la liaison : 16 bits coûtent le double de 8 bits, et le flottant 32 bits le quadruple, pour pas plus d'information que n'en avait le CAN du récepteur.

    8 bits est ce qui permet à un récepteur distant de fonctionner sur une liaison domestique, et c'est le bon choix pour presque tout. Un serveur peut imposer autre chose — certains sont configurés pour exiger un format — et le signale dans le journal lorsqu'il le fait. Prend effet à Appliquer.
settings-radio-gain = Gain
settings-radio-server-s-gain-stage-index =
    L'étage de gain du serveur, sous forme d'indice — non un nombre de décibels. Ce que signifie chaque indice appartient au récepteur de l'autre bout et change avec la bande ; le protocole ne le dit jamais, donc rien ici ne peut le convertir en dB sans inventer un chiffre.

    La plage réelle appartient au serveur et n'est connue qu'une fois connecté ; un indice au-delà est ramené à la limite. S'applique immédiatement — sans reconnexion.
settings-radio-index = indice{" "}
settings-radio-digital-gain = Gain numérique
settings-radio-how-far-server-scales-samples =
    De combien le serveur met à l'échelle ses échantillons avant de les quantifier pour le fil. Automatique surveille ce qui arrive réellement et maintient le crête à la moitié de la pleine échelle, ce qui est la seule chose qui puisse être juste à la fois sur une bande vide et sur une bande encombrée — les deux sont séparées de trente décibels et aucun chiffre fixe ne convient aux deux.

    Cela ne compte qu'à 8 bits, où un signal très en dessous de la pleine échelle perd ses bits de poids faible face au quantificateur, et où un signal au-dessus est écrêté à plat. Les formats plus larges ont de la marge et reçoivent la valeur que calcule tout autre client, à partir de l'indice de gain et de l'étage de décimation. S'applique immédiatement.
settings-radio-full-band-strip = Bande pleine largeur
settings-radio-server-s-own-fft-whole =
    La FFT propre au serveur sur toute la bande, dessinée dans la bande au-dessus du panoramique. Dans cette interface, c'est la seule vue de bande qui existe — le panoramique lui-même n'est large que de l'I/Q reçu.

    La désactiver laisse un récepteur sans moyen de voir quoi que ce soit auquel il n'est pas déjà accordé. S'applique immédiatement, avec une brève coupure de l'audio pendant que le serveur change ce qu'il envoie.
settings-radio-ask-server-low-rate-fft =
    Demander au serveur une FFT à faible débit de toute la bande en plus de l'I/Q, et la dessiner dans la bande au-dessus du panoramique.

    Cela ne coûte presque rien — quelques kilo-octets par trame, une douzaine de fois par seconde environ — et montre tout le récepteur plutôt que la tranche démodulée. À désactiver seulement sur une liaison où chaque octet compte. S'applique immédiatement, avec une brève coupure de l'audio.
settings-radio-show = Afficher
settings-radio-whole-band = toute la bande
settings-radio-how-much-receiver-strip-covers = La part du récepteur que couvre la bande. Toute la bande est la vue la plus large qui existe ; la rétrécir met le même nombre de cases sur un spectre plus étroit, ce qui donne un détail plus fin sur une portion plus petite.
settings-radio-strip-db-window = Fenêtre dB de la bande
settings-radio-server-quantises-fft-into-window =
    Le serveur quantifie sa FFT dans cette fenêtre avant de l'envoyer, un octet par case — donc ces réglages décident de la finesse de la mesure de la bande, pas seulement de son dessin. Le plancher et le plafond sous lesquels la bande est *affichée* sont réglés par la mise à niveau automatique du moteur et sont une autre chose.

    Les 150 dB par défaut sont toute la plage du protocole et ne demandent aucune attention. La resserrer autour du bruit de fond donne plus de résolution sur un récepteur dont tous les signaux se trouvent dans une petite partie de l'échelle. S'applique immédiatement.
settings-radio-top = haut{" "}
settings-radio-range-2 = plage{" "}
settings-radio-i-q-correction = Correction I/Q
settings-radio-remove-dc-spike-mirror-image = Supprime le pic continu et l'image miroir dans le DSP, de ce côté-ci. Le fait que le récepteur à l'autre bout en ait besoin dépend de ce qu'il est — un Airspy HF+ n'en a pas besoin, un RTL-SDR en a besoin — et le protocole ne dit pas à quel appareil il parle. S'applique immédiatement.
settings-radio-enabled = Activé
settings-radio-test-connection = Tester la connexion
settings-radio-connect-read-what-server-says = Se connecter, lire ce que le serveur dit de son récepteur, puis se déconnecter — sans démarrer de flux, donc on peut appuyer dessus face à un serveur qu'utilise quelqu'un d'autre.
settings-radio-receive-only-panadapter-narrow-i = Réception seule. Le panoramique est la fenêtre I/Q étroite, qui suit le cadran ; la vue de bande est la FFT du serveur dans la bande au-dessus. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-receive-only-wideband-i-q = Réception seule. I/Q large bande, comme pour toute SDR locale. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-receiver-address = Adresse du récepteur
settings-radio-where-receiver-serves-web-page =
    Là où le récepteur sert sa page web : une adresse, ou une adresse et un port. Le port vaut 8073 par défaut, qui est celui d'un KiwiSDR.

    Notez qu'un récepteur joint par le proxy du projet (quelque chose.proxy.kiwisdr.com — presque la moitié des publics) répond plutôt sur le port 80, donc indiquez-le explicitement. Choisir un récepteur dans « SDR publics » remplit ceci correctement.

    Prend effet à Appliquer.
settings-radio-host-host-port-e-g-3 = hôte ou hôte:port, p. ex. kiwi.example.org:8073
settings-radio-password = Mot de passe
settings-radio-receiver-s-user-password-where =
    Le mot de passe *utilisateur* du récepteur, lorsque son opérateur en a défini un. Vide sur presque tous les récepteurs publics. Pas le mot de passe administrateur.

    Prend effet à Appliquer.
settings-radio-announce = Annoncer comme
settings-radio-name-connection-shows-up-under =
    Le nom sous lequel cette connexion apparaît, que le propriétaire du récepteur et tous les autres auditeurs peuvent voir.

    Laissé vide, c'est l'indicatif de votre station, ce qui est la convention du réseau — certains opérateurs bloquent les clients qui ne s'identifient pas. Prend effet à Appliquer.
settings-radio-blank-station-callsign = vide = indicatif de la station
settings-radio-band-view = Vue de bande
settings-radio-ask-receiver-own-waterfall-well =
    Demander au récepteur sa propre cascade en plus de son I/Q, et la dessiner dans la bande au-dessus du panoramique.

    Cela vaut la peine : l'I/Q ne fait qu'environ 12 kHz de large, donc sans cela il n'y a rien pour accorder *par*. C'est une seconde connexion qui transporte environ 20 ko/s à pleine vitesse, contre 44 ko/s pour l'I/Q.

    Prend effet à Appliquer.
settings-radio-show-receiver-s-0-30 = afficher la cascade 0-30 MHz du récepteur
settings-radio-band-view-speed = Vitesse de la vue de bande
settings-radio-how-often-receiver-sends-waterfall =
    La fréquence à laquelle le récepteur envoie une ligne de cascade, de 1 (le plus lent) à 4. Le seul réglage ici qui modifie le coût de la liaison en fonctionnement, donc baissez-le sur une connexion facturée au volume.

    S'applique immédiatement.
settings-radio-band-view-span = Largeur de la vue de bande
settings-radio-how-much-band-receiver-s =
    La largeur de bande que couvre la cascade du récepteur. Sa pleine plage 0-30 MHz à gauche du curseur, divisée par deux à chaque cran vers la droite.

    La cascade fait toujours 1024 cases de large quelle que soit la fenêtre, donc c'est en réalité une commande de résolution : toute la bande fait 29 kHz par case, ce qui est une *carte* de bande plutôt qu'une image de quoi que ce soit, alors que quelques centaines de kilohertz montrent des stations individuelles dans une bande de radiodiffusion. Zoomé, la fenêtre suit votre cadran.

    Toute la bande est le réglage par défaut et c'est ce qui fait de la bande un outil d'accord ; le panoramique situé dessous est les ~12 kHz de l'I/Q. S'applique immédiatement.
settings-radio-receiver-agc = CAG du récepteur
settings-radio-receiver-s-agc-which-far =
    Le CAG du *récepteur*, qui se trouve de l'autre côté de la liaison et avant l'I/Q — donc il agit avant tout ce que fait sdroxide.

    Activé par défaut, à la différence de toutes les SDR locales ici, car l'alternative mesurée était pire : sur un récepteur en direct, le gain manuel n'était pas monotone sur sa plage et son extrémité haute écrêtait l'I/Q à pleine échelle, tandis que le CAG tenait environ -24 dBFS avec 7 dB de marge.

    Cela signifie que l'amplitude des échantillons n'est pas un niveau de signal, ce qui explique pourquoi le S-mètre est lu sur le chiffre propre au récepteur. S'applique immédiatement.
settings-radio-let-receiver-ride-own-gain = laisser le récepteur gérer son propre gain
settings-radio-manual-gain = Gain manuel
settings-radio-fixed-gain-when-receiver-s = Gain fixe lorsque le CAG du récepteur est désactivé, sur sa propre échelle de 0-90. Ce ne sont pas des décibels de quoi que ce soit d'annoncé — le protocole l'appelle manGain et n'en dit pas davantage, et il a été mesuré comme non monotone, donc traitez-le comme un cadran pour trouver un bon point plutôt que comme un étalonnage. S'applique immédiatement.
settings-radio-read-receiver-s-status-page = Lit la page d'état du récepteur : ce qu'il est, ce qu'il couvre, combien de canaux sont libres, et si son opérateur autorise les connexions d'autres applications qu'un navigateur. N'occupe aucun de ses canaux.
settings-radio-host-port-e-g-127 = hôte:port, p. ex. 127.0.0.1:50001
settings-radio-iq-sample-rate = Taux d'échantillonnage IQ
settings-radio-receiver = Récepteur
settings-radio-rig-two-receivers-sunsdr2dx-can = Un appareil à deux récepteurs (SunSDR2DX) peut servir deux onglets radio à partir d'une seule connexion — faites tourner cette radio sur RX1 et une autre sur RX2. L'émetteur appartient à la radio RX1.
settings-radio-stream-delay = Retard du flux
settings-radio-how-long-rig-s-iq = Le temps que met l'I/Q de l'appareil pour arriver sur un nouveau centre après que sdroxide l'a déplacé. C'est le pipeline DSP de l'appareil lui-même, non le réseau — un serveur sur cette machine en a un aussi — et pendant que le panoramique est glissé entièrement dézoomé, se tromper de cette quantité déplace les lignes les plus récentes de la cascade de côté par rapport à l'historique, de l'erreur multipliée par la vitesse du glissement. Trop haut les déplace exactement autant que trop bas, dans l'autre sens. Mesurez-le avec `cargo run --release -p sdroxide-tci --example retune_latency` ; la valeur par défaut est celle d'un SunSDR2DX sur ExpertSDR3 à 192 kHz.
settings-radio-wideband-iq-receive-audio-transmit = I/Q large bande en réception, audio en émission. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-radio-address = Adresse de la radio
settings-radio-address-shown-radio-under-set = L'adresse affichée sur la radio sous SET > Network. Le contrôle réseau doit y être activé, et la radio doit avoir un nom d'utilisateur et un mot de passe réseau définis.
settings-radio-host-ip-e-g-192 = hôte ou IP, p. ex. 192.168.1.50
settings-radio-control-port = Port de commande
settings-radio-t-50001-unless-has-been-changed = 50001 sauf s'il a été modifié sur la radio.
settings-radio-network-user = Utilisateur réseau
settings-radio-stored-clear-radio-json-protocol = Stocké en clair dans radio.json. Le protocole le brouille de façon réversible sur le fil, donc rien ici n'en ferait un secret.
settings-radio-receive = Recevoir depuis
settings-radio-audio-sample-rate = Taux d'échantillonnage audio
settings-radio-t-12-khz-if-needs-48000 = Une FI 12 kHz exige le flux à 48000 Hz — à ce taux, c'est l'audio démodulé de la radio qui est utilisé.
settings-radio-if-spectrum = Spectre FI
settings-radio-which-way-round-radio-s = Dans quel sens la FI 12 kHz de la radio est tournée. « Automatique » utilise ce que l'on sait du modèle — inversée sur un IC-7760, normale sur tous les autres Icom. Réglez-la à la main si la BLU sort sur la mauvaise bande latérale : le signe est d'être obligé de sélectionner USB là où la bande est en LSB, alors que l'affichage de mode de la radio concorde avec sdroxide en permanence.
settings-radio-normal = Normale
settings-radio-mirrored = Inversée
settings-radio-displayed-bandwidth = Largeur affichée
settings-radio-width-audio-band-panadapter-cat = Largeur du panoramique de bande audio, comme pour un appareil CAT. Utilisée dans les modes numériques et lorsque l'oscilloscope de la radio est désactivé ; sinon le panoramique est l'oscilloscope, et la Largeur de l'oscilloscope en fixe la largeur.
settings-radio-how-cw-panel-s-keyer-2 = Comment le manipulateur du panneau CW émet. « Manipulateur de la radio » met la radio en CW et lui remet le texte à envoyer avec son propre manipulateur (CI-V 0x17). « Carte son » envoie à la place la manipulation sous forme de signal latéral audio par le réseau (MCW), un ton à cadran + tonalité — et comme une radio en CW ignorerait son entrée de modulateur, le choix de CW maintient alors la radio en USB simple, le même mode que celui des modes numériques.
settings-radio-how-much-audio-radio-holds = La quantité d'audio que la radio retient avant de moduler. Plus cela est grand, plus cela survit à un mauvais réseau, au prix de la latence à l'émission.
settings-radio-streams-radio-s-own-475 = Diffuse le balayage propre à la radio, de 475 cases. En AF c'est le panoramique — l'audio que la radio envoie est ce qui est passé par son filtre, non une image de la bande — et sur la FI 12 kHz c'est la cascade pleine bande au-dessus d'un tel balayage. Dans les deux cas c'est l'image de la radio, non le DSP de sdroxide : il n'y a pas d'I/Q à partir duquel en calculer une.
settings-radio-how-wide-sweep-only-wide = La largeur du balayage. C'est la seule vue large qu'un Icom possède, et la radio conserve la largeur choisie en dernier sur son propre écran — souvent quelques kHz, ce qui explique que la bande ne puisse pas s'afficher plus large que le panoramique qui se trouve dessous. Régler une largeur ici place aussi l'oscilloscope en mode centré, donc il suit le cadran. Cela modifie aussi l'affichage propre à la radio ; « Comme réglé sur la radio » le laisse tranquille.
settings-radio-switch-modulation-input-lan = Basculer l'entrée de modulation sur LAN
settings-radio-transmit-audio-only-heard-when = L'audio d'émission n'est entendu que lorsque l'entrée MOD de la radio est réglée sur LAN. sdroxide peut l'écrire sur un modèle dont il connaît la numérotation des menus ; sur tout autre, il le signale et laisse le menu tranquille. C'est un prêt : ce que la radio contenait à cet endroit est lu d'abord, puis remis en place à la fin de la session, si bien qu'un appareil utilisé seul ensuite entend toujours son propre microphone.
settings-radio-copy-diagnostic-report = Copier le rapport de diagnostic
settings-radio-control-audio-radio-s-own = Commande, audio et oscilloscope propre à la radio sur une seule connexion réseau — ni câble série ni carte son. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-testing = Test…
settings-radio-was-only-check-press-apply = Ce n'était qu'une vérification — appuyez sur Appliquer / reconnecter ci-dessous pour commencer à utiliser cette radio.
settings-radio-radios = Radios
settings-radio-asks-network-iio-devices-also = Interroge le réseau à la recherche de périphériques IIO, et essaie aussi 192.168.2.1 directement — un Pluto relié par câble USB est souvent injoignable par multidiffusion, bien que l'adresse fonctionne.
settings-radio-no-radios-press-discover = aucune radio — appuyez sur Découvrir
settings-radio-address = Adresse
settings-radio-overrides-selection-above-usb-cable = Remplace la sélection ci-dessus. Le câble USB présente le Pluto comme une carte réseau, non comme un port série, donc c'est une adresse IP même lorsque la radio est sur votre bureau.
settings-radio-pluto-revision-c-pluto-unlocked = Un Pluto+ ou un Pluto révision C déverrouillé en 2R2T peut servir deux onglets radio depuis un seul boîtier — cette radio sur RX1 et une autre sur RX2, chacune sur sa propre antenne. Les deux chaînes partagent un même oscillateur, donc accorder l'une déplace les deux ; ce que RX2 apporte est une seconde antenne sur le même spectre (diversité), non une seconde bande. L'émetteur appartient à la radio RX1. Un Pluto 1R1T d'origine refuse RX2 à la connexion.
settings-radio-width-spectrum-sdroxide-receives-ad9361 = Largeur du spectre que reçoit sdroxide. Le AD9361 atteint 61,44 Msps ; la liaison réseau USB non, et c'est à elle que cette liste est calibrée. Les taux les plus bas exigent une configuration de filtre chargée dans le AD9361, ce que sdroxide ne fait pas — un Pluto d'origine les fait tourner à environ 2,084 Msps à la place, et le signale à la connexion. Prend effet à Appliquer.
settings-radio-more-than-usb-2-will = {"  "}(plus que ce que porte l'USB 2)
settings-radio-stock-pluto-runs-2-084 = {"  "}(un Pluto d'origine tourne à 2,084)
settings-radio-analog-filter = Filtre analogique
settings-radio-ad9361-s-baseband-filter-leave = Le filtre de bande de base du AD9361. Laissez 0 pour l'automatique, qui l'ouvre aux neuf dixièmes du taux d'échantillonnage — large à dessein, car le récepteur place son oscillateur à un quart de la largeur du cadran pour garder les signaux à l'écart du pic continu, et un filtre étroit couperait justement la partie vers laquelle il les a déplacés. Prend effet à Appliquer.
settings-radio-ad9361-has-four-modes-not = Le AD9361 a quatre modes, non un interrupteur marche/arrêt. Attaque lente convient à la BLU et à la CW ; attaque rapide aux signaux par rafales ; manuel est le réglage pour la mesure et les modes numériques en signaux faibles. S'applique immédiatement.
settings-radio-rx-gain = Gain RX
settings-radio-applies-immediately-no-reconnect-ignored = S'applique immédiatement — sans reconnexion. Ignoré sauf si le CAG est réglé sur manuel, ce qui est le comportement propre au AD9361, non celui de sdroxide.
settings-radio-tx-gain = Gain TX
settings-radio-negative-because-ad9361-expresses-transmit = Négatif parce que le AD9361 exprime le niveau d'émission en atténuation : 0 dB est la sortie pleine. S'applique immédiatement. L'émetteur est mis à son niveau le plus bas avant que cette valeur ne soit appliquée à la connexion, pour que rien de ce que le programme précédent a laissé ne puisse être actif.
settings-radio-reference-error-parts-per-million = Erreur de référence en parties par million. Lancez avec RUST_LOG=sdroxide_pluto=debug et le journal affiche l'erreur d'horloge mesurée après environ 20 secondes — c'est ce nombre-là qu'il faut saisir. Appliquée par sdroxide, non écrite dans le réglage fin persistant de la radio. S'applique immédiatement.
settings-radio-full-duplex = Duplex intégral
settings-radio-keep-receiving-while-transmitting = Continuer à recevoir pendant l'émission
settings-radio-leave-off-pluto-reached-over =
    Laissez ceci désactivé sur un Pluto joint par son câble USB. Cette liaison ne peut pas porter un mégaéchantillon par seconde dans les deux sens à la fois, et un trafic qui affame le tampon d'émission part haché — donc par défaut la réception s'arrête pendant la durée d'un trafic.

    Activez-le pour une carte sur un vrai Ethernet (un LibreSDR, un Pluto sur un adaptateur gigabit), où il y a de la place pour les deux : vous entendez alors le récepteur pendant votre propre émission, ce qui est la façon dont une station QO-100 écoute sa propre descente.

    Un trafic demande deux fois le taux d'échantillonnage en débit de liaison — 2,5 Msps représentent 10 Mo/s dans chaque sens — donc baissez le taux si le journal signale que la liaison ne le porte pas. Le panoramique montre toujours le signal émis pendant un trafic ; c'est l'audio qui continue d'arriver.

    Prend effet à Appliquer.
settings-radio-duplex = Duplex
settings-radio-whether-ad9361-runs-both-directions =
    Si le AD9361 fonctionne dans les deux sens à la fois (FDD, qui est l'état dans lequel démarre un Pluto et dans lequel sdroxide l'a toujours laissé) ou l'un après l'autre (TDD).

    Laissez ceci sur FDD sauf si vous voulez les broches PTT ci-dessous — c'est le TDD qui les commande, et il exclut le Duplex intégral ci-dessus. Prend effet à Appliquer.
settings-radio-ptt-pins = Broches PTT
settings-radio-pluto-s-four-gpo-test =
    Les quatre points de test GPO du Pluto peuvent à eux seuls commander un amplificateur de puissance externe, un LNA ou un commutateur émission-réception. Choisissez une paire : une broche est à l'état haut pendant toute la réception, l'autre pendant toute l'émission — ni logiciel hôte dans la boucle, ni ligne PTT série à câbler.

    Cela met la radio en TDD quelle que soit la ligne Duplex, car les broches suivent les lignes d'activation du AD9361, et en FDD les deux sont actives pendant toute la session. Le Duplex intégral s'éteint avec lui, donc laissez les deux à leur valeur par défaut si vous travaillez les satellites et devez entendre votre propre descente.

    Les notes d'Analog Devices placent un LNA externe sur GPO0/GPO1, donc utilisez GPO2/GPO3 si votre carte est câblée ainsi. Les broches sont à environ 1,3 V pour quelques milliampères : pilotez un transistor ou un opto-isolateur avec elles, jamais une bobine de relais.

    Prend effet à Appliquer.
settings-radio-buffer-size = Taille du tampon
settings-radio-how-much-device-holds-before =
    Ce que retient le périphérique avant chaque transfert, en échantillons complexes. La valeur par défaut de 32768 représente environ 16 ms à 2 Msps : assez longue pour que l'aller-retour de chaque transfert ne soit pas le goulot d'étranglement, assez courte pour qu'un changement d'accord ne soit pas visiblement en retard.

    Divisez-la par deux si le journal signale le remplacement du socket de réception. Une liaison limite — un Pluto joint par son câble USB à un taux d'échantillonnage élevé est le cas habituel — se bloque en plein transfert, et un transfert plus petit est à la fois moins susceptible d'être interrompu et plus rapide à rétablir ensuite. Augmentez-la pour échanger la latence d'accord contre moins d'allers-retours.

    Prend effet à Appliquer.
settings-radio-samples = {" "}échantillons
settings-radio-rx-tx-port = Port RX / TX
settings-radio-ad9361-s-rf-port-select = Le rf_port_select du AD9361. Un Pluto d'origine câble un de chaque, donc laissez ceci vide sauf si vous avez une carte qui ne le fait pas. Prend effet à Appliquer.
settings-radio-opens-radio-reads-what-says = Ouvre la radio, lit ce qu'elle dit d'elle-même, et annonce la plage d'accord de cette carte-ci. Ne démarre pas de flux.
settings-radio-wideband-iq-receive-transmit-half = I/Q large bande en réception et en émission. Demi-duplex — la réception s'arrête pendant la durée d'un trafic, car la liaison réseau USB ne peut pas porter les deux à la fois. Ni SoapySDR ni libiio nécessaires.
settings-radio-flexradio-announces-itself-local-network = Une FlexRadio s'annonce sur le réseau local environ une fois par seconde. Une radio jointe par un routeur ou un VPN ne vous diffuse jamais rien — saisissez son adresse ci-dessous à la place.
settings-radio-another-gui-client-has-radio = Un autre client graphique utilise cette radio et multiFLEX est désactivé. Déconnectez ce client, ou activez multiFLEX sur la radio.
settings-radio-overrides-selection-above-use-radio = Remplace la sélection ci-dessus. À utiliser pour une radio sur un autre sous-réseau, derrière un VPN, ou sur un port non standard.
settings-radio-width-spectrum-sdroxide-receives-192 = Largeur du spectre que reçoit sdroxide. 192 kHz est le maximum de la radio pour un flux DAX IQ, donc la plus large largeur que cette interface peut afficher.
settings-radio-t-192-khz-widest-dax-iq = 192 kHz est le flux DAX IQ le plus large qu'une FLEX enverra, et la radio lie la largeur du panoramique à celui-ci — c'est donc toute la largeur, et aucun réglage ici ne l'élargit.
settings-radio-dax-iq-channel = Canal DAX IQ
settings-radio-radio-has-four-change-only = La radio en a quatre. Ne changez ceci que si quelque chose d'autre sur le réseau utilise déjà le canal 1 — la radio refuse un canal en double.
settings-radio-station-name = Nom de la station
settings-radio-shown-against-session-radio-s = Affiché pour cette session dans la liste des clients de la radio. La radio s'en sert aussi pour reconnaître un client, donc le renommer fait que la radio traite sdroxide comme un nouveau client.
settings-radio-gui-client-id = Identifiant du client graphique
settings-radio-identity-radio-restores-session-s = L'identité à laquelle la radio rétablit les tranches et les panoramiques de cette session. Laissé vide, il est dérivé du nom de la station — stable, mais identique sur chaque sdroxide qui a gardé le nom par défaut, et une radio règle un doublon en éjectant le client précédent. sdroxide le repère sur le fil et revient à une identité propre à la session, ce qui coûte la restauration ; mettez ici un UUID de votre choix pour la conserver.
settings-radio-optional-e-g-uuid-your = facultatif, p. ex. un UUID de votre choix
settings-radio-mirror-radio-s-i-q =
    Symétrise l'I/Q de la radio autour du centre du panoramique. Désactivé par défaut, c'est ainsi qu'un FLEX-6600 a été vérifié.

    Essayez-le si la réception audio est inintelligible en USB *et* en LSB et que rien ne se décode, alors que la cascade paraît tout à fait convaincante — c'est à cela que ressemble un flux symétrisé, et c'est le seul défaut sans autre symptôme (ticket n° 368). Si ce n'est pas cela, activer ceci le rend évident plutôt que subtil. S'applique à Appliquer / reconnecter.
settings-radio-network-mtu = MTU réseau
settings-radio-largest-datagram-radio-may-send = Le plus grand datagramme que la radio peut envoyer. 1450 est ce que demande SmartSDR lui-même. Baissez-le sur un chemin à MTU plus petit — un VPN ou un tunnel — où les fragments sont perdus et où aucun spectre n'arrive du tout.
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = Vérifie que la radio répond, sans s'enregistrer comme client graphique — donc cela ne perturbera pas une session SmartSDR déjà en cours.
settings-radio-wideband-iq-receive-over-dax = I/Q large bande en réception par DAX, audio d'émission que la radio module. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-not-yet-verified-against-real-2 = Pas encore vérifié sur du matériel réel. En cas de comportement anormal, envoyez le rapport de diagnostic — il contient chaque ligne de protocole échangée avec la radio.
settings-radio-devices-soapysdr-can-see = Périphériques que SoapySDR voit
settings-radio-waiting-machine-radio-attached-where = En attente de la machine à laquelle la radio est reliée, là où les modules sont installés. Le périphérique ouvert est `device_args` dans le config.toml de cette machine, ou son --device.
settings-radio-ask-every-installed-soapysdr-module = Demande à chaque module SoapySDR installé de chercher. Rien n'est ouvert, donc c'est sans danger pendant la réception — mais cela peut prendre un moment.
settings-radio-not-enumerated-yet-press-rescan = Pas encore énuméré — appuyez sur Rechercher.
settings-radio-no-soapysdr-devices-found-check = Aucun périphérique SoapySDR trouvé. Vérifiez que le module de votre radio est installé et que vous avez le droit d'accéder au périphérique.
settings-radio-not-radio = pas une radio
settings-radio-sound-card-listed-above-if = Une carte son figure ci-dessus comme si c'était une SDR. Elle accepte n'importe quelle fréquence et l'ignore, donc l'ouvrir affiche l'entrée de la carte son au lieu de la bande. sdroxide ne les choisit pas automatiquement — mais une ligne device_args qui en désigne une est tout de même obéie.
settings-radio-which-one-opens-set-device = Celui qui s'ouvre se règle par --device ou device_args dans config.toml ; sans l'un ni l'autre, c'est la première radio listée qui l'emporte.
settings-radio-no-device = aucun périphérique
settings-radio-rig-has-no-software-adjustable = Cet appareil n'a aucun gain réglable par logiciel.
settings-radio-rx-gains = Gains RX
settings-radio-tx-gains = Gains TX
settings-radio-antennas = Antennes
settings-radio-remembered-next-start = Mémorisé pour le prochain démarrage.
settings-radio-these-controls-appear-once-soapysdr = Ces commandes apparaissent une fois qu'un périphérique SoapySDR est ouvert — ce sont celles du périphérique, et il faut le lui demander.
settings-radio-stream = Flux
settings-radio-follow-app-setting = Suivre le réglage de l'application
settings-radio-msps = ou, en Msps
settings-radio-baseband-filter = Filtre de bande de base
settings-radio-let-driver-choose = Laisser le pilote choisir
settings-radio-changing-either-reopens-radio = Modifier l'un ou l'autre rouvre la radio.
settings-radio-driver-publishes-no-settings-own = Ce pilote ne publie aucun réglage qui lui soit propre.
settings-radio-applied-once-remembered-next-start = Appliqué aussitôt, et mémorisé pour le prochain démarrage.
settings-radio-no-rx-888-found-press = Aucun RX-888 trouvé — appuyez sur Rechercher
settings-radio-adc-clock = Horloge du CAN
settings-radio-t-129-6-msps-needs-superspeed = 129,6 Msps exige une liaison SuperSpeed et un hôte rapide ; 64,8 est la valeur sûre par défaut. Modifier ce réglage rouvre automatiquement le récepteur, ce qui prend un instant mais ne demande aucun redémarrage.
settings-radio-panadapter-width = Largeur du panoramique
settings-radio-hf-only = {"  "}· HF uniquement
settings-radio-usable-both-front-ends = Utilisable sur les deux étages d'entrée.
settings-radio-below-vhf-crossover-ordinary-width = Sous la limite VHF, c'est une largeur ordinaire. Au-dessus, la FI de 8 MHz de l'accord ne peut pas être centrée dans une fenêtre aussi large : le spectre en direct se place d'un côté du panoramique, avec du vide à côté.
settings-radio-dial-above-crossover-now-so = Le cadran est maintenant au-dessus de la limite, donc cette largeur affiche le 8 MHz de l'accord décentré avec du spectre mort à côté. Réduisez la largeur pour remplir le panoramique.
settings-radio-vga-gain = Gain VGA
settings-radio-ad8370-variable-gain-amplifier-ahead = Amplificateur à gain variable AD8370 placé devant le CAN.
settings-radio-attenuator = Atténuateur
settings-radio-pe4304-step-attenuator-0-5 = Atténuateur à pas PE4304, par pas de 0,5 dB.
settings-radio-adc-range = Plage du CAN
settings-radio-wide-2-25-vp-p = Large (2,25 Vc-c)
settings-radio-selects-adc-s-wider-input = Sélectionne la plage d'entrée la plus large du CAN : plus de marge pour les forts signaux de radiodiffusion, moins de graduations pour les faibles. Désactivé sélectionne la plage 1,5 Vc-c, plus sensible.
settings-radio-dither = Dither
settings-radio-adds-small-dither-signal-ahead = Ajoute un petit signal de dither devant le CAN : coûte un peu de bruit de fond, rapporte de la dynamique libre de parasites.
settings-radio-randomiser = Randomiseur
settings-radio-adc-scrambles-output-so-digital = Le CAN brouille sa sortie pour que le bus numérique cesse de rayonner dans l'étage d'entrée ; le pilote la débrouille. Laissez ceci activé sauf si vous déboguez. S'applique à la reconnexion.
settings-radio-dc-hf-antenna-port = Courant continu sur la prise antenne HF
settings-radio-powers-active-antenna-preamp-down = Alimente une antenne active ou un préampli par le câble coaxial.
settings-radio-vhf-tuner-gain = Gain de l'accord VHF
settings-radio-r828d-rf-gain-used-above = Gain RF du R828D, utilisé au-dessus de la limite automatique HF/VHF. 29 pas discrets ; le plus proche est utilisé.
settings-radio-auto = Auto
settings-radio-let-tuner-run-own-lna = Laisser l'accord faire tourner ses propres boucles LNA et mélangeur.
settings-radio-bias-tee-vhf = Té de polarisation (VHF)
settings-radio-dc-vhf-antenna-port = Courant continu sur la prise antenne VHF
settings-radio-clock-trim = Réglage de l'horloge
settings-radio-corrects-reference-oscillator-applied-when = Corrige l'oscillateur de référence. Appliqué lorsque vous relâchez la valeur — rouvrir à chaque pixel d'un glissement redémarrerait le récepteur des centaines de fois.
settings-radio-receive-only-below-adc-s = Réception seule. Sous la limite de Nyquist du CAN, l'antenne est échantillonnée directement et l'accord est instantané, car il n'y a pas de convertisseur matériel — tout le flux du CAN est converti en bande de base par l'hôte. Au-dessus, le récepteur bascule automatiquement sur son accord R828D et la prise SMA VHF, donc les deux antennes doivent être branchées. La VHF exige une horloge de CAN d'au moins 32,4 Msps pour que la FI de l'accord tienne, et à des horloges inférieures à 48 Msps il existe un trou entre les deux plages que rien ne peut atteindre. Chaque réglage ici s'applique tout de suite — il n'y a pas de bouton Appliquer à presser.
settings-radio-device = Périphérique
settings-radio-no-devices-press-rescan = aucun périphérique — appuyez sur Rechercher
settings-radio-six-rates-are-six-different =
    Les six taux sont six images FPGA différentes, non un registre — c'est pourquoi rien dans le protocole d'ELAD ne choisit entre elles, et pourquoi ce que signifie ce réglage dépend du modèle.

    FDM-S1 / FDM-S2 : une commande. Leur FPGA est chargé par l'ordinateur à chaque mise sous tension, et sdroxide lance le chargeur elad-firmware d'ELAD pour y mettre l'image de ce taux. Sans ce chargeur installé, un échantillonneur n'envoie aucun échantillon, et sdroxide le signale à l'écran.

    FDM-DUO : pas une commande. La radio démarre son propre FPGA et arrive au taux dans lequel elle s'est allumée ou que FDM-SW2 lui a laissé en dernier (192 kHz sur une radio neuve), et ceci indique seulement comment le flux est LU. Une erreur et le panoramique est simplement de la mauvaise largeur, chaque fréquence à l'intérieur étant mise à l'échelle en conséquence.

    Le taux réel du flux est mesuré quelques secondes après son démarrage, et un écart est signalé à l'écran. Prend effet à Appliquer.
settings-radio-fdm-s-fpga-image-loaded = FDM-S : l'image FPGA chargée — FDM-DUO : comment le flux est lu
settings-radio-receiver-s-input-pad-one = L'atténuateur d'entrée du récepteur — un pas, en plus ou en moins. C'est la même commande que le curseur Gain de la fenêtre principale.
settings-radio-pre-selection-filters = Filtres de présélection
settings-radio-low-pass-bank-front-adc = Le banc de filtres passe-bas devant le CAN. Le contourner donne la vue la plus large et le pire comportement près des forts signaux hors bande — un émetteur de radiodiffusion à quelques kilomètres placera des images sur toute la bande. Laissez-le activé sauf si vous écoutez volontairement en dehors de la plage filtrée.
settings-radio-filters-circuit = Filtres dans le circuit
settings-radio-rig-control-fdm-duo-only = Commande de l'appareil — FDM-DUO uniquement
settings-radio-transceiver-s-cat-usb-port = Le port USB CAT de l'émetteur-récepteur, qui est un câble distinct de celui de réception. Laissez le port non défini sur un FDM-S1 ou un FDM-S2, qui n'en ont pas — et sur un FDM-DUO, mieux vaut piloter par le seul câble de réception, où il peut toujours être accordé et manipulé, mais aucun wattmètre ne peut être lu.
settings-radio-port-set-radio-s-display = Lorsque le port est renseigné, l'affichage de la radio et ce cadran sont le même nombre dans les deux sens : la fenêtre de réception du FDM-DUO est construite autour de son VFO, donc accorder ici déplace la face avant, et tourner le bouton de la face avant déplace l'affichage ici. Le panoramique se recentre sur le cadran au fil de l'accord. Sans port — ou avec un port auquel la radio ne répond pas — il n'y a aucun VFO à commander, et l'accord reste dans la fenêtre que la radio envoie déjà.
settings-radio-none-usb-control-only = — aucun (commande USB seulement) —
settings-radio-must-match-menu-70-cat = Doit correspondre au menu 70 « CAT BAUD » de la radio, qui est livrée à 38400. La radio n'a que ces quatre débits, et un port ouvert à un autre est muet dans les deux sens — la radio ignore le cadran et refuse de manipuler, sans rien pour en dire la raison.
settings-radio-how-transmit-keyed-cat-sends = Comment l'émission est déclenchée. « CAT » envoie la commande TX propre à la radio et ne demande aucun réglage sur l'appareil ; RTS exige que le menu 54 « PTT » soit réglé sur PTT+RTS.
settings-radio-where-radio-takes-transmit-audio-2 =
    D'où la radio prend son audio d'émission — la commande TI de l'appareil, qui est le menu 32 « TX IN » sur la face avant. Envoyé à l'ouverture du port.

    « Audio USB » est ce qui fait fonctionner l'émission ici : le FDM-DUO émet ce que sdroxide met dans sa carte son USB. Une radio laissée sur « Microphone » émet à la place ce qui se dit dans la pièce, et rien à l'écran ne le signale.
settings-radio-which-two-sockets-back-receiver-2 =
    Laquelle des deux prises à l'arrière le récepteur écoute — la commande AN de l'appareil, qui est le menu 31 « ANTENNAS » sur la face avant et l'indicateur « ANT 1 2 » sur son afficheur.

    « RTX » est une antenne qui remplit les deux fonctions, sur la prise de type M qui porte aussi l'émission. « RX seul » fait passer la réception sur la seconde prise et laisse l'émission sur RTX — une antenne de réception, une boucle ou une Beverage, le faisceau restant sur l'émetteur.

    Cela déplace tout ce récepteur : le panoramique, les démodulateurs et l'audio propre de l'appareil viennent tous de la prise choisie ici.

    S'applique immédiatement, et est relu depuis la radio à l'ouverture du port de commande — c'est le réglage propre à l'appareil, non une copie tenue ici.
settings-radio-how-often-radio-asked-what-2 = La fréquence à laquelle la radio est interrogée sur ce qu'elle fait — son cadran, son mode et ses indicateurs. Tournez le bouton VFO de l'appareil et l'affichage suit en un cycle d'interrogation. C'est aussi tout le trafic de commande produit de ce côté, donc baissez-le si l'audio de la radio coupe.
settings-radio-transmit-audio = Audio d'émission
settings-radio-only-receiving-fdm-duo-has = Seule la réception sur un FDM-DUO a été testée sur du matériel réel jusqu'ici. Si quelque chose se comporte mal, joignez le rapport de diagnostic à un rapport de bogue.
settings-radio-cw-keyed-radio-s-own = La CW est manipulée par la manipulation ou la palette propre à la radio : le FDM-DUO n'a aucune commande qui accepte du texte, donc le panneau CW ne peut pas le manipuler.
settings-radio-press-apply-reopen-radio = Appuyez sur Appliquer pour rouvrir la radio.
settings-radio-no-receivers-press-rescan = aucun récepteur — appuyez sur Rechercher
settings-radio-which-rates-receiver-has-depends = Les taux dont dispose un récepteur dépendent du modèle et du micrologiciel. Prend effet à Appliquer.
settings-radio-every-rate-any-hf-model = tous les taux qu'offre un modèle HF+ — en connecter un pour voir le sien
settings-radio-receiver-s-own-gain-control = La commande de gain propre au récepteur. Laissez-la activée pour l'écoute courante ; désactivez-la pour régler l'atténuateur à la main, pour la mesure.
settings-radio-agc-threshold = Seuil du CAG
settings-radio-high = Élevé
settings-radio-high-trades-little-sensitivity-headroom = Élevé échange un peu de sensibilité contre de la marge face aux voisins puissants — le bon réglage sur une bande encombrée, le soir.
settings-radio-front-end-attenuation-gain-0 = Atténuation d'entrée, exprimée en gain — 0 dB signifie aucune. Respecté seulement lorsque le CAG est désactivé.
settings-radio-preamp = Préampli
settings-radio-hf-low-noise-amplifier-buys = L'amplificateur HF à faible bruit. Il apporte de la sensibilité au prix de l'intermodulation, donc il est désactivé par défaut — ce qui est en général le bon réglage sur une vraie antenne.
settings-radio-lna = LNA activé
settings-radio-frequency-calibration = Étalonnage de la fréquence
settings-radio-parts-per-billion-receiver-s = Parties par milliard — l'unité propre à ce récepteur, mille fois plus fine que le ppm qu'utilise un RTL-SDR. Rien ici n'est jamais écrit dans la mémoire flash du récepteur : cela remplace la valeur enregistrée pour la seule session.
settings-radio-use-receiver-s-stored-value = Utiliser la valeur enregistrée du récepteur
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = Alimenter en continu par le câble coaxial
settings-radio-not-every-hf-has-one = Tous les HF+ n'en ont pas ; sur un récepteur qui n'en a pas, ceci ne fait rien.
settings-radio-host-dsp = DSP de l'hôte
settings-radio-image-balancer-zero-if-offset = L'équilibreur d'image, le décalage zéro-FI et l'oscillateur de réglage fin. Ne le désactivez que pour voir la sortie brute du matériel — désactivé, l'image miroir apparaît sur les taux à zéro-FI et l'accord n'est juste qu'au kilohertz près.
settings-radio-correct-image-fine-tune = Corriger l'image et régler finement
settings-radio-receive-only-no-soapysdr-no = Réception seule. Ni SoapySDR ni libairspyhf nécessaires. Sous le plancher du synthétiseur, c'est l'oscillateur de l'hôte qui accorde, et c'est ainsi que ce récepteur atteint le VLF. Le récepteur et le taux d'échantillonnage prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez.
settings-radio-not-yet-verified-against-real-3 = Pas encore vérifié sur du matériel réel. En cas de comportement anormal, envoyez le rapport de diagnostic — il contient chaque commande échangée avec le récepteur, et les premiers octets du flux d'échantillons décodés en paires I/Q.
settings-radio-no-airspy-r2-mini-found = Aucun Airspy R2 ni Mini trouvé — appuyez sur Rechercher
settings-radio-read-receiver = lire depuis ce récepteur
settings-radio-r2-mini-offer-different-rates = un R2 et un Mini proposent des taux différents et ne peuvent pas être distingués tant que l'un n'est pas ouvert
settings-radio-rate-you-get-receiver-s = C'est le taux que vous obtenez. Le CAN du récepteur fonctionne au double — il numérise un signal réel, et sdroxide en fait une bande de base complexe sur l'hôte.
settings-radio-gain-curve = Courbe de gain
settings-radio-step-along-curve-above-not = Un pas le long de la courbe ci-dessus, non une valeur en dB — le LNA, le mélangeur et le VGA de l'accord bougent ensemble, et ce que vaut chaque pas dépend de la courbe et de la bande. 0 est l'extrémité silencieuse.
settings-radio-tuner-agc = CAG de l'accord
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = Confie le LNA à la boucle propre de l'accord, en remplaçant la courbe.
settings-radio-mixer = Mélangeur
settings-radio-same-mixer-stage = Idem pour l'étage mélangeur.
settings-radio-loop-running-gain-slider-no = Une boucle étant active, le curseur de gain ne règle plus l'étage qu'elle possède — la boucle l'écrase.
settings-radio-dc-antenna-port = Courant continu sur la prise antenne
settings-radio-t-12-bit-packing = Compactage 12 bits
settings-radio-third-less-usb-traffic-leave = Un tiers de trafic USB en moins. Laissez-le activé : c'est un appareil USB 2.0 et le taux maximal est de 30 Mo/s compacté contre 40 non compacté. S'applique à la reconnexion.
settings-radio-dc-removal = Suppression du continu
settings-radio-remove-adc-s-offset = Supprimer le décalage du CAN
settings-radio-turn-off-see-raw-hardware = Désactivez-le pour voir la sortie brute du matériel. Il est bon de savoir où va la raie : le décalage tombe au bord de la largeur, et non en son centre, car le signal est translaté d'un quart du taux d'échantillonnage au passage.
settings-radio-every-command-exchanged-receiver-sample = Chaque commande échangée avec le récepteur, le calcul des taux d'échantillonnage, et les premiers échantillons décodés en I/Q.
settings-radio-receive-only-24-1800-mhz = Réception seule, 24–1800 MHz. Ni SoapySDR ni libairspy nécessaires. Le récepteur, le taux d'échantillonnage et le compactage prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez.
settings-radio-not-yet-verified-against-real-4 = Pas encore vérifié sur du matériel réel. En cas de comportement anormal, envoyez le rapport de diagnostic — il contient chaque commande échangée avec le récepteur, le calcul des taux, et les premiers échantillons décodés en paires I/Q.
settings-radio-no-hydrasdr-rfone-found-press = Aucun HydraSDR RFOne trouvé — appuyez sur Rechercher
settings-radio-these-are-rates-receiver-turned = ce sont les taux que ce récepteur s'est révélé avoir
settings-radio-receiver-only-reports-three-these = le récepteur n'en signale que trois ; les autres figurent dans la table alternative du micrologiciel, et une version plus ancienne peut ne pas les porter
settings-radio-rf-input = Entrée RF
settings-radio-antenna-sma-only-socket-bias = Le SMA antenne — la seule prise qui ait un té de polarisation derrière elle.
settings-radio-cable-socket-no-bias-tee = Une prise pour câble. Pas de té de polarisation ici ; le matériel le place sur ANT seule.
settings-radio-powers-active-antenna-preamp-down-2 = Alimente une antenne active ou un préampli par le câble coaxial. Seule la prise ANT en a une — les deux ports câble sont de simples entrées.
settings-radio-third-less-usb-traffic-leave-2 = Un tiers de trafic USB en moins. Laissez-le activé : c'est un appareil USB 2.0 et le taux maximal est de 36 Mo/s compacté contre 48 non compacté. S'applique à la reconnexion.
settings-radio-one-these-boards-1d50-60a1 = L'une de ces cartes est en 1d50:60a1, l'identifiant USB que partagent les prototypes de HydraSDR avec l'Airspy R2 et le Mini. sdroxide vérifie le micrologiciel après l'ouverture et le signalera si la mauvaise interface a été choisie, dans un sens comme dans l'autre.
settings-radio-receive-only-24-1800-mhz-2 = Réception seule, 24–1800 MHz. Ni SoapySDR ni libhydrasdr nécessaires. Le récepteur, le taux d'échantillonnage et le compactage prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez.
settings-radio-re-list-fobos-sdrs-no = Relister les SDR Fobos. Aucun périphérique n'est ouvert, donc c'est sans danger pendant la réception.
settings-radio-no-fobos-sdr-found-press = Aucun SDR Fobos trouvé — appuyez sur Rechercher
settings-radio-input-2 = Entrée
settings-radio-through-tuner-lna-vga-mixer = Par l'accord : LNA, VGA, et la plage de fréquences propre au mélangeur.
settings-radio-direct-sampling-hf1-antenna-input = Échantillonnage direct sur l'entrée antenne HF1 — ni accord, ni gain LNA/VGA, accordé entièrement par logiciel.
settings-radio-direct-sampling-hf2-antenna-input = Échantillonnage direct sur l'entrée antenne HF2, comme HF1.
settings-radio-both-real-adc-channels-once = Les deux canaux réels du CAN à la fois, combinés par le filtre de diversité ci-dessous — une source de bruit annulée, ou deux trajets à évanouissement combinés.
settings-radio-direct-sampling-tuner-so-lna = Échantillonnage direct — l'accord (et donc le gain LNA/VGA) n'est pas sur le trajet. Le taux ci-dessous est une cible pour le convertisseur logiciel, non un réglage matériel, si bien que le taux atteint peut différer de lui.
settings-radio-receiver-s-own-reported-rates = les taux annoncés par ce récepteur lui-même
settings-radio-measured-one-real-unit-connect = mesuré sur une unité réelle — connectez ce récepteur et faites Rechercher pour voir sa propre liste
settings-radio-clock-source = Source d'horloge
settings-radio-external-reference = Référence externe
settings-radio-diversity-taps = Prises de diversité
settings-radio-taps = {" "}prises
settings-radio-one-tap-gain-phase-null = Une prise est un gain et une phase — un zéro à une fréquence qui se dégrade de part et d'autre, ce que peut faire un déphaseur analogique. Chaque prise supplémentaire achète une période d'échantillonnage de la différence de trajet entre HF1 et HF2 que le filtre peut égaliser, et c'est ce qui transforme cette encoche en une bande silencieuse d'un bout à l'autre.
settings-radio-receive-only-rf-port-tuner = Réception seule. Le port RF (accord), HF1/HF2 (échantillonnage direct, décodé par logiciel) et HF1+HF2 (les deux à la fois, combinés par le filtre de diversité) fonctionnent tous. Le récepteur, l'entrée et le taux d'échantillonnage prennent effet à Appliquer ; le gain, la source d'horloge et les prises s'appliquent dès que vous les modifiez — le mode, le taux d'adaptation et le maintien du filtre de diversité se règlent dans la boîte DIV propre à la fenêtre principale, une fois cette radio sélectionnée.
settings-radio-no-hackrf-found-press-rescan = Aucun HackRF trouvé — appuyez sur Rechercher
settings-radio-front-end-amplifier-8-db = Amplificateur d'entrée, par pas de 8 dB. C'est l'étage qui modifie la sensibilité — et l'étage qui sature en premier sur une vraie antenne.
settings-radio-baseband-amplifier-after-mixer-2 = Amplificateur de bande de base après le mélangeur, par pas de 2 dB. Montez-le pour un signal faible avant de toucher au LNA.
settings-radio-rf-amp = Ampli RF
settings-radio-one-switch-shared-transmit-setting = Un seul interrupteur, partagé avec le réglage d'émission ci-dessous — la radio applique celui qui correspond au sens dans lequel elle entre. Désactivé est en général le bon réglage sur une vraie antenne.
settings-radio-hackrf-pro-chooses-itself-three = Un HackRF Pro le choisit lui-même — les trois quarts du taux d'échantillonnage, filtrés dans le FPGA — et ignore ce que demande l'hôte.
settings-radio-leave-automatic-choosing-one-too = Laissez sur Automatique. En en choisissant un trop étroit, on ne fait pas que adoucir les bords de bande — on retire le décalage d'oscillateur local qui écarte le pic continu de votre signal.
settings-radio-about-3-v-50-ma = Environ 3 V sous 50 mA par le câble coaxial, pour une antenne active ou un préampli. HackRF One ou Pro uniquement ; le Jawbreaker et le rad1o n'ont pas un tel circuit.
settings-radio-remove-dc-mirror-image = Supprimer le continu et l'image miroir
settings-radio-zero-if-radio-so-own = C'est une radio à zéro-FI, donc sa propre fuite d'oscillateur local se trouve au milieu de la largeur et l'erreur de quadrature du mélangeur y étend une image miroir. Désactiver ceci montre la sortie brute du matériel, ce qui est le moyen rapide de distinguer un problème de pilote d'un problème de DSP.
settings-radio-corrects-reference-oscillator = Corrige l'oscillateur de référence.
settings-radio-transmit = Émission
settings-radio-transmitter = Émetteur
settings-radio-off-default-while-off-backend = Désactivé par défaut. Tant que ceci est désactivé, le backend ne publie aucune voie d'émission, donc rien ne peut manipuler la radio. S'applique à la reconnexion.
settings-radio-tx-vga-gain = Gain VGA d'émission
settings-radio-transmit-driver-amplifier-drive-applied = L'amplificateur pilote d'émission. L'excitation est appliquée numériquement avant cet étage ; laissez donc l'excitation élevée et réglez le niveau de sortie ici — baisser l'excitation fait travailler le DAC à une fraction de la pleine échelle et augmente l'intermodulation.
settings-radio-same-switch-receive-setting-above = Le même interrupteur que le réglage de réception ci-dessus, appliqué lorsque la radio change de sens.
settings-radio-transmit-armed-into-dummy-load = L'émission est armée. Sur une charge fictive jusqu'à ce que vous l'ayez mesurée : les harmoniques d'un HackRF sont assez fortes pour exiger un filtre passe-bas externe pour la bande où vous êtes, et c'est un demi-duplex, donc la réception s'arrête pendant la durée de chaque trafic.
settings-radio-every-command-exchanged-radio-session = Chaque commande échangée avec la radio pendant cette session, dans l'ordre, y compris la séquence autour de chaque appui de manipulation.
settings-radio-dc-7-25-ghz-half = DC – 7,25 GHz, demi-duplex. Ni SoapySDR, ni libusb, ni libhackrf nécessaires. La radio et le taux d'échantillonnage prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez. Les bords de bande sont ceux du micrologiciel : un HackRF est spécifié pour 1 MHz – 6 GHz (100 kHz sur un Pro) et est fortement atténué hors de cette plage, mais il y accorde bien.
settings-radio-ask-sdrplay-api-service-device = Demander au service de l'API SDRplay sa liste de périphériques. Rien n'est ouvert, donc c'est sans danger pendant la réception.
settings-radio-no-rsps-press-rescan-needs = aucun RSP — appuyez sur Rechercher (exige le service de l'API SDRplay)
settings-radio-both-tuners-running-api-fixes = Les deux accords en marche, l'API fixe le CAN à 6 MHz et renvoie 2 Msps depuis une FI basse — donc 2 Msps est la plus large largeur, et les plus étroites sont ce sous-échantillonnage. Prend effet à Appliquer.
settings-radio-rates-below-2-msps-run = Les taux inférieurs à 2 Msps font tourner le CAN à 2 Msps et décimer dans le service. Prend effet à Appliquer.
settings-radio-reduced-adc-resolution = {"  "}(résolution du CAN réduite)
settings-radio-if-bandwidth = Largeur de la FI
settings-radio-tuner-s-analog-filter-auto = Le filtre analogique de l'accord. Auto choisit le plus large qui convienne au taux d'échantillonnage. Prend effet à Appliquer.
settings-radio-rsp-s-own-if-gain = La boucle de gain FI propre au RSP, gérée par le service de l'API. Désactivé rend le curseur de gain FI à vous — le réglage pour la mesure et les modes numériques en signaux faibles.
settings-radio-agc-set-point = Consigne du CAG
settings-radio-signal-level-loop-holds-adc = Le niveau de signal que la boucle maintient sur le CAN. Plus bas laisse plus de marge aux signaux hors canal. Sa valeur minimale dépend du taux d'échantillonnage — au-dessus de 8,064 Msps le convertisseur a moins de marge à offrir, et au-dessus de 9,216 Msps encore moins.
settings-radio-rsp-s-native-gain-unit = L'unité de gain native du RSP : 20 dB est le gain maximal (0 avec la plage étendue ci-dessous), 59 dB le minimum. Applique immédiatement. Ignoré pendant que le CAG tourne — la boucle possède alors cette valeur, et le S-mètre montre où elle s'est fixée.
settings-radio-extended-if-range = Plage FI étendue
settings-radio-lets-if-gain-reduction-go = Permet à la réduction de gain FI de descendre sous 20 dB, jusqu'à 0 — les 20 derniers dB de gain du récepteur. Désactivé est le réglage par défaut de l'API et le bon pour l'écoute courante, car le bas de la plage est là où un RSP est le plus facile à saturer. À activer pour les signaux faibles lorsque le LNA est déjà à 0. Permet aussi à la consigne du CAG de monter jusqu'à 0 dBFS. Applique immédiatement.
settings-radio-allow-if-gain-reduction-below = Autoriser la réduction de gain FI sous 20 dB
settings-radio-front-end-attenuation-steps-0 = Atténuation d'entrée par pas : 0 est le gain maximal, chaque pas vers le haut enclenche davantage d'atténuation. Certaines bandes ont moins de pas — le pilote borne la valeur et conserve votre choix pour quand vous revenez accorder ; la réglette ici s'arrête à ce que la bande où se trouve la radio accepte. Un choix plus élevé fait sur une autre bande est conservé et reste affiché à côté. Applique immédiatement.
settings-radio-t-0-max-gain = 0 = gain maximal
settings-radio-reference-error-parts-per-million-2 = Erreur de référence en parties par million, appliquée par le périphérique lui-même. Applique immédiatement.
settings-radio-radio-s-tuner = L'accord de cette radio
settings-radio-tuner = Accord
settings-radio-which-rspduo-s-two-tuners = Lequel des deux accords du RSPduo cette radio écoute. L'autre appartient à la radio configurée pour lui — donnez à celui-là le même récepteur et l'autre accord. Prend effet à Appliquer.
settings-radio-which-rspduo-s-two-tuners-2 = Lequel des deux accords du RSPduo porte l'antenne que vous écoutez. L'autre porte la seconde antenne. Prend effet à Appliquer.
settings-radio-which-rspduo-s-two-tuners-3 = Lequel des deux accords du RSPduo faire tourner (un à la fois). Prend effet à Appliquer.
settings-radio-applies-immediately = S'applique immédiatement.
settings-radio-dab-notch = Coupe-bande DAB
settings-radio-hardware-notch-over-165-230 = Coupe-bande matériel sur la bande DAB 165–230 MHz. S'applique immédiatement.
settings-radio-hdr-mode = Mode HDR
settings-radio-enable-below-2-mhz = Activer sous 2 MHz
settings-radio-rspdx-s-high-dynamic-range =
    Le trajet à grande dynamique du RSPdx pour LF/MF. S'applique immédiatement.

    Il ne fonctionne pas encore : sur le seul RSPdx sur lequel il a été essayé, l'activer réduit le récepteur au silence — à chaque centre, filtre, port, gain et taux essayés. Ce dont ce mode a besoin ne figure pas dans l'API documentée, et le trouver est le travail de quelqu'un d'autre. Laissez-le désactivé sauf si c'est vous qui cherchez.

    Ce qui est connu : ce n'est pas un mode qui suit le cadran. Le trajet a un filtre analogique fixe, construit seulement aux centres listés sous le filtre ci-dessous, si bien qu'accorder ailleurs ne pourrait pas fonctionner même si le reste marchait.
settings-radio-hdr-filter = Filtre HDR
settings-radio-feed-4-7-v-dc = Alimenter ~4,7 V continus par le câble coaxial
settings-radio-second-tuner = Second accord
settings-radio-rspduo-two-whole-tuners-one = Un RSPduo est deux accords complets sur une seule carte, cadencés par une même référence. Faites-les tourner tous les deux et ils entendent leurs largeurs au même instant depuis la même horloge — ce qui rend deux antennes cohérentes entre elles, avec une phase relative fixée par les lignes d'alimentation plutôt que par hasard, et permet de combiner la paire. Ou laissez-les séparés : les accords s'accordent indépendamment, si bien que l'autre peut être une seconde radio sur sa propre bande.
settings-radio-run-both-tuners = Faire tourner les deux accords
settings-radio-puts-rspduo-api-s-dual = Met le RSPduo en mode double accord de l'API, où le CAN est fixé à 6 MHz et la plus large largeur est de 2 Msps. Prend effet à Appliquer, car le mode est choisi à l'ouverture du périphérique.
settings-radio-used = Utilisé pour
settings-radio-what-other-tuner-takes-effect = À quoi sert l'autre accord. Prend effet à Appliquer — comme faire tourner les deux, cela se choisit à l'ouverture de la carte.
settings-radio-add-second-radio-settings-radio = Ajoutez une seconde radio (Réglages → Radio → +), donnez-lui ce même récepteur et l'autre accord du RSPduo, et réglez-la pour faire tourner aussi les deux accords — celle qui ouvre la carte la met en mode double accord, donc les deux doivent s'y attendre. Les deux s'accordent ensuite indépendamment : HF dans un onglet et VHF dans l'autre, depuis une seule carte. Elles partagent une même horloge de CAN, donc les deux tournent au taux demandé par la radio qui a ouvert la carte, et aucune ne peut émettre.
settings-radio-what-do = Que faire avec
settings-radio-lna-state-2 = Son état LNA
settings-radio-second-tuner-s-own-front = L'atténuation d'entrée propre au second accord. Réglée pour que les deux antennes affichent à peu près le même bruit de fond : c'est le réglage sur lequel repose tout le reste, car la combinaison pondère les deux branches par leur bruit, et un second étage d'entrée poussé en saturation remet au filtre une copie déformée du brouillage — qui ne peut pas être soustraite d'une copie non déformée. Applique immédiatement.
settings-radio-if-gain-reduction-2 = Sa réduction de gain FI
settings-radio-second-tuner-s-if-gain = Le gain FI du second accord, dans l'unité native du RSP : 20 dB est le gain maximal. Ignoré pendant que le CAG tourne — et un gain stable est ce que veut le filtre, donc couper le CAG vaut la peine pour un zéro que vous voulez maintenir.
settings-radio-filter-length = Longueur du filtre
settings-radio-one-tap-gain-phase-null-2 = Une prise est un gain et une phase — un zéro à une fréquence qui se dégrade de part et d'autre, ce que peut faire un déphaseur analogique. Chaque prise supplémentaire achète une période d'échantillonnage de la différence de trajet entre les deux antennes que le filtre peut égaliser, et c'est ce qui transforme cette encoche en une bande silencieuse d'un bout à l'autre.
settings-radio-adaptation-2 = Adaptation
settings-radio-slow-steady-left-converging-inside = Lent et régulier à gauche, convergeant en moins d'une fraction de seconde, et visiblement en recherche à droite. Commencez vite pour trouver le zéro, puis maintenez-le.
settings-radio-stop-filter-moving-reach-moment = Arrêter le mouvement du filtre. Utilisez-le dès qu'un zéro apparaît : un filtre laissé en adaptation se réorientera vers ce qui devient le plus fort, et sur une bande calme, c'est la station que vous écoutez.
settings-radio-restart = Redémarrer
settings-radio-zero-filter-find-null-again = Remettre le filtre à zéro et retrouver le zéro.
settings-radio-nothing-here-can-tell-wanted = Rien ici ne peut distinguer un signal utile d'un signal indésirable — le filtre ne sait que ce que les deux antennes ont en commun. En Annuler, la seconde antenne doit entendre la source de bruit et le moins possible de la bande, sinon elle annulera docilement la station aussi. En Combiner, les deux doivent entendre la même station. La profondeur du zéro est écrite dans le journal toutes les quelques secondes.
settings-radio-not-yet-verified-against-rspduo = Pas encore vérifié sur un RSPduo : le fonctionnement à double accord est ici écrit à partir de l'API de SDRplay plutôt que mesuré sur le matériel.
settings-radio-no-receiver-listed-check-against = Aucun récepteur n'est listé pour vérifier cela — appuyez sur Rechercher, ou lisez le journal pour voir ce qui s'est réellement ouvert.
settings-radio-receive-only-1-khz-2 = Réception seule, 1 kHz–2 GHz. Exige le service d'API SDRplay du constructeur (sdrplay.com/api) — les RSP postérieurs au RSP1 d'origine n'ont pas de protocole ouvert. Le périphérique, le taux d'échantillonnage, la largeur de bande et l'accord du RSPduo prennent effet à Appliquer ; tout le reste s'applique dès que vous le modifiez.
settings-radio-board = Carte
settings-radio-first-one-found-2 = Le premier trouvé
settings-radio-receive-chain = Chaîne de réception
settings-radio-both-chains-tune-together-they = Les deux chaînes s'accordent ensemble — elles partagent un même synthétiseur — mais ce sont deux étages d'entrée distincts sur des prises distinctes. Choisissez celle sur laquelle est votre antenne, c'est le réglage à utiliser lorsqu'une chaîne a reçu la modification d'adaptation HF et que l'autre est d'origine.
settings-radio-receive-gain = Gain de réception
settings-radio-one-combined-figure-which-limesuite = Une valeur combinée, que LimeSuite répartit elle-même entre le LNA, le TIA et le PGA. Elle accepte des décibels entiers ; tout ce qui est plus fin est tronqué.
settings-radio-receive-port = Port de réception
settings-radio-automatic-follows-frequency-lnal-low = Automatique suit la fréquence : LNAL en bas, LNAH en haut — sauf si un LimeRFE est connecté en dessous, ce qui fait un câble dans une prise, et alors c'est LNAW à toutes les fréquences. Nommez la prise à laquelle le vôtre est câblé si ce n'est pas celle-là. La chaîne à laquelle appartient la prise se choisit dans le sélecteur ci-dessus.
settings-radio-t-0-follows-sample-rate-worth = 0 suit le taux d'échantillonnage. Il vaut la peine de le laisser là : un filtre plus étroit qu'un quart de la largeur retire silencieusement le décalage d'oscillateur local zéro-FI, ce qui remet la fuite de l'OL sur le signal que vous écoutez.
settings-radio-following-rate = suit le taux
settings-radio-corrections = Corrections
settings-radio-host-iq-dc-correction = Correction I/Q / continu de l'hôte
settings-radio-adaptive-image-dc-removal-side = Suppression adaptative de l'image et du continu de ce côté, en plus de l'étalonnage propre à la puce. La désactiver est le moyen en un clic de distinguer un problème de pilote d'un problème de DSP.
settings-radio-calibrate-automatically = Étalonner automatiquement
settings-radio-runs-chip-s-own-dc = Fait tourner l'étalonnage propre à la puce pour le décalage continu et l'image lorsque la radio s'ouvre, et de nouveau une fois que le cadran s'est stabilisé sur une nouvelle bande ou une autre prise. Ces valeurs sont mesurées à une fréquence et sont fausses ailleurs, ce qu'est en général une porteuse au milieu de la largeur. Coûte environ une seconde à chaque fois.
settings-radio-calibrate-now = Étalonner maintenant
settings-radio-stalls-receiver-better-part-second = Bloque le récepteur pendant la plus grande partie d'une seconde.
settings-radio-second-aerial = Seconde antenne
settings-radio-socket = Sa prise
settings-radio-same-first = Identique à la première
settings-radio-gain-2 = Son gain
settings-radio-set-low-coupled-sample-your = À régler BAS. L'échantillon couplé de votre propre émetteur est un signal fort, et une chaîne de rétroaction poussée en compression mesure mal la courbe de l'amplificateur — elle enseigne à la correction sa propre distorsion. Partez du bas et utilisez l'atténuateur du coupleur.
settings-radio-set-so-both-aerials-show = À régler pour que les deux antennes affichent à peu près le même bruit de fond. C'est le réglage sur lequel repose tout le reste : la combinaison pondère les deux branches par leur bruit, et une seconde chaîne poussée en compression remet au filtre une copie déformée du brouillage — qui ne peut pas être soustraite d'une copie non déformée.
settings-radio-table-steps-2 = Pas du tableau
settings-radio-how-finely-correction-follows-amplifier = La finesse avec laquelle la correction suit la courbe de l'amplificateur. Davantage de pas suivent un coude plus net, mais chacun doit être appris à partir des échantillons qui y sont tombés — et le haut d'un histogramme d'amplitude de parole est mince. Trente-deux suffisent pour la courbe lisse qu'a réellement un amplificateur HF. Modifier ce réglage recommence la correction.
settings-radio-how-hard-each-block-feedback = La force avec laquelle chaque bloc de rétroaction déplace la correction. La courbe d'un amplificateur ne change pas, donc il n'y a pas lieu de se presser — le milieu en moyenne plusieurs trafics de bruit.
settings-radio-keep-correction-curve-learned-clean = Conserver la correction telle quelle. Une courbe apprise sur un trafic propre vaut la peine d'être gardée, et l'amplificateur n'aura pas changé au suivant.
settings-radio-forget-correction-learn-again = Oublier la correction et la réapprendre.
settings-radio-directional-coupler-amplifier-s-output = Un coupleur directif sur la sortie de l'amplificateur entre dans cette chaîne, et l'émetteur compare ce qui est revenu avec ce qu'il voulait envoyer — puis envoie l'inverse de la différence, si bien que ce qui sort de l'amplificateur est droit. Une vingtaine de décibels d'intermodulation en moins sur les QSO des autres, l'amplificateur gardant sa puissance. La correction reste à l'unité tant que la rétroaction ne s'aligne pas sur l'émission, donc un coupleur non branché ne coûte rien ; et elle ne peut jamais demander plus que la pleine échelle au convertisseur, donc un trajet de rétroaction qui lit n'importe quoi ne peut rien sur-piloter. Son avancement est écrit dans le journal pendant que vous émettez.
settings-radio-transmit-not-armed-so-there = L'émission n'est pas armée, donc il n'y a rien ici à corriger.
settings-radio-off-interface-publishes-no-transmit = Désactivé, l'interface ne publie aucune voie d'émission, donc rien ne peut manipuler la radio.
settings-radio-transmit-gain = Gain d'émission
settings-radio-transmit-port = Port d'émission
settings-radio-board-s-two-transmit-sockets = Les deux prises d'émission de la carte sont deux réseaux d'adaptation différents, et non deux prises sur le même : BAND1 (TX_1) porte de 30 MHz à 1,9 GHz, et BAND2 (TX_2) est le port 13 cm. Manipuler une bande amateur inférieure au 23 cm depuis BAND2 envoie le trafic dans un réseau d'adaptation qui n'en laisse presque rien passer — chaque réglage paraît juste, l'excitation est celle que vous avez fixée, et le wattmètre reste à zéro. Laissez sur Automatique et il suit le cadran.
settings-radio-limesdr-transmits-about-100-khz = Un LimeSDR émet d'environ 100 kHz à 3,8 GHz sans filtrage propre. Utilisez un filtre passe-bas, un canal LimeRFE approprié, ou une charge fictive.
settings-radio-limerfe-front-end = Étage d'entrée LimeRFE
settings-radio-connected = Connecté par
settings-radio-limerfe-s-own-micro-usb = Le port micro-USB propre au LimeRFE, non celui de la radio. 9600 bauds, fixés par son micrologiciel.
settings-radio-receive-connector = Connecteur de réception
settings-radio-transmit-connector = Connecteur d'émission
settings-radio-follow-dial = Suivre le cadran
settings-radio-switch-filters-match-operating-frequency = Basculer les filtres sur la fréquence de travail, avant qu'aucune HF n'apparaisse. Accorder dans une même bande ne met rien sur la liaison de commande.
settings-radio-relays = Relais
settings-radio-receive-attenuator = Atténuateur de réception
settings-radio-notch-filter = Filtre coupe-bande
settings-radio-fan = Ventilateur
settings-radio-worth-having-any-sustained-transmitting = À activer pour toute émission soutenue.
settings-radio-automatic-board-receives-switched-transmit = En Automatique, la carte reçoit, et elle est basculée en émission à l'appui sur la manipulation et de nouveau en réception au relâchement — quel que soit le câblage. Ses voies amateurs ont un filtre avec un commutateur émission/réception de part et d'autre, donc une carte à qui l'on demande les deux à la fois place ce commutateur sur l'émetteur et cesse d'entendre quoi que ce soit. La voie 30 MHz n'est accessible que par J5, qui est un seul connecteur pour les deux sens ; au-dessus, l'émission depuis J4 garde le trajet de réception hors du connecteur que pilote l'amplificateur.
settings-radio-limesdr-support-has-not-been = La prise en charge du LimeSDR n'a pas été vérifiée sur du matériel. Ceci est la trace de la dernière session, pour un rapport de problème.
settings-radio-gains-filters-corrections-antenna-sockets = Les gains, les filtres, les corrections, les prises d'antenne et chaque commande du LimeRFE — ses connecteurs, sa bande, ses relais, son atténuateur, son coupe-bande et son ventilateur — s'appliquent immédiatement. La carte, la chaîne de réception, le taux d'échantillonnage, l'armement de l'émission et la connexion du LimeRFE prennent effet à Appliquer.

## Settings window, Radio tab: messages with values

settings-radio-fill-from = REMPLIR DEPUIS { $name }
settings-radio-decimation-auto = Automatique (le plus proche de { $khz } kHz)
settings-radio-decimation-stage-server = Étage { $stage } — le taux du serveur ÷ { $divisor }
settings-radio-decimation-stage = Étage { $stage } — taux ÷ { $divisor }
settings-radio-kiwi-receive-only = Réception seule — c'est l'antenne de quelqu'un d'autre. Le panoramique est les ~12 kHz que le récepteur envoie en I/Q, qui suit le cadran ; la vue de bande au-dessus est sa propre cascade, et accorder à travers elle réaccorde le récepteur. Environ { $rate } ko/s pendant la connexion. Appuyez sur « Appliquer / reconnecter » pour basculer sans redémarrer.
settings-radio-connected-detail = Connecté : { $detail }
settings-radio-failed = Échec : { $error }
settings-radio-pluto-buffer = ≈ { $ms } ms, { $kib } Kio par transfert
settings-radio-driver-settings = Réglages de { $driver }
settings-radio-rx888-coverage = Couverture 0–{ $mhz } MHz, { $mbytes } Mo/s par USB
settings-radio-rx888-digitised = sur les { $mhz } MHz numérisés
settings-radio-elad-16-bit = { $khz } kHz (échantillons 16 bits)
settings-radio-elad-pad = Atténuateur de { $db } dB enclenché
settings-radio-hackrf-rx-preamp = Préampli de { $db } dB en réception
settings-radio-hackrf-tx-preamp = Préampli de { $db } dB en émission
settings-radio-device-not-found = { $device } (introuvable)
settings-radio-lime-chain = Chaîne { $n } (RX{ $n }_*)
settings-radio-lime-chain-tx = Chaîne { $n } (RX{ $n }_* / TX{ $n }_*)
settings-radio-lime-second-chain = L'autre chaîne de réception de la carte, sur les prises { $sockets }. Elle partage le synthétiseur, donc elle entend la même largeur au même instant que la première — ce qui lui permet de porter une seconde antenne, ou un échantillon de votre propre émetteur.
settings-radio-tx-blocked = L'émission est bloquée : { $reason }
settings-radio-rfe-rx-fallback = Réception sur { $port } : celles-ci repassent sur le trajet large bande non filtré : { $bands }.
settings-radio-rfe-tx-fallback = Émission sur { $port } : celles-ci repassent sur le trajet large bande — ni amplificateur de bande ni filtrage : { $bands }.

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = Volume, gain, CAG, squelch, les commandes de bruit, et l'affichage RDS en radiodiffusion FM
topbar-receiver = RÉCEPTEUR
topbar-vfo-b-split-rit-xit = VFO A/B, split, et les décalages RIT/XIT
topbar-tuning = ACCORD
topbar-diversity-filter-which-way-combines = Le filtre de diversité : dans quel sens il combine les deux antennes, la vitesse à laquelle il s'adapte, et le maintien en place
topbar-diversity = DIVERSITÉ
topbar-second-receiver-s-frequency-mode = La fréquence, le mode, le filtre et le niveau du second récepteur
topbar-sub-receiver = SOUS-RÉCEPTEUR
topbar-transceiver-s-own-aerial-socket = La prise antenne propre à l'émetteur-récepteur, et son interrupteur d'alimentation
topbar-radio = RADIO
topbar-tune-voice-keyer-drive-mic = Accord, le manipulateur vocal, et les niveaux d'excitation et de micro
topbar-transmit = ÉMISSION
topbar-tune = {" "}ACCORD{" "}
topbar-display = AFFICHAGE
topbar-logbook-spots-awards-memories-scanner = Journal, spots, distinctions, mémoires, le scanner, les réglages et le manuel
topbar-system = SYSTÈME
topbar-linked = Lié
topbar-not-linked = Non lié
topbar-bar-trace = barre / tracé
topbar-needle-bar-trace = aiguille / barre / tracé
topbar-swap-vfos = Échanger les VFO
topbar-copy-b = Copier A vers B
topbar-second-receiver-right-ear-tunes = Second récepteur, dans l'oreille droite. Il s'accorde indépendamment de A/B — ses commandes apparaissent dans le module SUB, et sa bande passante sur la cascade.
topbar-band-plan = , d'après le plan de bande
topbar-repeater-shift-following-band-plan = Décalage relais : suit le plan de bande, qui n'a pas de décalage pour cette fréquence — simplex
topbar-repeater-shift-transmit-above-below = Décalage relais : émettre au-dessus ou en dessous du cadran, pour travailler un relais. Simplex pour l'instant
topbar-repeater-shift = DÉCALAGE RELAIS
topbar-auto = AUTO
topbar-take-shift-band-plan-dial = Prendre le décalage dans le plan de bande lorsque le cadran bouge. Il n'intervient que dans une sous-bande de sortie de relais — partout ailleurs il laisse la radio en simplex, si bien que les canaux d'appel restent en simplex.
topbar-offset = Décalage
topbar-repeater-tone-ctcss-dcs-goes = Tonalité relais : le CTCSS/DCS qui part sous la voix, la salve 1750 Hz, et le squelch à tonalité en réception
topbar-transmit-tone = TONALITÉ D'ÉMISSION
topbar-set-here-sent-nfm-sub = réglé ici, envoyé en NFM — la signalisation infra-audio est propre à un canal FM
topbar-normal = NORMAL
topbar-invert = INVERSER
topbar-bit-order-dcs-encoded-here = l'ordre des bits avec lequel le DCS est codé ici est transcrit de la norme et n'a jamais été vérifié sur un relais — s'il ne s'ouvre pas, essayez le CTCSS
topbar-t-1750-hz-burst = SALVE 1750 HZ
topbar-every-over = À CHAQUE TRAFIC
topbar-open-burst-every-time-transmitter = Ouvrir avec la salve à chaque fois que l'émetteur est manipulé
topbar-send = ENVOYER
topbar-send-burst-now-keying-transmitter = Envoyer la salve maintenant — en manipulant l'émetteur pendant sa durée s'il ne l'est pas déjà
topbar-nfm-only-burst-fm-repeater = NFM uniquement : la salve est l'ouvre-porte d'un relais FM
topbar-receive-tone-squelch = SQUELCH À TONALITÉ EN RÉCEPTION
topbar-require-receive-what-station-transmits = Exiger en réception ce que cette station émet
topbar-hide-docked-band-selector-click = Masquer le sélecteur de bande accroché — cliquez à nouveau pour le faire revenir
topbar-show-docked-band-selector = Afficher le sélecteur de bande accroché
topbar-band-mode = BANDE ET MODE
topbar-hide-band-chip-brings-back = Masquer — la puce de bande le fait revenir
topbar-dec-off = DEC off
topbar-vol = Vol
topbar-counted-steps-not-decibels-what =


    Compté en pas, non en décibels : ce que vaut un pas relève du récepteur lui-même, et sur un SDRplay cela dépend aussi de la bande. Le bon réglage reste plus de gain.
topbar-gain = Gain
topbar-plutosdr-s-own-agc-setting =


    Le CAG propre au PlutoSDR règle ce gain, donc déplacer le curseur ne fait rien. Réglez Réglages → Radio → CAG sur Manuel pour le commander ici.
topbar-agc-hang-time-click-cycle = Temps de maintien du CAG — cliquez pour parcourir : Arrêt / Lent / Moyen / Rapide
topbar-man = Man
topbar-manual-audio-gain-used-while = Gain audio manuel, utilisé pendant que le CAG est désactivé. Initialisé au niveau que le CAG maintenait au moment de sa désactivation.
topbar-radio-s-own-squelch-sent = Le squelch propre à la radio, envoyé par la liaison de commande. C'est la porte que traverse réellement l'audio — celle du logiciel ne ferait que se fermer davantage sur ce que l'appareil a déjà laissé passer.
topbar-max-gain = gain max
topbar-manual-gain = gain manuel
topbar-noise-reduction = réduction de bruit
topbar-impulse-noise-blanker = Limiteur de bruit impulsionnel
topbar-auto-notch-cancel-constant-tone = Coupe-bande automatique : annule les éléments à tonalité constante (hétérodynes)
topbar-binaural-audio-not-while-sub = Audio binaural — pas pendant que le sous-récepteur tient l'oreille droite
topbar-binaural-audio-passband-spread-across = Audio binaural : la bande passante est répartie entre les deux oreilles, si bien que des signaux de hauteurs différentes viennent de directions différentes et que celui que vous accordez glisse d'un côté à l'autre. Cliquez pour revenir au mono
topbar-binaural-cw-spread-passband-across = CW binaural : répartit la bande passante entre les deux oreilles, si bien que des signaux de hauteurs différentes viennent de directions différentes — un pile-up devient plusieurs endroits au lieu d'une seule note encombrée, et accorder une station la fait glisser d'un côté à l'autre. Idéal au casque
topbar-binaural-audio-spread-passband-across = Audio binaural : répartit la bande passante entre les deux oreilles. En voix, le bruit se répartit sur toute l'image tandis que la station reste au milieu, ce qui est plus agréable à écouter pendant une heure — au prix de la voix elle-même, étalée. Idéal au casque
topbar-mute = COUPER LE SON
topbar-record-audio-raw-i-q = Enregistrer l'audio, l'I/Q brut, ou les deux
topbar-wfm-stereo-forced-off-click = Stéréo WFM forcée désactivée — cliquez pour la stéréo automatique
topbar-wfm-stereo-pilot-locked-click = Stéréo WFM : pilote verrouillé. Cliquez pour forcer le mono
topbar-wfm-stereo-automatic-no-pilot = Stéréo WFM : automatique, pas de pilote sur cette station
topbar-rds-data-arriving-click-see = Les données RDS arrivent — cliquez pour les voir
topbar-rds-no-data-station-click = RDS — aucune donnée sur cette station. Cliquez quand même pour les diagnostics du décodeur
topbar-drm-decoding-click-details = DRM en cours de décodage — cliquez pour les détails
topbar-drm-click-decoder-s-state = DRM — cliquez pour l'état du décodeur
topbar-hd-radio-decoding-click-details = HD Radio en cours de décodage — cliquez pour les détails
topbar-hd-radio-click-decoder-s = HD Radio — cliquez pour l'état du décodeur
topbar-presets = PRÉRÉGLAGES
topbar-width-per-sideband = LARGEUR PAR BANDE LATÉRALE
topbar-width = LARGEUR
topbar-passband-width-hertz-type-drag = Largeur de la bande passante en hertz — saisissez-la, ou faites glisser. Cliquez dans le champ pour saisir une valeur exacte.
topbar-edges = BORDS
topbar-low-edge-hz-carrier = Bord bas, en Hz depuis la porteuse
topbar-high-edge-hz-carrier = Bord haut, en Hz depuis la porteuse
topbar-noise-reduction-voice-off-click = Réduction de bruit (voix), désactivée — cliquez pour choisir un moteur
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = Ce que vous entendez, en MP3 : le récepteur sur un canal et votre propre émission sur l'autre. C'est l'enregistrement d'un QSO.
topbar-mono = MONO
topbar-mixed-one-channel-click-two = Mélangé sur un canal — cliquez pour deux
topbar-two-channels-rx-left-tx = Deux canaux : RX à gauche, TX à droite — cliquez pour un canal mélangé
topbar-quick-clip = EXTRAIT RAPIDE
topbar-stop-after = ARRÊT APRÈS
topbar-no-stop = pas d'arrêt
topbar-auto-record = ENREGISTREMENT AUTO
topbar-set-squelch-auto-record-needs = Réglez un squelch — l'enregistrement automatique en a besoin pour reconnaître le silence
topbar-auto-record-needs-sdr-front = L'enregistrement automatique exige un étage d'entrée SDR avec un squelch logiciel
topbar-stop-following-squelch = Cesser de suivre le squelch
topbar-waiting-signal = en attente d'un signal
topbar-record-spectrum = ENREGISTRER LE SPECTRE
topbar-i-q-wav = ● WAV I/Q
topbar-i-q-wav-2 = WAV I/Q
topbar-raw-spectrum-receiver-delivering-stereo = Le spectre brut que fournit le récepteur, en WAV stéréo à virgule flottante 32 bits (RF64 au-delà de 4 Go) que SDR#, SDRuno, HDSDR et SDRangel ouvrent — et que sdroxide lui-même rejoue avec --file. Volumineux : 8 octets par échantillon.
topbar-radio-hands-over-demodulated-audio = Cette radio fournit de l'audio démodulé, donc il n'y a aucun I/Q à enregistrer.
topbar-what-capture-costs-sample-rate = Ce que coûte une capture à ce taux d'échantillonnage
topbar-engine = MOTEUR
topbar-no-noise-reduction-decoders-never = Aucune réduction de bruit — les décodeurs ne l'ont de toute façon jamais vue
topbar-strength = FORCE
topbar-tone = TONALITÉ
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS — aucune tonalité infra-audio sur ce signal
topbar-tone-squelch = SQUELCH À TONALITÉ
topbar-carrier-squelch-open-any-signal = Squelch de porteuse : s'ouvre sur tout signal
topbar-require-tone-currently-being-received = Exiger la tonalité actuellement reçue
topbar-ctcss = CTCSS
topbar-any-dcs = TOUT DCS
topbar-open-any-dcs-coded-signal = S'ouvre sur tout signal codé en DCS. Le code parmi les 104 qu'il porte ne peut pas être lu de façon fiable ici, donc il n'y a rien de plus fin à choisir
topbar-which-socket-back-radio-receiving =
    Quelle prise à l'arrière la radio reçoit — sa propre commande ANT, le même réglage que le bouton ANT de la face avant. Cliquez pour passer à la suivante.

    Le choix est mémorisé par bande, et remis en place la prochaine fois que le cadran entre dans cette bande.
topbar-radio-s-separate-receiving-antenna =
    L'antenne de réception séparée de la radio, commutée dans le trajet de réception ou hors de celui-ci — son propre réglage RX ANT. L'antenne sur la prise principale reste en émission dans tous les cas.

    La radio mémorise cela elle-même par bande, donc sdroxide le relit après chaque changement de bande plutôt que de remettre ce qu'il a vu en dernier : cliquer ici est la seule chose qui le modifie.
topbar-radio-s-own-power-switch =
    L'interrupteur d'alimentation propre à la radio, par la liaison de commande, et le seul vrai marche/arrêt du programme — non l'interrupteur LIAISON de sdroxide, qui ferme le côté de sdroxide et laisse la radio en marche.

    Pour que MARCHE atteigne quoi que ce soit, l'extrémité de commande de la radio doit rester éveillée pendant qu'elle est éteinte : le contrôle réseau sur le réseau local, ou un port CI-V encore alimenté par le secteur sur un appareil éteint par la face avant.
topbar-switch-radio = Allumer la radio
topbar-switch-radio-off-audio-meters = Éteindre la radio. L'audio et les indicateurs s'arrêtent avec elle.
topbar-what-second-aerial-click-swap =
    À quoi sert la seconde antenne — cliquez pour permuter.

    ANNULER la soustrait de la première, avec le gain, la phase et le retard qui alignent une source de bruit locale sur les deux : la forme DSP d'un déphaseur anti-bruit. COMBINER additionne les deux dans la phase qui renforce, pondérées par la qualité d'écoute de chacune — réception en diversité, qui comble les évanouissements.
topbar-hold = MAINTENIR
topbar-stop-filter-moving-reach-moment = Arrêter le mouvement du filtre. Utilisez-le dès qu'un zéro apparaît : un filtre laissé en adaptation se réorientera vers ce qui devient le plus fort, et sur une bande calme, c'est la station que vous écoutez.
topbar-restart = REDÉMARRER
topbar-zero-filter-find-null-again = Remettre le filtre à zéro et retrouver le zéro.
topbar-adapt = Adapter
topbar-how-fast-filter-chases-slow = La vitesse à laquelle le filtre poursuit : lent et régulier à gauche, convergeant en moins d'une fraction de seconde, et visiblement en recherche à droite. Commencez vite pour trouver le zéro, puis MAINTENEZ-le.
topbar-where-sub-receiver-listens-shift = Où écoute le sous-récepteur. Maj-clic sur la cascade, ou glissez dans la bande passante du sous-récepteur, pour le déplacer.
topbar-dial = ←CADRAN
topbar-move-sub-receiver-main-dial = Déplacer le sous-récepteur sur le cadran principal
topbar-dial-2 = CADRAN←
topbar-move-main-dial-sub-receiver = Déplacer le cadran principal sur le sous-récepteur
topbar-filter = Filtre
topbar-sub-receiver-passband-edges-hz = Bords de la bande passante du sous-récepteur, en Hz
topbar-sub-receiver-level-plays-right = Niveau du sous-récepteur (il joue dans l'oreille droite)
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = Manipulateur vocal : enregistrer et émettre les messages mémorisés
topbar-drive = Excitation
topbar-tune-2 = Accord
topbar-mic = Micro
topbar-cessb = CESSB
topbar-microphone-gain = Gain du microphone
topbar-deviation-how-far-mode-s =
    Excursion : jusqu'où la salve de ce mode fait dévier une radio qui le module elle-même. Un émetteur FM transforme le niveau audio en excursion de fréquence et n'a pas de CAL pour le retenir ; donc une pleine échelle envoyée à une entrée de données réglée pour la voix dépasse l'excursion — ce qui paraît parfaitement normal à l'écoute et ne décode pour personne.

    Mémorisée par mode, si bien qu'une excursion réglée pour le paquet 1200 bauds n'atterrit jamais sur le FT8.
topbar-transmit-audio-how-hard-mode =
    Audio d'émission : la force avec laquelle ce mode pilote le modulateur d'une radio qui module ce qu'on lui envoie — un appareil CAT sur sa carte son, un FLEX, un Icom sur son port réseau. Baissez-la jusqu'à ce que le CAL de l'appareil bouge à peine, et réglez la puissance sur la radio ; un CAL qui monte sur un mode numérique à enveloppe constante, c'est ce qui fait les éclaboussures.

    L'excitation n'est pas cette commande : sur ces radios, l'excitation atteint le registre de puissance de l'appareil et ne touche jamais à son audio.

    Mémorisée par mode — FT8, RTTY, PSK et MCW gardent chacun la leur.
topbar-tx-audio = Audio d'émission
topbar-cw-psk-rtty-skimmers-decode = Skimmers CW / PSK / RTTY — décodent les signaux sur toute la bande et les marquent sur la cascade
topbar-skimmers = SKIMMERS
topbar-run-skimmer = Lancer ce skimmer
topbar-minimum-snr-decoded-signal-needs = Rapport signal/bruit minimal qu'un signal décodé doit avoir pour être repéré
topbar-needs-wideband-iq-source = nécessite une source I/Q large bande
topbar-cw-decoder = DÉCODEUR CW
topbar-how-many-signals-model-reads = Le nombre de signaux que lit le modèle à la fois. Les autres gardent leur repère mais ne portent aucun texte.
topbar-spots = SPOTS
topbar-save-skimmer-s-spot-list = Enregistrer la liste des spots du skimmer dans un fichier
topbar-show-hide-full-band-waterfall = Afficher/masquer la bande de cascade pleine largeur au-dessus du panoramique — tout ce que ce récepteur voit à la fois
topbar-spectrum-waterfall-either-layer-both = Spectre et cascade — l'une ou l'autre couche, les deux, ou aucune. Allumé lorsque les deux sont affichées.
topbar-spectrum = SPECTRE
topbar-show-spectrum = AFFICHER LE SPECTRE
topbar-draw-spectrum-line-across-top = Dessiner la ligne du spectre en haut du panoramique. Désactivé, la cascade prend toute la hauteur.
topbar-peak-hold = MÉMOIRE DE CRÊTE
topbar-trace-highest-level-each-column = Trace le niveau le plus haut atteint par chaque colonne au-dessus de la ligne en direct, qui redescend ensuite — ce que la bande a fait pendant que vous regardiez ailleurs
topbar-draw-spectrum-receding-surface-instead = Dessiner le spectre comme une surface qui s'éloigne au lieu d'une ligne plate : le spectre le plus récent à l'avant, les précédents s'éloignant de vous. Les deux dernières secondes de la bande en paysage — une porteuse qui apparaît et disparaît est une crête plutôt qu'une ligne qui tremble. La grille et la mémoire de crête appartiennent à la ligne plate et ne sont pas dessinées dessus.
topbar-how-quickly-spectrum-line-follows = La vitesse à laquelle la ligne du spectre suit la bande. Plus lent moyenne davantage de trames entre elles : une ligne plus stable, et une porteuse faible qui reste assez longtemps immobile pour être lue. La cascade n'en est pas affectée — ces lignes reçoivent chaque trame dans les deux cas.
topbar-show-waterfall = AFFICHER LA CASCADE
topbar-draw-scrolling-waterfall-below-spectrum = Dessiner la cascade défilante sous le spectre. Désactivée, la ligne du spectre prend toute la hauteur.
topbar-how-fast-waterfall-scrolls-lines = La vitesse de défilement de la cascade, en lignes par seconde : Lent 5, Moyen 28, Rapide 56, Plus rapide 112, Le plus rapide 224. Le moteur cadence lui-même ces valeurs, donc les deux plus rapides apportent un vrai détail plutôt que la même ligne dessinée deux fois — dans la mesure où le récepteur peut les alimenter : une ligne ne peut jamais montrer plus d'une transformée, et un étage d'entrée étroit n'en produit que quelques dizaines par seconde. Elles coûtent de l'historique, puisque la cascade garde un nombre fixe de lignes — 73 secondes à Moyen, 9 au Le plus rapide.
topbar-decode-labels = ÉTIQUETTES DE DÉCODAGE
topbar-mark-every-decoded-station-waterfall = Marquer chaque station décodée sur la cascade avec son indicatif, à la fréquence où elle a été entendue — FT8, FT4 et les autres modes à créneaux. Un bon départ en place une trentaine sur la largeur deux fois par minute, par-dessus les traces que vous lisez ; désactivez-les et la cascade n'est plus que la bande. La liste des décodages à côté contient toujours chaque indicatif.
topbar-switch-3d-set = Activez la 3D pour régler ceci
topbar-how-3d-spectrum-drawn-only = La façon dont le spectre 3D est dessiné. Seule la forme diffère — les deux rendus montrent les mêmes spectres, et tous deux masquent ce qui se trouve derrière, si bien qu'un signal fort au premier plan se dresse au-dessus de la bande qu'il couvre.
topbar-lines = LIGNES
topbar-one-trace-per-remembered-spectrum = Une trace par spectre mémorisé, dans la couleur propre à la ligne plate, chacune masquant celles qui sont derrière. La forme sans les niveaux, et la lecture à choisir lorsque la forme est ce que vous cherchez — elle coûte un peu plus que la pleine, au lieu de moins, car elle dessine la même surface puis trace chaque crête par-dessus.
topbar-solid = PLEIN
topbar-filled-surface-coloured-waterfall-s = Une surface pleine colorée par la palette de la cascade, si bien que le niveau est dans la couleur comme dans la hauteur, et que les deux moitiés du panoramique s'accordent sur l'aspect d'un signal fort. Modifiez la palette dans Réglages › Affichage.
topbar-how-fast-3d-spectrum-flows = La vitesse à laquelle le spectre 3D s'éloigne de vous, en lignes par seconde : Lent 6, Moyen 12, Rapide 24, Plus rapide 48. La surface a une profondeur fixe en lignes, donc c'est aussi la durée qu'elle retient. Plus lent, c'est une mémoire plus longue et une surface qui rampe ; plus rapide, une mémoire plus courte qui bouge. La cascade garde sa propre vitesse de défilement, ci-dessous.
topbar-how-many-columns-panadapter-waterfall = Le nombre de colonnes avec lesquelles le panoramique et sa cascade sont dessinés. Auto lit le moteur de rendu de cette machine et la taille du panoramique, et choisit le maximum qu'il peut porter — un écran 4K en veut 4096. Chaque colonne est un octet dans chaque trame, donc un client connecté à une station distante paie le détail sur sa liaison : 4096 colonnes à 60 i/s représentent environ un quart de mégaoctet par seconde, soit le double de la largeur standard.
topbar-read-machine-screen-take-most = Lire cette machine et cet écran, prendre le maximum qu'ils peuvent porter, et les suivre s'ils changent
topbar-width-force-right-now-whatever = La largeur appliquée en ce moment, quelle que soit la demande de la rangée ci-dessus
topbar-keep-floor-ceiling-set-best = Garder le plancher/plafond réglé pour le meilleur contraste de cascade — rétablis doucement lors d'un changement de bande, après un déplacement ou un zoom, et lorsque les niveaux dérivent. Activez pour ajuster immédiatement ; désactivez pour garder les niveaux là où vous les avez réglés.
topbar-keep-tuned-frequency-middle-panadapter = Garder la fréquence accordée au milieu du panoramique : la fenêtre glisse sous le cadran au lieu que l'image saute d'une largeur entière lorsque vous accordez au-delà du bord. Activez pour centrer immédiatement ; désactivez pour déplacer et zoomer où vous voulez.
topbar-ceil = plaf{" "}
topbar-fft-size = TAILLE DE LA FFT
topbar-scroll-waterfall-upwards-newest-row = Faire défiler la cascade vers le haut — ligne la plus récente en bas (V)
topbar-logbook-all-qsos-digital-manual = Journal — tous les QSO (numériques + manuels)
topbar-award-tracking-dxcc-was-waz = Suivi des distinctions — DXCC / WAS / WAZ / carrés
topbar-band-conditions-published-forecast-beside = Conditions de bande — la prévision publiée à côté de ce qui a réellement été entendu sur chaque bande
topbar-satellite-qo-100-beacon-calibration = Satellite — l'étalonnage par la balise QO-100 est en cours
topbar-satellite-doppler-tracking-qo-100 = Satellite — suivi Doppler, et étalonnage par la balise QO-100
topbar-ism-band-devices-decoding-now = Appareils de la bande ISM — décodés en ce moment, que cette fenêtre soit ouverte ou non. Désactivez-le avec DÉCODAGE dans la fenêtre.
topbar-ism-band-devices-weather-sensors = Appareils de la bande ISM — capteurs météo, compteurs et domotique entendus autour de vous
topbar-public-sdrs-internet-browse-kiwisdr = SDR publics sur Internet — parcourez les annuaires KiwiSDR et SpyServer, et ouvrez-en un comme radio
topbar-winlink-radio-email = Courriel radio Winlink
topbar-memory-channels = Canaux mémoire
topbar-scanner-stopped-signal = Scanner — arrêté sur un signal
topbar-scanner-running = Scanner — en marche
topbar-scan-memory-channels-frequency-range = Balayer des canaux mémoire ou une plage de fréquences
topbar-grid-tracker-worked-maidenhead-squares = Suivi des carrés — carrés Maidenhead travaillés sur une carte, avec ce qui est entendu
topbar-settings-device-gains-antennas-audio = Réglages — gains des appareils, antennes, périphériques audio
topbar-user-manual-f1 = Manuel d'utilisation (F1)
topbar-range-was-typed-settings-radio = Cette plage a été saisie dans Réglages ▸ Radio ▸ Plage RX, non annoncée par la radio. Élargissez-la, ou videz la case pour utiliser ce que le périphérique annonce de lui-même.
topbar-what-device-reports-about-itself = C'est ce que le périphérique annonce de lui-même. Si la radio couvre bien cette bande, indiquez sa plage réelle dans Réglages ▸ Radio ▸ Plage RX.
topbar-band = BANDE
topbar-mode = MODE

## Top bar: the System box's window chips.

topbar-chip-log = JOURNAL
topbar-chip-awards = DISTINCTIONS
topbar-chip-bands = BANDES
topbar-chip-sat = SAT
topbar-chip-ism = ISM
topbar-chip-mail = MAIL
topbar-chip-mem = MÉM
topbar-chip-scan = SCAN
topbar-chip-hfdl = HFDL
topbar-chip-grid = CARRÉS

## Top bar: the mode-defaults chip.

topbar-changed-agc = CAG
topbar-changed-squelch = squelch
topbar-changed-auto-notch = coupe-bande auto
topbar-changed-stereo = stéréo
topbar-changed-binaural = binaural
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    Retour aux valeurs par défaut de { $mode }. Modifiées par rapport à elles : { $changed }.

    Cliquez pour les rétablir. Les valeurs propres à { $mode } reviennent, et ce que vous avez réglé ici est oublié.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = la radio n'a pas de réglage de débit { $baud } — { $default } sera utilisé à la place
settings-radio-icom-scope-hint =
    Diffuse le balayage de l'oscilloscope de la radio par la liaison CI-V et le dessine en panoramique — la seule image de la bande qu'un appareil à audio démodulé puisse donner, puisque l'audio qu'il envoie est déjà passé par son filtre.

    Exige que la radio soit configurée pour cela : débit USB CI-V réglé sur 115200 et port USB CI-V sur « Détacher de {"["}REMOTE] » (tous deux dans SET > Connectors > CI-V), et le débit ci-dessus réglé en conséquence — en dessous de { $min_baud }, les balayages ne passent pas dans la liaison et l'oscilloscope reste éteint.

    Les balayages partagent le bus USB interne de la radio avec sa propre carte son. Si l'audio reçu commence à couper avec l'oscilloscope allumé, cette case est la première chose à essayer de désactiver.
settings-radio-rx-tx-words = { $rx } / TX { $tx }
settings-radio-soapy-native = Le matériel ci-dessus est pris en charge directement par sdroxide : { $names }. Choisir cette interface ci-dessus vous donne ses propres réglages — étages de gain, filtres et coupe-bandes que SoapySDR ne sait pas exprimer — et n'exige aucun module SoapySDR.
settings-radio-rx888-width-hint = La part du spectre numérisé que le panoramique affiche à la fois. Toute la chaîne DSP tourne à cette largeur, donc une largeur plus grande coûte proportionnellement plus de CPU — 1/2 est toute la bande dans la cascade, et une quantité sérieuse de calcul. Au-dessus de la limite VHF ({ $crossover } MHz), le panoramique ne regarde pas l'antenne mais la FI de l'accord, qui fait 8 MHz de large et est placée à { $if_centre } MHz — donc tout ce qui est plus large que { $widest } ne peut pas y être centré, et la largeur en plus est du spectre mort à côté du signal plutôt que davantage de signal.
settings-radio-sdrplay-serial-missing = Le numéro de série { $serial } ne figure pas parmi les récepteurs que signale le service SDRplay. Choisissez l'un des récepteurs listés, ou rebranchez celui-ci et appuyez sur Rechercher.
settings-radio-hdr-filter-hint =
    Le filtre analogique placé devant le trajet HDR — une commande différente de la largeur de bande propre au récepteur. Chaque réglage est construit à un ensemble fixe de centres et ne peut rien faire ailleurs ; celui-ci est construit à { $centres }. S'applique immédiatement.

    Le trajet lui-même ne fonctionne pas encore — voir le commutateur ci-dessus — donc ceci choisit un filtre pour un mode qui est actuellement muet.
settings-radio-rspduo-only = Seul un RSPduo a un second accord, et le récepteur sélectionné ici est un { $model }, donc cela ne fera rien.
settings-radio-lime-tx-gain-low = Le gain d'émission est de { $db } dB, au bas de sa plage — quelques microwatts sortant de la carte, qui apparaîtront comme rien sur un wattmètre, quoi qu'il y ait en aval. C'est la valeur par défaut pour qu'un émetteur armé ne puisse pas vous surprendre ; augmentez-la avant de manipuler.

## Settings window, General: SWR

settings-general-swr-tune-limit = (accord : { $limit }:1)

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Cliquez pour faire défiler les échelles du wattmètre : { $faces }
topbar-step-down = Diminuer de { $step }
topbar-step-up = Augmenter de { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = Décalage relais { $shift } — réception sur { $rx }, émission sur { $tx } MHz{ $auto }
topbar-tone-and-burst = Émission sur { $tone } sous la voix, et une salve de { $ms } ms à 1750 Hz au début de chaque trafic
topbar-tone-under-voice = Émission sur { $tone } sous la voix
topbar-burst-every-over = Une salve de { $ms } ms à 1750 Hz au début de chaque trafic
topbar-match-tx = CORRESPOND À L'ÉMISSION ({ $tone })
topbar-decimation-on = Décimation en entrée : le récepteur reçoit le milieu du { $msps } Msps que diffuse cette radio, soit 1/{ $n } — { $span } — et le reste est jeté avant d'atteindre le récepteur.
topbar-decimation-off = Décimation en entrée, désactivée : le récepteur voit toute la largeur de { $span } que diffuse cette radio.
topbar-decimation-hint =
    { $what }

    Une largeur plus étroite donne une résolution de cascade plus fine, un bruit de fond plus bas (3 dB par division par deux) et moins de CPU — au prix des bandes de part et d'autre. Le cadran accorde toujours partout ; la radio déplace son OL pour suivre.

    Cliquez pour parcourir : désactivé / 2 / 4 … { $max }.
topbar-rx-gain-hint = Gain RX d'entrée ({ $stage }). Trop écrête le CAN du récepteur et étale les signaux parasites sur la bande ; trop peu le rend sourd.
topbar-rx-gain-stages =
    {""}

    Cet appareil a { $n } étages de gain RX — les autres se trouvent dans Réglages → Périphérique.
topbar-passband-now =
    {""}

    La bande passante est à { $dbfs } dBFS maintenant.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    Coupe l'audio en dessous de cette puissance dans la bande passante de réception, en dBFS. À gauche, c'est ouvert.{ $level } Réglez-le au-dessus du bruit et en dessous du signal.

    Un flux qui arrive avec le CAG propre à la radio déjà intégré — la FI 12 kHz d'un Icom, par exemple — se situe bien plus haut sur cette échelle que la bande de base brute d'un SDR, ce qui explique que la réglette atteigne la pleine échelle.

    Cliquez pour parcourir l'intensité : Faible / Moy. / Forte / Arrêt. Le moteur figure dans les lignes ci-dessous.
topbar-recording-both = Enregistrement de { $audio } et de { $iq }
topbar-recording-audio = Enregistrement de l'audio vers { $audio }
topbar-recording-iq = Enregistrement de l'I/Q vers { $iq }
topbar-auto-record-armed = Enregistrement automatique armé : un fichier par émission, fermé après { $secs } s de silence
topbar-rds-hover = RDS : { $ps }. Cliquez pour les données de la station
topbar-drm-hover = DRM : { $service }. Cliquez pour les détails de l'émission
topbar-drm-no-codec = DRM : { $summary } — verrouillé, mais son audio { $codec } ne peut pas être décodé ici. Cliquez pour savoir ce qui manque
topbar-drm-state = DRM : { $summary } — cliquez pour l'état du décodeur
topbar-hd-hover = HD Radio : { $station }. Cliquez pour les détails de l'émission
topbar-hd-state = HD Radio : { $summary } — cliquez pour l'état du décodeur
topbar-binaural-preset = { $preset } dans chaque oreille — { $lo } … { $hi } Hz, les deux bandes latérales
topbar-nr-hover =
    Réduction de bruit : { $strength } — { $engine }

    Cliquez pour changer de moteur ou d'intensité
topbar-clipping = Extrait { $span } — appuyez à nouveau pour le recommencer
topbar-record-clip = Enregistrer { $span } et arrêter — un extrait à joindre à un rapport de réception
topbar-starting-clip = démarrage d'un extrait de { $span }…
topbar-stops-in = s'arrête dans { $time }
topbar-auto-record-hint = Enregistrer chaque émission dans son propre fichier, fermé après { $secs } secondes de silence
topbar-recording-silence = enregistrement · { $secs } s de silence (se ferme à { $hold })
topbar-tone-open = Réception de { $tone }, qui est le squelch à tonalité — audio ouvert
topbar-tone-closed = Réception de { $heard }, mais le squelch à tonalité attend { $wanted } — l'audio reste fermé
topbar-tone-heard = Réception du CTCSS/DCS { $tone }
topbar-tone-nothing = Squelch à tonalité { $tone } : rien qui lui corresponde n'est reçu
topbar-tone-use = UTILISER { $tone }
topbar-sockets = Prises : { $sockets }
topbar-voice-keyer-tx = Émission de { $slot } — cliquez pour ouvrir le manipulateur vocal
topbar-drive-uncal-gen = Cette fréquence, qui ne se trouve sur aucune bande amateur, n'est pas étalonnée : le réglage atteint donc l'émetteur tel quel.
topbar-drive-uncal-band = { $band } n'est pas étalonnée : le réglage atteint donc l'émetteur tel quel.
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = Cette fréquence, qui ne se trouve sur aucune bande amateur, est étalonnée à { $trim } dB : le même réglage envoie { $diff } dB { $more ->
        [less] de moins
       *[more] de plus
    } sur l'antenne ici que sur une bande non étalonnée.
topbar-drive-cal-band = { $band } est étalonnée à { $trim } dB : le même réglage envoie { $diff } dB { $more ->
        [less] de moins
       *[more] de plus
    } sur l'antenne ici que sur une bande non étalonnée.
topbar-drive-hover =
    La force avec laquelle l'émetteur est piloté. Sur une radio que sdroxide module lui-même, cela met à l'échelle les échantillons modulés ; sur un appareil doté de son propre réglage de puissance, c'est la fraction de la puissance nominale demandée à l'appareil.

    { $trim }

    Réglages → Radio → Excitation à l'émission par bande.
topbar-cessb-hover =
    SSB à enveloppe contrôlée : plus de puissance moyenne pour le même crête, sans éclaboussures — trois ou quatre décibels de volume apparent à l'autre bout. De combien de décibels la voix est poussée dans le processeur ; 0 est désactivé, 6 est un premier essai raisonnable. Cela ne touche pas à l'Excitation.

    Maintenant : { $now }
topbar-waterfall-rate = { $rows } lignes par seconde — { $seconds } secondes de bande
topbar-detail-auto-columns = { $n } colonnes, quel que soit le choix d'Auto ici
topbar-n-columns = { $n } colonnes
topbar-fft-bins-whole = { $n } cases sur tout le flux — une par colonne, ou plus fin
topbar-fft-bins-pooled = { $n } cases regroupées dans les { $cols } colonnes du panoramique : des signaux plus nets, pas davantage de signaux
topbar-sat-locked = Satellite — verrouillé sur { $name }
topbar-filter-edges = { $lo } … { $hi } Hz depuis la porteuse
topbar-filter-isb = Filtre de réception : { $width } Hz dans chaque oreille ({ $edges }). Les deux bandes latérales de l'ISB sont des émissions distinctes, donc la largeur est celle de l'une d'elles.
topbar-filter-width = Filtre de réception : { $width } Hz de large ({ $edges }).
topbar-filter-hint =
    { $what }

    Cliquez pour la largeur, les deux bords et les préréglages de ce mode, en chiffres — les poignées du panoramique placent une bande passante à l'œil, c'est ici qu'on saisit une valeur exacte.
topbar-band-outside-range =
    { $band } est hors de ce que reçoit cette radio ({ $ranges } MHz).

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band } : { $verdict } ({ $time ->
        [day] de jour
       *[night] de nuit
    }) — prévision de HAMQSL.com à partir des indices solaires, non une mesure de votre propre trajet.

## Top bar: short words and captions

topbar-squelch-open = ouvert
topbar-off = arrêt
topbar-recording = enregistrement
topbar-sql = sql
topbar-stations = stations
topbar-caption-reaction = réaction
topbar-caption-scroll = défilement
topbar-caption-surface = surface
topbar-caption-flow = flux
topbar-caption-detail = détail
topbar-floor = plancher

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = AVIONS
pane-chart = GRAPHE
pane-chat = DISCUSSION
pane-decodes = DÉCODAGES
pane-files = FICHIERS
pane-heard = ENTENDUS
pane-image = IMAGE
pane-map = CARTE
pane-messages = MESSAGES
pane-monitor = MONITEUR
pane-panel = PANNEAU
pane-qso = QSO
pane-reading = LECTURE
pane-receive = RÉCEPTION
pane-saved = ENREGISTRÉS
pane-send = ÉMISSION
pane-spots = SPOTS
pane-stations = STATIONS
pane-status = ÉTAT
pane-terminal = TERMINAL
pane-text = TEXTE
pane-traffic = TRAFIC
pane-vessels = NAVIRES
pane-wfall = CASCADE

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}ENREGISTRER{" "}
panels-nothing-decoded-save = Rien de décodé à enregistrer
panels-clear-rx = {" "}EFFACER RX{" "}
panels-empty-receive-window-nothing-air = Vider la fenêtre de réception. Rien de ce qui est en ondes ne s'arrête.
panels-nothing-received-clear = Rien de reçu à effacer
panels-save-what-panel-has-decoded = Enregistrer dans un fichier ce que ce panneau a décodé
panels-prop = PROP
panels-shade-map-where-signals-are = Ombrer la carte selon l'endroit où les signaux passent réellement, sur chaque bande. Chaque réception est placée au milieu de son trajet — la portion de l'ionosphère qui l'a courbé — et non à la station distante, donc c'est une carte du ciel plutôt que des endroits où vivent les radioamateurs. Construite à partir des trajets auxquels cette station a pris part, plus — lorsque le Reverse Beacon Network est activé dans Réglages → Spots — ce qu'entendent les skimmers du monde, ce qui couvre les bandes sur lesquelles cette radio n'est pas.
panels-night = NUIT
panels-shade-where-sun-down-twilight = Ombrer là où le Soleil est couché, et le crépuscule entre les deux, pour que la ligne grise apparaisse sur la carte. Les bandes basses s'allongent et les hautes se referment du côté nuit de celle-ci, et la ligne de terminateur elle-même est là où se trouve le DX.
panels-all-bands = TOUTES LES BANDES
panels-every-band-once-one-hue = Chaque bande à la fois, une teinte chacune — conditions d'ensemble.
panels-one-band = UNE BANDE
panels-one-band-cold-hot-blue = Une bande, du froid au chaud : bleu, vert, jaune, rouge.
panels-frequency-you-are-actually-working = La fréquence que vous travaillez réellement : le cadran plus le décalage de tonalité à côté. C'est ce qui va dans le journal — le cadran seul est plus bas de cette valeur.
panels-send-return = ENVOYER AU RETOUR
panels-freq = ⇵ FRÉQ
panels-nothing-agreed-mode-save-your = Rien de convenu pour ce mode — enregistrez la vôtre ci-dessous.
panels-forget-saved-frequency = Oublier cette fréquence enregistrée
panels-dial-already-one-your-saved = Ce cadran est déjà l'une de vos fréquences enregistrées.
panels-dial-already-agreed-list-mode = Ce cadran figure déjà dans la liste convenue pour ce mode.
panels-decode-squelch-raise-stop-decoding = Squelch de décodage — montez-le pour cesser de décoder le bruit
panels-message-repeats-long-transmit-held = Le message se répète tant que l'émission est maintenue — une météorite capte la partie qui passe.
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}TX ACTIVÉ{"  "}
panels-tx-2 = {"   "}TX{"   "}
panels-call-cq = {" "}APPEL CQ{" "}
panels-nocall = SANS APPEL

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = Trop peu de parole entendue — parlez normalement pendant tout le décompte et réessayez.
txaudio-tx-audio = TX Audio
txaudio-mode-not-voice-mode-so = Ce mode n'est pas un mode vocal, donc rien de ce qu'il envoie ne traverse la chaîne. Les réglages faits ici s'appliquent en BLU, AM et FM.
txaudio-bypass = CONTOURNER
txaudio-switch-every-stage-out-once = Désactive d'un seul coup chaque étage, débruiteur compris, pour que le microphone parte sans traitement. Cliquez à nouveau pour rétablir exactement les étages qui étaient actifs — le moyen le plus rapide de comparer ce que fait la chaîne.
txaudio-denoise = DÉBRUITER
txaudio-rnnoise-microphone-ahead-every-other = RNNoise sur le microphone, avant tous les autres étages, pour que le bruit de la pièce soit retiré avant que la porte ou le compresseur ne puisse le remonter. Ajoute environ 10 ms de retard.
txaudio-digital-voice = VOIX NUMÉRIQUE
txaudio-also-run-freedv-rade-digital = Faire passer aussi la voix numérique FreeDV / RADE par la chaîne. Désactivé par défaut : un vocodeur est entraîné sur une parole naturelle et peut mal décoder une voix fortement traitée, donc essayez-le en ondes avant de vous y fier. L'audio TCI et les modes numériques ne passent jamais par la chaîne.
txaudio-test = Tester
txaudio-rec = {" "}⏺ ENR{" "}
txaudio-play = {" "}▶ LIRE{" "}
txaudio-play-test-phrase-through-chain = Lire la phrase de test à travers la chaîne telle qu'elle est réglée maintenant, dans vos haut-parleurs. Réglez pendant la lecture et entendez le changement. L'audio de réception est remplacé pendant la lecture.
txaudio-recording = enregistrement…
txaudio-no-phrase-yet = pas encore de phrase
txaudio-live = En direct
txaudio-live-2 = {" "}🎧 DIRECT{" "}
txaudio-hear-your-microphone-through-chain = Écoutez votre microphone à travers la chaîne pendant que vous parlez, sans manipuler l'émetteur. Utilisez un casque : sur haut-parleurs, cela crée une boucle de retour. Vous vous entendez avec quelques dizaines de millisecondes de retard, ce qui rend la parole maladroite — réglez les niveaux sur les indicateurs, et jugez le son avec ENR / LIRE. L'audio de réception est remplacé pendant son fonctionnement ; appuyer sur PTT l'arrête.
txaudio-meters = {" "}INDICATEURS{" "}
txaudio-run-your-microphone-through-chain = Faire passer votre microphone par la chaîne pour les seuls indicateurs : rien sur les haut-parleurs, donc aucun retard auquel parler et rien à faire boucler. La façon de régler le gain du micro et de l'entrée, le seuil de la porte et l'intensité du compresseur. L'audio de réception continue ; appuyer sur PTT l'arrête.
txaudio-monitoring-live-use-headphones-speakers = Écoute en direct — utilisez un casque, ou les haut-parleurs reboucleront dans le microphone.
txaudio-voice-modes-only-tci-clients = Modes vocaux uniquement. Les clients TCI et les modes numériques contournent la chaîne ; lorsqu'elle est en service, gardez l'égaliseur et le processeur d'émission propres de la radio à plat.
txaudio-auto-set-stopped-microphone-was = Réglage auto arrêté : le microphone a été pris.
txaudio-microphone-may-clipping-before-gets = {" "}Le microphone écrête peut-être avant d'arriver ici — baissez son propre gain s'il sonne agressif.
txaudio-level = Niveau
txaudio-mic-control-same-one-transmit = La commande Micro — la même que sur la barre d'émission, partagée par tous les profils. 0 dB au milieu, +6 dB en haut.
txaudio-mic = Micro
txaudio-gain-top-mic-microphone-stored = Gain ajouté au Micro pour ce microphone, mémorisé avec le profil. Les seuils des préréglages supposent une voix culminant vers -10 dBFS sur l'indicateur ENTRÉE ; le Réglage auto l'y place. Reste en place sous CONTOURNER, et s'applique aussi à la voix numérique : c'est du niveau, non du traitement.
txaudio-input = Entrée
txaudio-auto-set = Réglage auto
txaudio-auto-set-cancelled = Réglage auto annulé.
txaudio-talk-now-you-would-air = Parlez maintenant, comme vous le feriez en ondes…
txaudio-db = -∞ dB
txaudio-chain = Chaîne
txaudio-order-audio-meets-them = Dans l'ordre où l'audio les rencontre
txaudio-switch-stage = Activer cet étage
txaudio-earlier-chain = Plus tôt dans la chaîne
txaudio-later-chain = Plus tard dans la chaîne
txaudio-output-limiter = Le limiteur de sortie
txaudio-output = Sortie
txaudio-trim-limiter-always-last = Ajustement et limiteur — toujours en dernier
txaudio-profiles = Profils…
txaudio-gate-expander = Porte / expandeur
txaudio-pushes-room-noise-fan-rumble = Repousse le bruit de la pièce, le ronflement du ventilateur et le souffle dans les pauses. Réglez le seuil juste au-dessus de votre bruit de fond et en dessous de vos syllabes les plus faibles.
txaudio-gate = PORTE
txaudio-hard-gate-10-1-down = Porte franche : 10:1, jusqu'à -40 dB
txaudio-expander = EXPANDEUR
txaudio-gentle-expander-2-1-never = Expandeur doux : 2:1, jamais plus de -15 dB — la pièce ne devient jamais morte
txaudio-reset = RÉINITIALISER
txaudio-back-defaults = Revenir aux valeurs par défaut
txaudio-threshold = Seuil
txaudio-below-gate-starts-close = En dessous de cette valeur, la porte commence à se fermer.
txaudio-return = Retour
txaudio-hysteresis-once-open-closes-only = Hystérésis : une fois ouverte, elle ne se referme qu'à cette distance sous le seuil, pour qu'une voix qui hésite autour ne fasse pas trembler la porte.
txaudio-hold = Maintien
txaudio-how-long-stays-open-after = Combien de temps elle reste ouverte après la chute de la voix, pour que les fins de mots ne soient pas coupées.
txaudio-release = Relâchement
txaudio-how-fast-closes-once-hold = La vitesse de fermeture une fois le maintien terminé. Allongez-la si la porte pompe.
txaudio-floor = Plancher
txaudio-most-ever-turns-pauses-down = Le maximum dont elle baisse les pauses.
txaudio-ratio = Rapport
txaudio-how-steeply-closes-below-threshold = La pente avec laquelle elle se ferme sous le seuil.
txaudio-look-ahead = Anticipation
txaudio-opens-much-before-word-starts = S'ouvre cette durée avant le début d'un mot, pour que sa première consonne ne soit pas coupée. Ajoute le même retard à tout ce qui est émis.
txaudio-high-pass-low-shelf-two = Passe-haut, plateau grave, deux pics, plateau aigu et passe-bas. Faites glisser un point sur la courbe, ou réglez-le ci-dessous. Réglez le passe-bas sur la largeur de votre filtre d'émission.
txaudio-flat = PLAT
txaudio-every-band-back-0-db = Chaque bande ramenée à 0 dB, les deux bords désactivés
txaudio-freq = Fréq
txaudio-gain = Gain
txaudio-q-slope = Q / Pente
txaudio-high-pass = Passe-haut
txaudio-removes-handling-noise-breath-pops = Supprime le bruit de manipulation, les souffles et le ronflement en dessous de la coupure.
txaudio-low-shelf = Plateau grave
txaudio-body-mud-cut-little-here = Le corps, ou la boue : coupez un peu ici pour une voix qui sonne boîteuse.
txaudio-peak-1 = Pic 1
txaudio-bump-dip-voice-band-often = Une bosse ou un creux dans la bande vocale — souvent une coupure vers 300–500 Hz.
txaudio-peak-2 = Pic 2
txaudio-presence-lift-around-2-3 = Présence : une remontée vers 2–3 kHz est ce qui perce en BLU.
txaudio-high-shelf = Plateau aigu
txaudio-brightness = Brillance.
txaudio-low-pass = Passe-bas
txaudio-stops-energy-above-transmit-filter = Empêche l'énergie au-dessus du filtre d'émission d'être dépensée dans le modulateur.
txaudio-t-24-db-oct = 24 dB/oct
txaudio-t-12-db-oct = 12 dB/oct
txaudio-click-steeper-gentler-slope = Cliquez pour une pente plus raide ou plus douce
txaudio-de-esser = Dé-esseur
txaudio-turns-down-only-sibilant-band = Ne baisse que la bande sifflante lorsque « s » et « ch » dépassent le seuil, pour que le reste de la voix garde son niveau. Trouvez la fréquence en la balayant pendant que vous dites « sister silver cellophane » et arrêtez-vous là où le sifflement est le plus fort.
txaudio-frequency = Fréquence
txaudio-centre-sibilant-band = Centre de la bande sifflante.
txaudio-width-band-higher-narrower = Largeur de la bande — plus haut, c'est plus étroit.
txaudio-band-level-where-reduction-starts = Niveau de la bande où la réduction commence. Trop bas, elle travaille tout le temps, et ternit la voix.
txaudio-amount = Quantité
txaudio-most-band-ever-turned-down = Le maximum dont la bande est jamais baissée. Commencez vers -6 dB.
txaudio-attack = Attaque
txaudio-how-fast-reacts = La vitesse de réaction.
txaudio-how-fast-lets-go = La vitesse de relâchement.
txaudio-compressor = Compresseur
txaudio-brings-quiet-syllables-up-holds = Remonte les syllabes faibles et retient les fortes. Visez 3–6 dB de réduction sur les crêtes ; plus de 10 dB est une compression évidente, mieux adaptée aux concours.
txaudio-level-above-which-starts-compress = Niveau au-dessus duquel la compression commence.
txaudio-how-hard-compresses-above-threshold = La force de la compression au-dessus du seuil.
txaudio-how-fast-reacts-peak-10 = La vitesse de réaction à une crête. 10–25 ms garde les consonnes nettes.
txaudio-how-fast-recovers = La vitesse de récupération.
txaudio-knee = Genou
txaudio-soft-knee-width-0-hard = Largeur du genou souple ; 0 est un genou dur.
txaudio-makeup = Compensation
txaudio-gain-after-compression-bring-level = Gain après la compression, pour remonter le niveau.
txaudio-drive = Excitation
txaudio-gain-ahead-compressor-so-more = Gain avant le compresseur, pour que davantage de voix franchisse le seuil — augmente la puissance moyenne. À associer au limiteur de sortie.
txaudio-phase = Phase
txaudio-all-pass-phase-rotator-ahead = Rotateur de phase passe-tout placé avant le détecteur. Équilibre les crêtes déséquilibrées qu'a une voix, si bien qu'un même niveau de crête porte davantage de puissance moyenne. 4 est un réglage courant en radiodiffusion ; 0 est désactivé.
txaudio-last-stage-trim-look-ahead = Le dernier étage : un ajustement, et un limiteur à anticipation qui maintient chaque crête sous le plafond sans les éclaboussures qu'un écrêtage dur produit. Désactivé par défaut, pour qu'activer la chaîne ne modifie pas un niveau d'excitation déjà réglé.
txaudio-limiter = Limiteur
txaudio-on = Activé
txaudio-ceiling = Plafond
txaudio-no-peak-leaves-chain-above = Aucune crête ne quitte la chaîne au-dessus de cette valeur.
txaudio-trim = Ajustement
txaudio-output-gain-ahead-limiter = Gain de sortie, avant le limiteur.
txaudio-off = {"  "}arrêt
txaudio-meters-move-while-you-transmit = Les indicateurs bougent pendant que vous émettez une voix, que vous lisez la phrase de test, ou que vous utilisez DIRECT ou INDICATEURS.
txaudio-profiles-2 = Profils
txaudio-profile-whole-chain-every-stage = Un profil est toute la chaîne : les réglages de chaque étage, lesquels sont actifs, leur ordre, et le gain d'entrée de votre microphone. Charger un profil remplace la chaîne ; la commande Micro et l'excitation restent telles quelles.
txaudio-name = Nom
txaudio-save = Enregistrer
txaudio-store-chain-now-under-name = Mémoriser la chaîne telle qu'elle est maintenant sous ce nom, en remplaçant un profil du même nom
txaudio-import = Importer…
txaudio-read-profile-json-file = Lire un profil depuis un fichier .json
txaudio-tx-audio-profile = Profil d'audio d'émission
txaudio-no-profiles-yet-save-stores = Aucun profil pour l'instant. Enregistrer mémorise la chaîne que vous avez configurée maintenant.
txaudio-load = Charger
txaudio-export = Exporter…
txaudio-delete = Supprimer
txaudio-starting-points = Points de départ
txaudio-set-your-level-auto-set = Réglez d'abord votre niveau avec Réglage auto — les seuils des préréglages le supposent — puis ajustez à partir d'ici plutôt que d'émettre tel que chargé : enregistrez une phrase et écoutez après chaque changement. Un préréglage conserve votre gain d'entrée.
txaudio-profile-loaded = Profil chargé.

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = PROP
solar-where-signals-are-actually-getting =
    Là où les signaux passent réellement, bande par bande. Chaque réception à laquelle cette station prend part — WSPR, FT8/FT4, JS8, et le journal — est placée au milieu de son trajet, c'est-à-dire la portion de l'ionosphère qui l'a courbé, plutôt qu'à la station distante. C'est donc une carte du ciel, et non des endroits où vivent les radioamateurs.

    Elle ne peut montrer que les trajets dont cette station a été l'une des extrémités. Les océans s'allument parce que les milieux des longs trajets s'y trouvent ; l'Antarctique reste sombre parce que personne n'y émet. Désactivé par défaut — elle peint la planète entière.
solar-orbits = ORBITES
solar-orbital-paths = Trajectoires orbitales
solar-clouds = NUAGES
solar-cloud-cover-noaa-s-global = La couverture nuageuse de la mosaïque géostationnaire mondiale de la NOAA, dessinée comme une épaisseur d'air plutôt que comme une image collée à la surface : l'infrarouge indique la température de chaque sommet nuageux, et c'est ainsi qu'on sait à quelle hauteur il se trouve. Les orages éclatent dans les tours qu'ils construisent.
solar-planets = PLANÈTES
solar-other-seven-planets-eighteen-their = Les sept autres planètes, dix-huit de leurs lunes, et les anneaux de Saturne et d'Uranus. Les positions des planètes viennent du jeu d'éléments képlériens du JPL ; les lunes sont des orbites circulaires ajustées sur JPL Horizons.
solar-coronal-mass-ejection-trajectory-cones = Cônes de trajectoire des éjections de masse coronale
solar-sun-obs = OBS SOLEIL
solar-solar-observations-sun-s-disk = Observations solaires sur le disque du Soleil : régions actives des taches solaires, et d'où viennent les éruptions
solar-labels = ÉTIQUETTES
solar-body-region-labels = Étiquettes des corps et des régions
solar-small-bodies = PETITS CORPS
solar-names-asteroids-comets-they-are = Noms portés par les astéroïdes et les comètes. Ils sont dessinés dans tous les cas — ceci indique seulement si trente-cinq désignations sont dessinées avec eux. Quel que soit ce réglage, le corps sur lequel est la caméra et ce que la boîte de recherche a trouvé restent nommés.
solar-decoded-ft8-ft4-stations-path = Stations FT8/FT4 décodées et le trajet vers la station travaillée
solar-sats = SATS
solar-amateur-radio-satellites-their-orbits = Satellites radioamateurs, leurs orbites et leur élévation depuis votre QTH
solar-aurora = AURORE
solar-auroral-oval-noaa-s-ovation = L'ovale auroral du modèle OVATION de la NOAA, dessiné comme des couches d'émission à leurs altitudes réelles, avec le bord tourné vers l'équateur du contour à 10 % marqué sur la surface
solar-awards = DISTINCTIONS
solar-award-coverage-every-dxcc-entity = Couverture des distinctions : chaque entité DXCC peinte selon ce qui manque encore à votre journal. L'orange brûle là où vous n'avez jamais travaillé l'entité, l'ambre là où vous l'avez fait sans confirmation, un vert atténué lorsqu'une QSL est revenue. Suit le filtre de bande de la fenêtre DISTINCTIONS.
solar-view = VUE
solar-which-body-camera-orbits-animated = Autour de quel corps la caméra tourne, et la visite animée
solar-layers = COUCHES
solar-what-drawn-orbits-clouds-cmes = Ce qui est dessiné : orbites, nuages, CME, étiquettes, QSO, aurore
solar-which-sdo-channel-wraps-sun = Quel canal SDO enveloppe le Soleil, et son âge
solar-weather = MÉTÉO
solar-aurora-propagation-numbers-which-bands = L'aurore, les indices de propagation et les bandes ouvertes
solar-space-weather = MÉTÉO SPATIALE
solar-nothing-measured-yet = rien de mesuré pour l'instant
solar-scale = ÉCHELLE
solar-body-moon-orbit-exaggeration = Exagération du corps et de l'orbite lunaire
solar-time = TEMPS
solar-scrub-whole-scene-forward-back = Faire défiler toute la scène en avant et en arrière
solar-activity = ACTIVITÉ
solar-replay-last-hour-decodes-globe = Rejouer la dernière heure de décodages sur le globe
solar-what-propagation-heat-map-showing = Ce qu'affiche la carte thermique de propagation
solar-propagation = PROPAGATION
solar-auto = ▶ AUTO
solar-fly-spline-through-set-framed = Survoler une spline à travers un ensemble de points de vue cadrés. Toute action de la souris l'annule.
solar-moons-ride-circular-orbits-fitted = Les lunes suivent des orbites circulaires ajustées sur JPL Horizons : à un degré ou deux près de leur position réelle, et jusqu'à six pour Miranda, dont le plan orbital oscille trop vite pour qu'un cercle puisse le suivre.
solar-fetch-everything-again-now = Tout récupérer à nouveau maintenant
solar-all-sats = TOUS LES SATS
solar-show-every-satellite-element-set = Afficher chaque satellite du jeu d'éléments, pas seulement les plus populaires. Les anneaux d'orbite restent sur la sélection réduite — quatre-vingt-dix d'un coup sont illisibles.
solar-starting = démarrage…
solar-body = corps{" "}
solar-moon-orbit = orbite lunaire{" "}
solar-stretch-earth-moon-distance-so = Étirer la distance Terre→Lune pour que la paire soit visible séparément
solar-sun = soleil{" "}
solar-sun-radius-exaggeration-leave-1 = Exagération du rayon du Soleil. Laissez à 1× pour garder la géométrie des CME lisible — un Soleil gonflé avale la base de chaque cône.
solar-show = AFFICHER
solar-paint-globe-what-getting-through = Peindre le globe selon ce qui passe
solar-paints-globe-what-getting-through = Peint le globe selon ce qui passe, sur chaque bande. Chaque réception est placée au milieu de son trajet — là où l'ionosphère a fait le travail — plutôt qu'à la station distante. Construit à partir de ce que cette station entend, plus les skimmers du monde lorsque le Reverse Beacon Network est activé.
solar-display = AFFICHAGE
solar-all-bands = TOUTES LES BANDES
solar-every-band-once-one-hue = Chaque bande à la fois, une teinte chacune, en mélangeant là où deux se chevauchent. La vue pour « à quoi ressemblent les conditions » plutôt que « le 20 m est-il ouvert ».
solar-one-band = UNE BANDE
solar-one-band-cold-hot-blue = Une bande, du froid au chaud : bleu, vert, jaune, rouge.
solar-band = BANDE
solar-nothing-heard-yet = rien d'entendu pour l'instant
solar-bands = BANDES
solar-sources = SOURCES
solar-contacts-logbook-path-was-open = Contacts du journal. Un trajet qui était ouvert, sans rapport de signal digne de ce nom — un RST n'est pas un rapport signal/bruit, donc ceux-ci comptent dans l'affluence d'une cellule et jamais dans sa marge.
solar-stations-reported-hearing-one-downloaded = Stations qui ont déclaré entendre celle-ci, téléchargées depuis WSPRnet. Le seul retour qu'obtient jamais une balise émettrice.
solar-reverse-beacon-network-skimmers-what = Skimmers du Reverse Beacon Network : ce qu'entendent tous les autres, sur chaque bande, y compris celles où cette radio n'est pas. Plus grossier que le reste — une ligne RBN ne porte aucun locator, donc les deux extrémités sont placées au centre de leur pays, ce qui est proche pour un petit pays et très faux pour un grand. Activez-le dans Réglages → Spots.
solar-decodes-station-made-each-measured = Décodages faits par cette station. Chacun est mesuré par rapport au plancher de son propre mode, si bien qu'un rapport WSPR et un rapport FT4 sont comparables.
solar-memory = MÉMOIRE
solar-half-life = demi-vie{" "}
solar-how-long-reception-takes-count = Le temps qu'une réception met à compter pour moitié moins. La mémoire propre de l'ionosphère est courte — une ouverture vieille de deux heures ne devrait pas se disputer avec une de deux minutes.
solar-live = DIRECT
solar-follow-band-happens = Suivre la bande au fil de l'eau
solar-replay = ⏸ REJOUER
solar-replay-2 = ▶ REJOUER
solar-replay-last-hour-decodes-over = Rejouer la dernière heure de décodages, en boucle
solar-min-ago = {" "}min
solar-where-last-hour-globe-showing = Où, dans la dernière heure, le globe affiche
solar-trail = traînée{" "}
solar-how-long-decode-s-arc = Combien de temps l'arc d'un décodage reste sur le globe derrière la tête
solar-speed = vitesse{" "}
solar-how-much-faster-than-real = De combien plus vite que le temps réel tourne le rejeu
solar-qso-layer-off = couche QSO désactivée
solar-no-decodes-yet = aucun décodage pour l'instant
solar-qth-not-set-enter-your = QTH non défini — saisissez votre carré dans les Réglages
solar-mode = MODE
solar-path = TRAJET
solar-sent = ENVOYÉ
solar-rcvd = REÇU
solar-grid = CARRÉ
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = Géostationnaire — toujours au-dessus de votre horizon.
solar-no-passes-next-48-hours = Aucun passage dans les 48 prochaines heures depuis votre QTH.
solar-start = DÉBUT
solar-max-el = ÉL MAX
solar-aos-los-are-azimuths-horizon = AOS/LOS sont des azimuts à l'horizon. Les heures sont en UTC.
solar-locked-unlock = {" "}● VERROUILLÉ — DÉVERROUILLER{" "}
solar-release-satellite-lock = Libérer le verrouillage du satellite
solar-lock = {" "}VERROUILLER{" "}
solar-track-satellite-main-window-doppler = Suivre ce satellite dans la fenêtre principale : RX et TX corrigés de l'effet Doppler, liaison montante selon le répéteur, pilotage du rotateur
solar-no-frequencies-file-one-add = Aucune fréquence enregistrée pour celui-ci — ajoutez-les dans Réglages ▸ TLE.
solar-frequencies-yours = FRÉQUENCES · VOS
solar-frequencies = FRÉQUENCES
solar-link = LIAISON
solar-downlink-mhz = DESCENDANTE (MHz)
solar-uplink-mhz = MONTANTE (MHz)
solar-doppler-shifts-these-few-khz = L'effet Doppler les décale de quelques kHz pendant un passage en orbite basse.
solar-satellite-planet-comet = satellite, planète ou comète
solar-planet-asteroid-comet = planète, astéroïde ou comète
solar-clear-search = Effacer la recherche
solar-no-match = aucune correspondance
solar-no-sounder = pas de sondeur
solar-heard = ENTENDU ≥
solar-kp = Kp / A
solar-x-ray = Rayons X
solar-observed-above-sounder-better-than = observé au-dessus du sondeur — mieux que modélisé
solar-power-n-s = puissance N/S
solar-edge-n-s = bord N/S
solar-kp-peak-24-h = pic Kp 24 h
solar-observed-forecast = observé | prévision
solar-bands-open = BANDES OUVERTES
solar-dxcc-coverage = COUVERTURE DXCC
solar-ir-vis = IR+VIS
solar-ir-only = IR seul
solar-no-deep-convection = pas de convection profonde
solar-t-1-storm = 1 orage
solar-glancing = {" "}· rasant
solar-direction-estimated = {" "}· direction estimée

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = Réglages APRS
setup-packet-setup = Réglages paquet
setup-ft8-ft4-ft2-setup = Réglages FT8 / FT4 / FT2
setup-my-callsign = Mon indicatif
setup-my-grid = Mon carré
setup-station-call = Indicatif de la station
setup-ssid-oe3jjs-10-packet-station = Avec un SSID — OE3JJS-10. Une station paquet utilise par convention un SSID différent de celui de l'opérateur, et rien n'émet tant que ceci n'est pas réglé.
setup-speed = Vitesse
setup-t-9600-needs-radio-s-data = 9600 exige le port de données de la radio. Un trajet micro et haut-parleur le détruit aux deux bouts — c'est la radio, non sdroxide.
setup-tx-delay = Délai d'émission
setup-flags-sent-before-frame-so = Drapeaux envoyés avant une trame, pour que l'autre bout nous entende et verrouille son horloge. Sur un appareil CAT, sdroxide seul passe 165–240 ms à se mettre en ondes, et le temps de disponibilité à l'émission propre à l'appareil s'ajoute.
setup-packet-length = Longueur de paquet
setup-bytes = {" "}octets
setup-most-single-frame-carries-shorter = Le maximum qu'une seule trame porte. Des trames plus courtes survivent sur un trajet limite — seule la trame touchée est renvoyée — mais coûtent plus de surcharge. 128 en HF, 256 lorsque le trajet est bon.
setup-frames-sent-before-waiting-acknowledgement = Trames envoyées avant d'attendre un accusé de réception. Davantage remplit un bon trajet ; sur un mauvais, c'est davantage à renvoyer.
setup-extended = Étendu
setup-ask-mod-128 = Demander le mod-128
setup-extended-sequence-numbers-window-bigger = Numéros de séquence étendus, pour une fenêtre supérieure à sept. Beaucoup de nœuds refusent la demande par un DM, ce qui ressemble exactement à une station qui ne voudrait pas vous parler — donc laissez-le désactivé sauf si vous savez que l'autre bout le veut.
setup-answer-calls = Répondre aux appels
setup-accept-connections = Accepter les connexions
setup-off-winlink-client-which-dials = Désactivé pour un client Winlink, qui compose vers l'extérieur et n'a aucune raison de répondre. Activé pour être joignable comme pair : un appel arrive dans le volet terminal et vous parlez à celui qui l'a passé. Une station dont la liaison est déjà occupée par une session Winlink refuse les appels jusqu'à la fin de celle-ci.
setup-connect-text = Texte de connexion
setup-what-caller-greeted = ce qui accueille un appelant
setup-sent-station-connects-us-once = Envoyé à une station qui se connecte à nous, une fois la liaison établie. Vide n'envoie rien — ce qui paraît cassé à celui qui a appelé.
setup-default-via = Via par défaut
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = Le chemin de répéteurs numériques avec lequel le terminal démarre. Le chemin vers votre nœud local est le même à chaque fois, et le retaper est ce qui fait oublier un relais.
setup-beacon = Balise
setup-t-0-disables-timer = 0 désactive la minuterie
setup-beacon-text = texte de balise
setup-kiss-server = Serveur KISS
setup-serve = Servir
setup-offer-modem-kiss-tnc-socket = Proposer ce modem comme TNC KISS sur un socket, pour que Pat, un client APRS ou la pile AX.25 de Linux puisse utiliser la radio.
setup-aprs-call = Indicatif APRS
setup-callsign-station-beacons-under-ssid = L'indicatif sous lequel cette station émet ses balises, avec son SSID — OE3JJS-9 pour une voiture, -10 pour une passerelle I, -5 pour un téléphone. Laissez-le vide pour utiliser l'indicatif ci-dessus.
setup-no-callsign-station-will-not = Aucun indicatif : cette station n'émettra pas du tout — ni balise, ni message, ni accusé de réception.
setup-symbol = Symbole
setup-two-characters-protocol-carries-them = Les deux caractères tels que le protocole les porte : une table (`/` ou `\`) et un code de symbole. Un chiffre ou une lettre en première position est une superposition, dessinée par-dessus l'icône de la table alternative.
setup-path = Chemin
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = Jusqu'où vous demandez à être répété. C'est le réglage le plus lourd de conséquences du canal : chaque saut multiplie les émissions de tout le réseau sur une fréquence partagée.
setup-position = Position
setup-my-grid-2 = Depuis mon carré
setup-beacon-centre-locator-above-reported = Émettre le centre du locator ci-dessus, annoncé avec l'ambiguïté qu'a réellement un locator — un de six caractères couvre quelques kilomètres, et le dire est honnête. Décochez pour donner à la place des coordonnées exactes.
setup-latitude = Latitude
setup-longitude = Longitude
setup-locator = depuis le locator
setup-not-coordinate = pas une coordonnée
setup-comment = Commentaire
setup-sent-every-beacon = envoyé avec chaque balise
setup-t-0-default-never-beacons-timer = 0 — la valeur par défaut — n'émet jamais de balise sur minuterie. Trente minutes est la convention pour une station fixe ; une station mobile émet plus souvent, mais chaque balise prend le temps d'antenne de quelqu'un d'autre.
setup-compressed = Compressé
setup-compressed-position-format-third-air = Le format de position compressé : un tiers du temps d'antenne et plus précis. Chaque récepteur depuis les années 1990 le lit.
setup-first-goes-out-one-interval = La première part un intervalle à partir de maintenant, non immédiatement.
setup-messages = Messages
setup-acknowledge = Accuser réception
setup-answer-messages-addressed-you-acknowledgement = Répondre aux messages qui vous sont adressés. Un accusé de réception est une émission que cette station fait sans qu'on le lui demande, donc une configuration en réception seule doit le désactiver — et la balise cesse alors aussi de prétendre être joignable.
setup-flags-sent-ahead-every-frame = Drapeaux envoyés avant chaque trame, pour que le modem de l'autre bout entende la porteuse et verrouille son horloge avant le début des données. Il doit durer plus longtemps que tout ce qui se passe entre l'appui sur émettre et le rayonnement : sdroxide seul en passe 165–240 ms sur un appareil CAT, et une radio qui reçoit l'audio par le réseau en met davantage. Trop court, l'autre bout ne se verrouille jamais — l'émission est là et rien ne la décode.
setup-shared-packet-mode = partagé avec le mode paquet
setup-keep-stations = Garder les stations
setup-how-long-station-stays-map = Combien de temps une station reste sur la carte après sa dernière écoute. C'est aussi la référence du fondu de la carte, si bien qu'une fenêtre courte n'affiche que ce qui est en direct.
setup-auto-reply = Réponse automatique
setup-answer-snr-grid-status = Répondre SNR? / GRID? / STATUS?
setup-answer-direct-question-addressed-you = Répondre à une question directe qui vous est adressée, ou à @ALLCALL, avec la réponse plutôt qu'un accusé de réception. Ne répond jamais au trafic d'une autre station, et jamais à soi-même.
setup-heartbeat = Pulsation
setup-off = Désactivé
setup-turbo-does-not-beacon-local = Turbo n'émet pas de balise — c'est la vitesse locale et VHF.
setup-sends-your-callsign-grid-so = Envoie votre indicatif et votre carré pour que d'autres sachent que vous êtes joignable. La première part un intervalle à partir de maintenant, non immédiatement.
setup-beacon-frequency = Fréquence de balise
setup-t-500-1000-hz = 500–1000 Hz
setup-move-each-beacon-free-slot = Déplacer chaque balise vers un créneau libre dans la sous-bande de pulsation, comme le fait JS8Call : c'est là que regardent les stations qui guettent les balises, et cela garde un émetteur sans surveillance hors du QSO de quelqu'un d'autre. Le créneau est choisi au moment où la balise part réellement, à l'écart de tout ce qui est décodé.
setup-working-freq = Fréquence de travail
setup-beacon-where-you-are-working = Émettre la balise là où vous travaillez à la place. Contraire à la convention de bande, mais cela garde tout ce que vous émettez au même endroit.
setup-heartbeat-reply = Réponse à la pulsation
setup-answer-heartbeats-signal-report = Répondre aux pulsations par un rapport de signal
setup-tell-station-beaconed-how-well = Dire à une station qui a émis une balise à quel point vous l'avez bien reçue. Désactivé par défaut : une bande chargée porte une pulsation à chaque créneau, et y répondre à toutes inonderait justement la bande que les pulsations servent à garder calme. Limité à une réponse par station toutes les 15 minutes, et jamais pendant qu'un message arrive encore ou que vous avez quelque chose en attente d'envoi.
setup-status-message = Message d'état
setup-tx-period = Période d'émission
setup-even = Paire
setup-odd = Impaire
setup-auto-sequence = Séquence automatique
setup-decode-depth = Profondeur de décodage
setup-fast = Rapide
setup-normal = Normale
setup-deep = Profonde

    Rapide — une seule passe, sans soustraction de signal : le plus rapide, le moins de décodages.
    Normal — soustraction à plusieurs passes régulière : un peu plus rapide que Profonde, un peu moins complète.
    Profonde — la passe multiple avec points de contrôle : le plus de décodages, et environ 1,2 s sur un créneau chargé.
setup-auto-tx-frequency = Fréquence d'émission automatique
setup-choose-transmit-frequency-automatically-quietest = Choisir la fréquence d'émission automatiquement : l'endroit le plus calme de la période dans laquelle vous émettez, plutôt que la fréquence de la station à laquelle vous répondez. Désactivé, cela ne la maintient PAS — la réponse se fait sur la fréquence de la station appelée. Pour la maintenir, utilisez Maintenir la fréquence d'émission.
setup-hold-tx-frequency = Maintenir la fréquence d'émission
setup-pin-transmit-tone-where-nothing = Fixer la tonalité d'émission là où elle est. Rien ne la déplace : ni la réponse à une station, ni la file d'appels, ni un appel CQ, ni un clic sur un décodage ou sur la cascade. Remplace la Fréquence d'émission automatique. Changer de bande est la seule exception : le décalage que vous avez réglé en dernier sur la nouvelle bande revient avec elle. Pour une licence plus étroite que le plan de bande — sur un cadran britannique de 60 m à 5357 kHz, l'attribution se termine à 5358,0, donc la tonalité doit rester sous 1000 Hz.
setup-tx-watchdog = Chien de garde d'émission
setup-stop-transmitting-after-long-no = Cesser d'émettre après ce délai sans réponse ni action de votre part. 0 le désactive.
setup-give-up-after = Abandonner après
setup-calls = {" "}appels
setup-contest = Concours
setup-everyday-exchange-grid-report-rr73 = L'échange courant : carré, rapport, RR73.
setup-european-vhf-contests-send-signal = Concours VHF européens : envoie un rapport de signal, un numéro de série et votre locator à SIX caractères, dans la mise en forme de message que WSJT-X appelle EU VHF Contest. Les CQ partent en CQ TEST.
setup-serial = Numéro
setup-serial-number-next-exchange-will = Le numéro de série que portera le prochain échange. Il avance tout seul à chaque contact journalisé et est mémorisé entre les sessions — ne le réglez à la main que pour démarrer un concours, ou pour reprendre là où en était un journal papier.
setup-needs-6-character-grid = ⚠ exige un carré à 6 caractères
setup-eu-vhf-contest-messages-carry = Les messages du concours VHF européen portent un locator à six caractères (JN88DD, et non JN88). Tant que Mon carré n'en a pas un, l'échange ordinaire est envoyé à la place.
setup-dxpedition = DXpedition
setup-ordinary-ft8-operation = Fonctionnement FT8 ordinaire.
setup-calling-dxpedition-running-fox-mode = Appeler une DXpedition en mode Fox : appelez depuis au-dessus de 1000 Hz, descendez sur le Fox lorsqu'il répond, et journalisez sur son RR73 sans envoyer 73.
setup-run-pile-up-several-signals = Mener le pile-up : plusieurs signaux à la fois, une file d'appelants, traités d'abord les plus forts et les plus rares. APPEL CQ le lance, ARRÊT QSO le met en veille.
setup-fox-signals = Signaux Fox
setup-once = {" "}à la fois
setup-simultaneous-transmissions-spaced-60-hz = Émissions simultanées, espacées de 60 Hz. Elles se partagent la puissance de l'émetteur, donc davantage de signaux signifie que chacun est plus faible.
setup-tx-audio = Audio d'émission
setup-deviation-fm = excursion, en FM
setup-drive-ssb = excitation, en BLU
setup-grid = Carré
setup-report = Rapport
setup-r-report = R+Rapport

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = , échelle 10 W, remise à l'échelle
amp-t-10-w-scale = , échelle 10 W
amp-t-2-w-scale-rescaled = , échelle 2 W, remise à l'échelle
amp-t-2-w-scale = , échelle 2 W
amp-t-100-w-scale-rescaled = , échelle 100 W, remise à l'échelle
amp-t-100-w-scale = , échelle 100 W
amp-t-20-w-scale-rescaled = , échelle 20 W, remise à l'échelle
amp-t-20-w-scale = , échelle 20 W
amp-rescaled = , remise à l'échelle
amp-and = {" "}ET{" "}
amp-not-while-transmitting-switching-relay = Pas pendant l'émission : commuter un relais sous HF peut l'endommager. Arrêtez d'abord la manipulation.
amp-no-amplifier-set-up-choose = Aucun amplificateur n'est configuré. Choisissez son port dans Réglages → Amplificateur.
amp-settings = RÉGLAGES
amp-dock = ACCROCHER
amp-put-amp-window-back-inside = Remettre la fenêtre AMP dans la fenêtre principale
amp-pop-out = DÉTACHER
amp-give-amp-window-window-own = Donner à la fenêtre AMP sa propre fenêtre, à déplacer sur un autre écran
amp-rig = , depuis l'appareil
amp-rig-flagged-reading = , l'appareil a signalé sa lecture
amp-drv = , depuis DRV
amp-rig-s-swr = , depuis le ROS de l'appareil
amp-ptt-keys-amplifier-radio-off = PTT commande l'amplificateur avec la radio ; OFF le laisse en contournement. Cliquez pour basculer.
amp-switch-antenna-jack = Commuter la prise d'antenne
amp-band-down = Bande précédente
amp-band-up = Bande suivante
amp-tuner = COUPLEUR
amp-tuned = Accordé
amp-tune = ACCORDER
amp-start-tune-then-key-steady = Lancer un accord — puis manipuler une porteuse stable (CW, FM ou RTTY). Appuyez à nouveau pour arrêter.
amp-put-tuner-line-bypass = Mettre le coupleur en ligne, ou le contourner
amp-tuning = Accord en cours…
amp-bottom = BAS
amp-pick-any-reading-either-needle = Choisissez n'importe quelle lecture pour l'une ou l'autre aiguille. L'aiguille du haut est blanche, celle du bas bleue.
amp-forward-power-measured-radio-amplifier = Puissance directe, mesurée par la radio : l'amplificateur n'en signale aucune en OFF
amp-forward-power-taken-drv-amplifier = Puissance directe, prise sur DRV : l'amplificateur n'en signale aucune en OFF, et en contournement la puissance de la radio passe directement
amp-forward-power = Puissance directe
amp-reflected-power-measured-radio-amplifier = Puissance réfléchie, mesurée par la radio : l'amplificateur n'en signale aucune en OFF
amp-reflected-power-worked-out-fwd = Puissance réfléchie, calculée à partir de FWD et du ROS : l'amplificateur n'en signale aucune en OFF
amp-reflected-power = Puissance réfléchie
amp-standing-wave-ratio-red-past = Rapport d'ondes stationnaires : rouge à la limite d'accord (Réglages → Général) ou au-delà. Vérifiez l'antenne
amp-standing-wave-ratio-measured-radio = Rapport d'ondes stationnaires, mesuré par la radio : l'amplificateur n'en signale aucun en OFF
amp-standing-wave-ratio = Rapport d'ondes stationnaires
amp-drive-radio = Excitation depuis la radio
amp-drain-supply = Tension de drain
amp-drain-current = Courant de drain
amp-temp = TEMP
amp-heat-spreader-temperature-yellow-49 = Température du dissipateur : jaune à partir de 49 °C (120 °F), rouge au-dessus de 65 °C (149 °F), hors ligne au-dessus de 70 °C (158 °F). Cliquez pour °C/°F.
amp-c = °C
amp-f = °F
amp-performance = PERFORMANCES
amp-numbers-hold-peaks-bars-are = les chiffres gardent les crêtes · les barres sont en direct
amp-input-power = PUISSANCE D'ENTRÉE
amp-supply-volts-drain-amps = Tension d'alimentation × courant de drain
amp-efficiency = RENDEMENT
amp-output-power-input-power = Puissance de sortie ÷ puissance d'entrée
amp-gain = GAIN
amp-output-power-over-drive-decibels = Puissance de sortie par rapport à l'excitation, en décibels
amp-off-line-over-temperature = HORS LIGNE · température excessive
amp-connected-transmitting = CONNECTÉ · ÉMISSION
amp-connected = CONNECTÉ
amp-connecting = Connexion…
amp-atu-bypassed = COUPLEUR CONTOURNÉ
amp-no-inductance-needed = aucune inductance nécessaire
amp-no-capacitance-needed = aucune capacité nécessaire
amp-c-out-antenna-side = C OUT · côté antenne
amp-c-transmitter-side = C IN · côté émetteur

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = Fragments reçus (allumés) et toujours manquants (sombres)
sstv-sstv = SSTV
sstv-auto = Auto
sstv-signal = Signal
sstv-station-sending-mode-sdroxide-does = Une station émet dans un mode que sdroxide ne connaît pas. Le signal et l'accord sont corrects — il n'existe simplement aucun décodeur pour celui-ci. Rien à corriger de votre côté.
sstv-listening = écoute…
sstv-restart-rx = Relancer la RX
sstv-abandon-picture-being-received-listen = Abandonner l'image en cours de réception et écouter l'en-tête suivant. Pour une émission qui a commencé à se décoder dans le mauvais mode — le récepteur reste sinon engagé jusqu'à la fin de ce mode.
sstv-callsign-last-station-sent-fsk = L'indicatif que la dernière station a envoyé en identification FSK après son image.
sstv-tx-slant = Inclinaison en émission
sstv-transmit-clock-trim-ppm-remove = Réglage de l'horloge d'émission (ppm) pour supprimer l'inclinaison sur le décodeur distant
sstv-reset-0-ppm = Remettre à 0 ppm
sstv-fsk-id = Identification FSK
sstv-send-your-callsign-tones-after = Envoyer votre indicatif en tonalités après chaque image — l'identification que lisent les répéteurs SSTV et d'autres programmes. Ajoute environ 2,5 secondes, et n'envoie rien tant que vous n'avez pas réglé un indicatif.
sstv-tx-lead = Avance d'émission
sstv-silence-sent-after-keying-before = Silence envoyé après la manipulation et avant l'amorce de l'image et le code VIS. Un décodeur qui manque une partie de cet en-tête n'affiche aucune image, donc ceci couvre l'écart entre la demande de PTT à la radio et son passage réel en ondes. 0 pour une SDR qui manipule instantanément.
sstv-live = DIRECT
sstv-waiting-picture = en attente d'une image…
sstv-no-low-audio = pas d'audio / audio faible
sstv-received = REÇUES
sstv-reading-radio-s-pictures = Lecture des images de la radio…
sstv-decoded-pictures-collect-here = Les images décodées s'accumulent ici.
sstv-click-enlarge-right-click-delete = Cliquez pour agrandir · clic droit pour supprimer
sstv-delete-picture = Supprimer cette image
sstv-loading = chargement…
sstv-transmit = ÉMETTRE
sstv-click-edit-slot-s-message = Cliquez pour modifier le message de cet emplacement · double-cliquez pour charger une image
sstv-change-image = Changer d'image…
sstv-load-image = Charger une image…
sstv-clear = Effacer
sstv-empty-slot-s-picture-message = Vider l'image de cet emplacement (le message reste)
sstv-banner = Bandeau…
sstv-what-printed-across-top-every = Ce qui est imprimé en haut de chaque image que cette station envoie
sstv-fetching-slot-s-picture = Récupération de l'image de cet emplacement…
sstv-load-image-into-slot = Charger une image dans cet emplacement →
sstv-drawn-slot-s-image = Dessiné sur l'image de cet emplacement
sstv-abort-tx = {" "}ANNULER L'ÉMISSION{" "}
sstv-delete-picture-store = Supprimer cette image du stock
sstv-received-image = Image reçue
sstv-no-longer-store = plus dans le stock
sstv-loading-full-size = chargement en taille réelle…
sstv-save-picture = Enregistrer l'image…
sstv-delete-sure = Supprimer — vous êtes sûr ?
sstv-click-again-delete-good = Cliquez à nouveau pour la supprimer définitivement
sstv-delete = Supprimer…
sstv-image-banner = Bandeau de l'image
sstv-drawn-into-every-picture-station = Dessiné dans chaque image que cette station émet, par-dessus le message propre à l'emplacement.
sstv-draw-banner = Dessiner le bandeau
sstv-substituted-when-picture-composed = Remplacé lors de la composition de l'image :
sstv-top-left = Haut gauche
sstv-top-right = Haut droit
sstv-colours = Couleurs
sstv-strip-top-edge-fades-black = La bande, sur son bord supérieur — elle s'estompe vers le noir, ou vers la couleur du dégradé situé dessous
sstv-both-texts = Les deux textes
sstv-gradient = Dégradé
sstv-fade = s'estompe vers
sstv-fade-strip-top-colour-second = Faire s'estomper la bande de sa couleur du haut vers une seconde couleur, au lieu du noir.
sstv-text-outline = Contour du texte
sstv-draw-outline-around-banner-text = Dessiner un contour autour du texte du bandeau, pour qu'il reste lisible sur une bande chargée ou claire.
sstv-text-gradient = Dégradé du texte
sstv-fade-banner-text-colour-second = Faire passer le texte du bandeau de sa couleur à une seconde couleur sur toute la largeur de l'image.
sstv-rainbow-text = Texte arc-en-ciel
sstv-all-text-overriding-colours-above = tout le texte, en remplaçant les couleurs ci-dessus
sstv-draw-every-text-picture-banner = Dessiner chaque texte de l'image — bandeau et message — en arc-en-ciel horizontal, en ignorant les couleurs ci-dessus.
sstv-message = Message
sstv-slot-message-s-text-colour = La couleur du texte du message de l'emplacement
sstv-outline = contour en
sstv-height = Hauteur
sstv-how-tall-strip-pixels-transmitted = La hauteur de la bande, en pixels de l'image émise. Le texte est dimensionné d'après elle — une trame SSTV ne fait que 320 pixels de large, et un bandeau plus haut est ce qui le rend lisible à l'autre bout.
sstv-reset = Réinitialiser
sstv-back-callsign-left-version-right = Revenir à l'indicatif à gauche et à la version à droite
sstv-waiting-station-s-digital-mode = En attente du chargement des réglages de mode numérique de la station…
sstv-rifp = RIFP
sstv-size = Taille
sstv-calling-frequency-draft-names = La fréquence d'appel que nomme le brouillon
sstv-encode = Encodage
sstv-how-picture-encoded-into-object = Comment l'image est encodée dans l'objet que transporte RIFP. Auto essaie chacun et envoie le plus petit.
sstv-try-every-encoding-send-smallest = Essayer chaque encodage, envoyer le plus petit (jamais avec perte)
sstv-gray = Gris
sstv-grayscale-depth-rifp-s-raster = Profondeur de gris. La trame de RIFP est en niveaux de gris par définition — la couleur n'a pas sa place dans son manifeste.
sstv-dither = Tramage
sstv-diffuse-quantisation-error-worth-below = Diffuser l'erreur de quantification — à utiliser en dessous de 8 bits
sstv-caption = Légende
sstv-sent-manifest-content-hint-shown = Envoyée dans le manifeste comme indication de contenu, et affichée sous l'image par les récepteurs. Transmise comme texte — elle n'est pas dessinée dans l'image.
sstv-what-picture = Ce que représente cette image
sstv-repeat-data = Répéter les données
sstv-send-every-data-frame-many = Envoyer chaque trame de données ce nombre de fois. RIFP est à sens unique, sans demande de réparation, donc c'est la seule récupération dont dispose un récepteur.
sstv-chunk = Fragment
sstv-payload-octets-per-data-frame = Octets utiles par trame de données (le profil recommande 192)
sstv-valid-frames-frames-failed-crc = Trames valides, trames en échec de CRC, et images complètes vérifiées
sstv-no-transfer-progress = aucun transfert en cours
sstv-forget-incomplete-transfer = Oublier ce transfert incomplet

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = Rappeler
windows-delete = Supprimer
windows-edit-name-frequency-mode-antenna = Modifier le nom, la fréquence, le mode, l'antenne et le réglage du relais
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = Le décalage relais, la tonalité et la salve 1750 Hz mémorisés avec ce canal
windows-abandon-edit-escape = Abandonner la modification (Échap)
windows-save = ENREGISTRER
windows-keep-changes-enter = Garder les modifications (Entrée)
windows-recall-channel-without-moving-antenna = Rappeler ce canal sans déplacer l'antenne
windows-antenna-switch-when-channel-recalled = L'antenne à commuter lorsque ce canal est rappelé. — laisse ce que la radio utilise déjà.
windows-shift = Décalage
windows-tone = Tonalité
windows-invert = INVERSER
windows-normal = NORMAL
windows-dcs-polarity = Polarité DCS
windows-open-every-over-tone-burst = Ouvrir chaque trafic sur la salve de tonalité pendant que ce canal est rappelé
windows-memories = Mémoires
windows-memory-name = nom de la mémoire
windows-store = Mémoriser
windows-folder-name = nom du dossier
windows-new-folder = Nouveau dossier
windows-import = IMPORTER
windows-read-channel-list-chirp-csv =
    Lire une liste de canaux depuis un fichier CSV CHIRP (.csv) — un export d'annuaire de répéteurs, une table de canaux marine ou PMR, ou tout ce que CHIRP sait écrire.

    Chaque canal apporte sa fréquence, son mode, son décalage relais et sa tonalité CTCSS ou DCS. Les canaux déjà présents dans cette liste — même fréquence, même mode — sont ignorés, si bien que réimporter un annuaire mis à jour ajoute ce qui est nouveau au lieu de doubler ce qui ne l'est pas.
windows-chirp-csv = CSV CHIRP
windows-export = EXPORTER
windows-write-list-out-chirp-csv = Écrire cette liste dans un fichier CSV CHIRP, à charger dans un portatif ou à garder en sauvegarde.
windows-no-memories-yet = aucune mémoire pour l'instant
windows-drag-memory-onto-folder-file = faites glisser une mémoire sur un dossier pour la classer, sous elles pour la déclasser
windows-drop-here-unfile = déposer ici pour déclasser
windows-sort = Trier
windows-order-they-were-stored = Dans l'ordre où elles ont été mémorisées
windows-name-ignoring-case = Par nom, sans tenir compte de la casse
windows-frequency = Par fréquence
windows-band-then-frequency-inside-each = Par bande, puis par fréquence à l'intérieur de chacune
windows-descending-click-ascending = Décroissant — cliquez pour croissant
windows-ascending-click-descending = Croissant — cliquez pour décroissant
windows-delete-folder-memories-move-top = Supprimer le dossier — ses mémoires remontent au niveau supérieur
windows-rename-folder = Renommer le dossier
windows-empty-drop-memories-here = vide — déposez des mémoires ici
windows-voice-keyer = Manipulateur vocal
windows-rec-records-your-microphone-play = ENR enregistre depuis votre microphone, LIRE vous permet d'écouter ce que vous avez enregistré, ÉMET le met en ondes — comme le fait une touche du pavé numérique, une pad MIDI, ou send_voice_mem de rigctld.
windows-stop-store = Arrêter et mémoriser
windows-stop = ARRÊT
windows-play = LIRE
windows-stop-listening = Arrêter l'écoute
windows-nothing-recorded-slot = Rien d'enregistré dans cet emplacement
windows-not-while-transmitting = Pas pendant l'émission
windows-listen-message-nothing-transmitted = Écouter ce message — rien n'est émis
windows-stop-transmitting = Arrêter l'émission
windows-tune-active-switch-off-first = ACCORD est actif — désactivez-le d'abord
windows-transmit-message = Émettre ce message
windows-erase-recording = Effacer cet enregistrement
windows-rade-message-encoded-digital-voice = RADE : le message est encodé par le codec de voix numérique, exactement comme le serait un trafic en direct.

## TX Audio window: messages with values

txaudio-rec-hint = Enregistrer une phrase de test depuis le microphone — jusqu'à { $max } s, sans manipuler l'émetteur. Cliquez à nouveau pour arrêter. Elle est conservée sans traitement, pour que LIRE fasse passer la même phrase à travers chaque modification que vous faites.
txaudio-phrase-length = Phrase de { $secs } s
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = Gain d'entrée réglé à { $gain } dB (les crêtes de voix étaient à { $loud } dBFS).{ $clip }
txaudio-listening = Écoute… { $secs } s
txaudio-auto-set-hint = Parlez normalement pendant { $secs } s — comme vous le feriez en ondes — et le gain d'entrée est réglé pour que vos crêtes de voix tournent autour de { $target } dBFS. Cliquez à nouveau pour annuler.
txaudio-eq-point = { $hz } Hz, { $db } dB — glissez pour déplacer
txaudio-not-a-profile = Ce n'est pas un profil d'audio d'émission : { $error }
txaudio-imported = « { $name } » importé.
txaudio-saved = « { $name } » enregistré.
txaudio-deleted = « { $name } » supprimé.
txaudio-loaded-preset = Point de départ « { $name } » chargé.

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · hors ligne
solar-radius-exaggeration = Exagération du rayon Terre/Lune (max { $max }× à cette échelle d'orbite lunaire)
solar-brightest-cell = cellule la plus lumineuse ≈ { $peak } trajets · { $bands } { $bands ->
        [one] bande
       *[other] bandes
    }
solar-hits-in-hour = { $hits } dans l'heure
solar-passes-from = { $name } · PASSAGES DEPUIS { $grid }
solar-elements-age = éléments vieux de { $age } · SGP4
solar-point-at = Pointer à { $az }° ({ $compass }), élévation { $el }°.
solar-published-as = publié sous le nom { $name }
solar-fly-there = { $name } — ↵ pour y aller
solar-sats-tracked = { $n } sur { $total } suivis
solar-bodies-found = { $n } sur { $total } corps
solar-sats-and-bodies = { $n } sur { $total } suivis · { $bodies } sur { $bodies_total } corps
solar-confidence-on-band = { $confidence } sur { $band }
solar-kp-now = { $kp } maintenant
solar-kp-in = { $kp } dans { $hours } h
solar-valid-age = valide { $valid } · { $age } d'ancienneté
solar-full-bar = barre pleine = { $share } du monde
solar-own-paths-memory = où sont allés les trajets propres à cette station · mémoire de { $minutes } min
solar-n-storms = { $n } orages
solar-clouds-legend = NUAGES { $time } · { $age } d'ancienneté · { $channels } · { $storms } · foudre simulée
solar-cme-eta = ETA { $eta } (+{ $hours } h)
solar-cme-arrived = l'arrivée était { $eta } (il y a { $hours } h)
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = CME DIRIGÉE VERS LA TERRE { $time } · { $speed } km/s · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = Réglages { $mode }
setup-transmits-as = Émet sous { $call }
setup-decimal-degrees = Degrés décimaux, négatifs pour { $hemisphere }. Collez-en un directement depuis une carte.

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } PEU FIABLE À CE ROS
amp-lower-drive-first = Baissez d'abord l'excitation : { $drive } W surchargeraient l'amplificateur en ligne (sa limite est de { $limit } W).
amp-live-value = en direct { $value } { $unit }
amp-tripped = DÉCLENCHÉ{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = maximum ≈ { $n } trajets
panels-no-agreed-frequency = Aucune fréquence convenue pour { $mode } — ouvrez ceci pour enregistrer celle sur laquelle vous êtes
panels-mode-frequencies-hint = Les { $n } fréquences pour { $mode } — en choisir une accorde le cadran, et vous pouvez enregistrer la vôtre
panels-mode-frequencies = { $mode } · fréquences
panels-global-convention = Une convention mondiale que le plan de bande de la région UIR { $region } ne réserve pas aux données étroites — vérifiez votre propre plan de bande avant d'émettre ici.
panels-outside-data-segment = Ambre : hors du segment données de la région { $region }.
panels-save-dial = ＋ Enregistrer { $mhz } MHz
panels-save-dial-hint = Mémoriser ce cadran sous { $mode } et le proposer ici désormais. Enregistré sur la station, si bien que chaque écran relié à cette radio le possède.
panels-tr-period = Période E/R de { $period } secondes
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = Ce tour : { $into } s sur { $slot }. Les décodages arrivent à la fin, et c'est alors que le prochain trafic peut commencer.{ $mark }
panels-burst-mark = {" "}Le repère marque où s'arrête la salve, à { $secs } s.
panels-tr-period-burst = Période E/R de { $slot } secondes, salve de { $burst } s

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } { $n ->
        [one] canal
       *[other] canaux
    }
windows-slot-n = Emplacement { $n }
windows-record-from-mic = Enregistrer depuis le microphone (jusqu'à { $max } s)

## SSTV panel: messages with values

sstv-picture-too-big = Cette image fait { $size } Mo — la limite est de { $limit } Mo.
sstv-auto-mode = Auto ({ $mode })
sstv-not-decoded = { $what } — non décodé
sstv-last = dernier : { $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = Les images décodées sont enregistrées { $place } et s'accumulent ici.
sstv-older-in-store = { $n } plus anciennes dans le stock
sstv-older-load-more = { $n } plus anciennes — en charger davantage
sstv-delete-picture-from = Supprimer cette image { $place }
sstv-rifp-profile = { $profile } · ~{ $khz } kHz occupés · le cadran est le centre du canal
sstv-rifp-too-wide = ⚠ { $profile } occupe ~{ $khz } kHz — trop large pour un segment à bande étroite
sstv-rifp-no-frequency = RIFP n'attribue aucune fréquence, et sdroxide l'émettra où que vous accordiez. Un canal de { $khz } kHz ne tient que là où la large bande ou la FM est autorisée — { $segments } — et non dans un segment à bande étroite, à plus forte raison en HF. Même à l'intérieur de ceux-ci, les conditions de votre propre licence peuvent être plus étroites. Vous êtes l'opérateur : vérifiez vos propres règles.
sstv-rifp-tx-frame = ● Trame d'émission { $frame }/{ $frames } · { $secs } s restantes
sstv-rifp-sent-as = envoyé en { $encoding } · { $octets } octets
sstv-rifp-rx-stats = trames { $frames } · mauvaises { $bad } · images { $pictures }
sstv-rifp-session-idle = session { $session } · inactive { $secs } s
sstv-rifp-session-waiting = session { $session } · fragments retenus, en attente du manifeste · inactive { $secs } s
sstv-store-on-radio = sur la radio, dans { $dir }
sstv-store-in = dans { $dir }

## decodes (src/app/panels/decodes.rs)

decodes-decodes = DÉCODAGES
decodes-adif = ADIF
decodes-sort = Trier
decodes-single-list = Liste unique
decodes-every-decode-one-list-newest = Chaque décodage dans une seule liste, le tour le plus récent en premier, au lieu de blocs de tours pairs/impairs. Les puces de tri ordonnent alors toute la liste et non chaque tour, et chaque ligne porte l'heure de son créneau — cyan pour un créneau pair, or pour un impair.
decodes-cq-only = CQ seulement
decodes-only-stations-calling-cq-only = Seulement les stations qui appellent en CQ — et seulement les appels auxquels vous pouvez répondre : un CQ dirigé (DX, EU, JA, POTA, TEST …) est listé lorsqu'il vous nomme, et masqué lorsqu'il en nomme un autre.
decodes-new-only = Nouveaux seulement
decodes-only-stations-would-new-entity = Seulement les stations qui seraient nouvelles : entité, couple bande-créneau, carré ou indicatif
decodes-auto-tx-frq = FRQ émission auto
decodes-overridden-hold-tx-lift-hold = Remplacé par Maintenir la fréquence d'émission. Levez le maintien pour choisir dans quel sens la fréquence d'émission se déplace.
decodes-hold-tx = Maintenir la fréquence d'émission
decodes-pin-transmit-tone-where-nothing =
    Fixer la tonalité d'émission là où elle est. Rien ne la déplace : ni la réponse à une station, ni la file d'appels, ni un appel CQ, ni un clic sur un décodage ou sur la cascade. Désactivez-le pour la déplacer, puis réactivez-le.

    Changer de bande est la seule exception, et c'est un geste de votre part : le décalage que vous avez réglé en dernier sur la nouvelle bande revient avec elle, si bien qu'une valeur de 60 m n'est pas reportée sur 20 m, ni celle de 20 m sur 60 m.

    Pour une licence plus étroite que le plan de bande — sur un cadran britannique de 60 m à 5357 kHz, l'attribution se termine à 5358,0, donc la tonalité doit rester sous 1000 Hz, et l'un ou l'autre déplaceur automatique sortira de la bande entre un trafic et le suivant.
decodes-type-transmit-offset-hz-then = Saisissez le décalage d'émission en Hz, puis appuyez sur Entrée. Lorsque votre licence est plus étroite que le plan de bande, toute la plage sous le bord vous appartient pour choisir, donc rien n'est suggéré ici.
decodes-clear-rx = {" "}EFFACER RX{" "}
decodes-empty-decode-list-nothing-air = Vider la liste des décodages. Rien de ce qui est en ondes ne s'arrête.
decodes-even = PAIR
decodes-text = TEXTE
decodes-dxcc = DXCC
decodes-band = BANDE
decodes-grid = CARRÉ
decodes-dupe = DOUBLON
decodes-reply = RÉPONSE
decodes-queued-click-remove = En file d'attente — cliquez pour retirer
decodes-work-station-after-current-one = Travailler cette station après la station en cours
decodes-qsos-worked-since-sdroxide-was = QSO travaillés depuis le lancement de sdroxide
decodes-qso-complete = ✓ QSO TERMINÉ
decodes-tx = ● TX
decodes-dxpedition-mode-transmit-frequency-held = Mode DXpedition. La fréquence d'émission est tenue hors de la moitié du Fox de la bande passante (sous 1000 Hz) jusqu'à ce que le Fox réponde.
decodes-watchdog = CHIEN DE GARDE
decodes-transmitting-stopped-no-reply-no = Émission arrêtée : aucune réponse et aucune action pendant la période du chien de garde. Lancez un appel CQ ou choisissez un message pour reprendre.
decodes-well-inside-tolerance = Bien dans la tolérance.
decodes-no-active-qso-pick-decode = aucun QSO actif — choisissez un décodage pour répondre, ou lancez un appel CQ
decodes-ft8-engine-idle = moteur FT8 inactif
decodes-being-worked = en cours de travail
decodes-clear = EFFACER
decodes-no-messages = — aucun message —
decodes-call-cq = {"  "}APPEL CQ{"  "}
decodes-stop-qso = {" "}ARRÊT QSO{" "}
decodes-stop-tx = {" "}ARRÊT TX{" "}
decodes-free-text-13-chars = texte libre (13 caractères)
decodes-send = ENVOYER

## sat (src/app/sat.rs)

sat-pass-profile = PROFIL DE PASSAGE
sat-satellite = SATELLITE
sat-satellites = SATELLITES
sat-passes = PASSAGES
sat-set-your-grid-passes-settings = Réglez votre carré pour les passages (Réglages ▸ Général).
sat-next = SUIVANT
sat-peak = CRÊTE ≥
sat-all-satellites = tous les satellites
sat-off-only-satellites-frequencies-file = Désactivé : seulement les satellites dont les fréquences sont enregistrées — ceux que vous pouvez travailler — et ceux que vous avez collés vous-même. Activé : chaque satellite de la liste, ce qui peut représenter des milliers avec un abonnement et prend du temps à rechercher.
sat-no-passes-window-lower-peak = Aucun passage dans cette fenêtre. Abaissez la crête, ou élargissez la fenêtre.
sat-length = DURÉE
sat-peak-2 = CRÊTE
sat-from-to = DE → À
sat-up-now = ● au-dessus de l'horizon
sat-unlock = DÉVERROUILLER
sat-release-lock-dial-stays-where = Libérer le verrouillage ; le cadran reste où il est
sat-azimuth = AZIMUT
sat-elevation = ÉLÉVATION
sat-range = DISTANCE
sat-rate = DÉBIT
sat-downlink = DOWNLINK
sat-doppler-rx = DOPPLER RX
sat-uplink = UPLINK
sat-doppler-tx = DOPPLER TX
sat-no-pass-inside-next-48 = Aucun passage dans les 48 prochaines heures depuis votre QTH.
sat-elements-stale-corrections-suspended-until = ⚠ Éléments périmés — corrections suspendues jusqu'à une mise à jour du TLE
sat-doppler-correction-needs-iq-front = La correction Doppler exige un étage d'entrée IQ — un appareil CAT ne suit que
sat-rotator-not-connected = Rotateur : non connecté
sat-doppler = DOPPLER
sat-apply-computed-correction-rx-tx = Appliquer la correction calculée à RX et TX
sat-rotator = ROTATEUR
sat-steer-antenna-through-rotctld-client = Orienter l'antenne par le client rotctld (Réglages ▸ Serveurs)
sat-name-norad = nom ou NORAD
sat-set-your-grid-passes-settings-2 = réglez votre carré pour les passages (Réglages ▸ Général)
sat-norad = NORAD
sat-pass = PASSAGE
sat-always-visible = toujours visible
sat-not-here = pas visible d'ici
sat-no-elements = aucun élément
sat-pick-satellite-see-links-lock = Choisissez un satellite pour voir ses liaisons et vous y verrouiller.
sat-no-frequencies-file-one-add = Aucune fréquence enregistrée pour celui-ci — ajoutez-les dans Réglages ▸ TLE.
sat-link = LIAISON
sat-downlink-mhz = DOWNLINK (MHz)
sat-uplink-mhz = UPLINK (MHz)
sat-mode = MODE
sat-tune = ACCORD
sat-set-dial-mode-link-no = Régler le cadran et le mode sur cette liaison — ni verrouillage, ni correction
sat-locked = {" "}VERROUILLÉ{" "}
sat-lock = {" "}VERROUILLER{" "}
sat-track-satellite-doppler-correction-rx = Suivre ce satellite : correction Doppler en RX et TX, liaison montante selon le répéteur, et la vue 3D le suit

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ ARRÊT{" "}
wefax-end-chart-now-save-what = Terminer la carte maintenant et enregistrer ce qui est arrivé
wefax-start = {" "}● DÉMARRER{" "}
wefax-start-chart-now-without-waiting = Démarrer une carte maintenant, sans attendre une tonalité de départ
wefax-phasing = phasage…
wefax-subcarrier-offset-1900-hz-tune = Décalage de la sous-porteuse depuis 1900 Hz. Accordez pour environ zéro : la porteuse fax va de 1500 Hz pour le noir à 2300 Hz pour le blanc, et un récepteur décalé de quelques centaines de hertz écrête l'image en noir ou en blanc uniforme.
wefax-auto-start = DÉPART AUTO
wefax-auto-stop = ARRÊT AUTO
wefax-end-450-hz-stop-tone = La terminer sur la tonalité d'arrêt de 450 Hz. Désactivez pour continuer l'enregistrement pendant une station qui envoie plusieurs cartes à la suite.
wefax-phase = PHASAGE
wefax-slant = INCLINAISON
wefax-sample-clock-trim-if-chart = Réglage de l'horloge d'échantillonnage. Si la carte penche vers la gauche, augmentez-le ; vers la droite, diminuez-le. Une carte son décalée de cent ppm dérive d'une ligne sur le côté pour une carte d'un quart d'heure.
wefax-view = AFFICHAGE
wefax-scale-chart-panel-width = Mettre la carte à l'échelle de la largeur du panneau
wefax-whole = ENTIÈRE
wefax-shrink-chart-until-all-view = Réduire la carte jusqu'à ce qu'elle tienne entièrement à l'écran
wefax-half-size = Demi-taille
wefax-one-screen-pixel-per-fax = Un pixel d'écran par pixel de fax — faire défiler pour le détail
wefax-twice-size-scroll-move-around = Double taille ; faites défiler pour vous déplacer sur la carte
wefax-height = HAUTEUR
wefax-stretch-picture-vertically-chart-came = Étirer l'image verticalement. Une carte sortie écrasée ou étirée est en général décodée au mauvais débit de lignes — cela la rend lisible, et les puces LPM la corrigent proprement. Double-cliquez pour saisir une valeur.
wefax-reset = RÉINITIALISER
wefax-back-picture-s-own-proportions = Revenir aux proportions propres de l'image
wefax-follow = SUIVRE
wefax-keep-newest-lines-view-scrolling = Garder les lignes les plus récentes à l'écran. Faire défiler vers le haut désactive ceci pour lire ce qui est déjà arrivé ; revenir tout en bas le réactive.
wefax-saved = ENREGISTRÉES
wefax-path = CHEMIN
wefax-reading-radio-s-charts = Lecture des cartes de la radio…
wefax-finished-charts-collect-here = Les cartes terminées s'accumulent ici.
wefax-delete-chart = Supprimer cette carte
wefax-loading = chargement…
wefax-stations = STATIONS
wefax-broadcast-radiofax-schedules-picking-one = Créneaux de radiofax de radiodiffusion — en choisir un accorde le cadran et règle la vitesse de ligne
wefax-frequencies-are-published-carrier-dial = Les fréquences sont la porteuse publiée ; le cadran est réglé 1,9 kHz en dessous, ce qui est fait pour vous. Les créneaux changent et les stations ferment — considérez cela comme un point de départ pour chercher, non comme un horaire.
wefax-delete-chart-store = Supprimer cette carte du stock
wefax-newer = ◀ PLUS RÉCENTE
wefax-chart-received-after-one = La carte reçue après celle-ci
wefax-older = PLUS ANCIENNE ▶
wefax-chart-received-before-one = La carte reçue avant celle-ci
wefax-no-longer-store = plus dans le stock
wefax-loading-full-size = chargement en taille réelle…
wefax-save = ENREGISTRER
wefax-save-copy-chart-computer = Enregistrer une copie de cette carte sur cet ordinateur
wefax-sure = SÛR ?
wefax-click-again-delete-chart-good = Cliquez à nouveau pour supprimer définitivement cette carte
wefax-delete = SUPPRIMER
wefax-file-chart-was-saved = Le fichier sous lequel cette carte a été enregistrée

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = démarrage du modem APRS…
aprs-aprs = APRS
aprs-no-callsign = aucun indicatif
aprs-nothing-will-transmitted-no-beacon = Rien ne sera émis — ni balise, ni message, pas même un accusé de réception — tant que cette station n'a pas d'indicatif. Réglez-en un dans Réglages → Général, ou un propre à APRS avec son SSID dans CONFIGURATION.
aprs-no-position = aucune position
aprs-beacon-needs-somewhere-report-fill = Une balise a besoin d'un endroit à signaler. Renseignez votre locator dans Réglages → Général, ou indiquez des coordonnées dans CONFIGURATION.
aprs-setup = CONFIGURATION
aprs-callsign-symbol-digipeater-path-position = Indicatif, symbole, chemin de répéteurs, position et balise
aprs-how-often-beacon-your-position = La fréquence d'émission de votre position, sans surveillance. `off` — la valeur par défaut — n'émet jamais de balise : choisir un mode ne doit pas mettre une station en ondes. Trente minutes est la convention pour une station fixe, plus souvent pour une station mobile, et chaque balise prend le temps d'antenne de quelqu'un d'autre.
aprs-beacon = {" "}BALISE{" "}
aprs-station-has-no-callsign-see = Cette station n'a pas d'indicatif — voir l'avertissement à gauche.
aprs-station-has-no-position-report = Cette station n'a pas de position à signaler — voir l'avertissement à gauche.
aprs-send-one-position-report-now = Envoyer un seul rapport de position maintenant, sans attendre l'intervalle. Il attend que le canal soit libre, comme tout le reste.
aprs-stations = STATIONS
aprs-nothing-heard-yet = rien d'entendu pour l'instant
aprs-heard-direct = entendu directement
aprs-two-characters-station-sent-say = Les deux caractères envoyés par la station pour dire ce qu'elle est
aprs-killed = SUPPRIMÉ
aprs-station-put-object-here-has = La station qui a placé cet objet ici l'a annulé
aprs-sender-deliberately-reported-square-rather = L'émetteur a volontairement signalé un carré plutôt qu'un point. La carte dessine le carré.
aprs-digipeaters-last-frame-came-through = Les répéteurs numériques par lesquels la dernière trame est passée. Un `*` marque celui qui l'a effectivement répétée.
aprs-message = {" "}MESSAGE{" "}
aprs-address-message-box-station = Adresser la zone de message à cette station
aprs-center = {" "}CENTRER{" "}
aprs-put-station-middle-map-double = Placer cette station au milieu de la carte. Double-cliquez sur la carte pour rendre la vue à l'ajustement automatique.
aprs-channel = CANAL
aprs-messages = MESSAGES
aprs-frames-arrived-failed-their-check = Trames arrivées qui ont échoué à leur contrôle de séquence — une collision, un évanouissement, ou un signal trop faible pour être lu. Un compteur qui monte alors que rien ne se décode signifie que le modem *entend* le canal : vérifiez l'indicateur de niveau, et ouvrez le squelch de la radio.
aprs-frames-read-cleanly-off-channel = Trames lues proprement sur le canal qui n'étaient pas de l'APRS — la session paquet de quelqu'un d'autre, ou un format que cette version ne décode pas.
aprs-every-frame-channel-arrived-other = Chaque trame du canal, telle qu'elle est arrivée — le trafic des autres inclus, et tout ce que le décodeur n'a pas pu lire.
aprs-nothing-channel-yet = rien sur le canal pour l'instant
aprs-no-messages = Aucun message.
aprs-waiting-channel = en attente du canal
aprs-sent-retried-still-waiting-acknowledged = envoyé et renvoyé, toujours en attente d'un accusé de réception
aprs-air-waiting-acknowledged = en ondes, en attente d'un accusé de réception
aprs-acknowledged-far-end = accusé de réception par l'autre bout
aprs-refused-far-end-not-retried = refusé par l'autre bout — non renvoyé
aprs-no-ack = pas d'accusé
aprs-no-answer-after-every-retry = aucune réponse après chaque nouvel essai
aprs-send = {" "}ENVOYER{" "}
aprs-station-has-no-callsign-so = Cette station n'a pas d'indicatif, donc rien ne peut être émis. Réglez-en un dans Réglages → Général, ou un propre à APRS avec son SSID dans CONFIGURATION.
aprs-needs-station-address-something-say = Il faut une station à qui s'adresser et quelque chose à dire.
aprs-send-keep-retrying-until-far = Envoyer, et continuer à réessayer jusqu'à ce que l'autre bout accuse réception. Les messages sont la seule chose, sur ce canal, à laquelle on répond.
aprs-busy = OCCUPÉ
aprs-stations-heard-still-inside-window = Stations entendues et encore dans la fenêtre réglée dans Configuration APRS, et si une autre station est sur le canal en ce moment. Rien n'émet pendant qu'il est occupé.
aprs-tx = ● TX
aprs-over-progress-frames-waiting-channel = Le trafic en cours, les trames qui attendent que le canal se libère, ou le temps jusqu'à la prochaine balise programmée.
aprs-audio-reaching-modem-flat-means = Audio parvenant au modem. Plat signifie que rien n'arrive du tout — vérifiez que la sortie données de la radio est bien celle qu'écoute sdroxide. Il s'allume en vert pendant que le modem entend une porteuse.

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = démarrage du décodeur VDL2…
vdl2-aircraft-datalink = liaison de données aéronautique
vdl2-tune-middle-vdl2-group-decoder = Accordez au milieu du groupe VDL2. La fenêtre propre au décodeur glisse à partir de là pour prendre autant des quatorze canaux que le récepteur peut atteindre.
vdl2-setup = CONFIGURATION
vdl2-which-channels-listen-how-hard = Sur quels canaux écouter, à quel point une rafale doit être forte, et quelle quantité de journal conserver
vdl2-save-vdl2-message-log-file = Enregistrer le journal des messages VDL2 dans un fichier
vdl2-vdl2-channel = Canal VDL2
vdl2-messages = MESSAGES
vdl2-address-flight-registration-label = adresse, vol, immatriculation ou étiquette
vdl2-held = FIGÉ
vdl2-hold = FIGER
vdl2-hold-message-log-where-so = Figer le journal des messages là où il est, pour qu'on puisse en lire un sans que la transmission suivante le fasse remonter à l'écran. Le décodeur continue de tourner et les compteurs continuent d'avancer ; relâcher affiche tout ce qui est arrivé entre-temps.
vdl2-nothing-decoded-yet = rien de décodé pour l'instant
vdl2-nothing-matches-filter = rien ne correspond au filtre
vdl2-stations = STATIONS
vdl2-nobody-heard-yet = personne d'entendu pour l'instant
vdl2-vdl2-setup = Configuration VDL2
vdl2-channels = Canaux
vdl2-every-25-khz-slot-136 = Chaque créneau de 25 kHz de 136,650 à 136,975 MHz. Un convertisseur de fréquence chacun, tous dans la même fenêtre de récepteur. En désactiver un économise un peu de temps processeur ; cela ne rend pas les autres plus sensibles.
vdl2-burst-threshold = Seuil de rafale
vdl2-above-each-channel-s-own = au-dessus du bruit de fond propre à chaque canal
vdl2-lower-catches-weaker-transmissions-costs = Plus bas, il capte les émissions plus faibles mais coûte du temps processeur sur le bruit ; plus haut, il les manque. Le plancher est appris par canal, et un changement ici ne le jette pas.
vdl2-keep-log = Garder dans le journal
vdl2-track-most = Suivre au maximum
vdl2-longest-silent-go-first = les plus longtemps silencieuses partent en premier
vdl2-forget-station-after = Oublier une station après
vdl2-nothing-heard = sans rien avoir entendu d'elle
vdl2-show-unread-payloads = Afficher les charges non lues
vdl2-hex = en hexadécimal
vdl2-frames-carrying-x-25-clnp = Trames transportant du X.25, du CLNP ou les applications de liaison de données au-dessus. SDRoxide les nomme et affiche les octets au lieu de les lire, et les masquer masquerait la part qu'occupe ce trafic.
vdl2-registration = Immatriculation
vdl2-flight = Vol
vdl2-label = Étiquette
vdl2-block = Bloc
vdl2-sequence = Séquence
vdl2-mode = Mode
vdl2-more-blocks-follow = d'autres blocs suivent
vdl2-check = Contrôle
vdl2-message-crc-good = CRC du message bon
vdl2-message-crc-not-checked = CRC du message non vérifié
vdl2-parity = Parité
vdl2-link-control-no-payload = contrôle de liaison, sans charge utile
vdl2-name = NOM
vdl2-addr = ADR
vdl2-type = TYPE
vdl2-msgs = MSGS

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = La tonalité reçue, et la tonalité émise — en CW, c'est la même fréquence. Cliquez sur la cascade pour la placer sur un signal.
cw-down-10-hz = Descendre de 10 Hz
cw-up-10-hz = Monter de 10 Hz
cw-qrg-main-readout-tuning-line = QRG : l'affichage principal et la ligne d'accord sont sur la fréquence travaillée. Cliquez pour le cadran à la place, avec une tonalité de contrôle en dessous — ce que montrent la plupart des radios.
cw-put-main-readout-tuning-line =
    Placer l'affichage principal et la ligne d'accord sur le signal plutôt que sur le cadran, pour que la fréquence affichée soit celle que citeraient les deux opérateurs et que la ligne d'accord se trouve au milieu de la bande passante. L'accord ne change pas ; seuls les chiffres se déplacent.

    Cliquer sur un signal ne le pose sur le curseur qu'aussi finement que le pas de clic le permet — Commandes → arrondi de l'accord au clic, 10 Hz par défaut. Un pas grossier laisse le signal décalé de la tonalité jusqu'à la moitié de celui-ci, et l'affichage le signalera.
cw-sending-speed-read-off-signal = Vitesse d'émission lue sur le signal
cw-signal-noise-500-hz-same = Rapport signal/bruit dans 500 Hz — le même chiffre que cite un rapport
cw-how-far-off-cursor-signal = De combien le signal est réellement décalé du curseur. Le décodeur le suit ; la bande passante, non — déplacez légèrement le curseur s'il s'éloigne.
cw-listening = — écoute —
cw-tx = ● TX
cw-nothing-copied-yet = — rien de copié pour l'instant —
cw-key-send-characters-you-send = Manipulez pour émettre — les caractères que vous envoyez apparaissent ici.
cw-type-line-return-sends = Tapez une ligne, Entrée l'envoie…
cw-type-here-send = Tapez ici pour envoyer…
cw-key = {" "}MANIPULATEUR ●{" "}
cw-key-2 = {" "}MANIPULATEUR{" "}
cw-hold-key-bound-cw-straight = Maintenir la touche affectée au manipulateur droit CW — Espace par défaut, et toute touche de votre choix dans Réglages → Commandes — en manipulateur droit : abaissée tant qu'elle est maintenue, relevée au relâchement, au lieu de taper du texte. La zone d'émission est verrouillée tant que ceci est actif, et tout le manipulateur est remis à la touche : le texte qui était en file est abandonné.

    Pour la manipuler à la main, réglez la manipulation CW sur « Carte son (MCW) » dans Réglages → Radio. L'appareil est alors maintenu sur une bande latérale et le sidetone du manipulateur est émis en audio, ce que pilote le manipulateur droit.
cw-watchdog = CHIEN DE GARDE
cw-straight-key-was-held-down = Le manipulateur droit a été maintenu abaissé trop longtemps — un relâchement perdu plutôt qu'une main — si bien que la porteuse a été coupée et l'émission arrêtée. Appuyez à nouveau sur la touche pour continuer.
cw-tx-2 = {"  "}TX ACTIVÉ{"  "}
cw-tx-3 = {"   "}TX{"   "}
cw-send-what-box-now-without = Envoyer maintenant ce qui est dans la zone, sans attendre Entrée
cw-hold-key-down-between-characters = Maintenir la touche abaissée entre les caractères, pour que rien de ce qui est tapé n'attende
cw-call-cq = {" "}APPEL CQ{" "}
cw-nocall = SANS APPEL
cw-clear = {" "}EFFACER{" "}
cw-stop-sending-drop-whatever-has = Arrêter l'envoi et abandonner ce qui n'est pas encore parti. Avec le manipulateur droit activé, la relecture de ce que vous avez manipulé part avec.
cw-sidetone = SIDETONE
cw-play-keyed-tone-through-computer = Jouer la tonalité manipulée par les haut-parleurs de cet ordinateur en plus de l'émettre, pour entendre ce que vous envoyez. Sur `Carte son (MCW)`, la tonalité part vers la radio et nulle part ailleurs, donc sans cela vous émettez dans le silence. À désactiver lorsque le contrôle propre à la radio ou une copie hors ondes fait déjà le travail.
cw-hold-what-typed-until-return = Garder ce qui est tapé jusqu'à Entrée, puis envoyer la ligne d'un bloc au lieu de manipuler chaque caractère au fur et à mesure qu'il est tapé. Utile sur un émetteur-récepteur qui se manipule à partir du texte, où chaque transfert vers son manipulateur est un nouveau cycle émission-réception.
cw-cw-messages = MESSAGES CW
cw-clear-rx = {" "}EFFACER RX{" "}
cw-empty-receive-window-straight-key = Vider la fenêtre de réception et la relecture du manipulateur droit. Rien de ce qui est en ondes ne s'arrête.
cw-lock = VERROUILLER
cw-decode-speed-set-here-instead = Décoder à la vitesse réglée ici au lieu de la lire sur le signal. Aide un signal trop faible pour que la recherche de vitesse se stabilise.
cw-farnsworth-send-characters-full-speed = Farnsworth : envoyer les caractères à pleine vitesse et n'étirer que les intervalles entre eux, pour qu'ils soient entendus au bon rythme mais arrivent assez lentement pour être notés.
cw-off-normal-spacing = Désactivé — espacement normal
cw-keying-speed = Vitesse de manipulation
cw-idle-0 = REPOS 0
cw-how-long-transmit-held-after = Combien de temps l'émission est maintenue après le dernier caractère, ou après le relâchement du manipulateur droit, avant que la porteuse ne tombe. Plus long couvre les pauses d'un opérateur lent ; plus court quitte la fréquence plus vite ; Désactivé la coupe dès que tout ce qui est en file est parti.
cw-off-drop-once = Désactivé — couper tout de suite

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = Descendre l'accord de 10 Hz
textmodem-tune-up-10-hz = Monter l'accord de 10 Hz
textmodem-tx = ● TX
textmodem-listening = — écoute —
textmodem-type-line-return-sends = Tapez une ligne, Entrée l'envoie…
textmodem-type-here-transmit = Tapez ici pour émettre…
textmodem-tx-2 = {"  "}TX ACTIVÉ{"  "}
textmodem-tx-3 = {"   "}TX{"   "}
textmodem-call-cq = {" "}APPEL CQ{" "}
textmodem-nocall = SANS APPEL
textmodem-clear = {" "}EFFACER{" "}
textmodem-hold-what-typed-until-return = Garder ce qui est tapé jusqu'à Entrée, puis envoyer la ligne d'un bloc au lieu de diffuser chaque caractère en ondes au fur et à mesure qu'il est tapé. Permet de relire une ligne et de la corriger avant qu'une partie n'en parte.
textmodem-messages = MESSAGES
textmodem-hell = HELL
textmodem-contrast = Contraste
textmodem-harder-softer-dots-redraws-whole = Points plus marqués ou plus doux — redessine toute la bande
textmodem-width = Largeur
textmodem-t-2row = {" "}2LIGNES{" "}
textmodem-draw-every-column-twice-stacked = Dessiner chaque colonne deux fois, l'une au-dessus de l'autre. Hell n'a pas de synchronisation verticale, donc cela garde une copie complète du texte lisible quelle que soit la phase.
textmodem-rev = {" "}INV{" "}
textmodem-reverse-video-light-dots-dark = Vidéo inversée — points clairs sur papier sombre
textmodem-clear-rx = {" "}EFFACER RX{" "}
textmodem-hold-channel-idle-sends-blank = Tenir le canal : au repos, envoie du papier vierge, si bien que la bande continue de défiler
textmodem-hold-what-typed-until-return-2 = Garder ce qui est tapé jusqu'à Entrée, puis envoyer la ligne d'un bloc au lieu de peindre chaque caractère sur la bande au fur et à mesure qu'il est tapé. Une correction tapée en direct est déjà sur le papier ; une faite avant Entrée ne l'a jamais été.
textmodem-shift = Décalage
textmodem-shift-5-hz = Décalage −5 Hz
textmodem-shift-5-hz-2 = Décalage +5 Hz
textmodem-baud = Bauds
textmodem-reverse-swap-mark-space-needed = Inversion : échange la marque et l'espace. Nécessaire lorsque le signal est reçu sur la bande latérale opposée à celle sur laquelle il a été émis, ce qui inverse les tonalités.
textmodem-track-tuning-error-automatically-instead = Suivre l'erreur d'accord automatiquement, au lieu de rester fixé sur le curseur.
textmodem-tones = Tonalités
textmodem-mode = Mode
textmodem-off-keyed = tout ou rien

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = démarrage du décodeur AIS…
ais-tune-between-two-ais-channels = Accordez entre les deux canaux AIS — 161,975 (AIS 1, maritime 87B) et 162,025 (AIS 2, 88B). Le cadran se place au milieu parce que rien n'émet là, si bien que la pointe continue d'un récepteur à zéro-FI ne tombe sur aucun des deux.
ais-setup = CONFIGURATION
ais-channels-timeouts-trail-length-how = Canaux, délais, longueur de traînée et portée des vecteurs
ais-vessels = NAVIRES
ais-nothing-heard-yet = rien d'entendu pour l'instant
ais-virtual-nothing-there = virtuel — il n'y a rien
ais-position-stale = position périmée
ais-search-rescue-transmitter-man-overboard = un émetteur de recherche et sauvetage, une balise homme à la mer ou un EPIRB AIS
ais-call-sign = indicatif
ais-station-time = heure de la station
ais-dgnss = {" "}(DGNSS)
ais-first-heard = première écoute
ais-center = CENTRER
ais-put-vessel-middle-chart = Placer ce navire au milieu de la carte
ais-last-message-station-nmea-sentences = Le dernier message de cette station, sous forme des phrases NMEA qu'un récepteur mettrait sur le fil. Collez-le dans un autre décodeur AIS pour vérifier ce que sdroxide en a fait.
ais-ais-setup = Configuration AIS
ais-channels = Canaux
ais-drop-chart-after = Retirer de la carte après
ais-without-position-report = sans rapport de position
ais-five-minutes-default-not-ads = Cinq minutes par défaut, et non les dix secondes de l'ADS-B : un navire au mouillage ne signale qu'une fois toutes les trois minutes, et une fenêtre plus courte viderait la plus grande partie d'un port entre deux rapports parfaitement valables. Au-delà, le navire quitte la carte et sa ligne grisonne — il n'est pas estompé, car un symbole pâle à une position périmée reste une affirmation sur l'endroit où se trouve un navire, à la même encre que les vrais.
ais-drop-list-after = Retirer de la liste après
ais-nothing-heard-all = sans rien avoir entendu du tout
ais-message-carrying-ship-s-name = Le message qui porte le nom d'un navire revient toutes les six minutes, donc une fenêtre de liste courte ne cesse de retirer des navires juste avant qu'ils ne disent comment ils s'appellent.
ais-trail-length = Longueur de traînée
ais-history-behind-each-target = d'historique derrière chaque cible
ais-minutes-rather-than-points-because = En minutes plutôt qu'en points, car les cadences de signalement AIS s'étendent sur deux ordres de grandeur : un nombre fixe serait quatre-vingts secondes pour un ferry et deux heures pour un pétrolier au mouillage, tracés à l'identique. Zéro désactive les traînées.
ais-speed-vector = Vecteur de vitesse
ais-how-far-ahead-vector-reaches = jusqu'où le vecteur s'étend devant
ais-slot-threshold = Seuil de créneau
ais-above-channel-s-learned-noise = au-dessus du bruit de fond appris du canal
ais-track-most = Suivre au maximum
ais-vessels-2 = {" "}navires
ais-ais-receive-only-here-always = L'AIS est en réception seule ici, et le restera : c'est un service de sauvegarde de la vie humaine, et y mettre de faux trafics de navires n'est pas couvert par une licence.
ais-fill-my-grid-digimode-setup = Renseignez Mon carré dans la configuration des modes numériques pour les distances et les relèvements, et pour que la carte se cadre autour de vous plutôt qu'autour de ce qui est le plus loin.
ais-name = NOM
ais-mmsi = MMSI
ais-type = TYPE

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = Ce récepteur ne peut pas atteindre 10489,750 MHz seul — configurez d'abord un décalage LNB/convertisseur (Réglages ▸ Radio)
qo100-run-fast-spectral-tracker-every = Lancer le suiveur spectral rapide : chaque seconde, regarder dans la fenêtre de stationnement les deux lobes symétriques de la balise et indiquer où se trouve la porteuse
qo100-telemetry = TÉLÉMÉTRIE
qo100-also-run-ao-40-uncoded = Lancer aussi le décodeur de trames non codées AO-40 — mot de synchronisation, CRC et texte de télémétrie — avec un affichage pas à pas (porteuse → synchronisation → CRC) de la progression de chaque passage
qo100-auto = AUTO
qo100-auto-correct-let-tracker-adjust = Correction automatique : laisser le suiveur ajuster seul le décalage du convertisseur/LNB — une boucle fermée lente qui, sur une estimation propre et stable, ramène la balise sur 10489,750 MHz et l'y maintient lorsque le LNB dérive. Rouvre l'étage d'entrée à chaque action.
qo100-narrower-search-smaller-slice = Plus étroit — rechercher une tranche plus petite
qo100-wider-when-beacon-isn-t = Plus large — lorsque la balise n'est pas trouvée à la largeur actuelle
qo100-clear-mark = effacer le repère
qo100-drop-hand-placed-beacon-mark = Supprimer le repère de balise placé à la main et revenir à la lecture du décodeur
qo100-lo = bas −
qo100-lo-2 = bas +
qo100-hi = haut −
qo100-hi-2 = haut +
qo100-radio-s-own-tuning-range = La plage d'accord propre à cette radio n'atteint pas 10489,750 MHz — elle exige un décalage LNB/convertisseur (Réglages ▸ Radio ▸ Convertisseur) avant que cette fenêtre puisse faire quoi que ce soit.
qo100-open-settings-radio = Ouvrir Réglages ▸ Radio
qo100-tune-10489-750-mhz = Accorder sur 10489,750 MHz
qo100-tip-if-you-can-see = astuce : si vous voyez la balise dans la bande mais qu'elle ne se verrouille pas, double-cliquez dessus — cela la marque comme étant 10489,750 MHz et permet à APPLIQUER de la corriger vers cette valeur
qo100-tracker = SUIVEUR
qo100-no-twin-lobe-shape-parking = aucune forme à deux lobes dans la fenêtre de stationnement
qo100-shape = FORME
qo100-drift = DÉRIVE
qo100-need-longer-run-estimates = — (il faut une suite d'estimations plus longue)
qo100-cycles = CYCLES
qo100-receiver = RÉCEPTEUR
qo100-target = CIBLE
qo100-measured = MESURÉ
qo100-clicked = {"  "}(cliqué)
qo100-tracker-2 = {"  "}(suiveur)
qo100-not-locked-yet = pas encore verrouillé
qo100-converter-offset = DÉCALAGE DU CONVERTISSEUR
qo100-decode = DÉCODAGE
qo100-decode-filling-first-frame-buffer = DÉCODAGE — remplissage de la première mémoire de trames…
qo100-decode-off-turn-telemetry-chip = DÉCODAGE — désactivé. Activez la puce TÉLÉMÉTRIE (rangée du haut) pour l'affichage porteuse → synchronisation → CRC.
qo100-apply-correction = {" "}APPLIQUER LA CORRECTION{" "}
qo100-write-converter-lnb-offset-puts = Écrire le décalage du convertisseur/LNB qui amène la balise, là où vous avez cliqué, sur 10489,750 MHz, et rouvrir le récepteur — la même brève interruption que fait Réglages ▸ Radio ▸ Appliquer
qo100-write-converter-lnb-offset-puts-2 = Écrire le décalage du convertisseur/LNB qui amène l'estimation du suiveur sur 10489,750 MHz, et rouvrir le récepteur — la même brève interruption que fait Réglages ▸ Radio ▸ Appliquer
qo100-waiting-second-crc-valid-frame = En attente d'une seconde trame au CRC valide avant de proposer d'écrire ceci — un seul verrouillage pourrait être une coïncidence
qo100-write-corrected-converter-lnb-offset = Écrire le décalage corrigé du convertisseur/LNB et rouvrir le récepteur — une brève interruption, la même que fait Réglages ▸ Radio ▸ Appliquer
qo100-decoder-runs-receiving-station-readout = le décodeur tourne sur la station réceptrice — son affichage n'est pas envoyé aux clients distants
qo100-starting = démarrage…
qo100-searching-first-window-fills-after = recherche — la première fenêtre se remplit après environ 24 s, puis se répète

## js8 (src/app/panels/js8.rs)

js8-multi = MULTI
js8-decode-every-js8-speed-not = Décoder chaque vitesse JS8, et non seulement celle à laquelle vous émettez. Les quatre vitesses partagent la sous-bande et sont quatre formes d'onde différentes, donc sans cela une station sur une autre vitesse est invisible — et un échange entre deux vitesses ne peut pas avoir lieu du tout. Coûte environ quatre fois le CPU de réception.
js8-hb-auto = HB AUTO
js8-turbo-does-not-beacon-local = Turbo n'émet pas de balise — c'est la vitesse locale et VHF
js8-beacon-your-callsign-grid-every = Émettre votre indicatif et votre carré toutes les 15 minutes
js8-until-next-heartbeat = Jusqu'à la prochaine pulsation
js8-tx = ● TX
js8-heard = ENTENDU
js8-nothing-heard-yet = — rien d'entendu pour l'instant —
js8-dxcc = DXCC
js8-band = BANDE
js8-grid = CARRÉ
js8-dupe = DOUBLON
js8-reply = RÉPONSE
js8-no-messages = — aucun message —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}HB{" "}
js8-clear = {" "}EFFACER VERS{" "}
js8-forget-selected-station-composer-goes = Oublier la station sélectionnée — le rédacteur revient à @ALLCALL
js8-already-addressing-allcall = Adresse déjà @ALLCALL
js8-stop = {" "}ARRÊT{" "}
js8-send = {" "}ENVOYER{" "}
js8-message = Message…

## logbook (src/app/logbook.rs)

logbook-logbook = JOURNAL
logbook-new-entry = + NOUVELLE ENTRÉE
logbook-adif = ADIF
logbook-import = IMPORTER
logbook-import-qsos-adif-adi-adif = Importer des QSO depuis un fichier ADIF (.adi ou .adif)
logbook-new-qso = NOUVEAU QSO
logbook-edit-qso = MODIFIER LE QSO
logbook-call = Indicatif
logbook-lookup = RECHERCHE
logbook-look-up-name-qth-grid = Rechercher nom / QTH / carré
logbook-grid = Carré
logbook-freq-mhz = Fréq MHz
logbook-mode = Mode
logbook-rst-sent = RST envoyé
logbook-rst-rcvd = RST reçu
logbook-name = Nom
logbook-state = Région
logbook-country = Pays
logbook-date-utc = Date UTC
logbook-time = Heure
logbook-pwr-w = Puiss W
logbook-contest = Concours
logbook-s-sent = N° envoyé
logbook-s-rcvd = N° reçu
logbook-comment = Commentaire
logbook-save = {" "}ENREGISTRER{" "}
logbook-cancel = ANNULER
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = QSL ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = Supprimer cette entrée
logbook-edit = MODIFIER
logbook-upload-qso-configured-logs = Envoyer ce QSO vers les journaux configurés
logbook-no-qsos-yet-run-ft8 = aucun QSO pour l'instant — lancez FT8/FT4 ou ajoutez une entrée manuelle

## scanner (src/app/scanner.rs)

scanner-scanner = Scanner
scanner-work-through-stored-memory-channels = Parcourir les canaux mémoire enregistrés
scanner-work-through-slice-band-channel = Parcourir une portion de bande sur une grille de canaux
scanner-stop = ARRÊT
scanner-start = DÉMARRER
scanner-from = De
scanner-step = Pas
scanner-mode = Mode
scanner-range-empty-high-edge-has = Cette plage est vide — le bord haut doit être au moins un canal au-dessus du bord bas.
scanner-skip-while-holding-pass-over = IGNORER pendant qu'il le retient, pour passer ce canal sur ce point
scanner-skipping = Ignoré
scanner-clear = EFFACER
scanner-stop-passing-over-any-them = Cesser de passer sur l'un d'eux
scanner-stop-passing-over-channel = Cesser de passer sur ce canal
scanner-no-memory-channels-stored-yet = Aucun canal mémoire enregistré — enregistrez-en dans la fenêtre MEM.
scanner-skip-channel-pass-over = IGNORER un canal pour le passer
scanner-skip = IGNORER
scanner-look-all-channels-fall-inside = Rechercher tous les canaux qui tombent dans une même fenêtre de récepteur, sur la même transformée que celle du panoramique, et n'accorder que ceux sur lesquels il y a quelque chose. Une liste sur une bande ne coûte alors qu'un accord par tour, quelle que soit sa longueur, au lieu d'un temps de stabilisation par canal. Le balayage écoute encore chaque candidat avant de s'arrêter, donc ce qui l'arrête ne change pas — mais le balayage mesure par la FFT plutôt que par le filtre du récepteur, donc vérifiez le seuil s'il commence à s'arrêter sur rien.
scanner-radio-hands-over-demodulated-audio = Cette radio fournit de l'audio démodulé et n'a aucun spectre propre à explorer, donc son balayage mémoire visite chaque canal de toute façon.
scanner-fast = RAPIDE
scanner-read-list-off-spectrum-instead = lire la liste sur le spectre au lieu de visiter chaque canal
scanner-folders = Dossiers
scanner-scan-every-folder-every-folder = Balayer chaque dossier, et chaque dossier créé à partir de maintenant. Choisissez des dossiers pour ne balayer que ceux-là.
scanner-unfiled = Non classés
scanner-nothing-filed-under-folders-you = Rien n'est classé dans les dossiers que vous avez choisis.
scanner-stops = S'arrête à
scanner-use-receiver-s-own-squelch = Utiliser le squelch propre au récepteur, pour que le balayage s'arrête exactement là où l'audio s'ouvrirait
scanner-squelch-off-scan-will-stop = le squelch est désactivé — le balayage s'arrêtera sur la première chose qu'il examine
scanner-channel-power-signal-has-reach = Puissance du canal qu'un signal doit atteindre pour arrêter le balayage
scanner-listens = Écoute pendant
scanner-how-long-stay-candidate-before = Combien de temps rester sur un candidat avant de le juger. En dessous d'environ un dixième de seconde, l'indicateur de niveau ne s'est pas stabilisé et les signaux faibles sont manqués
scanner-resumes = Reprend
scanner-carry-once-signal-drops = Continuer une fois le signal tombé
scanner-carry-after-fixed-time-regardless = Continuer après un temps fixe, quoi qu'il arrive
scanner-stay-until-you-press-next = Rester jusqu'à ce que vous appuyiez sur SUIVANT
scanner-how-long-stay = Combien de temps rester
scanner-how-long-wait-after-signal = Combien de temps attendre après la chute du signal
scanner-move-don-t-stop-channel = Passer à la suite, et ne plus s'arrêter sur ce canal
scanner-next = SUIVANT
scanner-move-now = Passer à la suite maintenant

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = démarrage du décodeur ADS-B…
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = Mode S 1090 MHz
adsb-tune-receiver-1090-000-mhz = Accordez le récepteur sur 1090,000 MHz, la liaison descendante ADS-B mondiale. Il n'y a qu'un seul canal.
adsb-setup = CONFIGURATION
adsb-timeouts-trail-length-how-far = Délais, longueur de traînée et portée des vecteurs de vitesse
adsb-aircraft = AVIONS
adsb-nothing-heard-yet = rien d'entendu pour l'instant
adsb-ground = au sol
adsb-position-stale = position périmée
adsb-gnss-altitude = altitude GNSS
adsb-ground-speed = vitesse sol
adsb-vertical-rate = taux vertical
adsb-first-heard = première écoute
adsb-center = CENTRER
adsb-put-aircraft-middle-map = Placer cet avion au milieu de la carte
adsb-last-frame-accepted-aircraft = La dernière trame acceptée de cet avion
adsb-ads-b-setup = Configuration ADS-B
adsb-drop-map-after = Retirer de la carte après
adsb-without-position-report = sans rapport de position
adsb-past-aircraft-comes-off-map = Au-delà, l'avion quitte la carte et sa ligne grisonne. Il n'est pas estompé : un carré pâle à une position périmée reste une affirmation sur l'endroit où se trouve un avion, à la même encre que les vrais.
adsb-drop-list-after = Retirer de la liste après
adsb-nothing-heard-all = sans rien avoir entendu du tout
adsb-trail-length = Longueur de traînée
adsb-points = {" "}points
adsb-history-dots-behind-each-target = points d'historique derrière chaque cible
adsb-speed-vector = Vecteur de vitesse
adsb-how-far-ahead-leader-line = jusqu'où la ligne directrice s'étend devant
adsb-one-minute-usual-radar-convention = Une minute est la convention radar habituelle : la ligne a exactement la longueur de la distance que parcourt l'avion dans ce temps, donc deux directrices égales sont deux vitesses égales à tout zoom. Zéro les désactive.
adsb-track-most = Suivre au maximum
adsb-aircraft-2 = {" "}avions
adsb-aircraft-ground-are-placed-against = Les avions au sol sont placés par rapport à la position propre de la station — un squitter de surface n'a pas de décodage non ambigu à lui seul — donc renseignez Mon carré dans la configuration des modes numériques si l'aéroport voisin n'affiche rien.
adsb-call = INDICATIF
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = en attente du récepteur…
drm-not-drm-set-mode-drm = Pas en DRM. Réglez le mode sur DRM sur une émission numérique d'ondes courtes — le cadran se place sur le centre du canal, non sur une bande latérale.
drm-no-drm-signal-locked-decoder = Aucun signal DRM verrouillé. Le décodeur a besoin de quelques secondes sur une porteuse propre : le DRM entrelace sur 400 ms ou 2 s avant que quoi que ce soit ne puisse être lu.
drm-station-s-audio-codec-not = Le codec audio de cette station ne se décode pas ici. Le xHE-AAC exige libfdk-aac sur le système — il ne peut pas être intégré, car sa licence et celle de ce programme ne sont pas compatibles. Installez-le (Debian/Ubuntu : libfdk-aac2, Arch : libfdk-aac, macOS : brew install fdk-aac, Windows : libfdk-aac-2.dll à côté de sdroxide.exe) puis redémarrez. Tout le reste de cet écran se décode normalement.
drm-quality = QUALITÉ
drm-last-minute = dernière minute
drm-waiting-locked-signal = en attente d'un signal verrouillé
drm-audio-frames-decoding-but-station = Trames audio en cours de décodage — mais le codec de cette station ne peut pas être décodé ici
drm-audio-frames-decoding = Trames audio en cours de décodage
drm-samples-reaching-decoder = Échantillons atteignant le décodeur
drm-time = HEURE
drm-symbol-timing-recovered = Cadencement des symboles retrouvé
drm-frame = TRAME
drm-transmission-frames-found = Trames de transmission trouvées
drm-fast-access-channel-what-transmission = Canal d'accès rapide — ce qu'est la transmission
drm-service-description-channel-what-services = Canal de description des services — ce que sont les services
drm-audio = AUDIO
drm-constellation = CONSTELLATION
drm-waiting-decoded-symbols = en attente de symboles décodés
drm-sampled-across-frame = échantillonné sur toute la trame
drm-whole-frame = trame entière
drm-mode = MODE
drm-robustness-mode-channel-width-ground = Mode de robustesse et largeur de canal. A convient à un trajet par onde de sol et porte le plus ; D convient à un trajet par onde ionosphérique fortement diffusé et porte le moins.
drm-interleave = ENTRELACEMENT
drm-how-far-transmission-spreads-each = Jusqu'où la transmission étale chaque trame dans le temps. Long supporte des évanouissements plus profonds et met d'autant plus de temps à s'accrocher.
drm-protection = PROTECTION
drm-offset = DÉCALAGE
drm-residual-sample-clock-error-against = Erreur résiduelle de l'horloge d'échantillonnage par rapport à l'émetteur. Grande et stable signifie que la référence du récepteur est décalée, non la diffusion.
drm-doppler = DOPPLER
drm-delay = RETARD
drm-doppler-delay-spread-path-how = Étalement Doppler et temporel du trajet — à quelle vitesse il bouge et à quel point ses échos arrivent espacés.
drm-service = SERVICE
drm-decode-service-multiplex = Décoder ce service du multiplex

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = démarrage de la station AtCHAT NET…
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET — clavier multistation et fichiers
atchat-net = {" "}SUR LE NET{" "}
atchat-net-click-leave-state-kept = Sur le net — cliquez pour le quitter (l'état est conservé ; cliquez à nouveau pour revenir)
atchat-rejoin = {" "}REJOINDRE{" "}
atchat-off-net-click-rejoin = Hors du net — cliquez pour revenir
atchat-off-net = hors net
atchat-carrier = ● PORTEUSE
atchat-another-station-transmitting-channel-busy = Une autre station émet — le canal est occupé.
atchat-tx = ● TX
atchat-station-callsign-virtual-channel-address = Indicatif de la station et adresse du canal virtuel
atchat-virtual-channel = CANAL VIRTUEL
atchat-work-net-over-tcp-endpoint = Travailler le net par un point de terminaison TCP au lieu de la radio — un bouclage compatible channel_server pour le développement sans radio. Désactivé met la station en ondes.
atchat-roster = LISTE
atchat-nobody-heard-yet = personne d'entendu pour l'instant
atchat-chat = {" "}DISCUSSION{" "}
atchat-common-channel-lines-here-go = Le canal commun — les lignes ici vont à tout le monde (ALL)
atchat-log = {" "}JOURNAL{" "}
atchat-station-s-own-air-activity = L'activité en ondes propre à la station — élection du maître, vieillissement de la liste, nouvelles tentatives ARQ
atchat-no-activity-logged-yet = Aucune activité journalisée pour l'instant.
atchat-no-messages-yet = Aucun message pour l'instant.
atchat-message-everyone-all = message à tout le monde (ALL)
atchat-send = {" "}ENVOYER{" "}
atchat-transfers = TRANSFERTS
atchat-all = → ALL
atchat-sent-file-follows-active-chat = Un fichier envoyé suit l'onglet de discussion actif
atchat-send-file = {" "}ENVOYER UN FICHIER{" "}
atchat-send-file-image-over-air = Envoyer un fichier ou une image en ondes, en bloc CRC-ARQ
atchat-nothing-flight = rien en transit
atchat-images = IMAGES
atchat-no-images-received = Aucune image reçue.

## rds (src/app/rds.rs)

rds-station = STATION
rds-diagnostics = DIAGNOSTICS
rds-which-programme-type-table-read = Quelle table de types de programme utiliser pour lire cette station. Le RDS est utilisé partout hors d'Amérique du Nord ; le RBDS ajoute l'indicatif qu'épelle le code d'identité d'une station américaine. Auto suit le code pays étendu envoyé par la station, et tant qu'il n'en arrive aucun il n'épelle pas d'indicatif — les plages d'identité qu'il lirait sont partagées avec les codes RDS de neuf pays, donc choisissez RBDS pour nommer une station qui n'envoie pas de code pays.
rds-waiting-receiver = en attente du récepteur…
rds-no-rds-station-not-every = Pas de RDS sur cette station. Tous les émetteurs ne le transportent pas, et un émetteur faible peut le transporter sans qu'il arrive.
rds-rds-only-carried-wfm-broadcast = Le RDS n'est transporté que par les stations de radiodiffusion WFM.
rds-holding = EN ATTENTE
rds-block-sync-has-dropped-last = La synchronisation des blocs s'est perdue — c'est la dernière station lue.
rds-identity = IDENTITÉ
rds-programme = PROGRAMME
rds-described = DÉCRIT COMME
rds-content = CONTENU
rds-music = Musique
rds-speech = Parole
rds-traffic = TRAFIC
rds-announcement-air-now = annonce en ondes maintenant
rds-carried-station = transporté par cette station
rds-not-carried = non transporté
rds-radio-text = TEXTE RADIO
rds-station-clock = HORLOGE DE LA STATION
rds-country-code = CODE PAYS
rds-also = ÉGALEMENT SUR
rds-also-tagged = ÉGALEMENT ÉTIQUETÉ
rds-sync = SYNC
rds-no-sync = PAS DE SYNC
rds-blocks-failed-their-check-plus = Blocs qui ont échoué à leur contrôle, plus ceux que le correcteur d'erreurs a dû réparer. Les corrections comptent comme des erreurs à dessein : un chiffre qui monte est le premier signe qu'une station va cesser d'être décodée.
rds-front-end-overloading = ÉTAGE D'ENTRÉE SATURÉ
rds-converter-clipping-which-puts-distortion = Le convertisseur écrête, ce qui met de la distorsion sur tout le multiplex — y compris sur la sous-porteuse de données à 57 kHz. Baissez le gain RF, ou branchez un atténuateur : le RDS se dégrade d'environ un décibel avant que quoi que ce soit ne soit audible, donc une station qui sonne parfaitement peut être bien trop forte pour que ses données survivent.
rds-group-types = TYPES DE GROUPES
rds-recent-groups-newest-last = GROUPES RÉCENTS — le plus récent en dernier
rds-nothing-decoded-yet = rien de décodé pour l'instant

## winlink (src/app/winlink.rs)

winlink-mail = COURRIER
winlink-abort = ABANDONNER
winlink-stop-session-progress = Arrêter la session en cours
winlink-connect = CONNEXION
winlink-compose = RÉDIGER
winlink-connecting = connexion…
winlink-not-connected-yet = pas encore connecté
winlink-loading = chargement…
winlink-date = DATE
winlink-from = DE
winlink-subject = OBJET
winlink-no-subject = (sans objet)
winlink-delete-message = Supprimer ce message
winlink-arch = ARCH
winlink-move-archive = Déplacer vers les archives
winlink-select-message = sélectionnez un message
winlink-reply = RÉPONDRE
winlink-delete = SUPPRIMER
winlink-to = À
winlink-cc = Cc
winlink-subject-2 = Objet
winlink-callsign-smtp-someone-example-org = Un indicatif, ou SMTP:quelquun@example.org pour le courrier Internet. Séparez-en plusieurs par des virgules.
winlink-file-outbox = CLASSER DANS LA BOÎTE D'ENVOI
winlink-discard = ABANDONNER
winlink-filed-messages-go-out-next = les messages classés partent à la session suivante
winlink-mail-log = JOURNAL DU COURRIER
winlink-session-running = session en cours…
winlink-no-session-yet = aucune session pour l'instant

## wspr (src/app/panels/wspr.rs)

wspr-receptions = RÉCEPTIONS
wspr-save-wspr-reception-list-csv = Enregistrer la liste des réceptions WSPR en CSV
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = balise de propagation en signaux faibles
wspr-time-left-two-minute-slot = Temps restant dans ce créneau de deux minutes. À sa fin, l'enregistrement part vers le décodeur et la prochaine décision d'émission tombe.
wspr-tx = ● TX
wspr-decoding = décodage…
wspr-band = Bande
wspr-last-slot = Dernier créneau
wspr-nothing-heard = rien d'entendu
wspr-receive-only = réception seule
wspr-next-slot = Prochain créneau
wspr-waiting-radio-take-change = en attente que la radio prenne le changement…
wspr-hop-next = Bande suivante
wspr-transmit = ÉMETTRE
wspr-receive-only-2 = Réception seule.
wspr-power = PUISSANCE
wspr-what-you-actually-radiate = ce que vous rayonnez réellement
wspr-goes-out-message-everyone-who = Cela part dans le message, et chacun de ceux qui vous entendent juge le trajet d'après lui — donc un chiffre optimiste ici fausse aussi leurs mesures, et pas seulement les vôtres.
wspr-roam = ROAM
wspr-pick-different-offset-inside-200 = Choisir un décalage différent dans la fenêtre de 200 Hz pour chaque émission. C'est la convention : deux cents hertz partagés par tous ne fonctionnent que si personne ne se pose au milieu. Désactivé, on reste là où est le curseur.
wspr-band-hop = SAUT DE BANDE
wspr-move-dial-band-band-between = Déplacer le cadran de bande en bande entre les créneaux, pour qu'un seul récepteur échantillonne tout le spectre au lieu d'une seule tranche. Tourner vous-même le VFO la met en pause — appuyez deux fois sur ceci pour la reprendre.
wspr-upload = ENVOYER
wspr-send-what-station-decodes-wsprnet = Envoyer ce que décode cette station à wsprnet.org. Transmettre ce que vous entendez est ce qui fait d'un récepteur WSPR un élément du réseau ; cela ne met rien en ondes.
wspr-who-heard-me = QUI M'A ENTENDU
wspr-ask-wsprnet-org-every-few = Demander à wsprnet.org toutes les quelques minutes quelles stations ont décodé celle-ci. WSPR n'a aucun accusé de réception, donc c'est le seul moyen pour une balise d'apprendre quoi que ce soit sur sa portée.
wspr-bands = BANDES
wspr-none-selected-hopping-has-nowhere = aucune sélectionnée — le saut de bande n'a nulle part où aller
wspr-set-your-callsign-grid-general = Réglez votre indicatif et votre carré dans l'onglet Général des Réglages avant d'émettre.

## packet (src/app/panels/packet.rs)

packet-monitor = MONITEUR
packet-busy = OCCUPÉ
packet-frames-arrived-but-failed-their = Trames arrivées mais ayant échoué à leur contrôle de séquence — une collision, un évanouissement, ou un signal trop faible pour être lu.
packet-station-callsign-speed-tx-delay = Indicatif de la station, vitesse, délai d'émission, chemin de répéteurs, balise et serveur KISS
packet-starting-packet-modem = démarrage du modem paquet…
packet-nothing-heard-yet = rien d'entendu pour l'instant
packet-terminal = TERMINAL
packet-connected = {" "}CONNECTÉ{" "}
packet-calling = {" "}APPEL EN COURS{" "}
packet-idle = {" "}INACTIF{" "}
packet-extended-sequence-numbers-window-up = Numéros de séquence étendus — une fenêtre allant jusqu'à 127 trames.
packet-frames-sent-not-yet-acknowledged = Trames envoyées et pas encore acquittées par l'autre bout.
packet-retries-against-link-s-limit = Nouvelles tentatives par rapport à la limite de cette liaison. Un compteur qui monte pendant que les trames non acquittées restent en place est ce à quoi ressemble un trajet en évanouissement vu de ce côté.
packet-mail-window-using-link = La fenêtre COURRIER utilise la liaison.
packet-one-radio-one-channel-one = Une radio, un canal, une liaison. Une session Winlink et ce terminal sont deux façons de l'utiliser, et celui qui la demande en premier l'obtient.
packet-digipeaters-nearest-first-separated-commas = Répéteurs numériques, le plus proche en premier, séparés par des virgules — OE3XLR-1,OE3XMS-1. Laissez-le vide pour une station que vous entendez directement, ou pour utiliser le chemin par défaut des réglages paquet.
packet-disconnect = {" "}DÉCONNEXION{" "}
packet-hang-up-properly-wait-far = Raccrocher proprement, et attendre que l'autre bout soit d'accord. Changer de mode met fin à la session sans en prévenir personne.
packet-connect = {" "}CONNEXION{" "}
packet-mail-window-has-link-finish = La fenêtre COURRIER a la liaison. Terminez ou arrêtez d'abord cette session.
packet-station-has-no-callsign-yet = Cette station n'a pas encore d'indicatif — réglez-en un dans CONFIGURATION, au-dessus du volet moniteur. Rien n'émet tant qu'il n'est pas réglé.
packet-call-station-connected-mode-node = Appeler cette station en mode connecté — un nœud, un BBS ou un autre opérateur.
packet-needs-callsign-call = Il faut un indicatif à appeler.
packet-clear = {" "}EFFACER{" "}
packet-empty-transcript-link-untouched-connected = Vider la transcription. La liaison n'est pas touchée — une station connectée reste connectée.
packet-not-connected-type-callsign-above = Non connecté. Tapez un indicatif ci-dessus et appuyez sur CONNEXION pour appeler un nœud ou un BBS.
packet-type-here = tapez ici
packet-not-connected = non connecté
packet-send = {" "}ENVOYER{" "}
packet-send-line-node-bbs-reads = Envoyer la ligne. Un nœud ou un BBS lit une ligne à la fois.

## fsq (src/app/panels/fsq.rs)

fsq-speed = Vitesse
fsq-call = Indicatif
fsq-callsign-directed-fsqcall-messages-defaults = Indicatif pour les messages dirigés (FSQCALL) ; par défaut votre indicatif
fsq-tx = ● TX
fsq-contacts = CONTACTS
fsq-heard = ENTENDUS
fsq-none = — aucun —
fsq-allcall = ALLCALL
fsq-image = IMAGE
fsq-clear = EFFACER
fsq-forget-pictures-received-so-far = Oublier les images reçues jusqu'ici
fsq-send-image = Envoyer une image…
fsq-right-click-remove = Clic droit pour retirer
fsq-remove-picture = Retirer cette image
fsq-listening = — écoute —
fsq-message = Message…
fsq-send = {" "}ENVOYER{" "}
fsq-call-cq = {" "}APPEL CQ{" "}
fsq-heard-2 = {" "}? entendu{" "}
fsq-clear-2 = {" "}EFFACER{" "}
fsq-fsq-contacts = Contacts FSQ
fsq-add = Ajouter

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + ONGLET
publicsdr-open-receiver-new-radio-own = Ouvrir ce récepteur comme une nouvelle radio, dans son propre onglet
publicsdr-point-radio-receiver-replacing-whatever = Diriger *cette* radio vers le récepteur, en remplaçant l'interface sur laquelle elle se trouve actuellement — demandé à nouveau d'abord, car c'est une configuration complète de radio
publicsdr-not-stated = non indiqué
publicsdr-replace = REMPLACER
publicsdr-point-radio-you-are-receiver = Diriger la radio sur laquelle vous êtes vers ce récepteur
publicsdr-tab-instead = + ONGLET À LA PLACE
publicsdr-open-receiver-another-radio-leave = Ouvrir le récepteur comme une autre radio et laisser celle-ci tranquille
publicsdr-cancel = ANNULER
publicsdr-public-sdrs = SDR PUBLICS
publicsdr-available = DISPONIBLES
publicsdr-hide-receivers-are-full-ones = Masquer les récepteurs complets, et ceux dont l'opérateur n'a ouvert aucun canal à des applications autres qu'un navigateur
publicsdr-band = DANS LA BANDE
publicsdr-only-receivers-cover-current-dial = Seulement les récepteurs qui couvrent la fréquence actuelle du cadran
publicsdr-low-bw = FAIBLE DÉBIT
publicsdr-take-spyserver-low-bandwidth-shape = Prendre un SpyServer dans sa forme à faible débit : une fenêtre I/Q étroite qui suit le cadran, plus la vue de bande propre au serveur, au lieu de mégabits d'I/Q large bande. Sans effet sur un KiwiSDR, qui n'a que cette forme.
publicsdr-refresh = ⟳ ACTUALISER
publicsdr-fetch-both-directories-again = Récupérer à nouveau les deux annuaires
publicsdr-name-place-antenna-band = nom, lieu, antenne, bande
publicsdr-clear-search = Effacer la recherche
publicsdr-fetching-receiver-lists = Récupération des listes de récepteurs…
publicsdr-machine-radio-attached-does-not = La machine à laquelle cette radio est reliée ne répond pas aux questions sur les périphériques, donc elle ne peut pas récupérer les listes non plus.
publicsdr-no-match = aucune correspondance
publicsdr-nothing-matches-try-turning-filter = rien ne correspond — essayez de réactiver une puce de filtre
publicsdr-radio = cette radio
publicsdr-radio-s-configuration-has-not = La configuration de cette radio n'est pas encore arrivée depuis la machine où elle se trouve, donc il n'y a rien à diriger vers le récepteur — prenez-le avec + ONGLET à la place.
publicsdr-position-not-published-so-nothing = {" "}Sa position n'est pas publiée, donc rien de ce qu'elle entend ne sera transmis.

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = en attente du récepteur…
hd-not-hd-radio-set-mode = Pas en HD Radio. Réglez le mode sur HD Radio sur une diffusion FM portant les porteuses numériques associées — le cadran se place sur le centre de la porteuse analogique, non à côté.
hd-no-hd-radio-lock-digital = Aucun verrouillage HD Radio. Les bandes latérales numériques sont émises environ 20 dB sous la porteuse analogique, donc une station audible peut encore être trop bruitée pour être décodée : le décodeur a besoin de quelques secondes sur une station propre.
hd-sync = SYNC
hd-ofdm-frame-timing-recovered = Cadencement des trames OFDM retrouvé
hd-audio = AUDIO
hd-audio-frames-decoding = Trames audio en cours de décodage
hd-quality = QUALITÉ
hd-lower = INFÉRIEURE
hd-upper = SUPÉRIEURE
hd-last-minute = dernière minute
hd-waiting-locked-signal = en attente d'un signal verrouillé
hd-mer-l = MER L
hd-modulation-error-ratio-lower-digital = Rapport d'erreur de modulation de la bande latérale numérique inférieure.
hd-mer-u = MER U
hd-modulation-error-ratio-upper-digital = Rapport d'erreur de modulation de la bande latérale numérique supérieure.
hd-cber = CBER
hd-channel-bit-error-ratio-after = Taux d'erreur binaire du canal après le code interne. En dessous d'environ 1e-3, le code externe peut le corriger ; au-delà, l'audio commence à se hacher.
hd-offset = DÉCALAGE
hd-residual-carrier-frequency-offset-large = Décalage résiduel de la fréquence de la porteuse. Grand et stable signifie que la référence du récepteur est décalée, non la diffusion.
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = Indicateur de mode de service principal : lequel des modes de service hybrides FM la station émet.
hd-programme = PROGRAMME
hd-restricted-access = {" "}— accès restreint

## bands (src/app/bands.rs)

bands-bands = BANDES
bands-daylight = ☀ JOUR
bands-night = ☾ NUIT
bands-no-forecast-yet-fetching-hamqsl = · pas encore de prévision — récupération depuis HAMQSL.com
bands-band = BANDE

bands-condx = CONDX
bands-paths = TRAJETS
bands-reach = PORTÉE
bands-best = MEILLEUR
bands-ibp-beacons = BALISES IBP
bands-meteor-showers = PLUIES DE MÉTÉORES
bands-peak = CRÊTE

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = SPOTS
spots-longwave-shortwave-broadcast-stations-air = Stations de radiodiffusion en ondes longues et courtes actuellement en ondes
spots-view = DANS LA VUE
spots-only-spots-inside-panadapter-span = Seulement les spots dans la largeur du panoramique
spots-setup = ⚙ CONFIGURATION
spots-feeds-lookup-upload-settings = Flux, recherche et réglages d'envoi
spots-call-station-site-frequency = indicatif, station, site, fréquence
spots-clear-search = Effacer la recherche
spots-no-match = aucune correspondance
spots-no-spots-enable-feed-setup = aucun spot — activez un flux dans ⚙ CONFIGURATION
spots-nothing-matches-search = rien ne correspond à la recherche

## ism (src/app/ism.rs)

ism-ism-devices = APPAREILS ISM
ism-decoding = DÉCODAGE
ism-needs-wideband-iq-source-cat = Exige une source IQ large bande ; un appareil CAT n'envoie que de l'audio
ism-how-far-burst-must-stand = De combien une rafale doit dépasser le bruit de fond propre au canal avant d'être décodée
ism-give-each-band-width-normally = Donnez à chaque bande la largeur qu'elle exige en général — un quart de mégahertz pour les bandes OOK à 315, 345 et 433 MHz, un mégahertz entier pour 868 et 915.
ism-several-hundred-more-device-decoders = Plusieurs centaines d'autres décodeurs d'appareils, issus du projet rtl_433, intégrés. Lit les appareils OOK à 433 MHz que les décodeurs propres à sdroxide n'atteignent pas du tout. Lorsque les deux peuvent lire un appareil, rtl_433 connaît bien plus de variantes et prend le relais.
ism-t-1-your-own = 1 à vous
ism-device-decoders-registered-how-many = Décodeurs d'appareils enregistrés, combien proviennent de votre rtl433_flex.conf, et combien de transmissions ont été décodées depuis le démarrage.
ism-reload-decoders = RECHARGER LES DÉCODEURS
ism-re-read-rtl433-flex-conf = Relire rtl433_flex.conf après l'avoir modifié. Les appareils déjà entendus restent dans la liste.
ism-rtl-433-knows-more-variants = rtl_433 connaît davantage de variantes de ceux-ci que les décodeurs propres à sdroxide, donc il les lit pendant l'écoute ici. Sélectionnez une autre bande et les décodeurs propres à sdroxide reprennent la main.
ism-decoder-your-rtl433-flex-conf = Un décodeur de votre rtl433_flex.conf a été refusé. Les autres ont été chargés. Corrigez la ligne, puis appuyez sur RECHARGER LES DÉCODEURS.
ism-waiting-decoder = en attente du décodeur…
ism-where-decoder-actually-listening-follows = L'endroit où le décodeur écoute réellement. Ceci suit le centre matériel du récepteur, non le VFO — sur un étage d'entrée large, les deux peuvent être très éloignés, et c'est la fenêtre qui décide des canaux qui sont atteignables.
ism-transmissions-gate-opened-how-many = Les transmissions sur lesquelles la porte s'est ouverte, et combien ont produit une trame valide. Beaucoup de rafales et aucun décodage signifie que la bande est occupée par des appareils que sdroxide ne sait pas encore lire. Aucune du tout signifie que rien n'atteint le seuil — baissez-le, ou vérifiez que le cadran est sur la bande.
ism-nothing-heard-yet = rien d'entendu pour l'instant
ism-click-tune-receiver =


    Cliquez pour accorder le récepteur dessus

## app (src/app/mod.rs)

app-hpsdr = HPSDR
app-rtl-sdr = RTL-SDR
app-rtl-sdr-rtl-tcp = RTL-SDR (rtl_tcp)
app-spyserver-vfo = SpyServer (VFO)
app-airspy-hf = Airspy HF+
app-sdrplay = SDRplay
app-elad = ELAD
app-airspy = Airspy
app-sxceiver = SXceiver
app-usrp = USRP

## navtex (src/app/panels/navtex.rs)

navtex-messages = MESSAGES
navtex-sync = SYNCHRO
navtex-character-phase-locked-signal-being = La phase des caractères est verrouillée — un signal est en cours de lecture.
navtex-characters-whose-first-copy-was = Caractères dont la première copie était corrompue et ont été pris sur la répétition cinq créneaux plus tard, et ceux où les deux étaient mauvaises. Un caractère NAVTEX est envoyé deux fois ; c'est toute sa correction d'erreurs.
navtex-swap-mark-space-tones-signal = Inverser les tonalités de marque et d'espace, pour un signal reçu sur l'autre bande latérale. Désactivé, c'est la bande latérale supérieure sur la fréquence du canal, ce que disent toutes les consignes de réglage publiées pour le service.
navtex-starting-navtex-receiver = démarrage du récepteur NAVTEX…
navtex-nothing-yet-station-transmits-ten = rien pour l'instant — une station émet pendant dix minutes toutes les quatre heures
navtex-cut-short = interrompu
navtex-message = MESSAGE
navtex-time-day-stated-message-utc = Heure du jour indiquée dans le message (UTC) : affichage seulement
navtex-receiving = RÉCEPTION
navtex-monitor = MONITEUR
navtex-everything-decoded-message-not = tout ce qui est décodé, message ou non

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = PEINTURE RF
rfpaint-scan-speed = Vitesse de balayage
rfpaint-how-fast-text-image-scanned = La vitesse à laquelle le texte ou l'image est balayé sur la cascade. Plus bas est plus lent et plus lisible ; 100 % = débit de base, 25 % (centre) est la valeur par défaut.
rfpaint-abort = Abandonner
rfpaint-text-paint = PEINTURE DE TEXTE
rfpaint-type-text-paint = Saisissez le texte à peindre…
rfpaint-preview-waterfall = APERÇU DE LA CASCADE
rfpaint-type-text-preview = saisissez du texte pour l'aperçu
rfpaint-transmit = {"  "}ÉMETTRE{"  "}
rfpaint-image-paint = PEINTURE D'IMAGE
rfpaint-no-image-loaded = aucune image chargée
rfpaint-load-image = Charger une image…
rfpaint-load-image-preview = chargez une image pour l'aperçu

## hfdl (src/app/hfdl.rs)

hfdl-listen = ÉCOUTER
hfdl-switches-engine-s-hfdl-downconverter = Active ou désactive le convertisseur de fréquence et le décodeur HFDL du moteur. Le décodage coûte une voie de 24 kHz et un fil d'exécution, que la fenêtre soit ouverte ou non.
hfdl-channel-assigned-frequency-listen-anything = Le canal (la fréquence attribuée) à écouter. Tout ce qui se trouve dans la bande fonctionne — les stations HFDL émettent de 2,8 à 22 MHz — face à une voie fixe de 24 kHz centrée ici.
hfdl-save-hfdl-decode-log-file = Enregistrer le journal de décodage HFDL dans un fichier
hfdl-riverhead-northern-atlantic-north-america = Riverhead (Atlantique Nord / Amérique du Nord) — la valeur par défaut, et le canal sur lequel le décodeur a été validé.
hfdl-hfdl-assigned-frequency = Une fréquence attribuée HFDL.
hfdl-one-your-saved-channels-click = L'un de vos canaux enregistrés — cliquez pour accorder, clic droit pour supprimer.
hfdl-hfdl-selected-but-decoding-off = HFDL est sélectionné mais le décodage est désactivé — appuyez sur ÉCOUTER ci-dessus
hfdl-decodes = DÉCODAGES
hfdl-nothing-decoded-yet-ground-station = Rien de décodé pour l'instant — la balise d'une station au sol se répète toutes les ~32 s dès qu'une est dans le canal.
hfdl-no-decode-matches-filter = Aucun décodage ne correspond au filtre.
hfdl-decoding = DÉCODAGE
hfdl-decoding-off = DÉCODAGE DÉSACTIVÉ
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = Voix numérique FreeDV V1
rade-tx = ● TX
rade-mute-analog = COUPER L'ANALOGIQUE
rade-mute-demodulated-audio-so-only = Couper l'audio démodulé, pour n'entendre que la parole décodée. Sinon le signal brut passe chaque fois que le modem n'a rien à jouer — ce souffle est ce qui permet de repérer un trafic avant sa synchronisation, donc laissez ceci désactivé pendant l'accord.
rade-sync = SYNCHRO
rade-how-far-received-signal-sits = De combien le signal reçu se trouve loin de l'endroit où le modem l'attend. Les grandes valeurs se décodent encore — la boucle d'acquisition les suit — mais rapprocher le cadran pour ramener ceci près de zéro donne la meilleure marge.
rade-snr = RSB —
rade-callsign-station-put-end-over =
    L'indicatif que la station a mis dans sa trame de fin de trafic — le seul point d'un trafic RADE où il est envoyé. C'est aussi ce qui est signalé à FreeDV Reporter, si bien qu'une station que vous entendez apparaît sur qso.freedv.org comme entendue par vous.

    Conservé une minute après la disparition du signal, et abandonné lorsque vous commencez un trafic à vous : il nomme la station sur la fréquence, non toutes les stations que le récepteur a jamais entendues.
rade-stop-talking = ARRÊTER DE PARLER
rade-talk-ptt = PARLER (PTT)
rade-open-close-rade-over-modem = Ouvrir ou fermer un trafic RADE. Le modem a besoin d'environ 120 ms de parole avant que la première trame ne parte, et envoie une trame de fin de trafic lorsque vous arrêtez, si bien que l'émission se prolonge un peu après le bouton.
rade-rade-v1-occupies-roughly-1060 = RADE V1 occupe environ 1060–1880 Hz de la bande passante — bande latérale supérieure, ou inférieure sur 160, 80 et 40 m, où RADE garde la pratique de la phonie et où sdroxide bascule pour vous. Placez le signal dans la bande ombrée de la cascade ; le modem le trouve à partir de là.

## frame (src/app/frame.rs)

frame-acknowledge = Accuser réception
frame-dismiss = Fermer
frame-reconnecting = Reconnexion…
frame-not-connected = Non connecté.
frame-reconnect-now = Reconnecter maintenant
frame-transmit-lockout-disabled = ⚠ VERROUILLAGE D'ÉMISSION DÉSACTIVÉ
frame-engine-was-started-oob-tx = Ce moteur a été lancé avec --oob-tx. Le verrouillage sur les bandes amateur est désactivé : il manipulera l'émetteur sur toute fréquence que le matériel supporte.
frame-transmitting-outside-your-licence-offence = Émettre hors de votre licence est une infraction dans chaque pays qui en délivre une. Ne continuez que si vous êtes autorisé à utiliser les fréquences sur lesquelles vous êtes sur le point de manipuler — une licence MARS/CAP ou commerciale, un permis expérimental, ou une charge fictive.
frame-i-understand = {"  "}J'AI COMPRIS{"  "}
frame-restart-without-oob-tx-put = Redémarrez sans --oob-tx pour rétablir le verrouillage.

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC — la force avec laquelle l'émetteur est piloté, en pourcentage de la puissance maximale qu'il peut sortir. 100 % est la pleine échelle, non la pleine puissance en watts : ce que cela vaut à l'antenne dépend de la radio et de l'Excitation.
smeter-radio-reports-no-forward-power =


    Cette radio n'annonce aucune puissance directe, donc le chiffre à côté de l'ALC est de nouveau la lecture de l'ALC plutôt que des watts.
smeter-no-swr-radio-has-no =


    Pas de ROS : cette radio n'a pas de pont pour le mesurer, et sdroxide n'en inventera pas un — il ne peut afficher que ce que la radio annonce elle-même. Un appareil doté d'un ROS-mètre (par CAT, TCI ou une liaison réseau) et une carte HPSDR le renseignent tous deux ; un simple émetteur SDR ne le peut pas.
smeter-no-signal-report-yet-receiver = Pas encore de rapport de signal — le récepteur n'a pas fourni de lecture du S-mètre.
smeter-s9-73-dbm-each-s =


    S9 correspond à −73 dBm, et chaque unité S en dessous vaut 6 dB. La valeur en dBm n'est aussi honnête que l'étalonnage de l'étage d'entrée — un récepteur non étalonné affiche des dBFS avec une étiquette dBm.
smeter-ovld-converter-into-rails-so =


    OVLD : le convertisseur est en butée, donc le niveau affiché sous-estime le signal. Ramenez le gain de l'étage d'entrée.
smeter-table-held = , tableau maintenu
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● TX
multi-link = Power
multi-mute-radio-s-audio = Couper l'audio de cette radio
multi-close-radio-s-split-view = Fermer la vue scindée de cette radio
multi-open-radio-split-view-own = Ouvrir cette radio dans une vue scindée à part
multi-already-open-another-split-view = Déjà ouverte dans une autre vue scindée
multi-show-radio-here = Afficher cette radio ici
multi-add-radio-computer-settings-radio = Ajouter une radio sur cet ordinateur (Réglages → Radio pour en ajouter une sur une station)
multi-add-radio = Ajouter une radio
multi-asked-station-another-radio = Demande à la station une autre radio…
multi-client-cannot-open-connection = Ce client ne peut pas ouvrir de connexion.

## net (src/app/net.rs)

net-fail = ÉCHEC
net-downloading = Téléchargement…
net-adif-import-failed-file-could = Échec de l'import ADIF : le fichier n'a pas pu être lu
net-channel-import-failed-file-could = Échec de l'import des canaux : le fichier n'a pas pu être lu
net-channel-import-nothing-read-chirp = Import des canaux : rien à lire — un fichier CSV CHIRP commence par une ligne d'en-tête qui nomme ses colonnes, dont l'une doit être Frequency

## widgets (src/app/panels/widgets.rs)

widgets-free-text = texte libre
widgets-hashed-callsign-not-yet-heard = indicatif haché, pas encore entendu en entier
widgets-entity-unknown = entité inconnue
widgets-band = cette bande
widgets-new-entity-never-worked-any = Nouvelle entité — jamais travaillée, sur aucune bande
widgets-new-grid-square = Nouveau carré
widgets-not-worked-before = Non travaillé auparavant
widgets-worked-before-but-not-band = Travaillé auparavant, mais pas sur cette bande
widgets-call-queue = Dans la file d'appels
widgets-image = Image

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = RÉCEPTIONS
pi4-save-pi4-reception-list-csv = Enregistrer la liste des réceptions PI4 en CSV
pi4-next-generation-beacon = Balise de nouvelle génération
pi4-time-left-one-minute-beacon = Temps restant dans ce cycle de balise d'une minute. Le message PI4 est recherché une fois que la fenêtre a eu le temps de se remplir au-delà de l'endroit où il se terminerait.
pi4-decoding = décodage…
pi4-band = Bande
pi4-last-cycle = Dernier cycle
pi4-nothing-heard = rien d'entendu
pi4-receive-only-tune-so-beacon = Réception seule. Accordez de sorte que l'identification CW de la balise tombe à 800 Hz audio — la convention propre au réseau — et que les quatre tonalités PI4 tombent là où ce décodeur les cherche par défaut.

## awards (src/app/awards.rs)

awards-awards = DISTINCTIONS
awards-band = Bande
awards-all = Toutes
awards-dxcc = DXCC
awards-grids = Carrés
awards-worked-all-states = Worked All States
awards-cq-zones-waz = Zones CQ (WAZ)
awards-dxcc-entities = Entités DXCC

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = Audio dans la bande passante du décodeur.
acars-blocks-framed-blocks-whose-check = Blocs cadrés, et blocs dont la séquence de contrôle a échoué.
acars-starting-acars-receiver = démarrage du récepteur ACARS…
acars-channel = CANAL
acars-tune-dial-here = Accordez le cadran ici.

acars-channel-out-of-range = Hors de la plage de réception de cette radio.
acars-nothing-yet-acars-quiet-between = Rien pour l'instant. ACARS est silencieux entre les avions — laissez-le sur un canal et attendez une rafale.
acars-check-failed = contrôle échoué
acars-block-check-sequence-did-not = La séquence de contrôle du bloc ne correspond pas — le texte peut être faux.
acars-no-text = (aucun texte)

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = Identifiant
login-password = Mot de passe
login-remember-device = Mémoriser sur cet appareil
login-signs-without-asking-next-time = Connecte sans demander la prochaine fois, et laisse l'onglet de la vue 3D s'ouvrir sans rien demander. Le mot de passe est conservé en clair, donc laissez ceci désactivé sur une machine que d'autres utilisent.
login-keeps-password-device-clear = Conserve le mot de passe sur cet appareil, en clair.
login-checking = {" "}VÉRIFICATION…{" "}
login-sign = {" "}SE CONNECTER{" "}
login-wrong-password-shuts-server-s = Un mauvais mot de passe ferme la porte du serveur pendant quelques secondes, donc cela peut prendre un moment.

## help (src/help.rs)

help-find = RECHERCHER
help-find-manual = rechercher dans le manuel
help-previous-match-shift-enter-shift = Résultat précédent (Maj+Entrée, Maj+F3)
help-next-match-enter-f3 = Résultat suivant (Entrée, F3)
help-no-matches = aucun résultat
help-clear-search-esc = Effacer la recherche (Échap)
help-sdroxide-manual = MANUEL DE SDROXIDE
help-contents = SOMMAIRE

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = 1 station entendue — en attente qu'elle signale une position
aismap-double-click-reframe = double-cliquez pour recadrer
aismap-zoom-out-wheel-pinch-do = Dézoomer — la molette ou un pincement font de même
aismap-zoom-about-middle-chart = Zoomer autour du milieu de la carte
aismap-frame-everything-being-tracked-again = Recadrer tout ce qui est suivi — un double-clic fait de même

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = en attente du spectre…
specview-hounds = CHIENS

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = Cette fenêtre n'a pas de moteur de rendu GPU, donc la cascade reste à sa largeur standard.
spectrum-machine-drawing-without-gpu-every = Cette machine dessine sans GPU — chaque colonne de la cascade est le travail du processeur, qu'il partage déjà avec la radio.
spectrum-window-renders-through-opengl-which = Cette fenêtre effectue le rendu par OpenGL, qui est la voie de compatibilité de sdroxide — un Raspberry Pi, une puce graphique ancienne, ou un navigateur sans WebGPU. Une cascade plus large n'en vaut pas la peine là, à cause de la fréquence d'images.
spectrum-wider-than-renderer-can-draw = Plus large que ce moteur de rendu peut dessiner.

## macros (src/app/panels/macros.rs)

macros-nocall = SANS APPEL
macros-remove-button = Supprimer ce bouton

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = SUIVI DES CARRÉS
gridtracker-drag-pan-wheel-zoom-amber = Glisser pour déplacer, molette pour zoomer. Ambre = travaillé, vert = confirmé, cyan = entendu.

## remote (src/remote.rs)

remote-server-busy-another-client-connected = serveur occupé — un autre client est connecté
remote-transmitting-but-client-s-microphone = Émission, mais le microphone de ce client ne produit aucun audio — rien n'est modulé. Dans un navigateur, vérifiez que la page a obtenu l'accès au microphone et qu'aucune autre application ne le tient.
remote-connection-closed = connexion fermée

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = Vue 3D du système solaire — Soleil, Terre, Lune, taches solaires et CME (fenêtre séparée)
solarbtn-solar-system-3d-view-sun-2 = Vue 3D du système solaire — Soleil, Terre, Lune, taches solaires et CME (nouvel onglet du navigateur)

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = Chargement de la voix…
speech-browser-client-cannot-speak-yet = le client navigateur ne peut pas encore parler

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = Double-cliquez pour saisir une fréquence
freqdisplay-tap-type-frequency = Touchez pour saisir une fréquence

## solarstate (src/solar3d/state.rs)

solarstate-home = ACCUEIL
solarstate-dwarf-planets = PLANÈTES NAINES

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = double-cliquez pour recadrer

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = double-cliquez pour recadrer

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = connexion fermée

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = impossible de démarrer le fil d'alerte

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = aucun avion localisé pour l'instant

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = DOUBLE-CLIQUEZ POUR RECADRER

## chrome (src/chrome.rs)


## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = Règle le réglage de puissance propre au FDM-DUO (menu 34 TX POWER). Excitation et Accord règlent ensuite la puissance à l'intérieur de celui-ci, et SDRoxide renvoie le réglage à chaque connexion.

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $day } { $month ->
        [1] janv.
        [2] févr.
        [3] mars
        [4] avr.
        [5] mai
        [6] juin
        [7] juil.
        [8] août
        [9] sept.
        [10] oct.
        [11] nov.
       *[12] déc.
    }

## download (src/download.rs)

download-browser-could-not-read-file = le navigateur n'a pas pu lire ce fichier

## morse (src/app/morse.rs)

morse-translate = TRADUIRE
morse-practice = S'ENTRAÎNER
morse-learn = APPRENDRE
morse-morse = MORSE
morse-text-morse = Texte → Morse
morse-morse-text = Morse → Texte
morse-per-character-space-between-characters = `.` et `-` par caractère, un espace entre les caractères, ` / ` entre les mots. L'alphabet UIT, les chiffres, la ponctuation et les prosignes courantes.
morse-reference-translator-available-desktop-build = Le traducteur de référence est disponible dans la version de bureau.
morse-play-what-you-type-your = Jouer ce que vous tapez, sur vos propres haut-parleurs
morse-play = JOUER
morse-stop = ARRÊT
morse-every-character-learned-well-done = Tous les caractères appris — bravo.
morse-reveal = RÉVÉLER
morse-reset = RÉINITIALISER
morse-replay = REJOUER
morse-what-did-you-hear = Qu'avez-vous entendu ?
morse-check = VÉRIFIER
morse-press-new-hear-character = Appuyez sur NOUVEAU pour entendre un caractère.
morse-tone = Tonalité
morse-speed = Vitesse
morse-spacing = Espacement
morse-characters-go-out-speed-above = Les caractères partent à la vitesse ci-dessus ; c'est la vitesse à laquelle sont envoyés les *intervalles*, si bien qu'un débutant entend la lettre dans son ensemble avant de pouvoir la transmettre aussi vite.
morse-playback-available-desktop-build = La lecture est disponible dans la version de bureau.

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = altitude
adsb-row-track = route
adsb-row-turning = virage
adsb-row-squawk = squawk
adsb-row-range = distance
adsb-row-position = position
adsb-row-signal = signal
adsb-row-frames = trames
rade-transmitting = émission
decodes-odd = IMPAIR
decodes-new = NOUVEAU
js8-new = NOUVEAU

## AIS panel: vessel detail rows.

ais-row-status = état
ais-row-speed = vitesse
ais-row-course = route
ais-row-heading = cap
ais-row-turning = virage
ais-row-altitude = altitude
ais-row-size = taille
ais-row-draught = tirant d'eau
ais-row-destination = destination
ais-row-range = distance
ais-row-signal = signal
ais-row-type = type
ais-row-position = position
ais-row-messages = messages
ism-off = ARRÊT
textmodem-frequency-shifted = décalée en fréquence
winlink-to-heading = À

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = { $heard } entendus — aucune position plus récente que { $drop_map_s } s
aismap-stations-heard-waiting-position-report = { $heard } stations entendues — en attente d'un rapport de position
aismap-km-across = { $lon_span }° · ~{ $km } km de large

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = alarme sur { $device }
alerts-alarm-device-unavailable = périphérique d'alarme indisponible : { $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = { $w } travaillés
awards-confirmed = { $value } confirmés

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · prévision de HAMQSL.com, vieille de { $value }
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · prochaine balise dans { $now } s
bands-radiants-placed = radiants placés pour { $lat }°, { $lon }°
bands-zhr = ZHR { $zhr }
bands-radiant = radiant { $alt_deg }° { $az_deg }
bands-radiant-down = radiant sous l'horizon ({ $alt_deg }°)

    Un radiant au-dessus de l'horizon signifie que les traînées peuvent vous atteindre ; une pluie rapide laisse des traînées ionisées plus durables pour la diffusion météorique sur 6 m et 2 m, et les brèves ouvertures sur 10 m.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } ne se décode pas
drm-symbols = { $value } symboles · { $frame }
drm-not-decodable = { $value } — non décodable
drm-broadcaster-s-clock-utc = horloge du diffuseur { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    MAX : la pleine puissance du FDM-DUO, environ 8 W. Le réglage à utiliser avec le HR500 en ligne.

    { $base }
duopower-t-5-w-hr500-line-will =
    5 W. Avec le HR500 en ligne, il n'atteindra pas la pleine puissance sur la plupart des bandes à ce réglage : utilisez MAX, et Excitation pour la faire descendre.

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } W, pour fonctionner sans amplificateur ou en QRP.

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = Réactive l'émission après { $swr }:1. Vérifiez d'abord l'antenne : si le défaut est toujours là, la prochaine émission s'arrête aussi.
frame-reconnecting-s = Reconnexion dans { $value } s…

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = { $worked } travaillés
gridtracker-confirmed = { $confirmed } confirmés
gridtracker-heard = { $heard } entendus

## hd: messages with values (src/app/hd.rs)

hd-decode = Décoder { $value } ({ $codec_label })

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } kHz a déjà une puce
hfdl-save-current-channel-khz-one = Enregistrer le canal actuel ({ $cur_khz } kHz) comme l'une de vos propres puces
hfdl-aircraft = { $aircraft } avions
hfdl-bursts = { $bursts } rafales
hfdl-decodes-2 = { $decodes } décodages
hfdl-fixed = { $fec } corrigés

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint } — pas encore décodé
ism-listen-tunes-radio-there-one = Écouter sur { $value } — accorde la radio là. Une bande à la fois : elles sont trop éloignées pour qu'un récepteur en couvre deux à la fois.
ism-watch-hz-around-band-centre = Surveiller { $value } Hz autour du centre de la bande, quoi que la bande aurait demandé. Plus étroit convient à un récepteur qui ne peut pas fournir la largeur propre à la bande ; plus large atteint les appareils situés plus loin du centre. Votre récepteur doit fournir environ un tiers de plus que cela.
ism-rtl-433 = rtl_433 — { $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } kHz
ism-your-own = { $flex } à vous
ism-decoders-decoded = { $decoders } décodeurs ({ $flex }), { $decodes } décodés
ism-handled-rtl-433 = géré par rtl_433 : { $names }
ism-flex = flex : { $error }
ism-window-mhz-mhz-wide = fenêtre { $mhz } MHz, large de { $e62 } MHz
ism-bursts-decoded = { $bursts } rafales, { $decodes } décodées
ism-devices = { $ism_reports } appareils
ism-id-mhz-db-heard-first =
    { $protocol } id { $device }
    { $e6 } MHz { $snr_db } dB entendu { $value }×
    premier entendu il y a { $first_at }

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = Confirmations : { $recs } téléchargées, { $matched } nouvellement confirmées
logbook-qso = { $qso_log } QSO
logbook-worked-before = ⚠ DÉJÀ TRAVAILLÉ ({ $dupe_band })
logbook-utc-qso = { $oldest }–{ $newest } UTC · { $value } QSO

## app: messages with values (src/app/mod.rs)

app-radio = radio { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = impossible de démarrer le fil audio Morse : { $error }
morse-no-audio-output = aucune sortie audio : { $error }
morse-about-s = environ { $farnsworth_wpm } s
morse-characters-play = Caractères en jeu : { $set }
morse-score-run = Score : { $correct } / { $total }. Série : { $streak } sur { $value }.
morse-correct = juste — { $value }
morse-was = c'était { $value }

## net: messages with values (src/app/net.rs)

net-adif-import-failed = Échec de l'import ADIF : { $error }
net-not-unicode-read = {" "}(pas en Unicode ; lu comme { $enc })
net-received-reports-swl-left-out = , { $n } rapports de réception (SWL) laissés de côté
net-adif-import-added-duplicates-skipped = Import ADIF : { $added } ajoutés, { $skipped } doublons ignorés{ $reports }{ $assumed }
net-channel-import-failed = Échec de l'import des canaux : { $error }
net-line-s-skipped = , { $n } ligne(s) ignorée(s)
net-not-unicode-read-2 = {" "}(pas en Unicode ; lu comme { $enc })
net-channel-import-channel-s-read = Import des canaux : { $n } canal(aux) lu(s){ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = { $frames } trames · { $bad } mauvaises
acars-mode = mode { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = { $aircraft } avions
adsb-frames = { $frames } trames
adsb-preambles = { $preambles } préambules
adsb-bad-crc = { $bad_crc } CRC mauvais
adsb-tune = ACCORD { $mhz }
adsb-ft-barometric = { $ft } ft barométriques

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = { $vessels } navires
ais-messages = { $messages } messages
ais-slots = { $bursts } créneaux
ais-bad-fcs = { $bad_fcs } FCS mauvais
ais-ais-mhz = AIS { $value } ({ $mhz } MHz) — { $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value } ({ $mhz } MHz) — { $bursts } créneaux, { $messages } messages, plancher { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = ACCORD { $mhz }
ais-ships-heard-are-khz-off = les navires entendus sont décalés de { $khz } kHz en fréquence ({ $mhz } ppm à 162 MHz) — réglez la correction de fréquence de l'étage d'entrée, sinon les transmissions au-delà d'environ 5 kHz cesseront de se décoder
ais-over-ground = { $value }° par rapport au sol
ais-dbfs-db-snr = { $rssi_dbfs } dBFS, { $snr_db } dB de RSB
ais-last-type-ais = { $messages } (dernier type { $last_type }, AIS { $channel })
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } MHz — canal maritime { $ch }. Un navire alterne entre les deux, donc en désactiver un divise par deux la fréquence à laquelle chaque navire est entendu.

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = objet de { $reported_by }
aprs-via = via { $via }
aprs-object-reported = objet signalé par { $reported_by }
aprs-digit-s-blanked = ± { $ambiguity } chiffre(s) masqué(s)
aprs-wind-m-s = vent { $value }° { $value2 } m/s
aprs-gust = rafale { $value }
aprs-rain-mm-h = pluie { $value } mm/h
aprs-frame-s-last-ago = { $packets } trame(s), la dernière il y a { $last_heard }
aprs-via-2 = via { $via }
aprs-bad = { $bad_frames } mauvaises
aprs-other = { $non_aprs } autres
aprs-sent = envoyé ×{ $tries }
aprs-stn = { $stations } stn
aprs-queued = { $tx_queue } en file

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = maître { $value }
atchat-last-heard-s-ago-click = { $status } — dernière écoute il y a { $age_s } s · cliquez pour ouvrir un onglet de message direct
atchat-close-tab = fermer l'onglet { $peer }
atchat-direct-message = message direct à { $value }
atchat-cannot-display = impossible d'afficher { $path }

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Cliquez pour le décodeur { $neural }. Le neuronal lit plus loin et lit le CW manipulé à la main que l'ajustement temporel n'accepte pas ; le temporel est le seul qui lit les lettres accentuées — Ä, Ö, Å, Ü, É — parce que le modèle n'a pas de classe de sortie pour elles.
cw-idle-s = REPOS { $idle } s

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = Enregistrer les { $n } décodages de la liste en ADIF — rapports de réception, non contacts. Les décodages qui ne nomment aucun émetteur sont laissés de côté.
decodes-save-decodes-list-csv-one = Enregistrer les { $n } décodages de la liste en CSV, une ligne chacun
decodes-session-qso = Session : { $session } QSO
decodes-save-whole-logbook-qso-adif = Enregistrer tout le journal ({ $logged } QSO) en ADIF
decodes-save-whole-logbook-qso-text = Enregistrer tout le journal ({ $logged } QSO) en texte
decodes-hz-slots = { $audio_hz } Hz · { $odd } créneaux
decodes-your-slot-timing-against-stations =
    Votre synchronisation de créneau par rapport aux stations que vous entendez.
    { $after }

    Cela couvre toute la chaîne de réception, donc une chaîne audio ou réseau lente compte comme une mauvaise horloge. Sous 0,5 s, c'est confortable.
decodes-pile-up = PILE-UP { $fox_queue }
decodes-queue = FILE { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } dB · { $audio_hz } Hz{ $unwrap_or_default }
    Cliquez pour retirer

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = Toutes les vitesses sont décodées ; ▸ marque { $speed }, celle à laquelle vous émettez
js8-every-speed-being-decoded-click = Toutes les vitesses sont décodées. Cliquez pour émettre à { $speed } à la place
js8-transmit-decode = Émettre et décoder à { $speed }
js8-beaconing-every-min-click-stop = Balise toutes les { $hb_min } min — cliquez pour arrêter
js8-last-beacon-went-out-hz = La dernière balise est partie à { $hz } Hz — un créneau libre dans la sous-bande de pulsation de { $value }–{ $value2 } Hz, choisi pour tomber à l'écart des signaux décodés.
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } trames · { $left } s
js8-decoded = Décodé à { $speed }
js8-frames = … ({ $frames } trames)
js8-reply-2 = Répondre à { $value } : « { $value2 } »
js8-address-composer = Adresser le rédacteur à { $value }

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i } : envoie « { $text } »
macros-sends = Envoie « { $text } »
macros-most = { $value } est le maximum

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = { $repaired } réparés · { $lost } perdus
navtex-characters-lost = { $lost } caractères perdus

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = { $bad_frames } mauvaises
packet-link-layer = La couche liaison est en { $link }
packet-via = {" "}via { $via }
packet-unacked = { $unacked } non acquittées
packet-retry = nouvelle tentative { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] balise
       *[other] balises
    }
pi4-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } il y a

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = RSB { $snr_db } dB
rade-offset-hz = décalage { $freq_offset_hz } Hz
rade-heard = entendu { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ { $dropped } échantillons perdus entre le récepteur et le décodeur — cette machine ne suit pas le décodage neuronal.

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } car./s
textmodem-char-s-hz-wide = { $value } — { $chars_per_sec } car./s, large de { $bandwidth_hz } Hz, { $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = { $frames } trames
vdl2-bursts = { $bursts } rafales
vdl2-sync = { $syncs } synchro
vdl2-hdlc-bad = { $hdlc_bad } HDLC mauvais
vdl2-bad-fcs = { $fcs_bad } FCS mauvais
vdl2-mhz-bursts-frames-noise-floor = { $mhz } MHz, { $what } — { $bursts } rafales, { $frames } trames, bruit de fond { $floor_dbfs } dBFS
vdl2-tune = ACCORD { $mhz }
vdl2-held-2 = FIGÉ +{ $since }
vdl2-use-worldwide-where-every-link = { $what } — utilisé dans le monde entier, et là où commence chaque liaison. Celui à garder si vous n'en gardez qu'un.
vdl2-assigned-which-channels-carry-anything = Attribué à un { $what }. Les canaux qui transportent quelque chose dépendent de l'endroit où vous êtes : laissez-les tous activés sauf si vous savez mieux.
vdl2-mhz-db-snr-dbfs-evm = { $mhz } MHz · { $snr_db } dB de RSB · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Hz · { $rs_corrected } symboles RS corrigés
vdl2-characters = { $parity_errors } caractères
vdl2-parameter-sdroxide-does-not-read = { $unknown } { $unknown ->
        [one] paramètre
       *[other] paramètres
    } que SDRoxide ne lit pas — affichés plus haut sous forme d'identifiant et d'octets

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = { $live_size } lignes conservées
wefax-pixels-per-line = { $width } pixels par ligne
wefax-shift-picture-pixels = Décaler l'image de { $px } pixels
wefax-charts-are-saved-click-copy =
    Les cartes sont enregistrées { $place }

    Cliquez pour copier le chemin
wefax-finished-charts-are-saved-collect = Les cartes terminées sont enregistrées { $place } et s'accumulent ici.
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } pixels
    { $value }

    Clic droit pour supprimer
wefax-older-store = { $older } plus anciennes dans le stock
wefax-older-load-more = { $older } plus anciennes — en charger davantage
wefax-published-carrier-khz-dial-khz = Porteuse publiée { $f } kHz → cadran { $value } kHz USB
wefax-delete-chart-2 = Supprimer cette carte { $dir }

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · zone CQ { $cq_zone } · zone UIT { $itu_zone }
widgets-new-entity = Nouvelle entité sur { $band_label }
widgets-worked-before = Déjà travaillé sur { $band_label }
widgets-calling-cq-includes-you = Appel CQ { $target } — vous êtes inclus
widgets-calling-cq-not-aimed-you = Appel CQ { $target } — non destiné à vous

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = { $heard_us } rx · { $heard_us2 } nous ont entendus
wspr-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] balise
       *[other] balises
    }
wspr-beaconing-duty = balise — { $wspr_power_dbm }, cycle de { $wspr_tx_percent } %
wspr-listening-duty = écoute (cycle de { $wspr_tx_percent } %)
wspr-beacon-one-two-minute-slot = Balise dans un créneau de deux minutes sur chaque { $pct } — soit { $pct2 } % d'entre eux, répartis régulièrement — à { $wspr_power_dbm }. Le créneau où commence le cycle est tiré de votre indicatif, si bien que deux stations qui exécutent ce programme ne commencent pas ensemble.
wspr-transmitting-set-general-tab-settings = Émission sous { $call } dans { $sent } — réglé dans l'onglet Général des Réglages.
wspr-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } il y a

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    antenne : { $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = { $entries } récepteurs · { $per_net } · récupéré il y a { $age }
publicsdr-showing-search-narrow = affichage de { $shown } sur { $n } — recherchez pour affiner
publicsdr-match = { $n } correspondance
publicsdr-replace-taking-receiver-points-same =
    Remplacer { $this } par { $entry } ?

    { $this2 } est sur { $iface }. Prendre ce récepteur fait pointer le même onglet sur { $address } à la place, le renomme « { $entry2 } », et maintient le cadran sur la { $range_label } que publie le récepteur — rien ici n'émet.

    Les réglages de { $iface2 } restent en place, ainsi que les plages qui leur sont attribuées : repasser l'interface dans Réglages → Radio rétablit la radio comme elle était.
publicsdr-receptions-will-reported = {" "}Les réceptions seront transmises depuis { $value }.
publicsdr-pointing-radio = Diriger cette radio vers { $entry }…{ $site }
publicsdr-opening-another-radio = Ouverture de { $entry } comme une autre radio…

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } kHz
qo100-before-switching-tune-so-beacon = Avant d'activer : accordez pour que les deux lobes de la balise se trouvent entre +{ $park_lo_hz } et +{ $park_hi_hz } kHz dans la bande (la voie ombrée).
qo100-receiver-currently-listening-around-mhz = Le récepteur écoute actuellement autour de { $mhz } MHz — loin de la balise à 10489,750 MHz, donc il n'y a rien à dessiner ici. Le décodeur continue de chercher quand même.
qo100-receiver-currently-covers-only-khz = le récepteur ne couvre ici que ±{ $khz } kHz — le reste de la bande reste vide
qo100-null-db-sym-snr-db = creux { $est_null_depth_db } dB sym { $est_symmetry } RSB { $est_snr_db } dB
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Hz/s { $est_drift_accel_hz_s2 } Hz/s² (dé-rotation avant décodage)
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Hz/s (dé-rotation avant décodage)
qo100-found-empty = { $est_updates } trouvées, { $est_misses } vides
qo100-last-s-ago = {"   "}dernière { $auto_last_hz } (il y a { $auto_last_unix } s)
qo100-over-correction = { $auto_total_hz } sur { $auto_applies } { $auto_applies ->
        [one] correction
       *[other] corrections
    }{ $last }
qo100-sync-32-err = synchro { $sync_bit_errors } / 32 err ×{ $sync_matches }
qo100-frame-buffer = mémoire de trames { $frame_fill } %
qo100-blocks-tried-decoded = { $blocks_tried } blocs essayés, { $blocks_locked } décodés
qo100-last-applied-s-ago-hz = dernière appliquée il y a { $ago } s : { $old } → { $new } Hz ({ $old2 } Hz)
qo100-locked-block-tried-locked = verrouillé — { $blocks_tried } { $blocks_tried ->
        [one] bloc essayé
       *[other] blocs essayés
    }, { $blocks_locked } verrouillés
qo100-searching-block-tried-none-locked = recherche — { $blocks_tried } { $blocks_tried ->
        [one] bloc essayé
       *[other] blocs essayés
    }, aucun verrouillé pour l'instant

## rds: messages with values (src/app/rds.rs)

rds-auto = Auto ({ $standard })
rds-type = type { $class }
rds-groups = · { $groups } groupes
rds-blocks-error = · { $ber } % des blocs en erreur
rds-clean-corrected-lost = { $blocks_ok } propres · { $blocks_corrected } corrigés · { $blocks_bad } perdus

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC ({ $offset } locale)
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } au-dessus de l'horizon
sat-min = dans { $value } min
sat-h-min = dans { $value } h { $value2 } min
sat-searching = recherche… { $next } sur { $entries }
sat-locked-2 = ● VERROUILLÉ — { $t }
sat-pass-until-max = Passage jusqu'à { $set_unix } · max { $max_el }°
sat-next-pass-rises-max = Prochain passage { $rise_unix } { $rise_unix2 } ({ $now }) · monte à { $rise_az }° { $rise_az2 } · max { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ { $clients } { $clients ->
        [one] client rigctld connecté
       *[other] clients rigctld connectés
    } — un logiciel satellite qui pilote le cadran là corrigerait l'effet Doppler deux fois
sat-rotator-antenna-az-el = Rotateur : antenne à { $az }° az { $el }° el
sat-rotator-2 = Rotateur : { $error }
sat-up-now-2 = ● au-dessus de l'horizon, { $el }°
sat-inv = { $mode } · inv
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = maintien sur { $here } MHz
scanner-scanning-mhz = balayage · { $here } MHz

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = Ce moteur de rendu ne peut pas contenir une texture plus large que { $max_texture_dim } pixels.

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = Voix : { $voice }
speech-speech-unavailable = La parole est indisponible : { $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = { $stations } émissions

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = dernière session échouée — { $err }
winlink-last-session-received-sent = dernière session : { $last_received } reçus, { $last_sent } envoyés
winlink-empty = { $to_lowercase } est vide
winlink-showing = affichage de { $entries } sur { $total }
winlink-fetching = récupération de { $mid }…
winlink-from-2 = de { $msg } { $date }
winlink-attachment-bytes = pièce jointe : { $att } ({ $data } octets)

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    objet de { $reported_by }
aprsmap-heard-ago =
    {""}
    entendu il y a { $age }

## download: messages with values (src/download.rs)

download-reading = lecture de { $path } : { $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}image manquante : { $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } MHz
hfdlmap-fix =
    {""}
    { $fixes } fix{ $es }
hfdlmap-heard-ago =
    {""}
    entendu il y a { $last_at }

## multi: messages with values (src/multi.rs)

multi-radio = Radio { $n }
multi-could-not-add-radio = Impossible d'ajouter une radio : { $error }
multi-was-closed-station = { $value } a été fermée sur la station.
multi-could-not-open = Impossible d'ouvrir { $peer } : { $error }
multi-has-tab-own-now = { $value } a maintenant son propre onglet.

## remote: messages with values (src/remote.rs)

remote-protocol-error = erreur de protocole : { $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = mauvais message : { $error }
solarnet-solar-protocol-mismatch-server-client = protocole solaire incompatible : serveur { $proto }, client { $value }

## smeter: messages with values (src/widgets/smeter.rs)

smeter-forward-power-w-measured-radio =
    {""}

    Puissance directe — { $w } W, mesurée par le capteur propre à la radio.
smeter-po-radio-s-own-power =
    {""}

    PO — le wattmètre de sortie propre à la radio, à { $clamp } % de sa pleine échelle. Une position d'aiguille telle que la radio la signale, non une puissance étalonnée ; ce qu'il faut surveiller, c'est si elle baisse pendant que l'Excitation reste en place.
smeter-swr-1-match-antenna-presenting =
    {""}

    ROS — { $swr }:1, l'adaptation que présente l'antenne, telle que la radio la mesure. En dessous de 2:1, rien d'extraordinaire ; au-delà de 3:1, l'échelle passe au rouge.
smeter-received-signal-strength-s9-db = Force du signal reçu : S9 + { $over } dB ({ $s_dbm } dBm dans la bande passante).
smeter-received-signal-strength-s-dbm = Force du signal reçu : S{ $value } ({ $s_dbm } dBm dans la bande passante).
smeter-radio-reports-c =
    {""}

    La radio annonce { $t } °C.
smeter-ps-puresignal-correcting-db-compression =
    {""}

    PS — PureSignal corrige { $correction_db } dB de compression{ $held } (la rétroaction correspondait à l'émission à { $score }).
smeter-ps-puresignal-switched-but-has =
    {""}

    PS −− — PureSignal est activé mais n'a pas trouvé l'émission dans le trajet de rétroaction (meilleure correspondance { $score }), donc l'émetteur sort sans correction. Vérifiez qu'un échantillon de la sortie de l'amplificateur atteint bien une entrée de réception que le commutateur E/R ne retire pas à l'émission, et que son atténuateur ne l'avale pas. La boucle n'écoute que ce récepteur — un coupleur relié à un second récepteur (RX2, ADC2) n'est jamais lu comme rétroaction.

## specview: messages with values (src/widgets/spectrum_view.rs)

specview-rit-hz = RIT { $hz } Hz
specview-xit-hz = XIT { $hz } Hz

## decodes: nested messages

decodes-you-transmit-s-everyone-else = Vous émettez { $off } s { $after } tous les autres — la raison habituelle des appels restés sans réponse. Synchronisez l'horloge de votre ordinateur.

## publicsdr: nested messages

publicsdr-noise-floor-score = score de bruit de fond { $snr }
publicsdr-up-khz-i-q = jusqu'à { $khz } kHz d'I/Q

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = RG porte { $gate_gr_db } dé-ess { $deess_gr_db } comp { $comp_gr_db } lim { $limiter_gr_db }

## amp: more messages

amp-h-step = ~{ $l_step } µH · pas { $l_step2 }
amp-pf-step = ~{ $c_step } pF · pas { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel } bits · { $media_type } / { $content_encoding } · { $encoded_size } octets en { $chunk_count } fragments ({ $chunks_first_pass } au premier passage) · session { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value } — { $symbol_rate } bauds CPFSK, ±{ $deviation_hz } Hz, largeur occupée { $bandwidth_hz } kHz

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · { $local } locale
sat-in-seconds = dans { $s } s
winlink-to-list = à { $to }
winlink-cc-list = cc { $cc }
spots-n-of-total = { $n } sur { $total }
wefax-n-of-total = { $n } sur { $total }
ais-range-bearing = { $km } km à { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = Désactivé (accord seul, au-dessus de 24 MHz)
rtl-sdr-hf-mode-auto = Automatique
rtl-sdr-hf-mode-direct-q = Échantillonnage direct (branche Q)
rtl-sdr-agc-manual = Manuel (sans CAG)
rtl-sdr-agc-tuner = CAG de l'accord
rtl-sdr-agc-rtl = CAG numérique RTL
rtl-sdr-agc-both = CAG accord + RTL
mode-control-cat = CAT
mode-control-radio = Contrôlé par la radio
line-state-none = Aucun
line-state-high = Haut
line-state-low = Bas
elad-tx-input-usb-audio = Audio USB
elad-tx-input-mic = Microphone
elad-tx-input-auto = Automatique
elad-tx-input-leave = Laisser comme réglé sur la radio
tr-usdx-audio-one-cable = Un seul câble (audio dans le flux CAT)
tr-usdx-audio-sound-card = Carte son USB (prise 3,5 mm)
spy-server-format-uint8 = 8 bits (débit le plus faible)
spy-server-format-int16 = 16 bits
spy-server-format-float32 = Flottant 32 bits
sound-format-iq = IQ (stéréo)
sound-format-demod-audio = Audio démodulé
sdr-play-duo-tuner-tuner1 = Accord 1 (50 Ohm / Hi-Z)
sdr-play-duo-tuner-tuner2 = Accord 2 (50 Ohm)
sdr-play-duo-role-diversity = Une seconde antenne (diversité / suppression de QRM)
sdr-play-duo-role-second-radio = Une seconde radio, sur sa propre fréquence
sdr-play-agc-off = Désactivé (gain FI manuel)
sdr-play-agc-hz5 = 5 Hz
sdr-play-agc-hz50 = 50 Hz
sdr-play-agc-hz100 = 100 Hz
pluto-ptt-off = Désactivé
pluto-ptt-gpo01 = GPO0 = RX, GPO1 = TX
pluto-ptt-gpo23 = GPO2 = RX, GPO3 = TX
pluto-duplex-fdd = FDD (les deux à la fois)
pluto-duplex-tdd = TDD (un à la fois)
pluto-agc-manual = Manuel
pluto-agc-slow-attack = Attaque lente
pluto-agc-fast-attack = Attaque rapide
pluto-agc-hybrid = Hybride
parity-none = Aucune
parity-even = Paire
parity-odd = Impaire
panadapter-tap-antenna = Antenne (partagée avec la radio)
panadapter-tap-if-output = La sortie F.I. de la radio
panadapter-audio-attached = Le récepteur relié
panadapter-audio-transceiver = L'émetteur-récepteur
kenwood-send-ts2000 = Style TS-2000 (TX;)
kenwood-send-ts590 = Style TS-590 (TX1;)
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = Contrôlé par la radio
cw-keying-cat = Manipulateur de la radio (CAT)
cw-keying-audio = Carte son (MCW)
alert-sound-ding = Ding
alert-sound-two-tone = Deux tons
alert-sound-triplet = Triplet
alert-sound-warble = Trémolo
alert-sound-digital = Numérique
alert-reply-tone = Tonalité
alert-reply-voice = Voix
alert-reply-both = Tonalité + voix
icom-model-ic7300 = IC-7300
icom-model-ic7300-mk2 = IC-7300MK2
icom-model-ic705 = IC-705
icom-model-ic7760 = IC-7760
icom-model-ic905 = IC-905
icom-model-ic9700 = IC-9700
icom-model-ic7610 = IC-7610
icom-model-ic7100 = IC-7100
icom-model-ic7410 = IC-7410
icom-model-ic7600 = IC-7600
icom-model-ic7700 = IC-7700
icom-model-ic7800 = IC-7800
icom-model-ic9100 = IC-9100
icom-model-ic7200 = IC-7200
icom-model-ic7000 = IC-7000
icom-model-other = Autre (régler l'adresse à la main)
backend-auto = Détection automatique (SoapySDR / CAT)
backend-soapy = SoapySDR
backend-cat = CAT / Audio
backend-hpsdr = HPSDR (réseau)
backend-tci = TCI (réseau)
backend-icom-net = Icom LAN (réseau)
backend-smart-sdr = SmartSDR / FlexRadio (réseau)
backend-pluto = PlutoSDR (réseau)
backend-rtl-sdr = RTL-SDR (USB)
backend-rtl-tcp = RTL-SDR par rtl_tcp (réseau)
backend-spy-server = SpyServer (réseau)
backend-spy-server-vfo = SpyServer VFO+FFT, faible débit (réseau)
backend-kiwi-sdr = KiwiSDR / Web-888 (réseau)
backend-rx888 = RX-888 (USB)
backend-airspy-hf = Airspy HF+ (USB)
backend-airspy = Airspy R2 / Mini (USB)
backend-hack-rf = HackRF One / Pro (USB)
backend-sdr-play = SDRplay RSP (USB)
backend-elad = ELAD FDM-DUO / FDM-S (USB)
backend-lime = LimeSDR + LimeRFE (LimeSuite)
backend-hydra-sdr = HydraSDR RFOne (USB)
backend-fobos = RigExpert Fobos SDR (USB)
backend-usb-audio = Radio USB audio (carte son)
backend-none = Non configurée
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld (réseau)
cat-family-flrig = flrig (réseau)
publicsdr-its-interface = son interface

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = Arrêt
agc-mode-slow = Lent
agc-mode-med = Moyen
agc-mode-fast = Rapide
airspy-gain-linearity = Linéarité (meilleure tenue aux signaux forts)
airspy-gain-sensitivity = Sensibilité (meilleure pour les signaux faibles)
ais-kind-class-a = Classe A
ais-kind-class-b = Classe B
ais-kind-base-station = station de base
ais-kind-sar-aircraft = avion SAR
ais-kind-aid-to-navigation = aide à la navigation
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = engin associé
alert-event-called = Une station m'appelle
alert-event-cq = Un CQ auquel je pourrais répondre
alert-event-new-dxcc = Nouvelle entité DXCC
alert-event-new-dxcc-band = Nouvelle entité sur cette bande
alert-event-new-grid = Nouveau carré
audio-routing-both = Gauche + droite
audio-routing-left = Gauche seulement
audio-routing-right = Droite seulement
button-mode-momentary = Maintenir
button-mode-toggle = Bascule
contest-mode-none = Aucun
contest-mode-eu-vhf = Concours VHF européen
converter-tx-off = Désactivé pendant la conversion
converter-tx-transverter = Par le même convertisseur
converter-tx-own = Son propre décalage
cw-engine-neural = NEURONAL
cw-engine-timing = TEMPOREL
decode-sort-none = Aucun
decode-sort-signal = RSB
decode-sort-distance = Dist
decode-sort-country = Pays
diversity-mode-cancel = Annuler — éteindre une source de bruit
diversity-mode-combine = Combiner — réception en diversité
dxped-mode-normal = Normal
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = Refuser d'émettre
fail-safe-warn-only = Émettre quand même, avec avertissement
ftp-security-plain = FTP (sans chiffrement)
ftp-security-explicit-tls = FTPS explicite (AUTH TLS)
ftp-security-implicit-tls = FTPS implicite
hpsdr-filter-board-none = Aucune — les sorties restent désactivées
hpsdr-filter-board-n2adr = Carte de filtres N2ADR
hpsdr-filter-board-alex = Code de bande Alex / Hermes (Zeus, HiQSDR, Quisk)
hpsdr-filter-board-custom = Personnalisé — mon propre tableau ci-dessous
hpsdr-io-rx-input-radio = Entrée propre de la radio
hpsdr-io-rx-input-io-board = Carte IO J9
hpsdr-io-rx-input-io-board-pure-signal = Carte IO J9, PureSignal à l'émission
hydra-sdr-gain-linearity = Linéarité (meilleure tenue aux signaux forts)
hydra-sdr-gain-sensitivity = Sensibilité (meilleure pour les signaux faibles)
icom-rx-source-af = AF — la radio démodule
icom-rx-source-if12k = FI 12 kHz — sdroxide démodule
icom-scope-span-radio = Comme réglé sur la radio
icom-scope-span-khz2-5 = 5 kHz (±2,5 k)
icom-scope-span-khz5 = 10 kHz (±5 k)
icom-scope-span-khz10 = 20 kHz (±10 k)
icom-scope-span-khz25 = 50 kHz (±25 k)
icom-scope-span-khz50 = 100 kHz (±50 k)
icom-scope-span-khz100 = 200 kHz (±100 k)
icom-scope-span-khz250 = 500 kHz (±250 k)
icom-scope-span-khz500 = 1 MHz (±500 k)
ism-family-weather = MÉTÉO
ism-family-meter = COMPTEURS
ism-family-home-auto = MAISON
ism-family-lo-ra = LORA
ism-family-unidentified = INCONNU
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = non identifié
js8-speed-normal = NORMAL
js8-speed-fast = RAPIDE
js8-speed-turbo = TURBO
js8-speed-slow = LENT
lime-aux-role-off = Non utilisée
lime-aux-role-diversity = Une seconde antenne (diversité / suppression de QRM)
lime-aux-role-pure-signal = Rétroaction d'émission (prédistorsion PureSignal)
lookup-provider-none = Désactivé
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = RÉCEPTION
mail-folder-outbox = ENVOI
mail-folder-sent = ENVOYÉS
mail-folder-archive = ARCHIVES
memory-sort-name = Nom
memory-sort-freq = Fréq
memory-sort-band = Bande
memory-sort-stored = Mémorisé
mouse-button-middle = Milieu
mouse-button-extra1 = Arrière (supplémentaire 1)
mouse-button-extra2 = Avant (supplémentaire 2)
nr-level-off = Arrêt
nr-level-low = Faible
nr-level-medium = Moyenne
nr-level-high = Forte
nr-level-rnn-low = RNN faible
nr-level-rnn-med = RNN moyen
nr-level-rnn-high = RNN fort
nr-level-spec-low = SPEC faible
nr-level-spec-med = SPEC moyen
nr-level-spec-high = SPEC fort
nr-level-nr2-low = NR2 faible
nr-level-nr2-med = NR2 moyen
nr-level-nr2-high = NR2 fort
nr-level-df-low = DFNR faible
nr-level-df-med = DFNR moyen
nr-level-df-high = DFNR fort
nr-strength-low = Faible
nr-strength-med = Moyenne
nr-strength-high = Forte
orbit-rings-none = aucun
orbit-rings-curated = sélection
orbit-rings-all = tous
qso-step-idle = Repos
qso-step-wait-cq = Attente CQ
qso-step-calling-cq = Appel CQ
qso-step-tx-grid = Émission carré
qso-step-tx-report = Émission rapport
qso-step-tx-r-report = Émission R+rapport
qso-step-tx-rr73 = Émission RR73
qso-step-tx73 = Émission 73
qso-step-confirming = Confirmation
rds-standard-auto = Auto
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = Région 1 — Europe, Afrique, Moyen-Orient, Asie du Nord
region-r2 = Région 2 — Amériques
region-r3 = Région 3 — Asie du Sud et de l'Est, Australasie, Pacifique
relative-mode-absolute = Absolu (curseur)
relative-mode-sign-magnitude = Relatif : signe / amplitude
relative-mode-twos-complement = Relatif : complément à deux
relative-mode-binary-offset = Relatif : centré sur 64
relay-link-off = Aucune
relay-link-serial = Carte à relais USB (série)
relay-link-serial-lines = Ligne série RTS/DTR
relay-link-hid = Carte à relais USB (HID)
relay-link-cm108 = GPIO de carte son CM108/CM119
relay-link-gpio = Ligne GPIO Linux
relay-link-command = Commande externe
relay-role-unused = Non utilisé
relay-role-sdr-antenna = Mettre à la masse l'antenne du SDR
relay-role-amplifier = Commander un amplificateur / relais E/R
relay-role-aux = Auxiliaire
relay-role-band-decoder = Décodeur de bande (filtre / transverteur)
rfe-channel-wb1000 = Large bande 1 – 1000 MHz
rfe-channel-wb4000 = Large bande 1 – 4 GHz
rfe-channel-ham0030 = HF, jusqu'à 30 MHz
rfe-channel-ham0070 = 6 m / 4 m (50 – 70 MHz)
rfe-channel-ham0145 = 2 m (140 – 150 MHz)
rfe-channel-ham0220 = 1,25 m (220 – 225 MHz)
rfe-channel-ham0435 = 70 cm (400 – 450 MHz)
rfe-channel-ham0920 = 33 cm (902 – 928 MHz)
rfe-channel-ham1280 = 23 cm (1220 – 1420 MHz)
rfe-channel-ham2400 = 13 cm (2,3 – 2,5 GHz)
rfe-channel-ham3500 = 9 cm (3,3 – 3,7 GHz)
rfe-channel-cell-band01 = Bande cellulaire 1
rfe-channel-cell-band02 = Bande cellulaire 2
rfe-channel-cell-band03 = Bande cellulaire 3
rfe-channel-cell-band07 = Bande cellulaire 7
rfe-channel-cell-band38 = Bande cellulaire 38
rfe-channel-not-selected = Aucun choisi
rfe-link-off = Non connecté
rfe-link-serial = Son propre câble USB (série)
rfe-link-board = Par le LimeSDR (GPIO / I²C)
rfe-mode-control-auto = Automatique (suit le câblage)
rfe-mode-control-rx = Toujours en réception
rfe-mode-control-tx = Toujours en émission
rfe-mode-control-tx-rx = Toujours les deux (connecteurs séparés seulement)
rifp-encoding-auto = Auto
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = Brut
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = MÉM
scan-kind-range = PLAGE
scan-resume-carrier = PORTEUSE
scan-resume-timed = TEMPORISÉ
scan-resume-manual = MANUEL
sense-line-off = Non câblée
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD (détection de porteuse)
shift-simplex = SIMPLEX
shift-minus = −
shift-plus = +
speed-slow = Lent
speed-medium = Moyen
speed-fast = Rapide
speed-faster = Plus rapide
speed-fastest = Le plus rapide
tls-verify-system = Magasin de confiance du système
tls-verify-pinned = Empreinte épinglée
tls-verify-off = Ne pas vérifier
tone-mode-off = OFF
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN (en rotation)
upload-naming-original = Nom de fichier d'origine
wheel-action-zoom = Zoom
wheel-action-tune = Accord
wheel-action-none = Aucune action

settings-alerts-say = DIRE

settings-alerts-say-hint = Écouter cette alerte énoncée

settings-alerts-say-need-voice = Choisissez Voix ou Tonalité + voix pour l'entendre

settings-alerts-say-voice-off = Activez la voix dans Réglages → Interface → Annonces vocales

settings-alerts-speaks = dit : { $text }

settings-picup-keep = garder

jtty-starting-receiver = démarrage du récepteur JTTY…

jtty-receive-audio-level = Niveau audio de réception

jtty-sending = {" "}ÉMISSION{" "}

jtty-sent-once-note = Le message est envoyé une seule fois quand vous appuyez sur TX — JTTY est asynchrone : il n'y a pas de période à attendre et rien ne se répète.

signalid-title = ID SIGNAL

signalid-tuned-now = Accordé maintenant

signalid-passband = bande passante { $bw }

signalid-band = bande { $band }

signalid-find = Chercher

signalid-find-hint = FT8, météore, météo, avion…

signalid-candidates = { $n } candidats pour ce cadran

signalid-matches = { $n } résultats pour « { $query } »

signalid-nothing-fits = Rien dans le catalogue ne correspond à cette fréquence, ce mode et cette bande passante.

signalid-no-match = Aucune entrée du catalogue ne correspond.

signalid-open-url = Ouvrir { $url }

windows-chip-del = SUPPR

windows-chip-edt = MOD

windows-chip-ren = RENOM

windows-chip-rec = ENR

windows-name-hint = nom

windows-waterfall-marks = Cascade

windows-stacked = empilés

jtty-listening-note = À l'écoute. Les émissions JTTY peuvent commencer à tout moment — un message apparaît ici un instant après avoir été entendu.

extras-title = EXTRAS

extras-receivers = Récepteurs

extras-dab-hint = Radio numérique DAB / DAB+ — le lecteur s'ouvre sous la cascade

extras-aircraft-watch-hint = AÉRONEFS — un aéronef de votre liste de surveillance est en l'air ; ouvrez la vue Surveillance

extras-aircraft-hint = AÉRONEFS — la carte de ce que la radio entend : aéronefs et radiosondes, dans une fenêtre détachable

extras-radio-hint = RADIO CLASSIC — une façade de radio classique pour l'écoute des émissions : bandes, mémoires, nom de la station et RadioText. Elle s'ancre sous la cascade dans les modes phonie ; Réglages → Détachable la met dans une fenêtre

extras-antenna = ANTENNE

extras-antenna-hint = ANTENNE (expérimental/bêta) — longueurs de coupe, impédance au point d'alimentation et diagramme pour les antennes filaires, verticales et Yagi courantes

recjobs-title = ENREGISTREMENTS

recjobs-finished = « { $name } » terminé

recjobs-skipped = « { $name } » ignoré — enregistrement déjà en cours

recjobs-recording-name = Enregistrement de « { $name } »

recjobs-new-job = + NOUVELLE TÂCHE

recjobs-recording = ● enregistrement

recjobs-none = Aucun enregistrement programmé — appuyez sur + NOUVELLE TÂCHE pour en ajouter un.

recjobs-done = terminé

recjobs-edit = modifier

recjobs-del = suppr.

recjobs-station = Station

recjobs-station-hint = BBC World Service

recjobs-kind = Type

recjobs-start-utc = Début (UTC)

recjobs-for = Durée

recjobs-save = ENREGISTRER

recjobs-kind-audio = Audio

recjobs-kind-iq = I/Q

recjobs-kind-both = Les deux

panels-sensitivity = Sensibilité

navtex-hunting = recherche

navtex-hunting-hint =
    Rien n'est encore lu.
    
    C'est presque toujours une question d'accord. Ne tapez PAS le numéro annoncé (518) dans la case de fréquence : NAVTEX est indiqué par canal, et ce programme écoute juste en dessous. Cliquez plutôt sur le bouton 518 ci-dessus — il règle le bon cadran pour vous (516,300).
    
    Si un signal fort est clairement présent mais que le texte reste incompréhensible, appuyez sur REV : il arrive à l'envers.
    
    N'oubliez pas non plus que NAVTEX suit un horaire, ce n'est pas une conversation : une station côtière émet dix minutes toutes les quatre heures, le silence est donc normal la plupart du temps. Il faut écouter au moment où votre station doit émettre. Et le 518 kHz ne s'anime que la nuit — une bande silencieuse de jour n'est pas un décodeur en panne.

navtex-sensitivity-hint =
    Sensibilité — la facilité avec laquelle le décodeur s'accroche à un signal et le garde.
    
    Auto utilise les seuils de référence (0), le comportement livré d'origine.
    
    + accepte un signal plus faible ou plus bruité ; − attend un signal plus propre avant de déclarer la synchronisation, ce qui évite de lire du bruit comme des caractères.
    
    Ce n'est pas le volume : NAVTEX est de la FSK, les gris viennent des tons et non de la force du signal.


navtex-auto-hint = Revient aux seuils de référence (0). Mis en évidence quand actif.

navtex-tune-dial-usb-below-channel = Accorde le cadran ici (USB, 1,710 kHz sous le canal).

settings-audio-your-audio = Votre audio

settings-audio-your-audio-why = Les haut-parleurs sur lesquels vous écoutez et le microphone par lequel part votre voix. Ils appartiennent à cette machine et non à une radio : ils restent les mêmes quelle que soit la radio utilisée.

settings-audio-browser-note = Ici, les périphériques audio se choisissent dans le navigateur, pas dans cette fenêtre : la page demande au navigateur les haut-parleurs et le microphone. Les réglages de carte son de votre station, plus bas sur cette page, s'appliquent toujours.

settings-audio-speakers = Haut-parleurs

settings-audio-channels-hint = Quel(s) canal(aux) des haut-parleurs reçoivent l'audio reçu. Gauche ou droite coupe l'autre, pour n'écouter que d'un côté.

settings-audio-microphone-note = Le microphone porte votre voix à l'émission quand le poste est commandé par le son (MCW) ou par VOX. Si votre poste se met en émission via CAT, le microphone n'est pas du tout utilisé pour émettre et cette ligne ne change rien.

settings-audio-stream-out = Diffuser l'audio vers l'extérieur

settings-audio-stream-out-why = Envoie ce que reçoit cette radio vers un autre programme par le réseau (décodeur, enregistreur, deuxième machine) sans passer par une carte son.

settings-audio-send-to-hint =
    `hôte:port`. Mono, 16 bits, à la cadence de réception, un datagramme par bloc audio. C'est l'audio **pris à l'enregistreur** : le volume AF, MUTE et l'émission n'ont aucun effet, car un décodeur externe veut le signal de la radio et non le niveau de votre casque.
    
    Une adresse numérique, pas un nom d'hôte. Vide : désactivé.

settings-audio-send-to-note = `127.0.0.1:7355` l'envoie à un autre programme de cet ordinateur ; l'adresse propre de cette machine l'envoie sur le réseau.

settings-audio-radio-audio = Audio de cette radio

settings-audio-radio-audio-why = La carte son entre cette radio et cet ordinateur. Elle appartient à la radio et non à la machine : deux radios peuvent partager l'ordinateur et avoir besoin de cartes différentes, donc ces réglages restent liés à la radio depuis laquelle vous avez ouvert les Réglages.

settings-audio-no-cards-yet = Cette machine n'a pas encore signalé ses cartes son. Vous pouvez quand même saisir les noms ci-dessous ; une liste apparaît dès que les périphériques ont été interrogés.

settings-audio-rescan = Réanalyser les cartes son

settings-audio-rescan-hint =
    Demande à nouveau à la machine quelles cartes son elle possède. Rien n'est ouvert, on peut donc l'utiliser sans risque pendant la réception.
    
    À utiliser après avoir branché un codec. Cela ne rouvre pas la radio (c'est Appliquer / reconnecter dans l'onglet Radio) et ne peut pas réparer une carte renommée par le serveur audio, ce qui est signalé sur la radio.

settings-audio-from-radio = Depuis la radio

settings-audio-to-radio = Vers la radio

settings-audio-radio-note = { "*" }*Depuis la radio** est ce que votre poste envoie : une sortie audio USB, ou la prise casque d'un récepteur vers l'entrée ligne d'une carte son. **Vers la radio** est ce que le poste écoute pour votre voix ; c'est aussi ainsi que le programme commande un poste par le son : laissez vide et le microphone du poste est utilisé, s'il en a un.

settings-audio-recording = Enregistrement

settings-audio-recording-why = Où sont écrits les enregistrements audio, les enregistrements planifiés et les captures I/Q brutes.

enigma-need-more = donnez-lui plus de texte chiffré à traiter

enigma-title = ENIGMA

enigma-no-cables = aucun câble

enigma-thin = fine

enigma-rotor = Rotor

enigma-window-hint = Grundstellung — cliquez pour avancer la roue

enigma-input-hint = tapez, ou utilisez les touches ci-dessus

enigma-clear = EFFACER

enigma-copy = COPIER ⇄

enigma-solve-note = LÖSEN — collez un texte chiffré. Donnez un mot probable (crib) attendu dans le message pour un texte câblé ; sans lui, le solveur traite une machine sans câbles.

enigma-crib = Crib

enigma-solve = RÉSOUDRE

enigma-flaw = Une lettre ne se chiffre jamais en elle-même — le réflecteur y veille. C'est le défaut de la vraie machine, et c'est sur lui que le solveur élague.

settings-audio-takes-over-speakers = { "*" }*Ces haut-parleurs sont un périphérique du serveur audio, pas une carte son.** L'utiliser en fait aussi le périphérique de lecture de *tout le reste* de cet ordinateur : les autres programmes joueront eux aussi par là. PipeWire ne permet pas à un programme de choisir sa propre cible de lecture ici, c'est donc inévitable. Pour y remédier : choisissez une carte son, ou rétablissez le périphérique par défaut de la machine (`wpctl set-default <nom>`) puis relancez la recherche.

settings-audio-takes-over-microphone = { "*" }*Ce microphone est un périphérique du serveur audio, pas une carte son.** L'utiliser en fait aussi le périphérique d'enregistrement de *tout le reste* de cet ordinateur : les autres programmes enregistreront eux aussi depuis lui. PipeWire ne permet pas à un programme de choisir sa propre source de capture, c'est donc inévitable. Pour y remédier : choisissez une carte son, ou rétablissez le périphérique par défaut de la machine (`wpctl set-default <nom>`) puis relancez la recherche.

settings-audio-currently-chosen = ⚠  Actuellement choisi : { $device }

knowncalls-no-table = Ce mode ne tient pas de table d'indicatifs. C'est un décodeur FT8/FT4 qui hache les indicatifs — tout autre mode n'a rien à afficher ici.

knowncalls-not-asked = Pas encore demandé — appuyez sur ACTUALISER.

knowncalls-nobody = Personne pour l'instant. Une station est ajoutée dès qu'on l'entend épeler son indicatif — un CQ, un rapport avec les deux indicatifs en clair, ou un simple appel en 11 m.

knowncalls-title = STATIONS CONNUES

knowncalls-intro = Chaque indicatif que peut actuellement nommer un <...> haché sur ce récepteur. Un hash ne se lit pas à l'envers — une station n'apparaît ici qu'une fois entendue épelant son indicatif, ce à quoi sert un simple appel en 11 m.

knowncalls-find-hint = indicatif, pays, préfixe…

knowncalls-refresh = ACTUALISER

knowncalls-capped = les { $shown } plus récents sur { $total } — ACTUALISER pour redemander, la liste est un instantané

knowncalls-known = { $shown } connus

knowncalls-matches =
    { $n } { $n ->
        [one] résultat
       *[other] résultats
    }

knowncalls-no-match = Aucune station connue ne correspond.

knowncalls-no-country = aucun pays — pas un indicatif que cette table connaît

classic-controls = COMMANDES

classic-fold-the-column-away-to-a-rail-on = Replier la colonne en un rail à gauche. L'indicateur de fréquence et le S-mètre restent dans la barre, et la commande qui la rouvre se trouve sur le rail.

classic-click-to-open-or-close-this = Cliquer pour ouvrir ou fermer cette section. La faire glisser sur une autre section pour la déplacer — l'ordre est mémorisé.

classic-bring-the-control-column-back = Ramener la colonne de commandes.

classic-no-radio = aucune radio

classic-rate = Débit

classic-dial = Fréquence

classic-receive = Réception

classic-listening-screen = écran d'écoute

classic-radio-settings = RÉGLAGES RADIO

classic-the-backend-the-sample-rate-the = Le backend, le taux d'échantillonnage, l'antenne, le convertisseur et l'étalonnage propre à cette radio — Réglages → Radio, où ils ont toujours été. La colonne montre leurs valeurs au lieu d'une seconde copie des commandes.

classic-counted-in-steps-not-decibels-what = Compté en pas, pas en décibels : la valeur d'un pas dépend du récepteur, et sur un SDRplay elle dépend aussi de la bande. Vers la droite, c'est toujours plus de gain.

classic-the-plutosdr-s-own-agc-is-setting = Le CAG propre du PlutoSDR règle ce gain, donc déplacer le curseur ne change rien. Réglez Réglages → Radio → CAG sur Manuel pour le commander ici.

classic-squelch = Squelch

classic-gate-the-audio-below-this-power-in = Coupe l'audio sous cette puissance dans la bande passante de réception, en dBFS. À gauche, c'est ouvert. Réglez au-dessus du bruit et sous le signal.

classic-this-radio-has-not-reported-what = Cette radio n'a pas encore indiqué ce qu'elle sait faire — sa liaison est fermée ou elle n'a pas répondu.

classic-open-the-link-in-the-radio-strip = Ouvrez la liaison dans la barre radio et les commandes d'émission apparaissent ici.

classic-this-device-receives-only-it-has = Cet appareil ne fait que recevoir : il n'a pas d'émetteur que ce programme puisse commander.

classic-transmit-is-offered-by-a-rig-with = L'émission est proposée par un équipement avec une voie de commande ou audio — Réglages → Radio.

classic-transmit-is-off-for-this-radio-s = L'émission est désactivée pour l'écran de cette radio.

classic-settings-radio-show-transmit = Réglages → Radio → « Afficher les commandes d'émission » la rétablit.

classic-key = Manip.

classic-controlled-envelope-ssb-how-much = SSB à enveloppe contrôlée : quelle part du traitement de la parole appliquer. Plus haut = plus fort et plus plat, au prix d'un peu d'étalement de part et d'autre.

classic-holding-the-squelch-open-click-to = Squelch maintenu ouvert. Cliquer pour rétablir le seuil précédent.

classic-click-to-hold-the-squelch-open-to = Cliquer pour maintenir le squelch ouvert — pour entendre si quelque chose est là — et cliquer à nouveau pour le remettre exactement comme avant.

panels-received = Reçu

panels-text = Texte

panels-hex = Hex

panels-nothing-received-yet = Rien reçu pour l'instant.

panels-payload = Charge utile

panels-blocks = Blocs

panels-sequence = Séquence

uvpacket-starting-receiver = démarrage du récepteur UVPacket…

uvpacket-frames-count = { $n } trames

uvpacket-frames-count-hint = Trames décodées depuis le démarrage du récepteur.

uvpacket-intro = Un mode paquet pour groupes privés VHF/UHF. Réglez le cadran sur le canal du groupe ; le sous-mode est détecté à partir de chaque trame.

uvpacket-nothing-yet = Rien pour l'instant. UVPacket est silencieux entre les émissions — laissez-le sur un canal et attendez une salve.

uvpacket-app-seq = app { $app } · séq { $seq }

uvpacket-click-read-frame = Cliquez pour lire la trame entière.

uvpacket-bps-net = { $bps } bit/s net

uvpacket-sub-mode = Sous-mode

uvpacket-app-type = Type d'app

uvpacket-payload-bytes = { $n } octets

dsc-starting-receiver = démarrage du récepteur DSC…

dsc-seq-tone = { $n } séq · tons { $pct } %

dsc-seq-tone-hint = Séquences ayant atteint leur marqueur de fin, et netteté de la séparation des deux tons.

dsc-nothing-yet = Rien pour l'instant. DSC est silencieux entre les navires — laissez-le sur un canal et attendez une salve.

panels-marginal = limite

dsc-marginal-hint = Un caractère a échoué au contrôle BCH — les champs affichés peuvent être erronés.

dsc-click-read-sequence = Cliquez pour lire toute la séquence.

panels-marginal-caps = LIMITE

dsc-marginal-caps-hint = Au moins un caractère a échoué au contrôle BCH. La séquence est affichée car une alerte de détresse reçue imparfaitement mérite quand même d'être lue.

dsc-category = Catégorie

dsc-nature = Nature

dsc-caller-mmsi = MMSI appelant

dsc-target-mmsi = MMSI destinataire

dsc-alert-time = Heure d'alerte

dsc-working-ch = Canal trafic

dsc-raw-symbols = Symboles bruts

dsc-none = (aucun)

settings-general-device-type-name = …ou saisissez le nom — vide : périphérique par défaut du système

settings-general-signed-in-station = Station connectée

settings-general-sign-out-hint = Oublie la connexion de cet appareil. La station devra redemander l'identification à la prochaine connexion ; celle-ci reste ouverte jusqu'à sa fermeture.

settings-general-recordings-remote = Les enregistrements sont écrits sur la machine à laquelle la radio est reliée. Définissez le dossier là-bas.

settings-general-folder = Dossier

settings-general-choose = CHOISIR…

settings-general-choose-hint = Choisir le dossier où sont écrits les enregistrements

settings-general-default = DÉFAUT

settings-general-default-hint = Revenir au dossier par défaut : votre dossier Musique, ou le répertoire de configuration si le système n'en a pas

settings-general-recordings-dir-note = Où sont écrits les enregistrements audio, les enregistrements planifiés et les captures I/Q brutes. Par défaut : le sous-dossier sdroxide de votre dossier Musique, ou le dossier recordings du répertoire de configuration si le système n'expose pas de dossier Musique.

setup-suffix-cq = { " " }CQ

setup-suffix-tries = { " " }essais

setup-window = fenêtre

setup-south = sud

setup-west = ouest

setup-message-templates = Modèles de messages  { $tokens }

setup-ft8-decoder-effort-hint =
    Effort du décodeur FT8 sur les signaux faibles.
    
    Fast — une seule passe, sans soustraction de signal : le plus rapide, le moins de décodages.
    Normal — soustraction multi-passes simple : ~0,4 s sur un créneau chargé, quelques décodages de moins que Deep.
    Deep — la passe à points de reprise de WSJT-X : ~1,2 s et le plus de décodages (par défaut).
    
    Seul le second lot de décodages est concerné ; la passe simple est toujours émise en premier, une réponse n'est donc jamais retardée par ce réglage.

setup-auto-cq-repeats-hint = Nombre d'appels CQ qu'une session effectue avant de s'arrêter. 0 = pas de limite propre : le chien de garde d'émission et l'arrêt sur inactivité bornent quand même la session.

setup-auto-cq-then-listen-hint = Après ces appels CQ, passer d'AUTO CQ à AUTO RX et continuer à écouter plutôt que de tout arrêter. Désactivé, la session s'arrête.

setup-auto-call-retries-hint = Nombre de fois qu'une station est appelée avant que la session y renonce et passe à la suivante dans la file.

setup-auto-session-hint = Durée maximale d'une session sans surveillance, quoi qu'elle soit en train de faire. 0 = pas de limite propre.

setup-serial-contests-hint = Concours à numéro de série — ARRL RTTY Roundup et CQ WPX en FT8 : un report et un numéro de série (ou un État américain), sans locator. Le CQ part sous la forme CQ RU.

setup-tx-level-fm-hint =
    Niveau auquel la salve de ce mode est remise à un émetteur-récepteur qui la module lui-même — et en FM c'est l'excursion, car un émetteur FM transforme le niveau audio en excursion de fréquence et n'a pas d'ALC pour l'arrêter. Le packet 1200 bauds demande environ 3 kHz là où la voix en demande 5 : à pleine échelle sur une entrée données réglée pour la voix, on sur-dévie — ce qui paraît parfaitement normal à l'écoute et ne se décode chez personne. Baissez-le jusqu'à ce que d'autres stations vous reçoivent, ou réglez le niveau sur la radio.
    
    Ce niveau appartient à ce mode seul. Un mode jamais réglé part de la valeur FM, et les modes en bande latérale partent de la leur — une excursion réglée pour le 1200 bauds n'arrive donc jamais sur le FT8.

setup-tx-level-ssb-hint =
    Niveau auquel la transmission de ce mode est remise à un émetteur-récepteur qui la module lui-même — un poste CAT sur sa carte son, un FLEX, un Icom sur son port réseau. En bande latérale c'est l'excitation du modulateur : baissez-la jusqu'à ce que l'ALC du poste bouge à peine et réglez la puissance sur la radio, car une ALC qui travaille sur un mode à enveloppe constante est ce qui provoque les splatters. L'excitation atteint ici le registre de puissance du poste, pas son audio : c'est donc bien le niveau.
    
    Ce niveau appartient à ce mode seul : FT8, RTTY, PSK et MCW gardent chacun le leur, car ils ne chargent pas un modulateur de la même façon. Le même curseur se trouve sur la barre d'émission, accessible pendant que vous émettez.

setup-repeats-hint-base = Appels sans réponse vers une même station avant de passer à la suivante. Les appels CQ sont exclus.

setup-repeats-hint-duration = { " " }Avec la durée de créneau du { $mode } ({ $slot } s), cela fait environ { $mins } min.

setup-repeats-hint-disables = { " " }0 le désactive.

setup-repeats-hint-11m = { " " }Sur 11 m, c'est aussi ce qui met fin à une série de CQ : le chien de garde d'émission n'en coupe pas, car une station qui répond à un CQ est attendue en une trentaine de secondes environ et une horloge ne distingue pas une série récente d'une série périmée.

setup-symbol-home-station = Station fixe

setup-symbol-car = Voiture

setup-symbol-truck = Camion

setup-symbol-van = Fourgonnette

setup-symbol-motorcycle = Moto

setup-symbol-bicycle = Vélo

setup-symbol-person-on-foot = Personne à pied

setup-symbol-motorhome = Camping-car

setup-symbol-boat = Bateau

setup-symbol-yacht = Yacht

setup-symbol-light-aircraft = Avion léger

setup-symbol-balloon = Ballon

setup-symbol-digipeater = Digipeater

setup-symbol-i-gate = I-gate

setup-symbol-repeater = Relais

setup-symbol-weather-station = Station météo

setup-symbol-hospital = Hôpital

setup-symbol-portable-campsite = Portable / camping

settings-general-sign-out = DÉCONNEXION

settings-general-fallback-password = Mot de passe mémorisé dans ce navigateur pour { $stations }, car la station n'a pas accepté de cookie de connexion. Les stations plus anciennes procèdent ainsi ; ce n'est pas la même chose.

settings-general-device-hint-speakers = Le nom exact sous lequel vos haut-parleurs sont ouverts. Choisissez-le dans la liste s'il y figure ; saisissez-le sinon : un serveur audio peut proposer un périphérique que l'API hôte ne liste pas. Vide : périphérique par défaut du système.

settings-general-device-hint-microphone = Le nom exact sous lequel votre microphone est ouvert. Choisissez-le dans la liste s'il y figure ; saisissez-le sinon : un serveur audio peut proposer un périphérique que l'API hôte ne liste pas. Vide : périphérique par défaut du système.

settings-general-device-hint-radio-rx = Le nom exact sous lequel l'audio de réception de cette radio est ouvert. Choisissez-le dans la liste s'il y figure ; saisissez-le sinon : un serveur audio peut proposer un périphérique que l'API hôte ne liste pas. Vide : périphérique par défaut du système.

settings-general-device-hint-radio-tx = Le nom exact sous lequel l'audio d'émission de cette radio est ouvert. Choisissez-le dans la liste s'il y figure ; saisissez-le sinon : un serveur audio peut proposer un périphérique que l'API hôte ne liste pas. Vide : périphérique par défaut du système.

settings-general-device-hint-other = Le nom exact sous lequel le périphérique est ouvert. Choisissez-le dans la liste s'il y figure ; saisissez-le sinon : un serveur audio peut proposer un périphérique que l'API hôte ne liste pas. Vide : périphérique par défaut du système.

settings-general-cb-world = Monde / bande libre — 80 canaux

settings-general-cb-cept = CEPT / UE — 40 canaux

settings-general-cb-germany = Allemagne — 80 canaux

settings-general-cb-uk = Royaume-Uni — 27/81, 40 canaux

settings-general-cb-us = États-Unis — 40 canaux

settings-general-cb-australia = Australie — 40 canaux

settings-general-cb-pmr446 = PMR446 — 16 canaux (446 MHz)

settings-general-cb-dpmr446 = dPMR446 — 32 canaux (446 MHz)

wefax-receiving-progress = réception { $min } min · { $lines } lignes

wefax-listening = écoute

wefax-status-hint =
    En attente d'une carte.
    
    Le WEFAX suit un horaire, ce n'est pas une conversation : une station émet ses cartes à heures fixes, une fréquence choisie est donc silencieuse la plupart de la journée — c'est normal. Utilisez le bouton STATIONS pour en choisir une et vérifier l'horaire.
    
    Si un signal est présent mais que l'image sort d'un seul ton uniforme (tout gris, tout noir ou tout blanc), c'est l'accord ou la vitesse — utilisez l'indicateur +0 Hz et le réglage IOC/vitesse, et vérifiez que l'entrée STATIONS correspond à ce qui est émis.

wefax-centre = CENTRER

wefax-centre-hint = Décale le cadran de { $sign }{ $off } Hz pour que la sous-porteuse retombe sur 1900 Hz — le récepteur est décalé de { $off } Hz, ce qui transforme la carte en page blanche.

wefax-auto-start-hint = Démarre une carte à l'écoute du ton de début de 300 Hz (IOC 576) ou 675 Hz, ou, si le ton s'est perdu dans un évanouissement ou sous une porteuse, quand quelques secondes de signal de phasage arrivent — et termine la carte en cours pour phaser la nouvelle à l'arrivée de l'émission suivante, qu'un ton d'arrêt soit venu ou non. Une porteuse sur le canal ne déclenche rien.

wefax-fit = AJUST.

wefax-save-chart-hint = Enregistre sur cet ordinateur la carte affichée, dans l'état où elle est arrivée

wefax-waiting-for-chart = Accordez un horaire fax en USB et attendez le début de la carte (ton de début ou signal de phasage), ou appuyez sur START pour commencer en cours de carte.

wefax-carrier-dial-note = porteuse { $carrier } · cadran { $dial } kHz (USB −1,9k)

wefax-carrier-note-hint = La puce de la station indique la porteuse publiée ; la sous-porteuse radiofax se situe 1900 Hz au-dessus, le cadran est donc volontairement 1,9 kHz plus bas. Accordez le cadran sur la porteuse elle-même et le signal tombe hors de la bande passante.

contest-title = CONCOURS

contest-intro = Choisissez un concours et saisissez ce que vous envoyez. Le journal fonctionne dans tous les modes — tapez l'échange en CW ou en SSB, et le côté FT8 le remplit tout seul lorsque le concours a un format FT8.

contest-you-send = { "  " }vous envoyez : { $exchange }

contest-they-send = { "  " }ils envoient : { $exchange }

contest-cb-note = Activité CB / 11 m : un report et un échange en texte libre — mettez-y le canal, le nom ou la zone qu'utilise l'activité.

contest-your-exchange = Votre échange

contest-start = DÉMARRER

contest-score-line = QSO { $qsos } · { $points } pts · { $mults } mult · SCORE { $total }

contest-rate = { $r10 } en 10 min · { $r60 }/h

contest-stop = ARRÊTER

contest-estimate = le score est une estimation — l'organisateur arbitre

contest-call = INDICATIF

contest-dupe = DOUBLON

contest-sent = ENVOYÉ

contest-rcvd = REÇU

contest-sending = envoi de { $rst }

contest-sending-ours = envoi de { $rst } { $ours }

contest-log = ENREG.

playback-radio-n = { " " }· radio { $n }

playback-no-stamp = pas d'horodatage dans le nom — lu dans le mode actuel

playback-title = LECTURE — audio enregistré

playback-find-hint = mode, MHz, date, nom…

playback-refresh-hint = Relire le dossier des enregistrements

playback-asking = Demande des enregistrements à la station…

playback-folder = Dossier : { $dir }

playback-none = Aucun enregistrement pour l'instant. Appuyez sur REC → MP3 ou WAV pendant qu'une émission passe ; le fichier apparaît ici, nommé d'après son heure, sa fréquence et son mode.

playback-on = ▶ EN COURS

playback-play = ▶ LIRE

playback-iq-hint = Une capture I/Q, pas de l'audio. La lecture prend l'audio écrit par l'enregistreur ; une capture I/Q se lit avec --file.

playback-not-while-tx = Impossible pendant l'émission

playback-play-hint = Lire cet enregistrement vers les décodeurs depuis le début

playback-busy-playing = En lecture — arrêtez-la avant de la supprimer

playback-busy-recording = Encore en cours d'enregistrement — arrêtez REC avant de la supprimer

playback-delete-ask = SUPPRIMER ?

playback-delete-hint-armed = Appuyez à nouveau pour supprimer ce fichier définitivement. Cette action est irréversible.

playback-delete-hint = Supprimer cet enregistrement du dossier (demande confirmation d'abord)

playback-strip-label = ▶ LECTURE

playback-not-live = pas en direct — la radio est à l'arrêt

playback-carry-on = Reprendre la lecture

playback-pause = ⏸ PAUSE

playback-pause-hint = Pause — les décodeurs n'entendent que du silence jusqu'à la reprise

playback-again = ↺ ENCORE

playback-again-hint = Retour au début — un nouvel essai pour une image ou un décodage

playback-loop = BOUCLE

playback-loop-hint = Reprendre au début à la fin du fichier

playback-stop = ■ ARRÊT

playback-stop-hint = Terminer la lecture et remettre la radio comme elle était

playback-files = FICHIERS

playback-end-of-file = fin du fichier

playback-paused = en pause

playback-recorded-on-mode = Enregistré sur { $mhz } MHz en { $mode } — la fréquence et le mode ont été réglés d'après le nom du fichier.

playback-recorded-on = Enregistré sur { $mhz } MHz — la fréquence et le mode ont été réglés d'après le nom du fichier.

playback-no-dial-mode = Le nom n'indique ni fréquence ni mode ; la lecture se fait donc dans le mode actuel — choisissez le décodeur dans le menu des modes.

playback-format-stereo = { $rate } Hz, stéréo — le canal gauche (réception) est lu.

playback-format-mono = { $rate } Hz, mono — l'unique canal est lu.

playback-stopped = Arrêté : { $fault }

playback-intro = Rejouez un enregistrement vers les décodeurs — SSTV, CW, RTTY, PSK, FT8 et tous les autres modes qui décodent à partir de l'audio. La radio se met en retrait pendant la lecture, la fréquence et le mode passent à ceux de l'enregistrement, et ARRÊT remet la radio en place. Changez de mode pendant la lecture pour essayer un autre décodeur.

packet-callsign-hint = indicatif

packet-via-hint = via

aprs-filter-hint = filtre

aprs-msg-sent = envoyé

aprs-msg-ack = reçu

aprs-msg-rejected = refusé

aprs-reply-chip = répondre

aprs-to-hint = à

aprs-message-hint = message

aprs-raw-chip = BRUT

fsq-to-button = À

fsq-del-chip = SUPPR

macros-column-key = touche

macros-column-button = bouton

macros-column-sends = envoie

macros-label-hint = libellé

macros-add = AJOUTER

macros-note-placeholders = { $mycall } et { $mygrid } sont remplacés au moment de l'envoi du message.

macros-buttons-note = Chaque bouton envoie tout son texte d'un coup. F1–F9 actionnent les neuf premiers, tant que rien à l'écran n'a le clavier.



widgets-worked-before-any = Déjà contacté

vdl2-col-sig = SIG

vdl2-col-age = ÂGE

vdl2-command = commande

vdl2-response = réponse

vdl2-frame-hint = trame

vdl2-suffix-messages = { " " }messages

vdl2-suffix-stations = { " " }stations

ale-words-count = { $n } mots

ale-waiting-audio = En attente de l'audio…

ale-no-words-yet = Aucun mot pour l'instant. Accordez-vous sur un canal ALE (p. ex. 8992 ou 11175 kHz USB) et attendez un appel.

atchat-role-master = MAÎTRE

atchat-role-backup = SECOURS

atchat-me = moi

dab-sub-channel = { " " }· sous-canal { $n }

dab-starting-receiver = démarrage du récepteur DAB…

dab-no-ensemble-yet = aucun multiplex identifié pour l'instant — y a-t-il une émission DAB sur ce canal ?

dab-window-stats = { $mhz } MHz · { $frames } trames · { $fibs } FIB

dab-volume-meter-hint = La station telle que vous l'entendez, gauche et droite — et le volume : faites glisser ou cliquez le long des barres, ou utilisez la molette. Cliquez dessus, puis les flèches font varier le volume par pas. Fonctionne sur un écran verrouillé avec le cadenas.

rade-searching = recherche

rfpaint-swl-note = RF Paint est un mode d'émission seule — rien à afficher en SWL.

cw-hand-key-arm-hint = Armez le manipulateur et utilisez le manipulateur USB. Les contacts vont au moteur, qui fabrique les points et les traits à la vitesse d'émission — ici, pas dans le panneau, donc le timing n'est pas arrondi à une image. La zone d'émission est verrouillée tant que c'est activé et le texte en attente est supprimé. Une clé droite sur la même prise fonctionne aussi : choisissez le type Clé droite.

cw-own-keyer-hint =
    Cette radio émet depuis son propre manipulateur : le texte passe par le port de commande et la radio cadence les éléments, il n'y a donc rien entre la main et l'antenne qu'une clé puisse piloter — pas plus un manipulateur à palettes qu'une clé droite.
    
    Pour manipuler à la main, réglez la manipulation CW sur « Carte son (MCW) » dans Réglages → Radio. La radio reste alors en bande latérale et la note de contrôle du manipulateur est transmise en audio, ce qui est la voie qu'emprunte la clé droite.

cw-no-paddle-found = aucun manipulateur trouvé — choisissez-en un dans Réglages → CW

cw-msg-hint = Vos propres boutons de message — un échange de concours, une réponse nom-et-QTH, TNX 73 GL. Chacun envoie son texte entier d'un coup, et F1–F9 actionnent les neuf premiers. Ils font partie de la configuration de la station, un client distant les a donc aussi.

wspr-clear-rx = EFFACER RX

wspr-rx-count = { $n } rx

wspr-sort-dist = DIST

wspr-sort-country = PAYS

wspr-tx-enable = ACTIVER TX

wspr-stop-tx = ARRÊT TX

wspr-listening-as = À l'écoute en tant que { $call } en { $sent } — défini dans l'onglet Général des Réglages.

wspr-set-callsign-grid-label-spots = Indiquez votre indicatif et votre locator dans l'onglet Général des Réglages pour identifier vos spots.

schedule-source-mine = les miennes

schedule-all-of-them = toutes

schedule-title = PROGRAMMES

schedule-at = À

schedule-now-chip = MAINTENANT

schedule-set-time = fixer l'heure

schedule-find-hint = BBC, 6185, 15,4 MHz…

schedule-target = Cible

schedule-any = toutes

schedule-source = Source

schedule-all-hint = Tous les horaires téléchargés dans un seul stock. Une station listée par deux d'entre eux apparaît deux fois, ce qui est précisément l'intérêt de cette fenêtre — le panadapter n'affiche qu'un seul nom.

schedule-favs = ★ FAVORIS

schedule-favs-hint = Seulement les stations que vous avez marquées d'une étoile

schedule-all-favs = ⭐ TOUT

schedule-all-favs-hint = Mettre en favori toutes les stations de l'horaire et afficher leurs étiquettes sur la cascade tant que c'est activé. Cliquez à nouveau pour effacer tous les favoris.

schedule-solar-time = HEURE LOCALE

schedule-solar-time-hint = Afficher pour chaque ligne l'heure locale à l'émetteur. C'est l'heure du Soleil — quatre minutes par degré de longitude — et non un fuseau horaire civil : elle ignore l'heure d'été et les frontières de fuseau.

schedule-map = VOIR CARTE

schedule-map-hint = Ouvrir ÉCOUTE — cet horaire sous forme de carte. Choisissez-y une station, notez ce que vous avez entendu et enregistrez-le. La station sélectionnée est reprise

schedule-on-air = { $count } en émission à { $time } UTC

schedule-now = maintenant

schedule-solar-hover = { $local } heure solaire locale à { $site }, { $lon }°

schedule-favourite-hint = Mettre cette station en favori

rc-radio-classic = RADIO CLASSIC

rc-bands = Bandes

rc-mem-programs = Mém. · Programmes

rc-recall-band-mode-frequency-and = Rappel : bande, mode, fréquence et filtre en une fois

rc-no-memories-yet-store-one-with-mem = aucune mémoire — en enregistrer une avec MEM dans la barre du haut

rc-broadcast-schedule-what-is-on-when = Grille des programmes — ce qui passe, quand et où

rc-bw-this-mode-has-one-fixed = BW — ce mode a une bande passante fixe

rc-bw-click-next-filter-right-click = BW — clic : filtre suivant · clic droit : précédent · molette : pas

rc-step = PAS

rc-step-click-next-tuning-step-right = PAS — clic : pas d'accord suivant · clic droit : précédent · molette : pas

rc-agc-click-next-speed-right-click = AGC — clic : vitesse suivante · clic droit : précédente · molette : pas

rc-open = OUVERT

rc-sql-click-open-back-to-the-last = SQL — clic : ouvrir / revenir au dernier seuil · molette : monter ou baisser le seuil

rc-s-meter = S-MÈTRE

rc-wheel-on-a-digit-click-upper-lower = molette sur un chiffre · clic moitié haute/basse ± · clic droit remet à zéro · double-clic pour saisir

rc-music = MUSIQUE ♪

rc-speech = PAROLE

rc-stereo-the-19-khz-pilot-is-locked = Stéréo — la sous-porteuse pilote 19 kHz est verrouillée — cliquer pour passer en mono

rc-mono-the-broadcast-stereo-decode = Mono — le décodage stéréo de la radiodiffusion est désactivé — cliquer pour l'activer

rc-mono-noise-reduction-is-on-and = Mono — la réduction de bruit est active et la stéréo lui cède la place

rc-mono-no-19-khz-pilot-the-station = Mono — pas de pilote 19 kHz : la station émet en mono ou le signal est trop faible

rc-stereo = STÉRÉO

rc-group-broadcast = DIFFUSION

rc-group-ham = AMATEUR

rc-group-air = AIR

rc-group-marine = MARINE

rc-cap-lw-mw-fm = GO / PO · FM

rc-cap-shortwave = OC · ONDES COURTES

rc-cap-amateur-bands = BANDES AMATEUR

rc-cap-airband = BANDE AÉRONAUTIQUE

rc-cap-marine = VHF MARINE · MF

rc-cap-cb = BANDE CITIZEN (CB)

rc-preset-fm-ext = FM étendue 64–87.5

rc-mem-n = MÉM { $id }

controls-action-tune = Accorder le VFO

controls-action-rit-offset = Décalage RIT

controls-action-xit-offset = Décalage XIT

controls-action-volume = Volume

controls-action-sub-volume = Volume du sous-RX

controls-action-squelch = Squelch

controls-action-filter-width = Largeur du filtre

controls-action-filter-shift = Décalage du filtre

controls-action-agc-max-gain = Gain max. de l'AGC

controls-action-manual-gain = Gain manuel (AGC désactivé)

controls-action-tx-drive = Puissance d'excitation TX

controls-action-tune-drive = Excitation d'accord

controls-action-mic-gain = Gain micro

controls-action-digi-audio-freq = Décalage audio numérique

controls-action-spectrum-zoom = Zoom du spectre

controls-action-spectrum-pan = Défilement du spectre

controls-action-spectrum-floor = Plancher du spectre

controls-action-spectrum-ceiling = Plafond du spectre

controls-action-ptt = PTT

controls-action-tune-carrier = Porteuse TUNE

controls-action-mute = Silence

controls-action-noise-blanker = Réducteur d'impulsions

controls-action-nr-cycle = Réduction de bruit (cycle)

controls-action-auto-notch = Notch automatique

controls-action-binaural = Audio binaural

controls-action-agc-cycle = AGC (cycle)

controls-action-sub-rx = Sous-récepteur

controls-action-split = Split

controls-action-rit-enable = RIT activé/désactivé

controls-action-xit-enable = XIT activé/désactivé

controls-action-rit-clear = Effacer le RIT

controls-action-xit-clear = Effacer le XIT

controls-action-vfo-toggle = Bascule VFO A/B

controls-action-swap-vfos = Échanger A↔B

controls-action-copy-a-to-b = Copier A→B

controls-action-band-up = Bande suivante

controls-action-band-down = Bande précédente

controls-action-mode-next = Mode suivant

controls-action-mode-prev = Mode précédent

controls-action-record-toggle = Enregistrement activé/désactivé

controls-action-abort-tx = Interrompre l'émission

controls-action-cw-straight = Manipulateur droit CW

controls-action-voice-stop = Arrêt du keyer vocal

controls-action-tone-burst = Tonalité 1750 Hz

controls-action-fit-span = Ajuster la plage

controls-action-zoom-in = Zoom avant

controls-action-zoom-out = Zoom arrière

controls-action-peak-hold = Maintien des crêtes

controls-action-spectrum-collapse = Replier le spectre

controls-action-waterfall-collapse = Replier la cascade

controls-action-waterfall-flip = Inverser la cascade

controls-action-toggle-help = Fenêtre du manuel

controls-action-toggle-settings = Fenêtre des réglages

controls-action-toggle-logbook = Fenêtre du journal de trafic

controls-action-toggle-spots = Fenêtre des spots

controls-action-toggle-memories = Fenêtre des mémoires

controls-action-toggle-voice = Fenêtre du keyer vocal

controls-action-workspaces = Fenêtre des espaces de travail

controls-action-speak-status = Annoncer l'état

controls-action-speak-repeat = Répéter la dernière annonce

controls-action-speech-silence = Arrêter la voix

controls-action-speech-toggle = Annonces activées/désactivées

controls-action-broadcast-labels = Étiquettes de diffusion (accordée / bande entière)

controls-action-select-vfo = Sélectionner le VFO { $vfo }

controls-action-band = Bande { $band }

controls-action-mode = Mode { $mode }

controls-action-recall-memory = Rappeler la mémoire { $n }

controls-action-voice-slot = Emplacement { $n } du keyer vocal

controls-group-tuning = Accord

controls-group-receive = Réception

controls-group-transmit = Émission

controls-group-display = Affichage

controls-group-windows = Fenêtres

controls-group-speech = Voix

controls-midi-note = Note

controls-midi-cc = CC

controls-midi-pitch-bend = Pitch bend

controls-midi-program = Programme

controls-midi-any = tous

controls-midi-msg = { $kind } { $data }  can. { $ch }

controls-midi-msg-pitch-bend = Pitch bend  can. { $ch }

settings-controls-down = bas

settings-controls-rev = inv.

antenna-antenna = ANTENNE

antenna-cutting-lengths = Longueurs de coupe

antenna-the-frequency-to-cut-for = La fréquence pour laquelle couper

antenna-optimizing = Optimisation…

antenna-tune-the-yagi-s-element-lengths = Ajuste les longueurs et espacements des éléments de la Yagi selon l'objectif choisi. Une montée locale sur un thread d'arrière-plan ; la fenêtre reste utilisable pendant ce temps.

antenna-solved = Résolu

antenna-optimized-yagi = Yagi optimisée

antenna-obstacles-approximate = Obstacles (approximatifs)

antenna-trees = Arbres

antenna-in-leaf = avec feuilles

antenna-ridge = Crête

dab-channel-received-mhz = Canal reçu { $channel } { $mhz }

dab-no-channel-received = Aucun canal reçu

dab-channel-received = Canal reçu { $channel }

dab-slideshow = DIAPORAMA

dab-no-picture-yet = la station n'a pas encore envoyé d'image

dab-no-station-chosen = aucune station choisie

dab-no-logo-yet = pas encore de logo

dab-playing = ▶ en lecture

dab-chosen = choisie

dab-no-text-yet = la station n'a pas encore envoyé de texte

dab-clear-channel-hint = Efface ce canal et ses stations. SCAN le retrouve.

dab-clear-all-channels = × effacer tous les canaux

dab-clear-all-channels-hint = Oublie tous les canaux et les stations qu'ils portent. SCAN les retrouve ; les étoiles restent.

dab-n-stations = { $n } stations

dab-scanning-progress = balayage { $channel } · { $at }/{ $n } · { $found } trouvés

dab-clear-station-hint = Retire cette station de la liste. Le prochain SCAN la ramène si le multiplex la porte encore.

dab-prev = PRÉC

dab-prev-hint = La station précédente de la liste.

dab-preset = FAVORI

dab-preset-hint = Marque cette station d'une étoile, ou retire son étoile.

dab-next = SUIV

dab-next-hint = La station suivante de la liste.

dab-pty-hint = Affiche le type de programme de chaque station dans la liste.

dab-stop-sweep-hint = Arrête le balayage ici. Ce qui a été trouvé jusqu'ici est conservé.

dab-scan-hint = Parcourt chaque bloc de la bande III et conserve ceux qui portent des stations.

swl-title = JOURNAL SWL

swl-new = + NOUVEAU

swl-jobs = TÂCHES

swl-jobs-hint = Enregistrements programmés — enregistrer une bande à heure fixe

swl-sigid = ID SIGNAL

swl-sigid-hint = Qu'y a-t-il sur cette fréquence ? Un guide des signaux par fréquence, mode et largeur de bande

swl-replay = REVOIR

swl-replay-hint = Lire les deux dernières minutes au lieu du direct — rattrapez l'identification de la station que vous venez de manquer

swl-report = RAPPORT

swl-report-hint = Copier cette réception sous forme de rapport à envoyer à la station

swl-stats-heard = { $n } entendues

swl-stats-reported = { $n } rapportées

swl-stats-qsl = { $n } QSL

swl-stats-pirates = { $n } pirates

swl-stats-hint = Le journal entier — les filtres d'affichage ne le modifient pas

swl-antenna = Antenne

swl-antenna-hint = Long fil 20 m, boucle MLA-30, mini-whip …

swl-antenna-tip = Figure sur la ligne Antenna: du rapport de réception

swl-tone = Tonalité

swl-tone-on = actif

swl-tone-bass = Graves

swl-tone-mid = Médiums

swl-tone-treble = Aigus

swl-report-copied = Rapport copié dans le presse-papiers — collez-le dans votre programme de messagerie. Aucun gestionnaire de courrier n'a pu être ouvert.

swl-show = Afficher

swl-find-hint = station, langue, site …

swl-find-tip = Correspond à la station, la langue, au site de l'émetteur et aux notes

swl-all-bands = Toutes les bandes

swl-all-days = Tous les jours

swl-pirates-only = Pirates seulement

swl-pirates-only-tip = Seulement les prises non autorisées

swl-adif-tip = Enregistrer tout le journal en ADIF — des enregistrements de réception, pas des contacts

swl-csv-tip = Enregistrer tout le journal en CSV, une ligne par réception

swl-shown-of = { $shown } sur { $n }

swl-empty = Rien d'enregistré pour l'instant — accordez une station et appuyez sur + NOUVEAU.

swl-transmitted-from = émis depuis { $site }

swl-click-to-copy = (cliquer pour copier)

swl-addr-btn = adr.

swl-mail-btn = mél

swl-mark-sent = envoyé

swl-recall = rappel

swl-recall-tip = Revenir à cette fréquence et ce mode — voir si la station est revenue

swl-edit = modifier

swl-del = suppr.

swl-no-match = Aucune réception ne correspond au filtre — EFFACER les affiche toutes.

swl-form-new = NOUVELLE RÉCEPTION

swl-form-edit = MODIFIER LA RÉCEPTION

swl-station-hint = Radio Taiwan International

swl-language-hint = français

swl-received = Reçu à

swl-received-hint = votre locator

swl-email = E-mail

swl-email-hint = rapports de réception

swl-email-tip = Figure sur la ligne Send to: du rapport

swl-postal = Postal

swl-postal-hint = rue, ville, pays

swl-postal-tip = Destination de secours du rapport lorsqu'il n'y a pas d'e-mail

swl-heard = Entendu

swl-smeter = S-mètre

swl-report-label = Rapport

swl-judged = évalué

swl-report-sent = Rapport envoyé

swl-not-yet = pas encore

swl-qsl-received = QSL reçue

swl-awaiting = en attente

swl-notes = Notes

swl-notes-hint = notes sur le programme

swl-pirate = Pirate

swl-pirate-tip = Une émission non autorisée — une station qui émet en dehors de toute attribution. Le journal la marque d'un drapeau pirate.

swl-mail-why-no-address =
    Cette station n'a pas d'adresse e-mail — cherchez-la dans la fenêtre HORAIRES, ou saisissez-en une ici, et cette option devient disponible.
    
    Un rapport de réception est une courtoisie que la station lit et à laquelle elle répond par une carte QSL ; sans adresse, il n'y a nulle part où l'envoyer.

swl-mail-why-bad-address = Cette adresse e-mail n'est pas une adresse unique, elle n'est donc pas utilisée — un rapport remis à un inconnu qui n'a jamais entendu la station est pire que pas de rapport.

swl-mail-why-ok =
    Rédiger le rapport de réception et l'ouvrir dans votre client de messagerie.
    
    L'image et l'extrait sonore sont à joindre par vos soins — ce programme ne peut pas mettre de fichier dans un message.

swl-mail-report = ENVOYER LE RAPPORT…

antenna-feed-ohms = alim. ≈ { $r } Ω

antenna-takeoff = élévation { $deg }°

antenna-director = directeur { $n }

antenna-reflector = réflecteur

antenna-driven = élément alimenté

antenna-gap = espacement { $v } m

antenna-az = az

antenna-depth = prof.

antenna-height = haut.

antenna-attenuation = affaiblissement dans l'azimut 0° : { $att } dB

morse-no-paddle-found-connect-one-then = aucune clé trouvée — branchez-en une, puis rouvrez ce volet

morse-send-with-a-paddle-played-here = Envoi à la clé — joué ici, jamais émis

morse-no-paddle-found-in-dev-input-by-id = Aucune clé trouvée dans /dev/input/by-id. Branchez-en une et rouvrez ce volet. Un boîtier manipulateur qui signale ses deux contacts (souvent comme les boutons d'une souris) convient ; un boîtier qui fait lui-même le manipulateur iambique et envoie des frappes ne convient pas.

morse-reverse = INVERSER

morse-swap-dit-and-dah-the-switch-on = Échange point et trait. Sur beaucoup de clés, un commutateur le fait sur la clé elle-même ; ceci sert quand il est dans l'autre position.

morse-stop-key = ARRÊTER LA CLÉ

morse-start-key = DÉMARRER LA CLÉ

morse-key-the-character-above-the-keyer = Manipulez le caractère ci-dessus. Le manipulateur rend les points et les traits propres — vous choisissez la clé et la durée de la pause. Un appui est un élément ; maintenez pour répéter.

morse-the-sending-drill-needs-a-linux = L'exercice d'envoi nécessite un bureau Linux et une clé.

panels-float-chip-hint = FLOAT — sortir ce panneau en panneau flottant pour rendre la place à la cascade ; le bouton d'ancrage sur le panneau le remet en place

panels-map-style-radar = Radar — fond sombre, terres remplies. Le fond sur lequel se lit une situation aérienne

panels-map-style-terrain = Relief — terres éclaircies et réchauffées pour se lire comme du sol, frontières atténuées

panels-map-style-chart = Carte — terres pâles, frontières soulignées et graticule, l'aspect aéronautique

panels-heard-me-chip = ENTENDU

panels-heard-me-hint = Entoure chaque station qui a signalé avoir entendu cet indicatif sur la bande actuelle, d'après PSK Reporter, placée sur le locator de la station qui l'a reporté. L'inverse des spots PSK : ceux-là sont les stations que cette radio entend. Actualisé toutes les heures, sur la dernière heure ; nécessite le flux PSK activé dans Réglages → Spots et un indicatif défini dans l'onglet Général.

panels-period-label = période

panels-sub-mode-label = sous-mode

panels-front-end-overloading = ⚠ ÉTAGE D'ENTRÉE SATURÉ — réduisez le gain RF (LNA/FI), ou déplacez l'antenne

panels-front-end-overloading-hint = Le récepteur est poussé au-delà de la pleine échelle : l'audio est fort mais ne porte aucun signal — rien ne se décodera tant que cela dure. Baissez le gain RF/LNA (plus d'atténuation), baissez le gain FI, ou activez l'AGC. La cause habituelle est un émetteur puissant tout proche, ou un autre poste sur la même bande.

panels-enigma-hint = Machine Enigma — chiffrez un message sur une face avant d'époque, et résolvez un texte chiffré copié à l'antenne

panels-slot-bar-heard-hint = Suit le son des haut-parleurs : blanc pendant qu'une trame est entendue, vert une fois qu'elle s'est arrêtée, et retour au début quand la suivante commence. Les durées sont celles du mode : une trame plus courte remplit donc quand même la barre.


morse-send-correct = correct : { $want }

morse-send-heard = entendu { $got }, attendu { $want }

morse-send-received = reçu : { $text }

amp-title-model = AMPLI · { $model }

amp-state-off = ARRÊT

amp-state-on = ON

amp-relay-dn = BAS

amp-relay-up = HAUT

amp-band-unknown = INC

amp-needle-top = HAUT

decodes-not-uploaded = ⚠ { $services } non envoyé

decodes-not-uploaded-hint = Le contact est dans le journal local, mais l'envoi a échoué. Vérifiez le jeton et la connexion dans Réglages → Spots, puis renvoyez-le depuis le journal.

decodes-logged-to = ↑ enregistré sur { $services }

decodes-logged-to-hint = Également envoyé aux journaux en ligne configurés pour recevoir chaque nouveau contact.

decodes-known = CONNUS

decodes-known-hint = Chaque indicatif qu'un <...> haché peut actuellement résoudre ici, avec son pays. Un hachage ne se lit pas à l'envers — une station n'apparaît qu'après avoir été entendue épeler son indicatif, ce à quoi sert un appel nu en 11 m.

decodes-ignored-clear = { $n } ignorés · effacer

decodes-auto-tx-frequency-hint =
    Choisit automatiquement notre fréquence d'émission : l'endroit le plus calme dans la période où nous émettons, plutôt que la fréquence de la station à laquelle nous répondons — elle émet dans l'autre période, la sienne ne dit donc rien de qui est là quand nous émettons. Désactivé NE maintient PAS la fréquence : on répond sur la fréquence de la station appelée. Pour la maintenir, utilisez Hold TX.
    
    Une bande fait exception : en 11 m, la norme WSJT-CB est de répondre à un CQ sur la tonalité de l'appelant et d'y rester pendant tout le contact ; la recherche est donc ignorée et une réponse à un CQ reste sur la fréquence où il a été entendu, que cette option soit activée ou non.

decodes-muted-caps = MUET

decodes-muted-hint = Rendu muet pour cette session — cliquez pour l'entendre à nouveau

decodes-ignore-hint = Ignore cette station pour le reste de la session : elle quitte la liste de décodage, ne déclenche plus les alertes et n'est jamais proposée au mode auto. Rien n'est envoyé ni téléversé en son nom, et redémarrer sdroxide la ramène.

decodes-qso-complete-hint = Le contact est terminé et dans le journal local. Il reste affiché ici quelques minutes pour que le dernier message puisse être renvoyé si l'autre station répète le sien — rien de plus n'est dû.

decodes-slot-even = paires

decodes-slot-odd = impaires

decodes-before = avant

decodes-after = après

decodes-beam-hint = Oriente l'antenne vers le locator de cette station

decodes-queue-next = suivant

decodes-queue-waiting = en attente


decodes-setup-hint = RÉGLAGES — l'identité de la station et les modèles que ces modes QSO partagent

decodes-auto-armed-hint =
    { $role } est armé. Cliquez pour arrêter.
    
    { $note }
    
    Étape du moteur : { $step } — une station qui répond à un CQ n'est prise en compte qu'en CallingCq sans contact en cours.

decodes-auto-rx-hint = Répond au CQ d'une nouvelle station, sans surveillance, et n'appelle jamais CQ lui-même. Seuls les indicatifs absents du journal reçoivent une réponse. Le chien de garde d'émission le cadence et l'arrêt sur inactivité le désarme.

decodes-auto-continuous-hint = Tout le déroulement, sans surveillance : terminer le QSO, travailler qui vous appelle, répondre à une station qui appelle CQ, et n'appeler CQ que s'il n'y a rien d'autre. Fonctionne jusqu'à ce que vous l'arrêtiez{ $until }. { $bound }

decodes-auto-or-minutes-up = { " " }ou que { $n } minutes soient écoulées

decodes-auto-cq-run-unlimited = Chaque série de CQ n'a pas de limite d'appels propre.

decodes-auto-cq-run-bounded = Chaque série de CQ est limitée par son propre nombre d'appels.

decodes-auto-no-call-limit = Pas de limite d'appels propre — le chien de garde et l'arrêt sur inactivité mettent fin à la série.

decodes-auto-after-calls-listen = Après { $repeats } appels, il passe en AUTO RX et continue d'écouter.

decodes-auto-after-calls-stop = Après { $repeats } appels, il s'arrête.

decodes-auto-cq-hint = Appelle CQ sans surveillance et travaille qui répond. { $bound } Une série trop courte et une bande calme ne vous entend jamais ; trop longue et l'émetteur reste sur une fréquence où personne ne répond.

decodes-auto-stopped-by-operator = { $role } arrêté par l'opérateur

decodes-cont = cont

settings-servers-decoded-text-udp = Texte décodé sur UDP

settings-servers-decoded-text-udp-note = Une ligne JSON par décodage, pour un script de votre cru : la famille FT8, WSPR et les lignes décodées de chaque mode clavier (CW, RTTY, PSK, Olivia, THOR, MFSK, DominoEX, Hell), chacune avec la fréquence du cadran où elle a été entendue. La diffusion WSJT-X ci-dessus est un protocole de logiciel de log et n'a pas de message pour ces modes, d'où l'existence de celle-ci.

settings-servers-decoded-text-udp-hint = hôte:port, en UDP. Vide : désactivé (par défaut). Sortie seule : rien sur ce socket ne peut agir sur la radio. Un nom est résolu quand vous appuyez sur APPLIQUER.

settings-servers-rotator-note = Oriente une antenne motorisée vers le satellite sur lequel vous êtes verrouillé, ou à la main depuis la fenêtre ROTATEUR et les cartes. Par défaut, c'est un démon Hamlib rotctld (lancez p. ex. « rotctld -m 603 -r /dev/ttyUSB0 » pour un rotor GS-232B, ou « rotctld -m 1 » pour essayer sans matériel) — un seul démon couvre tous les rotors gérés par Hamlib. Un contrôleur sur port série peut aussi être piloté directement.

settings-servers-connection = Connexion

settings-servers-rotctld-port-hint = 4533 est la valeur par défaut de rotctld

settings-servers-baud-hint = La plupart des contrôleurs fonctionnent à 9600

settings-servers-pstrotator-note = Port UDP propre à PSTRotator. Dans son Setup, activez UDP Control ; il reçoit les commandes sur ce port et répond sur le suivant.

settings-servers-el = él

settings-servers-persist-re-connect-rotator = Enregistrer et (re)connecter au rotor

settings-servers-tci-note = Permet aux programmes compatibles TCI de piloter cette radio : fréquence et mode, IQ large bande, audio de réception et audio d'émission. Dans WSJT-X, choisissez le type de radio SunSDR (TCI), pointez-le sur cette adresse et réglez le PTT sur TCI.

settings-servers-decoded-text-udp-format = Chaque ligne est un datagramme terminé par un saut de ligne : { $example }

settings-servers-transport-rotctld = Hamlib rotctld (TCP)

settings-servers-transport-easycomm = EasyComm II (série)

settings-servers-transport-gs232 = Yaesu GS-232 (série)

settings-servers-transport-pstrotator = PSTRotator (UDP)

spots-active = ACTIF

spots-closing = FERMETURE

spots-none-left-in-the-window = plus aucun dans la fenêtre

spots-openings = OUVERTURES

spots-band-opening-detections-paths = Détections d'ouverture de bande : liaisons dont l'activité récente a dépassé leur propre niveau de référence sur 3 heures

spots-save-the-spots-you-can-see-as-a = Enregistre les spots visibles dans un fichier KML — pour Google Earth et tout autre outil cartographique qui le lit. Seuls les filtres activés, et seuls les spots avec une position, car une épingle sans coordonnées est une épingle à l'autre bout du monde.

spots-no-spots-to-save-the-filters-are = Aucun spot à enregistrer — tous les filtres sont désactivés

spots-bc-all = BC TOUS

spots-bc-tuned = BC SYNTONISÉ

spots-which-broadcasters-the-waterfall =
    Quelles stations de radiodiffusion la **cascade** nomme.
    
    { "**" }BC SYNTONISÉ** est la valeur par défaut, selon la règle de bebepip : le nom de la station sur laquelle vous êtes accordé, rien d'autre — une bande étiquetée de bout en bout est un mur de noms au-dessus de la bande passante que vous essayez de lire.
    
    { "**" }BC TOUS** nomme chaque station que la grille a à l'antenne en ce moment, où qu'elle soit, ce qu'on veut en parcourant une bande plutôt qu'en écoutant une station. Cela ne dit rien sur le fait que la pastille BC ci-dessus affiche ou non les radiodiffuseurs.

app-rx-only-hint = Cette radio ne peut que recevoir — elle n'a pas d'émetteur à commander.

spots-n-calls = { $n } indicatifs

spots-opening-hover = Liaison { $band } de { $from } vers { $to } : la dernière fenêtre de 15 minutes a dépassé la référence de 3 heures de la liaison — appelants récents : { $calls }


spots-download-unsupported = { $source } ne peut pas être téléchargé sur cette plateforme

spots-kml-partial = { $placed } sur { $total } enregistrés — les autres n'ont pas de position et ne peuvent pas être épinglés

spots-net-log = JOURNAL RÉSEAU ({ $n })

decodes-beam = ORIENTER

settings-radio-no-answer-yet = En attente de la machine à laquelle la radio est reliée : ces commandes interrogent son matériel et son réseau, et elle ne répond qu'à une question à la fois.

settings-radio-elad-no-baud = la radio n'a pas de réglage à { $baud } bauds — { $default } sera utilisé à la place

settings-radio-elad-antenna-rx-only = RX seul

settings-radio-rx-only-n = RX{ $n } seul

settings-radio-ng-level = Niveau nG

settings-radio-ng-level-hint =
    nG ajoute deux commandes que le micrologiciel 2.00x n'a pas, et c'est le seul endroit où on peut les régler : nG ne répond à aucune et n'en mémorise aucune, donc sdroxide les renvoie à chaque ouverture de la liaison.
    
    Le volume est le niveau que la radio envoie dans le flux USB. nG prend cet audio après VOL et AGC ; c'est donc le bon endroit pour corriger un décodeur qui voit trop peu ou trop de signal — pas le mélangeur de l'ordinateur.
    
    Le contrôle de gain est le réglage AGC propre à la radio. Auto le met sur DIGI pour tout mode numérique et sur on sinon, ce que les notes de nG demandent pour le FT8 : le gain tient alors pendant les pauses d'émission de l'autre station au lieu de remonter dedans.

settings-radio-ng-volume-hint = 0–31, sous la forme `AG0nn;`. Le niveau que la radio envoie à l'ordinateur.

settings-radio-gain-control = Contrôle de gain

settings-radio-speaker = Haut-parleur

settings-radio-on-while-streaming = Actif pendant le flux

settings-radio-ng-speaker-hint = Décoché, coupe le haut-parleur propre de la radio pendant le flux (`UA2;`), si bien que l'audio n'est entendu que dans sdroxide. Le 2.00x n'avait pas cette forme.

settings-radio-ng-agc-auto = Auto (DIGI en modes de données)

settings-radio-ng-agc-off = Désactivé

settings-radio-ng-agc-on = Activé (rapide)

settings-radio-sound-card-note = La carte son qui porte l'audio de cette radio se choisit sous **Audio**, avec vos propres haut-parleurs et microphone — ce sont les mêmes réglages quelle que soit la radio utilisée, d'où une seule page.

settings-radio-no-control-cable-audio-page = Cette radio n'a pas de câble de commande — elle se déclenche par VOX, donc il n'y a aucun PTT à commander. L'émission est de l'audio envoyé au micro de la radio, ce qui explique que l'audio d'émission de la radio se trouve sur la page Audio et non ici.

settings-radio-sound-card-note-short = La carte son qui porte l'audio se choisit sous **Audio**.

settings-radio-control-link = Liaison de commande

settings-radio-atsmini-host-hint =
    Le serveur TCP ad hoc du récepteur : `atsmini.local` (mDNS) ou son IP. Sur la radio, Settings → TCP Port → Ad hoc, avec le Wi-Fi sur Connect, AP+Connect ou AP Only.
    
    `atsmini.local` nécessite un résolveur mDNS (Avahi/nss-mdns) sur cette machine. Si la liaison de commande ne s'établit pas, son IP s'affiche sur l'écran Wi-Fi de la radio — saisissez-la à la place.

settings-radio-atsmini-find-hint = Rechercher le port de commande du récepteur sur le réseau de cette machine

settings-radio-tcp-port = Port TCP

settings-radio-tcp-port-hint = Le micrologiciel utilise 60000 par défaut.

settings-radio-atsmini-receive-only = Réception seule : le Si4732 démodule en matériel ; le mode choisi ici est donc commandé à la radio, et tout le reste — détection, décodage, journal — se fait sur cet ordinateur.

settings-radio-atsmini-tuning-note =
    Le réglage suit le cycle de bande propre au récepteur : le cadran traîne donc un instant derrière votre molette, surtout quand la fréquence passe dans une autre bande. Laissez-le se stabiliser avant de continuer à faire défiler ; il finira toujours sur la fréquence demandée.
    
    Le pas d'accord du récepteur lui est propre (le contrôle STEP ci-dessus) : une variation plus fine que ce pas est arrondie par la radio, et le cadran revient au pas sur lequel il s'est réellement posé.

settings-radio-live-controls = Commandes en direct

settings-radio-atsmini-vol = VOL

settings-radio-atsmini-step = PAS

settings-radio-atsmini-sent-at-once = Envoyé à la radio immédiatement. VOL est le volume propre du récepteur (son bouton, et celui du Si4732), pas le curseur AF de sdroxide.

settings-radio-refresh = Actualiser

settings-radio-read-99-slots = Lire les 99 emplacements du récepteur

settings-radio-tune-radio-slot = Accorder cette radio sur l'emplacement

settings-radio-no-slots-stored = Aucun emplacement mémorisé dans le récepteur.

settings-radio-press-refresh-slots = Appuyez sur Actualiser pour lire les emplacements mémoire du récepteur.

settings-radio-icomnet-receive-hint = AF : la radio démodule et sdroxide affiche la bande audio. FI 12 kHz : la radio envoie à la place sa fréquence intermédiaire DRM et sdroxide démodule, ce qui apporte ses propres filtres, sa réduction de bruit et ses décodeurs sur environ ±12 kHz. Dans les deux cas, la cascade large est le scope propre de la radio — aucun Icom ne fournit d'I/Q.

settings-radio-copy-icom-session-hint = Copie la dernière session de cette radio — sa négociation et sa trace CI-V — dans le presse-papiers (ou dans un fichier, dans un navigateur qui ne laisse pas une page copier), pour un rapport de bogue. Celle de cette radio, non celle de la station : avec deux Icom sur le réseau local, chaque onglet répond pour sa propre adresse.

settings-radio-copy-protocol-trace-hint = Copie la trace du protocole de la dernière session dans le presse-papiers (ou dans un fichier, dans un navigateur qui ne laisse pas une page copier), pour un rapport de bogue.

settings-radio-copy-trace-device-hint = Copie la trace de la dernière session dans le presse-papiers (ou dans un fichier, dans un navigateur qui ne laisse pas une page copier), pour un rapport de bogue : chaque commande échangée avec le périphérique, et les premiers octets du flux d'échantillons.

settings-radio-copy-trace-receiver-hint = Copie la trace de la dernière session dans le presse-papiers (ou dans un fichier, dans un navigateur qui ne laisse pas une page copier), pour un rapport de bogue : chaque commande échangée avec le récepteur, et les premiers octets du flux d'échantillons.

settings-radio-pluto-choose-receiver = Veuillez choisir votre récepteur

settings-radio-dab-rate-hint = Choisissez 2.500 Msps pour décoder le DAB/DAB+

settings-radio-rx888-dab-rate-hint = Choisissez 80.0 Msps et la largeur de 2.500 MHz (1/32) pour décoder le DAB/DAB+

settings-radio-pluto-both-chains-hint = Pour écouter sur les deux antennes : ajoutez une seconde radio à la même adresse et réglez-la sur RX2. Les deux radios partagent la même fréquence.

settings-radio-duplex-tdd-hint =
    Indique si l'AD9361 fonctionne dans les deux sens à la fois (FDD, comme le Pluto démarre et comme sdroxide l'a toujours laissé) ou un sens à la fois (TDD).
    
    Laissez FDD sauf si vous voulez les broches PTT ci-dessous — le TDD est ce qui les commande, et il exclut le full duplex ci-dessus. Prend effet à l'application.
    
    PlutoSDR : le full duplex est désactivé — le TDD n'active qu'un sens à la fois, et c'est ce que commandent les broches GPO émission-réception. Mettez les broches PTT sur Off et le duplex sur FDD pour écouter pendant une transmission.

settings-radio-fm-mw-notch = Coupe-bande FM/PO

settings-radio-fm-mw-notch-hint = Coupe-bande matériel sur la bande de radiodiffusion FM (88–108 MHz) et sur les bandes d'ondes moyennes, pour quand un émetteur local sature tout le reste. S'applique immédiatement.

scanner-broadcast-band = Bande de radiodiffusion

scanner-all = TOUS

scanner-stop-only-on-channels-that-sound = S'arrêter seulement sur les canaux qui ressemblent à quelqu'un qui parle : une voix est périodique, remplit la bande vocale et monte et descend quelques fois par seconde, ce que ne font ni une porteuse, ni un parasite, ni une salve de données. Chaque candidat est écouté au moins une demi-seconde, car c'est le temps qu'il faut pour entendre une syllabe.

scanner-nothing-in-this-scan-is-in-a-mode = Rien dans ce balayage n'est dans un mode qui transporte la parole — une recherche de voix ici tournerait sans jamais s'arrêter, donc elle n'est proposée que lorsqu'il y en a un. La FM, l'AM et les bandes latérales transportent la voix ; les modes numériques non.

scanner-voice = VOIX

scanner-stop-only-where-somebody-is = s'arrêter seulement là où quelqu'un parle

scanner-no-voice-to-look-for-in-these = pas de voix à chercher dans ces modes

scanner-sql = SQL

scanner-how-long-to-stay-on-a-candidate = Combien de temps rester sur un candidat avant de le juger. La vérification de voix en exige au moins autant : une voix est un rythme de syllabes, et une demi-seconde en fait deux.

scanner-listening-for-a-voice = écoute d'une voix…

scanner-scan-band-hover = Balayer { $name } ({ $lo }-{ $hi } kHz)

scanner-range-to = à

scanner-mode-no-speech = { $mode } ne transporte pas de parole — ces canaux sont ignorés

scanner-voice-holding = voix { $score } · quelqu'un parle

scanner-voice-passed = { $mhz } MHz passé · voix { $score }

aircraft-hfdl-aircraft = Aéronef HFDL { $id }

aircraft-no-icao = pas d'adresse OACI (HFDL)

aircraft-provenance = { $source }, il y a { $age }

aircraft-label-flight = Vol

aircraft-label-registration = Immatriculation

aircraft-label-position = Position

aircraft-label-altitude = Altitude

aircraft-label-speed = Vitesse

aircraft-label-track = Route

aircraft-label-last-message = Dernier message

aircraft-label-type = Type

aircraft-label-operator = Exploitant

aircraft-label-origin = Origine

aircraft-label-destination = Destination

aircraft-label-online = En ligne

aircraft-missing-flight = pas encore entendu — ADS-B ou VDL2 le transportent

aircraft-missing-registration = pas encore entendu — seul VDL2 le transporte

aircraft-missing-position = pas encore entendu — ADS-B, ou un rapport de position VDL2 ou HFDL

aircraft-missing-adsb = pas encore entendu — ADS-B le transporte

aircraft-missing-message = aucun pour l'instant — VDL2, ou l'audio ACARS une fois que VDL2 a nommé l'aéronef

aircraft-online-unavailable-disabled = en ligne indisponible — l'interrupteur En ligne est coupé

aircraft-online-unavailable-no-station = en ligne indisponible — pas de station à interroger ; les données radio sont tout ce qu'il y a

aircraft-online-unavailable-unauthorized = en ligne indisponible — non connecté à la station

aircraft-online-unavailable-retry = lecture en ligne échouée — nouvel essai dans un instant

aircraft-switch-no-answer = la station n'a pas répondu

aircraft-switch-not-signed-in = non connecté à la station

aircraft-switch-refused = la station a refusé la demande

aircraft-switch-too-old = la station ne propose pas cet interrupteur (trop ancienne ?)

aircraft-switch-failed = la station n'a pas pu le modifier

aircraft-online-source = adsbdb, via la station

aircraft-asking-station = interrogation de la station

aircraft-provenance-radio = radio

aircraft-provenance-radio-online = radio + en ligne

aircraft-provenance-online = en ligne

aircraft-photo-credit = Photo : { $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = Aucun rotor d'antenne n'est configuré.

rotor-set-one-up-in-settings-servers-a = Configurez-en un dans Réglages ▸ Serveurs — un démon Hamlib rotctld ou un contrôleur sur un port série. Pointez ensuite le faisceau depuis ce cadran, depuis un indicatif ou depuis la carte.

rotor-halt-the-antenna-where-it-is = Arrêter l'antenne là où elle est

rotor-park = PARQUER

rotor-hand-the-antenna-back-to-the = Rend l'antenne au verrouillage satellite — elle suit le satellite et se gare quand il n'y en a pas.

rotor-point = POINTER

rotor-dx-country = Pays DX

rotor-set-your-grid-locator-settings = Indiquez votre locator (Réglages ▸ Général) pour pointer vers un pays.

rotor-connected = Connecté — antenne à { $az }°

rotor-not-connected-why = Non connecté : { $e }

rotor-beam = Faisceau { $az }° az  { $el }° él

rotor-target = cible { $az }° / { $el }°

rotor-park-hover = Aller au relèvement de parking ({ $az }° / { $el }°)

rotor-country-bearing = { $name } — relèvement { $az }°

aircraft-switch-no-station = il n'y a pas de station à commuter

aircraft-title = AÉRONEFS

aircraft-mode-adsb-hover = ADS-B, 1090 MHz — aéronefs avec leur position, altitude et vitesse

aircraft-mode-hfdl-hover = HFDL, liaison de données HF — aéronefs au-dessus des océans et des pôles, d'après leurs comptes rendus de position

aircraft-mode-vdl2-hover = VDL2, liaison de données VHF — messages d'aéronefs ; peu portent une position

aircraft-mode-acars-hover = ACARS — messages courts entre les aéronefs et le sol

aircraft-badge-online = en ligne

aircraft-badge-history = historique

aircraft-stats-total =
    { $n } { $n ->
        [one] aéronef
       *[other] aéronefs
    }

aircraft-stats-most-first = Plus nombreux d'abord

aircraft-stats-empty = Aucun aéronef pour l'instant. Les tableaux se remplissent à partir d'ADS-B, VDL2, HFDL et ACARS ; rien ici n'a besoin d'internet.

aircraft-stats-relay-note = Modèle, exploitant et route ne comptent que les aéronefs sur lesquels le relais a répondu — ouvrez une fiche dans Fiches pour en chercher un.

aircraft-stats-known-kinds = { $known } connus · { $kinds } sortes

aircraft-and-more = et { $more } de plus

aircraft-show = Afficher

aircraft-aviation = Aviation

aircraft-cards-chip = Fiches { $n }

aircraft-cards-hover = Chaque aéronef entendu en ADS-B, VDL2, HFDL et ACARS, une fiche chacun — conservée dix minutes, quel que soit le mode actuel de la radio

aircraft-watch-chip = Surveillance { $n }{ $suffix }

aircraft-watch-hover = Aéronefs dont vous avez demandé à être informé, par indicatif ou immatriculation — la liste est conservée par la station, et une alerte s'affiche quand l'un d'eux passe en émission

aircraft-stats-chip = Stats

aircraft-stats-hover = Les catégories, compagnies, modèles, exploitants et routes les plus courants parmi les aéronefs que connaît cette station — comptés d'abord d'après la radio, avec les fiches du relais en complément

aircraft-export = Exporter

aircraft-export-csv-hover = Enregistrer les aéronefs en tableau CSV pour un tableur : les dernières 24 heures de la station si elle garde un historique, sinon ce qui est entendu maintenant. Sans les positions.

aircraft-export-json-hover = Enregistrer les aéronefs en JSON : les dernières 24 heures de la station si elle garde un historique, sinon ce qui est entendu maintenant. Sans les positions.

aircraft-watch-intro = Indicatifs et immatriculations dont vous voulez être informé. La liste est conservée par la station, elle est donc identique depuis tous les écrans ; une alerte survient quand l'un d'eux apparaît, ou revient après dix minutes hors antenne.

aircraft-watch-needs-station = La liste de surveillance est conservée par la station à laquelle vous êtes connecté. Connectez-vous à une station pour l'utiliser.

aircraft-watch-reading = Lecture de la liste…

aircraft-watch-cannot-read = Impossible de lire la liste : { $why }.

aircraft-watch-input-hint = KLM1234 ou PH-BXA

aircraft-watch-add = Ajouter

aircraft-watch-sending = Envoi…

aircraft-watch-empty = Rien dans la liste pour l'instant.

aircraft-watch-on-air = en émission maintenant

aircraft-watch-remove = Retirer

aircraft-watch-alerts = Alertes

aircraft-watch-clear = Effacer

aircraft-watch-none-yet = Aucune pour l'instant.

aircraft-cards-empty = Aucun aéronef entendu pour l'instant. Les fiches se remplissent à partir d'ADS-B, VDL2, HFDL et ACARS — appuyez sur l'un d'eux et écoutez. Rien ici n'a besoin d'internet.

aircraft-cards-no-match = Aucun aéronef ne correspond à « { $query } ».

aircraft-history-hover = Afficher où cet aéronef est passé durant le dernier jour, d'après l'enregistrement de la station. Cela interroge votre station, jamais internet.

aircraft-copy-summary = Copier le résumé

aircraft-copy-summary-hover = Copier cette fiche en texte : ce qui est connu, et d'où vient chaque ligne. La position et le dernier message sont omis.

aircraft-asking-station-dots = interrogation de la station…

aircraft-not-changed = Non modifié : { $why }.

aircraft-loading-photo = chargement de la photo…

aircraft-radio-is-on = La radio est en { $mode } — appuyez sur { $label } pour écouter. Voici ce que { $label } a entendu auparavant.

aircraft-no-adsb = Aucun ADS-B entendu pour l'instant.

aircraft-hfdl-off = Le décodeur HFDL est désactivé — appuyez sur HFDL pour l'activer.

aircraft-vdl2-starting = Démarrage du décodeur VDL2…

aircraft-no-view = { $label } n'a pas de vue ici.

textmodem-waiting-modem = en attente du modem…

textmodem-locked = VERROUILLÉ

textmodem-no-frame = PAS DE TRAME

textmodem-locked-hint = Une trame est suivie. Elle n'est acceptée que lorsque la corrélation de synchronisation, repliée sur douze trames, se détache nettement de son propre nul et que l'autocorrélation à la période de trame indique la présence d'une trame.

textmodem-sync-readout = sync { $sync } (nul { $null })

textmodem-sync-hint = La corrélation de synchronisation différentielle repliée sur chaque phase de trame, et la médiane sur ces phases — le nul qu'elle doit dépasser. Mesuré en réel : 0,85 sur un 4285 contre 0,14 sur une forte porteuse PSK d'un autre mode.

textmodem-frame-readout = trame { $acf }

textmodem-frame-hint = La part d'une trame qui se répète une trame plus tard. Un 4285 répète ses 80 symboles de synchronisation, donc 0,3-0,6 ; la porteuse d'un autre mode mesure environ 0,03.

textmodem-refs-readout = réf { $a }/{ $b }/{ $c }

textmodem-refs-hint = Les concentrations des trois blocs de référence après désembrouillage. Ils valent tous le symbole 0 avant embrouillage, donc une trame correcte ramène chacun sur un seul point — 0,7 et plus. Seize symboles de bruit donnent 0,25.

textmodem-rate-hint = Ce que la constellation permet de dire. Chaque débit non codé partage sa constellation avec un débit codé ; les débits codés ont donc besoin de l'Annexe E pour être distingués — et leurs données ont besoin de la FEC de l'Annexe E pour être lues.

textmodem-offset-frames = { $offset } Hz · { $n } trames

textmodem-offset-frames-hint = Le décalage de porteuse sur lequel s'accordent les blocs de référence, et le nombre de trames démodulées pendant cette session.

textmodem-som-readout = SOM { $best }/32 · { $n } msg

textmodem-som-hint = Le motif de début de message de l'Annexe E, et à quel point la sortie du décodeur s'en est approchée. Le bruit se situe autour de 16 sur 32 ; une émission codée atteint la vingtaine même quand un évanouissement empêche la concordance, et une concordance est ce qui désigne le débit codé et démarre un message. Le code et l'entrelaceur décodent exactement un signal généré, mais aucune station codée n'a encore été lue en réel ; une concordance ici sera donc la première chose à attendre quand ce sera le cas.

textmodem-rx-only = RÉCEPTION SEULE

textmodem-rx-only-hint = Le décodeur de ce mode est porté et lit ses enregistrements ; son modulateur n'est pas encore écrit, la partie émission est donc désactivée plutôt que de refuser après coup.

textmodem-msg-hint = Vos propres boutons de message — conditions de travail, météo, une réponse type. Chacun envoie son texte entier d'un coup, et F1–F9 actionnent les neuf premiers. Ils font partie de la configuration de la station, un client distant les a donc aussi.

textmodem-weather-chip = {" "}MÉTÉO { $n }{" "}

textmodem-weather-hint =
    Rapports SYNOP et SHIP décodés entendus sur cette bande.
    
    Ils sont conservés ici plutôt que dans la liste de décodage car les chiffres seuls ne signifient rien — et parce que la météo à une position il y a trois heures est ce que l'on vient rechercher.
    
    Trouvés dans le texte RTTY à mesure qu'il arrive : le mode doit donc être réglé sur le décalage et les bauds de la station. Une station terrestre n'envoie pas de position, ces lignes portent donc le numéro de station et les chiffres mais pas de coordonnées ; un rapport de navire porte les siennes.

textmodem-olivia-reverse-hint = Inverse l'ordre des tons, dans les deux sens. Une station qui émet à l'envers est lue comme du bruit tant que ce n'est pas activé.

textmodem-olivia-tones-raised = (le { $n } d'Olivia → 4)

textmodem-ifk-reverse-hint = Inverse les pas entre tons. L'IFK+ porte ses données dans le pas entre tons successifs ; une station reçue sur la bande latérale opposée a donc tous ses pas négatifs et est lue comme du bruit tant que ce n'est pas activé.

textmodem-domino-hint = { $name } — { $baud } bauds, 18 tons, manipulation incrémentale de fréquence sans correction d'erreurs directe

textmodem-mt63-long-hint = MT63-{ $label }, { $bw } Hz de large — l'entrelacement long répartit une salve sur { $chars } caractères

textmodem-mt63-short-hint = MT63-{ $label }, { $bw } Hz de large, entrelacement court — l'entrelacement court répartit une salve sur { $chars } caractères

textmodem-mt63-not-offered = 1K et 500L pas encore proposés

textmodem-mt63-not-offered-hint = MT63-1K et l'entrelacement long de 500 Hz se lisent avec le décodeur par lots mais pas encore avec le récepteur en direct : le verrouillage cherche la phase des symboles et la polarité mais pas le centre de bande, et une recherche qui le fait s'est avérée échanger une variante contre une autre. Voir docs/internal/MODE-PROGRESS.md.

textmodem-mfsk-caveat-mfsk8 = { " " }— l'enregistrement du corpus perd un passage de son texte ; le décodeur est celui que toutes les autres variantes utilisent

textmodem-mfsk-caveat-mfsk11 = { " " }— l'enregistrement du corpus perd son premier groupe à cause du démarrage du rééchantillonneur

textmodem-mfsk-hint = { $label } — { $baud } bauds, { $tones } tons, { $bw } Hz de large{ $caveat }

textmodem-mfsk-reverse-hint = Inverse l'ordre des tons. Une station reçue sur la bande latérale opposée à celle d'émission arrive en miroir autour de son centre et est lue comme du bruit tant que ce n'est pas activé.

textmodem-psk-family = Famille

textmodem-psk-family-plain-hint = La famille propre du PSK31 : une porteuse différentielle, sans correction d'erreurs directe

textmodem-psk-family-bpsk63f-hint = Le code robuste à 62,5 bauds, sans entrelaceur — le seul mode codé que fldigi émet sans entrelaceur

textmodem-psk-family-robust-hint = Une porteuse avec le code robuste et un entrelaceur 2x2xN

textmodem-psk-family-multi-hint = Plusieurs porteuses, le varicode du PSK31 et **aucun** code — plus large et plus rapide, il exige un signal propre

textmodem-psk-family-robust-multi-hint = Plusieurs porteuses **et** le code robuste : le plus large et le plus robuste des modes PSK

textmodem-psk-baud-fast-hint = { $baud } bauds — se lit à partir d'environ une seconde : à ce débit, le temps de verrouillage du récepteur représente un tiers de message

textmodem-psk-baud-hint = { $baud } bauds

textmodem-psk-modulation = Modulation

textmodem-psk-bpsk-hint = Un bit par symbole, détecté de façon différentielle

textmodem-psk-qpsk-hint = Deux bits par symbole avec un code de rendement 1/2, donc un bit de donnée par symbole : le même débit de texte que le BPSK à la même vitesse, avec moitié moins d'erreurs

textmodem-psk-8psk-hint = Trois bits par symbole, sans code : le plus rapide des trois à vitesse donnée et le moins robuste. Réception seule — il n'y a pas d'émetteur à huit phases. Validé sur un signal synthétique : les enregistrements 8PSK du corpus de test ne forment pas une constellation propre

textmodem-psk-coded = codé

textmodem-psk-no-code = sans code

textmodem-psk-coded-hint =
    { $baud } bauds, { $n } { $n ->
            [one] porteuse
           *[other] porteuses
        }, { $code }, { $bw } Hz de large — réception seule : l'émetteur PSK codé n'est pas écrit

textmodem-psk-wide-bank-hint = { "*" }*Ce large banc ne décode pas encore.** Ses porteuses sont assez proches pour que le filtre du récepteur à la fréquence d'entrée ne puisse pas les séparer ; les bancs plus étroits lisent bien leurs propres enregistrements. Rien de ce que vous entendez ici n'est perdu — c'est l'étage d'entrée qui est en cause, pas le signal.

textmodem-hell-slant = Inclinaison

textmodem-hell-slant-hint = Réglage fin de l'horloge d'échantillonnage en réception. Une capture dont les points arrivent 0,4 % trop longs — une carte son ordinaire, et ce que mesure l'enregistrement Hell de SkySweep — nécessite environ -4000. Ajustez jusqu'à ce que les glyphes soient droits.

settings-net-rbn-enabled-hint = Lit les skimmers du monde entier et en alimente la carte de propagation. C'est ce qui permet à la carte d'afficher des bandes que cette radio n'écoute pas. Rien n'est émis, aucun compte n'est nécessaire, et l'indicatif de l'onglet Général est utilisé. Les spots RBN n'apparaissent pas dans la liste des spots : ce sont des mesures, pas des invitations. L'hôte n'est pas lié à reversebeacon.net : tout nœud qui parle le même flux telnet convient.

settings-net-rbn-port-hint = Sur reversebeacon.net, 7000 est le flux CW/RTTY, 7001 celui FT8/FT4

settings-net-wavelog-intro = Wavelog est un journal de trafic que **vous** exploitez, à l'adresse que vous indiquez. Les contacts enregistrés ici sont envoyés vers l'emplacement de station choisi, et ceux qu'il contient déjà peuvent être rapatriés pour que « déjà contacté » connaisse les QSO faits depuis un autre programme, un autre poste ou son propre formulaire web.

settings-net-wavelog-address-hint = L'adresse de la page d'accueil de Wavelog : https://log.example.org, ou son adresse sur le réseau local. /index.php est accepté dans les deux cas.

settings-net-wavelog-key-hint = Une clé API issue de la page API de Wavelog. Une clé en lecture/écriture est nécessaire pour envoyer ; une clé en lecture seule ne permet que de télécharger.

settings-net-pick-one = choisir

settings-net-test-to-list-locations = testez la connexion pour lister les emplacements

settings-net-trust = FAIRE CONFIANCE

settings-net-trust-hint = Accepter **ce** certificat et aucun autre pour ce serveur. Une épingle plutôt que « tout accepter » : s'il change un jour, la connexion est refusée et on vous le dit, au lieu de s'adresser en silence à ce qui répond à cette adresse.

settings-net-test-wavelog = Tester Wavelog

settings-net-test-wavelog-hint = Demande au serveur si cette adresse et cette clé fonctionnent, et liste ses emplacements de station. Applique d'abord les réglages ; n'enregistre rien.

settings-net-wavelog-download = Télécharger les contacts depuis Wavelog

settings-net-every-min = Toutes les (min)

settings-net-locations = Emplacements

settings-net-download-now = Télécharger maintenant

settings-net-download-now-hint = Rapatrie ce que Wavelog a ajouté depuis le dernier téléchargement et marque comme envoyés les contacts qu'il détient déjà. Rien de ce qui est déjà dans le journal n'est modifié.

settings-net-wavelog-upload-note = Seuls les contacts enregistrés **ici** sont envoyés automatiquement ; l'arriéré propre à Wavelog est envoyé en cochant un QSO dans le journal, de sorte que rien ne quitte cette machine sans votre choix.

settings-net-broadcast-note = Les stations en ondes longues et courtes affichées sur la cascade. Chaque source ci-dessous est téléchargée pour la saison de diffusion en cours et retéléchargée au changement de saison. Toutes vont dans **un seul stock**, de sorte que le filtre Source de la fenêtre PROGRAMME en montre une ou toutes.

settings-net-downloaded = téléchargé

settings-net-schedules = Programmes

settings-net-schedule-yours = le vôtre

settings-net-download = Télécharger

settings-net-hfcc-hint = HFCC publie les noms de ses stations dans un fichier séparé, il faut donc les deux. Tant que ce n'est pas fait, il le dit plutôt que de charger un programme de simples indicatifs.

settings-net-refetch-hint = Récupérer à nouveau la copie de cette saison, en remplaçant celle du cache

settings-net-from-file = Depuis un fichier…

settings-net-aoki-file-hint = Prendre le programme d'Aoki dans un fichier que vous avez enregistré — le .zip publié par Aoki ou le .txt qu'il contient. Son site change avec la saison, d'où ce bouton.

settings-net-eibi-file-hint = Prendre le programme d'EiBi dans son fichier sked .csv enregistré sur cette machine

settings-net-filter-aoki = Programme Aoki

settings-net-filter-eibi = Programme EiBi

settings-net-download-all = Tout télécharger maintenant

settings-net-download-all-hint = Récupérer toutes les sources récupérables, en conservant celles qui fonctionnent déjà

settings-net-own-list-note = Votre propre liste est un **programme** — ce que vous comptez entendre — et non le journal de réception, qui consigne ce que vous avez réellement entendu. Importez-la ci-dessous ou modifiez le fichier.

settings-net-import-list = Importer ma liste de stations…

settings-net-import-list-hint = Lit un CSV ou une simple liste de stations et la fusionne dans le fichier ci-dessus. Les fréquences peuvent être en kHz ou en MHz ; l'unité se déduit du nombre.

settings-net-filter-station-lists = Listes de stations

settings-net-dx-status = DX — { $body }

settings-net-pin-own = le sien — { $pin }…

settings-net-schedule-rows = { $name } { $n } émissions

settings-net-schedule-not-downloaded = { $name }     non téléchargé

settings-net-schedule-imported = { $source } { $season } — { $n } émissions, maintenant dans le stock

settings-net-stations-imported =
    { $n ->
        [one] { $n } station dans votre liste, maintenant dans le stock
       *[other] { $n } stations dans votre liste, maintenant dans le stock
    }

txaudio-natural = Naturel

txaudio-ragchews-and-local-contacts-the = Bavardages et contacts locaux : le réducteur de bruit, un expandeur doux dans les pauses, une légère compression. Vous, sans la pièce.

txaudio-ssb-everyday = SSB courante

txaudio-everyday-ssb-100-hz-2-9-khz-a = SSB de tous les jours : 100 Hz – 2,9 kHz, un peu de présence vers 2 kHz, compression modérée.

txaudio-ssb-punch-dx = SSB mordante / DX

txaudio-weak-signals-and-pileups-a-narrow = Signaux faibles et pile-ups : une voix étroite de 300 Hz – 2,7 kHz avec une forte présence, poussée dans un compresseur rapide avec le rotateur de phase, pour le maximum de puissance de parole que l'émetteur peut donner. Pas de la haute fidélité.

txaudio-ssb-punch-for-hours-in-a-noisy = SSB mordante pour des heures dans une station bruyante : le réducteur de bruit et un gate sévère tiennent ventilateurs et opérateur voisin hors de vos pauses, et le compresseur travaille encore plus fort.

txaudio-essb-hi-fi = ESSB / haute fidélité

txaudio-wide-ssb-or-am-50-hz-4-khz-with = SSB large ou AM : 50 Hz – 4 kHz en gardant les graves, compression douce et un dé-esseur. Exige un filtre d'émission aussi large — et une bande calme.

txaudio-fm-repeater = FM / relais

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM : un filtre passe-haut raide à 300 Hz éloigne la voix du sous-ton CTCSS, le haut s'arrête à 3 kHz pour le canal, et le limiteur tient l'excursion.

txaudio-mobile-noisy-shack = Mobile / shack bruyant

txaudio-fans-engine-traffic-the-denoiser-a = Ventilateurs, moteur, circulation : le réducteur de bruit, un passe-haut raide à 150 Hz et un gate sévère, avec assez de compression pour dominer ce qui reste.

txaudio-digital-voice-2 = Voix numérique

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE : réducteur de bruit et nivellement doux uniquement, pas d'égaliseur — le vocodeur préfère la parole naturelle. Active VOIX NUMÉRIQUE.

txaudio-imported-2 = Importé

frame-no-contrast-fit = Aucun contraste — cliquez pour FIT

frame-levels-hidden-hint = Le plancher et le plafond d'affichage sont au-delà de tout ce qui est à l'écran : la cascade est d'une seule couleur uniforme. Cliquez pour les ajuster à ce que la radio entend.

frame-bw-waterfall = BW cascade · { $span }

frame-bw-waterfall-unknown = BW cascade · —

frame-every-module-own-window = chaque module est dans sa propre fenêtre

frame-window-chip-hint = WINDOW — déplacer cette fenêtre dans une fenêtre système à part, pour la mettre sur un autre écran

frame-dock-panel-pane-hint = DOCK — remettre ce panneau dans le volet sous la cascade

frame-dock-panel-hint = DOCK — remettre ce panneau sous la cascade

frame-dock-spectrum-hint = DOCK — remettre ce spectre dans la fenêtre principale, comme en fermant celle-ci sans chercher sa case de fermeture.

frame-dock-band-menu-hint = DOCK — remettre le sélecteur de bande et de mode dans la fenêtre principale, comme en fermant celle-ci sans chercher sa case de fermeture.

frame-dock-rotator-hint = DOCK — remettre le rotor dans la fenêtre principale, comme en fermant celle-ci sans chercher sa case de fermeture.

frame-module-panadapter = panadapter

frame-module-panel = panneau

frame-module-controls = commandes

frame-module-aux-panadapter = panadapter auxiliaire

frame-module-band-mode = bande et mode

frame-module-rotator = rotor d'antenne

frame-detached-title = { $flavor } — { $name }

frame-workspace-missing = Aucun espace de travail { $n } enregistré — enregistrez-en un dans Réglages › Interface › Espaces de travail et cette touche le rappellera.

frame-no-operating-panel = Pas de panneau d'exploitation en { $mode } — il est détaché, et ce mode n'en a pas.

frame-rx-only-nudge = Cette radio est en réception seule — elle n'a pas d'émetteur. Masquer les commandes d'émission et utiliser l'écran d'écoute ?

frame-listening-controls = Commandes d'écoute

frame-listening-controls-hint = Masque les commandes d'émission et passe cette radio sur l'écran d'écoute (Réglages → Radio). Rien n'est re-accordé et le décodeur continue de tourner.

frame-update-available = SDR Oxide Brown { $tag } est disponible — vous utilisez la { $version }.

frame-download = Télécharger

frame-download-hint = Ouvre la page de la version dans votre navigateur

frame-tuning-catching-up = accord en cours — la radio rattrape son retard

frame-roll-up-hint = Enrouler — ne garder que la barre de titre et rendre la place à la cascade

frame-unroll-hint = Dérouler — afficher à nouveau le panneau

frame-window-panel-hint = WINDOW — donner à ce panneau sa propre fenêtre, à placer sur un autre écran

frame-panel-window-title = { $mode } — panneau

frame-auto-stopped-stop-control = auto arrêté : une commande d'arrêt a été utilisée

frame-dont-show-again = Ne plus afficher

frame-dont-show-again-hint = Mémorise cet accusé de réception sur cet écran. Le verrouillage reste désactivé tant que le moteur tourne avec --oob-tx — cela empêche seulement que l'avertissement soit affiché à nouveau.

frame-cb-window-title = ⚠  11 m (CB) — PAS UNE BANDE RADIOAMATEUR

frame-cb-about-to-allow = Vous allez autoriser l'émission sur la bande des 11 m.

frame-cb-not-amateur = Le 11 m n'est pas une bande radioamateur. C'est un service radio distinct, avec ses propres canaux, modes, limites de puissance et matériels homologués, qui diffèrent d'un pays à l'autre. Y émettre est soumis aux règles du pays depuis lequel vous opérez.

frame-cb-only-continue = Ne continuez que si vous savez ce que votre pays autorise sur 27 MHz et que vous opérez dans ce cadre — vous restez responsable de l'émission.

frame-cancel = Annuler

frame-saved-to-profile = enregistré dans le profil { $name }

frame-saved-as-default = enregistré comme défaut de cette station (le serveur n'a pas de mot de passe : c'est ce que reçoit chaque client)

frame-bindings-offer-title = Ce profil contient des raccourcis clavier

frame-bindings-offer-profile = Le profil sous lequel vous êtes connecté ({ $name }) stocke des raccourcis clavier et souris sur le serveur, et ce navigateur ne les utilise pas. C'est le choix sûr par défaut — sur une station partagée, le clavier l'est aussi, et qu'une connexion adopte le PTT ou la barre d'espace d'une autre est un vrai danger.

frame-bindings-offer-default = Le profil par défaut de cette station stocke des raccourcis clavier et souris sur le serveur, et ce navigateur ne les utilise pas. C'est le choix sûr par défaut — sur une station partagée, le clavier l'est aussi, et qu'une connexion adopte le PTT ou la barre d'espace d'une autre est un vrai danger.

frame-bindings-offer-choice = Utilisez-les ici, ou laissez-les. Dans les deux cas, on vous dira ce qui a été fait, et rien n'est modifié sans votre choix.

frame-bindings-use = Utiliser les raccourcis de ce profil

frame-bindings-keep-mine = Garder les miens (et ne plus demander)

frame-bindings-using-profile = utilisation des raccourcis clavier enregistrés pour ce profil

frame-bindings-keeping-own = conservation des raccourcis clavier propres à cet appareil ; la question ne sera plus posée

frame-look-back-to-profile = retour à l'apparence enregistrée pour ce profil

frame-look-none-stored = aucune apparence enregistrée pour ce profil — enregistrez-en une d'abord

frame-antenna-rotator-caption = ROTOR D'ANTENNE

bands-condx-global-forecast-rest-what = CONDX est une prévision mondiale. WSPR est ce que le réseau WSPR mondial a entendu (wspr.live) ; PATHS, REACH et BEST sont ce que cette station et le RBN ont entendu. Une ligne vide signifie que rien n'a été décodé — la bande était peut-être fermée, ou simplement personne n'y était.

bands-km-s-parent-peak-rate =
    { $name } ({ $code }) — { $velocity_kms } km/s, corps parent { $parent }. Pic ZHR { $zhr } vers le { $peak }.
    
    Un radiant au-dessus de l'horizon signifie que les traînées peuvent vous atteindre ; une pluie rapide laisse des traînées ionisées plus durables pour la diffusion par météores sur 6 m et 2 m, et les brèves ouvertures sur 10 m/11 m.

bands-hearing-means-path-open-each = { $callsign } à { $location } — l'entendre signifie que la liaison { $band } est ouverte. 10m (28,200 MHz) est le substitut en bande amateur le plus proche des conditions sur 11 m. Chaque balise monte d'une bande toutes les 10 s, donc cette ligne change à chaque créneau.

bands-wspr-activity-old = · activité WSPR vieille de { $age } (wspr.live)

bands-psk-activity-old = · activité PSK vieille de { $age }

bands-activity-hover =
    Activité mondiale sur { $band } ces 15 dernières minutes, d'après { $source } :
    { $paths } rapports de réception · { $tx } émetteurs · { $rx } récepteurs
    
    { $note }
    
    Plus c'est clair, plus la bande est active ; cela dit que la bande s'entend quelque part, pas qu'elle est ouverte pour vous.

bands-derived-hover =
    { $band } n'est pas l'une des bandes notées par HAMQSL.com ; c'est le verdict publié du groupe { $group }, pris comme substitut le plus proche.
    
    Une prévision, pas une mesure de votre propre liaison.

bands-wspr-note = Le réseau WSPR mondial — balises pour signaux faibles.

bands-psk-note = Rapports de réception des modes d'activité : FT8, FT4 et le reporting CW/RTTY que les balises WSPR ne couvrent pas.

aircraft-history-reason-disabled = la station n'a pas activé son historique

aircraft-history-reason-unauthorized = la station demande une connexion

aircraft-history-reason-no-station = l'historique est conservé par la station à laquelle vous êtes connecté

aircraft-history-reason-unknown = la réponse de la station n'a pas été comprise

aircraft-history-show = Afficher l'historique

aircraft-history-hide = Masquer l'historique

aircraft-label-history = Historique

aircraft-label-first-heard = Premier entendu

aircraft-label-last-heard = Dernier entendu

aircraft-label-track-points = Points de trace

aircraft-label-highest = Plus haut

aircraft-label-callsigns = Indicatifs

aircraft-label-registrations = Immatriculations

aircraft-history-nothing-kept = rien de conservé pour cet aéronef pour l'instant

aircraft-history-by-station = par cette station

aircraft-history-kept-24h = conservés pour les dernières 24 h

aircraft-history-of-kept = parmi les points conservés

aircraft-history-used-24h = utilisés ces dernières 24 h

aircraft-history-seen-24h = vues ces dernières 24 h

aircraft-history-looked-up = consulté il y a { $age }

aircraft-alert-on-air-as = { $entry } est en émission, comme { $callsign }

aircraft-alert-on-air = { $entry } est en émission

aircraft-watch-already = Déjà dans la liste.

aircraft-watch-absent = Pas dans la liste.

aircraft-watch-invalid = Ni un indicatif ni une immatriculation : 3 à 10 lettres ou chiffres.

aircraft-watch-full = La liste est pleine : 100 aéronefs.

multi-module-windows = FENÊTRES DES MODULES

multi-in-its-own-window = DANS SA FENÊTRE

multi-docked = ANCRÉ

multi-dock-all-windows = ANCRER TOUTES LES FENÊTRES

multi-bring-every-module-window-back = Ramène chaque fenêtre de module dans la fenêtre principale

multi-undock-all-windows = DÉTACHER TOUTES LES FENÊTRES

multi-put-every-module-in-a-window-of = Met chaque module dans sa propre fenêtre — la fenêtre principale ne garde que le spectre et les décodeurs

multi-two-radios-side-by-side-need-a = Deux radios côte à côte demandent une fenêtre plus large — cet écran n'en montre qu'une à la fois

multi-the-station-s-windows-some-modules = Les fenêtres de la station — certains modules sont dans leurs propres fenêtres système. Ouvrez ceci pour changer cela.

multi-put-a-module-in-its-own-window-sp1 = Mettez un module dans sa propre fenêtre — SP1 ou AUX SP sur un autre écran, la commande RX à côté. Les mêmes lignes se trouvent dans Réglages → Interface → Détaché.

multi-module-sp1 = SP1 — le spectre

multi-module-sp2 = SP2 (AUX) — un second spectre

multi-module-panel = Panneau d'exploitation

multi-module-controls = Commande RX

multi-module-band-mode = Bande et mode

multi-module-rotor = Rotor d'antenne

multi-bring-back = Ramener { $name } dans la fenêtre principale

multi-put-own-window = Mettre { $name } dans sa propre fenêtre système — avec son propre app-id, pour qu'une règle de fenêtre puisse l'épingler à un écran

multi-windows-chip = FENÊTRES

aircraft-stats-category = Catégorie

aircraft-stats-airline = Compagnie

aircraft-stats-model = Modèle

aircraft-stats-operator = Exploitant

aircraft-stats-route = Route

aircraft-stats-src-no-station = Comptage des aéronefs entendus maintenant. La station à laquelle vous vous connectez garde le dernier jour.

aircraft-stats-src-asking = Demande à la station du dernier jour…

aircraft-stats-src-ready = Les dernières 24 h conservées par la station, avec les aéronefs entendus maintenant.

aircraft-stats-src-disabled = La station n'a pas activé son historique — comptage des aéronefs entendus maintenant.

aircraft-stats-src-reason = { $reason } — comptage des aéronefs entendus maintenant.

aircraft-export-saved-station = { $name } enregistré : les dernières 24 heures de la station.

aircraft-export-saved-local = { $reason } : à la place, les { $n } aéronefs entendus ces dix dernières minutes ont été enregistrés.

login-remember-this-station = Se souvenir de cette station

login-remember-me = SE SOUVENIR DE MOI

login-this-station-keeps-your-sign-in = Cette station garde votre connexion, pas cette page — rien à remplir sur les autres radios, et rien à oublier quand vous partez.

login-not-this-time-this-browser-forgets = Pas cette fois : ce navigateur l'oublie à sa fermeture, on vous le redemandera à la prochaine session et rien de cette connexion ne reste sur l'appareil.

aircraft-label-range = Distance

aircraft-label-vertical = Vertical

aircraft-label-emergency = Urgence

aircraft-from-your-station = depuis votre station

aircraft-footer-disabled = en ligne indisponible — relais désactivé sur la station

aircraft-footer-no-station = en ligne indisponible — pas de station à interroger

aircraft-footer-unauthorized = en ligne indisponible — non connecté

aircraft-hover-close = Fermer (Échap)

aircraft-hover-lookup = Rechercher en ligne

aircraft-hover-lookup-hint = Interroger le relais de cette station sur cet aéronef : type, exploitant, route et une photo. Rien n'est demandé tant que vous n'appuyez pas.

aircraft-hover-lookup-waiting = une autre recherche est en cours — une à la fois

aircraft-hover-history-hint = Où cet aéronef est passé durant le dernier jour, d'après l'enregistrement de votre station. Cela interroge votre station, jamais internet.

auto-stopped-reason = auto arrêté : { $reason }

auto-stopped-idle = auto arrêté : { $mins } minutes sans action de l'opérateur

auto-answering = { $role } : réponse à { $call }

auto-caller-went-away = auto : un appelant est parti

auto-nobody-left = auto : plus personne à qui répondre

auto-calling-cq = { $role } : appel CQ (n° { $n })

auto-cq-no-answer = auto cq : pas de réponse après { $limit } appels — écoute (auto rx)

auto-listening = { $role } : écoute

auto-role-stopped = { $role } arrêté : { $sentence }

auto-rx-listening = auto rx : écoute

auto-cq-calling = auto cq : appel CQ

auto-cont-running = auto cont : en cours

aircraft-search-missing-online = { $missing } aéronefs sur { $total } sans données en ligne — ouvrez leur fiche

aircraft-search-shown-of = { $shown } sur { $total }

aircraft-search-help =
    Rechercher parmi les aéronefs déjà entendus : l'adresse (OACI), l'indicatif, l'immatriculation, le type, l'exploitant ou la catégorie. Plusieurs mots doivent tous correspondre. Limitez un mot à un champ avec icao: call: reg: type: op: — par exemple  reg:PH op:KLM.
    Le type et l'exploitant viennent de la recherche en ligne de la station, donc ils ne sont trouvés que pour les aéronefs dont la fiche a été consultée. La recherche ne demande rien.

aircraft-search-hint = rechercher

aircraft-search-clear = Effacer la recherche

gridtracker-shade-squares-heard-live-decode = Colorer ce qui est entendu dans la liste de décodage en direct, pas seulement ce qui est dans le journal. La version « auditeur » de la carte : ce qui est en l'air maintenant, en cyan sous les marques des contacts faits.

gridtracker-grid-ham = GRILLE/RADIOAMATEUR

gridtracker-maidenhead-squares-what-a-ham-log = Carrés Maidenhead — ce que contient un journal de radioamateur. Aucun indicatif CB ici : un échange sur 11 m porte un pays, jamais un locator.

gridtracker-country-cb = PAYS/CB

gridtracker-dxcc-countries-at-their-nominal = Pays DXCC en leur centre nominal — ce que contient un journal CB, puisqu'un échange sur 11 m n'a pas de locator. Pas d'indicatifs radioamateur ici non plus : ce sont des pays, pas des carrés.

colouredit-paste-a-colour-code-rrggbb-or-r-g = Coller un code couleur (#RRGGBB ou r, g, b) — ou Cmd/Ctrl+V sur l'échantillon

cwkey-thread-failed = impossible de démarrer le thread de la clé CW : { $error }

cwkey-grab-failed = impossible de prendre la clé en exclusivité — ses contacts peuvent aussi cliquer dans d'autres fenêtres (un autre programme l'utilise-t-il ?)

ignore-muted-hover =
    Mises en sourdine pour cette session : { $names }.
    
    Une station en sourdine disparaît de la liste de décodage, ne déclenche pas les alertes, n'est pas lue à voix haute et n'est jamais proposée au mode auto comme station à appeler. Rien n'est émis ni téléversé en son nom, et la liste est oubliée à la fermeture de sdroxide — il n'y a aucune copie enregistrée, donc rien ne reste caché après une soirée d'écoute.
    
    Cliquez pour toutes les réactiver, ou appuyez sur le bouton − sur l'une de leurs lignes.

js8-setup-hint = RÉGLAGES — indicatif, groupes, réponse automatique, intervalle de balise et message d'état

logbook-contest-logger-a-session-dupes-a = Journal de concours — une session, les doublons, un score en direct et Cabrillo

logbook-up-chip = ENV.

net-wavelog-stopped = arrêté : { $error }

net-wavelog-download-stopped = Téléchargement Wavelog arrêté : { $error }

net-wavelog-new-already-here = { $added } nouveaux, { $matched } déjà présents

net-wavelog-fill-in-first = l'adresse et la clé API doivent d'abord être renseignées

net-wavelog-no-station-location = aucun emplacement de station à télécharger — testez d'abord la connexion

publicsdr-could-not-connect = connexion impossible pendant cette session

publicsdr-use-chip = UTILISER

weather-title = BULLETINS MÉTÉO

weather-intro = Bulletins météo codés de navires et de stations terrestres, en RTTY sur 4583, 7646 et 10100,8 kHz. SHIP est émis vers 02:00 et 07:35 UTC, SYNOP vers 00:35 et 12:10. Un bulletin est conservé ici plutôt que dans la liste de décodage, car les chiffres seuls ne veulent rien dire.

weather-nothing-decoded = Rien de décodé pour l'instant. Il n'y a qu'à laisser tourner le RTTY sur l'un des horaires ci-dessus — ce sont les heures les plus calmes de la bande.

weather-head-call-day-hour = { $call } — le { $day } à { $hour }:00 UTC

weather-head-day = à { $day }:00 UTC

weather-head-report = bulletin

weather-not-reported = { $label } : non communiqué

weather-in-their-words = dans leurs mots : { $remark }

weather-ago-now = à l'instant

weather-ago-minute = il y a une minute

weather-ago-minutes = il y a { $m } minutes

weather-ago-hours =
    il y a { $h } { $h ->
            [one] heure
           *[other] heures
        }

weather-ago-days = il y a { $d } jours

sat-off-only-satellites-with = Désactivé : seulement les satellites avec des fréquences connues — ceux que vous pouvez contacter — et ceux que vous avez collés vous-même. Activé : tous les satellites que la station suit, dont la plupart ne sont écoutés par personne.

adsbmap-tune-hint = double-clic pour accorder · maj pour l'autre fréquence

adsbmap-airport-large = grand aéroport · altitude { $elev }

adsbmap-airport-medium = aéroport moyen · altitude { $elev }

adsbmap-from-station = { $dist }  { $bear }° depuis votre station

adsbmap-aircraft-within =
    { $n } { $n ->
        [one] aéronef
       *[other] aéronefs
    } dans un rayon de { $dist }

adsbmap-navaid-mw = { $khz } kHz (ondes moyennes)

adsbmap-click-to-close = cliquez sur la carte pour fermer

settings-ui-broadcast-stations-hint =
    Quelles stations de radiodiffusion la fenêtre ÉCOUTE et la cascade peuvent afficher.
    
    { "**" }Sur cette plage** est ce que faisait chaque écran avant que ce soit un choix : une demi-plage de chaque côté du cadran, ce que doit couvrir une étiquette sur cette cascade. **Toute la bande** affiche tout ce que le programme contient avec une position, ce que cherche un auditeur qui parcourt une bande — au prix d'une carte plus chargée.

settings-ui-waterfall-labels = Étiquettes de la cascade

settings-ui-tuned-only = Accordée seulement

settings-ui-tuned-only-hint = Nommer la station sur laquelle vous êtes accordé, et rien d'autre : un seul nom, pour ce que vous écoutez, plutôt qu'un mur de noms sur la bande passante.

settings-ui-whole-band = Toute la bande

settings-ui-whole-band-hint = Nommer chaque diffuseur que le programme donne en ondes en ce moment, où qu'il soit. Ce qu'on veut en parcourant une bande plutôt qu'en écoutant une station, et plus chargé en conséquence. Aussi dans la fenêtre SPOTS, sous le nom BC ALL.

settings-ui-classic-ui = Interface classique

settings-ui-classic-ui-hint =
    Place tous les contrôles dans une colonne à gauche, avec des groupes repliables, comme le font SDR++ et SDR++Brown, et donne tout le reste à l'image. Mêmes contrôles, mêmes réglages, disposés comme la plupart des auditeurs d'ondes courtes en ont déjà l'habitude.
    
    Il faut une fenêtre assez large pour laisser à la cascade sa part : en dessous, la colonne est omise et la barre est dessinée comme d'habitude ; le panneau s'élargit ou se rétrécit en tirant son bord droit. Désactivé, la barre propre à ce programme est utilisée exactement comme avant.

settings-ui-classic-ui-check = contrôles dans une colonne à gauche

settings-ui-updates = Mises à jour

settings-ui-updates-check = chercher une nouvelle version au démarrage

settings-ui-updates-hint = Interroge une fois au démarrage les Releases GitHub du fork pour connaître la dernière version, et affiche un bandeau si elle est plus récente que cette compilation. Seules les versions complètes comptent (la nightly est une préversion) ; désactivé, aucun appel sortant n'est fait.

settings-ui-undocked = Détachée — sa propre fenêtre

settings-ui-docked = Ancrée dans cette fenêtre

settings-ui-dock-all-hint = Ramène chaque fenêtre détachée dans celle-ci.

settings-ui-undock-all-hint = Place chaque module dans sa propre fenêtre ; la fenêtre principale ne garde que le spectre et les décodeurs. Un module sans rien à afficher, comme le panneau d'exploitation en mode phonie, n'ouvre pas de fenêtre.

settings-ui-spectrum-line = Courbe du spectre

settings-ui-spectrum-line-hint = La couleur sous la courbe du spectre et son ombrage : Fixe est une couleur uniforme, Dégradé est pleine en bas et s'estompe vers le haut, Mixte est pleine dans la moitié basse et s'estompe sur la moitié haute. La courbe elle-même reste cyan. La puce FILL de la fenêtre SPEC active le remplissage, et sa ligne choisit l'ombrage.

settings-ui-fill = Remplissage

settings-ui-fill-colour = couleur

settings-ui-simple-ui = Interface simple

settings-ui-simple-ui-hint =
    Masque les extras avancés de la barre du haut : la vue 3D, les skimmers, les commutateurs de couches spectre/cascade, le suivi des diplômes, les satellites, le décodage ISM et le courrier radio. Les commandes dont un cibiste ou un auditeur d'ondes courtes a besoin (accord, mode, volume, squelch, bande passante, cascade, mémoires, balayage) restent toutes.
    
    Rien n'est désactivé, seulement masqué : réactivez cette option pour retrouver les puces.

settings-ui-simple-ui-check = masquer les puces avancées

settings-ui-wake-lock = Écran maintenu allumé

settings-ui-wake-lock-hint =
    Garde l'écran allumé tant que cette page est au premier plan, pour qu'un téléphone ou une tablette laissé sur la cascade ne s'assombrisse pas et ne se verrouille pas après une minute sans contact. La réception, l'audio et le décodage n'en ont pas besoin — ils continuent dans tous les cas — c'est pour que l'écran reste lisible.
    
    Va de pair avec le cadenas des barres téléphone et tablette : verrouillez l'écran contre les doigts maladroits, et gardez-le allumé.
    
    Le navigateur reprend le verrou dès que la page est masquée (autre onglet, ou bouton d'alimentation du téléphone) et il est redemandé dès que la page revient. Il faut un navigateur qui le gère et une page sécurisée (https, ou localhost) ; la ligne à côté de la case indique quand il n'est pas disponible ou a été refusé. Cela consomme de la batterie, donc c'est désactivé tant que vous ne le demandez pas. Le programme installé sur un ordinateur suit à la place les réglages d'alimentation du système.

settings-ui-wake-lock-check = garder l'écran allumé

settings-ui-sound-welcome = Message de bienvenue sonore

settings-ui-sound-welcome-hint =
    À l'ouverture de la page web (nouvelle session, ou rechargement avec F5), le navigateur ne joue aucun son tant que vous ne touchez pas la page. C'est la règle du navigateur, et sans un mot à l'écran une session toute neuve semble ne pas fonctionner.
    
    Activé, un message au milieu de l'écran l'explique : 🔇 Touchez, cliquez ou appuyez sur une touche n'importe où pour démarrer le son. Le premier clic, toucher ou appui — n'importe où, à la souris, au pavé tactile, au doigt ou au clavier — le ferme et démarre le son, sans rien faire d'autre : il n'accorde pas, n'appuie sur aucun bouton et ne lance aucun raccourci. Il n'apparaît qu'à l'ouverture de la page, jamais plus pendant la session, quoi qu'il arrive ensuite au son.
    
    Désactivez-le quand vous savez quoi faire : le son attend toujours votre premier contact, qui fonctionne alors normalement. Conservé avec votre profil sur le serveur. Activé par défaut.

settings-ui-sound-welcome-check = message de bienvenue pour le son

settings-ui-sound-welcome-browser-only = navigateur uniquement — le programme installé joue le son sans contact

settings-ui-profile-screen = L'écran suit votre profil

settings-ui-profile-screen-hint =
    Lorsque vous êtes client distant d'un `sdroxide --server`, l'*apparence* de cet écran — thème, disposition, cascade et spectre, polices, interface simple, couches des cartes, vue propre du panadapter — est conservée avec le profil sous lequel vous vous êtes connecté, et revient sur n'importe quelle machine au lieu de repartir des valeurs par défaut.
    
    { "**" }Enregistrée sur le serveur, et enregistrée au fil de l'eau tant que vous êtes connecté** — il n'y a rien à presser. C'est voulu : on ne peut pas se fier au stockage du navigateur, car `persist()` se comporte différemment dans chaque navigateur, et une session conservée là ne peut pas être réinitialisée par quelqu'un qui n'est pas devant cet appareil. Un profil sur le serveur est un fichier que l'opérateur de la station peut remettre en place.
    
    Sur un serveur **sans mot de passe**, tous les clients partagent le même profil : rien n'est écrit sauf si vous appuyez sur le bouton ci-dessous — le thème d'un opérateur ne doit pas devenir celui du suivant. Rien de ce qui appartient à la machine n'est jamais stocké, et une radio locale n'est pas concernée.

settings-ui-save-to-profile = Enregistrer dans le profil

settings-ui-back-to-profile = Revenir au profil

settings-ui-carry-bindings = Emporter les raccourcis de commande

settings-ui-carry-bindings-hint =
    Conserve aussi vos raccourcis clavier et souris avec le profil sur le serveur, pour qu'ils suivent la connexion d'un navigateur et d'un appareil à l'autre — même idée que l'écran ci-dessus, pour les touches.
    
    { "**" }DÉCONSEILLÉ, et désactivé par défaut.** Sur une station partagée par plusieurs personnes, le clavier l'est aussi : un profil qui emporte des raccourcis peut réassigner le PTT, la barre d'espace ou les touches d'accord d'un autre opérateur dès qu'il se connecte, et l'activer peut avoir d'autres effets non évidents aujourd'hui. Ne l'activez que si le serveur est à vous seul et que vous y accédez depuis plusieurs de vos appareils.
    
    Propre au fork : ce n'est pas une fonction du projet d'origine. Les réglages d'écran ci-dessus voyagent toujours ; c'est la seule option à activer ici.

settings-ui-carry-bindings-on = activé — DÉCONSEILLÉ

settings-ui-carry-bindings-off = les emporter (déconseillé)

settings-ui-carry-bindings-title = Emporter les raccourcis de commande sur le serveur ?

settings-ui-carry-bindings-body =
    Cela enregistre vos raccourcis clavier et souris avec votre profil sur le serveur, pour qu'ils suivent votre connexion d'un navigateur et d'un appareil à l'autre.
    
    Sur une station utilisée par d'autres, le clavier l'est aussi. Un profil qui emporte des raccourcis peut réassigner le PTT, la barre d'espace ou les touches d'accord d'un autre opérateur simplement en se connectant — et l'activer peut avoir d'autres effets non évidents aujourd'hui.
    
    C'est pourquoi c'est déconseillé. Ne l'activez que si le serveur est à vous seul.

settings-ui-carry-bindings-confirm = J'ai compris — les emporter quand même

settings-ui-start-swl = Démarrer en mode SWL

settings-ui-start-swl-hint =
    Ouvre chaque session avec le mode SWL déjà activé, de sorte que l'écran d'un auditeur soit ce que le programme affiche au démarrage — pour toutes les radios, c'est une préférence d'écran et non d'une radio.
    
    L'interrupteur par radio se trouve dans Réglages → Radio → Commandes d'émission : une station avec une radio d'écoute et un émetteur-récepteur règle celle d'écoute là et laisse ceci désactivé. Utiliser cet interrupteur désactive cette valeur initiale pour la session. `--swl` l'emporte toujours.

settings-ui-start-swl-check = démarrer avec le mode SWL activé

settings-ui-listen-hides-tx = L'onglet ÉCOUTE masque l'émission

settings-ui-listen-hides-tx-hint =
    Choisir **ÉCOUTE** dans le menu bande/mode masque toutes les commandes d'émission de cet onglet, et choisir **EXPLOITATION** remet ce que dit le réglage propre à cette radio.
    
    Cela ne peut pas activer un émetteur. Une radio configurée en réception seule le reste, et le verrouillage de bande et l'avertissement hors bande ne sont pas touchés.
    
    Le menu des bandes se souvient de l'onglet laissé, donc une station laissée à l'écoute revient à l'écoute — les commandes d'émission sont aussi masquées au prochain démarrage, et choisir EXPLOITATION les ramène. Désactivé par défaut car cela change le sens de l'onglet ; il demande confirmation une fois.

settings-ui-listen-hides-tx-check = masquer l'émission dans l'onglet ÉCOUTE

settings-ui-listen-hides-tx-title = L'onglet ÉCOUTE masquera les commandes d'émission

settings-ui-listen-hides-tx-body =
    Dans l'onglet ÉCOUTE, les commandes d'émission disparaissent ; dans EXPLOITATION, elles reviennent exactement comme le réglage propre à cette radio les a laissées.
    
    Cela n'active rien. C'est une vue de la même station : le verrouillage de bande, l'avertissement hors bande et les limites d'émission ne sont pas touchés.
    
    L'onglet est mémorisé : une station laissée sur ÉCOUTE démarre sur ÉCOUTE, avec les commandes d'émission toujours masquées, jusqu'à ce qu'on choisisse EXPLOITATION.
    
    À savoir : une station laissée sur ÉCOUTE reçoit toujours sur ce que l'onglet a accordé, et une commande masquée n'est pas une commande désactivée — l'émission est désactivée parce que les réglages le disent, pas parce que le bouton est hors de vue.

settings-ui-listen-hides-tx-confirm = J'ai compris — les masquer sur ÉCOUTE

settings-ui-swl-mode = Mode SWL

settings-ui-swl-mode-hint =
    Masque toutes les commandes d'émission de cette radio — le PTT, APPEL CQ, le niveau TX, SEND, BEACON, tout — et remplace les extras radioamateurs de la barre (spots, diplômes) par ceux de l'auditeur (PROGRAMME et JOURNAL SWL, le journal de réception). C'est le même interrupteur que Réglages → Radio → Commandes d'émission, par radio : une clé d'écoute peut rester dans ce mode pendant que l'émetteur-récepteur voisin garde son émission.
    
    Le matériel peut toujours émettre ; cela ne fait que masquer les commandes. `--swl` l'impose pour toutes les radios pour la durée de l'exécution.
    
    Prend effet avec Appliquer.

settings-ui-swl-mode-check = masquer toutes les commandes d'émission

settings-ui-report-picture = Image du rapport

settings-ui-report-picture-hint =
    L'image que porte un rapport de réception lorsque vous l'envoyez à un diffuseur : votre propre carte, une photographie, ce que vous voulez montrer à la station. Une image pour la station, utilisée pour chaque rapport.
    
    Laissez-la vide et le rapport part en texte brut, ce qui est un rapport tout à fait valable.

settings-ui-report-picture-none = aucune — le rapport part en texte

settings-ui-choose = Choisir…

settings-ui-report-picture-pick = Choisir l'image sur cet ordinateur

settings-ui-report-note = Note du rapport

settings-ui-report-note-hint = Votre note permanente, préremplie dans chaque rapport de réception : qui vous êtes, sur quoi vous écoutez, tout ce qu'une station demande une fois plutôt qu'à chaque lettre. Texte brut dans un fichier ordinaire : ce n'est pas un secret.

settings-ui-report-note-hint-text = facultatif — p. ex. « Détails sur demande »

sstv-keep-dial = GARDER CADRAN

sstv-keep-dial-on-hint = Activé : choisir SSTV laisse le cadran où vous l'avez réglé, le décodeur fonctionne donc sur n'importe quelle fréquence. Les fréquences SSTV publiées restent dans le sélecteur ⇵. Cliquez pour revenir à l'arrivée sur la fréquence SSTV de la bande.

sstv-keep-dial-off-hint = Désactivé : choisir SSTV déplace le cadran sur la fréquence SSTV de la bande quand vous en êtes à plus de 3 kHz. Cliquez pour garder plutôt le cadran où vous l'avez réglé et décoder la SSTV sur n'importe quelle fréquence. Un réglage fait après avoir choisi SSTV n'est jamais annulé, dans les deux cas.

sstv-clear-rx = Effacer Rx

sstv-clear-rx-hint = Efface l'image actuellement affichée dans la vue LIVE. Le décodeur continue de tourner — utilisez Redémarrer RX pour abandonner une image en cours et attendre le prochain en-tête.

sstv-audio-present-no-header = audio présent · pas encore d'en-tête SSTV

sstv-editor = Éditeur…

sstv-editor-hint = Compose l'image elle-même : texte déplaçable, n'importe quelle police, une bande en travers ou le long du cadre

sstv-save-image-as = Enregistrer l'image sous…

sstv-save-copy-asks-file = Enregistre une copie sur cet ordinateur — demande quel fichier

sstv-picture-still-loading = L'image est encore en cours de chargement

sstv-share-hint-direct = Envoie cette image à quelqu'un — WhatsApp, Telegram ou toute autre application de cet appareil

sstv-share-hint-save = Enregistre cette image et ouvre un message pour l'envoyer

sstv-share-picture = Partager l'image…

sstv-reupload-hint = Renvoie cette image sur l'air — charge l'emplacement d'émission { $slot }

sstv-radio-cannot-transmit = Cette radio ne peut pas émettre

sstv-reupload = Renvoyer

sstv-unidentified = non identifié

sstv-send-picture = Envoyer cette image

sstv-qsl-to = À

sstv-the-card = La carte

sstv-the-card-hint = Ces éléments figurent sur une carte imprimée à côté de l'image. Laissez-en un vide et il n'est tout simplement pas dessiné — une carte ne mentionne jamais un rapport que personne n'a donné.

sstv-date-utc = Date (UTC)

sstv-rst-sent = RST envoyé

sstv-rst-received = RST reçu

sstv-qsl-note = Note

sstv-add-address = Ajoutez une adresse pour l'envoyer.

sstv-saves-opens-mail-hint = Enregistre l'image et ouvre votre client de messagerie — la joindre y demande un clic.

sstv-browser-cannot-attach-hint = Un onglet de navigateur ne peut pas joindre de fichier à un message : cela enregistre donc l'image et ouvre le message ; ajoutez la pièce jointe à la main.

sstv-save-open-mail = Enregistrer l'image et ouvrir la messagerie

sstv-mail-not-opened = Image enregistrée et message copié — collez-le dans votre programme de messagerie. Aucun gestionnaire de messagerie n'a pu être ouvert.

sstv-qsl-saved-where-chosen = { $name } (enregistré là où vous avez choisi)

sstv-qsl-downloaded = { $name } — téléchargé dans le dossier de téléchargement de ce navigateur

sstv-picture-editor = Éditeur d'image

sstv-slot-n = Emplacement { $n }

sstv-slot-layers-hint = Les calques propres à cet emplacement d'émission

sstv-station-overlay = Surimpression de la station

sstv-station-overlay-hint = Un jeu de calques que chaque emplacement dessine sous les siens — l'indicatif, dans le coin de votre choix. Réglé une fois, composé avec le texte propre de chaque emplacement sans le remplacer.

sstv-draw-under-slot = le dessiner sous cet emplacement

sstv-draw-under-slot-hint = Dessine la surimpression de la station sous les calques propres à cet emplacement. Désactivez-la pour une image qui porte déjà l'identification — généralement une mosaïque retransmise — afin qu'elle ne soit pas apposée deux fois.

sstv-overlay-note = La surimpression de la station : chaque emplacement dessine ces calques sous les siens. Faites glisser une ligne sur l'image pour la déplacer.

sstv-slot-text-note = Le texte propre à cet emplacement, dessiné où vous le placez. Faites glisser une ligne sur l'image pour la déplacer ; la bande d'une ligne la suit.

sstv-no-picture-in-slot = Pas d'image dans cet emplacement — chargez-en une ci-dessous

sstv-base = Fond

sstv-base-hint = L'image de quel emplacement sert de fond à ces calques

sstv-compose-over-hint = Composer sur l'image de cet emplacement

sstv-slot-empty = Cet emplacement est vide

sstv-picture-btn = Image…

sstv-load-picture-hint = Charge une image dans cet emplacement

sstv-use-received = Utiliser la reçue

sstv-use-received-hint = Place l'image qui vient d'arriver sous ces calques, afin qu'une réponse porte son image et votre indicatif

sstv-received-now-base = l'image reçue sert maintenant de fond

sstv-add-text = ＋ Texte

sstv-add-strip = ＋ Bande

sstv-n-lines =
    { $n } { $n ->
            [one] ligne
           *[other] lignes
        }

sstv-draw-this-line = Dessiner cette ligne

sstv-strip-paren = (bande)

sstv-draw-later-hint = Dessine cette ligne après celles qui sont au-dessus

sstv-draw-earlier-hint = Dessine cette ligne plus tôt

sstv-copy = Copier

sstv-face = Police

sstv-colour = Couleur

sstv-line-text-hint = ce que dit la ligne

sstv-the-letters = Les lettres

sstv-composer-outline = contour

sstv-the-edge = Le bord qui les entoure

sstv-placement = Placement

sstv-down-frame = le long du cadre

sstv-down-frame-hint = Fait pivoter la ligne d'un quart de tour pour qu'elle se lise vers le bas — un indicatif le long du côté de l'image

sstv-strip = Bande

sstv-behind = derrière

sstv-behind-hint = Une bande de couleur derrière la ligne, en travers de l'image ou le long de celle-ci, centrée sur la ligne

sstv-strip-first-edge = La bande, à son premier bord

sstv-thickness = Épaisseur

adsbpanel-paths = TRACÉS

adsbpanel-paths-hint = Dessiner la trajectoire de chaque vol comme une seule ligne — d'où il vient, où il est, où il va. Désactivé, la trace est dessinée comme une traînée de points.

adsbpanel-nav = NAV

adsbpanel-nav-hint = Aéroports et aides radio à la navigation sous l'image, d'après les tables intégrées — les grands aéroports toujours, les moyens de plus près, et les aides une fois la vue assez serrée pour qu'une station soit un lieu. Les balises en ondes moyennes, celles qu'on peut entendre, sont dessinées dans leur propre couleur.

adsbpanel-units = Unités

adsbpanel-units-hint = Comment l'altitude, la vitesse, le taux de montée et la distance sont écrits dans les vues d'aéronefs. Le décodeur et les exports ne sont pas affectés.

adsbpanel-age = ÂGE

touchlock-locked-hover = Écran verrouillé contre les touches accidentelles. Maintenez ici 3 secondes pour déverrouiller.

touchlock-unlocked-hover = Verrouille l'écran contre les touches accidentelles : fréquence, bande, mode, filtres et cascade ne répondent plus au doigt. La réception et l'audio continuent. Maintenez 3 secondes pour déverrouiller. Après un déverrouillage, le cadenas reste inactif 10 secondes avant qu'une touche le reverrouille, pour que le doigt qui l'a ouvert ne le referme pas aussitôt.

touchlock-hold = MAINTENIR { $s } s…

touchlock-unlocked = DÉVERROUILLÉ

touchlock-locked-hold = VERROUILLÉ · maintenir 3 s

touchlock-unlocked-relock = DÉVERROUILLÉ · verrouillage dans { $s } s

wake-held = écran maintenu allumé

wake-asking = demande au navigateur…

wake-unsupported = indisponible ici — il faut un navigateur qui le gère et une page https

wake-refused = refusé par le navigateur (économie d'énergie ?)

wake-native = navigateur uniquement — le programme installé suit les réglages d'énergie du système

tooltipcopy-copied = ✔ Copié

chrome-hide-message-hover = Masquer ce message pour de bon. Rien ne change dans vos réglages, et le conseil figure dans le manuel.

chrome-close-message-hover = Fermer ce message. Rien d'autre ne change.

settings-ui-window-label = Fenêtre { $module }

settings-ui-undock-hint =
    Détaché, ce module passe dans sa propre fenêtre et la fenêtre principale rend la place à ce qui reste : mettez la cascade sur un deuxième écran, ou les commandes sur un troisième. Fermer sa fenêtre le rattache.
    
    Sous Wayland la taille est restaurée mais c'est le compositeur qui décide de l'emplacement d'une fenêtre : rendez-la flottante et épinglez-la sur un écran avec une règle de fenêtre correspondant à l'app-id « { $app_id } ».

settings-ui-workspace-restore-out = Rétablir cette disposition : { $n } module(s) dans leur(s) propre(s) fenêtre(s)

settings-ui-profile-in-use-named = en cours : le profil { $name }

settings-ui-profile-in-use-default = en cours : la valeur par défaut de cette station

settings-ui-profile-not-in-use = pas encore utilisé : rien n'est enregistré pour cette connexion

settings-ui-module-panadapter = Panadapter

settings-ui-module-controls = Commandes

settings-ui-module-aux = AUX SP (second spectre)

settings-ui-module-band-menu = Sélecteur de bande et de mode

settings-ui-broadcast-scope-in-view = Sur cette plage

settings-ui-fill-fixed = Fixe

settings-ui-fill-gradient = Dégradé

settings-ui-fill-mixed = Mixte

settings-ui-spot-kind-heard-me = ENTENDU

settings-ui-wake-held = écran maintenu allumé

settings-ui-wake-asking = demande au navigateur…

settings-ui-wake-unsupported = indisponible ici : il faut un navigateur qui le gère et une page https

settings-ui-wake-refused = refusé par le navigateur (économie d'énergie ?)

settings-ui-wake-native = navigateur uniquement : le programme installé suit les réglages d'alimentation du système

settings-ui-workspaces = Espaces de travail

settings-ui-workspaces-note = Une disposition de fenêtres nommée : quels modules sont dans leur propre fenêtre et où se trouvent ces fenêtres. Enregistrez l'actuelle ou rétablissez-en une enregistrée. Dix au maximum.

settings-ui-workspaces-none = Aucun espace de travail enregistré.

settings-ui-workspace-restore-home = Rétablir cette disposition : tous les modules dans la fenêtre principale

settings-ui-workspace-forget = Oublier cet espace de travail

settings-ui-workspace-name = nommer cette disposition

settings-ui-workspace-save = ENREGISTRER L'ACTUELLE

settings-ui-workspace-full = Dix espaces de travail, c'est la limite de SDRuno ; supprimez-en un d'abord

settings-ui-workspace-save-hint = Enregistrer la disposition actuelle sous ce nom

settings-ui-workspace-name-first = Saisissez d'abord un nom

sstv-composer-fade = dégradé

sstv-layouts = MISES EN PAGE

sstv-name-layout-hint = nommez cette mise en page

sstv-keep-layout-hint = Conserve cette mise en page sous ce nom

sstv-layout-renamed = renommée en « { $name } »

sstv-layout-saved-as = enregistrée sous « { $name } »

sstv-rename-cancelled = renommage annulé

sstv-no-layouts-yet = Aucune mise en page enregistrée — nommez-en une ci-dessus.

sstv-load-layout-hint = Charge cette mise en page dans ce que vous modifiez

sstv-layout-loaded = « { $name } » chargée

sstv-rename-layout-hint = Renommer « { $name } »

sstv-layout-renaming = renommage de « { $name } »

sstv-duplicate-layout-hint = Dupliquer « { $name } »

sstv-layout-copy-name = { $name } copie

sstv-layout-duplicated = dupliquée sous « { $name } »

sstv-forget-layout-hint = Oublier la mise en page « { $name } »

sstv-layout-forgot = « { $name } » oubliée

sstv-mosaic-header = Mosaïque d'images reçues

sstv-mosaic-nothing = Rien reçu pour l'instant — une mosaïque se construit à partir des images de la galerie.

sstv-mosaic-picked = { $n } sur { $max } choisies, disposées dans cet ordre — l'ordre est la position dans la grille. Chaque image garde sa forme ; les vides sont noirs.

sstv-mosaic-earlier-hint = Déplace cette image d'une case plus tôt

sstv-mosaic-later-hint = Déplace cette image d'une case plus tard

sstv-mosaic-max = { $n } est le maximum d'une mosaïque

sstv-mosaic-compose = Composer dans cet emplacement

sstv-mosaic-compose-hint = Construit la mosaïque et la charge dans cet emplacement comme image ; les originaux reçus restent intacts

sstv-mosaic-composed = { $n } images composées dans l'emplacement { $slot }

sstv-clear-picks = Effacer la sélection

sstv-composed-saved = enregistré

sstv-send = Envoyer

sstv-send-now-hint = Émet cette composition maintenant

sstv-composed-sending = envoi en cours

sstv-forget = Oublier

sstv-forget-hint = Vide le document de ce préréglage — la bande-bannière et la légende simple reprennent la main sur son image

sstv-forget-done = effacé — la bannière et la légende sont de retour

sstv-copy-to-slot = Copier vers l'emplacement

sstv-copy-to-slot-hint = Donne cette image et ce texte à un autre emplacement, pour pouvoir l'envoyer depuis celui-ci

sstv-copied-into-slot = copié dans l'emplacement { $n }

sstv-word-strip = bande

sstv-word-text = texte

sstv-text-size = Taille du texte

sstv-text-size-hint = Le texte de la bannière en pourcentage de la taille propre de la bande. 100 % est ce qu'il a toujours été ; un QSO exige que l'indicatif reste lisible après un décodage bruité, et ceci agrandit les lettres sans épaissir la bande.

sstv-placement-hint = Le bord le long duquel la bande est dessinée. La surimpression du message commence à l'extrémité opposée : une bannière en bas laisse donc le haut de l'image au texte.

sstv-top = Haut

sstv-bottom = Bas

topbar-ptt-hold-hint = Maintenir pour émettre — un clic le verrouille, et l'appui suivant le relâche


topbar-above-the-dial = au-dessus de

topbar-below-the-dial = au-dessous de

topbar-step-chip-hint = De combien un appui déplace le cadran. Touchez pour passer au pas suivant : 10 Hz, 100 Hz, 500 Hz, 1, 2,5, 5, 6,25, 9, 10, 12,5 et 25 kHz. Désactivez cette rangée dans Réglages › Interface.

topbar-mem-chip-hint = Canaux mémoire — rappeler une fréquence enregistrée

topbar-disp-menu-hint = Le panadapter — ses deux couches, leurs vitesses et leur détail, le maintien des crêtes — plus les niveaux de la cascade, les options d'affichage et les skimmers

topbar-phone-menu-hint = Tout ce que la barre du téléphone n'affiche pas : bandes, modes, récepteur et émetteur, fenêtres de décodage, réglages et manuel

topbar-group-band = Bande

topbar-group-system = Système

topbar-caption-diversity = Diversité

topbar-group-decode-windows = Fenêtres de décodage

topbar-group-extras = Extras

topbar-grid-tracker-hint = Suivi des locators — carrés Maidenhead contactés sur une carte

topbar-settings-chip-hint = Réglages — gains de l'appareil, antennes, périphériques audio

topbar-windows-chip-hint =
    ▣  Gauche : ramener toutes les fenêtres de modules
    Droit : la liste des fenêtres de modules

topbar-manual-chip-hint = Manuel d'utilisation (F1)

topbar-module-windows-title = Fenêtres de modules

topbar-wname-panadapter = SP1 — le spectre

topbar-wname-aux = SP2 (AUX) — un second spectre

topbar-wname-panel = Panneau d'exploitation

topbar-wname-controls = Commande RX

topbar-wname-band-menu = Bande et mode

topbar-wname-rotor = Rotor d'antenne

topbar-wstate-put-back = Remettre { $name } dans cette fenêtre

topbar-wstate-floating-panel-only = Flottant est un état propre au panneau d'exploitation — un spectre ou une console n'a qu'une place dans cette fenêtre

topbar-wstate-float = Sortir { $name } du volet et le faire flotter au-dessus de cette fenêtre

topbar-wstate-no-panel = Pas de panneau d'exploitation en { $mode } — cela s'applique dans un mode qui en a un

topbar-wstate-own-window = Donner à { $name } sa propre fenêtre, sur n'importe quel écran

topbar-wstate-here-now = { $name } est ici actuellement

topbar-dock-all = TOUT ANCRER

topbar-dock-all-hint = Ramener toutes les fenêtres de modules dans cette fenêtre

topbar-undock-all = TOUT DÉTACHER

topbar-undock-all-hint = Mettre chaque module dans sa propre fenêtre — cette fenêtre ne garde que le spectre et les décodeurs

topbar-rx-tx-freq = RX { $rx }   TX { $tx } MHz

topbar-band-dock-hint = DOCK — garder le sélecteur de bande et de mode ouvert à côté de la cascade au lieu de fermer cette fenêtre à chaque fois

topbar-band-window-hint = WINDOW — ouvrir le sélecteur de bande et de mode dans sa propre fenêtre, aussi large que vous voulez, sur n'importe quel écran à côté de la cascade

topbar-band-return-popup-hint = DOCK — remettre le sélecteur dans la fenêtre de la barre du haut, où il se ferme après chaque choix

topbar-band-window2-hint = WINDOW — ouvrir le sélecteur dans sa propre fenêtre, aussi large que vous voulez, sur n'importe quel écran

topbar-squelch-held-hint = Le squelch est maintenu ouvert. Cliquez pour remettre le seuil que vous aviez.

topbar-squelch-hold-hint = Cliquez pour maintenir le squelch ouvert — pour entendre s'il y a quelque chose — et cliquez à nouveau pour le remettre exactement comme il était.

topbar-changed-tone = tonalité

topbar-changed-loudness = LOUDNESS

topbar-nr-chip-hint =
    Réduction de bruit : { $strength } — { $engine }
    
    { $advice }
    
    Cliquez pour changer la force : Faible / Moyenne / Forte / Off. Le moteur se choisit dans la rangée du dessous.

topbar-nr-off-narrow-hint = La réduction de bruit est désactivée. Cinq débruiteurs sont proposés dans la rangée du dessous — RNNoise, DeepFilterNet3, spectral-bleach, NR2 et le spectral d'origine — et un clic ici l'active à une certaine force. Commencez par RNNoise sur la voix.

topbar-eq-tone-hover = Tonalité : graves { $bass } dB, médiums { $mid } dB, aigus { $treble } dB

topbar-eq-closed-hover = Tonalité — un égaliseur trois bandes sur l'audio reçu. Cliquez pour ouvrir

topbar-eq-loudness-on = LOUDNESS activé

topbar-stereo-yields-hint = La stéréo WFM est coupée tant que la réduction de bruit ou le notch automatique fonctionne — ils brouilleraient les deux canaux. Désactivez-les pour entendre la stéréo

topbar-stereo-playback-hint = Stéréo : l'enregistrement est lu sur les deux oreilles, tel qu'enregistré. Cliquez pour forcer le mono

topbar-stereo-mono-recording-hint = Cet enregistrement est mono — il n'y a pas de second canal à lire

topbar-preset-edges = { $lo } … { $hi } Hz

topbar-caption-tone = Tonalité

topbar-eq-on = activé

topbar-eq-bass = Graves

topbar-eq-mid = Médiums

topbar-eq-treble = Aigus

topbar-eq-remembered = Mémorisé pour { $mode } — chaque mode garde le sien.

topbar-loudness-hint = Renforce les graves (et un peu les aigus) quand vous baissez le volume, pour qu'un haut-parleur faible sonne quand même plein. Fonctionne avec ou sans la tonalité ; sans effet à plein volume et ne joue jamais plus fort que le plein volume. Désactivé par défaut, et mémorisé pour le mode en cours.

topbar-caption-record-audio = Enregistrer l'audio

topbar-stop-recording-after-min = Arrêter l'enregistrement après { $minutes } minutes

topbar-caption-play-back = Relecture

topbar-recordings-chip = ▶ ENREGISTREMENTS…

topbar-recordings-hint = Rejouer un enregistrement dans les décodeurs — une image SSTV manquée, un signal RTTY ou CW faible, un créneau FT8 — avec la cascade et le journal de décodage, comme s'il était à l'antenne. La radio se met en retrait pendant la lecture.

topbar-ant-label = ANT

topbar-pwr-label = ALIM

topbar-drive-less = moins

topbar-drive-more = plus

topbar-fill-hint = Colorie la zone sous la courbe du spectre, pleine en bas et s'estompant en montant vers la courbe, pour qu'un signal ressorte par sa masse autant que par son contour. La couleur se choisit dans Réglages → Interface → Courbe du spectre. Non dessiné sur la surface 3D.

topbar-group-spectrum = Spectre

topbar-group-waterfall = Cascade

topbar-freeze-on-tx-hint = Arrête le défilement de la cascade pendant votre propre émission et le reprend là où il s'était arrêté — l'image est ainsi un historique reçu continu plutôt qu'un bloc de lignes montrant votre porteuse (ou, sur une radio qui coupe son récepteur pendant l'émission, des lignes de bruit qui n'ont jamais existé). La courbe du spectre n'est pas touchée. Utile pour suivre un mode où vous émettez souvent, afin que la transmission de l'autre station ne soit pas chassée de l'écran par la vôtre.

topbar-fft-chip-hint = Niveaux et contraste de la cascade, taille FFT et sens de défilement

topbar-caption-waterfall = Cascade

topbar-flatten-hint = Aplanit le plancher de bruit sur la bande : un tronçon plus calme que ses voisins est relevé vers leur plancher, pour qu'un signal faible se voie sur une bande chargée. Cela ne fait que relever — une porteuse n'est jamais abaissée — et jamais de plus de 20 dB. La courbe du spectre au-dessus reste exactement telle que mesurée.

topbar-log-swl-hover = Journal de réception — stations entendues, avec SINPO/SIO

topbar-programs-hint = Grille des émissions — ce qui passe, quand et où

topbar-listen-hint = Ce qui est dans cette plage, sur une carte — choisissez une station, accordez-vous, notez ce que vous avez entendu et consignez-le

topbar-spots-hint = Spots en direct — PSK Reporter (et, hors mode SWL, le DX cluster, POTA et SOTA)

topbar-sigid-hint = Qu'y a-t-il sur ce cadran ? Un guide des signaux par fréquence, mode et largeur de bande

topbar-extras-hint = EXTRAS — le DAB et les autres récepteurs, dans une fenêtre détachable

topbar-fill-style-caption = courbe

topbar-fill-style-hint = Comment la couleur sous la courbe du spectre est dégradée. La couleur elle-même se choisit dans Réglages → Interface → Courbe du spectre.

topbar-fill-fixed = FIXE

topbar-fill-fixed-hint = Une seule couleur, la même partout sous la courbe

topbar-fill-gradient = DÉGRADÉ

topbar-fill-gradient-hint = Plein en bas du spectre, s'estompant vers le haut

topbar-fill-mixed = MIXTE

topbar-fill-mixed-hint = Plein dans la moitié basse, s'estompant sur la moitié haute

topbar-fill-switch-on = Activez FILL pour régler ceci

topbar-nothing = rien

topbar-olivia-unconfirmed = OLIVIA — la réception fonctionne, l'émission n'a pas été testée avec d'autres programmes. Ce décodeur lit d'autres stations (confirmé à l'antenne), vous pouvez donc copier un QSO Olivia. Notre propre émission passe maintenant par notre propre récepteur, mais elle n'a pas été copiée par fldigi ni MultiPSK et une convention de signe diffère de la source de fldigi — vérifiez donc le premier échange plutôt que d'attendre une réponse.

topbar-dab-bandwidth = DAB / DAB+, éprouvé à l'antenne. Il faut 2,5 Msps de bande passante en entrée avant d'avoir le moindre son : aux 1,536 MHz qu'occupe un multiplex, le récepteur est au plancher de son CAN et perd des échantillons, et un décodage DAB exige un flux continu, donc le multiplex ne s'accroche jamais et vous n'entendez rien. Réglez d'abord la fréquence d'échantillonnage de la radio sur 2,500 Msps.

topbar-stanag-unproven = STANAG 4285 — réception seule. L'identification est éprouvée à l'antenne : un 4285 s'accroche, sa trame de 106,67 ms est mesurée et ses blocs de référence s'effondrent, le panneau peut donc dire ce qu'il y a sur la fréquence et à quel débit. La lecture de son *texte* n'est pas éprouvée à l'antenne : le code et l'entrelaceur de l'Annexe E décodent exactement un signal généré, mais aucune station codée n'a encore été copiée. Un texte codé qui apparaît est réel ; l'absence de texte signifie que la station n'est pas codée ou transporte du trafic chiffré, ce qui est le cas habituel.

topbar-cis11-unproven = CIS-11 (TORG-11) — réception seule. Le téléscripteur météorologique russe : 2-FSK à 100 bauds, environ 500 Hz de décalage, et un caractère de 11 bits avec un bit de parité impaire. Ce qui est prouvé a été mesuré sur de vrais enregistrements : le récepteur trouve ses propres tonalités et lit une émission au repos avec 121 caractères sur 123 de parité valide. Ce qui n'est pas réglé, c'est la façon dont les dix bits de données se répartissent en caractères ; le panneau montre donc les bits en plus de sa lecture ITA-2 — et le trafic lui-même est chiffré, ce qui est le cas habituel.

topbar-thor-unproven = THOR — la réception ne lit pas encore les émissions de fldigi. Le décodeur fonctionne avec son propre modulateur mais aucun enregistrement à l'antenne ou de référence n'a été copié ; considérez donc ce qui apparaît comme provisoire.

topbar-mode-not-on-band = { $mode } n'est pas utilisé sur { $band } — choisissez une bande qui lui convient

topbar-radio-classic-hint = RADIO CLASSIC — une façade de radio classique pour l'écoute des émissions. L'activer lui donne le récepteur : les autres modes et leurs fenêtres sont fermés

topbar-ats-bands-caption = Bandes proposées par le récepteur

topbar-ats-band-hover = { $name } — { $mhz } MHz, { $mode } par défaut. Amène le récepteur sur cette bande.

topbar-caption-mode = Mode

topbar-caption-digital = Numérique

topbar-ats-receive-only = Réception seule — le Si4732 démodule en matériel et il n'y a rien à commander.

topbar-band-forecast-derived =
    { $band } : ≈{ $verdict } ({ $time ->
            [day] jour
           *[night] nuit
        }) — verdict du groupe { $group } publié, le substitut le plus proche ; HAMQSL.com ne note rien pour { $band } lui-même. Une prévision, pas une mesure de votre propre trajet.

topbar-keypad-readout-khz = { $digits } kHz

topbar-caption-band-keypad = Clavier de bandes

topbar-keypad-bands = Bandes

topbar-keypad-bands-hint = Les touches désignent des bandes : appuyez sur l'une pour y aller, en restant dans le mode actuel. Revenir de MHz efface ce qui a été saisi.

topbar-keypad-mhz-hint = Saisissez une fréquence sur les touches, en kilohertz : 14074 vaut 14,074 MHz. ENTER l'envoie, CLEAR vide la saisie.

topbar-keypad-key-stay = Rester en { $mode } et s'accorder sur { $mhz } MHz — où il se pratique sur { $band }.

topbar-keypad-key-go = Aller sur { $band }.

topbar-keypad-clear = Effacer

topbar-keypad-clear-typing-hint = Vider la fréquence saisie. Les touches restent en MHz.

topbar-keypad-clear-bands-hint = Rien n'est saisi — les touches sont sur les bandes, où un appui mène à une bande.

topbar-keypad-enter = Entrée

topbar-keypad-enter-empty-hint = Rien de saisi pour l'instant. Les touches sont en MHz — appuyez d'abord dessus.

topbar-keypad-enter-ok-hint = S'accorder sur { $mhz } MHz et revenir aux touches de bandes.

topbar-keypad-enter-range-hint = { $mhz } MHz est hors de ce que cette radio reçoit ({ $ranges } MHz).

topbar-nothing-published = rien de publié

topbar-tab-listen = ÉCOUTE

topbar-tab-operate = EXPLOITATION

topbar-listen-tab-note = Cet onglet peut aussi masquer les commandes d'émission : activez « masquer TX sur l'onglet ÉCOUTE » dans Réglages → Interface

topbar-caption-show-bands = Afficher les bandes

topbar-band-filter-hint = N'afficher que les bandes { $filter } dans la liste ci-dessous. Cliquez à nouveau pour afficher toutes les bandes.

topbar-show-every-band = tout afficher

topbar-all-chip = TOUT

topbar-all-chip-hint = Couverture générale : effacer la bande et laisser le cadran aller n'importe où. Différent du filtre de bandes, qui ne fait que réduire cette liste.

topbar-caption-sw-metre-bands = Bandes métriques OC

topbar-metre-band-hint = S'accorder au milieu de la radiodiffusion { $name }

topbar-caption-wideband = Large bande

topbar-caption-cb = 11M/CB

topbar-caption-experimental = Expérimental (non éprouvé)

topbar-caption-receive-modes = Modes de réception

topbar-wav-hint = Le même audio en PCM 16 bits non compressé à 48 kHz — environ 11 Mo par minute en stéréo. Pour un éditeur ou un décodeur de votre choix.

topbar-mode-offset-hint-side = Ce mode travaille le signal { $side } le cadran. Le nombre est le cadran, car c'est ce que vous accordez ; le signal est à { $mhz } MHz.

levelslider-hover =
    Niveau de la cascade
    plancher { $floor } dB · plafond { $ceil } dB
    
    Glisser ou faire défiler pour éclaircir ou assombrir ; Maj pour le contraste.
    Le déplacer désactive l'ajustement auto (FIT).

smeter-audio-level-hover =
    Niveau audio : { $dbfs } dBFS{ $over }.
    
    Cette radio n'a **pas de S-mètre propre à signaler** ; ceci est donc le niveau de l'audio qu'elle envoie — mesuré ici, avec le CAG et le squelch de l'appareil déjà appliqués. Il suit l'audio, pas la HF : du bruit sur la carte son, une souris sans fil près de l'appareil ou le pompage du CAG le font bouger alors que le signal n'a pas changé.

smeter-audio-over = (S9 + { $over } dB sur l'échelle audio)

spectrum-offset-left = ◀ tonalités { $hz } Hz

spectrum-offset-right = { $hz } Hz tonalités ▶

worldmap-no-snr = pas de SNR

solar-tune-the-radio-to-this-downlink = Accorder la radio sur cette liaison descendante

qo100-width-label = largeur

qo100-park-label = parking

qo100-led-carrier = porteuse

qo100-led-sync = synchro

scanner-stopped = arrêté

ism-band-label = bande

amp-none = aucun

colormap-classic = Classique

colormap-viridis = Viridis

colormap-gray = Gris

colormap-icom = Icom

colormap-neon = Néon

colormap-synthwave = Synthwave

colormap-matrix = Matrix

colormap-tron = Tron

colormap-amber = Ambre

colormap-rainbow = Arc-en-ciel

colormap-blue = Bleu

colormap-calmer = Plus calme

colormap-compline = Complies

colormap-lauds = Laudes

settings-cb-wide-calls-hint =
    Accepte la forme d'indicatif 11 m plus large { $wide } — trois lettres au milieu, jusqu'à quatre chiffres après — en plus de { $narrow } de WSJT-CB. Expérimental : WSJT-CB lui-même rejette la forme plus large, donc un indicatif qui en a besoin peut ne pas être compris par une station WSJT-CB ; cela élargit ce que cette station entend plus que ce qu'elle peut émettre de façon fiable. Désactivé, la syntaxe exacte de WSJT-CB est conservée.
    
    Cette option change la *forme*, pas la *longueur* : un indicatif 11 m fait toujours 11 caractères, suffixe /P, /MM ou /QRP compris, donc 19DCG3733/P passe et 19TST1001/QRP non.

settings-idle-stop-disable = AUCUN ARRÊT

settings-xvtr-tx-off-hint =
    Désactivé pendant la conversion : réception seule — la ligne est un convertisseur et non un transverter, et rien n'est émis sur cette bande.
    
    Par le même convertisseur : il fonctionne dans les deux sens et l'émission prend le même décalage.

settings-xvtr-drive-hint =
    Un plafond de puissance d'excitation à l'émission tant que ce transverter est sélectionné, en pourcentage du maximum. L'entrée F.I. d'un transverter accepte des milliwatts et l'excitation adaptée aux bandes propres de la radio la détruirait : c'est donc le champ le plus important de la ligne. 100 % signifie aucune limite.
    
    Votre réglage de puissance est maintenu en dessous plutôt que modifié, de sorte que la valeur utilisée en HF revient quand le cadran quitte la bande.

settings-brown-build-note = La version Brown de sdroxide : le programme d'origine, plus la bande 11 m (CB), l'écoute des ondes courtes et leur décodage. Fonctionne comme une station radioamateur ordinaire avec ces bandes et ces outils en plus.

settings-gain-by-band = Mémoriser le gain d'entrée par bande

settings-gain-by-band-hint =
    Garde un gain de réception distinct pour chaque bande et le rétablit quand le cadran passe dans cette bande. Réglez une fois votre gain HF et une fois votre gain 11 m ou VHF ; la radio suit la bande.
    
    Désactivé par défaut. Désactivé, un seul gain s'applique partout, exactement comme avant. L'activer enregistre la bande où vous êtes, de sorte qu'aucune bande n'en soit dépourvue ; une bande jamais réglée garde le gain actuel.

settings-callsign-grid-note = Votre indicatif et votre locator, partagés entre FT8/FT4/FT2, les en-têtes d'images SSTV, les modes numériques CB et le journal de trafic. C'est l'identité qui **émet** — un indicatif CB si vous travaillez en 11 m, un indicatif radioamateur sinon. Pour ce dont est signé un **rapport de réception**, voir le champ **Rapport signé (SWL)** de l'onglet Spots. Modifiable aussi depuis la boîte de réglage FT8 / SSTV.

settings-cb-grammar = Syntaxe des indicatifs 11 m / WSJT-CB

settings-cb-wide-calls = INDICATIFS CB ÉTENDUS — EXPÉRIMENTAL

settings-cb-wide-calls-note = Activez si votre communauté 11 m utilise des groupes de trois lettres ou des numéros d'unité à quatre chiffres. S'applique au décodage et à l'émission des modes numériques CB. Dans les deux cas, un indicatif 11 m ne doit pas dépasser **11 caractères**, suffixe compris — 19DC3733/P passe, 19TST1001/QRP non.

settings-cb-plan-hint = Le plan de canaux dans lequel le cadran lit : les plans de la bande 11 m (CB) et ceux du PMR446 en 446 MHz. Seuls les numéros de canaux et le canal d'ouverture de la bande changent — les limites de bande restent en place, donc changer ne déplace jamais ce qui reçoit ou émet. Le numéro de canal est affiché sur la ligne d'accord chaque fois que le cadran est sur un canal du plan.

settings-cb-allow-tx = Autoriser l'émission en 11 m (CB)

settings-cb-allow-tx-hint = Le verrouillage des bandes amateur refuse l'émission sur toute bande qui n'est pas une attribution radioamateur, et le 11 m en est une : c'est un service radio distinct, avec ses propres règles et son propre matériel homologué. L'activer autorise l'émission là — et nulle part ailleurs ; les bandes de radiodiffusion restent en réception seule. Une confirmation vous sera demandée la première fois.

settings-cb-licence-note = La CB est sans licence dans de nombreux pays, mais pas tous, et les canaux, modes et puissances autorisés diffèrent. L'usage de la bande est soumis aux règles du pays où vous vous trouvez.

settings-radio-callsign-hint =
    L'indicatif sous lequel cette radio émet, enregistre et diffuse ses spots. Laissez vide pour utiliser l'indicatif de la station de l'onglet Général — le défaut, adapté à une station à identité unique.
    
    Renseignez-le sur une radio qui est quelqu'un d'autre : un indicatif CB sur le poste 11 m, un indicatif radioamateur sur le poste HF, chacun gardant le sien. L'indicatif de l'onglet Général reste la valeur de repli pour toute radio qui ne le remplace pas.
    
    Les *rapports* de réception peuvent toujours être signés avec le numéro SWL de l'onglet Spots, ce que ceci ne change pas.
    
    Prend effet avec Appliquer.

settings-radio-swap-iq-own = Cette interface a sa propre inversion I/Q dans sa section plus bas sur cette page : utilisez celle-là.

settings-radio-swap-iq-hint =
    Inverse l'I/Q de ce récepteur avant la démodulation. À utiliser quand I et Q d'un récepteur sont inversés et que la BLU sort sur la mauvaise bande latérale (LSB entendue comme USB), avec le spectre en miroir autour du cadran.
    
    Une propriété du matériel, pas de la personne connectée.
    
    Prend effet avec Appliquer.

settings-radio-transmit-controls = Commandes d'émission

settings-radio-hide-tx-all = tout masquer (mode SWL)

settings-radio-hide-tx-hint =
    Masque toutes les commandes d'émission de *cette* radio — le PTT, APPEL CQ, le niveau TX, SEND, BEACON, tout — et remplace les extras radioamateurs de la barre (spots, diplômes) par ceux de l'auditeur (PROGRAMME et JOURNAL SWL, le journal de réception). Par radio : une clé d'écoute sur le long fil peut rester dans ce mode pendant que l'émetteur-récepteur voisin garde son émission.
    
    Le matériel peut toujours émettre ; cela ne fait que masquer les commandes. `--swl` l'impose pour toutes les radios pour la durée de l'exécution.
    
    Prend effet avec Appliquer.

settings-radio-auto-mode = Mode auto

settings-radio-auto-stop-label = arrêt sans action de l'opérateur

settings-radio-auto-stop-hint-long =
    Durée pendant laquelle le séquenceur FT8/FT4/FT2 sans surveillance peut tourner sans action de l'opérateur avant de se désarmer. Par radio : un poste peut tourner plus longtemps qu'un autre.
    
    Sur un PlutoSDR, cela monte à six heures, et AUCUN ARRÊT supprime tout arrêt : le mode auto tourne alors jusqu'à ce que vous l'arrêtiez. Le chien de garde d'émission et le verrouillage de bande s'appliquent toujours.
    
    Prend effet avec Appliquer.

settings-radio-auto-stop-hint =
    Durée pendant laquelle le séquenceur FT8/FT4/FT2 sans surveillance peut tourner sans action de l'opérateur avant de se désarmer. Par radio : un poste peut tourner plus longtemps qu'un autre.
    
    Plafonné à 45 minutes volontairement — le mode auto sert pour une courte pause, pas pour laisser une station faire un concours sans personne derrière.
    
    Prend effet avec Appliquer.

settings-radio-smeter-calib = Étalonnage du S-mètre

settings-tx-eq = Égaliseur d'émission

settings-tx-eq-hint = Un égaliseur paramétrique à 3 bandes sur l'audio du microphone, avant le modulateur. Modes phonie seulement (BLU/AM/FM) ; les modes numériques et la CW transportent un audio synthétisé ou manipulé qui ne l'atteint jamais. Désactivé par défaut et plat une fois activé : l'activer ne change rien à l'antenne tant qu'une bande ci-dessous n'est pas réellement déplacée. S'applique immédiatement.

settings-tx-eq-low-hint = Atténue/renforce les grondements et bruits de manipulation.

settings-tx-eq-mid = Pic médium

settings-tx-eq-mid-hint = Présence : une étroite bosse ou un creux quelque part dans la bande vocale.

settings-tx-eq-high-hint = Brillance / désibilisation.

settings-cb-call-too-long = ⚠ Trop long pour le 11 m — { $len } caractères, et le FT8 en transporte 11 pour un indicatif CB, suffixe /P, /MM ou /QRP *compris*. Cet indicatif d'activation ne peut pas être émis. 19DC3733/P (10) passe ; 19TST1001/QRP (13) non.

settings-cb-plan-note = Le 11 m (CB) n'est pas la même bande partout. Ceci choisit les canaux de quel pays le cadran 11 m lit : { $plan }. Seuls changent les canaux et le canal d'ouverture de la bande — les limites de bande restent larges, donc changer de plan ne modifie jamais ce qui reçoit ou émet. S'applique à toutes les radios de cette station.

settings-cw-enter-mode = Passez une fois en CW (ou dans un mode numérique) pour charger les valeurs enregistrées.

settings-cw-where-from = Source :

settings-cw-straight-key-hint = L'assignation `Manipulateur droit CW` (Espace par défaut) est lue tant que KEY est activé dans le panneau CW. Un seul contact : c'est un manipulateur droit — l'application décode le tempo propre à l'opérateur.

settings-cw-speed = Vitesse :

settings-cw-speed-note = la longueur des points et des traits du manipulateur

settings-cw-key-tx = COMMANDER L'ÉMETTEUR

settings-cw-key-tx-hint = Désactivé : la clé ne pilote que l'entraîneur local. Activé : elle commande la radio par le chemin ordinaire de la manipulation manuelle, avec le verrouillage de bande et le chien de garde autour.

settings-cw-key-self-note = Pour un poste qui se manipule lui-même, réglez Réglages → Radio → Manipulation CW sur `Carte son (MCW)` : la manipulation manuelle est refusée par conception sur la voie du manipulateur interne.

settings-detachable-windows = Fenêtres détachables

settings-detachable-windows-note = Sortir un panneau dans sa propre fenêtre du système. Fermer la fenêtre le rattache.

settings-tool-windows = Fenêtres d'outils

settings-tool-windows-note = Panneaux convertis en fenêtres flottantes. Leur état ouvert est mémorisé.

settings-tool-window-scanner = SCANNER

settings-tool-window-memories = MÉMOIRES

settings-tool-window-playback = LECTURE

settings-tool-window-open = Ouverte

settings-tool-window-closed = Fermée

settings-detachable-native-only = Les fenêtres détachables ne sont disponibles que dans l'application native.

settings-psk-reporter-band-hint = Le flux d'activité de la bande : les stations entendues sur la bande actuelle, dessinées en points colorés. Active aussi la requête « qui m'a entendu » derrière la puce HEARD ME de la carte, relancée chaque heure.

settings-psk-upload-hint = Signale à pskreporter.info ce que cette station entend, pour qu'elle y apparaisse comme récepteur. Utilise l'indicatif et le locator de l'onglet Général, ou le numéro SWL ci-dessous s'il est défini.

settings-swl-report-as = Rapport signé (SWL)

settings-swl-report-as-hint = L'identité propre de l'auditeur pour les rapports de réception — un numéro SWL enregistré, un numéro de club, un nom. Quand elle est définie, elle signe ce que cette station *entend* (envois PSK Reporter et WSPRnet, et rapport de réception que le journal SWL recopie) à la place de l'indicatif. Laissez vide pour signer avec l'indicatif. Elle n'est jamais émise, ne commande jamais une radio et ne nomme jamais un QSO ou un spotteur, donc un auditeur sans indicatif peut aussi faire des rapports.

settings-wsjtcb-spot-server = Serveur de spots WSJT-CB

settings-wsjtcb-report = Signaler mes décodages 11 m au serveur de spots WSJT-CB

settings-wsjtcb-report-hint = Envoie chaque spot décodé sur la bande 11 m au service de spots de la communauté CB (xzgroup.net) — le même serveur auquel le client WSJT-CB peut publier. Désactivé par défaut : c'est un service tiers et rien n'est envoyé tant que vous ne le demandez pas. Votre indicatif est le spotteur ; PSK Reporter est l'interrupteur séparé ci-dessus.

settings-eqsl-qth-hint = Dans quel profil QTH un envoi est classé — nécessaire seulement quand le compte en possède plusieurs, à laisser vide sinon.

settings-log11dx-api-url = URL de l'API

settings-log11dx-api-token = Jeton d'API

settings-log11dx-note = Les deux proviennent de la page de votre profil LOG11DX. Son propre pont WSJT-X n'est pas nécessaire : sdroxide enregistre le QSO et l'y envoie directement. Le site masque le jeton, copiez-le donc au moment de le générer — il est stocké dans net.json comme les autres identifiants.

settings-pan-attached-note =
    Son spectre, sa cascade et tout ce qui s'en lit sont sur l'onglet de cette radio, d'où l'absence d'onglet propre. Les réglages d'interface ci-dessus restent ceux de cette radio et s'appliquent toujours — appuyez sur Appliquer / reconnecter après les avoir modifiés et les deux radios redémarrent ensemble.
    
    Pour l'utiliser de nouveau seule, videz le champ Récepteur du panadapter sur la page de cette radio.

settings-pan-source-hint =
    Quelle radio reçoit pour celle-ci. Elle quitte la barre d'onglets tant qu'elle est prêtée — son étage d'entrée appartient au moteur de cette radio — et revient aussitôt quand ceci est réglé sur Aucune.
    
    Prend effet avec Appliquer.

settings-pan-offset-hint =
    De combien le récepteur se situe au-dessus du cadran de cette radio, en hertz. 0 pour un récepteur sur la même antenne ; la fréquence intermédiaire pour une prise F.I. — 9000000 pour une F.I. de 9 MHz, 70455000 pour une de 70,455 MHz.
    
    Ce n'est pas le champ Convertisseur ci-dessus, qui réaccorde la radio, ni le décalage du centre I/Q de l'onglet CAT, qui concerne la carte son propre à cette radio. Rien de ce qui est saisi ici n'est jamais envoyé à l'une ou l'autre radio.
    
    Prend effet avec Appliquer.

settings-cw-device = Périphérique :

settings-cw-device-none = (aucun trouvé)

settings-cw-device-auto = auto (premier nommé « key »)

settings-cw-reverse-hint = Inverse points et traits, pour l'interrupteur du manipulateur lui-même.

settings-cw-key-type = Type de manipulateur :

settings-cw-key-straight = DROIT

settings-cw-key-iambic-a = IAMBIQUE A

settings-cw-key-iambic-b = IAMBIQUE B

settings-cw-usb-note = Un boîtier de manipulateur qui signale ses deux contacts (souvent comme boutons de souris) fonctionne ; un boîtier qui fait lui-même l'iambique et envoie des touches finies ne fonctionne pas. Tant que la clé est armée, le périphérique est pris en exclusivité, de sorte que ses contacts ne cliquent pas dans d'autres fenêtres.

settings-cw-usb-linux = Un manipulateur USB nécessite un bureau Linux.

settings-smeter-zero = 0 (tel que mesuré)

settings-smeter-cal-hint =
    Corrige le S-mètre de cette radio quand il lit trop haut ou trop bas, par pas de 6 dB (unités S).
    
    Une valeur négative abaisse la lecture — pour un instrument qui indique trop, un plancher de bruit à S5 là où il devrait être à S2. Une valeur positive la relève. 0 est l'instrument tel que mesuré.
    
    Seule la lecture bouge : l'aiguille, les valeurs S et dBm, le niveau dont part un rapport de réception et le S-mètre parlé. Le signal, le squelch, l'enregistrement et les décodeurs ne sont pas touchés.
    
    Enregistré avec cette radio sur la machine à laquelle elle est reliée, et pris en compte immédiatement.

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1 et 2 utilisent tous deux { $host }:{ $port } — chaque spot arrivera deux fois. Donnez-leur des ports différents (7000 CW/RTTY, 7001 FT8/FT4) ou désactivez-en un.

settings-rx-site-station = À la station

settings-rx-site-elsewhere = Ailleurs

settings-remote-tab-note =
    La station apparaît comme un onglet à côté des radios déjà ouvertes ici, et la barre d'onglets en haut de la fenêtre permet de passer de l'une à l'autre (⊞ en place deux côte à côte). Fermez-la depuis la liste en haut de l'onglet Radio : cela raccroche ; rien n'est modifié sur le serveur.
    
    Si le serveur demande un nom d'utilisateur et un mot de passe, son écran de connexion s'affiche dans le nouvel onglet. Un lien ws:// simple ne chiffre rien de tout cela : sur l'internet ouvert, passez donc par un VPN ou un tunnel SSH, ou terminez le HTTPS devant le serveur et cochez Sécurisé ci-dessus.

settings-cw-source-keyboard = CLAVIER

settings-cw-source-usb = PADDLE USB

pane-frame = TRAME

pane-frames = TRAMES

pane-words = MOTS

antenna-feed-z0 = Z0 ligne :

antenna-feed-z0-hint = Impédance de la ligne/du système pour le ROS (50 Ω typique)

antenna-height-label = Hauteur :

antenna-height-hint = Hauteur du point d'alimentation/de l'antenne au-dessus du sol (influe sur l'angle de départ et le diagramme)

antenna-m-agl = m sol

antenna-apply = Appliquer

antenna-apply-hint = Lance le solveur par méthode des moments sur les préréglages.

antenna-yagi-optimizer = Optimiseur Yagi

antenna-stack = Empilement :

antenna-objective = Objectif :

antenna-optimize = Optimiser

antenna-swr-value = ROS { $v }

antenna-swr-ref = ROS { $v } ({ $z }Ω)

antenna-radiation-pattern = Diagramme de rayonnement

antenna-view = Vue

antenna-view-front = Face

antenna-view-side = Côté

antenna-view-top = Dessus

antenna-3d-hint = 3D : glisser pour tourner • chaleur = intensité du champ

antenna-yagi-x = Yagi ×{ $n }

settings-tool-window-session-qsos = QSO DE SESSION

sessionqso-time = Heure

sessionqso-call = Indicatif

sessionqso-grid = Locator

sessionqso-band = Bande

sessionqso-mode = Mode

sessionqso-rst-s = RST E

sessionqso-rst-r = RST R

sessionqso-country = Pays

sessionqso-dist = Dist

sessionqso-final = Final

sessionqso-count = { $n } QSO cette session

sessionqso-include-cw = Inclure le CW

decodes-live-log = JOURNAL EN DIRECT

decodes-setup = ⚙ RÉGLAGES

schedule-colour-all = Couleur

schedule-colour-all-hint = Couleur des étiquettes de la cascade pour toutes les stations — remplace celles choisies par station

schedule-colour-station-hint = Couleur de l'étiquette de cette station sur la cascade

schedule-colour-reset = DÉFAUT

schedule-colour-reset-hint = Retour à la couleur d'origine pour toutes les stations, en oubliant les choix faits

settings-language = Langue

settings-language-note = La langue dans laquelle l interface est affichee. Par defaut, elle suit le systeme d exploitation ; choisissez-en une autre pour la remplacer. Le choix est enregistre et memorise.

aprs-all-chip = TOUS

aprs-all-chip-hint = Allumé : tous les messages entendus sur le canal, quel que soit le destinataire. Éteint : seulement ceux qui vous concernent, et les bulletins.

aprs-composer-hidden-swl = L'envoi est masqué : le mode SWL / écoute est actif. Désactivez-le pour écrire et envoyer un message.

aprs-compose-count-hint = Caractères utilisés sur les 67 qu'un message APRS peut contenir.

tx-bands-title = Bandes d'émission

tx-bands-note = Mettez TX sur OUI uniquement pour les bandes où cette radio émet réellement. Ce réglage passe avant tous les autres réglages d'émission ; OUI ne lève jamais les autres sécurités.

tx-bands-careful = ⚠ Une mauvaise bande peut détruire la radio : vérifiez bien chaque option.

tx-bands-yes = OUI

tx-bands-no = NON

tx-bands-confirmed = ✓ Bandes d'émission validées pour cette radio

tx-bands-ack = J'ai vérifié ces bandes par rapport à ma radio

tx-bands-confirm = Valider ces bandes

tx-bands-confirm-hint = Cochez d'abord la case : la validation se fait une fois par radio.

tx-bands-check = Vérifier les bandes…

tx-bands-notice-new = L'émission est coupée sur toutes les bandes. Choisissez les bandes où cette radio émet et validez-les une seule fois. En raison des risques pour la radio, vérifiez bien les options choisies.

tx-bands-notice-carried = Vos bandes d'émission ont été reprises de la version précédente. L'émission reste bloquée tant que vous ne les avez pas vérifiées et validées une fois. En raison des risques pour la radio, vérifiez bien chaque option.


adsbmap-surface-asphalt = asphalte

adsbmap-surface-concrete = béton

adsbmap-surface-grass = herbe

adsbmap-surface-gravel = gravier

adsbmap-surface-dirt = terre

adsbmap-surface-water = eau

adsbmap-airport-runway-lit = éclairée

adsbmap-airport-more = +{ $n } autres

adsbmap-airport-freq-head = Fréquences, MHz

adsbmap-airport-data-date = données pistes et fréquences du { $date }
aircraft-online-switch = En ligne

aircraft-online-switch-on-hover = En ligne : le type, l'exploitant, la route et la photo viennent d'adsbdb et de Planespotters, les images d'aéroport de Wikipédia, et l'heure locale et la température d'un aéroport d'Open-Meteo (seule la position de l'aéroport est envoyée), demandées par votre station pour vous. Cliquez pour désactiver ; la station garde votre choix. Rien d'autre ne quitte la station.

aircraft-online-switch-off-hover = Hors ligne : rien ne quitte la station. Cliquez pour activer le type, l'exploitant, la route et la photo (adsbdb et Planespotters), les images d'aéroport (Wikipédia) et la météo d'aéroport (Open-Meteo).

aircraft-switch-held = l'exploitant de la station le fixe avec SDROXIDE_AIRCRAFT_ONLINE

adsbmap-airport-photo-credit = Photo : Wikipédia

adsbmap-airport-local-time = Heure locale { $time } ({ $zone })

adsbmap-airport-temperature = Température { $temp }

adsbmap-airport-weather-credit = Météo : Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time } (heure locale : chargement…)

adsbmap-airport-utc-offline = UTC { $time } (heure locale : hors ligne)

schedule-listen = ÉCOUTER

schedule-listen-hint = Accorder le récepteur sur cette station

schedule-sort = Trier

schedule-sort-asc = Croissant

schedule-sort-desc = Décroissant

schedule-col-freq = Fréq.

schedule-col-band = Bande

schedule-col-name = Nom

schedule-col-lang = Langue

schedule-col-target = Cible

schedule-col-site = Lieu

schedule-col-local = Locale

tx-bands-off = L'émission est coupée sur { $band } : cette bande n'est pas autorisée dans vos réglages. Votre émetteur n'est donc pas compatible en émission sur cette bande. (Affiché { $max } fois maximum par session.)

frame-band-tx = BAND TX

tx-off-band = ÉMISSION INTERDITE — { $freq } MHz ({ $band }) n'est pas une bande d'émission validée. Cochez et validez-la dans Réglages → Radio → Bandes d'émission.

tx-off-unconfirmed = ÉMISSION INTERDITE — les bandes d'émission de cette radio ne sont pas encore validées. Vérifiez-les et validez dans Réglages → Radio → Bandes d'émission.

aircraft-hover-lookup-retry = pas de réponse — nouvel essai dans { $s } s

adsbmap-airport-navaids-head = Balises sur le terrain

adsbmap-airport-no-freq = Aucune fréquence publiée pour cet aéroport

topbar-preset-right-click = Clic droit pour modifier ce préréglage

topbar-preset-edit = MODIFIER LE PRÉRÉGLAGE

topbar-preset-label = Libellé

topbar-preset-label-hint = Le texte de la pastille. 8 caractères au maximum.

topbar-preset-lo = Bas

topbar-preset-hi = Haut

topbar-preset-use-current = Prendre l'actuel

topbar-preset-use-current-hint = Reprendre les bords du filtre actuellement réglé sur le récepteur

topbar-preset-save = Enregistrer

topbar-preset-reset = Réinitialiser

topbar-preset-reset-hint = Remettre le préréglage d'origine de ce mode

topbar-preset-cancel = Annuler
wefax-schedule = HORAIRES
wefax-schedule-hint = Ouvre dans un navigateur les horaires publiés par { $station } — cartes et heures UTC.

tx-bands-check-again = VÉRIFIER

tx-bands-check-again-hint = Retire la confirmation pour revoir la liste. Changer une bande le fait déjà tout seul.
