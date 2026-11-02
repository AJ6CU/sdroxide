# sdroxide — tradução da interface para o português do Brasil. A referência é
# en-US/sdroxide.ftl; o que falta aqui é mostrado em inglês.
#
# Ids are grouped by where they are shown, and named after it:
# settings-ui-* is the UI tab of the Settings window, and so on. A message
# shown in capitals is written in capitals here; the code does not upper-case
# translated text.

## The language's own name, as the language picker lists it.
language-name = Português (Brasil)

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = Automático
layout-mode-desktop = Desktop
layout-mode-tablet = Tablet
layout-mode-small = Tela pequena
layout-mode-phone = Celular

ui-theme-default = Padrão
ui-theme-light = Claro
ui-theme-high-contrast = Alto contraste
ui-theme-green-phosphor = Fósforo verde
ui-theme-amber-phosphor = Fósforo âmbar
ui-theme-teal-orange = Verde-azulado / laranja
ui-theme-rainbow = Arco-íris
ui-theme-nord = Nord
ui-theme-nord-dark = Nord escuro
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized escuro
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin mocha
ui-theme-catppuccin-latte = Catppuccin latte
ui-theme-modern-minimalist = Minimalista moderno

chrome-style-angled = Angular
chrome-style-rectangular = Retangular
chrome-style-rounded = Arredondado
chrome-style-gradient = Gradiente
chrome-style-bevel = Bisel 3D
chrome-style-terminal = Terminal

font-size-small = Pequeno
font-size-medium = Médio
font-size-large = Grande

bandplan-kind-ham = Amador
bandplan-kind-cw = CW
bandplan-kind-digi = Digital
bandplan-kind-phone = Voz
bandplan-kind-beacon = Balizas
bandplan-kind-broadcast = Radiodifusão
bandplan-kind-am = AM / OL / OM
bandplan-kind-cb = CB

speech-verbosity-terse = Conciso
speech-verbosity-normal = Normal
speech-verbosity-full = Completo

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = Dial (14 point 074)
speech-freq-style-digits = Cada dígito
speech-freq-style-full = Dial com unidades

speech-callsign-style-phonetic = Fonético (kilo one alpha)
speech-callsign-style-letters = Letras (kay one ay)
speech-callsign-style-as-is = Como escrito

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = Tela
settings-ui-moved-note = O detalhe do panadaptador, a reação do espectro e a velocidade de rolagem da cascata são definidos pelo botão SPEC na barra de exibição — o menu DISP em uma janela estreita.
settings-ui-language = Idioma
    .hint = O idioma da interface. O manual aberto com F1 e os anúncios de voz continuam em inglês, qualquer que seja a opção escolhida aqui.
settings-ui-layout = Layout
settings-ui-theme = Tema
settings-ui-button-style = Estilo dos botões
settings-ui-frame-rate = Taxa de atualização da tela
settings-ui-palette = Paleta da cascata
settings-ui-tuning-buttons = Botões de sintonia
settings-ui-tuning-buttons-show = Mostrar em celular e tablet
    .hint = Uma linha de botões diminuir / passo / aumentar abaixo da barra de controles em uma tela sensível ao toque. Não há roda para girar um dígito nem dial, então, sem eles, a única forma de mover um passo conhecido é digitar a frequência inteira. Nunca aparece em desktop.
settings-ui-tuning-step = Passo: { $step }
settings-ui-tuning-snap = O primeiro toque ajusta ao passo
    .hint = Um dial parado entre dois múltiplos do passo vai para o próximo na direção pressionada — 7 074 300 sobe para 7 075 000 com passo de 1 kHz, desce para 7 074 000. Depois disso, os botões se movem pelo passo normalmente. Desligado, cada toque move exatamente o passo.
settings-ui-smoothing = Suavização da cascata
settings-ui-smoothing-interpolate = Interpolar
    .hint = Mistura cada pixel da tela com as raias e linhas vizinhas, para que um sinal pareça contínuo quando a tela é mais larga que a transformada. Desligue para uma cascata retangular — um bloco por raia, um por linha —, que é o necessário para ler a forma de um sinal na imagem: um sinal interpolado não se distingue de um genuinamente mais largo. Uma FFT maior (o chip FFT) é a outra metade disso.
settings-ui-background = Fundo do espectro
settings-ui-background-gradient = Gradiente
settings-ui-background-top = topo
settings-ui-background-bottom = base
settings-ui-spot-colours = Cores dos rótulos de spots
    .hint = A cor que cada fonte de spots assume: as caixas ao longo da base da cascata, os distintivos na lista SPOTS e os pontos no mapa-múndi.
settings-ui-spot-colour = Cor para spots de { $kind }
settings-ui-spot-colours-reset = Devolver todas as cores de spots ao padrão
settings-ui-reset = Redefinir
settings-ui-bandplan-colours = Cores do plano de faixas
    .hint = O tom em que cada classe de alocação é pintada na faixa do plano de faixas ao longo da base da cascata. Os blocos são desenhados semitransparentes sobre a cascata, então ficam mais escuros lá do que na amostra aqui.
settings-ui-bandplan-colour = Cor para alocações de { $kind }
settings-ui-bandplan-colours-reset = Devolver todas as cores do plano de faixas ao padrão
settings-ui-skimmer-font = Tamanho da fonte do skimmer
settings-ui-waterfall-font = Tamanho da fonte da cascata / espectro
settings-ui-interface-font = Tamanho da fonte da interface
    .hint = Escala a interface inteira — menus, diálogos, janelas, as abas de rádio, a barra superior e seus botões. Os dois tamanhos acima são relativos a ele.
settings-ui-cities = Cidades nos mapas
    .hint =
        Desenha as cidades do mundo — um ponto por lugar, com o nome ao lado quando há espaço — nos mapas planos: FT8/WSPR, APRS, ADS-B e AIS.

        They are most of what says *where* a dot is on a map of the whole world, and they are also the busiest thing on it. Turn them off if the names are getting in the way of the stations you are reading. The 3D globe is unaffected — its cities are night-side lights rather than markers, and nothing there is written across a contact.
settings-ui-cities-show = mostrar cidades e seus nomes
settings-ui-3d-view = Visão 3D
settings-ui-clouds = Renderização de nuvens
settings-ui-clouds-layered = Em camadas
settings-ui-clouds-volumetric = Volumétrica
settings-ui-clouds-note = Como a camada CLOUDS da visão 3D desenha o tempo. Em camadas empilha fatias da troposfera e é a opção leve. Volumétrica percorre um raio por ela, então o Sol projeta os topos das nuvens sobre a camada de baixo e os relâmpagos brilham de dentro da tempestade que os produz, em vez de apenas clarear o seu exterior — a um custo por pixel de várias vezes maior.

## Settings window, UI tab: voice announcements.

speech-title = Anúncios de voz
speech-enabled = Falar as mudanças do rádio
    .hint = Lê em voz alta o que mudou, para que o rádio possa ser operado sem ser visto
speech-voice = Voz
speech-voice-shipped = Voz padrão
speech-speed = Velocidade
    .hint = A voz estica ou comprime a própria entonação, então o tom não muda. Acima de cerca de 2×, ela deixa de encurtar.
speech-volume = Volume
speech-output = Saída
speech-detail = Detalhe
speech-duck = Reduzir o receptor
speech-duck-while = Enquanto fala
speech-test = Testar
speech-what = O que anunciar
speech-cat-frequency = Frequência
speech-cat-mode-band = Modo e faixa
speech-cat-vfo-split = VFO e split
speech-cat-agc-gain = AGC e ganho
speech-cat-levels = Excitação, sintonia e microfone
speech-cat-ptt = Transmissão e recepção
speech-cat-rit-xit = RIT e XIT
speech-cat-memory-scan = Memórias e varredura
speech-cat-band-edge = Saída de uma faixa de amador
speech-cat-notices = Avisos e mensagens
speech-cat-filters = Filtros, squelch e redução de ruído
    .hint = Desligado por padrão: estes mudam o tempo todo ao seguir um sinal
speech-decoded = Mensagens decodificadas
speech-ft8-to-me = Chamadas FT8 para mim
speech-ft8-cq = CQs FT8 que eu poderia responder
    .hint = Uma noite movimentada em vinte metros são uma centena destes por minuto
speech-js8 = Mensagens JS8 para mim
speech-js8-allcall = JS8 @ALLCALL também
speech-fsq = Mensagens FSQ para mim
speech-include-snr = Incluir o relatório
    .hint = Só onde a mensagem não traz um próprio — uma decodificação que já informa um número não recebe também o nosso
speech-ft8-qso = Minha própria troca FT8
    .hint = O que o sequenciador está prestes a enviar, e o término do contato
speech-reading-text = Ler em voz alta o texto decodificado
speech-text-cw = CW
speech-text-rtty-psk = RTTY, PSK, Olivia, THOR, FSQ
speech-text-cw-locked = CW somente enquanto o decodificador estiver travado
    .hint = Ler a saída de um decodificador destravado é pior que silêncio
speech-text-note = Ambos vêm desligados por padrão. Um decodificador produz texto mais depressa do que a fala consegue lê-lo, então tudo o que fica muito atrás do áudio ao vivo é descartado em vez de enfileirado.
speech-tuning-up = Ajustando
speech-swr-while-tuning = Dizer a ROE enquanto TUNE estiver pressionado
speech-swr-every = A cada
speech-swr-summary = Informar a melhor casamento ao soltar
speech-swr-alarm = Avisar sobre ROE alta durante qualquer transmissão
speech-how-read = Como as coisas são lidas
speech-frequencies = Frequências
speech-callsigns = Indicativos
speech-quiet-on-tx = Ficar em silêncio durante a transmissão
    .hint = A fala sai pelos seus alto-falantes e, portanto, entra no seu microfone. Os avisos de ROE alta continuam passando.
speech-device-note = Os anúncios tocam no próprio dispositivo de som, então nunca são gravados nem enviados a quem estiver ouvindo remotamente. As teclas para falar o status, repetir o último anúncio e interromper no meio de uma frase ficam na aba Controles.

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = Aguardando a configuração de rede da estação…
settings-device-reports = como o dispositivo informa
settings-transverters = Transverters
settings-one-row-per-box-while = Uma linha por caixa. Enquanto o dial estiver dentro da faixa de uma linha, o rádio é sintonizado em dial + deslocamento e valem a regra de transmissão e o limite de excitação dessa linha; fora de todas as linhas, o rádio fica nas próprias faixas. As linhas são testadas de cima para baixo.
settings-name = Nome
settings-band-low = Início da faixa
settings-band-high = Fim da faixa
settings-offset = Deslocamento
settings-transmit = Transmissão
settings-max-drive = Excitação máxima
settings-off-takes-transverter-out-line = Desligado tira este transverter da linha sem perder a linha — a faixa volta ao que o rádio alcança por conta própria.
settings-t-2-m = 2 m
settings-what-you-call-shown-log = Como você o chama. Mostrado no log quando o dial o seleciona.
settings-bottom-band-transverter-works-dial = O início da faixa em que este transverter trabalha, no dial. 144 para 2 m.
settings-top-band-dial-148-2 = O fim da faixa, no dial. 148 para 2 m nas Regiões 2 e 3, 146 na Região 1.
settings-radio-ends-up-dial-offset = O rádio acaba em dial + deslocamento, então um transverter que leva uma faixa para um F.I. é negativo: 2 m em um F.I. de 28 MHz é -116, porque 144 - 116 = 28. Arraste para ajustar um hertz por vez, para um oscilador um pouco fora de frequência.

    Through the same converter: it works both ways, and transmit takes the same offset.

    Your Drive setting is held under this rather than moved, so the number you use on HF comes back when the dial leaves the band.
settings-remove = REMOVER
settings-add-transverter = ADICIONAR TRANSVERTER
settings-takes-effect-apply-reconnect-like = Tem efeito ao Aplicar / reconectar, como o deslocamento do conversor acima.
settings-maximum-transmit-drive = Excitação máxima de transmissão
settings-hard-limit-drive-tune-controls = Um limite rígido que os controles de Excitação e TUNE não podem ultrapassar, aplicado depois da calibração de faixa abaixo para que nada possa elevar a excitação acima dele. Defina-o no ponto em que o rádio entrega a potência nominal, e todo o controle de Excitação fica utilizável — em um transmissor que modula as próprias amostras (HPSDR, LimeSDR, PlutoSDR, HackRF), a excitação total é o transmissor no máximo, o que, em um amplificador de alto ganho, fica muito acima do que os seus finais suportam. Desligado, os controles chegam à excitação total.
settings-limit-drive = Limitar a excitação a
settings-per-cent-full-drive-key = Porcentagem da excitação total. Coloque o rádio em uma carga fantasma e reduza até que o medidor mostre a potência para a qual o amplificador é especificado.
settings-applies-immediately-every-band-tune = Vale imediatamente, em todas as faixas e também para TUNE.
settings-transmit-drive-band = Excitação de transmissão por faixa
settings-every-amplifier-has-different-gain = Todo amplificador tem ganho diferente em cada faixa, então uma mesma configuração de Excitação produz uma potência diferente em cada uma. Meça a saída em cada faixa e depois ajuste as faixas que saem altas até que todas combinem: defina a Excitação para a faixa que mais precisa (geralmente a mais alta) e reduza as outras até ela. Decibéis de potência de saída, aplicados igualmente a voz, digital e TUNE. Zero em todas — o padrão — não muda nada.
settings-other = Outras
settings-clear-all = LIMPAR TUDO
settings-back-no-calibration-drive-setting = Voltar a nenhuma calibração: a configuração de Excitação alcança cada faixa integralmente.
settings-applies-immediately-band-you-would = Vale imediatamente, e para a faixa em que você transmitiria — atrás de um transverter, é a faixa no dial, e o limite de excitação do próprio transverter ainda prevalece sobre qualquer coisa definida aqui.
settings-settings = Configurações
settings-enter-server-s-address-first = informe primeiro o endereço do servidor
settings-enter-receiver-s-address-first = informe primeiro o endereço do receptor, ou escolha um em SDRs públicos
settings-no-radio-selected-press-discover = nenhum rádio selecionado — pressione Descobrir ou informe um endereço
settings-enter-radio-s-address-first = informe primeiro o endereço do rádio
settings-general = Geral
settings-radio = Rádio
settings-ui = UI
settings-alerts = Alertas
settings-controls = Controles
settings-spots = Spots
settings-freedv = FreeDV
settings-uploads = Envios
settings-winlink = Winlink
settings-servers = Servidores
settings-t-r-switch = Chave T/R
settings-tle = TLE
settings-profiles = Perfis
settings-station = Estação
settings-enter-digital-mode-ft8-sstv = Informe um modo digital (FT8 / SSTV / …) uma vez para carregar os valores salvos.
settings-callsign = Indicativo
settings-grid-square = Quadrado locator
settings-iaru-region = Região IARU
settings-which-radio-interface-server-uses = A interface de rádio que o servidor usa é definida na máquina onde ele roda.
settings-radio-interface = Interface do rádio
settings-waiting-machine-radio-attached-interface = Aguardando a máquina à qual o rádio está conectado. As configurações da interface dela estão abaixo e podem ser alteradas daqui enquanto isso.
settings-converter = Conversor
settings-manual-type-offset-below = Manual — digite um deslocamento abaixo
settings-what-box-reads-when-offset = O que esta caixa mostra quando o deslocamento não é um dos predefinidos. Defina-o digitando no campo Deslocamento abaixo; não há nada a selecionar aqui.
settings-frequency-converter-front-receiver-pick = Um conversor de frequência na frente do receptor. Escolha um, ou digite um deslocamento ao lado.
settings-how-far-converter-moves-signal =
    Quanto um conversor desloca o sinal no caminho até o receptor, em Hz — o mesmo número e sinal que a documentação de todo conversor e de todo outro programa SDR informa. O rádio acaba em dial + deslocamento: positivo para um upconverter (um Ham It Up é 125000000), negativo para qualquer coisa que leva uma faixa a um F.I. — um LNB de satélite, ou um transverter de 2 m em um F.I. de 28 MHz, que é -116000000 (28 − 144). 0 = sem conversor.

    Drag to trim it a hertz at a time, which is what a converter whose oscillator is slightly off wants.

    This is the receive path. What is in the transmit line is the row below.

    Takes effect on Apply.
settings-what-transmit-line-while-converter =
    O que entra na linha de transmissão enquanto um conversor estiver definido.

    Off while converting: nothing is transmitted — the safe default, and right for a receive-only accessory.

    Through the same converter: one box converts both ways, a transverter. Transmit takes the offset above and follows it when it is trimmed.

    Its own offset: the transmit line is different from the receive one. Nought is the common case — a converter (an LNB, a Ham It Up) on receive with the transmitter on its own antenna, which is the QO-100 station.

    The amateur-band check still applies, on the frequency you are transmitting on.

    Takes effect on Apply.
settings-how-far-transmit-line-moves =
    Quanto a linha de transmissão desloca o sinal, em Hz, com a mesma regra de sinal do deslocamento de recepção: o rádio transmite em dial + deslocamento.

    0 = the transmitter is wired to its own antenna and works on the frequency the dial says — a QO-100 station's 2.4 GHz uplink.

    A transmit converter that takes an I.F. *up* is a negative number: the radio works below the dial.
settings-rx-range = Faixa de RX
settings-which-frequencies-radio-receives-mhz =
    Quais frequências este rádio recebe, em MHz: 144-146, 430-440. Deixe vazio para usar o que o dispositivo informa sobre si mesmo.

    Band buttons outside the range are greyed out and the dial will not go there.

    These are dial frequencies. With a converter set, state the band you tune — 144-148 for a 2 m transverter — not the I.F. the radio is really on.

    Takes effect on Apply.
settings-tx-range = Faixa de TX
settings-which-frequencies-radio-transmits-mhz =
    Em quais frequências este rádio transmite, em MHz: 144-146, 430-440. Deixe vazio para usar o que o dispositivo informa — e, se ele não informar nada, o driver é aceito pela palavra e qualquer frequência é permitida.

    Dial frequencies, like the receive range above.

    This is a limit you set, not a licence: transmitting outside the amateur bands is refused regardless unless you have turned that off in config.toml. Nor does it give a receive-only device a transmitter.

    Takes effect on Apply.
settings-antenna = A antena está
settings-where-radio-listens-station-antenna =
    De onde este rádio escuta.

    At the station: the antenna is yours, so your own locator says where it is. The default, and right for everything in the shack.

    Somewhere else: an online receiver, or your own set up on a hilltop. What it hears is reported to PSK Reporter, WSPRnet and FreeDV Reporter from the locator beside this, never from yours — and ADS-B places aircraft against it too.

    Picking a receiver under "Public SDRs" fills this in for you.

    Takes effect on Apply.
settings-receiver-s-maidenhead-locator-jn88ec =
    O locator Maidenhead do receptor — JN88ec, DO30db. Quatro ou seis caracteres.

    Leave it empty if you do not know where the receiver is: nothing is then reported at all, which is the only honest answer. Reporting from your own square would put somebody else's reception on the wrong continent.

    On top of cal_offset_db in config.toml, which is one figure for the whole station.
settings-antenna-somewhere-else-no-locator = A antena está em outro lugar e nenhum locator está definido, então nada que este rádio ouve é informado ao PSK Reporter, WSPRnet ou FreeDV Reporter. Digite o locator do receptor acima para informar a partir dele.
settings-ranges-are-mhz-low-high = Os intervalos estão em MHz, inicial-final, separados por vírgulas: 144-146, 430-440 — ou seja, 144000000-146000000 Hz e 430000000-440000000 Hz. São frequências de dial: um deslocamento de conversor não as move. O deslocamento de conversor acima é o campo em hertz. Deixe um intervalo vazio para usar o que o dispositivo informa sobre si mesmo; um dispositivo que não informa nada é aceito pela palavra.
settings-careful-rtl-sdr-blog-v4 = Cuidado com um RTL-SDR: o Blog V4 faz upconversão por conta própria abaixo de 28,8 MHz, então um deslocamento negativo que leve o hardware para lá desloca duas vezes.
settings-pick-radio-interface-above-configuration = Escolha uma interface de rádio acima (esta configuração usava o modo de detecção automática, que foi removido).
settings-configuration-used-removed-auto-detect = Esta configuração usava o modo de detecção automática, que foi removido. Escolha uma interface acima quando a máquina à qual o rádio está conectado tiver respondido.
settings-radio-has-no-interface-yet = Este rádio ainda não tem interface — escolha uma acima e pressione Aplicar / reconectar.
settings-radio-has-no-interface-yet-2 = Este rádio ainda não tem interface. Uma pode ser escolhida acima quando a máquina à qual o rádio está conectado tiver respondido.
settings-apply-reconnect = Aplicar / reconectar
settings-switch-interface-now-no-restart = Trocar para esta interface agora — sem precisar reiniciar
settings-switch-server-s-radio-interface = Trocar agora o rádio do servidor para esta interface — sem precisar reiniciar. A antiga continua rodando se a nova não puder ser aberta.
settings-switches-live-radio-without-restarting = Troca o rádio em uso sem reiniciar.
settings-everything-above-server-s-own = Tudo acima é a configuração própria do servidor, e as alterações são salvas lá. A maioria das configurações vale enquanto você as altera; as que são fixadas quando o dispositivo é aberto — a taxa de amostragem, um endereço, a própria interface — precisam deste botão.
settings-dx-cluster-telnet = Cluster DX (telnet)
settings-enabled = Ativado
settings-host = Host
settings-port = Porta
settings-login-call = Indicativo de login
settings-commands = Comandos
settings-one-per-line-e-g = um por linha, ex.: SET/FT8
settings-one-per-line-e-g-2 = um por linha, ex.: set/filter cont=eu
settings-sent-after-login-place-narrow = Enviado após o login. O lugar para restringir o feed — sem um filtro, são todos os skimmers da Terra.
settings-rbn-paths-are-placed-country = Os caminhos do RBN são posicionados a partir de centros de países, não de locators — precisos para um país pequeno, e errados por muito para um grande. Eles são uma camada separada do mapa de propagação e podem ser desligados ali.
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = Spots de ativadores POTA
settings-sota-spots = Spots SOTA
settings-psk-reporter-current-band = PSK Reporter (faixa atual)
settings-upload-my-ft8-ft4-ft2 = Enviar minhas decodificações FT8/FT4/FT2/JS8
settings-antenna-2 = Antena
settings-collector = Coletor
settings-t-4739-live-collector-14739-test = 4739 é o coletor ao vivo, 14739 o de teste
settings-max-age-s = Idade máxima (s)
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = Enviar minhas decodificações WSPR
settings-send-every-wspr-reception-wsprnet = Envia cada recepção WSPR ao wsprnet.org. Ligado por padrão: não coloca nada no ar, e informar o que você ouve é o que torna um receptor WSPR parte da rede em vez de uma curiosidade privada. Um intervalo que não decodificou nada também é informado, que é como a rede distingue uma faixa fechada de um receptor desligado.
settings-download-who-heard-me = Baixar quem me ouviu
settings-ask-wsprnet-org-which-stations = Pergunta ao wsprnet.org quais estações decodificaram esta. O WSPR não tem nenhum tipo de confirmação, então esta é a única forma de um balizador transmissor saber algo sobre o próprio alcance.
settings-ask-every = Perguntar a cada
settings-looking-back = olhando para trás
settings-apply = {" "}APLICAR{" "}
settings-persist-re-connect-feeds = Salvar e (re)conectar os feeds
settings-broadcast-stations = Emissoras de rádio
settings-winlink-account = Conta Winlink
settings-password = Senha
settings-winlink-account-password-not-gateway = A senha da conta Winlink, não a senha do gateway. Diferencia maiúsculas e minúsculas — digite exatamente como foi emitida.
settings-locator = Locator
settings-how-connect = Como conectar
settings-route = Rota
settings-internet = Internet
settings-forward-cms-over-internet = Encaminhar com o CMS pela internet
settings-radio-packet = Rádio (pacote)
settings-call-rms-gateway-air-radio = Chama um gateway RMS pelo ar. O rádio precisa estar em PACKET ou PACKET-HF.
settings-gateway = Gateway
settings-via = Via
settings-digipeaters-order-separated-spaces-usually = Digipeaters, em ordem, separados por espaços. Geralmente vazio — um gateway que você ouve diretamente é um gateway que você deve chamar diretamente.
settings-speed = Velocidade
settings-most-rms-gateways-answer-1200 = A maioria dos gateways RMS responde a 1200. Chamar um gateway de 1200 a 9600 soa exatamente como um gateway fora do ar, então defina isto a partir do que o dono do gateway publica, em vez de chutar. 9600 também exige a porta de dados do rádio nas duas pontas. 300 é HF.
settings-frequency = Frequência
settings-clear = LIMPAR
settings-zero-leaves-dial-alone-which = Zero deixa o dial em paz, o que você quer quando fica em um canal só. Qualquer outro valor sintoniza o rádio quando a sessão começa.
settings-my-gateways = Meus gateways
settings-winlink-s-published-gateway-list = A lista de gateways publicada pelo Winlink exige uma chave de API que o sdroxide não tem, então mantenha a sua. Os dois ou três gateways alcançáveis a partir de um local são descobertos tentando, e raramente mudam.
settings-use-2 = USAR
settings-call-one-next-connect = Chamar este no próximo connect
settings-current-dial = dial atual
settings-forget = ESQUECER
settings-forget-gateway = Esquecer este gateway
settings-add-gateway = + ADICIONAR GATEWAY
settings-save-gateway-above-list = Salvar o gateway acima na lista
settings-internet-gateway = Gateway de internet
settings-cms-address = Endereço do CMS
settings-client-name = Nome do cliente
settings-winlink-s-production-servers-only = Os servidores de produção do Winlink só aceitam nomes de cliente que conhecem, e respondem a um desconhecido com "Unknown client types are not allowed on production servers". Até que o sdroxide seja registrado junto à Winlink Development Team, conectar exige um nome que eles reconheçam.
settings-automatic-connection = Conexão automática
settings-connect-timer = Conectar por temporizador
settings-every = A cada
settings-persist-winlink-account = Salvar a conta Winlink
settings-callsign-lookup = Consulta de indicativo
settings-provider = Provedor
settings-auto-fill-name-qth-grid = Preencher automaticamente nome/QTH/locator ao clicar em spot e QSO
settings-choose-provider-enter-login = Escolha um provedor para informar o seu login.
settings-qrz-user = Usuário QRZ
settings-qrz-pass = Senha QRZ
settings-hamqth-user = Usuário HamQTH
settings-hamqth-pass = Senha HamQTH
settings-one-hamqth-account-does-both = Uma conta HamQTH faz as duas coisas — este login é o que a aba de envio HamQTH usa.
settings-upload = Enviar
settings-auto-upload-each-new-qso = Enviar automaticamente cada novo QSO
settings-master-switch-which-services-qso = A chave geral. Para quais serviços um QSO vai é definido em cada aba abaixo.
settings-turn-auto-upload-each-new = Ative primeiro "Enviar automaticamente cada novo QSO" acima — um serviço marcado aqui não é enviado até que isso esteja ativo.
settings-auto-upload-off-above-so = O envio automático está desligado acima, então as marcações de serviço ficam desativadas e nada é enviado automaticamente ainda — o botão ENVIAR por QSO no diário continua funcionando.
settings-qrz-log-key = Chave de log QRZ
settings-qrz-logbook-api-key-your = A chave de API do diário QRZ, das suas configurações do diário QRZ — não o login da consulta XML.
settings-eqsl-user = Usuário eQSL
settings-eqsl-pass = Senha eQSL
settings-eqsl-qth = QTH eQSL
settings-same-login-hamqth-callsign-lookup = O mesmo login da consulta de indicativo HamQTH — preencher um preenche os dois.
settings-club-log-email = E-mail do Club Log
settings-club-log-pass = Senha do Club Log
settings-club-log-key = Chave do Club Log
settings-wrl-api-key = Chave de API WRL
settings-developer-api-key-world-radio = A chave de API de desenvolvedor, em World Radio League → Integrations → Developer API. Ela é mostrada uma só vez, quando você a gera, então copie-a nesse momento. Os contatos vão para o seu diário padrão — defina um no WRL se tiver mais de um.
settings-confirmations-download = Confirmações (download)
settings-lotw-user = Usuário LoTW
settings-lotw-pass = Senha LoTW
settings-lotw-upload-uses-tqsl-export = O envio ao LoTW usa o TQSL — exporte o ADIF do diário e assine-o. As confirmações LoTW/eQSL são baixadas aqui para marcar trabalhado versus confirmado.
settings-sync-confirmations = SINCRONIZAR CONFIRMAÇÕES
settings-fetching-subscriptions = Buscando assinaturas…

    To use it on its own again, clear the Panadapter receiver box on that radio's page.
settings-panadapter = Panadaptador
settings-give-radio-wideband-panadapter-borrowing = Dê a este rádio um panadaptador de banda larga emprestando o receptor de outro rádio — um SDR na mesma antena, ou um alimentado pela saída de F.I. deste rádio. O espectro, a cascata, o sub receptor, os modos digitais e o skimmer vêm todos desse receptor; o dial, o modo, o filtro e o transmissor continuam aqui.
settings-receiver = Receptor
settings-none = Nenhum

    Takes effect on Apply.
settings-connected = Conectado a

    This is not the Converter field above, which retunes the radio, and not the CAT tab's I/Q centre offset, which is about this radio's own sound card. Nothing typed here is ever sent to either radio.

    Takes effect on Apply.
settings-audio-2 = Áudio de
settings-follow-dial = Seguir o dial
settings-keep-receiver-radio-s-dial = Mantém o receptor no dial deste rádio, e move o dial quando você sintoniza no panadaptador. Desligado deixa o receptor onde está — uma vigília fixa em um segmento enquanto o rádio vai para outro lugar.
settings-invert-spectrum = Inverter o espectro
settings-mirror-receiver-s-span-about = Espelha a largura do receptor em torno do centro, para uma tomada cujo oscilador fica acima do sinal e entrega a faixa ao contrário. Deixe desligado a menos que a cascata se encha de sinais que estão todos do lado errado do dial.
settings-mute-transmit = Silenciar na transmissão
settings-silence-receive-audio-while-radio = Silencia o áudio de recepção enquanto este rádio está transmitindo. Ligado por padrão: com o receptor na mesma antena, ou no F.I. deste rádio, o que ele ouve durante uma transmissão é o seu próprio transmissor. Transmissões acionadas no próprio rádio — o PTT do microfone, um pedal, MOX — também contam, assim que o link CAT informa que o equipamento foi para transmissão.
settings-blank-transmit = Apagar na transmissão
settings-stop-panadapter-waterfall-while-radio = Para o panadaptador e a cascata enquanto este rádio estiver transmitindo. Ligado por padrão: um transmissor pintado em toda a largura apaga a faixa atrás dele. Desligue para acompanhar a faixa — ou o seu próprio sinal — durante uma transmissão, o que vale ter com o receptor em uma antena separada. Vale para as transmissões que o sdroxide aciona; uma acionada no próprio rádio é recebida sem interrupção, então a imagem continua rodando independentemente do que isto diz.
settings-per-mode-offsets-radio-whose = Deslocamentos por modo, para um rádio cujo F.I. muda com o modo. Um modo deixado vazio usa o deslocamento acima.
settings-computer = Neste computador
settings-add-radio-computer = Adicionar um rádio neste computador
settings-radio-s-settings-are-below = As configurações deste rádio estão abaixo. Arraste para movê-lo ao longo da barra.
settings-switch-radio-dialog-follows-drag = Trocar para este rádio (a caixa acompanha). Arraste para movê-lo ao longo da barra.
settings-tx = ● TX
settings-mute-radio-s-audio = Silenciar o áudio deste rádio
settings-close-radio-configuration-kept = Fechar este rádio (a configuração é mantida)
settings-close-tab-close-radio-station = Fechar esta aba, ou fechar o rádio na sua estação
settings-close-tab = Fechar esta aba
settings-transverters-full = { $max } é o máximo que esta tabela comporta
settings-dialling = Discando { $url }…
settings-region-note = Onde fica a estação. Define todos os planos de faixas: as bordas das faixas, os subsegmentos de CW / dados / fonia na barra da cascata, onde os skimmers escutam, as frequências de chamada oferecidas e o que conta como fora da faixa para transmissão. Na Região { $region }, isso faz com que 70 cm { $region ->
        [1] 430–440 MHz — então 446 está fora da faixa, e 40 m para em 7.200
        [2] 420–450 MHz, e 40 m vai até 7.300
       *[3] 430–450 MHz, e 80 m para em 3.900
    }. Vale imediatamente, e se aplica a todos os rádios desta estação.
settings-rx-grid-reported = As recepções por este rádio são informadas a partir de { $grid }, não a partir do seu próprio locator.
settings-rx-grid-invalid = { $grid } não é um locator Maidenhead, então nada que este rádio ouve é informado. Quatro ou seis caracteres: duas letras, dois dígitos e, opcionalmente, mais duas letras.
settings-gateway-via = { " " }via { $via }
settings-auto-upload-to = Enviar automaticamente cada novo QSO para { $service }
settings-tle-updated = Atualizado: { $n } { $n ->
        [one] assinatura
       *[other] assinaturas
    }: { $total } satélites.
settings-tle-updated-failed = Atualizado: { $ok } de { $n }; { $failed } falharam — veja as linhas acima.
settings-panadapter-of = Este rádio é o panadaptador de { $radio }
settings-radio-n = rádio { $n }
settings-on-station = Em { $station }
settings-add-radio-on-station = Adicionar um rádio em { $station } — a estação o atende imediatamente
settings-panadapter-receiver-for = Receptor do panadaptador para { $radio }
settings-hang-up-stays-on = Desligar. O rádio continua em { $station }.
settings-close-on-station = Fechar { $radio } em { $station }? A configuração dele continua naquela máquina; a aba aqui fecha junto, e também a de todos os outros.
settings-close-it-on = Fechar em { $station }
settings-locator-hint = locator

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = Alertas sonoros
settings-alerts-sound-alarm-when-decode-matters = Tocar um alarme quando uma decodificação importa
settings-alerts-plays-over-own-audio-output = Toca pela própria saída de áudio, então é ouvido mesmo quando a faixa está em um alto-falante diferente desta tela.
settings-alerts-volume = Volume
settings-alerts-output = Saída
settings-alerts-test = Testar
settings-alerts-what-sound = O que soar
settings-alerts-each-station-quiet-while-after = Cada estação fica quieta por um tempo depois de um alerta, para que uma faixa movimentada não toque a cada intervalo. Voz e Tom + voz são lidos pela voz de anúncios falados — ative-a em Configurações → UI.

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = Amplificador
settings-amp-waiting-station-s-amplifier-configuration = Aguardando a configuração do amplificador da estação…
settings-amp-hobbypcb-hardrock-500-hardrock-50 = Um HobbyPCB Hardrock-500 ou Hardrock-50 na sua porta serial USB. O SDRoxide lê os seus medidores, troca a faixa, o modo e a antena dele, e controla o sintonizador a partir da janela AMP. O amplificador continua sendo acionado pela linha PTT do rádio, como antes.
settings-amp-enable = Ativar
settings-amp-model = Modelo
settings-amp-untested-panel-has-only-been = Não testado: este painel só foi testado com um Hardrock-500. Os limites de medidor deste modelo são escalonados a partir do Hardrock-500 e podem estar errados. Acompanhe o display do próprio amplificador.
settings-amp-port = Porta
settings-amp-choose = — escolha —
settings-amp-baud = Baud
settings-amp-must-match-amplifier-s-menu = Deve coincidir com o menu do amplificador. O painel de controle da HobbyPCB exige 115200.
settings-amp-follow-radio-s-band = Seguir a faixa do rádio
settings-amp-sends-transmit-frequency-amplifier-every = Envia a frequência de transmissão ao amplificador a cada troca de faixa, para que os filtros dele acompanhem o rádio sem um cabo CAT próprio. Nunca é enviado durante uma transmissão.
settings-amp-show-power-swr-tx-meter = Mostrar a potência e a ROE dele no medidor TX
settings-amp-only-where-radio-does-not = Só onde o rádio não as mede por conta própria, e só enquanto o amplificador estiver na linha.
settings-amp-off = Desligado
settings-amp-connecting = Conectando…
settings-amp-status = Estado
settings-amp-apply = {" "}APLICAR{" "}
settings-amp-save-re-open-amplifier-s = Salvar e (re)abrir a porta do amplificador

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = Teclado
settings-controls-click-shortcut-rebind-then-press = Clique em um atalho para redefini-lo, depois pressione a combinação de teclas (Esc cancela). As associações são ignoradas enquanto você digita em um campo de texto.
settings-controls-shortcut = Atalho
settings-controls-does = Faz
settings-controls-step-mode = Passo / modo
settings-controls-accel = Acel
settings-controls-on = Ligado
settings-controls-press-key = pressione uma tecla…
settings-controls-remove-binding = Remover esta associação
settings-controls-add-shortcut = + Adicionar atalho
settings-controls-restore-defaults = Restaurar padrões
settings-controls-bind-hold-talk-space = Associar falar-segurando à Barra de espaço
settings-controls-hold-space-transmit-releasing-losing = Segure a Barra de espaço para transmitir; soltá-la — ou perder o foco da janela — desacopla
settings-controls-unkey-held-ptt-after = Desacoplar um PTT segurado após
settings-controls-backstop-against-stuck-key-controller = Proteção contra uma tecla presa ou um controle que para de responder. 0 desativa.
settings-controls-panadapter-mouse = Mouse do panadaptador
settings-controls-wheel = Roda
settings-controls-wheel-shift = Roda + Shift
settings-controls-tune-step = Passo de sintonia
settings-controls-zoom-rate = Velocidade do zoom
settings-controls-click-tune-rounding = Arredondamento do clique para sintonizar
settings-controls-invert-wheel-direction = Inverter a direção da roda
settings-controls-left-drag-tunes-well-pans = Arrastar com o botão esquerdo sintoniza além de deslocar
settings-controls-off-makes-left-drag-pan = Desligado faz o arraste com o botão esquerdo apenas deslocar a visão, como o direito.
settings-controls-scroll-digit-frequency-readout-tune = Rolar um dígito no visor de frequência para sintonizá-lo
settings-controls-both-wheel-actions-are-tune = As duas ações da roda são Sintonizar — não sobra nenhuma forma de dar zoom.
settings-controls-restore-mouse-defaults = Restaurar padrões do mouse
settings-controls-mouse-buttons = Botões do mouse
settings-controls-left-right-buttons-are-reserved = Os botões esquerdo e direito são reservados para sintonia e deslocamento; os botões do meio e extras estão livres. Um botão lateral segurado para PTT se comporta como um pedal.
settings-controls-add-mouse-button = + Adicionar botão do mouse
settings-controls-f1-always-opens-manual-even = F1 sempre abre este manual, mesmo enquanto você digita, então não pode ser redefinido.
settings-controls-midi-controller = Controlador MIDI
settings-controls-midi-controllers-need-native-app = Controladores MIDI precisam do aplicativo nativo — o cliente de navegador não tem acesso a MIDI.
settings-controls-any-class-compliant-midi-surface = Qualquer superfície MIDI compatível com a classe funciona: a roda de jog de um controlador de DJ vira um bom botão de VFO, seus pads acionam PTT e botões de faixa, seus faders controlam ganhos.
settings-controls-enable = Ativar
settings-controls-controller = Controlador
settings-controls-feedback = Retorno para
settings-controls-rescan-ports = Buscar portas novamente
settings-controls-not-connected = Não conectado.
settings-controls-move-control-see-here = Mova um controle para vê-lo aqui.
settings-controls-control = Controle
settings-controls-reads = Lê como
settings-controls-led = LED
settings-controls-move = mova-o…
settings-controls-click-then-move-control-you = Clique e depois mova o controle que você quer associar
settings-controls-speed-sensitivity-spin-faster-tune = Sensibilidade de velocidade: gire mais rápido para sintonizar mais depressa
settings-controls-send-current-value-back-light = Envia o valor atual de volta, para acender um LED ou mover um fader
settings-controls-add-midi-control = + Adicionar controle MIDI
settings-controls-clear-all = Limpar tudo
settings-controls-endless-jog-encoders-send-relative = Encoders sem fim (jog) enviam um passo relativo em vez de uma posição, em uma de três codificações que parecem iguais com pequenos movimentos. APRENDER adivinha a partir de uma volta no sentido horário; se o botão então sintonizar no sentido errado, marque “inverter”.
settings-controls-none = — nenhum —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = Padrão do sistema
settings-general-remote-access = Acesso remoto
settings-general-what-remote-client-browser-page = O que um cliente remoto — a página do navegador, ou outro sdroxide iniciado com --connect — precisa apresentar antes que esta estação o deixe operar. Vale no modo servidor (--server); o próximo login pega uma alteração, sem reinício.
settings-general-username = Usuário
settings-general-password = Senha
settings-general-clients-must-give-password-leaving = Os clientes precisam informar a senha. Deixar o usuário vazio é aceitável.
settings-general-clients-must-sign = Os clientes precisam fazer login.
settings-general-empty-anyone-who-can-reach = ⚠ Vazio: qualquer um que alcance a porta do servidor pode operar este rádio, no seu indicativo. Defina uma senha antes de encaminhar a porta.
settings-general-stored-clear-config-toml-like = Armazenada em texto puro no config.toml, como as outras senhas que o sdroxide guarda.
settings-general-receive-audio-gain = Ganho do áudio de recepção
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    Ganho extra sobre tudo o que este rádio envia aos alto-falantes, além do controle de volume. Deixe em 0 dB a menos que o rádio esteja baixo com o volume no máximo: o topo do controle de volume é o áudio como chega, então ele pode diminuir um rádio mas nunca aumentá-lo, e a saída de som USB de alguns transceptores fica bem abaixo da escala total.

    Go up 6 dB at a time. Too much clips — the audio is limited at full scale rather than allowed to wrap round, so overdoing it sounds harsh rather than loud. Recordings are taken ahead of this and are not affected.
settings-general-reload-band-plan = RECARREGAR PLANO DE FAIXAS
settings-general-re-read-bandplan-json-machine = Relê o bandplan.json na máquina à qual o rádio está conectado e o aplica — bordas de faixa, subsegmentos e janelas de skimmer — sem reiniciar.
settings-general-running-station-s-own-band = Rodando com o plano de faixas próprio da estação.
settings-general-running-built-iaru-tables = Rodando com as tabelas IARU embutidas.
settings-general-settings-file = Arquivo de configurações
settings-general-settings-are-machine-radio-attached = As configurações ficam na máquina à qual o rádio está conectado. Exporte-as lá.
settings-general-export = EXPORTAR…
settings-general-write-every-setting-station-radios = Grava toda configuração desta estação — os rádios, os modos, os servidores, as memórias, o plano de faixas — em um arquivo que você pode levar para outra instalação. Seu diário de contatos e qualquer senha de servidor salva ficam aqui.
settings-general-import = IMPORTAR…
settings-general-replace-station-s-settings-ones = Substitui as configurações desta estação pelas de um arquivo exportado de outra instalação. Reinicie o sdroxide depois.
settings-general-import-overwrites-what-here-file = Uma importação sobrescreve o que está aqui, arquivo por arquivo, e tem efeito na próxima vez que o sdroxide iniciar — caso contrário, as configurações já na memória seriam gravadas de volta por cima. O que o arquivo não menciona fica como está, então um pacote de uma estação com um rádio não remove um segundo rádio daqui. Seu diário de contatos nunca está no arquivo: exporte-o como ADIF pela janela LOG se quiser levá-lo também.
settings-general-sdroxide-settings = Configurações do sdroxide
settings-general-swr-guard = Proteção de ROE
settings-general-stop-transmitting-high-swr = Parar a transmissão com ROE alta
settings-general-trip = Disparar em
settings-general-stops-transmission-when-radio-reports =
    Interrompe a transmissão quando o rádio informa uma ROE igual ou acima deste valor, e mantém a transmissão bloqueada até que você a reconheça. Pega uma antena desconectada, uma linha de alimentação com falha, ou uma chave deixada na porta errada.

    Tuning is treated differently, because feeding a mismatch is the point of it: an antenna tuner gets double the limit and about five seconds before the guard applies at all. A manual tuner that takes longer than that wants the guard switched off for the session.

    Needs a rig that reports SWR over CAT. Ignores the first fifth of a second of each transmission, and does not wait for high power.
settings-general-per-mode-settings = Configurações por modo
settings-general-reset-every-mode = REDEFINIR TODOS OS MODOS
settings-general-forget-every-mode-s-remembered = Esquece o AGC, o squelch, a redução de ruído, o notch e as chaves de estéreo lembrados de cada modo, e devolve os padrões próprios de cada modo.
settings-general-selecting-mode-lays-mode-s = Selecionar um modo aplica os valores iniciais próprios desse modo para AGC, squelch, redução de ruído, notch e chaves de estéreo no receptor, e alterar um deles o lembra apenas para esse modo. O chip de seta circular no fim da linha de filtro/ruído do receptor lista o que foi alterado e devolve o modo atual aos padrões.
settings-general-channels = Canais

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = Operador
settings-net-callsign-grid-are-set-general = O indicativo e o locator são definidos na aba Geral.
settings-net-set-your-callsign-grid-general = ⚠ Defina seu indicativo e locator na aba Geral.
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = Ativar
settings-net-connects-whenever-enabled-your-station = Conecta sempre que ativado. Sua estação só aparece para os outros enquanto o rádio estiver em RADE — em qualquer outro modo você continua conectado, mas oculto.
settings-net-station = Estação
settings-net-message = Mensagem
settings-net-receive-only-i-cannot-transmit = Somente recepção (não posso transmitir)
settings-net-server = Servidor
settings-net-host = Host
settings-net-port = Porta
settings-net-tls-wss = TLS (wss://)
settings-net-not-yet-implemented-freedv-gui = Ainda não implementado — a GUI do FreeDV também usa ws:// simples.
settings-net-reporting = Informes
settings-net-report-stations-i-decode = Informar as estações que eu decodifico
settings-net-sends-rx-report-each-callsign = Envia um rx_report para cada indicativo recuperado de um quadro End-of-Over do RADE.
settings-net-show-other-reporter-stations-spots = Mostrar outras estações reporter como spots
settings-net-adds-them-panadapter-overlay-world = Adiciona-as à sobreposição do panadaptador, ao mapa-múndi e à janela SPOTS sob o filtro FREEDV.
settings-net-set-your-callsign-grid-general-2 = ⚠ Defina seu indicativo e locator na aba Geral. Sem os dois, a conexão é apenas de visualização: você verá outras estações mas não aparecerá você mesmo.
settings-net-apply = {" "}APLICAR{" "}
settings-net-persist-re-connect = Salvar e (re)conectar
settings-net-season = Temporada
settings-net-downloading = baixando…
settings-net-using-built-copy = usando a cópia embutida
settings-net-your-own-stations-corrections-go = Suas próprias estações e correções vão neste arquivo. O sdroxide nunca o grava, e uma entrada aqui substitui uma programada com o mesmo nome e frequência.
settings-net-reload = Recarregar
settings-net-re-read-your-station-file = Relê seu arquivo de estações depois de editá-lo
settings-net-broadcast-stations-labelled-waterfall-come = As emissoras de rádio identificadas na cascata vêm da lista embutida nesta versão. Editá-las exige o aplicativo de desktop.
settings-net-ask-service-whether-these-credentials = Pergunta ao serviço se estas credenciais funcionam. Aplica primeiro as configurações acima, e não registra nada: apenas lê.
settings-net-checking = verificando…

## Settings window: messages with values filled in.

settings-amp-connected-to = Conectado ao { $model }
settings-controls-last-midi = Última mensagem: { $message }  valor { $value }
settings-general-band-plan-file = Toda borda de faixa e subsegmento vem de { $path }, gravado a partir das tabelas IARU embutidas na primeira vez e seu para editar depois — estreite uma faixa para a sua licença e o sdroxide recusará transmitir fora dela. As frequências estão em MHz; apague o arquivo para uma cópia nova dos padrões. Isto é a alocação regional, não a sua licença: suas condições próprias podem ser mais restritas, e os planos nacionais diferem dentro de uma região.
settings-general-exported = { $note } exportado — escolha onde salvar.
settings-general-export-failed = Falha na exportação: { $error }
settings-general-import-failed = Falha na importação: { $error }
settings-general-imported = { $summary } — reinicie o sdroxide para usá-los.
settings-general-import-skipped = { $path } ignorado: { $why }
settings-general-swr-tripped = ⚠ Atualmente disparado em { $swr }:1 — a transmissão está bloqueada.
settings-net-call-grid-set-on-general = { $call } / { $grid } — definido na aba Geral
settings-net-reporting-as = Informando como { $call } / { $grid } — SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ Falha no download: { $error }. Usando a programação embutida.
settings-net-test-service = Testar { $service }

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = A transmissão fica desligada enquanto um conversor estiver definido — diga o que há na linha de transmissão acima para religá-la.
settings-transmit-goes-through-same-converter = A transmissão passa pelo mesmo conversor, então o rádio trabalha o mesmo deslocamento em relação ao dial nos dois sentidos.
settings-transmit-not-converted-radio-transmits = A transmissão não é convertida: o rádio transmite na frequência que o dial mostra, enquanto a recepção passa pelo conversor.
settings-transmit-takes-own-offset-separate = A transmissão usa um deslocamento próprio, separado do de recepção.
settings-no-enabled-subscriptions-update = Não há assinaturas ativas para atualizar.
settings-add-radio-here-station-you = Adicionar um rádio — aqui, ou em uma estação à qual você esteja conectado

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = Toda borda de faixa e subsegmento vem do bandplan.json na máquina à qual o rádio está conectado. Isto é a alocação regional, não a sua licença: suas condições próprias podem ser mais restritas, e os planos nacionais diferem dentro de uma região.

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = Imagens recebidas → FTP
settings-picup-uploads-every-received-sstv-picture = Envia cada imagem SSTV recebida (e, se marcado, cada carta meteorológica) assim que é salva, organizada da forma que o QSSTV faz, para que páginas web escritas para o QSSTV funcionem sem alteração.
settings-picup-upload-received-pictures = Enviar imagens recebidas
settings-picup-server = Servidor
settings-picup-port = Porta
settings-picup-security = Segurança
settings-picup-user = Usuário
settings-picup-password = Senha
settings-picup-certificate = Certificado
settings-picup-trusted-when-computer-s-operating = Confiável quando o sistema operacional deste computador confia nele, como um navegador faria aqui. Uma CA privada pode ser adicionada abaixo (PEM).
settings-picup-extra-ca = CA extra
settings-picup-only-certificate-fingerprint-accepted-way = Só é aceito o certificado com esta impressão digital — a forma de confiar em um servidor autoassinado. TESTAR mostra a do servidor.
settings-picup-any-certificate-accepted-link-encrypted = ⚠ Qualquer certificado é aceito. A conexão é criptografada, mas qualquer um no caminho pode se passar pelo servidor e ler a senha.
settings-picup-plain-ftp-sends-password-pictures = ⚠ FTP simples envia a senha e as imagens sem criptografia.
settings-picup-sstv = SSTV
settings-picup-sstv-dir = Pasta SSTV
settings-picup-weather-fax = Fax meteorológico
settings-picup-wefax-dir = Pasta WEFAX
settings-picup-empty-login-directory = Vazio é o diretório de login.
settings-picup-file-names = Nomes de arquivo
settings-picup-format = Formato
settings-picup-add-extension = adicionar extensão
settings-picup-image1-png-rather-than-qsstv = image1.png em vez do image1 simples do QSSTV
settings-picup-caption-bar = Barra de legenda
settings-picup-time-frequency-mode-sender-s = Hora, frequência, modo e a identificação FSK do remetente no topo, como o QSSTV desenha. Só a cópia enviada é marcada.
settings-picup-fit-960-768 = Ajustar a 960 × 768
settings-picup-qsstv-s-heartbeat-tiny-picture = O heartbeat do QSSTV: uma imagem minúscula cuja altura é o minuto, enviada junto a cada uma, e feita com dois pixels de largura quando o programa é fechado.
settings-picup-nat-workaround = Correção de NAT
settings-picup-ignore-private-address-server-s = Ignora um endereço privado na resposta do modo passivo do servidor e usa o próprio — para um servidor atrás de NAT que não conhece o seu endereço público.
settings-picup-apply = {" "}APLICAR{" "}
settings-picup-test = TESTAR
settings-picup-connect-settings-above-applied-not = Conecta com as configurações acima — aplicadas ou não — faz login, e grava e apaga um pequeno arquivo em cada pasta de envio.
settings-picup-checking = verificando…
settings-picup-trust-certificate = Confiar neste certificado
settings-picup-pin-switch-pinned-fingerprint-one = Fixe-o: mude para "Impressão digital fixada" com esta. Confira-a com o servidor antes — isto é o que o servidor informou, não prova de quem ele é. Depois, APLICAR.
settings-picup-last-upload = Último envio

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = Perfis: as configurações salvas da estação
settings-profiles-profile-captures-whole-working-setup = Um perfil captura toda a configuração de trabalho — os dials e VFOs, o modo e os filtros, os ganhos, a excitação e as antenas, seu indicativo e modelos de mensagem, e os registros por faixa — e a aplica de volta com um clique. O hardware (back end, dispositivos de áudio, conversores) fica deliberadamente de fora: um perfil é a forma como você opera a estação, não o rádio em que ela roda.
settings-profiles-save-current-setup = Salvar a configuração atual
settings-profiles-profile-name = nome do perfil
settings-profiles-save = Salvar
settings-profiles-applies-whatever-sdroxide-attached-dials = Aplica-se ao que o sdroxide estiver conectado: dials e VFOs, modo e filtros, níveis, ganhos, excitação, antenas, a identidade digital e as pilhas de faixas.
settings-profiles-no-profiles-saved-yet-name = Nenhum perfil salvo ainda — dê um nome acima e pressione Salvar.
settings-profiles-saved-profiles = Perfis salvos
settings-profiles-apply = Aplicar

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = Chave de transmissão/recepção
settings-relay-waiting-station-s-t-r = Aguardando a configuração da chave T/R da estação…
settings-relay-closes-contact-while-station-transmits = Fecha um contato enquanto esta estação transmite — para desconectar e aterrar a entrada de antena do SDR, e para acionar um amplificador ou um relé T/R externo em sequência com ela. Funciona com as placas de relé USB baratas (LCUS, KMtronic, Numato), com uma linha serial RTS/DTR em qualquer interface que aceite um fechamento de PTT, e com qualquer outra coisa por meio de um gancho de comando.
settings-relay-hardware = Hardware
settings-relay-board = Placa
settings-relay-lcus-four-byte-a0-command = A LCUS é a placa de comando A0 de quatro bytes vendida sob uma dúzia de nomes; a KMtronic e a Numato também podem ser consultadas sobre como estão configuradas
settings-relay-port = Porta
settings-relay-choose = — escolha —
settings-relay-device = Dispositivo
settings-relay-gpio-pin = Pino GPIO
settings-relay-pin-3-every-homebrew-plan = Pino 3 em todo plano caseiro e em toda placa comercial — é o que fica no fim do encapsulamento, onde um fio pode ser soldado. Se o relé não estalar, a placa é um clone que nem implementa esses pinos.
settings-relay-gpio-chip = Chip GPIO
settings-relay-linux-only-gpiodetect-lists-chips = Somente Linux. `gpiodetect` lista os chips da máquina.
settings-relay-transmit = Na transmissão
settings-relay-receive = Na recepção
settings-relay-nothing-switched = Nada é comutado.
settings-relay-starting-program-costs-unpredictable-amount = Iniciar um programa custa um tempo imprevisível, e nada disso é contado na antecipação — então deixe uma margem maior aqui do que deixaria para uma placa de relé. Os números da tabela de contatos não significam nada para este link: seus comandos são executados uma vez ao pressionar a tecla e uma vez ao soltá-la.
settings-relay-contacts = Contatos
settings-relay-contact-1-rts-contact-2 = O contato 1 é RTS e o contato 2 é DTR.
settings-relay-no = Nº
settings-relay-gpio = GPIO
settings-relay-name = Nome
settings-relay-job = Função
settings-relay-tx-closes = TX fecha
settings-relay-lead = Antecipação
settings-relay-hold = Retenção
settings-relay-line-s-offset-chip-bcm = O deslocamento da linha neste chip — o número BCM em um conector de Raspberry Pi, que não é o número físico do pino
settings-relay-sdr-antenna = Antena do SDR
settings-relay-decides-default-timings-what-log = Define os tempos padrão e o nome que o log usa para ele — e, para um decodificador de faixa, que o contato segue a tabela de faixas abaixo
settings-relay-transmitting-energises-coil-off-transmitting =
    Ligado: transmitir energiza a bobina. Desligado: transmitir a libera.

    This is a wiring decision before it is a setting — see the note below.
settings-relay-how-long-before-rf-contact = Quanto tempo antes da RF este contato passa da palavra RX da sua faixa para a palavra TX. Uma resintonia o move de imediato; isto serve para o momento da tecla, quando um filtro só de transmissão precisa estar no circuito antes de a potência estar.
settings-relay-closed-long-before-rf-small = Fechado este tempo antes da RF. Um pequeno relé de coaxial comuta em 5 a 15 ms. A transmissão espera pelo maior destes, então um valor grande é uma falha audível no áudio de recepção; qualquer valor acima de 250 ms é ignorado.
settings-relay-how-long-after-rf-stops = Quanto tempo depois de a RF parar este contato volta da palavra TX para a palavra RX.
settings-relay-opened-long-after-rf-stops = Aberto este tempo depois de a RF parar. Mais longo que a antecipação de propósito: deixar a antena voltar com um momento de atraso não custa nada, e deixá-la voltar cedo custa um front end. Zero é uma resposta válida, e a certa para a linha de tecla de um amplificador.
settings-relay-test = {" "}TESTAR{" "}
settings-relay-not-while-station-air = Não enquanto a estação estiver no ar
settings-relay-close-contact-half-second-so = Fecha este contato por meio segundo, para que você possa ouvir o relé e conferir a fiação com o transmissor desligado
settings-relay-remove = Remover
settings-relay-add-contact = + adicionar um contato
settings-relay-if-will-not-answer = Se não responder
settings-relay-switch-exists-protect-receiver-not = Uma chave que existe para proteger um receptor não vale a pena se deixar de protegê-lo em silêncio — por isso recusar é o padrão. Escolha a outra opção quando um cabo USB solto encerrando o seu concurso for o pior desfecho.
settings-relay-order-timings-longest-lead-closes = A ordem são os tempos: a maior antecipação fecha primeiro, e a maior retenção abre por último. Então dê à chave da antena a antecipação e a retenção maiores, e ao amplificador as menores de cada uma — assim a antena é sempre comutada antes de o amplificador ser acionado, e o amplificador sempre é desacionado antes de a antena voltar.
settings-relay-transmit-sense-input = Entrada de detecção de transmissão
settings-relay-when-transceiver-keys-itself-own = Quando um transceptor se aciona sozinho — o próprio botão do microfone, pedal, VOX ou manipulador — o sdroxide descobre perguntando pelo CAT, e essa resposta chega algumas centenas de milissegundos depois do início da transmissão. Ligue a linha SEND do rádio, por meio de um optoisolador, a uma entrada de handshake desta mesma porta, e ela é vista em poucos milissegundos. O medidor e o intertravamento de transmissão ficam mais rápidos com isso.
settings-relay-line = Linha
settings-relay-transmitting = Transmitindo quando
settings-relay-high-line = a linha está em nível alto
settings-relay-most-opto-isolated-interfaces-pull = A maioria das interfaces optoisoladas puxa a linha para baixo quando o rádio aciona, então isto costuma ficar desligado
settings-relay-belongs-radio = Pertence ao rádio
settings-relay-which-radio-tab-sensed-transceiver = Em qual aba de rádio está o transceptor detectado. Em uma estação com um rádio, isto é 1 e nada depende disso.
settings-relay-needs-serial-port-handshake-lines = Exige uma porta serial — as linhas de handshake estão nela.
settings-relay-wiring-only-fail-safe-survives = A fiação é a única proteção de segurança que sobrevive a este programa.
settings-relay-choose-tx-closes-so-de = Escolha “TX fecha” para que o contato *desenergizado* seja o estado desejado quando a placa for desconectada, o computador estiver desligado ou o sdroxide não estiver rodando. Ligado de modo que desenergizado signifique aterrado, um relé morto deixa o SDR surdo — incômodo, e seguro. Ligado do outro jeito, um relé morto mais uma transmissão é um front end morto. Com um amplificador real atrás, escolha a primeira.
settings-relay-cannot-protect-first-instant-over = Isto não consegue proteger o primeiro instante de uma transmissão que você inicia no rádio. Ao acionar o rádio a partir do sdroxide, os contatos sempre antecedem a RF; com a entrada de detecção acima ligada, eles a seguem em milissegundos. Para um front end realmente digno de proteção, use também uma chave T/R de hardware com detecção de RF — nenhum programa em um computador pode ser a resposta completa.
settings-relay-contacts-transmit = contatos em TRANSMISSÃO
settings-relay-not-open = Não aberto.
settings-relay-status-unknown-press-apply = Estado desconhecido — pressione APLICAR.
settings-relay-station-air-apply-will-take = A estação está no ar — APLICAR terá efeito depois desta transmissão.
settings-relay-band-decoder-outputs = Saídas do decodificador de faixa
settings-relay-one-control-word-per-band = Uma palavra de controle por faixa: o bit 0 é o contato 1, o bit 1 o contato 2, e assim por diante, seguindo a numeração acima. RX segue a faixa do dial de recepção; TX a faixa do rádio que acionou — comutados na antecipação de cada contato antes da RF e de volta depois da retenção. Dê o mesmo valor aos dois para um filtro que não se importa com o sentido da RF, e valores diferentes para um desvio de pré-amplificador só de recepção ou um banco de LPF só de transmissão. Faixas deixadas em 0 não afirmam nada. Vale ao Aplicar / reconectar.
settings-relay-band = Faixa
settings-relay-rx = RX
settings-relay-tx = TX
settings-relay-outputs-asserted = Saídas acionadas
settings-relay-other = Outras
settings-relay-everywhere-outside-amateur-bands-short = Em todo lugar fora das faixas de amador — escuta em ondas curtas, e qualquer coisa em que o dial de um transverter caia e que nenhuma faixa cubra.
settings-relay-apply = {" "}APLICAR{" "}
settings-relay-persist-re-open-switch = Salvar e (re)abrir a chave

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = Conectar a um servidor
settings-remote-drive-sdroxide-running-somewhere-else = Controla um sdroxide rodando em outro lugar — a máquina da estação, um local remoto — pela rede. Inicie-o lá com --server e informe o endereço aqui. O rádio, os decodificadores e o diário de contatos ficam nessa máquina; esta tela recebe a cascata e o áudio, e usa o seu alto-falante e microfone.
settings-remote-address = Endereço
settings-remote-host-name-ip = nome do host ou IP
settings-remote-machine-running-sdroxide-server-host = A máquina que roda sdroxide --server: um nome de host, um endereço IPv4 ou IPv6, ou uma URL ws:// (ou wss://) completa se o servidor ficar atrás de um proxy reverso — uma URL é usada exatamente como digitada e ignora a porta e a chave abaixo.
settings-remote-port = Porta
settings-remote-port-server-listens-server-port = A porta em que esse servidor escuta — server_port no config.toml dele. 4950, a menos que tenha recebido --port.
settings-remote-secure = Seguro
settings-remote-wss-tls = wss:// (TLS)
settings-remote-tick-where-server-sits-behind = Marque isto quando o servidor estiver atrás de algo que termina o HTTPS por ele — um proxy reverso na porta 443, que é a forma usual de chegar a uma estação pela internet aberta. O servidor próprio do sdroxide fala ws:// simples e não quer isto.
settings-remote-connect = {" "}CONECTAR{" "}
settings-remote-open-station-radio-tab-own = Abrir esta estação como uma aba de rádio própria
settings-remote-client-cannot-hold-second-connection = Este cliente não pode manter uma segunda conexão.

    If the server asks for a username and password, its sign-in screen comes up in the new tab. A plain ws:// link carries none of this encrypted, so across the open internet either put it through a VPN or an SSH tunnel, or terminate HTTPS in front of the server and tick Secure above.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = Transmissão UDP do WSJT-X
settings-servers-waiting-station-s-wsjt-x = Aguardando a configuração de transmissão do WSJT-X da estação…
settings-servers-sends-decodes-station-status-logged = Envia decodificações, estado da estação e QSOs registrados da mesma forma que o WSJT-X, para que GridTracker, JTAlert, N1MM+ e Log4OM funcionem com o sdroxide sem alteração. Somente saída — nada neste socket pode tocar no rádio.
settings-servers-enable = Ativar
settings-servers-send = Enviar para
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 alcança os clientes desta máquina; um endereço da LAN ou um grupo multicast (224.0.0.1) alcança os outros
settings-servers-port = Porta
settings-servers-t-2237-port-every-client-defaults = 2237 é a porta que todo cliente usa por padrão
settings-servers-identify = Identificar-se como
settings-servers-name-clients-see-some-loggers = O nome que os clientes veem. Alguns loggers só aceitam tráfego que se identifique como WSJT-X.
settings-servers-n1mm-contactinfo-broadcast = Transmissão contactinfo do N1MM+
settings-servers-same-news-n1mm-s-words = As mesmas notícias nas palavras do N1MM: um datagrama XML por contato registrado, para os loggers que escutam o N1MM em vez do WSJT-X — a ponte da World Radio League aceita qualquer um. Somente contatos; não há fluxo de decodificações neste protocolo. Tem chave e porta próprias, porque uma estação pode querer as duas.
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 alcança os clientes desta máquina. A orientação do próprio N1MM para uma rede de concurso é o endereço de broadcast desta sub-rede — 192.168.1.255 para uma rede 192.168.1.n — que alcança todas as posições de uma vez.
settings-servers-t-12060-port-n1mm-s-documentation = 12060 é a porta que a documentação do N1MM recomenda
settings-servers-station-name = Nome da estação
settings-servers-what-n1mm-calls-stationname-name = O que o N1MM chama de StationName: o nome do computador que enviou o pacote. Os loggers o mostram para distinguir uma posição de operação de outra.
settings-servers-cw-panel-s-copy-plain = A cópia do painel de CW como texto simples, um datagrama UDP por trecho à medida que se fixa — para um controlador de estação, a janela de CW de um logger ou um script. Sem protocolo: um ouvinte apenas anexa o que chega (tente nc -ul 9999). Só o texto fixado é enviado, nunca o final que o decodificador ainda está revisando. Cada rádio tem a sua própria chave.
settings-servers-apply = {" "}APLICAR{" "}
settings-servers-persist-re-open-broadcast-sockets = Salvar e (re)abrir os sockets de transmissão
settings-servers-hamlib-rigctld-server = Servidor rigctld do Hamlib
settings-servers-waiting-station-s-rigctld-configuration = Aguardando a configuração do rigctld da estação…
settings-servers-listen = Escutar em
settings-servers-t-127-0-0-1-machine = 127.0.0.1 (esta máquina)
settings-servers-t-0-0-0-0-whole = 0.0.0.0 (rede inteira)
settings-servers-t-4532-port-every-rigctld-client = 4532 é a porta que todo cliente rigctld assume
settings-servers-rig-name = Nome do equipamento
settings-servers-reported-clients = informado aos clientes
settings-servers-max-clients = Máximo de clientes
settings-servers-allow-clients-transmit = Permitir que os clientes transmitam
settings-servers-off-refuses-every-key-request = Desligado recusa todo pedido de tecla e deixa de anunciar uma faixa de transmissão, então o próprio Hamlib se recusa a acionar.
settings-servers-rigctld-has-no-authentication-0 = ⚠ O rigctld não tem autenticação. Em 0.0.0.0, qualquer um que alcance esta porta pode sintonizar e acionar o seu transmissor.
settings-servers-t-1-client = 1 cliente
settings-servers-not-running = Não está rodando.
settings-servers-status-unknown-press-apply = Estado desconhecido — pressione APLICAR.
settings-servers-persist-re-bind-server = Salvar e (re)vincular o servidor
settings-servers-lets-any-hamlib-capable-program = Permite que qualquer programa compatível com Hamlib controle este rádio: frequência, modo, PTT, split, RIT e potência. No WSJT-X, fldigi ou CQRLOG escolha o equipamento “Hamlib NET rigctl” (modelo 2) e aponte para este endereço; no GPredict e no N1MM informe o host e a porta diretamente. Ao contrário do servidor TCI, carrega somente controle — sem áudio, sem IQ.
settings-servers-antenna-rotator = Rotor de antena
settings-servers-waiting-station-s-rotator-configuration = Aguardando a configuração do rotor da estação…
settings-servers-host = Host
settings-servers-min-elevation = Elevação mínima
settings-servers-below-rotator-parks-instead-tracking = Abaixo disto o rotor estaciona em vez de rastrear — defina-o para o seu horizonte local ou a linha do telhado
settings-servers-azimuth-offset = Deslocamento de azimute
settings-servers-added-every-command-rotator-whose = Somado a cada comando, para um rotor cujo norte está desalinhado
settings-servers-min-movement = Movimento mínimo
settings-servers-steps-smaller-than-are-not = Passos menores que isto não são enviados — perseguir décimos de grau gasta os motores à toa
settings-servers-park-position = Posição de estacionamento
settings-servers-drive-position-when-idle = levar a uma posição quando ocioso
settings-servers-off-leaves-antenna-wherever-last = Desligado deixa a antena onde a última passagem terminou
settings-servers-not-connected = Não conectado.
settings-servers-waiting-station-s-tci-server = Aguardando a configuração do servidor TCI da estação…
settings-servers-built-tci-server = Servidor TCI embutido
settings-servers-device-name = Nome do dispositivo
settings-servers-off-leaves-control-receive-streams = Desligado mantém o controle e os fluxos de recepção funcionando, mas todo pedido de tecla é recusado.
settings-servers-tci-has-no-authentication-0 = ⚠ O TCI não tem autenticação. Em 0.0.0.0, qualquer um que alcance esta porta pode sintonizar e acionar o seu transmissor.

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = Satélites: conjuntos de elementos e frequências
settings-tle-waiting-station-s-satellite-configuration = Aguardando a configuração de satélites da estação…
settings-tle-tracker-already-fetches-celestrak-s = O rastreador já busca por conta própria o grupo de radioamador do CelesTrak. Isto serve para todo o resto: os satélites meteorológicos da NOAA, um cubesat novo demais para estar no grupo, ou um conjunto de elementos mais recente que o que chegou. As listas são buscadas — e mantidas — onde o motor de rádio roda, então é o mesmo conjunto de satélites onde quer que o aplicativo esteja aberto.
settings-tle-subscriptions = Assinaturas
settings-tle-listings-fetched-kept-current-same = Listas buscadas e mantidas atualizadas, no mesmo ciclo de seis horas do conjunto de radioamador. Atualizadas enquanto a janela solar estiver aberta, e por ATUALIZAR AGORA aqui.
settings-tle-fetch-track-listing = Buscar e rastrear esta lista
settings-tle-remove-subscription = Remover esta assinatura
settings-tle-orbits = Órbitas
settings-tle-which-satellites-listing-get-orbit = Quais satélites desta lista recebem um anel de órbita e um rótulo. Um grupo inteiro pede “curado”: noventa anéis de uma vez é ilegível, e nenhum deixa noventa pontos anônimos.
settings-tle-nothing-listing-sdroxide-s-curated = Nada nesta lista está na lista curada do sdroxide — essa lista tem dez satélites de radioamador, então isto se comportaria exatamente como “nenhum”.
settings-tle-all-satellites = todos os satélites
settings-tle-catalogue-numbers-keep-comma-separated = Números de catálogo a manter, separados por vírgula. Vazio rastreia tudo o que a lista traz.
settings-tle-not-fetched-yet = ainda não buscado
settings-tle-subscription = + Assinatura
settings-tle-update-now = {" "}ATUALIZAR AGORA{" "}
settings-tle-ask-radio-engine-fetch-every = Pede ao motor de rádio que busque agora todas as assinaturas ativas
settings-tle-celestrak-groups = Grupos do CelesTrak
settings-tle-pasted-element-sets = Conjuntos de elementos colados
settings-tle-one-off-these-do-not = Para uso pontual. Estes não se atualizam sozinhos, e o SGP4 deixa de propagar um conjunto de elementos quando ele passa de uma quinzena da sua época — assine em vez disso o que pretende manter.
settings-tle-track-one = Rastrear este
settings-tle-show-two-element-lines = Mostrar as duas linhas de elementos
settings-tle-remove = Remover
settings-tle-paste-two-three-line-element = Cole aqui conjuntos de elementos de duas ou três linhas
settings-tle-add-pasted = + Adicionar colado
settings-tle-nothing-paste-box-looked-like = Nada na caixa de colagem parecia um conjunto de elementos.
settings-tle-clear-box = Limpar caixa
settings-tle-frequencies = Frequências
settings-tle-shown-under-pass-table-solar = Mostrado sob a tabela de passagens na janela solar. Uma entrada aqui substitui por completo a embutida para aquele número de catálogo, então parta de uma cópia dela, a menos que pretenda descartar o restante.
settings-tle-remove-satellite-s-entry = Remover a entrada deste satélite
settings-tle-link = ENLACE
settings-tle-downlink = DESCIDA
settings-tle-uplink = SUBIDA
settings-tle-mode = MODO
settings-tle-note = OBS
settings-tle-fm-repeater = Repetidora FM
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67,0 Hz
settings-tle-link-2 = + Enlace
settings-tle-copy-built = Copiar embutido
settings-tle-frequency-either-one-number-145 = Uma frequência é um número só (145.800) ou uma faixa de passagem de transponder escrita 145.950-145.970. Deixe uma direção em branco para uma baliza.
settings-tle-norad = NORAD
settings-tle-satellite = + Satélite
settings-tle-frequency-entry-needs-satellite-s = Uma entrada de frequência exige o número de catálogo NORAD do satélite.

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = Endereço
settings-wavelog-api-key = Chave de API
settings-wavelog-your-wavelog-s-front-page = O endereço da página inicial do seu Wavelog — https://log.example.org ou http://192.168.1.10/wavelog — e uma chave da página de API dele. Enviar exige uma chave de leitura e escrita.
settings-wavelog-plain-http-address-internet-api = http:// simples para um endereço na internet: a chave de API vai sem criptografia. Tudo bem dentro da sua própria rede; use https:// pela internet.
settings-wavelog-trusted-cert = Certificado confiável
settings-wavelog-forget-certificate = Esquecer este certificado
settings-wavelog-go-back-ordinary-checking-against = Volta à verificação comum com as autoridades certificadoras públicas — para um servidor que agora tem um certificado adequado.
settings-wavelog-connection-check-runs-machine-radio = A verificação de conexão roda na máquina à qual o rádio está conectado — use as configurações de lá para testá-la e listar os locais da estação.
settings-wavelog-server-presented-certificate-sha-256 = O servidor apresentou este certificado (SHA-256):
settings-wavelog-compare-your-server-s-e = Compare com o do seu servidor — por exemplo, `openssl x509 -noout -fingerprint -sha256 -in cert.pem` — e confie só se coincidirem. A partir daí este certificado exato é aceito e qualquer outro é recusado.
settings-wavelog-trust-certificate = Confiar neste certificado
settings-wavelog-station-location = Local da estação
settings-wavelog-none = — nenhum —
settings-wavelog-press-test-above-list-station = Pressione Testar acima para listar os locais de estação definidos no seu Wavelog.
settings-wavelog-default = Padrão
settings-wavelog-where-qsos-go-when-operating = Para onde vão os QSOs quando o perfil de operação não indica um local.
settings-wavelog-profile = Este perfil
settings-wavelog-use-default = Usar o padrão
settings-wavelog-not-set = não definido
settings-wavelog-no-station-location-chosen-uploads = Nenhum local de estação escolhido — os envios falharão até que um seja.
settings-wavelog-qsos-not-wavelog = QSOs que não estão no Wavelog
settings-wavelog-every-qso-logbook-wavelog = Todo QSO do diário está no Wavelog.
settings-wavelog-everything-logbook-wavelog-has-not = Tudo no diário que o Wavelog não confirmou — inclusive contatos anteriores ao envio automático. Os que ele já tem voltam como duplicados e são simplesmente marcados como enviados.
settings-wavelog-set-address-key-station-location = Defina primeiro o endereço, a chave e um local de estação.
settings-wavelog-yes-send-them = Sim, enviar
settings-wavelog-cancel = Cancelar
settings-wavelog-download-into-logbook = Baixar para este diário
settings-wavelog-downloading-runs-where-radio-attached = O download roda onde o rádio está conectado, para o diário daquela máquina — configure-o lá.
settings-wavelog-download-qsos-start-every = Baixar QSOs ao iniciar e a cada
settings-wavelog-downloaded-qsos-join-logbook-so = Os QSOs baixados entram no diário, então o FT8 e as listas de spots sabem o que você já trabalhou por outros programas e rádios. Cada download busca só o que é novo.
settings-wavelog-press-test-above-choose-which = Pressione Testar acima para escolher quais locais de estação baixar.
settings-wavelog-sync-now = Sincronizar agora
settings-wavelog-fetch-what-wavelog-has-added = Busca o que o Wavelog acrescentou desde o último download. Aplica primeiro as configurações acima.
settings-wavelog-full-resync = Ressincronização completa…
settings-wavelog-download-ticked-locations-whole-compare = Baixa inteiros os locais marcados e os compara com o diário: QSOs editados ou apagados no Wavelog, ou de um local que não está mais marcado, são postos de acordo. Você vê as mudanças antes de qualquer coisa acontecer.
settings-wavelog-full-resync-changes-make = Ressincronização completa — alterações a fazer
settings-wavelog-qsos-logged-here-imported-file = QSOs registrados aqui ou importados de um arquivo nunca são alterados nem removidos — apenas os que foram baixados do Wavelog.
settings-wavelog-apply = Aplicar
settings-wavelog-discard = Descartar
settings-wavelog-live-radio-status = Estado do rádio ao vivo
settings-wavelog-send-frequency-mode-wavelog-they = Envia frequência e modo ao Wavelog conforme mudam
settings-wavelog-radio-name = Nome do rádio
settings-wavelog-pick-radio-wavelog-s-live = Escolha este rádio no registro ao vivo do Wavelog e o formulário acompanha o dial. Exige uma chave de leitura e escrita. Somente a partir do rádio principal da estação.

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = SHA-256 do certificado do servidor: { $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = Devolve a estação a “{ $name }”: dials, VFOs, modo, filtros, ganhos, excitação, antenas, identidade e pilhas de faixas.
settings-profiles-delete = Excluir “{ $name }”

## Settings window, T/R switch: messages with values

settings-relay-not-usable = Ainda não utilizável: { $why }
settings-relay-receiving = recebendo
settings-relay-rx-tx-words = { $rx } / TX { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n ->
        [one] { $n } cliente
       *[other] { $n } clientes
    }
settings-servers-listening = ● Escutando em { $addr } — { $clients }
settings-servers-failed = Falhou: { $error }
settings-servers-rotator-at = ● Conectado — antena em { $az }° az, { $el }° el

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } satélites · { $age } de idade
settings-tle-subscribed = Assinado em { $name }. Pressione ATUALIZAR AGORA para buscá-lo.
settings-tle-norad-too-stale = NORAD { $norad } · { $age } de idade — velho demais para propagar
settings-tle-norad-age = NORAD { $norad } · { $age } de idade
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = { $n } { $n ->
        [one] conjunto de elementos adicionado
       *[other] conjuntos de elementos adicionados
    }.
settings-tle-added-refreshed = { $added } adicionado(s) e { $r } { $r ->
        [one] conjunto de elementos atualizado
       *[other] conjuntos de elementos atualizados
    }.
settings-tle-links = { $n } { $n ->
        [one] enlace
       *[other] enlaces
    }
settings-tle-replace-links = Substituir estes enlaces pelos embutidos de { $name }

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = O local "{ $location }" é { $station_call }, mas você registra como { $call } — o Wavelog recusará estes QSOs.
settings-wavelog-grid-mismatch = O local "{ $location }" fica em { $station_grid }, mas o seu locator é { $grid } — o Wavelog recusará estes QSOs.
settings-wavelog-location-n = local nº { $id }
settings-wavelog-saved-with-callsign = Salvo com o indicativo ({ $call }) em Configurações → Perfis, para que um perfil portátil possa registrar em um local portátil.
settings-wavelog-owed = { $n } { $n ->
        [one] QSO registrado
       *[other] QSOs registrados
    } desde que o envio automático foi ativado ainda por enviar — nova tentativa a cada 5 minutos.
settings-wavelog-send-unsent = Enviar { $n } { $n ->
        [one] QSO ainda não presente
       *[other] QSOs ainda não presentes
    } no Wavelog…
settings-wavelog-send-confirm = Enviar { $n } QSOs para { $location }? As configurações são aplicadas primeiro.
settings-wavelog-plan-add = • { $n } QSOs do Wavelog a acrescentar ao diário
settings-wavelog-plan-update = • { $n } QSOs baixados alterados no Wavelog a atualizar
settings-wavelog-plan-remove = • { $n } QSOs baixados que não estão mais no Wavelog (ou de um local desmarcado) a remover
settings-wavelog-plan-mark = • { $n } dos seus próprios QSOs a marcar como presentes no Wavelog

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = Antena
settings-radio-which-socket-back-radio-using =
    Qual conector na parte traseira o rádio está usando — o próprio comando ANT dele, a mesma configuração do botão ANT do painel frontal.

    Applies immediately, and it is the radio's setting rather than a copy kept here: the socket the radio is on is read back when the session opens.

    The choice is remembered per band. Switch to the beam on 2 m and the wire on 40, and each comes back the next time the dial crosses into that band — and the next time sdroxide starts. A memory channel stored here carries its socket too.
settings-radio-receive-antenna = Antena de recepção
settings-radio-radio-s-separate-receiving-antenna =
    A antena de recepção separada do rádio — RX ANT na traseira — ligada ao caminho de recepção ou desligada dele. Configuração própria, a mesma do botão RX ANT do rádio.

    The aerial on the main socket stays on transmit either way: this is an extra input for receiving, a loop or a beverage, not a choice of socket.

    Unlike the antenna above, this is *not* remembered here. The radio holds it per band itself, so sdroxide reads it back after every band change and shows what the radio says; ticking this box is the only thing that moves it.
settings-radio-receive-path = No caminho de recepção
settings-radio-radio-power = Energia do rádio
settings-radio-switch-radio-itself-off-back =
    Desliga o próprio rádio e liga-o de novo, pelo link de controle. Esta é a única liga/desliga real do programa — não a chave LINK do próprio sdroxide, que fecha a ponta do sdroxide e deixa o rádio funcionando.

    For the switch back on to reach anything, the radio's control end has to stay awake while it is off. Over the network that is what Network Control does. Over a serial cable it is the radio's CI-V port, which stays powered from the mains supply on a set that is switched off at the front rather than unplugged; sdroxide sends the wake-up run Icom's own documentation asks for in front of the power-on.

    Switching off ends the audio and the meters — there is no radio behind them — and sdroxide keeps the control link open so the switch back on has somewhere to go.
settings-radio-on = Ligado
settings-radio-off = Desligado
settings-radio-waiting-configuration-machine-radio-attached = Aguardando a configuração da máquina à qual o rádio está conectado.
settings-radio-sound-format = Formato de som
settings-radio-invert-spectrum = Inverter o espectro
settings-radio-swap-i-q = Trocar I/Q
settings-radio-mirror-panadapter-about-tuned-frequency =
    Espelha o panadaptador em torno da frequência sintonizada, para um equipamento que traz I e Q invertidos na própria placa de som. O indício é uma cascata cheia de sinais convincentes, todos do lado errado do dial, com a SSB saindo na banda lateral oposta — trocar os dois cabos na placa de som resolveria do mesmo jeito.

    Receive only: transmit hands the radio one real audio signal, which has no sideband to invert.
settings-radio-i-q-sample-rate = Taxa de amostragem I/Q
settings-radio-how-fast-radio-s-i =
    Em que velocidade a placa de som I/Q do rádio é executada — e, portanto, quanta faixa o panadaptador mostra, porque um fluxo em quadratura abrange toda a sua taxa de amostragem: 48 kHz dá ±24 kHz de cada lado do dial, 192 kHz dá ±96.

    The card has the final say. One that cannot do the rate picked here is opened at the nearest it can, and the panadapter is that much narrower — the log says so at startup when it happens.

    A faster card is also more work: the machine has proportionally less time to empty it, and one that cannot keep up drops samples. That shows up as audio breaking up while the waterfall still looks perfect, and it too is logged. If you see it, come back down a step.
settings-radio-t-48-khz-24-khz = 48 kHz (±24 kHz)
settings-radio-t-96-khz-48-khz = 96 kHz (±48 kHz)
settings-radio-t-192-khz-96-khz = 192 kHz (±96 kHz)
settings-radio-t-384-khz-192-khz = 384 kHz (±192 kHz)
settings-radio-i-q-centre-offset = Deslocamento do centro I/Q
settings-radio-how-far-radio-s-i =
    Quanto a saída I/Q do rádio é centralizada acima do próprio dial, para um equipamento cujo F.I. de recepção foi deslocado de zero — em um Elecraft, MENU:RX SHFT ajustado para 8.0 em vez de NOR, o que tira o dial do pico de CC do misturador (e impede que uma forte estação SSB/AM próxima seja detectada como AM).

    Not a converter: the radio still displays and transmits on the real frequency, and nothing here is ever sent to it. This only says where the samples on the sound card sit, and the stream is shifted back onto the dial as it arrives.

    Leave at 0 unless you have turned such a menu entry on. If signals land twice the offset away, the sign is the other way round.
settings-radio-iq-correction = Correção I/Q
settings-radio-cancel-mirror-image-every-signal =
    Cancela a imagem espelhada de cada sinal e o pico de CC no meio da cascata.

    A radio's I/Q output is two analogue paths that are never quite equal in gain nor exactly 90° apart, and what that leaves is a copy of every signal reflected about the tuned frequency, usually 30-40 dB down — strong enough to look like a station that is not there, and to be decoded as one. Radios with front-panel balance trimmers are adjusting exactly this; here it is measured off the received noise and needs no setting.

    Leave it on. Turn it off if you are listening to AM tuned dead on the carrier — the carrier is DC, so it goes with the spike — or to check whether a signal is real by watching whether it survives.
settings-radio-cancel-mirror-images = Cancelar imagens espelhadas
settings-radio-dc-notch = Notch de CC
settings-radio-widen-hole-taken-out-middle =
    Alarga o buraco retirado do meio da faixa, para um rádio cujo pico central é mais largo que o deslocamento logo abaixo dele.

    0 leaves the ordinary blocker, which is a few tens of hertz wide and removes the offset without touching anything else. Wind it up and the bottom of the span goes with it: a first-order high-pass, 3 dB down at the figure set here and further in below it.

    It is centred where the radio's I/Q is centred, which is the dial unless the offset above has moved it — so anything tuned there goes too. A CW note at 600 Hz is inside a 600 Hz setting.
settings-radio-panadapter-bw = Largura do panadaptador
settings-radio-cat-family = Família CAT
settings-radio-rigctld-address = Endereço do rigctld
settings-radio-host-port-running-hamlib-rigctld =
    host:porta de um rigctld do Hamlib em execução — 127.0.0.1:4532 é o padrão dele, nesta máquina.

    Start one with, for example, `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, where -m is the Hamlib model number for your radio (`rigctl -l` lists them).

    This is the catch-all: it reaches the frequency, mode, PTT, power, S-meter and SWR and nothing else. Where one of the native families above fits your radio it does more — keying from the rig's own text buffer, the receive filter, and per-model meter scales.
settings-radio-flrig-address = Endereço do flrig
settings-radio-host-port-running-flrig-127 =
    host:porta de um flrig em execução — 127.0.0.1:12345 é o padrão dele, nesta máquina. O flrig serve XML-RPC sempre que está rodando; a porta fica em Config → Setup → Server.

    Like the Hamlib option this drives a daemon rather than the radio, but through flrig's own per-model driver — on a number of rigs its power and filter handling is the more faithful of the two. It reaches the frequency, mode, PTT, transmit power (in whole watts), the receive bandwidth, the S-meter, SWR and power-out. CW keys through flrig's own cwio port, which must be configured in flrig itself.
settings-radio-serial-port = Porta serial
settings-radio-select = — selecionar —
settings-radio-baud = Baud
settings-radio-data-bits = Bits de dados
settings-radio-parity = Paridade
settings-radio-stop-bits = Bits de parada
settings-radio-force-rts = Forçar RTS
settings-radio-force-dtr = Forçar DTR
settings-radio-ptt-method = Método de PTT
settings-radio-how-transmit-keyed = Como a transmissão é acionada.
settings-radio-how-transmit-keyed-network-link = Como a transmissão é acionada. Um link de rede não tem linhas de controle, então DTR e RTS não acionam nada aqui — use CAT, que pede ao daemon para acionar o rádio, ou VOX.
settings-radio-mode-control = Controle de modo
settings-radio-digimode-mode = Modo do modo digital
settings-radio-what-put-radio-mode-sdroxide =
    Em que colocar o rádio para um modo que o sdroxide modula pela placa de som. "DIGI" seleciona a posição DATA/PKT do equipamento, que pega o áudio de transmissão da entrada USB ou ACC com o processamento de voz do caminho do microfone fora dele; "USB" deixa na banda lateral comum, para um rádio já ajustado para modular SSB pela entrada de dados, ou um sem posição DATA nenhuma.

    It covers every digital mode — FT8, FT4, PSK, RTTY, SSTV and the rest — and CW sent as "Sound card", and it overrides Mode control for them. SSTV is the one whose sideband follows the band, so there it means the DATA position on that sideband: USB-D above 40 m, LSB-D at and below it. SSTV-FM is not part of it — an FM carrier has no sideband to choose.
settings-radio-cw-keying = Acionamento de CW
settings-radio-how-cw-panel-s-keyer =
    Como o manipulador do painel CW transmite. "Manipulador do rádio" coloca o rádio em CW e lhe entrega o texto a enviar com o manipulador próprio dele. Usa a velocidade de manipulação do equipamento (definida pelo WPM do painel), e na Yaesu envia pela memória de manipulador 1, sobrescrevendo o que estava guardado nela.

    It needs break-in on at the radio, which sdroxide turns on itself with each message on Icom, Yaesu and Kenwood — a rig with break-in off takes the message and transmits nothing. It is left on afterwards, and a full break-in is never turned down to semi.

    "Sound card" sends the keyed sidetone as audio instead (MCW), a tone at dial + pitch — and because a rig in CW would ignore its sound card entirely, selecting CW then follows the Digimode mode setting (USB, DATA, or Radio controlled) instead of switching the rig to CW. Pick "Radio controlled" there for rigs whose data position can't be commanded over CAT (a Xiegu's U-D) and park the rig on it yourself, as for FT8.
settings-radio-poll-rate = Taxa de consulta
settings-radio-how-often-radio-asked-what =
    Com que frequência o rádio é perguntado sobre o que está fazendo — o dial, o modo e os medidores. Esta é a metade do link de controle que vai do rádio de volta ao sdroxide: gire o botão VFO do equipamento ou troque o modo no painel frontal e a leitura, a faixa e o panadaptador acompanham em uma consulta.

    It is also the whole of the control traffic this end generates, and on a modern Icom that traffic is not free: the CI-V port and the sound card sit behind one USB hub inside the radio, so frames asked for here are bus time the audio does not get — heard as dropouts that look like a DSP fault and are not one. The default of 2 Hz is half a second behind the knob and quiet enough to stay out of the audio's way. Raise it where the control port is its own device (a separate USB-serial adapter, a network rigctld); lower it if the radio's audio breaks up.

    An Icom with CI-V Transceive switched on in its own menu needs none of this for the dial: it reports the knob the instant it moves, and sdroxide stands the dial poll down as soon as it sees it do so — and picks it back up again if the radio ever moves without saying so.

    The rate is the dial's. The mode rides along with only every fourth poll, since it is a setting that changes a few times in an evening rather than one that follows a knob.
settings-radio-send-command = Comando de envio
settings-radio-which-transceiver-generation-keys-rig =
    Qual geração de transceptor aciona o equipamento, para o método de PTT "CAT". As duas discordam sobre o que o parâmetro TX significa, e nada no fio as distingue, então escolha a que combina com o seu rádio.

    "TS-2000 style (TX;)" — TS-480, TS-570, TS-870, TS-2000: the ordinary send, on the main band.

    "TS-590 style (TX1;)" — TS-590S/SG, TS-890, TS-990: DATA SEND, which keys with the ACC2/USB audio input live. On these rigs the plain send selects the microphone instead and mutes the audio sdroxide transmits.

    Set wrong, a TS-590 transmits silence — but a TS-2000 transmits on the sub-band, which is another band entirely.
settings-radio-radio = Rádio
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    Um perfil cobre a família: o conjunto de comandos do K3, que o KX3, o KX2 e o K4 respondem. Não há nada a escolher aqui — quantos watts o controle de Excitação abrange (12 em um KX2 ou KX3 puro, 110 com um KPA3 ou um KXPA100) é lido da própria consulta de módulos de opção do equipamento quando a porta abre.

    Note the baud rates above: the K3, K3S, KX3 and KX2 go no faster than 38400, and a rig set below the rate chosen here answers nothing at all.
settings-radio-transmit-input = Entrada de transmissão
settings-radio-where-radio-takes-transmit-audio =
    De onde o rádio pega o áudio de transmissão — o comando TI do equipamento, que é o menu 32 "TX IN" no painel frontal. Afirmado quando a porta abre.

    "USB audio" is what makes this interface work: the FDM-DUO transmits what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.

    "Auto" lets the rig choose — the microphone for a PTT press on the microphone, the USB port for a CAT or RTS key-down.

    "Leave as set on the radio" sends no TI at all, for an operator who sets this at the front panel.
settings-radio-which-two-sockets-back-receiver =
    Qual dos dois conectores da traseira o receptor escuta — o comando AN do equipamento, que é o menu 31 "ANTENNAS" no painel frontal e o indicador "ANT 1 2" no display.

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    Applies immediately, and is read back from the radio when the port opens: this is the rig's own setting rather than a copy kept here.
settings-radio-one-profile-covers-both-there =
    Um perfil cobre os dois. Não há nada a escolher aqui — o rádio se identifica quando a porta abre.

    This is the CAT half only: it drives the dial, the mode, PTT, the S-meter, the SWR and the transmit power over the rig's CAT USB port, and takes audio from its USB Audio port. The FDM-DUO's third USB port carries wideband I/Q, which this interface does not read — select the ELAD interface above for that.

    Note the baud rate: menu 70 "CAT BAUD" on the radio must match, and it ships at 38400.

    CW is keyed by the radio's own key or paddle. The FDM-DUO has no command that accepts text, so the CW panel cannot key it over CAT.
settings-radio-one-profile-covers-range-qrp =
    Um perfil cobre a linha: o conjunto de comandos próprio da QRP Labs, que é um subconjunto do Kenwood TS-480 com bastante coisa acrescentada. Não há nada a escolher aqui — o rádio informa o nome e a versão de firmware quando a porta abre, e a versão é o que decide se a leitura de proteção de ROE é pedida.

    Not the Kenwood profile: on a QMX the PC command is the power METER rather than the power control, so a radio driven as a Kenwood would have its meter read written to as if it were a setting, and MD8 — a Kenwood mode — is SWR Tune here.

    The transmit power is set at the radio. There is no CAT command for it, so the Drive slider only reaches the level of the audio going into the sound card.

    The receive filter is the radio's too: it reports the width its mode implies (3.2 kHz in Digi, 300 Hz in CW) and offers nothing to change it with.
settings-radio-baud-rate-ignored-port-usb = A taxa de baud é ignorada — a porta é USB
settings-radio-qmx-serves-own-virtual-com =
    Um QMX serve as próprias portas COM virtuais por USB, então a taxa definida acima não tem efeito em nenhuma das pontas. Ele oferece até três delas; se o rádio não responder nada, a causa mais provável é a errada.

    One thing does matter about the port: a carriage return sent to it switches the radio into terminal mode for the rest of the session. sdroxide never sends one — but a terminal program left open on the same port will, and CAT stops working the moment it does.
settings-radio-i-q-mode-switched-radio = O modo I/Q é ligado no rádio
settings-radio-radio-s-sound-card-carries =
    A placa de som do rádio leva ou o áudio demodulado ou o I/Q bruto que o ADC dele enxerga, e o sdroxide afirma o que o Formato de som acima pede quando a porta abre (o comando Q9 do rádio, a entrada de menu "IQ mode").

    The centre offset and sample rate above have been set to what a QMX needs: its receiver is a superhet with a 12 kHz I.F., so the synthesiser sits 12 kHz below the dial and everything on the card is 12 kHz above the middle of the span. The card runs at 48 kHz, which makes the panadapter 48 kHz wide.

    In CW the radio moves that I.F. by a further ~700 Hz, which one figure cannot follow — add it by hand if you run the panadapter with the radio in CW. In Digi, where a radio used as an I/Q front end normally sits, there is nothing to add.

    QRP Labs note that I/Q mode is not suitable for WSJT-X and other programs that expect demodulated audio; here the demodulation happens on this side, so it is.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ (transceptor HF de 5 W)
settings-radio-hobbypcb-s-rs-hfiq-there =
    O RS-HFIQ da HobbyPCB. Não há nada a escolher aqui: o perfil é todo o conjunto de comandos do rádio, e a versão de firmware é registrada no log quando a porta abre.

    The control link carries two things and no more — where to put the oscillator, and whether to transmit. There is no mode, power, filter, squelch or meter command, because the radio has none of them. Everything else is sdroxide's: what comes off the sound card is complex baseband centred on the dial, so the mode, the filter and the modulation all happen on this side.

    The frequency command covers 3–30 MHz. Ask for anything outside that and the radio refuses it and stays where it is; the log says so.

    CW is keyed as audio through the transmit chain — this radio has no keyer that takes text. (It has an internal CW generator; sdroxide never uses it, on the firmware's own advice.)
settings-radio-sound-format-baud-ptt-are = Formato de som, taxa de baud e PTT são do rádio, não configurações
settings-radio-all-three-have-been-filled =
    Os três foram preenchidos, e nenhum deles é preferência:

    • Sound format is I/Q (stereo) — the card carries complex baseband and nothing else.
    • 57600 baud, 8N1 — the firmware's one rate, with no menu to change it. Any other is a silent link, so it is pinned when the port opens.
    • PTT over CAT — the *X command is the only transmit switch the interface has.

    The centre offset has been set to zero: the synthesiser runs at four times the dial into a quadrature detector, so the middle of the span is the dial. Set the sample rate above to whatever your sound card is actually running at; that is what makes the panadapter as wide as it is.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    O transceptor QRP de bolso de DL2MAN/PE1NNZ e o firmware aberto uSDX de onde ele surgiu. Ele emula um Kenwood TS-480, mas o subconjunto é magro: o dial, o modo, o PTT e a limpeza de RIT/XIT são tudo o que ele responde. Sem S-metro, sem ROE, sem controle de potência, sem VFO B, sem split e sem manipulador CAT, então o controle de Excitação e os medidores não fazem nada pelo CAT — o nível de áudio é o único controle de transmissão que existe.

    CW is keyed as audio (MCW) through the sideband, or with the key or paddle at the radio; there is no command that takes text.
settings-radio-audio = Áudio
settings-radio-tr-usdx-has-no-sound =
    Um (tr)uSDX não tem placa de som própria e tem duas formas de ser ouvido, e qual é a certa depende do seu shack, não do rádio.

    "One cable" takes the audio inside the CAT serial link itself — the radio's own 8-bit stream, over the same USB cable as the control, with nothing else to plug in. The catch is the firmware: it cannot take a CAT command while its stream is running (a command sent into the stream does not pause it, it kills it), so this mode never polls. Tuning and mode changes are sent as you make them, each pausing the stream for a moment and starting it again, so expect a brief gap on each change; nothing is sent while you are simply listening. The radio's own dial and mode are not followed either — there is no read to follow them with — so drive it from here.

    "USB sound card" takes the audio from an external sound card wired to the radio's 3.5 mm speaker/mic jack, and behaves as any other CAT rig: the dial poll runs, so the radio's own knob and mode are followed. Pick the card under Radio audio below.

    Both modes open the port with DTR held high: the serial adapter's DTR is wired to the radio's reset, so opening the port reboots it and the first second or two is quiet while it comes up. DTR is never used to key.
settings-radio-radio-model = Modelo do rádio
settings-radio-which-icom-two-things-ci =
    Qual Icom, pelas duas coisas que o CI-V não faz da mesma forma em todos.

    The transceiver address below is filled in from it: every model ships with a different one, and a frame sent to the wrong address is simply ignored — a radio that answers nothing, with no error anywhere to say why.

    It also decides whether sdroxide can select DATA mode. On CI-V, USB and USB-DATA are the same mode byte and a separate command tells them apart; without it a digital-mode over goes out through the microphone input, with the rig's speech processing and SSB filter in the path.

    "Other" leaves the address to you and sends no DATA-mode command at all.
settings-radio-radio-id-hex = ID do rádio (hex)
settings-radio-show-radio-s-spectrum-scope = Mostrar o scope de espectro do rádio
settings-radio-scope-span = Largura do scope
settings-radio-how-wide-sweep-radio-keeps = Quão larga é a varredura. O rádio mantém a largura escolhida por último na própria tela — muitas vezes alguns kHz. Definir uma largura aqui também coloca o scope em modo de centro, para que ele siga o dial. Isso altera também a tela do próprio rádio; "Como ajustado no rádio" a deixa intacta.
settings-radio-press-apply-reconnect-switch-without = Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-radio-audio-sound-card = Áudio do rádio (placa de som)
settings-radio-devices = Dispositivos
settings-radio-discover = Descobrir
settings-radio-no-devices-press-discover = nenhum dispositivo — pressione Descobrir
settings-radio-manual-ip = IP manual
settings-radio-optional-e-g-192-168 = opcional, ex.: 192.168.1.50
settings-radio-sample-rate = Taxa de amostragem
settings-radio-receiver-ddc = Receptor (DDC)
settings-radio-protocol-2-board-carries-several = Uma placa Protocol 2 leva vários receptores sintonizáveis de forma independente (DDCs) em uma mesma conexão — rode este rádio no DDC1 e outro rádio, no mesmo endereço, no DDC2. O transmissor pertence ao rádio do DDC1. Placas Protocol 1 têm só o DDC1.
settings-radio-lna-gain = Ganho do LNA
settings-radio-front-end-gain-hermes-lite = Ganho do front end de uma Hermes-Lite 2. Tem efeito imediatamente — sem reconectar — e é lembrado como o nível em que o rádio começa. Alto demais satura o ADC e a faixa inteira parece distorcida; baixo demais e o receptor fica surdo.
settings-radio-overload-protection = Proteção contra sobrecarga
settings-radio-back-lna-gain-off-itself =
    Reduz o ganho do LNA por conta própria enquanto a placa informa que o ADC está estourando, e o eleva de novo quando isso para.

    A Hermes-Lite 2 samples the whole of 0-38 MHz onto one 12-bit converter with no mixer and no preselector in front of it, so a broadcast station a band away can drive it into overflow while the band you are looking at shows nothing wrong at all — the noise floor climbs, everything intermodulates and the decoders stop. The board knows, and this is what acts on it.

    Off by default: it moves a control you set. Nothing happens while transmitting.
settings-radio-wind-gain-back-when-adc = Reduzir o ganho quando o ADC estourar
settings-radio-applies-apply-reconnect-main-window = Vale em Aplicar / reconectar. A barra de Ganho da janela principal mostra o ganho que a placa está de fato executando, então acompanha o laço e você pode ver o que ele faz; o controle deslizante acima é o nível em que o rádio começa, e o laço não o reescreve.
settings-radio-how-far-gain-moves-each = Quanto o ganho se move a cada vez, em dB. Um é o passo do próprio registro de ganho da placa.
settings-radio-attack-decay = {"  "}Ataque / decaimento
settings-radio-how-often-gain-may-come =
    Com que frequência o ganho pode descer enquanto o conversor está estourando, e com que frequência pode subir de novo depois que parou.

    The two are deliberately a hundred times apart. Retreat immediately: every millisecond of overflow is a receiver full of intermodulation. Return slowly: whatever caused it — a neighbour keying, a broadcaster coming up at dusk — has usually not gone away, and a loop that recovered as fast as it retreated would spend the evening oscillating across the threshold. 100 ms and 10 s per decibel is what PowerSDR's Auto S-Att and N1GP's HermesIntf have both used.
settings-radio-range = {"  "}Faixa
settings-radio-lowest-highest-gain-loop-may = O menor e o maior ganho que o laço pode usar, em dB. O teto impede que ele decida quão sensível deve ser o seu receptor; o piso é onde você define que, abaixo de certo ponto, a sobrecarga é problema de outro e a solução é um filtro, não mais vinte decibéis.
settings-radio-filter-board = Placa de filtro
settings-radio-accessory-board-hermes-lite-2 = Placa de acessórios no conector J16 da Hermes-Lite 2 (ou nas saídas de coletor aberto de qualquer outra placa openHPSDR — inclusive Protocol 2). "N2ADR" escolhe um relé por faixa; "Código de banda Alex / Hermes" coloca o número da faixa nas saídas 1–4, que é o que a placa Alex de um ANAN, um Zeus SDR, um HiQSDR e o Quisk esperam. "Personalizado" abre uma tabela abaixo em que você declara as sete saídas por conta própria, faixa a faixa, na recepção e na transmissão — para uma chave de antena, o decodificador de faixa de um amplificador ou qualquer outra coisa que nenhuma das duas predefinições cubra; qualquer uma das predefinições pode ser despejada nela como ponto de partida. Deixe isto em "Nenhum" a menos que algo esteja realmente instalado: esses sete pinos são saídas de coletor aberto de uso geral, e os operadores também os ligam ao PTT de amplificador, a relés de antena e à comutação de transverters. Acioná-los a partir dos dados de faixa começaria a operar o que estiver conectado. Vale em Aplicar / reconectar.
settings-radio-io-board-rx-input = Entrada RX da placa IO
settings-radio-where-n2adr-hl2ioboard-takes-receive = De onde uma HL2IOBoard da N2ADR pega o sinal de recepção. A placa em si é encontrada automaticamente e não precisa de configuração; esta existe apenas para operadores que ligaram os próprios conectores SMA dela. Deixe em "Entrada própria do rádio" caso contrário — selecionar o J9 da placa IO sem nada conectado a ele deixa o receptor surdo. Vale em Aplicar / reconectar. Com qualquer uma das opções de J9, as entradas também aparecem no controle ANT e são lembradas por faixa.
settings-radio-power-amplifier = Amplificador de potência
settings-radio-use-hermes-lite-2-s = Usar o PA embutido da Hermes-Lite 2
settings-radio-default-what-you-want-unless = Ligado por padrão, e é o que você quer, a menos que um amplificador externo seja acionado pela saída de baixa potência RF1 da placa. Com ele desligado, o rádio ainda aciona — o relé T/R comuta e qualquer placa de acessórios acompanha —, mas o conector de antena não produz potência nenhuma, e o relé fica deliberadamente em recepção. Ignorado em placas que não são Hermes-Lite.
settings-radio-mirror-board-s-spectrum-about = Espelha o espectro da placa em torno da frequência sintonizada, na transmissão e na recepção. Ligado por padrão: uma Hermes-Lite 2 precisa disso. Desligue apenas se os sinais aparecerem do lado errado do dial e nada decodificar — o indício é uma cascata cheia de traços convincentes enquanto a SSB cai na banda lateral errada e o FT8 não devolve decodificação nenhuma.
settings-radio-frequency-correction = Correção de frequência
settings-radio-crystal-tcxo-error-ppm-applied = Erro do cristal/TCXO em ppm, aplicado a RX e TX. Vale imediatamente.
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = Buffer de transmissão
settings-radio-how-far-ahead-real-time = Quanto à frente do tempo real o áudio de transmissão é enviado à placa pela rede, antes de o sdroxide desacelerar para acompanhar. A própria placa não guarda buffer nenhum — isto apenas amplia a margem do próprio sdroxide. Aumente em um link WiFi ou VPN, onde o padrão baixo (adequado para uma conexão cabeada direta) não dá folga suficiente contra o jitter e o áudio transmitido ou o PTT travam; um valor maior custa latência na transmissão. Tem efeito em APLICAR, que reconecta à placa.
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = Pré-distorção adaptativa: linearizar o transmissor a partir de uma amostra do que ele realmente emitiu, para umas vinte e poucas decibéis a menos de intermodulação na mesma potência. O receptor é o caminho de realimentação — a placa continua recebendo durante uma transmissão —, então um acoplador direcional e um atenuador precisam levar uma amostra da saída do amplificador a uma entrada que a chave T/R não retire na transmissão. Em uma Hermes-Lite 2, isso significa o conector PureSignal da placa IO, com a entrada de recepção acima ajustada para combinar; em uma placa sem ele, a porta RX de um ANAN, ou um relé externo que liga o acoplador durante toda a transmissão. É este receptor que é lido — uma placa com um segundo ADC não pode usar o RX2 como tomada de realimentação, por mais que o firmware dela o roteie. Sem nada acoplado, o laço nunca trava e o transmissor fica exatamente como estaria. Vale em Aplicar / reconectar, e só no rádio que é dono do transmissor (DDC1).
settings-radio-correct-transmitter-receiver-s-own = Corrigir o transmissor a partir da realimentação do próprio receptor
settings-radio-how-finely-amplifier-s-curve = Com que precisão a curva do amplificador é modelada. Mais passos seguem um joelho mais agudo e levam mais tempo para preencher; 32 é um bom começo.
settings-radio-how-fast-correction-follows-what = Com que rapidez a correção acompanha o que o acoplador informa. Lento é o certo — isto faz a média da curva de um amplificador, que não se move, a partir de um caminho de realimentação que tem ruído.
settings-radio-hold = Manter
settings-radio-stop-adapting-keep-correction-where = Para de adaptar e mantém a correção onde está — para medir, e para um operador satisfeito com o que ela aprendeu.
settings-radio-manual-ip-overrides-discovery-press = Um IP manual tem precedência sobre a descoberta. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-open-collector-outputs-band = Saídas de coletor aberto por faixa
settings-radio-one-control-word-per-band = Uma palavra de controle por faixa, como a documentação do seu hardware a descreve: o bit 0 é a saída 1, o bit 6 é a saída 7. RX é afirmado durante a recepção nessa faixa, e TX enquanto o transmissor está acionado — dê o mesmo valor aos dois para um filtro, e valores diferentes para algo que pertença a um só lado da comutação (a linha de tecla de um amplificador, o desvio de um pré-amplificador de recepção). Faixas deixadas em 00 não afirmam nada. Vale em Aplicar / reconectar.
settings-radio-alex-band-code = CÓDIGO DE BANDA ALEX
settings-radio-replace-table-what-preset-would = Substitui a tabela pelo que esta predefinição enviaria em cada faixa, como ponto de partida para editar. Nada mais na configuração muda.
settings-radio-clear = LIMPAR
settings-radio-every-output-off-every-band = Cada saída desligada em cada faixa, que é o que "Nenhum" faz.
settings-radio-band = Faixa
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = Saídas afirmadas
settings-radio-other = Outras
settings-radio-everywhere-outside-amateur-bands-short = Em todo lugar fora das faixas de amador — escuta em ondas curtas, e qualquer coisa em que o dial de um transverter caia e que nenhuma faixa cubra.
settings-radio-dongle = Dongle
settings-radio-rescan = Buscar novamente
settings-radio-re-list-usb-bus-no = Relista o barramento USB. Nenhum dispositivo é aberto, então é seguro pressionar durante a recepção.
settings-radio-no-dongles-press-rescan = nenhum dongle — pressione Buscar novamente
settings-radio-first-one-found = — o primeiro encontrado —
settings-radio-rtl2832u-s-resampler-reaches-225 = O reamostrador do RTL2832U alcança 225–300 kHz e 900 kHz–3,2 MHz, nada entre eles. Tem efeito em Aplicar.
settings-radio-often-drops-samples = {"  "}(muitas vezes descarta amostras)
settings-radio-agc = AGC
settings-radio-manual-setting-measurement-weak-signal = Manual é a configuração para medição e modos digitais de sinal fraco. O sintonizador e o demodulador têm laços automáticos independentes.
settings-radio-tuner-gain = Ganho do sintonizador
settings-radio-applies-immediately-no-reconnect-tuner = Vale imediatamente — sem reconectar. O sintonizador tem 29 passos discretos, então o valor se fixa no mais próximo que ele consegue produzir. Ignorado enquanto o AGC do sintonizador está rodando.
settings-radio-crystal-error-parts-per-million = Erro do cristal em partes por milhão. Rode com RUST_LOG=sdroxide_rtlsdr=debug e o log imprime o erro de clock medido após cerca de 20 segundos — esse é o número a informar. Vale imediatamente.
settings-radio-hf-reception = Recepção de HF
settings-radio-tuner-itself-starts-24-mhz =
    O próprio sintonizador começa em 24 MHz. Um RTL-SDR Blog V4 faz upconversão abaixo disso por hardware; outros dongles alcançam HF apenas amostrando o ADC diretamente, pela porta HF do V3. Automático entrega tudo abaixo de 24 MHz ao ADC e tudo acima dele ao sintonizador.

    Direct sampling covers every HF band, 17 m and 15 m included — they arrive in the ADC's second Nyquist zone, the right way up. Nothing filters the ADC's input, though, so whatever is at 28.8 MHz minus the dial comes with them: 10.7 MHz under 17 m, 7.726 MHz under 15 m.

    Switching modes briefly interrupts the stream.
settings-radio-removes-dongle-s-own-dc =
    Remove o pico de CC próprio do dongle do centro da faixa, e a imagem espelhada que cada sinal deixa refletida em torno dele, medindo o desequilíbrio nas próprias amostras — sem calibração, e vale imediatamente. O sintonizador não tem modo de deslocamento de oscilador, então esta é a única forma de limpar o centro.

    An AM carrier tuned dead on the dial is at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-remove-centre-spike-mirror-image = Remover o pico central e a imagem espelhada
settings-radio-bias-tee = Bias tee
settings-radio-feed-4-5-v-dc = Alimentar ~4,5 V CC pelo coaxial
settings-radio-bias-tee-never-connect-transceiver = O bias tee está LIGADO. Nunca conecte um transceptor, uma antena aterrada ou um pré-amplificador alimentado de outra fonte enquanto isto estiver ativo — a CC vai direto pela linha de alimentação.
settings-radio-receive-only-dongle-sample-rate = Somente recepção. O dongle e a taxa de amostragem têm efeito em Aplicar; o restante vale à medida que você altera.
settings-radio-server-address = Endereço do servidor
settings-radio-where-rtl-tcp-listening-address =
    Onde o rtl_tcp está escutando: um endereço, ou um endereço e porta. A porta padrão é 1234, que é o padrão do próprio rtl_tcp.

    On the far end, start it as `rtl_tcp -a 0.0.0.0` — bound to 127.0.0.1, which is what it does with no -a, it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g = host ou host:porta, ex.: raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    Solicitado ao servidor, com os mesmos limites de reamostrador de um dongle local — é o mesmo silício na outra ponta.

    The figure beside each rate is what it costs on the link: the samples are sent uncompressed, and a rate the network cannot carry makes rtl_tcp drop the connection rather than degrade. Takes effect on Apply.
settings-radio-wired-link = {"  "}(link cabeado)
settings-radio-runs-server-s-dongle-manual = Roda no dongle do servidor. Manual é a configuração para medição e modos digitais de sinal fraco.
settings-radio-applies-immediately-no-reconnect-sent = Vale imediatamente — sem reconectar. Enviado em décimos de dB e ajustado pelo servidor a um passo que o sintonizador dele tenha; o protocolo não tem respostas, então o valor em que ele se fixou não pode ser lido de volta, e este controle continua mostrando o que foi pedido. Ignorado enquanto o AGC do sintonizador está rodando.
settings-radio-crystal-error-server-s-dongle =
    Erro do cristal do dongle do *servidor*, em partes por milhão — uma propriedade daquele hardware, então é definido aqui e não nos dongles desta máquina. Vale imediatamente.

    The measured clock error the USB interface prints is not available here: over a network what that measurement sees is the buffering, not the crystal, and it is wrong by thousands of ppm. Calibrate the dongle on USB once and carry the number over, or tune a broadcast station of known frequency and adjust until it sits on the dial.
settings-radio-tuner-starts-24-mhz-below =
    O sintonizador começa em 24 MHz; abaixo disso a outra ponta precisa de ajuda. Um Blog V4 faz upconversão por conta própria e o Automático não mexe nele — que é a única coisa que ele pode fazer, já que o protocolo informa o chip do sintonizador e nada além disso, e um V4 é indistinguível de um R828D comum pelo fio.

    On a V3 or any other dongle, Automatic switches the server to direct sampling below the tuner's own 24 MHz floor — which covers every HF band, 17 m and 15 m arriving in the ADC's second Nyquist zone with 28.8 MHz minus the dial folded on top. Choose Direct sampling explicitly for a plain R828D that hears nothing on HF. Switching briefly interrupts the stream.
settings-radio-removes-dongle-s-dc-spike =
    Remove aqui o pico de CC do dongle e a imagem espelhada, a partir das amostras à medida que chegam — são artefatos do hardware, então viajam pela rede junto com todo o resto. Vale imediatamente.

    An AM carrier tuned dead on the dial sits at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-powers-preamp-far-end-s = Alimenta um pré-amplificador pelo dongle da outra ponta. Servidores mais antigos não implementam o comando e o ignoram em silêncio — o protocolo não tem como dizer não —, então um bias tee que não liga não é necessariamente culpa desta ponta.
settings-radio-feed-4-5-v-dc-2 = Alimentar ~4,5 V CC pelo coaxial remoto
settings-radio-bias-tee-hardware-somewhere-else = O bias tee está LIGADO, em hardware que fica em outro lugar. O que estiver na outra ponta dessa linha de alimentação — um transceptor, uma antena aterrada, um pré-amplificador já alimentado — não está na sua frente para conferir.
settings-radio-sdrplay-server-rsp-tcp = Servidor SDRplay (rsp_tcp)
settings-radio-sdrplay-server-greets-exactly-like = Um servidor SDRplay se apresenta exatamente como um dongle, então estes campos são sempre mostrados, e não quando um é detectado. Com um servidor rtl_tcp comum eles são simplesmente ignorados — o protocolo não tem respostas e descarta comandos que não conhece. A aba Dispositivo identifica o servidor como rsp_tcp quando ele se identifica.
settings-radio-input = Entrada A
settings-radio-input-b = Entrada B
settings-radio-hi-z = Hi-Z
settings-radio-lna-state = Estado do LNA
settings-radio-step-index-not-db-figure = Um índice de passo, não um valor em dB: o quanto cada passo vale depende do modelo do RSP e da faixa. 0 é o máximo de ganho.
settings-radio-if-gain-reduction = Redução de ganho de F.I.
settings-radio-reduction-so-more-less-signal = Uma redução, então mais significa menos sinal. Só é obedecida com o AGC do RSP desligado.
settings-radio-enable = Ativar
settings-radio-level-rsp-s-agc-aims = O nível que o AGC do RSP procura manter.
settings-radio-notches = Notches
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = Saída de referência
settings-radio-t-24-mhz-clock-out = Saída de clock de 24 MHz
settings-radio-rsp2-rspduo-only = Somente RSP2 e RSPduo.
settings-radio-receive-only-address-sample-rate = Somente recepção. O endereço e a taxa de amostragem têm efeito em Aplicar; o restante vale à medida que você altera. Uma conexão caída é tentada de novo sozinha, então um servidor reiniciado volta sem que nada aqui precise ser tocado.
settings-radio-where-spyserver-listening-address-address =
    Onde o spyserver está escutando: um endereço, ou um endereço e porta. A porta padrão é 5555, que é o padrão do próprio spyserver.

    On the far end, check that its config file binds an address other machines can reach — bound to 127.0.0.1 it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-2 = host ou host:porta, ex.: raspberrypi.local:5555
settings-radio-i-q-bandwidth = Largura de banda I/Q
settings-radio-how-wide-slice-band-arrives =
    Quão larga é a fatia da faixa que chega como I/Q — e, portanto, quanto do link isto usa. Esta é a janela *demodulada*: ela segue o dial, e tudo que for mais largo que ela é a FFT do servidor na faixa acima do panadaptador.

    Automatic aims at about 96 kHz, which carries every mode here including wide FM and still fits down a cellular uplink. Takes effect on Apply.
settings-radio-which-stage-server-s-own =
    Qual estágio da escada de taxas própria do servidor pedir. Cada receptor tem uma escada diferente — é a taxa máxima dele dividida pela metade, estágio a estágio —, então isto é guardado como o estágio e não como um valor em hertz, e a mesma configuração ainda faz sentido apontada para outro servidor.

    Automatic aims at about 1 Msps. Press Test connection to see what the stages come to on this server. Takes effect on Apply.
settings-radio-automatic = Automático
settings-radio-sample-format = Formato de amostra
settings-radio-bits-per-component-wire-so =
    Bits por componente no fio, e portanto o que isto custa no link: 16 bits é o dobro de 8 bits, e ponto flutuante de 32 bits é quatro vezes isso sem mais informação do que o ADC do receptor tinha.

    8-bit is what makes a remote receiver work over a domestic uplink and is right for almost everything. A server may override this — some are configured to insist on one format — and says so in the log when it does. Takes effect on Apply.
settings-radio-gain = Ganho
settings-radio-server-s-gain-stage-index =
    O estágio de ganho do servidor, como índice — não como um número de decibéis. O que cada índice significa pertence ao receptor da outra ponta e muda com a faixa, e o protocolo nunca diz, então nada aqui pode convertê-lo em dB sem inventar um valor.

    The real range is the server's and is only known once connected; an index past it is clamped. Applies immediately — no reconnect.
settings-radio-index = índice{" "}
settings-radio-digital-gain = Ganho digital
settings-radio-how-far-server-scales-samples =
    Quanto o servidor amplia as amostras antes de quantizá-las para o fio. Automático observa o que de fato chega e mantém o pico na metade da escala total, que é a única coisa que pode estar certa tanto em uma faixa vazia quanto em uma lotada — as duas estão a trinta decibéis de distância e nenhum valor fixo serve às duas.

    It matters only at 8 bits, where a signal far below full scale loses its lower bits to the quantiser and one above it is clipped flat. Wider formats have room to spare and get the figure every other client computes, from the gain index and the decimation stage. Applies immediately.
settings-radio-full-band-strip = Faixa de banda completa
settings-radio-server-s-own-fft-whole =
    A FFT própria do servidor da faixa inteira, desenhada na faixa acima do panadaptador. Nesta interface é a única visão de banda que existe — o próprio panadaptador tem só a largura do I/Q que está sendo recebido.

    Switching it off leaves a receiver with no way to see anything it is not already tuned to. Applies immediately, with a short break in the audio while the server changes what it is sending.
settings-radio-ask-server-low-rate-fft =
    Pedir ao servidor uma FFT de baixa taxa da faixa inteira, além do I/Q, e desenhá-la na faixa acima do panadaptador.

    It costs almost nothing — a couple of kilobytes a frame, a dozen or so times a second — and it shows the whole receiver rather than the slice being demodulated. Worth switching off only on a link where every byte counts. Applies immediately, with a short break in the audio.
settings-radio-show = Mostrar
settings-radio-whole-band = banda inteira
settings-radio-how-much-receiver-strip-covers = Quanto do receptor a faixa cobre. A banda inteira é a visão mais larga que existe; estreitá-la coloca o mesmo número de raias em menos espectro, que é um detalhe mais fino sobre um trecho menor.
settings-radio-strip-db-window = Janela de dB da faixa
settings-radio-server-quantises-fft-into-window =
    O servidor quantiza a FFT dele nesta janela antes de enviá-la, um byte por raia — então estes valores decidem com que precisão a faixa é medida, não só como ela é desenhada. O piso e o teto com que a faixa é *exibida* são definidos pelo ajuste automático de nível do próprio motor e são outra coisa.

    The default 150 dB is the whole protocol range and needs no attention. Narrowing it around the noise floor buys resolution on a receiver whose signals all sit in a small part of the scale. Applies immediately.
settings-radio-top = topo{" "}
settings-radio-range-2 = faixa{" "}
settings-radio-i-q-correction = Correção I/Q
settings-radio-remove-dc-spike-mirror-image = Remove o pico de CC e a imagem espelhada no DSP, deste lado. Se o receptor da outra ponta precisa disto depende do que ele é — um Airspy HF+ não precisa, um RTL-SDR precisa — e o protocolo não diz com qual está falando. Vale imediatamente.
settings-radio-enabled = Ativado
settings-radio-test-connection = Testar conexão
settings-radio-connect-read-what-server-says = Conecta, lê o que o servidor diz sobre o receptor dele e desconecta de novo — sem iniciar um fluxo, então é seguro pressionar em um servidor que outra pessoa está usando.
settings-radio-receive-only-panadapter-narrow-i = Somente recepção. O panadaptador é a janela I/Q estreita, que segue o dial; a visão de banda é a FFT do servidor na faixa acima dele. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-receive-only-wideband-i-q = Somente recepção. I/Q de banda larga, igual a qualquer SDR local. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-receiver-address = Endereço do receptor
settings-radio-where-receiver-serves-web-page =
    Onde o receptor serve a página web dele: um endereço, ou um endereço e porta. A porta padrão é 8073, que é a do próprio KiwiSDR.

    Note that a receiver reached through the project's proxy (something.proxy.kiwisdr.com — nearly half the public ones) answers on port 80 instead, so give it explicitly. Picking a receiver from "Public SDRs" fills this in correctly.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-3 = host ou host:porta, ex.: kiwi.example.org:8073
settings-radio-password = Senha
settings-radio-receiver-s-user-password-where =
    A senha de *usuário* do receptor, quando o operador dele definiu uma. Em branco em quase todo receptor público. Não é a senha de administrador.

    Takes effect on Apply.
settings-radio-announce = Anunciar como
settings-radio-name-connection-shows-up-under =
    O nome sob o qual esta conexão aparece, que o dono do receptor e todos os outros que o escutam podem ver.

    Left blank it is your station callsign, which is the network's own convention — some operators do block clients that will not identify. Takes effect on Apply.
settings-radio-blank-station-callsign = em branco = indicativo da estação
settings-radio-band-view = Visão de banda
settings-radio-ask-receiver-own-waterfall-well =
    Pedir ao receptor a cascata dele, além do I/Q, e desenhá-la na faixa acima do panadaptador.

    Worth having: the I/Q is only about 12 kHz wide, so without this there is nothing to tune *by*. It is a second connection carrying roughly 20 kB/s at full speed, against the I/Q's 44 kB/s.

    Takes effect on Apply.
settings-radio-show-receiver-s-0-30 = mostrar a cascata de 0-30 MHz do receptor
settings-radio-band-view-speed = Velocidade da visão de banda
settings-radio-how-often-receiver-sends-waterfall =
    Com que frequência o receptor envia uma linha da cascata, de 1 (mais lento) a 4. É o único ajuste aqui que muda o que o link custa durante a execução, então reduza-o em uma conexão medida.

    Applies immediately.
settings-radio-band-view-span = Largura da visão de banda
settings-radio-how-much-band-receiver-s =
    Quanta banda a cascata do receptor cobre. A faixa completa de 0-30 MHz fica à esquerda do controle deslizante, e cai pela metade a cada passo para a direita.

    The waterfall is always 1024 bins wide however wide the window is, so this is really a resolution control: the whole band is 29 kHz to a bin, which is a band *map* rather than a picture of anything, while a few hundred kilohertz shows individual stations in a broadcast band. Zoomed in, the window follows your dial.

    The whole band is the default and is what makes the strip a thing to tune by; the panadapter below it is the ~12 kHz of I/Q. Applies immediately.
settings-radio-receiver-agc = AGC do receptor
settings-radio-receiver-s-agc-which-far =
    O AGC *do receptor*, que fica do outro lado do link e antes do I/Q — então ele age antes de qualquer coisa que o sdroxide faça.

    On by default, unlike every local SDR here, because the measured alternative was worse: on a live receiver the manual gain was not monotonic across its range and its top end clipped the I/Q at full scale, while the AGC held about -24 dBFS with 7 dB of headroom.

    It does mean the sample amplitude is not a signal level, which is why the S-meter is read from the receiver's own figure instead. Applies immediately.
settings-radio-let-receiver-ride-own-gain = deixar o receptor controlar o próprio ganho
settings-radio-manual-gain = Ganho manual
settings-radio-fixed-gain-when-receiver-s = Ganho fixo quando o AGC do receptor está desligado, em uma escala própria de 0-90. Não são decibéis de nada declarado — o protocolo o chama de manGain e não diz mais nada, e medimos que ele não é monotônico, então trate-o como um controle para achar um bom ponto, não como calibração. Vale imediatamente.
settings-radio-read-receiver-s-status-page = Lê a página de status do receptor: o que ele é, o que cobre, quantos canais estão livres e se o operador permite conexões de aplicativos que não sejam um navegador. Não ocupa nenhum dos canais dele.
settings-radio-host-port-e-g-127 = host:porta, ex.: 127.0.0.1:50001
settings-radio-iq-sample-rate = Taxa de amostragem IQ
settings-radio-receiver = Receptor
settings-radio-rig-two-receivers-sunsdr2dx-can = Um equipamento com dois receptores (SunSDR2DX) pode atender duas abas de rádio em uma só conexão — rode este rádio no RX1 e outro no RX2. O transmissor pertence ao rádio do RX1.
settings-radio-stream-delay = Atraso do fluxo
settings-radio-how-long-rig-s-iq = Quanto tempo o IQ do equipamento leva para chegar em um novo centro depois que o sdroxide o move. É o pipeline de DSP do próprio equipamento, não a rede — um servidor nesta máquina também tem um —, e, enquanto o panadaptador é arrastado com zoom totalmente afastado, errar por esse tanto desloca as linhas mais recentes da cascata para o lado do histórico pelo erro vezes a velocidade do arraste. Alto demais as desloca exatamente tanto quanto baixo demais, no sentido contrário. Meça com `cargo run --release -p sdroxide-tci --example retune_latency`; o padrão é um SunSDR2DX no ExpertSDR3 em 192 kHz.
settings-radio-wideband-iq-receive-audio-transmit = I/Q de banda larga na recepção, áudio na transmissão. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-radio-address = Endereço do rádio
settings-radio-address-shown-radio-under-set = O endereço mostrado no rádio em SET > Network. O Network Control precisa estar ativado ali, e o rádio precisa ter um usuário e senha de rede definidos.
settings-radio-host-ip-e-g-192 = host ou IP, ex.: 192.168.1.50
settings-radio-control-port = Porta de controle
settings-radio-t-50001-unless-has-been-changed = 50001, a menos que tenha sido alterada no rádio.
settings-radio-network-user = Usuário de rede
settings-radio-stored-clear-radio-json-protocol = Armazenado em texto puro no radio.json. O protocolo o ofusca de forma reversível no fio, então nada aqui o tornaria um segredo.
settings-radio-receive = Receber de
settings-radio-audio-sample-rate = Taxa de amostragem de áudio
settings-radio-t-12-khz-if-needs-48000 = Um F.I. de 12 kHz exige o fluxo de 48000 Hz — nessa taxa é usado o áudio demodulado do rádio.
settings-radio-if-spectrum = Espectro de F.I.
settings-radio-which-way-round-radio-s = Em que sentido roda o F.I. de 12 kHz do rádio. "Automático" usa o que o modelo é conhecido por fazer — espelhado em um IC-7760, normal em todos os outros Icom. Ajuste à mão se a SSB sair na banda lateral errada: o indício é ter de selecionar USB onde a faixa roda LSB, enquanto a indicação de modo do próprio rádio concorda com o sdroxide o tempo todo.
settings-radio-normal = Normal
settings-radio-mirrored = Espelhado
settings-radio-displayed-bandwidth = Largura de banda exibida
settings-radio-width-audio-band-panadapter-cat = Largura do panadaptador da faixa de áudio, como para um equipamento CAT. Usado nos modos digitais e quando o scope do rádio está desligado; caso contrário, o panadaptador é o scope, e a Largura do scope define a largura dele.
settings-radio-how-cw-panel-s-keyer-2 = Como o manipulador do painel CW transmite. "Manipulador do rádio" coloca o rádio em CW e lhe entrega o texto a enviar com o manipulador próprio dele (CI-V 0x17). "Placa de som" envia o sidetone manipulado pela LAN como áudio (MCW), um tom em dial + tom — e, como um equipamento em CW ignoraria a entrada do modulador, selecionar CW então mantém o rádio em USB comum, o mesmo modo em que os modos digitais trafegam.
settings-radio-how-much-audio-radio-holds = Quanto áudio o rádio guarda antes de modular. Mais sobrevive a uma rede pior, ao custo de latência de transmissão.
settings-radio-streams-radio-s-own-475 = Transmite a varredura própria de 475 raias do rádio. Em AF é o panadaptador — o áudio que o rádio envia é o que passou pelo filtro dele, não uma imagem da banda — e na F.I. de 12 kHz é a cascata da banda inteira acima de um. De qualquer forma, é a imagem do rádio, não o DSP do sdroxide: não há I/Q a partir do qual calcular uma.
settings-radio-how-wide-sweep-only-wide = Quão larga é a varredura. Esta é a única visão larga que um Icom tem, e o rádio mantém a largura escolhida por último na própria tela — muitas vezes alguns kHz, por isso a faixa pode aparecer não mais larga que o panadaptador abaixo dela. Definir uma largura aqui também coloca o scope em modo de centro, para que siga o dial. Isso altera também a tela do próprio rádio; "Como ajustado no rádio" a deixa intacta.
settings-radio-switch-modulation-input-lan = Trocar a entrada de modulação para LAN
settings-radio-transmit-audio-only-heard-when = O áudio de transmissão só é ouvido quando a entrada MOD do rádio está em LAN. O sdroxide consegue escrever isso em um modelo cuja numeração de menu ele conhece; em qualquer outro, informa e deixa o menu como está. É um empréstimo: o que o rádio tinha ali é lido primeiro e devolvido quando a sessão termina, então um equipamento usado sozinho depois ainda ouve o próprio microfone.
settings-radio-copy-diagnostic-report = Copiar relatório de diagnóstico
settings-radio-control-audio-radio-s-own = Controle, áudio e o scope próprio do rádio por uma única conexão de rede — sem cabo serial e sem placa de som. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-testing = Testando…
settings-radio-was-only-check-press-apply = Foi só uma verificação — pressione Aplicar / reconectar abaixo para começar a usar este rádio.
settings-radio-radios = Rádios
settings-radio-asks-network-iio-devices-also = Pergunta à rede pelos dispositivos IIO, e também tenta 192.168.2.1 diretamente — um Pluto na ponta de um cabo USB muitas vezes não é alcançável por multicast, embora o endereço funcione.
settings-radio-no-radios-press-discover = nenhum rádio — pressione Descobrir
settings-radio-address = Endereço
settings-radio-overrides-selection-above-usb-cable = Tem precedência sobre a seleção acima. O cabo USB apresenta o Pluto como um adaptador de rede, não como uma porta serial, então isto é um endereço IP mesmo quando o rádio está na sua mesa.
settings-radio-pluto-revision-c-pluto-unlocked = Um Pluto+ ou um Pluto de revisão C desbloqueado para 2R2T pode atender duas abas de rádio em uma só caixa — este rádio no RX1 e outro no RX2, cada um na sua antena. As duas cadeias compartilham o mesmo oscilador, então sintonizar qualquer um move os dois; o que o RX2 traz é uma segunda antena no mesmo espectro (diversidade), não uma segunda faixa. O transmissor pertence ao rádio do RX1. Um Pluto 1R1T de fábrica recusa o RX2 quando conecta.
settings-radio-width-spectrum-sdroxide-receives-ad9361 = Largura do espectro que o sdroxide recebe. O AD9361 chega a 61,44 Msps; o link de rede USB não, e é a isso que esta lista está ajustada. As taxas mais baixas exigem uma configuração de filtro carregada no AD9361, o que o sdroxide não faz — um Pluto de fábrica roda nelas a cerca de 2,084 Msps e informa isso ao conectar. Tem efeito em Aplicar.
settings-radio-more-than-usb-2-will = {"  "}(mais do que o USB 2 suporta)
settings-radio-stock-pluto-runs-2-084 = {"  "}(um Pluto de fábrica roda a 2,084)
settings-radio-analog-filter = Filtro analógico
settings-radio-ad9361-s-baseband-filter-leave = O filtro de banda base do AD9361. Deixe em 0 para automático, que o abre até nove décimos da taxa de amostragem — largo de propósito, porque o receptor estaciona o oscilador um quarto de largura de faixa fora do dial para manter os sinais longe do pico de CC, e um filtro estreito cortaria exatamente a parte para a qual os moveu. Tem efeito em Aplicar.
settings-radio-ad9361-has-four-modes-not = O AD9361 tem quatro modos, não um interruptor liga/desliga. Ataque lento serve a SSB e CW; ataque rápido serve sinais em rajadas; manual é a configuração para medição e modos digitais de sinal fraco. Vale imediatamente.
settings-radio-rx-gain = Ganho RX
settings-radio-applies-immediately-no-reconnect-ignored = Vale imediatamente — sem reconectar. Ignorado a menos que o AGC esteja em manual, o que é o comportamento do próprio AD9361, não do sdroxide.
settings-radio-tx-gain = Ganho TX
settings-radio-negative-because-ad9361-expresses-transmit = Negativo porque o AD9361 expressa o nível de transmissão como atenuação: 0 dB é a saída total. Vale imediatamente. O transmissor é ajustado para o mínimo antes de este valor ser aplicado na conexão, para que nada que o programa anterior deixou possa estar ativo.
settings-radio-reference-error-parts-per-million = Erro de referência em partes por milhão. Rode com RUST_LOG=sdroxide_pluto=debug e o log imprime o erro de clock medido após cerca de 20 segundos — esse é o número a informar. Aplicado pelo sdroxide, não gravado no ajuste fino persistente do próprio rádio. Vale imediatamente.
settings-radio-full-duplex = Duplex total
settings-radio-keep-receiving-while-transmitting = Continuar recebendo durante a transmissão
settings-radio-leave-off-pluto-reached-over =
    Deixe isto desligado em um Pluto acessado pelo cabo USB. Esse link não aguenta um megassample por segundo nos dois sentidos ao mesmo tempo, e uma transmissão que faz faltar dados ao buffer de transmissão sai picotada — então, por padrão, a recepção para durante a transmissão.

    Turn it on for a board on real Ethernet (a LibreSDR, a Pluto on a gigabit adapter), where there is room for both: you then hear the receiver through your own transmission, which is how a QO-100 station listens to its own downlink.

    An over needs twice the sample rate's worth of link — 2.5 Msps is 10 MB/s each way — so lower the rate if the log starts saying the link is not carrying it. The panadapter still shows the transmitted signal during an over; it is the audio that keeps coming.

    Takes effect on Apply.
settings-radio-duplex = Duplex
settings-radio-whether-ad9361-runs-both-directions =
    Se o AD9361 roda as duas direções ao mesmo tempo (FDD, que é como um Pluto inicia e em que o sdroxide sempre o deixou) ou uma de cada vez (TDD).

    Leave this on FDD unless you want the PTT pins below — TDD is what those key from, and it rules out Full duplex above. Takes effect on Apply.
settings-radio-ptt-pins = Pinos de PTT
settings-radio-pluto-s-four-gpo-test =
    Os quatro pontos de teste GPO do Pluto podem acionar sozinhos um amplificador de potência externo, um LNA ou uma chave de transmissão/recepção. Escolha um par e um pino fica em nível alto o tempo todo em que o rádio recebe, e o outro o tempo todo em que transmite — sem software do computador no meio e sem linha serial de PTT para fiar.

    This puts the radio in TDD whatever the Duplex row says, because the pins follow the AD9361's enable lines and in FDD both of those are asserted the entire session. Full duplex goes off with it, so leave both at their defaults if you work satellites and need to hear your own downlink.

    Analog Devices' own note puts an external LNA on GPO0/GPO1, so use GPO2/GPO3 if your board is wired that way. The pins are about 1.3 V at a few milliamps: drive a transistor or an opto-isolator with them, never a relay coil.

    Takes effect on Apply.
settings-radio-buffer-size = Tamanho do buffer
settings-radio-how-much-device-holds-before =
    Quanto o dispositivo guarda antes de cada transferência, em amostras complexas. O padrão de 32768 são cerca de 16 ms a 2 Msps: longo o bastante para que o ida-e-volta de cada transferência não seja o gargalo, curto o bastante para que um reajuste não fique visivelmente atrasado.

    Halve it if the log reports the receive socket being replaced. A marginal link — a Pluto reached over its USB cable at a high sample rate is the usual one — stalls part-way through a transfer, and a smaller transfer is both less likely to be caught mid-flight and quicker to make good afterwards. Raise it to trade retune latency for fewer round trips.

    Takes effect on Apply.
settings-radio-samples = {" "}amostras
settings-radio-rx-tx-port = Porta RX / TX
settings-radio-ad9361-s-rf-port-select = O rf_port_select do AD9361. Um Pluto de fábrica liga um de cada, então deixe estes vazios, a menos que tenha uma placa que não faça isso. Tem efeito em Aplicar.
settings-radio-opens-radio-reads-what-says = Abre o rádio, lê o que ele diz sobre si mesmo e informa a faixa de sintonia que esta placa específica tem. Não inicia um fluxo.
settings-radio-wideband-iq-receive-transmit-half = I/Q de banda larga na recepção e na transmissão. Meio duplex — a recepção para durante a transmissão, porque o link de rede USB não aguenta os dois ao mesmo tempo. Não precisa de SoapySDR nem de libiio.
settings-radio-flexradio-announces-itself-local-network = Um FlexRadio se anuncia na rede local cerca de uma vez por segundo. Um rádio acessado por roteador ou VPN nunca transmite para você — informe o endereço dele abaixo.
settings-radio-another-gui-client-has-radio = Outro cliente gráfico tem este rádio e o multiFLEX está desativado. Desconecte esse cliente, ou ative o multiFLEX no rádio.
settings-radio-overrides-selection-above-use-radio = Tem precedência sobre a seleção acima. Use isto para um rádio em outra sub-rede, atrás de uma VPN ou em uma porta fora do padrão.
settings-radio-width-spectrum-sdroxide-receives-192 = Largura do espectro que o sdroxide recebe. 192 kHz é o máximo do rádio para um fluxo DAX IQ, e portanto a maior largura que esta interface pode mostrar.
settings-radio-t-192-khz-widest-dax-iq = 192 kHz é o fluxo DAX IQ mais largo que um FLEX envia, e o rádio prende a largura do panadaptador a ele — então essa é a faixa inteira, e nenhuma configuração aqui a amplia.
settings-radio-dax-iq-channel = Canal DAX IQ
settings-radio-radio-has-four-change-only = O rádio tem quatro. Altere isto só se outra coisa na rede já estiver usando o canal 1 — o rádio recusa um canal duplicado.
settings-radio-station-name = Nome da estação
settings-radio-shown-against-session-radio-s = Mostrado junto a esta sessão na lista de clientes do rádio. O rádio também reconhece um cliente por ele, então renomear faz o rádio tratar o sdroxide como um novo.
settings-radio-gui-client-id = ID do cliente gráfico
settings-radio-identity-radio-restores-session-s = A identidade para a qual o rádio restaura os slices e panadaptadores desta sessão. Vazio, ela é derivada do nome da estação — estável, mas igual em todo sdroxide que manteve o nome padrão, e um rádio resolve uma duplicata derrubando o cliente anterior. O sdroxide percebe isso no fio e recorre a uma identidade de uma só sessão, o que custa a restauração; informe aqui um UUID próprio para mantê-la.
settings-radio-optional-e-g-uuid-your = opcional, ex.: um UUID próprio
settings-radio-mirror-radio-s-i-q =
    Espelha o I/Q do rádio em torno do centro do panadaptador. Desligado por padrão, que é como um FLEX-6600 foi verificado.

    Try it if receive audio is unintelligible on USB *and* on LSB and nothing decodes, while the waterfall looks entirely convincing — that is what a mirrored stream looks like, and it is the one fault with no other symptom (issue #368). If it is not that, turning this on makes it obvious rather than subtle. Applies on Apply / reconnect.
settings-radio-network-mtu = MTU de rede
settings-radio-largest-datagram-radio-may-send = O maior datagrama que o rádio pode enviar. 1450 é o que o próprio SmartSDR pede. Reduza em um caminho com MTU menor — uma VPN ou um túnel — onde os fragmentos são descartados e nenhum espectro chega.
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = Verifica se o rádio responde, sem se registrar como cliente gráfico — então não perturba uma sessão do SmartSDR que já esteja rodando.
settings-radio-wideband-iq-receive-over-dax = I/Q de banda larga na recepção pelo DAX, áudio de transmissão que o rádio modula. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-not-yet-verified-against-real-2 = Ainda não verificado em hardware real. Se apresentar mau funcionamento, envie o relatório de diagnóstico — ele contém cada linha de protocolo trocada com o rádio.
settings-radio-devices-soapysdr-can-see = Dispositivos que o SoapySDR enxerga
settings-radio-waiting-machine-radio-attached-where = Aguardando a máquina à qual o rádio está conectado, onde os módulos estão instalados. O dispositivo aberto é o `device_args` do config.toml dessa máquina, ou o --device dela.
settings-radio-ask-every-installed-soapysdr-module = Pede a todo módulo SoapySDR instalado que faça uma varredura. Nada é aberto, então é seguro durante a recepção — mas pode levar um momento.
settings-radio-not-enumerated-yet-press-rescan = Ainda não enumerado — pressione Buscar novamente.
settings-radio-no-soapysdr-devices-found-check = Nenhum dispositivo SoapySDR encontrado. Verifique se o módulo do seu rádio está instalado e se você pode acessar o dispositivo.
settings-radio-not-radio = não é um rádio
settings-radio-sound-card-listed-above-if = Uma placa de som aparece acima como se fosse um SDR. Ela aceita qualquer frequência e a ignora, então abri-la mostra a entrada da placa de som em vez da faixa. O sdroxide não as escolhe automaticamente — mas uma linha device_args que nomeie uma delas ainda é obedecida.
settings-radio-which-one-opens-set-device = O que abre é definido por --device ou device_args no config.toml; sem nenhum dos dois, vence o primeiro rádio da lista.
settings-radio-no-device = nenhum dispositivo
settings-radio-rig-has-no-software-adjustable = Este equipamento não tem ganhos ajustáveis por software.
settings-radio-rx-gains = Ganhos RX
settings-radio-tx-gains = Ganhos TX
settings-radio-antennas = Antenas
settings-radio-remembered-next-start = Lembrado para o próximo início.
settings-radio-these-controls-appear-once-soapysdr = Estes controles aparecem assim que um dispositivo SoapySDR é aberto — são do próprio dispositivo, e é preciso perguntar a ele.
settings-radio-stream = Fluxo
settings-radio-follow-app-setting = Seguir a configuração do aplicativo
settings-radio-msps = ou, em Msps
settings-radio-baseband-filter = Filtro de banda base
settings-radio-let-driver-choose = Deixar o driver escolher
settings-radio-changing-either-reopens-radio = Alterar qualquer um dos dois reabre o rádio.
settings-radio-driver-publishes-no-settings-own = Este driver não publica configurações próprias.
settings-radio-applied-once-remembered-next-start = Aplicado de imediato e lembrado para o próximo início.
settings-radio-no-rx-888-found-press = Nenhum RX-888 encontrado — pressione Buscar novamente
settings-radio-adc-clock = Clock do ADC
settings-radio-t-129-6-msps-needs-superspeed = 129,6 Msps exige um link SuperSpeed e um computador rápido; 64,8 é o padrão seguro. Alterá-lo reabre o receptor automaticamente, o que leva um momento mas não exige reinício.
settings-radio-panadapter-width = Largura do panadaptador
settings-radio-hf-only = {"  "}· somente HF
settings-radio-usable-both-front-ends = Utilizável nos dois front ends.
settings-radio-below-vhf-crossover-ordinary-width = Abaixo da transição VHF esta é uma largura comum. Acima dela, o F.I. de 8 MHz do sintonizador não pode ser centralizado em uma janela tão larga, então o espectro ao vivo fica deslocado para um lado do panadaptador, com nada ao lado.
settings-radio-dial-above-crossover-now-so = O dial já está acima da transição, então esta largura mostra o F.I. de 8 MHz do sintonizador fora do centro, com espectro morto ao lado. Reduza a largura para preencher o panadaptador.
settings-radio-vga-gain = Ganho VGA
settings-radio-ad8370-variable-gain-amplifier-ahead = Amplificador de ganho variável AD8370 antes do ADC.
settings-radio-attenuator = Atenuador
settings-radio-pe4304-step-attenuator-0-5 = Atenuador por passos PE4304, em passos de 0,5 dB.
settings-radio-adc-range = Faixa do ADC
settings-radio-wide-2-25-vp-p = Larga (2,25 Vp-p)
settings-radio-selects-adc-s-wider-input = Seleciona a faixa de entrada mais larga do ADC: mais folga para sinais fortes de radiodifusão, menos contagens para os fracos. Desligado seleciona a faixa mais sensível de 1,5 Vp-p.
settings-radio-dither = Dither
settings-radio-adds-small-dither-signal-ahead = Adiciona um pequeno sinal de dither antes do ADC: custa um pouco de piso de ruído, mas compra faixa dinâmica livre de espúrios.
settings-radio-randomiser = Randomizador
settings-radio-adc-scrambles-output-so-digital = O ADC embaralha a saída para que o barramento digital pare de irradiar para o front end; o driver a desembaralha. Deixe ligado a menos que esteja depurando. Vale ao reconectar.
settings-radio-dc-hf-antenna-port = CC na porta de antena HF
settings-radio-powers-active-antenna-preamp-down = Alimenta uma antena ativa ou pré-amplificador pelo coaxial.
settings-radio-vhf-tuner-gain = Ganho do sintonizador VHF
settings-radio-r828d-rf-gain-used-above = Ganho de RF do R828D, usado acima da transição automática HF/VHF. 29 passos discretos; o mais próximo é usado.
settings-radio-auto = Automático
settings-radio-let-tuner-run-own-lna = Deixar o sintonizador rodar os próprios laços de LNA e misturador.
settings-radio-bias-tee-vhf = Bias tee (VHF)
settings-radio-dc-vhf-antenna-port = CC na porta de antena VHF
settings-radio-clock-trim = Ajuste do clock
settings-radio-corrects-reference-oscillator-applied-when = Corrige o oscilador de referência. Aplicado quando você solta o valor — reabrir a cada pixel de um arraste reiniciaria o receptor centenas de vezes.
settings-radio-receive-only-below-adc-s = Somente recepção. Abaixo do limite de Nyquist do ADC, a antena é amostrada diretamente e a sintonia é instantânea, porque não há conversor de descida de hardware — o fluxo completo do ADC é convertido para banda base no computador. Acima dele, o receptor passa automaticamente para o sintonizador R828D e para a SMA de VHF, então as duas antenas precisam estar conectadas. VHF exige um clock do ADC de 32,4 Msps ou mais para o F.I. do sintonizador caber, e com clocks abaixo de 48 Msps existe um intervalo entre as duas faixas que nada consegue alcançar. Todas as configurações aqui valem de imediato — não há botão Aplicar.
settings-radio-device = Dispositivo
settings-radio-no-devices-press-rescan = nenhum dispositivo — pressione Buscar novamente
settings-radio-six-rates-are-six-different =
    As seis taxas são seis imagens diferentes de FPGA, não um registrador — por isso nada no protocolo da ELAD seleciona entre elas, e o que esta configuração significa depende do modelo.

    FDM-S1 / FDM-S2: a command. Their FPGA is loaded by the computer at every power-up, and sdroxide runs ELAD's own elad-firmware loader to put this rate's image in. Without that loader installed a sampler sends no samples at all, and sdroxide says so on screen.

    FDM-DUO: not a command. The radio boots its own FPGA and arrives at whatever rate it powered up in or FDM-SW2 last left it in (192 kHz on a fresh one), and this only says how the stream is READ. Get it wrong and the panadapter is simply the wrong width, with every frequency inside it scaled to match.

    The stream's real rate is measured a couple of seconds after it starts, and a mismatch is reported on screen. Takes effect on Apply.
settings-radio-fdm-s-fpga-image-loaded = FDM-S: a imagem da FPGA carregada — FDM-DUO: como o fluxo é lido
settings-radio-receiver-s-input-pad-one = O atenuador de entrada do receptor — um passo, para dentro ou para fora. O mesmo controle do controle deslizante de Ganho da janela principal.
settings-radio-pre-selection-filters = Filtros de pré-seleção
settings-radio-low-pass-bank-front-adc = O banco de passa-baixas antes do ADC. Desviá-lo dá a visão mais larga e o pior comportamento perto de sinais fortes fora da faixa — um transmissor de radiodifusão a alguns quilômetros vai colocar imagens por toda a faixa. Deixe ligado a menos que esteja deliberadamente escutando fora da faixa filtrada.
settings-radio-filters-circuit = Filtros no circuito
settings-radio-rig-control-fdm-duo-only = Controle do equipamento — somente FDM-DUO
settings-radio-transceiver-s-cat-usb-port = A porta USB CAT do transceptor, que é um cabo separado do de recepção. Deixe a porta sem definir em um FDM-S1 ou FDM-S2, que não têm nenhuma — e em um FDM-DUO você preferiria dirigir só pelo cabo de recepção, onde ele ainda pode ser sintonizado e acionado, mas nenhum medidor pode ser lido.
settings-radio-port-set-radio-s-display = Com a porta definida, o visor do rádio e este dial são o mesmo número nos dois sentidos: a janela de recepção do FDM-DUO é construída em torno do seu VFO, então sintonizar aqui move o painel frontal, e girar o botão do painel frontal move a leitura aqui. O panadaptador se recentraliza no dial conforme anda. Sem porta — ou com uma à qual o rádio não responda —, não há VFO a comandar, e a sintonia fica dentro da janela que o rádio já está enviando.
settings-radio-none-usb-control-only = — nenhuma (somente controle USB) —
settings-radio-must-match-menu-70-cat = Deve coincidir com o menu 70 "CAT BAUD" do rádio, que sai de fábrica em 38400. O rádio tem estas quatro taxas e nenhuma outra, e uma porta aberta em qualquer outra fica muda nos dois sentidos — o rádio ignora o dial e recusa acionar, sem nada que diga por quê.
settings-radio-how-transmit-keyed-cat-sends = Como a transmissão é acionada. "CAT" envia o próprio comando TX do rádio e não exige nada configurado no equipamento; RTS exige o menu 54 "PTT" em PTT+RTS.
settings-radio-where-radio-takes-transmit-audio-2 =
    De onde o rádio pega o áudio de transmissão — o comando TI do equipamento, que é o menu 32 "TX IN" no painel frontal. Afirmado quando a porta abre.

    "USB audio" is what makes transmit work here: the FDM-DUO sends what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.
settings-radio-which-two-sockets-back-receiver-2 =
    Qual dos dois conectores da traseira o receptor escuta — o comando AN do equipamento, que é o menu 31 "ANTENNAS" no painel frontal e o indicador "ANT 1 2" no visor.

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    It moves this whole receiver: the panadapter, the demodulators and the rig's own audio all come from the socket selected here.

    Applies immediately, and is read back from the radio when the control port opens — this is the rig's own setting, not a copy of it kept here.
settings-radio-how-often-radio-asked-what-2 = Com que frequência o rádio é perguntado sobre o que está fazendo — o dial, o modo e os medidores. Gire o botão VFO do próprio equipamento e a leitura acompanha em uma consulta. É também todo o tráfego de controle que esta ponta gera, então reduza-o se o áudio do rádio falhar.
settings-radio-transmit-audio = Áudio de transmissão
settings-radio-only-receiving-fdm-duo-has = Até agora, só a recepção em um FDM-DUO foi executada em hardware real. Se algo apresentar mau funcionamento, anexe o relatório de diagnóstico a um relatório de bug.
settings-radio-cw-keyed-radio-s-own = O CW é acionado pela tecla ou manipulador próprio do rádio: o FDM-DUO não tem comando que aceite texto, então o painel CW não consegue acioná-lo.
settings-radio-press-apply-reopen-radio = Pressione Aplicar para reabrir o rádio.
settings-radio-no-receivers-press-rescan = nenhum receptor — pressione Buscar novamente
settings-radio-which-rates-receiver-has-depends = Quais taxas um receptor tem depende do modelo e do firmware. Tem efeito em Aplicar.
settings-radio-every-rate-any-hf-model = todas as taxas que qualquer modelo HF+ oferece — conecte um para ver a sua própria
settings-radio-receiver-s-own-gain-control = O controle de ganho próprio do receptor. Deixe-o ligado para a escuta geral; desligue-o para ajustar o atenuador à mão, para medição.
settings-radio-agc-threshold = Limiar do AGC
settings-radio-high = Alto
settings-radio-high-trades-little-sensitivity-headroom = Alto troca um pouco de sensibilidade por folga contra vizinhos fortes — a configuração certa em uma faixa lotada à noite.
settings-radio-front-end-attenuation-gain-0 = Atenuação do front end, como ganho — 0 dB é nenhuma. Só é obedecida com o AGC desligado.
settings-radio-preamp = Pré-amplificador
settings-radio-hf-low-noise-amplifier-buys = O amplificador de baixo ruído de HF. Compra sensibilidade ao custo de intermodulação, então vem desligado por padrão — o que costuma ser o certo com uma antena real.
settings-radio-lna = LNA ligado
settings-radio-frequency-calibration = Calibração de frequência
settings-radio-parts-per-billion-receiver-s = Partes por bilhão — a unidade própria deste receptor, mil vezes mais fina que o ppm que um RTL-SDR usa. Nada disto é gravado na memória flash do receptor: isto substitui o valor armazenado só para a sessão.
settings-radio-use-receiver-s-stored-value = Usar o valor armazenado no receptor
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = Alimentar CC pelo coaxial
settings-radio-not-every-hf-has-one = Nem todo HF+ tem um; em um receptor sem ele, isto não faz nada.
settings-radio-host-dsp = DSP do computador
settings-radio-image-balancer-zero-if-offset = O balanceador de imagem, o deslocamento de zero-IF e o oscilador de ajuste fino. Desligue-os apenas para ver a saída bruta do hardware — com eles desligados, a imagem espelhada aparece nas taxas de zero-IF e o dial é preciso só até o quilohertz mais próximo.
settings-radio-correct-image-fine-tune = Corrigir a imagem e o ajuste fino
settings-radio-receive-only-no-soapysdr-no = Somente recepção. Não precisa de SoapySDR nem de libairspyhf. Abaixo do piso do sintetizador, o oscilador do computador faz a sintonia, que é como este receptor alcança VLF. O receptor e a taxa de amostragem têm efeito em Aplicar; o restante vale à medida que você altera.
settings-radio-not-yet-verified-against-real-3 = Ainda não verificado em hardware real. Se apresentar mau funcionamento, envie o relatório de diagnóstico — ele contém cada comando trocado com o receptor e os primeiros bytes do fluxo de amostras decodificados como pares I/Q.
settings-radio-no-airspy-r2-mini-found = Nenhum Airspy R2 ou Mini encontrado — pressione Buscar novamente
settings-radio-read-receiver = ler deste receptor
settings-radio-r2-mini-offer-different-rates = um R2 e um Mini oferecem taxas diferentes e não podem ser distinguidos até que um esteja aberto
settings-radio-rate-you-get-receiver-s = Esta é a taxa que você obtém. O ADC do receptor roda o dobro dela — digitaliza um sinal real, e o sdroxide faz a banda base complexa a partir dele no computador.
settings-radio-gain-curve = Curva de ganho
settings-radio-step-along-curve-above-not = Um passo ao longo da curva acima, não um valor em dB — o LNA, o misturador e o VGA do sintonizador se movem juntos, e quanto cada passo vale depende da curva e da faixa. 0 é o extremo silencioso.
settings-radio-tuner-agc = AGC do sintonizador
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = Entrega o LNA ao laço próprio do sintonizador, substituindo a curva.
settings-radio-mixer = Misturador
settings-radio-same-mixer-stage = O mesmo para o estágio do misturador.
settings-radio-loop-running-gain-slider-no = Com um laço rodando, o controle deslizante de ganho não define mais o estágio que ele possui — o laço o sobrescreve.
settings-radio-dc-antenna-port = CC na porta de antena
settings-radio-t-12-bit-packing = Empacotamento de 12 bits
settings-radio-third-less-usb-traffic-leave = Um terço a menos de tráfego USB. Deixe ligado: este é um dispositivo USB 2.0 e a taxa máxima é 30 MB/s empacotada contra 40 sem empacotar. Vale ao reconectar.
settings-radio-dc-removal = Remoção de CC
settings-radio-remove-adc-s-offset = Remove o offset do ADC
settings-radio-turn-off-see-raw-hardware = Desligue para ver a saída bruta do hardware. Vale saber para onde vai o pico: o offset cai na borda da faixa, não no centro, porque o sinal é transladado por um quarto da taxa de amostragem no caminho.
settings-radio-every-command-exchanged-receiver-sample = Cada comando trocado com o receptor, a aritmética da taxa de amostragem e as primeiras amostras decodificadas como I/Q.
settings-radio-receive-only-24-1800-mhz = Somente recepção, 24–1800 MHz. Não precisa de SoapySDR nem de libairspy. O receptor, a taxa de amostragem e o empacotamento têm efeito em Aplicar; o restante vale à medida que você altera.
settings-radio-not-yet-verified-against-real-4 = Ainda não verificado em hardware real. Se apresentar mau funcionamento, envie o relatório de diagnóstico — ele contém cada comando trocado com o receptor, a aritmética de taxas e as primeiras amostras decodificadas como pares I/Q.
settings-radio-no-hydrasdr-rfone-found-press = Nenhum HydraSDR RFOne encontrado — pressione Buscar novamente
settings-radio-these-are-rates-receiver-turned = estas são as taxas que este receptor acabou tendo
settings-radio-receiver-only-reports-three-these = o receptor só informa três destas; as demais estão na tabela alternativa do firmware, e uma versão mais antiga pode não tê-las
settings-radio-rf-input = Entrada de RF
settings-radio-antenna-sma-only-socket-bias = A SMA de antena — o único conector com o bias tee por trás.
settings-radio-cable-socket-no-bias-tee = Um conector de cabo. Sem bias tee aqui; o hardware o coloca somente em ANT.
settings-radio-powers-active-antenna-preamp-down-2 = Alimenta uma antena ativa ou pré-amplificador pelo coaxial. Somente o conector ANT tem isso — as duas portas de cabo são entradas comuns.
settings-radio-third-less-usb-traffic-leave-2 = Um terço a menos de tráfego USB. Deixe ligado: este é um dispositivo USB 2.0 e a taxa máxima é 36 MB/s empacotada contra 48 sem empacotar. Vale ao reconectar.
settings-radio-one-these-boards-1d50-60a1 = Uma destas placas está em 1d50:60a1, o ID USB que os protótipos da HydraSDR compartilham com o Airspy R2 e o Mini. O sdroxide confere o firmware depois de abrir e informa se a interface errada foi escolhida, nos dois sentidos.
settings-radio-receive-only-24-1800-mhz-2 = Somente recepção, 24–1800 MHz. Não precisa de SoapySDR nem de libhydrasdr. O receptor, a taxa de amostragem e o empacotamento têm efeito em Aplicar; o restante vale à medida que você altera.
settings-radio-re-list-fobos-sdrs-no = Relistar os SDRs Fobos. Nenhum dispositivo é aberto, então é seguro pressionar durante a recepção.
settings-radio-no-fobos-sdr-found-press = Nenhum SDR Fobos encontrado — pressione Buscar novamente
settings-radio-input-2 = Entrada
settings-radio-through-tuner-lna-vga-mixer = Pelo sintonizador: LNA, VGA e a faixa de frequência do próprio misturador.
settings-radio-direct-sampling-hf1-antenna-input = Amostragem direta na entrada de antena HF1 — sem sintonizador, sem ganho de LNA/VGA, sintonizada inteiramente em software.
settings-radio-direct-sampling-hf2-antenna-input = Amostragem direta na entrada de antena HF2, igual à HF1.
settings-radio-both-real-adc-channels-once = Os dois canais reais do ADC ao mesmo tempo, combinados pelo filtro de diversidade abaixo — uma fonte de ruído anulada, ou dois caminhos com desvanecimento combinados.
settings-radio-direct-sampling-tuner-so-lna = Amostragem direta — o sintonizador (e portanto o ganho de LNA/VGA) não está no caminho. A taxa abaixo é um alvo para o conversor de descida em software, não uma configuração de hardware, então a taxa obtida pode diferir dela.
settings-radio-receiver-s-own-reported-rates = as taxas que este receptor informa
settings-radio-measured-one-real-unit-connect = medido em uma unidade real — conecte este receptor e Buscar novamente para ver a lista dele
settings-radio-clock-source = Fonte de clock
settings-radio-external-reference = Referência externa
settings-radio-diversity-taps = Derivações de diversidade
settings-radio-taps = {" "}derivações
settings-radio-one-tap-gain-phase-null = Uma derivação é um ganho e uma fase — um nulo em uma frequência que piora de cada lado, que é tudo o que um defasador analógico consegue fazer. Cada derivação adicional compra um período de amostra da diferença de caminho entre HF1 e HF2 que o filtro consegue equalizar, e é isso que transforma esse entalhe em uma faixa silenciosa de ponta a ponta.
settings-radio-receive-only-rf-port-tuner = Somente recepção. Porta RF (sintonizador), HF1/HF2 (amostragem direta, decodificada em software) e HF1+HF2 (os dois ao mesmo tempo, combinados pelo filtro de diversidade) funcionam todos. O receptor, a entrada e a taxa de amostragem têm efeito em Aplicar; ganho, fonte de clock e derivações valem à medida que você os altera — o modo, a taxa de adaptação e a retenção do filtro de diversidade ficam na caixa DIV da própria janela principal, depois que este rádio é selecionado.
settings-radio-no-hackrf-found-press-rescan = Nenhum HackRF encontrado — pressione Buscar novamente
settings-radio-front-end-amplifier-8-db = Amplificador de front end, em passos de 8 dB. Este é o estágio que muda a sensibilidade — e o estágio que sobrecarrega primeiro com uma antena real.
settings-radio-baseband-amplifier-after-mixer-2 = Amplificador de banda base depois do misturador, em passos de 2 dB. Aumente-o para um sinal fraco antes de recorrer ao LNA.
settings-radio-rf-amp = Amp RF
settings-radio-one-switch-shared-transmit-setting = Uma chave, compartilhada com a configuração de transmissão abaixo — o rádio aplica a que pertence à direção em que está entrando. Desligado costuma ser o certo com uma antena real.
settings-radio-hackrf-pro-chooses-itself-three = Um HackRF Pro escolhe isto sozinho — três quartos da taxa de amostragem, filtrada na FPGA — e ignora qualquer coisa que o computador peça.
settings-radio-leave-automatic-choosing-one-too = Deixe em Automático. Escolher uma muito estreita não suaviza só as bordas da faixa — ela retira o deslocamento do LO que mantém o pico de CC fora do seu sinal.
settings-radio-about-3-v-50-ma = Cerca de 3 V a 50 mA pelo coaxial, para uma antena ativa ou um pré-amplificador. Somente HackRF One ou Pro; o Jawbreaker e o rad1o não têm esse circuito.
settings-radio-remove-dc-mirror-image = Remover CC e a imagem espelhada
settings-radio-zero-if-radio-so-own = Este é um rádio de zero-IF, então o vazamento do próprio LO fica no meio da faixa, e o erro de quadratura do misturador coloca uma imagem espelhada sobre ele. Desligar isto mostra a saída bruta do hardware, que é a forma rápida de distinguir um problema de driver de um problema de DSP.
settings-radio-corrects-reference-oscillator = Corrige o oscilador de referência.
settings-radio-transmit = Transmitir
settings-radio-transmitter = Transmissor
settings-radio-off-default-while-off-backend = Desligado por padrão. Enquanto isto está desligado, o backend não publica nenhum canal de transmissão, então nada pode acionar o rádio. Vale ao reconectar.
settings-radio-tx-vga-gain = Ganho VGA de TX
settings-radio-transmit-driver-amplifier-drive-applied = O amplificador de driver de transmissão. A Excitação é aplicada digitalmente antes deste estágio, então deixe a excitação alta e defina o nível de saída aqui — reduzir a excitação faz o DAC rodar a uma fração da escala total e aumenta a intermodulação.
settings-radio-same-switch-receive-setting-above = A mesma chave da configuração de recepção acima, aplicada quando o rádio muda de direção.
settings-radio-transmit-armed-into-dummy-load = A transmissão está armada. Em carga fantasma até você tê-la medido: os harmônicos de um HackRF são fortes o bastante para exigir um filtro passa-baixas externo para a faixa em que você está, e ele é meio duplex, então a recepção para durante cada transmissão.
settings-radio-every-command-exchanged-radio-session = Cada comando trocado com o rádio nesta sessão, em ordem, inclusive a sequência em torno de cada acionamento.
settings-radio-dc-7-25-ghz-half = CC – 7,25 GHz, meio duplex. Não precisa de SoapySDR, libusb nem libhackrf. O rádio e a taxa de amostragem têm efeito em Aplicar; o restante vale à medida que você altera. As bordas de faixa são as do firmware: um HackRF é especificado para 1 MHz – 6 GHz (100 kHz em um Pro) e fica muito atenuado fora disso, mas sintoniza ali.
settings-radio-ask-sdrplay-api-service-device = Pede ao serviço SDRplay API a lista de dispositivos dele. Nada é aberto, então é seguro pressionar durante a recepção.
settings-radio-no-rsps-press-rescan-needs = nenhum RSP — pressione Buscar novamente (precisa do serviço SDRplay API)
settings-radio-both-tuners-running-api-fixes = Com os dois sintonizadores rodando, a API fixa o ADC em 6 MHz e devolve 2 Msps a partir de um F.I. baixo — então 2 Msps é a largura mais ampla, e as mais estreitas são essa taxa decimada. Tem efeito em Aplicar.
settings-radio-rates-below-2-msps-run = Taxas abaixo de 2 Msps fazem o ADC rodar a 2 Msps e decimam no serviço. Tem efeito em Aplicar.
settings-radio-reduced-adc-resolution = {"  "}(resolução reduzida do ADC)
settings-radio-if-bandwidth = Largura de banda de F.I.
settings-radio-tuner-s-analog-filter-auto = O filtro analógico do sintonizador. Automático escolhe o mais largo que cabe na taxa de amostragem. Tem efeito em Aplicar.
settings-radio-rsp-s-own-if-gain = O laço de ganho de F.I. próprio do RSP, rodado pelo serviço de API. Desligado devolve a você o controle deslizante de ganho de F.I. — a configuração para medição e modos digitais de sinal fraco.
settings-radio-agc-set-point = Ponto de ajuste do AGC
settings-radio-signal-level-loop-holds-adc = Nível de sinal em que o laço mantém o ADC. Mais baixo deixa mais folga para sinais fora do canal. Quão baixo ele pode ir depende da taxa de amostragem — acima de 8,064 Msps o conversor tem menos folga para dar, e acima de 9,216 Msps, menos ainda.
settings-radio-rsp-s-native-gain-unit = A unidade de ganho nativa do RSP: 20 dB é o ganho máximo (0 com a faixa estendida abaixo), 59 dB o mínimo. Vale imediatamente. Ignorado enquanto o AGC está rodando — o laço é dono deste valor então, e o S-metro mostra em que ele se fixou.
settings-radio-extended-if-range = Faixa de F.I. estendida
settings-radio-lets-if-gain-reduction-go = Deixa a redução de ganho de F.I. descer abaixo de 20 dB, até 0 — os últimos 20 dB de ganho que o receptor tem. Desligado é o padrão da própria API e o certo para a escuta comum, porque o fundo da faixa é onde um RSP mais facilmente sobrecarrega. Vale ligar para sinais fracos com o LNA já em 0. Também permite que o ponto de ajuste do AGC suba até 0 dBFS. Vale imediatamente.
settings-radio-allow-if-gain-reduction-below = Permitir redução de ganho de F.I. abaixo de 20 dB
settings-radio-front-end-attenuation-steps-0 = Atenuação do front end em passos: 0 é o ganho máximo, cada passo acima acrescenta mais atenuação. Algumas faixas têm menos passos — o driver limita e mantém a sua escolha para quando você sintonizar de volta, e a barra aqui termina no que a faixa em que o rádio está aceitará. Uma escolha mais alta feita em outra faixa é mantida e continua mostrada ao lado. Vale imediatamente.
settings-radio-t-0-max-gain = 0 = ganho máximo
settings-radio-reference-error-parts-per-million-2 = Erro de referência em partes por milhão, aplicado pelo próprio dispositivo. Vale imediatamente.
settings-radio-radio-s-tuner = O sintonizador deste rádio
settings-radio-tuner = Sintonizador
settings-radio-which-rspduo-s-two-tuners = Qual dos dois sintonizadores do RSPduo este rádio escuta. O outro pertence ao rádio configurado para ele — dê a esse o mesmo receptor e o outro sintonizador. Tem efeito em Aplicar.
settings-radio-which-rspduo-s-two-tuners-2 = Qual dos dois sintonizadores do RSPduo carrega a antena que você está escutando. O outro carrega a segunda antena. Tem efeito em Aplicar.
settings-radio-which-rspduo-s-two-tuners-3 = Qual dos dois sintonizadores do RSPduo rodar (um de cada vez). Tem efeito em Aplicar.
settings-radio-applies-immediately = Vale imediatamente.
settings-radio-dab-notch = Notch DAB
settings-radio-hardware-notch-over-165-230 = Notch de hardware sobre a faixa DAB de 165–230 MHz. Vale imediatamente.
settings-radio-hdr-mode = Modo HDR
settings-radio-enable-below-2-mhz = Ativar abaixo de 2 MHz
settings-radio-rspdx-s-high-dynamic-range =
    O caminho de alta faixa dinâmica do RSPdx para LF/MF. Vale imediatamente.

    It does not work yet: on the one RSPdx it has been tried against, switching it on silences the receiver — at every centre, filter, port, gain and rate tried. Whatever the mode needs is not on the documented API surface, and finding it is somebody's next job. Leave this off unless you are the one looking.

    What is known: it is not a mode that follows the dial. The path has a fixed analog filter, built only at the centres listed under the filter below, so tuning anywhere else could not work even once the rest does.
settings-radio-hdr-filter = Filtro HDR
settings-radio-feed-4-7-v-dc = Alimentar ~4,7 V CC pelo coaxial
settings-radio-second-tuner = Segundo sintonizador
settings-radio-rspduo-two-whole-tuners-one = Um RSPduo são dois sintonizadores inteiros em uma placa, com clock de uma mesma referência. Rodando os dois, eles ouvem seus trechos no mesmo instante a partir do mesmo clock — o que torna coerentes duas antenas ligadas a eles, com uma fase relativa definida pelas linhas de alimentação e não pelo acaso, e permite combinar o par. Ou deixe-os separados: os sintonizadores sintonizam de forma independente, então o outro pode ser um segundo rádio na própria faixa.
settings-radio-run-both-tuners = Rodar os dois sintonizadores
settings-radio-puts-rspduo-api-s-dual = Coloca o RSPduo no modo de dois sintonizadores da API, em que o ADC fica fixo em 6 MHz e a maior largura é 2 Msps. Tem efeito em Aplicar, porque o modo é escolhido quando o dispositivo é aberto.
settings-radio-used = Usado para
settings-radio-what-other-tuner-takes-effect = Para que serve o outro sintonizador. Tem efeito em Aplicar — como rodar os dois, isto é escolhido quando a placa é aberta.
settings-radio-add-second-radio-settings-radio = Adicione um segundo rádio (Configurações → Rádio → +), dê a ele este mesmo receptor e o outro sintonizador do RSPduo, e configure-o para rodar os dois sintonizadores também — o rádio que abrir a placa a coloca no modo de dois sintonizadores, então ambos precisam esperar por isso. Os dois então sintonizam de forma independente: HF em uma aba e VHF na outra, a partir de uma só placa. Eles compartilham um clock de ADC, então ambos rodam na taxa de amostragem que o rádio que abriu a placa pediu, e nenhum dos dois pode transmitir.
settings-radio-what-do = O que fazer com ele
settings-radio-lna-state-2 = Seu estado de LNA
settings-radio-second-tuner-s-own-front = A atenuação de front end própria do segundo sintonizador. Ajuste para que as duas antenas mostrem aproximadamente o mesmo piso de ruído: este é o ajuste em que tudo o mais se apoia, porque a combinação pondera os dois ramos pelo ruído deles, e um segundo front end levado à sobrecarga entrega ao filtro uma cópia distorcida da interferência — que não pode ser subtraída de uma não distorcida. Vale imediatamente.
settings-radio-if-gain-reduction-2 = A redução de ganho de F.I. dele
settings-radio-second-tuner-s-if-gain = O ganho de F.I. do segundo sintonizador, na unidade nativa do RSP: 20 dB é o ganho máximo. Ignorado enquanto o AGC está rodando — e um ganho estável é o que o filtro quer, então desligar o AGC vale a pena para um nulo que você pretende manter.
settings-radio-filter-length = Comprimento do filtro
settings-radio-one-tap-gain-phase-null-2 = Uma derivação é um ganho e uma fase — um nulo em uma frequência que piora de cada lado, que é tudo o que um defasador analógico consegue fazer. Cada derivação adicional compra um período de amostra da diferença de caminho entre as duas antenas que o filtro consegue equalizar, e é isso que transforma esse entalhe em uma faixa silenciosa de ponta a ponta.
settings-radio-adaptation-2 = Adaptação
settings-radio-slow-steady-left-converging-inside = Lento e estável à esquerda, convergindo em menos de um segundo e visivelmente procurando à direita. Comece rápido para achar o nulo, depois mantenha-o.
settings-radio-stop-filter-moving-reach-moment = Pare o filtro de se mover. Use isto no momento em que um nulo aparecer: um filtro deixado adaptando se reapontará para o que ficar mais forte, o que, em uma faixa silenciosa, é a estação que você está ouvindo.
settings-radio-restart = Reiniciar
settings-radio-zero-filter-find-null-again = Zera o filtro e encontra o nulo de novo.
settings-radio-nothing-here-can-tell-wanted = Nada aqui consegue distinguir um sinal desejado de um indesejado — o filtro só sabe o que as duas antenas têm em comum. Em Cancelar, a segunda antena quer ouvir a fonte de ruído e o mínimo possível da faixa, ou vai cancelar a estação também, diligentemente. Em Combinar, as duas querem ouvir a mesma estação. A profundidade do nulo é registrada no log a cada poucos segundos.
settings-radio-not-yet-verified-against-rspduo = Ainda não verificado em um RSPduo: a operação com dois sintonizadores aqui foi escrita a partir da API da SDRplay, não medida no hardware.
settings-radio-no-receiver-listed-check-against = Nenhum receptor está listado para conferir isto — pressione Buscar novamente, ou leia o log para ver o que de fato abriu.
settings-radio-receive-only-1-khz-2 = Somente recepção, 1 kHz–2 GHz. Precisa do serviço SDRplay API do fabricante (sdrplay.com/api) — os RSPs posteriores ao RSP1 original não têm protocolo aberto. Dispositivo, taxa de amostragem, largura de banda e sintonizador do RSPduo têm efeito em Aplicar; o restante vale à medida que você altera.
settings-radio-board = Placa
settings-radio-first-one-found-2 = O primeiro encontrado
settings-radio-receive-chain = Cadeia de recepção
settings-radio-both-chains-tune-together-they = As duas cadeias sintonizam juntas — compartilham um sintetizador —, mas são front ends separados em conectores separados. Escolha aquela em que sua antena está, que é a configuração a usar quando uma cadeia recebeu a modificação de casamento de HF e a outra é de fábrica.
settings-radio-receive-gain = Ganho de recepção
settings-radio-one-combined-figure-which-limesuite = Um valor combinado, que o LimeSuite distribui entre o LNA, o TIA e o PGA por conta própria. Aceita decibéis inteiros, então qualquer coisa mais fina é truncada.
settings-radio-receive-port = Porta de recepção
settings-radio-automatic-follows-frequency-lnal-low = O automático segue a frequência: LNAL em baixa, LNAH em alta — a menos que um LimeRFE esteja conectado abaixo, que é um cabo em um conector, e então é LNAW em todas as frequências. Informe o conector ao qual o seu está ligado, se não for esse. A cadeia a que o conector pertence é o seletor acima.
settings-radio-t-0-follows-sample-rate-worth = 0 segue a taxa de amostragem. Vale deixar assim: um filtro mais estreito que um quarto da largura de faixa retira em silêncio o deslocamento do LO de zero-IF, o que devolve o vazamento do LO por cima do sinal que você está ouvindo.
settings-radio-following-rate = seguindo a taxa
settings-radio-corrections = Correções
settings-radio-host-iq-dc-correction = Correção I/Q / CC no computador
settings-radio-adaptive-image-dc-removal-side = Remoção adaptativa de imagem e de CC deste lado, além da calibração própria do chip. Desligá-la é a forma de um clique para distinguir um problema de driver de um problema de DSP.
settings-radio-calibrate-automatically = Calibrar automaticamente
settings-radio-runs-chip-s-own-dc = Roda a calibração própria de offset de CC e de imagem do chip quando o rádio é aberto, e de novo depois que o dial se estabiliza em uma nova faixa ou um conector diferente. Esses números são medidos em uma frequência e erram em outras, que é o que uma portadora no meio da faixa normalmente é. Custa cerca de um segundo cada vez.
settings-radio-calibrate-now = Calibrar agora
settings-radio-stalls-receiver-better-part-second = Trava o receptor por boa parte de um segundo.
settings-radio-second-aerial = Segunda antena
settings-radio-socket = Seu conector
settings-radio-same-first = Igual à primeira
settings-radio-gain-2 = Seu ganho
settings-radio-set-low-coupled-sample-your = Ajuste BAIXO. A amostra acoplada do seu próprio transmissor é um sinal forte, e uma cadeia de realimentação levada à compressão mede mal a curva do amplificador — ela ensina à correção a própria distorção. Comece no fundo e use o atenuador do acoplador.
settings-radio-set-so-both-aerials-show = Ajuste para que as duas antenas mostrem aproximadamente o mesmo piso de ruído. Este é o ajuste em que tudo se apoia: a combinação pondera os dois ramos pelo ruído deles, e uma segunda cadeia levada à compressão entrega ao filtro uma cópia distorcida da interferência — que não pode ser subtraída de uma não distorcida.
settings-radio-table-steps-2 = Passos da tabela
settings-radio-how-finely-correction-follows-amplifier = Com que precisão a correção segue a curva do amplificador. Mais passos acompanham um joelho mais agudo, mas cada um precisa ser aprendido a partir das amostras que caíram nele — e o topo de um histograma de amplitude de voz é fino. Trinta e dois basta para a curva suave que um amplificador de HF de fato tem. Alterá-lo reinicia a correção.
settings-radio-how-hard-each-block-feedback = Quanto cada bloco de realimentação move a correção. A curva de um amplificador não muda, então não há pressa — o meio tira dela o ruído de várias transmissões.
settings-radio-keep-correction-curve-learned-clean = Mantenha a correção como está. Uma curva aprendida em uma transmissão limpa vale a pena manter, e o amplificador não terá mudado até a próxima.
settings-radio-forget-correction-learn-again = Esquecer a correção e aprendê-la de novo.
settings-radio-directional-coupler-amplifier-s-output = Um acoplador direcional na saída do amplificador entra nesta cadeia, e o transmissor compara o que voltou com o que pretendia enviar — então envia o inverso da diferença, de modo que o que sai do amplificador fica reto. Uns vinte e poucos decibéis a menos de intermodulação nos QSOs dos outros, com o amplificador mantendo a potência. A correção fica em unidade até que a realimentação se alinhe com a transmissão, então um acoplador não conectado não custa nada; e ela nunca pode pedir ao conversor mais que a escala total, então um caminho de realimentação lendo bobagem não sobrecarrega nada. Como está indo, é registrado no log enquanto você transmite.
settings-radio-transmit-not-armed-so-there = A transmissão não está armada, então não há nada aqui para corrigir.
settings-radio-off-interface-publishes-no-transmit = Com isto desligado, a interface não publica nenhum canal de transmissão, então nada pode acionar o rádio.
settings-radio-transmit-gain = Ganho de transmissão
settings-radio-transmit-port = Porta de transmissão
settings-radio-board-s-two-transmit-sockets = Os dois conectores de transmissão da placa são duas redes de casamento diferentes, não duas tomadas na mesma: BAND1 (TX_1) cobre de 30 MHz a 1,9 GHz e BAND2 (TX_2) é a porta de 13 cm. Acionar uma faixa de amador abaixo de 23 cm pela BAND2 põe a transmissão em uma rede de casamento que deixa passar quase nada — todas as configurações leem correto, a excitação é a que você ajustou, e o medidor de potência fica em zero. Deixe isto em Automático e ele segue o dial.
settings-radio-limesdr-transmits-about-100-khz = Uma LimeSDR transmite de cerca de 100 kHz a 3,8 GHz sem filtragem própria. Use um passa-baixas, um canal LimeRFE apropriado ou uma carga fantasma.
settings-radio-limerfe-front-end = Front end LimeRFE
settings-radio-connected = Conectado por
settings-radio-limerfe-s-own-micro-usb = A porta micro-USB do próprio LimeRFE, não a do rádio. 9600 baud, fixo pelo firmware dele.
settings-radio-receive-connector = Conector de recepção
settings-radio-transmit-connector = Conector de transmissão
settings-radio-follow-dial = Seguir o dial
settings-radio-switch-filters-match-operating-frequency = Comuta os filtros para casar com a frequência de operação, antes de qualquer RF aparecer. Sintonizar dentro de uma só faixa não põe nada no link de controle.
settings-radio-relays = Relés
settings-radio-receive-attenuator = Atenuador de recepção
settings-radio-notch-filter = Filtro notch
settings-radio-fan = Ventilador
settings-radio-worth-having-any-sustained-transmitting = Vale ligar para qualquer transmissão contínua.
settings-radio-automatic-board-receives-switched-transmit = No Automático a placa recebe, e é comutada para transmitir na tecla para baixo e de volta na tecla para cima — em qualquer cabeamento. Os canais de amador dela têm um filtro com uma chave de transmissão/recepção de cada lado, então uma placa solicitada para os dois ao mesmo tempo põe essa chave no transmissor e para de ouvir qualquer coisa. O canal de 30 MHz só é alcançável pelo J5, que é um conector para as duas direções; acima dele, transmitir pelo J4 mantém o caminho de recepção fora do conector que o amplificador está acionando.
settings-radio-limesdr-support-has-not-been = O suporte à LimeSDR não foi verificado em hardware. Este é o rastro da última sessão, para um relatório de problema.
settings-radio-gains-filters-corrections-antenna-sockets = Ganhos, filtros, correções, os conectores de antena e todo controle do LimeRFE — conectores, faixa, relés, atenuador, notch e ventilador — valem imediatamente. A placa, a cadeia de recepção, a taxa de amostragem, o armamento da transmissão e a conexão do LimeRFE têm efeito em Aplicar.

## Settings window, Radio tab: messages with values

settings-radio-fill-from = PREENCHER DE { $name }
settings-radio-decimation-auto = Automático (mais próximo de { $khz } kHz)
settings-radio-decimation-stage-server = Estágio { $stage } — a taxa do servidor ÷ { $divisor }
settings-radio-decimation-stage = Estágio { $stage } — taxa ÷ { $divisor }
settings-radio-kiwi-receive-only = Somente recepção — esta é a antena de outra pessoa. O panadaptador é os cerca de 12 kHz que o receptor envia como I/Q, que segue o dial; a visão de banda acima é a cascata própria dele, e sintonizar por ela sintoniza o receptor. Cerca de { $rate } kB/s enquanto conectado. Pressione "Aplicar / reconectar" para trocar sem reiniciar.
settings-radio-connected-detail = Conectado: { $detail }
settings-radio-failed = Falhou: { $error }
settings-radio-pluto-buffer = ≈ { $ms } ms, { $kib } KiB por transferência
settings-radio-driver-settings = Configurações de { $driver }
settings-radio-rx888-coverage = Cobertura de 0–{ $mhz } MHz, { $mbytes } MB/s pelo USB
settings-radio-rx888-digitised = de { $mhz } MHz digitalizados
settings-radio-elad-16-bit = { $khz } kHz (amostras de 16 bits)
settings-radio-elad-pad = Atenuador de { $db } dB ativado
settings-radio-hackrf-rx-preamp = Pré-amplificador de { $db } dB na recepção
settings-radio-hackrf-tx-preamp = Pré-amplificador de { $db } dB na transmissão
settings-radio-device-not-found = { $device } (não encontrado)
settings-radio-lime-chain = Cadeia { $n } (RX{ $n }_*)
settings-radio-lime-chain-tx = Cadeia { $n } (RX{ $n }_* / TX{ $n }_*)
settings-radio-lime-second-chain = A outra cadeia de recepção da placa, nos conectores { $sockets }. Ela compartilha o sintetizador, então ouve a mesma faixa no mesmo instante que a primeira — o que permite que leve uma segunda antena, ou uma amostra do seu próprio transmissor.
settings-radio-tx-blocked = A transmissão está bloqueada: { $reason }
settings-radio-rfe-rx-fallback = Recebendo em { $port }, estas caem para o caminho de banda larga sem filtro: { $bands }.
settings-radio-rfe-tx-fallback = Transmitindo em { $port }, estas caem para o caminho de banda larga — sem amplificador de faixa e sem filtragem: { $bands }.

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = Volume, ganho, AGC, squelch, os controles de ruído e o visor RDS na radiodifusão FM
topbar-receiver = RECEPTOR
topbar-vfo-b-split-rit-xit = VFO A/B, split e os deslocamentos RIT/XIT
topbar-tuning = SINTONIA
topbar-diversity-filter-which-way-combines = O filtro de diversidade: de que forma combina as duas antenas, com que rapidez se adapta e como mantê-lo onde está
topbar-diversity = DIVERSIDADE
topbar-second-receiver-s-frequency-mode = A frequência, o modo, o filtro e o nível do segundo receptor
topbar-sub-receiver = SUB-RECEPTOR
topbar-transceiver-s-own-aerial-socket = O conector de antena próprio do transceptor, e a chave de energia dele
topbar-radio = RÁDIO
topbar-tune-voice-keyer-drive-mic = Tune, o manipulador de voz, e os níveis de excitação e de microfone
topbar-transmit = TRANSMISSÃO
topbar-tune = {" "}TUNE{" "}
topbar-display = EXIBIÇÃO
topbar-logbook-spots-awards-memories-scanner = Diário de contatos, spots, prêmios, memórias, o scanner, as configurações e o manual
topbar-system = SISTEMA
topbar-linked = Conectado
topbar-not-linked = Não conectado
topbar-bar-trace = barra / traço
topbar-needle-bar-trace = agulha / barra / traço
topbar-swap-vfos = Trocar VFOs
topbar-copy-b = Copiar A para B
topbar-second-receiver-right-ear-tunes = Segundo receptor, no ouvido direito. Ele sintoniza independentemente de A/B — seus controles aparecem no módulo SUB, e sua banda de passagem na cascata.
topbar-band-plan = , do plano de faixas
topbar-repeater-shift-following-band-plan = Deslocamento de repetidora: seguindo o plano de faixas, que não tem deslocamento para esta frequência — simplex
topbar-repeater-shift-transmit-above-below = Deslocamento de repetidora: transmitir acima ou abaixo do dial, para trabalhar uma repetidora. Simplex agora
topbar-repeater-shift = DESLOCAMENTO DE REPETIDORA
topbar-auto = AUTO
topbar-take-shift-band-plan-dial = Obter o deslocamento a partir do plano de faixas conforme o dial se move. Ele só age dentro de uma sub-faixa de saída de repetidora — em qualquer outro lugar, deixa o rádio em simplex, para que os canais de chamada fiquem em simplex.
topbar-offset = Deslocamento
topbar-repeater-tone-ctcss-dcs-goes = Tom de repetidora: o CTCSS/DCS que vai por baixo da voz, a rajada de 1750 Hz, e o squelch de tom de recepção
topbar-transmit-tone = TOM DE TRANSMISSÃO
topbar-set-here-sent-nfm-sub = definido aqui, enviado em NFM — a sinalização subaudível é de um canal FM
topbar-normal = NORMAL
topbar-invert = INVERTER
topbar-bit-order-dcs-encoded-here = A ordem de bits com que o DCS é codificado aqui é transcrita do padrão e nunca foi conferida com uma repetidora — se não abrir, tente CTCSS
topbar-t-1750-hz-burst = RAJADA DE 1750 HZ
topbar-every-over = A CADA TRANSMISSÃO
topbar-open-burst-every-time-transmitter = Abre com a rajada toda vez que o transmissor é acionado
topbar-send = ENVIAR
topbar-send-burst-now-keying-transmitter = Envia a rajada agora — acionando o transmissor pelo tempo dela se ele ainda não estiver acionado
topbar-nfm-only-burst-fm-repeater = Somente NFM: a rajada é o abridor de porta de uma repetidora FM
topbar-receive-tone-squelch = SQUELCH DE TOM DE RECEPÇÃO
topbar-require-receive-what-station-transmits = Exigir na recepção o que esta estação transmite
topbar-hide-docked-band-selector-click = Ocultar o seletor de faixa acoplado — clique de novo para trazê-lo de volta
topbar-show-docked-band-selector = Mostrar o seletor de faixa acoplado
topbar-band-mode = FAIXA E MODO
topbar-hide-band-chip-brings-back = Ocultar — o chip de faixa o traz de volta
topbar-dec-off = DEC desligado
topbar-vol = Vol
topbar-counted-steps-not-decibels-what =


    Counted in steps, not decibels: what one step is worth is the receiver's own business, and on an SDRplay it depends on the band as well. Right is still more gain.
topbar-gain = Ganho
topbar-plutosdr-s-own-agc-setting =


    The PlutoSDR's own AGC is setting this gain, so moving the slider does nothing. Set Settings → Radio → AGC to Manual to control it here.
topbar-agc-hang-time-click-cycle = Tempo de retenção do AGC — clique para alternar: Desligado / Lento / Médio / Rápido
topbar-man = Man
topbar-manual-audio-gain-used-while = Ganho de áudio manual, usado enquanto o AGC está desligado. Começa do nível que o AGC mantinha quando foi desligado.
topbar-radio-s-own-squelch-sent = O squelch próprio do rádio, enviado pelo link de controle. Esta é a porta pela qual o áudio realmente passa — o squelch por software só fecharia ainda mais sobre o que o equipamento já deixou passar.
topbar-max-gain = ganho máximo
topbar-manual-gain = ganho manual
topbar-noise-reduction = redução de ruído
topbar-impulse-noise-blanker = Limitador de ruído impulsivo
topbar-auto-notch-cancel-constant-tone = Notch automático: cancela os elementos de tom constante (heterodinos)
topbar-binaural-audio-not-while-sub = Áudio binaural — não enquanto o sub-receptor estiver com o ouvido direito
topbar-binaural-audio-passband-spread-across = Áudio binaural: a banda de passagem é espalhada entre os dois ouvidos, então sinais em tons diferentes vêm de direções diferentes, e o que você sintoniza flutua de um lado para o outro. Clique para voltar ao mono
topbar-binaural-cw-spread-passband-across = CW binaural: espalha a banda de passagem entre os dois ouvidos, para que sinais em tons diferentes venham de direções diferentes — uma pilha de chamadas vira vários lugares em vez de uma nota lotada, e sintonizar uma estação a faz flutuar. Melhor com fones
topbar-binaural-audio-spread-passband-across = Áudio binaural: espalha a banda de passagem entre os dois ouvidos. Na voz, o ruído se espalha por toda a imagem enquanto a estação fica no meio dela, o que é mais fácil de ouvir por uma hora — ao custo de a própria voz ficar espalhada. Melhor com fones
topbar-mute = SILENCIAR
topbar-record-audio-raw-i-q = Gravar o áudio, o I/Q bruto, ou os dois
topbar-wfm-stereo-forced-off-click = Estéreo WFM forçado desligado — clique para estéreo automático
topbar-wfm-stereo-pilot-locked-click = Estéreo WFM: piloto travado. Clique para forçar mono
topbar-wfm-stereo-automatic-no-pilot = Estéreo WFM: automático, sem piloto nesta estação
topbar-rds-data-arriving-click-see = Chegam dados RDS — clique para vê-los
topbar-rds-no-data-station-click = RDS — sem dados nesta estação. Clique assim mesmo para os diagnósticos do decodificador
topbar-drm-decoding-click-details = DRM decodificando — clique para os detalhes
topbar-drm-click-decoder-s-state = DRM — clique para o estado do decodificador
topbar-hd-radio-decoding-click-details = HD Radio decodificando — clique para os detalhes
topbar-hd-radio-click-decoder-s = HD Radio — clique para o estado do decodificador
topbar-presets = PREDEFINIÇÕES
topbar-width-per-sideband = LARGURA POR BANDA LATERAL
topbar-width = LARGURA
topbar-passband-width-hertz-type-drag = Largura da banda de passagem em hertz — digite-a ou arraste. Clique no campo para informar um valor exato.
topbar-edges = BORDAS
topbar-low-edge-hz-carrier = Borda inferior, em Hz a partir da portadora
topbar-high-edge-hz-carrier = Borda superior, em Hz a partir da portadora
topbar-noise-reduction-voice-off-click = Redução de ruído (voz), desligada — clique para escolher um motor
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = O que você ouve, em MP3: o receptor em um canal e a sua própria transmissão no outro. Esta é a gravação de um QSO.
topbar-mono = MONO
topbar-mixed-one-channel-click-two = Misturado em um canal — clique para dois
topbar-two-channels-rx-left-tx = Dois canais: RX à esquerda, TX à direita — clique para um canal misturado
topbar-quick-clip = CLIP RÁPIDO
topbar-stop-after = PARAR APÓS
topbar-no-stop = sem parada
topbar-auto-record = GRAVAÇÃO AUTOMÁTICA
topbar-set-squelch-auto-record-needs = Defina um squelch — a gravação automática precisa de um para reconhecer o silêncio
topbar-auto-record-needs-sdr-front = A gravação automática exige um front end SDR com squelch por software
topbar-stop-following-squelch = Parar de seguir o squelch
topbar-waiting-signal = aguardando um sinal
topbar-record-spectrum = GRAVAR ESPECTRO
topbar-i-q-wav = ● I/Q WAV
topbar-i-q-wav-2 = I/Q WAV
topbar-raw-spectrum-receiver-delivering-stereo = O espectro bruto que o receptor entrega, como WAV estéreo de ponto flutuante de 32 bits (RF64 acima de 4 GB), que SDR#, SDRuno, HDSDR e SDRangel abrem — e que o próprio sdroxide reproduz com --file. Grande: 8 bytes por amostra.
topbar-radio-hands-over-demodulated-audio = Este rádio entrega áudio demodulado, então não há I/Q para gravar.
topbar-what-capture-costs-sample-rate = Quanto uma captura custa nesta taxa de amostragem
topbar-engine = MOTOR
topbar-no-noise-reduction-decoders-never = Sem redução de ruído — os decodificadores nunca a viram de qualquer forma
topbar-strength = INTENSIDADE
topbar-tone = TOM
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS — nenhum tom subaudível neste sinal
topbar-tone-squelch = SQUELCH DE TOM
topbar-carrier-squelch-open-any-signal = Squelch de portadora: abre com qualquer sinal
topbar-require-tone-currently-being-received = Exigir o tom que está sendo recebido agora
topbar-ctcss = CTCSS
topbar-any-dcs = QUALQUER DCS
topbar-open-any-dcs-coded-signal = Abre com qualquer sinal codificado em DCS. Qual dos 104 códigos ele carrega não pode ser lido de forma confiável aqui, então não há nada mais fino para escolher
topbar-which-socket-back-radio-receiving =
    Qual conector na traseira o rádio está recebendo — o próprio comando ANT dele, a mesma configuração do botão ANT do painel frontal. Clique para avançar ao próximo.

    The choice is remembered per band, and put back the next time the dial crosses into that band.
topbar-radio-s-separate-receiving-antenna =
    A antena de recepção separada do rádio, ligada ao caminho de recepção ou desligada dele — a configuração RX ANT própria dele. A antena no conector principal continua na transmissão de qualquer forma.

    The radio remembers this per band itself, so sdroxide reads it back after every band change rather than putting back what it last saw: clicking here is the only thing that moves it.
topbar-radio-s-own-power-switch =
    A chave de energia própria do rádio, pelo link de controle, e a única liga/desliga real do programa — não a chave LINK do sdroxide, que fecha a ponta do sdroxide e deixa o rádio funcionando.

    For ON to reach anything the radio's control end has to stay awake while it is off: Network Control over the LAN, or a CI-V port still fed from the mains on a set switched off at the front.
topbar-switch-radio = Ligar o rádio
topbar-switch-radio-off-audio-meters = Desligar o rádio. O áudio e os medidores param com ele.
topbar-what-second-aerial-click-swap =
    Para que serve a segunda antena — clique para trocar.

    CANCEL subtracts it from the first, in the gain, phase and delay that make a local noise source line up on both: the DSP form of a noise-cancelling phaser. COMBINE adds the two in the phase that reinforces, weighted by how well each hears — diversity reception, which fills in fades.
topbar-hold = MANTER
topbar-stop-filter-moving-reach-moment = Pare o filtro de se mover. Use isto no momento em que um nulo aparecer: um filtro deixado adaptando se reapontará para o que ficar mais forte, o que, em uma faixa silenciosa, é a estação que você está ouvindo.
topbar-restart = REINICIAR
topbar-zero-filter-find-null-again = Zera o filtro e encontra o nulo de novo.
topbar-adapt = Adaptar
topbar-how-fast-filter-chases-slow = Com que rapidez o filtro persegue: lento e estável à esquerda, convergindo em menos de um segundo e visivelmente procurando à direita. Comece rápido para achar o nulo, depois MANTENHA-o.
topbar-where-sub-receiver-listens-shift = Onde o sub-receptor escuta. Shift-clique na cascata, ou arraste dentro da banda de passagem do sub, para movê-lo.
topbar-dial = ←DIAL
topbar-move-sub-receiver-main-dial = Mover o sub-receptor para o dial principal
topbar-dial-2 = DIAL←
topbar-move-main-dial-sub-receiver = Mover o dial principal para o sub-receptor
topbar-filter = Filtro
topbar-sub-receiver-passband-edges-hz = Bordas da banda de passagem do sub-receptor, em Hz
topbar-sub-receiver-level-plays-right = Nível do sub-receptor (toca no ouvido direito)
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = Manipulador de voz: gravar e transmitir mensagens armazenadas
topbar-drive = Excitação
topbar-tune-2 = Tune
topbar-mic = Mic
topbar-cessb = CESSB
topbar-microphone-gain = Ganho do microfone
topbar-deviation-how-far-mode-s =
    Desvio: quanto a rajada deste modo oscila um rádio que a modula por conta própria. Um transmissor FM converte nível de áudio em oscilação de frequência e não tem ALC para segurá-la, então a escala total em uma entrada de dados configurada para voz desvia demais — o que soa perfeitamente normal para quem ouve e não decodifica para ninguém.

    Kept per mode, so a deviation set for 1200 baud packet never lands on FT8.
topbar-transmit-audio-how-hard-mode =
    Áudio de transmissão: com que força este modo aciona o modulador de um rádio que modula o que enviamos a ele — um equipamento CAT pela placa de som, um FLEX, um Icom pela porta de rede. Reduza até que o ALC do equipamento mal se mova e defina a potência no rádio; ALC trabalhando em um modo digital de envelope constante é o que espalha o sinal.

    Drive is not this control: on these radios Drive reaches the rig's power register and never touches its audio.

    Kept per mode — FT8, RTTY, PSK and MCW each keep their own.
topbar-tx-audio = Áudio TX
topbar-cw-psk-rtty-skimmers-decode = Skimmers de CW / PSK / RTTY — decodificam sinais por toda a faixa e os marcam na cascata
topbar-skimmers = SKIMMERS
topbar-run-skimmer = Executar este skimmer
topbar-minimum-snr-decoded-signal-needs = SNR mínima que um sinal decodificado precisa ter para ser marcado
topbar-needs-wideband-iq-source = precisa de uma fonte I/Q de banda larga
topbar-cw-decoder = DECODIFICADOR CW
topbar-how-many-signals-model-reads = Quantos sinais o modelo lê ao mesmo tempo. Os demais mantêm a marcação, mas não carregam texto.
topbar-spots = SPOTS
topbar-save-skimmer-s-spot-list = Salvar a lista de spots do skimmer em um arquivo
topbar-show-hide-full-band-waterfall = Mostrar/ocultar a faixa de cascata da banda inteira acima do panadaptador — tudo o que este receptor consegue ver de uma vez
topbar-spectrum-waterfall-either-layer-both = Espectro e cascata — uma das camadas, as duas, ou nenhuma. Acesa quando as duas estão visíveis.
topbar-spectrum = ESPECTRO
topbar-show-spectrum = MOSTRAR ESPECTRO
topbar-draw-spectrum-line-across-top = Desenha a linha do espectro no topo do panadaptador. Desligada, a cascata ocupa toda a altura.
topbar-peak-hold = PEAK HOLD
topbar-trace-highest-level-each-column = Traça o nível mais alto que cada coluna atingiu sobre a linha ao vivo, decaindo de volta — o que a banda fez enquanto você olhava para outro lado
topbar-draw-spectrum-receding-surface-instead = Desenha o espectro como uma superfície que recua, em vez de uma linha chapada: o espectro mais recente na frente, os anteriores fluindo para longe de você. Os últimos segundos da banda como uma paisagem — um portadora que aparece e some é uma crista, e não uma linha que treme. A grade e o peak hold pertencem à linha chapada e não são desenhados nela.
topbar-how-quickly-spectrum-line-follows = Com que rapidez a linha do espectro acompanha a banda. Mais lento faz a média de mais quadros uns com os outros: uma linha mais estável, e uma portadora fraca que fica parada tempo suficiente para ser lida. A cascata não é afetada por isso — essas linhas recebem todos os quadros de qualquer forma.
topbar-show-waterfall = MOSTRAR CASCATA
topbar-draw-scrolling-waterfall-below-spectrum = Desenha a cascata em rolagem abaixo do espectro. Desligada, a linha do espectro ocupa toda a altura.
topbar-how-fast-waterfall-scrolls-lines = Com que rapidez a cascata rola, em linhas por segundo: Lento 5, Médio 28, Rápido 56, Mais rápido 112, Mais rápido ainda 224. O motor cronometra estes sozinho, então os dois mais rápidos são detalhe real, e não a mesma linha desenhada duas vezes — na medida em que o receptor consegue alimentá-los: uma linha nunca pode mostrar mais de uma transformada, e um front end estreito gera apenas algumas dezenas por segundo. Eles custam histórico, já que a cascata guarda um número fixo de linhas — 73 segundos em Médio, 9 em Mais rápido ainda.
topbar-decode-labels = RÓTULOS DE DECODIFICAÇÃO
topbar-mark-every-decoded-station-waterfall = Marca cada estação decodificada na cascata com o indicativo, na frequência em que foi ouvida — FT8, FT4 e os outros modos por slots. Uma boa abertura põe trinta delas pela faixa duas vezes por minuto, sobre os traços que você está lendo; desligue-as e a cascata é só a banda. A lista de decodificações ao lado ainda tem todos os indicativos.
topbar-switch-3d-set = Ligue o 3D para definir isto
topbar-how-3d-spectrum-drawn-only = Como o espectro 3D é desenhado. Só a forma difere — as duas renderizações mostram os mesmos espectros, e ambas ocultam o que está atrás delas, então um sinal forte à frente fica de pé sobre a faixa que cobre.
topbar-lines = LINHAS
topbar-one-trace-per-remembered-spectrum = Um traço por espectro lembrado, na cor da própria linha chapada, cada um ocultando os que estão atrás. A forma sem os níveis, e a leitura a escolher quando a forma é o que se procura — custa um pouco mais que a sólida, em vez de menos, porque desenha a mesma superfície e depois contorna cada crista por cima dela.
topbar-solid = SÓLIDO
topbar-filled-surface-coloured-waterfall-s = Uma superfície preenchida, colorida pela paleta da cascata, para que o nível esteja na cor e também na altura, e as duas metades do panadaptador concordem sobre como é um sinal forte. Mude a paleta em Configurações › Exibição.
topbar-how-fast-3d-spectrum-flows = Com que rapidez o espectro 3D flui para longe de você, em linhas por segundo: Lento 6, Médio 12, Rápido 24, Mais rápido 48. A superfície tem uma profundidade fixa de linhas, então isto também define quanto tempo ela guarda. Mais lento é uma memória maior e uma superfície que rasteja; mais rápido é uma mais curta que se move. A cascata mantém a própria taxa de rolagem, abaixo.
topbar-how-many-columns-panadapter-waterfall = Quantas colunas o panadaptador e a sua cascata são desenhados. Automático lê o renderizador desta máquina e o tamanho do panadaptador e escolhe o máximo que ele pode suportar — uma tela 4K quer 4096. Cada coluna é um byte em cada quadro, então um cliente conectado a uma estação remota paga pelo detalhe no próprio link: 4096 colunas a 60 fps são cerca de um quarto de megabyte por segundo, o dobro da largura padrão.
topbar-read-machine-screen-take-most = Lê esta máquina e esta tela, pega o máximo que elas aguentam e as acompanha se mudarem
topbar-width-force-right-now-whatever = A largura em vigor agora, seja qual for a que a linha acima pede
topbar-keep-floor-ceiling-set-best = Mantém o piso/teto definidos para o melhor contraste da cascata — reajustados suavemente após uma troca de faixa, um deslocamento ou zoom, e quando os níveis derivam. Ligue para ajustar de imediato; desligue para manter os níveis onde você os definiu.
topbar-keep-tuned-frequency-middle-panadapter = Mantém a frequência sintonizada no meio do panadaptador: a janela desliza sob o dial, em vez de a imagem saltar uma faixa inteira quando você sintoniza além da borda. Ligue para centralizar de imediato; desligue para deslocar e dar zoom onde quiser.
topbar-ceil = teto{" "}
topbar-fft-size = TAMANHO DA FFT
topbar-scroll-waterfall-upwards-newest-row = Rola a cascata para cima — linha mais recente embaixo (V)
topbar-logbook-all-qsos-digital-manual = Diário de contatos — todos os QSOs (digitais + manuais)
topbar-award-tracking-dxcc-was-waz = Acompanhamento de prêmios — DXCC / WAS / WAZ / locators
topbar-band-conditions-published-forecast-beside = Condições de faixa — a previsão publicada ao lado do que de fato foi ouvido em cada faixa
topbar-satellite-qo-100-beacon-calibration = Satélite — a calibração pela baliza do QO-100 está rodando
topbar-satellite-doppler-tracking-qo-100 = Satélite — rastreamento Doppler, e a calibração pela baliza do QO-100
topbar-ism-band-devices-decoding-now = Dispositivos de faixa ISM — decodificando agora, esteja esta janela aberta ou não. Desligue-os com DECODIFICANDO dentro da janela.
topbar-ism-band-devices-weather-sensors = Dispositivos de faixa ISM — sensores meteorológicos, medidores e automação residencial ouvidos ao seu redor
topbar-public-sdrs-internet-browse-kiwisdr = SDRs públicos na internet — navegue pelos diretórios KiwiSDR e SpyServer e abra um como rádio
topbar-winlink-radio-email = E-mail por rádio Winlink
topbar-memory-channels = Canais de memória
topbar-scanner-stopped-signal = Scanner — parado em um sinal
topbar-scanner-running = Scanner — em execução
topbar-scan-memory-channels-frequency-range = Varre canais de memória ou uma faixa de frequências
topbar-grid-tracker-worked-maidenhead-squares = Rastreador de locators — quadrados Maidenhead trabalhados num mapa, com o que é ouvido
topbar-settings-device-gains-antennas-audio = Configurações — ganhos dos dispositivos, antenas, dispositivos de áudio
topbar-user-manual-f1 = Manual do usuário (F1)
topbar-range-was-typed-settings-radio = Essa faixa foi digitada em Configurações ▸ Rádio ▸ Faixa de RX, não informada pelo rádio. Amplie-a, ou esvazie a caixa para usar o que o dispositivo informa sobre si mesmo.
topbar-what-device-reports-about-itself = Isto é o que o dispositivo informa sobre si mesmo. Se o rádio cobre esta faixa, informe a faixa real dele em Configurações ▸ Rádio ▸ Faixa de RX.
topbar-band = FAIXA
topbar-mode = MODO

## Top bar: the System box's window chips.

topbar-chip-log = LOG
topbar-chip-awards = PRÊMIOS
topbar-chip-bands = FAIXAS
topbar-chip-sat = SAT
topbar-chip-ism = ISM
topbar-chip-mail = E-MAIL
topbar-chip-mem = MEM
topbar-chip-scan = SCAN
topbar-chip-hfdl = HFDL
topbar-chip-grid = GRID

## Top bar: the mode-defaults chip.

topbar-changed-agc = AGC
topbar-changed-squelch = squelch
topbar-changed-auto-notch = notch automático
topbar-changed-stereo = estéreo
topbar-changed-binaural = binaural
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    Volta aos padrões de { $mode }. Alterados em relação a eles: { $changed }.

    Click to put them back. { $mode }'s own values return, and what you set here is forgotten.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = o rádio não tem configuração de { $baud } baud — { $default } será usado no lugar
settings-radio-icom-scope-hint =
    Transmite a varredura do scope próprio do rádio pelo link CI-V e a desenha como panadaptador — a única imagem da banda que um equipamento de áudio demodulado pode dar, já que o áudio que ele envia é o que já passou pelo filtro dele.

    Needs the radio set up for it: CI-V USB Baud Rate to 115200 and CI-V USB Port to "Unlink from {"["}REMOTE]" (both under SET > Connectors > CI-V), and the baud rate above set to match — below { $min_baud } the sweeps do not fit down the link and the scope stays off.

    The sweeps share the radio's internal USB bus with its own sound card. If received audio starts to drop out with the scope on, this box is the first thing to try turning off.
settings-radio-rx-tx-words = { $rx } / TX { $tx }
settings-radio-soapy-native = O hardware acima é suportado diretamente pelo sdroxide: { $names }. Escolher essa interface acima dá a você as próprias configurações dela — estágios de ganho, filtros e notches que o SoapySDR não consegue expressar — e não precisa de nenhum módulo SoapySDR.
settings-radio-rx888-width-hint = Quanto do espectro digitalizado o panadaptador mostra de uma vez. Toda a cadeia de DSP roda nesta largura, então uma largura maior custa proporcionalmente mais CPU — 1/2 é a banda inteira na cascata, e uma quantidade séria de aritmética. Acima da transição VHF ({ $crossover } MHz) o panadaptador não está olhando para a antena, mas para o F.I. do sintonizador, que tem 8 MHz de largura e fica em { $if_centre } MHz — então qualquer coisa mais larga que { $widest } não pode ser centralizada nele, e a largura extra é espectro morto ao lado do sinal, e não mais dele.
settings-radio-sdrplay-serial-missing = O serial { $serial } não está entre os receptores que o serviço SDRplay informa. Escolha um dos receptores listados, ou reconecte este e pressione Buscar novamente.
settings-radio-hdr-filter-hint =
    O filtro analógico na frente do caminho HDR — um controle diferente da própria largura de banda do receptor. Cada configuração é construída em um conjunto fixo de centros e não pode fazer nada em outros lugares; esta é construída em { $centres }. Vale imediatamente.

    The path itself does not work yet — see the switch above — so this chooses a filter for a mode that is currently silent.
settings-radio-rspduo-only = Somente um RSPduo tem um segundo sintonizador, e o receptor selecionado aqui é um { $model }, então isto não fará nada.
settings-radio-lime-tx-gain-low = O ganho de transmissão está em { $db } dB, no fundo da sua faixa — alguns microwatts saindo da placa, que aparecerão como nada em um medidor de potência, seja o que for que venha depois dela. Este é o padrão para que um transmissor armado não o surpreenda; aumente antes de acionar.

## Settings window, General: SWR

settings-general-swr-tune-limit = (sintonia: { $limit }:1)

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Click to cycle meter face: { $faces }
topbar-step-down = Para baixo { $step }
topbar-step-up = Para cima { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = Deslocamento de repetidora { $shift } — recebendo em { $rx }, transmitindo em { $tx } MHz{ $auto }
topbar-tone-and-burst = Transmitindo { $tone } por baixo da voz, e uma rajada de 1750 Hz de { $ms } ms no início de cada transmissão
topbar-tone-under-voice = Transmitindo { $tone } por baixo da voz
topbar-burst-every-over = Uma rajada de 1750 Hz de { $ms } ms no início de cada transmissão
topbar-match-tx = CASAR TX ({ $tone })
topbar-decimation-on = Decimação no front end: o receptor recebe o centro de 1/{ $n } dos { $msps } Msps que este rádio transmite — { $span } — e o restante é descartado antes que qualquer parte chegue ao receptor.
topbar-decimation-off = Decimação no front end, desligada: o receptor vê os { $span } inteiros que este rádio transmite.
topbar-decimation-hint =
    { $what }

    A narrower span means finer waterfall resolution, a quieter noise floor (3 dB per halving) and less CPU — at the cost of the band either side of it. The dial still tunes anywhere; the radio moves its LO to follow.

    Click to cycle: off / 2 / 4 … { $max }.
topbar-rx-gain-hint = Ganho RX do front end ({ $stage }). Demais corta o ADC do receptor e espalha sinais espúrios pela faixa; de menos, e ele fica surdo.
topbar-rx-gain-stages =
    {""}

    This rig has { $n } RX gain stages — the rest are in Settings → Device.
topbar-passband-now =
    {""}

    The passband is at { $dbfs } dBFS now.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    Silencia o áudio abaixo desta potência na banda de passagem de recepção, em dBFS. À esquerda está aberto.{ $level } Defina-o acima do ruído e abaixo do sinal.

    A stream that arrives with the radio's own AGC already in it — an Icom's 12 kHz IF, for one — sits far higher on this scale than an SDR's raw baseband does, which is why the rail reaches full scale.

    Click to cycle the strength: Low / Med / High / Off. The engine is in the rows below.
topbar-recording-both = Gravando { $audio } e { $iq }
topbar-recording-audio = Gravando áudio em { $audio }
topbar-recording-iq = Gravando I/Q em { $iq }
topbar-auto-record-armed = Gravação automática armada: um arquivo por transmissão, fechado após { $secs } s de silêncio
topbar-rds-hover = RDS: { $ps }. Clique para os dados da estação
topbar-drm-hover = DRM: { $service }. Clique para os detalhes da transmissão
topbar-drm-no-codec = DRM: { $summary } — travado, mas o seu áudio { $codec } não pode ser decodificado aqui. Clique para ver o que falta
topbar-drm-state = DRM: { $summary } — clique para o estado do decodificador
topbar-hd-hover = HD Radio: { $station }. Clique para os detalhes da transmissão
topbar-hd-state = HD Radio: { $summary } — clique para o estado do decodificador
topbar-binaural-preset = { $preset } em cada ouvido — { $lo } … { $hi } Hz, as duas bandas laterais
topbar-nr-hover =
    Redução de ruído: { $strength } — { $engine }

    Click to change engine or strength
topbar-clipping = Cortando { $span } — pressione de novo para recomeçar
topbar-record-clip = Gravar { $span } e parar — um trecho para anexar a um relatório de recepção
topbar-starting-clip = iniciando um trecho de { $span }…
topbar-stops-in = para em { $time }
topbar-auto-record-hint = Grava cada transmissão em seu próprio arquivo, fechado após { $secs } segundos de silêncio
topbar-recording-silence = gravando · { $secs } s de silêncio (fecha em { $hold })
topbar-tone-open = Recebendo { $tone }, que é o squelch de tom — áudio aberto
topbar-tone-closed = Recebendo { $heard }, mas o squelch de tom quer { $wanted } — o áudio continua fechado
topbar-tone-heard = Recebendo CTCSS/DCS { $tone }
topbar-tone-nothing = Squelch de tom { $tone }: nada correspondente a ele está sendo recebido
topbar-tone-use = USAR { $tone }
topbar-sockets = Conectores: { $sockets }
topbar-voice-keyer-tx = Transmitindo { $slot } — clique para abrir o manipulador de voz
topbar-drive-uncal-gen = Esta frequência, que não está em nenhuma faixa de amador, não é calibrada, então a configuração chega ao transmissor integralmente.
topbar-drive-uncal-band = { $band } não é calibrada, então a configuração chega ao transmissor integralmente.
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = Esta frequência, que não está em nenhuma faixa de amador, é calibrada em { $trim } dB, então a mesma configuração põe { $diff } dB { $more ->
        [less] a menos
       *[more] a mais
    } no ar aqui do que em uma faixa não calibrada.
topbar-drive-cal-band = { $band } é calibrada em { $trim } dB, então a mesma configuração põe { $diff } dB { $more ->
        [less] a menos
       *[more] a mais
    } no ar aqui do que em uma faixa não calibrada.
topbar-drive-hover =
    Com que força o transmissor é excitado. Em um rádio que o sdroxide modula por conta própria, ele escala as amostras moduladas; em um equipamento com controle de potência próprio, é a fração da potência nominal que o equipamento recebe.

    { $trim }

    Settings → Radio → Transmit drive by band.
topbar-cessb-hover =
    SSB de envelope controlado: mais potência média para o mesmo pico, sem espalhamento — três ou quatro decibéis de sonoridade aparente na outra ponta. Quantos decibéis a voz é levada ao processador; 0 é desligado, 6 é uma primeira tentativa sensata. Não mexe na Excitação.

    Now: { $now }
topbar-waterfall-rate = { $rows } linhas por segundo — { $seconds } segundos de banda
topbar-detail-auto-columns = { $n } colunas, seja qual for a que o Automático escolheria aqui
topbar-n-columns = { $n } colunas
topbar-fft-bins-whole = { $n } raias em todo o fluxo — uma por coluna, ou mais finas
topbar-fft-bins-pooled = { $n } raias agrupadas nas { $cols } colunas do panadaptador: sinais mais nítidos, não mais deles
topbar-sat-locked = Satélite — travado em { $name }
topbar-filter-edges = { $lo } … { $hi } Hz da portadora
topbar-filter-isb = Filtro de recepção: { $width } Hz em cada ouvido ({ $edges }). As duas bandas laterais do ISB são transmissões separadas, então a largura é a de uma delas.
topbar-filter-width = Filtro de recepção: { $width } Hz de largura ({ $edges }).
topbar-filter-hint =
    { $what }

    Click for the width, the two edges and this mode's presets, as numbers — the panadapter's grips place a passband by eye, this is where an exact figure is typed.
topbar-band-outside-range =
    { $band } está fora do que este rádio recebe ({ $ranges } MHz).

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band }: { $verdict } ({ $time ->
        [day] de dia
       *[night] à noite
    }) — previsão do HAMQSL.com a partir dos índices solares, não uma medição do seu próprio percurso.

## Top bar: short words and captions

topbar-squelch-open = aberto
topbar-off = desligado
topbar-recording = gravando
topbar-sql = sql
topbar-stations = estações
topbar-caption-reaction = reação
topbar-caption-scroll = rolagem
topbar-caption-surface = superfície
topbar-caption-flow = fluxo
topbar-caption-detail = detalhe
topbar-floor = piso

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = AERONAVES
pane-chart = GRÁFICO
pane-chat = CHAT
pane-decodes = DECODIFICAÇÕES
pane-files = ARQUIVOS
pane-heard = OUVIDAS
pane-image = IMAGEM
pane-map = MAPA
pane-messages = MENSAGENS
pane-monitor = MONITOR
pane-panel = PAINEL
pane-qso = QSO
pane-reading = LEITURA
pane-receive = RECEBER
pane-saved = SALVAS
pane-send = ENVIAR
pane-spots = SPOTS
pane-stations = ESTAÇÕES
pane-status = ESTADO
pane-terminal = TERMINAL
pane-text = TEXTO
pane-traffic = TRÁFEGO
pane-vessels = EMBARCAÇÕES
pane-wfall = CASCATA

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}SALVAR{" "}
panels-nothing-decoded-save = Nada decodificado para salvar
panels-clear-rx = {" "}LIMPAR RX{" "}
panels-empty-receive-window-nothing-air = Esvazia a janela de recepção. Nada que está no ar para.
panels-nothing-received-clear = Nada recebido para limpar
panels-save-what-panel-has-decoded = Salva o que este painel decodificou em um arquivo
panels-prop = PROP
panels-shade-map-where-signals-are = Sombreia o mapa conforme onde os sinais de fato estão chegando, em cada faixa. Cada recepção é posicionada no ponto médio do seu percurso — a porção da ionosfera que a curvou — e não na estação distante, então isto é um mapa do céu, e não de onde os radioamadores estão. Construído a partir dos percursos em que esta estação tomou parte, mais — quando a Reverse Beacon Network está ligada em Configurações → Spots — o que os skimmers do mundo estão ouvindo, o que cobre as faixas em que este rádio não está.
panels-night = NOITE
panels-shade-where-sun-down-twilight = Sombreia onde o Sol está abaixo do horizonte, e o crepúsculo entre, para que a linha cinza apareça no mapa. As faixas baixas se alongam e as altas se fecham no lado noturno dela, e o próprio terminador é onde está o DX.
panels-all-bands = TODAS AS FAIXAS
panels-every-band-once-one-hue = Todas as faixas ao mesmo tempo, um tom para cada — condições gerais.
panels-one-band = UMA FAIXA
panels-one-band-cold-hot-blue = Uma faixa, do frio ao quente: azul, verde, amarelo, vermelho.
panels-frequency-you-are-actually-working = A frequência em que você está de fato operando: o dial mais o deslocamento de tom ao lado. É isto que vai para o log — só o dial fica esse tanto abaixo.
panels-send-return = ENVIAR AO RETORNAR
panels-freq = ⇵ FREQ
panels-nothing-agreed-mode-save-your = Nada combinado para este modo — salve a sua abaixo.
panels-forget-saved-frequency = Esquecer esta frequência salva
panels-dial-already-one-your-saved = Este dial já é uma das suas frequências salvas.
panels-dial-already-agreed-list-mode = Este dial já está na lista combinada para este modo.
panels-decode-squelch-raise-stop-decoding = Squelch de decodificação — aumente para parar de decodificar ruído
panels-message-repeats-long-transmit-held = A mensagem se repete enquanto a transmissão estiver pressionada — um meteoro captura a parte dela que estiver passando.
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}TX LIGADO{"  "}
panels-tx-2 = {"   "}TX{"   "}
panels-call-cq = {" "}CHAMAR CQ{" "}
panels-nocall = SEMCHAMADA

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = Ouviu fala de menos — fale normalmente durante toda a contagem e tente de novo.
txaudio-tx-audio = Áudio TX
txaudio-mode-not-voice-mode-so = Este modo não é um modo de voz, então nada que ele envia passa pela cadeia. As configurações feitas aqui valem em SSB, AM e FM.
txaudio-bypass = DESVIAR
txaudio-switch-every-stage-out-once = Desliga todos os estágios de uma vez, inclusive o redutor de ruído, para que o microfone saia sem processamento. Clique de novo para restaurar exatamente os estágios que estavam ligados — a comparação A/B mais rápida do que a cadeia está fazendo.
txaudio-denoise = REDUZIR RUÍDO
txaudio-rnnoise-microphone-ahead-every-other = RNNoise no microfone, antes de todos os outros estágios, para que o ruído do ambiente seja removido antes que o gate ou o compressor possa elevá-lo. Acrescenta cerca de 10 ms de atraso.
txaudio-digital-voice = VOZ DIGITAL
txaudio-also-run-freedv-rade-digital = Também passa a voz digital FreeDV / RADE pela cadeia. Desligado por padrão: um vocoder é treinado com fala natural e pode decodificar pior uma voz muito processada, então teste no ar antes de confiar nisso. O áudio TCI e os modos digitais nunca passam pela cadeia.
txaudio-test = Testar
txaudio-rec = {" "}⏺ GRAVAR{" "}
txaudio-play = {" "}▶ TOCAR{" "}
txaudio-play-test-phrase-through-chain = Toca a frase de teste pela cadeia como está ajustada agora, nos seus alto-falantes. Ajuste enquanto ela toca e ouça a mudança. O áudio de recepção é substituído enquanto ela toca.
txaudio-recording = gravando…
txaudio-no-phrase-yet = nenhuma frase ainda
txaudio-live = Ao vivo
txaudio-live-2 = {" "}🎧 AO VIVO{" "}
txaudio-hear-your-microphone-through-chain = Ouça o seu microfone pela cadeia enquanto fala, sem acionar o transmissor. Use fones: pelos alto-falantes ele realimenta. Você se ouve com alguns décimos de segundo de atraso, o que torna a fala desconfortável — ajuste os níveis nos medidores e julgue o som com GRAVAR / TOCAR. O áudio de recepção é substituído enquanto isso roda; pressionar PTT para.
txaudio-meters = {" "}MEDIDORES{" "}
txaudio-run-your-microphone-through-chain = Passa o seu microfone pela cadeia só para os medidores: nada nos alto-falantes, então não há atraso para falar em cima nem nada para realimentar. É a forma de ajustar o ganho do microfone e de entrada, o limiar do gate e o quanto o compressor trabalha. O áudio de recepção continua; pressionar PTT para.
txaudio-monitoring-live-use-headphones-speakers = Monitorando ao vivo — use fones, ou os alto-falantes vão realimentar o microfone.
txaudio-voice-modes-only-tci-clients = Somente modos de voz. Clientes TCI e os modos digitais contornam a cadeia; quando ela está em uso, mantenha o EQ e o processador de TX do próprio rádio planos.
txaudio-auto-set-stopped-microphone-was = O ajuste automático parou: o microfone foi tomado.
txaudio-microphone-may-clipping-before-gets = {" "}O microfone pode estar cortando antes de chegar aqui — reduza o ganho dele se soar áspero.
txaudio-level = Nível
txaudio-mic-control-same-one-transmit = O controle Mic — o mesmo da barra de transmissão, compartilhado por todos os perfis. 0 dB no meio, +6 dB no topo.
txaudio-mic = Mic
txaudio-gain-top-mic-microphone-stored = Ganho sobre o Mic para este microfone, guardado com o perfil. Os limiares das predefinições supõem uma voz com pico em torno de -10 dBFS no medidor IN; o Ajuste automático o coloca ali. Permanece dentro dele com DESVIAR, e vale também para a voz digital: é nível, não processamento.
txaudio-input = Entrada
txaudio-auto-set = Ajuste automático
txaudio-auto-set-cancelled = Ajuste automático cancelado.
txaudio-talk-now-you-would-air = Fale agora, como faria no ar…
txaudio-db = -∞ dB
txaudio-chain = Cadeia
txaudio-order-audio-meets-them = Na ordem em que o áudio os encontra
txaudio-switch-stage = Ligar este estágio
txaudio-earlier-chain = Antes na cadeia
txaudio-later-chain = Depois na cadeia
txaudio-output-limiter = O limitador de saída
txaudio-output = Saída
txaudio-trim-limiter-always-last = Ajuste e limitador — sempre por último
txaudio-profiles = Perfis…
txaudio-gate-expander = Gate / expansor
txaudio-pushes-room-noise-fan-rumble = Empurra para baixo o ruído do ambiente, o ronco do ventilador e o chiado nas pausas. Defina o limiar logo acima do seu piso de ruído e abaixo das sílabas mais suaves.
txaudio-gate = GATE
txaudio-hard-gate-10-1-down = Gate duro: 10:1, até -40 dB
txaudio-expander = EXPANSOR
txaudio-gentle-expander-2-1-never = Expansor suave: 2:1, nunca mais que -15 dB — o ambiente nunca fica morto
txaudio-reset = REDEFINIR
txaudio-back-defaults = Volta aos padrões
txaudio-threshold = Limiar
txaudio-below-gate-starts-close = Abaixo disto o gate começa a fechar.
txaudio-return = Retorno
txaudio-hysteresis-once-open-closes-only = Histerese: uma vez aberto, ele só fecha este tanto abaixo do limiar, para que uma voz que oscile ali não faça o gate tremer.
txaudio-hold = Retenção
txaudio-how-long-stays-open-after = Por quanto tempo fica aberto depois que a voz cai, para que o fim das palavras não seja cortado.
txaudio-release = Liberação
txaudio-how-fast-closes-once-hold = Com que rapidez fecha quando a retenção termina. Aumente se o gate ficar bombeando.
txaudio-floor = Piso
txaudio-most-ever-turns-pauses-down = O máximo que ele reduz as pausas.
txaudio-ratio = Razão
txaudio-how-steeply-closes-below-threshold = O quão abruptamente fecha abaixo do limiar.
txaudio-look-ahead = Antecipação
txaudio-opens-much-before-word-starts = Abre este tanto antes de a palavra começar, para que a primeira consoante não seja cortada. Acrescenta o mesmo atraso a tudo o que é transmitido.
txaudio-high-pass-low-shelf-two = Passa-altas, shelf de graves, dois picos, shelf de agudos e passa-baixas. Arraste um ponto na curva, ou defina-o abaixo. Ajuste o passa-baixas para a largura do seu filtro de transmissão.
txaudio-flat = PLANO
txaudio-every-band-back-0-db = Todas as faixas de volta a 0 dB, as duas bordas desligadas
txaudio-freq = Freq
txaudio-gain = Ganho
txaudio-q-slope = Q / Inclinação
txaudio-high-pass = Passa-altas
txaudio-removes-handling-noise-breath-pops = Remove ruído de manuseio, estouros de respiração e zumbido abaixo do corte.
txaudio-low-shelf = Shelf de graves
txaudio-body-mud-cut-little-here = Corpo, ou lama: corte um pouco aqui para uma voz que soe oca.
txaudio-peak-1 = Pico 1
txaudio-bump-dip-voice-band-often = Um realce ou corte na banda da voz — frequentemente um corte em torno de 300–500 Hz.
txaudio-peak-2 = Pico 2
txaudio-presence-lift-around-2-3 = Presença: um realce em torno de 2–3 kHz é o que se destaca na SSB.
txaudio-high-shelf = Shelf de agudos
txaudio-brightness = Brilho.
txaudio-low-pass = Passa-baixas
txaudio-stops-energy-above-transmit-filter = Impede que energia acima do filtro de transmissão seja gasta no modulador.
txaudio-t-24-db-oct = 24 dB/oitava
txaudio-t-12-db-oct = 12 dB/oitava
txaudio-click-steeper-gentler-slope = Clique para uma inclinação mais íngreme ou mais suave
txaudio-de-esser = De-esser
txaudio-turns-down-only-sibilant-band = Reduz somente a banda sibilante quando "s" e "sh" ultrapassam o limiar, para que o resto da voz mantenha o nível. Encontre a frequência varrendo-a enquanto diz "sister silver cellophane" e pare onde o chiado for mais forte.
txaudio-frequency = Frequência
txaudio-centre-sibilant-band = Centro da banda sibilante.
txaudio-width-band-higher-narrower = Largura da banda — quanto mais alto, mais estreita.
txaudio-band-level-where-reduction-starts = Nível da banda em que a redução começa. Baixo demais e ela atua o tempo todo, abafando a voz.
txaudio-amount = Quantidade
txaudio-most-band-ever-turned-down = O máximo que a banda é reduzida. Comece em torno de -6 dB.
txaudio-attack = Ataque
txaudio-how-fast-reacts = Com que rapidez reage.
txaudio-how-fast-lets-go = Com que rapidez solta.
txaudio-compressor = Compressor
txaudio-brings-quiet-syllables-up-holds = Traz as sílabas baixas para cima e segura as altas. Mire em 3–6 dB de redução nos picos; mais de 10 dB é compressão evidente, mais adequada a concursos.
txaudio-level-above-which-starts-compress = Nível acima do qual começa a comprimir.
txaudio-how-hard-compresses-above-threshold = O quanto comprime acima do limiar.
txaudio-how-fast-reacts-peak-10 = Com que rapidez reage a um pico. 10–25 ms mantém as consoantes nítidas.
txaudio-how-fast-recovers = Com que rapidez se recupera.
txaudio-knee = Joelho
txaudio-soft-knee-width-0-hard = Largura do joelho suave; 0 é um joelho duro.
txaudio-makeup = Compensação
txaudio-gain-after-compression-bring-level = Ganho depois da compressão, para trazer o nível de volta.
txaudio-drive = Excitação
txaudio-gain-ahead-compressor-so-more = Ganho antes do compressor, para que mais da voz cruze o limiar — eleva a potência média. Use junto com o limitador de saída.
txaudio-phase = Fase
txaudio-all-pass-phase-rotator-ahead = Rotador de fase passa-tudo antes do detector. Equilibra os picos assimétricos que uma voz tem, para que o mesmo nível de pico carregue mais potência média. 4 é uma configuração comum de radiodifusão; 0 é desligado.
txaudio-last-stage-trim-look-ahead = O último estágio: um ajuste, e um limitador com antecipação que mantém todo pico abaixo do teto sem o espalhamento que o corte brusco causa. Desligado por padrão, então ligar a cadeia não altera um nível de excitação que você já tenha definido.
txaudio-limiter = Limitador
txaudio-on = Ligado
txaudio-ceiling = Teto
txaudio-no-peak-leaves-chain-above = Nenhum pico sai da cadeia acima disto.
txaudio-trim = Ajuste
txaudio-output-gain-ahead-limiter = Ganho de saída, antes do limitador.
txaudio-off = {"  "}desligado
txaudio-meters-move-while-you-transmit = Os medidores se movem enquanto você transmite uma voz, toca a frase de teste, ou roda AO VIVO ou MEDIDORES.
txaudio-profiles-2 = Perfis
txaudio-profile-whole-chain-every-stage = Um perfil é a cadeia inteira: as configurações de cada estágio, quais estão ligados, a ordem deles e o ganho de entrada do seu microfone. Carregar um substitui a cadeia; o controle Mic e a excitação permanecem como estão.
txaudio-name = Nome
txaudio-save = Salvar
txaudio-store-chain-now-under-name = Guarda a cadeia como está agora com este nome, substituindo um perfil de mesmo nome
txaudio-import = Importar…
txaudio-read-profile-json-file = Lê um perfil de um arquivo .json
txaudio-tx-audio-profile = Perfil de áudio TX
txaudio-no-profiles-yet-save-stores = Nenhum perfil ainda. Salvar guarda a cadeia que você configurou agora.
txaudio-load = Carregar
txaudio-export = Exportar…
txaudio-delete = Excluir
txaudio-starting-points = Pontos de partida
txaudio-set-your-level-auto-set = Defina o seu nível primeiro com o Ajuste automático — os limiares das predefinições o supõem — e depois ajuste daqui, em vez de transmitir como carregado: grave uma frase e ouça após cada alteração. Uma predefinição mantém o seu ganho de entrada.
txaudio-profile-loaded = Perfil carregado.

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = PROP
solar-where-signals-are-actually-getting =
    Onde os sinais estão de fato chegando, faixa por faixa. Cada recepção em que esta estação toma parte — WSPR, FT8/FT4, JS8 e o diário de contatos — é posicionada no ponto médio do seu percurso, que é a porção da ionosfera que o curvou, e não na estação distante. Assim, isto é um mapa do céu, e não de onde os radioamadores estão.

    It can only show paths this station has been one end of. Oceans light up because the midpoints of long paths fall there; Antarctica stays dark because nobody is transmitting from it. Off by default — it paints the whole planet.
solar-orbits = ÓRBITAS
solar-orbital-paths = Trajetórias orbitais
solar-clouds = NUVENS
solar-cloud-cover-noaa-s-global = Cobertura de nuvens do mosaico geoestacionário global da NOAA, desenhada como profundidade de ar em vez de uma imagem colada na superfície: o infravermelho diz quão fria é cada topo de nuvem, e é assim que se sabe a altura dela. Tempestades relampejam dentro das torres que constroem.
solar-planets = PLANETAS
solar-other-seven-planets-eighteen-their = Os outros sete planetas, dezoito das suas luas, e os anéis de Saturno e Urano. As posições planetárias vêm do conjunto de elementos kepleriano do JPL; as luas são órbitas circulares ajustadas ao JPL Horizons.
solar-coronal-mass-ejection-trajectory-cones = Cones de trajetória de ejeção de massa coronal
solar-sun-obs = OBS SOL
solar-solar-observations-sun-s-disk = Observações solares no disco do Sol: regiões ativas de manchas solares, e de onde vieram os flares
solar-labels = RÓTULOS
solar-body-region-labels = Rótulos de corpos e regiões
solar-small-bodies = CORPOS PEQUENOS
solar-names-asteroids-comets-they-are = Nomes nos asteroides e cometas. Eles são desenhados de qualquer forma — isto é apenas se as trinta e cinco designações são desenhadas junto. Qualquer que seja a configuração, o corpo em que a câmera está e o que a caixa de busca encontrou continuam nomeados.
solar-decoded-ft8-ft4-stations-path = Estações FT8/FT4 decodificadas e o percurso até a estação em QSO
solar-sats = SATS
solar-amateur-radio-satellites-their-orbits = Satélites de radioamador, suas órbitas e elevação a partir do seu QTH
solar-aurora = AURORA
solar-auroral-oval-noaa-s-ovation = O oval auroral do modelo OVATION da NOAA, desenhado como cascas de emissão em suas altitudes reais, com a borda voltada ao equador do contorno de 10 % marcada na superfície
solar-awards = PRÊMIOS
solar-award-coverage-every-dxcc-entity = Cobertura de prêmios: cada entidade DXCC pintada conforme o que o seu diário ainda não tem. Laranja queima onde você nunca trabalhou a entidade, âmbar onde trabalhou mas não está confirmada, verde-escuro onde um QSL já voltou. Acompanha o filtro de faixa da janela PRÊMIOS.
solar-view = VISÃO
solar-which-body-camera-orbits-animated = Em torno de qual corpo a câmera orbita, e o passeio animado
solar-layers = CAMADAS
solar-what-drawn-orbits-clouds-cmes = O que é desenhado: órbitas, nuvens, CMEs, rótulos, QSOs, aurora
solar-which-sdo-channel-wraps-sun = Qual canal do SDO envolve o Sol, e quão antiga é a imagem
solar-weather = TEMPO
solar-aurora-propagation-numbers-which-bands = A aurora, os números de propagação e quais faixas estão abertas
solar-space-weather = CLIMA ESPACIAL
solar-nothing-measured-yet = nada medido ainda
solar-scale = ESCALA
solar-body-moon-orbit-exaggeration = Exagero do corpo e da órbita da Lua
solar-time = TEMPO
solar-scrub-whole-scene-forward-back = Percorre a cena inteira para frente e para trás
solar-activity = ATIVIDADE
solar-replay-last-hour-decodes-globe = Reproduz a última hora de decodificações no globo
solar-what-propagation-heat-map-showing = O que o mapa de calor de propagação está mostrando
solar-propagation = PROPAGAÇÃO
solar-auto = ▶ AUTO
solar-fly-spline-through-set-framed = Percorre uma spline por um conjunto de pontos de vista enquadrados. Qualquer entrada do mouse cancela.
solar-moons-ride-circular-orbits-fitted = As luas seguem órbitas circulares ajustadas ao JPL Horizons: a um ou dois graus de onde realmente estão, e até seis para Miranda, cujo plano orbital oscila rápido demais para um círculo acompanhar.
solar-fetch-everything-again-now = Buscar tudo de novo agora
solar-all-sats = TODOS OS SATS
solar-show-every-satellite-element-set = Mostra todo satélite do conjunto de elementos, e não só os populares. Os anéis de órbita ficam nos poucos curados — noventa de uma vez é ilegível.
solar-starting = iniciando…
solar-body = corpo{" "}
solar-moon-orbit = órbita da lua{" "}
solar-stretch-earth-moon-distance-so = Estica a distância Terra→Lua para que o par possa ser visto separado
solar-sun = sol{" "}
solar-sun-radius-exaggeration-leave-1 = Exagero do raio do Sol. Deixe em 1× para manter a geometria dos CMEs legível — um Sol inchado engole a base de cada cone.
solar-show = MOSTRAR
solar-paint-globe-what-getting-through = Pinta o globo pelo que está chegando
solar-paints-globe-what-getting-through = Pinta o globo conforme o que está chegando, em cada faixa. Cada recepção é posicionada no ponto médio do seu percurso — onde a ionosfera fez o trabalho — e não na estação distante. Construído a partir do que esta estação ouve, mais os skimmers do mundo quando a Reverse Beacon Network está ligada.
solar-display = EXIBIÇÃO
solar-all-bands = TODAS AS FAIXAS
solar-every-band-once-one-hue = Todas as faixas ao mesmo tempo, um tom para cada, misturando onde duas se sobrepõem. A visão para “como estão as condições” em vez de “os 20 m estão abertos”.
solar-one-band = UMA FAIXA
solar-one-band-cold-hot-blue = Uma faixa, do frio ao quente: azul, verde, amarelo, vermelho.
solar-band = FAIXA
solar-nothing-heard-yet = nada ouvido ainda
solar-bands = FAIXAS
solar-sources = FONTES
solar-contacts-logbook-path-was-open = Contatos no diário. Um percurso que estava aberto, sem relatório de sinal que valha o nome — um RST não é um SNR, então estes contam para o quão movimentada está uma célula e nunca para a sua margem.
solar-stations-reported-hearing-one-downloaded = Estações que informaram ter ouvido esta, baixadas do WSPRnet. O único retorno que um balizador transmissor recebe.
solar-reverse-beacon-network-skimmers-what = Skimmers da Reverse Beacon Network: o que todos os outros estão ouvindo, em cada faixa, inclusive as em que este rádio não está. Mais grosseiros que o resto — uma linha do RBN não traz locators, então as duas pontas são posicionadas no centro do país, o que é próximo para um país pequeno e muito errado para um grande. Ligue em Configurações → Spots.
solar-decodes-station-made-each-measured = Decodificações feitas por esta estação. Cada uma é medida contra o piso do próprio modo, para que um relatório WSPR e um FT4 sejam comparáveis.
solar-memory = MEMÓRIA
solar-half-life = meia-vida{" "}
solar-how-long-reception-takes-count = Quanto tempo uma recepção leva para contar metade do valor. A memória da própria ionosfera é curta — uma abertura de duas horas não deveria discutir com uma de dois minutos atrás.
solar-live = AO VIVO
solar-follow-band-happens = Acompanha a faixa à medida que acontece
solar-replay = ⏸ REPRODUZIR
solar-replay-2 = ▶ REPRODUZIR
solar-replay-last-hour-decodes-over = Reproduz a última hora de decodificações, repetidamente
solar-min-ago = {" "}min atrás
solar-where-last-hour-globe-showing = Onde, na última hora, o globo está mostrando
solar-trail = rastro{" "}
solar-how-long-decode-s-arc = Por quanto tempo o arco de uma decodificação fica no globo atrás da cabeça
solar-speed = velocidade{" "}
solar-how-much-faster-than-real = Quanto mais rápido que o tempo real a reprodução roda
solar-qso-layer-off = Camada de QSO desligada
solar-no-decodes-yet = nenhuma decodificação ainda
solar-qth-not-set-enter-your = QTH não definido — informe seu quadrado locator em Configurações
solar-mode = MODO
solar-path = PERCURSO
solar-sent = ENVIADO
solar-rcvd = RECEBIDO
solar-grid = LOCATOR
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = Geoestacionário — sempre acima do seu horizonte.
solar-no-passes-next-48-hours = Nenhuma passagem nas próximas 48 horas a partir do seu QTH.
solar-start = INÍCIO
solar-max-el = EL MÁX
solar-aos-los-are-azimuths-horizon = AOS/LOS são azimutes no horizonte. Os horários são UTC.
solar-locked-unlock = {" "}● TRAVADO — DESTRAVAR{" "}
solar-release-satellite-lock = Libera o travamento no satélite
solar-lock = {" "}TRAVAR{" "}
solar-track-satellite-main-window-doppler = Rastreia este satélite na janela principal: RX e TX com correção Doppler, uplink mapeado no transponder, comando do rotor
solar-no-frequencies-file-one-add = Não há frequências cadastradas para este — adicione-as em Configurações ▸ TLE.
solar-frequencies-yours = FREQUÊNCIAS · SUAS
solar-frequencies = FREQUÊNCIAS
solar-link = ENLACE
solar-downlink-mhz = DESCIDA (MHz)
solar-uplink-mhz = SUBIDA (MHz)
solar-doppler-shifts-these-few-khz = O Doppler os desloca em alguns kHz durante uma passagem em órbita baixa.
solar-satellite-planet-comet = satélite, planeta ou cometa
solar-planet-asteroid-comet = planeta, asteroide ou cometa
solar-clear-search = Limpar a busca
solar-no-match = nenhum resultado
solar-no-sounder = sem sondador
solar-heard = OUVIDO ≥
solar-kp = Kp / A
solar-x-ray = Raios X
solar-observed-above-sounder-better-than = observado pelo sondador — melhor que o modelado
solar-power-n-s = potência N/S
solar-edge-n-s = borda N/S
solar-kp-peak-24-h = pico de Kp em 24 h
solar-observed-forecast = observado | previsto
solar-bands-open = FAIXAS ABERTAS
solar-dxcc-coverage = COBERTURA DXCC
solar-ir-vis = IV+VIS
solar-ir-only = só IV
solar-no-deep-convection = sem convecção profunda
solar-t-1-storm = 1 tempestade
solar-glancing = {" "}· tangencial
solar-direction-estimated = {" "}· direção estimada

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = Configuração APRS
setup-packet-setup = Configuração de pacote
setup-ft8-ft4-ft2-setup = Configuração FT8 / FT4 / FT2
setup-my-callsign = Meu indicativo
setup-my-grid = Meu locator
setup-station-call = Indicativo da estação
setup-ssid-oe3jjs-10-packet-station = Com um SSID — OE3JJS-10. Uma estação de pacote usa, por convenção, um SSID diferente do operador, e nada transmite até que isto esteja definido.
setup-speed = Velocidade
setup-t-9600-needs-radio-s-data = 9600 exige a porta de dados do rádio. Um caminho de microfone e alto-falante a destrói nas duas pontas — isso é do rádio, não do sdroxide.
setup-tx-delay = Atraso de TX
setup-flags-sent-before-frame-so = Flags enviadas antes de um quadro, para que a outra ponta possa nos ouvir e travar o clock dela. Em um equipamento CAT, só o sdroxide gasta de 165 a 240 ms para entrar no ar, e o tempo de prontidão de transmissão do próprio equipamento soma-se a isso.
setup-packet-length = Tamanho do pacote
setup-bytes = {" "}bytes
setup-most-single-frame-carries-shorter = O máximo que um único quadro carrega. Quadros menores sobrevivem a um caminho marginal — só o quadro atingido é reenviado — e custam mais sobrecarga. 128 em HF, 256 onde o caminho é sólido.
setup-frames-sent-before-waiting-acknowledgement = Quadros enviados antes de esperar uma confirmação. Mais enche um bom caminho; em um ruim, é mais para reenviar.
setup-extended = Estendido
setup-ask-mod-128 = Pedir mod-128
setup-extended-sequence-numbers-window-bigger = Números de sequência estendidos, para uma janela maior que sete. Muitos nós recusam o pedido com um DM, que se parece exatamente com uma estação que não quer falar com você — então deixe desligado, a menos que saiba que a outra ponta o quer.
setup-answer-calls = Responder chamadas
setup-accept-connections = Aceitar conexões
setup-off-winlink-client-which-dials = Desligado para um cliente Winlink, que disca para fora e não tem motivo para responder. Ligado para ser alcançável como par: uma chamada chega no painel de terminal e você conversa com quem a fez. Uma estação cujo link já está ocupado com uma sessão Winlink recusa chamadas até que ela termine.
setup-connect-text = Texto de conexão
setup-what-caller-greeted = com o que quem chama é recebido
setup-sent-station-connects-us-once = Enviado a uma estação que se conecta a nós, quando o link está ativo. Vazio não envia nada — o que parece defeito para quem chamou.
setup-default-via = Via padrão
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = O caminho de digipeaters com que o terminal começa. O caminho até o seu nó local é sempre o mesmo, e redigitá-lo é como um salto acaba esquecido.
setup-beacon = Baliza
setup-t-0-disables-timer = 0 desativa o temporizador
setup-beacon-text = texto da baliza
setup-kiss-server = Servidor KISS
setup-serve = Servir
setup-offer-modem-kiss-tnc-socket = Oferece este modem como um TNC KISS em um socket, para que o Pat, um cliente APRS ou a pilha AX.25 do Linux possam usar o rádio.
setup-aprs-call = Indicativo APRS
setup-callsign-station-beacons-under-ssid = O indicativo sob o qual esta estação transmite a baliza, com o seu SSID — OE3JJS-9 para um carro, -10 para um I-gate, -5 para um celular. Deixe vazio para usar o indicativo acima.
setup-no-callsign-station-will-not = Sem indicativo: esta estação não transmitirá nada — nem baliza, nem mensagem, nem confirmação.
setup-symbol = Símbolo
setup-two-characters-protocol-carries-them = Os dois caracteres como o protocolo os carrega: uma tabela (`/` ou `\`) e um código de símbolo. Um dígito ou uma letra na primeira posição é uma sobreposição, desenhada por cima do ícone da tabela alternativa.
setup-path = Caminho
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = Até onde você pede para ser repetido. Esta é a configuração mais consequente do canal: cada salto multiplica as transmissões que a rede inteira faz em uma frequência compartilhada.
setup-position = Posição
setup-my-grid-2 = A partir do meu locator
setup-beacon-centre-locator-above-reported = Transmite a baliza do centro do locator acima, informado com a ambiguidade que um locator de fato tem — um de seis caracteres tem alguns quilômetros de largura, e dizer isso é honesto. Desmarque para informar coordenadas exatas.
setup-latitude = Latitude
setup-longitude = Longitude
setup-locator = a partir do locator
setup-not-coordinate = não é uma coordenada
setup-comment = Comentário
setup-sent-every-beacon = enviado em toda baliza
setup-t-0-default-never-beacons-timer = 0 — o padrão — nunca transmite a baliza por temporizador. Trinta minutos é a convenção para uma estação fixa; uma estação em movimento transmite com mais frequência, mas cada baliza é tempo de canal de outra pessoa.
setup-compressed = Comprimido
setup-compressed-position-format-third-air = O formato de posição comprimido: um terço do tempo no ar e mais preciso. Todo receptor desde os anos 1990 o lê.
setup-first-goes-out-one-interval = A primeira sai um intervalo a partir de agora, não imediatamente.
setup-messages = Mensagens
setup-acknowledge = Confirmar
setup-answer-messages-addressed-you-acknowledgement = Responde às mensagens endereçadas a você. Uma confirmação é uma transmissão que esta estação faz sem ser solicitada, então uma configuração só de recepção deve desligá-la — e a baliza então deixa de afirmar que também é alcançável.
setup-flags-sent-ahead-every-frame = Flags enviadas antes de cada quadro, para que o modem da outra ponta ouça a portadora e trave o clock antes de os dados começarem. Precisa durar mais que tudo entre apertar transmitir e irradiar: só o sdroxide gasta nisso de 165 a 240 ms em um equipamento CAT, e um rádio que recebe o áudio pela rede armazena mais por cima. Pouco demais e a outra ponta nunca trava — a transmissão está lá e nada a decodifica.
setup-shared-packet-mode = compartilhado com o modo pacote
setup-keep-stations = Manter estações
setup-how-long-station-stays-map = Por quanto tempo uma estação permanece no mapa depois de ter sido ouvida pela última vez. Também é contra isto que o desvanecimento do mapa é medido, então uma janela curta mostra só o que está ao vivo.
setup-auto-reply = Resposta automática
setup-answer-snr-grid-status = Responder SNR? / LOCATOR? / STATUS?
setup-answer-direct-question-addressed-you = Responde a uma pergunta direta endereçada a você ou a @ALLCALL, com a resposta em vez de uma confirmação. Nunca responde ao tráfego de outra estação, e nunca responde a si mesma.
setup-heartbeat = Heartbeat
setup-off = Desligado
setup-turbo-does-not-beacon-local = O Turbo não transmite baliza — é a velocidade local e de VHF.
setup-sends-your-callsign-grid-so = Envia o seu indicativo e locator para que outros saibam que você pode ser recebido. A primeira sai um intervalo a partir de agora, não imediatamente.
setup-beacon-frequency = Frequência da baliza
setup-t-500-1000-hz = 500–1000 Hz
setup-move-each-beacon-free-slot = Move cada baliza para um slot livre na sub-faixa de heartbeat, como o JS8Call faz: é onde as estações que monitoram balizas procuram, e mantém um transmissor sem supervisão fora do QSO de outra pessoa. O slot é escolhido quando a baliza realmente sai, livre de tudo o que está sendo decodificado.
setup-working-freq = Frequência de trabalho
setup-beacon-where-you-are-working = Transmite a baliza onde você está trabalhando. Contra a convenção da faixa, mas mantém tudo o que você transmite em um só lugar.
setup-heartbeat-reply = Resposta ao heartbeat
setup-answer-heartbeats-signal-report = Responder aos heartbeats com um relatório de sinal
setup-tell-station-beaconed-how-well = Diz a uma estação que transmitiu a baliza o quão bem você a copiou. Desligado por padrão: uma faixa movimentada traz um heartbeat a cada slot, e responder a todos inundaria justamente a faixa que os heartbeats existem para manter silenciosa. Limitado a uma resposta por estação a cada 15 minutos, e nunca enquanto uma mensagem ainda está chegando ou enquanto você tem algo na fila para enviar.
setup-status-message = Mensagem de estado
setup-tx-period = Período de TX
setup-even = Par
setup-odd = Ímpar
setup-auto-sequence = Sequência automática
setup-decode-depth = Profundidade de decodificação
setup-fast = Rápido
setup-normal = Normal
setup-deep = Profundo

    Fast — one pass, no signal subtraction: quickest, fewest decodes.
    Normal — flat multi-pass subtraction: a little quicker than Deep, a little less thorough.
    Deep — the checkpointed multi-pass: the most decodes, and ~1.2 s on a busy slot.
setup-auto-tx-frequency = Frequência de TX automática
setup-choose-transmit-frequency-automatically-quietest = Escolhe a frequência de transmissão automaticamente: o ponto mais silencioso no período em que você transmite, em vez da frequência da estação que você está respondendo. Desligado NÃO a mantém — ele responde na frequência da estação chamada. Para mantê-la, use Manter frequência de TX.
setup-hold-tx-frequency = Manter frequência de TX
setup-pin-transmit-tone-where-nothing = Fixa o tom de transmissão onde está. Nada o move: não responder a uma estação, nem a fila de chamadas, nem chamar CQ, nem um clique em uma decodificação ou na cascata. Tem precedência sobre a Frequência de TX automática. Trocar de faixa é a única exceção: o deslocamento que você definiu por último na nova faixa volta com ela. Para quando a sua licença é mais estreita que o plano de faixas — em um dial britânico de 60 m, em 5357 kHz, a alocação termina em 5358,0, então o tom precisa ficar abaixo de 1000 Hz.
setup-tx-watchdog = Vigia de TX
setup-stop-transmitting-after-long-no = Para de transmitir depois deste tempo sem resposta e sem ação sua. 0 desativa.
setup-give-up-after = Desistir após
setup-calls = {" "}chamadas
setup-contest = Concurso
setup-everyday-exchange-grid-report-rr73 = A troca do dia a dia: locator, relatório, RR73.
setup-european-vhf-contests-send-signal = Concursos europeus de VHF: envia um relatório de sinal, um número de série e o seu locator de SEIS caracteres, no layout de mensagem que o WSJT-X chama de EU VHF Contest. O CQ sai como CQ TEST.
setup-serial = Série
setup-serial-number-next-exchange-will = O número de série que a próxima troca levará. Ele avança sozinho à medida que cada contato é registrado e é lembrado entre sessões — defina-o à mão só para começar um concurso, ou para retomar de onde um diário de papel parou.
setup-needs-6-character-grid = ⚠ exige um locator de 6 caracteres
setup-eu-vhf-contest-messages-carry = As mensagens do EU VHF Contest levam um locator de seis caracteres (JN88DD, não JN88). Até que o Meu locator tenha um, a troca comum é enviada no lugar.
setup-dxpedition = DXpedition
setup-ordinary-ft8-operation = Operação FT8 comum.
setup-calling-dxpedition-running-fox-mode = Chamando uma DXpedition em modo Fox: chame de acima de 1000 Hz, desça para o Fox quando ele responder, e registre-o no RR73 dele sem enviar 73.
setup-run-pile-up-several-signals = Trabalha o pile-up: vários sinais ao mesmo tempo, uma fila de chamadores, atendidos primeiro os mais fortes e mais raros. CHAMAR CQ o inicia, PARAR QSO o encerra.
setup-fox-signals = Sinais Fox
setup-once = {" "}de uma vez
setup-simultaneous-transmissions-spaced-60-hz = Transmissões simultâneas, espaçadas 60 Hz umas das outras. Elas dividem a potência do transmissor, então mais sinais significa que cada um é mais fraco.
setup-tx-audio = Áudio TX
setup-deviation-fm = desvio, em FM
setup-drive-ssb = excitação, em SSB
setup-grid = Locator
setup-report = Relatório
setup-r-report = R+Relatório

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = , escala de 10 W, redimensionada
amp-t-10-w-scale = , escala de 10 W
amp-t-2-w-scale-rescaled = , escala de 2 W, redimensionada
amp-t-2-w-scale = , escala de 2 W
amp-t-100-w-scale-rescaled = , escala de 100 W, redimensionada
amp-t-100-w-scale = , escala de 100 W
amp-t-20-w-scale-rescaled = , escala de 20 W, redimensionada
amp-t-20-w-scale = , escala de 20 W
amp-rescaled = , redimensionada
amp-and = {" "}E{" "}
amp-not-while-transmitting-switching-relay = Não durante a transmissão: comutar um relé com RF presente pode danificá-lo. Solte a tecla antes.
amp-no-amplifier-set-up-choose = Nenhum amplificador configurado. Escolha a porta dele em Configurações → Amplificador.
amp-settings = CONFIGURAÇÕES
amp-dock = ACOPLAR
amp-put-amp-window-back-inside = Recolocar a janela AMP dentro da janela principal
amp-pop-out = DESTACAR
amp-give-amp-window-window-own = Dar à janela AMP uma janela própria, para movê-la a outra tela
amp-rig = , do equipamento
amp-rig-flagged-reading = , o equipamento sinalizou a leitura
amp-drv = , de DRV
amp-rig-s-swr = , da ROE do equipamento
amp-ptt-keys-amplifier-radio-off = O PTT aciona o amplificador junto com o rádio; DESLIGADO o deixa em desvio. Clique para trocar.
amp-switch-antenna-jack = Trocar o conector de antena
amp-band-down = Faixa abaixo
amp-band-up = Faixa acima
amp-tuner = SINTONIZADOR
amp-tuned = Sintonizado
amp-tune = SINTONIZAR
amp-start-tune-then-key-steady = Inicie uma sintonia — depois acione uma portadora contínua (CW, FM ou RTTY). Pressione de novo para parar.
amp-put-tuner-line-bypass = Colocar o sintonizador na linha, ou desviá-lo
amp-tuning = Sintonizando…
amp-bottom = INFERIOR
amp-pick-any-reading-either-needle = Escolha qualquer leitura para qualquer um dos ponteiros. O ponteiro superior é branco, o inferior azul.
amp-forward-power-measured-radio-amplifier = Potência direta, medida pelo rádio: o amplificador não informa nenhuma em DESLIGADO
amp-forward-power-taken-drv-amplifier = Potência direta, obtida de DRV: o amplificador não informa nenhuma em DESLIGADO, e em desvio a potência do rádio passa direto
amp-forward-power = Potência direta
amp-reflected-power-measured-radio-amplifier = Potência refletida, medida pelo rádio: o amplificador não informa nenhuma em DESLIGADO
amp-reflected-power-worked-out-fwd = Potência refletida, calculada a partir de FWD e da ROE: o amplificador não informa nenhuma em DESLIGADO
amp-reflected-power = Potência refletida
amp-standing-wave-ratio-red-past = Relação de onda estacionária: vermelho no limite de sintonia ou acima (Configurações → Geral). Verifique a antena
amp-standing-wave-ratio-measured-radio = Relação de onda estacionária, medida pelo rádio: o amplificador não informa nenhuma em DESLIGADO
amp-standing-wave-ratio = Relação de onda estacionária
amp-drive-radio = Excitação do rádio
amp-drain-supply = Tensão de dreno
amp-drain-current = Corrente de dreno
amp-temp = TEMP
amp-heat-spreader-temperature-yellow-49 = Temperatura do dissipador: amarelo a partir de 49 °C (120 °F), vermelho acima de 65 °C (149 °F), fora da linha acima de 70 °C (158 °F). Clique para °C/°F.
amp-c = °C
amp-f = °F
amp-performance = DESEMPENHO
amp-numbers-hold-peaks-bars-are = os números mantêm os picos · as barras são ao vivo
amp-input-power = POTÊNCIA DE ENTRADA
amp-supply-volts-drain-amps = Tensão de alimentação × corrente de dreno
amp-efficiency = EFICIÊNCIA
amp-output-power-input-power = Potência de saída ÷ potência de entrada
amp-gain = GANHO
amp-output-power-over-drive-decibels = Potência de saída sobre a excitação, em decibéis
amp-off-line-over-temperature = FORA DA LINHA · temperatura excessiva
amp-connected-transmitting = CONECTADO · TRANSMITINDO
amp-connected = CONECTADO
amp-connecting = Conectando…
amp-atu-bypassed = ATU DESVIADO
amp-no-inductance-needed = não precisa de indutância
amp-no-capacitance-needed = não precisa de capacitância
amp-c-out-antenna-side = C SAÍDA · lado da antena
amp-c-transmitter-side = C ENTRADA · lado do transmissor

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = Blocos recebidos (acesos) e ainda faltando (apagados)
sstv-sstv = SSTV
sstv-auto = Automático
sstv-signal = Sinal
sstv-station-sending-mode-sdroxide-does = Uma estação está transmitindo em um modo que o sdroxide não tem. O sinal e a sintonia estão bons — simplesmente não há decodificador para este. Nada a corrigir do seu lado.
sstv-listening = escutando…
sstv-restart-rx = Reiniciar RX
sstv-abandon-picture-being-received-listen = Abandona a imagem que está sendo recebida e escuta o próximo cabeçalho. Para uma transmissão que começou a decodificar no modo errado — o receptor fica comprometido com ela até esse modo acabar.
sstv-callsign-last-station-sent-fsk = O indicativo que a última estação enviou como identificação FSK depois da imagem.
sstv-tx-slant = Inclinação de TX
sstv-transmit-clock-trim-ppm-remove = Ajuste do clock de transmissão (ppm) para remover a inclinação no decodificador da outra ponta
sstv-reset-0-ppm = Redefinir para 0 ppm
sstv-fsk-id = ID FSK
sstv-send-your-callsign-tones-after = Envia o seu indicativo em tons depois de cada imagem — a identificação que os repetidores SSTV e outros programas leem. Acrescenta cerca de 2,5 segundos, e não envia nada até que você tenha definido um indicativo.
sstv-tx-lead = Antecedência de TX
sstv-silence-sent-after-keying-before = Silêncio enviado depois de acionar e antes do líder da imagem e do código VIS. Um decodificador que perde qualquer parte desse cabeçalho não mostra imagem nenhuma, então isto cobre o intervalo entre pedir o PTT a um equipamento e ele estar de fato no ar. 0 para um SDR que aciona instantaneamente.
sstv-live = AO VIVO
sstv-waiting-picture = aguardando uma imagem…
sstv-no-low-audio = sem áudio / áudio baixo
sstv-received = RECEBIDAS
sstv-reading-radio-s-pictures = Lendo as imagens do rádio…
sstv-decoded-pictures-collect-here = As imagens decodificadas se acumulam aqui.
sstv-click-enlarge-right-click-delete = Clique para ampliar · clique com o botão direito para excluir
sstv-delete-picture = Excluir esta imagem
sstv-loading = carregando…
sstv-transmit = TRANSMITIR
sstv-click-edit-slot-s-message = Clique para editar a mensagem deste slot · clique duplo para carregar uma imagem
sstv-change-image = Trocar imagem…
sstv-load-image = Carregar imagem…
sstv-clear = Limpar
sstv-empty-slot-s-picture-message = Esvaziar a imagem deste slot (a mensagem permanece)
sstv-banner = Faixa…
sstv-what-printed-across-top-every = O que é impresso no topo de cada imagem que esta estação envia
sstv-fetching-slot-s-picture = Buscando a imagem deste slot…
sstv-load-image-into-slot = Carregar uma imagem neste slot →
sstv-drawn-slot-s-image = Desenhado sobre a imagem deste slot
sstv-abort-tx = {" "}CANCELAR TX{" "}
sstv-delete-picture-store = Excluir esta imagem do armazenamento
sstv-received-image = Imagem recebida
sstv-no-longer-store = não está mais no armazenamento
sstv-loading-full-size = carregando em tamanho real…
sstv-save-picture = Salvar imagem…
sstv-delete-sure = Excluir — tem certeza?
sstv-click-again-delete-good = Clique de novo para excluir definitivamente
sstv-delete = Excluir…
sstv-image-banner = Faixa da imagem
sstv-drawn-into-every-picture-station = Desenhada em cada imagem que esta estação transmite, sobre a mensagem do próprio slot.
sstv-draw-banner = Desenhar a faixa
sstv-substituted-when-picture-composed = Substituído quando a imagem é composta:
sstv-top-left = Superior esquerdo
sstv-top-right = Superior direito
sstv-colours = Cores
sstv-strip-top-edge-fades-black = A faixa, na sua borda superior — esmaece até o preto, ou até a cor do gradiente abaixo dela
sstv-both-texts = Ambos os textos
sstv-gradient = Gradiente
sstv-fade = esmaecer até
sstv-fade-strip-top-colour-second = Esmaece a faixa da sua cor superior até uma segunda cor, em vez de até o preto.
sstv-text-outline = Contorno do texto
sstv-draw-outline-around-banner-text = Desenha um contorno em torno do texto da faixa, para que ele continue legível sobre uma faixa movimentada ou clara.
sstv-text-gradient = Gradiente do texto
sstv-fade-banner-text-colour-second = Esmaece o texto da faixa da sua cor até uma segunda cor, através da largura da imagem.
sstv-rainbow-text = Texto arco-íris
sstv-all-text-overriding-colours-above = todo o texto, substituindo as cores acima
sstv-draw-every-text-picture-banner = Desenha todo texto da imagem — faixa e mensagem — como um arco-íris horizontal, ignorando as cores acima.
sstv-message = Mensagem
sstv-slot-message-s-text-colour = A cor do texto da mensagem do slot
sstv-outline = contorno em
sstv-height = Altura
sstv-how-tall-strip-pixels-transmitted = Quão alta é a faixa, em pixels da imagem transmitida. O texto é dimensionado a partir dela — um quadro SSTV tem só 320 pixels de largura, então uma faixa mais alta é o que a torna legível na outra ponta.
sstv-reset = Redefinir
sstv-back-callsign-left-version-right = Volta ao indicativo à esquerda e à versão à direita
sstv-waiting-station-s-digital-mode = Aguardando carregar as configurações de modo digital da estação…
sstv-rifp = RIFP
sstv-size = Tamanho
sstv-calling-frequency-draft-names = A frequência de chamada que o rascunho indica
sstv-encode = Codificar
sstv-how-picture-encoded-into-object = Como a imagem é codificada no objeto que o RIFP carrega. Automático tenta cada uma e envia a menor.
sstv-try-every-encoding-send-smallest = Tenta cada codificação, envia a menor (nunca com perdas)
sstv-gray = Cinza
sstv-grayscale-depth-rifp-s-raster = Profundidade de escala de cinza. A matriz do RIFP é em escala de cinza por definição — cor não tem lugar no manifesto dele.
sstv-dither = Dither
sstv-diffuse-quantisation-error-worth-below = Difunde o erro de quantização — vale a pena abaixo de 8 bits
sstv-caption = Legenda
sstv-sent-manifest-content-hint-shown = Enviada no manifesto como dica de conteúdo, e mostrada sob a imagem pelos receptores. Viaja como texto — não é desenhada na imagem.
sstv-what-picture = O que esta imagem é
sstv-repeat-data = Repetir dados
sstv-send-every-data-frame-many = Envia cada quadro de dados essa quantidade de vezes. O RIFP é unidirecional, sem pedidos de reparo, então esta é a única recuperação que um receptor tem.
sstv-chunk = Bloco
sstv-payload-octets-per-data-frame = Octetos de carga por quadro de dados (o perfil recomenda 192)
sstv-valid-frames-frames-failed-crc = Quadros válidos, quadros que falharam no CRC, e imagens completas verificadas
sstv-no-transfer-progress = nenhuma transferência em andamento
sstv-forget-incomplete-transfer = Esquecer esta transferência incompleta

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = Recuperar
windows-delete = Excluir
windows-edit-name-frequency-mode-antenna = Edita o nome, a frequência, o modo, a antena e a configuração de repetidora
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = O deslocamento de repetidora, o tom e a rajada de 1750 Hz guardados com este canal
windows-abandon-edit-escape = Descartar a edição (Escape)
windows-save = SALVAR
windows-keep-changes-enter = Manter as alterações (Enter)
windows-recall-channel-without-moving-antenna = Recupera este canal sem mover a antena
windows-antenna-switch-when-channel-recalled = A antena para a qual trocar quando este canal for recuperado. — deixa o que o rádio já está usando.
windows-shift = Deslocamento
windows-tone = Tom
windows-invert = INVERTER
windows-normal = NORMAL
windows-dcs-polarity = Polaridade DCS
windows-open-every-over-tone-burst = Abre cada transmissão com a rajada de tom enquanto este canal está recuperado
windows-memories = Memórias
windows-memory-name = nome da memória
windows-store = Guardar
windows-folder-name = nome da pasta
windows-new-folder = Nova pasta
windows-import = IMPORTAR
windows-read-channel-list-chirp-csv =
    Lê uma lista de canais de um arquivo CSV do CHIRP (.csv) — uma exportação de diretório de repetidoras, uma tabela de canais marítimos ou PMR, ou qualquer outra coisa que o CHIRP possa gravar.

    Each channel brings its frequency, mode, repeater shift and CTCSS or DCS tone with it. Channels already on this list — same frequency, same mode — are skipped, so re-importing an updated directory adds what is new rather than doubling what is not.
windows-chirp-csv = CSV do CHIRP
windows-export = EXPORTAR
windows-write-list-out-chirp-csv = Grava esta lista em um arquivo CSV do CHIRP, para carregar em um portátil ou guardar como cópia de segurança.
windows-no-memories-yet = nenhuma memória ainda
windows-drag-memory-onto-folder-file = arraste uma memória para uma pasta para arquivá-la, para baixo delas para desarquivá-la
windows-drop-here-unfile = solte aqui para desarquivar
windows-sort = Ordenar
windows-order-they-were-stored = Na ordem em que foram guardadas
windows-name-ignoring-case = Por nome, sem diferenciar maiúsculas
windows-frequency = Por frequência
windows-band-then-frequency-inside-each = Por faixa, depois por frequência dentro de cada uma
windows-descending-click-ascending = Decrescente — clique para crescente
windows-ascending-click-descending = Crescente — clique para decrescente
windows-delete-folder-memories-move-top = Excluir pasta — as memórias dela passam ao nível superior
windows-rename-folder = Renomear pasta
windows-empty-drop-memories-here = vazia — solte memórias aqui
windows-voice-keyer = Manipulador de voz
windows-rec-records-your-microphone-play = REC grava pelo seu microfone, TOCAR permite ouvir o que foi gravado, TX a põe no ar — assim como uma tecla do teclado numérico, um pad MIDI, ou o send_voice_mem do rigctld.
windows-stop-store = Parar e guardar
windows-stop = PARAR
windows-play = TOCAR
windows-stop-listening = Parar de ouvir
windows-nothing-recorded-slot = Nada gravado neste slot
windows-not-while-transmitting = Não durante a transmissão
windows-listen-message-nothing-transmitted = Ouvir esta mensagem — nada é transmitido
windows-stop-transmitting = Parar a transmissão
windows-tune-active-switch-off-first = TUNE está ativo — desligue-o primeiro
windows-transmit-message = Transmitir esta mensagem
windows-erase-recording = Apagar esta gravação
windows-rade-message-encoded-digital-voice = RADE: a mensagem é codificada pelo codec de voz digital, exatamente como seria uma transmissão ao vivo.

## TX Audio window: messages with values

txaudio-rec-hint = Grava uma frase de teste pelo microfone — até { $max } s, sem acionar o transmissor. Clique de novo para parar. Ela é mantida sem processamento, então TOCAR pode levar a mesma frase por cada alteração que você fizer.
txaudio-phrase-length = frase de { $secs } s
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = Ganho de entrada definido em { $gain } dB (os picos de voz estavam em { $loud } dBFS).{ $clip }
txaudio-listening = Ouvindo… { $secs } s
txaudio-auto-set-hint = Fale normalmente por { $secs } s — como faria no ar — e o ganho de entrada é definido para que a sua voz fique com picos em torno de { $target } dBFS. Clique de novo para cancelar.
txaudio-eq-point = { $hz } Hz, { $db } dB — arraste para mover
txaudio-not-a-profile = Não é um perfil de áudio TX: { $error }
txaudio-imported = "{ $name }" importado.
txaudio-saved = "{ $name }" salvo.
txaudio-deleted = "{ $name }" excluído.
txaudio-loaded-preset = Ponto de partida "{ $name }" carregado.

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · offline
solar-radius-exaggeration = Exagero do raio da Terra/Lua (máx. { $max }× nesta escala de órbita lunar)
solar-brightest-cell = célula mais intensa ≈ { $peak } percursos · { $bands } { $bands ->
        [one] faixa
       *[other] faixas
    }
solar-hits-in-hour = { $hits } na hora
solar-passes-from = { $name } · PASSAGENS DE { $grid }
solar-elements-age = elementos com { $age } de idade · SGP4
solar-point-at = Apontar para { $az }° ({ $compass }), elevação { $el }°.
solar-published-as = publicado como { $name }
solar-fly-there = { $name } — ↵ para ir até lá
solar-sats-tracked = { $n } de { $total } rastreados
solar-bodies-found = { $n } de { $total } corpos
solar-sats-and-bodies = { $n } de { $total } rastreados · { $bodies } de { $bodies_total } corpos
solar-confidence-on-band = { $confidence } em { $band }
solar-kp-now = { $kp } agora
solar-kp-in = { $kp } em { $hours } h
solar-valid-age = válido { $valid } · { $age } de idade
solar-full-bar = barra cheia = { $share } do mundo
solar-own-paths-memory = para onde foram os percursos desta estação · memória de { $minutes } min
solar-n-storms = { $n } tempestades
solar-clouds-legend = NUVENS { $time } · { $age } de idade · { $channels } · { $storms } · relâmpagos simulados
solar-cme-eta = ETA { $eta } (+{ $hours } h)
solar-cme-arrived = a chegada foi em { $eta } (há { $hours } h)
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = CME DIRIGIDA À TERRA { $time } · { $speed } km/s · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = Configuração de { $mode }
setup-transmits-as = Transmite como { $call }
setup-decimal-degrees = Graus decimais, negativos para { $hemisphere }. Cole um diretamente de um mapa.

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } NÃO CONFIÁVEL NESTA ROE
amp-lower-drive-first = Reduza a excitação primeiro: { $drive } W sobrecarregariam o amplificador na linha (o limite dele é { $limit } W).
amp-live-value = ao vivo { $value } { $unit }
amp-tripped = DISPARADO{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = topo ≈ { $n } percursos
panels-no-agreed-frequency = Nenhuma frequência combinada para { $mode } — abra isto para salvar a em que você está
panels-mode-frequencies-hint = As { $n } frequências de { $mode } — escolher uma sintoniza o dial, e você pode salvar a sua
panels-mode-frequencies = { $mode } · frequências
panels-global-convention = Uma convenção global que o plano de faixas da Região IARU { $region } não coloca em dados estreitos — confira o seu próprio plano de faixas antes de transmitir aqui.
panels-outside-data-segment = Âmbar: fora do segmento de dados da Região { $region }.
panels-save-dial = ＋ Salvar { $mhz } MHz
panels-save-dial-hint = Lembra este dial sob { $mode } e o oferece aqui daqui em diante. Salvo na estação, então cada tela ligada a este rádio o tem.
panels-tr-period = Período T/R de { $period } segundos
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = Esta vez: { $into } s de { $slot }. As decodificações chegam no fim dele, e é então que a próxima transmissão pode começar.{ $mark }
panels-burst-mark = {" "}A marca é onde a rajada termina, { $secs } s depois do início.
panels-tr-period-burst = Período T/R de { $slot } segundos, rajada de { $burst } s

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } { $n ->
        [one] canal
       *[other] canais
    }
windows-slot-n = Slot { $n }
windows-record-from-mic = Gravar pelo microfone (até { $max } s)

## SSTV panel: messages with values

sstv-picture-too-big = Esta imagem tem { $size } MB — o limite é { $limit } MB.
sstv-auto-mode = Automático ({ $mode })
sstv-not-decoded = { $what } — não decodificado
sstv-last = última: { $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = As imagens decodificadas são salvas { $place } e se acumulam aqui.
sstv-older-in-store = { $n } mais antigas no armazenamento
sstv-older-load-more = { $n } mais antigas — carregar mais
sstv-delete-picture-from = Excluir esta imagem { $place }
sstv-rifp-profile = { $profile } · ~{ $khz } kHz ocupados · o dial é o centro do canal
sstv-rifp-too-wide = ⚠ { $profile } ocupa ~{ $khz } kHz — largo demais para um segmento de faixa estreita
sstv-rifp-no-frequency = O RIFP não atribui frequência, e o sdroxide o transmitirá onde você sintonizar. Um canal de { $khz } kHz só cabe onde a operação em banda larga ou FM é permitida — { $segments } — e não em um segmento de faixa estreita, menos ainda em HF. Mesmo dentro desses, as condições da sua própria licença podem ser mais restritas. Você é o operador; confira as suas próprias regras.
sstv-rifp-tx-frame = ● Quadro TX { $frame }/{ $frames } · faltam { $secs } s
sstv-rifp-sent-as = enviado como { $encoding } · { $octets } octetos
sstv-rifp-rx-stats = quadros { $frames } · ruins { $bad } · imagens { $pictures }
sstv-rifp-session-idle = sessão { $session } · ociosa há { $secs } s
sstv-rifp-session-waiting = sessão { $session } · blocos retidos, ainda aguardando o manifesto · ociosa há { $secs } s
sstv-store-on-radio = no rádio, em { $dir }
sstv-store-in = em { $dir }

## decodes (src/app/panels/decodes.rs)

decodes-decodes = DECODIFICAÇÕES
decodes-adif = ADIF
decodes-sort = Ordenar
decodes-single-list = Lista única
decodes-every-decode-one-list-newest = Cada decodificação em uma só lista, com a vez mais recente primeiro, em vez de blocos de vezes pares/ímpares. Os chips de Ordenar então ordenam a lista inteira em vez de cada vez, e cada linha traz o horário do seu slot — ciano para um slot par, dourado para um ímpar.
decodes-cq-only = Somente CQ
decodes-only-stations-calling-cq-only = Somente estações chamando CQ — e somente as chamadas a que você pode responder: um CQ dirigido (DX, EU, JA, POTA, TEST …) aparece quando cita você e fica oculto quando cita outra pessoa.
decodes-new-only = Somente novas
decodes-only-stations-would-new-entity = Somente estações que seriam novas: entidade, slot de faixa, locator ou indicativo
decodes-auto-tx-frq = FRQ TX automática
decodes-overridden-hold-tx-lift-hold = Substituída por Manter TX. Libere a manutenção para escolher para que lado a frequência de transmissão se move.
decodes-hold-tx = Manter TX
decodes-pin-transmit-tone-where-nothing =
    Fixa o tom de transmissão onde está. Nada o move: não responder a uma estação, nem a fila de chamadas, nem chamar CQ, nem um clique em uma decodificação ou na cascata. Desligue para mover e depois ligue de novo.

    Changing band is the one exception, and it is your own act: the offset you last set on the new band comes back with it, so a 60 m figure is not carried onto 20 m and 20 m's is not carried onto 60 m.

    For where the licence is narrower than the band plan — on a UK 60 m dial of 5357 kHz the allocation ends at 5358.0, so the tone has to stay under 1000 Hz, and either automatic mover will walk out of the band between one over and the next.
decodes-type-transmit-offset-hz-then = Digite o deslocamento de transmissão em Hz, depois pressione Enter. Quando a sua licença é mais estreita que o plano de faixas, toda a faixa abaixo da borda fica à sua escolha, então nada é sugerido aqui.
decodes-clear-rx = {" "}LIMPAR RX{" "}
decodes-empty-decode-list-nothing-air = Esvazia a lista de decodificações. Nada que está no ar para.
decodes-even = PAR
decodes-text = TEXTO
decodes-dxcc = DXCC
decodes-band = FAIXA
decodes-grid = LOCATOR
decodes-dupe = DUPLICADA
decodes-reply = RESPOSTA
decodes-queued-click-remove = Na fila — clique para remover
decodes-work-station-after-current-one = Trabalhar esta estação depois da atual
decodes-qsos-worked-since-sdroxide-was = QSOs trabalhados desde que o sdroxide foi iniciado
decodes-qso-complete = ✓ QSO CONCLUÍDO
decodes-tx = ● TX
decodes-dxpedition-mode-transmit-frequency-held = Modo DXpedition. A frequência de transmissão é mantida fora da metade do Fox na banda de passagem (abaixo de 1000 Hz) até que o Fox responda.
decodes-watchdog = VIGIA
decodes-transmitting-stopped-no-reply-no = Transmissão interrompida: nenhuma resposta e nenhuma ação durante o período do vigia. Chame CQ ou escolha uma mensagem para retomar.
decodes-well-inside-tolerance = Bem dentro da tolerância.
decodes-no-active-qso-pick-decode = nenhum QSO ativo — escolha uma decodificação para responder, ou Chamar CQ
decodes-ft8-engine-idle = Motor FT8 ocioso
decodes-being-worked = sendo trabalhada
decodes-clear = LIMPAR
decodes-no-messages = — nenhuma mensagem —
decodes-call-cq = {"  "}CHAMAR CQ{"  "}
decodes-stop-qso = {" "}PARAR QSO{" "}
decodes-stop-tx = {" "}PARAR TX{" "}
decodes-free-text-13-chars = texto livre (13 caracteres)
decodes-send = ENVIAR

## sat (src/app/sat.rs)

sat-pass-profile = PERFIL DE PASSAGEM
sat-satellite = SATÉLITE
sat-satellites = SATÉLITES
sat-passes = PASSAGENS
sat-set-your-grid-passes-settings = Defina o seu locator para as passagens (Configurações ▸ Geral).
sat-next = PRÓXIMA
sat-peak = PICO ≥
sat-all-satellites = todos os satélites
sat-off-only-satellites-frequencies-file = Desligado: somente satélites com frequências cadastradas — os que você pode trabalhar — e qualquer um que você tenha colado. Ligado: todo satélite da lista, que podem ser milhares com uma assinatura e levam um tempo para pesquisar.
sat-no-passes-window-lower-peak = Nenhuma passagem nesta janela. Reduza o pico ou amplie a janela.
sat-length = DURAÇÃO
sat-peak-2 = PICO
sat-from-to = DE → PARA
sat-up-now = ● visível agora
sat-unlock = DESTRAVAR
sat-release-lock-dial-stays-where = Libera o travamento; o dial fica onde está
sat-azimuth = AZIMUTE
sat-elevation = ELEVAÇÃO
sat-range = DISTÂNCIA
sat-rate = TAXA
sat-downlink = DESCIDA
sat-doppler-rx = DOPPLER RX
sat-uplink = SUBIDA
sat-doppler-tx = DOPPLER TX
sat-no-pass-inside-next-48 = Nenhuma passagem nas próximas 48 horas a partir do seu QTH.
sat-elements-stale-corrections-suspended-until = ⚠ Elementos desatualizados — correções suspensas até uma atualização do TLE
sat-doppler-correction-needs-iq-front = A correção Doppler exige um front end I/Q — um equipamento CAT só rastreia
sat-rotator-not-connected = Rotor: não conectado
sat-doppler = DOPPLER
sat-apply-computed-correction-rx-tx = Aplicar a correção calculada ao RX e ao TX
sat-rotator = ROTOR
sat-steer-antenna-through-rotctld-client = Comanda a antena pelo cliente rotctld (Configurações ▸ Servidores)
sat-name-norad = nome ou NORAD
sat-set-your-grid-passes-settings-2 = defina o seu locator para as passagens (Configurações ▸ Geral)
sat-norad = NORAD
sat-pass = PASSAGEM
sat-always-visible = sempre visível
sat-not-here = não daqui
sat-no-elements = sem elementos
sat-pick-satellite-see-links-lock = Escolha um satélite para ver os enlaces dele e travar.
sat-no-frequencies-file-one-add = Não há frequências cadastradas para este — adicione-as em Configurações ▸ TLE.
sat-link = ENLACE
sat-downlink-mhz = DESCIDA (MHz)
sat-uplink-mhz = SUBIDA (MHz)
sat-mode = MODO
sat-tune = SINTONIZAR
sat-set-dial-mode-link-no = Define o dial e o modo para este enlace — sem travamento, sem correção
sat-locked = {" "}TRAVADO{" "}
sat-lock = {" "}TRAVAR{" "}
sat-track-satellite-doppler-correction-rx = Rastreia este satélite: correção Doppler no RX e no TX, subida mapeada no transponder, e a visão 3D o acompanha

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ PARAR{" "}
wefax-end-chart-now-save-what = Encerra a carta agora e salva o que chegou
wefax-start = {" "}● INICIAR{" "}
wefax-start-chart-now-without-waiting = Inicia uma carta agora, sem esperar um tom de início
wefax-phasing = fasando…
wefax-subcarrier-offset-1900-hz-tune = Deslocamento da subportadora a partir de 1900 Hz. Sintonize para aproximadamente zero: a portadora do fax vai de 1500 Hz (preto) a 2300 Hz (branco), e um receptor a algumas centenas de hertz de distância corta a imagem para preto ou branco sólidos.
wefax-auto-start = INÍCIO AUTOMÁTICO
wefax-auto-stop = PARADA AUTOMÁTICA
wefax-end-450-hz-stop-tone = Encerra no tom de parada de 450 Hz. Desligue para continuar gravando durante uma estação que envia várias cartas em sequência.
wefax-phase = FASE
wefax-slant = INCLINAÇÃO
wefax-sample-clock-trim-if-chart = Ajuste do clock de amostragem. Se a carta inclina para a esquerda, aumente; para a direita, reduza. Uma placa de som 100 ppm fora desloca uma carta de um quarto de hora quase uma linha inteira de lado.
wefax-view = VISÃO
wefax-scale-chart-panel-width = Ajustar a carta à largura do painel
wefax-whole = INTEIRA
wefax-shrink-chart-until-all-view = Reduz a carta até que ela inteira caiba na tela de uma vez
wefax-half-size = Meio tamanho
wefax-one-screen-pixel-per-fax = Um pixel da tela por pixel do fax — role para ver detalhes
wefax-twice-size-scroll-move-around = Tamanho dobrado; role para se mover pela carta
wefax-height = ALTURA
wefax-stretch-picture-vertically-chart-came = Estica a imagem verticalmente. Uma carta que saiu achatada ou esticada geralmente está sendo decodificada na taxa de linhas errada — isto a torna legível, e os chips LPM a corrigem de verdade. Clique duas vezes para digitar um valor.
wefax-reset = REDEFINIR
wefax-back-picture-s-own-proportions = Volta às proporções da própria imagem
wefax-follow = SEGUIR
wefax-keep-newest-lines-view-scrolling = Mantém as linhas mais recentes na tela. Rolar para cima desliga isto para que você possa ler o que já chegou; rolar de volta até o fim liga de novo.
wefax-saved = SALVAS
wefax-path = CAMINHO
wefax-reading-radio-s-charts = Lendo as cartas do rádio…
wefax-finished-charts-collect-here = As cartas concluídas se acumulam aqui.
wefax-delete-chart = Excluir esta carta
wefax-loading = carregando…
wefax-stations = ESTAÇÕES
wefax-broadcast-radiofax-schedules-picking-one = Programações de radiofax de radiodifusão — escolher uma sintoniza o dial e define a velocidade de linha
wefax-frequencies-are-published-carrier-dial = As frequências são a portadora publicada; o dial fica 1,9 kHz abaixo dela, o que é feito para você. As programações mudam e as estações encerram — trate isto como onde começar a procurar, não como uma tabela de horários.
wefax-delete-chart-store = Excluir esta carta do armazenamento
wefax-newer = ◀ MAIS NOVA
wefax-chart-received-after-one = A carta recebida depois desta
wefax-older = MAIS ANTIGA ▶
wefax-chart-received-before-one = A carta recebida antes desta
wefax-no-longer-store = não está mais no armazenamento
wefax-loading-full-size = carregando em tamanho real…
wefax-save = SALVAR
wefax-save-copy-chart-computer = Salvar uma cópia desta carta neste computador
wefax-sure = CERTEZA?
wefax-click-again-delete-chart-good = Clique de novo para excluir esta carta definitivamente
wefax-delete = EXCLUIR
wefax-file-chart-was-saved = O arquivo em que esta carta foi salva

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = iniciando o modem APRS…
aprs-aprs = APRS
aprs-no-callsign = sem indicativo
aprs-nothing-will-transmitted-no-beacon = Nada será transmitido — nenhuma baliza, nenhuma mensagem, nem mesmo uma confirmação — até que esta estação tenha um indicativo. Defina um em Configurações → Geral, ou um específico do APRS com o SSID dele em CONFIG.
aprs-no-position = sem posição
aprs-beacon-needs-somewhere-report-fill = Uma baliza precisa de um lugar para informar. Preencha o seu locator em Configurações → Geral, ou informe coordenadas em CONFIG.
aprs-setup = CONFIG
aprs-callsign-symbol-digipeater-path-position = Indicativo, símbolo, caminho de digipeaters, posição e baliza
aprs-how-often-beacon-your-position = Com que frequência transmitir a sua posição, sem supervisão. `off` — o padrão — nunca transmite a baliza: escolher um modo não pode colocar uma estação no ar. Trinta minutos é a convenção para uma estação fixa, com mais frequência para uma em movimento, e cada baliza é tempo de canal de outra pessoa.
aprs-beacon = {" "}BALIZA{" "}
aprs-station-has-no-callsign-see = Esta estação não tem indicativo — veja o aviso à esquerda.
aprs-station-has-no-position-report = Esta estação não tem posição para informar — veja o aviso à esquerda.
aprs-send-one-position-report-now = Envia um relatório de posição agora, sem esperar o intervalo. Ele espera o canal ficar livre como tudo o mais.
aprs-stations = ESTAÇÕES
aprs-nothing-heard-yet = nada ouvido ainda
aprs-heard-direct = ouvida diretamente
aprs-two-characters-station-sent-say = Os dois caracteres que a estação enviou para dizer o que ela é
aprs-killed = CANCELADO
aprs-station-put-object-here-has = A estação que colocou este objeto aqui o cancelou
aprs-sender-deliberately-reported-square-rather = O remetente informou deliberadamente um quadrado, e não um ponto. O mapa desenha o quadrado.
aprs-digipeaters-last-frame-came-through = Os digipeaters por onde passou o último quadro. Um `*` marca o que de fato o repetiu.
aprs-message = {" "}MENSAGEM{" "}
aprs-address-message-box-station = Endereça a caixa de mensagem a esta estação
aprs-center = {" "}CENTRALIZAR{" "}
aprs-put-station-middle-map-double = Coloca esta estação no meio do mapa. Clique duas vezes no mapa para devolver a visão ao ajuste automático.
aprs-channel = CANAL
aprs-messages = MENSAGENS
aprs-frames-arrived-failed-their-check = Quadros que chegaram e falharam na verificação de sequência — uma colisão, um desvanecimento, ou um sinal fraco demais para ler. Uma contagem subindo sem nada decodificado significa que o modem *está* ouvindo o canal: confira o medidor de nível e abra o squelch do rádio.
aprs-frames-read-cleanly-off-channel = Quadros lidos limpos do canal que não eram APRS — a sessão de pacote de outra pessoa, ou um formato que esta versão não decodifica.
aprs-every-frame-channel-arrived-other = Cada quadro do canal, como chegou — inclusive o tráfego de outras pessoas e tudo o que o decodificador não conseguiu ler.
aprs-nothing-channel-yet = nada no canal ainda
aprs-no-messages = Nenhuma mensagem.
aprs-waiting-channel = aguardando o canal
aprs-sent-retried-still-waiting-acknowledged = enviada e reenviada, ainda aguardando confirmação
aprs-air-waiting-acknowledged = no ar, aguardando confirmação
aprs-acknowledged-far-end = confirmada pela outra ponta
aprs-refused-far-end-not-retried = recusada pela outra ponta — não reenviada
aprs-no-ack = sem confirmação
aprs-no-answer-after-every-retry = nenhuma resposta após todas as tentativas
aprs-send = {" "}ENVIAR{" "}
aprs-station-has-no-callsign-so = Esta estação não tem indicativo, então nada pode ser transmitido. Defina um em Configurações → Geral, ou um específico do APRS com o SSID dele em CONFIG.
aprs-needs-station-address-something-say = Precisa de uma estação para endereçar e de algo a dizer.
aprs-send-keep-retrying-until-far = Envia, e continua tentando até que a outra ponta confirme. Mensagens são a única coisa neste canal que recebe resposta.
aprs-busy = OCUPADO
aprs-stations-heard-still-inside-window = Estações ouvidas que ainda estão dentro da janela definida em CONFIG APRS, e se outra estação está no canal agora. Nada aciona enquanto estiver ocupado.
aprs-tx = ● TX
aprs-over-progress-frames-waiting-channel = A transmissão em andamento, os quadros aguardando o canal ficar livre, ou o tempo até a próxima baliza agendada.
aprs-audio-reaching-modem-flat-means = Áudio chegando ao modem. Plano significa que nada está chegando — confira se a saída de dados do rádio é a que o sdroxide está escutando. Acende em verde enquanto o modem ouve uma portadora.

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = iniciando o decodificador VDL2…
vdl2-aircraft-datalink = enlace de dados de aeronaves
vdl2-tune-middle-vdl2-group-decoder = Sintoniza no meio do grupo VDL2. A janela própria do decodificador desliza a partir daí para abranger o máximo possível dos quatorze canais que o receptor alcança.
vdl2-setup = CONFIG
vdl2-which-channels-listen-how-hard = Em quais canais escutar, quão forte uma rajada precisa ser, e quanto log manter
vdl2-save-vdl2-message-log-file = Salvar o log de mensagens VDL2 em um arquivo
vdl2-vdl2-channel = Canal VDL2
vdl2-messages = MENSAGENS
vdl2-address-flight-registration-label = endereço, voo, matrícula ou rótulo
vdl2-held = RETIDO
vdl2-hold = RETER
vdl2-hold-message-log-where-so = Mantém o log de mensagens onde está, para que uma delas possa ser lida sem que a próxima transmissão a empurre para cima na tela. O decodificador continua rodando e os contadores continuam andando; soltar mostra tudo o que chegou nesse meio-tempo.
vdl2-nothing-decoded-yet = nada decodificado ainda
vdl2-nothing-matches-filter = nada corresponde ao filtro
vdl2-stations = ESTAÇÕES
vdl2-nobody-heard-yet = ninguém ouvido ainda
vdl2-vdl2-setup = Configuração VDL2
vdl2-channels = Canais
vdl2-every-25-khz-slot-136 = Cada slot de 25 kHz de 136,650 a 136,975 MHz. Um conversor de descida para cada um, todos dentro da mesma janela do receptor. Desligar um economiza um pouco de tempo de processador; não torna os outros mais sensíveis.
vdl2-burst-threshold = Limiar de rajada
vdl2-above-each-channel-s-own = acima do piso de ruído próprio de cada canal
vdl2-lower-catches-weaker-transmissions-costs = Um valor menor captura transmissões mais fracas e custa tempo de processador com ruído; um maior as perde. O piso é aprendido por canal, e uma alteração aqui não o descarta.
vdl2-keep-log = Manter no log
vdl2-track-most = Rastrear no máximo
vdl2-longest-silent-go-first = as que estão mais tempo em silêncio saem primeiro
vdl2-forget-station-after = Esquecer uma estação após
vdl2-nothing-heard = sem nada ouvido dela
vdl2-show-unread-payloads = Mostrar cargas não lidas
vdl2-hex = em hexadecimal
vdl2-frames-carrying-x-25-clnp = Quadros que carregam X.25, CLNP ou as aplicações de enlace de dados acima deles. O SDRoxide os identifica e mostra os bytes em vez de lê-los, e ocultá-los esconderia quanto do tráfego eles são.
vdl2-registration = Matrícula
vdl2-flight = Voo
vdl2-label = Rótulo
vdl2-block = Bloco
vdl2-sequence = Sequência
vdl2-mode = Modo
vdl2-more-blocks-follow = mais blocos seguem
vdl2-check = Verificação
vdl2-message-crc-good = CRC da mensagem correto
vdl2-message-crc-not-checked = CRC da mensagem não verificado
vdl2-parity = Paridade
vdl2-link-control-no-payload = controle de enlace, sem carga
vdl2-name = NOME
vdl2-addr = END
vdl2-type = TIPO
vdl2-msgs = MSGS

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = O tom que está sendo copiado e o tom transmitido — em CW são a mesma frequência. Clique na cascata para movê-lo sobre um sinal.
cw-down-10-hz = Descer 10 Hz
cw-up-10-hz = Subir 10 Hz
cw-qrg-main-readout-tuning-line = QRG: a leitura principal e a linha de sintonia ficam na frequência em que se opera. Clique para o dial, com um tom de sidetone abaixo dele — o que a maioria dos rádios mostra.
cw-put-main-readout-tuning-line =
    Coloca a leitura principal e a linha de sintonia sobre o sinal, em vez de sobre o dial, para que a frequência mostrada seja a que os dois operadores citariam, e a linha de sintonia fique no meio da banda de passagem. A sintonia não muda; só os números se movem.

    Clicking a signal lands it on the cursor only as closely as the click step allows — Controls → click-to-tune rounding, 10 Hz by default. A coarse step leaves the signal off the pitch by up to half of it, and the readout will say so.
cw-sending-speed-read-off-signal = Velocidade de envio lida do sinal
cw-signal-noise-500-hz-same = Sinal para ruído em 500 Hz — o mesmo número que um relatório cita
cw-how-far-off-cursor-signal = Quão longe do cursor o sinal realmente está. O decodificador o segue; a banda de passagem não, então empurre o cursor se isto crescer.
cw-listening = — escutando —
cw-tx = ● TX
cw-nothing-copied-yet = — nada copiado ainda —
cw-key-send-characters-you-send = Manipule para enviar — os caracteres que você envia aparecem aqui.
cw-type-line-return-sends = Digite uma linha, Enter envia…
cw-type-here-send = Digite aqui para enviar…
cw-key = {" "}MANIPULADOR ●{" "}
cw-key-2 = {" "}MANIPULADOR{" "}
cw-hold-key-bound-cw-straight = Segure a tecla associada ao manipulador reto de CW — Barra de espaço por padrão, e qualquer tecla que você quiser em Configurações → Controles — como um manipulador reto: abaixada enquanto é mantida, levantada ao soltar, em vez de digitar texto. A caixa de transmissão fica travada enquanto isto está ligado, e todo o manipulador automático passa para a tecla: o texto que estava na fila é descartado.

    To hand-key it, set CW keying to "Sound card (MCW)" in Settings → Radio. The rig is then held on a sideband and the keyer's own sidetone is transmitted as audio, which is the route the straight key drives.
cw-watchdog = VIGIA
cw-straight-key-was-held-down = O manipulador reto ficou abaixado tempo demais — uma liberação perdida, e não uma mão — então a portadora foi cortada e a transmissão desligada. Pressione a tecla de novo para continuar.
cw-tx-2 = {"  "}TX LIGADO{"  "}
cw-tx-3 = {"   "}TX{"   "}
cw-send-what-box-now-without = Envia agora o que está na caixa, sem esperar o Enter
cw-hold-key-down-between-characters = Mantém a tecla abaixada entre os caracteres, para que nada digitado espere
cw-call-cq = {" "}CHAMAR CQ{" "}
cw-nocall = SEMCHAMADA
cw-clear = {" "}LIMPAR{" "}
cw-stop-sending-drop-whatever-has = Para de enviar e descarta o que ainda não saiu. Com o manipulador reto ligado, a leitura de retorno do que foi manipulado vai junto.
cw-sidetone = SIDETONE
cw-play-keyed-tone-through-computer = Toca o tom manipulado pelos alto-falantes deste computador, além de enviá-lo, para que você ouça o que está mandando. Em `Placa de som (MCW)` o tom vai para o rádio e para mais nada, então sem isto você envia em silêncio. Desligue onde o monitor do próprio rádio ou uma cópia fora do ar já faz o trabalho.
cw-hold-what-typed-until-return = Segura o que é digitado até o Enter, e então envia a linha de uma vez, em vez de manipular cada caractere à medida que é digitado. Vale ter em um transceptor que se aciona sozinho a partir do texto, onde cada entrega ao manipulador dele é mais um ciclo de transmissão e recepção.
cw-cw-messages = MENSAGENS CW
cw-clear-rx = {" "}LIMPAR RX{" "}
cw-empty-receive-window-straight-key = Esvazia a janela de recepção e a leitura de retorno do manipulador reto. Nada que está no ar para.
cw-lock = TRAVAR
cw-decode-speed-set-here-instead = Decodifica na velocidade definida aqui, em vez de lê-la do sinal. Ajuda um sinal fraco demais para que a busca de velocidade se fixe.
cw-farnsworth-send-characters-full-speed = Farnsworth: envia os caracteres em velocidade plena e estica apenas os intervalos entre eles, para que sejam ouvidos no ritmo certo mas cheguem devagar o bastante para serem anotados.
cw-off-normal-spacing = Desligado — espaçamento normal
cw-keying-speed = Velocidade de manipulação
cw-idle-0 = OCIOSO 0
cw-how-long-transmit-held-after = Por quanto tempo a transmissão é mantida depois do último caractere, ou depois de o manipulador reto ser solto, antes de a portadora cair. Um valor maior cobre as pausas de quem digita devagar; um menor sai da frequência mais cedo; Desligado a derruba assim que tudo na fila já saiu.
cw-off-drop-once = Desligado — derrubar de imediato

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = Sintonizar 10 Hz para baixo
textmodem-tune-up-10-hz = Sintonizar 10 Hz para cima
textmodem-tx = ● TX
textmodem-listening = — escutando —
textmodem-type-line-return-sends = Digite uma linha, Enter envia…
textmodem-type-here-transmit = Digite aqui para transmitir…
textmodem-tx-2 = {"  "}TX LIGADO{"  "}
textmodem-tx-3 = {"   "}TX{"   "}
textmodem-call-cq = {" "}CHAMAR CQ{" "}
textmodem-nocall = SEMCHAMADA
textmodem-clear = {" "}LIMPAR{" "}
textmodem-hold-what-typed-until-return = Segura o que é digitado até o Enter, e então envia a linha de uma vez, em vez de transmitir cada caractere no ar à medida que é digitado. Permite que uma linha seja relida e corrigida antes que qualquer parte saia.
textmodem-messages = MENSAGENS
textmodem-hell = HELL
textmodem-contrast = Contraste
textmodem-harder-softer-dots-redraws-whole = Pontos mais fortes ou mais fracos — redesenha a faixa inteira
textmodem-width = Largura
textmodem-t-2row = {" "}2LINHAS{" "}
textmodem-draw-every-column-twice-stacked = Desenha cada coluna duas vezes, empilhadas. O Hell não tem sincronismo vertical, então isto mantém uma cópia completa do texto legível qualquer que seja a fase.
textmodem-rev = {" "}INV{" "}
textmodem-reverse-video-light-dots-dark = Vídeo invertido — pontos claros em papel escuro
textmodem-clear-rx = {" "}LIMPAR RX{" "}
textmodem-hold-channel-idle-sends-blank = Mantém o canal: ocioso envia papel em branco, para que a faixa continue rolando
textmodem-hold-what-typed-until-return-2 = Segura o que é digitado até o Enter, e então envia a linha de uma vez, em vez de pintar cada caractere na faixa à medida que é digitado. Uma correção digitada ao vivo já está no papel; uma feita antes do Enter nunca esteve.
textmodem-shift = Deslocamento
textmodem-shift-5-hz = Deslocamento −5 Hz
textmodem-shift-5-hz-2 = Deslocamento +5 Hz
textmodem-baud = Baud
textmodem-reverse-swap-mark-space-needed = Inverter: troca marca e espaço. Necessário quando o sinal é recebido na banda lateral oposta à que foi enviado, o que inverte os tons.
textmodem-track-tuning-error-automatically-instead = Acompanha o erro de sintonia automaticamente, em vez de ficar preso ao cursor.
textmodem-tones = Tons
textmodem-mode = Modo
textmodem-off-keyed = acionado liga/desliga

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = iniciando o decodificador AIS…
ais-tune-between-two-ais-channels = Sintoniza entre os dois canais AIS — 161,975 (AIS 1, marítimo 87B) e 162,025 (AIS 2, 88B). O dial fica no meio porque nada transmite ali, então o pico de CC de um receptor de zero-IF não cai em nenhum dos dois.
ais-setup = CONFIG
ais-channels-timeouts-trail-length-how = Canais, tempos limite, comprimento do rastro e até onde os vetores alcançam
ais-vessels = EMBARCAÇÕES
ais-nothing-heard-yet = nada ouvido ainda
ais-virtual-nothing-there = virtual — não há nada ali
ais-position-stale = posição desatualizada
ais-search-rescue-transmitter-man-overboard = um transmissor de busca e salvamento, baliza de homem ao mar ou EPIRB AIS
ais-call-sign = indicativo de chamada
ais-station-time = hora da estação
ais-dgnss = {" "}(DGNSS)
ais-first-heard = ouvido pela primeira vez
ais-center = CENTRALIZAR
ais-put-vessel-middle-chart = Coloca esta embarcação no meio da carta
ais-last-message-station-nmea-sentences = A última mensagem desta estação, como as sentenças NMEA que um receptor colocaria no fio. Cole-a em qualquer outro decodificador AIS para conferir o que o sdroxide fez dela.
ais-ais-setup = Configuração AIS
ais-channels = Canais
ais-drop-chart-after = Remover da carta após
ais-without-position-report = sem relatório de posição
ais-five-minutes-default-not-ads = Cinco minutos por padrão, não os dez segundos do ADS-B: uma embarcação fundeada informa uma vez a cada três minutos, e uma janela menor apagaria a maior parte de um porto entre dois relatórios perfeitamente bons. Depois disso a embarcação sai da carta e a linha dela fica cinza — não esmaecida, porque um símbolo apagado em uma posição antiga ainda é uma afirmação sobre onde o navio está, na mesma tinta que as verdadeiras.
ais-drop-list-after = Remover da lista após
ais-nothing-heard-all = sem nada ouvido
ais-message-carrying-ship-s-name = A mensagem que traz o nome de um navio volta a cada seis minutos, então uma janela curta na lista continua descartando embarcações justamente antes de elas dizerem como se chamam.
ais-trail-length = Comprimento do rastro
ais-history-behind-each-target = de histórico atrás de cada alvo
ais-minutes-rather-than-points-because = Em minutos, e não em pontos, porque as taxas de relatório do AIS abrangem duas ordens de grandeza: uma contagem fixa seria oitenta segundos de uma balsa e duas horas de um petroleiro fundeado, desenhados do mesmo jeito. Zero desliga os rastros.
ais-speed-vector = Vetor de velocidade
ais-how-far-ahead-vector-reaches = até onde o vetor alcança à frente
ais-slot-threshold = Limiar do slot
ais-above-channel-s-learned-noise = acima do piso de ruído aprendido do canal
ais-track-most = Rastrear no máximo
ais-vessels-2 = {" "}embarcações
ais-ais-receive-only-here-always = O AIS é somente recepção aqui e sempre será: é um serviço de segurança da vida, e colocar tráfego falso de embarcações nele não é algo que uma licença cubra.
ais-fill-my-grid-digimode-setup = Preencha Meu locator na configuração de modos digitais para distâncias e rumos, e para que a carta se enquadre em torno de onde você está, e não em torno do que estiver mais longe.
ais-name = NOME
ais-mmsi = MMSI
ais-type = TIPO

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = Este receptor não alcança 10489,750 MHz sozinho — configure antes um deslocamento de LNB/conversor (Configurações ▸ Rádio)
qo100-run-fast-spectral-tracker-every = Roda o rastreador espectral rápido: a cada segundo, procura na janela de estacionamento os dois lóbulos simétricos da baliza e informa onde está a portadora
qo100-telemetry = TELEMETRIA
qo100-also-run-ao-40-uncoded = Também roda o decodificador de quadros sem codificação do AO-40 — palavra de sincronismo, CRC e o texto de telemetria — com uma leitura passo a passo (portadora → sincronismo → CRC) de até onde cada passagem chega
qo100-auto = AUTO
qo100-auto-correct-let-tracker-adjust = Correção automática: deixa o rastreador ajustar sozinho o deslocamento do conversor/LNB — um laço fechado lento que, com uma estimativa limpa e estável, empurra a baliza de volta para 10489,750 MHz e a mantém ali conforme o LNB deriva. Reabre o front end cada vez que age.
qo100-narrower-search-smaller-slice = Mais estreita — pesquisa uma fatia menor
qo100-wider-when-beacon-isn-t = Mais larga — para quando a baliza não é encontrada na largura atual
qo100-clear-mark = limpar marcação
qo100-drop-hand-placed-beacon-mark = Remove a marcação da baliza colocada à mão e volta à leitura do próprio decodificador
qo100-lo = lo −
qo100-lo-2 = lo +
qo100-hi = hi −
qo100-hi-2 = hi +
qo100-radio-s-own-tuning-range = A faixa de sintonia própria deste rádio não alcança 10489,750 MHz — ele precisa de um deslocamento de LNB/conversor (Configurações ▸ Rádio ▸ Conversor) antes que esta janela possa fazer qualquer coisa.
qo100-open-settings-radio = Abrir Configurações ▸ Rádio
qo100-tune-10489-750-mhz = Sintonizar em 10489,750 MHz
qo100-tip-if-you-can-see = dica: se você vê a baliza na faixa mas ela não trava, clique duas vezes nela — isso a marca como 10489,750 MHz e permite que APLICAR corrija para ela
qo100-tracker = RASTREADOR
qo100-no-twin-lobe-shape-parking = nenhuma forma de dois lóbulos na janela de estacionamento
qo100-shape = FORMA
qo100-drift = DERIVA
qo100-need-longer-run-estimates = — (precisa de uma sequência maior de estimativas)
qo100-cycles = CICLOS
qo100-receiver = RECEPTOR
qo100-target = ALVO
qo100-measured = MEDIDO
qo100-clicked = {"  "}(clicado)
qo100-tracker-2 = {"  "}(rastreador)
qo100-not-locked-yet = ainda não travado
qo100-converter-offset = DESLOCAMENTO DO CONVERSOR
qo100-decode = DECODIFICAR
qo100-decode-filling-first-frame-buffer = DECODIFICAR — enchendo o primeiro buffer de quadros…
qo100-decode-off-turn-telemetry-chip = DECODIFICAR — desligado. Ligue o chip TELEMETRIA (linha superior) para a leitura portadora → sincronismo → CRC.
qo100-apply-correction = {" "}APLICAR CORREÇÃO{" "}
qo100-write-converter-lnb-offset-puts = Grava o deslocamento do conversor/LNB que coloca a baliza, onde você clicou, em 10489,750 MHz, e reabre o receptor — a mesma breve interrupção que Configurações ▸ Rádio ▸ Aplicar causa
qo100-write-converter-lnb-offset-puts-2 = Grava o deslocamento do conversor/LNB que coloca a estimativa do rastreador em 10489,750 MHz, e reabre o receptor — a mesma breve interrupção que Configurações ▸ Rádio ▸ Aplicar causa
qo100-waiting-second-crc-valid-frame = Aguardando um segundo quadro com CRC válido antes de oferecer para gravar isto — um travamento sozinho pode ser uma coincidência
qo100-write-corrected-converter-lnb-offset = Grava o deslocamento corrigido do conversor/LNB e reabre o receptor — uma breve interrupção, a mesma que Configurações ▸ Rádio ▸ Aplicar causa
qo100-decoder-runs-receiving-station-readout = o decodificador roda na estação receptora — sua leitura não é enviada a clientes remotos
qo100-starting = iniciando…
qo100-searching-first-window-fills-after = pesquisando — a primeira janela se completa em cerca de 24 s, depois se repete

## js8 (src/app/panels/js8.rs)

js8-multi = MULTI
js8-decode-every-js8-speed-not = Decodifica todas as velocidades do JS8, e não só aquela em que você transmite. As quatro velocidades dividem a sub-faixa e são quatro formas de onda diferentes, então sem isto uma estação em outra velocidade é invisível — e uma troca entre duas velocidades não pode acontecer de jeito nenhum. Custa cerca de quatro vezes a CPU de recepção.
js8-hb-auto = HB AUTO
js8-turbo-does-not-beacon-local = O Turbo não transmite baliza — é a velocidade local e de VHF
js8-beacon-your-callsign-grid-every = Transmite o seu indicativo e locator a cada 15 minutos
js8-until-next-heartbeat = Até o próximo heartbeat
js8-tx = ● TX
js8-heard = OUVIDAS
js8-nothing-heard-yet = — nada ouvido ainda —
js8-dxcc = DXCC
js8-band = FAIXA
js8-grid = LOCATOR
js8-dupe = DUPLICADA
js8-reply = RESPOSTA
js8-no-messages = — nenhuma mensagem —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}HB{" "}
js8-clear = {" "}LIMPAR PARA{" "}
js8-forget-selected-station-composer-goes = Esquece a estação selecionada — o compositor volta para @ALLCALL
js8-already-addressing-allcall = Já endereçando @ALLCALL
js8-stop = {" "}PARAR{" "}
js8-send = {" "}ENVIAR{" "}
js8-message = Mensagem…

## logbook (src/app/logbook.rs)

logbook-logbook = DIÁRIO
logbook-new-entry = + NOVO REGISTRO
logbook-adif = ADIF
logbook-import = IMPORTAR
logbook-import-qsos-adif-adi-adif = Importa QSOs de um arquivo ADIF (.adi ou .adif)
logbook-new-qso = NOVO QSO
logbook-edit-qso = EDITAR QSO
logbook-call = Indicativo
logbook-lookup = CONSULTAR
logbook-look-up-name-qth-grid = Consulta nome / QTH / locator
logbook-grid = Locator
logbook-freq-mhz = Freq MHz
logbook-mode = Modo
logbook-rst-sent = RST enviado
logbook-rst-rcvd = RST recebido
logbook-name = Nome
logbook-state = Estado
logbook-country = País
logbook-date-utc = Data UTC
logbook-time = Hora
logbook-pwr-w = Pot W
logbook-contest = Concurso
logbook-s-sent = Nº enviado
logbook-s-rcvd = Nº recebido
logbook-comment = Comentário
logbook-save = {" "}SALVAR{" "}
logbook-cancel = CANCELAR
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = cartão ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = Excluir este registro
logbook-edit = EDITAR
logbook-upload-qso-configured-logs = Envia este QSO para os logs configurados
logbook-no-qsos-yet-run-ft8 = nenhum QSO ainda — rode FT8/FT4 ou adicione um registro manual

## scanner (src/app/scanner.rs)

scanner-scanner = Scanner
scanner-work-through-stored-memory-channels = Percorre os canais de memória guardados
scanner-work-through-slice-band-channel = Percorre uma fatia de uma faixa em uma grade de canais
scanner-stop = PARAR
scanner-start = INICIAR
scanner-from = De
scanner-step = Passo
scanner-mode = Modo
scanner-range-empty-high-edge-has = Esta faixa está vazia — a borda superior precisa estar pelo menos um canal acima da inferior.
scanner-skip-while-holding-pass-over = PULAR enquanto estiver mantido para passar por cima deste canal daqui em diante
scanner-skipping = Pulando
scanner-clear = LIMPAR
scanner-stop-passing-over-any-them = Parar de passar por cima de qualquer um deles
scanner-stop-passing-over-channel = Parar de passar por cima deste canal
scanner-no-memory-channels-stored-yet = Nenhum canal de memória guardado ainda — guarde alguns na janela MEM.
scanner-skip-channel-pass-over = PULAR um canal para passar por cima dele
scanner-skip = PULAR
scanner-look-all-channels-fall-inside = Procura todos os canais que caem dentro de uma janela de receptor, na mesma transformada de que o panadaptador é feito, e só sintoniza naqueles em que algo está. Uma lista em uma faixa passa então a custar uma sintonia por volta, não importa o tamanho, em vez de um tempo de acomodação por canal. A varredura ainda escuta cada candidato antes de parar, então o que a interrompe não muda — mas a varredura mede pela FFT, e não pelo filtro do receptor, então confira o limiar se ele começar a parar em nada.
scanner-radio-hands-over-demodulated-audio = Este rádio entrega áudio demodulado e não tem espectro próprio para pesquisar, então a varredura de memórias visita cada canal de qualquer forma.
scanner-fast = RÁPIDO
scanner-read-list-off-spectrum-instead = lê a lista a partir do espectro, em vez de visitar cada canal
scanner-folders = Pastas
scanner-scan-every-folder-every-folder = Varre toda pasta, e toda pasta criada daqui em diante. Escolha pastas para varrer só essas.
scanner-unfiled = Sem pasta
scanner-nothing-filed-under-folders-you = Nada está arquivado nas pastas que você escolheu.
scanner-stops = Para em
scanner-use-receiver-s-own-squelch = Usa o squelch próprio do receptor, para que a varredura pare exatamente onde o áudio abriria
scanner-squelch-off-scan-will-stop = o squelch está desligado — a varredura parará no primeiro sinal que examinar
scanner-channel-power-signal-has-reach = Potência do canal que um sinal precisa alcançar para parar a varredura
scanner-listens = Escuta por
scanner-how-long-stay-candidate-before = Quanto tempo permanecer em um candidato antes de julgá-lo. Abaixo de cerca de um décimo de segundo o medidor de nível não se estabiliza e sinais fracos são perdidos
scanner-resumes = Retoma
scanner-carry-once-signal-drops = Continua assim que o sinal cai
scanner-carry-after-fixed-time-regardless = Continua após um tempo fixo, seja como for
scanner-stay-until-you-press-next = Fica até você pressionar PRÓXIMO
scanner-how-long-stay = Quanto tempo permanecer
scanner-how-long-wait-after-signal = Quanto tempo esperar depois que o sinal cai
scanner-move-don-t-stop-channel = Segue adiante, e não para de novo neste canal
scanner-next = PRÓXIMO
scanner-move-now = Seguir adiante agora

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = iniciando o decodificador ADS-B…
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = Modo S de 1090 MHz
adsb-tune-receiver-1090-000-mhz = Sintoniza o receptor em 1090,000 MHz, o enlace de descida ADS-B mundial. Há apenas um canal.
adsb-setup = CONFIG
adsb-timeouts-trail-length-how-far = Tempos limite, comprimento do rastro e até onde os vetores de velocidade alcançam
adsb-aircraft = AERONAVES
adsb-nothing-heard-yet = nada ouvido ainda
adsb-ground = em solo
adsb-position-stale = posição desatualizada
adsb-gnss-altitude = Altitude GNSS
adsb-ground-speed = velocidade no solo
adsb-vertical-rate = razão vertical
adsb-first-heard = ouvido pela primeira vez
adsb-center = CENTRALIZAR
adsb-put-aircraft-middle-map = Coloca esta aeronave no meio do mapa
adsb-last-frame-accepted-aircraft = O último quadro aceito desta aeronave
adsb-ads-b-setup = Configuração ADS-B
adsb-drop-map-after = Remover do mapa após
adsb-without-position-report = sem relatório de posição
adsb-past-aircraft-comes-off-map = Depois disto, a aeronave sai do mapa e a linha dela fica cinza. Não é esmaecida: um quadrado apagado em uma posição antiga ainda é uma afirmação sobre onde um avião está, na mesma tinta que as verdadeiras.
adsb-drop-list-after = Remover da lista após
adsb-nothing-heard-all = sem nada ouvido
adsb-trail-length = Comprimento do rastro
adsb-points = {" "}pontos
adsb-history-dots-behind-each-target = pontos de histórico atrás de cada alvo
adsb-speed-vector = Vetor de velocidade
adsb-how-far-ahead-leader-line = até onde a linha guia alcança à frente
adsb-one-minute-usual-radar-convention = Um minuto é a convenção usual de radar: a linha tem exatamente o comprimento da distância que a aeronave percorre nesse tempo, então duas guias iguais são duas velocidades iguais em qualquer zoom. Zero as desliga.
adsb-track-most = Rastrear no máximo
adsb-aircraft-2 = {" "}aeronaves
adsb-aircraft-ground-are-placed-against = Aeronaves no solo são posicionadas em relação à própria posição da estação — um squitter de superfície não tem decodificação inequívoca por conta própria — então preencha Meu locator na configuração de modos digitais se o aeroporto próximo não mostrar nada.
adsb-call = INDICATIVO
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = aguardando o receptor…
drm-not-drm-set-mode-drm = Não está em DRM. Defina o modo como DRM em uma transmissão digital de ondas curtas — o dial fica no centro do canal, não em uma banda lateral.
drm-no-drm-signal-locked-decoder = Nenhum sinal DRM travado. O decodificador precisa de alguns segundos em uma portadora limpa: o DRM entrelaça ao longo de 400 ms ou 2 s antes que qualquer parte possa ser lida.
drm-station-s-audio-codec-not = O codec de áudio desta estação não decodifica aqui. O xHE-AAC precisa do libfdk-aac no sistema — não pode ser embutido, porque a licença dele e a deste programa são incompatíveis. Instale-o (Debian/Ubuntu: libfdk-aac2, Arch: libfdk-aac, macOS: brew install fdk-aac, Windows: libfdk-aac-2.dll ao lado do sdroxide.exe) e reinicie. Todo o resto desta tela está decodificando normalmente.
drm-quality = QUALIDADE
drm-last-minute = último minuto
drm-waiting-locked-signal = aguardando um sinal travado
drm-audio-frames-decoding-but-station = Quadros de áudio decodificando — mas o codec desta estação não pode ser decodificado aqui
drm-audio-frames-decoding = Quadros de áudio decodificando
drm-samples-reaching-decoder = Amostras chegando ao decodificador
drm-time = HORA
drm-symbol-timing-recovered = Temporização de símbolo recuperada
drm-frame = QUADRO
drm-transmission-frames-found = Quadros de transmissão encontrados
drm-fast-access-channel-what-transmission = Canal de Acesso Rápido — o que é a transmissão
drm-service-description-channel-what-services = Canal de Descrição de Serviços — quais são os serviços
drm-audio = ÁUDIO
drm-constellation = CONSTELAÇÃO
drm-waiting-decoded-symbols = aguardando símbolos decodificados
drm-sampled-across-frame = amostrada ao longo do quadro
drm-whole-frame = quadro inteiro
drm-mode = MODO
drm-robustness-mode-channel-width-ground = Modo de robustez e largura de canal. A é para um percurso de onda de superfície e carrega mais; D é para um percurso de onda celeste muito espalhado e carrega menos.
drm-interleave = ENTRELAÇAMENTO
drm-how-far-transmission-spreads-each = Quanto a transmissão espalha cada quadro no tempo. Longo aguenta desvanecimentos mais profundos e leva proporcionalmente mais tempo para adquirir.
drm-protection = PROTEÇÃO
drm-offset = DESLOCAMENTO
drm-residual-sample-clock-error-against = Erro residual do clock de amostragem em relação ao transmissor. Grande e constante significa que a referência do receptor está fora, e não a transmissão.
drm-doppler = DOPPLER
drm-delay = ATRASO
drm-doppler-delay-spread-path-how = Espalhamento Doppler e de atraso do percurso — quão depressa ele se move e quão distantes chegam os seus ecos.
drm-service = SERVIÇO
drm-decode-service-multiplex = Decodifica este serviço do multiplex

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = iniciando a estação AtCHAT NET…
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET — teclado e arquivo multiestação
atchat-net = {" "}NA NET{" "}
atchat-net-click-leave-state-kept = Na net — clique para sair (o estado é mantido; clique de novo para voltar)
atchat-rejoin = {" "}VOLTAR{" "}
atchat-off-net-click-rejoin = Fora da net — clique para voltar
atchat-off-net = fora da net
atchat-carrier = ● PORTADORA
atchat-another-station-transmitting-channel-busy = Outra estação está transmitindo — o canal está ocupado.
atchat-tx = ● TX
atchat-station-callsign-virtual-channel-address = Indicativo da estação e o endereço do canal virtual
atchat-virtual-channel = CANAL VIRTUAL
atchat-work-net-over-tcp-endpoint = Trabalha a net por um endpoint TCP em vez do rádio — um loopback compatível com channel_server para desenvolvimento sem rádio. Desligado coloca a estação no ar.
atchat-roster = LISTA
atchat-nobody-heard-yet = ninguém ouvido ainda
atchat-chat = {" "}CHAT{" "}
atchat-common-channel-lines-here-go = O canal comum — as linhas daqui vão para todos (ALL)
atchat-log = {" "}LOG{" "}
atchat-station-s-own-air-activity = A atividade no ar da própria estação — eleição de mestre, envelhecimento da lista, retentativas ARQ
atchat-no-activity-logged-yet = Nenhuma atividade registrada ainda.
atchat-no-messages-yet = Nenhuma mensagem ainda.
atchat-message-everyone-all = mensagem para todos (ALL)
atchat-send = {" "}ENVIAR{" "}
atchat-transfers = TRANSFERÊNCIAS
atchat-all = → ALL
atchat-sent-file-follows-active-chat = Um arquivo enviado segue a aba de chat ativa
atchat-send-file = {" "}ENVIAR ARQUIVO{" "}
atchat-send-file-image-over-air = Envia um arquivo ou imagem pelo ar, com bloco-CRC-ARQ
atchat-nothing-flight = nada em trânsito
atchat-images = IMAGENS
atchat-no-images-received = Nenhuma imagem recebida.


## rds (src/app/rds.rs)

rds-station = ESTAÇÃO
rds-diagnostics = DIAGNÓSTICO
rds-which-programme-type-table-read = Qual tabela de tipo de programa usar para ler esta estação. O RDS é usado em todo lugar fora da América do Norte; o RBDS acrescenta o indicativo que o código de identificação de uma estação dos EUA soletra. O Automático segue o código de país estendido que a estação envia, e até que um chegue não soletra indicativo algum — as faixas de identificação que ele leria são compartilhadas com os códigos RDS de nove países, então selecione RBDS para nomear uma estação que não envia código de país.
rds-waiting-receiver = aguardando o receptor…
rds-no-rds-station-not-every = Nenhum RDS nesta estação. Nem todo transmissor o transmite, e um fraco pode transmiti-lo sem que ele chegue.
rds-rds-only-carried-wfm-broadcast = O RDS só é transmitido por estações de radiodifusão WFM.
rds-holding = RETIDO
rds-block-sync-has-dropped-last = A sincronia de blocos caiu — esta é a última estação lida.
rds-identity = IDENTIDADE
rds-programme = PROGRAMA
rds-described = DESCRITO COMO
rds-content = CONTEÚDO
rds-music = Música
rds-speech = Fala
rds-traffic = TRÂNSITO
rds-announcement-air-now = anúncio no ar agora
rds-carried-station = transmitido por esta estação
rds-not-carried = não transmitido
rds-radio-text = TEXTO DE RÁDIO
rds-station-clock = RELÓGIO DA ESTAÇÃO
rds-country-code = CÓDIGO DE PAÍS
rds-also = TAMBÉM EM
rds-also-tagged = TAMBÉM MARCADO
rds-sync = SINC
rds-no-sync = SEM SINC
rds-blocks-failed-their-check-plus = Blocos que falharam na verificação, mais os que o corretor de erros teve de reparar. As correções contam como erros de propósito: um número subindo é o primeiro sinal de que uma estação está prestes a parar de decodificar.
rds-front-end-overloading = FRONT END SOBRECARREGADO
rds-converter-clipping-which-puts-distortion = O conversor está cortando, o que espalha distorção por todo o multiplex — inclusive na subportadora de dados de 57 kHz. Reduza o ganho de RF, ou insira um atenuador: o RDS sai do ar cerca de um decibel antes de qualquer coisa ficar audível, então uma estação que soa perfeita pode ainda estar forte demais para que seus dados sobrevivam.
rds-group-types = TIPOS DE GRUPO
rds-recent-groups-newest-last = GRUPOS RECENTES — mais novos por último
rds-nothing-decoded-yet = nada decodificado ainda

## winlink (src/app/winlink.rs)

winlink-mail = E-MAIL
winlink-abort = ABORTAR
winlink-stop-session-progress = Interrompe a sessão em andamento
winlink-connect = CONECTAR
winlink-compose = ESCREVER
winlink-connecting = conectando…
winlink-not-connected-yet = ainda não conectado
winlink-loading = carregando…
winlink-date = DATA
winlink-from = DE
winlink-subject = ASSUNTO
winlink-no-subject = (sem assunto)
winlink-delete-message = Excluir esta mensagem
winlink-arch = ARQ
winlink-move-archive = Mover para o arquivo
winlink-select-message = selecione uma mensagem
winlink-reply = RESPONDER
winlink-delete = EXCLUIR
winlink-to = Para
winlink-cc = Cc
winlink-subject-2 = Assunto
winlink-callsign-smtp-someone-example-org = Um indicativo, ou SMTP:alguem@exemplo.org para e-mail da internet. Separe vários com vírgulas.
winlink-file-outbox = ARQUIVAR NA CAIXA DE SAÍDA
winlink-discard = DESCARTAR
winlink-filed-messages-go-out-next = as mensagens arquivadas saem na próxima sessão
winlink-mail-log = LOG DE E-MAIL
winlink-session-running = sessão em andamento…
winlink-no-session-yet = nenhuma sessão ainda

## wspr (src/app/panels/wspr.rs)

wspr-receptions = RECEPÇÕES
wspr-save-wspr-reception-list-csv = Salvar a lista de recepções WSPR como CSV
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = baliza de propagação de sinal fraco
wspr-time-left-two-minute-slot = Tempo restante neste slot de dois minutos. No fim dele, a gravação vai para o decodificador e a próxima decisão de transmissão é tomada.
wspr-tx = ● TX
wspr-decoding = decodificando…
wspr-band = Faixa
wspr-last-slot = Último slot
wspr-nothing-heard = nada ouvido
wspr-receive-only = somente recepção
wspr-next-slot = Próximo slot
wspr-waiting-radio-take-change = aguardando o rádio aceitar a mudança…
wspr-hop-next = Saltar em seguida
wspr-transmit = TRANSMITIR
wspr-receive-only-2 = Somente recepção.
wspr-power = POTÊNCIA
wspr-what-you-actually-radiate = o que você de fato irradia
wspr-goes-out-message-everyone-who = Isto sai na mensagem, e todos que o ouvem julgam o percurso por ela — então um número otimista aqui torna as medições deles erradas, além das suas.
wspr-roam = VARIAR
wspr-pick-different-offset-inside-200 = Escolhe um deslocamento diferente dentro da janela de 200 Hz a cada transmissão. Esta é a convenção: duzentos hertz compartilhados por todos só funcionam se ninguém se estacionar no meio. Desligado mantém onde está o cursor.
wspr-band-hop = SALTO DE FAIXA
wspr-move-dial-band-band-between = Move o dial de faixa em faixa entre os slots, para que um receptor amostre o espectro inteiro em vez de uma fatia dele. Girar o VFO você mesmo a pausa — pressione isto duas vezes para retomar.
wspr-upload = ENVIAR
wspr-send-what-station-decodes-wsprnet = Envia o que esta estação decodifica para o wsprnet.org. Informar o que você ouve é o que torna um receptor WSPR parte da rede; não coloca nada no ar.
wspr-who-heard-me = QUEM ME OUVIU
wspr-ask-wsprnet-org-every-few = Pergunta ao wsprnet.org a cada poucos minutos quais estações decodificaram esta. O WSPR não tem nenhuma forma de confirmação, então esta é a única maneira de uma baliza aprender algo sobre o próprio alcance.
wspr-bands = FAIXAS
wspr-none-selected-hopping-has-nowhere = nenhuma selecionada — o salto não tem para onde ir
wspr-set-your-callsign-grid-general = Defina o seu indicativo e locator na aba Geral das Configurações antes de transmitir.

## packet (src/app/panels/packet.rs)

packet-monitor = MONITOR
packet-busy = OCUPADO
packet-frames-arrived-but-failed-their = Quadros que chegaram mas falharam na verificação de sequência — uma colisão, um desvanecimento, ou um sinal fraco demais para ser lido.
packet-station-callsign-speed-tx-delay = Indicativo da estação, velocidade, atraso de TX, o caminho de digipeaters, a baliza e o servidor KISS
packet-starting-packet-modem = iniciando o modem de pacote…
packet-nothing-heard-yet = nada ouvido ainda
packet-terminal = TERMINAL
packet-connected = {" "}CONECTADO{" "}
packet-calling = {" "}CHAMANDO{" "}
packet-idle = {" "}OCIOSO{" "}
packet-extended-sequence-numbers-window-up = Números de sequência estendidos — uma janela de até 127 quadros.
packet-frames-sent-not-yet-acknowledged = Quadros enviados e ainda não confirmados pela outra ponta.
packet-retries-against-link-s-limit = Retentativas contra o limite deste enlace. Uma contagem subindo enquanto os quadros não confirmados ficam parados é o que um percurso com desvanecimento parece deste lado.
packet-mail-window-using-link = A janela E-MAIL está usando o enlace.
packet-one-radio-one-channel-one = Um rádio, um canal, um enlace. Uma sessão Winlink e este terminal são duas formas de usá-lo, e quem pedir primeiro fica com ele.
packet-digipeaters-nearest-first-separated-commas = Digipeaters, o mais próximo primeiro, separados por vírgulas — OE3XLR-1,OE3XMS-1. Deixe vazio para uma estação que você ouve diretamente, ou para usar o caminho padrão das configurações de pacote.
packet-disconnect = {" "}DESCONECTAR{" "}
packet-hang-up-properly-wait-far = Desligue corretamente, e espere a outra ponta concordar. Trocar de modo, em vez disso, encerra a sessão sem avisar ninguém.
packet-connect = {" "}CONECTAR{" "}
packet-mail-window-has-link-finish = A janela E-MAIL tem o enlace. Termine ou interrompa essa sessão primeiro.
packet-station-has-no-callsign-yet = Esta estação ainda não tem indicativo — defina um em CONFIG, acima do painel do monitor. Nada transmite até que seja definido.
packet-call-station-connected-mode-node = Chama esta estação em modo conectado — um nó, um BBS ou outro operador.
packet-needs-callsign-call = Precisa de um indicativo para chamar.
packet-clear = {" "}LIMPAR{" "}
packet-empty-transcript-link-untouched-connected = Esvazia a transcrição. O enlace não é afetado — uma estação conectada continua conectada.
packet-not-connected-type-callsign-above = Não conectado. Digite um indicativo acima e pressione CONECTAR para chamar um nó ou um BBS.
packet-type-here = digite aqui
packet-not-connected = não conectado
packet-send = {" "}ENVIAR{" "}
packet-send-line-node-bbs-reads = Envia a linha. Um nó ou um BBS lê uma linha por vez.

## fsq (src/app/panels/fsq.rs)

fsq-speed = Velocidade
fsq-call = Chamada
fsq-callsign-directed-fsqcall-messages-defaults = Indicativo para mensagens dirigidas (FSQCALL); usa o seu indicativo por padrão
fsq-tx = ● TX
fsq-contacts = CONTATOS
fsq-heard = OUVIDOS
fsq-none = — nenhum —
fsq-allcall = ALLCALL
fsq-image = IMAGEM
fsq-clear = LIMPAR
fsq-forget-pictures-received-so-far = Esquece as imagens recebidas até agora
fsq-send-image = Enviar imagem…
fsq-right-click-remove = Clique com o botão direito para remover
fsq-remove-picture = Remover esta imagem
fsq-listening = — escutando —
fsq-message = Mensagem…
fsq-send = {" "}ENVIAR{" "}
fsq-call-cq = {" "}CHAMAR CQ{" "}
fsq-heard-2 = {" "}? ouvido{" "}
fsq-clear-2 = {" "}LIMPAR{" "}
fsq-fsq-contacts = Contatos FSQ
fsq-add = Adicionar

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + ABA
publicsdr-open-receiver-new-radio-own = Abre este receptor como um novo rádio, em aba própria
publicsdr-point-radio-receiver-replacing-whatever = Aponta *este* rádio para o receptor, substituindo a interface em que ele está agora — perguntado de novo antes, porque isso equivale a configurar um rádio inteiro
publicsdr-not-stated = não informado
publicsdr-replace = SUBSTITUIR
publicsdr-point-radio-you-are-receiver = Aponta o rádio em que você está para este receptor
publicsdr-tab-instead = + ABA EM VEZ DISSO
publicsdr-open-receiver-another-radio-leave = Abre o receptor como outro rádio e deixa este em paz
publicsdr-cancel = CANCELAR
publicsdr-public-sdrs = SDRS PÚBLICOS
publicsdr-available = DISPONÍVEL
publicsdr-hide-receivers-are-full-ones = Oculta os receptores que estão lotados, e os cujo operador não abriu canais para aplicativos além de um navegador
publicsdr-band = NA FAIXA
publicsdr-only-receivers-cover-current-dial = Somente receptores que cobrem a frequência atual do dial
publicsdr-low-bw = BAIXA LARG
publicsdr-take-spyserver-low-bandwidth-shape = Usa um SpyServer na sua forma de baixa largura de banda: uma janela I/Q estreita que segue o dial, mais a visão de banda do próprio servidor, em vez de megabits de I/Q de banda larga. Sem efeito em um KiwiSDR, que só tem essa forma.
publicsdr-refresh = ⟳ ATUALIZAR
publicsdr-fetch-both-directories-again = Busca os dois diretórios de novo
publicsdr-name-place-antenna-band = nome, local, antena, faixa
publicsdr-clear-search = Limpar a busca
publicsdr-fetching-receiver-lists = Buscando as listas de receptores…
publicsdr-machine-radio-attached-does-not = A máquina à qual este rádio está conectado não responde a perguntas sobre dispositivos, então também não consegue buscar as listas.
publicsdr-no-match = nenhum resultado
publicsdr-nothing-matches-try-turning-filter = nada corresponde — tente reativar um dos filtros
publicsdr-radio = este rádio
publicsdr-radio-s-configuration-has-not = A configuração deste rádio ainda não chegou da máquina em que ele está, então não há nada a apontar para o receptor — use + ABA para isso.
publicsdr-position-not-published-so-nothing = {" "}A posição dele não é publicada, então nada que ele ouvir será informado.

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = aguardando o receptor…
hd-not-hd-radio-set-mode = Não está em HD Radio. Defina o modo como HD Radio em uma portadora de radiodifusão FM que traga os sidecars digitais — o dial fica no centro da portadora analógica, e não ao lado dela.
hd-no-hd-radio-lock-digital = Nenhum travamento de HD Radio. As bandas laterais digitais são transmitidas cerca de 20 dB abaixo da portadora analógica, então uma estação audível ainda pode ser ruidosa demais para decodificar: o decodificador precisa de alguns segundos em uma limpa.
hd-sync = SINC
hd-ofdm-frame-timing-recovered = Temporização de quadro OFDM recuperada
hd-audio = ÁUDIO
hd-audio-frames-decoding = Quadros de áudio decodificando
hd-quality = QUALIDADE
hd-lower = INFERIOR
hd-upper = SUPERIOR
hd-last-minute = último minuto
hd-waiting-locked-signal = aguardando um sinal travado
hd-mer-l = MER I
hd-modulation-error-ratio-lower-digital = Razão de erro de modulação da banda lateral digital inferior.
hd-mer-u = MER S
hd-modulation-error-ratio-upper-digital = Razão de erro de modulação da banda lateral digital superior.
hd-cber = CBER
hd-channel-bit-error-ratio-after = Razão de erro de bit do canal depois do código interno. Abaixo de cerca de 1e-3 o código externo consegue corrigi-la; acima disso o áudio começa a falhar.
hd-offset = DESLOCAMENTO
hd-residual-carrier-frequency-offset-large = Deslocamento residual da frequência da portadora. Grande e constante significa que a referência do receptor está fora, e não a transmissão.
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = Indicador de Modo de Serviço Primário: qual dos modos híbridos de serviço FM a estação está transmitindo.
hd-programme = PROGRAMA
hd-restricted-access = {" "}— acesso restrito

## bands (src/app/bands.rs)

bands-bands = FAIXAS
bands-daylight = ☀ DIA
bands-night = ☾ NOITE
bands-no-forecast-yet-fetching-hamqsl = · sem previsão ainda — buscando no HAMQSL.com
bands-band = FAIXA
bands-condx = CONDX
bands-paths = PERCURSOS
bands-reach = ALCANCE
bands-best = MELHOR
bands-ibp-beacons = BALIZAS IBP
bands-meteor-showers = CHUVAS DE METEORO
bands-peak = PICO

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = SPOTS
spots-longwave-shortwave-broadcast-stations-air = Estações de radiodifusão de ondas longas e curtas no ar agora
spots-view = NA TELA
spots-only-spots-inside-panadapter-span = Somente spots dentro da largura do panadaptador
spots-setup = ⚙ CONFIG
spots-feeds-lookup-upload-settings = Feeds, consulta e configurações de envio
spots-call-station-site-frequency = indicativo, estação, local, frequência
spots-clear-search = Limpar a busca
spots-no-match = nenhum resultado
spots-no-spots-enable-feed-setup = nenhum spot — ative um feed em ⚙ CONFIG
spots-nothing-matches-search = nada corresponde à busca

## ism (src/app/ism.rs)

ism-ism-devices = DISPOSITIVOS ISM
ism-decoding = DECODIFICANDO
ism-needs-wideband-iq-source-cat = Precisa de uma fonte I/Q de banda larga; um equipamento CAT envia somente áudio
ism-how-far-burst-must-stand = Quanto uma rajada precisa se sobressair acima do piso de ruído próprio do canal antes de ser decodificada
ism-give-each-band-width-normally = Dê a cada faixa a largura que ela normalmente precisa — um quarto de megahertz para as faixas OOK em 315, 345 e 433 MHz, uma inteira para 868 e 915.
ism-several-hundred-more-device-decoders = Várias centenas de decodificadores de dispositivos a mais, do projeto rtl_433, embutidos. Lê os dispositivos OOK de 433 MHz que os decodificadores próprios do sdroxide não alcançam de jeito nenhum. Onde os dois podem ler um dispositivo, o rtl_433 conhece muito mais variantes e assume.
ism-t-1-your-own = 1 seu
ism-device-decoders-registered-how-many = Decodificadores de dispositivos registrados, quantos vieram do seu rtl433_flex.conf, e quantas transmissões foram decodificadas desde que começou.
ism-reload-decoders = RECARREGAR DECODIFICADORES
ism-re-read-rtl433-flex-conf = Relê o rtl433_flex.conf depois de editá-lo. Os dispositivos já ouvidos continuam na lista.
ism-rtl-433-knows-more-variants = O rtl_433 conhece mais variantes destes do que os decodificadores próprios do sdroxide, então as lê enquanto escuta aqui. Escolha outra faixa e os próprios do sdroxide voltam a assumir.
ism-decoder-your-rtl433-flex-conf = Um decodificador no seu rtl433_flex.conf foi recusado. Os outros foram carregados. Corrija a linha e pressione RECARREGAR DECODIFICADORES.
ism-waiting-decoder = aguardando o decodificador…
ism-where-decoder-actually-listening-follows = Onde o decodificador está de fato escutando. Isto segue o centro de hardware do receptor, não o VFO — em um front end largo os dois podem estar bem distantes, e é a janela que decide quais canais são alcançáveis.
ism-transmissions-gate-opened-how-many = Transmissões em que a porta abriu, e quantas produziram um quadro válido. Muitas rajadas e nenhuma decodificação significa que a faixa está ocupada com dispositivos que o sdroxide ainda não consegue ler. Nenhuma significa que nada chega ao limiar — baixe-o, ou confira se o dial está na faixa.
ism-nothing-heard-yet = nada ouvido ainda
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

navtex-messages = MENSAGENS
navtex-sync = SINC
navtex-character-phase-locked-signal-being = A fase do caractere está travada — um sinal está sendo lido.
navtex-characters-whose-first-copy-was = Caracteres cuja primeira cópia estava corrompida e foram tirados da repetição cinco slots depois, e aqueles em que as duas estavam ruins. Um caractere NAVTEX é enviado duas vezes; isso é toda a sua correção de erro.
navtex-swap-mark-space-tones-signal = Troca os tons de marca e espaço, para um sinal recebido na outra banda lateral. Desligado é a banda lateral superior na frequência do canal, que é o que todas as instruções publicadas de sintonia do serviço dizem.
navtex-starting-navtex-receiver = iniciando o receptor NAVTEX…
navtex-nothing-yet-station-transmits-ten = nada ainda — uma estação transmite por dez minutos a cada quatro horas
navtex-cut-short = interrompida
navtex-message = MENSAGEM
navtex-time-day-stated-message-utc = Hora do dia informada na mensagem (UTC): somente exibição
navtex-receiving = RECEBENDO
navtex-monitor = MONITOR
navtex-everything-decoded-message-not = tudo decodificado, seja mensagem ou não

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = PINTURA RF
rfpaint-scan-speed = Velocidade de varredura
rfpaint-how-fast-text-image-scanned = Com que rapidez o texto/imagem é varrido na cascata. Mais baixo é mais lento e mais legível; 100% = taxa base, 25% (centro) é o padrão.
rfpaint-abort = Abortar
rfpaint-text-paint = PINTURA DE TEXTO
rfpaint-type-text-paint = Digite o texto para pintar…
rfpaint-preview-waterfall = PRÉVIA NA CASCATA
rfpaint-type-text-preview = digite um texto para visualizar
rfpaint-transmit = {"  "}TRANSMITIR{"  "}
rfpaint-image-paint = PINTURA DE IMAGEM
rfpaint-no-image-loaded = nenhuma imagem carregada
rfpaint-load-image = Carregar imagem…
rfpaint-load-image-preview = carregue uma imagem para visualizar

## hfdl (src/app/hfdl.rs)

hfdl-listen = ESCUTAR
hfdl-switches-engine-s-hfdl-downconverter = Liga ou desliga o conversor de descida e o decodificador HFDL do motor. A decodificação custa uma faixa de 24 kHz e uma thread de trabalho, esteja a janela aberta ou não.
hfdl-channel-assigned-frequency-listen-anything = O canal (a frequência atribuída) em que escutar. Qualquer coisa na faixa funciona — as estações HFDL transmitem de 2,8 a 22 MHz — contra uma faixa fixa de 24 kHz centrada aqui.
hfdl-save-hfdl-decode-log-file = Salvar o log de decodificação HFDL em um arquivo
hfdl-riverhead-northern-atlantic-north-america = Riverhead (Atlântico Norte / América do Norte) — o padrão, e o canal em que o decodificador foi validado.
hfdl-hfdl-assigned-frequency = Uma frequência atribuída HFDL.
hfdl-one-your-saved-channels-click = Um dos seus canais salvos — clique para sintonizar, clique com o botão direito para remover.
hfdl-hfdl-selected-but-decoding-off = HFDL está selecionado, mas a decodificação está desligada — pressione ESCUTAR acima
hfdl-decodes = DECODIFICAÇÕES
hfdl-nothing-decoded-yet-ground-station = Nada decodificado ainda — o squitter de uma estação de solo se repete a cada ~32 s assim que há uma no canal.
hfdl-no-decode-matches-filter = Nenhuma decodificação corresponde ao filtro.
hfdl-decoding = DECODIFICANDO
hfdl-decoding-off = DECODIFICAÇÃO DESLIGADA
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = Voz digital FreeDV V1
rade-tx = ● TX
rade-mute-analog = SILENCIAR ANALÓGICO
rade-mute-demodulated-audio-so-only = Silencia o áudio demodulado, para que só a fala decodificada seja ouvida. De outro modo, o sinal bruto passa sempre que o modem não tem o que tocar — esse chiado é como se encontra uma transmissão antes de ela sincronizar, então deixe isto desligado ao sintonizar.
rade-sync = SINC
rade-how-far-received-signal-sits = Quão longe o sinal recebido está de onde o modem o espera. Valores grandes ainda decodificam — o laço de aquisição os acompanha —, mas empurrar o dial para trazer isto perto de zero dá a melhor margem.
rade-snr = SNR —
rade-callsign-station-put-end-over =
    O indicativo que a estação colocou no quadro End-of-Over — o único ponto de uma transmissão RADE em que um é enviado. É também o que é informado ao FreeDV Reporter, então uma estação que você ouve aparece no qso.freedv.org como ouvida por você.

    Held for a minute after the signal goes, and dropped when you start an over of your own: it names the station on frequency, not every station the receiver has ever heard.
rade-stop-talking = PARAR DE FALAR
rade-talk-ptt = FALAR (PTT)
rade-open-close-rade-over-modem = Abre ou fecha uma transmissão RADE. O modem precisa de cerca de 120 ms de fala antes que o primeiro quadro saia, e envia um quadro de fim de transmissão quando você para, então a transmissão continua um pouco depois do botão.
rade-rade-v1-occupies-roughly-1060 = RADE V1 ocupa cerca de 1060–1880 Hz da banda de passagem — banda lateral superior, ou inferior em 160, 80 e 40 m, onde o RADE mantém a prática de fonia e o sdroxide troca para você. Coloque o sinal dentro da faixa sombreada na cascata; o modem o encontra a partir daí.

## frame (src/app/frame.rs)

frame-acknowledge = Confirmar
frame-dismiss = Dispensar
frame-reconnecting = Reconectando…
frame-not-connected = Não conectado.
frame-reconnect-now = Reconectar agora
frame-transmit-lockout-disabled = ⚠ BLOQUEIO DE TRANSMISSÃO DESATIVADO
frame-engine-was-started-oob-tx = Este motor foi iniciado com --oob-tx. O bloqueio das faixas de amador está desligado: ele acionará o transmissor em qualquer frequência que o hardware suporte.
frame-transmitting-outside-your-licence-offence = Transmitir fora da sua licença é infração em todo país que emite uma. Continue somente se você estiver autorizado a usar as frequências em que está prestes a acionar — uma licença MARS/CAP ou comercial, uma autorização experimental, ou uma carga fantasma.
frame-i-understand = {"  "}ENTENDO{"  "}
frame-restart-without-oob-tx-put = Reinicie sem --oob-tx para restaurar o bloqueio.

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC — com que força o transmissor está sendo excitado, em porcentagem da potência máxima que ele pode entregar. 100 % é a escala total, não a potência total em watts: quanto isso vale na antena depende do rádio e da Excitação.
smeter-radio-reports-no-forward-power =


    This radio reports no forward power, so the figure beside ALC is the ALC reading again rather than watts.
smeter-no-swr-radio-has-no =


    No SWR: this radio has no bridge to measure it with, and sdroxide will not invent one — it can only show what the radio itself reports. A rig with an SWR meter (over CAT, TCI or a LAN link) and an HPSDR board both fill this in; a plain SDR transmitter cannot.
smeter-no-signal-report-yet-receiver = Nenhum relatório de sinal ainda — o receptor não entregou uma leitura de medidor.
smeter-s9-73-dbm-each-s =


    S9 is −73 dBm and each S-unit below it is 6 dB. The dBm figure is only as honest as the front end's calibration — an uncalibrated receiver reports dBFS with a dBm label.
smeter-ovld-converter-into-rails-so =


    OVLD: the converter is into its rails, so the level shown understates the signal. Wind the front-end gain back.
smeter-table-held = , tabela mantida
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● TX
multi-link = Potência
multi-mute-radio-s-audio = Silenciar o áudio deste rádio
multi-close-radio-s-split-view = Fechar a visão dividida deste rádio
multi-open-radio-split-view-own = Abrir este rádio em uma visão dividida própria
multi-already-open-another-split-view = Já aberto em outra visão dividida
multi-show-radio-here = Mostrar este rádio aqui
multi-add-radio-computer-settings-radio = Adicionar um rádio neste computador (Configurações → Rádio para adicionar um em uma estação)
multi-add-radio = Adicionar um rádio
multi-asked-station-another-radio = Pedi à estação outro rádio…
multi-client-cannot-open-connection = Este cliente não consegue abrir uma conexão.

## net (src/app/net.rs)

net-fail = FALHA
net-downloading = Baixando…
net-adif-import-failed-file-could = Falha na importação ADIF: o arquivo não pôde ser lido
net-channel-import-failed-file-could = Falha na importação de canais: o arquivo não pôde ser lido
net-channel-import-nothing-read-chirp = Importação de canais: nada para ler — um arquivo CSV do CHIRP começa com uma linha de cabeçalho com os nomes das colunas, uma das quais deve ser Frequency

## widgets (src/app/panels/widgets.rs)

widgets-free-text = texto livre
widgets-hashed-callsign-not-yet-heard = indicativo com hash, ainda não ouvido por inteiro
widgets-entity-unknown = entidade desconhecida
widgets-band = esta faixa
widgets-new-entity-never-worked-any = Entidade nova — nunca trabalhada, em nenhuma faixa
widgets-new-grid-square = Novo quadrado locator
widgets-not-worked-before = Não trabalhado antes
widgets-worked-before-but-not-band = Trabalhado antes, mas não nesta faixa
widgets-call-queue = Na fila de chamadas
widgets-image = Imagem

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = RECEPÇÕES
pi4-save-pi4-reception-list-csv = Salvar a lista de recepções PI4 como CSV
pi4-next-generation-beacon = Beacon de Próxima Geração
pi4-time-left-one-minute-beacon = Tempo restante neste ciclo de baliza de um minuto. A mensagem PI4 é procurada assim que a janela teve tempo de se encher além de onde ela terminaria.
pi4-decoding = decodificando…
pi4-band = Faixa
pi4-last-cycle = Último ciclo
pi4-nothing-heard = nada ouvido
pi4-receive-only-tune-so-beacon = Somente recepção. Sintonize para que a identificação CW da baliza fique em 800 Hz de áudio — a convenção própria da rede — e os quatro tons PI4 caiam onde este decodificador os procura por padrão.

## awards (src/app/awards.rs)

awards-awards = PRÊMIOS
awards-band = Faixa
awards-all = Todas
awards-dxcc = DXCC
awards-grids = Locators
awards-worked-all-states = Trabalhou Todos os Estados
awards-cq-zones-waz = Zonas CQ (WAZ)
awards-dxcc-entities = Entidades DXCC

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = Áudio na banda de passagem do decodificador.
acars-blocks-framed-blocks-whose-check = Blocos enquadrados, e blocos cuja sequência de verificação falhou.
acars-starting-acars-receiver = iniciando o receptor ACARS…
acars-channel = CANAL
acars-tune-dial-here = Sintonize o dial aqui.

acars-channel-out-of-range = Fora da faixa de recepção deste rádio.
acars-nothing-yet-acars-quiet-between = Nada ainda. O ACARS fica silencioso entre aeronaves — deixe-o em um canal e aguarde uma rajada.
acars-check-failed = verificação falhou
acars-block-check-sequence-did-not = A sequência de verificação do bloco não coincidiu — o texto pode estar errado.
acars-no-text = (sem texto)

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = Usuário
login-password = Senha
login-remember-device = Lembrar neste dispositivo
login-signs-without-asking-next-time = Entra sem perguntar na próxima vez, e deixa a aba da visão 3D entrar sem perguntar de jeito nenhum. A senha fica guardada em texto puro, então desligue isto em uma máquina que outras pessoas usam.
login-keeps-password-device-clear = Mantém a senha neste dispositivo, em texto puro.
login-checking = {" "}VERIFICANDO…{" "}
login-sign = {" "}ENTRAR{" "}
login-wrong-password-shuts-server-s = Uma senha errada fecha a porta do servidor por alguns segundos, então isto pode levar um momento.

## help (src/help.rs)

help-find = BUSCAR
help-find-manual = buscar no manual
help-previous-match-shift-enter-shift = Ocorrência anterior (Shift+Enter, Shift+F3)
help-next-match-enter-f3 = Próxima ocorrência (Enter, F3)
help-no-matches = nenhuma ocorrência
help-clear-search-esc = Limpar a busca (Esc)
help-sdroxide-manual = MANUAL DO SDROXIDE
help-contents = CONTEÚDO

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = 1 estação ouvida — aguardando que ela informe uma posição
aismap-double-click-reframe = clique duas vezes para reenquadrar
aismap-zoom-out-wheel-pinch-do = Reduzir o zoom — a roda ou um pinça também fazem isto
aismap-zoom-about-middle-chart = Ampliar em torno do meio da carta
aismap-frame-everything-being-tracked-again = Enquadrar tudo o que está sendo rastreado de novo — um clique duplo também faz isto

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = aguardando o espectro…
specview-hounds = CÃES

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = Esta janela não tem renderizador de GPU, então a cascata fica na largura padrão.
spectrum-machine-drawing-without-gpu-every = Esta máquina desenha sem GPU — cada coluna da cascata é trabalho do processador, que já o divide com o rádio.
spectrum-window-renders-through-opengl-which = Esta janela renderiza por OpenGL, que é o caminho de compatibilidade do sdroxide — um Raspberry Pi, um chip gráfico antigo, ou um navegador sem WebGPU. Uma cascata mais larga não vale a taxa de quadros nesses casos.
spectrum-wider-than-renderer-can-draw = Mais larga do que este renderizador consegue desenhar.

## macros (src/app/panels/macros.rs)

macros-nocall = SEMCHAMADA
macros-remove-button = Remover este botão

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = RASTREADOR DE LOCATORES
gridtracker-drag-pan-wheel-zoom-amber = Arraste para deslocar, a roda para ampliar. Âmbar = trabalhado, verde = confirmado, ciano = ouvido.

## remote (src/remote.rs)

remote-server-busy-another-client-connected = servidor ocupado — outro cliente está conectado
remote-transmitting-but-client-s-microphone = Transmitindo, mas o microfone deste cliente não produz áudio nenhum — nada está sendo modulado. Em um navegador, confira se a página recebeu permissão para o microfone e se nenhum outro aplicativo o está usando.
remote-connection-closed = conexão encerrada

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = Visão 3D do sistema solar — Sol, Terra, Lua, manchas solares e CMEs (janela separada)
solarbtn-solar-system-3d-view-sun-2 = Visão 3D do sistema solar — Sol, Terra, Lua, manchas solares e CMEs (nova aba do navegador)

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = Carregando a voz…
speech-browser-client-cannot-speak-yet = o cliente de navegador ainda não consegue falar

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = Clique duas vezes para digitar uma frequência
freqdisplay-tap-type-frequency = Toque para digitar uma frequência

## solarstate (src/solar3d/state.rs)

solarstate-home = INÍCIO
solarstate-dwarf-planets = PLANETAS ANÕES

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = clique duas vezes para reenquadrar

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = clique duas vezes para reenquadrar

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = conexão encerrada

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = não foi possível iniciar a thread de alerta

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = nenhuma aeronave localizada ainda

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = CLIQUE DUAS VEZES PARA REENQUADRAR

## chrome (src/chrome.rs)



## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = Define a configuração de potência própria do FDM-DUO (menu 34 TX POWER). Excitação e Tune então definem a potência dentro dela, e o SDRoxide envia a configuração de novo a cada conexão.

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $day } { $month ->
        [1] jan
        [2] fev
        [3] mar
        [4] abr
        [5] mai
        [6] jun
        [7] jul
        [8] ago
        [9] set
        [10] out
        [11] nov
       *[12] dez
    }

## download (src/download.rs)

download-browser-could-not-read-file = o navegador não conseguiu ler esse arquivo

## morse (src/app/morse.rs)

morse-translate = TRADUZIR
morse-practice = PRATICAR
morse-learn = APRENDER
morse-morse = MORSE
morse-text-morse = Texto → Morse
morse-morse-text = Morse → Texto
morse-per-character-space-between-characters = `.` e `-` por caractere, um espaço entre caracteres, ` / ` entre palavras. O alfabeto ITU, dígitos, pontuação e os prosignos comuns.
morse-reference-translator-available-desktop-build = O tradutor de referência está disponível na versão para desktop.
morse-play-what-you-type-your = Toca o que você digita, nos seus próprios alto-falantes
morse-play = TOCAR
morse-stop = PARAR
morse-every-character-learned-well-done = Todos os caracteres aprendidos — muito bem.
morse-reveal = REVELAR
morse-reset = REDEFINIR
morse-replay = REPETIR
morse-what-did-you-hear = O que você ouviu?
morse-check = VERIFICAR
morse-press-new-hear-character = Pressione NOVO para ouvir um caractere.
morse-tone = Tom
morse-speed = Velocidade
morse-spacing = Espaçamento
morse-characters-go-out-speed-above = Os caracteres saem na velocidade acima; esta é a velocidade em que os *intervalos* são enviados, para que um iniciante ouça a letra inteira antes de conseguir enviá-la nessa velocidade.
morse-playback-available-desktop-build = A reprodução está disponível na versão para desktop.

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = altitude
adsb-row-track = rota
adsb-row-turning = fazendo curva
adsb-row-squawk = squawk
adsb-row-range = alcance
adsb-row-position = posição
adsb-row-signal = sinal
adsb-row-frames = quadros
rade-transmitting = transmitindo
decodes-odd = ÍMPAR
decodes-new = NOVA
js8-new = NOVA

## AIS panel: vessel detail rows.

ais-row-status = estado
ais-row-speed = velocidade
ais-row-course = rumo
ais-row-heading = proa
ais-row-turning = fazendo curva
ais-row-altitude = altitude
ais-row-size = tamanho
ais-row-draught = calado
ais-row-destination = destino
ais-row-range = alcance
ais-row-signal = sinal
ais-row-type = tipo
ais-row-position = posição
ais-row-messages = mensagens
ism-off = DESLIGADO
textmodem-frequency-shifted = deslocado em frequência
winlink-to-heading = PARA

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = { $heard } ouvidas — nenhuma posição mais recente que { $drop_map_s } s
aismap-stations-heard-waiting-position-report = { $heard } estações ouvidas — aguardando um relatório de posição
aismap-km-across = { $lon_span }° · ~{ $km } km de largura

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = alarmando em { $device }
alerts-alarm-device-unavailable = dispositivo de alarme indisponível: { $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = { $w } trabalhadas
awards-confirmed = { $value } confirmadas

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · previsão do HAMQSL.com, de { $value } atrás
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · próxima baliza em { $now } s
bands-radiants-placed = radiantes posicionados para { $lat }°, { $lon }°
bands-zhr = ZHR { $zhr }
bands-radiant = radiante { $alt_deg }° { $az_deg }
bands-radiant-down = radiante abaixo ({ $alt_deg }°)

    A radiant above the horizon means the trails can reach you; a fast shower leaves longer-lived ionised trails for meteor scatter on 6 m and 2 m, and the brief 10 m openings.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } não decodificando
drm-symbols = { $value } símbolos · { $frame }
drm-not-decodable = { $value } — não decodificável
drm-broadcaster-s-clock-utc = relógio da emissora { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    MAX: a potência total do FDM-DUO, cerca de 8 W. A configuração a usar com o HR500 na linha.

    { $base }
duopower-t-5-w-hr500-line-will =
    5 W. Com o HR500 na linha, ele não atingirá a potência total na maioria das faixas com esta configuração: use MAX, e Excitação para reduzi-la.

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } W, para operar sem amplificador ou em QRP.

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = Reativa a transmissão após { $swr }:1. Verifique a antena primeiro: se a falha continuar, a próxima transmissão também para.
frame-reconnecting-s = Reconectando em { $value }s…

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = { $worked } trabalhados
gridtracker-confirmed = { $confirmed } confirmados
gridtracker-heard = { $heard } ouvidos

## hd: messages with values (src/app/hd.rs)

hd-decode = Decodificar { $value } ({ $codec_label })

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } kHz já tem um chip
hfdl-save-current-channel-khz-one = Salvar o canal atual ({ $cur_khz } kHz) como um dos seus próprios chips
hfdl-aircraft = { $aircraft } aeronaves
hfdl-bursts = { $bursts } rajadas
hfdl-decodes-2 = { $decodes } decodificações
hfdl-fixed = { $fec } corrigidas

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint } — ainda não decodificado
ism-listen-tunes-radio-there-one = Escutar em { $value } — sintoniza o rádio ali. Uma faixa por vez: elas estão distantes demais para qualquer receptor cobrir duas ao mesmo tempo.
ism-watch-hz-around-band-centre = Observa { $value } Hz em torno do centro da faixa, seja o que for que a faixa pediria. Mais estreito serve a um receptor que não consegue entregar a largura própria da faixa; mais largo alcança dispositivos mais afastados do centro. O seu receptor precisa entregar cerca de um terço a mais que isto.
ism-rtl-433 = rtl_433 — { $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } kHz
ism-your-own = { $flex } seu
ism-decoders-decoded = { $decoders } decodificadores ({ $flex }), { $decodes } decodificados
ism-handled-rtl-433 = tratado pelo rtl_433: { $names }
ism-flex = flex: { $error }
ism-window-mhz-mhz-wide = janela { $mhz } MHz, { $e62 } MHz de largura
ism-bursts-decoded = { $bursts } rajadas, { $decodes } decodificadas
ism-devices = { $ism_reports } dispositivos
ism-id-mhz-db-heard-first =
    { $protocol } id { $device }
    { $e6 } MHz { $snr_db } dB ouvido { $value }×
    ouvido pela primeira vez há { $first_at }

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = Confirmações: { $recs } baixadas, { $matched } recém-confirmadas
logbook-qso = { $qso_log } QSO
logbook-worked-before = ⚠ TRABALHADA ANTES ({ $dupe_band })
logbook-utc-qso = { $oldest }–{ $newest } UTC · { $value } QSO

## app: messages with values (src/app/mod.rs)

app-radio = rádio { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = não foi possível iniciar a thread de áudio Morse: { $error }
morse-no-audio-output = sem saída de áudio: { $error }
morse-about-s = cerca de { $farnsworth_wpm } s
morse-characters-play = Caracteres em uso: { $set }
morse-score-run = Pontuação: { $correct } / { $total }. Sequência: { $streak } de { $value }.
morse-correct = correto — { $value }
morse-was = era { $value }

## net: messages with values (src/app/net.rs)

net-adif-import-failed = Falha na importação ADIF: { $error }
net-not-unicode-read = {" "}(não Unicode; lido como { $enc })
net-received-reports-swl-left-out = , { $n } relatórios recebidos (SWL) deixados de fora
net-adif-import-added-duplicates-skipped = Importação ADIF: { $added } acrescentados, { $skipped } duplicados ignorados{ $reports }{ $assumed }
net-channel-import-failed = Falha na importação de canais: { $error }
net-line-s-skipped = , { $n } linha(s) ignorada(s)
net-not-unicode-read-2 = {" "}(não Unicode; lido como { $enc })
net-channel-import-channel-s-read = Importação de canais: { $n } canal(is) lido(s){ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = { $frames } quadros · { $bad } ruins
acars-mode = modo { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = { $aircraft } aeronaves
adsb-frames = { $frames } quadros
adsb-preambles = { $preambles } preâmbulos
adsb-bad-crc = { $bad_crc } CRC ruins
adsb-tune = SINTONIZAR { $mhz }
adsb-ft-barometric = { $ft } pés barométricos

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = { $vessels } embarcações
ais-messages = { $messages } mensagens
ais-slots = { $bursts } slots
ais-bad-fcs = { $bad_fcs } FCS ruins
ais-ais-mhz = AIS { $value } ({ $mhz } MHz) — { $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value } ({ $mhz } MHz) — { $bursts } slots, { $messages } mensagens, piso { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = SINTONIZAR { $mhz }
ais-ships-heard-are-khz-off = As embarcações ouvidas estão { $khz } kHz fora de frequência ({ $mhz } ppm em 162 MHz) — ajuste a correção de frequência do front end, ou as transmissões acima de cerca de 5 kHz deixarão de decodificar
ais-over-ground = { $value }° sobre o solo
ais-dbfs-db-snr = { $rssi_dbfs } dBFS, { $snr_db } dB de SNR
ais-last-type-ais = { $messages } (último tipo { $last_type }, AIS { $channel })
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } MHz — canal marítimo { $ch }. Um navio alterna entre os dois, então desligar um reduz pela metade a frequência com que cada embarcação é ouvida.

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = objeto de { $reported_by }
aprs-via = via { $via }
aprs-object-reported = objeto informado por { $reported_by }
aprs-digit-s-blanked = ± { $ambiguity } dígito(s) apagado(s)
aprs-wind-m-s = vento { $value }° { $value2 } m/s
aprs-gust = rajada { $value }
aprs-rain-mm-h = chuva { $value } mm/h
aprs-frame-s-last-ago = { $packets } quadro(s), último há { $last_heard }
aprs-via-2 = via { $via }
aprs-bad = { $bad_frames } ruins
aprs-other = { $non_aprs } outros
aprs-sent = enviado ×{ $tries }
aprs-stn = { $stations } est.
aprs-queued = { $tx_queue } na fila

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = mestre { $value }
atchat-last-heard-s-ago-click = { $status } — última escuta há { $age_s }s · clique para abrir uma aba de mensagem direta
atchat-close-tab = fechar a aba de { $peer }
atchat-direct-message = mensagem direta para { $value }
atchat-cannot-display = não é possível exibir { $path }

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Click for the { $neural } decoder. The neural one copies further down and reads hand-sent CW a timing fit will not accept; the timing one is the only one that copies the accented letters — Ä, Ö, Å, Ü, É — because the model has no output class for them.
cw-idle-s = OCIOSO { $idle }s

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = Salvar as { $n } decodificações da lista como ADIF — relatórios recebidos, não contatos. Decodificações que não nomeiam remetente ficam de fora.
decodes-save-decodes-list-csv-one = Salvar as { $n } decodificações da lista como CSV, uma linha cada
decodes-session-qso = Sessão: { $session } QSO
decodes-save-whole-logbook-qso-adif = Salvar o diário inteiro ({ $logged } QSO) como ADIF
decodes-save-whole-logbook-qso-text = Salvar o diário inteiro ({ $logged } QSO) como texto
decodes-hz-slots = { $audio_hz } Hz · { $odd } slots
decodes-your-slot-timing-against-stations =
    A sua temporização de slot em relação às estações que você ouve.
    { $after }

    It covers the whole receive path, so a slow audio or network chain counts the same as a wrong clock. Under 0.5 s is comfortable.
decodes-pile-up = PILE-UP { $fox_queue }
decodes-queue = FILA { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } dB · { $audio_hz } Hz{ $unwrap_or_default }
    Clique para remover

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = Todas as velocidades estão sendo decodificadas; ▸ marca { $speed }, a em que você transmite
js8-every-speed-being-decoded-click = Todas as velocidades estão sendo decodificadas. Clique para transmitir em { $speed }
js8-transmit-decode = Transmitir e decodificar em { $speed }
js8-beaconing-every-min-click-stop = Transmitindo baliza a cada { $hb_min } min — clique para parar
js8-last-beacon-went-out-hz = A última baliza saiu em { $hz } Hz — um slot livre na sub-faixa de heartbeat de { $value }–{ $value2 } Hz, escolhido para ficar longe dos sinais que estão sendo decodificados.
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } quadros · { $left }s
js8-decoded = Decodificado em { $speed }
js8-frames = … ({ $frames } quadros)
js8-reply-2 = Responder a { $value }: “{ $value2 }”
js8-address-composer = Endereçar o compositor a { $value }

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i }: envia “{ $text }”
macros-sends = Envia “{ $text }”
macros-most = { $value } é o máximo

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = { $repaired } reparados · { $lost } perdidos
navtex-characters-lost = { $lost } caracteres perdidos

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = { $bad_frames } ruins
packet-link-layer = A camada de enlace está em { $link }
packet-via = {" "}via { $via }
packet-unacked = { $unacked } sem confirmação
packet-retry = retentativa { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] baliza
       *[other] balizas
    }
pi4-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } atrás

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = SNR { $snr_db } dB
rade-offset-hz = deslocamento { $freq_offset_hz } Hz
rade-heard = ouvido { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ { $dropped } amostras descartadas entre o receptor e o decodificador — esta máquina não está acompanhando a decodificação neural.

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } caract./s
textmodem-char-s-hz-wide = { $value } — { $chars_per_sec } caract./s, { $bandwidth_hz } Hz de largura, { $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = { $frames } quadros
vdl2-bursts = { $bursts } rajadas
vdl2-sync = { $syncs } sincronismo
vdl2-hdlc-bad = { $hdlc_bad } HDLC ruins
vdl2-bad-fcs = { $fcs_bad } FCS ruins
vdl2-mhz-bursts-frames-noise-floor = { $mhz } MHz, { $what } — { $bursts } rajadas, { $frames } quadros, piso de ruído { $floor_dbfs } dBFS
vdl2-tune = SINTONIZAR { $mhz }
vdl2-held-2 = RETIDO +{ $since }
vdl2-use-worldwide-where-every-link = { $what } — em uso no mundo todo, e onde todo enlace começa. A que manter se você mantiver só uma.
vdl2-assigned-which-channels-carry-anything = Atribuído a um { $what }. Quais canais transportam algo depende de onde você está: deixe todos ligados, a menos que saiba de outra forma.
vdl2-mhz-db-snr-dbfs-evm = { $mhz } MHz · { $snr_db } dB SNR · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Hz · { $rs_corrected } símbolos RS corrigidos
vdl2-characters = { $parity_errors } caracteres
vdl2-parameter-sdroxide-does-not-read = { $unknown } { $unknown ->
        [one] parâmetro
       *[other] parâmetros
    } que o SDRoxide não lê — mostrados acima como identificador e bytes

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = { $live_size } linhas retidas
wefax-pixels-per-line = { $width } pixels por linha
wefax-shift-picture-pixels = Desloca a imagem { $px } pixels
wefax-charts-are-saved-click-copy =
    As cartas são salvas { $place }

    Click to copy the path
wefax-finished-charts-are-saved-collect = As cartas concluídas são salvas { $place } e se acumulam aqui.
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } pixels
    { $value }

    Right-click to delete
wefax-older-store = { $older } mais antigas no armazenamento
wefax-older-load-more = { $older } mais antigas — carregar mais
wefax-published-carrier-khz-dial-khz = Portadora publicada { $f } kHz → dial { $value } kHz USB
wefax-delete-chart-2 = Excluir esta carta { $dir }

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · zona CQ { $cq_zone } · zona ITU { $itu_zone }
widgets-new-entity = Entidade nova em { $band_label }
widgets-worked-before = Trabalhado antes em { $band_label }
widgets-calling-cq-includes-you = Chamando CQ { $target } — inclui você
widgets-calling-cq-not-aimed-you = Chamando CQ { $target } — não dirigido a você

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = { $heard_us } rx · { $heard_us2 } ouviram a gente
wspr-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] baliza
       *[other] balizas
    }
wspr-beaconing-duty = transmitindo baliza — { $wspr_power_dbm }, ciclo de { $wspr_tx_percent }%
wspr-listening-duty = escutando (ciclo de { $wspr_tx_percent }%)
wspr-beacon-one-two-minute-slot = Baliza em um slot de dois minutos a cada { $pct } — { $pct2 }% deles, espaçados igualmente — em { $wspr_power_dbm }. O slot em que o ciclo começa é sorteado a partir do seu indicativo, então duas estações rodando este programa não começam juntas.
wspr-transmitting-set-general-tab-settings = Transmitindo como { $call } em { $sent } — definido na aba Geral das Configurações.
wspr-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } atrás

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    antena: { $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = { $entries } receptores · { $per_net } · buscado há { $age }
publicsdr-showing-search-narrow = mostrando { $shown } de { $n } — pesquise para restringir
publicsdr-match = { $n } resultado
publicsdr-replace-taking-receiver-points-same =
    Substituir { $this } por { $entry }?

    { $this2 } is on { $iface }. Taking this receiver points the same tab at { $address } instead, renames it “{ $entry2 }”, and holds the dial to the { $range_label } the receiver publishes — nothing here transmits.

    The { $iface2 } settings stay where they are, and so do the ranges stated for them: switching the interface back in Settings → Radio brings the radio back as it was.
publicsdr-receptions-will-reported = {" "}As recepções serão informadas a partir de { $value }.
publicsdr-pointing-radio = Apontando este rádio para { $entry }…{ $site }
publicsdr-opening-another-radio = Abrindo { $entry } como outro rádio…

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } kHz
qo100-before-switching-tune-so-beacon = Antes de LIGAR: sintonize para que os dois lóbulos da baliza fiquem entre +{ $park_lo_hz } e +{ $park_hi_hz } kHz na faixa (a pista sombreada).
qo100-receiver-currently-listening-around-mhz = O receptor está escutando em torno de { $mhz } MHz — longe da baliza de 10489,750 MHz, então não há nada aqui para desenhar. O decodificador continua pesquisando de qualquer forma.
qo100-receiver-currently-covers-only-khz = o receptor cobre só ±{ $khz } kHz aqui — o resto da faixa fica em branco
qo100-null-db-sym-snr-db = nulo { $est_null_depth_db } dB sim { $est_symmetry } snr { $est_snr_db } dB
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Hz/s { $est_drift_accel_hz_s2 } Hz/s² (desrotacionado antes da decodificação)
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Hz/s (desrotacionado antes da decodificação)
qo100-found-empty = { $est_updates } encontradas, { $est_misses } vazias
qo100-last-s-ago = {"   "}última { $auto_last_hz } (há { $auto_last_unix }s)
qo100-over-correction = { $auto_total_hz } em { $auto_applies } { $auto_applies ->
        [one] correção
       *[other] correções
    }{ $last }
qo100-sync-32-err = sincronismo { $sync_bit_errors } / 32 erros ×{ $sync_matches }
qo100-frame-buffer = buffer de quadros { $frame_fill }%
qo100-blocks-tried-decoded = { $blocks_tried } blocos tentados, { $blocks_locked } decodificados
qo100-last-applied-s-ago-hz = última aplicada há { $ago }s: { $old } → { $new } Hz ({ $old2 } Hz)
qo100-locked-block-tried-locked = travado — { $blocks_tried } { $blocks_tried ->
        [one] bloco tentado
       *[other] blocos tentados
    }, { $blocks_locked } travados
qo100-searching-block-tried-none-locked = pesquisando — { $blocks_tried } { $blocks_tried ->
        [one] bloco tentado
       *[other] blocos tentados
    }, nenhum travado ainda

## rds: messages with values (src/app/rds.rs)

rds-auto = Automático ({ $standard })
rds-type = tipo { $class }
rds-groups = · { $groups } grupos
rds-blocks-error = · { $ber } % dos blocos com erro
rds-clean-corrected-lost = { $blocks_ok } limpos · { $blocks_corrected } corrigidos · { $blocks_bad } perdidos

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC (local { $offset })
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } acima do horizonte
sat-min = em { $value } min
sat-h-min = em { $value } h { $value2 } min
sat-searching = pesquisando… { $next } de { $entries }
sat-locked-2 = ● TRAVADO — { $t }
sat-pass-until-max = Passagem até { $set_unix } · máx. { $max_el }°
sat-next-pass-rises-max = Próxima passagem { $rise_unix } { $rise_unix2 } ({ $now }) · sobe em { $rise_az }° { $rise_az2 } · máx. { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ { $clients } { $clients ->
        [one] cliente rigctld conectado
       *[other] clientes rigctld conectados
    } — um software de satélite comandando o dial ali corrigiria o Doppler duas vezes
sat-rotator-antenna-az-el = Rotor: antena em { $az }° az { $el }° el
sat-rotator-2 = Rotor: { $error }
sat-up-now-2 = ● visível agora, { $el }°
sat-inv = { $mode } · inv
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = retendo { $here } MHz
scanner-scanning-mhz = varrendo · { $here } MHz

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = Este renderizador não suporta uma textura mais larga que { $max_texture_dim } pixels.

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = Voz: { $voice }
speech-speech-unavailable = A fala está indisponível: { $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = { $stations } transmissões

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = última sessão falhou — { $err }
winlink-last-session-received-sent = última sessão: { $last_received } recebidas, { $last_sent } enviadas
winlink-empty = { $to_lowercase } está vazia
winlink-showing = mostrando { $entries } de { $total }
winlink-fetching = buscando { $mid }…
winlink-from-2 = de { $msg } { $date }
winlink-attachment-bytes = anexo: { $att } ({ $data } bytes)

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    objeto de { $reported_by }
aprsmap-heard-ago =
    {""}
    ouvido há { $age }

## download: messages with values (src/download.rs)

download-reading = lendo { $path }: { $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}imagem ausente: { $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } MHz
hfdlmap-fix =
    {""}
    { $fixes } posição{ $es }
hfdlmap-heard-ago =
    {""}
    ouvido há { $last_at }

## multi: messages with values (src/multi.rs)

multi-radio = Rádio { $n }
multi-could-not-add-radio = Não foi possível adicionar um rádio: { $error }
multi-was-closed-station = { $value } foi fechado na estação.
multi-could-not-open = Não foi possível abrir { $peer }: { $error }
multi-has-tab-own-now = { $value } agora tem uma aba própria.

## remote: messages with values (src/remote.rs)

remote-protocol-error = erro de protocolo: { $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = mensagem inválida: { $error }
solarnet-solar-protocol-mismatch-server-client = incompatibilidade de protocolo solar: servidor { $proto }, cliente { $value }

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
smeter-received-signal-strength-s9-db = Intensidade do sinal recebido: S9 + { $over } dB ({ $s_dbm } dBm na banda de passagem).
smeter-received-signal-strength-s-dbm = Intensidade do sinal recebido: S{ $value } ({ $s_dbm } dBm na banda de passagem).
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

decodes-you-transmit-s-everyone-else = Você transmite { $off } s { $after } os demais — o motivo habitual de chamadas sem resposta. Sincronize o relógio do seu computador.

## publicsdr: nested messages

publicsdr-noise-floor-score = pontuação do piso de ruído { $snr }
publicsdr-up-khz-i-q = até { $khz } kHz de I/Q

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = GR gate { $gate_gr_db } de-esser { $deess_gr_db } comp { $comp_gr_db } lim { $limiter_gr_db }

## amp: more messages

amp-h-step = ~{ $l_step } µH · passo { $l_step2 }
amp-pf-step = ~{ $c_step } pF · passo { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel }-bit · { $media_type } / { $content_encoding } · { $encoded_size } octetos em { $chunk_count } blocos ({ $chunks_first_pass } na primeira passagem) · sessão { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value } — { $symbol_rate } baud CPFSK, ±{ $deviation_hz } Hz, { $bandwidth_hz } kHz de largura ocupada

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · { $local } local
sat-in-seconds = em { $s } s
winlink-to-list = para { $to }
winlink-cc-list = cc { $cc }
spots-n-of-total = { $n } de { $total }
wefax-n-of-total = { $n } de { $total }
ais-range-bearing = { $km } km a { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = Desligado (somente sintonizador, 24 MHz acima)
rtl-sdr-hf-mode-auto = Automático
rtl-sdr-hf-mode-direct-q = Amostragem direta (ramo Q)
rtl-sdr-agc-manual = Manual (sem AGC)
rtl-sdr-agc-tuner = AGC do sintonizador
rtl-sdr-agc-rtl = AGC digital do RTL
rtl-sdr-agc-both = AGC do sintonizador + RTL
mode-control-cat = CAT
mode-control-radio = Controlado pelo rádio
line-state-none = Nenhum
line-state-high = Alto
line-state-low = Baixo
elad-tx-input-usb-audio = Áudio USB
elad-tx-input-mic = Microfone
elad-tx-input-auto = Automático
elad-tx-input-leave = Deixar como ajustado no rádio
tr-usdx-audio-one-cable = Um cabo (áudio no fluxo CAT)
tr-usdx-audio-sound-card = Placa de som USB (conector de 3,5 mm)
spy-server-format-uint8 = 8 bits (menor largura de banda)
spy-server-format-int16 = 16 bits
spy-server-format-float32 = Ponto flutuante de 32 bits
sound-format-iq = IQ (estéreo)
sound-format-demod-audio = Áudio demodulado
sdr-play-duo-tuner-tuner1 = Sintonizador 1 (50 Ohm / Hi-Z)
sdr-play-duo-tuner-tuner2 = Sintonizador 2 (50 Ohm)
sdr-play-duo-role-diversity = Uma segunda antena (diversidade / supressão de QRM)
sdr-play-duo-role-second-radio = Um segundo rádio, na própria frequência
sdr-play-agc-off = Desligado (ganho de F.I. manual)
sdr-play-agc-hz5 = 5 Hz
sdr-play-agc-hz50 = 50 Hz
sdr-play-agc-hz100 = 100 Hz
pluto-ptt-off = Desligado
pluto-ptt-gpo01 = GPO0 = RX, GPO1 = TX
pluto-ptt-gpo23 = GPO2 = RX, GPO3 = TX
pluto-duplex-fdd = FDD (os dois ao mesmo tempo)
pluto-duplex-tdd = TDD (um de cada vez)
pluto-agc-manual = Manual
pluto-agc-slow-attack = Ataque lento
pluto-agc-fast-attack = Ataque rápido
pluto-agc-hybrid = Híbrido
parity-none = Nenhuma
parity-even = Par
parity-odd = Ímpar
panadapter-tap-antenna = Antena (compartilhada com o rádio)
panadapter-tap-if-output = A saída de F.I. do rádio
panadapter-audio-attached = O receptor conectado
panadapter-audio-transceiver = O transceptor
kenwood-send-ts2000 = Estilo TS-2000 (TX;)
kenwood-send-ts590 = Estilo TS-590 (TX1;)
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = Controlado pelo rádio
cw-keying-cat = Manipulador do rádio (CAT)
cw-keying-audio = Placa de som (MCW)
alert-sound-ding = Ding
alert-sound-two-tone = Dois tons
alert-sound-triplet = Tríplice
alert-sound-warble = Trinado
alert-sound-digital = Digital
alert-reply-tone = Tom
alert-reply-voice = Voz
alert-reply-both = Tom + voz
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
icom-model-other = Outro (definir o endereço manualmente)
backend-auto = Detecção automática (SoapySDR / CAT)
backend-soapy = SoapySDR
backend-cat = CAT / Áudio
backend-hpsdr = HPSDR (rede)
backend-tci = TCI (rede)
backend-icom-net = Icom LAN (rede)
backend-smart-sdr = SmartSDR / FlexRadio (rede)
backend-pluto = PlutoSDR (rede)
backend-rtl-sdr = RTL-SDR (USB)
backend-rtl-tcp = RTL-SDR por rtl_tcp (rede)
backend-spy-server = SpyServer (rede)
backend-spy-server-vfo = SpyServer VFO+FFT, baixa largura de banda (rede)
backend-kiwi-sdr = KiwiSDR / Web-888 (rede)
backend-rx888 = RX-888 (USB)
backend-airspy-hf = Airspy HF+ (USB)
backend-airspy = Airspy R2 / Mini (USB)
backend-hack-rf = HackRF One / Pro (USB)
backend-sdr-play = SDRplay RSP (USB)
backend-elad = ELAD FDM-DUO / FDM-S (USB)
backend-lime = LimeSDR + LimeRFE (LimeSuite)
backend-hydra-sdr = HydraSDR RFOne (USB)
backend-fobos = RigExpert Fobos SDR (USB)
backend-usb-audio = Rádio USB com áudio (placa de som)
backend-none = Não configurado
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld (rede)
cat-family-flrig = flrig (rede)
publicsdr-its-interface = a sua interface

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = Desligado
agc-mode-slow = Lento
agc-mode-med = Médio
agc-mode-fast = Rápido
airspy-gain-linearity = Linearidade (melhor para sinais fortes)
airspy-gain-sensitivity = Sensibilidade (melhor para sinais fracos)
ais-kind-class-a = Classe A
ais-kind-class-b = Classe B
ais-kind-base-station = estação-base
ais-kind-sar-aircraft = aeronave SAR
ais-kind-aid-to-navigation = auxílio à navegação
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = embarcação associada
alert-event-called = Uma estação está me chamando
alert-event-cq = Um CQ a que eu poderia responder
alert-event-new-dxcc = Nova entidade DXCC
alert-event-new-dxcc-band = Nova entidade nesta faixa
alert-event-new-grid = Novo quadrado locator
audio-routing-both = Esquerdo + direito
audio-routing-left = Somente esquerdo
audio-routing-right = Somente direito
button-mode-momentary = Segurar
button-mode-toggle = Alternar
contest-mode-none = Nenhum
contest-mode-eu-vhf = EU VHF Contest
converter-tx-off = Desligado enquanto converte
converter-tx-transverter = Pelo mesmo conversor
converter-tx-own = Deslocamento próprio
cw-engine-neural = NEURAL
cw-engine-timing = TEMPO
decode-sort-none = Nenhum
decode-sort-signal = SNR
decode-sort-distance = Dist
decode-sort-country = País
diversity-mode-cancel = Cancelar — anula uma fonte de ruído
diversity-mode-combine = Combinar — recepção por diversidade
dxped-mode-normal = Normal
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = Recusar transmitir
fail-safe-warn-only = Transmitir mesmo assim, avisar
ftp-security-plain = FTP (sem criptografia)
ftp-security-explicit-tls = FTPS explícito (AUTH TLS)
ftp-security-implicit-tls = FTPS implícito
hpsdr-filter-board-none = Nenhuma — as saídas ficam desligadas
hpsdr-filter-board-n2adr = Placa de filtro N2ADR
hpsdr-filter-board-alex = Código de banda Alex / Hermes (Zeus, HiQSDR, Quisk)
hpsdr-filter-board-custom = Personalizada — minha própria tabela abaixo
hpsdr-io-rx-input-radio = Entrada própria do rádio
hpsdr-io-rx-input-io-board = Placa IO J9
hpsdr-io-rx-input-io-board-pure-signal = Placa IO J9, PureSignal na transmissão
hydra-sdr-gain-linearity = Linearidade (melhor para sinais fortes)
hydra-sdr-gain-sensitivity = Sensibilidade (melhor para sinais fracos)
icom-rx-source-af = AF — o rádio demodula
icom-rx-source-if12k = F.I. de 12 kHz — o sdroxide demodula
icom-scope-span-radio = Como ajustado no rádio
icom-scope-span-khz2-5 = 5 kHz (±2,5k)
icom-scope-span-khz5 = 10 kHz (±5k)
icom-scope-span-khz10 = 20 kHz (±10k)
icom-scope-span-khz25 = 50 kHz (±25k)
icom-scope-span-khz50 = 100 kHz (±50k)
icom-scope-span-khz100 = 200 kHz (±100k)
icom-scope-span-khz250 = 500 kHz (±250k)
icom-scope-span-khz500 = 1 MHz (±500k)
ism-family-weather = CLIMA
ism-family-meter = MEDIDORES
ism-family-home-auto = CASA
ism-family-lo-ra = LORA
ism-family-unidentified = DESCONHECIDO
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = não identificado
js8-speed-normal = NORMAL
js8-speed-fast = RÁPIDO
js8-speed-turbo = TURBO
js8-speed-slow = LENTO
lime-aux-role-off = Não usada
lime-aux-role-diversity = Uma segunda antena (diversidade / supressão de QRM)
lime-aux-role-pure-signal = Realimentação de transmissão (pré-distorção PureSignal)
lookup-provider-none = Desligado
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = CAIXA DE ENTRADA
mail-folder-outbox = CAIXA DE SAÍDA
mail-folder-sent = ENVIADAS
mail-folder-archive = ARQUIVO
memory-sort-name = Nome
memory-sort-freq = Freq
memory-sort-band = Faixa
memory-sort-stored = Guardada
mouse-button-middle = Meio
mouse-button-extra1 = Voltar (extra 1)
mouse-button-extra2 = Avançar (extra 2)
nr-level-off = Desligado
nr-level-low = Baixo
nr-level-medium = Médio
nr-level-high = Alto
nr-level-rnn-low = RNN Baixo
nr-level-rnn-med = RNN Médio
nr-level-rnn-high = RNN Alto
nr-level-spec-low = SPEC Baixo
nr-level-spec-med = SPEC Médio
nr-level-spec-high = SPEC Alto
nr-level-nr2-low = NR2 Baixo
nr-level-nr2-med = NR2 Médio
nr-level-nr2-high = NR2 Alto
nr-level-df-low = DFNR Baixo
nr-level-df-med = DFNR Médio
nr-level-df-high = DFNR Alto
nr-strength-low = Baixa
nr-strength-med = Média
nr-strength-high = Alta
orbit-rings-none = nenhum
orbit-rings-curated = curados
orbit-rings-all = todos
qso-step-idle = Ocioso
qso-step-wait-cq = Aguardar CQ
qso-step-calling-cq = Chamando CQ
qso-step-tx-grid = TX Locator
qso-step-tx-report = TX Relatório
qso-step-tx-r-report = TX R+Relatório
qso-step-tx-rr73 = TX RR73
qso-step-tx73 = TX 73
qso-step-confirming = Confirmando
rds-standard-auto = Automático
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = Região 1 — Europa, África, Oriente Médio, Ásia do Norte
region-r2 = Região 2 — Américas
region-r3 = Região 3 — Sul e Leste da Ásia, Austrália e Oceania, Pacífico
relative-mode-absolute = Absoluto (fader)
relative-mode-sign-magnitude = Relativo: sinal/magnitude
relative-mode-twos-complement = Relativo: complemento de dois
relative-mode-binary-offset = Relativo: centrado em 64
relay-link-off = Nenhum
relay-link-serial = Placa de relé USB (serial)
relay-link-serial-lines = Linha serial RTS/DTR
relay-link-hid = Placa de relé USB (HID)
relay-link-cm108 = GPIO de placa de som CM108/CM119
relay-link-gpio = Linha GPIO do Linux
relay-link-command = Comando externo
relay-role-unused = Não usado
relay-role-sdr-antenna = Aterrar a antena do SDR
relay-role-amplifier = Acionar um amplificador / relé T-R
relay-role-aux = Auxiliar
relay-role-band-decoder = Decodificador de faixa (filtro / transverter)
rfe-channel-wb1000 = Banda larga 1 – 1000 MHz
rfe-channel-wb4000 = Banda larga 1 – 4 GHz
rfe-channel-ham0030 = HF, até 30 MHz
rfe-channel-ham0070 = 6 m / 4 m (50 – 70 MHz)
rfe-channel-ham0145 = 2 m (140 – 150 MHz)
rfe-channel-ham0220 = 1,25 m (220 – 225 MHz)
rfe-channel-ham0435 = 70 cm (400 – 450 MHz)
rfe-channel-ham0920 = 33 cm (902 – 928 MHz)
rfe-channel-ham1280 = 23 cm (1220 – 1420 MHz)
rfe-channel-ham2400 = 13 cm (2,3 – 2,5 GHz)
rfe-channel-ham3500 = 9 cm (3,3 – 3,7 GHz)
rfe-channel-cell-band01 = Banda celular 1
rfe-channel-cell-band02 = Banda celular 2
rfe-channel-cell-band03 = Banda celular 3
rfe-channel-cell-band07 = Banda celular 7
rfe-channel-cell-band38 = Banda celular 38
rfe-channel-not-selected = Nenhum selecionado
rfe-link-off = Não conectado
rfe-link-serial = Cabo USB próprio (serial)
rfe-link-board = Pelo LimeSDR (GPIO / I²C)
rfe-mode-control-auto = Automático (segue a fiação)
rfe-mode-control-rx = Sempre recebendo
rfe-mode-control-tx = Sempre transmitindo
rfe-mode-control-tx-rx = Sempre os dois (somente conectores separados)
rifp-encoding-auto = Automático
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = Bruto
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = MEM
scan-kind-range = FAIXA
scan-resume-carrier = PORTADORA
scan-resume-timed = TEMPORIZADO
scan-resume-manual = MANUAL
sense-line-off = Não ligada
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD (detecção de portadora)
shift-simplex = SIMPLEX
shift-minus = −
shift-plus = +
speed-slow = Lento
speed-medium = Médio
speed-fast = Rápido
speed-faster = Mais rápido
speed-fastest = Mais rápido ainda
tls-verify-system = Repositório de confiança do sistema
tls-verify-pinned = Impressão digital fixada
tls-verify-off = Não verificar
tone-mode-off = DESLIGADO
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN (rotativo)
upload-naming-original = Nome original do arquivo
wheel-action-zoom = Zoom
wheel-action-tune = Sintonizar
wheel-action-none = Nada

settings-alerts-say = FALAR

settings-alerts-say-hint = Ouvir este alerta falado

settings-alerts-say-need-voice = Escolha Voz ou Tom + voz para ouvir

settings-alerts-say-voice-off = Ative a voz em Configurações → UI → Anúncios por voz

settings-alerts-speaks = fala: { $text }

settings-picup-keep = manter

jtty-starting-receiver = iniciando o receptor JTTY…

jtty-receive-audio-level = Nível de áudio de recepção

jtty-sending = {" "}ENVIANDO{" "}

jtty-sent-once-note = A mensagem é enviada uma vez ao pressionar TX — o JTTY é assíncrono, então não há período a aguardar e nada se repete.

signalid-title = ID DE SINAL

signalid-tuned-now = Sintonizado agora

signalid-passband = banda passante { $bw }

signalid-band = banda de { $band }

signalid-find = Buscar

signalid-find-hint = FT8, meteoro, meteorologia, aeronaves…

signalid-candidates = { $n } candidatos para este dial

signalid-matches = { $n } resultados para “{ $query }”

signalid-nothing-fits = Nada no catálogo corresponde a este dial, modo e banda passante.

signalid-no-match = Nenhuma entrada do catálogo corresponde.

signalid-open-url = Abrir { $url }

windows-chip-del = DEL

windows-chip-edt = EDT

windows-chip-ren = REN

windows-chip-rec = REC

windows-name-hint = nome

windows-waterfall-marks = Cascata

windows-stacked = empilhado

jtty-listening-note = Escutando. As transmissões JTTY começam a qualquer momento — uma mensagem aparece aqui logo após ser ouvida.

extras-title = EXTRAS

extras-receivers = Receptores

extras-dab-hint = Rádio digital DAB / DAB+ — o player abre abaixo da cascata

extras-aircraft-watch-hint = AERONAVES — uma aeronave da sua lista de acompanhamento está no ar; abra a visão Acompanhar

extras-aircraft-hint = AERONAVES — o mapa do que o rádio ouve: aeronaves e radiossondas, em uma janela que pode ser destacada

extras-radio-hint = RADIO CLASSIC — um painel de rádio clássico para escuta de radiodifusão: bandas, memórias, o nome da estação e RadioText. Ele acopla sob a cascata nos modos de voz; Configurações → Destacável o coloca em uma janela

extras-antenna = ANTENA

extras-antenna-hint = ANTENA (Experimental/Beta) — comprimentos de corte, impedância no ponto de alimentação e diagrama para antenas comuns de fio/vertical/Yagi

recjobs-title = GRAVAÇÕES

recjobs-finished = Concluída “{ $name }”

recjobs-skipped = Ignorada “{ $name }” — já está gravando

recjobs-recording-name = Gravando “{ $name }”

recjobs-new-job = + NOVA TAREFA

recjobs-recording = ● gravando

recjobs-none = Nenhuma gravação agendada — pressione + NOVA TAREFA para adicionar.

recjobs-done = concluída

recjobs-edit = editar

recjobs-del = excl

recjobs-station = Estação

recjobs-station-hint = BBC World Service

recjobs-kind = Tipo

recjobs-start-utc = Início (UTC)

recjobs-for = Duração

recjobs-save = SALVAR

recjobs-kind-audio = Áudio

recjobs-kind-iq = I/Q

recjobs-kind-both = Ambos

panels-sensitivity = Sensibilidade

navtex-hunting = procurando

navtex-hunting-hint =
    Nada está sendo lido ainda.

    Quase sempre é a sintonia. NÃO digite o número anunciado (518) na caixa de frequência: o NAVTEX é citado como canal, e este programa escuta logo abaixo dele. Clique no botão 518 acima — ele ajusta o dial correto para você (516,300).

    Se há claramente um sinal forte mas o texto continua sem sentido, pressione REV: ele está chegando invertido.

    Lembre-se também de que o NAVTEX é uma grade horária, não uma conversa: uma estação costeira transmite por dez minutos a cada quatro horas, então o silêncio é normal na maior parte do tempo. É preciso estar escutando quando sua estação estiver programada. E os 518 kHz só ganham vida à noite — uma banda quieta de dia não é um decodificador quebrado.

navtex-sensitivity-hint =
    Sensibilidade — com que facilidade o decodificador sincroniza com o sinal e o mantém.
navtex-auto-hint = Volta aos limiares de referência (0). Destacado enquanto ativo.

navtex-tune-dial-usb-below-channel = Sintonize o dial aqui (USB, 1,710 kHz abaixo do canal).

settings-audio-your-audio = Seu áudio

settings-audio-your-audio-why = Os alto-falantes em que você escuta e o microfone por onde sua voz sai. Pertencem a esta máquina e não a um rádio, então são os mesmos qualquer que seja o rádio em uso.

settings-audio-browser-note = Os dispositivos de som são escolhidos aqui no navegador, não nesta janela — a página pede ao navegador os alto-falantes e o microfone. As configurações de placa de som da própria estação, mais abaixo nesta página, continuam valendo.

settings-audio-speakers = Alto-falantes

settings-audio-channels-hint = Em quais canais de alto-falante o áudio recebido chega. Esquerdo ou direito silencia o outro, para ouvir de um lado só.

settings-audio-microphone-note = O microfone é o que leva sua voz quando o rádio é acionado por som (MCW) ou por VOX. Se o seu rádio é acionado por CAT, o microfone não é usado na transmissão e esta linha não altera nada.

settings-audio-stream-out = Enviar o áudio para fora

settings-audio-stream-out-why = Envia o que este rádio recebe para outro programa pela rede — um decodificador, um gravador, outra máquina — sem passar por uma placa de som.

settings-audio-send-to-hint =
    `host:porta`. Mono, 16 bits, na taxa de recepção, um datagrama por bloco de áudio, e é o áudio do **ponto de gravação**: o volume de AF, o MUTE e a transmissão não o afetam, porque um decodificador externo quer o rádio e não o nível dos seus fones.

    Um endereço numérico, não um nome de host. Vazio é desligado.

settings-audio-send-to-note = `127.0.0.1:7355` envia para outro programa neste computador; o endereço próprio desta máquina envia pela rede.

settings-audio-radio-audio = Áudio deste rádio

settings-audio-radio-audio-why = A placa de som entre este rádio e este computador. Ela pertence ao rádio e não à máquina: dois rádios aqui podem dividir o computador e ainda precisar de placas diferentes, por isso estas configurações ficam com o rádio a partir do qual você abriu as Configurações.

settings-audio-no-cards-yet = Esta máquina ainda não informou suas placas de som. Os nomes ainda podem ser digitados abaixo; uma lista aparece quando os dispositivos forem consultados.

settings-audio-rescan = Reescanear placas de som

settings-audio-rescan-hint =
    Pergunta de novo à máquina quais placas de som ela tem. Nada é aberto, então é seguro pressionar durante a recepção.

    Use depois de conectar um codec. Isso não reabre o rádio — isso é Aplicar / reconectar na aba Rádio — e não consegue reparar uma placa que o servidor de som renomeou, o que é informado no rádio.

settings-audio-from-radio = Do rádio

settings-audio-to-radio = Para o rádio

settings-audio-radio-note = { "*" }*Do rádio** é o que seu rádio envia — uma saída de som USB, ou a saída de fones de um receptor na entrada de linha de uma placa de som. **Para o rádio** é o que o rádio escuta da sua voz, e é também como o programa aciona um rádio por som: deixe vazio e o microfone do próprio rádio será usado, se houver.

settings-audio-recording = Gravação

settings-audio-recording-why = Onde são gravados os áudios, as gravações agendadas e as capturas I/Q brutas.

enigma-need-more = dê mais texto cifrado para trabalhar

enigma-title = ENIGMA

enigma-no-cables = sem cabos

enigma-thin = pouco

enigma-rotor = Rotor

enigma-window-hint = Grundstellung — clique para avançar a roda

enigma-input-hint = digite, ou use as teclas acima

enigma-clear = LIMPAR

enigma-copy = COPIAR ⇄

enigma-solve-note = LÖSEN — cole um texto cifrado. Informe uma palavra provável (crib) que você espera na mensagem para um texto com cabos; sem ela, o resolvedor trata uma máquina sem cabos.

enigma-crib = Crib

enigma-solve = RESOLVER

enigma-flaw = Uma letra nunca é cifrada nela mesma — o refletor garante isso. Essa é a falha da máquina real, e é nela que o resolvedor poda.

settings-audio-takes-over-speakers = { "*" }*Estes alto-falantes são um dispositivo do servidor de som, não uma placa de som.** Usá-los também os torna o dispositivo de reprodução de *tudo o mais* neste computador — outros programas também tocarão por eles. O PipeWire não permite que um programa escolha seu próprio destino de reprodução aqui, então isso é inevitável. Para mudar: escolha uma placa de som, ou volte o padrão da máquina (`wpctl set-default <nome>`) e reescaneie.

settings-audio-takes-over-microphone = { "*" }*Este microfone é um dispositivo do servidor de som, não uma placa de som.** Usá-lo também o torna o dispositivo de gravação de *tudo o mais* neste computador — outros programas também gravarão dele. O PipeWire não permite que um programa escolha sua própria fonte de captura, então isso é inevitável. Para mudar: escolha uma placa de som, ou volte o padrão da máquina (`wpctl set-default <nome>`) e reescaneie.

settings-audio-currently-chosen = ⚠  Escolhido atualmente: { $device }

knowncalls-no-table = Este modo não mantém tabela de indicativos. É um decodificador FT8/FT4 que usa hash de indicativos — qualquer outro não tem nada para mostrar aqui.

knowncalls-not-asked = Ainda não consultado — pressione ATUALIZAR.

knowncalls-nobody = Ninguém ainda. Uma estação é adicionada no momento em que é ouvida soletrando seu indicativo — um CQ, um relatório com os dois indicativos em claro, ou uma chamada simples em 11 m.

knowncalls-title = ESTAÇÕES CONHECIDAS

knowncalls-intro = Todo indicativo que um <...> com hash neste receptor pode nomear no momento. Um hash não pode ser lido de trás para frente — uma estação só aparece aqui depois de ser ouvida soletrando seu indicativo, que é para o que serve uma chamada simples em 11 m.

knowncalls-find-hint = indicativo, país, prefixo…

knowncalls-refresh = ATUALIZAR

knowncalls-capped = mais recentes { $shown } de { $total } — ATUALIZAR para consultar de novo, a lista é um instantâneo

knowncalls-known = { $shown } conhecidas

knowncalls-matches =
    { $n } { $n ->
        [one] resultado
       *[other] resultados
    }

knowncalls-no-match = Nenhuma estação conhecida corresponde.

knowncalls-no-country = sem país — não é um indicativo que esta tabela conhece

classic-controls = CONTROLES

classic-fold-the-column-away-to-a-rail-on = Recolhe a coluna em uma barra à esquerda. O visor e o medidor ficam na barra, e o controle que a traz de volta está na barra lateral.

classic-click-to-open-or-close-this = Clique para abrir ou fechar esta seção. Arraste-a sobre outra seção para movê-la — a ordem é lembrada.

classic-bring-the-control-column-back = Traz de volta a coluna de controles.

classic-no-radio = sem rádio

classic-rate = Taxa

classic-dial = Dial

classic-receive = Recepção

classic-listening-screen = tela de escuta

classic-radio-settings = CONFIGURAÇÕES DO RÁDIO

classic-the-backend-the-sample-rate-the = O backend, a taxa de amostragem, a antena, o conversor e a calibração própria deste rádio — Configurações → Rádio, onde sempre estiveram. A coluna mostra como estão ajustados, em vez de uma segunda cópia dos controles.

classic-counted-in-steps-not-decibels-what = Contado em passos, não em decibéis: quanto vale um passo é assunto do próprio receptor, e em um SDRplay depende também da banda. Para a direita continua sendo mais ganho.

classic-the-plutosdr-s-own-agc-is-setting = O próprio AGC do PlutoSDR está definindo este ganho, então mover o controle não faz nada. Defina Configurações → Rádio → AGC como Manual para controlá-lo aqui.

classic-squelch = Squelch

classic-gate-the-audio-below-this-power-in = Bloqueia o áudio abaixo desta potência na banda passante de recepção, em dBFS. Esquerda é aberto. Ajuste acima do ruído e abaixo do sinal.

classic-this-radio-has-not-reported-what = Este rádio ainda não informou o que sabe fazer — seu enlace está fechado, ou ele não respondeu.

classic-open-the-link-in-the-radio-strip = Abra o enlace na barra do rádio, e os controles de transmissão aparecem aqui.

classic-this-device-receives-only-it-has = Este dispositivo só recebe: não tem transmissor para este programa acionar.

classic-transmit-is-offered-by-a-rig-with = A transmissão é oferecida por um rádio com via de controle ou de áudio — Configurações → Rádio.

classic-transmit-is-off-for-this-radio-s = A transmissão está desativada para a tela deste rádio.

classic-settings-radio-show-transmit = Configurações → Rádio → "Mostrar controles de transmissão" a traz de volta.

classic-key = Manip

classic-controlled-envelope-ssb-how-much = SSB de envoltória controlada: quanto do processamento de voz aplicar. Para cima é mais forte e mais achatado, ao custo de algum splatter nos dois lados.

classic-holding-the-squelch-open-click-to = Mantendo o squelch aberto. Clique para restaurar o limiar que você tinha.

classic-click-to-hold-the-squelch-open-to = Clique para manter o squelch aberto — para ouvir se há algo ali — e clique de novo para restaurá-lo exatamente como estava.

panels-received = Recebido

panels-text = Texto

panels-hex = Hex

panels-nothing-received-yet = Nada recebido ainda.

panels-payload = Carga útil

panels-blocks = Blocos

panels-sequence = Sequência

uvpacket-starting-receiver = iniciando o receptor UVPacket…

uvpacket-frames-count = { $n } quadros

uvpacket-frames-count-hint = Quadros decodificados desde que o receptor iniciou.

uvpacket-intro = Um modo de pacote para grupos privados de VHF/UHF. Sintonize o dial no canal do grupo; o submodo é detectado em cada quadro.

uvpacket-nothing-yet = Nada ainda. O UVPacket fica quieto entre transmissões — deixe-o em um canal e aguarde uma rajada.

uvpacket-app-seq = app { $app } · seq { $seq }

uvpacket-click-read-frame = Clique para ler o quadro inteiro.

uvpacket-bps-net = { $bps } bps líquidos

uvpacket-sub-mode = Submodo

uvpacket-app-type = Tipo de app

uvpacket-payload-bytes = { $n } bytes

dsc-starting-receiver = iniciando o receptor DSC…

dsc-seq-tone = { $n } seq · tom { $pct }%

dsc-seq-tone-hint = Sequências que chegaram ao marcador final, e quão nitidamente os dois tons se separam.

dsc-nothing-yet = Nada ainda. O DSC fica quieto entre embarcações — deixe-o em um canal e aguarde uma rajada.

panels-marginal = marginal

dsc-marginal-hint = Um caractere falhou na verificação BCH — os campos mostrados podem estar errados.

dsc-click-read-sequence = Clique para ler a sequência inteira.

panels-marginal-caps = MARGINAL

dsc-marginal-caps-hint = Pelo menos um caractere falhou na verificação BCH. A sequência é mostrada porque um alerta de socorro que chegou imperfeito ainda vale ser lido.

dsc-category = Categoria

dsc-nature = Natureza

dsc-caller-mmsi = MMSI do chamador

dsc-target-mmsi = MMSI de destino

dsc-alert-time = Hora do alerta

dsc-working-ch = Canal de trabalho

dsc-raw-symbols = Símbolos brutos

dsc-none = (nenhum)

settings-general-device-type-name = …ou digite o nome — vazio é o padrão do sistema

settings-general-signed-in-station = Estação conectada

settings-general-sign-out-hint = Esquece o login deste dispositivo. A estação terá de pedir de novo na próxima conexão; esta continua ativa até você fechá-la.

settings-general-recordings-remote = As gravações são gravadas na máquina à qual o rádio está conectado. Defina a pasta lá.

settings-general-folder = Pasta

settings-general-choose = ESCOLHER…

settings-general-choose-hint = Escolha a pasta onde as gravações são salvas

settings-general-default = PADRÃO

settings-general-default-hint = Voltar ao padrão — sua pasta de músicas, ou o diretório de configuração quando o sistema não tem uma

settings-general-recordings-dir-note = Onde são gravados os áudios, as gravações agendadas e as capturas I/Q brutas. Padrão: a subpasta sdroxide da sua pasta de músicas, ou a pasta de gravações do diretório de configuração quando o sistema não expõe uma pasta de músicas.

setup-suffix-cq = { " " }cq

setup-suffix-tries = { " " }tentativas

setup-window = janela

setup-south = sul

setup-west = oeste

setup-message-templates = Modelos de mensagem  { $tokens }

setup-ft8-decoder-effort-hint =
    Quanto o decodificador FT8 se esforça por sinais fracos.

    Rápido — uma passada, sem subtração de sinal: o mais veloz, menos decodificações.
    Normal — múltiplas passadas planas com subtração: ~0,4 s em um slot cheio, algumas decodificações a menos que o Profundo.
    Profundo — a passada com checkpoints do WSJT-X: ~1,2 s e o máximo de decodificações (o padrão).

    Apenas o segundo lote de decodificações é controlado; a passada simples é sempre emitida primeiro, então uma resposta nunca é atrasada por ele.

setup-auto-cq-repeats-hint = Quantas chamadas CQ uma rodada faz antes de terminar. 0 é sem limite próprio: o watchdog de transmissão e a parada por inatividade ainda limitam a rodada.

setup-auto-cq-then-listen-hint = Depois dessas chamadas CQ, muda de AUTO CQ para AUTO RX e continua escutando em vez de parar de vez. Desligado, a rodada para.

setup-auto-call-retries-hint = Quantas vezes uma estação é chamada antes de a rodada desistir dela e passar para a próxima da fila.

setup-auto-session-hint = Quanto tempo uma rodada sem supervisão pode durar, seja qual for o trabalho. 0 é sem limite próprio.

setup-serial-contests-hint = Concursos com número de série — ARRL RTTY Roundup e CQ WPX em FT8: um relatório de sinal e um número de série (ou um estado dos EUA), sem localizador. O CQ sai como CQ RU.

setup-tx-level-fm-hint =
    Com que intensidade o burst deste modo é entregue a um rádio que o modula por conta própria — e em FM isso é o desvio, porque um transmissor FM converte nível de áudio em excursão de frequência e não tem ALC para contê-la. O packet de 1200 baud quer cerca de 3 kHz onde a voz quer 5, então fundo de escala em uma entrada de dados ajustada para voz causa desvio excessivo — o que soa completamente normal para quem ouve e não decodifica para ninguém. Diminua até que outras estações o reportem, ou ajuste o nível no rádio.

    Este nível pertence apenas a este modo. Um modo que você nunca ajustou parte do valor do FM, e os modos de banda lateral partem do seu próprio — assim um desvio ajustado para 1200 baud nunca chega ao FT8.

setup-tx-level-ssb-hint =
    Com que intensidade a transmissão deste modo é entregue a um rádio que a modula por conta própria — um rádio CAT pela placa de som, um FLEX, um Icom pela porta de rede. Em banda lateral isto é o drive no modulador: diminua até o ALC do rádio mal se mexer e ajuste a potência no rádio, porque ALC atuando em um modo de envoltória constante é o que causa splatter. O drive chega ao registrador de potência do rádio aqui, não ao áudio, então este é o nível.

    Este nível pertence apenas a este modo: FT8, RTTY, PSK e MCW mantêm cada um o seu, porque não carregam o modulador do mesmo jeito. O mesmo controle está na barra de transmissão, onde você o alcança enquanto transmite.

setup-repeats-hint-base = Chamadas sem resposta a uma estação antes de seguir adiante. Chamar CQ é isento.

setup-repeats-hint-duration = { " " }Com a duração de slot de { $mode } ({ $slot } s), isso dá cerca de { $mins } min.

setup-repeats-hint-disables = { " " }0 desativa.

setup-repeats-hint-11m = { " " }Em 11 m isto também é o que encerra uma rodada de CQ: o watchdog de transmissão não a corta, porque uma estação que responde a um CQ é esperada em cerca de 30 s e um relógio não distingue uma rodada recente de uma velha.

setup-symbol-home-station = Estação fixa

setup-symbol-car = Carro

setup-symbol-truck = Caminhão

setup-symbol-van = Van

setup-symbol-motorcycle = Moto

setup-symbol-bicycle = Bicicleta

setup-symbol-person-on-foot = Pessoa a pé

setup-symbol-motorhome = Motorhome

setup-symbol-boat = Barco

setup-symbol-yacht = Iate

setup-symbol-light-aircraft = Aeronave leve

setup-symbol-balloon = Balão

setup-symbol-digipeater = Digipeater

setup-symbol-i-gate = I-gate

setup-symbol-repeater = Repetidora

setup-symbol-weather-station = Estação meteorológica

setup-symbol-hospital = Hospital

setup-symbol-portable-campsite = Portátil / acampamento

settings-general-sign-out = SAIR

settings-general-fallback-password = Lembrando uma senha neste navegador para { $stations }, porque a estação não respondeu com um cookie de login. Estações mais antigas fazem isso; não é a mesma coisa.

settings-general-device-hint-speakers = O nome exato pelo qual seus alto-falantes são abertos. Escolha da lista quando estiver lá; digite quando a lista não o mostrar — um servidor de som pode oferecer um dispositivo que a API do host não enumera. Vazio usa o padrão do sistema.

settings-general-device-hint-microphone = O nome exato pelo qual seu microfone é aberto. Escolha da lista quando estiver lá; digite quando a lista não o mostrar — um servidor de som pode oferecer um dispositivo que a API do host não enumera. Vazio usa o padrão do sistema.

settings-general-device-hint-radio-rx = O nome exato pelo qual o áudio de recepção deste rádio é aberto. Escolha da lista quando estiver lá; digite quando a lista não o mostrar — um servidor de som pode oferecer um dispositivo que a API do host não enumera. Vazio usa o padrão do sistema.

settings-general-device-hint-radio-tx = O nome exato pelo qual o áudio de transmissão deste rádio é aberto. Escolha da lista quando estiver lá; digite quando a lista não o mostrar — um servidor de som pode oferecer um dispositivo que a API do host não enumera. Vazio usa o padrão do sistema.

settings-general-device-hint-other = O nome exato pelo qual o dispositivo é aberto. Escolha da lista quando estiver lá; digite quando a lista não o mostrar — um servidor de som pode oferecer um dispositivo que a API do host não enumera. Vazio usa o padrão do sistema.

settings-general-cb-world = Mundo / freeband — 80 canais

settings-general-cb-cept = CEPT / UE — 40 canais

settings-general-cb-germany = Alemanha — 80 canais

settings-general-cb-uk = Reino Unido — 27/81, 40 canais

settings-general-cb-us = Estados Unidos — 40 canais

settings-general-cb-australia = Austrália — 40 canais

settings-general-cb-pmr446 = PMR446 — 16 canais (446 MHz)

settings-general-cb-dpmr446 = dPMR446 — 32 canais (446 MHz)

wefax-receiving-progress = recebendo { $min } min · { $lines } linhas

wefax-listening = escutando

wefax-status-hint =
    Aguardando uma carta.

    O WEFAX é uma grade horária, não uma conversa: uma estação envia suas cartas em horários definidos, então uma frequência escolhida fica quieta na maior parte do dia — isso é normal. Use o botão ESTAÇÕES para escolher uma e conferir o horário.

    Se há um sinal mas a imagem sai de um só tom (toda cinza, toda preta ou toda branca), é a sintonia ou a velocidade — use o indicador +0 Hz e o controle IOC/velocidade, e confira se a entrada de ESTAÇÕES corresponde ao que está sendo transmitido.

wefax-centre = CENTRO

wefax-centre-hint = Ajuste o dial em { $sign }{ $off } Hz para que a subportadora volte a ficar em 1900 Hz — o receptor está { $off } Hz fora, o que transforma a carta em uma página branca.

wefax-auto-start-hint = Inicia uma carta quando o tom de início de 300 Hz (IOC 576) ou 675 Hz é ouvido, ou, se o tom se perdeu em um desvanecimento ou sob uma portadora, quando chegam alguns segundos do sinal de fasagem — e encerra a carta em andamento e faz a fasagem da nova quando a próxima transmissão chega, tenha ou não vindo um tom de parada antes. Uma portadora no canal não inicia nada.

wefax-fit = AJUSTAR

wefax-save-chart-hint = Salvar a carta da tela neste computador, até onde chegou

wefax-waiting-for-chart = Sintonize um horário de fax em USB e aguarde o início da carta (tom de início ou sinal de fasagem), ou pressione INICIAR para começar no meio da carta.

wefax-carrier-dial-note = portadora { $carrier } · dial { $dial } kHz (USB −1,9k)

wefax-carrier-note-hint = O chip da estação indica a portadora publicada; a subportadora do radiofax fica 1900 Hz acima dela, então o dial é deliberadamente 1,9 kHz mais baixo. Sintonize o dial na própria portadora e o sinal cai fora da banda passante.

contest-title = CONCURSO

contest-intro = Escolha um concurso e informe o que você envia. O log funciona em qualquer modo — digite a troca para CW ou SSB, e o lado FT8 a preenche sozinho onde o concurso tem um formato FT8.

contest-you-send = { "  " }você envia: { $exchange }

contest-they-send = { "  " }eles enviam: { $exchange }

contest-cb-note = Atividade PX / 11 m: um relatório e uma troca em texto livre — coloque nela o canal, nome ou área que a atividade usa.

contest-your-exchange = Sua troca

contest-start = INICIAR

contest-score-line = QSO { $qsos } · { $points } pts · { $mults } mult · PONTOS { $total }

contest-rate = { $r10 } em 10 min · { $r60 }/h

contest-stop = PARAR

contest-estimate = a pontuação é uma estimativa — o patrocinador decide

contest-call = INDICATIVO

contest-dupe = DUPLO

contest-sent = ENV

contest-rcvd = REC

contest-sending = enviando { $rst }

contest-sending-ours = enviando { $rst } { $ours }

contest-log = LOG

playback-radio-n = { " " }· rádio { $n }

playback-no-stamp = sem carimbo no nome — toca no modo em que você está

playback-title = REPRODUÇÃO — áudio gravado

playback-find-hint = modo, MHz, data, nome…

playback-refresh-hint = Ler a pasta de gravações de novo

playback-asking = Pedindo à estação suas gravações…

playback-folder = Pasta: { $dir }

playback-none = Nenhuma gravação ainda. Pressione REC → MP3 ou WAV enquanto algo estiver no ar; o arquivo aparece aqui, nomeado com hora, frequência e modo.

playback-on = ▶ LIGADO

playback-play = ▶ TOCAR

playback-iq-hint = Uma captura I/Q, não áudio. A reprodução usa o áudio que o gravador escreve; uma captura I/Q é tocada com --file.

playback-not-while-tx = Não durante a transmissão

playback-play-hint = Tocar esta gravação nos decodificadores desde o início

playback-busy-playing = Reproduzindo — pare antes de excluir

playback-busy-recording = Ainda sendo gravada — pare o REC antes de excluir

playback-delete-ask = EXCLUIR?

playback-delete-hint-armed = Pressione de novo para excluir este arquivo definitivamente. Não pode ser desfeito.

playback-delete-hint = Excluir esta gravação da pasta (pergunta mais uma vez antes)

playback-strip-label = ▶ REPRODUÇÃO

playback-not-live = fora do ar — o rádio está em espera

playback-carry-on = Continuar reproduzindo

playback-pause = ⏸ PAUSAR

playback-pause-hint = Pausar — os decodificadores ouvem silêncio até você continuar

playback-again = ↺ REPETIR

playback-again-hint = Volta ao início — outra tentativa em uma imagem ou decodificação

playback-loop = LOOP

playback-loop-hint = Recomeçar do início quando o arquivo terminar

playback-stop = ■ PARAR

playback-stop-hint = Encerrar a reprodução e devolver o rádio ao que estava

playback-files = ARQUIVOS

playback-end-of-file = fim do arquivo

playback-paused = pausado

playback-recorded-on-mode = Gravado em { $mhz } MHz em { $mode } — o dial e o modo foram ajustados a partir do nome do arquivo.

playback-recorded-on = Gravado em { $mhz } MHz — o dial e o modo foram ajustados a partir do nome do arquivo.

playback-no-dial-mode = O nome não registra dial nem modo, então toca no modo em que você está — escolha o decodificador no menu de modo.

playback-format-stereo = { $rate } Hz, estéreo — o canal esquerdo (recepção) é reproduzido.

playback-format-mono = { $rate } Hz, mono — o único canal é reproduzido.

playback-stopped = Parado: { $fault }

playback-intro = Reproduz uma gravação nos decodificadores — SSTV, CW, RTTY, PSK, FT8 e todos os outros modos que decodificam a partir de áudio. O rádio fica em espera enquanto toca, o dial e o modo vão para o que a gravação usou, e PARAR devolve o rádio. Mude o modo durante a reprodução para tentar outro decodificador.

packet-callsign-hint = indicativo

packet-via-hint = via

aprs-filter-hint = filtro

aprs-msg-sent = enviada

aprs-msg-ack = ack

aprs-msg-rejected = rejeitada

aprs-reply-chip = responder

aprs-to-hint = para

aprs-message-hint = mensagem

aprs-raw-chip = RAW

fsq-to-button = PARA

fsq-del-chip = DEL

macros-column-key = tecla

macros-column-button = botão

macros-column-sends = envia

macros-label-hint = rótulo

macros-add = ADD

macros-note-placeholders = { $mycall } e { $mygrid } são preenchidos quando a mensagem sai.

macros-buttons-note = Cada botão envia o texto inteiro de uma vez. F1–F9 acionam os nove primeiros, desde que nada na tela esteja com o teclado.



widgets-worked-before-any = Já trabalhado

vdl2-col-sig = SIG

vdl2-col-age = IDADE

vdl2-command = comando

vdl2-response = resposta

vdl2-frame-hint = quadro

vdl2-suffix-messages = { " " }mensagens

vdl2-suffix-stations = { " " }estações

ale-words-count = { $n } palavras

ale-waiting-audio = Aguardando áudio…

ale-no-words-yet = Nenhuma palavra ainda. Sintonize um canal ALE (ex.: 8992 ou 11175 kHz USB) e aguarde uma chamada.

atchat-role-master = MESTRE

atchat-role-backup = RESERVA

atchat-me = eu

dab-sub-channel = { " " }· sub { $n }

dab-starting-receiver = iniciando o receptor DAB…

dab-no-ensemble-yet = nenhum múltiplex nomeado ainda — há uma transmissão DAB neste canal?

dab-window-stats = { $mhz } MHz · { $frames } quadros · { $fibs } fibs

dab-volume-meter-hint = A estação como você a ouve, esquerda e direita — e o volume: arraste ou clique ao longo das barras, ou use a roda. Clique nele e use as setas para ajustar por passos. Funciona em uma tela bloqueada com o cadeado.

rade-searching = procurando

rfpaint-swl-note = O RF Paint é um modo somente de transmissão — nada a mostrar em SWL.

cw-hand-key-arm-hint = Arme o manipulador e use a palheta USB. Os contatos vão para o motor, que faz os pontos e traços na velocidade transmitida — aqui, não no painel, para que o tempo não seja arredondado a um quadro. A caixa de transmissão fica travada enquanto isto está ligado e qualquer texto na fila é descartado. Um manipulador simples (straight key) na mesma caixa também funciona: escolha o tipo Straight key.

cw-own-keyer-hint =
    Este rádio envia a partir do próprio manipulador: o texto vai pela porta de controle e o rádio faz a temporização dos elementos, então não há nada entre a mão e o ar para uma chave acionar — uma palheta tanto quanto um manipulador simples.

    Para manipular à mão, defina a manipulação CW como "Placa de som (MCW)" em Configurações → Rádio. O rádio fica então em uma banda lateral e o tom de monitoração do manipulador é transmitido como áudio, que é a via acionada pelo manipulador simples.

cw-no-paddle-found = nenhuma palheta encontrada — escolha uma em Configurações → CW

cw-msg-hint = Seus próprios botões de mensagem — uma troca de concurso, uma resposta com nome e QTH, TNX 73 GL. Cada um envia o texto inteiro de uma vez, e F1–F9 acionam os nove primeiros. Eles viajam com a configuração da estação, então um cliente remoto também os tem.

wspr-clear-rx = LIMPAR RX

wspr-rx-count = { $n } rx

wspr-sort-dist = DIST

wspr-sort-country = PAÍS

wspr-tx-enable = HABILITAR TX

wspr-stop-tx = PARAR TX

wspr-listening-as = Escutando como { $call } em { $sent } — definido na aba Geral das Configurações.

wspr-set-callsign-grid-label-spots = Defina seu indicativo e grid na aba Geral das Configurações para identificar seus spots.

schedule-source-mine = meus

schedule-all-of-them = todos

schedule-title = PROGRAMAS

schedule-at = Às

schedule-now-chip = AGORA

schedule-set-time = definir hora

schedule-find-hint = BBC, 6185, 15,4 MHz…

schedule-target = Alvo

schedule-any = qualquer

schedule-source = Fonte

schedule-all-hint = Todas as grades baixadas no mesmo repositório. Uma estação listada por duas delas aparece duas vezes, que é o detalhe para o qual esta janela serve — o panadaptador mostra um só nome.

schedule-favs = ★ FAVS

schedule-favs-hint = Somente as estações que você marcou com estrela

schedule-all-favs = ⭐ TODAS

schedule-all-favs-hint = Favoritar todas as estações da grade e mostrar seus rótulos na cascata enquanto estiver ligado. Clique de novo para limpar todos os favoritos.

schedule-solar-time = HORA LOCAL

schedule-solar-time-hint = Mostra a hora local de cada linha no transmissor. É o relógio do Sol — quatro minutos por grau de longitude — e não um fuso horário civil, portanto não considera horário de verão nem limites de fuso.

schedule-map = MOSTRAR MAPA

schedule-map-hint = Abrir LISTEN — esta grade como um mapa. Escolha uma estação nele, escreva o que ouviu e registre. A estação selecionada vem junto

schedule-on-air = { $count } no ar às { $time } UTC

schedule-now = agora

schedule-solar-hover = { $local } hora solar local em { $site }, { $lon }°

schedule-favourite-hint = Favoritar esta estação

rc-radio-classic = RADIO CLASSIC

rc-bands = Bandas

rc-mem-programs = Mem · Programas

rc-recall-band-mode-frequency-and = Recuperar: banda, modo, frequência e filtro de uma vez

rc-no-memories-yet-store-one-with-mem = nenhuma memória ainda — guarde uma com MEM na barra superior

rc-broadcast-schedule-what-is-on-when = Grade de radiodifusão — o que está no ar, quando e onde

rc-bw-this-mode-has-one-fixed = BW — este modo tem uma única banda passante fixa

rc-bw-click-next-filter-right-click = BW — clique: próximo filtro · botão direito: anterior · roda: passo

rc-step = PASSO

rc-step-click-next-tuning-step-right = PASSO — clique: próximo passo de sintonia · botão direito: anterior · roda: passo

rc-agc-click-next-speed-right-click = AGC — clique: próxima velocidade · botão direito: anterior · roda: passo

rc-open = ABERTO

rc-sql-click-open-back-to-the-last = SQL — clique: abrir / voltar ao último limiar · roda: subir ou descer o limiar

rc-s-meter = S-METER

rc-wheel-on-a-digit-click-upper-lower = roda sobre um dígito · clique na metade superior/inferior ± · botão direito zera · duplo clique para digitar

rc-music = MÚSICA ♪

rc-speech = VOZ

rc-stereo-the-19-khz-pilot-is-locked = Estéreo — o piloto de 19 kHz está travado — clique para mono

rc-mono-the-broadcast-stereo-decode = Mono — a decodificação estéreo da transmissão está desligada — clique para ligá-la

rc-mono-noise-reduction-is-on-and = Mono — a redução de ruído está ligada, e o estéreo cede a ela

rc-mono-no-19-khz-pilot-the-station = Mono — sem piloto de 19 kHz: a estação transmite em mono, ou o sinal está muito fraco

rc-stereo = ESTÉREO

rc-group-broadcast = RADIODIFUSÃO

rc-group-ham = RADIOAMADOR

rc-group-air = AÉREA

rc-group-marine = MARÍTIMA

rc-cap-lw-mw-fm = LW / MW · FM

rc-cap-shortwave = SW · ONDAS CURTAS

rc-cap-amateur-bands = BANDAS DE RADIOAMADOR

rc-cap-airband = BANDA AÉREA

rc-cap-marine = VHF · MF MARÍTIMO

rc-cap-cb = FAIXA DO CIDADÃO

rc-preset-fm-ext = FM ext 64–87,5

rc-mem-n = MEM { $id }

controls-action-tune = Sintonizar VFO

controls-action-rit-offset = Deslocamento RIT

controls-action-xit-offset = Deslocamento XIT

controls-action-volume = Volume

controls-action-sub-volume = Volume do sub-RX

controls-action-squelch = Squelch

controls-action-filter-width = Largura do filtro

controls-action-filter-shift = Deslocamento do filtro

controls-action-agc-max-gain = Ganho máx. do AGC

controls-action-manual-gain = Ganho manual (AGC desligado)

controls-action-tx-drive = Drive de TX

controls-action-tune-drive = Drive de sintonia

controls-action-mic-gain = Ganho do microfone

controls-action-digi-audio-freq = Deslocamento de áudio digital

controls-action-spectrum-zoom = Zoom do espectro

controls-action-spectrum-pan = Deslocar espectro

controls-action-spectrum-floor = Piso do espectro

controls-action-spectrum-ceiling = Teto do espectro

controls-action-ptt = PTT

controls-action-tune-carrier = Portadora de TUNE

controls-action-mute = Mudo

controls-action-noise-blanker = Noise blanker

controls-action-nr-cycle = Redução de ruído (alternar)

controls-action-auto-notch = Notch automático

controls-action-binaural = Áudio binaural

controls-action-agc-cycle = AGC (alternar)

controls-action-sub-rx = Sub-receptor

controls-action-split = Split

controls-action-rit-enable = RIT ligar/desligar

controls-action-xit-enable = XIT ligar/desligar

controls-action-rit-clear = Zerar RIT

controls-action-xit-clear = Zerar XIT

controls-action-vfo-toggle = Alternar VFO A/B

controls-action-swap-vfos = Trocar A↔B

controls-action-copy-a-to-b = Copiar A→B

controls-action-band-up = Banda acima

controls-action-band-down = Banda abaixo

controls-action-mode-next = Próximo modo

controls-action-mode-prev = Modo anterior

controls-action-record-toggle = Gravar ligar/desligar

controls-action-abort-tx = Abortar transmissão

controls-action-cw-straight = Manipulador simples CW

controls-action-voice-stop = Parar gravador de voz

controls-action-tone-burst = Tone burst de 1750 Hz

controls-action-fit-span = Ajustar span

controls-action-zoom-in = Aproximar

controls-action-zoom-out = Afastar

controls-action-peak-hold = Retenção de pico

controls-action-spectrum-collapse = Recolher espectro

controls-action-waterfall-collapse = Recolher cascata

controls-action-waterfall-flip = Inverter cascata

controls-action-toggle-help = Janela do manual

controls-action-toggle-settings = Janela de configurações

controls-action-toggle-logbook = Janela do diário de bordo

controls-action-toggle-spots = Janela de spots

controls-action-toggle-memories = Janela de memórias

controls-action-toggle-voice = Janela do gravador de voz

controls-action-workspaces = Janela de espaços de trabalho

controls-action-speak-status = Falar status

controls-action-speak-repeat = Repetir último anúncio

controls-action-speech-silence = Parar de falar

controls-action-speech-toggle = Anúncios ligar/desligar

controls-action-broadcast-labels = Rótulos de radiodifusão (sintonizada / banda toda)

controls-action-select-vfo = Selecionar VFO { $vfo }

controls-action-band = Banda { $band }

controls-action-mode = Modo { $mode }

controls-action-recall-memory = Recuperar memória { $n }

controls-action-voice-slot = Posição do gravador de voz { $n }

controls-group-tuning = Sintonia

controls-group-receive = Recepção

controls-group-transmit = Transmissão

controls-group-display = Exibição

controls-group-windows = Janelas

controls-group-speech = Voz

controls-midi-note = Nota

controls-midi-cc = CC

controls-midi-pitch-bend = Pitch bend

controls-midi-program = Programa

controls-midi-any = qualquer

controls-midi-msg = { $kind } { $data }  ch { $ch }

controls-midi-msg-pitch-bend = Pitch bend  ch { $ch }

settings-controls-down = baixo

settings-controls-rev = inv

antenna-antenna = ANTENA

antenna-cutting-lengths = Comprimentos de corte

antenna-the-frequency-to-cut-for = A frequência para a qual cortar

antenna-optimizing = Otimizando…

antenna-tune-the-yagi-s-element-lengths = Ajusta os comprimentos e espaçamentos dos elementos da Yagi segundo o objetivo escolhido. Uma busca local (hill-climb) em uma thread de segundo plano; a janela continua utilizável enquanto roda.

antenna-solved = Resolvido

antenna-optimized-yagi = Yagi otimizada

antenna-obstacles-approximate = Obstáculos (aproximado)

antenna-trees = Árvores

antenna-in-leaf = com folhas

antenna-ridge = Cume

dab-channel-received-mhz = Canal recebido { $channel } { $mhz }

dab-no-channel-received = Nenhum canal recebido

dab-channel-received = Canal recebido { $channel }

dab-slideshow = SLIDESHOW

dab-no-picture-yet = a estação ainda não enviou imagem

dab-no-station-chosen = nenhuma estação escolhida

dab-no-logo-yet = sem logotipo ainda

dab-playing = ▶ tocando

dab-chosen = escolhida

dab-no-text-yet = a estação ainda não enviou texto

dab-clear-channel-hint = Remove este canal e suas estações. O SCAN o encontra de novo.

dab-clear-all-channels = × remover todos os canais

dab-clear-all-channels-hint = Esquece todos os canais e as estações neles. O SCAN os encontra de novo; as estrelas permanecem.

dab-n-stations = { $n } estações

dab-scanning-progress = escaneando { $channel } · { $at }/{ $n } · { $found } encontradas

dab-clear-station-hint = Remove esta estação da lista. O próximo SCAN a traz de volta se o múltiplex ainda a transportar.

dab-prev = ANT

dab-prev-hint = A estação anterior da lista.

dab-preset = PREDEF

dab-preset-hint = Marcar esta estação com estrela, ou tirar a estrela.

dab-next = PRÓX

dab-next-hint = A próxima estação da lista.

dab-pty-hint = Mostrar o tipo de programa de cada estação na lista.

dab-stop-sweep-hint = Parar a varredura aqui. O que foi encontrado até agora permanece.

dab-scan-hint = Percorre todos os blocos da Banda III e mantém os que transportam estações.

swl-title = LOG SWL

swl-new = + NOVO

swl-jobs = TAREFAS

swl-jobs-hint = Gravações agendadas — grave uma banda em um horário definido

swl-sigid = ID SINAL

swl-sigid-hint = O que há neste dial? Um guia de sinais por frequência, modo e largura de banda

swl-replay = REPLAY

swl-replay-hint = Toca os últimos dois minutos em vez do ao vivo — pegue a identificação da estação que você acabou de perder

swl-report = RELATÓRIO

swl-report-hint = Copiar esta recepção como um relatório para enviar à estação

swl-stats-heard = { $n } ouvidas

swl-stats-reported = { $n } reportadas

swl-stats-qsl = { $n } QSL

swl-stats-pirates = { $n } piratas

swl-stats-hint = O log inteiro — os filtros de Mostrar não o alteram

swl-antenna = Antena

swl-antenna-hint = Fio longo 20 m, loop MLA-30, mini-whip …

swl-antenna-tip = Vai na linha Antenna: do relatório de recepção

swl-tone = Tom

swl-tone-on = ligado

swl-tone-bass = Graves

swl-tone-mid = Médios

swl-tone-treble = Agudos

swl-report-copied = Relatório copiado para a área de transferência — cole-o no seu programa de e-mail. Não foi possível abrir nenhum cliente de e-mail.

swl-show = Mostrar

swl-find-hint = estação, idioma, local …

swl-find-tip = Corresponde à estação, idioma, local do transmissor e notas

swl-all-bands = Todas as bandas

swl-all-days = Todos os dias

swl-pirates-only = Somente piratas

swl-pirates-only-tip = Somente as capturas não licenciadas

swl-adif-tip = Salvar o log inteiro como ADIF — registros de recepção, não contatos

swl-csv-tip = Salvar o log inteiro como CSV, uma linha por recepção

swl-shown-of = { $shown } de { $n }

swl-empty = Nada registrado ainda — sintonize uma estação e pressione + NOVO.

swl-transmitted-from = transmitido de { $site }

swl-click-to-copy = (clique para copiar)

swl-addr-btn = end

swl-mail-btn = e-mail

swl-mark-sent = enviado

swl-recall = rcl

swl-recall-tip = Voltar a esta frequência e modo — veja se a estação retornou

swl-edit = editar

swl-del = excl

swl-no-match = Nenhuma recepção corresponde ao filtro — LIMPAR mostra todas.

swl-form-new = NOVA RECEPÇÃO

swl-form-edit = EDITAR RECEPÇÃO

swl-station-hint = Radio Taiwan International

swl-language-hint = Inglês

swl-received = Recebido

swl-received-hint = seu grid

swl-email = E-mail

swl-email-hint = relatórios de recepção

swl-email-tip = Vai na linha Send to: do relatório

swl-postal = Postal

swl-postal-hint = rua, cidade, país

swl-postal-tip = O destino alternativo do relatório quando não há e-mail

swl-heard = Ouvido

swl-smeter = S-meter

swl-report-label = Relatório

swl-judged = avaliado

swl-report-sent = Relatório enviado

swl-not-yet = ainda não

swl-qsl-received = QSL recebida

swl-awaiting = aguardando

swl-notes = Notas

swl-notes-hint = notas do programa

swl-pirate = Pirata

swl-pirate-tip = Uma transmissão não licenciada — uma estação transmitindo fora de qualquer alocação. O log a marca com uma bandeira pirata.

swl-mail-why-no-address =
    Esta estação não tem endereço de e-mail — procure na janela de GRADE, ou digite um aqui, e isto fica disponível.

    Um relatório de recepção é uma cortesia que a estação lê e responde com um cartão QSL; sem endereço não há para onde enviá-lo.

swl-mail-why-bad-address = Esse endereço de e-mail não é um único endereço, então não é usado — um relatório entregue a um estranho que nunca ouviu a estação é pior que nenhum.

swl-mail-why-ok =
    Escreve o relatório de recepção e o abre no seu cliente de e-mail.

    A imagem e o trecho de áudio ficam por sua conta para anexar lá — este programa não consegue colocar um arquivo em uma mensagem.

swl-mail-report = ENVIAR RELATÓRIO…

antenna-feed-ohms = alim. ≈ { $r } Ω

antenna-takeoff = ângulo de saída { $deg }°

antenna-director = diretor { $n }

antenna-reflector = refletor

antenna-driven = irradiador

antenna-gap = espaç. { $v } m

antenna-az = az

antenna-depth = profundidade

antenna-height = altura

antenna-attenuation = atenuação na direção 0°: { $att } dB

morse-no-paddle-found-connect-one-then = nenhuma palheta encontrada — conecte uma e reabra este painel

morse-send-with-a-paddle-played-here = Envie com uma palheta — tocado aqui, nunca no ar

morse-no-paddle-found-in-dev-input-by-id = Nenhuma palheta encontrada em /dev/input/by-id. Conecte uma e reabra este painel. Uma caixa de manipulador que informe seus dois contatos (muitas vezes como botões de mouse) funciona; uma que faz o próprio manipulador iâmbico e envia toques não funciona.

morse-reverse = INVERTER

morse-swap-dit-and-dah-the-switch-on = Troca ponto e traço. A chave de muitas palhetas faz isso na própria palheta; isto é para quando ela está na outra posição.

morse-stop-key = PARAR MANIP

morse-start-key = INICIAR MANIP

morse-key-the-character-above-the-keyer = Manipule o caractere acima. O manipulador faz os pontos e traços limpos — você escolhe qual palheta e quanto tempo de pausa. Um toque é um elemento; segure para repetir.

morse-the-sending-drill-needs-a-linux = O exercício de envio precisa de um desktop Linux e de uma palheta.

panels-float-chip-hint = FLUTUAR — tira este painel para um painel flutuante e devolve o espaço à cascata; o chip de acoplar nele o recoloca

panels-map-style-radar = Radar — o fundo escuro, terra preenchida. Contra o que se lê um quadro aéreo

panels-map-style-terrain = Relevo — terra realçada e aquecida para parecer terreno, fronteiras atenuadas

panels-map-style-chart = Carta — terra clara, fronteiras destacadas e um reticulado, o visual aeronáutico

panels-heard-me-chip = ME OUVIRAM

panels-heard-me-hint = Circunda toda estação que reportou ter ouvido este indicativo na banda atual, do PSK Reporter, posicionada no grid do próprio reportador. O inverso dos spots PSK: aqueles são estações que este rádio está ouvindo. Atualizado de hora em hora, cobrindo a última hora; precisa do feed PSK ligado em Configurações → Spots e de um indicativo definido na aba Geral.

panels-period-label = período

panels-sub-mode-label = submodo

panels-front-end-overloading = ⚠ FRONT END SOBRECARREGADO — reduza o ganho de RF (LNA/IF), ou afaste a antena

panels-front-end-overloading-hint = O receptor está sendo levado além do fundo de escala, então o áudio é alto mas não carrega sinal — nada será decodificado enquanto isso durar. Reduza o ganho de RF/LNA (mais atenuação), diminua o ganho de IF, ou ative o AGC. Um transmissor forte e próximo, ou outro rádio na mesma banda, é a causa usual.

panels-enigma-hint = Máquina Enigma — cifre uma mensagem em um painel dos tempos de guerra, e resolva um texto cifrado que você copiou do ar

panels-slot-bar-heard-hint = Acompanha o som nos alto-falantes: branca enquanto um quadro é ouvido, verde quando parou, e volta ao início quando o próximo começa. Os tempos são os do modo, então um quadro que termina cedo ainda preenche a barra.


morse-send-correct = correto: { $want }

morse-send-heard = ouvido { $got }, esperado { $want }

morse-send-received = recebido: { $text }

amp-title-model = AMP · { $model }

amp-state-off = OFF

amp-state-on = ON

amp-relay-dn = DN

amp-relay-up = UP

amp-band-unknown = UNK

amp-needle-top = TOP

decodes-not-uploaded = ⚠ { $services } não enviado

decodes-not-uploaded-hint = O contato está no log local, mas o envio falhou. Verifique o token e a conexão em Configurações → Spots, e envie de novo pelo diário de bordo.

decodes-logged-to = ↑ registrado em { $services }

decodes-logged-to-hint = Também enviado aos diários online configurados para receber todo novo contato.

decodes-known = CONHECIDAS

decodes-known-hint = Todo indicativo que um <...> com hash aqui pode resolver no momento, com seu país. Um hash não pode ser lido de trás para frente — uma estação só aparece depois de ser ouvida soletrando seu indicativo, que é para o que serve uma chamada simples em 11 m.

decodes-ignored-clear = { $n } ignoradas · limpar

decodes-auto-tx-frequency-hint =
    Escolhe nossa frequência de transmissão automaticamente: o ponto mais quieto no período em que transmitimos, em vez da frequência de quem estamos respondendo — essa pessoa transmite no outro período, então a frequência dela nada diz sobre quem está lá quando acionamos. Desligado NÃO mantém a frequência: responde na frequência da estação chamada. Para manter, use Hold TX.

    Uma banda é exceção: em 11 m a norma do WSJT-CB é responder a um CQ no tom do próprio chamador e ficar ali durante todo o contato, então a busca é ignorada e uma resposta a CQ fica na frequência em que foi ouvida, com isto ligado ou não.

decodes-muted-caps = SILENCIADA

decodes-muted-hint = Silenciada nesta sessão — clique para ouvir de novo

decodes-ignore-hint = Ignorar esta estação pelo resto da sessão: ela sai da lista de decodificações, deixa de disparar os alertas e nunca é oferecida ao modo automático. Nada é enviado ou transmitido em seu nome, e reiniciar o sdroxide a traz de volta.

decodes-qso-complete-hint = O contato está completo e no log local. Fica aqui por alguns minutos para que a mensagem final possa ser reenviada se a outra estação repetir a dela — nada mais é devido.

decodes-slot-even = par

decodes-slot-odd = ímpar

decodes-before = antes

decodes-after = depois

decodes-beam-hint = Apontar a antena para o localizador desta estação

decodes-queue-next = próxima

decodes-queue-waiting = aguardando


decodes-setup-hint = AJUSTES — a identidade da estação e os modelos que estes modos de QSO compartilham

decodes-auto-armed-hint =
    { $role } está armado. Clique para parar.

    { $note }

    Etapa do motor: { $step } — uma estação que responde a um CQ só é atendida em CallingCq sem contato em andamento.

decodes-auto-rx-hint = Responde ao CQ de uma nova estação, sem supervisão, e nunca chama CQ por conta própria. Só são respondidos indicativos que ainda não estão no log. O watchdog de transmissão dita o ritmo e a parada por inatividade o desarma.

decodes-auto-continuous-hint = O fluxo completo, sem supervisão: terminar o QSO, trabalhar quem está chamando você, responder a uma estação que chama CQ, e chamar CQ só quando não há mais nada. Roda até você parar{ $until }. { $bound }

decodes-auto-or-minutes-up = { " " }ou até { $n } minutos se passarem

decodes-auto-cq-run-unlimited = Cada rodada de CQ não tem limite próprio de chamadas.

decodes-auto-cq-run-bounded = Cada rodada de CQ é limitada pelo seu próprio número de chamadas.

decodes-auto-no-call-limit = Sem limite próprio de chamadas — o watchdog e a parada por inatividade encerram a rodada.

decodes-auto-after-calls-listen = Após { $repeats } chamadas, muda para AUTO RX e continua escutando.

decodes-auto-after-calls-stop = Após { $repeats } chamadas, para.

decodes-auto-cq-hint = Chama CQ sem supervisão e trabalha quem responder. { $bound } Uma rodada curta demais e uma banda quieta nunca o ouve; longa demais e o transmissor fica em uma frequência que ninguém responde.

decodes-auto-stopped-by-operator = { $role } parado pelo operador

decodes-cont = cont

settings-servers-decoded-text-udp = Texto decodificado por UDP

settings-servers-decoded-text-udp-note = Uma linha JSON por decodificação, para um script seu: a família FT8, WSPR e as linhas decodificadas de todo modo de teclado (CW, RTTY, PSK, Olivia, THOR, MFSK, DominoEX, Hell), cada uma com o dial em que foi ouvida. A transmissão WSJT-X acima é um protocolo de logger e não tem mensagem para esses modos, e é por isso que isto existe separadamente.

settings-servers-decoded-text-udp-hint = host:porta, por UDP. Vazio é desligado — o padrão. Somente saída: nada neste socket pode tocar no rádio. Um nome é resolvido ao pressionar APLICAR.

settings-servers-rotator-note = Aponta uma antena motorizada para o satélite em que você travou, ou à mão pela janela ROTATOR e pelos mapas. O padrão é um daemon Hamlib rotctld (execute, por exemplo, “rotctld -m 603 -r /dev/ttyUSB0” para um rotor GS-232B, ou “rotctld -m 1” para testar sem hardware) — um daemon cobre todo rotor que o Hamlib aciona. Um controlador em porta serial também pode ser acionado diretamente.

settings-servers-connection = Conexão

settings-servers-rotctld-port-hint = 4533 é o padrão do rotctld

settings-servers-baud-hint = A maioria dos controladores roda a 9600

settings-servers-pstrotator-note = A porta UDP própria do PSTRotator. Na Setup dele, ative UDP Control; ele responde aos comandos nesta porta e envia as respostas na seguinte.

settings-servers-el = el

settings-servers-persist-re-connect-rotator = Salvar e (re)conectar ao rotor

settings-servers-tci-note = Permite que programas compatíveis com TCI acionem este rádio: controle de frequência e modo, IQ de banda larga, áudio de recepção e áudio de transmissão. No WSJT-X escolha o tipo de rádio SunSDR (TCI), aponte para este endereço e defina o PTT como TCI.

settings-servers-decoded-text-udp-format = Cada linha é um datagrama, terminado em nova linha: { $example }

settings-servers-transport-rotctld = Hamlib rotctld (TCP)

settings-servers-transport-easycomm = EasyComm II (serial)

settings-servers-transport-gs232 = Yaesu GS-232 (serial)

settings-servers-transport-pstrotator = PSTRotator (UDP)

spots-active = ATIVAS

spots-closing = FECHANDO

spots-none-left-in-the-window = nenhuma restante na janela

spots-openings = ABERTURAS

spots-band-opening-detections-paths = Detecções de abertura de banda: caminhos cuja atividade recente superou sua própria linha de base de 3 horas

spots-save-the-spots-you-can-see-as-a = Salva os spots que você vê como um arquivo KML — Google Earth, e qualquer outra ferramenta de mapas que o leia. Somente os filtros ligados, e somente os spots que trazem posição, já que um alfinete sem coordenadas é um alfinete do outro lado do mundo.

spots-no-spots-to-save-the-filters-are = Nenhum spot para salvar — os filtros estão todos desligados

spots-bc-all = BC TODAS

spots-bc-tuned = BC SINTONIZADA

spots-which-broadcasters-the-waterfall =
    Quais emissoras a **cascata** nomeia.

    { "**" }BC SINTONIZADA** é o padrão e a regra do bebepip: o nome na estação em que você está sintonizado, e mais nada — uma banda rotulada de ponta a ponta é um muro de nomes sobre a banda passante que você tenta ler.

    { "**" }BC TODAS** nomeia toda estação que a grade tem no ar agora, onde quer que esteja, que é o que você quer ao percorrer uma banda em vez de escutar uma estação. Nada diz sobre se o chip BC acima mostra emissoras.

app-rx-only-hint = Este rádio só recebe — não tem transmissor para acionar.

spots-n-calls = { $n } chamadas

spots-opening-hover = Caminho na banda de { $band } de { $from } para { $to }: a última janela de 15 minutos superou a linha de base de 3 horas do caminho — chamadores recentes: { $calls }


spots-download-unsupported = { $source } não pôde ser baixado nesta plataforma

spots-kml-partial = { $placed } de { $total } salvos — o restante não tem posição e não pode ser marcado

spots-net-log = LOG DA REDE ({ $n })

decodes-beam = ANTENA

settings-radio-no-answer-yet = Aguardando a máquina à qual o rádio está conectado: estas consultas perguntam sobre o hardware e a rede dela, e ela responde uma de cada vez.

settings-radio-elad-no-baud = o rádio não tem configuração de { $baud } baud — será usado { $default }

settings-radio-elad-antenna-rx-only = Somente RX

settings-radio-rx-only-n = RX{ $n } apenas

settings-radio-ng-level = Nível nG

settings-radio-ng-level-hint =
    O nG adiciona dois comandos que o firmware 2.00x não tem, e este é o único lugar onde podem ser definidos: o nG não responde nem armazena nenhum, então o sdroxide os reenvia sempre que o enlace abre.

    Volume é o nível que o rádio envia pelo fluxo USB. O nG pega esse áudio depois do VOL e do AGC, então este é o lugar certo para corrigir um decodificador que vê pouco ou muito — não o mixer do computador.

    O controle de ganho é a configuração de AGC do próprio rádio. Auto o coloca em DIGI para qualquer modo digital e em ligado nos demais, que é o que as notas do nG pedem para FT8: o ganho então se mantém durante as pausas de transmissão da outra estação em vez de subir regulando dentro delas.

settings-radio-ng-volume-hint = 0–31, como `AG0nn;`. O nível que o rádio envia ao computador.

settings-radio-gain-control = Controle de ganho

settings-radio-speaker = Alto-falante

settings-radio-on-while-streaming = Ligado durante o streaming

settings-radio-ng-speaker-hint = Desligado apaga o alto-falante do próprio rádio enquanto ele faz streaming (`UA2;`), de modo que o áudio é ouvido só no sdroxide. O 2.00x não tinha essa forma.

settings-radio-ng-agc-auto = Auto (DIGI nos modos de dados)

settings-radio-ng-agc-off = Desligado

settings-radio-ng-agc-on = Ligado (rápido)

settings-radio-sound-card-note = Qual placa de som leva o áudio deste rádio está em **Áudio**, junto com seus próprios alto-falantes e microfone — é o mesmo par de configurações qualquer que seja o rádio em uso, então é uma página só.

settings-radio-no-control-cable-audio-page = Este rádio não tem cabo de controle — ele se aciona sozinho por VOX, então não há PTT a comandar. A transmissão é áudio no microfone do rádio, e por isso o áudio de transmissão do rádio está na página Áudio e não aqui.

settings-radio-sound-card-note-short = Qual placa de som leva o áudio está em **Áudio**.

settings-radio-control-link = Enlace de controle

settings-radio-atsmini-host-hint =
    O servidor TCP ad hoc do receptor: `atsmini.local` (mDNS) ou seu IP. No rádio, Settings → TCP Port → Ad hoc, com o Wi-Fi em Connect, AP+Connect ou AP Only.

    O `atsmini.local` precisa de um resolvedor mDNS (Avahi/nss-mdns) nesta máquina. Se o enlace de controle não subir, o IP aparece na tela Wi-Fi do rádio — digite-o no lugar.

settings-radio-atsmini-find-hint = Varre a rede desta máquina em busca da porta de controle do receptor

settings-radio-tcp-port = Porta TCP

settings-radio-tcp-port-hint = O firmware usa 60000 por padrão.

settings-radio-atsmini-receive-only = Somente recepção: o Si4732 demodula em hardware, então o modo escolhido aqui é comandado ao rádio e todo o resto — detecção, decodificação, o log — é feito neste computador.

settings-radio-atsmini-tuning-note =
    A sintonia segue o ciclo de bandas do próprio receptor, então o dial fica um instante atrás da sua rolagem — mais quando a frequência cruza para outra banda. Deixe assentar antes de continuar rolando, e ele sempre chegará à frequência pedida.

    O passo de sintonia do receptor é dele mesmo (o controle STEP acima): uma mudança mais fina que esse passo é arredondada no rádio, e o dial volta ao passo em que realmente caiu.

settings-radio-live-controls = Controles ao vivo

settings-radio-atsmini-vol = VOL

settings-radio-atsmini-step = PASSO

settings-radio-atsmini-sent-at-once = Enviado ao rádio imediatamente. VOL é o volume do próprio receptor (seu botão, e o do Si4732), não o controle de AF do sdroxide.

settings-radio-refresh = Atualizar

settings-radio-read-99-slots = Ler as 99 posições do receptor

settings-radio-tune-radio-slot = Sintonizar este rádio na posição

settings-radio-no-slots-stored = Nenhuma posição armazenada no receptor.

settings-radio-press-refresh-slots = Pressione Atualizar para ler as posições de memória do receptor.

settings-radio-icomnet-receive-hint = AF: o rádio demodula e o sdroxide mostra a banda de áudio. FI de 12 kHz: o rádio envia sua frequência intermediária DRM e o sdroxide demodula, o que traz seus próprios filtros, redução de ruído e decodificadores a atuar sobre cerca de ±12 kHz. Em ambos os casos a cascata larga é o próprio scope do rádio — nenhum Icom entrega I/Q.

settings-radio-copy-icom-session-hint = Copia a última sessão deste rádio — seu handshake e o trace CI-V — para a área de transferência (ou para um arquivo, em um navegador que não deixa a página copiar), para um relatório de bug. Deste rádio, não da estação: com dois Icoms na LAN cada aba responde sobre o próprio endereço.

settings-radio-copy-protocol-trace-hint = Copia o trace de protocolo da última sessão para a área de transferência (ou para um arquivo, em um navegador que não deixa a página copiar), para um relatório de bug.

settings-radio-copy-trace-device-hint = Copia o trace da última sessão para a área de transferência (ou para um arquivo, em um navegador que não deixa a página copiar), para um relatório de bug: todo comando trocado com o dispositivo e os primeiros bytes do fluxo de amostras.

settings-radio-copy-trace-receiver-hint = Copia o trace da última sessão para a área de transferência (ou para um arquivo, em um navegador que não deixa a página copiar), para um relatório de bug: todo comando trocado com o receptor e os primeiros bytes do fluxo de amostras.

settings-radio-pluto-choose-receiver = Escolha seu receptor

settings-radio-dab-rate-hint = Escolha 2,500 Msps se quiser decodificar DAB/DAB+

settings-radio-rx888-dab-rate-hint = Escolha 80,0 Msps e a largura de 2,500 MHz (1/32) se quiser decodificar DAB/DAB+

settings-radio-pluto-both-chains-hint = Para escutar nas duas antenas: adicione um segundo rádio no mesmo endereço e defina-o como RX2. Os dois rádios compartilham a mesma frequência.

settings-radio-duplex-tdd-hint =
    Se o AD9361 opera nas duas direções ao mesmo tempo (FDD, que é como um Pluto inicia e como o sdroxide sempre o deixou) ou uma de cada vez (TDD).

    Deixe em FDD a menos que queira os pinos de PTT abaixo — o TDD é o que eles acionam, e exclui o Full duplex acima. Tem efeito em Aplicar.

    PlutoSDR: o full duplex está desligado — o TDD habilita uma direção por vez, e é isso que os pinos GPO de transmissão-recepção acionam. Defina os pinos de PTT como Desligado e o duplex como FDD para escutar durante uma transmissão.

settings-radio-fm-mw-notch = Notch FM/MW

settings-radio-fm-mw-notch-hint = Notch de hardware sobre as bandas de radiodifusão FM (88–108 MHz) e de ondas médias, para quando um transmissor local sobrecarrega todo o resto. Aplica-se imediatamente.

scanner-broadcast-band = Banda de radiodifusão

scanner-all = TODAS

scanner-stop-only-on-channels-that-sound = Para somente em canais que soam como alguém falando: uma voz é periódica, preenche a banda de voz e sobe e desce algumas vezes por segundo, o que uma portadora, um birdie e uma rajada de dados não fazem. Cada candidato é escutado por pelo menos meio segundo, porque é o que leva para ouvir uma sílaba.

scanner-nothing-in-this-scan-is-in-a-mode = Nada nesta varredura está em um modo que transporta voz — uma busca de voz aqui rodaria e não pararia em lugar nenhum, então isto não é oferecido até haver um. FM, AM e as bandas laterais transportam vozes; os modos digitais não.

scanner-voice = VOZ

scanner-stop-only-where-somebody-is = parar só onde alguém está falando

scanner-no-voice-to-look-for-in-these = sem voz a procurar nestes modos

scanner-sql = SQL

scanner-how-long-to-stay-on-a-candidate = Quanto tempo ficar em um candidato antes de julgá-lo. A verificação de voz precisa de pelo menos isso: uma voz é um ritmo de sílabas, e meio segundo são duas delas

scanner-listening-for-a-voice = escutando em busca de voz…

scanner-scan-band-hover = Varrer { $name } ({ $lo }-{ $hi } kHz)

scanner-range-to = até

scanner-mode-no-speech = { $mode } não transporta voz — passando direto por estes canais

scanner-voice-holding = voz { $score } · alguém está falando

scanner-voice-passed = passou { $mhz } MHz · voz { $score }

aircraft-hfdl-aircraft = Aeronave HFDL { $id }

aircraft-no-icao = sem endereço ICAO (HFDL)

aircraft-provenance = { $source }, há { $age }

aircraft-label-flight = Voo

aircraft-label-registration = Matrícula

aircraft-label-position = Posição

aircraft-label-altitude = Altitude

aircraft-label-speed = Velocidade

aircraft-label-track = Rumo

aircraft-label-last-message = Última mensagem

aircraft-label-type = Tipo

aircraft-label-operator = Operador

aircraft-label-origin = Origem

aircraft-label-destination = Destino

aircraft-label-online = Online

aircraft-missing-flight = ainda não ouvido — ADS-B ou VDL2 o transportam

aircraft-missing-registration = ainda não ouvida — só o VDL2 a transporta

aircraft-missing-position = ainda não ouvida — ADS-B, ou um relatório de posição VDL2 ou HFDL

aircraft-missing-adsb = ainda não ouvido — o ADS-B o transporta

aircraft-missing-message = nenhuma ainda — VDL2, ou áudio ACARS depois que o VDL2 nomear a aeronave

aircraft-online-unavailable-disabled = online indisponível — a chave Online está desligada

aircraft-online-unavailable-no-station = online indisponível — não há estação a consultar; os dados de rádio são tudo o que há

aircraft-online-unavailable-unauthorized = online indisponível — não conectado à estação

aircraft-online-unavailable-retry = consulta online falhou — nova tentativa em breve

aircraft-switch-no-answer = a estação não respondeu

aircraft-switch-not-signed-in = não conectado à estação

aircraft-switch-refused = a estação recusou o pedido

aircraft-switch-too-old = a estação não oferece esta chave (versão antiga?)

aircraft-switch-failed = a estação não conseguiu alterá-la

aircraft-online-source = adsbdb, pela estação

aircraft-asking-station = consultando a estação

aircraft-provenance-radio = rádio

aircraft-provenance-radio-online = rádio + online

aircraft-provenance-online = online

aircraft-photo-credit = Foto: { $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = Nenhum rotor de antena está configurado.

rotor-set-one-up-in-settings-servers-a = Configure um em Configurações ▸ Servidores — um daemon Hamlib rotctld, ou um controlador em porta serial. Depois aponte o feixe a partir deste dial, de um indicativo, ou do mapa.

rotor-halt-the-antenna-where-it-is = Parar a antena onde está

rotor-park = ESTACIONAR

rotor-hand-the-antenna-back-to-the = Devolve a antena ao travamento no satélite — ela acompanha o satélite, e estaciona quando não há nenhum.

rotor-point = APONTAR

rotor-dx-country = País DX

rotor-set-your-grid-locator-settings = Defina seu localizador grid (Configurações ▸ Geral) para apontar para um país.

rotor-connected = Conectado — antena em { $az }°

rotor-not-connected-why = Não conectado: { $e }

rotor-beam = Feixe { $az }° az  { $el }° el

rotor-target = alvo { $az }° / { $el }°

rotor-park-hover = Girar para o rumo de estacionamento ({ $az }° / { $el }°)

rotor-country-bearing = { $name } — rumo { $az }°

aircraft-switch-no-station = não há estação para alternar

aircraft-title = AERONAVES

aircraft-mode-adsb-hover = ADS-B, 1090 MHz — aeronaves com posição, altitude e velocidade

aircraft-mode-hfdl-hover = HFDL, enlace de dados HF — aeronaves sobre oceanos e polos, a partir de seus relatórios de posição

aircraft-mode-vdl2-hover = VDL2, enlace de dados VHF — mensagens de aeronaves; poucas trazem posição

aircraft-mode-acars-hover = ACARS — mensagens curtas entre aeronaves e o solo

aircraft-badge-online = online

aircraft-badge-history = histórico

aircraft-stats-total = { $n } aeronaves

aircraft-stats-most-first = Mais frequentes primeiro

aircraft-stats-empty = Nenhuma aeronave ainda. As tabelas se preenchem a partir de ADS-B, VDL2, HFDL e ACARS; nada aqui precisa de internet.

aircraft-stats-relay-note = Modelo, operador e rota contam apenas as aeronaves sobre as quais o relé respondeu — abra um cartão em Cartões para consultar uma.

aircraft-stats-known-kinds = { $known } conhecidas · { $kinds } tipos

aircraft-and-more = e mais { $more }

aircraft-show = Mostrar

aircraft-aviation = Aviação

aircraft-cards-chip = Cartões { $n }

aircraft-cards-hover = Toda aeronave ouvida em ADS-B, VDL2, HFDL e ACARS, um cartão cada — mantidos por dez minutos, qualquer que seja o modo atual do rádio

aircraft-watch-chip = Acompanhar { $n }{ $suffix }

aircraft-watch-hover = Aeronaves sobre as quais você pediu aviso, por indicativo ou matrícula — a lista é mantida pela estação, e um alerta é exibido quando uma entra no ar

aircraft-stats-chip = Estatísticas

aircraft-stats-hover = As categorias, companhias, modelos, operadores e rotas mais comuns entre as aeronaves que esta estação conhece — contadas primeiro a partir do rádio, com os cartões do relé como complemento

aircraft-export = Exportar

aircraft-export-csv-hover = Salvar as aeronaves como tabela CSV para uma planilha: as últimas 24 horas da estação se ela mantém histórico, senão o que é ouvido agora. Sem posições.

aircraft-export-json-hover = Salvar as aeronaves como JSON: as últimas 24 horas da estação se ela mantém histórico, senão o que é ouvido agora. Sem posições.

aircraft-watch-intro = Indicativos e matrículas sobre os quais você quer ser avisado. A lista fica guardada na estação, então é a mesma em todas as telas; um alerta aparece quando um deles surge, ou volta ao ar depois de dez minutos fora.

aircraft-watch-needs-station = A lista de vigilância fica guardada na estação à qual você está conectado. Conecte-se a uma para usá-la.

aircraft-watch-reading = Lendo a lista…

aircraft-watch-cannot-read = Não foi possível ler a lista: { $why }.

aircraft-watch-input-hint = KLM1234 ou PH-BXA

aircraft-watch-add = Adicionar

aircraft-watch-sending = Enviando…

aircraft-watch-empty = Nada na lista ainda.

aircraft-watch-on-air = no ar agora

aircraft-watch-remove = Remover

aircraft-watch-alerts = Alertas

aircraft-watch-clear = Limpar

aircraft-watch-none-yet = Nenhum ainda.

aircraft-cards-empty = Nenhuma aeronave ouvida ainda. Os cartões são preenchidos por ADS-B, VDL2, HFDL e ACARS — pressione um deles e escute. Nada aqui precisa da internet.

aircraft-cards-no-match = Nenhuma aeronave corresponde a “{ $query }”.

aircraft-history-hover = Mostra por onde esta aeronave passou no último dia, a partir do registro da própria estação. Consulta a sua estação, nunca a internet.

aircraft-copy-summary = Copiar resumo

aircraft-copy-summary-hover = Copia este cartão como texto: o que se sabe e de onde veio cada linha. A posição e a última mensagem ficam de fora.

aircraft-asking-station-dots = consultando a estação…

aircraft-not-changed = Não alterado: { $why }.

aircraft-loading-photo = carregando foto…

aircraft-radio-is-on = O rádio está em { $mode } — pressione { $label } para escutar. Isto é o que { $label } ouviu antes.

aircraft-no-adsb = Nenhum ADS-B ouvido ainda.

aircraft-hfdl-off = O decodificador HFDL está desligado — pressione HFDL para ligá-lo.

aircraft-vdl2-starting = Iniciando o decodificador VDL2…

aircraft-no-view = { $label } não tem visualização aqui.

textmodem-waiting-modem = aguardando o modem…

textmodem-locked = TRAVADO

textmodem-no-frame = SEM QUADRO

textmodem-locked-hint = Um quadro está sendo acompanhado. Ele só é aceito quando a correlação de sincronismo, somada ao longo de doze quadros, se destaca claramente do próprio nulo e a autocorrelação do período de quadro indica que há um quadro presente.

textmodem-sync-readout = sinc { $sync } (nulo { $null })

textmodem-sync-hint = A correlação diferencial de sincronismo somada sobre todas as fases do quadro, e a mediana dessas fases — o nulo que ela precisa superar. Medido no ar: 0,85 em um 4285 contra 0,14 em uma portadora PSK forte de outro modo.

textmodem-frame-readout = quadro { $acf }

textmodem-frame-hint = Quanto de um quadro se repete um quadro depois. Um 4285 repete seus 80 símbolos de sincronismo, portanto 0,3-0,6; a portadora de outro modo mede cerca de 0,03.

textmodem-refs-readout = refs { $a }/{ $b }/{ $c }

textmodem-refs-hint = As concentrações dos três blocos de referência após o descrambling. Todos são símbolo 0 antes do scrambling, então um quadro correto colapsa cada um deles em um único ponto — 0,7 ou mais. Dezesseis símbolos de ruído dão 0,25.

textmodem-rate-hint = O que a constelação consegue dizer. Cada taxa não codificada divide sua constelação com uma codificada, então as taxas codificadas precisam do Annex E para serem distinguidas — e seu payload precisa do FEC do Annex E para ser lido.

textmodem-offset-frames = { $offset } Hz · { $n } quadros

textmodem-offset-frames-hint = O deslocamento de portadora em que os blocos de referência concordam, e quantos quadros foram demodulados nesta sessão.

textmodem-som-readout = SOM { $best }/32 · { $n } msg

textmodem-som-hint = O padrão de início de mensagem do Annex E e o quanto a saída do decodificador chegou perto dele. O ruído fica perto de 16 de 32; uma transmissão codificada chega aos vinte e poucos mesmo quando um fading impede a correspondência, e uma correspondência é o que identifica a taxa codificada e inicia uma mensagem. O código e o entrelaçador decodificam exatamente um sinal gerado, mas nenhuma estação codificada foi lida no ar até agora, então uma correspondência aqui é a primeira coisa a esperar quando houver uma.

textmodem-rx-only = SÓ RX

textmodem-rx-only-hint = O decodificador deste modo está portado e lê suas gravações; o modulador ainda não foi escrito, então a parte de envio fica desligada em vez de recusar depois do fato.

textmodem-msg-hint = Seus próprios botões de mensagem — condições de trabalho, o clima, uma resposta padrão. Cada um envia o texto inteiro de uma vez, e F1–F9 acionam os nove primeiros. Eles viajam com a configuração da estação, então um cliente remoto também os tem.

textmodem-weather-chip = {" "}CLIMA { $n }{" "}

textmodem-weather-hint =
    Relatórios SYNOP e SHIP decodificados ouvidos nesta banda.

    Ficam guardados aqui e não na lista de decodificação porque os números sozinhos não significam nada — e porque o tempo numa posição de três horas atrás é o que você volta para consultar.

    Encontrados no texto RTTY conforme ele chega, então o modo precisa estar ajustado ao shift e ao baud da estação. Uma estação em terra não envia posição, então essas linhas trazem o número da estação e os valores, mas sem coordenadas; um relatório de navio traz a sua própria.

textmodem-olivia-reverse-hint = Inverte a ordem dos tons, nos dois sentidos. Uma estação transmitindo invertida soa como ruído até isto ser ativado.

textmodem-olivia-tones-raised = ({ $n } do Olivia → 4)

textmodem-ifk-reverse-hint = Inverte os passos de tom. O IFK+ leva seus dados no passo entre tons sucessivos, então uma estação recebida na banda lateral oposta tem todos os passos negativos e soa como ruído até isto ser ativado.

textmodem-domino-hint = { $name } — { $baud } baud, 18 tons, modulação incremental de frequência sem correção de erros direta

textmodem-mt63-long-hint = MT63-{ $label }, { $bw } Hz de largura — o entrelaçamento longo espalha uma rajada por { $chars } caracteres

textmodem-mt63-short-hint = MT63-{ $label }, { $bw } Hz de largura, entrelaçamento curto — o entrelaçamento curto espalha uma rajada por { $chars } caracteres

textmodem-mt63-not-offered = 1K e 500L ainda não disponíveis

textmodem-mt63-not-offered-hint = O MT63-1K e o entrelaçamento longo de 500 Hz são lidos pelo decodificador em lote, mas ainda não pelo receptor ao vivo: o travamento procura a fase do símbolo e a polaridade, mas não o centro da banda, e uma busca que o faça trocava uma variante por outra. Veja docs/internal/MODE-PROGRESS.md.

textmodem-mfsk-caveat-mfsk8 = { " " }— a gravação do corpus perde um trecho do texto; o decodificador é o mesmo usado por todas as outras variantes

textmodem-mfsk-caveat-mfsk11 = { " " }— a gravação do corpus perde o grupo inicial por causa da partida do reamostrador

textmodem-mfsk-hint = { $label } — { $baud } baud, { $tones } tons, { $bw } Hz de largura{ $caveat }

textmodem-mfsk-reverse-hint = Inverte a ordem dos tons. Uma estação recebida na banda lateral oposta àquela em que foi transmitida chega espelhada em torno do seu centro e soa como ruído até isto ser ativado.

textmodem-psk-family = Família

textmodem-psk-family-plain-hint = A família própria do PSK31: uma portadora diferencial, sem correção de erros direta

textmodem-psk-family-bpsk63f-hint = O código robusto a 62,5 baud, sem entrelaçador — o único modo codificado que o fldigi envia sem um

textmodem-psk-family-robust-hint = Uma portadora com o código robusto e um entrelaçador 2x2xN

textmodem-psk-family-multi-hint = Várias portadoras, varicode do PSK31 e **sem** código — mais largo e mais rápido, e exige um sinal limpo

textmodem-psk-family-robust-multi-hint = Várias portadoras **e** o código robusto: o mais largo e o mais robusto dos modos PSK

textmodem-psk-baud-fast-hint = { $baud } baud — lê a partir de cerca de um segundo: nesta taxa, o tempo de travamento do receptor é um terço de uma mensagem

textmodem-psk-baud-hint = { $baud } baud

textmodem-psk-modulation = Modulação

textmodem-psk-bpsk-hint = Um bit por símbolo, detectado diferencialmente

textmodem-psk-qpsk-hint = Dois bits por símbolo com um código de taxa 1/2, ou seja, um bit de dados por símbolo: a mesma taxa de texto do BPSK na mesma velocidade, com metade dos erros

textmodem-psk-8psk-hint = Três bits por símbolo, sem código: o mais rápido dos três em qualquer velocidade e o menos robusto. Somente recepção — não existe transmissor de oito fases. Comprovado em sinal sintético: as gravações 8PSK do corpus de testes não são uma constelação limpa

textmodem-psk-coded = codificado

textmodem-psk-no-code = sem código

textmodem-psk-coded-hint =
    { $baud } baud, { $n } { $n ->
            [one] portadora
           *[other] portadoras
        }, { $code }, { $bw } Hz de largura — somente recepção: o transmissor PSK codificado não foi escrito

textmodem-psk-wide-bank-hint = { "*" }*Este banco largo ainda não decodifica.** Suas portadoras estão tão próximas que o filtro do receptor na taxa de entrada não consegue separá-las; os bancos mais estreitos leem suas próprias gravações. Nada do que você ouve aqui se perde — é o front end, não o sinal.

textmodem-hell-slant = Inclinação

textmodem-hell-slant-hint = Ajuste do clock de amostragem na recepção. Uma captura cujos pontos chegam 0,4 % mais longos — uma placa de som comum, e o que a gravação Hell off-air do SkySweep mede — precisa de cerca de -4000. Ajuste até os caracteres ficarem na vertical.

settings-net-rbn-enabled-hint = Lê os skimmers do mundo todo e alimenta o mapa de propagação com eles. É isso que faz o mapa mostrar bandas que este rádio não está escutando. Não coloca nada no ar, não precisa de conta e usa o indicativo da aba Geral. Os spots do RBN não aparecem na lista de spots — são medições, não convites. O host não está preso a reversebeacon.net: qualquer nó que fale o mesmo feed telnet serve.

settings-net-rbn-port-hint = No reversebeacon.net, 7000 é o feed de CW/RTTY e 7001 o de FT8/FT4

settings-net-wavelog-intro = O Wavelog é um logbook que **você** mesmo hospeda, em um endereço que você informa. Os contatos registrados aqui são enviados para o local de estação que você escolher, e os contatos que ele já tem podem ser trazidos de volta para que o “já trabalhado” conheça QSOs feitos em outro programa, outro rádio ou no seu próprio formulário web.

settings-net-wavelog-address-hint = Onde fica a página inicial do Wavelog: https://log.example.org, ou seu endereço na LAN. /index.php é aceito de qualquer forma.

settings-net-wavelog-key-hint = Uma chave de API da página de API do próprio Wavelog. Uma chave de leitura/escrita é necessária para enviar; uma somente de leitura só consegue baixar.

settings-net-pick-one = escolha um

settings-net-test-to-list-locations = teste a conexão para listar os locais

settings-net-trust = CONFIAR

settings-net-trust-hint = Aceita **este** certificado e nenhum outro, para este servidor. Uma fixação, e não “aceitar qualquer coisa”: se ele mudar um dia, a conexão é recusada e isso é informado, em vez de ir silenciosamente para o que quer que responda naquele endereço.

settings-net-test-wavelog = Testar Wavelog

settings-net-test-wavelog-hint = Pergunta ao servidor se este endereço e esta chave funcionam, e lista seus locais de estação. Aplica as configurações antes; não registra nada.

settings-net-wavelog-download = Baixar contatos do Wavelog

settings-net-every-min = A cada (min)

settings-net-locations = Locais

settings-net-download-now = Baixar agora

settings-net-download-now-hint = Traz o que o Wavelog adicionou desde o último download e marca como enviados os contatos que ele já tem. Nada que já esteja no logbook é alterado.

settings-net-wavelog-upload-note = Somente contatos registrados **aqui** são enviados automaticamente; o backlog do próprio Wavelog é enviado marcando um QSO no logbook, de modo que nada sai desta máquina sem que você tenha escolhido.

settings-net-broadcast-note = As estações de ondas longas e curtas identificadas no waterfall. Cada fonte abaixo é baixada para a temporada de radiodifusão atual e baixada de novo quando a temporada muda. Todas vão para **um único repositório**, então o filtro Fonte da janela PROGRAMAÇÃO mostra uma delas ou todas.

settings-net-downloaded = baixado

settings-net-schedules = Programações

settings-net-schedule-yours = sua

settings-net-download = Baixar

settings-net-hfcc-hint = O HFCC publica os nomes de suas estações em um arquivo separado, então são necessários os dois. Enquanto isso não ocorre, ele avisa em vez de carregar uma programação só com indicativos.

settings-net-refetch-hint = Busca de novo a cópia desta temporada, substituindo a que está em cache

settings-net-from-file = De arquivo…

settings-net-aoki-file-hint = Usa a programação do Aoki a partir de um arquivo que você mesmo salvou — o .zip que o Aoki publica, ou o .txt de dentro dele. O site dele muda a cada temporada, e é por isso que isto existe.

settings-net-eibi-file-hint = Usa a programação do EiBi a partir do seu sked .csv, salvo nesta máquina

settings-net-filter-aoki = Programação Aoki

settings-net-filter-eibi = Programação EiBi

settings-net-download-all = Baixar tudo agora

settings-net-download-all-hint = Busca todas as fontes que podem ser buscadas, mantendo as que já funcionam

settings-net-own-list-note = Sua própria lista é uma **programação** — o que você espera ouvir — e não o log de recepção, que registra o que você realmente ouviu. Importe-a abaixo, ou edite o arquivo.

settings-net-import-list = Importar minha lista de estações…

settings-net-import-list-hint = Lê um CSV ou uma lista simples de estações e mescla no arquivo acima. As frequências podem estar em kHz ou MHz; a unidade é deduzida pelo número.

settings-net-filter-station-lists = Listas de estações

settings-net-dx-status = DX — { $body }

settings-net-pin-own = a sua própria — { $pin }…

settings-net-schedule-rows = { $name } { $n } transmissões

settings-net-schedule-not-downloaded = { $name }     não baixado

settings-net-schedule-imported = { $source } { $season } — { $n } transmissões, agora no repositório

settings-net-stations-imported =
    { $n ->
        [one] { $n } estação na sua lista, agora no repositório
       *[other] { $n } estações na sua lista, agora no repositório
    }

txaudio-natural = Natural

txaudio-ragchews-and-local-contacts-the = Papos longos e contatos locais: o redutor de ruído, um expansor suave nas pausas, compressão leve. Soa como você, menos a sala.

txaudio-ssb-everyday = SSB do dia a dia

txaudio-everyday-ssb-100-hz-2-9-khz-a = SSB do dia a dia: 100 Hz – 2,9 kHz, um pouco de presença em torno de 2 kHz, compressão moderada.

txaudio-ssb-punch-dx = SSB punch / DX

txaudio-weak-signals-and-pileups-a-narrow = Sinais fracos e pileups: voz estreita de 300 Hz – 2,7 kHz com forte realce de presença, levada a um compressor rápido com o rotador de fase, para a máxima potência de fala que o transmissor tem. Não é hi-fi.

txaudio-ssb-punch-for-hours-in-a-noisy = SSB punch por horas em uma estação ruidosa: o redutor de ruído e um gate forte mantêm ventiladores e o próximo operador fora das suas pausas, e o compressor trabalha ainda mais.

txaudio-essb-hi-fi = ESSB / hi-fi

txaudio-wide-ssb-or-am-50-hz-4-khz-with = SSB largo ou AM: 50 Hz – 4 kHz mantendo os graves, compressão suave e um de-esser. Exige um filtro de transmissão tão largo — e uma banda silenciosa.

txaudio-fm-repeater = FM / repetidora

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM: um passa-altas íngreme de 300 Hz mantém a voz longe do tom CTCSS, o topo para em 3 kHz por causa do canal, e o limitador segura o desvio.

txaudio-mobile-noisy-shack = Móvel / shack ruidoso

txaudio-fans-engine-traffic-the-denoiser-a = Ventiladores, motor, trânsito: o redutor de ruído, um passa-altas íngreme em 150 Hz e um gate forte, com compressão suficiente para ficar acima do que sobra.

txaudio-digital-voice-2 = Voz digital

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE: apenas o redutor de ruído e um nivelamento suave, sem EQ — o vocoder prefere fala natural. Ativa VOZ DIGITAL.

txaudio-imported-2 = Importado

frame-no-contrast-fit = Sem contraste — clique para AJUSTAR

frame-levels-hidden-hint = O piso/teto do display estão além de tudo o que há na tela, então o waterfall é de uma só cor. Clique para ajustá-los ao que o rádio está ouvindo.

frame-bw-waterfall = Waterfall BW · { $span }

frame-bw-waterfall-unknown = Waterfall BW · —

frame-every-module-own-window = cada módulo está em sua própria janela

frame-window-chip-hint = JANELA — move esta janela para uma janela própria do sistema, para colocá-la em outro monitor

frame-dock-panel-pane-hint = ACOPLAR — devolve este painel ao painel abaixo do waterfall

frame-dock-panel-hint = ACOPLAR — coloca este painel de volta sob o waterfall

frame-dock-spectrum-hint = ACOPLAR — devolve este espectro à janela principal, o mesmo que fechar esta sem procurar o botão de fechar.

frame-dock-band-menu-hint = ACOPLAR — devolve o seletor de banda e modo à janela principal, o mesmo que fechar esta sem procurar o botão de fechar.

frame-dock-rotator-hint = ACOPLAR — devolve o rotador à janela principal, o mesmo que fechar esta sem procurar o botão de fechar.

frame-module-panadapter = panadapter

frame-module-panel = painel

frame-module-controls = controles

frame-module-aux-panadapter = panadapter aux

frame-module-band-mode = banda e modo

frame-module-rotator = rotador de antena

frame-detached-title = { $flavor } — { $name }

frame-workspace-missing = Nenhuma área de trabalho { $n } salva — salve uma em Configurações › UI › Áreas de trabalho e esta tecla a chamará de volta.

frame-no-operating-panel = Sem painel de operação em { $mode } — ele está desacoplado, e este modo não tem nenhum.

frame-rx-only-nudge = Este rádio é somente receptor — não tem transmissor. Ocultar os controles de transmissão e usar a tela de escuta?

frame-listening-controls = Controles de escuta

frame-listening-controls-hint = Oculta os controles de transmissão e muda este rádio para a tela de escuta (Configurações → Rádio). Nada é resintonizado e o decodificador continua funcionando.

frame-update-available = O SDR Oxide Brown { $tag } está disponível — você está na { $version }.

frame-download = Baixar

frame-download-hint = Abre a página da versão no seu navegador

frame-tuning-catching-up = sintonizando — o rádio está acompanhando

frame-roll-up-hint = Recolher — sai da barra de título e devolve a janela ao waterfall

frame-unroll-hint = Expandir — mostra o painel de novo

frame-window-panel-hint = JANELA — dá a este painel uma janela própria, para colocar em outro monitor

frame-panel-window-title = { $mode } — painel

frame-auto-stopped-stop-control = parada automática: um controle de parada foi usado

frame-dont-show-again = Não mostrar novamente

frame-dont-show-again-hint = Lembra este aviso nesta tela. O bloqueio continua desativado enquanto o motor roda com --oob-tx — isto apenas impede que o aviso seja mostrado de novo.

frame-cb-window-title = ⚠  11 m (PX) — NÃO É UMA BANDA DE RADIOAMADOR

frame-cb-about-to-allow = Você está prestes a permitir transmissão na banda de 11 m.

frame-cb-not-amateur = 11 m não é uma banda de radioamador. É um serviço de rádio separado, com seus próprios canais, modos, limites de potência e equipamentos homologados, e isso varia de país para país. Transmitir ali está sujeito às regras do país de onde você opera.

frame-cb-only-continue = Só continue se você sabe o que o seu país permite em 27 MHz e está operando dentro disso — você continua sendo responsável pela transmissão.

frame-cancel = Cancelar

frame-saved-to-profile = salvo no perfil { $name }

frame-saved-as-default = salvo como padrão desta estação (o servidor não tem senha, então é isto que todo cliente recebe)

frame-bindings-offer-title = Este perfil traz atalhos de teclado

frame-bindings-offer-profile = O perfil com o qual você entrou ({ $name }) guarda atalhos de teclado e mouse no servidor, e este navegador não os está usando. Esse é o padrão seguro — numa estação compartilhada com outras pessoas, o teclado também é compartilhado, e um login adotar o PTT ou o Espaço de outro é um risco real.

frame-bindings-offer-default = O perfil padrão desta estação guarda atalhos de teclado e mouse no servidor, e este navegador não os está usando. Esse é o padrão seguro — numa estação compartilhada com outras pessoas, o teclado também é compartilhado, e um login adotar o PTT ou o Espaço de outro é um risco real.

frame-bindings-offer-choice = Use-os aqui, ou deixe-os de lado. De qualquer forma você será informado de qual foi, e nada muda sem que você escolha.

frame-bindings-use = Usar os atalhos deste perfil

frame-bindings-keep-mine = Manter os meus (e não perguntar mais)

frame-bindings-using-profile = usando os atalhos de teclado guardados para este perfil

frame-bindings-keeping-own = mantendo os atalhos de teclado deste dispositivo; não será perguntado de novo

frame-look-back-to-profile = de volta ao visual guardado para este perfil

frame-look-none-stored = ainda não há visual guardado para este perfil — salve um primeiro

frame-antenna-rotator-caption = ROTADOR DE ANTENA

bands-condx-global-forecast-rest-what = CONDX é uma previsão global. WSPR é o que a rede WSPR do mundo ouviu (wspr.live); PATHS, REACH e BEST são o que esta estação e o RBN ouviram. Uma linha vazia significa que nada foi decodificado — o que pode indicar que a banda estava fechada, ou apenas que ninguém estava nela.

bands-km-s-parent-peak-rate =
    { $name } ({ $code }) — { $velocity_kms } km/s, origem { $parent }. Taxa de pico ZHR { $zhr } por volta de { $peak }.

    Um radiante acima do horizonte significa que os rastros podem alcançar você; uma chuva rápida deixa rastros ionizados mais duradouros para meteor scatter em 6 m e 2 m, e as breves aberturas em 10 m/11 m.

bands-hearing-means-path-open-each = { $callsign } em { $location } — ouvi-lo significa que o caminho de { $band } está aberto. 10m (28,200 MHz) é o substituto mais próximo em banda de radioamador para as condições de 11 m. Cada baliza sobe uma banda a cada 10 s, então esta linha muda a cada slot.

bands-wspr-activity-old = · atividade WSPR com { $age } (wspr.live)

bands-psk-activity-old = · atividade PSK com { $age }

bands-activity-hover =
    Atividade global em { $band } nos últimos 15 minutos, de { $source }:
    { $paths } relatórios de recepção · { $tx } transmissores · { $rx } receptores

    { $note }

    Aparece mais forte quanto mais movimentada a banda; indica que a banda está sendo ouvida em algum lugar, não que esteja aberta para você.

bands-derived-hover =
    { $band } não é uma das bandas que o HAMQSL.com classifica; este é o veredito do grupo { $group } publicado, lido como o substituto mais próximo.

    Uma previsão, não uma medição do seu próprio caminho.

bands-wspr-note = A rede WSPR do mundo — balizas de sinal fraco.

bands-psk-note = Relatórios de recepção dos modos de atividade: FT8, FT4 e os relatórios de CW/RTTY que as balizas WSPR não cobrem.

aircraft-history-reason-disabled = a estação não ativou seu histórico

aircraft-history-reason-unauthorized = a estação exige login

aircraft-history-reason-no-station = o histórico é guardado pela estação à qual você está conectado

aircraft-history-reason-unknown = a resposta da estação não foi compreendida

aircraft-history-show = Mostrar histórico

aircraft-history-hide = Ocultar histórico

aircraft-label-history = Histórico

aircraft-label-first-heard = Primeira escuta

aircraft-label-last-heard = Última escuta

aircraft-label-track-points = Pontos da trilha

aircraft-label-highest = Mais alto

aircraft-label-callsigns = Indicativos

aircraft-label-registrations = Matrículas

aircraft-history-nothing-kept = nada guardado para esta aeronave ainda

aircraft-history-by-station = por esta estação

aircraft-history-kept-24h = guardado nas últimas 24 h

aircraft-history-of-kept = dos pontos guardados

aircraft-history-used-24h = usado nas últimas 24 h

aircraft-history-seen-24h = visto nas últimas 24 h

aircraft-history-looked-up = consultado há { $age }

aircraft-alert-on-air-as = { $entry } está no ar, como { $callsign }

aircraft-alert-on-air = { $entry } está no ar

aircraft-watch-already = Já está na lista.

aircraft-watch-absent = Não está na lista.

aircraft-watch-invalid = Não é um indicativo nem uma matrícula: de 3 a 10 letras ou dígitos.

aircraft-watch-full = A lista está cheia: 100 aeronaves.

multi-module-windows = JANELAS DE MÓDULOS

multi-in-its-own-window = EM JANELA PRÓPRIA

multi-docked = ACOPLADO

multi-dock-all-windows = ACOPLAR TODAS AS JANELAS

multi-bring-every-module-window-back = Traz todas as janelas de módulos de volta à janela principal

multi-undock-all-windows = DESACOPLAR TODAS AS JANELAS

multi-put-every-module-in-a-window-of = Coloca cada módulo em uma janela própria — a janela principal fica com o espectro e os decodificadores

multi-two-radios-side-by-side-need-a = Dois rádios lado a lado precisam de uma janela mais larga — esta tela mostra um por vez

multi-the-station-s-windows-some-modules = As janelas da estação — alguns módulos estão em janelas próprias do sistema. Abra isto para alterar.

multi-put-a-module-in-its-own-window-sp1 = Coloque um módulo em janela própria — SP1 ou AUX SP em outro monitor, o controle RX ao lado deles. As mesmas linhas estão em Configurações → UI → Desacoplados.

multi-module-sp1 = SP1 — o espectro

multi-module-sp2 = SP2 (AUX) — um segundo espectro

multi-module-panel = Painel de operação

multi-module-controls = Controle RX

multi-module-band-mode = Banda e modo

multi-module-rotor = Rotador de antena

multi-bring-back = Trazer { $name } de volta à janela principal

multi-put-own-window = Colocar { $name } em uma janela própria do sistema — com seu próprio app-id, para que uma regra de janela possa fixá-la em um monitor

multi-windows-chip = JANELAS

aircraft-stats-category = Categoria

aircraft-stats-airline = Companhia aérea

aircraft-stats-model = Modelo

aircraft-stats-operator = Operador

aircraft-stats-route = Rota

aircraft-stats-src-no-station = Contando as aeronaves ouvidas agora. A estação à qual você se conecta guarda o último dia.

aircraft-stats-src-asking = Consultando a estação sobre o último dia…

aircraft-stats-src-ready = As últimas 24 h guardadas pela estação, com as aeronaves ouvidas agora.

aircraft-stats-src-disabled = A estação não ativou seu histórico — contando as aeronaves ouvidas agora.

aircraft-stats-src-reason = { $reason } — contando as aeronaves ouvidas agora.

aircraft-export-saved-station = { $name } salvo: as últimas 24 horas da estação.

aircraft-export-saved-local = { $reason }: em vez disso, foram salvas as { $n } aeronaves ouvidas nos últimos dez minutos.

login-remember-this-station = Lembrar desta estação

login-remember-me = LEMBRAR DE MIM

login-this-station-keeps-your-sign-in = Esta estação guarda o seu login, não esta página — nada a preencher nos outros rádios, e nada a esquecer quando você sair.

login-not-this-time-this-browser-forgets = Desta vez não: este navegador esquece quando é fechado, então você será perguntado de novo na próxima sessão e nada deste login fica no dispositivo.

aircraft-label-range = Alcance

aircraft-label-vertical = Vertical

aircraft-label-emergency = Emergência

aircraft-from-your-station = da sua estação

aircraft-footer-disabled = online indisponível — relay desligado na estação

aircraft-footer-no-station = online indisponível — sem estação para consultar

aircraft-footer-unauthorized = online indisponível — sem login

aircraft-hover-close = Fechar (Esc)

aircraft-hover-lookup = Consultar online

aircraft-hover-lookup-hint = Pergunta ao relay desta estação sobre esta aeronave: tipo, operador, rota e uma foto. Nada é consultado até você pressionar.

aircraft-hover-lookup-waiting = outra consulta está em andamento — uma de cada vez

aircraft-hover-history-hint = Por onde esta aeronave passou no último dia, a partir do registro da sua própria estação. Consulta a sua estação, nunca a internet.

auto-stopped-reason = parada automática: { $reason }

auto-stopped-idle = parada automática: { $mins } minutos sem ação do operador

auto-answering = { $role }: respondendo a { $call }

auto-caller-went-away = auto: um chamador foi embora

auto-nobody-left = auto: ninguém mais para responder

auto-calling-cq = { $role }: chamando CQ (#{ $n })

auto-cq-no-answer = auto cq: sem resposta após { $limit } chamadas — escutando (auto rx)

auto-listening = { $role }: escutando

auto-role-stopped = { $role } parou: { $sentence }

auto-rx-listening = auto rx: escutando

auto-cq-calling = auto cq: chamando CQ

auto-cont-running = auto cont: em execução

aircraft-search-missing-online = { $missing } de { $total } aeronaves sem dados online — abra o cartão delas

aircraft-search-shown-of = { $shown } de { $total }

aircraft-search-help =
    Pesquisa nas aeronaves já ouvidas: o endereço (ICAO), indicativo, matrícula, tipo, operador ou categoria. Várias palavras precisam coincidir todas. Limite uma palavra a um campo com icao: call: reg: type: op: — por exemplo  reg:PH op:KLM.
    Tipo e operador vêm da consulta online da estação, então só são encontrados para aeronaves cujo cartão já foi consultado. A pesquisa não pede nada.

aircraft-search-hint = pesquisar

aircraft-search-clear = Limpar a pesquisa

gridtracker-shade-squares-heard-live-decode = Sombreia o que é ouvido na lista de decodificação ao vivo, e não só o que está no log. Uma versão do mapa para quem escuta: o que está no ar agora, em ciano sob as marcas de trabalhados.

gridtracker-grid-ham = GRID/HAM

gridtracker-maidenhead-squares-what-a-ham-log = Quadrados Maidenhead — o que um log de radioamador traz. Nenhum indicativo de PX cai aqui: uma troca em 11 m traz um país, nunca um locator.

gridtracker-country-cb = PAÍS/PX

gridtracker-dxcc-countries-at-their-nominal = Países DXCC em seu centro nominal — o que um log de PX traz, já que uma troca em 11 m não tem locator. Nenhum indicativo de radioamador cai aqui também: são países, não quadrados.

colouredit-paste-a-colour-code-rrggbb-or-r-g = Cole um código de cor (#RRGGBB ou r, g, b) — ou Cmd/Ctrl+V sobre a amostra

cwkey-thread-failed = não foi possível iniciar a thread da chave CW: { $error }

cwkey-grab-failed = não foi possível tomar o manipulador com exclusividade — seus contatos também podem clicar em outras janelas (outro programa o está usando?)

ignore-muted-hover =
    Silenciadas nesta sessão: { $names }.

    Uma estação silenciada fica fora da lista de decodificação, não dispara os alertas, não é lida em voz alta e nunca é oferecida ao modo automático como alguém a responder. Nada é transmitido nem enviado em nome dela, e a lista é esquecida quando o sdroxide fecha — não há cópia guardada, então nada fica oculto depois de uma noite de escuta.

    Clique para reativar todas, ou pressione o botão − em qualquer uma de suas linhas.

js8-setup-hint = CONFIGURAÇÃO — indicativo, grupos, a resposta automática, o intervalo da baliza e a mensagem de status

logbook-contest-logger-a-session-dupes-a = Log de concurso — uma sessão, duplicados, pontuação ao vivo e Cabrillo

logbook-up-chip = ENV

net-wavelog-stopped = parado: { $error }

net-wavelog-download-stopped = Download do Wavelog interrompido: { $error }

net-wavelog-new-already-here = { $added } novos, { $matched } já estavam aqui

net-wavelog-fill-in-first = o endereço e a chave de API precisam ser preenchidos primeiro

net-wavelog-no-station-location = nenhum local de estação de onde baixar — teste a conexão primeiro

publicsdr-could-not-connect = não foi possível conectar nesta sessão

publicsdr-use-chip = USAR

weather-title = RELATÓRIOS METEOROLÓGICOS

weather-intro = Relatórios meteorológicos codificados de navios e estações em terra, em RTTY em 4583, 7646 e 10100,8 kHz. SHIP sai por volta das 02:00 e 07:35 UTC, SYNOP por volta das 00:35 e 12:10. Um relatório fica guardado aqui e não na lista de decodificação porque os números sozinhos não significam nada.

weather-nothing-decoded = Nada decodificado ainda. Só resta deixar o RTTY rodando em um dos horários acima — são as horas mais silenciosas da banda.

weather-head-call-day-hour = { $call } — dia { $day } às { $hour }:00 UTC

weather-head-day = às { $day }:00 UTC

weather-head-report = relatório

weather-not-reported = { $label }: não informado

weather-in-their-words = nas palavras deles: { $remark }

weather-ago-now = agora mesmo

weather-ago-minute = há um minuto

weather-ago-minutes = há { $m } minutos

weather-ago-hours =
    há { $h } { $h ->
            [one] hora
           *[other] horas
        }

weather-ago-days = há { $d } dias

sat-off-only-satellites-with = Desligado: apenas satélites com frequências cadastradas — os que você consegue trabalhar — e os que você colou por conta própria. Ligado: todos os satélites que a estação rastreia, a maioria dos quais ninguém consegue ouvir.

adsbmap-tune-hint = clique duplo para sintonizar · shift para o outro dial

adsbmap-airport-large = aeroporto grande · elevação { $elev }

adsbmap-airport-medium = aeroporto médio · elevação { $elev }

adsbmap-from-station = { $dist }  { $bear }° da sua estação

adsbmap-aircraft-within =
    { $n } { $n ->
        [one] aeronave
       *[other] aeronaves
    } em { $dist }

adsbmap-navaid-mw = { $khz } kHz (onda média)

adsbmap-click-to-close = clique no mapa para fechar

settings-ui-broadcast-stations-hint =
    Quais estações de radiodifusão a janela ESCUTAR e os rótulos do waterfall podem mostrar.

    { "**" }Neste trecho** é o que todas as telas faziam antes de isto ser uma escolha: meio trecho para cada lado do dial, que é o que um rótulo naquele waterfall deve cobrir. **A banda inteira** mostra tudo o que a programação tem com posição nela, que é o que quem navega por uma banda procura — ao custo de um mapa mais carregado.

settings-ui-waterfall-labels = Rótulos do waterfall

settings-ui-tuned-only = Só a sintonizada

settings-ui-tuned-only-hint = Nomeia a estação em que você está sintonizado, e nada mais — um nome, para aquilo que você está ouvindo, em vez de uma parede de nomes sobre a banda passante.

settings-ui-whole-band = A banda inteira

settings-ui-whole-band-hint = Nomeia todas as emissoras que a programação tem no ar agora, onde quer que estejam. É o que você quer ao navegar por uma banda em vez de ouvir uma só estação, e fica mais carregado por isso. Também na janela SPOTS, como BC ALL.

settings-ui-classic-ui = UI clássica

settings-ui-classic-ui-hint =
    Coloca todos os controles em uma única coluna à esquerda, com os grupos recolhíveis, do jeito que o SDR++ e o SDR++Brown organizam os seus, e dá todo o resto à imagem. Mesmos controles, mesmas configurações, dispostos como a maioria dos ouvintes de ondas curtas já está acostumada.

    Exige uma janela larga o bastante para manter a parte do waterfall: abaixo disso a coluna é omitida e a faixa é desenhada como de costume, e o painel é arrastado para mais largo ou mais estreito pela borda direita. Desligado, a faixa própria deste programa é usada exatamente como antes.

settings-ui-classic-ui-check = controles em uma coluna à esquerda

settings-ui-updates = Atualizações

settings-ui-updates-check = verificar nova versão ao iniciar

settings-ui-updates-hint = Consulta os GitHub Releases do fork pela última versão uma vez ao iniciar, e mostra um aviso se for mais nova que esta build. Só versões completas contam — a nightly é uma pré-versão — e desligado significa nenhuma chamada externa.

settings-ui-undocked = Desacoplado — janela própria

settings-ui-docked = Acoplado nesta janela

settings-ui-dock-all-hint = Traz todas as janelas desacopladas de volta para esta.

settings-ui-undock-all-hint = Coloca cada módulo em uma janela própria — a janela principal fica com o espectro e os decodificadores. Um módulo sem nada a mostrar, como o painel de operação em um modo de voz, não abre janela.

settings-ui-spectrum-line = Linha do espectro

settings-ui-spectrum-line-hint = A cor sob a linha do espectro e como ela é sombreada: Fixo é uma cor só em toda parte, Gradiente é cheio embaixo e se esvai em direção ao topo, Misto é cheio na metade inferior e se esvai pela metade superior. A linha em si continua ciano. O chip PREENCH. no popup SPEC liga o preenchimento, e sua linha escolhe o sombreamento.

settings-ui-fill = Preenchimento

settings-ui-fill-colour = cor

settings-ui-simple-ui = UI simples

settings-ui-simple-ui-hint =
    Oculta os extras avançados da faixa superior: a vista 3D, os skimmers, os botões de camadas de espectro/waterfall, o acompanhamento de diplomas, satélites, decodificação ISM e e-mail por rádio. Os controles que um operador de PX ou um ouvinte de ondas curtas procura — sintonia, modo, volume, squelch, largura de banda, o waterfall, memórias, varredura — permanecem todos.

    Nada é desligado, apenas ocultado: ligue isto de novo para trazer os chips de volta.

settings-ui-simple-ui-check = ocultar chips avançados

settings-ui-wake-lock = Wake lock

settings-ui-wake-lock-hint =
    Mantém a tela ligada enquanto esta página está à frente, para que um celular ou tablet deixado no waterfall não escureça nem bloqueie sozinho depois de um minuto sem toque. Recepção, áudio e decodificação não precisam disso — continuam de qualquer jeito — é para que a tela continue legível.

    Combina com o cadeado das faixas de celular e tablet: bloqueie a tela contra dedos acidentais e mantenha-a acesa.

    O navegador retoma o bloqueio sempre que a página é ocultada — outra aba, ou o botão de energia do próprio celular — e ele é pedido de novo assim que a página volta. Exige um navegador que o tenha e uma página segura (https, ou localhost); a linha ao lado da caixa informa quando não está disponível ou foi recusado. Gasta bateria, então fica desligado até você pedir. O programa instalado em um computador segue as configurações de energia do próprio sistema.

settings-ui-wake-lock-check = manter a tela ligada

settings-ui-sound-welcome = Boas-vindas do som

settings-ui-sound-welcome-hint =
    Quando a página web abre — uma sessão nova, ou um recarregamento com F5 — o navegador não toca som até você tocar na página. Essa é uma regra do navegador, e sem uma palavra na tela uma sessão nova parece não funcionar.

    Com isto ligado, uma mensagem no meio da tela avisa: 🔇 Toque, clique ou pressione uma tecla em qualquer lugar para iniciar o som. O primeiro clique, toque ou tecla — em qualquer lugar, com mouse, touchpad, dedo ou teclado — a fecha e inicia o som, e não faz mais nada: não sintoniza, não pressiona botão nem executa atalho. Ela só aparece quando a página abre, nunca mais durante a sessão, aconteça o que acontecer com o som depois.

    Desligue quando já souber o que fazer: o som continua esperando seu primeiro toque, e esse toque então funciona normalmente. Guardado com o seu perfil no servidor. Ligado por padrão.

settings-ui-sound-welcome-check = mensagem de boas-vindas do som

settings-ui-sound-welcome-browser-only = somente navegador — o programa instalado toca som sem um toque

settings-ui-profile-screen = A tela acompanha o seu perfil

settings-ui-profile-screen-hint =
    Quando você é um cliente remoto do `sdroxide --server`, o *visual* desta tela — tema, layout, waterfall e espectro, fontes, UI simples, as camadas do mapa, a vista própria do panadapter — fica guardado com o perfil com o qual você entrou, então volta em qualquer máquina em vez de começar do padrão.

    { "**" }Salvo no servidor, e salvo automaticamente enquanto você está logado** — não há nada para pressionar. Isso é proposital: não se pode confiar no armazenamento do navegador, porque o `persist()` se comporta de forma diferente em cada navegador, e uma sessão guardada ali não pode ser redefinida por quem não está naquele dispositivo. Um perfil no servidor é um arquivo que o operador da estação pode restaurar.

    Em um servidor **sem senha**, todo cliente é o mesmo perfil, então nada é gravado a menos que você pressione o botão abaixo — o tema de um operador não deve virar o do próximo. Nada que pertença à máquina é guardado, e um rádio local não é afetado.

settings-ui-save-to-profile = Salvar no perfil

settings-ui-back-to-profile = Voltar ao perfil

settings-ui-carry-bindings = Levar os atalhos de controle

settings-ui-carry-bindings-hint =
    Guarda também seus atalhos de teclado e mouse com o perfil no servidor, para que acompanhem o login entre navegadores e dispositivos — a mesma ideia da tela acima, para as teclas.

    { "**" }NÃO RECOMENDADO, e desligado por padrão.** Em uma estação que várias pessoas compartilham, o teclado é compartilhado com ela: um perfil que leva atalhos pode reatribuir o PTT, o Espaço ou as teclas de sintonia de outro operador no instante em que entra, e ligar isto pode ter outros efeitos que não são óbvios agora. Ligue somente onde o servidor é só seu e você o acessa de vários dispositivos seus.

    Exclusivo do fork: não é um recurso do upstream. As configurações de tela acima sempre viajam; esta é a única adesão opcional aqui.

settings-ui-carry-bindings-on = ligado — NÃO RECOMENDADO

settings-ui-carry-bindings-off = levá-los (não recomendado)

settings-ui-carry-bindings-title = Levar os atalhos de controle no servidor?

settings-ui-carry-bindings-body =
    Isto guarda seus atalhos de teclado e mouse com o seu perfil no servidor, para que acompanhem o seu login entre navegadores e dispositivos.

    Em uma estação usada por outras pessoas, o teclado é compartilhado com ela. Um perfil que leva atalhos pode reatribuir o PTT, o Espaço ou as teclas de sintonia de outro operador apenas ao entrar — e ativar isto pode ter outros efeitos que não são óbvios agora.

    É por isso que não é recomendado. Ative somente se o servidor for só seu.

settings-ui-carry-bindings-confirm = Entendo — levar mesmo assim

settings-ui-start-swl = Iniciar no modo SWL

settings-ui-start-swl-hint =
    Abre toda sessão com o modo SWL já ligado, de modo que a tela de um ouvinte seja o que o programa mostra ao iniciar — para todos os rádios, por ser uma preferência da tela e não de um rádio.

    A chave por rádio fica em Configurações → Rádio → Controles de transmissão, então uma estação com um rádio de escuta e um transceptor ajusta o de escuta lá e deixa isto desligado. Usar essa chave desliga esta semente na sessão. `--swl` sempre vence.

settings-ui-start-swl-check = iniciar com o modo SWL ligado

settings-ui-listen-hides-tx = A aba ESCUTAR oculta a transmissão

settings-ui-listen-hides-tx-hint =
    Escolher **ESCUTAR** no menu de banda/modo oculta todos os controles de transmissão nessa aba, e escolher **OPERAR** restaura o que a configuração própria deste rádio determinar.

    Isso não consegue ligar um transmissor. Um rádio configurado como somente receptor continua somente receptor, e o bloqueio de banda e o aviso de fora de banda não são tocados.

    O menu de banda lembra em qual aba foi deixado, então uma estação deixada em escuta volta em escuta — os controles de transmissão ficam ocultos também na próxima inicialização, e escolher OPERAR os traz de volta. Desligado por padrão porque muda o significado da aba; ele pergunta uma vez.

settings-ui-listen-hides-tx-check = ocultar TX na aba ESCUTAR

settings-ui-listen-hides-tx-title = A aba ESCUTAR vai ocultar os controles de transmissão

settings-ui-listen-hides-tx-body =
    Na aba ESCUTAR os controles de transmissão desaparecem; em OPERAR eles voltam exatamente como a configuração própria deste rádio os deixou.

    Isso não habilita nada. É uma vista da mesma estação, então o bloqueio de banda, o aviso de fora de banda e os limites de transmissão não são tocados.

    A aba é lembrada: uma estação deixada em ESCUTAR inicia em ESCUTAR, com os controles de transmissão ainda ocultos, até que OPERAR seja escolhido.

    Vale saber: uma estação deixada em ESCUTAR continua recebendo no que a aba sintonizou, e um controle oculto não é um controle desligado — a transmissão está desligada porque as configurações dizem isso, não porque o botão está fora de vista.

settings-ui-listen-hides-tx-confirm = Entendo — ocultar em ESCUTAR

settings-ui-swl-mode = Modo SWL

settings-ui-swl-mode-hint =
    Oculta todos os controles de transmissão deste rádio — o PTT, CHAMAR CQ, nível de TX, ENVIAR, BALIZA, tudo — e troca os extras de radioamador da faixa (spots, diplomas) pelos do ouvinte (PROGRAMAÇÃO e LOG SWL, o log de recepção). A mesma chave de Configurações → Rádio → Controles de transmissão, e por rádio: um dongle de escuta pode ficar neste modo enquanto o transceptor ao lado mantém seu transmissor.

    O hardware ainda pode transmitir; isto apenas oculta os controles. `--swl` força o modo ligado em todos os rádios durante a execução.

    Tem efeito ao Aplicar.

settings-ui-swl-mode-check = ocultar todos os controles de transmissão

settings-ui-report-picture = Imagem do relatório

settings-ui-report-picture-hint =
    A imagem que um relatório de recepção leva quando você o envia por e-mail a uma emissora — seu próprio cartão, uma fotografia, o que você quiser que a estação veja. Uma imagem para a estação, usada em todos os relatórios.

    Deixe sem definir e o relatório vai como texto simples, que é um relatório perfeitamente bom.

settings-ui-report-picture-none = nenhuma — o relatório vai como texto

settings-ui-choose = Escolher…

settings-ui-report-picture-pick = Escolha a imagem neste computador

settings-ui-report-note = Nota do relatório

settings-ui-report-note-hint = Sua nota fixa, preenchida em todo relatório de recepção — quem você é, com o que escuta, qualquer coisa que uma estação pergunta uma vez e não a cada carta. Texto simples em um arquivo simples: não é segredo.

settings-ui-report-note-hint-text = opcional — ex.: "Detalhes sob consulta"

sstv-keep-dial = MANTER DIAL

sstv-keep-dial-on-hint = Ligado: escolher SSTV deixa o dial onde você sintonizou, então o decodificador funciona em qualquer frequência. As frequências SSTV publicadas continuam no seletor ⇵. Clique para voltar a pousar na frequência SSTV da banda.

sstv-keep-dial-off-hint = Desligado: escolher SSTV move o dial para a frequência SSTV da banda quando você está a mais de 3 kHz de uma. Clique para manter o dial onde você sintonizou e decodificar SSTV em qualquer frequência. Sintonizar depois de escolher SSTV nunca é desfeito, de qualquer forma.

sstv-clear-rx = Limpar Rx

sstv-clear-rx-hint = Limpa a imagem exibida atualmente na vista AO VIVO. O decodificador continua funcionando — use Reiniciar RX para abandonar uma imagem em andamento e aguardar o próximo cabeçalho.

sstv-audio-present-no-header = áudio presente · ainda sem cabeçalho SSTV

sstv-editor = Editor…

sstv-editor-hint = Componha a própria imagem: texto móvel, qualquer fonte, uma faixa na horizontal ou vertical do quadro

sstv-save-image-as = Salvar imagem como…

sstv-save-copy-asks-file = Salva uma cópia neste computador — pergunta qual arquivo

sstv-picture-still-loading = A imagem ainda está carregando

sstv-share-hint-direct = Envie esta imagem a alguém — WhatsApp, Telegram, ou qualquer outro app deste dispositivo

sstv-share-hint-save = Salva esta imagem e abre uma mensagem para enviá-la

sstv-share-picture = Compartilhar imagem…

sstv-reupload-hint = Envia esta imagem de volta ao ar — carrega o slot de transmissão { $slot }

sstv-radio-cannot-transmit = Este rádio não pode transmitir

sstv-reupload = Reenviar

sstv-unidentified = não identificado

sstv-send-picture = Enviar esta imagem

sstv-qsl-to = Para

sstv-the-card = O cartão

sstv-the-card-hint = Isto vai em um cartão impresso ao lado da imagem. Deixe qualquer um em branco e ele simplesmente não é desenhado — um cartão nunca informa um relatório que ninguém deu.

sstv-date-utc = Data (UTC)

sstv-rst-sent = RST enviado

sstv-rst-received = RST recebido

sstv-qsl-note = Nota

sstv-add-address = Adicione um endereço para enviar isto.

sstv-saves-opens-mail-hint = Salva a imagem e abre seu cliente de e-mail — anexá-la é um clique lá.

sstv-browser-cannot-attach-hint = Uma aba do navegador não pode anexar um arquivo a uma mensagem, então isto salva a imagem e abre a mensagem; adicione o anexo manualmente.

sstv-save-open-mail = Salvar imagem e abrir e-mail

sstv-mail-not-opened = Imagem salva e mensagem copiada — cole-a no seu programa de e-mail. Nenhum aplicativo de e-mail pôde ser aberto.

sstv-qsl-saved-where-chosen = { $name } (salvo onde você escolheu)

sstv-qsl-downloaded = { $name } — baixado para a pasta de downloads deste navegador

sstv-picture-editor = Editor de imagem

sstv-slot-n = Slot { $n }

sstv-slot-layers-hint = As camadas próprias deste slot de transmissão

sstv-station-overlay = Sobreposição da estação

sstv-station-overlay-hint = Um conjunto de camadas que todo slot desenha por baixo das suas próprias — o indicativo, no canto que você preferir. Definido uma vez, composto com o texto próprio de cada slot em vez de substituí-lo.

sstv-draw-under-slot = desenhar sob este slot

sstv-draw-under-slot-hint = Desenha a sobreposição da estação sob as camadas próprias deste slot. Desligado para uma imagem que já traz a identificação — um mosaico retransmitido, geralmente — para que não seja carimbada duas vezes.

sstv-overlay-note = A sobreposição da estação: todo slot desenha estas camadas sob as suas próprias. Arraste uma linha na imagem para movê-la.

sstv-slot-text-note = O texto próprio deste slot, desenhado onde você o colocar. Arraste uma linha na imagem para movê-la; a faixa de uma linha se move com ela.

sstv-no-picture-in-slot = Nenhuma imagem neste slot — carregue uma abaixo

sstv-base = Base

sstv-base-hint = Sobre a imagem de qual slot estas camadas são desenhadas

sstv-compose-over-hint = Compor sobre a imagem deste slot

sstv-slot-empty = Este slot está vazio

sstv-picture-btn = Imagem…

sstv-load-picture-hint = Carregar uma imagem neste slot

sstv-use-received = Usar recebida

sstv-use-received-hint = Coloca a imagem que acabou de chegar sob estas camadas, para que uma resposta leve a imagem dele e o seu indicativo

sstv-received-now-base = a imagem recebida agora é a base

sstv-add-text = ＋ Texto

sstv-add-strip = ＋ Faixa

sstv-n-lines =
    { $n } { $n ->
            [one] linha
           *[other] linhas
        }

sstv-draw-this-line = Desenhar esta linha

sstv-strip-paren = (faixa)

sstv-draw-later-hint = Desenhar esta linha depois das de cima

sstv-draw-earlier-hint = Desenhar esta linha antes

sstv-copy = Copiar

sstv-face = Fonte

sstv-colour = Cor

sstv-line-text-hint = o que a linha diz

sstv-the-letters = As letras

sstv-composer-outline = contorno

sstv-the-edge = A borda ao redor delas

sstv-placement = Posição

sstv-down-frame = descendo pelo quadro

sstv-down-frame-hint = Gira a linha um quarto de volta, para que seja lida para baixo — um indicativo ao longo da lateral da imagem

sstv-strip = Faixa

sstv-behind = atrás

sstv-behind-hint = Uma faixa de cor atrás da linha, atravessando a imagem ou descendo por ela, centralizada na linha

sstv-strip-first-edge = A faixa, na primeira borda

sstv-thickness = Espessura

adsbpanel-paths = TRAJETOS

adsbpanel-paths-hint = Desenha o trajeto de cada voo como uma única linha — por onde passou, por onde está, para onde vai. Desligado, a trilha é desenhada como um rastro de pontos.

adsbpanel-nav = NAV

adsbpanel-nav-hint = Aeroportos e auxílios à navegação por rádio sob a imagem, das tabelas embutidas — aeroportos grandes sempre, os médios mais de perto, e os auxílios quando a vista está fechada o bastante para que uma estação seja um lugar. As balizas de onda média, que são as que você consegue ouvir, são desenhadas em cor própria.

adsbpanel-units = Unidades

adsbpanel-units-hint = Como altitude, velocidade, razão de subida e distância são escritas nas vistas de aeronaves. O decodificador e as exportações não são afetados.

adsbpanel-age = IDADE

touchlock-locked-hover = Tela bloqueada contra toques acidentais. Segure aqui por 3 segundos para desbloquear.

touchlock-unlocked-hover = Bloqueia a tela contra toques acidentais: frequência, banda, modo, filtros e o waterfall deixam de responder ao dedo. Recepção e áudio continuam. Segure por 3 segundos para desbloquear. Após um desbloqueio o cadeado descansa por 10 segundos antes que um toque o bloqueie de novo, para que o dedo que o desbloqueou não o bloqueie logo em seguida.

touchlock-hold = SEGURE { $s } s…

touchlock-unlocked = DESBLOQUEADO

touchlock-locked-hold = BLOQUEADO · segure 3 s

touchlock-unlocked-relock = DESBLOQUEADO · bloqueia em { $s } s

wake-held = tela mantida ligada

wake-asking = consultando o navegador…

wake-unsupported = indisponível aqui — exige um navegador que o tenha e uma página https

wake-refused = recusado pelo navegador (economia de energia?)

wake-native = somente navegador — o programa instalado segue as configurações de energia do sistema

tooltipcopy-copied = ✔ Copiado

chrome-hide-message-hover = Oculta esta mensagem de vez. Nada nas suas configurações muda, e o conselho está no manual.

chrome-close-message-hover = Fecha esta mensagem. Nada mais muda.

settings-ui-window-label = Janela { $module }

settings-ui-undock-hint =
    Desacoplado, este módulo vai para uma janela própria e a janela principal devolve o espaço ao que sobra — ponha o waterfall em um segundo monitor, ou os controles em um terceiro. Fechar a janela o acopla de novo.

    No Wayland o tamanho volta, mas o compositor decide onde uma janela fica: deixe-a flutuante e fixe-a em um monitor com uma regra de janela que corresponda ao app-id "{ $app_id }".

settings-ui-workspace-restore-out = Restaurar esta disposição — { $n } módulo(s) em janela(s) própria(s)

settings-ui-profile-in-use-named = em uso: o perfil { $name }

settings-ui-profile-in-use-default = em uso: o padrão desta estação

settings-ui-profile-not-in-use = ainda não em uso — nada guardado para este login

settings-ui-module-panadapter = Panadapter

settings-ui-module-controls = Controles

settings-ui-module-aux = AUX SP (segundo espectro)

settings-ui-module-band-menu = Seletor de banda e modo

settings-ui-broadcast-scope-in-view = Neste trecho

settings-ui-fill-fixed = Fixo

settings-ui-fill-gradient = Gradiente

settings-ui-fill-mixed = Misto

settings-ui-spot-kind-heard-me = ME OUVIRAM

settings-ui-wake-held = tela mantida ligada

settings-ui-wake-asking = consultando o navegador…

settings-ui-wake-unsupported = indisponível aqui — exige um navegador que o tenha e uma página https

settings-ui-wake-refused = recusado pelo navegador (economia de energia?)

settings-ui-wake-native = somente navegador — o programa instalado segue as configurações de energia do sistema

settings-ui-workspaces = Áreas de trabalho

settings-ui-workspaces-note = Uma disposição de janelas com nome — quais módulos estão em janelas próprias e onde essas janelas ficam. Salve a que você tem, ou restaure uma salva. Até dez.

settings-ui-workspaces-none = Nenhuma área de trabalho salva ainda.

settings-ui-workspace-restore-home = Restaurar esta disposição — todos os módulos na janela principal

settings-ui-workspace-forget = Esquecer esta área de trabalho

settings-ui-workspace-name = dê um nome a esta disposição

settings-ui-workspace-save = SALVAR ATUAL

settings-ui-workspace-full = Dez áreas de trabalho é o limite que o SDRuno guarda; apague uma primeiro

settings-ui-workspace-save-hint = Salva a disposição atual com este nome

settings-ui-workspace-name-first = Digite um nome primeiro

sstv-composer-fade = esmaecer

sstv-layouts = LAYOUTS

sstv-name-layout-hint = dê um nome a este layout

sstv-keep-layout-hint = Guardar este layout com esse nome

sstv-layout-renamed = renomeado para "{ $name }"

sstv-layout-saved-as = salvo como "{ $name }"

sstv-rename-cancelled = renomeação cancelada

sstv-no-layouts-yet = Nenhum layout salvo ainda — dê nome a um acima.

sstv-load-layout-hint = Carregar este layout no que você está editando

sstv-layout-loaded = "{ $name }" carregado

sstv-rename-layout-hint = Renomear "{ $name }"

sstv-layout-renaming = renomeando "{ $name }"

sstv-duplicate-layout-hint = Duplicar "{ $name }"

sstv-layout-copy-name = { $name } cópia

sstv-layout-duplicated = duplicado como "{ $name }"

sstv-forget-layout-hint = Esquecer o layout "{ $name }"

sstv-layout-forgot = "{ $name }" esquecido

sstv-mosaic-header = Mosaico de imagens recebidas

sstv-mosaic-nothing = Nada recebido ainda — um mosaico é montado a partir de imagens da galeria.

sstv-mosaic-picked = { $n } de { $max } escolhidas, dispostas nesta ordem — a ordem é a posição na grade. Cada imagem mantém sua forma; os vazios são pretos.

sstv-mosaic-earlier-hint = Mover esta imagem uma célula para trás

sstv-mosaic-later-hint = Mover esta imagem uma célula para frente

sstv-mosaic-max = { $n } é o máximo que um mosaico comporta

sstv-mosaic-compose = Compor neste slot

sstv-mosaic-compose-hint = Monta o mosaico e o carrega neste slot como sua imagem; os originais recebidos não são tocados

sstv-mosaic-composed = { $n } imagens compostas no slot { $slot }

sstv-clear-picks = Limpar escolhas

sstv-composed-saved = salvo

sstv-send = Enviar

sstv-send-now-hint = Transmite esta composição agora

sstv-composed-sending = enviando

sstv-forget = Esquecer

sstv-forget-hint = Esvazia o documento desta predefinição — a faixa de banner e a legenda simples voltam a controlar a imagem

sstv-forget-done = limpo — o banner e a legenda voltaram

sstv-copy-to-slot = Copiar para o slot

sstv-copy-to-slot-hint = Dá a outro slot esta imagem e este texto, para que possa ser enviado de lá

sstv-copied-into-slot = copiado para o slot { $n }

sstv-word-strip = faixa

sstv-word-text = texto

sstv-text-size = Tamanho do texto

sstv-text-size-hint = O texto do banner como porcentagem do tamanho da própria faixa. 100% é o que sempre foi; um QSO precisa do indicativo legível após uma decodificação ruidosa, e isto escala as letras sem deixar a faixa mais alta.

sstv-placement-hint = Em qual borda a faixa é desenhada. A sobreposição de mensagem começa na extremidade oposta, então um banner embaixo deixa o topo da imagem para o texto.

sstv-top = Topo

sstv-bottom = Base

topbar-ptt-hold-hint = Segure para transmitir — um clique o trava ligado, e o próximo toque o solta


topbar-above-the-dial = acima

topbar-below-the-dial = abaixo

topbar-step-chip-hint = Quanto um toque move o dial. Toque para pegar o próximo passo: 10 Hz, 100 Hz, 500 Hz, 1, 2,5, 5, 6,25, 9, 10, 12,5 e 25 kHz. Desligue a linha em Configurações › UI.

topbar-mem-chip-hint = Canais de memória — recupera uma frequência guardada

topbar-disp-menu-hint = O panadapter — suas duas camadas, suas velocidades e detalhe, peak hold — mais os níveis do waterfall, as opções de vista e os skimmers

topbar-phone-menu-hint = Tudo o que a faixa do celular não mostra: bandas, modos, o receptor e o transmissor, as janelas de decodificação, configurações e o manual

topbar-group-band = Banda

topbar-group-system = Sistema

topbar-caption-diversity = Diversity

topbar-group-decode-windows = Janelas de decodificação

topbar-group-extras = Extras

topbar-grid-tracker-hint = Grid tracker — quadrados Maidenhead trabalhados em um mapa

topbar-settings-chip-hint = Configurações — ganhos do dispositivo, antenas, dispositivos de áudio

topbar-windows-chip-hint =
    ▣  Esquerdo: traz todas as janelas de módulos para casa
    Direito: a lista de janelas de módulos

topbar-manual-chip-hint = Manual do usuário (F1)

topbar-module-windows-title = Janelas de módulos

topbar-wname-panadapter = SP1 — o espectro

topbar-wname-aux = SP2 (AUX) — um segundo espectro

topbar-wname-panel = Painel de operação

topbar-wname-controls = Controle RX

topbar-wname-band-menu = Banda e modo

topbar-wname-rotor = Rotador de antena

topbar-wstate-put-back = Colocar { $name } de volta nesta janela

topbar-wstate-floating-panel-only = Flutuar é um estado próprio do painel de operação — um espectro ou um console tem um só lugar nesta janela

topbar-wstate-float = Tirar { $name } do painel e deixá-lo flutuando sobre esta janela

topbar-wstate-no-panel = Sem painel de operação em { $mode } — isto vale em um modo que tenha um

topbar-wstate-own-window = Dar a { $name } uma janela própria, em qualquer monitor

topbar-wstate-here-now = { $name } está aqui agora

topbar-dock-all = ACOPLAR TUDO

topbar-dock-all-hint = Traz todas as janelas de módulos de volta a esta janela

topbar-undock-all = DESACOPLAR TUDO

topbar-undock-all-hint = Coloca cada módulo em uma janela própria — esta janela fica com o espectro e os decodificadores

topbar-rx-tx-freq = RX { $rx }   TX { $tx } MHz

topbar-band-dock-hint = ACOPLAR — mantém o seletor de banda e modo aberto ao lado do waterfall em vez de fechar este popup a cada vez

topbar-band-window-hint = JANELA — abre o seletor de banda e modo em uma janela própria, tão larga quanto você quiser, em qualquer monitor ao lado do waterfall

topbar-band-return-popup-hint = ACOPLAR — devolve o seletor ao popup da barra superior, onde ele fecha a cada escolha

topbar-band-window2-hint = JANELA — abre o seletor em uma janela própria, tão larga quanto você quiser, em qualquer monitor

topbar-squelch-held-hint = O squelch está mantido aberto. Clique para restaurar o limiar que você tinha.

topbar-squelch-hold-hint = Clique para manter o squelch aberto — para ouvir se há algo ali — e clique de novo para restaurá-lo exatamente como estava.

topbar-changed-tone = tom

topbar-changed-loudness = LOUDNESS

topbar-nr-chip-hint =
    Redução de ruído: { $strength } — { $engine }

    { $advice }

    Clique para alternar a intensidade: Baixa / Média / Alta / Desligada. O motor está na linha abaixo desta.

topbar-nr-off-narrow-hint = A redução de ruído está desligada. Cinco redutores são oferecidos na linha abaixo desta — RNNoise, DeepFilterNet3, spectral-bleach, NR2 e o espectral original — e clicar aqui a liga em uma intensidade. Comece com o RNNoise em voz.

topbar-eq-tone-hover = Tom: graves { $bass } dB, médios { $mid } dB, agudos { $treble } dB

topbar-eq-closed-hover = Tom — um equalizador de três bandas no áudio de recepção. Clique para abrir

topbar-eq-loudness-on = LOUDNESS ligado

topbar-stereo-yields-hint = O estéreo WFM fica desligado enquanto a redução de ruído ou o auto-notch estão ativos — eles borrariam os dois canais. Desligue-os para ouvir estéreo

topbar-stereo-playback-hint = Estéreo: a gravação está sendo reproduzida nos dois ouvidos, como gravada. Clique para forçar mono

topbar-stereo-mono-recording-hint = Esta gravação é mono — não há segundo canal para reproduzir

topbar-preset-edges = { $lo } … { $hi } Hz

topbar-caption-tone = Tom

topbar-eq-on = ligado

topbar-eq-bass = Graves

topbar-eq-mid = Médios

topbar-eq-treble = Agudos

topbar-eq-remembered = Lembrado para { $mode } — cada modo guarda o seu.

topbar-loudness-hint = Reforça os graves (e um pouco dos agudos) conforme você abaixa o volume, para que um alto-falante baixo ainda soe cheio. Funciona com ou sem o tom; não faz nada em volume máximo e nunca toca mais alto do que o volume máximo tocaria. Desligado por padrão, e lembrado para o modo em que você está.

topbar-caption-record-audio = Gravar áudio

topbar-stop-recording-after-min = Parar a gravação após { $minutes } minutos

topbar-caption-play-back = Reproduzir

topbar-recordings-chip = ▶ GRAVAÇÕES…

topbar-recordings-hint = Reproduz uma gravação de volta nos decodificadores — uma imagem SSTV perdida, um sinal RTTY ou CW fraco, um slot de FT8 — com o waterfall e o log de decodificação, como se estivesse no ar. O rádio fica em espera enquanto ela toca.

topbar-ant-label = ANT

topbar-pwr-label = PWR

topbar-drive-less = menos

topbar-drive-more = mais

topbar-fill-hint = Colore a banda sob a linha do espectro, cheia embaixo e esvaindo conforme sobe até a linha, para que um sinal se destaque pela massa e também pelo contorno. A cor é escolhida em Configurações → UI → Linha do espectro. Não é desenhado na superfície 3D.

topbar-group-spectrum = Espectro

topbar-group-waterfall = Waterfall

topbar-freeze-on-tx-hint = Para a rolagem do waterfall durante a sua própria transmissão e a retoma de onde parou — assim a imagem é um histórico recebido contínuo, e não um bloco de linhas mostrando a sua portadora (ou, em um rádio que silencia o receptor enquanto está em transmissão, linhas de ruído que nunca existiram). A linha do espectro não é afetada. Útil ao acompanhar um modo em que você transmite com frequência, para que a vez da outra estação não seja empurrada para fora da tela pela sua.

topbar-fft-chip-hint = Níveis e contraste do waterfall, tamanho da FFT e a direção da rolagem

topbar-caption-waterfall = Waterfall

topbar-flatten-hint = Nivela o piso de ruído ao longo da banda: um trecho mais silencioso que os vizinhos é elevado em direção ao piso deles, para que um sinal fraco apareça em uma banda movimentada. Só eleva, nunca abaixa — uma portadora nunca é empurrada para baixo — e nunca mais de 20 dB. O traço do espectro acima fica exatamente como medido.

topbar-log-swl-hover = Log de recepção — estações ouvidas, com SINPO/SIO

topbar-programs-hint = Programação de radiodifusão — o que está no ar, quando e onde

topbar-listen-hint = O que há neste trecho, em um mapa — escolha uma estação, sintonize, anote o que ouviu e registre

topbar-spots-hint = Spots ao vivo — PSK Reporter (e, fora do modo SWL, o DX cluster, POTA e SOTA)

topbar-sigid-hint = O que há neste dial? Um guia de sinais por frequência, modo e largura de banda

topbar-extras-hint = EXTRAS — DAB e os outros receptores, em uma janela que você pode destacar

topbar-fill-style-caption = linha

topbar-fill-style-hint = Como a cor sob a linha do espectro é sombreada. A cor em si é escolhida em Configurações → UI → Linha do espectro.

topbar-fill-fixed = FIXO

topbar-fill-fixed-hint = Uma cor, a mesma em todo lugar sob a linha

topbar-fill-gradient = GRADIENTE

topbar-fill-gradient-hint = Cheio na base do espectro, esvaindo em direção ao topo

topbar-fill-mixed = MISTO

topbar-fill-mixed-hint = Cheio na metade inferior, esvaindo pela metade superior

topbar-fill-switch-on = Ligue o PREENCH. para definir isto

topbar-nothing = nada

topbar-olivia-unconfirmed = OLIVIA — a recepção funciona, a transmissão não foi testada com outros programas. Este decodificador lê outras estações (confirmado no ar), então você consegue copiar um QSO em Olivia. Nossa própria transmissão agora fecha o ciclo com nosso próprio receptor, mas não foi copiada pelo fldigi nem pelo MultiPSK e uma convenção de sinal difere do código-fonte do fldigi — então verifique a primeira transmissão em vez de esperar uma resposta.

topbar-dab-bandwidth = DAB / DAB+, comprovado no ar. Precisa de 2,5 Msps de largura de banda no front end antes de haver qualquer áudio: nos 1,536 MHz que um ensemble ocupa o receptor está no piso do seu ADC e perde amostras, e uma decodificação DAB precisa de um fluxo contínuo, então o ensemble nunca trava e você não ouve nada. Ajuste primeiro a taxa de amostragem do rádio para 2,500 Msps.

topbar-stanag-unproven = STANAG 4285 — somente recepção. A identificação está comprovada no ar: um 4285 trava, seu quadro de 106,67 ms é medido e seus blocos de referência colapsam, então o painel consegue dizer o que há na frequência e em que taxa. Ler seu *texto* não está comprovado no ar: o código e o entrelaçador do Annex E decodificam exatamente um sinal gerado, mas nenhuma estação codificada foi copiada ainda. Um texto codificado que apareça é real; nenhum texto significa que a estação é não codificada ou leva tráfego criptografado, que é o caso habitual.

topbar-cis11-unproven = CIS-11 (TORG-11) — somente recepção. O teleimpressor meteorológico russo: 2-FSK a 100 baud, cerca de 500 Hz de shift, e um caractere de 11 bits com um bit de paridade ímpar. O que está comprovado foi medido em gravações reais: o receptor encontra seus próprios tons e lê uma transmissão ociosa com 121 de 123 caracteres válidos na paridade. O que não está resolvido é como os dez bits de dados se dividem em caracteres, então o painel mostra os bits além da leitura ITA-2 — e o próprio tráfego é criptografado, que é o caso habitual.

topbar-thor-unproven = THOR — o lado de recepção ainda não lê transmissões do fldigi. O decodificador funciona contra seu próprio modulador, mas nenhuma gravação off-air ou de referência foi copiada, então trate o que aparecer como provisório.

topbar-mode-not-on-band = { $mode } não é usado em { $band } — escolha uma banda a que ele pertença

topbar-radio-classic-hint = RADIO CLASSIC — uma face de rádio clássica para escuta de radiodifusão. Ligá-lo dá a ele o receptor: os outros modos e suas janelas são fechados

topbar-ats-bands-caption = Bandas que o receptor oferece

topbar-ats-band-hover = { $name } — { $mhz } MHz, { $mode } no padrão. Leva o receptor a esta banda.

topbar-caption-mode = Modo

topbar-caption-digital = Digital

topbar-ats-receive-only = Somente recepção — o Si4732 demodula em hardware e não há nada para transmitir.

topbar-band-forecast-derived =
    { $band }: ≈{ $verdict } ({ $time ->
            [day] dia
           *[night] noite
        }) — o veredito do grupo { $group } publicado, o substituto mais próximo; o HAMQSL.com não classifica nada para { $band } em si. Uma previsão, não uma medição do seu próprio caminho.

topbar-keypad-readout-khz = { $digits } kHz

topbar-caption-band-keypad = Teclado de bandas

topbar-keypad-bands = Bandas

topbar-keypad-bands-hint = As teclas nomeiam bandas: pressione uma para ir até ela, permanecendo no modo em que você está. Voltar de MHz apaga o que foi digitado.

topbar-keypad-mhz-hint = Digite uma frequência nas teclas, em quilohertz: 14074 é 14,074 MHz. ENTER a envia, LIMPAR apaga a entrada.

topbar-keypad-key-stay = Permanecer em { $mode } e sintonizar { $mhz } MHz — onde se trabalha em { $band }.

topbar-keypad-key-go = Ir para { $band }.

topbar-keypad-clear = Limpar

topbar-keypad-clear-typing-hint = Apaga a frequência que você digitou. As teclas continuam em MHz.

topbar-keypad-clear-bands-hint = Nada foi digitado — as teclas estão em bandas, onde um toque vai a uma banda.

topbar-keypad-enter = Enter

topbar-keypad-enter-empty-hint = Nada digitado ainda. As teclas estão em MHz — pressione-as primeiro.

topbar-keypad-enter-ok-hint = Sintonizar { $mhz } MHz e voltar às teclas de banda.

topbar-keypad-enter-range-hint = { $mhz } MHz está fora do que este rádio recebe ({ $ranges } MHz).

topbar-nothing-published = nada publicado

topbar-tab-listen = ESCUTAR

topbar-tab-operate = OPERAR

topbar-listen-tab-note = Esta aba também pode ocultar os controles de transmissão: ligue "ocultar TX na aba ESCUTAR" em Configurações → UI

topbar-caption-show-bands = Mostrar bandas

topbar-band-filter-hint = Mostra apenas as bandas { $filter } na lista abaixo. Clique de novo para mostrar todas as bandas.

topbar-show-every-band = mostrar todas as bandas

topbar-all-chip = TODAS

topbar-all-chip-hint = Cobertura geral: libera a banda e deixa o dial ir a qualquer lugar. Não é o mesmo que o filtro de banda, que apenas reduz esta lista.

topbar-caption-sw-metre-bands = Bandas de ondas curtas em metros

topbar-metre-band-hint = Sintoniza o meio da radiodifusão de { $name }

topbar-caption-wideband = Banda larga

topbar-caption-cb = 11M/PX

topbar-caption-experimental = Experimental (não comprovado)

topbar-caption-receive-modes = Modos de recepção

topbar-wav-hint = O mesmo áudio em PCM de 16 bits sem compressão a 48 kHz — cerca de 11 MB por minuto em estéreo. Para um editor ou um decodificador seu.

topbar-mode-offset-hint-side = Este modo trabalha o sinal { $side } o dial. O número é o do dial, porque é o que você sintoniza; o sinal está em { $mhz } MHz.

levelslider-hover =
    Nível do waterfall
    piso { $floor } dB · teto { $ceil } dB

    Arraste ou role para clarear ou escurecer; shift para o contraste.
    Movê-lo desliga o ajuste automático (FIT).

smeter-audio-level-hover =
    Nível de áudio: { $dbfs } dBFS{ $over }.

    Este rádio **não tem S-meter próprio para informar**, então este é o nível do áudio que ele envia — medido aqui, com o AGC e o squelch do próprio rádio já aplicados. Ele acompanha o áudio, não o RF: ruído na placa de som, um mouse sem fio perto do rádio ou o AGC bombeando vão movê-lo mesmo sem o sinal ter mudado.

smeter-audio-over = (S9 + { $over } dB na escala de áudio)

spectrum-offset-left = ◀ tons { $hz } Hz

spectrum-offset-right = { $hz } Hz tons ▶

worldmap-no-snr = sem SNR

solar-tune-the-radio-to-this-downlink = Sintonizar o rádio neste downlink

qo100-width-label = largura

qo100-park-label = estacionar

qo100-led-carrier = portadora

qo100-led-sync = sinc

scanner-stopped = parado

ism-band-label = banda

amp-none = nenhum

colormap-classic = Clássico

colormap-viridis = Viridis

colormap-gray = Cinza

colormap-icom = Icom

colormap-neon = Neon

colormap-synthwave = Synthwave

colormap-matrix = Matrix

colormap-tron = Tron

colormap-amber = Âmbar

colormap-rainbow = Arco-íris

colormap-blue = Azul

colormap-calmer = Calmer

colormap-compline = Compline

colormap-lauds = Lauds

settings-cb-wide-calls-hint =
    Aceita o formato mais amplo de indicativo de 11 m { $wide } — três letras no meio, até quatro dígitos depois — junto com o { $narrow } do WSJT-CB. Experimental: o próprio WSJT-CB rejeita o formato mais amplo, então um indicativo que o exija pode não ser entendido por uma estação WSJT-CB; ele amplia mais o que esta estação ouve do que o que ela consegue enviar com segurança. Desligado mantém a gramática exata do WSJT-CB.

    Esta chave altera o *formato*, não o *comprimento*: um indicativo de 11 m continua com 11 caracteres, incluindo qualquer sufixo /P, /MM ou /QRP, então 19DCG3733/P cabe e 19TST1001/QRP não.

settings-idle-stop-disable = DESATIVAR

settings-xvtr-tx-off-hint =
    Desligado durante a conversão: somente recepção — a linha é um conversor, não um transverter, e nada é transmitido nesta banda.

    Pelo mesmo conversor: funciona nos dois sentidos, e a transmissão usa o mesmo deslocamento.

settings-xvtr-drive-hint =
    Um teto para o drive de transmissão enquanto este transverter está selecionado, como porcentagem do total. A entrada de F.I. de um transverter aceita miliwatts e o drive certo para as bandas do próprio rádio vai destruí-la, então este é o campo mais importante da linha. 100% é sem limite.

    Sua configuração de Drive é mantida abaixo deste valor em vez de movida, então o número que você usa em HF volta quando o dial sai da banda.

settings-brown-build-note = A build Brown do sdroxide: o programa do upstream, mais a banda do cidadão de 11 m, escuta de ondas curtas e decodificação. Roda como uma estação de radioamador comum com essas bandas e ferramentas acrescentadas.

settings-gain-by-band = Lembrar o ganho do front end por banda

settings-gain-by-band-hint =
    Guarda um ganho de receptor separado para cada banda e o restaura quando o dial entra nessa banda. Ajuste seu ganho de HF uma vez e o de 11 m ou VHF uma vez; o rádio acompanha a banda.

    Desligado por padrão. Enquanto desligado, um único ganho vale em toda parte, exatamente como antes. Ligá-lo registra a banda em que você está agora, para que nenhuma banda fique sem um; uma banda que você nunca ajustou mantém o ganho atual.

settings-callsign-grid-note = Seu indicativo e grid, compartilhados entre FT8/FT4/FT2, cabeçalhos de imagem SSTV, os modos digitais de PX e o logbook. Esta é a identidade que **transmite** — um indicativo de PX se você trabalha 11 m, um indicativo de radioamador caso contrário. Para o que assina um **relatório de recepção**, veja a caixa **Relatar como (SWL)** na aba Spots. Também editável pelo diálogo de configuração de FT8 / SSTV.

settings-cb-grammar = Gramática de indicativos de 11 m / WSJT-CB

settings-cb-wide-calls = INDICATIVOS DE PX LARGOS — EXPERIMENTAL

settings-cb-wide-calls-note = Ligue se a sua comunidade de 11 m usa grupos de três letras ou números de unidade de quatro dígitos. Vale para a decodificação e a transmissão dos modos digitais de PX. De qualquer forma, um indicativo de 11 m não pode passar de **11 caracteres**, sufixo incluído — 19DC3733/P cabe, 19TST1001/QRP não.

settings-cb-plan-hint = Em qual plano de canais o dial lê: os planos da banda do cidadão de 11 m, e os de PMR446 em 446 MHz. Mudam apenas os números dos canais e o canal em que a banda abre — os limites da banda ficam onde estão, então trocar nunca muda o que recebe ou transmite. O número do canal é desenhado na linha de sintonia sempre que o dial está em um dos canais do plano.

settings-cb-allow-tx = Permitir transmissão em 11 m (PX)

settings-cb-allow-tx-hint = O bloqueio de bandas de radioamador recusa a transmissão em toda banda que não seja uma alocação de radioamador, e 11 m é uma: é um serviço de rádio separado, com suas próprias regras e seu próprio equipamento homologado. Ligar isto permite transmitir ali — e em nenhum outro lugar; as bandas de radiodifusão continuam somente de recepção. Você será solicitado a confirmar na primeira vez.

settings-cb-licence-note = O PX dispensa licença em muitos países, mas não em todos, e os canais, modos e potência permitidos variam. O uso da banda está sujeito às regras do país em que você está.

settings-radio-callsign-hint =
    O indicativo com o qual este rádio transmite, registra e envia spots. Deixe vazio para usar o indicativo da estação da aba Geral — o padrão, e o certo para uma estação com uma única identidade.

    Defina em um rádio que seja de outra pessoa: um indicativo de PX no rádio de 11 m, um indicativo de radioamador no rádio de HF, cada um mantendo o seu próprio. O indicativo da aba Geral continua como alternativa para todo rádio que não o substitua.

    Os *relatórios* de recepção ainda podem ser assinados com o número SWL da aba Spots, que isto não altera.

    Tem efeito ao Aplicar.

settings-radio-swap-iq-own = Esta interface tem sua própria inversão de I/Q na seção mais abaixo desta página — use aquela.

settings-radio-swap-iq-hint =
    Inverte o I/Q deste receptor antes da demodulação. Use quando o I e o Q de um receptor estão trocados e o SSB sai na banda lateral errada (LSB ouvido como USB), com o espectro espelhado em torno do dial.

    Uma propriedade do hardware, não de quem está logado.

    Tem efeito ao Aplicar.

settings-radio-transmit-controls = Controles de transmissão

settings-radio-hide-tx-all = ocultar todos (modo SWL)

settings-radio-hide-tx-hint =
    Oculta todos os controles de transmissão *deste* rádio — o PTT, CHAMAR CQ, nível de TX, ENVIAR, BALIZA, tudo — e troca os extras de radioamador da faixa (spots, diplomas) pelos do ouvinte (PROGRAMAÇÃO e LOG SWL, o log de recepção). Por rádio: um dongle de escuta na antena de fio longo pode ficar neste modo enquanto o transceptor ao lado mantém seu transmissor.

    O hardware ainda pode transmitir; isto apenas oculta os controles. `--swl` força o modo ligado em todos os rádios durante a execução.

    Tem efeito ao Aplicar.

settings-radio-auto-mode = Modo automático

settings-radio-auto-stop-label = parar sem atividade

settings-radio-auto-stop-hint-long =
    Por quanto tempo o sequenciador FT8/FT4/FT2 sem supervisão pode rodar sem ação do operador antes de se desarmar. Por rádio: um pode ser deixado rodando mais tempo que outro.

    Em um PlutoSDR vai até seis horas, e DESATIVAR desliga a parada por completo: o modo automático então roda até você parar. O watchdog de transmissão e o bloqueio de banda continuam valendo.

    Tem efeito ao Aplicar.

settings-radio-auto-stop-hint =
    Por quanto tempo o sequenciador FT8/FT4/FT2 sem supervisão pode rodar sem ação do operador antes de se desarmar. Por rádio: um pode ser deixado rodando mais tempo que outro.

    Limitado a 45 minutos de propósito — o modo automático é para uma pausa rápida, não para deixar uma estação trabalhando um concurso sem ninguém por trás.

    Tem efeito ao Aplicar.

settings-radio-smeter-calib = Calibração do S-Meter

settings-tx-eq = EQ de transmissão

settings-tx-eq-hint = Um EQ paramétrico de 3 bandas no áudio do microfone, antes do modulador. Somente modos de voz (SSB/AM/FM); modos digitais e CW levam áudio sintetizado ou manipulado que nunca passa por ele. Desligado por padrão e plano quando ligado, então ativá-lo não muda nada no ar até que uma banda abaixo seja de fato movida. Aplica imediatamente.

settings-tx-eq-low-hint = Corta/reforça ronco e ruído de manuseio.

settings-tx-eq-mid = Pico médio

settings-tx-eq-mid-hint = Presença: uma elevação ou queda estreita em algum ponto da faixa de voz.

settings-tx-eq-high-hint = Brilho/de-ess.

settings-cb-call-too-long = ⚠ Longo demais para 11 m — { $len } caracteres, e o FT8 comporta 11 para um indicativo de PX *incluindo* qualquer sufixo /P, /MM ou /QRP. Este indicativo de ativação não pode ser enviado. 19DC3733/P (10) cabe; 19TST1001/QRP (13) não.

settings-cb-plan-note = 11 m (PX) não é a mesma banda em todo lugar. Isto escolhe em quais canais de qual país o dial de 11 m lê: { $plan }. Apenas os canais e o canal em que a banda abre — os limites da banda ficam amplos, então trocar de plano nunca muda o que recebe ou transmite. Vale para todo rádio desta estação.

settings-cw-enter-mode = Entre em CW (ou em qualquer modo digital) uma vez para carregar os valores salvos.

settings-cw-where-from = De onde:

settings-cw-straight-key-hint = O atalho `CW straight key` (Espaço por padrão) é lido enquanto a CHAVE do painel CW está ligada. Um contato só, portanto é uma chave reta — o app decodifica a temporização do próprio operador.

settings-cw-speed = Velocidade:

settings-cw-speed-note = a duração do ponto e do traço do keyer

settings-cw-key-tx = MANIPULAR O TRANSMISSOR

settings-cw-key-tx-hint = Desligado: a chave só aciona o treinador local. Ligado: ela aciona o rádio pelo caminho normal de chave manual, com o bloqueio de banda e o watchdog ao redor.

settings-cw-key-self-note = Para um rádio que manipula a si mesmo, defina Configurações → Rádio → Manipulação CW como `Placa de som (MCW)`: a manipulação manual é recusada na rota do keyer do rádio por projeto.

settings-detachable-windows = Janelas destacáveis

settings-detachable-windows-note = Puxe um painel para uma janela própria do sistema. Fechar a janela o acopla de novo.

settings-tool-windows = Janelas de ferramentas

settings-tool-windows-note = Painéis convertidos em janelas flutuantes. O estado aberto delas é lembrado.

settings-tool-window-scanner = SCANNER

settings-tool-window-memories = MEMÓRIAS

settings-tool-window-playback = REPRODUÇÃO

settings-tool-window-open = Abrir

settings-tool-window-closed = Fechada

settings-detachable-native-only = Janelas destacáveis existem apenas no app nativo.

settings-psk-reporter-band-hint = O feed de atividade da banda: estações sendo ouvidas na banda atual, desenhadas como pontos coloridos. Também habilita a consulta "quem me ouviu" por trás do chip ME OUVIRAM do mapa — atualizada a cada hora também.

settings-psk-upload-hint = Relata o que esta estação ouve ao pskreporter.info, para que apareça lá como receptora. Usa o indicativo e o grid da aba Geral, ou o número SWL abaixo quando definido.

settings-swl-report-as = Relatar como (SWL)

settings-swl-report-as-hint = A identidade própria do ouvinte para relatórios de recepção — um número SWL registrado, um número de clube, um nome. Quando definida, assina o que esta estação *ouve* (envios ao PSK Reporter e ao WSPRnet, e o relatório de recepção que o log SWL copia) em vez do indicativo. Deixe vazia para relatar como o indicativo. Nunca é transmitida, nunca aciona um rádio e nunca nomeia um QSO ou spotter, então um ouvinte sem indicativo também pode relatar.

settings-wsjtcb-spot-server = Servidor de spots WSJT-CB

settings-wsjtcb-report = Relatar minhas decodificações de 11 m ao servidor de spots WSJT-CB

settings-wsjtcb-report-hint = Envia cada spot decodificado na banda de 11 m ao serviço de spots da comunidade PX (xzgroup.net) — o mesmo servidor em que o cliente do próprio WSJT-CB pode publicar. Desligado por padrão: é um serviço de terceiros e nada é enviado até você pedir. Seu indicativo é o spotter; o PSK Reporter é a chave separada acima.

settings-eqsl-qth-hint = Em qual perfil de QTH um envio é arquivado — necessário apenas quando a conta tem mais de um, e deixado em branco caso contrário.

settings-log11dx-api-url = URL da API

settings-log11dx-api-token = Token da API

settings-log11dx-note = Ambos vêm da página de perfil do seu LOG11DX. A ponte própria do WSJT-X não é necessária: o sdroxide registra o QSO e o envia direto para lá. O site mascara o token, então copie-o quando o gerar — ele é guardado em net.json como as outras credenciais.

settings-pan-attached-note =
    Seu espectro, seu waterfall e tudo o que é lido deles estão na aba daquele rádio, e é por isso que ele não tem os seus. As configurações de interface acima ainda são deste rádio e ainda valem — pressione Aplicar / reconectar depois de alterá-las e os dois rádios voltam juntos.

    Para usá-lo sozinho de novo, limpe a caixa Receptor do panadapter na página daquele rádio.

settings-pan-source-hint =
    Qual rádio recebe por este. Ele sai da faixa de abas enquanto está emprestado — seu front end pertence ao motor deste rádio — e volta imediatamente quando isto é definido como Nenhum.

    Tem efeito ao Aplicar.

settings-pan-offset-hint =
    A que distância acima do dial deste rádio o receptor está, em hertz. 0 para um receptor na mesma antena; a frequência intermediária para uma derivação de F.I. — 9000000 para uma F.I. de 9 MHz, 70455000 para uma de 70,455 MHz.

    Este não é o campo Conversor acima, que ressintoniza o rádio, nem o deslocamento do centro de I/Q da aba CAT, que trata da placa de som deste rádio. Nada digitado aqui é enviado a nenhum dos rádios.

    Tem efeito ao Aplicar.

settings-cw-device = Dispositivo:

settings-cw-device-none = (nenhum encontrado)

settings-cw-device-auto = auto (o primeiro chamado "key")

settings-cw-reverse-hint = Troca ponto e traço, para a chave do próprio manipulador.

settings-cw-key-type = Tipo de chave:

settings-cw-key-straight = RETA

settings-cw-key-iambic-a = IAMBIC A

settings-cw-key-iambic-b = IAMBIC B

settings-cw-usb-note = Uma caixa de keyer que informa seus dois contatos (muitas vezes como botões de mouse) funciona; uma que faz o próprio iambic e envia toques já prontos não. Enquanto a chave está armada o dispositivo é tomado com exclusividade, então seus contatos não podem clicar em outras janelas.

settings-cw-usb-linux = Um manipulador USB exige um desktop Linux.

settings-smeter-zero = 0 (como medido)

settings-smeter-cal-hint =
    Corrige o S-meter deste rádio quando ele lê alto ou baixo demais, em unidades S de 6 dB.

    Um valor negativo abaixa a leitura — para um medidor que mostra demais, um piso de ruído em S5 onde deveria ser S2. Um valor positivo a eleva. 0 é o medidor como medido.

    Só a leitura se move: o ponteiro, os valores S e dBm, o nível de onde parte um relatório de recepção e o S-meter falado. O sinal, o squelch, a gravação e os decodificadores não são tocados.

    Salvo com este rádio na máquina à qual está conectado, e tem efeito imediato.

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1 e 2 usam ambos { $host }:{ $port } — cada spot chegará duas vezes. Dê a eles portas diferentes (7000 CW/RTTY, 7001 FT8/FT4) ou desligue um.

settings-rx-site-station = Na estação

settings-rx-site-elsewhere = Em outro lugar

settings-remote-tab-note =
    A estação aparece como uma aba ao lado dos rádios já abertos aqui, e a faixa de abas no topo da janela alterna entre eles (⊞ coloca dois lado a lado). Feche-a pela lista no topo da aba Rádio — isso desliga a conexão; nada no servidor é alterado.

    Se o servidor pedir usuário e senha, a tela de login dele aparece na nova aba. Um link ws:// simples não leva nada disto criptografado, então pela internet aberta use uma VPN ou um túnel SSH, ou termine o HTTPS na frente do servidor e marque Seguro acima.

settings-cw-source-keyboard = TECLADO

settings-cw-source-usb = MANIPULADOR USB

pane-frame = QUADRO

pane-frames = QUADROS

pane-words = PALAVRAS

antenna-feed-z0 = Z0 da linha:

antenna-feed-z0-hint = Impedância da linha de transmissão/sistema para a ROE (50 Ω típico)

antenna-height-label = Altura:

antenna-height-hint = Altura do ponto de alimentação/antena acima do solo (afeta o ângulo de saída/diagrama)

antenna-m-agl = m AGL

antenna-apply = Aplicar

antenna-apply-hint = Executa o solver de método dos momentos nas predefinições.

antenna-yagi-optimizer = Otimizador de Yagi

antenna-stack = Empilhamento:

antenna-objective = Objetivo:

antenna-optimize = Otimizar

antenna-swr-value = ROE { $v }

antenna-swr-ref = ROE { $v } ({ $z }Ω)

antenna-radiation-pattern = Diagrama de radiação

antenna-view = Vista

antenna-view-front = Frente

antenna-view-side = Lateral

antenna-view-top = Topo

antenna-3d-hint = 3D arraste para girar • calor = intensidade de campo

antenna-yagi-x = Yagi ×{ $n }

settings-tool-window-session-qsos = QSOs DA SESSÃO

sessionqso-time = Hora

sessionqso-call = Indicativo

sessionqso-grid = Grid

sessionqso-band = Banda

sessionqso-mode = Modo

sessionqso-rst-s = RST E

sessionqso-rst-r = RST R

sessionqso-country = País

sessionqso-dist = Dist

sessionqso-final = Final

sessionqso-count = { $n } QSO nesta sessão

sessionqso-include-cw = Incluir CW

decodes-live-log = LOG AO VIVO

decodes-setup = ⚙ CONFIG

schedule-colour-all = Cor

schedule-colour-all-hint = Cor do rótulo no waterfall para todas as estações — substitui as escolhidas por estação

schedule-colour-station-hint = Cor do rótulo no waterfall desta estação

schedule-colour-reset = PADRÃO

schedule-colour-reset-hint = Volta à cor padrão de todas as estações, esquecendo as escolhidas

settings-language = Idioma

settings-language-note = O idioma em que a interface é exibida. Padrão do sistema segue o sistema operacional; escolha outro para substituí-lo. A escolha é salva e lembrada.

aprs-all-chip = TODAS

aprs-all-chip-hint = Aceso: todas as mensagens ouvidas no canal, seja qual for o destinatário. Apagado: só as para você ou de você, e boletins.

aprs-composer-hidden-swl = O envio está oculto: o modo SWL / escuta está ativado. Desative-o para escrever e enviar uma mensagem.

aprs-compose-count-hint = Caracteres usados dos 67 que uma mensagem APRS pode conter.

tx-bands-title = Bandas de transmissão

tx-bands-note = Mude TX para SIM apenas nas bandas em que este rádio realmente transmite. Isto vem antes de qualquer outra configuração de transmissão; SIM nunca remove os demais limites de segurança.

tx-bands-careful = ⚠ Uma banda errada pode destruir o rádio: verifique cada opção com atenção.

tx-bands-yes = SIM

tx-bands-no = NÃO

tx-bands-confirmed = ✓ Bandas de transmissão confirmadas para este rádio

tx-bands-ack = Verifiquei estas bandas no meu rádio

tx-bands-confirm = Confirmar estas bandas

tx-bands-confirm-hint = Marque a caixa primeiro: a confirmação é feita uma vez para cada rádio.

tx-bands-check = Verificar bandas…

tx-bands-notice-new = A transmissão está desligada em todas as bandas. Escolha as bandas em que este rádio transmite e confirme-as uma vez. Devido ao risco para o rádio, verifique com atenção as opções escolhidas.

tx-bands-notice-carried = Suas bandas de transmissão foram trazidas da versão anterior. A transmissão continua bloqueada até que você as verifique e confirme uma vez. Devido ao risco para o rádio, verifique cada opção com atenção.


adsbmap-surface-asphalt = asfalto

adsbmap-surface-concrete = concreto

adsbmap-surface-grass = grama

adsbmap-surface-gravel = cascalho

adsbmap-surface-dirt = terra

adsbmap-surface-water = água

adsbmap-airport-runway-lit = iluminada

adsbmap-airport-more = +{ $n } mais

adsbmap-airport-freq-head = Frequências, MHz

adsbmap-airport-data-date = dados de pistas e frequências de { $date }
aircraft-online-switch = Online

aircraft-online-switch-on-hover = Online: tipo, operador, rota e foto vêm do adsbdb e do Planespotters, imagens de aeroportos da Wikipedia, e a hora local e a temperatura de um aeroporto do Open-Meteo (só a posição do aeroporto é enviada), consultados pela sua estação para você. Clique para desligar; a estação guarda a sua escolha. Nada mais sai da estação.

aircraft-online-switch-off-hover = Offline: nada sai da estação. Clique para ativar tipo, operador, rota e foto (adsbdb e Planespotters), imagens de aeroportos (Wikipedia) e clima do aeroporto (Open-Meteo).

aircraft-switch-held = o operador da estação a mantém com SDROXIDE_AIRCRAFT_ONLINE

adsbmap-airport-photo-credit = Foto: Wikipedia

adsbmap-airport-local-time = Hora local { $time } ({ $zone })

adsbmap-airport-temperature = Temperatura { $temp }

adsbmap-airport-weather-credit = Clima: Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time } (hora local: carregando…)

adsbmap-airport-utc-offline = UTC { $time } (hora local: offline)

schedule-listen = OUVIR

schedule-listen-hint = Sintoniza o receptor nesta estação

schedule-sort = Ordenar

schedule-sort-asc = Crescente

schedule-sort-desc = Decrescente

schedule-col-freq = Freq.

schedule-col-band = Banda

schedule-col-name = Nome

schedule-col-lang = Idioma

schedule-col-target = Alvo

schedule-col-site = Local

schedule-col-local = Local

tx-bands-off = A transmissão está desligada em { $band }: esta banda não é permitida nas suas configurações, portanto seu transmissor não é compatível com transmissão nesta banda. (Mostrado no máximo { $max } vezes por sessão.)

tx-off-band = TRANSMISSÃO PROIBIDA — { $freq } MHz ({ $band }) não é uma banda de transmissão validada. Marque e valide em Configurações → Rádio → Bandas de transmissão.

tx-off-unconfirmed = TRANSMISSÃO PROIBIDA — as bandas de transmissão deste rádio ainda não foram validadas. Verifique e confirme em Configurações → Rádio → Bandas de transmissão.

aircraft-hover-lookup-retry = sem resposta — nova tentativa em { $s } s

adsbmap-airport-navaids-head = Auxílios de rádio no aeródromo

adsbmap-airport-no-freq = Nenhuma frequência publicada para este aeroporto

topbar-preset-right-click = Clique com o botão direito para alterar esta predefinição

topbar-preset-edit = EDITAR PREDEFINIÇÃO

topbar-preset-label = Rótulo

topbar-preset-label-hint = O que o chip mostra. Até 8 caracteres.

topbar-preset-lo = Inf.

topbar-preset-hi = Sup.

topbar-preset-use-current = Usar atual

topbar-preset-use-current-hint = Usar as bordas do filtro que o receptor tem agora

topbar-preset-save = Salvar

topbar-preset-reset = Redefinir

topbar-preset-reset-hint = Restaurar a predefinição integrada deste modo

topbar-preset-cancel = Cancelar
wefax-schedule = HORÁRIO
wefax-schedule-hint = Abre no navegador a programação publicada por { $station } — cartas e horários UTC.

tx-bands-check-again = REVER

tx-bands-check-again-hint = Retira a confirmação para rever a lista. Alterar qualquer banda já faz isso sozinho.
