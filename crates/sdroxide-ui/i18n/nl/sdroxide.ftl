# sdroxide — Nederlandse vertaling van de interface. De referentie is
# en-US/sdroxide.ftl; wat hier ontbreekt, wordt in het Engels getoond.

## The language's own name, as the language picker lists it.
language-name = Nederlands

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = Auto
layout-mode-desktop = Desktop
layout-mode-tablet = Tablet
layout-mode-small = Klein scherm
layout-mode-phone = Telefoon

ui-theme-default = Standaard
ui-theme-light = Licht
ui-theme-high-contrast = Hoog contrast
ui-theme-green-phosphor = Groen fosfor
ui-theme-amber-phosphor = Amber fosfor
ui-theme-teal-orange = Blauwgroen / oranje
ui-theme-rainbow = Regenboog
ui-theme-nord = Nord
ui-theme-nord-dark = Nord donker
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized donker
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin mocha
ui-theme-catppuccin-latte = Catppuccin latte
ui-theme-modern-minimalist = Modern minimalistisch

chrome-style-angled = Hoekig
chrome-style-rectangular = Rechthoekig
chrome-style-rounded = Afgerond
chrome-style-gradient = Kleurverloop
chrome-style-bevel = 3D-afschuining
chrome-style-terminal = Terminal

font-size-small = Klein
font-size-medium = Gemiddeld
font-size-large = Groot

bandplan-kind-ham = Zendamateur
bandplan-kind-cw = CW
bandplan-kind-digi = Digitaal
bandplan-kind-phone = Spraak
bandplan-kind-beacon = Baken
bandplan-kind-broadcast = Omroep
bandplan-kind-am = AM / LW / MW
bandplan-kind-cb = CB

speech-verbosity-terse = Beknopt
speech-verbosity-normal = Normaal
speech-verbosity-full = Volledig

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = Afstemming (14 point 074)
speech-freq-style-digits = Elk cijfer
speech-freq-style-full = Afstemming met eenheden

speech-callsign-style-phonetic = Fonetisch (kilo one alpha)
speech-callsign-style-letters = Letters (kay one ay)
speech-callsign-style-as-is = Zoals geschreven

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = Weergave
settings-ui-moved-note = Panadapter-detail, de reactie van het spectrum en de scrollsnelheid van de waterval stel je in met de SPEC-knop op de weergavebalk — het DISP-menu in een smal venster.
settings-ui-language = Taal
    .hint = De taal van de interface. De handleiding achter F1 en de spraakmeldingen zijn in het Engels, wat je hier ook kiest.
settings-ui-layout = Indeling
settings-ui-theme = Thema
settings-ui-button-style = Knopstijl
settings-ui-frame-rate = Schermverversingssnelheid
settings-ui-palette = Watervalpalet
settings-ui-tuning-buttons = Afstemknoppen
settings-ui-tuning-buttons-show = Tonen op telefoon en tablet
    .hint = Een rij knoppen stap omlaag / stap / stap omhoog onder de bedieningsbalk op een aanraakscherm. Er is geen wiel om een cijfer mee te scrollen en geen afstemknop, dus zonder deze knoppen kun je een bekende stap alleen maken door de hele frequentie in te typen. Wordt nooit getoond op een desktop.
settings-ui-tuning-step = Stap: { $step }
settings-ui-tuning-snap = Eerste druk springt naar de stap
    .hint = Staat de afstemming tussen twee veelvouden van de stap, dan gaat hij naar het volgende veelvoud in de ingedrukte richting — 7 074 300 gaat omhoog naar 7 075 000 bij een stap van 1 kHz, omlaag naar 7 074 000. Daarna verplaatsen de knoppen zoals gewoonlijk met de stap. Uit: elke druk verplaatst precies de stap.
settings-ui-smoothing = Watervalafvlakking
settings-ui-smoothing-interpolate = Interpoleren
    .hint = Meng elke schermpixel met de bins en rijen eromheen, zodat een signaal doorlopend lijkt waar het scherm breder is dan de transformatie. Zet uit voor een rechthoekige waterval — één blok per bin, één per rij — wat nodig is om de vorm van een signaal van het beeld af te lezen: een geïnterpoleerd signaal is niet te onderscheiden van een werkelijk breder signaal. Een grotere FFT (de FFT-chip) is de andere helft daarvan.
settings-ui-background = Spectrumachtergrond
settings-ui-background-gradient = Kleurverloop
settings-ui-background-top = boven
settings-ui-background-bottom = onder
settings-ui-spot-colours = Kleuren van spotlabels
    .hint = De tint van elke spotbron: de vakken onderaan de waterval, de badges in de SPOTS-lijst en de stippen op de wereldkaart.
settings-ui-spot-colour = Kleur voor { $kind }-spots
settings-ui-spot-colours-reset = Zet elke spotkleur terug naar de standaard
settings-ui-reset = Herstellen
settings-ui-bandplan-colours = Kleuren van het bandplan
    .hint = De tint waarin elke klasse van toewijzing wordt geschilderd op de bandplanstrook onderaan de waterval. De blokken worden half doorzichtig over de waterval getekend, dus ze komen daar donkerder uit dan in het staal hier.
settings-ui-bandplan-colour = Kleur voor { $kind }-toewijzingen
settings-ui-bandplan-colours-reset = Zet elke bandplankleur terug naar de standaard
settings-ui-skimmer-font = Lettergrootte skimmer
settings-ui-waterfall-font = Lettergrootte waterval / spectrum
settings-ui-interface-font = Lettergrootte interface
    .hint = Schaalt de hele interface — menu's, dialogen, vensters, de radiotabs, de bovenbalk en de knoppen daarin. De twee groottes hierboven zijn daaraan relatief.
settings-ui-cities = Steden op kaarten
    .hint =
        Teken de steden van de wereld — een stip per plaats, met de naam ernaast waar ruimte is — op de platte kaarten: FT8/WSPR, APRS, ADS-B en AIS.

        Ze zijn het meeste van wat aangeeft *waar* een stip ligt op een kaart van de hele wereld, en ook het drukste wat erop staat. Zet ze uit als de namen de stations die je leest in de weg zitten. De 3D-wereldbol blijft ongewijzigd — zijn steden zijn lichtjes aan de nachtzijde in plaats van markeringen, en er wordt niets over een contact heen geschreven.
settings-ui-cities-show = toon steden en hun namen
settings-ui-3d-view = 3D-weergave
settings-ui-clouds = Wolkenweergave
settings-ui-clouds-layered = In lagen
settings-ui-clouds-volumetric = Volumetrisch
settings-ui-clouds-note = Hoe de CLOUDS-laag in de 3D-weergave het weer tekent. In lagen stapelt plakken door de troposfeer en is de goedkope optie. Volumetrisch laat in plaats daarvan een straal erdoorheen lopen, zodat de zon de wolkentoppen op het dek eronder laat schaduwen en bliksem door de onweerswolk heen gloeit die hem veroorzaakt, in plaats van alleen de buitenkant op te lichten — tegen een meervoud van de kosten per pixel.

## Settings window, UI tab: voice announcements.

speech-title = Spraakmeldingen
speech-enabled = Wijzigingen aan de radio uitspreken
    .hint = Leest voor wat er veranderd is, zodat de radio bediend kan worden zonder hem te zien
speech-voice = Stem
speech-voice-shipped = Meegeleverde stem
speech-speed = Snelheid
    .hint = De stem rekt of comprimeert zijn eigen frasering, zodat de toonhoogte niet verandert. Boven ongeveer 2× wordt het niet korter meer.
speech-volume = Volume
speech-output = Uitvoer
speech-detail = Detail
speech-duck = Ontvanger dempen
speech-duck-while = Tijdens het spreken
speech-test = Test
speech-what = Wat melden
speech-cat-frequency = Frequentie
speech-cat-mode-band = Modus en band
speech-cat-vfo-split = VFO en split
speech-cat-agc-gain = AGC en versterking
speech-cat-levels = Drive, tune en microfoon
speech-cat-ptt = Zenden en ontvangen
speech-cat-rit-xit = RIT en XIT
speech-cat-memory-scan = Geheugens en scannen
speech-cat-band-edge = Een amateurband verlaten
speech-cat-notices = Waarschuwingen en berichten
speech-cat-filters = Filters, squelch en ruisonderdrukking
    .hint = Standaard uit: deze veranderen constant tijdens het zoeken naar een signaal
speech-decoded = Gedecodeerde berichten
speech-ft8-to-me = FT8-oproepen aan mij
speech-ft8-cq = FT8-CQ's die ik kan beantwoorden
    .hint = Een drukke avond op twintig meter is er honderd per minuut
speech-js8 = JS8-berichten aan mij
speech-js8-allcall = Ook JS8 @ALLCALL
speech-fsq = FSQ-berichten aan mij
speech-include-snr = Rapport meenemen
    .hint = Alleen waar het bericht er zelf geen heeft — een decodering die al een getal meldt krijgt het onze er niet bij
speech-ft8-qso = Mijn eigen FT8-uitwisseling
    .hint = Wat de sequencer staat te verzenden, en het einde van het contact
speech-reading-text = Gedecodeerde tekst hardop voorlezen
speech-text-cw = CW
speech-text-rtty-psk = RTTY, PSK, Olivia, THOR, FSQ
speech-text-cw-locked = CW alleen als de decoder vergrendeld is
    .hint = De uitvoer van een niet-vergrendelde decoder voorlezen is slechter dan stilte
speech-text-note = Beide staan standaard uit. Een decoder produceert tekst sneller dan spraak die kan voorlezen, dus alles wat te ver achterloopt op de live audio wordt weggelaten in plaats van in de wachtrij gezet.
speech-tuning-up = Afstemmen
speech-swr-while-tuning = Lees de SWR voor zolang TUNE ingedrukt is
speech-swr-every = Elke
speech-swr-summary = Meld de beste aanpassing bij loslaten
speech-swr-alarm = Waarschuw voor hoge SWR tijdens elke uitzending
speech-how-read = Hoe dingen worden voorgelezen
speech-frequencies = Frequenties
speech-callsigns = Roepletters
speech-quiet-on-tx = Stil blijven tijdens zenden
    .hint = Spraak gaat naar je luidsprekers en dus ook in je microfoon. Waarschuwingen voor hoge SWR komen nog wel door.
speech-device-note = Meldingen worden afgespeeld op hun eigen geluidsapparaat, dus ze worden nooit opgenomen en nooit verstuurd naar iemand die op afstand meeluistert. Toetsen om de status uit te spreken, de laatste melding te herhalen en midden in een zin te stoppen staan op het tabblad Bediening.

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = Wachten op de netwerkconfiguratie van het station…
settings-device-reports = zoals het apparaat meldt
settings-transverters = Transverters
settings-one-row-per-box-while = Eén rij per kast. Zolang de afstemming binnen de band van een rij ligt, wordt de radio afgestemd op afstemming + offset en gelden de zendregel en de drivelimiet van die rij; buiten elke rij werkt de radio op zijn eigen banden. Rijen worden van boven naar beneden geprobeerd.
settings-name = Naam
settings-band-low = Band laag
settings-band-high = Band hoog
settings-offset = Offset
settings-transmit = Zenden
settings-max-drive = Max. drive
settings-off-takes-transverter-out-line = Uit haalt deze transverter uit de keten zonder de rij te verliezen — de band gaat terug naar wat de radio zelf bereikt.
settings-t-2-m = 2 m
settings-what-you-call-shown-log = Hoe je hem noemt. Wordt in het logboek getoond als de afstemming hem selecteert.
settings-bottom-band-transverter-works-dial = De onderkant van de band waarop deze transverter werkt, op de afstemming. 144 voor 2 m.
settings-top-band-dial-148-2 = De bovenkant van de band, op de afstemming. 148 voor 2 m in regio 2 en 3, 146 in regio 1.
settings-radio-ends-up-dial-offset = De radio komt uit op afstemming + offset, dus een transverter die een band omlaag brengt naar een I.F. is negatief: 2 m naar een 28 MHz I.F. is -116, want 144 - 116 = 28. Sleep om hem hertz voor hertz bij te stellen voor een oscillator die een beetje afwijkt.

    Via dezelfde converter: het werkt in beide richtingen, en zenden gebruikt dezelfde offset.

    Je Drive-instelling wordt hieronder gehouden in plaats van verschoven, zodat het getal dat je op HF gebruikt terugkomt als de afstemming de band verlaat.
settings-remove = VERWIJDEREN
settings-add-transverter = TRANSVERTER TOEVOEGEN
settings-takes-effect-apply-reconnect-like = Wordt actief bij Toepassen / opnieuw verbinden, net als de converteroffset hierboven.
settings-maximum-transmit-drive = Maximale zenddrive
settings-hard-limit-drive-tune-controls = Een harde limiet waar de bedieningen Drive en TUNE niet voorbij kunnen, toegepast na de bandkalibratie hieronder zodat niets de drive er weer overheen kan tillen. Stel hem in waar de radio zijn nominale vermogen haalt en de hele Drive-regeling wordt bruikbaar — bij een zender die zijn eigen samples moduleert (HPSDR, LimeSDR, PlutoSDR, HackRF) is volle drive de zender wagenwijd open, wat bij een versterker met hoge winst ruim boven de nominale waarde van de eindtrap ligt. Uit betekent dat de bedieningen volle drive bereiken.
settings-limit-drive = Drive beperken tot
settings-per-cent-full-drive-key = Procent van volle drive. Zend de radio in op een kunstbelasting en breng dit omlaag tot de meter het vermogen toont waarvoor de versterker is gespecificeerd.
settings-applies-immediately-every-band-tune = Geldt direct, op elke band en ook voor TUNE.
settings-transmit-drive-band = Zenddrive per band
settings-every-amplifier-has-different-gain = Elke versterker heeft op elke band een andere winst, dus één Drive-instelling geeft op elke band een ander vermogen. Meet de uitvoer op elke band en stel dan de banden die te hoog uitkomen bij tot ze allemaal gelijk zijn: stel Drive in voor de band die het meeste nodig heeft (meestal de hoogste) en breng de andere daarnaar omlaag. Decibels uitgangsvermogen, toegepast op spraak, digitaal en TUNE alike. Overal nul — de standaard — verandert niets.
settings-other = Overig
settings-clear-all = ALLES WISSEN
settings-back-no-calibration-drive-setting = Terug naar geen kalibratie: de Drive-instelling bereikt elke band volledig.
settings-applies-immediately-band-you-would = Geldt direct, en voor de band waarop je zou zenden — achter een transverter is dat de band op de afstemming, en de eigen drivelimiet van de transverter wint nog steeds van alles wat hier is ingesteld.
settings-settings = Instellingen
settings-enter-server-s-address-first = voer eerst het adres van de server in
settings-enter-receiver-s-address-first = voer eerst het adres van de ontvanger in, of kies er een uit Openbare SDR's
settings-no-radio-selected-press-discover = geen radio geselecteerd — druk op Zoeken, of voer een adres in
settings-enter-radio-s-address-first = voer eerst het adres van de radio in
settings-general = Algemeen
settings-radio = Radio
settings-ui = UI
settings-alerts = Meldingen
settings-controls = Bediening
settings-spots = Spots
settings-freedv = FreeDV
settings-uploads = Uploads
settings-winlink = Winlink
settings-servers = Servers
settings-t-r-switch = T/R-schakelaar
settings-tle = TLE
settings-profiles = Profielen
settings-station = Station
settings-enter-digital-mode-ft8-sstv = Ga één keer een digitale modus in (FT8 / SSTV / …) om de opgeslagen waarden te laden.
settings-callsign = Roepletters
settings-grid-square = Locator
settings-iaru-region = IARU-regio
settings-which-radio-interface-server-uses = Welke radio-interface de server gebruikt, wordt ingesteld op de machine waarop hij draait.
settings-radio-interface = Radio-interface
settings-waiting-machine-radio-attached-interface = Wachten op de machine waaraan de radio hangt. De interface-instellingen staan hieronder en kunnen intussen van hieruit worden gewijzigd.
settings-converter = Converter
settings-manual-type-offset-below = Handmatig — typ hieronder een offset
settings-what-box-reads-when-offset = Wat dit vak toont als de offset geen van de voorinstellingen is. Stel hem in door in het veld Offset hieronder te typen; hier valt niets te selecteren.
settings-frequency-converter-front-receiver-pick = Een frequentieconverter voor de ontvanger. Kies er een, of typ ernaast een offset.
settings-how-far-converter-moves-signal =
    Hoe ver een converter het signaal verschuift op weg naar de ontvanger, in Hz — hetzelfde getal en teken als in de documentatie van elke converter en elk ander SDR-programma. De radio komt uit op afstemming + offset: positief voor een upconverter (een Ham It Up is 125000000), negatief voor alles wat een band omlaag brengt naar een I.F. — een satelliet-LNB, of een 2 m-transverter op een 28 MHz I.F., die -116000000 is (28 − 144). 0 = geen converter.

    Sleep om hem hertz voor hertz bij te stellen, wat een converter met een licht afwijkende oscillator nodig heeft.

    Dit is het ontvangstpad. Wat in de zendlijn zit, is de rij hieronder.

    Wordt actief bij Toepassen.
settings-what-transmit-line-while-converter =
    Wat in de zendlijn zit terwijl een converter is ingesteld.

    Uit tijdens converteren: er wordt niets uitgezonden — de veilige standaard, en goed voor een accessoire alleen voor ontvangst.

    Via dezelfde converter: één kast converteert in beide richtingen, een transverter. Zenden gebruikt de offset hierboven en volgt die als hij wordt bijgesteld.

    Eigen offset: de zendlijn verschilt van de ontvangstlijn. Nul is het gangbare geval — een converter (een LNB, een Ham It Up) bij ontvangst met de zender op zijn eigen antenne, wat het QO-100-station is.

    De controle op amateurbanden geldt nog steeds, op de frequentie waarop je zendt.

    Wordt actief bij Toepassen.
settings-how-far-transmit-line-moves =
    Hoe ver de zendlijn het signaal verschuift, in Hz, volgens dezelfde tekenregel als de ontvangstoffset: de radio zendt op afstemming + offset.

    0 = de zender is aangesloten op zijn eigen antenne en werkt op de frequentie die de afstemming aangeeft — de 2,4 GHz-uplink van een QO-100-station.

    Een zendconverter die een I.F. *omhoog* brengt, is een negatief getal: de radio werkt onder de afstemming.
settings-rx-range = RX-bereik
settings-which-frequencies-radio-receives-mhz =
    Welke frequenties deze radio ontvangt, in MHz: 144-146, 430-440. Laat leeg om te gebruiken wat het apparaat over zichzelf meldt.

    Bandknoppen buiten het bereik zijn grijs en de afstemming gaat daar niet heen.

    Dit zijn afstemfrequenties. Met een converter ingesteld geef je de band op die je afstemt — 144-148 voor een 2 m-transverter — niet de I.F. waarop de radio werkelijk staat.

    Wordt actief bij Toepassen.
settings-tx-range = TX-bereik
settings-which-frequencies-radio-transmits-mhz =
    Op welke frequenties deze radio zendt, in MHz: 144-146, 430-440. Laat leeg om te gebruiken wat het apparaat meldt — en als het niets meldt, wordt de driver op zijn woord geloofd en is elke frequentie toegestaan.

    Afstemfrequenties, zoals het ontvangstbereik hierboven.

    Dit is een limiet die jij instelt, geen vergunning: zenden buiten de amateurbanden wordt hoe dan ook geweigerd, tenzij je dat in config.toml hebt uitgezet. Het geeft ook geen zender aan een apparaat dat alleen kan ontvangen.

    Wordt actief bij Toepassen.
settings-antenna = Antenne is
settings-where-radio-listens-station-antenna =
    Van waar deze radio luistert.

    Op het station: de antenne is de jouwe, dus je eigen locator zegt waar hij is. De standaard, en goed voor alles in de shack.

    Elders: een online ontvanger, of je eigen set op een heuveltop. Wat hij hoort wordt gerapporteerd aan PSK Reporter, WSPRnet en FreeDV Reporter vanaf de locator hiernaast, nooit vanaf de jouwe — en ADS-B plaatst er ook vliegtuigen tegenover.

    Een ontvanger kiezen onder "Openbare SDR's" vult dit voor je in.

    Wordt actief bij Toepassen.
settings-receiver-s-maidenhead-locator-jn88ec =
    De Maidenhead-locator van de ontvanger — JN88ec, DO30db. Vier of zes tekens.

    Laat leeg als je niet weet waar de ontvanger staat: er wordt dan helemaal niets gerapporteerd, wat het enige eerlijke antwoord is. Rapporteren vanuit je eigen locator zou de ontvangst van een ander op het verkeerde continent zetten.

    Bovenop cal_offset_db in config.toml, dat één getal voor het hele station is.
settings-antenna-somewhere-else-no-locator = De antenne staat elders en er is geen locator ingesteld, dus niets wat deze radio hoort wordt gerapporteerd aan PSK Reporter, WSPRnet of FreeDV Reporter. Typ hierboven de locator van de ontvanger om daarvandaan te rapporteren.
settings-ranges-are-mhz-low-high = Bereiken zijn in MHz, laag-hoog, gescheiden door komma's: 144-146, 430-440 — dat is 144000000-146000000 Hz en 430000000-440000000 Hz. Het zijn afstemfrequenties: een converteroffset verschuift ze niet. De converteroffset hierboven is het veld in hertz. Laat een bereik leeg om te gebruiken wat het apparaat over zichzelf meldt; een apparaat dat niets meldt wordt op zijn woord geloofd.
settings-careful-rtl-sdr-blog-v4 = Pas op bij een RTL-SDR: de Blog V4 upconverteert zelf onder 28,8 MHz, dus een negatieve offset die de hardware daar laat uitkomen verschuift twee keer.
settings-pick-radio-interface-above-configuration = Kies hierboven een radio-interface (deze configuratie gebruikte de verwijderde automatische detectie).
settings-configuration-used-removed-auto-detect = Deze configuratie gebruikte de verwijderde automatische detectie. Kies hierboven een interface zodra de machine waaraan de radio hangt heeft geantwoord.
settings-radio-has-no-interface-yet = Deze radio heeft nog geen interface — kies er hierboven een en druk op Toepassen / opnieuw verbinden.
settings-radio-has-no-interface-yet-2 = Deze radio heeft nog geen interface. Hierboven kan er een gekozen worden zodra de machine waaraan de radio hangt heeft geantwoord.
settings-apply-reconnect = Toepassen / opnieuw verbinden
settings-switch-interface-now-no-restart = Nu naar deze interface overschakelen — geen herstart nodig
settings-switch-server-s-radio-interface = Schakel de radio van de server nu over naar deze interface — geen herstart nodig. De oude blijft draaien als de nieuwe niet geopend kan worden.
settings-switches-live-radio-without-restarting = Schakelt de actieve radio over zonder herstart.
settings-everything-above-server-s-own = Alles hierboven is de eigen configuratie van de server, en wijzigingen worden daar opgeslagen. De meeste instellingen gelden zodra je ze wijzigt; die worden vastgelegd bij het openen van het apparaat — de samplefrequentie, een adres, de interface zelf — hebben deze knop nodig.
settings-dx-cluster-telnet = DX-cluster (telnet)
settings-enabled = Ingeschakeld
settings-host = Host
settings-port = Poort
settings-login-call = Inlogroepletters
settings-commands = Commando's
settings-one-per-line-e-g = één per regel, bijv. SET/FT8
settings-one-per-line-e-g-2 = één per regel, bijv. set/filter cont=eu
settings-sent-after-login-place-narrow = Verzonden na het inloggen. Dé plek om de feed te beperken — zonder filter is dit elke skimmer op aarde.
settings-rbn-paths-are-placed-country = RBN-paden worden geplaatst vanuit landmiddelpunten, niet vanuit locators — nauwkeurig voor een klein land, ver ernaast voor een groot. Ze zijn een aparte laag op de propagatiekaart en kunnen daar worden uitgezet.
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = POTA-activatorspots
settings-sota-spots = SOTA-spots
settings-psk-reporter-current-band = PSK Reporter (huidige band)
settings-upload-my-ft8-ft4-ft2 = Mijn FT8/FT4/FT2/JS8-decoderingen uploaden
settings-antenna-2 = Antenne
settings-collector = Collector
settings-t-4739-live-collector-14739-test = 4739 is de live collector, 14739 de testcollector
settings-max-age-s = Max. leeftijd (s)
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = Mijn WSPR-decoderingen uploaden
settings-send-every-wspr-reception-wsprnet = Stuur elke WSPR-ontvangst naar wsprnet.org. Standaard aan: het brengt niets de ether in, en rapporteren wat je hoort maakt van een WSPR-ontvanger een deel van het netwerk in plaats van een privécuriositeit. Een slot waarin niets gedecodeerd is, wordt ook gemeld, zo weet het netwerk een gesloten band van een uitgeschakelde ontvanger te onderscheiden.
settings-download-who-heard-me = Downloaden wie mij hoorde
settings-ask-wsprnet-org-which-stations = Vraag wsprnet.org welke stations deze hebben gedecodeerd. WSPR kent geen enkele bevestiging, dus dit is de enige manier waarop een zendbaken iets over zijn eigen bereik te weten komt.
settings-ask-every = Vraag elke
settings-looking-back = terugkijkend
settings-apply = {" "}TOEPASSEN{" "}
settings-persist-re-connect-feeds = Opslaan en de feeds (opnieuw) verbinden
settings-broadcast-stations = Omroepstations
settings-winlink-account = Winlink-account
settings-password = Wachtwoord
settings-winlink-account-password-not-gateway = Het wachtwoord van het Winlink-account, niet het gatewaywachtwoord. Het is hoofdlettergevoelig — voer het precies in zoals het is uitgegeven.
settings-locator = Locator
settings-how-connect = Hoe verbinden
settings-route = Route
settings-internet = Internet
settings-forward-cms-over-internet = Doorsturen met de CMS via internet
settings-radio-packet = Radio (packet)
settings-call-rms-gateway-air-radio = Roep een RMS-gateway op de ether aan. De radio moet in PACKET of PACKET-HF staan.
settings-gateway = Gateway
settings-via = Via
settings-digipeaters-order-separated-spaces-usually = Digipeaters, op volgorde, gescheiden door spaties. Meestal leeg — een gateway die je direct kunt horen, is een gateway die je direct moet aanroepen.
settings-speed = Snelheid
settings-most-rms-gateways-answer-1200 = De meeste RMS-gateways antwoorden op 1200. Een 1200-gateway op 9600 aanroepen klinkt precies als een gateway die uit de lucht is, dus stel dit in op wat de eigenaar van de gateway publiceert in plaats van te gokken. 9600 vereist ook de datapoort van de radio aan beide kanten. 300 is HF.
settings-frequency = Frequentie
settings-clear = WISSEN
settings-zero-leaves-dial-alone-which = Nul laat de afstemming met rust, wat je wilt als je op één kanaal blijft staan. Alles anders stemt de radio af zodra de sessie begint.
settings-my-gateways = Mijn gateways
settings-winlink-s-published-gateway-list = De gepubliceerde gatewaylijst van Winlink heeft een API-sleutel nodig die sdroxide niet heeft, dus houd er zelf een bij. De twee of drie gateways die vanaf één locatie bereikbaar zijn, leer je door te proberen, en ze veranderen zelden.
settings-use-2 = GEBRUIK
settings-call-one-next-connect = Roep deze aan bij de volgende verbinding
settings-current-dial = huidige afstemming
settings-forget = VERGEET
settings-forget-gateway = Vergeet deze gateway
settings-add-gateway = + GATEWAY TOEVOEGEN
settings-save-gateway-above-list = Sla de gateway hierboven op in de lijst
settings-internet-gateway = Internetgateway
settings-cms-address = CMS-adres
settings-client-name = Clientnaam
settings-winlink-s-production-servers-only = De productieservers van Winlink accepteren alleen clientnamen die ze kennen, en beantwoorden een onbekende met "Unknown client types are not allowed on production servers". Zolang sdroxide niet is geregistreerd bij het Winlink Development Team, heeft verbinden een naam nodig die ze herkennen.
settings-automatic-connection = Automatische verbinding
settings-connect-timer = Verbinden met een timer
settings-every = Elke
settings-persist-winlink-account = Het Winlink-account opslaan
settings-callsign-lookup = Roepletters opzoeken
settings-provider = Aanbieder
settings-auto-fill-name-qth-grid = Naam/QTH/locator automatisch invullen bij klik op spot & QSO
settings-choose-provider-enter-login = Kies een aanbieder om zijn login in te voeren.
settings-qrz-user = QRZ-gebruiker
settings-qrz-pass = QRZ-wachtwoord
settings-hamqth-user = HamQTH-gebruiker
settings-hamqth-pass = HamQTH-wachtwoord
settings-one-hamqth-account-does-both = Eén HamQTH-account doet beide — deze login is degene die het uploadtabblad van HamQTH gebruikt.
settings-upload = Uploaden
settings-auto-upload-each-new-qso = Elke nieuwe QSO automatisch uploaden
settings-master-switch-which-services-qso = De hoofdschakelaar. Naar welke diensten een QSO gaat, stel je in op elk tabblad hieronder.
settings-turn-auto-upload-each-new = Zet eerst "Elke nieuwe QSO automatisch uploaden" hierboven aan — een hier aangevinkte dienst wordt pas gebruikt als dat aan staat.
settings-auto-upload-off-above-so = Automatisch uploaden staat hierboven uit, dus de dienstvinkjes zijn uitgeschakeld en er wordt nog niets automatisch verstuurd — de UP-knop per QSO in het logboek werkt nog steeds.
settings-qrz-log-key = QRZ-logsleutel
settings-qrz-logbook-api-key-your = De API-sleutel van het QRZ-logboek uit je QRZ-logboekinstellingen — niet de login voor XML-opzoeking.
settings-eqsl-user = eQSL-gebruiker
settings-eqsl-pass = eQSL-wachtwoord
settings-eqsl-qth = eQSL QTH
settings-same-login-hamqth-callsign-lookup = Dezelfde login als de HamQTH-roepletteropzoeking — het invullen van de een vult ook de ander in.
settings-club-log-email = Club Log e-mail
settings-club-log-pass = Club Log wachtwoord
settings-club-log-key = Club Log sleutel
settings-wrl-api-key = WRL API-sleutel
settings-developer-api-key-world-radio = De ontwikkelaars-API-sleutel van World Radio League → Integrations → Developer API. Hij wordt eenmalig getoond als je hem aanmaakt, dus kopieer hem dan. Contacten gaan naar je standaardlogboek — stel er een in bij WRL als je er meer dan één bijhoudt.
settings-confirmations-download = Bevestigingen (downloaden)
settings-lotw-user = LoTW-gebruiker
settings-lotw-pass = LoTW-wachtwoord
settings-lotw-upload-uses-tqsl-export = LoTW-upload gebruikt TQSL — exporteer ADIF uit het logboek en onderteken het. LoTW/eQSL-bevestigingen worden hier gedownload om gewerkt-versus-bevestigd te markeren.
settings-sync-confirmations = BEVESTIGINGEN SYNCHRONISEREN
settings-fetching-subscriptions = Abonnementen ophalen…

    Om hem weer zelfstandig te gebruiken, wis je het vak Panadapter-ontvanger op de pagina van die radio.
settings-panadapter = Panadapter
settings-give-radio-wideband-panadapter-borrowing = Geef deze radio een breedband-panadapter door de ontvanger van een andere radio te lenen — een SDR op dezelfde antenne, of een die wordt gevoed vanuit de I.F.-uitgang van deze radio. Het spectrum, de waterval, de subontvanger, de digitale modi en de skimmer komen allemaal van die ontvanger; de afstemming, de modus, het filter en de zender blijven hier.
settings-receiver = Ontvanger
settings-none = Geen

    Wordt actief bij Toepassen.
settings-connected = Verbonden met

    Dit is niet het veld Converter hierboven, dat de radio opnieuw afstemt, en niet de I/Q-middenoffset van het CAT-tabblad, die over de eigen geluidskaart van deze radio gaat. Wat hier wordt getypt, wordt nooit naar een van beide radio's gestuurd.

    Wordt actief bij Toepassen.
settings-audio-2 = Audio van
settings-follow-dial = De afstemming volgen
settings-keep-receiver-radio-s-dial = Houd de ontvanger op de afstemming van deze radio, en verplaats de afstemming als je op de panadapter afstemt. Uit laat de ontvanger staan waar hij is — een vaste wacht op één segment terwijl de radio elders heen gaat.
settings-invert-spectrum = Spectrum omkeren
settings-mirror-receiver-s-span-about = Spiegel het bereik van de ontvanger om zijn midden, voor een aftakking waarvan de oscillator boven het signaal zit en de band verkeerd om aanlevert. Laat uit tenzij de waterval vol raakt met signalen die allemaal aan de verkeerde kant van de afstemming staan.
settings-mute-transmit = Dempen bij zenden
settings-silence-receive-audio-while-radio = Demp de ontvangstaudio terwijl deze radio zendt. Standaard aan: met de ontvanger op dezelfde antenne, of op de I.F. van deze radio, hoort hij tijdens een beurt je eigen zender. Beurten die aan de radio zelf worden ingeschakeld — de PTT van de microfoon, een voetschakelaar, MOX — tellen ook mee, zodra de CAT-verbinding meldt dat de radio is gaan zenden.
settings-blank-transmit = Wissen bij zenden
settings-stop-panadapter-waterfall-while-radio = Stop de panadapter en waterval terwijl deze radio zendt. Standaard aan: een zender die over de hele span wordt geschilderd, wist de band erachter. Zet uit om de band — of je eigen signaal — tijdens een beurt te bekijken, wat nuttig is met de ontvanger op een aparte antenne. Geldt voor beurten die sdroxide inschakelt: een beurt aan de radio zelf wordt gewoon ontvangen, dus het beeld blijft lopen wat dit ook zegt.
settings-per-mode-offsets-radio-whose = Offsets per modus, voor een radio waarvan de I.F. met de modus meebeweegt. Een modus die leeg blijft gebruikt de offset hierboven.
settings-computer = Op deze computer
settings-add-radio-computer = Een radio op deze computer toevoegen
settings-radio-s-settings-are-below = De instellingen van deze radio staan hieronder. Sleep om hem langs de balk te verplaatsen.
settings-switch-radio-dialog-follows-drag = Schakel over naar deze radio (het dialoogvenster volgt). Sleep om hem langs de balk te verplaatsen.
settings-tx = ● TX
settings-mute-radio-s-audio = Demp de audio van deze radio
settings-close-radio-configuration-kept = Sluit deze radio (de configuratie blijft bewaard)
settings-close-tab-close-radio-station = Sluit dit tabblad, of sluit de radio op zijn station
settings-close-tab = Sluit dit tabblad
settings-transverters-full = { $max } is het maximum van deze tabel
settings-dialling = { $url } bellen…
settings-region-note = Waar het station staat. Bepaalt elk bandplan: de bandgrenzen, de CW- / data- / spraaksubsegmenten op de watervalstrook, waar de skimmers luisteren, de aangeboden oproepfrequenties, en wat voor zenden als buiten de band telt. In regio { $region } maakt dat 70 cm { $region ->
        [1] 430–440 MHz — dus 446 is buiten de band, en 40 m stopt bij 7.200
        [2] 420–450 MHz, en 40 m loopt tot 7.300
       *[3] 430–450 MHz, en 80 m stopt bij 3.900
    }. Gaat direct in en geldt voor elke radio op dit station.
settings-rx-grid-reported = Ontvangsten via deze radio worden gerapporteerd vanuit { $grid }, niet vanuit je eigen locator.
settings-rx-grid-invalid = { $grid } is geen Maidenhead-locator, dus niets wat deze radio hoort wordt gerapporteerd. Vier of zes tekens: twee letters, twee cijfers, en eventueel nog twee letters.
settings-gateway-via = { " " }via { $via }
settings-auto-upload-to = Elke nieuwe QSO automatisch uploaden naar { $service }
settings-tle-updated = { $n } { $n ->
        [one] abonnement
       *[other] abonnementen
    } bijgewerkt: { $total } satellieten.
settings-tle-updated-failed = { $ok } van { $n } bijgewerkt; { $failed } mislukt — zie de rijen hierboven.
settings-panadapter-of = Deze radio is de panadapter van { $radio }
settings-radio-n = radio { $n }
settings-on-station = Op { $station }
settings-add-radio-on-station = Een radio toevoegen op { $station } — het station serveert hem meteen
settings-panadapter-receiver-for = Panadapter-ontvanger voor { $radio }
settings-hang-up-stays-on = Ophangen. De radio blijft op { $station }.
settings-close-on-station = { $radio } sluiten op { $station }? De configuratie blijft op die machine; het tabblad hier sluit mee, en dat van alle anderen ook.
settings-close-it-on = Sluit hem op { $station }
settings-locator-hint = locator

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = Hoorbare meldingen
settings-alerts-sound-alarm-when-decode-matters = Alarm geven als een decodering ertoe doet
settings-alerts-plays-over-own-audio-output = Speelt af via zijn eigen audio-uitgang, zodat het ook te horen is als de band in een andere luidspreker zit dan dit scherm.
settings-alerts-volume = Volume
settings-alerts-output = Uitvoer
settings-alerts-test = Test
settings-alerts-what-sound = Wat laten klinken
settings-alerts-each-station-quiet-while-after = Elk station is na een melding een tijd stil, zodat een drukke band niet bij elk slot afgaat. Stem en Toon + stem worden voorgelezen door de stem voor gesproken meldingen — zet die aan in Instellingen → UI.

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = Versterker
settings-amp-waiting-station-s-amplifier-configuration = Wachten op de versterkerconfiguratie van het station…
settings-amp-hobbypcb-hardrock-500-hardrock-50 = Een HobbyPCB Hardrock-500 of Hardrock-50 op zijn USB-seriële poort. SDRoxide leest de meters, schakelt band, modus en antenne, en bedient de tuner vanuit het AMP-venster. De versterker wordt nog steeds ingeschakeld via de PTT-lijn van de radio, zoals voorheen.
settings-amp-enable = Inschakelen
settings-amp-model = Model
settings-amp-untested-panel-has-only-been = Niet getest: dit paneel is alleen getest met een Hardrock-500. Meetgrenzen voor dit model zijn afgeleid van die van de Hardrock-500 en kunnen onjuist zijn. Houd het eigen display van de versterker in de gaten.
settings-amp-port = Poort
settings-amp-choose = — kies —
settings-amp-baud = Baud
settings-amp-must-match-amplifier-s-menu = Moet overeenkomen met het menu van de versterker. Het eigen bedieningspaneel van HobbyPCB vereist 115200.
settings-amp-follow-radio-s-band = De band van de radio volgen
settings-amp-sends-transmit-frequency-amplifier-every = Stuurt bij elke bandwissel de zendfrequentie naar de versterker, zodat zijn filters de radio volgen zonder eigen CAT-kabel. Wordt nooit tijdens het zenden verstuurd.
settings-amp-show-power-swr-tx-meter = Toon zijn vermogen en SWR op de TX-meter
settings-amp-only-where-radio-does-not = Alleen waar de radio ze niet zelf meet, en alleen terwijl de versterker in de keten zit.
settings-amp-off = Uit
settings-amp-connecting = Verbinden…
settings-amp-status = Status
settings-amp-apply = {" "}TOEPASSEN{" "}
settings-amp-save-re-open-amplifier-s = Opslaan en de poort van de versterker (opnieuw) openen

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = Toetsenbord
settings-controls-click-shortcut-rebind-then-press = Klik op een sneltoets om hem opnieuw toe te wijzen en druk dan op de toetscombinatie (Esc annuleert). Toewijzingen worden genegeerd terwijl je in een tekstveld typt.
settings-controls-shortcut = Sneltoets
settings-controls-does = Doet
settings-controls-step-mode = Stap / modus
settings-controls-accel = Versn.
settings-controls-on = Aan
settings-controls-press-key = druk op een toets…
settings-controls-remove-binding = Verwijder deze toewijzing
settings-controls-add-shortcut = + Sneltoets toevoegen
settings-controls-restore-defaults = Standaardwaarden herstellen
settings-controls-bind-hold-talk-space = Houd-om-te-praten aan Spatie koppelen
settings-controls-hold-space-transmit-releasing-losing = Houd Spatie ingedrukt om te zenden; loslaten — of het verliezen van de vensterfocus — stopt het zenden
settings-controls-unkey-held-ptt-after = Een vastgehouden PTT loslaten na
settings-controls-backstop-against-stuck-key-controller = Vangnet tegen een vastzittende toets of een controller die stopt met rapporteren. 0 schakelt uit.
settings-controls-panadapter-mouse = Panadapter-muis
settings-controls-wheel = Wiel
settings-controls-wheel-shift = Wiel + Shift
settings-controls-tune-step = Afstemstap
settings-controls-zoom-rate = Zoomsnelheid
settings-controls-click-tune-rounding = Afronding bij klik-afstemmen
settings-controls-invert-wheel-direction = Wielrichting omkeren
settings-controls-left-drag-tunes-well-pans = Links slepen stemt af én verschuift
settings-controls-off-makes-left-drag-pan = Uit laat links slepen alleen de weergave verschuiven, zoals rechts slepen.
settings-controls-scroll-digit-frequency-readout-tune = Scroll een cijfer op de frequentieweergave om af te stemmen
settings-controls-both-wheel-actions-are-tune = Beide wielacties zijn Afstemmen — er is geen manier meer om te zoomen.
settings-controls-restore-mouse-defaults = Muisstandaarden herstellen
settings-controls-mouse-buttons = Muisknoppen
settings-controls-left-right-buttons-are-reserved = De linker- en rechterknop zijn gereserveerd voor afstemmen en verschuiven; de middelste en extra knoppen zijn vrij. Een zijknop die voor PTT wordt ingedrukt gedraagt zich als een voetschakelaar.
settings-controls-add-mouse-button = + Muisknop toevoegen
settings-controls-f1-always-opens-manual-even = F1 opent altijd deze handleiding, ook tijdens het typen, dus kan niet opnieuw worden toegewezen.
settings-controls-midi-controller = MIDI-controller
settings-controls-midi-controllers-need-native-app = MIDI-controllers hebben de native app nodig — de browserclient heeft geen MIDI-toegang.
settings-controls-any-class-compliant-midi-surface = Elk class-compliant MIDI-oppervlak werkt: het jogwiel van een DJ-controller is een prima VFO-knop, de pads maken PTT- en bandknoppen, de faders maken versterkingsregelingen.
settings-controls-enable = Inschakelen
settings-controls-controller = Controller
settings-controls-feedback = Feedback naar
settings-controls-rescan-ports = Poorten opnieuw scannen
settings-controls-not-connected = Niet verbonden.
settings-controls-move-control-see-here = Beweeg een regelaar om hem hier te zien.
settings-controls-control = Regelaar
settings-controls-reads = Leest als
settings-controls-led = LED
settings-controls-move = beweeg…
settings-controls-click-then-move-control-you = Klik, en beweeg dan de regelaar die je wilt koppelen
settings-controls-speed-sensitivity-spin-faster-tune = Snelheidsgevoeligheid: draai sneller om sneller af te stemmen
settings-controls-send-current-value-back-light = Stuur de huidige waarde terug, om een LED te laten branden of een fader te bewegen
settings-controls-add-midi-control = + MIDI-regelaar toevoegen
settings-controls-clear-all = Alles wissen
settings-controls-endless-jog-encoders-send-relative = Eindeloze (jog-)encoders sturen een relatieve stap in plaats van een positie, in een van drie coderingen die bij kleine bewegingen op elkaar lijken. LEARN raadt aan de hand van een draai met de klok mee; als de knop dan de verkeerde kant op afstemt, vink dan “rev” aan.
settings-controls-none = — geen —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = Systeemstandaard
settings-general-remote-access = Toegang op afstand
settings-general-what-remote-client-browser-page = Wat een client op afstand — de browserpagina, of een andere sdroxide gestart met --connect — moet opgeven voordat dit station hem laat bedienen. Geldt in servermodus (--server); de volgende aanmelding neemt een wijziging over, zonder herstart.
settings-general-username = Gebruikersnaam
settings-general-password = Wachtwoord
settings-general-clients-must-give-password-leaving = Clients moeten het wachtwoord opgeven. De gebruikersnaam leeg laten is prima.
settings-general-clients-must-sign = Clients moeten inloggen.
settings-general-empty-anyone-who-can-reach = ⚠ Leeg: iedereen die de poort van de server kan bereiken, kan deze radio bedienen, onder jouw roepletters. Stel een wachtwoord in voordat je de poort doorstuurt.
settings-general-stored-clear-config-toml-like = Leesbaar opgeslagen in config.toml, zoals de andere wachtwoorden die sdroxide bewaart.
settings-general-receive-audio-gain = Versterking ontvangstaudio
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    Extra versterking op alles wat deze radio naar de luidsprekers stuurt, bovenop de volumeregeling. Laat op 0 dB tenzij de radio op vol volume zacht is: de bovenkant van de volumebalk is de audio zoals die binnenkomt, dus hij kan een radio zachter maar nooit harder zetten, en bij sommige transceivers ligt de USB-geluidsuitgang ruim onder de volle schaal.

    Ga in stappen van 6 dB omhoog. Te veel clipt — de audio wordt op volle schaal begrensd in plaats van rond te laten slaan, dus overdrijven klinkt hard in plaats van luid. Opnamen worden hiervoor gemaakt en worden er niet door beïnvloed.
settings-general-reload-band-plan = BANDPLAN OPNIEUW LADEN
settings-general-re-read-bandplan-json-machine = Lees bandplan.json opnieuw op de machine waaraan de radio hangt en pas het toe — bandgrenzen, subsegmenten en skimmervensters — zonder herstart.
settings-general-running-station-s-own-band = Draait op het eigen bandplan van het station.
settings-general-running-built-iaru-tables = Draait op de ingebouwde IARU-tabellen.
settings-general-settings-file = Instellingenbestand
settings-general-settings-are-machine-radio-attached = De instellingen staan op de machine waaraan de radio hangt. Exporteer ze daar.
settings-general-export = EXPORTEREN…
settings-general-write-every-setting-station-radios = Schrijf elke instelling van dit station — de radio's, de modi, de servers, de geheugens, het bandplan — naar één bestand dat je naar een andere installatie kunt meenemen. Je logboek en een opgeslagen serverwachtwoord blijven hier.
settings-general-import = IMPORTEREN…
settings-general-replace-station-s-settings-ones = Vervang de instellingen van dit station door die in een bestand dat uit een andere installatie is geëxporteerd. Herstart sdroxide daarna.
settings-general-import-overwrites-what-here-file = Een import overschrijft wat hier staat, bestand voor bestand, en gaat in bij de volgende start van sdroxide — de instellingen die al in het geheugen zitten zouden er anders meteen overheen worden teruggeschreven. Alles wat het bestand niet noemt blijft zoals het is, dus een bundel van een station met één radio verwijdert hier geen tweede radio. Je logboek zit nooit in het bestand: exporteer het als ADIF vanuit het LOG-venster als je dat ook wilt verhuizen.
settings-general-sdroxide-settings = sdroxide-instellingen
settings-general-swr-guard = SWR-bewaking
settings-general-stop-transmitting-high-swr = Stoppen met zenden bij hoge SWR
settings-general-trip = Afslaan bij
settings-general-stops-transmission-when-radio-reports =
    Stopt de uitzending wanneer de radio een SWR meldt op of boven dit getal, en houdt zenden vergrendeld totdat je het bevestigt. Vangt een losgekoppelde antenne, een defecte voedingslijn, of een schakelaar die op de verkeerde poort is blijven staan.

    Afstemmen wordt anders behandeld, omdat het voeden van een mismatch daar juist het punt van is: een antennetuner krijgt de dubbele limiet en ongeveer vijf seconden voordat de bewaking helemaal geldt. Een handmatige tuner die langer nodig heeft, wil de bewaking voor de sessie uitgeschakeld hebben.

    Vereist een radio die SWR via CAT meldt. Negeert de eerste vijfde seconde van elke uitzending en wacht niet op hoog vermogen.
settings-general-per-mode-settings = Instellingen per modus
settings-general-reset-every-mode = ELKE MODUS HERSTELLEN
settings-general-forget-every-mode-s-remembered = Vergeet de onthouden AGC-, squelch-, ruisonderdrukkings-, notch- en stereoschakelaars van elke modus, en zet de eigen standaardwaarden van elke modus terug.
settings-general-selecting-mode-lays-mode-s = Een modus kiezen zet de eigen startwaarden van die modus voor AGC, squelch, ruisonderdrukking, de notch en de stereoschakelaars op de ontvanger, en een wijziging wordt alleen voor die modus onthouden. De cirkelpijl-chip aan het eind van de filter/ruis-rij van de ontvanger toont wat er is gewijzigd en zet de huidige modus terug.
settings-general-channels = Kanalen

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = Operator
settings-net-callsign-grid-are-set-general = Roepletters en locator stel je in op het tabblad Algemeen.
settings-net-set-your-callsign-grid-general = ⚠ Stel je roepletters en locator in op het tabblad Algemeen.
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = Inschakelen
settings-net-connects-whenever-enabled-your-station = Verbindt zodra het is ingeschakeld. Je station wordt alleen aan anderen getoond terwijl de radio in RADE staat — in elke andere modus blijf je verbonden maar verborgen.
settings-net-station = Station
settings-net-message = Bericht
settings-net-receive-only-i-cannot-transmit = Alleen ontvangen (ik kan niet zenden)
settings-net-server = Server
settings-net-host = Host
settings-net-port = Poort
settings-net-tls-wss = TLS (wss://)
settings-net-not-yet-implemented-freedv-gui = Nog niet geïmplementeerd — FreeDV GUI gebruikt ook gewoon ws://.
settings-net-reporting = Rapportage
settings-net-report-stations-i-decode = Stations die ik decodeer rapporteren
settings-net-sends-rx-report-each-callsign = Stuurt een rx_report voor elke roepletters die uit een RADE End-of-Over-frame is hersteld.
settings-net-show-other-reporter-stations-spots = Andere reporterstations als spots tonen
settings-net-adds-them-panadapter-overlay-world = Voegt ze toe aan de panadapter-overlay, de wereldkaart en het SPOTS-venster onder het FREEDV-filter.
settings-net-set-your-callsign-grid-general-2 = ⚠ Stel je roepletters en locator in op het tabblad Algemeen. Zonder beide is de verbinding alleen-lezen: je ziet andere stations maar verschijnt zelf niet.
settings-net-apply = {" "}TOEPASSEN{" "}
settings-net-persist-re-connect = Opslaan en (opnieuw) verbinden
settings-net-season = Seizoen
settings-net-downloading = downloaden…
settings-net-using-built-copy = de ingebouwde kopie wordt gebruikt
settings-net-your-own-stations-corrections-go = Je eigen stations en correcties horen in dit bestand. sdroxide schrijft het nooit, en een item hier vervangt een geplande met dezelfde naam en frequentie.
settings-net-reload = Herladen
settings-net-re-read-your-station-file = Lees je stationsbestand opnieuw na het bewerken
settings-net-broadcast-stations-labelled-waterfall-come = De omroepstations die op de waterval worden benoemd, komen uit de lijst die in deze build is ingebouwd. Bewerken vereist de desktop-app.
settings-net-ask-service-whether-these-credentials = Vraag de dienst of deze inloggegevens werken. Past eerst de instellingen hierboven toe, en logt niets: het leest alleen.
settings-net-checking = controleren…

## Settings window: messages with values filled in.

settings-amp-connected-to = Verbonden met de { $model }
settings-controls-last-midi = Laatste bericht: { $message }  waarde { $value }
settings-general-band-plan-file = Elke bandgrens en elk subsegment komt uit { $path }, de eerste keer geschreven uit de ingebouwde IARU-tabellen en daarna van jou om te bewerken — beperk een band tot je vergunning en sdroxide weigert erbuiten te zenden. Frequenties zijn in MHz; verwijder het bestand voor een verse kopie van de standaardwaarden. Dit is de regionale toewijzing, niet je vergunning: je eigen voorwaarden kunnen smaller zijn, en nationale plannen verschillen binnen een regio.
settings-general-exported = { $note } geëxporteerd — kies waar je het opslaat.
settings-general-export-failed = Exporteren mislukt: { $error }
settings-general-import-failed = Importeren mislukt: { $error }
settings-general-imported = { $summary } — herstart sdroxide om ze te gebruiken.
settings-general-import-skipped = { $path } overgeslagen: { $why }
settings-general-swr-tripped = ⚠ Momenteel afgeslagen bij { $swr }:1 — zenden is vergrendeld.
settings-net-call-grid-set-on-general = { $call } / { $grid } — ingesteld op het tabblad Algemeen
settings-net-reporting-as = Rapporteren als { $call } / { $grid } — SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ Downloaden mislukt: { $error }. Het ingebouwde schema wordt gebruikt.
settings-net-test-service = { $service } testen

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = Zenden staat uit terwijl een converter is ingesteld — geef hierboven aan wat in de zendlijn zit om het weer aan te zetten.
settings-transmit-goes-through-same-converter = Zenden gaat via dezelfde converter, dus de radio werkt in beide richtingen dezelfde offset van de afstemming.
settings-transmit-not-converted-radio-transmits = Zenden wordt niet geconverteerd: de radio zendt op de frequentie die de afstemming toont, terwijl ontvangst via de converter komt.
settings-transmit-takes-own-offset-separate = Zenden gebruikt een eigen offset, los van die van ontvangst.
settings-no-enabled-subscriptions-update = Geen ingeschakelde abonnementen om bij te werken.
settings-add-radio-here-station-you = Voeg een radio toe — hier, of op een station waarmee je verbonden bent

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = Elke bandgrens en elk subsegment komt uit bandplan.json op de machine waaraan de radio hangt. Dit is de regionale toewijzing, niet je vergunning: je eigen voorwaarden kunnen smaller zijn, en nationale plannen verschillen binnen een regio.

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = Ontvangen afbeeldingen → FTP
settings-picup-uploads-every-received-sstv-picture = Uploadt elke ontvangen SSTV-afbeelding (en, indien aangevinkt, elke weerkaart) zodra die wordt opgeslagen, ingedeeld zoals QSSTV het doet, zodat webpagina's voor QSSTV ongewijzigd werken.
settings-picup-upload-received-pictures = Ontvangen afbeeldingen uploaden
settings-picup-server = Server
settings-picup-port = Poort
settings-picup-security = Beveiliging
settings-picup-user = Gebruiker
settings-picup-password = Wachtwoord
settings-picup-certificate = Certificaat
settings-picup-trusted-when-computer-s-operating = Vertrouwd als het besturingssysteem van deze computer het vertrouwt, zoals een browser hier zou doen. Hieronder kan een privé-CA worden toegevoegd (PEM).
settings-picup-extra-ca = Extra CA
settings-picup-only-certificate-fingerprint-accepted-way = Alleen het certificaat met deze vingerafdruk wordt geaccepteerd — de manier om een zelfondertekende server te vertrouwen. TEST toont die van de server.
settings-picup-any-certificate-accepted-link-encrypted = ⚠ Elk certificaat wordt geaccepteerd. De verbinding is versleuteld, maar iedereen op het pad kan zich als de server voordoen en het wachtwoord lezen.
settings-picup-plain-ftp-sends-password-pictures = ⚠ Gewone FTP stuurt het wachtwoord en de afbeeldingen onversleuteld.
settings-picup-sstv = SSTV
settings-picup-sstv-dir = SSTV-map
settings-picup-weather-fax = Weerfax
settings-picup-wefax-dir = WEFAX-map
settings-picup-empty-login-directory = Leeg is de loginmap.
settings-picup-file-names = Bestandsnamen
settings-picup-format = Indeling
settings-picup-add-extension = extensie toevoegen
settings-picup-image1-png-rather-than-qsstv = image1.png in plaats van de kale image1 van QSSTV
settings-picup-caption-bar = Onderschriftbalk
settings-picup-time-frequency-mode-sender-s = Tijd, frequentie, modus en de FSK-ID van de zender bovenaan, zoals QSSTV het tekent. Alleen de geüploade kopie wordt gemarkeerd.
settings-picup-fit-960-768 = Passend maken op 960 × 768
settings-picup-qsstv-s-heartbeat-tiny-picture = De hartslag van QSSTV: een piepklein beeld waarvan de hoogte de minuut is, naast elk beeld geüpload, en twee pixels breed gemaakt wanneer het programma sluit.
settings-picup-nat-workaround = NAT-omweg
settings-picup-ignore-private-address-server-s = Negeer een privéadres in het passieve-modusantwoord van de server en gebruik het eigen adres van de server — voor een server achter NAT die zijn publieke adres niet kent.
settings-picup-apply = {" "}TOEPASSEN{" "}
settings-picup-test = TEST
settings-picup-connect-settings-above-applied-not = Maak verbinding met de instellingen hierboven — toegepast of niet — log in, en schrijf en verwijder een klein bestand in elke uploadmap.
settings-picup-checking = controleren…
settings-picup-trust-certificate = Vertrouw dit certificaat
settings-picup-pin-switch-pinned-fingerprint-one = Pin het vast: schakel over naar "Vastgezette vingerafdruk" met deze. Controleer hem eerst bij de server — dit is wat de server zei, geen bewijs van wie hij is. Dan TOEPASSEN.
settings-picup-last-upload = Laatste upload

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = Profielen: de opgeslagen opstellingen van het station
settings-profiles-profile-captures-whole-working-setup = Een profiel legt de hele werkopstelling vast — de afstemmingen en VFO's, de modus en filters, de versterkingen, drive en antennes, je roepletters en berichtsjablonen, en de registers per band — en past die met één klik weer toe. De hardware (back-end, audioapparaten, converters) blijft bewust met rust gelaten: een profiel is de manier waarop je het station bedient, niet de radio waarop het draait.
settings-profiles-save-current-setup = De huidige opstelling opslaan
settings-profiles-profile-name = profielnaam
settings-profiles-save = Opslaan
settings-profiles-applies-whatever-sdroxide-attached-dials = Geldt voor waaraan sdroxide ook is gekoppeld: afstemmingen en VFO's, modus en filters, niveaus, versterkingen, drive, antennes, de digitale identiteit en de bandstapels.
settings-profiles-no-profiles-saved-yet-name = Nog geen profielen opgeslagen — geef er hierboven een naam aan en druk op Opslaan.
settings-profiles-saved-profiles = Opgeslagen profielen
settings-profiles-apply = Toepassen

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = Zend-/ontvangstschakelaar
settings-relay-waiting-station-s-t-r = Wachten op de T/R-schakelaarconfiguratie van het station…
settings-relay-closes-contact-while-station-transmits = Sluit een contact terwijl dit station zendt — om de antenne-ingang van de SDR los te koppelen en te aarden, en om een versterker of een externe T/R-relais in volgorde mee te schakelen. Werkt met de goedkope USB-relaisborden (LCUS, KMtronic, Numato), met een seriële RTS/DTR-lijn naar elke interface die een PTT-sluiting wil, en met al het andere via een commandohaak.
settings-relay-hardware = Hardware
settings-relay-board = Bord
settings-relay-lcus-four-byte-a0-command = LCUS is het vierbyte-A0-commandobord dat onder een dozijn namen wordt verkocht; KMtronic en Numato kunnen ook gevraagd worden waarop ze staan
settings-relay-port = Poort
settings-relay-choose = — kies —
settings-relay-device = Apparaat
settings-relay-gpio-pin = GPIO-pin
settings-relay-pin-3-every-homebrew-plan = Pin 3 op elk zelfbouwplan en elk commercieel bord — het is die aan het eind van het pakket waar een draad aan kan worden gesoldeerd. Als het relais niet klikt, is de kaart een kloon die deze pinnen helemaal niet implementeert.
settings-relay-gpio-chip = GPIO-chip
settings-relay-linux-only-gpiodetect-lists-chips = Alleen Linux. `gpiodetect` toont de chips op de machine.
settings-relay-transmit = Bij zenden
settings-relay-receive = Bij ontvangen
settings-relay-nothing-switched = Er wordt niets geschakeld.
settings-relay-starting-program-costs-unpredictable-amount = Een programma starten kost een onvoorspelbare hoeveelheid tijd, en daar wordt niets van in de voorloop meegeteld — laat hier dus een ruimere marge dan bij een relaisbord. De getallen in de contacttabel betekenen niets voor deze verbinding: je commando's worden één keer bij key-down en één keer bij key-up uitgevoerd.
settings-relay-contacts = Contacten
settings-relay-contact-1-rts-contact-2 = Contact 1 is RTS en contact 2 is DTR.
settings-relay-no = Nr.
settings-relay-gpio = GPIO
settings-relay-name = Naam
settings-relay-job = Taak
settings-relay-tx-closes = TX sluit
settings-relay-lead = Voorloop
settings-relay-hold = Nahouden
settings-relay-line-s-offset-chip-bcm = De offset van de lijn op deze chip — het BCM-nummer op een Raspberry Pi-header, wat niet het fysieke pinnummer is
settings-relay-sdr-antenna = SDR-antenne
settings-relay-decides-default-timings-what-log = Bepaalt de standaardtijden en hoe het logboek het noemt — en, voor een banddecoder, dat het contact de bandtabel hieronder volgt
settings-relay-transmitting-energises-coil-off-transmitting =
    Aan: zenden bekrachtigt de spoel. Uit: zenden laat hem los.

    Dit is een bedradingsbeslissing voordat het een instelling is — zie de opmerking hieronder.
settings-relay-how-long-before-rf-contact = Hoe lang vóór RF dit contact van het RX-woord van zijn band naar het TX-woord gaat. Een herafstemming verplaatst het direct; dit is voor key-down, wanneer een filter alleen voor zenden in het circuit moet zitten voordat het vermogen er is.
settings-relay-closed-long-before-rf-small = Zo lang vóór RF gesloten. Een klein coaxrelais schakelt in 5 tot 15 ms. Zenden wacht op de langste hiervan, dus een grote waarde is een hoorbare onderbreking in de ontvangstaudio; alles boven 250 ms wordt genegeerd.
settings-relay-how-long-after-rf-stops = Hoe lang nadat RF stopt dit contact van zijn TX-woord terug naar zijn RX-woord gaat.
settings-relay-opened-long-after-rf-stops = Zo lang nadat RF stopt geopend. Met opzet langer dan de voorloop: de antenne een moment te laat terugsturen kost niets, en te vroeg kost een frontend. Nul is een echt antwoord, en het juiste voor de keylijn van een versterker.
settings-relay-test = {" "}TEST{" "}
settings-relay-not-while-station-air = Niet terwijl het station in de lucht is
settings-relay-close-contact-half-second-so = Sluit dit contact een halve seconde, zodat je het relais kunt horen en de bedrading kunt controleren met de zender koud
settings-relay-remove = Verwijderen
settings-relay-add-contact = + contact toevoegen
settings-relay-if-will-not-answer = Als het niet antwoordt
settings-relay-switch-exists-protect-receiver-not = Een schakelaar die bestaat om een ontvanger te beschermen is het niet waard als hij stilletjes ophoudt hem te beschermen — dus weigeren is de standaard. Kies het andere waar een losse USB-kabel die je contest beëindigt de slechtere uitkomst is.
settings-relay-order-timings-longest-lead-closes = De volgorde zijn de tijden: de langste voorloop sluit het eerst, en het langste nahouden opent het laatst. Geef het antennerelais dus de langere voorloop en het langere nahouden, en de versterker de kortere van elk — dan wordt de antenne altijd geschakeld voordat de versterker wordt ingeschakeld, en de versterker valt altijd af voordat de antenne terugkomt.
settings-relay-transmit-sense-input = Zendherkenningsingang
settings-relay-when-transceiver-keys-itself-own = Als een transceiver zichzelf inschakelt — zijn eigen microfoonknop, voetschakelaar, VOX of keyer — komt sdroxide dat te weten door via CAT te vragen, en dat antwoord komt een paar honderd milliseconden na het begin van de beurt. Bedraad de SEND-lijn van de rig, via een opto-isolator, op een handshake-ingang van dezelfde poort en het wordt in een paar milliseconden gezien. De meter en de zendvergrendeling worden er sneller van.
settings-relay-line = Lijn
settings-relay-transmitting = Zenden is
settings-relay-high-line = een hoge lijn
settings-relay-most-opto-isolated-interfaces-pull = De meeste opto-geïsoleerde interfaces trekken de lijn omlaag als de rig zendt, dus dit staat meestal uit
settings-relay-belongs-radio = Hoort bij radio
settings-relay-which-radio-tab-sensed-transceiver = Welk radiotabblad de herkende transceiver is. Op een station met één radio is dit 1 en hangt er niets van af.
settings-relay-needs-serial-port-handshake-lines = Vereist een seriële poort — de handshake-lijnen zitten daarop.
settings-relay-wiring-only-fail-safe-survives = Bedrading is de enige beveiliging die dit programma overleeft.
settings-relay-choose-tx-closes-so-de = Kies “TX sluit” zodat het *onbekrachtigde* contact de toestand is die je wilt als het bord is losgekoppeld, de computer uit staat of sdroxide niet draait. Bedraad zodat onbekrachtigd geaard betekent, laat een dood relais de SDR doof — vervelend, en veilig. Andersom bedraad is een dood relais plus één uitzending een dode frontend. Kies met een echte versterker erachter de eerste.
settings-relay-cannot-protect-first-instant-over = Dit kan het eerste moment van een beurt die je aan de radio start niet beschermen. Zend de rig vanuit sdroxide en de contacten lopen altijd voor op de RF; bedraad de herkenningsingang hierboven en ze volgen binnen milliseconden. Gebruik voor een frontend die echt bescherming waard is ook een RF-gestuurde hardware-T/R-schakelaar — geen enkel programma op een computer kan het hele antwoord zijn.
settings-relay-contacts-transmit = contacten in ZENDEN
settings-relay-not-open = Niet geopend.
settings-relay-status-unknown-press-apply = Status onbekend — druk op TOEPASSEN.
settings-relay-station-air-apply-will-take = Het station is in de lucht — TOEPASSEN gaat in na deze beurt.
settings-relay-band-decoder-outputs = Banddecoderuitgangen
settings-relay-one-control-word-per-band = Eén stuurwoord per band: bit 0 is contact 1, bit 1 contact 2 enzovoort, overeenkomstig de nummering hierboven. RX volgt de band van de ontvangstafstemming; TX de band van de radio die zond, ingeschakeld op de voorloop van elk contact vóór RF en teruggezet na zijn nahouden — geef ze dezelfde waarde voor een filter dat niet uitmaakt welke kant de RF op gaat, en verschillende voor een voorversterkerbypass alleen voor ontvangst of een LPF-bank alleen voor zenden. Banden die op 0 blijven, activeren niets. Geldt bij Toepassen / opnieuw verbinden.
settings-relay-band = Band
settings-relay-rx = RX
settings-relay-tx = TX
settings-relay-outputs-asserted = Geactiveerde uitgangen
settings-relay-other = Overig
settings-relay-everywhere-outside-amateur-bands-short = Overal buiten de amateurbanden — kortegolfluisteren, en alles waar de afstemming van een transverter op uitkomt dat geen band dekt.
settings-relay-apply = {" "}TOEPASSEN{" "}
settings-relay-persist-re-open-switch = Opslaan en de schakelaar (opnieuw) openen

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = Verbinden met een server
settings-remote-drive-sdroxide-running-somewhere-else = Bedien een sdroxide die elders draait — de shack-machine, een locatie op afstand — via het netwerk. Start hem daar met --server en voer hier zijn adres in. De radio, de decoders en het logboek blijven op die machine; dit scherm krijgt de waterval en de audio, en jouw luidsprekers en microfoon worden gebruikt.
settings-remote-address = Adres
settings-remote-host-name-ip = hostnaam of IP
settings-remote-machine-running-sdroxide-server-host = De machine die sdroxide --server draait: een hostnaam, een IPv4- of IPv6-adres, of een volledige ws://- (of wss://-)URL als de server achter een reverse proxy staat — een URL wordt precies gebruikt zoals getypt en negeert de poort en de schakelaar hieronder.
settings-remote-port = Poort
settings-remote-port-server-listens-server-port = De poort waarop die server luistert — server_port in zijn config.toml. 4950 tenzij --port is opgegeven.
settings-remote-secure = Beveiligd
settings-remote-wss-tls = wss:// (TLS)
settings-remote-tick-where-server-sits-behind = Vink dit aan waar de server achter iets staat dat HTTPS voor hem afhandelt — een reverse proxy op poort 443, de gebruikelijke manier om een station over het open internet te bereiken. De eigen server van sdroxide spreekt gewone ws:// en wil dit niet.
settings-remote-connect = {" "}VERBINDEN{" "}
settings-remote-open-station-radio-tab-own = Open dit station als een eigen radiotabblad
settings-remote-client-cannot-hold-second-connection = Deze client kan geen tweede verbinding aanhouden.

    Als de server om een gebruikersnaam en wachtwoord vraagt, verschijnt zijn aanmeldscherm in het nieuwe tabblad. Een gewone ws://-verbinding draagt niets hiervan versleuteld, dus leid hem over het open internet via een VPN of een SSH-tunnel, of handel HTTPS af vóór de server en vink hierboven Beveiligd aan.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = WSJT-X UDP-broadcast
settings-servers-waiting-station-s-wsjt-x = Wachten op de WSJT-X-broadcastconfiguratie van het station…
settings-servers-sends-decodes-station-status-logged = Stuurt decoderingen, stationsstatus en gelogde QSO's zoals WSJT-X dat doet, zodat GridTracker, JTAlert, N1MM+ en Log4OM ongewijzigd met sdroxide werken. Alleen uitvoer — niets op deze socket kan de radio aanraken.
settings-servers-enable = Inschakelen
settings-servers-send = Verzenden naar
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 bereikt clients op deze machine; een LAN-adres of een multicastgroep (224.0.0.1) bereikt anderen
settings-servers-port = Poort
settings-servers-t-2237-port-every-client-defaults = 2237 is de poort die elke client standaard gebruikt
settings-servers-identify = Identificeren als
settings-servers-name-clients-see-some-loggers = De naam die clients zien. Sommige loggers accepteren alleen verkeer dat zich als WSJT-X identificeert.
settings-servers-n1mm-contactinfo-broadcast = N1MM+ contactinfo-broadcast
settings-servers-same-news-n1mm-s-words = Hetzelfde nieuws in de woorden van N1MM: één XML-datagram per gelogd contact, voor de loggers die naar N1MM luisteren in plaats van naar WSJT-X — de brug van de World Radio League accepteert beide. Alleen contacten; er is geen decoderingsstroom in dit protocol. Een eigen schakelaar en een eigen poort, omdat een station beide wil.
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 bereikt clients op deze machine. Het eigen advies van N1MM voor een contestnetwerk is het broadcastadres van dit subnet — 192.168.1.255 voor een 192.168.1.n-netwerk — dat elke positie tegelijk bereikt.
settings-servers-t-12060-port-n1mm-s-documentation = 12060 is de poort die de documentatie van N1MM aanbeveelt
settings-servers-station-name = Stationsnaam
settings-servers-what-n1mm-calls-stationname-name = Wat N1MM de StationName noemt: de naam van de computer die het pakket verstuurde. Loggers tonen die om de ene operatorpositie van de andere te onderscheiden.
settings-servers-cw-panel-s-copy-plain = De kopie van het CW-paneel als platte tekst, één UDP-datagram per stuk zodra het vaststaat — voor een stationscontroller, het CW-venster van een logger of een script. Geen protocol: een luisteraar voegt gewoon toe wat binnenkomt (probeer nc -ul 9999). Alleen vaststaande tekst wordt verzonden, nooit de staart die de decoder nog herziet. Elke radio heeft zijn eigen schakelaar.
settings-servers-apply = {" "}TOEPASSEN{" "}
settings-servers-persist-re-open-broadcast-sockets = Opslaan en de broadcastsockets (opnieuw) openen
settings-servers-hamlib-rigctld-server = Hamlib rigctld-server
settings-servers-waiting-station-s-rigctld-configuration = Wachten op de rigctld-configuratie van het station…
settings-servers-listen = Luisteren op
settings-servers-t-127-0-0-1-machine = 127.0.0.1 (deze machine)
settings-servers-t-0-0-0-0-whole = 0.0.0.0 (heel het netwerk)
settings-servers-t-4532-port-every-rigctld-client = 4532 is de poort die elke rigctld-client aanneemt
settings-servers-rig-name = Rignaam
settings-servers-reported-clients = gemeld aan clients
settings-servers-max-clients = Max. clients
settings-servers-allow-clients-transmit = Clients toestaan te zenden
settings-servers-off-refuses-every-key-request = Uit weigert elk zendverzoek en stopt met het adverteren van een zendbereik, zodat Hamlib zelf weigert te zenden.
settings-servers-rigctld-has-no-authentication-0 = ⚠ rigctld heeft geen authenticatie. Op 0.0.0.0 kan iedereen die deze poort kan bereiken je zender afstemmen en inschakelen.
settings-servers-t-1-client = 1 client
settings-servers-not-running = Draait niet.
settings-servers-status-unknown-press-apply = Status onbekend — druk op TOEPASSEN.
settings-servers-persist-re-bind-server = Opslaan en de server (opnieuw) binden
settings-servers-lets-any-hamlib-capable-program = Laat elk Hamlib-geschikt programma deze radio bedienen: frequentie, modus, PTT, split, RIT en vermogen. Kies in WSJT-X, fldigi of CQRLOG de rig “Hamlib NET rigctl” (model 2) en wijs hem naar dit adres; voer in GPredict en N1MM de host en poort direct in. Anders dan de TCI-server draagt hij alleen besturing — geen audio, geen IQ.
settings-servers-antenna-rotator = Antennerotor
settings-servers-waiting-station-s-rotator-configuration = Wachten op de rotorconfiguratie van het station…
settings-servers-host = Host
settings-servers-min-elevation = Min. elevatie
settings-servers-below-rotator-parks-instead-tracking = Daaronder parkeert de rotor in plaats van te volgen — stel het in op je lokale horizon of daklijn
settings-servers-azimuth-offset = Azimutoffset
settings-servers-added-every-command-rotator-whose = Opgeteld bij elk commando, voor een rotor waarvan het noorden niet klopt
settings-servers-min-movement = Min. beweging
settings-servers-steps-smaller-than-are-not = Stappen kleiner dan dit worden niet verzonden — tienden van een graad najagen slijt motoren voor niets
settings-servers-park-position = Parkeerpositie
settings-servers-drive-position-when-idle = naar een positie rijden bij inactiviteit
settings-servers-off-leaves-antenna-wherever-last = Uit laat de antenne staan waar de laatste passage eindigde
settings-servers-not-connected = Niet verbonden.
settings-servers-waiting-station-s-tci-server = Wachten op de TCI-serverconfiguratie van het station…
settings-servers-built-tci-server = Ingebouwde TCI-server
settings-servers-device-name = Apparaatnaam
settings-servers-off-leaves-control-receive-streams = Uit laat besturing en de ontvangststromen werken, maar elk zendverzoek wordt geweigerd.
settings-servers-tci-has-no-authentication-0 = ⚠ TCI heeft geen authenticatie. Op 0.0.0.0 kan iedereen die deze poort kan bereiken je zender afstemmen en inschakelen.

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = Satellieten: elementensets en frequenties
settings-tle-waiting-station-s-satellite-configuration = Wachten op de satellietconfiguratie van het station…
settings-tle-tracker-already-fetches-celestrak-s = De tracker haalt de amateurgroep van CelesTrak al zelf op. Dit is voor al het andere: de NOAA-weersatellieten, een cubesat die te nieuw is voor de groep, of een verser elementenset dan degene die binnenkwam. De lijsten worden opgehaald — en bewaard — waar de radio-engine draait, dus dit is dezelfde set satellieten waar de app ook open is.
settings-tle-subscriptions = Abonnementen
settings-tle-listings-fetched-kept-current-same = Lijsten worden opgehaald en actueel gehouden, op hetzelfde zesuurlijkse ritme als de amateurset. Ververst terwijl het zonnevenster open is, en door NU BIJWERKEN hier.
settings-tle-fetch-track-listing = Deze lijst ophalen en volgen
settings-tle-remove-subscription = Dit abonnement verwijderen
settings-tle-orbits = Banen
settings-tle-which-satellites-listing-get-orbit = Welke satellieten in deze lijst een baanring en een label krijgen. Een hele groep wil "curated": negentig ringen tegelijk is onleesbaar, en geen enkele laat negentig anonieme stippen over.
settings-tle-nothing-listing-sdroxide-s-curated = Niets in deze lijst staat in de samengestelde lijst van sdroxide — die lijst is tien amateursatellieten, dus dit zou zich precies gedragen als "none".
settings-tle-all-satellites = alle satellieten
settings-tle-catalogue-numbers-keep-comma-separated = Catalogusnummers om te behouden, gescheiden door komma's. Leeg volgt alles wat de lijst bevat.
settings-tle-not-fetched-yet = nog niet opgehaald
settings-tle-subscription = + Abonnement
settings-tle-update-now = {" "}NU BIJWERKEN{" "}
settings-tle-ask-radio-engine-fetch-every = Vraag de radio-engine om nu elk ingeschakeld abonnement op te halen
settings-tle-celestrak-groups = CelesTrak-groepen
settings-tle-pasted-element-sets = Geplakte elementensets
settings-tle-one-off-these-do-not = Voor eenmalig gebruik. Deze werken zichzelf niet bij, en SGP4 stopt met propageren van een elementenset zodra die twee weken na zijn epoch is — abonneer je in plaats daarvan voor alles wat je wilt houden.
settings-tle-track-one = Deze volgen
settings-tle-show-two-element-lines = De twee elementenregels tonen
settings-tle-remove = Verwijderen
settings-tle-paste-two-three-line-element = Plak hier elementensets van twee of drie regels
settings-tle-add-pasted = + Geplakte toevoegen
settings-tle-nothing-paste-box-looked-like = Niets in het plakvak leek op een elementenset.
settings-tle-clear-box = Vak wissen
settings-tle-frequencies = Frequenties
settings-tle-shown-under-pass-table-solar = Getoond onder de passagetabel in het zonnevenster. Een item hier vervangt het ingebouwde voor dat catalogusnummer volledig, dus begin met een kopie ervan tenzij je de rest wilt laten vallen.
settings-tle-remove-satellite-s-entry = Het item van deze satelliet verwijderen
settings-tle-link = LINK
settings-tle-downlink = DOWNLINK
settings-tle-uplink = UPLINK
settings-tle-mode = MODUS
settings-tle-note = OPMERKING
settings-tle-fm-repeater = FM-repeater
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67.0 Hz
settings-tle-link-2 = + Link
settings-tle-copy-built = Ingebouwde kopiëren
settings-tle-frequency-either-one-number-145 = Een frequentie is ofwel één getal (145.800) of een transponderdoorlaatband geschreven als 145.950-145.970. Laat een richting leeg voor een baken.
settings-tle-norad = NORAD
settings-tle-satellite = + Satelliet
settings-tle-frequency-entry-needs-satellite-s = Een frequentie-item heeft het NORAD-catalogusnummer van de satelliet nodig.

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = Adres
settings-wavelog-api-key = API-sleutel
settings-wavelog-your-wavelog-s-front-page = Het adres van de voorpagina van je Wavelog — https://log.example.org of http://192.168.1.10/wavelog — en een sleutel van de API-pagina. Voor uploaden is een lees-/schrijfsleutel nodig.
settings-wavelog-plain-http-address-internet-api = Gewone http:// naar een adres op internet: de API-sleutel wordt onversleuteld verstuurd. Prima binnen je eigen netwerk; gebruik https:// via internet.
settings-wavelog-trusted-cert = Vertrouwd cert.
settings-wavelog-forget-certificate = Dit certificaat vergeten
settings-wavelog-go-back-ordinary-checking-against = Terug naar de gewone controle tegen de openbare certificeringsinstanties — voor een server die nu een correct certificaat heeft.
settings-wavelog-connection-check-runs-machine-radio = De verbindingstest draait op de machine waarop de radio is aangesloten — gebruik daar de instellingen om te testen en de stationslocaties op te halen.
settings-wavelog-server-presented-certificate-sha-256 = De server presenteerde dit certificaat (SHA-256):
settings-wavelog-compare-your-server-s-e = Vergelijk het met dat van je server — bijv. `openssl x509 -noout -fingerprint -sha256 -in cert.pem` — en vertrouw het alleen als ze overeenkomen. Vanaf dat moment wordt precies dit certificaat geaccepteerd en elk ander geweigerd.
settings-wavelog-trust-certificate = Dit certificaat vertrouwen
settings-wavelog-station-location = Stationslocatie
settings-wavelog-none = — geen —
settings-wavelog-press-test-above-list-station = Druk hierboven op Test om de in je Wavelog gedefinieerde stationslocaties op te halen.
settings-wavelog-default = Standaard
settings-wavelog-where-qsos-go-when-operating = Waar QSO's naartoe gaan als het gebruiksprofiel geen locatie noemt.
settings-wavelog-profile = Dit profiel
settings-wavelog-use-default = Standaard gebruiken
settings-wavelog-not-set = niet ingesteld
settings-wavelog-no-station-location-chosen-uploads = Geen stationslocatie gekozen — uploaden mislukt totdat er een is gekozen.
settings-wavelog-qsos-not-wavelog = QSO's niet in Wavelog
settings-wavelog-every-qso-logbook-wavelog = Elke QSO in het logboek staat in Wavelog.
settings-wavelog-everything-logbook-wavelog-has-not = Alles in het logboek wat Wavelog niet heeft bevestigd — inclusief contacten van vóór het automatisch uploaden. Wat het al heeft, komt terug als dubbel en wordt gewoon als verzonden gemarkeerd.
settings-wavelog-set-address-key-station-location = Stel eerst het adres, de sleutel en een stationslocatie in.
settings-wavelog-yes-send-them = Ja, verzenden
settings-wavelog-cancel = Annuleren
settings-wavelog-download-into-logbook = Downloaden naar dit logboek
settings-wavelog-downloading-runs-where-radio-attached = Downloaden gebeurt op de machine waarop de radio is aangesloten, in het logboek van die machine — stel het daar in.
settings-wavelog-download-qsos-start-every = QSO's downloaden bij het starten en elke
settings-wavelog-downloaded-qsos-join-logbook-so = Gedownloade QSO's komen in het logboek, zodat FT8 en de spotlijsten weten wat je met andere programma's en radio's hebt gewerkt. Elke download haalt alleen op wat nieuw is.
settings-wavelog-press-test-above-choose-which = Druk hierboven op Test om te kiezen welke stationslocaties worden gedownload.
settings-wavelog-sync-now = Nu synchroniseren
settings-wavelog-fetch-what-wavelog-has-added = Haal op wat Wavelog sinds de laatste download heeft toegevoegd. Past eerst de instellingen hierboven toe.
settings-wavelog-full-resync = Volledige hersynchronisatie…
settings-wavelog-download-ticked-locations-whole-compare = Download de aangevinkte locaties volledig en vergelijk ze met het log: QSO's die in Wavelog zijn gewijzigd of verwijderd, of afkomstig zijn van een niet meer aangevinkte locatie, worden gelijkgetrokken. Je ziet de wijzigingen voordat er iets gebeurt.
settings-wavelog-full-resync-changes-make = Volledige hersynchronisatie — door te voeren wijzigingen
settings-wavelog-qsos-logged-here-imported-file = QSO's die hier zijn gelogd of uit een bestand zijn geïmporteerd, worden nooit gewijzigd of verwijderd — alleen de QSO's die uit Wavelog zijn gedownload.
settings-wavelog-apply = Toepassen
settings-wavelog-discard = Verwerpen
settings-wavelog-live-radio-status = Live radiostatus
settings-wavelog-send-frequency-mode-wavelog-they = Frequentie en mode naar Wavelog sturen zodra ze veranderen
settings-wavelog-radio-name = Radionaam
settings-wavelog-pick-radio-wavelog-s-live = Kies deze radio in het live loggen van Wavelog en het formulier volgt de afstemming. Vereist een lees-/schrijfsleutel. Alleen van de hoofdradio van het station.

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = Servercertificaat SHA-256: { $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = Zet het station terug op “{ $name }”: afstemming, VFO's, mode, filters, versterkingen, stuurvermogen, antennes, identiteit en bandstacks.
settings-profiles-delete = “{ $name }” verwijderen

## Settings window, T/R switch: messages with values

settings-relay-not-usable = Nog niet bruikbaar: { $why }
settings-relay-receiving = ontvangen
settings-relay-rx-tx-words = { $rx } / TX { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n ->
        [one] { $n } client
       *[other] { $n } clients
    }
settings-servers-listening = ● Luistert op { $addr } — { $clients }
settings-servers-failed = Mislukt: { $error }
settings-servers-rotator-at = ● Verbonden — antenne op { $az }° az, { $el }° el

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } satellieten · { $age } oud
settings-tle-subscribed = Geabonneerd op { $name }. Druk op NU BIJWERKEN om op te halen.
settings-tle-norad-too-stale = NORAD { $norad } · { $age } oud — te verouderd om te propageren
settings-tle-norad-age = NORAD { $norad } · { $age } oud
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = { $n } elementenset { $n ->
        [one] toegevoegd
       *[other] toegevoegd
    }.
settings-tle-added-refreshed = { $added } toegevoegd en { $r } elementenset { $r ->
        [one] vernieuwd
       *[other] vernieuwd
    }.
settings-tle-links = { $n } { $n ->
        [one] link
       *[other] links
    }
settings-tle-replace-links = Deze links vervangen door de ingebouwde voor { $name }

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = Locatie "{ $location }" is { $station_call }, maar je logt als { $call } — Wavelog weigert deze QSO's.
settings-wavelog-grid-mismatch = Locatie "{ $location }" ligt in { $station_grid }, maar jouw locator is { $grid } — Wavelog weigert deze QSO's.
settings-wavelog-location-n = locatie #{ $id }
settings-wavelog-saved-with-callsign = Opgeslagen met de roepletters ({ $call }) in Instellingen → Profielen, zodat een portable profiel naar een portable locatie kan loggen.
settings-wavelog-owed = { $n } { $n ->
        [one] QSO
       *[other] QSO's
    } gelogd sinds automatisch uploaden is ingeschakeld, nog te verzenden — elke 5 minuten opnieuw geprobeerd.
settings-wavelog-send-unsent = { $n } { $n ->
        [one] QSO
       *[other] QSO's
    } die nog niet in Wavelog { $n ->
        [one] staat
       *[other] staan
    } verzenden…
settings-wavelog-send-confirm = { $n } QSO's naar { $location } verzenden? Eerst worden de instellingen toegepast.
settings-wavelog-plan-add = • { $n } QSO's in Wavelog om aan het log toe te voegen
settings-wavelog-plan-update = • { $n } gedownloade QSO's die in Wavelog zijn gewijzigd, om bij te werken
settings-wavelog-plan-remove = • { $n } gedownloade QSO's die niet meer in Wavelog staan (of van een niet-aangevinkte locatie zijn), om te verwijderen
settings-wavelog-plan-mark = • { $n } van je eigen QSO's om als aanwezig in Wavelog te markeren

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = Antenne
settings-radio-which-socket-back-radio-using =
    Welke aansluiting aan de achterkant de radio gebruikt — zijn eigen ANT-commando, dezelfde instelling als de ANT-knop op het voorpaneel.

    Wordt direct toegepast en is de instelling van de radio zelf, geen hier bewaarde kopie: de aansluiting waarop de radio staat wordt teruggelezen wanneer de sessie opent.

    De keuze wordt per band onthouden. Schakel op 2 m naar de beam en op 40 naar de draad, en elk komt terug zodra de afstemming weer die band in gaat — en ook de volgende keer dat sdroxide start. Een hier opgeslagen geheugenkanaal bevat ook zijn aansluiting.
settings-radio-receive-antenna = Ontvangstantenne
settings-radio-radio-s-separate-receiving-antenna =
    De aparte ontvangstantenne van de radio — RX ANT aan de achterkant — in of uit het ontvangstpad geschakeld. Het is een eigen instelling, dezelfde als de RX ANT-knop van de radio.

    De antenne op de hoofdaansluiting blijft in beide gevallen op zenden: dit is een extra ingang voor ontvangst, een loop of een beverage, geen keuze van aansluiting.

    Anders dan bij de antenne hierboven wordt dit hier *niet* onthouden. De radio bewaart het zelf per band, dus sdroxide leest het na elke bandwissel terug en toont wat de radio meldt; alleen dit vakje aanvinken verandert het.
settings-radio-receive-path = In het ontvangstpad
settings-radio-radio-power = Radio-voeding
settings-radio-switch-radio-itself-off-back =
    Schakel de radio zelf uit en weer in via de besturingsverbinding. Dit is de enige echte aan/uit in het programma — niet de eigen LINK-schakelaar van sdroxide, die het uiteinde van sdroxide sluit en de radio laat doorlopen.

    Om de radio weer aan te kunnen zetten moet het besturingsuiteinde van de radio wakker blijven terwijl hij uit staat. Via het netwerk doet Network Control dat. Via een seriële kabel is dat de CI-V-poort van de radio, die voeding houdt vanuit het lichtnet bij een toestel dat op het voorpaneel is uitgeschakeld in plaats van losgekoppeld; sdroxide stuurt de wekreeks die de documentatie van Icom zelf vraagt vóór het inschakelcommando.

    Uitschakelen beëindigt het geluid en de meters — er is geen radio meer achter — en sdroxide houdt de besturingsverbinding open, zodat het weer inschakelen een bestemming heeft.
settings-radio-on = Aan
settings-radio-off = Uit
settings-radio-waiting-configuration-machine-radio-attached = Wachten op de configuratie van de machine waarop de radio is aangesloten.
settings-radio-sound-format = Geluidsformaat
settings-radio-invert-spectrum = Spectrum omkeren
settings-radio-swap-i-q = I/Q omwisselen
settings-radio-mirror-panadapter-about-tuned-frequency =
    Spiegel de panadapter om de afgestemde frequentie, voor een radio die I en Q op de geluidskaart andersom aanlevert. Het verraad is een waterval vol overtuigende signalen die allemaal aan de verkeerde kant van de afstemming staan, waarbij SSB op de tegenovergestelde zijband uitkomt — de twee kabels op de geluidskaart omwisselen zou het net zo goed oplossen.

    Alleen ontvangst: zenden geeft de radio één reëel audiosignaal, dat geen zijband heeft om om te keren.
settings-radio-i-q-sample-rate = I/Q-samplefrequentie
settings-radio-how-fast-radio-s-i =
    Hoe snel de I/Q-geluidskaart van de radio draait — en dus hoeveel band de panadapter toont, want een quadratuurstroom beslaat zijn hele samplefrequentie: 48 kHz geeft ±24 kHz aan weerszijden van de afstemming, 192 kHz geeft ±96.

    De kaart heeft het laatste woord. Een kaart die de hier gekozen frequentie niet aankan, wordt geopend op de dichtstbijzijnde die hij wel kan, en de panadapter is dan zoveel smaller — het log meldt dit bij het opstarten als het gebeurt.

    Een snellere kaart is ook meer werk: de machine heeft proportioneel minder tijd om hem leeg te maken, en een machine die het niet bijhoudt, laat samples vallen. Dat merk je aan wegvallend geluid terwijl de waterval er nog perfect uitziet, en het wordt ook gelogd. Zie je dat, ga dan een stap terug.
settings-radio-t-48-khz-24-khz = 48 kHz (±24 kHz)
settings-radio-t-96-khz-48-khz = 96 kHz (±48 kHz)
settings-radio-t-192-khz-96-khz = 192 kHz (±96 kHz)
settings-radio-t-384-khz-192-khz = 384 kHz (±192 kHz)
settings-radio-i-q-centre-offset = I/Q-middenoffset
settings-radio-how-far-radio-s-i =
    Hoe ver de I/Q-uitgang van de radio boven zijn eigen afstemming is gecentreerd, voor een radio waarvan de ontvangst-F.I. van nul is verschoven — bij een Elecraft: MENU:RX SHFT ingesteld op 8.0 in plaats van NOR, wat de afstemming van de DC-piek van de mixer afhaalt (en voorkomt dat een sterk nabij SSB/AM-station AM-gedetecteerd wordt).

    Geen converter: de radio toont en zendt nog steeds op de echte frequentie, en er wordt hier nooit iets naartoe gestuurd. Dit geeft alleen aan waar de samples op de geluidskaart zitten, en de stroom wordt bij binnenkomst weer naar de afstemming terugverschoven.

    Laat op 0 staan tenzij je zo'n menu-item hebt ingeschakeld. Als signalen op tweemaal de offset afstand verschijnen, is het teken omgekeerd.
settings-radio-iq-correction = IQ-correctie
settings-radio-cancel-mirror-image-every-signal =
    Onderdruk het spiegelbeeld van elk signaal en de DC-piek in het midden van de waterval.

    De I/Q-uitgang van een radio bestaat uit twee analoge paden die nooit helemaal gelijk zijn in versterking of precies 90° uit fase zijn, en wat dat achterlaat is een kopie van elk signaal, gespiegeld om de afgestemde frequentie, meestal 30-40 dB lager — sterk genoeg om op een station te lijken dat er niet is, en als zodanig gedecodeerd te worden. Radio's met balanstrimmers op het voorpaneel regelen precies dit; hier wordt het gemeten aan de ontvangen ruis en is geen instelling nodig.

    Laat het aan staan. Zet het uit als je naar AM luistert, precies op de draaggolf afgestemd — de draaggolf is DC, dus verdwijnt hij samen met de piek — of om te controleren of een signaal echt is door te kijken of het blijft bestaan.
settings-radio-cancel-mirror-images = Spiegelbeelden onderdrukken
settings-radio-dc-notch = DC-notch
settings-radio-widen-hole-taken-out-middle =
    Maak het gat in het midden van de span breder, voor een radio waarvan de middenpiek breder is dan de offset eronder.

    0 laat de gewone blokkering staan, die enkele tientallen hertz breed is en de offset verwijdert zonder iets anders aan te raken. Draai hem op en de onderkant van de span gaat mee: een eerste-orde hoogdoorlaat, 3 dB lager bij de hier ingestelde waarde en verder omlaag eronder.

    Hij is gecentreerd waar de I/Q van de radio is gecentreerd, wat de afstemming is tenzij de offset hierboven die heeft verschoven — dus alles wat daar is afgestemd verdwijnt ook. Een CW-toon van 600 Hz valt binnen een instelling van 600 Hz.
settings-radio-panadapter-bw = Panadapter-BW
settings-radio-cat-family = CAT-familie
settings-radio-rigctld-address = rigctld-adres
settings-radio-host-port-running-hamlib-rigctld =
    host:poort van een draaiende Hamlib rigctld — 127.0.0.1:4532 is de eigen standaard, op deze machine.

    Start er een met bijvoorbeeld `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, waarbij -m het Hamlib-modelnummer van je radio is (`rigctl -l` toont ze).

    Dit is de vangnetoptie: het bereikt de frequentie, mode, PTT, vermogen, S-meter en SWR en niets anders. Waar een van de native families hierboven bij je radio past, doet die meer — keyen vanuit de eigen tekstbuffer van de radio, het ontvangstfilter en meterschalen per model.
settings-radio-flrig-address = flrig-adres
settings-radio-host-port-running-flrig-127 =
    host:poort van een draaiende flrig — 127.0.0.1:12345 is de eigen standaard, op deze machine. flrig biedt XML-RPC aan zolang het draait; de poort staat onder Config → Setup → Server.

    Net als de Hamlib-optie stuurt dit een daemon aan in plaats van de radio, maar via het eigen per-model stuurprogramma van flrig — bij een aantal radio's is de afhandeling van vermogen en filter daarvan de getrouwere van de twee. Het bereikt de frequentie, mode, PTT, zendvermogen (in hele watts), de ontvangstbandbreedte, de S-meter, SWR en uitgangsvermogen. CW wordt gekeyd via de eigen cwio-poort van flrig, die in flrig zelf moet worden ingesteld.
settings-radio-serial-port = Seriële poort
settings-radio-select = — kies —
settings-radio-baud = Baud
settings-radio-data-bits = Databits
settings-radio-parity = Pariteit
settings-radio-stop-bits = Stopbits
settings-radio-force-rts = RTS forceren
settings-radio-force-dtr = DTR forceren
settings-radio-ptt-method = PTT-methode
settings-radio-how-transmit-keyed = Hoe het zenden wordt gekeyd.
settings-radio-how-transmit-keyed-network-link = Hoe het zenden wordt gekeyd. Een netwerkverbinding heeft geen stuurlijnen, dus DTR en RTS keyen hier niets — gebruik CAT, waarmee de daemon de radio keyt, of VOX.
settings-radio-mode-control = Modebesturing
settings-radio-digimode-mode = Digimode-mode
settings-radio-what-put-radio-mode-sdroxide =
    Waarop de radio moet worden gezet voor een mode die sdroxide via zijn geluidskaart moduleert. "DIGI" kiest de DATA/PKT-stand van de radio, die de zendaudio van de USB- of ACC-ingang neemt zonder de spraakbewerking van het microfoonpad; "USB" laat hem op de gewone zijband, voor een radio die al is ingesteld om SSB te moduleren vanaf zijn data-ingang of een zonder DATA-stand.

    Het geldt voor elke digitale mode — FT8, FT4, PSK, RTTY, SSTV en de rest — en voor CW verzonden als "Geluidskaart", en het gaat voor hen boven Modebesturing. SSTV is de mode waarvan de zijband de band volgt, dus daar betekent het de DATA-stand op die zijband: USB-D boven 40 m, LSB-D op en onder 40 m. SSTV-FM valt er niet onder — een FM-draaggolf heeft geen zijband om uit te kiezen.
settings-radio-cw-keying = CW-keying
settings-radio-how-cw-panel-s-keyer =
    Hoe de keyer van het CW-paneel zendt. "Radio-keyer" zet de radio in CW en geeft hem de te verzenden tekst voor zijn eigen keyer. Het gebruikt de keyersnelheid van de radio (ingesteld vanuit de WPM van het paneel), en bij Yaesu wordt verzonden via keyergeheugen 1, waarbij wat daarin stond wordt overschreven.

    Het vereist dat break-in aan staat op de radio, wat sdroxide bij elk bericht zelf inschakelt bij Icom, Yaesu en Kenwood — een radio met break-in uit neemt het bericht aan en zendt niets. Daarna blijft het aan, en een volledige break-in wordt nooit teruggezet naar semi.

    "Geluidskaart" verzendt de gekeyde sidetone als audio (MCW), een toon op afstemming + toonhoogte — en omdat een radio in CW zijn geluidskaart helemaal zou negeren, volgt het kiezen van CW dan de instelling Digimode-mode (USB, DATA of Radio-gestuurd) in plaats van de radio op CW te zetten. Kies daar "Radio-gestuurd" voor radio's waarvan de datastand niet via CAT kan worden bediend (de U-D van een Xiegu) en zet de radio er zelf op, zoals bij FT8.
settings-radio-poll-rate = Pollsnelheid
settings-radio-how-often-radio-asked-what =
    Hoe vaak de radio wordt gevraagd wat hij doet — zijn afstemming, zijn mode en zijn meters. Dit is de helft van de besturingsverbinding die van de radio terug naar sdroxide loopt: draai aan de VFO-knop van de radio of verander zijn mode op het voorpaneel en de uitlezing, de band en de panadapter volgen binnen één poll.

    Het is ook het hele besturingsverkeer dat dit uiteinde genereert, en bij een moderne Icom is dat verkeer niet gratis: de CI-V-poort en de geluidskaart zitten achter één USB-hub in de radio, dus frames die hier worden opgevraagd zijn businstijd die het geluid niet krijgt — hoorbaar als uitval die op een DSP-fout lijkt en dat niet is. De standaard van 2 Hz loopt een halve seconde achter op de knop en is rustig genoeg om het geluid niet in de weg te zitten. Verhoog hem waar de besturingspoort een eigen apparaat is (een aparte USB-serieel-adapter, een netwerk-rigctld); verlaag hem als het geluid van de radio wegvalt.

    Een Icom met CI-V Transceive ingeschakeld in zijn eigen menu heeft hier niets van nodig voor de afstemming: hij meldt de knop op het moment dat die beweegt, en sdroxide zet de afstemmingspoll uit zodra het dat ziet — en pakt hem weer op als de radio ooit beweegt zonder het te melden.

    De snelheid is die van de afstemming. De mode gaat slechts bij elke vierde poll mee, want het is een instelling die een paar keer per avond verandert en niet een die een knop volgt.
settings-radio-send-command = Commando verzenden
settings-radio-which-transceiver-generation-keys-rig =
    Welke transceivergeneratie de radio keyt, voor PTT-methode "CAT". De twee verschillen over wat de TX-parameter betekent en niets op de lijn onderscheidt ze, dus kies degene die bij je radio past.

    "TS-2000-stijl (TX;)" — TS-480, TS-570, TS-870, TS-2000: het gewone zenden, op de hoofdband.

    "TS-590-stijl (TX1;)" — TS-590S/SG, TS-890, TS-990: DATA SEND, dat keyt met de ACC2/USB-audio-ingang actief. Op deze radio's kiest het gewone zenden juist de microfoon en dempt het de audio die sdroxide zendt.

    Verkeerd ingesteld zendt een TS-590 stilte — maar een TS-2000 zendt op de subband, wat een heel andere band is.
settings-radio-radio = Radio
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    Eén profiel dekt de familie: de K3-commandoset, waarop de KX3, KX2 en K4 allemaal antwoorden. Hier valt niets te kiezen — hoeveel watt de Drive-schuif beslaat (12 op een kale KX2 of KX3, 110 met een KPA3 of een KXPA100) wordt uit de optiemodule-opvraag van de radio zelf gelezen wanneer de poort opent.

    Let op de baudrates hierboven: de K3, K3S, KX3 en KX2 gaan niet sneller dan 38400, en een radio die lager is ingesteld dan de hier gekozen snelheid antwoordt helemaal niets.
settings-radio-transmit-input = Zendingang
settings-radio-where-radio-takes-transmit-audio =
    Waar de radio zijn zendaudio vandaan haalt — het TI-commando van de radio, dat menu 32 "TX IN" op het voorpaneel is. Wordt bevestigd wanneer de poort opent.

    "USB-audio" is wat deze interface laat werken: de FDM-DUO zendt wat sdroxide in zijn USB-geluidskaart stopt. Een radio die op "Microfoon" blijft staan, zendt in plaats daarvan de kamer, en niets op het scherm zegt dat.

    "Auto" laat de radio kiezen — de microfoon bij een PTT-druk op de microfoon, de USB-poort bij een CAT- of RTS-keydown.

    "Laten zoals ingesteld op de radio" stuurt helemaal geen TI, voor een operator die dit op het voorpaneel instelt.
settings-radio-which-two-sockets-back-receiver =
    Op welke van de twee aansluitingen aan de achterkant de ontvanger luistert — het AN-commando van de radio, dat menu 31 "ANTENNAS" op het voorpaneel is en de "ANT 1 2"-indicator op het display.

    "RTX" is één antenne voor beide taken, op de M-aansluiting die ook het zenden draagt. "Alleen RX" verplaatst de ontvangst naar de tweede aansluiting en laat het zenden op RTX — een ontvangstantenne, een loop of een beverage, met de beam nog op de zender.

    Wordt direct toegepast en wordt teruggelezen van de radio wanneer de poort opent: dit is de instelling van de radio zelf, geen hier bewaarde kopie.
settings-radio-one-profile-covers-both-there =
    Eén profiel dekt beide. Hier valt niets te kiezen — de radio meldt zijn eigen naam wanneer de poort opent.

    Dit is alleen de CAT-helft: het bedient de afstemming, de mode, PTT, de S-meter, de SWR en het zendvermogen via de CAT-USB-poort van de radio, en neemt audio van zijn USB-audiopoort. De derde USB-poort van de FDM-DUO draagt breedband-I/Q, dat deze interface niet leest — kies daarvoor hierboven de ELAD-interface.

    Let op de baudrate: menu 70 "CAT BAUD" op de radio moet overeenkomen, en staat af fabriek op 38400.

    CW wordt gekeyd met de eigen sleutel of paddle van de radio. De FDM-DUO heeft geen commando dat tekst accepteert, dus het CW-paneel kan hem niet via CAT keyen.
settings-radio-one-profile-covers-range-qrp =
    Eén profiel dekt het assortiment: de eigen commandoset van QRP Labs, die een subset is van die van de Kenwood TS-480 met er flink wat bij. Hier valt niets te kiezen — de radio meldt zijn eigen naam en firmwareversie wanneer de poort opent, en de versie bepaalt of de SWR-beveiligingsuitlezing überhaupt wordt opgevraagd.

    Niet het Kenwood-profiel: op een QMX is het PC-commando de vermogensMETER en niet de vermogensregeling, dus een als Kenwood aangestuurde radio zou zijn meteruitlezing beschreven krijgen alsof het een instelling was, en MD8 — een Kenwood-mode — is hier SWR Tune.

    Het zendvermogen wordt op de radio ingesteld. Er is geen CAT-commando voor, dus de Drive-schuif bereikt alleen het niveau van de audio die de geluidskaart in gaat.

    Het ontvangstfilter is ook van de radio: hij meldt de breedte die zijn mode impliceert (3,2 kHz in Digi, 300 Hz in CW) en biedt niets om die te wijzigen.
settings-radio-baud-rate-ignored-port-usb = Baudrate wordt genegeerd — de poort is USB
settings-radio-qmx-serves-own-virtual-com =
    Een QMX biedt zijn eigen virtuele COM-poorten via USB aan, dus de hierboven ingestelde snelheid heeft aan geen van beide kanten effect. Hij biedt er maximaal drie aan; als de radio niets antwoordt, is de verkeerde de meest waarschijnlijke reden.

    Eén ding doet er wel toe bij de poort: een carriage return die ernaartoe wordt gestuurd, zet de radio voor de rest van de sessie in terminalmodus. sdroxide stuurt er nooit een — maar een terminalprogramma dat op dezelfde poort open blijft staan wel, en CAT werkt niet meer zodra dat gebeurt.
settings-radio-i-q-mode-switched-radio = I/Q-mode wordt op de radio ingeschakeld
settings-radio-radio-s-sound-card-carries =
    De geluidskaart van de radio draagt ofwel gedemoduleerde audio ofwel de ruwe I/Q die zijn ADC ziet, en sdroxide stelt in wat het Geluidsformaat hierboven vraagt wanneer de poort opent (het Q9-commando van de radio, het menu-item "IQ mode").

    De middenoffset en samplefrequentie hierboven zijn ingesteld op wat een QMX nodig heeft: zijn ontvanger is een superhet met een F.I. van 12 kHz, dus de synthesizer zit 12 kHz onder de afstemming en alles op de kaart zit 12 kHz boven het midden van de span. De kaart draait op 48 kHz, waardoor de panadapter 48 kHz breed is.

    In CW verschuift de radio die F.I. nog eens ~700 Hz, wat één getal niet kan volgen — tel het er met de hand bij op als je de panadapter gebruikt met de radio in CW. In Digi, waar een als I/Q-frontend gebruikte radio normaal staat, valt niets op te tellen.

    QRP Labs merkt op dat de I/Q-mode niet geschikt is voor WSJT-X en andere programma's die gedemoduleerde audio verwachten; hier gebeurt de demodulatie aan deze kant, dus het werkt wel.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ (5 W HF-transceiver)
settings-radio-hobbypcb-s-rs-hfiq-there =
    De RS-HFIQ van HobbyPCB. Hier valt niets te kiezen: het profiel is de volledige commandoset van de radio, en de firmwareversie wordt gelogd wanneer de poort opent.

    De besturingsverbinding draagt twee dingen en niet meer — waar de oscillator moet staan en of er gezonden wordt. Er is geen mode-, vermogens-, filter-, squelch- of metercommando, want de radio heeft ze niet. Al het andere is van sdroxide: wat uit de geluidskaart komt is complexe basisband gecentreerd op de afstemming, dus de mode, het filter en de modulatie gebeuren allemaal aan deze kant.

    Het frequentiecommando dekt 3–30 MHz. Vraag je iets daarbuiten, dan weigert de radio het en blijft waar hij is; het log meldt dat.

    CW wordt als audio gekeyd via de zendketen — deze radio heeft geen keyer die tekst accepteert. (Hij heeft een interne CW-generator; sdroxide gebruikt die nooit, op advies van de firmware zelf.)
settings-radio-sound-format-baud-ptt-are = Geluidsformaat, baud en PTT zijn die van de radio, geen instellingen
settings-radio-all-three-have-been-filled =
    Alle drie zijn ingevuld en geen ervan is een voorkeur:

    • Geluidsformaat is I/Q (stereo) — de kaart draagt complexe basisband en niets anders.
    • 57600 baud, 8N1 — de ene snelheid van de firmware, zonder menu om die te wijzigen. Elke andere is een stille verbinding, dus het wordt vastgezet wanneer de poort opent.
    • PTT via CAT — het *X-commando is de enige zendschakelaar die de interface heeft.

    De middenoffset is op nul gezet: de synthesizer draait op viermaal de afstemming in een kwadratuurdetector, dus het midden van de span is de afstemming. Stel de samplefrequentie hierboven in op wat je geluidskaart werkelijk draait; dat bepaalt hoe breed de panadapter is.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    De pocket-QRP-transceiver van DL2MAN/PE1NNZ en de open uSDX-firmware waaruit hij is voortgekomen. Hij emuleert een Kenwood TS-480, maar de subset is dun: de afstemming, de mode, PTT en RIT/XIT wissen is alles wat hij beantwoordt. Geen S-meter, geen SWR, geen vermogensregeling, geen VFO B, geen split en geen CAT-keyer, dus de Drive-schuif en de meters doen niets via CAT — het audioniveau is de enige zendregeling die er is.

    CW wordt als audio (MCW) via de zijband gekeyd, of met de sleutel of paddle op de radio; er is geen commando dat tekst accepteert.
settings-radio-audio = Audio
settings-radio-tr-usdx-has-no-sound =
    Een (tr)uSDX heeft geen eigen geluidskaart en twee manieren om gehoord te worden, en welke de juiste is, hangt af van je shack en niet van de radio.

    "Eén kabel" neemt de audio binnen de CAT-seriële verbinding zelf — de eigen 8-bits stroom van de radio, over dezelfde USB-kabel als de besturing, met niets anders om aan te sluiten. De adder onder het gras is de firmware: die kan geen CAT-commando aannemen terwijl zijn stroom loopt (een commando dat in de stroom wordt gestuurd pauzeert hem niet, het maakt hem kapot), dus deze mode pollt nooit. Afstemming en modewijzigingen worden verstuurd zodra je ze doet, waarbij elk de stroom even pauzeert en weer start, dus verwacht een korte onderbreking bij elke wijziging; er wordt niets verstuurd zolang je alleen luistert. De eigen afstemming en mode van de radio worden ook niet gevolgd — er is geen uitlezing om ze mee te volgen — dus bedien hem vanaf hier.

    "USB-geluidskaart" neemt de audio van een externe geluidskaart die op de 3,5 mm luidspreker-/microfoonaansluiting van de radio is aangesloten, en gedraagt zich als elke andere CAT-radio: de afstemmingspoll draait, dus de eigen knop en mode van de radio worden gevolgd. Kies de kaart hieronder onder Radio-audio.

    Beide modes openen de poort met DTR hoog: de DTR van de seriële adapter is verbonden met de reset van de radio, dus het openen van de poort start hem opnieuw op en de eerste seconde of twee is stil terwijl hij opstart. DTR wordt nooit gebruikt om te keyen.
settings-radio-radio-model = Radiomodel
settings-radio-which-icom-two-things-ci =
    Welke Icom, voor de twee dingen die CI-V niet op alle modellen hetzelfde doet.

    Het transceiveradres hieronder wordt hieruit ingevuld: elk model heeft een ander adres af fabriek, en een frame naar het verkeerde adres wordt gewoon genegeerd — een radio die niets antwoordt, zonder dat ergens een fout zegt waarom.

    Het bepaalt ook of sdroxide de DATA-mode kan kiezen. Op CI-V zijn USB en USB-DATA dezelfde modebyte en onderscheidt een apart commando ze; zonder dat gaat een beurt in een digitale mode via de microfooningang uit, met de spraakbewerking en het SSB-filter van de radio in het pad.

    "Overig" laat het adres aan jou over en stuurt helemaal geen DATA-modecommando.
settings-radio-radio-id-hex = Radio-ID (hex)
settings-radio-show-radio-s-spectrum-scope = Spectrumscope van de radio tonen
settings-radio-scope-span = Scopespan
settings-radio-how-wide-sweep-radio-keeps = Hoe breed hij moet vegen. De radio houdt de span aan die het laatst op zijn eigen scherm is gekozen — vaak een paar kHz. Een span hier instellen zet de scope ook in middenmodus, zodat hij de afstemming volgt. Het wijzigt ook het eigen display van de radio; "Zoals ingesteld op de radio" laat het met rust.
settings-radio-press-apply-reconnect-switch-without = Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-radio-audio-sound-card = Radio-audio (geluidskaart)
settings-radio-devices = Apparaten
settings-radio-discover = Zoeken
settings-radio-no-devices-press-discover = geen apparaten — druk op Zoeken
settings-radio-manual-ip = Handmatig IP
settings-radio-optional-e-g-192-168 = optioneel, bijv. 192.168.1.50
settings-radio-sample-rate = Samplefrequentie
settings-radio-receiver-ddc = Ontvanger (DDC)
settings-radio-protocol-2-board-carries-several = Een Protocol 2-bord draagt meerdere onafhankelijk af te stemmen ontvangers (DDC's) op één verbinding — draai deze radio op DDC1 en een andere radio, zelfde adres, op DDC2. De zender hoort bij de DDC1-radio. Protocol 1-borden hebben alleen DDC1.
settings-radio-lna-gain = LNA-versterking
settings-radio-front-end-gain-hermes-lite = Front-end-versterking van een Hermes-Lite 2. Werkt direct — zonder opnieuw verbinden — en wordt onthouden als het niveau waarop de radio start. Te hoog laat de ADC clippen en de hele band lijkt vervormd; te laag en de ontvanger wordt doof.
settings-radio-overload-protection = Overbelastingsbeveiliging
settings-radio-back-lna-gain-off-itself =
    Draai de LNA-versterking zelf terug zolang het bord meldt dat zijn ADC overstuurd is, en laat hem weer oplopen zodra het ophoudt.

    Een Hermes-Lite 2 samplet heel 0-38 MHz op één 12-bits converter zonder mixer en zonder preselector ervoor, dus een omroepstation een band verderop kan hem in overbelasting drijven terwijl de band waar je naar kijkt niets verkeerds laat zien — de ruisvloer stijgt, alles intermoduleert en de decoders stoppen. Het bord weet het, en dit is wat erop reageert.

    Standaard uit: het verplaatst een regelaar die jij hebt ingesteld. Tijdens zenden gebeurt er niets.
settings-radio-wind-gain-back-when-adc = De versterking terugdraaien als de ADC overstuurt
settings-radio-applies-apply-reconnect-main-window = Wordt toegepast bij Toepassen / opnieuw verbinden. De Gain-rail van het hoofdvenster leest de versterking waarop het bord werkelijk draait, dus die volgt de lus en je kunt zien wat hij doet; de schuif hierboven is het niveau waarop de radio start, en de lus herschrijft die niet.
settings-radio-how-far-gain-moves-each = Hoever de versterking telkens beweegt, in dB. Eén is de stap die het eigen versterkingsregister van het bord heeft.
settings-radio-attack-decay = {"  "}Attack / decay
settings-radio-how-often-gain-may-come =
    Hoe vaak de versterking omlaag mag terwijl de converter overstuurt, en hoe vaak hij weer omhoog mag zodra het is gestopt.

    De twee liggen bewust een factor honderd uit elkaar. Direct terugtrekken: elke milliseconde overbelasting is een ontvanger vol intermodulatie. Langzaam terugkeren: wat de oorzaak ook was — een buur die keyt, een omroeper die bij het vallen van de avond opkomt — is meestal niet verdwenen, en een lus die even snel herstelde als hij terugtrok zou de hele avond om de drempel heen oscilleren. 100 ms en 10 s per decibel is wat zowel PowerSDR's Auto S-Att als N1GP's HermesIntf hebben gebruikt.
settings-radio-range = {"  "}Bereik
settings-radio-lowest-highest-gain-loop-may = De laagste en hoogste versterking die de lus mag gebruiken, in dB. Het plafond voorkomt dat hij bepaalt hoe gevoelig je ontvanger moet zijn; de vloer is waar je zegt dat de overbelasting onder een bepaald punt iemand anders' probleem is en het antwoord een filter is, geen twintig decibel extra.
settings-radio-filter-board = Filterbord
settings-radio-accessory-board-hermes-lite-2 = Accessoirebord op de J16-header van de Hermes-Lite 2 (of de open-collector-uitgangen van elk ander openHPSDR-bord — Protocol 2 inbegrepen). "N2ADR" kiest één relais per band; "Alex / Hermes-bandcode" zet het bandnummer op uitgangen 1-4, wat het Alex-bord van een ANAN, een Zeus SDR, een HiQSDR en Quisk verwachten. "Aangepast" opent hieronder een tabel waarin je de zeven uitgangen zelf opgeeft, band voor band, bij ontvangst en bij zenden — voor een antenneschakelaar, een banddecoder van een eindversterker of iets anders waar geen van beide presets bij past; beide presets kunnen er als beginpunt in worden gegoten. Laat dit op "Geen" staan tenzij er echt iets is aangesloten: die zeven pinnen zijn algemene open-collector-uitgangen, en operators bedraden ze ook voor PTT van een eindversterker, antennerelais en transverterschakeling. Ze aansturen met banddata zou alles wat is aangesloten aan het werk zetten. Wordt toegepast bij Toepassen / opnieuw verbinden.
settings-radio-io-board-rx-input = RX-ingang IO-bord
settings-radio-where-n2adr-hl2ioboard-takes-receive = Waar een N2ADR HL2IOBoard zijn ontvangstsignaal vandaan haalt. Het bord zelf wordt automatisch gevonden en heeft geen instelling nodig; deze bestaat alleen voor operators die de eigen SMA-bussen hebben bedraad. Laat hem anders op "Eigen ingang van de radio" — J9 van het IO-bord kiezen terwijl er niets op is aangesloten, maakt de ontvanger doof. Wordt toegepast bij Toepassen / opnieuw verbinden. Bij een van beide J9-keuzes verschijnen de ingangen ook op de ANT-regeling en worden ze per band onthouden.
settings-radio-power-amplifier = Eindversterker
settings-radio-use-hermes-lite-2-s = De ingebouwde PA van de Hermes-Lite 2 gebruiken
settings-radio-default-what-you-want-unless = Standaard aan, en wat je wilt tenzij een externe versterker wordt aangestuurd vanuit de laagvermogen-RF1-uitgang van het bord. Met uit keyt de radio nog steeds — het T/R-relais schakelt en een accessoirebord volgt — maar de antenneaansluiting maakt helemaal geen vermogen, en het relais wordt bewust in ontvangst gehouden. Genegeerd op andere borden dan een Hermes-Lite.
settings-radio-mirror-board-s-spectrum-about = Spiegel het spectrum van het bord om de afgestemde frequentie, bij zenden en bij ontvangen. Standaard aan: een Hermes-Lite 2 heeft het nodig. Zet het alleen uit als signalen aan de verkeerde kant van de afstemming verschijnen en niets decodeert — het verraad is een waterval vol overtuigende sporen terwijl SSB op de verkeerde zijband landt en FT8 helemaal niets decodeert.
settings-radio-frequency-correction = Frequentiecorrectie
settings-radio-crystal-tcxo-error-ppm-applied = Kristal-/TCXO-fout in ppm, toegepast op RX en TX. Wordt direct toegepast.
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = Zendbuffer
settings-radio-how-far-ahead-real-time = Hoever vóór de echte tijd zendaudio via het netwerk naar het bord wordt gevoerd, voordat sdroxide vertraagt om erbij aan te sluiten. Het bord zelf heeft geen dergelijke buffer — dit vergroot alleen de eigen marge van sdroxide. Verhoog hem bij een WiFi-verbinding of VPN, waar de lage standaard (goed voor een directe bekabelde verbinding) niet genoeg speelruimte tegen jitter geeft en de gezonden audio of PTT hapert; hoger kost zendlatentie. Werkt na TOEPASSEN, waarbij opnieuw met het bord wordt verbonden.
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = Adaptieve predistortie: de zender lineariseren aan de hand van een monster van wat hij werkelijk uitzond, voor een twintigtal decibel minder intermodulatie bij hetzelfde vermogen. De ontvanger is het terugkoppelpad — het bord blijft ontvangen tijdens een beurt — dus een richtkoppelaar en een verzwakker moeten een monster van de uitgang van de versterker naar een ingang brengen die de T/R-schakelaar bij zenden niet wegneemt. Op een Hermes-Lite 2 betekent dat de PureSignal-bus van het IO-bord, met de ontvangstingang hierboven daarop afgestemd; op een bord zonder die bus de RX-poort van een ANAN of een extern relais dat de koppelaar voor de duur van de beurt inschakelt. Het is deze ontvanger die wordt uitgelezen — een bord met een tweede ADC kan RX2 niet als terugkoppelaftakking gebruiken, hoe zijn eigen firmware het ook routeert. Zonder gekoppeld signaal vergrendelt de lus nooit en blijft de zender precies zoals hij zou zijn geweest. Wordt toegepast bij Toepassen / opnieuw verbinden, en alleen op de radio die de zender bezit (DDC1).
settings-radio-correct-transmitter-receiver-s-own = De zender corrigeren met de eigen terugkoppeling van de ontvanger
settings-radio-how-finely-amplifier-s-curve = Hoe fijn de curve van de versterker wordt gemodelleerd. Meer stappen volgen een scherpere knik en kosten meer tijd om in te vullen; 32 is een verstandig begin.
settings-radio-how-fast-correction-follows-what = Hoe snel de correctie volgt wat de koppelaar meldt. Langzaam is goed — dit middelt de curve van een versterker, die niet beweegt, uit een terugkoppelpad waar ruis in zit.
settings-radio-hold = Vasthouden
settings-radio-stop-adapting-keep-correction-where = Stop met aanpassen en houd de correctie waar ze is — om te meten, en voor een operator die tevreden is met wat is geleerd.
settings-radio-manual-ip-overrides-discovery-press = Een handmatig IP gaat voor op zoeken. Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-open-collector-outputs-band = Open-collector-uitgangen per band
settings-radio-one-control-word-per-band = Eén stuurwoord per band, zoals de documentatie van je hardware het vermeldt: bit 0 is uitgang 1, bit 6 is uitgang 7. RX wordt aangestuurd tijdens ontvangen op die band en TX terwijl de zender gekeyd is — geef ze dezelfde waarde voor een filter, en verschillende voor alles wat maar aan één kant van de omschakeling hoort (de keylijn van een versterker, de bypass van een ontvangstvoorversterker). Banden die op 00 blijven, sturen niets aan. Wordt toegepast bij Toepassen / opnieuw verbinden.
settings-radio-alex-band-code = ALEX-BANDCODE
settings-radio-replace-table-what-preset-would = Vervang de tabel door wat deze preset op elke band zou sturen, als beginpunt om te bewerken. Niets anders in de configuratie verandert.
settings-radio-clear = WISSEN
settings-radio-every-output-off-every-band = Elke uitgang uit op elke band, wat "Geen" doet.
settings-radio-band = Band
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = Aangestuurde uitgangen
settings-radio-other = Overig
settings-radio-everywhere-outside-amateur-bands-short = Overal buiten de amateurbanden — kortegolfluisteren, en alles waar de afstemming van een transverter op landt dat geen band dekt.
settings-radio-dongle = Dongle
settings-radio-rescan = Opnieuw scannen
settings-radio-re-list-usb-bus-no = De USB-bus opnieuw weergeven. Er wordt geen apparaat geopend, dus dit is veilig om in te drukken tijdens het ontvangen.
settings-radio-no-dongles-press-rescan = geen dongles — druk op Opnieuw scannen
settings-radio-first-one-found = — eerste gevonden —
settings-radio-rtl2832u-s-resampler-reaches-225 = De resampler van de RTL2832U bereikt 225–300 kHz en 900 kHz–3,2 MHz, niets ertussen. Werkt na Toepassen.
settings-radio-often-drops-samples = {"  "}(laat vaak samples vallen)
settings-radio-agc = AGC
settings-radio-manual-setting-measurement-weak-signal = Handmatig is de instelling voor metingen en digitale modes voor zwakke signalen. De tuner en de demodulator hebben onafhankelijke automatische lussen.
settings-radio-tuner-gain = Tunerversterking
settings-radio-applies-immediately-no-reconnect-tuner = Werkt direct — zonder opnieuw verbinden. De tuner heeft 29 discrete stappen, dus de waarde springt naar de dichtstbijzijnde die hij kan maken. Genegeerd zolang de tuner-AGC loopt.
settings-radio-crystal-error-parts-per-million = Kristalfout in parts per million. Draai met RUST_LOG=sdroxide_rtlsdr=debug en het log toont na ongeveer 20 seconden de gemeten klokfout — dat is het getal om in te voeren. Wordt direct toegepast.
settings-radio-hf-reception = HF-ontvangst
settings-radio-tuner-itself-starts-24-mhz =
    De tuner zelf begint bij 24 MHz. Een RTL-SDR Blog V4 upconverteert daaronder in hardware; andere dongles bereiken HF alleen door de ADC rechtstreeks te samplen, via de HF-poort van de V3. Automatisch geeft alles onder 24 MHz aan de ADC en alles erboven aan de tuner.

    Directe sampling dekt elke HF-band, 17 m en 15 m inbegrepen — ze komen aan in de tweede Nyquist-zone van de ADC, de goede kant op. Niets filtert de ingang van de ADC echter, dus wat er op 28,8 MHz min de afstemming zit, komt mee: 10,7 MHz onder 17 m, 7,726 MHz onder 15 m.

    Van mode wisselen onderbreekt de stroom kort.
settings-radio-removes-dongle-s-own-dc =
    Verwijdert de eigen DC-piek van de dongle uit het midden van de span, en het spiegelbeeld dat elk signaal eromheen achterlaat, door de onbalans in de samples zelf te meten — geen kalibratie, en het werkt direct. De tuner heeft geen offsetafstemmode, dus dit is de enige manier om het midden vrij te maken.

    Een AM-draaggolf precies op de afstemming zit ook op DC, dus verdwijnt hij samen met de piek: stem een kilohertz ernaast af, of schakel dit uit.
settings-radio-remove-centre-spike-mirror-image = De middenpiek en het spiegelbeeld verwijderen
settings-radio-bias-tee = Bias-tee
settings-radio-feed-4-5-v-dc = ~4,5 V DC over de coax voeden
settings-radio-bias-tee-never-connect-transceiver = Bias-tee staat AAN. Sluit nooit een transceiver, een geaarde antenne of een elders gevoede voorversterker aan terwijl dit is ingeschakeld — de DC gaat rechtstreeks de voedingslijn in.
settings-radio-receive-only-dongle-sample-rate = Alleen ontvangst. De dongle en samplefrequentie werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt.
settings-radio-server-address = Serveradres
settings-radio-where-rtl-tcp-listening-address =
    Waar rtl_tcp luistert: een adres, of een adres en poort. De poort is standaard 1234, de eigen standaard van rtl_tcp.

    Start het aan de andere kant als `rtl_tcp -a 0.0.0.0` — gebonden aan 127.0.0.1, wat het doet zonder -a, accepteert het alleen verbindingen van dezelfde machine.

    Werkt na Toepassen.
settings-radio-host-host-port-e-g = host of host:poort, bijv. raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    Gevraagd aan de server, met dezelfde resamplerlimieten als een lokale dongle — het is hetzelfde silicium aan de andere kant.

    Het getal naast elke snelheid is wat het op de verbinding kost: de samples worden ongecomprimeerd verstuurd, en een snelheid die het netwerk niet kan dragen laat rtl_tcp de verbinding verbreken in plaats van te degraderen. Werkt na Toepassen.
settings-radio-wired-link = {"  "}(bekabelde verbinding)
settings-radio-runs-server-s-dongle-manual = Draait op de dongle van de server. Handmatig is de instelling voor metingen en digitale modes voor zwakke signalen.
settings-radio-applies-immediately-no-reconnect-sent = Werkt direct — zonder opnieuw verbinden. Verzonden in tienden van een dB en door de server afgerond op een stap die zijn tuner heeft; het protocol kent geen antwoorden, dus waarop het is uitgekomen kan niet worden teruggelezen en deze schuif blijft tonen wat is gevraagd. Genegeerd zolang de tuner-AGC loopt.
settings-radio-crystal-error-server-s-dongle =
    Kristalfout van de dongle van de *server*, in parts per million — een eigenschap van die hardware, dus ingesteld hier en niet op de dongles van deze machine. Wordt direct toegepast.

    De gemeten klokfout die de USB-interface toont, is hier niet beschikbaar: via een netwerk ziet die meting de buffering, niet het kristal, en zit hij duizenden ppm ernaast. Kalibreer de dongle eenmaal op USB en neem het getal over, of stem af op een omroepstation met bekende frequentie en pas aan tot het op de afstemming zit.
settings-radio-tuner-starts-24-mhz-below =
    De tuner begint bij 24 MHz; daaronder heeft de andere kant hulp nodig. Een Blog V4 upconverteert zelf en Automatisch laat hem met rust — het enige wat het kan doen, omdat het protocol de tunerchip meldt en verder niets, en een V4 over de lijn niet te onderscheiden is van een gewone R828D.

    Op een V3 of een andere dongle schakelt Automatisch de server onder de eigen 24 MHz-ondergrens van de tuner naar directe sampling — wat elke HF-band dekt, waarbij 17 m en 15 m in de tweede Nyquist-zone van de ADC aankomen met 28,8 MHz min de afstemming erbovenop gevouwen. Kies expliciet Directe sampling voor een gewone R828D die op HF niets hoort. Wisselen onderbreekt de stroom kort.
settings-radio-removes-dongle-s-dc-spike =
    Verwijdert de DC-piek en het spiegelbeeld van de dongle hier, uit de samples zodra ze binnenkomen — het zijn artefacten van de hardware, dus ze reizen met al het andere over het netwerk. Wordt direct toegepast.

    Een AM-draaggolf precies op de afstemming zit ook op DC, dus verdwijnt hij samen met de piek: stem een kilohertz ernaast af, of schakel dit uit.
settings-radio-powers-preamp-far-end-s = Voedt een voorversterker vanaf de dongle aan de andere kant. Oudere servers implementeren het commando niet en negeren het stilzwijgend — het protocol kan geen nee zeggen — dus een bias-tee die niet aangaat, ligt niet noodzakelijk aan dit uiteinde.
settings-radio-feed-4-5-v-dc-2 = ~4,5 V DC over de externe coax voeden
settings-radio-bias-tee-hardware-somewhere-else = Bias-tee staat AAN, op hardware die ergens anders staat. Wat er aan het andere eind van die voedingslijn zit — een transceiver, een geaarde antenne, een al gevoede voorversterker — kun je niet voor je neus controleren.
settings-radio-sdrplay-server-rsp-tcp = SDRplay-server (rsp_tcp)
settings-radio-sdrplay-server-greets-exactly-like = Een SDRplay-server meldt zich precies als een dongle, dus deze worden altijd getoond in plaats van alleen wanneer er een wordt gedetecteerd. Bij een gewone rtl_tcp-server worden ze simpelweg genegeerd — het protocol kent geen antwoorden en verwerpt commando's die het niet kent. Het tabblad Apparaat noemt de server rsp_tcp zodra er een zich heeft geïdentificeerd.
settings-radio-input = Ingang A
settings-radio-input-b = Ingang B
settings-radio-hi-z = Hi-Z
settings-radio-lna-state = LNA-stand
settings-radio-step-index-not-db-figure = Een stapindex, geen dB-getal: wat elke stap waard is, hangt af van het RSP-model en de band. 0 is de meeste versterking.
settings-radio-if-gain-reduction = F.I.-versterkingsreductie
settings-radio-reduction-so-more-less-signal = Een reductie, dus meer is minder signaal. Wordt alleen opgevolgd met de AGC van de RSP uit.
settings-radio-enable = Inschakelen
settings-radio-level-rsp-s-agc-aims = Het niveau dat de AGC van de RSP probeert vast te houden.
settings-radio-notches = Notches
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = Referentie-uit
settings-radio-t-24-mhz-clock-out = 24 MHz-klokuitgang
settings-radio-rsp2-rspduo-only = Alleen RSP2 en RSPduo.
settings-radio-receive-only-address-sample-rate = Alleen ontvangst. Het adres en de samplefrequentie werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt. Een verbroken verbinding wordt vanzelf opnieuw geprobeerd, dus een server die opnieuw wordt gestart komt terug zonder dat je hier iets aanraakt.
settings-radio-where-spyserver-listening-address-address =
    Waar spyserver luistert: een adres, of een adres en poort. De poort is standaard 5555, de eigen standaard van spyserver.

    Controleer aan de andere kant dat het configuratiebestand een adres bindt dat andere machines kunnen bereiken — gebonden aan 127.0.0.1 accepteert het alleen verbindingen van dezelfde machine.

    Werkt na Toepassen.
settings-radio-host-host-port-e-g-2 = host of host:poort, bijv. raspberrypi.local:5555
settings-radio-i-q-bandwidth = I/Q-bandbreedte
settings-radio-how-wide-slice-band-arrives =
    Hoe breed een stuk van de band als I/Q binnenkomt — en dus hoeveel van de verbinding dit gebruikt. Dit is het *gedemoduleerde* venster: het volgt de afstemming, en alles wat breder is, is de FFT van de server in de strook boven de panadapter.

    Automatisch mikt op ongeveer 96 kHz, wat elke mode hier draagt, breedband-FM inbegrepen, en nog steeds door een mobiele uplink past. Werkt na Toepassen.
settings-radio-which-stage-server-s-own =
    Welke trap van de eigen samplefrequentieladder van de server te vragen. Elke ontvanger heeft een andere ladder — het is zijn maximale snelheid trap voor trap gehalveerd — dus dit wordt opgeslagen als de trap en niet als een getal in hertz, en dezelfde instelling betekent nog steeds iets zinnigs wanneer hij naar een andere server wijst.

    Automatisch mikt op ongeveer 1 Msps. Druk op Verbinding testen om te zien waar de trappen op deze server op uitkomen. Werkt na Toepassen.
settings-radio-automatic = Automatisch
settings-radio-sample-format = Sampleformaat
settings-radio-bits-per-component-wire-so =
    Bits per component op de lijn, en dus wat dit op de verbinding kost: 16-bit is tweemaal 8-bit, en 32-bit float is viermaal zoveel voor niet meer informatie dan de ADC van de ontvanger had.

    8-bit is wat een externe ontvanger over een thuisuplink laat werken en is goed voor bijna alles. Een server kan dit overrulen — sommige zijn geconfigureerd om op één formaat te staan — en meldt dat in het log wanneer het gebeurt. Werkt na Toepassen.
settings-radio-gain = Versterking
settings-radio-server-s-gain-stage-index =
    De versterkingstrap van de server, als index — niet een aantal decibel. Wat elke index betekent, hoort bij de ontvanger aan de andere kant en verandert met de band, en het protocol zegt het nooit, dus niets hier kan het in dB omzetten zonder een getal te verzinnen.

    Het echte bereik is dat van de server en is pas bekend na het verbinden; een index erboven wordt begrensd. Werkt direct — zonder opnieuw verbinden.
settings-radio-index = index{" "}
settings-radio-digital-gain = Digitale versterking
settings-radio-how-far-server-scales-samples =
    Hoeveel de server zijn samples opschaalt voordat hij ze voor de lijn kwantiseert. Automatisch kijkt naar wat er werkelijk aankomt en houdt de piek op de helft van de volle schaal, het enige dat zowel op een dode als op een drukke band goed kan zijn — die twee liggen dertig decibel uit elkaar en geen vast getal bedient beide.

    Het doet er alleen toe bij 8 bits, waar een signaal ver onder de volle schaal zijn lage bits aan de kwantiseerder verliest en een signaal erboven plat wordt geclipt. Bredere formaten hebben ruimte over en krijgen het getal dat elke andere client berekent, uit de versterkingsindex en de decimatietrap. Werkt direct.
settings-radio-full-band-strip = Strook volledige band
settings-radio-server-s-own-fft-whole =
    De eigen FFT van de server van de hele band, getekend in de strook boven de panadapter. In deze interface is het de enige bandweergave die er is — de panadapter zelf is slechts zo breed als de ontvangen I/Q.

    Uitschakelen laat een ontvanger zonder enige mogelijkheid om iets te zien waarop hij niet al is afgestemd. Werkt direct, met een korte onderbreking in de audio terwijl de server wijzigt wat hij verstuurt.
settings-radio-ask-server-low-rate-fft =
    Vraag de server naast de I/Q ook om een FFT met lage snelheid van de hele band, en teken die in de strook boven de panadapter.

    Het kost bijna niets — een paar kilobyte per frame, een stuk of twaalf keer per seconde — en het toont de hele ontvanger in plaats van het gedemoduleerde stuk. Alleen de moeite waard om uit te schakelen op een verbinding waar elke byte telt. Werkt direct, met een korte onderbreking in de audio.
settings-radio-show = Tonen
settings-radio-whole-band = hele band
settings-radio-how-much-receiver-strip-covers = Hoeveel van de ontvanger de strook dekt. De hele band is de breedste weergave die er is; versmallen zet hetzelfde aantal bins over minder spectrum, wat fijner detail over een kleiner stuk geeft.
settings-radio-strip-db-window = dB-venster strook
settings-radio-server-quantises-fft-into-window =
    De server kwantiseert zijn FFT in dit venster voordat hij hem verstuurt, één byte per bin — dus deze bepalen hoe fijn de strook wordt gemeten, niet alleen hoe hij wordt getekend. De vloer en het plafond waarmee de strook wordt *getoond* worden bepaald door de eigen auto-levelling van de engine en zijn iets anders.

    De standaard van 150 dB is het hele protocolbereik en vraagt geen aandacht. Het rond de ruisvloer versmallen levert resolutie op bij een ontvanger waarvan de signalen allemaal in een klein deel van de schaal zitten. Werkt direct.
settings-radio-top = boven{" "}
settings-radio-range-2 = bereik{" "}
settings-radio-i-q-correction = I/Q-correctie
settings-radio-remove-dc-spike-mirror-image = Verwijder de DC-piek en het spiegelbeeld in DSP, aan deze kant. Of de ontvanger aan de andere kant het nodig heeft, hangt af van wat het is — een Airspy HF+ niet, een RTL-SDR wel — en het protocol zegt niet met welke het praat. Werkt direct.
settings-radio-enabled = Ingeschakeld
settings-radio-test-connection = Verbinding testen
settings-radio-connect-read-what-server-says = Verbind, lees wat de server over zijn ontvanger zegt, en verbreek weer — zonder een stroom te starten, dus het is veilig om in te drukken op een server die iemand anders gebruikt.
settings-radio-receive-only-panadapter-narrow-i = Alleen ontvangst. De panadapter is het smalle I/Q-venster, dat de afstemming volgt; de bandweergave is de FFT van de server in de strook erboven. Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-receive-only-wideband-i-q = Alleen ontvangst. Breedband-I/Q, net als elke lokale SDR. Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-receiver-address = Ontvangeradres
settings-radio-where-receiver-serves-web-page =
    Waar de ontvanger zijn webpagina aanbiedt: een adres, of een adres en poort. De poort is standaard 8073, de eigen poort van een KiwiSDR.

    Let op dat een ontvanger die via de proxy van het project wordt bereikt (iets.proxy.kiwisdr.com — bijna de helft van de openbare) in plaats daarvan op poort 80 antwoordt, dus geef die expliciet op. Een ontvanger kiezen uit "Openbare SDR's" vult dit correct in.

    Werkt na Toepassen.
settings-radio-host-host-port-e-g-3 = host of host:poort, bijv. kiwi.example.org:8073
settings-radio-password = Wachtwoord
settings-radio-receiver-s-user-password-where =
    Het *gebruikers*wachtwoord van de ontvanger, waar de beheerder er een heeft ingesteld. Leeg op bijna elke openbare ontvanger. Niet het beheerderswachtwoord.

    Werkt na Toepassen.
settings-radio-announce = Melden als
settings-radio-name-connection-shows-up-under =
    De naam waaronder deze verbinding verschijnt, die de eigenaar van de ontvanger en iedereen die er verder naar luistert kan zien.

    Leeg is het je stationsroepletters, wat de eigen conventie van het netwerk is — sommige operators blokkeren clients die zich niet identificeren. Werkt na Toepassen.
settings-radio-blank-station-callsign = leeg = roepletters van het station
settings-radio-band-view = Bandweergave
settings-radio-ask-receiver-own-waterfall-well =
    Vraag de ontvanger naast de I/Q ook om zijn eigen waterval, en teken die in de strook boven de panadapter.

    De moeite waard: de I/Q is maar ongeveer 12 kHz breed, dus zonder dit is er niets om *op* af te stemmen. Het is een tweede verbinding die op volle snelheid ongeveer 20 kB/s draagt, tegen de 44 kB/s van de I/Q.

    Werkt na Toepassen.
settings-radio-show-receiver-s-0-30 = de 0-30 MHz-waterval van de ontvanger tonen
settings-radio-band-view-speed = Snelheid bandweergave
settings-radio-how-often-receiver-sends-waterfall =
    Hoe vaak de ontvanger een watervalrij verstuurt, 1 (langzaamst) tot 4. De enige instelling hier die verandert wat de verbinding kost tijdens het draaien, dus zet hem lager op een verbinding met databudget.

    Werkt direct.
settings-radio-band-view-span = Span bandweergave
settings-radio-how-much-band-receiver-s =
    Hoeveel band de waterval van de ontvanger dekt. De volle 0-30 MHz links op de schuif, bij elke stap naar rechts gehalveerd.

    De waterval is altijd 1024 bins breed, hoe breed het venster ook is, dus dit is eigenlijk een resolutieregeling: de hele band is 29 kHz per bin, wat een bandkaart is en geen beeld van iets, terwijl een paar honderd kilohertz individuele stations in een omroepband toont. Ingezoomd volgt het venster je afstemming.

    De hele band is de standaard en maakt van de strook iets om op af te stemmen; de panadapter eronder is de ~12 kHz aan I/Q. Werkt direct.
settings-radio-receiver-agc = Ontvanger-AGC
settings-radio-receiver-s-agc-which-far =
    De AGC van de *ontvanger*, die aan de andere kant van de verbinding zit en vóór de I/Q — dus hij werkt voordat sdroxide iets doet.

    Standaard aan, anders dan bij elke lokale SDR hier, omdat het gemeten alternatief slechter was: op een live ontvanger was de handmatige versterking niet monotoon over het bereik en clipte de top de I/Q op volle schaal, terwijl de AGC rond -24 dBFS bleef met 7 dB speling.

    Het betekent wel dat de samplesamplitude geen signaalniveau is, en daarom wordt de S-meter uit het eigen getal van de ontvanger gelezen. Werkt direct.
settings-radio-let-receiver-ride-own-gain = laat de ontvanger zijn eigen versterking regelen
settings-radio-manual-gain = Handmatige versterking
settings-radio-fixed-gain-when-receiver-s = Vaste versterking als de AGC van de ontvanger uit staat, op zijn eigen 0-90-schaal. Geen decibels van iets bepaalds — het protocol noemt het manGain en zegt niets meer, en het bleek niet monotoon te zijn, dus zie het als een knop om een goede plek op te vinden en niet als een kalibratie. Werkt direct.
settings-radio-read-receiver-s-status-page = Lees de statuspagina van de ontvanger: wat het is, wat het dekt, hoeveel kanalen vrij zijn en of de beheerder verbindingen toestaat van andere apps dan een browser. Gebruikt geen van zijn kanalen.
settings-radio-host-port-e-g-127 = host:poort, bijv. 127.0.0.1:50001
settings-radio-iq-sample-rate = IQ-samplefrequentie
settings-radio-receiver = Ontvanger
settings-radio-rig-two-receivers-sunsdr2dx-can = Een radio met twee ontvangers (SunSDR2DX) kan twee radiotabbladen bedienen vanaf één verbinding — draai deze radio op RX1 en een andere op RX2. De zender hoort bij de RX1-radio.
settings-radio-stream-delay = Streamvertraging
settings-radio-how-long-rig-s-iq = Hoe lang de IQ van de radio erover doet om op een nieuw midden aan te komen nadat sdroxide het verplaatst. Het is de eigen DSP-pijplijn van de radio, niet het netwerk — een server op deze machine heeft er ook een — en terwijl de panadapter volledig uitgezoomd wordt gesleept, verschuift een fout van deze grootte de nieuwste watervalrijen zijwaarts ten opzichte van de historie met de fout maal de sleepsnelheid. Te hoog verplaatst ze precies zo ver als te laag, de andere kant op. Meet het met `cargo run --release -p sdroxide-tci --example retune_latency`; de standaard is een SunSDR2DX op ExpertSDR3 bij 192 kHz.
settings-radio-wideband-iq-receive-audio-transmit = Breedband-IQ ontvangen, audio zenden. Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-radio-address = Radioadres
settings-radio-address-shown-radio-under-set = Het adres dat de radio toont onder SET > Network. Network Control moet daar aan staan, en de radio heeft een netwerkgebruikersnaam en wachtwoord nodig.
settings-radio-host-ip-e-g-192 = host of IP, bijv. 192.168.1.50
settings-radio-control-port = Besturingspoort
settings-radio-t-50001-unless-has-been-changed = 50001 tenzij gewijzigd op de radio.
settings-radio-network-user = Netwerkgebruiker
settings-radio-stored-clear-radio-json-protocol = Leesbaar opgeslagen in radio.json. Het protocol versluiert het omkeerbaar op de lijn, dus niets hier maakt er een geheim van.
settings-radio-receive = Ontvangen van
settings-radio-audio-sample-rate = Audio-samplefrequentie
settings-radio-t-12-khz-if-needs-48000 = Een 12 kHz F.I. heeft de 48000 Hz-stroom nodig — bij deze snelheid wordt in plaats daarvan de gedemoduleerde audio van de radio gebruikt.
settings-radio-if-spectrum = F.I.-spectrum
settings-radio-which-way-round-radio-s = Welke kant op de 12 kHz F.I. van de radio loopt. "Automatisch" gebruikt wat het model bekend is te doen — gespiegeld op een IC-7760, normaal op elke andere Icom. Stel het met de hand in als SSB op de verkeerde zijband uitkomt: het verraad is dat je USB moet kiezen waar de band LSB loopt terwijl het modedisplay van de radio zelf steeds met sdroxide overeenkomt.
settings-radio-normal = Normaal
settings-radio-mirrored = Gespiegeld
settings-radio-displayed-bandwidth = Getoonde bandbreedte
settings-radio-width-audio-band-panadapter-cat = Breedte van de audioband-panadapter, zoals bij een CAT-radio. Gebruikt in de digitale modes en waar de scope van de radio uit staat; anders is de panadapter de scope, en bepaalt de Scopespan zijn breedte.
settings-radio-how-cw-panel-s-keyer-2 = Hoe de keyer van het CW-paneel zendt. "Radio-keyer" zet de radio in CW en geeft hem de te verzenden tekst voor zijn eigen keyer (CI-V 0x17). "Geluidskaart" verzendt de gekeyde sidetone als audio over het LAN (MCW), een toon op afstemming + toonhoogte — en omdat een radio in CW zijn modulatoringang zou negeren, houdt het kiezen van CW de radio dan in gewone USB, dezelfde mode waarop de digitale modes meeliften.
settings-radio-how-much-audio-radio-holds = Hoeveel audio de radio vasthoudt voordat hij moduleert. Meer overleeft een slechter netwerk, ten koste van zendlatentie.
settings-radio-streams-radio-s-own-475 = Streamt de eigen sweep van 475 bins van de radio. Op AF is het de panadapter — de audio die de radio verstuurt is wat door zijn filter kwam, geen beeld van de band — en op de 12 kHz F.I. is het de volledige-band-waterval erboven. Hoe dan ook is het het beeld van de radio, niet van de DSP van sdroxide: er is geen I/Q om er een uit te berekenen.
settings-radio-how-wide-sweep-only-wide = Hoe breed hij moet vegen. Dit is de enige brede weergave die een Icom heeft, en de radio houdt de span aan die het laatst op zijn eigen scherm is gekozen — vaak een paar kHz, waardoor de strook niet breder kan uitkomen dan de panadapter eronder. Een span hier instellen zet de scope ook in middenmodus, zodat hij de afstemming volgt. Het wijzigt ook het eigen display van de radio; "Zoals ingesteld op de radio" laat het met rust.
settings-radio-switch-modulation-input-lan = Moduleringsingang naar LAN schakelen
settings-radio-transmit-audio-only-heard-when = Zendaudio wordt alleen gehoord als de MOD-ingang van de radio op LAN staat. sdroxide kan dat instellen op een model waarvan het de menunummering kent; op elk ander meldt het dat en laat het het menu met rust. Het is een lening: wat de radio daar had, wordt eerst gelezen en teruggezet als de sessie eindigt, zodat een radio die daarna op zichzelf wordt gebruikt zijn eigen microfoon nog hoort.
settings-radio-copy-diagnostic-report = Diagnoserapport kopiëren
settings-radio-control-audio-radio-s-own = Besturing, audio en de eigen scope van de radio over één netwerkverbinding — geen seriële kabel en geen geluidskaart. Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-testing = Testen…
settings-radio-was-only-check-press-apply = Dat was slechts een controle — druk hieronder op Toepassen / opnieuw verbinden om deze radio te gaan gebruiken.
settings-radio-radios = Radio's
settings-radio-asks-network-iio-devices-also = Vraagt het netwerk om IIO-apparaten, en probeert ook rechtstreeks 192.168.2.1 — een Pluto aan het eind van een USB-kabel is vaak onbereikbaar via multicast, ook al werkt het adres.
settings-radio-no-radios-press-discover = geen radio's — druk op Zoeken
settings-radio-address = Adres
settings-radio-overrides-selection-above-usb-cable = Gaat voor op de selectie hierboven. De USB-kabel presenteert de Pluto als netwerkadapter, niet als seriële poort, dus dit is een IP-adres, ook als de radio op je bureau staat.
settings-radio-pluto-revision-c-pluto-unlocked = Een Pluto+ of een revisie-C Pluto ontgrendeld naar 2R2T kan twee radiotabbladen bedienen vanuit één kastje — deze radio op RX1 en een andere op RX2, elk op zijn eigen antenne. De twee ketens delen dezelfde oscillator, dus het hernieuwd afstemmen van de ene radio verplaatst beide; wat RX2 oplevert is een tweede antenne op hetzelfde spectrum (diversity), geen tweede band. De zender hoort bij de RX1-radio. Een standaard 1R1T Pluto weigert RX2 wanneer hij verbindt.
settings-radio-width-spectrum-sdroxide-receives-ad9361 = Breedte van het spectrum dat sdroxide ontvangt. De AD9361 haalt 61,44 Msps; de USB-netwerkverbinding niet, en daarop is deze lijst geschaald. De laagste snelheden vereisen een filterconfiguratie die in de AD9361 wordt geladen, wat sdroxide niet doet — een standaard Pluto draait ze in plaats daarvan op ongeveer 2,084 Msps en meldt dat wanneer hij verbindt. Werkt na Toepassen.
settings-radio-more-than-usb-2-will = {"  "}(meer dan USB 2 aankan)
settings-radio-stock-pluto-runs-2-084 = {"  "}(een standaard Pluto draait op 2,084)
settings-radio-analog-filter = Analoog filter
settings-radio-ad9361-s-baseband-filter-leave = Het basisbandfilter van de AD9361. Laat op 0 staan voor automatisch, wat het opent tot negen tienden van de samplefrequentie — met opzet breed, omdat de ontvanger zijn oscillator een kwart span naast de afstemming parkeert om signalen vrij van de DC-piek te houden, en een smal filter precies het deel zou afsnijden waar hij ze naartoe heeft verplaatst. Werkt na Toepassen.
settings-radio-ad9361-has-four-modes-not = De AD9361 heeft vier modes, geen aan/uit-schakelaar. Slow attack past bij SSB en CW; fast attack bij stootsgewijze signalen; handmatig is de instelling voor metingen en digitale modes voor zwakke signalen. Werkt direct.
settings-radio-rx-gain = RX-versterking
settings-radio-applies-immediately-no-reconnect-ignored = Werkt direct — zonder opnieuw verbinden. Genegeerd tenzij de AGC op handmatig staat, wat het eigen gedrag van de AD9361 is, niet van sdroxide.
settings-radio-tx-gain = TX-versterking
settings-radio-negative-because-ad9361-expresses-transmit = Negatief omdat de AD9361 het zendniveau als demping uitdrukt: 0 dB is volle uitgang. Werkt direct. De zender wordt op zijn stilst gezet voordat deze waarde bij het verbinden wordt toegepast, zodat niets wat het vorige programma achterliet actief kan zijn.
settings-radio-reference-error-parts-per-million = Referentiefout in parts per million. Draai met RUST_LOG=sdroxide_pluto=debug en het log toont na ongeveer 20 seconden de gemeten klokfout — dat is het getal om in te voeren. Toegepast door sdroxide, niet naar de eigen permanente trim van de radio geschreven. Werkt direct.
settings-radio-full-duplex = Full duplex
settings-radio-keep-receiving-while-transmitting = Blijven ontvangen tijdens het zenden
settings-radio-leave-off-pluto-reached-over =
    Laat dit uit op een Pluto die via zijn USB-kabel wordt bereikt. Die verbinding kan niet een megasample per seconde in beide richtingen tegelijk dragen, en een beurt die de zendbuffer uithongert gaat verhakkeld de lucht in — dus standaard stopt de ontvangst voor de duur van een beurt.

    Zet het aan voor een bord op echt Ethernet (een LibreSDR, een Pluto op een gigabit-adapter), waar ruimte is voor beide: je hoort dan de ontvanger door je eigen zending heen, zoals een QO-100-station zijn eigen downlink beluistert.

    Een beurt vraagt tweemaal de samplefrequentie aan verbinding — 2,5 Msps is 10 MB/s in elke richting — dus verlaag de snelheid als het log begint te zeggen dat de verbinding het niet draagt. De panadapter toont tijdens een beurt nog steeds het gezonden signaal; het is de audio die blijft doorkomen.

    Werkt na Toepassen.
settings-radio-duplex = Duplex
settings-radio-whether-ad9361-runs-both-directions =
    Of de AD9361 beide richtingen tegelijk draait (FDD, zoals een Pluto opstart en wat sdroxide hem altijd heeft gelaten) of één tegelijk (TDD).

    Laat dit op FDD tenzij je de PTT-pinnen hieronder wilt — TDD is wat die aansturen, en het sluit Full duplex hierboven uit. Werkt na Toepassen.
settings-radio-ptt-pins = PTT-pinnen
settings-radio-pluto-s-four-gpo-test =
    De vier GPO-testpunten van de Pluto kunnen zelfstandig een externe eindversterker, LNA of zend-ontvangstschakelaar keyen. Kies een paar en één pin is hoog zolang de radio ontvangt, de andere zolang hij zendt — geen hostsoftware in de lus en geen seriële PTT-lijn om te bedraden.

    Dit zet de radio in TDD wat de rij Duplex ook zegt, omdat de pinnen de enable-lijnen van de AD9361 volgen en in FDD beide de hele sessie actief zijn. Full duplex gaat daarmee uit, dus laat beide op hun standaard als je satellieten werkt en je eigen downlink moet horen.

    De eigen opmerking van Analog Devices plaatst een externe LNA op GPO0/GPO1, dus gebruik GPO2/GPO3 als je bord zo is bedraad. De pinnen leveren ongeveer 1,3 V bij enkele milliampères: stuur er een transistor of opto-isolator mee aan, nooit een relaisspoel.

    Werkt na Toepassen.
settings-radio-buffer-size = Buffergrootte
settings-radio-how-much-device-holds-before =
    Hoeveel het apparaat vasthoudt vóór elke overdracht, in complexe samples. De standaard van 32768 is ongeveer 16 ms bij 2 Msps: lang genoeg dat de heen-en-terugreis per overdracht niet de bottleneck is, kort genoeg dat een nieuwe afstemming niet zichtbaar te laat is.

    Halveer hem als het log meldt dat de ontvangstsocket wordt vervangen. Een marginale verbinding — een Pluto via zijn USB-kabel bij een hoge samplefrequentie is de gebruikelijke — blokkeert halverwege een overdracht, en een kleinere overdracht wordt minder snel halverwege betrapt en is daarna sneller hersteld. Verhoog hem om afstemlatentie in te ruilen voor minder heen-en-terugreizen.

    Werkt na Toepassen.
settings-radio-samples = {" "}samples
settings-radio-rx-tx-port = RX- / TX-poort
settings-radio-ad9361-s-rf-port-select = De rf_port_select van de AD9361. Een standaard Pluto bedraadt er van elk één, dus laat deze leeg tenzij je een bord hebt dat dat niet doet. Werkt na Toepassen.
settings-radio-opens-radio-reads-what-says = Opent de radio, leest wat hij over zichzelf zegt, en meldt het afstembereik van dit specifieke bord. Start geen stroom.
settings-radio-wideband-iq-receive-transmit-half = Breedband-IQ ontvangen en zenden. Half duplex — de ontvangst stopt voor de duur van een beurt, omdat de USB-netwerkverbinding niet beide tegelijk draagt. Geen SoapySDR en geen libiio nodig.
settings-radio-flexradio-announces-itself-local-network = Een FlexRadio meldt zich ongeveer eens per seconde op het lokale netwerk. Een radio die via een router of VPN wordt bereikt, zendt nooit naar jou — voer in plaats daarvan hieronder zijn adres in.
settings-radio-another-gui-client-has-radio = Een andere GUI-client heeft deze radio en multiFLEX is uitgeschakeld. Verbreek die client, of schakel multiFLEX op de radio in.
settings-radio-overrides-selection-above-use-radio = Gaat voor op de selectie hierboven. Gebruik dit voor een radio op een ander subnet, achter een VPN of op een niet-standaardpoort.
settings-radio-width-spectrum-sdroxide-receives-192 = Breedte van het spectrum dat sdroxide ontvangt. 192 kHz is het maximum van de radio voor een DAX-IQ-stroom, en dus de breedste span die deze interface kan tonen.
settings-radio-t-192-khz-widest-dax-iq = 192 kHz is de breedste DAX-IQ-stroom die een FLEX zendt, en de radio koppelt de span van de panadapter eraan — dus dat is de hele span, en geen instelling hier maakt hem breder.
settings-radio-dax-iq-channel = DAX-IQ-kanaal
settings-radio-radio-has-four-change-only = De radio heeft er vier. Wijzig dit alleen als iets anders op het netwerk kanaal 1 al gebruikt — de radio weigert een kanaal tweemaal.
settings-radio-station-name = Stationsnaam
settings-radio-shown-against-session-radio-s = Getoond bij deze sessie in de clientlijst van de radio. De radio onthoudt een client er ook aan, dus hernoemen laat de radio sdroxide als een nieuwe behandelen.
settings-radio-gui-client-id = GUI-client-ID
settings-radio-identity-radio-restores-session-s = De identiteit waarop de radio de slices en panadapters van deze sessie herstelt. Leeg wordt hij afgeleid van de stationsnaam — stabiel, maar gelijk op elke sdroxide die de standaardnaam hield, en een radio lost een duplicaat op door de eerdere client eruit te gooien. sdroxide ziet dat op de lijn en valt terug op een identiteit voor één sessie, wat het herstel kost; zet hier een eigen UUID neer om het te behouden.
settings-radio-optional-e-g-uuid-your = optioneel, bijv. een eigen UUID
settings-radio-mirror-radio-s-i-q =
    Spiegel de I/Q van de radio om het midden van de panadapter. Standaard uit, zoals een FLEX-6600 is geverifieerd.

    Probeer het als de ontvangstaudio onverstaanbaar is op USB *en* op LSB en niets decodeert, terwijl de waterval volkomen overtuigend lijkt — zo ziet een gespiegelde stroom eruit, en het is de enige fout zonder ander symptoom (issue #368). Als het dat niet is, maakt aanzetten het duidelijk in plaats van subtiel. Wordt toegepast bij Toepassen / opnieuw verbinden.
settings-radio-network-mtu = Netwerk-MTU
settings-radio-largest-datagram-radio-may-send = Grootste datagram dat de radio mag sturen. 1450 is wat SmartSDR zelf vraagt. Verlaag het op een pad met een kleinere MTU — een VPN of tunnel — waar de fragmenten worden weggegooid en helemaal geen spectrum aankomt.
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = Controleert of de radio antwoordt, zonder zich als GUI-client te registreren — dus een al lopende SmartSDR-sessie wordt niet verstoord.
settings-radio-wideband-iq-receive-over-dax = Breedband-IQ ontvangen via DAX, audio zenden die de radio moduleert. Druk op "Toepassen / opnieuw verbinden" om te wisselen zonder herstart.
settings-radio-not-yet-verified-against-real-2 = Nog niet geverifieerd met echte hardware. Als het zich misdraagt, stuur dan het diagnoserapport — het bevat elke protocolregel die met de radio is uitgewisseld.
settings-radio-devices-soapysdr-can-see = Apparaten die SoapySDR kan zien
settings-radio-waiting-machine-radio-attached-where = Wachten op de machine waarop de radio is aangesloten, waar de modules zijn geïnstalleerd. Welk apparaat hij opent is `device_args` in de config.toml van die machine, of zijn --device.
settings-radio-ask-every-installed-soapysdr-module = Vraag elke geïnstalleerde SoapySDR-module om te scannen. Er wordt niets geopend, dus dit is veilig tijdens ontvangst — maar het kan even duren.
settings-radio-not-enumerated-yet-press-rescan = Nog niet opgesomd — druk op Opnieuw scannen.
settings-radio-no-soapysdr-devices-found-check = Geen SoapySDR-apparaten gevonden. Controleer of de module voor je radio is geïnstalleerd en of je toegang tot het apparaat hebt.
settings-radio-not-radio = geen radio
settings-radio-sound-card-listed-above-if = Hierboven staat een geluidskaart alsof het een SDR is. Hij accepteert elke frequentie en negeert die, dus openen toont de ingang van de geluidskaart in plaats van de band. sdroxide kiest die niet automatisch — maar een device_args-regel die er een noemt, wordt nog steeds gehoorzaamd.
settings-radio-which-one-opens-set-device = Welke opent wordt bepaald door --device of device_args in config.toml; met geen van beide wint de eerste vermelde radio.
settings-radio-no-device = geen apparaat
settings-radio-rig-has-no-software-adjustable = Deze radio heeft geen via software instelbare versterkingen.
settings-radio-rx-gains = RX-versterkingen
settings-radio-tx-gains = TX-versterkingen
settings-radio-antennas = Antennes
settings-radio-remembered-next-start = Onthouden voor de volgende start.
settings-radio-these-controls-appear-once-soapysdr = Deze regelaars verschijnen zodra een SoapySDR-apparaat open is — het zijn die van het apparaat zelf, en het moet erom gevraagd worden.
settings-radio-stream = Stroom
settings-radio-follow-app-setting = De app-instelling volgen
settings-radio-msps = of, in Msps
settings-radio-baseband-filter = Basisbandfilter
settings-radio-let-driver-choose = Laat het stuurprogramma kiezen
settings-radio-changing-either-reopens-radio = Het wijzigen van een van beide heropent de radio.
settings-radio-driver-publishes-no-settings-own = Dit stuurprogramma publiceert geen eigen instellingen.
settings-radio-applied-once-remembered-next-start = Direct toegepast en onthouden voor de volgende start.
settings-radio-no-rx-888-found-press = Geen RX-888 gevonden — druk op Opnieuw scannen
settings-radio-adc-clock = ADC-klok
settings-radio-t-129-6-msps-needs-superspeed = 129,6 Msps vereist een SuperSpeed-verbinding en een snelle host; 64,8 is de veilige standaard. Wijzigen heropent de ontvanger automatisch, wat even duurt maar geen herstart vereist.
settings-radio-panadapter-width = Panadapterbreedte
settings-radio-hf-only = {"  "}· alleen HF
settings-radio-usable-both-front-ends = Bruikbaar op beide front-ends.
settings-radio-below-vhf-crossover-ordinary-width = Onder de VHF-overgang is dit een gewone breedte. Erboven kan de 8 MHz-F.I. van de tuner niet in zo'n breed venster worden gecentreerd, dus het live spectrum zit aan één kant van de panadapter met niets ernaast.
settings-radio-dial-above-crossover-now-so = De afstemming ligt nu boven de overgang, dus deze breedte toont de 8 MHz van de tuner uit het midden met dood spectrum ernaast. Maak de breedte smaller om de panadapter te vullen.
settings-radio-vga-gain = VGA-versterking
settings-radio-ad8370-variable-gain-amplifier-ahead = AD8370-versterker met variabele versterking vóór de ADC.
settings-radio-attenuator = Verzwakker
settings-radio-pe4304-step-attenuator-0-5 = PE4304-stapverzwakker, in stappen van 0,5 dB.
settings-radio-adc-range = ADC-bereik
settings-radio-wide-2-25-vp-p = Breed (2,25 Vp-p)
settings-radio-selects-adc-s-wider-input = Kiest het bredere ingangsbereik van de ADC: meer speling voor sterke omroepsignalen, minder tellen voor zwakke. Uit kiest het gevoeligere bereik van 1,5 Vp-p.
settings-radio-dither = Dither
settings-radio-adds-small-dither-signal-ahead = Voegt een klein dithersignaal toe vóór de ADC: kost wat ruisvloer, levert spurvrij dynamisch bereik op.
settings-radio-randomiser = Randomiser
settings-radio-adc-scrambles-output-so-digital = De ADC husselt zijn uitgang zodat de digitale bus ophoudt in de front-end te stralen; het stuurprogramma husselt het terug. Laat dit aan tenzij je aan het debuggen bent. Werkt bij opnieuw verbinden.
settings-radio-dc-hf-antenna-port = DC op de HF-antenneaansluiting
settings-radio-powers-active-antenna-preamp-down = Voedt een actieve antenne of voorversterker over de coax.
settings-radio-vhf-tuner-gain = VHF-tunerversterking
settings-radio-r828d-rf-gain-used-above = R828D-RF-versterking, gebruikt boven de automatische HF/VHF-overgang. 29 discrete stappen; de dichtstbijzijnde wordt gebruikt.
settings-radio-auto = Auto
settings-radio-let-tuner-run-own-lna = Laat de tuner zijn eigen LNA- en mixerlussen draaien.
settings-radio-bias-tee-vhf = Bias-tee (VHF)
settings-radio-dc-vhf-antenna-port = DC op de VHF-antenneaansluiting
settings-radio-clock-trim = Kloktrim
settings-radio-corrects-reference-oscillator-applied-when = Corrigeert de referentieoscillator. Toegepast wanneer je de waarde loslaat — bij elke pixel van een sleepbeweging heropenen zou de ontvanger honderden keren herstarten.
settings-radio-receive-only-below-adc-s = Alleen ontvangst. Onder de Nyquist-limiet van de ADC wordt de antenne rechtstreeks gesampled en is opnieuw afstemmen onmiddellijk, omdat er geen hardware-downconverter is — de volledige ADC-stroom wordt op de host naar basisband omgezet. Erboven schakelt de ontvanger automatisch over naar zijn R828D-tuner en de VHF-SMA, dus beide antennes moeten zijn aangesloten. VHF vereist een ADC-klok van 32,4 Msps of meer om de F.I. van de tuner te laten passen, en bij klokken onder 48 Msps zit er een gat tussen de twee bereiken dat niets kan bereiken. Elke instelling hier werkt meteen — er is geen Toepassen-knop om in te drukken.
settings-radio-device = Apparaat
settings-radio-no-devices-press-rescan = geen apparaten — druk op Opnieuw scannen
settings-radio-six-rates-are-six-different =
    De zes snelheden zijn zes verschillende FPGA-images, geen register — daarom selecteert niets in het protocol van ELAD ertussen, en daarom hangt de betekenis van deze instelling af van het model.

    FDM-S1 / FDM-S2: een commando. Hun FPGA wordt bij elke start door de computer geladen, en sdroxide draait de eigen elad-firmware-loader van ELAD om het image van deze snelheid erin te zetten. Zonder die geïnstalleerde loader stuurt een sampler helemaal geen samples, en sdroxide meldt dat op het scherm.

    FDM-DUO: geen commando. De radio start zijn eigen FPGA en komt aan op de snelheid waarmee hij opstartte of waarop FDM-SW2 hem het laatst liet (192 kHz bij een nieuwe), en dit zegt alleen hoe de stroom wordt GELEZEN. Fout ingesteld en de panadapter heeft gewoon de verkeerde breedte, met elke frequentie erin dienovereenkomstig geschaald.

    De werkelijke snelheid van de stroom wordt een paar seconden na de start gemeten, en een afwijking wordt op het scherm gemeld. Werkt na Toepassen.
settings-radio-fdm-s-fpga-image-loaded = FDM-S: het geladen FPGA-image — FDM-DUO: hoe de stroom wordt gelezen
settings-radio-receiver-s-input-pad-one = De ingangsverzwakker van de ontvanger — één stap, in of uit. Dezelfde regelaar als de Gain-schuif van het hoofdvenster.
settings-radio-pre-selection-filters = Preselectiefilters
settings-radio-low-pass-bank-front-adc = De laagdoorlaatbank vóór de ADC. Omzeilen geeft het breedste beeld en het slechtste gedrag bij sterke signalen buiten de band — een omroepzender een paar kilometer verderop zet beelden over de hele band. Laat aan tenzij je bewust buiten het gefilterde bereik luistert.
settings-radio-filters-circuit = Filters in het circuit
settings-radio-rig-control-fdm-duo-only = Radiobesturing — alleen FDM-DUO
settings-radio-transceiver-s-cat-usb-port = De CAT-USB-poort van de transceiver, die een aparte kabel is van die voor ontvangst. Laat de poort leeg bij een FDM-S1 of FDM-S2, die er geen hebben — en bij een FDM-DUO die je liever alleen via zijn ontvangstkabel bedient, waar hij nog wel kan worden afgestemd en gekeyd maar geen meter kan worden gelezen.
settings-radio-port-set-radio-s-display = Met de poort ingesteld zijn het display van de radio en deze afstemming in beide richtingen hetzelfde getal: het ontvangstvenster van de FDM-DUO is om zijn VFO gebouwd, dus hier afstemmen verplaatst het voorpaneel en aan de knop op het voorpaneel draaien verplaatst de uitlezing hier. De panadapter centreert zich onderweg opnieuw op de afstemming. Zonder poort — of een waarop de radio niet antwoordt — is er geen VFO om aan te sturen, en blijft het afstemmen binnen het venster dat de radio al verstuurt.
settings-radio-none-usb-control-only = — geen (alleen USB-besturing) —
settings-radio-must-match-menu-70-cat = Moet overeenkomen met menu 70 "CAT BAUD" op de radio, dat af fabriek op 38400 staat. De radio heeft deze vier snelheden en geen andere, en een poort die op een andere is geopend, is in beide richtingen stil — de radio negeert de afstemming en weigert te keyen, zonder te zeggen waarom.
settings-radio-how-transmit-keyed-cat-sends = Hoe het zenden wordt gekeyd. "CAT" stuurt het eigen TX-commando van de radio en vereist niets op de radio; RTS vereist dat menu 54 "PTT" op PTT+RTS staat.
settings-radio-where-radio-takes-transmit-audio-2 =
    Waar de radio zijn zendaudio vandaan haalt — het TI-commando van de radio, dat menu 32 "TX IN" op het voorpaneel is. Wordt bevestigd wanneer de poort opent.

    "USB-audio" is wat het zenden hier laat werken: de FDM-DUO verstuurt wat sdroxide in zijn USB-geluidskaart stopt. Een radio die op "Microfoon" blijft staan, zendt in plaats daarvan de kamer, en niets op het scherm zegt dat.
settings-radio-which-two-sockets-back-receiver-2 =
    Op welke van de twee aansluitingen aan de achterkant de ontvanger luistert — het AN-commando van de radio, dat menu 31 "ANTENNAS" op het voorpaneel is en de "ANT 1 2"-indicator op het display.

    "RTX" is één antenne voor beide taken, op de M-aansluiting die ook het zenden draagt. "Alleen RX" verplaatst de ontvangst naar de tweede aansluiting en laat het zenden op RTX — een ontvangstantenne, een loop of een beverage, met de beam nog op de zender.

    Het verplaatst deze hele ontvanger: de panadapter, de demodulatoren en de eigen audio van de radio komen allemaal van de hier gekozen aansluiting.

    Wordt direct toegepast en wordt teruggelezen van de radio wanneer de besturingspoort opent — dit is de instelling van de radio zelf, geen hier bewaarde kopie.
settings-radio-how-often-radio-asked-what-2 = Hoe vaak de radio wordt gevraagd wat hij doet — zijn afstemming, zijn mode en zijn meters. Draai aan de VFO-knop van de radio en de uitlezing volgt binnen één poll. Het is ook het hele besturingsverkeer dat dit uiteinde genereert, dus verlaag het als het geluid van de radio wegvalt.
settings-radio-transmit-audio = Zendaudio
settings-radio-only-receiving-fdm-duo-has = Tot nu toe is alleen ontvangen op een FDM-DUO op echte hardware gedraaid. Als iets zich misdraagt, voeg dan het diagnoserapport toe aan een bugmelding.
settings-radio-cw-keyed-radio-s-own = CW wordt gekeyd met de eigen sleutel of paddle van de radio: de FDM-DUO heeft geen commando dat tekst accepteert, dus het CW-paneel kan hem niet keyen.
settings-radio-press-apply-reopen-radio = Druk op Toepassen om de radio te heropenen.
settings-radio-no-receivers-press-rescan = geen ontvangers — druk op Opnieuw scannen
settings-radio-which-rates-receiver-has-depends = Welke snelheden een ontvanger heeft, hangt af van het model en de firmware. Werkt na Toepassen.
settings-radio-every-rate-any-hf-model = elke snelheid die een HF+-model biedt — verbind er een om de eigen te zien
settings-radio-receiver-s-own-gain-control = De eigen versterkingsregeling van de ontvanger. Laat aan voor algemeen luisteren; zet uit om de verzwakker met de hand in te stellen voor metingen.
settings-radio-agc-threshold = AGC-drempel
settings-radio-high = Hoog
settings-radio-high-trades-little-sensitivity-headroom = Hoog ruilt wat gevoeligheid in voor speling tegen sterke buren — de juiste instelling op een drukke band 's avonds.
settings-radio-front-end-attenuation-gain-0 = Front-end-demping, als versterking — 0 dB is geen. Wordt alleen opgevolgd met de AGC uit.
settings-radio-preamp = Voorversterker
settings-radio-hf-low-noise-amplifier-buys = De HF-laagruisversterker. Levert gevoeligheid op ten koste van intermodulatie, dus standaard uit — wat op een echte antenne meestal goed is.
settings-radio-lna = LNA aan
settings-radio-frequency-calibration = Frequentiekalibratie
settings-radio-parts-per-billion-receiver-s = Parts per billion — de eigen eenheid van deze ontvanger, duizend keer fijner dan de ppm die een RTL-SDR gebruikt. Hier wordt nooit iets naar het flashgeheugen van de ontvanger geschreven: dit overschrijft de opgeslagen waarde alleen voor de sessie.
settings-radio-use-receiver-s-stored-value = De opgeslagen waarde van de ontvanger gebruiken
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = DC over de coax voeden
settings-radio-not-every-hf-has-one = Niet elke HF+ heeft er een; op een ontvanger zonder doet dit niets.
settings-radio-host-dsp = Host-DSP
settings-radio-image-balancer-zero-if-offset = De beeldbalancer, de zero-IF-offset en de fijnafstemoscillator. Zet hem alleen uit om de ruwe hardware-uitvoer te zien — met uit verschijnt het spiegelbeeld op de zero-IF-snelheden en is de afstemming alleen nauwkeurig tot op de dichtstbijzijnde kilohertz.
settings-radio-correct-image-fine-tune = Het beeld corrigeren en fijnafstemmen
settings-radio-receive-only-no-soapysdr-no = Alleen ontvangst. Geen SoapySDR en geen libairspyhf nodig. Onder de ondergrens van de synthesizer doet de host-oscillator het afstemmen, waarmee deze ontvanger VLF bereikt. De ontvanger en samplefrequentie werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt.
settings-radio-not-yet-verified-against-real-3 = Nog niet geverifieerd met echte hardware. Als het zich misdraagt, stuur dan het diagnoserapport — het bevat elk commando dat met de ontvanger is uitgewisseld, en de eerste bytes van de samplestroom gedecodeerd als I/Q-paren.
settings-radio-no-airspy-r2-mini-found = Geen Airspy R2 of Mini gevonden — druk op Opnieuw scannen
settings-radio-read-receiver = gelezen van deze ontvanger
settings-radio-r2-mini-offer-different-rates = een R2 en een Mini bieden verschillende snelheden en zijn niet te onderscheiden totdat er een open is
settings-radio-rate-you-get-receiver-s = Dit is de snelheid die je krijgt. De ADC van de ontvanger draait op het dubbele — hij digitaliseert een reëel signaal en sdroxide maakt er op de host complexe basisband van.
settings-radio-gain-curve = Versterkingscurve
settings-radio-step-along-curve-above-not = Een stap langs de curve hierboven, geen dB-getal — de LNA, mixer en VGA van de tuner bewegen samen, en wat elke stap waard is, hangt af van de curve en de band. 0 is het stille eind.
settings-radio-tuner-agc = Tuner-AGC
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = Geeft de LNA aan de eigen lus van de tuner, en gaat voor op de curve.
settings-radio-mixer = Mixer
settings-radio-same-mixer-stage = Hetzelfde voor de mixertrap.
settings-radio-loop-running-gain-slider-no = Met een draaiende lus stelt de versterkingsschuif de trap die hij bezit niet meer in — de lus overschrijft hem.
settings-radio-dc-antenna-port = DC op de antenneaansluiting
settings-radio-t-12-bit-packing = 12-bits packing
settings-radio-third-less-usb-traffic-leave = Een derde minder USB-verkeer. Laat aan: dit is een USB 2.0-apparaat en de hoogste snelheid is 30 MB/s gepakt tegen 40 ongepakt. Werkt bij opnieuw verbinden.
settings-radio-dc-removal = DC-verwijdering
settings-radio-remove-adc-s-offset = De offset van de ADC verwijderen
settings-radio-turn-off-see-raw-hardware = Zet uit om de ruwe hardware-uitvoer te zien. Goed om te weten waar de spur terechtkomt: de offset belandt aan de rand van de span, niet in het midden, omdat het signaal onderweg een kwart van de samplefrequentie wordt verschoven.
settings-radio-every-command-exchanged-receiver-sample = Elk commando dat met de ontvanger is uitgewisseld, de samplefrequentieberekening en de eerste samples gedecodeerd als I/Q.
settings-radio-receive-only-24-1800-mhz = Alleen ontvangst, 24–1800 MHz. Geen SoapySDR en geen libairspy nodig. De ontvanger, samplefrequentie en packing werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt.
settings-radio-not-yet-verified-against-real-4 = Nog niet geverifieerd met echte hardware. Als het zich misdraagt, stuur dan het diagnoserapport — het bevat elk commando dat met de ontvanger is uitgewisseld, de snelheidsberekening en de eerste samples gedecodeerd als I/Q-paren.
settings-radio-no-hydrasdr-rfone-found-press = Geen HydraSDR RFOne gevonden — druk op Opnieuw scannen
settings-radio-these-are-rates-receiver-turned = dit zijn de snelheden die deze ontvanger bleek te hebben
settings-radio-receiver-only-reports-three-these = de ontvanger meldt er maar drie van deze; de rest staat in de alternatieve tabel van de firmware en een oudere build heeft ze mogelijk niet
settings-radio-rf-input = RF-ingang
settings-radio-antenna-sma-only-socket-bias = De antenne-SMA — de enige aansluiting met de bias-tee erachter.
settings-radio-cable-socket-no-bias-tee = Een kabelaansluiting. Hier geen bias-tee; de hardware zet die alleen op ANT.
settings-radio-powers-active-antenna-preamp-down-2 = Voedt een actieve antenne of voorversterker over de coax. Alleen de ANT-aansluiting heeft er een — de twee kabelpoorten zijn gewone ingangen.
settings-radio-third-less-usb-traffic-leave-2 = Een derde minder USB-verkeer. Laat aan: dit is een USB 2.0-apparaat en de hoogste snelheid is 36 MB/s gepakt tegen 48 ongepakt. Werkt bij opnieuw verbinden.
settings-radio-one-these-boards-1d50-60a1 = Een van deze borden zit op 1d50:60a1, de USB-id die de prototypes van HydraSDR delen met de Airspy R2 en Mini. sdroxide controleert de firmware na het openen en meldt het als de verkeerde interface is gekozen, in beide richtingen.
settings-radio-receive-only-24-1800-mhz-2 = Alleen ontvangst, 24–1800 MHz. Geen SoapySDR en geen libhydrasdr nodig. De ontvanger, samplefrequentie en packing werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt.
settings-radio-re-list-fobos-sdrs-no = Fobos SDR's opnieuw weergeven. Er wordt geen apparaat geopend, dus dit is veilig om in te drukken tijdens het ontvangen.
settings-radio-no-fobos-sdr-found-press = Geen Fobos SDR gevonden — druk op Opnieuw scannen
settings-radio-input-2 = Ingang
settings-radio-through-tuner-lna-vga-mixer = Via de tuner: LNA, VGA en het eigen frequentiebereik van de mixer.
settings-radio-direct-sampling-hf1-antenna-input = Directe sampling op de HF1-antenne-ingang — geen tuner, geen LNA/VGA-versterking, volledig in software afgestemd.
settings-radio-direct-sampling-hf2-antenna-input = Directe sampling op de HF2-antenne-ingang, hetzelfde als HF1.
settings-radio-both-real-adc-channels-once = Beide reële ADC-kanalen tegelijk, gecombineerd door het diversityfilter hieronder — een ruisbron weggenuld, of twee fadende paden gecombineerd.
settings-radio-direct-sampling-tuner-so-lna = Directe sampling — de tuner (en dus de LNA/VGA-versterking) zit niet in het pad. De snelheid hieronder is een doel voor de softwaredownconverter, geen hardware-instelling, dus de bereikte snelheid kan ervan afwijken.
settings-radio-receiver-s-own-reported-rates = de eigen gemelde snelheden van deze ontvanger
settings-radio-measured-one-real-unit-connect = gemeten op één echt exemplaar — verbind deze ontvanger en scan opnieuw om de eigen lijst te zien
settings-radio-clock-source = Klokbron
settings-radio-external-reference = Externe referentie
settings-radio-diversity-taps = Diversity-taps
settings-radio-taps = {" "}taps
settings-radio-one-tap-gain-phase-null = Eén tap is een versterking en een fase — een nul op één frequentie die aan beide kanten slechter wordt, wat alles is wat een analoge phaser kan. Elke extra tap koopt één sampleperiode van het padverschil tussen HF1 en HF2 dat het filter kan egaliseren, waardoor die notch een band wordt die helemaal stil is.
settings-radio-receive-only-rf-port-tuner = Alleen ontvangst. RF-poort (tuner), HF1/HF2 (directe sampling, in software gedecodeerd) en HF1+HF2 (beide tegelijk, gecombineerd door het diversityfilter) werken allemaal. De ontvanger, ingang en samplefrequentie werken na Toepassen; versterking, klokbron en taps worden toegepast zodra je ze wijzigt — de mode, adaptatiesnelheid en hold van het diversityfilter staan in het eigen DIV-vak van het hoofdvenster zodra deze radio is gekozen.
settings-radio-no-hackrf-found-press-rescan = Geen HackRF gevonden — druk op Opnieuw scannen
settings-radio-front-end-amplifier-8-db = Front-end-versterker, in stappen van 8 dB. Dit is de trap die de gevoeligheid verandert — en de trap die op een echte antenne als eerste overbelast raakt.
settings-radio-baseband-amplifier-after-mixer-2 = Basisbandversterker na de mixer, in stappen van 2 dB. Draai deze open voor een zwak signaal voordat je naar de LNA grijpt.
settings-radio-rf-amp = RF-versterker
settings-radio-one-switch-shared-transmit-setting = Eén schakelaar, gedeeld met de zendinstelling hieronder — de radio past degene toe die hoort bij de richting waarin hij overgaat. Uit is op een echte antenne meestal goed.
settings-radio-hackrf-pro-chooses-itself-three = Een HackRF Pro kiest dit zelf — driekwart van de samplefrequentie, gefilterd in de FPGA — en negeert alles wat de host vraagt.
settings-radio-leave-automatic-choosing-one-too = Laat op Automatisch. Een te smalle kiezen verzacht niet alleen de bandranden — het trekt de LO-offset in die de DC-piek van je signaal af houdt.
settings-radio-about-3-v-50-ma = Ongeveer 3 V bij 50 mA over de coax, voor een actieve antenne of een voorversterker. Alleen een HackRF One of Pro; de Jawbreaker en rad1o hebben zo'n schakeling niet.
settings-radio-remove-dc-mirror-image = DC en het spiegelbeeld verwijderen
settings-radio-zero-if-radio-so-own = Dit is een zero-IF-radio, dus zijn eigen LO-lek zit midden in de span en de kwadratuurfout van de mixer zet er een spiegelbeeld omheen. Dit uitzetten toont de ruwe hardware-uitvoer, de snelle manier om een stuurprogrammaprobleem van een DSP-probleem te onderscheiden.
settings-radio-corrects-reference-oscillator = Corrigeert de referentieoscillator.
settings-radio-transmit = Zenden
settings-radio-transmitter = Zender
settings-radio-off-default-while-off-backend = Standaard uit. Zolang dit uit staat, publiceert de backend helemaal geen zendkanaal, dus niets kan de radio keyen. Werkt bij opnieuw verbinden.
settings-radio-tx-vga-gain = TX-VGA-versterking
settings-radio-transmit-driver-amplifier-drive-applied = De zendsturingsversterker. De Drive wordt digitaal vóór deze trap toegepast, dus laat de drive hoog en stel het uitgangsniveau hier in — de drive juist verlagen laat de DAC op een fractie van de volle schaal draaien en verhoogt de intermodulatie.
settings-radio-same-switch-receive-setting-above = Dezelfde schakelaar als de ontvangstinstelling hierboven, toegepast wanneer de radio van richting verandert.
settings-radio-transmit-armed-into-dummy-load = Zenden is gewapend. Op een dummy load totdat je het hebt gemeten: de harmonischen van een HackRF zijn sterk genoeg om een extern laagdoorlaatfilter voor de band waarop je zit nodig te hebben, en hij is half duplex, dus de ontvangst stopt voor de duur van elke beurt.
settings-radio-every-command-exchanged-radio-session = Elk commando dat deze sessie met de radio is uitgewisseld, op volgorde, inclusief de reeks rond elke key-down.
settings-radio-dc-7-25-ghz-half = DC – 7,25 GHz, half duplex. Geen SoapySDR, geen libusb en geen libhackrf nodig. De radio en de samplefrequentie werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt. De bandranden zijn die van de firmware: een HackRF is gespecificeerd voor 1 MHz – 6 GHz (100 kHz bij een Pro) en wordt daarbuiten sterk gedempt, maar stemt er wel af.
settings-radio-ask-sdrplay-api-service-device = Vraag de SDRplay-API-service om zijn apparatenlijst. Er wordt niets geopend, dus dit is veilig om in te drukken tijdens het ontvangen.
settings-radio-no-rsps-press-rescan-needs = geen RSP's — druk op Opnieuw scannen (vereist de SDRplay-API-service)
settings-radio-both-tuners-running-api-fixes = Met beide tuners actief zet de API de ADC vast op 6 MHz en geeft 2 Msps terug vanaf een lage F.I. — dus 2 Msps is de breedste span, en de smallere zijn dat gedecimeerd. Werkt na Toepassen.
settings-radio-rates-below-2-msps-run = Snelheden onder 2 Msps laten de ADC op 2 Msps draaien en decimeren in de service. Werkt na Toepassen.
settings-radio-reduced-adc-resolution = {"  "}(verminderde ADC-resolutie)
settings-radio-if-bandwidth = F.I.-bandbreedte
settings-radio-tuner-s-analog-filter-auto = Het analoge filter van de tuner. Auto kiest het breedste dat bij de samplefrequentie past. Werkt na Toepassen.
settings-radio-rsp-s-own-if-gain = De eigen F.I.-versterkingslus van de RSP, aangestuurd door de API-service. Uit geeft de F.I.-versterkingsschuif aan jou terug — de instelling voor metingen en digitale modes voor zwakke signalen.
settings-radio-agc-set-point = AGC-instelpunt
settings-radio-signal-level-loop-holds-adc = Signaalniveau waarop de lus de ADC houdt. Lager laat meer speling voor signalen buiten het kanaal. Hoe laag het mag, hangt af van de samplefrequentie — boven 8,064 Msps heeft de converter minder speling te geven, en boven 9,216 Msps nog minder.
settings-radio-rsp-s-native-gain-unit = De oorspronkelijke versterkingseenheid van de RSP: 20 dB is maximale versterking (0 met het uitgebreide bereik hieronder), 59 dB minimaal. Werkt direct. Genegeerd zolang de AGC loopt — de lus bezit dan deze waarde, en de S-meter toont waarop hij uitkwam.
settings-radio-extended-if-range = Uitgebreid F.I.-bereik
settings-radio-lets-if-gain-reduction-go = Laat de F.I.-versterkingsreductie onder 20 dB gaan, tot 0 — de laatste 20 dB versterking die de ontvanger heeft. Uit is de eigen standaard van de API en de juiste voor gewoon luisteren, omdat de onderkant van het bereik is waar een RSP het makkelijkst overbelast raakt. De moeite waard voor zwakke signalen met de LNA al op 0. Het laat ook het AGC-instelpunt tot 0 dBFS oplopen. Werkt direct.
settings-radio-allow-if-gain-reduction-below = F.I.-versterkingsreductie onder 20 dB toestaan
settings-radio-front-end-attenuation-steps-0 = Front-end-demping in stappen: 0 is maximale versterking, elke stap omhoog schakelt meer demping in. Sommige banden hebben minder stappen — het stuurprogramma begrenst en houdt je keuze vast voor als je terug afstemt, en de rail hier eindigt bij wat de band waarop de radio zit toelaat. Een hogere keuze op een andere band blijft bewaard en wordt er nog naast getoond. Werkt direct.
settings-radio-t-0-max-gain = 0 = max. versterking
settings-radio-reference-error-parts-per-million-2 = Referentiefout in parts per million, toegepast door het apparaat zelf. Werkt direct.
settings-radio-radio-s-tuner = De tuner van deze radio
settings-radio-tuner = Tuner
settings-radio-which-rspduo-s-two-tuners = Op welke van de twee tuners van de RSPduo deze radio luistert. De andere hoort bij de radio die ervoor is geconfigureerd — geef die dezelfde ontvanger en de andere tuner. Werkt na Toepassen.
settings-radio-which-rspduo-s-two-tuners-2 = Welke van de twee tuners van de RSPduo de antenne draagt waarnaar je luistert. De andere draagt de tweede antenne. Werkt na Toepassen.
settings-radio-which-rspduo-s-two-tuners-3 = Welke van de twee tuners van de RSPduo moet draaien (één tegelijk). Werkt na Toepassen.
settings-radio-applies-immediately = Werkt direct.
settings-radio-dab-notch = DAB-notch
settings-radio-hardware-notch-over-165-230 = Hardwarenotch over de 165–230 MHz DAB-band. Werkt direct.
settings-radio-hdr-mode = HDR-mode
settings-radio-enable-below-2-mhz = Inschakelen onder 2 MHz
settings-radio-rspdx-s-high-dynamic-range =
    Het high-dynamic-range-pad van de RSPdx voor LF/MF. Werkt direct.

    Het werkt nog niet: op de ene RSPdx waarop het is geprobeerd, maakt aanzetten de ontvanger stil — bij elk geprobeerd midden, filter, poort, versterking en snelheid. Wat de mode nodig heeft, staat niet op het gedocumenteerde API-oppervlak, en het te vinden is iemands volgende klus. Laat dit uit tenzij jij degene bent die zoekt.

    Wat bekend is: het is geen mode die de afstemming volgt. Het pad heeft een vast analoog filter, alleen gebouwd voor de middens die onder het filter hieronder staan, dus ergens anders afstemmen zou niet kunnen werken, zelfs niet als de rest ooit werkt.
settings-radio-hdr-filter = HDR-filter
settings-radio-feed-4-7-v-dc = ~4,7 V DC over de coax voeden
settings-radio-second-tuner = Tweede tuner
settings-radio-rspduo-two-whole-tuners-one = Een RSPduo is twee volledige tuners op één bord, geklokt vanaf één referentie. Draai beide en ze horen hun spans op hetzelfde moment van dezelfde klok — wat twee antennes erop coherent maakt, met een relatieve fase die door de voedingslijnen wordt bepaald in plaats van door toeval, en het paar laat combineren. Of laat ze los van elkaar: de tuners stemmen apart af, dus de andere kan een tweede radio op zijn eigen band zijn.
settings-radio-run-both-tuners = Beide tuners laten draaien
settings-radio-puts-rspduo-api-s-dual = Zet de RSPduo in de dual-tunermode van de API, waar de ADC vast op 6 MHz staat en de breedste span 2 Msps is. Werkt na Toepassen, omdat de mode wordt gekozen wanneer het apparaat wordt geopend.
settings-radio-used = Gebruikt voor
settings-radio-what-other-tuner-takes-effect = Waar de andere tuner voor dient. Werkt na Toepassen — net als beide laten draaien wordt dit gekozen wanneer het bord wordt geopend.
settings-radio-add-second-radio-settings-radio = Voeg een tweede radio toe (Instellingen → Radio → +), geef die dezelfde ontvanger en de andere tuner van de RSPduo, en stel hem in om ook beide tuners te laten draaien — de radio die het bord opent zet het in de dual-tunermode, dus beide moeten het verwachten. De twee stemmen dan onafhankelijk af: HF in het ene tabblad en VHF in het andere, van één bord. Ze delen één ADC-klok, dus beide draaien op de samplefrequentie waar de radio die het bord opende om vroeg, en geen van beide kan zenden.
settings-radio-what-do = Wat ermee te doen
settings-radio-lna-state-2 = Zijn LNA-stand
settings-radio-second-tuner-s-own-front = De eigen front-end-demping van de tweede tuner. Stel in zodat beide antennes ongeveer dezelfde ruisvloer tonen: dit is de afstelling waar al het andere op rust, omdat het combineren de twee takken weegt naar hun ruis, en een tweede front-end dat in overbelasting wordt gedreven het filter een vervormde kopie van de storing geeft — die niet van een onvervormde kan worden afgetrokken. Werkt direct.
settings-radio-if-gain-reduction-2 = Zijn F.I.-versterkingsreductie
settings-radio-second-tuner-s-if-gain = De F.I.-versterking van de tweede tuner, in de oorspronkelijke eenheid van de RSP: 20 dB is maximale versterking. Genegeerd zolang de AGC loopt — en een stabiele versterking is wat het filter wil, dus de AGC uitzetten loont voor een nul die je wilt vasthouden.
settings-radio-filter-length = Filterlengte
settings-radio-one-tap-gain-phase-null-2 = Eén tap is een versterking en een fase — een nul op één frequentie die aan beide kanten slechter wordt, wat alles is wat een analoge phaser kan. Elke extra tap koopt één sampleperiode van het padverschil tussen de twee antennes dat het filter kan egaliseren, waardoor die notch een band wordt die helemaal stil is.
settings-radio-adaptation-2 = Adaptatie
settings-radio-slow-steady-left-converging-inside = Langzaam en gestaag links, binnen een fractie van een seconde convergerend en zichtbaar zoekend rechts. Begin snel om de nul te vinden, houd hem dan vast.
settings-radio-stop-filter-moving-reach-moment = Laat het filter stoppen met bewegen. Grijp hiernaar op het moment dat een nul verschijnt: een filter dat blijft adapteren richt zich opnieuw op wat het luidst wordt, wat op een stille band het station is waarnaar je luistert.
settings-radio-restart = Herstarten
settings-radio-zero-filter-find-null-again = Het filter op nul zetten en de nul opnieuw vinden.
settings-radio-nothing-here-can-tell-wanted = Niets hier kan een gewenst signaal van een ongewenst onderscheiden — het filter weet alleen wat de twee antennes gemeen hebben. Bij Cancel wil de tweede antenne de ruisbron horen en zo min mogelijk van de band, anders annuleert hij braaf ook het station. Bij Combine willen beide hetzelfde station horen. Hoe diep de nul wordt, gaat om de paar seconden naar het log.
settings-radio-not-yet-verified-against-rspduo = Nog niet geverifieerd met een RSPduo: de dual-tunerwerking hier is geschreven vanuit de API van SDRplay en niet op de hardware gemeten.
settings-radio-no-receiver-listed-check-against = Er is geen ontvanger vermeld om dit tegen te controleren — druk op Opnieuw scannen, of lees het log voor wat er werkelijk opende.
settings-radio-receive-only-1-khz-2 = Alleen ontvangst, 1 kHz–2 GHz. Vereist de SDRplay-API-service van de fabrikant (sdrplay.com/api) — de RSP's na de oorspronkelijke RSP1 hebben geen open protocol. Apparaat, samplefrequentie, bandbreedte en RSPduo-tuner werken na Toepassen; al het andere wordt toegepast zodra je het wijzigt.
settings-radio-board = Bord
settings-radio-first-one-found-2 = Eerste gevonden
settings-radio-receive-chain = Ontvangstketen
settings-radio-both-chains-tune-together-they = Beide ketens stemmen samen af — ze delen één synthesizer — maar het zijn aparte front-ends op aparte aansluitingen. Kies degene waarop je antenne zit, de instelling om naar te grijpen als de ene keten de HF-aanpassingsmodificatie heeft gehad en de andere standaard is.
settings-radio-receive-gain = Ontvangstversterking
settings-radio-one-combined-figure-which-limesuite = Eén gecombineerd getal, dat LimeSuite zelf over de LNA, de TIA en de PGA verdeelt. Het neemt hele decibels, dus alles fijners wordt afgekapt.
settings-radio-receive-port = Ontvangstpoort
settings-radio-automatic-follows-frequency-lnal-low = Automatisch volgt de frequentie: LNAL laag, LNAH hoog — tenzij hieronder een LimeRFE is aangesloten, wat één kabel in één aansluiting is, en dan is het LNAW op elke frequentie. Noem de aansluiting waarop de jouwe is bedraad als die het niet is. Bij welke keten de aansluiting hoort, bepaalt de keuze hierboven.
settings-radio-t-0-follows-sample-rate-worth = 0 volgt de samplefrequentie. De moeite waard om daar te laten: een filter smaller dan een kwart van de span trekt stilzwijgend de zero-IF-LO-offset in, wat het LO-lek weer bovenop het signaal zet waarnaar je luistert.
settings-radio-following-rate = volgt de snelheid
settings-radio-corrections = Correcties
settings-radio-host-iq-dc-correction = Host-IQ-/DC-correctie
settings-radio-adaptive-image-dc-removal-side = Adaptieve beeld- en DC-verwijdering aan deze kant, bovenop de eigen kalibratie van de chip. Uitzetten is de ene-klik-manier om een stuurprogrammaprobleem van een DSP-probleem te onderscheiden.
settings-radio-calibrate-automatically = Automatisch kalibreren
settings-radio-runs-chip-s-own-dc = Draait de eigen DC-offset- en beeldkalibratie van de chip wanneer de radio wordt geopend, en opnieuw zodra de afstemming zich op een nieuwe band of een andere aansluiting heeft gezet. Die getallen zijn op één frequentie gemeten en zijn elders fout, wat een draaggolf in het midden van de span meestal is. Kost elke keer ongeveer een seconde.
settings-radio-calibrate-now = Nu kalibreren
settings-radio-stalls-receiver-better-part-second = Houdt de ontvanger het grootste deel van een seconde stil.
settings-radio-second-aerial = Tweede antenne
settings-radio-socket = Zijn aansluiting
settings-radio-same-first = Dezelfde als de eerste
settings-radio-gain-2 = Zijn versterking
settings-radio-set-low-coupled-sample-your = Stel dit LAAG in. Het gekoppelde monster van je eigen zender is een sterk signaal, en een terugkoppelketen die in compressie wordt gedreven meet de curve van de versterker verkeerd — ze leert de correctie zijn eigen vervorming. Begin onderaan en gebruik de verzwakker van de koppelaar.
settings-radio-set-so-both-aerials-show = Stel in zodat beide antennes ongeveer dezelfde ruisvloer tonen. Dit is de afstelling waar het geheel op rust: het combineren weegt de twee takken naar hun ruis, en een tweede keten die in compressie wordt gedreven geeft het filter een vervormde kopie van de storing — die niet van een onvervormde kan worden afgetrokken.
settings-radio-table-steps-2 = Tabelstappen
settings-radio-how-finely-correction-follows-amplifier = Hoe fijn de correctie de curve van de versterker volgt. Meer stappen volgen een scherpere knik, maar elke stap moet worden geleerd uit de samples die erin zijn beland — en de top van een spraakamplitudehistogram is dun. Tweeëndertig is genoeg voor de gladde curve die een HF-versterker werkelijk heeft. Wijzigen start de correctie opnieuw.
settings-radio-how-hard-each-block-feedback = Hoe hard elk blok terugkoppeling de correctie verplaatst. De curve van een versterker verandert niet, dus haast is niet nodig — het midden middelt de ruis van meerdere beurten eruit.
settings-radio-keep-correction-curve-learned-clean = Houd de correctie zoals ze is. Een curve die op een schone beurt is geleerd, is het waard om vast te houden, en de versterker zal bij de volgende niet zijn veranderd.
settings-radio-forget-correction-learn-again = De correctie vergeten en opnieuw leren.
settings-radio-directional-coupler-amplifier-s-output = Een richtkoppelaar op de uitgang van de versterker gaat in deze keten, en de zender vergelijkt wat terugkwam met wat hij wilde zenden — en stuurt dan het inverse van het verschil, zodat wat de versterker verlaat recht is. Twintig-en-nog-wat decibel minder intermodulatie op de QSO's van anderen, terwijl de versterker zijn vermogen houdt. De correctie blijft op unity totdat de terugkoppeling met de zending in lijn is, dus een koppelaar die niet is aangesloten kost niets; en hij kan de converter nooit om meer dan de volle schaal vragen, dus een terugkoppelpad dat onzin leest, kan niets overdrijven. Hoe het gaat, loopt naar het log terwijl je zendt.
settings-radio-transmit-not-armed-so-there = Zenden is niet gewapend, dus er is hier niets te corrigeren.
settings-radio-off-interface-publishes-no-transmit = Met dit uit publiceert de interface helemaal geen zendkanaal, dus niets kan de radio keyen.
settings-radio-transmit-gain = Zendversterking
settings-radio-transmit-port = Zendpoort
settings-radio-board-s-two-transmit-sockets = De twee zendaansluitingen van het bord zijn twee verschillende aanpassingsnetwerken, geen twee bussen op hetzelfde: BAND1 (TX_1) draagt 30 MHz tot 1,9 GHz en BAND2 (TX_2) is de 13 cm-poort. Een amateurband onder 23 cm via BAND2 keyen zet de beurt in een aanpassingsnetwerk dat er bijna niets van doorlaat — elke instelling lijkt correct, de drive is wat je instelde, en de vermogensmeter blijft op nul. Laat dit op Automatisch staan en het volgt de afstemming.
settings-radio-limesdr-transmits-about-100-khz = Een LimeSDR zendt van ongeveer 100 kHz tot 3,8 GHz zonder eigen filtering. Gebruik een laagdoorlaatfilter, een passend LimeRFE-kanaal of een dummy load.
settings-radio-limerfe-front-end = LimeRFE-front-end
settings-radio-connected = Verbonden via
settings-radio-limerfe-s-own-micro-usb = De eigen micro-USB-poort van de LimeRFE, niet die van de radio. 9600 baud, vastgelegd door zijn firmware.
settings-radio-receive-connector = Ontvangstconnector
settings-radio-transmit-connector = Zendconnector
settings-radio-follow-dial = De afstemming volgen
settings-radio-switch-filters-match-operating-frequency = Schakel de filters om aan te sluiten bij de gebruiksfrequentie, voordat er enige RF verschijnt. Afstemmen binnen één band zet niets op de besturingsverbinding.
settings-radio-relays = Relais
settings-radio-receive-attenuator = Ontvangstverzwakker
settings-radio-notch-filter = Notchfilter
settings-radio-fan = Ventilator
settings-radio-worth-having-any-sustained-transmitting = De moeite waard om aan te zetten bij elk langdurig zenden.
settings-radio-automatic-board-receives-switched-transmit = Op Automatisch ontvangt het bord, en wordt het bij key-down naar zenden geschakeld en bij key-up terug — op beide bekabelingen. Zijn amateurkanalen hebben één filter met aan weerszijden een zend/ontvangstschakelaar, dus een bord dat om beide tegelijk wordt gevraagd zet die schakelaar op de zender en hoort niets meer. Het 30 MHz-kanaal is alleen bereikbaar via J5, één connector voor beide richtingen; erboven houdt zenden vanaf J4 het ontvangstpad weg van de connector die de versterker aanstuurt.
settings-radio-limesdr-support-has-not-been = LimeSDR-ondersteuning is niet met hardware geverifieerd. Dit is de trace van de laatste sessie, voor een probleemmelding.
settings-radio-gains-filters-corrections-antenna-sockets = Versterkingen, filters, correcties, de antenneaansluitingen en elke LimeRFE-regeling — zijn connectoren, band, relais, verzwakker, notch en ventilator — werken direct. Het bord, de ontvangstketen, de samplefrequentie, het wapenen van zenden en de verbinding van de LimeRFE werken na Toepassen.

## Settings window, Radio tab: messages with values

settings-radio-fill-from = VULLEN VANUIT { $name }
settings-radio-decimation-auto = Automatisch (dichtstbijzijnde { $khz } kHz)
settings-radio-decimation-stage-server = Trap { $stage } — de samplefrequentie van de server ÷ { $divisor }
settings-radio-decimation-stage = Trap { $stage } — samplefrequentie ÷ { $divisor }
settings-radio-kiwi-receive-only = Alleen ontvangen — dit is de antenne van iemand anders. De panadapter is de ~12 kHz die de ontvanger als I/Q stuurt en die de afstemming volgt; de bandweergave erboven is een eigen waterval, en erdoorheen afstemmen stemt de ontvanger opnieuw af. Ongeveer { $rate } kB/s zolang er verbinding is. Druk op "Apply / reconnect" om te wisselen zonder herstart.
settings-radio-connected-detail = Verbonden: { $detail }
settings-radio-failed = Mislukt: { $error }
settings-radio-pluto-buffer = ≈ { $ms } ms, { $kib } KiB per overdracht
settings-radio-driver-settings = Instellingen { $driver }
settings-radio-rx888-coverage = Bereik 0–{ $mhz } MHz, { $mbytes } MB/s via USB
settings-radio-rx888-digitised = van de { $mhz } MHz gedigitaliseerd
settings-radio-elad-16-bit = { $khz } kHz (16-bit samples)
settings-radio-elad-pad = { $db } dB pad ingeschakeld
settings-radio-hackrf-rx-preamp = { $db } dB voorversterker bij ontvangst
settings-radio-hackrf-tx-preamp = { $db } dB voorversterker bij zenden
settings-radio-device-not-found = { $device } (niet gevonden)
settings-radio-lime-chain = Keten { $n } (RX{ $n }_*)
settings-radio-lime-chain-tx = Keten { $n } (RX{ $n }_* / TX{ $n }_*)
settings-radio-lime-second-chain = De andere ontvangstketen van het bord, op de { $sockets }-aansluitingen. Hij deelt de synthesizer, dus hij hoort hetzelfde spectrum op hetzelfde moment als de eerste — waardoor hij een tweede antenne kan voeden, of een monster van je eigen zender.
settings-radio-tx-blocked = Zenden is geblokkeerd: { $reason }
settings-radio-rfe-rx-fallback = Bij ontvangst op { $port } vallen deze terug op het ongefilterde breedbandpad: { $bands }.
settings-radio-rfe-tx-fallback = Bij zenden op { $port } vallen deze terug op het breedbandpad — geen bandversterker en geen filtering: { $bands }.

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = Volume, versterking, AGC, squelch, de ruisbesturing en de RDS-weergave bij FM-omroep
topbar-receiver = ONTVANGER
topbar-vfo-b-split-rit-xit = VFO A/B, split en de RIT/XIT-offsets
topbar-tuning = AFSTEMMING
topbar-diversity-filter-which-way-combines = Het diversity-filter: hoe het de twee antennes combineert, hoe snel het zich aanpast en het vasthouden op de huidige stand
topbar-diversity = DIVERSITY
topbar-second-receiver-s-frequency-mode = Frequentie, modus, filter en niveau van de tweede ontvanger
topbar-sub-receiver = SUBONTVANGER
topbar-transceiver-s-own-aerial-socket = De eigen antenne-aansluiting van de transceiver en de aan/uit-schakelaar
topbar-radio = RADIO
topbar-tune-voice-keyer-drive-mic = Tune, de voice keyer en de drive- en microfoonniveaus
topbar-transmit = ZENDEN
topbar-tune = {" "}TUNE{" "}
topbar-display = WEERGAVE
topbar-logbook-spots-awards-memories-scanner = Logboek, spots, awards, geheugens, de scanner, instellingen en de handleiding
topbar-system = SYSTEEM
topbar-linked = Gekoppeld
topbar-not-linked = Niet gekoppeld
topbar-bar-trace = balk / spoor
topbar-needle-bar-trace = naald / balk / spoor
topbar-swap-vfos = VFO's wisselen
topbar-copy-b = A naar B kopiëren
topbar-second-receiver-right-ear-tunes = Tweede ontvanger, in het rechteroor. Hij stemt onafhankelijk van A/B af — zijn regelaars staan in de SUB-module en zijn doorlaatband in de waterval.
topbar-band-plan = , uit het bandplan
topbar-repeater-shift-following-band-plan = Repeaterverschuiving: volgt het bandplan, dat voor deze frequentie geen verschuiving kent — simplex
topbar-repeater-shift-transmit-above-below = Repeaterverschuiving: zenden boven of onder de afstemming om een repeater te werken. Nu simplex
topbar-repeater-shift = REPEATERVERSCHUIVING
topbar-auto = AUTO
topbar-take-shift-band-plan-dial = Neem de verschuiving uit het bandplan over terwijl de afstemming beweegt. Dit werkt alleen binnen een repeater-uitgangssubband — overal elders blijft de radio simplex, zodat de oproepkanalen simplex blijven.
topbar-offset = Offset
topbar-repeater-tone-ctcss-dcs-goes = Repeatertoon: de CTCSS/DCS die onder de spraak mee uitgaat, de 1750 Hz-burst en de toonsquelch bij ontvangst
topbar-transmit-tone = ZENDTOON
topbar-set-here-sent-nfm-sub = hier ingesteld, verzonden op NFM — sub-hoorbare signalering hoort bij een FM-kanaal
topbar-normal = NORMAAL
topbar-invert = INVERTEREN
topbar-bit-order-dcs-encoded-here = de bitvolgorde waarmee DCS hier wordt gecodeerd is overgenomen uit de standaard en nooit bij een repeater getest — als hij niet opent, probeer dan CTCSS
topbar-t-1750-hz-burst = 1750 HZ BURST
topbar-every-over = ELKE BEURT
topbar-open-burst-every-time-transmitter = Openen met de burst telkens als de zender wordt ingeschakeld
topbar-send = ZENDEN
topbar-send-burst-now-keying-transmitter = Verstuur de burst nu — de zender wordt voor de duur ervan ingeschakeld als hij dat nog niet is
topbar-nfm-only-burst-fm-repeater = Alleen NFM: de burst is de deuropener van een FM-repeater
topbar-receive-tone-squelch = TOONSQUELCH BIJ ONTVANGST
topbar-require-receive-what-station-transmits = Vereis bij ontvangst wat dit station zendt
topbar-hide-docked-band-selector-click = Verberg de vastgezette bandkeuze — klik opnieuw om hem terug te halen
topbar-show-docked-band-selector = Toon de vastgezette bandkeuze
topbar-band-mode = BAND & MODUS
topbar-hide-band-chip-brings-back = Verbergen — de bandchip brengt hem terug
topbar-dec-off = DEC uit
topbar-vol = Vol
topbar-counted-steps-not-decibels-what =


    In stappen geteld, niet in decibel: wat één stap waard is bepaalt de ontvanger zelf, en bij een SDRplay hangt het ook van de band af. Rechts is nog steeds meer versterking.
topbar-gain = Versterking
topbar-plutosdr-s-own-agc-setting =


    De eigen AGC van de PlutoSDR bepaalt deze versterking, dus de schuifregelaar verplaatsen doet niets. Zet Settings → Radio → AGC op Manual om het hier te regelen.
topbar-agc-hang-time-click-cycle = AGC-hangtijd — klik om te wisselen: Uit / Langzaam / Midden / Snel
topbar-man = Hand
topbar-manual-audio-gain-used-while = Handmatige audioversterking, gebruikt zolang de AGC uit staat. Gestart met het niveau dat de AGC vasthield toen hij werd uitgeschakeld.
topbar-radio-s-own-squelch-sent = De eigen squelch van de radio, verstuurd via de besturingsverbinding. Dit is de poort waar het geluid werkelijk doorheen gaat — de software-squelch zou alleen verder dichtgaan op wat de radio al doorliet.
topbar-max-gain = max. versterking
topbar-manual-gain = handmatige versterking
topbar-noise-reduction = ruisonderdrukking
topbar-impulse-noise-blanker = Impulsruisonderdrukker
topbar-auto-notch-cancel-constant-tone = Auto-notch: onderdrukt constante tonen (zwevingen)
topbar-binaural-audio-not-while-sub = Binauraal geluid — niet zolang de subontvanger het rechteroor heeft
topbar-binaural-audio-passband-spread-across = Binauraal geluid: de doorlaatband wordt over beide oren verdeeld, zodat signalen met verschillende toonhoogte uit verschillende richtingen komen en het signaal waarop je afstemt over het beeld zweeft. Klik om terug te gaan naar mono
topbar-binaural-cw-spread-passband-across = Binaurale CW: verdeel de doorlaatband over beide oren, zodat signalen met verschillende toonhoogte uit verschillende richtingen komen — een pile-up wordt meerdere plaatsen in plaats van één drukke toon, en afstemmen op een station laat het over het beeld zweven. Het best met een hoofdtelefoon
topbar-binaural-audio-spread-passband-across = Binauraal geluid: verdeel de doorlaatband over beide oren. Bij spraak spreidt de ruis zich over het hele beeld terwijl het station in het midden blijft, wat een uur lang luisteren makkelijker maakt — ten koste van de spraak zelf, die uitgesmeerd wordt. Het best met een hoofdtelefoon
topbar-mute = DEMPEN
topbar-record-audio-raw-i-q = Neem het geluid, de ruwe I/Q of beide op
topbar-wfm-stereo-forced-off-click = WFM-stereo geforceerd uit — klik voor automatische stereo
topbar-wfm-stereo-pilot-locked-click = WFM-stereo: pilootvergrendeld. Klik om mono te forceren
topbar-wfm-stereo-automatic-no-pilot = WFM-stereo: automatisch, geen pilootsignaal op dit station
topbar-rds-data-arriving-click-see = Er komen RDS-gegevens binnen — klik om ze te bekijken
topbar-rds-no-data-station-click = RDS — geen gegevens op dit station. Klik toch voor de diagnostiek van de decoder
topbar-drm-decoding-click-details = DRM wordt gedecodeerd — klik voor de details
topbar-drm-click-decoder-s-state = DRM — klik voor de status van de decoder
topbar-hd-radio-decoding-click-details = HD Radio wordt gedecodeerd — klik voor de details
topbar-hd-radio-click-decoder-s = HD Radio — klik voor de status van de decoder
topbar-presets = VOORINSTELLINGEN
topbar-width-per-sideband = BREEDTE PER ZIJBAND
topbar-width = BREEDTE
topbar-passband-width-hertz-type-drag = Breedte van de doorlaatband in hertz — typ hem of sleep. Klik in het veld om een exacte waarde in te voeren.
topbar-edges = RANDEN
topbar-low-edge-hz-carrier = Onderrand, in Hz vanaf de draaggolf
topbar-high-edge-hz-carrier = Bovenrand, in Hz vanaf de draaggolf
topbar-noise-reduction-voice-off-click = Ruisonderdrukking (spraak), uit — klik om een engine te kiezen
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = Wat je hoort, als MP3: de ontvanger in het ene kanaal en je eigen zending in het andere. Dit is de opname van een QSO.
topbar-mono = MONO
topbar-mixed-one-channel-click-two = Gemengd tot één kanaal — klik voor twee
topbar-two-channels-rx-left-tx = Twee kanalen: RX links, TX rechts — klik voor één gemengd kanaal
topbar-quick-clip = SNELLE CLIP
topbar-stop-after = STOPPEN NA
topbar-no-stop = niet stoppen
topbar-auto-record = AUTO-OPNAME
topbar-set-squelch-auto-record-needs = Stel een squelch in — auto-opname heeft er een nodig om stilte te herkennen
topbar-auto-record-needs-sdr-front = Auto-opname heeft een SDR-frontend met een softwaresquelch nodig
topbar-stop-following-squelch = Stop met de squelch volgen
topbar-waiting-signal = wacht op een signaal
topbar-record-spectrum = SPECTRUM OPNEMEN
topbar-i-q-wav = ● I/Q WAV
topbar-i-q-wav-2 = I/Q WAV
topbar-raw-spectrum-receiver-delivering-stereo = Het ruwe spectrum dat de ontvanger levert, als stereo 32-bit float WAV (RF64 boven 4 GB) dat SDR#, SDRuno, HDSDR en SDRangel openen — en dat sdroxide zelf afspeelt met --file. Groot: 8 bytes per sample.
topbar-radio-hands-over-demodulated-audio = Deze radio levert gedemoduleerd geluid, dus er is geen I/Q om op te nemen.
topbar-what-capture-costs-sample-rate = Wat een opname kost bij deze samplefrequentie
topbar-engine = ENGINE
topbar-no-noise-reduction-decoders-never = Geen ruisonderdrukking — de decoders zagen die toch nooit
topbar-strength = STERKTE
topbar-tone = TOON
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS — geen sub-hoorbare toon op dit signaal
topbar-tone-squelch = TOONSQUELCH
topbar-carrier-squelch-open-any-signal = Draaggolfsquelch: opent bij elk signaal
topbar-require-tone-currently-being-received = Vereis de toon die nu wordt ontvangen
topbar-ctcss = CTCSS
topbar-any-dcs = ELKE DCS
topbar-open-any-dcs-coded-signal = Opent bij elk DCS-gecodeerd signaal. Welke van de 104 codes het draagt is hier niet betrouwbaar te lezen, dus er valt niets fijners te kiezen
topbar-which-socket-back-radio-receiving =
    Op welke aansluiting aan de achterkant de radio ontvangt — zijn eigen ANT-commando, dezelfde instelling als de ANT-knop op het voorpaneel. Klik om naar de volgende te gaan.

    De keuze wordt per band onthouden en teruggezet zodra de afstemming die band weer binnengaat.
topbar-radio-s-separate-receiving-antenna =
    De aparte ontvangstantenne van de radio, in of uit het ontvangstpad geschakeld — zijn eigen RX ANT-instelling. De antenne op de hoofdaansluiting blijft in beide gevallen voor het zenden.

    De radio onthoudt dit zelf per band, dus sdroxide leest het na elke bandwissel terug in plaats van terug te zetten wat het laatst zag: alleen een klik hier verandert het.
topbar-radio-s-own-power-switch =
    De eigen aan/uit-schakelaar van de radio, via de besturingsverbinding, en de enige echte aan/uit in het programma — niet de LINK-schakelaar van sdroxide, die alleen de kant van sdroxide sluit en de radio laat draaien.

    Om AAN iets te laten bereiken moet de besturingskant van de radio wakker blijven terwijl hij uit staat: Network Control via het LAN, of een CI-V-poort die nog van netspanning wordt voorzien bij een toestel dat aan de voorkant is uitgezet.
topbar-switch-radio = Schakel de radio in
topbar-switch-radio-off-audio-meters = Schakel de radio uit. Het geluid en de meters stoppen daarmee.
topbar-what-second-aerial-click-swap =
    Waar de tweede antenne voor dient — klik om te wisselen.

    CANCEL trekt hem van de eerste af, met de versterking, fase en vertraging waarmee een lokale ruisbron op beide gelijk komt te liggen: de DSP-vorm van een ruisonderdrukkende phaser. COMBINE telt de twee op in de fase die versterkt, gewogen naar hoe goed elk hoort — diversity-ontvangst, die fading opvult.
topbar-hold = VASTHOUDEN
topbar-stop-filter-moving-reach-moment = Laat het filter stoppen met bewegen. Grijp hiernaar op het moment dat er een null verschijnt: een filter dat blijft adapteren richt zich opnieuw op wat het luidst wordt, en op een stille band is dat het station waarnaar je luistert.
topbar-restart = HERSTARTEN
topbar-zero-filter-find-null-again = Zet het filter op nul en zoek de null opnieuw.
topbar-adapt = Adapteren
topbar-how-fast-filter-chases-slow = Hoe snel het filter volgt: langzaam en gestaag links, binnen een fractie van een seconde convergerend en zichtbaar zoekend rechts. Begin snel om de null te vinden, en HOUD hem dan vast.
topbar-where-sub-receiver-listens-shift = Waar de subontvanger luistert. Shift-klik op de waterval, of sleep binnen de doorlaatband van de sub, om hem te verplaatsen.
topbar-dial = ←AFSTEMMING
topbar-move-sub-receiver-main-dial = Verplaats de subontvanger naar de hoofdafstemming
topbar-dial-2 = AFSTEMMING←
topbar-move-main-dial-sub-receiver = Verplaats de hoofdafstemming naar de subontvanger
topbar-filter = Filter
topbar-sub-receiver-passband-edges-hz = Doorlaatbandranden van de subontvanger, in Hz
topbar-sub-receiver-level-plays-right = Niveau van de subontvanger (hij speelt in het rechteroor)
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = Voice keyer: opgeslagen berichten opnemen en zenden
topbar-drive = Drive
topbar-tune-2 = Tune
topbar-mic = Mic
topbar-cessb = CESSB
topbar-microphone-gain = Microfoonversterking
topbar-deviation-how-far-mode-s =
    Deviatie: hoe ver de burst van deze modus uitslaat bij een radio die zelf moduleert. Een FM-zender zet audioniveau om in frequentiezwaai en heeft geen ALC om dat op te vangen, dus volle schaal op een data-ingang die voor spraak is ingesteld geeft overdeviatie — wat voor een luisteraar volkomen normaal klinkt en voor niemand decodeert.

    Per modus bewaard, zodat een deviatie voor 1200 baud packet nooit op FT8 terechtkomt.
topbar-transmit-audio-how-hard-mode =
    Zendaudio: hoe hard deze modus de modulator aanstuurt van een radio die moduleert wat wij hem sturen — een CAT-rig op zijn geluidskaart, een FLEX, een Icom op zijn netwerkpoort. Verlaag tot de ALC van de rig nauwelijks beweegt en stel het vermogen in op de radio; ALC die op een digitale modus met constante envelop rijdt veroorzaakt splatter.

    Drive is niet deze regelaar: bij deze radio's bereikt Drive het vermogensregister van de rig en raakt het nooit zijn audio.

    Per modus bewaard — FT8, RTTY, PSK en MCW houden elk hun eigen waarde.
topbar-tx-audio = TX-audio
topbar-cw-psk-rtty-skimmers-decode = CW / PSK / RTTY-skimmers — decoderen signalen over de hele band en markeren ze op de waterval
topbar-skimmers = SKIMMERS
topbar-run-skimmer = Deze skimmer starten
topbar-minimum-snr-decoded-signal-needs = Minimale SNR die een gedecodeerd signaal nodig heeft om als spot te gelden
topbar-needs-wideband-iq-source = vereist een breedband-IQ-bron
topbar-cw-decoder = CW-DECODER
topbar-how-many-signals-model-reads = Hoeveel signalen het model tegelijk leest. De rest houdt zijn markering maar draagt geen tekst.
topbar-spots = SPOTS
topbar-save-skimmer-s-spot-list = Sla de spotlijst van de skimmer op in een bestand
topbar-show-hide-full-band-waterfall = Toon/verberg de volledige-bandwaterval boven de panadapter — alles wat deze ontvanger in één keer kan zien
topbar-spectrum-waterfall-either-layer-both = Spectrum en waterval — een van beide lagen, beide of geen. Verlicht zolang beide zichtbaar zijn.
topbar-spectrum = SPECTRUM
topbar-show-spectrum = SPECTRUM TONEN
topbar-draw-spectrum-line-across-top = Teken de spectrumlijn bovenaan de panadapter. Uitgeschakeld neemt de waterval de volledige hoogte.
topbar-peak-hold = PEAK HOLD
topbar-trace-highest-level-each-column = Teken per kolom het hoogste bereikte niveau over de live lijn, langzaam weer dalend — wat de band deed terwijl je elders keek
topbar-draw-spectrum-receding-surface-instead = Teken het spectrum als een wegtrekkend oppervlak in plaats van een vlakke lijn: het nieuwste spectrum vooraan, de eerdere stromen van je weg. De laatste paar seconden van de band als landschap — een draaggolf die komt en gaat is een rug in plaats van een lijn die trilt. Het raster en de peak hold horen bij de vlakke lijn en worden er niet op getekend.
topbar-how-quickly-spectrum-line-follows = Hoe snel de spectrumlijn de band volgt. Langzamer middelt meer frames: een rustigere lijn, en een zwakke draaggolf die lang genoeg stilstaat om te lezen. De waterval blijft hiervan onaangeroerd — die rijen krijgen in beide gevallen elk frame.
topbar-show-waterfall = WATERVAL TONEN
topbar-draw-scrolling-waterfall-below-spectrum = Teken de scrollende waterval onder het spectrum. Uitgeschakeld neemt de spectrumlijn de volledige hoogte.
topbar-how-fast-waterfall-scrolls-lines = Hoe snel de waterval scrolt, in regels per seconde: Langzaam 5, Midden 28, Snel 56, Sneller 112, Snelst 224. De engine klokt deze zelf, dus de twee snelste zijn echt detail in plaats van dezelfde regel twee keer getekend — voor zover de ontvanger ze kan leveren: een regel kan nooit meer dan één transformatie tonen, en een smalle frontend levert maar enkele tientallen per seconde. Ze gaan ten koste van de geschiedenis, want de waterval bewaart een vast aantal regels — 73 seconden bij Midden, 9 bij Snelst.
topbar-decode-labels = DECODEERLABELS
topbar-mark-every-decoded-station-waterfall = Markeer elk gedecodeerd station op de waterval met zijn roepletters, op de frequentie waarop het werd gehoord — FT8, FT4 en de andere modi met tijdslots. Een goede opening zet er twee keer per minuut dertig over het bereik, boven de sporen die je leest; schakel ze uit en de waterval is gewoon de band. De decodelijst ernaast heeft nog steeds alle roepletters.
topbar-switch-3d-set = Schakel 3D in om dit in te stellen
topbar-how-3d-spectrum-drawn-only = Hoe het 3D-spectrum wordt getekend. Alleen de vorm verschilt — beide weergaven tonen dezelfde spectra en verbergen allebei wat erachter ligt, zodat een sterk signaal vooraan boven de band staat die het bedekt.
topbar-lines = LIJNEN
topbar-one-trace-per-remembered-spectrum = Eén spoor per onthouden spectrum, in de eigen kleur van de vlakke lijn, elk met de erachter liggende sporen verborgen. De vorm zonder de niveaus, en de keuze als de vorm is wat je zoekt — het kost iets meer dan de massieve variant in plaats van minder, omdat hetzelfde oppervlak wordt getekend en daarna elke top erbovenop wordt getrokken.
topbar-solid = MASSIEF
topbar-filled-surface-coloured-waterfall-s = Een gevuld oppervlak gekleurd met het palet van de waterval, zodat het niveau zowel in de kleur als in de hoogte zit en beide helften van de panadapter het eens zijn over hoe een sterk signaal eruitziet. Wijzig het palet in Settings › Display.
topbar-how-fast-3d-spectrum-flows = Hoe snel het 3D-spectrum van je wegstroomt, in rijen per seconde: Langzaam 6, Midden 12, Snel 24, Sneller 48. Het oppervlak is een vast aantal rijen diep, dus dit bepaalt ook hoeveel tijd het bevat. Langzamer is een langer geheugen en een oppervlak dat kruipt; sneller is een korter geheugen dat beweegt. De waterval houdt zijn eigen scrollsnelheid, hieronder.
topbar-how-many-columns-panadapter-waterfall = Met hoeveel kolommen de panadapter en zijn waterval worden getekend. Auto leest de renderer van deze machine en de grootte van de panadapter en kiest het meeste dat hij aankan — een 4K-scherm wil 4096. Elke kolom is een byte in elk frame, dus een client die met een extern station is verbonden betaalt voor het detail op zijn verbinding: 4096 kolommen bij 60 fps is ongeveer een kwart megabyte per seconde, twee keer de standaardbreedte.
topbar-read-machine-screen-take-most = Lees deze machine en dit scherm, neem het meeste dat ze aankunnen en volg ze als ze veranderen
topbar-width-force-right-now-whatever = De breedte die nu geldt, wat de rij erboven ook vraagt
topbar-keep-floor-ceiling-set-best = Houd de ondergrens/bovengrens op de beste instelling voor waterval-contrast — langzaam op zijn plaats gebracht na een bandwissel, na pannen of zoomen, en als de niveaus afdrijven. Schakel in om meteen te passen; schakel uit om de niveaus te houden waar je ze zette.
topbar-keep-tuned-frequency-middle-panadapter = Houd de afgestemde frequentie in het midden van de panadapter: het venster schuift onder de afstemming in plaats van dat het beeld een hele span verspringt als je voorbij de rand afstemt. Schakel in om meteen te centreren; schakel uit om naar believen te pannen en zoomen.
topbar-ceil = max.{" "}
topbar-fft-size = FFT-GROOTTE
topbar-scroll-waterfall-upwards-newest-row = Scrol de waterval omhoog — nieuwste rij onderaan (V)
topbar-logbook-all-qsos-digital-manual = Logboek — alle QSO's (digitaal + handmatig)
topbar-award-tracking-dxcc-was-waz = Awardbijhouding — DXCC / WAS / WAZ / locators
topbar-band-conditions-published-forecast-beside = Bandcondities — de gepubliceerde voorspelling naast wat er daadwerkelijk op elke band is gehoord
topbar-satellite-qo-100-beacon-calibration = Satelliet — de QO-100-bakenkalibratie loopt
topbar-satellite-doppler-tracking-qo-100 = Satelliet — Doppler-tracking en de QO-100-bakenkalibratie
topbar-ism-band-devices-decoding-now = ISM-bandapparaten — worden nu gedecodeerd, of dit venster nu open is of niet. Schakel uit met DECODING in het venster.
topbar-ism-band-devices-weather-sensors = ISM-bandapparaten — weersensoren, meters en domotica die in je omgeving te horen zijn
topbar-public-sdrs-internet-browse-kiwisdr = Openbare SDR's op internet — blader door de KiwiSDR- en SpyServer-mappen en open er een als radio
topbar-winlink-radio-email = Winlink-radio-e-mail
topbar-memory-channels = Geheugenkanalen
topbar-scanner-stopped-signal = Scanner — gestopt op een signaal
topbar-scanner-running = Scanner — actief
topbar-scan-memory-channels-frequency-range = Scan geheugenkanalen of een frequentiebereik
topbar-grid-tracker-worked-maidenhead-squares = Locatortracker — gewerkte Maidenhead-vierkanten op een kaart, met wat er wordt gehoord
topbar-settings-device-gains-antennas-audio = Instellingen — apparaatversterkingen, antennes, audioapparaten
topbar-user-manual-f1 = Handleiding (F1)
topbar-range-was-typed-settings-radio = Dat bereik is ingetypt in Settings ▸ Radio ▸ RX range, niet door de radio gemeld. Verbreed het, of maak het vak leeg om te gebruiken wat het apparaat over zichzelf zegt.
topbar-what-device-reports-about-itself = Dat is wat het apparaat over zichzelf meldt. Als de radio deze band wel dekt, geef dan het echte bereik op in Settings ▸ Radio ▸ RX range.
topbar-band = BAND
topbar-mode = MODUS

## Top bar: the System box's window chips.

topbar-chip-log = LOG
topbar-chip-awards = AWARDS
topbar-chip-bands = BANDEN
topbar-chip-sat = SAT
topbar-chip-ism = ISM
topbar-chip-mail = MAIL
topbar-chip-mem = MEM
topbar-chip-scan = SCAN
topbar-chip-hfdl = HFDL
topbar-chip-grid = LOCATOR

## Top bar: the mode-defaults chip.

topbar-changed-agc = AGC
topbar-changed-squelch = squelch
topbar-changed-auto-notch = auto-notch
topbar-changed-stereo = stereo
topbar-changed-binaural = binauraal
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    Terug naar de standaardwaarden van { $mode }. Daarvan gewijzigd: { $changed }.

    Klik om ze terug te zetten. De eigen waarden van { $mode } keren terug en wat je hier instelde wordt vergeten.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = de radio heeft geen instelling voor { $baud } baud — { $default } wordt in plaats daarvan gebruikt
settings-radio-icom-scope-hint =
    Streamt de eigen scope-sweep van de radio over de CI-V-verbinding en tekent die als panadapter — het enige beeld van de band dat een rig met demodulatie-audio kan geven, omdat het geluid dat hij stuurt al door zijn filter is gegaan.

    Vereist dat de radio ervoor is ingesteld: CI-V USB Baud Rate op 115200 en CI-V USB Port op "Unlink from {"["}REMOTE]" (beide onder SET > Connectors > CI-V), en de baudsnelheid hierboven daarmee in overeenstemming — onder { $min_baud } passen de sweeps niet over de verbinding en blijft de scope uit.

    De sweeps delen de interne USB-bus van de radio met zijn eigen geluidskaart. Als ontvangen geluid begint weg te vallen terwijl de scope aan staat, is dit vak het eerste om uit te zetten.
settings-radio-rx-tx-words = { $rx } / TX { $tx }
settings-radio-soapy-native = Hardware hierboven wordt rechtstreeks door sdroxide ondersteund: { $names }. Als je die interface hierboven kiest, krijg je zijn eigen instellingen — versterkingstrappen, filters en notches die SoapySDR niet kan uitdrukken — en is geen SoapySDR-module nodig.
settings-radio-rx888-width-hint = Hoeveel van het gedigitaliseerde spectrum de panadapter tegelijk toont. De hele DSP-keten draait op deze breedte, dus breder kost proportioneel meer CPU — 1/2 is de hele band in de waterval, en een flinke hoeveelheid rekenwerk. Boven de VHF-overgang ({ $crossover } MHz) kijkt de panadapter niet naar de antenne maar naar de IF van de tuner, die 8 MHz breed is en op { $if_centre } MHz staat — dus alles breder dan { $widest } kan er niet op gecentreerd worden, en de extra breedte is dood spectrum naast het signaal in plaats van meer ervan.
settings-radio-sdrplay-serial-missing = Serienummer { $serial } staat niet bij de ontvangers die de SDRplay-service meldt. Kies een van de genoemde ontvangers, of steek deze opnieuw in en druk op Rescan.
settings-radio-hdr-filter-hint =
    Het analoge filter vóór het HDR-pad — een andere regelaar dan de eigen bandbreedte van de ontvanger. Elke instelling is gebouwd op een vaste set middenfrequenties en kan elders niets doen; deze is gebouwd op { $centres }. Werkt direct.

    Het pad zelf werkt nog niet — zie de schakelaar hierboven — dus dit kiest een filter voor een modus die nu stil is.
settings-radio-rspduo-only = Alleen een RSPduo heeft een tweede tuner, en de hier gekozen ontvanger is een { $model }, dus dit doet niets.
settings-radio-lime-tx-gain-low = De zendversterking is { $db } dB, onderaan het bereik — een paar microwatt uit het bord, wat op een vermogensmeter als niets leest, wat er ook achter hangt. Dat is de standaard zodat een gewapende zender je niet kan verrassen; verhoog hem voordat je zendt.

## Settings window, General: SWR

settings-general-swr-tune-limit = (afstemmen: { $limit }:1)

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Klik om de meterweergave te wisselen: { $faces }
topbar-step-down = Omlaag { $step }
topbar-step-up = Omhoog { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = Repeaterverschuiving { $shift } — ontvangen op { $rx }, zenden op { $tx } MHz{ $auto }
topbar-tone-and-burst = { $tone } onder de spraak mee verzonden, en een burst van 1750 Hz van { $ms } ms aan het begin van elke beurt
topbar-tone-under-voice = { $tone } onder de spraak mee verzonden
topbar-burst-every-over = Een burst van 1750 Hz van { $ms } ms aan het begin van elke beurt
topbar-match-tx = ZELFDE ALS TX ({ $tone })
topbar-decimation-on = Frontend-decimatie: de ontvanger krijgt het middelste 1/{ $n } van de { $msps } Msps die deze radio streamt — { $span } — en de rest wordt weggegooid voordat iets ervan de ontvanger bereikt.
topbar-decimation-off = Frontend-decimatie uit: de ontvanger ziet de hele { $span } die deze radio streamt.
topbar-decimation-hint =
    { $what }

    Een smallere span betekent fijnere waterfallresolutie, een stillere ruisvloer (3 dB per halvering) en minder CPU — ten koste van de band aan weerszijden. De afstemming werkt nog overal; de radio verplaatst zijn LO om te volgen.

    Klik om te wisselen: uit / 2 / 4 … { $max }.
topbar-rx-gain-hint = Frontend-RX-versterking ({ $stage }). Te veel overstuurt de ADC van de ontvanger en smeert stoorsignalen over de band; te weinig en hij wordt doof.
topbar-rx-gain-stages =
    {""}

    Deze rig heeft { $n } RX-versterkingstrappen — de rest staat in Settings → Device.
topbar-passband-now =
    {""}

    De doorlaatband staat nu op { $dbfs } dBFS.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    Onderdruk het geluid onder dit vermogen in de ontvangstdoorlaatband, in dBFS. Links is open.{ $level } Stel hem boven de ruis en onder het signaal in.

    Een stroom die met de eigen AGC van de radio er al in aankomt — de 12 kHz IF van een Icom bijvoorbeeld — ligt op deze schaal veel hoger dan de ruwe basisband van een SDR, en daarom reikt de rail tot volle schaal.

    Klik om de sterkte te wisselen: Laag / Midden / Hoog / Uit. De engine staat in de rijen hieronder.
topbar-recording-both = Opname van { $audio } en { $iq }
topbar-recording-audio = Geluid wordt opgenomen naar { $audio }
topbar-recording-iq = I/Q wordt opgenomen naar { $iq }
topbar-auto-record-armed = Auto-opname actief: een bestand per zending, gesloten na { $secs } s stilte
topbar-rds-hover = RDS: { $ps }. Klik voor de gegevens van het station
topbar-drm-hover = DRM: { $service }. Klik voor de details van de uitzending
topbar-drm-no-codec = DRM: { $summary } — vergrendeld, maar het { $codec }-geluid kan hier niet worden gedecodeerd. Klik voor wat ontbreekt
topbar-drm-state = DRM: { $summary } — klik voor de status van de decoder
topbar-hd-hover = HD Radio: { $station }. Klik voor de details van de uitzending
topbar-hd-state = HD Radio: { $summary } — klik voor de status van de decoder
topbar-binaural-preset = { $preset } in elk oor — { $lo } … { $hi } Hz, beide zijbanden
topbar-nr-hover =
    Ruisonderdrukking: { $strength } — { $engine }

    Klik om engine of sterkte te wijzigen
topbar-clipping = Clip van { $span } — druk opnieuw om opnieuw te beginnen
topbar-record-clip = Neem { $span } op en stop — een clip om bij een ontvangstrapport te voegen
topbar-starting-clip = clip van { $span } wordt gestart…
topbar-stops-in = stopt over { $time }
topbar-auto-record-hint = Neem elke zending op in een eigen bestand, gesloten na { $secs } seconden stilte
topbar-recording-silence = opnemen · { $secs } s stilte (sluit om { $hold })
topbar-tone-open = Ontvangt { $tone }, de toonsquelch — geluid open
topbar-tone-closed = Ontvangt { $heard }, maar de toonsquelch wil { $wanted } — geluid blijft dicht
topbar-tone-heard = Ontvangt CTCSS/DCS { $tone }
topbar-tone-nothing = Toonsquelch { $tone }: er wordt niets ontvangen dat daarbij past
topbar-tone-use = GEBRUIK { $tone }
topbar-sockets = Aansluitingen: { $sockets }
topbar-voice-keyer-tx = { $slot } wordt verzonden — klik om de voice keyer te openen
topbar-drive-uncal-gen = Deze frequentie, die op geen amateurband ligt, is niet gekalibreerd, dus de instelling bereikt de zender ongewijzigd.
topbar-drive-uncal-band = { $band } is niet gekalibreerd, dus de instelling bereikt de zender ongewijzigd.
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = Deze frequentie, die op geen amateurband ligt, is { $trim } dB gekalibreerd, dus dezelfde instelling brengt hier { $diff } dB { $more ->
        [less] minder
       *[more] meer
    } de lucht in dan op een niet-gekalibreerde band.
topbar-drive-cal-band = { $band } is { $trim } dB gekalibreerd, dus dezelfde instelling brengt hier { $diff } dB { $more ->
        [less] minder
       *[more] meer
    } de lucht in dan op een niet-gekalibreerde band.
topbar-drive-hover =
    Hoe hard de zender wordt aangestuurd. Bij een radio die sdroxide zelf moduleert schaalt het de gemoduleerde samples; bij een rig met eigen vermogensregeling is het het deel van het nominale vermogen dat aan de rig wordt gevraagd.

    { $trim }

    Settings → Radio → Transmit drive by band.
topbar-cessb-hover =
    Controlled-envelope SSB: meer gemiddeld vermogen bij dezelfde piek, zonder splatter — drie of vier decibel schijnbaar luider aan de andere kant. Hoeveel decibel de spraak de processor in wordt gestuurd; 0 is uit, 6 is een verstandige eerste poging. Het raakt Drive niet.

    Nu: { $now }
topbar-waterfall-rate = { $rows } rijen per seconde — { $seconds } seconden band
topbar-detail-auto-columns = { $n } kolommen, wat Auto hier zou hebben gekozen
topbar-n-columns = { $n } kolommen
topbar-fft-bins-whole = { $n } bins over de hele stroom — één per kolom, of fijner
topbar-fft-bins-pooled = { $n } bins samengevoegd tot de { $cols } kolommen van de panadapter: scherpere signalen, niet meer ervan
topbar-sat-locked = Satelliet — vergrendeld op { $name }
topbar-filter-edges = { $lo } … { $hi } Hz vanaf de draaggolf
topbar-filter-isb = Ontvangstfilter: { $width } Hz in elk oor ({ $edges }). De twee zijbanden van ISB zijn aparte zendingen, dus de breedte is die van één ervan.
topbar-filter-width = Ontvangstfilter: { $width } Hz breed ({ $edges }).
topbar-filter-hint =
    { $what }

    Klik voor de breedte, de twee randen en de voorinstellingen van deze modus, als getallen — de grepen van de panadapter plaatsen een doorlaatband op het oog, hier typ je een exacte waarde.
topbar-band-outside-range =
    { $band } valt buiten wat deze radio ontvangt ({ $ranges } MHz).

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band }: { $verdict } ({ $time ->
        [day] overdag
       *[night] 's nachts
    }) — voorspeld door HAMQSL.com op basis van de zonne-indices, geen meting van je eigen pad.

## Top bar: short words and captions

topbar-squelch-open = open
topbar-off = uit
topbar-recording = opnemen
topbar-sql = sql
topbar-stations = stations
topbar-caption-reaction = reactie
topbar-caption-scroll = scrol
topbar-caption-surface = oppervlak
topbar-caption-flow = stroming
topbar-caption-detail = detail
topbar-floor = ondergrens

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = VLIEGTUIGEN
pane-chart = GRAFIEK
pane-chat = CHAT
pane-decodes = DECODES
pane-files = BESTANDEN
pane-heard = GEHOORD
pane-image = BEELD
pane-map = KAART
pane-messages = BERICHTEN
pane-monitor = MONITOR
pane-panel = PANEEL
pane-qso = QSO
pane-reading = LEZEN
pane-receive = ONTVANGEN
pane-saved = OPGESLAGEN
pane-send = ZENDEN
pane-spots = SPOTS
pane-stations = STATIONS
pane-status = STATUS
pane-terminal = TERMINAL
pane-text = TEKST
pane-traffic = VERKEER
pane-vessels = SCHEPEN
pane-wfall = WFALL

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}OPSLAAN{" "}
panels-nothing-decoded-save = Niets gedecodeerd om op te slaan
panels-clear-rx = {" "}RX WISSEN{" "}
panels-empty-receive-window-nothing-air = Maak het ontvangstvenster leeg. Wat er in de ether is, stopt niet.
panels-nothing-received-clear = Niets ontvangen om te wissen
panels-save-what-panel-has-decoded = Sla op wat dit paneel heeft gedecodeerd in een bestand
panels-prop = PROP
panels-shade-map-where-signals-are = Kleur de kaart naar waar signalen daadwerkelijk doorkomen, per band. Elke ontvangst wordt op het midden van haar pad geplaatst — het stuk ionosfeer dat haar boog — niet bij het verre station, dus dit is een kaart van de hemel en niet van waar radioamateurs wonen. Opgebouwd uit paden waaraan dit station deelnam, plus — als het Reverse Beacon Network is ingeschakeld onder Settings → Spots — wat de skimmers van de wereld horen, wat de banden dekt waarop deze radio niet zit.
panels-night = NACHT
panels-shade-where-sun-down-twilight = Kleur waar de zon onder is, en de schemering ertussen, zodat de grijze lijn op de kaart zichtbaar is. Lage banden gaan lang en hoge banden sluiten aan de nachtzijde ervan, en de terminator zelf is waar de DX is.
panels-all-bands = ALLE BANDEN
panels-every-band-once-one-hue = Alle banden tegelijk, elk één tint — algemene condities.
panels-one-band = ÉÉN BAND
panels-one-band-cold-hot-blue = Eén band, van koud naar heet: blauw, groen, geel, rood.
panels-frequency-you-are-actually-working = De frequentie waarop je werkelijk werkt: de afstemming plus de tooncorrectie ernaast. Dit komt in het log — de afstemming alleen is zoveel te laag.
panels-send-return = VERSTUREN MET RETURN
panels-freq = ⇵ FREQ
panels-nothing-agreed-mode-save-your = Niets afgesproken voor deze modus — sla hieronder je eigen op.
panels-forget-saved-frequency = Vergeet deze opgeslagen frequentie
panels-dial-already-one-your-saved = Deze afstemming is al een van je opgeslagen frequenties.
panels-dial-already-agreed-list-mode = Deze afstemming staat al in de afgesproken lijst voor deze modus.
panels-decode-squelch-raise-stop-decoding = Decodeer-squelch — verhoog om te stoppen met ruis decoderen
panels-message-repeats-long-transmit-held = Het bericht wordt herhaald zolang zenden wordt ingehouden — een meteoor vangt het deel dat er net langskomt.
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}TX AAN{"  "}
panels-tx-2 = {"   "}TX{"   "}
panels-call-cq = {" "}CQ ROEPEN{" "}
panels-nocall = NOCALL

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = Te weinig spraak gehoord — praat normaal gedurende de hele telling en probeer opnieuw.
txaudio-tx-audio = TX-audio
txaudio-mode-not-voice-mode-so = Deze modus is geen spraakmodus, dus wat hij verzendt gaat niet door de keten. Instellingen die je hier maakt werken in SSB, AM en FM.
txaudio-bypass = OVERBRUGGEN
txaudio-switch-every-stage-out-once = Schakel alle trappen tegelijk uit, ook de ruisonderdrukker, zodat de microfoon onbewerkt uitgaat. Klik opnieuw om precies de trappen te herstellen die aan stonden — de snelste A/B van wat de keten doet.
txaudio-denoise = RUISONDERDRUKKING
txaudio-rnnoise-microphone-ahead-every-other = RNNoise op de microfoon, vóór alle andere trappen, zodat kamerruis wordt verwijderd voordat de gate of de compressor hem omhoog kan halen. Voegt ongeveer 10 ms vertraging toe.
txaudio-digital-voice = DIGITALE SPRAAK
txaudio-also-run-freedv-rade-digital = Laat ook FreeDV / RADE digitale spraak door de keten lopen. Standaard uit: een vocoder is getraind op natuurlijke spraak en decodeert mogelijk slechter uit sterk bewerkte spraak, dus probeer het in de ether voordat je erop vertrouwt. TCI-audio en de digitale modi gaan nooit door de keten.
txaudio-test = Test
txaudio-rec = {" "}⏺ OPN{" "}
txaudio-play = {" "}▶ AFSPELEN{" "}
txaudio-play-test-phrase-through-chain = Speel de testzin af door de keten zoals die nu is ingesteld, naar je luidsprekers. Pas aan terwijl het speelt en hoor het verschil. Ontvangstaudio wordt vervangen zolang het speelt.
txaudio-recording = opnemen…
txaudio-no-phrase-yet = nog geen zin
txaudio-live = Live
txaudio-live-2 = {" "}🎧 LIVE{" "}
txaudio-hear-your-microphone-through-chain = Hoor je microfoon door de keten terwijl je praat, zonder de zender in te schakelen. Gebruik een hoofdtelefoon: via luidsprekers koppelt het terug. Je hoort jezelf enkele tientallen milliseconden te laat, wat praten lastig maakt — stel niveaus in op de meters en beoordeel het geluid met OPN / AFSPELEN. Ontvangstaudio wordt vervangen zolang het loopt; PTT indrukken stopt het.
txaudio-meters = {" "}METERS{" "}
txaudio-run-your-microphone-through-chain = Laat je microfoon alleen voor de meters door de keten lopen: niets op de luidsprekers, dus geen vertraging om tegenaan te praten en niets om terug te koppelen. De manier om mic- en ingangsversterking, de gate-drempel en de werking van de compressor in te stellen. Ontvangstaudio gaat door; PTT indrukken stopt het.
txaudio-monitoring-live-use-headphones-speakers = Live meeluisteren — gebruik een hoofdtelefoon, anders koppelen de luidsprekers terug in de microfoon.
txaudio-voice-modes-only-tci-clients = Alleen spraakmodi. TCI-clients en de digitale modi omzeilen de keten; houd bij gebruik de eigen TX EQ en processor van de radio vlak.
txaudio-auto-set-stopped-microphone-was = Auto-instelling gestopt: de microfoon werd weggenomen.
txaudio-microphone-may-clipping-before-gets = {" "}De microfoon clipt mogelijk al voordat het signaal hier komt — zet zijn eigen versterking lager als het hard klinkt.
txaudio-level = Niveau
txaudio-mic-control-same-one-transmit = De Mic-regelaar — dezelfde als op de zendbalk, gedeeld door elk profiel. 0 dB in het midden, +6 dB bovenaan.
txaudio-mic = Mic
txaudio-gain-top-mic-microphone-stored = Versterking bovenop Mic voor deze microfoon, bewaard met het profiel. De drempels van de voorinstellingen gaan uit van een stem die rond -10 dBFS piekt op de IN-meter; Auto-set zet hem daar. Blijft actief onder OVERBRUGGEN en geldt ook voor digitale spraak: het is niveau, geen bewerking.
txaudio-input = Ingang
txaudio-auto-set = Auto-set
txaudio-auto-set-cancelled = Auto-set geannuleerd.
txaudio-talk-now-you-would-air = Praat nu, zoals je in de ether zou doen…
txaudio-db = -∞ dB
txaudio-chain = Keten
txaudio-order-audio-meets-them = In de volgorde waarin het geluid ze tegenkomt
txaudio-switch-stage = Schakel deze trap in
txaudio-earlier-chain = Eerder in de keten
txaudio-later-chain = Later in de keten
txaudio-output-limiter = De uitgangslimiter
txaudio-output = Uitgang
txaudio-trim-limiter-always-last = Trim en limiter — altijd als laatste
txaudio-profiles = Profielen…
txaudio-gate-expander = Gate / expander
txaudio-pushes-room-noise-fan-rumble = Drukt kamerruis, ventilatorgebrom en sis omlaag in de pauzes. Stel de drempel net boven je ruisvloer en onder je zachtste lettergrepen in.
txaudio-gate = GATE
txaudio-hard-gate-10-1-down = Harde gate: 10:1, tot -40 dB omlaag
txaudio-expander = EXPANDER
txaudio-gentle-expander-2-1-never = Zachte expander: 2:1, nooit meer dan -15 dB — de kamer wordt nooit dood
txaudio-reset = RESET
txaudio-back-defaults = Terug naar de standaardwaarden
txaudio-threshold = Drempel
txaudio-below-gate-starts-close = Eronder begint de gate te sluiten.
txaudio-return = Terugval
txaudio-hysteresis-once-open-closes-only = Hysterese: eenmaal open sluit hij pas zo ver onder de drempel, zodat een stem die rond de drempel zweeft de gate niet laat fladderen.
txaudio-hold = Hold
txaudio-how-long-stays-open-after = Hoe lang hij open blijft nadat de stem wegvalt, zodat woordeinden niet worden afgekapt.
txaudio-release = Release
txaudio-how-fast-closes-once-hold = Hoe snel hij sluit als de hold voorbij is. Verleng hem als de gate pompt.
txaudio-floor = Ondergrens
txaudio-most-ever-turns-pauses-down = Het meeste waarmee hij de pauzes ooit omlaag draait.
txaudio-ratio = Ratio
txaudio-how-steeply-closes-below-threshold = Hoe steil hij onder de drempel sluit.
txaudio-look-ahead = Look-ahead
txaudio-opens-much-before-word-starts = Opent zoveel vóór een woord begint, zodat de eerste medeklinker niet wordt afgehakt. Voegt dezelfde vertraging toe aan alles wat wordt verzonden.
txaudio-high-pass-low-shelf-two = Hoogdoorlaat, lage shelf, twee pieken, hoge shelf en laagdoorlaat. Sleep een punt op de curve, of stel hieronder in. Zet de laagdoorlaat op de breedte van je zendfilter.
txaudio-flat = VLAK
txaudio-every-band-back-0-db = Elke band terug naar 0 dB, beide randen uit
txaudio-freq = Freq
txaudio-gain = Versterking
txaudio-q-slope = Q / Helling
txaudio-high-pass = Hoogdoorlaat
txaudio-removes-handling-noise-breath-pops = Verwijdert hanteringsgeluid, ademploffen en brom onder de hoekfrequentie.
txaudio-low-shelf = Lage shelf
txaudio-body-mud-cut-little-here = Lichaam, of modder: verlaag hier een beetje voor een stem die dof klinkt.
txaudio-peak-1 = Piek 1
txaudio-bump-dip-voice-band-often = Een bult of dip in het spraakbereik — vaak een verlaging rond 300–500 Hz.
txaudio-peak-2 = Piek 2
txaudio-presence-lift-around-2-3 = Presence: een verhoging rond 2–3 kHz is wat op SSB doorkomt.
txaudio-high-shelf = Hoge shelf
txaudio-brightness = Helderheid.
txaudio-low-pass = Laagdoorlaat
txaudio-stops-energy-above-transmit-filter = Voorkomt dat energie boven het zendfilter in de modulator wordt verspild.
txaudio-t-24-db-oct = 24 dB/oct
txaudio-t-12-db-oct = 12 dB/oct
txaudio-click-steeper-gentler-slope = Klik voor een steilere of zachtere helling
txaudio-de-esser = De-esser
txaudio-turns-down-only-sibilant-band = Draait alleen het sisgeluidsbereik omlaag wanneer "s" en "sh" de drempel overschrijden, zodat de rest van de stem zijn niveau houdt. Vind de frequentie door erover te schuiven terwijl je "sister silver cellophane" zegt en te stoppen waar de sis het luidst is.
txaudio-frequency = Frequentie
txaudio-centre-sibilant-band = Midden van het sisgeluidsbereik.
txaudio-width-band-higher-narrower = Breedte van het bereik — hoger is smaller.
txaudio-band-level-where-reduction-starts = Bereikniveau waar de reductie begint. Te laag en hij werkt de hele tijd, wat de stem dof maakt.
txaudio-amount = Hoeveelheid
txaudio-most-band-ever-turned-down = Het meeste waarmee het bereik ooit omlaag wordt gedraaid. Begin rond -6 dB.
txaudio-attack = Attack
txaudio-how-fast-reacts = Hoe snel hij reageert.
txaudio-how-fast-lets-go = Hoe snel hij loslaat.
txaudio-compressor = Compressor
txaudio-brings-quiet-syllables-up-holds = Haalt zachte lettergrepen omhoog en houdt luide tegen. Mik op 3–6 dB reductie op pieken; meer dan 10 dB is duidelijke compressie, beter geschikt voor contests.
txaudio-level-above-which-starts-compress = Niveau waarboven hij begint te comprimeren.
txaudio-how-hard-compresses-above-threshold = Hoe hard hij boven de drempel comprimeert.
txaudio-how-fast-reacts-peak-10 = Hoe snel hij op een piek reageert. 10–25 ms houdt medeklinkers scherp.
txaudio-how-fast-recovers = Hoe snel hij herstelt.
txaudio-knee = Knee
txaudio-soft-knee-width-0-hard = Breedte van de zachte knee; 0 is een harde knee.
txaudio-makeup = Makeup
txaudio-gain-after-compression-bring-level = Versterking na compressie, om het niveau weer omhoog te brengen.
txaudio-drive = Drive
txaudio-gain-ahead-compressor-so-more = Versterking vóór de compressor, zodat meer van de stem de drempel overschrijdt — verhoogt het gemiddelde vermogen. Combineer met de uitgangslimiter.
txaudio-phase = Fase
txaudio-all-pass-phase-rotator-ahead = Allpass-faserotator vóór de detector. Egaliseert de scheve pieken van een stem, zodat hetzelfde piekniveau meer gemiddeld vermogen draagt. 4 is een gangbare omroepinstelling; 0 is uit.
txaudio-last-stage-trim-look-ahead = De laatste trap: een trim en een look-ahead-limiter die elke piek onder het plafond houdt zonder de splatter van hard clippen. Standaard uit, zodat het inschakelen van de keten een reeds ingesteld drive-niveau niet verandert.
txaudio-limiter = Limiter
txaudio-on = Aan
txaudio-ceiling = Plafond
txaudio-no-peak-leaves-chain-above = Geen piek verlaat de keten boven dit niveau.
txaudio-trim = Trim
txaudio-output-gain-ahead-limiter = Uitgangsversterking, vóór de limiter.
txaudio-off = {"  "}uit
txaudio-meters-move-while-you-transmit = De meters bewegen terwijl je een spraakbeurt zendt, de testzin afspeelt, of LIVE of METERS draait.
txaudio-profiles-2 = Profielen
txaudio-profile-whole-chain-every-stage = Een profiel is de hele keten: de instellingen van elke trap, welke aan staan, hun volgorde en de ingangsversterking voor je microfoon. Een profiel laden vervangt de keten; de Mic-regelaar en drive blijven zoals ze zijn.
txaudio-name = Naam
txaudio-save = Opslaan
txaudio-store-chain-now-under-name = Sla de keten zoals hij nu is op onder deze naam, en vervang een profiel met dezelfde naam
txaudio-import = Importeren…
txaudio-read-profile-json-file = Lees een profiel uit een .json-bestand
txaudio-tx-audio-profile = TX-audioprofiel
txaudio-no-profiles-yet-save-stores = Nog geen profielen. Opslaan bewaart de keten die je nu hebt ingesteld.
txaudio-load = Laden
txaudio-export = Exporteren…
txaudio-delete = Verwijderen
txaudio-starting-points = Startpunten
txaudio-set-your-level-auto-set = Stel eerst je niveau in met Auto-set — de drempels van de voorinstellingen gaan ervan uit — en stem dan hiervandaan af in plaats van te zenden zoals geladen: neem een zin op en luister na elke wijziging. Een voorinstelling behoudt je ingangsversterking.
txaudio-profile-loaded = Profiel geladen.

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = PROP
solar-where-signals-are-actually-getting =
    Waar signalen daadwerkelijk doorkomen, band voor band. Elke ontvangst waaraan dit station deelneemt — WSPR, FT8/FT4, JS8 en het logboek — wordt op het midden van haar pad geplaatst, het stuk ionosfeer dat haar boog, in plaats van bij het verre station. Dit is dus een kaart van de hemel, niet van waar radioamateurs wonen.

    Het kan alleen paden tonen waarvan dit station een uiteinde was. Oceanen lichten op omdat de middelpunten van lange paden daar vallen; Antarctica blijft donker omdat niemand ervandaan zendt. Standaard uit — het kleurt de hele planeet.
solar-orbits = BANEN
solar-orbital-paths = Baanpaden
solar-clouds = WOLKEN
solar-cloud-cover-noaa-s-global = Wolkendek uit NOAA's wereldwijde geostationaire mozaïek, getekend als een dikte lucht in plaats van een plaatje op het oppervlak: infrarood zegt hoe koud elke wolkentop is, en daaraan zie je hoe hoog hij staat. Onweersbuien flitsen binnen de torens die ze bouwen.
solar-planets = PLANETEN
solar-other-seven-planets-eighteen-their = De andere zeven planeten, achttien van hun manen en de ringen van Saturnus en Uranus. Planeetposities komen uit de Kepler-elementenset van JPL; de manen zijn cirkelbanen aangepast aan JPL Horizons.
solar-coronal-mass-ejection-trajectory-cones = Baankegels van coronale massa-ejecties
solar-sun-obs = ZON OBS
solar-solar-observations-sun-s-disk = Zonswaarnemingen op de zonneschijf: actieve zonnevlekgebieden en waar de uitbarstingen vandaan kwamen
solar-labels = LABELS
solar-body-region-labels = Labels voor hemellichamen en gebieden
solar-small-bodies = KLEINE LICHAMEN
solar-names-asteroids-comets-they-are = Namen bij de asteroïden en kometen. Ze worden hoe dan ook getekend — dit bepaalt alleen of vijfendertig aanduidingen ermee worden getekend. Wat de instelling ook is, het lichaam waarop de camera staat en alles wat het zoekvak heeft gevonden blijft benoemd.
solar-decoded-ft8-ft4-stations-path = Gedecodeerde FT8/FT4-stations en het pad naar het station dat wordt gewerkt
solar-sats = SATS
solar-amateur-radio-satellites-their-orbits = Amateurradiosatellieten, hun banen en elevatie vanaf je QTH
solar-aurora = POOLLICHT
solar-auroral-oval-noaa-s-ovation = De poollichtovaal uit het OVATION-model van NOAA, getekend als emissieschillen op hun echte hoogten, met de evenaarwaartse rand van de 10 %-contour op het oppervlak gemarkeerd
solar-awards = AWARDS
solar-award-coverage-every-dxcc-entity = Awarddekking: elke DXCC-entiteit gekleurd naar wat je logboek nog mist. Oranje brandt waar je de entiteit nooit hebt gewerkt, amber waar wel maar onbevestigd, gedempt groen waar een QSL is teruggekomen. Volgt het bandfilter in het AWARDS-venster.
solar-view = WEERGAVE
solar-which-body-camera-orbits-animated = Om welk lichaam de camera draait, en de geanimeerde rondleiding
solar-layers = LAGEN
solar-what-drawn-orbits-clouds-cmes = Wat wordt getekend: banen, wolken, CME's, labels, QSO's, poollicht
solar-which-sdo-channel-wraps-sun = Welk SDO-kanaal de zon omhult, en hoe oud het is
solar-weather = WEER
solar-aurora-propagation-numbers-which-bands = Het poollicht, de propagatiegetallen en welke banden open zijn
solar-space-weather = RUIMTEWEER
solar-nothing-measured-yet = nog niets gemeten
solar-scale = SCHAAL
solar-body-moon-orbit-exaggeration = Overdrijving van lichaam en maanbaan
solar-time = TIJD
solar-scrub-whole-scene-forward-back = Scrub de hele scène vooruit en terug
solar-activity = ACTIVITEIT
solar-replay-last-hour-decodes-globe = Speel het laatste uur aan decodes opnieuw af op de globe
solar-what-propagation-heat-map-showing = Wat de propagatiewarmtekaart toont
solar-propagation = PROPAGATIE
solar-auto = ▶ AUTO
solar-fly-spline-through-set-framed = Vlieg een spline door een reeks vastgelegde gezichtspunten. Elke muisinvoer onderbreekt het.
solar-moons-ride-circular-orbits-fitted = Manen volgen cirkelbanen aangepast aan JPL Horizons: binnen een graad of twee van waar ze echt zijn, en tot zes voor Miranda, waarvan het baanvlak te snel zwaait om met een cirkel te volgen.
solar-fetch-everything-again-now = Haal nu alles opnieuw op
solar-all-sats = ALLE SATS
solar-show-every-satellite-element-set = Toon elke satelliet in de elementenset, niet alleen de populaire. Baanringen blijven bij de geselecteerde paar — negentig tegelijk is onleesbaar.
solar-starting = starten…
solar-body = lichaam{" "}
solar-moon-orbit = maanbaan{" "}
solar-stretch-earth-moon-distance-so = Rek de afstand Aarde→Maan op zodat het paar apart te zien is
solar-sun = zon{" "}
solar-sun-radius-exaggeration-leave-1 = Overdrijving van de zonnestraal. Laat op 1× om de CME-geometrie leesbaar te houden — een opgezwollen zon verzwelgt de basis van elke kegel.
solar-show = TONEN
solar-paint-globe-what-getting-through = Kleur de globe naar wat er doorkomt
solar-paints-globe-what-getting-through = Kleurt de globe naar wat er doorkomt, per band. Elke ontvangst wordt op het midden van haar pad geplaatst — waar de ionosfeer het werk deed — in plaats van bij het verre station. Opgebouwd uit wat dit station hoort, plus de skimmers van de wereld als het Reverse Beacon Network is ingeschakeld.
solar-display = WEERGAVE
solar-all-bands = ALLE BANDEN
solar-every-band-once-one-hue = Alle banden tegelijk, elk één tint, mengend waar twee overlappen. De weergave voor 'hoe zijn de condities' in plaats van 'is twintig open'.
solar-one-band = ÉÉN BAND
solar-one-band-cold-hot-blue = Eén band, van koud naar heet: blauw, groen, geel, rood.
solar-band = BAND
solar-nothing-heard-yet = nog niets gehoord
solar-bands = BANDEN
solar-sources = BRONNEN
solar-contacts-logbook-path-was-open = Contacten in het logboek. Een pad dat open was, zonder signaalrapport dat die naam waard is — een RST is geen SNR, dus deze tellen mee voor hoe druk een cel is en nooit voor de marge ervan.
solar-stations-reported-hearing-one-downloaded = Stations die meldden dit station te horen, gedownload van WSPRnet. De enige terugkoppeling die een zendend baken ooit krijgt.
solar-reverse-beacon-network-skimmers-what = Reverse Beacon Network-skimmers: wat iedereen verder hoort, op elke band, ook de banden waarop deze radio niet zit. Grover dan de rest — een RBN-regel bevat geen locators, dus beide uiteinden worden in het midden van hun land geplaatst, wat dichtbij is voor een klein land en ver ernaast voor een groot. Schakel in onder Settings → Spots.
solar-decodes-station-made-each-measured = Decodes die dit station maakte. Elke wordt gemeten tegen de ondergrens van haar eigen modus, zodat een WSPR-rapport en een FT4-rapport vergelijkbaar zijn.
solar-memory = GEHEUGEN
solar-half-life = halfwaardetijd{" "}
solar-how-long-reception-takes-count = Hoe lang een ontvangst erover doet om half zo zwaar te tellen. Het eigen geheugen van de ionosfeer is kort — een opening van twee uur oud moet niet ruziën met een van twee minuten geleden.
solar-live = LIVE
solar-follow-band-happens = Volg de band terwijl het gebeurt
solar-replay = ⏸ HERHALING
solar-replay-2 = ▶ HERHALING
solar-replay-last-hour-decodes-over = Speel het laatste uur aan decodes steeds opnieuw af
solar-min-ago = {" "}min geleden
solar-where-last-hour-globe-showing = Waar in het laatste uur de globe naar toont
solar-trail = spoor{" "}
solar-how-long-decode-s-arc = Hoe lang de boog van een decode achter de kop op de globe blijft
solar-speed = snelheid{" "}
solar-how-much-faster-than-real = Hoeveel sneller dan real time de herhaling loopt
solar-qso-layer-off = QSO-laag uit
solar-no-decodes-yet = nog geen decodes
solar-qth-not-set-enter-your = QTH niet ingesteld — voer je locator in bij Settings
solar-mode = MODUS
solar-path = PAD
solar-sent = VERZ.
solar-rcvd = ONTV.
solar-grid = LOCATOR
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = Geostationair — altijd boven je horizon.
solar-no-passes-next-48-hours = Geen overvluchten in de komende 48 uur vanaf je QTH.
solar-start = START
solar-max-el = MAX EL
solar-aos-los-are-azimuths-horizon = AOS/LOS zijn azimuts aan de horizon. Tijden zijn UTC.
solar-locked-unlock = {" "}● VERGRENDELD — ONTGRENDEL{" "}
solar-release-satellite-lock = Laat de satellietvergrendeling los
solar-lock = {" "}VERGRENDEL{" "}
solar-track-satellite-main-window-doppler = Volg deze satelliet in het hoofdvenster: Doppler-gecorrigeerde RX en TX, transponder-gekoppelde uplink, rotorsturing
solar-no-frequencies-file-one-add = Geen frequenties bekend voor deze — voeg ze toe in Settings ▸ TLE.
solar-frequencies-yours = FREQUENTIES · JOUW
solar-frequencies = FREQUENTIES
solar-link = LINK
solar-downlink-mhz = DOWNLINK (MHz)
solar-uplink-mhz = UPLINK (MHz)
solar-doppler-shifts-these-few-khz = Doppler verschuift deze met enkele kHz tijdens een LEO-overvlucht.
solar-satellite-planet-comet = satelliet, planeet of komeet
solar-planet-asteroid-comet = planeet, asteroïde of komeet
solar-clear-search = Zoekopdracht wissen
solar-no-match = geen resultaat
solar-no-sounder = geen sounder
solar-heard = GEHOORD ≥
solar-kp = Kp / A
solar-x-ray = Röntgen
solar-observed-above-sounder-better-than = waargenomen boven de sounder — beter dan gemodelleerd
solar-power-n-s = vermogen N/Z
solar-edge-n-s = rand N/Z
solar-kp-peak-24-h = Kp-piek 24 u
solar-observed-forecast = waargenomen | voorspeld
solar-bands-open = BANDEN OPEN
solar-dxcc-coverage = DXCC-DEKKING
solar-ir-vis = IR+VIS
solar-ir-only = alleen IR
solar-no-deep-convection = geen diepe convectie
solar-t-1-storm = 1 storm
solar-glancing = {" "}· strijkend
solar-direction-estimated = {" "}· richting geschat

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = APRS-instellingen
setup-packet-setup = Packet-instellingen
setup-ft8-ft4-ft2-setup = FT8 / FT4 / FT2-instellingen
setup-my-callsign = Mijn roepletters
setup-my-grid = Mijn locator
setup-station-call = Roepletters station
setup-ssid-oe3jjs-10-packet-station = Met een SSID — OE3JJS-10. Een packetstation heeft gewoonlijk een andere SSID dan de operator, en er wordt niets gezonden voordat dit is ingesteld.
setup-speed = Snelheid
setup-t-9600-needs-radio-s-data = 9600 vereist de datapoort van de radio. Een microfoon- en luidsprekerpad verwoest het aan beide kanten — dat ligt aan de radio, niet aan sdroxide.
setup-tx-delay = TX-vertraging
setup-flags-sent-before-frame-so = Flags die vóór een frame worden verzonden, zodat de andere kant ons kan horen en zijn klok kan vergrendelen. Bij een CAT-rig besteedt sdroxide alleen al 165–240 ms aan in de ether komen, en de eigen zendklaar-tijd van de rig komt daar bovenop.
setup-packet-length = Pakketlengte
setup-bytes = {" "}bytes
setup-most-single-frame-carries-shorter = Het meeste dat één frame draagt. Kortere frames overleven een marginaal pad — alleen het getroffen frame wordt opnieuw verzonden — en kosten meer overhead. 128 op HF, 256 waar het pad stabiel is.
setup-frames-sent-before-waiting-acknowledgement = Frames die worden verzonden voordat op een bevestiging wordt gewacht. Meer vult een goed pad; op een slecht pad is het meer om opnieuw te zenden.
setup-extended = Uitgebreid
setup-ask-mod-128 = Vraag om mod-128
setup-extended-sequence-numbers-window-bigger = Uitgebreide volgnummers, voor een venster groter dan zeven. Veel nodes weigeren het verzoek met een DM, wat er precies uitziet als een station dat niet met je wil praten — laat het dus uit tenzij je weet dat de andere kant het wil.
setup-answer-calls = Oproepen beantwoorden
setup-accept-connections = Verbindingen accepteren
setup-off-winlink-client-which-dials = Uit voor een Winlink-client, die zelf belt en geen reden heeft om te antwoorden. Aan om als peer bereikbaar te zijn: een oproep verschijnt in het terminalpaneel en je praat met wie hem deed. Een station waarvan de verbinding al bezig is met een Winlink-sessie weigert oproepen tot die klaar is.
setup-connect-text = Verbindingstekst
setup-what-caller-greeted = waarmee een oproeper wordt begroet
setup-sent-station-connects-us-once = Verzonden naar een station dat met ons verbindt, zodra de verbinding er is. Leeg verzendt niets — wat er kapot uitziet voor wie belde.
setup-default-via = Standaard via
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = Het digipeaterpad waarmee de terminal begint. Het pad naar je lokale node is elke keer hetzelfde, en opnieuw intypen is hoe een hop wordt vergeten.
setup-beacon = Baken
setup-t-0-disables-timer = 0 schakelt de timer uit
setup-beacon-text = baken-tekst
setup-kiss-server = KISS-server
setup-serve = Aanbieden
setup-offer-modem-kiss-tnc-socket = Bied deze modem aan als KISS-TNC op een socket, zodat Pat, een APRS-client of de Linux AX.25-stack de radio kan gebruiken.
setup-aprs-call = APRS-roepletters
setup-callsign-station-beacons-under-ssid = De roepletters waaronder dit station bakent, met zijn SSID — OE3JJS-9 voor een auto, -10 voor een I-gate, -5 voor een telefoon. Laat leeg om de roepletters hierboven te gebruiken.
setup-no-callsign-station-will-not = Geen roepletters: dit station zendt helemaal niet — geen baken, geen bericht, geen bevestiging.
setup-symbol = Symbool
setup-two-characters-protocol-carries-them = De twee tekens zoals het protocol ze draagt: een tabel (`/` of `\`) en een symboolcode. Een cijfer of letter op de eerste positie is een overlay, bovenop het pictogram van de alternatieve tabel getekend.
setup-path = Pad
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = Hoe ver je vraagt om te worden herhaald. Dit is de meest ingrijpende instelling op het kanaal: elke hop vermenigvuldigt de zendingen die het hele netwerk op één gedeelde frequentie doet.
setup-position = Positie
setup-my-grid-2 = Uit mijn locator
setup-beacon-centre-locator-above-reported = Baken het midden van de locator hierboven, gemeld met de onzekerheid die een locator werkelijk heeft — een zescijferige is een paar kilometer breed, en dat zeggen is eerlijk. Haal het vinkje weg om in plaats daarvan exacte coördinaten te geven.
setup-latitude = Breedtegraad
setup-longitude = Lengtegraad
setup-locator = uit de locator
setup-not-coordinate = geen coördinaat
setup-comment = Opmerking
setup-sent-every-beacon = bij elk baken verzonden
setup-t-0-default-never-beacons-timer = 0 — de standaard — bakent nooit op een timer. Dertig minuten is de conventie voor een vast station; een bewegend station bakent vaker, maar elk baken is de kanaaltijd van iemand anders.
setup-compressed = Gecomprimeerd
setup-compressed-position-format-third-air = Het gecomprimeerde positieformaat: een derde van de zendtijd en nauwkeuriger. Elke ontvanger sinds de jaren 1990 leest het.
setup-first-goes-out-one-interval = Het eerste gaat over één interval uit, niet meteen.
setup-messages = Berichten
setup-acknowledge = Bevestigen
setup-answer-messages-addressed-you-acknowledgement = Beantwoord berichten die aan jou zijn gericht. Een bevestiging is een zending die dit station ongevraagd doet, dus een alleen-ontvangen opstelling moet haar uitzetten — en het baken stopt dan ook met beweren bereikbaar te zijn.
setup-flags-sent-ahead-every-frame = Flags die vóór elk frame worden verzonden, zodat de modem aan de andere kant de draaggolf hoort en zijn klok vergrendelt voordat de data begint. Het moet alles overbruggen tussen het indrukken van zenden en het uitstralen: sdroxide besteedt er alleen al 165–240 ms aan bij een CAT-rig, en een radio die audio via een netwerk neemt buffert er nog meer bovenop. Te weinig en de andere kant vergrendelt nooit — de zending is er en niets decodeert haar.
setup-shared-packet-mode = gedeeld met de packetmodus
setup-keep-stations = Stations bewaren
setup-how-long-station-stays-map = Hoe lang een station op de kaart blijft nadat het voor het laatst is gehoord. Ook waartegen het vervagen van de kaart wordt gemeten, dus een kort venster toont alleen wat live is.
setup-auto-reply = Auto-antwoord
setup-answer-snr-grid-status = Beantwoord SNR? / GRID? / STATUS?
setup-answer-direct-question-addressed-you = Beantwoord een directe vraag aan jou of aan @ALLCALL, met het antwoord in plaats van een bevestiging. Beantwoordt nooit het verkeer van een ander station, en nooit zichzelf.
setup-heartbeat = Heartbeat
setup-off = Uit
setup-turbo-does-not-beacon-local = Turbo bakent niet — het is de lokale en VHF-snelheid.
setup-sends-your-callsign-grid-so = Verstuurt je roepletters en locator zodat anderen weten dat je te ontvangen bent. Het eerste gaat over één interval uit, niet meteen.
setup-beacon-frequency = Bakenfrequentie
setup-t-500-1000-hz = 500–1000 Hz
setup-move-each-beacon-free-slot = Verplaats elk baken naar een vrij slot in de heartbeat-subband, zoals JS8Call doet: daar kijken stations die naar bakens zoeken, en het houdt een onbemande zender uit de QSO van een ander. Het slot wordt gekozen op het moment dat het baken werkelijk uitgaat, vrij van alles wat wordt gedecodeerd.
setup-working-freq = Werkfrequentie
setup-beacon-where-you-are-working = Baken in plaats daarvan waar je werkt. Tegen de bandconventie in, maar het houdt alles wat je zendt op één plek.
setup-heartbeat-reply = Heartbeat-antwoord
setup-answer-heartbeats-signal-report = Beantwoord heartbeats met een signaalrapport
setup-tell-station-beaconed-how-well = Vertel een station dat bakende hoe goed je het kopieerde. Standaard uit: een drukke band draagt elk slot een heartbeat, en ze allemaal beantwoorden zou precies de band overspoelen die heartbeats stil moeten houden. Beperkt tot één antwoord per station per 15 minuten, en nooit terwijl een bericht nog binnenkomt of terwijl je iets in de wachtrij hebt om te zenden.
setup-status-message = Statusbericht
setup-tx-period = TX-periode
setup-even = Even
setup-odd = Oneven
setup-auto-sequence = Auto-volgorde
setup-decode-depth = Decodeerdiepte
setup-fast = Snel
setup-normal = Normaal
setup-deep = Diep

    Snel — één ronde, geen signaalaftrekking: het snelst, de minste decodes.
    Normaal — vlakke meerrondige aftrekking: iets sneller dan Diep, iets minder grondig.
    Diep — de meerrondige met checkpoints: de meeste decodes, en ~1,2 s op een drukke slot.
setup-auto-tx-frequency = Auto TX-frequentie
setup-choose-transmit-frequency-automatically-quietest = Kies de zendfrequentie automatisch: de stilste plek in de periode waarin je zendt, in plaats van de frequentie van het station dat je beantwoordt. Uit houdt hem NIET vast — hij antwoordt op de frequentie van het station dat wordt geroepen. Gebruik Hold TX om vast te houden.
setup-hold-tx-frequency = TX-frequentie vasthouden
setup-pin-transmit-tone-where-nothing = Zet de zendtoon vast waar hij is. Niets verplaatst hem: niet het beantwoorden van een station, niet de oproepwachtrij, niet CQ roepen, niet een klik op een decode of de waterval. Heeft voorrang op Auto TX-frequentie. Bandwissel is de enige uitzondering: de offset die je het laatst op de nieuwe band instelde komt ermee terug. Voor waar je vergunning smaller is dan het bandplan — op een Britse 60 m-afstemming van 5357 kHz eindigt de toewijzing op 5358,0, dus de toon moet onder 1000 Hz blijven.
setup-tx-watchdog = TX-watchdog
setup-stop-transmitting-after-long-no = Stop met zenden na zo lang zonder antwoord en zonder actie van jou. 0 schakelt het uit.
setup-give-up-after = Opgeven na
setup-calls = {" "}oproepen
setup-contest = Contest
setup-everyday-exchange-grid-report-rr73 = De alledaagse uitwisseling: locator, rapport, RR73.
setup-european-vhf-contests-send-signal = Europese VHF-contests: verstuur een signaalrapport, een volgnummer en je ZES-tekens-locator, in de berichtindeling die WSJT-X EU VHF Contest noemt. CQ gaat uit als CQ TEST.
setup-serial = Volgnummer
setup-serial-number-next-exchange-will = Het volgnummer dat de volgende uitwisseling draagt. Het loopt vanzelf op bij elk gelogd contact en wordt tussen sessies onthouden — stel het alleen met de hand in om een contest te beginnen, of om verder te gaan waar een papieren log was gebleven.
setup-needs-6-character-grid = ⚠ vereist een locator van 6 tekens
setup-eu-vhf-contest-messages-carry = EU VHF Contest-berichten bevatten een locator van zes tekens (JN88DD, niet JN88). Tot Mijn locator er een heeft, wordt de gewone uitwisseling verstuurd.
setup-dxpedition = DXpeditie
setup-ordinary-ft8-operation = Gewone FT8-werking.
setup-calling-dxpedition-running-fox-mode = Een DXpeditie roepen die in Fox-modus draait: roep van boven 1000 Hz, ga naar beneden naar de Fox als hij antwoordt, en log op zijn RR73 zonder 73 te sturen.
setup-run-pile-up-several-signals = Draai de pile-up: meerdere signalen tegelijk, een wachtrij van oproepers, sterkste en zeldzaamste eerst gewerkt. CQ ROEPEN start het, STOP QSO zet het stil.
setup-fox-signals = Fox-signalen
setup-once = {" "}tegelijk
setup-simultaneous-transmissions-spaced-60-hz = Gelijktijdige zendingen, 60 Hz uit elkaar. Ze delen het vermogen van de zender, dus meer signalen betekent dat elk zwakker is.
setup-tx-audio = TX-audio
setup-deviation-fm = deviatie, bij FM
setup-drive-ssb = drive, bij SSB
setup-grid = Locator
setup-report = Rapport
setup-r-report = R+Rapport

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = , schaal 10 W, herschaald
amp-t-10-w-scale = , schaal 10 W
amp-t-2-w-scale-rescaled = , schaal 2 W, herschaald
amp-t-2-w-scale = , schaal 2 W
amp-t-100-w-scale-rescaled = , schaal 100 W, herschaald
amp-t-100-w-scale = , schaal 100 W
amp-t-20-w-scale-rescaled = , schaal 20 W, herschaald
amp-t-20-w-scale = , schaal 20 W
amp-rescaled = , herschaald
amp-and = {" "}EN{" "}
amp-not-while-transmitting-switching-relay = Niet tijdens zenden: een relais schakelen terwijl er RF op staat kan het beschadigen. Stop eerst met zenden.
amp-no-amplifier-set-up-choose = Er is geen eindversterker ingesteld. Kies de poort onder Instellingen → Eindversterker.
amp-settings = INSTELLINGEN
amp-dock = VASTZETTEN
amp-put-amp-window-back-inside = Zet het AMP-venster terug in het hoofdvenster
amp-pop-out = LOSMAKEN
amp-give-amp-window-window-own = Geef het AMP-venster een eigen venster, om naar een ander scherm te verplaatsen
amp-rig = , van de rig
amp-rig-flagged-reading = , de rig markeerde zijn meting
amp-drv = , van DRV
amp-rig-s-swr = , van de SWR van de rig
amp-ptt-keys-amplifier-radio-off = PTT schakelt de eindversterker samen met de radio; UIT laat hem in bypass. Klik om te wisselen.
amp-switch-antenna-jack = Antenne-aansluiting omschakelen
amp-band-down = Band lager
amp-band-up = Band hoger
amp-tuner = TUNER
amp-tuned = Afgestemd
amp-tune = TUNE
amp-start-tune-then-key-steady = Start een afstemming — zend daarna een constante draaggolf (CW, FM of RTTY). Druk nogmaals om te stoppen.
amp-put-tuner-line-bypass = Schakel de tuner in de lijn, of omzeil hem
amp-tuning = Afstemmen…
amp-bottom = ONDER
amp-pick-any-reading-either-needle = Kies een willekeurige meting voor elke naald. De bovenste naald is wit, de onderste blauw.
amp-forward-power-measured-radio-amplifier = Voorwaarts vermogen, gemeten door de radio: de eindversterker meldt er geen in UIT
amp-forward-power-taken-drv-amplifier = Voorwaarts vermogen, overgenomen van DRV: de eindversterker meldt er geen in UIT, en in bypass gaat het vermogen van de radio rechtstreeks door
amp-forward-power = Voorwaarts vermogen
amp-reflected-power-measured-radio-amplifier = Gereflecteerd vermogen, gemeten door de radio: de eindversterker meldt er geen in UIT
amp-reflected-power-worked-out-fwd = Gereflecteerd vermogen, berekend uit FWD en de SWR: de eindversterker meldt er geen in UIT
amp-reflected-power = Gereflecteerd vermogen
amp-standing-wave-ratio-red-past = Staandegolfverhouding: rood bij of voorbij de afstemlimiet (Instellingen → Algemeen). Controleer de antenne
amp-standing-wave-ratio-measured-radio = Staandegolfverhouding, gemeten door de radio: de eindversterker meldt er geen in UIT
amp-standing-wave-ratio = Staandegolfverhouding
amp-drive-radio = Aansturing door de radio
amp-drain-supply = Drainvoeding
amp-drain-current = Drainstroom
amp-temp = TEMP
amp-heat-spreader-temperature-yellow-49 = Temperatuur van de koellichaamplaat: geel vanaf 49 °C (120 °F), rood boven 65 °C (149 °F), uit de lijn boven 70 °C (158 °F). Klik voor °C/°F.
amp-c = °C
amp-f = °F
amp-performance = PRESTATIES
amp-numbers-hold-peaks-bars-are = cijfers houden pieken vast · balken zijn live
amp-input-power = INGANGSVERMOGEN
amp-supply-volts-drain-amps = Voedingsspanning × drainstroom
amp-efficiency = RENDEMENT
amp-output-power-input-power = Uitgangsvermogen ÷ ingangsvermogen
amp-gain = GAIN
amp-output-power-over-drive-decibels = Uitgangsvermogen ten opzichte van de aansturing, in decibel
amp-off-line-over-temperature = UIT DE LIJN · te hoge temperatuur
amp-connected-transmitting = VERBONDEN · ZENDEN
amp-connected = VERBONDEN
amp-connecting = Verbinden…
amp-atu-bypassed = ATU OMZEILD
amp-no-inductance-needed = geen inductantie nodig
amp-no-capacitance-needed = geen capaciteit nodig
amp-c-out-antenna-side = C UIT · antennezijde
amp-c-transmitter-side = C IN · zenderzijde

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = Ontvangen delen (verlicht) en nog ontbrekende (donker)
sstv-sstv = SSTV
sstv-auto = Auto
sstv-signal = Signaal
sstv-station-sending-mode-sdroxide-does = Een station zendt in een modus die sdroxide niet heeft. Het signaal en de afstemming zijn in orde — er is gewoon geen decoder voor deze. Aan jouw kant hoeft niets te worden aangepast.
sstv-listening = luisteren…
sstv-restart-rx = RX herstarten
sstv-abandon-picture-being-received-listen = Laat het beeld dat wordt ontvangen vallen en luister naar de volgende header. Voor een uitzending die in de verkeerde modus is gaan decoderen — de ontvanger zit anders vast totdat die modus afloopt.
sstv-callsign-last-station-sent-fsk = De roepletters die het laatste station na zijn beeld als FSK-ID heeft verzonden.
sstv-tx-slant = TX-scheefstand
sstv-transmit-clock-trim-ppm-remove = Trimming van de zendklok (ppm) om scheefstand op de decoder aan de andere kant te verwijderen
sstv-reset-0-ppm = Terugzetten op 0 ppm
sstv-fsk-id = FSK-ID
sstv-send-your-callsign-tones-after = Zend na elk beeld je roepletters in tonen — de identificatie die SSTV-repeaters en andere programma's lezen. Voegt ongeveer 2,5 seconden toe en zendt helemaal niets zolang je geen roepletters hebt ingesteld.
sstv-tx-lead = TX-voorloop
sstv-silence-sent-after-keying-before = Stilte die na het inschakelen van de zender wordt verzonden, vóór de leader en VIS-code van het beeld. Een decoder die iets van die header mist, toont helemaal geen beeld; dit overbrugt dus de tijd tussen het vragen om PTT aan een rig en het moment dat die echt in de lucht is. 0 voor een SDR die direct schakelt.
sstv-live = LIVE
sstv-waiting-picture = wachten op een beeld…
sstv-no-low-audio = geen / weinig audio
sstv-received = ONTVANGEN
sstv-reading-radio-s-pictures = Beelden van de radio lezen…
sstv-decoded-pictures-collect-here = Gedecodeerde beelden verzamelen zich hier.
sstv-click-enlarge-right-click-delete = Klik om te vergroten · rechtsklik om te verwijderen
sstv-delete-picture = Dit beeld verwijderen
sstv-loading = laden…
sstv-transmit = ZENDEN
sstv-click-edit-slot-s-message = Klik om het bericht van dit vak te bewerken · dubbelklik om een afbeelding te laden
sstv-change-image = Afbeelding wijzigen…
sstv-load-image = Afbeelding laden…
sstv-clear = Wissen
sstv-empty-slot-s-picture-message = Maak het beeld van dit vak leeg (het bericht blijft)
sstv-banner = Banner…
sstv-what-printed-across-top-every = Wat bovenaan elk beeld wordt afgedrukt dat dit station zendt
sstv-fetching-slot-s-picture = Beeld van dit vak ophalen…
sstv-load-image-into-slot = Laad een afbeelding in dit vak →
sstv-drawn-slot-s-image = Getekend op de afbeelding van dit vak
sstv-abort-tx = {" "}TX AFBREKEN{" "}
sstv-delete-picture-store = Dit beeld uit de opslag verwijderen
sstv-received-image = Ontvangen afbeelding
sstv-no-longer-store = niet meer in de opslag
sstv-loading-full-size = volledige grootte laden…
sstv-save-picture = Beeld opslaan…
sstv-delete-sure = Verwijderen — zeker?
sstv-click-again-delete-good = Klik nogmaals om het definitief te verwijderen
sstv-delete = Verwijderen…
sstv-image-banner = Afbeeldingsbanner
sstv-drawn-into-every-picture-station = In elk beeld getekend dat dit station zendt, over het eigen bericht van het vak heen.
sstv-draw-banner = De banner tekenen
sstv-substituted-when-picture-composed = Vervangen wanneer het beeld wordt samengesteld:
sstv-top-left = Linksboven
sstv-top-right = Rechtsboven
sstv-colours = Kleuren
sstv-strip-top-edge-fades-black = De strook, aan de bovenrand — ze vervaagt naar zwart, of naar de verloopkleur eronder
sstv-both-texts = Beide teksten
sstv-gradient = Verloop
sstv-fade = vervagen naar
sstv-fade-strip-top-colour-second = Laat de strook van haar bovenkleur naar een tweede kleur vervagen in plaats van naar zwart.
sstv-text-outline = Tekstomlijning
sstv-draw-outline-around-banner-text = Teken een omlijning rond de bannertekst, zodat die leesbaar blijft op een drukke of lichte strook.
sstv-text-gradient = Tekstverloop
sstv-fade-banner-text-colour-second = Laat de bannertekst over de breedte van het beeld van zijn kleur naar een tweede kleur vervagen.
sstv-rainbow-text = Regenboogtekst
sstv-all-text-overriding-colours-above = alle tekst, de kleuren hierboven overschrijvend
sstv-draw-every-text-picture-banner = Teken alle tekst in het beeld — banner en bericht — als horizontale regenboog, zonder de kleuren hierboven.
sstv-message = Bericht
sstv-slot-message-s-text-colour = De tekstkleur van het vakbericht
sstv-outline = omlijning in
sstv-height = Hoogte
sstv-how-tall-strip-pixels-transmitted = Hoe hoog de strook is, in pixels van het verzonden beeld. De tekst wordt hierop afgestemd — een SSTV-frame is maar 320 pixels breed, dus een hogere banner maakt het aan de andere kant leesbaar.
sstv-reset = Terugzetten
sstv-back-callsign-left-version-right = Terug naar de roepletters links en de versie rechts
sstv-waiting-station-s-digital-mode = Wachten tot de digitale-modusinstellingen van het station zijn geladen…
sstv-rifp = RIFP
sstv-size = Grootte
sstv-calling-frequency-draft-names = De aanroepfrequentie die het ontwerp noemt
sstv-encode = Codering
sstv-how-picture-encoded-into-object = Hoe het beeld wordt gecodeerd tot het object dat RIFP draagt. Auto probeert ze allemaal en zendt de kleinste.
sstv-try-every-encoding-send-smallest = Probeer elke codering, zend de kleinste (nooit met verlies)
sstv-gray = Grijs
sstv-grayscale-depth-rifp-s-raster = Grijswaardendiepte. Het raster van RIFP is per definitie grijswaarden — kleur heeft geen plaats in het manifest.
sstv-dither = Dither
sstv-diffuse-quantisation-error-worth-below = Kwantisatiefout verspreiden — de moeite waard onder 8 bits
sstv-caption = Bijschrift
sstv-sent-manifest-content-hint-shown = Verzonden in het manifest als inhoudsaanwijzing en door ontvangers onder het beeld getoond. Reist als tekst — het wordt niet in de afbeelding getekend.
sstv-what-picture = Wat dit beeld is
sstv-repeat-data = Data herhalen
sstv-send-every-data-frame-many = Zend elk dataframe zo vaak. RIFP is eenrichtingsverkeer zonder verzoeken om herstel, dus dit is het enige herstel dat een ontvanger krijgt.
sstv-chunk = Deel
sstv-payload-octets-per-data-frame = Octetten nuttige lading per dataframe (het profiel adviseert 192)
sstv-valid-frames-frames-failed-crc = Geldige frames, frames met een CRC-fout en volledige geverifieerde beelden
sstv-no-transfer-progress = geen overdracht bezig
sstv-forget-incomplete-transfer = Deze onvolledige overdracht vergeten

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = Oproepen
windows-delete = Verwijderen
windows-edit-name-frequency-mode-antenna = Bewerk de naam, frequentie, modus, antenne en repeater-instelling
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = De repeaterversprong, toon en 1750 Hz-burst die bij dit kanaal zijn opgeslagen
windows-abandon-edit-escape = Bewerking afbreken (Escape)
windows-save = OPSLAAN
windows-keep-changes-enter = De wijzigingen behouden (Enter)
windows-recall-channel-without-moving-antenna = Dit kanaal oproepen zonder de antenne te verplaatsen
windows-antenna-switch-when-channel-recalled = De antenne waarnaar wordt geschakeld wanneer dit kanaal wordt opgeroepen. — laat de radio op wat hij al gebruikt.
windows-shift = Versprong
windows-tone = Toon
windows-invert = OMKEREN
windows-normal = NORMAAL
windows-dcs-polarity = DCS-polariteit
windows-open-every-over-tone-burst = Open elke overgang met de toonburst zolang dit kanaal is opgeroepen
windows-memories = Geheugens
windows-memory-name = geheugennaam
windows-store = Opslaan
windows-folder-name = mapnaam
windows-new-folder = Nieuwe map
windows-import = IMPORTEREN
windows-read-channel-list-chirp-csv =
    Lees een kanaallijst uit een CHIRP CSV-bestand (.csv) — een export van een repeaterlijst, een marine- of PMR-kanaaltabel, of al het andere dat CHIRP kan schrijven.

    Elk kanaal brengt zijn frequentie, modus, repeaterversprong en CTCSS- of DCS-toon mee. Kanalen die al in deze lijst staan — dezelfde frequentie, dezelfde modus — worden overgeslagen, zodat opnieuw importeren van een bijgewerkte lijst toevoegt wat nieuw is in plaats van te verdubbelen wat dat niet is.
windows-chirp-csv = CHIRP CSV
windows-export = EXPORTEREN
windows-write-list-out-chirp-csv = Schrijf deze lijst weg als CHIRP CSV-bestand, om in een handheld te laden of als back-up te bewaren.
windows-no-memories-yet = nog geen geheugens
windows-drag-memory-onto-folder-file = sleep een geheugen op een map om het in te delen, eronder om het uit te delen
windows-drop-here-unfile = hier neerzetten om uit te delen
windows-sort = Sorteren
windows-order-they-were-stored = In de volgorde van opslaan
windows-name-ignoring-case = Op naam, hoofdletters negerend
windows-frequency = Op frequentie
windows-band-then-frequency-inside-each = Op band, daarna op frequentie binnen elke band
windows-descending-click-ascending = Aflopend — klik voor oplopend
windows-ascending-click-descending = Oplopend — klik voor aflopend
windows-delete-folder-memories-move-top = Map verwijderen — de geheugens gaan naar het hoogste niveau
windows-rename-folder = Map hernoemen
windows-empty-drop-memories-here = leeg — zet geheugens hier neer
windows-voice-keyer = Voice keyer
windows-rec-records-your-microphone-play = REC neemt op via je microfoon, PLAY laat je beluisteren wat je hebt opgenomen, TX zendt het uit — evenals een numpad-toets, een MIDI-pad of send_voice_mem van rigctld.
windows-stop-store = Stoppen en opslaan
windows-stop = STOP
windows-play = PLAY
windows-stop-listening = Stoppen met luisteren
windows-nothing-recorded-slot = Niets opgenomen in dit vak
windows-not-while-transmitting = Niet tijdens zenden
windows-listen-message-nothing-transmitted = Beluister dit bericht — er wordt niets gezonden
windows-stop-transmitting = Stoppen met zenden
windows-tune-active-switch-off-first = TUNE is actief — schakel het eerst uit
windows-transmit-message = Dit bericht zenden
windows-erase-recording = Deze opname wissen
windows-rade-message-encoded-digital-voice = RADE: het bericht wordt gecodeerd door de digitale-spraakcodec, precies zoals een live overgang zou worden.

## TX Audio window: messages with values

txaudio-rec-hint = Neem een testzin op via de microfoon — maximaal { $max } s, zonder de zender in te schakelen. Klik nogmaals om te stoppen. Hij wordt onbewerkt bewaard, zodat PLAY dezelfde zin door elke wijziging die je maakt kan halen.
txaudio-phrase-length = Zin van { $secs } s
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = Ingangsgain ingesteld op { $gain } dB (spraakpieken waren { $loud } dBFS).{ $clip }
txaudio-listening = Luisteren… { $secs } s
txaudio-auto-set-hint = Praat { $secs } s normaal — zoals je in de lucht zou doen — en de ingangsgain wordt zo ingesteld dat je spraakpieken rond { $target } dBFS liggen. Klik nogmaals om te annuleren.
txaudio-eq-point = { $hz } Hz, { $db } dB — sleep om te verplaatsen
txaudio-not-a-profile = Geen TX-audioprofiel: { $error }
txaudio-imported = "{ $name }" geïmporteerd.
txaudio-saved = "{ $name }" opgeslagen.
txaudio-deleted = "{ $name }" verwijderd.
txaudio-loaded-preset = Beginpunt "{ $name }" geladen.

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · offline
solar-radius-exaggeration = Overdrijving van aard-/maanstraal (max { $max }× bij deze schaal van de maanbaan)
solar-brightest-cell = helderste cel ≈ { $peak } paden · { $bands } { $bands ->
        [one] band
       *[other] banden
    }
solar-hits-in-hour = { $hits } in het uur
solar-passes-from = { $name } · OVERVLUCHTEN VANUIT { $grid }
solar-elements-age = elementen { $age } oud · SGP4
solar-point-at = Richt op { $az }° ({ $compass }), elevatie { $el }°.
solar-published-as = gepubliceerd als { $name }
solar-fly-there = { $name } — ↵ om erheen te vliegen
solar-sats-tracked = { $n } van { $total } gevolgd
solar-bodies-found = { $n } van { $total } hemellichamen
solar-sats-and-bodies = { $n } van { $total } gevolgd · { $bodies } van { $bodies_total } hemellichamen
solar-confidence-on-band = { $confidence } op { $band }
solar-kp-now = { $kp } nu
solar-kp-in = { $kp } over { $hours } u
solar-valid-age = geldig { $valid } · { $age } oud
solar-full-bar = volle balk = { $share } van de wereld
solar-own-paths-memory = waar de eigen paden van dit station heen gingen · { $minutes } min geheugen
solar-n-storms = { $n } onweersbuien
solar-clouds-legend = WOLKEN { $time } · { $age } oud · { $channels } · { $storms } · bliksem gesimuleerd
solar-cme-eta = ETA { $eta } (+{ $hours } u)
solar-cme-arrived = aankomst was { $eta } ({ $hours } u geleden)
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = OP DE AARDE GERICHTE CME { $time } · { $speed } km/s · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = { $mode }-instellingen
setup-transmits-as = Zendt als { $call }
setup-decimal-degrees = Decimale graden, negatief voor { $hemisphere }. Plak er gewoon een uit een kaart.

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } ONBETROUWBAAR BIJ DEZE SWR
amp-lower-drive-first = Verlaag eerst de aansturing: { $drive } W zou de eindversterker in de lijn overbelasten (de limiet is { $limit } W).
amp-live-value = live { $value } { $unit }
amp-tripped = AFGESLAGEN{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = top ≈ { $n } paden
panels-no-agreed-frequency = Er is geen frequentie afgesproken voor { $mode } — open dit om degene waarop je zit op te slaan
panels-mode-frequencies-hint = De { $n } frequenties voor { $mode } — een kiezen stemt de afstemknop af, en je kunt je eigen opslaan
panels-mode-frequencies = { $mode } · frequenties
panels-global-convention = Een wereldwijde afspraak waarop het IARU Regio { $region } bandplan geen smalle data plaatst — controleer je eigen bandplan voordat je hier zendt.
panels-outside-data-segment = Oranje: buiten het datasegment van Regio { $region }.
panels-save-dial = ＋ { $mhz } MHz opslaan
panels-save-dial-hint = Onthoud deze afstemming onder { $mode } en bied haar vanaf nu hier aan. Opgeslagen op het station, dus elk scherm dat met deze radio is verbonden heeft haar.
panels-tr-period = T/R-periode van { $period } seconden
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = Deze beurt: { $into } s van { $slot }. Decodes komen aan het einde ervan binnen, en dan mag de volgende zending beginnen.{ $mark }
panels-burst-mark = {" "}De markering is waar de burst stopt, { $secs } s na het begin.
panels-tr-period-burst = T/R-periode van { $slot } seconden, burst van { $burst } s

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } { $n ->
        [one] kanaal
       *[other] kanalen
    }
windows-slot-n = Vak { $n }
windows-record-from-mic = Opnemen via de microfoon (maximaal { $max } s)

## SSTV panel: messages with values

sstv-picture-too-big = Dat beeld is { $size } MB — de limiet is { $limit } MB.
sstv-auto-mode = Auto ({ $mode })
sstv-not-decoded = { $what } — niet gedecodeerd
sstv-last = laatste: { $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = Gedecodeerde beelden worden { $place } opgeslagen en verzamelen zich hier.
sstv-older-in-store = { $n } oudere in de opslag
sstv-older-load-more = { $n } oudere — meer laden
sstv-delete-picture-from = Dit beeld verwijderen { $place }
sstv-rifp-profile = { $profile } · ~{ $khz } kHz bezet · afstemming is het kanaalmidden
sstv-rifp-too-wide = ⚠ { $profile } bezet ~{ $khz } kHz — te breed voor een smalbandig segment
sstv-rifp-no-frequency = RIFP wijst geen frequentie toe, en sdroxide zendt het uit waar je ook afstemt. Een kanaal van { $khz } kHz past alleen waar breedband- of FM-gebruik is toegestaan — { $segments } — en niet in een smalbandig segment, al helemaal niet op HF. Zelfs daarbinnen kunnen je eigen licentievoorwaarden smaller zijn. Jij bent de operator; controleer je eigen regels.
sstv-rifp-tx-frame = ● TX-frame { $frame }/{ $frames } · nog { $secs } s
sstv-rifp-sent-as = verzonden als { $encoding } · { $octets } octetten
sstv-rifp-rx-stats = frames { $frames } · slecht { $bad } · beelden { $pictures }
sstv-rifp-session-idle = sessie { $session } · inactief { $secs } s
sstv-rifp-session-waiting = sessie { $session } · delen vastgehouden, wachten nog op het manifest · inactief { $secs } s
sstv-store-on-radio = op de radio, in { $dir }
sstv-store-in = in { $dir }

## decodes (src/app/panels/decodes.rs)

decodes-decodes = DECODES
decodes-adif = ADIF
decodes-sort = Sorteren
decodes-single-list = Enkele lijst
decodes-every-decode-one-list-newest = Elke decode in één lijst, nieuwste beurt eerst, in plaats van blokken met oneven/even beurten. De sorteerchips ordenen dan de hele lijst in plaats van elke beurt, en elke rij toont haar slottijd — cyaan voor een even slot, goud voor een oneven.
decodes-cq-only = Alleen CQ
decodes-only-stations-calling-cq-only = Alleen stations die CQ roepen — en alleen de oproepen die je mag beantwoorden: een gerichte CQ (DX, EU, JA, POTA, TEST …) wordt getoond als hij jou noemt en verborgen als hij iemand anders noemt.
decodes-new-only = Alleen nieuw
decodes-only-stations-would-new-entity = Alleen stations die nieuw zouden zijn: entiteit, bandslot, grid of roepletters
decodes-auto-tx-frq = Auto TX-FRQ
decodes-overridden-hold-tx-lift-hold = Overschreven door Hold TX. Heft de hold op om te kiezen welke kant de zendfrequentie op beweegt.
decodes-hold-tx = Hold TX
decodes-pin-transmit-tone-where-nothing =
    Zet de zendtoon vast waar hij is. Niets verplaatst hem: niet het beantwoorden van een station, niet de oproepwachtrij, niet CQ roepen, niet een klik op een decode of op de waterfall. Zet het uit om te verplaatsen, en daarna weer aan.

    Van band wisselen is de enige uitzondering, en het is je eigen handeling: de offset die je het laatst op de nieuwe band instelde komt er weer mee terug, zodat een waarde van 60 m niet naar 20 m wordt meegenomen en die van 20 m niet naar 60 m.

    Voor waar de licentie smaller is dan het bandplan — bij een Britse 60 m-afstemming van 5357 kHz eindigt de toewijzing op 5358,0, dus moet de toon onder 1000 Hz blijven, en elke automatische verplaatser loopt tussen de ene overgang en de volgende de band uit.
decodes-type-transmit-offset-hz-then = Typ de zendoffset in Hz en druk dan op Enter. Waar je licentie smaller is dan het bandplan, mag je uit het hele bereik onder de rand kiezen, dus hier wordt niets voorgesteld.
decodes-clear-rx = {" "}RX WISSEN{" "}
decodes-empty-decode-list-nothing-air = Maak de decodelijst leeg. Niets dat in de lucht is stopt.
decodes-even = EVEN
decodes-text = TEKST
decodes-dxcc = DXCC
decodes-band = BAND
decodes-grid = GRID
decodes-dupe = DUBBEL
decodes-reply = ANTWOORD
decodes-queued-click-remove = In de wachtrij — klik om te verwijderen
decodes-work-station-after-current-one = Werk dit station na het huidige
decodes-qsos-worked-since-sdroxide-was = QSO's gewerkt sinds sdroxide is gestart
decodes-qso-complete = ✓ QSO VOLTOOID
decodes-tx = ● TX
decodes-dxpedition-mode-transmit-frequency-held = DXpedition-modus. De zendfrequentie blijft uit de helft van de Fox in de doorlaatband (onder 1000 Hz) tot de Fox antwoordt.
decodes-watchdog = WATCHDOG
decodes-transmitting-stopped-no-reply-no = Zenden gestopt: geen antwoord en geen handeling gedurende de watchdogperiode. Roep CQ of kies een bericht om te hervatten.
decodes-well-inside-tolerance = Ruim binnen de tolerantie.
decodes-no-active-qso-pick-decode = geen actieve QSO — kies een decode om te antwoorden, of roep CQ
decodes-ft8-engine-idle = FT8-engine inactief
decodes-being-worked = wordt gewerkt
decodes-clear = WISSEN
decodes-no-messages = — geen berichten —
decodes-call-cq = {"  "}CQ ROEPEN{"  "}
decodes-stop-qso = {" "}QSO STOPPEN{" "}
decodes-stop-tx = {" "}TX STOPPEN{" "}
decodes-free-text-13-chars = vrije tekst (13 tekens)
decodes-send = ZENDEN

## sat (src/app/sat.rs)

sat-pass-profile = OVERVLUCHTPROFIEL
sat-satellite = SATELLIET
sat-satellites = SATELLIETEN
sat-passes = OVERVLUCHTEN
sat-set-your-grid-passes-settings = Stel je grid in voor overvluchten (Instellingen ▸ Algemeen).
sat-next = VOLGENDE
sat-peak = PIEK ≥
sat-all-satellites = alle satellieten
sat-off-only-satellites-frequencies-file = Uit: alleen satellieten met frequenties in het bestand — die je kunt werken — en alle die je zelf hebt geplakt. Aan: elke satelliet in de lijst, wat er met een abonnement duizenden kunnen zijn en even duurt om te doorzoeken.
sat-no-passes-window-lower-peak = Geen overvluchten in dit venster. Verlaag de piek, of verbreed het venster.
sat-length = DUUR
sat-peak-2 = PIEK
sat-from-to = VAN → NAAR
sat-up-now = ● nu boven
sat-unlock = ONTGRENDELEN
sat-release-lock-dial-stays-where = Laat de vergrendeling los; de afstemknop blijft waar hij is
sat-azimuth = AZIMUT
sat-elevation = ELEVATIE
sat-range = AFSTAND
sat-rate = SNELHEID
sat-downlink = DOWNLINK
sat-doppler-rx = DOPPLER RX
sat-uplink = UPLINK
sat-doppler-tx = DOPPLER TX
sat-no-pass-inside-next-48 = Geen overvlucht in de komende 48 uur vanaf je QTH.
sat-elements-stale-corrections-suspended-until = ⚠ Elementen verouderd — correcties opgeschort tot een TLE-verversing
sat-doppler-correction-needs-iq-front = Dopplercorrectie vereist een IQ-front-end — een CAT-rig volgt alleen
sat-rotator-not-connected = Rotor: niet verbonden
sat-doppler = DOPPLER
sat-apply-computed-correction-rx-tx = Pas de berekende correctie toe op RX en TX
sat-rotator = ROTOR
sat-steer-antenna-through-rotctld-client = Stuur de antenne aan via de rotctld-client (Instellingen ▸ Servers)
sat-name-norad = naam of NORAD
sat-set-your-grid-passes-settings-2 = stel je grid in voor overvluchten (Instellingen ▸ Algemeen)
sat-norad = NORAD
sat-pass = OVERVLUCHT
sat-always-visible = altijd zichtbaar
sat-not-here = niet vanaf hier
sat-no-elements = geen elementen
sat-pick-satellite-see-links-lock = Kies een satelliet om zijn verbindingen te zien en te vergrendelen.
sat-no-frequencies-file-one-add = Geen frequenties bekend voor deze — voeg ze toe in Instellingen ▸ TLE.
sat-link = LINK
sat-downlink-mhz = DOWNLINK (MHz)
sat-uplink-mhz = UPLINK (MHz)
sat-mode = MODUS
sat-tune = TUNE
sat-set-dial-mode-link-no = Stel afstemming en modus in op deze link — geen vergrendeling, geen correctie
sat-locked = {" "}VERGRENDELD{" "}
sat-lock = {" "}VERGRENDELEN{" "}
sat-track-satellite-doppler-correction-rx = Volg deze satelliet: dopplercorrectie op RX en TX, transponder-gekoppelde uplink, en de 3D-weergave volgt hem

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ STOP{" "}
wefax-end-chart-now-save-what = Beëindig de kaart nu en sla op wat is binnengekomen
wefax-start = {" "}● START{" "}
wefax-start-chart-now-without-waiting = Start nu een kaart, zonder op een starttoon te wachten
wefax-phasing = fasering…
wefax-subcarrier-offset-1900-hz-tune = Offset van de subdraaggolf vanaf 1900 Hz. Stem af op ongeveer nul: de faxdraaggolf loopt van 1500 Hz zwart tot 2300 Hz wit, en een ontvanger die een paar honderd hertz ernaast zit, knipt het beeld naar effen zwart of effen wit.
wefax-auto-start = AUTO START
wefax-auto-stop = AUTO STOP
wefax-end-450-hz-stop-tone = Beëindig op de stoptoon van 450 Hz. Zet uit om te blijven opnemen bij een station dat meerdere kaarten achter elkaar zendt.
wefax-phase = FASE
wefax-slant = SCHEEFSTAND
wefax-sample-clock-trim-if-chart = Trimming van de samplekloek. Als de kaart naar links helt, verhoog dit; naar rechts, verlaag het. Een geluidskaart die honderd ppm afwijkt, laat een kaart van een kwartier bijna een regel opzij lopen.
wefax-view = WEERGAVE
wefax-scale-chart-panel-width = Schaal de kaart naar de breedte van het paneel
wefax-whole = GEHEEL
wefax-shrink-chart-until-all-view = Verklein de kaart tot ze helemaal in één keer in beeld is
wefax-half-size = Halve grootte
wefax-one-screen-pixel-per-fax = Eén schermpixel per faxpixel — scroll voor detail
wefax-twice-size-scroll-move-around = Dubbele grootte; scroll om over de kaart te bewegen
wefax-height = HOOGTE
wefax-stretch-picture-vertically-chart-came = Rek het beeld verticaal uit. Een kaart die samengedrukt of uitgerekt is uitgekomen, wordt meestal met de verkeerde regelsnelheid gedecodeerd — dit maakt haar leesbaar, en de LPM-chips lossen het goed op. Dubbelklik om een waarde te typen.
wefax-reset = TERUGZETTEN
wefax-back-picture-s-own-proportions = Terug naar de eigen verhoudingen van het beeld
wefax-follow = VOLGEN
wefax-keep-newest-lines-view-scrolling = Houd de nieuwste regels in beeld. Omhoog scrollen zet dit uit zodat je kunt lezen wat al is binnengekomen; terug naar beneden scrollen zet het weer aan.
wefax-saved = OPGESLAGEN
wefax-path = PAD
wefax-reading-radio-s-charts = Kaarten van de radio lezen…
wefax-finished-charts-collect-here = Voltooide kaarten verzamelen zich hier.
wefax-delete-chart = Deze kaart verwijderen
wefax-loading = laden…
wefax-stations = STATIONS
wefax-broadcast-radiofax-schedules-picking-one = Radiofax-uitzendschema's — een kiezen stemt de afstemknop af en zet de lijnsnelheid
wefax-frequencies-are-published-carrier-dial = Frequenties zijn de gepubliceerde draaggolf; de afstemming gaat 1,9 kHz eronder, dat wordt voor je gedaan. Schema's veranderen en stations sluiten — zie dit als waar je kunt beginnen te zoeken, niet als dienstregeling.
wefax-delete-chart-store = Deze kaart uit de opslag verwijderen
wefax-newer = ◀ NIEUWER
wefax-chart-received-after-one = De kaart die na deze is ontvangen
wefax-older = OUDER ▶
wefax-chart-received-before-one = De kaart die vóór deze is ontvangen
wefax-no-longer-store = niet meer in de opslag
wefax-loading-full-size = volledige grootte laden…
wefax-save = OPSLAAN
wefax-save-copy-chart-computer = Sla een kopie van deze kaart op deze computer op
wefax-sure = ZEKER?
wefax-click-again-delete-chart-good = Klik nogmaals om deze kaart definitief te verwijderen
wefax-delete = VERWIJDEREN
wefax-file-chart-was-saved = Het bestand waarin deze kaart is opgeslagen

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = APRS-modem starten…
aprs-aprs = APRS
aprs-no-callsign = geen roepletters
aprs-nothing-will-transmitted-no-beacon = Er wordt niets gezonden — geen baken, geen bericht, zelfs geen bevestiging — totdat dit station roepletters heeft. Stel ze in onder Instellingen → Algemeen, of een APRS-specifieke met SSID onder SETUP.
aprs-no-position = geen positie
aprs-beacon-needs-somewhere-report-fill = Een baken heeft een plek nodig om te melden. Vul je locator in onder Instellingen → Algemeen, of geef coördinaten onder SETUP.
aprs-setup = SETUP
aprs-callsign-symbol-digipeater-path-position = Roepletters, symbool, digipeaterpad, positie en baken
aprs-how-often-beacon-your-position = Hoe vaak je positie onbemand wordt gebaken. `off` — de standaard — baken nooit: een modus kiezen mag geen station de lucht in sturen. Dertig minuten is de afspraak voor een vast station, vaker voor een bewegend station, en elk baken is de kanaaltijd van iemand anders.
aprs-beacon = {" "}BAKEN{" "}
aprs-station-has-no-callsign-see = Dit station heeft geen roepletters — zie de waarschuwing links.
aprs-station-has-no-position-report = Dit station heeft geen positie om te melden — zie de waarschuwing links.
aprs-send-one-position-report-now = Zend nu één positiemelding, zonder het interval af te wachten. Er wordt net als bij al het andere gewacht tot het kanaal vrij is.
aprs-stations = STATIONS
aprs-nothing-heard-yet = nog niets gehoord
aprs-heard-direct = direct gehoord
aprs-two-characters-station-sent-say = De twee tekens die het station zond om aan te geven wat het is
aprs-killed = INGETROKKEN
aprs-station-put-object-here-has = Het station dat dit object hier plaatste, heeft het geannuleerd
aprs-sender-deliberately-reported-square-rather = De zender meldde bewust een vierkant in plaats van een punt. De kaart tekent het vierkant.
aprs-digipeaters-last-frame-came-through = De digipeaters waar het laatste frame doorheen kwam. Een `*` markeert er een die het daadwerkelijk heeft herhaald.
aprs-message = {" "}BERICHT{" "}
aprs-address-message-box-station = Adresseer het berichtvak aan dit station
aprs-center = {" "}CENTREREN{" "}
aprs-put-station-middle-map-double = Zet dit station in het midden van de kaart. Dubbelklik op de kaart om de weergave terug te geven aan de automatische passing.
aprs-channel = KANAAL
aprs-messages = BERICHTEN
aprs-frames-arrived-failed-their-check = Frames die zijn aangekomen en hun controlereeks niet doorstonden — een botsing, een fade of een signaal te zwak om te lezen. Een stijgende teller terwijl er niets wordt gedecodeerd betekent dat de modem het kanaal *wel* hoort: controleer de niveaumeter en open de squelch van de radio.
aprs-frames-read-cleanly-off-channel = Frames die zonder fouten van het kanaal zijn gelezen en geen APRS waren — de pakketsessie van iemand anders, of een formaat dat deze build niet decodeert.
aprs-every-frame-channel-arrived-other = Elk frame op het kanaal, zoals het is aangekomen — ook het verkeer van anderen, en alles wat de decoder niet kon lezen.
aprs-nothing-channel-yet = nog niets op het kanaal
aprs-no-messages = Geen berichten.
aprs-waiting-channel = wachten op het kanaal
aprs-sent-retried-still-waiting-acknowledged = verzonden en opnieuw geprobeerd, wacht nog op bevestiging
aprs-air-waiting-acknowledged = in de lucht, wacht op bevestiging
aprs-acknowledged-far-end = bevestigd door de andere kant
aprs-refused-far-end-not-retried = geweigerd door de andere kant — niet opnieuw geprobeerd
aprs-no-ack = geen ack
aprs-no-answer-after-every-retry = geen antwoord na elke nieuwe poging
aprs-send = {" "}ZENDEN{" "}
aprs-station-has-no-callsign-so = Dit station heeft geen roepletters, dus er kan niets worden gezonden. Stel ze in onder Instellingen → Algemeen, of een APRS-specifieke met SSID onder SETUP.
aprs-needs-station-address-something-say = Heeft een station om aan te schrijven en iets om te zeggen nodig.
aprs-send-keep-retrying-until-far = Zend het, en blijf het opnieuw proberen tot de andere kant bevestigt. Berichten zijn het enige op dit kanaal waarop wordt geantwoord.
aprs-busy = BEZET
aprs-stations-heard-still-inside-window = Stations die zijn gehoord en nog binnen het in APRS-instellingen gekozen venster vallen, en of er nu een ander station op het kanaal zit. Niets zendt zolang het bezet is.
aprs-tx = ● TX
aprs-over-progress-frames-waiting-channel = De lopende overgang, de frames die wachten tot het kanaal vrij is, of de tijd tot het volgende geplande baken.
aprs-audio-reaching-modem-flat-means = Audio die de modem bereikt. Vlak betekent dat er helemaal niets aankomt — controleer of de data-uitgang van de radio degene is waarnaar sdroxide luistert. Het licht groen op zolang de modem een draaggolf hoort.

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = VDL2-decoder starten…
vdl2-aircraft-datalink = vliegtuigdatalink
vdl2-tune-middle-vdl2-group-decoder = Stem af op het midden van de VDL2-groep. Het eigen venster van de decoder schuift vanaf daar om zoveel van de veertien kanalen te bestrijken als de ontvanger kan bereiken.
vdl2-setup = SETUP
vdl2-which-channels-listen-how-hard = Op welke kanalen te luisteren, hoe sterk een burst moet zijn en hoeveel log te bewaren
vdl2-save-vdl2-message-log-file = Sla het VDL2-berichtenlog op in een bestand
vdl2-vdl2-channel = VDL2-kanaal
vdl2-messages = BERICHTEN
vdl2-address-flight-registration-label = adres, vlucht, registratie of label
vdl2-held = VASTGEHOUDEN
vdl2-hold = HOLD
vdl2-hold-message-log-where-so = Houd het berichtenlog waar het is, zodat er een gelezen kan worden zonder dat de volgende zending het omhoog het scherm uit duwt. De decoder blijft draaien en de tellers lopen door; loslaten toont alles wat intussen is binnengekomen.
vdl2-nothing-decoded-yet = nog niets gedecodeerd
vdl2-nothing-matches-filter = niets komt overeen met het filter
vdl2-stations = STATIONS
vdl2-nobody-heard-yet = nog niemand gehoord
vdl2-vdl2-setup = VDL2-instellingen
vdl2-channels = Kanalen
vdl2-every-25-khz-slot-136 = Elk 25 kHz-slot van 136,650 tot 136,975 MHz. Elk een eigen downconverter, allemaal binnen hetzelfde ontvangervenster. Er één uitzetten bespaart wat processortijd; het maakt de andere niet gevoeliger.
vdl2-burst-threshold = Burstdrempel
vdl2-above-each-channel-s-own = boven de eigen ruisvloer van elk kanaal
vdl2-lower-catches-weaker-transmissions-costs = Lager vangt zwakkere zendingen op en kost processortijd aan ruis; hoger mist ze. De vloer wordt per kanaal geleerd en een wijziging hier gooit hem niet weg.
vdl2-keep-log = In het log bewaren
vdl2-track-most = Maximaal volgen
vdl2-longest-silent-go-first = de langst stille gaan eerst
vdl2-forget-station-after = Een station vergeten na
vdl2-nothing-heard = zonder iets van het station te horen
vdl2-show-unread-payloads = Ongelezen payloads tonen
vdl2-hex = als hex
vdl2-frames-carrying-x-25-clnp = Frames met X.25, CLNP of de datalinktoepassingen erboven. SDRoxide benoemt ze en toont de bytes in plaats van ze te lezen, en ze verbergen zou verbergen hoeveel van het verkeer dat is.
vdl2-registration = Registratie
vdl2-flight = Vlucht
vdl2-label = Label
vdl2-block = Blok
vdl2-sequence = Reeks
vdl2-mode = Modus
vdl2-more-blocks-follow = meer blokken volgen
vdl2-check = Controle
vdl2-message-crc-good = bericht-CRC goed
vdl2-message-crc-not-checked = bericht-CRC niet gecontroleerd
vdl2-parity = Pariteit
vdl2-link-control-no-payload = linkbesturing, geen payload
vdl2-name = NAAM
vdl2-addr = ADRES
vdl2-type = TYPE
vdl2-msgs = BERICHTEN

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = De toon die wordt gelezen en de toon die wordt gezonden — in CW zijn ze dezelfde frequentie. Klik op de waterfall om hem op een signaal te zetten.
cw-down-10-hz = 10 Hz lager
cw-up-10-hz = 10 Hz hoger
cw-qrg-main-readout-tuning-line = QRG: de hoofdweergave en de afstemlijn staan op de frequentie waarop wordt gewerkt. Klik voor de afstemknop in plaats daarvan, met een zoemtoonhoogte eronder — wat de meeste radio's tonen.
cw-put-main-readout-tuning-line =
    Zet de hoofdweergave en de afstemlijn op het signaal in plaats van op de afstemknop, zodat de getoonde frequentie degene is die beide operators zouden noemen en de afstemlijn in het midden van de doorlaatband staat. De afstemming verandert niet; alleen de getallen verschuiven.

    Een signaal aanklikken zet het alleen zo nauwkeurig op de cursor als de klikstap toelaat — Bediening → afronding bij klik-om-af-te-stemmen, standaard 10 Hz. Een grove stap laat het signaal tot de helft ervan naast de toonhoogte staan, en de weergave zegt dat.
cw-sending-speed-read-off-signal = Zendsnelheid afgelezen van het signaal
cw-signal-noise-500-hz-same = Signaal-ruisverhouding in 500 Hz — dezelfde waarde die een rapport noemt
cw-how-far-off-cursor-signal = Hoe ver het signaal werkelijk van de cursor af zit. De decoder volgt het; de doorlaatband niet, dus schuif de cursor bij als het groeit.
cw-listening = — luisteren —
cw-tx = ● TX
cw-nothing-copied-yet = — nog niets gelezen —
cw-key-send-characters-you-send = Seinen om te zenden — de tekens die je zendt verschijnen hier.
cw-type-line-return-sends = Typ een regel, Return zendt hem…
cw-type-here-send = Typ hier om te zenden…
cw-key = {" "}SEINEN ●{" "}
cw-key-2 = {" "}SEINEN{" "}
cw-hold-key-bound-cw-straight = Houd de toets ingedrukt die aan CW-sleutel is gekoppeld — standaard Spatie, en elke toets die je wilt in Instellingen → Bediening — als een rechte sleutel: omlaag zolang hij is ingedrukt, omhoog bij loslaten, in plaats van tekst te typen. Het zendvak is vergrendeld zolang dit aan staat, en de hele keyer wordt aan de toets overgedragen: alle tekst in de wachtrij wordt verworpen.

    Om met de hand te seinen, stel je CW-keying in op "Geluidskaart (MCW)" in Instellingen → Radio. De rig wordt dan op een zijband gehouden en de eigen zoemtoon van de keyer wordt als audio gezonden, wat de route is die de rechte sleutel aanstuurt.
cw-watchdog = WATCHDOG
cw-straight-key-was-held-down = De rechte sleutel werd te lang ingedrukt gehouden — een gemiste loslating in plaats van een hand — dus de draaggolf is weggenomen en het zenden is uitgeschakeld. Druk de toets nogmaals in om verder te gaan.
cw-tx-2 = {"  "}TX AAN{"  "}
cw-tx-3 = {"   "}TX{"   "}
cw-send-what-box-now-without = Zend nu wat in het vak staat, zonder op Return te wachten
cw-hold-key-down-between-characters = Houd de sleutel tussen tekens omlaag, zodat niets wat is getypt hoeft te wachten
cw-call-cq = {" "}CQ ROEPEN{" "}
cw-nocall = NOCALL
cw-clear = {" "}WISSEN{" "}
cw-stop-sending-drop-whatever-has = Stop met zenden en verwerp alles wat nog niet is uitgegaan. Met de rechte sleutel aan gaat ook de terugleesweergave van wat je seinde mee.
cw-sidetone = ZOEMTOON
cw-play-keyed-tone-through-computer = Speel de geseinde toon ook af via de luidsprekers van deze computer, naast het zenden, zodat je hoort wat je zendt. Bij `Geluidskaart (MCW)` gaat de toon alleen naar de rig, dus zonder dit zend je in stilte. Uit waar de eigen monitor van de rig of een kopie via de ether de taak al doet.
cw-hold-what-typed-until-return = Houd wat is getypt vast tot Return, en zend dan de regel in één keer in plaats van elk teken te seinen terwijl het wordt getypt. Handig bij een transceiver die zelf uit tekst seint, waar elke overdracht aan zijn keyer een extra zend-ontvangstcyclus is.
cw-cw-messages = CW-BERICHTEN
cw-clear-rx = {" "}RX WISSEN{" "}
cw-empty-receive-window-straight-key = Maak het ontvangstvenster en de terugleesweergave van de rechte sleutel leeg. Niets dat in de lucht is stopt.
cw-lock = VERGRENDELEN
cw-decode-speed-set-here-instead = Decodeer op de hier ingestelde snelheid in plaats van die van het signaal af te lezen. Helpt bij een signaal dat te zwak is om de snelheidszoektocht te laten stabiliseren.
cw-farnsworth-send-characters-full-speed = Farnsworth: zend de tekens op volle snelheid en rek alleen de pauzes ertussen, zodat ze in het juiste ritme worden gehoord maar langzaam genoeg aankomen om op te schrijven.
cw-off-normal-spacing = Uit — normale spaties
cw-keying-speed = Seinsnelheid
cw-idle-0 = IDLE 0
cw-how-long-transmit-held-after = Hoe lang het zenden wordt vastgehouden na het laatste teken, of nadat de rechte sleutel is losgelaten, voordat de draaggolf wegvalt. Langer overbrugt de pauzes van een trage typist; korter verlaat de frequentie eerder; Uit laat hem vallen zodra alles in de wachtrij is uitgegaan.
cw-off-drop-once = Uit — direct laten vallen

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = 10 Hz lager afstemmen
textmodem-tune-up-10-hz = 10 Hz hoger afstemmen
textmodem-tx = ● TX
textmodem-listening = — luisteren —
textmodem-type-line-return-sends = Typ een regel, Return zendt hem…
textmodem-type-here-transmit = Typ hier om te zenden…
textmodem-tx-2 = {"  "}TX AAN{"  "}
textmodem-tx-3 = {"   "}TX{"   "}
textmodem-call-cq = {" "}CQ ROEPEN{" "}
textmodem-nocall = NOCALL
textmodem-clear = {" "}WISSEN{" "}
textmodem-hold-what-typed-until-return = Houd wat is getypt vast tot Return, en zend dan de regel in één keer in plaats van elk teken de lucht in te streamen terwijl het wordt getypt. Laat een regel teruglezen en corrigeren voordat er iets van uitgaat.
textmodem-messages = BERICHTEN
textmodem-hell = HELL
textmodem-contrast = Contrast
textmodem-harder-softer-dots-redraws-whole = Hardere of zachtere stippen — tekent de hele strook opnieuw
textmodem-width = Breedte
textmodem-t-2row = {" "}2ROW{" "}
textmodem-draw-every-column-twice-stacked = Teken elke kolom twee keer, gestapeld. Hell heeft geen verticale synchronisatie, dus zo blijft één volledige kopie van de tekst leesbaar, welke fase ook.
textmodem-rev = {" "}REV{" "}
textmodem-reverse-video-light-dots-dark = Omgekeerd beeld — lichte stippen op donker papier
textmodem-clear-rx = {" "}RX WISSEN{" "}
textmodem-hold-channel-idle-sends-blank = Houd het kanaal vast: in rust zendt het leeg papier, zodat de strook blijft scrollen
textmodem-hold-what-typed-until-return-2 = Houd wat is getypt vast tot Return, en zend dan de regel in één keer in plaats van elk teken op de strook te schilderen terwijl het wordt getypt. Een live getypte correctie staat al op het papier; een die vóór Return is gemaakt nooit.
textmodem-shift = Shift
textmodem-shift-5-hz = Shift −5 Hz
textmodem-shift-5-hz-2 = Shift +5 Hz
textmodem-baud = Baud
textmodem-reverse-swap-mark-space-needed = Omkeren: wissel mark en space om. Nodig wanneer het signaal op de tegenovergestelde zijband wordt ontvangen als waarop het is gezonden, wat de tonen omkeert.
textmodem-track-tuning-error-automatically-instead = Volg de afstemfout automatisch in plaats van aan de cursor vast te blijven zitten.
textmodem-tones = Tonen
textmodem-mode = Modus
textmodem-off-keyed = aan/uit geseind

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = AIS-decoder starten…
ais-tune-between-two-ais-channels = Stem af tussen de twee AIS-kanalen — 161,975 (AIS 1, marine 87B) en 162,025 (AIS 2, 88B). De afstemming komt in het midden omdat daar niets zendt, zodat de DC-piek van een zero-IF-ontvanger op geen van beide valt.
ais-setup = SETUP
ais-channels-timeouts-trail-length-how = Kanalen, time-outs, sleeplengte en hoe ver de vectoren vooruit reiken
ais-vessels = SCHEPEN
ais-nothing-heard-yet = nog niets gehoord
ais-virtual-nothing-there = virtueel — er is niets
ais-position-stale = positie verouderd
ais-search-rescue-transmitter-man-overboard = een search-and-rescue-zender, man-overboord-baken of AIS-EPIRB
ais-call-sign = roepletters
ais-station-time = stationstijd
ais-dgnss = {" "}(DGNSS)
ais-first-heard = eerst gehoord
ais-center = CENTREREN
ais-put-vessel-middle-chart = Zet dit schip in het midden van de kaart
ais-last-message-station-nmea-sentences = Het laatste bericht van dit station, als de NMEA-zinnen die een ontvanger op de lijn zou zetten. Plak het in een andere AIS-decoder om te controleren wat sdroxide ervan maakte.
ais-ais-setup = AIS-instellingen
ais-channels = Kanalen
ais-drop-chart-after = Van de kaart halen na
ais-without-position-report = zonder positiemelding
ais-five-minutes-default-not-ads = Standaard vijf minuten, niet de tien seconden van ADS-B: een schip voor anker meldt eens per drie minuten, en een korter venster zou het grootste deel van een haven leegmaken tussen twee volkomen goede meldingen. Daarna gaat het schip van de kaart en wordt zijn rij grijs — het wordt niet vervaagd, omdat een dof symbool op een verouderde positie nog steeds een bewering over waar een schip is, in dezelfde inkt als de ware.
ais-drop-list-after = Uit de lijst halen na
ais-nothing-heard-all = zonder iets gehoord te hebben
ais-message-carrying-ship-s-name = Het bericht met de naam van een schip komt elke zes minuten rond, dus een kort lijstvenster blijft schepen weggooien vlak voordat ze zeggen hoe ze heten.
ais-trail-length = Sleeplengte
ais-history-behind-each-target = geschiedenis achter elk doel
ais-minutes-rather-than-points-because = In minuten in plaats van in punten, omdat AIS-meldfrequenties twee ordes van grootte beslaan: een vast aantal zou tachtig seconden van een veerboot en twee uur van een voor anker liggende tanker zijn, identiek getekend. Nul schakelt sleep uit.
ais-speed-vector = Snelheidsvector
ais-how-far-ahead-vector-reaches = hoe ver de vector vooruit reikt
ais-slot-threshold = Slotdrempel
ais-above-channel-s-learned-noise = boven de geleerde ruisvloer van het kanaal
ais-track-most = Maximaal volgen
ais-vessels-2 = {" "}schepen
ais-ais-receive-only-here-always = AIS is hier alleen-ontvangst en zal dat altijd blijven: het is een dienst voor veiligheid van mensenlevens, en er vals scheepsverkeer op zetten valt niet onder een licentie.
ais-fill-my-grid-digimode-setup = Vul Mijn grid in bij de digimode-instellingen voor afstanden en peilingen, en zodat de kaart zichzelf rond jouw positie kadreert in plaats van rond wat het verst weg is.
ais-name = NAAM
ais-mmsi = MMSI
ais-type = TYPE

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = Deze ontvanger kan 10489,750 MHz niet zelf bereiken — stel eerst een LNB-/converteroffset in (Instellingen ▸ Radio)
qo100-run-fast-spectral-tracker-every = Draai de snelle spectrale tracker: zoek elke seconde in het parkeervenster naar de twee symmetrische lobben van het baken en meld waar de draaggolf zit
qo100-telemetry = TELEMETRIE
qo100-also-run-ao-40-uncoded = Draai ook de ongecodeerde AO-40-framedecoder — syncwoord, CRC en de telemetrietekst — met een stapsgewijze weergave (draaggolf → sync → CRC) van hoe ver elke doorgang komt
qo100-auto = AUTO
qo100-auto-correct-let-tracker-adjust = Automatisch corrigeren: laat de tracker de converter-/LNB-offset zelf bijstellen — een trage gesloten lus die, bij een schone en stabiele schatting, het baken terug op 10489,750 MHz duwt en daar houdt terwijl de LNB driftt. Heropent de front-end telkens als hij ingrijpt.
qo100-narrower-search-smaller-slice = Smaller — zoek in een kleiner deel
qo100-wider-when-beacon-isn-t = Breder — voor als het baken bij de huidige breedte niet wordt gevonden
qo100-clear-mark = markering wissen
qo100-drop-hand-placed-beacon-mark = Laat de met de hand geplaatste bakenmarkering vallen en ga terug naar de eigen meting van de decoder
qo100-lo = lo −
qo100-lo-2 = lo +
qo100-hi = hi −
qo100-hi-2 = hi +
qo100-radio-s-own-tuning-range = Het eigen afstembereik van deze radio bereikt 10489,750 MHz niet — hij heeft een LNB-/converteroffset nodig (Instellingen ▸ Radio ▸ Converter) voordat dit venster iets kan doen.
qo100-open-settings-radio = Open Instellingen ▸ Radio
qo100-tune-10489-750-mhz = Afstemmen op 10489,750 MHz
qo100-tip-if-you-can-see = tip: als je het baken in de strook ziet maar het niet vergrendelt, dubbelklik erop — dat markeert het als 10489,750 MHz en laat TOEPASSEN erop corrigeren
qo100-tracker = TRACKER
qo100-no-twin-lobe-shape-parking = geen tweelobbige vorm in het parkeervenster
qo100-shape = VORM
qo100-drift = DRIFT
qo100-need-longer-run-estimates = — (langere reeks schattingen nodig)
qo100-cycles = CYCLI
qo100-receiver = ONTVANGER
qo100-target = DOEL
qo100-measured = GEMETEN
qo100-clicked = {"  "}(aangeklikt)
qo100-tracker-2 = {"  "}(tracker)
qo100-not-locked-yet = nog niet vergrendeld
qo100-converter-offset = CONVERTEROFFSET
qo100-decode = DECODEREN
qo100-decode-filling-first-frame-buffer = DECODEREN — eerste framebuffer vullen…
qo100-decode-off-turn-telemetry-chip = DECODEREN — uit. Zet de TELEMETRIE-chip aan (bovenste rij) voor de weergave draaggolf → sync → CRC.
qo100-apply-correction = {" "}CORRECTIE TOEPASSEN{" "}
qo100-write-converter-lnb-offset-puts = Schrijf de converter-/LNB-offset die het baken waar je klikte op 10489,750 MHz zet, en heropen de ontvanger — dezelfde korte onderbreking als Instellingen ▸ Radio ▸ Toepassen veroorzaakt
qo100-write-converter-lnb-offset-puts-2 = Schrijf de converter-/LNB-offset die de schatting van de tracker op 10489,750 MHz zet, en heropen de ontvanger — dezelfde korte onderbreking als Instellingen ▸ Radio ▸ Toepassen veroorzaakt
qo100-waiting-second-crc-valid-frame = Wachten op een tweede frame met geldige CRC voordat dit wordt aangeboden — één vergrendeling alleen kan toeval zijn
qo100-write-corrected-converter-lnb-offset = Schrijf de gecorrigeerde converter-/LNB-offset en heropen de ontvanger — een korte onderbreking, dezelfde als Instellingen ▸ Radio ▸ Toepassen veroorzaakt
qo100-decoder-runs-receiving-station-readout = de decoder draait op het ontvangende station — de weergave wordt niet naar externe clients gestuurd
qo100-starting = starten…
qo100-searching-first-window-fills-after = zoeken — het eerste venster vult zich na ongeveer 24 s en herhaalt dan

## js8 (src/app/panels/js8.rs)

js8-multi = MULTI
js8-decode-every-js8-speed-not = Decodeer elke JS8-snelheid, niet alleen die waarmee je zendt. De vier snelheden delen het subband en zijn vier verschillende golfvormen, dus zonder dit is een station op een andere snelheid onzichtbaar — en een uitwisseling tussen twee snelheden kan helemaal niet plaatsvinden. Kost ongeveer vier keer de ontvangst-CPU.
js8-hb-auto = HB AUTO
js8-turbo-does-not-beacon-local = Turbo baken niet — het is de lokale en VHF-snelheid
js8-beacon-your-callsign-grid-every = Baken je roepletters en grid elke 15 minuten
js8-until-next-heartbeat = Tot de volgende heartbeat
js8-tx = ● TX
js8-heard = GEHOORD
js8-nothing-heard-yet = — nog niets gehoord —
js8-dxcc = DXCC
js8-band = BAND
js8-grid = GRID
js8-dupe = DUBBEL
js8-reply = ANTWOORD
js8-no-messages = — geen berichten —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}HB{" "}
js8-clear = {" "}WISSEN AAN{" "}
js8-forget-selected-station-composer-goes = Vergeet het geselecteerde station — de editor gaat terug naar @ALLCALL
js8-already-addressing-allcall = Spreekt al @ALLCALL aan
js8-stop = {" "}STOP{" "}
js8-send = {" "}ZENDEN{" "}
js8-message = Bericht…

## logbook (src/app/logbook.rs)

logbook-logbook = LOGBOEK
logbook-new-entry = + NIEUWE INVOER
logbook-adif = ADIF
logbook-import = IMPORTEREN
logbook-import-qsos-adif-adi-adif = QSO's importeren uit een ADIF-bestand (.adi of .adif)
logbook-new-qso = NIEUWE QSO
logbook-edit-qso = QSO BEWERKEN
logbook-call = Roepletters
logbook-lookup = OPZOEKEN
logbook-look-up-name-qth-grid = Naam / QTH / grid opzoeken
logbook-grid = Grid
logbook-freq-mhz = Freq MHz
logbook-mode = Modus
logbook-rst-sent = RST verzonden
logbook-rst-rcvd = RST ontvangen
logbook-name = Naam
logbook-state = Staat
logbook-country = Land
logbook-date-utc = Datum UTC
logbook-time = Tijd
logbook-pwr-w = Verm. W
logbook-contest = Contest
logbook-s-sent = S# verzonden
logbook-s-rcvd = S# ontvangen
logbook-comment = Opmerking
logbook-save = {" "}OPSLAAN{" "}
logbook-cancel = ANNULEREN
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = kaart ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = Deze invoer verwijderen
logbook-edit = BEWERKEN
logbook-upload-qso-configured-logs = Upload deze QSO naar de geconfigureerde logs
logbook-no-qsos-yet-run-ft8 = nog geen QSO's — draai FT8/FT4 of voeg handmatig een invoer toe

## scanner (src/app/scanner.rs)

scanner-scanner = Scanner
scanner-work-through-stored-memory-channels = Doorloop de opgeslagen geheugenkanalen
scanner-work-through-slice-band-channel = Doorloop een deel van een band op een kanaalraster
scanner-stop = STOP
scanner-start = START
scanner-from = Van
scanner-step = Stap
scanner-mode = Modus
scanner-range-empty-high-edge-has = Dat bereik is leeg — de bovengrens moet minstens één kanaal boven de ondergrens liggen.
scanner-skip-while-holding-pass-over = SKIP terwijl hij blijft staan om dat kanaal voortaan over te slaan
scanner-skipping = Overgeslagen
scanner-clear = WISSEN
scanner-stop-passing-over-any-them = Stop met het overslaan van alle kanalen
scanner-stop-passing-over-channel = Stop met het overslaan van dit kanaal
scanner-no-memory-channels-stored-yet = Nog geen geheugenkanalen opgeslagen — sla er enkele op in het MEM-venster.
scanner-skip-channel-pass-over = SKIP een kanaal om het over te slaan
scanner-skip = SKIP
scanner-look-all-channels-fall-inside = Zoek alle kanalen die binnen één ontvangervenster vallen op dezelfde transformatie waarmee de panadapter wordt opgebouwd, en stem alleen af op de kanalen waarop iets te horen is. Een lijst op één band kost zo één afstemming per ronde, hoe lang ook, in plaats van een insteltijd per kanaal. De scan luistert nog steeds op elke kandidaat voordat hij stopt, dus wat hem stopt blijft gelijk — maar de sweep meet via de FFT in plaats van via het filter van de ontvanger, dus controleer de drempel als hij op niets begint te stoppen.
scanner-radio-hands-over-demodulated-audio = Deze radio levert gedemoduleerde audio en heeft geen eigen spectrum om te doorzoeken, dus de geheugenscan bezoekt hoe dan ook elk kanaal.
scanner-fast = SNEL
scanner-read-list-off-spectrum-instead = lees de lijst van het spectrum af in plaats van elk kanaal te bezoeken
scanner-folders = Mappen
scanner-scan-every-folder-every-folder = Scan elke map, en elke map die vanaf nu wordt gemaakt. Kies mappen om alleen die te scannen.
scanner-unfiled = Niet ingedeeld
scanner-nothing-filed-under-folders-you = Er is niets ingedeeld onder de mappen die je hebt gekozen.
scanner-stops = Stopt bij
scanner-use-receiver-s-own-squelch = Gebruik de eigen squelch van de ontvanger, zodat de scan precies stopt waar de audio zou openen
scanner-squelch-off-scan-will-stop = squelch is uit — de scan stopt bij het eerste wat hij tegenkomt
scanner-channel-power-signal-has-reach = Kanaalvermogen dat een signaal moet halen om de scan te stoppen
scanner-listens = Luistert
scanner-how-long-stay-candidate-before = Hoe lang op een kandidaat te blijven voordat hij wordt beoordeeld. Onder ongeveer een tiende seconde is de niveaumeter nog niet gestabiliseerd en worden zwakke signalen gemist
scanner-resumes = Hervat
scanner-carry-once-signal-drops = Ga door zodra het signaal wegvalt
scanner-carry-after-fixed-time-regardless = Ga door na een vaste tijd, wat er ook gebeurt
scanner-stay-until-you-press-next = Blijf staan tot je op VOLGENDE drukt
scanner-how-long-stay = Hoe lang blijven
scanner-how-long-wait-after-signal = Hoe lang wachten nadat het signaal wegvalt
scanner-move-don-t-stop-channel = Ga verder en stop niet meer op dit kanaal
scanner-next = VOLGENDE
scanner-move-now = Ga nu verder

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = de ADS-B-decoder wordt gestart…
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = 1090 MHz Mode S
adsb-tune-receiver-1090-000-mhz = Stem de ontvanger af op 1090.000 MHz, de wereldwijde ADS-B-downlink. Er is maar één kanaal.
adsb-setup = INSTELLEN
adsb-timeouts-trail-length-how-far = Time-outs, sporenlengte en hoe ver de snelheidsvectoren vooruit reiken
adsb-aircraft = VLIEGTUIGEN
adsb-nothing-heard-yet = nog niets ontvangen
adsb-ground = op de grond
adsb-position-stale = positie verouderd
adsb-gnss-altitude = GNSS-hoogte
adsb-ground-speed = grondsnelheid
adsb-vertical-rate = verticale snelheid
adsb-first-heard = eerst ontvangen
adsb-center = CENTREREN
adsb-put-aircraft-middle-map = Zet dit vliegtuig in het midden van de kaart
adsb-last-frame-accepted-aircraft = Het laatste frame dat van dit vliegtuig is geaccepteerd
adsb-ads-b-setup = ADS-B-instellingen
adsb-drop-map-after = Van kaart halen na
adsb-without-position-report = zonder positiemelding
adsb-past-aircraft-comes-off-map = Hierna verdwijnt het vliegtuig van de kaart en wordt de rij grijs. Het wordt niet vervaagd: een gedimd vierkantje op een verouderde positie blijft een bewering over waar een vliegtuig is, in dezelfde inkt als de juiste.
adsb-drop-list-after = Van lijst halen na
adsb-nothing-heard-all = zonder dat er iets is ontvangen
adsb-trail-length = Sporenlengte
adsb-points = {" "}punten
adsb-history-dots-behind-each-target = historiepunten achter elk doel
adsb-speed-vector = Snelheidsvector
adsb-how-far-ahead-leader-line = hoe ver de leiderlijn vooruit reikt
adsb-one-minute-usual-radar-convention = Eén minuut is de gebruikelijke radarconventie: de lijn is precies zo lang als de afstand die het vliegtuig in die tijd aflegt, dus twee even lange lijnen zijn twee gelijke snelheden bij elke zoom. Nul schakelt ze uit.
adsb-track-most = Volg maximaal
adsb-aircraft-2 = {" "}vliegtuigen
adsb-aircraft-ground-are-placed-against = Vliegtuigen op de grond worden geplaatst ten opzichte van de eigen positie van het station — een surface squitter heeft geen eenduidige eigen decodering — vul dus Mijn grid in bij de digimode-instellingen als het vliegveld in de buurt niets toont.
adsb-call = CALL
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = wachten op de ontvanger…
drm-not-drm-set-mode-drm = Niet in DRM. Zet de modus op DRM bij een digitale kortegolfomroep — de afstemming komt op het kanaalmidden, niet op een zijband.
drm-no-drm-signal-locked-decoder = Geen DRM-signaal vergrendeld. De decoder heeft een paar seconden nodig op een schone draaggolf: DRM verweeft over 400 ms of 2 s voordat er iets van te lezen is.
drm-station-s-audio-codec-not = De audiocodec van dit station wordt hier niet gedecodeerd. xHE-AAC heeft libfdk-aac op het systeem nodig — het kan niet worden ingebouwd, omdat de licentie en die van dit programma onverenigbaar zijn. Installeer het (Debian/Ubuntu: libfdk-aac2, Arch: libfdk-aac, macOS: brew install fdk-aac, Windows: libfdk-aac-2.dll naast sdroxide.exe) en start opnieuw. Al het andere op dit scherm wordt normaal gedecodeerd.
drm-quality = KWALITEIT
drm-last-minute = laatste minuut
drm-waiting-locked-signal = wachten op een vergrendeld signaal
drm-audio-frames-decoding-but-station = Audioframes worden gedecodeerd — maar de codec van dit station kan hier niet worden gedecodeerd
drm-audio-frames-decoding = Audioframes worden gedecodeerd
drm-samples-reaching-decoder = Samples die de decoder bereiken
drm-time = TIJD
drm-symbol-timing-recovered = Symboolsynchronisatie hersteld
drm-frame = FRAME
drm-transmission-frames-found = Transmissieframes gevonden
drm-fast-access-channel-what-transmission = Fast Access Channel — wat de transmissie is
drm-service-description-channel-what-services = Service Description Channel — wat de diensten zijn
drm-audio = AUDIO
drm-constellation = CONSTELLATIE
drm-waiting-decoded-symbols = wachten op gedecodeerde symbolen
drm-sampled-across-frame = bemonsterd over het frame
drm-whole-frame = heel frame
drm-mode = MODUS
drm-robustness-mode-channel-width-ground = Robuustheidsmodus en kanaalbreedte. A is voor een grondgolfpad en draagt het meest; D is voor een sterk verstrooid hemelgolfpad en draagt het minst.
drm-interleave = VERWEVING
drm-how-far-transmission-spreads-each = Hoe ver de transmissie zich per frame in de tijd uitspreidt. Lang doorstaat diepere fading en duurt navenant langer om te verwerven.
drm-protection = BESCHERMING
drm-offset = OFFSET
drm-residual-sample-clock-error-against = Resterende sampleklokfout ten opzichte van de zender. Groot en constant betekent dat de referentie van de ontvanger afwijkt, niet de uitzending.
drm-doppler = DOPPLER
drm-delay = VERTRAGING
drm-doppler-delay-spread-path-how = Doppler- en vertragingsspreiding van het pad — hoe snel het beweegt en hoe ver de echo's uit elkaar aankomen.
drm-service = DIENST
drm-decode-service-multiplex = Decodeer deze dienst van de multiplex

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = het AtCHAT NET-station wordt gestart…
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET — toetsenbord en bestanden voor meerdere stations
atchat-net = {" "}OP NET{" "}
atchat-net-click-leave-state-kept = Op het net — klik om te verlaten (de status blijft bewaard; klik opnieuw om weer deel te nemen)
atchat-rejoin = {" "}OPNIEUW{" "}
atchat-off-net-click-rejoin = Van het net — klik om weer deel te nemen
atchat-off-net = van net
atchat-carrier = ● DRAAGGOLF
atchat-another-station-transmitting-channel-busy = Een ander station zendt — het kanaal is bezet.
atchat-tx = ● TX
atchat-station-callsign-virtual-channel-address = Roepletters van het station en het adres van het virtuele kanaal
atchat-virtual-channel = VIRTUEEL KANAAL
atchat-work-net-over-tcp-endpoint = Werk het net via een TCP-eindpunt in plaats van de radio — een channel_server-compatibele loopback voor ontwikkeling zonder radio. Uit zet het station in de ether.
atchat-roster = LEDENLIJST
atchat-nobody-heard-yet = nog niemand ontvangen
atchat-chat = {" "}CHAT{" "}
atchat-common-channel-lines-here-go = Het gemeenschappelijke kanaal — regels hier gaan naar iedereen (ALL)
atchat-log = {" "}LOG{" "}
atchat-station-s-own-air-activity = De eigen activiteit van het station in de ether — masterverkiezing, veroudering van de ledenlijst, ARQ-herhalingen
atchat-no-activity-logged-yet = Nog geen activiteit vastgelegd.
atchat-no-messages-yet = Nog geen berichten.
atchat-message-everyone-all = bericht aan iedereen (ALL)
atchat-send = {" "}VERZEND{" "}
atchat-transfers = OVERDRACHTEN
atchat-all = → ALL
atchat-sent-file-follows-active-chat = Een verzonden bestand volgt het actieve chattabblad
atchat-send-file = {" "}BESTAND ZENDEN{" "}
atchat-send-file-image-over-air = Verzend een bestand of afbeelding via de ether, blok-CRC-ARQ
atchat-nothing-flight = niets onderweg
atchat-images = AFBEELDINGEN
atchat-no-images-received = Geen afbeeldingen ontvangen.


## rds (src/app/rds.rs)

rds-station = STATION
rds-diagnostics = DIAGNOSE
rds-which-programme-type-table-read = Tegen welke programmatypetabel dit station wordt gelezen. RDS wordt overal buiten Noord-Amerika gebruikt; RBDS voegt de roepletters toe die de identiteitscode van een Amerikaans station uitspelt. Auto volgt de uitgebreide landcode die het station zendt, en zolang er geen binnenkomt, spelt het geen roepletters uit — de identiteitsbereiken die het zou lezen worden gedeeld met de RDS-codes van negen landen, dus kies RBDS om een station te benoemen dat geen landcode zendt.
rds-waiting-receiver = wachten op de ontvanger…
rds-no-rds-station-not-every = Geen RDS op dit station. Niet elke zender voert het, en een zwakke kan het voeren zonder dat het aankomt.
rds-rds-only-carried-wfm-broadcast = RDS wordt alleen gevoerd door WFM-omroepstations.
rds-holding = VASTGEHOUDEN
rds-block-sync-has-dropped-last = Blokssynchronisatie is weggevallen — dit is het laatst gelezen station.
rds-identity = IDENTITEIT
rds-programme = PROGRAMMA
rds-described = OMSCHREVEN ALS
rds-content = INHOUD
rds-music = Muziek
rds-speech = Spraak
rds-traffic = VERKEER
rds-announcement-air-now = omroep in de ether
rds-carried-station = gevoerd door dit station
rds-not-carried = niet gevoerd
rds-radio-text = RADIOTEKST
rds-station-clock = STATIONSKLOK
rds-country-code = LANDCODE
rds-also = OOK OP
rds-also-tagged = OOK GEMARKEERD
rds-sync = SYNC
rds-no-sync = GEEN SYNC
rds-blocks-failed-their-check-plus = Blokken die hun controle niet doorstonden, plus de blokken die de foutcorrector moest herstellen. Correcties tellen bewust als fouten: een stijgend cijfer is het eerste teken dat een station niet meer te decoderen valt.
rds-front-end-overloading = FRONT-END OVERBELAST
rds-converter-clipping-which-puts-distortion = De omzetter clipt, wat vervorming over de hele multiplex veroorzaakt — ook op de 57 kHz-datasubdraaggolf. Zet de RF-versterking lager, of schakel een verzwakker in: RDS gaat ongeveer een decibel eerder kapot dan iets hoorbaar is, dus een station dat perfect klinkt kan nog steeds veel te sterk zijn om zijn data te laten overleven.
rds-group-types = GROEPSTYPEN
rds-recent-groups-newest-last = RECENTE GROEPEN — nieuwste laatst
rds-nothing-decoded-yet = nog niets gedecodeerd

## winlink (src/app/winlink.rs)

winlink-mail = POST
winlink-abort = AFBREKEN
winlink-stop-session-progress = Stop de lopende sessie
winlink-connect = VERBINDEN
winlink-compose = OPSTELLEN
winlink-connecting = verbinden…
winlink-not-connected-yet = nog niet verbonden
winlink-loading = laden…
winlink-date = DATUM
winlink-from = VAN
winlink-subject = ONDERWERP
winlink-no-subject = (geen onderwerp)
winlink-delete-message = Dit bericht verwijderen
winlink-arch = ARCH
winlink-move-archive = Verplaats naar het archief
winlink-select-message = kies een bericht
winlink-reply = ANTWOORD
winlink-delete = VERWIJDER
winlink-to = Aan
winlink-cc = Cc
winlink-subject-2 = Onderwerp
winlink-callsign-smtp-someone-example-org = Een roepletters, of SMTP:someone@example.org voor internetpost. Scheid meerdere met komma's.
winlink-file-outbox = IN UITBAK PLAATSEN
winlink-discard = VERWERPEN
winlink-filed-messages-go-out-next = geplaatste berichten gaan uit bij de volgende sessie
winlink-mail-log = POSTLOG
winlink-session-running = sessie loopt…
winlink-no-session-yet = nog geen sessie

## wspr (src/app/panels/wspr.rs)

wspr-receptions = ONTVANGSTEN
wspr-save-wspr-reception-list-csv = Sla de WSPR-ontvangstlijst op als CSV
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = propagatiebaken voor zwakke signalen
wspr-time-left-two-minute-slot = Resterende tijd in dit slot van twee minuten. Aan het einde gaat de opname naar de decoder en volgt de volgende zendbeslissing.
wspr-tx = ● TX
wspr-decoding = decoderen…
wspr-band = Band
wspr-last-slot = Laatste slot
wspr-nothing-heard = niets ontvangen
wspr-receive-only = alleen ontvangen
wspr-next-slot = Volgend slot
wspr-waiting-radio-take-change = wachten tot de radio de wijziging overneemt…
wspr-hop-next = Spring naar volgende
wspr-transmit = ZENDEN
wspr-receive-only-2 = Alleen ontvangen.
wspr-power = VERMOGEN
wspr-what-you-actually-radiate = wat je werkelijk uitstraalt
wspr-goes-out-message-everyone-who = Dit gaat mee in het bericht, en iedereen die je hoort beoordeelt het pad ermee — een optimistisch cijfer hier maakt dus hun metingen net zo fout als de jouwe.
wspr-roam = ZWERVEN
wspr-pick-different-offset-inside-200 = Kies voor elke transmissie een andere offset binnen het venster van 200 Hz. Dit is de afspraak: tweehonderd hertz die door iedereen worden gedeeld werkt alleen als niemand er midden in gaat staan. Uit blijft staan waar de cursor is.
wspr-band-hop = BANDSPRONG
wspr-move-dial-band-band-between = Verplaats de afstemming tussen slots van band naar band, zodat één ontvanger het hele spectrum bemonstert in plaats van één plak ervan. Zelf aan de VFO draaien pauzeert dit — druk hier twee keer op om te hervatten.
wspr-upload = UPLOADEN
wspr-send-what-station-decodes-wsprnet = Stuur wat dit station decodeert naar wsprnet.org. Rapporteren wat je hoort maakt een WSPR-ontvanger onderdeel van het netwerk; het zet niets in de ether.
wspr-who-heard-me = WIE HOORDE MIJ
wspr-ask-wsprnet-org-every-few = Vraag wsprnet.org om de paar minuten welke stations dit station hebben gedecodeerd. WSPR heeft geen enkele bevestiging, dus dit is de enige manier waarop een baken iets te weten komt over zijn eigen bereik.
wspr-bands = BANDEN
wspr-none-selected-hopping-has-nowhere = geen geselecteerd — springen heeft nergens heen te gaan
wspr-set-your-callsign-grid-general = Stel je roepletters en grid in op het tabblad Algemeen van de Instellingen voordat je gaat zenden.

## packet (src/app/panels/packet.rs)

packet-monitor = MONITOR
packet-busy = BEZET
packet-frames-arrived-but-failed-their = Frames die aankwamen maar hun controlesequentie niet doorstonden — een botsing, een fade of een signaal te zwak om te lezen.
packet-station-callsign-speed-tx-delay = Roepletters van het station, snelheid, TX-vertraging, het digipeaterpad, het baken en de KISS-server
packet-starting-packet-modem = de packetmodem wordt gestart…
packet-nothing-heard-yet = nog niets ontvangen
packet-terminal = TERMINAL
packet-connected = {" "}VERBONDEN{" "}
packet-calling = {" "}ROEPEN{" "}
packet-idle = {" "}INACTIEF{" "}
packet-extended-sequence-numbers-window-up = Uitgebreide volgnummers — een venster van maximaal 127 frames.
packet-frames-sent-not-yet-acknowledged = Frames die zijn verzonden en nog niet door de andere kant bevestigd.
packet-retries-against-link-s-limit = Herhalingen ten opzichte van de limiet van deze link. Een oplopend aantal terwijl de onbevestigde frames gelijk blijven is hoe een fadend pad er vanaf deze kant uitziet.
packet-mail-window-using-link = Het POST-venster gebruikt de link.
packet-one-radio-one-channel-one = Eén radio, één kanaal, één link. Een Winlink-sessie en deze terminal zijn twee manieren om hem te gebruiken, en wie het eerst vraagt krijgt hem.
packet-digipeaters-nearest-first-separated-commas = Digipeaters, dichtstbijzijnde eerst, gescheiden door komma's — OE3XLR-1,OE3XMS-1. Laat leeg voor een station dat je direct kunt horen, of om het standaardpad uit de packetinstellingen te gebruiken.
packet-disconnect = {" "}VERBREKEN{" "}
packet-hang-up-properly-wait-far = Netjes ophangen en wachten tot de andere kant akkoord gaat. Van modus wisselen beëindigt de sessie in plaats daarvan zonder het iemand te vertellen.
packet-connect = {" "}VERBINDEN{" "}
packet-mail-window-has-link-finish = Het POST-venster heeft de link. Rond die sessie eerst af of stop haar.
packet-station-has-no-callsign-yet = Dit station heeft nog geen roepletters — stel ze in onder INSTELLEN, boven het monitorvenster. Er wordt niets gezonden tot ze zijn ingesteld.
packet-call-station-connected-mode-node = Roep dit station op in verbonden modus — een node, een BBS of een andere operator.
packet-needs-callsign-call = Roepletters nodig om te roepen.
packet-clear = {" "}WISSEN{" "}
packet-empty-transcript-link-untouched-connected = Maak het transcript leeg. De link blijft ongemoeid — een verbonden station blijft verbonden.
packet-not-connected-type-callsign-above = Niet verbonden. Typ hierboven een roepletters en druk op VERBINDEN om een node of een BBS te roepen.
packet-type-here = typ hier
packet-not-connected = niet verbonden
packet-send = {" "}VERZEND{" "}
packet-send-line-node-bbs-reads = Verstuur de regel. Een node of een BBS leest één regel tegelijk.

## fsq (src/app/panels/fsq.rs)

fsq-speed = Snelheid
fsq-call = Roep
fsq-callsign-directed-fsqcall-messages-defaults = Roepletters voor gerichte (FSQCALL) berichten; standaard je eigen roepletters
fsq-tx = ● TX
fsq-contacts = CONTACTEN
fsq-heard = ONTVANGEN
fsq-none = — geen —
fsq-allcall = ALLCALL
fsq-image = AFBEELDING
fsq-clear = WISSEN
fsq-forget-pictures-received-so-far = Vergeet de tot nu toe ontvangen afbeeldingen
fsq-send-image = Afbeelding verzenden…
fsq-right-click-remove = Rechtsklik om te verwijderen
fsq-remove-picture = Verwijder deze afbeelding
fsq-listening = — luistert —
fsq-message = Bericht…
fsq-send = {" "}VERZEND{" "}
fsq-call-cq = {" "}CQ ROEPEN{" "}
fsq-heard-2 = {" "}? ontvangen{" "}
fsq-clear-2 = {" "}WISSEN{" "}
fsq-fsq-contacts = FSQ-contacten
fsq-add = Toevoegen

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + TAB
publicsdr-open-receiver-new-radio-own = Open deze ontvanger als nieuwe radio, in een eigen tabblad
publicsdr-point-radio-receiver-replacing-whatever = Richt *deze* radio op de ontvanger, ter vervanging van de interface waarop hij nu staat — er wordt eerst nogmaals om bevestiging gevraagd, omdat dat het instellen van een hele radio is
publicsdr-not-stated = niet opgegeven
publicsdr-replace = VERVANGEN
publicsdr-point-radio-you-are-receiver = Richt de radio waarop je zit op deze ontvanger
publicsdr-tab-instead = + TAB IN PLAATS DAARVAN
publicsdr-open-receiver-another-radio-leave = Open de ontvanger als een andere radio en laat deze met rust
publicsdr-cancel = ANNULEREN
publicsdr-public-sdrs = PUBLIEKE SDR'S
publicsdr-available = BESCHIKBAAR
publicsdr-hide-receivers-are-full-ones = Verberg ontvangers die vol zijn, en die waarvan de beheerder geen kanalen heeft geopend voor andere apps dan een browser
publicsdr-band = IN BAND
publicsdr-only-receivers-cover-current-dial = Alleen ontvangers die de huidige afstemfrequentie dekken
publicsdr-low-bw = LAGE BW
publicsdr-take-spyserver-low-bandwidth-shape = Neem een SpyServer in zijn lage-bandbreedtevorm: een smal I/Q-venster dat de afstemming volgt plus het eigen bandoverzicht van de server, in plaats van megabits aan breedband-I/Q. Geen effect op een KiwiSDR, die maar één vorm heeft.
publicsdr-refresh = ⟳ VERVERSEN
publicsdr-fetch-both-directories-again = Haal beide mappen opnieuw op
publicsdr-name-place-antenna-band = naam, plaats, antenne, band
publicsdr-clear-search = Wis de zoekopdracht
publicsdr-fetching-receiver-lists = De ontvangerlijsten worden opgehaald…
publicsdr-machine-radio-attached-does-not = De machine waaraan deze radio hangt beantwoordt geen apparaatvragen, dus hij kan de lijsten ook niet ophalen.
publicsdr-no-match = geen overeenkomst
publicsdr-nothing-matches-try-turning-filter = niets komt overeen — probeer een filterchip weer aan te zetten
publicsdr-radio = deze radio
publicsdr-radio-s-configuration-has-not = De configuratie van deze radio is nog niet aangekomen van de machine waarop hij staat, dus er is niets om op de ontvanger te richten — neem hem in plaats daarvan met + TAB.
publicsdr-position-not-published-so-nothing = {" "}Zijn positie is niet gepubliceerd, dus niets wat hij hoort wordt gerapporteerd.

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = wachten op de ontvanger…
hd-not-hd-radio-set-mode = Niet in HD Radio. Zet de modus op HD Radio bij een FM-omroep met de digitale zijbanden — de afstemming komt op het midden van de analoge draaggolf, niet ernaast.
hd-no-hd-radio-lock-digital = Geen HD Radio-vergrendeling. De digitale zijbanden worden ongeveer 20 dB onder de analoge draaggolf uitgezonden, dus een goed luisterbaar station kan nog te ruisig zijn om te decoderen: de decoder heeft een paar seconden nodig op een schoon station.
hd-sync = SYNC
hd-ofdm-frame-timing-recovered = OFDM-frametiming hersteld
hd-audio = AUDIO
hd-audio-frames-decoding = Audioframes worden gedecodeerd
hd-quality = KWALITEIT
hd-lower = ONDER
hd-upper = BOVEN
hd-last-minute = laatste minuut
hd-waiting-locked-signal = wachten op een vergrendeld signaal
hd-mer-l = MER O
hd-modulation-error-ratio-lower-digital = Modulatiefoutverhouding van de onderste digitale zijband.
hd-mer-u = MER B
hd-modulation-error-ratio-upper-digital = Modulatiefoutverhouding van de bovenste digitale zijband.
hd-cber = CBER
hd-channel-bit-error-ratio-after = Kanaalbitfoutverhouding na de binnenste code. Onder ongeveer 1e-3 kan de buitenste code het corrigeren; daarboven begint de audio te haperen.
hd-offset = OFFSET
hd-residual-carrier-frequency-offset-large = Resterende draaggolffrequentie-offset. Groot en constant betekent dat de referentie van de ontvanger afwijkt, niet de uitzending.
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = Primary Service Mode Indicator: welke van de FM-hybride servicemodi het station uitzendt.
hd-programme = PROGRAMMA
hd-restricted-access = {" "}— beperkte toegang

## bands (src/app/bands.rs)

bands-bands = BANDEN
bands-daylight = ☀ DAGLICHT
bands-night = ☾ NACHT
bands-no-forecast-yet-fetching-hamqsl = · nog geen voorspelling — wordt opgehaald van HAMQSL.com
bands-band = BAND
bands-condx = CONDX
bands-paths = PADEN
bands-reach = BEREIK
bands-best = BESTE
bands-ibp-beacons = IBP-BAKENS
bands-meteor-showers = METEORENZWERMEN
bands-peak = PIEK

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = SPOTS
spots-longwave-shortwave-broadcast-stations-air = Lange- en kortegolf-omroepstations nu in de ether
spots-view = IN BEELD
spots-only-spots-inside-panadapter-span = Alleen spots binnen de panadapterspan
spots-setup = ⚙ INSTELLEN
spots-feeds-lookup-upload-settings = Feeds, opzoek- en uploadinstellingen
spots-call-station-site-frequency = roepletters, station, locatie, frequentie
spots-clear-search = Wis de zoekopdracht
spots-no-match = geen overeenkomst
spots-no-spots-enable-feed-setup = geen spots — schakel een feed in via ⚙ INSTELLEN
spots-nothing-matches-search = niets komt overeen met de zoekopdracht

## ism (src/app/ism.rs)

ism-ism-devices = ISM-APPARATEN
ism-decoding = DECODEREN
ism-needs-wideband-iq-source-cat = Heeft een breedband-IQ-bron nodig; een CAT-rig stuurt alleen audio
ism-how-far-burst-must-stand = Hoe ver een burst boven de eigen ruisvloer van het kanaal moet uitsteken voordat hij wordt gedecodeerd
ism-give-each-band-width-normally = Geef elke band de breedte die hij normaal nodig heeft — een kwart megahertz voor de OOK-banden op 315, 345 en 433 MHz, een volle voor 868 en 915.
ism-several-hundred-more-device-decoders = Enkele honderden extra apparaatdecoders, uit het rtl_433-project, ingebouwd. Leest de 433 MHz OOK-apparaten die de eigen decoders van sdroxide helemaal niet bereiken. Waar beide een apparaat kunnen lezen, kent rtl_433 veel meer varianten en neemt het over.
ism-t-1-your-own = 1 van jezelf
ism-device-decoders-registered-how-many = Geregistreerde apparaatdecoders, hoeveel er uit je rtl433_flex.conf komen, en hoeveel transmissies er zijn gedecodeerd sinds de start.
ism-reload-decoders = DECODERS HERLADEN
ism-re-read-rtl433-flex-conf = Lees rtl433_flex.conf opnieuw na het bewerken. De al ontvangen apparaten blijven in de lijst.
ism-rtl-433-knows-more-variants = rtl_433 kent meer varianten hiervan dan de eigen decoders van sdroxide, dus het leest ze zolang het hier luistert. Kies een andere band en die van sdroxide nemen het weer over.
ism-decoder-your-rtl433-flex-conf = Een decoder in je rtl433_flex.conf is geweigerd. De andere zijn wel geladen. Herstel de regel en druk dan op DECODERS HERLADEN.
ism-waiting-decoder = wachten op de decoder…
ism-where-decoder-actually-listening-follows = Waar de decoder daadwerkelijk luistert. Dit volgt het hardwaremidden van de ontvanger, niet de VFO — bij een brede front-end kunnen die twee ver uit elkaar liggen, en het is het venster dat bepaalt welke kanalen bereikbaar zijn.
ism-transmissions-gate-opened-how-many = Transmissies waarop de poort opende, en hoeveel een geldig frame opleverden. Veel bursts en geen decodes betekent dat de band druk is met apparaten die sdroxide nog niet kan lezen. Helemaal geen betekent dat niets de drempel haalt — zet hem lager, of controleer of de afstemming op de band staat.
ism-nothing-heard-yet = nog niets ontvangen
ism-click-tune-receiver =


    Klik om de ontvanger erop af te stemmen

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

navtex-messages = BERICHTEN
navtex-sync = SYNC
navtex-character-phase-locked-signal-being = De tekenfase is vergrendeld — een signaal wordt gelezen.
navtex-characters-whose-first-copy-was = Tekens waarvan de eerste kopie beschadigd was en die uit de herhaling vijf slots later zijn genomen, en die waarbij beide slecht waren. Een NAVTEX-teken wordt twee keer verzonden; dat is de hele foutcorrectie.
navtex-swap-mark-space-tones-signal = Wissel de mark- en spacetonen om, voor een signaal dat op de andere zijband is ontvangen. Uit is bovenzijband op de kanaalfrequentie, wat elke gepubliceerde afstemaanwijzing voor de dienst zegt.
navtex-starting-navtex-receiver = de NAVTEX-ontvanger wordt gestart…
navtex-nothing-yet-station-transmits-ten = nog niets — een station zendt elke vier uur tien minuten
navtex-cut-short = afgebroken
navtex-message = BERICHT
navtex-time-day-stated-message-utc = Tijdstip van de dag volgens het bericht (UTC): alleen ter weergave
navtex-receiving = ONTVANGEN
navtex-monitor = MONITOR
navtex-everything-decoded-message-not = alles wat is gedecodeerd, bericht of niet

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = RF PAINT
rfpaint-scan-speed = Scansnelheid
rfpaint-how-fast-text-image-scanned = Hoe snel de tekst/afbeelding op de waterval wordt gescand. Lager is langzamer en leesbaarder; 100% = basissnelheid, 25% (midden) is de standaard.
rfpaint-abort = Afbreken
rfpaint-text-paint = TEKST PAINT
rfpaint-type-text-paint = Typ tekst om te schilderen…
rfpaint-preview-waterfall = WATERVAL VOORBEELD
rfpaint-type-text-preview = typ tekst voor een voorbeeld
rfpaint-transmit = {"  "}ZENDEN{"  "}
rfpaint-image-paint = AFBEELDING PAINT
rfpaint-no-image-loaded = geen afbeelding geladen
rfpaint-load-image = Afbeelding laden…
rfpaint-load-image-preview = laad een afbeelding voor een voorbeeld

## hfdl (src/app/hfdl.rs)

hfdl-listen = LUISTER
hfdl-switches-engine-s-hfdl-downconverter = Schakelt de HFDL-downconverter en -decoder van de engine in of uit. Decoderen kost een lane van 24 kHz en een workerthread, of het venster nu open is of niet.
hfdl-channel-assigned-frequency-listen-anything = Het kanaal (de toegewezen frequentie) om op te luisteren. Alles in de band werkt — HFDL-stations zenden op 2,8 tot 22 MHz — tegen een vaste lane van 24 kHz gecentreerd hier.
hfdl-save-hfdl-decode-log-file = Sla het HFDL-decodelogboek op in een bestand
hfdl-riverhead-northern-atlantic-north-america = Riverhead (noordelijke Atlantische Oceaan/Noord-Amerika) — de standaard, en het kanaal waarop de decoder is gevalideerd.
hfdl-hfdl-assigned-frequency = Een toegewezen HFDL-frequentie.
hfdl-one-your-saved-channels-click = Een van je opgeslagen kanalen — klik om af te stemmen, rechtsklik om te verwijderen.
hfdl-hfdl-selected-but-decoding-off = HFDL is geselecteerd maar decoderen staat uit — druk hierboven op LUISTER
hfdl-decodes = DECODES
hfdl-nothing-decoded-yet-ground-station = Nog niets gedecodeerd — de squitter van een grondstation herhaalt zich elke ~32 s zodra er een in het kanaal zit.
hfdl-no-decode-matches-filter = Geen decode komt overeen met het filter.
hfdl-decoding = DECODEREN
hfdl-decoding-off = DECODEREN UIT
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = FreeDV V1 digitale spraak
rade-tx = ● TX
rade-mute-analog = ANALOOG DEMPEN
rade-mute-demodulated-audio-so-only = Demp de gedemoduleerde audio, zodat alleen gedecodeerde spraak te horen is. Het ruwe signaal wordt anders doorgelaten zodra de modem niets te spelen heeft — die ruis is hoe je een over vindt voordat hij synchroniseert, dus laat dit uit tijdens het afstemmen.
rade-sync = SYNC
rade-how-far-received-signal-sits = Hoe ver het ontvangen signaal zit van waar de modem het verwacht. Grote waarden worden nog steeds gedecodeerd — de acquisitielus volgt ze — maar de afstemming bijsturen tot dit bijna nul is geeft de beste marge.
rade-snr = SNR —
rade-callsign-station-put-end-over =
    De roepletters die het station in zijn End-of-Over-frame heeft gezet — het enige punt in een RADE-over waarop er een wordt verzonden. Het is ook wat aan FreeDV Reporter wordt gerapporteerd, zodat een station dat je hoort op qso.freedv.org verschijnt als door jou gehoord.

    Een minuut vastgehouden nadat het signaal wegvalt, en vergeten zodra je een eigen over begint: het noemt het station op de frequentie, niet elk station dat de ontvanger ooit heeft gehoord.
rade-stop-talking = STOP MET PRATEN
rade-talk-ptt = PRAAT (PTT)
rade-open-close-rade-over-modem = Open of sluit een RADE-over. De modem heeft ~120 ms spraak nodig voordat het eerste frame uitgaat, en verstuurt een end-of-over-frame als je stopt, dus het zenden loopt iets door na de knop.
rade-rade-v1-occupies-roughly-1060 = RADE V1 beslaat ongeveer 1060–1880 Hz van de doorlaatband — bovenzijband, of onderzijband op 160, 80 en 40 m, waar RADE de telefoniepraktijk volgt en sdroxide voor je omschakelt. Zet het signaal binnen de gearceerde band op de waterval; de modem vindt het vandaar.

## frame (src/app/frame.rs)

frame-acknowledge = Bevestigen
frame-dismiss = Sluiten
frame-reconnecting = Opnieuw verbinden…
frame-not-connected = Niet verbonden.
frame-reconnect-now = Nu opnieuw verbinden
frame-transmit-lockout-disabled = ⚠ ZENDVERGRENDELING UITGESCHAKELD
frame-engine-was-started-oob-tx = Deze engine is gestart met --oob-tx. De amateurbandvergrendeling staat uit: hij zal de zender activeren op elke frequentie die de hardware ondersteunt.
frame-transmitting-outside-your-licence-offence = Zenden buiten je vergunning is een overtreding in elk land dat er een uitgeeft. Ga alleen verder als je bevoegd bent de frequenties te gebruiken waarop je gaat zenden — een MARS/CAP- of commerciële vergunning, een experimentele vergunning, of een dummy load.
frame-i-understand = {"  "}IK BEGRIJP HET{"  "}
frame-restart-without-oob-tx-put = Start opnieuw zonder --oob-tx om de vergrendeling terug te zetten.

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC — hoe hard de zender wordt aangestuurd, als percentage van het maximum dat hij kan leveren. 100 % is volle schaal, niet vol vermogen in watt: wat dat aan de antenne waard is hangt af van de radio en van Drive.
smeter-radio-reports-no-forward-power =


    Deze radio meldt geen voorwaarts vermogen, dus het cijfer naast ALC is opnieuw de ALC-meting in plaats van watt.
smeter-no-swr-radio-has-no =


    Geen SWR: deze radio heeft geen brug om het te meten, en sdroxide verzint er geen — het kan alleen tonen wat de radio zelf meldt. Een rig met een SWR-meter (via CAT, TCI of een LAN-link) en een HPSDR-bord vullen dit allebei in; een gewone SDR-zender kan dat niet.
smeter-no-signal-report-yet-receiver = Nog geen signaalrapport — de ontvanger heeft nog geen meterwaarde geleverd.
smeter-s9-73-dbm-each-s =


    S9 is −73 dBm en elke S-eenheid daaronder is 6 dB. Het dBm-cijfer is slechts zo eerlijk als de kalibratie van de front-end — een ongekalibreerde ontvanger meldt dBFS met een dBm-label.
smeter-ovld-converter-into-rails-so =


    OVLD: de omzetter zit tegen zijn grenzen aan, dus het getoonde niveau onderschat het signaal. Draai de front-end-versterking terug.
smeter-table-held = , tabel vastgehouden
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● TX
multi-link = Power
multi-mute-radio-s-audio = Demp de audio van deze radio
multi-close-radio-s-split-view = Sluit de gesplitste weergave van deze radio
multi-open-radio-split-view-own = Open deze radio in een eigen gesplitste weergave
multi-already-open-another-split-view = Al geopend in een andere gesplitste weergave
multi-show-radio-here = Toon deze radio hier
multi-add-radio-computer-settings-radio = Voeg een radio toe op deze computer (Instellingen → Radio om er een bij een station toe te voegen)
multi-add-radio = Radio toevoegen
multi-asked-station-another-radio = Het station om nog een radio gevraagd…
multi-client-cannot-open-connection = Deze client kan geen verbinding openen.

## net (src/app/net.rs)

net-fail = MISLUKT
net-downloading = Downloaden…
net-adif-import-failed-file-could = ADIF-import mislukt: het bestand kon niet worden verwerkt
net-channel-import-failed-file-could = Kanaalimport mislukt: het bestand kon niet worden gelezen
net-channel-import-nothing-read-chirp = Kanaalimport: niets om te lezen — een CHIRP CSV-bestand begint met een koprij die de kolommen benoemt, waarvan er één Frequency moet zijn

## widgets (src/app/panels/widgets.rs)

widgets-free-text = vrije tekst
widgets-hashed-callsign-not-yet-heard = gehashte roepletters, nog niet volledig ontvangen
widgets-entity-unknown = entiteit onbekend
widgets-band = deze band
widgets-new-entity-never-worked-any = Nieuwe entiteit — nooit gewerkt, op geen enkele band
widgets-new-grid-square = Nieuw gridvierkant
widgets-not-worked-before = Nog niet eerder gewerkt
widgets-worked-before-but-not-band = Eerder gewerkt, maar niet op deze band
widgets-call-queue = In de oproepwachtrij
widgets-image = Afbeelding

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = ONTVANGSTEN
pi4-save-pi4-reception-list-csv = Sla de PI4-ontvangstlijst op als CSV
pi4-next-generation-beacon = Next Generation Beacon
pi4-time-left-one-minute-beacon = Resterende tijd in deze bakencyclus van een minuut. Het PI4-bericht wordt gezocht zodra het venster tijd heeft gehad om voorbij het verwachte einde te vullen.
pi4-decoding = decoderen…
pi4-band = Band
pi4-last-cycle = Laatste cyclus
pi4-nothing-heard = niets ontvangen
pi4-receive-only-tune-so-beacon = Alleen ontvangen. Stem af zodat de CW-identificatie van het baken op 800 Hz audio zit — de eigen conventie van het netwerk — en de vier PI4-tonen vallen waar deze decoder er standaard naar zoekt.

## awards (src/app/awards.rs)

awards-awards = DIPLOMA'S
awards-band = Band
awards-all = Alle
awards-dxcc = DXCC
awards-grids = Grids
awards-worked-all-states = Worked All States
awards-cq-zones-waz = CQ-zones (WAZ)
awards-dxcc-entities = DXCC-entiteiten

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = Audio in de doorlaatband van de decoder.
acars-blocks-framed-blocks-whose-check = Blokken die zijn geframed, en blokken waarvan de controlesequentie mislukte.
acars-starting-acars-receiver = de ACARS-ontvanger wordt gestart…
acars-channel = KANAAL
acars-tune-dial-here = Stem hier af.

acars-channel-out-of-range = Buiten het ontvangstbereik van deze radio.
acars-nothing-yet-acars-quiet-between = Nog niets. ACARS is stil tussen vliegtuigen — laat het op een kanaal staan en wacht op een burst.
acars-check-failed = controle mislukt
acars-block-check-sequence-did-not = De blokcontrolesequentie kwam niet overeen — de tekst kan fout zijn.
acars-no-text = (geen tekst)

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = Gebruikersnaam
login-password = Wachtwoord
login-remember-device = Onthouden op dit apparaat
login-signs-without-asking-next-time = Meldt de volgende keer aan zonder te vragen, en laat het tabblad van de 3D-weergave helemaal zonder vragen toe. Het wachtwoord wordt leesbaar bewaard, dus laat dit uit op een machine die anderen gebruiken.
login-keeps-password-device-clear = Bewaart het wachtwoord op dit apparaat, leesbaar.
login-checking = {" "}CONTROLEREN…{" "}
login-sign = {" "}AANMELDEN{" "}
login-wrong-password-shuts-server-s = Een fout wachtwoord sluit de deur van de server een paar seconden, dus dit kan even duren.

## help (src/help.rs)

help-find = ZOEK
help-find-manual = zoeken in handleiding
help-previous-match-shift-enter-shift = Vorige overeenkomst (Shift+Enter, Shift+F3)
help-next-match-enter-f3 = Volgende overeenkomst (Enter, F3)
help-no-matches = geen overeenkomsten
help-clear-search-esc = Wis de zoekopdracht (Esc)
help-sdroxide-manual = SDROXIDE HANDLEIDING
help-contents = INHOUD

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = 1 station ontvangen — wachten tot het een positie meldt
aismap-double-click-reframe = dubbelklik om opnieuw in beeld te brengen
aismap-zoom-out-wheel-pinch-do = Uitzoomen — het wiel of een knijpbeweging doet dit ook
aismap-zoom-about-middle-chart = Inzoomen rond het midden van de kaart
aismap-frame-everything-being-tracked-again = Breng alles wat wordt gevolgd weer in beeld — een dubbelklik doet dat ook

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = wachten op spectrum…
specview-hounds = HOUNDS

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = Dit venster heeft geen GPU-renderer, dus de waterval blijft op zijn standaardbreedte.
spectrum-machine-drawing-without-gpu-every = Deze machine tekent zonder GPU — elke kolom van de waterval is werk voor de processor, en die deelt dat al met de radio.
spectrum-window-renders-through-opengl-which = Dit venster rendert via OpenGL, het compatibiliteitspad van sdroxide — een Raspberry Pi, een oudere grafische chip of een browser zonder WebGPU. Een bredere waterval is daar de framerate niet waard.
spectrum-wider-than-renderer-can-draw = Breder dan deze renderer kan tekenen.

## macros (src/app/panels/macros.rs)

macros-nocall = NOCALL
macros-remove-button = Verwijder deze knop

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = GRID TRACKER
gridtracker-drag-pan-wheel-zoom-amber = Sleep om te verschuiven, wiel om te zoomen. Amber = gewerkt, groen = bevestigd, cyaan = gehoord.

## remote (src/remote.rs)

remote-server-busy-another-client-connected = server bezet — een andere client is verbonden
remote-transmitting-but-client-s-microphone = Zenden, maar de microfoon van deze client levert helemaal geen audio — er wordt niets gemoduleerd. Controleer in een browser of de pagina toegang tot de microfoon heeft gekregen en of geen andere toepassing hem vasthoudt.
remote-connection-closed = verbinding gesloten

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = 3D-weergave van het zonnestelsel — Zon, Aarde, Maan, zonnevlekken en CME's (apart venster)
solarbtn-solar-system-3d-view-sun-2 = 3D-weergave van het zonnestelsel — Zon, Aarde, Maan, zonnevlekken en CME's (nieuw browsertabblad)

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = De stem wordt geladen…
speech-browser-client-cannot-speak-yet = de browserclient kan nog niet spreken

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = Dubbelklik om een frequentie te typen
freqdisplay-tap-type-frequency = Tik om een frequentie te typen

## solarstate (src/solar3d/state.rs)

solarstate-home = HOME
solarstate-dwarf-planets = DWERGPLANETEN

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = dubbelklik om opnieuw in beeld te brengen

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = dubbelklik om opnieuw in beeld te brengen

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = verbinding gesloten

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = kon de alarmthread niet starten

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = nog geen vliegtuigen gelokaliseerd

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = DUBBELKLIK OM OPNIEUW IN BEELD TE BRENGEN

## chrome (src/chrome.rs)



## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = Stelt de eigen vermogensinstelling van de FDM-DUO in (menu 34 TX POWER). Drive en Tune stellen het vermogen daarbinnen in, en SDRoxide stuurt de instelling opnieuw bij elke verbinding.

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $day } { $month ->
        [1] jan
        [2] feb
        [3] mrt
        [4] apr
        [5] mei
        [6] jun
        [7] jul
        [8] aug
        [9] sep
        [10] okt
        [11] nov
       *[12] dec
    }

## download (src/download.rs)

download-browser-could-not-read-file = de browser kon dat bestand niet lezen

## morse (src/app/morse.rs)

morse-translate = VERTALEN
morse-practice = OEFENEN
morse-learn = LEREN
morse-morse = MORSE
morse-text-morse = Tekst → Morse
morse-morse-text = Morse → Tekst
morse-per-character-space-between-characters = `.` en `-` per teken, een spatie tussen tekens, ` / ` tussen woorden. Het ITU-alfabet, cijfers, leestekens en de gangbare prosigns.
morse-reference-translator-available-desktop-build = De referentievertaler is beschikbaar in de desktopversie.
morse-play-what-you-type-your = Speel af wat je typt, via je eigen luidsprekers
morse-play = AFSPELEN
morse-stop = STOP
morse-every-character-learned-well-done = Elk teken geleerd — goed gedaan.
morse-reveal = TONEN
morse-reset = RESET
morse-replay = OPNIEUW
morse-what-did-you-hear = Wat heb je gehoord?
morse-check = CONTROLEER
morse-press-new-hear-character = Druk op NIEUW om een teken te horen.
morse-tone = Toon
morse-speed = Snelheid
morse-spacing = Tussenruimte
morse-characters-go-out-speed-above = Tekens gaan uit met de snelheid hierboven; dit is de snelheid waarmee de *pauzes* worden verzonden, zodat een beginner de letter als geheel hoort voordat hij haar zo snel kan verzenden.
morse-playback-available-desktop-build = Afspelen is beschikbaar in de desktopversie.

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = hoogte
adsb-row-track = koers
adsb-row-turning = draaiend
adsb-row-squawk = squawk
adsb-row-range = afstand
adsb-row-position = positie
adsb-row-signal = signaal
adsb-row-frames = frames
rade-transmitting = zenden
decodes-odd = ODD
decodes-new = NIEUW
js8-new = NIEUW

## AIS panel: vessel detail rows.

ais-row-status = status
ais-row-speed = snelheid
ais-row-course = koers
ais-row-heading = richting
ais-row-turning = draaiend
ais-row-altitude = hoogte
ais-row-size = grootte
ais-row-draught = diepgang
ais-row-destination = bestemming
ais-row-range = afstand
ais-row-signal = signaal
ais-row-type = type
ais-row-position = positie
ais-row-messages = berichten
ism-off = UIT
textmodem-frequency-shifted = frequentieverschoven
winlink-to-heading = AAN

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = { $heard } ontvangen — geen positie nieuwer dan { $drop_map_s }s
aismap-stations-heard-waiting-position-report = { $heard } stations ontvangen — wachten op een positiemelding
aismap-km-across = { $lon_span }° · ~{ $km } km breed

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = alarm op { $device }
alerts-alarm-device-unavailable = alarmapparaat niet beschikbaar: { $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = { $w } gewerkt
awards-confirmed = { $value } bevestigd

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · voorspelling van HAMQSL.com, { $value } oud
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · volgend baken over { $now } s
bands-radiants-placed = radianten geplaatst voor { $lat }°, { $lon }°
bands-zhr = ZHR { $zhr }
bands-radiant = radiant { $alt_deg }° { $az_deg }
bands-radiant-down = radiant onder de horizon ({ $alt_deg }°)

    Een radiant boven de horizon betekent dat de sporen je kunnen bereiken; een snelle zwerm laat langer levende geïoniseerde sporen na voor meteorscatter op 6 m en 2 m, en de korte openingen op 10 m.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } wordt niet gedecodeerd
drm-symbols = { $value } symbolen · { $frame }
drm-not-decodable = { $value } — niet te decoderen
drm-broadcaster-s-clock-utc = klok van de omroeper { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    MAX: het volledige uitgangsvermogen van de FDM-DUO, ongeveer 8 W. De instelling voor gebruik met de HR500 in de keten.

    { $base }
duopower-t-5-w-hr500-line-will =
    5 W. Met de HR500 in de keten bereikt hij op de meeste banden bij deze instelling niet het volle vermogen: gebruik MAX, en Drive om het te verlagen.

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } W, voor zenden zonder eindtrap of QRP.

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = Schakelt zenden weer in na { $swr }:1. Controleer eerst de antenne: als de storing er nog is, stopt ook de volgende transmissie.
frame-reconnecting-s = Opnieuw verbinden over { $value }s…

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = { $worked } gewerkt
gridtracker-confirmed = { $confirmed } bevestigd
gridtracker-heard = { $heard } gehoord

## hd: messages with values (src/app/hd.rs)

hd-decode = Decoderen { $value } ({ $codec_label })

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } kHz heeft al een chip
hfdl-save-current-channel-khz-one = Het huidige kanaal ({ $cur_khz } kHz) opslaan als een van je eigen chips
hfdl-aircraft = { $aircraft } vliegtuigen
hfdl-bursts = { $bursts } bursts
hfdl-decodes-2 = { $decodes } decoderingen
hfdl-fixed = { $fec } hersteld

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint } — nog niet gedecodeerd
ism-listen-tunes-radio-there-one = Luisteren op { $value } — stemt de radio daarop af. Eén band tegelijk: ze liggen te ver uit elkaar om met één ontvanger twee tegelijk te dekken.
ism-watch-hz-around-band-centre = Bewaak { $value }Hz rond het bandcentrum, wat de band zelf ook zou vragen. Smaller past bij een ontvanger die de eigen bandbreedte van de band niet kan leveren; breder bereikt apparaten die verder van het centrum zitten. Je ontvanger moet ongeveer een derde meer leveren dan dit.
ism-rtl-433 = rtl_433 — { $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } kHz
ism-your-own = { $flex } van jezelf
ism-decoders-decoded = { $decoders } decoders ({ $flex }), { $decodes } gedecodeerd
ism-handled-rtl-433 = afgehandeld door rtl_433: { $names }
ism-flex = flex: { $error }
ism-window-mhz-mhz-wide = venster { $mhz } MHz, { $e62 } MHz breed
ism-bursts-decoded = { $bursts } bursts, { $decodes } gedecodeerd
ism-devices = { $ism_reports } apparaten
ism-id-mhz-db-heard-first =
    { $protocol } id { $device }
    { $e6 } MHz { $snr_db } dB { $value }× gehoord
    voor het eerst gehoord { $first_at } geleden

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = Bevestigingen: { $recs } gedownload, { $matched } nieuw bevestigd
logbook-qso = { $qso_log } QSO
logbook-worked-before = ⚠ AL EERDER GEWERKT ({ $dupe_band })
logbook-utc-qso = { $oldest }–{ $newest } UTC · { $value } QSO

## app: messages with values (src/app/mod.rs)

app-radio = radio { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = kon de Morse-audiothread niet starten: { $error }
morse-no-audio-output = geen audio-uitvoer: { $error }
morse-about-s = ongeveer { $farnsworth_wpm } s
morse-characters-play = Tekens in het spel: { $set }
morse-score-run = Score: { $correct } / { $total }. Reeks: { $streak } van { $value }.
morse-correct = juist — { $value }
morse-was = dat was { $value }

## net: messages with values (src/app/net.rs)

net-adif-import-failed = ADIF-import mislukt: { $error }
net-not-unicode-read = {" "}(geen Unicode; gelezen als { $enc })
net-received-reports-swl-left-out = , { $n } ontvangstrapporten (SWL) weggelaten
net-adif-import-added-duplicates-skipped = ADIF-import: { $added } toegevoegd, { $skipped } duplicaten overgeslagen{ $reports }{ $assumed }
net-channel-import-failed = Kanaalimport mislukt: { $error }
net-line-s-skipped = , { $n } regel(s) overgeslagen
net-not-unicode-read-2 = {" "}(geen Unicode; gelezen als { $enc })
net-channel-import-channel-s-read = Kanaalimport: { $n } kanaal/kanalen gelezen{ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = { $frames } frames · { $bad } slecht
acars-mode = modus { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = { $aircraft } vliegtuigen
adsb-frames = { $frames } frames
adsb-preambles = { $preambles } preambules
adsb-bad-crc = { $bad_crc } slechte CRC
adsb-tune = AFSTEMMEN { $mhz }
adsb-ft-barometric = { $ft } ft barometrisch

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = { $vessels } schepen
ais-messages = { $messages } berichten
ais-slots = { $bursts } slots
ais-bad-fcs = { $bad_fcs } slechte FCS
ais-ais-mhz = AIS { $value } ({ $mhz } MHz) — { $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value } ({ $mhz } MHz) — { $bursts } slots, { $messages } berichten, ruisvloer { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = AFSTEMMEN { $mhz }
ais-ships-heard-are-khz-off = de gehoorde schepen zitten { $khz } kHz naast de frequentie ({ $mhz } ppm bij 162 MHz) — stel de frequentiecorrectie van de front-end in, anders worden zendingen vanaf ongeveer 5 kHz niet meer gedecodeerd
ais-over-ground = { $value }° over de grond
ais-dbfs-db-snr = { $rssi_dbfs } dBFS, { $snr_db } dB SNR
ais-last-type-ais = { $messages } (laatste type { $last_type }, AIS { $channel })
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } MHz — marifoonkanaal { $ch }. Een schip wisselt tussen de twee, dus als je er een uitzet, wordt elk schip half zo vaak gehoord.

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = object van { $reported_by }
aprs-via = via { $via }
aprs-object-reported = object gemeld door { $reported_by }
aprs-digit-s-blanked = ± { $ambiguity } cijfer(s) leeggelaten
aprs-wind-m-s = wind { $value }° { $value2 } m/s
aprs-gust = windstoot { $value }
aprs-rain-mm-h = regen { $value } mm/h
aprs-frame-s-last-ago = { $packets } frame(s), laatste { $last_heard } geleden
aprs-via-2 = via { $via }
aprs-bad = { $bad_frames } slecht
aprs-other = { $non_aprs } overig
aprs-sent = verzonden ×{ $tries }
aprs-stn = { $stations } stn
aprs-queued = { $tx_queue } in wachtrij

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = master { $value }
atchat-last-heard-s-ago-click = { $status } — laatst gehoord { $age_s }s geleden · klik om een tabblad voor privéberichten te openen
atchat-close-tab = het tabblad { $peer } sluiten
atchat-direct-message = privébericht aan { $value }
atchat-cannot-display = kan { $path } niet weergeven

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Klik voor de { $neural }-decoder. De neurale decoder kopieert dieper in de ruis en leest met de hand gegeven CW die een timingfit niet accepteert; de timingdecoder is de enige die de letters met accenten kopieert — Ä, Ö, Å, Ü, É — omdat het model er geen uitvoerklasse voor heeft.
cw-idle-s = INACTIEF { $idle }s

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = De { $n } decoderingen in de lijst opslaan als ADIF — ontvangstrapporten, geen contacten. Decoderingen zonder zender worden weggelaten.
decodes-save-decodes-list-csv-one = De { $n } decoderingen in de lijst opslaan als CSV, één rij per stuk
decodes-session-qso = Sessie: { $session } QSO
decodes-save-whole-logbook-qso-adif = Het hele logboek ({ $logged } QSO) opslaan als ADIF
decodes-save-whole-logbook-qso-text = Het hele logboek ({ $logged } QSO) opslaan als tekst
decodes-hz-slots = { $audio_hz } Hz · { $odd } slots
decodes-your-slot-timing-against-stations =
    Jouw slottiming ten opzichte van de stations die je hoort.
    { $after }

    Het omvat het hele ontvangstpad, dus een trage audio- of netwerkketen telt net zo zwaar als een verkeerde klok. Onder 0,5 s is prima.
decodes-pile-up = PILE-UP { $fox_queue }
decodes-queue = WACHTRIJ { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } dB · { $audio_hz } Hz{ $unwrap_or_default }
    Klik om te verwijderen

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = Elke snelheid wordt gedecodeerd; ▸ markeert { $speed }, waarmee je zendt
js8-every-speed-being-decoded-click = Elke snelheid wordt gedecodeerd. Klik om in plaats daarvan met { $speed } te zenden
js8-transmit-decode = Zenden en decoderen met { $speed }
js8-beaconing-every-min-click-stop = Bakenzending elke { $hb_min } min — klik om te stoppen
js8-last-beacon-went-out-hz = Het laatste baken ging uit op { $hz } Hz — een vrij slot in de heartbeat-subband van { $value }–{ $value2 } Hz, gekozen zodat het vrij blijft van de signalen die worden gedecodeerd.
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } frames · { $left }s
js8-decoded = Gedecodeerd met { $speed }
js8-frames = … ({ $frames } frames)
js8-reply-2 = Antwoord aan { $value }: “{ $value2 }”
js8-address-composer = Het opstelvenster aan { $value } richten

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i }: verzendt “{ $text }”
macros-sends = Verzendt “{ $text }”
macros-most = { $value } is het meest

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = { $repaired } hersteld · { $lost } verloren
navtex-characters-lost = { $lost } tekens verloren

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = { $bad_frames } slecht
packet-link-layer = De verbindingslaag staat op { $link }
packet-via = {" "}via { $via }
packet-unacked = { $unacked } onbevestigd
packet-retry = nieuwe poging { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] baken
       *[other] bakens
    }
pi4-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } geleden

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = SNR { $snr_db } dB
rade-offset-hz = offset { $freq_offset_hz } Hz
rade-heard = gehoord { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ { $dropped } samples verloren tussen de ontvanger en de decoder — deze machine kan de neurale decodering niet bijhouden.

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } teken/s
textmodem-char-s-hz-wide = { $value } — { $chars_per_sec } teken/s, { $bandwidth_hz } Hz breed, { $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = { $frames } frames
vdl2-bursts = { $bursts } bursts
vdl2-sync = { $syncs } sync
vdl2-hdlc-bad = { $hdlc_bad } HDLC slecht
vdl2-bad-fcs = { $fcs_bad } slechte FCS
vdl2-mhz-bursts-frames-noise-floor = { $mhz } MHz, { $what } — { $bursts } bursts, { $frames } frames, ruisvloer { $floor_dbfs } dBFS
vdl2-tune = AFSTEMMEN { $mhz }
vdl2-held-2 = VASTGEHOUDEN +{ $since }
vdl2-use-worldwide-where-every-link = { $what } — wereldwijd in gebruik, en waar elke verbinding begint. Die houd je aan als je er maar één houdt.
vdl2-assigned-which-channels-carry-anything = Toegewezen aan een { $what }. Welke kanalen iets voeren hangt af van waar je bent: laat ze allemaal aan tenzij je het anders weet.
vdl2-mhz-db-snr-dbfs-evm = { $mhz } MHz · { $snr_db } dB SNR · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Hz · { $rs_corrected } RS-symbolen hersteld
vdl2-characters = { $parity_errors } tekens
vdl2-parameter-sdroxide-does-not-read = { $unknown } { $unknown ->
        [one] parameter
       *[other] parameters
    } die SDRoxide niet leest — hierboven getoond als identificatie en bytes

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = { $live_size } regels vastgehouden
wefax-pixels-per-line = { $width } pixels per regel
wefax-shift-picture-pixels = Het beeld { $px } pixels verschuiven
wefax-charts-are-saved-click-copy =
    Kaarten worden opgeslagen { $place }

    Klik om het pad te kopiëren
wefax-finished-charts-are-saved-collect = Voltooide kaarten worden opgeslagen { $place } en verzamelen zich hier.
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } pixels
    { $value }

    Klik met rechts om te verwijderen
wefax-older-store = { $older } ouder in de opslag
wefax-older-load-more = { $older } ouder — meer laden
wefax-published-carrier-khz-dial-khz = Gepubliceerde draaggolf { $f } kHz → afstemming { $value } kHz USB
wefax-delete-chart-2 = Deze kaart verwijderen { $dir }

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · CQ-zone { $cq_zone } · ITU-zone { $itu_zone }
widgets-new-entity = Nieuwe entiteit op { $band_label }
widgets-worked-before = Eerder gewerkt op { $band_label }
widgets-calling-cq-includes-you = Roept CQ { $target } — dat geldt ook voor jou
widgets-calling-cq-not-aimed-you = Roept CQ { $target } — niet op jou gericht

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = { $heard_us } rx · { $heard_us2 } hoorden ons
wspr-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] baken
       *[other] bakens
    }
wspr-beaconing-duty = bakenzending — { $wspr_power_dbm }, { $wspr_tx_percent }% duty
wspr-listening-duty = luisteren ({ $wspr_tx_percent }% duty)
wspr-beacon-one-two-minute-slot = Baken in één tweeminutenslot van elke { $pct } — { $pct2 }% ervan, gelijkmatig verdeeld — op { $wspr_power_dbm }. Op welk slot de cyclus begint wordt afgeleid van je roepletters, dus twee stations met dit programma beginnen niet tegelijk.
wspr-transmitting-set-general-tab-settings = Zenden als { $call } in { $sent } — in te stellen op het tabblad Algemeen van de Instellingen.
wspr-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } geleden

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    antenne: { $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = { $entries } ontvangers · { $per_net } · opgehaald { $age } geleden
publicsdr-showing-search-narrow = toont { $shown } van { $n } — zoek om te verfijnen
publicsdr-match = { $n } treffer
publicsdr-replace-taking-receiver-points-same =
    { $this } vervangen door { $entry }?

    { $this2 } staat op { $iface }. Als je deze ontvanger neemt, wijst hetzelfde tabblad voortaan naar { $address }, wordt het hernoemd naar “{ $entry2 }” en blijft de afstemming beperkt tot het { $range_label } dat de ontvanger publiceert — hier wordt niets verzonden.

    De instellingen van { $iface2 } blijven zoals ze zijn, net als de bereiken die ervoor zijn opgegeven: als je in Instellingen → Radio weer naar de interface terugschakelt, is de radio weer zoals hij was.
publicsdr-receptions-will-reported = {" "}Ontvangsten worden gemeld vanaf { $value }.
publicsdr-pointing-radio = Deze radio richten op { $entry }…{ $site }
publicsdr-opening-another-radio = { $entry } openen als een andere radio…

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } kHz
qo100-before-switching-tune-so-beacon = Vóór het AANZETTEN: stem zo af dat de twee lobben van het baken tussen +{ $park_lo_hz } en +{ $park_hi_hz } kHz in de strook liggen (de gearceerde baan).
qo100-receiver-currently-listening-around-mhz = De ontvanger luistert momenteel rond { $mhz } MHz — ver van het baken op 10489,750 MHz, dus er is hier niets te tekenen. De decoder blijft hoe dan ook zoeken.
qo100-receiver-currently-covers-only-khz = ontvanger dekt hier momenteel slechts ±{ $khz } kHz — de rest van de strook blijft leeg
qo100-null-db-sym-snr-db = null { $est_null_depth_db } dB sym { $est_symmetry } snr { $est_snr_db } dB
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Hz/s { $est_drift_accel_hz_s2 } Hz/s² (vóór decodering terugroteerd)
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Hz/s (vóór decodering terugroteerd)
qo100-found-empty = { $est_updates } gevonden, { $est_misses } leeg
qo100-last-s-ago = {"   "}laatste { $auto_last_hz } ({ $auto_last_unix }s geleden)
qo100-over-correction = { $auto_total_hz } over { $auto_applies } { $auto_applies ->
        [one] correctie
       *[other] correcties
    }{ $last }
qo100-sync-32-err = sync { $sync_bit_errors } / 32 fout ×{ $sync_matches }
qo100-frame-buffer = framebuffer { $frame_fill }%
qo100-blocks-tried-decoded = { $blocks_tried } blokken geprobeerd, { $blocks_locked } gedecodeerd
qo100-last-applied-s-ago-hz = laatst toegepast { $ago }s geleden: { $old } → { $new } Hz ({ $old2 } Hz)
qo100-locked-block-tried-locked = vergrendeld — { $blocks_tried } { $blocks_tried ->
        [one] blok
       *[other] blokken
    } geprobeerd, { $blocks_locked } vergrendeld
qo100-searching-block-tried-none-locked = zoeken — { $blocks_tried } { $blocks_tried ->
        [one] blok
       *[other] blokken
    } geprobeerd, nog geen vergrendeld

## rds: messages with values (src/app/rds.rs)

rds-auto = Auto ({ $standard })
rds-type = type { $class }
rds-groups = · { $groups } groepen
rds-blocks-error = · { $ber } % van de blokken met fouten
rds-clean-corrected-lost = { $blocks_ok } schoon · { $blocks_corrected } gecorrigeerd · { $blocks_bad } verloren

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC ({ $offset } lokaal)
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } boven de horizon
sat-min = over { $value } min
sat-h-min = over { $value } u { $value2 } min
sat-searching = zoeken… { $next } van { $entries }
sat-locked-2 = ● VERGRENDELD — { $t }
sat-pass-until-max = Overkomst tot { $set_unix } · max { $max_el }°
sat-next-pass-rises-max = Volgende overkomst { $rise_unix } { $rise_unix2 } ({ $now }) · komt op { $rise_az }° { $rise_az2 } · max { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ { $clients } rigctld-{ $clients ->
        [one] client
       *[other] clients
    } verbonden — satellietsoftware die de afstemming daar bijstuurt, zou Doppler twee keer corrigeren
sat-rotator-antenna-az-el = Rotor: antenne op { $az }° az { $el }° el
sat-rotator-2 = Rotor: { $error }
sat-up-now-2 = ● nu boven de horizon, { $el }°
sat-inv = { $mode } · inv
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = vasthouden { $here } MHz
scanner-scanning-mhz = scannen · { $here } MHz

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = Deze renderer verwerkt geen textuur die breder is dan { $max_texture_dim } pixels.

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = Stem: { $voice }
speech-speech-unavailable = Spraak is niet beschikbaar: { $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = { $stations } zendingen

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = laatste sessie mislukt — { $err }
winlink-last-session-received-sent = laatste sessie: { $last_received } ontvangen, { $last_sent } verzonden
winlink-empty = { $to_lowercase } is leeg
winlink-showing = toont { $entries } van { $total }
winlink-fetching = ophalen { $mid }…
winlink-from-2 = van { $msg } { $date }
winlink-attachment-bytes = bijlage: { $att } ({ $data } bytes)

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    object van { $reported_by }
aprsmap-heard-ago =
    {""}
    gehoord { $age } geleden

## download: messages with values (src/download.rs)

download-reading = lezen van { $path }: { $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}afbeelding ontbreekt: { $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } MHz
hfdlmap-fix =
    {""}
    { $fixes } positiebepaling{ $es }
hfdlmap-heard-ago =
    {""}
    gehoord { $last_at } geleden

## multi: messages with values (src/multi.rs)

multi-radio = Radio { $n }
multi-could-not-add-radio = Kon geen radio toevoegen: { $error }
multi-was-closed-station = { $value } is gesloten bij het station.
multi-could-not-open = Kon { $peer } niet openen: { $error }
multi-has-tab-own-now = { $value } heeft nu een eigen tabblad.

## remote: messages with values (src/remote.rs)

remote-protocol-error = protocolfout: { $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = ongeldig bericht: { $error }
solarnet-solar-protocol-mismatch-server-client = zonneprotocol komt niet overeen: server { $proto }, client { $value }

## smeter: messages with values (src/widgets/smeter.rs)

smeter-forward-power-w-measured-radio =
    {""}

    Voorwaarts vermogen — { $w } W, gemeten door de eigen sensor van de radio.
smeter-po-radio-s-own-power =
    {""}

    PO — de eigen uitgangsvermogenmeter van de radio, op { $clamp } % van het volle bereik. Een naaldstand zoals de radio die meldt, geen gekalibreerd wattage; let erop of hij daalt terwijl Drive gelijk blijft.
smeter-swr-1-match-antenna-presenting =
    {""}

    SWR — { $swr }:1, de aanpassing die de antenne biedt, zoals de radio die meet. Onder 2:1 is niets bijzonders; boven 3:1 kleurt de schaal rood.
smeter-received-signal-strength-s9-db = Ontvangen signaalsterkte: S9 + { $over } dB ({ $s_dbm } dBm in de doorlaatband).
smeter-received-signal-strength-s-dbm = Ontvangen signaalsterkte: S{ $value } ({ $s_dbm } dBm in de doorlaatband).
smeter-radio-reports-c =
    {""}

    De radio meldt { $t } °C.
smeter-ps-puresignal-correcting-db-compression =
    {""}

    PS — PureSignal corrigeert { $correction_db } dB compressie{ $held } (de terugkoppeling kwam overeen met de zending op { $score }).
smeter-ps-puresignal-switched-but-has =
    {""}

    PS −− — PureSignal staat aan maar heeft de zending niet gevonden in het terugkoppelpad (beste overeenkomst { $score }), dus de zender gaat ongecorrigeerd de lucht in. Controleer of een monster van de uitgang van de versterker echt een ontvangstingang bereikt die de T/R-schakelaar bij zenden niet wegneemt, en of de verzwakker het niet opslokt. De lus luistert alleen op deze ontvanger — een koppelaar die op een tweede ontvanger is aangesloten (RX2, ADC2) wordt nooit als terugkoppeling gelezen.

## specview: messages with values (src/widgets/spectrum_view.rs)

specview-rit-hz = RIT { $hz } Hz
specview-xit-hz = XIT { $hz } Hz

## decodes: nested messages

decodes-you-transmit-s-everyone-else = Je zendt { $off } s { $after } alle anderen — de gebruikelijke reden dat oproepen onbeantwoord blijven. Synchroniseer je computerklok.

## publicsdr: nested messages

publicsdr-noise-floor-score = ruisvloerscore { $snr }
publicsdr-up-khz-i-q = tot { $khz } kHz aan I/Q

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = GR gate { $gate_gr_db } de-ess { $deess_gr_db } comp { $comp_gr_db } lim { $limiter_gr_db }

## amp: more messages

amp-h-step = ~{ $l_step } µH · stap { $l_step2 }
amp-pf-step = ~{ $c_step } pF · stap { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel }-bit · { $media_type } / { $content_encoding } · { $encoded_size } octetten in { $chunk_count } chunks ({ $chunks_first_pass } eerste ronde) · sessie { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value } — { $symbol_rate } baud CPFSK, ±{ $deviation_hz } Hz, { $bandwidth_hz } kHz bezette bandbreedte

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · { $local } lokaal
sat-in-seconds = over { $s } s
winlink-to-list = aan { $to }
winlink-cc-list = cc { $cc }
spots-n-of-total = { $n } van { $total }
wefax-n-of-total = { $n } van { $total }
ais-range-bearing = { $km } km op { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = Uit (alleen tuner, vanaf 24 MHz)
rtl-sdr-hf-mode-auto = Automatisch
rtl-sdr-hf-mode-direct-q = Directe bemonstering (Q-tak)
rtl-sdr-agc-manual = Handmatig (geen AGC)
rtl-sdr-agc-tuner = Tuner-AGC
rtl-sdr-agc-rtl = Digitale RTL-AGC
rtl-sdr-agc-both = Tuner + RTL-AGC
mode-control-cat = CAT
mode-control-radio = Door radio bestuurd
line-state-none = Geen
line-state-high = Hoog
line-state-low = Laag
elad-tx-input-usb-audio = USB-audio
elad-tx-input-mic = Microfoon
elad-tx-input-auto = Auto
elad-tx-input-leave = Laten zoals ingesteld op de radio
tr-usdx-audio-one-cable = Eén kabel (audio in de CAT-stroom)
tr-usdx-audio-sound-card = USB-geluidskaart (3,5 mm-aansluiting)
spy-server-format-uint8 = 8-bit (laagste bandbreedte)
spy-server-format-int16 = 16-bit
spy-server-format-float32 = 32-bit float
sound-format-iq = IQ (stereo)
sound-format-demod-audio = Gedemoduleerde audio
sdr-play-duo-tuner-tuner1 = Tuner 1 (50 Ohm / Hi-Z)
sdr-play-duo-tuner-tuner2 = Tuner 2 (50 Ohm)
sdr-play-duo-role-diversity = Een tweede antenne (diversity / QRM-onderdrukking)
sdr-play-duo-role-second-radio = Een tweede radio, op een eigen frequentie
sdr-play-agc-off = Uit (handmatige IF-versterking)
sdr-play-agc-hz5 = 5 Hz
sdr-play-agc-hz50 = 50 Hz
sdr-play-agc-hz100 = 100 Hz
pluto-ptt-off = Uit
pluto-ptt-gpo01 = GPO0 = RX, GPO1 = TX
pluto-ptt-gpo23 = GPO2 = RX, GPO3 = TX
pluto-duplex-fdd = FDD (beide tegelijk)
pluto-duplex-tdd = TDD (één tegelijk)
pluto-agc-manual = Handmatig
pluto-agc-slow-attack = Trage attack
pluto-agc-fast-attack = Snelle attack
pluto-agc-hybrid = Hybride
parity-none = Geen
parity-even = Even
parity-odd = Oneven
panadapter-tap-antenna = Antenne (gedeeld met de radio)
panadapter-tap-if-output = De IF-uitgang van de radio
panadapter-audio-attached = De aangesloten ontvanger
panadapter-audio-transceiver = De transceiver
kenwood-send-ts2000 = TS-2000-stijl (TX;)
kenwood-send-ts590 = TS-590-stijl (TX1;)
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = Door radio bestuurd
cw-keying-cat = Rig-keyer (CAT)
cw-keying-audio = Geluidskaart (MCW)
alert-sound-ding = Ding
alert-sound-two-tone = Tweetonig
alert-sound-triplet = Triplet
alert-sound-warble = Tril
alert-sound-digital = Digitaal
alert-reply-tone = Toon
alert-reply-voice = Stem
alert-reply-both = Toon + stem
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
icom-model-other = Anders (adres handmatig instellen)
backend-auto = Automatisch detecteren (SoapySDR / CAT)
backend-soapy = SoapySDR
backend-cat = CAT / Audio
backend-hpsdr = HPSDR (netwerk)
backend-tci = TCI (netwerk)
backend-icom-net = Icom LAN (netwerk)
backend-smart-sdr = SmartSDR / FlexRadio (netwerk)
backend-pluto = PlutoSDR (netwerk)
backend-rtl-sdr = RTL-SDR (USB)
backend-rtl-tcp = RTL-SDR via rtl_tcp (netwerk)
backend-spy-server = SpyServer (netwerk)
backend-spy-server-vfo = SpyServer VFO+FFT, lage bandbreedte (netwerk)
backend-kiwi-sdr = KiwiSDR / Web-888 (netwerk)
backend-rx888 = RX-888 (USB)
backend-airspy-hf = Airspy HF+ (USB)
backend-airspy = Airspy R2 / Mini (USB)
backend-hack-rf = HackRF One / Pro (USB)
backend-sdr-play = SDRplay RSP (USB)
backend-elad = ELAD FDM-DUO / FDM-S (USB)
backend-lime = LimeSDR + LimeRFE (LimeSuite)
backend-hydra-sdr = HydraSDR RFOne (USB)
backend-fobos = RigExpert Fobos SDR (USB)
backend-usb-audio = USB-audioradio (geluidskaart)
backend-none = Niet geconfigureerd
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld (netwerk)
cat-family-flrig = flrig (netwerk)
publicsdr-its-interface = zijn interface

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = Uit
agc-mode-slow = Langzaam
agc-mode-med = Med
agc-mode-fast = Snel
airspy-gain-linearity = Lineariteit (beste verwerking van sterke signalen)
airspy-gain-sensitivity = Gevoeligheid (beste voor zwakke signalen)
ais-kind-class-a = Klasse A
ais-kind-class-b = Klasse B
ais-kind-base-station = basisstation
ais-kind-sar-aircraft = SAR-vliegtuig
ais-kind-aid-to-navigation = navigatiehulpmiddel
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = bijbehorend vaartuig
alert-event-called = Een station roept mij
alert-event-cq = Een CQ die ik kan beantwoorden
alert-event-new-dxcc = Nieuwe DXCC-entiteit
alert-event-new-dxcc-band = Nieuwe entiteit op deze band
alert-event-new-grid = Nieuw locatorvak
audio-routing-both = Links + rechts
audio-routing-left = Alleen links
audio-routing-right = Alleen rechts
button-mode-momentary = Ingedrukt houden
button-mode-toggle = Schakelen
contest-mode-none = Geen
contest-mode-eu-vhf = EU VHF-contest
converter-tx-off = Uit tijdens conversie
converter-tx-transverter = Via dezelfde converter
converter-tx-own = Eigen offset
cw-engine-neural = NEURAAL
cw-engine-timing = TIMING
decode-sort-none = Geen
decode-sort-signal = SNR
decode-sort-distance = Afst
decode-sort-country = Land
diversity-mode-cancel = Opheffen — een ruisbron nullen
diversity-mode-combine = Combineren — diversity-ontvangst
dxped-mode-normal = Normaal
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = Weigeren te zenden
fail-safe-warn-only = Toch zenden, waarschuwen
ftp-security-plain = FTP (geen versleuteling)
ftp-security-explicit-tls = FTPS expliciet (AUTH TLS)
ftp-security-implicit-tls = FTPS impliciet
hpsdr-filter-board-none = Geen — uitgangen blijven uit
hpsdr-filter-board-n2adr = N2ADR-filterbord
hpsdr-filter-board-alex = Alex / Hermes-bandcode (Zeus, HiQSDR, Quisk)
hpsdr-filter-board-custom = Aangepast — mijn eigen tabel hieronder
hpsdr-io-rx-input-radio = Eigen ingang van de radio
hpsdr-io-rx-input-io-board = IO-bord J9
hpsdr-io-rx-input-io-board-pure-signal = IO-bord J9, PureSignal bij zenden
hydra-sdr-gain-linearity = Lineariteit (beste verwerking van sterke signalen)
hydra-sdr-gain-sensitivity = Gevoeligheid (beste voor zwakke signalen)
icom-rx-source-af = AF — de radio demoduleert
icom-rx-source-if12k = 12 kHz IF — sdroxide demoduleert
icom-scope-span-radio = Zoals ingesteld op de radio
icom-scope-span-khz2-5 = 5 kHz (±2,5k)
icom-scope-span-khz5 = 10 kHz (±5k)
icom-scope-span-khz10 = 20 kHz (±10k)
icom-scope-span-khz25 = 50 kHz (±25k)
icom-scope-span-khz50 = 100 kHz (±50k)
icom-scope-span-khz100 = 200 kHz (±100k)
icom-scope-span-khz250 = 500 kHz (±250k)
icom-scope-span-khz500 = 1 MHz (±500k)
ism-family-weather = WEER
ism-family-meter = METERS
ism-family-home-auto = THUIS
ism-family-lo-ra = LORA
ism-family-unidentified = ONBEKEND
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = niet geïdentificeerd
js8-speed-normal = NORMAAL
js8-speed-fast = SNEL
js8-speed-turbo = TURBO
js8-speed-slow = LANGZAAM
lime-aux-role-off = Niet gebruikt
lime-aux-role-diversity = Een tweede antenne (diversity / QRM-onderdrukking)
lime-aux-role-pure-signal = Zendterugkoppeling (PureSignal-predistorsie)
lookup-provider-none = Uit
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = INBOX
mail-folder-outbox = UITBOX
mail-folder-sent = VERZONDEN
mail-folder-archive = ARCHIEF
memory-sort-name = Naam
memory-sort-freq = Freq
memory-sort-band = Band
memory-sort-stored = Opgeslagen
mouse-button-middle = Midden
mouse-button-extra1 = Terug (extra 1)
mouse-button-extra2 = Vooruit (extra 2)
nr-level-off = Uit
nr-level-low = Laag
nr-level-medium = Mid
nr-level-high = Hoog
nr-level-rnn-low = RNN laag
nr-level-rnn-med = RNN mid
nr-level-rnn-high = RNN hoog
nr-level-spec-low = SPEC laag
nr-level-spec-med = SPEC mid
nr-level-spec-high = SPEC hoog
nr-level-nr2-low = NR2 laag
nr-level-nr2-med = NR2 mid
nr-level-nr2-high = NR2 hoog
nr-level-df-low = DFNR laag
nr-level-df-med = DFNR mid
nr-level-df-high = DFNR hoog
nr-strength-low = Laag
nr-strength-med = Mid
nr-strength-high = Hoog
orbit-rings-none = geen
orbit-rings-curated = geselecteerd
orbit-rings-all = alle
qso-step-idle = Inactief
qso-step-wait-cq = Wacht op CQ
qso-step-calling-cq = Roept CQ
qso-step-tx-grid = Zend locator
qso-step-tx-report = Zend rapport
qso-step-tx-r-report = Zend R+rapport
qso-step-tx-rr73 = Zend RR73
qso-step-tx73 = Zend 73
qso-step-confirming = Bevestigen
rds-standard-auto = Auto
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = Regio 1 — Europa, Afrika, Midden-Oosten, Noord-Azië
region-r2 = Regio 2 — Amerika
region-r3 = Regio 3 — Zuid- en Oost-Azië, Australazië, Pacific
relative-mode-absolute = Absoluut (fader)
relative-mode-sign-magnitude = Relatief: teken/grootte
relative-mode-twos-complement = Relatief: tweeder complement
relative-mode-binary-offset = Relatief: 64-gecentreerd
relay-link-off = Geen
relay-link-serial = USB-relaisbord (serieel)
relay-link-serial-lines = Seriële RTS/DTR-lijn
relay-link-hid = USB-relaisbord (HID)
relay-link-cm108 = CM108/CM119 geluidskaart-GPIO
relay-link-gpio = Linux GPIO-lijn
relay-link-command = Extern commando
relay-role-unused = Niet gebruikt
relay-role-sdr-antenna = De SDR-antenne aarden
relay-role-amplifier = Een eindversterker / T-R-relais sturen
relay-role-aux = Hulp
relay-role-band-decoder = Banddecoder (filter / transverter)
rfe-channel-wb1000 = Breedband 1 – 1000 MHz
rfe-channel-wb4000 = Breedband 1 – 4 GHz
rfe-channel-ham0030 = HF, tot 30 MHz
rfe-channel-ham0070 = 6 m / 4 m (50 – 70 MHz)
rfe-channel-ham0145 = 2 m (140 – 150 MHz)
rfe-channel-ham0220 = 1,25 m (220 – 225 MHz)
rfe-channel-ham0435 = 70 cm (400 – 450 MHz)
rfe-channel-ham0920 = 33 cm (902 – 928 MHz)
rfe-channel-ham1280 = 23 cm (1220 – 1420 MHz)
rfe-channel-ham2400 = 13 cm (2,3 – 2,5 GHz)
rfe-channel-ham3500 = 9 cm (3,3 – 3,7 GHz)
rfe-channel-cell-band01 = Mobiele band 1
rfe-channel-cell-band02 = Mobiele band 2
rfe-channel-cell-band03 = Mobiele band 3
rfe-channel-cell-band07 = Mobiele band 7
rfe-channel-cell-band38 = Mobiele band 38
rfe-channel-not-selected = Niets geselecteerd
rfe-link-off = Niet verbonden
rfe-link-serial = Eigen USB-kabel (serieel)
rfe-link-board = Via de LimeSDR (GPIO / I²C)
rfe-mode-control-auto = Automatisch (volgt de bekabeling)
rfe-mode-control-rx = Altijd ontvangen
rfe-mode-control-tx = Altijd zenden
rfe-mode-control-tx-rx = Altijd beide (alleen gesplitste connectors)
rifp-encoding-auto = Auto
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = Raw
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = MEM
scan-kind-range = BEREIK
scan-resume-carrier = DRAAGGOLF
scan-resume-timed = TIMER
scan-resume-manual = HANDMATIG
sense-line-off = Niet aangesloten
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD (carrier detect)
shift-simplex = SIMPLEX
shift-minus = −
shift-plus = +
speed-slow = Langzaam
speed-medium = Gemiddeld
speed-fast = Snel
speed-faster = Sneller
speed-fastest = Snelst
tls-verify-system = Vertrouwensopslag van het systeem
tls-verify-pinned = Vastgezette vingerafdruk
tls-verify-off = Niet verifiëren
tone-mode-off = UIT
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN (rouleren)
upload-naming-original = Originele bestandsnaam
wheel-action-zoom = Zoom
wheel-action-tune = Afstemmen
wheel-action-none = Niets

settings-alerts-say = ZEG

settings-alerts-say-hint = Dit alarm hardop laten uitspreken

settings-alerts-say-need-voice = Kies Stem of Toon + stem om het te horen

settings-alerts-say-voice-off = Zet de stem aan in Instellingen → UI → Stemmeldingen

settings-alerts-speaks = zegt: { $text }

settings-picup-keep = bewaren

jtty-starting-receiver = JTTY-ontvanger starten…

jtty-receive-audio-level = Ontvangen audioniveau

jtty-sending = {" "}ZENDEN{" "}

jtty-sent-once-note = Het bericht wordt eenmaal verzonden als je op TX drukt — JTTY is asynchroon, dus er is geen periode om op te wachten en er wordt niets herhaald.

signalid-title = SIGNAAL-ID

signalid-tuned-now = Nu afgestemd

signalid-passband = doorlaatband { $bw }

signalid-band = band { $band }

signalid-find = Zoeken

signalid-find-hint = FT8, meteor, weer, vliegtuigen…

signalid-candidates = { $n } kandidaten voor deze afstemming

signalid-matches = { $n } treffers voor “{ $query }”

signalid-nothing-fits = Niets in de catalogus past bij deze afstemming, modus en doorlaatband.

signalid-no-match = Geen catalogusitem komt hiermee overeen.

signalid-open-url = { $url } openen

windows-chip-del = DEL

windows-chip-edt = EDT

windows-chip-ren = REN

windows-chip-rec = REC

windows-name-hint = naam

windows-waterfall-marks = Waterval

windows-stacked = gestapeld

jtty-listening-note = Luisteren. JTTY-zendingen kunnen op elk moment beginnen — een bericht verschijnt hier kort nadat het is gehoord.

extras-title = EXTRA'S

extras-receivers = Ontvangers

extras-dab-hint = DAB / DAB+ digitale radio — de speler opent onder de waterval

extras-aircraft-watch-hint = VLIEGTUIGEN — een vliegtuig op je volglijst is in de lucht; open de weergave Volgen

extras-aircraft-hint = VLIEGTUIGEN — de kaart van wat de radio hoort: vliegtuigen en radiosondes, in een venster dat je kunt losmaken

extras-radio-hint = RADIO CLASSIC — een klassiek radiopaneel voor omroepontvangst: banden, geheugens, de zendernaam en RadioText. Het dockt onder de waterval in de spraakmodi; Instellingen → Losmaakbaar zet het in een venster

extras-antenna = ANTENNE

extras-antenna-hint = ANTENNE (Experimenteel/Beta) — knippengtes, voetpuntimpedantie en patroon voor gangbare draad-/verticaal-/Yagi-antennes

recjobs-title = OPNAMEN

recjobs-finished = Klaar “{ $name }”

recjobs-skipped = Overgeslagen “{ $name }” — neemt al op

recjobs-recording-name = Opnemen “{ $name }”

recjobs-new-job = + NIEUWE TAAK

recjobs-recording = ● opnemen

recjobs-none = Geen geplande opnames — druk op + NEW JOB om er een toe te voegen.

recjobs-done = klaar

recjobs-edit = bewerken

recjobs-del = wissen

recjobs-station = Zender

recjobs-station-hint = BBC World Service

recjobs-kind = Soort

recjobs-start-utc = Start (UTC)

recjobs-for = Duur

recjobs-save = OPSLAAN

recjobs-kind-audio = Audio

recjobs-kind-iq = I/Q

recjobs-kind-both = Beide

panels-sensitivity = Gevoeligheid

navtex-hunting = zoeken

navtex-hunting-hint =
    Er wordt nog niets gelezen.
    
    Het ligt bijna altijd aan de afstemming. Typ het aangekondigde getal (518) NIET in het frequentievak: NAVTEX wordt als kanaal vermeld en dit programma luistert er net onder. Klik in plaats daarvan op de knop 518 hierboven — die stelt de juiste dial voor je in (516.300).
    
    Als er duidelijk een sterk signaal is maar de tekst nog steeds onzin is, druk dan op REV: het komt omgekeerd binnen.
    
    Bedenk ook dat NAVTEX een dienstregeling is, geen gesprek: een kuststation zendt elke vier uur tien minuten lang, dus stilte is meestal normaal. Je moet luisteren als jouw station aan de beurt is. En 518 kHz komt pas 's nachts tot leven — een stille dagband is geen kapotte decoder.

navtex-sensitivity-hint =
    Gevoeligheid — hoe snel de decoder een signaal vastpakt en vasthoudt.
    
    Auto gebruikt de referentiedrempels (0), het gedrag zoals geleverd.
    
    + accepteert een zwakker of ruiziger signaal; − wacht op een schoner signaal voordat sync wordt gemeld, wat voorkomt dat ruis als tekens wordt gelezen.
    
    Geen volume: NAVTEX is FSK, dus de grijstinten komen van de tonen, niet van de signaalsterkte.


navtex-auto-hint = Keert terug naar de referentiedrempels (0). Gemarkeerd zolang actief.

navtex-tune-dial-usb-below-channel = Stem de dial hier af (USB, 1.710 kHz onder het kanaal).

settings-audio-your-audio = Jouw audio

settings-audio-your-audio-why = De luidsprekers waarop je luistert en de microfoon waarmee je stem uitgaat. Deze horen bij deze machine en niet bij een radio, dus ze zijn hetzelfde voor elke radio waarmee je werkt.

settings-audio-browser-note = Geluidsapparaten worden hier in de browser gekozen, niet in dit venster — de pagina vraagt je browser om de luidsprekers en de microfoon. De geluidskaartinstellingen van je station, verderop op deze pagina, blijven gelden.

settings-audio-speakers = Luidsprekers

settings-audio-channels-hint = Welk(e) luidsprekerkanaal/-kanalen het ontvangen audiosignaal bereikt. Links of rechts dempt het andere kanaal, om aan één kant te luisteren.

settings-audio-microphone-note = De microfoon draagt je stem naar buiten wanneer de rig via geluid (MCW) of via VOX wordt gekeyd. Als je rig zichzelf via CAT keyt, wordt de microfoon helemaal niet gebruikt voor zenden en verandert deze rij niets.

settings-audio-stream-out = Audio uitstromen

settings-audio-stream-out-why = Stuur wat deze radio ontvangt via het netwerk naar een ander programma — een decoder, een recorder, een tweede machine — zonder geluidskaart.

settings-audio-send-to-hint =
    `host:port`. Mono, 16-bit, met de ontvangstsnelheid, één datagram per audioblok, en het is de audio van de **opnametap**: AF-volume, MUTE en zenden hebben er geen invloed op, omdat een externe decoder de radio wil horen en niet je koptelefoonniveau.
    
    Een numeriek adres, geen hostnaam. Leeg is uit.

settings-audio-send-to-note = `127.0.0.1:7355` stuurt het naar een ander programma op deze computer; het eigen adres van deze machine stuurt het over het netwerk.

settings-audio-radio-audio = Audio van deze radio

settings-audio-radio-audio-why = De geluidskaart tussen deze radio en deze computer. Die hoort bij de radio en niet bij de machine: twee radio's hier kunnen de computer delen en toch verschillende kaarten nodig hebben, dus deze instellingen blijven bij de radio waarvandaan je Instellingen hebt geopend.

settings-audio-no-cards-yet = Deze machine heeft haar geluidskaarten nog niet doorgegeven. De namen kunnen hieronder nog steeds worden getypt; een lijst verschijnt zodra de apparaten zijn opgevraagd.

settings-audio-rescan = Geluidskaarten opnieuw zoeken

settings-audio-rescan-hint =
    Vraag de machine opnieuw welke geluidskaarten ze heeft. Er wordt niets geopend, dus dit is veilig tijdens het ontvangen.
    
    Gebruik het nadat je een codec hebt aangesloten. Het opent de radio niet opnieuw — dat is Toepassen / opnieuw verbinden op het tabblad Radio — en het kan een kaart die de geluidsserver heeft hernoemd niet herstellen; dat wordt bij de radio gemeld.

settings-audio-from-radio = Van de radio

settings-audio-to-radio = Naar de radio

settings-audio-radio-note = { "*" }*Van de radio** is wat je rig uitstuurt — een USB-geluidsuitgang, of de koptelefoonaansluiting van een ontvanger naar de lijningang van een geluidskaart. **Naar de radio** is wat de rig voor je stem beluistert, en is ook hoe het programma een rig via geluid keyt: laat het leeg en de eigen microfoon van de rig wordt gebruikt, als die er is.

settings-audio-recording = Opname

settings-audio-recording-why = Waar audio-opnames, geplande opnames en ruwe I/Q-opnames naartoe worden geschreven.

enigma-need-more = geef meer cijfertekst om mee te werken

enigma-title = ENIGMA

enigma-no-cables = geen kabels

enigma-thin = dun

enigma-rotor = Rotor

enigma-window-hint = Grundstellung — klik om het wiel een stap te draaien

enigma-input-hint = typ, of gebruik de toetsen hierboven

enigma-clear = WISSEN

enigma-copy = KOPIËREN ⇄

enigma-solve-note = LÖSEN — plak een cijfertekst. Geef een crib-woord dat je in het bericht verwacht voor een bekabelde tekst; zonder crib verwerkt de solver een machine zonder bekabeling.

enigma-crib = Crib

enigma-solve = OPLOSSEN

enigma-flaw = Een letter wordt nooit in zichzelf versleuteld — daar zorgt de reflector voor. Dat is de echte fout van de machine, en daarop snoeit de solver.

settings-audio-takes-over-speakers = { "*" }*Deze luidsprekers zijn een apparaat van de geluidsserver, geen geluidskaart.** Door ze te gebruiken worden ze ook het afspeelapparaat voor *al het andere* op deze computer — andere programma's spelen er dan ook doorheen. PipeWire biedt een programma hier geen manier om een eigen afspeeldoel te kiezen, dus dit is onvermijdelijk. Om het te wijzigen: kies een geluidskaart, of zet de standaard van de machine terug (`wpctl set-default <name>`) en zoek opnieuw.

settings-audio-takes-over-microphone = { "*" }*Deze microfoon is een apparaat van de geluidsserver, geen geluidskaart.** Door hem te gebruiken wordt hij ook het opnameapparaat voor *al het andere* op deze computer — andere programma's nemen er dan ook van op. PipeWire biedt een programma hier geen manier om een eigen opnamebron te kiezen, dus dit is onvermijdelijk. Om het te wijzigen: kies een geluidskaart, of zet de standaard van de machine terug (`wpctl set-default <name>`) en zoek opnieuw.

settings-audio-currently-chosen = ⚠  Momenteel gekozen: { $device }

knowncalls-no-table = Deze modus houdt geen roepnaamtabel bij. Het is een FT8/FT4-decoder die roepnamen hasht — voor al het andere is hier niets te tonen.

knowncalls-not-asked = Nog niet opgevraagd — druk op VERVERSEN.

knowncalls-nobody = Nog niemand. Een station wordt toegevoegd zodra het zijn roepnaam uitspreekt — een CQ, een rapport met beide roepnamen in klare tekst, of een kale 11 m-roep.

knowncalls-title = BEKENDE STATIONS

knowncalls-intro = Elke roepnaam die een gehashte <...> op deze ontvanger op dit moment kan benoemen. Een hash kan niet worden teruggelezen — een station verschijnt hier pas nadat het zijn roepnaam heeft uitgesproken, waar een kale 11 m-roep voor dient.

knowncalls-find-hint = roepnaam, land, prefix…

knowncalls-refresh = VERVERSEN

knowncalls-capped = nieuwste { $shown } van { $total } — VERVERSEN om opnieuw te vragen, de lijst is een momentopname

knowncalls-known = { $shown } bekend

knowncalls-matches =
    { $n } { $n ->
        [one] treffer
       *[other] treffers
    }

knowncalls-no-match = Geen bekend station komt hiermee overeen.

knowncalls-no-country = geen land — geen roepnaam die deze tabel kent

classic-controls = BEDIENING

classic-fold-the-column-away-to-a-rail-on = Klap de kolom weg naar een balk links. De uitlezing en de meter blijven op de balk staan, en de knop om dit terug te halen zit op de balk.

classic-click-to-open-or-close-this = Klik om dit onderdeel te openen of te sluiten. Sleep het naar een ander onderdeel om het te verplaatsen — de volgorde wordt onthouden.

classic-bring-the-control-column-back = Haal de bedieningskolom terug.

classic-no-radio = geen radio

classic-rate = Snelheid

classic-dial = Dial

classic-receive = Ontvangen

classic-listening-screen = luisterscherm

classic-radio-settings = RADIO-INSTELLINGEN

classic-the-backend-the-sample-rate-the = De backend, de samplefrequentie, de antenne, de converter en de eigen kalibratie van deze radio — Instellingen → Radio, waar ze altijd al stonden. De kolom toont waarop ze staan in plaats van een tweede kopie van de bediening.

classic-counted-in-steps-not-decibels-what = Geteld in stappen, niet in decibel: wat één stap waard is, bepaalt de ontvanger zelf, en bij een SDRplay hangt dat ook van de band af. Rechts is nog steeds meer gain.

classic-the-plutosdr-s-own-agc-is-setting = De eigen AGC van de PlutoSDR stelt deze gain in, dus de schuifregelaar verplaatsen doet niets. Zet Instellingen → Radio → AGC op Handmatig om het hier te bedienen.

classic-squelch = Squelch

classic-gate-the-audio-below-this-power-in = Sluit de audio onder dit vermogen in de ontvangstdoorlaatband, in dBFS. Links is open. Stel het boven de ruis en onder het signaal in.

classic-this-radio-has-not-reported-what = Deze radio heeft nog niet doorgegeven wat ze kan — de verbinding is gesloten, of ze heeft niet geantwoord.

classic-open-the-link-in-the-radio-strip = Open de verbinding in de radiobalk, en de zendbediening verschijnt hier.

classic-this-device-receives-only-it-has = Dit apparaat ontvangt alleen: het heeft geen zender die dit programma kan keyen.

classic-transmit-is-offered-by-a-rig-with = Zenden wordt aangeboden door een rig met een besturings- of audiopad — Instellingen → Radio.

classic-transmit-is-off-for-this-radio-s = Zenden is uitgeschakeld voor het scherm van deze radio.

classic-settings-radio-show-transmit = Instellingen → Radio → "Zendbediening tonen" haalt het terug.

classic-key = Toets

classic-controlled-envelope-ssb-how-much = SSB met gestuurde omhullende: hoeveel van de spraakverwerking wordt toegepast. Omhoog is luider en vlakker, ten koste van wat splatter aan beide kanten.

classic-holding-the-squelch-open-click-to = De squelch wordt open gehouden. Klik om de eerdere drempel terug te zetten.

classic-click-to-hold-the-squelch-open-to = Klik om de squelch open te houden — om te horen of er iets is — en klik nogmaals om hem precies terug te zetten zoals hij was.

panels-received = Ontvangen

panels-text = Tekst

panels-hex = Hex

panels-nothing-received-yet = Nog niets ontvangen.

panels-payload = Payload

panels-blocks = Blokken

panels-sequence = Reeks

uvpacket-starting-receiver = UVPacket-ontvanger starten…

uvpacket-frames-count = { $n } frames

uvpacket-frames-count-hint = Frames gedecodeerd sinds de ontvanger startte.

uvpacket-intro = Een packetmodus voor besloten VHF/UHF-groepen. Stem de dial af op het kanaal van de groep; de submodus wordt per frame gedetecteerd.

uvpacket-nothing-yet = Nog niets. UVPacket is stil tussen uitzendingen — laat het op een kanaal staan en wacht op een burst.

uvpacket-app-seq = app { $app } · seq { $seq }

uvpacket-click-read-frame = Klik om het hele frame te lezen.

uvpacket-bps-net = { $bps } bps netto

uvpacket-sub-mode = Submodus

uvpacket-app-type = App-type

uvpacket-payload-bytes = { $n } bytes

dsc-starting-receiver = DSC-ontvanger starten…

dsc-seq-tone = { $n } seq · toon { $pct }%

dsc-seq-tone-hint = Reeksen die hun eindmarkering bereikten, en hoe zuiver de twee tonen te scheiden zijn.

dsc-nothing-yet = Nog niets. DSC is stil tussen schepen — laat het op een kanaal staan en wacht op een burst.

panels-marginal = marginaal

dsc-marginal-hint = Een teken faalde bij de BCH-controle — de getoonde velden kunnen onjuist zijn.

dsc-click-read-sequence = Klik om de hele reeks te lezen.

panels-marginal-caps = MARGINAAL

dsc-marginal-caps-hint = Minstens één teken faalde bij de BCH-controle. De reeks wordt getoond omdat een noodoproep die onvolmaakt aankwam toch het lezen waard is.

dsc-category = Categorie

dsc-nature = Aard

dsc-caller-mmsi = MMSI oproeper

dsc-target-mmsi = MMSI doel

dsc-alert-time = Alarmtijd

dsc-working-ch = Werkkanaal

dsc-raw-symbols = Ruwe symbolen

dsc-none = (geen)

settings-general-device-type-name = …of typ de naam — leeg is de systeemstandaard

settings-general-signed-in-station = Aangemeld station

settings-general-sign-out-hint = Vergeet de aanmelding van dit apparaat. Het station moet bij de volgende verbinding opnieuw vragen; deze blijft actief tot je hem sluit.

settings-general-recordings-remote = Opnames worden geschreven op de machine waaraan de radio is gekoppeld. Stel daar de map in.

settings-general-folder = Map

settings-general-choose = KIEZEN…

settings-general-choose-hint = Kies de map waar opnames naartoe worden geschreven

settings-general-default = STANDAARD

settings-general-default-hint = Terug naar de standaard — je muziekmap, of de configuratiemap als het systeem er geen heeft

settings-general-recordings-dir-note = Waar de audio-opnames, de geplande opnames en de ruwe I/Q-opnames naartoe worden geschreven. Standaard: de submap sdroxide van je muziekmap, of de map recordings van de configuratiemap als het systeem geen muziekmap heeft.

setup-suffix-cq = { " " }cq

setup-suffix-tries = { " " }pogingen

setup-window = venster

setup-south = zuid

setup-west = west

setup-message-templates = Berichtsjablonen  { $tokens }

setup-ft8-decoder-effort-hint =
    Hoe hard de FT8-decoder werkt voor zwakke signalen.
    
    Fast — één pass, geen signaalaftrek: snelst, minste decodes.
    Normal — vlakke multi-pass aftrek: ~0,4 s op een drukke slot, een paar decodes minder dan Deep.
    Deep — de checkpointed pass van WSJT-X: ~1,2 s en de meeste decodes (de standaard).
    
    Alleen de tweede reeks decodes wordt hierdoor bepaald; de gewone pass wordt altijd eerst uitgezonden, dus een antwoord wordt er nooit door opgehouden.

setup-auto-cq-repeats-hint = Hoeveel CQ-oproepen één run doet voordat hij eindigt. 0 is geen eigen limiet: de zendwatchdog en de inactiviteitsstop begrenzen de run nog steeds.

setup-auto-cq-then-listen-hint = Schakel na die CQ-oproepen over van AUTO CQ naar AUTO RX en blijf luisteren in plaats van helemaal te stoppen. Uit, dan stopt de run.

setup-auto-call-retries-hint = Hoe vaak één station wordt aangeroepen voordat de run het opgeeft en doorgaat naar de volgende in de wachtrij.

setup-auto-session-hint = Hoe lang een onbeheerde run mag duren, ongeacht waarmee hij bezig is. 0 is geen eigen limiet.

setup-serial-contests-hint = Contesten met serienummer — ARRL RTTY Roundup en CQ WPX via FT8: een signaalrapport en een serienummer (of een Amerikaanse staat), zonder locator. CQ gaat uit als CQ RU.

setup-tx-level-fm-hint =
    Hoe luid de burst van deze modus wordt doorgegeven aan een radio die zelf moduleert — en bij FM is dat de deviatie, omdat een FM-zender audioniveau omzet in frequentiezwaai en geen ALC heeft om dat op te vangen. 1200 baud packet wil ongeveer 3 kHz waar spraak 5 wil, dus vol bereik op een data-ingang die voor spraak is ingesteld geeft overdeviatie — wat voor een luisteraar volkomen normaal klinkt en voor niemand decodeert. Zet het zachter totdat andere stations je rapporteren, of stel het niveau in op de radio.
    
    Dit niveau hoort bij alleen deze modus. Een modus die je nooit hebt ingesteld begint bij de FM-waarde, en de sideband-modi beginnen bij hun eigen waarde — dus een deviatie die voor 1200 baud is ingesteld belandt nooit bij FT8.

setup-tx-level-ssb-hint =
    Hoe luid de over van deze modus wordt doorgegeven aan een radio die zelf moduleert — een CAT-rig op zijn geluidskaart, een FLEX, een Icom op zijn netwerkpoort. Bij sideband is dit de aansturing van de modulator: zet het zachter tot de ALC van de rig nauwelijks beweegt en stel het vermogen in op de radio, want ALC die op een modus met constante omhullende rijdt, veroorzaakt splatter. Drive bereikt hier het vermogensregister van de rig, niet zijn audio, dus dit is het niveau.
    
    Dit niveau hoort bij alleen deze modus: FT8, RTTY, PSK en MCW houden elk hun eigen niveau, omdat ze een modulator niet op dezelfde manier belasten. Dezelfde regelaar staat op de zendbalk, waar je erbij kunt terwijl je zendt.

setup-repeats-hint-base = Onbeantwoorde oproepen aan één station voordat wordt doorgegaan. CQ roepen is uitgezonderd.

setup-repeats-hint-duration = { " " }Bij de slotlengte van { $mode } ({ $slot } s) is dat ongeveer { $mins } min.

setup-repeats-hint-disables = { " " }0 schakelt het uit.

setup-repeats-hint-11m = { " " }Op 11 m beëindigt dit ook een CQ-run: de zendwatchdog onderbreekt er geen, omdat een station dat een CQ beantwoordt binnen ongeveer 30 s verwacht wordt en een klok een jonge run niet van een oude kan onderscheiden.

setup-symbol-home-station = Thuisstation

setup-symbol-car = Auto

setup-symbol-truck = Vrachtwagen

setup-symbol-van = Bestelwagen

setup-symbol-motorcycle = Motorfiets

setup-symbol-bicycle = Fiets

setup-symbol-person-on-foot = Persoon te voet

setup-symbol-motorhome = Camper

setup-symbol-boat = Boot

setup-symbol-yacht = Jacht

setup-symbol-light-aircraft = Licht vliegtuig

setup-symbol-balloon = Ballon

setup-symbol-digipeater = Digipeater

setup-symbol-i-gate = I-gate

setup-symbol-repeater = Repeater

setup-symbol-weather-station = Weerstation

setup-symbol-hospital = Ziekenhuis

setup-symbol-portable-campsite = Portable / kampeerterrein

settings-general-sign-out = AFMELDEN

settings-general-fallback-password = Een wachtwoord wordt in deze browser onthouden voor { $stations }, omdat het geen aanmeldcookie heeft teruggegeven. Oudere stations doen dit; het is niet hetzelfde.

settings-general-device-hint-speakers = De exacte naam waarmee je luidsprekers worden geopend. Kies hem uit de lijst als hij er staat; typ hem als de lijst hem niet toont — een geluidsserver kan een apparaat aanbieden dat de host-API niet opsomt. Leeg gebruikt de systeemstandaard.

settings-general-device-hint-microphone = De exacte naam waarmee je microfoon wordt geopend. Kies hem uit de lijst als hij er staat; typ hem als de lijst hem niet toont — een geluidsserver kan een apparaat aanbieden dat de host-API niet opsomt. Leeg gebruikt de systeemstandaard.

settings-general-device-hint-radio-rx = De exacte naam waarmee de ontvangstaudio van deze radio wordt geopend. Kies hem uit de lijst als hij er staat; typ hem als de lijst hem niet toont — een geluidsserver kan een apparaat aanbieden dat de host-API niet opsomt. Leeg gebruikt de systeemstandaard.

settings-general-device-hint-radio-tx = De exacte naam waarmee de zendaudio van deze radio wordt geopend. Kies hem uit de lijst als hij er staat; typ hem als de lijst hem niet toont — een geluidsserver kan een apparaat aanbieden dat de host-API niet opsomt. Leeg gebruikt de systeemstandaard.

settings-general-device-hint-other = De exacte naam waarmee het apparaat wordt geopend. Kies hem uit de lijst als hij er staat; typ hem als de lijst hem niet toont — een geluidsserver kan een apparaat aanbieden dat de host-API niet opsomt. Leeg gebruikt de systeemstandaard.

settings-general-cb-world = Wereld / freeband — 80 kanalen

settings-general-cb-cept = CEPT / EU — 40 kanalen

settings-general-cb-germany = Duitsland — 80 kanalen

settings-general-cb-uk = Verenigd Koninkrijk — 27/81, 40 kanalen

settings-general-cb-us = Verenigde Staten — 40 kanalen

settings-general-cb-australia = Australië — 40 kanalen

settings-general-cb-pmr446 = PMR446 — 16 kanalen (446 MHz)

settings-general-cb-dpmr446 = dPMR446 — 32 kanalen (446 MHz)

wefax-receiving-progress = ontvangen { $min } min · { $lines } regels

wefax-listening = luisteren

wefax-status-hint =
    Wachten op een kaart.
    
    WEFAX is een dienstregeling, geen gesprek: een station zendt zijn kaarten op vaste tijden uit, dus een gekozen frequentie is het grootste deel van de dag stil — dat is normaal. Gebruik de knop STATIONS om er een te kiezen en de tijd te controleren.
    
    Als er een signaal is maar het beeld één vlakke toon wordt (alles grijs, zwart of wit), ligt het aan de afstemming of de snelheid — gebruik de +0 Hz-uitlezing en de IOC/snelheidsregeling, en controleer of de STATIONS-invoer overeenkomt met wat er wordt uitgezonden.

wefax-centre = CENTREREN

wefax-centre-hint = Verschuif de dial { $sign }{ $off } Hz zodat de subdraaggolf weer op 1900 Hz zit — de ontvanger zit { $off } Hz ernaast, waardoor de kaart een witte pagina wordt.

wefax-auto-start-hint = Begin een kaart wanneer de starttoon van 300 Hz (IOC 576) of 675 Hz wordt gehoord, of, als de toon in een fade of onder een draaggolf verloren ging, wanneer enkele seconden van het fasesignaal binnenkomen — en beëindig de lopende kaart en fase de nieuwe wanneer de volgende uitzending aankomt, ongeacht of er eerst een stoptoon kwam. Een draaggolf op het kanaal start niets.

wefax-fit = PASSEND

wefax-save-chart-hint = Sla de kaart op het scherm op deze computer op, voor zover die is aangekomen

wefax-waiting-for-chart = Stem een faxschema af in USB en wacht tot de kaart begint (starttoon of fasesignaal), of druk op START om midden in de kaart te beginnen.

wefax-carrier-dial-note = draaggolf { $carrier } · dial { $dial } kHz (USB −1,9k)

wefax-carrier-note-hint = De stationschip noemt de gepubliceerde draaggolf; de radiofax-subdraaggolf zit 1900 Hz erboven, dus de dial staat bewust 1,9 kHz lager. Stem je de dial op de draaggolf zelf af, dan valt het signaal buiten de doorlaatband.

contest-title = CONTEST

contest-intro = Kies een contest en voer in wat je verzendt. De logger werkt op elke modus — typ de uitwisseling voor CW of SSB, en de FT8-kant vult hem zelf in waar de contest een FT8-opzet heeft.

contest-you-send = { "  " }jij zendt: { $exchange }

contest-they-send = { "  " }zij zenden: { $exchange }

contest-cb-note = CB / 11 m-activiteit: een rapport en een vrije-tekstuitwisseling — zet het kanaal, de naam of het gebied dat de activiteit gebruikt erin.

contest-your-exchange = Jouw uitwisseling

contest-start = START

contest-score-line = QSO { $qsos } · { $points } pts · { $mults } mult · SCORE { $total }

contest-rate = { $r10 } in 10 min · { $r60 }/uur

contest-stop = STOP

contest-estimate = score is een schatting — de organisator beslist

contest-call = ROEPNAAM

contest-dupe = DUBBEL

contest-sent = ZEND

contest-rcvd = ONTV

contest-sending = verzenden { $rst }

contest-sending-ours = verzenden { $rst } { $ours }

contest-log = LOG

playback-radio-n = { " " }· radio { $n }

playback-no-stamp = geen tijdstempel in de naam — speelt af in de modus waarin je zit

playback-title = AFSPELEN — opgenomen audio

playback-find-hint = modus, MHz, datum, naam…

playback-refresh-hint = Lees de opnamemap opnieuw

playback-asking = Het station om zijn opnames vragen…

playback-folder = Map: { $dir }

playback-none = Nog geen opnames. Druk op REC → MP3 of WAV terwijl er iets in de lucht is; het bestand verschijnt hier, genoemd naar tijd, frequentie en modus.

playback-on = ▶ AAN

playback-play = ▶ AFSPELEN

playback-iq-hint = Een I/Q-opname, geen audio. Afspelen neemt de audio die de recorder schrijft; een I/Q-opname wordt afgespeeld met --file.

playback-not-while-tx = Niet tijdens zenden

playback-play-hint = Speel deze opname vanaf het begin af in de decoders

playback-busy-playing = Wordt afgespeeld — stop het voordat je het verwijdert

playback-busy-recording = Wordt nog opgenomen — stop REC voordat je het verwijdert

playback-delete-ask = WISSEN?

playback-delete-hint-armed = Druk nogmaals om dit bestand definitief te wissen. Het kan niet ongedaan worden gemaakt.

playback-delete-hint = Wis deze opname uit de map (vraagt eerst nog één keer)

playback-strip-label = ▶ AFSPELEN

playback-not-live = niet live — de radio staat stil

playback-carry-on = Verder afspelen

playback-pause = ⏸ PAUZE

playback-pause-hint = Pauze — de decoders horen stilte totdat je verdergaat

playback-again = ↺ OPNIEUW

playback-again-hint = Terug naar het begin — nog een poging voor een beeld of een decode

playback-loop = LUS

playback-loop-hint = Begin opnieuw bovenaan wanneer het bestand eindigt

playback-stop = ■ STOP

playback-stop-hint = Beëindig het afspelen en zet de radio terug waar hij was

playback-files = BESTANDEN

playback-end-of-file = einde bestand

playback-paused = gepauzeerd

playback-recorded-on-mode = Opgenomen op { $mhz } MHz in { $mode } — de dial en modus zijn ingesteld op basis van de bestandsnaam.

playback-recorded-on = Opgenomen op { $mhz } MHz — de dial en modus zijn ingesteld op basis van de bestandsnaam.

playback-no-dial-mode = De naam bevat geen dial of modus, dus het speelt af in de modus waarin je zit — kies de decoder in het modusmenu.

playback-format-stereo = { $rate } Hz, stereo — het linker (ontvangst)kanaal wordt afgespeeld.

playback-format-mono = { $rate } Hz, mono — het enige kanaal wordt afgespeeld.

playback-stopped = Gestopt: { $fault }

playback-intro = Speel een opname terug in de decoders — SSTV, CW, RTTY, PSK, FT8 en elke andere modus die uit audio decodeert. De radio staat stil tijdens het afspelen, de dial en modus gaan naar waarop de opname is gemaakt, en STOP zet de radio terug. Wijzig de modus tijdens het afspelen om een andere decoder te proberen.

packet-callsign-hint = roepnaam

packet-via-hint = via

aprs-filter-hint = filter

aprs-msg-sent = verzonden

aprs-msg-ack = ack

aprs-msg-rejected = afgewezen

aprs-reply-chip = antwoord

aprs-to-hint = aan

aprs-message-hint = bericht

aprs-raw-chip = RAW

fsq-to-button = AAN

fsq-del-chip = WIS

macros-column-key = toets

macros-column-button = knop

macros-column-sends = verzendt

macros-label-hint = label

macros-add = TOEVOEGEN

macros-note-placeholders = { $mycall } en { $mygrid } worden ingevuld zodra het bericht uitgaat.

macros-buttons-note = Elke knop verzendt zijn hele tekst in één keer. F1–F9 drukken de eerste negen in, zolang niets op het scherm het toetsenbord heeft.



widgets-worked-before-any = Eerder gewerkt

vdl2-col-sig = SIG

vdl2-col-age = LEEFT

vdl2-command = commando

vdl2-response = antwoord

vdl2-frame-hint = frame

vdl2-suffix-messages = { " " }berichten

vdl2-suffix-stations = { " " }stations

ale-words-count = { $n } woorden

ale-waiting-audio = Wachten op audio…

ale-no-words-yet = Nog geen woorden. Stem af op een ALE-kanaal (bijv. 8992 of 11175 kHz USB) en wacht op een oproep.

atchat-role-master = MASTER

atchat-role-backup = BACKUP

atchat-me = ik

dab-sub-channel = { " " }· sub { $n }

dab-starting-receiver = DAB-ontvanger starten…

dab-no-ensemble-yet = nog geen ensemble benoemd — is er een DAB-uitzending op dit kanaal?

dab-window-stats = { $mhz } MHz · { $frames } frames · { $fibs } fibs

dab-volume-meter-hint = Het station zoals je het hoort, links en rechts — en het volume: sleep of klik langs de balken, of gebruik het wiel. Klik erop, dan stappen de pijltoetsen erdoorheen. Werkt op een scherm dat met het hangslot is vergrendeld.

rade-searching = zoeken

rfpaint-swl-note = RF Paint is een modus alleen voor zenden — niets te tonen in SWL.

cw-hand-key-arm-hint = Schakel de keyer in en gebruik de USB-paddle. De contacten gaan naar de engine, die de dits en dahs op de zendsnelheid maakt — hier, niet in het paneel, zodat de timing niet op een frame wordt afgerond. Het zendvak is vergrendeld zolang dit aan staat en tekst in de wachtrij wordt verwijderd. Een straight key in hetzelfde vak werkt ook: kies het type Straight key.

cw-own-keyer-hint =
    Deze radio zendt vanuit zijn eigen keyer: de tekst gaat over de besturingspoort en de rig bepaalt de timing van de elementen, dus er is niets tussen de hand en de lucht dat een key kan aansturen — een paddle evenmin als een straight key.
    
    Om met de hand te keyen, zet je CW-keying op "Geluidskaart (MCW)" in Instellingen → Radio. De rig blijft dan op een zijband staan en de sidetone van de keyer wordt als audio uitgezonden, wat de route is die de straight key aanstuurt.

cw-no-paddle-found = geen paddle gevonden — kies er een in Instellingen → CW

cw-msg-hint = Je eigen berichtknoppen — een contestuitwisseling, een antwoord met naam en QTH, TNX 73 GL. Elke knop verzendt zijn hele tekst in één keer, en F1–F9 drukken de eerste negen in. Ze reizen mee met de configuratie van het station, dus een remote client heeft ze ook.

wspr-clear-rx = RX WISSEN

wspr-rx-count = { $n } rx

wspr-sort-dist = AFST

wspr-sort-country = LAND

wspr-tx-enable = TX AAN

wspr-stop-tx = STOP TX

wspr-listening-as = Luisteren als { $call } in { $sent } — in te stellen op het tabblad Algemeen van Instellingen.

wspr-set-callsign-grid-label-spots = Stel je roepnaam en grid in op het tabblad Algemeen van Instellingen om je spots te labelen.

schedule-source-mine = mijn

schedule-all-of-them = allemaal

schedule-title = PROGRAMMA'S

schedule-at = Om

schedule-now-chip = NU

schedule-set-time = tijd instellen

schedule-find-hint = BBC, 6185, 15.4 MHz…

schedule-target = Doel

schedule-any = elk

schedule-source = Bron

schedule-all-hint = Elk gedownload schema in één opslag. Een station dat in twee ervan staat, verschijnt twee keer, wat het detail is waarvoor dit venster dient — de panadapter toont één naam.

schedule-favs = ★ FAVO'S

schedule-favs-hint = Alleen de stations die je met een ster hebt gemarkeerd

schedule-all-favs = ⭐ ALLES

schedule-all-favs-hint = Maak van elk schemastation een favoriet en toon hun labels op de waterfall zolang dit aan staat. Klik opnieuw om alle favorieten te wissen.

schedule-solar-time = LOKALE TIJD

schedule-solar-time-hint = Toon de lokale tijd van elke rij bij de zender. Het is de klok van de zon — vier minuten per graad lengte — en geen burgerlijke tijdzone: zomertijd en zonegrenzen kent het niet.

schedule-map = KAART TONEN

schedule-map-hint = Open LISTEN — dit schema als kaart. Kies er een station op, schrijf op wat je hoorde en log het. Het geselecteerde station komt mee

schedule-on-air = { $count } in de lucht om { $time } UTC

schedule-now = nu

schedule-solar-hover = { $local } lokale zonnetijd bij { $site }, { $lon }°

schedule-favourite-hint = Maak dit station favoriet

rc-radio-classic = RADIO CLASSIC

rc-bands = Banden

rc-mem-programs = Geh · Programma's

rc-recall-band-mode-frequency-and = Oproepen: band, modus, frequentie en filter ineens

rc-no-memories-yet-store-one-with-mem = nog geen geheugens — sla er een op met MEM in de bovenbalk

rc-broadcast-schedule-what-is-on-when = Omroepschema — wat er wanneer en waar is

rc-bw-this-mode-has-one-fixed = BW — deze modus heeft één vaste doorlaatband

rc-bw-click-next-filter-right-click = BW — klik: volgend filter · rechtsklik: vorig · wiel: stap

rc-step = STAP

rc-step-click-next-tuning-step-right = STAP — klik: volgende afstemstap · rechtsklik: vorige · wiel: stap

rc-agc-click-next-speed-right-click = AGC — klik: volgende snelheid · rechtsklik: vorige · wiel: stap

rc-open = OPEN

rc-sql-click-open-back-to-the-last = SQL — klik: open / terug naar de laatste drempel · wiel: de poort verhogen of verlagen

rc-s-meter = S-METER

rc-wheel-on-a-digit-click-upper-lower = wiel op een cijfer · klik bovenste/onderste helft ± · rechtsklik nul · dubbelklik om te typen

rc-music = MUZIEK ♪

rc-speech = SPRAAK

rc-stereo-the-19-khz-pilot-is-locked = Stereo — de 19 kHz-pilot is vergrendeld — klik voor mono

rc-mono-the-broadcast-stereo-decode = Mono — de stereodecodering van de omroep is uitgeschakeld — klik om in te schakelen

rc-mono-noise-reduction-is-on-and = Mono — ruisonderdrukking staat aan, en stereo wijkt ervoor

rc-mono-no-19-khz-pilot-the-station = Mono — geen 19 kHz-pilot: het station zendt mono, of het signaal is te zwak

rc-stereo = STEREO

rc-group-broadcast = OMROEP

rc-group-ham = HAM

rc-group-air = LUCHT

rc-group-marine = MARIEN

rc-cap-lw-mw-fm = LG / MG · FM

rc-cap-shortwave = KG · KORTE GOLF

rc-cap-amateur-bands = AMATEURBANDEN

rc-cap-airband = LUCHTVAARTBAND

rc-cap-marine = MARIFOON VHF · MF

rc-cap-cb = CB-BAND

rc-preset-fm-ext = FM ext 64–87.5

rc-mem-n = MEM { $id }

controls-action-tune = VFO afstemmen

controls-action-rit-offset = RIT-offset

controls-action-xit-offset = XIT-offset

controls-action-volume = Volume

controls-action-sub-volume = Sub-RX-volume

controls-action-squelch = Squelch

controls-action-filter-width = Filterbreedte

controls-action-filter-shift = Filterverschuiving

controls-action-agc-max-gain = AGC max gain

controls-action-manual-gain = Handmatige gain (AGC uit)

controls-action-tx-drive = TX-drive

controls-action-tune-drive = Tune-drive

controls-action-mic-gain = Microfoongain

controls-action-digi-audio-freq = Digi-audio-offset

controls-action-spectrum-zoom = Spectrumzoom

controls-action-spectrum-pan = Spectrum verschuiven

controls-action-spectrum-floor = Spectrumbodem

controls-action-spectrum-ceiling = Spectrumplafond

controls-action-ptt = PTT

controls-action-tune-carrier = TUNE-draaggolf

controls-action-mute = Dempen

controls-action-noise-blanker = Noise blanker

controls-action-nr-cycle = Ruisonderdrukking (wisselen)

controls-action-auto-notch = Auto-notch

controls-action-binaural = Binaurale audio

controls-action-agc-cycle = AGC (wisselen)

controls-action-sub-rx = Subontvanger

controls-action-split = Split

controls-action-rit-enable = RIT aan/uit

controls-action-xit-enable = XIT aan/uit

controls-action-rit-clear = RIT wissen

controls-action-xit-clear = XIT wissen

controls-action-vfo-toggle = VFO A/B wisselen

controls-action-swap-vfos = Wissel A↔B

controls-action-copy-a-to-b = Kopieer A→B

controls-action-band-up = Band omhoog

controls-action-band-down = Band omlaag

controls-action-mode-next = Volgende modus

controls-action-mode-prev = Vorige modus

controls-action-record-toggle = Opnemen aan/uit

controls-action-abort-tx = Zenden afbreken

controls-action-cw-straight = CW straight key

controls-action-voice-stop = Voice keyer stoppen

controls-action-tone-burst = 1750 Hz toneburst

controls-action-fit-span = Span passend maken

controls-action-zoom-in = Inzoomen

controls-action-zoom-out = Uitzoomen

controls-action-peak-hold = Peak hold

controls-action-spectrum-collapse = Spectrum inklappen

controls-action-waterfall-collapse = Waterval inklappen

controls-action-waterfall-flip = Waterval omkeren

controls-action-toggle-help = Handleidingvenster

controls-action-toggle-settings = Instellingenvenster

controls-action-toggle-logbook = Logboekvenster

controls-action-toggle-spots = Spotsvenster

controls-action-toggle-memories = Geheugenvenster

controls-action-toggle-voice = Voice-keyervenster

controls-action-workspaces = Werkruimtevenster

controls-action-speak-status = Status uitspreken

controls-action-speak-repeat = Laatste melding herhalen

controls-action-speech-silence = Spraak stoppen

controls-action-speech-toggle = Meldingen aan/uit

controls-action-broadcast-labels = Omroeplabels (afgestemd / hele band)

controls-action-select-vfo = VFO { $vfo } kiezen

controls-action-band = Band { $band }

controls-action-mode = Modus { $mode }

controls-action-recall-memory = Geheugen { $n } oproepen

controls-action-voice-slot = Voice-keyerslot { $n }

controls-group-tuning = Afstemming

controls-group-receive = Ontvangen

controls-group-transmit = Zenden

controls-group-display = Weergave

controls-group-windows = Vensters

controls-group-speech = Spraak

controls-midi-note = Noot

controls-midi-cc = CC

controls-midi-pitch-bend = Pitch bend

controls-midi-program = Programma

controls-midi-any = elke

controls-midi-msg = { $kind } { $data }  ch { $ch }

controls-midi-msg-pitch-bend = Pitch bend  ch { $ch }

settings-controls-down = omlaag

settings-controls-rev = omk.

antenna-antenna = ANTENNE

antenna-cutting-lengths = Snijlengtes

antenna-the-frequency-to-cut-for = De frequentie waarvoor gesneden wordt

antenna-optimizing = Optimaliseren…

antenna-tune-the-yagi-s-element-lengths = Stem de elementlengtes en tussenruimtes van de Yagi af op het gekozen doel. Een lokale hill-climb op een achtergrondthread; het venster blijft bruikbaar terwijl het loopt.

antenna-solved = Opgelost

antenna-optimized-yagi = Geoptimaliseerde Yagi

antenna-obstacles-approximate = Obstakels (bij benadering)

antenna-trees = Bomen

antenna-in-leaf = met blad

antenna-ridge = Bergrug

dab-channel-received-mhz = Kanaal ontvangen { $channel } { $mhz }

dab-no-channel-received = Geen kanaal ontvangen

dab-channel-received = Kanaal ontvangen { $channel }

dab-slideshow = DIAVOORSTELLING

dab-no-picture-yet = de zender heeft nog geen beeld gestuurd

dab-no-station-chosen = geen zender gekozen

dab-no-logo-yet = nog geen logo

dab-playing = ▶ speelt

dab-chosen = gekozen

dab-no-text-yet = de zender heeft nog geen tekst gestuurd

dab-clear-channel-hint = Wis dit kanaal en zijn zenders. SCAN vindt het weer terug.

dab-clear-all-channels = × alle kanalen wissen

dab-clear-all-channels-hint = Vergeet elk kanaal en de zenders erop. SCAN vindt ze weer terug; de sterren blijven.

dab-n-stations = { $n } zenders

dab-scanning-progress = scannen { $channel } · { $at }/{ $n } · { $found } gevonden

dab-clear-station-hint = Wis deze zender uit de lijst. De volgende SCAN brengt hem terug als het ensemble hem nog bevat.

dab-prev = VORIGE

dab-prev-hint = De vorige zender in de lijst.

dab-preset = PRESET

dab-preset-hint = Geef deze zender een ster, of haal de ster weg.

dab-next = VOLGENDE

dab-next-hint = De volgende zender in de lijst.

dab-pty-hint = Toon het programmatype van elke zender in de lijst.

dab-stop-sweep-hint = Stop de sweep hier. Wat al gevonden is, blijft staan.

dab-scan-hint = Loop elk Band III-blok af en bewaar de blokken met zenders.

swl-title = SWL-LOG

swl-new = + NIEUW

swl-jobs = TAKEN

swl-jobs-hint = Geplande opnames — neem een band op een vast tijdstip op

swl-sigid = SIGNAAL-ID

swl-sigid-hint = Wat staat er op deze frequentie? Een gids voor signalen op frequentie, modus en bandbreedte

swl-replay = HERHAAL

swl-replay-hint = Speel de laatste twee minuten af in plaats van live — vang de zenderidentificatie die je net miste

swl-report = RAPPORT

swl-report-hint = Kopieer deze ontvangst als rapport om naar de zender te sturen

swl-stats-heard = { $n } gehoord

swl-stats-reported = { $n } gerapporteerd

swl-stats-qsl = { $n } QSL

swl-stats-pirates = { $n } piraten

swl-stats-hint = Het hele log — de Toon-filters veranderen dit niet

swl-antenna = Antenne

swl-antenna-hint = Langdraad 20 m, MLA-30-loop, mini-whip …

swl-antenna-tip = Komt op de regel Antenne: van het ontvangstrapport

swl-tone = Toon

swl-tone-on = aan

swl-tone-bass = Bas

swl-tone-mid = Midden

swl-tone-treble = Hoog

swl-report-copied = Rapport naar het klembord gekopieerd — plak het in je mailprogramma. Er kon geen mailprogramma worden geopend.

swl-show = Toon

swl-find-hint = zender, taal, locatie …

swl-find-tip = Komt overeen met zender, taal, zenderlocatie en notities

swl-all-bands = Alle banden

swl-all-days = Alle dagen

swl-pirates-only = Alleen piraten

swl-pirates-only-tip = Alleen de ongelicentieerde vangsten

swl-adif-tip = Sla het hele log op als ADIF — ontvangstrecords, geen contacten

swl-csv-tip = Sla het hele log op als CSV, één rij per ontvangst

swl-shown-of = { $shown } van { $n }

swl-empty = Nog niets gelogd — stem af op een zender en druk op + NIEUW.

swl-transmitted-from = uitgezonden vanuit { $site }

swl-click-to-copy = (klik om te kopiëren)

swl-addr-btn = adres

swl-mail-btn = mail

swl-mark-sent = verzonden

swl-recall = oproep

swl-recall-tip = Stem terug af op deze frequentie en modus — kijk of de zender terug is

swl-edit = bewerk

swl-del = wis

swl-no-match = Geen ontvangst komt overeen met het filter — WISSEN toont ze allemaal.

swl-form-new = NIEUWE ONTVANGST

swl-form-edit = ONTVANGST BEWERKEN

swl-station-hint = Radio Taiwan International

swl-language-hint = Engels

swl-received = Ontvangen

swl-received-hint = jouw grid

swl-email = E-mail

swl-email-hint = ontvangstrapporten

swl-email-tip = Komt op de regel Verzenden naar: van het rapport

swl-postal = Post

swl-postal-hint = straat, plaats, land

swl-postal-tip = Het terugvaladres voor het rapport als er geen e-mail is

swl-heard = Gehoord

swl-smeter = S-meter

swl-report-label = Rapport

swl-judged = geschat

swl-report-sent = Rapport verzonden

swl-not-yet = nog niet

swl-qsl-received = QSL ontvangen

swl-awaiting = in afwachting

swl-notes = Notities

swl-notes-hint = programmanotities

swl-pirate = Piraat

swl-pirate-tip = Een ongelicentieerde uitzending — een zender die buiten elke toewijzing zendt. Het log markeert hem met een piratenvlag.

swl-mail-why-no-address =
    Deze zender heeft geen e-mailadres — zoek het op in het SCHEDULE-venster, of typ er hier een, en dit wordt beschikbaar.

    Een ontvangstrapport is een beleefdheid die een zender leest en beantwoordt met een QSL-kaart; zonder adres is er nergens om het naartoe te sturen.

swl-mail-why-bad-address = Dat e-mailadres is niet één adres, dus het wordt niet gebruikt — een rapport dat bij een vreemde terechtkomt die de zender nooit heeft gehoord, is erger dan geen rapport.

swl-mail-why-ok =
    Schrijf het ontvangstrapport en open het in je mailprogramma.

    De afbeelding en het geluidsfragment voeg je daar zelf toe — dit programma kan geen bestand aan een bericht toevoegen.

swl-mail-report = RAPPORT MAILEN…

antenna-feed-ohms = voeding ≈ { $r } Ω

antenna-takeoff = takeoff { $deg }°

antenna-director = director { $n }

antenna-reflector = reflector

antenna-driven = gevoed element

antenna-gap = tussenruimte { $v } m

antenna-az = az

antenna-depth = diepte

antenna-height = hoogte

antenna-attenuation = demping op de 0°-peiling: { $att } dB

morse-no-paddle-found-connect-one-then = geen paddle gevonden — sluit er een aan en open dit paneel opnieuw

morse-send-with-a-paddle-played-here = Zend met een paddle — hier afgespeeld, nooit in de ether

morse-no-paddle-found-in-dev-input-by-id = Geen paddle gevonden in /dev/input/by-id. Sluit er een aan en open dit paneel opnieuw. Een keyerbox die zijn twee contacten meldt (vaak als de knoppen van een muis) werkt; een box die zelf iambisch keyt en aanrakingen verstuurt, werkt niet.

morse-reverse = OMKEREN

morse-swap-dit-and-dah-the-switch-on = Verwissel dit en dah. De schakelaar op veel paddles doet dit op de sleutel zelf; dit is voor als die in de andere stand staat.

morse-stop-key = STOP SLEUTEL

morse-start-key = START SLEUTEL

morse-key-the-character-above-the-keyer = Seinen van het teken hierboven. De keyer maakt de dits en dahs zuiver — jij kiest welke paddle en hoe lang de pauze is. Een tik is één element; ingedrukt houden herhaalt.

morse-the-sending-drill-needs-a-linux = De zendoefening vereist een Linux-desktop en een paddle.

panels-float-chip-hint = ZWEVEND — til dit paneel los als zwevend paneel en geef de waterval zijn ruimte terug; de dock-chip erop zet het terug

panels-map-style-radar = Radar — de donkere ondergrond, land gevuld. Waartegen een luchtbeeld wordt gelezen

panels-map-style-terrain = Terrein — land verhoogd en warmer gemaakt zodat het als grond leest, grenzen gedimd

panels-map-style-chart = Kaart — bleek land, geaccentueerde grenzen en een graticule, de luchtvaartkaartlook

panels-heard-me-chip = HOORDEN MIJ

panels-heard-me-hint = Omring elk station dat meldde deze roepnaam op de huidige band te horen, volgens PSK Reporter, geplaatst op het eigen grid van de melder. Het omgekeerde van de PSK-spots: dat zijn stations die deze radio hoort. Elk uur ververst, over het laatste uur; vereist dat de PSK-feed aan staat onder Instellingen → Spots en een roepnaam op het tabblad Algemeen.

panels-period-label = periode

panels-sub-mode-label = submodus

panels-front-end-overloading = ⚠ FRONT-END OVERSTUURD — verlaag de RF-versterking (LNA/IF), of verplaats de antenne

panels-front-end-overloading-hint = De ontvanger wordt voorbij volle schaal aangestuurd, dus het geluid is hard maar bevat geen signaal — er wordt niets gedecodeerd zolang dit duurt. Zet de RF/LNA-versterking lager (meer demping), verlaag de IF-versterking, of schakel AGC in. Een sterke zender in de buurt, of een andere rig op dezelfde band, is de gebruikelijke oorzaak.

panels-enigma-hint = Enigma-machine — versleutel een bericht op een oorlogsfrontplaat en ontcijfer een cijfertekst die je uit de ether hebt gehaald

panels-slot-bar-heard-hint = Volgt het geluid in de luidsprekers: wit zolang een frame wordt gehoord, groen zodra het is gestopt, en terug naar het begin als het volgende begint. De tijden zijn die van de modus, dus een frame dat korter duurt vult toch de balk.


morse-send-correct = juist: { $want }

morse-send-heard = gehoord { $got }, gewenst { $want }

morse-send-received = ontvangen: { $text }

amp-title-model = AMP · { $model }

amp-state-off = UIT

amp-state-on = AAN

amp-relay-dn = DN

amp-relay-up = UP

amp-band-unknown = ONB

amp-needle-top = TOP

decodes-not-uploaded = ⚠ { $services } niet geüpload

decodes-not-uploaded-hint = Het contact staat in het lokale log, maar de upload is mislukt. Controleer het token en de verbinding onder Instellingen → Spots en upload het daarna opnieuw vanuit het logboek.

decodes-logged-to = ↑ gelogd naar { $services }

decodes-logged-to-hint = Ook geüpload naar de online logboeken die elk nieuw contact overnemen.

decodes-known = BEKEND

decodes-known-hint = Elke roepnaam die een gehashte <...> hier nu kan oplossen, met zijn land. Een hash kan niet teruggelezen worden — een station verschijnt pas nadat het zijn roepnaam heeft uitgespeld, en daar dient een kale 11 m-roepnaam voor.

decodes-ignored-clear = { $n } genegeerd · wissen

decodes-auto-tx-frequency-hint =
    Kies onze zendfrequentie automatisch: de rustigste plek in de periode waarin we zenden, in plaats van de frequentie van wie we beantwoorden — die zendt in de andere periode, dus die zegt niets over wie er zit als wij zenden. Uit houdt de frequentie NIET vast: dan antwoordt het op de frequentie van het station dat wordt aangeroepen. Gebruik Hold TX om vast te houden.

    Eén band is uitgezonderd: op 11 m is de WSJT-CB-norm om een CQ op de eigen toon van de aanroeper te beantwoorden en daar het hele contact te blijven, dus de zoektocht wordt overgeslagen en een CQ-antwoord blijft op de frequentie waarop het werd gehoord, of dit nu aan of uit staat.

decodes-muted-caps = GEDEMPT

decodes-muted-hint = Gedempt voor deze sessie — klik om het weer te horen

decodes-ignore-hint = Negeer dit station voor de rest van de sessie: het verdwijnt uit de decodelijst, geeft geen meldingen meer en wordt nooit aan de automodus aangeboden. Er wordt niets namens dit station verzonden of geüpload, en na een herstart van sdroxide is het terug.

decodes-qso-complete-hint = Het contact is compleet en staat in het lokale log. Het blijft hier een paar minuten staan zodat het laatste bericht opnieuw kan worden verzonden als het andere station het zijne herhaalt — er is verder niets verschuldigd.

decodes-slot-even = even

decodes-slot-odd = oneven

decodes-before = vóór

decodes-after = na

decodes-beam-hint = Richt de antenne op de locator van dit station

decodes-queue-next = volgende

decodes-queue-waiting = wachtend


decodes-setup-hint = SETUP — de stationsidentiteit en de sjablonen die deze QSO-modi delen

decodes-auto-armed-hint =
    { $role } is gewapend. Klik om te stoppen.

    { $note }

    Engine-stap: { $step } — een station dat een CQ beantwoordt wordt alleen aangenomen in CallingCq zonder lopend contact.

decodes-auto-rx-hint = Beantwoord de CQ van een nieuw station, onbemand, en roep zelf nooit CQ. Alleen roepnamen die nog niet in het log staan worden beantwoord. De zendwatchdog bepaalt het tempo en de inactiviteitsstop schakelt het uit.

decodes-auto-continuous-hint = De hele werkwijze, onbemand: rond de QSO af, werk wie je aanroept, beantwoord een station dat CQ roept, en roep alleen CQ als er niets anders is. Loopt tot je het stopt{ $until }. { $bound }

decodes-auto-or-minutes-up = { " " }of { $n } minuten voorbij zijn

decodes-auto-cq-run-unlimited = Elke CQ-reeks heeft geen eigen oproeplimiet.

decodes-auto-cq-run-bounded = Elke CQ-reeks wordt begrensd door een eigen aantal oproepen.

decodes-auto-no-call-limit = Geen eigen oproeplimiet — de watchdog en de inactiviteitsstop beëindigen de reeks.

decodes-auto-after-calls-listen = Na { $repeats } oproepen schakelt het over op AUTO RX en blijft luisteren.

decodes-auto-after-calls-stop = Na { $repeats } oproepen stopt het.

decodes-auto-cq-hint = Roep CQ onbemand en werk wie antwoordt. { $bound } Te kort en een stille band hoort je nooit; te lang en de zender zit op een frequentie die niemand beantwoordt.

decodes-auto-stopped-by-operator = { $role } gestopt door de operator

decodes-cont = vervolg

settings-servers-decoded-text-udp = Gedecodeerde tekst via UDP

settings-servers-decoded-text-udp-note = Eén JSON-regel per decode, voor een eigen script: de FT8-familie, WSPR en de gedecodeerde regels van elke toetsenbordmodus (CW, RTTY, PSK, Olivia, THOR, MFSK, DominoEX, Hell), elk met de dial waarop het werd gehoord. De WSJT-X-broadcast hierboven is een loggerprotocol en heeft geen bericht voor die modi, daarom bestaat dit apart.

settings-servers-decoded-text-udp-hint = host:poort, via UDP. Leeg is uit — de standaard. Alleen uitvoer: niets op deze socket kan de radio aanraken. Een naam wordt opgelost zodra je op TOEPASSEN drukt.

settings-servers-rotator-note = Richt een gemotoriseerde antenne op de satelliet waarop je bent vergrendeld, of met de hand vanuit het ROTATOR-venster en de kaarten. De standaard is een Hamlib rotctld-daemon (start bijv. “rotctld -m 603 -r /dev/ttyUSB0” voor een GS-232B-rotator, of “rotctld -m 1” om het zonder hardware te proberen) — één daemon dekt elke rotator die Hamlib aanstuurt. Een controller op een seriële poort kan ook rechtstreeks worden aangestuurd.

settings-servers-connection = Verbinding

settings-servers-rotctld-port-hint = 4533 is de standaard van rotctld

settings-servers-baud-hint = De meeste controllers draaien op 9600

settings-servers-pstrotator-note = De eigen UDP-poort van PSTRotator. Zet in de Setup UDP Control aan; het beantwoordt commando's op deze poort en antwoordt op de volgende.

settings-servers-el = el

settings-servers-persist-re-connect-rotator = Bewaar en (her)verbind met de rotator

settings-servers-tci-note = Laat TCI-geschikte programma's deze radio aansturen: frequentie- en modusbesturing, breedband-IQ, ontvangstaudio en zendaudio. Kies in WSJT-X het rigtype SunSDR (TCI), richt het op dit adres en zet PTT op TCI.

settings-servers-decoded-text-udp-format = Elke regel is één datagram, afgesloten met een nieuwe regel: { $example }

settings-servers-transport-rotctld = Hamlib rotctld (TCP)

settings-servers-transport-easycomm = EasyComm II (serieel)

settings-servers-transport-gs232 = Yaesu GS-232 (serieel)

settings-servers-transport-pstrotator = PSTRotator (UDP)

spots-active = ACTIEF

spots-closing = SLUITEND

spots-none-left-in-the-window = geen meer in het venster

spots-openings = OPENINGEN

spots-band-opening-detections-paths = Detecties van bandopeningen: paden waarvan de recente activiteit hun eigen basislijn van 3 uur oversteeg

spots-save-the-spots-you-can-see-as-a = Sla de zichtbare spots op als KML-bestand — Google Earth en elk ander kaartprogramma dat het leest. Alleen de ingeschakelde filters, en alleen de spots met een positie, want een speld zonder coördinaten is een speld aan het verkeerde eind van de wereld.

spots-no-spots-to-save-the-filters-are = Geen spots om op te slaan — de filters staan allemaal uit

spots-bc-all = BC ALLE

spots-bc-tuned = BC AFGESTEMD

spots-which-broadcasters-the-waterfall =
    Welke omroepen de **waterval** noemt.
    
    { "**" }BC AFGESTEMD** is de standaard en de regel van bebepip: de naam van het station waarop je bent afgestemd, en niets anders — een band die van begin tot eind is gelabeld is een muur van namen over de passband die je probeert te lezen.
    
    { "**" }BC ALLE** noemt elk station dat het schema nu in de lucht heeft, waar het ook zit, wat je wilt als je een band doorbladert in plaats van naar één station te luisteren. Het zegt niets over of de BC-chip hierboven omroepen überhaupt toont.

app-rx-only-hint = Deze radio kan alleen ontvangen — hij heeft geen zender om te keyen.

spots-n-calls = { $n } oproepen

spots-opening-hover = Bandpad { $band } van { $from } naar { $to }: het laatste venster van 15 minuten overschreed de basislijn van 3 uur van het pad — recente aanroepers: { $calls }


spots-download-unsupported = { $source } kon op dit platform niet worden gedownload

spots-kml-partial = { $placed } van { $total } opgeslagen — de rest heeft geen positie en kan niet worden vastgepind

spots-net-log = NET-LOG ({ $n })

decodes-beam = BEAM

settings-radio-no-answer-yet = Wachten op de machine waarop de radio is aangesloten: deze vragen gaan over zijn hardware en zijn netwerk, en hij beantwoordt ze één voor één.

settings-radio-elad-no-baud = de radio heeft geen instelling voor { $baud } baud — { $default } wordt in plaats daarvan gebruikt

settings-radio-elad-antenna-rx-only = Alleen RX

settings-radio-rx-only-n = RX{ $n } alleen

settings-radio-ng-level = nG-niveau

settings-radio-ng-level-hint =
    nG voegt twee commando's toe die de 2.00x-firmware niet heeft, en dit is de enige plek waar ze kunnen worden ingesteld: nG beantwoordt noch bewaart ze, dus sdroxide verstuurt ze opnieuw telkens als de verbinding opent.
    
    Volume is het niveau dat de radio over de USB-stroom stuurt. nG neemt die audio na VOL en AGC, dus dit is de juiste plek om een decoder te corrigeren die te weinig of te veel ziet — niet de mixer van de computer.
    
    De versterkingsregeling is de eigen AGC-instelling van de radio. Auto zet hem op DIGI voor elke digitale modus en anders aan, wat de notities van nG voor FT8 vragen: de versterking blijft dan constant tijdens de zendpauzes van het andere station in plaats van erin op te regelen.

settings-radio-ng-volume-hint = 0–31, als `AG0nn;`. Het niveau dat de radio naar de computer streamt.

settings-radio-gain-control = Versterkingsregeling

settings-radio-speaker = Luidspreker

settings-radio-on-while-streaming = Aan tijdens streamen

settings-radio-ng-speaker-hint = Uit schakelt de eigen luidspreker van de radio uit terwijl hij streamt (`UA2;`), zodat het geluid alleen in sdroxide te horen is. 2.00x had zo'n vorm niet.

settings-radio-ng-agc-auto = Auto (DIGI in datamodi)

settings-radio-ng-agc-off = Uit

settings-radio-ng-agc-on = Aan (snel)

settings-radio-sound-card-note = Welke geluidskaart het geluid van deze radio draagt staat onder **Audio**, samen met je eigen luidsprekers en microfoon — het is hetzelfde paar instellingen welke radio je ook gebruikt, dus het is één pagina.

settings-radio-no-control-cable-audio-page = Deze radio heeft geen stuurkabel — hij keyt zichzelf via VOX, dus er is geen PTT om aan te sturen. Zenden is audio naar de microfoon van de radio, daarom staat de zendaudio van de radio op de pagina Audio en niet hier.

settings-radio-sound-card-note-short = Welke geluidskaart het geluid draagt staat onder **Audio**.

settings-radio-control-link = Stuurverbinding

settings-radio-atsmini-host-hint =
    De ad-hoc TCP-server van de ontvanger: `atsmini.local` (mDNS) of zijn IP. Op de radio: Settings → TCP Port → Ad hoc, met Wi-Fi op Connect, AP+Connect of AP Only.
    
    `atsmini.local` vereist een mDNS-resolver (Avahi/nss-mdns) op deze machine. Als de stuurverbinding niet tot stand komt, staat zijn IP op het Wi-Fi-scherm van de radio — voer dat dan in.

settings-radio-atsmini-find-hint = Scan het netwerk van deze machine naar de stuurpoort van de ontvanger

settings-radio-tcp-port = TCP-poort

settings-radio-tcp-port-hint = De firmware gebruikt standaard 60000.

settings-radio-atsmini-receive-only = Alleen ontvangen: de Si4732 demoduleert in hardware, dus de hier gekozen modus wordt naar de radio gestuurd en al het overige — detectie, decodering, het log — gebeurt op deze computer.

settings-radio-atsmini-tuning-note =
    De afstemming volgt de eigen bandcyclus van de ontvanger, dus de dial loopt even achter op je scroll — het meest als de frequentie een andere band in gaat. Laat het eerst tot rust komen voor je verder scrolt, dan komt het altijd op de gevraagde frequentie uit.
    
    De afstapgrootte van de ontvanger is zijn eigen (de STEP-regeling hierboven): een wijziging die fijner is dan die stap wordt op de radio afgerond, en de dial springt terug naar de stap waarop hij echt uitkwam.

settings-radio-live-controls = Live-regelingen

settings-radio-atsmini-vol = VOL

settings-radio-atsmini-step = STEP

settings-radio-atsmini-sent-at-once = Direct naar de radio gestuurd. VOL is het eigen volume van de ontvanger (zijn knop, en dat van de Si4732), niet de AF-schuif van sdroxide.

settings-radio-refresh = Vernieuwen

settings-radio-read-99-slots = Lees de 99 slots van de ontvanger

settings-radio-tune-radio-slot = Stem deze radio af op het slot

settings-radio-no-slots-stored = Geen slots opgeslagen op de ontvanger.

settings-radio-press-refresh-slots = Druk op Vernieuwen om de geheugenslots van de ontvanger te lezen.

settings-radio-icomnet-receive-hint = AF: de radio demoduleert en sdroxide toont de audioband. 12 kHz IF: de radio stuurt in plaats daarvan zijn DRM-tussenfrequentie en sdroxide demoduleert, wat zijn eigen filters, ruisonderdrukking en decoders inzet over ongeveer ±12 kHz. In beide gevallen is de brede waterval de eigen scope van de radio — geen enkele Icom levert I/Q.

settings-radio-copy-icom-session-hint = Kopieert de laatste sessie van deze radio — zijn handshake en CI-V-trace — naar het klembord (of naar een bestand, in een browser die een pagina niet laat kopiëren), voor een bugrapport. Van deze radio, niet van het station: met twee Icoms op het LAN antwoordt elk tabblad over zijn eigen adres.

settings-radio-copy-protocol-trace-hint = Kopieert de protocoltrace van de laatste sessie naar het klembord (of naar een bestand, in een browser die een pagina niet laat kopiëren), voor een bugrapport.

settings-radio-copy-trace-device-hint = Kopieert de trace van de laatste sessie naar het klembord (of naar een bestand, in een browser die een pagina niet laat kopiëren), voor een bugrapport: elk commando dat met het apparaat is uitgewisseld en de eerste bytes van de samplestroom.

settings-radio-copy-trace-receiver-hint = Kopieert de trace van de laatste sessie naar het klembord (of naar een bestand, in een browser die een pagina niet laat kopiëren), voor een bugrapport: elk commando dat met de ontvanger is uitgewisseld en de eerste bytes van de samplestroom.

settings-radio-pluto-choose-receiver = Kies je ontvanger

settings-radio-dab-rate-hint = Kies 2.500 Msps als je DAB/DAB+ wilt decoderen

settings-radio-rx888-dab-rate-hint = Kies 80.0 Msps en de breedte 2.500 MHz (1/32) als je DAB/DAB+ wilt decoderen

settings-radio-pluto-both-chains-hint = Om op beide antennes te luisteren: voeg een tweede radio toe op hetzelfde adres en zet die op RX2. Beide radio's delen dezelfde frequentie.

settings-radio-duplex-tdd-hint =
    Of de AD9361 beide richtingen tegelijk doet (FDD, zoals een Pluto opstart en waarin sdroxide hem altijd heeft gelaten) of één tegelijk (TDD).
    
    Laat dit op FDD tenzij je de PTT-pinnen hieronder wilt — TDD is waarvan die keyen, en het sluit Full duplex hierboven uit. Wordt van kracht bij Toepassen.
    
    PlutoSDR: full duplex is uit — TDD schakelt één richting tegelijk in, en daar keyen de GPO-zend-ontvangpinnen van. Zet de PTT-pinnen op Uit en de duplex op FDD om tijdens een over mee te luisteren.

settings-radio-fm-mw-notch = FM/MW-notch

settings-radio-fm-mw-notch-hint = Hardwarematige notch over de FM-omroepband (88–108 MHz) en de middengolfbanden, voor als een lokale zender al het andere overstuurt. Werkt direct.

scanner-broadcast-band = Omroepband

scanner-all = ALLE

scanner-stop-only-on-channels-that-sound = Stop alleen op kanalen die klinken als iemand die praat: een stem is periodiek, vult de spraakband en stijgt en daalt een paar keer per seconde, wat een draaggolf, een birdie en een databurst niet doen. Elke kandidaat wordt minstens een halve seconde beluisterd, want zo lang duurt het om een lettergreep te horen.

scanner-nothing-in-this-scan-is-in-a-mode = Niets in deze scan staat in een modus die spraak draagt — een stemjacht zou hier lopen en nergens stoppen, dus dit wordt pas aangeboden als dat wel zo is. FM, AM en de zijbanden dragen stemmen; de digitale modi niet.

scanner-voice = STEM

scanner-stop-only-where-somebody-is = stop alleen waar iemand praat

scanner-no-voice-to-look-for-in-these = geen stem om naar te zoeken in deze modi

scanner-sql = SQL

scanner-how-long-to-stay-on-a-candidate = Hoe lang op een kandidaat blijven voor het oordeel. De stemcontrole heeft minstens zoveel nodig: een stem is een ritme van lettergrepen, en een halve seconde zijn er twee

scanner-listening-for-a-voice = luisteren naar een stem…

scanner-scan-band-hover = Scan { $name } ({ $lo }-{ $hi } kHz)

scanner-range-to = tot

scanner-mode-no-speech = { $mode } draagt geen spraak — deze kanalen worden overgeslagen

scanner-voice-holding = stem { $score } · iemand praat

scanner-voice-passed = overgeslagen { $mhz } MHz · stem { $score }

aircraft-hfdl-aircraft = HFDL-vliegtuig { $id }

aircraft-no-icao = geen ICAO-adres (HFDL)

aircraft-provenance = { $source }, { $age } geleden

aircraft-label-flight = Vlucht

aircraft-label-registration = Registratie

aircraft-label-position = Positie

aircraft-label-altitude = Hoogte

aircraft-label-speed = Snelheid

aircraft-label-track = Koers

aircraft-label-last-message = Laatste bericht

aircraft-label-type = Type

aircraft-label-operator = Operator

aircraft-label-origin = Herkomst

aircraft-label-destination = Bestemming

aircraft-label-online = Online

aircraft-missing-flight = nog niet gehoord — ADS-B of VDL2 dragen het

aircraft-missing-registration = nog niet gehoord — alleen VDL2 draagt het

aircraft-missing-position = nog niet gehoord — ADS-B, of een VDL2- of HFDL-positierapport

aircraft-missing-adsb = nog niet gehoord — ADS-B draagt het

aircraft-missing-message = nog geen — VDL2, of ACARS-audio zodra VDL2 het vliegtuig heeft genoemd

aircraft-online-unavailable-disabled = online niet beschikbaar — de schakelaar Online staat uit

aircraft-online-unavailable-no-station = online niet beschikbaar — geen station om te vragen; de radiodata is alles wat er is

aircraft-online-unavailable-unauthorized = online niet beschikbaar — niet aangemeld bij het station

aircraft-online-unavailable-retry = online opvraag mislukt — wordt zo opnieuw geprobeerd

aircraft-switch-no-answer = het station antwoordde niet

aircraft-switch-not-signed-in = niet aangemeld bij het station

aircraft-switch-refused = het station weigerde het verzoek

aircraft-switch-too-old = het station biedt deze schakelaar niet aan (te oud?)

aircraft-switch-failed = het station kon het niet wijzigen

aircraft-online-source = adsbdb, via het station

aircraft-asking-station = het station wordt gevraagd

aircraft-provenance-radio = radio

aircraft-provenance-radio-online = radio + online

aircraft-provenance-online = online

aircraft-photo-credit = Foto: { $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = Er is geen antennerotator ingesteld.

rotor-set-one-up-in-settings-servers-a = Stel er een in onder Instellingen ▸ Servers — een Hamlib rotctld-daemon, of een controller op een seriële poort. Richt daarna de bundel vanaf deze dial, vanaf een roepnaam of vanaf de kaart.

rotor-halt-the-antenna-where-it-is = Stop de antenne waar hij is

rotor-park = PARKEER

rotor-hand-the-antenna-back-to-the = Geef de antenne terug aan de satellietvergrendeling — die volgt de satelliet en parkeert als er geen is.

rotor-point = RICHT

rotor-dx-country = DX-land

rotor-set-your-grid-locator-settings = Stel je grid-locator in (Instellingen ▸ Algemeen) om op een land te richten.

rotor-connected = Verbonden — antenne op { $az }°

rotor-not-connected-why = Niet verbonden: { $e }

rotor-beam = Bundel { $az }° az  { $el }° el

rotor-target = doel { $az }° / { $el }°

rotor-park-hover = Draai naar de parkeerpeiling ({ $az }° / { $el }°)

rotor-country-bearing = { $name } — peiling { $az }°

aircraft-switch-no-station = er is geen station om te schakelen

aircraft-title = VLIEGTUIGEN

aircraft-mode-adsb-hover = ADS-B, 1090 MHz — vliegtuigen met hun positie, hoogte en snelheid

aircraft-mode-hfdl-hover = HFDL, HF-datalink — vliegtuigen boven oceanen en polen, uit hun positierapporten

aircraft-mode-vdl2-hover = VDL2, VHF-datalink — berichten van vliegtuigen; weinig dragen een positie

aircraft-mode-acars-hover = ACARS — korte berichten tussen vliegtuigen en de grond

aircraft-badge-online = online

aircraft-badge-history = historie

aircraft-stats-total = { $n } vliegtuigen

aircraft-stats-most-first = Meeste eerst

aircraft-stats-empty = Nog geen vliegtuigen. De tabellen vullen zich vanuit ADS-B, VDL2, HFDL en ACARS; niets hier heeft internet nodig.

aircraft-stats-relay-note = Model, operator en route tellen alleen de vliegtuigen waarover de relay heeft geantwoord — open een kaart in Kaarten om er een op te zoeken.

aircraft-stats-known-kinds = { $known } bekend · { $kinds } soorten

aircraft-and-more = en { $more } meer

aircraft-show = Toon

aircraft-aviation = Luchtvaart

aircraft-cards-chip = Kaarten { $n }

aircraft-cards-hover = Elk vliegtuig dat op ADS-B, VDL2, HFDL en ACARS is gehoord, één kaart elk — tien minuten bewaard, welke modus de radio nu ook heeft

aircraft-watch-chip = Volglijst { $n }{ $suffix }

aircraft-watch-hover = Vliegtuigen waarover je gewaarschuwd wilt worden, op roepnaam of registratie — de lijst wordt door het station bijgehouden, en er verschijnt een melding als er een in de lucht komt

aircraft-stats-chip = Statistieken

aircraft-stats-hover = De meest voorkomende categorieën, luchtvaartmaatschappijen, modellen, operators en routes onder de vliegtuigen die dit station kent — eerst geteld uit de radio, met de kaarten van de relay als aanvulling

aircraft-export = Exporteren

aircraft-export-csv-hover = Sla de vliegtuigen op als CSV-tabel voor een spreadsheet: de laatste 24 uur van het station als het een historie bijhoudt, anders wat nu wordt gehoord. Geen posities.

aircraft-export-json-hover = Sla de vliegtuigen op als JSON: de laatste 24 uur van het station als het een historie bijhoudt, anders wat nu wordt gehoord. Geen posities.

aircraft-watch-intro = Roepnamen en registraties waarover je gewaarschuwd wilt worden. De lijst wordt door het station bijgehouden, dus is op elk scherm dezelfde; er komt een melding als er een verschijnt, of terugkomt na tien minuten uit de lucht.

aircraft-watch-needs-station = De volglijst wordt bijgehouden door het station waarmee je verbonden bent. Maak verbinding met een station om hem te gebruiken.

aircraft-watch-reading = De lijst wordt gelezen…

aircraft-watch-cannot-read = Kan de lijst niet lezen: { $why }.

aircraft-watch-input-hint = KLM1234 of PH-BXA

aircraft-watch-add = Toevoegen

aircraft-watch-sending = Verzenden…

aircraft-watch-empty = Nog niets op de lijst.

aircraft-watch-on-air = nu in de lucht

aircraft-watch-remove = Verwijderen

aircraft-watch-alerts = Meldingen

aircraft-watch-clear = Wissen

aircraft-watch-none-yet = Nog geen.

aircraft-cards-empty = Nog geen vliegtuigen gehoord. Kaarten vullen zich vanuit ADS-B, VDL2, HFDL en ACARS — druk op een ervan en luister. Niets hier heeft internet nodig.

aircraft-cards-no-match = Geen vliegtuigen komen overeen met “{ $query }”.

aircraft-history-hover = Toon waar dit vliegtuig het laatste etmaal is geweest, volgens het eigen archief van het station. Het vraagt je station, nooit het internet.

aircraft-copy-summary = Samenvatting kopiëren

aircraft-copy-summary-hover = Kopieer deze kaart als tekst: wat bekend is en waar elke regel vandaan komt. De positie en het laatste bericht worden weggelaten.

aircraft-asking-station-dots = het station wordt gevraagd…

aircraft-not-changed = Niet gewijzigd: { $why }.

aircraft-loading-photo = foto laden…

aircraft-radio-is-on = De radio staat op { $mode } — druk op { $label } om te luisteren. Dit is wat { $label } eerder hoorde.

aircraft-no-adsb = Nog geen ADS-B gehoord.

aircraft-hfdl-off = De HFDL-decoder staat uit — druk op HFDL om hem aan te zetten.

aircraft-vdl2-starting = De VDL2-decoder wordt gestart…

aircraft-no-view = { $label } heeft hier geen weergave.

textmodem-waiting-modem = wachten op de modem…

textmodem-locked = VERGRENDELD

textmodem-no-frame = GEEN FRAME

textmodem-locked-hint = Een frame wordt gevolgd. Het wordt alleen geaccepteerd als de synccorrelatie, gevouwen over twaalf frames, duidelijk boven haar eigen nul uitkomt en de autocorrelatie van de frameperiode zegt dat er een frame aanwezig is.

textmodem-sync-readout = sync { $sync } (nul { $null })

textmodem-sync-hint = De differentiële synccorrelatie gevouwen over elke framefase, en de mediaan over die fasen — de nul die ze moet verslaan. Gemeten uit de ether: 0.85 op een 4285 tegen 0.14 op een sterke PSK-draaggolf van een andere modus.

textmodem-frame-readout = frame { $acf }

textmodem-frame-hint = Hoeveel van een frame zich één frame later herhaalt. Een 4285 herhaalt zijn 80 syncsymbolen, dus 0.3-0.6; de draaggolf van een andere modus meet ongeveer 0.03.

textmodem-refs-readout = refs { $a }/{ $b }/{ $c }

textmodem-refs-hint = De concentraties van de drie referentieblokken na descrambling. Ze zijn allemaal symbool 0 vóór scrambling, dus een correct frame laat elk ervan op één punt samenvallen — 0.7 en hoger. Zestien symbolen ruis geven 0.25.

textmodem-rate-hint = Wat de constellatie kan zeggen. Elke ongecodeerde rate deelt zijn constellatie met een gecodeerde, dus de gecodeerde rates hebben Annex E nodig om onderscheiden te worden — en hun payload heeft de FEC van Annex E nodig om gelezen te worden.

textmodem-offset-frames = { $offset } Hz · { $n } frames

textmodem-offset-frames-hint = De draaggolfoffset waarover de referentieblokken het eens zijn, en hoeveel frames deze sessie zijn gedemoduleerd.

textmodem-som-readout = SOM { $best }/32 · { $n } ber.

textmodem-som-hint = Het start-van-bericht-patroon uit Annex E, en hoe dicht de uitvoer van de decoder erbij in de buurt is gekomen. Ruis zit rond 16 van 32; een gecodeerde uitzending haalt de twintig, ook als een fade een match verhindert, en een match is wat de gecodeerde snelheid benoemt en een bericht start. De code en de interleaver decoderen een gegenereerd signaal exact, maar er is nog geen gecodeerd station via de ether gelezen, dus een match hier is het eerste wat je mag verwachten als dat lukt.

textmodem-rx-only = ALLEEN RX

textmodem-rx-only-hint = De decoder van deze modus is overgezet en leest zijn opnames; de modulator is nog niet geschreven, dus de zendkant staat uit in plaats van achteraf te weigeren.

textmodem-msg-hint = Je eigen berichtknoppen — werkomstandigheden, het weer, een standaardantwoord. Elke knop verzendt zijn hele tekst in één keer, en F1–F9 drukken de eerste negen in. Ze reizen mee met de configuratie van het station, dus een client op afstand heeft ze ook.

textmodem-weather-chip = {" "}WEER { $n }{" "}

textmodem-weather-hint =
    Gedecodeerde SYNOP- en SHIP-rapporten die op deze band zijn ontvangen.

    Ze worden hier bewaard in plaats van in de decodeerlijst, omdat de cijfers op zichzelf niets betekenen — en omdat het weer op een positie van drie uur geleden is waarvoor je terugkomt.

    Gevonden in de RTTY-tekst zodra die binnenkomt, dus de modus moet op de shift en baud van het station staan. Een landstation stuurt geen positie, dus die rijen bevatten het stationsnummer en de cijfers maar geen coördinaten; een scheepsrapport bevat die zelf.

textmodem-olivia-reverse-hint = Draai de toonvolgorde om, in beide richtingen. Een station dat omgekeerd zendt, klinkt als ruis totdat dit aan staat.

textmodem-olivia-tones-raised = (Olivia's { $n } → 4)

textmodem-ifk-reverse-hint = Draai de toonstappen om. IFK+ draagt zijn data in de stap tussen opeenvolgende tonen, dus een station dat op de andere zijband is ontvangen heeft elke stap negatief en klinkt als ruis totdat dit aan staat.

textmodem-domino-hint = { $name } — { $baud } baud, 18 tonen, incrementele frequentiesleuteling zonder foutcorrectie vooraf

textmodem-mt63-long-hint = MT63-{ $label }, { $bw } Hz breed — de lange interleave spreidt een burst over { $chars } tekens

textmodem-mt63-short-hint = MT63-{ $label }, { $bw } Hz breed, korte interleave — de korte interleave spreidt een burst over { $chars } tekens

textmodem-mt63-not-offered = 1K en 500L nog niet beschikbaar

textmodem-mt63-not-offered-hint = MT63-1K en de lange interleave van 500 Hz worden gelezen met de batchdecoder, maar nog niet via de live ontvanger: de lock zoekt de symboolfase en de polariteit maar niet het bandcentrum, en een zoekactie die dat wel doet bleek de ene variant voor de andere in te ruilen. Zie docs/internal/MODE-PROGRESS.md.

textmodem-mfsk-caveat-mfsk8 = { " " }— de corpusopname verliest één stuk van de tekst; de decoder is dezelfde als bij alle andere varianten

textmodem-mfsk-caveat-mfsk11 = { " " }— de corpusopname verliest de eerste groep door het opstarten van de resampler

textmodem-mfsk-hint = { $label } — { $baud } baud, { $tones } tonen, { $bw } Hz breed{ $caveat }

textmodem-mfsk-reverse-hint = Draai de toonvolgorde om. Een station dat is ontvangen op de andere zijband dan waarop het is verzonden, komt gespiegeld om zijn midden binnen en klinkt als ruis totdat dit aan staat.

textmodem-psk-family = Familie

textmodem-psk-family-plain-hint = De eigen familie van PSK31: één differentiële draaggolf, geen foutcorrectie vooraf

textmodem-psk-family-bpsk63f-hint = De robuuste code op 62,5 baud, zonder interleaver — de enige gecodeerde modus die fldigi zonder verzendt

textmodem-psk-family-robust-hint = Eén draaggolf met de robuuste code en een 2x2xN-interleaver

textmodem-psk-family-multi-hint = Meerdere draaggolven, de varicode van PSK31 en **geen** code — breder en sneller, en het vraagt een schoon signaal

textmodem-psk-family-robust-multi-hint = Meerdere draaggolven **en** de robuuste code: de breedste en de meest robuuste van de PSK-modi

textmodem-psk-baud-fast-hint = { $baud } baud — leest vanaf ongeveer een seconde: bij deze snelheid is de lock-tijd van de ontvanger een derde van een bericht

textmodem-psk-baud-hint = { $baud } baud

textmodem-psk-modulation = Modulatie

textmodem-psk-bpsk-hint = Eén bit per symbool, differentieel gedetecteerd

textmodem-psk-qpsk-hint = Twee bits per symbool met een rate-1/2-code, dus één databit per symbool: dezelfde tekstsnelheid als BPSK bij dezelfde snelheid, met de helft van de fouten

textmodem-psk-8psk-hint = Drie bits per symbool, geen code: de snelste van de drie bij elke gegeven snelheid en de minst robuuste. Alleen ontvangen — er is geen achtfasige zender. Bewezen op een synthetisch signaal: de 8PSK-opnames in het testcorpus zijn geen zuivere constellatie

textmodem-psk-coded = gecodeerd

textmodem-psk-no-code = geen code

textmodem-psk-coded-hint =
    { $baud } baud, { $n } { $n ->
            [one] draaggolf
           *[other] draaggolven
        }, { $code }, { $bw } Hz breed — alleen ontvangen: de gecodeerde PSK-zender is niet geschreven

textmodem-psk-wide-bank-hint = { "*" }*Deze brede bank decodeert nog niet.** De draaggolven liggen zo dicht bij elkaar dat het filter van de ontvanger op de ingangssnelheid ze niet kan scheiden; de smallere banken lezen hun eigen opnames wel. Er gaat niets verloren van wat je hier hoort — het ligt aan de front-end, niet aan het signaal.

textmodem-hell-slant = Helling

textmodem-hell-slant-hint = Bijstelling van de ontvangst-samplekloks. Een opname waarvan de stippen 0,4 % te lang aankomen — een gewone geluidskaart, en wat de off-air Hell-opname van SkySweep meet — heeft ongeveer -4000 nodig. Stel bij totdat de tekens rechtop staan.

settings-net-rbn-enabled-hint = Lees de skimmers van de wereld en voed er de propagatiekaart mee. Hierdoor toont de kaart ook banden waarop deze radio niet luistert. Er gaat niets de ether in, er is geen account nodig, en het gebruikt de roepletters van het tabblad Algemeen. RBN-spots verschijnen niet in de spotlijst — het zijn metingen, geen uitnodigingen. De host is niet gebonden aan reversebeacon.net: elke node die dezelfde telnet-feed spreekt, werkt.

settings-net-rbn-port-hint = Op reversebeacon.net is 7000 de CW/RTTY-feed, 7001 die voor FT8/FT4

settings-net-wavelog-intro = Wavelog is een logboek dat **jij** draait, op een adres dat jij opgeeft. Hier gelogde contacten worden geüpload naar de stationslocatie die je kiest, en contacten die het al bevat kunnen worden teruggehaald zodat “eerder gewerkt” weet van QSO's die vanuit een ander programma, een andere rig of het eigen webformulier zijn gemaakt.

settings-net-wavelog-address-hint = Waar de voorpagina van Wavelog staat: https://log.example.org, of het LAN-adres. /index.php wordt in beide gevallen geaccepteerd.

settings-net-wavelog-key-hint = Een API-sleutel van de eigen API-pagina van Wavelog. Een lees/schrijf-sleutel is nodig om te uploaden; een alleen-lezen sleutel kan alleen downloaden.

settings-net-pick-one = kies er een

settings-net-test-to-list-locations = test de verbinding om de locaties te tonen

settings-net-trust = VERTROUW

settings-net-trust-hint = Accepteer **dit** certificaat en geen ander, voor deze server. Een pin in plaats van “alles accepteren”: als het ooit verandert, wordt de verbinding geweigerd en wordt dat gemeld, in plaats van stilletjes te verbinden met wat er op dat adres antwoordt.

settings-net-test-wavelog = Test Wavelog

settings-net-test-wavelog-hint = Vraag de server of dit adres en deze sleutel werken, en toon de stationslocaties. Past eerst de instellingen toe; logt niets.

settings-net-wavelog-download = Contacten downloaden van Wavelog

settings-net-every-min = Elke (min)

settings-net-locations = Locaties

settings-net-download-now = Nu downloaden

settings-net-download-now-hint = Haal binnen wat Wavelog sinds de laatste download heeft toegevoegd, en markeer contacten die het al bevat als geüpload. Wat al in het logboek staat, wordt niet gewijzigd.

settings-net-wavelog-upload-note = Alleen contacten die **hier** zijn gelogd worden automatisch geüpload; de eigen achterstand van Wavelog wordt verzonden door een QSO in het logboek aan te vinken, zodat er niets deze machine verlaat wat je niet hebt gekozen.

settings-net-broadcast-note = De lang- en kortegolfstations die op de waterval zijn gelabeld. Elke bron hieronder wordt gedownload voor het huidige uitzendseizoen en opnieuw gedownload als het seizoen wisselt. Ze komen allemaal in **één opslag**, zodat het bronfilter van het SCHEDULE-venster er één van toont of alle.

settings-net-downloaded = gedownload

settings-net-schedules = Schema's

settings-net-schedule-yours = van jou

settings-net-download = Downloaden

settings-net-hfcc-hint = HFCC publiceert zijn stationsnamen in een apart bestand, dus dit heeft beide nodig. Zolang dat niet zo is, meldt het dat in plaats van een schema van kale roepletters te laden.

settings-net-refetch-hint = Haal de kopie van dit seizoen opnieuw op en vervang de gecachete

settings-net-from-file = Uit bestand…

settings-net-aoki-file-hint = Neem het schema van Aoki uit een bestand dat je zelf hebt opgeslagen — de .zip die Aoki publiceert, of de .txt daarin. De site verhuist met het seizoen mee, vandaar dat dit hier staat.

settings-net-eibi-file-hint = Neem het schema van EiBi uit zijn sked-.csv, opgeslagen op deze machine

settings-net-filter-aoki = Aoki-schema

settings-net-filter-eibi = EiBi-schema

settings-net-download-all = Alles nu downloaden

settings-net-download-all-hint = Haal elke bron op die opgehaald kan worden, en behoud die al werken

settings-net-own-list-note = Je eigen lijst is een **schema** — wat je verwacht te horen — niet het ontvangstlogboek, dat vastlegt wat je echt hebt gehoord. Importeer het hieronder, of bewerk het bestand.

settings-net-import-list = Mijn stationslijst importeren…

settings-net-import-list-hint = Lees een CSV of een gewone lijst met stations en voeg die samen met het bestand hierboven. Frequenties mogen in kHz of MHz zijn; de eenheid wordt uit het getal afgeleid.

settings-net-filter-station-lists = Stationslijsten

settings-net-dx-status = DX — { $body }

settings-net-pin-own = eigen — { $pin }…

settings-net-schedule-rows = { $name } { $n } uitzendingen

settings-net-schedule-not-downloaded = { $name }     niet gedownload

settings-net-schedule-imported = { $source } { $season } — { $n } uitzendingen, nu in de opslag

settings-net-stations-imported =
    { $n ->
        [one] { $n } station in je lijst, nu in de opslag
       *[other] { $n } stations in je lijst, nu in de opslag
    }

txaudio-natural = Natuurlijk

txaudio-ragchews-and-local-contacts-the = Ragchews en lokale contacten: de ruisonderdrukker, een zachte expander in de pauzes, lichte compressie. Klinkt als jij, minus de kamer.

txaudio-ssb-everyday = SSB dagelijks

txaudio-everyday-ssb-100-hz-2-9-khz-a = Dagelijkse SSB: 100 Hz – 2,9 kHz, een beetje presence rond 2 kHz, matige compressie.

txaudio-ssb-punch-dx = SSB punch / DX

txaudio-weak-signals-and-pileups-a-narrow = Zwakke signalen en pileups: een smalle stem van 300 Hz – 2,7 kHz met een sterke presence-verhoging, in een snelle compressor gestuurd met de fasedraaier, voor het meeste spreekvermogen dat de zender heeft. Geen hi-fi.

txaudio-ssb-punch-for-hours-in-a-noisy = SSB punch urenlang in een rumoerig station: de ruisonderdrukker en een harde gate houden ventilatoren en de volgende operator uit je pauzes, en de compressor werkt nog harder.

txaudio-essb-hi-fi = ESSB / hi-fi

txaudio-wide-ssb-or-am-50-hz-4-khz-with = Brede SSB of AM: 50 Hz – 4 kHz met behoud van het lage eind, zachte compressie en een de-esser. Vereist een zendfilter dat zo breed is — en een rustige band.

txaudio-fm-repeater = FM / repeater

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM: een steil hoogdoorlaatfilter van 300 Hz houdt de stem vrij van de CTCSS-toon, de top stopt bij 3 kHz voor het kanaal, en de limiter houdt de deviatie in toom.

txaudio-mobile-noisy-shack = Mobiel / rumoerige shack

txaudio-fans-engine-traffic-the-denoiser-a = Ventilatoren, motor, verkeer: de ruisonderdrukker, een steil hoogdoorlaatfilter op 150 Hz en een harde gate, met genoeg compressie om boven wat overblijft uit te komen.

txaudio-digital-voice-2 = Digitale spraak

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE: alleen de ruisonderdrukker en zachte egalisatie, geen EQ — de vocoder geeft de voorkeur aan natuurlijke spraak. Zet DIGITAL VOICE aan.

txaudio-imported-2 = Geïmporteerd

frame-no-contrast-fit = Geen contrast — klik om te FITTEN

frame-levels-hidden-hint = De onder- en bovengrens van het scherm liggen voorbij alles wat in beeld is, dus de waterval is één egale kleur. Klik om ze aan te passen aan wat de radio hoort.

frame-bw-waterfall = BW-waterval · { $span }

frame-bw-waterfall-unknown = BW-waterval · —

frame-every-module-own-window = elke module staat in een eigen venster

frame-window-chip-hint = VENSTER — verplaats dit venster naar een eigen OS-venster, om het op een andere monitor te zetten

frame-dock-panel-pane-hint = DOCK — zet dit paneel terug in het vlak onder de waterval

frame-dock-panel-hint = DOCK — zet dit paneel terug onder de waterval

frame-dock-spectrum-hint = DOCK — zet dit spectrum terug in het hoofdvenster, hetzelfde als dit venster sluiten zonder het sluitvakje te zoeken.

frame-dock-band-menu-hint = DOCK — zet de band- en modusselectie terug in het hoofdvenster, hetzelfde als dit venster sluiten zonder het sluitvakje te zoeken.

frame-dock-rotator-hint = DOCK — zet de rotor terug in het hoofdvenster, hetzelfde als dit venster sluiten zonder het sluitvakje te zoeken.

frame-module-panadapter = panadapter

frame-module-panel = paneel

frame-module-controls = bediening

frame-module-aux-panadapter = aux-panadapter

frame-module-band-mode = band & modus

frame-module-rotator = antennerotor

frame-detached-title = { $flavor } — { $name }

frame-workspace-missing = Geen werkruimte { $n } opgeslagen — sla er een op in Instellingen › UI › Werkruimtes en deze toets roept hem terug.

frame-no-operating-panel = Geen bedieningspaneel in { $mode } — het is losgekoppeld, en deze modus heeft er geen.

frame-rx-only-nudge = Deze radio is alleen-ontvangen — hij heeft geen zender. De zendbediening verbergen en het luisterscherm gebruiken?

frame-listening-controls = Luisterbediening

frame-listening-controls-hint = Verberg de zendbediening en zet deze radio op het luisterscherm (Instellingen → Radio). Er wordt niets herafgestemd en de decoder blijft draaien.

frame-update-available = SDR Oxide Brown { $tag } is beschikbaar — je gebruikt { $version }.

frame-download = Downloaden

frame-download-hint = Opent de releasepagina in je browser

frame-tuning-catching-up = afstemmen — de radio loopt bij

frame-roll-up-hint = Oprollen — verlaat de titelbalk en geef het venster terug aan de waterval

frame-unroll-hint = Uitrollen — toon het paneel weer

frame-window-panel-hint = VENSTER — geef dit paneel een eigen venster, om op een andere monitor te zetten

frame-panel-window-title = { $mode } — paneel

frame-auto-stopped-stop-control = auto gestopt: er is een stopbediening gebruikt

frame-dont-show-again = Niet meer tonen

frame-dont-show-again-hint = Onthoud deze bevestiging op dit scherm. De vergrendeling blijft uit zolang de engine met --oob-tx draait — dit voorkomt alleen dat de waarschuwing opnieuw wordt getoond.

frame-cb-window-title = ⚠  11 m (CB) — GEEN AMATEURBAND

frame-cb-about-to-allow = Je staat op het punt zenden op de 11 m-band toe te staan.

frame-cb-not-amateur = 11 m is geen amateurband. Het is een aparte radiodienst met eigen kanalen, modi, vermogenslimieten en typegoedgekeurde apparatuur, en die verschillen van land tot land. Zenden daar valt onder de regels van het land waar je opereert.

frame-cb-only-continue = Ga alleen door als je weet wat jouw land op 27 MHz toestaat en je daarbinnen opereert — je blijft verantwoordelijk voor de uitzending.

frame-cancel = Annuleren

frame-saved-to-profile = opgeslagen in het profiel { $name }

frame-saved-as-default = opgeslagen als standaard van dit station (de server heeft geen wachtwoord, dus dit is wat elke client krijgt)

frame-bindings-offer-title = Dit profiel bevat toetsenbordkoppelingen

frame-bindings-offer-profile = Het profiel waarmee je bent ingelogd ({ $name }) bewaart toetsenbord- en muiskoppelingen op de server, en deze browser gebruikt ze niet. Dat is de veilige standaard — op een station dat anderen delen, wordt ook het toetsenbord gedeeld, en dat één login de PTT of Spatie van een ander overneemt, is een echt risico.

frame-bindings-offer-default = Het standaardprofiel van dit station bewaart toetsenbord- en muiskoppelingen op de server, en deze browser gebruikt ze niet. Dat is de veilige standaard — op een station dat anderen delen, wordt ook het toetsenbord gedeeld, en dat één login de PTT of Spatie van een ander overneemt, is een echt risico.

frame-bindings-offer-choice = Gebruik ze hier, of laat ze. In beide gevallen krijg je te horen welke, en er wordt niets gewijzigd zonder dat jij kiest.

frame-bindings-use = De koppelingen van dit profiel gebruiken

frame-bindings-keep-mine = Mijn eigen houden (en niet meer vragen)

frame-bindings-using-profile = de voor dit profiel opgeslagen toetsenbordkoppelingen worden gebruikt

frame-bindings-keeping-own = de eigen toetsenbordkoppelingen van dit apparaat blijven behouden; er wordt niet meer gevraagd

frame-look-back-to-profile = terug naar het voor dit profiel opgeslagen uiterlijk

frame-look-none-stored = nog geen opgeslagen uiterlijk voor dit profiel — sla er eerst een op

frame-antenna-rotator-caption = ANTENNEROTOR

bands-condx-global-forecast-rest-what = CONDX is een wereldwijde voorspelling. WSPR is wat het WSPR-netwerk van de wereld heeft gehoord (wspr.live); PATHS, REACH en BEST zijn wat dit station en de RBN hebben gehoord. Een lege rij betekent dat er niets is gedecodeerd — wat kan betekenen dat de band dicht was, of alleen dat niemand erop zat.

bands-km-s-parent-peak-rate =
    { $name } ({ $code }) — { $velocity_kms } km/s, moeder { $parent }. Piekwaarde ZHR { $zhr } rond { $peak }.
    
    Een radiant boven de horizon betekent dat de sporen je kunnen bereiken; een snelle zwerm laat langerlevende geïoniseerde sporen na voor meteorscatter op 6 m en 2 m, en de korte openingen op 10 m/11 m.

bands-hearing-means-path-open-each = { $callsign } bij { $location } — het horen ervan betekent dat het { $band }-pad open is. 10m (28,200 MHz) is de dichtstbijzijnde amateurband-benadering voor de condities op 11 m. Elke baken schuift elke 10 s een band op, dus deze rij verandert elk slot.

bands-wspr-activity-old = · WSPR-activiteit { $age } oud (wspr.live)

bands-psk-activity-old = · PSK-activiteit { $age } oud

bands-activity-hover =
    Wereldwijde activiteit op { $band } in de afgelopen 15 minuten, van { $source }:
    { $paths } ontvangstrapporten · { $tx } zenders · { $rx } ontvangers
    
    { $note }
    
    Helderder weergegeven naarmate de band drukker is; het zegt dat de band ergens wordt gehoord, niet dat hij voor jou open is.

bands-derived-hover =
    { $band } is geen van de banden die HAMQSL.com beoordeelt; dit is het oordeel van de gepubliceerde { $group }-groep, gelezen als de dichtstbijzijnde vervanger.
    
    Een voorspelling, geen meting van je eigen pad.

bands-wspr-note = Het WSPR-netwerk van de wereld — zwaksignaalbakens.

bands-psk-note = Ontvangstrapporten van de activiteitsmodi: FT8, FT4 en de CW/RTTY-rapportage die de bakens van WSPR niet dekken.

aircraft-history-reason-disabled = het station heeft zijn geschiedenis niet ingeschakeld

aircraft-history-reason-unauthorized = het station wil een aanmelding

aircraft-history-reason-no-station = de geschiedenis wordt bewaard door het station waarmee je verbonden bent

aircraft-history-reason-unknown = het antwoord van het station werd niet begrepen

aircraft-history-show = Geschiedenis tonen

aircraft-history-hide = Geschiedenis verbergen

aircraft-label-history = Geschiedenis

aircraft-label-first-heard = Eerst gehoord

aircraft-label-last-heard = Laatst gehoord

aircraft-label-track-points = Trackpunten

aircraft-label-highest = Hoogste

aircraft-label-callsigns = Roepletters

aircraft-label-registrations = Registraties

aircraft-history-nothing-kept = nog niets bewaard voor dit vliegtuig

aircraft-history-by-station = door dit station

aircraft-history-kept-24h = bewaard voor de laatste 24 u

aircraft-history-of-kept = van de bewaarde punten

aircraft-history-used-24h = gebruikt in de laatste 24 u

aircraft-history-seen-24h = gezien in de laatste 24 u

aircraft-history-looked-up = { $age } geleden opgezocht

aircraft-alert-on-air-as = { $entry } is in de lucht, als { $callsign }

aircraft-alert-on-air = { $entry } is in de lucht

aircraft-watch-already = Staat al op de lijst.

aircraft-watch-absent = Staat niet op de lijst.

aircraft-watch-invalid = Geen roepletters of registratie: 3 tot 10 letters of cijfers.

aircraft-watch-full = De lijst is vol: 100 vliegtuigen.

multi-module-windows = MODULEVENSTERS

multi-in-its-own-window = IN EIGEN VENSTER

multi-docked = GEDOCKT

multi-dock-all-windows = ALLE VENSTERS DOCKEN

multi-bring-every-module-window-back = Breng elk modulevenster terug in het hoofdvenster

multi-undock-all-windows = ALLE VENSTERS LOSKOPPELEN

multi-put-every-module-in-a-window-of = Zet elke module in een eigen venster — het hoofdvenster blijft over als het spectrum en de decoders

multi-two-radios-side-by-side-need-a = Twee radio's naast elkaar vragen een breder venster — dit scherm toont er één tegelijk

multi-the-station-s-windows-some-modules = De vensters van het station — sommige modules staan in hun eigen OS-vensters. Open dit om dat te wijzigen.

multi-put-a-module-in-its-own-window-sp1 = Zet een module in een eigen venster — SP1 of AUX SP op een andere monitor, de RX-bediening ernaast. Dezelfde rijen staan in Instellingen → UI → Losgekoppeld.

multi-module-sp1 = SP1 — het spectrum

multi-module-sp2 = SP2 (AUX) — een tweede spectrum

multi-module-panel = Bedieningspaneel

multi-module-controls = RX-bediening

multi-module-band-mode = Band & modus

multi-module-rotor = Antennerotor

multi-bring-back = Breng { $name } terug in het hoofdvenster

multi-put-own-window = Zet { $name } in een eigen OS-venster — met een eigen app-id, zodat een vensterregel het aan een monitor kan vastpinnen

multi-windows-chip = VENSTERS

aircraft-stats-category = Categorie

aircraft-stats-airline = Maatschappij

aircraft-stats-model = Model

aircraft-stats-operator = Operator

aircraft-stats-route = Route

aircraft-stats-src-no-station = De nu gehoorde vliegtuigen worden geteld. Het station waarmee je verbindt bewaart de laatste dag.

aircraft-stats-src-asking = Het station om de laatste dag vragen…

aircraft-stats-src-ready = De laatste 24 u bewaard door het station, met de nu gehoorde vliegtuigen.

aircraft-stats-src-disabled = Het station heeft zijn geschiedenis niet ingeschakeld — de nu gehoorde vliegtuigen worden geteld.

aircraft-stats-src-reason = { $reason } — de nu gehoorde vliegtuigen worden geteld.

aircraft-export-saved-station = { $name } opgeslagen: de laatste 24 uur van het station.

aircraft-export-saved-local = { $reason }: in plaats daarvan zijn de { $n } vliegtuigen van de afgelopen tien minuten opgeslagen.

login-remember-this-station = Dit station onthouden

login-remember-me = ONTHOUD MIJ

login-this-station-keeps-your-sign-in = Dit station bewaart je aanmelding, niet deze pagina — niets in te vullen op de andere radio's, en niets te vergeten als je weggaat.

login-not-this-time-this-browser-forgets = Deze keer niet: deze browser vergeet het als hij sluit, dus je wordt de volgende sessie opnieuw gevraagd en er blijft niets van deze aanmelding op het apparaat achter.

aircraft-label-range = Bereik

aircraft-label-vertical = Verticaal

aircraft-label-emergency = Noodgeval

aircraft-from-your-station = van je station

aircraft-footer-disabled = online niet beschikbaar — relay uit op het station

aircraft-footer-no-station = online niet beschikbaar — geen station om te vragen

aircraft-footer-unauthorized = online niet beschikbaar — niet aangemeld

aircraft-hover-close = Sluiten (Esc)

aircraft-hover-lookup = Online opzoeken

aircraft-hover-lookup-hint = Vraag de relay van dit station naar dit vliegtuig: type, operator, route en een foto. Er wordt niets gevraagd totdat je erop drukt.

aircraft-hover-lookup-waiting = er loopt al een andere opzoeking — één tegelijk

aircraft-hover-history-hint = Waar dit vliegtuig de laatste dag is geweest, uit de eigen opname van je station. Het vraagt je station, nooit het internet.

auto-stopped-reason = auto gestopt: { $reason }

auto-stopped-idle = auto gestopt: { $mins } minuten zonder invoer van de operator

auto-answering = { $role }: antwoordt { $call }

auto-caller-went-away = auto: een oproeper is weggegaan

auto-nobody-left = auto: niemand meer om te beantwoorden

auto-calling-cq = { $role }: roept CQ (#{ $n })

auto-cq-no-answer = auto cq: geen antwoord na { $limit } oproepen — luistert (auto rx)

auto-listening = { $role }: luistert

auto-role-stopped = { $role } gestopt: { $sentence }

auto-rx-listening = auto rx: luistert

auto-cq-calling = auto cq: roept CQ

auto-cont-running = auto cont: actief

aircraft-search-missing-online = { $missing } van { $total } vliegtuigen zonder onlinegegevens — open hun kaart

aircraft-search-shown-of = { $shown } van { $total }

aircraft-search-help =
    Doorzoek de al gehoorde vliegtuigen: het adres (ICAO), de roepletters, registratie, type, operator of categorie. Meerdere woorden moeten allemaal overeenkomen. Beperk een woord tot één veld met icao: call: reg: type: op: — bijvoorbeeld  reg:PH op:KLM.
    Type en operator komen uit de online opzoeking van het station, dus ze worden alleen gevonden voor vliegtuigen waarvan de kaart is opgezocht. De zoekopdracht vraagt nergens naar.

aircraft-search-hint = zoeken

aircraft-search-clear = Zoekopdracht wissen

gridtracker-shade-squares-heard-live-decode = Kleur wat op de live decodeerlijst wordt gehoord, niet alleen wat in het logboek staat. De versie van een luisteraar van de kaart: wat nu in de lucht is, in cyaan onder de gewerkte markeringen.

gridtracker-grid-ham = GRID/HAM

gridtracker-maidenhead-squares-what-a-ham-log = Maidenhead-vierkanten — wat een amateurlogboek bevat. Hier komen geen CB-roepnamen: een 11 m-uitwisseling bevat een land, nooit een locator.

gridtracker-country-cb = LAND/CB

gridtracker-dxcc-countries-at-their-nominal = DXCC-landen op hun nominale middelpunt — wat een CB-logboek bevat, aangezien een 11 m-uitwisseling geen locator heeft. Ook hier komen geen amateurroepletters: dit zijn landen, geen vierkanten.

colouredit-paste-a-colour-code-rrggbb-or-r-g = Plak een kleurcode (#RRGGBB of r, g, b) — of Cmd/Ctrl+V op het staal

cwkey-thread-failed = kon de CW-sleutelthread niet starten: { $error }

cwkey-grab-failed = kon de paddle niet exclusief overnemen — de contacten kunnen ook in andere vensters klikken (gebruikt een ander programma hem?)

ignore-muted-hover =
    Gedempt voor deze sessie: { $names }.
    
    Een gedempt station staat niet in de decodeerlijst, laat de meldingen niet klinken, wordt niet voorgelezen en wordt nooit aan de automodus aangeboden om te beantwoorden. Er wordt niets namens hem verzonden of geüpload, en de lijst wordt vergeten als sdroxide sluit — er is geen opgeslagen kopie, dus na een avond luisteren blijft er niets verborgen.
    
    Klik om ze allemaal te dempen op te heffen, of druk op de knop − bij een van hun rijen.

js8-setup-hint = SETUP — roepletters, groepen, het automatische antwoord, het baken-interval en het statusbericht

logbook-contest-logger-a-session-dupes-a = Contestlogger — een sessie, dupes, een live score en Cabrillo

logbook-up-chip = UP

net-wavelog-stopped = gestopt: { $error }

net-wavelog-download-stopped = Wavelog-download gestopt: { $error }

net-wavelog-new-already-here = { $added } nieuw, { $matched } al aanwezig

net-wavelog-fill-in-first = het adres en de API-sleutel moeten eerst worden ingevuld

net-wavelog-no-station-location = geen stationslocatie om van te downloaden — test eerst de verbinding

publicsdr-could-not-connect = kon niet verbinden in deze sessie

publicsdr-use-chip = GEBRUIK

weather-title = WEERRAPPORTEN

weather-intro = Gecodeerde weerrapporten van schepen en landstations, in RTTY op 4583, 7646 en 10100,8 kHz. SHIP gaat rond 02:00 en 07:35 UTC uit, SYNOP rond 00:35 en 12:10. Een rapport wordt hier bewaard in plaats van in de decodeerlijst, omdat de cijfers op zichzelf niets betekenen.

weather-nothing-decoded = Nog niets gedecodeerd. Er is niets anders te doen dan RTTY te laten draaien rond een van de tijden hierboven — dit zijn de rustigste uren op de band.

weather-head-call-day-hour = { $call } — { $day } om { $hour }:00 UTC

weather-head-day = om { $day }:00 UTC

weather-head-report = rapport

weather-not-reported = { $label }: niet gerapporteerd

weather-in-their-words = in hun woorden: { $remark }

weather-ago-now = zojuist

weather-ago-minute = een minuut geleden

weather-ago-minutes = { $m } minuten geleden

weather-ago-hours =
    { $h } { $h ->
            [one] uur
           *[other] uur
        } geleden

weather-ago-days = { $d } dagen geleden

sat-off-only-satellites-with = Uit: alleen satellieten met frequenties in het bestand — die je kunt werken — en alle die je zelf hebt geplakt. Aan: elke satelliet die het station volgt, wat meestal degene zijn die niemand kan horen.

adsbmap-tune-hint = dubbelklik om af te stemmen · shift voor de andere draaiknop

adsbmap-airport-large = grote luchthaven · veld { $elev }

adsbmap-airport-medium = middelgrote luchthaven · veld { $elev }

adsbmap-from-station = { $dist }  { $bear }° vanaf je station

adsbmap-aircraft-within =
    { $n } { $n ->
        [one] vliegtuig
       *[other] vliegtuigen
    } binnen { $dist }

adsbmap-navaid-mw = { $khz } kHz (middengolf)

adsbmap-click-to-close = klik op de kaart om te sluiten

settings-ui-broadcast-stations-hint =
    Welke omroepstations het LISTEN-venster en het waterval-label mogen tonen.
    
    { "**" }Op deze span** is wat elk scherm deed voordat dit een keuze was: een halve span aan weerszijden van de draaiknop, wat een label op die waterval hoort te dekken. **De hele band** toont alles wat het schema met een positie daarop heeft, wat een luisteraar die een band doorbladert zoekt — ten koste van een drukkere kaart.

settings-ui-waterfall-labels = Waterfall-labels

settings-ui-tuned-only = Alleen afgestemd

settings-ui-tuned-only-hint = Noem het station waarop je bent afgestemd, en niets anders — één naam, voor wat je beluistert, in plaats van een muur van namen over een doorlaatband.

settings-ui-whole-band = De hele band

settings-ui-whole-band-hint = Noem elke omroep die het schema nu in de lucht heeft, waar die ook zit. Wat je wilt tijdens het doorbladeren van een band in plaats van het beluisteren van één station, en daardoor drukker. Ook in het SPOTS-venster, als BC ALL.

settings-ui-classic-ui = Klassieke UI

settings-ui-classic-ui-hint =
    Zet alle bediening in één kolom links met inklapbare groepen, zoals SDR++ en SDR++Brown het indelen, en geef alle overige ruimte aan het beeld. Dezelfde bediening, dezelfde instellingen, gerangschikt zoals de meeste kortegolfluisteraars het al in handen hebben.
    
    Het vraagt een venster dat breed genoeg is om de waterval zijn deel te laten houden: daaronder blijft de kolom weg en wordt de strip gewoon getekend, en het paneel wordt breder of smaller gesleept aan de rechterrand. Uit wordt de eigen strip van dit programma precies zoals voorheen gebruikt.

settings-ui-classic-ui-check = bediening in een linkerkolom

settings-ui-updates = Updates

settings-ui-updates-check = bij het opstarten controleren op een nieuwe release

settings-ui-updates-hint = Vraagt eenmaal bij het opstarten de GitHub Releases van de fork om de laatste release, en toont een banner als die nieuwer is dan deze build. Alleen volledige releases tellen — de nightly is een pre-release — en uit betekent helemaal geen uitgaande aanroep.

settings-ui-undocked = Losgekoppeld — eigen venster

settings-ui-docked = Gedockt in dit venster

settings-ui-dock-all-hint = Breng elk losgekoppeld venster terug in dit venster.

settings-ui-undock-all-hint = Zet elke module in een eigen venster — het hoofdvenster blijft over als het spectrum en de decoders. Een module zonder iets om te tonen, zoals het bedieningspaneel in een spraakmodus, opent geen venster.

settings-ui-spectrum-line = Spectrumlijn

settings-ui-spectrum-line-hint = De kleur onder de spectrumlijn, en hoe die is geschaduwd: Vast is één kleur overal, Verloop is vol onderaan en vervaagt naar boven, Gemengd is vol in de onderste helft en vervaagt over de bovenste helft. De lijn zelf blijft cyaan. De FILL-chip in de SPEC-popup schakelt de vulling, en de lijnrij kiest de schaduw.

settings-ui-fill = Vulling

settings-ui-fill-colour = kleur

settings-ui-simple-ui = Eenvoudige UI

settings-ui-simple-ui-hint =
    Verberg de geavanceerde extra's uit de bovenste strip: de 3D-weergave, de skimmers, de laagschakelaars voor spectrum/waterval, awardtracking, satellieten, ISM-decodering en radio-e-mail. De bediening die een CB-operator of kortegolfluisteraar pakt — afstemmen, modus, volume, squelch, bandbreedte, de waterval, geheugens, scannen — blijft allemaal.
    
    Er wordt niets uitgezet, alleen verborgen: zet dit weer aan om de chips terug te halen.

settings-ui-simple-ui-check = geavanceerde chips verbergen

settings-ui-wake-lock = Wake lock

settings-ui-wake-lock-hint =
    Houd het scherm aan zolang deze pagina vooraan staat, zodat een telefoon of tablet die op de waterval ligt niet dimt en zichzelf vergrendelt na een minuut zonder aanraking. Ontvangst, audio en decodering hebben het niet nodig — ze gaan hoe dan ook door — het is zodat het scherm leesbaar blijft.
    
    Hoort bij het hangslot op de telefoon- en tabletstrips: vergrendel het scherm tegen verdwaalde vingers, en houd het verlicht.
    
    De browser neemt de lock terug zodra de pagina verborgen is — een ander tabblad, of de eigen aan/uit-knop van de telefoon — en hij wordt opnieuw gevraagd zodra de pagina terug is. Het vereist een browser die het heeft en een beveiligde pagina (https, of localhost); de regel naast het vakje meldt wanneer het niet beschikbaar is of is geweigerd. Het kost batterij, dus het staat uit totdat je erom vraagt. Het geïnstalleerde programma op een computer volgt in plaats daarvan de eigen energie-instellingen van het systeem.

settings-ui-wake-lock-check = scherm aan houden

settings-ui-sound-welcome = Geluidswelkom

settings-ui-sound-welcome-hint =
    Wanneer de webpagina opent — een nieuwe sessie, of een herlaad met F5 — speelt de browser geen geluid totdat je de pagina aanraakt. Dat is de regel van de browser, en zonder een woord op het scherm lijkt een verse sessie niet te werken.
    
    Met dit aan zegt een bericht midden op het scherm het: 🔇 Tik, klik of druk ergens op een toets om het geluid te starten. De eerste klik, tik of toets — overal, met een muis, een touchpad, een vinger of het toetsenbord — sluit het en start het geluid, en doet verder niets: het stemt niet af, drukt geen knop in en voert geen sneltoets uit. Het wordt alleen getoond wanneer de pagina opent, nooit meer tijdens de sessie, wat er later ook met het geluid gebeurt.
    
    Zet het uit zodra je weet wat je moet doen: het geluid wacht nog steeds op je eerste aanraking, en die aanraking werkt dan als gewoonlijk. Bewaard bij je profiel op de server. Standaard aan.

settings-ui-sound-welcome-check = welkomstbericht voor het geluid

settings-ui-sound-welcome-browser-only = alleen browser — het geïnstalleerde programma speelt geluid zonder aanraking

settings-ui-profile-screen = Scherm volgt je profiel

settings-ui-profile-screen-hint =
    Wanneer je een client op afstand bent van `sdroxide --server`, wordt het *uiterlijk* van dit scherm — thema, indeling, waterval en spectrum, lettertypen, Eenvoudige UI, de kaartlagen, de eigen weergave van de panadapter — bewaard bij het profiel waarmee je bent aangemeld, zodat het op elke machine terugkomt in plaats van met de standaardwaarden te beginnen.
    
    { "**" }Opgeslagen op de server, en opgeslagen terwijl je bezig bent zolang je bent aangemeld** — er hoeft niets te worden ingedrukt. Dat is bewust: browseropslag is niet te vertrouwen, omdat `persist()` in elke browser anders werkt, en een daar bewaarde sessie kan niet worden gereset door iemand die niet bij dat apparaat zit. Een profiel op de server is een bestand dat de operator van het station kan terugzetten.
    
    Op een server **zonder wachtwoord** is elke client hetzelfde profiel, dus er wordt niets geschreven tenzij je op de knop hieronder drukt — het thema van de ene operator mag niet dat van de volgende worden. Niets wat bij de machine hoort wordt ooit opgeslagen, en een lokale radio is niet betrokken.

settings-ui-save-to-profile = Opslaan in profiel

settings-ui-back-to-profile = Terug naar profiel

settings-ui-carry-bindings = Bedieningskoppelingen meenemen

settings-ui-carry-bindings-hint =
    Bewaar je toetsenbord- en muiskoppelingen ook bij het profiel op de server, zodat ze de login volgen tussen browsers en apparaten — hetzelfde idee als het scherm hierboven, voor de toetsen.
    
    { "**" }AFGERADEN, en standaard uit.** Op een station dat meerdere mensen delen, wordt het toetsenbord gedeeld: een profiel met koppelingen kan de PTT-, Spatie- of afstemtoetsen van een andere operator opnieuw toewijzen zodra het aanmeldt, en het inschakelen kan andere effecten hebben die nu niet duidelijk zijn. Zet het alleen aan waar de server van jou alleen is en je er vanaf meerdere eigen apparaten bij komt.
    
    Alleen in de fork: dit is geen upstream-functie. De scherminstellingen hierboven reizen altijd mee; dit is de enige opt-in hier.

settings-ui-carry-bindings-on = aan — AFGERADEN

settings-ui-carry-bindings-off = meenemen (afgeraden)

settings-ui-carry-bindings-title = Bedieningskoppelingen op de server meenemen?

settings-ui-carry-bindings-body =
    Dit bewaart je toetsenbord- en muiskoppelingen bij je profiel op de server, zodat ze je login volgen tussen browsers en apparaten.
    
    Op een station dat anderen gebruiken, wordt het toetsenbord gedeeld. Een profiel met koppelingen kan de PTT-, Spatie- of afstemtoetsen van een andere operator opnieuw toewijzen door simpelweg aan te melden — en het inschakelen kan andere effecten hebben die nu niet duidelijk zijn.
    
    Daarom wordt het afgeraden. Zet het alleen aan als de server van jou alleen is.

settings-ui-carry-bindings-confirm = Ik begrijp het — toch meenemen

settings-ui-start-swl = Starten in SWL-modus

settings-ui-start-swl-hint =
    Open elke sessie met SWL-modus al aan, zodat een luisterscherm is wat het programma als eerste toont — voor elke radio, omdat dit de voorkeur van het scherm is en niet die van een radio.
    
    De schakelaar per radio staat in Instellingen → Radio → Zendbediening, dus een station met één luisterradio en één transceiver zet daar de luisterende aan en laat dit uit. Het gebruik van die schakelaar zet dit zaad voor de sessie uit. `--swl` wint altijd.

settings-ui-start-swl-check = starten met SWL-modus aan

settings-ui-listen-hides-tx = LISTEN-tabblad verbergt zenden

settings-ui-listen-hides-tx-hint =
    Als je **LISTEN** kiest in het band/modus-menu, verbergt dat elke zendbediening op dat tabblad, en als je **OPERATE** kiest komt terug wat de eigen instelling van deze radio zegt.
    
    Het kan geen zender aanzetten. Een radio die alleen-ontvangen is geconfigureerd blijft alleen-ontvangen, en de bandvergrendeling en de waarschuwing buiten de band blijven onaangeroerd.
    
    Het bandmenu onthoudt op welk tabblad het is achtergelaten, dus een station dat op luisteren is achtergelaten komt terug op luisteren — de zendbediening is ook bij de volgende start verborgen, en OPERATE kiezen brengt hem terug. Standaard uit omdat het verandert wat het tabblad betekent; het vraagt één keer.

settings-ui-listen-hides-tx-check = TX verbergen op het LISTEN-tabblad

settings-ui-listen-hides-tx-title = Het LISTEN-tabblad verbergt de zendbediening

settings-ui-listen-hides-tx-body =
    Op het LISTEN-tabblad verdwijnt de zendbediening; op OPERATE komt hij precies terug zoals de eigen instelling van deze radio hem achterliet.
    
    Het schakelt niets in. Het is een weergave van hetzelfde station, dus de bandvergrendeling, de waarschuwing buiten de band en de zendrails blijven onaangeroerd.
    
    Het tabblad wordt onthouden: een station dat op LISTEN is achtergelaten start op LISTEN, met de zendbediening nog steeds verborgen, totdat OPERATE wordt gekozen.
    
    Goed om te weten: een station dat op LISTEN is achtergelaten ontvangt nog steeds op wat het tabblad heeft afgestemd, en een bediening die verborgen is, is geen bediening die uit staat — zenden is uit omdat de instellingen dat zeggen, niet omdat de knop uit het zicht is.

settings-ui-listen-hides-tx-confirm = Ik begrijp het — verberg ze op LISTEN

settings-ui-swl-mode = SWL-modus

settings-ui-swl-mode-hint =
    Verberg alle zendbediening voor deze radio — de PTT, CALL CQ, TX-niveau, SEND, BEACON, alles — en vervang de ham-extra's van de strip (spots, awards) door die van de luisteraar (SCHEDULE en SWL LOG, het ontvangstlogboek). Dezelfde schakelaar als Instellingen → Radio → Zendbediening, en per radio: een luisterdongle kan in deze modus staan terwijl de transceiver ernaast zijn zender behoudt.
    
    De hardware kan nog steeds zenden; dit verbergt alleen de bediening. `--swl` forceert het aan voor elke radio voor de duur van de run.
    
    Wordt van kracht bij Toepassen.

settings-ui-swl-mode-check = alle zendbediening verbergen

settings-ui-report-picture = Rapportafbeelding

settings-ui-report-picture-hint =
    De afbeelding die een ontvangstrapport bevat wanneer je het naar een omroep mailt — je eigen kaart, een foto, wat je het station ook wilt laten zien. Eén afbeelding voor het station, gebruikt voor elk rapport.
    
    Laat het leeg en het rapport gaat als platte tekst, wat een prima rapport is.

settings-ui-report-picture-none = geen — het rapport gaat als tekst

settings-ui-choose = Kiezen…

settings-ui-report-picture-pick = Kies de afbeelding op deze computer

settings-ui-report-note = Rapportnotitie

settings-ui-report-note-hint = Je vaste notitie, vooraf ingevuld in elk ontvangstrapport — wie je bent, waarop je luistert, alles wat een station eenmaal vraagt in plaats van per brief. Platte tekst in een plat bestand: geen geheim.

settings-ui-report-note-hint-text = optioneel — bijv. "Details op aanvraag"

sstv-keep-dial = DRAAIKNOP BEHOUDEN

sstv-keep-dial-on-hint = Aan: SSTV kiezen laat de draaiknop waar je hem hebt afgestemd, dus de decoder werkt op elke frequentie. De gepubliceerde SSTV-frequenties staan nog steeds in de ⇵-kiezer. Klik om terug te gaan naar landen op de SSTV-frequentie van de band.

sstv-keep-dial-off-hint = Uit: SSTV kiezen verplaatst de draaiknop naar de SSTV-frequentie van de band als je meer dan 3 kHz van een af zit. Klik om de draaiknop te laten waar je hem hebt afgestemd, en SSTV op elke frequentie te decoderen. Afstemmen na het kiezen van SSTV wordt hoe dan ook nooit ongedaan gemaakt.

sstv-clear-rx = RX wissen

sstv-clear-rx-hint = Wis de afbeelding die nu in de LIVE-weergave staat. De decoder blijft draaien — gebruik RX herstarten om een afbeelding in uitvoering te laten varen en op de volgende header te luisteren.

sstv-audio-present-no-header = audio aanwezig · nog geen SSTV-header

sstv-editor = Editor…

sstv-editor-hint = Stel de afbeelding zelf samen: verplaatsbare tekst, elk lettertype, een strook over of langs het beeld

sstv-save-image-as = Afbeelding opslaan als…

sstv-save-copy-asks-file = Sla een kopie op deze computer op — er wordt gevraagd welk bestand

sstv-picture-still-loading = De afbeelding wordt nog geladen

sstv-share-hint-direct = Stuur deze afbeelding naar iemand — WhatsApp, Telegram, of iets anders op dit apparaat

sstv-share-hint-save = Sla deze afbeelding op en open een bericht om hem te verzenden

sstv-share-picture = Afbeelding delen…

sstv-reupload-hint = Stuur deze afbeelding weer uit — laadt zendslot { $slot }

sstv-radio-cannot-transmit = Deze radio kan niet zenden

sstv-reupload = Opnieuw uploaden

sstv-unidentified = niet geïdentificeerd

sstv-send-picture = Deze afbeelding verzenden

sstv-qsl-to = Aan

sstv-the-card = De kaart

sstv-the-card-hint = Deze komen op een gedrukte kaart naast de afbeelding. Laat er een leeg en hij wordt gewoon niet getekend — een kaart meldt nooit een rapport dat niemand heeft gegeven.

sstv-date-utc = Datum (UTC)

sstv-rst-sent = RST verzonden

sstv-rst-received = RST ontvangen

sstv-qsl-note = Notitie

sstv-add-address = Voeg een adres toe om dit te verzenden.

sstv-saves-opens-mail-hint = Slaat de afbeelding op en opent je mailprogramma — daar bijvoegen is één klik.

sstv-browser-cannot-attach-hint = Een browsertabblad kan geen bestand aan een bericht hechten, dus dit slaat de afbeelding op en opent het bericht; voeg de bijlage handmatig toe.

sstv-save-open-mail = Afbeelding opslaan & mail openen

sstv-mail-not-opened = Afbeelding opgeslagen en het bericht gekopieerd — plak het in je mailprogramma. Er kon geen mailprogramma worden geopend.

sstv-qsl-saved-where-chosen = { $name } (opgeslagen waar je koos)

sstv-qsl-downloaded = { $name } — gedownload naar de downloadmap van deze browser

sstv-picture-editor = Afbeeldingseditor

sstv-slot-n = Slot { $n }

sstv-slot-layers-hint = De eigen lagen van dit zendslot

sstv-station-overlay = Stationsoverlay

sstv-station-overlay-hint = Eén set lagen die elk slot onder zijn eigen lagen tekent — de roepletters, in de hoek die je wilt. Eenmaal ingesteld, samengesteld met de eigen tekst van elk slot in plaats van die te vervangen.

sstv-draw-under-slot = onder dit slot tekenen

sstv-draw-under-slot-hint = Teken de stationsoverlay onder de eigen lagen van dit slot. Uit voor een afbeelding die de identificatie al bevat — meestal een opnieuw uitgezonden mozaïek — zodat die niet twee keer wordt gestempeld.

sstv-overlay-note = De stationsoverlay: elk slot tekent deze lagen onder zijn eigen lagen. Sleep een regel op de afbeelding om hem te verplaatsen.

sstv-slot-text-note = De eigen tekst van dit slot, getekend waar je hem zet. Sleep een regel op de afbeelding om hem te verplaatsen; de strook van een regel beweegt mee.

sstv-no-picture-in-slot = Geen afbeelding in dit slot — laad er hieronder een

sstv-base = Basis

sstv-base-hint = Over welke slotafbeelding deze lagen worden getekend

sstv-compose-over-hint = Samenstellen over de afbeelding van dit slot

sstv-slot-empty = Dit slot is leeg

sstv-picture-btn = Afbeelding…

sstv-load-picture-hint = Laad een afbeelding in dit slot

sstv-use-received = Ontvangen gebruiken

sstv-use-received-hint = Zet de zojuist aangekomen afbeelding onder deze lagen, zodat een antwoord zijn afbeelding en jouw roepletters bevat

sstv-received-now-base = het ontvangen beeld is nu de basis

sstv-add-text = ＋ Tekst

sstv-add-strip = ＋ Strook

sstv-n-lines =
    { $n } { $n ->
            [one] regel
           *[other] regels
        }

sstv-draw-this-line = Deze regel tekenen

sstv-strip-paren = (strook)

sstv-draw-later-hint = Teken deze regel later dan de regels erboven

sstv-draw-earlier-hint = Teken deze regel eerder

sstv-copy = Kopiëren

sstv-face = Lettertype

sstv-colour = Kleur

sstv-line-text-hint = wat de regel zegt

sstv-the-letters = De letters

sstv-composer-outline = omlijning

sstv-the-edge = De rand eromheen

sstv-placement = Plaatsing

sstv-down-frame = langs het kader

sstv-down-frame-hint = Draai de regel een kwartslag, zodat hij naar beneden leest — een roepnaam langs de zijkant van het beeld

sstv-strip = Strook

sstv-behind = erachter

sstv-behind-hint = Een kleurstrook achter de regel, over het beeld of eronder langs, gecentreerd op de regel

sstv-strip-first-edge = De strook, aan de eerste rand

sstv-thickness = Dikte

adsbpanel-paths = PADEN

adsbpanel-paths-hint = Teken het pad van elke vlucht als één lijn — waar het toestel geweest is, waar het nu is en waar het heen gaat. Uit wordt het spoor als een stippellijn van puntjes getekend.

adsbpanel-nav = NAV

adsbpanel-nav-hint = Luchthavens en radionavigatiehulpen onder het beeld, uit de ingebouwde tabellen — grote luchthavens altijd, middelgrote dichterbij, en de navaids zodra de weergave ver genoeg is ingezoomd dat een station een plaats is. De middengolfbakens, die je kunt horen, worden in hun eigen kleur getekend.

adsbpanel-units = Eenheden

adsbpanel-units-hint = Hoe hoogte, snelheid, stijgsnelheid en afstand in de vliegtuigweergaven worden geschreven. De decoder en de exports worden niet beïnvloed.

adsbpanel-age = LEEFTIJD

touchlock-locked-hover = Scherm vergrendeld tegen onbedoelde aanrakingen. Houd hier 3 seconden ingedrukt om te ontgrendelen.

touchlock-unlocked-hover = Vergrendel het scherm tegen onbedoelde aanrakingen: frequentie, band, modus, filters en de waterfall reageren niet meer op een vinger. Ontvangst en audio gaan door. Houd 3 seconden ingedrukt om te ontgrendelen. Na het ontgrendelen rust het hangslot 10 seconden voordat een tik het weer vergrendelt, zodat de vinger die het ontgrendelde het niet meteen terug kan vergrendelen.

touchlock-hold = HOUD { $s } s…

touchlock-unlocked = ONTGRENDELD

touchlock-locked-hold = VERGRENDELD · houd 3 s

touchlock-unlocked-relock = ONTGRENDELD · vergrendel over { $s } s

wake-held = scherm blijft aan

wake-asking = de browser vragen…

wake-unsupported = hier niet beschikbaar — vereist een browser die dit kan en een https-pagina

wake-refused = geweigerd door de browser (energiebesparing?)

wake-native = alleen browser — het geïnstalleerde programma volgt de energie-instellingen van het systeem

tooltipcopy-copied = ✔ Gekopieerd

chrome-hide-message-hover = Verberg dit bericht voorgoed. Er verandert niets aan je instellingen, en het advies staat in de handleiding.

chrome-close-message-hover = Sluit dit bericht. Er verandert verder niets.

settings-ui-window-label = Venster { $module }

settings-ui-undock-hint =
    Losgekoppeld verhuist deze module naar een eigen venster en geeft het hoofdvenster de ruimte terug aan wat overblijft — zet de waterfall op een tweede monitor, of de bediening op een derde. Het venster sluiten koppelt de module weer vast.

    Onder Wayland komt de grootte terug, maar de compositor bepaalt waar een venster komt: laat het zweven en zet het vast op een monitor met een vensterregel die overeenkomt met app-id "{ $app_id }".

settings-ui-workspace-restore-out = Zet deze indeling terug — { $n } module(s) in eigen venster(s)

settings-ui-profile-in-use-named = in gebruik: het profiel { $name }

settings-ui-profile-in-use-default = in gebruik: de standaard van dit station

settings-ui-profile-not-in-use = nog niet in gebruik — niets opgeslagen voor deze login

settings-ui-module-panadapter = Panadapter

settings-ui-module-controls = Bediening

settings-ui-module-aux = AUX SP (tweede spectrum)

settings-ui-module-band-menu = Band- en modusselectie

settings-ui-broadcast-scope-in-view = Op dit bereik

settings-ui-fill-fixed = Vast

settings-ui-fill-gradient = Verloop

settings-ui-fill-mixed = Gemengd

settings-ui-spot-kind-heard-me = HEEFT MIJ GEHOORD

settings-ui-wake-held = scherm blijft aan

settings-ui-wake-asking = de browser vragen…

settings-ui-wake-unsupported = hier niet beschikbaar — vereist een browser die dit kan en een https-pagina

settings-ui-wake-refused = geweigerd door de browser (energiebesparing?)

settings-ui-wake-native = alleen browser — het geïnstalleerde programma volgt de energie-instellingen van het systeem

settings-ui-workspaces = Werkruimtes

settings-ui-workspaces-note = Een benoemde vensterindeling — welke modules in eigen vensters staan en waar die vensters zijn. Sla de huidige op, of zet een opgeslagen indeling terug. Maximaal tien.

settings-ui-workspaces-none = Nog geen werkruimtes opgeslagen.

settings-ui-workspace-restore-home = Zet deze indeling terug — elke module in het hoofdvenster

settings-ui-workspace-forget = Deze werkruimte vergeten

settings-ui-workspace-name = geef deze indeling een naam

settings-ui-workspace-save = HUIDIGE OPSLAAN

settings-ui-workspace-full = Tien werkruimtes is de limiet die SDRuno bewaart; verwijder er eerst een

settings-ui-workspace-save-hint = Sla de huidige indeling onder deze naam op

settings-ui-workspace-name-first = Typ eerst een naam

sstv-composer-fade = vervagen

sstv-layouts = LAYOUTS

sstv-name-layout-hint = geef deze layout een naam

sstv-keep-layout-hint = Bewaar deze layout onder die naam

sstv-layout-renamed = hernoemd naar "{ $name }"

sstv-layout-saved-as = opgeslagen als "{ $name }"

sstv-rename-cancelled = hernoemen geannuleerd

sstv-no-layouts-yet = Nog geen opgeslagen layouts — geef hierboven een naam aan een layout.

sstv-load-layout-hint = Laad deze layout in wat je aan het bewerken bent

sstv-layout-loaded = "{ $name }" geladen

sstv-rename-layout-hint = "{ $name }" hernoemen

sstv-layout-renaming = "{ $name }" wordt hernoemd

sstv-duplicate-layout-hint = "{ $name }" dupliceren

sstv-layout-copy-name = { $name } kopie

sstv-layout-duplicated = gedupliceerd als "{ $name }"

sstv-forget-layout-hint = De layout "{ $name }" vergeten

sstv-layout-forgot = "{ $name }" vergeten

sstv-mosaic-header = Mozaïek van ontvangen beelden

sstv-mosaic-nothing = Nog niets ontvangen — een mozaïek wordt opgebouwd uit beelden in de galerij.

sstv-mosaic-picked = { $n } van { $max } gekozen, in deze volgorde neergelegd — de volgorde is de rasterpositie. Elk beeld behoudt zijn vorm; de tussenruimtes zijn zwart.

sstv-mosaic-earlier-hint = Verplaats dit beeld één cel eerder

sstv-mosaic-later-hint = Verplaats dit beeld één cel later

sstv-mosaic-max = { $n } is het maximum van een mozaïek

sstv-mosaic-compose = In dit slot samenstellen

sstv-mosaic-compose-hint = Bouw het mozaïek en laad het als beeld in dit slot; de ontvangen originelen blijven ongewijzigd

sstv-mosaic-composed = { $n } beelden samengesteld in slot { $slot }

sstv-clear-picks = Keuze wissen

sstv-composed-saved = opgeslagen

sstv-send = Verzenden

sstv-send-now-hint = Zend deze compositie nu uit

sstv-composed-sending = wordt verzonden

sstv-forget = Vergeten

sstv-forget-hint = Maak het document van deze preset leeg — de bannerstrook en het gewone onderschrift bepalen weer het beeld

sstv-forget-done = gewist — de banner en het onderschrift zijn terug

sstv-copy-to-slot = Kopiëren naar slot

sstv-copy-to-slot-hint = Geef een ander slot dit beeld en deze tekst, zodat het van daaruit verzonden kan worden

sstv-copied-into-slot = gekopieerd naar slot { $n }

sstv-word-strip = strook

sstv-word-text = tekst

sstv-text-size = Tekstgrootte

sstv-text-size-hint = De bannertekst als percentage van de eigen grootte van de strook. 100% is wat het altijd was; een QSO heeft een leesbare roepnaam nodig na een rumoerige decodering, en dit schaalt de letters zonder de strook hoger te maken.

sstv-placement-hint = Langs welke rand de strook wordt getekend. De berichtoverlay begint aan het tegenoverliggende uiteinde, dus een banner onderaan laat de bovenkant van het beeld vrij voor tekst.

sstv-top = Boven

sstv-bottom = Onder

topbar-ptt-hold-hint = Ingedrukt houden om te zenden — een klik houdt het vast, en de volgende druk laat los


topbar-above-the-dial = boven

topbar-below-the-dial = onder

topbar-step-chip-hint = Hoe ver één druk de afstemming verplaatst. Tik voor de volgende stap: 10 Hz, 100 Hz, 500 Hz, 1, 2.5, 5, 6.25, 9, 10, 12.5 en 25 kHz. Zet de rij uit in Instellingen › UI.

topbar-mem-chip-hint = Geheugenkanalen — roep een opgeslagen frequentie op

topbar-disp-menu-hint = De panadapter — zijn twee lagen, hun snelheden en detail, peak hold — plus waterfall-niveaus, de weergave-opties en de skimmers

topbar-phone-menu-hint = Alles wat de telefoonbalk niet toont: banden, modi, de ontvanger en zender, de decodeervensters, instellingen en de handleiding

topbar-group-band = Band

topbar-group-system = Systeem

topbar-caption-diversity = Diversity

topbar-group-decode-windows = Decodeervensters

topbar-group-extras = Extra's

topbar-grid-tracker-hint = Gridtracker — gewerkte Maidenhead-vierkanten op een kaart

topbar-settings-chip-hint = Instellingen — versterkingen van het apparaat, antennes, audioapparaten

topbar-windows-chip-hint =
    ▣  Links: breng elk modulevenster naar huis
    Rechts: de lijst met modulevensters

topbar-manual-chip-hint = Handleiding (F1)

topbar-module-windows-title = Modulevensters

topbar-wname-panadapter = SP1 — het spectrum

topbar-wname-aux = SP2 (AUX) — een tweede spectrum

topbar-wname-panel = Bedieningspaneel

topbar-wname-controls = RX-bediening

topbar-wname-band-menu = Band en modus

topbar-wname-rotor = Antennerotor

topbar-wstate-put-back = Zet { $name } terug in dit venster

topbar-wstate-floating-panel-only = Zweven is de eigen toestand van het bedieningspaneel — een spectrum of een console heeft één plek in dit venster

topbar-wstate-float = Til { $name } uit het paneel en laat het zweven boven dit venster

topbar-wstate-no-panel = Geen bedieningspaneel in { $mode } — dit geldt in een modus die er een heeft

topbar-wstate-own-window = Geef { $name } een eigen venster, op elke monitor

topbar-wstate-here-now = { $name } is nu hier

topbar-dock-all = ALLES VASTZETTEN

topbar-dock-all-hint = Breng elk modulevenster terug in dit venster

topbar-undock-all = ALLES LOSKOPPELEN

topbar-undock-all-hint = Zet elke module in een eigen venster — dit venster houdt alleen het spectrum en de decoders

topbar-rx-tx-freq = RX { $rx }   TX { $tx } MHz

topbar-band-dock-hint = VASTZETTEN — houd de band- en modusselectie open naast de waterfall in plaats van dit pop-upvenster na elke keuze te sluiten

topbar-band-window-hint = VENSTER — open de band- en modusselectie in een eigen venster, zo breed als je wilt, op elke monitor naast de waterfall

topbar-band-return-popup-hint = VASTZETTEN — zet de selectie terug in de pop-up van de bovenbalk, waar hij na elke keuze sluit

topbar-band-window2-hint = VENSTER — open de selectie in een eigen venster, zo breed als je wilt, op elke monitor

topbar-squelch-held-hint = De squelch wordt open gehouden. Klik om de poort terug te zetten zoals je die had.

topbar-squelch-hold-hint = Klik om de squelch open te houden — om te horen of er iets is — en klik nogmaals om hem precies terug te zetten zoals hij was.

topbar-changed-tone = toon

topbar-changed-loudness = LOUDNESS

topbar-nr-chip-hint =
    Ruisonderdrukking: { $strength } — { $engine }
    
    { $advice }
    
    Klik om de sterkte te doorlopen: Laag / Middel / Hoog / Uit. De engine staat in de rij hieronder.

topbar-nr-off-narrow-hint = Ruisonderdrukking is uit. In de rij hieronder worden vijf ruisfilters aangeboden — RNNoise, DeepFilterNet3, spectral-bleach, NR2 en de originele spectrale — en een klik hier zet het aan met een bepaalde sterkte. Begin met RNNoise bij spraak.

topbar-eq-tone-hover = Toon: bas { $bass } dB, midden { $mid } dB, hoog { $treble } dB

topbar-eq-closed-hover = Toon — een equalizer met drie banden op de ontvangstaudio. Klik om te openen

topbar-eq-loudness-on = LOUDNESS aan

topbar-stereo-yields-hint = WFM-stereo is uit zolang ruisonderdrukking of de auto-notch actief is — ze zouden de twee kanalen uitsmeren. Zet ze uit om stereo te horen

topbar-stereo-playback-hint = Stereo: de opname wordt in beide oren afgespeeld, zoals opgenomen. Klik om mono te forceren

topbar-stereo-mono-recording-hint = Deze opname is mono — er is geen tweede kanaal om af te spelen

topbar-preset-edges = { $lo } … { $hi } Hz

topbar-caption-tone = Toon

topbar-eq-on = aan

topbar-eq-bass = Bas

topbar-eq-mid = Midden

topbar-eq-treble = Hoog

topbar-eq-remembered = Onthouden voor { $mode } — elke modus houdt zijn eigen instelling.

topbar-loudness-hint = Versterkt de bas (en een beetje de hoge tonen) als je het volume lager zet, zodat een zachte luidspreker toch vol klinkt. Werkt met of zonder de toon; doet niets bij vol volume en speelt nooit luider dan vol volume. Standaard uit, en onthouden voor de modus waarin je zit.

topbar-caption-record-audio = Audio opnemen

topbar-stop-recording-after-min = Stop de opname na { $minutes } minuten

topbar-caption-play-back = Afspelen

topbar-recordings-chip = ▶ OPNAMES…

topbar-recordings-hint = Speel een opname terug in de decoders — een gemist SSTV-beeld, een zwak RTTY- of CW-signaal, een FT8-slot — met de waterfall en het decodeerlogboek, alsof het in de ether was. De radio houdt zich stil terwijl het speelt.

topbar-ant-label = ANT

topbar-pwr-label = PWR

topbar-drive-less = minder

topbar-drive-more = meer

topbar-fill-hint = Kleur het gebied onder de spectrumlijn, vol onderaan en vervagend naarmate het naar de lijn stijgt, zodat een signaal opvalt door zijn massa en niet alleen door zijn omtrek. De kleur kies je in Instellingen → UI → Spectrumlijn. Niet getekend op het 3D-oppervlak.

topbar-group-spectrum = Spectrum

topbar-group-waterfall = Waterfall

topbar-freeze-on-tx-hint = Laat de waterfall niet scrollen zolang je zelf zendt, en ga verder waar hij gebleven was — zodat het beeld doorlopende ontvangen geschiedenis is in plaats van een blok rijen met je draaggolf (of, bij een radio die de ontvanger dempt tijdens het zenden, rijen ruis die nooit bestonden). De spectrumlijn blijft ongemoeid. Handig bij een modus waarin je vaak zendt, zodat de beurt van het andere station niet van het scherm wordt geduwd door de jouwe.

topbar-fft-chip-hint = Waterfall-niveaus en contrast, FFT-grootte en de scrollrichting

topbar-caption-waterfall = Waterfall

topbar-flatten-hint = Maak de ruisvloer over de band vlak: een stuk dat stiller is dan zijn buren wordt naar hun vloer opgetrokken, zodat een zwak signaal zichtbaar wordt op een drukke band. Het verhoogt alleen — een draaggolf wordt nooit omlaag gedrukt — en nooit met meer dan 20 dB. De spectrumtrace erboven blijft precies zoals gemeten.

topbar-log-swl-hover = Ontvangstlogboek — gehoorde stations, met SINPO/SIO

topbar-programs-hint = Uitzendschema — wat er is, wanneer en waar

topbar-listen-hint = Wat er op dit bereik is, op een kaart — kies een station, stem erop af, schrijf op wat je hoorde en log het

topbar-spots-hint = Live spots — PSK Reporter (en, buiten de SWL-modus, de DX-cluster, POTA en SOTA)

topbar-sigid-hint = Wat is er op deze frequentie? Een gids voor signalen op frequentie, modus en bandbreedte

topbar-extras-hint = EXTRAS — DAB en de andere ontvangers, in een venster dat je kunt losmaken

topbar-fill-style-caption = lijn

topbar-fill-style-hint = Hoe de kleur onder de spectrumlijn wordt gearceerd. De kleur zelf kies je in Instellingen → UI → Spectrumlijn.

topbar-fill-fixed = VAST

topbar-fill-fixed-hint = Eén kleur, overal gelijk onder de lijn

topbar-fill-gradient = VERLOOP

topbar-fill-gradient-hint = Vol onderaan het spectrum, vervagend naar boven

topbar-fill-mixed = GEMENGD

topbar-fill-mixed-hint = Vol in de onderste helft, vervagend over de bovenste helft

topbar-fill-switch-on = Zet FILL aan om dit in te stellen

topbar-nothing = niets

topbar-olivia-unconfirmed = OLIVIA — ontvangen werkt, zenden is niet getest tegen andere programma's. Deze decoder leest andere stations (bevestigd in de ether), dus je kunt een Olivia-QSO meelezen. Onze eigen uitzending gaat nu rond via onze eigen ontvanger, maar is niet gelezen door fldigi of MultiPSK en één tekenconventie verschilt van de broncode van fldigi — controleer dus de eerste beurt in plaats van een antwoord te verwachten.

topbar-dab-bandwidth = DAB / DAB+, in de ether bewezen. Het heeft 2.5 Msps bandbreedte aan de front-end nodig voordat er audio is: bij de 1.536 MHz die een ensemble inneemt zit de ontvanger op zijn ADC-ondergrens en verliest samples, en een DAB-decodering heeft een ononderbroken stroom nodig, dus het ensemble vergrendelt nooit en je hoort niets. Stel eerst de samplerate van de radio in op 2.500 Msps.

topbar-stanag-unproven = STANAG 4285 — alleen ontvangen. Identificatie is in de ether bewezen: een 4285 vergrendelt, zijn frame van 106.67 ms wordt gemeten en zijn referentieblokken vallen samen, zodat het paneel kan zeggen wat er op de frequentie is en met welke snelheid. Het lezen van de *tekst* is niet in de ether bewezen: de Annex E-code en de interleaver decoderen een gegenereerd signaal exact, maar nog geen gecodeerd station is gelezen. Een gecodeerde tekst die wel verschijnt is echt; helemaal geen tekst betekent dat het station ongecodeerd is of versleuteld verkeer voert, wat het gebruikelijke geval is.

topbar-cis11-unproven = CIS-11 (TORG-11) — alleen ontvangen. De Russische meteorologische teleprinter: 2-FSK op 100 baud, ongeveer 500 Hz shift, en een 11-bits teken met een oneven pariteitsbit. Wat bewezen is, is gemeten op echte opnames: de ontvanger vindt zijn eigen tonen en leest een inactieve uitzending met 121 van 123 tekens met geldige pariteit. Wat niet vaststaat, is hoe de tien databits in tekens verdeeld zijn, dus het paneel toont de bits naast zijn ITA-2-lezing — en het verkeer zelf is versleuteld, wat het gebruikelijke geval is.

topbar-thor-unproven = THOR — de ontvangstkant leest fldigi-uitzendingen nog niet. De decoder werkt tegen zijn eigen modulator, maar er is geen opname uit de ether of referentieopname gelezen, dus beschouw wat verschijnt als voorlopig.

topbar-mode-not-on-band = { $mode } wordt niet gebruikt op { $band } — kies een band waar het bij hoort

topbar-radio-classic-hint = RADIO CLASSIC — een klassiek radiogezicht voor omroepluisteren. Als je het aanzet, krijgt het de ontvanger: de andere modi en hun vensters worden gesloten

topbar-ats-bands-caption = Banden die de ontvanger biedt

topbar-ats-band-hover = { $name } — { $mhz } MHz, { $mode } op zijn standaard. Zet de ontvanger op deze band.

topbar-caption-mode = Modus

topbar-caption-digital = Digitaal

topbar-ats-receive-only = Alleen ontvangen — de Si4732 demoduleert in hardware en er is niets om te zenden.

topbar-band-forecast-derived =
    { $band }: ≈{ $verdict } ({ $time ->
            [day] overdag
           *[night] 's nachts
        }) — het oordeel van de gepubliceerde groep { $group }, de dichtstbijzijnde vervanger; HAMQSL.com beoordeelt niets voor { $band } zelf. Een voorspelling, geen meting van je eigen pad.

topbar-keypad-readout-khz = { $digits } kHz

topbar-caption-band-keypad = Bandtoetsen

topbar-keypad-bands = Banden

topbar-keypad-bands-hint = De toetsen noemen banden: druk er een in om erheen te gaan, in de modus waarin je zit. Terugkeren van MHz wist wat getypt was.

topbar-keypad-mhz-hint = Typ een frequentie op de toetsen, in kilohertz: 14074 is 14.074 MHz. ENTER verstuurt hem, CLEAR wist de invoer.

topbar-keypad-key-stay = Blijf in { $mode } en stem af op { $mhz } MHz — waar het op { $band } wordt gewerkt.

topbar-keypad-key-go = Ga naar { $band }.

topbar-keypad-clear = Wissen

topbar-keypad-clear-typing-hint = Wis de frequentie die je hebt getypt. De toetsen blijven in MHz.

topbar-keypad-clear-bands-hint = Er is niets getypt — de toetsen staan op banden, waar een druk naar een band gaat.

topbar-keypad-enter = Enter

topbar-keypad-enter-empty-hint = Nog niets getypt. De toetsen staan in MHz — druk ze eerst in.

topbar-keypad-enter-ok-hint = Stem af op { $mhz } MHz en ga terug naar de bandtoetsen.

topbar-keypad-enter-range-hint = { $mhz } MHz valt buiten wat deze radio ontvangt ({ $ranges } MHz).

topbar-nothing-published = niets gepubliceerd

topbar-tab-listen = LUISTEREN

topbar-tab-operate = BEDIENEN

topbar-listen-tab-note = Dit tabblad kan ook de zendbediening verbergen: zet "TX verbergen op het tabblad LUISTEREN" aan in Instellingen → UI

topbar-caption-show-bands = Banden tonen

topbar-band-filter-hint = Toon alleen de { $filter }-banden in de lijst hieronder. Klik nogmaals om elke band te tonen.

topbar-show-every-band = toon elke band

topbar-all-chip = ALLES

topbar-all-chip-hint = Algemene dekking: wis de band en laat de afstemming overal heen gaan. Niet hetzelfde als het bandfilter, dat alleen deze lijst beperkt.

topbar-caption-sw-metre-bands = KG-meterbanden

topbar-metre-band-hint = Stem af op het midden van de { $name }-omroep

topbar-caption-wideband = Breedband

topbar-caption-cb = 11M/CB

topbar-caption-experimental = Experimenteel (onbewezen)

topbar-caption-receive-modes = Ontvangstmodi

topbar-wav-hint = Dezelfde audio als ongecomprimeerde 16-bit PCM op 48 kHz — ongeveer 11 MB per minuut in stereo. Voor een editor of een decoder van jezelf.

topbar-mode-offset-hint-side = Deze modus werkt het signaal { $side } de afstemming. Het getal is de afstemming, want dat stem je af; het signaal zit op { $mhz } MHz.

levelslider-hover =
    Waterfall-niveau
    vloer { $floor } dB · plafond { $ceil } dB
    
    Sleep of scrol om helderder of donkerder te maken; shift voor contrast.
    Verplaatsen zet auto-fit (FIT) uit.

smeter-audio-level-hover =
    Audioniveau: { $dbfs } dBFS{ $over }.
    
    Deze radio heeft **geen eigen S-meter om te rapporteren**, dus dit is het niveau van de audio die hij stuurt — hier gemeten, met de eigen AGC en squelch van de rig al toegepast. Het volgt de audio, niet de RF: ruis op de geluidskaart, een draadloze muis bij de rig of een pompende AGC verplaatsen het allemaal terwijl het signaal niet is veranderd.

smeter-audio-over = (S9 + { $over } dB op de audioschaal)

spectrum-offset-left = ◀ tonen { $hz } Hz

spectrum-offset-right = { $hz } Hz tonen ▶

worldmap-no-snr = geen SNR

solar-tune-the-radio-to-this-downlink = Stem de radio af op deze downlink

qo100-width-label = breedte

qo100-park-label = parkeren

qo100-led-carrier = draaggolf

qo100-led-sync = sync

scanner-stopped = gestopt

ism-band-label = band

amp-none = geen

colormap-classic = Klassiek

colormap-viridis = Viridis

colormap-gray = Grijs

colormap-icom = Icom

colormap-neon = Neon

colormap-synthwave = Synthwave

colormap-matrix = Matrix

colormap-tron = Tron

colormap-amber = Amber

colormap-rainbow = Regenboog

colormap-blue = Blauw

colormap-calmer = Calmer

colormap-compline = Compline

colormap-lauds = Lauds

settings-cb-wide-calls-hint =
    Accepteer de bredere 11 m-roepnaamvorm { $wide } — drie letters in het midden, tot vier cijfers erna — naast die van WSJT-CB: { $narrow }. Experimenteel: WSJT-CB zelf weigert de bredere vorm, dus een roepnaam die dit nodig heeft wordt mogelijk niet begrepen door een WSJT-CB-station; het verbreedt wat dit station hoort meer dan wat het betrouwbaar kan zenden. Uit houdt de exacte grammatica van WSJT-CB aan.
    
    Deze schakelaar verandert de *vorm*, niet de *lengte*: een 11 m-roepnaam is nog steeds 11 tekens inclusief een eventuele /P-, /MM- of /QRP-toevoeging, dus 19DCG3733/P past en 19TST1001/QRP niet.

settings-idle-stop-disable = UITSCHAKELEN

settings-xvtr-tx-off-hint =
    Uit tijdens het converteren: alleen ontvangen — de rij is een converter, geen transverter, en er wordt niets gezonden op deze band.
    
    Via dezelfde converter: het werkt in beide richtingen en zenden gebruikt dezelfde offset.

settings-xvtr-drive-hint =
    Een plafond voor de zendaansturing zolang deze transverter geselecteerd is, als percentage van vol. De I.F.-ingang van een transverter verwerkt milliwatts en de aansturing die goed is voor de eigen banden van de radio zal hem vernielen, dus dit is het belangrijkste veld van de rij. 100% is geen limiet.
    
    Je Drive-instelling wordt hieronder gehouden in plaats van verplaatst, zodat het getal dat je op HF gebruikt terugkomt als de afstemming de band verlaat.

settings-brown-build-note = De Brown-build van sdroxide: het programma van upstream, plus de 11 m-burgerband, kortegolfluisteren en decoderen. Draait als een gewoon amateurstation met die banden en hulpmiddelen erbij.

settings-gain-by-band = Onthoud de front-end-versterking per band

settings-gain-by-band-hint =
    Houd een aparte ontvangerversterking per band aan en herstel die als de afstemming die band binnenkomt. Stel je HF-versterking één keer in en je 11 m- of VHF-versterking één keer; de radio volgt de band.
    
    Standaard uit. Zolang het uit staat, geldt overal één versterking, precies zoals voorheen. Aanzetten legt de band vast waarop je nu zit, zodat geen band zonder is; een band die je nooit hebt ingesteld houdt de huidige versterking.

settings-callsign-grid-note = Je roepnaam en grid, gedeeld door FT8/FT4/FT2, SSTV-beeldkoppen, de digitale CB-modi en het logboek. Dit is de identiteit die **zendt** — een CB-roepnaam als je op 11 m werkt, anders een amateurroepnaam. Waarmee een **ontvangstrapport** wordt ondertekend, zie het vak **Rapporteren als (SWL)** op het tabblad Spots. Ook te bewerken in het FT8- / SSTV-instelvenster.

settings-cb-grammar = 11 m / WSJT-CB-roepnaamgrammatica

settings-cb-wide-calls = BREDE CB-ROEPNAMEN — EXPERIMENTEEL

settings-cb-wide-calls-note = Zet aan als je 11 m-gemeenschap groepen van drie letters of eenheidsnummers van vier cijfers gebruikt. Geldt voor het decoderen en zenden van de digitale CB-modi. In beide gevallen mag een 11 m-roepnaam niet meer dan **11 tekens** hebben, toevoeging inbegrepen — 19DC3733/P past, 19TST1001/QRP niet.

settings-cb-plan-hint = In welk kanaalplan de afstemming leest: de 11 m-burgerbandplannen en de 446 MHz-PMR446-plannen. Alleen de kanaalnummers en het kanaal waarop de band opent veranderen — de bandgrenzen blijven waar ze zijn, dus omschakelen verplaatst nooit wat ontvangt of zendt. Het kanaalnummer wordt op de afstemlijn getekend overal waar de afstemming op een kanaal van het plan staat.

settings-cb-allow-tx = Zenden toestaan op 11 m (CB)

settings-cb-allow-tx-hint = De amateurbandblokkade weigert zenden op elke band die geen amateurtoewijzing is, en 11 m is er een: het is een aparte radiodienst met eigen regels en eigen goedgekeurde apparatuur. Dit aanzetten staat zenden daar toe — en nergens anders; de omroepbanden blijven alleen-ontvangen. De eerste keer word je gevraagd het te bevestigen.

settings-cb-licence-note = CB is in veel landen vergunningvrij, maar niet in alle, en de toegestane kanalen, modi en vermogens verschillen. Het gebruik van de band valt onder de regels van het land waar je bent.

settings-radio-callsign-hint =
    De roepnaam waarmee deze radio zendt, logt en spot. Laat leeg om de stationsroepnaam van het tabblad Algemeen te gebruiken — de standaard, en goed voor een station met één identiteit.
    
    Stel hem in op een radio die iemand anders is: een CB-roepnaam op de 11 m-set, een amateurroepnaam op de HF-rig, elk terwijl de ander de eigen houdt. De roepnaam van het tabblad Algemeen blijft de terugval voor elke radio die hem niet overschrijft.
    
    Ontvangst*rapporten* kunnen nog steeds worden ondertekend met het SWL-nummer van het tabblad Spots, wat hierdoor niet verandert.
    
    Wordt actief na Toepassen.

settings-radio-swap-iq-own = Deze interface heeft zijn eigen I/Q-inversie in zijn sectie verderop op deze pagina — gebruik die.

settings-radio-swap-iq-hint =
    Keer de I/Q van deze ontvanger om vóór demodulatie. Gebruik dit als I en Q van een ontvanger omgekeerd zijn en SSB op de verkeerde zijband uitkomt (LSB gehoord als USB), met het spectrum gespiegeld om de afstemming.
    
    Een eigenschap van de hardware, niet van wie is ingelogd.
    
    Wordt actief na Toepassen.

settings-radio-transmit-controls = Zendbediening

settings-radio-hide-tx-all = verberg alles (SWL-modus)

settings-radio-hide-tx-hint =
    Verberg alle zendbediening voor *deze* radio — de PTT, CALL CQ, TX-niveau, SEND, BEACON, alles — en vervang de ham-extra's van de balk (spots, awards) door die van de luisteraar (SCHEDULE en SWL LOG, het ontvangstlogboek). Per radio: een luisterdongle aan de lange draad kan in deze modus staan terwijl de transceiver ernaast zijn zender houdt.
    
    De hardware kan nog steeds zenden; dit verbergt alleen de bediening. `--swl` forceert het aan voor elke radio gedurende de run.
    
    Wordt actief na Toepassen.

settings-radio-auto-mode = Automodus

settings-radio-auto-stop-label = stop na geen invoer

settings-radio-auto-stop-hint-long =
    Hoe lang de onbewaakte FT8/FT4/FT2-sequencer mag draaien zonder invoer van de operator voordat hij zich uitschakelt. Per radio: de ene set kan langer blijven draaien dan de andere.
    
    Op een PlutoSDR gaat het tot zes uur, en UITSCHAKELEN zet de stop helemaal uit: de automodus draait dan tot je stopt. De zendwatchdog en de bandblokkade blijven gelden.
    
    Wordt actief na Toepassen.

settings-radio-auto-stop-hint =
    Hoe lang de onbewaakte FT8/FT4/FT2-sequencer mag draaien zonder invoer van de operator voordat hij zich uitschakelt. Per radio: de ene set kan langer blijven draaien dan de andere.
    
    Met opzet begrensd op 45 minuten — de automodus is voor een toiletpauze, niet om een station een contest te laten werken zonder iemand erachter.
    
    Wordt actief na Toepassen.

settings-radio-smeter-calib = S-meter-kalibratie

settings-tx-eq = Zend-EQ

settings-tx-eq-hint = Een parametrische EQ met 3 banden op de microfoonaudio, vóór de modulator. Alleen spraakmodi (SSB/AM/FM); digitale modi en CW voeren gesynthetiseerde of getoetste audio die er nooit komt. Standaard uit en vlak als hij wordt aangezet, dus aanzetten verandert niets in de ether totdat een band hieronder daadwerkelijk wordt verplaatst. Werkt direct.

settings-tx-eq-low-hint = Dempt/versterkt gerommel en hanteergeluid.

settings-tx-eq-mid = Middenpiek

settings-tx-eq-mid-hint = Aanwezigheid: een smalle verhoging of dip ergens in de spraakband.

settings-tx-eq-high-hint = Helderheid/de-ess.

settings-cb-call-too-long = ⚠ Te lang voor 11 m — { $len } tekens, en FT8 draagt er 11 voor een CB-roepnaam *inclusief* een eventuele /P-, /MM- of /QRP-toevoeging. Deze activeringsroepnaam kan niet worden verzonden. 19DC3733/P (10) past; 19TST1001/QRP (13) niet.

settings-cb-plan-note = 11 m (CB) is niet overal dezelfde band. Dit kiest in de kanalen van welk land de 11 m-afstemming leest: { $plan }. Alleen de kanalen en het kanaal waarop de band opent — de bandgrenzen blijven wijd, dus van plan wisselen verandert nooit wat ontvangt of zendt. Geldt voor elke radio op dit station.

settings-cw-enter-mode = Ga één keer naar CW (of een digitale modus) om de opgeslagen waarden te laden.

settings-cw-where-from = Waar vandaan:

settings-cw-straight-key-hint = De binding `CW straight key` (standaard Spatie) wordt gelezen zolang KEY van het CW-paneel aan staat. Eén contact, dus het is een straight key — de app decodeert de eigen timing van de operator.

settings-cw-speed = Snelheid:

settings-cw-speed-note = de dit- en dah-lengte van de keyer

settings-cw-key-tx = DE ZENDER TOETSEN

settings-cw-key-tx-hint = Uit: de key stuurt alleen de lokale trainer aan. Aan: hij toetst de radio via het gewone handmatige toetspad, met de bandblokkade en de watchdog eromheen.

settings-cw-key-self-note = Voor een rig die zelf toetst, stel Instellingen → Radio → CW-toetsing in op `Sound card (MCW)`: handtoetsen wordt op de rig-keyerroute met opzet geweigerd.

settings-detachable-windows = Losmaakbare vensters

settings-detachable-windows-note = Trek een paneel naar een eigen OS-venster. Het venster sluiten zet het weer vast.

settings-tool-windows = Gereedschapsvensters

settings-tool-windows-note = Panelen omgezet naar zwevende vensters. Hun open-toestand wordt onthouden.

settings-tool-window-scanner = SCANNER

settings-tool-window-memories = GEHEUGENS

settings-tool-window-playback = AFSPELEN

settings-tool-window-open = Open

settings-tool-window-closed = Gesloten

settings-detachable-native-only = Losmaakbare vensters zijn alleen native.

settings-psk-reporter-band-hint = De bandactiviteitsfeed: stations die op de huidige band worden gehoord, getekend als gekleurde puntjes. Zet ook de "wie hoorde mij"-query aan achter de HEARD ME-chip van de kaart — ook elk uur opnieuw opgehaald.

settings-psk-upload-hint = Rapporteer wat dit station hoort aan pskreporter.info, zodat het daar als ontvanger verschijnt. Gebruikt de roepnaam en het grid van het tabblad Algemeen, of het SWL-nummer hieronder als dat is ingesteld.

settings-swl-report-as = Rapporteren als (SWL)

settings-swl-report-as-hint = De eigen identiteit van de luisteraar voor ontvangstrapporten — een geregistreerd SWL-nummer, een clubnummer, een naam. Als het is ingesteld, ondertekent het wat dit station *hoort* (PSK Reporter- en WSPRnet-uploads, en het ontvangstrapport dat het SWL-logboek uitschrijft) in plaats van de roepnaam. Laat leeg om als de roepnaam te rapporteren. Het wordt nooit uitgezonden, toetst nooit een radio en noemt nooit een QSO of spotter, zodat ook een luisteraar zonder roepnaam kan rapporteren.

settings-wsjtcb-spot-server = WSJT-CB-spotserver

settings-wsjtcb-report = Rapporteer mijn 11 m-decoderingen aan de WSJT-CB-spotserver

settings-wsjtcb-report-hint = Stuur elke op de 11 m-band gedecodeerde spot naar de spotdienst van de CB-gemeenschap (xzgroup.net) — dezelfde server waar de eigen client van WSJT-CB naar kan posten. Standaard uit: het is een dienst van een derde partij en er wordt niets verstuurd totdat je erom vraagt. Je roepnaam is de spotter; PSK Reporter is de aparte schakelaar hierboven.

settings-eqsl-qth-hint = In welk QTH-profiel een upload wordt opgeborgen — alleen nodig als het account er meer dan één heeft, en anders leeg gelaten.

settings-log11dx-api-url = API-URL

settings-log11dx-api-token = API-token

settings-log11dx-note = Beide komen van je LOG11DX-profielpagina. Zijn eigen WSJT-X-brug is niet nodig: sdroxide logt de QSO en post hem rechtstreeks daarheen. De site maskeert het token, dus kopieer het zodra je het genereert — het wordt opgeslagen in net.json zoals de andere inloggegevens.

settings-pan-attached-note =
    Zijn spectrum, zijn waterfall en alles wat daarvan wordt afgelezen staan op het tabblad van die radio, daarom heeft hij er zelf geen. De interface-instellingen hierboven zijn nog steeds van deze radio en gelden nog steeds — druk op Toepassen / opnieuw verbinden na het wijzigen ervan en beide radio's komen samen weer op.
    
    Om hem weer zelfstandig te gebruiken, wis je het vak Panadapter-ontvanger op de pagina van die radio.

settings-pan-source-hint =
    Welke radio voor deze ontvangt. Hij verdwijnt uit de tabbalk zolang hij uitgeleend is — zijn front-end hoort bij de engine van deze radio — en komt direct terug als dit op Geen wordt gezet.
    
    Wordt actief na Toepassen.

settings-pan-offset-hint =
    Hoeveel boven de afstemming van deze radio de ontvanger zit, in hertz. 0 voor een ontvanger op dezelfde antenne; de middenfrequentie voor een I.F.-aftakking — 9000000 voor een 9 MHz I.F., 70455000 voor een van 70.455 MHz.
    
    Dit is niet het veld Converter hierboven, dat de radio herafstemt, en niet de I/Q-middenoffset van het CAT-tabblad, die over de eigen geluidskaart van deze radio gaat. Wat hier wordt getypt, wordt nooit naar een van beide radio's gestuurd.
    
    Wordt actief na Toepassen.

settings-cw-device = Apparaat:

settings-cw-device-none = (niets gevonden)

settings-cw-device-auto = auto (eerste met naam "key")

settings-cw-reverse-hint = Verwissel dit en dah, voor de schakelaar op de paddle zelf.

settings-cw-key-type = Keytype:

settings-cw-key-straight = STRAIGHT

settings-cw-key-iambic-a = IAMBIC A

settings-cw-key-iambic-b = IAMBIC B

settings-cw-usb-note = Een keyerbox die zijn twee contacten meldt (vaak als de knoppen van een muis) werkt; een die zelf iambic doet en afgewerkte aanslagen stuurt niet. Zolang de key is ingeschakeld wordt het apparaat exclusief genomen, zodat zijn contacten niet in andere vensters kunnen klikken.

settings-cw-usb-linux = Een USB-paddle vereist een Linux-desktop.

settings-smeter-zero = 0 (zoals gemeten)

settings-smeter-cal-hint =
    Corrigeert de S-meter van deze radio als hij te hoog of te laag aanwijst, in S-eenheden van 6 dB.
    
    Een negatieve waarde verlaagt de aflezing — voor een meter die te veel toont, een ruisvloer op S5 waar het S2 zou moeten zijn. Een positieve waarde verhoogt hem. 0 is de meter zoals gemeten.
    
    Alleen de aflezing verschuift: de naald, de S- en dBm-cijfers, het niveau waarvan een ontvangstrapport uitgaat, en de gesproken S-meter. Het signaal, de squelch, opnemen en de decoders blijven ongemoeid.
    
    Opgeslagen bij deze radio op de machine waaraan hij hangt, en werkt direct.

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1 en 2 gebruiken beide { $host }:{ $port } — elke spot komt dubbel binnen. Geef ze verschillende poorten (7000 CW/RTTY, 7001 FT8/FT4) of zet er een uit.

settings-rx-site-station = Bij het station

settings-rx-site-elsewhere = Ergens anders

settings-remote-tab-note =
    Het station verschijnt als tabblad naast de radio's die hier al open zijn, en de tabbalk bovenaan het venster schakelt ertussen (⊞ zet er twee naast elkaar). Sluit het via de lijst bovenaan het tabblad Radio — dat verbreekt de verbinding; op de server verandert niets.
    
    Als de server om een gebruikersnaam en wachtwoord vraagt, verschijnt zijn inlogscherm in het nieuwe tabblad. Een gewone ws://-link draagt dit niet versleuteld, dus over het open internet leid je hem ofwel door een VPN of een SSH-tunnel, of beëindig je HTTPS vóór de server en vink je hierboven Secure aan.

settings-cw-source-keyboard = TOETSENBORD

settings-cw-source-usb = USB-PADDLE

pane-frame = FRAME

pane-frames = FRAMES

pane-words = WOORDEN

antenna-feed-z0 = Voeding Z0:

antenna-feed-z0-hint = Impedantie van voedingslijn/systeem voor SWR (50 Ω gebruikelijk)

antenna-height-label = Hoogte:

antenna-height-hint = Hoogte van voedingspunt/antenne boven de grond (beïnvloedt afstraling/patroon)

antenna-m-agl = m AGL

antenna-apply = Toepassen

antenna-apply-hint = Draai de momentenmethode-solver op de presets.

antenna-yagi-optimizer = Yagi-optimalisator

antenna-stack = Stack:

antenna-objective = Doel:

antenna-optimize = Optimaliseren

antenna-swr-value = SWR { $v }

antenna-swr-ref = SWR { $v } ({ $z }Ω)

antenna-radiation-pattern = Stralingspatroon

antenna-view = Weergave

antenna-view-front = Voor

antenna-view-side = Zij

antenna-view-top = Boven

antenna-3d-hint = 3D slepen om te draaien • hitte = veldsterkte

antenna-yagi-x = Yagi ×{ $n }

settings-tool-window-session-qsos = SESSIE-QSO'S

sessionqso-time = Tijd

sessionqso-call = Roepnaam

sessionqso-grid = Grid

sessionqso-band = Band

sessionqso-mode = Modus

sessionqso-rst-s = RST S

sessionqso-rst-r = RST R

sessionqso-country = Land

sessionqso-dist = Afst.

sessionqso-final = Eind

sessionqso-count = { $n } QSO deze sessie

sessionqso-include-cw = CW opnemen

decodes-live-log = LIVE LOG

decodes-setup = ⚙ INSTELLEN

schedule-colour-all = Kleur

schedule-colour-all-hint = Kleur van de waterval-labels voor alle zenders — vervangt de per zender gekozen kleuren

schedule-colour-station-hint = Kleur van het waterval-label van deze zender

schedule-colour-reset = STANDAARD

schedule-colour-reset-hint = Terug naar de standaardkleur voor alle zenders; de gekozen kleuren worden vergeten

settings-language = Taal

settings-language-note = De taal waarin de interface wordt getekend. Standaard volgt die het besturingssysteem; kies een andere om die te vervangen. De keuze wordt opgeslagen en onthouden.

aprs-all-chip = ALLES

aprs-all-chip-hint = Aan: elk bericht dat op het kanaal te horen is, voor wie het ook bedoeld is. Uit: alleen de berichten aan of van u, en bulletins.

aprs-composer-hidden-swl = Verzenden is verborgen: de SWL-/luistermodus staat aan. Zet hem uit om een bericht te schrijven en te verzenden.

aprs-compose-count-hint = Gebruikte tekens van de 67 die een APRS-bericht kan bevatten.

tx-bands-title = Zendbanden

tx-bands-note = Zet TX alleen op JA voor de banden waarop deze radio echt zendt. Dit gaat voor alle andere zendinstellingen; JA heft de andere veiligheidsgrenzen nooit op.

tx-bands-careful = ⚠ Een verkeerde band kan de radio vernietigen: controleer elke optie zorgvuldig.

tx-bands-yes = JA

tx-bands-no = NEE

tx-bands-confirmed = ✓ Zendbanden bevestigd voor deze radio

tx-bands-ack = Ik heb deze banden met mijn radio vergeleken

tx-bands-confirm = Deze banden bevestigen

tx-bands-confirm-hint = Vink eerst het vakje aan: u bevestigt één keer per radio.

tx-bands-check = Banden controleren…

tx-bands-notice-new = Zenden staat op alle banden uit. Kies de banden waarop deze radio zendt en bevestig ze één keer. Vanwege het risico voor de radio: controleer de gekozen opties zorgvuldig.

tx-bands-notice-carried = Uw zendbanden zijn overgenomen uit de vorige versie. Zenden blijft geblokkeerd tot u ze één keer hebt gecontroleerd en bevestigd. Vanwege het risico voor de radio: controleer elke optie zorgvuldig.


adsbmap-surface-asphalt = asfalt

adsbmap-surface-concrete = beton

adsbmap-surface-grass = gras

adsbmap-surface-gravel = grind

adsbmap-surface-dirt = aarde

adsbmap-surface-water = water

adsbmap-airport-runway-lit = verlicht

adsbmap-airport-more = +{ $n } meer

adsbmap-airport-freq-head = Frequenties, MHz

adsbmap-airport-data-date = baan- en frequentiegegevens van { $date }
aircraft-online-switch = Online

aircraft-online-switch-on-hover = Online: type, exploitant, route en foto komen van adsbdb en Planespotters, luchthavenbeelden van Wikipedia en de lokale tijd en temperatuur van een luchthaven van Open-Meteo (alleen de positie van de luchthaven wordt verzonden), door uw station voor u opgevraagd. Klik om uit te zetten; het station onthoudt uw keuze. Er verlaat verder niets het station.

aircraft-online-switch-off-hover = Offline: er verlaat niets het station. Klik om type, exploitant, route en foto (adsbdb en Planespotters), luchthavenbeelden (Wikipedia) en het luchthavenweer (Open-Meteo) aan te zetten.

aircraft-switch-held = de beheerder van het station legt het vast met SDROXIDE_AIRCRAFT_ONLINE

adsbmap-airport-photo-credit = Foto: Wikipedia

adsbmap-airport-local-time = Lokale tijd { $time } ({ $zone })

adsbmap-airport-temperature = Temperatuur { $temp }

adsbmap-airport-weather-credit = Weer: Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time } (lokale tijd: laden…)

adsbmap-airport-utc-offline = UTC { $time } (lokale tijd: offline)

schedule-listen = LUISTER

schedule-listen-hint = Stem de ontvanger af op dit station

schedule-sort = Sorteren

schedule-sort-asc = Oplopend

schedule-sort-desc = Aflopend

schedule-col-freq = Freq.

schedule-col-band = Band

schedule-col-name = Naam

schedule-col-lang = Taal

schedule-col-target = Doelgebied

schedule-col-site = Locatie

schedule-col-local = Lokaal

tx-bands-off = Zenden staat uit op { $band }: deze band is in uw instellingen niet toegestaan, dus uw zender is niet geschikt om op deze band te zenden. (Maximaal { $max } keer per sessie getoond.)

frame-band-tx = BAND TX

tx-off-band = ZENDEN VERBODEN — { $freq } MHz ({ $band }) is geen gevalideerde zendband. Vink aan en valideer onder Instellingen → Radio → Zendbanden.

tx-off-unconfirmed = ZENDEN VERBODEN — de zendbanden van deze radio zijn nog niet gevalideerd. Controleer en bevestig onder Instellingen → Radio → Zendbanden.

aircraft-hover-lookup-retry = geen antwoord — opnieuw proberen over { $s } s

adsbmap-airport-navaids-head = Radiobakens op het terrein

adsbmap-airport-no-freq = Geen frequentie gepubliceerd voor deze luchthaven

topbar-preset-right-click = Rechtsklik om deze voorinstelling te wijzigen

topbar-preset-edit = VOORINSTELLING BEWERKEN

topbar-preset-label = Label

topbar-preset-label-hint = Wat de chip toont. Maximaal 8 tekens.

topbar-preset-lo = Laag

topbar-preset-hi = Hoog

topbar-preset-use-current = Huidige gebruiken

topbar-preset-use-current-hint = De randen van het huidige filter van de ontvanger overnemen

topbar-preset-save = Opslaan

topbar-preset-reset = Herstellen

topbar-preset-reset-hint = De ingebouwde voorinstelling van deze modus terugzetten

topbar-preset-cancel = Annuleren
wefax-schedule = SCHEMA
wefax-schedule-hint = Opent het door { $station } gepubliceerde schema — kaarten en UTC-tijden — in een browser.

tx-bands-check-again = OPNIEUW

tx-bands-check-again-hint = Trekt de bevestiging in om de lijst opnieuw te bekijken. Een band wijzigen doet dat al vanzelf.
