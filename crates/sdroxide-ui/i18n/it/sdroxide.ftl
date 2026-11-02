# sdroxide — la traduzione italiana dell'interfaccia. Il riferimento è
# en-US/sdroxide.ftl; ciò che manca qui viene mostrato in inglese.
#
# Ids are grouped by where they are shown, and named after it:
# settings-ui-* is the UI tab of the Settings window, and so on. A message
# shown in capitals is written in capitals here; the code does not upper-case
# translated text.

## The language's own name, as the language picker lists it.
language-name = Italiano

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = Automatico
layout-mode-desktop = Desktop
layout-mode-tablet = Tablet
layout-mode-small = Schermo piccolo
layout-mode-phone = Telefono

ui-theme-default = Predefinito
ui-theme-light = Chiaro
ui-theme-high-contrast = Alto contrasto
ui-theme-green-phosphor = Fosforo verde
ui-theme-amber-phosphor = Fosforo ambra
ui-theme-teal-orange = Verde acqua / arancio
ui-theme-rainbow = Arcobaleno
ui-theme-nord = Nord
ui-theme-nord-dark = Nord scuro
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized scuro
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin mocha
ui-theme-catppuccin-latte = Catppuccin latte
ui-theme-modern-minimalist = Minimalista moderno

chrome-style-angled = Angolare
chrome-style-rectangular = Rettangolare
chrome-style-rounded = Arrotondato
chrome-style-gradient = Sfumato
chrome-style-bevel = Bordo 3D
chrome-style-terminal = Terminale

font-size-small = Piccolo
font-size-medium = Medio
font-size-large = Grande

bandplan-kind-ham = Radioamatori
bandplan-kind-cw = CW
bandplan-kind-digi = Digitale
bandplan-kind-phone = Fonia
bandplan-kind-beacon = Radiofari
bandplan-kind-broadcast = Radiodiffusione
bandplan-kind-am = AM / LW / MW
bandplan-kind-cb = CB

speech-verbosity-terse = Essenziale
speech-verbosity-normal = Normale
speech-verbosity-full = Completo

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = Sintonia (14 point 074)
speech-freq-style-digits = Ogni cifra
speech-freq-style-full = Sintonia con unità

speech-callsign-style-phonetic = Fonetico (kilo one alpha)
speech-callsign-style-letters = Lettere (kay one ay)
speech-callsign-style-as-is = Come scritto

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = Schermo
settings-ui-moved-note = L'ampiezza del panadapter, la reazione dello spettro e la velocità di scorrimento della waterfall si impostano con il pulsante SPEC sulla barra del display — con la finestra stretta, il menu DISP.
settings-ui-language = Lingua
    .hint = La lingua dell'interfaccia. Il manuale aperto con F1 e gli annunci vocali restano in inglese, qualunque sia la lingua scelta qui.
settings-ui-layout = Disposizione
settings-ui-theme = Tema
settings-ui-button-style = Stile dei pulsanti
settings-ui-frame-rate = Frequenza di aggiornamento dello schermo
settings-ui-palette = Palette della waterfall
settings-ui-tuning-buttons = Pulsanti di sintonia
settings-ui-tuning-buttons-show = Mostra su telefono e tablet
    .hint = Una riga di pulsanti abbassa / passo / alza sotto la barra di controllo, su uno schermo a tocco. Non c'è una rotella per scorrere una cifra né una manopola, quindi senza questi pulsanti per spostarsi di un passo noto bisogna digitare l'intera frequenza. Non compaiono mai su desktop.
settings-ui-tuning-step = Passo: { $step }
settings-ui-tuning-snap = Il primo tocco si aggancia al passo
    .hint = Una sintonia che si trova tra due multipli del passo va al multiplo successivo nella direzione premuta — 7 074 300 sale a 7 075 000 con un passo di 1 kHz, scende a 7 074 000. Dopo, i pulsanti si spostano di un passo come sempre. Se disattivato, ogni pressione sposta esattamente di un passo.
settings-ui-smoothing = Levigatura della waterfall
settings-ui-smoothing-interpolate = Interpola
    .hint = Mescola ogni pixel dello schermo con i bin e le righe intorno, così un segnale appare continuo quando il display è più largo della trasformata. Disattivala per una waterfall rettangolare — un blocco per bin, uno per riga — necessaria per leggere la forma di un segnale dall'immagine: un segnale interpolato non si distingue da uno realmente più largo. L'altra metà è una FFT più grande (il chip FFT).
settings-ui-background = Sfondo dello spettro
settings-ui-background-gradient = Sfumato
settings-ui-background-top = alto
settings-ui-background-bottom = basso
settings-ui-spot-colours = Colori delle etichette spot
    .hint = La tinta di ciascuna sorgente di spot: i riquadri lungo il fondo della waterfall, le etichette nella lista SPOTS e i punti sulla mappa del mondo.
settings-ui-spot-colour = Colore per gli spot { $kind }
settings-ui-spot-colours-reset = Riporta ogni colore degli spot al predefinito
settings-ui-reset = Ripristina
settings-ui-bandplan-colours = Colori del piano di banda
    .hint = La tonalità con cui ogni classe di assegnazione viene dipinta sulla striscia del piano di banda lungo il fondo della waterfall. I blocchi sono disegnati semitrasparenti sopra la waterfall, quindi lì risultano più scuri che nel campione qui.
settings-ui-bandplan-colour = Colore per le assegnazioni { $kind }
settings-ui-bandplan-colours-reset = Riporta ogni colore del piano di banda al predefinito
settings-ui-skimmer-font = Dimensione del carattere dello skimmer
settings-ui-waterfall-font = Dimensione del carattere di waterfall / spettro
settings-ui-interface-font = Dimensione del carattere dell'interfaccia
    .hint = Scala l'intera interfaccia — menu, finestre di dialogo, finestre, le schede della radio, la barra superiore e i suoi pulsanti. Le due dimensioni qui sopra sono relative a questa.
settings-ui-cities = Città sulle mappe
    .hint =
        Disegna le città del mondo — un punto per località, con il nome accanto dove c'è spazio — sulle mappe piane: FT8/WSPR, APRS, ADS-B e AIS.

        They are most of what says *where* a dot is on a map of the whole world, and they are also the busiest thing on it. Turn them off if the names are getting in the way of the stations you are reading. The 3D globe is unaffected — its cities are night-side lights rather than markers, and nothing there is written across a contact.
settings-ui-cities-show = mostra le città e i loro nomi
settings-ui-3d-view = Vista 3D
settings-ui-clouds = Resa delle nuvole
settings-ui-clouds-layered = A strati
settings-ui-clouds-volumetric = Volumetrica
settings-ui-clouds-note = Come il livello NUVOLE nella vista 3D disegna il tempo. A strati sovrappone fette della troposfera ed è l'opzione economica. Volumetrica fa correre un raggio attraverso di esse, così il Sole proietta le sommità delle nuvole sulla superficie sottostante e i fulmini illuminano la tempesta dall'interno, anziché solo schiarirne l'esterno — a un costo per pixel di diverse volte superiore.

## Settings window, UI tab: voice announcements.

speech-title = Annunci vocali
speech-enabled = Comunica le variazioni alla radio
    .hint = Legge ad alta voce ciò che è cambiato, così la radio si può usare senza guardarla
speech-voice = Voce
speech-voice-shipped = Voce di serie
speech-speed = Velocità
    .hint = La voce allunga o comprime la propria cadenza, così il tono non cambia. Oltre circa 2× non diventa più breve.
speech-volume = Volume
speech-output = Uscita
speech-detail = Dettaglio
speech-duck = Abbassa il ricevitore
speech-duck-while = Mentre parla
speech-test = Prova
speech-what = Cosa annunciare
speech-cat-frequency = Frequenza
speech-cat-mode-band = Modo e banda
speech-cat-vfo-split = VFO e split
speech-cat-agc-gain = AGC e guadagno
speech-cat-levels = Pilotaggio, accordo e microfono
speech-cat-ptt = Trasmissione e ricezione
speech-cat-rit-xit = RIT e XIT
speech-cat-memory-scan = Memorie e scansione
speech-cat-band-edge = Uscita da una banda radioamatoriale
speech-cat-notices = Avvisi e messaggi
speech-cat-filters = Filtri, squelch e riduzione del rumore
    .hint = Disattivato di default: cambiano continuamente mentre si insegue un segnale
speech-decoded = Messaggi decodificati
speech-ft8-to-me = Chiamate FT8 per me
speech-ft8-cq = CQ FT8 a cui potrei rispondere
    .hint = Una sera affollata sui venti metri ne produce un centinaio al minuto
speech-js8 = Messaggi JS8 per me
speech-js8-allcall = Anche JS8 @ALLCALL
speech-fsq = Messaggi FSQ per me
speech-include-snr = Includi il rapporto
    .hint = Solo quando il messaggio non ne porta uno proprio — una decodifica che riporta già un numero non riceve anche il nostro
speech-ft8-qso = Il mio scambio FT8
    .hint = Ciò che il sequenziatore sta per inviare, e la chiusura del contatto
speech-reading-text = Lettura ad alta voce del testo decodificato
speech-text-cw = CW
speech-text-rtty-psk = RTTY, PSK, Olivia, THOR, FSQ
speech-text-cw-locked = CW solo quando il decodificatore è agganciato
    .hint = Leggere l'uscita di un decodificatore non agganciato è peggio del silenzio
speech-text-note = Entrambi sono disattivati di default. Un decodificatore produce testo più in fretta di quanto la voce riesca a leggerlo, quindi tutto ciò che resta troppo indietro rispetto all'audio in diretta viene scartato anziché messo in coda.
speech-tuning-up = Accordo in corso
speech-swr-while-tuning = Leggi il ROS mentre TUNE è tenuto premuto
speech-swr-every = Ogni
speech-swr-summary = Riporta la migliore adattazione al rilascio
speech-swr-alarm = Avvisa del ROS alto durante ogni trasmissione
speech-how-read = Come vengono letti i dati
speech-frequencies = Frequenze
speech-callsigns = Nominativi
speech-quiet-on-tx = Resta in silenzio durante la trasmissione
    .hint = La voce va agli altoparlanti e quindi nel microfono. Gli avvisi di ROS alto passano comunque.
speech-device-note = Gli annunci vengono riprodotti sul proprio dispositivo audio, quindi non vengono mai registrati né inviati a chi ascolta da remoto. I tasti per pronunciare lo stato, ripetere l'ultimo annuncio e interrompere a metà frase sono nella scheda Controlli.

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = In attesa della configurazione di rete della stazione…
settings-device-reports = come riportato dal dispositivo
settings-transverters = Transverter
settings-one-row-per-box-while = Una riga per ogni casella. Quando la sintonia è dentro la banda di una riga, la radio è sintonizzata su sintonia + offset e si applicano la regola di trasmissione e il limite di pilotaggio di quella riga; fuori da ogni riga la radio opera sulle proprie bande. Le righe vengono provate dall'alto in basso.
settings-name = Nome
settings-band-low = Inizio banda
settings-band-high = Fine banda
settings-offset = Offset
settings-transmit = Trasmissione
settings-max-drive = Pilotaggio massimo
settings-off-takes-transverter-out-line = Disattivato toglie questo transverter dalla linea senza perdere la riga — la banda torna a ciò che la radio raggiunge da sola.
settings-t-2-m = 2 m
settings-what-you-call-shown-log = Come lo chiami. Mostrato nel log quando la sintonia lo seleziona.
settings-bottom-band-transverter-works-dial = Il limite inferiore della banda in cui opera questo transverter, sul dial. 144 per i 2 m.
settings-top-band-dial-148-2 = Il limite superiore della banda, sul dial. 148 per i 2 m nelle Regioni 2 e 3, 146 nella Regione 1.
settings-radio-ends-up-dial-offset = La radio finisce su sintonia + offset, quindi un transverter che porta una banda a una I.F. ha valore negativo: i 2 m in una I.F. di 28 MHz sono -116, perché 144 - 116 = 28. Trascina per correggere di un hertz alla volta un oscillatore leggermente fuori.

    Through the same converter: it works both ways, and transmit takes the same offset.

    Your Drive setting is held under this rather than moved, so the number you use on HF comes back when the dial leaves the band.
settings-remove = RIMUOVI
settings-add-transverter = AGGIUNGI TRANSVERTER
settings-takes-effect-apply-reconnect-like = Ha effetto con Applica / riconnetti, come l'offset del convertitore qui sopra.
settings-maximum-transmit-drive = Pilotaggio massimo in trasmissione
settings-hard-limit-drive-tune-controls = Un limite rigido oltre il quale i comandi Pilotaggio e TUNE non possono andare, applicato dopo la calibrazione di banda qui sotto così che nulla possa riportare il pilotaggio al di sopra. Impostalo dove la radio eroga la potenza nominale, e tutto il comando Pilotaggio diventa utilizzabile — su un trasmettitore che modula i propri campioni (HPSDR, LimeSDR, PlutoSDR, HackRF) il pilotaggio pieno significa il trasmettitore spalancato, che con un amplificatore ad alto guadagno supera di molto ciò per cui i finali sono nominali. Disattivato significa che i comandi arrivano al pilotaggio pieno.
settings-limit-drive = Limita il pilotaggio a
settings-per-cent-full-drive-key = Percento del pilotaggio pieno. Metti la radio in trasmissione su un carico fittizio e abbassa questo valore finché il misuratore non legge la potenza per cui l'amplificatore è nominale.
settings-applies-immediately-every-band-tune = Ha effetto subito, su ogni banda e anche su TUNE.
settings-transmit-drive-band = Pilotaggio in trasmissione per banda
settings-every-amplifier-has-different-gain = Ogni amplificatore ha un guadagno diverso su ogni banda, quindi una stessa impostazione di Pilotaggio dà una potenza diversa su ciascuna. Misura l'uscita su ogni banda, poi riduci le bande che risultano alte finché non coincidono tutte: imposta Pilotaggio per la banda che ne ha più bisogno (di solito la più alta), e porta le altre al suo livello. Decibel di potenza in uscita, applicati alla fonia, ai modi digitali e a TUNE allo stesso modo. Zero ovunque — il predefinito — non cambia nulla.
settings-other = Altro
settings-clear-all = CANCELLA TUTTO
settings-back-no-calibration-drive-setting = Torna nessuna calibrazione: l'impostazione Pilotaggio raggiunge ogni banda per intero.
settings-applies-immediately-band-you-would = Ha effetto subito, e sulla banda su cui trasmetteresti — dietro un transverter è la banda sulla sintonia, e il limite di pilotaggio del transverter prevale comunque su qualsiasi cosa impostata qui.
settings-settings = Impostazioni
settings-enter-server-s-address-first = inserisci prima l'indirizzo del server
settings-enter-receiver-s-address-first = inserisci prima l'indirizzo del ricevitore, oppure scegline uno tra gli SDR pubblici
settings-no-radio-selected-press-discover = nessuna radio selezionata — premi Cerca, oppure inserisci un indirizzo
settings-enter-radio-s-address-first = inserisci prima l'indirizzo della radio
settings-general = Generale
settings-radio = Radio
settings-ui = Interfaccia
settings-alerts = Avvisi
settings-controls = Controlli
settings-spots = Spot
settings-freedv = FreeDV
settings-uploads = Invii
settings-winlink = Winlink
settings-servers = Server
settings-t-r-switch = Commutatore T/R
settings-tle = TLE
settings-profiles = Profili
settings-station = Stazione
settings-enter-digital-mode-ft8-sstv = Inserisci un modo digitale (FT8 / SSTV / …) una volta per caricare i valori salvati.
settings-callsign = Nominativo
settings-grid-square = Quadrato locatore
settings-iaru-region = Regione IARU
settings-which-radio-interface-server-uses = L'interfaccia radio usata dal server si imposta sulla macchina su cui gira.
settings-radio-interface = Interfaccia radio
settings-waiting-machine-radio-attached-interface = In attesa della macchina a cui è collegata la radio. Le sue impostazioni di interfaccia sono qui sotto e nel frattempo si possono cambiare da qui.
settings-converter = Convertitore
settings-manual-type-offset-below = Manuale — scrivi un offset qui sotto
settings-what-box-reads-when-offset = Il valore letto da questa casella quando l'offset non è uno dei preset. Si imposta scrivendo nel campo Offset qui sotto; qui non c'è nulla da selezionare.
settings-frequency-converter-front-receiver-pick = Un convertitore di frequenza davanti al ricevitore. Scegline uno, oppure scrivi accanto un offset.
settings-how-far-converter-moves-signal =
    Di quanto un convertitore sposta il segnale sulla via verso il ricevitore, in Hz — lo stesso numero e lo stesso segno che riporta la documentazione di ogni convertitore e di ogni altro programma SDR. La radio finisce su sintonia + offset: positivo per un upconverter (un Ham It Up è 125000000), negativo per tutto ciò che porta una banda a una I.F. — un LNB satellitare, o un transverter per i 2 m su una I.F. di 28 MHz, che è -116000000 (28 − 144). 0 = nessun convertitore.

    Drag to trim it a hertz at a time, which is what a converter whose oscillator is slightly off wants.

    This is the receive path. What is in the transmit line is the row below.

    Takes effect on Apply.
settings-what-transmit-line-while-converter =
    Ciò che è nella linea di trasmissione mentre un convertitore è impostato.

    Off while converting: nothing is transmitted — the safe default, and right for a receive-only accessory.

    Through the same converter: one box converts both ways, a transverter. Transmit takes the offset above and follows it when it is trimmed.

    Its own offset: the transmit line is different from the receive one. Nought is the common case — a converter (an LNB, a Ham It Up) on receive with the transmitter on its own antenna, which is the QO-100 station.

    The amateur-band check still applies, on the frequency you are transmitting on.

    Takes effect on Apply.
settings-how-far-transmit-line-moves =
    Di quanto la linea di trasmissione sposta il segnale, in Hz, con la stessa regola di segno dell'offset di ricezione: la radio trasmette su sintonia + offset.

    0 = the transmitter is wired to its own antenna and works on the frequency the dial says — a QO-100 station's 2.4 GHz uplink.

    A transmit converter that takes an I.F. *up* is a negative number: the radio works below the dial.
settings-rx-range = Gamma RX
settings-which-frequencies-radio-receives-mhz =
    Quali frequenze riceve questa radio, in MHz: 144-146, 430-440. Lascia vuoto per usare ciò che il dispositivo riferisce di sé.

    Band buttons outside the range are greyed out and the dial will not go there.

    These are dial frequencies. With a converter set, state the band you tune — 144-148 for a 2 m transverter — not the I.F. the radio is really on.

    Takes effect on Apply.
settings-tx-range = Gamma TX
settings-which-frequencies-radio-transmits-mhz =
    Su quali frequenze trasmette questa radio, in MHz: 144-146, 430-440. Lascia vuoto per usare ciò che riferisce il dispositivo — e se non riferisce nulla, si dà credito al driver e ogni frequenza è consentita.

    Dial frequencies, like the receive range above.

    This is a limit you set, not a licence: transmitting outside the amateur bands is refused regardless unless you have turned that off in config.toml. Nor does it give a receive-only device a transmitter.

    Takes effect on Apply.
settings-antenna = L'antenna è
settings-where-radio-listens-station-antenna =
    Da dove ascolta questa radio.

    At the station: the antenna is yours, so your own locator says where it is. The default, and right for everything in the shack.

    Somewhere else: an online receiver, or your own set up on a hilltop. What it hears is reported to PSK Reporter, WSPRnet and FreeDV Reporter from the locator beside this, never from yours — and ADS-B places aircraft against it too.

    Picking a receiver under "Public SDRs" fills this in for you.

    Takes effect on Apply.
settings-receiver-s-maidenhead-locator-jn88ec =
    Il locatore Maidenhead del ricevitore — JN88ec, DO30db. Quattro o sei caratteri.

    Leave it empty if you do not know where the receiver is: nothing is then reported at all, which is the only honest answer. Reporting from your own square would put somebody else's reception on the wrong continent.

    On top of cal_offset_db in config.toml, which is one figure for the whole station.
settings-antenna-somewhere-else-no-locator = L'antenna è altrove e non è impostato alcun locatore, quindi nulla di ciò che questa radio sente viene riportato a PSK Reporter, WSPRnet o FreeDV Reporter. Scrivi il locatore del ricevitore qui sopra per riportare da lì.
settings-ranges-are-mhz-low-high = Le gamme sono in MHz, da-a, separate da virgole: 144-146, 430-440 — cioè 144000000-146000000 Hz e 430000000-440000000 Hz. Sono frequenze di sintonia: un offset del convertitore non le sposta. L'offset del convertitore qui sopra è il campo in hertz. Lascia vuota una gamma per usare ciò che il dispositivo riferisce di sé; un dispositivo che non riferisce nulla viene preso sulla parola.
settings-careful-rtl-sdr-blog-v4 = Attenzione con un RTL-SDR: il Blog V4 fa un upconversion da solo sotto 28,8 MHz, quindi un offset negativo che porta l'hardware lì sposta il segnale due volte.
settings-pick-radio-interface-above-configuration = Scegli un'interfaccia radio qui sopra (questa configurazione usava la modalità di rilevamento automatico, ormai rimossa).
settings-configuration-used-removed-auto-detect = Questa configurazione usava la modalità di rilevamento automatico, ormai rimossa. Scegli un'interfaccia qui sopra quando la macchina a cui è collegata la radio ha risposto.
settings-radio-has-no-interface-yet = Questa radio non ha ancora un'interfaccia — scegline una qui sopra e premi Applica / riconnetti.
settings-radio-has-no-interface-yet-2 = Questa radio non ha ancora un'interfaccia. Se ne può scegliere una qui sopra quando la macchina a cui è collegata la radio ha risposto.
settings-apply-reconnect = Applica / riconnetti
settings-switch-interface-now-no-restart = Passa subito a questa interfaccia — nessun riavvio necessario
settings-switch-server-s-radio-interface = Passa subito la radio del server a questa interfaccia — nessun riavvio necessario. La vecchia resta attiva se la nuova non può essere aperta.
settings-switches-live-radio-without-restarting = Cambia la radio in funzione senza riavviare.
settings-everything-above-server-s-own = Tutto ciò che è qui sopra è la configurazione propria del server, e le modifiche vengono salvate lì. La maggior parte delle impostazioni ha effetto mentre le cambi; quelle fissate all'apertura del dispositivo — la frequenza di campionamento, un indirizzo, l'interfaccia stessa — richiedono questo pulsante.
settings-dx-cluster-telnet = DX cluster (telnet)
settings-enabled = Attivo
settings-host = Host
settings-port = Porta
settings-login-call = Nominativo di accesso
settings-commands = Comandi
settings-one-per-line-e-g = uno per riga, ad es. SET/FT8
settings-one-per-line-e-g-2 = uno per riga, ad es. set/filter cont=eu
settings-sent-after-login-place-narrow = Inviato dopo l'accesso. È il punto in cui restringere il flusso — senza un filtro sono tutti gli skimmer della Terra.
settings-rbn-paths-are-placed-country = I percorsi RBN sono collocati dai centri dei paesi, non dai locatori — precisi per un paese piccolo, lontanissimi per uno grande. Sono un livello separato sulla mappa di propagazione e si possono disattivare lì.
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = Spot di attivatori POTA
settings-sota-spots = Spot SOTA
settings-psk-reporter-current-band = PSK Reporter (banda corrente)
settings-upload-my-ft8-ft4-ft2 = Invia le mie decodifiche FT8/FT4/FT2/JS8
settings-antenna-2 = Antenna
settings-collector = Collettore
settings-t-4739-live-collector-14739-test = 4739 è il collettore in funzione, 14739 quello di prova
settings-max-age-s = Età massima (s)
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = Invia le mie decodifiche WSPR
settings-send-every-wspr-reception-wsprnet = Invia ogni ricezione WSPR a wsprnet.org. Attivo di default: non trasmette nulla, e riportare ciò che si sente è ciò che fa di un ricevitore WSPR una parte della rete anziché una curiosità privata. Viene riportato anche un intervallo in cui non è stato decodificato nulla, ed è così che la rete distingue una banda chiusa da un ricevitore spento.
settings-download-who-heard-me = Scarica chi mi ha sentito
settings-ask-wsprnet-org-which-stations = Chiede a wsprnet.org quali stazioni hanno decodificato questa. WSPR non prevede alcuna forma di conferma, quindi è l'unico modo in cui un radiofaro in trasmissione viene a sapere qualcosa della propria portata.
settings-ask-every = Chiedi ogni
settings-looking-back = guardando indietro
settings-apply = {" "}APPLICA{" "}
settings-persist-re-connect-feeds = Salva e (ri)collega i flussi
settings-broadcast-stations = Stazioni di radiodiffusione
settings-winlink-account = Account Winlink
settings-password = Password
settings-winlink-account-password-not-gateway = La password dell'account Winlink, non quella del gateway. Fa distinzione tra maiuscole e minuscole — inseriscila esattamente come ti è stata rilasciata.
settings-locator = Locatore
settings-how-connect = Come collegarsi
settings-route = Percorso
settings-internet = Internet
settings-forward-cms-over-internet = Inoltra con il CMS via internet
settings-radio-packet = Radio (pacchetto)
settings-call-rms-gateway-air-radio = Chiama un gateway RMS via radio. La radio deve essere in PACKET o PACKET-HF.
settings-gateway = Gateway
settings-via = Via
settings-digipeaters-order-separated-spaces-usually = Digipeater, in ordine, separati da spazi. Di solito vuoto — un gateway che senti direttamente è un gateway da chiamare direttamente.
settings-speed = Velocità
settings-most-rms-gateways-answer-1200 = La maggior parte dei gateway RMS risponde a 1200. Chiamare un gateway a 1200 a 9600 suona esattamente come un gateway spento, quindi imposta questo valore in base a ciò che pubblica il titolare del gateway, senza indovinare. 9600 richiede inoltre la porta dati della radio a entrambi i capi. 300 è HF.
settings-frequency = Frequenza
settings-clear = CANCELLA
settings-zero-leaves-dial-alone-which = Zero lascia in pace la sintonia, che è ciò che vuoi quando resti su un solo canale. Qualsiasi altro valore sintonizza la radio all'avvio della sessione.
settings-my-gateways = I miei gateway
settings-winlink-s-published-gateway-list = L'elenco dei gateway pubblicato da Winlink richiede una chiave API che sdroxide non ha, quindi tieni il tuo. I due o tre gateway raggiungibili da una stessa località si imparano provando, e cambiano di rado.
settings-use-2 = USA
settings-call-one-next-connect = Chiama questo alla prossima connessione
settings-current-dial = sintonia attuale
settings-forget = DIMENTICA
settings-forget-gateway = Dimentica questo gateway
settings-add-gateway = + AGGIUNGI GATEWAY
settings-save-gateway-above-list = Salva il gateway qui sopra nell'elenco
settings-internet-gateway = Gateway internet
settings-cms-address = Indirizzo CMS
settings-client-name = Nome del client
settings-winlink-s-production-servers-only = I server di produzione di Winlink accettano solo nomi di client che conoscono, e rispondono a un nome sconosciuto con "Unknown client types are not allowed on production servers". Finché sdroxide non sarà registrato presso il Winlink Development Team, per collegarsi serve un nome che lo riconoscano.
settings-automatic-connection = Connessione automatica
settings-connect-timer = Collega a intervalli
settings-every = Ogni
settings-persist-winlink-account = Salva l'account Winlink
settings-callsign-lookup = Ricerca nominativi
settings-provider = Fornitore
settings-auto-fill-name-qth-grid = Compila automaticamente nome/QTH/locatore al clic su uno spot e per il QSO
settings-choose-provider-enter-login = Scegli un fornitore per inserire il suo accesso.
settings-qrz-user = Utente QRZ
settings-qrz-pass = Password QRZ
settings-hamqth-user = Utente HamQTH
settings-hamqth-pass = Password HamQTH
settings-one-hamqth-account-does-both = Un solo account HamQTH fa entrambe le cose — questo accesso è quello usato dalla scheda di invio HamQTH.
settings-upload = Invio
settings-auto-upload-each-new-qso = Invia automaticamente ogni nuovo QSO
settings-master-switch-which-services-qso = L'interruttore generale. I servizi a cui va un QSO si impostano in ciascuna scheda qui sotto.
settings-turn-auto-upload-each-new = Attiva prima "Invia automaticamente ogni nuovo QSO" qui sopra — un servizio selezionato qui non viene inviato finché non lo è.
settings-auto-upload-off-above-so = L'invio automatico è disattivato qui sopra, quindi le selezioni dei servizi sono disabilitate e per ora non si invia nulla in automatico — il pulsante UP per singolo QSO nel log funziona comunque.
settings-qrz-log-key = Chiave del log QRZ
settings-qrz-logbook-api-key-your = La chiave API del logbook QRZ, dalle impostazioni del tuo logbook QRZ — non l'accesso per la ricerca XML.
settings-eqsl-user = Utente eQSL
settings-eqsl-pass = Password eQSL
settings-eqsl-qth = QTH eQSL
settings-same-login-hamqth-callsign-lookup = Lo stesso accesso della ricerca nominativi HamQTH — compilarne uno compila entrambi.
settings-club-log-email = Email di Club Log
settings-club-log-pass = Password di Club Log
settings-club-log-key = Chiave di Club Log
settings-wrl-api-key = Chiave API WRL
settings-developer-api-key-world-radio = La chiave API per sviluppatori di World Radio League → Integrations → Developer API. Viene mostrata una sola volta quando la generi, quindi copiala subito. I contatti vanno al tuo logbook predefinito — impostane uno in WRL se ne hai più di uno.
settings-confirmations-download = Conferme (download)
settings-lotw-user = Utente LoTW
settings-lotw-pass = Password LoTW
settings-lotw-upload-uses-tqsl-export = L'invio a LoTW usa TQSL — esporta l'ADIF dal logbook e firmalo. Le conferme LoTW/eQSL vengono scaricate qui per distinguere i collegamenti lavorati da quelli confermati.
settings-sync-confirmations = SINCRONIZZA CONFERME
settings-fetching-subscriptions = Recupero degli abbonamenti…

    To use it on its own again, clear the Panadapter receiver box on that radio's page.
settings-panadapter = Panadapter
settings-give-radio-wideband-panadapter-borrowing = Dai a questa radio un panadapter a banda larga prendendo in prestito il ricevitore di un'altra radio — un SDR sulla stessa antenna, oppure alimentato dall'uscita I.F. di questa radio. Lo spettro, la waterfall, il sub-ricevitore, i modi digitali e lo skimmer vengono tutti da quel ricevitore; sintonia, modo, filtro e trasmettitore restano qui.
settings-receiver = Ricevitore
settings-none = Nessuno

    Takes effect on Apply.
settings-connected = Collegato a

    This is not the Converter field above, which retunes the radio, and not the CAT tab's I/Q centre offset, which is about this radio's own sound card. Nothing typed here is ever sent to either radio.

    Takes effect on Apply.
settings-audio-2 = Audio da
settings-follow-dial = Segui la sintonia
settings-keep-receiver-radio-s-dial = Mantieni il ricevitore sulla sintonia di questa radio, e sposta la sintonia quando sintonizzi sul panadapter. Disattivato, il ricevitore resta dov'è — un ascolto fisso su un segmento mentre la radio va altrove.
settings-invert-spectrum = Inverti lo spettro
settings-mirror-receiver-s-span-about = Specchia l'ampiezza del ricevitore rispetto al suo centro, per una derivazione il cui oscillatore sta sopra il segnale e che consegna la banda al contrario. Lascialo disattivato a meno che la waterfall non si riempia di segnali tutti dalla parte sbagliata della sintonia.
settings-mute-transmit = Muto in trasmissione
settings-silence-receive-audio-while-radio = Silenzia l'audio in ricezione mentre questa radio trasmette. Attivo di default: con il ricevitore sulla stessa antenna, o sulla I.F. di questa radio, ciò che si sente durante una trasmissione è il proprio trasmettitore. Anche le trasmissioni avviate sulla radio stessa — il PTT del microfono, un pedale, MOX — vengono contate, appena il collegamento CAT segnala che l'apparato è passato in trasmissione.
settings-blank-transmit = Oscura in trasmissione
settings-stop-panadapter-waterfall-while-radio = Ferma il panadapter e la waterfall mentre questa radio trasmette. Attivo di default: un trasmettitore che dipinge l'intera ampiezza cancella la banda dietro di sé. Disattivalo per guardare la banda — o il proprio segnale — durante una trasmissione, cosa utile con il ricevitore su un'antenna separata. Vale per le trasmissioni avviate da sdroxide; una avviata sulla radio viene ricevuta fino in fondo, quindi l'immagine continua comunque, qualunque cosa dica questo.
settings-per-mode-offsets-radio-whose = Offset per modo, per una radio la cui I.F. si sposta con il modo. Un modo lasciato vuoto usa l'offset qui sopra.
settings-computer = Su questo computer
settings-add-radio-computer = Aggiungi una radio su questo computer
settings-radio-s-settings-are-below = Le impostazioni di questa radio sono qui sotto. Trascina per spostarla lungo la barra.
settings-switch-radio-dialog-follows-drag = Passa a questa radio (la finestra segue). Trascina per spostarla lungo la barra.
settings-tx = ● TX
settings-mute-radio-s-audio = Silenzia l'audio di questa radio
settings-close-radio-configuration-kept = Chiudi questa radio (la sua configurazione viene conservata)
settings-close-tab-close-radio-station = Chiudi questa scheda, oppure chiudi la radio sulla sua stazione
settings-close-tab = Chiudi questa scheda
settings-transverters-full = { $max } è il massimo che questa tabella può contenere
settings-dialling = Composizione { $url }…
settings-region-note = Dove si trova la stazione. Imposta ogni piano di banda: i limiti delle bande, i sotto-segmenti CW / dati / fonia sulla striscia della waterfall, dove ascoltano gli skimmer, le frequenze di chiamata proposte e ciò che conta come fuori banda in trasmissione. Nella Regione { $region } questo fa sì che i 70 cm { $region ->
        [1] siano 430–440 MHz — quindi 446 è fuori banda, e i 40 m si fermano a 7.200
        [2] siano 420–450 MHz, e i 40 m arrivano a 7.300
       *[3] siano 430–450 MHz, e gli 80 m si fermano a 3.900
    }. Ha effetto subito e vale per ogni radio di questa stazione.
settings-rx-grid-reported = Le ricezioni attraverso questa radio vengono riportate dal locatore { $grid }, non dal tuo.
settings-rx-grid-invalid = { $grid } non è un locatore Maidenhead, quindi non viene riportato nulla di ciò che sente questa radio. Quattro o sei caratteri: due lettere, due cifre e, facoltativamente, altre due lettere.
settings-gateway-via = { " " }via { $via }
settings-auto-upload-to = Invia automaticamente ogni nuovo QSO a { $service }
settings-tle-updated = Aggiornat{ $n ->
        [one] o
       *[other] i
    } { $n } { $n ->
        [one] abbonamento
       *[other] abbonamenti
    }: { $total } satelliti.
settings-tle-updated-failed = Aggiornati { $ok } su { $n }; { $failed } non riusciti — vedi le righe qui sopra.
settings-panadapter-of = Questa radio è il panadapter di { $radio }
settings-radio-n = radio { $n }
settings-on-station = Su { $station }
settings-add-radio-on-station = Aggiungi una radio su { $station } — la stazione la serve subito
settings-panadapter-receiver-for = Ricevitore del panadapter per { $radio }
settings-hang-up-stays-on = Riaggancia. La radio resta su { $station }.
settings-close-on-station = Chiudere { $radio } su { $station }? La sua configurazione resta su quella macchina; la scheda qui si chiude con essa, e con essa quella di tutti gli altri.
settings-close-it-on = Chiudila su { $station }
settings-locator-hint = locatore

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = Avvisi sonori
settings-alerts-sound-alarm-when-decode-matters = Suona un allarme quando una decodifica è importante
settings-alerts-plays-over-own-audio-output = Viene riprodotto sulla propria uscita audio, così si sente anche quando la banda è in un altoparlante diverso da questo schermo.
settings-alerts-volume = Volume
settings-alerts-output = Uscita
settings-alerts-test = Prova
settings-alerts-what-sound = Cosa far suonare
settings-alerts-each-station-quiet-while-after = Ogni stazione resta silenziosa per un po' dopo un avviso, così una banda affollata non fa suonare ogni slot. Voce e Tono + voce sono letti dalla voce degli annunci parlati — attivala in Impostazioni → Interfaccia.

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = Amplificatore
settings-amp-waiting-station-s-amplifier-configuration = In attesa della configurazione dell'amplificatore della stazione…
settings-amp-hobbypcb-hardrock-500-hardrock-50 = Un HobbyPCB Hardrock-500 o Hardrock-50 sulla sua porta seriale USB. SDRoxide legge i suoi strumenti, ne commuta banda, modo e antenna, e ne controlla il sintonizzatore dalla finestra AMP. L'amplificatore resta comandato dalla linea PTT della radio come prima.
settings-amp-enable = Attiva
settings-amp-model = Modello
settings-amp-untested-panel-has-only-been = Non testato: questo pannello è stato provato solo con un Hardrock-500. I limiti dello strumento per questo modello sono scalati da quelli dell'Hardrock-500 e potrebbero essere sbagliati. Controlla il display dell'amplificatore stesso.
settings-amp-port = Porta
settings-amp-choose = — scegli —
settings-amp-baud = Baud
settings-amp-must-match-amplifier-s-menu = Deve corrispondere al menu dell'amplificatore. Il pannello di controllo di HobbyPCB richiede 115200.
settings-amp-follow-radio-s-band = Segui la banda della radio
settings-amp-sends-transmit-frequency-amplifier-every = Invia la frequenza di trasmissione all'amplificatore a ogni cambio di banda, così i suoi filtri seguono la radio senza un cavo CAT proprio. Mai inviata durante una trasmissione.
settings-amp-show-power-swr-tx-meter = Mostra la sua potenza e il ROS sullo strumento TX
settings-amp-only-where-radio-does-not = Solo dove la radio non le misura da sé, e solo mentre l'amplificatore è in linea.
settings-amp-off = Disattivato
settings-amp-connecting = Connessione…
settings-amp-status = Stato
settings-amp-apply = {" "}APPLICA{" "}
settings-amp-save-re-open-amplifier-s = Salva e (ri)apri la porta dell'amplificatore

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = Tastiera
settings-controls-click-shortcut-rebind-then-press = Clicca una scorciatoia per riassegnarla, poi premi la combinazione di tasti (Esc annulla). Le assegnazioni vengono ignorate mentre scrivi in un campo di testo.
settings-controls-shortcut = Scorciatoia
settings-controls-does = Fa
settings-controls-step-mode = Passo / modo
settings-controls-accel = Accel
settings-controls-on = Attivo
settings-controls-press-key = premi un tasto…
settings-controls-remove-binding = Rimuovi questa assegnazione
settings-controls-add-shortcut = + Aggiungi scorciatoia
settings-controls-restore-defaults = Ripristina i predefiniti
settings-controls-bind-hold-talk-space = Assegna il parla-a-tenere alla barra spaziatrice
settings-controls-hold-space-transmit-releasing-losing = Tieni premuta la barra spaziatrice per trasmettere; rilasciandola — o perdendo il fuoco della finestra — si smette di trasmettere
settings-controls-unkey-held-ptt-after = Interrompi un PTT tenuto premuto dopo
settings-controls-backstop-against-stuck-key-controller = Protezione contro un tasto bloccato o un controller che smette di inviare dati. 0 disattiva.
settings-controls-panadapter-mouse = Mouse sul panadapter
settings-controls-wheel = Rotella
settings-controls-wheel-shift = Rotella + Maiusc
settings-controls-tune-step = Passo di sintonia
settings-controls-zoom-rate = Velocità di zoom
settings-controls-click-tune-rounding = Arrotondamento della sintonia al clic
settings-controls-invert-wheel-direction = Inverti la direzione della rotella
settings-controls-left-drag-tunes-well-pans = Il trascinamento sinistro sintonizza oltre a spostare la vista
settings-controls-off-makes-left-drag-pan = Disattivato, il trascinamento sinistro sposta solo la vista, come quello destro.
settings-controls-scroll-digit-frequency-readout-tune = Scorri una cifra sulla lettura di frequenza per sintonizzarla
settings-controls-both-wheel-actions-are-tune = Entrambe le azioni della rotella sono Sintonia — non resta alcun modo per zoomare.
settings-controls-restore-mouse-defaults = Ripristina i predefiniti del mouse
settings-controls-mouse-buttons = Pulsanti del mouse
settings-controls-left-right-buttons-are-reserved = I pulsanti sinistro e destro sono riservati alla sintonia e allo spostamento; il pulsante centrale e quelli extra sono liberi. Un pulsante laterale tenuto premuto per il PTT si comporta come un pedale.
settings-controls-add-mouse-button = + Aggiungi pulsante del mouse
settings-controls-f1-always-opens-manual-even = F1 apre sempre questo manuale, anche mentre scrivi, quindi non è riassegnabile.
settings-controls-midi-controller = Controller MIDI
settings-controls-midi-controllers-need-native-app = I controller MIDI richiedono l'app nativa — il client nel browser non ha accesso al MIDI.
settings-controls-any-class-compliant-midi-surface = Funziona qualsiasi superficie MIDI conforme alla classe: la rotella jog di un controller DJ fa una buona manopola del VFO, i suoi pad fanno PTT e pulsanti di banda, i suoi fader fanno controlli di guadagno.
settings-controls-enable = Attiva
settings-controls-controller = Controller
settings-controls-feedback = Retroazione verso
settings-controls-rescan-ports = Riesamina le porte
settings-controls-not-connected = Non collegato.
settings-controls-move-control-see-here = Muovi un comando per vederlo qui.
settings-controls-control = Comando
settings-controls-reads = Legge come
settings-controls-led = LED
settings-controls-move = muovilo…
settings-controls-click-then-move-control-you = Clicca, poi muovi il comando che vuoi assegnare
settings-controls-speed-sensitivity-spin-faster-tune = Sensibilità di velocità: gira più in fretta per sintonizzare più in fretta
settings-controls-send-current-value-back-light = Rimanda il valore attuale, per accendere un LED o muovere un fader
settings-controls-add-midi-control = + Aggiungi comando MIDI
settings-controls-clear-all = Cancella tutto
settings-controls-endless-jog-encoders-send-relative = Gli encoder infiniti (jog) inviano un passo relativo anziché una posizione, in una di tre codifiche che da piccoli movimenti sembrano uguali. IMPARA indovina da una rotazione in senso orario; se poi la manopola sintonizza nel verso sbagliato, spunta “rev”.
settings-controls-none = — nessuno —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = Predefinito di sistema
settings-general-remote-access = Accesso remoto
settings-general-what-remote-client-browser-page = Ciò che un client remoto — la pagina nel browser, oppure un altro sdroxide avviato con --connect — deve fornire prima che questa stazione lo lasci operare. Vale in modalità server (--server); il prossimo accesso riprende una modifica, senza riavvio.
settings-general-username = Nome utente
settings-general-password = Password
settings-general-clients-must-give-password-leaving = I client devono fornire la password. Lasciare vuoto il nome utente va bene.
settings-general-clients-must-sign = I client devono accedere.
settings-general-empty-anyone-who-can-reach = ⚠ Vuoto: chiunque possa raggiungere la porta del server può operare questa radio, con il tuo nominativo. Imposta una password prima di inoltrare la porta.
settings-general-stored-clear-config-toml-like = Salvata in chiaro in config.toml, come le altre password che sdroxide conserva.
settings-general-receive-audio-gain = Guadagno audio in ricezione
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    Guadagno extra su tutto ciò che questa radio invia agli altoparlanti, oltre al controllo del volume. Lascialo a 0 dB a meno che la radio non sia debole a volume pieno: il fondo della scala del volume è l'audio così come arriva, quindi può abbassare una radio ma mai alzarla, e l'uscita audio USB di alcuni ricetrasmettitori sta ben sotto il fondo scala.

    Go up 6 dB at a time. Too much clips — the audio is limited at full scale rather than allowed to wrap round, so overdoing it sounds harsh rather than loud. Recordings are taken ahead of this and are not affected.
settings-general-reload-band-plan = RICARICA PIANO DI BANDA
settings-general-re-read-bandplan-json-machine = Rilegge bandplan.json sulla macchina a cui è collegata la radio e lo applica — limiti di banda, sotto-segmenti e finestre degli skimmer — senza riavviare.
settings-general-running-station-s-own-band = In funzione con il piano di banda proprio della stazione.
settings-general-running-built-iaru-tables = In funzione con le tabelle IARU integrate.
settings-general-settings-file = File delle impostazioni
settings-general-settings-are-machine-radio-attached = Le impostazioni si trovano sulla macchina a cui è collegata la radio. Esportale da lì.
settings-general-export = ESPORTA…
settings-general-write-every-setting-station-radios = Scrive ogni impostazione di questa stazione — le radio, i modi, i server, le memorie, il piano di banda — in un unico file che puoi portare su un'altra installazione. Il tuo logbook e le eventuali password dei server salvate restano qui.
settings-general-import = IMPORTA…
settings-general-replace-station-s-settings-ones = Sostituisce le impostazioni di questa stazione con quelle di un file esportato da un'altra installazione. Riavvia sdroxide dopo.
settings-general-import-overwrites-what-here-file = Un'importazione sovrascrive ciò che c'è qui, file per file, e ha effetto la prossima volta che sdroxide si avvia — altrimenti le impostazioni già in memoria le riscriverebbero sopra. Ciò che il file non menziona resta com'è, quindi un pacchetto di una stazione con una sola radio non rimuove una seconda radio qui. Il tuo logbook non è mai nel file: se vuoi spostare anche quello, esportalo come ADIF dalla finestra LOG.
settings-general-sdroxide-settings = Impostazioni di sdroxide
settings-general-swr-guard = Protezione ROS
settings-general-stop-transmitting-high-swr = Interrompi la trasmissione con ROS alto
settings-general-trip = Scatta a
settings-general-stops-transmission-when-radio-reports =
    Interrompe la trasmissione quando la radio riporta un ROS pari o superiore a questo valore, e blocca la trasmissione finché non lo confermi. Intercetta un'antenna scollegata, una linea di alimentazione guasta o un commutatore lasciato sulla porta sbagliata.

    Tuning is treated differently, because feeding a mismatch is the point of it: an antenna tuner gets double the limit and about five seconds before the guard applies at all. A manual tuner that takes longer than that wants the guard switched off for the session.

    Needs a rig that reports SWR over CAT. Ignores the first fifth of a second of each transmission, and does not wait for high power.
settings-general-per-mode-settings = Impostazioni per modo
settings-general-reset-every-mode = AZZERA OGNI MODO
settings-general-forget-every-mode-s-remembered = Dimentica l'AGC, lo squelch, la riduzione del rumore, il notch e gli interruttori stereo memorizzati per ogni modo, e ripristina i valori predefiniti di ciascun modo.
settings-general-selecting-mode-lays-mode-s = Selezionare un modo imposta sul ricevitore i valori iniziali propri di quel modo per AGC, squelch, riduzione del rumore, notch e interruttori stereo, e modificarne uno lo memorizza solo per quel modo. Il pulsante a freccia circolare in fondo alla riga filtro/rumore del ricevitore elenca ciò che è stato cambiato e riporta il modo attuale.
settings-general-channels = Canali

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = Operatore
settings-net-callsign-grid-are-set-general = Il nominativo e il locatore si impostano nella scheda Generale.
settings-net-set-your-callsign-grid-general = ⚠ Imposta il tuo nominativo e il locatore nella scheda Generale.
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = Attiva
settings-net-connects-whenever-enabled-your-station = Si collega ogni volta che è attivo. La tua stazione è mostrata agli altri solo mentre la radio è in RADE — in qualunque altro modo resti collegato ma nascosto.
settings-net-station = Stazione
settings-net-message = Messaggio
settings-net-receive-only-i-cannot-transmit = Solo ricezione (non posso trasmettere)
settings-net-server = Server
settings-net-host = Host
settings-net-port = Porta
settings-net-tls-wss = TLS (wss://)
settings-net-not-yet-implemented-freedv-gui = Non ancora implementato — anche l'interfaccia FreeDV usa ws:// semplice.
settings-net-reporting = Segnalazione
settings-net-report-stations-i-decode = Segnala le stazioni che decodifico
settings-net-sends-rx-report-each-callsign = Invia un rx_report per ogni nominativo recuperato da un frame End-of-Over di RADE.
settings-net-show-other-reporter-stations-spots = Mostra le altre stazioni reporter come spot
settings-net-adds-them-panadapter-overlay-world = Le aggiunge al livello del panadapter, alla mappa del mondo e alla finestra SPOTS, sotto il filtro FREEDV.
settings-net-set-your-callsign-grid-general-2 = ⚠ Imposta il tuo nominativo e il locatore nella scheda Generale. Senza entrambi, il collegamento è solo in visualizzazione: vedi le altre stazioni ma tu non compari.
settings-net-apply = {" "}APPLICA{" "}
settings-net-persist-re-connect = Salva e (ri)collega
settings-net-season = Stagione
settings-net-downloading = download in corso…
settings-net-using-built-copy = uso la copia integrata
settings-net-your-own-stations-corrections-go = Le tue stazioni e le correzioni vanno in questo file. sdroxide non lo scrive mai, e una voce qui sostituisce una voce programmata con lo stesso nome e la stessa frequenza.
settings-net-reload = Ricarica
settings-net-re-read-your-station-file = Rilegge il file delle tue stazioni dopo averlo modificato
settings-net-broadcast-stations-labelled-waterfall-come = Le stazioni di radiodiffusione etichettate sulla waterfall provengono dall'elenco integrato in questa build. Per modificarle serve l'app desktop.
settings-net-ask-service-whether-these-credentials = Chiede al servizio se queste credenziali funzionano. Applica prima le impostazioni qui sopra, e non registra nulla: si limita a leggere.
settings-net-checking = verifica in corso…

## Settings window: messages with values filled in.

settings-amp-connected-to = Collegato al { $model }
settings-controls-last-midi = Ultimo messaggio: { $message }  valore { $value }
settings-general-band-plan-file = Ogni limite di banda e sotto-segmento proviene da { $path }, scritto dalle tabelle IARU integrate la prima volta e da allora modificabile da te — restringi una banda alla tua licenza e sdroxide rifiuterà di trasmettere al di fuori di essa. Le frequenze sono in MHz; elimina il file per ottenere una copia nuova dei predefiniti. Questa è l'assegnazione regionale, non la tua licenza: le tue condizioni personali possono essere più ristrette, e i piani nazionali differiscono all'interno di una regione.
settings-general-exported = Esportato { $note } — scegli dove salvarlo.
settings-general-export-failed = Esportazione non riuscita: { $error }
settings-general-import-failed = Importazione non riuscita: { $error }
settings-general-imported = { $summary } — riavvia sdroxide per usarli.
settings-general-import-skipped = Saltato { $path }: { $why }
settings-general-swr-tripped = ⚠ Attualmente scattata a { $swr }:1 — la trasmissione è bloccata.
settings-net-call-grid-set-on-general = { $call } / { $grid } — impostati nella scheda Generale
settings-net-reporting-as = Segnalazione come { $call } / { $grid } — SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ Download non riuscito: { $error }. Uso il programma integrato.
settings-net-test-service = Prova { $service }

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = La trasmissione è disattivata mentre un convertitore è impostato — indica ciò che c'è nella linea di trasmissione qui sopra per riattivarla.
settings-transmit-goes-through-same-converter = La trasmissione passa dallo stesso convertitore, quindi la radio lavora con lo stesso offset rispetto alla sintonia in entrambe le direzioni.
settings-transmit-not-converted-radio-transmits = La trasmissione non è convertita: la radio trasmette sulla frequenza indicata dalla sintonia, mentre la ricezione passa dal convertitore.
settings-transmit-takes-own-offset-separate = La trasmissione usa un proprio offset, separato da quello di ricezione.
settings-no-enabled-subscriptions-update = Nessun abbonamento attivo da aggiornare.
settings-add-radio-here-station-you = Aggiungi una radio — qui, oppure su una stazione a cui sei collegato

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = Ogni limite di banda e sotto-segmento proviene da bandplan.json sulla macchina a cui è collegata la radio. Questa è l'assegnazione regionale, non la tua licenza: le tue condizioni personali possono essere più ristrette, e i piani nazionali differiscono all'interno di una regione.

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = Immagini ricevute → FTP
settings-picup-uploads-every-received-sstv-picture = Invia ogni immagine SSTV ricevuta (e, se selezionate, ogni carta meteo) appena viene salvata, disposta come la fa QSSTV, così le pagine web scritte per QSSTV funzionano senza modifiche.
settings-picup-upload-received-pictures = Invia le immagini ricevute
settings-picup-server = Server
settings-picup-port = Porta
settings-picup-security = Sicurezza
settings-picup-user = Utente
settings-picup-password = Password
settings-picup-certificate = Certificato
settings-picup-trusted-when-computer-s-operating = Considerato attendibile quando il sistema operativo di questo computer lo considera tale, come farebbe un browser qui. Si può aggiungere sotto una CA privata (PEM).
settings-picup-extra-ca = CA aggiuntiva
settings-picup-only-certificate-fingerprint-accepted-way = Viene accettato solo il certificato con questa impronta — il modo per fidarsi di un server autofirmato. TEST mostra quella del server.
settings-picup-any-certificate-accepted-link-encrypted = ⚠ Viene accettato qualsiasi certificato. Il collegamento è cifrato, ma chiunque sul percorso può farsi passare per il server e leggere la password.
settings-picup-plain-ftp-sends-password-pictures = ⚠ L'FTP semplice invia la password e le immagini senza cifratura.
settings-picup-sstv = SSTV
settings-picup-sstv-dir = Cartella SSTV
settings-picup-weather-fax = Fax meteo
settings-picup-wefax-dir = Cartella WEFAX
settings-picup-empty-login-directory = Vuoto significa la cartella di accesso.
settings-picup-file-names = Nomi dei file
settings-picup-format = Formato
settings-picup-add-extension = aggiungi estensione
settings-picup-image1-png-rather-than-qsstv = image1.png anziché il semplice image1 di QSSTV
settings-picup-caption-bar = Barra didascalia
settings-picup-time-frequency-mode-sender-s = Ora, frequenza, modo e il FSK ID del mittente in cima, come lo disegna QSSTV. Viene contrassegnata solo la copia inviata.
settings-picup-fit-960-768 = Adatta a 960 × 768
settings-picup-qsstv-s-heartbeat-tiny-picture = Il battito di QSSTV: un'immagine minuscola la cui altezza è il minuto, inviata accanto a ciascuna, e resa larga due pixel alla chiusura del programma.
settings-picup-nat-workaround = Aggiramento NAT
settings-picup-ignore-private-address-server-s = Ignora un indirizzo privato nella risposta in modalità passiva del server e usa quello del server stesso — per un server dietro NAT che non conosce il proprio indirizzo pubblico.
settings-picup-apply = {" "}APPLICA{" "}
settings-picup-test = PROVA
settings-picup-connect-settings-above-applied-not = Collegati con le impostazioni qui sopra — applicate o no — accedi, e scrivi ed elimina un piccolo file in ciascuna cartella di invio.
settings-picup-checking = verifica in corso…
settings-picup-trust-certificate = Considera attendibile questo certificato
settings-picup-pin-switch-pinned-fingerprint-one = Fissalo: passa a "Impronta fissata" con questa. Verificala prima sul server — questo è ciò che il server ha detto, non una prova di chi sia. Poi APPLICA.
settings-picup-last-upload = Ultimo invio

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = Profili: le configurazioni salvate della stazione
settings-profiles-profile-captures-whole-working-setup = Un profilo cattura l'intera configurazione di lavoro — sintonie e VFO, modo e filtri, guadagni, pilotaggio e antenne, il tuo nominativo e i modelli di messaggio, e i registri per banda — e la riapplica con un clic. L'hardware (back end, dispositivi audio, convertitori) viene deliberatamente lasciato stare: un profilo è il modo in cui lavori con la stazione, non la radio su cui gira.
settings-profiles-save-current-setup = Salva la configurazione attuale
settings-profiles-profile-name = nome del profilo
settings-profiles-save = Salva
settings-profiles-applies-whatever-sdroxide-attached-dials = Si applica a ciò a cui sdroxide è collegato: sintonie e VFO, modo e filtri, livelli, guadagni, pilotaggio, antenne, l'identità digitale e le pile di banda.
settings-profiles-no-profiles-saved-yet-name = Nessun profilo salvato — dagli un nome sopra e premi Salva.
settings-profiles-saved-profiles = Profili salvati
settings-profiles-apply = Applica

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = Commutatore trasmissione/ricezione
settings-relay-waiting-station-s-t-r = In attesa della configurazione del commutatore T/R della stazione…
settings-relay-closes-contact-while-station-transmits = Chiude un contatto mentre questa stazione trasmette — per scollegare e mettere a massa l'ingresso antenna dell'SDR, e per comandare in sequenza un amplificatore o un relè T/R esterno. Funziona con le schede relè USB economiche (LCUS, KMtronic, Numato), con una linea seriale RTS/DTR verso qualunque interfaccia che voglia una chiusura PTT, e con qualsiasi altra cosa tramite un hook di comando.
settings-relay-hardware = Hardware
settings-relay-board = Scheda
settings-relay-lcus-four-byte-a0-command = LCUS è la scheda a comandi A0 da quattro byte venduta con una dozzina di nomi; anche KMtronic e Numato possono essere interrogati su come sono impostati
settings-relay-port = Porta
settings-relay-choose = — scegli —
settings-relay-device = Dispositivo
settings-relay-gpio-pin = Pin GPIO
settings-relay-pin-3-every-homebrew-plan = Il pin 3 su ogni piano fai-da-te e su ogni scheda commerciale — è quello in fondo al pacchetto a cui si può saldare un filo. Se il relè non scatta, la scheda è un clone che non implementa affatto questi pin.
settings-relay-gpio-chip = Chip GPIO
settings-relay-linux-only-gpiodetect-lists-chips = Solo Linux. `gpiodetect` elenca i chip della macchina.
settings-relay-transmit = In trasmissione
settings-relay-receive = In ricezione
settings-relay-nothing-switched = Non si commuta nulla.
settings-relay-starting-program-costs-unpredictable-amount = Avviare un programma costa una quantità di tempo imprevedibile, e nulla di esso viene conteggiato nell'anticipo — quindi lascia un margine più ampio qui di quanto faresti per una scheda relè. I numeri della tabella dei contatti non significano nulla per questo collegamento: i tuoi comandi vengono eseguiti una volta alla pressione e una volta al rilascio.
settings-relay-contacts = Contatti
settings-relay-contact-1-rts-contact-2 = Il contatto 1 è RTS e il contatto 2 è DTR.
settings-relay-no = N.
settings-relay-gpio = GPIO
settings-relay-name = Nome
settings-relay-job = Funzione
settings-relay-tx-closes = Chiude in TX
settings-relay-lead = Anticipo
settings-relay-hold = Tenuta
settings-relay-line-s-offset-chip-bcm = Lo scostamento della linea su questo chip — il numero BCM su un connettore di Raspberry Pi, che non è il numero del pin fisico
settings-relay-sdr-antenna = Antenna SDR
settings-relay-decides-default-timings-what-log = Decide i tempi predefiniti e il nome con cui lo registra il log — e, per un decodificatore di banda, che il contatto segue la tabella delle bande qui sotto
settings-relay-transmitting-energises-coil-off-transmitting =
    Attivo: trasmettere eccita la bobina. Disattivo: trasmettere la rilascia.

    This is a wiring decision before it is a setting — see the note below.
settings-relay-how-long-before-rf-contact = Quanto prima dell'RF questo contatto passa dalla parola RX della sua banda alla parola TX. Un cambio di sintonia lo sposta subito; questo serve per il key-down, quando un filtro solo-trasmissione deve essere inserito prima che arrivi la potenza.
settings-relay-closed-long-before-rf-small = Chiuso con questo anticipo rispetto all'RF. Un piccolo relè coassiale scatta in 5–15 ms. La trasmissione attende il più lungo di questi, quindi un valore alto è un vuoto udibile nell'audio in ricezione; oltre 250 ms viene ignorato.
settings-relay-how-long-after-rf-stops = Quanto dopo che l'RF si ferma questo contatto torna dalla sua parola TX alla sua parola RX.
settings-relay-opened-long-after-rf-stops = Si apre questo tempo dopo che l'RF si ferma. Più lungo dell'anticipo di proposito: lasciare tornare l'antenna con un attimo di ritardo non costa nulla, mentre farlo in anticipo costa uno stadio d'ingresso. Zero è una risposta valida, ed è quella giusta per la linea di comando di un amplificatore.
settings-relay-test = {" "}PROVA{" "}
settings-relay-not-while-station-air = Non mentre la stazione è in trasmissione
settings-relay-close-contact-half-second-so = Chiudi questo contatto per mezzo secondo, così puoi sentire il relè e controllare il cablaggio con il trasmettitore spento
settings-relay-remove = Rimuovi
settings-relay-add-contact = + aggiungi un contatto
settings-relay-if-will-not-answer = Se non risponde
settings-relay-switch-exists-protect-receiver-not = Un commutatore che esiste per proteggere un ricevitore non vale nulla se smette in silenzio di proteggerlo — quindi il rifiuto è l'impostazione predefinita. Scegli l'altra opzione dove un cavo USB allentato che interrompe la gara è l'esito peggiore.
settings-relay-order-timings-longest-lead-closes = L'ordine è dato dai tempi: l'anticipo più lungo chiude per primo, e la tenuta più lunga apre per ultima. Quindi dai al relè dell'antenna l'anticipo e la tenuta più lunghi, e all'amplificatore quelli più corti — così l'antenna viene sempre commutata prima che l'amplificatore sia comandato, e l'amplificatore si disattiva sempre prima che l'antenna torni.
settings-relay-transmit-sense-input = Ingresso di rilevamento trasmissione
settings-relay-when-transceiver-keys-itself-own = Quando un ricetrasmettitore si attiva da solo — il proprio tasto del microfono, un pedale, VOX o il keyer — sdroxide lo scopre chiedendolo via CAT, e quella risposta arriva qualche centinaio di millisecondi dopo l'inizio della trasmissione. Collega la linea SEND dell'apparato, tramite un optoisolatore, a un ingresso di handshake su questa stessa porta, e viene rilevato in pochi millisecondi. Con questo il misuratore e l'interblocco di trasmissione diventano più rapidi.
settings-relay-line = Linea
settings-relay-transmitting = In trasmissione è
settings-relay-high-line = una linea alta
settings-relay-most-opto-isolated-interfaces-pull = La maggior parte delle interfacce optoisolate porta la linea a massa quando l'apparato trasmette, quindi di solito questa è disattivata
settings-relay-belongs-radio = Appartiene alla radio
settings-relay-which-radio-tab-sensed-transceiver = A quale scheda radio appartiene il ricetrasmettitore rilevato. In una stazione con una sola radio è 1 e nulla dipende da questo.
settings-relay-needs-serial-port-handshake-lines = Richiede una porta seriale — su di essa ci sono le linee di handshake.
settings-relay-wiring-only-fail-safe-survives = Il cablaggio è l'unica protezione di sicurezza che sopravvive a questo programma.
settings-relay-choose-tx-closes-so-de = Scegli “Chiude in TX” così che il contatto *diseccitato* sia lo stato che vuoi quando la scheda viene staccata, il computer è spento o sdroxide non è in funzione. Cablato in modo che diseccitato significhi a massa, un relè guasto lascia l'SDR sordo — fastidioso, ma sicuro. Cablato al contrario, un relè guasto più una trasmissione significa uno stadio d'ingresso morto. Con un vero amplificatore a valle, scegli la prima.
settings-relay-cannot-protect-first-instant-over = Questo non può proteggere il primo istante di una trasmissione che avvii dalla radio. Se comandi l'apparato da sdroxide, i contatti precedono sempre l'RF; con l'ingresso di rilevamento collegato come sopra, la seguono in pochi millisecondi. Per uno stadio d'ingresso davvero da proteggere, usa anche un commutatore T/R hardware sensibile all'RF — nessun programma su un computer può essere la soluzione completa.
settings-relay-contacts-transmit = contatti in TRASMISSIONE
settings-relay-not-open = Non aperto.
settings-relay-status-unknown-press-apply = Stato sconosciuto — premi APPLICA.
settings-relay-station-air-apply-will-take = La stazione è in trasmissione — APPLICA avrà effetto dopo questa trasmissione.
settings-relay-band-decoder-outputs = Uscite del decodificatore di banda
settings-relay-one-control-word-per-band = Una parola di controllo per banda: il bit 0 è il contatto 1, il bit 1 il contatto 2 e così via, come nella numerazione qui sopra. RX segue la banda della sintonia di ricezione; TX la banda di qualunque radio abbia avviato la trasmissione, inserita sull'anticipo di ciascun contatto prima dell'RF e rilasciata dopo la sua tenuta — dai lo stesso valore a entrambi per un filtro a cui non interessa in quale direzione vada l'RF, e valori diversi per un bypass di preamplificatore solo in ricezione o un banco di filtri passa-basso solo in trasmissione. Le bande lasciate a 0 non attivano nulla. Ha effetto con Applica / riconnetti.
settings-relay-band = Banda
settings-relay-rx = RX
settings-relay-tx = TX
settings-relay-outputs-asserted = Uscite attive
settings-relay-other = Altro
settings-relay-everywhere-outside-amateur-bands-short = Ovunque fuori dalle bande radioamatoriali — ascolto in onde corte, e tutto ciò su cui cade la sintonia di un transverter che nessuna banda copre.
settings-relay-apply = {" "}APPLICA{" "}
settings-relay-persist-re-open-switch = Salva e (ri)apri il commutatore

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = Collegati a un server
settings-remote-drive-sdroxide-running-somewhere-else = Comanda via rete un sdroxide in funzione altrove — la macchina dello shack, un sito remoto. Avvialo lì con --server, e inserisci qui il suo indirizzo. La radio, i decodificatori e il logbook restano su quella macchina; questo schermo riceve la waterfall e l'audio, e usa il proprio altoparlante e microfono.
settings-remote-address = Indirizzo
settings-remote-host-name-ip = nome host o IP
settings-remote-machine-running-sdroxide-server-host = La macchina che esegue sdroxide --server: un nome host, un indirizzo IPv4 o IPv6, oppure un URL ws:// (o wss://) completo se il server si trova dietro un reverse proxy — un URL viene usato esattamente come scritto e ignora la porta e l'interruttore qui sotto.
settings-remote-port = Porta
settings-remote-port-server-listens-server-port = La porta su cui ascolta quel server — server_port nel suo config.toml. 4950 se non gli è stata data --port.
settings-remote-secure = Sicuro
settings-remote-wss-tls = wss:// (TLS)
settings-remote-tick-where-server-sits-behind = Seleziona questa opzione quando il server sta dietro qualcosa che termina HTTPS per lui — un reverse proxy sulla porta 443, che è il modo abituale per raggiungere una stazione attraverso internet aperta. Il server di sdroxide parla ws:// semplice e non la vuole.
settings-remote-connect = {" "}COLLEGA{" "}
settings-remote-open-station-radio-tab-own = Apri questa stazione come scheda radio a sé
settings-remote-client-cannot-hold-second-connection = Questo client non può mantenere una seconda connessione.

    If the server asks for a username and password, its sign-in screen comes up in the new tab. A plain ws:// link carries none of this encrypted, so across the open internet either put it through a VPN or an SSH tunnel, or terminate HTTPS in front of the server and tick Secure above.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = Trasmissione UDP WSJT-X
settings-servers-waiting-station-s-wsjt-x = In attesa della configurazione di trasmissione WSJT-X della stazione…
settings-servers-sends-decodes-station-status-logged = Invia decodifiche, stato della stazione e QSO registrati nel modo di WSJT-X, così GridTracker, JTAlert, N1MM+ e Log4OM funzionano con sdroxide senza modifiche. Solo in uscita — nulla su questo socket può toccare la radio.
settings-servers-enable = Attiva
settings-servers-send = Invia a
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 raggiunge i client su questa macchina; un indirizzo LAN o un gruppo multicast (224.0.0.1) raggiunge gli altri
settings-servers-port = Porta
settings-servers-t-2237-port-every-client-defaults = 2237 è la porta predefinita di ogni client
settings-servers-identify = Identificati come
settings-servers-name-clients-see-some-loggers = Il nome che vedono i client. Alcuni logger accettano solo traffico che si dichiara WSJT-X.
settings-servers-n1mm-contactinfo-broadcast = Trasmissione contactinfo N1MM+
settings-servers-same-news-n1mm-s-words = Le stesse notizie con le parole di N1MM: un datagramma XML per ogni contatto registrato, per i logger che ascoltano N1MM anziché WSJT-X — il ponte della World Radio League accetta entrambi. Solo contatti; in questo protocollo non c'è un flusso di decodifiche. Ha un proprio interruttore e una propria porta, perché una stazione può volerli entrambi.
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 raggiunge i client su questa macchina. Il consiglio di N1MM stesso per una rete di gara è l'indirizzo di broadcast di questa sottorete — 192.168.1.255 per una rete 192.168.1.n — che raggiunge tutte le postazioni insieme.
settings-servers-t-12060-port-n1mm-s-documentation = 12060 è la porta che raccomanda la documentazione di N1MM
settings-servers-station-name = Nome della stazione
settings-servers-what-n1mm-calls-stationname-name = Ciò che N1MM chiama StationName: il nome del computer che ha inviato il pacchetto. I logger lo mostrano per distinguere una postazione operativa dall'altra.
settings-servers-cw-panel-s-copy-plain = La copia del pannello CW come testo semplice, un datagramma UDP per ogni pezzo non appena si stabilizza — per un controller di stazione, la finestra CW di un logger o uno script. Nessun protocollo: un ascoltatore accoda semplicemente ciò che arriva (prova nc -ul 9999). Viene inviato solo testo stabilizzato, mai la coda che il decodificatore sta ancora rivedendo. Ogni radio ha il proprio interruttore.
settings-servers-apply = {" "}APPLICA{" "}
settings-servers-persist-re-open-broadcast-sockets = Salva e (ri)apri i socket di trasmissione
settings-servers-hamlib-rigctld-server = Server rigctld di Hamlib
settings-servers-waiting-station-s-rigctld-configuration = In attesa della configurazione rigctld della stazione…
settings-servers-listen = Ascolta su
settings-servers-t-127-0-0-1-machine = 127.0.0.1 (questa macchina)
settings-servers-t-0-0-0-0-whole = 0.0.0.0 (intera rete)
settings-servers-t-4532-port-every-rigctld-client = 4532 è la porta che presuppone ogni client rigctld
settings-servers-rig-name = Nome dell'apparato
settings-servers-reported-clients = riportato ai client
settings-servers-max-clients = Client massimi
settings-servers-allow-clients-transmit = Consenti ai client di trasmettere
settings-servers-off-refuses-every-key-request = Disattivato rifiuta ogni richiesta di trasmissione e smette di pubblicizzare un intervallo di trasmissione, così Hamlib stessa rifiuta di attivare il tasto.
settings-servers-rigctld-has-no-authentication-0 = ⚠ rigctld non ha autenticazione. Su 0.0.0.0, chiunque possa raggiungere questa porta può sintonizzare e attivare il tuo trasmettitore.
settings-servers-t-1-client = 1 client
settings-servers-not-running = Non in funzione.
settings-servers-status-unknown-press-apply = Stato sconosciuto — premi APPLICA.
settings-servers-persist-re-bind-server = Salva e (ri)associa il server
settings-servers-lets-any-hamlib-capable-program = Consente a qualunque programma compatibile con Hamlib di comandare questa radio: frequenza, modo, PTT, split, RIT e potenza. In WSJT-X, fldigi o CQRLOG scegli l'apparato “Hamlib NET rigctl” (modello 2) e puntalo a questo indirizzo; in GPredict e N1MM inserisci direttamente host e porta. A differenza del server TCI, porta solo controllo — niente audio, niente IQ.
settings-servers-antenna-rotator = Rotore d'antenna
settings-servers-waiting-station-s-rotator-configuration = In attesa della configurazione del rotore della stazione…
settings-servers-host = Host
settings-servers-min-elevation = Elevazione minima
settings-servers-below-rotator-parks-instead-tracking = Sotto questo valore il rotore si mette in posizione di riposo anziché seguire — impostalo sul tuo orizzonte locale o sul profilo dei tetti
settings-servers-azimuth-offset = Offset di azimut
settings-servers-added-every-command-rotator-whose = Aggiunto a ogni comando, per un rotore il cui nord è sfasato
settings-servers-min-movement = Movimento minimo
settings-servers-steps-smaller-than-are-not = I passi più piccoli di questo non vengono inviati — inseguire decimi di grado consuma i motori senza motivo
settings-servers-park-position = Posizione di riposo
settings-servers-drive-position-when-idle = porta in una posizione quando è inattivo
settings-servers-off-leaves-antenna-wherever-last = Disattivato lascia l'antenna dove è finita l'ultima passata
settings-servers-not-connected = Non collegato.
settings-servers-waiting-station-s-tci-server = In attesa della configurazione del server TCI della stazione…
settings-servers-built-tci-server = Server TCI integrato
settings-servers-device-name = Nome del dispositivo
settings-servers-off-leaves-control-receive-streams = Disattivato lascia funzionanti controllo e flussi di ricezione, ma ogni richiesta di trasmissione viene rifiutata.
settings-servers-tci-has-no-authentication-0 = ⚠ TCI non ha autenticazione. Su 0.0.0.0, chiunque possa raggiungere questa porta può sintonizzare e attivare il tuo trasmettitore.

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = Satelliti: elementi orbitali e frequenze
settings-tle-waiting-station-s-satellite-configuration = In attesa della configurazione satellitare della stazione…
settings-tle-tracker-already-fetches-celestrak-s = Il tracker scarica già da solo il gruppo radioamatoriale di CelesTrak. Questo serve per tutto il resto: i satelliti meteo NOAA, un cubesat troppo recente per essere nel gruppo, o un insieme di elementi più aggiornato di quello arrivato. Gli elenchi vengono scaricati — e conservati — dove gira il motore radio, quindi è lo stesso insieme di satelliti ovunque sia aperta l'app.
settings-tle-subscriptions = Abbonamenti
settings-tle-listings-fetched-kept-current-same = Elenchi scaricati e mantenuti aggiornati, con la stessa cadenza di sei ore del gruppo radioamatoriale. Vengono aggiornati mentre la finestra solare è aperta, e con AGGIORNA ORA qui.
settings-tle-fetch-track-listing = Scarica e traccia questo elenco
settings-tle-remove-subscription = Rimuovi questo abbonamento
settings-tle-orbits = Orbite
settings-tle-which-satellites-listing-get-orbit = Quali satelliti di questo elenco ottengono un anello orbitale e un'etichetta. Un gruppo intero vuole “selezionati”: novanta anelli insieme sono illeggibili, e nessuno lascia novanta puntini anonimi.
settings-tle-nothing-listing-sdroxide-s-curated = Nulla in questo elenco è nell'elenco selezionato di sdroxide — quell'elenco contiene dieci satelliti radioamatoriali, quindi questo si comporterebbe esattamente come “nessuno”.
settings-tle-all-satellites = tutti i satelliti
settings-tle-catalogue-numbers-keep-comma-separated = Numeri di catalogo da conservare, separati da virgole. Vuoto traccia tutto ciò che contiene l'elenco.
settings-tle-not-fetched-yet = non ancora scaricato
settings-tle-subscription = + Abbonamento
settings-tle-update-now = {" "}AGGIORNA ORA{" "}
settings-tle-ask-radio-engine-fetch-every = Chiedi al motore radio di scaricare ora ogni abbonamento attivo
settings-tle-celestrak-groups = Gruppi CelesTrak
settings-tle-pasted-element-sets = Elementi orbitali incollati
settings-tle-one-off-these-do-not = Per un'una tantum. Questi non si aggiornano da soli, e SGP4 smette di propagare un insieme di elementi una volta che è passata una quindicina di giorni dalla sua epoca — per tutto ciò che intendi conservare, abbonati.
settings-tle-track-one = Traccia questo
settings-tle-show-two-element-lines = Mostra le due righe degli elementi
settings-tle-remove = Rimuovi
settings-tle-paste-two-three-line-element = Incolla qui insiemi di elementi a due o tre righe
settings-tle-add-pasted = + Aggiungi incollati
settings-tle-nothing-paste-box-looked-like = Nulla nella casella di incolla somigliava a un insieme di elementi.
settings-tle-clear-box = Svuota la casella
settings-tle-frequencies = Frequenze
settings-tle-shown-under-pass-table-solar = Mostrate sotto la tabella dei passaggi nella finestra solare. Una voce qui sostituisce del tutto quella integrata per quel numero di catalogo, quindi parti da una copia di essa a meno che tu non voglia eliminare le altre.
settings-tle-remove-satellite-s-entry = Rimuovi la voce di questo satellite
settings-tle-link = COLLEGAMENTO
settings-tle-downlink = DISCESA
settings-tle-uplink = SALITA
settings-tle-mode = MODO
settings-tle-note = NOTA
settings-tle-fm-repeater = Ripetitore FM
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67,0 Hz
settings-tle-link-2 = + Collegamento
settings-tle-copy-built = Copia integrata
settings-tle-frequency-either-one-number-145 = Una frequenza è un solo numero (145.800) oppure una banda passante del transponder scritta 145.950-145.970. Lascia vuota una direzione per un radiofaro.
settings-tle-norad = NORAD
settings-tle-satellite = + Satellite
settings-tle-frequency-entry-needs-satellite-s = Una voce di frequenza richiede il numero di catalogo NORAD del satellite.

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = Indirizzo
settings-wavelog-api-key = Chiave API
settings-wavelog-your-wavelog-s-front-page = L'indirizzo della pagina principale del tuo Wavelog — https://log.example.org oppure http://192.168.1.10/wavelog — e una chiave dalla sua pagina API. L'invio richiede una chiave in lettura/scrittura.
settings-wavelog-plain-http-address-internet-api = http:// semplice verso un indirizzo su internet: la chiave API viaggia non cifrata. Va bene nella tua rete; attraverso internet usa https://.
settings-wavelog-trusted-cert = Certificato attendibile
settings-wavelog-forget-certificate = Dimentica questo certificato
settings-wavelog-go-back-ordinary-checking-against = Torna al normale controllo sulle autorità di certificazione pubbliche — per un server che ora ha un certificato regolare.
settings-wavelog-connection-check-runs-machine-radio = Il controllo della connessione viene eseguito sulla macchina a cui è collegata la radio — usa lì le impostazioni per provarlo ed elencare le ubicazioni della stazione.
settings-wavelog-server-presented-certificate-sha-256 = Il server ha presentato questo certificato (SHA-256):
settings-wavelog-compare-your-server-s-e = Confrontalo con quello del tuo server — ad es. `openssl x509 -noout -fingerprint -sha256 -in cert.pem` — e consideralo attendibile solo se coincidono. Da quel momento viene accettato questo esatto certificato e ogni altro viene rifiutato.
settings-wavelog-trust-certificate = Considera attendibile questo certificato
settings-wavelog-station-location = Ubicazione della stazione
settings-wavelog-none = — nessuna —
settings-wavelog-press-test-above-list-station = Premi Prova qui sopra per elencare le ubicazioni della stazione definite nel tuo Wavelog.
settings-wavelog-default = Predefinita
settings-wavelog-where-qsos-go-when-operating = Dove vanno i QSO quando il profilo operativo non indica un'ubicazione.
settings-wavelog-profile = Questo profilo
settings-wavelog-use-default = Usa la predefinita
settings-wavelog-not-set = non impostata
settings-wavelog-no-station-location-chosen-uploads = Nessuna ubicazione della stazione scelta — gli invii falliranno finché non ne viene scelta una.
settings-wavelog-qsos-not-wavelog = QSO non presenti in Wavelog
settings-wavelog-every-qso-logbook-wavelog = Ogni QSO del logbook è in Wavelog.
settings-wavelog-everything-logbook-wavelog-has-not = Tutto ciò che il logbook contiene e Wavelog non ha confermato — inclusi i contatti precedenti all'invio automatico. Quelli che ha già tornano come duplicati e vengono semplicemente segnati come inviati.
settings-wavelog-set-address-key-station-location = Imposta prima l'indirizzo, la chiave e un'ubicazione della stazione.
settings-wavelog-yes-send-them = Sì, inviali
settings-wavelog-cancel = Annulla
settings-wavelog-download-into-logbook = Scarica nel logbook
settings-wavelog-downloading-runs-where-radio-attached = Lo scaricamento viene eseguito dove è collegata la radio, nel logbook di quella macchina — configuralo lì.
settings-wavelog-download-qsos-start-every = Scarica i QSO all'avvio e ogni
settings-wavelog-downloaded-qsos-join-logbook-so = I QSO scaricati entrano nel logbook, così FT8 e le liste degli spot sanno cosa hai già lavorato da altri programmi e apparati. Ogni scaricamento prende solo ciò che è nuovo.
settings-wavelog-press-test-above-choose-which = Premi Prova qui sopra per scegliere quali ubicazioni della stazione scaricare.
settings-wavelog-sync-now = Sincronizza ora
settings-wavelog-fetch-what-wavelog-has-added = Scarica ciò che Wavelog ha aggiunto dall'ultimo scaricamento. Applica prima le impostazioni qui sopra.
settings-wavelog-full-resync = Risincronizzazione completa…
settings-wavelog-download-ticked-locations-whole-compare = Scarica interamente le ubicazioni selezionate e confrontale con il log: i QSO modificati o cancellati in Wavelog, oppure provenienti da un'ubicazione non più selezionata, vengono riportati in linea. Vedi le modifiche prima che accada qualcosa.
settings-wavelog-full-resync-changes-make = Risincronizzazione completa — modifiche da fare
settings-wavelog-qsos-logged-here-imported-file = I QSO registrati qui o importati da un file non vengono mai modificati o rimossi — solo quelli scaricati da Wavelog.
settings-wavelog-apply = Applica
settings-wavelog-discard = Scarta
settings-wavelog-live-radio-status = Stato radio in tempo reale
settings-wavelog-send-frequency-mode-wavelog-they = Invia frequenza e modo a Wavelog man mano che cambiano
settings-wavelog-radio-name = Nome della radio
settings-wavelog-pick-radio-wavelog-s-live = Scegli questa radio nel logging in tempo reale di Wavelog e il suo modulo segue la sintonia. Richiede una chiave in lettura/scrittura. Solo dalla radio principale della stazione.

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = Impronta SHA-256 del certificato del server: { $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = Riporta la stazione su “{ $name }”: sintonie, VFO, modo, filtri, guadagni, pilotaggio, antenne, identità e pile di banda.
settings-profiles-delete = Elimina “{ $name }”

## Settings window, T/R switch: messages with values

settings-relay-not-usable = Non ancora utilizzabile: { $why }
settings-relay-receiving = in ricezione
settings-relay-rx-tx-words = { $rx } / TX { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n ->
        [one] { $n } client
       *[other] { $n } client
    }
settings-servers-listening = ● In ascolto su { $addr } — { $clients }
settings-servers-failed = Errore: { $error }
settings-servers-rotator-at = ● Collegato — antenna a { $az }° az, { $el }° el

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } satelliti · { $age } di età
settings-tle-subscribed = Abbonato a { $name }. Premi AGGIORNA ORA per scaricarlo.
settings-tle-norad-too-stale = NORAD { $norad } · { $age } di età — troppo vecchio per propagare
settings-tle-norad-age = NORAD { $norad } · { $age } di età
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = Aggiunt{ $n ->
        [one] o { $n } insieme di elementi
       *[other] i { $n } insiemi di elementi
    }.
settings-tle-added-refreshed = Aggiunt{ $added ->
        [one] o { $added }
       *[other] i { $added }
    } e aggiornat{ $r ->
        [one] o { $r } insieme di elementi
       *[other] i { $r } insiemi di elementi
    }.
settings-tle-links = { $n } { $n ->
        [one] collegamento
       *[other] collegamenti
    }
settings-tle-replace-links = Sostituisci questi collegamenti con quelli integrati per { $name }

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = L'ubicazione "{ $location }" è { $station_call }, ma tu registri come { $call } — Wavelog rifiuterà questi QSO.
settings-wavelog-grid-mismatch = L'ubicazione "{ $location }" si trova su { $station_grid }, ma il tuo locatore è { $grid } — Wavelog rifiuterà questi QSO.
settings-wavelog-location-n = ubicazione #{ $id }
settings-wavelog-saved-with-callsign = Salvata con il nominativo ({ $call }) in Impostazioni → Profili, così un profilo portatile può registrare su un'ubicazione portatile.
settings-wavelog-owed = { $n } { $n ->
        [one] QSO
       *[other] QSO
    } registrat{ $n ->
        [one] o
       *[other] i
    } dall'attivazione dell'invio automatico ancora da inviare — nuovo tentativo ogni 5 minuti.
settings-wavelog-send-unsent = Invia { $n } { $n ->
        [one] QSO
       *[other] QSO
    } non ancora presenti in Wavelog…
settings-wavelog-send-confirm = Inviare { $n } QSO a { $location }? Le impostazioni vengono applicate prima.
settings-wavelog-plan-add = • { $n } QSO presenti in Wavelog da aggiungere al log
settings-wavelog-plan-update = • { $n } QSO scaricati modificati in Wavelog da aggiornare
settings-wavelog-plan-remove = • { $n } QSO scaricati non più presenti in Wavelog (o provenienti da un'ubicazione non selezionata) da rimuovere
settings-wavelog-plan-mark = • { $n } dei tuoi QSO da segnare come presenti in Wavelog

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = Antenna
settings-radio-which-socket-back-radio-using =
    Quale presa sul retro usa la radio — il suo comando ANT, la stessa impostazione del pulsante ANT sul pannello frontale.

    Applies immediately, and it is the radio's setting rather than a copy kept here: the socket the radio is on is read back when the session opens.

    The choice is remembered per band. Switch to the beam on 2 m and the wire on 40, and each comes back the next time the dial crosses into that band — and the next time sdroxide starts. A memory channel stored here carries its socket too.
settings-radio-receive-antenna = Antenna di ricezione
settings-radio-radio-s-separate-receiving-antenna =
    L'antenna di ricezione separata della radio — RX ANT sul retro — inserita nel percorso di ricezione o esclusa da esso. È un'impostazione propria, la stessa del pulsante RX ANT della radio.

    The aerial on the main socket stays on transmit either way: this is an extra input for receiving, a loop or a beverage, not a choice of socket.

    Unlike the antenna above, this is *not* remembered here. The radio holds it per band itself, so sdroxide reads it back after every band change and shows what the radio says; ticking this box is the only thing that moves it.
settings-radio-receive-path = Nel percorso di ricezione
settings-radio-radio-power = Alimentazione della radio
settings-radio-switch-radio-itself-off-back =
    Spegni la radio stessa, e riaccendila, tramite il collegamento di controllo. È l'unico vero interruttore on/off del programma — non l'interruttore LINK di sdroxide, che chiude la parte di sdroxide e lascia la radio accesa.

    For the switch back on to reach anything, the radio's control end has to stay awake while it is off. Over the network that is what Network Control does. Over a serial cable it is the radio's CI-V port, which stays powered from the mains supply on a set that is switched off at the front rather than unplugged; sdroxide sends the wake-up run Icom's own documentation asks for in front of the power-on.

    Switching off ends the audio and the meters — there is no radio behind them — and sdroxide keeps the control link open so the switch back on has somewhere to go.
settings-radio-on = Acceso
settings-radio-off = Spento
settings-radio-waiting-configuration-machine-radio-attached = In attesa della configurazione della macchina a cui è collegata la radio.
settings-radio-sound-format = Formato audio
settings-radio-invert-spectrum = Inverti lo spettro
settings-radio-swap-i-q = Scambia I/Q
settings-radio-mirror-panadapter-about-tuned-frequency =
    Specchia il panadapter rispetto alla frequenza sintonizzata, per un apparato che porta I e Q invertiti sulla sua scheda audio. Il segnale rivelatore è una waterfall piena di segnali convincenti, tutti dalla parte sbagliata della sintonia, con la SSB che esce sulla banda laterale opposta — scambiare i due cavi sulla scheda audio risolverebbe allo stesso modo.

    Receive only: transmit hands the radio one real audio signal, which has no sideband to invert.
settings-radio-i-q-sample-rate = Frequenza di campionamento I/Q
settings-radio-how-fast-radio-s-i =
    Quanto velocemente viene fatta girare la scheda audio I/Q della radio — e quindi quanta banda mostra il panadapter, perché un flusso in quadratura copre tutta la sua frequenza di campionamento: 48 kHz danno ±24 kHz per lato rispetto alla sintonia, 192 kHz danno ±96.

    The card has the final say. One that cannot do the rate picked here is opened at the nearest it can, and the panadapter is that much narrower — the log says so at startup when it happens.

    A faster card is also more work: the machine has proportionally less time to empty it, and one that cannot keep up drops samples. That shows up as audio breaking up while the waterfall still looks perfect, and it too is logged. If you see it, come back down a step.
settings-radio-t-48-khz-24-khz = 48 kHz (±24 kHz)
settings-radio-t-96-khz-48-khz = 96 kHz (±48 kHz)
settings-radio-t-192-khz-96-khz = 192 kHz (±96 kHz)
settings-radio-t-384-khz-192-khz = 384 kHz (±192 kHz)
settings-radio-i-q-centre-offset = Offset del centro I/Q
settings-radio-how-far-radio-s-i =
    Di quanto l'uscita I/Q della radio è centrata sopra la propria sintonia, per un apparato la cui I.F. di ricezione è stata spostata lontano da zero — su un Elecraft, MENU:RX SHFT impostato su 8.0 anziché NOR, che allontana la sintonia dal picco DC del mixer (e impedisce che una stazione SSB/AM forte vicina venga rivelata in AM).

    Not a converter: the radio still displays and transmits on the real frequency, and nothing here is ever sent to it. This only says where the samples on the sound card sit, and the stream is shifted back onto the dial as it arrives.

    Leave at 0 unless you have turned such a menu entry on. If signals land twice the offset away, the sign is the other way round.
settings-radio-iq-correction = Correzione IQ
settings-radio-cancel-mirror-image-every-signal =
    Cancella l'immagine speculare di ogni segnale, e il picco DC al centro della waterfall.

    A radio's I/Q output is two analogue paths that are never quite equal in gain nor exactly 90° apart, and what that leaves is a copy of every signal reflected about the tuned frequency, usually 30-40 dB down — strong enough to look like a station that is not there, and to be decoded as one. Radios with front-panel balance trimmers are adjusting exactly this; here it is measured off the received noise and needs no setting.

    Leave it on. Turn it off if you are listening to AM tuned dead on the carrier — the carrier is DC, so it goes with the spike — or to check whether a signal is real by watching whether it survives.
settings-radio-cancel-mirror-images = Cancella le immagini speculari
settings-radio-dc-notch = Notch DC
settings-radio-widen-hole-taken-out-middle =
    Allarga il buco ricavato al centro dell'ampiezza, per una radio il cui picco centrale è più largo dell'offset sottostante.

    0 leaves the ordinary blocker, which is a few tens of hertz wide and removes the offset without touching anything else. Wind it up and the bottom of the span goes with it: a first-order high-pass, 3 dB down at the figure set here and further in below it.

    It is centred where the radio's I/Q is centred, which is the dial unless the offset above has moved it — so anything tuned there goes too. A CW note at 600 Hz is inside a 600 Hz setting.
settings-radio-panadapter-bw = Larghezza di banda del panadapter
settings-radio-cat-family = Famiglia CAT
settings-radio-rigctld-address = Indirizzo rigctld
settings-radio-host-port-running-hamlib-rigctld =
    host:porta di un rigctld di Hamlib in funzione — 127.0.0.1:4532 è il suo predefinito, su questa macchina.

    Start one with, for example, `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, where -m is the Hamlib model number for your radio (`rigctl -l` lists them).

    This is the catch-all: it reaches the frequency, mode, PTT, power, S-meter and SWR and nothing else. Where one of the native families above fits your radio it does more — keying from the rig's own text buffer, the receive filter, and per-model meter scales.
settings-radio-flrig-address = Indirizzo flrig
settings-radio-host-port-running-flrig-127 =
    host:porta di un flrig in funzione — 127.0.0.1:12345 è il suo predefinito, su questa macchina. flrig serve XML-RPC ogni volta che è in funzione; la porta si trova in Config → Setup → Server.

    Like the Hamlib option this drives a daemon rather than the radio, but through flrig's own per-model driver — on a number of rigs its power and filter handling is the more faithful of the two. It reaches the frequency, mode, PTT, transmit power (in whole watts), the receive bandwidth, the S-meter, SWR and power-out. CW keys through flrig's own cwio port, which must be configured in flrig itself.
settings-radio-serial-port = Porta seriale
settings-radio-select = — seleziona —
settings-radio-baud = Baud
settings-radio-data-bits = Bit di dati
settings-radio-parity = Parità
settings-radio-stop-bits = Bit di stop
settings-radio-force-rts = Forza RTS
settings-radio-force-dtr = Forza DTR
settings-radio-ptt-method = Metodo PTT
settings-radio-how-transmit-keyed = Come viene attivata la trasmissione.
settings-radio-how-transmit-keyed-network-link = Come viene attivata la trasmissione. Un collegamento di rete non ha linee di controllo, quindi DTR e RTS qui non attivano nulla — usa CAT, che chiede al demone di attivare la radio, oppure VOX.
settings-radio-mode-control = Controllo del modo
settings-radio-digimode-mode = Modo per i modi digitali
settings-radio-what-put-radio-mode-sdroxide =
    In quale modo mettere la radio per un modo che sdroxide modula tramite la scheda audio. "DIGI" seleziona la posizione DATA/PKT dell'apparato, che prende l'audio di trasmissione dall'ingresso USB o ACC senza l'elaborazione vocale del percorso microfono; "USB" lo lascia sulla banda laterale semplice, per una radio già impostata a modulare la SSB dal suo ingresso dati o che non ha affatto una posizione DATA.

    It covers every digital mode — FT8, FT4, PSK, RTTY, SSTV and the rest — and CW sent as "Sound card", and it overrides Mode control for them. SSTV is the one whose sideband follows the band, so there it means the DATA position on that sideband: USB-D above 40 m, LSB-D at and below it. SSTV-FM is not part of it — an FM carrier has no sideband to choose.
settings-radio-cw-keying = Manipolazione CW
settings-radio-how-cw-panel-s-keyer =
    Come trasmette il manipolatore del pannello CW. "Manipolatore dell'apparato" mette la radio in CW e le consegna il testo da inviare con il proprio manipolatore. Usa la velocità del manipolatore dell'apparato (impostata dal WPM del pannello), e su Yaesu invia tramite la memoria del manipolatore 1, sovrascrivendo ciò che vi era memorizzato.

    It needs break-in on at the radio, which sdroxide turns on itself with each message on Icom, Yaesu and Kenwood — a rig with break-in off takes the message and transmits nothing. It is left on afterwards, and a full break-in is never turned down to semi.

    "Sound card" sends the keyed sidetone as audio instead (MCW), a tone at dial + pitch — and because a rig in CW would ignore its sound card entirely, selecting CW then follows the Digimode mode setting (USB, DATA, or Radio controlled) instead of switching the rig to CW. Pick "Radio controlled" there for rigs whose data position can't be commanded over CAT (a Xiegu's U-D) and park the rig on it yourself, as for FT8.
settings-radio-poll-rate = Frequenza di interrogazione
settings-radio-how-often-radio-asked-what =
    Ogni quanto viene chiesto alla radio cosa sta facendo — la sua sintonia, il suo modo e i suoi misuratori. È la metà del collegamento di controllo che va dalla radio a sdroxide: gira la manopola VFO dell'apparato o cambia il suo modo dal pannello frontale, e la lettura, la banda e il panadapter seguono entro un'interrogazione.

    It is also the whole of the control traffic this end generates, and on a modern Icom that traffic is not free: the CI-V port and the sound card sit behind one USB hub inside the radio, so frames asked for here are bus time the audio does not get — heard as dropouts that look like a DSP fault and are not one. The default of 2 Hz is half a second behind the knob and quiet enough to stay out of the audio's way. Raise it where the control port is its own device (a separate USB-serial adapter, a network rigctld); lower it if the radio's audio breaks up.

    An Icom with CI-V Transceive switched on in its own menu needs none of this for the dial: it reports the knob the instant it moves, and sdroxide stands the dial poll down as soon as it sees it do so — and picks it back up again if the radio ever moves without saying so.

    The rate is the dial's. The mode rides along with only every fourth poll, since it is a setting that changes a few times in an evening rather than one that follows a knob.
settings-radio-send-command = Comando di invio
settings-radio-which-transceiver-generation-keys-rig =
    Quale generazione di ricetrasmettitori attiva l'apparato, per il metodo PTT "CAT". Le due non concordano su cosa significhi il parametro TX e nulla sul filo le distingue, quindi scegli quella che corrisponde alla tua radio.

    "TS-2000 style (TX;)" — TS-480, TS-570, TS-870, TS-2000: the ordinary send, on the main band.

    "TS-590 style (TX1;)" — TS-590S/SG, TS-890, TS-990: DATA SEND, which keys with the ACC2/USB audio input live. On these rigs the plain send selects the microphone instead and mutes the audio sdroxide transmits.

    Set wrong, a TS-590 transmits silence — but a TS-2000 transmits on the sub-band, which is another band entirely.
settings-radio-radio = Radio
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    Un solo profilo copre la famiglia: il set di comandi K3, a cui rispondono KX3, KX2 e K4. Qui non c'è nulla da scegliere — quanti watt copre il cursore Pilotaggio (12 su un KX2 o KX3 nudo, 110 con un KPA3 o un KXPA100) viene letto dalla richiesta dei moduli opzionali dell'apparato stesso all'apertura della porta.

    Note the baud rates above: the K3, K3S, KX3 and KX2 go no faster than 38400, and a rig set below the rate chosen here answers nothing at all.
settings-radio-transmit-input = Ingresso di trasmissione
settings-radio-where-radio-takes-transmit-audio =
    Da dove la radio prende l'audio di trasmissione — il comando TI dell'apparato, che è il menu 32 "TX IN" sul pannello frontale. Viene impostato all'apertura della porta.

    "USB audio" is what makes this interface work: the FDM-DUO transmits what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.

    "Auto" lets the rig choose — the microphone for a PTT press on the microphone, the USB port for a CAT or RTS key-down.

    "Leave as set on the radio" sends no TI at all, for an operator who sets this at the front panel.
settings-radio-which-two-sockets-back-receiver =
    Quale delle due prese sul retro ascolta il ricevitore — il comando AN dell'apparato, che è il menu 31 "ANTENNAS" sul pannello frontale e l'indicatore "ANT 1 2" sul suo display.

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    Applies immediately, and is read back from the radio when the port opens: this is the rig's own setting rather than a copy kept here.
settings-radio-one-profile-covers-both-there =
    Un solo profilo copre entrambi. Qui non c'è nulla da scegliere — la radio si identifica da sola all'apertura della porta.

    This is the CAT half only: it drives the dial, the mode, PTT, the S-meter, the SWR and the transmit power over the rig's CAT USB port, and takes audio from its USB Audio port. The FDM-DUO's third USB port carries wideband I/Q, which this interface does not read — select the ELAD interface above for that.

    Note the baud rate: menu 70 "CAT BAUD" on the radio must match, and it ships at 38400.

    CW is keyed by the radio's own key or paddle. The FDM-DUO has no command that accepts text, so the CW panel cannot key it over CAT.
settings-radio-one-profile-covers-range-qrp =
    Un solo profilo copre la gamma: il set di comandi proprio di QRP Labs, che è un sottoinsieme di quello del Kenwood TS-480 con parecchie aggiunte. Qui non c'è nulla da scegliere — la radio si identifica con il proprio nome e la versione del firmware all'apertura della porta, e la versione decide se viene richiesta affatto la lettura della protezione ROS.

    Not the Kenwood profile: on a QMX the PC command is the power METER rather than the power control, so a radio driven as a Kenwood would have its meter read written to as if it were a setting, and MD8 — a Kenwood mode — is SWR Tune here.

    The transmit power is set at the radio. There is no CAT command for it, so the Drive slider only reaches the level of the audio going into the sound card.

    The receive filter is the radio's too: it reports the width its mode implies (3.2 kHz in Digi, 300 Hz in CW) and offers nothing to change it with.
settings-radio-baud-rate-ignored-port-usb = La velocità di baud viene ignorata — la porta è USB
settings-radio-qmx-serves-own-virtual-com =
    Un QMX offre le proprie porte COM virtuali via USB, quindi la velocità impostata qui non ha effetto su nessuna delle due estremità. Ne offre fino a tre; se la radio non risponde nulla, la causa più probabile è quella sbagliata.

    One thing does matter about the port: a carriage return sent to it switches the radio into terminal mode for the rest of the session. sdroxide never sends one — but a terminal program left open on the same port will, and CAT stops working the moment it does.
settings-radio-i-q-mode-switched-radio = La modalità I/Q è attivata sulla radio
settings-radio-radio-s-sound-card-carries =
    La scheda audio della radio trasporta o l'audio demodulato o l'I/Q grezzo che vede il suo ADC, e sdroxide imposta quello che chiede il Formato audio qui sopra all'apertura della porta (il comando Q9 della radio, la voce di menu "IQ mode").

    The centre offset and sample rate above have been set to what a QMX needs: its receiver is a superhet with a 12 kHz I.F., so the synthesiser sits 12 kHz below the dial and everything on the card is 12 kHz above the middle of the span. The card runs at 48 kHz, which makes the panadapter 48 kHz wide.

    In CW the radio moves that I.F. by a further ~700 Hz, which one figure cannot follow — add it by hand if you run the panadapter with the radio in CW. In Digi, where a radio used as an I/Q front end normally sits, there is nothing to add.

    QRP Labs note that I/Q mode is not suitable for WSJT-X and other programs that expect demodulated audio; here the demodulation happens on this side, so it is.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ (ricetrasmettitore HF da 5 W)
settings-radio-hobbypcb-s-rs-hfiq-there =
    L'RS-HFIQ di HobbyPCB. Qui non c'è nulla da scegliere: il profilo è l'intero set di comandi della radio, e la versione del firmware viene registrata all'apertura della porta.

    The control link carries two things and no more — where to put the oscillator, and whether to transmit. There is no mode, power, filter, squelch or meter command, because the radio has none of them. Everything else is sdroxide's: what comes off the sound card is complex baseband centred on the dial, so the mode, the filter and the modulation all happen on this side.

    The frequency command covers 3–30 MHz. Ask for anything outside that and the radio refuses it and stays where it is; the log says so.

    CW is keyed as audio through the transmit chain — this radio has no keyer that takes text. (It has an internal CW generator; sdroxide never uses it, on the firmware's own advice.)
settings-radio-sound-format-baud-ptt-are = Formato audio, velocità di baud e PTT sono della radio, non impostazioni
settings-radio-all-three-have-been-filled =
    Tutti e tre sono stati compilati e nessuno di essi è una preferenza:

    • Sound format is I/Q (stereo) — the card carries complex baseband and nothing else.
    • 57600 baud, 8N1 — the firmware's one rate, with no menu to change it. Any other is a silent link, so it is pinned when the port opens.
    • PTT over CAT — the *X command is the only transmit switch the interface has.

    The centre offset has been set to zero: the synthesiser runs at four times the dial into a quadrature detector, so the middle of the span is the dial. Set the sample rate above to whatever your sound card is actually running at; that is what makes the panadapter as wide as it is.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    Il ricetrasmettitore QRP tascabile di DL2MAN/PE1NNZ e il firmware aperto uSDX da cui è nato. Emula un Kenwood TS-480, ma il sottoinsieme è scarno: risponde solo a sintonia, modo, PTT e azzeramento RIT/XIT. Nessun S-meter, nessun ROS, nessun controllo di potenza, nessun VFO B, nessuno split e nessun manipolatore CAT, quindi il cursore Pilotaggio e i misuratori non fanno nulla via CAT — il livello audio è l'unico controllo di trasmissione che esiste.

    CW is keyed as audio (MCW) through the sideband, or with the key or paddle at the radio; there is no command that takes text.
settings-radio-audio = Audio
settings-radio-tr-usdx-has-no-sound =
    Un (tr)uSDX non ha una scheda audio propria e due modi per essere ascoltato, e quale sia quello giusto dipende dalla tua stazione più che dalla radio.

    "One cable" takes the audio inside the CAT serial link itself — the radio's own 8-bit stream, over the same USB cable as the control, with nothing else to plug in. The catch is the firmware: it cannot take a CAT command while its stream is running (a command sent into the stream does not pause it, it kills it), so this mode never polls. Tuning and mode changes are sent as you make them, each pausing the stream for a moment and starting it again, so expect a brief gap on each change; nothing is sent while you are simply listening. The radio's own dial and mode are not followed either — there is no read to follow them with — so drive it from here.

    "USB sound card" takes the audio from an external sound card wired to the radio's 3.5 mm speaker/mic jack, and behaves as any other CAT rig: the dial poll runs, so the radio's own knob and mode are followed. Pick the card under Radio audio below.

    Both modes open the port with DTR held high: the serial adapter's DTR is wired to the radio's reset, so opening the port reboots it and the first second or two is quiet while it comes up. DTR is never used to key.
settings-radio-radio-model = Modello della radio
settings-radio-which-icom-two-things-ci =
    Quale Icom, per le due cose che il CI-V non fa allo stesso modo su tutti.

    The transceiver address below is filled in from it: every model ships with a different one, and a frame sent to the wrong address is simply ignored — a radio that answers nothing, with no error anywhere to say why.

    It also decides whether sdroxide can select DATA mode. On CI-V, USB and USB-DATA are the same mode byte and a separate command tells them apart; without it a digital-mode over goes out through the microphone input, with the rig's speech processing and SSB filter in the path.

    "Other" leaves the address to you and sends no DATA-mode command at all.
settings-radio-radio-id-hex = ID radio (esadecimale)
settings-radio-show-radio-s-spectrum-scope = Mostra lo scope spettrale della radio
settings-radio-scope-span = Ampiezza dello scope
settings-radio-how-wide-sweep-radio-keeps = Quanto ampia è la scansione. La radio mantiene l'ampiezza scelta per ultima sul proprio schermo — spesso qualche kHz. Impostare qui un'ampiezza mette anche lo scope in modalità centro, quindi segue la sintonia. Cambia anche la visualizzazione della radio stessa; "Come impostato sulla radio" la lascia com'è.
settings-radio-press-apply-reconnect-switch-without = Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-radio-audio-sound-card = Audio della radio (scheda audio)
settings-radio-devices = Dispositivi
settings-radio-discover = Cerca
settings-radio-no-devices-press-discover = nessun dispositivo — premi Cerca
settings-radio-manual-ip = IP manuale
settings-radio-optional-e-g-192-168 = facoltativo, ad es. 192.168.1.50
settings-radio-sample-rate = Frequenza di campionamento
settings-radio-receiver-ddc = Ricevitore (DDC)
settings-radio-protocol-2-board-carries-several = Una scheda Protocollo 2 porta più ricevitori sintonizzabili in modo indipendente (DDC) su un'unica connessione — fai girare questa radio su DDC1 e un'altra radio, stesso indirizzo, su DDC2. Il trasmettitore appartiene alla radio su DDC1. Le schede Protocollo 1 hanno solo DDC1.
settings-radio-lna-gain = Guadagno LNA
settings-radio-front-end-gain-hermes-lite = Guadagno del front-end di un Hermes-Lite 2. Ha effetto subito — senza riconnettere — e viene ricordato come il livello da cui la radio parte. Troppo alto satura l'ADC e tutta la banda appare distorta; troppo basso e il ricevitore diventa sordo.
settings-radio-overload-protection = Protezione da sovraccarico
settings-radio-back-lna-gain-off-itself =
    Riduce da sé il guadagno LNA mentre la scheda segnala che il suo ADC va in overflow, e lo rialza quando smette.

    A Hermes-Lite 2 samples the whole of 0-38 MHz onto one 12-bit converter with no mixer and no preselector in front of it, so a broadcast station a band away can drive it into overflow while the band you are looking at shows nothing wrong at all — the noise floor climbs, everything intermodulates and the decoders stop. The board knows, and this is what acts on it.

    Off by default: it moves a control you set. Nothing happens while transmitting.
settings-radio-wind-gain-back-when-adc = Riduci il guadagno quando l'ADC va in overflow
settings-radio-applies-apply-reconnect-main-window = Ha effetto con Applica / riconnetti. La barra del guadagno della finestra principale mostra il guadagno che la scheda sta davvero usando, quindi segue il ciclo e puoi osservarne l'effetto; il cursore qui sopra è il livello da cui la radio parte, e il ciclo non lo riscrive.
settings-radio-how-far-gain-moves-each = Di quanto si sposta il guadagno ogni volta, in dB. Uno è il passo del registro di guadagno proprio della scheda.
settings-radio-attack-decay = {"  "}Attacco / rilascio
settings-radio-how-often-gain-may-come =
    Ogni quanto il guadagno può scendere mentre il convertitore è in overflow, e ogni quanto può risalire una volta che è smesso.

    The two are deliberately a hundred times apart. Retreat immediately: every millisecond of overflow is a receiver full of intermodulation. Return slowly: whatever caused it — a neighbour keying, a broadcaster coming up at dusk — has usually not gone away, and a loop that recovered as fast as it retreated would spend the evening oscillating across the threshold. 100 ms and 10 s per decibel is what PowerSDR's Auto S-Att and N1GP's HermesIntf have both used.
settings-radio-range = {"  "}Intervallo
settings-radio-lowest-highest-gain-loop-may = Il guadagno più basso e più alto che il ciclo può usare, in dB. Il tetto serve a impedire che decida quanto debba essere sensibile il tuo ricevitore; il minimo è dove dichiari che, sotto un certo punto, il sovraccarico è un problema di qualcun altro e la risposta è un filtro, non altri venti decibel.
settings-radio-filter-board = Scheda filtri
settings-radio-accessory-board-hermes-lite-2 = Scheda accessoria sul connettore J16 dell'Hermes-Lite 2 (o sulle uscite open-collector di qualunque altra scheda openHPSDR — anche Protocollo 2). "N2ADR" sceglie un relè per banda; "Codice banda Alex / Hermes" mette il numero della banda sulle uscite 1-4, che è ciò che si aspettano la scheda Alex di un ANAN, uno Zeus SDR, un HiQSDR e Quisk. "Personalizzata" apre una tabella qui sotto in cui indichi tu le sette uscite, banda per banda, in ricezione e in trasmissione — per un commutatore d'antenna, il decodificatore di banda di un amplificatore o qualsiasi altra cosa che nessuno dei due preset copra; uno dei due preset può essere copiato in essa come punto di partenza. Lascia questo su "Nessuna" a meno che non sia davvero montato qualcosa: quei sette pin sono uscite open-collector di uso generale, e gli operatori le collegano anche al PTT di amplificatori, ai relè d'antenna e alla commutazione dei transverter. Pilotarle con i dati di banda avvierebbe qualunque cosa sia collegata. Ha effetto con Applica / riconnetti.
settings-radio-io-board-rx-input = Ingresso RX della scheda IO
settings-radio-where-n2adr-hl2ioboard-takes-receive = Da dove una scheda N2ADR HL2IOBoard prende il segnale di ricezione. La scheda stessa viene trovata automaticamente e non richiede impostazioni; questa esiste solo per chi ha cablato le sue prese SMA. Altrimenti lascia su "Ingresso proprio della radio" — selezionare il J9 della scheda IO senza nulla collegato lascia il ricevitore sordo. Ha effetto con Applica / riconnetti. Impostato su una delle due scelte J9, gli ingressi compaiono anche sul comando ANT e vengono ricordati per banda.
settings-radio-power-amplifier = Amplificatore di potenza
settings-radio-use-hermes-lite-2-s = Usa il PA di bordo dell'Hermes-Lite 2
settings-radio-default-what-you-want-unless = Attivo di default, ed è ciò che vuoi a meno che un amplificatore esterno non venga pilotato dall'uscita RF1 a bassa potenza della scheda. Disattivato, la radio si attiva comunque — il relè T/R scatta e ogni scheda accessoria lo segue — ma la presa d'antenna non produce alcuna potenza, e il relè è volutamente tenuto in ricezione. Ignorato sulle schede diverse da un Hermes-Lite.
settings-radio-mirror-board-s-spectrum-about = Specchia lo spettro della scheda rispetto alla sintonia, in trasmissione così come in ricezione. Attivo di default: un Hermes-Lite 2 ne ha bisogno. Disattivalo solo se i segnali compaiono dalla parte sbagliata della sintonia e nulla viene decodificato — il rivelatore è una waterfall piena di tracce convincenti mentre la SSB cade sulla banda laterale sbagliata e FT8 non restituisce alcuna decodifica.
settings-radio-frequency-correction = Correzione della frequenza
settings-radio-crystal-tcxo-error-ppm-applied = Errore del cristallo/TCXO in ppm, applicato a RX e TX. Ha effetto subito.
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = Buffer di trasmissione
settings-radio-how-far-ahead-real-time = Quanto in anticipo rispetto al tempo reale l'audio di trasmissione viene inviato alla scheda via rete, prima che sdroxide rallenti per adeguarsi. La scheda stessa non conserva alcun buffer di questo tipo — questo amplia soltanto il margine proprio di sdroxide. Alzalo su un collegamento WiFi o una VPN, dove il valore predefinito basso (giusto per una connessione cablata diretta) non dà margine sufficiente contro il jitter e l'audio trasmesso o il PTT si spezzano; un valore più alto costa latenza in trasmissione. Ha effetto con APPLICA, che ricollega la scheda.
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = Predistorsione adattiva: linearizza il trasmettitore a partire da un campione di ciò che ha effettivamente emesso, per una ventina di decibel in meno di intermodulazione alla stessa potenza. Il ricevitore è il percorso di retroazione — la scheda continua a ricevere durante una trasmissione — quindi un accoppiatore direzionale e un attenuatore devono portare un campione dell'uscita dell'amplificatore su un ingresso che il commutatore T/R non toglie in trasmissione. Su un Hermes-Lite 2 vuol dire la presa PureSignal della scheda IO, con l'ingresso di ricezione qui sopra impostato di conseguenza; su una scheda che non ne ha, la porta RX di un ANAN, oppure un relè esterno che inserisce l'accoppiatore per la durata della trasmissione. È questo ricevitore che viene letto — una scheda con un secondo ADC non può usare RX2 come presa di retroazione, comunque il suo firmware lo instradi. Senza nulla accoppiato, il ciclo non si aggancia mai e il trasmettitore resta esattamente come sarebbe stato. Ha effetto con Applica / riconnetti, e solo sulla radio che possiede il trasmettitore (DDC1).
settings-radio-correct-transmitter-receiver-s-own = Correggi il trasmettitore dalla retroazione del ricevitore stesso
settings-radio-how-finely-amplifier-s-curve = Quanto finemente viene modellata la curva dell'amplificatore. Più passi seguono un ginocchio più netto e impiegano più tempo a riempirsi; 32 è un buon punto di partenza.
settings-radio-how-fast-correction-follows-what = Quanto velocemente la correzione segue ciò che riporta l'accoppiatore. Lento è giusto — si tratta di mediare la curva di un amplificatore, che non cambia, a partire da un percorso di retroazione che ha del rumore.
settings-radio-hold = Blocca
settings-radio-stop-adapting-keep-correction-where = Smetti di adattarti e mantieni la correzione com'è — per misurare, e per un operatore soddisfatto di ciò che ha appreso.
settings-radio-manual-ip-overrides-discovery-press = Un IP manuale prevale sulla ricerca. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-open-collector-outputs-band = Uscite open-collector per banda
settings-radio-one-control-word-per-band = Una parola di controllo per banda, come la documentazione del tuo hardware la descrive: il bit 0 è l'uscita 1, il bit 6 l'uscita 7. RX è attiva durante la ricezione su quella banda e TX mentre il trasmettitore è attivato — dai lo stesso valore a entrambi per un filtro, e valori diversi per tutto ciò che appartiene a un solo lato della commutazione (la linea di comando di un amplificatore, il bypass di un preamplificatore di ricezione). Le bande lasciate a 00 non attivano nulla. Ha effetto con Applica / riconnetti.
settings-radio-alex-band-code = CODICE BANDA ALEX
settings-radio-replace-table-what-preset-would = Sostituisce la tabella con ciò che questo preset invierebbe su ogni banda, come punto di partenza da modificare. Nient'altro nella configurazione cambia.
settings-radio-clear = CANCELLA
settings-radio-every-output-off-every-band = Ogni uscita spenta su ogni banda, che è ciò che fa "Nessuna".
settings-radio-band = Banda
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = Uscite attive
settings-radio-other = Altro
settings-radio-everywhere-outside-amateur-bands-short = Ovunque fuori dalle bande radioamatoriali — ascolto in onde corte, e tutto ciò su cui cade la sintonia di un transverter che nessuna banda copre.
settings-radio-dongle = Chiavetta
settings-radio-rescan = Riesamina
settings-radio-re-list-usb-bus-no = Rielenca il bus USB. Nessun dispositivo viene aperto, quindi è sicuro premerlo durante la ricezione.
settings-radio-no-dongles-press-rescan = nessuna chiavetta — premi Riesamina
settings-radio-first-one-found = — la prima trovata —
settings-radio-rtl2832u-s-resampler-reaches-225 = Il ricampionatore dell'RTL2832U arriva a 225–300 kHz e da 900 kHz a 3,2 MHz, niente di mezzo. Ha effetto con Applica.
settings-radio-often-drops-samples = {"  "}(spesso perde campioni)
settings-radio-agc = AGC
settings-radio-manual-setting-measurement-weak-signal = Manuale è l'impostazione per le misure e per i modi digitali a segnale debole. Il sintonizzatore e il demodulatore hanno anelli automatici indipendenti.
settings-radio-tuner-gain = Guadagno del sintonizzatore
settings-radio-applies-immediately-no-reconnect-tuner = Ha effetto subito — senza riconnettere. Il sintonizzatore ha 29 passi discreti, quindi il valore si aggancia al più vicino che può produrre. Ignorato mentre l'AGC del sintonizzatore è attivo.
settings-radio-crystal-error-parts-per-million = Errore del cristallo in parti per milione. Avvia con RUST_LOG=sdroxide_rtlsdr=debug e il log stampa l'errore di clock misurato dopo circa 20 secondi — è il numero da inserire. Ha effetto subito.
settings-radio-hf-reception = Ricezione HF
settings-radio-tuner-itself-starts-24-mhz =
    Il sintonizzatore stesso parte da 24 MHz. Un RTL-SDR Blog V4 converte verso l'alto sotto quella frequenza via hardware; le altre chiavette raggiungono le HF solo campionando direttamente l'ADC, attraverso la porta HF del V3. Automatico affida all'ADC tutto ciò che è sotto i 24 MHz e al sintonizzatore tutto ciò che è sopra.

    Direct sampling covers every HF band, 17 m and 15 m included — they arrive in the ADC's second Nyquist zone, the right way up. Nothing filters the ADC's input, though, so whatever is at 28.8 MHz minus the dial comes with them: 10.7 MHz under 17 m, 7.726 MHz under 15 m.

    Switching modes briefly interrupts the stream.
settings-radio-removes-dongle-s-own-dc =
    Elimina il picco DC proprio della chiavetta dal centro dell'ampiezza, e l'immagine speculare che ogni segnale lascia riflessa rispetto ad esso, misurando lo squilibrio nei campioni stessi — nessuna calibrazione, e ha effetto subito. Il sintonizzatore non ha un modo di sintonia a offset, quindi questo è l'unico modo per liberare il centro.

    An AM carrier tuned dead on the dial is at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-remove-centre-spike-mirror-image = Rimuovi il picco centrale e l'immagine speculare
settings-radio-bias-tee = Bias tee
settings-radio-feed-4-5-v-dc = Invia circa 4,5 V DC lungo il cavo coassiale
settings-radio-bias-tee-never-connect-transceiver = Il bias tee è ATTIVO. Non collegare mai un ricetrasmettitore, un'antenna a massa o un preamplificatore alimentato altrove mentre questo è attivo — la corrente continua scende direttamente lungo la linea di alimentazione.
settings-radio-receive-only-dongle-sample-rate = Solo ricezione. La chiavetta e la frequenza di campionamento hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi.
settings-radio-server-address = Indirizzo del server
settings-radio-where-rtl-tcp-listening-address =
    Dove è in ascolto rtl_tcp: un indirizzo, oppure un indirizzo e una porta. La porta predefinita è 1234, che è quella di rtl_tcp.

    On the far end, start it as `rtl_tcp -a 0.0.0.0` — bound to 127.0.0.1, which is what it does with no -a, it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g = host o host:porta, ad es. raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    Richiesto al server, con gli stessi limiti del ricampionatore di una chiavetta locale — è lo stesso silicio dall'altro capo.

    The figure beside each rate is what it costs on the link: the samples are sent uncompressed, and a rate the network cannot carry makes rtl_tcp drop the connection rather than degrade. Takes effect on Apply.
settings-radio-wired-link = {"  "}(collegamento cablato)
settings-radio-runs-server-s-dongle-manual = Gira sulla chiavetta del server. Manuale è l'impostazione per le misure e per i modi digitali a segnale debole.
settings-radio-applies-immediately-no-reconnect-sent = Ha effetto subito — senza riconnettere. Inviato in decimi di dB e agganciato dal server a un passo che il suo sintonizzatore possiede; il protocollo non prevede risposte, quindi non si può leggere su cosa si è stabilizzato, e questo cursore continua a mostrare ciò che è stato richiesto. Ignorato mentre l'AGC del sintonizzatore è attivo.
settings-radio-crystal-error-server-s-dongle =
    Errore del cristallo della chiavetta del *server*, in parti per milione — una caratteristica di quell'hardware, quindi si imposta qui e non sulle chiavette di questa macchina. Ha effetto subito.

    The measured clock error the USB interface prints is not available here: over a network what that measurement sees is the buffering, not the crystal, and it is wrong by thousands of ppm. Calibrate the dongle on USB once and carry the number over, or tune a broadcast station of known frequency and adjust until it sits on the dial.
settings-radio-tuner-starts-24-mhz-below =
    Il sintonizzatore parte da 24 MHz; sotto quella frequenza il capo remoto ha bisogno di aiuto. Un Blog V4 converte verso l'alto da solo e Automatico non lo tocca — che è l'unica cosa che può fare, perché il protocollo riporta il chip del sintonizzatore e nient'altro, e un V4 è indistinguibile da un R828D normale sul filo.

    On a V3 or any other dongle, Automatic switches the server to direct sampling below the tuner's own 24 MHz floor — which covers every HF band, 17 m and 15 m arriving in the ADC's second Nyquist zone with 28.8 MHz minus the dial folded on top. Choose Direct sampling explicitly for a plain R828D that hears nothing on HF. Switching briefly interrupts the stream.
settings-radio-removes-dongle-s-dc-spike =
    Elimina qui il picco DC della chiavetta e la sua immagine speculare, dai campioni man mano che arrivano — sono artefatti dell'hardware, quindi viaggiano in rete con tutto il resto. Ha effetto subito.

    An AM carrier tuned dead on the dial sits at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-powers-preamp-far-end-s = Alimenta un preamplificatore dalla chiavetta del capo remoto. I server più vecchi non implementano il comando e lo ignorano in silenzio — il protocollo non ha modo di dire di no — quindi un bias tee che non si accende non è necessariamente colpa di questo capo.
settings-radio-feed-4-5-v-dc-2 = Invia circa 4,5 V DC lungo il cavo coassiale remoto
settings-radio-bias-tee-hardware-somewhere-else = Il bias tee è ATTIVO, su un hardware che si trova altrove. Ciò che sta sull'altro capo di quella linea di alimentazione — un ricetrasmettitore, un'antenna a massa, un preamplificatore già alimentato — non è davanti a te per controllarlo.
settings-radio-sdrplay-server-rsp-tcp = Server SDRplay (rsp_tcp)
settings-radio-sdrplay-server-greets-exactly-like = Un server SDRplay si presenta esattamente come una chiavetta, quindi questi campi sono mostrati sempre anziché solo quando ne viene rilevato uno. Con un normale server rtl_tcp vengono semplicemente ignorati — il protocollo non ha risposte e scarta i comandi che non conosce. La scheda Dispositivo indica il server come rsp_tcp non appena si è identificato.
settings-radio-input = Ingresso A
settings-radio-input-b = Ingresso B
settings-radio-hi-z = Hi-Z
settings-radio-lna-state = Stato LNA
settings-radio-step-index-not-db-figure = Un indice di passo, non un valore in dB: quanto vale ogni passo dipende dal modello RSP e dalla banda. 0 è il guadagno massimo.
settings-radio-if-gain-reduction = Riduzione del guadagno IF
settings-radio-reduction-so-more-less-signal = Una riduzione, quindi di più significa meno segnale. Obbedita solo con l'AGC dell'RSP disattivato.
settings-radio-enable = Attiva
settings-radio-level-rsp-s-agc-aims = Il livello che l'AGC dell'RSP punta a mantenere.
settings-radio-notches = Notch
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = Uscita di riferimento
settings-radio-t-24-mhz-clock-out = Uscita di clock a 24 MHz
settings-radio-rsp2-rspduo-only = Solo RSP2 e RSPduo.
settings-radio-receive-only-address-sample-rate = Solo ricezione. L'indirizzo e la frequenza di campionamento hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi. Una connessione caduta viene ritentata da sola, quindi un server riavviato torna senza toccare nulla qui.
settings-radio-where-spyserver-listening-address-address =
    Dove è in ascolto spyserver: un indirizzo, oppure un indirizzo e una porta. La porta predefinita è 5555, che è quella di spyserver.

    On the far end, check that its config file binds an address other machines can reach — bound to 127.0.0.1 it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-2 = host o host:porta, ad es. raspberrypi.local:5555
settings-radio-i-q-bandwidth = Larghezza di banda I/Q
settings-radio-how-wide-slice-band-arrives =
    Quanto è larga la porzione di banda che arriva come I/Q — e quindi quanto del collegamento usa. Questa è la finestra *demodulata*: segue la sintonia, e tutto ciò che è più largo è la FFT del server nella striscia sopra il panadapter.

    Automatic aims at about 96 kHz, which carries every mode here including wide FM and still fits down a cellular uplink. Takes effect on Apply.
settings-radio-which-stage-server-s-own =
    Quale stadio della scala di frequenze del server chiedere. Ogni ricevitore ha una scala diversa — è la sua frequenza massima dimezzata stadio per stadio — quindi questo viene memorizzato come stadio anziché come cifra in hertz, e la stessa impostazione continua ad avere senso puntata su un altro server.

    Automatic aims at about 1 Msps. Press Test connection to see what the stages come to on this server. Takes effect on Apply.
settings-radio-automatic = Automatico
settings-radio-sample-format = Formato dei campioni
settings-radio-bits-per-component-wire-so =
    Bit per componente sul filo, e quindi quanto costa sul collegamento: 16 bit sono il doppio di 8 bit, e il float a 32 bit è quattro volte tanto per non più informazione di quella che aveva l'ADC del ricevitore.

    8-bit is what makes a remote receiver work over a domestic uplink and is right for almost everything. A server may override this — some are configured to insist on one format — and says so in the log when it does. Takes effect on Apply.
settings-radio-gain = Guadagno
settings-radio-server-s-gain-stage-index =
    Lo stadio di guadagno del server, come indice — non come numero di decibel. Ciò che ogni indice significa appartiene al ricevitore dall'altro capo e cambia con la banda, e il protocollo non lo dice mai, quindi nulla qui può convertirlo in dB senza inventare una cifra.

    The real range is the server's and is only known once connected; an index past it is clamped. Applies immediately — no reconnect.
settings-radio-index = indice{" "}
settings-radio-digital-gain = Guadagno digitale
settings-radio-how-far-server-scales-samples =
    Di quanto il server scala i propri campioni verso l'alto prima di quantizzarli per il filo. Automatico osserva ciò che arriva davvero e mantiene il picco a metà del fondo scala, che è l'unica cosa che può andare bene sia su una banda morta sia su una affollata — le due distano trenta decibel, e nessuna cifra fissa serve a entrambe.

    It matters only at 8 bits, where a signal far below full scale loses its lower bits to the quantiser and one above it is clipped flat. Wider formats have room to spare and get the figure every other client computes, from the gain index and the decimation stage. Applies immediately.
settings-radio-full-band-strip = Striscia a banda piena
settings-radio-server-s-own-fft-whole =
    La FFT propria del server su tutta la banda, disegnata nella striscia sopra il panadapter. In questa interfaccia è l'unica vista di banda che esiste — il panadapter stesso è largo solo quanto l'I/Q ricevuto.

    Switching it off leaves a receiver with no way to see anything it is not already tuned to. Applies immediately, with a short break in the audio while the server changes what it is sending.
settings-radio-ask-server-low-rate-fft =
    Chiedi al server anche una FFT a bassa frequenza di tutta la banda oltre all'I/Q, e disegnala nella striscia sopra il panadapter.

    It costs almost nothing — a couple of kilobytes a frame, a dozen or so times a second — and it shows the whole receiver rather than the slice being demodulated. Worth switching off only on a link where every byte counts. Applies immediately, with a short break in the audio.
settings-radio-show = Mostra
settings-radio-whole-band = intera banda
settings-radio-how-much-receiver-strip-covers = Quanto del ricevitore copre la striscia. L'intera banda è la vista più larga che esiste; restringerla mette lo stesso numero di bin su meno spettro, cioè un dettaglio più fine su un tratto più piccolo.
settings-radio-strip-db-window = Finestra dB della striscia
settings-radio-server-quantises-fft-into-window =
    Il server quantizza la sua FFT in questa finestra prima di inviarla, un byte per bin — quindi queste decidono quanto finemente viene misurata la striscia, non solo come viene disegnata. Il pavimento e il soffitto con cui la striscia viene *visualizzata* sono impostati dal livellamento automatico del motore e sono un'altra cosa.

    The default 150 dB is the whole protocol range and needs no attention. Narrowing it around the noise floor buys resolution on a receiver whose signals all sit in a small part of the scale. Applies immediately.
settings-radio-top = alto{" "}
settings-radio-range-2 = intervallo{" "}
settings-radio-i-q-correction = Correzione I/Q
settings-radio-remove-dc-spike-mirror-image = Rimuovi il picco DC e l'immagine speculare nel DSP, da questo lato. Se il ricevitore dall'altro capo ne abbia bisogno dipende da cosa sia — un Airspy HF+ no, un RTL-SDR sì — e il protocollo non dice con quale stia parlando. Ha effetto subito.
settings-radio-enabled = Attivo
settings-radio-test-connection = Prova connessione
settings-radio-connect-read-what-server-says = Collegati, leggi cosa dice il server sul suo ricevitore, e disconnettiti di nuovo — senza avviare un flusso, quindi è sicuro da premere su un server che qualcun altro sta usando.
settings-radio-receive-only-panadapter-narrow-i = Solo ricezione. Il panadapter è la finestra I/Q stretta, che segue la sintonia; la vista di banda è la FFT del server nella striscia sopra di esso. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-receive-only-wideband-i-q = Solo ricezione. I/Q a banda larga, come ogni SDR locale. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-receiver-address = Indirizzo del ricevitore
settings-radio-where-receiver-serves-web-page =
    Dove il ricevitore serve la sua pagina web: un indirizzo, oppure un indirizzo e una porta. La porta predefinita è 8073, quella di un KiwiSDR.

    Note that a receiver reached through the project's proxy (something.proxy.kiwisdr.com — nearly half the public ones) answers on port 80 instead, so give it explicitly. Picking a receiver from "Public SDRs" fills this in correctly.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-3 = host o host:porta, ad es. kiwi.example.org:8073
settings-radio-password = Password
settings-radio-receiver-s-user-password-where =
    La password *utente* del ricevitore, quando il suo operatore ne ha impostata una. Vuota su quasi tutti i ricevitori pubblici. Non la password di amministratore.

    Takes effect on Apply.
settings-radio-announce = Annuncia come
settings-radio-name-connection-shows-up-under =
    Il nome con cui compare questa connessione, visibile al proprietario del ricevitore e a tutti gli altri che lo ascoltano.

    Left blank it is your station callsign, which is the network's own convention — some operators do block clients that will not identify. Takes effect on Apply.
settings-radio-blank-station-callsign = vuoto = nominativo di stazione
settings-radio-band-view = Vista di banda
settings-radio-ask-receiver-own-waterfall-well =
    Chiedi al ricevitore la sua waterfall oltre al suo I/Q, e disegnala nella striscia sopra il panadapter.

    Worth having: the I/Q is only about 12 kHz wide, so without this there is nothing to tune *by*. It is a second connection carrying roughly 20 kB/s at full speed, against the I/Q's 44 kB/s.

    Takes effect on Apply.
settings-radio-show-receiver-s-0-30 = mostra la waterfall 0-30 MHz del ricevitore
settings-radio-band-view-speed = Velocità della vista di banda
settings-radio-how-often-receiver-sends-waterfall =
    Ogni quanto il ricevitore invia una riga di waterfall, da 1 (più lento) a 4. È l'unica impostazione qui che cambia il costo del collegamento durante il funzionamento, quindi abbassala su una connessione a consumo misurato.

    Applies immediately.
settings-radio-band-view-span = Ampiezza della vista di banda
settings-radio-how-much-band-receiver-s =
    Quanta banda copre la waterfall del ricevitore. Tutta la sua 0-30 MHz a sinistra del cursore, dimezzandosi a ogni passo verso destra.

    The waterfall is always 1024 bins wide however wide the window is, so this is really a resolution control: the whole band is 29 kHz to a bin, which is a band *map* rather than a picture of anything, while a few hundred kilohertz shows individual stations in a broadcast band. Zoomed in, the window follows your dial.

    The whole band is the default and is what makes the strip a thing to tune by; the panadapter below it is the ~12 kHz of I/Q. Applies immediately.
settings-radio-receiver-agc = AGC del ricevitore
settings-radio-receiver-s-agc-which-far =
    L'AGC del *ricevitore*, che sta dall'altro lato del collegamento e prima dell'I/Q — quindi agisce prima di qualsiasi cosa faccia sdroxide.

    On by default, unlike every local SDR here, because the measured alternative was worse: on a live receiver the manual gain was not monotonic across its range and its top end clipped the I/Q at full scale, while the AGC held about -24 dBFS with 7 dB of headroom.

    It does mean the sample amplitude is not a signal level, which is why the S-meter is read from the receiver's own figure instead. Applies immediately.
settings-radio-let-receiver-ride-own-gain = lascia che il ricevitore gestisca il proprio guadagno
settings-radio-manual-gain = Guadagno manuale
settings-radio-fixed-gain-when-receiver-s = Guadagno fisso quando l'AGC del ricevitore è disattivato, su una scala propria da 0 a 90. Non sono decibel di nulla di dichiarato — il protocollo lo chiama manGain e non dice altro, ed è stato misurato come non monotono, quindi trattalo come una manopola per trovare un buon punto e non come una calibrazione. Ha effetto subito.
settings-radio-read-receiver-s-status-page = Leggi la pagina di stato del ricevitore: cos'è, cosa copre, quanti canali sono liberi, e se il suo operatore consente connessioni da app diverse dal browser. Non occupa nessuno dei suoi canali.
settings-radio-host-port-e-g-127 = host:porta, ad es. 127.0.0.1:50001
settings-radio-iq-sample-rate = Frequenza di campionamento IQ
settings-radio-receiver = Ricevitore
settings-radio-rig-two-receivers-sunsdr2dx-can = Un apparato con due ricevitori (SunSDR2DX) può servire due schede radio da un'unica connessione — fai girare questa radio su RX1 e un'altra su RX2. Il trasmettitore appartiene alla radio su RX1.
settings-radio-stream-delay = Ritardo del flusso
settings-radio-how-long-rig-s-iq = Quanto tempo impiega l'I/Q dell'apparato per arrivare su un nuovo centro dopo che sdroxide lo sposta. È la pipeline DSP dell'apparato, non la rete — anche un server su questa macchina ne ha una — e mentre il panadapter viene trascinato con zoom massimo in fuori, sbagliarsi di questo tanto sposta lateralmente le righe più recenti della waterfall rispetto alla storia, di tanto quanto è l'errore per la velocità del trascinamento. Un valore troppo alto le sposta esattamente quanto uno troppo basso, nel verso opposto. Misuralo con `cargo run --release -p sdroxide-tci --example retune_latency`; il predefinito è un SunSDR2DX su ExpertSDR3 a 192 kHz.
settings-radio-wideband-iq-receive-audio-transmit = I/Q a banda larga in ricezione, audio in trasmissione. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-radio-address = Indirizzo della radio
settings-radio-address-shown-radio-under-set = L'indirizzo mostrato sulla radio sotto SET > Network. Lì deve essere attivo Network Control, e la radio deve avere un nome utente e una password di rete impostati.
settings-radio-host-ip-e-g-192 = host o IP, ad es. 192.168.1.50
settings-radio-control-port = Porta di controllo
settings-radio-t-50001-unless-has-been-changed = 50001 a meno che non sia stata cambiata sulla radio.
settings-radio-network-user = Utente di rete
settings-radio-stored-clear-radio-json-protocol = Salvata in chiaro in radio.json. Il protocollo la offusca in modo reversibile sul filo, quindi nulla qui la renderebbe un segreto.
settings-radio-receive = Ricevi da
settings-radio-audio-sample-rate = Frequenza di campionamento audio
settings-radio-t-12-khz-if-needs-48000 = Una IF a 12 kHz richiede il flusso a 48000 Hz — a questa frequenza viene usato l'audio demodulato della radio.
settings-radio-if-spectrum = Spettro IF
settings-radio-which-way-round-radio-s = In che verso gira la IF a 12 kHz della radio. "Automatico" usa ciò che il modello è noto per fare — specchiata su un IC-7760, normale su ogni altro Icom. Impostala a mano se la SSB esce sulla banda laterale sbagliata: il segnale è dover selezionare USB dove la banda va in LSB, mentre il display di modo della radio concorda con sdroxide per tutto il tempo.
settings-radio-normal = Normale
settings-radio-mirrored = Specchiata
settings-radio-displayed-bandwidth = Larghezza di banda visualizzata
settings-radio-width-audio-band-panadapter-cat = Larghezza del panadapter della banda audio, come per un apparato CAT. Usata nei modi digitali e dove lo scope della radio è spento; altrimenti il panadapter è lo scope, e Ampiezza dello scope ne imposta la larghezza.
settings-radio-how-cw-panel-s-keyer-2 = Come trasmette il manipolatore del pannello CW. "Manipolatore dell'apparato" mette la radio in CW e le consegna il testo da inviare con il proprio manipolatore (CI-V 0x17). "Scheda audio" invia invece il sidetone manipolato sulla LAN come audio (MCW), un tono a sintonia + tono — e poiché un apparato in CW ignorerebbe il suo ingresso modulatore, selezionando CW la radio resta su USB semplice, lo stesso modo su cui viaggiano i modi digitali.
settings-radio-how-much-audio-radio-holds = Quanto audio trattiene la radio prima di modulare. Di più sopravvive a una rete peggiore, al costo della latenza in trasmissione.
settings-radio-streams-radio-s-own-475 = Trasmette lo sweep proprio della radio di 475 bin. Su AF è il panadapter — l'audio che la radio invia è ciò che è passato dal suo filtro, non un'immagine della banda — e sulla IF a 12 kHz è la waterfall a banda piena sopra uno di essi. In entrambi i casi è l'immagine della radio, non il DSP di sdroxide: non c'è I/Q da cui calcolarne una.
settings-radio-how-wide-sweep-only-wide = Quanto ampio è lo sweep. È l'unica vista ampia che un Icom abbia, e la radio mantiene l'ampiezza scelta per ultima sul proprio schermo — spesso qualche kHz, motivo per cui la striscia può comparire non più larga del panadapter sotto di essa. Impostare qui un'ampiezza mette anche lo scope in modalità centro, quindi segue la sintonia. Cambia anche la visualizzazione della radio stessa; "Come impostato sulla radio" la lascia com'è.
settings-radio-switch-modulation-input-lan = Passa l'ingresso di modulazione a LAN
settings-radio-transmit-audio-only-heard-when = L'audio di trasmissione si sente solo quando l'ingresso MOD della radio è impostato su LAN. sdroxide può scriverlo su un modello di cui conosce la numerazione dei menu; su un altro lo dice e lascia il menu com'è. È un prestito: ciò che la radio aveva lì viene letto per primo e rimesso al suo posto alla fine della sessione, così un apparato usato da solo in seguito sente ancora il proprio microfono.
settings-radio-copy-diagnostic-report = Copia il rapporto diagnostico
settings-radio-control-audio-radio-s-own = Controllo, audio e lo scope proprio della radio su un'unica connessione di rete — nessun cavo seriale e nessuna scheda audio. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-testing = Prova in corso…
settings-radio-was-only-check-press-apply = Era solo una verifica — premi Applica / riconnetti qui sotto per iniziare a usare questa radio.
settings-radio-radios = Radio
settings-radio-asks-network-iio-devices-also = Chiede alla rete i dispositivi IIO, e prova anche direttamente 192.168.2.1 — un Pluto alla fine di un cavo USB spesso non è raggiungibile via multicast anche se l'indirizzo funziona.
settings-radio-no-radios-press-discover = nessuna radio — premi Cerca
settings-radio-address = Indirizzo
settings-radio-overrides-selection-above-usb-cable = Prevale sulla selezione qui sopra. Il cavo USB presenta il Pluto come scheda di rete, non come porta seriale, quindi è un indirizzo IP anche quando la radio è sulla tua scrivania.
settings-radio-pluto-revision-c-pluto-unlocked = Un Pluto+ o un Pluto di revisione C sbloccato a 2R2T può servire due schede radio da una sola unità — questa radio su RX1 e un'altra su RX2, ciascuna sulla propria antenna. Le due catene condividono un unico oscillatore, quindi sintonizzare una delle due radio sposta entrambe; ciò che RX2 offre è una seconda antenna sullo stesso spettro (diversità), non una seconda banda. Il trasmettitore appartiene alla radio su RX1. Un Pluto 1R1T di serie rifiuta RX2 al collegamento.
settings-radio-width-spectrum-sdroxide-receives-ad9361 = Ampiezza dello spettro che sdroxide riceve. L'AD9361 arriva a 61,44 Msps; il collegamento di rete USB no, ed è a questo che è scalato l'elenco. Le frequenze più basse richiedono una configurazione del filtro caricata nell'AD9361, cosa che sdroxide non fa — un Pluto di serie le fa funzionare a circa 2,084 Msps e lo dice al collegamento. Ha effetto con Applica.
settings-radio-more-than-usb-2-will = {"  "}(più di quanto il USB 2 possa portare)
settings-radio-stock-pluto-runs-2-084 = {"  "}(un Pluto di serie gira a 2,084)
settings-radio-analog-filter = Filtro analogico
settings-radio-ad9361-s-baseband-filter-leave = Il filtro di banda base dell'AD9361. Lascia a 0 per automatico, che lo apre fino a nove decimi della frequenza di campionamento — largo di proposito, perché il ricevitore sposta il proprio oscillatore di un quarto di ampiezza lontano dalla sintonia per tenere i segnali lontani dal picco DC, e un filtro stretto taglierebbe proprio la parte in cui li ha spostati. Ha effetto con Applica.
settings-radio-ad9361-has-four-modes-not = L'AD9361 ha quattro modi, non un interruttore on/off. Attacco lento va bene per SSB e CW; attacco veloce per i segnali a raffica; manuale è l'impostazione per le misure e per i modi digitali a segnale debole. Ha effetto subito.
settings-radio-rx-gain = Guadagno RX
settings-radio-applies-immediately-no-reconnect-ignored = Ha effetto subito — senza riconnettere. Ignorato a meno che l'AGC non sia impostato su manuale, che è il comportamento dell'AD9361 stesso, non di sdroxide.
settings-radio-tx-gain = Guadagno TX
settings-radio-negative-because-ad9361-expresses-transmit = Negativo perché l'AD9361 esprime il livello di trasmissione come attenuazione: 0 dB è l'uscita piena. Ha effetto subito. Il trasmettitore viene portato al suo livello più basso prima che questo valore venga applicato alla connessione, così nulla lasciato dal programma precedente può essere attivo.
settings-radio-reference-error-parts-per-million = Errore di riferimento in parti per milione. Avvia con RUST_LOG=sdroxide_pluto=debug e il log stampa l'errore di clock misurato dopo circa 20 secondi — è il numero da inserire. Applicato da sdroxide, non scritto nella correzione persistente della radio. Ha effetto subito.
settings-radio-full-duplex = Full duplex
settings-radio-keep-receiving-while-transmitting = Continua a ricevere durante la trasmissione
settings-radio-leave-off-pluto-reached-over =
    Lascialo disattivato su un Pluto raggiunto tramite il suo cavo USB. Quel collegamento non può portare un megasample al secondo in entrambe le direzioni contemporaneamente, e una trasmissione che affama il buffer di trasmissione esce a scatti — quindi per impostazione predefinita la ricezione si ferma per la durata di una trasmissione.

    Turn it on for a board on real Ethernet (a LibreSDR, a Pluto on a gigabit adapter), where there is room for both: you then hear the receiver through your own transmission, which is how a QO-100 station listens to its own downlink.

    An over needs twice the sample rate's worth of link — 2.5 Msps is 10 MB/s each way — so lower the rate if the log starts saying the link is not carrying it. The panadapter still shows the transmitted signal during an over; it is the audio that keeps coming.

    Takes effect on Apply.
settings-radio-duplex = Duplex
settings-radio-whether-ad9361-runs-both-directions =
    Se l'AD9361 funziona in entrambe le direzioni contemporaneamente (FDD, che è come un Pluto si avvia e ciò in cui sdroxide lo ha sempre lasciato) o una alla volta (TDD).

    Leave this on FDD unless you want the PTT pins below — TDD is what those key from, and it rules out Full duplex above. Takes effect on Apply.
settings-radio-ptt-pins = Pin PTT
settings-radio-pluto-s-four-gpo-test =
    I quattro punti di test GPO del Pluto possono comandare da soli un amplificatore di potenza esterno, un LNA o un commutatore trasmissione-ricezione. Scegli una coppia e un pin è alto per tutto il tempo in cui la radio riceve, l'altro per tutto il tempo in cui trasmette — nessun software sul computer nel mezzo e nessuna linea PTT seriale da cablare.

    This puts the radio in TDD whatever the Duplex row says, because the pins follow the AD9361's enable lines and in FDD both of those are asserted the entire session. Full duplex goes off with it, so leave both at their defaults if you work satellites and need to hear your own downlink.

    Analog Devices' own note puts an external LNA on GPO0/GPO1, so use GPO2/GPO3 if your board is wired that way. The pins are about 1.3 V at a few milliamps: drive a transistor or an opto-isolator with them, never a relay coil.

    Takes effect on Apply.
settings-radio-buffer-size = Dimensione del buffer
settings-radio-how-much-device-holds-before =
    Quanto trattiene il dispositivo prima di ogni trasferimento, in campioni complessi. Il valore predefinito di 32768 sono circa 16 ms a 2 Msps: abbastanza lungo perché il tempo di andata e ritorno di ogni trasferimento non sia il collo di bottiglia, abbastanza corto perché un cambio di sintonia non sia visibilmente in ritardo.

    Halve it if the log reports the receive socket being replaced. A marginal link — a Pluto reached over its USB cable at a high sample rate is the usual one — stalls part-way through a transfer, and a smaller transfer is both less likely to be caught mid-flight and quicker to make good afterwards. Raise it to trade retune latency for fewer round trips.

    Takes effect on Apply.
settings-radio-samples = {" "}campioni
settings-radio-rx-tx-port = Porta RX / TX
settings-radio-ad9361-s-rf-port-select = L'rf_port_select dell'AD9361. Un Pluto di serie collega uno per tipo, quindi lascia vuoti questi campi a meno che tu non abbia una scheda che non lo fa. Ha effetto con Applica.
settings-radio-opens-radio-reads-what-says = Apre la radio, legge cosa dice di sé, e riporta l'intervallo di sintonia che questa particolare scheda ha. Non avvia alcun flusso.
settings-radio-wideband-iq-receive-transmit-half = I/Q a banda larga in ricezione e trasmissione. Semi-duplex — la ricezione si ferma per la durata di una trasmissione, perché il collegamento di rete USB non regge entrambe insieme. Non servono SoapySDR né libiio.
settings-radio-flexradio-announces-itself-local-network = Una FlexRadio si annuncia sulla rete locale circa una volta al secondo. Una radio raggiunta tramite un router o una VPN non trasmette mai verso di te — inseriscine l'indirizzo qui sotto.
settings-radio-another-gui-client-has-radio = Un altro client grafico ha questa radio e multiFLEX è disattivato. Disconnetti quel client, oppure attiva multiFLEX sulla radio.
settings-radio-overrides-selection-above-use-radio = Prevale sulla selezione qui sopra. Usalo per una radio su un'altra sottorete, dietro una VPN, o su una porta non standard.
settings-radio-width-spectrum-sdroxide-receives-192 = Ampiezza dello spettro che sdroxide riceve. 192 kHz è il massimo della radio per un flusso DAX IQ, quindi l'ampiezza più larga che questa interfaccia può mostrare.
settings-radio-t-192-khz-widest-dax-iq = 192 kHz è il flusso DAX IQ più largo che una FLEX invia, e la radio lega a esso l'ampiezza del panadapter — quindi quella è l'intera ampiezza, e nessuna impostazione qui la allarga.
settings-radio-dax-iq-channel = Canale DAX IQ
settings-radio-radio-has-four-change-only = La radio ne ha quattro. Cambialo solo se qualcos'altro sulla rete sta già usando il canale 1 — la radio rifiuta un canale due volte.
settings-radio-station-name = Nome della stazione
settings-radio-shown-against-session-radio-s = Mostrato accanto a questa sessione nell'elenco client della radio. La radio riconosce anche un client tramite esso, quindi rinominarlo fa sì che la radio tratti sdroxide come uno nuovo.
settings-radio-gui-client-id = ID client grafico
settings-radio-identity-radio-restores-session-s = L'identità con cui la radio ripristina i slice e i panadapter di questa sessione. Lasciato vuoto viene derivato dal nome della stazione — stabile, ma uguale su ogni sdroxide che ha mantenuto il nome predefinito, e una radio risolve un duplicato scacciando il client precedente. sdroxide lo rileva sul filo e ricade su un'identità di una sola sessione, che costa il ripristino; inserisci qui un UUID tuo per mantenerlo.
settings-radio-optional-e-g-uuid-your = facoltativo, ad es. un UUID tuo
settings-radio-mirror-radio-s-i-q =
    Specchia l'I/Q della radio rispetto al centro del panadapter. Disattivato di default, che è come è stata verificata una FLEX-6600.

    Try it if receive audio is unintelligible on USB *and* on LSB and nothing decodes, while the waterfall looks entirely convincing — that is what a mirrored stream looks like, and it is the one fault with no other symptom (issue #368). If it is not that, turning this on makes it obvious rather than subtle. Applies on Apply / reconnect.
settings-radio-network-mtu = MTU di rete
settings-radio-largest-datagram-radio-may-send = Il datagramma più grande che la radio può inviare. 1450 è ciò che chiede lo stesso SmartSDR. Abbassalo su un percorso con MTU più piccolo — una VPN o un tunnel — dove i frammenti vanno persi e non arriva affatto alcuno spettro.
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = Verifica che la radio risponda, senza registrarsi come client grafico — quindi non disturba una sessione di SmartSDR già in corso.
settings-radio-wideband-iq-receive-over-dax = I/Q a banda larga in ricezione tramite DAX, audio in trasmissione che la radio modula. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-not-yet-verified-against-real-2 = Non ancora verificato su hardware reale. Se si comporta male, invia il rapporto diagnostico — contiene ogni riga di protocollo scambiata con la radio.
settings-radio-devices-soapysdr-can-see = Dispositivi che SoapySDR può vedere
settings-radio-waiting-machine-radio-attached-where = In attesa della macchina a cui è collegata la radio, dove sono installati i moduli. Quale dispositivo viene aperto è `device_args` nel config.toml di quella macchina, oppure il suo --device.
settings-radio-ask-every-installed-soapysdr-module = Chiedi a ogni modulo SoapySDR installato di cercare. Non si apre nulla, quindi è sicuro durante la ricezione — ma può richiedere un momento.
settings-radio-not-enumerated-yet-press-rescan = Non ancora enumerato — premi Riesamina.
settings-radio-no-soapysdr-devices-found-check = Nessun dispositivo SoapySDR trovato. Verifica che il modulo per la tua radio sia installato e che tu possa accedere al dispositivo.
settings-radio-not-radio = non è una radio
settings-radio-sound-card-listed-above-if = Una scheda audio è elencata sopra come se fosse un SDR. Accetta qualsiasi frequenza e la ignora, quindi aprirla mostra l'ingresso della scheda audio invece della banda. sdroxide non le sceglie automaticamente — ma una riga device_args che ne indica una viene comunque obbedita.
settings-radio-which-one-opens-set-device = Quale viene aperto è impostato da --device o da device_args nel config.toml; senza nessuno dei due vince la prima radio elencata.
settings-radio-no-device = nessun dispositivo
settings-radio-rig-has-no-software-adjustable = Questo apparato non ha guadagni regolabili via software.
settings-radio-rx-gains = Guadagni RX
settings-radio-tx-gains = Guadagni TX
settings-radio-antennas = Antenne
settings-radio-remembered-next-start = Ricordato per il prossimo avvio.
settings-radio-these-controls-appear-once-soapysdr = Questi comandi compaiono quando un dispositivo SoapySDR è aperto — sono quelli propri del dispositivo, e va interrogato.
settings-radio-stream = Flusso
settings-radio-follow-app-setting = Segui l'impostazione dell'app
settings-radio-msps = oppure, in Msps
settings-radio-baseband-filter = Filtro di banda base
settings-radio-let-driver-choose = Lascia scegliere al driver
settings-radio-changing-either-reopens-radio = Cambiarne uno riapre la radio.
settings-radio-driver-publishes-no-settings-own = Questo driver non pubblica impostazioni proprie.
settings-radio-applied-once-remembered-next-start = Applicato subito, e ricordato per il prossimo avvio.
settings-radio-no-rx-888-found-press = Nessun RX-888 trovato — premi Riesamina
settings-radio-adc-clock = Clock dell'ADC
settings-radio-t-129-6-msps-needs-superspeed = 129,6 Msps richiede un collegamento SuperSpeed e un host veloce; 64,8 è il predefinito sicuro. Cambiarlo riapre automaticamente il ricevitore, cosa che richiede un momento ma non un riavvio.
settings-radio-panadapter-width = Larghezza del panadapter
settings-radio-hf-only = {"  "}· solo HF
settings-radio-usable-both-front-ends = Utilizzabile su entrambi i front-end.
settings-radio-below-vhf-crossover-ordinary-width = Sotto il punto di incrocio VHF questa è una larghezza normale. Sopra di esso la IF da 8 MHz del sintonizzatore non può essere centrata in una finestra così larga, quindi lo spettro in tempo reale sta spostato da un lato del panadapter con nulla accanto.
settings-radio-dial-above-crossover-now-so = La sintonia ora è sopra l'incrocio, quindi questa larghezza mostra l'8 MHz del sintonizzatore fuori centro con spettro morto accanto. Restringi la larghezza per riempire il panadapter.
settings-radio-vga-gain = Guadagno VGA
settings-radio-ad8370-variable-gain-amplifier-ahead = Amplificatore a guadagno variabile AD8370 davanti all'ADC.
settings-radio-attenuator = Attenuatore
settings-radio-pe4304-step-attenuator-0-5 = Attenuatore a passi PE4304, a passi di 0,5 dB.
settings-radio-adc-range = Gamma dell'ADC
settings-radio-wide-2-25-vp-p = Ampia (2,25 Vp-p)
settings-radio-selects-adc-s-wider-input = Seleziona la gamma d'ingresso più ampia dell'ADC: più margine per i segnali forti di radiodiffusione, meno conteggi per quelli deboli. Disattivato seleziona la gamma più sensibile da 1,5 Vp-p.
settings-radio-dither = Dither
settings-radio-adds-small-dither-signal-ahead = Aggiunge un piccolo segnale di dither davanti all'ADC: costa un po' di rumore di fondo, e in cambio dà più gamma dinamica libera da spurie.
settings-radio-randomiser = Randomizzatore
settings-radio-adc-scrambles-output-so-digital = L'ADC mescola la sua uscita così il bus digitale smette di irradiare nel front-end; il driver la rimette in ordine. Lascialo attivo a meno che non stai facendo debug. Ha effetto alla riconnessione.
settings-radio-dc-hf-antenna-port = DC sulla porta d'antenna HF
settings-radio-powers-active-antenna-preamp-down = Alimenta verso il basso un'antenna attiva o un preamplificatore lungo il cavo coassiale.
settings-radio-vhf-tuner-gain = Guadagno del sintonizzatore VHF
settings-radio-r828d-rf-gain-used-above = Guadagno RF dell'R828D, usato sopra l'incrocio automatico HF/VHF. 29 passi discreti; si usa il più vicino.
settings-radio-auto = Automatico
settings-radio-let-tuner-run-own-lna = Lascia che il sintonizzatore faccia girare i propri anelli LNA e mixer.
settings-radio-bias-tee-vhf = Bias tee (VHF)
settings-radio-dc-vhf-antenna-port = DC sulla porta d'antenna VHF
settings-radio-clock-trim = Correzione del clock
settings-radio-corrects-reference-oscillator-applied-when = Corregge l'oscillatore di riferimento. Applicato quando rilasci il valore — riaprire la radio a ogni pixel di un trascinamento la riavvierebbe centinaia di volte.
settings-radio-receive-only-below-adc-s = Solo ricezione. Sotto il limite di Nyquist dell'ADC l'antenna viene campionata direttamente e la risintonia è istantanea, perché non c'è un convertitore hardware di discesa — tutto il flusso dell'ADC viene convertito in banda base dall'host. Sopra di esso il ricevitore passa automaticamente al suo sintonizzatore R828D e alla SMA VHF, quindi entrambe le antenne devono essere collegate. Il VHF richiede un clock ADC di 32,4 Msps o più perché la IF del sintonizzatore possa stare dentro, e a clock sotto 48 Msps resta un vuoto tra le due gamme che nulla può raggiungere. Ogni impostazione qui ha effetto subito — non c'è un pulsante Applica da premere.
settings-radio-device = Dispositivo
settings-radio-no-devices-press-rescan = nessun dispositivo — premi Riesamina
settings-radio-six-rates-are-six-different =
    Le sei frequenze sono sei immagini FPGA diverse, non un registro — ed è per questo che nulla nel protocollo di ELAD seleziona tra di esse, e perché ciò che questa impostazione significa dipende dal modello.

    FDM-S1 / FDM-S2: a command. Their FPGA is loaded by the computer at every power-up, and sdroxide runs ELAD's own elad-firmware loader to put this rate's image in. Without that loader installed a sampler sends no samples at all, and sdroxide says so on screen.

    FDM-DUO: not a command. The radio boots its own FPGA and arrives at whatever rate it powered up in or FDM-SW2 last left it in (192 kHz on a fresh one), and this only says how the stream is READ. Get it wrong and the panadapter is simply the wrong width, with every frequency inside it scaled to match.

    The stream's real rate is measured a couple of seconds after it starts, and a mismatch is reported on screen. Takes effect on Apply.
settings-radio-fdm-s-fpga-image-loaded = FDM-S: l'immagine FPGA caricata — FDM-DUO: come viene letto il flusso
settings-radio-receiver-s-input-pad-one = L'attenuatore d'ingresso del ricevitore — un passo, in più o in meno. Lo stesso comando del cursore Guadagno della finestra principale.
settings-radio-pre-selection-filters = Filtri di preselezione
settings-radio-low-pass-bank-front-adc = Il banco di passa-basso davanti all'ADC. Escluderlo dà la vista più ampia e il comportamento peggiore vicino a segnali forti fuori banda — un trasmettitore di radiodiffusione a qualche miglio di distanza metterà immagini su tutta la banda. Lascialo attivo a meno che tu non stia deliberatamente ascoltando fuori dalla gamma filtrata.
settings-radio-filters-circuit = Filtri nel circuito
settings-radio-rig-control-fdm-duo-only = Controllo dell'apparato — solo FDM-DUO
settings-radio-transceiver-s-cat-usb-port = La porta USB CAT del ricetrasmettitore, che è un cavo separato da quello di ricezione. Lascia la porta non impostata su un FDM-S1 o FDM-S2, che non ne hanno — e su un FDM-DUO preferiresti guidare solo attraverso il suo cavo di ricezione, dove può ancora essere sintonizzato e manipolato ma non si può leggere alcun misuratore.
settings-radio-port-set-radio-s-display = Con la porta impostata, il display della radio e questa sintonia sono lo stesso numero in entrambe le direzioni: la finestra di ricezione dell'FDM-DUO è costruita attorno al suo VFO, quindi sintonizzare qui muove il pannello frontale e girare la manopola del pannello frontale muove la lettura qui. Il panadapter si ricentra sulla sintonia man mano che va. Senza porta — o con una porta a cui la radio non risponde — non c'è alcun VFO da comandare, e la sintonia resta dentro la finestra che la radio sta già inviando.
settings-radio-none-usb-control-only = — nessuna (solo controllo USB) —
settings-radio-must-match-menu-70-cat = Deve corrispondere al menu 70 "CAT BAUD" della radio, che di fabbrica è 38400. La radio ha queste quattro velocità e nessun'altra, e una porta aperta a qualsiasi altra è muta in entrambe le direzioni — la radio ignora la sintonia e rifiuta di attivarsi, senza nulla che spieghi perché.
settings-radio-how-transmit-keyed-cat-sends = Come viene attivata la trasmissione. "CAT" invia il comando TX proprio della radio e non richiede nulla da impostare sull'apparato; RTS richiede il menu 54 "PTT" impostato su PTT+RTS.
settings-radio-where-radio-takes-transmit-audio-2 =
    Da dove la radio prende l'audio di trasmissione — il comando TI dell'apparato, che è il menu 32 "TX IN" sul pannello frontale. Viene impostato all'apertura della porta.

    "USB audio" is what makes transmit work here: the FDM-DUO sends what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.
settings-radio-which-two-sockets-back-receiver-2 =
    Quale delle due prese sul retro ascolta il ricevitore — il comando AN dell'apparato, che è il menu 31 "ANTENNAS" sul pannello frontale e l'indicatore "ANT 1 2" sul suo display.

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    It moves this whole receiver: the panadapter, the demodulators and the rig's own audio all come from the socket selected here.

    Applies immediately, and is read back from the radio when the control port opens — this is the rig's own setting, not a copy of it kept here.
settings-radio-how-often-radio-asked-what-2 = Ogni quanto viene chiesto alla radio cosa sta facendo — la sua sintonia, il suo modo e i suoi misuratori. Gira la manopola VFO dell'apparato e la lettura segue entro un'interrogazione. È anche tutto il traffico di controllo che questa parte genera, quindi abbassalo se l'audio della radio si spezza.
settings-radio-transmit-audio = Audio di trasmissione
settings-radio-only-receiving-fdm-duo-has = Finora è stata eseguita su hardware reale solo la ricezione con un FDM-DUO. Se qualcosa si comporta male, allega il rapporto diagnostico a una segnalazione di bug.
settings-radio-cw-keyed-radio-s-own = Il CW viene manipolato dal tasto o dal paddle propri della radio: l'FDM-DUO non ha un comando che accetti testo, quindi il pannello CW non può manipolarlo.
settings-radio-press-apply-reopen-radio = Premi Applica per riaprire la radio.
settings-radio-no-receivers-press-rescan = nessun ricevitore — premi Riesamina
settings-radio-which-rates-receiver-has-depends = Quali frequenze ha un ricevitore dipende dal modello e dal firmware. Ha effetto con Applica.
settings-radio-every-rate-any-hf-model = ogni frequenza offerta da qualunque modello HF+ — collegane uno per vedere la sua
settings-radio-receiver-s-own-gain-control = Il controllo del guadagno proprio del ricevitore. Lascialo attivo per l'ascolto generico; disattivalo per impostare l'attenuatore a mano per le misure.
settings-radio-agc-threshold = Soglia AGC
settings-radio-high = Alta
settings-radio-high-trades-little-sensitivity-headroom = Alta scambia un po' di sensibilità per margine contro i vicini forti — l'impostazione giusta su una banda affollata di notte.
settings-radio-front-end-attenuation-gain-0 = Attenuazione del front-end, come guadagno — 0 dB significa nessuna. Obbedita solo con l'AGC disattivato.
settings-radio-preamp = Preamplificatore
settings-radio-hf-low-noise-amplifier-buys = L'amplificatore HF a basso rumore. Compra sensibilità al prezzo dell'intermodulazione, quindi è disattivato di default — il che di solito va bene su un'antenna vera.
settings-radio-lna = LNA attivo
settings-radio-frequency-calibration = Calibrazione della frequenza
settings-radio-parts-per-billion-receiver-s = Parti per miliardo — l'unità propria di questo ricevitore, mille volte più fine dei ppm usati da un RTL-SDR. Nulla qui viene mai scritto nella flash del ricevitore: questo sostituisce il valore memorizzato solo per la sessione.
settings-radio-use-receiver-s-stored-value = Usa il valore memorizzato nel ricevitore
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = Invia corrente continua lungo il coassiale
settings-radio-not-every-hf-has-one = Non tutti gli HF+ ne hanno uno; su un ricevitore che non lo ha, non fa nulla.
settings-radio-host-dsp = DSP dell'host
settings-radio-image-balancer-zero-if-offset = Il bilanciatore d'immagine, l'offset zero-IF e l'oscillatore di sintonia fine. Disattivali solo per vedere l'uscita grezza dell'hardware — con essi disattivati, l'immagine speculare compare sulle frequenze zero-IF e la sintonia è precisa solo al kilohertz più vicino.
settings-radio-correct-image-fine-tune = Correggi l'immagine e la sintonia fine
settings-radio-receive-only-no-soapysdr-no = Solo ricezione. Non servono SoapySDR né libairspyhf. Sotto il limite del sintetizzatore è l'oscillatore dell'host a fare la sintonia, ed è così che questo ricevitore raggiunge le VLF. Il ricevitore e la frequenza di campionamento hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi.
settings-radio-not-yet-verified-against-real-3 = Non ancora verificato su hardware reale. Se si comporta male, invia il rapporto diagnostico — contiene ogni comando scambiato con il ricevitore e i primi byte del flusso di campioni decodificati come coppie I/Q.
settings-radio-no-airspy-r2-mini-found = Nessun Airspy R2 o Mini trovato — premi Riesamina
settings-radio-read-receiver = leggi dal ricevitore
settings-radio-r2-mini-offer-different-rates = un R2 e un Mini offrono frequenze diverse e non si possono distinguere finché uno non è aperto
settings-radio-rate-you-get-receiver-s = Questa è la frequenza che ottieni. L'ADC del ricevitore gira al doppio — digitalizza un segnale reale e sdroxide ne ricava banda base complessa sull'host.
settings-radio-gain-curve = Curva del guadagno
settings-radio-step-along-curve-above-not = Un passo lungo la curva qui sopra, non un valore in dB — LNA, mixer e VGA del sintonizzatore si muovono insieme, e quanto vale ogni passo dipende dalla curva e dalla banda. 0 è l'estremo silenzioso.
settings-radio-tuner-agc = AGC del sintonizzatore
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = Consegna l'LNA all'anello proprio del sintonizzatore, sostituendo la curva.
settings-radio-mixer = Mixer
settings-radio-same-mixer-stage = Lo stesso per lo stadio mixer.
settings-radio-loop-running-gain-slider-no = Con un anello attivo, il cursore del guadagno non imposta più lo stadio che possiede — l'anello lo sovrascrive.
settings-radio-dc-antenna-port = DC sulla porta d'antenna
settings-radio-t-12-bit-packing = Compattamento a 12 bit
settings-radio-third-less-usb-traffic-leave = Un terzo di traffico USB in meno. Lascialo attivo: è un dispositivo USB 2.0 e la frequenza massima è 30 MB/s compattata contro 40 non compattata. Ha effetto alla riconnessione.
settings-radio-dc-removal = Rimozione DC
settings-radio-remove-adc-s-offset = Rimuovi l'offset dell'ADC
settings-radio-turn-off-see-raw-hardware = Disattivalo per vedere l'uscita grezza dell'hardware. Vale la pena sapere dove finisce lo spurio: l'offset cade al bordo dell'ampiezza, non al centro, perché il segnale viene traslato di un quarto della frequenza di campionamento lungo il percorso.
settings-radio-every-command-exchanged-receiver-sample = Ogni comando scambiato con il ricevitore, l'aritmetica delle frequenze di campionamento, e i primi campioni decodificati come I/Q.
settings-radio-receive-only-24-1800-mhz = Solo ricezione, 24–1800 MHz. Non servono SoapySDR né libairspy. Il ricevitore, la frequenza di campionamento e il compattamento hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi.
settings-radio-not-yet-verified-against-real-4 = Non ancora verificato su hardware reale. Se si comporta male, invia il rapporto diagnostico — contiene ogni comando scambiato con il ricevitore, l'aritmetica delle frequenze e i primi campioni decodificati come coppie I/Q.
settings-radio-no-hydrasdr-rfone-found-press = Nessun HydraSDR RFOne trovato — premi Riesamina
settings-radio-these-are-rates-receiver-turned = queste sono le frequenze che questo ricevitore si è rivelato avere
settings-radio-receiver-only-reports-three-these = il ricevitore ne riporta solo tre; le altre sono nella tabella alternativa del firmware e una build più vecchia potrebbe non includerle
settings-radio-rf-input = Ingresso RF
settings-radio-antenna-sma-only-socket-bias = La SMA d'antenna — l'unica presa con il bias tee dietro di essa.
settings-radio-cable-socket-no-bias-tee = Una presa per cavo. Qui non c'è bias tee; l'hardware lo mette sulla sola ANT.
settings-radio-powers-active-antenna-preamp-down-2 = Alimenta verso il basso un'antenna attiva o un preamplificatore lungo il coassiale. Solo la presa ANT ce l'ha — le due porte per cavo sono ingressi semplici.
settings-radio-third-less-usb-traffic-leave-2 = Un terzo di traffico USB in meno. Lascialo attivo: è un dispositivo USB 2.0 e la frequenza massima è 36 MB/s compattata contro 48 non compattata. Ha effetto alla riconnessione.
settings-radio-one-these-boards-1d50-60a1 = Una di queste schede è su 1d50:60a1, l'ID USB che i prototipi di HydraSDR condividono con l'Airspy R2 e il Mini. sdroxide controlla il firmware dopo l'apertura e lo segnala se è stata scelta l'interfaccia sbagliata, in entrambi i sensi.
settings-radio-receive-only-24-1800-mhz-2 = Solo ricezione, 24–1800 MHz. Non servono SoapySDR né libhydrasdr. Il ricevitore, la frequenza di campionamento e il compattamento hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi.
settings-radio-re-list-fobos-sdrs-no = Rielenca gli SDR Fobos. Nessun dispositivo viene aperto, quindi è sicuro premerlo durante la ricezione.
settings-radio-no-fobos-sdr-found-press = Nessun SDR Fobos trovato — premi Riesamina
settings-radio-input-2 = Ingresso
settings-radio-through-tuner-lna-vga-mixer = Attraverso il sintonizzatore: LNA, VGA e l'intervallo di frequenza proprio del mixer.
settings-radio-direct-sampling-hf1-antenna-input = Campionamento diretto sull'ingresso d'antenna HF1 — nessun sintonizzatore, nessun guadagno LNA/VGA, sintonizzato interamente via software.
settings-radio-direct-sampling-hf2-antenna-input = Campionamento diretto sull'ingresso d'antenna HF2, come HF1.
settings-radio-both-real-adc-channels-once = Entrambi i canali ADC reali insieme, combinati dal filtro di diversità qui sotto — una sorgente di rumore annullata, o due percorsi con evanescenza combinati.
settings-radio-direct-sampling-tuner-so-lna = Campionamento diretto — il sintonizzatore (e quindi il guadagno LNA/VGA) non è nel percorso. La frequenza qui sotto è un obiettivo per il convertitore a discesa software, non un'impostazione hardware, quindi la frequenza effettivamente ottenuta può differire da essa.
settings-radio-receiver-s-own-reported-rates = le frequenze che questo ricevitore riporta da sé
settings-radio-measured-one-real-unit-connect = misurate su un'unità reale — collega questo ricevitore e premi Riesamina per vedere il suo elenco
settings-radio-clock-source = Sorgente di clock
settings-radio-external-reference = Riferimento esterno
settings-radio-diversity-taps = Prese di diversità
settings-radio-taps = {" "}prese
settings-radio-one-tap-gain-phase-null =Una presa è un guadagno e una fase — un nullo a una frequenza che peggiora da entrambi i lati, che è tutto ciò che un fasatore analogico può fare. Ogni presa in più compra un periodo di campionamento della differenza di percorso tra HF1 e HF2 che il filtro può equalizzare, ed è ciò che trasforma quel notch in una banda silenziosa per tutta la sua estensione.
settings-radio-receive-only-rf-port-tuner = Solo ricezione. Porta RF (sintonizzatore), HF1/HF2 (campionamento diretto, decodificato via software) e HF1+HF2 (entrambi insieme, combinati dal filtro di diversità) funzionano tutti. Il ricevitore, l'ingresso e la frequenza di campionamento hanno effetto con Applica; guadagno, sorgente di clock e prese hanno effetto mentre li modifichi — la modalità, la frequenza di adattamento e il blocco del filtro di diversità stanno nel box DIV della finestra principale una volta selezionata questa radio.
settings-radio-no-hackrf-found-press-rescan = Nessun HackRF trovato — premi Riesamina
settings-radio-front-end-amplifier-8-db = Amplificatore del front-end, a passi di 8 dB. È lo stadio che cambia la sensibilità — e lo stadio che va in sovraccarico per primo su un'antenna vera.
settings-radio-baseband-amplifier-after-mixer-2 = Amplificatore di banda base dopo il mixer, a passi di 2 dB. Alzalo per un segnale debole prima di ricorrere all'LNA.
settings-radio-rf-amp = Amplificatore RF
settings-radio-one-switch-shared-transmit-setting = Un solo interruttore, condiviso con l'impostazione di trasmissione qui sotto — la radio applica quello che appartiene alla direzione in cui sta entrando. Disattivato di solito va bene su un'antenna vera.
settings-radio-hackrf-pro-chooses-itself-three = Un HackRF Pro sceglie questo da sé — tre quarti della frequenza di campionamento, filtrata nell'FPGA — e ignora tutto ciò che l'host chiede.
settings-radio-leave-automatic-choosing-one-too = Lascialo su Automatico. Sceglierne uno troppo stretto non ammorbidisce soltanto i bordi della banda — ritira l'offset dell'oscillatore locale che tiene il picco DC lontano dal tuo segnale.
settings-radio-about-3-v-50-ma = Circa 3 V a 50 mA lungo il coassiale, per un'antenna attiva o un preamplificatore. Solo HackRF One o Pro; il Jawbreaker e il rad1o non hanno un simile circuito.
settings-radio-remove-dc-mirror-image = Rimuovi DC e l'immagine speculare
settings-radio-zero-if-radio-so-own = Questa è una radio zero-IF, quindi la propria fuga dell'oscillatore locale sta a metà ampiezza e l'errore di quadratura del mixer vi mette sopra un'immagine speculare. Disattivarlo mostra l'uscita grezza dell'hardware, che è il modo rapido per distinguere un problema del driver da uno del DSP.
settings-radio-corrects-reference-oscillator = Corregge l'oscillatore di riferimento.
settings-radio-transmit = Trasmissione
settings-radio-transmitter = Trasmettitore
settings-radio-off-default-while-off-backend = Disattivato di default. Finché è spento, il backend non pubblica alcun canale di trasmissione, quindi nulla può attivare la radio. Ha effetto alla riconnessione.
settings-radio-tx-vga-gain = Guadagno VGA in trasmissione
settings-radio-transmit-driver-amplifier-drive-applied = L'amplificatore pilota della trasmissione. Il pilotaggio viene applicato in digitale prima di questo stadio, quindi lascia il pilotaggio alto e imposta qui il livello di uscita — abbassare il pilotaggio fa invece lavorare il DAC a una frazione del fondo scala e aumenta l'intermodulazione.
settings-radio-same-switch-receive-setting-above = Lo stesso interruttore dell'impostazione di ricezione qui sopra, applicato quando la radio cambia direzione.
settings-radio-transmit-armed-into-dummy-load = La trasmissione è armata. Su un carico fittizio finché non l'hai misurata: le armoniche di un HackRF sono abbastanza forti da richiedere un filtro passa-basso esterno per la banda su cui sei, ed è semi-duplex, quindi la ricezione si ferma per la durata di ogni trasmissione.
settings-radio-every-command-exchanged-radio-session = Ogni comando scambiato con la radio in questa sessione, in ordine, compresa la sequenza attorno a ogni attivazione.
settings-radio-dc-7-25-ghz-half = DC – 7,25 GHz, semi-duplex. Non servono SoapySDR, libusb né libhackrf. La radio e la frequenza di campionamento hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi. I limiti della banda sono quelli del firmware: un HackRF è specificato per 1 MHz – 6 GHz (100 kHz su un Pro) ed è fortemente attenuato fuori di esso, ma lì si sintonizza.
settings-radio-ask-sdrplay-api-service-device = Chiedi al servizio SDRplay API l'elenco dei dispositivi. Nulla viene aperto, quindi è sicuro premerlo durante la ricezione.
settings-radio-no-rsps-press-rescan-needs = nessun RSP — premi Riesamina (richiede il servizio SDRplay API)
settings-radio-both-tuners-running-api-fixes = Con entrambi i sintonizzatori attivi, l'API fissa l'ADC a 6 MHz e restituisce 2 Msps da una IF bassa — quindi 2 Msps è l'ampiezza più larga, e quelle più strette sono questa ridotta. Ha effetto con Applica.
settings-radio-rates-below-2-msps-run = Le frequenze sotto 2 Msps fanno girare l'ADC a 2 Msps e riducono nel servizio. Ha effetto con Applica.
settings-radio-reduced-adc-resolution = {"  "}(risoluzione ADC ridotta)
settings-radio-if-bandwidth = Larghezza di banda IF
settings-radio-tuner-s-analog-filter-auto = Il filtro analogico del sintonizzatore. Automatico sceglie il più largo che entra nella frequenza di campionamento. Ha effetto con Applica.
settings-radio-rsp-s-own-if-gain = L'anello di guadagno IF proprio dell'RSP, eseguito dal servizio API. Disattivato restituisce il cursore del guadagno IF a te — l'impostazione per le misure e per i modi digitali a segnale debole.
settings-radio-agc-set-point = Punto di riferimento dell'AGC
settings-radio-signal-level-loop-holds-adc = Livello del segnale che l'anello mantiene sull'ADC. Più basso lascia più margine per i segnali fuori canale. Quanto può scendere dipende dalla frequenza di campionamento — sopra 8,064 Msps il convertitore ha meno margine da offrire, e sopra 9,216 Msps ancora meno.
settings-radio-rsp-s-native-gain-unit = L'unità di guadagno nativa dell'RSP: 20 dB è il guadagno massimo (0 con la gamma estesa qui sotto), 59 dB il minimo. Ha effetto subito. Ignorato mentre l'AGC è attivo — allora il valore è dell'anello, e lo S-meter mostra su cosa si è stabilizzato.
settings-radio-extended-if-range = Gamma IF estesa
settings-radio-lets-if-gain-reduction-go = Lascia che la riduzione del guadagno IF scenda sotto 20 dB, fino a 0 — gli ultimi 20 dB di guadagno che il ricevitore ha. Disattivato è il predefinito dell'API e quello giusto per l'ascolto normale, perché il fondo della gamma è dove un RSP è più facile da sovraccaricare. Vale la pena attivarlo per i segnali deboli con l'LNA già a 0. Permette anche al punto di riferimento dell'AGC di salire fino a 0 dBFS. Ha effetto subito.
settings-radio-allow-if-gain-reduction-below = Consenti la riduzione del guadagno IF sotto 20 dB
settings-radio-front-end-attenuation-steps-0 = Attenuazione del front-end a passi: 0 è il guadagno massimo, ogni passo in più inserisce più attenuazione. Alcune bande hanno meno passi — il driver li limita e conserva la tua scelta per quando risintonizzi, e il cursore qui finisce dove arriva la banda su cui è la radio. Una scelta più alta fatta su un'altra banda viene conservata e mostrata accanto. Ha effetto subito.
settings-radio-t-0-max-gain = 0 = guadagno massimo
settings-radio-reference-error-parts-per-million-2 = Errore di riferimento in parti per milione, applicato dal dispositivo stesso. Ha effetto subito.
settings-radio-radio-s-tuner = Il sintonizzatore di questa radio
settings-radio-tuner = Sintonizzatore
settings-radio-which-rspduo-s-two-tuners = Quale dei due sintonizzatori dell'RSPduo ascolta questa radio. L'altro appartiene alla radio configurata per esso — dai a quest'ultima lo stesso ricevitore e l'altro sintonizzatore. Ha effetto con Applica.
settings-radio-which-rspduo-s-two-tuners-2 = Quale dei due sintonizzatori dell'RSPduo porta l'antenna che stai ascoltando. L'altro porta la seconda antenna. Ha effetto con Applica.
settings-radio-which-rspduo-s-two-tuners-3 = Quale dei due sintonizzatori dell'RSPduo far girare (uno alla volta). Ha effetto con Applica.
settings-radio-applies-immediately = Ha effetto subito.
settings-radio-dab-notch = Notch DAB
settings-radio-hardware-notch-over-165-230 = Notch hardware sulla banda DAB 165–230 MHz. Ha effetto subito.
settings-radio-hdr-mode = Modalità HDR
settings-radio-enable-below-2-mhz = Attiva sotto 2 MHz
settings-radio-rspdx-s-high-dynamic-range =
    Il percorso ad alta gamma dinamica dell'RSPdx per LF/MF. Ha effetto subito.

    It does not work yet: on the one RSPdx it has been tried against, switching it on silences the receiver — at every centre, filter, port, gain and rate tried. Whatever the mode needs is not on the documented API surface, and finding it is somebody's next job. Leave this off unless you are the one looking.

    What is known: it is not a mode that follows the dial. The path has a fixed analog filter, built only at the centres listed under the filter below, so tuning anywhere else could not work even once the rest does.
settings-radio-hdr-filter = Filtro HDR
settings-radio-feed-4-7-v-dc = Invia circa 4,7 V DC lungo il coassiale
settings-radio-second-tuner = Secondo sintonizzatore
settings-radio-rspduo-two-whole-tuners-one = Un RSPduo sono due sintonizzatori interi su una sola scheda, con clock da un unico riferimento. Fai girare entrambi e sentono le loro ampiezze nello stesso istante dallo stesso clock — il che rende coerenti due antenne su di essi, con una fase relativa fissata dai cavi di alimentazione anziché dal caso, e permette di combinare la coppia. Oppure tienili separati: i sintonizzatori si sintonizzano indipendentemente, quindi l'altro può essere una seconda radio sulla sua banda.
settings-radio-run-both-tuners = Fai girare entrambi i sintonizzatori
settings-radio-puts-rspduo-api-s-dual = Mette l'RSPduo nella modalità a doppio sintonizzatore dell'API, dove l'ADC è fissato a 6 MHz e l'ampiezza più larga è 2 Msps. Ha effetto con Applica, perché la modalità viene scelta all'apertura del dispositivo.
settings-radio-used = Usato per
settings-radio-what-other-tuner-takes-effect = A cosa serve l'altro sintonizzatore. Ha effetto con Applica — come far girare entrambi, questo viene scelto all'apertura della scheda.
settings-radio-add-second-radio-settings-radio = Aggiungi una seconda radio (Impostazioni → Radio → +), dalle questo stesso ricevitore e l'altro sintonizzatore dell'RSPduo, e impostala a far girare anche entrambi i sintonizzatori — qualunque radio apra la scheda la mette in modalità a doppio sintonizzatore, quindi entrambe devono aspettarselo. Le due poi si sintonizzano indipendentemente: HF in una scheda e VHF nell'altra, da una sola scheda. Condividono un unico clock ADC, quindi girano entrambe alla frequenza di campionamento richiesta da qualunque radio abbia aperto la scheda, e nessuna delle due può trasmettere.
settings-radio-what-do = Cosa farne
settings-radio-lna-state-2 = Il suo stato LNA
settings-radio-second-tuner-s-own-front = L'attenuazione del front-end proprio del secondo sintonizzatore. Impostata perché entrambe le antenne mostrino circa lo stesso rumore di fondo: è la regolazione su cui poggia tutto il resto, perché combinare pesa i due rami per il loro rumore, e un secondo front-end portato in sovraccarico consegna al filtro una copia distorta dell'interferenza — che non si può sottrarre da una non distorta. Ha effetto subito.
settings-radio-if-gain-reduction-2 = La sua riduzione del guadagno IF
settings-radio-second-tuner-s-if-gain = Il guadagno IF del secondo sintonizzatore, nell'unità nativa dell'RSP: 20 dB è il guadagno massimo. Ignorato mentre l'AGC è attivo — e un guadagno fisso è ciò che vuole il filtro, quindi vale la pena spegnere l'AGC per un nullo che intendi mantenere.
settings-radio-filter-length = Lunghezza del filtro
settings-radio-one-tap-gain-phase-null-2 = Una presa è un guadagno e una fase — un nullo a una frequenza che peggiora da entrambi i lati, che è tutto ciò che un fasatore analogico può fare. Ogni presa in più compra un periodo di campionamento della differenza di percorso tra le due antenne che il filtro può equalizzare, ed è ciò che trasforma quel notch in una banda silenziosa per tutta la sua estensione.
settings-radio-adaptation-2 = Adattamento
settings-radio-slow-steady-left-converging-inside = Lento e costante a sinistra, converge in meno di un secondo e a destra si vede che cerca. Parti veloce per trovare il nullo, poi mantienilo.
settings-radio-stop-filter-moving-reach-moment = Ferma il movimento del filtro. Raggiungilo nel momento in cui compare un nullo: un filtro lasciato in adattamento si rimirerà verso qualunque cosa diventi più forte, che su una banda tranquilla è la stazione che stai ascoltando.
settings-radio-restart = Riavvia
settings-radio-zero-filter-find-null-again = Azzera il filtro e ritrova il nullo.
settings-radio-nothing-here-can-tell-wanted = Nulla qui può distinguere un segnale desiderato da uno indesiderato — il filtro sa solo ciò che le due antenne hanno in comune. In Cancella, la seconda antenna vuole sentire la sorgente di rumore e il meno possibile della banda, altrimenti cancellerà diligentemente anche la stazione. In Combina, entrambe vogliono sentire la stessa stazione. Quanto profondo è il nullo finisce nel log ogni pochi secondi.
settings-radio-not-yet-verified-against-rspduo = Non ancora verificato su un RSPduo: il funzionamento a doppio sintonizzatore qui è scritto a partire dall'API di SDRplay anziché misurato sull'hardware.
settings-radio-no-receiver-listed-check-against = Nessun ricevitore è elencato con cui verificarlo — premi Riesamina, oppure leggi nel log cosa si è davvero aperto.
settings-radio-receive-only-1-khz-2 = Solo ricezione, 1 kHz–2 GHz. Richiede il servizio API SDRplay del produttore (sdrplay.com/api) — gli RSP successivi all'RSP1 originale non hanno un protocollo aperto. Dispositivo, frequenza di campionamento, banda e sintonizzatore RSPduo hanno effetto con Applica; tutto il resto ha effetto mentre lo modifichi.
settings-radio-board = Scheda
settings-radio-first-one-found-2 = Il primo trovato
settings-radio-receive-chain = Catena di ricezione
settings-radio-both-chains-tune-together-they = Entrambe le catene si sintonizzano insieme — condividono un unico sintetizzatore — ma sono front-end separati su prese separate. Scegli quella su cui è la tua antenna, che è l'impostazione da raggiungere quando una catena ha avuto la modifica di adattamento HF e l'altra è di serie.
settings-radio-receive-gain = Guadagno in ricezione
settings-radio-one-combined-figure-which-limesuite = Un'unica cifra combinata, che LimeSuite distribuisce tra LNA, TIA e PGA da sé. Accetta decibel interi, quindi tutto ciò che è più fine viene troncato.
settings-radio-receive-port = Porta di ricezione
settings-radio-automatic-follows-frequency-lnal-low = Automatico segue la frequenza: LNAL bassa, LNAH alta — a meno che un LimeRFE non sia collegato più in basso, che è un cavo su una presa, e allora è LNAW a ogni frequenza. Indica la presa su cui è cablata la tua, se non è quella. A quale catena appartiene la presa è il selettore qui sopra.
settings-radio-t-0-follows-sample-rate-worth = 0 segue la frequenza di campionamento. Vale la pena lasciarlo lì: un filtro più stretto di un quarto dell'ampiezza ritira in silenzio l'offset dell'oscillatore locale zero-IF, e la fuga dell'LO torna sopra il segnale che stai ascoltando.
settings-radio-following-rate = seguendo la frequenza
settings-radio-corrections = Correzioni
settings-radio-host-iq-dc-correction = Correzione I/Q / DC dell'host
settings-radio-adaptive-image-dc-removal-side = Rimozione adattiva dell'immagine e del DC da questo lato, in aggiunta alla calibrazione propria del chip. Disattivarla è il modo con un clic per distinguere un problema del driver da uno del DSP.
settings-radio-calibrate-automatically = Calibra automaticamente
settings-radio-runs-chip-s-own-dc = Esegue la calibrazione propria del chip per offset DC e immagine quando la radio viene aperta, e di nuovo quando la sintonia si è assestata su una nuova banda o un'altra presa. Quei valori sono misurati a una frequenza e sono sbagliati altrove, che è ciò che di solito è una portante al centro dell'ampiezza. Costa circa un secondo ogni volta.
settings-radio-calibrate-now = Calibra ora
settings-radio-stalls-receiver-better-part-second = Blocca il ricevitore per gran parte di un secondo.
settings-radio-second-aerial = Seconda antenna
settings-radio-socket = La sua presa
settings-radio-same-first = Uguale alla prima
settings-radio-gain-2 = Il suo guadagno
settings-radio-set-low-coupled-sample-your = Impostalo BASSO. Il campione accoppiato del tuo stesso trasmettitore è un segnale forte, e una catena di retroazione portata in compressione misura male la curva dell'amplificatore — insegna alla correzione la propria distorsione. Parti dal fondo e usa l'attenuatore dell'accoppiatore.
settings-radio-set-so-both-aerials-show = Impostalo perché entrambe le antenne mostrino circa lo stesso rumore di fondo. È la regolazione su cui poggia tutto: combinare pesa i due rami per il loro rumore, e una seconda catena portata in compressione consegna al filtro una copia distorta dell'interferenza — che non si può sottrarre da una non distorta.
settings-radio-table-steps-2 = Passi della tabella
settings-radio-how-finely-correction-follows-amplifier = Quanto finemente la correzione segue la curva dell'amplificatore. Più passi seguono un ginocchio più netto, ma ciascuno va appreso dai campioni che vi sono finiti — e la parte alta di un istogramma di ampiezza vocale è rada. Trentadue bastano per la curva liscia che un amplificatore HF ha davvero. Cambiarlo fa ricominciare la correzione.
settings-radio-how-hard-each-block-feedback = Quanto forte ogni blocco di retroazione sposta la correzione. La curva di un amplificatore non cambia, quindi non c'è bisogno di fretta — il centro media via il rumore di più trasmissioni.
settings-radio-keep-correction-curve-learned-clean = Mantieni la correzione com'è. Una curva appresa su una trasmissione pulita vale la pena di essere tenuta, e l'amplificatore non sarà cambiato alla successiva.
settings-radio-forget-correction-learn-again = Dimentica la correzione e riapprendila.
settings-radio-directional-coupler-amplifier-s-output = Un accoppiatore direzionale sull'uscita dell'amplificatore entra in questa catena, e il trasmettitore confronta ciò che è tornato con ciò che intendeva inviare — poi invia l'inverso della differenza, così ciò che esce dall'amplificatore è diritto. Una ventina di decibel in meno di intermodulazione sulle QSO altrui, con l'amplificatore che mantiene la sua potenza. La correzione resta unitaria finché la retroazione non si allinea alla trasmissione, quindi un accoppiatore non collegato non costa nulla; e non può mai chiedere al convertitore più del fondo scala, quindi un percorso di retroazione che legge sciocchezze non può sovrapilotare nulla. Come sta andando finisce nel log mentre trasmetti.
settings-radio-transmit-not-armed-so-there = La trasmissione non è armata, quindi non c'è nulla qui da correggere.
settings-radio-off-interface-publishes-no-transmit = Con questo disattivato l'interfaccia non pubblica alcun canale di trasmissione, quindi nulla può attivare la radio.
settings-radio-transmit-gain = Guadagno in trasmissione
settings-radio-transmit-port = Porta di trasmissione
settings-radio-board-s-two-transmit-sockets = Le due prese di trasmissione della scheda sono due reti di adattamento diverse, non due jack sulla stessa: BAND1 (TX_1) copre da 30 MHz a 1,9 GHz e BAND2 (TX_2) è la porta dei 13 cm. Attivare una banda radioamatoriale sotto i 23 cm dalla BAND2 manda la trasmissione in una rete di adattamento che ne lascia passare quasi nulla — ogni impostazione risulta corretta, il pilotaggio è quello che hai impostato, e il misuratore di potenza resta a zero. Lascialo su Automatico e segue la sintonia.
settings-radio-limesdr-transmits-about-100-khz = Un LimeSDR trasmette da circa 100 kHz a 3,8 GHz senza filtraggio proprio. Usa un filtro passa-basso, un canale LimeRFE appropriato, oppure un carico fittizio.
settings-radio-limerfe-front-end = Front-end LimeRFE
settings-radio-connected = Collegato tramite
settings-radio-limerfe-s-own-micro-usb = La porta micro-USB propria del LimeRFE, non quella della radio. 9600 baud, fissi dal suo firmware.
settings-radio-receive-connector = Connettore di ricezione
settings-radio-transmit-connector = Connettore di trasmissione
settings-radio-follow-dial = Segui la sintonia
settings-radio-switch-filters-match-operating-frequency = Commuta i filtri in modo che corrispondano alla frequenza operativa, prima che compaia qualsiasi RF. Sintonizzarsi all'interno di una banda non invia nulla sul collegamento di controllo.
settings-radio-relays = Relè
settings-radio-receive-attenuator = Attenuatore in ricezione
settings-radio-notch-filter = Filtro notch
settings-radio-fan = Ventola
settings-radio-worth-having-any-sustained-transmitting = Vale la pena tenerla attiva per qualsiasi trasmissione prolungata.
settings-radio-automatic-board-receives-switched-transmit = Su Automatico la scheda riceve, e viene commutata in trasmissione al key-down e riportata in ricezione al key-up — con qualunque cablaggio. I suoi canali radioamatoriali hanno un filtro con un commutatore trasmissione/ricezione su ciascun lato, quindi una scheda a cui vengono chieste entrambe le cose insieme mette quel commutatore sul trasmettitore e smette di sentire qualsiasi cosa. Il canale dei 30 MHz è raggiungibile solo tramite J5, che è un unico connettore per entrambe le direzioni; sopra di esso, trasmettere da J4 tiene il percorso di ricezione lontano dal connettore che pilota l'amplificatore.
settings-radio-limesdr-support-has-not-been = Il supporto LimeSDR non è stato verificato su hardware. Questa è la traccia dell'ultima sessione, per una segnalazione di problema.
settings-radio-gains-filters-corrections-antenna-sockets = Guadagni, filtri, correzioni, le prese d'antenna e ogni comando LimeRFE — i suoi connettori, la banda, i relè, l'attenuatore, il notch e la ventola — hanno effetto subito. La scheda, la catena di ricezione, la frequenza di campionamento, l'armamento della trasmissione e la connessione del LimeRFE hanno effetto con Applica.

## Settings window, Radio tab: messages with values

settings-radio-fill-from = RIEMPI DA { $name }
settings-radio-decimation-auto = Automatico (più vicina { $khz } kHz)
settings-radio-decimation-stage-server = Stadio { $stage } — la frequenza del server ÷ { $divisor }
settings-radio-decimation-stage = Stadio { $stage } — frequenza ÷ { $divisor }
settings-radio-kiwi-receive-only = Solo ricezione — è l'antenna di qualcun altro. Il panadapter è il circa 12 kHz che il ricevitore invia come I/Q, e segue la sintonia; la vista di banda sopra di esso è la sua waterfall, e sintonizzarsi attraverso di essa risintonizza il ricevitore. Circa { $rate } kB/s mentre è collegato. Premi "Applica / riconnetti" per cambiare senza riavviare.
settings-radio-connected-detail = Collegato: { $detail }
settings-radio-failed = Errore: { $error }
settings-radio-pluto-buffer = ≈ { $ms } ms, { $kib } KiB per trasferimento
settings-radio-driver-settings = Impostazioni di { $driver }
settings-radio-rx888-coverage = Copertura 0–{ $mhz } MHz, { $mbytes } MB/s via USB
settings-radio-rx888-digitised = dei { $mhz } MHz digitalizzati
settings-radio-elad-16-bit = { $khz } kHz (campioni a 16 bit)
settings-radio-elad-pad = Attenuatore d'ingresso da { $db } dB
settings-radio-hackrf-rx-preamp = Preamplificatore da { $db } dB in ricezione
settings-radio-hackrf-tx-preamp = Preamplificatore da { $db } dB in trasmissione
settings-radio-device-not-found = { $device } (non trovato)
settings-radio-lime-chain = Catena { $n } (RX{ $n }_*)
settings-radio-lime-chain-tx = Catena { $n } (RX{ $n }_* / TX{ $n }_*)
settings-radio-lime-second-chain = L'altra catena di ricezione della scheda, sulle prese { $sockets }. Condivide il sintetizzatore, quindi sente la stessa ampiezza nello stesso istante della prima — che è ciò che le permette di portare una seconda antenna, oppure un campione del tuo stesso trasmettitore.
settings-radio-tx-blocked = La trasmissione è bloccata: { $reason }
settings-radio-rfe-rx-fallback = In ricezione su { $port }, per queste si ricade sul percorso a banda larga non filtrato: { $bands }.
settings-radio-rfe-tx-fallback = In trasmissione su { $port }, per queste si ricade sul percorso a banda larga — né amplificatore di banda né filtraggio: { $bands }.

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = Volume, guadagno, AGC, squelch, i comandi del rumore e la lettura RDS sulla radiodiffusione FM
topbar-receiver = RICEVITORE
topbar-vfo-b-split-rit-xit = VFO A/B, split e gli offset RIT/XIT
topbar-tuning = SINTONIA
topbar-diversity-filter-which-way-combines = Il filtro di diversità: in che modo combina le due antenne, quanto in fretta si adatta, e il blocco nella posizione attuale
topbar-diversity = DIVERSITÀ
topbar-second-receiver-s-frequency-mode = La frequenza, il modo, il filtro e il livello del secondo ricevitore
topbar-sub-receiver = SUB RICEVITORE
topbar-transceiver-s-own-aerial-socket = La presa d'antenna propria del ricetrasmettitore, e il suo interruttore di alimentazione
topbar-radio = RADIO
topbar-tune-voice-keyer-drive-mic = Accordo, il manipolatore vocale, e i livelli di pilotaggio e microfono
topbar-transmit = TRASMISSIONE
topbar-tune = {" "}TUNE{" "}
topbar-display = DISPLAY
topbar-logbook-spots-awards-memories-scanner = Logbook, spot, premi, memorie, lo scanner, le impostazioni e il manuale
topbar-system = SISTEMA
topbar-linked = Collegato
topbar-not-linked = Non collegato
topbar-bar-trace = barra / traccia
topbar-needle-bar-trace = ago / barra / traccia
topbar-swap-vfos = Scambia i VFO
topbar-copy-b = Copia A in B
topbar-second-receiver-right-ear-tunes = Secondo ricevitore, nell'orecchio destro. Si sintonizza indipendentemente da A/B — i suoi comandi compaiono nel modulo SUB, e la sua banda passante sulla waterfall.
topbar-band-plan = , dal piano di banda
topbar-repeater-shift-following-band-plan = Spostamento ripetitore: segue il piano di banda, che per questa frequenza non ha spostamento — simplex
topbar-repeater-shift-transmit-above-below = Spostamento ripetitore: trasmetti sopra o sotto la sintonia, per lavorare un ripetitore. Ora simplex
topbar-repeater-shift = SPOSTAMENTO RIPETITORE
topbar-auto = AUTO
topbar-take-shift-band-plan-dial = Prende lo spostamento dal piano di banda man mano che la sintonia si muove. Interviene solo dentro una sotto-banda di uscita di ripetitore — altrove lascia la radio in simplex, così i canali di chiamata restano in simplex.
topbar-offset = Offset
topbar-repeater-tone-ctcss-dcs-goes = Tono del ripetitore: il CTCSS/DCS inviato sotto la voce, il burst a 1750 Hz, e lo squelch a tono in ricezione
topbar-transmit-tone = TONO IN TRASMISSIONE
topbar-set-here-sent-nfm-sub = impostato qui, inviato in NFM — la segnalazione subaudibile è propria di un canale FM
topbar-normal = NORMALE
topbar-invert = INVERTI
topbar-bit-order-dcs-encoded-here = L'ordine dei bit con cui qui viene codificato il DCS è trascritto dallo standard e non è mai stato verificato su un ripetitore — se non si apre, prova il CTCSS
topbar-t-1750-hz-burst = BURST 1750 HZ
topbar-every-over = OGNI TRASMISSIONE
topbar-open-burst-every-time-transmitter = Apre con il burst ogni volta che il trasmettitore viene attivato
topbar-send = INVIA
topbar-send-burst-now-keying-transmitter = Invia il burst ora — attivando il trasmettitore per la sua durata, se non è già attivo
topbar-nfm-only-burst-fm-repeater = Solo NFM: il burst è l'apriporta di un ripetitore FM
topbar-receive-tone-squelch = SQUELCH A TONO IN RICEZIONE
topbar-require-receive-what-station-transmits = Richiedi in ricezione ciò che questa stazione trasmette
topbar-hide-docked-band-selector-click = Nascondi il selettore di banda agganciato — clicca di nuovo per riportarlo
topbar-show-docked-band-selector = Mostra il selettore di banda agganciato
topbar-band-mode = BANDA E MODO
topbar-hide-band-chip-brings-back = Nascondi — il chip di banda lo riporta
topbar-dec-off = DEC off
topbar-vol = Vol
topbar-counted-steps-not-decibels-what =


    Counted in steps, not decibels: what one step is worth is the receiver's own business, and on an SDRplay it depends on the band as well. Right is still more gain.
topbar-gain = Guadagno
topbar-plutosdr-s-own-agc-setting =


    The PlutoSDR's own AGC is setting this gain, so moving the slider does nothing. Set Settings → Radio → AGC to Manual to control it here.
topbar-agc-hang-time-click-cycle = Tempo di tenuta dell'AGC — clicca per scorrere: Off / Lento / Medio / Veloce
topbar-man = Man
topbar-manual-audio-gain-used-while = Guadagno audio manuale, usato mentre l'AGC è disattivato. Parte dal livello che l'AGC stava mantenendo quando è stato disattivato.
topbar-radio-s-own-squelch-sent = Lo squelch proprio della radio, inviato sul collegamento di controllo. È il cancello attraverso cui passa davvero l'audio — quello software si chiuderebbe solo di più su ciò che l'apparato ha già lasciato passare.
topbar-max-gain = guadagno massimo
topbar-manual-gain = guadagno manuale
topbar-noise-reduction = riduzione del rumore
topbar-impulse-noise-blanker = Limitatore di rumore impulsivo
topbar-auto-notch-cancel-constant-tone = Notch automatico: cancella le componenti a tono costante (battimenti)
topbar-binaural-audio-not-while-sub = Audio binaurale — non mentre il sub ricevitore ha l'orecchio destro
topbar-binaural-audio-passband-spread-across = Audio binaurale: la banda passante è distribuita sulle due orecchie, così i segnali a tonalità diverse arrivano da direzioni diverse e quello che sintonizzi scorre da un lato all'altro. Clicca per tornare al mono
topbar-binaural-cw-spread-passband-across = CW binaurale: distribuisce la banda passante sulle due orecchie, così i segnali a tonalità diverse arrivano da direzioni diverse — un pile-up diventa più posti invece di una sola nota affollata, e sintonizzare una stazione la fa scorrere. Meglio con le cuffie
topbar-binaural-audio-spread-passband-across = Audio binaurale: distribuisce la banda passante sulle due orecchie. Nella voce il rumore si spande sull'intera immagine mentre la stazione resta al centro, il che è più facile da ascoltare per un'ora — a costo che la voce stessa risulti spanta. Meglio con le cuffie
topbar-mute = SILENZIO
topbar-record-audio-raw-i-q = Registra l'audio, l'I/Q grezzo, oppure entrambi
topbar-wfm-stereo-forced-off-click = Stereo WFM forzato disattivato — clicca per lo stereo automatico
topbar-wfm-stereo-pilot-locked-click = Stereo WFM: pilota agganciato. Clicca per forzare il mono
topbar-wfm-stereo-automatic-no-pilot = Stereo WFM: automatico, nessun pilota su questa stazione
topbar-rds-data-arriving-click-see = I dati RDS stanno arrivando — clicca per vederli
topbar-rds-no-data-station-click = RDS — nessun dato su questa stazione. Clicca comunque per la diagnostica del decodificatore
topbar-drm-decoding-click-details = DRM in decodifica — clicca per i dettagli
topbar-drm-click-decoder-s-state = DRM — clicca per lo stato del decodificatore
topbar-hd-radio-decoding-click-details = HD Radio in decodifica — clicca per i dettagli
topbar-hd-radio-click-decoder-s = HD Radio — clicca per lo stato del decodificatore
topbar-presets = PRESET
topbar-width-per-sideband = LARGHEZZA PER BANDA LATERALE
topbar-width = LARGHEZZA
topbar-passband-width-hertz-type-drag = Larghezza della banda passante in hertz — scrivila, oppure trascina. Clicca nel campo per inserire un valore esatto.
topbar-edges = BORDI
topbar-low-edge-hz-carrier = Bordo inferiore, in Hz dalla portante
topbar-high-edge-hz-carrier = Bordo superiore, in Hz dalla portante
topbar-noise-reduction-voice-off-click = Riduzione del rumore (voce), disattivata — clicca per scegliere un motore
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = Ciò che senti, in MP3: il ricevitore su un canale e la tua trasmissione sull'altro. È la registrazione di un QSO.
topbar-mono = MONO
topbar-mixed-one-channel-click-two = Miscelato su un canale — clicca per due
topbar-two-channels-rx-left-tx = Due canali: RX a sinistra, TX a destra — clicca per un canale miscelato
topbar-quick-clip = CLIP RAPIDA
topbar-stop-after = FERMA DOPO
topbar-no-stop = nessun arresto
topbar-auto-record = REGISTRAZIONE AUTOMATICA
topbar-set-squelch-auto-record-needs = Imposta uno squelch — la registrazione automatica ne ha bisogno per riconoscere il silenzio
topbar-auto-record-needs-sdr-front = La registrazione automatica richiede un front-end SDR con squelch software
topbar-stop-following-squelch = Smetti di seguire lo squelch
topbar-waiting-signal = in attesa di un segnale
topbar-record-spectrum = REGISTRA SPETTRO
topbar-i-q-wav = ● WAV I/Q
topbar-i-q-wav-2 = WAV I/Q
topbar-raw-spectrum-receiver-delivering-stereo = Lo spettro grezzo che il ricevitore fornisce, come WAV stereo a 32 bit float (RF64 oltre 4 GB) che SDR#, SDRuno, HDSDR e SDRangel aprono — e che sdroxide stesso riproduce con --file. Pesante: 8 byte per campione.
topbar-radio-hands-over-demodulated-audio = Questa radio consegna audio demodulato, quindi non c'è alcun I/Q da registrare.
topbar-what-capture-costs-sample-rate = Quanto costa una cattura a questa frequenza di campionamento
topbar-engine = MOTORE
topbar-no-noise-reduction-decoders-never = Nessuna riduzione del rumore — i decodificatori non la vedevano comunque
topbar-strength = INTENSITÀ
topbar-tone = TONO
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS — nessun tono subaudibile su questo segnale
topbar-tone-squelch = SQUELCH A TONO
topbar-carrier-squelch-open-any-signal = Squelch di portante: si apre con qualsiasi segnale
topbar-require-tone-currently-being-received = Richiedi il tono attualmente ricevuto
topbar-ctcss = CTCSS
topbar-any-dcs = QUALSIASI DCS
topbar-open-any-dcs-coded-signal = Si apre con qualsiasi segnale codificato DCS. Quale dei 104 codici trasporti non si può leggere in modo affidabile qui, quindi non c'è nulla di più fine da scegliere
topbar-which-socket-back-radio-receiving =
    Quale presa sul retro la radio sta ricevendo — il suo comando ANT, la stessa impostazione del pulsante ANT sul pannello frontale. Clicca per passare alla successiva.

    The choice is remembered per band, and put back the next time the dial crosses into that band.
topbar-radio-s-separate-receiving-antenna =
    L'antenna di ricezione separata della radio, inserita nel percorso di ricezione o esclusa da esso — la sua impostazione RX ANT propria. L'antenna sulla presa principale resta in trasmissione in ogni caso.

    The radio remembers this per band itself, so sdroxide reads it back after every band change rather than putting back what it last saw: clicking here is the only thing that moves it.
topbar-radio-s-own-power-switch =
    L'interruttore di alimentazione proprio della radio, tramite il collegamento di controllo, ed è l'unico vero on/off del programma — non l'interruttore LINK di sdroxide, che chiude la parte di sdroxide e lascia la radio accesa.

    For ON to reach anything the radio's control end has to stay awake while it is off: Network Control over the LAN, or a CI-V port still fed from the mains on a set switched off at the front.
topbar-switch-radio = Accendi la radio
topbar-switch-radio-off-audio-meters = Spegni la radio. Audio e misuratori si fermano con essa.
topbar-what-second-aerial-click-swap =
    A cosa serve la seconda antenna — clicca per scambiare.

    CANCEL subtracts it from the first, in the gain, phase and delay that make a local noise source line up on both: the DSP form of a noise-cancelling phaser. COMBINE adds the two in the phase that reinforces, weighted by how well each hears — diversity reception, which fills in fades.
topbar-hold = BLOCCA
topbar-stop-filter-moving-reach-moment = Ferma il movimento del filtro. Raggiungilo nel momento in cui compare un nullo: un filtro lasciato in adattamento si rimirerà verso qualunque cosa diventi più forte, che su una banda tranquilla è la stazione che stai ascoltando.
topbar-restart = RIAVVIA
topbar-zero-filter-find-null-again = Azzera il filtro e ritrova il nullo.
topbar-adapt = Adatta
topbar-how-fast-filter-chases-slow = Quanto in fretta il filtro insegue: lento e costante a sinistra, converge in meno di un secondo e a destra si vede che cerca. Parti veloce per trovare il nullo, poi BLOCCALO.
topbar-where-sub-receiver-listens-shift = Dove ascolta il sub ricevitore. Maiusc-clic sulla waterfall, oppure trascina dentro la banda passante del sub, per spostarlo.
topbar-dial = ←SINTONIA
topbar-move-sub-receiver-main-dial = Sposta il sub ricevitore sulla sintonia principale
topbar-dial-2 = SINTONIA←
topbar-move-main-dial-sub-receiver = Sposta la sintonia principale sul sub ricevitore
topbar-filter = Filtro
topbar-sub-receiver-passband-edges-hz = Bordi della banda passante del sub ricevitore, in Hz
topbar-sub-receiver-level-plays-right = Livello del sub ricevitore (suona nell'orecchio destro)
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = Manipolatore vocale: registra e trasmetti i messaggi memorizzati
topbar-drive = Pilotaggio
topbar-tune-2 = Accordo
topbar-mic = Mic
topbar-cessb = CESSB
topbar-microphone-gain = Guadagno del microfono
topbar-deviation-how-far-mode-s =
    Deviazione: di quanto la trasmissione di questo modo fa oscillare una radio che la modula da sé. Un trasmettitore FM converte il livello audio in oscillazione di frequenza e non ha un ALC che lo fermi, quindi il fondo scala verso un ingresso dati impostato per la voce supera la deviazione — e suona del tutto normale a chi ascolta, e non viene decodificato da nessuno.

    Kept per mode, so a deviation set for 1200 baud packet never lands on FT8.
topbar-transmit-audio-how-hard-mode =
    Audio in trasmissione: quanto forte questo modo pilota il modulatore di una radio che modula ciò che le inviamo — un apparato CAT sulla sua scheda audio, un FLEX, un Icom sulla sua porta di rete. Abbassalo finché l'ALC dell'apparato a malapena si muove, e imposta la potenza sulla radio; un ALC che lavora su un modo digitale a inviluppo costante è ciò che produce gli splatter.

    Drive is not this control: on these radios Drive reaches the rig's power register and never touches its audio.

    Kept per mode — FT8, RTTY, PSK and MCW each keep their own.
topbar-tx-audio = Audio TX
topbar-cw-psk-rtty-skimmers-decode = Skimmer CW / PSK / RTTY — decodificano i segnali su tutta la banda e li segnano sulla waterfall
topbar-skimmers = SKIMMER
topbar-run-skimmer = Avvia questo skimmer
topbar-minimum-snr-decoded-signal-needs = SNR minimo che un segnale decodificato deve avere per essere segnalato
topbar-needs-wideband-iq-source = richiede una sorgente I/Q a banda larga
topbar-cw-decoder = DECODIFICATORE CW
topbar-how-many-signals-model-reads = Quanti segnali il modello legge contemporaneamente. Gli altri mantengono il loro marcatore ma non portano testo.
topbar-spots = SPOT
topbar-save-skimmer-s-spot-list = Salva l'elenco spot dello skimmer su un file
topbar-show-hide-full-band-waterfall = Mostra/nascondi la striscia della waterfall a banda piena sopra il panadapter — tutto ciò che questo ricevitore può vedere in una volta
topbar-spectrum-waterfall-either-layer-both = Spettro e waterfall — uno dei due livelli, entrambi, o nessuno. Acceso quando entrambi sono visibili.
topbar-spectrum = SPETTRO
topbar-show-spectrum = MOSTRA SPETTRO
topbar-draw-spectrum-line-across-top = Disegna la linea dello spettro in cima al panadapter. Disattivata, la waterfall prende tutta l'altezza.
topbar-peak-hold = PEAK HOLD
topbar-trace-highest-level-each-column = Traccia il livello più alto che ogni colonna ha raggiunto sulla linea in tempo reale, decadendo di nuovo — ciò che la banda ha fatto mentre guardavi altrove
topbar-draw-spectrum-receding-surface-instead = Disegna lo spettro come una superficie che si allontana anziché come una linea piatta: lo spettro più recente in primo piano, quelli precedenti che scorrono via da te. Gli ultimi due secondi della banda come un paesaggio — una portante che va e viene è una cresta anziché una linea che si agita. Griglia e peak hold appartengono alla linea piatta e su di essa non vengono disegnati.
topbar-how-quickly-spectrum-line-follows = Quanto velocemente la linea dello spettro segue la banda. Più lento media più fotogrammi l'uno nell'altro: una linea più stabile, e una portante debole che resta ferma abbastanza a lungo da leggerla. La waterfall non ne è toccata — quelle righe ricevono ogni fotogramma comunque.
topbar-show-waterfall = MOSTRA WATERFALL
topbar-draw-scrolling-waterfall-below-spectrum = Disegna la waterfall che scorre sotto lo spettro. Disattivata, la linea dello spettro prende tutta l'altezza.
topbar-how-fast-waterfall-scrolls-lines = Quanto velocemente scorre la waterfall, in righe al secondo: Lento 5, Medio 28, Veloce 56, Più veloce 112, Velocissimo 224. Il motore li scandisce da sé, quindi i due più veloci sono dettaglio reale anziché la stessa riga disegnata due volte — nei limiti in cui il ricevitore può alimentarli: una riga non può mostrare più di una trasformata, e un front-end stretto ne produce solo qualche decina al secondo. Costano storia, perché la waterfall conserva un numero fisso di righe — 73 secondi a Medio, 9 a Velocissimo.
topbar-decode-labels = ETICHETTE DI DECODIFICA
topbar-mark-every-decoded-station-waterfall = Segna sulla waterfall ogni stazione decodificata con il suo nominativo, alla frequenza su cui è stata sentita — FT8, FT4 e gli altri modi a slot. Un buon inizio ne mette trenta sull'ampiezza due volte al minuto, sopra le tracce che stai leggendo; disattivandole la waterfall è soltanto la banda. L'elenco delle decodifiche accanto ha comunque tutti i nominativi.
topbar-switch-3d-set = Attiva il 3D per impostare questo
topbar-how-3d-spectrum-drawn-only = Come viene disegnato lo spettro 3D. Cambia solo la forma — entrambe le resa mostrano gli stessi spettri, e nascondono entrambe ciò che sta dietro, quindi un segnale forte in primo piano sovrasta la banda che copre.
topbar-lines = LINEE
topbar-one-trace-per-remembered-spectrum = Una traccia per ogni spettro ricordato, nel colore della linea piatta, ciascuna che nasconde quelle dietro di sé. La forma senza i livelli, e la lettura da scegliere quando la forma è ciò che cerchi — costa un po' di più di quella piena anziché meno, perché disegna la stessa superficie e poi traccia ogni cresta sopra di essa.
topbar-solid = PIENO
topbar-filled-surface-coloured-waterfall-s = Una superficie piena colorata con la palette della waterfall, così il livello sta nel colore oltre che nell'altezza e le due metà del panadapter concordano su come appare un segnale forte. Cambia la palette in Impostazioni › Display.
topbar-how-fast-3d-spectrum-flows = Quanto velocemente lo spettro 3D scorre via da te, in righe al secondo: Lento 6, Medio 12, Veloce 24, Più veloce 48. La superficie ha una profondità fissa di righe, quindi è anche quanto tempo trattiene. Più lento significa memoria più lunga e una superficie che striscia; più veloce una più corta che si muove. La waterfall mantiene la propria velocità di scorrimento, più sotto.
topbar-how-many-columns-panadapter-waterfall = Quante colonne vengono usate per disegnare il panadapter e la sua waterfall. Auto legge il renderer di questa macchina e la dimensione del panadapter e sceglie il massimo che può reggere — uno schermo 4K ne vuole 4096. Ogni colonna è un byte in ogni fotogramma, quindi un client collegato a una stazione remota paga il dettaglio sul proprio collegamento: 4096 colonne a 60 fps sono circa un quarto di megabyte al secondo, il doppio della larghezza standard.
topbar-read-machine-screen-take-most = Legge questa macchina e questo schermo, prende il massimo che possono reggere, e li segue se cambiano
topbar-width-force-right-now-whatever = La larghezza in vigore in questo momento, qualunque cosa chieda la riga sopra
topbar-keep-floor-ceiling-set-best = Mantieni il pavimento/soffitto impostati per il miglior contrasto della waterfall — riassestati dopo un cambio di banda, dopo uno spostamento o uno zoom, e quando i livelli derivano. Attivalo per adattare subito; disattivalo per tenere i livelli dove li hai impostati.
topbar-keep-tuned-frequency-middle-panadapter = Mantieni la frequenza sintonizzata al centro del panadapter: la finestra scorre sotto la sintonia anziché far saltare l'immagine di un'intera ampiezza quando sintonizzi oltre il bordo. Attivalo per centrare subito; disattivalo per spostare e zoomare dove vuoi.
topbar-ceil = soffitto{" "}
topbar-fft-size = DIMENSIONE FFT
topbar-scroll-waterfall-upwards-newest-row = Fai scorrere la waterfall verso l'alto — riga più recente in basso (V)
topbar-logbook-all-qsos-digital-manual = Logbook — tutti i QSO (digitali + manuali)
topbar-award-tracking-dxcc-was-waz = Monitoraggio premi — DXCC / WAS / WAZ / locatori
topbar-band-conditions-published-forecast-beside = Condizioni di banda — la previsione pubblicata accanto a ciò che è stato davvero sentito su ciascuna banda
topbar-satellite-qo-100-beacon-calibration = Satellite — la calibrazione con il radiofaro QO-100 è in corso
topbar-satellite-doppler-tracking-qo-100 = Satellite — inseguimento Doppler, e la calibrazione con il radiofaro QO-100
topbar-ism-band-devices-decoding-now = Dispositivi in banda ISM — in decodifica ora, che la finestra sia aperta o meno. Disattivalo con DECODIFICA dentro la finestra.
topbar-ism-band-devices-weather-sensors = Dispositivi in banda ISM — sensori meteo, contatori e domotica sentiti intorno a te
topbar-public-sdrs-internet-browse-kiwisdr = SDR pubblici su internet — sfoglia le directory di KiwiSDR e SpyServer e aprine uno come radio
topbar-winlink-radio-email = Posta radio Winlink
topbar-memory-channels = Canali di memoria
topbar-scanner-stopped-signal = Scanner — fermo su un segnale
topbar-scanner-running = Scanner — in funzione
topbar-scan-memory-channels-frequency-range = Scansiona canali di memoria o un intervallo di frequenze
topbar-grid-tracker-worked-maidenhead-squares = Tracciatore di locatori — quadrati Maidenhead lavorati su una mappa, con ciò che viene sentito
topbar-settings-device-gains-antennas-audio = Impostazioni — guadagni del dispositivo, antenne, dispositivi audio
topbar-user-manual-f1 = Manuale utente (F1)
topbar-range-was-typed-settings-radio = Quell'intervallo è stato scritto in Impostazioni ▸ Radio ▸ Gamma RX, non riportato dalla radio. Allargalo, oppure svuota la casella per usare ciò che il dispositivo riferisce di sé.
topbar-what-device-reports-about-itself = È ciò che il dispositivo riferisce di sé. Se la radio copre davvero questa banda, indica il suo intervallo reale in Impostazioni ▸ Radio ▸ Gamma RX.
topbar-band = BANDA
topbar-mode = MODO

## Top bar: the System box's window chips.

topbar-chip-log = LOG
topbar-chip-awards = PREMI
topbar-chip-bands = BANDE
topbar-chip-sat = SAT
topbar-chip-ism = ISM
topbar-chip-mail = POSTA
topbar-chip-mem = MEM
topbar-chip-scan = SCAN
topbar-chip-hfdl = HFDL
topbar-chip-grid = LOCAT.

## Top bar: the mode-defaults chip.

topbar-changed-agc = AGC
topbar-changed-squelch = squelch
topbar-changed-auto-notch = notch automatico
topbar-changed-stereo = stereo
topbar-changed-binaural = binaurale
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    Torna ai valori predefiniti di { $mode }. Modificati rispetto ad essi: { $changed }.

    Click to put them back. { $mode }'s own values return, and what you set here is forgotten.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = la radio non ha un'impostazione a { $baud } baud — verrà usato { $default }
settings-radio-icom-scope-hint =
    Trasmette lo sweep dello scope proprio della radio sul collegamento CI-V e lo disegna come panadapter — l'unica immagine della banda che un apparato con audio demodulato può dare, dato che l'audio che invia è ciò che è già passato dal suo filtro.

    Needs the radio set up for it: CI-V USB Baud Rate to 115200 and CI-V USB Port to "Unlink from {"["}REMOTE]" (both under SET > Connectors > CI-V), and the baud rate above set to match — below { $min_baud } the sweeps do not fit down the link and the scope stays off.

    The sweeps share the radio's internal USB bus with its own sound card. If received audio starts to drop out with the scope on, this box is the first thing to try turning off.
settings-radio-rx-tx-words = { $rx } / TX { $tx }
settings-radio-soapy-native = L'hardware qui sopra è supportato direttamente da sdroxide: { $names }. Selezionando quell'interfaccia qui sopra si ottengono le sue impostazioni proprie — stadi di guadagno, filtri e notch che SoapySDR non può esprimere — e non serve alcun modulo SoapySDR.
settings-radio-rx888-width-hint = Quanto dello spettro digitalizzato mostra il panadapter in una volta. L'intera catena DSP lavora a questa larghezza, quindi una larghezza maggiore costa proporzionalmente più CPU — 1/2 è l'intera banda nella waterfall, e una quantità seria di calcoli. Sopra il punto di incrocio VHF ({ $crossover } MHz) il panadapter non guarda l'antenna ma la IF del sintonizzatore, che è larga 8 MHz e fissa a { $if_centre } MHz — quindi nulla più largo di { $widest } può esservi centrato, e la larghezza in più è spettro morto accanto al segnale anziché altro spettro.
settings-radio-sdrplay-serial-missing = Il seriale { $serial } non è tra i ricevitori che il servizio SDRplay riporta. Scegline uno dei ricevitori elencati, oppure ricollega questo e premi Riesamina.
settings-radio-hdr-filter-hint =
    Il filtro analogico davanti al percorso HDR — un controllo diverso dalla larghezza di banda propria del ricevitore. Ogni impostazione è costruita su un insieme fisso di centri e non può fare nulla altrove; questa è costruita su { $centres }. Ha effetto subito.

    The path itself does not work yet — see the switch above — so this chooses a filter for a mode that is currently silent.
settings-radio-rspduo-only = Solo un RSPduo ha un secondo sintonizzatore, e il ricevitore selezionato qui è un { $model }, quindi non farà nulla.
settings-radio-lime-tx-gain-low = Il guadagno in trasmissione è { $db } dB, al fondo della sua gamma — pochi microwatt dalla scheda, che risulteranno nulla su un misuratore di potenza qualunque cosa ci sia a valle. È il predefinito perché un trasmettitore armato non ti sorprenda; alzalo prima di attivare.

## Settings window, General: SWR

settings-general-swr-tune-limit = (accordo: { $limit }:1)

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Click to cycle meter face: { $faces }
topbar-step-down = Giù { $step }
topbar-step-up = Su { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = Spostamento ripetitore { $shift } — in ricezione su { $rx }, in trasmissione su { $tx } MHz{ $auto }
topbar-tone-and-burst = Trasmissione di { $tone } sotto la voce, e un burst di { $ms } ms a 1750 Hz all'inizio di ogni trasmissione
topbar-tone-under-voice = Trasmissione di { $tone } sotto la voce
topbar-burst-every-over = Un burst di { $ms } ms a 1750 Hz all'inizio di ogni trasmissione
topbar-match-tx = CORRISPONDE TX ({ $tone })
topbar-decimation-on = Decimazione del front-end: al ricevitore viene data la parte centrale 1/{ $n } dei { $msps } Msps che questa radio trasmette — { $span } — e il resto viene scartato prima che una parte di esso raggiunga il ricevitore.
topbar-decimation-off = Decimazione del front-end, disattivata: il ricevitore vede tutti i { $span } che questa radio trasmette.
topbar-decimation-hint =
    { $what }

    A narrower span means finer waterfall resolution, a quieter noise floor (3 dB per halving) and less CPU — at the cost of the band either side of it. The dial still tunes anywhere; the radio moves its LO to follow.

    Click to cycle: off / 2 / 4 … { $max }.
topbar-rx-gain-hint = Guadagno RX del front-end ({ $stage }). Troppo satura l'ADC del ricevitore e spalma segnali spuri su tutta la banda; troppo poco e diventa sordo.
topbar-rx-gain-stages =
    {""}

    This rig has { $n } RX gain stages — the rest are in Settings → Device.
topbar-passband-now =
    {""}

    The passband is at { $dbfs } dBFS now.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    Fa passare l'audio solo sopra questa potenza nella banda passante di ricezione, in dBFS. A sinistra è aperto.{ $level } Impostalo sopra il rumore e sotto il segnale.

    A stream that arrives with the radio's own AGC already in it — an Icom's 12 kHz IF, for one — sits far higher on this scale than an SDR's raw baseband does, which is why the rail reaches full scale.

    Click to cycle the strength: Low / Med / High / Off. The engine is in the rows below.
topbar-recording-both = Registrazione di { $audio } e { $iq }
topbar-recording-audio = Registrazione dell'audio in { $audio }
topbar-recording-iq = Registrazione dell'I/Q in { $iq }
topbar-auto-record-armed = Registrazione automatica armata: un file per trasmissione, chiuso dopo { $secs } s di silenzio
topbar-rds-hover = RDS: { $ps }. Clicca per i dati della stazione
topbar-drm-hover = DRM: { $service }. Clicca per i dettagli della trasmissione
topbar-drm-no-codec = DRM: { $summary } — agganciato, ma il suo audio { $codec } non può essere decodificato qui. Clicca per sapere cosa manca
topbar-drm-state = DRM: { $summary } — clicca per lo stato del decodificatore
topbar-hd-hover = HD Radio: { $station }. Clicca per i dettagli della trasmissione
topbar-hd-state = HD Radio: { $summary } — clicca per lo stato del decodificatore
topbar-binaural-preset = { $preset } in ciascun orecchio — da { $lo } a { $hi } Hz, entrambe le bande laterali
topbar-nr-hover =
    Riduzione del rumore: { $strength } — { $engine }

    Click to change engine or strength
topbar-clipping = Ritaglio { $span } — premi di nuovo per ricominciarlo
topbar-record-clip = Registra { $span } e fermati — un ritaglio da allegare a un rapporto di ricezione
topbar-starting-clip = avvio di un ritaglio da { $span }…
topbar-stops-in = si ferma tra { $time }
topbar-auto-record-hint = Registra ogni trasmissione in un proprio file, chiuso dopo { $secs } secondi di silenzio
topbar-recording-silence = registrazione · { $secs } s di silenzio (si chiude a { $hold })
topbar-tone-open = In ricezione { $tone }, cioè lo squelch a tono — audio aperto
topbar-tone-closed = In ricezione { $heard }, ma lo squelch a tono vuole { $wanted } — l'audio resta chiuso
topbar-tone-heard = In ricezione CTCSS/DCS { $tone }
topbar-tone-nothing = Squelch a tono { $tone }: non viene ricevuto nulla che vi corrisponda
topbar-tone-use = USA { $tone }
topbar-sockets = Prese: { $sockets }
topbar-voice-keyer-tx = In trasmissione { $slot } — clicca per aprire il manipolatore vocale
topbar-drive-uncal-gen = Questa frequenza, che non appartiene ad alcuna banda radioamatoriale, non è calibrata, quindi l'impostazione raggiunge il trasmettitore per intero.
topbar-drive-uncal-band = { $band } non è calibrata, quindi l'impostazione raggiunge il trasmettitore per intero.
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = Questa frequenza, che non appartiene ad alcuna banda radioamatoriale, è calibrata { $trim } dB, quindi la stessa impostazione mette in aria { $diff } dB { $more ->
        [less] in meno
       *[more] in più
    } qui rispetto a una banda non calibrata.
topbar-drive-cal-band = { $band } è calibrata { $trim } dB, quindi la stessa impostazione mette in aria { $diff } dB { $more ->
        [less] in meno
       *[more] in più
    } qui rispetto a una banda non calibrata.
topbar-drive-hover =
    Quanto viene pilotato il trasmettitore. Su una radio che sdroxide modula da sé scala i campioni modulati; su un apparato con un proprio controllo di potenza è la frazione della potenza nominale che gli viene richiesta.

    { $trim }

    Settings → Radio → Transmit drive by band.
topbar-cessb-hover =
    SSB a inviluppo controllato: più potenza media a parità di picco, senza splatter — tre o quattro decibel di volume percepito dall'altro capo. Di quanti decibel la voce viene spinta nel processore; 0 è disattivato, 6 è un buon primo tentativo. Non tocca il Pilotaggio.

    Now: { $now }
topbar-waterfall-rate = { $rows } righe al secondo — { $seconds } secondi di banda
topbar-detail-auto-columns = { $n } colonne, qualunque cosa Auto avrebbe scelto qui
topbar-n-columns = { $n } colonne
topbar-fft-bins-whole = { $n } bin su tutto il flusso — uno per colonna, o più fini
topbar-fft-bins-pooled = { $n } bin raccolti nelle { $cols } colonne del panadapter: segnali più nitidi, non più segnali
topbar-sat-locked = Satellite — agganciato su { $name }
topbar-filter-edges = Da { $lo } a { $hi } Hz dalla portante
topbar-filter-isb = Filtro di ricezione: { $width } Hz per ciascun orecchio ({ $edges }). Le due bande laterali dell'ISB sono trasmissioni separate, quindi la larghezza è quella di una sola di esse.
topbar-filter-width = Filtro di ricezione: { $width } Hz di larghezza ({ $edges }).
topbar-filter-hint =
    { $what }

    Click for the width, the two edges and this mode's presets, as numbers — the panadapter's grips place a passband by eye, this is where an exact figure is typed.
topbar-band-outside-range =
    { $band } è fuori da ciò che questa radio riceve ({ $ranges } MHz).

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band }: { $verdict } ({ $time ->
        [day] di giorno
       *[night] di notte
    }) — previsione di HAMQSL.com dagli indici solari, non una misura del tuo percorso.

## Top bar: short words and captions

topbar-squelch-open = aperto
topbar-off = off
topbar-recording = registrazione
topbar-sql = sql
topbar-stations = stazioni
topbar-caption-reaction = reazione
topbar-caption-scroll = scorrimento
topbar-caption-surface = superficie
topbar-caption-flow = flusso
topbar-caption-detail = dettaglio
topbar-floor = pavimento

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = AEREI
pane-chart = GRAFICO
pane-chat = CHAT
pane-decodes = DECODIFICHE
pane-files = FILE
pane-heard = SENTITI
pane-image = IMMAGINE
pane-map = MAPPA
pane-messages = MESSAGGI
pane-monitor = MONITOR
pane-panel = PANNELLO
pane-qso = QSO
pane-reading = LETTURA
pane-receive = RICEZIONE
pane-saved = SALVATI
pane-send = INVIO
pane-spots = SPOT
pane-stations = STAZIONI
pane-status = STATO
pane-terminal = TERMINALE
pane-text = TESTO
pane-traffic = TRAFFICO
pane-vessels = NAVI
pane-wfall = WFALL

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}SALVA{" "}
panels-nothing-decoded-save = Niente da salvare: nulla è stato decodificato
panels-clear-rx = {" "}CANCELLA RX{" "}
panels-empty-receive-window-nothing-air = Svuota la finestra di ricezione. Nulla di ciò che è in aria si ferma.
panels-nothing-received-clear = Niente da cancellare: nulla è stato ricevuto
panels-save-what-panel-has-decoded = Salva su un file ciò che questo pannello ha decodificato
panels-prop = PROP
panels-shade-map-where-signals-are = Colora la mappa secondo dove i segnali stanno davvero arrivando, su ciascuna banda. Ogni ricezione è posta al punto medio del suo percorso — la porzione di ionosfera che lo ha piegato — non alla stazione lontana, quindi questa è una mappa del cielo anziché di dove vivono i radioamatori. È costruita dai percorsi a cui questa stazione ha preso parte, più — quando la Reverse Beacon Network è attiva in Impostazioni → Spot — ciò che gli skimmer del mondo stanno sentendo, che copre le bande su cui questa radio non è.
panels-night = NOTTE
panels-shade-where-sun-down-twilight = Colora dove il Sole è tramontato, e il crepuscolo tra i due, così la linea grigia compare sulla mappa. Le bande basse si allungano e quelle alte si chiudono dal lato notturno, e il terminatore stesso è dove sta il DX.
panels-all-bands = TUTTE LE BANDE
panels-every-band-once-one-hue = Ogni banda insieme, una tinta ciascuna — condizioni complessive.
panels-one-band = UNA BANDA
panels-one-band-cold-hot-blue = Una banda, da freddo a caldo: blu, verde, giallo, rosso.
panels-frequency-you-are-actually-working = La frequenza su cui stai davvero lavorando: la sintonia più l'offset di tono accanto. È ciò che va nel log — la sola sintonia risulta bassa di tanto.
panels-send-return = INVIA AL RITORNO
panels-freq = ⇵ FREQ
panels-nothing-agreed-mode-save-your = Nessuna frequenza concordata per questo modo — salva la tua qui sotto.
panels-forget-saved-frequency = Dimentica questa frequenza salvata
panels-dial-already-one-your-saved = Questa sintonia è già una delle tue frequenze salvate.
panels-dial-already-agreed-list-mode = Questa sintonia è già nell'elenco concordato per questo modo.
panels-decode-squelch-raise-stop-decoding = Squelch di decodifica — alzalo per smettere di decodificare il rumore
panels-message-repeats-long-transmit-held = Il messaggio si ripete finché la trasmissione è tenuta premuta — una meteora cattura la parte di esso che sta passando.
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}TX ON{"  "}
panels-tx-2 = {"   "}TX{"   "}
panels-call-cq = {" "}CHIAMA CQ{" "}
panels-nocall = NOCALL

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = Sentito troppo poco parlato — parla normalmente per tutto il conteggio e riprova.
txaudio-tx-audio = TX Audio
txaudio-mode-not-voice-mode-so = Questo modo non è un modo vocale, quindi ciò che invia non passa per la catena. Le impostazioni fatte qui hanno effetto in SSB, AM e FM.
txaudio-bypass = ESCLUDI
txaudio-switch-every-stage-out-once = Escludi tutti gli stadi in una volta, compreso il riduttore di rumore, così il microfono esce non elaborato. Clicca di nuovo per ripristinare esattamente gli stadi che erano attivi — il confronto A/B più rapido di ciò che sta facendo la catena.
txaudio-denoise = RIDUCI RUMORE
txaudio-rnnoise-microphone-ahead-every-other = RNNoise sul microfono, prima di ogni altro stadio, così il rumore dell'ambiente viene rimosso prima che gate o compressore possano alzarlo. Aggiunge circa 10 ms di ritardo.
txaudio-digital-voice = VOCE DIGITALE
txaudio-also-run-freedv-rade-digital = Fai passare anche la voce digitale FreeDV / RADE nella catena. Disattivata di default: un vocoder è addestrato sul parlato naturale e potrebbe decodificare peggio una voce molto elaborata, quindi provalo in aria prima di affidarti ad esso. L'audio TCI e i modi digitali non passano mai per la catena.
txaudio-test = Prova
txaudio-rec = {" "}⏺ REC{" "}
txaudio-play = {" "}▶ RIPRODUCI{" "}
txaudio-play-test-phrase-through-chain = Riproduci la frase di prova attraverso la catena com'è impostata ora, sugli altoparlanti. Regola mentre suona e senti il cambiamento. L'audio in ricezione viene sostituito mentre suona.
txaudio-recording = registrazione…
txaudio-no-phrase-yet = nessuna frase ancora
txaudio-live = Dal vivo
txaudio-live-2 = {" "}🎧 DAL VIVO{" "}
txaudio-hear-your-microphone-through-chain = Senti il tuo microfono attraverso la catena mentre parli, senza attivare il trasmettitore. Usa le cuffie: attraverso gli altoparlanti si auto-alimenta. Ti senti con un ritardo di qualche decina di millisecondi, il che rende difficile parlare — regola i livelli sugli strumenti, e giudica il suono con REC / RIPRODUCI. L'audio in ricezione viene sostituito mentre questo è in funzione; premere PTT lo ferma.
txaudio-meters = {" "}STRUMENTI{" "}
txaudio-run-your-microphone-through-chain = Fai passare il microfono attraverso la catena solo per gli strumenti: niente sugli altoparlanti, quindi nessun ritardo su cui parlare e nulla da far riecheggiare. È il modo di impostare il guadagno del microfono e dell'ingresso, la soglia del gate e quanto lavora il compressore. L'audio in ricezione prosegue; premere PTT lo ferma.
txaudio-monitoring-live-use-headphones-speakers = Monitoraggio in diretta — usa le cuffie, altrimenti gli altoparlanti rientreranno nel microfono.
txaudio-voice-modes-only-tci-clients = Solo modi vocali. I client TCI e i modi digitali saltano la catena; quando è in uso, tieni piatti l'equalizzatore e il processore TX propri della radio.
txaudio-auto-set-stopped-microphone-was = Imposta automatico interrotto: il microfono è stato occupato.
txaudio-microphone-may-clipping-before-gets = {" "}Il microfono potrebbe andare in saturazione prima di arrivare qui — abbassa il suo guadagno proprio se suona aspro.
txaudio-level = Livello
txaudio-mic-control-same-one-transmit = Il comando Mic — lo stesso della barra di trasmissione, condiviso da ogni profilo. 0 dB al centro, +6 dB in cima.
txaudio-mic = Mic
txaudio-gain-top-mic-microphone-stored = Guadagno aggiunto a Mic per questo microfono, memorizzato con il profilo. Le soglie dei preset presuppongono una voce che raggiunge circa -10 dBFS sullo strumento IN; Imposta automatico la porta lì. Resta dentro sotto ESCLUDI, e vale anche per la voce digitale: è livello, non elaborazione.
txaudio-input = Ingresso
txaudio-auto-set = Imposta automatico
txaudio-auto-set-cancelled = Imposta automatico annullato.
txaudio-talk-now-you-would-air = Parla ora, come faresti in aria…
txaudio-db = -∞ dB
txaudio-chain = Catena
txaudio-order-audio-meets-them = In ordine, l'audio li incontra
txaudio-switch-stage = Attiva questo stadio
txaudio-earlier-chain = Più a monte nella catena
txaudio-later-chain = Più a valle nella catena
txaudio-output-limiter = Il limitatore d'uscita
txaudio-output = Uscita
txaudio-trim-limiter-always-last = Correzione e limitatore — sempre ultimi
txaudio-profiles = Profili…
txaudio-gate-expander = Gate / espansore
txaudio-pushes-room-noise-fan-rumble = Abbassa il rumore dell'ambiente, il ronzio della ventola e il sibilo nelle pause. Imposta la soglia appena sopra il rumore di fondo e sotto le sillabe più deboli.
txaudio-gate = GATE
txaudio-hard-gate-10-1-down = Gate netto: 10:1, fino a -40 dB
txaudio-expander = ESPANSORE
txaudio-gentle-expander-2-1-never = Espansore dolce: 2:1, mai oltre -15 dB — la stanza non diventa mai morta
txaudio-reset = AZZERA
txaudio-back-defaults = Torna ai valori predefiniti
txaudio-threshold = Soglia
txaudio-below-gate-starts-close = Sotto questo valore il gate inizia a chiudersi.
txaudio-return = Ritorno
txaudio-hysteresis-once-open-closes-only = Isteresi: una volta aperto, si chiude solo questo tanto sotto la soglia, così una voce che le sta intorno non fa vibrare il gate.
txaudio-hold = Tenuta
txaudio-how-long-stays-open-after = Quanto resta aperto dopo che la voce cala, così le finali delle parole non vengono tagliate.
txaudio-release = Rilascio
txaudio-how-fast-closes-once-hold = Quanto in fretta si chiude una volta finita la tenuta. Allungalo se il gate pompa.
txaudio-floor = Pavimento
txaudio-most-ever-turns-pauses-down = Il massimo di abbassamento che applica alle pause.
txaudio-ratio = Rapporto
txaudio-how-steeply-closes-below-threshold = Quanto ripidamente si chiude sotto la soglia.
txaudio-look-ahead = Anticipo
txaudio-opens-much-before-word-starts = Si apre questo tanto prima che inizi una parola, così la sua prima consonante non viene tagliata. Aggiunge lo stesso ritardo a tutto ciò che viene trasmesso.
txaudio-high-pass-low-shelf-two = Passa-alto, shelf dei bassi, due picchi, shelf degli alti e passa-basso. Trascina un punto sulla curva, oppure impostalo qui sotto. Imposta il passa-basso sulla larghezza del tuo filtro di trasmissione.
txaudio-flat = PIATTO
txaudio-every-band-back-0-db = Ogni banda riportata a 0 dB, entrambi i bordi disattivati
txaudio-freq = Freq
txaudio-gain = Guadagno
txaudio-q-slope = Q / Pendenza
txaudio-high-pass = Passa-alto
txaudio-removes-handling-noise-breath-pops = Elimina il rumore di manipolazione, gli scoppi del respiro e il ronzio sotto l'angolo.
txaudio-low-shelf = Shelf dei bassi
txaudio-body-mud-cut-little-here = Corpo, oppure fango: taglia un po' qui per una voce che suona scatolata.
txaudio-peak-1 = Picco 1
txaudio-bump-dip-voice-band-often = Una gobba o un avvallamento nella banda vocale — spesso un taglio intorno a 300–500 Hz.
txaudio-peak-2 = Picco 2
txaudio-presence-lift-around-2-3 = Presenza: un'esaltazione intorno a 2–3 kHz è ciò che emerge in SSB.
txaudio-high-shelf = Shelf degli alti
txaudio-brightness = Brillantezza.
txaudio-low-pass = Passa-basso
txaudio-stops-energy-above-transmit-filter = Impedisce che l'energia oltre il filtro di trasmissione venga spesa nel modulatore.
txaudio-t-24-db-oct = 24 dB/ott
txaudio-t-12-db-oct = 12 dB/ott
txaudio-click-steeper-gentler-slope = Clicca per una pendenza più ripida o più dolce
txaudio-de-esser = De-esser
txaudio-turns-down-only-sibilant-band = Abbassa solo la banda sibilante quando "s" e "sh" superano la soglia, così il resto della voce mantiene il suo livello. Trova la frequenza spazzandola mentre dici "sister silver cellophane" e fermati dove il sibilo è più forte.
txaudio-frequency = Frequenza
txaudio-centre-sibilant-band = Centro della banda sibilante.
txaudio-width-band-higher-narrower = Larghezza della banda — più alto è più stretta.
txaudio-band-level-where-reduction-starts = Livello della banda da cui inizia la riduzione. Troppo basso e lavora sempre, smorzando la voce.
txaudio-amount = Quantità
txaudio-most-band-ever-turned-down = Il massimo di abbassamento della banda. Parti intorno a -6 dB.
txaudio-attack = Attacco
txaudio-how-fast-reacts = Quanto velocemente reagisce.
txaudio-how-fast-lets-go = Quanto velocemente lascia andare.
txaudio-compressor = Compressore
txaudio-brings-quiet-syllables-up-holds = Alza le sillabe deboli e trattiene quelle forti. Punta a 3–6 dB di riduzione sui picchi; più di 10 dB è compressione evidente, più adatta alle gare.
txaudio-level-above-which-starts-compress = Livello oltre il quale inizia a comprimere.
txaudio-how-hard-compresses-above-threshold = Quanto comprime sopra la soglia.
txaudio-how-fast-reacts-peak-10 = Quanto velocemente reagisce a un picco. 10–25 ms mantiene le consonanti nitide.
txaudio-how-fast-recovers = Quanto velocemente si riprende.
txaudio-knee = Ginocchio
txaudio-soft-knee-width-0-hard = Larghezza del ginocchio morbido; 0 è un ginocchio netto.
txaudio-makeup = Recupero
txaudio-gain-after-compression-bring-level = Guadagno dopo la compressione, per riportare su il livello.
txaudio-drive = Pilotaggio
txaudio-gain-ahead-compressor-so-more = Guadagno prima del compressore, così più voce supera la soglia — alza la potenza media. Usalo insieme al limitatore d'uscita.
txaudio-phase = Fase
txaudio-all-pass-phase-rotator-ahead = Rotatore di fase passa-tutto prima del rivelatore. Livella i picchi sbilanciati che ha una voce, così lo stesso livello di picco porta più potenza media. 4 è un'impostazione comune in radiodiffusione; 0 è disattivato.
txaudio-last-stage-trim-look-ahead = L'ultimo stadio: una correzione, e un limitatore con anticipo che tiene ogni picco sotto il soffitto senza lo splatter che produce il ritaglio netto. Disattivato di default, così accendere la catena non cambia un livello di pilotaggio che hai già impostato.
txaudio-limiter = Limitatore
txaudio-on = Acceso
txaudio-ceiling = Soffitto
txaudio-no-peak-leaves-chain-above = Nessun picco esce dalla catena sopra questo valore.
txaudio-trim = Correzione
txaudio-output-gain-ahead-limiter = Guadagno d'uscita, prima del limitatore.
txaudio-off = {"  "}off
txaudio-meters-move-while-you-transmit = Gli strumenti si muovono mentre trasmetti una voce, riproduci la frase di prova, oppure usi DAL VIVO o STRUMENTI.
txaudio-profiles-2 = Profili
txaudio-profile-whole-chain-every-stage = Un profilo è l'intera catena: le impostazioni di ogni stadio, quali sono attivi, il loro ordine e il guadagno d'ingresso per il tuo microfono. Caricarne uno sostituisce la catena; il comando Mic e il pilotaggio restano come sono.
txaudio-name = Nome
txaudio-save = Salva
txaudio-store-chain-now-under-name = Memorizza la catena così com'è ora con questo nome, sostituendo un profilo con lo stesso nome
txaudio-import = Importa…
txaudio-read-profile-json-file = Leggi un profilo da un file .json
txaudio-tx-audio-profile = Profilo audio TX
txaudio-no-profiles-yet-save-stores = Nessun profilo ancora. Salva memorizza la catena che hai impostato ora.
txaudio-load = Carica
txaudio-export = Esporta…
txaudio-delete = Elimina
txaudio-starting-points = Punti di partenza
txaudio-set-your-level-auto-set = Imposta prima il tuo livello con Imposta automatico — le soglie dei preset lo presuppongono — poi regola da qui anziché trasmettere come caricato: registra una frase e ascolta dopo ogni modifica. Un preset mantiene il tuo guadagno d'ingresso.
txaudio-profile-loaded = Profilo caricato.

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = PROP
solar-where-signals-are-actually-getting =
    Dove i segnali stanno davvero arrivando, banda per banda. Ogni ricezione a cui questa stazione prende parte — WSPR, FT8/FT4, JS8 e il logbook — è posta al punto medio del suo percorso, cioè la porzione di ionosfera che lo ha piegato, anziché alla stazione lontana. Quindi questa è una mappa del cielo, non di dove vivono i radioamatori.

    It can only show paths this station has been one end of. Oceans light up because the midpoints of long paths fall there; Antarctica stays dark because nobody is transmitting from it. Off by default — it paints the whole planet.
solar-orbits = ORBITE
solar-orbital-paths = Percorsi orbitali
solar-clouds = NUVOLE
solar-cloud-cover-noaa-s-global = La copertura nuvolosa dal mosaico geostazionario globale della NOAA, disegnata come una profondità d'aria anziché come un'immagine appiccicata sulla superficie: l'infrarosso dice quanto è fredda la sommità di ogni nube, e da lì si ricava quanto è alta. I temporali lampeggiano dentro le torri che costruiscono.
solar-planets = PIANETI
solar-other-seven-planets-eighteen-their = Gli altri sette pianeti, diciotto delle loro lune, e gli anelli di Saturno e Urano. Le posizioni dei pianeti vengono dal set di elementi kepleriani del JPL; le lune sono orbite circolari adattate ai dati JPL Horizons.
solar-coronal-mass-ejection-trajectory-cones = Coni di traiettoria delle espulsioni di massa coronale
solar-sun-obs = OSS. SOLE
solar-solar-observations-sun-s-disk = Osservazioni solari sul disco del Sole: regioni attive con macchie solari, e da dove provenivano i brillamenti
solar-labels = ETICHETTE
solar-body-region-labels = Etichette di corpi e regioni
solar-small-bodies = PICCOLI CORPI
solar-names-asteroids-comets-they-are = Nomi su asteroidi e comete. Vengono disegnati in ogni caso — questo decide solo se con loro vengono disegnate anche trentacinque designazioni. Qualunque sia l'impostazione, il corpo su cui è la camera e ciò che ha trovato la casella di ricerca restano con il nome.
solar-decoded-ft8-ft4-stations-path = Stazioni FT8/FT4 decodificate e il percorso verso la stazione con cui si lavora
solar-sats = SAT
solar-amateur-radio-satellites-their-orbits = Satelliti radioamatoriali, le loro orbite e l'elevazione dal tuo QTH
solar-aurora = AURORA
solar-auroral-oval-noaa-s-ovation = L'ovale aurorale dal modello OVATION della NOAA, disegnato come gusci di emissione alle loro altitudini reali, con il bordo verso l'equatore della curva del 10 % segnato sulla superficie
solar-awards = PREMI
solar-award-coverage-every-dxcc-entity = Copertura dei premi: ogni entità DXCC colorata da ciò che il tuo logbook ancora manca. L'arancio brucia dove non hai mai lavorato l'entità, l'ambra dove l'hai lavorata ma non è confermata, il verde spento dove è tornata una QSL. Segue il filtro di banda della finestra PREMI.
solar-view = VISTA
solar-which-body-camera-orbits-animated = Attorno a quale corpo orbita la camera, e il giro animato
solar-layers = LIVELLI
solar-what-drawn-orbits-clouds-cmes = Cosa viene disegnato: orbite, nuvole, CME, etichette, QSO, aurora
solar-which-sdo-channel-wraps-sun = Quale canale SDO avvolge il Sole, e quanto è vecchio
solar-weather = METEO
solar-aurora-propagation-numbers-which-bands = L'aurora, i numeri di propagazione e quali bande sono aperte
solar-space-weather = METEO SPAZIALE
solar-nothing-measured-yet = niente ancora misurato
solar-scale = SCALA
solar-body-moon-orbit-exaggeration = Esagerazione del corpo e dell'orbita lunare
solar-time = TEMPO
solar-scrub-whole-scene-forward-back = Scorri l'intera scena avanti e indietro
solar-activity = ATTIVITÀ
solar-replay-last-hour-decodes-globe = Riproduci l'ultima ora di decodifiche sul globo
solar-what-propagation-heat-map-showing = Cosa mostra la mappa termica della propagazione
solar-propagation = PROPAGAZIONE
solar-auto = ▶ AUTO
solar-fly-spline-through-set-framed = Vola lungo una spline attraverso un insieme di punti di vista inquadrati. Qualunque input del mouse la annulla.
solar-moons-ride-circular-orbits-fitted = Le lune seguono orbite circolari adattate ai dati JPL Horizons: entro uno o due gradi da dove si trovano davvero, e fino a sei per Miranda, il cui piano orbitale oscilla troppo in fretta perché un cerchio possa seguirlo.
solar-fetch-everything-again-now = Scarica di nuovo tutto ora
solar-all-sats = TUTTI I SAT
solar-show-every-satellite-element-set = Mostra ogni satellite dell'insieme di elementi, non solo quelli più noti. Gli anelli orbitali restano sui pochi selezionati — novanta insieme sono illeggibili.
solar-starting = avvio…
solar-body = corpo{" "}
solar-moon-orbit = orbita lunare{" "}
solar-stretch-earth-moon-distance-so = Allunga la distanza Terra→Luna così la coppia si può vedere separata
solar-sun = sole{" "}
solar-sun-radius-exaggeration-leave-1 = Esagerazione del raggio del Sole. Lascia a 1× per mantenere leggibile la geometria delle CME — un Sole gonfio inghiotte la base di ogni cono.
solar-show = MOSTRA
solar-paint-globe-what-getting-through = Colora il globo in base a ciò che sta passando
solar-paints-globe-what-getting-through = Colora il globo in base a ciò che sta passando, su ciascuna banda. Ogni ricezione è posta al punto medio del suo percorso — dove ha lavorato la ionosfera — anziché alla stazione lontana. È costruito da ciò che questa stazione sente, più gli skimmer del mondo quando la Reverse Beacon Network è attiva.
solar-display = DISPLAY
solar-all-bands = TUTTE LE BANDE
solar-every-band-once-one-hue = Ogni banda insieme, una tinta ciascuna, mescolando dove due si sovrappongono. La vista per "come sono le condizioni" anziché "è aperti i venti".
solar-one-band = UNA BANDA
solar-one-band-cold-hot-blue = Una banda, da freddo a caldo: blu, verde, giallo, rosso.
solar-band = BANDA
solar-nothing-heard-yet = niente di sentito ancora
solar-bands = BANDE
solar-sources = FONTI
solar-contacts-logbook-path-was-open = Contatti nel logbook. Un percorso che era aperto, senza un rapporto di segnale degno di questo nome — un RST non è un SNR, quindi questi contano per quanto è trafficata una cella e mai per il suo margine.
solar-stations-reported-hearing-one-downloaded = Stazioni che hanno riportato di sentire questa, scaricate da WSPRnet. L'unico riscontro che un radiofaro in trasmissione riceva mai.
solar-reverse-beacon-network-skimmers-what = Skimmer della Reverse Beacon Network: ciò che tutti gli altri stanno sentendo, su ogni banda, comprese quelle su cui questa radio non è. Più grossolani del resto — una riga RBN non porta locatori, quindi entrambi i capi vengono posti al centro del loro paese, il che è vicino per un paese piccolo e molto sbagliato per uno grande. Si attiva in Impostazioni → Spot.
solar-decodes-station-made-each-measured = Decodifiche fatte da questa stazione. Ognuna viene misurata rispetto al pavimento del proprio modo, così un rapporto WSPR e uno FT4 sono confrontabili.
solar-memory = MEMORIA
solar-half-life = emivita{" "}
solar-how-long-reception-takes-count = Quanto tempo deve passare perché una ricezione conti la metà. La memoria della ionosfera è breve — un'apertura di due ore fa non dovrebbe discutere con una di due minuti fa.
solar-live = DAL VIVO
solar-follow-band-happens = Segui la banda mentre accade
solar-replay = ⏸ RIPRODUCI
solar-replay-2 = ▶ RIPRODUCI
solar-replay-last-hour-decodes-over = Riproduci l'ultima ora di decodifiche, ancora e ancora
solar-min-ago = {" "}min fa
solar-where-last-hour-globe-showing = Dove nell'ultima ora sta mostrando il globo
solar-trail = scia{" "}
solar-how-long-decode-s-arc = Quanto a lungo l'arco di una decodifica resta sul globo dietro la testa
solar-speed = velocità{" "}
solar-how-much-faster-than-real = Quanto più veloce del tempo reale va la riproduzione
solar-qso-layer-off = Livello QSO disattivato
solar-no-decodes-yet = nessuna decodifica ancora
solar-qth-not-set-enter-your = QTH non impostato — inserisci il tuo quadrato locatore nelle Impostazioni
solar-mode = MODO
solar-path = PERCORSO
solar-sent = INVIATO
solar-rcvd = RICEV
solar-grid = LOCATORE
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = Geostazionario — sempre sopra il tuo orizzonte.
solar-no-passes-next-48-hours = Nessun passaggio nelle prossime 48 ore dal tuo QTH.
solar-start = INIZIO
solar-max-el = EL MAX
solar-aos-los-are-azimuths-horizon = AOS/LOS sono azimut sull'orizzonte. Gli orari sono in UTC.
solar-locked-unlock = {" "}● BLOCCATO — SBLOCCA{" "}
solar-release-satellite-lock = Rilascia l'aggancio al satellite
solar-lock = {" "}BLOCCA SU{" "}
solar-track-satellite-main-window-doppler = Segui questo satellite nella finestra principale: RX e TX corretti per l'effetto Doppler, uplink mappato sul transponder, comando del rotore
solar-no-frequencies-file-one-add = Nessuna frequenza in archivio per questo — aggiungile in Impostazioni ▸ TLE.
solar-frequencies-yours = FREQUENZE · TUE
solar-frequencies = FREQUENZE
solar-link = COLLEGAMENTO
solar-downlink-mhz = DISCESA (MHz)
solar-uplink-mhz = SALITA (MHz)
solar-doppler-shifts-these-few-khz = L'effetto Doppler le sposta di qualche kHz durante un passaggio LEO.
solar-satellite-planet-comet = satellite, pianeta o cometa
solar-planet-asteroid-comet = pianeta, asteroide o cometa
solar-clear-search = Cancella la ricerca
solar-no-match = nessuna corrispondenza
solar-no-sounder = nessun sondatore
solar-heard = SENTITI ≥
solar-kp = Kp / A
solar-x-ray = Raggi X
solar-observed-above-sounder-better-than = osservato sopra il sondatore — migliore del modello
solar-power-n-s = potenza N/S
solar-edge-n-s = bordo N/S
solar-kp-peak-24-h = picco Kp 24 h
solar-observed-forecast = osservato | previsto
solar-bands-open = BANDE APERTE
solar-dxcc-coverage = COPERTURA DXCC
solar-ir-vis = IR+VIS
solar-ir-only = solo IR
solar-no-deep-convection = nessuna convezione profonda
solar-t-1-storm = 1 tempesta
solar-glancing = {" "}· radente
solar-direction-estimated = {" "}· direzione stimata

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = Configurazione APRS
setup-packet-setup = Configurazione pacchetto
setup-ft8-ft4-ft2-setup = Configurazione FT8 / FT4 / FT2
setup-my-callsign = Il mio nominativo
setup-my-grid = Il mio locatore
setup-station-call = Nominativo di stazione
setup-ssid-oe3jjs-10-packet-station = Con un SSID — OE3JJS-10. Una stazione a pacchetto usa per convenzione un SSID diverso dall'operatore, e non trasmette nulla finché questo non è impostato.
setup-speed = Velocità
setup-t-9600-needs-radio-s-data = 9600 richiede la porta dati della radio. Un percorso microfono e altoparlante lo distrugge a entrambe le estremità — è la radio, non sdroxide.
setup-tx-delay = Ritardo TX
setup-flags-sent-before-frame-so = Flag inviati prima di un frame, così l'altro capo può sentirci e agganciare il suo clock. Su un apparato CAT sdroxide da solo impiega 165–240 ms per andare in aria, e il tempo di prontezza in trasmissione dell'apparato si aggiunge a questo.
setup-packet-length = Lunghezza del pacchetto
setup-bytes = {" "}byte
setup-most-single-frame-carries-shorter = Il massimo che un singolo frame porta. I frame più corti sopravvivono a un percorso marginale — viene rispedito solo il frame colpito — ma costano più overhead. 128 in HF, 256 dove il percorso è solido.
setup-frames-sent-before-waiting-acknowledgement = Frame inviati prima di attendere una conferma. Di più riempie un buon percorso; su uno cattivo è più roba da rispedire.
setup-extended = Esteso
setup-ask-mod-128 = Richiedi mod-128
setup-extended-sequence-numbers-window-bigger = Numeri di sequenza estesi, per una finestra più grande di sette. Molti nodi rifiutano la richiesta con un DM, che sembra esattamente una stazione che non vuole parlarti — quindi lascialo disattivato a meno che tu non sappia che l'altro capo lo vuole.
setup-answer-calls = Rispondi alle chiamate
setup-accept-connections = Accetta connessioni
setup-off-winlink-client-which-dials = Disattivato per un client Winlink, che compone in uscita e non ha motivo di rispondere. Attivato per essere raggiungibile come peer: una chiamata arriva nel riquadro del terminale e parli con chi l'ha fatta. Una stazione il cui collegamento è già occupato da una sessione Winlink rifiuta le chiamate finché quella non finisce.
setup-connect-text = Testo di connessione
setup-what-caller-greeted = con cosa viene salutato chi chiama
setup-sent-station-connects-us-once = Inviato a una stazione che si collega a noi, una volta attivo il collegamento. Vuoto non invia nulla — il che sembra un guasto per chi ha chiamato.
setup-default-via = Predefinito via
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = Il percorso di digipeater con cui parte il terminale. Il percorso verso il tuo nodo locale è sempre lo stesso, e riscriverlo è il modo in cui si dimentica un salto.
setup-beacon = Radiofaro
setup-t-0-disables-timer = 0 disattiva il timer
setup-beacon-text = testo del radiofaro
setup-kiss-server = Server KISS
setup-serve = Servi
setup-offer-modem-kiss-tnc-socket = Offre questo modem come TNC KISS su un socket, così Pat, un client APRS o lo stack AX.25 di Linux possono usare la radio.
setup-aprs-call = Nominativo APRS
setup-callsign-station-beacons-under-ssid = Il nominativo con cui questa stazione invia il radiofaro, con il suo SSID — OE3JJS-9 per un'auto, -10 per un I-gate, -5 per un telefono. Lascialo vuoto per usare il nominativo qui sopra.
setup-no-callsign-station-will-not = Nessun nominativo: questa stazione non trasmette affatto — né radiofaro, né messaggio, né conferma.
setup-symbol = Simbolo
setup-two-characters-protocol-carries-them = I due caratteri come li porta il protocollo: una tabella (`/` o `\`) e un codice simbolo. Una cifra o una lettera in prima posizione è un overlay, disegnato sopra l'icona della tabella alternativa.
setup-path = Percorso
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = Fino a dove chiedi di essere ripetuto. È l'impostazione più importante del canale: ogni salto moltiplica le trasmissioni che l'intera rete fa su una frequenza condivisa.
setup-position = Posizione
setup-my-grid-2 = Dal mio locatore
setup-beacon-centre-locator-above-reported = Invia il radiofaro al centro del locatore qui sopra, riportato con l'ambiguità che un locatore ha davvero — uno a sei caratteri copre un paio di chilometri, e dirlo è onesto. Deseleziona per dare invece coordinate esatte.
setup-latitude = Latitudine
setup-longitude = Longitudine
setup-locator = dal locatore
setup-not-coordinate = non una coordinata
setup-comment = Commento
setup-sent-every-beacon = inviato con ogni radiofaro
setup-t-0-default-never-beacons-timer = 0 — il predefinito — non invia mai radiofari a intervalli. Trenta minuti è la convenzione per una stazione fissa; una stazione in movimento ne invia di più, ma ogni radiofaro è tempo di canale di qualcun altro.
setup-compressed = Compresso
setup-compressed-position-format-third-air = Il formato di posizione compresso: un terzo del tempo in aria e più preciso. Lo legge ogni ricevitore dagli anni '90.
setup-first-goes-out-one-interval = Il primo parte un intervallo da ora, non subito.
setup-messages = Messaggi
setup-acknowledge = Conferma
setup-answer-messages-addressed-you-acknowledgement = Rispondi ai messaggi indirizzati a te. Una conferma è una trasmissione che questa stazione fa senza che gliela si chieda, quindi una configurazione solo ricezione dovrebbe disattivarla — e il radiofaro smette così anche di dichiararsi raggiungibile.
setup-flags-sent-ahead-every-frame = Flag inviati davanti a ogni frame, così il modem dell'altro capo sente la portante e aggancia il suo clock prima che inizino i dati. Deve durare più di tutto ciò che sta tra la pressione di trasmissione e l'irradiazione: sdroxide da solo ne spende 165–240 ms su un apparato CAT, e una radio che riceve l'audio via rete ne accumula altri. Troppo poco e l'altro capo non si aggancia mai — la trasmissione c'è e nulla la decodifica.
setup-shared-packet-mode = condiviso con il modo pacchetto
setup-keep-stations = Mantieni le stazioni
setup-how-long-station-stays-map = Quanto una stazione resta sulla mappa dopo l'ultima volta che è stata sentita. È anche il metro della dissolvenza della mappa, così una finestra corta mostra solo ciò che è attivo.
setup-auto-reply = Risposta automatica
setup-answer-snr-grid-status = Rispondi a SNR? / GRID? / STATUS?
setup-answer-direct-question-addressed-you = Rispondi a una domanda diretta indirizzata a te o a @ALLCALL, con la risposta anziché con una conferma. Non risponde mai al traffico di un'altra stazione, e non risponde mai a se stessa.
setup-heartbeat = Heartbeat
setup-off = Disattivato
setup-turbo-does-not-beacon-local = Turbo non invia radiofari — è la velocità locale e VHF.
setup-sends-your-callsign-grid-so = Invia il tuo nominativo e il locatore, così gli altri sanno che sei ricevibile. Il primo parte un intervallo da ora, non subito.
setup-beacon-frequency = Frequenza del radiofaro
setup-t-500-1000-hz = 500–1000 Hz
setup-move-each-beacon-free-slot = Sposta ogni radiofaro in uno slot libero nella sotto-banda heartbeat, come fa JS8Call: è dove guardano le stazioni che cercano radiofari, e tiene un trasmettitore incustodito lontano dal QSO di qualcun altro. Lo slot viene scelto quando il radiofaro parte davvero, libero da tutto ciò che viene decodificato.
setup-working-freq = Frequenza di lavoro
setup-beacon-where-you-are-working = Invia il radiofaro dove stai lavorando. Contro la convenzione della banda, ma tiene tutto ciò che trasmetti in un solo posto.
setup-heartbeat-reply = Risposta heartbeat
setup-answer-heartbeats-signal-report = Rispondi agli heartbeat con un rapporto di segnale
setup-tell-station-beaconed-how-well = Dici a una stazione che ha inviato un radiofaro quanto bene l'hai ricevuta. Disattivato di default: una banda affollata porta un heartbeat in ogni slot, e rispondere a tutti inonderebbe proprio la banda che gli heartbeat servono a tenere quieta. Limitato a una risposta per stazione ogni 15 minuti, e mai mentre un messaggio sta ancora arrivando o mentre hai qualcosa in coda da inviare.
setup-status-message = Messaggio di stato
setup-tx-period = Periodo TX
setup-even = Pari
setup-odd = Dispari
setup-auto-sequence = Sequenza automatica
setup-decode-depth = Profondità di decodifica
setup-fast = Veloce
setup-normal = Normale
setup-deep = Profonda

    Fast — one pass, no signal subtraction: quickest, fewest decodes.
    Normal — flat multi-pass subtraction: a little quicker than Deep, a little less thorough.
    Deep — the checkpointed multi-pass: the most decodes, and ~1.2 s on a busy slot.
setup-auto-tx-frequency = Frequenza TX automatica
setup-choose-transmit-frequency-automatically-quietest = Scegli automaticamente la frequenza di trasmissione: il punto più tranquillo nel periodo in cui trasmetti, anziché la frequenza della stazione a cui rispondi. Disattivato NON la mantiene — risponde sulla frequenza della stazione chiamata. Per mantenerla, usa Mantieni TX.
setup-hold-tx-frequency = Mantieni frequenza TX
setup-pin-transmit-tone-where-nothing = Fissa il tono di trasmissione dove si trova. Nulla lo sposta: né rispondere a una stazione, né la coda delle chiamate, né chiamare CQ, né un clic su una decodifica o sulla waterfall. Prevale su Frequenza TX automatica. Cambiare banda è l'unica eccezione: l'offset impostato per ultimo sulla nuova banda torna con essa. Serve quando la tua licenza è più stretta del piano di banda — su un 60 m britannico con sintonia a 5357 kHz l'assegnazione finisce a 5358.0, quindi il tono deve restare sotto 1000 Hz.
setup-tx-watchdog = Sorveglianza TX
setup-stop-transmitting-after-long-no = Smetti di trasmettere dopo questo tempo senza risposta e senza alcuna tua azione. 0 la disattiva.
setup-give-up-after = Rinuncia dopo
setup-calls = {" "}chiamate
setup-contest = Contest
setup-everyday-exchange-grid-report-rr73 = Lo scambio ordinario: locatore, rapporto, RR73.
setup-european-vhf-contests-send-signal = Contest VHF europei: invia un rapporto di segnale, un numero di serie e il tuo locatore a SEI caratteri, nel layout dei messaggi che WSJT-X chiama EU VHF Contest. CQ viene inviato come CQ TEST.
setup-serial = Seriale
setup-serial-number-next-exchange-will = Il numero di serie che porterà il prossimo scambio. Avanza da solo a ogni contatto registrato ed è ricordato tra le sessioni — impostalo a mano solo per iniziare un contest, o per riprendere da dove era arrivato un registro cartaceo.
setup-needs-6-character-grid = ⚠ richiede un locatore a 6 caratteri
setup-eu-vhf-contest-messages-carry = I messaggi dell'EU VHF Contest portano un locatore a sei caratteri (JN88DD, non JN88). Finché il mio locatore non ne ha uno, viene inviato invece lo scambio ordinario.
setup-dxpedition = DXpedition
setup-ordinary-ft8-operation = Operatività FT8 ordinaria.
setup-calling-dxpedition-running-fox-mode = Chiamare una DXpedition in modo Fox: chiama da sopra i 1000 Hz, scendi sulla Fox quando risponde, e registra sul suo RR73 senza inviare 73.
setup-run-pile-up-several-signals = Gestisci il pile-up: più segnali insieme, una coda di chiamanti, lavorati prima i più forti e i più rari. CHIAMA CQ lo avvia, FERMA QSO lo sospende.
setup-fox-signals = Segnali Fox
setup-once = {" "}insieme
setup-simultaneous-transmissions-spaced-60-hz = Trasmissioni simultanee, distanziate di 60 Hz. Condividono la potenza del trasmettitore, quindi più segnali significa che ciascuno è più debole.
setup-tx-audio = Audio TX
setup-deviation-fm = deviazione, in FM
setup-drive-ssb = pilotaggio, in SSB
setup-grid = Locatore
setup-report = Rapporto
setup-r-report = R+Rapporto

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = , scala 10 W, riscalata
amp-t-10-w-scale = , scala 10 W
amp-t-2-w-scale-rescaled = , scala 2 W, riscalata
amp-t-2-w-scale = , scala 2 W
amp-t-100-w-scale-rescaled = , scala 100 W, riscalata
amp-t-100-w-scale = , scala 100 W
amp-t-20-w-scale-rescaled = , scala 20 W, riscalata
amp-t-20-w-scale = , scala 20 W
amp-rescaled = , riscalata
amp-and = {" "}E{" "}
amp-not-while-transmitting-switching-relay = Non durante la trasmissione: commutare un relè con l'RF presente può danneggiarlo. Disattiva prima la trasmissione.
amp-no-amplifier-set-up-choose = Nessun amplificatore configurato. Scegli la sua porta in Impostazioni → Amplificatore.
amp-settings = IMPOSTAZIONI
amp-dock = AGGANCIA
amp-put-amp-window-back-inside = Riporta la finestra AMP dentro la finestra principale
amp-pop-out = STACCA
amp-give-amp-window-window-own = Dai alla finestra AMP una finestra propria, da spostare su un altro schermo
amp-rig = , dall'apparato
amp-rig-flagged-reading = , l'apparato ha segnalato la sua lettura
amp-drv = , da DRV
amp-rig-s-swr = , dal ROS dell'apparato
amp-ptt-keys-amplifier-radio-off = PTT attiva l'amplificatore con la radio; OFF lo lascia in bypass. Clicca per commutare.
amp-switch-antenna-jack = Commuta la presa d'antenna
amp-band-down = Banda giù
amp-band-up = Banda su
amp-tuner = ACCORDATORE
amp-tuned = Accordato
amp-tune = ACCORDA
amp-start-tune-then-key-steady = Avvia un accordo — poi attiva una portante stabile (CW, FM o RTTY). Premi di nuovo per fermarlo.
amp-put-tuner-line-bypass = Inserisci l'accordatore in linea, oppure escludilo
amp-tuning = Accordo in corso…
amp-bottom = IN BASSO
amp-pick-any-reading-either-needle = Scegli una lettura qualsiasi per uno dei due aghi. L'ago superiore è bianco, quello inferiore blu.
amp-forward-power-measured-radio-amplifier = Potenza diretta, misurata dalla radio: l'amplificatore non ne riporta in OFF
amp-forward-power-taken-drv-amplifier = Potenza diretta, presa da DRV: l'amplificatore non ne riporta in OFF, e in bypass la potenza della radio passa direttamente
amp-forward-power = Potenza diretta
amp-reflected-power-measured-radio-amplifier = Potenza riflessa, misurata dalla radio: l'amplificatore non ne riporta in OFF
amp-reflected-power-worked-out-fwd = Potenza riflessa, calcolata da FWD e dal ROS: l'amplificatore non ne riporta in OFF
amp-reflected-power = Potenza riflessa
amp-standing-wave-ratio-red-past = Rapporto d'onda stazionaria: rosso al limite di accordo o oltre (Impostazioni → Generale). Controlla l'antenna
amp-standing-wave-ratio-measured-radio = Rapporto d'onda stazionaria, misurato dalla radio: l'amplificatore non ne riporta in OFF
amp-standing-wave-ratio = Rapporto d'onda stazionaria
amp-drive-radio = Pilotaggio dalla radio
amp-drain-supply = Alimentazione di drain
amp-drain-current = Corrente di drain
amp-temp = TEMP
amp-heat-spreader-temperature-yellow-49 = Temperatura del dissipatore: giallo da 49 °C (120 °F), rosso sopra 65 °C (149 °F), fuori linea sopra 70 °C (158 °F). Clicca per °C/°F.
amp-c = °C
amp-f = °F
amp-performance = PRESTAZIONI
amp-numbers-hold-peaks-bars-are = i numeri mantengono i picchi · le barre sono in tempo reale
amp-input-power = POTENZA D'INGRESSO
amp-supply-volts-drain-amps = Tensione di alimentazione × corrente di drain
amp-efficiency = EFFICIENZA
amp-output-power-input-power = Potenza in uscita ÷ potenza in ingresso
amp-gain = GUADAGNO
amp-output-power-over-drive-decibels = Potenza in uscita rispetto al pilotaggio, in decibel
amp-off-line-over-temperature = FUORI LINEA · temperatura eccessiva
amp-connected-transmitting = COLLEGATO · IN TRASMISSIONE
amp-connected = COLLEGATO
amp-connecting = Connessione…
amp-atu-bypassed = ATU ESCLUSO
amp-no-inductance-needed = nessuna induttanza necessaria
amp-no-capacitance-needed = nessuna capacità necessaria
amp-c-out-antenna-side = C OUT · lato antenna
amp-c-transmitter-side = C IN · lato trasmettitore

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = Blocchi ricevuti (accesi) e ancora mancanti (spenti)
sstv-sstv = SSTV
sstv-auto = Auto
sstv-signal = Segnale
sstv-station-sending-mode-sdroxide-does = Una stazione sta trasmettendo in un modo che sdroxide non ha. Il segnale e la sintonia vanno bene — semplicemente non esiste un decodificatore per questo. Nulla da correggere dalla tua parte.
sstv-listening = in ascolto…
sstv-restart-rx = Riavvia RX
sstv-abandon-picture-being-received-listen = Abbandona l'immagine in ricezione e resta in ascolto della successiva intestazione. Per una trasmissione che ha iniziato a decodificarsi nel modo sbagliato — altrimenti il ricevitore resta impegnato finché quel modo non finisce.
sstv-callsign-last-station-sent-fsk = Il nominativo che l'ultima stazione ha inviato come FSK ID dopo la sua immagine.
sstv-tx-slant = Inclinazione TX
sstv-transmit-clock-trim-ppm-remove = Correzione del clock di trasmissione (ppm) per eliminare l'inclinazione sul decodificatore dell'altro capo
sstv-reset-0-ppm = Riporta a 0 ppm
sstv-fsk-id = FSK ID
sstv-send-your-callsign-tones-after = Invia il tuo nominativo in toni dopo ogni immagine — l'identificazione che leggono i ripetitori SSTV e gli altri programmi. Aggiunge circa 2,5 secondi, e non invia nulla finché non hai impostato un nominativo.
sstv-tx-lead = Anticipo TX
sstv-silence-sent-after-keying-before = Silenzio inviato dopo l'attivazione e prima della testata e del codice VIS dell'immagine. Un decodificatore che perde una qualsiasi parte di quell'intestazione non mostra affatto l'immagine, quindi questo copre il tempo tra la richiesta di PTT all'apparato e il suo effettivo ingresso in aria. 0 per un SDR che si attiva istantaneamente.
sstv-live = DAL VIVO
sstv-waiting-picture = in attesa di un'immagine…
sstv-no-low-audio = audio assente / basso
sstv-received = RICEVUTE
sstv-reading-radio-s-pictures = Lettura delle immagini della radio…
sstv-decoded-pictures-collect-here = Le immagini decodificate si raccolgono qui.
sstv-click-enlarge-right-click-delete = Clicca per ingrandire · clic destro per eliminare
sstv-delete-picture = Elimina questa immagine
sstv-loading = caricamento…
sstv-transmit = TRASMETTI
sstv-click-edit-slot-s-message = Clicca per modificare il messaggio di questo slot · doppio clic per caricare un'immagine
sstv-change-image = Cambia immagine…
sstv-load-image = Carica immagine…
sstv-clear = Cancella
sstv-empty-slot-s-picture-message = Svuota l'immagine di questo slot (il messaggio resta)
sstv-banner = Banner…
sstv-what-printed-across-top-every = Ciò che è stampato in cima a ogni immagine che questa stazione invia
sstv-fetching-slot-s-picture = Recupero dell'immagine di questo slot…
sstv-load-image-into-slot = Carica un'immagine in questo slot →
sstv-drawn-slot-s-image = Disegnato sull'immagine di questo slot
sstv-abort-tx = {" "}ANNULLA TX{" "}
sstv-delete-picture-store = Elimina questa immagine dall'archivio
sstv-received-image = Immagine ricevuta
sstv-no-longer-store = non più nell'archivio
sstv-loading-full-size = caricamento a dimensione piena…
sstv-save-picture = Salva immagine…
sstv-delete-sure = Eliminare — sei sicuro?
sstv-click-again-delete-good = Clicca di nuovo per eliminarla definitivamente
sstv-delete = Elimina…
sstv-image-banner = Banner dell'immagine
sstv-drawn-into-every-picture-station = Disegnato in ogni immagine che questa stazione trasmette, sopra il messaggio proprio dello slot.
sstv-draw-banner = Disegna il banner
sstv-substituted-when-picture-composed = Sostituito quando l'immagine viene composta:
sstv-top-left = In alto a sinistra
sstv-top-right = In alto a destra
sstv-colours = Colori
sstv-strip-top-edge-fades-black = La striscia, sul suo bordo superiore — sfuma in nero, oppure nel colore del gradiente sotto di essa
sstv-both-texts = Entrambi i testi
sstv-gradient = Gradiente
sstv-fade = sfuma verso
sstv-fade-strip-top-colour-second = Fai sfumare la striscia dal suo colore superiore a un secondo colore anziché al nero.
sstv-text-outline = Contorno del testo
sstv-draw-outline-around-banner-text = Disegna un contorno attorno al testo del banner, così resta leggibile su una striscia affollata o chiara.
sstv-text-gradient = Gradiente del testo
sstv-fade-banner-text-colour-second = Fai sfumare il testo del banner dal suo colore a un secondo colore lungo la larghezza dell'immagine.
sstv-rainbow-text = Testo arcobaleno
sstv-all-text-overriding-colours-above = tutto il testo, sostituendo i colori qui sopra
sstv-draw-every-text-picture-banner = Disegna ogni testo dell'immagine — banner e messaggio — come un arcobaleno orizzontale, ignorando i colori qui sopra.
sstv-message = Messaggio
sstv-slot-message-s-text-colour = Colore del testo del messaggio dello slot
sstv-outline = contorno in
sstv-height = Altezza
sstv-how-tall-strip-pixels-transmitted = Quanto è alta la striscia, in pixel dell'immagine trasmessa. Il testo è dimensionato su di essa — un fotogramma SSTV è largo solo 320 pixel, quindi un banner più alto è ciò che lo rende leggibile dall'altro capo.
sstv-reset = Ripristina
sstv-back-callsign-left-version-right = Torna al nominativo a sinistra e alla versione a destra
sstv-waiting-station-s-digital-mode = In attesa che si carichino le impostazioni dei modi digitali della stazione…
sstv-rifp = RIFP
sstv-size = Dimensione
sstv-calling-frequency-draft-names = La frequenza di chiamata che la bozza indica
sstv-encode = Codifica
sstv-how-picture-encoded-into-object = Come l'immagine viene codificata nell'oggetto che trasporta RIFP. Auto prova ciascuna e invia la più piccola.
sstv-try-every-encoding-send-smallest = Prova ogni codifica, invia la più piccola (mai con perdita)
sstv-gray = Grigio
sstv-grayscale-depth-rifp-s-raster = Profondità della scala di grigi. Il raster di RIFP è in scala di grigi per definizione — il colore non trova posto nel suo manifest.
sstv-dither = Dither
sstv-diffuse-quantisation-error-worth-below = Diffondi l'errore di quantizzazione — conviene sotto gli 8 bit
sstv-caption = Didascalia
sstv-sent-manifest-content-hint-shown = Inviata nel manifest come suggerimento del contenuto, e mostrata sotto l'immagine dai ricevitori. Viaggia come testo — non viene disegnata nell'immagine.
sstv-what-picture = Cos'è questa immagine
sstv-repeat-data = Ripeti i dati
sstv-send-every-data-frame-many = Invia ogni frame di dati questo numero di volte. RIFP è unidirezionale e senza richieste di ripetizione, quindi questo è l'unico recupero che un ricevitore ottiene.
sstv-chunk = Blocco
sstv-payload-octets-per-data-frame = Ottetti di payload per frame di dati (il profilo consiglia 192)
sstv-valid-frames-frames-failed-crc = Frame validi, frame con CRC errato e immagini complete verificate
sstv-no-transfer-progress = nessun trasferimento in corso
sstv-forget-incomplete-transfer = Dimentica questo trasferimento incompleto

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = Richiama
windows-delete = Elimina
windows-edit-name-frequency-mode-antenna = Modifica il nome, la frequenza, il modo, l'antenna e la configurazione del ripetitore
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = Lo spostamento del ripetitore, il tono e il burst a 1750 Hz memorizzati con questo canale
windows-abandon-edit-escape = Annulla la modifica (Esc)
windows-save = SALVA
windows-keep-changes-enter = Conserva le modifiche (Invio)
windows-recall-channel-without-moving-antenna = Richiama questo canale senza spostare l'antenna
windows-antenna-switch-when-channel-recalled = L'antenna su cui commutare quando questo canale viene richiamato. — lascia com'è ciò su cui si trova già la radio.
windows-shift = Spostamento
windows-tone = Tono
windows-invert = INVERTI
windows-normal = NORMALE
windows-dcs-polarity = Polarità DCS
windows-open-every-over-tone-burst = Apri ogni trasmissione con il burst di tono mentre questo canale è richiamato
windows-memories = Memorie
windows-memory-name = nome della memoria
windows-store = Memorizza
windows-folder-name = nome della cartella
windows-new-folder = Nuova cartella
windows-import = IMPORTA
windows-read-channel-list-chirp-csv =
    Legge un elenco di canali da un file CSV di CHIRP (.csv) — un'esportazione di un elenco di ripetitori, una tabella di canali marittimi o PMR, o qualsiasi altra cosa che CHIRP sappia scrivere.

    Each channel brings its frequency, mode, repeater shift and CTCSS or DCS tone with it. Channels already on this list — same frequency, same mode — are skipped, so re-importing an updated directory adds what is new rather than doubling what is not.
windows-chirp-csv = CSV CHIRP
windows-export = ESPORTA
windows-write-list-out-chirp-csv = Scrive questo elenco come file CSV di CHIRP, da caricare su un portatile o da conservare come copia di sicurezza.
windows-no-memories-yet = nessuna memoria ancora
windows-drag-memory-onto-folder-file = trascina una memoria su una cartella per archiviarla, sotto di esse per toglierla dall'archivio
windows-drop-here-unfile = rilascia qui per togliere dall'archivio
windows-sort = Ordina
windows-order-they-were-stored = Nell'ordine in cui sono state memorizzate
windows-name-ignoring-case = Per nome, senza distinguere maiuscole e minuscole
windows-frequency = Per frequenza
windows-band-then-frequency-inside-each = Per banda, poi per frequenza all'interno di ciascuna
windows-descending-click-ascending = Decrescente — clicca per crescente
windows-ascending-click-descending = Crescente — clicca per decrescente
windows-delete-folder-memories-move-top = Elimina cartella — le sue memorie passano al livello superiore
windows-rename-folder = Rinomina cartella
windows-empty-drop-memories-here = vuota — trascina qui delle memorie
windows-voice-keyer = Manipolatore vocale
windows-rec-records-your-microphone-play = REC registra dal microfono, RIPRODUCI ti permette di ascoltare ciò che hai registrato, TX lo manda in aria — come fa un tasto del tastierino numerico, un pad MIDI, oppure send_voice_mem di rigctld.
windows-stop-store = Ferma e memorizza
windows-stop = FERMA
windows-play = RIPRODUCI
windows-stop-listening = Smetti di ascoltare
windows-nothing-recorded-slot = Nulla registrato in questo slot
windows-not-while-transmitting = Non durante la trasmissione
windows-listen-message-nothing-transmitted = Ascolta questo messaggio — non viene trasmesso nulla
windows-stop-transmitting = Smetti di trasmettere
windows-tune-active-switch-off-first = TUNE è attivo — disattivalo prima
windows-transmit-message = Trasmetti questo messaggio
windows-erase-recording = Cancella questa registrazione
windows-rade-message-encoded-digital-voice = RADE: il messaggio viene codificato dal codec di voce digitale, esattamente come lo sarebbe una trasmissione dal vivo.

## TX Audio window: messages with values

txaudio-rec-hint = Registra una frase di prova dal microfono — fino a { $max } s, senza attivare il trasmettitore. Clicca di nuovo per fermare. Viene conservata non elaborata, così RIPRODUCI può far passare la stessa frase attraverso ogni modifica che fai.
txaudio-phrase-length = frase di { $secs } s
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = Guadagno d'ingresso impostato a { $gain } dB (i picchi della voce erano a { $loud } dBFS).{ $clip }
txaudio-listening = In ascolto… { $secs } s
txaudio-auto-set-hint = Parla normalmente per { $secs } s — come faresti in aria — e il guadagno d'ingresso viene impostato così che i picchi della voce stiano intorno a { $target } dBFS. Clicca di nuovo per annullare.
txaudio-eq-point = { $hz } Hz, { $db } dB — trascina per spostare
txaudio-not-a-profile = Non è un profilo audio TX: { $error }
txaudio-imported = Importato "{ $name }".
txaudio-saved = Salvato "{ $name }".
txaudio-deleted = Eliminato "{ $name }".
txaudio-loaded-preset = Caricato il punto di partenza "{ $name }".

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · offline
solar-radius-exaggeration = Esagerazione del raggio di Terra/Luna (massimo { $max }× a questa scala dell'orbita lunare)
solar-brightest-cell = cella più luminosa ≈ { $peak } percorsi · { $bands } { $bands ->
        [one] banda
       *[other] bande
    }
solar-hits-in-hour = { $hits } nell'ora
solar-passes-from = { $name } · PASSAGGI DA { $grid }
solar-elements-age = elementi di { $age } · SGP4
solar-point-at = Punta a { $az }° ({ $compass }), elevazione { $el }°.
solar-published-as = pubblicato come { $name }
solar-fly-there = { $name } — ↵ per volarci
solar-sats-tracked = { $n } su { $total } tracciati
solar-bodies-found = { $n } su { $total } corpi
solar-sats-and-bodies = { $n } su { $total } tracciati · { $bodies } su { $bodies_total } corpi
solar-confidence-on-band = { $confidence } su { $band }
solar-kp-now = { $kp } ora
solar-kp-in = { $kp } tra { $hours } h
solar-valid-age = valido { $valid } · { $age } di età
solar-full-bar = barra piena = { $share } del mondo
solar-own-paths-memory = dove sono andati i percorsi di questa stazione · memoria di { $minutes } min
solar-n-storms = { $n } tempeste
solar-clouds-legend = NUVOLE { $time } · { $age } di età · { $channels } · { $storms } · fulmini simulati
solar-cme-eta = ETA { $eta } (+{ $hours } h)
solar-cme-arrived = l'arrivo era { $eta } ({ $hours } h fa)
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = CME DIRETTA VERSO LA TERRA { $time } · { $speed } km/s · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = Configurazione { $mode }
setup-transmits-as = Trasmette come { $call }
setup-decimal-degrees = Gradi decimali, negativi per { $hemisphere }. Incollane uno direttamente da una mappa.

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } NON AFFIDABILE CON QUESTO ROS
amp-lower-drive-first = Abbassa prima il pilotaggio: { $drive } W sovrapiloterebbero l'amplificatore in linea (il suo limite è { $limit } W).
amp-live-value = in tempo reale { $value } { $unit }
amp-tripped = SCATTATO{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = massimo ≈ { $n } percorsi
panels-no-agreed-frequency = Nessuna frequenza concordata per { $mode } — apri questo per salvare quella su cui sei
panels-mode-frequencies-hint = Le { $n } frequenze per { $mode } — sceglierne una sintonizza la sintonia, e puoi salvare le tue
panels-mode-frequencies = { $mode } · frequenze
panels-global-convention = Una convenzione globale che il piano di banda della Regione IARU { $region } non riserva ai dati a banda stretta — controlla il tuo piano di banda prima di trasmettere qui.
panels-outside-data-segment = Ambra: fuori dal segmento dati della Regione { $region }.
panels-save-dial = ＋ Salva { $mhz } MHz
panels-save-dial-hint = Ricorda questa sintonia sotto { $mode } e offrila qui d'ora in poi. Salvata sulla stazione, quindi ogni schermo collegato a questa radio la ha.
panels-tr-period = Periodo T/R di { $period } secondi
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = Questo turno: { $into } s di { $slot }. Le decodifiche arrivano alla fine, ed è allora che può iniziare la trasmissione successiva.{ $mark }
panels-burst-mark = {" "}Il segno è dove finisce il burst, a { $secs } s.
panels-tr-period-burst = Periodo T/R di { $slot } secondi, burst di { $burst } s

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } { $n ->
        [one] canale
       *[other] canali
    }
windows-slot-n = Slot { $n }
windows-record-from-mic = Registra dal microfono (fino a { $max } s)

## SSTV panel: messages with values

sstv-picture-too-big = Quell'immagine pesa { $size } MB — il limite è { $limit } MB.
sstv-auto-mode = Auto ({ $mode })
sstv-not-decoded = { $what } — non decodificato
sstv-last = ultimo: { $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = Le immagini decodificate vengono salvate { $place } e si raccolgono qui.
sstv-older-in-store = { $n } più vecchie nell'archivio
sstv-older-load-more = { $n } più vecchie — carica altre
sstv-delete-picture-from = Elimina questa immagine { $place }
sstv-rifp-profile = { $profile } · ~{ $khz } kHz occupati · la sintonia è il centro del canale
sstv-rifp-too-wide = ⚠ { $profile } occupa ~{ $khz } kHz — troppo largo per un segmento a banda stretta
sstv-rifp-no-frequency = RIFP non assegna alcuna frequenza, e sdroxide lo trasmetterà ovunque tu sintonizzi. Un canale da { $khz } kHz sta solo dove è consentito il funzionamento a banda larga o FM — { $segments } — e non in un segmento a banda stretta, tanto meno in HF. Anche dentro quei segmenti le condizioni della tua licenza possono essere più ristrette. Sei tu l'operatore; controlla le tue regole.
sstv-rifp-tx-frame = ● Frame TX { $frame }/{ $frames } · { $secs } s rimanenti
sstv-rifp-sent-as = inviato come { $encoding } · { $octets } ottetti
sstv-rifp-rx-stats = frame { $frames } · errati { $bad } · immagini { $pictures }
sstv-rifp-session-idle = sessione { $session } · inattiva { $secs } s
sstv-rifp-session-waiting = sessione { $session } · blocchi trattenuti, in attesa del manifest · inattiva { $secs } s
sstv-store-on-radio = sulla radio, in { $dir }
sstv-store-in = in { $dir }

## decodes (src/app/panels/decodes.rs)

decodes-decodes = DECODIFICHE
decodes-adif = ADIF
decodes-sort = Ordina
decodes-single-list = Elenco unico
decodes-every-decode-one-list-newest = Ogni decodifica in un unico elenco, prima il turno più recente, anziché blocchi di turni pari/dispari. I chip di ordinamento ordinano allora l'intero elenco anziché ciascun turno, e ogni riga porta il suo orario di slot — ciano per uno slot pari, oro per uno dispari.
decodes-cq-only = Solo CQ
decodes-only-stations-calling-cq-only = Solo le stazioni che chiamano CQ — e solo le chiamate a cui puoi rispondere: un CQ diretto (DX, EU, JA, POTA, TEST …) viene elencato quando nomina te e nascosto quando nomina qualcun altro.
decodes-new-only = Solo nuove
decodes-only-stations-would-new-entity = Solo le stazioni che sarebbero nuove: entità, banda-slot, locatore o nominativo
decodes-auto-tx-frq = FRQ TX automatica
decodes-overridden-hold-tx-lift-hold = Sostituito da Mantieni TX. Togli il blocco per scegliere in che verso si sposta la frequenza di trasmissione.
decodes-hold-tx = Mantieni TX
decodes-pin-transmit-tone-where-nothing =
    Fissa il tono di trasmissione dove si trova. Nulla lo sposta: né rispondere a una stazione, né la coda delle chiamate, né chiamare CQ, né un clic su una decodifica o sulla waterfall. Disattivalo per spostarlo, poi riattivalo.

    Changing band is the one exception, and it is your own act: the offset you last set on the new band comes back with it, so a 60 m figure is not carried onto 20 m and 20 m's is not carried onto 60 m.

    For where the licence is narrower than the band plan — on a UK 60 m dial of 5357 kHz the allocation ends at 5358.0, so the tone has to stay under 1000 Hz, and either automatic mover will walk out of the band between one over and the next.
decodes-type-transmit-offset-hz-then = Scrivi l'offset di trasmissione in Hz, poi premi Invio. Dove la licenza è più stretta del piano di banda, l'intero intervallo sotto il bordo è tuo da scegliere, quindi qui non viene suggerito nulla.
decodes-clear-rx = {" "}CANCELLA RX{" "}
decodes-empty-decode-list-nothing-air = Svuota l'elenco delle decodifiche. Nulla di ciò che è in aria si ferma.
decodes-even = PARI
decodes-text = TESTO
decodes-dxcc = DXCC
decodes-band = BANDA
decodes-grid = LOCATORE
decodes-dupe = DOPPIO
decodes-reply = RISPOSTA
decodes-queued-click-remove = In coda — clicca per rimuovere
decodes-work-station-after-current-one = Lavora questa stazione dopo quella attuale
decodes-qsos-worked-since-sdroxide-was = QSO lavorati da quando sdroxide è stato avviato
decodes-qso-complete = ✓ QSO COMPLETATO
decodes-tx = ● TX
decodes-dxpedition-mode-transmit-frequency-held = Modalità DXpedition. La frequenza di trasmissione è tenuta fuori dalla metà della banda passante della Fox (sotto 1000 Hz) finché la Fox non risponde.
decodes-watchdog = SORVEGLIANZA
decodes-transmitting-stopped-no-reply-no = Trasmissione interrotta: nessuna risposta e nessuna azione per il periodo di sorveglianza. Chiama CQ o scegli un messaggio per riprendere.
decodes-well-inside-tolerance = Ben dentro la tolleranza.
decodes-no-active-qso-pick-decode = nessun QSO attivo — scegli una decodifica a cui rispondere, oppure Chiama CQ
decodes-ft8-engine-idle = Motore FT8 inattivo
decodes-being-worked = in lavorazione
decodes-clear = CANCELLA
decodes-no-messages = — nessun messaggio —
decodes-call-cq = {"  "}CHIAMA CQ{"  "}
decodes-stop-qso = {" "}FERMA QSO{" "}
decodes-stop-tx = {" "}FERMA TX{" "}
decodes-free-text-13-chars = testo libero (13 caratteri)
decodes-send = INVIA

## sat (src/app/sat.rs)

sat-pass-profile = PROFILO DEL PASSAGGIO
sat-satellite = SATELLITE
sat-satellites = SATELLITI
sat-passes = PASSAGGI
sat-set-your-grid-passes-settings = Imposta il tuo locatore per i passaggi (Impostazioni ▸ Generale).
sat-next = PROSSIMO
sat-peak = PICCO ≥
sat-all-satellites = tutti i satelliti
sat-off-only-satellites-frequencies-file = Disattivato: solo i satelliti con frequenze in archivio — quelli con cui puoi lavorare — e quelli che hai incollato tu. Attivato: ogni satellite dell'elenco, che con un abbonamento possono essere migliaia e richiedono un po' di tempo per la ricerca.
sat-no-passes-window-lower-peak = Nessun passaggio in questa finestra. Abbassa il picco, oppure allarga la finestra.
sat-length = DURATA
sat-peak-2 = PICCO
sat-from-to = DA → A
sat-up-now = ● visibile ora
sat-unlock = SBLOCCA
sat-release-lock-dial-stays-where = Rilascia il blocco; la sintonia resta dov'è
sat-azimuth = AZIMUT
sat-elevation = ELEVAZIONE
sat-range = DISTANZA
sat-rate = VELOCITÀ
sat-downlink = DISCESA
sat-doppler-rx = DOPPLER RX
sat-uplink = SALITA
sat-doppler-tx = DOPPLER TX
sat-no-pass-inside-next-48 =Nessun passaggio nelle prossime 48 ore dal tuo QTH.
sat-elements-stale-corrections-suspended-until = ⚠ Elementi obsoleti — correzioni sospese fino a un aggiornamento del TLE
sat-doppler-correction-needs-iq-front = La correzione Doppler richiede un front-end IQ — un apparato CAT traccia solo
sat-rotator-not-connected = Rotore: non collegato
sat-doppler = DOPPLER
sat-apply-computed-correction-rx-tx = Applica la correzione calcolata a RX e TX
sat-rotator = ROTORE
sat-steer-antenna-through-rotctld-client = Orienta l'antenna tramite il client rotctld (Impostazioni ▸ Server)
sat-name-norad = nome o NORAD
sat-set-your-grid-passes-settings-2 = imposta il tuo locatore per i passaggi (Impostazioni ▸ Generale)
sat-norad = NORAD
sat-pass = PASSAGGIO
sat-always-visible = sempre visibile
sat-not-here = non visibile da qui
sat-no-elements = nessun elemento
sat-pick-satellite-see-links-lock = Scegli un satellite per vederne i collegamenti e agganciarlo.
sat-no-frequencies-file-one-add = Nessuna frequenza in archivio per questo — aggiungile in Impostazioni ▸ TLE.
sat-link = COLLEGAMENTO
sat-downlink-mhz = DISCESA (MHz)
sat-uplink-mhz = SALITA (MHz)
sat-mode = MODO
sat-tune = SINTONIZZA
sat-set-dial-mode-link-no = Imposta sintonia e modo su questo collegamento — nessun blocco, nessuna correzione
sat-locked = {" "}AGGANCIATO{" "}
sat-lock = {" "}BLOCCA SU{" "}
sat-track-satellite-doppler-correction-rx = Segui questo satellite: correzione Doppler su RX e TX, salita mappata sul transponder, e la vista 3D lo segue

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ FERMA{" "}
wefax-end-chart-now-save-what = Chiude la carta ora e salva ciò che è arrivato
wefax-start = {" "}● AVVIA{" "}
wefax-start-chart-now-without-waiting = Avvia una carta subito, senza attendere un tono di avvio
wefax-phasing = fasatura…
wefax-subcarrier-offset-1900-hz-tune = Offset della sottoportante da 1900 Hz. Sintonizza per circa zero: la portante fax va da 1500 Hz (nero) a 2300 Hz (bianco), e un ricevitore disallineato di qualche centinaio di hertz taglia l'immagine a nero pieno o bianco pieno.
wefax-auto-start = AVVIO AUTOMATICO
wefax-auto-stop = STOP AUTOMATICO
wefax-end-450-hz-stop-tone = Chiude sul tono di stop a 450 Hz. Disattivalo per continuare a registrare attraverso una stazione che invia più carte di seguito.
wefax-phase = FASE
wefax-slant = INCLINAZIONE
wefax-sample-clock-trim-if-chart = Correzione del clock di campionamento. Se la carta pende a sinistra, aumentala; a destra, diminuiscila. Una scheda audio di cento ppm fuori dalla norma sposta lateralmente di una riga intera una carta da un quarto d'ora.
wefax-view = VISTA
wefax-scale-chart-panel-width = Scala la carta alla larghezza del pannello
wefax-whole = INTERA
wefax-shrink-chart-until-all-view = Riduci la carta finché tutta non è visibile in una volta
wefax-half-size = Metà dimensione
wefax-one-screen-pixel-per-fax = Un pixel dello schermo per ogni pixel del fax — scorri per il dettaglio
wefax-twice-size-scroll-move-around = Doppia dimensione; scorri per muoverti nella carta
wefax-height = ALTEZZA
wefax-stretch-picture-vertically-chart-came = Allunga l'immagine in verticale. Una carta uscita schiacciata o allungata di solito è decodificata alla velocità di riga sbagliata — questo la rende leggibile, e i chip LPM la correggono davvero. Doppio clic per scrivere un valore.
wefax-reset = RIPRISTINA
wefax-back-picture-s-own-proportions = Torna alle proporzioni dell'immagine
wefax-follow = SEGUI
wefax-keep-newest-lines-view-scrolling = Tiene visibili le righe più recenti. Scorrere verso l'alto lo disattiva, così puoi leggere ciò che è già arrivato; tornare in fondo lo riattiva.
wefax-saved = SALVATE
wefax-path = PERCORSO
wefax-reading-radio-s-charts = Lettura delle carte della radio…
wefax-finished-charts-collect-here = Le carte completate si raccolgono qui.
wefax-delete-chart = Elimina questa carta
wefax-loading = caricamento…
wefax-stations = STAZIONI
wefax-broadcast-radiofax-schedules-picking-one = Programmi di radiofax di radiodiffusione — sceglierne uno sintonizza la sintonia e imposta la velocità di linea
wefax-frequencies-are-published-carrier-dial = Le frequenze sono la portante pubblicata; la sintonia va 1,9 kHz sotto di essa, cosa che viene fatta per te. I programmi cambiano e le stazioni chiudono — consideralo un punto da cui iniziare a cercare, non un orario.
wefax-delete-chart-store = Elimina questa carta dall'archivio
wefax-newer = ◀ PIÙ RECENTE
wefax-chart-received-after-one = La carta ricevuta dopo questa
wefax-older = PIÙ VECCHIA ▶
wefax-chart-received-before-one = La carta ricevuta prima di questa
wefax-no-longer-store = non più nell'archivio
wefax-loading-full-size = caricamento a dimensione piena…
wefax-save = SALVA
wefax-save-copy-chart-computer = Salva una copia di questa carta su questo computer
wefax-sure = SEI SICURO?
wefax-click-again-delete-chart-good = Clicca di nuovo per eliminare questa carta definitivamente
wefax-delete = ELIMINA
wefax-file-chart-was-saved = Il file con cui questa carta è stata salvata

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = avvio del modem APRS…
aprs-aprs = APRS
aprs-no-callsign = nessun nominativo
aprs-nothing-will-transmitted-no-beacon = Non verrà trasmesso nulla — né radiofaro, né messaggio, nemmeno una conferma — finché questa stazione non ha un nominativo. Impostane uno in Impostazioni → Generale, oppure uno specifico per APRS con il suo SSID in CONFIGURA.
aprs-no-position = nessuna posizione
aprs-beacon-needs-somewhere-report-fill = Un radiofaro ha bisogno di un punto da riportare. Compila il tuo locatore in Impostazioni → Generale, oppure indica le coordinate in CONFIGURA.
aprs-setup = CONFIGURA
aprs-callsign-symbol-digipeater-path-position = Nominativo, simbolo, percorso dei digipeater, posizione e radiofaro
aprs-how-often-beacon-your-position = Ogni quanto inviare la tua posizione come radiofaro, senza presidio. `off` — il predefinito — non invia mai radiofari: scegliere un modo non deve mettere una stazione in aria. Trenta minuti è la convenzione per una stazione fissa, più spesso per una in movimento, e ogni radiofaro è tempo di canale di qualcun altro.
aprs-beacon = {" "}RADIOFARO{" "}
aprs-station-has-no-callsign-see = Questa stazione non ha un nominativo — vedi l'avviso a sinistra.
aprs-station-has-no-position-report = Questa stazione non ha una posizione da riportare — vedi l'avviso a sinistra.
aprs-send-one-position-report-now = Invia subito un rapporto di posizione, senza attendere l'intervallo. Attende che il canale sia libero come tutto il resto.
aprs-stations = STAZIONI
aprs-nothing-heard-yet = niente di sentito ancora
aprs-heard-direct = sentito direttamente
aprs-two-characters-station-sent-say = I due caratteri che la stazione ha inviato per dire cos'è
aprs-killed = ANNULLATO
aprs-station-put-object-here-has = La stazione che ha messo qui questo oggetto l'ha annullato
aprs-sender-deliberately-reported-square-rather = Il mittente ha riportato deliberatamente un quadrato anziché un punto. La mappa disegna il quadrato.
aprs-digipeaters-last-frame-came-through = I digipeater attraverso cui è passato l'ultimo frame. Un `*` segna quello che l'ha davvero ripetuto.
aprs-message = {" "}MESSAGGIO{" "}
aprs-address-message-box-station = Indirizza la casella dei messaggi a questa stazione
aprs-center = {" "}CENTRA{" "}
aprs-put-station-middle-map-double = Metti questa stazione al centro della mappa. Doppio clic sulla mappa per restituire la vista all'adattamento automatico.
aprs-channel = CANALE
aprs-messages = MESSAGGI
aprs-frames-arrived-failed-their-check =Frame arrivati che non hanno superato la sequenza di controllo — una collisione, un'attenuazione o un segnale troppo debole per essere letto. Un conteggio che sale senza che nulla venga decodificato significa che il modem *sta* sentendo il canale: controlla lo strumento di livello e apri lo squelch della radio.
aprs-frames-read-cleanly-off-channel = Frame letti senza errori sul canale che non erano APRS — la sessione pacchetto di qualcun altro, oppure un formato che questa build non decodifica.
aprs-every-frame-channel-arrived-other = Ogni frame sul canale, così come è arrivato — incluso il traffico altrui, e tutto ciò che il decodificatore non ha saputo leggere.
aprs-nothing-channel-yet = nulla sul canale ancora
aprs-no-messages = Nessun messaggio.
aprs-waiting-channel = in attesa del canale
aprs-sent-retried-still-waiting-acknowledged = inviato e ritentato, in attesa ancora di conferma
aprs-air-waiting-acknowledged = in aria, in attesa di conferma
aprs-acknowledged-far-end = confermato dall'altro capo
aprs-refused-far-end-not-retried = rifiutato dall'altro capo — non ritentato
aprs-no-ack = nessuna conferma
aprs-no-answer-after-every-retry = nessuna risposta dopo ogni tentativo
aprs-send = {" "}INVIA{" "}
aprs-station-has-no-callsign-so = Questa stazione non ha un nominativo, quindi non si può trasmettere nulla. Impostane uno in Impostazioni → Generale, oppure uno specifico per APRS con il suo SSID in CONFIGURA.
aprs-needs-station-address-something-say = Servono una stazione a cui indirizzarlo e qualcosa da dire.
aprs-send-keep-retrying-until-far = Invialo, e continua a ritentare finché l'altro capo non conferma. I messaggi sono l'unica cosa su questo canale a cui si risponde.
aprs-busy = OCCUPATO
aprs-stations-heard-still-inside-window = Stazioni sentite ancora nella finestra impostata in Configurazione APRS, e se un'altra stazione è sul canale in questo momento. Nulla verrà attivato mentre è occupato.
aprs-tx = ● TX
aprs-over-progress-frames-waiting-channel = La trasmissione in corso, i frame in attesa che il canale si liberi, oppure il tempo fino al prossimo radiofaro programmato.
aprs-audio-reaching-modem-flat-means = Audio che arriva al modem. Piatto significa che non arriva nulla — verifica che l'uscita dati della radio sia quella su cui sta ascoltando sdroxide. Si accende di verde mentre il modem sente una portante.

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = avvio del decodificatore VDL2…
vdl2-aircraft-datalink = collegamento dati aeronautico
vdl2-tune-middle-vdl2-group-decoder = Sintonizzati al centro del gruppo VDL2. La finestra propria del decodificatore scorre da lì per includere quanti più dei quattordici canali la radio riesce a raggiungere.
vdl2-setup = CONFIGURA
vdl2-which-channels-listen-how-hard = Su quali canali ascoltare, quanto forte deve essere un burst, e quanto log conservare
vdl2-save-vdl2-message-log-file = Salva il registro dei messaggi VDL2 su un file
vdl2-vdl2-channel = Canale VDL2
vdl2-messages = MESSAGGI
vdl2-address-flight-registration-label = indirizzo, volo, registrazione o etichetta
vdl2-held = BLOCCATO
vdl2-hold = BLOCCA
vdl2-hold-message-log-where-so = Blocca il registro dei messaggi dove si trova, così uno può essere letto senza che la trasmissione successiva lo spinga su nello schermo. Il decodificatore continua a girare e i contatori continuano a muoversi; rilasciando si vede tutto ciò che è arrivato nel frattempo.
vdl2-nothing-decoded-yet = nulla di decodificato ancora
vdl2-nothing-matches-filter = nulla corrisponde al filtro
vdl2-stations = STAZIONI
vdl2-nobody-heard-yet = nessuno sentito ancora
vdl2-vdl2-setup = Configurazione VDL2
vdl2-channels = Canali
vdl2-every-25-khz-slot-136 = Ogni slot da 25 kHz da 136,650 a 136,975 MHz. Un convertitore a discesa per ciascuno, tutti dentro la stessa finestra del ricevitore. Disattivarne uno fa risparmiare un po' di tempo di processore; non rende gli altri più sensibili.
vdl2-burst-threshold = Soglia del burst
vdl2-above-each-channel-s-own = sopra il rumore di fondo proprio di ciascun canale
vdl2-lower-catches-weaker-transmissions-costs = Più bassa cattura trasmissioni più deboli e costa tempo di processore sul rumore; più alta le perde. Il fondo è appreso per canale e una modifica qui non lo butta via.
vdl2-keep-log = Conserva nel registro
vdl2-track-most = Traccia al massimo
vdl2-longest-silent-go-first = prima vanno quelle più silenziose da più tempo
vdl2-forget-station-after = Dimentica una stazione dopo
vdl2-nothing-heard = senza aver sentito nulla da essa
vdl2-show-unread-payloads = Mostra i payload non letti
vdl2-hex = in esadecimale
vdl2-frames-carrying-x-25-clnp = Frame che portano X.25, CLNP o le applicazioni di collegamento dati sopra di essi. SDRoxide li indica e mostra i byte anziché leggerli, e nasconderli nasconderebbe quanto di quel traffico sia.
vdl2-registration = Registrazione
vdl2-flight = Volo
vdl2-label = Etichetta
vdl2-block = Blocco
vdl2-sequence = Sequenza
vdl2-mode = Modo
vdl2-more-blocks-follow = seguono altri blocchi
vdl2-check = Controllo
vdl2-message-crc-good = CRC del messaggio corretto
vdl2-message-crc-not-checked = CRC del messaggio non verificato
vdl2-parity = Parità
vdl2-link-control-no-payload = controllo di collegamento, nessun payload
vdl2-name = NOME
vdl2-addr = IND
vdl2-type = TIPO
vdl2-msgs = MSG

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = Il tono che viene copiato, e il tono trasmesso — in CW sono la stessa frequenza. Clicca sulla waterfall per spostarlo su un segnale.
cw-down-10-hz = Giù di 10 Hz
cw-up-10-hz = Su di 10 Hz
cw-qrg-main-readout-tuning-line = QRG: la lettura principale e la linea di sintonia sono sulla frequenza su cui si lavora. Clicca per la sintonia del dial, con un tono di sidetone sotto di essa — ciò che mostra la maggior parte delle radio.
cw-put-main-readout-tuning-line =
    Porta la lettura principale e la linea di sintonia sul segnale anziché sul dial, così la frequenza mostrata è quella che citerebbero entrambi gli operatori e la linea di sintonia sta al centro della banda passante. La sintonia non cambia; cambiano solo i numeri.

    Clicking a signal lands it on the cursor only as closely as the click step allows — Controls → click-to-tune rounding, 10 Hz by default. A coarse step leaves the signal off the pitch by up to half of it, and the readout will say so.
cw-sending-speed-read-off-signal = Velocità di invio letta dal segnale
cw-signal-noise-500-hz-same = Segnale su rumore in 500 Hz — la stessa cifra che riporta un rapporto
cw-how-far-off-cursor-signal = Quanto il segnale è davvero lontano dal cursore. Il decodificatore lo segue; la banda passante no, quindi sposta un po' il cursore se cresce.
cw-listening = — in ascolto —
cw-tx = ● TX
cw-nothing-copied-yet = — nulla di copiato ancora —
cw-key-send-characters-you-send = Premi il tasto per inviare — i caratteri che invii compaiono qui.
cw-type-line-return-sends = Scrivi una riga, Invio la invia…
cw-type-here-send = Scrivi qui per inviare…
cw-key = {" "}TASTO ●{" "}
cw-key-2 = {" "}TASTO{" "}
cw-hold-key-bound-cw-straight = Tieni premuto il tasto assegnato al tasto CW dritto — la barra spaziatrice per impostazione predefinita, e qualunque tasto in Impostazioni → Comandi — come tasto dritto: giù finché lo tieni, su al rilascio, invece di scrivere testo. La casella di trasmissione è bloccata mentre è attivo, e l'intero manipolatore è affidato al tasto: il testo in coda viene scartato.

    To hand-key it, set CW keying to "Sound card (MCW)" in Settings → Radio. The rig is then held on a sideband and the keyer's own sidetone is transmitted as audio, which is the route the straight key drives.
cw-watchdog = SORVEGLIANZA
cw-straight-key-was-held-down = Il tasto dritto è stato tenuto premuto troppo a lungo — un rilascio perso anziché una mano — quindi la portante è stata abbattuta e la trasmissione disattivata. Premi di nuovo il tasto per continuare.
cw-tx-2 = {"  "}TX ON{"  "}
cw-tx-3 = {"   "}TX{"   "}
cw-send-what-box-now-without = Invia subito ciò che c'è nella casella, senza attendere Invio
cw-hold-key-down-between-characters = Tieni premuto il tasto tra un carattere e l'altro, così nulla di ciò che scrivi resta in attesa
cw-call-cq = {" "}CHIAMA CQ{" "}
cw-nocall = NOCALL
cw-clear = {" "}CANCELLA{" "}
cw-stop-sending-drop-whatever-has = Smetti di inviare e scarta ciò che non è ancora uscito. Con il tasto dritto attivo, va via con esso anche la rilettura di ciò che hai manipolato.
cw-sidetone = SIDETONE
cw-play-keyed-tone-through-computer = Riproduci il tono manipolato dagli altoparlanti di questo computer oltre a inviarlo, così senti ciò che stai inviando. Con `Scheda audio (MCW)` il tono va alla radio e da nessun'altra parte, quindi senza questo si invia nel silenzio. Disattivalo dove il monitor della radio o una copia in aria fa già il lavoro.
cw-hold-what-typed-until-return = Tieni ciò che scrivi finché non premi Invio, poi invia la riga in un unico blocco anziché manipolare ogni carattere mentre lo scrivi. Vale la pena averlo su un ricetrasmettitore che si attiva da solo dal testo, dove ogni passaggio al suo manipolatore è un altro ciclo trasmissione-ricezione.
cw-cw-messages = MESSAGGI CW
cw-clear-rx = {" "}CANCELLA RX{" "}
cw-empty-receive-window-straight-key = Svuota la finestra di ricezione e la rilettura del tasto dritto. Nulla di ciò che è in aria si ferma.
cw-lock = BLOCCA
cw-decode-speed-set-here-instead = Decodifica alla velocità impostata qui anziché leggerla dal segnale. Aiuta un segnale troppo debole perché la ricerca di velocità si stabilizzi.
cw-farnsworth-send-characters-full-speed = Farnsworth: invia i caratteri a piena velocità e allunga solo gli intervalli tra di essi, così si sentono nel ritmo giusto ma arrivano abbastanza lenti da poterli scrivere.
cw-off-normal-spacing = Disattivato — spaziatura normale
cw-keying-speed = Velocità di manipolazione
cw-idle-0 = INATTIVO 0
cw-how-long-transmit-held-after = Per quanto tempo la trasmissione resta attiva dopo l'ultimo carattere, o dopo il rilascio del tasto dritto, prima che la portante cada. Più lungo copre le pause di chi scrive lentamente; più corto esce prima dalla frequenza; Disattivato la abbatte appena tutto ciò che è in coda è uscito.
cw-off-drop-once = Disattivato — abbatti subito

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = Sintonizza giù di 10 Hz
textmodem-tune-up-10-hz = Sintonizza su di 10 Hz
textmodem-tx = ● TX
textmodem-listening = — in ascolto —
textmodem-type-line-return-sends = Scrivi una riga, Invio la invia…
textmodem-type-here-transmit = Scrivi qui per trasmettere…
textmodem-tx-2 = {"  "}TX ON{"  "}
textmodem-tx-3 = {"   "}TX{"   "}
textmodem-call-cq = {" "}CHIAMA CQ{" "}
textmodem-nocall = NOCALL
textmodem-clear = {" "}CANCELLA{" "}
textmodem-hold-what-typed-until-return = Tieni ciò che scrivi finché non premi Invio, poi invia la riga in un unico blocco anziché trasmettere ogni carattere in aria mentre lo scrivi. Permette di rileggere e correggere una riga prima che ne esca qualcosa.
textmodem-messages = MESSAGGI
textmodem-hell = HELL
textmodem-contrast = Contrasto
textmodem-harder-softer-dots-redraws-whole = Punti più o meno marcati — ridisegna l'intera striscia
textmodem-width = Larghezza
textmodem-t-2row = {" "}2RIGHE{" "}
textmodem-draw-every-column-twice-stacked = Disegna ogni colonna due volte, impilata. Hell non ha sincronizzazione verticale, quindi questo mantiene una copia completa del testo leggibile qualunque sia la fase.
textmodem-rev = {" "}INV{" "}
textmodem-reverse-video-light-dots-dark = Video invertito — punti chiari su carta scura
textmodem-clear-rx = {" "}CANCELLA RX{" "}
textmodem-hold-channel-idle-sends-blank = Blocca il canale: a riposo invia carta bianca, così la striscia continua a scorrere
textmodem-hold-what-typed-until-return-2 = Tieni ciò che scrivi finché non premi Invio, poi invia la riga in un unico blocco anziché disegnare ogni carattere sulla striscia mentre lo scrivi. Una correzione scritta in diretta è già sulla carta; una fatta prima di Invio non ci è mai stata.
textmodem-shift = Spostamento
textmodem-shift-5-hz = Spostamento −5 Hz
textmodem-shift-5-hz-2 = Spostamento +5 Hz
textmodem-baud = Baud
textmodem-reverse-swap-mark-space-needed = Inverti: scambia marcatura e spazio. Serve quando il segnale viene ricevuto sulla banda laterale opposta a quella su cui è stato inviato, il che inverte i toni.
textmodem-track-tuning-error-automatically-instead = Segui automaticamente l'errore di sintonia anziché restare fissato sul cursore.
textmodem-tones = Toni
textmodem-mode = Modo
textmodem-off-keyed = attivo/disattivo a manipolazione

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = avvio del decodificatore AIS…
ais-tune-between-two-ais-channels = Sintonizzati fra i due canali AIS — 161,975 (AIS 1, marittimo 87B) e 162,025 (AIS 2, 88B). La sintonia va in mezzo perché lì non trasmette nulla, quindi il picco DC di un ricevitore zero-IF non cade su nessuno dei due.
ais-setup = CONFIGURA
ais-channels-timeouts-trail-length-how = Canali, timeout, lunghezza della scia e fino a dove arrivano i vettori
ais-vessels = NAVI
ais-nothing-heard-yet = niente di sentito ancora
ais-virtual-nothing-there = virtuale — non c'è niente
ais-position-stale = posizione non aggiornata
ais-search-rescue-transmitter-man-overboard = un trasmettitore di ricerca e soccorso, un radiofaro di uomo in mare o un EPIRB AIS
ais-call-sign = nominativo
ais-station-time = ora della stazione
ais-dgnss = {" "}(DGNSS)
ais-first-heard = sentito per la prima volta
ais-center = CENTRA
ais-put-vessel-middle-chart = Metti questa nave al centro della carta
ais-last-message-station-nmea-sentences = L'ultimo messaggio di questa stazione, come le frasi NMEA che un ricevitore metterebbe sul filo. Incollalo in qualunque altro decodificatore AIS per verificare cosa ne ha ricavato sdroxide.
ais-ais-setup = Configurazione AIS
ais-channels = Canali
ais-drop-chart-after = Rimuovi dalla carta dopo
ais-without-position-report = senza un rapporto di posizione
ais-five-minutes-default-not-ads = Cinque minuti di default, non i dieci secondi di ADS-B: una nave all'ancora riporta una volta ogni tre minuti, e una finestra più corta svuoterebbe gran parte di un porto tra due rapporti perfettamente validi. Oltre quel tempo la nave esce dalla carta e la sua riga diventa grigia — non viene sbiadita, perché un simbolo attenuato su una posizione vecchia è comunque un'affermazione su dove si trova una nave, con lo stesso inchiostro di quelle vere.
ais-drop-list-after = Rimuovi dall'elenco dopo
ais-nothing-heard-all = senza aver sentito nulla
ais-message-carrying-ship-s-name = Il messaggio che porta il nome di una nave torna ogni sei minuti, quindi una finestra d'elenco breve continua a scartare navi proprio prima che dicano come si chiamano.
ais-trail-length = Lunghezza della scia
ais-history-behind-each-target = di storia dietro ogni bersaglio
ais-minutes-rather-than-points-because = In minuti anziché in punti, perché le frequenze di segnalazione AIS coprono due ordini di grandezza: un conteggio fisso sarebbe ottanta secondi di un traghetto e due ore di una petroliera all'ancora, disegnate allo stesso modo. Zero disattiva le scie.
ais-speed-vector = Vettore di velocità
ais-how-far-ahead-vector-reaches = fino a dove arriva il vettore in avanti
ais-slot-threshold = Soglia dello slot
ais-above-channel-s-learned-noise = sopra il rumore di fondo appreso del canale
ais-track-most = Traccia al massimo
ais-vessels-2 = {" "}navi
ais-ais-receive-only-here-always = L'AIS qui è solo ricezione e lo sarà sempre: è un servizio per la sicurezza della vita in mare, e inserirvi traffico di navi falso non è coperto da alcuna licenza.
ais-fill-my-grid-digimode-setup = Compila Il mio locatore nella configurazione dei modi digitali per distanze e rilevamenti, così la carta si inquadra attorno a dove sei anziché attorno a ciò che è più lontano.
ais-name = NOME
ais-mmsi = MMSI
ais-type = TIPO

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = Questo ricevitore non può raggiungere 10489.750 MHz da solo — configura prima un offset LNB/convertitore (Impostazioni ▸ Radio)
qo100-run-fast-spectral-tracker-every = Avvia il tracker spettrale veloce: ogni secondo cerca, nella finestra di parcheggio, i due lobi simmetrici del radiofaro e riporta dove si trova la portante
qo100-telemetry = TELEMETRIA
qo100-also-run-ao-40-uncoded = Avvia anche il decodificatore di frame non codificati AO-40 — parola di sincronizzazione, CRC e il testo di telemetria — con una lettura passo passo (portante → sincronizzazione → CRC) di quanto arriva ogni passaggio
qo100-auto = AUTO
qo100-auto-correct-let-tracker-adjust = Auto-correzione: lascia che il tracker regoli da solo l'offset del convertitore/LNB — un anello chiuso lento che, su una stima pulita e stabile, riporta il radiofaro su 10489.750 MHz e lo tiene lì mentre l'LNB deriva. Riapre il front-end ogni volta che agisce.
qo100-narrower-search-smaller-slice = Più stretta — cerca in una porzione più piccola
qo100-wider-when-beacon-isn-t = Più larga — per quando il radiofaro non viene trovato alla larghezza attuale
qo100-clear-mark = cancella il segno
qo100-drop-hand-placed-beacon-mark = Togli il segno del radiofaro posto a mano e torna alla lettura del decodificatore
qo100-lo = lo −
qo100-lo-2 = lo +
qo100-hi = hi −
qo100-hi-2 = hi +
qo100-radio-s-own-tuning-range = La gamma di sintonia propria di questa radio non arriva a 10489.750 MHz — serve un offset LNB/convertitore (Impostazioni ▸ Radio ▸ Convertitore) prima che questa finestra possa fare qualcosa.
qo100-open-settings-radio = Apri Impostazioni ▸ Radio
qo100-tune-10489-750-mhz = Sintonizza su 10489.750 MHz
qo100-tip-if-you-can-see = suggerimento: se vedi il radiofaro nella striscia ma non si aggancia, fai doppio clic su di esso — lo segna come 10489.750 MHz e permette ad APPLICA di correggere su di esso
qo100-tracker = TRACKER
qo100-no-twin-lobe-shape-parking = nessuna forma a doppio lobo nella finestra di parcheggio
qo100-shape = FORMA
qo100-drift = DERIVA
qo100-need-longer-run-estimates = — (servono più stime in sequenza)
qo100-cycles = CICLI
qo100-receiver = RICEVITORE
qo100-target = BERSAGLIO
qo100-measured = MISURATO
qo100-clicked = {"  "}(cliccato)
qo100-tracker-2 = {"  "}(tracker)
qo100-not-locked-yet = non ancora agganciato
qo100-converter-offset = OFFSET DEL CONVERTITORE
qo100-decode = DECODIFICA
qo100-decode-filling-first-frame-buffer = DECODIFICA — riempimento del primo buffer di frame…
qo100-decode-off-turn-telemetry-chip = DECODIFICA — disattivata. Attiva il chip TELEMETRIA (riga in alto) per la lettura portante → sincronizzazione → CRC.
qo100-apply-correction = {" "}APPLICA CORREZIONE{" "}
qo100-write-converter-lnb-offset-puts = Scrive l'offset del convertitore/LNB che porta il radiofaro dove hai cliccato su 10489.750 MHz, e riapre il ricevitore — la stessa breve interruzione che fa Impostazioni ▸ Radio ▸ Applica
qo100-write-converter-lnb-offset-puts-2 = Scrive l'offset del convertitore/LNB che porta la stima del tracker su 10489.750 MHz, e riapre il ricevitore — la stessa breve interruzione che fa Impostazioni ▸ Radio ▸ Applica
qo100-waiting-second-crc-valid-frame = In attesa di un secondo frame con CRC valido prima di proporre di scrivere questo — un solo aggancio potrebbe essere una coincidenza
qo100-write-corrected-converter-lnb-offset = Scrive l'offset del convertitore/LNB corretto e riapre il ricevitore — una breve interruzione, la stessa che fa Impostazioni ▸ Radio ▸ Applica
qo100-decoder-runs-receiving-station-readout = il decodificatore gira sulla stazione ricevente — la sua lettura non viene inviata ai client remoti
qo100-starting = avvio…
qo100-searching-first-window-fills-after = ricerca in corso — la prima finestra si riempie dopo circa 24 s, poi si ripete

## js8 (src/app/panels/js8.rs)

js8-multi = MULTI
js8-decode-every-js8-speed-not = Decodifica ogni velocità JS8, non solo quella su cui trasmetti. Le quattro velocità condividono la sotto-banda e sono quattro forme d'onda diverse, quindi senza questo una stazione su un'altra velocità è invisibile — e uno scambio tra due velocità non può avvenire affatto. Costa circa quattro volte la CPU in ricezione.
js8-hb-auto = HB AUTO
js8-turbo-does-not-beacon-local = Turbo non invia radiofari — è la velocità locale e VHF
js8-beacon-your-callsign-grid-every = Invia il tuo nominativo e il locatore ogni 15 minuti
js8-until-next-heartbeat = Fino al prossimo heartbeat
js8-tx = ● TX
js8-heard = SENTITI
js8-nothing-heard-yet = — niente di sentito ancora —
js8-dxcc = DXCC
js8-band = BANDA
js8-grid = LOCATORE
js8-dupe = DOPPIO
js8-reply = RISPOSTA
js8-no-messages = — nessun messaggio —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}HB{" "}
js8-clear = {" "}CANCELLA A{" "}
js8-forget-selected-station-composer-goes = Dimentica la stazione selezionata — il compositore torna a @ALLCALL
js8-already-addressing-allcall = Si sta già indirizzando a @ALLCALL
js8-stop = {" "}FERMA{" "}
js8-send = {" "}INVIA{" "}
js8-message = Messaggio…

## logbook (src/app/logbook.rs)

logbook-logbook = LOGBOOK
logbook-new-entry = + NUOVA VOCE
logbook-adif = ADIF
logbook-import = IMPORTA
logbook-import-qsos-adif-adi-adif = Importa QSO da un file ADIF (.adi o .adif)
logbook-new-qso = NUOVO QSO
logbook-edit-qso = MODIFICA QSO
logbook-call = Nominativo
logbook-lookup = CERCA
logbook-look-up-name-qth-grid = Cerca nome / QTH / locatore
logbook-grid = Locatore
logbook-freq-mhz = Freq MHz
logbook-mode = Modo
logbook-rst-sent = RST inviato
logbook-rst-rcvd = RST ricevuto
logbook-name = Nome
logbook-state = Stato
logbook-country = Paese
logbook-date-utc = Data UTC
logbook-time = Ora
logbook-pwr-w = Pot W
logbook-contest = Contest
logbook-s-sent = N.S. inviato
logbook-s-rcvd = N.S. ricevuto
logbook-comment = Commento
logbook-save = {" "}SALVA{" "}
logbook-cancel = ANNULLA
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = QSL ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = Elimina questa voce
logbook-edit = MODIFICA
logbook-upload-qso-configured-logs = Invia questo QSO ai log configurati
logbook-no-qsos-yet-run-ft8 = nessun QSO ancora — avvia FT8/FT4 oppure aggiungi una voce manuale

## scanner (src/app/scanner.rs)

scanner-scanner = Scanner
scanner-work-through-stored-memory-channels = Scorri i canali di memoria memorizzati
scanner-work-through-slice-band-channel = Scorri una porzione di banda su una griglia di canali
scanner-stop = STOP
scanner-start = AVVIO
scanner-from = Da
scanner-step = Passo
scanner-mode = Modo
scanner-range-empty-high-edge-has = Quell'intervallo è vuoto — il limite superiore deve stare almeno un canale sopra quello inferiore.
scanner-skip-while-holding-pass-over = SALTA mentre è tenuto premuto per passare oltre quel canale da ora in poi
scanner-skipping = Salto
scanner-clear = CANCELLA
scanner-stop-passing-over-any-them = Smetti di passare oltre uno qualsiasi di essi
scanner-stop-passing-over-channel = Smetti di passare oltre questo canale
scanner-no-memory-channels-stored-yet = Nessun canale di memoria ancora — memorizzane alcuni nella finestra MEM.
scanner-skip-channel-pass-over = SALTA un canale per passarvi oltre
scanner-skip = SALTA
scanner-look-all-channels-fall-inside = Cerca tutti i canali che cadono dentro una finestra del ricevitore sulla stessa trasformata da cui è costruito il panadapter, e sintonizza solo quelli su cui c'è qualcosa. Un elenco su una banda costa allora una sintonia per giro, per quanto lungo sia, invece di un tempo di assestamento per canale. La scansione ascolta comunque ogni candidato prima di fermarsi, quindi ciò che la ferma non cambia — ma lo sweep misura tramite la FFT anziché tramite il filtro del ricevitore, quindi controlla la soglia se comincia a fermarsi su niente.
scanner-radio-hands-over-demodulated-audio = Questa radio consegna audio demodulato e non ha uno spettro proprio in cui cercare, quindi la sua scansione di memoria visita ogni canale comunque.
scanner-fast = VELOCE
scanner-read-list-off-spectrum-instead = leggi l'elenco dallo spettro invece di visitare ogni canale
scanner-folders = Cartelle
scanner-scan-every-folder-every-folder = Scansiona ogni cartella, e ogni cartella creata d'ora in poi. Scegli le cartelle per scansionare solo quelle.
scanner-unfiled = Senza cartella
scanner-nothing-filed-under-folders-you = Nulla è archiviato nelle cartelle che hai scelto.
scanner-stops = Si ferma a
scanner-use-receiver-s-own-squelch = Usa lo squelch proprio del ricevitore, così la scansione si ferma esattamente dove si aprirebbe l'audio
scanner-squelch-off-scan-will-stop = lo squelch è disattivato — la scansione si fermerà sulla prima cosa che esamina
scanner-channel-power-signal-has-reach = Potenza del canale che un segnale deve raggiungere per fermare la scansione
scanner-listens = Ascolta per
scanner-how-long-stay-candidate-before = Quanto restare su un candidato prima di giudicarlo. Sotto circa un decimo di secondo lo strumento di livello non si è assestato e i segnali deboli vengono persi
scanner-resumes = Riprende
scanner-carry-once-signal-drops = Prosegui appena il segnale cade
scanner-carry-after-fixed-time-regardless = Prosegui dopo un tempo fisso, in ogni caso
scanner-stay-until-you-press-next = Resta finché non premi SUCCESSIVO
scanner-how-long-stay = Quanto restare
scanner-how-long-wait-after-signal = Quanto attendere dopo che il segnale cade
scanner-move-don-t-stop-channel = Passa oltre, e non fermarti più su questo canale
scanner-next = SUCCESSIVO
scanner-move-now = Passa oltre ora

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = avvio del decodificatore ADS-B…
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = Mode S a 1090 MHz
adsb-tune-receiver-1090-000-mhz = Sintonizza il ricevitore su 1090.000 MHz, il collegamento in discesa ADS-B mondiale. C'è un solo canale.
adsb-setup = CONFIGURA
adsb-timeouts-trail-length-how-far = Timeout, lunghezza della scia e fino a dove arrivano i vettori di velocità
adsb-aircraft = AEREI
adsb-nothing-heard-yet = niente di sentito ancora
adsb-ground = a terra
adsb-position-stale = posizione non aggiornata
adsb-gnss-altitude = Altitudine GNSS
adsb-ground-speed = velocità al suolo
adsb-vertical-rate = velocità verticale
adsb-first-heard = sentito per la prima volta
adsb-center = CENTRA
adsb-put-aircraft-middle-map = Metti questo aereo al centro della mappa
adsb-last-frame-accepted-aircraft = L'ultimo frame accettato da questo aereo
adsb-ads-b-setup = Configurazione ADS-B
adsb-drop-map-after = Rimuovi dalla mappa dopo
adsb-without-position-report = senza un rapporto di posizione
adsb-past-aircraft-comes-off-map = Oltre questo tempo l'aereo esce dalla mappa e la sua riga diventa grigia. Non viene sbiadito: un quadrato spento su una posizione vecchia è comunque un'affermazione su dove si trova un aereo, con lo stesso inchiostro di quelli veri.
adsb-drop-list-after = Rimuovi dall'elenco dopo
adsb-nothing-heard-all = senza aver sentito nulla
adsb-trail-length = Lunghezza della scia
adsb-points = {" "}punti
adsb-history-dots-behind-each-target = punti di storia dietro ogni bersaglio
adsb-speed-vector = Vettore di velocità
adsb-how-far-ahead-leader-line = fino a dove arriva la linea guida in avanti
adsb-one-minute-usual-radar-convention = Un minuto è la convenzione radar usuale: la linea è esattamente lunga quanto la distanza che l'aereo percorre in quel tempo, quindi due guide uguali sono due velocità uguali a qualunque zoom. Zero le disattiva.
adsb-track-most = Traccia al massimo
adsb-aircraft-2 = {" "}aerei
adsb-aircraft-ground-are-placed-against = Gli aerei a terra sono collocati rispetto alla posizione della stazione — un squitter di superficie non ha una decodifica univoca propria — quindi compila Il mio locatore nella configurazione dei modi digitali se l'aeroporto vicino non mostra nulla.
adsb-call = NOMINATIVO
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = in attesa del ricevitore…
drm-not-drm-set-mode-drm = Non in DRM. Imposta il modo su DRM su una radiodiffusione digitale in onde corte — la sintonia va sul centro del canale, non su una banda laterale.
drm-no-drm-signal-locked-decoder = Nessun segnale DRM agganciato. Il decodificatore ha bisogno di qualche secondo su una portante pulita: il DRM intreccia su 400 ms o 2 s prima che se ne possa leggere qualcosa.
drm-station-s-audio-codec-not = Il codec audio di questa stazione non viene decodificato qui. L'xHE-AAC richiede libfdk-aac nel sistema — non può essere incluso, perché la sua licenza e quella di questo programma sono incompatibili. Installalo (Debian/Ubuntu: libfdk-aac2, Arch: libfdk-aac, macOS: brew install fdk-aac, Windows: libfdk-aac-2.dll accanto a sdroxide.exe) e riavvia. Tutto il resto di questa schermata viene decodificato normalmente.
drm-quality = QUALITÀ
drm-last-minute = ultimo minuto
drm-waiting-locked-signal = in attesa di un segnale agganciato
drm-audio-frames-decoding-but-station = Frame audio in decodifica — ma il codec di questa stazione non può essere decodificato qui
drm-audio-frames-decoding = Frame audio in decodifica
drm-samples-reaching-decoder = Campioni che raggiungono il decodificatore
drm-time = ORA
drm-symbol-timing-recovered = Temporizzazione dei simboli recuperata
drm-frame = FRAME
drm-transmission-frames-found = Frame di trasmissione trovati
drm-fast-access-channel-what-transmission = Canale di accesso rapido — cos'è la trasmissione
drm-service-description-channel-what-services = Canale di descrizione dei servizi — quali sono i servizi
drm-audio = AUDIO
drm-constellation = COSTELLAZIONE
drm-waiting-decoded-symbols = in attesa dei simboli decodificati
drm-sampled-across-frame = campionato lungo il frame
drm-whole-frame = intero frame
drm-mode = MODO
drm-robustness-mode-channel-width-ground = Modo di robustezza e larghezza di canale. A è per un percorso a onda di superficie e porta il massimo; D è per un percorso a onda ionosferica molto disperso e porta il minimo.
drm-interleave = INTERLEAVING
drm-how-far-transmission-spreads-each = Quanto lontano la trasmissione distribuisce ogni frame nel tempo. Lungo resiste a evanescenze più profonde e richiede proporzionalmente più tempo per agganciare.
drm-protection = PROTEZIONE
drm-offset = OFFSET
drm-residual-sample-clock-error-against = Errore residuo del clock di campionamento rispetto al trasmettitore. Grande e costante significa che il riferimento del ricevitore è fuori, non la trasmissione.
drm-doppler = DOPPLER
drm-delay = RITARDO
drm-doppler-delay-spread-path-how = Dispersione Doppler e di ritardo del percorso — quanto velocemente si muove e quanto distanti arrivano i suoi echi.
drm-service = SERVIZIO
drm-decode-service-multiplex = Decodifica questo servizio del multiplex

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = avvio della stazione AtCHAT NET…
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET — tastiera multi-stazione e file
atchat-net = {" "}IN RETE{" "}
atchat-net-click-leave-state-kept = In rete — clicca per uscire (lo stato viene conservato; clicca di nuovo per rientrare)
atchat-rejoin = {" "}RIENTRA{" "}
atchat-off-net-click-rejoin = Fuori rete — clicca per rientrare
atchat-off-net = fuori rete
atchat-carrier = ● PORTANTE
atchat-another-station-transmitting-channel-busy = Un'altra stazione sta trasmettendo — il canale è occupato.
atchat-tx = ● TX
atchat-station-callsign-virtual-channel-address = Nominativo della stazione e indirizzo del canale virtuale
atchat-virtual-channel = CANALE VIRTUALE
atchat-work-net-over-tcp-endpoint = Lavora in rete tramite un endpoint TCP anziché la radio — un loopback compatibile con channel_server per lo sviluppo senza radio. Disattivato mette la stazione in aria.
atchat-roster = ELENCO
atchat-nobody-heard-yet = nessuno sentito ancora
atchat-chat = {" "}CHAT{" "}
atchat-common-channel-lines-here-go = Il canale comune — le righe qui vanno a tutti (ALL)
atchat-log = {" "}LOG{" "}
atchat-station-s-own-air-activity = L'attività in aria della stazione — elezione del master, invecchiamento dell'elenco, tentativi ARQ
atchat-no-activity-logged-yet = Nessuna attività registrata ancora.
atchat-no-messages-yet = Nessun messaggio ancora.
atchat-message-everyone-all = messaggio a tutti (ALL)
atchat-send = {" "}INVIA{" "}
atchat-transfers = TRASFERIMENTI
atchat-all = → ALL
atchat-sent-file-follows-active-chat = Un file inviato segue la scheda di chat attiva
atchat-send-file = {" "}INVIA FILE{" "}
atchat-send-file-image-over-air = Invia un file o un'immagine in aria, con CRC a blocchi e ARQ
atchat-nothing-flight = nulla in volo
atchat-images = IMMAGINI
atchat-no-images-received = Nessuna immagine ricevuta.


## rds (src/app/rds.rs)

rds-station = STAZIONE
rds-diagnostics = DIAGNOSTICA
rds-which-programme-type-table-read = Con quale tabella di tipo di programma leggere questa stazione. RDS si usa ovunque fuori dal Nord America; RBDS aggiunge il nominativo che il codice di identità di una stazione statunitense compone. Automatico segue il codice paese esteso che la stazione invia, e finché non ne arriva uno non compone alcun nominativo — gli intervalli di identità che leggerebbe sono condivisi con i codici RDS di nove paesi, quindi seleziona RBDS per dare un nome a una stazione che non invia un codice paese.
rds-waiting-receiver = in attesa del ricevitore…
rds-no-rds-station-not-every = Nessun RDS su questa stazione. Non ogni trasmettitore lo porta, e uno debole può portarlo senza che arrivi.
rds-rds-only-carried-wfm-broadcast = L'RDS è portato solo dalle stazioni di radiodiffusione WFM.
rds-holding = BLOCCATO
rds-block-sync-has-dropped-last = La sincronizzazione dei blocchi è caduta — questa è l'ultima stazione letta.
rds-identity = IDENTITÀ
rds-programme = PROGRAMMA
rds-described = DESCRITTO COME
rds-content = CONTENUTO
rds-music = Musica
rds-speech = Parlato
rds-traffic = TRAFFICO
rds-announcement-air-now = annuncio in onda ora
rds-carried-station = portato da questa stazione
rds-not-carried = non portato
rds-radio-text = TESTO RADIO
rds-station-clock = OROLOGIO DELLA STAZIONE
rds-country-code = CODICE PAESE
rds-also = ANCHE SU
rds-also-tagged = ANCHE CONTRASSEGNATO
rds-sync = SINCRO
rds-no-sync = NESSUNA SINCRO
rds-blocks-failed-their-check-plus = Blocchi che non hanno superato il controllo, più quelli che il correttore d'errore ha dovuto riparare. Le correzioni contano come errori di proposito: una cifra in salita è il primo segno che una stazione sta per smettere di decodificarsi.
rds-front-end-overloading = FRONT-END IN SOVRACCARICO
rds-converter-clipping-which-puts-distortion = Il convertitore sta saturando, il che mette distorsione su tutto il multiplex — compreso il sottoportante dati a 57 kHz. Abbassa il guadagno RF, oppure inserisci un attenuatore: l'RDS cede circa un decibel prima che si senta qualcosa, quindi una stazione che suona perfetta può essere ancora troppo forte perché i suoi dati sopravvivano.
rds-group-types = TIPI DI GRUPPO
rds-recent-groups-newest-last = GRUPPI RECENTI — il più recente in fondo
rds-nothing-decoded-yet = nulla di decodificato ancora

## winlink (src/app/winlink.rs)

winlink-mail = POSTA
winlink-abort = ANNULLA
winlink-stop-session-progress = Ferma la sessione in corso
winlink-connect = COLLEGA
winlink-compose = SCRIVI
winlink-connecting = connessione…
winlink-not-connected-yet = non ancora collegato
winlink-loading = caricamento…
winlink-date = DATA
winlink-from = DA
winlink-subject = OGGETTO
winlink-no-subject = (nessun oggetto)
winlink-delete-message = Elimina questo messaggio
winlink-arch = ARCH
winlink-move-archive = Sposta nell'archivio
winlink-select-message = seleziona un messaggio
winlink-reply = RISPONDI
winlink-delete = ELIMINA
winlink-to = A
winlink-cc = Cc
winlink-subject-2 = Oggetto
winlink-callsign-smtp-someone-example-org = Un nominativo, oppure SMTP:qualcuno@example.org per la posta internet. Separa più destinatari con virgole.
winlink-file-outbox = METTI IN POSTA IN USCITA
winlink-discard = SCARTA
winlink-filed-messages-go-out-next = i messaggi messi in posta in uscita partono alla prossima sessione
winlink-mail-log = REGISTRO POSTA
winlink-session-running = sessione in corso…
winlink-no-session-yet = nessuna sessione ancora

## wspr (src/app/panels/wspr.rs)

wspr-receptions = RICEZIONI
wspr-save-wspr-reception-list-csv = Salva l'elenco delle ricezioni WSPR in CSV
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = radiofaro di propagazione a segnali deboli
wspr-time-left-two-minute-slot = Tempo rimasto in questo slot da due minuti. Alla fine la registrazione passa al decodificatore e cade la successiva decisione di trasmissione.
wspr-tx = ● TX
wspr-decoding = decodifica…
wspr-band = Banda
wspr-last-slot = Ultimo slot
wspr-nothing-heard = nulla di sentito
wspr-receive-only = solo ricezione
wspr-next-slot = Prossimo slot
wspr-waiting-radio-take-change = in attesa che la radio accetti il cambio…
wspr-hop-next = Salta al prossimo
wspr-transmit = TRASMETTI
wspr-receive-only-2 = Solo ricezione.
wspr-power = POTENZA
wspr-what-you-actually-radiate = ciò che irradi davvero
wspr-goes-out-message-everyone-who = Questo viene inviato nel messaggio, e chiunque ti senta giudica il percorso da esso — quindi una cifra ottimistica qui rende sbagliate le loro misure oltre alle tue.
wspr-roam = ROAM
wspr-pick-different-offset-inside-200 = Scegli un offset diverso dentro la finestra da 200 Hz per ogni trasmissione. È la convenzione: duecento hertz condivisi da tutti funzionano solo se nessuno si ferma nel mezzo. Disattivato resta dove si trova il cursore.
wspr-band-hop = SALTO DI BANDA
wspr-move-dial-band-band-between = Sposta la sintonia da banda a banda tra uno slot e l'altro, così un solo ricevitore campiona l'intero spettro anziché una sua porzione. Se giri tu il VFO la funzione si mette in pausa — premi questo due volte per riprendere.
wspr-upload = INVIA
wspr-send-what-station-decodes-wsprnet = Invia a wsprnet.org ciò che questa stazione decodifica. Riportare ciò che senti è ciò che fa di un ricevitore WSPR una parte della rete; non mette nulla in aria.
wspr-who-heard-me = CHI MI HA SENTITO
wspr-ask-wsprnet-org-every-few = Chiedi a wsprnet.org ogni pochi minuti quali stazioni hanno decodificato questa. WSPR non prevede alcuna forma di conferma, quindi è l'unico modo in cui un radiofaro impara qualcosa della propria portata.
wspr-bands = BANDE
wspr-none-selected-hopping-has-nowhere = nessuna selezionata — il salto non ha dove andare
wspr-set-your-callsign-grid-general = Imposta il tuo nominativo e il locatore nella scheda Generale delle Impostazioni prima di trasmettere.

## packet (src/app/panels/packet.rs)

packet-monitor = MONITOR
packet-busy = OCCUPATO
packet-frames-arrived-but-failed-their = Frame arrivati che non hanno superato la sequenza di controllo — una collisione, un'attenuazione, o un segnale troppo debole per essere letto.
packet-station-callsign-speed-tx-delay = Nominativo di stazione, velocità, ritardo TX, il percorso dei digipeater, il radiofaro e il server KISS
packet-starting-packet-modem = avvio del modem a pacchetto…
packet-nothing-heard-yet = niente di sentito ancora
packet-terminal = TERMINALE
packet-connected = {" "}COLLEGATO{" "}
packet-calling = {" "}CHIAMATA{" "}
packet-idle = {" "}INATTIVO{" "}
packet-extended-sequence-numbers-window-up = Numeri di sequenza estesi — una finestra fino a 127 frame.
packet-frames-sent-not-yet-acknowledged = Frame inviati e non ancora confermati dall'altro capo.
packet-retries-against-link-s-limit = Tentativi rispetto al limite di questo collegamento. Un conteggio che sale mentre i frame non confermati restano fermi è ciò che un percorso in attenuazione appare da questa parte.
packet-mail-window-using-link = La finestra POSTA sta usando il collegamento.
packet-one-radio-one-channel-one = Una radio, un canale, un collegamento. Una sessione Winlink e questo terminale sono due modi di usarlo, e chi lo chiede per primo lo ottiene.
packet-digipeaters-nearest-first-separated-commas = Digipeater, il più vicino per primo, separati da virgole — OE3XLR-1,OE3XMS-1. Lascialo vuoto per una stazione che senti direttamente, oppure per usare il percorso predefinito delle impostazioni pacchetto.
packet-disconnect = {" "}DISCONNETTI{" "}
packet-hang-up-properly-wait-far = Riattacca come si deve, e attendi che l'altro capo sia d'accordo. Cambiare modo invece termina la sessione senza avvisare nessuno.
packet-connect = {" "}COLLEGA{" "}
packet-mail-window-has-link-finish = La finestra POSTA ha il collegamento. Finisci o ferma prima quella sessione.
packet-station-has-no-callsign-yet = Questa stazione non ha ancora un nominativo — impostane uno in CONFIGURA, sopra il riquadro del monitor. Non trasmette nulla finché non è impostato.
packet-call-station-connected-mode-node = Chiama questa stazione in modo collegato — un nodo, un BBS, o un altro operatore.
packet-needs-callsign-call = Serve un nominativo da chiamare.
packet-clear = {" "}CANCELLA{" "}
packet-empty-transcript-link-untouched-connected = Svuota la trascrizione. Il collegamento non viene toccato — una stazione collegata resta collegata.
packet-not-connected-type-callsign-above = Non collegato. Scrivi un nominativo sopra e premi COLLEGA per chiamare un nodo o un BBS.
packet-type-here = scrivi qui
packet-not-connected = non collegato
packet-send = {" "}INVIA{" "}
packet-send-line-node-bbs-reads = Invia la riga. Un nodo o un BBS legge una riga alla volta.

## fsq (src/app/panels/fsq.rs)

fsq-speed = Velocità
fsq-call = Chiamata
fsq-callsign-directed-fsqcall-messages-defaults = Nominativo per i messaggi diretti (FSQCALL); il predefinito è il tuo nominativo
fsq-tx = ● TX
fsq-contacts = CONTATTI
fsq-heard = SENTITI
fsq-none = — nessuno —
fsq-allcall = ALLCALL
fsq-image = IMMAGINE
fsq-clear = CANCELLA
fsq-forget-pictures-received-so-far = Dimentica le immagini ricevute finora
fsq-send-image = Invia immagine…
fsq-right-click-remove = Clic destro per rimuovere
fsq-remove-picture = Rimuovi questa immagine
fsq-listening = — in ascolto —
fsq-message = Messaggio…
fsq-send = {" "}INVIA{" "}
fsq-call-cq = {" "}CHIAMA CQ{" "}
fsq-heard-2 = {" "}? sentito{" "}
fsq-clear-2 = {" "}CANCELLA{" "}
fsq-fsq-contacts = Contatti FSQ
fsq-add = Aggiungi

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + SCHEDA
publicsdr-open-receiver-new-radio-own = Apri questo ricevitore come nuova radio, nella sua scheda
publicsdr-point-radio-receiver-replacing-whatever = Punta *questa* radio al ricevitore, sostituendo qualunque interfaccia usi ora — chiesto di nuovo prima, perché è la configurazione di un'intera radio
publicsdr-not-stated = non indicato
publicsdr-replace = SOSTITUISCI
publicsdr-point-radio-you-are-receiver = Punta la radio su cui sei a questo ricevitore
publicsdr-tab-instead = + SCHEDA INVECE
publicsdr-open-receiver-another-radio-leave = Apri il ricevitore come un'altra radio e lascia questa in pace
publicsdr-cancel = ANNULLA
publicsdr-public-sdrs = SDR PUBBLICI
publicsdr-available = DISPONIBILI
publicsdr-hide-receivers-are-full-ones = Nascondi i ricevitori che sono pieni, e quelli il cui operatore non ha aperto canali ad app diverse dal browser
publicsdr-band = IN BANDA
publicsdr-only-receivers-cover-current-dial = Solo i ricevitori che coprono la frequenza di sintonia attuale
publicsdr-low-bw = BASSA LARGH.
publicsdr-take-spyserver-low-bandwidth-shape = Usa uno SpyServer nella sua forma a bassa larghezza di banda: una finestra I/Q stretta che segue la sintonia più la vista di banda propria del server, invece di megabit di I/Q a banda larga. Nessun effetto su un KiwiSDR, che ha una sola forma.
publicsdr-refresh = ⟳ AGGIORNA
publicsdr-fetch-both-directories-again = Scarica di nuovo entrambe le directory
publicsdr-name-place-antenna-band = nome, luogo, antenna, banda
publicsdr-clear-search = Cancella la ricerca
publicsdr-fetching-receiver-lists = Recupero degli elenchi dei ricevitori…
publicsdr-machine-radio-attached-does-not = La macchina a cui è collegata questa radio non risponde alle domande sui dispositivi, quindi non può nemmeno scaricare gli elenchi.
publicsdr-no-match = nessuna corrispondenza
publicsdr-nothing-matches-try-turning-filter = nulla corrisponde — prova a riattivare un filtro
publicsdr-radio = questa radio
publicsdr-radio-s-configuration-has-not = La configurazione di questa radio non è ancora arrivata dalla macchina su cui si trova, quindi non c'è nulla a cui puntare il ricevitore — prendilo con + SCHEDA.
publicsdr-position-not-published-so-nothing = {" "}La sua posizione non è pubblicata, quindi nulla di ciò che sente verrà riportato.

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = in attesa del ricevitore…
hd-not-hd-radio-set-mode = Non in HD Radio. Imposta il modo su HD Radio su una portante di radiodiffusione FM con le sidecar digitali — la sintonia va sul centro della portante analogica, non accanto ad essa.
hd-no-hd-radio-lock-digital = Nessun aggancio HD Radio. Le bande laterali digitali sono trasmesse circa 20 dB sotto la portante analogica, quindi una stazione ascoltabile può essere comunque troppo rumorosa per essere decodificata: il decodificatore ha bisogno di qualche secondo su una pulita.
hd-sync = SINCRO
hd-ofdm-frame-timing-recovered = Temporizzazione dei frame OFDM recuperata
hd-audio = AUDIO
hd-audio-frames-decoding = Frame audio in decodifica
hd-quality = QUALITÀ
hd-lower = INFERIORE
hd-upper = SUPERIORE
hd-last-minute = ultimo minuto
hd-waiting-locked-signal = in attesa di un segnale agganciato
hd-mer-l = MER L
hd-modulation-error-ratio-lower-digital = Rapporto di errore di modulazione della banda laterale digitale inferiore.
hd-mer-u = MER U
hd-modulation-error-ratio-upper-digital = Rapporto di errore di modulazione della banda laterale digitale superiore.
hd-cber = CBER
hd-channel-bit-error-ratio-after = Rapporto di errore sui bit di canale dopo il codice interno. Sotto circa 1e-3 il codice esterno può correggerlo; sopra, l'audio comincia a spezzarsi.
hd-offset = OFFSET
hd-residual-carrier-frequency-offset-large = Offset residuo di frequenza della portante. Grande e costante significa che il riferimento del ricevitore è fuori, non la trasmissione.
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = Indicatore primario del modo di servizio: quale dei modi di servizio ibridi FM la stazione sta trasmettendo.
hd-programme = PROGRAMMA
hd-restricted-access = {" "}— accesso limitato

## bands (src/app/bands.rs)

bands-bands = BANDE
bands-daylight = ☀ GIORNO
bands-night = ☾ NOTTE
bands-no-forecast-yet-fetching-hamqsl = · nessuna previsione ancora — recupero da HAMQSL.com
bands-band = BANDA
bands-condx = CONDX
bands-paths = PERCORSI
bands-reach = PORTATA
bands-best = MIGLIORE
bands-ibp-beacons = RADIOFARI IBP
bands-meteor-showers = PIOGGE DI METEORE
bands-peak = PICCO

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = SPOT
spots-longwave-shortwave-broadcast-stations-air = Stazioni di radiodiffusione in onde lunghe e corte in onda ora
spots-view = IN VISTA
spots-only-spots-inside-panadapter-span = Solo gli spot entro l'ampiezza del panadapter
spots-setup = ⚙ CONFIGURA
spots-feeds-lookup-upload-settings = Flussi, ricerca e impostazioni di invio
spots-call-station-site-frequency = nominativo, stazione, sito, frequenza
spots-clear-search = Cancella la ricerca
spots-no-match = nessuna corrispondenza
spots-no-spots-enable-feed-setup = nessuno spot — attiva un flusso in ⚙ CONFIGURA
spots-nothing-matches-search = nulla corrisponde alla ricerca

## ism (src/app/ism.rs)

ism-ism-devices = DISPOSITIVI ISM
ism-decoding = DECODIFICA
ism-needs-wideband-iq-source-cat = Richiede una sorgente IQ a banda larga; un apparato CAT invia solo audio
ism-how-far-burst-must-stand = Di quanto un burst deve emergere sopra il rumore di fondo proprio del canale prima di essere decodificato
ism-give-each-band-width-normally = Dai a ogni banda la larghezza di cui di solito ha bisogno — un quarto di megahertz per le bande OOK a 315, 345 e 433 MHz, un megahertz intero per 868 e 915.
ism-several-hundred-more-device-decoders = Altre centinaia di decodificatori di dispositivi, dal progetto rtl_433, incluse. Legge i dispositivi OOK a 433 MHz che i decodificatori propri di sdroxide non raggiungono affatto. Dove entrambi possono leggere un dispositivo, rtl_433 conosce molte più varianti e prende il sopravvento.
ism-t-1-your-own = 1 proprio
ism-device-decoders-registered-how-many = Decodificatori di dispositivi registrati, quanti vengono dal tuo rtl433_flex.conf, e quante trasmissioni sono state decodificate da quando è partito.
ism-reload-decoders = RICARICA DECODIFICATORI
ism-re-read-rtl433-flex-conf = Rilegge rtl433_flex.conf dopo averlo modificato. I dispositivi già sentiti restano nell'elenco.
ism-rtl-433-knows-more-variants = rtl_433 conosce più varianti di queste rispetto ai decodificatori propri di sdroxide, quindi le legge mentre ascolta qui. Seleziona un'altra banda e tornano a prevalere quelli propri di sdroxide.
ism-decoder-your-rtl433-flex-conf = Un decodificatore nel tuo rtl433_flex.conf è stato rifiutato. Gli altri sono stati caricati. Correggi la riga, poi premi RICARICA DECODIFICATORI.
ism-waiting-decoder = in attesa del decodificatore…
ism-where-decoder-actually-listening-follows = Dove il decodificatore sta davvero ascoltando. Segue il centro hardware del ricevitore, non il VFO — su un front-end largo i due possono distare molto, ed è la finestra a decidere quali canali sono raggiungibili.
ism-transmissions-gate-opened-how-many = Le trasmissioni su cui il cancello si è aperto, e quante hanno prodotto un frame valido. Molti burst e nessuna decodifica significa che la banda è occupata da dispositivi che sdroxide non sa ancora leggere. Nessuno affatto significa che nulla raggiunge la soglia — abbassala, oppure verifica che la sintonia sia sulla banda.
ism-nothing-heard-yet = niente di sentito ancora
ism-click-tune-receiver =


    Click to tune the receiver to it

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

navtex-messages = MESSAGGI
navtex-sync = SINCRO
navtex-character-phase-locked-signal-being = La fase dei caratteri è agganciata — si sta leggendo un segnale.
navtex-characters-whose-first-copy-was = Caratteri la cui prima copia era corrotta e sono stati presi dalla ripetizione cinque slot dopo, e quelli in cui entrambe erano errate. Un carattere NAVTEX è inviato due volte; questa è tutta la sua correzione d'errore.
navtex-swap-mark-space-tones-signal = Scambia i toni di marcatura e spazio, per un segnale ricevuto sull'altra banda laterale. Disattivato è la banda laterale superiore sulla frequenza del canale, che è ciò che indica ogni istruzione di sintonia pubblicata per il servizio.
navtex-starting-navtex-receiver = avvio del ricevitore NAVTEX…
navtex-nothing-yet-station-transmits-ten = ancora nulla — una stazione trasmette per dieci minuti ogni quattro ore
navtex-cut-short = interrotto
navtex-message = MESSAGGIO
navtex-time-day-stated-message-utc = Ora del giorno indicata nel messaggio (UTC): solo visualizzazione
navtex-receiving = RICEZIONE
navtex-monitor = MONITOR
navtex-everything-decoded-message-not = tutto ciò che è decodificato, messaggio o no

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = RF PAINT
rfpaint-scan-speed = Velocità di scansione
rfpaint-how-fast-text-image-scanned = Quanto velocemente il testo o l'immagine viene scansionata sulla waterfall. Più basso è più lento e più leggibile; 100% = velocità base, 25% (centro) è il predefinito.
rfpaint-abort = Annulla
rfpaint-text-paint = TEXT PAINT
rfpaint-type-text-paint = Scrivi il testo da dipingere…
rfpaint-preview-waterfall = ANTEPRIMA WATERFALL
rfpaint-type-text-preview = scrivi del testo per l'anteprima
rfpaint-transmit = {"  "}TRASMETTI{"  "}
rfpaint-image-paint = IMAGE PAINT
rfpaint-no-image-loaded = nessuna immagine caricata
rfpaint-load-image = Carica immagine…
rfpaint-load-image-preview = carica un'immagine per l'anteprima

## hfdl (src/app/hfdl.rs)

hfdl-listen = ASCOLTA
hfdl-switches-engine-s-hfdl-downconverter = Attiva o disattiva il convertitore a discesa e il decodificatore HFDL del motore. La decodifica costa una corsia da 24 kHz e un thread di lavoro, che la finestra sia aperta o no.
hfdl-channel-assigned-frequency-listen-anything = Il canale (la frequenza assegnata) su cui ascoltare. Va bene qualunque punto della banda — le stazioni HFDL trasmettono da 2,8 a 22 MHz — contro una corsia fissa da 24 kHz centrata qui.
hfdl-save-hfdl-decode-log-file = Salva il registro delle decodifiche HFDL su un file
hfdl-riverhead-northern-atlantic-north-america = Riverhead (Atlantico settentrionale / Nord America) — il predefinito, e il canale su cui il decodificatore è stato validato.
hfdl-hfdl-assigned-frequency = Una frequenza assegnata HFDL.
hfdl-one-your-saved-channels-click = Uno dei tuoi canali salvati — clicca per sintonizzare, clic destro per rimuovere.
hfdl-hfdl-selected-but-decoding-off = HFDL è selezionato ma la decodifica è disattivata — premi ASCOLTA qui sopra
hfdl-decodes = DECODIFICHE
hfdl-nothing-decoded-yet-ground-station = Nulla di decodificato ancora — il squitter di una stazione di terra si ripete ogni ~32 s una volta che è nel canale.
hfdl-no-decode-matches-filter = Nessuna decodifica corrisponde al filtro.
hfdl-decoding = DECODIFICA
hfdl-decoding-off = DECODIFICA DISATTIVATA
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = Voce digitale FreeDV V1
rade-tx = ● TX
rade-mute-analog = MUTA ANALOGICO
rade-mute-demodulated-audio-so-only = Silenzia l'audio demodulato, così si sente solo il parlato decodificato. Altrimenti il segnale grezzo passa ogni volta che il modem non ha nulla da riprodurre — quel fruscio è il modo di individuare una trasmissione prima che si sincronizzi, quindi lascialo disattivato durante l'accordo.
rade-sync = SINCRO
rade-how-far-received-signal-sits = Di quanto il segnale ricevuto si trova lontano da dove il modem se lo aspetta. Valori grandi si decodificano comunque — il ciclo di acquisizione li segue — ma spostare un po' la sintonia per portarlo vicino allo zero dà il margine migliore.
rade-snr = SNR —
rade-callsign-station-put-end-over =
    Il nominativo che la stazione ha inserito nel suo frame di fine trasmissione — l'unico punto di una trasmissione RADE in cui ne viene inviato uno. È anche ciò che viene riportato a FreeDV Reporter, quindi una stazione che senti compare su qso.freedv.org come sentita da te.

    Held for a minute after the signal goes, and dropped when you start an over of your own: it names the station on frequency, not every station the receiver has ever heard.
rade-stop-talking = SMETTI DI PARLARE
rade-talk-ptt = PARLA (PTT)
rade-open-close-rade-over-modem = Apre o chiude una trasmissione RADE. Il modem ha bisogno di circa 120 ms di parlato prima che parta il primo frame, e invia un frame di fine trasmissione quando smetti, quindi la trasmissione prosegue un po' oltre il pulsante.
rade-rade-v1-occupies-roughly-1060 = RADE V1 occupa circa 1060–1880 Hz della banda passante — banda laterale superiore, oppure inferiore su 160, 80 e 40 m, dove RADE mantiene la pratica in fonia e sdroxide commuta per te. Metti il segnale dentro la banda ombreggiata sulla waterfall; da lì il modem lo trova.

## frame (src/app/frame.rs)

frame-acknowledge = Conferma
frame-dismiss = Chiudi
frame-reconnecting = Riconnessione…
frame-not-connected = Non collegato.
frame-reconnect-now = Riconnetti ora
frame-transmit-lockout-disabled = ⚠ BLOCCO DI TRASMISSIONE DISATTIVATO
frame-engine-was-started-oob-tx = Questo motore è stato avviato con --oob-tx. Il blocco sulle bande radioamatoriali è disattivato: attiverà il trasmettitore su qualunque frequenza l'hardware supporti.
frame-transmitting-outside-your-licence-offence = Trasmettere fuori dalla propria licenza è un reato in ogni paese che ne rilascia una. Continua solo se sei autorizzato a usare le frequenze su cui stai per attivare — una licenza MARS/CAP o commerciale, un permesso sperimentale, oppure un carico fittizio.
frame-i-understand = {"  "}HO CAPITO{"  "}
frame-restart-without-oob-tx-put = Riavvia senza --oob-tx per ripristinare il blocco.

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC — quanto è pilotato il trasmettitore, in percentuale della massima potenza che può emettere. 100 % è il fondo scala, non la piena potenza in watt: quanto valga in antenna dipende dalla radio e dal Pilotaggio.
smeter-radio-reports-no-forward-power =


    This radio reports no forward power, so the figure beside ALC is the ALC reading again rather than watts.
smeter-no-swr-radio-has-no =


    No SWR: this radio has no bridge to measure it with, and sdroxide will not invent one — it can only show what the radio itself reports. A rig with an SWR meter (over CAT, TCI or a LAN link) and an HPSDR board both fill this in; a plain SDR transmitter cannot.
smeter-no-signal-report-yet-receiver = Nessun rapporto di segnale ancora — il ricevitore non ha fornito una lettura dello strumento.
smeter-s9-73-dbm-each-s =


    S9 is −73 dBm and each S-unit below it is 6 dB. The dBm figure is only as honest as the front end's calibration — an uncalibrated receiver reports dBFS with a dBm label.
smeter-ovld-converter-into-rails-so =


    OVLD: the converter is into its rails, so the level shown understates the signal. Wind the front-end gain back.
smeter-table-held = , tabella bloccata
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● TX
multi-link = Potenza
multi-mute-radio-s-audio = Silenzia l'audio di questa radio
multi-close-radio-s-split-view = Chiudi la vista divisa di questa radio
multi-open-radio-split-view-own = Apri questa radio in una vista divisa propria
multi-already-open-another-split-view = Già aperta in un'altra vista divisa
multi-show-radio-here = Mostra questa radio qui
multi-add-radio-computer-settings-radio = Aggiungi una radio su questo computer (per aggiungerne una a una stazione, usa Impostazioni → Radio)
multi-add-radio = Aggiungi una radio
multi-asked-station-another-radio = Chiesta alla stazione un'altra radio…
multi-client-cannot-open-connection = Questo client non può aprire una connessione.

## net (src/app/net.rs)

net-fail = ERRORE
net-downloading = Download in corso…
net-adif-import-failed-file-could = Importazione ADIF non riuscita: il file non può essere letto
net-channel-import-failed-file-could = Importazione dei canali non riuscita: il file non può essere letto
net-channel-import-nothing-read-chirp = Importazione dei canali: nulla da leggere — un file CSV di CHIRP inizia con una riga di intestazione che nomina le colonne, una delle quali deve essere Frequency

## widgets (src/app/panels/widgets.rs)

widgets-free-text = testo libero
widgets-hashed-callsign-not-yet-heard = nominativo con hash, non ancora sentito per intero
widgets-entity-unknown = entità sconosciuta
widgets-band = questa banda
widgets-new-entity-never-worked-any = Nuova entità — mai lavorata, su nessuna banda
widgets-new-grid-square = Nuovo quadrato locatore
widgets-not-worked-before = Non lavorato prima
widgets-worked-before-but-not-band = Lavorato prima, ma non su questa banda
widgets-call-queue = Nella coda delle chiamate
widgets-image = Immagine

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = RICEZIONI
pi4-save-pi4-reception-list-csv = Salva l'elenco delle ricezioni PI4 in CSV
pi4-next-generation-beacon = Radiofaro di nuova generazione
pi4-time-left-one-minute-beacon = Tempo rimasto in questo ciclo di radiofaro da un minuto. Il messaggio PI4 viene cercato quando la finestra ha avuto il tempo di riempirsi oltre il punto in cui finirebbe.
pi4-decoding = decodifica…
pi4-band = Banda
pi4-last-cycle = Ultimo ciclo
pi4-nothing-heard = nulla di sentito
pi4-receive-only-tune-so-beacon = Solo ricezione. Sintonizza in modo che l'identificazione CW del radiofaro cada a 800 Hz audio — la convenzione della rete stessa — e i quattro toni PI4 cadano dove questo decodificatore li cerca per impostazione predefinita.

## awards (src/app/awards.rs)

awards-awards = PREMI
awards-band = Banda
awards-all = Tutte
awards-dxcc = DXCC
awards-grids = Locatori
awards-worked-all-states = Worked All States
awards-cq-zones-waz = Zone CQ (WAZ)
awards-dxcc-entities = Entità DXCC

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = Audio nella banda passante del decodificatore.
acars-blocks-framed-blocks-whose-check = Blocchi incorniciati, e blocchi la cui sequenza di controllo è fallita.
acars-starting-acars-receiver = avvio del ricevitore ACARS…
acars-channel = CANALE
acars-tune-dial-here = Sintonizza qui la sintonia.

acars-channel-out-of-range = Fuori dalla gamma di ricezione di questa radio.
acars-nothing-yet-acars-quiet-between = Ancora nulla. ACARS è silenzioso tra un aereo e l'altro — lascialo su un canale e aspetta un burst.
acars-check-failed = controllo fallito
acars-block-check-sequence-did-not = La sequenza di controllo del blocco non corrisponde — il testo potrebbe essere sbagliato.
acars-no-text = (nessun testo)

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = Nome utente
login-password = Password
login-remember-device = Ricorda su questo dispositivo
login-signs-without-asking-next-time = Accede senza chiedere la volta successiva, e fa entrare la scheda della vista 3D senza chiedere affatto. La password viene conservata in chiaro, quindi lascia questo disattivato su una macchina che usano altre persone.
login-keeps-password-device-clear = Conserva la password su questo dispositivo, in chiaro.
login-checking = {" "}VERIFICA…{" "}
login-sign = {" "}ACCEDI{" "}
login-wrong-password-shuts-server-s = Una password sbagliata chiude la porta del server per alcuni secondi, quindi può volerci un momento.

## help (src/help.rs)

help-find = CERCA
help-find-manual = cerca nel manuale
help-previous-match-shift-enter-shift = Risultato precedente (Maiusc+Invio, Maiusc+F3)
help-next-match-enter-f3 = Risultato successivo (Invio, F3)
help-no-matches = nessun risultato
help-clear-search-esc = Cancella la ricerca (Esc)
help-sdroxide-manual = MANUALE DI SDROXIDE
help-contents = INDICE

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = 1 stazione sentita — in attesa che riporti una posizione
aismap-double-click-reframe = doppio clic per reinquadrare
aismap-zoom-out-wheel-pinch-do = Zoom indietro — anche la rotella o il pizzico fanno questo
aismap-zoom-about-middle-chart = Zoom avanti sul centro della carta
aismap-frame-everything-being-tracked-again = Inquadra di nuovo tutto ciò che viene tracciato — anche un doppio clic lo fa

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = in attesa dello spettro…
specview-hounds = HOUNDS

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = Questa finestra non ha un renderer GPU, quindi la waterfall resta alla sua larghezza standard.
spectrum-machine-drawing-without-gpu-every = Questa macchina disegna senza GPU — ogni colonna della waterfall è lavoro del processore, che lo condivide già con la radio.
spectrum-window-renders-through-opengl-which = Questa finestra disegna tramite OpenGL, che è il percorso di compatibilità di sdroxide — un Raspberry Pi, un vecchio chip grafico, o un browser senza WebGPU. Lì una waterfall più larga non vale il frame rate.
spectrum-wider-than-renderer-can-draw = Più larga di quanto questo renderer possa disegnare.

## macros (src/app/panels/macros.rs)

macros-nocall = NOCALL
macros-remove-button = Rimuovi questo pulsante

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = GRID TRACKER
gridtracker-drag-pan-wheel-zoom-amber = Trascina per spostarti, rotella per lo zoom. Ambra = lavorato, verde = confermato, ciano = sentito.

## remote (src/remote.rs)

remote-server-busy-another-client-connected = server occupato — un altro client è collegato
remote-transmitting-but-client-s-microphone = In trasmissione, ma il microfono di questo client non produce alcun audio — non viene modulato nulla. In un browser, verifica che alla pagina sia stato concesso il microfono e che nessun'altra applicazione lo tenga occupato.
remote-connection-closed = connessione chiusa

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = Vista 3D del sistema solare — Sole, Terra, Luna, macchie solari e CME (finestra separata)
solarbtn-solar-system-3d-view-sun-2 = Vista 3D del sistema solare — Sole, Terra, Luna, macchie solari e CME (nuova scheda del browser)

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = Caricamento della voce…
speech-browser-client-cannot-speak-yet = il client nel browser non può ancora parlare

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = Doppio clic per scrivere una frequenza
freqdisplay-tap-type-frequency = Tocca per scrivere una frequenza

## solarstate (src/solar3d/state.rs)

solarstate-home = HOME
solarstate-dwarf-planets = PIANETI NANI

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = doppio clic per reinquadrare

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = doppio clic per reinquadrare

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = connessione chiusa

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = impossibile avviare il thread degli avvisi

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = nessun aereo localizzato ancora

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = DOPPIO CLIC PER REINQUADRARE

## chrome (src/chrome.rs)



## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = Imposta la potenza propria dell'FDM-DUO (menu 34 TX POWER). Pilotaggio e Accordo impostano poi la potenza al suo interno, e SDRoxide invia di nuovo l'impostazione ogni volta che si collega.

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $day } { $month ->
        [1] gen
        [2] feb
        [3] mar
        [4] apr
        [5] mag
        [6] giu
        [7] lug
        [8] ago
        [9] set
        [10] ott
        [11] nov
       *[12] dic
    }

## download (src/download.rs)

download-browser-could-not-read-file = il browser non è riuscito a leggere quel file

## morse (src/app/morse.rs)

morse-translate = TRADUCI
morse-practice = ESERCIZIO
morse-learn = IMPARA
morse-morse = MORSE
morse-text-morse = Testo → Morse
morse-morse-text = Morse → Testo
morse-per-character-space-between-characters = `.` e `-` per carattere, uno spazio tra i caratteri, ` / ` tra le parole. L'alfabeto ITU, le cifre, la punteggiatura e i prosigni comuni.
morse-reference-translator-available-desktop-build = Il traduttore di riferimento è disponibile nella versione desktop.
morse-play-what-you-type-your = Riproduci ciò che scrivi, sui tuoi altoparlanti
morse-play = RIPRODUCI
morse-stop = STOP
morse-every-character-learned-well-done = Ogni carattere imparato — ben fatto.
morse-reveal = MOSTRA
morse-reset = RIPRISTINA
morse-replay = RIASCOLTA
morse-what-did-you-hear = Cosa hai sentito?
morse-check = VERIFICA
morse-press-new-hear-character = Premi NUOVO per sentire un carattere.
morse-tone = Tono
morse-speed = Velocità
morse-spacing = Spaziatura
morse-characters-go-out-speed-above = I caratteri partono alla velocità qui sopra; questa è la velocità con cui vengono inviati gli *intervalli*, così un principiante sente la lettera nel suo insieme prima di poterla inviare a quella velocità.
morse-playback-available-desktop-build = La riproduzione è disponibile nella versione desktop.

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = altitudine
adsb-row-track = rotta
adsb-row-turning = in virata
adsb-row-squawk = squawk
adsb-row-range = distanza
adsb-row-position = posizione
adsb-row-signal = segnale
adsb-row-frames = frame
rade-transmitting = in trasmissione
decodes-odd = DISPARI
decodes-new = NUOVA
js8-new = NUOVA

## AIS panel: vessel detail rows.

ais-row-status = stato
ais-row-speed = velocità
ais-row-course = rotta
ais-row-heading = prua
ais-row-turning = in virata
ais-row-altitude = altitudine
ais-row-size = dimensione
ais-row-draught = pescaggio
ais-row-destination = destinazione
ais-row-range = distanza
ais-row-signal = segnale
ais-row-type = tipo
ais-row-position = posizione
ais-row-messages = messaggi
ism-off = OFF
textmodem-frequency-shifted = a spostamento di frequenza
winlink-to-heading = A

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = { $heard } sentite — nessuna posizione più recente di { $drop_map_s }s
aismap-stations-heard-waiting-position-report = { $heard } stazioni sentite — in attesa di un rapporto di posizione
aismap-km-across = { $lon_span }° · ~{ $km } km di larghezza

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = allarme su { $device }
alerts-alarm-device-unavailable = dispositivo di allarme non disponibile: { $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = { $w } lavorati
awards-confirmed = { $value } confermati

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · previsione da HAMQSL.com, di { $value }
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · prossimo radiofaro tra { $now } s
bands-radiants-placed = radianti posizionati per { $lat }°, { $lon }°
bands-zhr = ZHR { $zhr }
bands-radiant = radiante { $alt_deg }° { $az_deg }
bands-radiant-down = radiante sotto ({ $alt_deg }°)

    A radiant above the horizon means the trails can reach you; a fast shower leaves longer-lived ionised trails for meteor scatter on 6 m and 2 m, and the brief 10 m openings.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } non decodificato
drm-symbols = { $value } simboli · { $frame }
drm-not-decodable = { $value } — non decodificabile
drm-broadcaster-s-clock-utc = orologio del trasmettitore { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    MAX: la potenza piena dell'FDM-DUO, circa 8 W. L'impostazione da usare con l'HR500 in linea.

    { $base }
duopower-t-5-w-hr500-line-will =
    5 W. Con l'HR500 in linea a questa impostazione non raggiungerà la piena potenza sulla maggior parte delle bande: usa MAX, e Pilotaggio per abbassarla.

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } W, per lavorare senza amplificatore o in QRP.

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = Riabilita la trasmissione dopo { $swr }:1. Controlla prima l'antenna: se il guasto c'è ancora, anche la trasmissione successiva si ferma.
frame-reconnecting-s = Riconnessione tra { $value } s…

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = { $worked } lavorati
gridtracker-confirmed = { $confirmed } confermati
gridtracker-heard = { $heard } sentiti

## hd: messages with values (src/app/hd.rs)

hd-decode = Decodifica { $value } ({ $codec_label })

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } kHz ha già un chip
hfdl-save-current-channel-khz-one = Salva il canale attuale ({ $cur_khz } kHz) come uno dei tuoi chip
hfdl-aircraft = { $aircraft } aerei
hfdl-bursts = { $bursts } burst
hfdl-decodes-2 = { $decodes } decodifiche
hfdl-fixed = { $fec } corretti

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint } — non ancora decodificato
ism-listen-tunes-radio-there-one = Ascolta su { $value } — sintonizza lì la radio. Una banda alla volta: sono troppo distanti perché un ricevitore ne copra due insieme.
ism-watch-hz-around-band-centre = Osserva { $value }Hz attorno al centro della banda, qualunque cosa la banda avrebbe richiesto. Più stretto va bene per un ricevitore che non può fornire la larghezza propria della banda; più largo raggiunge dispositivi più lontani dal centro. Il tuo ricevitore deve fornirne circa un terzo in più di questo.
ism-rtl-433 = rtl_433 — { $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } kHz
ism-your-own = { $flex } propri
ism-decoders-decoded = { $decoders } decodificatori ({ $flex }), { $decodes } decodificate
ism-handled-rtl-433 = gestiti da rtl_433: { $names }
ism-flex = flex: { $error }
ism-window-mhz-mhz-wide = finestra { $mhz } MHz, larga { $e62 } MHz
ism-bursts-decoded = { $bursts } burst, { $decodes } decodificati
ism-devices = { $ism_reports } dispositivi
ism-id-mhz-db-heard-first =
    { $protocol } id { $device }
    { $e6 } MHz { $snr_db } dB sentito { $value }×
    primo ascolto { $first_at } fa

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = Conferme: { $recs } scaricate, { $matched } appena confermate
logbook-qso = { $qso_log } QSO
logbook-worked-before = ⚠ GIÀ LAVORATO ({ $dupe_band })
logbook-utc-qso = { $oldest }–{ $newest } UTC · { $value } QSO

## app: messages with values (src/app/mod.rs)

app-radio = radio { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = impossibile avviare il thread audio Morse: { $error }
morse-no-audio-output = nessuna uscita audio: { $error }
morse-about-s = circa { $farnsworth_wpm } s
morse-characters-play = Caratteri in gioco: { $set }
morse-score-run = Punteggio: { $correct } / { $total }. Serie: { $streak } di { $value }.
morse-correct = corretto — { $value }
morse-was = era { $value }

## net: messages with values (src/app/net.rs)

net-adif-import-failed = Importazione ADIF non riuscita: { $error }
net-not-unicode-read = {" "}(non Unicode; letto come { $enc })
net-received-reports-swl-left-out = , { $n } rapporti di ricezione (SWL) esclusi
net-adif-import-added-duplicates-skipped = Importazione ADIF: { $added } aggiunti, { $skipped } duplicati saltati{ $reports }{ $assumed }
net-channel-import-failed = Importazione dei canali non riuscita: { $error }
net-line-s-skipped = , { $n } riga/e saltata/e
net-not-unicode-read-2 = {" "}(non Unicode; letto come { $enc })
net-channel-import-channel-s-read = Importazione dei canali: { $n } canale/i letto/i{ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = { $frames } frame · { $bad } errati
acars-mode = modo { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = { $aircraft } aerei
adsb-frames = { $frames } frame
adsb-preambles = { $preambles } preamboli
adsb-bad-crc = { $bad_crc } CRC errati
adsb-tune = SINTONIZZA { $mhz }
adsb-ft-barometric = { $ft } ft barometrici

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = { $vessels } navi
ais-messages = { $messages } messaggi
ais-slots = { $bursts } slot
ais-bad-fcs = { $bad_fcs } FCS errati
ais-ais-mhz = AIS { $value } ({ $mhz } MHz) — { $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value } ({ $mhz } MHz) — { $bursts } slot, { $messages } messaggi, fondo { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = SINTONIZZA { $mhz }
ais-ships-heard-are-khz-off = le navi sentite sono fuori frequenza di { $khz } kHz ({ $mhz } ppm a 162 MHz) — imposta la correzione di frequenza del front-end, oppure le trasmissioni oltre circa 5 kHz smetteranno di decodificarsi
ais-over-ground = { $value }° sul fondo
ais-dbfs-db-snr = { $rssi_dbfs } dBFS, { $snr_db } dB SNR
ais-last-type-ais = { $messages } (ultimo tipo { $last_type }, AIS { $channel })
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } MHz — canale marittimo { $ch }. Una nave alterna i due, quindi disattivarne uno dimezza quanto spesso viene sentita ogni nave.

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = oggetto da { $reported_by }
aprs-via = via { $via }
aprs-object-reported = oggetto riportato da { $reported_by }
aprs-digit-s-blanked = ± { $ambiguity } cifra/e oscurata/e
aprs-wind-m-s = vento { $value }° { $value2 } m/s
aprs-gust = raffica { $value }
aprs-rain-mm-h = pioggia { $value } mm/h
aprs-frame-s-last-ago = { $packets } frame, l'ultimo { $last_heard } fa
aprs-via-2 = via { $via }
aprs-bad = { $bad_frames } errati
aprs-other = { $non_aprs } altri
aprs-sent = inviato ×{ $tries }
aprs-stn = { $stations } staz
aprs-queued = { $tx_queue } in coda

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = master { $value }
atchat-last-heard-s-ago-click = { $status } — ultimo ascolto { $age_s }s fa · clicca per aprire una scheda di messaggi diretti
atchat-close-tab = chiudi la scheda di { $peer }
atchat-direct-message = messaggio diretto a { $value }
atchat-cannot-display = impossibile mostrare { $path }

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Click for the { $neural } decoder. The neural one copies further down and reads hand-sent CW a timing fit will not accept; the timing one is the only one that copies the accented letters — Ä, Ö, Å, Ü, É — because the model has no output class for them.
cw-idle-s = INATTIVO { $idle }s

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = Salva le { $n } decodifiche nell'elenco come ADIF — rapporti ricevuti, non contatti. Le decodifiche che non indicano un mittente vengono escluse.
decodes-save-decodes-list-csv-one = Salva le { $n } decodifiche nell'elenco come CSV, una riga ciascuna
decodes-session-qso = Sessione: { $session } QSO
decodes-save-whole-logbook-qso-adif = Salva l'intero logbook ({ $logged } QSO) come ADIF
decodes-save-whole-logbook-qso-text = Salva l'intero logbook ({ $logged } QSO) come testo
decodes-hz-slots = { $audio_hz } Hz · { $odd } slot
decodes-your-slot-timing-against-stations =
    La temporizzazione dei tuoi slot rispetto alle stazioni che stai sentendo.
    { $after }

    It covers the whole receive path, so a slow audio or network chain counts the same as a wrong clock. Under 0.5 s is comfortable.
decodes-pile-up = PILE-UP { $fox_queue }
decodes-queue = CODA { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } dB · { $audio_hz } Hz{ $unwrap_or_default }
    Clicca per rimuovere

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = Ogni velocità viene decodificata; ▸ segna { $speed }, quella su cui trasmetti
js8-every-speed-being-decoded-click = Ogni velocità viene decodificata. Clicca per trasmettere invece a { $speed }
js8-transmit-decode = Trasmetti e decodifica a { $speed }
js8-beaconing-every-min-click-stop = Radiofaro ogni { $hb_min } min — clicca per fermare
js8-last-beacon-went-out-hz = L'ultimo radiofaro è partito a { $hz } Hz — uno slot libero nella sotto-banda heartbeat da { $value }–{ $value2 } Hz, scelto perché cada lontano dai segnali decodificati.
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } frame · { $left }s
js8-decoded = Decodificato a { $speed }
js8-frames = … ({ $frames } frame)
js8-reply-2 = Rispondi a { $value }: “{ $value2 }”
js8-address-composer = Indirizza il compositore a { $value }

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i }: invia “{ $text }”
macros-sends = Invia “{ $text }”
macros-most = { $value } è il massimo

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = { $repaired } riparati · { $lost } persi
navtex-characters-lost = { $lost } caratteri persi

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = { $bad_frames } errati
packet-link-layer = Il livello di collegamento è in { $link }
packet-via = {" "}via { $via }
packet-unacked = { $unacked } non confermati
packet-retry = tentativo { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] radiofaro
       *[other] radiofari
    }
pi4-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } fa

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = SNR { $snr_db } dB
rade-offset-hz = offset { $freq_offset_hz } Hz
rade-heard = sentito { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ { $dropped } campioni persi tra il ricevitore e il decodificatore — questa macchina non sta al passo con la decodifica neurale.

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } car/s
textmodem-char-s-hz-wide = { $value } — { $chars_per_sec } car/s, larga { $bandwidth_hz } Hz, { $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = { $frames } frame
vdl2-bursts = { $bursts } burst
vdl2-sync = { $syncs } sincro
vdl2-hdlc-bad = { $hdlc_bad } HDLC errati
vdl2-bad-fcs = { $fcs_bad } FCS errati
vdl2-mhz-bursts-frames-noise-floor = { $mhz } MHz, { $what } — { $bursts } burst, { $frames } frame, rumore di fondo { $floor_dbfs } dBFS
vdl2-tune = SINTONIZZA { $mhz }
vdl2-held-2 = BLOCCATO +{ $since }
vdl2-use-worldwide-where-every-link = { $what } — in uso in tutto il mondo, ed è dove inizia ogni collegamento. Quello da tenere se ne tieni uno solo.
vdl2-assigned-which-channels-carry-anything = Assegnato a un { $what }. Quali canali portano qualcosa dipende da dove sei: lasciali tutti attivi a meno che tu non sappia altrimenti.
vdl2-mhz-db-snr-dbfs-evm = { $mhz } MHz · { $snr_db } dB SNR · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Hz · { $rs_corrected } simboli RS corretti
vdl2-characters = { $parity_errors } caratteri
vdl2-parameter-sdroxide-does-not-read = { $unknown } { $unknown ->
        [one] parametro
       *[other] parametri
    } che SDRoxide non legge — mostrati sopra come identificatore e byte

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = { $live_size } righe trattenute
wefax-pixels-per-line = { $width } pixel per riga
wefax-shift-picture-pixels = Sposta l'immagine di { $px } pixel
wefax-charts-are-saved-click-copy =
    Le carte vengono salvate { $place }

    Click to copy the path
wefax-finished-charts-are-saved-collect = Le carte completate vengono salvate { $place } e si raccolgono qui.
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } pixel
    { $value }

    Right-click to delete
wefax-older-store = { $older } più vecchie nell'archivio
wefax-older-load-more = { $older } più vecchie — carica altre
wefax-published-carrier-khz-dial-khz = Portante pubblicata { $f } kHz → sintonia { $value } kHz USB
wefax-delete-chart-2 = Elimina questa carta { $dir }

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · zona CQ { $cq_zone } · zona ITU { $itu_zone }
widgets-new-entity = Nuova entità su { $band_label }
widgets-worked-before = Lavorato prima su { $band_label }
widgets-calling-cq-includes-you = Chiama CQ { $target } — include te
widgets-calling-cq-not-aimed-you = Chiama CQ { $target } — non rivolto a te

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = { $heard_us } rx · { $heard_us2 } ci hanno sentiti
wspr-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] radiofaro
       *[other] radiofari
    }
wspr-beaconing-duty = radiofaro — { $wspr_power_dbm }, ciclo { $wspr_tx_percent }%
wspr-listening-duty = in ascolto (ciclo { $wspr_tx_percent }%)
wspr-beacon-one-two-minute-slot = Radiofaro in uno slot da due minuti su ogni { $pct } — il { $pct2 }% di essi, distribuiti in modo uniforme — a { $wspr_power_dbm }. Lo slot in cui inizia il ciclo è scelto dal tuo nominativo, così due stazioni che usano questo programma non partono insieme.
wspr-transmitting-set-general-tab-settings = Trasmette come { $call } in { $sent } — impostato nella scheda Generale delle Impostazioni.
wspr-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } fa

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    antenna: { $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = { $entries } ricevitori · { $per_net } · scaricati { $age } fa
publicsdr-showing-search-narrow = mostrati { $shown } di { $n } — cerca per restringere
publicsdr-match = { $n } corrispondenza
publicsdr-replace-taking-receiver-points-same =
    Sostituire { $this } con { $entry }?

    { $this2 } is on { $iface }. Taking this receiver points the same tab at { $address } instead, renames it “{ $entry2 }”, and holds the dial to the { $range_label } the receiver publishes — nothing here transmits.

    The { $iface2 } settings stay where they are, and so do the ranges stated for them: switching the interface back in Settings → Radio brings the radio back as it was.
publicsdr-receptions-will-reported = {" "}Le ricezioni verranno riportate da { $value }.
publicsdr-pointing-radio = Puntamento di questa radio su { $entry }…{ $site }
publicsdr-opening-another-radio = Apertura di { $entry } come un'altra radio…

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } kHz
qo100-before-switching-tune-so-beacon = Prima di attivare: sintonizza in modo che i due lobi del radiofaro cadano tra +{ $park_lo_hz } e +{ $park_hi_hz } kHz nella striscia (la corsia ombreggiata).
qo100-receiver-currently-listening-around-mhz = Il ricevitore sta ascoltando attorno a { $mhz } MHz — ben lontano dal radiofaro a 10489.750 MHz, quindi qui non c'è nulla da disegnare. Il decodificatore continua comunque a cercare.
qo100-receiver-currently-covers-only-khz = il ricevitore copre qui solo ±{ $khz } kHz — il resto della striscia resta vuoto
qo100-null-db-sym-snr-db = nullo { $est_null_depth_db } dB sim { $est_symmetry } snr { $est_snr_db } dB
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Hz/s { $est_drift_accel_hz_s2 } Hz/s² (derotato prima della decodifica)
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Hz/s (derotato prima della decodifica)
qo100-found-empty = { $est_updates } trovate, { $est_misses } vuote
qo100-last-s-ago = {"   "}ultimo { $auto_last_hz } ({ $auto_last_unix }s fa)
qo100-over-correction = { $auto_total_hz } su { $auto_applies } { $auto_applies ->
        [one] correzione
       *[other] correzioni
    }{ $last }
qo100-sync-32-err = sincro { $sync_bit_errors } / 32 err ×{ $sync_matches }
qo100-frame-buffer = buffer dei frame { $frame_fill }%
qo100-blocks-tried-decoded = { $blocks_tried } blocchi provati, { $blocks_locked } decodificati
qo100-last-applied-s-ago-hz = ultimo applicato { $ago }s fa: { $old } → { $new } Hz ({ $old2 } Hz)
qo100-locked-block-tried-locked = agganciato — { $blocks_tried } { $blocks_tried ->
        [one] blocco
       *[other] blocchi
    } provati, { $blocks_locked } agganciati
qo100-searching-block-tried-none-locked = ricerca in corso — { $blocks_tried } { $blocks_tried ->
        [one] blocco
       *[other] blocchi
    } provati, nessuno ancora agganciato

## rds: messages with values (src/app/rds.rs)

rds-auto = Automatico ({ $standard })
rds-type = tipo { $class }
rds-groups = · { $groups } gruppi
rds-blocks-error = · { $ber } % dei blocchi con errore
rds-clean-corrected-lost = { $blocks_ok } puliti · { $blocks_corrected } corretti · { $blocks_bad } persi

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC (ora locale { $offset })
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } sopra l'orizzonte
sat-min = tra { $value } min
sat-h-min = tra { $value } h { $value2 } min
sat-searching = ricerca… { $next } di { $entries }
sat-locked-2 = ● AGGANCIATO — { $t }
sat-pass-until-max = Passaggio fino alle { $set_unix } · massimo { $max_el }°
sat-next-pass-rises-max = Prossimo passaggio { $rise_unix } { $rise_unix2 } ({ $now }) · sorge a { $rise_az }° { $rise_az2 } · massimo { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ { $clients } { $clients ->
        [one] client rigctld collegato
       *[other] client rigctld collegati
    } — il software satellitare che sposta lì la sintonia correggerebbe il Doppler due volte
sat-rotator-antenna-az-el = Rotore: antenna a { $az }° az { $el }° el
sat-rotator-2 = Rotore: { $error }
sat-up-now-2 = ● visibile ora, { $el }°
sat-inv = { $mode } · inv
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = in attesa su { $here } MHz
scanner-scanning-mhz = scansione · { $here } MHz

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = Questo renderer non può contenere una texture più larga di { $max_texture_dim } pixel.

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = Voce: { $voice }
speech-speech-unavailable = Il parlato non è disponibile: { $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = { $stations } trasmissioni

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = ultima sessione non riuscita — { $err }
winlink-last-session-received-sent = ultima sessione: { $last_received } ricevuti, { $last_sent } inviati
winlink-empty = { $to_lowercase } è vuota
winlink-showing = mostrati { $entries } di { $total }
winlink-fetching = recupero di { $mid }…
winlink-from-2 = da { $msg } { $date }
winlink-attachment-bytes = allegato: { $att } ({ $data } byte)

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    oggetto da { $reported_by }
aprsmap-heard-ago =
    {""}
    sentito { $age } fa

## download: messages with values (src/download.rs)

download-reading = lettura di { $path }: { $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}immagine mancante: { $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } MHz
hfdlmap-fix =
    {""}
    { $fixes } fix{ $es }
hfdlmap-heard-ago =
    {""}
    sentito { $last_at } fa

## multi: messages with values (src/multi.rs)

multi-radio = Radio { $n }
multi-could-not-add-radio = Impossibile aggiungere una radio: { $error }
multi-was-closed-station = { $value } è stata chiusa sulla stazione.
multi-could-not-open = Impossibile aprire { $peer }: { $error }
multi-has-tab-own-now = { $value } ha ora una scheda propria.

## remote: messages with values (src/remote.rs)

remote-protocol-error = errore di protocollo: { $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = messaggio non valido: { $error }
solarnet-solar-protocol-mismatch-server-client = protocollo solare non corrispondente: server { $proto }, client { $value }

## smeter: messages with values (src/widgets/smeter.rs)

smeter-forward-power-w-measured-radio =
    {""}

    Forward power — { $w } W, measured by the radio's own sensor.
smeter-po-radio-s-own-power =
    {""}

    PO — the radio's own power-output meter, at { $clamp } % of its full scale. A needle position as the radio reports it, not a calibrated wattage; what to watch is whether it falls while Drive stays put.
smeter-swr-1-match-antenna-presenting =
    {""}

    SWR — { $swr }:1, the match the antenna is presenting, as the radio measures it. Under 2:1 is unremarkable; past 3:1 the scale turns red.
smeter-received-signal-strength-s9-db = Intensità del segnale ricevuto: S9 + { $over } dB ({ $s_dbm } dBm nella banda passante).
smeter-received-signal-strength-s-dbm = Intensità del segnale ricevuto: S{ $value } ({ $s_dbm } dBm nella banda passante).
smeter-radio-reports-c =
    {""}

    The radio reports { $t } °C.
smeter-ps-puresignal-correcting-db-compression =
    {""}

    PS — PureSignal is correcting { $correction_db } dB of compression{ $held } (the feedback matched the transmission at { $score }).
smeter-ps-puresignal-switched-but-has =
    {""}

    PS −− — PureSignal is switched on but has not found the transmission in the feedback path (best match { $score }), so the transmitter is going out uncorrected. Check that a sample of the amplifier's output really reaches a receive input the T/R switch does not take away on transmit, and that its attenuator is not swallowing it. The loop listens on this receiver only — a coupler wired to a second receiver (RX2, ADC2) is never read as the feedback.

## specview: messages with values (src/widgets/spectrum_view.rs)

specview-rit-hz = RIT { $hz } Hz
specview-xit-hz = XIT { $hz } Hz

## decodes: nested messages

decodes-you-transmit-s-everyone-else = Trasmetti { $off } s { $after } tutti gli altri — il motivo abituale per cui le chiamate restano senza risposta. Sincronizza l'orologio del computer.

## publicsdr: nested messages

publicsdr-noise-floor-score = punteggio del rumore di fondo { $snr }
publicsdr-up-khz-i-q = fino a { $khz } kHz di I/Q

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = GR gate { $gate_gr_db } de-ess { $deess_gr_db } comp { $comp_gr_db } lim { $limiter_gr_db }

## amp: more messages

amp-h-step = ~{ $l_step } µH · passo { $l_step2 }
amp-pf-step = ~{ $c_step } pF · passo { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel }-bit · { $media_type } / { $content_encoding } · { $encoded_size } ottetti in { $chunk_count } blocchi ({ $chunks_first_pass } al primo passaggio) · sessione { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value } — { $symbol_rate } baud CPFSK, ±{ $deviation_hz } Hz, larghezza di banda occupata { $bandwidth_hz } kHz

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · { $local } locale
sat-in-seconds = tra { $s } s
winlink-to-list = a { $to }
winlink-cc-list = cc { $cc }
spots-n-of-total = { $n } di { $total }
wefax-n-of-total = { $n } di { $total }
ais-range-bearing = { $km } km a { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = Disattivato (solo sintonizzatore, da 24 MHz in su)
rtl-sdr-hf-mode-auto = Automatico
rtl-sdr-hf-mode-direct-q = Campionamento diretto (ramo Q)
rtl-sdr-agc-manual = Manuale (nessun AGC)
rtl-sdr-agc-tuner = AGC del sintonizzatore
rtl-sdr-agc-rtl = AGC digitale RTL
rtl-sdr-agc-both = AGC sintonizzatore + RTL
mode-control-cat = CAT
mode-control-radio = Controllato dalla radio
line-state-none = Nessuna
line-state-high = Alta
line-state-low = Bassa
elad-tx-input-usb-audio = Audio USB
elad-tx-input-mic = Microfono
elad-tx-input-auto = Automatico
elad-tx-input-leave = Lascia come impostato sulla radio
tr-usdx-audio-one-cable = Un cavo (audio nel flusso CAT)
tr-usdx-audio-sound-card = Scheda audio USB (jack 3,5 mm)
spy-server-format-uint8 = 8 bit (minor larghezza di banda)
spy-server-format-int16 = 16 bit
spy-server-format-float32 = Float a 32 bit
sound-format-iq = IQ (stereo)
sound-format-demod-audio = Audio demodulato
sdr-play-duo-tuner-tuner1 = Sintonizzatore 1 (50 Ohm / Hi-Z)
sdr-play-duo-tuner-tuner2 = Sintonizzatore 2 (50 Ohm)
sdr-play-duo-role-diversity = Una seconda antenna (diversità / soppressione QRM)
sdr-play-duo-role-second-radio = Una seconda radio, sulla propria frequenza
sdr-play-agc-off = Disattivato (guadagno IF manuale)
sdr-play-agc-hz5 = 5 Hz
sdr-play-agc-hz50 = 50 Hz
sdr-play-agc-hz100 = 100 Hz
pluto-ptt-off = Disattivato
pluto-ptt-gpo01 = GPO0 = RX, GPO1 = TX
pluto-ptt-gpo23 = GPO2 = RX, GPO3 = TX
pluto-duplex-fdd = FDD (entrambe insieme)
pluto-duplex-tdd = TDD (una alla volta)
pluto-agc-manual = Manuale
pluto-agc-slow-attack = Attacco lento
pluto-agc-fast-attack = Attacco veloce
pluto-agc-hybrid = Ibrido
parity-none = Nessuna
parity-even = Pari
parity-odd = Dispari
panadapter-tap-antenna = Antenna (condivisa con la radio)
panadapter-tap-if-output = L'uscita I.F. della radio
panadapter-audio-attached = Il ricevitore collegato
panadapter-audio-transceiver = Il ricetrasmettitore
kenwood-send-ts2000 = Stile TS-2000 (TX;)
kenwood-send-ts590 = Stile TS-590 (TX1;)
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = Controllato dalla radio
cw-keying-cat = Manipolatore dell'apparato (CAT)
cw-keying-audio = Scheda audio (MCW)
alert-sound-ding = Campanella
alert-sound-two-tone = Due toni
alert-sound-triplet = Terzina
alert-sound-warble = Trillo
alert-sound-digital = Digitale
alert-reply-tone = Tono
alert-reply-voice = Voce
alert-reply-both = Tono + voce
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
icom-model-other = Altro (imposta l'indirizzo a mano)
backend-auto = Rilevamento automatico (SoapySDR / CAT)
backend-soapy = SoapySDR
backend-cat = CAT / Audio
backend-hpsdr = HPSDR (rete)
backend-tci = TCI (rete)
backend-icom-net = Icom LAN (rete)
backend-smart-sdr = SmartSDR / FlexRadio (rete)
backend-pluto = PlutoSDR (rete)
backend-rtl-sdr = RTL-SDR (USB)
backend-rtl-tcp = RTL-SDR tramite rtl_tcp (rete)
backend-spy-server = SpyServer (rete)
backend-spy-server-vfo = SpyServer VFO+FFT, bassa larghezza di banda (rete)
backend-kiwi-sdr = KiwiSDR / Web-888 (rete)
backend-rx888 = RX-888 (USB)
backend-airspy-hf = Airspy HF+ (USB)
backend-airspy = Airspy R2 / Mini (USB)
backend-hack-rf = HackRF One / Pro (USB)
backend-sdr-play = SDRplay RSP (USB)
backend-elad = ELAD FDM-DUO / FDM-S (USB)
backend-lime = LimeSDR + LimeRFE (LimeSuite)
backend-hydra-sdr = HydraSDR RFOne (USB)
backend-fobos = RigExpert Fobos SDR (USB)
backend-usb-audio = Radio USB audio (scheda audio)
backend-none = Non configurata
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld (rete)
cat-family-flrig = flrig (rete)
publicsdr-its-interface = la sua interfaccia

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = Off
agc-mode-slow = Lento
agc-mode-med = Medio
agc-mode-fast = Veloce
airspy-gain-linearity = Linearità (migliore con segnali forti)
airspy-gain-sensitivity = Sensibilità (migliore con segnali deboli)
ais-kind-class-a = Classe A
ais-kind-class-b = Classe B
ais-kind-base-station = stazione base
ais-kind-sar-aircraft = aereo SAR
ais-kind-aid-to-navigation = ausilio alla navigazione
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = unità associata
alert-event-called = Una stazione mi sta chiamando
alert-event-cq = Un CQ a cui potrei rispondere
alert-event-new-dxcc = Nuova entità DXCC
alert-event-new-dxcc-band = Nuova entità su questa banda
alert-event-new-grid = Nuovo quadrato locatore
audio-routing-both = Sinistra + destra
audio-routing-left = Solo sinistra
audio-routing-right = Solo destra
button-mode-momentary = Tieni premuto
button-mode-toggle = Alterna
contest-mode-none = Nessuno
contest-mode-eu-vhf = EU VHF Contest
converter-tx-off = Disattivato durante la conversione
converter-tx-transverter = Attraverso lo stesso convertitore
converter-tx-own = Offset proprio
cw-engine-neural = NEURALE
cw-engine-timing = TEMPORIZZAZIONE
decode-sort-none = Nessuno
decode-sort-signal = SNR
decode-sort-distance = Dist
decode-sort-country = Paese
diversity-mode-cancel = Cancella — annulla una sorgente di rumore
diversity-mode-combine = Combina — ricezione in diversità
dxped-mode-normal = Normale
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = Rifiuta di trasmettere
fail-safe-warn-only = Trasmetti comunque, avvisa
ftp-security-plain = FTP (senza cifratura)
ftp-security-explicit-tls = FTPS esplicito (AUTH TLS)
ftp-security-implicit-tls = FTPS implicito
hpsdr-filter-board-none = Nessuna — le uscite restano spente
hpsdr-filter-board-n2adr = Scheda filtri N2ADR
hpsdr-filter-board-alex = Codice banda Alex / Hermes (Zeus, HiQSDR, Quisk)
hpsdr-filter-board-custom = Personalizzata — la mia tabella qui sotto
hpsdr-io-rx-input-radio = Ingresso proprio della radio
hpsdr-io-rx-input-io-board = Scheda IO J9
hpsdr-io-rx-input-io-board-pure-signal = Scheda IO J9, PureSignal in trasmissione
hydra-sdr-gain-linearity = Linearità (migliore con segnali forti)
hydra-sdr-gain-sensitivity = Sensibilità (migliore con segnali deboli)
icom-rx-source-af = AF — la radio demodula
icom-rx-source-if12k = IF a 12 kHz — sdroxide demodula
icom-scope-span-radio = Come impostato sulla radio
icom-scope-span-khz2-5 = 5 kHz (±2,5k)
icom-scope-span-khz5 = 10 kHz (±5k)
icom-scope-span-khz10 = 20 kHz (±10k)
icom-scope-span-khz25 = 50 kHz (±25k)
icom-scope-span-khz50 = 100 kHz (±50k)
icom-scope-span-khz100 = 200 kHz (±100k)
icom-scope-span-khz250 = 500 kHz (±250k)
icom-scope-span-khz500 = 1 MHz (±500k)
ism-family-weather = METEO
ism-family-meter = CONTATORI
ism-family-home-auto = CASA
ism-family-lo-ra = LORA
ism-family-unidentified = SCONOSCIUTO
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = non identificato
js8-speed-normal = NORMALE
js8-speed-fast = VELOCE
js8-speed-turbo = TURBO
js8-speed-slow = LENTO
lime-aux-role-off = Non usata
lime-aux-role-diversity = Una seconda antenna (diversità / soppressione QRM)
lime-aux-role-pure-signal = Retroazione di trasmissione (predistorsione PureSignal)
lookup-provider-none = Off
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = POSTA IN ARRIVO
mail-folder-outbox = POSTA IN USCITA
mail-folder-sent = INVIATA
mail-folder-archive = ARCHIVIO
memory-sort-name = Nome
memory-sort-freq = Freq
memory-sort-band = Banda
memory-sort-stored = Memorizzata
mouse-button-middle = Centrale
mouse-button-extra1 = Indietro (extra 1)
mouse-button-extra2 = Avanti (extra 2)
nr-level-off = Off
nr-level-low = Bassa
nr-level-medium = Media
nr-level-high = Alta
nr-level-rnn-low = RNN Bassa
nr-level-rnn-med = RNN Media
nr-level-rnn-high = RNN Alta
nr-level-spec-low = SPEC Bassa
nr-level-spec-med = SPEC Media
nr-level-spec-high = SPEC Alta
nr-level-nr2-low = NR2 Bassa
nr-level-nr2-med = NR2 Media
nr-level-nr2-high = NR2 Alta
nr-level-df-low = DFNR Bassa
nr-level-df-med = DFNR Media
nr-level-df-high = DFNR Alta
nr-strength-low = Bassa
nr-strength-med = Media
nr-strength-high = Alta
orbit-rings-none = nessuno
orbit-rings-curated = selezionati
orbit-rings-all = tutti
qso-step-idle = Inattivo
qso-step-wait-cq = Attesa CQ
qso-step-calling-cq = Chiamata CQ
qso-step-tx-grid = TX Locatore
qso-step-tx-report = TX Rapporto
qso-step-tx-r-report = TX R+Rapporto
qso-step-tx-rr73 = TX RR73
qso-step-tx73 = TX 73
qso-step-confirming = Conferma
rds-standard-auto = Automatico
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = Regione 1 — Europa, Africa, Medio Oriente, Asia settentrionale
region-r2 = Regione 2 — Americhe
region-r3 = Regione 3 — Asia meridionale e orientale, Oceania, Pacifico
relative-mode-absolute = Assoluto (cursore)
relative-mode-sign-magnitude = Relativo: segno/modulo
relative-mode-twos-complement = Relativo: complemento a due
relative-mode-binary-offset = Relativo: centrato su 64
relay-link-off = Nessuno
relay-link-serial = Scheda relè USB (seriale)
relay-link-serial-lines = Linea seriale RTS/DTR
relay-link-hid = Scheda relè USB (HID)
relay-link-cm108 = GPIO scheda audio CM108/CM119
relay-link-gpio = Linea GPIO Linux
relay-link-command = Comando esterno
relay-role-unused = Non usato
relay-role-sdr-antenna = Metti a massa l'antenna SDR
relay-role-amplifier = Comanda un amplificatore / relè T-R
relay-role-aux = Ausiliario
relay-role-band-decoder = Decodificatore di banda (filtro / transverter)
rfe-channel-wb1000 = Banda larga 1 – 1000 MHz
rfe-channel-wb4000 = Banda larga 1 – 4 GHz
rfe-channel-ham0030 = HF, fino a 30 MHz
rfe-channel-ham0070 = 6 m / 4 m (50 – 70 MHz)
rfe-channel-ham0145 = 2 m (140 – 150 MHz)
rfe-channel-ham0220 = 1,25 m (220 – 225 MHz)
rfe-channel-ham0435 = 70 cm (400 – 450 MHz)
rfe-channel-ham0920 = 33 cm (902 – 928 MHz)
rfe-channel-ham1280 = 23 cm (1220 – 1420 MHz)
rfe-channel-ham2400 = 13 cm (2,3 – 2,5 GHz)
rfe-channel-ham3500 = 9 cm (3,3 – 3,7 GHz)
rfe-channel-cell-band01 = Banda cellulare 1
rfe-channel-cell-band02 = Banda cellulare 2
rfe-channel-cell-band03 = Banda cellulare 3
rfe-channel-cell-band07 = Banda cellulare 7
rfe-channel-cell-band38 = Banda cellulare 38
rfe-channel-not-selected = Nessuno selezionato
rfe-link-off = Non collegato
rfe-link-serial = Il proprio cavo USB (seriale)
rfe-link-board = Tramite il LimeSDR (GPIO / I²C)
rfe-mode-control-auto = Automatico (segue il cablaggio)
rfe-mode-control-rx = Ricezione sempre
rfe-mode-control-tx = Trasmissione sempre
rfe-mode-control-tx-rx = Entrambe sempre (solo connettori separati)
rifp-encoding-auto = Automatico
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = Grezzo
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = MEM
scan-kind-range = INTERVALLO
scan-resume-carrier = PORTANTE
scan-resume-timed = A TEMPO
scan-resume-manual = MANUALE
sense-line-off = Non collegata
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD (rilevamento portante)
shift-simplex = SIMPLEX
shift-minus = −
shift-plus = +
speed-slow = Lento
speed-medium = Medio
speed-fast = Veloce
speed-faster = Più veloce
speed-fastest = Velocissimo
tls-verify-system = Archivio di fiducia di sistema
tls-verify-pinned = Impronta fissata
tls-verify-off = Non verificare
tone-mode-off = OFF
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN (in rotazione)
upload-naming-original = Nome file originale
wheel-action-zoom = Zoom
wheel-action-tune = Sintonia
wheel-action-none = Nulla

settings-alerts-say = DI'

settings-alerts-say-hint = Ascolta questo avviso pronunciato a voce

settings-alerts-say-need-voice = Scegli Voce oppure Tono + voce per ascoltarlo

settings-alerts-say-voice-off = Attiva la voce in Impostazioni → UI → Annunci vocali

settings-alerts-speaks = dice: { $text }

settings-picup-keep = conserva

jtty-starting-receiver = avvio del ricevitore JTTY…

jtty-receive-audio-level = Livello audio in ricezione

jtty-sending = {" "}TRASMISSIONE{" "}

jtty-sent-once-note = Il messaggio viene inviato una sola volta quando premi TX — JTTY è asincrono, quindi non c'è alcun periodo da attendere e nulla si ripete.

signalid-title = ID SEGNALE

signalid-tuned-now = Sintonizzato ora

signalid-passband = banda passante { $bw }

signalid-band = banda { $band }

signalid-find = Cerca

signalid-find-hint = FT8, meteore, meteo, aerei…

signalid-candidates = { $n } candidati per questa sintonia

signalid-matches = { $n } risultati per “{ $query }”

signalid-nothing-fits = Nulla nel catalogo corrisponde a questa sintonia, modo e banda passante.

signalid-no-match = Nessuna voce del catalogo corrisponde.

signalid-open-url = Apri { $url }

windows-chip-del = CANC

windows-chip-edt = MOD

windows-chip-ren = RIN

windows-chip-rec = REC

windows-name-hint = nome

windows-waterfall-marks = Waterfall

windows-stacked = in pila

jtty-listening-note = In ascolto. Le trasmissioni JTTY possono iniziare in qualsiasi momento — un messaggio compare qui un istante dopo essere stato ricevuto.

extras-title = EXTRA

extras-receivers = Ricevitori

extras-dab-hint = Radio digitale DAB / DAB+ — il lettore si apre sotto il waterfall

extras-aircraft-watch-hint = AEREI — un aereo della tua lista di controllo è in aria; apri la vista Watch

extras-aircraft-hint = AEREI — la mappa di ciò che la radio riceve: aerei e radiosonde, in una finestra staccabile

extras-radio-hint = RADIO CLASSIC — un frontale radio classico per l'ascolto broadcast: bande, memorie, nome della stazione e RadioText. Si aggancia sotto il waterfall nei modi voce; Impostazioni → Staccabile lo porta in una finestra

extras-antenna = ANTENNA

extras-antenna-hint = ANTENNA (Sperimentale/Beta) — lunghezze di taglio, impedenza al punto di alimentazione e diagramma per le antenne comuni a filo/verticali/Yagi

recjobs-title = REGISTRAZIONI

recjobs-finished = Terminata “{ $name }”

recjobs-skipped = Saltata “{ $name }” — già in registrazione

recjobs-recording-name = Registrazione di “{ $name }”

recjobs-new-job = + NUOVA ATTIVITÀ

recjobs-recording = ● in registrazione

recjobs-none = Nessuna registrazione programmata — premi + NUOVA ATTIVITÀ per aggiungerne una.

recjobs-done = fatto

recjobs-edit = modifica

recjobs-del = canc

recjobs-station = Stazione

recjobs-station-hint = BBC World Service

recjobs-kind = Tipo

recjobs-start-utc = Inizio (UTC)

recjobs-for = Durata

recjobs-save = SALVA

recjobs-kind-audio = Audio

recjobs-kind-iq = I/Q

recjobs-kind-both = Entrambi

panels-sensitivity = Sensibilità

navtex-hunting = in ricerca

navtex-hunting-hint =
    Non viene ancora letto nulla.

    Quasi sempre è colpa della sintonia. NON digitare il numero indicato (518) nella casella della frequenza: NAVTEX è quotato come canale e questo programma ascolta appena sotto. Clicca invece il pulsante 518 qui sopra — imposta lui la sintonia corretta (516.300).

    Se c'è chiaramente un segnale forte ma il testo è comunque senza senso, premi REV: sta arrivando al contrario.

    Ricorda inoltre che NAVTEX segue un orario, non è una conversazione: una stazione costiera trasmette per dieci minuti ogni quattro ore, quindi il silenzio è normale per la maggior parte del tempo. Devi essere in ascolto quando tocca alla tua stazione. E i 518 kHz si animano solo di notte — una banda silenziosa di giorno non è un decodificatore rotto.

navtex-sensitivity-hint =
    Sensibilità — con quanta facilità il decodificatore si aggancia a un segnale e lo mantiene.
navtex-auto-hint = Torna alle soglie di riferimento (0). Evidenziato quando attivo.

navtex-tune-dial-usb-below-channel = Sintonizza qui (USB, 1.710 kHz sotto il canale).

settings-audio-your-audio = Il tuo audio

settings-audio-your-audio-why = Gli altoparlanti con cui ascolti e il microfono attraverso cui esce la tua voce. Appartengono a questo computer e non a una radio, quindi sono gli stessi con qualsiasi radio tu stia usando.

settings-audio-browser-note = Qui i dispositivi audio si scelgono nel browser, non in questa finestra — la pagina chiede al browser altoparlanti e microfono. Le impostazioni della scheda audio della tua stazione, più in basso in questa pagina, restano valide.

settings-audio-speakers = Altoparlanti

settings-audio-channels-hint = Su quale/i canale/i degli altoparlanti arriva l'audio ricevuto. Sinistro o destro silenzia l'altro, per ascoltare da un solo lato.

settings-audio-microphone-note = Il microfono è ciò che porta la tua voce in uscita quando il baracchino è commutato in trasmissione dal suono (MCW) o dal VOX. Se invece il tuo apparato si commuta da solo via CAT, il microfono non viene usato affatto per trasmettere e questa riga non cambia nulla.

settings-audio-stream-out = Invia l'audio in uscita

settings-audio-stream-out-why = Invia ciò che questa radio riceve a un altro programma attraverso la rete — un decodificatore, un registratore, un secondo computer — senza passare da una scheda audio.

settings-audio-send-to-hint =
    `host:porta`. Mono, 16 bit, alla frequenza di campionamento in ricezione, un datagramma per blocco audio, ed è l'audio del **punto di registrazione**: volume AF, MUTE e trasmissione non lo influenzano, perché un decodificatore esterno vuole la radio e non il livello delle tue cuffie.

    Un indirizzo numerico, non un nome host. Vuoto significa disattivato.

settings-audio-send-to-note = `127.0.0.1:7355` lo invia a un altro programma su questo computer; l'indirizzo di questo computer lo invia attraverso la rete.

settings-audio-radio-audio = Audio di questa radio

settings-audio-radio-audio-why = La scheda audio tra questa radio e questo computer. Appartiene alla radio e non al computer: due radio qui possono condividere il computer e richiedere comunque schede diverse, quindi queste impostazioni restano legate alla radio da cui hai aperto le Impostazioni.

settings-audio-no-cards-yet = Questo computer non ha ancora comunicato le sue schede audio. I nomi si possono comunque digitare qui sotto; un elenco compare dopo aver richiesto i dispositivi.

settings-audio-rescan = Riesamina le schede audio

settings-audio-rescan-hint =
    Chiede di nuovo al computer quali schede audio possiede. Non viene aperto nulla, quindi si può premere in sicurezza durante la ricezione.

    Usalo dopo aver collegato un codec. Non riapre la radio — quello è Applica / riconnetti nella scheda Radio — e non può riparare una scheda rinominata dal server audio, cosa che viene segnalata sulla radio.

settings-audio-from-radio = Dalla radio

settings-audio-to-radio = Alla radio

settings-audio-radio-note = { "*" }*Dalla radio** è ciò che il tuo apparato invia — un'uscita audio USB, oppure la presa cuffie di un ricevitore verso l'ingresso di linea di una scheda audio. **Alla radio** è ciò che l'apparato ascolta per la tua voce, ed è anche il modo in cui il programma commuta un apparato in trasmissione tramite il suono: lascialo vuoto e viene usato il microfono dell'apparato, se ne ha uno.

settings-audio-recording = Registrazione

settings-audio-recording-why = Dove vengono scritte le registrazioni audio, le registrazioni programmate e le acquisizioni I/Q grezze.

enigma-need-more = servono più dati cifrati su cui lavorare

enigma-title = ENIGMA

enigma-no-cables = senza cavi

enigma-thin = scarso

enigma-rotor = Rotore

enigma-window-hint = Grundstellung — clicca per far avanzare la ruota

enigma-input-hint = digita, o usa i tasti qui sopra

enigma-clear = CANCELLA

enigma-copy = COPIA ⇄

enigma-solve-note = LÖSEN — incolla un testo cifrato. Fornisci una parola crib che ti aspetti nel messaggio per un testo con cavi; senza di essa il risolutore tratta una macchina senza cablaggio.

enigma-crib = Crib

enigma-solve = RISOLVI

enigma-flaw = Una lettera non si cifra mai in se stessa — ci pensa il riflettore. È il difetto della macchina reale, ed è su questo che il risolutore pota.

settings-audio-takes-over-speakers = { "*" }*Questi altoparlanti sono un dispositivo del server audio, non una scheda audio.** Usarli li rende anche il dispositivo di riproduzione per *tutto il resto* su questo computer — anche gli altri programmi suoneranno attraverso di essi. PipeWire non permette a un programma di scegliere il proprio destinatario di riproduzione, quindi è inevitabile. Per cambiare: scegli una scheda audio, oppure ripristina il predefinito del computer (`wpctl set-default <nome>`) ed esegui una nuova scansione.

settings-audio-takes-over-microphone = { "*" }*Questo microfono è un dispositivo del server audio, non una scheda audio.** Usarlo lo rende anche il dispositivo di registrazione per *tutto il resto* su questo computer — anche gli altri programmi registreranno da esso. PipeWire non permette a un programma di scegliere la propria sorgente di acquisizione, quindi è inevitabile. Per cambiare: scegli una scheda audio, oppure ripristina il predefinito del computer (`wpctl set-default <nome>`) ed esegui una nuova scansione.

settings-audio-currently-chosen = ⚠  Attualmente scelto: { $device }

knowncalls-no-table = Questo modo non tiene alcuna tabella di nominativi. È un decodificatore FT8/FT4 che calcola l'hash dei nominativi — qualsiasi altro non ha nulla da mostrare qui.

knowncalls-not-asked = Non ancora richiesto — premi AGGIORNA.

knowncalls-nobody = Ancora nessuno. Una stazione viene aggiunta nel momento in cui la si sente compitare il proprio nominativo — un CQ, un rapporto con entrambi i nominativi in chiaro, oppure una semplice chiamata in 11 m.

knowncalls-title = STAZIONI NOTE

knowncalls-intro = Ogni nominativo che un <...> con hash può attualmente nominare su questo ricevitore. Un hash non si può leggere al contrario — una stazione compare qui solo dopo essere stata sentita compitare il proprio nominativo, ed è a questo che serve una semplice chiamata in 11 m.

knowncalls-find-hint = nominativo, paese, prefisso…

knowncalls-refresh = AGGIORNA

knowncalls-capped = le { $shown } più recenti su { $total } — AGGIORNA per richiedere di nuovo, l'elenco è un'istantanea

knowncalls-known = { $shown } note

knowncalls-matches =
    { $n } { $n ->
        [one] risultato
       *[other] risultati
    }

knowncalls-no-match = Nessuna stazione nota corrisponde.

knowncalls-no-country = nessun paese — non è un nominativo che questa tabella conosce

classic-controls = COMANDI

classic-fold-the-column-away-to-a-rail-on = Ripiega la colonna in una barra a sinistra. L'indicatore e lo strumento restano sulla barra, e il comando che la riporta indietro si trova sulla barra laterale.

classic-click-to-open-or-close-this = Clicca per aprire o chiudere questa sezione. Trascinala su un'altra sezione per spostarla — l'ordine viene ricordato.

classic-bring-the-control-column-back = Riporta la colonna dei comandi.

classic-no-radio = nessuna radio

classic-rate = Frequenza di campionamento

classic-dial = Sintonia

classic-receive = Ricezione

classic-listening-screen = schermata di ascolto

classic-radio-settings = IMPOSTAZIONI RADIO

classic-the-backend-the-sample-rate-the = Il backend, la frequenza di campionamento, l'antenna, il convertitore e la calibrazione propria di questa radio — Impostazioni → Radio, dove sono sempre state. La colonna mostra come sono impostati anziché una seconda copia dei comandi.

classic-counted-in-steps-not-decibels-what = Conteggiato in passi, non in decibel: quanto vale un passo dipende dal ricevitore stesso, e su un SDRplay dipende anche dalla banda. Destra resta più guadagno.

classic-the-plutosdr-s-own-agc-is-setting = L'AGC del PlutoSDR sta impostando questo guadagno, quindi muovere il cursore non ha effetto. Imposta Impostazioni → Radio → AGC su Manuale per controllarlo qui.

classic-squelch = Squelch

classic-gate-the-audio-below-this-power-in = Blocca l'audio sotto questa potenza nella banda passante in ricezione, in dBFS. Sinistra è aperto. Impostalo sopra il rumore e sotto il segnale.

classic-this-radio-has-not-reported-what = Questa radio non ha ancora comunicato cosa sa fare — il collegamento è chiuso, oppure non ha risposto.

classic-open-the-link-in-the-radio-strip = Apri il collegamento nella barra della radio e i comandi di trasmissione compariranno qui.

classic-this-device-receives-only-it-has = Questo dispositivo è solo ricevente: non ha un trasmettitore che il programma possa commutare.

classic-transmit-is-offered-by-a-rig-with = La trasmissione è offerta da un apparato con un percorso di controllo o audio — Impostazioni → Radio.

classic-transmit-is-off-for-this-radio-s = La trasmissione è disattivata per la schermata di questa radio.

classic-settings-radio-show-transmit = Impostazioni → Radio → "Mostra i comandi di trasmissione" la riporta.

classic-key = Tasto

classic-controlled-envelope-ssb-how-much = SSB a inviluppo controllato: quanta elaborazione del parlato applicare. Più in alto è più forte e più piatto, al prezzo di un po' di splatter ai lati.

classic-holding-the-squelch-open-click-to = Squelch tenuto aperto. Clicca per rimettere la soglia che avevi.

classic-click-to-hold-the-squelch-open-to = Clicca per tenere aperto lo squelch — per sentire se c'è qualcosa — e clicca di nuovo per rimetterlo esattamente com'era.

panels-received = Ricevuto

panels-text = Testo

panels-hex = Esadecimale

panels-nothing-received-yet = Ancora nulla ricevuto.

panels-payload = Carico utile

panels-blocks = Blocchi

panels-sequence = Sequenza

uvpacket-starting-receiver = avvio del ricevitore UVPacket…

uvpacket-frames-count = { $n } frame

uvpacket-frames-count-hint = Frame decodificati dall'avvio del ricevitore.

uvpacket-intro = Un modo packet per gruppi privati VHF/UHF. Sintonizza sul canale del gruppo; il sotto-modo viene rilevato da ogni frame.

uvpacket-nothing-yet = Ancora nulla. UVPacket è silenzioso tra una trasmissione e l'altra — lascialo su un canale e attendi un burst.

uvpacket-app-seq = app { $app } · seq { $seq }

uvpacket-click-read-frame = Clicca per leggere l'intero frame.

uvpacket-bps-net = { $bps } bps netti

uvpacket-sub-mode = Sotto-modo

uvpacket-app-type = Tipo di app

uvpacket-payload-bytes = { $n } byte

dsc-starting-receiver = avvio del ricevitore DSC…

dsc-seq-tone = { $n } seq · tono { $pct }%

dsc-seq-tone-hint = Sequenze arrivate al marcatore di fine, e quanto nettamente si separano i due toni.

dsc-nothing-yet = Ancora nulla. DSC è silenzioso tra un'unità e l'altra — lascialo su un canale e attendi un burst.

panels-marginal = marginale

dsc-marginal-hint = Un carattere ha fallito il controllo BCH — i campi mostrati potrebbero essere errati.

dsc-click-read-sequence = Clicca per leggere l'intera sequenza.

panels-marginal-caps = MARGINALE

dsc-marginal-caps-hint = Almeno un carattere ha fallito il controllo BCH. La sequenza viene mostrata perché un allarme di soccorso arrivato in modo imperfetto vale comunque la pena di essere letto.

dsc-category = Categoria

dsc-nature = Natura

dsc-caller-mmsi = MMSI chiamante

dsc-target-mmsi = MMSI destinatario

dsc-alert-time = Ora dell'allarme

dsc-working-ch = Canale di lavoro

dsc-raw-symbols = Simboli grezzi

dsc-none = (nessuno)

settings-general-device-type-name = …oppure digita il nome — vuoto è il predefinito di sistema

settings-general-signed-in-station = Stazione connessa

settings-general-sign-out-hint = Dimentica l'accesso di questo dispositivo. La stazione dovrà chiederlo di nuovo alla prossima connessione; quella attuale resta attiva finché non la chiudi.

settings-general-recordings-remote = Le registrazioni vengono scritte sul computer a cui è collegata la radio. Imposta la cartella lì.

settings-general-folder = Cartella

settings-general-choose = SCEGLI…

settings-general-choose-hint = Scegli la cartella in cui scrivere le registrazioni

settings-general-default = PREDEFINITO

settings-general-default-hint = Torna al predefinito — la tua cartella Musica, oppure la cartella di configurazione se il sistema non ne ha una

settings-general-recordings-dir-note = Dove vengono scritte le registrazioni audio, le registrazioni programmate e le acquisizioni I/Q grezze. Predefinito: la sottocartella sdroxide della tua cartella Musica, oppure la cartella recordings della cartella di configurazione quando il sistema non espone alcuna cartella Musica.

setup-suffix-cq = { " " }cq

setup-suffix-tries = { " " }tentativi

setup-window = finestra

setup-south = sud

setup-west = ovest

setup-message-templates = Modelli di messaggio  { $tokens }

setup-ft8-decoder-effort-hint =
    Quanto si impegna il decodificatore FT8 sui segnali deboli.

    Veloce — un solo passaggio, nessuna sottrazione del segnale: il più rapido, meno decodifiche.
    Normale — sottrazione piatta a più passaggi: ~0,4 s su uno slot affollato, qualche decodifica in meno di Profondo.
    Profondo — il passaggio con checkpoint di WSJT-X: ~1,2 s e il maggior numero di decodifiche (il predefinito).

    Solo il secondo gruppo di decodifiche è regolato; il passaggio semplice viene sempre emesso per primo, quindi una risposta non è mai ritardata da esso.

setup-auto-cq-repeats-hint = Quante chiamate CQ compie un ciclo prima di terminare. 0 significa nessun limite proprio: il watchdog di trasmissione e lo stop per inattività continuano a limitare il ciclo.

setup-auto-cq-then-listen-hint = Dopo queste chiamate CQ, passa da AUTO CQ ad AUTO RX e continua ad ascoltare invece di fermarsi del tutto. Disattivato, il ciclo si ferma.

setup-auto-call-retries-hint = Quante volte viene chiamata una stazione prima che il ciclo rinunci e passi oltre nella coda.

setup-auto-session-hint = Quanto può durare un ciclo non presidiato, qualunque cosa stia facendo. 0 significa nessun limite proprio.

setup-serial-contests-hint = Contest con numero progressivo — ARRL RTTY Roundup e CQ WPX via FT8: un rapporto e un numero progressivo (o uno stato USA), senza locatore. Il CQ parte come CQ RU.

setup-tx-level-fm-hint =
    Quanto forte viene consegnato il burst di questo modo a una radio che lo modula da sé — e in FM questa è la deviazione, perché un trasmettitore FM trasforma il livello audio in escursione di frequenza e non ha un ALC che lo trattenga. Il packet a 1200 baud vuole circa 3 kHz dove la voce ne vuole 5, quindi il fondo scala su un ingresso dati impostato per la voce sovradeviata — cosa che a chi ascolta suona del tutto normale e non viene decodificata da nessuno. Abbassalo finché altre stazioni ti riportano, oppure regola il livello sulla radio.

    Questo livello appartiene solo a questo modo. Un modo che non hai mai impostato parte dal valore FM, e i modi in banda laterale partono dal proprio — così una deviazione impostata per i 1200 baud non finisce mai su FT8.

setup-tx-level-ssb-hint =
    Quanto forte viene consegnato il passaggio di questo modo a una radio che lo modula da sé — un apparato CAT sulla sua scheda audio, un FLEX, un Icom sulla sua porta di rete. In banda laterale questo è il pilotaggio del modulatore: abbassalo finché l'ALC dell'apparato si muove appena e imposta la potenza sulla radio, perché un ALC che lavora su un modo a inviluppo costante è ciò che genera splatter. Qui il pilotaggio raggiunge il registro di potenza dell'apparato, non il suo audio, quindi questo è il livello.

    Questo livello appartiene solo a questo modo: FT8, RTTY, PSK e MCW ne tengono ciascuno uno proprio, perché non caricano un modulatore allo stesso modo. Lo stesso cursore è sulla barra di trasmissione, dove puoi raggiungerlo mentre trasmetti.

setup-repeats-hint-base = Chiamate senza risposta a una stazione prima di passare oltre. Chiamare CQ è esente.

setup-repeats-hint-duration = { " " }Con la durata di slot di { $mode } ({ $slot } s) sono circa { $mins } min.

setup-repeats-hint-disables = { " " }0 lo disattiva.

setup-repeats-hint-11m = { " " }In 11 m è anche ciò che pone fine a un ciclo di CQ: il watchdog di trasmissione non lo interrompe, perché una stazione che risponde a un CQ è attesa entro circa 30 s e un orologio non sa distinguere un ciclo recente da uno stantio.

setup-symbol-home-station = Stazione fissa

setup-symbol-car = Auto

setup-symbol-truck = Camion

setup-symbol-van = Furgone

setup-symbol-motorcycle = Moto

setup-symbol-bicycle = Bicicletta

setup-symbol-person-on-foot = Persona a piedi

setup-symbol-motorhome = Camper

setup-symbol-boat = Barca

setup-symbol-yacht = Yacht

setup-symbol-light-aircraft = Aereo leggero

setup-symbol-balloon = Pallone

setup-symbol-digipeater = Digipeater

setup-symbol-i-gate = I-gate

setup-symbol-repeater = Ripetitore

setup-symbol-weather-station = Stazione meteo

setup-symbol-hospital = Ospedale

setup-symbol-portable-campsite = Portatile / campeggio

settings-general-sign-out = ESCI

settings-general-fallback-password = Memorizzo una password in questo browser per { $stations }, perché non ha risposto con alcun cookie di accesso. Le stazioni più vecchie lo fanno; non è la stessa cosa.

settings-general-device-hint-speakers = Il nome esatto con cui vengono aperti i tuoi altoparlanti. Sceglilo dall'elenco quando c'è; digitalo quando l'elenco non lo mostra — un server audio può offrire un dispositivo che l'API dell'host non enumera. Vuoto usa il predefinito di sistema.

settings-general-device-hint-microphone = Il nome esatto con cui viene aperto il tuo microfono. Sceglilo dall'elenco quando c'è; digitalo quando l'elenco non lo mostra — un server audio può offrire un dispositivo che l'API dell'host non enumera. Vuoto usa il predefinito di sistema.

settings-general-device-hint-radio-rx = Il nome esatto con cui viene aperto l'audio in ricezione di questa radio. Sceglilo dall'elenco quando c'è; digitalo quando l'elenco non lo mostra — un server audio può offrire un dispositivo che l'API dell'host non enumera. Vuoto usa il predefinito di sistema.

settings-general-device-hint-radio-tx = Il nome esatto con cui viene aperto l'audio in trasmissione di questa radio. Sceglilo dall'elenco quando c'è; digitalo quando l'elenco non lo mostra — un server audio può offrire un dispositivo che l'API dell'host non enumera. Vuoto usa il predefinito di sistema.

settings-general-device-hint-other = Il nome esatto con cui viene aperto il dispositivo. Sceglilo dall'elenco quando c'è; digitalo quando l'elenco non lo mostra — un server audio può offrire un dispositivo che l'API dell'host non enumera. Vuoto usa il predefinito di sistema.

settings-general-cb-world = Mondo / freeband — 80 canali

settings-general-cb-cept = CEPT / UE — 40 canali

settings-general-cb-germany = Germania — 80 canali

settings-general-cb-uk = Regno Unito — 27/81, 40 canali

settings-general-cb-us = Stati Uniti — 40 canali

settings-general-cb-australia = Australia — 40 canali

settings-general-cb-pmr446 = PMR446 — 16 canali (446 MHz)

settings-general-cb-dpmr446 = dPMR446 — 32 canali (446 MHz)

wefax-receiving-progress = ricezione { $min } min · { $lines } righe

wefax-listening = in ascolto

wefax-status-hint =
    In attesa di una carta.

    WEFAX segue un orario, non è una conversazione: una stazione trasmette le sue carte a orari stabiliti, quindi una frequenza scelta è silenziosa per gran parte della giornata — è normale. Usa il pulsante STAZIONI per sceglierne una e controllare l'orario.

    Se un segnale c'è ma l'immagine esce di un solo tono piatto (tutta grigia, tutta nera o tutta bianca), è la sintonia o la velocità — usa l'indicatore +0 Hz e il comando IOC/velocità, e controlla che la voce di STAZIONI corrisponda a ciò che viene trasmesso.

wefax-centre = CENTRA

wefax-centre-hint = Sposta la sintonia di { $sign }{ $off } Hz in modo che la sottoportante torni a 1900 Hz — il ricevitore è fuori di { $off } Hz, ed è ciò che trasforma la carta in una pagina bianca.

wefax-auto-start-hint = Inizia una carta quando si sente il tono di avvio a 300 Hz (IOC 576) o a 675 Hz, oppure, se il tono è andato perso in un fading o sotto una portante, quando arrivano alcuni secondi del segnale di fasatura — e termina quella in corso e fasa la nuova quando arriva la trasmissione successiva, che sia o no arrivato prima un tono di stop. Una portante sul canale non avvia nulla.

wefax-fit = ADATTA

wefax-save-chart-hint = Salva su questo computer la carta sullo schermo, per quanto è arrivata

wefax-waiting-for-chart = Sintonizza un orario fax in USB e attendi l'inizio della carta (tono di avvio o segnale di fasatura), oppure premi AVVIA per iniziare a metà carta.

wefax-carrier-dial-note = portante { $carrier } · sintonia { $dial } kHz (USB −1,9k)

wefax-carrier-note-hint = Il chip della stazione indica la portante pubblicata; la sottoportante radiofax si trova 1900 Hz sopra di essa, quindi la sintonia è volutamente 1,9 kHz più bassa. Sintonizza sulla portante stessa e il segnale cade fuori dalla banda passante.

contest-title = CONTEST

contest-intro = Scegli un contest e inserisci ciò che invii. Il logger funziona con qualsiasi modo — digita lo scambio per CW o SSB, e la parte FT8 lo compila da sola quando il contest ha un formato FT8.

contest-you-send = { "  " }invii: { $exchange }

contest-they-send = { "  " }inviano: { $exchange }

contest-cb-note = Attività CB / 11 m: un rapporto e uno scambio a testo libero — inserisci il canale, il nome o la zona usati dall'attività.

contest-your-exchange = Il tuo scambio

contest-start = AVVIA

contest-score-line = QSO { $qsos } · { $points } pti · { $mults } molt · PUNTEGGIO { $total }

contest-rate = { $r10 } in 10 min · { $r60 }/h

contest-stop = FERMA

contest-estimate = il punteggio è una stima — lo sponsor lo convalida

contest-call = NOMINATIVO

contest-dupe = DOPPIO

contest-sent = INVIATO

contest-rcvd = RICEVUTO

contest-sending = invio { $rst }

contest-sending-ours = invio { $rst } { $ours }

contest-log = LOG

playback-radio-n = { " " }· radio { $n }

playback-no-stamp = nessuna data nel nome — viene riprodotto nel modo in cui ti trovi

playback-title = RIPRODUZIONE — audio registrato

playback-find-hint = modo, MHz, data, nome…

playback-refresh-hint = Rileggi la cartella delle registrazioni

playback-asking = Richiesta delle registrazioni alla stazione…

playback-folder = Cartella: { $dir }

playback-none = Ancora nessuna registrazione. Premi REC → MP3 o WAV mentre c'è qualcosa in aria; il file compare qui, con nome composto da ora, frequenza e modo.

playback-on = ▶ ATTIVO

playback-play = ▶ RIPRODUCI

playback-iq-hint = Un'acquisizione I/Q, non audio. La riproduzione prende l'audio scritto dal registratore; un'acquisizione I/Q si riproduce con --file.

playback-not-while-tx = Non durante la trasmissione

playback-play-hint = Riproduci questa registrazione nei decodificatori dall'inizio

playback-busy-playing = In riproduzione — fermala prima di eliminarla

playback-busy-recording = Ancora in registrazione — ferma REC prima di eliminarla

playback-delete-ask = ELIMINARE?

playback-delete-hint-armed = Premi di nuovo per eliminare definitivamente questo file. Non si può annullare.

playback-delete-hint = Elimina questa registrazione dalla cartella (chiede ancora una volta prima)

playback-strip-label = ▶ RIPRODUZIONE

playback-not-live = non in diretta — la radio è in pausa

playback-carry-on = Continua a riprodurre

playback-pause = ⏸ PAUSA

playback-pause-hint = Pausa — i decodificatori sentono silenzio finché non riprendi

playback-again = ↺ DA CAPO

playback-again-hint = Torna all'inizio — un altro tentativo su un'immagine o una decodifica

playback-loop = LOOP

playback-loop-hint = Ricomincia dall'inizio quando il file finisce

playback-stop = ■ STOP

playback-stop-hint = Termina la riproduzione e rimette la radio com'era

playback-files = FILE

playback-end-of-file = fine del file

playback-paused = in pausa

playback-recorded-on-mode = Registrato su { $mhz } MHz in { $mode } — la sintonia e il modo sono stati impostati dal nome del file.

playback-recorded-on = Registrato su { $mhz } MHz — la sintonia e il modo sono stati impostati dal nome del file.

playback-no-dial-mode = Il nome non riporta né sintonia né modo, quindi viene riprodotto nel modo in cui ti trovi — scegli il decodificatore dal menu dei modi.

playback-format-stereo = { $rate } Hz, stereo — viene riprodotto il canale sinistro (ricezione).

playback-format-mono = { $rate } Hz, mono — viene riprodotto l'unico canale.

playback-stopped = Interrotto: { $fault }

playback-intro = Riproduci una registrazione nei decodificatori — SSTV, CW, RTTY, PSK, FT8 e ogni altro modo che decodifica dall'audio. La radio resta in pausa durante la riproduzione, la sintonia e il modo passano a quelli su cui è stata fatta la registrazione, e STOP rimette la radio com'era. Cambia modo durante la riproduzione per provare un altro decodificatore.

packet-callsign-hint = nominativo

packet-via-hint = via

aprs-filter-hint = filtro

aprs-msg-sent = inviato

aprs-msg-ack = ack

aprs-msg-rejected = rifiutato

aprs-reply-chip = rispondi

aprs-to-hint = a

aprs-message-hint = messaggio

aprs-raw-chip = RAW

fsq-to-button = A

fsq-del-chip = CANC

macros-column-key = tasto

macros-column-button = pulsante

macros-column-sends = invia

macros-label-hint = etichetta

macros-add = AGGIUNGI

macros-note-placeholders = { $mycall } e { $mygrid } vengono compilati quando il messaggio parte.

macros-buttons-note = Ogni pulsante invia tutto il suo testo in un colpo solo. F1–F9 premono i primi nove, purché nulla sullo schermo abbia la tastiera.



widgets-worked-before-any = Già collegato

vdl2-col-sig = SEG

vdl2-col-age = ETÀ

vdl2-command = comando

vdl2-response = risposta

vdl2-frame-hint = frame

vdl2-suffix-messages = { " " }messaggi

vdl2-suffix-stations = { " " }stazioni

ale-words-count = { $n } parole

ale-waiting-audio = In attesa dell'audio…

ale-no-words-yet = Ancora nessuna parola. Sintonizza un canale ALE (es. 8992 o 11175 kHz USB) e attendi una chiamata.

atchat-role-master = MASTER

atchat-role-backup = BACKUP

atchat-me = io

dab-sub-channel = { " " }· sub { $n }

dab-starting-receiver = avvio del ricevitore DAB…

dab-no-ensemble-yet = nessun ensemble ancora nominato — c'è una trasmissione DAB su questo canale?

dab-window-stats = { $mhz } MHz · { $frames } frame · { $fibs } fib

dab-volume-meter-hint = La stazione come la senti, sinistra e destra — e il volume: trascina o clicca lungo le barre, oppure usa la rotella. Cliccalo, poi le frecce lo regolano a passi. Funziona su uno schermo bloccato con il lucchetto.

rade-searching = in ricerca

rfpaint-swl-note = RF Paint è un modo solo trasmissione — nulla da mostrare in SWL.

cw-hand-key-arm-hint = Arma il keyer e usa il paddle USB. I contatti vanno al motore, che produce punti e linee alla velocità trasmessa — qui, non nel pannello, così la temporizzazione non viene arrotondata a un frame. La casella di trasmissione è bloccata finché è attivo e il testo in coda viene scartato. Funziona anche un tasto verticale nella stessa casella: scegli il tipo Tasto verticale.

cw-own-keyer-hint =
    Questa radio trasmette dal proprio keyer: il testo passa dalla porta di controllo e l'apparato temporizza gli elementi, quindi non c'è nulla tra la mano e l'aria che un tasto possa pilotare — né un paddle né un tasto verticale.

    Per manipolare a mano, imposta Manipolazione CW su "Scheda audio (MCW)" in Impostazioni → Radio. L'apparato resta quindi su una banda laterale e il sidetone del keyer viene trasmesso come audio, che è il percorso pilotato dal tasto verticale.

cw-no-paddle-found = nessun paddle trovato — sceglilo in Impostazioni → CW

cw-msg-hint = I tuoi pulsanti di messaggio — uno scambio di contest, una risposta con nome e QTH, TNX 73 GL. Ognuno invia tutto il suo testo in un colpo solo, e F1–F9 premono i primi nove. Viaggiano con la configurazione della stazione, quindi anche un client remoto li ha.

wspr-clear-rx = CANCELLA RX

wspr-rx-count = { $n } rx

wspr-sort-dist = DIST

wspr-sort-country = PAESE

wspr-tx-enable = ABILITA TX

wspr-stop-tx = FERMA TX

wspr-listening-as = In ascolto come { $call } in { $sent } — impostato nella scheda Generale delle Impostazioni.

wspr-set-callsign-grid-label-spots = Imposta nominativo e locatore nella scheda Generale delle Impostazioni per etichettare i tuoi spot.

schedule-source-mine = miei

schedule-all-of-them = tutti

schedule-title = PROGRAMMI

schedule-at = Alle

schedule-now-chip = ORA

schedule-set-time = imposta ora

schedule-find-hint = BBC, 6185, 15.4 MHz…

schedule-target = Bersaglio

schedule-any = qualsiasi

schedule-source = Fonte

schedule-all-hint = Ogni palinsesto scaricato nell'archivio unico. Una stazione elencata da due di essi compare due volte, che è il dettaglio per cui serve questa finestra — il panadapter mostra un solo nome.

schedule-favs = ★ PREF

schedule-favs-hint = Solo le stazioni che hai contrassegnato con la stella

schedule-all-favs = ⭐ TUTTE

schedule-all-favs-hint = Rende preferite tutte le stazioni del palinsesto e ne mostra le etichette sul waterfall finché resta attivo. Clicca di nuovo per cancellare tutti i preferiti.

schedule-solar-time = ORA LOCALE

schedule-solar-time-hint = Mostra l'ora locale di ogni riga al trasmettitore. È l'orologio del Sole — quattro minuti per grado di longitudine — non un fuso orario civile, quindi non conosce l'ora legale né i confini dei fusi.

schedule-map = MOSTRA MAPPA

schedule-map-hint = Apri LISTEN — questo palinsesto come mappa. Scegli una stazione, scrivi cosa hai sentito e registralo. La stazione selezionata viene trasferita

schedule-on-air = { $count } in onda alle { $time } UTC

schedule-now = ora

schedule-solar-hover = { $local } ora solare locale a { $site }, { $lon }°

schedule-favourite-hint = Rendi preferita questa stazione

rc-radio-classic = RADIO CLASSIC

rc-bands = Bande

rc-mem-programs = Mem · Programmi

rc-recall-band-mode-frequency-and = Richiama: banda, modo, frequenza e filtro in un colpo

rc-no-memories-yet-store-one-with-mem = ancora nessuna memoria — salvane una con MEM nella barra superiore

rc-broadcast-schedule-what-is-on-when = Palinsesto broadcast — cosa c'è, quando e dove

rc-bw-this-mode-has-one-fixed = BW — questo modo ha un'unica banda passante fissa

rc-bw-click-next-filter-right-click = BW — clic: filtro successivo · clic destro: precedente · rotella: passo

rc-step = PASSO

rc-step-click-next-tuning-step-right = PASSO — clic: passo di sintonia successivo · clic destro: precedente · rotella: passo

rc-agc-click-next-speed-right-click = AGC — clic: velocità successiva · clic destro: precedente · rotella: passo

rc-open = APERTO

rc-sql-click-open-back-to-the-last = SQL — clic: aperto / torna all'ultima soglia · rotella: alza o abbassa la soglia

rc-s-meter = S-METER

rc-wheel-on-a-digit-click-upper-lower = rotella su una cifra · clic sulla metà alta/bassa ± · clic destro azzera · doppio clic per digitare

rc-music = MUSICA ♪

rc-speech = PARLATO

rc-stereo-the-19-khz-pilot-is-locked = Stereo — il pilota a 19 kHz è agganciato — clic per mono

rc-mono-the-broadcast-stereo-decode = Mono — la decodifica stereo della trasmissione è disattivata — clic per attivarla

rc-mono-noise-reduction-is-on-and = Mono — la riduzione del rumore è attiva e lo stereo le cede il passo

rc-mono-no-19-khz-pilot-the-station = Mono — nessun pilota a 19 kHz: la stazione trasmette in mono, oppure il segnale è troppo debole

rc-stereo = STEREO

rc-group-broadcast = BROADCAST

rc-group-ham = RADIOAMATORI

rc-group-air = AERONAUTICA

rc-group-marine = MARINA

rc-cap-lw-mw-fm = OL / OM · FM

rc-cap-shortwave = OC · ONDE CORTE

rc-cap-amateur-bands = BANDE RADIOAMATORIALI

rc-cap-airband = BANDA AEREA

rc-cap-marine = VHF · MF MARINA

rc-cap-cb = BANDA CITTADINA

rc-preset-fm-ext = FM est. 64–87,5

rc-mem-n = MEM { $id }

controls-action-tune = Sintonizza VFO

controls-action-rit-offset = Offset RIT

controls-action-xit-offset = Offset XIT

controls-action-volume = Volume

controls-action-sub-volume = Volume Sub-RX

controls-action-squelch = Squelch

controls-action-filter-width = Larghezza filtro

controls-action-filter-shift = Shift filtro

controls-action-agc-max-gain = Guadagno max AGC

controls-action-manual-gain = Guadagno manuale (AGC off)

controls-action-tx-drive = Pilotaggio TX

controls-action-tune-drive = Pilotaggio TUNE

controls-action-mic-gain = Guadagno microfono

controls-action-digi-audio-freq = Offset audio digi

controls-action-spectrum-zoom = Zoom spettro

controls-action-spectrum-pan = Scorrimento spettro

controls-action-spectrum-floor = Minimo spettro

controls-action-spectrum-ceiling = Massimo spettro

controls-action-ptt = PTT

controls-action-tune-carrier = Portante TUNE

controls-action-mute = Muto

controls-action-noise-blanker = Noise blanker

controls-action-nr-cycle = Riduzione rumore (ciclo)

controls-action-auto-notch = Notch automatico

controls-action-binaural = Audio binaurale

controls-action-agc-cycle = AGC (ciclo)

controls-action-sub-rx = Sub ricevitore

controls-action-split = Split

controls-action-rit-enable = RIT on/off

controls-action-xit-enable = XIT on/off

controls-action-rit-clear = Azzera RIT

controls-action-xit-clear = Azzera XIT

controls-action-vfo-toggle = Alterna VFO A/B

controls-action-swap-vfos = Scambia A↔B

controls-action-copy-a-to-b = Copia A→B

controls-action-band-up = Banda su

controls-action-band-down = Banda giù

controls-action-mode-next = Modo successivo

controls-action-mode-prev = Modo precedente

controls-action-record-toggle = Registrazione on/off

controls-action-abort-tx = Interrompi trasmissione

controls-action-cw-straight = Tasto verticale CW

controls-action-voice-stop = Stop voice keyer

controls-action-tone-burst = Burst di tono 1750 Hz

controls-action-fit-span = Adatta span

controls-action-zoom-in = Ingrandisci

controls-action-zoom-out = Riduci

controls-action-peak-hold = Peak hold

controls-action-spectrum-collapse = Comprimi spettro

controls-action-waterfall-collapse = Comprimi waterfall

controls-action-waterfall-flip = Inverti waterfall

controls-action-toggle-help = Finestra del manuale

controls-action-toggle-settings = Finestra Impostazioni

controls-action-toggle-logbook = Finestra Logbook

controls-action-toggle-spots = Finestra Spot

controls-action-toggle-memories = Finestra Memorie

controls-action-toggle-voice = Finestra voice keyer

controls-action-workspaces = Finestra Aree di lavoro

controls-action-speak-status = Annuncia stato

controls-action-speak-repeat = Ripeti ultimo annuncio

controls-action-speech-silence = Smetti di parlare

controls-action-speech-toggle = Annunci on/off

controls-action-broadcast-labels = Etichette broadcast (sintonizzata / intera banda)

controls-action-select-vfo = Seleziona VFO { $vfo }

controls-action-band = Banda { $band }

controls-action-mode = Modo { $mode }

controls-action-recall-memory = Richiama memoria { $n }

controls-action-voice-slot = Slot voice keyer { $n }

controls-group-tuning = Sintonia

controls-group-receive = Ricezione

controls-group-transmit = Trasmissione

controls-group-display = Visualizzazione

controls-group-windows = Finestre

controls-group-speech = Voce

controls-midi-note = Nota

controls-midi-cc = CC

controls-midi-pitch-bend = Pitch bend

controls-midi-program = Programma

controls-midi-any = qualsiasi

controls-midi-msg = { $kind } { $data }  can { $ch }

controls-midi-msg-pitch-bend = Pitch bend  can { $ch }

settings-controls-down = giù

settings-controls-rev = inv

antenna-antenna = ANTENNA

antenna-cutting-lengths = Lunghezze di taglio

antenna-the-frequency-to-cut-for = La frequenza per cui tagliare

antenna-optimizing = Ottimizzazione…

antenna-tune-the-yagi-s-element-lengths = Regola le lunghezze e le spaziature degli elementi della Yagi rispetto all'obiettivo scelto. Una ricerca locale in salita su un thread in background; la finestra resta utilizzabile mentre gira.

antenna-solved = Risolto

antenna-optimized-yagi = Yagi ottimizzata

antenna-obstacles-approximate = Ostacoli (approssimativi)

antenna-trees = Alberi

antenna-in-leaf = con foglie

antenna-ridge = Crinale

dab-channel-received-mhz = Canale ricevuto { $channel } { $mhz }

dab-no-channel-received = Nessun canale ricevuto

dab-channel-received = Canale ricevuto { $channel }

dab-slideshow = SLIDESHOW

dab-no-picture-yet = la stazione non ha ancora inviato immagini

dab-no-station-chosen = nessuna stazione scelta

dab-no-logo-yet = ancora nessun logo

dab-playing = ▶ in riproduzione

dab-chosen = scelta

dab-no-text-yet = la stazione non ha ancora inviato testo

dab-clear-channel-hint = Cancella questo canale e le sue stazioni. SCAN lo ritrova.

dab-clear-all-channels = × cancella tutti i canali

dab-clear-all-channels-hint = Dimentica ogni canale e le stazioni su di essi. SCAN le ritrova; le stelle restano.

dab-n-stations = { $n } stazioni

dab-scanning-progress = scansione { $channel } · { $at }/{ $n } · { $found } trovati

dab-clear-station-hint = Cancella questa stazione dall'elenco. La prossima SCAN la riporta se l'ensemble la trasmette ancora.

dab-prev = PREC

dab-prev-hint = La stazione precedente nell'elenco.

dab-preset = PRESET

dab-preset-hint = Contrassegna questa stazione con la stella, o toglila.

dab-next = SUCC

dab-next-hint = La stazione successiva nell'elenco.

dab-pty-hint = Mostra il tipo di programma di ogni stazione nell'elenco.

dab-stop-sweep-hint = Ferma qui la scansione. Quanto trovato finora resta.

dab-scan-hint = Percorre ogni blocco della Banda III e tiene quelli che portano stazioni.

swl-title = LOG SWL

swl-new = + NUOVO

swl-jobs = ATTIVITÀ

swl-jobs-hint = Registrazioni programmate — registra una banda a un'ora stabilita

swl-sigid = ID SEG

swl-sigid-hint = Cosa c'è su questa sintonia? Una guida ai segnali per frequenza, modo e larghezza di banda

swl-replay = RIASCOLTA

swl-replay-hint = Riproduci gli ultimi due minuti invece della diretta — recupera l'identificativo della stazione che ti sei appena perso

swl-report = RAPPORTO

swl-report-hint = Copia questa ricezione come rapporto da inviare alla stazione

swl-stats-heard = { $n } ascoltate

swl-stats-reported = { $n } riportate

swl-stats-qsl = { $n } QSL

swl-stats-pirates = { $n } pirata

swl-stats-hint = L'intero log — i filtri Mostra non lo modificano

swl-antenna = Antenna

swl-antenna-hint = Filare lungo 20 m, loop MLA-30, mini-whip …

swl-antenna-tip = Va nella riga Antenna del rapporto di ricezione

swl-tone = Tono

swl-tone-on = attivo

swl-tone-bass = Bassi

swl-tone-mid = Medi

swl-tone-treble = Acuti

swl-report-copied = Rapporto copiato negli appunti — incollalo nel tuo programma di posta. Non è stato possibile aprire alcun gestore di posta.

swl-show = Mostra

swl-find-hint = stazione, lingua, sito …

swl-find-tip = Corrisponde a stazione, lingua, sito del trasmettitore e note

swl-all-bands = Tutte le bande

swl-all-days = Tutti i giorni

swl-pirates-only = Solo pirata

swl-pirates-only-tip = Solo le catture non autorizzate

swl-adif-tip = Salva l'intero log come ADIF — registri di ricezione, non collegamenti

swl-csv-tip = Salva l'intero log come CSV, una riga per ricezione

swl-shown-of = { $shown } di { $n }

swl-empty = Ancora nulla registrato — sintonizza una stazione e premi + NUOVO.

swl-transmitted-from = trasmesso da { $site }

swl-click-to-copy = (clicca per copiare)

swl-addr-btn = ind.

swl-mail-btn = posta

swl-mark-sent = inviato

swl-recall = ric

swl-recall-tip = Torna a questa frequenza e a questo modo — vedi se la stazione è tornata

swl-edit = modifica

swl-del = canc

swl-no-match = Nessuna ricezione corrisponde al filtro — CANCELLA le mostra tutte.

swl-form-new = NUOVA RICEZIONE

swl-form-edit = MODIFICA RICEZIONE

swl-station-hint = Radio Taiwan International

swl-language-hint = Inglese

swl-received = Ricevuta

swl-received-hint = il tuo locatore

swl-email = E-mail

swl-email-hint = rapporti di ricezione

swl-email-tip = Va nella riga A: del rapporto

swl-postal = Postale

swl-postal-hint = via, città, paese

swl-postal-tip = La destinazione alternativa del rapporto quando non c'è l'e-mail

swl-heard = Ascoltata

swl-smeter = S-meter

swl-report-label = Rapporto

swl-judged = stimato

swl-report-sent = Rapporto inviato

swl-not-yet = non ancora

swl-qsl-received = QSL ricevuta

swl-awaiting = in attesa

swl-notes = Note

swl-notes-hint = note sul programma

swl-pirate = Pirata

swl-pirate-tip = Una trasmissione non autorizzata — una stazione che trasmette fuori da qualsiasi assegnazione. Il log la contrassegna con una bandiera pirata.

swl-mail-why-no-address =
    Questa stazione non ha un indirizzo e-mail — cercalo nella finestra PALINSESTO, oppure digitane uno qui, e questa funzione diventa disponibile.

    Un rapporto di ricezione è una cortesia che una stazione legge e a cui risponde con una cartolina QSL; senza un indirizzo non c'è dove inviarlo.

swl-mail-why-bad-address = Quell'indirizzo e-mail non è un singolo indirizzo, quindi non viene usato — un rapporto consegnato a un estraneo che non ha mai ascoltato la stazione è peggio di nessuno.

swl-mail-why-ok =
    Scrivi il rapporto di ricezione e aprilo nel tuo client di posta.

    L'immagine e la clip audio vanno allegate da te lì — questo programma non può mettere un file in un messaggio.

swl-mail-report = INVIA RAPPORTO…

antenna-feed-ohms = alim. ≈ { $r } Ω

antenna-takeoff = angolo di partenza { $deg }°

antenna-director = direttore { $n }

antenna-reflector = riflettore

antenna-driven = radiatore

antenna-gap = spaziatura { $v } m

antenna-az = az

antenna-depth = profondità

antenna-height = altezza

antenna-attenuation = attenuazione sulla direzione 0°: { $att } dB

morse-no-paddle-found-connect-one-then = nessun paddle trovato — collegane uno, poi riapri questo pannello

morse-send-with-a-paddle-played-here = Trasmetti con un paddle — riprodotto qui, mai in aria

morse-no-paddle-found-in-dev-input-by-id = Nessun paddle trovato in /dev/input/by-id. Collegane uno e riapri questo pannello. Funziona un box keyer che riporta i suoi due contatti (spesso come pulsanti di un mouse); non funziona uno che fa da sé la manipolazione iambic e invia pressioni.

morse-reverse = INVERTI

morse-swap-dit-and-dah-the-switch-on = Scambia punto e linea. L'interruttore su molti paddle lo fa sul tasto stesso; questo serve quando è nell'altra posizione.

morse-stop-key = FERMA TASTO

morse-start-key = AVVIA TASTO

morse-key-the-character-above-the-keyer = Manipola il carattere qui sopra. Il keyer rende puliti punti e linee — tu scegli quale paddle e quanto fare pausa. Un tocco è un elemento; tieni premuto per ripetere.

morse-the-sending-drill-needs-a-linux = L'esercizio di trasmissione richiede un desktop Linux e un paddle.

panels-float-chip-hint = FLOAT — stacca questo pannello in un pannello flottante e restituisce spazio al waterfall; il chip di aggancio su di esso lo rimette a posto

panels-map-style-radar = Radar — sfondo scuro, terra riempita. Ciò su cui si legge un quadro aereo

panels-map-style-terrain = Rilievo — terra rialzata e riscaldata perché si legga come suolo, confini attenuati

panels-map-style-chart = Carta — terra chiara, confini evidenziati e reticolo, l'aspetto aeronautico

panels-heard-me-chip = MI HANNO SENTITO

panels-heard-me-hint = Cerchia ogni stazione che ha segnalato di aver sentito questo nominativo sulla banda attuale, da PSK Reporter, posizionata sul locatore del segnalatore stesso. L'inverso degli spot PSK: quelli sono stazioni che questa radio sta ascoltando. Aggiornato ogni ora, copre l'ultima ora; richiede il feed PSK attivo in Impostazioni → Spot e un nominativo impostato nella scheda Generale.

panels-period-label = periodo

panels-sub-mode-label = sotto-modo

panels-front-end-overloading = ⚠ FRONT END IN SOVRACCARICO — riduci il guadagno RF (LNA/IF), oppure sposta l'antenna

panels-front-end-overloading-hint = Il ricevitore è pilotato oltre il fondo scala, quindi l'audio è forte ma non porta alcun segnale — nulla verrà decodificato finché dura. Abbassa il guadagno RF/LNA (più attenuazione), riduci il guadagno IF, oppure attiva l'AGC. Un forte trasmettitore vicino, o un altro apparato sulla stessa banda, è la causa abituale.

panels-enigma-hint = Macchina Enigma — cifra un messaggio su un frontale d'epoca, e risolvi un testo cifrato copiato dall'etere

panels-slot-bar-heard-hint = Segue il suono negli altoparlanti: bianca mentre un frame è udito, verde quando è terminato, e torna all'inizio quando comincia il successivo. I tempi sono quelli del modo, quindi un frame più breve riempie comunque la barra.


morse-send-correct = corretto: { $want }

morse-send-heard = sentito { $got }, atteso { $want }

morse-send-received = ricevuto: { $text }

amp-title-model = AMP · { $model }

amp-state-off = OFF

amp-state-on = ON

amp-relay-dn = GIÙ

amp-relay-up = SU

amp-band-unknown = N/D

amp-needle-top = MAX

decodes-not-uploaded = ⚠ { $services } non caricato

decodes-not-uploaded-hint = Il collegamento è nel log locale, ma il caricamento non è riuscito. Controlla il token e la connessione in Impostazioni → Spot, poi caricalo di nuovo dal logbook.

decodes-logged-to = ↑ registrato su { $services }

decodes-logged-to-hint = Caricato anche sui logbook online impostati per ricevere ogni nuovo collegamento.

decodes-known = NOTI

decodes-known-hint = Ogni nominativo che un <...> con hash qui può attualmente risolvere, con il suo paese. Un hash non si può leggere al contrario — una stazione compare solo dopo essere stata sentita compitare il proprio nominativo, ed è a questo che serve una semplice chiamata in 11 m.

decodes-ignored-clear = { $n } ignorate · cancella

decodes-auto-tx-frequency-hint =
    Scegli automaticamente la nostra frequenza di trasmissione: il punto più libero nel periodo in cui trasmettiamo, anziché la frequenza di chi stiamo rispondendo — trasmette nell'altro periodo, quindi la sua non dice nulla su chi c'è quando noi trasmettiamo. Disattivato NON mantiene la frequenza: risponde sulla frequenza della stazione chiamata. Per mantenerla, usa Hold TX.

    Una banda è esente: in 11 m la norma WSJT-CB è rispondere a un CQ sul tono stesso di chi chiama e restarci per tutto il collegamento, quindi la ricerca viene saltata e una risposta a un CQ resta sulla frequenza a cui è stata sentita, che questo sia attivo o no.

decodes-muted-caps = SILENZIATO

decodes-muted-hint = Silenziato per questa sessione — clicca per riascoltarlo

decodes-ignore-hint = Ignora questa stazione per il resto della sessione: esce dall'elenco delle decodifiche, smette di far suonare gli avvisi e non viene mai proposta alla modalità automatica. Non viene inviato né caricato nulla a suo nome, e riavviare sdroxide la riporta.

decodes-qso-complete-hint = Il collegamento è completo e nel log locale. Viene tenuto qui per qualche minuto così il messaggio finale può essere reinviato se l'altra stazione ripete il suo — non è dovuto altro.

decodes-slot-even = pari

decodes-slot-odd = dispari

decodes-before = prima

decodes-after = dopo

decodes-beam-hint = Punta l'antenna sul locatore di questa stazione

decodes-queue-next = prossima

decodes-queue-waiting = in attesa


decodes-setup-hint = SETUP — l'identità della stazione e i modelli condivisi da questi modi QSO

decodes-auto-armed-hint =
    { $role } è armato. Clicca per fermarlo.

    { $note }

    Passo del motore: { $step } — una stazione che risponde a un CQ viene presa solo in CallingCq senza alcun collegamento in corso.

decodes-auto-rx-hint = Rispondi al CQ di una nuova stazione, senza presidio, e non chiamare mai CQ da solo. Si risponde solo ai nominativi non già nel log. Il watchdog di trasmissione ne scandisce il ritmo e lo stop per inattività lo disarma.

decodes-auto-continuous-hint = L'intero flusso di lavoro, senza presidio: concludere il QSO, lavorare chi ti chiama, rispondere a una stazione che chiama CQ, e chiamare CQ solo quando non c'è altro. Va avanti finché non lo fermi{ $until }. { $bound }

decodes-auto-or-minutes-up = { " " }o scadono { $n } minuti

decodes-auto-cq-run-unlimited = Ogni ciclo di CQ non ha un limite proprio di chiamate.

decodes-auto-cq-run-bounded = Ogni ciclo di CQ è limitato dal proprio numero di chiamate.

decodes-auto-no-call-limit = Nessun limite proprio di chiamate — il watchdog e lo stop per inattività pongono fine al ciclo.

decodes-auto-after-calls-listen = Dopo { $repeats } chiamate passa ad AUTO RX e continua ad ascoltare.

decodes-auto-after-calls-stop = Dopo { $repeats } chiamate si ferma.

decodes-auto-cq-hint = Chiama CQ senza presidio e lavora chi risponde. { $bound } Un ciclo troppo breve e una banda tranquilla non ti sente mai; troppo lungo e il trasmettitore resta su una frequenza a cui nessuno risponde.

decodes-auto-stopped-by-operator = { $role } fermato dall'operatore

decodes-cont = cont

settings-servers-decoded-text-udp = Testo decodificato via UDP

settings-servers-decoded-text-udp-note = Una riga JSON per decodifica, per uno script tuo: la famiglia FT8, WSPR e le righe decodificate di ogni modo da tastiera (CW, RTTY, PSK, Olivia, THOR, MFSK, DominoEX, Hell), ciascuna con la sintonia su cui è stata sentita. Il broadcast WSJT-X qui sopra è un protocollo da logger e non ha messaggi per questi modi, ed è per questo che questa funzione esiste a parte.

settings-servers-decoded-text-udp-hint = host:porta, su UDP. Vuoto è disattivato — il predefinito. Solo uscita: nulla su questo socket può toccare la radio. Un nome viene risolto quando premi APPLICA.

settings-servers-rotator-note = Punta un'antenna motorizzata sul satellite su cui sei agganciato, oppure a mano dalla finestra ROTATORE e dalle mappe. Il predefinito è un demone Hamlib rotctld (esegui ad es. “rotctld -m 603 -r /dev/ttyUSB0” per un rotore GS-232B, o “rotctld -m 1” per provarlo senza hardware) — un demone copre ogni rotore che Hamlib pilota. Si può anche pilotare direttamente un controller su porta seriale.

settings-servers-connection = Connessione

settings-servers-rotctld-port-hint = 4533 è il predefinito di rotctld

settings-servers-baud-hint = La maggior parte dei controller va a 9600

settings-servers-pstrotator-note = La porta UDP propria di PSTRotator. Nel suo Setup, attiva UDP Control; risponde ai comandi su questa porta e risponde sulla successiva.

settings-servers-el = el

settings-servers-persist-re-connect-rotator = Salva e (ri)connetti al rotore

settings-servers-tci-note = Permette ai programmi compatibili TCI di pilotare questa radio: controllo di frequenza e modo, IQ a banda larga, audio in ricezione e audio in trasmissione. In WSJT-X scegli il tipo di apparato SunSDR (TCI), puntalo a questo indirizzo e imposta il PTT su TCI.

settings-servers-decoded-text-udp-format = Ogni riga è un datagramma, terminato da a capo: { $example }

settings-servers-transport-rotctld = Hamlib rotctld (TCP)

settings-servers-transport-easycomm = EasyComm II (seriale)

settings-servers-transport-gs232 = Yaesu GS-232 (seriale)

settings-servers-transport-pstrotator = PSTRotator (UDP)

spots-active = ATTIVE

spots-closing = IN CHIUSURA

spots-none-left-in-the-window = nessuna rimasta nella finestra

spots-openings = APERTURE

spots-band-opening-detections-paths = Rilevamenti di apertura di banda: percorsi la cui attività recente ha superato la propria base di 3 ore

spots-save-the-spots-you-can-see-as-a = Salva gli spot che vedi come file KML — Google Earth e qualsiasi altro strumento cartografico che lo legga. Solo i filtri attivi, e solo gli spot che portano una posizione, perché uno spillo senza coordinate è uno spillo dall'altra parte del mondo.

spots-no-spots-to-save-the-filters-are = Nessuno spot da salvare — i filtri sono tutti disattivati

spots-bc-all = BC TUTTE

spots-bc-tuned = BC SINTONIZZATA

spots-which-broadcasters-the-waterfall =
    Quali emittenti il **waterfall** nomina.

    { "**" }BC SINTONIZZATA** è il predefinito e la regola di bebepip: il nome sulla stazione su cui sei sintonizzato, e nient'altro — una banda etichettata da un capo all'altro è un muro di nomi sulla banda passante che stai cercando di leggere.

    { "**" }BC TUTTE** nomina ogni stazione che il palinsesto ha in onda ora, ovunque si trovi, ed è ciò che vuoi mentre scorri una banda anziché ascoltare una stazione. Non dice nulla sul fatto che il chip BC qui sopra mostri le emittenti o meno.

app-rx-only-hint = Questa radio può solo ricevere — non ha un trasmettitore da commutare.

spots-n-calls = { $n } chiamate

spots-opening-hover = Percorso in banda { $band } da { $from } a { $to }: l'ultima finestra di 15 minuti ha superato la base di 3 ore del percorso — chiamanti recenti: { $calls }


spots-download-unsupported = Impossibile scaricare { $source } su questa piattaforma

spots-kml-partial = { $placed } di { $total } salvati — gli altri non hanno posizione e non si possono fissare

spots-net-log = LOG NET ({ $n })

decodes-beam = PUNTA

settings-radio-no-answer-yet = In attesa del computer a cui è collegata la radio: queste richieste riguardano il suo hardware e la sua rete, e risponde una alla volta.

settings-radio-elad-no-baud = la radio non ha l'impostazione { $baud } baud — verrà usato { $default }

settings-radio-elad-antenna-rx-only = Solo RX

settings-radio-rx-only-n = Solo RX{ $n }

settings-radio-ng-level = Livello nG

settings-radio-ng-level-hint =
    nG aggiunge due comandi che il firmware 2.00x non ha, e questo è l'unico posto in cui si possono impostare: nG non risponde a nessuno dei due e non li memorizza, quindi sdroxide li reinvia ogni volta che il collegamento si apre.

    Volume è il livello che la radio invia lungo il flusso USB. nG prende quell'audio dopo VOL e AGC, quindi questo è il posto giusto per correggere un decodificatore che vede troppo poco o troppo — non il mixer del computer.

    Il controllo del guadagno è l'impostazione AGC propria della radio. Auto la mette in DIGI per qualsiasi modo digitale e su on altrimenti, che è ciò che le note di nG chiedono per FT8: il guadagno allora si mantiene durante le pause di trasmissione dell'altra stazione invece di regolarsi verso l'alto in esse.

settings-radio-ng-volume-hint = 0–31, come `AG0nn;`. Il livello che la radio trasmette al computer.

settings-radio-gain-control = Controllo del guadagno

settings-radio-speaker = Altoparlante

settings-radio-on-while-streaming = Attivo durante lo streaming

settings-radio-ng-speaker-hint = Disattivato spegne l'altoparlante della radio mentre trasmette in streaming (`UA2;`), così l'audio si sente solo in sdroxide. La 2.00x non aveva questa forma.

settings-radio-ng-agc-auto = Auto (DIGI nei modi dati)

settings-radio-ng-agc-off = Off

settings-radio-ng-agc-on = On (veloce)

settings-radio-sound-card-note = Quale scheda audio porta l'audio di questa radio si trova in **Audio**, insieme ai tuoi altoparlanti e al microfono — è la stessa coppia di impostazioni con qualsiasi radio tu stia usando, quindi è un'unica pagina.

settings-radio-no-control-cable-audio-page = Questa radio non ha cavo di controllo — si commuta da sola tramite VOX, quindi non c'è alcun PTT da comandare. La trasmissione è audio nel microfono della radio, ed è per questo che l'audio di trasmissione della radio è nella pagina Audio e non qui.

settings-radio-sound-card-note-short = Quale scheda audio porta l'audio si trova in **Audio**.

settings-radio-control-link = Collegamento di controllo

settings-radio-atsmini-host-hint =
    Il server TCP ad hoc del ricevitore: `atsmini.local` (mDNS) oppure il suo IP. Sulla radio, Settings → TCP Port → Ad hoc, con il Wi-Fi in Connect, AP+Connect o AP Only.

    `atsmini.local` richiede un resolver mDNS (Avahi/nss-mdns) su questo computer. Se il collegamento di controllo non si stabilisce, il suo IP è mostrato nella schermata Wi-Fi della radio — inserisci quello.

settings-radio-atsmini-find-hint = Cerca nella rete di questo computer la porta di controllo del ricevitore

settings-radio-tcp-port = Porta TCP

settings-radio-tcp-port-hint = Il firmware usa 60000 come predefinito.

settings-radio-atsmini-receive-only = Solo ricezione: il Si4732 demodula in hardware, quindi il modo scelto qui viene comandato alla radio e tutto il resto — rivelazione, decodifica, log — avviene su questo computer.

settings-radio-atsmini-tuning-note =
    La sintonia segue il ciclo di bande proprio del ricevitore, quindi la sintonia è in ritardo rispetto alla tua rotella per un attimo — di più quando la frequenza passa in un'altra banda. Lascia che si assesti prima di continuare a scorrere, e atterrerà sempre sulla frequenza richiesta.

    Il passo di sintonia del ricevitore è suo proprio (il comando PASSO qui sopra): una variazione più fine di quel passo viene arrotondata sulla radio, e la sintonia torna al passo su cui è effettivamente atterrata.

settings-radio-live-controls = Comandi in tempo reale

settings-radio-atsmini-vol = VOL

settings-radio-atsmini-step = PASSO

settings-radio-atsmini-sent-at-once = Inviato subito alla radio. VOL è il volume proprio del ricevitore (la sua manopola, e quello del Si4732), non il cursore AF di sdroxide.

settings-radio-refresh = Aggiorna

settings-radio-read-99-slots = Leggi i 99 slot dal ricevitore

settings-radio-tune-radio-slot = Sintonizza questa radio sullo slot

settings-radio-no-slots-stored = Nessuno slot memorizzato sul ricevitore.

settings-radio-press-refresh-slots = Premi Aggiorna per leggere gli slot di memoria del ricevitore.

settings-radio-icomnet-receive-hint = AF: la radio demodula e sdroxide mostra la banda audio. IF 12 kHz: la radio invia invece la sua frequenza intermedia DRM e sdroxide demodula, il che porta i propri filtri, riduzione del rumore e decodificatori su circa ±12 kHz. In ogni caso il waterfall largo è lo scope della radio stessa — nessun Icom fornisce I/Q.

settings-radio-copy-icom-session-hint = Copia negli appunti l'ultima sessione di questa radio — handshake e traccia CI-V — (o in un file, in un browser che non lascia copiare a una pagina), per una segnalazione di bug. Di questa radio, non della stazione: con due Icom sulla LAN ogni scheda risponde del proprio indirizzo.

settings-radio-copy-protocol-trace-hint = Copia negli appunti la traccia di protocollo dell'ultima sessione (o in un file, in un browser che non lascia copiare a una pagina), per una segnalazione di bug.

settings-radio-copy-trace-device-hint = Copia negli appunti la traccia dell'ultima sessione (o in un file, in un browser che non lascia copiare a una pagina), per una segnalazione di bug: ogni comando scambiato con il dispositivo e i primi byte del flusso di campioni.

settings-radio-copy-trace-receiver-hint = Copia negli appunti la traccia dell'ultima sessione (o in un file, in un browser che non lascia copiare a una pagina), per una segnalazione di bug: ogni comando scambiato con il ricevitore e i primi byte del flusso di campioni.

settings-radio-pluto-choose-receiver = Scegli il tuo ricevitore

settings-radio-dab-rate-hint = Scegli 2.500 Msps se vuoi decodificare DAB/DAB+

settings-radio-rx888-dab-rate-hint = Scegli 80.0 Msps e la larghezza 2.500 MHz (1/32) se vuoi decodificare DAB/DAB+

settings-radio-pluto-both-chains-hint = Per ascoltare su entrambe le antenne: aggiungi una seconda radio con lo stesso indirizzo e impostala su RX2. Entrambe le radio condividono la stessa frequenza.

settings-radio-duplex-tdd-hint =
    Se l'AD9361 funziona in entrambe le direzioni insieme (FDD, come si avvia un Pluto e come sdroxide l'ha sempre lasciato) oppure una alla volta (TDD).

    Lascialo su FDD a meno che tu non voglia i pin PTT qui sotto — il TDD è ciò da cui essi commutano, ed esclude il Full duplex qui sopra. Ha effetto con Applica.

    PlutoSDR: il full duplex è disattivato — il TDD abilita una direzione alla volta, ed è da questo che commutano i pin GPO di trasmissione-ricezione. Imposta i pin PTT su Off e il duplex su FDD per ascoltare durante un passaggio.

settings-radio-fm-mw-notch = Notch FM/MW

settings-radio-fm-mw-notch-hint = Notch hardware sulla banda broadcast FM (88–108 MHz) e sulle onde medie, per quando un trasmettitore locale sovraccarica tutto il resto. Ha effetto immediato.

scanner-broadcast-band = Banda broadcast

scanner-all = TUTTE

scanner-stop-only-on-channels-that-sound = Fermati solo sui canali che sembrano qualcuno che parla: una voce è periodica, riempie la banda del parlato e sale e scende alcune volte al secondo, cosa che una portante, un birdie e un burst di dati non fanno. Ogni candidato viene ascoltato per almeno mezzo secondo, perché è ciò che serve per sentire una sillaba.

scanner-nothing-in-this-scan-is-in-a-mode = Nulla in questa scansione è in un modo che porta il parlato — una ricerca vocale qui girerebbe senza fermarsi mai, quindi non viene offerta finché non ce n'è uno. FM, AM e le bande laterali portano voci; i modi digitali no.

scanner-voice = VOCE

scanner-stop-only-where-somebody-is = fermati solo dove qualcuno sta parlando

scanner-no-voice-to-look-for-in-these = nessuna voce da cercare in questi modi

scanner-sql = SQL

scanner-how-long-to-stay-on-a-candidate = Quanto restare su un candidato prima di giudicarlo. Il controllo della voce richiede almeno questo tempo: una voce è un ritmo di sillabe, e mezzo secondo ne contiene due

scanner-listening-for-a-voice = in ascolto di una voce…

scanner-scan-band-hover = Scansiona { $name } ({ $lo }-{ $hi } kHz)

scanner-range-to = a

scanner-mode-no-speech = { $mode } non porta parlato — questi canali vengono saltati

scanner-voice-holding = voce { $score } · qualcuno sta parlando

scanner-voice-passed = saltato { $mhz } MHz · voce { $score }

aircraft-hfdl-aircraft = Aereo HFDL { $id }

aircraft-no-icao = nessun indirizzo ICAO (HFDL)

aircraft-provenance = { $source }, { $age } fa

aircraft-label-flight = Volo

aircraft-label-registration = Marche

aircraft-label-position = Posizione

aircraft-label-altitude = Quota

aircraft-label-speed = Velocità

aircraft-label-track = Rotta

aircraft-label-last-message = Ultimo messaggio

aircraft-label-type = Tipo

aircraft-label-operator = Operatore

aircraft-label-origin = Origine

aircraft-label-destination = Destinazione

aircraft-label-online = Online

aircraft-missing-flight = non ancora ricevuto — lo portano ADS-B o VDL2

aircraft-missing-registration = non ancora ricevuto — lo porta solo VDL2

aircraft-missing-position = non ancora ricevuta — ADS-B, oppure un rapporto di posizione VDL2 o HFDL

aircraft-missing-adsb = non ancora ricevuto — lo porta ADS-B

aircraft-missing-message = nessuno ancora — VDL2, oppure audio ACARS dopo che VDL2 ha nominato l'aereo

aircraft-online-unavailable-disabled = online non disponibile — l'interruttore Online è spento

aircraft-online-unavailable-no-station = online non disponibile — nessuna stazione da interrogare; i dati radio sono tutto ciò che c'è

aircraft-online-unavailable-unauthorized = online non disponibile — accesso alla stazione non effettuato

aircraft-online-unavailable-retry = ricerca online non riuscita — nuovo tentativo a breve

aircraft-switch-no-answer = la stazione non ha risposto

aircraft-switch-not-signed-in = accesso alla stazione non effettuato

aircraft-switch-refused = la stazione ha rifiutato la richiesta

aircraft-switch-too-old = la stazione non offre questo interruttore (troppo vecchia?)

aircraft-switch-failed = la stazione non ha potuto cambiarlo

aircraft-online-source = adsbdb, tramite la stazione

aircraft-asking-station = interrogazione della stazione

aircraft-provenance-radio = radio

aircraft-provenance-radio-online = radio + online

aircraft-provenance-online = online

aircraft-photo-credit = Foto: { $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = Nessun rotore d'antenna configurato.

rotor-set-one-up-in-settings-servers-a = Configurane uno in Impostazioni ▸ Server — un demone Hamlib rotctld, oppure un controller su porta seriale. Poi punta il fascio da questa sintonia, da un nominativo o dalla mappa.

rotor-halt-the-antenna-where-it-is = Ferma l'antenna dove si trova

rotor-park = PARCHEGGIA

rotor-hand-the-antenna-back-to-the = Restituisci l'antenna all'aggancio del satellite — insegue il satellite, e si parcheggia quando non ce n'è.

rotor-point = PUNTA

rotor-dx-country = Paese DX

rotor-set-your-grid-locator-settings = Imposta il tuo locatore (Impostazioni ▸ Generale) per puntare su un paese.

rotor-connected = Connesso — antenna a { $az }°

rotor-not-connected-why = Non connesso: { $e }

rotor-beam = Fascio { $az }° az  { $el }° el

rotor-target = bersaglio { $az }° / { $el }°

rotor-park-hover = Ruota alla direzione di parcheggio ({ $az }° / { $el }°)

rotor-country-bearing = { $name } — direzione { $az }°

aircraft-switch-no-station = non c'è alcuna stazione da commutare

aircraft-title = AEREI

aircraft-mode-adsb-hover = ADS-B, 1090 MHz — aerei con posizione, quota e velocità

aircraft-mode-hfdl-hover = HFDL, collegamento dati HF — aerei sugli oceani e ai poli, dai loro rapporti di posizione

aircraft-mode-vdl2-hover = VDL2, collegamento dati VHF — messaggi dagli aerei; pochi portano una posizione

aircraft-mode-acars-hover = ACARS — brevi messaggi tra aerei e terra

aircraft-badge-online = online

aircraft-badge-history = storico

aircraft-stats-total = { $n } aerei

aircraft-stats-most-first = Prima i più numerosi

aircraft-stats-empty = Ancora nessun aereo. Le tabelle si riempiono da ADS-B, VDL2, HFDL e ACARS; nulla qui richiede internet.

aircraft-stats-relay-note = Modello, operatore e rotta contano solo gli aerei su cui il relay ha risposto — apri una scheda in Schede per cercarne uno.

aircraft-stats-known-kinds = { $known } noti · { $kinds } tipi

aircraft-and-more = e altri { $more }

aircraft-show = Mostra

aircraft-aviation = Aviazione

aircraft-cards-chip = Schede { $n }

aircraft-cards-hover = Ogni aereo ricevuto su ADS-B, VDL2, HFDL e ACARS, una scheda ciascuno — conservata per dieci minuti, qualunque sia il modo su cui è la radio ora

aircraft-watch-chip = Watch { $n }{ $suffix }

aircraft-watch-hover = Aerei di cui hai chiesto di essere avvisato, per nominativo di volo o marche — l'elenco è tenuto dalla stazione, e un avviso viene mostrato quando uno va in onda

aircraft-stats-chip = Statistiche

aircraft-stats-hover = Le categorie, compagnie, modelli, operatori e rotte più comuni tra gli aerei che questa stazione conosce — contati prima dalla radio, con le schede del relay come complemento

aircraft-export = Esporta

aircraft-export-csv-hover = Salva gli aerei come tabella CSV per un foglio di calcolo: le ultime 24 ore della stazione se tiene uno storico, altrimenti ciò che si riceve ora. Senza posizioni.

aircraft-export-json-hover = Salva gli aerei come JSON: le ultime 24 ore della stazione se tiene uno storico, altrimenti ciò che si riceve ora. Senza posizioni.

aircraft-watch-intro = Nominativi e marche che vuoi tenere d'occhio. L'elenco è conservato dalla stazione, quindi è lo stesso da ogni schermo; un avviso arriva quando uno compare, o torna in aria dopo dieci minuti di assenza.

aircraft-watch-needs-station = L'elenco di controllo è conservato dalla stazione a cui sei connesso. Connettiti a una stazione per usarlo.

aircraft-watch-reading = Lettura dell'elenco…

aircraft-watch-cannot-read = Impossibile leggere l'elenco: { $why }.

aircraft-watch-input-hint = KLM1234 o PH-BXA

aircraft-watch-add = Aggiungi

aircraft-watch-sending = Invio…

aircraft-watch-empty = Ancora nulla nell'elenco.

aircraft-watch-on-air = in aria ora

aircraft-watch-remove = Rimuovi

aircraft-watch-alerts = Avvisi

aircraft-watch-clear = Cancella

aircraft-watch-none-yet = Ancora nessuno.

aircraft-cards-empty = Nessun aereo ricevuto finora. Le schede si riempiono da ADS-B, VDL2, HFDL e ACARS: premi uno di questi e ascolta. Qui nulla richiede internet.

aircraft-cards-no-match = Nessun aereo corrisponde a “{ $query }”.

aircraft-history-hover = Mostra dove è stato questo aereo nell'ultimo giorno, dal registro della stazione stessa. Interroga la tua stazione, mai internet.

aircraft-copy-summary = Copia riepilogo

aircraft-copy-summary-hover = Copia questa scheda come testo: cosa si sa e da dove viene ogni riga. La posizione e l'ultimo messaggio sono esclusi.

aircraft-asking-station-dots = interrogo la stazione…

aircraft-not-changed = Non modificato: { $why }.

aircraft-loading-photo = caricamento foto…

aircraft-radio-is-on = La radio è su { $mode }: premi { $label } per ascoltare. Questo è ciò che { $label } ha ricevuto prima.

aircraft-no-adsb = Nessun ADS-B ricevuto finora.

aircraft-hfdl-off = Il decodificatore HFDL è spento: premi HFDL per accenderlo.

aircraft-vdl2-starting = Avvio del decodificatore VDL2…

aircraft-no-view = { $label } non ha una vista qui.

textmodem-waiting-modem = in attesa del modem…

textmodem-locked = AGGANCIATO

textmodem-no-frame = NESSUN FRAME

textmodem-locked-hint = Un frame è in tracciamento. Viene accettato solo se la correlazione di sincronismo, ripiegata su dodici frame, si distingue chiaramente dal proprio nullo e l'autocorrelazione del periodo di frame indica la presenza di un frame.

textmodem-sync-readout = sync { $sync } (nullo { $null })

textmodem-sync-hint = La correlazione differenziale di sincronismo ripiegata su ogni fase di frame, e la mediana su quelle fasi: il nullo che deve superare. Misurato via radio: 0,85 su un 4285 contro 0,14 su una forte portante PSK di un altro modo.

textmodem-frame-readout = frame { $acf }

textmodem-frame-hint = Quanta parte di un frame si ripete un frame dopo. Un 4285 ripete i suoi 80 simboli di sincronismo, quindi 0,3-0,6; la portante di un altro modo misura circa 0,03.

textmodem-refs-readout = rif { $a }/{ $b }/{ $c }

textmodem-refs-hint = Le concentrazioni dei tre blocchi di riferimento dopo lo scrambling inverso. Prima dello scrambling sono tutti il simbolo 0, quindi un frame corretto fa collassare ciascuno su un unico punto: 0,7 e oltre. Sedici simboli di rumore danno 0,25.

textmodem-rate-hint = Ciò che la costellazione può dire. Ogni velocità non codificata condivide la costellazione con una codificata, quindi le velocità codificate richiedono l'Annex E per essere distinte, e il loro payload richiede la FEC dell'Annex E per essere letto.

textmodem-offset-frames = { $offset } Hz · { $n } frame

textmodem-offset-frames-hint = L'offset di portante su cui concordano i blocchi di riferimento, e quanti frame sono stati demodulati in questa sessione.

textmodem-som-readout = SOM { $best }/32 · { $n } msg

textmodem-som-hint = Il pattern di inizio messaggio dell'Annex E, e quanto l'uscita del decodificatore gli si è avvicinata. Il rumore sta intorno a 16 su 32; una trasmissione codificata arriva a venti e oltre anche quando un fading ne impedisce la corrispondenza, e una corrispondenza è ciò che individua la velocità codificata e avvia un messaggio. Il codice e l'interleaver decodificano esattamente un segnale generato, ma nessuna stazione codificata è ancora stata letta via radio, quindi una corrispondenza qui è la prima cosa da aspettarsi quando ce ne sarà una.

textmodem-rx-only = SOLO RX

textmodem-rx-only-hint = Il decodificatore di questo modo è stato portato e legge le sue registrazioni; il suo modulatore non è ancora scritto, quindi la parte in trasmissione è disattivata invece di rifiutare a cose fatte.

textmodem-msg-hint = I tuoi pulsanti di messaggio: condizioni di lavoro, il meteo, una risposta standard. Ciascuno invia tutto il suo testo in un colpo solo, e F1–F9 premono i primi nove. Viaggiano con la configurazione della stazione, quindi anche un client remoto li ha.

textmodem-weather-chip = {" "}METEO { $n }{" "}

textmodem-weather-hint =
    Rapporti SYNOP e SHIP decodificati ricevuti su questa banda.

    Sono conservati qui e non nell'elenco di decodifica perché le cifre da sole non significano nulla, e perché il meteo in una posizione di tre ore fa è ciò a cui si torna.

    Trovati nel testo RTTY man mano che arriva, quindi il modo deve essere sintonizzato sullo shift e sul baud della stazione. Una stazione a terra non invia la posizione, quindi quelle righe riportano il numero di stazione e le cifre ma nessuna coordinata; un rapporto da nave porta la propria.

textmodem-olivia-reverse-hint = Inverte l'ordine dei toni, in entrambe le direzioni. Una stazione che trasmette invertita viene letta come rumore finché questa opzione non è attiva.

textmodem-olivia-tones-raised = ({ $n } di Olivia → 4)

textmodem-ifk-reverse-hint = Inverte i passi dei toni. IFK+ porta i dati nel passo tra toni successivi, quindi una stazione ricevuta sulla banda laterale opposta ha ogni passo negativo e viene letta come rumore finché questa opzione non è attiva.

textmodem-domino-hint = { $name } — { $baud } baud, 18 toni, modulazione incrementale di frequenza senza correzione d'errore in avanti

textmodem-mt63-long-hint = MT63-{ $label }, larghezza { $bw } Hz — l'interleave lungo distribuisce una raffica su { $chars } caratteri

textmodem-mt63-short-hint = MT63-{ $label }, larghezza { $bw } Hz, interleave corto — l'interleave corto distribuisce una raffica su { $chars } caratteri

textmodem-mt63-not-offered = 1K e 500L non ancora offerti

textmodem-mt63-not-offered-hint = MT63-1K e l'interleave lungo a 500 Hz si leggono con il decodificatore batch ma non ancora attraverso il ricevitore in tempo reale: l'aggancio cerca la fase dei simboli e la polarità ma non il centro banda, e una ricerca che lo faccia si è rivelata scambiare una variante con un'altra. Vedi docs/internal/MODE-PROGRESS.md.

textmodem-mfsk-caveat-mfsk8 = { " " }— la registrazione del corpus perde un tratto del suo testo; il decodificatore è lo stesso usato da ogni altra variante

textmodem-mfsk-caveat-mfsk11 = { " " }— la registrazione del corpus perde il gruppo iniziale a causa dell'avvio del ricampionatore

textmodem-mfsk-hint = { $label } — { $baud } baud, { $tones } toni, larghezza { $bw } Hz{ $caveat }

textmodem-mfsk-reverse-hint = Inverte l'ordine dei toni. Una stazione ricevuta sulla banda laterale opposta a quella con cui è stata trasmessa arriva speculare rispetto al suo centro e viene letta come rumore finché questa opzione non è attiva.

textmodem-psk-family = Famiglia

textmodem-psk-family-plain-hint = La famiglia propria del PSK31: una portante differenziale, nessuna correzione d'errore in avanti

textmodem-psk-family-bpsk63f-hint = Il codice robusto a 62,5 baud, senza interleaver: l'unico modo codificato che fldigi trasmette senza

textmodem-psk-family-robust-hint = Una portante con il codice robusto e un interleaver 2x2xN

textmodem-psk-family-multi-hint = Più portanti, varicode del PSK31 e **nessun** codice: più larghi e veloci, e richiedono un segnale pulito

textmodem-psk-family-robust-multi-hint = Più portanti **e** il codice robusto: il più largo e il più robusto dei modi PSK

textmodem-psk-baud-fast-hint = { $baud } baud — si legge dopo circa un secondo: a questa velocità il tempo di aggancio del ricevitore è un terzo di un messaggio

textmodem-psk-baud-hint = { $baud } baud

textmodem-psk-modulation = Modulazione

textmodem-psk-bpsk-hint = Un bit per simbolo, rivelazione differenziale

textmodem-psk-qpsk-hint = Due bit per simbolo con un codice di rateo 1/2, quindi un bit di dati per simbolo: la stessa velocità di testo del BPSK alla stessa velocità, con metà degli errori

textmodem-psk-8psk-hint = Tre bit per simbolo, nessun codice: il più veloce dei tre a parità di velocità e il meno robusto. Solo ricezione: non esiste un trasmettitore a otto fasi. Provato su un segnale sintetico: le registrazioni 8PSK nel corpus di test non sono una costellazione pulita

textmodem-psk-coded = codificato

textmodem-psk-no-code = senza codice

textmodem-psk-coded-hint =
    { $baud } baud, { $n } { $n ->
            [one] portante
           *[other] portanti
        }, { $code }, larghezza { $bw } Hz — solo ricezione: il trasmettitore PSK codificato non è scritto

textmodem-psk-wide-bank-hint = { "*" }*Questo banco largo non decodifica ancora.** Le sue portanti sono così vicine che il filtro del ricevitore alla frequenza d'ingresso non riesce a separarle; i banchi più stretti leggono le proprie registrazioni. Nulla di ciò che senti qui va perso: è il front end, non il segnale.

textmodem-hell-slant = Inclinazione

textmodem-hell-slant-hint = Correzione del clock di campionamento in ricezione. Una cattura i cui punti arrivano lunghi dello 0,4 % — una normale scheda audio, ed è ciò che misura la registrazione Hell via radio di SkySweep — richiede circa -4000. Regola finché i glifi non stanno dritti.

settings-net-rbn-enabled-hint = Legge gli skimmer di tutto il mondo e li usa per alimentare la mappa di propagazione. È ciò che fa mostrare alla mappa le bande che questa radio non sta ascoltando. Non emette nulla in aria, non richiede un account e usa il nominativo della scheda Generale. Gli spot RBN non compaiono nell'elenco spot: sono misurazioni, non inviti. L'host non è legato a reversebeacon.net: va bene qualsiasi nodo che parli lo stesso feed telnet.

settings-net-rbn-port-hint = Su reversebeacon.net, 7000 è il feed CW/RTTY, 7001 quello FT8/FT4

settings-net-wavelog-intro = Wavelog è un logbook che **tu** gestisci, a un indirizzo che gli dai. I contatti registrati qui vengono caricati nella postazione che scegli, e i contatti che già contiene possono essere scaricati in modo che “già collegato” conosca i QSO fatti da un altro programma, un altro apparato o dal suo modulo web.

settings-net-wavelog-address-hint = Dove si trova la pagina principale di Wavelog: https://log.example.org, o il suo indirizzo LAN. /index.php è accettato in entrambi i casi.

settings-net-wavelog-key-hint = Una chiave API dalla pagina API di Wavelog. Per caricare serve una chiave di lettura/scrittura; una di sola lettura può solo scaricare.

settings-net-pick-one = sceglierne una

settings-net-test-to-list-locations = prova la connessione per elencare le postazioni

settings-net-trust = FIDATI

settings-net-trust-hint = Accetta **questo** certificato e nessun altro, per questo server. Un pin e non “accetta tutto”: se mai cambiasse, la connessione viene rifiutata e lo dice, invece di andare tranquillamente a chiunque risponda a quell'indirizzo.

settings-net-test-wavelog = Prova Wavelog

settings-net-test-wavelog-hint = Chiede al server se questo indirizzo e questa chiave funzionano, ed elenca le sue postazioni. Applica prima le impostazioni; non registra nulla.

settings-net-wavelog-download = Scarica i contatti da Wavelog

settings-net-every-min = Ogni (min)

settings-net-locations = Postazioni

settings-net-download-now = Scarica ora

settings-net-download-now-hint = Importa ciò che Wavelog ha aggiunto dall'ultimo scaricamento e segna come caricati i contatti che già possiede. Nulla di ciò che è già nel logbook viene modificato.

settings-net-wavelog-upload-note = Solo i contatti registrati **qui** vengono caricati automaticamente; l'arretrato di Wavelog si invia spuntando un QSO nel logbook, così nulla lascia questa macchina senza una tua scelta.

settings-net-broadcast-note = Le stazioni in onde lunghe e corte etichettate sul waterfall. Ogni fonte qui sotto viene scaricata per la stagione di trasmissione corrente e riscaricata al cambio di stagione. Confluiscono tutte in **un unico archivio**, quindi il filtro Fonte della finestra PALINSESTO ne mostra una o tutte.

settings-net-downloaded = scaricato

settings-net-schedules = Palinsesti

settings-net-schedule-yours = tuo

settings-net-download = Scarica

settings-net-hfcc-hint = HFCC pubblica i nomi delle sue stazioni in un file separato, quindi servono entrambi. Finché non li ha, lo dice invece di caricare un palinsesto di soli nominativi.

settings-net-refetch-hint = Scarica di nuovo la copia di questa stagione, sostituendo quella in cache

settings-net-from-file = Da file…

settings-net-aoki-file-hint = Prendi il palinsesto di Aoki da un file che hai salvato tu: lo .zip pubblicato da Aoki, o il .txt al suo interno. Il suo sito cambia con la stagione, ed ecco perché questa opzione è qui.

settings-net-eibi-file-hint = Prendi il palinsesto di EiBi dal suo sked .csv, salvato su questa macchina

settings-net-filter-aoki = Palinsesto Aoki

settings-net-filter-eibi = Palinsesto EiBi

settings-net-download-all = Scarica tutto ora

settings-net-download-all-hint = Scarica ogni fonte scaricabile, mantenendo quelle che già funzionano

settings-net-own-list-note = Il tuo elenco è un **palinsesto**, cioè ciò che ti aspetti di ricevere, non il registro di ascolto, che riporta ciò che hai effettivamente ricevuto. Importalo qui sotto, o modifica il file.

settings-net-import-list = Importa il mio elenco di stazioni…

settings-net-import-list-hint = Legge un CSV o un semplice elenco di stazioni e lo unisce al file qui sopra. Le frequenze possono essere in kHz o MHz; l'unità si ricava dal numero.

settings-net-filter-station-lists = Elenchi di stazioni

settings-net-dx-status = DX — { $body }

settings-net-pin-own = proprio — { $pin }…

settings-net-schedule-rows = { $name } { $n } trasmissioni

settings-net-schedule-not-downloaded = { $name }     non scaricato

settings-net-schedule-imported = { $source } { $season } — { $n } trasmissioni, ora nell'archivio

settings-net-stations-imported =
    { $n ->
        [one] { $n } stazione nel tuo elenco, ora nell'archivio
       *[other] { $n } stazioni nel tuo elenco, ora nell'archivio
    }

txaudio-natural = Naturale

txaudio-ragchews-and-local-contacts-the = Chiacchierate e contatti locali: il denoiser, un expander delicato nelle pause, compressione leggera. Suona come te, senza la stanza.

txaudio-ssb-everyday = SSB quotidiano

txaudio-everyday-ssb-100-hz-2-9-khz-a = SSB quotidiano: 100 Hz – 2,9 kHz, un po' di presenza attorno a 2 kHz, compressione moderata.

txaudio-ssb-punch-dx = SSB punch / DX

txaudio-weak-signals-and-pileups-a-narrow = Segnali deboli e pileup: una voce stretta 300 Hz – 2,7 kHz con forte enfasi di presenza, spinta in un compressore veloce con il rotatore di fase, per la massima potenza di parlato che il trasmettitore può dare. Non è hi-fi.

txaudio-ssb-punch-for-hours-in-a-noisy = SSB punch per ore in una stazione rumorosa: il denoiser e un gate deciso tengono ventole e il prossimo operatore fuori dalle tue pause, e il compressore lavora ancora di più.

txaudio-essb-hi-fi = ESSB / hi-fi

txaudio-wide-ssb-or-am-50-hz-4-khz-with = SSB largo o AM: 50 Hz – 4 kHz mantenendo i bassi, compressione delicata e un de-esser. Richiede un filtro di trasmissione così largo — e una banda silenziosa.

txaudio-fm-repeater = FM / ripetitore

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM: un passa-alto ripido a 300 Hz tiene la voce lontana dal tono CTCSS, il limite superiore si ferma a 3 kHz per il canale e il limiter mantiene la deviazione.

txaudio-mobile-noisy-shack = Mobile / shack rumoroso

txaudio-fans-engine-traffic-the-denoiser-a = Ventole, motore, traffico: il denoiser, un passa-alto ripido a 150 Hz e un gate deciso, con compressione sufficiente a stare sopra ciò che resta.

txaudio-digital-voice-2 = Voce digitale

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE: solo il denoiser e un delicato livellamento, nessun EQ — il vocoder preferisce il parlato naturale. Attiva VOCE DIGITALE.

txaudio-imported-2 = Importato

frame-no-contrast-fit = Nessun contrasto — clic per ADATTARE

frame-levels-hidden-hint = Il minimo/massimo del display sono oltre tutto ciò che c'è sullo schermo, quindi il waterfall è di un solo colore uniforme. Clicca per adattarli a ciò che la radio sta ricevendo.

frame-bw-waterfall = Waterfall BW · { $span }

frame-bw-waterfall-unknown = Waterfall BW · —

frame-every-module-own-window = ogni modulo è nella sua finestra

frame-window-chip-hint = FINESTRA — sposta questa finestra in una finestra del sistema operativo a sé, per metterla su un altro monitor

frame-dock-panel-pane-hint = ACCORPA — riporta questo pannello nel riquadro sotto il waterfall

frame-dock-panel-hint = ACCORPA — rimette questo pannello sotto il waterfall

frame-dock-spectrum-hint = ACCORPA — riporta questo spettro nella finestra principale, come chiudere questa senza cercare il suo pulsante di chiusura.

frame-dock-band-menu-hint = ACCORPA — riporta il selettore di banda e modo nella finestra principale, come chiudere questa senza cercare il suo pulsante di chiusura.

frame-dock-rotator-hint = ACCORPA — riporta il rotore nella finestra principale, come chiudere questa senza cercare il suo pulsante di chiusura.

frame-module-panadapter = panadapter

frame-module-panel = pannello

frame-module-controls = controlli

frame-module-aux-panadapter = panadapter aux

frame-module-band-mode = banda e modo

frame-module-rotator = rotore d'antenna

frame-detached-title = { $flavor } — { $name }

frame-workspace-missing = Nessuno spazio di lavoro { $n } salvato: salvane uno in Impostazioni › UI › Spazi di lavoro e questo tasto lo richiamerà.

frame-no-operating-panel = Nessun pannello operativo in { $mode }: è sganciato, e questo modo non ne ha.

frame-rx-only-nudge = Questa radio è solo ricevente: non ha trasmettitore. Nascondere i controlli di trasmissione e usare la schermata di ascolto?

frame-listening-controls = Controlli di ascolto

frame-listening-controls-hint = Nasconde i controlli di trasmissione e porta questa radio sulla schermata di ascolto (Impostazioni → Radio). Nulla viene risintonizzato e il decodificatore continua a funzionare.

frame-update-available = SDR Oxide Brown { $tag } è disponibile: stai usando { $version }.

frame-download = Scarica

frame-download-hint = Apre la pagina della release nel browser

frame-tuning-catching-up = sintonia — la radio si sta adeguando

frame-roll-up-hint = Arrotola — lascia la barra del titolo e restituisce la finestra al waterfall

frame-unroll-hint = Srotola — mostra di nuovo il pannello

frame-window-panel-hint = FINESTRA — dà a questo pannello una finestra propria, da mettere su un altro monitor

frame-panel-window-title = { $mode } — pannello

frame-auto-stopped-stop-control = auto fermato: è stato usato un controllo di arresto

frame-dont-show-again = Non mostrare più

frame-dont-show-again-hint = Ricorda questa presa visione su questa schermata. Il blocco resta disattivato finché il motore gira con --oob-tx: questo impedisce solo di mostrare di nuovo l'avviso.

frame-cb-window-title = ⚠  11 m (CB) — NON È UNA BANDA RADIOAMATORIALE

frame-cb-about-to-allow = Stai per consentire la trasmissione sulla banda degli 11 m.

frame-cb-not-amateur = Gli 11 m non sono una banda radioamatoriale. Sono un servizio radio separato con propri canali, modi, limiti di potenza e apparati omologati, e questi differiscono da paese a paese. Trasmettere lì è soggetto alle norme del paese da cui operi.

frame-cb-only-continue = Continua solo se sai cosa consente il tuo paese sui 27 MHz e operi nei suoi limiti: resti responsabile della trasmissione.

frame-cancel = Annulla

frame-saved-to-profile = salvato nel profilo { $name }

frame-saved-as-default = salvato come predefinito di questa stazione (il server non ha password, quindi è ciò che ottiene ogni client)

frame-bindings-offer-title = Questo profilo contiene associazioni di tastiera

frame-bindings-offer-profile = Il profilo con cui hai effettuato l'accesso ({ $name }) memorizza sul server le associazioni di tastiera e mouse, e questo browser non le sta usando. È l'impostazione predefinita sicura: su una stazione condivisa con altri anche la tastiera è condivisa, e un accesso che adotta il PTT o la barra spaziatrice di un altro è un rischio reale.

frame-bindings-offer-default = Il profilo predefinito di questa stazione memorizza sul server le associazioni di tastiera e mouse, e questo browser non le sta usando. È l'impostazione predefinita sicura: su una stazione condivisa con altri anche la tastiera è condivisa, e un accesso che adotta il PTT o la barra spaziatrice di un altro è un rischio reale.

frame-bindings-offer-choice = Usale qui, oppure lasciale. In ogni caso ti verrà detto cosa è stato fatto, e nulla cambia senza una tua scelta.

frame-bindings-use = Usa le associazioni di questo profilo

frame-bindings-keep-mine = Tieni le mie (e non chiedere più)

frame-bindings-using-profile = uso le associazioni di tastiera memorizzate per questo profilo

frame-bindings-keeping-own = mantengo le associazioni di tastiera di questo dispositivo; non verrà chiesto di nuovo

frame-look-back-to-profile = ritorno all'aspetto memorizzato per questo profilo

frame-look-none-stored = ancora nessun aspetto memorizzato per questo profilo: salvane uno prima

frame-antenna-rotator-caption = ROTORE D'ANTENNA

bands-condx-global-forecast-rest-what = CONDX è una previsione globale. WSPR è ciò che la rete WSPR mondiale ha ricevuto (wspr.live); PATHS, REACH e BEST sono ciò che questa stazione e la RBN hanno ricevuto. Una riga vuota significa che nulla è stato decodificato, il che può voler dire che la banda era chiusa, o solo che nessuno c'era.

bands-km-s-parent-peak-rate =
    { $name } ({ $code }) — { $velocity_kms } km/s, origine { $parent }. Tasso di picco ZHR { $zhr } intorno a { $peak }.

    Un radiante sopra l'orizzonte significa che le scie possono raggiungerti; uno sciame veloce lascia scie ionizzate più durature per il meteor scatter sui 6 m e 2 m, e per le brevi aperture sui 10 m/11 m.

bands-hearing-means-path-open-each = { $callsign } a { $location } — riceverlo significa che il percorso { $band } è aperto. I 10 m (28,200 MHz) sono il riferimento più vicino in banda radioamatoriale per le condizioni sugli 11 m. Ogni beacon sale di una banda ogni 10 s, quindi questa riga cambia a ogni slot.

bands-wspr-activity-old = · attività WSPR di { $age } fa (wspr.live)

bands-psk-activity-old = · attività PSK di { $age } fa

bands-activity-hover =
    Attività globale su { $band } negli ultimi 15 minuti, da { $source }:
    { $paths } rapporti di ricezione · { $tx } trasmettitori · { $rx } ricevitori

    { $note }

    Appare più luminosa quanto più la banda è affollata; dice che la banda viene ricevuta da qualche parte, non che sia aperta per te.

bands-derived-hover =
    { $band } non è una delle bande che HAMQSL.com valuta; questo è il verdetto del gruppo { $group } pubblicato, letto come sostituto più vicino.

    Una previsione, non una misura del tuo percorso.

bands-wspr-note = La rete WSPR mondiale — beacon per segnali deboli.

bands-psk-note = Rapporti di ricezione dai modi di attività: FT8, FT4 e i rapporti CW/RTTY che i beacon WSPR non coprono.

aircraft-history-reason-disabled = la stazione non ha attivato il suo storico

aircraft-history-reason-unauthorized = la stazione richiede un accesso

aircraft-history-reason-no-station = lo storico è conservato dalla stazione a cui sei connesso

aircraft-history-reason-unknown = la risposta della stazione non è stata compresa

aircraft-history-show = Mostra storico

aircraft-history-hide = Nascondi storico

aircraft-label-history = Storico

aircraft-label-first-heard = Prima ricezione

aircraft-label-last-heard = Ultima ricezione

aircraft-label-track-points = Punti traccia

aircraft-label-highest = Massima

aircraft-label-callsigns = Nominativi

aircraft-label-registrations = Marche

aircraft-history-nothing-kept = ancora nulla conservato per questo aereo

aircraft-history-by-station = da questa stazione

aircraft-history-kept-24h = conservati per le ultime 24 h

aircraft-history-of-kept = dei punti conservati

aircraft-history-used-24h = usati nelle ultime 24 h

aircraft-history-seen-24h = visti nelle ultime 24 h

aircraft-history-looked-up = cercato { $age } fa

aircraft-alert-on-air-as = { $entry } è in aria, come { $callsign }

aircraft-alert-on-air = { $entry } è in aria

aircraft-watch-already = Già nell'elenco.

aircraft-watch-absent = Non è nell'elenco.

aircraft-watch-invalid = Non è un nominativo né una marca: da 3 a 10 lettere o cifre.

aircraft-watch-full = L'elenco è pieno: 100 aerei.

multi-module-windows = FINESTRE DEI MODULI

multi-in-its-own-window = NELLA SUA FINESTRA

multi-docked = ACCORPATO

multi-dock-all-windows = ACCORPA TUTTE LE FINESTRE

multi-bring-every-module-window-back = Riporta ogni finestra di modulo nella finestra principale

multi-undock-all-windows = SGANCIA TUTTE LE FINESTRE

multi-put-every-module-in-a-window-of = Metti ogni modulo in una finestra propria: la finestra principale resta con lo spettro e i decodificatori

multi-two-radios-side-by-side-need-a = Due radio affiancate richiedono una finestra più larga: questa schermata ne mostra una alla volta

multi-the-station-s-windows-some-modules = Le finestre della stazione: alcuni moduli sono in finestre proprie del sistema operativo. Apri questo per cambiarlo.

multi-put-a-module-in-its-own-window-sp1 = Metti un modulo in una finestra propria: SP1 o AUX SP su un altro monitor, il controllo RX accanto. Le stesse righe sono in Impostazioni → UI → Sganciati.

multi-module-sp1 = SP1 — lo spettro

multi-module-sp2 = SP2 (AUX) — un secondo spettro

multi-module-panel = Pannello operativo

multi-module-controls = Controllo RX

multi-module-band-mode = Banda e modo

multi-module-rotor = Rotore d'antenna

multi-bring-back = Riporta { $name } nella finestra principale

multi-put-own-window = Metti { $name } in una finestra propria del sistema operativo: ha un proprio app-id, quindi una regola di finestra può fissarla a un monitor

multi-windows-chip = FINESTRE

aircraft-stats-category = Categoria

aircraft-stats-airline = Compagnia

aircraft-stats-model = Modello

aircraft-stats-operator = Operatore

aircraft-stats-route = Rotta

aircraft-stats-src-no-station = Conteggio degli aerei ricevuti ora. La stazione a cui ti connetti conserva l'ultimo giorno.

aircraft-stats-src-asking = Richiesta alla stazione dell'ultimo giorno…

aircraft-stats-src-ready = Le ultime 24 h conservate dalla stazione, con gli aerei ricevuti ora.

aircraft-stats-src-disabled = La stazione non ha attivato il suo storico: conteggio degli aerei ricevuti ora.

aircraft-stats-src-reason = { $reason } — conteggio degli aerei ricevuti ora.

aircraft-export-saved-station = Salvato { $name }: le ultime 24 ore della stazione.

aircraft-export-saved-local = { $reason }: salvati invece i { $n } aerei ricevuti negli ultimi dieci minuti.

login-remember-this-station = Ricorda questa stazione

login-remember-me = RICORDAMI

login-this-station-keeps-your-sign-in = Questa stazione conserva il tuo accesso, non questa pagina: nulla da compilare sulle altre radio, e nulla da dimenticare quando te ne vai.

login-not-this-time-this-browser-forgets = Non questa volta: questo browser lo dimentica alla chiusura, quindi ti verrà chiesto di nuovo alla prossima sessione e nulla di questo accesso resta sul dispositivo.

aircraft-label-range = Distanza

aircraft-label-vertical = Verticale

aircraft-label-emergency = Emergenza

aircraft-from-your-station = dalla tua stazione

aircraft-footer-disabled = online non disponibile — relay spento sulla stazione

aircraft-footer-no-station = online non disponibile — nessuna stazione da interrogare

aircraft-footer-unauthorized = online non disponibile — accesso non effettuato

aircraft-hover-close = Chiudi (Esc)

aircraft-hover-lookup = Cerca online

aircraft-hover-lookup-hint = Chiede al relay di questa stazione di questo aereo: tipo, operatore, rotta e una foto. Nulla viene richiesto finché non lo premi.

aircraft-hover-lookup-waiting = un'altra ricerca è in corso: una alla volta

aircraft-hover-history-hint = Dove è stato questo aereo nell'ultimo giorno, dal registro della tua stazione. Interroga la tua stazione, mai internet.

auto-stopped-reason = auto fermato: { $reason }

auto-stopped-idle = auto fermato: { $mins } minuti senza intervento dell'operatore

auto-answering = { $role }: risposta a { $call }

auto-caller-went-away = auto: un chiamante se n'è andato

auto-nobody-left = auto: nessuno a cui rispondere

auto-calling-cq = { $role }: chiamata CQ (#{ $n })

auto-cq-no-answer = auto cq: nessuna risposta dopo { $limit } chiamate — in ascolto (auto rx)

auto-listening = { $role }: in ascolto

auto-role-stopped = { $role } fermato: { $sentence }

auto-rx-listening = auto rx: in ascolto

auto-cq-calling = auto cq: chiamata CQ

auto-cont-running = auto cont: in esecuzione

aircraft-search-missing-online = { $missing } aerei su { $total } senza dati online — apri la loro scheda

aircraft-search-shown-of = { $shown } di { $total }

aircraft-search-help =
    Cerca tra gli aerei già ricevuti: indirizzo (ICAO), nominativo, marche, tipo, operatore o categoria. Più parole devono corrispondere tutte. Limita una parola a un campo con icao: call: reg: type: op: — per esempio  reg:PH op:KLM.
    Tipo e operatore provengono dalla ricerca online della stazione, quindi si trovano solo per gli aerei la cui scheda è stata cercata. La ricerca non richiede nulla.

aircraft-search-hint = cerca

aircraft-search-clear = Cancella la ricerca

gridtracker-shade-squares-heard-live-decode = Evidenzia ciò che è ricevuto nell'elenco di decodifica in tempo reale, non solo ciò che è nel log. La versione per ascoltatori della mappa: ciò che è in aria ora, in ciano sotto i segni dei collegati.

gridtracker-grid-ham = GRID/HAM

gridtracker-maidenhead-squares-what-a-ham-log = Quadrati Maidenhead — ciò che porta un log radioamatoriale. Qui non compare alcun nominativo CB: uno scambio sugli 11 m porta un paese, mai un locatore.

gridtracker-country-cb = PAESE/CB

gridtracker-dxcc-countries-at-their-nominal = Paesi DXCC al loro centro nominale — ciò che porta un log CB, dato che uno scambio sugli 11 m non contiene un locatore. Qui non compare nemmeno alcun nominativo radioamatoriale: sono paesi, non quadrati.

colouredit-paste-a-colour-code-rrggbb-or-r-g = Incolla un codice colore (#RRGGBB o r, g, b) — oppure Cmd/Ctrl+V sul campione

cwkey-thread-failed = impossibile avviare il thread del tasto CW: { $error }

cwkey-grab-failed = impossibile prendere il paddle in modo esclusivo: i suoi contatti potrebbero fare clic anche in altre finestre (lo sta usando un altro programma?)

ignore-muted-hover =
    Silenziate per questa sessione: { $names }.

    Una stazione silenziata esce dall'elenco di decodifica, non fa suonare gli avvisi, non viene letta ad alta voce e non viene mai proposta alla modalità auto come stazione a cui rispondere. Nulla viene trasmesso o caricato per suo conto, e l'elenco viene dimenticato alla chiusura di sdroxide: non esiste una copia memorizzata, quindi nulla resta nascosto dopo una serata di ascolto.

    Clicca per riattivarle tutte, oppure premi il pulsante − su una qualsiasi delle sue righe.

js8-setup-hint = CONFIGURAZIONE — nominativo, gruppi, risposta automatica, intervallo del beacon e messaggio di stato

logbook-contest-logger-a-session-dupes-a = Logger di contest — una sessione, doppi, un punteggio in tempo reale e Cabrillo

logbook-up-chip = UP

net-wavelog-stopped = fermato: { $error }

net-wavelog-download-stopped = Scaricamento da Wavelog fermato: { $error }

net-wavelog-new-already-here = { $added } nuovi, { $matched } già presenti

net-wavelog-fill-in-first = prima vanno compilati l'indirizzo e la chiave API

net-wavelog-no-station-location = nessuna postazione da cui scaricare: prova prima la connessione

publicsdr-could-not-connect = impossibile connettersi in questa sessione

publicsdr-use-chip = USA

weather-title = RAPPORTI METEO

weather-intro = Rapporti meteo codificati da navi e stazioni a terra, in RTTY su 4583, 7646 e 10100,8 kHz. SHIP esce intorno alle 02:00 e alle 07:35 UTC, SYNOP intorno alle 00:35 e alle 12:10. Un rapporto è conservato qui e non nell'elenco di decodifica perché le cifre da sole non significano nulla.

weather-nothing-decoded = Ancora nulla decodificato. Non resta che lasciare l'RTTY attivo in uno degli orari qui sopra: sono le ore più silenziose della banda.

weather-head-call-day-hour = { $call } — { $day } alle { $hour }:00 UTC

weather-head-day = alle { $day }:00 UTC

weather-head-report = rapporto

weather-not-reported = { $label }: non riportato

weather-in-their-words = con le loro parole: { $remark }

weather-ago-now = adesso

weather-ago-minute = un minuto fa

weather-ago-minutes = { $m } minuti fa

weather-ago-hours =
    { $h } { $h ->
            [one] ora
           *[other] ore
        } fa

weather-ago-days = { $d } giorni fa

sat-off-only-satellites-with = Off: solo i satelliti con frequenze in archivio — quelli che puoi collegare — e quelli che hai incollato tu. On: ogni satellite tracciato dalla stazione, per lo più quelli che nessuno può ricevere.

adsbmap-tune-hint = doppio clic per sintonizzare · maiusc per l'altro dial

adsbmap-airport-large = grande aeroporto · quota { $elev }

adsbmap-airport-medium = medio aeroporto · quota { $elev }

adsbmap-from-station = { $dist }  { $bear }° dalla tua stazione

adsbmap-aircraft-within =
    { $n } { $n ->
        [one] aereo
       *[other] aerei
    } entro { $dist }

adsbmap-navaid-mw = { $khz } kHz (onde medie)

adsbmap-click-to-close = clicca sulla mappa per chiudere

settings-ui-broadcast-stations-hint =
    Quali stazioni di radiodiffusione possono mostrare la finestra ASCOLTO e le etichette del waterfall.

    { "**" }Su questo span** è ciò che ogni schermata faceva prima che fosse una scelta: mezzo span per lato rispetto al dial, che è ciò che un'etichetta su quel waterfall dovrebbe coprire. **Tutta la banda** mostra tutto ciò che il palinsesto contiene con una posizione al suo interno, ed è ciò che cerca chi scorre una banda, al prezzo di una mappa più affollata.

settings-ui-waterfall-labels = Etichette del waterfall

settings-ui-tuned-only = Solo sintonizzata

settings-ui-tuned-only-hint = Indica la stazione su cui sei sintonizzato e nient'altro: un solo nome, per ciò che stai ascoltando, invece di un muro di nomi sopra una banda passante.

settings-ui-whole-band = Tutta la banda

settings-ui-whole-band-hint = Indica ogni emittente che il palinsesto ha in onda ora, ovunque si trovi. Ciò che serve quando si scorre una banda anziché ascoltare una sola stazione, e più affollato per questo. Anche nella finestra SPOTS, come BC ALL.

settings-ui-classic-ui = UI classica

settings-ui-classic-ui-hint =
    Mette ogni controllo in una sola colonna a sinistra con i gruppi che si ripiegano, come nella disposizione di SDR++ e SDR++Brown, e dà tutto il resto all'immagine. Stessi controlli, stesse impostazioni, disposti come la maggior parte degli ascoltatori in onde corte ha già in mano.

    Richiede una finestra abbastanza larga da lasciare al waterfall la sua parte: sotto quella soglia la colonna viene omessa e la barra è disegnata come al solito, e il pannello si allarga o restringe trascinandolo dal bordo destro. Se disattivata, la barra propria di questo programma viene usata esattamente come prima.

settings-ui-classic-ui-check = controlli in una colonna a sinistra

settings-ui-updates = Aggiornamenti

settings-ui-updates-check = cerca una nuova release all'avvio

settings-ui-updates-hint = Chiede una sola volta all'avvio le GitHub Releases del fork per l'ultima release, e mostra un banner se è più recente di questa build. Contano solo le release complete (la nightly è una pre-release), e disattivato significa nessuna chiamata verso l'esterno.

settings-ui-undocked = Sganciato — finestra propria

settings-ui-docked = Accorpato in questa finestra

settings-ui-dock-all-hint = Riporta ogni finestra sganciata in questa.

settings-ui-undock-all-hint = Metti ogni modulo in una finestra propria: la finestra principale resta con lo spettro e i decodificatori. Un modulo senza nulla da mostrare, come il pannello operativo in un modo voce, non apre alcuna finestra.

settings-ui-spectrum-line = Linea dello spettro

settings-ui-spectrum-line-hint = Il colore sotto la linea dello spettro e come è sfumato: Fisso è un solo colore ovunque, Gradiente è pieno in basso e sfuma verso l'alto, Misto è pieno nella metà inferiore e sfuma nella metà superiore. La linea stessa resta ciano. Il chip RIEMPI nel popup SPEC attiva il riempimento, e la sua riga linea sceglie la sfumatura.

settings-ui-fill = Riempimento

settings-ui-fill-colour = colore

settings-ui-simple-ui = UI semplice

settings-ui-simple-ui-hint =
    Nasconde gli extra avanzati dalla barra superiore: la vista 3D, gli skimmer, i commutatori di livello spettro/waterfall, il tracciamento dei diplomi, i satelliti, la decodifica ISM e la posta via radio. I controlli a cui ricorre un operatore CB o un ascoltatore in onde corte — sintonia, modo, volume, squelch, larghezza di banda, il waterfall, memorie, scansione — restano tutti.

    Nulla viene disattivato, solo nascosto: riattiva questa opzione per riportare i chip.

settings-ui-simple-ui-check = nascondi i chip avanzati

settings-ui-wake-lock = Blocco di sospensione

settings-ui-wake-lock-hint =
    Tiene lo schermo acceso mentre questa pagina è in primo piano, così un telefono o un tablet lasciato sul waterfall non si oscura e non si blocca dopo un minuto senza tocchi. Ricezione, audio e decodifica non ne hanno bisogno — proseguono comunque — serve a mantenere leggibile lo schermo.

    Si abbina al lucchetto sulle barre di telefono e tablet: blocca lo schermo contro le dita vaganti, e tienilo acceso.

    Il browser riprende il blocco ogni volta che la pagina è nascosta — un'altra scheda, o il tasto di accensione del telefono — e viene richiesto di nuovo non appena la pagina torna. Richiede un browser che lo supporti e una pagina sicura (https, o localhost); la riga accanto alla casella dice quando non è disponibile o è stato rifiutato. Consuma batteria, quindi resta disattivato finché non lo chiedi. Il programma installato su un computer segue invece le impostazioni di alimentazione del sistema.

settings-ui-wake-lock-check = tieni lo schermo acceso

settings-ui-sound-welcome = Benvenuto audio

settings-ui-sound-welcome-hint =
    Quando la pagina web si apre — una nuova sessione, o un ricaricamento con F5 — il browser non riproduce alcun suono finché non tocchi la pagina. È una regola del browser, e senza una parola a schermo una sessione nuova sembra non funzionare.

    Con questa opzione attiva, un messaggio al centro dello schermo lo dice: 🔇 Tocca, clicca o premi un tasto ovunque per avviare l'audio. Il primo clic, tocco o tasto — ovunque, con un mouse, un touchpad, un dito o la tastiera — lo chiude e avvia l'audio, e non fa altro: non sintonizza, non preme pulsanti e non esegue scorciatoie. Viene mostrato solo all'apertura della pagina, mai più durante la sessione, qualunque cosa accada poi all'audio.

    Disattivalo quando sai cosa fare: l'audio attende comunque il tuo primo tocco, e quel tocco funziona poi normalmente. Conservato con il tuo profilo sul server. Attivo per impostazione predefinita.

settings-ui-sound-welcome-check = messaggio di benvenuto per l'audio

settings-ui-sound-welcome-browser-only = solo browser — il programma installato riproduce l'audio senza un tocco

settings-ui-profile-screen = Lo schermo segue il tuo profilo

settings-ui-profile-screen-hint =
    Quando sei un client remoto di `sdroxide --server`, l'*aspetto* di questa schermata — tema, disposizione, waterfall e spettro, caratteri, UI semplice, i livelli della mappa, la vista propria del panadapter — è conservato con il profilo con cui hai effettuato l'accesso, così torna su qualsiasi macchina invece di ripartire dai valori predefiniti.

    { "**" }Salvato sul server, e salvato man mano mentre sei connesso** — non c'è nulla da premere. È voluto: non ci si può affidare all'archiviazione del browser, perché `persist()` si comporta in modo diverso in ogni browser, e una sessione conservata lì non può essere azzerata da chi non è davanti a quel dispositivo. Un profilo sul server è un file che l'operatore della stazione può rimettere a posto.

    Su un server **senza password** ogni client è lo stesso profilo, quindi nulla viene scritto a meno che tu non prema il pulsante qui sotto: il tema di un operatore non deve diventare quello del successivo. Nulla che appartenga alla macchina viene mai memorizzato, e una radio locale non è interessata.

settings-ui-save-to-profile = Salva nel profilo

settings-ui-back-to-profile = Torna al profilo

settings-ui-carry-bindings = Porta con te le associazioni dei comandi

settings-ui-carry-bindings-hint =
    Conserva anche le tue associazioni di tastiera e mouse con il profilo sul server, così seguono l'accesso tra browser e dispositivi — la stessa idea della schermata qui sopra, per i tasti.

    { "**" }SCONSIGLIATO, e disattivato per impostazione predefinita.** Su una stazione condivisa da più persone la tastiera è condivisa con essa: un profilo che porta associazioni può riassegnare il PTT, la barra spaziatrice o i tasti di sintonia di un altro operatore nel momento in cui accede, e attivarlo può avere altri effetti non evidenti ora. Attivalo solo dove il server è solo tuo e lo raggiungi da più dispositivi tuoi.

    Solo del fork: non è una funzione di upstream. Le impostazioni dello schermo qui sopra viaggiano sempre; questa è l'unica opzione da attivare qui.

settings-ui-carry-bindings-on = attivo — SCONSIGLIATO

settings-ui-carry-bindings-off = portale con te (sconsigliato)

settings-ui-carry-bindings-title = Portare le associazioni dei comandi sul server?

settings-ui-carry-bindings-body =
    Questo memorizza le tue associazioni di tastiera e mouse con il tuo profilo sul server, così seguono il tuo accesso tra browser e dispositivi.

    Su una stazione usata da altre persone la tastiera è condivisa con essa. Un profilo che porta associazioni può riassegnare il PTT, la barra spaziatrice o i tasti di sintonia di un altro operatore semplicemente accedendo — e attivarlo può avere altri effetti non evidenti ora.

    Per questo è sconsigliato. Attivalo solo se il server è solo tuo.

settings-ui-carry-bindings-confirm = Ho capito — portale comunque

settings-ui-start-swl = Avvia in modo SWL

settings-ui-start-swl-hint =
    Apre ogni sessione con il modo SWL già attivo, così ciò che il programma mostra all'avvio è la schermata di un ascoltatore — per ogni radio, trattandosi di una preferenza della schermata e non di una radio.

    L'interruttore per radio è in Impostazioni → Radio → Controlli di trasmissione, quindi una stazione con una radio solo d'ascolto e un ricetrasmettitore imposta lì quella d'ascolto e lascia questa disattivata. Usare quell'interruttore disattiva questo valore iniziale per la sessione. `--swl` ha sempre la precedenza.

settings-ui-start-swl-check = avvia con il modo SWL attivo

settings-ui-listen-hides-tx = La scheda ASCOLTO nasconde la TX

settings-ui-listen-hides-tx-hint =
    Scegliere **ASCOLTO** nel menu banda/modo nasconde ogni controllo di trasmissione in quella scheda, e scegliere **OPERA** rimette ciò che dice l'impostazione propria di questa radio.

    Non può attivare un trasmettitore. Una radio configurata solo ricevente resta solo ricevente, e il blocco di banda e l'avviso di fuori banda non vengono toccati.

    Il menu banda ricorda su quale scheda è stato lasciato, quindi una stazione lasciata in ascolto torna in ascolto: anche al prossimo avvio i controlli di trasmissione sono nascosti, e scegliere OPERA li riporta. Disattivato per impostazione predefinita perché cambia il significato della scheda; chiede conferma una volta.

settings-ui-listen-hides-tx-check = nascondi la TX nella scheda ASCOLTO

settings-ui-listen-hides-tx-title = La scheda ASCOLTO nasconderà i controlli di trasmissione

settings-ui-listen-hides-tx-body =
    Nella scheda ASCOLTO i controlli di trasmissione scompaiono; in OPERA tornano esattamente come li ha lasciati l'impostazione propria di questa radio.

    Non abilita nulla. È una vista della stessa stazione, quindi il blocco di banda, l'avviso di fuori banda e le protezioni di trasmissione non vengono toccati.

    La scheda viene ricordata: una stazione lasciata su ASCOLTO parte su ASCOLTO, con i controlli di trasmissione ancora nascosti, finché non si sceglie OPERA.

    Da sapere: una stazione lasciata su ASCOLTO riceve comunque su ciò che la scheda ha sintonizzato, e un controllo nascosto non è un controllo spento — la trasmissione è disattivata perché lo dicono le impostazioni, non perché il pulsante è fuori vista.

settings-ui-listen-hides-tx-confirm = Ho capito — nascondili su ASCOLTO

settings-ui-swl-mode = Modo SWL

settings-ui-swl-mode-hint =
    Nasconde ogni controllo di trasmissione per questa radio — il PTT, CALL CQ, livello TX, INVIA, BEACON, tutto — e sostituisce gli extra radioamatoriali della barra (spot, diplomi) con quelli dell'ascoltatore (PALINSESTO e LOG SWL, il registro di ascolto). Lo stesso interruttore di Impostazioni → Radio → Controlli di trasmissione, e per radio: una chiavetta d'ascolto può stare in questo modo mentre il ricetrasmettitore accanto mantiene il suo trasmettitore.

    L'hardware può ancora trasmettere; questo nasconde solo i controlli. `--swl` lo impone per ogni radio per la durata dell'esecuzione.

    Ha effetto con Applica.

settings-ui-swl-mode-check = nascondi tutti i controlli di trasmissione

settings-ui-report-picture = Immagine del rapporto

settings-ui-report-picture-hint =
    L'immagine che un rapporto di ricezione porta quando lo invii a un'emittente: la tua cartolina, una fotografia, ciò che vuoi che la stazione veda. Una sola immagine per la stazione, usata per ogni rapporto.

    Lasciala non impostata e il rapporto parte come solo testo, che è un rapporto più che valido.

settings-ui-report-picture-none = nessuna — il rapporto parte come testo

settings-ui-choose = Scegli…

settings-ui-report-picture-pick = Scegli l'immagine su questo computer

settings-ui-report-note = Nota del rapporto

settings-ui-report-note-hint = La tua nota fissa, precompilata in ogni rapporto di ricezione: chi sei, con cosa ascolti, tutto ciò che una stazione chiede una volta e non a ogni lettera. Testo semplice in un file semplice: non è un segreto.

settings-ui-report-note-hint-text = facoltativo — es. "Dettagli su richiesta"

sstv-keep-dial = MANTIENI DIAL

sstv-keep-dial-on-hint = On: scegliendo SSTV il dial resta dove l'hai sintonizzato, così il decodificatore funziona su qualsiasi frequenza. Le frequenze SSTV pubblicate sono ancora nel selettore ⇵. Clicca per tornare ad atterrare sulla frequenza SSTV della banda.

sstv-keep-dial-off-hint = Off: scegliendo SSTV il dial si sposta sulla frequenza SSTV della banda quando sei a più di 3 kHz da una. Clicca per mantenere invece il dial dove l'hai sintonizzato, e decodificare SSTV su qualsiasi frequenza. Sintonizzare dopo aver scelto SSTV non viene mai annullato in nessuno dei due casi.

sstv-clear-rx = Cancella Rx

sstv-clear-rx-hint = Cancella l'immagine attualmente mostrata nella vista LIVE. Il decodificatore continua a funzionare: usa Riavvia RX per abbandonare un'immagine in corso e attendere la prossima intestazione.

sstv-audio-present-no-header = audio presente · ancora nessuna intestazione SSTV

sstv-editor = Editor…

sstv-editor-hint = Componi l'immagine stessa: testo spostabile, qualsiasi carattere, una striscia attraverso o lungo il quadro

sstv-save-image-as = Salva immagine come…

sstv-save-copy-asks-file = Salva una copia su questo computer: chiede quale file

sstv-picture-still-loading = L'immagine è ancora in caricamento

sstv-share-hint-direct = Invia questa immagine a qualcuno: WhatsApp, Telegram, o qualsiasi altra cosa su questo dispositivo

sstv-share-hint-save = Salva questa immagine e apri un messaggio per inviarla

sstv-share-picture = Condividi immagine…

sstv-reupload-hint = Rimanda fuori questa immagine: carica lo slot di trasmissione { $slot }

sstv-radio-cannot-transmit = Questa radio non può trasmettere

sstv-reupload = Ritrasmetti

sstv-unidentified = non identificato

sstv-send-picture = Invia questa immagine

sstv-qsl-to = A

sstv-the-card = La cartolina

sstv-the-card-hint = Questi vanno su una cartolina stampata accanto all'immagine. Lasciane uno in bianco e semplicemente non viene disegnato: una cartolina non riporta mai un rapporto che nessuno ha dato.

sstv-date-utc = Data (UTC)

sstv-rst-sent = RST inviato

sstv-rst-received = RST ricevuto

sstv-qsl-note = Nota

sstv-add-address = Aggiungi un indirizzo per inviarla.

sstv-saves-opens-mail-hint = Salva l'immagine e apre il tuo client di posta: allegarla richiede un clic lì.

sstv-browser-cannot-attach-hint = Una scheda del browser non può allegare un file a un messaggio, quindi questo salva l'immagine e apre il messaggio; aggiungi l'allegato a mano.

sstv-save-open-mail = Salva immagine e apri posta

sstv-mail-not-opened = Immagine salvata e messaggio copiato: incollalo nel tuo programma di posta. Non è stato possibile aprire alcun gestore di posta.

sstv-qsl-saved-where-chosen = { $name } (salvato dove hai scelto)

sstv-qsl-downloaded = { $name } — scaricato nella cartella dei download di questo browser

sstv-picture-editor = Editor immagine

sstv-slot-n = Slot { $n }

sstv-slot-layers-hint = I livelli propri di questo slot di trasmissione

sstv-station-overlay = Sovrapposizione stazione

sstv-station-overlay-hint = Un insieme di livelli che ogni slot disegna sotto i propri: il nominativo, nell'angolo che preferisci. Impostato una volta, composto con il testo di ogni slot invece di sostituirlo.

sstv-draw-under-slot = disegnala sotto questo slot

sstv-draw-under-slot-hint = Disegna la sovrapposizione stazione sotto i livelli propri di questo slot. Disattivata per un'immagine che porta già l'identificazione — di solito un mosaico ritrasmesso — così non viene impressa due volte.

sstv-overlay-note = La sovrapposizione stazione: ogni slot disegna questi livelli sotto i propri. Trascina una riga sull'immagine per spostarla.

sstv-slot-text-note = Il testo proprio di questo slot, disegnato dove lo metti. Trascina una riga sull'immagine per spostarla; la striscia di una riga si sposta con essa.

sstv-no-picture-in-slot = Nessuna immagine in questo slot: caricane una qui sotto

sstv-base = Base

sstv-base-hint = Su quale immagine di slot vengono disegnati questi livelli

sstv-compose-over-hint = Componi sopra l'immagine di questo slot

sstv-slot-empty = Questo slot è vuoto

sstv-picture-btn = Immagine…

sstv-load-picture-hint = Carica un'immagine in questo slot

sstv-use-received = Usa ricevuta

sstv-use-received-hint = Mette l'immagine appena arrivata sotto questi livelli, così una risposta porta la sua immagine e il tuo nominativo

sstv-received-now-base = l'immagine ricevuta è ora la base

sstv-add-text = ＋ Testo

sstv-add-strip = ＋ Striscia

sstv-n-lines =
    { $n } { $n ->
            [one] riga
           *[other] righe
        }

sstv-draw-this-line = Disegna questa riga

sstv-strip-paren = (striscia)

sstv-draw-later-hint = Disegna questa riga dopo quelle sopra di essa

sstv-draw-earlier-hint = Disegna questa riga prima

sstv-copy = Copia

sstv-face = Carattere

sstv-colour = Colore

sstv-line-text-hint = ciò che dice la riga

sstv-the-letters = Le lettere

sstv-composer-outline = contorno

sstv-the-edge = Il bordo attorno ad esse

sstv-placement = Posizione

sstv-down-frame = lungo il quadro

sstv-down-frame-hint = Ruota la riga di un quarto di giro, così si legge verso il basso: un nominativo lungo il lato dell'immagine

sstv-strip = Striscia

sstv-behind = dietro

sstv-behind-hint = Una striscia di colore dietro la riga, attraverso l'immagine o lungo di essa, centrata sulla riga

sstv-strip-first-edge = La striscia, al suo primo bordo

sstv-thickness = Spessore

adsbpanel-paths = PERCORSI

adsbpanel-paths-hint = Disegna il percorso di ogni volo come un'unica linea: dove è stato, attraverso dove si trova, verso dove sta andando. Se disattivato, la traccia è disegnata come una scia di punti.

adsbpanel-nav = NAV

adsbpanel-nav-hint = Aeroporti e radioassistenze sotto l'immagine, dalle tabelle integrate: i grandi aeroporti sempre, i medi più da vicino, e le radioassistenze quando la vista è abbastanza stretta perché una stazione sia un luogo. I beacon in onde medie, che sono quelli che si possono ricevere, sono disegnati con un colore proprio.

adsbpanel-units = Unità

adsbpanel-units-hint = Come altitudine, velocità, rateo di salita e distanza sono scritti nelle viste degli aerei. Il decodificatore e le esportazioni non sono interessati.

adsbpanel-age = ETÀ

touchlock-locked-hover = Schermo bloccato contro i tocchi accidentali. Tieni premuto qui per 3 secondi per sbloccare.

touchlock-unlocked-hover = Blocca lo schermo contro i tocchi accidentali: frequenza, banda, modo, filtri e waterfall smettono di rispondere a un dito. Ricezione e audio proseguono. Tieni premuto per 3 secondi per sbloccare. Dopo uno sblocco il lucchetto riposa per 10 secondi prima che un tocco lo blocchi di nuovo, così il dito che lo ha sbloccato non può bloccarlo subito.

touchlock-hold = TIENI { $s } s…

touchlock-unlocked = SBLOCCATO

touchlock-locked-hold = BLOCCATO · tieni 3 s

touchlock-unlocked-relock = SBLOCCATO · blocco tra { $s } s

wake-held = schermo mantenuto acceso

wake-asking = richiesta al browser…

wake-unsupported = non disponibile qui — richiede un browser che lo supporti e una pagina https

wake-refused = rifiutato dal browser (risparmio energetico?)

wake-native = solo browser — il programma installato segue le impostazioni di alimentazione del sistema

tooltipcopy-copied = ✔ Copiato

chrome-hide-message-hover = Nasconde questo messaggio per sempre. Nulla delle tue impostazioni cambia, e il consiglio è nel manuale.

chrome-close-message-hover = Chiude questo messaggio. Nient'altro cambia.

settings-ui-window-label = finestra { $module }

settings-ui-undock-hint =
    Sganciato, questo modulo si sposta nella sua finestra e la finestra principale restituisce lo spazio a ciò che resta: metti il waterfall su un secondo monitor, o i controlli su un terzo. Chiudere la sua finestra lo riaccorpa.

    Su Wayland la dimensione viene ripristinata ma è il compositor a decidere dove va una finestra: rendila flottante e fissala a un monitor con una regola di finestra che corrisponda all'app-id "{ $app_id }".

settings-ui-workspace-restore-out = Ripristina questa disposizione — { $n } modulo/i nella/e propria/e finestra/e

settings-ui-profile-in-use-named = in uso: il profilo { $name }

settings-ui-profile-in-use-default = in uso: il predefinito di questa stazione

settings-ui-profile-not-in-use = non ancora in uso — nulla memorizzato per questo accesso

settings-ui-module-panadapter = Panadapter

settings-ui-module-controls = Controlli

settings-ui-module-aux = AUX SP (secondo spettro)

settings-ui-module-band-menu = Selettore di banda e modo

settings-ui-broadcast-scope-in-view = Su questo span

settings-ui-fill-fixed = Fisso

settings-ui-fill-gradient = Gradiente

settings-ui-fill-mixed = Misto

settings-ui-spot-kind-heard-me = MI HANNO SENTITO

settings-ui-wake-held = schermo mantenuto acceso

settings-ui-wake-asking = richiesta al browser…

settings-ui-wake-unsupported = non disponibile qui — richiede un browser che lo supporti e una pagina https

settings-ui-wake-refused = rifiutato dal browser (risparmio energetico?)

settings-ui-wake-native = solo browser — il programma installato segue le impostazioni di alimentazione del sistema

settings-ui-workspaces = Spazi di lavoro

settings-ui-workspaces-note = Una disposizione di finestre con nome: quali moduli sono nelle proprie finestre e dove si trovano. Salva quella che hai, o ripristinane una salvata. Fino a dieci.

settings-ui-workspaces-none = Ancora nessuno spazio di lavoro salvato.

settings-ui-workspace-restore-home = Ripristina questa disposizione — ogni modulo nella finestra principale

settings-ui-workspace-forget = Dimentica questo spazio di lavoro

settings-ui-workspace-name = dai un nome a questa disposizione

settings-ui-workspace-save = SALVA ATTUALE

settings-ui-workspace-full = Dieci spazi di lavoro è il limite che SDRuno conserva; eliminane uno prima

settings-ui-workspace-save-hint = Salva la disposizione attuale con questo nome

settings-ui-workspace-name-first = Scrivi prima un nome

sstv-composer-fade = dissolvenza

sstv-layouts = LAYOUT

sstv-name-layout-hint = dai un nome a questo layout

sstv-keep-layout-hint = Conserva questo layout con quel nome

sstv-layout-renamed = rinominato in "{ $name }"

sstv-layout-saved-as = salvato come "{ $name }"

sstv-rename-cancelled = rinomina annullata

sstv-no-layouts-yet = Ancora nessun layout salvato: dai un nome a uno qui sopra.

sstv-load-layout-hint = Carica questo layout in ciò che stai modificando

sstv-layout-loaded = caricato "{ $name }"

sstv-rename-layout-hint = Rinomina "{ $name }"

sstv-layout-renaming = rinomina di "{ $name }"

sstv-duplicate-layout-hint = Duplica "{ $name }"

sstv-layout-copy-name = copia di { $name }

sstv-layout-duplicated = duplicato come "{ $name }"

sstv-forget-layout-hint = Dimentica il layout "{ $name }"

sstv-layout-forgot = dimenticato "{ $name }"

sstv-mosaic-header = Mosaico da immagini ricevute

sstv-mosaic-nothing = Ancora nulla ricevuto: un mosaico si costruisce con le immagini della galleria.

sstv-mosaic-picked = Scelte { $n } di { $max }, disposte in quest'ordine: l'ordine è la posizione nella griglia. Ogni immagine mantiene la sua forma; gli spazi sono neri.

sstv-mosaic-earlier-hint = Sposta questa immagine di una cella prima

sstv-mosaic-later-hint = Sposta questa immagine di una cella dopo

sstv-mosaic-max = { $n } è il massimo che un mosaico contiene

sstv-mosaic-compose = Componi in questo slot

sstv-mosaic-compose-hint = Costruisce il mosaico e lo carica in questo slot come sua immagine; gli originali ricevuti non vengono toccati

sstv-mosaic-composed = { $n } immagini composte nello slot { $slot }

sstv-clear-picks = Cancella scelte

sstv-composed-saved = salvato

sstv-send = Invia

sstv-send-now-hint = Trasmetti subito questa composizione

sstv-composed-sending = invio

sstv-forget = Dimentica

sstv-forget-hint = Svuota il documento di questo preset: la striscia banner e la didascalia semplice tornano a gestire la sua immagine

sstv-forget-done = cancellato — il banner e la didascalia sono tornati

sstv-copy-to-slot = Copia nello slot

sstv-copy-to-slot-hint = Dà a un altro slot questa immagine e questo testo, così può essere inviata da lì

sstv-copied-into-slot = copiato nello slot { $n }

sstv-word-strip = striscia

sstv-word-text = testo

sstv-text-size = Dimensione del testo

sstv-text-size-hint = Il testo del banner come percentuale della dimensione della striscia stessa. 100% è ciò che è sempre stato; un QSO richiede che il nominativo sia leggibile dopo una decodifica rumorosa, e questo scala le lettere senza rendere più alta la striscia.

sstv-placement-hint = Lungo quale bordo è disegnata la striscia. La sovrapposizione del messaggio parte dall'estremo opposto, quindi un banner in basso lascia la parte alta dell'immagine al testo.

sstv-top = Alto

sstv-bottom = Basso

topbar-ptt-hold-hint = Tieni premuto per trasmettere — un clic lo blocca attivo, e la pressione successiva lo rilascia


topbar-above-the-dial = sopra

topbar-below-the-dial = sotto

topbar-step-chip-hint = Di quanto si sposta il dial a ogni pressione. Tocca per passare al passo successivo: 10 Hz, 100 Hz, 500 Hz, 1, 2,5, 5, 6,25, 9, 10, 12,5 e 25 kHz. Disattiva la riga in Impostazioni › UI.

topbar-mem-chip-hint = Canali di memoria — richiama una frequenza memorizzata

topbar-disp-menu-hint = Il panadapter — i suoi due livelli, le loro velocità e il dettaglio, il peak hold — più i livelli del waterfall, le opzioni di vista e gli skimmer

topbar-phone-menu-hint = Tutto ciò che la barra del telefono non mostra: bande, modi, ricevitore e trasmettitore, le finestre di decodifica, le impostazioni e il manuale

topbar-group-band = Banda

topbar-group-system = Sistema

topbar-caption-diversity = Diversity

topbar-group-decode-windows = Finestre di decodifica

topbar-group-extras = Extra

topbar-grid-tracker-hint = Grid tracker — quadrati Maidenhead collegati su una mappa

topbar-settings-chip-hint = Impostazioni — guadagni del dispositivo, antenne, dispositivi audio

topbar-windows-chip-hint =
    ▣  Sinistro: riporta a casa ogni finestra di modulo
    Destro: l'elenco delle finestre dei moduli

topbar-manual-chip-hint = Manuale utente (F1)

topbar-module-windows-title = Finestre dei moduli

topbar-wname-panadapter = SP1 — lo spettro

topbar-wname-aux = SP2 (AUX) — un secondo spettro

topbar-wname-panel = Pannello operativo

topbar-wname-controls = Controllo RX

topbar-wname-band-menu = Banda e modo

topbar-wname-rotor = Rotore d'antenna

topbar-wstate-put-back = Rimetti { $name } in questa finestra

topbar-wstate-floating-panel-only = Flottante è lo stato proprio del pannello operativo: uno spettro o una console ha un solo posto in questa finestra

topbar-wstate-float = Solleva { $name } dal riquadro e fallo fluttuare sopra questa finestra

topbar-wstate-no-panel = Nessun pannello operativo in { $mode }: si applica in un modo che ne ha uno

topbar-wstate-own-window = Dai a { $name } una finestra propria, su qualsiasi monitor

topbar-wstate-here-now = { $name } è qui adesso

topbar-dock-all = ACCORPA TUTTO

topbar-dock-all-hint = Riporta ogni finestra di modulo in questa finestra

topbar-undock-all = SGANCIA TUTTO

topbar-undock-all-hint = Metti ogni modulo in una finestra propria: questa finestra resta con lo spettro e i decodificatori

topbar-rx-tx-freq = RX { $rx }   TX { $tx } MHz

topbar-band-dock-hint = ACCORPA — tiene aperto il selettore di banda e modo accanto al waterfall invece di chiudere questo popup ogni volta

topbar-band-window-hint = FINESTRA — apre il selettore di banda e modo in una finestra propria, larga quanto vuoi, su qualsiasi monitor accanto al waterfall

topbar-band-return-popup-hint = ACCORPA — riporta il selettore nel popup della barra superiore, dove si chiude dopo ogni scelta

topbar-band-window2-hint = FINESTRA — apre il selettore in una finestra propria, larga quanto vuoi, su qualsiasi monitor

topbar-squelch-held-hint = Lo squelch è tenuto aperto. Clicca per rimettere il gate che avevi.

topbar-squelch-hold-hint = Clicca per tenere lo squelch aperto — per sentire se c'è qualcosa — e clicca di nuovo per rimetterlo esattamente com'era.

topbar-changed-tone = tono

topbar-changed-loudness = LOUDNESS

topbar-nr-chip-hint =
    Riduzione del rumore: { $strength } — { $engine }

    { $advice }

    Clicca per ciclare l'intensità: Bassa / Media / Alta / Off. Il motore è la riga sotto questa.

topbar-nr-off-narrow-hint = La riduzione del rumore è disattivata. Nella riga sotto questa sono offerti cinque denoiser — RNNoise, DeepFilterNet3, spectral-bleach, NR2 e lo spettrale originale — e cliccare qui la attiva a una data intensità. Inizia con RNNoise sulla voce.

topbar-eq-tone-hover = Tono: bassi { $bass } dB, medi { $mid } dB, acuti { $treble } dB

topbar-eq-closed-hover = Tono — un equalizzatore a tre bande sull'audio in ricezione. Clicca per aprire

topbar-eq-loudness-on = LOUDNESS attivo

topbar-stereo-yields-hint = Lo stereo WFM è disattivato mentre la riduzione del rumore o l'auto-notch sono attivi — impasterebbero i due canali. Disattivali per sentire lo stereo

topbar-stereo-playback-hint = Stereo: la registrazione viene riprodotta in entrambe le orecchie, come registrata. Clicca per forzare il mono

topbar-stereo-mono-recording-hint = Questa registrazione è mono — non c'è un secondo canale da riprodurre

topbar-preset-edges = { $lo } … { $hi } Hz

topbar-caption-tone = Tono

topbar-eq-on = on

topbar-eq-bass = Bassi

topbar-eq-mid = Medi

topbar-eq-treble = Acuti

topbar-eq-remembered = Ricordato per { $mode } — ogni modo mantiene il suo.

topbar-loudness-hint = Alza i bassi (e un po' gli acuti) quando abbassi il volume, così un altoparlante basso suona comunque pieno. Funziona con o senza il tono; non fa nulla a volume pieno e non suona mai più forte di quanto farebbe il volume pieno. Disattivato per impostazione predefinita, e ricordato per il modo in cui ti trovi.

topbar-caption-record-audio = Registra audio

topbar-stop-recording-after-min = Ferma la registrazione dopo { $minutes } minuti

topbar-caption-play-back = Riproduci

topbar-recordings-chip = ▶ REGISTRAZIONI…

topbar-recordings-hint = Riproduce una registrazione nei decodificatori — un'immagine SSTV persa, un debole segnale RTTY o CW, uno slot FT8 — con il waterfall e il log di decodifica, come se fosse in aria. La radio si fa da parte mentre riproduce.

topbar-ant-label = ANT

topbar-pwr-label = PWR

topbar-drive-less = meno

topbar-drive-more = più

topbar-fill-hint = Colora la banda sotto la linea dello spettro, piena in basso e sfumata salendo verso la linea, così un segnale si distingue per la sua massa oltre che per il contorno. Il colore si sceglie in Impostazioni → UI → Linea dello spettro. Non disegnato sulla superficie 3D.

topbar-group-spectrum = Spettro

topbar-group-waterfall = Waterfall

topbar-freeze-on-tx-hint = Ferma lo scorrimento del waterfall per la durata della tua trasmissione, e lo riprende da dove si era interrotto — così l'immagine è una storia di ricezione continua e non un blocco di righe che mostrano la tua portante (o, su una radio che silenzia il ricevitore mentre è in trasmissione, righe di rumore mai avvenuto). La linea dello spettro non viene toccata. Utile quando si osserva un modo in cui si trasmette spesso, così il turno dell'altra stazione non viene spinto fuori dallo schermo dal tuo.

topbar-fft-chip-hint = Livelli e contrasto del waterfall, dimensione FFT e direzione di scorrimento

topbar-caption-waterfall = Waterfall

topbar-flatten-hint = Livella il pavimento di rumore nella banda: un tratto più silenzioso dei suoi vicini viene alzato verso il loro pavimento, così un segnale debole si vede su una banda affollata. Alza soltanto — una portante non viene mai abbassata — e mai di più di 20 dB. La traccia dello spettro sopra resta esattamente come misurata.

topbar-log-swl-hover = Registro di ascolto — stazioni ricevute, con SINPO/SIO

topbar-programs-hint = Palinsesto di radiodiffusione — cosa è in onda, quando e dove

topbar-listen-hint = Cosa c'è su questo span, su una mappa — scegli una stazione, sintonizzati, scrivi cosa hai ricevuto e registralo

topbar-spots-hint = Spot in tempo reale — PSK Reporter (e, fuori dal modo SWL, il DX cluster, POTA e SOTA)

topbar-sigid-hint = Cosa c'è su questo dial? Una guida ai segnali per frequenza, modo e larghezza di banda

topbar-extras-hint = EXTRA — DAB e gli altri ricevitori, in una finestra che puoi staccare

topbar-fill-style-caption = linea

topbar-fill-style-hint = Come viene sfumato il colore sotto la linea dello spettro. Il colore stesso si sceglie in Impostazioni → UI → Linea dello spettro.

topbar-fill-fixed = FISSO

topbar-fill-fixed-hint = Un solo colore, uguale ovunque sotto la linea

topbar-fill-gradient = GRADIENTE

topbar-fill-gradient-hint = Pieno in fondo allo spettro, sfuma verso l'alto

topbar-fill-mixed = MISTO

topbar-fill-mixed-hint = Pieno nella metà inferiore, sfuma nella metà superiore

topbar-fill-switch-on = Attiva RIEMPI per impostarlo

topbar-nothing = nulla

topbar-olivia-unconfirmed = OLIVIA — la ricezione funziona, la trasmissione non è testata con altri programmi. Questo decodificatore legge altre stazioni (confermato via radio), quindi puoi copiare un QSO Olivia. La nostra trasmissione ora fa il giro completo attraverso il nostro ricevitore, ma non è stata copiata da fldigi o MultiPSK e una convenzione di segno differisce dal sorgente di fldigi — quindi controlla il primo passaggio invece di aspettarti una risposta.

topbar-dab-bandwidth = DAB / DAB+, provato in aria. Richiede 2,5 Msps di larghezza di banda al front end prima che ci sia audio: agli 1,536 MHz che un ensemble occupa il ricevitore è al suo limite ADC e perde campioni, e una decodifica DAB richiede un flusso continuo, quindi l'ensemble non si aggancia mai e non senti nulla. Imposta prima la frequenza di campionamento della radio a 2,500 Msps.

topbar-stanag-unproven = STANAG 4285 — solo ricezione. L'identificazione è provata via radio: un 4285 si aggancia, il suo frame di 106,67 ms viene misurato e i suoi blocchi di riferimento collassano, così il pannello può dire cosa c'è sulla frequenza e a quale velocità. Leggere il suo *testo* non è provato via radio: il codice e l'interleaver dell'Annex E decodificano esattamente un segnale generato, ma nessuna stazione codificata è ancora stata copiata. Un testo codificato che compare è reale; nessun testo significa che la stazione non è codificata o porta traffico cifrato, che è il caso più comune.

topbar-cis11-unproven = CIS-11 (TORG-11) — solo ricezione. La telescrivente meteorologica russa: 2-FSK a 100 baud, circa 500 Hz di shift e un carattere a 11 bit con un bit di parità dispari. Ciò che è provato è misurato su registrazioni reali: il ricevitore trova da sé i propri toni e legge una trasmissione inattiva con 121 caratteri su 123 validi per parità. Ciò che non è stabilito è come i dieci bit di dati si dividano in caratteri, quindi il pannello mostra i bit oltre alla sua lettura ITA-2 — e il traffico stesso è cifrato, che è il caso più comune.

topbar-thor-unproven = THOR — il lato ricezione non legge ancora le trasmissioni di fldigi. Il decodificatore funziona contro il proprio modulatore ma nessuna registrazione via radio o di riferimento è stata copiata, quindi considera provvisorio ciò che appare.

topbar-mode-not-on-band = { $mode } non è usato su { $band }: scegli una banda a cui appartiene

topbar-radio-classic-hint = RADIO CLASSIC — un aspetto di radio classica per l'ascolto broadcast. Attivarlo gli dà il ricevitore: gli altri modi e le loro finestre vengono chiusi

topbar-ats-bands-caption = Bande offerte dal ricevitore

topbar-ats-band-hover = { $name } — { $mhz } MHz, { $mode } al suo valore predefinito. Porta il ricevitore su questa banda.

topbar-caption-mode = Modo

topbar-caption-digital = Digitale

topbar-ats-receive-only = Solo ricezione — il Si4732 demodula in hardware e non c'è nulla da manipolare.

topbar-band-forecast-derived =
    { $band }: ≈{ $verdict } ({ $time ->
            [day] giorno
           *[night] notte
        }) — il verdetto del gruppo { $group } pubblicato, il sostituto più vicino; HAMQSL.com non valuta nulla per { $band } stessa. Una previsione, non una misura del tuo percorso.

topbar-keypad-readout-khz = { $digits } kHz

topbar-caption-band-keypad = Tastierino bande

topbar-keypad-bands = Bande

topbar-keypad-bands-hint = I tasti indicano le bande: premine uno per andarci, restando nel modo in cui ti trovi. Tornando da MHz si svuota ciò che era stato digitato.

topbar-keypad-mhz-hint = Digita una frequenza sui tasti, in kilohertz: 14074 è 14,074 MHz. INVIO la invia, CANCELLA svuota l'inserimento.

topbar-keypad-key-stay = Resta in { $mode } e sintonizza su { $mhz } MHz — dove si lavora su { $band }.

topbar-keypad-key-go = Vai su { $band }.

topbar-keypad-clear = Cancella

topbar-keypad-clear-typing-hint = Svuota la frequenza che hai digitato. I tasti restano in MHz.

topbar-keypad-clear-bands-hint = Nulla è digitato: i tasti sono sulle bande, dove una pressione porta a una banda.

topbar-keypad-enter = Invio

topbar-keypad-enter-empty-hint = Ancora nulla digitato. I tasti sono in MHz: premili prima.

topbar-keypad-enter-ok-hint = Sintonizza su { $mhz } MHz e torna ai tasti banda.

topbar-keypad-enter-range-hint = { $mhz } MHz è fuori da ciò che questa radio riceve ({ $ranges } MHz).

topbar-nothing-published = nulla di pubblicato

topbar-tab-listen = ASCOLTO

topbar-tab-operate = OPERA

topbar-listen-tab-note = Questa scheda può nascondere anche i controlli di trasmissione: attiva "nascondi la TX nella scheda ASCOLTO" in Impostazioni → UI

topbar-caption-show-bands = Mostra bande

topbar-band-filter-hint = Mostra solo le bande { $filter } nell'elenco qui sotto. Clicca di nuovo per mostrare ogni banda.

topbar-show-every-band = mostra ogni banda

topbar-all-chip = TUTTE

topbar-all-chip-hint = Copertura generale: cancella la banda e lascia andare il dial ovunque. Non è lo stesso del filtro di banda, che restringe solo questo elenco.

topbar-caption-sw-metre-bands = Bande metriche OC

topbar-metre-band-hint = Sintonizza al centro della radiodiffusione { $name }

topbar-caption-wideband = Banda larga

topbar-caption-cb = 11M/CB

topbar-caption-experimental = Sperimentale (non provato)

topbar-caption-receive-modes = Modi di ricezione

topbar-wav-hint = Lo stesso audio come PCM a 16 bit non compresso a 48 kHz — circa 11 MB al minuto in stereo. Per un editor o un decodificatore tuo.

topbar-mode-offset-hint-side = Questo modo lavora il segnale { $side } il dial. Il numero è il dial, perché è ciò che si sintonizza; il segnale è a { $mhz } MHz.

levelslider-hover =
    Livello del waterfall
    minimo { $floor } dB · massimo { $ceil } dB

    Trascina o scorri per schiarire o scurire; maiusc per il contrasto.
    Muoverlo disattiva l'adattamento automatico (FIT).

smeter-audio-level-hover =
    Livello audio: { $dbfs } dBFS{ $over }.

    Questa radio **non ha un S-meter proprio da riportare**, quindi questo è il livello dell'audio che invia — misurato qui, con l'AGC e lo squelch dell'apparato già applicati. Segue l'audio, non la RF: il rumore sulla scheda audio, un mouse wireless vicino all'apparato o l'AGC che pompa lo muoveranno tutti mentre il segnale non è cambiato.

smeter-audio-over = (S9 + { $over } dB sulla scala audio)

spectrum-offset-left = ◀ toni { $hz } Hz

spectrum-offset-right = { $hz } Hz toni ▶

worldmap-no-snr = nessun SNR

solar-tune-the-radio-to-this-downlink = Sintonizza la radio su questo downlink

qo100-width-label = larghezza

qo100-park-label = parcheggio

qo100-led-carrier = portante

qo100-led-sync = sync

scanner-stopped = fermato

ism-band-label = banda

amp-none = nessuno

colormap-classic = Classica

colormap-viridis = Viridis

colormap-gray = Grigio

colormap-icom = Icom

colormap-neon = Neon

colormap-synthwave = Synthwave

colormap-matrix = Matrix

colormap-tron = Tron

colormap-amber = Ambra

colormap-rainbow = Arcobaleno

colormap-blue = Blu

colormap-calmer = Calmer

colormap-compline = Compline

colormap-lauds = Lauds

settings-cb-wide-calls-hint =
    Accetta la forma più ampia di nominativo sugli 11 m { $wide } — tre lettere al centro, fino a quattro cifre dopo — accanto a { $narrow } di WSJT-CB. Sperimentale: WSJT-CB stesso rifiuta la forma più ampia, quindi un nominativo che la richiede potrebbe non essere compreso da una stazione WSJT-CB; allarga ciò che questa stazione riceve più di ciò che può inviare in modo affidabile. Disattivato mantiene l'esatta grammatica di WSJT-CB.

    Questa opzione cambia la *forma*, non la *lunghezza*: un nominativo sugli 11 m è comunque di 11 caratteri compreso qualsiasi suffisso /P, /MM o /QRP, quindi 19DCG3733/P ci sta e 19TST1001/QRP no.

settings-idle-stop-disable = DISATTIVA

settings-xvtr-tx-off-hint =
    Off durante la conversione: solo ricezione — la riga è un convertitore, non un transverter, e nulla viene manipolato su questa banda.

    Attraverso lo stesso convertitore: funziona in entrambe le direzioni, e la trasmissione usa lo stesso offset.

settings-xvtr-drive-hint =
    Un limite massimo al drive di trasmissione mentre questo transverter è selezionato, come percentuale del pieno. L'ingresso I.F. di un transverter accetta milliwatt e il drive giusto per le bande proprie della radio lo distruggerebbe, quindi questo è il campo più importante della riga. 100% significa nessun limite.

    La tua impostazione Drive viene mantenuta sotto questo valore invece di essere spostata, così il numero che usi in HF ritorna quando il dial lascia la banda.

settings-brown-build-note = La build Brown di sdroxide: il programma di upstream, più la banda cittadina degli 11 m, l'ascolto in onde corte e la decodifica. Funziona come una normale stazione radioamatoriale con queste bande e strumenti aggiunti.

settings-gain-by-band = Ricorda il guadagno del front end per banda

settings-gain-by-band-hint =
    Mantiene un guadagno del ricevitore separato per ogni banda e lo ripristina quando il dial passa in quella banda. Imposta una volta il guadagno HF e una volta quello per gli 11 m o i VHF; la radio segue la banda.

    Disattivato per impostazione predefinita. Da disattivato, un solo guadagno vale ovunque, esattamente come prima. Attivarlo registra la banda in cui ti trovi ora, così nessuna banda resta senza; una banda che non hai mai impostato mantiene il guadagno attuale.

settings-callsign-grid-note = Il tuo nominativo e locatore, condivisi tra FT8/FT4/FT2, le intestazioni delle immagini SSTV, i modi digitali CB e il logbook. È l'identità che **trasmette**: un nominativo CB se lavori sugli 11 m, altrimenti un nominativo radioamatoriale. Per ciò con cui è firmato un **rapporto di ricezione**, vedi la casella **Rapporta come (SWL)** nella scheda Spot. Modificabile anche dalla finestra di configurazione FT8 / SSTV.

settings-cb-grammar = Grammatica dei nominativi 11 m / WSJT-CB

settings-cb-wide-calls = NOMINATIVI CB ESTESI — SPERIMENTALE

settings-cb-wide-calls-note = Attivalo se la tua comunità sugli 11 m usa gruppi di tre lettere o numeri di unità a quattro cifre. Si applica alla decodifica e alla trasmissione dei modi digitali CB. In ogni caso un nominativo sugli 11 m non può superare **11 caratteri**, suffisso compreso: 19DC3733/P ci sta, 19TST1001/QRP no.

settings-cb-plan-hint = In quale piano di canali legge il dial: i piani della banda cittadina degli 11 m, e quelli PMR446 a 446 MHz. Cambiano solo i numeri di canale e il canale su cui si apre la banda: i limiti di banda restano dove sono, quindi cambiare non sposta mai ciò che riceve o trasmette. Il numero di canale è disegnato sulla linea di sintonia ovunque il dial si trovi su uno dei canali del piano.

settings-cb-allow-tx = Consenti la trasmissione sugli 11 m (CB)

settings-cb-allow-tx-hint = Il blocco delle bande radioamatoriali rifiuta la trasmissione su ogni banda che non sia un'assegnazione radioamatoriale, e gli 11 m lo sono: è un servizio radio separato con regole proprie e apparati omologati propri. Attivarlo consente la trasmissione lì — e solo lì; le bande di radiodiffusione restano solo ricezione. Ti verrà chiesto di confermarlo la prima volta.

settings-cb-licence-note = La CB è libera da licenza in molti paesi, ma non in tutti, e i canali, i modi e la potenza consentiti differiscono. L'uso della banda è soggetto alle norme del paese in cui ti trovi.

settings-radio-callsign-hint =
    Il nominativo con cui questa radio trasmette, registra e fa spot. Lascialo vuoto per usare il nominativo della stazione nella scheda Generale: il predefinito, e giusto per una stazione con una sola identità.

    Impostalo su una radio che è di qualcun altro: un nominativo CB sull'apparato per gli 11 m, un nominativo radioamatoriale sul rig HF, ciascuno mentre l'altro mantiene il proprio. Il nominativo della scheda Generale resta il ripiego per ogni radio che non lo sostituisce.

    I *rapporti* di ricezione possono comunque essere firmati con il numero SWL della scheda Spot, che questo non cambia.

    Ha effetto con Applica.

settings-radio-swap-iq-own = Questa interfaccia ha una propria inversione I/Q nella sua sezione più in basso in questa pagina: usa quella.

settings-radio-swap-iq-hint =
    Inverte I/Q di questo ricevitore prima della demodulazione. Usalo quando I e Q di un ricevitore sono invertiti e l'SSB esce sulla banda laterale sbagliata (LSB ricevuta come USB), con lo spettro speculare rispetto al dial.

    Una proprietà dell'hardware, non di chi ha effettuato l'accesso.

    Ha effetto con Applica.

settings-radio-transmit-controls = Controlli di trasmissione

settings-radio-hide-tx-all = nascondili tutti (modo SWL)

settings-radio-hide-tx-hint =
    Nasconde ogni controllo di trasmissione per *questa* radio — il PTT, CALL CQ, livello TX, INVIA, BEACON, tutto — e sostituisce gli extra radioamatoriali della barra (spot, diplomi) con quelli dell'ascoltatore (PALINSESTO e LOG SWL, il registro di ascolto). Per radio: una chiavetta d'ascolto sul filo lungo può stare in questo modo mentre il ricetrasmettitore accanto mantiene il suo trasmettitore.

    L'hardware può ancora trasmettere; questo nasconde solo i controlli. `--swl` lo impone per ogni radio per la durata dell'esecuzione.

    Ha effetto con Applica.

settings-radio-auto-mode = Modo auto

settings-radio-auto-stop-label = ferma dopo nessun intervento

settings-radio-auto-stop-hint-long =
    Per quanto tempo il sequenziatore FT8/FT4/FT2 non presidiato può funzionare senza intervento dell'operatore prima di disarmarsi. Per radio: una può essere lasciata attiva più a lungo di un'altra.

    Su un PlutoSDR arriva fino a sei ore, e DISATTIVA spegne del tutto l'arresto: il modo auto funziona allora finché non lo fermi. Il watchdog di trasmissione e il blocco di banda si applicano comunque.

    Ha effetto con Applica.

settings-radio-auto-stop-hint =
    Per quanto tempo il sequenziatore FT8/FT4/FT2 non presidiato può funzionare senza intervento dell'operatore prima di disarmarsi. Per radio: una può essere lasciata attiva più a lungo di un'altra.

    Limitato a 45 minuti di proposito: il modo auto serve per una pausa in bagno, non per lasciare una stazione a fare un contest senza nessuno dietro.

    Ha effetto con Applica.

settings-radio-smeter-calib = Calib. S-Meter

settings-tx-eq = EQ di trasmissione

settings-tx-eq-hint = Un EQ parametrico a 3 bande sull'audio del microfono, prima del modulatore. Solo modi fonia (SSB/AM/FM); i modi digitali e il CW portano audio sintetizzato o manipolato che non lo raggiunge mai. Disattivato per impostazione predefinita e piatto quando attivato, quindi attivarlo non cambia nulla in aria finché una banda qui sotto non viene effettivamente mossa. Si applica immediatamente.

settings-tx-eq-low-hint = Taglia/enfatizza rombo e rumori di manipolazione.

settings-tx-eq-mid = Picco medio

settings-tx-eq-mid-hint = Presenza: un picco o una valle stretti da qualche parte nella banda vocale.

settings-tx-eq-high-hint = Brillantezza/de-ess.

settings-cb-call-too-long = ⚠ Troppo lungo per gli 11 m — { $len } caratteri, e FT8 ne porta 11 per un nominativo CB *compreso* qualsiasi suffisso /P, /MM o /QRP. Questo nominativo di attivazione non può essere inviato. 19DC3733/P (10) ci sta; 19TST1001/QRP (13) no.

settings-cb-plan-note = Gli 11 m (CB) non sono la stessa banda ovunque. Questo sceglie i canali di quale paese legge il dial degli 11 m: { $plan }. Solo i canali e il canale su cui si apre la banda: i limiti della banda sono lasciati larghi, quindi cambiare piano non cambia mai ciò che riceve o trasmette. Si applica a ogni radio di questa stazione.

settings-cw-enter-mode = Entra in CW (o in qualsiasi modo digitale) una volta per caricare i valori salvati.

settings-cw-where-from = Da dove:

settings-cw-straight-key-hint = L'associazione `CW straight key` (barra spaziatrice per impostazione predefinita) viene letta mentre il KEY del pannello CW è attivo. Un solo contatto, quindi è un tasto verticale: l'app decodifica la cadenza dell'operatore stesso.

settings-cw-speed = Velocità:

settings-cw-speed-note = la lunghezza di punti e linee del keyer

settings-cw-key-tx = MANIPOLA IL TRASMETTITORE

settings-cw-key-tx-hint = Off: il tasto pilota solo il trainer locale. On: manipola la radio attraverso il normale percorso di manipolazione manuale, con il blocco di banda e il watchdog attorno.

settings-cw-key-self-note = Per un apparato che si manipola da sé, imposta Impostazioni → Radio → Manipolazione CW su `Scheda audio (MCW)`: la manipolazione manuale è rifiutata per scelta sul percorso del keyer dell'apparato.

settings-detachable-windows = Finestre staccabili

settings-detachable-windows-note = Estrai un pannello in una finestra propria del sistema operativo. Chiudere la finestra lo riaccorpa.

settings-tool-windows = Finestre strumento

settings-tool-windows-note = Pannelli convertiti in finestre flottanti. Il loro stato di apertura viene ricordato.

settings-tool-window-scanner = SCANNER

settings-tool-window-memories = MEMORIE

settings-tool-window-playback = RIPRODUZIONE

settings-tool-window-open = Aperta

settings-tool-window-closed = Chiusa

settings-detachable-native-only = Le finestre staccabili sono solo native.

settings-psk-reporter-band-hint = Il feed di attività della banda: stazioni ricevute sulla banda attuale, disegnate come punti colorati. Abilita anche la richiesta "chi mi ha sentito" dietro il chip MI HANNO SENTITO della mappa — riscaricata anche ogni ora.

settings-psk-upload-hint = Riporta ciò che questa stazione riceve a pskreporter.info, così vi appare come ricevitore. Usa il nominativo e il locatore della scheda Generale, o il numero SWL qui sotto quando ne è impostato uno.

settings-swl-report-as = Rapporta come (SWL)

settings-swl-report-as-hint = L'identità propria dell'ascoltatore per i rapporti di ricezione: un numero SWL registrato, un numero di club, un nome. Quando è impostata firma ciò che questa stazione *riceve* (i caricamenti su PSK Reporter e WSPRnet, e il rapporto di ricezione che il log SWL copia) al posto del nominativo. Lasciala vuota per rapportare con il nominativo. Non viene mai trasmessa, non manipola mai una radio e non nomina mai un QSO o uno spotter, così anche un ascoltatore senza nominativo può rapportare.

settings-wsjtcb-spot-server = Server spot WSJT-CB

settings-wsjtcb-report = Riporta le mie decodifiche sugli 11 m al server spot WSJT-CB

settings-wsjtcb-report-hint = Invia ogni spot decodificato sulla banda degli 11 m al servizio di spotting della comunità CB (xzgroup.net): lo stesso server a cui può inviare il client di WSJT-CB. Disattivato per impostazione predefinita: è un servizio di terzi e nulla viene inviato finché non lo chiedi. Il tuo nominativo è lo spotter; PSK Reporter è l'interruttore separato qui sopra.

settings-eqsl-qth-hint = In quale profilo QTH viene archiviato un caricamento: serve solo quando l'account ne possiede più d'uno, e resta vuoto altrimenti.

settings-log11dx-api-url = URL API

settings-log11dx-api-token = Token API

settings-log11dx-note = Entrambi provengono dalla pagina del tuo profilo LOG11DX. Il suo bridge WSJT-X non serve: sdroxide registra il QSO e lo invia direttamente lì. Il sito maschera il token, quindi copialo quando lo generi: è memorizzato in net.json come le altre credenziali.

settings-pan-attached-note =
    Il suo spettro, il suo waterfall e tutto ciò che si ricava da essi sono nella scheda di quella radio, ed ecco perché non ne ha di propri. Le impostazioni dell'interfaccia qui sopra sono ancora di questa radio e si applicano ancora: premi Applica / riconnetti dopo averle cambiate e le due radio ripartono insieme.

    Per usarla di nuovo da sola, svuota la casella Ricevitore panadapter nella pagina di quella radio.

settings-pan-source-hint =
    Quale radio riceve per questa. Lascia la barra delle schede mentre è prestata — il suo front end appartiene al motore di questa radio — e ritorna subito quando questo è impostato su Nessuno.

    Ha effetto con Applica.

settings-pan-offset-hint =
    Di quanto sopra il dial di questa radio si trova il ricevitore, in hertz. 0 per un ricevitore sulla stessa antenna; la frequenza intermedia per una presa I.F. — 9000000 per una I.F. a 9 MHz, 70455000 per una a 70,455 MHz.

    Non è il campo Convertitore qui sopra, che risintonizza la radio, e non è l'offset del centro I/Q della scheda CAT, che riguarda la scheda audio di questa radio. Nulla di ciò che digiti qui viene mai inviato a nessuna delle due radio.

    Ha effetto con Applica.

settings-cw-device = Dispositivo:

settings-cw-device-none = (nessuno trovato)

settings-cw-device-auto = auto (primo chiamato "key")

settings-cw-reverse-hint = Scambia punti e linee, per l'interruttore sul paddle stesso.

settings-cw-key-type = Tipo di tasto:

settings-cw-key-straight = VERTICALE

settings-cw-key-iambic-a = IAMBIC A

settings-cw-key-iambic-b = IAMBIC B

settings-cw-usb-note = Funziona un box keyer che riporta i suoi due contatti (spesso come pulsanti di un mouse); uno che fa da sé l'iambic e invia tocchi già elaborati no. Mentre il tasto è armato il dispositivo viene preso in modo esclusivo, così i suoi contatti non possono fare clic in altre finestre.

settings-cw-usb-linux = Un paddle USB richiede un desktop Linux.

settings-smeter-zero = 0 (come misurato)

settings-smeter-cal-hint =
    Corregge l'S-meter di questa radio quando legge troppo alto o troppo basso, in unità S da 6 dB.

    Un valore negativo abbassa la lettura — per uno strumento che mostra troppo, un fruscio di fondo a S5 dove dovrebbe essere S2. Un valore positivo la alza. 0 è lo strumento come misurato.

    Si sposta solo la lettura: l'ago, le cifre S e dBm, il livello da cui parte un rapporto di ricezione e l'S-meter parlato. Il segnale, lo squelch, la registrazione e i decodificatori non vengono toccati.

    Salvato con questa radio sulla macchina a cui è collegata, e ha effetto subito.

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1 e 2 usano entrambe { $host }:{ $port }: ogni spot arriverà due volte. Dai loro porte diverse (7000 CW/RTTY, 7001 FT8/FT4) o disattivane una.

settings-rx-site-station = Alla stazione

settings-rx-site-elsewhere = Altrove

settings-remote-tab-note =
    La stazione appare come una scheda accanto alle radio già aperte qui, e la barra delle schede in alto nella finestra passa dall'una all'altra (⊞ ne affianca due). Chiudila dall'elenco in cima alla scheda Radio: quello riaggancia; nulla sul server viene cambiato.

    Se il server chiede nome utente e password, la sua schermata di accesso compare nella nuova scheda. Un semplice link ws:// non porta nulla di tutto questo cifrato, quindi attraverso la rete internet aperta o lo fai passare da una VPN o da un tunnel SSH, oppure termina l'HTTPS davanti al server e spunta Sicuro qui sopra.

settings-cw-source-keyboard = TASTIERA

settings-cw-source-usb = PADDLE USB

pane-frame = FRAME

pane-frames = FRAME

pane-words = PAROLE

antenna-feed-z0 = Z0 alimentazione:

antenna-feed-z0-hint = Impedenza della linea/del sistema per il ROS (50 Ω tipico)

antenna-height-label = Altezza:

antenna-height-hint = Altezza del punto di alimentazione/dell'antenna dal suolo (influisce su angolo di partenza/diagramma)

antenna-m-agl = m dal suolo

antenna-apply = Applica

antenna-apply-hint = Esegue il solver a metodo dei momenti sui preset.

antenna-yagi-optimizer = Ottimizzatore Yagi

antenna-stack = Stack:

antenna-objective = Obiettivo:

antenna-optimize = Ottimizza

antenna-swr-value = ROS { $v }

antenna-swr-ref = ROS { $v } ({ $z }Ω)

antenna-radiation-pattern = Diagramma di radiazione

antenna-view = Vista

antenna-view-front = Fronte

antenna-view-side = Lato

antenna-view-top = Alto

antenna-3d-hint = 3D trascina per ruotare • calore = intensità di campo

antenna-yagi-x = Yagi ×{ $n }

settings-tool-window-session-qsos = QSO DI SESSIONE

sessionqso-time = Ora

sessionqso-call = Nominativo

sessionqso-grid = Locatore

sessionqso-band = Banda

sessionqso-mode = Modo

sessionqso-rst-s = RST T

sessionqso-rst-r = RST R

sessionqso-country = Paese

sessionqso-dist = Dist

sessionqso-final = Finale

sessionqso-count = { $n } QSO in questa sessione

sessionqso-include-cw = Includi CW

decodes-live-log = LOG LIVE

decodes-setup = ⚙ CONFIGURA

schedule-colour-all = Colore

schedule-colour-all-hint = Colore dell'etichetta nel waterfall per ogni stazione — sostituisce quelli scelti per singola stazione

schedule-colour-station-hint = Colore dell'etichetta nel waterfall di questa stazione

schedule-colour-reset = PREDEFINITO

schedule-colour-reset-hint = Torna al colore di serie per ogni stazione, dimenticando quelli scelti

settings-language = Lingua

settings-language-note = La lingua in cui viene mostrata l'interfaccia. Predefinita di sistema segue il sistema operativo; scegline un'altra per sostituirla. La scelta viene salvata e ricordata.

aprs-all-chip = TUTTI

aprs-all-chip-hint = Acceso: ogni messaggio ascoltato sul canale, a chiunque sia destinato. Spento: solo quelli da o per te, e i bollettini.

aprs-composer-hidden-swl = L'invio è nascosto: la modalità SWL / ascolto è attiva. Disattivala per scrivere e inviare un messaggio.

aprs-compose-count-hint = Caratteri usati sui 67 che un messaggio APRS può contenere.

tx-bands-title = Bande di trasmissione

tx-bands-note = Imposta TX su SÌ solo per le bande su cui questa radio trasmette davvero. Questa impostazione ha la precedenza su ogni altra impostazione di trasmissione; SÌ non elimina mai gli altri limiti di sicurezza.

tx-bands-careful = ⚠ Una banda sbagliata può distruggere la radio: controlla con attenzione ogni opzione.

tx-bands-yes = SÌ

tx-bands-no = NO

tx-bands-confirmed = ✓ Bande di trasmissione confermate per questa radio

tx-bands-ack = Ho verificato queste bande rispetto alla mia radio

tx-bands-confirm = Conferma queste bande

tx-bands-confirm-hint = Spunta prima la casella: si conferma una sola volta per ogni radio.

tx-bands-check = Verifica bande…

tx-bands-notice-new = La trasmissione è disattivata su ogni banda. Scegli le bande su cui questa radio trasmette e confermale una volta. Per il rischio di danni alla radio, controlla con attenzione le opzioni scelte.

tx-bands-notice-carried = Le tue bande di trasmissione sono state riportate dalla versione precedente. La trasmissione resta bloccata finché non le verifichi e le confermi una volta. Per il rischio di danni alla radio, controlla con attenzione ogni opzione.


adsbmap-surface-asphalt = asfalto

adsbmap-surface-concrete = calcestruzzo

adsbmap-surface-grass = erba

adsbmap-surface-gravel = ghiaia

adsbmap-surface-dirt = terra

adsbmap-surface-water = acqua

adsbmap-airport-runway-lit = illuminata

adsbmap-airport-more = +{ $n } altre

adsbmap-airport-freq-head = Frequenze, MHz

adsbmap-airport-data-date = dati di piste e frequenze del { $date }
aircraft-online-switch = Online

aircraft-online-switch-on-hover = Online: tipo, operatore, rotta e foto provengono da adsbdb e Planespotters, le immagini degli aeroporti da Wikipedia, e l'ora locale e la temperatura di un aeroporto da Open-Meteo (viene inviata solo la posizione dell'aeroporto), richieste dalla tua stazione per conto tuo. Clic per disattivare; la stazione ricorda la tua scelta. Nient'altro esce dalla stazione.

aircraft-online-switch-off-hover = Offline: nulla esce dalla stazione. Clic per attivare tipo, operatore, rotta e foto (adsbdb e Planespotters), immagini degli aeroporti (Wikipedia) e meteo degli aeroporti (Open-Meteo).

aircraft-switch-held = l'operatore della stazione lo gestisce con SDROXIDE_AIRCRAFT_ONLINE

adsbmap-airport-photo-credit = Foto: Wikipedia

adsbmap-airport-local-time = Ora locale { $time } ({ $zone })

adsbmap-airport-temperature = Temperatura { $temp }

adsbmap-airport-weather-credit = Meteo: Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time } (ora locale: caricamento…)

adsbmap-airport-utc-offline = UTC { $time } (ora locale: offline)

schedule-listen = ASCOLTA

schedule-listen-hint = Sintonizza il ricevitore su questa stazione

schedule-sort = Ordina

schedule-sort-asc = Crescente

schedule-sort-desc = Decrescente

schedule-col-freq = Freq.

schedule-col-band = Banda

schedule-col-name = Nome

schedule-col-lang = Lingua

schedule-col-target = Zona

schedule-col-site = Sito

schedule-col-local = Locale

tx-bands-off = La trasmissione è disattivata su { $band }: questa banda non è consentita nelle tue impostazioni, quindi il tuo trasmettitore non è compatibile con la trasmissione su questa banda. (Mostrato al massimo { $max } volte per sessione.)

tx-off-band = TRASMISSIONE VIETATA — { $freq } MHz ({ $band }) non è una banda di trasmissione convalidata. Spuntala e convalidala in Impostazioni → Radio → Bande di trasmissione.

tx-off-unconfirmed = TRASMISSIONE VIETATA — le bande di trasmissione di questa radio non sono ancora convalidate. Controllale e convalidale in Impostazioni → Radio → Bande di trasmissione.

aircraft-hover-lookup-retry = nessuna risposta — nuovo tentativo tra { $s } s

adsbmap-airport-navaids-head = Radioassistenze sul campo

adsbmap-airport-no-freq = Nessuna frequenza pubblicata per questo aeroporto

topbar-preset-right-click = Clic destro per modificare questo preset

topbar-preset-edit = MODIFICA PRESET

topbar-preset-label = Etichetta

topbar-preset-label-hint = Il testo del chip. Fino a 8 caratteri.

topbar-preset-lo = Basso

topbar-preset-hi = Alto

topbar-preset-use-current = Usa attuale

topbar-preset-use-current-hint = Prendi i bordi del filtro attualmente impostato sul ricevitore

topbar-preset-save = Salva

topbar-preset-reset = Ripristina

topbar-preset-reset-hint = Ripristina il preset integrato di questa modalità

topbar-preset-cancel = Annulla
wefax-schedule = ORARIO
wefax-schedule-hint = Apre nel browser gli orari pubblicati da { $station } — carte e orari UTC.

tx-bands-check-again = RICONTROLLA

tx-bands-check-again-hint = Ritira la conferma per ricontrollare la lista. Cambiare una banda lo fa già da sé.
