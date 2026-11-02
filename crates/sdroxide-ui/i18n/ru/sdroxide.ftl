# sdroxide — русский перевод интерфейса. Образец — en-US/sdroxide.ftl;
# то, чего здесь нет, показывается по-английски.
#
# Ids are grouped by where they are shown, and named after it:
# settings-ui-* is the UI tab of the Settings window, and so on. A message
# shown in capitals is written in capitals here; the code does not upper-case
# translated text.

## The language's own name, as the language picker lists it.
language-name = Русский

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = Авто
layout-mode-desktop = Десктоп
layout-mode-tablet = Планшет
layout-mode-small = Маленький экран
layout-mode-phone = Телефон

ui-theme-default = По умолчанию
ui-theme-light = Светлая
ui-theme-high-contrast = Высокий контраст
ui-theme-green-phosphor = Зелёный фосфор
ui-theme-amber-phosphor = Янтарный фосфор
ui-theme-teal-orange = Бирюзовая / оранжевая
ui-theme-rainbow = Радуга
ui-theme-nord = Nord
ui-theme-nord-dark = Nord тёмная
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized тёмная
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin mocha
ui-theme-catppuccin-latte = Catppuccin latte
ui-theme-modern-minimalist = Современная минималистичная

chrome-style-angled = С углами
chrome-style-rectangular = Прямоугольная
chrome-style-rounded = Скруглённая
chrome-style-gradient = Градиент
chrome-style-bevel = Объёмная (3D)
chrome-style-terminal = Терминал

font-size-small = Малый
font-size-medium = Средний
font-size-large = Большой

bandplan-kind-ham = Любительская
bandplan-kind-cw = CW
bandplan-kind-digi = Цифровая
bandplan-kind-phone = Голосовая
bandplan-kind-beacon = Маяки
bandplan-kind-broadcast = Вещание
bandplan-kind-am = AM / ДВ / СВ
bandplan-kind-cb = CB

speech-verbosity-terse = Кратко
speech-verbosity-normal = Обычно
speech-verbosity-full = Полностью

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = Шкала (14 point 074)
speech-freq-style-digits = Каждая цифра
speech-freq-style-full = Шкала с единицами

speech-callsign-style-phonetic = Фонетически (kilo one alpha)
speech-callsign-style-letters = Буквами (kay one ay)
speech-callsign-style-as-is = Как написано

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = Отображение
settings-ui-moved-note = Детализация панорамы, реакция спектра и скорость прокрутки водопада задаются кнопкой SPEC на полосе отображения — меню DISP в узком окне.
settings-ui-language = Язык
    .hint = Язык интерфейса. Руководство к F1 и голосовые объявления остаются на английском независимо от выбора здесь.
settings-ui-layout = Раскладка
settings-ui-theme = Тема
settings-ui-button-style = Стиль кнопок
settings-ui-frame-rate = Частота обновления экрана
settings-ui-palette = Палитра водопада
settings-ui-tuning-buttons = Кнопки настройки
settings-ui-tuning-buttons-show = Показывать на телефоне и планшете
    .hint = Ряд кнопок «вниз / шаг / вверх» под полосой управления на сенсорном экране. Колеса для прокрутки цифр и ручки настройки нет, поэтому без них перейти на известный шаг можно только вводом всей частоты. На десктопе не показываются никогда.
settings-ui-tuning-step = Шаг: { $step }
settings-ui-tuning-snap = Первое нажатие выравнивает по шагу
    .hint = Если шкала остановилась между двумя кратными шага, следующее нажатие переводит её к следующему кратному в нужном направлении — 7 074 300 при шаге 1 кГц уходит вверх до 7 075 000 или вниз до 7 074 000. Дальше кнопки двигаются на шаг как обычно. Если выключено, каждое нажатие сдвигает ровно на шаг.
settings-ui-smoothing = Сглаживание водопада
settings-ui-smoothing-interpolate = Интерполяция
    .hint = Смешивает каждый пиксель экрана с соседними бинами и строками, чтобы сигнал выглядел непрерывным там, где дисплей шире преобразования. Выключите для прямоугольного водопада — один блок на бин и на строку, — который нужен, чтобы читать форму сигнала по картинке: интерполированный сигнал не отличить от действительно более широкого. Вторая половина этого — больший FFT (микросхема FFT).
settings-ui-background = Фон спектра
settings-ui-background-gradient = Градиент
settings-ui-background-top = верх
settings-ui-background-bottom = низ
settings-ui-spot-colours = Цвета меток спотов
    .hint = Оттенок, которым окрашивается каждый источник спотов: плашки внизу водопада, значки в списке SPOTS и точки на карте мира.
settings-ui-spot-colour = Цвет спотов: { $kind }
settings-ui-spot-colours-reset = Вернуть все цвета спотов по умолчанию
settings-ui-reset = Сброс
settings-ui-bandplan-colours = Цвета плана диапазонов
    .hint = Оттенок, которым закрашивается каждый класс распределения на полосе плана диапазонов внизу водопада. Блоки рисуются полупрозрачными поверх водопада, поэтому там они темнее, чем на образце здесь.
settings-ui-bandplan-colour = Цвет распределения: { $kind }
settings-ui-bandplan-colours-reset = Вернуть все цвета плана диапазонов по умолчанию
settings-ui-skimmer-font = Размер шрифта скиммера
settings-ui-waterfall-font = Размер шрифта водопада / спектра
settings-ui-interface-font = Размер шрифта интерфейса
    .hint = Масштабирует весь интерфейс — меню, диалоги, окна, вкладки радио, верхнюю панель и её кнопки. Два размера выше задаются относительно него.
settings-ui-cities = Города на картах
    .hint =
        Показывать города мира — точку на каждый населённый пункт, с его названием рядом, если есть место — на плоских картах: FT8/WSPR, APRS, ADS-B и AIS.

        They are most of what says *where* a dot is on a map of the whole world, and they are also the busiest thing on it. Turn them off if the names are getting in the way of the stations you are reading. The 3D globe is unaffected — its cities are night-side lights rather than markers, and nothing there is written across a contact.
settings-ui-cities-show = показывать города и их названия
settings-ui-3d-view = Трёхмерный вид
settings-ui-clouds = Отрисовка облаков
settings-ui-clouds-layered = Слоистые
settings-ui-clouds-volumetric = Объёмные
settings-ui-clouds-note = Как слой CLOUDS в трёхмерном виде рисует погоду. Слоистые складывают срезы тропосферы и это дешёвый вариант. Объёмные вместо этого пускают через неё луч, так что Солнце бросает тени верхушек облаков на нижележащую поверхность, а молнии светятся изнутри грозы, создавая её, а не просто делая ярче её внешность — ценой в несколько раз большей нагрузки на пиксель.

## Settings window, UI tab: voice announcements.

speech-title = Голосовые объявления
speech-enabled = Озвучивать изменения радио
    .hint = Зачитывает, что изменилось, чтобы радио можно было использовать, не глядя на него
speech-voice = Голос
speech-voice-shipped = Голос из комплекта
speech-speed = Скорость
    .hint = Голос растягивает или сжимает собственную фразировку, поэтому высота тона не меняется. Дальше примерно 2× короче уже не становится.
speech-volume = Громкость
speech-output = Вывод
speech-detail = Подробность
speech-duck = Приглушать приёмник
speech-duck-while = Пока говорит
speech-test = Проверить
speech-what = Что объявлять
speech-cat-frequency = Частота
speech-cat-mode-band = Режим и диапазон
speech-cat-vfo-split = VFO и split
speech-cat-agc-gain = AGC и усиление
speech-cat-levels = Мощность, настройка и микрофон
speech-cat-ptt = Передача и приём
speech-cat-rit-xit = RIT и XIT
speech-cat-memory-scan = Память и сканирование
speech-cat-band-edge = Выход за любительский диапазон
speech-cat-notices = Предупреждения и сообщения
speech-cat-filters = Фильтры, шумоподавитель и шумоподавление
    .hint = По умолчанию выключено: эти значения постоянно меняются во время поиска сигнала
speech-decoded = Декодированные сообщения
speech-ft8-to-me = Вызовы FT8 мне
speech-ft8-cq = Вызовы CQ FT8, на которые я могу ответить
    .hint = Оживлённый вечер на двадцатиметровом диапазоне — это сотня таких в минуту
speech-js8 = Сообщения JS8 мне
speech-js8-allcall = JS8 @ALLCALL тоже
speech-fsq = Сообщения FSQ мне
speech-include-snr = Включать отчёт
    .hint = Только там, где в сообщении нет собственного — декодирование, которое уже сообщает число, не получает наше
speech-ft8-qso = Мой собственный обмен FT8
    .hint = Что сейчас отправит секвенсор и завершение связи
speech-reading-text = Зачитывать декодированный текст вслух
speech-text-cw = CW
speech-text-rtty-psk = RTTY, PSK, Olivia, THOR, FSQ
speech-text-cw-locked = CW только когда декодер захвачен
    .hint = Чтение выхода незахваченного декодера хуже тишины
speech-text-note = Оба выключены по умолчанию. Декодер выдаёт текст быстрее, чем его можно прочитать вслух, поэтому всё, что отстаёт от живого звука слишком далеко, отбрасывается, а не ставится в очередь.
speech-tuning-up = Настройка
speech-swr-while-tuning = Зачитывать КСВ, пока удерживается TUNE
speech-swr-every = Каждые
speech-swr-summary = Сообщать лучшее согласование при отпускании
speech-swr-alarm = Предупреждать о высоком КСВ при любой передаче
speech-how-read = Как читаются значения
speech-frequencies = Частоты
speech-callsigns = Позывные
speech-quiet-on-tx = Молчать во время передачи
    .hint = Речь идёт в ваши динамики, а значит, в микрофон. Предупреждения о высоком КСВ всё равно проходят.
speech-device-note = Объявления воспроизводятся на собственном звуковом устройстве, поэтому их никогда не записывают и никому удалённо не отправляют. Клавиши для произнесения состояния, повтора последнего объявления и остановки посреди фразы — на вкладке Controls.

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = Ожидание сетевой конфигурации станции…
settings-device-reports = как сообщает устройство
settings-transverters = Трансверторы
settings-one-row-per-box-while = Одна строка на блок. Пока шкала находится внутри диапазона строки, радио настроено на шкалу + смещение, и действуют правило передачи и ограничение мощности этой строки; вне всех строк радио работает на своих диапазонах. Строки проверяются сверху вниз.
settings-name = Название
settings-band-low = Нижняя граница диапазона
settings-band-high = Верхняя граница диапазона
settings-offset = Смещение
settings-transmit = Передача
settings-max-drive = Макс. мощность
settings-off-takes-transverter-out-line = «Выкл.» отключает этот трансвертер от тракта, не теряя строки — диапазон возвращается к тому, что радио достаёт само.
settings-t-2-m = 2 м
settings-what-you-call-shown-log = Как вы его называете. Показывается в журнале, когда шкала выбирает его.
settings-bottom-band-transverter-works-dial = Нижняя граница диапазона, с которым работает трансвертер, по шкале. 144 для 2 м.
settings-top-band-dial-148-2 = Верхняя граница диапазона, по шкале. 148 для 2 м в регионах 2 и 3, 146 в регионе 1.
settings-radio-ends-up-dial-offset = Радио оказывается на шкале + смещение, поэтому трансвертер, который переносит диапазон на ПЧ, имеет отрицательное смещение: 2 м в ПЧ 28 МГц — это -116, потому что 144 - 116 = 28. Перетаскивайте, чтобы подстроить по герцу для гетеродина, который немного уходит.

    Through the same converter: it works both ways, and transmit takes the same offset.

    Your Drive setting is held under this rather than moved, so the number you use on HF comes back when the dial leaves the band.
settings-remove = УДАЛИТЬ
settings-add-transverter = ДОБАВИТЬ ТРАНСВЕРТЕР
settings-takes-effect-apply-reconnect-like = Вступает в силу после «Применить / переподключить», как и смещение конвертера выше.
settings-maximum-transmit-drive = Максимальная мощность передачи
settings-hard-limit-drive-tune-controls = Жёсткое ограничение, за которое не могут выйти регуляторы Drive и TUNE; применяется после калибровки диапазона ниже, поэтому ничто не может поднять мощность выше него. Задайте его там, где радио выдаёт номинальную мощность, и тогда весь регулятор Drive становится пригодным — на передатчике, который модулирует собственные отсчёты (HPSDR, LimeSDR, PlutoSDR, HackRF), полная мощность — это передатчик на полную, что на усилителе с высоким усилением заметно выше того, на что рассчитаны его выходные каскады. «Выкл.» означает, что регуляторы дойдут до полной мощности.
settings-limit-drive = Ограничить мощность до
settings-per-cent-full-drive-key = Процент от полной мощности. Подключите радио к эквиваленту нагрузки и уменьшайте значение, пока измеритель не покажет мощность, на которую рассчитан усилитель.
settings-applies-immediately-every-band-tune = Применяется сразу, на каждом диапазоне, и к TUNE тоже.
settings-transmit-drive-band = Мощность передачи по диапазонам
settings-every-amplifier-has-different-gain = У каждого усилителя свое усиление на каждом диапазоне, поэтому одна настройка Drive даёт на каждом разную мощность. Измерьте выход на каждом диапазоне, затем подстройте те, что выдают больше: задайте Drive для диапазона, которому нужно больше всего (обычно самого высокого), а остальные снизьте до него. Децибелы выходной мощности, применяются к голосу, цифровым режимам и TUNE одинаково. Нули везде — значение по умолчанию — ничего не меняют.
settings-other = Другие
settings-clear-all = СБРОСИТЬ ВСЁ
settings-back-no-calibration-drive-setting = Возврат к отсутствию калибровки: настройка Drive доходит до каждого диапазона целиком.
settings-applies-immediately-band-you-would = Применяется сразу и к диапазону, на котором вы бы передавали — за трансвертером это диапазон на шкале, и собственное ограничение мощности трансвертера всё равно важнее всего, что задано здесь.
settings-settings = Настройки
settings-enter-server-s-address-first = сначала введите адрес сервера
settings-enter-receiver-s-address-first = сначала введите адрес приёмника или выберите один из «Публичные SDR»
settings-no-radio-selected-press-discover = радио не выбрано — нажмите «Поиск» или введите адрес
settings-enter-radio-s-address-first = сначала введите адрес радио
settings-general = Общие
settings-radio = Радио
settings-ui = UI
settings-alerts = Оповещения
settings-controls = Управление
settings-spots = Споты
settings-freedv = FreeDV
settings-uploads = Выгрузки
settings-winlink = Winlink
settings-servers = Серверы
settings-t-r-switch = Переключатель T/R
settings-tle = TLE
settings-profiles = Профили
settings-station = Станция
settings-enter-digital-mode-ft8-sstv = Введите цифровой режим (FT8 / SSTV / …) один раз, чтобы загрузить сохранённые значения.
settings-callsign = Позывной
settings-grid-square = Квадрат Майденгеда
settings-iaru-region = Регион IARU
settings-which-radio-interface-server-uses = Какой интерфейс радио использует сервер, задаётся на машине, на которой он работает.
settings-radio-interface = Интерфейс радио
settings-waiting-machine-radio-attached-interface = Ожидание машины, к которой подключено радио. Её настройки интерфейса ниже, и пока их можно менять отсюда.
settings-converter = Конвертер
settings-manual-type-offset-below = Вручную — введите смещение ниже
settings-what-box-reads-when-offset = Что показывает это поле, когда смещение не из списка готовых. Задаётся вводом в поле «Смещение» ниже; выбирать здесь нечего.
settings-frequency-converter-front-receiver-pick = Частотный конвертер перед приёмником. Выберите один или введите смещение рядом с ним.
settings-how-far-converter-moves-signal =
    На сколько конвертер сдвигает сигнал на пути к приёмнику, в Гц — с тем же числом и знаком, что во всей документации всех конвертеров и во всех остальных программах для SDR. Радио оказывается на шкала + смещение: положительное для апконвертера (Ham It Up — 125000000), отрицательное для всего, что переносит диапазон на ПЧ — спутниковый LNB или трансвертер 2 м на ПЧ 28 МГц, то есть -116000000 (28 − 144). 0 = нет конвертера.

    Drag to trim it a hertz at a time, which is what a converter whose oscillator is slightly off wants.

    This is the receive path. What is in the transmit line is the row below.

    Takes effect on Apply.
settings-what-transmit-line-while-converter =
    Что стоит в тракте передачи, когда задан конвертер.

    Off while converting: nothing is transmitted — the safe default, and right for a receive-only accessory.

    Through the same converter: one box converts both ways, a transverter. Transmit takes the offset above and follows it when it is trimmed.

    Its own offset: the transmit line is different from the receive one. Nought is the common case — a converter (an LNB, a Ham It Up) on receive with the transmitter on its own antenna, which is the QO-100 station.

    The amateur-band check still applies, on the frequency you are transmitting on.

    Takes effect on Apply.
settings-how-far-transmit-line-moves =
    На сколько тракт передачи сдвигает сигнал, в Гц, по тому же правилу знака, что и смещение приёма: радио передаёт на шкала + смещение.

    0 = the transmitter is wired to its own antenna and works on the frequency the dial says — a QO-100 station's 2.4 GHz uplink.

    A transmit converter that takes an I.F. *up* is a negative number: the radio works below the dial.
settings-rx-range = Диапазон приёма
settings-which-frequencies-radio-receives-mhz =
    Какие частоты принимает это радио, в МГц: 144-146, 430-440. Оставьте пустым, чтобы использовать то, что устройство сообщает о себе.

    Band buttons outside the range are greyed out and the dial will not go there.

    These are dial frequencies. With a converter set, state the band you tune — 144-148 for a 2 m transverter — not the I.F. the radio is really on.

    Takes effect on Apply.
settings-tx-range = Диапазон передачи
settings-which-frequencies-radio-transmits-mhz =
    На каких частотах это радио передаёт, в МГц: 144-146, 430-440. Оставьте пустым, чтобы использовать то, что сообщает устройство, — а если оно ничего не сообщает, драйверу верят на слово и разрешена любая частота.

    Dial frequencies, like the receive range above.

    This is a limit you set, not a licence: transmitting outside the amateur bands is refused regardless unless you have turned that off in config.toml. Nor does it give a receive-only device a transmitter.

    Takes effect on Apply.
settings-antenna = Антенна —
settings-where-radio-listens-station-antenna =
    Где это радио слушает.

    At the station: the antenna is yours, so your own locator says where it is. The default, and right for everything in the shack.

    Somewhere else: an online receiver, or your own set up on a hilltop. What it hears is reported to PSK Reporter, WSPRnet and FreeDV Reporter from the locator beside this, never from yours — and ADS-B places aircraft against it too.

    Picking a receiver under "Public SDRs" fills this in for you.

    Takes effect on Apply.
settings-receiver-s-maidenhead-locator-jn88ec =
    Локатор Майденгеда приёмника — JN88ec, DO30db. Четыре или шесть символов.

    Leave it empty if you do not know where the receiver is: nothing is then reported at all, which is the only honest answer. Reporting from your own square would put somebody else's reception on the wrong continent.

    On top of cal_offset_db in config.toml, which is one figure for the whole station.
settings-antenna-somewhere-else-no-locator = Антенна находится в другом месте, и локатор не задан, поэтому ничто из того, что слышит это радио, не сообщается в PSK Reporter, WSPRnet или FreeDV Reporter. Введите локатор приёмника выше, чтобы сообщать отсюда.
settings-ranges-are-mhz-low-high = Диапазоны задаются в МГц, нижняя-верхняя, через запятую: 144-146, 430-440 — то есть 144000000-146000000 Гц и 430000000-440000000 Гц. Это частоты шкалы: смещение конвертера их не сдвигает. Смещение конвертера выше — поле в герцах. Оставьте диапазон пустым, чтобы использовать то, что устройство сообщает о себе; устройство, которое ничего не сообщает, принимается на слово.
settings-careful-rtl-sdr-blog-v4 = Осторожно с RTL-SDR: Blog V4 сам поднимает частоту ниже 28,8 МГц, поэтому отрицательное смещение, которое приводит аппаратуру туда, сдвигает сигнал дважды.
settings-pick-radio-interface-above-configuration = Выберите интерфейс радио выше (эта конфигурация использовала удалённый режим автоопределения).
settings-configuration-used-removed-auto-detect = Эта конфигурация использовала удалённый режим автоопределения. Выберите интерфейс выше, когда машина, к которой подключено радио, ответит.
settings-radio-has-no-interface-yet = У этого радио пока нет интерфейса — выберите его выше и нажмите «Применить / переподключить».
settings-radio-has-no-interface-yet-2 = У этого радио пока нет интерфейса. Его можно выбрать выше, когда машина, к которой подключено радио, ответит.
settings-apply-reconnect = Применить / переподключить
settings-switch-interface-now-no-restart = Переключить на этот интерфейс сейчас — перезапуск не нужен
settings-switch-server-s-radio-interface = Переключить радио сервера на этот интерфейс сейчас — перезапуск не нужен. Старый продолжает работать, если новый не удаётся открыть.
settings-switches-live-radio-without-restarting = Переключает работающее радио без перезапуска.
settings-everything-above-server-s-own = Всё выше — собственная конфигурация сервера, и изменения сохраняются там. Большинство настроек применяются по мере изменения; те, что фиксируются при открытии устройства — частота дискретизации, адрес, сам интерфейс — требуют этой кнопки.
settings-dx-cluster-telnet = DX-кластер (telnet)
settings-enabled = Включено
settings-host = Хост
settings-port = Порт
settings-login-call = Позывной для входа
settings-commands = Команды
settings-one-per-line-e-g = по одной на строку, например SET/FT8
settings-one-per-line-e-g-2 = по одной на строку, например set/filter cont=eu
settings-sent-after-login-place-narrow = Отправляется после входа. Место, чтобы сузить поток — без фильтра это каждый скиммер на Земле.
settings-rbn-paths-are-placed-country = Пути RBN размещаются по центрам стран, а не по локаторам — точно для маленькой страны и далеко не точно для большой. Это отдельный слой на карте распространения, и его можно выключить там.
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = Споты активаторов POTA
settings-sota-spots = Споты SOTA
settings-psk-reporter-current-band = PSK Reporter (текущий диапазон)
settings-upload-my-ft8-ft4-ft2 = Выгружать мои декодирования FT8/FT4/FT2/JS8
settings-antenna-2 = Антенна
settings-collector = Коллектор
settings-t-4739-live-collector-14739-test = 4739 — рабочий коллектор, 14739 — тестовый
settings-max-age-s = Макс. возраст (с)
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = Выгружать мои декодирования WSPR
settings-send-every-wspr-reception-wsprnet = Отправлять каждый приём WSPR на wsprnet.org. Включено по умолчанию: ничего не выходит в эфир, а сообщение о том, что вы слышите, — то, что делает приёмник WSPR частью сети, а не частным любопытством. Слот, в котором ничего не декодировано, тоже сообщается — так сеть отличает закрытый диапазон от выключенного приёмника.
settings-download-who-heard-me = Загрузить, кто меня слышал
settings-ask-wsprnet-org-which-stations = Спросить wsprnet.org, какие станции декодировали эту. У WSPR нет подтверждения никакого рода, поэтому это единственный способ для передающего маяка узнать что-либо о своём охвате.
settings-ask-every = Спрашивать каждые
settings-looking-back = глядя назад
settings-apply = {" "}ПРИМЕНИТЬ{" "}
settings-persist-re-connect-feeds = Сохранить и (пере)подключить потоки
settings-broadcast-stations = Станции вещания
settings-winlink-account = Учётная запись Winlink
settings-password = Пароль
settings-winlink-account-password-not-gateway = Пароль учётной записи Winlink, а не пароль шлюза. Он чувствителен к регистру — вводите ровно так, как его выдали.
settings-locator = Локатор
settings-how-connect = Как подключаться
settings-route = Маршрут
settings-internet = Интернет
settings-forward-cms-over-internet = Пересылать через CMS по интернету
settings-radio-packet = Радио (пакет)
settings-call-rms-gateway-air-radio = Вызвать шлюз RMS в эфире. Радио должно быть в режиме PACKET или PACKET-HF.
settings-gateway = Шлюз
settings-via = Через
settings-digipeaters-order-separated-spaces-usually = Диджипитеры по порядку, через пробел. Обычно пусто — шлюз, который вы слышите напрямую, следует вызывать напрямую.
settings-speed = Скорость
settings-most-rms-gateways-answer-1200 = Большинство шлюзов RMS отвечают на 1200. Вызов шлюза 1200 на 9600 звучит точно так же, как шлюз, не работающий в эфире, поэтому задавайте это по тому, что публикует владелец шлюза, а не угадывайте. 9600 также требует порта данных радио на обоих концах. 300 — это HF.
settings-frequency = Частота
settings-clear = СБРОС
settings-zero-leaves-dial-alone-which = Ноль не трогает шкалу, и это то, что нужно, когда вы стоите на одном канале. Любое другое значение настраивает радио при запуске сессии.
settings-my-gateways = Мои шлюзы
settings-winlink-s-published-gateway-list = Опубликованный список шлюзов Winlink требует ключ API, которого у sdroxide нет, поэтому ведите свой. Два-три шлюза, доступных из одной точки, узнаются опытным путём, и они редко меняются.
settings-use-2 = ИСП.
settings-call-one-next-connect = Вызывать этот при следующем подключении
settings-current-dial = текущая шкала
settings-forget = ЗАБЫТЬ
settings-forget-gateway = Забыть этот шлюз
settings-add-gateway = + ДОБАВИТЬ ШЛЮЗ
settings-save-gateway-above-list = Сохранить шлюз выше в список
settings-internet-gateway = Интернет-шлюз
settings-cms-address = Адрес CMS
settings-client-name = Имя клиента
settings-winlink-s-production-servers-only = Производственные серверы Winlink принимают только известные им имена клиентов и на неизвестное отвечают «Unknown client types are not allowed on production servers». Пока sdroxide не зарегистрирован в Winlink Development Team, для подключения нужно имя, которое они узнают.
settings-automatic-connection = Автоматическое подключение
settings-connect-timer = Подключаться по таймеру
settings-every = Каждые
settings-persist-winlink-account = Сохранять учётную запись Winlink
settings-callsign-lookup = Поиск позывного
settings-provider = Провайдер
settings-auto-fill-name-qth-grid = Автозаполнение имени/QTH/квадрата при клике по споту и QSO
settings-choose-provider-enter-login = Выберите провайдера, чтобы ввести его логин.
settings-qrz-user = Пользователь QRZ
settings-qrz-pass = Пароль QRZ
settings-hamqth-user = Пользователь HamQTH
settings-hamqth-pass = Пароль HamQTH
settings-one-hamqth-account-does-both = Одна учётная запись HamQTH делает оба — этот логин используется вкладкой выгрузки HamQTH.
settings-upload = Выгрузка
settings-auto-upload-each-new-qso = Автовыгрузка каждого нового QSO
settings-master-switch-which-services-qso = Главный переключатель. Куда отправляется QSO, задаётся на каждой вкладке ниже.
settings-turn-auto-upload-each-new = Сначала включите «Автовыгрузка каждого нового QSO» выше — сервис, отмеченный здесь, не выгружается, пока эта опция не включена.
settings-auto-upload-off-above-so = Автовыгрузка выключена выше, поэтому отметки сервисов отключены, и ничего пока не выгружается автоматически — кнопка UP для каждого QSO в журнале всё ещё работает.
settings-qrz-log-key = Ключ журнала QRZ
settings-qrz-logbook-api-key-your = Ключ API журнала QRZ из настроек журнала QRZ — не логин XML-поиска.
settings-eqsl-user = Пользователь eQSL
settings-eqsl-pass = Пароль eQSL
settings-eqsl-qth = QTH eQSL
settings-same-login-hamqth-callsign-lookup = Тот же логин, что у поиска позывного HamQTH — заполнение любого из них заполняет оба.
settings-club-log-email = Эл. почта Club Log
settings-club-log-pass = Пароль Club Log
settings-club-log-key = Ключ Club Log
settings-wrl-api-key = Ключ API WRL
settings-developer-api-key-world-radio = Ключ API разработчика из World Radio League → Integrations → Developer API. Он показывается один раз при создании, так что скопируйте его тогда. Связи уходят в ваш журнал по умолчанию — задайте его в WRL, если у вас их несколько.
settings-confirmations-download = Подтверждения (загрузка)
settings-lotw-user = Пользователь LoTW
settings-lotw-pass = Пароль LoTW
settings-lotw-upload-uses-tqsl-export = Выгрузка в LoTW использует TQSL — экспортируйте ADIF из журнала и подпишите его. Подтверждения LoTW/eQSL загружаются здесь, чтобы отмечать «проведено» и «подтверждено».
settings-sync-confirmations = СИНХРОНИЗИРОВАТЬ ПОДТВЕРЖДЕНИЯ
settings-fetching-subscriptions = Загрузка подписок…

    To use it on its own again, clear the Panadapter receiver box on that radio's page.
settings-panadapter = Панорама
settings-give-radio-wideband-panadapter-borrowing = Дайте этому радио широкополосную панораму, одолжив приёмник другого радио — SDR на той же антенне или питаемый от ПЧ этого радио. Спектр, водопад, вспомогательный приёмник, цифровые режимы и скиммер приходят от этого приёмника; шкала, режим, фильтр и передатчик остаются здесь.
settings-receiver = Приёмник
settings-none = Нет

    Takes effect on Apply.
settings-connected = Подключено к

    This is not the Converter field above, which retunes the radio, and not the CAT tab's I/Q centre offset, which is about this radio's own sound card. Nothing typed here is ever sent to either radio.

    Takes effect on Apply.
settings-audio-2 = Звук из
settings-follow-dial = Следовать за шкалой
settings-keep-receiver-radio-s-dial = Держать приёмник на шкале этого радио и двигать шкалу, когда вы настраиваетесь на панораме. Выкл. оставляет приёмник на месте — постоянное наблюдение за одним сегментом, пока радио уходит в другое место.
settings-invert-spectrum = Инвертировать спектр
settings-mirror-receiver-s-span-about = Отразить полосу приёмника относительно его центра — для отвода, гетеродин которого стоит выше сигнала и передаёт диапазон задом наперёд. Не включайте, если водопад не заполнен сигналами, которые все на неправильной стороне шкалы.
settings-mute-transmit = Отключать звук при передаче
settings-silence-receive-audio-while-radio = Глушить звук приёма, пока это радио передаёт. Включено по умолчанию: при приёмнике на той же антенне или на ПЧ этого радио то, что он слышит во время передачи, — ваш собственный передатчик. Передачи, запущенные на самом радио — PTT микрофона, педаль, MOX — тоже учитываются, как только CAT-связь сообщит, что аппарат перешёл на передачу.
settings-blank-transmit = Гасить при передаче
settings-stop-panadapter-waterfall-while-radio = Останавливать панораму и водопад, пока это радио передаёт. Включено по умолчанию: передатчик, нарисованный на всей полосе, стирает диапазон за собой. Выключите, чтобы смотреть диапазон — или свой собственный сигнал — во время передачи, что полезно при приёмнике на отдельной антенне. Относится к передачам, которые запускает sdroxide: передача, запущенная на самом радио, принимается насквозь, так что картинка продолжает идти независимо от этой настройки.
settings-per-mode-offsets-radio-whose = Смещения по режимам — для радио, чей ПЧ меняется вместе с режимом. Режим, оставленный пустым, использует смещение выше.
settings-computer = На этом компьютере
settings-add-radio-computer = Добавить радио на этот компьютер
settings-radio-s-settings-are-below = Настройки этого радио ниже. Перетащите, чтобы переместить вдоль полосы.
settings-switch-radio-dialog-follows-drag = Переключиться на это радио (диалог следует за ним). Перетащите, чтобы переместить вдоль полосы.
settings-tx = ● TX
settings-mute-radio-s-audio = Отключить звук этого радио
settings-close-radio-configuration-kept = Закрыть это радио (его конфигурация сохраняется)
settings-close-tab-close-radio-station = Закрыть эту вкладку или закрыть радио на его станции
settings-close-tab = Закрыть эту вкладку
settings-transverters-full = { $max } — максимум, который держит эта таблица
settings-dialling = Набор { $url }…
settings-region-note = Где находится станция. Задаёт каждый план диапазонов: границы диапазонов, подсегменты CW / данных / телефонии на полосе водопада, где слушают скиммеры, предлагаемые частоты вызова и что считается вне диапазона для передачи. В регионе { $region } это означает, что 70 см { $region ->
        [1] 430–440 МГц — так что 446 вне диапазона, а 40 м останавливается на 7.200
        [2] 420–450 МГц, а 40 м идёт до 7.300
       *[3] 430–450 МГц, а 80 м останавливается на 3.900
    }. Вступает в силу сразу и применяется ко всем радио этой станции.
settings-rx-grid-reported = Приёмы через это радио сообщаются из { $grid }, а не из вашего локатора.
settings-rx-grid-invalid = { $grid } — не локатор Майденгеда, поэтому ничто из того, что слышит это радио, не сообщается. Четыре или шесть символов: две буквы, две цифры и, по желанию, ещё две буквы.
settings-gateway-via = { " " }через { $via }
settings-auto-upload-to = Автовыгрузка каждого нового QSO в { $service }
settings-tle-updated = Обновлено: { $n } { $n ->
        [one] подписка
        [few] подписки
       *[other] подписок
    }: { $total } спутников.
settings-tle-updated-failed = Обновлено { $ok } из { $n }; { $failed } не удалось — см. строки выше.
settings-panadapter-of = Это радио — панорама { $radio }
settings-radio-n = радио { $n }
settings-on-station = На станции { $station }
settings-add-radio-on-station = Добавить радио на { $station } — станция обслуживает его сразу
settings-panadapter-receiver-for = Приёмник панорамы для { $radio }
settings-hang-up-stays-on = Положить трубку. Радио остаётся на { $station }.
settings-close-on-station = Закрыть { $radio } на { $station }? Его конфигурация остаётся на той машине; вкладка здесь закрывается вместе с ней, и вместе с ней — у всех остальных.
settings-close-it-on = Закрыть на { $station }
settings-locator-hint = локатор

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = Звуковые оповещения
settings-alerts-sound-alarm-when-decode-matters = Звуковой сигнал, когда декодирование важно
settings-alerts-plays-over-own-audio-output = Воспроизводится на собственном аудиовыходе, поэтому его слышно, даже когда диапазон звучит в другом динамике, чем этот экран.
settings-alerts-volume = Громкость
settings-alerts-output = Вывод
settings-alerts-test = Проверить
settings-alerts-what-sound = Что озвучивать
settings-alerts-each-station-quiet-while-after = После оповещения каждая станция какое-то время молчит, поэтому оживлённый диапазон не звонит в каждом слоте. Голос и «Тон + голос» озвучивает голос объявлений — включите его в Настройки → UI.

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = Усилитель
settings-amp-waiting-station-s-amplifier-configuration = Ожидание конфигурации усилителя станции…
settings-amp-hobbypcb-hardrock-500-hardrock-50 = HobbyPCB Hardrock-500 или Hardrock-50 на его порту USB-serial. SDRoxide читает его измерители, переключает его диапазон, режим и антенну и управляет его тюнером из окна AMP. Усилитель по‑прежнему манипулируется линией PTT радио, как раньше.
settings-amp-enable = Включить
settings-amp-model = Модель
settings-amp-untested-panel-has-only-been = Не проверено: эта панель проверялась только с Hardrock-500. Пределы измерителей для этой модели масштабированы от Hardrock-500 и могут быть неверны. Смотрите на собственный дисплей усилителя.
settings-amp-port = Порт
settings-amp-choose = — выберите —
settings-amp-baud = Скорость
settings-amp-must-match-amplifier-s-menu = Должна совпадать с меню усилителя. Собственная панель управления HobbyPCB требует 115200.
settings-amp-follow-radio-s-band = Следовать за диапазоном радио
settings-amp-sends-transmit-frequency-amplifier-every = Отправляет частоту передачи усилителю при каждой смене диапазона, чтобы его фильтры следовали за радио без собственного CAT-кабеля. Никогда не отправляется во время передачи.
settings-amp-show-power-swr-tx-meter = Показывать его мощность и КСВ на измерителе TX
settings-amp-only-where-radio-does-not = Только там, где радио их не измеряет само, и только пока усилитель в тракте.
settings-amp-off = Выкл.
settings-amp-connecting = Подключение…
settings-amp-status = Состояние
settings-amp-apply = {" "}ПРИМЕНИТЬ{" "}
settings-amp-save-re-open-amplifier-s = Сохранить и (пере)открыть порт усилителя

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = Клавиатура
settings-controls-click-shortcut-rebind-then-press = Щёлкните сочетание, чтобы переназначить его, затем нажмите комбинацию клавиш (Esc — отмена). Назначения игнорируются, пока вы печатаете в текстовом поле.
settings-controls-shortcut = Сочетание
settings-controls-does = Действие
settings-controls-step-mode = Шаг / режим
settings-controls-accel = Акселератор
settings-controls-on = Вкл.
settings-controls-press-key = нажмите клавишу…
settings-controls-remove-binding = Удалить это назначение
settings-controls-add-shortcut = + Добавить сочетание
settings-controls-restore-defaults = Восстановить по умолчанию
settings-controls-bind-hold-talk-space = Назначить разговор с удержанием на Space
settings-controls-hold-space-transmit-releasing-losing = Удерживайте Space для передачи; отпускание — или потеря фокуса окна — снимает с передачи
settings-controls-unkey-held-ptt-after = Снимать удерживаемый PTT через
settings-controls-backstop-against-stuck-key-controller = Страховка от залипшей клавиши или контроллера, который перестал отвечать. 0 отключает.
settings-controls-panadapter-mouse = Мышь на панораме
settings-controls-wheel = Колесо
settings-controls-wheel-shift = Колесо + Shift
settings-controls-tune-step = Шаг настройки
settings-controls-zoom-rate = Скорость масштабирования
settings-controls-click-tune-rounding = Округление при настройке кликом
settings-controls-invert-wheel-direction = Инвертировать направление колеса
settings-controls-left-drag-tunes-well-pans = Перетаскивание левой кнопкой настраивает, а не только двигает вид
settings-controls-off-makes-left-drag-pan = Выкл. — перетаскивание левой кнопкой только двигает вид, как правой.
settings-controls-scroll-digit-frequency-readout-tune = Прокрутить цифру на индикаторе частоты, чтобы настроить её
settings-controls-both-wheel-actions-are-tune = Оба действия колеса — настройка; масштабирования не остаётся.
settings-controls-restore-mouse-defaults = Восстановить настройки мыши по умолчанию
settings-controls-mouse-buttons = Кнопки мыши
settings-controls-left-right-buttons-are-reserved = Левая и правая кнопки зарезервированы для настройки и перемещения; средняя и дополнительные кнопки свободны. Боковая кнопка, удерживаемая для PTT, ведёт себя как педаль.
settings-controls-add-mouse-button = + Добавить кнопку мыши
settings-controls-f1-always-opens-manual-even = F1 всегда открывает это руководство, даже во время набора текста, поэтому переназначить её нельзя.
settings-controls-midi-controller = MIDI-контроллер
settings-controls-midi-controllers-need-native-app = MIDI-контроллерам нужно нативное приложение — в браузерном клиенте нет доступа к MIDI.
settings-controls-any-class-compliant-midi-surface = Работает любая MIDI-поверхность, соответствующая классу: колесо джога DJ-контроллера делает хорошую ручку VFO, его пэды делают кнопки PTT и диапазонов, его фейдеры — регуляторы усиления.
settings-controls-enable = Включить
settings-controls-controller = Контроллер
settings-controls-feedback = Обратная связь на
settings-controls-rescan-ports = Пересканировать порты
settings-controls-not-connected = Не подключено.
settings-controls-move-control-see-here = Переместите органы управления, чтобы увидеть их здесь.
settings-controls-control = Орган управления
settings-controls-reads = Читается как
settings-controls-led = Светодиод
settings-controls-move = переместите его…
settings-controls-click-then-move-control-you = Щёлкните, затем переместите орган управления, который хотите назначить
settings-controls-speed-sensitivity-spin-faster-tune = Чувствительность к скорости: крутите быстрее, чтобы настраивать быстрее
settings-controls-send-current-value-back-light = Отправлять текущее значение обратно, чтобы зажечь светодиод или подвинуть фейдер
settings-controls-add-midi-control = + Добавить MIDI-управление
settings-controls-clear-all = Очистить всё
settings-controls-endless-jog-encoders-send-relative = Бесконечные (джог) энкодеры посылают относительный шаг, а не позицию, в одной из трёх кодировок, которые похожи при малых движениях. LEARN угадывает по повороту по часовой стрелке; если ручка тогда настраивает в неверную сторону, отметьте «rev».
settings-controls-none = — нет —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = Системное по умолчанию
settings-general-remote-access = Удалённый доступ
settings-general-what-remote-client-browser-page = Что удалённый клиент — страница браузера или другой sdroxide, запущенный с --connect — должен предъявить, прежде чем эта станция позволит им управлять. Применяется в серверном режиме (--server); изменение учитывается при следующем входе, без перезапуска.
settings-general-username = Имя пользователя
settings-general-password = Пароль
settings-general-clients-must-give-password-leaving = Клиенты должны ввести пароль. Оставить имя пользователя пустым можно.
settings-general-clients-must-sign = Клиенты должны войти.
settings-general-empty-anyone-who-can-reach = ⚠ Пусто: любой, кто может достучаться до порта сервера, может управлять этим радио от вашего позывного. Задайте пароль, прежде чем пробрасывать порт.
settings-general-stored-clear-config-toml-like = Хранится открытым текстом в config.toml, как и другие пароли, которые хранит sdroxide.
settings-general-receive-audio-gain = Усиление звука приёма
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    Дополнительное усиление всего, что это радио посылает в динамики, поверх регулятора громкости. Оставьте 0 dB, если радио тихое на полной громкости: верхняя граница шкалы громкости — это звук в том виде, как он пришёл, поэтому она может сделать радио тише, но не громче, а USB-звуковой выход некоторых трансиверов сидит заметно ниже полной шкалы.

    Go up 6 dB at a time. Too much clips — the audio is limited at full scale rather than allowed to wrap round, so overdoing it sounds harsh rather than loud. Recordings are taken ahead of this and are not affected.
settings-general-reload-band-plan = ПЕРЕЧИТАТЬ ПЛАН ДИАПАЗОНОВ
settings-general-re-read-bandplan-json-machine = Перечитать bandplan.json на машине, к которой подключено радио, и применить — границы диапазонов, подсегменты и окна скиммеров — без перезапуска.
settings-general-running-station-s-own-band = Работа по собственному плану диапазонов станции.
settings-general-running-built-iaru-tables = Работа по встроенным таблицам IARU.
settings-general-settings-file = Файл настроек
settings-general-settings-are-machine-radio-attached = Настройки находятся на машине, к которой подключено радио. Экспортируйте их там.
settings-general-export = ЭКСПОРТ…
settings-general-write-every-setting-station-radios = Записать каждую настройку этой станции — радио, режимы, серверы, память, план диапазонов — в один файл, который можно перенести на другую установку. Журнал и любой сохранённый пароль сервера остаются здесь.
settings-general-import = ИМПОРТ…
settings-general-replace-station-s-settings-ones = Заменить настройки этой станции теми, что в файле, экспортированном с другой установки. После этого перезапустите sdroxide.
settings-general-import-overwrites-what-here-file = Импорт перезаписывает то, что здесь есть, файл за файлом, и вступает в силу при следующем запуске sdroxide — иначе настройки в памяти записали бы поверх него обратно. Всё, чего файл не упоминает, остаётся как есть, поэтому связка от станции с одним радио не удалит второе радио здесь. Журнал никогда не входит в файл: экспортируйте его как ADIF из окна LOG, если хотите перенести и его.
settings-general-sdroxide-settings = Настройки sdroxide
settings-general-swr-guard = Защита по КСВ
settings-general-stop-transmitting-high-swr = Прекращать передачу при высоком КСВ
settings-general-trip = Срабатывать при
settings-general-stops-transmission-when-radio-reports =
    Прекращает передачу, когда радио сообщает КСВ на этом значении или выше, и блокирует передачу, пока вы это не подтвердите. Ловит отключённую антенну, неисправный фидер или переключатель, оставленный на неверном порту.

    Tuning is treated differently, because feeding a mismatch is the point of it: an antenna tuner gets double the limit and about five seconds before the guard applies at all. A manual tuner that takes longer than that wants the guard switched off for the session.

    Needs a rig that reports SWR over CAT. Ignores the first fifth of a second of each transmission, and does not wait for high power.
settings-general-per-mode-settings = Настройки по режимам
settings-general-reset-every-mode = СБРОСИТЬ ВСЕ РЕЖИМЫ
settings-general-forget-every-mode-s-remembered = Забыть запомненные AGC, шумоподавитель, шумоподавление, режекторный фильтр и переключатели стерео каждого режима и вернуть собственные значения по умолчанию каждого режима.
settings-general-selecting-mode-lays-mode-s = Выбор режима задаёт в приёмнике собственные стартовые значения этого режима для AGC, шумоподавителя, шумоподавления, режекторного фильтра и переключателей стерео, а изменение одного запоминает его только для этого режима. Значок круговой стрелки в конце строки фильтра/шумов приёмника показывает, что было изменено, и возвращает текущий режим.
settings-general-channels = Каналы

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = Оператор
settings-net-callsign-grid-are-set-general = Позывной и квадрат задаются на вкладке General.
settings-net-set-your-callsign-grid-general = ⚠ Задайте позывной и квадрат на вкладке General.
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = Включить
settings-net-connects-whenever-enabled-your-station = Подключается, когда включено. Ваша станция видна другим, только пока радио в RADE — в любом другом режиме вы остаётесь подключены, но скрыты.
settings-net-station = Станция
settings-net-message = Сообщение
settings-net-receive-only-i-cannot-transmit = Только приём (я не могу передавать)
settings-net-server = Сервер
settings-net-host = Хост
settings-net-port = Порт
settings-net-tls-wss = TLS (wss://)
settings-net-not-yet-implemented-freedv-gui = Пока не реализовано — FreeDV GUI тоже использует обычный ws://.
settings-net-reporting = Сообщения
settings-net-report-stations-i-decode = Сообщать о станциях, которые я декодирую
settings-net-sends-rx-report-each-callsign = Отправляет rx_report для каждого позывного, извлечённого из кадра конца передачи RADE.
settings-net-show-other-reporter-stations-spots = Показывать станции других репортёров как споты
settings-net-adds-them-panadapter-overlay-world = Добавляет их на наложение панорамы, карту мира и окно SPOTS под фильтром FREEDV.
settings-net-set-your-callsign-grid-general-2 = ⚠ Задайте позывной и квадрат на вкладке General. Без обоих соединение только для просмотра: вы увидите другие станции, но сами не появитесь.
settings-net-apply = {" "}ПРИМЕНИТЬ{" "}
settings-net-persist-re-connect = Сохранить и (пере)подключить
settings-net-season = Сезон
settings-net-downloading = загрузка…
settings-net-using-built-copy = используется встроенная копия
settings-net-your-own-stations-corrections-go = Свои станции и поправки вносятся в этот файл. sdroxide его никогда не записывает, а запись здесь заменяет запланированную с тем же именем и частотой.
settings-net-reload = Перечитать
settings-net-re-read-your-station-file = Перечитать файл станций после редактирования
settings-net-broadcast-stations-labelled-waterfall-come = Станции вещания, подписанные на водопаде, берутся из списка, встроенного в эту сборку. Редактирование требует настольного приложения.
settings-net-ask-service-whether-these-credentials = Спросить у службы, работают ли эти учётные данные. Сначала применяет настройки выше и ничего не записывает: только читает.
settings-net-checking = проверка…

## Settings window: messages with values filled in.

settings-amp-connected-to = Подключено к { $model }
settings-controls-last-midi = Последнее сообщение: { $message }  значение { $value }
settings-general-band-plan-file = Каждая граница диапазона и подсегмент берутся из { $path }; файл записывается из встроенных таблиц IARU при первом запуске и дальше принадлежит вам для редактирования — сузьте диапазон до своей лицензии, и sdroxide откажется передавать за его пределами. Частоты в МГц; удалите файл, чтобы получить свежую копию значений по умолчанию. Это региональное распределение, а не ваша лицензия: ваши собственные условия могут быть уже, а национальные планы внутри региона различаются.
settings-general-exported = Экспортировано: { $note } — выберите, куда сохранить.
settings-general-export-failed = Ошибка экспорта: { $error }
settings-general-import-failed = Ошибка импорта: { $error }
settings-general-imported = { $summary } — перезапустите sdroxide, чтобы их использовать.
settings-general-import-skipped = Пропущено { $path }: { $why }
settings-general-swr-tripped = ⚠ Сейчас срабатывает при { $swr }:1 — передача заблокирована.
settings-net-call-grid-set-on-general = { $call } / { $grid } — задаётся на вкладке General
settings-net-reporting-as = Сообщается как { $call } / { $grid } — SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ Ошибка загрузки: { $error }. Используется встроенное расписание.
settings-net-test-service = Проверить { $service }

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = Передача выключена, пока задан конвертер — опишите, что стоит в тракте передачи выше, чтобы включить её снова.
settings-transmit-goes-through-same-converter = Передача идёт через тот же конвертер, поэтому радио работает с одинаковым смещением от шкалы в обе стороны.
settings-transmit-not-converted-radio-transmits = Передача не преобразуется: радио передаёт на частоте, которую показывает шкала, а приём идёт через конвертер.
settings-transmit-takes-own-offset-separate = Передача берёт собственное смещение, отдельное от приёмного.
settings-no-enabled-subscriptions-update = Нет включённых подписок для обновления.
settings-add-radio-here-station-you = Добавить радио — здесь или на станции, к которой вы подключены

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = Каждая граница диапазона и подсегмент берутся из bandplan.json на машине, к которой подключено радио. Это региональное распределение, а не ваша лицензия: ваши собственные условия могут быть уже, а национальные планы внутри региона различаются.

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = Полученные изображения → FTP
settings-picup-uploads-every-received-sstv-picture = Выгружает каждое принятое изображение SSTV (и, если отмечено, каждую метеокарту) по мере сохранения, оформляя его так, как это делает QSSTV, чтобы веб-страницы, написанные для QSSTV, работали без изменений.
settings-picup-upload-received-pictures = Выгружать полученные изображения
settings-picup-server = Сервер
settings-picup-port = Порт
settings-picup-security = Безопасность
settings-picup-user = Пользователь
settings-picup-password = Пароль
settings-picup-certificate = Сертификат
settings-picup-trusted-when-computer-s-operating = Доверяется, когда операционная система этого компьютера доверяет ему, как доверял бы браузер здесь. Частный CA можно добавить ниже (PEM).
settings-picup-extra-ca = Дополнительный CA
settings-picup-only-certificate-fingerprint-accepted-way = Принимается только сертификат с этим отпечатком — способ доверять самоподписанному серверу. TEST показывает отпечаток сервера.
settings-picup-any-certificate-accepted-link-encrypted = ⚠ Принимается любой сертификат. Соединение зашифровано, но любой на пути может выдать себя за сервер и прочитать пароль.
settings-picup-plain-ftp-sends-password-pictures = ⚠ Обычный FTP передаёт пароль и изображения незашифрованными.
settings-picup-sstv = SSTV
settings-picup-sstv-dir = Каталог SSTV
settings-picup-weather-fax = Метеофакс
settings-picup-wefax-dir = Каталог WEFAX
settings-picup-empty-login-directory = Пусто — это домашний каталог входа.
settings-picup-file-names = Имена файлов
settings-picup-format = Формат
settings-picup-add-extension = добавлять расширение
settings-picup-image1-png-rather-than-qsstv = image1.png, а не голый image1 у QSSTV
settings-picup-caption-bar = Строка подписи
settings-picup-time-frequency-mode-sender-s = Время, частота, режим и FSK ID отправителя вверху, как это рисует QSSTV. Отмечается только выгруженная копия.
settings-picup-fit-960-768 = Вписать 960 × 768
settings-picup-qsstv-s-heartbeat-tiny-picture = «Сердцебиение» QSSTV: крохотное изображение, высота которого — минута, выгружается рядом с каждым, а при закрытии программы становится два пикселя шириной.
settings-picup-nat-workaround = Обход NAT
settings-picup-ignore-private-address-server-s = Игнорировать частный адрес в ответе сервера в пассивном режиме и использовать собственный адрес сервера — для сервера за NAT, который не знает своего публичного адреса.
settings-picup-apply = {" "}ПРИМЕНИТЬ{" "}
settings-picup-test = TEST
settings-picup-connect-settings-above-applied-not = Подключиться с настройками выше — применены они или нет — войти, и записать и удалить небольшой файл в каждом каталоге выгрузки.
settings-picup-checking = проверка…
settings-picup-trust-certificate = Доверять этому сертификату
settings-picup-pin-switch-pinned-fingerprint-one = Закрепите: переключитесь на «Закреплённый отпечаток» с этим. Сначала сверьте его с сервером — это то, что сказал сервер, а не доказательство того, кто он. Затем ПРИМЕНИТЬ.
settings-picup-last-upload = Последняя выгрузка

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = Профили: сохранённые настройки станции
settings-profiles-profile-captures-whole-working-setup = Профиль фиксирует всю рабочую настройку — шкалы и VFO, режим и фильтры, усиления, мощность и антенны, ваш позывной и шаблоны сообщений, а также регистры по диапазонам — и возвращает её одним нажатием. Аппаратура (бэкенд, звуковые устройства, конвертеры) намеренно не затрагивается: профиль — это способ работать на станции, а не радио, на котором она работает.
settings-profiles-save-current-setup = Сохранить текущую настройку
settings-profiles-profile-name = имя профиля
settings-profiles-save = Сохранить
settings-profiles-applies-whatever-sdroxide-attached-dials = Применяется к тому, к чему подключён sdroxide: шкалы и VFO, режим и фильтры, уровни, усиления, мощность, антенны, цифровой позывной и стеки диапазонов.
settings-profiles-no-profiles-saved-yet-name = Профили пока не сохранены — назовите один выше и нажмите «Сохранить».
settings-profiles-saved-profiles = Сохранённые профили
settings-profiles-apply = Применить

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = Переключатель передача/приём
settings-relay-waiting-station-s-t-r = Ожидание конфигурации переключателя T/R станции…
settings-relay-closes-contact-while-station-transmits = Замыкает контакт, пока станция передаёт — чтобы отключить и заземлить вход антенны SDR, а также чтобы последовательно с ним манипулировать усилителем или внешним реле T/R. Работает с дешёвыми USB-платами реле (LCUS, KMtronic, Numato), с линией RTS/DTR последовательного порта в любом интерфейсе, которому нужно замыкание PTT, и с чем угодно ещё через командный хук.
settings-relay-hardware = Аппаратура
settings-relay-board = Плата
settings-relay-lcus-four-byte-a0-command = LCUS — плата с четырёхбайтовыми командами A0, продаваемая под десятком названий; у KMtronic и Numato можно также спросить, в каком они состоянии
settings-relay-port = Порт
settings-relay-choose = — выберите —
settings-relay-device = Устройство
settings-relay-gpio-pin = Вывод GPIO
settings-relay-pin-3-every-homebrew-plan = Вывод 3 в каждой самодельной схеме и на каждой промышленной плате — тот, что на конце корпуса, к которому можно прицепить провод. Если реле не щёлкает, плата — клон, который вообще не реализует эти выводы.
settings-relay-gpio-chip = Микросхема GPIO
settings-relay-linux-only-gpiodetect-lists-chips = Только Linux. `gpiodetect` перечисляет микросхемы на машине.
settings-relay-transmit = При передаче
settings-relay-receive = При приёме
settings-relay-nothing-switched = Ничего не переключается.
settings-relay-starting-program-costs-unpredictable-amount = Запуск программы занимает непредсказуемое время, и ничто из этого не учитывается в опережении — поэтому оставляйте здесь больший запас, чем для платы реле. Цифры таблицы контактов для этой связи ничего не значат: ваши команды запускаются один раз при нажатии и один раз при отпускании.
settings-relay-contacts = Контакты
settings-relay-contact-1-rts-contact-2 = Контакт 1 — это RTS, контакт 2 — DTR.
settings-relay-no = №
settings-relay-gpio = GPIO
settings-relay-name = Название
settings-relay-job = Задача
settings-relay-tx-closes = Замыкается при передаче
settings-relay-lead = Опережение
settings-relay-hold = Удержание
settings-relay-line-s-offset-chip-bcm = Смещение линии на этой микросхеме — номер BCM на разъёме Raspberry Pi, который не совпадает с номером физического контакта
settings-relay-sdr-antenna = Антенна SDR
settings-relay-decides-default-timings-what-log = Определяет тайминги по умолчанию и то, как журнал это называет — и, для декодера диапазонов, что контакт следует таблице диапазонов ниже
settings-relay-transmitting-energises-coil-off-transmitting =
    Вкл.: передача запитывает катушку. Выкл.: передача её отпускает.

    This is a wiring decision before it is a setting — see the note below.
settings-relay-how-long-before-rf-contact = За сколько до ВЧ этот контакт переходит из слова RX своего диапазона в слово TX. Перестройка переводит его сразу; это для нажатия, когда фильтр только передачи должен быть включён до того, как появится мощность.
settings-relay-closed-long-before-rf-small = Замкнут за столько до ВЧ. Небольшое коаксиальное реле срабатывает за 5–15 мс. Передача ждёт самого длинного из них, поэтому большое значение — слышимый провал в звуке приёма; всё, что больше 250 мс, игнорируется.
settings-relay-how-long-after-rf-stops = Через сколько после прекращения ВЧ этот контакт возвращается из слова TX в слово RX.
settings-relay-opened-long-after-rf-stops = Размыкается через столько после прекращения ВЧ. Намеренно дольше опережения: поздний возврат антенны на мгновение ничего не стоит, а ранний — стоит фронтенда. Ноль — допустимый ответ и правильный для линии ключа усилителя.
settings-relay-test = {" "}TEST{" "}
settings-relay-not-while-station-air = Не во время работы станции в эфире
settings-relay-close-contact-half-second-so = Замкнуть этот контакт на полсекунды, чтобы услышать реле и проверить монтаж при холодном передатчике
settings-relay-remove = Удалить
settings-relay-add-contact = + добавить контакт
settings-relay-if-will-not-answer = Если не отвечает
settings-relay-switch-exists-protect-receiver-not = Переключатель, который существует, чтобы защищать приёмник, не стоит иметь, если он молча перестаёт его защищать — поэтому отказ — по умолчанию. Выбирайте иное там, где плохо то, что USB-кабель, выпавший из контеста, — хуже.
settings-relay-order-timings-longest-lead-closes = Порядок задают тайминги: самое длинное опережение замыкается первым, а самое длинное удержание размыкается последним. Поэтому дайте реле антенны более длинное опережение и более длинное удержание, а усилителю — более короткие из каждого — тогда антенна всегда переключается до того, как усилитель получает ключ, и усилитель всегда снимается с ключа до того, как антенна возвращается.
settings-relay-transmit-sense-input = Вход контроля передачи
settings-relay-when-transceiver-keys-itself-own = Когда трансивер сам включает передачу — собственной кнопкой микрофона, педалью, VOX или манипулятором — sdroxide узнаёт об этом, спрашивая по CAT, и ответ приходит через несколько сотен миллисекунд после начала передачи. Если провести линию SEND аппарата через оптоизолятор на вход рукопожатия на этом же порту, это будет видно за несколько миллисекунд. Измеритель и блокировка передачи станут от этого быстрее.
settings-relay-line = Линия
settings-relay-transmitting = Передача —
settings-relay-high-line = высокий уровень на линии
settings-relay-most-opto-isolated-interfaces-pull = Большинство интерфейсов с оптоизоляцией тянут линию вниз, когда аппарат включает передачу, поэтому обычно это выключено
settings-relay-belongs-radio = Принадлежит радио
settings-relay-which-radio-tab-sensed-transceiver = Какой вкладке радио принадлежит контролируемый трансивер. На станции с одним радио это 1, и ничто от этого не зависит.
settings-relay-needs-serial-port-handshake-lines = Нужен последовательный порт — на нём находятся линии рукопожатия.
settings-relay-wiring-only-fail-safe-survives = Монтаж — единственная защита при сбое, которая переживает эту программу.
settings-relay-choose-tx-closes-so-de = Выбирайте «Замыкается при передаче», чтобы *обесточенный* контакт был нужным состоянием, когда плата отключена, компьютер выключен или sdroxide не запущен. При монтаже, где обесточенное означает заземлённое, мёртвое реле оставляет SDR глухим — неприятно, но безопасно. При монтаже наоборот мёртвое реле плюс одна передача — мёртвый фронтенд. С настоящим усилителем за ним выбирайте первое.
settings-relay-cannot-protect-first-instant-over = Это не может защитить первое мгновение передачи, которую вы начинаете на самом радио. Если включать аппарат из sdroxide, контакты всегда опережают ВЧ; если провести вход контроля выше, они следуют за ним в пределах миллисекунд. Для фронтенда, который действительно стоит защищать, используйте также аппаратный переключатель T/R с контролем ВЧ — никакая программа на компьютере не может быть полным ответом.
settings-relay-contacts-transmit = контакты в ПЕРЕДАЧЕ
settings-relay-not-open = Не разомкнуто.
settings-relay-status-unknown-press-apply = Состояние неизвестно — нажмите ПРИМЕНИТЬ.
settings-relay-station-air-apply-will-take = Станция в эфире — ПРИМЕНИТЬ вступит в силу после этой передачи.
settings-relay-band-decoder-outputs = Выходы декодера диапазонов
settings-relay-one-control-word-per-band = Одно управляющее слово на диапазон: бит 0 — контакт 1, бит 1 — контакт 2 и так далее, в соответствии с нумерацией выше. RX следует за диапазоном шкалы приёма; TX — за диапазоном того радио, которое включило передачу, переключается на опережении каждого контакта до ВЧ и обратно после его удержания. Дайте им одинаковое значение для фильтра, которому безразлично, в какую сторону идёт ВЧ, и разные — для обхода предусилителя только приёма или банка ФНЧ только передачи. Диапазоны, оставленные в 0, ничего не выставляют. Применяется при «Применить / переподключить».
settings-relay-band = Диапазон
settings-relay-rx = RX
settings-relay-tx = TX
settings-relay-outputs-asserted = Выставленные выходы
settings-relay-other = Прочее
settings-relay-everywhere-outside-amateur-bands-short = Везде вне любительских диапазонов — прослушивание коротких волн и всё, на что попадает шкала трансвертера, что не покрыто ни одним диапазоном.
settings-relay-apply = {" "}ПРИМЕНИТЬ{" "}
settings-relay-persist-re-open-switch = Сохранить и (пере)открыть переключатель

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = Подключиться к серверу
settings-remote-drive-sdroxide-running-somewhere-else = Управлять sdroxide, работающим в другом месте — машина в шэке, удалённая площадка — по сети. Запустите его там с --server и введите его адрес здесь. Радио, декодеры и журнал остаются на той машине; этот экран получает водопад и звук, а пользуетесь вы его динамиками и микрофоном.
settings-remote-address = Адрес
settings-remote-host-name-ip = имя хоста или IP
settings-remote-machine-running-sdroxide-server-host = Машина, на которой запущен sdroxide --server: имя хоста, адрес IPv4 или IPv6 либо полный URL ws:// (или wss://), если сервер стоит за обратным прокси — URL используется ровно так, как введён, и игнорирует порт и переключатель ниже.
settings-remote-port = Порт
settings-remote-port-server-listens-server-port = Порт, который слушает этот сервер — server_port в его config.toml. 4950, если не задан --port.
settings-remote-secure = Безопасно
settings-remote-wss-tls = wss:// (TLS)
settings-remote-tick-where-server-sits-behind = Отметьте это, если сервер стоит за чем-то, что завершает для него HTTPS — обратный прокси на порту 443, что обычный способ подключиться к станции через открытый интернет. Собственный сервер sdroxide говорит на обычном ws:// и этого не требует.
settings-remote-connect = {" "}ПОДКЛЮЧИТЬСЯ{" "}
settings-remote-open-station-radio-tab-own = Открыть эту станцию как отдельную вкладку радио
settings-remote-client-cannot-hold-second-connection = Этот клиент не может держать второе подключение.

    If the server asks for a username and password, its sign-in screen comes up in the new tab. A plain ws:// link carries none of this encrypted, so across the open internet either put it through a VPN or an SSH tunnel, or terminate HTTPS in front of the server and tick Secure above.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = Широковещание WSJT-X по UDP
settings-servers-waiting-station-s-wsjt-x = Ожидание конфигурации широковещания WSJT-X станции…
settings-servers-sends-decodes-station-status-logged = Отправляет декодирования, состояние станции и записанные QSO так, как это делает WSJT-X, чтобы GridTracker, JTAlert, N1MM+ и Log4OM работали с sdroxide без изменений. Только вывод — ничто в этом сокете не может затронуть радио.
settings-servers-enable = Включить
settings-servers-send = Отправлять на
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 достигает клиентов на этой машине; адрес LAN или multicast-группа (224.0.0.1) достигает других
settings-servers-port = Порт
settings-servers-t-2237-port-every-client-defaults = 2237 — порт, который по умолчанию использует каждый клиент
settings-servers-identify = Представляться как
settings-servers-name-clients-see-some-loggers = Имя, которое видят клиенты. Некоторые журналы принимают только трафик, представляющийся как WSJT-X.
settings-servers-n1mm-contactinfo-broadcast = Широковещание contactinfo N1MM+
settings-servers-same-news-n1mm-s-words = Те же новости на языке N1MM: одна XML-датаграмма на каждый записанный контакт, для журналов, которые слушают N1MM, а не WSJT-X — мост World Radio League принимает любой из них. Только контакты; декодирующего потока в этом протоколе нет. Собственный переключатель и собственный порт, потому что станции может понадобиться и то и другое.
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 достигает клиентов на этой машине. Совет самого N1MM для контест-сети — широковещательный адрес этой подсети: 192.168.1.255 для сети 192.168.1.n — он достигает всех рабочих мест сразу.
settings-servers-t-12060-port-n1mm-s-documentation = 12060 — порт, который рекомендует документация N1MM
settings-servers-station-name = Имя станции
settings-servers-what-n1mm-calls-stationname-name = Как N1MM называет StationName: имя компьютера, отправившего пакет. Журналы показывают его, чтобы отличить одно рабочее место от другого.
settings-servers-cw-panel-s-copy-plain = Текст панели CW как простой текст, одна UDP-датаграмма на каждый фрагмент по мере его устоявшегося состояния — для контроллера станции, окна CW журнала или скрипта. Никакого протокола: слушатель просто дописывает то, что приходит (попробуйте nc -ul 9999). Отправляется только устоявшийся текст, никогда хвост, который декодер ещё исправляет. У каждого радио свой переключатель.
settings-servers-apply = {" "}ПРИМЕНИТЬ{" "}
settings-servers-persist-re-open-broadcast-sockets = Сохранить и (пере)открыть широковещательные сокеты
settings-servers-hamlib-rigctld-server = Сервер Hamlib rigctld
settings-servers-waiting-station-s-rigctld-configuration = Ожидание конфигурации rigctld станции…
settings-servers-listen = Слушать на
settings-servers-t-127-0-0-1-machine = 127.0.0.1 (эта машина)
settings-servers-t-0-0-0-0-whole = 0.0.0.0 (вся сеть)
settings-servers-t-4532-port-every-rigctld-client = 4532 — порт, который предполагает каждый клиент rigctld
settings-servers-rig-name = Название аппарата
settings-servers-reported-clients = сообщается клиентам
settings-servers-max-clients = Макс. клиентов
settings-servers-allow-clients-transmit = Разрешить клиентам передачу
settings-servers-off-refuses-every-key-request = Выкл. отказывает на каждый запрос ключа и перестаёт объявлять диапазон передачи, поэтому сам Hamlib отказывается включать передачу.
settings-servers-rigctld-has-no-authentication-0 = ⚠ rigctld не имеет аутентификации. На 0.0.0.0 любой, кто может достучаться до этого порта, может настроить и включить ваш передатчик.
settings-servers-t-1-client = 1 клиент
settings-servers-not-running = Не запущено.
settings-servers-status-unknown-press-apply = Состояние неизвестно — нажмите ПРИМЕНИТЬ.
settings-servers-persist-re-bind-server = Сохранить и (пере)привязать сервер
settings-servers-lets-any-hamlib-capable-program = Позволяет любой программе с поддержкой Hamlib управлять этим радио: частотой, режимом, PTT, split, RIT и мощностью. В WSJT-X, fldigi или CQRLOG выберите аппарат «Hamlib NET rigctl» (модель 2) и укажите этот адрес; в GPredict и N1MM введите хост и порт напрямую. В отличие от сервера TCI, он несёт только управление — без звука и без IQ.
settings-servers-antenna-rotator = Поворотное устройство антенны
settings-servers-waiting-station-s-rotator-configuration = Ожидание конфигурации поворотного устройства станции…
settings-servers-host = Хост
settings-servers-min-elevation = Мин. возвышение
settings-servers-below-rotator-parks-instead-tracking = Ниже этого значения поворотное устройство паркуется вместо сопровождения — задайте здесь ваш местный горизонт или линию крыш
settings-servers-azimuth-offset = Смещение азимута
settings-servers-added-every-command-rotator-whose = Добавляется к каждой команде, для поворотного устройства, чей север смещён
settings-servers-min-movement = Мин. перемещение
settings-servers-steps-smaller-than-are-not = Шаги меньше этого не отправляются — погоня за десятыми долями градуса изнашивает моторы впустую
settings-servers-park-position = Положение парковки
settings-servers-drive-position-when-idle = вести в положение, когда простаивает
settings-servers-off-leaves-antenna-wherever-last = Выкл. оставляет антенну там, где закончился последний проход
settings-servers-not-connected = Не подключено.
settings-servers-waiting-station-s-tci-server = Ожидание конфигурации сервера TCI станции…
settings-servers-built-tci-server = Встроенный сервер TCI
settings-servers-device-name = Имя устройства
settings-servers-off-leaves-control-receive-streams = Выкл. оставляет управление и потоки приёма работающими, но каждый запрос ключа отклоняется.
settings-servers-tci-has-no-authentication-0 = ⚠ TCI не имеет аутентификации. На 0.0.0.0 любой, кто может достучаться до этого порта, может настроить и включить ваш передатчик.

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = Спутники: наборы элементов и частоты
settings-tle-waiting-station-s-satellite-configuration = Ожидание конфигурации спутников станции…
settings-tle-tracker-already-fetches-celestrak-s = Трекер и так сам загружает любительскую группу CelesTrak. Это для всего остального: метеоспутники NOAA, кьюбсат, слишком новый, чтобы быть в группе, или более свежий набор элементов, чем пришедший. Списки загружаются и хранятся там, где работает радиодвижок, так что это один и тот же набор спутников, где бы ни было открыто приложение.
settings-tle-subscriptions = Подписки
settings-tle-listings-fetched-kept-current-same = Списки загружаются и поддерживаются актуальными с тем же шестичасовым циклом, что и любительский набор. Обновляются, пока открыто солнечное окно, и кнопкой UPDATE NOW здесь.
settings-tle-fetch-track-listing = Загрузить и отслеживать этот список
settings-tle-remove-subscription = Удалить эту подписку
settings-tle-orbits = Орбиты
settings-tle-which-satellites-listing-get-orbit = Какие спутники из этого списка получают кольцо орбиты и подпись. Целой группе нужно «выборочно»: девяносто колец сразу нечитаемы, а ни одного — оставляют девяносто безымянных точек.
settings-tle-nothing-listing-sdroxide-s-curated = Ничего из этого списка нет в выборочном списке sdroxide — в нём десять любительских спутников, поэтому это вело бы себя точно как «ни одного».
settings-tle-all-satellites = все спутники
settings-tle-catalogue-numbers-keep-comma-separated = Каталожные номера, которые оставить, через запятую. Пусто — отслеживается всё, что есть в списке.
settings-tle-not-fetched-yet = ещё не загружено
settings-tle-subscription = + Подписка
settings-tle-update-now = {" "}ОБНОВИТЬ СЕЙЧАС{" "}
settings-tle-ask-radio-engine-fetch-every = Попросить радиодвижок сейчас загрузить каждую включённую подписку
settings-tle-celestrak-groups = Группы CelesTrak
settings-tle-pasted-element-sets = Вставленные наборы элементов
settings-tle-one-off-these-do-not = Разово. Они сами не обновляются, а SGP4 перестаёт прогнозировать набор элементов, когда он на две недели старше своей эпохи, — поэтому для всего, что хотите сохранить, подпишитесь.
settings-tle-track-one = Отслеживать этот
settings-tle-show-two-element-lines = Показать две строки элементов
settings-tle-remove = Удалить
settings-tle-paste-two-three-line-element = Вставьте сюда наборы элементов из двух или трёх строк
settings-tle-add-pasted = + Добавить вставленное
settings-tle-nothing-paste-box-looked-like = Ничто в поле вставки не похоже на набор элементов.
settings-tle-clear-box = Очистить поле
settings-tle-frequencies = Частоты
settings-tle-shown-under-pass-table-solar = Показывается под таблицей пролётов в солнечном окне. Запись здесь полностью заменяет встроенную для этого каталожного номера, поэтому начните с её копии, если не собираетесь убрать остальное.
settings-tle-remove-satellite-s-entry = Удалить запись этого спутника
settings-tle-link = СВЯЗЬ
settings-tle-downlink = ВНИЗ
settings-tle-uplink = ВВЕРХ
settings-tle-mode = РЕЖИМ
settings-tle-note = ПРИМЕЧАНИЕ
settings-tle-fm-repeater = FM-ретранслятор
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67.0 Гц
settings-tle-link-2 = + Связь
settings-tle-copy-built = Копия встроенного
settings-tle-frequency-either-one-number-145 = Частота — это либо одно число (145.800), либо полоса пропускания транспондера, записанная как 145.950-145.970. Оставьте направление пустым для маяка.
settings-tle-norad = NORAD
settings-tle-satellite = + Спутник
settings-tle-frequency-entry-needs-satellite-s = Запись частоты требует каталожного номера спутника NORAD.

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = Адрес
settings-wavelog-api-key = Ключ API
settings-wavelog-your-wavelog-s-front-page = Адрес главной страницы вашего Wavelog — https://log.example.org или http://192.168.1.10/wavelog — и ключ со страницы API. Для выгрузки нужен ключ с правом чтения и записи.
settings-wavelog-plain-http-address-internet-api = Обычный http:// на адрес в интернете: ключ API передаётся незашифрованным. Нормально в вашей собственной сети; через интернет используйте https://.
settings-wavelog-trusted-cert = Доверенный сертификат
settings-wavelog-forget-certificate = Забыть этот сертификат
settings-wavelog-go-back-ordinary-checking-against = Вернуться к обычной проверке по публичным центрам сертификации — для сервера, у которого теперь нормальный сертификат.
settings-wavelog-connection-check-runs-machine-radio = Проверка соединения выполняется на машине, к которой подключено радио — там используйте настройки, чтобы проверить его и вывести список местоположений станции.
settings-wavelog-server-presented-certificate-sha-256 = Сервер предъявил этот сертификат (SHA-256):
settings-wavelog-compare-your-server-s-e = Сравните его с сертификатом вашего сервера — например, `openssl x509 -noout -fingerprint -sha256 -in cert.pem` — и доверяйте, только если они совпадают. С этого момента принимается ровно этот сертификат, а любой другой отклоняется.
settings-wavelog-trust-certificate = Доверять этому сертификату
settings-wavelog-station-location = Местоположение станции
settings-wavelog-none = — нет —
settings-wavelog-press-test-above-list-station = Нажмите «Проверить» выше, чтобы вывести список местоположений станции, заданных в вашем Wavelog.
settings-wavelog-default = По умолчанию
settings-wavelog-where-qsos-go-when-operating = Куда идут QSO, когда рабочий профиль не называет местоположение.
settings-wavelog-profile = Этот профиль
settings-wavelog-use-default = Использовать по умолчанию
settings-wavelog-not-set = не задано
settings-wavelog-no-station-location-chosen-uploads = Местоположение станции не выбрано — выгрузки будут падать, пока оно не будет выбрано.
settings-wavelog-qsos-not-wavelog = QSO, которых нет в Wavelog
settings-wavelog-every-qso-logbook-wavelog = Каждое QSO в журнале есть в Wavelog.
settings-wavelog-everything-logbook-wavelog-has-not = Всё в журнале, что Wavelog не подтвердил — включая контакты до автовыгрузки. Те, что у него уже есть, вернутся как дубликаты и просто будут помечены как отправленные.
settings-wavelog-set-address-key-station-location = Сначала задайте адрес, ключ и местоположение станции.
settings-wavelog-yes-send-them = Да, отправить их
settings-wavelog-cancel = Отмена
settings-wavelog-download-into-logbook = Загрузить в этот журнал
settings-wavelog-downloading-runs-where-radio-attached = Загрузка выполняется там, где подключено радио, в журнал той машины — настройте его там.
settings-wavelog-download-qsos-start-every = Загружать QSO при запуске и каждые
settings-wavelog-downloaded-qsos-join-logbook-so = Загруженные QSO попадают в журнал, поэтому FT8 и списки спотов знают, с чем вы уже работали из других программ и с других радио. Каждая загрузка берёт только новое.
settings-wavelog-press-test-above-choose-which = Нажмите «Проверить» выше, чтобы выбрать, какие местоположения станции загружать.
settings-wavelog-sync-now = Синхронизировать сейчас
settings-wavelog-fetch-what-wavelog-has-added = Загрузить то, что Wavelog добавил с последней загрузки. Сначала применяет настройки выше.
settings-wavelog-full-resync = Полная пересинхронизация…
settings-wavelog-download-ticked-locations-whole-compare = Загрузить отмеченные местоположения целиком и сравнить их с журналом: QSO, изменённые или удалённые в Wavelog, либо из местоположения, которое больше не отмечено, приводятся в соответствие. Вы видите изменения, прежде чем что-либо произойдёт.
settings-wavelog-full-resync-changes-make = Полная пересинхронизация — изменения для внесения
settings-wavelog-qsos-logged-here-imported-file = QSO, записанные здесь или импортированные из файла, никогда не меняются и не удаляются — только те, что были загружены из Wavelog.
settings-wavelog-apply = Применить
settings-wavelog-discard = Отменить
settings-wavelog-live-radio-status = Текущее состояние радио
settings-wavelog-send-frequency-mode-wavelog-they = Отправлять частоту и режим в Wavelog по мере их изменения
settings-wavelog-radio-name = Название радио
settings-wavelog-pick-radio-wavelog-s-live = Выберите это радио в живом логировании Wavelog, и его форма будет следовать за шкалой. Нужен ключ с правом чтения и записи. Только с основного радио станции.

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = Отпечаток сертификата сервера SHA-256: { $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = Вернуть станцию к «{ $name }»: шкалы, VFO, режим, фильтры, усиления, мощность, антенны, позывной и стеки диапазонов.
settings-profiles-delete = Удалить «{ $name }»

## Settings window, T/R switch: messages with values

settings-relay-not-usable = Пока не годится: { $why }
settings-relay-receiving = приём
settings-relay-rx-tx-words = { $rx } / TX { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n ->
        [one] { $n } клиент
        [few] { $n } клиента
       *[other] { $n } клиентов
    }
settings-servers-listening = ● Слушает на { $addr } — { $clients }
settings-servers-failed = Ошибка: { $error }
settings-servers-rotator-at = ● Подключено — антенна на азимуте { $az }°, возвышение { $el }°

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } спутников · возраст { $age }
settings-tle-subscribed = Подписка на { $name } оформлена. Нажмите ОБНОВИТЬ СЕЙЧАС, чтобы загрузить её.
settings-tle-norad-too-stale = NORAD { $norad } · возраст { $age } — слишком старый для прогноза
settings-tle-norad-age = NORAD { $norad } · возраст { $age }
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = Добавлено { $n } { $n ->
        [one] набор элементов
        [few] набора элементов
       *[other] наборов элементов
    }.
settings-tle-added-refreshed = Добавлено { $added }, обновлено { $r } { $r ->
        [one] набор элементов
        [few] набора элементов
       *[other] наборов элементов
    }.
settings-tle-links = { $n } { $n ->
        [one] связь
        [few] связи
       *[other] связей
    }
settings-tle-replace-links = Заменить эти связи встроенными для { $name }

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = Местоположение «{ $location }» — это { $station_call }, но вы ведёте журнал как { $call } — Wavelog отклонит эти QSO.
settings-wavelog-grid-mismatch = Местоположение «{ $location }» находится в { $station_grid }, но ваш квадрат — { $grid } — Wavelog отклонит эти QSO.
settings-wavelog-location-n = местоположение #{ $id }
settings-wavelog-saved-with-callsign = Сохранено с позывным ({ $call }) в Настройки → Профили, чтобы переносной профиль мог вести журнал в переносное местоположение.
settings-wavelog-owed = { $n } { $n ->
        [one] QSO
        [few] QSO
       *[other] QSO
    }, записанных с момента включения автовыгрузки, ещё не отправлено — повтор каждые 5 минут.
settings-wavelog-send-unsent = Отправить { $n } { $n ->
        [one] QSO
        [few] QSO
       *[other] QSO
    }, которых ещё нет в Wavelog…
settings-wavelog-send-confirm = Отправить { $n } QSO в { $location }? Сначала применяются настройки.
settings-wavelog-plan-add = • { $n } QSO из Wavelog, которые нужно добавить в журнал
settings-wavelog-plan-update = • { $n } загруженных QSO, изменённых в Wavelog, для обновления
settings-wavelog-plan-remove = • { $n } загруженных QSO, которых больше нет в Wavelog (или из неотмеченного местоположения), для удаления
settings-wavelog-plan-mark = • { $n } ваших QSO, которые нужно отметить как находящиеся в Wavelog

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = Антенна
settings-radio-which-socket-back-radio-using =
    Какой разъём на задней панели использует радио — его собственная команда ANT, та же настройка, что и кнопка ANT на передней панели.

    Applies immediately, and it is the radio's setting rather than a copy kept here: the socket the radio is on is read back when the session opens.

    The choice is remembered per band. Switch to the beam on 2 m and the wire on 40, and each comes back the next time the dial crosses into that band — and the next time sdroxide starts. A memory channel stored here carries its socket too.
settings-radio-receive-antenna = Антенна приёма
settings-radio-radio-s-separate-receiving-antenna =
    Отдельная приёмная антенна радио — RX ANT на задней панели — подключается к тракту приёма или отключается от него. Собственная настройка, та же, что и кнопка RX ANT радио.

    The aerial on the main socket stays on transmit either way: this is an extra input for receiving, a loop or a beverage, not a choice of socket.

    Unlike the antenna above, this is *not* remembered here. The radio holds it per band itself, so sdroxide reads it back after every band change and shows what the radio says; ticking this box is the only thing that moves it.
settings-radio-receive-path = В тракте приёма
settings-radio-radio-power = Питание радио
settings-radio-switch-radio-itself-off-back =
    Выключите само радио и снова включите его по каналу управления. Это единственное настоящее включение и выключение в программе — не переключатель LINK в sdroxide, который закрывает конец sdroxide и оставляет радио работающим.

    For the switch back on to reach anything, the radio's control end has to stay awake while it is off. Over the network that is what Network Control does. Over a serial cable it is the radio's CI-V port, which stays powered from the mains supply on a set that is switched off at the front rather than unplugged; sdroxide sends the wake-up run Icom's own documentation asks for in front of the power-on.

    Switching off ends the audio and the meters — there is no radio behind them — and sdroxide keeps the control link open so the switch back on has somewhere to go.
settings-radio-on = Вкл.
settings-radio-off = Выкл.
settings-radio-waiting-configuration-machine-radio-attached = Ожидание конфигурации машины, к которой подключено радио.
settings-radio-sound-format = Формат звука
settings-radio-invert-spectrum = Инвертировать спектр
settings-radio-swap-i-q = Поменять I/Q местами
settings-radio-mirror-panadapter-about-tuned-frequency =
    Отразить панораму относительно настроенной частоты — для радио, у которого I и Q на звуковой карте идут в обратном порядке. Признак — водопад, полный убедительных сигналов, которые все с неправильной стороны шкалы, и SSB, выходящая на противоположной боковой полосе; перестановка двух кабелей на звуковой карте исправила бы то же самое.

    Receive only: transmit hands the radio one real audio signal, which has no sideband to invert.
settings-radio-i-q-sample-rate = Частота дискретизации I/Q
settings-radio-how-fast-radio-s-i =
    С какой скоростью работает звуковая карта I/Q радио — а значит, какой диапазон показывает панорама, потому что квадратурный поток охватывает всю свою частоту дискретизации: 48 кГц даёт ±24 кГц по обе стороны от шкалы, 192 кГц — ±96.

    The card has the final say. One that cannot do the rate picked here is opened at the nearest it can, and the panadapter is that much narrower — the log says so at startup when it happens.

    A faster card is also more work: the machine has proportionally less time to empty it, and one that cannot keep up drops samples. That shows up as audio breaking up while the waterfall still looks perfect, and it too is logged. If you see it, come back down a step.
settings-radio-t-48-khz-24-khz = 48 кГц (±24 кГц)
settings-radio-t-96-khz-48-khz = 96 кГц (±48 кГц)
settings-radio-t-192-khz-96-khz = 192 кГц (±96 кГц)
settings-radio-t-384-khz-192-khz = 384 кГц (±192 кГц)
settings-radio-i-q-centre-offset = Смещение центра I/Q
settings-radio-how-far-radio-s-i =
    На сколько выход I/Q радио центрирован выше собственной шкалы — для аппарата, у которого промежуточная частота приёма сдвинута с нуля: на Elecraft MENU:RX SHFT, установленный в 8.0 вместо NOR, что уводит шкалу от DC-пика смесителя (и перестаёт детектировать как AM сильную соседнюю SSB/AM-станцию).

    Not a converter: the radio still displays and transmits on the real frequency, and nothing here is ever sent to it. This only says where the samples on the sound card sit, and the stream is shifted back onto the dial as it arrives.

    Leave at 0 unless you have turned such a menu entry on. If signals land twice the offset away, the sign is the other way round.
settings-radio-iq-correction = Коррекция IQ
settings-radio-cancel-mirror-image-every-signal =
    Подавляет зеркальное изображение каждого сигнала и DC-пик в середине водопада.

    A radio's I/Q output is two analogue paths that are never quite equal in gain nor exactly 90° apart, and what that leaves is a copy of every signal reflected about the tuned frequency, usually 30-40 dB down — strong enough to look like a station that is not there, and to be decoded as one. Radios with front-panel balance trimmers are adjusting exactly this; here it is measured off the received noise and needs no setting.

    Leave it on. Turn it off if you are listening to AM tuned dead on the carrier — the carrier is DC, so it goes with the spike — or to check whether a signal is real by watching whether it survives.
settings-radio-cancel-mirror-images = Подавлять зеркальные изображения
settings-radio-dc-notch = Режекция DC
settings-radio-widen-hole-taken-out-middle =
    Расширить провал, вырезанный из середины полосы, для радио, у которого центральный пик шире смещения под ним.

    0 leaves the ordinary blocker, which is a few tens of hertz wide and removes the offset without touching anything else. Wind it up and the bottom of the span goes with it: a first-order high-pass, 3 dB down at the figure set here and further in below it.

    It is centred where the radio's I/Q is centred, which is the dial unless the offset above has moved it — so anything tuned there goes too. A CW note at 600 Hz is inside a 600 Hz setting.
settings-radio-panadapter-bw = Полоса панорамы
settings-radio-cat-family = Семейство CAT
settings-radio-rigctld-address = Адрес rigctld
settings-radio-host-port-running-hamlib-rigctld =
    хост:порт работающего Hamlib rigctld — 127.0.0.1:4532 — его собственный порт по умолчанию на этой машине.

    Start one with, for example, `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, where -m is the Hamlib model number for your radio (`rigctl -l` lists them).

    This is the catch-all: it reaches the frequency, mode, PTT, power, S-meter and SWR and nothing else. Where one of the native families above fits your radio it does more — keying from the rig's own text buffer, the receive filter, and per-model meter scales.
settings-radio-flrig-address = Адрес flrig
settings-radio-host-port-running-flrig-127 =
    хост:порт работающего flrig — 127.0.0.1:12345 — его собственный порт по умолчанию на этой машине. flrig отдаёт XML-RPC, пока работает; порт указан в его Config → Setup → Server.

    Like the Hamlib option this drives a daemon rather than the radio, but through flrig's own per-model driver — on a number of rigs its power and filter handling is the more faithful of the two. It reaches the frequency, mode, PTT, transmit power (in whole watts), the receive bandwidth, the S-meter, SWR and power-out. CW keys through flrig's own cwio port, which must be configured in flrig itself.
settings-radio-serial-port = Последовательный порт
settings-radio-select = — выберите —
settings-radio-baud = Скорость
settings-radio-data-bits = Биты данных
settings-radio-parity = Чётность
settings-radio-stop-bits = Стоп-биты
settings-radio-force-rts = Принудительно RTS
settings-radio-force-dtr = Принудительно DTR
settings-radio-ptt-method = Способ PTT
settings-radio-how-transmit-keyed = Как включается передача.
settings-radio-how-transmit-keyed-network-link = Как включается передача. Сетевая связь не имеет управляющих линий, поэтому DTR и RTS здесь ничего не манипулируют — используйте CAT, который просит демон включить радио, или VOX.
settings-radio-mode-control = Управление режимом
settings-radio-digimode-mode = Режим цифрового режима
settings-radio-what-put-radio-mode-sdroxide =
    В какой режим переводить радио для режима, который sdroxide модулирует через звуковую карту. «DIGI» выбирает положение DATA/PKT аппарата, которое берёт звук передачи со входа USB или ACC, с отключённой обработкой речи тракта микрофона; «USB» оставляет обычную боковую полосу — для радио, уже настроенного модулировать SSB со своего входа данных, или для радио без положения DATA вовсе.

    It covers every digital mode — FT8, FT4, PSK, RTTY, SSTV and the rest — and CW sent as "Sound card", and it overrides Mode control for them. SSTV is the one whose sideband follows the band, so there it means the DATA position on that sideband: USB-D above 40 m, LSB-D at and below it. SSTV-FM is not part of it — an FM carrier has no sideband to choose.
settings-radio-cw-keying = Манипуляция CW
settings-radio-how-cw-panel-s-keyer =
    Как манипулятор панели CW передаёт. «Манипулятор аппарата» переводит радио в CW и отдаёт ему текст для отправки собственным манипулятором. Он использует скорость манипулятора аппарата (задаётся из WPM панели), а на Yaesu отправляет через ячейку памяти манипулятора 1, перезаписывая то, что в ней хранилось.

    It needs break-in on at the radio, which sdroxide turns on itself with each message on Icom, Yaesu and Kenwood — a rig with break-in off takes the message and transmits nothing. It is left on afterwards, and a full break-in is never turned down to semi.

    "Sound card" sends the keyed sidetone as audio instead (MCW), a tone at dial + pitch — and because a rig in CW would ignore its sound card entirely, selecting CW then follows the Digimode mode setting (USB, DATA, or Radio controlled) instead of switching the rig to CW. Pick "Radio controlled" there for rigs whose data position can't be commanded over CAT (a Xiegu's U-D) and park the rig on it yourself, as for FT8.
settings-radio-poll-rate = Частота опроса
settings-radio-how-often-radio-asked-what =
    Как часто у радио спрашивают, что оно делает — его шкалу, режим и измерители. Это половина канала управления, которая идёт от радио обратно к sdroxide: поверните собственную ручку VFO аппарата или смените режим на передней панели, и индикатор, диапазон и панорама последуют в пределах одного опроса.

    It is also the whole of the control traffic this end generates, and on a modern Icom that traffic is not free: the CI-V port and the sound card sit behind one USB hub inside the radio, so frames asked for here are bus time the audio does not get — heard as dropouts that look like a DSP fault and are not one. The default of 2 Hz is half a second behind the knob and quiet enough to stay out of the audio's way. Raise it where the control port is its own device (a separate USB-serial adapter, a network rigctld); lower it if the radio's audio breaks up.

    An Icom with CI-V Transceive switched on in its own menu needs none of this for the dial: it reports the knob the instant it moves, and sdroxide stands the dial poll down as soon as it sees it do so — and picks it back up again if the radio ever moves without saying so.

    The rate is the dial's. The mode rides along with only every fourth poll, since it is a setting that changes a few times in an evening rather than one that follows a knob.
settings-radio-send-command = Команда передачи
settings-radio-which-transceiver-generation-keys-rig =
    Какое поколение трансиверов манипулирует аппаратом для способа PTT «CAT». Два поколения расходятся в том, что значит параметр TX, и по проводу их ничто не отличает, поэтому выберите тот, что соответствует вашему радио.

    "TS-2000 style (TX;)" — TS-480, TS-570, TS-870, TS-2000: the ordinary send, on the main band.

    "TS-590 style (TX1;)" — TS-590S/SG, TS-890, TS-990: DATA SEND, which keys with the ACC2/USB audio input live. On these rigs the plain send selects the microphone instead and mutes the audio sdroxide transmits.

    Set wrong, a TS-590 transmits silence — but a TS-2000 transmits on the sub-band, which is another band entirely.
settings-radio-radio = Радио
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    Один профиль покрывает семейство: набор команд K3, на который отвечают KX3, KX2 и K4. Здесь ничего выбирать не нужно — сколько ватт охватывает ползунок Drive (12 на голом KX2 или KX3, 110 с KPA3 или KXPA100), считывается из собственного запроса модулей аппарата при открытии порта.

    Note the baud rates above: the K3, K3S, KX3 and KX2 go no faster than 38400, and a rig set below the rate chosen here answers nothing at all.
settings-radio-transmit-input = Вход передачи
settings-radio-where-radio-takes-transmit-audio =
    Откуда радио берёт звук передачи — команда TI аппарата, которая на передней панели находится в меню 32 «TX IN». Выставляется при открытии порта.

    "USB audio" is what makes this interface work: the FDM-DUO transmits what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.

    "Auto" lets the rig choose — the microphone for a PTT press on the microphone, the USB port for a CAT or RTS key-down.

    "Leave as set on the radio" sends no TI at all, for an operator who sets this at the front panel.
settings-radio-which-two-sockets-back-receiver =
    Какой из двух разъёмов на задней панели слушает приёмник — команда AN аппарата, которая на передней панели находится в меню 31 «ANTENNAS» и отображается индикатором «ANT 1 2» на дисплее.

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    Applies immediately, and is read back from the radio when the port opens: this is the rig's own setting rather than a copy kept here.
settings-radio-one-profile-covers-both-there =
    Один профиль покрывает оба. Здесь ничего выбирать не нужно — радио называет себя при открытии порта.

    This is the CAT half only: it drives the dial, the mode, PTT, the S-meter, the SWR and the transmit power over the rig's CAT USB port, and takes audio from its USB Audio port. The FDM-DUO's third USB port carries wideband I/Q, which this interface does not read — select the ELAD interface above for that.

    Note the baud rate: menu 70 "CAT BAUD" on the radio must match, and it ships at 38400.

    CW is keyed by the radio's own key or paddle. The FDM-DUO has no command that accepts text, so the CW panel cannot key it over CAT.
settings-radio-one-profile-covers-range-qrp =
    Один профиль покрывает линейку: собственный набор команд QRP Labs, который является подмножеством Kenwood TS-480 с заметным количеством добавлений. Здесь ничего выбирать не нужно — радио называет себя и версию прошивки при открытии порта, и именно версия решает, запрашивается ли вообще чтение защиты по КСВ.

    Not the Kenwood profile: on a QMX the PC command is the power METER rather than the power control, so a radio driven as a Kenwood would have its meter read written to as if it were a setting, and MD8 — a Kenwood mode — is SWR Tune here.

    The transmit power is set at the radio. There is no CAT command for it, so the Drive slider only reaches the level of the audio going into the sound card.

    The receive filter is the radio's too: it reports the width its mode implies (3.2 kHz in Digi, 300 Hz in CW) and offers nothing to change it with.
settings-radio-baud-rate-ignored-port-usb = Скорость игнорируется — порт USB
settings-radio-qmx-serves-own-virtual-com =
    QMX отдаёт собственные виртуальные COM-порты по USB, поэтому заданная выше скорость не действует ни на один конец. Их до трёх; если радио ничего не отвечает, наиболее вероятна неверная причина.

    One thing does matter about the port: a carriage return sent to it switches the radio into terminal mode for the rest of the session. sdroxide never sends one — but a terminal program left open on the same port will, and CAT stops working the moment it does.
settings-radio-i-q-mode-switched-radio = Режим I/Q включается на самом радио
settings-radio-radio-s-sound-card-carries =
    Звуковая карта радио несёт либо демодулированный звук, либо сырой I/Q, который видит её АЦП, и sdroxide выставляет то, что просит «Формат звука» выше, при открытии порта (команда Q9 радио, пункт меню «IQ mode»).

    The centre offset and sample rate above have been set to what a QMX needs: its receiver is a superhet with a 12 kHz I.F., so the synthesiser sits 12 kHz below the dial and everything on the card is 12 kHz above the middle of the span. The card runs at 48 kHz, which makes the panadapter 48 kHz wide.

    In CW the radio moves that I.F. by a further ~700 Hz, which one figure cannot follow — add it by hand if you run the panadapter with the radio in CW. In Digi, where a radio used as an I/Q front end normally sits, there is nothing to add.

    QRP Labs note that I/Q mode is not suitable for WSJT-X and other programs that expect demodulated audio; here the demodulation happens on this side, so it is.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ (КВ-трансивер 5 Вт)
settings-radio-hobbypcb-s-rs-hfiq-there =
    RS-HFIQ от HobbyPCB. Здесь ничего выбирать не нужно: профиль — это весь набор команд радио, а версия прошивки записывается в журнал при открытии порта.

    The control link carries two things and no more — where to put the oscillator, and whether to transmit. There is no mode, power, filter, squelch or meter command, because the radio has none of them. Everything else is sdroxide's: what comes off the sound card is complex baseband centred on the dial, so the mode, the filter and the modulation all happen on this side.

    The frequency command covers 3–30 MHz. Ask for anything outside that and the radio refuses it and stays where it is; the log says so.

    CW is keyed as audio through the transmit chain — this radio has no keyer that takes text. (It has an internal CW generator; sdroxide never uses it, on the firmware's own advice.)
settings-radio-sound-format-baud-ptt-are = Формат звука, скорость и PTT — параметры радио, а не настройки
settings-radio-all-three-have-been-filled =
    Все три заполнены, и ни один не является предпочтением:

    • Sound format is I/Q (stereo) — the card carries complex baseband and nothing else.
    • 57600 baud, 8N1 — the firmware's one rate, with no menu to change it. Any other is a silent link, so it is pinned when the port opens.
    • PTT over CAT — the *X command is the only transmit switch the interface has.

    The centre offset has been set to zero: the synthesiser runs at four times the dial into a quadrature detector, so the middle of the span is the dial. Set the sample rate above to whatever your sound card is actually running at; that is what makes the panadapter as wide as it is.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    Карманный QRP-трансивер DL2MAN/PE1NNZ и открытая прошивка uSDX, из которой он вырос. Он эмулирует Kenwood TS-480, но подмножество скудное: он отвечает только на шкалу, режим, PTT и сброс RIT/XIT. Нет S-метра, КСВ, управления мощностью, VFO B, split и CAT-манипулятора, поэтому ползунок Drive и измерители по CAT ничего не делают — уровень аудио — единственное управление передачей.

    CW is keyed as audio (MCW) through the sideband, or with the key or paddle at the radio; there is no command that takes text.
settings-radio-audio = Аудио
settings-radio-tr-usdx-has-no-sound =
    У (tr)uSDX нет собственной звуковой карты, и есть два способа его слышать, а какой правильный, — факт о вашем шэке, а не о радио.

    "One cable" takes the audio inside the CAT serial link itself — the radio's own 8-bit stream, over the same USB cable as the control, with nothing else to plug in. The catch is the firmware: it cannot take a CAT command while its stream is running (a command sent into the stream does not pause it, it kills it), so this mode never polls. Tuning and mode changes are sent as you make them, each pausing the stream for a moment and starting it again, so expect a brief gap on each change; nothing is sent while you are simply listening. The radio's own dial and mode are not followed either — there is no read to follow them with — so drive it from here.

    "USB sound card" takes the audio from an external sound card wired to the radio's 3.5 mm speaker/mic jack, and behaves as any other CAT rig: the dial poll runs, so the radio's own knob and mode are followed. Pick the card under Radio audio below.

    Both modes open the port with DTR held high: the serial adapter's DTR is wired to the radio's reset, so opening the port reboots it and the first second or two is quiet while it comes up. DTR is never used to key.
settings-radio-radio-model = Модель радио
settings-radio-which-icom-two-things-ci =
    Какой Icom — для двух вещей, которые CI-V делает не одинаково на всех моделях.

    The transceiver address below is filled in from it: every model ships with a different one, and a frame sent to the wrong address is simply ignored — a radio that answers nothing, with no error anywhere to say why.

    It also decides whether sdroxide can select DATA mode. On CI-V, USB and USB-DATA are the same mode byte and a separate command tells them apart; without it a digital-mode over goes out through the microphone input, with the rig's speech processing and SSB filter in the path.

    "Other" leaves the address to you and sends no DATA-mode command at all.
settings-radio-radio-id-hex = ID радио (hex)
settings-radio-show-radio-s-spectrum-scope = Показывать спектроскоп радио
settings-radio-scope-span = Полоса спектроскопа
settings-radio-how-wide-sweep-radio-keeps = Насколько широко его разворачивать. Радио сохраняет полосу, последний раз выбранную на собственном экране, — часто несколько кГц. Задание полосы здесь также переводит спектроскоп в центральный режим, поэтому он следует за шкалой. Это меняет и собственный дисплей радио; «Как задано на радио» оставляет его как есть.
settings-radio-press-apply-reconnect-switch-without = Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-radio-audio-sound-card = Аудио радио (звуковая карта)
settings-radio-devices = Устройства
settings-radio-discover = Поиск
settings-radio-no-devices-press-discover = устройств нет — нажмите «Поиск»
settings-radio-manual-ip = IP вручную
settings-radio-optional-e-g-192-168 = необязательно, например 192.168.1.50
settings-radio-sample-rate = Частота дискретизации
settings-radio-receiver-ddc = Приёмник (DDC)
settings-radio-protocol-2-board-carries-several = Плата Protocol 2 несёт несколько независимо настраиваемых приёмников (DDC) на одном соединении — запустите это радио на DDC1, а другое радио с тем же адресом — на DDC2. Передатчик принадлежит радио на DDC1. Платы Protocol 1 имеют только DDC1.
settings-radio-lna-gain = Усиление LNA
settings-radio-front-end-gain-hermes-lite = Усиление фронтенда Hermes-Lite 2. Вступает в силу сразу — без переподключения — и запоминается как уровень, с которого стартует радио. Слишком высокое — АЦП ограничивает сигнал и весь диапазон выглядит искажённым; слишком низкое — приёмник глохнет.
settings-radio-overload-protection = Защита от перегрузки
settings-radio-back-lna-gain-off-itself =
    Самостоятельно снижать усиление LNA, пока плата сообщает о переполнении своего АЦП, и поднимать его обратно, когда переполнение прекращается.

    A Hermes-Lite 2 samples the whole of 0-38 MHz onto one 12-bit converter with no mixer and no preselector in front of it, so a broadcast station a band away can drive it into overflow while the band you are looking at shows nothing wrong at all — the noise floor climbs, everything intermodulates and the decoders stop. The board knows, and this is what acts on it.

    Off by default: it moves a control you set. Nothing happens while transmitting.
settings-radio-wind-gain-back-when-adc = Возвращать усиление назад, когда АЦП переполняется
settings-radio-applies-apply-reconnect-main-window = Применяется при «Применить / переподключить». Шкала Gain в главном окне показывает усиление, на котором плата реально работает, поэтому она следует за контуром, и вы можете видеть, что он делает; ползунок выше — уровень, с которого стартует радио, и контур его не переписывает.
settings-radio-how-far-gain-moves-each = На сколько дБ усиление сдвигается за раз. Один — шаг собственного регистра усиления платы.
settings-radio-attack-decay = {"  "}Атака / спад
settings-radio-how-often-gain-may-come =
    Как часто усиление может снижаться, пока конвертер переполнен, и как часто оно может подниматься обратно, когда переполнение прекращается.

    The two are deliberately a hundred times apart. Retreat immediately: every millisecond of overflow is a receiver full of intermodulation. Return slowly: whatever caused it — a neighbour keying, a broadcaster coming up at dusk — has usually not gone away, and a loop that recovered as fast as it retreated would spend the evening oscillating across the threshold. 100 ms and 10 s per decibel is what PowerSDR's Auto S-Att and N1GP's HermesIntf have both used.
settings-radio-range = {"  "}Диапазон
settings-radio-lowest-highest-gain-loop-may = Наименьшее и наибольшее усиление, которое может использовать контур, в дБ. Потолок не даёт ему решать, насколько чувствительным должен быть ваш приёмник; нижний предел — это то, где вы говорите, что ниже некоторого уровня перегрузка — чужая проблема, и ответ — фильтр, а не ещё двадцать децибел.
settings-radio-filter-board = Плата фильтров
settings-radio-accessory-board-hermes-lite-2 = Дополнительная плата на разъёме J16 Hermes-Lite 2 (или на выходах с открытым коллектором любой другой платы openHPSDR — включая Protocol 2). «N2ADR» выбирает одно реле на диапазон; «Alex / Hermes band code» выставляет номер диапазона на выходы 1–4, чего ждут плата Alex ANAN, Zeus SDR, HiQSDR и Quisk. «Custom» открывает таблицу ниже, где вы сами задаёте семь выходов, диапазон за диапазоном, на приём и на передачу — для антенного переключателя, декодера диапазонов усилителя или чего-то ещё, что не подходит ни одна из готовых настроек; любую из них можно перенести в неё как отправную точку. Оставьте «None», если ничего реально не установлено: эти семь контактов — универсальные выходы с открытым коллектором, и операторы также подключают их к PTT усилителя, антенным реле и переключению трансвертеров. Управление ими по данным диапазона начнёт управлять тем, что подключено. Применяется при «Применить / переподключить».
settings-radio-io-board-rx-input = Вход приёма платы IO
settings-radio-where-n2adr-hl2ioboard-takes-receive = Откуда плата N2ADR HL2IOBoard берёт сигнал приёма. Сама плата находится автоматически и не требует настройки; эта существует только для операторов, которые подключили её собственные SMA-разъёмы. Иначе оставьте «Собственный вход радио» — выбор J9 платы IO без подключённого к нему ничего оставляет приёмник глухим. Применяется при «Применить / переподключить». Если выбран любой из вариантов J9, входы также появляются в управлении ANT и запоминаются по диапазонам.
settings-radio-power-amplifier = Усилитель мощности
settings-radio-use-hermes-lite-2-s = Использовать встроенный УМ Hermes-Lite 2
settings-radio-default-what-you-want-unless = По умолчанию включено, и это то, что вам нужно, если только внешний усилитель не управляется от маломощного выхода RF1 платы. При выключении радио всё ещё манипулируется — переключатель T/R срабатывает, и любая дополнительная плата следует за ним, — но разъём антенны не отдаёт вообще никакой мощности, а реле намеренно удерживается в приёме. Игнорируется на платах, кроме Hermes-Lite.
settings-radio-mirror-board-s-spectrum-about = Отразить спектр платы относительно настроенной частоты — при передаче так же, как при приёме. По умолчанию включено: Hermes-Lite 2 это требует. Выключайте, только если сигналы появляются не на той стороне шкалы и ничего не декодируется — признак: водопад полон убедительных следов, пока SSB ложится на неверную боковую, а FT8 не возвращает декодов вовсе.
settings-radio-frequency-correction = Коррекция частоты
settings-radio-crystal-tcxo-error-ppm-applied = Погрешность кварца/TCXO в ppm, применяется к RX и TX. Применяется сразу.
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = Буфер передачи
settings-radio-how-far-ahead-real-time = На сколько вперёд реального времени звук передачи подаётся на плату по сети, прежде чем sdroxide замедлится, чтобы соответствовать. Сама плата такого буфера не держит — это лишь расширяет собственный запас sdroxide. Увеличивайте на Wi‑Fi или VPN, где низкого значения по умолчанию (подходящего для прямого проводного соединения) недостаточно для запаса против джиттера, и переданный звук или PTT заикаются; большее значение увеличивает задержку передачи. Вступает в силу при APPLY, которое переподключает плату.
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = Адаптивная предыскажение: линеаризует передатчик по образцу того, что он реально излучил, — на двадцать с лишним децибел меньше интермодуляции при той же мощности. Приёмник — это обратная связь: плата продолжает принимать во время передачи, поэтому направленный ответвитель и аттенюатор должны подать образец выхода усилителя на вход, который переключатель T/R не отключает на передаче. На Hermes-Lite 2 это гнездо PureSignal платы IO и вход приёма выше, заданный под него; на плате без него — порт RX ANAN или внешнее реле, которое включает ответвитель на время передачи. Считывается именно этот приёмник — плата со вторым АЦП не может использовать RX2 как точку обратной связи, как бы её собственная прошивка ни направляла сигнал. Если ничего не подключено, контур никогда не захватывает, и передатчик остаётся таким, каким был бы. Применяется при «Применить / переподключить» и только на радио, которому принадлежит передатчик (DDC1).
settings-radio-correct-transmitter-receiver-s-own = Корректировать передатчик по собственной обратной связи приёмника
settings-radio-how-finely-amplifier-s-curve = Насколько точно моделируется кривая усилителя. Больше шагов повторяют более резкое колено и дольше заполняются; 32 — разумное начало.
settings-radio-how-fast-correction-follows-what = Как быстро коррекция следует за тем, что сообщает ответвитель. Медленно — правильно: здесь усредняется кривая усилителя, которая не меняется, по тракту обратной связи, в котором есть шум.
settings-radio-hold = Удержание
settings-radio-stop-adapting-keep-correction-where = Прекратить адаптацию и оставить коррекцию как есть — для измерений и для оператора, довольного тем, что она выучила.
settings-radio-manual-ip-overrides-discovery-press = IP вручную перекрывает поиск. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-open-collector-outputs-band = Выходы с открытым коллектором по диапазонам
settings-radio-one-control-word-per-band = Одно управляющее слово на диапазон, как это указано в документации вашего оборудования: бит 0 — выход 1, бит 6 — выход 7. RX выставляется при приёме на этом диапазоне, TX — пока передатчик включён; задайте им одинаковое значение для фильтра и разные — для всего, что относится только к одной стороне переключения (ключ усилителя, обход предусилителя приёма). Диапазоны, оставленные в 00, ничего не выставляют. Применяется при «Применить / переподключить».
settings-radio-alex-band-code = КОД ДИАПАЗОНА ALEX
settings-radio-replace-table-what-preset-would = Заменить таблицу тем, что эта настройка отправила бы на каждом диапазоне, как отправной точкой для правки. Ничто другое в конфигурации не меняется.
settings-radio-clear = СБРОС
settings-radio-every-output-off-every-band = Каждый выход выключен на каждом диапазоне — то, что делает «None».
settings-radio-band = Диапазон
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = Выставленные выходы
settings-radio-other = Прочее
settings-radio-everywhere-outside-amateur-bands-short = Везде вне любительских диапазонов — прослушивание коротких волн и всё, на что попадает шкала трансвертера, что не покрыто ни одним диапазоном.
settings-radio-dongle = Донгл
settings-radio-rescan = Пересканировать
settings-radio-re-list-usb-bus-no = Заново перечислить шину USB. Ничто не открывается, поэтому можно нажать во время приёма.
settings-radio-no-dongles-press-rescan = донглов нет — нажмите «Пересканировать»
settings-radio-first-one-found = — первый найденный —
settings-radio-rtl2832u-s-resampler-reaches-225 = Ресемплер RTL2832U охватывает 225–300 кГц и 900 кГц–3,2 МГц, ничего между ними. Вступает в силу при «Применить».
settings-radio-often-drops-samples = {"  "}(часто теряет отсчёты)
settings-radio-agc = AGC
settings-radio-manual-setting-measurement-weak-signal = Ручной — настройка для измерений и цифровых режимов слабых сигналов. Тюнер и демодулятор имеют независимые автоматические контуры.
settings-radio-tuner-gain = Усиление тюнера
settings-radio-applies-immediately-no-reconnect-tuner = Применяется сразу — без переподключения. У тюнера 29 дискретных ступеней, поэтому значение прыгает к ближайшей, которую он может выдать. Игнорируется, пока работает AGC тюнера.
settings-radio-crystal-error-parts-per-million = Погрешность кварца в миллионных долях. Запустите с RUST_LOG=sdroxide_rtlsdr=debug, и журнал выведет измеренную погрешность тактового генератора примерно через 20 секунд — это число и нужно ввести. Применяется сразу.
settings-radio-hf-reception = Приём на КВ
settings-radio-tuner-itself-starts-24-mhz =
    Сам тюнер начинается с 24 МГц. RTL-SDR Blog V4 поднимает частоту ниже этого аппаратно; остальные донглы достигают КВ, только сэмплируя АЦП напрямую, через КВ-порт V3. «Автоматически» отдаёт всё ниже 24 МГц АЦП, а всё выше — тюнеру.

    Direct sampling covers every HF band, 17 m and 15 m included — they arrive in the ADC's second Nyquist zone, the right way up. Nothing filters the ADC's input, though, so whatever is at 28.8 MHz minus the dial comes with them: 10.7 MHz under 17 m, 7.726 MHz under 15 m.

    Switching modes briefly interrupts the stream.
settings-radio-removes-dongle-s-own-dc =
    Убирает собственный DC-пик донгла из центра полосы и зеркальное изображение, которое оставляет каждый сигнал, отражённым относительно него, измеряя дисбаланс в самих отсчётах — без калибровки, и применяется сразу. У тюнера нет режима смещённой настройки, поэтому это единственный способ очистить центр.

    An AM carrier tuned dead on the dial is at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-remove-centre-spike-mirror-image = Убрать центральный пик и зеркальное изображение
settings-radio-bias-tee = Bias-tee
settings-radio-feed-4-5-v-dc = Подавать ~4,5 В постоянного тока по коаксиалу
settings-radio-bias-tee-never-connect-transceiver = Bias-tee ВКЛЮЧЁН. Никогда не подключайте трансивер, заземлённую антенну или предусилитель с питанием от другого источника, пока это включено, — постоянный ток идёт прямо по фидеру.
settings-radio-receive-only-dongle-sample-rate = Только приём. Донгл и частота дискретизации вступают в силу при «Применить»; всё остальное применяется по мере изменения.
settings-radio-server-address = Адрес сервера
settings-radio-where-rtl-tcp-listening-address =
    Где слушает rtl_tcp: адрес или адрес и порт. Порт по умолчанию — 1234, это собственный порт rtl_tcp.

    On the far end, start it as `rtl_tcp -a 0.0.0.0` — bound to 127.0.0.1, which is what it does with no -a, it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g = хост или хост:порт, например raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    Запрашивается у сервера, с теми же пределами ресемплера, что у локального донгла — это тот же кремний на удалённом конце.

    The figure beside each rate is what it costs on the link: the samples are sent uncompressed, and a rate the network cannot carry makes rtl_tcp drop the connection rather than degrade. Takes effect on Apply.
settings-radio-wired-link = {"  "}(проводная связь)
settings-radio-runs-server-s-dongle-manual = Работает на донгле сервера. Ручной — настройка для измерений и цифровых режимов слабых сигналов.
settings-radio-applies-immediately-no-reconnect-sent = Применяется сразу — без переподключения. Отправляется в десятых долях дБ, и сервер прижимает её к ступени своего тюнера; протокол не имеет ответов, поэтому то, на чём он остановился, нельзя считать обратно, и этот ползунок продолжает показывать то, что было запрошено. Игнорируется, пока работает AGC тюнера.
settings-radio-crystal-error-server-s-dongle =
    Погрешность кварца донгла *сервера* в миллионных долях — свойство этого железа, поэтому задаётся здесь, а не на донглах этой машины. Применяется сразу.

    The measured clock error the USB interface prints is not available here: over a network what that measurement sees is the buffering, not the crystal, and it is wrong by thousands of ppm. Calibrate the dongle on USB once and carry the number over, or tune a broadcast station of known frequency and adjust until it sits on the dial.
settings-radio-tuner-starts-24-mhz-below =
    Тюнер начинается с 24 МГц; ниже этого удалённому концу нужна помощь. Blog V4 поднимает частоту сам, и «Автоматически» его не трогает — это единственное, что он может, потому что протокол сообщает чип тюнера и ничего больше, а V4 по проводу неотличим от обычного R828D.

    On a V3 or any other dongle, Automatic switches the server to direct sampling below the tuner's own 24 MHz floor — which covers every HF band, 17 m and 15 m arriving in the ADC's second Nyquist zone with 28.8 MHz minus the dial folded on top. Choose Direct sampling explicitly for a plain R828D that hears nothing on HF. Switching briefly interrupts the stream.
settings-radio-removes-dongle-s-dc-spike =
    Убирает здесь DC-пик донгла и зеркальное изображение, по мере поступления отсчётов — это артефакты железа, поэтому они передаются по сети вместе со всем остальным. Применяется сразу.

    An AM carrier tuned dead on the dial sits at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-powers-preamp-far-end-s = Питает предусилитель от донгла удалённого конца. Старые серверы этой команды не реализуют и молча её игнорируют — у протокола нет способа сказать «нет», — поэтому bias-tee, который не включается, не обязательно вина этого конца.
settings-radio-feed-4-5-v-dc-2 = Подавать ~4,5 В постоянного тока по удалённому коаксиалу
settings-radio-bias-tee-hardware-somewhere-else = Bias-tee ВКЛЮЧЁН, на аппаратуре, которая находится в другом месте. Что бы ни было на другом конце того фидера — трансивер, заземлённая антенна, уже запитанный предусилитель, — перед вами, чтобы проверить, этого нет.
settings-radio-sdrplay-server-rsp-tcp = Сервер SDRplay (rsp_tcp)
settings-radio-sdrplay-server-greets-exactly-like = Сервер SDRplay приветствует точно так же, как донгл, поэтому они показываются всегда, а не когда что-то обнаружено. С обычным сервером rtl_tcp они просто игнорируются — протокол не имеет ответов и отбрасывает неизвестные команды. Вкладка «Устройство» называет сервер rsp_tcp, как только он назовёт себя сам.
settings-radio-input = Вход A
settings-radio-input-b = Вход B
settings-radio-hi-z = Hi-Z
settings-radio-lna-state = Состояние LNA
settings-radio-step-index-not-db-figure = Индекс ступени, а не значение в дБ: сколько стоит каждая ступень, зависит от модели RSP и диапазона. 0 — максимальное усиление.
settings-radio-if-gain-reduction = Ослабление ПЧ
settings-radio-reduction-so-more-less-signal = Ослабление, поэтому больше — значит меньше сигнала. Соблюдается только при выключенной AGC RSP.
settings-radio-enable = Включить
settings-radio-level-rsp-s-agc-aims = Уровень, который стремится удерживать AGC RSP.
settings-radio-notches = Режекторные фильтры
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = Выход опорного сигнала
settings-radio-t-24-mhz-clock-out = Выход тактового сигнала 24 МГц
settings-radio-rsp2-rspduo-only = Только RSP2 и RSPduo.
settings-radio-receive-only-address-sample-rate = Только приём. Адрес и частота дискретизации вступают в силу при «Применить»; всё остальное применяется по мере изменения. Оборванное соединение переподключается само, поэтому сервер, который перезапустили, вернётся без вмешательства здесь.
settings-radio-where-spyserver-listening-address-address =
    Где слушает spyserver: адрес или адрес и порт. Порт по умолчанию — 5555, собственный порт spyserver.

    On the far end, check that its config file binds an address other machines can reach — bound to 127.0.0.1 it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-2 = хост или хост:порт, например raspberrypi.local:5555
settings-radio-i-q-bandwidth = Полоса I/Q
settings-radio-how-wide-slice-band-arrives =
    Насколько широкий срез диапазона приходит как I/Q — и, значит, сколько связи это использует. Это *демодулированное* окно: оно следует за шкалой, а всё, что шире него, — это БПФ сервера в полосе над панорамой.

    Automatic aims at about 96 kHz, which carries every mode here including wide FM and still fits down a cellular uplink. Takes effect on Apply.
settings-radio-which-stage-server-s-own = Какую ступень собственной лестницы частот сервера запросить. У каждого приёмника своя лестница — это его максимальная частота, делённая пополам ступень за ступенью, поэтому это хранится как ступень, а не как число в герцах, и та же настройка по‑прежнему означает что-то разумное, направленная на другой сервер.

    Automatic aims at about 1 Msps. Press Test connection to see what the stages come to on this server. Takes effect on Apply.
settings-radio-automatic = Автоматически
settings-radio-sample-format = Формат отсчётов
settings-radio-bits-per-component-wire-so =
    Бит на компонент в проводе, и значит, во что это обходится по линии: 16 бит — вдвое больше 8, а 32-битная float — вчетверо, без большей информации, чем была у АЦП приёмника.

    8-bit is what makes a remote receiver work over a domestic uplink and is right for almost everything. A server may override this — some are configured to insist on one format — and says so in the log when it does. Takes effect on Apply.
settings-radio-gain = Усиление
settings-radio-server-s-gain-stage-index =
    Ступень усиления сервера, как индекс — а не число децибел. Что означает каждый индекс, принадлежит приёмнику на удалённом конце и меняется с диапазоном, а протокол этого никогда не говорит, поэтому ничто здесь не может превратить его в дБ, не выдумав цифру.

    The real range is the server's and is only known once connected; an index past it is clamped. Applies immediately — no reconnect.
settings-radio-index = индекс{" "}
settings-radio-digital-gain = Цифровое усиление
settings-radio-how-far-server-scales-samples =
    На сколько сервер масштабирует отсчёты вверх, прежде чем квантовать их для провода. Автоматически смотрит, что реально приходит, и держит пик на половине полной шкалы — это единственное, что может быть правильным и на пустом диапазоне, и на загруженном: они отличаются на тридцать децибел, и никакое фиксированное значение не подходит обоим.

    It matters only at 8 bits, where a signal far below full scale loses its lower bits to the quantiser and one above it is clipped flat. Wider formats have room to spare and get the figure every other client computes, from the gain index and the decimation stage. Applies immediately.
settings-radio-full-band-strip = Полоса всего диапазона
settings-radio-server-s-own-fft-whole =
    Собственное БПФ сервера по всему диапазону, рисуемое в полосе над панорамой. В этом интерфейсе это единственный вид диапазона — сама панорама ровно такой ширины, как принимаемый I/Q.

    Switching it off leaves a receiver with no way to see anything it is not already tuned to. Applies immediately, with a short break in the audio while the server changes what it is sending.
settings-radio-ask-server-low-rate-fft =
    Запрашивать у сервера низкочастотное БПФ всего диапазона наряду с I/Q и рисовать его в полосе над панорамой.

    It costs almost nothing — a couple of kilobytes a frame, a dozen or so times a second — and it shows the whole receiver rather than the slice being demodulated. Worth switching off only on a link where every byte counts. Applies immediately, with a short break in the audio.
settings-radio-show = Показывать
settings-radio-whole-band = весь диапазон
settings-radio-how-much-receiver-strip-covers = Сколько приёмника охватывает полоса. Весь диапазон — самый широкий вид; сужение кладёт то же число бинов на меньший спектр, что даёт более тонкие детали на меньшем участке.
settings-radio-strip-db-window = Окно дБ полосы
settings-radio-server-quantises-fft-into-window =
    Сервер квантует своё БПФ в это окно перед отправкой, по одному байту на бин — поэтому они решают, насколько точно измеряется полоса, а не только как она рисуется. Пол и потолок, с которыми полоса *отображается*, задаются автоматическим выравниванием движка и — отдельная вещь.

    The default 150 dB is the whole protocol range and needs no attention. Narrowing it around the noise floor buys resolution on a receiver whose signals all sit in a small part of the scale. Applies immediately.
settings-radio-top = верх{" "}
settings-radio-range-2 = диапазон{" "}
settings-radio-i-q-correction = Коррекция I/Q
settings-radio-remove-dc-spike-mirror-image = Убрать DC-пик и зеркальное изображение в DSP, на этой стороне. Нужно ли это приёмнику на удалённом конце, зависит от того, что он такое — Airspy HF+ не нуждается, RTL-SDR нуждается, — а протокол не говорит, с кем он разговаривает. Применяется сразу.
settings-radio-enabled = Включено
settings-radio-test-connection = Проверить соединение
settings-radio-connect-read-what-server-says = Подключиться, прочитать, что сервер говорит о своём приёмнике, и снова отключиться — без запуска потока, поэтому безопасно нажимать против сервера, которым пользуется кто-то другой.
settings-radio-receive-only-panadapter-narrow-i = Только приём. Панорама — это узкое окно I/Q, следующее за шкалой; вид диапазона — это БПФ сервера в полосе над ней. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-receive-only-wideband-i-q = Только приём. Широкополосный I/Q, как у любого локального SDR. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-receiver-address = Адрес приёмника
settings-radio-where-receiver-serves-web-page =
    Где приёмник отдаёт свою веб-страницу: адрес или адрес и порт. Порт по умолчанию — 8073, собственный порт KiwiSDR.

    Note that a receiver reached through the project's proxy (something.proxy.kiwisdr.com — nearly half the public ones) answers on port 80 instead, so give it explicitly. Picking a receiver from "Public SDRs" fills this in correctly.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-3 = хост или хост:порт, например kiwi.example.org:8073
settings-radio-password = Пароль
settings-radio-receiver-s-user-password-where =
    Пользовательский пароль приёмника, если его оператор его задал. Пусто почти на каждом публичном приёмнике. Не административный пароль.

    Takes effect on Apply.
settings-radio-announce = Представляться как
settings-radio-name-connection-shows-up-under =
    Имя, под которым появляется это соединение и которое видят владелец приёмника и все, кто его слушает.

    Left blank it is your station callsign, which is the network's own convention — some operators do block clients that will not identify. Takes effect on Apply.
settings-radio-blank-station-callsign = пусто = позывной станции
settings-radio-band-view = Вид диапазона
settings-radio-ask-receiver-own-waterfall-well =
    Запросить у приёмника его собственный водопад наряду с I/Q и рисовать его в полосе над панорамой.

    Worth having: the I/Q is only about 12 kHz wide, so without this there is nothing to tune *by*. It is a second connection carrying roughly 20 kB/s at full speed, against the I/Q's 44 kB/s.

    Takes effect on Apply.
settings-radio-show-receiver-s-0-30 = показывать водопад приёмника 0–30 МГц
settings-radio-band-view-speed = Скорость вида диапазона
settings-radio-how-often-receiver-sends-waterfall =
    Как часто приёмник отправляет строку водопада, от 1 (медленнее всего) до 4. Единственная настройка здесь, которая меняет стоимость связи во время работы, поэтому уменьшайте её на тарифицируемом соединении.

    Applies immediately.
settings-radio-band-view-span = Полоса вида диапазона
settings-radio-how-much-band-receiver-s =
    Сколько диапазона охватывает водопад приёмника. Его полные 0–30 МГц — у левого края ползунка, вдвое меньше с каждым шагом вправо.

    The waterfall is always 1024 bins wide however wide the window is, so this is really a resolution control: the whole band is 29 kHz to a bin, which is a band *map* rather than a picture of anything, while a few hundred kilohertz shows individual stations in a broadcast band. Zoomed in, the window follows your dial.

    The whole band is the default and is what makes the strip a thing to tune by; the panadapter below it is the ~12 kHz of I/Q. Applies immediately.
settings-radio-receiver-agc = AGC приёмника
settings-radio-receiver-s-agc-which-far =
    AGC приёмника, который находится на дальней стороне связи и перед I/Q — поэтому действует раньше всего, что делает sdroxide.

    On by default, unlike every local SDR here, because the measured alternative was worse: on a live receiver the manual gain was not monotonic across its range and its top end clipped the I/Q at full scale, while the AGC held about -24 dBFS with 7 dB of headroom.

    It does mean the sample amplitude is not a signal level, which is why the S-meter is read from the receiver's own figure instead. Applies immediately.
settings-radio-let-receiver-ride-own-gain = пусть приёмник сам ведёт собственное усиление
settings-radio-manual-gain = Ручное усиление
settings-radio-fixed-gain-when-receiver-s = Фиксированное усиление, когда AGC приёмника выключен, на собственной шкале 0–90. Не децибелы чего-либо заявленного — протокол называет это manGain и больше ничего не говорит, и измерено, что оно немонотонно, поэтому относитесь к этому как к ручке, чтобы найти удачное место, а не как к калибровке. Применяется сразу.
settings-radio-read-receiver-s-status-page = Прочитать страницу состояния приёмника: что он такое, что охватывает, сколько каналов свободно и разрешает ли его оператор соединения от приложений, помимо браузера. Не занимает ни одного из его каналов.
settings-radio-host-port-e-g-127 = хост:порт, например 127.0.0.1:50001
settings-radio-iq-sample-rate = Частота дискретизации IQ
settings-radio-receiver = Приёмник
settings-radio-rig-two-receivers-sunsdr2dx-can = Аппарат с двумя приёмниками (SunSDR2DX) может обслуживать две вкладки радио через одно соединение — запустите это радио на RX1, а другое — на RX2. Передатчик принадлежит радио на RX1.
settings-radio-stream-delay = Задержка потока
settings-radio-how-long-rig-s-iq = Сколько времени IQ аппарата приходит на новый центр после того, как sdroxide его сдвинул. Это собственный конвейер DSP аппарата, а не сеть — сервер на этой машине тоже имеет такой, — и пока панораму тянут полностью отдалённой, ошибка на эту величину сдвигает самые новые строки водопада в сторону от истории на ошибку, умноженную на скорость тяги. Слишком большое значение смещает их ровно настолько же, как слишком малое, только в другую сторону. Измерьте командой `cargo run --release -p sdroxide-tci --example retune_latency`; значение по умолчанию — SunSDR2DX на ExpertSDR3 при 192 кГц.
settings-radio-wideband-iq-receive-audio-transmit = Широкополосный IQ на приём, аудио на передачу. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-radio-address = Адрес радио
settings-radio-address-shown-radio-under-set = Адрес, показанный на радио в SET > Network. Там должен быть включён Network Control, и у радио должны быть заданы имя пользователя и пароль сети.
settings-radio-host-ip-e-g-192 = хост или IP, например 192.168.1.50
settings-radio-control-port = Порт управления
settings-radio-t-50001-unless-has-been-changed = 50001, если не менялся на радио.
settings-radio-network-user = Пользователь сети
settings-radio-stored-clear-radio-json-protocol = Хранится открытым текстом в radio.json. Протокол обратимо маскирует его по проводу, поэтому здесь ничто не делает его секретом.
settings-radio-receive = Приём с
settings-radio-audio-sample-rate = Частота дискретизации аудио
settings-radio-t-12-khz-if-needs-48000 = IF 12 кГц требует поток 48000 Гц — на этой частоте используется демодулированный звук радио.
settings-radio-if-spectrum = Спектр IF
settings-radio-which-way-round-radio-s = В какую сторону работает 12-кГц IF радио. «Автоматически» использует то, что, как известно, делает модель, — зеркально на IC-7760, нормально на всех остальных Icom. Задайте вручную, если SSB выходит на неверной боковой: признак — приходится выбирать USB там, где диапазон LSB, при том что собственный индикатор режима радио везде совпадает с sdroxide.
settings-radio-normal = Обычный
settings-radio-mirrored = Зеркальный
settings-radio-displayed-bandwidth = Отображаемая полоса
settings-radio-width-audio-band-panadapter-cat = Ширина панорамы звуковой полосы, как для аппарата CAT. Используется в цифровых режимах и когда спектроскоп радио выключен; иначе панорама — это спектроскоп, а Полоса спектроскопа задаёт её ширину.
settings-radio-how-cw-panel-s-keyer-2 = Как манипулятор панели CW передаёт. «Манипулятор аппарата» переводит радио в CW и отдаёт ему текст для отправки собственным манипулятором (CI-V 0x17). «Звуковая карта» вместо этого отправляет манипулируемый сайдтон по LAN как аудио (MCW) — тон на шкала + высота тона; и поскольку аппарат в CW проигнорировал бы свой вход модулятора, выбор CW затем оставляет радио на обычном USB — той же модой, на которой едут цифровые режимы.
settings-radio-how-much-audio-radio-holds = Сколько аудио радио удерживает перед модуляцией. Больше переживёт худшую сеть ценой задержки передачи.
settings-radio-streams-radio-s-own-475 = Передаёт собственную развёртку радио на 475 бинов. На AF это панорама — аудио, которое радио посылает, — то, что прошло через его фильтр, а не картинка диапазона, — а на IF 12 кГц это водопад всего диапазона над одной. В любом случае это картинка радио, а не DSP sdroxide: I/Q, из которого её можно вычислить, нет.
settings-radio-how-wide-sweep-only-wide = Насколько широко её разворачивать. Это единственный широкий вид, который есть у Icom, и радио сохраняет полосу, последний раз выбранную на собственном экране — часто несколько кГц, поэтому полоса может появиться не шире панорамы под ней. Задание полосы здесь также переводит спектроскоп в центральный режим, поэтому он следует за шкалой. Это меняет и собственный дисплей радио; «Как задано на радио» оставляет его как есть.
settings-radio-switch-modulation-input-lan = Переключить вход модуляции на LAN
settings-radio-transmit-audio-only-heard-when = Звук передачи слышен, только когда вход MOD радио установлен на LAN. sdroxide может записать это на модели, нумерацию меню которой знает; на любой другой он сообщает об этом и не трогает меню. Это заём: то, что радио держало там, сначала считывается и возвращается, когда сессия заканчивается, поэтому аппарат, используемый потом сам по себе, по‑прежнему слышит собственный микрофон.
settings-radio-copy-diagnostic-report = Скопировать диагностический отчёт
settings-radio-control-audio-radio-s-own = Управление, аудио и собственный спектроскоп радио по одному сетевому соединению — без последовательного кабеля и без звуковой карты. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-testing = Проверка…
settings-radio-was-only-check-press-apply = Это была только проверка — нажмите «Применить / переподключить» ниже, чтобы начать использовать это радио.
settings-radio-radios = Радио
settings-radio-asks-network-iio-devices-also = Спрашивает в сети устройства IIO и также пробует 192.168.2.1 напрямую — Pluto на конце USB-кабеля часто недоступен по multicast, хотя адрес работает.
settings-radio-no-radios-press-discover = радио нет — нажмите «Поиск»
settings-radio-address = Адрес
settings-radio-overrides-selection-above-usb-cable = Перекрывает выбор выше. USB-кабель представляет Pluto как сетевой адаптер, а не последовательный порт, поэтому это IP-адрес, даже когда радио стоит у вас на столе.
settings-radio-pluto-revision-c-pluto-unlocked = Pluto+ или Pluto ревизии C, разблокированный до 2R2T, может обслуживать две вкладки радио из одной коробки — это радио на RX1 и другое на RX2, каждое на своей антенне. Две цепочки делят один гетеродин, поэтому перестройка любого из радио двигает оба; что RX2 даёт — вторую антенну на том же спектре (разнесённый приём), а не второй диапазон. Передатчик принадлежит радио на RX1. Стандартный 1R1T Pluto отказывает RX2 при подключении.
settings-radio-width-spectrum-sdroxide-receives-ad9361 = Ширина спектра, который принимает sdroxide. AD9361 достигает 61,44 Msps; USB-сетевое соединение — нет, и под него масштабирован этот список. Самые низкие частоты требуют загрузки конфигурации фильтра в AD9361, которую sdroxide не делает — стандартный Pluto вместо этого работает на них около 2,084 Msps и сообщает об этом при подключении. Вступает в силу при «Применить».
settings-radio-more-than-usb-2-will = {"  "}(больше, чем вынесет USB 2)
settings-radio-stock-pluto-runs-2-084 = {"  "}(стандартный Pluto работает на 2,084)
settings-radio-analog-filter = Аналоговый фильтр
settings-radio-ad9361-s-baseband-filter-leave = Базовополосный фильтр AD9361. Оставьте 0 для автоматического режима, который открывает его до девяти десятых частоты дискретизации — широко намеренно, потому что приёмник сдвигает свой гетеродин на четверть полосы от шкалы, чтобы держать сигналы подальше от DC-пика, а узкий фильтр срезал бы как раз ту часть, в которую их сдвинул. Вступает в силу при «Применить».
settings-radio-ad9361-has-four-modes-not = У AD9361 четыре режима, а не переключатель вкл./выкл. Медленная атака подходит SSB и CW; быстрая атака — для импульсных сигналов; ручной — настройка для измерений и цифровых режимов слабых сигналов. Применяется сразу.
settings-radio-rx-gain = Усиление RX
settings-radio-applies-immediately-no-reconnect-ignored = Применяется сразу — без переподключения. Игнорируется, если AGC не переведён в ручной режим — это собственное поведение AD9361, а не sdroxide.
settings-radio-tx-gain = Усиление TX
settings-radio-negative-because-ad9361-expresses-transmit = Отрицательное, потому что AD9361 выражает уровень передачи как ослабление: 0 дБ — полная мощность. Применяется сразу. Передатчик переводится в самое тихое состояние до того, как это значение применяется при подключении, поэтому ничто, что оставила предыдущая программа, не может быть активным.
settings-radio-reference-error-parts-per-million = Погрешность опорного генератора в миллионных долях. Запустите с RUST_LOG=sdroxide_pluto=debug, и журнал выведет измеренную погрешность тактового генератора примерно через 20 секунд — это число и нужно ввести. Применяется sdroxide, а не записывается в собственную постоянную подстройку радио. Применяется сразу.
settings-radio-full-duplex = Полный дуплекс
settings-radio-keep-receiving-while-transmitting = Продолжать приём во время передачи
settings-radio-leave-off-pluto-reached-over =
    Оставьте это выключенным на Pluto, до которого добираются по USB-кабелю. Эта связь не может нести мегасэмпл в секунду в обоих направлениях одновременно, а передача, которая лишает буфер передачи данных, уходит порванной — поэтому по умолчанию приём останавливается на время передачи.

    Turn it on for a board on real Ethernet (a LibreSDR, a Pluto on a gigabit adapter), where there is room for both: you then hear the receiver through your own transmission, which is how a QO-100 station listens to its own downlink.

    An over needs twice the sample rate's worth of link — 2.5 Msps is 10 MB/s each way — so lower the rate if the log starts saying the link is not carrying it. The panadapter still shows the transmitted signal during an over; it is the audio that keeps coming.

    Takes effect on Apply.
settings-radio-duplex = Дуплекс
settings-radio-whether-ad9361-runs-both-directions =
    Работает ли AD9361 в обоих направлениях одновременно (FDD — так загружается Pluto и в каком режиме sdroxide всегда его оставлял) или по одному (TDD).

    Leave this on FDD unless you want the PTT pins below — TDD is what those key from, and it rules out Full duplex above. Takes effect on Apply.
settings-radio-ptt-pins = Контакты PTT
settings-radio-pluto-s-four-gpo-test =
    Четыре тестовые точки GPO Pluto сами могут манипулировать внешним усилителем мощности, LNA или переключателем передача/приём. Выберите пару, и один контакт высокий всё время, пока радио принимает, а другой — всё время, пока передаёт, — без программного обеспечения хоста в контуре и без последовательной линии PTT для подключения.

    This puts the radio in TDD whatever the Duplex row says, because the pins follow the AD9361's enable lines and in FDD both of those are asserted the entire session. Full duplex goes off with it, so leave both at their defaults if you work satellites and need to hear your own downlink.

    Analog Devices' own note puts an external LNA on GPO0/GPO1, so use GPO2/GPO3 if your board is wired that way. The pins are about 1.3 V at a few milliamps: drive a transistor or an opto-isolator with them, never a relay coil.

    Takes effect on Apply.
settings-radio-buffer-size = Размер буфера
settings-radio-how-much-device-holds-before =
    Сколько устройство удерживает перед каждой передачей, в комплексных отсчётах. Значение по умолчанию 32768 — около 16 мс при 2 Msps: достаточно долго, чтобы задержка каждой передачи туда и обратно не была узким местом, и достаточно коротко, чтобы перестройка не запаздывала заметно.

    Halve it if the log reports the receive socket being replaced. A marginal link — a Pluto reached over its USB cable at a high sample rate is the usual one — stalls part-way through a transfer, and a smaller transfer is both less likely to be caught mid-flight and quicker to make good afterwards. Raise it to trade retune latency for fewer round trips.

    Takes effect on Apply.
settings-radio-samples = {" "}отсчётов
settings-radio-rx-tx-port = Порт RX / TX
settings-radio-ad9361-s-rf-port-select = rf_port_select AD9361. Стандартный Pluto разводит по одному каждого, поэтому оставляйте пустым, если у вас нет платы, которая этого не делает. Вступает в силу при «Применить».
settings-radio-opens-radio-reads-what-says = Открывает радио, читает, что оно говорит о себе, и сообщает диапазон настройки именно этой платы. Потока не запускает.
settings-radio-wideband-iq-receive-transmit-half = Широкополосный IQ на приём и передачу. Полудуплекс — приём останавливается на время передачи, потому что USB-сетевое соединение не понесёт оба сразу. SoapySDR и libiio не нужны.
settings-radio-flexradio-announces-itself-local-network = FlexRadio объявляет себя в локальной сети примерно раз в секунду. Радио, до которого добираются через роутер или VPN, никогда не вещает вам — введите его адрес ниже.
settings-radio-another-gui-client-has-radio = Другой GUI-клиент занял это радио, и multiFLEX отключён. Отключите тот клиент или включите multiFLEX на радио.
settings-radio-overrides-selection-above-use-radio = Перекрывает выбор выше. Используйте это для радио в другой подсети, за VPN или на нестандартном порту.
settings-radio-width-spectrum-sdroxide-receives-192 = Ширина спектра, который принимает sdroxide. 192 кГц — максимум радио для потока DAX IQ, поэтому это самая широкая полоса, которую может показать этот интерфейс.
settings-radio-t-192-khz-widest-dax-iq = 192 кГц — самый широкий поток DAX IQ, который FLEX отправит, и радио привязывает к нему полосу панорамы — так что это вся полоса, и ни одна настройка здесь её не расширит.
settings-radio-dax-iq-channel = Канал DAX IQ
settings-radio-radio-has-four-change-only = У радио их четыре. Меняйте, только если что-то другое в сети уже использует канал 1 — радио вдвойне отказывает в канале.
settings-radio-station-name = Имя станции
settings-radio-shown-against-session-radio-s = Показывается напротив этой сессии в списке клиентов радио. Радио также запоминает клиента по нему, поэтому переименование заставит радио считать sdroxide новым.
settings-radio-gui-client-id = ID GUI-клиента
settings-radio-identity-radio-restores-session-s = Идентичность, к которой радио восстанавливает слайсы и панорамы этой сессии. Если оставить пустой, она выводится из имени станции — стабильно, но одинаково на каждом sdroxide, сохранившем имя по умолчанию, а радио разрешает дубликат, выбивая более раннего клиента. sdroxide замечает это на проводе и переходит на идентичность одной сессии, что стоит восстановления; укажите здесь собственный UUID, чтобы сохранить его.
settings-radio-optional-e-g-uuid-your = необязательно, например собственный UUID
settings-radio-mirror-radio-s-i-q =
    Отразить I/Q радио относительно центра панорамы. По умолчанию выключено — так был проверен FLEX-6600.

    Try it if receive audio is unintelligible on USB *and* on LSB and nothing decodes, while the waterfall looks entirely convincing — that is what a mirrored stream looks like, and it is the one fault with no other symptom (issue #368). If it is not that, turning this on makes it obvious rather than subtle. Applies on Apply / reconnect.
settings-radio-network-mtu = Сетевой MTU
settings-radio-largest-datagram-radio-may-send = Наибольшая датаграмма, которую радио может отправить. 1450 — то, что сам SmartSDR запрашивает. Уменьшите на пути с меньшим MTU — VPN или туннель, — где фрагменты отбрасываются и спектр вообще не приходит.
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = Проверяет, отвечает ли радио, не регистрируясь как GUI-клиент — поэтому не помешает уже работающей сессии SmartSDR.
settings-radio-wideband-iq-receive-over-dax = Широкополосный IQ на приём через DAX, аудио на передачу, которое модулирует радио. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-not-yet-verified-against-real-2 = Ещё не проверено на настоящем железе. Если работает некорректно, пришлите диагностический отчёт — он содержит каждую строку протокола, обмененную с радио.
settings-radio-devices-soapysdr-can-see = Устройства, которые видит SoapySDR
settings-radio-waiting-machine-radio-attached-where = Ожидание машины, к которой подключено радио, на которой установлены модули. Какое устройство открывается, задаёт `device_args` в config.toml этой машины или её --device.
settings-radio-ask-every-installed-soapysdr-module = Попросить каждый установленный модуль SoapySDR выполнить сканирование. Ничего не открывается, поэтому это безопасно во время приёма — но может занять мгновение.
settings-radio-not-enumerated-yet-press-rescan = Ещё не перечислено — нажмите «Пересканировать».
settings-radio-no-soapysdr-devices-found-check = Устройства SoapySDR не найдены. Проверьте, что модуль для вашего радио установлен и что у вас есть доступ к устройству.
settings-radio-not-radio = не радио
settings-radio-sound-card-listed-above-if = Звуковая карта перечислена выше, как если бы это был SDR. Она принимает любую частоту и игнорирует её, поэтому её открытие показывает вход звуковой карты вместо диапазона. sdroxide сам такие не выбирает — но строка device_args, которая называет одну из них, всё равно выполняется.
settings-radio-which-one-opens-set-device = Какое из них открывается, задаётся --device или device_args в config.toml; без того и другого побеждает первое перечисленное радио.
settings-radio-no-device = нет устройства
settings-radio-rig-has-no-software-adjustable = У этого аппарата нет программно регулируемых усилений.
settings-radio-rx-gains = Усиления RX
settings-radio-tx-gains = Усиления TX
settings-radio-antennas = Антенны
settings-radio-remembered-next-start = Запоминается для следующего запуска.
settings-radio-these-controls-appear-once-soapysdr = Эти элементы управления появляются, когда открыто устройство SoapySDR — это собственные элементы устройства, и его нужно спросить.
settings-radio-stream = Поток
settings-radio-follow-app-setting = Следовать настройке приложения
settings-radio-msps = или в Msps
settings-radio-baseband-filter = Базовополосный фильтр
settings-radio-let-driver-choose = Пусть выбирает драйвер
settings-radio-changing-either-reopens-radio = Изменение любого переоткрывает радио.
settings-radio-driver-publishes-no-settings-own = Этот драйвер не публикует собственных настроек.
settings-radio-applied-once-remembered-next-start = Применяется сразу и запоминается для следующего запуска.
settings-radio-no-rx-888-found-press = RX-888 не найден — нажмите «Пересканировать»
settings-radio-adc-clock = Тактовая частота АЦП
settings-radio-t-129-6-msps-needs-superspeed = 129,6 Msps требует связи SuperSpeed и быстрого хоста; 64,8 — безопасное значение по умолчанию. Смена переоткрывает приёмник автоматически, что занимает мгновение, но не требует перезапуска.
settings-radio-panadapter-width = Ширина панорамы
settings-radio-hf-only = {"  "}· только КВ
settings-radio-usable-both-front-ends = Пригодно на обоих фронтендах.
settings-radio-below-vhf-crossover-ordinary-width = Ниже точки перехода на VHF это обычная ширина. Выше неё ПЧ 8 МГц тюнера не может быть отцентрирована в окне такой ширины, поэтому живой спектр оказывается сбоку от панорамы, а рядом с ним пусто.
settings-radio-dial-above-crossover-now-so = Шкала сейчас выше точки перехода, поэтому эта ширина показывает ПЧ 8 МГц тюнера со смещением от центра и мёртвый спектр рядом. Уменьшите ширину, чтобы заполнить панораму.
settings-radio-vga-gain = Усиление VGA
settings-radio-ad8370-variable-gain-amplifier-ahead = Усилитель с регулируемым усилением AD8370 перед АЦП.
settings-radio-attenuator = Аттенюатор
settings-radio-pe4304-step-attenuator-0-5 = Ступенчатый аттенюатор PE4304, шагами по 0,5 дБ.
settings-radio-adc-range = Диапазон АЦП
settings-radio-wide-2-25-vp-p = Широкий (2,25 Vp-p)
settings-radio-selects-adc-s-wider-input = Выбирает более широкий входной диапазон АЦП: больше запаса для сильных сигналов вещания, меньше отсчётов для слабых. «Выкл.» выбирает более чувствительный диапазон 1,5 Vp-p.
settings-radio-dither = Дизеринг
settings-radio-adds-small-dither-signal-ahead = Добавляет небольшой сигнал дизеринга перед АЦП: стоит немного шумового пола, зато даёт свободный от паразитных составляющих динамический диапазон.
settings-radio-randomiser = Рандомизатор
settings-radio-adc-scrambles-output-so-digital = АЦП перемешивает свой выход, чтобы цифровая шина перестала излучать во фронтенд; драйвер восстанавливает его. Оставьте включённым, если не отлаживаете. Применяется при переподключении.
settings-radio-dc-hf-antenna-port = Постоянный ток на КВ-разъёме антенны
settings-radio-powers-active-antenna-preamp-down = Питает активную антенну или предусилитель по коаксиалу.
settings-radio-vhf-tuner-gain = Усиление VHF-тюнера
settings-radio-r828d-rf-gain-used-above = ВЧ-усиление R828D, используется выше автоматического перехода КВ/VHF. 29 дискретных ступеней; берётся ближайшая.
settings-radio-auto = Авто
settings-radio-let-tuner-run-own-lna = Пусть тюнер ведёт собственные контуры LNA и смесителя.
settings-radio-bias-tee-vhf = Bias-tee (VHF)
settings-radio-dc-vhf-antenna-port = Постоянный ток на VHF-разъёме антенны
settings-radio-clock-trim = Подстройка тактовой частоты
settings-radio-corrects-reference-oscillator-applied-when = Корректирует опорный генератор. Применяется, когда вы отпускаете значение — переоткрывать приёмник на каждом пикселе перетаскивания означало бы перезапускать его сотни раз.
settings-radio-receive-only-below-adc-s = Только приём. Ниже предела Найквиста АЦП антенна сэмплируется напрямую, и перестройка мгновенна, потому что аппаратного понижающего преобразователя нет — весь поток АЦП переводится в базовую полосу на хосте. Выше него приёмник автоматически переключается на свой тюнер R828D и VHF-SMA, поэтому обе антенны должны быть подключены. Для VHF нужна тактовая частота АЦП 32,4 Msps или выше, чтобы уместилась ПЧ тюнера, а на тактовых частотах ниже 48 Msps между двумя диапазонами есть промежуток, до которого ничто не дотянется. Каждая настройка здесь применяется сразу — кнопки «Применить» нет.
settings-radio-device = Устройство
settings-radio-no-devices-press-rescan = устройств нет — нажмите «Пересканировать»
settings-radio-six-rates-are-six-different =
    Шесть частот — это шесть разных образов FPGA, а не регистр, — поэтому ничто в протоколе ELAD не выбирает между ними, и смысл этой настройки зависит от модели.

    FDM-S1 / FDM-S2: a command. Their FPGA is loaded by the computer at every power-up, and sdroxide runs ELAD's own elad-firmware loader to put this rate's image in. Without that loader installed a sampler sends no samples at all, and sdroxide says so on screen.

    FDM-DUO: not a command. The radio boots its own FPGA and arrives at whatever rate it powered up in or FDM-SW2 last left it in (192 kHz on a fresh one), and this only says how the stream is READ. Get it wrong and the panadapter is simply the wrong width, with every frequency inside it scaled to match.

    The stream's real rate is measured a couple of seconds after it starts, and a mismatch is reported on screen. Takes effect on Apply.
settings-radio-fdm-s-fpga-image-loaded = FDM-S: загруженный образ FPGA — FDM-DUO: как читается поток
settings-radio-receiver-s-input-pad-one = Входной аттенюатор приёмника — один шаг, вверх или вниз. Тот же регулятор, что и ползунок Gain в главном окне.
settings-radio-pre-selection-filters = Предварительные фильтры
settings-radio-low-pass-bank-front-adc = Банк фильтров нижних частот перед АЦП. Его обход даёт самый широкий вид и самое плохое поведение вблизи сильных внеполосных сигналов — передатчик вещания в нескольких милях даст муары по всему диапазону. Оставьте включённым, если намеренно не слушаете вне отфильтрованного диапазона.
settings-radio-filters-circuit = Фильтры в тракте
settings-radio-rig-control-fdm-duo-only = Управление аппаратом — только FDM-DUO
settings-radio-transceiver-s-cat-usb-port = CAT-порт USB трансивера — отдельный кабель, не тот, что для приёма. Оставьте порт не заданным на FDM-S1 или FDM-S2, у которых его нет; на FDM-DUO вы предпочтёте управлять только через приёмный кабель, где его по‑прежнему можно настраивать и манипулировать, но нельзя снимать показания измерителей.
settings-radio-port-set-radio-s-display = Когда порт задан, дисплей радио и эта шкала — одно и то же число в обе стороны: окно приёма FDM-DUO построено вокруг его VFO, поэтому настройка здесь двигает переднюю панель, а вращение ручки на передней панели двигает показание здесь. Панорама при этом центрируется по шкале. Без порта — или если радио не отвечает на тот, что задан, — VFO для команды нет, и настройка остаётся внутри окна, которое радио уже отправляет.
settings-radio-none-usb-control-only = — нет (только управление по USB) —
settings-radio-must-match-menu-70-cat = Должна совпадать с меню 70 «CAT BAUD» на радио, по умолчанию 38400. Радио имеет эти четыре скорости и никакие другие, а порт, открытый на любой другой, молчит в обе стороны — радио игнорирует шкалу и отказывается включать передачу, не объясняя почему.
settings-radio-how-transmit-keyed-cat-sends = Как включается передача. «CAT» посылает собственную команду TX радио и ничего не требует настраивать на аппарате; RTS требует, чтобы меню 54 «PTT» было установлено в PTT+RTS.
settings-radio-where-radio-takes-transmit-audio-2 =
    Откуда радио берёт звук передачи — команда TI аппарата, которая на передней панели находится в меню 32 «TX IN». Выставляется при открытии порта.

    "USB audio" is what makes transmit work here: the FDM-DUO sends what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.
settings-radio-which-two-sockets-back-receiver-2 =
    Какой из двух разъёмов на задней панели слушает приёмник — команда AN аппарата, которая на передней панели находится в меню 31 «ANTENNAS» и отображается индикатором «ANT 1 2» на дисплее.

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    It moves this whole receiver: the panadapter, the demodulators and the rig's own audio all come from the socket selected here.

    Applies immediately, and is read back from the radio when the control port opens — this is the rig's own setting, not a copy of it kept here.
settings-radio-how-often-radio-asked-what-2 = Как часто у радио спрашивают, что оно делает — его шкалу, режим и измерители. Поверните собственную ручку VFO аппарата, и показание следует за ней в пределах одного опроса. Это также весь управляющий трафик с этой стороны, поэтому уменьшите его, если звук радио распадается.
settings-radio-transmit-audio = Звук передачи
settings-radio-only-receiving-fdm-duo-has = Только приём на FDM-DUO пока прогонялся на настоящем железе. Если что-то работает некорректно, приложите диагностический отчёт к отчёту об ошибке.
settings-radio-cw-keyed-radio-s-own = CW манипулируется собственным ключом или манипулятором радио: у FDM-DUO нет команды, которая принимала бы текст, поэтому панель CW не может его манипулировать.
settings-radio-press-apply-reopen-radio = Нажмите «Применить», чтобы переоткрыть радио.
settings-radio-no-receivers-press-rescan = приёмников нет — нажмите «Пересканировать»
settings-radio-which-rates-receiver-has-depends = Какие частоты есть у приёмника, зависит от модели и прошивки. Вступает в силу при «Применить».
settings-radio-every-rate-any-hf-model = каждая частота, которую предлагает любая модель HF+ — подключите одну, чтобы увидеть её собственные
settings-radio-receiver-s-own-gain-control = Собственный регулятор усиления приёмника. Оставьте включённым для обычного прослушивания; выключите, чтобы задать аттенюатор вручную для измерений.
settings-radio-agc-threshold = Порог AGC
settings-radio-high = Высокий
settings-radio-high-trades-little-sensitivity-headroom = «Высокий» меняет немного чувствительности на запас против сильных соседей — правильная настройка на загруженном диапазоне ночью.
settings-radio-front-end-attenuation-gain-0 = Ослабление фронтенда как усиление — 0 дБ означает отсутствие. Соблюдается только при выключенной AGC.
settings-radio-preamp = Предусилитель
settings-radio-hf-low-noise-amplifier-buys = КВ-малошумящий усилитель. Даёт чувствительность ценой интермодуляции, поэтому по умолчанию выключен — что обычно правильно на реальной антенне.
settings-radio-lna = LNA включён
settings-radio-frequency-calibration = Калибровка частоты
settings-radio-parts-per-billion-receiver-s = Части на миллиард — собственная единица этого приёмника, в тысячу раз точнее ppm, которым пользуется RTL-SDR. Ничто здесь никогда не записывается во флеш-память приёмника: это переопределяет сохранённое значение только на сессию.
settings-radio-use-receiver-s-stored-value = Использовать сохранённое в приёмнике значение
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = Подавать постоянный ток по коаксиалу
settings-radio-not-every-hf-has-one = Не у каждого HF+ есть; у приёмника без него это ничего не делает.
settings-radio-host-dsp = DSP на хосте
settings-radio-image-balancer-zero-if-offset = Балансировщик изображения, смещение нулевой ПЧ и гетеродин тонкой настройки. Выключайте только чтобы увидеть сырой выход аппаратуры — при выключении зеркальное изображение появляется на частотах нулевой ПЧ, а шкала точна лишь до ближайшего килогерца.
settings-radio-correct-image-fine-tune = Корректировать изображение и выполнять тонкую настройку
settings-radio-receive-only-no-soapysdr-no = Только приём. SoapySDR и libairspyhf не нужны. Ниже порога синтезатора настройку выполняет гетеродин на хосте — так этот приёмник достигает VLF. Приёмник и частота дискретизации вступают в силу при «Применить»; всё остальное применяется по мере изменения.
settings-radio-not-yet-verified-against-real-3 = Ещё не проверено на настоящем железе. Если работает некорректно, пришлите диагностический отчёт — он содержит каждую команду, обмененную с приёмником, и первые байты потока отсчётов, декодированные как пары I/Q.
settings-radio-no-airspy-r2-mini-found = Airspy R2 или Mini не найдены — нажмите «Пересканировать»
settings-radio-read-receiver = считывается с этого приёмника
settings-radio-r2-mini-offer-different-rates = R2 и Mini предлагают разные частоты и не различаются, пока один из них не открыт
settings-radio-rate-you-get-receiver-s = Это частота, которую вы получаете. АЦП приёмника работает на вдвое большей — он оцифровывает реальный сигнал, а sdroxide делает из него комплексную базовую полосу на хосте.
settings-radio-gain-curve = Кривая усиления
settings-radio-step-along-curve-above-not = Шаг по кривой выше, а не значение в дБ — LNA, смеситель и VGA тюнера двигаются вместе, и то, сколько стоит каждый шаг, зависит от кривой и диапазона. 0 — тихий конец.
settings-radio-tuner-agc = AGC тюнера
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = Передаёт LNA собственному контуру тюнера, переопределяя кривую.
settings-radio-mixer = Смеситель
settings-radio-same-mixer-stage = То же самое для ступени смесителя.
settings-radio-loop-running-gain-slider-no = Когда контур работает, ползунок усиления больше не задаёт ступень, которой он владеет — контур её перезаписывает.
settings-radio-dc-antenna-port = Постоянный ток на разъёме антенны
settings-radio-t-12-bit-packing = 12-битная упаковка
settings-radio-third-less-usb-traffic-leave = На треть меньше трафика USB. Оставьте включённым: это устройство USB 2.0, а верхняя частота — 30 МБ/с упакованными против 40 неупакованных. Применяется при переподключении.
settings-radio-dc-removal = Удаление постоянной составляющей
settings-radio-remove-adc-s-offset = Убрать смещение АЦП
settings-radio-turn-off-see-raw-hardware = Выключите, чтобы увидеть сырой выход аппаратуры. Полезно знать, куда уходит паразит: смещение попадает на край полосы, а не на её центр, потому что сигнал на пути переносится на четверть частоты дискретизации.
settings-radio-every-command-exchanged-receiver-sample = Каждая команда, обмененная с приёмником, арифметика частоты дискретизации и первые отсчёты, декодированные как I/Q.
settings-radio-receive-only-24-1800-mhz = Только приём, 24–1800 МГц. SoapySDR и libairspy не нужны. Приёмник, частота дискретизации и упаковка вступают в силу при «Применить»; всё остальное применяется по мере изменения.
settings-radio-not-yet-verified-against-real-4 = Ещё не проверено на настоящем железе. Если работает некорректно, пришлите диагностический отчёт — он содержит каждую команду, обмененную с приёмником, арифметику частот и первые отсчёты, декодированные как пары I/Q.
settings-radio-no-hydrasdr-rfone-found-press = HydraSDR RFOne не найден — нажмите «Пересканировать»
settings-radio-these-are-rates-receiver-turned = это частоты, которые оказались у этого приёмника
settings-radio-receiver-only-reports-three-these = приёмник сообщает только три из них; остальные — в альтернативной таблице прошивки, и старая сборка может их не нести
settings-radio-rf-input = ВЧ-вход
settings-radio-antenna-sma-only-socket-bias = Антенный SMA — единственный разъём, за которым стоит bias-tee.
settings-radio-cable-socket-no-bias-tee = Кабельный разъём. Bias-tee здесь нет; аппаратура выводит его только на ANT.
settings-radio-powers-active-antenna-preamp-down-2 = Питает активную антенну или предусилитель по коаксиалу. Только у разъёма ANT это есть — два кабельных порта — обычные входы.
settings-radio-third-less-usb-traffic-leave-2 = На треть меньше трафика USB. Оставьте включённым: это устройство USB 2.0, а верхняя частота — 36 МБ/с упакованными против 48 неупакованных. Применяется при переподключении.
settings-radio-one-these-boards-1d50-60a1 = Одна из этих плат на 1d50:60a1 — USB-идентификатор, который прототипы HydraSDR делят с Airspy R2 и Mini. sdroxide проверяет прошивку после открытия и сообщит, если выбран неверный интерфейс, в любую сторону.
settings-radio-receive-only-24-1800-mhz-2 = Только приём, 24–1800 МГц. SoapySDR и libhydrasdr не нужны. Приёмник, частота дискретизации и упаковка вступают в силу при «Применить»; всё остальное применяется по мере изменения.
settings-radio-re-list-fobos-sdrs-no = Заново перечислить Fobos SDR. Ничто не открывается, поэтому можно нажать во время приёма.
settings-radio-no-fobos-sdr-found-press = Fobos SDR не найден — нажмите «Пересканировать»
settings-radio-input-2 = Вход
settings-radio-through-tuner-lna-vga-mixer = Через тюнер: LNA, VGA и собственный диапазон частот смесителя.
settings-radio-direct-sampling-hf1-antenna-input = Прямое сэмплирование на антенном входе HF1 — без тюнера, без усиления LNA/VGA, настраивается полностью программно.
settings-radio-direct-sampling-hf2-antenna-input = Прямое сэмплирование на антенном входе HF2, так же как HF1.
settings-radio-both-real-adc-channels-once = Оба реальных канала АЦП сразу, объединяемые диверсифицирующим фильтром ниже — подавленный источник шума или два замирающих тракта, сложенные вместе.
settings-radio-direct-sampling-tuner-so-lna = Прямое сэмплирование — тюнер (а значит, и усиление LNA/VGA) не в тракте. Частота ниже — это цель для программного понижающего преобразователя, а не аппаратная настройка, поэтому достигнутая частота может отличаться от неё.
settings-radio-receiver-s-own-reported-rates = собственные сообщаемые этим приёмником частоты
settings-radio-measured-one-real-unit-connect = измерено на одном реальном экземпляре — подключите этот приёмник и нажмите «Пересканировать», чтобы увидеть его собственный список
settings-radio-clock-source = Источник тактовой частоты
settings-radio-external-reference = Внешний эталон
settings-radio-diversity-taps = Отводы разнесения
settings-radio-taps = {" "}отводов
settings-radio-one-tap-gain-phase-null = Один отвод — это усиление и фаза: провал на одной частоте, который ухудшается по обе стороны от неё, — всё, что может аналоговый фазовращатель. Каждый следующий отвод даёт один период отсчётов разницы хода пути между HF1 и HF2, который фильтр может выровнять, — и это превращает такой провал в полосу, тихую по всей ширине.
settings-radio-receive-only-rf-port-tuner = Только приём. ВЧ-порт (тюнер), HF1/HF2 (прямое сэмплирование, декодируемое программно) и HF1+HF2 (оба сразу, объединяемые диверсифицирующим фильтром) работают. Приёмник, вход и частота дискретизации вступают в силу при «Применить»; усиление, источник тактовой частоты и отводы применяются по мере изменения — режим диверсифицирующего фильтра, скорость адаптации и удержание — в собственном блоке DIV главного окна, когда это радио выбрано.
settings-radio-no-hackrf-found-press-rescan = HackRF не найден — нажмите «Пересканировать»
settings-radio-front-end-amplifier-8-db = Усилитель фронтенда шагами по 8 дБ. Это ступень, которая меняет чувствительность, — и первая, которая перегружается на реальной антенне.
settings-radio-baseband-amplifier-after-mixer-2 = Усилитель базовой полосы после смесителя, шагами по 2 дБ. Поднимайте его для слабого сигнала, прежде чем тянуться к LNA.
settings-radio-rf-amp = ВЧ-усилитель
settings-radio-one-switch-shared-transmit-setting = Один переключатель, общий с настройкой передачи ниже — радио применяет тот, что относится к направлению, в которое оно переходит. Выключено обычно правильно на реальной антенне.
settings-radio-hackrf-pro-chooses-itself-three = HackRF Pro выбирает это сам — три четверти частоты дискретизации, отфильтрованные в FPGA — и игнорирует всё, что просит хост.
settings-radio-leave-automatic-choosing-one-too = Оставьте «Автоматически». Выбор слишком узкого не просто смягчает края диапазона — он снимает смещение гетеродина, которое уводит паразит DC с вашего сигнала.
settings-radio-about-3-v-50-ma = Около 3 В при 50 мА по коаксиалу, для активной антенны или предусилителя. Только HackRF One или Pro; у Jawbreaker и rad1o такой схемы нет.
settings-radio-remove-dc-mirror-image = Убрать постоянную составляющую и зеркальное изображение
settings-radio-zero-if-radio-so-own = Это радио с нулевой ПЧ, поэтому собственная утечка гетеродина сидит посреди спектра, а погрешность квадратуры смесителя ставит через неё зеркальное изображение. Выключение показывает сырой выход аппаратуры — быстрый способ отличить проблему драйвера от проблемы DSP.
settings-radio-corrects-reference-oscillator = Корректирует опорный генератор.
settings-radio-transmit = Передача
settings-radio-transmitter = Передатчик
settings-radio-off-default-while-off-backend = Выключено по умолчанию. Пока это выключено, бэкенд не публикует вообще никакого канала передачи, поэтому ничто не может манипулировать радио. Применяется при переподключении.
settings-radio-tx-vga-gain = Усиление TX VGA
settings-radio-transmit-driver-amplifier-drive-applied = Драйверный усилитель передачи. Drive применяется цифрово до этой ступени, поэтому оставьте drive высоким и задавайте уровень выхода здесь — уменьшение drive вместо этого гонит ЦАП на долю полной шкалы и повышает интермодуляцию.
settings-radio-same-switch-receive-setting-above = Тот же переключатель, что и настройка приёма выше, применяется, когда радио меняет направление.
settings-radio-transmit-armed-into-dummy-load = Передача взведена. Работайте в эквивалент нагрузки, пока не измерите её: гармоники HackRF достаточно сильны, чтобы требовать внешнего фильтра нижних частот для вашего диапазона, и он полудуплексный, поэтому приём останавливается на время каждой передачи.
settings-radio-every-command-exchanged-radio-session = Каждая команда, обмененная с радио в этой сессии, по порядку, включая последовательность вокруг каждого нажатия.
settings-radio-dc-7-25-ghz-half = Постоянный ток – 7,25 ГГц, полудуплекс. SoapySDR, libusb и libhackrf не нужны. Радио и частота дискретизации вступают в силу при «Применить»; всё остальное применяется по мере изменения. Границы диапазона — от прошивки: HackRF рассчитан на 1 МГц – 6 ГГц (100 кГц у Pro) и сильно ослаблен за его пределами, но настраивается и там.
settings-radio-ask-sdrplay-api-service-device = Запросить у службы SDRplay API её список устройств. Ничто не открывается, поэтому можно нажать во время приёма.
settings-radio-no-rsps-press-rescan-needs = RSP не найдены — нажмите «Пересканировать» (нужна служба SDRplay API)
settings-radio-both-tuners-running-api-fixes = Когда работают оба тюнера, API фиксирует АЦП на 6 МГц и отдаёт 2 Msps из низкой ПЧ — поэтому 2 Msps — самая широкая полоса, а более узкие — это её децимация. Вступает в силу при «Применить».
settings-radio-rates-below-2-msps-run = Частоты ниже 2 Msps работают с АЦП на 2 Msps и децимируют в службе. Вступает в силу при «Применить».
settings-radio-reduced-adc-resolution = {"  "}(сниженное разрешение АЦП)
settings-radio-if-bandwidth = Полоса ПЧ
settings-radio-tuner-s-analog-filter-auto = Аналоговый фильтр тюнера. «Автоматически» выбирает самый широкий, который подходит к частоте дискретизации. Вступает в силу при «Применить».
settings-radio-rsp-s-own-if-gain = Собственный контур усиления ПЧ RSP, которым управляет служба API. Выключено отдаёт ползунок усиления ПЧ обратно вам — настройка для измерений и цифровых режимов слабых сигналов.
settings-radio-agc-set-point = Уставка AGC
settings-radio-signal-level-loop-holds-adc = Уровень сигнала, который контур удерживает на АЦП. Меньше оставляет больше запаса для сигналов вне канала. Насколько низко он может опуститься, зависит от частоты дискретизации — выше 8,064 Msps у преобразователя меньше запаса, а выше 9,216 Msps — ещё меньше.
settings-radio-rsp-s-native-gain-unit = Собственная единица усиления RSP: 20 дБ — максимальное усиление (0 с расширенным диапазоном ниже), 59 дБ — минимальное. Применяется сразу. Игнорируется, пока работает AGC — тогда значение принадлежит контуру, а измеритель S показывает, на чём он остановился.
settings-radio-extended-if-range = Расширенный диапазон ПЧ
settings-radio-lets-if-gain-reduction-go = Позволяет ослаблению ПЧ опускаться ниже 20 дБ, до 0 — последние 20 дБ усиления, которые есть у приёмника. «Выкл.» — собственное значение API по умолчанию и правильное для обычного прослушивания, потому что у нижнего края диапазона RSP легче всего перегрузить. Стоит включить для слабых сигналов, когда LNA уже на 0. Также позволяет уставке AGC подняться до 0 dBFS. Применяется сразу.
settings-radio-allow-if-gain-reduction-below = Разрешить ослабление ПЧ ниже 20 дБ
settings-radio-front-end-attenuation-steps-0 = Ослабление фронтенда ступенями: 0 — максимальное усиление, каждая ступень вверх подключает больше ослабления. На некоторых диапазонах ступеней меньше — драйвер ограничивает и сохраняет ваш выбор до тех пор, пока вы не вернётесь настраиваться, а шкала здесь заканчивается там, где его примет диапазон, на котором стоит радио. Более высокий выбор, сделанный на другом диапазоне, сохраняется и по‑прежнему показывается рядом. Применяется сразу.
settings-radio-t-0-max-gain = 0 = максимальное усиление
settings-radio-reference-error-parts-per-million-2 = Погрешность опорного генератора в миллионных долях, применяется самим устройством. Применяется сразу.
settings-radio-radio-s-tuner = Тюнер этого радио
settings-radio-tuner = Тюнер
settings-radio-which-rspduo-s-two-tuners = Какой из двух тюнеров RSPduo слушает это радио. Другой принадлежит радио, настроенному для него, — дайте ему тот же приёмник, а другой тюнер. Вступает в силу при «Применить».
settings-radio-which-rspduo-s-two-tuners-2 = Какой из двух тюнеров RSPduo несёт антенну, которую вы слушаете. Другой несёт вторую антенну. Вступает в силу при «Применить».
settings-radio-which-rspduo-s-two-tuners-3 = Какой из двух тюнеров RSPduo запускать (по одному за раз). Вступает в силу при «Применить».
settings-radio-applies-immediately = Применяется сразу.
settings-radio-dab-notch = Режекция DAB
settings-radio-hardware-notch-over-165-230 = Аппаратная режекция над полосой DAB 165–230 МГц. Применяется сразу.
settings-radio-hdr-mode = Режим HDR
settings-radio-enable-below-2-mhz = Включить ниже 2 МГц
settings-radio-rspdx-s-high-dynamic-range =
    Тракт высокого динамического диапазона RSPdx для ДВ/СВ. Применяется сразу.

    It does not work yet: on the one RSPdx it has been tried against, switching it on silences the receiver — at every centre, filter, port, gain and rate tried. Whatever the mode needs is not on the documented API surface, and finding it is somebody's next job. Leave this off unless you are the one looking.

    What is known: it is not a mode that follows the dial. The path has a fixed analog filter, built only at the centres listed under the filter below, so tuning anywhere else could not work even once the rest does.
settings-radio-hdr-filter = Фильтр HDR
settings-radio-feed-4-7-v-dc = Подавать ~4,7 В постоянного тока по коаксиалу
settings-radio-second-tuner = Второй тюнер
settings-radio-rspduo-two-whole-tuners-one = RSPduo — это два полноценных тюнера на одной плате, тактируемых от одного эталона. Запустите оба, и они слышат свои полосы в один и тот же миг с одного и того же тактового генератора — что делает две антенны на них когерентными, с относительной фазой, заданной фидерами, а не случаем, и позволяет сложить пару. Или держите их раздельно: тюнеры настраиваются независимо, поэтому второй может быть вторым радио на своём диапазоне.
settings-radio-run-both-tuners = Запустить оба тюнера
settings-radio-puts-rspduo-api-s-dual = Переводит RSPduo в двухтюнерный режим API, где АЦП фиксирован на 6 МГц, а самая широкая полоса — 2 Msps. Вступает в силу при «Применить», потому что режим выбирается при открытии устройства.
settings-radio-used = Используется для
settings-radio-what-other-tuner-takes-effect = Для чего другой тюнер. Вступает в силу при «Применить» — как и запуск обоих вообще, это выбирается при открытии платы.
settings-radio-add-second-radio-settings-radio = Добавьте второе радио (Настройки → Радио → +), дайте ему этот же приёмник и другой тюнер RSPduo и тоже настройте его на запуск обоих тюнеров — какое бы радио ни открыло плату, оно переводит её в двухтюнерный режим, поэтому оба должны этого ожидать. Тогда оба настраиваются независимо: КВ на одной вкладке и VHF на другой, с одной платы. Они делят одну тактовую частоту АЦП, поэтому оба работают на частоте дискретизации, которую запросило бы любое радио, открывшее плату, и ни одно не может передавать.
settings-radio-what-do = Что с этим делать
settings-radio-lna-state-2 = Его состояние LNA
settings-radio-second-tuner-s-own-front = Собственное ослабление фронтенда второго тюнера. Задайте так, чтобы обе антенны показывали примерно одинаковый шумовой пол: это регулировка, на которой держится всё остальное, потому что сложение взвешивает обе ветви по их шуму, а второй фронтенд, загнанный в перегрузку, отдаёт фильтру искажённую копию помехи — которую нельзя вычесть из неискажённой. Применяется сразу.
settings-radio-if-gain-reduction-2 = Его ослабление ПЧ
settings-radio-second-tuner-s-if-gain = Усиление ПЧ второго тюнера в собственной единице RSP: 20 дБ — максимальное усиление. Игнорируется, пока работает AGC, — а фильтру нужно ровное усиление, поэтому отключение AGC стоит того для провала, который вы намерены удерживать.
settings-radio-filter-length = Длина фильтра
settings-radio-one-tap-gain-phase-null-2 = Один отвод — это усиление и фаза: провал на одной частоте, который ухудшается по обе стороны от неё, — всё, что может аналоговый фазовращатель. Каждый следующий отвод даёт один период отсчётов разницы хода пути между двумя антеннами, который фильтр может выровнять, — и это превращает такой провал в полосу, тихую по всей ширине.
settings-radio-adaptation-2 = Адаптация
settings-radio-slow-steady-left-converging-inside = Медленно и ровно слева, сходится за долю секунды и заметно плавает справа. Начните быстро, чтобы найти провал, затем удерживайте его.
settings-radio-stop-filter-moving-reach-moment = Остановить движение фильтра. Тянитесь к этому в тот же миг, как появляется провал: фильтр, оставленный адаптироваться, снова наведётся на то, что стало громче всего, а на тихом диапазоне это станция, которую вы слушаете.
settings-radio-restart = Перезапуск
settings-radio-zero-filter-find-null-again = Обнулить фильтр и снова найти провал.
settings-radio-nothing-here-can-tell-wanted = Здесь ничто не может отличить нужный сигнал от ненужного — фильтр знает лишь то, что у двух антенн общего. В режиме «Подавить» вторая антенна хочет слышать источник шума и как можно меньше диапазона, иначе она послушно подавит и станцию. В режиме «Сложить» обе хотят слышать одну и ту же станцию. Насколько глубок провал, раз в несколько секунд выводится в журнал.
settings-radio-not-yet-verified-against-rspduo = Ещё не проверено на RSPduo: работа с двумя тюнерами здесь написана по API SDRplay, а не измерена на железе.
settings-radio-no-receiver-listed-check-against = Нет приёмника, с которым это можно сверить, — нажмите «Пересканировать» или прочитайте журнал о том, что реально открылось.
settings-radio-receive-only-1-khz-2 = Только приём, 1 кГц–2 ГГц. Нужна служба API SDRplay от поставщика (sdrplay.com/api) — у RSP после исходного RSP1 нет открытого протокола. Устройство, частота дискретизации, полоса и тюнер RSPduo вступают в силу при «Применить»; всё остальное применяется по мере изменения.
settings-radio-board = Плата
settings-radio-first-one-found-2 = Первый найденный
settings-radio-receive-chain = Приёмный тракт
settings-radio-both-chains-tune-together-they = Оба тракта настраиваются вместе — они делят один синтезатор, — но это отдельные фронтенды на отдельных разъёмах. Выберите тот, на котором ваша антенна; к этому тянутся, когда в одном тракте сделана модификация согласования КВ, а другой стоит штатным.
settings-radio-receive-gain = Усиление приёма
settings-radio-one-combined-figure-which-limesuite = Одна суммарная величина, которую LimeSuite распределяет между LNA, TIA и PGA сам. Принимает целые децибелы, поэтому всё более тонкое обрезается.
settings-radio-receive-port = Порт приёма
settings-radio-automatic-follows-frequency-lnal-low = «Автоматически» следует за частотой: LNAL на низких, LNAH на высоких — если только ниже не подключён LimeRFE; это один кабель в один разъём, и тогда LNAW на любой частоте. Назовите разъём, к которому подключено у вас, если это не он. Какому тракту принадлежит разъём, задаёт выбор выше.
settings-radio-t-0-follows-sample-rate-worth = 0 следует за частотой дискретизации. Стоит оставить там: фильтр уже четверти полосы молча снимает смещение гетеродина нулевой ПЧ, и утечка гетеродина возвращается поверх сигнала, который вы слушаете.
settings-radio-following-rate = следует за частотой
settings-radio-corrections = Коррекции
settings-radio-host-iq-dc-correction = Коррекция IQ / постоянной составляющей на хосте
settings-radio-adaptive-image-dc-removal-side = Адаптивное удаление изображения и постоянной составляющей на этой стороне, поверх собственной калибровки чипа. Выключение — способ в один щелчок отличить проблему драйвера от проблемы DSP.
settings-radio-calibrate-automatically = Калибровать автоматически
settings-radio-runs-chip-s-own-dc = Запускает собственную калибровку чипа по смещению постоянной составляющей и изображению при открытии радио и снова, когда шкала успокоилась на новом диапазоне или другом разъёме. Эти числа измерены на одной частоте и неверны в других местах, — а несущая, сидящая посреди спектра, обычно как раз такая. Занимает около секунды каждый раз.
settings-radio-calibrate-now = Калибровать сейчас
settings-radio-stalls-receiver-better-part-second = Останавливает приёмник на большую часть секунды.
settings-radio-second-aerial = Вторая антенна
settings-radio-socket = Её разъём
settings-radio-same-first = То же, что первая
settings-radio-gain-2 = Её усиление
settings-radio-set-low-coupled-sample-your = Задайте НИЗКО. Связанный образец вашего собственного передатчика — сильный сигнал, а цепочка обратной связи, загнанная в компрессию, неверно измеряет кривую усилителя — она учит коррекцию собственным искажениям. Начните с низа и используйте аттенюатор ответвителя.
settings-radio-set-so-both-aerials-show = Задайте так, чтобы обе антенны показывали примерно одинаковый шумовой пол. Это регулировка, на которой держится всё: сложение взвешивает обе ветви по их шуму, а вторая цепочка, загнанная в компрессию, отдаёт фильтру искажённую копию помехи — которую нельзя вычесть из неискажённой.
settings-radio-table-steps-2 = Шаги таблицы
settings-radio-how-finely-correction-follows-amplifier = Насколько точно коррекция следует за кривой усилителя. Больше шагов повторяют более резкое колено, но каждый нужно выучить по отсчётам, попавшим в него, — а верх гистограммы амплитуд речи тонкий. Тридцать два достаточно для плавной кривой, которая у ВЧ-усилителя действительно есть. Изменение начинает коррекцию заново.
settings-radio-how-hard-each-block-feedback = Насколько сильно каждый блок обратной связи сдвигает коррекцию. Кривая усилителя не меняется, поэтому спешить незачем — середина усредняет из неё шум за несколько выходов.
settings-radio-keep-correction-curve-learned-clean = Оставить коррекцию как есть. Кривую, выученную на чистом выходе, стоит удерживать, и усилитель к следующему не изменится.
settings-radio-forget-correction-learn-again = Забыть коррекцию и выучить заново.
settings-radio-directional-coupler-amplifier-s-output = Направленный ответвитель на выходе усилителя подключается к этой цепочке, и передатчик сравнивает то, что вернулось, с тем, что собирался послать, — затем посылает обратное от разности, поэтому то, что уходит из усилителя, прямое. На двадцать с лишним децибел меньше интермодуляции в чужих QSO, при этом усилитель сохраняет мощность. Коррекция остаётся равной единице, пока обратная связь не совпадёт с передачей, поэтому ненадключённый ответвитель ничего не стоит; и она никогда не может запросить у конвертера больше полной шкалы, так что путь обратной связи, выдающий ерунду, ничего не перегрузит. Как идут дела, выводится в журнал, пока вы передаёте.
settings-radio-transmit-not-armed-so-there = Передача не взведена, поэтому здесь нечего корректировать.
settings-radio-off-interface-publishes-no-transmit = Когда это выключено, интерфейс не публикует вообще никакого канала передачи, поэтому ничто не может манипулировать радио.
settings-radio-transmit-gain = Усиление передачи
settings-radio-transmit-port = Порт передачи
settings-radio-board-s-two-transmit-sockets = Два разъёма передачи платы — это две разные согласующие цепи, а не два выхода на одну: BAND1 (TX_1) несёт 30 МГц – 1,9 ГГц, а BAND2 (TX_2) — порт 13 см. Манипуляция любительского диапазона короче 23 см через BAND2 загоняет выход в согласующую цепь, которая пропускает из него почти ничего: каждая настройка показывает правильно, мощность — та, что вы задали, а измеритель мощности остаётся на нуле. Оставьте «Автоматически», и он будет следовать за шкалой.
settings-radio-limesdr-transmits-about-100-khz = LimeSDR передаёт примерно от 100 кГц до 3,8 ГГц без собственной фильтрации. Используйте фильтр нижних частот, подходящий канал LimeRFE или эквивалент нагрузки.
settings-radio-limerfe-front-end = Фронтенд LimeRFE
settings-radio-connected = Подключён через
settings-radio-limerfe-s-own-micro-usb = Собственный micro-USB-порт LimeRFE, а не радио. 9600 бод, фиксировано его прошивкой.
settings-radio-receive-connector = Разъём приёма
settings-radio-transmit-connector = Разъём передачи
settings-radio-follow-dial = Следовать за шкалой
settings-radio-switch-filters-match-operating-frequency = Переключать фильтры под рабочую частоту, до того как появится какое-либо ВЧ. Настройка внутри одного диапазона не посылает ничего в канал управления.
settings-radio-relays = Реле
settings-radio-receive-attenuator = Аттенюатор приёма
settings-radio-notch-filter = Режекторный фильтр
settings-radio-fan = Вентилятор
settings-radio-worth-having-any-sustained-transmitting = Стоит включить при любой продолжительной передаче.
settings-radio-automatic-board-receives-switched-transmit = На «Автоматически» плата принимает и переключается на передачу при нажатии ключа и обратно при отпускании — при любой разводке кабелей. Её любительские каналы имеют один фильтр с переключателем передача/приём с каждой стороны, поэтому плата, которой одновременно запросили оба, ставит этот переключатель на передатчик и перестаёт что-либо слышать. Канал 30 МГц доступен только через J5, который является одним разъёмом для обоих направлений; выше него передача с J4 держит приёмный тракт вне разъёма, которым управляет усилитель.
settings-radio-limesdr-support-has-not-been = Поддержка LimeSDR не проверена на железе. Это трассировка последней сессии для отчёта о проблеме.
settings-radio-gains-filters-corrections-antenna-sockets = Усиления, фильтры, коррекции, антенные разъёмы и каждый элемент управления LimeRFE — его разъёмы, диапазон, реле, аттенюатор, режекция и вентилятор — применяются сразу. Плата, приёмный тракт, частота дискретизации, взведение передачи и подключение LimeRFE вступают в силу при «Применить».

## Settings window, Radio tab: messages with values

settings-radio-fill-from = ЗАПОЛНИТЬ ИЗ { $name }
settings-radio-decimation-auto = Автоматически (ближайшая { $khz } кГц)
settings-radio-decimation-stage-server = Ступень { $stage } — частота сервера ÷ { $divisor }
settings-radio-decimation-stage = Ступень { $stage } — частота ÷ { $divisor }
settings-radio-kiwi-receive-only = Только приём — это чужая антенна. Панорама — это ~12 кГц, которые приёмник отправляет как I/Q и которые следуют за шкалой; вид диапазона над ней — собственный водопад приёмника, и настройка по нему перенастраивает приёмник. Около { $rate } kB/s при подключении. Нажмите «Применить / переподключить», чтобы переключить без перезапуска.
settings-radio-connected-detail = Подключено: { $detail }
settings-radio-failed = Ошибка: { $error }
settings-radio-pluto-buffer = ≈ { $ms } мс, { $kib } KiB на передачу
settings-radio-driver-settings = Настройки: { $driver }
settings-radio-rx888-coverage = Покрытие 0–{ $mhz } МГц, { $mbytes } МБ/с по USB
settings-radio-rx888-digitised = из оцифрованных { $mhz } МГц
settings-radio-elad-16-bit = { $khz } кГц (16-битные отсчёты)
settings-radio-elad-pad = Аттенюатор { $db } дБ включён
settings-radio-hackrf-rx-preamp = Предусилитель { $db } дБ на приёме
settings-radio-hackrf-tx-preamp = Предусилитель { $db } дБ на передаче
settings-radio-device-not-found = { $device } (не найдено)
settings-radio-lime-chain = Тракт { $n } (RX{ $n }_*)
settings-radio-lime-chain-tx = Тракт { $n } (RX{ $n }_* / TX{ $n }_*)
settings-radio-lime-second-chain = Второй приёмный тракт платы, на разъёмах { $sockets }. Он делит синтезатор, поэтому слышит тот же участок в тот же миг, что и первый — что позволяет ему нести вторую антенну или образец вашего собственного передатчика.
settings-radio-tx-blocked = Передача заблокирована: { $reason }
settings-radio-rfe-rx-fallback = Приём на { $port}: для них используется нефильтрованный широкополосный тракт: { $bands }.
settings-radio-rfe-tx-fallback = Передача на { $port}: для них используется широкополосный тракт — без усилителя диапазона и без фильтрации: { $bands }.

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = Громкость, усиление, AGC, шумоподавитель, регуляторы шумов и показание RDS на FM-вещании
topbar-receiver = ПРИЁМНИК
topbar-vfo-b-split-rit-xit = VFO A/B, split и смещения RIT/XIT
topbar-tuning = НАСТРОЙКА
topbar-diversity-filter-which-way-combines = Диверсифицирующий фильтр: как он складывает обе антенны, как быстро адаптируется и удержание его на месте
topbar-diversity = DIVERSITY
topbar-second-receiver-s-frequency-mode = Частота, режим, фильтр и уровень второго приёмника
topbar-sub-receiver = ДОП. ПРИЁМНИК
topbar-transceiver-s-own-aerial-socket = Собственный антенный разъём трансивера и его выключатель питания
topbar-radio = РАДИО
topbar-tune-voice-keyer-drive-mic = Настройка, голосовой манипулятор, а также уровни Drive и микрофона
topbar-transmit = ПЕРЕДАЧА
topbar-tune = {" "}TUNE{" "}
topbar-display = ДИСПЛЕЙ
topbar-logbook-spots-awards-memories-scanner = Журнал, споты, награды, память, сканер, настройки и руководство
topbar-system = СИСТЕМА
topbar-linked = Связано
topbar-not-linked = Не связано
topbar-bar-trace = столбцы / след
topbar-needle-bar-trace = стрелка / столбцы / след
topbar-swap-vfos = Поменять VFO местами
topbar-copy-b = Копировать A в B
topbar-second-receiver-right-ear-tunes = Второй приёмник, в правое ухо. Он настраивается независимо от A/B — его элементы управления появляются в модуле SUB, а его полоса пропускания — на водопаде.
topbar-band-plan = , из плана диапазонов
topbar-repeater-shift-following-band-plan = Сдвиг ретранслятора: следует плану диапазонов, в котором для этой частоты сдвига нет — симплекс
topbar-repeater-shift-transmit-above-below = Сдвиг ретранслятора: передача выше или ниже шкалы для работы через ретранслятор. Сейчас симплекс
topbar-repeater-shift = СДВИГ РЕТРАНСЛЯТОРА
topbar-auto = АВТО
topbar-take-shift-band-plan-dial = Брать сдвиг из плана диапазонов при движении шкалы. Он действует только внутри поддиапазона выхода ретранслятора — везде в другом месте оставляет радио в симплексе, поэтому вызывные каналы остаются симплексными.
topbar-offset = Смещение
topbar-repeater-tone-ctcss-dcs-goes = Тон ретранслятора: CTCSS/DCS, уходящий под голос, пачка 1750 Гц и тональный шумоподавитель на приёме
topbar-transmit-tone = ТОН ПЕРЕДАЧИ
topbar-set-here-sent-nfm-sub = задаётся здесь, посылается в NFM — субаудиосигнализация принадлежит FM-каналу
topbar-normal = ОБЫЧНЫЙ
topbar-invert = ИНВЕРСИЯ
topbar-bit-order-dcs-encoded-here = порядок битов, которым здесь кодируется DCS, переписан из стандарта и никогда не проверялся на ретрансляторе — если не открывается, попробуйте CTCSS
topbar-t-1750-hz-burst = ПАЧКА 1750 ГЦ
topbar-every-over = КАЖДЫЙ ВЫХОД
topbar-open-burst-every-time-transmitter = Открывать пачкой каждый раз, когда включается передатчик
topbar-send = ОТПРАВИТЬ
topbar-send-burst-now-keying-transmitter = Отправить пачку сейчас — включая передатчик на её длину, если он ещё не включён
topbar-nfm-only-burst-fm-repeater = Только NFM: пачка — это открыватель двери FM-ретранслятора
topbar-receive-tone-squelch = ТОНАЛЬНЫЙ ШУМОПОДАВИТЕЛЬ НА ПРИЁМЕ
topbar-require-receive-what-station-transmits = Требовать на приёме то, что передаёт эта станция
topbar-hide-docked-band-selector-click = Скрыть закреплённый выбор диапазона — нажмите снова, чтобы вернуть
topbar-show-docked-band-selector = Показать закреплённый выбор диапазона
topbar-band-mode = ДИАПАЗОН И РЕЖИМ
topbar-hide-band-chip-brings-back = Скрыть — чип диапазона вернёт его
topbar-dec-off = DEC выкл.
topbar-vol = Гром.
topbar-counted-steps-not-decibels-what =


    Counted in steps, not decibels: what one step is worth is the receiver's own business, and on an SDRplay it depends on the band as well. Right is still more gain.
topbar-gain = Усиление
topbar-plutosdr-s-own-agc-setting =


    The PlutoSDR's own AGC is setting this gain, so moving the slider does nothing. Set Settings → Radio → AGC to Manual to control it here.
topbar-agc-hang-time-click-cycle = Время удержания AGC — нажмите, чтобы переключать: Выкл. / Медленно / Средне / Быстро
topbar-man = Ручн.
topbar-manual-audio-gain-used-while = Ручное усиление звука, используется, пока AGC выключен. Задаётся от уровня, который держал AGC, когда его выключили.
topbar-radio-s-own-squelch-sent = Собственный шумоподавитель радио, отправляемый по каналу управления. Это затвор, через который звук действительно проходит, — программный закрывался бы лишь сильнее на том, что аппарат уже пропустил.
topbar-max-gain = макс. усиление
topbar-manual-gain = ручное усиление
topbar-noise-reduction = шумоподавление
topbar-impulse-noise-blanker = Импульсный шумовой блокатор
topbar-auto-notch-cancel-constant-tone = Автоматический режекторный фильтр: подавляет постоянные тональные составляющие (гетеродины)
topbar-binaural-audio-not-while-sub = Бинауральный звук — не когда доп. приёмник занимает правое ухо
topbar-binaural-audio-passband-spread-across = Бинауральный звук: полоса пропускания распределяется по двум ушам, поэтому сигналы разной высоты приходят с разных направлений, а настраиваемый вами плывёт между ними. Нажмите, чтобы вернуться к моно
topbar-binaural-cw-spread-passband-across = Бинауральный CW: распределить полосу пропускания по двум ушам, чтобы сигналы разной высоты приходили с разных направлений — пайл-ап становится несколькими местами вместо одной тесной ноты, а настройка станции переносит её между ушами. Лучше всего в наушниках
topbar-binaural-audio-spread-passband-across = Бинауральный звук: распределить полосу пропускания по двум ушам. На голосе шум распределяется по всей картине, а станция остаётся в её середине — это легче слушать часами, — ценой того, что сам голос растянут. Лучше всего в наушниках
topbar-mute = БЕЗ ЗВУКА
topbar-record-audio-raw-i-q = Записать звук, сырой I/Q или оба
topbar-wfm-stereo-forced-off-click = Стерео WFM принудительно выключено — нажмите для автоматического стерео
topbar-wfm-stereo-pilot-locked-click = Стерео WFM: пилот захвачен. Нажмите, чтобы принудительно моно
topbar-wfm-stereo-automatic-no-pilot = Стерео WFM: автоматически, на этой станции нет пилота
topbar-rds-data-arriving-click-see = Данные RDS поступают — нажмите, чтобы увидеть их
topbar-rds-no-data-station-click = RDS — на этой станции данных нет. Всё равно нажмите для диагностики декодера
topbar-drm-decoding-click-details = DRM декодируется — нажмите для подробностей
topbar-drm-click-decoder-s-state = DRM — нажмите для состояния декодера
topbar-hd-radio-decoding-click-details = HD Radio декодируется — нажмите для подробностей
topbar-hd-radio-click-decoder-s = HD Radio — нажмите для состояния декодера
topbar-presets = ПРЕСЕТЫ
topbar-width-per-sideband = ШИРИНА ДЛЯ КАЖДОЙ БОКОВОЙ
topbar-width = ШИРИНА
topbar-passband-width-hertz-type-drag = Ширина полосы пропускания в герцах — введите или перетащите. Нажмите в поле, чтобы ввести точное значение.
topbar-edges = ГРАНИЦЫ
topbar-low-edge-hz-carrier = Нижняя граница, в Гц от несущей
topbar-high-edge-hz-carrier = Верхняя граница, в Гц от несущей
topbar-noise-reduction-voice-off-click = Шумоподавление (голос), выключено — нажмите, чтобы выбрать движок
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = Что вы слышите, в MP3: приёмник в одном канале, ваша собственная передача — в другом. Это запись QSO.
topbar-mono = МОНО
topbar-mixed-one-channel-click-two = Сведено в один канал — нажмите для двух
topbar-two-channels-rx-left-tx = Два канала: RX слева, TX справа — нажмите для одного сведённого канала
topbar-quick-clip = БЫСТРЫЙ КЛИП
topbar-stop-after = ОСТАНОВИТЬ ЧЕРЕЗ
topbar-no-stop = без остановки
topbar-auto-record = АВТОЗАПИСЬ
topbar-set-squelch-auto-record-needs = Задайте шумоподавитель — автозаписи нужен один, чтобы знать тишину
topbar-auto-record-needs-sdr-front = Автозаписи нужен SDR-фронтенд с программным шумоподавителем
topbar-stop-following-squelch = Перестать следовать за шумоподавителем
topbar-waiting-signal = ожидание сигнала
topbar-record-spectrum = ЗАПИСЬ СПЕКТРА
topbar-i-q-wav = ● I/Q WAV
topbar-i-q-wav-2 = I/Q WAV
topbar-raw-spectrum-receiver-delivering-stereo = Сырой спектр, который отдаёт приёмник, как стерео WAV 32-битный float (RF64 сверх 4 ГБ), который открывают SDR#, SDRuno, HDSDR и SDRangel — и который сам sdroxide воспроизводит с --file. Большой: 8 байт на отсчёт.
topbar-radio-hands-over-demodulated-audio = Это радио отдаёт демодулированный звук, поэтому I/Q для записи нет.
topbar-what-capture-costs-sample-rate = Во что обходится захват на этой частоте дискретизации
topbar-engine = ДВИЖОК
topbar-no-noise-reduction-decoders-never = Без шумоподавления — декодеры его всё равно никогда не видели
topbar-strength = СИЛА
topbar-tone = ТОН
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS — на этом сигнале нет субаудиотона
topbar-tone-squelch = ТОНАЛЬНЫЙ ШУМОПОДАВИТЕЛЬ
topbar-carrier-squelch-open-any-signal = Шумоподавитель по несущей: открывается на любой сигнал
topbar-require-tone-currently-being-received = Требовать тон, который сейчас принимается
topbar-ctcss = CTCSS
topbar-any-dcs = ЛЮБОЙ DCS
topbar-open-any-dcs-coded-signal = Открывается на любой сигнал с кодом DCS. Какой из 104 кодов он несёт, здесь надёжно не прочитать, поэтому точнее выбрать нечего
topbar-which-socket-back-radio-receiving =
    Какой разъём на задней панели принимает радио — его собственная команда ANT, та же настройка, что и кнопка ANT на передней панели. Нажмите, чтобы перейти к следующему.

    The choice is remembered per band, and put back the next time the dial crosses into that band.
topbar-radio-s-separate-receiving-antenna =
    Отдельная приёмная антенна радио, подключаемая к приёмному тракту или отключаемая от него — собственная настройка RX ANT. Антенна на основном разъёме остаётся на передаче в любом случае.

    The radio remembers this per band itself, so sdroxide reads it back after every band change rather than putting back what it last saw: clicking here is the only thing that moves it.
topbar-radio-s-own-power-switch =
    Собственный выключатель питания радио по каналу управления и единственное настоящее включение и выключение в программе — не переключатель LINK sdroxide, который закрывает конец sdroxide и оставляет радио работающим.

    For ON to reach anything the radio's control end has to stay awake while it is off: Network Control over the LAN, or a CI-V port still fed from the mains on a set switched off at the front.
topbar-switch-radio = Включить радио
topbar-switch-radio-off-audio-meters = Выключить радио. Вместе с ним останавливаются звук и измерители.
topbar-what-second-aerial-click-swap =
    Для чего вторая антенна — нажмите, чтобы поменять.

    CANCEL subtracts it from the first, in the gain, phase and delay that make a local noise source line up on both: the DSP form of a noise-cancelling phaser. COMBINE adds the two in the phase that reinforces, weighted by how well each hears — diversity reception, which fills in fades.
topbar-hold = УДЕРЖАНИЕ
topbar-stop-filter-moving-reach-moment = Остановить движение фильтра. Тянитесь к этому в тот же миг, как появляется провал: фильтр, оставленный адаптироваться, снова наведётся на то, что стало громче всего, а на тихом диапазоне это станция, которую вы слушаете.
topbar-restart = ПЕРЕЗАПУСК
topbar-zero-filter-find-null-again = Обнулить фильтр и снова найти провал.
topbar-adapt = Адаптация
topbar-how-fast-filter-chases-slow = Как быстро фильтр догоняет: медленно и ровно слева, сходится за долю секунды и заметно плавает справа. Начните быстро, чтобы найти провал, затем УДЕРЖИВАЙТЕ его.
topbar-where-sub-receiver-listens-shift = Где слушает доп. приёмник. Щёлкните с Shift по водопаду или перетащите внутри полосы пропускания доп. приёмника, чтобы сдвинуть его.
topbar-dial = ←ШКАЛА
topbar-move-sub-receiver-main-dial = Перенести доп. приёмник на основную шкалу
topbar-dial-2 = ШКАЛА←
topbar-move-main-dial-sub-receiver = Перенести основную шкалу на доп. приёмник
topbar-filter = Фильтр
topbar-sub-receiver-passband-edges-hz = Границы полосы пропускания доп. приёмника, в Гц
topbar-sub-receiver-level-plays-right = Уровень доп. приёмника (он звучит в правом ухе)
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = Голосовой манипулятор: записать и передать сохранённые сообщения
topbar-drive = Drive
topbar-tune-2 = Настройка
topbar-mic = Микр.
topbar-cessb = CESSB
topbar-microphone-gain = Усиление микрофона
topbar-deviation-how-far-mode-s =
    Девиация: насколько сильно пачка этого режима раскачивает радио, которое модулирует её само. FM-передатчик превращает уровень звука в отклонение частоты и не имеет ALC, чтобы это поймать, — поэтому полная шкала на вход данных, настроенный под голос, даёт перевышение девиации, которое слушателю звучит совершенно нормально и не декодируется никем.

    Kept per mode, so a deviation set for 1200 baud packet never lands on FT8.
topbar-transmit-audio-how-hard-mode =
    Звук передачи: насколько сильно этот режим раскачивает модулятор радио, которое модулирует то, что мы ему посылаем, — аппарат CAT через звуковую карту, FLEX, Icom по сетевому порту. Уменьшайте, пока ALC аппарата почти не движется, и задайте мощность на самом радио; ALC, «едущий» на цифровом режиме постоянной огибающей, и есть то, что даёт помехи-брызги.

    Drive is not this control: on these radios Drive reaches the rig's power register and never touches its audio.

    Kept per mode — FT8, RTTY, PSK and MCW each keep their own.
topbar-tx-audio = Звук передачи
topbar-cw-psk-rtty-skimmers-decode = Скиммеры CW / PSK / RTTY — декодируют сигналы по всему диапазону и отмечают их на водопаде
topbar-skimmers = СКИММЕРЫ
topbar-run-skimmer = Запустить этот скиммер
topbar-minimum-snr-decoded-signal-needs = Минимальное отношение сигнал/шум, которое нужно декодированному сигналу, чтобы его отметили
topbar-needs-wideband-iq-source = нужен широкополосный источник IQ
topbar-cw-decoder = ДЕКОДЕР CW
topbar-how-many-signals-model-reads = Сколько сигналов модель читает одновременно. Остальные сохраняют свою метку, но текста не несут.
topbar-spots = СПОТЫ
topbar-save-skimmer-s-spot-list = Сохранить список спотов скиммера в файл
topbar-show-hide-full-band-waterfall = Показать/скрыть полосу водопада всего диапазона над панорамой — всё, что этот приёмник может видеть сразу
topbar-spectrum-waterfall-either-layer-both = Спектр и водопад — любой слой, оба или ни одного. Горит, когда показаны оба.
topbar-spectrum = СПЕКТР
topbar-show-spectrum = ПОКАЗАТЬ СПЕКТР
topbar-draw-spectrum-line-across-top = Рисовать линию спектра поверх панорамы. Выключено — водопад занимает всю высоту.
topbar-peak-hold = УДЕРЖАНИЕ ПИКА
topbar-trace-highest-level-each-column = Вести след самого высокого уровня, который достигла каждая колонка поверх живой линии, медленно спадая — что делал диапазон, пока вы смотрели в другую сторону
topbar-draw-spectrum-receding-surface-instead = Рисовать спектр как уходящую поверхность вместо плоской линии: самый свежий спектр — спереди, а предыдущие уходят от вас. Последние пару секунд диапазона как ландшафт — несущая, которая появляется и исчезает, — это гребень, а не дёргающаяся линия. Сетка и удержание пика относятся к плоской линии и на ней не рисуются.
topbar-how-quickly-spectrum-line-follows = Как быстро линия спектра следует за диапазоном. Медленнее — больше кадров усредняется друг с другом: линия спокойнее, а слабая несущая, стоящая достаточно долго, чтобы её прочитать. Водопад это не затрагивает — эти строки получают каждый кадр в любом случае.
topbar-show-waterfall = ПОКАЗАТЬ ВОДОПАД
topbar-draw-scrolling-waterfall-below-spectrum = Рисовать прокручивающийся водопад под спектром. Выключено — линия спектра занимает всю высоту.
topbar-how-fast-waterfall-scrolls-lines = Как быстро прокручивается водопад, в строках в секунду: Медленно 5, Средне 28, Быстро 56, Быстрее 112, Самый быстрый 224. Движок задаёт эти значения сам, поэтому два самых быстрых — реальная детализация, а не та же строка, нарисованная дважды — насколько приёмник может их питать: строка никогда не показывает больше одного преобразования, а узкий фронтенд даёт лишь несколько десятков в секунду. Они расходуют историю, так как водопад хранит фиксированное число строк — 73 секунды на «Средне», 9 на «Самый быстрый».
topbar-decode-labels = МЕТКИ ДЕКОДОВ
topbar-mark-every-decoded-station-waterfall = Отмечать каждую декодированную станцию на водопаде её позывным на частоте, на которой её услышали, — FT8, FT4 и другие режимы со слотами. Хорошее начало ставит их по тридцать на диапазон дважды в минуту поверх следов, которые вы читаете; выключите — и водопад просто диапазон. Список декодов рядом по‑прежнему содержит каждый позывной.
topbar-switch-3d-set = Включите 3D, чтобы задать это
topbar-how-3d-spectrum-drawn-only = Как рисуется трёхмерный спектр. Различается только форма — оба вида показывают одни и те же спектры, и оба скрывают то, что за ними, поэтому сильный сигнал спереди стоит над полосой, которую закрывает.
topbar-lines = ЛИНИИ
topbar-one-trace-per-remembered-spectrum = Один след на запомненный спектр, цветом плоской линии, каждый скрывает те, что сзади. Форма без уровней — и вариант, который выбирают, когда важна именно форма: он стоит чуть дороже сплошного, а не дешевле, потому что рисует ту же поверхность, а затем обводит каждый гребень поверх.
topbar-solid = СПЛОШНОЙ
topbar-filled-surface-coloured-waterfall-s = Заполненная поверхность, окрашенная палитрой водопада, поэтому уровень виден и в цвете, и в высоте, а две половины панорамы согласованы в том, как выглядит сильный сигнал. Палитру меняйте в Настройки › Дисплей.
topbar-how-fast-3d-spectrum-flows = Как быстро трёхмерный спектр уходит от вас, в строках в секунду: Медленно 6, Средне 12, Быстро 24, Быстрее 48. Поверхность имеет фиксированную глубину в строках, поэтому это также то, сколько времени она хранит. Медленнее — длиннее память и поверхность ползёт; быстрее — короче, но движется. Водопад ведёт собственную скорость прокрутки, ниже.
topbar-how-many-columns-panadapter-waterfall = Сколько колонок рисуются у панорамы и её водопада. «Авто» читает рендерер этой машины и размер панорамы и берёт наибольшее, что может нести, — 4K-экран хочет 4096. Каждая колонка — байт в каждом кадре, поэтому клиент, подключённый к удалённой станции, платит за детализацию своей связью: 4096 колонок при 60 кадрах в секунду — около четверти мегабайта в секунду, вдвое больше стандартной ширины.
topbar-read-machine-screen-take-most = Читать эту машину и этот экран, брать наибольшее, что они могут нести, и следовать за ними, если они изменятся
topbar-width-force-right-now-whatever = Ширина, которая действует прямо сейчас, что бы ни запрашивала строка выше
topbar-keep-floor-ceiling-set-best = Держать пол/потолок заданными для лучшего контраста водопада — плавно возвращается на место при смене диапазона, после панорамирования или масштабирования и когда уровни уплывают. Включите, чтобы подогнать сразу; выключите, чтобы оставить уровни там, где вы их задали.
topbar-keep-tuned-frequency-middle-panadapter = Держать настроенную частоту в середине панорамы: окно скользит под шкалой, вместо того чтобы картинка прыгала на целый участок, когда вы настраиваетесь за край. Включите, чтобы сразу отцентрировать; выключите, чтобы панорамировать и масштабировать где угодно.
topbar-ceil = потолок{" "}
topbar-fft-size = РАЗМЕР FFT
topbar-scroll-waterfall-upwards-newest-row = Прокручивать водопад вверх — самая новая строка внизу (V)
topbar-logbook-all-qsos-digital-manual = Журнал — все QSO (цифровые + ручные)
topbar-award-tracking-dxcc-was-waz = Учёт наград — DXCC / WAS / WAZ / квадраты
topbar-band-conditions-published-forecast-beside = Условия на диапазонах — опубликованный прогноз рядом с тем, что реально слышно на каждом диапазоне
topbar-satellite-qo-100-beacon-calibration = Спутник — идёт калибровка маяка QO-100
topbar-satellite-doppler-tracking-qo-100 = Спутник — доплеровское сопровождение и калибровка маяка QO-100
topbar-ism-band-devices-decoding-now = Устройства ISM-диапазона — декодируются сейчас, открыто ли это окно или нет. Выключите кнопкой DECODING внутри окна.
topbar-ism-band-devices-weather-sensors = Устройства ISM-диапазона — метеодатчики, счётчики и домашняя автоматика вокруг вас
topbar-public-sdrs-internet-browse-kiwisdr = Публичные SDR в интернете — просмотрите каталоги KiwiSDR и SpyServer и откройте один как радио
topbar-winlink-radio-email = Радиопочта Winlink
topbar-memory-channels = Ячейки памяти
topbar-scanner-stopped-signal = Сканер — остановился на сигнале
topbar-scanner-running = Сканер — работает
topbar-scan-memory-channels-frequency-range = Сканировать ячейки памяти или диапазон частот
topbar-grid-tracker-worked-maidenhead-squares = Трекер квадратов — отработанные квадраты Майденгеда на карте вместе с тем, что слышно
topbar-settings-device-gains-antennas-audio = Настройки — усиления устройств, антенны, аудиоустройства
topbar-user-manual-f1 = Руководство пользователя (F1)
topbar-range-was-typed-settings-radio = Этот диапазон введён в Настройки ▸ Радио ▸ Диапазон приёма, а не сообщён радио. Расширьте его или очистите поле, чтобы использовать то, что устройство сообщает о себе.
topbar-what-device-reports-about-itself = Это то, что устройство сообщает о себе. Если радио действительно покрывает этот диапазон, укажите его реальный диапазон в Настройки ▸ Радио ▸ Диапазон приёма.
topbar-band = ДИАПАЗОН
topbar-mode = РЕЖИМ

## Top bar: the System box's window chips.

topbar-chip-log = ЖУРНАЛ
topbar-chip-awards = НАГРАДЫ
topbar-chip-bands = ДИАПАЗОНЫ
topbar-chip-sat = СПУТНИК
topbar-chip-ism = ISM
topbar-chip-mail = ПОЧТА
topbar-chip-mem = ПАМЯТЬ
topbar-chip-scan = СКАН
topbar-chip-hfdl = HFDL
topbar-chip-grid = КВАДРАТЫ

## Top bar: the mode-defaults chip.

topbar-changed-agc = AGC
topbar-changed-squelch = шумоподавитель
topbar-changed-auto-notch = авто-режекция
topbar-changed-stereo = стерео
topbar-changed-binaural = бинауральный
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    Вернуть значения по умолчанию для { $mode }. Изменено относительно них: { $changed }.

    Click to put them back. { $mode }'s own values return, and what you set here is forgotten.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = у радио нет настройки скорости { $baud } — вместо неё будет использовано { $default }
settings-radio-icom-scope-hint =
    Передаёт развёртку собственного спектроскопа радио по линии CI-V и рисует её как панораму — единственная картинка диапазона, которую может дать аппарат с демодулированным звуком, поскольку звук, который он посылает, уже прошёл через его фильтр.

    Needs the radio set up for it: CI-V USB Baud Rate to 115200 and CI-V USB Port to "Unlink from {"["}REMOTE]" (both under SET > Connectors > CI-V), and the baud rate above set to match — below { $min_baud } the sweeps do not fit down the link and the scope stays off.

    The sweeps share the radio's internal USB bus with its own sound card. If received audio starts to drop out with the scope on, this box is the first thing to try turning off.
settings-radio-rx-tx-words = { $rx } / TX { $tx }
settings-radio-soapy-native = Это оборудование напрямую поддерживается sdroxide: { $names }. Выбор этого интерфейса выше даёт его собственные настройки — ступени усиления, фильтры и режекции, которые SoapySDR не может выразить, — и не требует модуля SoapySDR.
settings-radio-rx888-width-hint = Какую часть оцифрованного спектра панорама показывает сразу. Вся цепочка DSP работает на этой ширине, поэтому более широкая стоит пропорционально больше ЦП — 1/2 — это весь диапазон на водопаде, и это серьёзный объём арифметики. Выше точки перехода на VHF ({ $crossover } МГц) панорама смотрит не на антенну, а на ПЧ тюнера, которая шириной 8 МГц и стоит на { $if_centre } МГц — поэтому всё шире { $widest } не может быть на ней отцентрировано, а лишняя ширина — мёртвый спектр рядом с сигналом, а не его больше.
settings-radio-sdrplay-serial-missing = Серийный номер { $serial } нет среди приёмников, которые сообщает служба SDRplay. Выберите один из перечисленных приёмников или переподключите этот и нажмите «Пересканировать».
settings-radio-hdr-filter-hint =
    Аналоговый фильтр перед трактом HDR — иной регулятор, чем собственная полоса приёмника. Каждая настройка построена на фиксированном наборе центров и ничего не может делать в других местах; этот построен на { $centres }. Применяется сразу.

    The path itself does not work yet — see the switch above — so this chooses a filter for a mode that is currently silent.
settings-radio-rspduo-only = Только у RSPduo есть второй тюнер, а выбранный здесь приёмник — { $model }, поэтому это ничего не сделает.
settings-radio-lime-tx-gain-low = Усиление передачи { $db } дБ, на самом нижнем краю диапазона — несколько микроватт с платы, которые на измерителе мощности будут показаны как ничего, что бы за ней ни стояло. Так задано по умолчанию, чтобы взведённый передатчик не мог вас удивить; поднимите его, прежде чем включать манипуляцию.

## Settings window, General: SWR

settings-general-swr-tune-limit = (настройка: { $limit }:1)

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Click to cycle meter face: { $faces }
topbar-step-down = Вниз { $step }
topbar-step-up = Вверх { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = Сдвиг ретранслятора { $shift } — приём на { $rx }, передача на { $tx } МГц{ $auto }
topbar-tone-and-burst = Передача с тоном { $tone } под голосом и пачка 1750 Гц { $ms } мс в начале каждого выхода
topbar-tone-under-voice = Передача с тоном { $tone } под голосом
topbar-burst-every-over = Пачка 1750 Гц { $ms } мс в начале каждого выхода
topbar-match-tx = КАК ПЕРЕДАЧА ({ $tone })
topbar-decimation-on = Децимация фронтенда: приёмнику дают средние 1/{ $n } из { $msps } Msps, которые передаёт это радио, — { $span }, а остальное отбрасывается до того, как что-либо дойдёт до приёмника.
topbar-decimation-off = Децимация фронтенда, выключено: приёмник видит весь { $span }, который передаёт это радио.
topbar-decimation-hint =
    { $what }

    A narrower span means finer waterfall resolution, a quieter noise floor (3 dB per halving) and less CPU — at the cost of the band either side of it. The dial still tunes anywhere; the radio moves its LO to follow.

    Click to cycle: off / 2 / 4 … { $max }.
topbar-rx-gain-hint = Усиление RX фронтенда ({ $stage }). Слишком много — АЦП приёмника ограничивает, и паразитные сигналы размазываются по диапазону; слишком мало — приёмник глохнет.
topbar-rx-gain-stages =
    {""}

    This rig has { $n } RX gain stages — the rest are in Settings → Device.
topbar-passband-now =
    {""}

    The passband is at { $dbfs } dBFS now.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    Пропускать звук ниже этой мощности в полосе приёма, в dBFS. Слева — открыто.{ $level } Установите выше шума и ниже сигнала.

    A stream that arrives with the radio's own AGC already in it — an Icom's 12 kHz IF, for one — sits far higher on this scale than an SDR's raw baseband does, which is why the rail reaches full scale.

    Click to cycle the strength: Low / Med / High / Off. The engine is in the rows below.
topbar-recording-both = Запись { $audio } и { $iq }
topbar-recording-audio = Запись звука в { $audio }
topbar-recording-iq = Запись I/Q в { $iq }
topbar-auto-record-armed = Автозапись взведена: файл на каждую передачу, закрывается после { $secs } с тишины
topbar-rds-hover = RDS: { $ps }. Нажмите для данных станции
topbar-drm-hover = DRM: { $service }. Нажмите для подробностей передачи
topbar-drm-no-codec = DRM: { $summary } — захвачен, но его аудио { $codec } здесь не декодируется. Нажмите, чтобы увидеть, чего не хватает
topbar-drm-state = DRM: { $summary } — нажмите для состояния декодера
topbar-hd-hover = HD Radio: { $station }. Нажмите для подробностей передачи
topbar-hd-state = HD Radio: { $summary } — нажмите для состояния декодера
topbar-binaural-preset = { $preset } в каждом ухе — { $lo } … { $hi } Гц, обе боковые
topbar-nr-hover =
    Шумоподавление: { $strength } — { $engine }

    Click to change engine or strength
topbar-clipping = Клип { $span } — нажмите ещё раз, чтобы начать заново
topbar-record-clip = Записать { $span } и остановить — клип для приложения к отчёту о приёме
topbar-starting-clip = запуск клипа { $span }…
topbar-stops-in = остановится через { $time }
topbar-auto-record-hint = Записывать каждую передачу в отдельный файл, закрывать после { $secs } секунд тишины
topbar-recording-silence = запись · { $secs } с тишины (закроется в { $hold })
topbar-tone-open = Принимается { $tone }, это тональный шумоподавитель — звук открыт
topbar-tone-closed = Принимается { $heard }, но тональному шумоподавителю нужен { $wanted } — звук остаётся закрытым
topbar-tone-heard = Принимается CTCSS/DCS { $tone }
topbar-tone-nothing = Тональный шумоподавитель { $tone }: ничего подходящего не принимается
topbar-tone-use = ИСП. { $tone }
topbar-sockets = Разъёмы: { $sockets }
topbar-voice-keyer-tx = Передача { $slot } — нажмите, чтобы открыть голосовой манипулятор
topbar-drive-uncal-gen = Эта частота, которая не относится ни к одному любительскому диапазону, не откалибрована, поэтому настройка доходит до передатчика целиком.
topbar-drive-uncal-band = { $band } не откалиброван, поэтому настройка доходит до передатчика целиком.
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = Эта частота, которая не относится ни к одному любительскому диапазону, откалибрована на { $trim } дБ, поэтому та же настройка даёт в эфире здесь на { $diff } дБ { $more ->
        [less] меньше
       *[more] больше
    }, чем на неоткалиброванном диапазоне.
topbar-drive-cal-band = { $band } откалиброван на { $trim } дБ, поэтому та же настройка даёт в эфире здесь на { $diff } дБ { $more ->
        [less] меньше
       *[more] больше
    }, чем на неоткалиброванном диапазоне.
topbar-drive-hover =
    Насколько сильно раскачивается передатчик. На радио, которое sdroxide модулирует сам, это масштабирует модулированные отсчёты; на аппарате с собственным управлением мощностью это доля номинальной мощности, которую запрашивают у аппарата.

    { $trim }

    Settings → Radio → Transmit drive by band.
topbar-cessb-hover =
    SSB с управляемой огибающей: больше средней мощности при том же пике без брызг — три-четыре децибела кажущейся громкости на другом конце. На сколько децибел голос загоняется в процессор; 0 — выключено, 6 — разумная первая попытка. Мощность Drive не затрагивает.

    Now: { $now }
topbar-waterfall-rate = { $rows } строк в секунду — { $seconds } секунд диапазона
topbar-detail-auto-columns = { $n } колонок, что выбрал бы здесь «Авто»
topbar-n-columns = { $n } колонок
topbar-fft-bins-whole = { $n } бинов на весь поток — по одному на колонку или точнее
topbar-fft-bins-pooled = { $n } бинов, объединённых в { $cols } колонок панорамы: резче сигналы, но не больше их
topbar-sat-locked = Спутник — захвачен на { $name }
topbar-filter-edges = { $lo } … { $hi } Гц от несущей
topbar-filter-isb = Фильтр приёма: { $width } Гц в каждом ухе ({ $edges }). Две боковые ISB — отдельные передачи, поэтому ширина — это ширина одной из них.
topbar-filter-width = Фильтр приёма: { $width } Гц шириной ({ $edges }).
topbar-filter-hint =
    { $what }

    Click for the width, the two edges and this mode's presets, as numbers — the panadapter's grips place a passband by eye, this is where an exact figure is typed.
topbar-band-outside-range =
    { $band } вне того, что принимает это радио ({ $ranges } МГц).

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band }: { $verdict } ({ $time ->
        [day] днём
       *[night] ночью
    }) — прогноз HamQSL.com по солнечным индексам, а не измерение вашего собственного пути.

## Top bar: short words and captions

topbar-squelch-open = открыто
topbar-off = выкл.
topbar-recording = запись
topbar-sql = SQL
topbar-stations = станции
topbar-caption-reaction = реакция
topbar-caption-scroll = прокрутка
topbar-caption-surface = поверхность
topbar-caption-flow = поток
topbar-caption-detail = детализация
topbar-floor = пол

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = САМОЛЁТЫ
pane-chart = ГРАФИК
pane-chat = ЧАТ
pane-decodes = ДЕКОДЫ
pane-files = ФАЙЛЫ
pane-heard = УСЛЫШАНО
pane-image = ИЗОБРАЖЕНИЕ
pane-map = КАРТА
pane-messages = СООБЩЕНИЯ
pane-monitor = МОНИТОР
pane-panel = ПАНЕЛЬ
pane-qso = QSO
pane-reading = ЧТЕНИЕ
pane-receive = ПРИЁМ
pane-saved = СОХРАНЕНО
pane-send = ПЕРЕДАЧА
pane-spots = СПОТЫ
pane-stations = СТАНЦИИ
pane-status = СТАТУС
pane-terminal = ТЕРМИНАЛ
pane-text = ТЕКСТ
pane-traffic = ТРАФИК
pane-vessels = СУДА
pane-wfall = ВОДОПАД

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}СОХРАНИТЬ{" "}
panels-nothing-decoded-save = Нечего сохранять — ничего не декодировано
panels-clear-rx = {" "}ОЧИСТИТЬ RX{" "}
panels-empty-receive-window-nothing-air = Очистить окно приёма. Ничто, что в эфире, не останавливается.
panels-nothing-received-clear = Нечего очищать — ничего не принято
panels-save-what-panel-has-decoded = Сохранить в файл то, что эта панель декодировала
panels-prop = PROP
panels-shade-map-where-signals-are = Затенять карту по тому, где сигналы реально проходят, на каждом диапазоне. Каждый приём помещается в середину своего пути — участок ионосферы, который его отразил, — а не на дальнюю станцию, поэтому это карта неба, а не места, где живут радиолюбители. Строится по трассам, в которых участвовала эта станция, плюс — когда Reverse Beacon Network включена в Настройки → Споты — то, что слышат скиммеры мира, а это покрывает диапазоны, на которых это радио не работает.
panels-night = НОЧЬ
panels-shade-where-sun-down-twilight = Затенять там, где Солнце за горизонтом, и сумерки между ними, чтобы серая линия была видна на карте. Низкие диапазоны уходят далеко, а высокие сжимаются на ночной стороне этой линии, и сама граница — это место DX.
panels-all-bands = ВСЕ ДИАПАЗОНЫ
panels-every-band-once-one-hue = Все диапазоны сразу, по одному оттенку на каждый — общие условия.
panels-one-band = ОДИН ДИАПАЗОН
panels-one-band-cold-hot-blue = Один диапазон, от холодного к горячему: синий, зелёный, жёлтый, красный.
panels-frequency-you-are-actually-working = Частота, на которой вы реально работаете: шкала плюс смещение тона рядом с ней. Именно она попадает в журнал — одна шкала даёт столько же ошибки.
panels-send-return = ОТПРАВИТЬ ПРИ ВОЗВРАТЕ
panels-freq = ⇵ ЧАСТ.
panels-nothing-agreed-mode-save-your = Для этого режима ничего не согласовано — сохраните свою ниже.
panels-forget-saved-frequency = Забыть эту сохранённую частоту
panels-dial-already-one-your-saved = Эта шкала уже есть среди ваших сохранённых частот.
panels-dial-already-agreed-list-mode = Эта шкала уже есть в согласованном списке для этого режима.
panels-decode-squelch-raise-stop-decoding = Шумоподавитель декодирования — поднимите, чтобы перестать декодировать шум
panels-message-repeats-long-transmit-held = Сообщение повторяется, пока удерживается передача, — метеор ловит ту часть, что проходит мимо.
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}ПЕРЕДАЧА ВКЛ{"  "}
panels-tx-2 = {"   "}TX{"   "}
panels-call-cq = {" "}ВЫЗОВ CQ{" "}
panels-nocall = NOCALL

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = Услышано слишком мало речи — говорите нормально весь отсчёт и попробуйте снова.
txaudio-tx-audio = TX Audio
txaudio-mode-not-voice-mode-so = Это не голосовой режим, поэтому то, что он посылает, не проходит через цепочку. Настройки, сделанные здесь, действуют в SSB, AM и FM.
txaudio-bypass = ОБХОД
txaudio-switch-every-stage-out-once = Отключить сразу каждую ступень, включая шумоподавитель, чтобы микрофон шёл необработанным. Нажмите ещё раз, чтобы восстановить именно те ступени, что были включены, — самое быстрое А/Б-сравнение того, что делает цепочка.
txaudio-denoise = ШУМОДАВ
txaudio-rnnoise-microphone-ahead-every-other = RNNoise на микрофоне, перед всеми остальными ступенями, чтобы шум помещения убирался раньше, чем гейт или компрессор его поднимут. Добавляет около 10 мс задержки.
txaudio-digital-voice = ЦИФРОВАЯ РЕЧЬ
txaudio-also-run-freedv-rade-digital = Также пропускать цифровую речь FreeDV / RADE через цепочку. По умолчанию выключено: вокодер обучен на естественной речи и может хуже декодироваться из сильно обработанного голоса, поэтому проверьте в эфире, прежде чем на него полагаться. Звук TCI и цифровые режимы через цепочку никогда не проходят.
txaudio-test = Проверка
txaudio-rec = {" "}⏺ ЗАП.{" "}
txaudio-play = {" "}▶ ВОСПР.{" "}
txaudio-play-test-phrase-through-chain = Воспроизвести тестовую фразу через цепочку в том виде, в каком она сейчас настроена, в ваши динамики. Меняйте во время воспроизведения и слышьте изменение. Звук приёма заменяется, пока оно играет.
txaudio-recording = запись…
txaudio-no-phrase-yet = фразы пока нет
txaudio-live = Живой
txaudio-live-2 = {" "}🎧 ЖИВОЙ{" "}
txaudio-hear-your-microphone-through-chain = Слышать свой микрофон через цепочку, пока говорите, не включая передатчик. Используйте наушники: через динамики будет обратная связь. Вы слышите себя с задержкой в несколько десятков миллисекунд, что делает разговор неловким — выставляйте уровни по измерителям и оценивайте звук через ЗАП. / ВОСПР. Звук приёма заменяется, пока оно работает; нажатие PTT останавливает его.
txaudio-meters = {" "}ИЗМЕРИТЕЛИ{" "}
txaudio-run-your-microphone-through-chain = Пропускать микрофон через цепочку только для измерителей: ничего в динамики, поэтому нет задержки, с которой нужно говорить, и нечему давать обратную связь. Так выставляют усиление микрофона и входа, порог гейта и то, насколько усердно работает компрессор. Звук приёма продолжается; нажатие PTT останавливает его.
txaudio-monitoring-live-use-headphones-speakers = Мониторинг в живом режиме — используйте наушники, иначе динамики дадут обратную связь в микрофон.
txaudio-voice-modes-only-tci-clients = Только голосовые режимы. Клиенты TCI и цифровые режимы обходят цепочку; когда она используется, держите собственный TX EQ и процессор радио плоскими.
txaudio-auto-set-stopped-microphone-was = Автоустановка остановлена: микрофон был занят.
txaudio-microphone-may-clipping-before-gets = {" "}Микрофон может ограничиваться ещё до того, как сигнал сюда попадёт — уменьшите его собственное усиление, если звучит резко.
txaudio-level = Уровень
txaudio-mic-control-same-one-transmit = Регулятор Mic — тот же, что на полосе передачи, общий для всех профилей. 0 дБ в его середине, +6 дБ наверху.
txaudio-mic = Mic
txaudio-gain-top-mic-microphone-stored = Усиление поверх Mic для этого микрофона, хранится с профилем. Пороги пресетов рассчитаны на голос с пиком около −10 dBFS на измерителе IN; Автоустановка выставляет его туда. Остаётся в пределах при ОБХОДЕ и применяется к цифровой речи тоже: это уровень, а не обработка.
txaudio-input = Вход
txaudio-auto-set = Автоустановка
txaudio-auto-set-cancelled = Автоустановка отменена.
txaudio-talk-now-you-would-air = Говорите сейчас, как будто в эфир…
txaudio-db = -∞ дБ
txaudio-chain = Цепочка
txaudio-order-audio-meets-them = По порядку, с которым их встречает звук
txaudio-switch-stage = Включить эту ступень
txaudio-earlier-chain = Раньше в цепочке
txaudio-later-chain = Позже в цепочке
txaudio-output-limiter = Выходной лимитер
txaudio-output = Выход
txaudio-trim-limiter-always-last = Подстройка и лимитер — всегда последние
txaudio-profiles = Профили…
txaudio-gate-expander = Гейт / экспандер
txaudio-pushes-room-noise-fan-rumble = Прижимает шум помещения, гул вентилятора и шипение в паузах. Выставьте порог чуть выше вашего шумового пола и ниже самых тихих слогов.
txaudio-gate = ГЕЙТ
txaudio-hard-gate-10-1-down = Жёсткий гейт: 10:1, до −40 дБ
txaudio-expander = ЭКСПАНДЕР
txaudio-gentle-expander-2-1-never = Мягкий экспандер: 2:1, никогда больше −15 дБ — помещение никогда не становится мёртвым
txaudio-reset = СБРОС
txaudio-back-defaults = Вернуть значения по умолчанию
txaudio-threshold = Порог
txaudio-below-gate-starts-close = Ниже этого гейт начинает закрываться.
txaudio-return = Возврат
txaudio-hysteresis-once-open-closes-only = Гистерезис: открывшись, он закрывается лишь на столько ниже порога, чтобы голос, колеблющийся у него, не заставлял гейт дрожать.
txaudio-hold = Удержание
txaudio-how-long-stays-open-after = Как долго он остаётся открытым после того, как голос пропал, чтобы окончания слов не обрезались.
txaudio-release = Спад
txaudio-how-fast-closes-once-hold = Как быстро он закрывается, когда удержание закончилось. Увеличьте, если гейт пульсирует.
txaudio-floor = Пол
txaudio-most-ever-turns-pauses-down = Насколько он вообще приглушает паузы — не больше.
txaudio-ratio = Соотношение
txaudio-how-steeply-closes-below-threshold = Насколько круто он закрывается ниже порога.
txaudio-look-ahead = Упреждение
txaudio-opens-much-before-word-starts = Открывается на столько раньше начала слова, чтобы его первая согласная не обрезалась. Добавляет одинаковую задержку ко всему передаваемому.
txaudio-high-pass-low-shelf-two = ВЧ-фильтр, низкочастотная полка, два пика, высокочастотная полка и ФНЧ. Перетаскивайте точку на кривой или задайте ниже. Выставьте ФНЧ по ширине вашего фильтра передачи.
txaudio-flat = РОВНО
txaudio-every-band-back-0-db = Каждый диапазон обратно на 0 дБ, оба края выключены
txaudio-freq = Част.
txaudio-gain = Усиление
txaudio-q-slope = Q / Наклон
txaudio-high-pass = ВЧ-фильтр
txaudio-removes-handling-noise-breath-pops = Убирает шум обращения, хлопки дыхания и фон ниже угла среза.
txaudio-low-shelf = Низкочастотная полка
txaudio-body-mud-cut-little-here = Тело, или грязь: немного срежьте здесь для голоса, который звучит коробчато.
txaudio-peak-1 = Пик 1
txaudio-bump-dip-voice-band-often = Подъём или провал в полосе голоса — часто срез около 300–500 Гц.
txaudio-peak-2 = Пик 2
txaudio-presence-lift-around-2-3 = Присутствие: подъём около 2–3 кГц — то, что пробивается на SSB.
txaudio-high-shelf = Высокочастотная полка
txaudio-brightness = Яркость.
txaudio-low-pass = ФНЧ
txaudio-stops-energy-above-transmit-filter = Не даёт энергии выше фильтра передачи тратиться в модуляторе.
txaudio-t-24-db-oct = 24 дБ/окт
txaudio-t-12-db-oct = 12 дБ/окт
txaudio-click-steeper-gentler-slope = Нажмите для более крутого или мягкого наклона
txaudio-de-esser = Де-эссер
txaudio-turns-down-only-sibilant-band = Приглушает только шипящую полосу, когда «с» и «ш» переходят порог, так что остальной голос сохраняет свой уровень. Найдите частоту, перебирая её и произнося «сестра серебряный целлофан», и остановитесь там, где шипение громче всего.
txaudio-frequency = Частота
txaudio-centre-sibilant-band = Центр шипящей полосы.
txaudio-width-band-higher-narrower = Ширина полосы — больше значит уже.
txaudio-band-level-where-reduction-starts = Уровень полосы, где начинается снижение. Слишком низко — работает всё время, притупляя голос.
txaudio-amount = Величина
txaudio-most-band-ever-turned-down = Насколько полоса вообще приглушается. Начните примерно с −6 дБ.
txaudio-attack = Атака
txaudio-how-fast-reacts = Как быстро реагирует.
txaudio-how-fast-lets-go = Как быстро отпускает.
txaudio-compressor = Компрессор
txaudio-brings-quiet-syllables-up-holds = Поднимает тихие слоги и сдерживает громкие. Цельтесь в 3–6 дБ снижения на пиках; больше 10 дБ — явное сжатие, лучше подходящее для контестов.
txaudio-level-above-which-starts-compress = Уровень, выше которого он начинает сжимать.
txaudio-how-hard-compresses-above-threshold = Насколько сильно он сжимает выше порога.
txaudio-how-fast-reacts-peak-10 = Как быстро он реагирует на пик. 10–25 мс сохраняют согласные чёткими.
txaudio-how-fast-recovers = Как быстро он восстанавливается.
txaudio-knee = Колено
txaudio-soft-knee-width-0-hard = Ширина мягкого колена; 0 — жёсткое колено.
txaudio-makeup = Компенсация
txaudio-gain-after-compression-bring-level = Усиление после сжатия, чтобы вернуть уровень.
txaudio-drive = Drive
txaudio-gain-ahead-compressor-so-more = Усиление перед компрессором, чтобы больше голоса пересекало порог — повышает среднюю мощность. Используйте вместе с выходным лимитером.
txaudio-phase = Фаза
txaudio-all-pass-phase-rotator-ahead = Всепропускающий фазовращатель перед детектором. Выравнивает несимметричные пики, которые есть у голоса, так что тот же уровень пика несёт больше средней мощности. 4 — распространённая вещательная настройка; 0 — выключено.
txaudio-last-stage-trim-look-ahead = Последняя ступень: подстройка и лимитер с упреждением, который удерживает каждый пик под потолком без брызг, которые даёт жёсткое ограничение. По умолчанию выключено, поэтому включение цепочки не меняет уровень drive, который вы уже задали.
txaudio-limiter = Лимитер
txaudio-on = Вкл.
txaudio-ceiling = Потолок
txaudio-no-peak-leaves-chain-above = Ни один пик не выходит из цепочки выше этого.
txaudio-trim = Подстройка
txaudio-output-gain-ahead-limiter = Выходное усиление, перед лимитером.
txaudio-off = {"  "}выкл.
txaudio-meters-move-while-you-transmit = Измерители двигаются, пока вы передаёте голосовую передачу, проигрываете тестовую фразу или работаете в ЖИВОЙ или ИЗМЕРИТЕЛИ.
txaudio-profiles-2 = Профили
txaudio-profile-whole-chain-every-stage = Профиль — это вся цепочка: настройки каждой ступени, какие включены, их порядок и входное усиление для вашего микрофона. Загрузка одного заменяет цепочку; регулятор Mic и drive остаются как есть.
txaudio-name = Название
txaudio-save = Сохранить
txaudio-store-chain-now-under-name = Сохранить цепочку в её нынешнем виде под этим названием, заменив профиль с тем же именем
txaudio-import = Импорт…
txaudio-read-profile-json-file = Прочитать профиль из файла .json
txaudio-tx-audio-profile = Профиль TX Audio
txaudio-no-profiles-yet-save-stores = Профилей пока нет. «Сохранить» запоминает цепочку, которую вы настроили сейчас.
txaudio-load = Загрузить
txaudio-export = Экспорт…
txaudio-delete = Удалить
txaudio-starting-points = Отправные точки
txaudio-set-your-level-auto-set = Сначала выставьте уровень через Автоустановку — пороги пресетов рассчитаны на неё, — а затем настраивайте отсюда, а не передавайте в загруженном виде: записывайте фразу и слушайте после каждого изменения. Пресет сохраняет ваше входное усиление.
txaudio-profile-loaded = Профиль загружен.

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = PROP
solar-where-signals-are-actually-getting =
    Где сигналы реально проходят, диапазон за диапазоном. Каждый приём, в котором участвует эта станция, — WSPR, FT8/FT4, JS8 и журнал — помещается в середину своего пути, то есть участка ионосферы, который его отразил, а не на дальнюю станцию. Поэтому это карта неба, а не место, где живут радиолюбители.

    It can only show paths this station has been one end of. Oceans light up because the midpoints of long paths fall there; Antarctica stays dark because nobody is transmitting from it. Off by default — it paints the whole planet.
solar-orbits = ОРБИТЫ
solar-orbital-paths = Орбитальные пути
solar-clouds = ОБЛАКА
solar-cloud-cover-noaa-s-global = Облачность из глобальной геостационарной мозаики NOAA, нарисованная как глубина воздуха, а не картинка, прилепленная к поверхности: инфракрасное излучение говорит, насколько холодна вершина каждого облака, а значит, на какой высоте она стоит. Грозы вспыхивают внутри башен, которые сами строят.
solar-planets = ПЛАНЕТЫ
solar-other-seven-planets-eighteen-their = Остальные семь планет, восемнадцать их спутников и кольца Сатурна и Урана. Положения планет берутся из кеплеровского набора элементов JPL; спутники — круговые орбиты, подогнанные под JPL Horizons.
solar-coronal-mass-ejection-trajectory-cones = Конусы траекторий корональных выбросов массы
solar-sun-obs = НАБЛЮД. СОЛНЦА
solar-solar-observations-sun-s-disk = Солнечные наблюдения на диске Солнца: активные области пятен и откуда пришли вспышки
solar-labels = ПОДПИСИ
solar-body-region-labels = Подписи тел и областей
solar-small-bodies = МАЛЫЕ ТЕЛА
solar-names-asteroids-comets-they-are = Названия на астероидах и кометах. Они рисуются в любом случае — это лишь вопрос, рисовать ли с ними тридцать пять обозначений. Что бы ни стояло, тело, на которое направлена камера, и всё, что совпало с окном поиска, остаются названными.
solar-decoded-ft8-ft4-stations-path = Декодированные станции FT8/FT4 и путь к работаемой станции
solar-sats = СПУТНИКИ
solar-amateur-radio-satellites-their-orbits = Любительские спутники, их орбиты и возвышение от вашего QTH
solar-aurora = СИЯНИЕ
solar-auroral-oval-noaa-s-ovation = Авроральный овал из модели OVATION NOAA, нарисованный как эмиссионные оболочки на их настоящих высотах, с экваториальным краем 10-процентного контура, отмеченным на поверхности
solar-awards = НАГРАДЫ
solar-award-coverage-every-dxcc-entity = Покрытие наград: каждое образование DXCC закрашено тем, чего ещё не хватает вашему журналу. Оранжевый горит там, где образование никогда не работалось, янтарный — где работалось, но не подтверждено, тусклый зелёный — где QSL вернулась. Следует фильтру диапазонов в окне НАГРАДЫ.
solar-view = ВИД
solar-which-body-camera-orbits-animated = Вокруг какого тела вращается камера, и анимированный тур
solar-layers = СЛОИ
solar-what-drawn-orbits-clouds-cmes = Что рисуется: орбиты, облака, CME, подписи, QSO, сияние
solar-which-sdo-channel-wraps-sun = Каким каналом SDO обёрнуто Солнце и насколько оно старое
solar-weather = ПОГОДА
solar-aurora-propagation-numbers-which-bands = Сияние, числа распространения и какие диапазоны открыты
solar-space-weather = КОСМИЧЕСКАЯ ПОГОДА
solar-nothing-measured-yet = пока ничего не измерено
solar-scale = МАСШТАБ
solar-body-moon-orbit-exaggeration = Преувеличение тела и орбиты Луны
solar-time = ВРЕМЯ
solar-scrub-whole-scene-forward-back = Перематывать всю сцену вперёд и назад
solar-activity = АКТИВНОСТЬ
solar-replay-last-hour-decodes-globe = Воспроизвести последний час декодов на глобусе
solar-what-propagation-heat-map-showing = Что показывает тепловая карта распространения
solar-propagation = РАСПРОСТРАНЕНИЕ
solar-auto = ▶ АВТО
solar-fly-spline-through-set-framed = Пролететь сплайном через набор кадрированных точек обзора. Любой ввод мышью отменяет его.
solar-moons-ride-circular-orbits-fitted = Спутники движутся по круговым орбитам, подогнанным под JPL Horizons: в пределах градуса-двух от того, где они реально находятся, и до шести для Миранды, плоскость орбиты которой качается слишком быстро для круга.
solar-fetch-everything-again-now = Загрузить всё заново сейчас
solar-all-sats = ВСЕ СПУТНИКИ
solar-show-every-satellite-element-set = Показывать каждый спутник набора элементов, а не только популярные. Кольца орбит остаются у немногих выборочных — девяносто сразу нечитаемы.
solar-starting = запуск…
solar-body = тело{" "}
solar-moon-orbit = орбита Луны{" "}
solar-stretch-earth-moon-distance-so = Растянуть расстояние Земля → Луна, чтобы пару можно было увидеть отдельно
solar-sun = Солнце{" "}
solar-sun-radius-exaggeration-leave-1 = Преувеличение радиуса Солнца. Оставьте 1× — чтобы геометрия CME оставалась читаемой: раздутое Солнце проглатывает основание каждого конуса.
solar-show = ПОКАЗАТЬ
solar-paint-globe-what-getting-through = Закрасить глобус по тому, что проходит
solar-paints-globe-what-getting-through = Закрашивает глобус по тому, что проходит, на каждом диапазоне. Каждый приём помещается в середину своего пути — где ионосфера выполнила работу — а не на дальнюю станцию. Строится по тому, что слышит эта станция, плюс скиммеры мира, когда включена Reverse Beacon Network.
solar-display = ДИСПЛЕЙ
solar-all-bands = ВСЕ ДИАПАЗОНЫ
solar-every-band-once-one-hue = Все диапазоны сразу, по одному оттенку на каждый, смешивая там, где два перекрываются. Вид для вопроса «каковы условия», а не «открыт ли двадцатиметровый».
solar-one-band = ОДИН ДИАПАЗОН
solar-one-band-cold-hot-blue = Один диапазон, от холодного к горячему: синий, зелёный, жёлтый, красный.
solar-band = ДИАПАЗОН
solar-nothing-heard-yet = пока ничего не слышно
solar-bands = ДИАПАЗОНЫ
solar-sources = ИСТОЧНИКИ
solar-contacts-logbook-path-was-open = Контакты в журнале. Путь, который был открыт, без отчёта о сигнале, достойного названия, — RST — это не SNR, поэтому эти контакты учитываются в загруженности ячейки и никогда — в её запасе.
solar-stations-reported-hearing-one-downloaded = Станции, сообщившие, что слышат эту, загруженные из WSPRnet. Единственная обратная связь, которую вообще получает передающий маяк.
solar-reverse-beacon-network-skimmers-what = Скиммеры Reverse Beacon Network: что слышат все остальные, на каждом диапазоне, включая те, на которых это радио не работает. Грубее остального — строка RBN не несёт локаторов, поэтому оба конца помещаются в центр страны, что близко для маленькой страны и сильно не так для большой. Включается в Настройки → Споты.
solar-decodes-station-made-each-measured = Декодирования, которые сделала эта станция. Каждое измеряется относительно порога своего режима, поэтому отчёт WSPR и отчёт FT4 сравнимы.
solar-memory = ПАМЯТЬ
solar-half-life = полураспад{" "}
solar-how-long-reception-takes-count = Как долго приём учитывается наполовину. Собственная память ионосферы короткая — открытие двухчасовой давности не должно спорить с тем, что было две минуты назад.
solar-live = ЖИВЬЁМ
solar-follow-band-happens = Следить за диапазоном по мере того, как это происходит
solar-replay = ⏸ ПОВТОР
solar-replay-2 = ▶ ПОВТОР
solar-replay-last-hour-decodes-over = Повторять последний час декодирований снова и снова
solar-min-ago = {" "}мин назад
solar-where-last-hour-globe-showing = Что в последний час показывает глобус
solar-trail = след{" "}
solar-how-long-decode-s-arc = Как долго дуга декодирования остаётся на глобусе позади головы
solar-speed = скорость{" "}
solar-how-much-faster-than-real = Насколько повтор идёт быстрее реального времени
solar-qso-layer-off = слой QSO выключен
solar-no-decodes-yet = декодирований пока нет
solar-qth-not-set-enter-your = QTH не задан — введите ваш квадрат в Настройках
solar-mode = РЕЖИМ
solar-path = ПУТЬ
solar-sent = ПЕРЕДАНО
solar-rcvd = ПРИНЯТО
solar-grid = КВАДРАТ
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = Геостационарный — всегда над вашим горизонтом.
solar-no-passes-next-48-hours = Пролётов в ближайшие 48 часов от вашего QTH нет.
solar-start = НАЧАЛО
solar-max-el = МАКС. ВОЗВ.
solar-aos-los-are-azimuths-horizon = AOS/LOS — это азимуты на горизонте. Время в UTC.
solar-locked-unlock = {" "}● ЗАХВАЧЕН — РАЗБЛОК.{" "}
solar-release-satellite-lock = Снять захват спутника
solar-lock = {" "}ЗАХВАТ{" "}
solar-track-satellite-main-window-doppler = Сопровождать этот спутник в главном окне: приём и передача с поправкой на доплер, восходящая линия по транспондеру, управление ротатором
solar-no-frequencies-file-one-add = Для этого спутника частот в файле нет — добавьте их в Настройки ▸ TLE.
solar-frequencies-yours = ЧАСТОТЫ · ВАШИ
solar-frequencies = ЧАСТОТЫ
solar-link = СВЯЗЬ
solar-downlink-mhz = НИСХОДЯЩАЯ (МГц)
solar-uplink-mhz = ВОСХОДЯЩАЯ (МГц)
solar-doppler-shifts-these-few-khz = Доплер сдвигает их на несколько кГц за пролёт НОО.
solar-satellite-planet-comet = спутник, планета или комета
solar-planet-asteroid-comet = планета, астероид или комета
solar-clear-search = Очистить поиск
solar-no-match = совпадений нет
solar-no-sounder = нет зондировщика
solar-heard = УСЛЫШАНО ≥
solar-kp = Kp / A
solar-x-ray = Рентген
solar-observed-above-sounder-better-than = наблюдается над зондировщиком — лучше модели
solar-power-n-s = мощность С/Ю
solar-edge-n-s = край С/Ю
solar-kp-peak-24-h = пик Kp за 24 ч
solar-observed-forecast = наблюдаемое | прогноз
solar-bands-open = ДИАПАЗОНЫ ОТКРЫТЫ
solar-dxcc-coverage = ПОКРЫТИЕ DXCC
solar-ir-vis = ИК+ВИД
solar-ir-only = только ИК
solar-no-deep-convection = глубокой конвекции нет
solar-t-1-storm = 1 шторм
solar-glancing = {" "}· скользящий
solar-direction-estimated = {" "}· направление оценено

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = Настройка APRS
setup-packet-setup = Настройка пакетного режима
setup-ft8-ft4-ft2-setup = Настройка FT8 / FT4 / FT2
setup-my-callsign = Мой позывной
setup-my-grid = Мой квадрат
setup-station-call = Позывной станции
setup-ssid-oe3jjs-10-packet-station = С SSID — OE3JJS-10. Пакетная станция по традиции имеет SSID, отличный от оператора, и ничего не передаётся, пока это не задано.
setup-speed = Скорость
setup-t-9600-needs-radio-s-data = 9600 требует порт данных радио. Тракт с микрофоном и динамиком губит его с обоих концов — это проблема радио, а не sdroxide.
setup-tx-delay = Задержка TX
setup-flags-sent-before-frame-so = Флаги, отправляемые перед кадром, чтобы удалённый конец мог услышать нас и захватить свой тактовый генератор. На аппарате CAT одна лишь sdroxide тратит 165–240 мс на выход в эфир, и сверху добавляется собственное время готовности передатчика аппарата.
setup-packet-length = Длина пакета
setup-bytes = {" "}байт
setup-most-single-frame-carries-shorter = Наибольшее, что несёт один кадр. Короткие кадры переживают слабый путь — переотправляется только тот кадр, который пострадал, — но стоят больше накладных расходов. 128 на КВ, 256 там, где путь надёжен.
setup-frames-sent-before-waiting-acknowledgement = Кадры, отправляемые до ожидания подтверждения. Больше — заполняет хороший путь; на плохом — больше переотправлять.
setup-extended = Расширенный
setup-ask-mod-128 = Запрашивать mod-128
setup-extended-sequence-numbers-window-bigger = Расширенные порядковые номера для окна больше семи. Многие узлы отклоняют запрос сообщением DM, которое выглядит точно как станция, не желающая с вами говорить, — поэтому оставьте выключенным, если не знаете, что удалённому концу это нужно.
setup-answer-calls = Отвечать на вызовы
setup-accept-connections = Принимать соединения
setup-off-winlink-client-which-dials = Выключено для клиента Winlink, который вызывает сам и не имеет причины отвечать. Включено, чтобы быть доступным как равный: вызов приходит в панель терминала, и вы говорите с тем, кто его сделал. Станция, чья связь уже занята сеансом Winlink, отклоняет вызовы, пока тот не закончится.
setup-connect-text = Текст при соединении
setup-what-caller-greeted = чем приветствуют вызывающего
setup-sent-station-connects-us-once = Отправляется станции, которая подключается к нам, как только связь установлена. Пусто — ничего не отправляется, что выглядит поломкой для того, кто вызвал.
setup-default-via = Маршрут по умолчанию
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = Путь через диджипитеры, с которым стартует терминал. Путь к вашему локальному узлу всегда одинаков, и если набирать его заново, то хоп легко пропустить.
setup-beacon = Маяк
setup-t-0-disables-timer = 0 отключает таймер
setup-beacon-text = текст маяка
setup-kiss-server = Сервер KISS
setup-serve = Обслуживать
setup-offer-modem-kiss-tnc-socket = Предложить этот модем как TNC KISS через сокет, чтобы Pat, клиент APRS или стек Linux AX.25 могли использовать радио.
setup-aprs-call = Позывной APRS
setup-callsign-station-beacons-under-ssid = Позывной, под которым станция передаёт маяк, с её SSID — OE3JJS-9 для автомобиля, -10 для I-gate, -5 для телефона. Оставьте пустым, чтобы использовать позывной выше.
setup-no-callsign-station-will-not = Нет позывного: эта станция вообще не передаёт — ни маяк, ни сообщение, ни подтверждение.
setup-symbol = Символ
setup-two-characters-protocol-carries-them = Два символа, как их несёт протокол: таблица (`/` или `\`) и код символа. Цифра или буква на первой позиции — это наложение, рисуемое поверх значка альтернативной таблицы.
setup-path = Путь
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = Насколько далеко вы просите ретранслировать. Это самая значимая настройка на канале: каждый хоп умножает передачи всей сети на одной общей частоте.
setup-position = Позиция
setup-my-grid-2 = Из моего квадрата
setup-beacon-centre-locator-above-reported = Передавать маяк из центра указанного выше локатора, с той неопределённостью, которая у локатора действительно есть — шестисимвольный охватывает пару километров в поперечнике, и честно сказать об этом. Снимите галочку, чтобы вместо этого давать точные координаты.
setup-latitude = Широта
setup-longitude = Долгота
setup-locator = из локатора
setup-not-coordinate = не координата
setup-comment = Комментарий
setup-sent-every-beacon = отправляется с каждым маяком
setup-t-0-default-never-beacons-timer = 0 — по умолчанию — никогда не передаёт маяк по таймеру. Тридцать минут — традиция для стационарной станции; подвижная передаёт чаще, но каждый маяк — это время чужого канала.
setup-compressed = Сжатый
setup-compressed-position-format-third-air = Формат сжатой позиции: треть времени в эфире и точнее. Его читает каждый приёмник с 1990-х.
setup-first-goes-out-one-interval = Первый уходит через один интервал от сейчас, а не сразу.
setup-messages = Сообщения
setup-acknowledge = Подтверждать
setup-answer-messages-addressed-you-acknowledgement = Отвечать на сообщения, адресованные вам. Подтверждение — это передача, которую станция делает без запроса, поэтому настройка только для приёма должна его выключить — и тогда маяк перестанет утверждать, что с вами можно связаться.
setup-flags-sent-ahead-every-frame = Флаги, отправляемые перед каждым кадром, чтобы модем удалённого конца услышал несущую и захватил свой тактовый генератор до начала данных. Это должно пережить всё между нажатием передачи и излучением: одна sdroxide тратит на аппарате CAT 165–240 мс, а радио, принимающее звук по сети, добавляет сверху буферизацию. Слишком мало — и удалённый конец никогда не захватит: передача есть, а декодировать её некому.
setup-shared-packet-mode = общее с пакетным режимом
setup-keep-stations = Хранить станции
setup-how-long-station-stays-map = Как долго станция остаётся на карте после того, как её последний раз слышали. Также от этого отсчитывается затухание на карте, поэтому короткое окно показывает только то, что живо.
setup-auto-reply = Автоответ
setup-answer-snr-grid-status = Отвечать на SNR? / GRID? / STATUS?
setup-answer-direct-question-addressed-you = Отвечать на прямой вопрос, адресованный вам или @ALLCALL, ответом, а не подтверждением. Никогда не отвечает на трафик другой станции и никогда не отвечает самой себе.
setup-heartbeat = Heartbeat
setup-off = Выкл.
setup-turbo-does-not-beacon-local = Turbo не передаёт маяк — это локальная скорость и скорость VHF.
setup-sends-your-callsign-grid-so = Отправляет ваш позывной и квадрат, чтобы другие знали, что с вами можно связаться. Первый уходит через один интервал от сейчас, а не сразу.
setup-beacon-frequency = Частота маяка
setup-t-500-1000-hz = 500–1000 Гц
setup-move-each-beacon-free-slot = Переносить каждый маяк на свободный слот в поддиапазоне heartbeat, как это делает JS8Call: там ищут маяки станции, и это не даёт ненаблюдаемому передатчику попасть в чужое QSO. Слот выбирается, когда маяк действительно уходит, вне всего, что декодируется.
setup-working-freq = Рабочая частота
setup-beacon-where-you-are-working = Вместо этого передавать маяк там, где вы работаете. Против конвенции диапазона, но всё, что вы передаёте, собирается в одном месте.
setup-heartbeat-reply = Ответ на heartbeat
setup-answer-heartbeats-signal-report = Отвечать на heartbeat отчётом о сигнале
setup-tell-station-beaconed-how-well = Сообщать станции, передавшей маяк, как хорошо вы её приняли. По умолчанию выключено: загруженный диапазон несёт heartbeat в каждом слоте, и ответ на все перегрузил бы как раз тот диапазон, который heartbeat призваны держать тихим. Ограничено одним ответом на станцию раз в 15 минут и никогда — пока ещё приходит сообщение или у вас что-то стоит в очереди на отправку.
setup-status-message = Сообщение статуса
setup-tx-period = Период TX
setup-even = Чётный
setup-odd = Нечётный
setup-auto-sequence = Авто-последовательность
setup-decode-depth = Глубина декодирования
setup-fast = Быстро
setup-normal = Обычно
setup-deep = Глубоко

    Fast — one pass, no signal subtraction: quickest, fewest decodes.
    Normal — flat multi-pass subtraction: a little quicker than Deep, a little less thorough.
    Deep — the checkpointed multi-pass: the most decodes, and ~1.2 s on a busy slot.
setup-auto-tx-frequency = Авто-частота TX
setup-choose-transmit-frequency-automatically-quietest = Выбирать частоту передачи автоматически: самое тихое место в периоде, в котором вы передаёте, а не частоту станции, которой отвечаете. Выключено, это НЕ удерживает её — ответ идёт на частоте вызываемой станции. Чтобы удерживать, используйте «Удержать TX».
setup-hold-tx-frequency = Удержать частоту TX
setup-pin-transmit-tone-where-nothing = Закрепить тон передачи там, где он есть. Ничто его не двигает: ни ответ станции, ни очередь вызовов, ни вызов CQ, ни клик по декодированию или водопаду. Перекрывает авто-частоту TX. Смена диапазона — единственное исключение: сдвиг, который вы задали последним на новом диапазоне, возвращается вместе с ним. Для случаев, где ваша лицензия уже плана диапазонов — на британском диапазоне 60 м на шкале 5357 кГц распределение заканчивается на 5358,0, поэтому тон должен оставаться ниже 1000 Гц.
setup-tx-watchdog = Сторожевой таймер TX
setup-stop-transmitting-after-long-no = Прекращать передачу после такого времени без ответа и без действий с вашей стороны. 0 отключает.
setup-give-up-after = Сдаться после
setup-calls = {" "}вызовов
setup-contest = Контест
setup-everyday-exchange-grid-report-rr73 = Повседневный обмен: квадрат, отчёт, RR73.
setup-european-vhf-contests-send-signal = Европейские контесты VHF: отправляется отчёт о сигнале, серийный номер и ваш ШЕСТИСИМВОЛЬНЫЙ локатор в формате сообщения, который WSJT-X называет EU VHF Contest. CQ уходит как CQ TEST.
setup-serial = Серийный
setup-serial-number-next-exchange-will = Серийный номер, который понесёт следующий обмен. Он сам продвигается по мере записи каждого контакта и запоминается между сессиями — задавайте его вручную, только чтобы начать контест или продолжить с того места, где дошёл бумажный журнал.
setup-needs-6-character-grid = ⚠ нужен шестисимвольный квадрат
setup-eu-vhf-contest-messages-carry = Сообщения EU VHF Contest несут шестисимвольный локатор (JN88DD, а не JN88). Пока у «Моего квадрата» его нет, отправляется обычный обмен.
setup-dxpedition = DXpedition
setup-ordinary-ft8-operation = Обычная работа FT8.
setup-calling-dxpedition-running-fox-mode = Вызов DXpedition, работающей в режиме Fox: вызывайте выше 1000 Гц, переходите вниз на Fox, когда он ответит, и логируйте по его RR73, не отправляя 73.
setup-run-pile-up-several-signals = Работа пайл-апа: несколько сигналов сразу, очередь вызывающих, работа от самых сильных и редких. CALL CQ запускает его, STOP QSO останавливает.
setup-fox-signals = Сигналы Fox
setup-once = {" "}сразу
setup-simultaneous-transmissions-spaced-60-hz = Одновременные передачи с разносом 60 Гц. Они делят мощность передатчика, поэтому больше сигналов — каждый слабее.
setup-tx-audio = Звук TX
setup-deviation-fm = девиация, на FM
setup-drive-ssb = drive, на SSB
setup-grid = Квадрат
setup-report = Отчёт
setup-r-report = R+Отчёт

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = , шкала 10 Вт, пересчитана
amp-t-10-w-scale = , шкала 10 Вт
amp-t-2-w-scale-rescaled = , шкала 2 Вт, пересчитана
amp-t-2-w-scale = , шкала 2 Вт
amp-t-100-w-scale-rescaled = , шкала 100 Вт, пересчитана
amp-t-100-w-scale = , шкала 100 Вт
amp-t-20-w-scale-rescaled = , шкала 20 Вт, пересчитана
amp-t-20-w-scale = , шкала 20 Вт
amp-rescaled = , пересчитана
amp-and = {" "}И{" "}
amp-not-while-transmitting-switching-relay = Не во время передачи: переключение реле с ВЧ на нём может его повредить. Сначала снимите с передачи.
amp-no-amplifier-set-up-choose = Усилитель не настроен. Выберите его порт в Настройки → Усилитель.
amp-settings = НАСТРОЙКИ
amp-dock = ЗАКРЕПИТЬ
amp-put-amp-window-back-inside = Вернуть окно AMP внутрь главного окна
amp-pop-out = ВЫНЕСТИ
amp-give-amp-window-window-own = Дать окну AMP собственное окно, чтобы переносить на другой экран
amp-rig = , с аппарата
amp-rig-flagged-reading = , аппарат пометил своё показание
amp-drv = , с DRV
amp-rig-s-swr = , с КСВ аппарата
amp-ptt-keys-amplifier-radio-off = PTT включает усилитель вместе с радио; OFF оставляет его в обходе. Нажмите, чтобы переключить.
amp-switch-antenna-jack = Переключить антенный разъём
amp-band-down = Диапазон вниз
amp-band-up = Диапазон вверх
amp-tuner = ТЮНЕР
amp-tuned = Настроен
amp-tune = НАСТРОИТЬ
amp-start-tune-then-key-steady = Запустите настройку — затем включите ровную несущую (CW, FM или RTTY). Нажмите ещё раз, чтобы остановить.
amp-put-tuner-line-bypass = Включить тюнер в тракт или обойти его
amp-tuning = Настройка…
amp-bottom = НИЗ
amp-pick-any-reading-either-needle = Выберите любое показание для любой стрелки. Верхняя стрелка белая, нижняя — синяя.
amp-forward-power-measured-radio-amplifier = Прямая мощность, измеряется радио: усилитель в OFF её не сообщает
amp-forward-power-taken-drv-amplifier = Прямая мощность, берётся с DRV: усилитель в OFF её не сообщает, а в обходе мощность радио проходит прямо
amp-forward-power = Прямая мощность
amp-reflected-power-measured-radio-amplifier = Отражённая мощность, измеряется радио: усилитель в OFF её не сообщает
amp-reflected-power-worked-out-fwd = Отражённая мощность, вычисляется по FWD и КСВ: усилитель в OFF её не сообщает
amp-reflected-power = Отражённая мощность
amp-standing-wave-ratio-red-past = Коэффициент стоячей волны: красный на пределе настройки или за ним (Настройки → Общие). Проверьте антенну
amp-standing-wave-ratio-measured-radio = Коэффициент стоячей волны, измеряется радио: усилитель в OFF его не сообщает
amp-standing-wave-ratio = Коэффициент стоячей волны
amp-drive-radio = Drive с радио
amp-drain-supply = Напряжение стока
amp-drain-current = Ток стока
amp-temp = ТЕМП.
amp-heat-spreader-temperature-yellow-49 = Температура теплораспределителя: жёлтый от 49 °C (120 °F), красный выше 65 °C (149 °F), отключается выше 70 °C (158 °F). Нажмите для °C/°F.
amp-c = °C
amp-f = °F
amp-performance = ХАРАКТЕРИСТИКИ
amp-numbers-hold-peaks-bars-are = числа держат пики · полосы живые
amp-input-power = ВХОДНАЯ МОЩНОСТЬ
amp-supply-volts-drain-amps = Напряжение питания × ток стока
amp-efficiency = КПД
amp-output-power-input-power = Выходная мощность ÷ входная мощность
amp-gain = УСИЛЕНИЕ
amp-output-power-over-drive-decibels = Выходная мощность относительно drive, в децибелах
amp-off-line-over-temperature = ОТКЛЮЧЕНО · перегрев
amp-connected-transmitting = ПОДКЛЮЧЕН · ПЕРЕДАЁТ
amp-connected = ПОДКЛЮЧЕН
amp-connecting = Подключение…
amp-atu-bypassed = ATU В ОБХОДЕ
amp-no-inductance-needed = индуктивность не нужна
amp-no-capacitance-needed = ёмкость не нужна
amp-c-out-antenna-side = C OUT · сторона антенны
amp-c-transmitter-side = C IN · сторона передатчика

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = Полученные фрагменты (светлые) и ещё не полученные (тёмные)
sstv-sstv = SSTV
sstv-auto = Авто
sstv-signal = Сигнал
sstv-station-sending-mode-sdroxide-does = Станция передаёт в режиме, которого у sdroxide нет. Сигнал и настройка в порядке — просто декодера для этого режима нет. С вашей стороны чинить нечего.
sstv-listening = прослушивание…
sstv-restart-rx = Перезапустить RX
sstv-abandon-picture-being-received-listen = Отказаться от принимаемого изображения и слушать следующий заголовок. Для передачи, которая начала декодироваться в неверном режиме — иначе приёмник занят, пока этот режим не закончится.
sstv-callsign-last-station-sent-fsk = Позывной, который последняя станция отправила как FSK ID после своего изображения.
sstv-tx-slant = Наклон TX
sstv-transmit-clock-trim-ppm-remove = Подстройка тактового генератора передатчика (ppm), чтобы убрать наклон на декодере удалённого конца
sstv-reset-0-ppm = Сбросить на 0 ppm
sstv-fsk-id = FSK ID
sstv-send-your-callsign-tones-after = Отправлять ваш позывной тонами после каждого изображения — идентификацию, которую читают ретрансляторы SSTV и другие программы. Добавляет около 2,5 секунды и ничего не отправляет, пока не задан позывной.
sstv-tx-lead = Опережение TX
sstv-silence-sent-after-keying-before = Тишина, отправляемая после включения и до преамбулы изображения и кода VIS. Декодер, пропустивший любую часть этого заголовка, не показывает никакого изображения, поэтому это покрывает промежуток между запросом PTT у аппарата и его реальным выходом в эфир. 0 — для SDR, который включается мгновенно.
sstv-live = ЖИВЬЁМ
sstv-waiting-picture = ожидание изображения…
sstv-no-low-audio = нет звука / низкий звук
sstv-received = ПРИНЯТО
sstv-reading-radio-s-pictures = Чтение изображений радио…
sstv-decoded-pictures-collect-here = Декодированные изображения собираются здесь.
sstv-click-enlarge-right-click-delete = Нажмите, чтобы увеличить · правый клик, чтобы удалить
sstv-delete-picture = Удалить это изображение
sstv-loading = загрузка…
sstv-transmit = ПЕРЕДАТЬ
sstv-click-edit-slot-s-message = Нажмите, чтобы изменить сообщение этого слота · двойной клик, чтобы загрузить изображение
sstv-change-image = Сменить изображение…
sstv-load-image = Загрузить изображение…
sstv-clear = Очистить
sstv-empty-slot-s-picture-message = Очистить изображение этого слота (сообщение остаётся)
sstv-banner = Баннер…
sstv-what-printed-across-top-every = Что печатается поперёк верха каждого изображения, которое отправляет эта станция
sstv-fetching-slot-s-picture = Загрузка изображения этого слота…
sstv-load-image-into-slot = Загрузить изображение в этот слот →
sstv-drawn-slot-s-image = Рисуется поверх изображения этого слота
sstv-abort-tx = {" "}ПРЕРВАТЬ TX{" "}
sstv-delete-picture-store = Удалить это изображение из хранилища
sstv-received-image = Принятое изображение
sstv-no-longer-store = больше нет в хранилище
sstv-loading-full-size = загрузка в полном размере…
sstv-save-picture = Сохранить изображение…
sstv-delete-sure = Удалить — уверены?
sstv-click-again-delete-good = Нажмите ещё раз, чтобы удалить навсегда
sstv-delete = Удалить…
sstv-image-banner = Баннер изображения
sstv-drawn-into-every-picture-station = Рисуется в каждом изображении, которое передаёт эта станция, поверх собственного сообщения слота.
sstv-draw-banner = Нарисовать баннер
sstv-substituted-when-picture-composed = Подставляется при сборке изображения:
sstv-top-left = Слева сверху
sstv-top-right = Справа сверху
sstv-colours = Цвета
sstv-strip-top-edge-fades-black = Полоса у верхнего края — она переходит в чёрный или в цвет градиента ниже
sstv-both-texts = Оба текста
sstv-gradient = Градиент
sstv-fade = переход к
sstv-fade-strip-top-colour-second = Переводить полосу от её верхнего цвета ко второму, а не к чёрному.
sstv-text-outline = Контур текста
sstv-draw-outline-around-banner-text = Рисовать контур вокруг текста баннера, чтобы он оставался читаемым на загруженной или светлой полосе.
sstv-text-gradient = Градиент текста
sstv-fade-banner-text-colour-second = Переводить текст баннера от его цвета ко второму по ширине изображения.
sstv-rainbow-text = Радужный текст
sstv-all-text-overriding-colours-above = весь текст, перекрывая цвета выше
sstv-draw-every-text-picture-banner = Рисовать каждый текст на изображении — баннер и сообщение — горизонтальной радугой, игнорируя цвета выше.
sstv-message = Сообщение
sstv-slot-message-s-text-colour = Цвет текста сообщения слота
sstv-outline = контур в
sstv-height = Высота
sstv-how-tall-strip-pixels-transmitted = Насколько высока полоса, в пикселях передаваемого изображения. От неё задаётся размер текста — кадр SSTV всего 320 пикселей в ширину, поэтому более высокий баннер делает его читаемым на другом конце.
sstv-reset = Сброс
sstv-back-callsign-left-version-right = Вернуть позывной слева и версию справа
sstv-waiting-station-s-digital-mode = Ожидание загрузки настроек цифрового режима станции…
sstv-rifp = RIFP
sstv-size = Размер
sstv-calling-frequency-draft-names = Частота вызова, которую называет черновик
sstv-encode = Кодирование
sstv-how-picture-encoded-into-object = Как изображение кодируется в объект, который несёт RIFP. Авто пробует каждое и отправляет наименьшее.
sstv-try-every-encoding-send-smallest = Пробовать каждое кодирование, отправлять наименьшее (никогда с потерями)
sstv-gray = Серый
sstv-grayscale-depth-rifp-s-raster = Глубина оттенков серого. Растр RIFP по определению в оттенках серого — в его манифесте цвету места нет.
sstv-dither = Дизеринг
sstv-diffuse-quantisation-error-worth-below = Рассеивание ошибки квантования — стоит того ниже 8 бит
sstv-caption = Подпись
sstv-sent-manifest-content-hint-shown = Отправляется в манифесте как подсказка содержимого и показывается под изображением приёмниками. Передаётся как текст — в изображение не рисуется.
sstv-what-picture = Что это за изображение
sstv-repeat-data = Повтор данных
sstv-send-every-data-frame-many = Отправлять каждый кадр данных столько раз. RIFP — односторонний, без запросов на исправление, поэтому это единственное восстановление, которое получает приёмник.
sstv-chunk = Фрагмент
sstv-payload-octets-per-data-frame = Октеты полезной нагрузки на кадр данных (профиль рекомендует 192)
sstv-valid-frames-frames-failed-crc = Верные кадры, кадры с ошибкой CRC и полностью проверенные изображения
sstv-no-transfer-progress = передача не идёт
sstv-forget-incomplete-transfer = Забыть эту незавершённую передачу

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = Вызвать
windows-delete = Удалить
windows-edit-name-frequency-mode-antenna = Изменить название, частоту, режим, антенну и настройку ретранслятора
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = Сдвиг ретранслятора, тон и пачка 1750 Гц, сохранённые с этим каналом
windows-abandon-edit-escape = Отменить правку (Escape)
windows-save = СОХРАНИТЬ
windows-keep-changes-enter = Сохранить изменения (Enter)
windows-recall-channel-without-moving-antenna = Вызвать этот канал, не переключая антенну
windows-antenna-switch-when-channel-recalled = Антенна, на которую переключиться при вызове этого канала. — оставляет то, на чём уже стоит радио.
windows-shift = Сдвиг
windows-tone = Тон
windows-invert = ИНВЕРСИЯ
windows-normal = ОБЫЧНЫЙ
windows-dcs-polarity = Полярность DCS
windows-open-every-over-tone-burst = Открывать каждый выход пачкой тона, пока этот канал вызван
windows-memories = Память
windows-memory-name = название ячейки
windows-store = Сохранить
windows-folder-name = название папки
windows-new-folder = Новая папка
windows-import = ИМПОРТ
windows-read-channel-list-chirp-csv =
    Прочитать список каналов из CSV CHIRP (.csv) — экспорт каталога ретрансляторов, таблицу морских или PMR каналов или что-либо ещё, что может записать CHIRP.

    Each channel brings its frequency, mode, repeater shift and CTCSS or DCS tone with it. Channels already on this list — same frequency, same mode — are skipped, so re-importing an updated directory adds what is new rather than doubling what is not.
windows-chirp-csv = CSV CHIRP
windows-export = ЭКСПОРТ
windows-write-list-out-chirp-csv = Записать этот список в CSV CHIRP, чтобы загрузить в портативную рацию или сохранить как резервную копию.
windows-no-memories-yet = ячеек памяти пока нет
windows-drag-memory-onto-folder-file = перетащите ячейку на папку, чтобы положить её туда, или ниже них, чтобы вынуть
windows-drop-here-unfile = бросить сюда, чтобы вынуть
windows-sort = Сортировка
windows-order-they-were-stored = В порядке, в котором сохранялись
windows-name-ignoring-case = По названию, без учёта регистра
windows-frequency = По частоте
windows-band-then-frequency-inside-each = По диапазону, затем по частоте внутри каждого
windows-descending-click-ascending = По убыванию — нажмите для возрастания
windows-ascending-click-descending = По возрастанию — нажмите для убывания
windows-delete-folder-memories-move-top = Удалить папку — её ячейки переходят на верхний уровень
windows-rename-folder = Переименовать папку
windows-empty-drop-memories-here = пусто — бросьте сюда ячейки
windows-voice-keyer = Голосовой манипулятор
windows-rec-records-your-microphone-play = REC записывает с вашего микрофона, PLAY позволяет прослушать записанное, TX выводит в эфир — как и клавиша цифровой клавиатуры, MIDI-пэд или send_voice_mem в rigctld.
windows-stop-store = Остановить и сохранить
windows-stop = СТОП
windows-play = ИГРАТЬ
windows-stop-listening = Прекратить прослушивание
windows-nothing-recorded-slot = В этом слоте ничего не записано
windows-not-while-transmitting = Не во время передачи
windows-listen-message-nothing-transmitted = Прослушать это сообщение — ничего не передаётся
windows-stop-transmitting = Остановить передачу
windows-tune-active-switch-off-first = НАСТРОЙКА активна — сначала выключите её
windows-transmit-message = Передать это сообщение
windows-erase-recording = Стереть эту запись
windows-rade-message-encoded-digital-voice = RADE: сообщение кодируется цифровым голосовым кодеком точно так же, как было бы закодировано живое выступление.

## TX Audio window: messages with values

txaudio-rec-hint = Записать тестовую фразу с микрофона — до { $max } с, без включения передатчика. Нажмите ещё раз, чтобы остановить. Она сохраняется необработанной, поэтому ИГРАТЬ может прогнать ту же фразу через каждое изменение, которое вы делаете.
txaudio-phrase-length = Фраза { $secs } с
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = Усиление входа установлено на { $gain } дБ (пики голоса были { $loud } dBFS).{ $clip }
txaudio-listening = Слушаю… { $secs } с
txaudio-auto-set-hint = Говорите обычно { $secs } с — так, как говорили бы в эфир — и усиление входа задаётся так, чтобы пики голоса были около { $target } dBFS. Нажмите ещё раз, чтобы отменить.
txaudio-eq-point = { $hz } Гц, { $db } дБ — перетащите, чтобы переместить
txaudio-not-a-profile = Это не профиль TX Audio: { $error }
txaudio-imported = Импортировано «{ $name }».
txaudio-saved = Сохранено «{ $name }».
txaudio-deleted = Удалено «{ $name }».
txaudio-loaded-preset = Загружена отправная точка «{ $name }».

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · не в сети
solar-radius-exaggeration = Преувеличение радиуса Земли/Луны (макс. { $max }× при этом масштабе орбиты Луны)
solar-brightest-cell = самая яркая ячейка ≈ { $peak } путей · { $bands } { $bands ->
        [one] диапазон
        [few] диапазона
       *[other] диапазонов
    }
solar-hits-in-hour = { $hits } за час
solar-passes-from = { $name } · ПРОЛЁТЫ ОТ { $grid }
solar-elements-age = элементы возрастом { $age } · SGP4
solar-point-at = Направить на { $az }° ({ $compass }), возвышение { $el }°.
solar-published-as = опубликовано как { $name }
solar-fly-there = { $name } — ↵ чтобы лететь туда
solar-sats-tracked = сопровождается { $n } из { $total }
solar-bodies-found = найдено тел: { $n } из { $total }
solar-sats-and-bodies = сопровождается { $n } из { $total } · тел { $bodies } из { $bodies_total }
solar-confidence-on-band = { $confidence } на { $band }
solar-kp-now = Kp { $kp } сейчас
solar-kp-in = Kp { $kp } через { $hours } ч
solar-valid-age = действует { $valid } · возраст { $age }
solar-full-bar = полная полоса = { $share } мира
solar-own-paths-memory = куда ходили пути этой станции · память { $minutes } мин
solar-n-storms = штормов: { $n }
solar-clouds-legend = ОБЛАКА { $time } · возраст { $age } · { $channels } · { $storms } · молния смоделирована
solar-cme-eta = ETA { $eta } (+{ $hours } ч)
solar-cme-arrived = прибытие было { $eta } ({ $hours } ч назад)
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = CME, НАПРАВЛЕННЫЙ НА ЗЕМЛЮ { $time } · { $speed } км/с · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = Настройка { $mode }
setup-transmits-as = Передаёт как { $call }
setup-decimal-degrees = Десятичные градусы, отрицательные для { $hemisphere }. Вставьте прямо с карты.

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } НЕНАДЁЖНО ПРИ ЭТОМ КСВ
amp-lower-drive-first = Сначала уменьшите drive: { $drive } Вт перегрузит включённый усилитель (его предел — { $limit } Вт).
amp-live-value = живое { $value } { $unit }
amp-tripped = СРАБОТАЛ{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = топ ≈ { $n } путей
panels-no-agreed-frequency = Для { $mode } частота не согласована — откройте, чтобы сохранить ту, на которой вы сейчас
panels-mode-frequencies-hint = { $n } частот для { $mode } — выбор одной настраивает шкалу, и вы можете сохранить свою
panels-mode-frequencies = { $mode } · частоты
panels-global-convention = Глобальная конвенция, которую план диапазонов IARU, Регион { $region }, не относит к узким данным — проверьте свой план диапазонов, прежде чем передавать здесь.
panels-outside-data-segment = Янтарный: вне сегмента данных Региона { $region }.
panels-save-dial = ＋ Сохранить { $mhz } МГц
panels-save-dial-hint = Запомнить эту шкалу под { $mode } и предлагать её здесь отныне. Сохраняется на станции, поэтому её видит каждый экран, подключённый к этому радио.
panels-tr-period = Период T/R { $period } с
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = Этот ход: { $into } с из { $slot }. Декодирования появляются в его конце, и именно тогда может начаться следующая передача.{ $mark }
panels-burst-mark = {" "}Отметка — там, где заканчивается пачка, через { $secs } с.
panels-tr-period-burst = Период T/R { $slot } с, пачка { $burst } с

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } { $n ->
        [one] канал
        [few] канала
       *[other] каналов
    }
windows-slot-n = Слот { $n }
windows-record-from-mic = Запись с микрофона (до { $max } с)

## SSTV panel: messages with values

sstv-picture-too-big = Это изображение — { $size } МБ; предел — { $limit } МБ.
sstv-auto-mode = Авто ({ $mode })
sstv-not-decoded = { $what } — не декодировано
sstv-last = последний: { $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = Декодированные изображения сохраняются { $place } и собираются здесь.
sstv-older-in-store = В хранилище ещё { $n } старых
sstv-older-load-more = Ещё { $n } старых — загрузить больше
sstv-delete-picture-from = Удалить это изображение { $place }
sstv-rifp-profile = { $profile } · занято ~{ $khz } кГц · шкала — центр канала
sstv-rifp-too-wide = ⚠ { $profile } занимает ~{ $khz } кГц — слишком широко для узкополосного сегмента
sstv-rifp-no-frequency = RIFP не назначает частоту, и sdroxide передаст его там, где вы настроитесь. Канал { $khz } кГц помещается только там, где разрешена широкополосная или FM работа — { $segments } — и не в узкополосном сегменте, тем более на КВ. Даже внутри них условия вашей собственной лицензии могут быть уже. Вы оператор; проверьте свои правила.
sstv-rifp-tx-frame = ● Кадр TX { $frame }/{ $frames } · осталось { $secs } с
sstv-rifp-sent-as = отправлено как { $encoding } · { $octets } октетов
sstv-rifp-rx-stats = кадры { $frames } · плохих { $bad } · изображения { $pictures }
sstv-rifp-session-idle = сессия { $session } · простой { $secs } с
sstv-rifp-session-waiting = сессия { $session } · фрагменты удержаны, всё ещё ждём манифест · простой { $secs } с
sstv-store-on-radio = на радио, в { $dir }
sstv-store-in = в { $dir }

## decodes (src/app/panels/decodes.rs)

decodes-decodes = ДЕКОДИРОВАНИЯ
decodes-adif = ADIF
decodes-sort = Сортировка
decodes-single-list = Один список
decodes-every-decode-one-list-newest = Каждое декодирование в одном списке, сначала самый свежий ход, вместо блоков чётных/нечётных ходов. Кнопки сортировки тогда упорядочивают весь список, а не каждый ход, и каждая строка несёт время своего слота — голубой для чётного слота, золотой для нечётного.
decodes-cq-only = Только CQ
decodes-only-stations-calling-cq-only = Только станции, вызывающие CQ — и только те вызовы, на которые вы можете ответить: направленный CQ (DX, EU, JA, POTA, TEST …) показывается, когда называет вас, и скрывается, когда называет кого-то другого.
decodes-new-only = Только новые
decodes-only-stations-would-new-entity = Только станции, которые были бы новыми: образование, диапазон-слот, квадрат или позывной
decodes-auto-tx-frq = Авто-частота TX
decodes-overridden-hold-tx-lift-hold = Перекрыто «Удержать TX». Снимите удержание, чтобы выбрать, в какую сторону сдвигается частота передачи.
decodes-hold-tx = Удержать TX
decodes-pin-transmit-tone-where-nothing =
    Закрепить тон передачи там, где он есть. Ничто его не двигает: ни ответ станции, ни очередь вызовов, ни вызов CQ, ни клик по декодированию или водопаду. Выключите, чтобы сдвинуть, и включите снова.

    Changing band is the one exception, and it is your own act: the offset you last set on the new band comes back with it, so a 60 m figure is not carried onto 20 m and 20 m's is not carried onto 60 m.

    For where the licence is narrower than the band plan — on a UK 60 m dial of 5357 kHz the allocation ends at 5358.0, so the tone has to stay under 1000 Hz, and either automatic mover will walk out of the band between one over and the next.
decodes-type-transmit-offset-hz-then = Введите сдвиг передачи в Гц, затем нажмите Enter. Если ваша лицензия уже плана диапазонов, весь диапазон ниже границы в вашем распоряжении, поэтому здесь ничего не предлагается.
decodes-clear-rx = {" "}ОЧИСТИТЬ RX{" "}
decodes-empty-decode-list-nothing-air = Очистить список декодирований. Ничто, что в эфире, не останавливается.
decodes-even = ЧЁТ
decodes-text = ТЕКСТ
decodes-dxcc = DXCC
decodes-band = ДИАПАЗОН
decodes-grid = КВАДРАТ
decodes-dupe = ПОВТОР
decodes-reply = ОТВЕТ
decodes-queued-click-remove = В очереди — нажмите, чтобы убрать
decodes-work-station-after-current-one = Работать эту станцию после текущей
decodes-qsos-worked-since-sdroxide-was = QSO, проведённые с момента запуска sdroxide
decodes-qso-complete = ✓ QSO ЗАВЕРШЕНО
decodes-tx = ● TX
decodes-dxpedition-mode-transmit-frequency-held = Режим DXpedition. Частота передачи удерживается вне половины Fox полосы пропускания (ниже 1000 Гц), пока Fox не ответит.
decodes-watchdog = СТОРОЖ
decodes-transmitting-stopped-no-reply-no = Передача остановлена: за период сторожа не было ни ответа, ни действия. Вызовите CQ или выберите сообщение, чтобы продолжить.
decodes-well-inside-tolerance = Хорошо внутри допуска.
decodes-no-active-qso-pick-decode = нет активного QSO — выберите декодирование, чтобы ответить, или вызовите CQ
decodes-ft8-engine-idle = Движок FT8 простаивает
decodes-being-worked = работают
decodes-clear = ОЧИСТИТЬ
decodes-no-messages = — сообщений нет —
decodes-call-cq = {"  "}ВЫЗОВ CQ{"  "}
decodes-stop-qso = {" "}СТОП QSO{" "}
decodes-stop-tx = {" "}СТОП TX{" "}
decodes-free-text-13-chars = свободный текст (13 символов)
decodes-send = ОТПРАВИТЬ

## sat (src/app/sat.rs)

sat-pass-profile = ПРОФИЛЬ ПРОЛЁТА
sat-satellite = СПУТНИК
sat-satellites = СПУТНИКИ
sat-passes = ПРОЛЁТЫ
sat-set-your-grid-passes-settings = Задайте ваш квадрат для пролётов (Настройки ▸ Общие).
sat-next = СЛЕД.
sat-peak = ПИК ≥
sat-all-satellites = все спутники
sat-off-only-satellites-frequencies-file = Выкл.: только спутники с частотами в файле — те, с которыми можно работать, — и любые, что вы вставили сами. Вкл.: каждый спутник в списке, а их могут быть тысячи при подписке, и поиск занимает время.
sat-no-passes-window-lower-peak = В этом окне пролётов нет. Снизьте пик или расширьте окно.
sat-length = ДЛИТЕЛЬНОСТЬ
sat-peak-2 = ПИК
sat-from-to = ОТ → ДО
sat-up-now = ● сейчас над горизонтом
sat-unlock = РАЗБЛОК.
sat-release-lock-dial-stays-where = Снять захват; шкала остаётся на месте
sat-azimuth = АЗИМУТ
sat-elevation = ВОЗВЫШЕНИЕ
sat-range = ДАЛЬНОСТЬ
sat-rate = СКОРОСТЬ
sat-downlink = НИСХОДЯЩАЯ
sat-doppler-rx = ДОПЛЕР RX
sat-uplink = ВОСХОДЯЩАЯ
sat-doppler-tx = ДОПЛЕР TX
sat-no-pass-inside-next-48 = В ближайшие 48 часов от вашего QTH пролётов нет.
sat-elements-stale-corrections-suspended-until = ⚠ Элементы устарели — поправки приостановлены до обновления TLE
sat-doppler-correction-needs-iq-front = Поправка на доплер требует фронтенд IQ — аппарат CAT отслеживает только
sat-rotator-not-connected = Ротатор: не подключён
sat-doppler = ДОПЛЕР
sat-apply-computed-correction-rx-tx = Применить вычисленную поправку к RX и TX
sat-rotator = РОТАТОР
sat-steer-antenna-through-rotctld-client = Наводить антенну через клиент rotctld (Настройки ▸ Серверы)
sat-name-norad = название или NORAD
sat-set-your-grid-passes-settings-2 = задайте ваш квадрат для пролётов (Настройки ▸ Общие)
sat-norad = NORAD
sat-pass = ПРОЛЁТ
sat-always-visible = всегда видим
sat-not-here = отсюда не виден
sat-no-elements = нет элементов
sat-pick-satellite-see-links-lock = Выберите спутник, чтобы увидеть его линии и захватить.
sat-no-frequencies-file-one-add = Для этого спутника частот в файле нет — добавьте их в Настройки ▸ TLE.
sat-link = СВЯЗЬ
sat-downlink-mhz = НИСХОДЯЩАЯ (МГц)
sat-uplink-mhz = ВОСХОДЯЩАЯ (МГц)
sat-mode = РЕЖИМ
sat-tune = НАСТРОИТЬ
sat-set-dial-mode-link-no = Установить шкалу и режим на эту связь — без захвата и поправок
sat-locked = {" "}ЗАХВАЧЕН{" "}
sat-lock = {" "}ЗАХВАТ{" "}
sat-track-satellite-doppler-correction-rx = Сопровождать этот спутник: поправка доплера на RX и TX, восходящая линия по транспондеру, и трёхмерный вид следует за ним

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ СТОП{" "}
wefax-end-chart-now-save-what = Завершить карту сейчас и сохранить то, что пришло
wefax-start = {" "}● СТАРТ{" "}
wefax-start-chart-now-without-waiting = Начать карту сейчас, не дожидаясь стартового тона
wefax-phasing = фазирование…
wefax-subcarrier-offset-1900-hz-tune = Смещение поднесущей от 1900 Гц. Настраивайте примерно на ноль: несущая факса — 1500 Гц для чёрного и 2300 Гц для белого, а приёмник, ушедший на несколько сотен герц, обрезает изображение до сплошного чёрного или сплошного белого.
wefax-auto-start = АВТОСТАРТ
wefax-auto-stop = АВТОСТОП
wefax-end-450-hz-stop-tone = Завершать по стоп-тону 450 Гц. Выключите, чтобы продолжать запись через станцию, которая шлёт несколько карт подряд.
wefax-phase = ФАЗА
wefax-slant = НАКЛОН
wefax-sample-clock-trim-if-chart = Подстройка тактового генератора отсчётов. Если карта наклонена влево, увеличьте это; вправо — уменьшите. Звуковая карта, ушедшая на сотню ppm, уводит карту на четверть часа почти на линию в сторону.
wefax-view = ВИД
wefax-scale-chart-panel-width = Масштабировать карту по ширине панели
wefax-whole = ЦЕЛИКОМ
wefax-shrink-chart-until-all-view = Уменьшить карту, пока вся она не окажется в виде сразу
wefax-half-size = Половина размера
wefax-one-screen-pixel-per-fax = Один пиксель экрана на пиксель факса — прокручивайте для деталей
wefax-twice-size-scroll-move-around = Двойной размер; прокрутите, чтобы перемещаться по карте
wefax-height = ВЫСОТА
wefax-stretch-picture-vertically-chart-came = Растянуть изображение по вертикали. Карта, которая вышла сжатой или растянутой, обычно декодируется с неверной частотой строк — это делает её читаемой, а чипы LPM исправляют это по-настоящему. Двойной клик, чтобы ввести значение.
wefax-reset = СБРОС
wefax-back-picture-s-own-proportions = Вернуть собственные пропорции изображения
wefax-follow = СЛЕДОВАТЬ
wefax-keep-newest-lines-view-scrolling = Держать самые новые строки в виде. Прокрутка вверх выключает это, чтобы можно было прочитать уже пришедшее; прокрутка до низа включает снова.
wefax-saved = СОХРАНЕНО
wefax-path = ПУТЬ
wefax-reading-radio-s-charts = Чтение карт радио…
wefax-finished-charts-collect-here = Готовые карты собираются здесь.
wefax-delete-chart = Удалить эту карту
wefax-loading = загрузка…
wefax-stations = СТАНЦИИ
wefax-broadcast-radiofax-schedules-picking-one = Расписания радиофакса вещательных станций — выбор одного настраивает шкалу и задаёт скорость строк
wefax-frequencies-are-published-carrier-dial = Частоты — опубликованные несущие; шкала уходит на 1,9 кГц ниже, это сделано за вас. Расписания меняются, а станции закрываются — считайте это местом, с которого начинать поиск, а не расписанием.
wefax-delete-chart-store = Удалить эту карту из хранилища
wefax-newer = ◀ НОВЕЕ
wefax-chart-received-after-one = Карта, принятая после этой
wefax-older = СТАРШЕ ▶
wefax-chart-received-before-one = Карта, принятая до этой
wefax-no-longer-store = больше нет в хранилище
wefax-loading-full-size = загрузка в полном размере…
wefax-save = СОХРАНИТЬ
wefax-save-copy-chart-computer = Сохранить копию этой карты на этом компьютере
wefax-sure = УВЕРЕНЫ?
wefax-click-again-delete-chart-good = Нажмите ещё раз, чтобы удалить эту карту навсегда
wefax-delete = УДАЛИТЬ
wefax-file-chart-was-saved = Файл, под которым сохранена эта карта

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = запуск модема APRS…
aprs-aprs = APRS
aprs-no-callsign = нет позывного
aprs-nothing-will-transmitted-no-beacon = Ничего не будет передано — ни маяк, ни сообщение, даже не подтверждение — пока у этой станции нет позывного. Задайте его в Настройки → Общие или отдельный для APRS с его SSID в НАСТРОЙКА.
aprs-no-position = нет позиции
aprs-beacon-needs-somewhere-report-fill = Маяку нужно, где сообщать. Введите ваш локатор в Настройки → Общие или задайте координаты в НАСТРОЙКА.
aprs-setup = НАСТРОЙКА
aprs-callsign-symbol-digipeater-path-position = Позывной, символ, путь через диджипитеры, позиция и маяк
aprs-how-often-beacon-your-position = Как часто передавать маяк с вашей позицией без присмотра. `off` — по умолчанию — маяк никогда не передаётся: выбор режима не должен выводить станцию в эфир. Тридцать минут — традиция для стационарной станции, чаще — для подвижной, и каждый маяк — это время чужого канала.
aprs-beacon = {" "}МАЯК{" "}
aprs-station-has-no-callsign-see = У этой станции нет позывного — см. предупреждение слева.
aprs-station-has-no-position-report = У этой станции нет позиции для отчёта — см. предупреждение слева.
aprs-send-one-position-report-now = Отправить один отчёт о позиции сейчас, не дожидаясь интервала. Он ждёт, пока канал освободится, как и всё остальное.
aprs-stations = СТАНЦИИ
aprs-nothing-heard-yet = пока ничего не слышно
aprs-heard-direct = слышно напрямую
aprs-two-characters-station-sent-say = Два символа, которые станция отправила, чтобы сказать, что она такое
aprs-killed = УДАЛЁН
aprs-station-put-object-here-has = Станция, которая поставила здесь этот объект, отменила его
aprs-sender-deliberately-reported-square-rather = Отправитель намеренно сообщил квадрат, а не точку. Карта рисует квадрат.
aprs-digipeaters-last-frame-came-through = Диджипитеры, через которые прошёл последний кадр. `*` отмечает тот, что действительно его ретранслировал.
aprs-message = {" "}СООБЩЕНИЕ{" "}
aprs-address-message-box-station = Адресовать окно сообщения этой станции
aprs-center = {" "}ЦЕНТР{" "}
aprs-put-station-middle-map-double = Поместить эту станцию в середину карты. Двойной клик по карте возвращает вид автоматической подгонке.
aprs-channel = КАНАЛ
aprs-messages = СООБЩЕНИЯ
aprs-frames-arrived-failed-their-check = Кадры, которые пришли и не прошли проверку последовательности — столкновение, замирание или слишком слабый сигнал. Рост счётчика, когда ничего не декодируется, означает, что модем *слышит* канал: проверьте измеритель уровня и откройте шумоподавитель радио.
aprs-frames-read-cleanly-off-channel = Кадры, прочитанные с канала без ошибок, которые не были APRS — чужая пакетная сессия или формат, который эта сборка не декодирует.
aprs-every-frame-channel-arrived-other = Каждый кадр в канале, как он пришёл — включая чужой трафик и всё, что декодер не смог прочитать.
aprs-nothing-channel-yet = на канале пока ничего нет
aprs-no-messages = Сообщений нет.
aprs-waiting-channel = ожидание канала
aprs-sent-retried-still-waiting-acknowledged = отправлено и повторено, всё ещё ждём подтверждения
aprs-air-waiting-acknowledged = в эфире, ждём подтверждения
aprs-acknowledged-far-end = подтверждено удалённым концом
aprs-refused-far-end-not-retried = отклонено удалённым концом — не повторяется
aprs-no-ack = нет подтверждения
aprs-no-answer-after-every-retry = нет ответа после всех повторов
aprs-send = {" "}ОТПРАВИТЬ{" "}
aprs-station-has-no-callsign-so = У этой станции нет позывного, поэтому ничего нельзя передать. Задайте его в Настройки → Общие или отдельный для APRS с его SSID в НАСТРОЙКА.
aprs-needs-station-address-something-say = Нужна станция, которой адресовать, и что сказать.
aprs-send-keep-retrying-until-far = Отправить и продолжать повторять, пока удалённый конец не подтвердит. Сообщения — единственное, на что на этом канале отвечают.
aprs-busy = ЗАНЯТО
aprs-stations-heard-still-inside-window = Станции, которые слышны и всё ещё в окне, заданном в Настройке APRS, и есть ли прямо сейчас на канале другая станция. Пока занято, ничего не включается.
aprs-tx = ● TX
aprs-over-progress-frames-waiting-channel = Выход в процессе, кадры, ждущие освобождения канала, или время до следующего запланированного маяка.
aprs-audio-reaching-modem-flat-means = Звук, поступающий в модем. Ровная линия означает, что ничего не приходит вовсе — проверьте, что выход данных радио именно тот, который слушает sdroxide. Загорается зелёным, пока модем слышит несущую.

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = запуск декодера VDL2…
vdl2-aircraft-datalink = авиационный канал передачи данных
vdl2-tune-middle-vdl2-group-decoder = Настройтесь на середину группы VDL2. Собственное окно декодера оттуда сдвигается, чтобы охватить столько из четырнадцати каналов, сколько может достать приёмник.
vdl2-setup = НАСТРОЙКА
vdl2-which-channels-listen-how-hard = На каких каналах слушать, насколько сильной должна быть пачка и сколько журнала хранить
vdl2-save-vdl2-message-log-file = Сохранить журнал сообщений VDL2 в файл
vdl2-vdl2-channel = Канал VDL2
vdl2-messages = СООБЩЕНИЯ
vdl2-address-flight-registration-label = адрес, рейс, регистрация или метка
vdl2-held = УДЕРЖАНО
vdl2-hold = УДЕРЖАТЬ
vdl2-hold-message-log-where-so = Удержать журнал сообщений на месте, чтобы одно можно было прочитать, не выталкивая его вверх следующей передачей. Декодер продолжает работать, счётчики продолжают двигаться; отпускание показывает всё, что пришло в это время.
vdl2-nothing-decoded-yet = пока ничего не декодировано
vdl2-nothing-matches-filter = ничто не совпадает с фильтром
vdl2-stations = СТАНЦИИ
vdl2-nobody-heard-yet = пока никого не слышно
vdl2-vdl2-setup = Настройка VDL2
vdl2-channels = Каналы
vdl2-every-25-khz-slot-136 = Каждый слот 25 кГц от 136,650 до 136,975 МГц. По одному понижающему преобразователю на каждый, все внутри одного окна приёмника. Выключение одного экономит немного процессорного времени; оно не делает остальные чувствительнее.
vdl2-burst-threshold = Порог пачки
vdl2-above-each-channel-s-own = выше собственного шумового пола каждого канала
vdl2-lower-catches-weaker-transmissions-costs = Меньше ловит более слабые передачи и тратит процессорное время на шум; больше — пропускает их. Пол выучивается для каждого канала, и изменение здесь его не выбрасывает.
vdl2-keep-log = Хранить в журнале
vdl2-track-most = Отслеживать не более
vdl2-longest-silent-go-first = сначала уходят самые долго молчавшие
vdl2-forget-station-after = Забыть станцию после
vdl2-nothing-heard = без единого сигнала от неё
vdl2-show-unread-payloads = Показывать непрочитанные полезные нагрузки
vdl2-hex = в hex
vdl2-frames-carrying-x-25-clnp = Кадры, несущие X.25, CLNP или прикладные протоколы канала передачи данных над ними. SDRoxide называет их и показывает байты, а не читает их, — а скрытие показало бы, какая часть трафика это составляет.
vdl2-registration = Регистрация
vdl2-flight = Рейс
vdl2-label = Метка
vdl2-block = Блок
vdl2-sequence = Последовательность
vdl2-mode = Режим
vdl2-more-blocks-follow = следуют ещё блоки
vdl2-check = Проверка
vdl2-message-crc-good = CRC сообщения верен
vdl2-message-crc-not-checked = CRC сообщения не проверялся
vdl2-parity = Чётность
vdl2-link-control-no-payload = управление линией, без полезной нагрузки
vdl2-name = ИМЯ
vdl2-addr = АДР.
vdl2-type = ТИП
vdl2-msgs = СООБЩ.

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = Тон, который принимается, и тон, который передаётся — в CW это одна и та же частота. Нажмите на водопад, чтобы перенести её на сигнал.
cw-down-10-hz = Вниз на 10 Гц
cw-up-10-hz = Вверх на 10 Гц
cw-qrg-main-readout-tuning-line = QRG: главный индикатор и линия настройки стоят на частоте, с которой работают. Нажмите, чтобы вместо этого показывалась шкала, с тоном ниже неё — как показывает большинство радио.
cw-put-main-readout-tuning-line =
    Поставить главный индикатор и линию настройки на сигнал, а не на шкалу, чтобы показанная частота была той, которую назвали бы оба оператора, а линия настройки стояла в середине полосы пропускания. Настройка не меняется; меняются только числа.

    Clicking a signal lands it on the cursor only as closely as the click step allows — Controls → click-to-tune rounding, 10 Hz by default. A coarse step leaves the signal off the pitch by up to half of it, and the readout will say so.
cw-sending-speed-read-off-signal = Скорость передачи считывается с сигнала
cw-signal-noise-500-hz-same = Сигнал к шуму в полосе 500 Гц — то же число, что приводит отчёт
cw-how-far-off-cursor-signal = Насколько сигнал реально отстоит от курсора. Декодер следует за ним; полоса пропускания — нет, поэтому подтолкните курсор, если разница растёт.
cw-listening = — прослушивание —
cw-tx = ● TX
cw-nothing-copied-yet = — пока ничего не принято —
cw-key-send-characters-you-send = Манипулируйте, чтобы отправить — знаки, которые вы посылаете, появляются здесь.
cw-type-line-return-sends = Введите строку, Return отправит…
cw-type-here-send = Введите здесь для отправки…
cw-key = {" "}КЛЮЧ ●{" "}
cw-key-2 = {" "}КЛЮЧ{" "}
cw-hold-key-bound-cw-straight = Удерживайте клавишу, назначенную CW straight key — по умолчанию Space, а любую на ваш вкус в Настройки → Управление, — как прямой ключ: вниз, пока она удерживается, вверх при отпускании, вместо набора текста. Окно передачи блокируется, пока это включено, и весь манипулятор отдан ключу: любой текст в очереди сбрасывается.

    To hand-key it, set CW keying to "Sound card (MCW)" in Settings → Radio. The rig is then held on a sideband and the keyer's own sidetone is transmitted as audio, which is the route the straight key drives.
cw-watchdog = СТОРОЖ
cw-straight-key-was-held-down = Прямой ключ удерживался слишком долго — потерянное отпускание, а не рука — поэтому несущая сброшена и передача выключена. Нажмите ключ снова, чтобы продолжить.
cw-tx-2 = {"  "}TX ВКЛ{"  "}
cw-tx-3 = {"   "}TX{"   "}
cw-send-what-box-now-without = Отправить то, что сейчас в поле, не дожидаясь Return
cw-hold-key-down-between-characters = Удерживать ключ между знаками, чтобы ничего набранного не ждало
cw-call-cq = {" "}ВЫЗОВ CQ{" "}
cw-nocall = NOCALL
cw-clear = {" "}ОЧИСТИТЬ{" "}
cw-stop-sending-drop-whatever-has = Прекратить отправку и сбросить всё, что не ушло. При включённом прямом ключе вместе с этим уходит и обратная проверка того, что вы манипулировали.
cw-sidetone = САЙДТОН
cw-play-keyed-tone-through-computer = Проигрывать манипулируемый тон через динамики этого компьютера в дополнение к отправке, чтобы слышать то, что вы отправляете. На `Звуковая карта (MCW)` тон уходит только в аппарат и никуда больше, поэтому без этого вы отправляете в тишине. Выключите там, где собственный монитор аппарата или приём с эфира уже делают эту работу.
cw-hold-what-typed-until-return = Удерживать набранное до Return, затем отправлять строку целиком, вместо того чтобы манипулировать каждым знаком по мере набора. Стоит иметь на трансивере, который манипулирует сам по тексту, где каждая передача манипулятору — это ещё один цикл передача-приём.
cw-cw-messages = СООБЩЕНИЯ CW
cw-clear-rx = {" "}ОЧИСТИТЬ RX{" "}
cw-empty-receive-window-straight-key = Очистить окно приёма и обратную проверку прямого ключа. Ничто, что в эфире, не останавливается.
cw-lock = ЗАХВАТ
cw-decode-speed-set-here-instead = Декодировать на скорости, заданной здесь, вместо того чтобы считывать её с сигнала. Помогает сигналу, который слишком слаб для поиска скорости.
cw-farnsworth-send-characters-full-speed = Фарнсворт: отправлять знаки на полной скорости и растягивать только промежутки между ними, так что их слышно в правильном ритме, но они приходят достаточно медленно, чтобы их записать.
cw-off-normal-spacing = Выкл. — обычные интервалы
cw-keying-speed = Скорость манипуляции
cw-idle-0 = ПАУЗА 0
cw-how-long-transmit-held-after = Сколько передача удерживается после последнего знака или после того, как отпущен прямой ключ, прежде чем несущая сбросится. Длиннее перекрывает паузы медленного оператора; короче быстрее уходит с частоты; «Выкл.» сбрасывает, как только всё из очереди ушло.
cw-off-drop-once = Выкл. — сбросить сразу

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = Настроить вниз на 10 Гц
textmodem-tune-up-10-hz = Настроить вверх на 10 Гц
textmodem-tx = ● TX
textmodem-listening = — прослушивание —
textmodem-type-line-return-sends = Введите строку, Return отправит…
textmodem-type-here-transmit = Введите здесь для передачи…
textmodem-tx-2 = {"  "}TX ВКЛ{"  "}
textmodem-tx-3 = {"   "}TX{"   "}
textmodem-call-cq = {" "}ВЫЗОВ CQ{" "}
textmodem-nocall = NOCALL
textmodem-clear = {" "}ОЧИСТИТЬ{" "}
textmodem-hold-what-typed-until-return = Удерживать набранное до Return, затем отправлять строку целиком, вместо того чтобы передавать каждый знак в эфир по мере набора. Позволяет прочитать строку и исправить её, прежде чем что-либо из неё уйдёт.
textmodem-messages = СООБЩЕНИЯ
textmodem-hell = HELL
textmodem-contrast = Контраст
textmodem-harder-softer-dots-redraws-whole = Жёстче или мягче точки — перерисовывает всю полосу
textmodem-width = Ширина
textmodem-t-2row = {" "}2СТР{" "}
textmodem-draw-every-column-twice-stacked = Рисовать каждую колонку дважды, одну над другой. У Hell нет вертикальной синхронизации, поэтому это сохраняет одну полную копию текста читаемой при любой фазе.
textmodem-rev = {" "}ИНВ{" "}
textmodem-reverse-video-light-dots-dark = Инверсное видео — светлые точки на тёмной бумаге
textmodem-clear-rx = {" "}ОЧИСТИТЬ RX{" "}
textmodem-hold-channel-idle-sends-blank = Удержать канал: в простое отправляется пустая бумага, так что полоса продолжает прокручиваться
textmodem-hold-what-typed-until-return-2 = Удерживать набранное до Return, затем отправлять строку целиком, вместо того чтобы наносить каждый знак на полосу по мере набора. Исправление, набранное в реальном времени, уже на бумаге; сделанное до Return — нет.
textmodem-shift = Сдвиг
textmodem-shift-5-hz = Сдвиг −5 Гц
textmodem-shift-5-hz-2 = Сдвиг +5 Гц
textmodem-baud = Бод
textmodem-reverse-swap-mark-space-needed = Инверсия: поменять местами метку и пробел. Нужна, когда сигнал принимается на боковой полосе, противоположной той, на которой был отправлен, — это инвертирует тоны.
textmodem-track-tuning-error-automatically-instead = Отслеживать ошибку настройки автоматически, вместо того чтобы оставаться закреплённым на курсоре.
textmodem-tones = Тоны
textmodem-mode = Режим
textmodem-off-keyed = вкл/выкл манипуляцией

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = запуск декодера AIS…
ais-tune-between-two-ais-channels = Настройтесь между двумя каналами AIS — 161,975 (AIS 1, морской 87B) и 162,025 (AIS 2, 88B). Шкала стоит посередине, потому что там ничего не передаётся, поэтому пик DC приёмника с нулевой ПЧ не попадает ни на один из них.
ais-setup = НАСТРОЙКА
ais-channels-timeouts-trail-length-how = Каналы, таймауты, длина следа и насколько далеко тянутся векторы
ais-vessels = СУДА
ais-nothing-heard-yet = пока ничего не слышно
ais-virtual-nothing-there = виртуальный — там ничего нет
ais-position-stale = позиция устарела
ais-search-rescue-transmitter-man-overboard = передатчик поиска и спасения, маяк «человек за бортом» или AIS EPIRB
ais-call-sign = позывной
ais-station-time = время станции
ais-dgnss = {" "}(DGNSS)
ais-first-heard = впервые услышано
ais-center = ЦЕНТР
ais-put-vessel-middle-chart = Поместить это судно в середину карты
ais-last-message-station-nmea-sentences = Последнее сообщение от этой станции — как предложения NMEA, которые приёмник выдал бы в линию. Вставьте его в любой другой декодер AIS, чтобы проверить, что из него сделал sdroxide.
ais-ais-setup = Настройка AIS
ais-channels = Каналы
ais-drop-chart-after = Убрать с карты после
ais-without-position-report = без отчёта о позиции
ais-five-minutes-default-not-ads = По умолчанию пять минут, а не десять секунд ADS-B: судно на якоре сообщает раз в три минуты, и более короткое окно опустошило бы большую часть гавани между двумя вполне годными отчётами. По истечении этого времени судно уходит с карты, а его строка сереет — не блёкнет, потому что тусклый символ на устаревшей позиции всё ещё утверждает, где находится судно, теми же чернилами, что и настоящие.
ais-drop-list-after = Убрать из списка после
ais-nothing-heard-all = без единого сигнала вообще
ais-message-carrying-ship-s-name = Сообщение с названием судна приходит каждые шесть минут, поэтому короткое окно списка постоянно выбрасывает суда как раз до того, как они назовут себя.
ais-trail-length = Длина следа
ais-history-behind-each-target = истории за каждой целью
ais-minutes-rather-than-points-because = В минутах, а не в точках, потому что частоты отчётов AIS различаются на два порядка: фиксированное число было бы восемьюдесятью секундами парома и двумя часами стоящего на якоре танкера, нарисованными одинаково. Ноль выключает следы.
ais-speed-vector = Вектор скорости
ais-how-far-ahead-vector-reaches = насколько далеко вперёд тянется вектор
ais-slot-threshold = Порог слота
ais-above-channel-s-learned-noise = выше выученного шумового пола канала
ais-track-most = Отслеживать не более
ais-vessels-2 = {" "}судов
ais-ais-receive-only-here-always = AIS здесь только для приёма и таким останется: это служба безопасности жизни, а вносить в неё ложный трафик судов лицензия не покрывает.
ais-fill-my-grid-digimode-setup = Заполните «Мой квадрат» в настройке цифрового режима для дальностей и пеленгов, чтобы карта обрамлялась вокруг вашего места, а не вокруг того, что дальше всего.
ais-name = НАЗВАНИЕ
ais-mmsi = MMSI
ais-type = ТИП

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = Этот приёмник сам не достаёт до 10489,750 МГц — сначала задайте смещение LNB/конвертера (Настройки ▸ Радио)
qo100-run-fast-spectral-tracker-every = Запустить быстрый спектральный трекер: каждую секунду ищет в окне парковки два симметричных лепестка маяка и сообщает, где сидит несущая
qo100-telemetry = ТЕЛЕМЕТРИЯ
qo100-also-run-ao-40-uncoded = Также запустить декодер некодированных кадров AO-40 — слово синхронизации, CRC и текст телеметрии — с пошаговым отчётом (несущая → синхронизация → CRC) о том, как далеко проходит каждый пролёт
qo100-auto = АВТО
qo100-auto-correct-let-tracker-adjust = Автоподстройка: пусть трекер сам подстраивает смещение конвертера/LNB — медленный замкнутый контур, который на чистой и устойчивой оценке подталкивает маяк обратно на 10489,750 МГц и удерживает его там по мере дрейфа LNB. Каждый раз открывает фронтенд заново, когда действует.
qo100-narrower-search-smaller-slice = Уже — искать меньший участок
qo100-wider-when-beacon-isn-t = Шире — когда маяк не найден при текущей ширине
qo100-clear-mark = сбросить отметку
qo100-drop-hand-placed-beacon-mark = Убрать отметку маяка, поставленную вручную, и вернуться к собственному показанию декодера
qo100-lo = lo −
qo100-lo-2 = lo +
qo100-hi = hi −
qo100-hi-2 = hi +
qo100-radio-s-own-tuning-range = Собственный диапазон настройки этого радио не достаёт до 10489,750 МГц — нужно смещение LNB/конвертера (Настройки ▸ Радио ▸ Конвертер), прежде чем это окно сможет что-либо сделать.
qo100-open-settings-radio = Открыть Настройки ▸ Радио
qo100-tune-10489-750-mhz = Настроиться на 10489,750 МГц
qo100-tip-if-you-can-see = совет: если маяк виден на полосе, но не захватывается, дважды щёлкните по нему — это отметит его как 10489,750 МГц и позволит ПРИМЕНИТЬ подстроиться к нему
qo100-tracker = ТРЕКЕР
qo100-no-twin-lobe-shape-parking = в окне парковки нет формы с двумя лепестками
qo100-shape = ФОРМА
qo100-drift = ДРЕЙФ
qo100-need-longer-run-estimates = — (нужна более длинная серия оценок)
qo100-cycles = ЦИКЛЫ
qo100-receiver = ПРИЁМНИК
qo100-target = ЦЕЛЬ
qo100-measured = ИЗМЕРЕНО
qo100-clicked = {"  "}(щелчок)
qo100-tracker-2 = {"  "}(трекер)
qo100-not-locked-yet = пока не захвачено
qo100-converter-offset = СМЕЩЕНИЕ КОНВЕРТЕРА
qo100-decode = ДЕКОДЕР
qo100-decode-filling-first-frame-buffer = ДЕКОДЕР — заполняется буфер первого кадра…
qo100-decode-off-turn-telemetry-chip = ДЕКОДЕР — выключен. Включите чип ТЕЛЕМЕТРИЯ (верхний ряд) для отчёта несущая → синхронизация → CRC.
qo100-apply-correction = {" "}ПРИМЕНИТЬ ПОПРАВКУ{" "}
qo100-write-converter-lnb-offset-puts = Записать смещение конвертера/LNB, которое ставит маяк туда, где вы щёлкнули, на 10489,750 МГц, и переоткрыть приёмник — то же кратковременное прерывание, что делает Настройки ▸ Радио ▸ Применить
qo100-write-converter-lnb-offset-puts-2 = Записать смещение конвертера/LNB, которое ставит оценку трекера на 10489,750 МГц, и переоткрыть приёмник — то же кратковременное прерывание, что делает Настройки ▸ Радио ▸ Применить
qo100-waiting-second-crc-valid-frame = Ожидание второго кадра с верным CRC, прежде чем предложить это записать — один захват может быть случайным совпадением
qo100-write-corrected-converter-lnb-offset = Записать исправленное смещение конвертера/LNB и переоткрыть приёмник — кратковременное прерывание, то же, что делает Настройки ▸ Радио ▸ Применить
qo100-decoder-runs-receiving-station-readout = декодер работает на принимающей станции — его показания удалённым клиентам не отправляются
qo100-starting = запуск…
qo100-searching-first-window-fills-after = поиск — первое окно заполняется примерно через 24 с, затем повторяется

## js8 (src/app/panels/js8.rs)

js8-multi = МНОГО
js8-decode-every-js8-speed-not = Декодировать каждую скорость JS8, а не только ту, на которой передаёте. Четыре скорости делят поддиапазон и являются четырьмя разными сигналами, поэтому без этого станция на другой скорости невидима — а обмен между двумя скоростями вообще невозможен. Стоит примерно в четыре раза больше процессорного времени приёма.
js8-hb-auto = HB АВТО
js8-turbo-does-not-beacon-local = Turbo не передаёт маяк — это локальная скорость и скорость VHF
js8-beacon-your-callsign-grid-every = Передавать ваш позывной и квадрат каждые 15 минут
js8-until-next-heartbeat = До следующего heartbeat
js8-tx = ● TX
js8-heard = СЛЫШНО
js8-nothing-heard-yet = — пока ничего не слышно —
js8-dxcc = DXCC
js8-band = ДИАПАЗОН
js8-grid = КВАДРАТ
js8-dupe = ПОВТОР
js8-reply = ОТВЕТ
js8-no-messages = — сообщений нет —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}HB{" "}
js8-clear = {" "}ОЧИСТИТЬ КОМУ{" "}
js8-forget-selected-station-composer-goes = Забыть выбранную станцию — редактор возвращается к @ALLCALL
js8-already-addressing-allcall = Уже адресуется @ALLCALL
js8-stop = {" "}СТОП{" "}
js8-send = {" "}ОТПРАВИТЬ{" "}
js8-message = Сообщение…

## logbook (src/app/logbook.rs)

logbook-logbook = ЖУРНАЛ
logbook-new-entry = + НОВАЯ ЗАПИСЬ
logbook-adif = ADIF
logbook-import = ИМПОРТ
logbook-import-qsos-adif-adi-adif = Импорт QSO из файла ADIF (.adi или .adif)
logbook-new-qso = НОВОЕ QSO
logbook-edit-qso = ПРАВКА QSO
logbook-call = Позывной
logbook-lookup = ПОИСК
logbook-look-up-name-qth-grid = Найти имя / QTH / квадрат
logbook-grid = Квадрат
logbook-freq-mhz = Частота МГц
logbook-mode = Режим
logbook-rst-sent = RST передано
logbook-rst-rcvd = RST принято
logbook-name = Имя
logbook-state = Область
logbook-country = Страна
logbook-date-utc = Дата UTC
logbook-time = Время
logbook-pwr-w = Мощн. Вт
logbook-contest = Контест
logbook-s-sent = S# передано
logbook-s-rcvd = S# принято
logbook-comment = Комментарий
logbook-save = {" "}СОХРАНИТЬ{" "}
logbook-cancel = ОТМЕНА
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = QSL ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = Удалить эту запись
logbook-edit = ПРАВКА
logbook-upload-qso-configured-logs = Выгрузить это QSO в настроенные журналы
logbook-no-qsos-yet-run-ft8 = QSO пока нет — проведите FT8/FT4 или добавьте запись вручную

## scanner (src/app/scanner.rs)

scanner-scanner = Сканер
scanner-work-through-stored-memory-channels = Пройтись по сохранённым ячейкам памяти
scanner-work-through-slice-band-channel = Пройтись по участку диапазона по сетке каналов
scanner-stop = СТОП
scanner-start = СТАРТ
scanner-from = От
scanner-step = Шаг
scanner-mode = Режим
scanner-range-empty-high-edge-has = Этот диапазон пуст — верхняя граница должна быть как минимум на один канал выше нижней.
scanner-skip-while-holding-pass-over = ПРОПУСК, пока он удерживается, — отныне пройти этот канал
scanner-skipping = Пропуск
scanner-clear = СБРОС
scanner-stop-passing-over-any-them = Перестать пропускать любой из них
scanner-stop-passing-over-channel = Перестать пропускать этот канал
scanner-no-memory-channels-stored-yet = Ячеек памяти пока нет — сохраните некоторые в окне MEM.
scanner-skip-channel-pass-over = ПРОПУСК — пропустить канал
scanner-skip = ПРОПУСК
scanner-look-all-channels-fall-inside = Искать все каналы, попадающие в одно окно приёмника, по тому же преобразованию, из которого сделана панорама, и настраиваться только на те, на которых что-то есть. Список на одном диапазоне тогда стоит одну настройку за проход, какой бы длинный он ни был, вместо времени установления на каждый канал. Сканирование всё равно слушает каждого кандидата, прежде чем остановиться, так что то, что останавливает его, не меняется — но свип измеряет через FFT, а не через фильтр приёмника, поэтому проверьте порог, если он начнёт останавливаться на пустоте.
scanner-radio-hands-over-demodulated-audio = Это радио отдаёт демодулированный звук и не имеет собственного спектра, по которому можно было бы искать, поэтому его сканирование памяти в любом случае посещает каждый канал.
scanner-fast = БЫСТРО
scanner-read-list-off-spectrum-instead = считывать список со спектра вместо посещения каждого канала
scanner-folders = Папки
scanner-scan-every-folder-every-folder = Сканировать каждую папку и каждую папку, созданную отныне. Выберите папки, чтобы сканировать только их.
scanner-unfiled = Без папки
scanner-nothing-filed-under-folders-you = В выбранных вами папках ничего не лежит.
scanner-stops = Останавливается на
scanner-use-receiver-s-own-squelch = Использовать собственный шумоподавитель приёмника, чтобы сканирование останавливалось ровно там, где открылся бы звук
scanner-squelch-off-scan-will-stop = шумоподавитель выключен — сканирование остановится на первом, что проверит
scanner-channel-power-signal-has-reach = Мощность канала, которой сигнал должен достичь, чтобы остановить сканирование
scanner-listens = Слушает
scanner-how-long-stay-candidate-before = Как долго оставаться на кандидате, прежде чем судить о нём. Меньше примерно одной десятой секунды — измеритель уровня не устаканивается, и слабые сигналы пропускаются
scanner-resumes = Продолжает
scanner-carry-once-signal-drops = Продолжить, когда сигнал пропадает
scanner-carry-after-fixed-time-regardless = Продолжить через фиксированное время независимо от всего
scanner-stay-until-you-press-next = Оставаться, пока не нажмёте ДАЛЕЕ
scanner-how-long-stay = Как долго оставаться
scanner-how-long-wait-after-signal = Как долго ждать после того, как сигнал пропал
scanner-move-don-t-stop-channel = Двигаться дальше и не останавливаться на этом канале снова
scanner-next = ДАЛЕЕ
scanner-move-now = Двигаться дальше сейчас

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = запуск декодера ADS-B…
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = 1090 МГц Mode S
adsb-tune-receiver-1090-000-mhz = Настройте приёмник на 1090,000 МГц — всемирную нисходящую линию ADS-B. Канал один.
adsb-setup = НАСТРОЙКА
adsb-timeouts-trail-length-how-far = Таймауты, длина следа и насколько далеко тянутся векторы скорости
adsb-aircraft = ВОЗДУШНЫЕ СУДА
adsb-nothing-heard-yet = пока ничего не слышно
adsb-ground = на земле
adsb-position-stale = позиция устарела
adsb-gnss-altitude = Высота GNSS
adsb-ground-speed = Наземная скорость
adsb-vertical-rate = Вертикальная скорость
adsb-first-heard = впервые услышано
adsb-center = ЦЕНТР
adsb-put-aircraft-middle-map = Поместить этот борт в середину карты
adsb-last-frame-accepted-aircraft = Последний кадр, принятый от этого борта
adsb-ads-b-setup = Настройка ADS-B
adsb-drop-map-after = Убрать с карты после
adsb-without-position-report = без отчёта о позиции
adsb-past-aircraft-comes-off-map = Дальше этого борт уходит с карты, а его строка сереет. Не блёкнет: тусклый квадрат на устаревшей позиции всё ещё утверждает, где находится самолёт, теми же чернилами, что и настоящие.
adsb-drop-list-after = Убрать из списка после
adsb-nothing-heard-all = без единого сигнала вообще
adsb-trail-length = Длина следа
adsb-points = {" "}точек
adsb-history-dots-behind-each-target = точек истории за каждой целью
adsb-speed-vector = Вектор скорости
adsb-how-far-ahead-leader-line = насколько далеко вперёд тянется линия-указатель
adsb-one-minute-usual-radar-convention = Одна минута — обычная радарная конвенция: линия ровно такой длины, какое расстояние борт проходит за это время, поэтому две равные линии — две равные скорости при любом масштабе. Ноль выключает их.
adsb-track-most = Отслеживать не более
adsb-aircraft-2 = {" "}бортов
adsb-aircraft-ground-are-placed-against = Борты на земле ставятся относительно собственной позиции станции — у сквиттера поверхности нет однозначного собственного декодирования, — поэтому заполните «Мой квадрат» в настройке цифрового режима, если ближайший аэропорт ничего не показывает.
adsb-call = ПОЗЫВ.
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = ожидание приёмника…
drm-not-drm-set-mode-drm = Не в DRM. Установите режим DRM на цифровой коротковолновой вещательной передаче — шкала стоит на центре канала, а не на боковой полосе.
drm-no-drm-signal-locked-decoder = Сигнал DRM не захвачен. Декодеру нужно несколько секунд на чистой несущей: DRM перемежает данные на 400 мс или 2 с, прежде чем что-либо из них можно прочитать.
drm-station-s-audio-codec-not = Аудиокодек этой станции здесь не декодируется. xHE-AAC требует libfdk-aac в системе — его нельзя встроить, потому что его лицензия и лицензия этой программы несовместимы. Установите его (Debian/Ubuntu: libfdk-aac2, Arch: libfdk-aac, macOS: brew install fdk-aac, Windows: libfdk-aac-2.dll рядом с sdroxide.exe) и перезапустите. Всё остальное на этом экране декодируется нормально.
drm-quality = КАЧЕСТВО
drm-last-minute = последняя минута
drm-waiting-locked-signal = ожидание захваченного сигнала
drm-audio-frames-decoding-but-station = Аудиокадры декодируются — но кодек этой станции здесь не декодируется
drm-audio-frames-decoding = Аудиокадры декодируются
drm-samples-reaching-decoder = Отсчёты поступают в декодер
drm-time = ВРЕМЯ
drm-symbol-timing-recovered = Тактирование символов восстановлено
drm-frame = КАДР
drm-transmission-frames-found = Найдены кадры передачи
drm-fast-access-channel-what-transmission = Канал быстрого доступа — что это за передача
drm-service-description-channel-what-services = Канал описания сервисов — какие это сервисы
drm-audio = АУДИО
drm-constellation = СОЗВЕЗДИЕ
drm-waiting-decoded-symbols = ожидание декодированных символов
drm-sampled-across-frame = выборка по всему кадру
drm-whole-frame = весь кадр
drm-mode = РЕЖИМ
drm-robustness-mode-channel-width-ground = Режим устойчивости и ширина канала. A — для наземной волны и несёт больше всего; D — для сильно рассеянного пути через ионосферу и несёт меньше всего.
drm-interleave = ПЕРЕМЕЖЕНИЕ
drm-how-far-transmission-spreads-each = На сколько передача разносит каждый кадр во времени. Длинное переживает более глубокие замирания и соответственно дольше захватывается.
drm-protection = ЗАЩИТА
drm-offset = СМЕЩЕНИЕ
drm-residual-sample-clock-error-against = Остаточная ошибка тактового генератора отсчётов относительно передатчика. Большая и устойчивая означает, что опорный сигнал приёмника неверен, а не вещание.
drm-doppler = ДОПЛЕР
drm-delay = ЗАДЕРЖКА
drm-doppler-delay-spread-path-how = Доплеровское рассеяние и разброс задержки пути — насколько быстро он движется и насколько далеко друг от друга приходят его эхо.
drm-service = СЕРВИС
drm-decode-service-multiplex = Декодировать этот сервис мультиплекса

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = запуск станции AtCHAT NET…
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET — многостанционная клавиатура и файлы
atchat-net = {" "}В СЕТИ{" "}
atchat-net-click-leave-state-kept = В сети — нажмите, чтобы выйти (состояние сохраняется; нажмите ещё раз, чтобы вернуться)
atchat-rejoin = {" "}ВЕРНУТЬСЯ{" "}
atchat-off-net-click-rejoin = Вне сети — нажмите, чтобы вернуться
atchat-off-net = вне сети
atchat-carrier = ● НЕСУЩАЯ
atchat-another-station-transmitting-channel-busy = Другая станция передаёт — канал занят.
atchat-tx = ● TX
atchat-station-callsign-virtual-channel-address = Позывной станции и адрес виртуального канала
atchat-virtual-channel = ВИРТУАЛЬНЫЙ КАНАЛ
atchat-work-net-over-tcp-endpoint = Работать с сетью через конечную точку TCP вместо радио — совместимая с channel_server петля для разработки без радио. Выключено выводит станцию в эфир.
atchat-roster = СОСТАВ
atchat-nobody-heard-yet = пока никого не слышно
atchat-chat = {" "}ЧАТ{" "}
atchat-common-channel-lines-here-go = Общий канал — строки здесь уходят всем (ALL)
atchat-log = {" "}ЖУРНАЛ{" "}
atchat-station-s-own-air-activity = Собственная активность станции в эфире — выбор мастера, старение состава, повторы ARQ
atchat-no-activity-logged-yet = Активность пока не записана.
atchat-no-messages-yet = Сообщений пока нет.
atchat-message-everyone-all = сообщение всем (ALL)
atchat-send = {" "}ОТПРАВИТЬ{" "}
atchat-transfers = ПЕРЕДАЧИ
atchat-all = → ALL
atchat-sent-file-follows-active-chat = Отправленный файл следует за активной вкладкой чата
atchat-send-file = {" "}ОТПРАВИТЬ ФАЙЛ{" "}
atchat-send-file-image-over-air = Отправить файл или изображение в эфир, block-CRC-ARQ
atchat-nothing-flight = ничего в полёте
atchat-images = ИЗОБРАЖЕНИЯ
atchat-no-images-received = Изображений не получено.


## rds (src/app/rds.rs)

rds-station = СТАНЦИЯ
rds-diagnostics = ДИАГНОСТИКА
rds-which-programme-type-table-read = По какой таблице типов программ читать эту станцию. RDS используется везде за пределами Северной Америки; RBDS добавляет позывной, который код идентификации американской станции записывает. «Авто» следует за расширенным кодом страны, который посылает станция, а пока он не пришёл, позывной не расшифровывает — диапазоны идентификации, которые он бы читал, общие с кодами RDS девяти стран, поэтому выберите RBDS, чтобы назвать станцию, которая не посылает кода страны.
rds-waiting-receiver = ожидание приёмника…
rds-no-rds-station-not-every = На этой станции нет RDS. Не каждый передатчик его несёт, а слабый может нести его, не доходя до приёмника.
rds-rds-only-carried-wfm-broadcast = RDS несут только станции WFM-вещания.
rds-holding = УДЕРЖАНИЕ
rds-block-sync-has-dropped-last = Синхронизация блоков пропала — это последняя прочитанная станция.
rds-identity = ИДЕНТИФИКАТОР
rds-programme = ПРОГРАММА
rds-described = ОПИСАНА КАК
rds-content = СОДЕРЖАНИЕ
rds-music = Музыка
rds-speech = Речь
rds-traffic = ДОРОЖНАЯ ОБСТАНОВКА
rds-announcement-air-now = объявление в эфире сейчас
rds-carried-station = передаётся этой станцией
rds-not-carried = не передаётся
rds-radio-text = ТЕКСТ РАДИО
rds-station-clock = ЧАСЫ СТАНЦИИ
rds-country-code = КОД СТРАНЫ
rds-also = ТАКЖЕ НА
rds-also-tagged = ТАКЖЕ ПОМЕЧЕНО
rds-sync = СИНХР.
rds-no-sync = НЕТ СИНХР.
rds-blocks-failed-their-check-plus = Блоки, не прошедшие проверку, плюс те, что корректору ошибок пришлось восстанавливать. Исправления считаются ошибками намеренно: растущая цифра — первый признак того, что станция вот-вот перестанет декодироваться.
rds-front-end-overloading = ФРОНТЕНД ПЕРЕГРУЖЕН
rds-converter-clipping-which-puts-distortion = Конвертер ограничивает сигнал, что даёт искажения по всему мультиплексу — включая поднесущую данных 57 кГц. Уменьшите ВЧ-усиление или включите аттенюатор: RDS уходит примерно на децибел раньше, чем что-либо слышно, поэтому станция, которая звучит идеально, может быть слишком сильной, чтобы её данные выжили.
rds-group-types = ТИПЫ ГРУПП
rds-recent-groups-newest-last = НЕДАВНИЕ ГРУППЫ — самые новые в конце
rds-nothing-decoded-yet = пока ничего не декодировано

## winlink (src/app/winlink.rs)

winlink-mail = ПОЧТА
winlink-abort = ПРЕРВАТЬ
winlink-stop-session-progress = Остановить текущий сеанс
winlink-connect = ПОДКЛЮЧИТЬСЯ
winlink-compose = НАПИСАТЬ
winlink-connecting = подключение…
winlink-not-connected-yet = пока не подключено
winlink-loading = загрузка…
winlink-date = ДАТА
winlink-from = ОТ
winlink-subject = ТЕМА
winlink-no-subject = (без темы)
winlink-delete-message = Удалить это сообщение
winlink-arch = АРХИВ
winlink-move-archive = Переместить в архив
winlink-select-message = выберите сообщение
winlink-reply = ОТВЕТИТЬ
winlink-delete = УДАЛИТЬ
winlink-to = Кому
winlink-cc = Копия
winlink-subject-2 = Тема
winlink-callsign-smtp-someone-example-org = Позывной или SMTP:someone@example.org для интернет-почты. Несколько разделяйте запятыми.
winlink-file-outbox = ФАЙЛ В ИСХОДЯЩИЕ
winlink-discard = ОТМЕНИТЬ
winlink-filed-messages-go-out-next = сохранённые сообщения уйдут в следующем сеансе
winlink-mail-log = ЖУРНАЛ ПОЧТЫ
winlink-session-running = сеанс идёт…
winlink-no-session-yet = сеансов пока нет

## wspr (src/app/panels/wspr.rs)

wspr-receptions = ПРИЁМЫ
wspr-save-wspr-reception-list-csv = Сохранить список приёмов WSPR в CSV
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = маяк распространения слабых сигналов
wspr-time-left-two-minute-slot = Время до конца этого двухминутного слота. В его конце запись уходит в декодер, и принимается следующее решение о передаче.
wspr-tx = ● TX
wspr-decoding = декодирование…
wspr-band = Диапазон
wspr-last-slot = Последний слот
wspr-nothing-heard = ничего не слышно
wspr-receive-only = только приём
wspr-next-slot = Следующий слот
wspr-waiting-radio-take-change = ожидание, пока радио примет изменение…
wspr-hop-next = Прыгнуть дальше
wspr-transmit = ПЕРЕДАВАТЬ
wspr-receive-only-2 = Только приём.
wspr-power = МОЩНОСТЬ
wspr-what-you-actually-radiate = сколько вы реально излучаете
wspr-goes-out-message-everyone-who = Это уходит в сообщении, и каждый, кто вас слышит, судит по нему о пути — поэтому оптимистичная цифра здесь портит измерения и у них, и у вас.
wspr-roam = БРОДИТЬ
wspr-pick-different-offset-inside-200 = Выбирать другой сдвиг внутри окна 200 Гц для каждой передачи. Это конвенция: двести герц, которыми пользуются все, работают, только если никто не встаёт в их середину. Выключено — держится там, где курсор.
wspr-band-hop = ПРЫЖКИ ПО ДИАПАЗОНАМ
wspr-move-dial-band-band-between = Переносить шкалу с диапазона на диапазон между слотами, чтобы один приёмник выбирал весь спектр, а не один его участок. Если вы сами крутите VFO, это приостанавливается — нажмите дважды, чтобы продолжить.
wspr-upload = ВЫГРУЗКА
wspr-send-what-station-decodes-wsprnet = Отправлять то, что станция декодирует, на wsprnet.org. Сообщение о том, что вы слышите, — то, что делает приёмник WSPR частью сети; в эфир это ничего не выводит.
wspr-who-heard-me = КТО МЕНЯ СЛЫШАЛ
wspr-ask-wsprnet-org-every-few = Спрашивать wsprnet.org каждые несколько минут, какие станции декодировали эту. У WSPR нет подтверждения никакого рода, поэтому это единственный способ для маяка узнать что-либо о собственном охвате.
wspr-bands = ДИАПАЗОНЫ
wspr-none-selected-hopping-has-nowhere = ничего не выбрано — прыгать некуда
wspr-set-your-callsign-grid-general = Перед передачей задайте ваш позывной и квадрат на вкладке «Общие» в Настройках.

## packet (src/app/panels/packet.rs)

packet-monitor = МОНИТОР
packet-busy = ЗАНЯТО
packet-frames-arrived-but-failed-their = Кадры, которые пришли, но не прошли проверку последовательности — столкновение, замирание или слишком слабый сигнал.
packet-station-callsign-speed-tx-delay = Позывной станции, скорость, задержка TX, путь через диджипитеры, маяк и сервер KISS
packet-starting-packet-modem = запуск пакетного модема…
packet-nothing-heard-yet = пока ничего не слышно
packet-terminal = ТЕРМИНАЛ
packet-connected = {" "}ПОДКЛЮЧЕНО{" "}
packet-calling = {" "}ВЫЗОВ{" "}
packet-idle = {" "}ПРОСТОЙ{" "}
packet-extended-sequence-numbers-window-up = Расширенные порядковые номера — окно до 127 кадров.
packet-frames-sent-not-yet-acknowledged = Кадры, отправленные и ещё не подтверждённые удалённым концом.
packet-retries-against-link-s-limit = Повторы относительно предела этой связи. Растущий счётчик, пока неподтверждённые кадры стоят на месте, — так выглядит замирающий путь с этой стороны.
packet-mail-window-using-link = Окно ПОЧТА использует связь.
packet-one-radio-one-channel-one = Одно радио, один канал, одна связь. Сеанс Winlink и этот терминал — два способа ею пользоваться, и кто спросит первым, тот её и получит.
packet-digipeaters-nearest-first-separated-commas = Диджипитеры, ближайший первым, через запятую — OE3XLR-1,OE3XMS-1. Оставьте пустым для станции, которую слышите напрямую, или чтобы использовать путь по умолчанию из настроек пакетного режима.
packet-disconnect = {" "}ОТКЛЮЧИТЬСЯ{" "}
packet-hang-up-properly-wait-far = Завершите связь как положено и дождитесь, пока удалённый конец согласится. Смена режима вместо этого завершает сеанс, никому не сообщив.
packet-connect = {" "}ПОДКЛЮЧИТЬСЯ{" "}
packet-mail-window-has-link-finish = Окно ПОЧТА держит связь. Сначала завершите или остановите тот сеанс.
packet-station-has-no-callsign-yet = У этой станции пока нет позывного — задайте его в НАСТРОЙКА над панелью монитора. Ничего не передаётся, пока он не задан.
packet-call-station-connected-mode-node = Вызвать эту станцию в режиме соединения — узел, BBS или другой оператор.
packet-needs-callsign-call = Для вызова нужен позывной.
packet-clear = {" "}ОЧИСТИТЬ{" "}
packet-empty-transcript-link-untouched-connected = Очистить стенограмму. Связь не затрагивается — подключённая станция остаётся подключённой.
packet-not-connected-type-callsign-above = Не подключено. Введите позывной выше и нажмите ПОДКЛЮЧИТЬСЯ, чтобы вызвать узел или BBS.
packet-type-here = введите здесь
packet-not-connected = не подключено
packet-send = {" "}ОТПРАВИТЬ{" "}
packet-send-line-node-bbs-reads = Отправить строку. Узел или BBS читает по одной строке.

## fsq (src/app/panels/fsq.rs)

fsq-speed = Скорость
fsq-call = Позывной
fsq-callsign-directed-fsqcall-messages-defaults = Позывной для адресных сообщений (FSQCALL); по умолчанию ваш позывной
fsq-tx = ● TX
fsq-contacts = КОНТАКТЫ
fsq-heard = СЛЫШНО
fsq-none = — нет —
fsq-allcall = ALLCALL
fsq-image = ИЗОБРАЖЕНИЕ
fsq-clear = ОЧИСТИТЬ
fsq-forget-pictures-received-so-far = Забыть изображения, полученные до сих пор
fsq-send-image = Отправить изображение…
fsq-right-click-remove = Правый клик, чтобы убрать
fsq-remove-picture = Убрать это изображение
fsq-listening = — прослушивание —
fsq-message = Сообщение…
fsq-send = {" "}ОТПРАВИТЬ{" "}
fsq-call-cq = {" "}ВЫЗОВ CQ{" "}
fsq-heard-2 = {" "}? слышно{" "}
fsq-clear-2 = {" "}ОЧИСТИТЬ{" "}
fsq-fsq-contacts = Контакты FSQ
fsq-add = Добавить

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + ВКЛАДКА
publicsdr-open-receiver-new-radio-own = Открыть этот приёмник как новое радио, на собственной вкладке
publicsdr-point-radio-receiver-replacing-whatever = Направить *это* радио на приёмник, заменив интерфейс, на котором оно сейчас, — спрашивается ещё раз, потому что это настройка целого радио
publicsdr-not-stated = не указано
publicsdr-replace = ЗАМЕНИТЬ
publicsdr-point-radio-you-are-receiver = Направить радио, на котором вы сидите, на этот приёмник
publicsdr-tab-instead = + ВКЛАДКА ВМЕСТО ЭТОГО
publicsdr-open-receiver-another-radio-leave = Открыть приёмник как другое радио и не трогать это
publicsdr-cancel = ОТМЕНА
publicsdr-public-sdrs = ПУБЛИЧНЫЕ SDR
publicsdr-available = ДОСТУПНО
publicsdr-hide-receivers-are-full-ones = Скрыть приёмники, которые заняты, и те, чей оператор не открыл никаких каналов для приложений, кроме браузера
publicsdr-band = В ДИАПАЗОНЕ
publicsdr-only-receivers-cover-current-dial = Только приёмники, которые покрывают текущую частоту шкалы
publicsdr-low-bw = НИЗКАЯ ПОЛОСА
publicsdr-take-spyserver-low-bandwidth-shape = Брать SpyServer в его форме низкой полосы: узкое окно I/Q, которое следует за шкалой, плюс собственный вид диапазона сервера, вместо мегабит широкополосного I/Q. На KiwiSDR это ничего не меняет — у него только одна форма.
publicsdr-refresh = ⟳ ОБНОВИТЬ
publicsdr-fetch-both-directories-again = Загрузить оба каталога заново
publicsdr-name-place-antenna-band = название, место, антенна, диапазон
publicsdr-clear-search = Очистить поиск
publicsdr-fetching-receiver-lists = Загрузка списков приёмников…
publicsdr-machine-radio-attached-does-not = Машина, к которой подключено это радио, не отвечает на вопросы об устройствах, поэтому она тоже не может загрузить списки.
publicsdr-no-match = совпадений нет
publicsdr-nothing-matches-try-turning-filter = ничто не совпадает — попробуйте снова включить один из фильтров
publicsdr-radio = это радио
publicsdr-radio-s-configuration-has-not = Конфигурация этого радио ещё не пришла с машины, к которой оно подключено, поэтому направлять на приёмник нечего — возьмите его через + ВКЛАДКА.
publicsdr-position-not-published-so-nothing = {" "}Его позиция не опубликована, поэтому ничего из того, что оно слышит, не будет сообщено.

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = ожидание приёмника…
hd-not-hd-radio-set-mode = Не в HD Radio. Установите режим HD Radio на FM-несущей, которая несёт цифровые боковые полосы, — шкала стоит на центре аналоговой несущей, а не рядом с ней.
hd-no-hd-radio-lock-digital = Захвата HD Radio нет. Цифровые боковые полосы передаются примерно на 20 дБ ниже аналоговой несущей, поэтому станция, которую можно слушать, всё равно может быть слишком шумной для декодирования: декодеру нужно несколько секунд на чистой.
hd-sync = СИНХР.
hd-ofdm-frame-timing-recovered = Тактирование кадров OFDM восстановлено
hd-audio = АУДИО
hd-audio-frames-decoding = Аудиокадры декодируются
hd-quality = КАЧЕСТВО
hd-lower = НИЖНЯЯ
hd-upper = ВЕРХНЯЯ
hd-last-minute = последняя минута
hd-waiting-locked-signal = ожидание захваченного сигнала
hd-mer-l = MER L
hd-modulation-error-ratio-lower-digital = Отношение ошибки модуляции нижней цифровой боковой полосы.
hd-mer-u = MER U
hd-modulation-error-ratio-upper-digital = Отношение ошибки модуляции верхней цифровой боковой полосы.
hd-cber = CBER
hd-channel-bit-error-ratio-after = Отношение ошибочных битов канала после внутреннего кода. Ниже примерно 1e-3 внешний код может это исправить; выше этого звук начинает распадаться.
hd-offset = СМЕЩЕНИЕ
hd-residual-carrier-frequency-offset-large = Остаточное смещение частоты несущей. Большое и устойчивое означает, что опорный сигнал приёмника неверен, а не вещание.
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = Индикатор первичного сервисного режима: какой из гибридных режимов FM передаёт станция.
hd-programme = ПРОГРАММА
hd-restricted-access = {" "}— ограниченный доступ

## bands (src/app/bands.rs)

bands-bands = ДИАПАЗОНЫ
bands-daylight = ☀ ДЕНЬ
bands-night = ☾ НОЧЬ
bands-no-forecast-yet-fetching-hamqsl = · прогноза пока нет — загружается с HAMQSL.com
bands-band = ДИАПАЗОН
bands-condx = УСЛОВИЯ
bands-paths = ПУТИ
bands-reach = ДАЛЬНОСТЬ
bands-best = ЛУЧШЕЕ
bands-ibp-beacons = МАЯКИ IBP
bands-meteor-showers = МЕТЕОРНЫЕ ПОТОКИ
bands-peak = ПИК

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = СПОТЫ
spots-longwave-shortwave-broadcast-stations-air = Станции вещания на длинных и коротких волнах в эфире сейчас
spots-view = В ВИДЕ
spots-only-spots-inside-panadapter-span = Только споты внутри полосы панорамы
spots-setup = ⚙ НАСТРОЙКА
spots-feeds-lookup-upload-settings = Потоки, поиск и настройки выгрузки
spots-call-station-site-frequency = позывной, станция, площадка, частота
spots-clear-search = Очистить поиск
spots-no-match = совпадений нет
spots-no-spots-enable-feed-setup = спотов нет — включите поток в ⚙ НАСТРОЙКА
spots-nothing-matches-search = ничто не совпадает с поиском

## ism (src/app/ism.rs)

ism-ism-devices = УСТРОЙСТВА ISM
ism-decoding = ДЕКОДИРОВАНИЕ
ism-needs-wideband-iq-source-cat = Нужен широкополосный источник IQ; аппарат CAT посылает только звук
ism-how-far-burst-must-stand = На сколько пачка должна возвышаться над собственным шумовым полом канала, прежде чем её декодируют
ism-give-each-band-width-normally = Дайте каждому диапазону ту ширину, которая ему обычно нужна, — четверть мегагерца для диапазонов OOK на 315, 345 и 433 МГц, полную — для 868 и 915.
ism-several-hundred-more-device-decoders = Несколько сотен дополнительных декодеров устройств из проекта rtl_433 встроены. Читают устройства OOK на 433 МГц, до которых собственные декодеры sdroxide не достают вовсе. Там, где могут читать оба, rtl_433 знает гораздо больше вариантов и берёт верх.
ism-t-1-your-own = 1 ваш собственный
ism-device-decoders-registered-how-many = Зарегистрированные декодеры устройств, сколько из них пришло из вашего rtl433_flex.conf и сколько передач декодировано с его запуска.
ism-reload-decoders = ПЕРЕЗАГРУЗИТЬ ДЕКОДЕРЫ
ism-re-read-rtl433-flex-conf = Перечитать rtl433_flex.conf после правки. Уже услышанные устройства остаются в списке.
ism-rtl-433-knows-more-variants = rtl_433 знает больше вариантов этих устройств, чем собственные декодеры sdroxide, поэтому читает их, пока здесь слушает. Выберите другой диапазон — и собственные декодеры sdroxide снова берут верх.
ism-decoder-your-rtl433-flex-conf = Декодер из вашего rtl433_flex.conf был отклонён. Остальные загрузились. Исправьте строку и нажмите ПЕРЕЗАГРУЗИТЬ ДЕКОДЕРЫ.
ism-waiting-decoder = ожидание декодера…
ism-where-decoder-actually-listening-follows = Где декодер реально слушает. Это следует за аппаратным центром приёмника, а не VFO — на широком фронтенде они могут быть далеко друг от друга, и именно окно решает, какие каналы достижимы.
ism-transmissions-gate-opened-how-many = Передачи, на которых открылся затвор, и сколько из них дали верный кадр. Много пачек и ни одного декодирования означает, что диапазон занят устройствами, которые sdroxide пока не может прочитать. Совсем ничего — значит, ничто не достигает порога: снизьте его или проверьте, что шкала на диапазоне.
ism-nothing-heard-yet = пока ничего не слышно
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

navtex-messages = СООБЩЕНИЯ
navtex-sync = СИНХР.
navtex-character-phase-locked-signal-being = Фаза знаков захвачена — сигнал читается.
navtex-characters-whose-first-copy-was = Знаки, первая копия которых была повреждена и взяты из повтора пятью слотами позже, а также те, где оба были плохими. Знак NAVTEX посылается дважды; в этом вся его коррекция ошибок.
navtex-swap-mark-space-tones-signal = Поменять местами тоны метки и пробела для сигнала, принятого на другой боковой полосе. Выключено — верхняя боковая на частоте канала, как и говорит каждая опубликованная инструкция по настройке этой службы.
navtex-starting-navtex-receiver = запуск приёмника NAVTEX…
navtex-nothing-yet-station-transmits-ten = пока ничего — станция передаёт десять минут каждые четыре часа
navtex-cut-short = обрезано
navtex-message = СООБЩЕНИЕ
navtex-time-day-stated-message-utc = Время суток, указанное в сообщении (UTC): только для отображения
navtex-receiving = ПРИЁМ
navtex-monitor = МОНИТОР
navtex-everything-decoded-message-not = всё декодированное, сообщение или нет

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = RF PAINT
rfpaint-scan-speed = Скорость развёртки
rfpaint-how-fast-text-image-scanned = С какой скоростью текст или изображение развёртывается на водопад. Меньше — медленнее и разборчивее; 100 % — базовая скорость, 25 % (центр) — по умолчанию.
rfpaint-abort = Прервать
rfpaint-text-paint = ТЕКСТОВАЯ ПЕЧАТЬ
rfpaint-type-text-paint = Введите текст для печати…
rfpaint-preview-waterfall = ПРЕДПРОСМОТР ВОДОПАДА
rfpaint-type-text-preview = введите текст для предпросмотра
rfpaint-transmit = {"  "}ПЕРЕДАТЬ{"  "}
rfpaint-image-paint = ПЕЧАТЬ ИЗОБРАЖЕНИЯ
rfpaint-no-image-loaded = изображение не загружено
rfpaint-load-image = Загрузить изображение…
rfpaint-load-image-preview = загрузите изображение для предпросмотра

## hfdl (src/app/hfdl.rs)

hfdl-listen = СЛУШАТЬ
hfdl-switches-engine-s-hfdl-downconverter = Включает или выключает HFDL-понижающий преобразователь и декодер движка. Декодирование стоит полосы 24 кГц и рабочего потока, открыто окно или нет.
hfdl-channel-assigned-frequency-listen-anything = Канал (назначенная частота), на котором слушать. Годится любое место в диапазоне — станции HFDL передают на 2,8–22 МГц — против фиксированной полосы 24 кГц, центрированной здесь.
hfdl-save-hfdl-decode-log-file = Сохранить журнал декодирования HFDL в файл
hfdl-riverhead-northern-atlantic-north-america = Riverhead (Северная Атлантика / Северная Америка) — по умолчанию и канал, на котором проверен декодер.
hfdl-hfdl-assigned-frequency = Назначенная частота HFDL.
hfdl-one-your-saved-channels-click = Один из ваших сохранённых каналов — нажмите, чтобы настроиться, правый клик, чтобы убрать.
hfdl-hfdl-selected-but-decoding-off = HFDL выбран, но декодирование выключено — нажмите СЛУШАТЬ выше
hfdl-decodes = ДЕКОДИРОВАНИЯ
hfdl-nothing-decoded-yet-ground-station = Пока ничего не декодировано — позывной наземной станции повторяется примерно каждые 32 с, как только она попадает в канал.
hfdl-no-decode-matches-filter = Ни одно декодирование не совпадает с фильтром.
hfdl-decoding = ДЕКОДИРОВАНИЕ
hfdl-decoding-off = ДЕКОДИРОВАНИЕ ВЫКЛ.
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = Цифровая речь FreeDV V1
rade-tx = ● TX
rade-mute-analog = ГЛУШИТЬ АНАЛОГ
rade-mute-demodulated-audio-so-only = Заглушить демодулированный звук, чтобы слышна была только декодированная речь. В остальных случаях сырой сигнал пропускается всякий раз, когда модему нечего воспроизвести, — этот шипящий звук и есть то, как вы находите выход до синхронизации, поэтому во время настройки оставьте это выключенным.
rade-sync = СИНХР.
rade-how-far-received-signal-sits = Насколько принятый сигнал отстоит от того места, где модем его ожидает. Большие значения всё равно декодируются — контур захвата их отслеживает, — но подтолкнуть шкалу, чтобы приблизить это к нулю, даёт наилучший запас.
rade-snr = SNR —
rade-callsign-station-put-end-over =
    Позывной, который станция поместила в свой кадр конца выхода — единственная точка в выходе RADE, где он отправляется. Его также получает FreeDV Reporter, поэтому станция, которую вы слышите, появляется на qso.freedv.org как услышанная вами.

    Held for a minute after the signal goes, and dropped when you start an over of your own: it names the station on frequency, not every station the receiver has ever heard.
rade-stop-talking = ПРЕКРАТИТЬ ГОВОРИТЬ
rade-talk-ptt = ГОВОРИТЬ (PTT)
rade-open-close-rade-over-modem = Открыть или закрыть выход RADE. Модему нужно около 120 мс речи, прежде чем уйдёт первый кадр, и он посылает кадр конца выхода, когда вы остановитесь, поэтому передача продолжается немного дольше после кнопки.
rade-rade-v1-occupies-roughly-1060 = RADE V1 занимает примерно 1060–1880 Гц полосы пропускания — верхняя боковая, или нижняя на 160, 80 и 40 м, где RADE ведёт телефонную практику и sdroxide переключает за вас. Поставьте сигнал внутрь затенённой полосы на водопаде; отсюда модем его найдёт.

## frame (src/app/frame.rs)

frame-acknowledge = Подтвердить
frame-dismiss = Закрыть
frame-reconnecting = Переподключение…
frame-not-connected = Не подключено.
frame-reconnect-now = Переподключиться сейчас
frame-transmit-lockout-disabled = ⚠ БЛОКИРОВКА ПЕРЕДАЧИ ОТКЛЮЧЕНА
frame-engine-was-started-oob-tx = Этот движок запущен с --oob-tx. Блокировка любительских диапазонов выключена: он включит передатчик на любой частоте, которую поддерживает аппаратура.
frame-transmitting-outside-your-licence-offence = Передача вне вашей лицензии — правонарушение в каждой стране, которая её выдаёт. Продолжайте, только если вы уполномочены пользоваться частотами, на которых собираетесь включить передачу, — лицензия MARS/CAP или коммерческая, экспериментальное разрешение или эквивалент нагрузки.
frame-i-understand = {"  "}Я ПОНИМАЮ{"  "}
frame-restart-without-oob-tx-put = Перезапустите без --oob-tx, чтобы вернуть блокировку.

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC — насколько сильно раскачивается передатчик, в процентах от наибольшего, что он может выдать. 100 % — полная шкала, а не полная мощность в ваттах: чего это стоит на антенне, зависит от радио и от Drive.
smeter-radio-reports-no-forward-power =


    This radio reports no forward power, so the figure beside ALC is the ALC reading again rather than watts.
smeter-no-swr-radio-has-no =


    No SWR: this radio has no bridge to measure it with, and sdroxide will not invent one — it can only show what the radio itself reports. A rig with an SWR meter (over CAT, TCI or a LAN link) and an HPSDR board both fill this in; a plain SDR transmitter cannot.
smeter-no-signal-report-yet-receiver = Отчёта о сигнале пока нет — приёмник не выдал показания измерителя.
smeter-s9-73-dbm-each-s =


    S9 is −73 dBm and each S-unit below it is 6 dB. The dBm figure is only as honest as the front end's calibration — an uncalibrated receiver reports dBFS with a dBm label.
smeter-ovld-converter-into-rails-so =


    OVLD: the converter is into its rails, so the level shown understates the signal. Wind the front-end gain back.
smeter-table-held = , таблица удержана
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● TX
multi-link = Мощность
multi-mute-radio-s-audio = Отключить звук этого радио
multi-close-radio-s-split-view = Закрыть разделённый вид этого радио
multi-open-radio-split-view-own = Открыть это радио в собственном разделённом виде
multi-already-open-another-split-view = Уже открыто в другом разделённом виде
multi-show-radio-here = Показать это радио здесь
multi-add-radio-computer-settings-radio = Добавить радио на этом компьютере (чтобы добавить его на станции, используйте Настройки → Радио)
multi-add-radio = Добавить радио
multi-asked-station-another-radio = Запрошено у станции другое радио…
multi-client-cannot-open-connection = Этот клиент не может открыть соединение.

## net (src/app/net.rs)

net-fail = ОШИБКА
net-downloading = Загрузка…
net-adif-import-failed-file-could = Импорт ADIF не удался: файл не удалось разобрать
net-channel-import-failed-file-could = Импорт каналов не удался: файл не удалось прочитать
net-channel-import-nothing-read-chirp = Импорт каналов: нечего читать — CSV CHIRP начинается со строки заголовков с именами столбцов, один из которых должен быть Frequency

## widgets (src/app/panels/widgets.rs)

widgets-free-text = свободный текст
widgets-hashed-callsign-not-yet-heard = хешированный позывной, ещё не услышанный целиком
widgets-entity-unknown = образование неизвестно
widgets-band = этот диапазон
widgets-new-entity-never-worked-any = Новое образование — никогда не работалось, ни на одном диапазоне
widgets-new-grid-square = Новый квадрат
widgets-not-worked-before = Раньше не работалось
widgets-worked-before-but-not-band = Работалось раньше, но не на этом диапазоне
widgets-call-queue = В очереди вызовов
widgets-image = Изображение

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = ПРИЁМЫ
pi4-save-pi4-reception-list-csv = Сохранить список приёмов PI4 в CSV
pi4-next-generation-beacon = Маяк нового поколения
pi4-time-left-one-minute-beacon = Время до конца этого цикла маяка в одну минуту. Сообщение PI4 ищется, когда окно успело заполниться дальше того места, где оно кончалось бы.
pi4-decoding = декодирование…
pi4-band = Диапазон
pi4-last-cycle = Последний цикл
pi4-nothing-heard = ничего не слышно
pi4-receive-only-tune-so-beacon = Только приём. Настройтесь так, чтобы CW-опознавание маяка приходилось на звуковые 800 Гц — собственная конвенция сети, — и четыре тона PI4 оказались там, где этот декодер по умолчанию их ищет.

## awards (src/app/awards.rs)

awards-awards = НАГРАДЫ
awards-band = Диапазон
awards-all = Все
awards-dxcc = DXCC
awards-grids = Квадраты
awards-worked-all-states = Работа со всеми штатами
awards-cq-zones-waz = Зоны CQ (WAZ)
awards-dxcc-entities = Образования DXCC

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = Звук в полосе пропускания декодера.
acars-blocks-framed-blocks-whose-check = Блоки, которые были выделены кадром, и блоки, у которых не сошлась проверочная последовательность.
acars-starting-acars-receiver = запуск приёмника ACARS…
acars-channel = КАНАЛ
acars-tune-dial-here = Настройте шкалу здесь.

acars-channel-out-of-range = Вне диапазона приёма этого радиоприёмника.
acars-nothing-yet-acars-quiet-between = Пока ничего. ACARS молчит между самолётами — оставьте его на канале и ждите пачку.
acars-check-failed = проверка не пройдена
acars-block-check-sequence-did-not = Проверочная последовательность блока не совпала — текст может быть неверным.
acars-no-text = (нет текста)

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = Имя пользователя
login-password = Пароль
login-remember-device = Запомнить на этом устройстве
login-signs-without-asking-next-time = Входит без вопроса в следующий раз и пускает на вкладку 3D-вида вообще без вопроса. Пароль хранится открытым текстом, поэтому выключайте это на машине, которой пользуются другие люди.
login-keeps-password-device-clear = Хранит пароль на этом устройстве открытым текстом.
login-checking = {" "}ПРОВЕРКА…{" "}
login-sign = {" "}ВОЙТИ{" "}
login-wrong-password-shuts-server-s = Неверный пароль на несколько секунд закрывает дверь сервера, поэтому это может занять мгновение.

## help (src/help.rs)

help-find = НАЙТИ
help-find-manual = найти в руководстве
help-previous-match-shift-enter-shift = Предыдущее совпадение (Shift+Enter, Shift+F3)
help-next-match-enter-f3 = Следующее совпадение (Enter, F3)
help-no-matches = совпадений нет
help-clear-search-esc = Очистить поиск (Esc)
help-sdroxide-manual = РУКОВОДСТВО SDROXIDE
help-contents = СОДЕРЖАНИЕ

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = Одна станция слышна — ожидание отчёта о её позиции
aismap-double-click-reframe = двойной клик, чтобы переподобрать кадр
aismap-zoom-out-wheel-pinch-do = Уменьшить — колесо или сведение пальцами тоже так делают
aismap-zoom-about-middle-chart = Увеличить относительно середины карты
aismap-frame-everything-being-tracked-again = Снова подогнать кадр под всё, что сопровождается, — двойной клик тоже так делает

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = ожидание спектра…
specview-hounds = HOUNDS

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = У этого окна нет GPU-рендерера, поэтому водопад остаётся стандартной ширины.
spectrum-machine-drawing-without-gpu-every = Эта машина рисует без GPU — каждая колонка водопада — работа процессора, который уже делит её с радио.
spectrum-window-renders-through-opengl-which = Это окно рендерится через OpenGL, то есть по пути совместимости sdroxide — Raspberry Pi, старый графический чип или браузер без WebGPU. Более широкий водопад там не стоит потери частоты кадров.
spectrum-wider-than-renderer-can-draw = Шире, чем может нарисовать этот рендерер.

## macros (src/app/panels/macros.rs)

macros-nocall = NOCALL
macros-remove-button = Убрать эту кнопку

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = ТРЕКЕР КВАДРАТОВ
gridtracker-drag-pan-wheel-zoom-amber = Перетаскивание — панорама, колесо — масштаб. Янтарный = отработан, зелёный = подтверждён, голубой = услышан.

## remote (src/remote.rs)

remote-server-busy-another-client-connected = сервер занят — подключён другой клиент
remote-transmitting-but-client-s-microphone = Передача, но микрофон этого клиента не даёт вообще никакого звука — ничто не модулируется. В браузере проверьте, что странице разрешён микрофон и что никакое другое приложение его не держит.
remote-connection-closed = соединение закрыто

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = Трёхмерный вид Солнечной системы — Солнце, Земля, Луна, пятна и CME (отдельное окно)
solarbtn-solar-system-3d-view-sun-2 = Трёхмерный вид Солнечной системы — Солнце, Земля, Луна, пятна и CME (новая вкладка браузера)

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = Загрузка голоса…
speech-browser-client-cannot-speak-yet = браузерный клиент пока не умеет говорить

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = Двойной клик, чтобы ввести частоту
freqdisplay-tap-type-frequency = Коснитесь, чтобы ввести частоту

## solarstate (src/solar3d/state.rs)

solarstate-home = ДОМ
solarstate-dwarf-planets = КАРЛИКОВЫЕ ПЛАНЕТЫ

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = двойной клик, чтобы переподобрать кадр

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = двойной клик, чтобы переподобрать кадр

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = соединение закрыто

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = не удалось запустить поток оповещений

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = воздушные суда пока не определены

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = ДВОЙНОЙ КЛИК, ЧТОБЫ ПЕРЕПОДОБРАТЬ КАДР

## chrome (src/chrome.rs)



## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = Задаёт собственную настройку мощности FDM-DUO (меню 34 TX POWER). Drive и TUNE затем задают мощность в её пределах, а SDRoxide отправляет эту настройку заново при каждом подключении.

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $day } { $month ->
        [1] янв
        [2] фев
        [3] мар
        [4] апр
        [5] май
        [6] июн
        [7] июл
        [8] авг
        [9] сен
        [10] окт
        [11] ноя
       *[12] дек
    }

## download (src/download.rs)

download-browser-could-not-read-file = браузер не смог прочитать этот файл

## morse (src/app/morse.rs)

morse-translate = ПЕРЕВОД
morse-practice = ТРЕНИРОВКА
morse-learn = ОБУЧЕНИЕ
morse-morse = МОРЗЕ
morse-text-morse = Текст → Морзе
morse-morse-text = Морзе → Текст
morse-per-character-space-between-characters = `.` и `-` на знак, пробел между знаками, ` / ` между словами. Международный алфавит, цифры, знаки препинания и распространённые просигны.
morse-reference-translator-available-desktop-build = Справочный переводчик доступен в настольной сборке.
morse-play-what-you-type-your = Проиграть набранное через ваши динамики
morse-play = ИГРАТЬ
morse-stop = СТОП
morse-every-character-learned-well-done = Все знаки выучены — молодец.
morse-reveal = ПОКАЗАТЬ
morse-reset = СБРОС
morse-replay = ПОВТОР
morse-what-did-you-hear = Что вы услышали?
morse-check = ПРОВЕРИТЬ
morse-press-new-hear-character = Нажмите НОВЫЙ, чтобы услышать знак.
morse-tone = Тон
morse-speed = Скорость
morse-spacing = Интервалы
morse-characters-go-out-speed-above = Знаки идут на скорости выше; это скорость, на которой передаются *промежутки*, поэтому новичок слышит букву целиком, прежде чем сможет передать её так быстро.
morse-playback-available-desktop-build = Воспроизведение доступно в настольной сборке.

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = высота
adsb-row-track = курс
adsb-row-turning = разворот
adsb-row-squawk = код
adsb-row-range = дальность
adsb-row-position = позиция
adsb-row-signal = сигнал
adsb-row-frames = кадры
rade-transmitting = передача
decodes-odd = НЕЧЁТ
decodes-new = НОВЫЙ
js8-new = НОВЫЙ

## AIS panel: vessel detail rows.

ais-row-status = статус
ais-row-speed = скорость
ais-row-course = курс
ais-row-heading = направление
ais-row-turning = разворот
ais-row-altitude = высота
ais-row-size = размер
ais-row-draught = осадка
ais-row-destination = назначение
ais-row-range = дальность
ais-row-signal = сигнал
ais-row-type = тип
ais-row-position = позиция
ais-row-messages = сообщения
ism-off = ВЫКЛ.
textmodem-frequency-shifted = частотная манипуляция
winlink-to-heading = КОМУ

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = Слышно: { $heard } — ни одной позиции новее { $drop_map_s} с
aismap-stations-heard-waiting-position-report = Слышно станций: { $heard } — ожидание отчёта о позиции
aismap-km-across = { $lon_span }° · ~{ $km } км в поперечнике

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = звучит на { $device }
alerts-alarm-device-unavailable = устройство сигнализации недоступно: { $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = { $w } отработано
awards-confirmed = { $value } подтверждено

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · прогноз с HAMQSL.com, возраст { $value }
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · следующий маяк через { $now } с
bands-radiants-placed = радианты нанесены для { $lat }°, { $lon }°
bands-zhr = ZHR { $zhr }
bands-radiant = радиант { $alt_deg }° { $az_deg }
bands-radiant-down = радиант под горизонтом ({ $alt_deg }°)

    A radiant above the horizon means the trails can reach you; a fast shower leaves longer-lived ionised trails for meteor scatter on 6 m and 2 m, and the brief 10 m openings.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } не декодируется
drm-symbols = { $value } символов · { $frame }
drm-not-decodable = { $value } — не декодируется
drm-broadcaster-s-clock-utc = часы вещателя { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    MAX: полная мощность FDM-DUO, около 8 Вт. Настройка для работы с включённым HR500.

    { $base }
duopower-t-5-w-hr500-line-will =
    5 Вт. С включённым HR500 при этой настройке на большинстве диапазонов полная мощность не будет достигнута: используйте MAX и Drive, чтобы её снизить.

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } Вт, для работы без усилителя или QRP.

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = Передача снова включается после { $swr }:1. Сначала проверьте антенну: если неисправность осталась, останавливается и следующая передача.
frame-reconnecting-s = Переподключение через { $value } с…

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = { $worked } отработано
gridtracker-confirmed = { $confirmed } подтверждено
gridtracker-heard = { $heard } услышано

## hd: messages with values (src/app/hd.rs)

hd-decode = Декодировать { $value } ({ $codec_label })

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } кГц уже имеет чип
hfdl-save-current-channel-khz-one = Сохранить текущий канал ({ $cur_khz } кГц) как один из ваших чипов
hfdl-aircraft = воздушных судов: { $aircraft }
hfdl-bursts = пачек: { $bursts }
hfdl-decodes-2 = декодирований: { $decodes }
hfdl-fixed = исправлено: { $fec }

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint } — пока не декодировано
ism-listen-tunes-radio-there-one = Слушать на { $value } — настраивает радио туда. По одному диапазону за раз: они слишком далеко друг от друга, чтобы какой-либо приёмник покрыл два сразу.
ism-watch-hz-around-band-centre = Смотреть { $value } Гц вокруг центра диапазона, чего бы диапазон ни запросил. Уже подходит приёмнику, который не может отдать собственную ширину диапазона; шире — достаёт устройства, которые сидят дальше от центра. Ваш приёмник должен отдать примерно на треть больше этого.
ism-rtl-433 = rtl_433 — { $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } кГц
ism-your-own = { $flex } ваших собственных
ism-decoders-decoded = Декодеров: { $decoders } ({ $flex }), декодировано: { $decodes }
ism-handled-rtl-433 = обрабатывает rtl_433: { $names }
ism-flex = flex: { $error }
ism-window-mhz-mhz-wide = окно { $mhz } МГц, шириной { $e62 } МГц
ism-bursts-decoded = пачек: { $bursts }, декодировано: { $decodes }
ism-devices = устройств: { $ism_reports }
ism-id-mhz-db-heard-first =
    { $protocol } id { $device }
    { $e6 } МГц { $snr_db } дБ, слышно { $value }×
    впервые слышно { $first_at } назад

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = Подтверждения: загружено { $recs }, новых подтверждено { $matched }
logbook-qso = { $qso_log } QSO
logbook-worked-before = ⚠ РАБОТАЛОСЬ РАНЬШЕ ({ $dupe_band })
logbook-utc-qso = { $oldest }–{ $newest } UTC · { $value } QSO

## app: messages with values (src/app/mod.rs)

app-radio = радио { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = не удалось запустить поток звука Морзе: { $error }
morse-no-audio-output = нет аудиовыхода: { $error }
morse-about-s = около { $farnsworth_wpm } с
morse-characters-play = Знаки в игре: { $set }
morse-score-run = Счёт: { $correct } / { $total }. Серия: { $streak } из { $value }.
morse-correct = верно — { $value }
morse-was = это было { $value }

## net: messages with values (src/app/net.rs)

net-adif-import-failed = Импорт ADIF не удался: { $error }
net-not-unicode-read = {" "}(не Unicode; прочитано как { $enc })
net-received-reports-swl-left-out = , { $n } принятых отчётов (SWL) не включено
net-adif-import-added-duplicates-skipped = Импорт ADIF: добавлено { $added }, дубликатов пропущено { $skipped }{ $reports }{ $assumed }
net-channel-import-failed = Импорт каналов не удался: { $error }
net-line-s-skipped = , пропущено строк: { $n }
net-not-unicode-read-2 = {" "}(не Unicode; прочитано как { $enc })
net-channel-import-channel-s-read = Импорт каналов: прочитано каналов: { $n }{ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = кадров: { $frames } · плохих: { $bad }
acars-mode = режим { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = { $aircraft } бортов
adsb-frames = кадров: { $frames }
adsb-preambles = преамбул: { $preambles }
adsb-bad-crc = плохих CRC: { $bad_crc }
adsb-tune = НАСТРОИТЬ { $mhz }
adsb-ft-barometric = { $ft } фт, барометрическая

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = { $vessels } судов
ais-messages = { $messages } сообщений
ais-slots = { $bursts } слотов
ais-bad-fcs = плохих FCS: { $bad_fcs }
ais-ais-mhz = AIS { $value } ({ $mhz } МГц) — { $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value } ({ $mhz } МГц) — { $bursts } слотов, { $messages } сообщений, пол { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = НАСТРОИТЬ { $mhz }
ais-ships-heard-are-khz-off = Слышные суда смещены по частоте на { $khz } кГц ({ $mhz } ppm на 162 МГц) — задайте коррекцию частоты фронтенда, иначе передачи дальше примерно 5 кГц перестанут декодироваться
ais-over-ground = { $value }° над землёй
ais-dbfs-db-snr = { $rssi_dbfs } dBFS, { $snr_db } дБ SNR
ais-last-type-ais = { $messages } (последний тип { $last_type }, AIS { $channel })
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } МГц — морской канал { $ch }. Судно переключается между ними, поэтому выключение одного вдвое уменьшает, как часто слышно каждое судно.

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = объект от { $reported_by }
aprs-via = через { $via }
aprs-object-reported = объект сообщён { $reported_by }
aprs-digit-s-blanked = ± скрыто цифр: { $ambiguity }
aprs-wind-m-s = ветер { $value }° { $value2 } м/с
aprs-gust = порывы { $value }
aprs-rain-mm-h = дождь { $value } мм/ч
aprs-frame-s-last-ago = кадров: { $packets }, последний { $last_heard } назад
aprs-via-2 = через { $via }
aprs-bad = { $bad_frames } плохих
aprs-other = { $non_aprs } прочих
aprs-sent = отправлено ×{ $tries }
aprs-stn = станций: { $stations }
aprs-queued = в очереди: { $tx_queue }

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = мастер { $value }
atchat-last-heard-s-ago-click = { $status } — последний раз слышно { $age_s } с назад · нажмите, чтобы открыть вкладку личных сообщений
atchat-close-tab = закрыть вкладку { $peer }
atchat-direct-message = личное сообщение для { $value }
atchat-cannot-display = не удаётся показать { $path }

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Click for the { $neural } decoder. The neural one copies further down and reads hand-sent CW a timing fit will not accept; the timing one is the only one that copies the accented letters — Ä, Ö, Å, Ü, É — because the model has no output class for them.
cw-idle-s = ПАУЗА { $idle } с

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = Сохранить { $n } декодирований из списка как ADIF — принятые отчёты, а не связи. Декодирования без отправителя пропускаются.
decodes-save-decodes-list-csv-one = Сохранить { $n } декодирований из списка как CSV, по одной строке на каждое
decodes-session-qso = Сессия: QSO — { $session }
decodes-save-whole-logbook-qso-adif = Сохранить весь журнал ({ $logged } QSO) как ADIF
decodes-save-whole-logbook-qso-text = Сохранить весь журнал ({ $logged } QSO) как текст
decodes-hz-slots = { $audio_hz } Гц · слотов: { $odd }
decodes-your-slot-timing-against-stations =
    Ваша синхронизация слотов относительно станций, которые вы слышите.
    { $after }

    It covers the whole receive path, so a slow audio or network chain counts the same as a wrong clock. Under 0.5 s is comfortable.
decodes-pile-up = ПАЙЛ-АП { $fox_queue }
decodes-queue = ОЧЕРЕДЬ { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } дБ · { $audio_hz } Гц{ $unwrap_or_default }
    Нажмите, чтобы убрать

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = Декодируются все скорости; ▸ отмечает { $speed } — ту, на которой вы передаёте
js8-every-speed-being-decoded-click = Декодируются все скорости. Нажмите, чтобы передавать на { $speed }
js8-transmit-decode = Передавать и декодировать на { $speed }
js8-beaconing-every-min-click-stop = Маяк каждые { $hb_min } мин — нажмите, чтобы остановить
js8-last-beacon-went-out-hz = Последний маяк ушёл на { $hz } Гц — свободный слот в поддиапазоне heartbeat { $value }–{ $value2 } Гц, выбранный так, чтобы не попадать в декодируемые сигналы.
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } кадров · { $left } с
js8-decoded = Декодировано на { $speed }
js8-frames = … (кадров: { $frames })
js8-reply-2 = Ответ для { $value }: «{ $value2 }»
js8-address-composer = Адресовать редактор на { $value }

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i }: отправляет «{ $text }»
macros-sends = Отправляет «{ $text }»
macros-most = { $value } — максимум

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = исправлено: { $repaired } · потеряно: { $lost }
navtex-characters-lost = потеряно знаков: { $lost }

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = { $bad_frames } плохих
packet-link-layer = Канальный уровень в { $link }
packet-via = {" "}через { $via }
packet-unacked = неподтверждённых: { $unacked }
packet-retry = повтор { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] маяк
        [few] маяка
       *[other] маяков
    }
pi4-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } назад

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = SNR { $snr_db } дБ
rade-offset-hz = смещение { $freq_offset_hz } Гц
rade-heard = слышно { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ { $dropped } отсчётов потеряно между приёмником и декодером — эта машина не успевает за нейросетевым декодированием.

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } знаков/с
textmodem-char-s-hz-wide = { $value } — { $chars_per_sec } знаков/с, ширина { $bandwidth_hz } Гц, { $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = кадров: { $frames }
vdl2-bursts = пачек: { $bursts }
vdl2-sync = синхр.: { $syncs }
vdl2-hdlc-bad = плохих HDLC: { $hdlc_bad }
vdl2-bad-fcs = плохих FCS: { $fcs_bad }
vdl2-mhz-bursts-frames-noise-floor = { $mhz } МГц, { $what } — пачек: { $bursts }, кадров: { $frames }, шумовой пол { $floor_dbfs } dBFS
vdl2-tune = НАСТРОИТЬ { $mhz }
vdl2-held-2 = УДЕРЖАНО +{ $since }
vdl2-use-worldwide-where-every-link = { $what } — используется по всему миру и там, где начинается каждая линия. Тот, что стоит оставить, если оставляете только один.
vdl2-assigned-which-channels-carry-anything = Назначен { $what }. Какие каналы несут что-либо, зависит от места: оставьте все включёнными, если не знаете иного.
vdl2-mhz-db-snr-dbfs-evm = { $mhz } МГц · { $snr_db } дБ SNR · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Гц · исправлено символов RS: { $rs_corrected }
vdl2-characters = { $parity_errors } знаков
vdl2-parameter-sdroxide-does-not-read = { $unknown } { $unknown ->
        [one] параметр
        [few] параметра
       *[other] параметров
    } SDRoxide не читает — показаны выше как идентификатор и байты

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = удержано строк: { $live_size }
wefax-pixels-per-line = { $width } пикселей в строке
wefax-shift-picture-pixels = Сдвинуть изображение на { $px } пикселей
wefax-charts-are-saved-click-copy =
    Карты сохраняются { $place }

    Click to copy the path
wefax-finished-charts-are-saved-collect = Готовые карты сохраняются { $place } и собираются здесь.
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } пикселей
    { $value }

    Right-click to delete
wefax-older-store = В хранилище ещё { $older } старых
wefax-older-load-more = Ещё { $older } старых — загрузить больше
wefax-published-carrier-khz-dial-khz = Опубликованная несущая { $f } кГц → шкала { $value } кГц USB
wefax-delete-chart-2 = Удалить эту карту { $dir }

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · зона CQ { $cq_zone } · зона ITU { $itu_zone }
widgets-new-entity = Новое образование на { $band_label }
widgets-worked-before = Работалось раньше на { $band_label }
widgets-calling-cq-includes-you = Вызов CQ { $target } — включает вас
widgets-calling-cq-not-aimed-you = Вызов CQ { $target } — не адресован вам

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = { $heard_us } rx · { $heard_us2 } услышали нас
wspr-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] маяк
        [few] маяка
       *[other] маяков
    }
wspr-beaconing-duty = маяк — { $wspr_power_dbm }, загрузка { $wspr_tx_percent }%
wspr-listening-duty = приём (загрузка { $wspr_tx_percent }%)
wspr-beacon-one-two-minute-slot = Маяк в одном двухминутном слоте из каждых { $pct } — { $pct2 }% от них, равномерно распределённых — на { $wspr_power_dbm }. На каком слоте начинается цикл, определяется вашим позывным, поэтому две станции с этой программой не начинают вместе.
wspr-transmitting-set-general-tab-settings = Передача как { $call } в { $sent } — задаётся на вкладке «Общие» в Настройках.
wspr-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } назад

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    антенна: { $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = приёмников: { $entries } · { $per_net } · загружено { $age } назад
publicsdr-showing-search-narrow = показано { $shown } из { $n } — поиском сузьте список
publicsdr-match = совпадений: { $n }
publicsdr-replace-taking-receiver-points-same =
    Заменить { $this } на { $entry }?

    { $this2 } is on { $iface }. Taking this receiver points the same tab at { $address } instead, renames it “{ $entry2 }”, and holds the dial to the { $range_label } the receiver publishes — nothing here transmits.

    The { $iface2 } settings stay where they are, and so do the ranges stated for them: switching the interface back in Settings → Radio brings the radio back as it was.
publicsdr-receptions-will-reported = {" "}Приёмы будут сообщаться из { $value }.
publicsdr-pointing-radio = Направляю это радио на { $entry }…{ $site }
publicsdr-opening-another-radio = Открываю { $entry } как другое радио…

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } кГц
qo100-before-switching-tune-so-beacon = Перед включением настройтесь так, чтобы два лепестка маяка лежали между +{ $park_lo_hz } и +{ $park_hi_hz } кГц на полосе (затенённая дорожка).
qo100-receiver-currently-listening-around-mhz = Приёмник сейчас слушает около { $mhz } МГц — далеко от маяка 10489,750 МГц, поэтому здесь нечего рисовать. Декодер всё равно продолжает поиск.
qo100-receiver-currently-covers-only-khz = приёмник здесь охватывает только ±{ $khz } кГц — остальная часть полосы остаётся пустой
qo100-null-db-sym-snr-db = провал { $est_null_depth_db } дБ сим. { $est_symmetry } SNR { $est_snr_db } дБ
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Гц/с { $est_drift_accel_hz_s2 } Гц/с² (сняты вращения до декодирования)
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Гц/с (сняты вращения до декодирования)
qo100-found-empty = найдено { $est_updates }, пусто { $est_misses }
qo100-last-s-ago = {"   "}последний { $auto_last_hz } ({ $auto_last_unix } с назад)
qo100-over-correction = { $auto_total_hz } за { $auto_applies } { $auto_applies ->
        [one] поправку
        [few] поправки
       *[other] поправок
    }{ $last }
qo100-sync-32-err = синхр. { $sync_bit_errors } / 32 ошибок ×{ $sync_matches }
qo100-frame-buffer = буфер кадра { $frame_fill }%
qo100-blocks-tried-decoded = попыток блоков: { $blocks_tried }, декодировано: { $blocks_locked }
qo100-last-applied-s-ago-hz = последняя применённая { $ago } с назад: { $old } → { $new } Гц ({ $old2 } Гц)
qo100-locked-block-tried-locked = захвачено — попыток блоков: { $blocks_tried }, захвачено: { $blocks_locked }
qo100-searching-block-tried-none-locked = поиск — попыток блоков: { $blocks_tried }, пока ничего не захвачено

## rds: messages with values (src/app/rds.rs)

rds-auto = Авто ({ $standard })
rds-type = тип { $class }
rds-groups = · групп: { $groups }
rds-blocks-error = · { $ber } % блоков с ошибкой
rds-clean-corrected-lost = чисто: { $blocks_ok } · исправлено: { $blocks_corrected } · потеряно: { $blocks_bad }

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC (местное { $offset })
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } над горизонтом
sat-min = через { $value } мин
sat-h-min = через { $value } ч { $value2 } мин
sat-searching = поиск… { $next } из { $entries }
sat-locked-2 = ● ЗАХВАЧЕН — { $t }
sat-pass-until-max = Пролёт до { $set_unix } · макс. { $max_el }°
sat-next-pass-rises-max = Следующий пролёт { $rise_unix } { $rise_unix2 } ({ $now }) · восход { $rise_az }° { $rise_az2 } · макс. { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ подключено клиентов rigctld: { $clients } — спутниковое ПО, управляющее шкалой там, поправит доплер дважды
sat-rotator-antenna-az-el = Ротатор: антенна на азимуте { $az }° возвышение { $el }°
sat-rotator-2 = Ротатор: { $error }
sat-up-now-2 = ● сейчас над горизонтом, { $el }°
sat-inv = { $mode } · инв.
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = удерживается { $here } МГц
scanner-scanning-mhz = сканирование · { $here } МГц

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = Этот рендерер не удержит текстуру шире { $max_texture_dim } пикселей.

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = Голос: { $voice }
speech-speech-unavailable = Речь недоступна: { $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = передач: { $stations }

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = последний сеанс не удался — { $err }
winlink-last-session-received-sent = последний сеанс: получено { $last_received }, отправлено { $last_sent }
winlink-empty = { $to_lowercase } пусто
winlink-showing = показано { $entries } из { $total }
winlink-fetching = загрузка { $mid }…
winlink-from-2 = от { $msg } { $date }
winlink-attachment-bytes = вложение: { $att } ({ $data } байт)

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    объект от { $reported_by }
aprsmap-heard-ago =
    {""}
    слышно { $age } назад

## download: messages with values (src/download.rs)

download-reading = чтение { $path }: { $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}отсутствует изображение: { $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } МГц
hfdlmap-fix =
    {""}
    исправлений: { $fixes }
hfdlmap-heard-ago =
    {""}
    слышно { $last_at } назад

## multi: messages with values (src/multi.rs)

multi-radio = Радио { $n }
multi-could-not-add-radio = Не удалось добавить радио: { $error }
multi-was-closed-station = { $value } закрыто на станции.
multi-could-not-open = Не удалось открыть { $peer }: { $error }
multi-has-tab-own-now = У { $value } теперь своя вкладка.

## remote: messages with values (src/remote.rs)

remote-protocol-error = ошибка протокола: { $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = неверное сообщение: { $error }
solarnet-solar-protocol-mismatch-server-client = несовпадение протокола солнечного вида: сервер { $proto }, клиент { $value }

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
smeter-received-signal-strength-s9-db = Уровень принятого сигнала: S9 + { $over } дБ ({ $s_dbm } дБм в полосе пропускания).
smeter-received-signal-strength-s-dbm = Уровень принятого сигнала: S{ $value } ({ $s_dbm } дБм в полосе пропускания).
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

specview-rit-hz = RIT { $hz } Гц
specview-xit-hz = XIT { $hz } Гц

## decodes: nested messages

decodes-you-transmit-s-everyone-else = Вы передаёте на { $off } с { $after } относительно всех остальных — обычная причина, по которой вызовы остаются без ответа. Синхронизируйте часы компьютера.

## publicsdr: nested messages

publicsdr-noise-floor-score = оценка по шумовому полу { $snr }
publicsdr-up-khz-i-q = до { $khz } кГц I/Q

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = GR гейт { $gate_gr_db } де-эссер { $deess_gr_db } компр. { $comp_gr_db } лим. { $limiter_gr_db }

## amp: more messages

amp-h-step = ~{ $l_step } мкГн · шаг { $l_step2 }
amp-pf-step = ~{ $c_step } пФ · шаг { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel }-бит · { $media_type } / { $content_encoding } · { $encoded_size } октетов в { $chunk_count } фрагментах ({ $chunks_first_pass } в первом проходе) · сессия { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value } — { $symbol_rate } бод CPFSK, ±{ $deviation_hz } Гц, занятая полоса { $bandwidth_hz } кГц

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · местное { $local }
sat-in-seconds = через { $s } с
winlink-to-list = кому { $to }
winlink-cc-list = копия { $cc }
spots-n-of-total = { $n } из { $total }
wefax-n-of-total = { $n } из { $total }
ais-range-bearing = { $km } км, пеленг { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = Выкл. (только тюнер, от 24 МГц)
rtl-sdr-hf-mode-auto = Автоматически
rtl-sdr-hf-mode-direct-q = Прямое сэмплирование (ветвь Q)
rtl-sdr-agc-manual = Ручная (без AGC)
rtl-sdr-agc-tuner = AGC тюнера
rtl-sdr-agc-rtl = Цифровой AGC RTL
rtl-sdr-agc-both = AGC тюнера + RTL
mode-control-cat = CAT
mode-control-radio = Управляется радио
line-state-none = Нет
line-state-high = Высокий
line-state-low = Низкий
elad-tx-input-usb-audio = USB-звук
elad-tx-input-mic = Микрофон
elad-tx-input-auto = Авто
elad-tx-input-leave = Оставить как задано на радио
tr-usdx-audio-one-cable = Один кабель (звук в потоке CAT)
tr-usdx-audio-sound-card = USB-звуковая карта (разъём 3,5 мм)
spy-server-format-uint8 = 8 бит (самая низкая полоса)
spy-server-format-int16 = 16 бит
spy-server-format-float32 = 32-битный float
sound-format-iq = IQ (стерео)
sound-format-demod-audio = Демодулированный звук
sdr-play-duo-tuner-tuner1 = Тюнер 1 (50 Ом / Hi-Z)
sdr-play-duo-tuner-tuner2 = Тюнер 2 (50 Ом)
sdr-play-duo-role-diversity = Вторая антенна (разнесённый приём / подавление помех)
sdr-play-duo-role-second-radio = Второе радио, на собственной частоте
sdr-play-agc-off = Выкл. (ручное усиление ПЧ)
sdr-play-agc-hz5 = 5 Гц
sdr-play-agc-hz50 = 50 Гц
sdr-play-agc-hz100 = 100 Гц
pluto-ptt-off = Выкл.
pluto-ptt-gpo01 = GPO0 = RX, GPO1 = TX
pluto-ptt-gpo23 = GPO2 = RX, GPO3 = TX
pluto-duplex-fdd = FDD (оба одновременно)
pluto-duplex-tdd = TDD (по одному)
pluto-agc-manual = Ручной
pluto-agc-slow-attack = Медленная атака
pluto-agc-fast-attack = Быстрая атака
pluto-agc-hybrid = Гибридный
parity-none = Нет
parity-even = Чётность
parity-odd = Нечётность
panadapter-tap-antenna = Антенна (общая с радио)
panadapter-tap-if-output = Выход ПЧ радио
panadapter-audio-attached = Подключённый приёмник
panadapter-audio-transceiver = Трансивер
kenwood-send-ts2000 = В стиле TS-2000 (TX;)
kenwood-send-ts590 = В стиле TS-590 (TX1;)
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = Управляется радио
cw-keying-cat = Манипулятор аппарата (CAT)
cw-keying-audio = Звуковая карта (MCW)
alert-sound-ding = Дзынь
alert-sound-two-tone = Двухтональный
alert-sound-triplet = Тройной
alert-sound-warble = Трель
alert-sound-digital = Цифровой
alert-reply-tone = Тон
alert-reply-voice = Голос
alert-reply-both = Тон + голос
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
icom-model-other = Другой (адрес задать вручную)
backend-auto = Автоопределение (SoapySDR / CAT)
backend-soapy = SoapySDR
backend-cat = CAT / звук
backend-hpsdr = HPSDR (сеть)
backend-tci = TCI (сеть)
backend-icom-net = Icom LAN (сеть)
backend-smart-sdr = SmartSDR / FlexRadio (сеть)
backend-pluto = PlutoSDR (сеть)
backend-rtl-sdr = RTL-SDR (USB)
backend-rtl-tcp = RTL-SDR через rtl_tcp (сеть)
backend-spy-server = SpyServer (сеть)
backend-spy-server-vfo = SpyServer VFO+FFT, низкая полоса (сеть)
backend-kiwi-sdr = KiwiSDR / Web-888 (сеть)
backend-rx888 = RX-888 (USB)
backend-airspy-hf = Airspy HF+ (USB)
backend-airspy = Airspy R2 / Mini (USB)
backend-hack-rf = HackRF One / Pro (USB)
backend-sdr-play = SDRplay RSP (USB)
backend-elad = ELAD FDM-DUO / FDM-S (USB)
backend-lime = LimeSDR + LimeRFE (LimeSuite)
backend-hydra-sdr = HydraSDR RFOne (USB)
backend-fobos = RigExpert Fobos SDR (USB)
backend-usb-audio = USB-аудио радио (звуковая карта)
backend-none = Не настроено
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld (сеть)
cat-family-flrig = flrig (сеть)
publicsdr-its-interface = его интерфейс

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = Выкл.
agc-mode-slow = Медл.
agc-mode-med = Средн.
agc-mode-fast = Быстр.
airspy-gain-linearity = Линейность (лучше для сильных сигналов)
airspy-gain-sensitivity = Чувствительность (лучше для слабых сигналов)
ais-kind-class-a = Класс A
ais-kind-class-b = Класс B
ais-kind-base-station = базовая станция
ais-kind-sar-aircraft = самолёт SAR
ais-kind-aid-to-navigation = навигационный знак
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = сопутствующее судно
alert-event-called = Станция вызывает меня
alert-event-cq = CQ, на который я могу ответить
alert-event-new-dxcc = Новое образование DXCC
alert-event-new-dxcc-band = Новое образование на этом диапазоне
alert-event-new-grid = Новый квадрат
audio-routing-both = Левый + правый
audio-routing-left = Только левый
audio-routing-right = Только правый
button-mode-momentary = Удерживать
button-mode-toggle = Переключать
contest-mode-none = Нет
contest-mode-eu-vhf = EU VHF Contest
converter-tx-off = Выкл. при конвертировании
converter-tx-transverter = Через тот же конвертер
converter-tx-own = Собственное смещение
cw-engine-neural = НЕЙРОСЕТЬ
cw-engine-timing = ПО ВРЕМЕНИ
decode-sort-none = Нет
decode-sort-signal = SNR
decode-sort-distance = Расст.
decode-sort-country = Страна
diversity-mode-cancel = Подавить — погасить источник шума
diversity-mode-combine = Сложить — разнесённый приём
dxped-mode-normal = Обычный
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = Не передавать
fail-safe-warn-only = Всё равно передавать, с предупреждением
ftp-security-plain = FTP (без шифрования)
ftp-security-explicit-tls = FTPS явный (AUTH TLS)
ftp-security-implicit-tls = FTPS неявный
hpsdr-filter-board-none = Нет — выходы остаются выключенными
hpsdr-filter-board-n2adr = Плата фильтров N2ADR
hpsdr-filter-board-alex = Alex / код диапазона Hermes (Zeus, HiQSDR, Quisk)
hpsdr-filter-board-custom = Свой — моя таблица ниже
hpsdr-io-rx-input-radio = Собственный вход радио
hpsdr-io-rx-input-io-board = Плата IO J9
hpsdr-io-rx-input-io-board-pure-signal = Плата IO J9, PureSignal на передаче
hydra-sdr-gain-linearity = Линейность (лучше для сильных сигналов)
hydra-sdr-gain-sensitivity = Чувствительность (лучше для слабых сигналов)
icom-rx-source-af = AF — радио демодулирует
icom-rx-source-if12k = ПЧ 12 кГц — демодулирует sdroxide
icom-scope-span-radio = Как задано на радио
icom-scope-span-khz2-5 = 5 кГц (±2,5к)
icom-scope-span-khz5 = 10 кГц (±5к)
icom-scope-span-khz10 = 20 кГц (±10к)
icom-scope-span-khz25 = 50 кГц (±25к)
icom-scope-span-khz50 = 100 кГц (±50к)
icom-scope-span-khz100 = 200 кГц (±100к)
icom-scope-span-khz250 = 500 кГц (±250к)
icom-scope-span-khz500 = 1 МГц (±500к)
ism-family-weather = ПОГОДА
ism-family-meter = СЧЁТЧИКИ
ism-family-home-auto = ДОМ
ism-family-lo-ra = LORA
ism-family-unidentified = НЕИЗВЕСТНО
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = не опознан
js8-speed-normal = ОБЫЧНАЯ
js8-speed-fast = БЫСТРАЯ
js8-speed-turbo = TURBO
js8-speed-slow = МЕДЛЕННАЯ
lime-aux-role-off = Не используется
lime-aux-role-diversity = Вторая антенна (разнесённый приём / подавление помех)
lime-aux-role-pure-signal = Обратная связь передачи (предыскажение PureSignal)
lookup-provider-none = Выкл.
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = ВХОДЯЩИЕ
mail-folder-outbox = ИСХОДЯЩИЕ
mail-folder-sent = ОТПРАВЛЕННЫЕ
mail-folder-archive = АРХИВ
memory-sort-name = Название
memory-sort-freq = Частота
memory-sort-band = Диапазон
memory-sort-stored = Сохранено
mouse-button-middle = Средняя
mouse-button-extra1 = Назад (доп. 1)
mouse-button-extra2 = Вперёд (доп. 2)
nr-level-off = Выкл.
nr-level-low = Низк.
nr-level-medium = Средн.
nr-level-high = Высок.
nr-level-rnn-low = RNN низк.
nr-level-rnn-med = RNN средн.
nr-level-rnn-high = RNN высок.
nr-level-spec-low = SPEC низк.
nr-level-spec-med = SPEC средн.
nr-level-spec-high = SPEC высок.
nr-level-nr2-low = NR2 низк.
nr-level-nr2-med = NR2 средн.
nr-level-nr2-high = NR2 высок.
nr-level-df-low = DFNR низк.
nr-level-df-med = DFNR средн.
nr-level-df-high = DFNR высок.
nr-strength-low = Низк.
nr-strength-med = Средн.
nr-strength-high = Высок.
orbit-rings-none = нет
orbit-rings-curated = выборочно
orbit-rings-all = все
qso-step-idle = Простой
qso-step-wait-cq = Ожидание CQ
qso-step-calling-cq = Вызов CQ
qso-step-tx-grid = Передача квадрата
qso-step-tx-report = Передача отчёта
qso-step-tx-r-report = Передача R+отчёта
qso-step-tx-rr73 = Передача RR73
qso-step-tx73 = Передача 73
qso-step-confirming = Подтверждение
rds-standard-auto = Авто
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = Регион 1 — Европа, Африка, Ближний Восток, Северная Азия
region-r2 = Регион 2 — Америка
region-r3 = Регион 3 — Южная и Восточная Азия, Австралазия, Тихий океан
relative-mode-absolute = Абсолютный (фейдер)
relative-mode-sign-magnitude = Относительный: знак/величина
relative-mode-twos-complement = Относительный: дополнительный код
relative-mode-binary-offset = Относительный: центр 64
relay-link-off = Нет
relay-link-serial = Плата USB-реле (последовательный порт)
relay-link-serial-lines = Линия RTS/DTR последовательного порта
relay-link-hid = Плата USB-реле (HID)
relay-link-cm108 = GPIO звуковой карты CM108/CM119
relay-link-gpio = Линия GPIO Linux
relay-link-command = Внешняя команда
relay-role-unused = Не используется
relay-role-sdr-antenna = Заземлять антенну SDR
relay-role-amplifier = Манипулировать усилителем / реле T/R
relay-role-aux = Вспомогательный
relay-role-band-decoder = Декодер диапазонов (фильтр / трансвертер)
rfe-channel-wb1000 = Широкополосный 1 – 1000 МГц
rfe-channel-wb4000 = Широкополосный 1 – 4 ГГц
rfe-channel-ham0030 = КВ, до 30 МГц
rfe-channel-ham0070 = 6 м / 4 м (50 – 70 МГц)
rfe-channel-ham0145 = 2 м (140 – 150 МГц)
rfe-channel-ham0220 = 1,25 м (220 – 225 МГц)
rfe-channel-ham0435 = 70 см (400 – 450 МГц)
rfe-channel-ham0920 = 33 см (902 – 928 МГц)
rfe-channel-ham1280 = 23 см (1220 – 1420 МГц)
rfe-channel-ham2400 = 13 см (2,3 – 2,5 ГГц)
rfe-channel-ham3500 = 9 см (3,3 – 3,7 ГГц)
rfe-channel-cell-band01 = Сотовый диапазон 1
rfe-channel-cell-band02 = Сотовый диапазон 2
rfe-channel-cell-band03 = Сотовый диапазон 3
rfe-channel-cell-band07 = Сотовый диапазон 7
rfe-channel-cell-band38 = Сотовый диапазон 38
rfe-channel-not-selected = Не выбрано
rfe-link-off = Не подключено
rfe-link-serial = Собственный USB-кабель (последовательный порт)
rfe-link-board = Через LimeSDR (GPIO / I²C)
rfe-mode-control-auto = Автоматически (по разводке кабелей)
rfe-mode-control-rx = Всегда приём
rfe-mode-control-tx = Всегда передача
rfe-mode-control-tx-rx = Всегда оба (только разнесённые разъёмы)
rifp-encoding-auto = Авто
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = Без сжатия
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = ПАМЯТЬ
scan-kind-range = ДИАПАЗОН
scan-resume-carrier = НЕСУЩАЯ
scan-resume-timed = ПО ВРЕМЕНИ
scan-resume-manual = ВРУЧНУЮ
sense-line-off = Не подключено
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD (детектор несущей)
shift-simplex = СИМПЛЕКС
shift-minus = −
shift-plus = +
speed-slow = Медленно
speed-medium = Средне
speed-fast = Быстро
speed-faster = Быстрее
speed-fastest = Самый быстрый
tls-verify-system = Хранилище доверенных сертификатов системы
tls-verify-pinned = Закреплённый отпечаток
tls-verify-off = Не проверять
tone-mode-off = ВЫКЛ.
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN (по кругу)
upload-naming-original = Исходное имя файла
wheel-action-zoom = Масштаб
wheel-action-tune = Настройка
wheel-action-none = Ничего

settings-alerts-say = ОЗВУЧИТЬ

settings-alerts-say-hint = Прослушать это оповещение голосом

settings-alerts-say-need-voice = Выберите «Голос» или «Тон + голос», чтобы услышать

settings-alerts-say-voice-off = Включите голос в Настройки → Интерфейс → Голосовые оповещения

settings-alerts-speaks = произносит: { $text }

settings-picup-keep = оставить

jtty-starting-receiver = запуск приёмника JTTY…

jtty-receive-audio-level = Уровень звука приёма

jtty-sending = {" "}ПЕРЕДАЧА{" "}

jtty-sent-once-note = Сообщение отправляется один раз при нажатии TX — JTTY асинхронный, поэтому ждать период не нужно, и ничего не повторяется.

signalid-title = ОПОЗНАНИЕ СИГНАЛА

signalid-tuned-now = Сейчас настроено

signalid-passband = полоса { $bw }

signalid-band = диапазон { $band }

signalid-find = Найти

signalid-find-hint = FT8, метеор, погода, самолёты…

signalid-candidates = Вариантов для этой настройки: { $n }

signalid-matches = Совпадений для «{ $query }»: { $n }

signalid-nothing-fits = В каталоге нет ничего, что подходит к этой частоте, виду излучения и полосе.

signalid-no-match = В каталоге нет подходящей записи.

signalid-open-url = Открыть { $url }

windows-chip-del = УДЛ

windows-chip-edt = ИЗМ

windows-chip-ren = ПЕРЕИМ

windows-chip-rec = ЗАП

windows-name-hint = имя

windows-waterfall-marks = Водопад

windows-stacked = стопкой

jtty-listening-note = Приём. Передачи JTTY могут начаться в любой момент — сообщение появится здесь вскоре после того, как будет услышано.

extras-title = ДОПОЛНИТЕЛЬНО

extras-receivers = Приёмники

extras-dab-hint = Цифровое радио DAB / DAB+ — плеер открывается под водопадом

extras-aircraft-watch-hint = САМОЛЁТЫ — самолёт из вашего списка наблюдения в эфире; откройте вид «Наблюдение»

extras-aircraft-hint = САМОЛЁТЫ — карта того, что слышит радио: самолёты и радиозонды, в отдельном окне

extras-radio-hint = RADIO CLASSIC — классическая панель радиоприёмника для вещательного прослушивания: диапазоны, память, название станции и RadioText. В голосовых видах излучения она закрепляется под водопадом; Настройки → Отсоединяемые выносят её в окно

extras-antenna = АНТЕННА

extras-antenna-hint = АНТЕННА (экспериментально/бета) — длины для резки, входное сопротивление и диаграмма направленности типовых проволочных/вертикальных антенн и антенн Уда–Яги

recjobs-title = ЗАПИСИ

recjobs-finished = Завершено «{ $name }»

recjobs-skipped = Пропущено «{ $name }» — запись уже идёт

recjobs-recording-name = Запись «{ $name }»

recjobs-new-job = + НОВОЕ ЗАДАНИЕ

recjobs-recording = ● идёт запись

recjobs-none = Запланированных записей нет — нажмите + НОВОЕ ЗАДАНИЕ, чтобы добавить.

recjobs-done = готово

recjobs-edit = изм.

recjobs-del = удл.

recjobs-station = Станция

recjobs-station-hint = BBC World Service

recjobs-kind = Тип

recjobs-start-utc = Начало (UTC)

recjobs-for = Длительность

recjobs-save = СОХРАНИТЬ

recjobs-kind-audio = Звук

recjobs-kind-iq = I/Q

recjobs-kind-both = Оба

panels-sensitivity = Чувствительность

navtex-hunting = поиск

navtex-hunting-hint =
    Пока ничего не читается.

    Почти всегда дело в настройке. НЕ вводите объявленное число (518) в поле частоты: NAVTEX указывают как канал, а программа слушает чуть ниже него. Нажмите кнопку 518 выше — она сама установит правильную частоту (516.300).

    Если сигнал явно сильный, а текст всё равно бессмысленный, нажмите REV: сигнал приходит в обратной полярности.

    Помните также, что NAVTEX работает по расписанию, а не как диалог: береговая станция передаёт десять минут каждые четыре часа, так что тишина — это нормально в большую часть времени. Нужно слушать тогда, когда ваша станция в эфире. А 518 кГц оживает только ночью — тихий дневной диапазон не означает, что декодер сломан.

navtex-sensitivity-hint =
    Чувствительность — насколько легко декодер захватывает сигнал и удерживает его.
navtex-auto-hint = Возврат к опорным порогам (0). Подсвечено, пока активно.

navtex-tune-dial-usb-below-channel = Настройте шкалу сюда (USB, на 1.710 кГц ниже канала).

settings-audio-your-audio = Ваш звук

settings-audio-your-audio-why = Колонки, на которых вы слушаете, и микрофон, через который уходит ваш голос. Они относятся к этому компьютеру, а не к какому-либо радио, поэтому одинаковы для любого радио, с которым вы работаете.

settings-audio-browser-note = Звуковые устройства здесь выбираются в браузере, а не в этом окне — страница запрашивает у браузера колонки и микрофон. Настройки звуковой карты вашей станции, ниже на этой странице, по-прежнему действуют.

settings-audio-speakers = Колонки

settings-audio-channels-hint = На какой канал (каналы) колонок поступает принимаемый звук. Левый или правый отключает другой, чтобы слушать только одной стороной.

settings-audio-microphone-note = Микрофон передаёт ваш голос, когда трансивер включается на передачу звуком (MCW) или по VOX. Если ваш трансивер включается по CAT, микрофон на передачу не используется вообще, и эта строка ничего не меняет.

settings-audio-stream-out = Передавать звук наружу

settings-audio-stream-out-why = Отправлять то, что принимает это радио, в другую программу по сети — декодер, рекордер, второй компьютер — минуя звуковую карту.

settings-audio-send-to-hint =
    `host:port`. Моно, 16 бит, с частотой приёма, одна датаграмма на звуковой блок; это звук **записывающего отвода**: громкость НЧ, MUTE и передача на него не влияют, потому что внешнему декодеру нужен сигнал с радио, а не уровень ваших наушников.

    Числовой адрес, а не имя хоста. Пусто — выключено.

settings-audio-send-to-note = `127.0.0.1:7355` отправляет звук в другую программу на этом компьютере; собственный адрес этого компьютера отправляет его по сети.

settings-audio-radio-audio = Звук этого радио

settings-audio-radio-audio-why = Звуковая карта между этим радио и этим компьютером. Она относится к радио, а не к компьютеру: два радио могут работать с одним компьютером и всё же требовать разных карт, поэтому эти настройки остаются за тем радио, из которого вы открыли Настройки.

settings-audio-no-cards-yet = Этот компьютер ещё не сообщил о своих звуковых картах. Имена всё равно можно ввести ниже; список появится, когда устройства будут опрошены.

settings-audio-rescan = Повторно опросить звуковые карты

settings-audio-rescan-hint =
    Ещё раз спросить у компьютера, какие у него звуковые карты. Ничего не открывается, поэтому нажимать можно и во время приёма.

    Используйте после подключения кодека. Это не переоткрывает радио — для этого служит Применить / переподключить на вкладке «Радио» — и не может исправить карту, переименованную звуковым сервером; об этом сообщается на радио.

settings-audio-from-radio = С радио

settings-audio-to-radio = На радио

settings-audio-radio-note = { "*" }*С радио** — то, что выдаёт ваш трансивер: USB-звуковой выход или гнездо наушников приёмника, подключённое к линейному входу звуковой карты. **На радио** — то, что трансивер слушает как ваш голос, а также способ, которым программа включает трансивер на передачу звуком: оставьте пустым, и будет использоваться собственный микрофон трансивера, если он есть.

settings-audio-recording = Запись

settings-audio-recording-why = Куда записываются аудиозаписи, запланированные записи и необработанные I/Q-захваты.

enigma-need-more = дайте ему больше шифртекста для работы

enigma-title = ENIGMA

enigma-no-cables = без кабелей

enigma-thin = мало

enigma-rotor = Ротор

enigma-window-hint = Grundstellung — нажмите, чтобы провернуть колесо

enigma-input-hint = вводите текст или используйте клавиши выше

enigma-clear = ОЧИСТИТЬ

enigma-copy = КОПИРОВАТЬ ⇄

enigma-solve-note = LÖSEN — вставьте шифртекст. Для текста с коммутацией укажите слово-подсказку (crib), которое ожидаете в сообщении; без него решатель работает с машиной без коммутации.

enigma-crib = Crib

enigma-solve = РЕШИТЬ

enigma-flaw = Буква никогда не шифруется сама в себя — за этим следит рефлектор. Это недостаток настоящей машины, и именно по нему решатель отсекает варианты.

settings-audio-takes-over-speakers = { "*" }*Эти колонки — устройство звукового сервера, а не звуковая карта.** Их использование делает их устройством воспроизведения и для *всего остального* на этом компьютере — другие программы тоже будут играть через них. В PipeWire одна программа не может выбрать собственное устройство воспроизведения, так что этого не избежать. Чтобы изменить: выберите звуковую карту или верните устройство по умолчанию (`wpctl set-default <name>`) и повторите опрос.

settings-audio-takes-over-microphone = { "*" }*Этот микрофон — устройство звукового сервера, а не звуковая карта.** Его использование делает его устройством записи и для *всего остального* на этом компьютере — другие программы тоже будут записывать с него. В PipeWire одна программа не может выбрать собственный источник записи, так что этого не избежать. Чтобы изменить: выберите звуковую карту или верните устройство по умолчанию (`wpctl set-default <name>`) и повторите опрос.

settings-audio-currently-chosen = ⚠  Выбрано сейчас: { $device }

knowncalls-no-table = В этом режиме таблица позывных не ведётся. Это декодер FT8/FT4, хэширующий позывные — в остальных режимах показывать здесь нечего.

knowncalls-not-asked = Ещё не запрошено — нажмите ОБНОВИТЬ.

knowncalls-nobody = Пока никого. Станция добавляется в тот момент, когда слышно, как она называет свой позывной — CQ, рапорт с обоими позывными открытым текстом или простой вызов на 11 м.

knowncalls-title = ИЗВЕСТНЫЕ СТАНЦИИ

knowncalls-intro = Все позывные, которые хэш <...> на этом приёмнике может сейчас назвать. Хэш нельзя прочитать в обратную сторону — станция появляется здесь, только когда слышно, как она называет свой позывной, для чего и служит простой вызов на 11 м.

knowncalls-find-hint = позывной, страна, префикс…

knowncalls-refresh = ОБНОВИТЬ

knowncalls-capped = новейшие { $shown } из { $total } — ОБНОВИТЬ, чтобы запросить снова; список — это снимок

knowncalls-known = известно: { $shown }

knowncalls-matches =
    { $n } { $n ->
        [one] совпадение
        [few] совпадения
        [many] совпадений
       *[other] совпадения
    }

knowncalls-no-match = Нет известных станций, подходящих под запрос.

knowncalls-no-country = нет страны — позывной, неизвестный этой таблице

classic-controls = УПРАВЛЕНИЕ

classic-fold-the-column-away-to-a-rail-on = Свернуть колонку в рейку слева. Индикация и измеритель остаются на панели, а элемент, возвращающий колонку, находится на рейке.

classic-click-to-open-or-close-this = Нажмите, чтобы открыть или закрыть этот раздел. Перетащите его на другой раздел, чтобы переместить — порядок запоминается.

classic-bring-the-control-column-back = Вернуть колонку управления.

classic-no-radio = нет радио

classic-rate = Частота дискр.

classic-dial = Шкала

classic-receive = Приём

classic-listening-screen = экран прослушивания

classic-radio-settings = НАСТРОЙКИ РАДИО

classic-the-backend-the-sample-rate-the = Бэкенд, частота дискретизации, антенна, конвертер и собственная калибровка этого радио — Настройки → Радио, где они всегда и были. Колонка показывает их текущие значения, а не вторую копию элементов управления.

classic-counted-in-steps-not-decibels-what = Считается в шагах, а не в децибелах: чему равен один шаг — дело самого приёмника, а на SDRplay это ещё зависит от диапазона. Вправо по-прежнему означает больше усиления.

classic-the-plutosdr-s-own-agc-is-setting = Собственная АРУ PlutoSDR задаёт это усиление, поэтому движение ползунка ничего не меняет. Установите Настройки → Радио → АРУ в Ручной, чтобы управлять им здесь.

classic-squelch = Шумоподавитель

classic-gate-the-audio-below-this-power-in = Закрывать звук при мощности ниже этой в полосе приёма, в dBFS. Влево — открыт. Установите выше шума и ниже сигнала.

classic-this-radio-has-not-reported-what = Это радио ещё не сообщило о своих возможностях — связь закрыта или оно не ответило.

classic-open-the-link-in-the-radio-strip = Откройте связь в панели радио, и здесь появятся органы управления передачей.

classic-this-device-receives-only-it-has = Это устройство только принимает: у него нет передатчика, которым могла бы управлять программа.

classic-transmit-is-offered-by-a-rig-with = Передача доступна для трансивера с каналом управления или звука — Настройки → Радио.

classic-transmit-is-off-for-this-radio-s = Передача отключена для экрана этого радио.

classic-settings-radio-show-transmit = Настройки → Радио → «Показывать органы управления передачей» вернёт её.

classic-key = Ключ

classic-controlled-envelope-ssb-how-much = SSB с управляемой огибающей: насколько применять обработку речи. Больше — громче и площе, ценой некоторых расплёсков по обе стороны.

classic-holding-the-squelch-open-click-to = Шумоподавитель удерживается открытым. Нажмите, чтобы вернуть прежний порог.

classic-click-to-hold-the-squelch-open-to = Нажмите, чтобы удерживать шумоподавитель открытым — послушать, есть ли что-то в эфире — и нажмите ещё раз, чтобы вернуть всё как было.

panels-received = Принято

panels-text = Текст

panels-hex = Hex

panels-nothing-received-yet = Пока ничего не принято.

panels-payload = Данные

panels-blocks = Блоки

panels-sequence = Последовательность

uvpacket-starting-receiver = запуск приёмника UVPacket…

uvpacket-frames-count = кадров: { $n }

uvpacket-frames-count-hint = Кадры, декодированные с момента запуска приёмника.

uvpacket-intro = Пакетный режим для закрытых групп УКВ/УВЧ. Настройте шкалу на канал группы; подрежим определяется по каждому кадру.

uvpacket-nothing-yet = Пока ничего. UVPacket молчит между передачами — оставьте его на канале и дождитесь всплеска.

uvpacket-app-seq = прил. { $app } · посл. { $seq }

uvpacket-click-read-frame = Нажмите, чтобы прочитать кадр целиком.

uvpacket-bps-net = { $bps } бит/с чистыми

uvpacket-sub-mode = Подрежим

uvpacket-app-type = Тип приложения

uvpacket-payload-bytes = байт: { $n }

dsc-starting-receiver = запуск приёмника DSC…

dsc-seq-tone = посл.: { $n } · тон { $pct }%

dsc-seq-tone-hint = Последовательности, дошедшие до маркера конца, и насколько чётко разделяются два тона.

dsc-nothing-yet = Пока ничего. DSC молчит между судами — оставьте его на канале и дождитесь всплеска.

panels-marginal = на грани

dsc-marginal-hint = Символ не прошёл проверку BCH — показанные поля могут быть неверны.

dsc-click-read-sequence = Нажмите, чтобы прочитать последовательность целиком.

panels-marginal-caps = НА ГРАНИ

dsc-marginal-caps-hint = Хотя бы один символ не прошёл проверку BCH. Последовательность показана, потому что сигнал бедствия, принятый с искажениями, всё равно стоит прочитать.

dsc-category = Категория

dsc-nature = Характер

dsc-caller-mmsi = MMSI вызывающего

dsc-target-mmsi = MMSI адресата

dsc-alert-time = Время оповещения

dsc-working-ch = Рабочий кан.

dsc-raw-symbols = Исходные символы

dsc-none = (нет)

settings-general-device-type-name = …или введите имя — пусто означает системное по умолчанию

settings-general-signed-in-station = Станция, в которую выполнен вход

settings-general-sign-out-hint = Забывает вход на этом устройстве. При следующем подключении станция снова запросит его; текущее подключение остаётся до закрытия.

settings-general-recordings-remote = Записи сохраняются на компьютере, к которому подключено радио. Укажите папку там.

settings-general-folder = Папка

settings-general-choose = ВЫБРАТЬ…

settings-general-choose-hint = Выберите папку, в которую сохраняются записи

settings-general-default = ПО УМОЛЧ.

settings-general-default-hint = Вернуться к значению по умолчанию — папка «Музыка» или каталог конфигурации, если в системе её нет

settings-general-recordings-dir-note = Куда сохраняются аудиозаписи, запланированные записи и необработанные I/Q-захваты. По умолчанию: подпапка sdroxide в вашей папке «Музыка» или папка recordings в каталоге конфигурации, если система не предоставляет папку «Музыка».

setup-suffix-cq = { " " }cq

setup-suffix-tries = { " " }попыток

setup-window = окно

setup-south = юг

setup-west = запад

setup-message-templates = Шаблоны сообщений  { $tokens }

setup-ft8-decoder-effort-hint =
    Насколько усердно декодер FT8 работает над слабыми сигналами.

    Быстро — один проход, без вычитания сигналов: быстрее всего, меньше всего декодирований.
    Обычно — плоское многопроходное вычитание: ~0.4 с на загруженный слот, на несколько декодирований меньше, чем в режиме «Глубоко».
    Глубоко — проход WSJT-X с контрольными точками: ~1.2 с и больше всего декодирований (по умолчанию).

    Управляется только вторая партия декодирований; обычный проход всегда выдаётся первым, поэтому ответ никогда не задерживается из-за него.

setup-auto-cq-repeats-hint = Сколько вызовов CQ делает один цикл до завершения. 0 — собственного предела нет: цикл по-прежнему ограничивают сторожевой таймер передачи и остановка по бездействию.

setup-auto-cq-then-listen-hint = После этих вызовов CQ переключить АВТО CQ на АВТО RX и продолжать слушать, а не останавливаться совсем. Выключено — цикл останавливается.

setup-auto-call-retries-hint = Сколько раз вызывается одна станция, прежде чем цикл откажется от неё и перейдёт дальше по очереди.

setup-auto-session-hint = Как долго может длиться работа без присмотра, независимо от того, чем она занята. 0 — собственного предела нет.

setup-serial-contests-hint = Соревнования с серийными номерами — ARRL RTTY Roundup и CQ WPX через FT8: рапорт и серийный номер (или штат США), без локатора. CQ уходит как CQ RU.

setup-tx-level-fm-hint =
    С какой громкостью пакет этого режима передаётся радио, которое модулирует его само — а в FM это девиация, потому что FM-передатчик превращает уровень звука в частотный размах и не имеет ALC, который бы его ограничил. Пакет 1200 бод требует около 3 кГц, тогда как для голоса нужно 5, поэтому полная шкала на вход данных, настроенный под голос, даёт передевиацию — на слух это совершенно нормально, а декодироваться не будет ни у кого. Уменьшайте, пока другие станции не начнут сообщать о вас, или установите уровень на самом радио.

    Этот уровень относится только к этому режиму. Режим, который вы никогда не настраивали, начинает со значения для FM, а режимы с боковой полосой — со своих, так что девиация, заданная для 1200 бод, никогда не попадёт на FT8.

setup-tx-level-ssb-hint =
    С какой громкостью передача этого режима подаётся на радио, которое модулирует её само — CAT-трансивер на своей звуковой карте, FLEX, Icom на сетевом порту. На боковой полосе это возбуждение модулятора: уменьшайте, пока ALC трансивера едва шевелится, а мощность задавайте на самом радио, потому что ALC на режиме с постоянной огибающей как раз и создаёт расплёски. Здесь возбуждение попадает в регистр мощности трансивера, а не в его звуковой тракт, поэтому именно это и есть уровень.

    Этот уровень относится только к этому режиму: FT8, RTTY, PSK и MCW хранят свои собственные, потому что нагружают модулятор по-разному. Такой же регулятор есть на панели передачи, где до него можно дотянуться во время передачи.

setup-repeats-hint-base = Безответных вызовов одной станции перед переходом к следующей. Вызов CQ не считается.

setup-repeats-hint-duration = { " " }При длине слота { $mode } ({ $slot } с) это около { $mins } мин.

setup-repeats-hint-disables = { " " }0 отключает это.

setup-repeats-hint-11m = { " " }На 11 м это также то, что завершает цикл CQ: сторожевой таймер передачи его не прерывает, потому что станция, отвечающая на CQ, ожидается примерно в течение 30 с, а часы не отличают свежий цикл от устаревшего.

setup-symbol-home-station = Домашняя станция

setup-symbol-car = Автомобиль

setup-symbol-truck = Грузовик

setup-symbol-van = Фургон

setup-symbol-motorcycle = Мотоцикл

setup-symbol-bicycle = Велосипед

setup-symbol-person-on-foot = Пешеход

setup-symbol-motorhome = Автодом

setup-symbol-boat = Лодка

setup-symbol-yacht = Яхта

setup-symbol-light-aircraft = Лёгкий самолёт

setup-symbol-balloon = Воздушный шар

setup-symbol-digipeater = Цифровой ретранслятор

setup-symbol-i-gate = I-gate

setup-symbol-repeater = Ретранслятор

setup-symbol-weather-station = Метеостанция

setup-symbol-hospital = Больница

setup-symbol-portable-campsite = Портатив / кемпинг

settings-general-sign-out = ВЫЙТИ

settings-general-fallback-password = Пароль запоминается в этом браузере для { $stations }, потому что станция не ответила cookie входа. Так делают старые станции; это не то же самое.

settings-general-device-hint-speakers = Точное имя, по которому открываются ваши колонки. Выберите его из списка, если оно там есть; введите вручную, если списке его нет — звуковой сервер может предлагать устройство, которое API хоста не перечисляет. Пусто — системное по умолчанию.

settings-general-device-hint-microphone = Точное имя, по которому открывается ваш микрофон. Выберите его из списка, если оно там есть; введите вручную, если в списке его нет — звуковой сервер может предлагать устройство, которое API хоста не перечисляет. Пусто — системное по умолчанию.

settings-general-device-hint-radio-rx = Точное имя, по которому открывается звук приёма этого радио. Выберите его из списка, если оно там есть; введите вручную, если в списке его нет — звуковой сервер может предлагать устройство, которое API хоста не перечисляет. Пусто — системное по умолчанию.

settings-general-device-hint-radio-tx = Точное имя, по которому открывается звук передачи этого радио. Выберите его из списка, если оно там есть; введите вручную, если в списке его нет — звуковой сервер может предлагать устройство, которое API хоста не перечисляет. Пусто — системное по умолчанию.

settings-general-device-hint-other = Точное имя, по которому открывается устройство. Выберите его из списка, если оно там есть; введите вручную, если в списке его нет — звуковой сервер может предлагать устройство, которое API хоста не перечисляет. Пусто — системное по умолчанию.

settings-general-cb-world = Мир / свободный диапазон — 80 каналов

settings-general-cb-cept = CEPT / ЕС — 40 каналов

settings-general-cb-germany = Германия — 80 каналов

settings-general-cb-uk = Великобритания — 27/81, 40 каналов

settings-general-cb-us = США — 40 каналов

settings-general-cb-australia = Австралия — 40 каналов

settings-general-cb-pmr446 = PMR446 — 16 каналов (446 МГц)

settings-general-cb-dpmr446 = dPMR446 — 32 канала (446 МГц)

wefax-receiving-progress = приём { $min } мин · строк: { $lines }

wefax-listening = приём в ожидании

wefax-status-hint =
    Ожидание карты.

    WEFAX работает по расписанию, а не как диалог: станция передаёт карты в заданное время, поэтому выбранная частота большую часть суток молчит — это нормально. Выберите станцию кнопкой STATIONS и проверьте время.

    Если сигнал есть, но картинка получается одного ровного тона (всё серое, чёрное или белое), дело в настройке или скорости — используйте индикацию +0 Гц и регулятор IOC/скорости и проверьте, что запись в STATIONS соответствует тому, что передаётся.

wefax-centre = ЦЕНТР

wefax-centre-hint = Сдвиньте шкалу на { $sign }{ $off } Гц, чтобы поднесущая снова оказалась на 1900 Гц — приёмник расстроен на { $off } Гц, из-за чего карта превращается в белый лист.

wefax-auto-start-hint = Начинать карту, когда слышен стартовый тон 300 Гц (IOC 576) или 675 Гц, либо, если тон потерян из-за замирания или под несущей, когда приходит несколько секунд фазирующего сигнала — и завершать текущую карту и фазировать новую при приходе следующей передачи, независимо от того, был ли перед этим стоп-тон. Несущая на канале ничего не запускает.

wefax-fit = ВПИСАТЬ

wefax-save-chart-hint = Сохранить карту с экрана на этот компьютер в том виде, в каком она успела прийти

wefax-waiting-for-chart = Настройтесь на факсимильное расписание в USB и дождитесь начала карты (стартовый тон или фазирующий сигнал) либо нажмите START, чтобы начать посреди карты.

wefax-carrier-dial-note = несущая { $carrier } · шкала { $dial } кГц (USB −1.9k)

wefax-carrier-note-hint = Метка станции называет опубликованную несущую; поднесущая радиофакса лежит на 1900 Гц выше неё, поэтому шкала намеренно на 1.9 кГц ниже. Если настроить шкалу на саму несущую, сигнал окажется вне полосы пропускания.

contest-title = СОРЕВНОВАНИЕ

contest-intro = Выберите соревнование и введите то, что вы передаёте. Журнал работает в любом режиме — вводите обмен для CW или SSB, а часть FT8 заполняется сама, если у соревнования есть формат для FT8.

contest-you-send = { "  " }вы передаёте: { $exchange }

contest-they-send = { "  " }они передают: { $exchange }

contest-cb-note = Активность CB / 11 м: рапорт и обмен свободным текстом — укажите в нём канал, имя или район, который используется в активности.

contest-your-exchange = Ваш обмен

contest-start = СТАРТ

contest-score-line = QSO { $qsos } · { $points } оч. · { $mults } множ. · ИТОГО { $total }

contest-rate = { $r10 } за 10 мин · { $r60 }/ч

contest-stop = СТОП

contest-estimate = счёт приблизительный — окончательно решает организатор

contest-call = ПОЗЫВНОЙ

contest-dupe = ДУБЛЬ

contest-sent = ПЕРЕД

contest-rcvd = ПРИН

contest-sending = передаётся { $rst }

contest-sending-ours = передаётся { $rst } { $ours }

contest-log = ЖУРНАЛ

playback-radio-n = { " " }· радио { $n }

playback-no-stamp = в имени нет метки — воспроизводится в текущем режиме

playback-title = ВОСПРОИЗВЕДЕНИЕ — записанный звук

playback-find-hint = режим, МГц, дата, имя…

playback-refresh-hint = Заново прочитать папку записей

playback-asking = Запрос записей у станции…

playback-folder = Папка: { $dir }

playback-none = Записей пока нет. Нажмите REC → MP3 или WAV, пока что-то идёт в эфире; файл появится здесь с именем из времени, частоты и режима.

playback-on = ▶ ИДЁТ

playback-play = ▶ ВОСПР.

playback-iq-hint = Это I/Q-захват, а не звук. Воспроизведение берёт звук, который пишет рекордер; I/Q-захват воспроизводится через --file.

playback-not-while-tx = Нельзя во время передачи

playback-play-hint = Воспроизвести эту запись в декодеры с начала

playback-busy-playing = Идёт воспроизведение — остановите его перед удалением

playback-busy-recording = Запись ещё идёт — остановите REC перед удалением

playback-delete-ask = УДАЛИТЬ?

playback-delete-hint-armed = Нажмите ещё раз, чтобы удалить этот файл навсегда. Отменить нельзя.

playback-delete-hint = Удалить эту запись из папки (сначала спросит ещё раз)

playback-strip-label = ▶ ВОСПРОИЗВЕДЕНИЕ

playback-not-live = не в эфире — радио отключено

playback-carry-on = Продолжить воспроизведение

playback-pause = ⏸ ПАУЗА

playback-pause-hint = Пауза — декодеры слышат тишину, пока вы не продолжите

playback-again = ↺ ЗАНОВО

playback-again-hint = Вернуться к началу — ещё одна попытка получить картинку или декодирование

playback-loop = ЦИКЛ

playback-loop-hint = Начинать с начала, когда файл закончится

playback-stop = ■ СТОП

playback-stop-hint = Завершить воспроизведение и вернуть радио в прежнее состояние

playback-files = ФАЙЛЫ

playback-end-of-file = конец файла

playback-paused = пауза

playback-recorded-on-mode = Записано на { $mhz } МГц в режиме { $mode } — шкала и режим установлены по имени файла.

playback-recorded-on = Записано на { $mhz } МГц — шкала и режим установлены по имени файла.

playback-no-dial-mode = В имени нет ни частоты, ни режима, поэтому файл воспроизводится в текущем режиме — выберите декодер в меню режимов.

playback-format-stereo = { $rate } Гц, стерео — воспроизводится левый (приёмный) канал.

playback-format-mono = { $rate } Гц, моно — воспроизводится единственный канал.

playback-stopped = Остановлено: { $fault }

playback-intro = Воспроизведение записи в декодеры — SSTV, CW, RTTY, PSK, FT8 и любой другой режим, декодирующий звук. Пока идёт воспроизведение, радио отключено, шкала и режим переходят к тем, на которых сделана запись, а STOP возвращает радио. Меняйте режим во время воспроизведения, чтобы попробовать другой декодер.

packet-callsign-hint = позывной

packet-via-hint = via

aprs-filter-hint = фильтр

aprs-msg-sent = отправлено

aprs-msg-ack = подтв.

aprs-msg-rejected = отклонено

aprs-reply-chip = ответ

aprs-to-hint = кому

aprs-message-hint = сообщение

aprs-raw-chip = RAW

fsq-to-button = КОМУ

fsq-del-chip = УДЛ

macros-column-key = клавиша

macros-column-button = кнопка

macros-column-sends = передаёт

macros-label-hint = подпись

macros-add = ДОБАВИТЬ

macros-note-placeholders = { $mycall } и { $mygrid } подставляются при отправке сообщения.

macros-buttons-note = Каждая кнопка отправляет весь свой текст сразу. F1–F9 нажимают первые девять, пока клавиатура не занята чем-то на экране.



widgets-worked-before-any = Работали ранее

vdl2-col-sig = СИГН

vdl2-col-age = ВОЗР

vdl2-command = команда

vdl2-response = ответ

vdl2-frame-hint = кадр

vdl2-suffix-messages = { " " }сообщений

vdl2-suffix-stations = { " " }станций

ale-words-count = слов: { $n }

ale-waiting-audio = Ожидание звука…

ale-no-words-yet = Слов пока нет. Настройтесь на канал ALE (например, 8992 или 11175 кГц USB) и дождитесь вызова.

atchat-role-master = ГЛАВНЫЙ

atchat-role-backup = РЕЗЕРВ

atchat-me = я

dab-sub-channel = { " " }· подканал { $n }

dab-starting-receiver = запуск приёмника DAB…

dab-no-ensemble-yet = мультиплекс ещё не назван — есть ли на этом канале передача DAB?

dab-window-stats = { $mhz } МГц · кадров: { $frames } · FIB: { $fibs }

dab-volume-meter-hint = Станция в том виде, как вы её слышите, левый и правый каналы — и громкость: перетаскивайте или нажимайте вдоль полос, либо используйте колесо. Нажмите на него, затем меняйте шаг стрелками. Работает на экране, заблокированном замком.

rade-searching = поиск

rfpaint-swl-note = RF Paint — режим только для передачи, в SWL показывать нечего.

cw-hand-key-arm-hint = Включите манипулятор и используйте USB-педали. Контакты идут в движок, который формирует точки и тире со скоростью передачи — здесь, а не в панели, поэтому тайминг не округляется до кадра. Пока это включено, поле передачи заблокировано, а текст в очереди отбрасывается. Прямой ключ в том же поле тоже работает: выберите тип «Прямой ключ».

cw-own-keyer-hint =
    Это радио передаёт через собственный манипулятор: текст идёт по порту управления, а трансивер сам формирует элементы, поэтому между рукой и эфиром нет ничего, чем мог бы управлять ключ — ни педали, ни прямой ключ.

    Чтобы работать вручную, установите в Настройки → Радио манипуляцию CW «Звуковая карта (MCW)». Тогда трансивер удерживается на боковой полосе, а собственный тон самоконтроля манипулятора передаётся как звук — именно этим путём работает прямой ключ.

cw-no-paddle-found = педали не найдены — выберите в Настройки → CW

cw-msg-hint = Ваши собственные кнопки сообщений — обмен для соревнований, ответ с именем и QTH, TNX 73 GL. Каждая отправляет весь свой текст сразу, а F1–F9 нажимают первые девять. Они хранятся в конфигурации станции, поэтому есть и у удалённого клиента.

wspr-clear-rx = ОЧИСТИТЬ RX

wspr-rx-count = rx: { $n }

wspr-sort-dist = РАССТ

wspr-sort-country = СТРАНА

wspr-tx-enable = TX ВКЛ

wspr-stop-tx = СТОП TX

wspr-listening-as = Приём как { $call } в { $sent } — задаётся на вкладке «Общие» в Настройках.

wspr-set-callsign-grid-label-spots = Укажите позывной и локатор на вкладке «Общие» в Настройках, чтобы подписывать ваши споты.

schedule-source-mine = мои

schedule-all-of-them = все

schedule-title = ПРОГРАММЫ

schedule-at = На

schedule-now-chip = СЕЙЧАС

schedule-set-time = задать время

schedule-find-hint = BBC, 6185, 15.4 МГц…

schedule-target = Цель

schedule-any = любая

schedule-source = Источник

schedule-all-hint = Каждое загруженное расписание в едином хранилище. Станция, указанная в двух из них, появляется дважды — ради этой детали окно и существует; панорама показывает одно имя.

schedule-favs = ★ ИЗБР

schedule-favs-hint = Только станции, отмеченные звёздочкой

schedule-all-favs = ⭐ ВСЕ

schedule-all-favs-hint = Добавить в избранное все станции расписания и показывать их подписи на водопаде, пока включено. Нажмите ещё раз, чтобы очистить всё избранное.

schedule-solar-time = МЕСТНОЕ ВРЕМЯ

schedule-solar-time-hint = Показывать местное время передатчика для каждой строки. Это солнечное время — четыре минуты на градус долготы, — а не гражданский часовой пояс, поэтому оно ничего не знает о переходе на летнее время и границах поясов.

schedule-map = ПОКАЗАТЬ КАРТУ

schedule-map-hint = Открыть LISTEN — это расписание в виде карты. Выберите на ней станцию, напишите, что услышали, и занесите в журнал. Выбранная станция переносится

schedule-on-air = в эфире: { $count } в { $time } UTC

schedule-now = сейчас

schedule-solar-hover = { $local } местное солнечное время в { $site }, { $lon }°

schedule-favourite-hint = Добавить эту станцию в избранное

rc-radio-classic = RADIO CLASSIC

rc-bands = Диапазоны

rc-mem-programs = Пам · Программы

rc-recall-band-mode-frequency-and = Вызов: диапазон, вид излучения, частота и фильтр сразу

rc-no-memories-yet-store-one-with-mem = ячеек памяти пока нет — сохраните одну кнопкой MEM в верхней панели

rc-broadcast-schedule-what-is-on-when = Расписание вещания — что, когда и где в эфире

rc-bw-this-mode-has-one-fixed = BW — у этого режима одна фиксированная полоса

rc-bw-click-next-filter-right-click = BW — щелчок: следующий фильтр · правый щелчок: предыдущий · колесо: шаг

rc-step = ШАГ

rc-step-click-next-tuning-step-right = STEP — щелчок: следующий шаг настройки · правый щелчок: предыдущий · колесо: шаг

rc-agc-click-next-speed-right-click = AGC — щелчок: следующая скорость · правый щелчок: предыдущая · колесо: шаг

rc-open = ОТКР

rc-sql-click-open-back-to-the-last = SQL — щелчок: открыт / возврат к прежнему порогу · колесо: поднять или опустить порог

rc-s-meter = S-МЕТР

rc-wheel-on-a-digit-click-upper-lower = колесо на цифре · щелчок по верхней/нижней половине ± · правый щелчок — ноль · двойной щелчок — ввод с клавиатуры

rc-music = МУЗЫКА ♪

rc-speech = РЕЧЬ

rc-stereo-the-19-khz-pilot-is-locked = Стерео — пилот-тон 19 кГц захвачен — нажмите для моно

rc-mono-the-broadcast-stereo-decode = Моно — стереодекодирование вещания выключено — нажмите, чтобы включить

rc-mono-noise-reduction-is-on-and = Моно — включено шумоподавление, и стерео уступает ему

rc-mono-no-19-khz-pilot-the-station = Моно — нет пилот-тона 19 кГц: станция передаёт моно или сигнал слишком слаб

rc-stereo = СТЕРЕО

rc-group-broadcast = ВЕЩАНИЕ

rc-group-ham = ЛЮБИТЕЛЬСКИЕ

rc-group-air = АВИАЦИЯ

rc-group-marine = МОРСКИЕ

rc-cap-lw-mw-fm = ДВ / СВ · FM

rc-cap-shortwave = КВ · КОРОТКИЕ ВОЛНЫ

rc-cap-amateur-bands = ЛЮБИТЕЛЬСКИЕ ДИАПАЗОНЫ

rc-cap-airband = АВИАДИАПАЗОН

rc-cap-marine = МОРСКОЙ УКВ · СВ

rc-cap-cb = ГРАЖДАНСКИЙ ДИАПАЗОН

rc-preset-fm-ext = FM расш. 64–87.5

rc-mem-n = ПАМ { $id }

controls-action-tune = Настройка VFO

controls-action-rit-offset = Расстройка RIT

controls-action-xit-offset = Расстройка XIT

controls-action-volume = Громкость

controls-action-sub-volume = Громкость доп. приёмника

controls-action-squelch = Шумоподавитель

controls-action-filter-width = Ширина фильтра

controls-action-filter-shift = Сдвиг фильтра

controls-action-agc-max-gain = Макс. усиление АРУ

controls-action-manual-gain = Ручное усиление (АРУ выкл.)

controls-action-tx-drive = Возбуждение TX

controls-action-tune-drive = Возбуждение TUNE

controls-action-mic-gain = Усиление микрофона

controls-action-digi-audio-freq = Смещение цифрового звука

controls-action-spectrum-zoom = Масштаб спектра

controls-action-spectrum-pan = Сдвиг спектра

controls-action-spectrum-floor = Нижний уровень спектра

controls-action-spectrum-ceiling = Верхний уровень спектра

controls-action-ptt = PTT

controls-action-tune-carrier = Несущая TUNE

controls-action-mute = Заглушить

controls-action-noise-blanker = Шумоподавитель импульсов

controls-action-nr-cycle = Шумоподавление (по кругу)

controls-action-auto-notch = Автоматический режекторный фильтр

controls-action-binaural = Бинауральный звук

controls-action-agc-cycle = АРУ (по кругу)

controls-action-sub-rx = Дополнительный приёмник

controls-action-split = Split

controls-action-rit-enable = RIT вкл/выкл

controls-action-xit-enable = XIT вкл/выкл

controls-action-rit-clear = Сброс RIT

controls-action-xit-clear = Сброс XIT

controls-action-vfo-toggle = Переключение VFO A/B

controls-action-swap-vfos = Обмен A↔B

controls-action-copy-a-to-b = Копировать A→B

controls-action-band-up = Диапазон выше

controls-action-band-down = Диапазон ниже

controls-action-mode-next = Следующий вид излучения

controls-action-mode-prev = Предыдущий вид излучения

controls-action-record-toggle = Запись вкл/выкл

controls-action-abort-tx = Прервать передачу

controls-action-cw-straight = Прямой ключ CW

controls-action-voice-stop = Остановка голосового манипулятора

controls-action-tone-burst = Тональный вызов 1750 Гц

controls-action-fit-span = Вписать полосу обзора

controls-action-zoom-in = Приблизить

controls-action-zoom-out = Отдалить

controls-action-peak-hold = Удержание пиков

controls-action-spectrum-collapse = Свернуть спектр

controls-action-waterfall-collapse = Свернуть водопад

controls-action-waterfall-flip = Перевернуть водопад

controls-action-toggle-help = Окно руководства

controls-action-toggle-settings = Окно настроек

controls-action-toggle-logbook = Окно журнала

controls-action-toggle-spots = Окно спотов

controls-action-toggle-memories = Окно памяти

controls-action-toggle-voice = Окно голосового манипулятора

controls-action-workspaces = Окно рабочих пространств

controls-action-speak-status = Озвучить состояние

controls-action-speak-repeat = Повторить последнее оповещение

controls-action-speech-silence = Прекратить озвучивание

controls-action-speech-toggle = Оповещения вкл/выкл

controls-action-broadcast-labels = Подписи вещательных станций (настроенная / весь диапазон)

controls-action-select-vfo = Выбрать VFO { $vfo }

controls-action-band = Диапазон { $band }

controls-action-mode = Вид излучения { $mode }

controls-action-recall-memory = Вызвать память { $n }

controls-action-voice-slot = Ячейка голосового манипулятора { $n }

controls-group-tuning = Настройка

controls-group-receive = Приём

controls-group-transmit = Передача

controls-group-display = Отображение

controls-group-windows = Окна

controls-group-speech = Речь

controls-midi-note = Нота

controls-midi-cc = CC

controls-midi-pitch-bend = Pitch bend

controls-midi-program = Программа

controls-midi-any = любой

controls-midi-msg = { $kind } { $data }  кан { $ch }

controls-midi-msg-pitch-bend = Pitch bend  кан { $ch }

settings-controls-down = вниз

settings-controls-rev = обр

antenna-antenna = АНТЕННА

antenna-cutting-lengths = Длины для резки

antenna-the-frequency-to-cut-for = Частота, под которую резать

antenna-optimizing = Оптимизация…

antenna-tune-the-yagi-s-element-lengths = Подбор длин элементов и расстояний антенны Уда–Яги под выбранную цель. Локальный спуск по градиенту в фоновом потоке; окно остаётся доступным, пока идёт расчёт.

antenna-solved = Решено

antenna-optimized-yagi = Оптимизированная антенна Уда–Яги

antenna-obstacles-approximate = Препятствия (приблизительно)

antenna-trees = Деревья

antenna-in-leaf = с листвой

antenna-ridge = Хребет

dab-channel-received-mhz = Принят канал { $channel } { $mhz }

dab-no-channel-received = Канал не принят

dab-channel-received = Принят канал { $channel }

dab-slideshow = СЛАЙД-ШОУ

dab-no-picture-yet = станция ещё не передала картинку

dab-no-station-chosen = станция не выбрана

dab-no-logo-yet = логотипа пока нет

dab-playing = ▶ воспроизводится

dab-chosen = выбрано

dab-no-text-yet = станция ещё не передала текст

dab-clear-channel-hint = Удалить этот канал и его станции. SCAN найдёт его снова.

dab-clear-all-channels = × удалить все каналы

dab-clear-all-channels-hint = Забыть все каналы и станции на них. SCAN найдёт их снова; звёздочки сохранятся.

dab-n-stations = станций: { $n }

dab-scanning-progress = сканирование { $channel } · { $at }/{ $n } · найдено: { $found }

dab-clear-station-hint = Удалить эту станцию из списка. Следующий SCAN вернёт её, если мультиплекс всё ещё её содержит.

dab-prev = ПРЕД

dab-prev-hint = Предыдущая станция в списке.

dab-preset = ПРЕСЕТ

dab-preset-hint = Отметить эту станцию звёздочкой или снять звёздочку.

dab-next = СЛЕД

dab-next-hint = Следующая станция в списке.

dab-pty-hint = Показывать тип программы каждой станции в списке.

dab-stop-sweep-hint = Остановить проход здесь. Уже найденное остаётся.

dab-scan-hint = Пройти по каждому блоку Band III и оставить те, на которых есть станции.

swl-title = ЖУРНАЛ SWL

swl-new = + НОВАЯ

swl-jobs = ЗАДАНИЯ

swl-jobs-hint = Запланированные записи — записать диапазон в заданное время

swl-sigid = ОПОЗНАНИЕ

swl-sigid-hint = Что на этой частоте? Справочник сигналов по частоте, виду излучения и полосе

swl-replay = ПОВТОР

swl-replay-hint = Воспроизвести последние две минуты вместо прямого эфира — услышать позывной станции, который вы только что пропустили

swl-report = РАПОРТ

swl-report-hint = Скопировать этот приём как рапорт для отправки станции

swl-stats-heard = услышано: { $n }

swl-stats-reported = отправлено рапортов: { $n }

swl-stats-qsl = QSL: { $n }

swl-stats-pirates = пиратов: { $n }

swl-stats-hint = Весь журнал — фильтры «Показать» его не меняют

swl-antenna = Антенна

swl-antenna-hint = Длинный провод 20 м, рамка MLA-30, мини-вип …

swl-antenna-tip = Попадает в строку Antenna: рапорта о приёме

swl-tone = Тембр

swl-tone-on = вкл

swl-tone-bass = НЧ

swl-tone-mid = СЧ

swl-tone-treble = ВЧ

swl-report-copied = Рапорт скопирован в буфер обмена — вставьте его в вашу почтовую программу. Не удалось открыть почтовый клиент.

swl-show = Показать

swl-find-hint = станция, язык, площадка …

swl-find-tip = Совпадает со станцией, языком, площадкой передатчика и заметками

swl-all-bands = Все диапазоны

swl-all-days = Все дни

swl-pirates-only = Только пираты

swl-pirates-only-tip = Только нелицензированные станции

swl-adif-tip = Сохранить весь журнал в ADIF — записи о приёме, а не о связях

swl-csv-tip = Сохранить весь журнал в CSV, одна строка на приём

swl-shown-of = { $shown } из { $n }

swl-empty = Пока ничего не записано — настройтесь на станцию и нажмите + НОВАЯ.

swl-transmitted-from = передаётся из { $site }

swl-click-to-copy = (нажмите, чтобы скопировать)

swl-addr-btn = адрес

swl-mail-btn = почта

swl-mark-sent = отпр.

swl-recall = вызв

swl-recall-tip = Вернуться к этой частоте и виду излучения — проверить, не вернулась ли станция

swl-edit = изм.

swl-del = удл.

swl-no-match = Ни один приём не подходит под фильтр — ОЧИСТИТЬ покажет все.

swl-form-new = НОВЫЙ ПРИЁМ

swl-form-edit = ПРАВКА ПРИЁМА

swl-station-hint = Radio Taiwan International

swl-language-hint = английский

swl-received = Принято

swl-received-hint = ваш локатор

swl-email = E-mail

swl-email-hint = рапорты о приёме

swl-email-tip = Попадает в строку Send to: рапорта

swl-postal = Почтовый адрес

swl-postal-hint = улица, город, страна

swl-postal-tip = Запасной адрес для рапорта, если нет e-mail

swl-heard = Слышно

swl-smeter = S-метр

swl-report-label = Рапорт

swl-judged = оценено

swl-report-sent = Рапорт отправлен

swl-not-yet = ещё нет

swl-qsl-received = QSL получена

swl-awaiting = ожидается

swl-notes = Заметки

swl-notes-hint = заметки о программе

swl-pirate = Пират

swl-pirate-tip = Нелицензированное вещание — станция, передающая вне каких-либо присвоений. В журнале помечается пиратским флагом.

swl-mail-why-no-address =
    У этой станции нет адреса электронной почты — найдите его в окне РАСПИСАНИЕ или введите здесь, и кнопка станет доступна.

    Рапорт о приёме — это любезность, которую станция читает и отвечает на неё карточкой QSL; без адреса отправлять его некуда.

swl-mail-why-bad-address = Этот адрес электронной почты не является одним адресом, поэтому не используется — рапорт, доставленный постороннему, который станцию не слышал, хуже, чем никакого.

swl-mail-why-ok =
    Составить рапорт о приёме и открыть его в вашем почтовом клиенте.

    Картинку и звуковой фрагмент вы прикрепляете сами — эта программа не может вложить файл в сообщение.

swl-mail-report = ОТПРАВИТЬ РАПОРТ…

antenna-feed-ohms = вход ≈ { $r } Ом

antenna-takeoff = угол излучения { $deg }°

antenna-director = директор { $n }

antenna-reflector = рефлектор

antenna-driven = вибратор

antenna-gap = зазор { $v } м

antenna-az = азим

antenna-depth = глубина

antenna-height = высота

antenna-attenuation = ослабление на азимуте 0°: { $att } дБ

morse-no-paddle-found-connect-one-then = педали не найдены — подключите, затем снова откройте эту панель

morse-send-with-a-paddle-played-here = Передача педалями — звучит только здесь, в эфир не уходит

morse-no-paddle-found-in-dev-input-by-id = Педали не найдены в /dev/input/by-id. Подключите их и снова откройте эту панель. Подойдёт блок манипулятора, сообщающий два контакта (часто как кнопки мыши); тот, что сам выполняет итеративную манипуляцию и посылает касания, не подойдёт.

morse-reverse = ОБРАТИТЬ

morse-swap-dit-and-dah-the-switch-on = Поменять точку и тире. Переключатель на многих педалях делает это на самом ключе; это на случай, когда он в другом положении.

morse-stop-key = СТОП

morse-start-key = СТАРТ

morse-key-the-character-above-the-keyer = Передать символ выше. Манипулятор формирует чёткие точки и тире — вы выбираете, какой педалью и с какой паузой. Касание — один элемент; удерживайте для повтора.

morse-the-sending-drill-needs-a-linux = Для тренировки передачи нужны рабочий стол Linux и педали.

panels-float-chip-hint = FLOAT — вынести эту панель в плавающую и вернуть место водопаду; метка закрепления на ней возвращает панель обратно

panels-map-style-radar = Радар — тёмный фон, суша залита. На чём читается воздушная обстановка

panels-map-style-terrain = Рельеф — суша приподнята и подкрашена тёплым, чтобы читаться как земля, границы приглушены

panels-map-style-chart = Карта — светлая суша, выделенные границы и градусная сетка, авиационный вид

panels-heard-me-chip = МЕНЯ СЛЫШАЛИ

panels-heard-me-hint = Обвести каждую станцию, сообщившую о приёме этого позывного на текущем диапазоне, по данным PSK Reporter, на локаторе самого сообщившего. Обратное PSK-спотам: те — станции, которые слышит это радио. Обновляется каждый час, охватывая последний час; нужна включённая лента PSK в Настройки → Споты и позывной на вкладке «Общие».

panels-period-label = период

panels-sub-mode-label = подрежим

panels-front-end-overloading = ⚠ ПЕРЕГРУЗКА ВХОДА — уменьшите усиление РЧ (LNA/ПЧ) или переместите антенну

panels-front-end-overloading-hint = Приёмник перегружен выше полной шкалы, поэтому звук громкий, но сигнала в нём нет — пока это длится, ничего не декодируется. Уменьшите усиление РЧ/LNA (больше затухания), снизьте усиление ПЧ или включите АРУ. Обычная причина — мощный близкий передатчик или другой трансивер на том же диапазоне.

panels-enigma-hint = Машина Enigma — зашифруйте сообщение на военной лицевой панели и расшифруйте шифртекст, скопированный из эфира

panels-slot-bar-heard-hint = Следует за звуком в колонках: белый, пока слышен кадр, зелёный, когда он закончился, и возврат к началу, когда начинается следующий. Времена — для режима, поэтому короткий кадр всё равно заполняет полосу.


morse-send-correct = верно: { $want }

morse-send-heard = услышано { $got }, нужно { $want }

morse-send-received = принято: { $text }

amp-title-model = УСИЛ · { $model }

amp-state-off = ВЫКЛ

amp-state-on = ВКЛ

amp-relay-dn = ВНИЗ

amp-relay-up = ВВЕРХ

amp-band-unknown = НЕИЗВ

amp-needle-top = ВЕРХ

decodes-not-uploaded = ⚠ { $services } не загружено

decodes-not-uploaded-hint = Связь есть в локальном журнале, но загрузка не удалась. Проверьте токен и соединение в Настройки → Споты, затем загрузите её снова из журнала.

decodes-logged-to = ↑ записано в { $services }

decodes-logged-to-hint = Также загружено в онлайн-журналы, настроенные принимать каждую новую связь.

decodes-known = ИЗВЕСТНЫЕ

decodes-known-hint = Все позывные, которые хэш <...> здесь может сейчас разрешить, со страной. Хэш нельзя прочитать в обратную сторону — станция появляется, только когда слышно, как она называет свой позывной, для чего и служит простой вызов на 11 м.

decodes-ignored-clear = игнорируется: { $n } · сбросить

decodes-auto-tx-frequency-hint =
    Выбирать частоту передачи автоматически: самое тихое место в периоде, в котором мы передаём, а не частоту той станции, которой отвечаем — она передаёт в другом периоде, поэтому её частота ничего не говорит о том, кто есть в эфире, когда мы включаем передачу. Выключено НЕ удерживает частоту: ответ идёт на частоте вызываемой станции. Чтобы удерживать, используйте Hold TX.

    Один диапазон — исключение: на 11 м норма WSJT-CB — отвечать на CQ на собственном тоне вызывающего и оставаться на нём всю связь, поэтому поиск пропускается, и ответ на CQ остаётся на частоте, на которой он был услышан, независимо от того, включено это или нет.

decodes-muted-caps = ЗАГЛУШЕНО

decodes-muted-hint = Заглушено на этот сеанс — нажмите, чтобы снова слышать

decodes-ignore-hint = Игнорировать эту станцию до конца сеанса: она исчезает из списка декодирований, перестаёт вызывать оповещения и никогда не предлагается автоматическому режиму. От её имени ничего не отправляется и не загружается, а перезапуск sdroxide возвращает её.

decodes-qso-complete-hint = Связь завершена и находится в локальном журнале. Она удерживается здесь несколько минут, чтобы итоговое сообщение можно было переслать, если другая станция повторит своё — больше ничего не требуется.

decodes-slot-even = чёт

decodes-slot-odd = нечёт

decodes-before = до

decodes-after = после

decodes-beam-hint = Направить антенну на локатор этой станции

decodes-queue-next = далее

decodes-queue-waiting = ожидает


decodes-setup-hint = НАСТРОЙКА — идентификация станции и шаблоны, общие для этих режимов QSO

decodes-auto-armed-hint =
    { $role } включён. Нажмите, чтобы остановить.

    { $note }

    Шаг движка: { $step } — станция, отвечающая на CQ, принимается только в CallingCq при отсутствии текущей связи.

decodes-auto-rx-hint = Отвечать на CQ новой станции без присмотра и никогда не вызывать CQ самому. Отвечает только позывным, которых ещё нет в журнале. Темп задаёт сторожевой таймер передачи, а остановка по бездействию отключает режим.

decodes-auto-continuous-hint = Весь цикл без присмотра: завершить QSO, работать с теми, кто вызывает вас, отвечать станции, вызывающей CQ, и вызывать CQ, только когда больше нечего делать. Работает, пока вы не остановите{ $until }. { $bound }

decodes-auto-or-minutes-up = { " " }или не истекут { $n } мин

decodes-auto-cq-run-unlimited = У каждого цикла CQ нет собственного предела вызовов.

decodes-auto-cq-run-bounded = Каждый цикл CQ ограничен собственным числом вызовов.

decodes-auto-no-call-limit = Собственного предела вызовов нет — цикл завершают сторожевой таймер и остановка по бездействию.

decodes-auto-after-calls-listen = После { $repeats } вызовов переключается на АВТО RX и продолжает слушать.

decodes-auto-after-calls-stop = После { $repeats } вызовов останавливается.

decodes-auto-cq-hint = Вызывать CQ без присмотра и работать с теми, кто отвечает. { $bound } Слишком короткий цикл — и тихий диапазон вас не услышит; слишком длинный — и передатчик сидит на частоте, на которой никто не отвечает.

decodes-auto-stopped-by-operator = { $role } остановлен оператором

decodes-cont = прод

settings-servers-decoded-text-udp = Декодированный текст по UDP

settings-servers-decoded-text-udp-note = Одна строка JSON на каждое декодирование, для вашего собственного скрипта: семейство FT8, WSPR и декодированные строки всех клавиатурных режимов (CW, RTTY, PSK, Olivia, THOR, MFSK, DominoEX, Hell), каждая с шкалой, на которой была услышана. Вещание WSJT-X выше — протокол журнала, и для этих режимов в нём нет сообщения, поэтому это и существует отдельно.

settings-servers-decoded-text-udp-hint = host:port, по UDP. Пусто — выключено, по умолчанию. Только вывод: ничто на этом сокете не может управлять радио. Имя разрешается при нажатии ПРИМЕНИТЬ.

settings-servers-rotator-note = Наводит моторизованную антенну на спутник, на который вы захвачены, или вручную из окна ROTATOR и карт. По умолчанию используется демон Hamlib rotctld (запустите, например, «rotctld -m 603 -r /dev/ttyUSB0» для поворотного устройства GS-232B или «rotctld -m 1», чтобы попробовать без оборудования) — один демон охватывает все поворотные устройства, которыми управляет Hamlib. Контроллером на последовательном порту также можно управлять напрямую.

settings-servers-connection = Подключение

settings-servers-rotctld-port-hint = 4533 — значение rotctld по умолчанию

settings-servers-baud-hint = Большинство контроллеров работают на 9600

settings-servers-pstrotator-note = Собственный UDP-порт PSTRotator. В его Setup включите UDP Control; он отвечает на команды на этом порту и отправляет ответы на следующий.

settings-servers-el = элев

settings-servers-persist-re-connect-rotator = Сохранить и (пере)подключиться к поворотному устройству

settings-servers-tci-note = Позволяет TCI-совместимым программам управлять этим радио: частота и вид излучения, широкополосный IQ, звук приёма и звук передачи. В WSJT-X выберите тип трансивера SunSDR (TCI), укажите этот адрес и задайте PTT через TCI.

settings-servers-decoded-text-udp-format = Каждая строка — одна датаграмма, оканчивающаяся переводом строки: { $example }

settings-servers-transport-rotctld = Hamlib rotctld (TCP)

settings-servers-transport-easycomm = EasyComm II (последовательный)

settings-servers-transport-gs232 = Yaesu GS-232 (последовательный)

settings-servers-transport-pstrotator = PSTRotator (UDP)

spots-active = АКТИВНЫЕ

spots-closing = ЗАКРЫВАЮТСЯ

spots-none-left-in-the-window = в окне не осталось

spots-openings = ОТКРЫТИЯ

spots-band-opening-detections-paths = Обнаружения открытия диапазона: трассы, недавняя активность которых превысила их собственный 3-часовой базовый уровень

spots-save-the-spots-you-can-see-as-a = Сохранить видимые споты в файл KML — для Google Earth и любой другой картографической программы, читающей его. Только включённые фильтры и только споты с положением, потому что метка без координат — это метка на другом конце света.

spots-no-spots-to-save-the-filters-are = Нет спотов для сохранения — все фильтры выключены

spots-bc-all = BC ВСЕ

spots-bc-tuned = BC НАСТР

spots-which-broadcasters-the-waterfall =
    Каких вещателей называет **водопад**.

    { "**" }BC НАСТР** — значение по умолчанию и правило bebepip: имя у станции, на которую вы настроены, и ничего больше — диапазон, подписанный от края до края, превращается в стену имён поверх полосы, которую вы пытаетесь читать.

    { "**" }BC ВСЕ** называет каждую станцию, которая сейчас в эфире по расписанию, где бы она ни находилась, — это нужно, когда вы просматриваете диапазон, а не слушаете одну станцию. Это ничего не говорит о том, показывает ли вообще метка BC выше вещателей.

app-rx-only-hint = Это радио только принимает — у него нет передатчика, которым можно управлять.

spots-n-calls = вызовов: { $n }

spots-opening-hover = Трасса диапазона { $band } от { $from } до { $to }: последнее 15-минутное окно превысило 3-часовой базовый уровень трассы — недавние вызывавшие: { $calls }


spots-download-unsupported = { $source } не удалось загрузить на этой платформе

spots-kml-partial = сохранено { $placed } из { $total } — остальные не имеют положения и не могут быть помечены

spots-net-log = ЖУРНАЛ СЕТИ ({ $n })

decodes-beam = ЛУЧ

settings-radio-no-answer-yet = Ожидание компьютера, к которому подключено радио: эти запросы касаются его оборудования и сети, а отвечает он по одному.

settings-radio-elad-no-baud = у радио нет настройки { $baud } бод — будет использовано { $default }

settings-radio-elad-antenna-rx-only = Только RX

settings-radio-rx-only-n = RX{ $n } только

settings-radio-ng-level = Уровень nG

settings-radio-ng-level-hint =
    nG добавляет две команды, которых нет в прошивке 2.00x, и задать их можно только здесь: nG не отвечает на них и не хранит, поэтому sdroxide отправляет их заново при каждом открытии связи.

    Громкость — это уровень, который радио отправляет по USB-потоку. nG берёт этот звук после VOL и AGC, поэтому именно здесь нужно исправлять декодер, который видит слишком мало или слишком много сигнала, — а не в микшере компьютера.

    Регулятор усиления — это собственная настройка АРУ радио. Авто ставит DIGI для любого цифрового режима и «вкл» в остальных, чего и требуют заметки nG для FT8: тогда усиление держится в паузах передачи другой станции, а не поднимается в них.

settings-radio-ng-volume-hint = 0–31, как `AG0nn;`. Уровень, который радио передаёт на компьютер.

settings-radio-gain-control = Регулировка усиления

settings-radio-speaker = Динамик

settings-radio-on-while-streaming = Включён при потоке

settings-radio-ng-speaker-hint = «Выкл» отключает собственный динамик радио на время потока (`UA2;`), так что звук слышен только в sdroxide. В 2.00x такой формы не было.

settings-radio-ng-agc-auto = Авто (DIGI в цифровых режимах)

settings-radio-ng-agc-off = Выкл

settings-radio-ng-agc-on = Вкл (быстрая)

settings-radio-sound-card-note = Какая звуковая карта несёт звук этого радио, указывается в разделе **Звук**, вместе с вашими колонками и микрофоном — это одна и та же пара настроек для любого радио, с которым вы работаете, поэтому страница одна.

settings-radio-no-control-cable-audio-page = У этого радио нет кабеля управления — оно включается само по VOX, поэтому команды PTT не нужны. Передача — это звук на микрофон радио, поэтому звук передачи радио находится на странице «Звук», а не здесь.

settings-radio-sound-card-note-short = Какая звуковая карта несёт звук, указывается в разделе **Звук**.

settings-radio-control-link = Канал управления

settings-radio-atsmini-host-hint =
    Ad hoc TCP-сервер приёмника: `atsmini.local` (mDNS) или его IP. На радио: Settings → TCP Port → Ad hoc, при Wi-Fi в режиме Connect, AP+Connect или AP Only.

    `atsmini.local` требует mDNS-резолвера (Avahi/nss-mdns) на этом компьютере. Если канал управления не поднимается, IP показан на экране Wi-Fi радио — введите его вместо имени.

settings-radio-atsmini-find-hint = Просканировать сеть этого компьютера в поисках порта управления приёмника

settings-radio-tcp-port = TCP-порт

settings-radio-tcp-port-hint = По умолчанию в прошивке 60000.

settings-radio-atsmini-receive-only = Только приём: Si4732 демодулирует аппаратно, поэтому выбранный здесь режим передаётся на радио командой, а всё остальное — детектирование, декодирование, журнал — выполняется на этом компьютере.

settings-radio-atsmini-tuning-note =
    Настройка следует собственному циклу диапазонов приёмника, поэтому шкала немного отстаёт от вашей прокрутки — сильнее всего, когда частота переходит в другой диапазон. Дайте ей установиться перед дальнейшей прокруткой, и она всегда придёт на запрошенную частоту.

    Шаг настройки приёмника у него свой (регулятор STEP выше): изменение мельче этого шага округляется на радио, и шкала возвращается к шагу, на который фактически попала.

settings-radio-live-controls = Оперативное управление

settings-radio-atsmini-vol = VOL

settings-radio-atsmini-step = STEP

settings-radio-atsmini-sent-at-once = Отправляется на радио сразу. VOL — собственная громкость приёмника (его ручка и Si4732), а не ползунок НЧ в sdroxide.

settings-radio-refresh = Обновить

settings-radio-read-99-slots = Прочитать 99 ячеек из приёмника

settings-radio-tune-radio-slot = Настроить это радио на ячейку

settings-radio-no-slots-stored = В приёмнике нет сохранённых ячеек.

settings-radio-press-refresh-slots = Нажмите «Обновить», чтобы прочитать ячейки памяти приёмника.

settings-radio-icomnet-receive-hint = AF: радио демодулирует, а sdroxide показывает звуковую полосу. 12 kHz IF: радио вместо этого отправляет свою промежуточную частоту DRM, а демодулирует sdroxide, что даёт свои фильтры, шумоподавление и декодеры на полосе около ±12 кГц. В любом случае широкий водопад — собственный обзор радио: ни один Icom не выдаёт I/Q.

settings-radio-copy-icom-session-hint = Копирует последний сеанс этого радио — рукопожатие и трассу CI-V — в буфер обмена (или в файл, в браузере, который не разрешает странице копировать) для отчёта об ошибке. Именно этого радио, а не станции: при двух Icom в локальной сети каждая вкладка отвечает за свой адрес.

settings-radio-copy-protocol-trace-hint = Копирует трассу протокола последнего сеанса в буфер обмена (или в файл, в браузере, который не разрешает странице копировать) для отчёта об ошибке.

settings-radio-copy-trace-device-hint = Копирует трассу последнего сеанса в буфер обмена (или в файл, в браузере, который не разрешает странице копировать) для отчёта об ошибке: каждая команда, обмененная с устройством, и первые байты потока отсчётов.

settings-radio-copy-trace-receiver-hint = Копирует трассу последнего сеанса в буфер обмена (или в файл, в браузере, который не разрешает странице копировать) для отчёта об ошибке: каждая команда, обмененная с приёмником, и первые байты потока отсчётов.

settings-radio-pluto-choose-receiver = Выберите ваш приёмник

settings-radio-dab-rate-hint = Выберите 2.500 Msps, если хотите декодировать DAB/DAB+

settings-radio-rx888-dab-rate-hint = Выберите 80.0 Msps и ширину 2.500 МГц (1/32), если хотите декодировать DAB/DAB+

settings-radio-pluto-both-chains-hint = Чтобы слушать на обеих антеннах: добавьте второе радио с тем же адресом и установите его в RX2. Оба радио используют одну частоту.

settings-radio-duplex-tdd-hint =
    Работает ли AD9361 в обоих направлениях одновременно (FDD — так Pluto загружается, и так sdroxide его всегда оставлял) или по очереди (TDD).

    Оставьте FDD, если вам не нужны выводы PTT ниже — TDD включает их, и он исключает «Полный дуплекс» выше. Вступает в силу после «Применить».

    PlutoSDR: полный дуплекс выключен — TDD включает одно направление за раз, и именно от этого управляются выводы GPO приём-передача. Установите выводы PTT в «Выкл», а дуплекс в FDD, чтобы слушать во время передачи.

settings-radio-fm-mw-notch = Режекция FM/СВ

settings-radio-fm-mw-notch-hint = Аппаратный режекторный фильтр на FM-вещание (88–108 МГц) и средневолновый диапазон, на случай, когда местный передатчик перегружает всё остальное. Применяется сразу.

scanner-broadcast-band = Вещательный диапазон

scanner-all = ВСЕ

scanner-stop-only-on-channels-that-sound = Останавливаться только на каналах, где звучит чья-то речь: голос периодичен, заполняет речевую полосу и поднимается и спадает несколько раз в секунду, чего не делают несущая, «птичка» и пакет данных. Каждый кандидат прослушивается не меньше полусекунды, потому что именно столько нужно, чтобы услышать слог.

scanner-nothing-in-this-scan-is-in-a-mode = В этом сканировании нет ничего в режиме, несущем речь, — поиск голоса здесь шёл бы и нигде не останавливался, поэтому он не предлагается, пока такого нет. Голос несут FM, AM и боковые полосы; цифровые режимы — нет.

scanner-voice = ГОЛОС

scanner-stop-only-where-somebody-is = останавливаться, только где кто-то говорит

scanner-no-voice-to-look-for-in-these = в этих режимах голоса искать нечего

scanner-sql = SQL

scanner-how-long-to-stay-on-a-candidate = Как долго оставаться на кандидате, прежде чем оценить его. Для проверки голоса нужно не меньше этого: голос — это ритм слогов, и полсекунды — это два из них

scanner-listening-for-a-voice = поиск голоса…

scanner-scan-band-hover = Сканировать { $name } ({ $lo }-{ $hi } кГц)

scanner-range-to = до

scanner-mode-no-speech = { $mode } не несёт речи — эти каналы пропускаются

scanner-voice-holding = голос { $score } · кто-то говорит

scanner-voice-passed = пропущено { $mhz } МГц · голос { $score }

aircraft-hfdl-aircraft = Самолёт HFDL { $id }

aircraft-no-icao = нет адреса ICAO (HFDL)

aircraft-provenance = { $source }, { $age } назад

aircraft-label-flight = Рейс

aircraft-label-registration = Регистрация

aircraft-label-position = Положение

aircraft-label-altitude = Высота

aircraft-label-speed = Скорость

aircraft-label-track = Курс

aircraft-label-last-message = Последнее сообщение

aircraft-label-type = Тип

aircraft-label-operator = Оператор

aircraft-label-origin = Откуда

aircraft-label-destination = Куда

aircraft-label-online = Онлайн

aircraft-missing-flight = ещё не слышно — его передают ADS-B или VDL2

aircraft-missing-registration = ещё не слышно — его передаёт только VDL2

aircraft-missing-position = ещё не слышно — ADS-B либо отчёт о положении по VDL2 или HFDL

aircraft-missing-adsb = ещё не слышно — его передаёт ADS-B

aircraft-missing-message = пока нет — VDL2 либо звук ACARS, когда VDL2 назвал самолёт

aircraft-online-unavailable-disabled = онлайн недоступен — переключатель «Онлайн» выключен

aircraft-online-unavailable-no-station = онлайн недоступен — некого спросить; есть только радиоданные

aircraft-online-unavailable-unauthorized = онлайн недоступен — вход на станцию не выполнен

aircraft-online-unavailable-retry = онлайн-запрос не удался — скоро будет повторная попытка

aircraft-switch-no-answer = станция не ответила

aircraft-switch-not-signed-in = вход на станцию не выполнен

aircraft-switch-refused = станция отклонила запрос

aircraft-switch-too-old = станция не предоставляет этот переключатель (слишком старая?)

aircraft-switch-failed = станция не смогла это изменить

aircraft-online-source = adsbdb, через станцию

aircraft-asking-station = запрос к станции

aircraft-provenance-radio = радио

aircraft-provenance-radio-online = радио + онлайн

aircraft-provenance-online = онлайн

aircraft-photo-credit = Фото: { $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = Поворотное устройство антенны не настроено.

rotor-set-one-up-in-settings-servers-a = Настройте его в Настройки ▸ Серверы — демон Hamlib rotctld или контроллер на последовательном порту. Затем наводите луч с этой шкалы, по позывному или с карты.

rotor-halt-the-antenna-where-it-is = Остановить антенну там, где она есть

rotor-park = ПАРК

rotor-hand-the-antenna-back-to-the = Вернуть антенну захвату спутника — она сопровождает спутник и паркуется, когда его нет.

rotor-point = НАВЕСТИ

rotor-dx-country = Страна DX

rotor-set-your-grid-locator-settings = Задайте ваш локатор (Настройки ▸ Общие), чтобы наводить на страну.

rotor-connected = Подключено — антенна на { $az }°

rotor-not-connected-why = Не подключено: { $e }

rotor-beam = Луч { $az }° азим  { $el }° элев

rotor-target = цель { $az }° / { $el }°

rotor-park-hover = Повернуть на азимут парковки ({ $az }° / { $el }°)

rotor-country-bearing = { $name } — азимут { $az }°

aircraft-switch-no-station = нет станции для переключения

aircraft-title = САМОЛЁТЫ

aircraft-mode-adsb-hover = ADS-B, 1090 МГц — самолёты с их положением, высотой и скоростью

aircraft-mode-hfdl-hover = HFDL, КВ-линия передачи данных — самолёты над океанами и полюсами, по их отчётам о положении

aircraft-mode-vdl2-hover = VDL2, УКВ-линия передачи данных — сообщения от самолётов; лишь немногие содержат положение

aircraft-mode-acars-hover = ACARS — короткие сообщения между самолётами и землёй

aircraft-badge-online = онлайн

aircraft-badge-history = история

aircraft-stats-total = самолётов: { $n }

aircraft-stats-most-first = Сначала частые

aircraft-stats-empty = Самолётов пока нет. Таблицы заполняются из ADS-B, VDL2, HFDL и ACARS; здесь ничего не требует интернета.

aircraft-stats-relay-note = Модель, оператор и маршрут учитывают только самолёты, о которых ответила ретрансляция — откройте карточку в разделе «Карточки», чтобы найти один из них.

aircraft-stats-known-kinds = известно: { $known } · видов: { $kinds }

aircraft-and-more = и ещё { $more }

aircraft-show = Показать

aircraft-aviation = Авиация

aircraft-cards-chip = Карточки { $n }

aircraft-cards-hover = Каждый самолёт, услышанный по ADS-B, VDL2, HFDL и ACARS, по одной карточке — хранятся десять минут, в каком бы режиме ни находилось радио сейчас

aircraft-watch-chip = Наблюдение { $n }{ $suffix }

aircraft-watch-hover = Самолёты, о которых вы просили сообщать, по позывному или регистрации — список хранится на станции, и оповещение показывается, когда один из них появляется в эфире

aircraft-stats-chip = Статистика

aircraft-stats-hover = Самые частые категории, авиакомпании, модели, операторы и маршруты среди самолётов, известных этой станции — сначала по радио, с карточками ретрансляции как дополнением

aircraft-export = Экспорт

aircraft-export-csv-hover = Сохранить самолёты в таблицу CSV для электронной таблицы: последние 24 часа станции, если она ведёт историю, иначе то, что слышно сейчас. Без положений.

aircraft-export-json-hover = Сохранить самолёты в JSON: последние 24 часа станции, если она ведёт историю, иначе то, что слышно сейчас. Без положений.

aircraft-watch-intro = Позывные и регистрации, о которых нужно сообщать. Список хранится на станции, поэтому одинаков на всех экранах; оповещение приходит, когда борт появляется в эфире или возвращается после десяти минут молчания.

aircraft-watch-needs-station = Список наблюдения хранится на станции, к которой вы подключены. Подключитесь к станции, чтобы им пользоваться.

aircraft-watch-reading = Чтение списка…

aircraft-watch-cannot-read = Не удаётся прочитать список: { $why }.

aircraft-watch-input-hint = KLM1234 или PH-BXA

aircraft-watch-add = Добавить

aircraft-watch-sending = Отправка…

aircraft-watch-empty = В списке пока ничего нет.

aircraft-watch-on-air = сейчас в эфире

aircraft-watch-remove = Удалить

aircraft-watch-alerts = Оповещения

aircraft-watch-clear = Очистить

aircraft-watch-none-yet = Пока нет.

aircraft-cards-empty = Самолётов пока не слышно. Карточки заполняются из ADS-B, VDL2, HFDL и ACARS — нажмите любой из них и слушайте. Интернет здесь не нужен.

aircraft-cards-no-match = Нет самолётов, соответствующих «{ $query }».

aircraft-history-hover = Показать, где этот самолёт был за последние сутки, по записи самой станции. Запрос идёт к вашей станции, а не в интернет.

aircraft-copy-summary = Копировать сводку

aircraft-copy-summary-hover = Копировать эту карточку как текст: что известно и откуда взята каждая строка. Положение и последнее сообщение не включаются.

aircraft-asking-station-dots = запрос к станции…

aircraft-not-changed = Не изменено: { $why }.

aircraft-loading-photo = загрузка фото…

aircraft-radio-is-on = Радио работает в режиме { $mode } — нажмите { $label }, чтобы слушать. Это то, что { $label } слышал раньше.

aircraft-no-adsb = ADS-B пока не слышно.

aircraft-hfdl-off = Декодер HFDL выключен — нажмите HFDL, чтобы включить его.

aircraft-vdl2-starting = Запуск декодера VDL2…

aircraft-no-view = У { $label } здесь нет вида.

textmodem-waiting-modem = ожидание модема…

textmodem-locked = ЗАХВАТ

textmodem-no-frame = НЕТ КАДРА

textmodem-locked-hint = Кадр отслеживается. Он принимается, только если корреляция синхронизации, свёрнутая по двенадцати кадрам, заметно превышает собственный нулевой уровень, а автокорреляция по периоду кадра подтверждает наличие кадра.

textmodem-sync-readout = синхр. { $sync } (нуль { $null })

textmodem-sync-hint = Дифференциальная корреляция синхронизации, свёрнутая по всем фазам кадра, и медиана по этим фазам — нулевой уровень, который нужно превзойти. Измерено в эфире: 0,85 на 4285 против 0,14 на сильной PSK-несущей другого вида модуляции.

textmodem-frame-readout = кадр { $acf }

textmodem-frame-hint = Какая доля кадра повторяется через один кадр. 4285 повторяет свои 80 символов синхронизации, то есть 0,3–0,6; несущая другого вида модуляции даёт около 0,03.

textmodem-refs-readout = опорн. { $a }/{ $b }/{ $c }

textmodem-refs-hint = Концентрации трёх опорных блоков после дескремблирования. До скремблирования все они — символ 0, поэтому верный кадр стягивает каждый из них в одну точку — 0,7 и выше. Шестнадцать символов шума дают 0,25.

textmodem-rate-hint = Что может сказать созвездие. Каждая некодированная скорость делит созвездие с кодированной, поэтому для различения кодированных скоростей нужно Annex E, а чтобы прочитать их полезную нагрузку — FEC из Annex E.

textmodem-offset-frames = { $offset } Гц · { $n } кадров

textmodem-offset-frames-hint = Смещение несущей, с которым согласуются опорные блоки, и сколько кадров демодулировано за этот сеанс.

textmodem-som-readout = SOM { $best }/32 · { $n } сообщ.

textmodem-som-hint = Шаблон начала сообщения из Annex E и то, насколько близко к нему подошёл выход декодера. Шум держится около 16 из 32; кодированная передача доходит до двадцати с лишним, даже если замирание мешает совпадению, а совпадение определяет кодированную скорость и начинает сообщение. Код и перемежитель точно декодируют сгенерированный сигнал, но ни одна кодированная станция в эфире пока не прочитана, поэтому совпадение здесь — первое, чего стоит ждать, когда такая появится.

textmodem-rx-only = ТОЛЬКО ПРИЁМ

textmodem-rx-only-hint = Декодер этого вида модуляции портирован и читает свои записи; модулятор ещё не написан, поэтому передающая часть отключена, а не отказывает постфактум.

textmodem-msg-hint = Ваши собственные кнопки сообщений — условия работы, погода, стандартный ответ. Каждая отправляет весь свой текст целиком, а F1–F9 нажимают первые девять. Они хранятся вместе с конфигурацией станции, поэтому доступны и удалённому клиенту.

textmodem-weather-chip = {" "}ПОГОДА { $n }{" "}

textmodem-weather-hint =
    Декодированные сводки SYNOP и SHIP, услышанные на этом диапазоне.

    Они хранятся здесь, а не в списке декодирования, потому что цифры сами по себе ничего не значат, а погода в точке три часа назад — это то, к чему возвращаются.

    Находятся в тексте RTTY по мере его поступления, поэтому режим должен быть настроен на сдвиг и скорость станции. Наземная станция не передаёт координат, так что в таких строках указаны номер станции и цифры, но нет координат; судовая сводка несёт свои.

textmodem-olivia-reverse-hint = Инвертировать порядок тонов в обе стороны. Станция, передающая в инверсии, воспринимается как шум, пока это не включено.

textmodem-olivia-tones-raised = (Olivia { $n } → 4)

textmodem-ifk-reverse-hint = Инвертировать шаги тонов. IFK+ несёт данные в шаге между последовательными тонами, поэтому станция, принятая на противоположной боковой полосе, имеет все шаги отрицательными и воспринимается как шум, пока это не включено.

textmodem-domino-hint = { $name } — { $baud } бод, 18 тонов, инкрементная частотная манипуляция без прямой коррекции ошибок

textmodem-mt63-long-hint = MT63-{ $label }, ширина { $bw } Гц — длинное перемежение распределяет пакет ошибок на { $chars } символов

textmodem-mt63-short-hint = MT63-{ $label }, ширина { $bw } Гц, короткое перемежение — короткое перемежение распределяет пакет ошибок на { $chars } символов

textmodem-mt63-not-offered = 1K и 500L пока недоступны

textmodem-mt63-not-offered-hint = MT63-1K и длинное перемежение 500 Гц читаются пакетным декодером, но пока не живым приёмником: захват ищет фазу символа и полярность, но не центр полосы, а поиск, который это делает, оказалось, меняет один вариант на другой. См. docs/internal/MODE-PROGRESS.md.

textmodem-mfsk-caveat-mfsk8 = { " " }— в эталонной записи теряется один фрагмент текста; декодер тот же, что и для всех остальных вариантов

textmodem-mfsk-caveat-mfsk11 = { " " }— в эталонной записи теряется первая группа из-за запуска передискретизатора

textmodem-mfsk-hint = { $label } — { $baud } бод, { $tones } тонов, ширина { $bw } Гц{ $caveat }

textmodem-mfsk-reverse-hint = Инвертировать порядок тонов. Станция, принятая на боковой полосе, противоположной той, на которой она передавалась, приходит зеркально отражённой относительно центра и воспринимается как шум, пока это не включено.

textmodem-psk-family = Семейство

textmodem-psk-family-plain-hint = Собственное семейство PSK31: одна дифференциальная несущая, без прямой коррекции ошибок

textmodem-psk-family-bpsk63f-hint = Надёжный код на 62,5 бод без перемежителя — единственный кодированный режим, который fldigi передаёт без него

textmodem-psk-family-robust-hint = Одна несущая с надёжным кодом и перемежителем 2x2xN

textmodem-psk-family-multi-hint = Несколько несущих, варикод PSK31 и **без** кода — шире и быстрее, требует чистого сигнала

textmodem-psk-family-robust-multi-hint = Несколько несущих **и** надёжный код: самый широкий и самый надёжный из режимов PSK

textmodem-psk-baud-fast-hint = { $baud } бод — читается примерно со второй секунды: на такой скорости время захвата приёмника составляет треть сообщения

textmodem-psk-baud-hint = { $baud } бод

textmodem-psk-modulation = Модуляция

textmodem-psk-bpsk-hint = Один бит на символ, дифференциальное детектирование

textmodem-psk-qpsk-hint = Два бита на символ с кодом со скоростью 1/2, то есть один информационный бит на символ: та же скорость текста, что у BPSK при той же скорости передачи, с вдвое меньшим числом ошибок

textmodem-psk-8psk-hint = Три бита на символ, без кода: самый быстрый из трёх при любой скорости и наименее устойчивый. Только приём — восьмифазного передатчика нет. Проверено на синтетическом сигнале: записи 8PSK в тестовом корпусе не дают чистого созвездия

textmodem-psk-coded = кодир.

textmodem-psk-no-code = без кода

textmodem-psk-coded-hint =
    { $baud } бод, { $n } { $n ->
            [one] несущая
            [few] несущие
            [many] несущих
           *[other] несущей
        }, { $code }, ширина { $bw } Гц — только приём: передатчик кодированного PSK не написан

textmodem-psk-wide-bank-hint = { "*" }*Этот широкий банк пока не декодирует.** Его несущие расположены так близко, что фильтр приёмника на входной частоте дискретизации не может их разделить; более узкие банки читают свои записи. Ничего из слышимого здесь не теряется — дело во входном каскаде, а не в сигнале.

textmodem-hell-slant = Наклон

textmodem-hell-slant-hint = Подстройка тактовой частоты приёма. Запись, в которой точки приходят на 0,4 % длиннее — обычная звуковая карта и то, что показывает внеэфирная запись Hell из SkySweep — требует около -4000. Подстраивайте, пока глифы не встанут прямо.

settings-net-rbn-enabled-hint = Читать скиммеры всего мира и передавать их данные на карту прохождения. Именно это позволяет карте показывать диапазоны, которые это радио не слушает. Ничего не излучается в эфир, учётная запись не нужна, используется позывной со вкладки «Общие». Споты RBN не попадают в список спотов — это измерения, а не приглашения. Хост не привязан к reversebeacon.net: подойдёт любой узел с тем же telnet-потоком.

settings-net-rbn-port-hint = На reversebeacon.net 7000 — поток CW/RTTY, 7001 — FT8/FT4

settings-net-wavelog-intro = Wavelog — это журнал, который **вы** запускаете по заданному вами адресу. Загруженные здесь связи отправляются в выбранное вами место станции, а уже имеющиеся у него связи можно забрать обратно, чтобы «уже работали» знало о QSO, проведённых из другой программы, на другой трансивер или через его веб-форму.

settings-net-wavelog-address-hint = Адрес главной страницы Wavelog: https://log.example.org или его адрес в локальной сети. /index.php принимается в любом случае.

settings-net-wavelog-key-hint = Ключ API со страницы API самого Wavelog. Для выгрузки нужен ключ чтения/записи; ключ только для чтения может лишь скачивать.

settings-net-pick-one = выберите

settings-net-test-to-list-locations = проверьте соединение, чтобы получить список мест

settings-net-trust = ДОВЕРЯТЬ

settings-net-trust-hint = Принять **этот** сертификат и никакой другой для этого сервера. Это закрепление, а не «принимать всё»: если он когда-либо изменится, соединение будет отклонено с сообщением об этом, а не тихо уйдёт к тому, кто отвечает по этому адресу.

settings-net-test-wavelog = Проверить Wavelog

settings-net-test-wavelog-hint = Спросить сервер, работают ли этот адрес и ключ, и получить список его мест станции. Сначала применяет настройки; ничего не заносит в журнал.

settings-net-wavelog-download = Скачать связи из Wavelog

settings-net-every-min = Каждые (мин)

settings-net-locations = Места

settings-net-download-now = Скачать сейчас

settings-net-download-now-hint = Забрать то, что Wavelog добавил с прошлой загрузки, и пометить уже имеющиеся у него связи как выгруженные. Ничего из уже имеющегося в журнале не изменяется.

settings-net-wavelog-upload-note = Автоматически выгружаются только связи, внесённые **здесь**; собственный архив Wavelog отправляется отметкой QSO в журнале, поэтому с этого компьютера не уходит ничего, чего вы не выбрали.

settings-net-broadcast-note = Длинноволновые и коротковолновые станции, подписанные на водопаде. Каждый источник ниже скачивается для текущего вещательного сезона и скачивается заново при смене сезона. Все они попадают в **одно хранилище**, поэтому фильтр «Источник» в окне SCHEDULE показывает один из них или все.

settings-net-downloaded = скачано

settings-net-schedules = Расписания

settings-net-schedule-yours = ваше

settings-net-download = Скачать

settings-net-hfcc-hint = HFCC публикует названия станций в отдельном файле, поэтому нужны оба. Пока его нет, выводится сообщение об этом, а не загружается расписание из одних позывных.

settings-net-refetch-hint = Снова получить копию этого сезона, заменив кэшированную

settings-net-from-file = Из файла…

settings-net-aoki-file-hint = Взять расписание Aoki из сохранённого вами файла — .zip, который публикует Aoki, или .txt из него. Его сайт меняется вместе с сезоном, поэтому этот вариант и существует.

settings-net-eibi-file-hint = Взять расписание EiBi из его sked .csv, сохранённого на этом компьютере

settings-net-filter-aoki = Расписание Aoki

settings-net-filter-eibi = Расписание EiBi

settings-net-download-all = Скачать всё сейчас

settings-net-download-all-hint = Получить все источники, которые можно получить, сохранив уже работающие

settings-net-own-list-note = Ваш собственный список — это **расписание**, то есть то, что вы рассчитываете услышать, а не журнал приёма, в котором записано то, что вы действительно услышали. Импортируйте его ниже или отредактируйте файл.

settings-net-import-list = Импортировать мой список станций…

settings-net-import-list-hint = Прочитать CSV или простой список станций и слить его с файлом выше. Частоты могут быть в кГц или МГц; единица определяется по числу.

settings-net-filter-station-lists = Списки станций

settings-net-dx-status = DX — { $body }

settings-net-pin-own = свой — { $pin }…

settings-net-schedule-rows = { $name } { $n } передач

settings-net-schedule-not-downloaded = { $name }     не скачано

settings-net-schedule-imported = { $source } { $season } — { $n } передач, теперь в хранилище

settings-net-stations-imported =
    { $n ->
        [one] { $n } станция в вашем списке, теперь в хранилище
        [few] { $n } станции в вашем списке, теперь в хранилище
        [many] { $n } станций в вашем списке, теперь в хранилище
       *[other] { $n } станции в вашем списке, теперь в хранилище
    }

txaudio-natural = Естественный

txaudio-ragchews-and-local-contacts-the = Длинные беседы и местные связи: шумоподавитель, мягкий экспандер в паузах, лёгкая компрессия. Звучит как вы, только без комнаты.

txaudio-ssb-everyday = SSB повседневный

txaudio-everyday-ssb-100-hz-2-9-khz-a = Повседневный SSB: 100 Гц – 2,9 кГц, немного присутствия около 2 кГц, умеренная компрессия.

txaudio-ssb-punch-dx = SSB напор / DX

txaudio-weak-signals-and-pileups-a-narrow = Слабые сигналы и пайлапы: узкий голос 300 Гц – 2,7 кГц с сильным подъёмом присутствия, пропущенный через быстрый компрессор с фазовращателем, чтобы получить максимум разговорной мощности передатчика. Не hi-fi.

txaudio-ssb-punch-for-hours-in-a-noisy = SSB напор на долгие часы в шумной станции: шумоподавитель и жёсткий гейт не пускают в паузы вентиляторы и следующего оператора, а компрессор работает ещё сильнее.

txaudio-essb-hi-fi = ESSB / hi-fi

txaudio-wide-ssb-or-am-50-hz-4-khz-with = Широкий SSB или AM: 50 Гц – 4 кГц с сохранением низких частот, мягкая компрессия и де-эссер. Требуется передающий фильтр такой ширины — и тихий диапазон.

txaudio-fm-repeater = FM / ретранслятор

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM: крутой ФВЧ 300 Гц отделяет голос от тона CTCSS, верх обрезается на 3 кГц под канал, а ограничитель удерживает девиацию.

txaudio-mobile-noisy-shack = Мобильная / шумная радиорубка

txaudio-fans-engine-traffic-the-denoiser-a = Вентиляторы, двигатель, дорожный шум: шумоподавитель, крутой ФВЧ на 150 Гц и жёсткий гейт, с достаточной компрессией, чтобы быть выше остатка.

txaudio-digital-voice-2 = Цифровой голос

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE: только шумоподавитель и мягкое выравнивание, без эквалайзера — вокодер предпочитает естественную речь. Включает ЦИФРОВОЙ ГОЛОС.

txaudio-imported-2 = Импортировано

frame-no-contrast-fit = Нет контраста — нажмите для FIT

frame-levels-hidden-hint = Нижняя и верхняя границы дисплея лежат за пределами всего на экране, поэтому водопад одного сплошного цвета. Нажмите, чтобы подогнать их под то, что слышит радио.

frame-bw-waterfall = BW водопад · { $span }

frame-bw-waterfall-unknown = BW водопад · —

frame-every-module-own-window = каждый модуль в своём окне

frame-window-chip-hint = WINDOW — вынести это окно в отдельное окно ОС, чтобы разместить его на другом мониторе

frame-dock-panel-pane-hint = DOCK — вернуть эту панель в область под водопадом

frame-dock-panel-hint = DOCK — вернуть эту панель обратно под водопад

frame-dock-spectrum-hint = DOCK — вернуть этот спектр в главное окно, так же, как закрытие этого окна, без поиска кнопки закрытия.

frame-dock-band-menu-hint = DOCK — вернуть выбор диапазона и режима в главное окно, так же, как закрытие этого окна, без поиска кнопки закрытия.

frame-dock-rotator-hint = DOCK — вернуть поворотное устройство в главное окно, так же, как закрытие этого окна, без поиска кнопки закрытия.

frame-module-panadapter = панадаптер

frame-module-panel = панель

frame-module-controls = управление

frame-module-aux-panadapter = доп. панадаптер

frame-module-band-mode = диапазон и режим

frame-module-rotator = поворотное устройство антенны

frame-detached-title = { $flavor } — { $name }

frame-workspace-missing = Рабочее пространство { $n } не сохранено — сохраните его в Настройки › UI › Рабочие пространства, и эта клавиша вызовет его обратно.

frame-no-operating-panel = В режиме { $mode } нет рабочей панели — она отстыкована, а у этого режима её нет.

frame-rx-only-nudge = Это радио только для приёма — передатчика у него нет. Скрыть элементы передачи и использовать экран прослушивания?

frame-listening-controls = Элементы прослушивания

frame-listening-controls-hint = Скрыть элементы передачи и переключить это радио на экран прослушивания (Настройки → Радио). Ничего не перенастраивается, декодер продолжает работать.

frame-update-available = Доступна SDR Oxide Brown { $tag } — у вас { $version }.

frame-download = Скачать

frame-download-hint = Открывает страницу релиза в браузере

frame-tuning-catching-up = настройка — радио догоняет

frame-roll-up-hint = Свернуть — убрать заголовок и вернуть окно водопаду

frame-unroll-hint = Развернуть — снова показать панель

frame-window-panel-hint = WINDOW — выделить этой панели отдельное окно, чтобы разместить на другом мониторе

frame-panel-window-title = { $mode } — панель

frame-auto-stopped-stop-control = авто остановлено: использован элемент остановки

frame-dont-show-again = Больше не показывать

frame-dont-show-again-hint = Запомнить это подтверждение на данном экране. Блокировка по-прежнему отключена, пока движок работает с --oob-tx — это лишь прекращает повторный показ предупреждения.

frame-cb-window-title = ⚠  11 м (CB) — НЕ ЛЮБИТЕЛЬСКИЙ ДИАПАЗОН

frame-cb-about-to-allow = Вы собираетесь разрешить передачу на диапазоне 11 м.

frame-cb-not-amateur = 11 м — не любительский диапазон. Это отдельная радиослужба со своими каналами, видами излучения, пределами мощности и сертифицированным оборудованием, которые различаются от страны к стране. Передача там подчиняется правилам страны, из которой вы работаете.

frame-cb-only-continue = Продолжайте, только если вы знаете, что ваша страна разрешает на 27 МГц, и работаете в этих рамках — ответственность за передачу остаётся на вас.

frame-cancel = Отмена

frame-saved-to-profile = сохранено в профиле { $name }

frame-saved-as-default = сохранено как профиль станции по умолчанию (на сервере нет пароля, поэтому его получает каждый клиент)

frame-bindings-offer-title = Этот профиль содержит привязки клавиатуры

frame-bindings-offer-profile = Профиль, под которым вы вошли ({ $name }), хранит привязки клавиатуры и мыши на сервере, а этот браузер их не использует. Это безопасное значение по умолчанию: на станции, которой пользуются другие, клавиатура тоже общая, и принятие одним входом чужих PTT или Пробела — реальная опасность.

frame-bindings-offer-default = Профиль станции по умолчанию хранит привязки клавиатуры и мыши на сервере, а этот браузер их не использует. Это безопасное значение по умолчанию: на станции, которой пользуются другие, клавиатура тоже общая, и принятие одним входом чужих PTT или Пробела — реальная опасность.

frame-bindings-offer-choice = Используйте их здесь или оставьте. В любом случае вам сообщат, что выбрано, и ничего не изменится без вашего выбора.

frame-bindings-use = Использовать привязки этого профиля

frame-bindings-keep-mine = Оставить мои (и больше не спрашивать)

frame-bindings-using-profile = используются привязки клавиатуры, сохранённые для этого профиля

frame-bindings-keeping-own = сохраняются собственные привязки клавиатуры этого устройства; больше не спрашивать

frame-look-back-to-profile = возврат к внешнему виду, сохранённому для этого профиля

frame-look-none-stored = для этого профиля ещё нет сохранённого вида — сначала сохраните его

frame-antenna-rotator-caption = ПОВОРОТНОЕ УСТРОЙСТВО АНТЕННЫ

bands-condx-global-forecast-rest-what = CONDX — глобальный прогноз. WSPR — то, что услышала мировая сеть WSPR (wspr.live); PATHS, REACH и BEST — то, что услышали эта станция и RBN. Пустая строка означает, что ничего не декодировано — это может значить, что диапазон был закрыт, или просто что на нём никого не было.

bands-km-s-parent-peak-rate =
    { $name } ({ $code }) — { $velocity_kms } км/с, родитель { $parent }. Пиковая интенсивность ZHR { $zhr } около { $peak }.

    Радиант над горизонтом означает, что следы могут дойти до вас; быстрый поток оставляет более долгоживущие ионизированные следы для метеорного рассеяния на 6 м и 2 м и краткие открытия на 10 м/11 м.

bands-hearing-means-path-open-each = { $callsign } в { $location } — если его слышно, трасса на { $band } открыта. 10 м (28,200 МГц) — ближайший любительский заменитель условий на 11 м. Каждый маяк переходит на диапазон выше каждые 10 с, поэтому эта строка меняется в каждом слоте.

bands-wspr-activity-old = · активность WSPR устарела на { $age } (wspr.live)

bands-psk-activity-old = · активность PSK устарела на { $age }

bands-activity-hover =
    Глобальная активность на { $band } за последние 15 минут по данным { $source }:
    { $paths } отчётов о приёме · { $tx } передатчиков · { $rx } приёмников

    { $note }

    Чем оживлённее диапазон, тем ярче он показан; это означает, что диапазон где-то слышно, а не что он открыт для вас.

bands-derived-hover =
    { $band } не входит в диапазоны, которые оценивает HAMQSL.com; это опубликованная оценка группы { $group }, принятая как ближайший заменитель.

    Прогноз, а не измерение вашей собственной трассы.

bands-wspr-note = Мировая сеть WSPR — маяки для слабых сигналов.

bands-psk-note = Отчёты о приёме от активных режимов: FT8, FT4 и отчёты по CW/RTTY, которые маяки WSPR не охватывают.

aircraft-history-reason-disabled = станция не включила свою историю

aircraft-history-reason-unauthorized = станция требует входа

aircraft-history-reason-no-station = история хранится на станции, к которой вы подключены

aircraft-history-reason-unknown = ответ станции не понят

aircraft-history-show = Показать историю

aircraft-history-hide = Скрыть историю

aircraft-label-history = История

aircraft-label-first-heard = Впервые услышан

aircraft-label-last-heard = Последний раз услышан

aircraft-label-track-points = Точки трека

aircraft-label-highest = Макс. высота

aircraft-label-callsigns = Позывные

aircraft-label-registrations = Регистрации

aircraft-history-nothing-kept = для этого самолёта пока ничего не сохранено

aircraft-history-by-station = этой станцией

aircraft-history-kept-24h = хранится за последние 24 ч

aircraft-history-of-kept = из сохранённых точек

aircraft-history-used-24h = использовано за последние 24 ч

aircraft-history-seen-24h = замечено за последние 24 ч

aircraft-history-looked-up = запрошено { $age } назад

aircraft-alert-on-air-as = { $entry } в эфире как { $callsign }

aircraft-alert-on-air = { $entry } в эфире

aircraft-watch-already = Уже в списке.

aircraft-watch-absent = Нет в списке.

aircraft-watch-invalid = Это не позывной и не регистрация: от 3 до 10 букв или цифр.

aircraft-watch-full = Список заполнен: 100 самолётов.

multi-module-windows = ОКНА МОДУЛЕЙ

multi-in-its-own-window = В ОТДЕЛЬНОМ ОКНЕ

multi-docked = ПРИСТЫКОВАНО

multi-dock-all-windows = ПРИСТЫКОВАТЬ ВСЕ ОКНА

multi-bring-every-module-window-back = Вернуть все окна модулей в главное окно

multi-undock-all-windows = ОТСТЫКОВАТЬ ВСЕ ОКНА

multi-put-every-module-in-a-window-of = Поместить каждый модуль в отдельное окно — главное окно останется со спектром и декодерами

multi-two-radios-side-by-side-need-a = Два радио рядом требуют более широкого окна — этот экран показывает по одному

multi-the-station-s-windows-some-modules = Окна станции — некоторые модули находятся в отдельных окнах ОС. Откройте это, чтобы изменить.

multi-put-a-module-in-its-own-window-sp1 = Поместить модуль в отдельное окно — SP1 или AUX SP на другом мониторе, управление RX рядом с ними. Те же строки есть в Настройки → UI → Отстыкованные.

multi-module-sp1 = SP1 — спектр

multi-module-sp2 = SP2 (AUX) — второй спектр

multi-module-panel = Рабочая панель

multi-module-controls = Управление RX

multi-module-band-mode = Диапазон и режим

multi-module-rotor = Поворотное устройство антенны

multi-bring-back = Вернуть { $name } в главное окно

multi-put-own-window = Поместить { $name } в отдельное окно ОС — со своим app-id, чтобы правило окна могло закрепить его за монитором

multi-windows-chip = ОКНА

aircraft-stats-category = Категория

aircraft-stats-airline = Авиакомпания

aircraft-stats-model = Модель

aircraft-stats-operator = Оператор

aircraft-stats-route = Маршрут

aircraft-stats-src-no-station = Подсчёт самолётов, слышимых сейчас. Станция, к которой вы подключаетесь, хранит последние сутки.

aircraft-stats-src-asking = Запрос к станции за последние сутки…

aircraft-stats-src-ready = Последние 24 ч, сохранённые станцией, вместе с самолётами, слышимыми сейчас.

aircraft-stats-src-disabled = Станция не включила свою историю — подсчёт самолётов, слышимых сейчас.

aircraft-stats-src-reason = { $reason } — подсчёт самолётов, слышимых сейчас.

aircraft-export-saved-station = Сохранено { $name }: последние 24 часа станции.

aircraft-export-saved-local = { $reason }: вместо этого сохранены { $n } самолётов, услышанных за последние десять минут.

login-remember-this-station = Запомнить эту станцию

login-remember-me = ЗАПОМНИТЬ МЕНЯ

login-this-station-keeps-your-sign-in = Эта станция хранит ваш вход, а не эта страница — на других радио ничего заполнять не нужно, и при выходе ничего забывать не нужно.

login-not-this-time-this-browser-forgets = Не в этот раз: браузер забудет это при закрытии, поэтому в следующем сеансе вас спросят снова, и на устройстве не останется ничего об этом входе.

aircraft-label-range = Дальность

aircraft-label-vertical = Вертикаль

aircraft-label-emergency = Авария

aircraft-from-your-station = от вашей станции

aircraft-footer-disabled = онлайн недоступен — ретрансляция на станции выключена

aircraft-footer-no-station = онлайн недоступен — нет станции для запроса

aircraft-footer-unauthorized = онлайн недоступен — вход не выполнен

aircraft-hover-close = Закрыть (Esc)

aircraft-hover-lookup = Найти онлайн

aircraft-hover-lookup-hint = Запросить у ретранслятора этой станции данные об этом самолёте: тип, оператор, маршрут и фото. Пока вы не нажмёте, ничего не запрашивается.

aircraft-hover-lookup-waiting = выполняется другой запрос — по одному

aircraft-hover-history-hint = Где этот самолёт был за последние сутки, по записи вашей станции. Запрос идёт к вашей станции, а не в интернет.

auto-stopped-reason = авто остановлено: { $reason }

auto-stopped-idle = авто остановлено: { $mins } минут без действий оператора

auto-answering = { $role }: ответ { $call }

auto-caller-went-away = авто: вызывающий ушёл

auto-nobody-left = авто: отвечать больше некому

auto-calling-cq = { $role }: вызов CQ (#{ $n })

auto-cq-no-answer = авто cq: нет ответа после { $limit } вызовов — слушаем (авто rx)

auto-listening = { $role }: слушаем

auto-role-stopped = { $role } остановлено: { $sentence }

auto-rx-listening = авто rx: слушаем

auto-cq-calling = авто cq: вызов CQ

auto-cont-running = авто cont: работает

aircraft-search-missing-online = { $missing } из { $total } самолётов без онлайн-данных — откройте их карточки

aircraft-search-shown-of = { $shown } из { $total }

aircraft-search-help =
    Поиск среди уже услышанных самолётов: по адресу (ICAO), позывному, регистрации, типу, оператору или категории. Несколько слов должны совпасть все. Ограничить слово одним полем можно с помощью icao: call: reg: type: op: — например  reg:PH op:KLM.
    Тип и оператор берутся из онлайн-поиска станции, поэтому находятся только у самолётов, чья карточка уже запрашивалась. Поиск ничего не запрашивает.

aircraft-search-hint = поиск

aircraft-search-clear = Очистить поиск

gridtracker-shade-squares-heard-live-decode = Затенять то, что слышно в живом списке декодирования, а не только то, что есть в журнале. Вариант карты для слушателя: то, что в эфире сейчас, голубым под отметками отработанных.

gridtracker-grid-ham = GRID/HAM

gridtracker-maidenhead-squares-what-a-ham-log = Квадраты Maidenhead — то, что содержит любительский журнал. Позывные CB сюда не попадают: обмен на 11 м содержит страну, но не локатор.

gridtracker-country-cb = COUNTRY/CB

gridtracker-dxcc-countries-at-their-nominal = Страны DXCC в их номинальном центре — то, что содержит журнал CB, поскольку в обмене на 11 м нет локатора. Любительские позывные сюда тоже не попадают: это страны, а не квадраты.

colouredit-paste-a-colour-code-rrggbb-or-r-g = Вставьте код цвета (#RRGGBB или r, g, b) — или Cmd/Ctrl+V на образце

cwkey-thread-failed = не удалось запустить поток CW-ключа: { $error }

cwkey-grab-failed = не удалось захватить манипулятор монопольно — его контакты могут также щёлкать в других окнах (не использует ли его другая программа?)

ignore-muted-hover =
    Заглушены на этот сеанс: { $names }.

    Заглушённая станция исключается из списка декодирования, не вызывает оповещений, не озвучивается и никогда не предлагается автоматическому режиму для ответа. От её имени ничего не передаётся и не выгружается, а список забывается при закрытии sdroxide — сохранённой копии нет, так что после вечера прослушивания ничего не остаётся скрытым.

    Нажмите, чтобы включить их все, или нажмите кнопку − в любой из их строк.

js8-setup-hint = НАСТРОЙКА — позывной, группы, автоответ, интервал маяка и статусное сообщение

logbook-contest-logger-a-session-dupes-a = Контестный журнал — сеанс, дубли, текущий счёт и Cabrillo

logbook-up-chip = UP

net-wavelog-stopped = остановлено: { $error }

net-wavelog-download-stopped = Загрузка из Wavelog остановлена: { $error }

net-wavelog-new-already-here = { $added } новых, { $matched } уже есть

net-wavelog-fill-in-first = сначала нужно заполнить адрес и ключ API

net-wavelog-no-station-location = нет места станции для скачивания — сначала проверьте соединение

publicsdr-could-not-connect = не удалось подключиться в этом сеансе

publicsdr-use-chip = USE

weather-title = СВОДКИ ПОГОДЫ

weather-intro = Кодированные сводки погоды от судов и наземных станций, в RTTY на 4583, 7646 и 10100,8 кГц. SHIP передаётся около 02:00 и 07:35 UTC, SYNOP около 00:35 и 12:10. Сводка хранится здесь, а не в списке декодирования, потому что цифры сами по себе ничего не значат.

weather-nothing-decoded = Пока ничего не декодировано. Остаётся лишь оставить RTTY работать в одно из указанных выше времён — это самые тихие часы на диапазоне.

weather-head-call-day-hour = { $call } — { $day } в { $hour }:00 UTC

weather-head-day = в { $day }:00 UTC

weather-head-report = сводка

weather-not-reported = { $label }: не сообщается

weather-in-their-words = их словами: { $remark }

weather-ago-now = только что

weather-ago-minute = минуту назад

weather-ago-minutes = { $m } мин. назад

weather-ago-hours =
    { $h } { $h ->
            [one] час
            [few] часа
            [many] часов
           *[other] часа
        } назад

weather-ago-days = { $d } дн. назад

sat-off-only-satellites-with = Выкл.: только спутники с известными частотами — те, с которыми можно работать, — и те, что вы добавили сами. Вкл.: все спутники, которые отслеживает станция, в большинстве своём те, которых никто не слышит.

adsbmap-tune-hint = двойной щелчок — настроить · shift — другая шкала

adsbmap-airport-large = крупный аэропорт · высота { $elev }

adsbmap-airport-medium = средний аэропорт · высота { $elev }

adsbmap-from-station = { $dist }  { $bear }° от вашей станции

adsbmap-aircraft-within =
    { $n } { $n ->
        [one] самолёт
        [few] самолёта
        [many] самолётов
       *[other] самолёта
    } в пределах { $dist }

adsbmap-navaid-mw = { $khz } кГц (средние волны)

adsbmap-click-to-close = щёлкните по карте, чтобы закрыть

settings-ui-broadcast-stations-hint =
    Какие вещательные станции могут показывать окно LISTEN и подписи на водопаде.

    { "**" }В этой полосе** — то, что делали все экраны до появления выбора: полполосы в обе стороны от настройки, что и должна охватывать подпись на этом водопаде. **Весь диапазон** показывает всё, что есть в расписании с положением на нём, — это то, что ищет слушатель, просматривающий диапазон, — ценой более загруженной карты.

settings-ui-waterfall-labels = Подписи на водопаде

settings-ui-tuned-only = Только настроенная

settings-ui-tuned-only-hint = Называть станцию, на которую вы настроены, и больше ничего — одно название для того, что вы слушаете, а не стена названий над полосой пропускания.

settings-ui-whole-band = Весь диапазон

settings-ui-whole-band-hint = Называть каждого вещателя, который сейчас в эфире по расписанию, где бы он ни находился. То, что нужно при просмотре диапазона, а не прослушивании одной станции, и от этого он более загружен. Также в окне SPOTS как BC ALL.

settings-ui-classic-ui = Классический интерфейс

settings-ui-classic-ui-hint =
    Разместить все элементы управления в одну колонку слева со сворачиваемыми группами, как это делают SDR++ и SDR++Brown, а всё остальное отдать изображению. Те же элементы, те же настройки, расположенные так, как большинство коротковолновых слушателей уже привыкло.

    Нужно окно достаточной ширины, чтобы водопад сохранил свою долю: уже этого колонка опускается, а полоса рисуется как обычно; панель можно сделать шире или уже, потянув за правый край. В выключенном состоянии используется собственная полоса программы, как раньше.

settings-ui-classic-ui-check = элементы управления в левой колонке

settings-ui-updates = Обновления

settings-ui-updates-check = проверять новый релиз при запуске

settings-ui-updates-hint = Один раз при запуске запрашивает у GitHub Releases форка последний релиз и показывает баннер, если он новее этой сборки. Учитываются только полные релизы — ночная сборка является предварительным релизом, — а при выключении исходящих обращений нет вообще.

settings-ui-undocked = Отстыковано — своё окно

settings-ui-docked = Пристыковано в этом окне

settings-ui-dock-all-hint = Вернуть все отстыкованные окна в это окно.

settings-ui-undock-all-hint = Поместить каждый модуль в отдельное окно — главное окно останется со спектром и декодерами. Модуль, которому нечего показывать, например рабочая панель в голосовом режиме, окна не открывает.

settings-ui-spectrum-line = Линия спектра

settings-ui-spectrum-line-hint = Цвет под линией спектра и способ его затенения: «Фиксированный» — один цвет везде, «Градиент» — полный внизу и затухающий к верху, «Смешанный» — полный в нижней половине и затухающий в верхней. Сама линия остаётся голубой. Чип FILL во всплывающем окне SPEC переключает заливку, а его строка линии выбирает затенение.

settings-ui-fill = Заливка

settings-ui-fill-colour = цвет

settings-ui-simple-ui = Простой интерфейс

settings-ui-simple-ui-hint =
    Скрыть из верхней полосы дополнительные функции: 3D-вид, скиммеры, переключатели слоёв спектра/водопада, учёт наград, спутники, декодирование ISM и радиопочту. Элементы, которыми пользуется оператор CB или коротковолновый слушатель — настройка, режим, громкость, шумоподавитель, полоса пропускания, водопад, память, сканирование — остаются.

    Ничего не выключается, лишь скрывается: включите это обратно, чтобы вернуть чипы.

settings-ui-simple-ui-check = скрыть дополнительные чипы

settings-ui-wake-lock = Блокировка сна

settings-ui-wake-lock-hint =
    Держать экран включённым, пока эта страница на переднем плане, чтобы телефон или планшет, оставленный на водопаде, не тускнел и не блокировался через минуту без касаний. Приёму, звуку и декодированию это не нужно — они продолжают работать в любом случае — это нужно, чтобы экран оставался читаемым.

    Работает в паре с замком на полосах телефона и планшета: заблокируйте экран от случайных пальцев и держите его освещённым.

    Браузер забирает блокировку обратно, когда страница скрыта — другая вкладка или собственная кнопка питания телефона, — и она запрашивается снова, как только страница возвращается. Нужен браузер, который это умеет, и защищённая страница (https или localhost); строка рядом с флажком сообщает, когда это недоступно или было отклонено. Это расходует батарею, поэтому по умолчанию выключено. Установленная программа на компьютере вместо этого следует собственным настройкам питания системы.

settings-ui-wake-lock-check = держать экран включённым

settings-ui-sound-welcome = Приветствие звука

settings-ui-sound-welcome-hint =
    Когда веб-страница открывается — новый сеанс или перезагрузка по F5 — браузер не воспроизводит звук, пока вы не коснётесь страницы. Это правило браузера, и без слова на экране новый сеанс выглядит так, будто не работает.

    Когда это включено, посреди экрана появляется сообщение об этом: 🔇 Коснитесь, щёлкните или нажмите клавишу в любом месте, чтобы включить звук. Первый щелчок, касание или клавиша — где угодно, мышью, тачпадом, пальцем или клавиатурой — закрывает его и включает звук и больше ничего не делает: не настраивает, не нажимает кнопку и не запускает команду. Оно показывается только при открытии страницы и больше не появляется в течение сеанса, что бы позже ни случилось со звуком.

    Выключите, как только поймёте, что делать: звук всё равно ждёт вашего первого касания, и это касание затем работает как обычно. Хранится вместе с вашим профилем на сервере. По умолчанию включено.

settings-ui-sound-welcome-check = приветственное сообщение о звуке

settings-ui-sound-welcome-browser-only = только в браузере — установленная программа воспроизводит звук без касания

settings-ui-profile-screen = Экран следует за профилем

settings-ui-profile-screen-hint =
    Когда вы — удалённый клиент `sdroxide --server`, *внешний вид* этого экрана — тема, раскладка, водопад и спектр, шрифты, простой интерфейс, слои карты, собственный вид панадаптера — хранится вместе с профилем, под которым вы вошли, и возвращается на любом компьютере, а не начинается со значений по умолчанию.

    { "**" }Сохраняется на сервере и сохраняется по ходу работы, пока вы вошли** — ничего нажимать не нужно. Это сделано намеренно: на хранилище браузера нельзя полагаться, потому что `persist()` ведёт себя по-разному в каждом браузере, а сеанс, хранящийся там, не может сбросить никто, кто не находится у этого устройства. Профиль на сервере — это файл, который оператор станции может вернуть на место.

    На сервере **без пароля** каждый клиент — один и тот же профиль, поэтому ничего не записывается, пока вы не нажмёте кнопку ниже — тема одного оператора не должна становиться темой следующего. Ничего, что относится к машине, не сохраняется, а локальное радио не затрагивается.

settings-ui-save-to-profile = Сохранить в профиль

settings-ui-back-to-profile = Вернуться к профилю

settings-ui-carry-bindings = Переносить привязки управления

settings-ui-carry-bindings-hint =
    Также хранить привязки клавиатуры и мыши вместе с профилем на сервере, чтобы они следовали за входом между браузерами и устройствами — та же идея, что и для экрана выше, но для клавиш.

    { "**" }НЕ РЕКОМЕНДУЕТСЯ и выключено по умолчанию.** На станции, которой пользуются несколько человек, клавиатура общая: профиль с привязками может переназначить PTT, Пробел или клавиши настройки другого оператора в момент входа, а включение может иметь другие последствия, которые сейчас неочевидны. Включайте только там, где сервер принадлежит только вам и вы заходите на него с нескольких своих устройств.

    Только в форке: это не функция оригинала. Настройки экрана выше переносятся всегда; это единственное включаемое здесь.

settings-ui-carry-bindings-on = вкл. — НЕ РЕКОМЕНДУЕТСЯ

settings-ui-carry-bindings-off = переносить (не рекомендуется)

settings-ui-carry-bindings-title = Переносить привязки управления на сервере?

settings-ui-carry-bindings-body =
    Это сохранит ваши привязки клавиатуры и мыши вместе с профилем на сервере, чтобы они следовали за вашим входом между браузерами и устройствами.

    На станции, которой пользуются другие, клавиатура общая. Профиль с привязками может переназначить PTT, Пробел или клавиши настройки другого оператора просто при входе — а включение может иметь другие последствия, которые сейчас неочевидны.

    Поэтому это не рекомендуется. Включайте, только если сервер принадлежит только вам.

settings-ui-carry-bindings-confirm = Я понимаю — всё равно переносить

settings-ui-start-swl = Запускать в режиме SWL

settings-ui-start-swl-hint =
    Открывать каждый сеанс с уже включённым режимом SWL, чтобы программа запускалась с экраном слушателя — для каждого радио, поскольку это предпочтение экрана, а не радио.

    Переключатель для отдельного радио находится в Настройки → Радио → Элементы передачи, поэтому станция с одним приёмным радио и одним трансивером настраивает приёмное там и оставляет это выключенным. Использование того переключателя отключает это начальное значение на время сеанса. `--swl` всегда имеет приоритет.

settings-ui-start-swl-check = запускать с включённым режимом SWL

settings-ui-listen-hides-tx = Вкладка LISTEN скрывает передачу

settings-ui-listen-hides-tx-hint =
    Выбор **LISTEN** в меню диапазона/режима скрывает все элементы передачи на этой вкладке, а выбор **OPERATE** возвращает то, что задано собственной настройкой этого радио.

    Это не может включить передатчик. Радио, настроенное только на приём, остаётся только на приёме, а блокировка диапазона и предупреждение о выходе за диапазон не затрагиваются.

    Меню диапазона запоминает, на какой вкладке его оставили, поэтому станция, оставленная на прослушивании, возвращается к прослушиванию — элементы передачи скрыты и при следующем запуске, а выбор OPERATE возвращает их. По умолчанию выключено, потому что меняет смысл вкладки; спрашивает один раз.

settings-ui-listen-hides-tx-check = скрывать TX на вкладке LISTEN

settings-ui-listen-hides-tx-title = Вкладка LISTEN будет скрывать элементы передачи

settings-ui-listen-hides-tx-body =
    На вкладке LISTEN элементы передачи исчезают; на OPERATE они возвращаются ровно такими, какими их оставила собственная настройка этого радио.

    Это ничего не включает. Это лишь вид той же станции, поэтому блокировка диапазона, предупреждение о выходе за диапазон и защита передачи не затрагиваются.

    Вкладка запоминается: станция, оставленная на LISTEN, запускается на LISTEN, с по-прежнему скрытыми элементами передачи, пока не будет выбран OPERATE.

    Стоит знать: станция, оставленная на LISTEN, всё равно принимает на том, на что настроена вкладка, а скрытый элемент управления — не то же самое, что выключенный: передача выключена потому, что так говорят настройки, а не потому, что кнопка не видна.

settings-ui-listen-hides-tx-confirm = Я понимаю — скрывать на LISTEN

settings-ui-swl-mode = Режим SWL

settings-ui-swl-mode-hint =
    Скрыть все элементы передачи для этого радио — PTT, CALL CQ, уровень TX, SEND, BEACON, всё — и заменить любительские дополнения полосы (споты, награды) на слушательские (SCHEDULE и SWL LOG, журнал приёма). Тот же переключатель, что и в Настройки → Радио → Элементы передачи, и для каждого радио отдельно: приёмный донгл может находиться в этом режиме, пока трансивер рядом сохраняет свой передатчик.

    Оборудование по-прежнему может передавать; это лишь скрывает элементы управления. `--swl` принудительно включает его для каждого радио на время запуска.

    Вступает в силу после «Применить».

settings-ui-swl-mode-check = скрыть все элементы передачи

settings-ui-report-picture = Картинка рапорта

settings-ui-report-picture-hint =
    Картинка, которую несёт рапорт о приёме при отправке вещателю по почте — ваша карточка, фотография, всё, что вы хотите показать станции. Одно изображение для станции, используется во всех рапортах.

    Оставьте не заданным, и рапорт уйдёт простым текстом, что вполне хороший рапорт.

settings-ui-report-picture-none = нет — рапорт уйдёт текстом

settings-ui-choose = Выбрать…

settings-ui-report-picture-pick = Выберите картинку на этом компьютере

settings-ui-report-note = Примечание к рапорту

settings-ui-report-note-hint = Ваше постоянное примечание, подставляемое в каждый рапорт о приёме — кто вы, на что слушаете, всё, что станция спрашивает один раз, а не в каждом письме. Простой текст в простом файле: не секрет.

settings-ui-report-note-hint-text = необязательно — напр. «Подробности по запросу»

sstv-keep-dial = ДЕРЖАТЬ НАСТРОЙКУ

sstv-keep-dial-on-hint = Вкл.: выбор SSTV оставляет настройку там, где вы настроились, поэтому декодер работает на любой частоте. Опубликованные частоты SSTV по-прежнему в списке ⇵. Нажмите, чтобы вернуться к переходу на частоту SSTV диапазона.

sstv-keep-dial-off-hint = Выкл.: выбор SSTV переносит настройку на частоту SSTV диапазона, если вы дальше чем на 3 кГц от неё. Нажмите, чтобы оставить настройку там, где вы настроились, и декодировать SSTV на любой частоте. Настройка после выбора SSTV никогда не отменяется ни в том, ни в другом случае.

sstv-clear-rx = Очистить Rx

sstv-clear-rx-hint = Очистить картинку, показанную сейчас в виде LIVE. Декодер продолжает работать — используйте «Перезапуск RX», чтобы бросить принимаемую картинку и ждать следующий заголовок.

sstv-audio-present-no-header = звук есть · заголовка SSTV пока нет

sstv-editor = Редактор…

sstv-editor-hint = Составить саму картинку: перемещаемый текст, любой шрифт, полоса поперёк или вдоль кадра

sstv-save-image-as = Сохранить изображение как…

sstv-save-copy-asks-file = Сохранить копию на этом компьютере — запросит имя файла

sstv-picture-still-loading = Картинка ещё загружается

sstv-share-hint-direct = Отправить эту картинку кому-нибудь — WhatsApp, Telegram или чем-то ещё на этом устройстве

sstv-share-hint-save = Сохранить эту картинку и открыть сообщение для её отправки

sstv-share-picture = Поделиться картинкой…

sstv-reupload-hint = Отправить эту картинку обратно в эфир — загружает слот передачи { $slot }

sstv-radio-cannot-transmit = Это радио не может передавать

sstv-reupload = Повторная отправка

sstv-unidentified = не опознано

sstv-send-picture = Отправить эту картинку

sstv-qsl-to = Кому

sstv-the-card = Карточка

sstv-the-card-hint = Это попадает на печатную карточку рядом с картинкой. Оставьте любое пустым, и оно просто не будет нарисовано — карточка никогда не сообщает рапорт, которого никто не давал.

sstv-date-utc = Дата (UTC)

sstv-rst-sent = RST передан

sstv-rst-received = RST принят

sstv-qsl-note = Примечание

sstv-add-address = Добавьте адрес, чтобы отправить это.

sstv-saves-opens-mail-hint = Сохраняет картинку и открывает вашу почтовую программу — вложить её там одним щелчком.

sstv-browser-cannot-attach-hint = Вкладка браузера не может прикрепить файл к сообщению, поэтому картинка сохраняется и открывается сообщение; вложите файл вручную.

sstv-save-open-mail = Сохранить картинку и открыть почту

sstv-mail-not-opened = Картинка сохранена, сообщение скопировано — вставьте его в почтовую программу. Не удалось открыть ни один почтовый обработчик.

sstv-qsl-saved-where-chosen = { $name } (сохранено там, где вы выбрали)

sstv-qsl-downloaded = { $name } — скачано в папку загрузок этого браузера

sstv-picture-editor = Редактор картинки

sstv-slot-n = Слот { $n }

sstv-slot-layers-hint = Собственные слои этого слота передачи

sstv-station-overlay = Наложение станции

sstv-station-overlay-hint = Один набор слоёв, который каждый слот рисует под своими — позывной в углу по вашему выбору. Задаётся один раз, совмещается с собственным текстом каждого слота, а не заменяет его.

sstv-draw-under-slot = рисовать под этим слотом

sstv-draw-under-slot-hint = Рисовать наложение станции под собственными слоями этого слота. Выключайте для картинки, уже содержащей идентификацию — обычно это ретранслируемая мозаика, — чтобы она не ставилась дважды.

sstv-overlay-note = Наложение станции: каждый слот рисует эти слои под своими. Перетащите строку на картинке, чтобы переместить её.

sstv-slot-text-note = Собственный текст этого слота, нарисованный там, куда вы его поставили. Перетащите строку на картинке, чтобы переместить её; полоса строки перемещается вместе с ней.

sstv-no-picture-in-slot = В этом слоте нет картинки — загрузите её ниже

sstv-base = Основа

sstv-base-hint = Картинка какого слота лежит под этими слоями

sstv-compose-over-hint = Наложить на картинку этого слота

sstv-slot-empty = Этот слот пуст

sstv-picture-btn = Картинка…

sstv-load-picture-hint = Загрузить картинку в этот слот

sstv-use-received = Взять принятую

sstv-use-received-hint = Положить только что пришедшую картинку под эти слои, чтобы ответ нёс его картинку и ваш позывной

sstv-received-now-base = принятая картинка теперь основа

sstv-add-text = ＋ Текст

sstv-add-strip = ＋ Полоса

sstv-n-lines =
    { $n } { $n ->
            [one] строка
            [few] строки
            [many] строк
           *[other] строки
        }

sstv-draw-this-line = Рисовать эту строку

sstv-strip-paren = (полоса)

sstv-draw-later-hint = Рисовать эту строку позже расположенных выше

sstv-draw-earlier-hint = Рисовать эту строку раньше

sstv-copy = Копировать

sstv-face = Шрифт

sstv-colour = Цвет

sstv-line-text-hint = что говорит строка

sstv-the-letters = Буквы

sstv-composer-outline = контур

sstv-the-edge = Кайма вокруг них

sstv-placement = Расположение

sstv-down-frame = вдоль кадра

sstv-down-frame-hint = Повернуть строку на четверть оборота, чтобы она читалась сверху вниз — позывной вдоль края картинки

sstv-strip = Полоса

sstv-behind = позади

sstv-behind-hint = Полоса цвета позади строки, поперёк картинки или вдоль неё, с центром на строке

sstv-strip-first-edge = Полоса у первого края

sstv-thickness = Толщина

adsbpanel-paths = PATHS

adsbpanel-paths-hint = Рисовать путь каждого рейса одной линией — где он был, через где он сейчас, туда, куда направляется. В выключенном состоянии трек рисуется цепочкой точек.

adsbpanel-nav = NAV

adsbpanel-nav-hint = Аэропорты и радионавигационные средства под картинкой, из встроенных таблиц — крупные аэропорты всегда, средние ближе, а навигационные средства — когда вид достаточно близок, чтобы станция была местом. Средневолновые маяки, которые можно услышать, рисуются своим цветом.

adsbpanel-units = Единицы

adsbpanel-units-hint = Как записываются высота, скорость, скороподъёмность и расстояние в видах самолётов. Декодер и экспорт не затрагиваются.

adsbpanel-age = AGE

touchlock-locked-hover = Экран заблокирован от случайных касаний. Удерживайте здесь 3 секунды, чтобы разблокировать.

touchlock-unlocked-hover = Заблокировать экран от случайных касаний: частота, диапазон, режим, фильтры и водопад перестают реагировать на палец. Приём и звук продолжаются. Удерживайте 3 секунды, чтобы разблокировать. После разблокировки замок отдыхает 10 секунд, прежде чем касание снова заблокирует его, чтобы палец, который разблокировал, не мог тут же заблокировать обратно.

touchlock-hold = УДЕРЖИВАЙТЕ { $s } с…

touchlock-unlocked = РАЗБЛОКИРОВАНО

touchlock-locked-hold = ЗАБЛОКИРОВАНО · удерживайте 3 с

touchlock-unlocked-relock = РАЗБЛОКИРОВАНО · блокировка через { $s } с

wake-held = экран включён

wake-asking = запрос к браузеру…

wake-unsupported = здесь недоступно — нужен браузер с этой функцией и страница https

wake-refused = отклонено браузером (энергосбережение?)

wake-native = только в браузере — установленная программа следует настройкам питания системы

tooltipcopy-copied = ✔ Скопировано

chrome-hide-message-hover = Скрыть это сообщение навсегда. Ваши настройки не меняются, а совет есть в руководстве.

chrome-close-message-hover = Закрыть это сообщение. Больше ничего не меняется.

settings-ui-window-label = Окно { $module }

settings-ui-undock-hint =
    После отстыковки этот модуль перемещается в своё окно, а главное окно отдаёт освободившееся место остальному — поместите водопад на второй монитор или управление на третий. Закрытие его окна пристыковывает его обратно.

    В Wayland размер восстанавливается, но место окна решает композитор: сделайте его плавающим и закрепите за монитором правилом окна с app-id "{ $app_id }".

settings-ui-workspace-restore-out = Вернуть эту раскладку — модулей в отдельных окнах: { $n }

settings-ui-profile-in-use-named = используется: профиль { $name }

settings-ui-profile-in-use-default = используется: профиль станции по умолчанию

settings-ui-profile-not-in-use = пока не используется — для этого входа ничего не сохранено

settings-ui-module-panadapter = Панадаптер

settings-ui-module-controls = Управление

settings-ui-module-aux = AUX SP (второй спектр)

settings-ui-module-band-menu = Выбор диапазона и режима

settings-ui-broadcast-scope-in-view = В этой полосе

settings-ui-fill-fixed = Фиксированный

settings-ui-fill-gradient = Градиент

settings-ui-fill-mixed = Смешанный

settings-ui-spot-kind-heard-me = УСЛЫШАЛИ МЕНЯ

settings-ui-wake-held = экран включён

settings-ui-wake-asking = запрос к браузеру…

settings-ui-wake-unsupported = здесь недоступно — нужен браузер с этой функцией и страница https

settings-ui-wake-refused = отклонено браузером (энергосбережение?)

settings-ui-wake-native = только в браузере — установленная программа следует настройкам питания системы

settings-ui-workspaces = Рабочие пространства

settings-ui-workspaces-note = Именованная раскладка окон — какие модули в отдельных окнах и где эти окна находятся. Сохраните текущую или верните сохранённую. До десяти.

settings-ui-workspaces-none = Рабочих пространств пока не сохранено.

settings-ui-workspace-restore-home = Вернуть эту раскладку — все модули в главном окне

settings-ui-workspace-forget = Забыть это рабочее пространство

settings-ui-workspace-name = назовите эту раскладку

settings-ui-workspace-save = СОХРАНИТЬ ТЕКУЩУЮ

settings-ui-workspace-full = Десять рабочих пространств — предел, который хранит SDRuno; сначала удалите одно

settings-ui-workspace-save-hint = Сохранить текущую раскладку под этим именем

settings-ui-workspace-name-first = Сначала введите имя

sstv-composer-fade = затухание

sstv-layouts = МАКЕТЫ

sstv-name-layout-hint = назовите этот макет

sstv-keep-layout-hint = Сохранить этот макет под этим именем

sstv-layout-renamed = переименовано в «{ $name }»

sstv-layout-saved-as = сохранено как «{ $name }»

sstv-rename-cancelled = переименование отменено

sstv-no-layouts-yet = Сохранённых макетов пока нет — назовите один выше.

sstv-load-layout-hint = Загрузить этот макет в редактируемое

sstv-layout-loaded = загружено «{ $name }»

sstv-rename-layout-hint = Переименовать «{ $name }»

sstv-layout-renaming = переименование «{ $name }»

sstv-duplicate-layout-hint = Дублировать «{ $name }»

sstv-layout-copy-name = { $name } копия

sstv-layout-duplicated = продублировано как «{ $name }»

sstv-forget-layout-hint = Забыть макет «{ $name }»

sstv-layout-forgot = забыто «{ $name }»

sstv-mosaic-header = Мозаика из принятых картинок

sstv-mosaic-nothing = Пока ничего не принято — мозаика собирается из картинок галереи.

sstv-mosaic-picked = Выбрано { $n } из { $max }, расположены в этом порядке — порядок задаёт позицию в сетке. Каждая картинка сохраняет свою форму; промежутки чёрные.

sstv-mosaic-earlier-hint = Сдвинуть эту картинку на одну ячейку раньше

sstv-mosaic-later-hint = Сдвинуть эту картинку на одну ячейку позже

sstv-mosaic-max = { $n } — максимум, который вмещает мозаика

sstv-mosaic-compose = Собрать в этот слот

sstv-mosaic-compose-hint = Собрать мозаику и загрузить её в этот слот как его картинку; принятые оригиналы не затрагиваются

sstv-mosaic-composed = Картинок собрано: { $n }, в слот { $slot }

sstv-clear-picks = Сбросить выбор

sstv-composed-saved = сохранено

sstv-send = Отправить

sstv-send-now-hint = Передать эту композицию сейчас

sstv-composed-sending = отправка

sstv-forget = Забыть

sstv-forget-hint = Очистить документ этого пресета — баннерная полоса и простая подпись снова управляют его картинкой

sstv-forget-done = очищено — баннер и подпись снова на месте

sstv-copy-to-slot = Копировать в слот

sstv-copy-to-slot-hint = Дать другому слоту эту картинку и этот текст, чтобы отправлять оттуда

sstv-copied-into-slot = скопировано в слот { $n }

sstv-word-strip = полоса

sstv-word-text = текст

sstv-text-size = Размер текста

sstv-text-size-hint = Текст баннера в процентах от размера самой полосы. 100% — как всегда; для QSO нужен читаемый позывной после шумного декодирования, и это масштабирует надпись, не делая полосу выше.

sstv-placement-hint = Вдоль какого края рисуется полоса. Наложение сообщения начинается с противоположного конца, поэтому баннер внизу оставляет верх картинки для текста.

sstv-top = Верх

sstv-bottom = Низ

topbar-ptt-hold-hint = Удерживайте для передачи — щелчок фиксирует её, а следующее нажатие отпускает


topbar-above-the-dial = выше

topbar-below-the-dial = ниже

topbar-step-chip-hint = На сколько сдвигает настройку одно нажатие. Нажмите, чтобы взять следующий шаг: 10 Гц, 100 Гц, 500 Гц, 1, 2,5, 5, 6,25, 9, 10, 12,5 и 25 кГц. Строку можно отключить в Настройки › UI.

topbar-mem-chip-hint = Каналы памяти — вызвать сохранённую частоту

topbar-disp-menu-hint = Панадаптер — его два слоя, их скорости и детализация, удержание пиков — а также уровни водопада, параметры вида и скиммеры

topbar-phone-menu-hint = Всё, чего не показывает полоса телефона: диапазоны, режимы, приёмник и передатчик, окна декодирования, настройки и руководство

topbar-group-band = Диапазон

topbar-group-system = Система

topbar-caption-diversity = Разнесение

topbar-group-decode-windows = Окна декодирования

topbar-group-extras = Дополнения

topbar-grid-tracker-hint = Grid tracker — отработанные квадраты Maidenhead на карте

topbar-settings-chip-hint = Настройки — усиления устройства, антенны, аудиоустройства

topbar-windows-chip-hint =
    ▣  Левая: вернуть все окна модулей домой
    Правая: список окон модулей

topbar-manual-chip-hint = Руководство пользователя (F1)

topbar-module-windows-title = Окна модулей

topbar-wname-panadapter = SP1 — спектр

topbar-wname-aux = SP2 (AUX) — второй спектр

topbar-wname-panel = Рабочая панель

topbar-wname-controls = Управление RX

topbar-wname-band-menu = Диапазон и режим

topbar-wname-rotor = Поворотное устройство антенны

topbar-wstate-put-back = Вернуть { $name } в это окно

topbar-wstate-floating-panel-only = Плавающее состояние — только у рабочей панели; у спектра или консоли в этом окне одно место

topbar-wstate-float = Вынуть { $name } из области и сделать плавающим над этим окном

topbar-wstate-no-panel = В режиме { $mode } нет рабочей панели — это применимо в режиме, где она есть

topbar-wstate-own-window = Дать { $name } отдельное окно на любом мониторе

topbar-wstate-here-now = { $name } сейчас здесь

topbar-dock-all = ПРИСТЫКОВАТЬ ВСЕ

topbar-dock-all-hint = Вернуть все окна модулей в это окно

topbar-undock-all = ОТСТЫКОВАТЬ ВСЕ

topbar-undock-all-hint = Поместить каждый модуль в отдельное окно — это окно останется со спектром и декодерами

topbar-rx-tx-freq = RX { $rx }   TX { $tx } МГц

topbar-band-dock-hint = DOCK — держать выбор диапазона и режима открытым рядом с водопадом, а не закрывать это всплывающее окно каждый раз

topbar-band-window-hint = WINDOW — открыть выбор диапазона и режима в отдельном окне любой ширины на любом мониторе рядом с водопадом

topbar-band-return-popup-hint = DOCK — вернуть выбор во всплывающее окно верхней панели, где оно закрывается после каждого выбора

topbar-band-window2-hint = WINDOW — открыть выбор в отдельном окне любой ширины на любом мониторе

topbar-squelch-held-hint = Шумоподавитель удерживается открытым. Нажмите, чтобы вернуть прежний порог.

topbar-squelch-hold-hint = Нажмите, чтобы удерживать шумоподавитель открытым — чтобы услышать, есть ли там что-то, — и нажмите снова, чтобы вернуть всё точно как было.

topbar-changed-tone = тембр

topbar-changed-loudness = ТЕМБРОКОМПЕНСАЦИЯ

topbar-nr-chip-hint =
    Шумоподавление: { $strength } — { $engine }

    { $advice }

    Нажмите, чтобы переключить силу: Low / Med / High / Off. Движок выбирается в строке ниже этой.

topbar-nr-off-narrow-hint = Шумоподавление выключено. В строке ниже предлагаются пять шумоподавителей — RNNoise, DeepFilterNet3, spectral-bleach, NR2 и исходный спектральный — и нажатие здесь включает его с выбранной силой. Для голоса начните с RNNoise.

topbar-eq-tone-hover = Тембр: низкие { $bass } дБ, средние { $mid } дБ, высокие { $treble } дБ

topbar-eq-closed-hover = Тембр — трёхполосный эквалайзер принимаемого звука. Нажмите, чтобы открыть

topbar-eq-loudness-on = ТЕМБРОКОМПЕНСАЦИЯ вкл.

topbar-stereo-yields-hint = Стерео WFM отключено, пока работают шумоподавление или автоматический режекторный фильтр — они размывали бы два канала. Выключите их, чтобы слышать стерео

topbar-stereo-playback-hint = Стерео: запись воспроизводится в оба уха, как записано. Нажмите, чтобы принудительно включить моно

topbar-stereo-mono-recording-hint = Эта запись моно — второго канала для воспроизведения нет

topbar-preset-edges = { $lo } … { $hi } Гц

topbar-caption-tone = Тембр

topbar-eq-on = вкл.

topbar-eq-bass = Низкие

topbar-eq-mid = Средние

topbar-eq-treble = Высокие

topbar-eq-remembered = Запомнено для { $mode } — у каждого режима своё.

topbar-loudness-hint = Поднимает низкие (и немного высокие) при уменьшении громкости, чтобы тихий динамик звучал полно. Работает с тембром или без него; на полной громкости ничего не делает и никогда не играет громче, чем играла бы полная громкость. По умолчанию выключено и запоминается для текущего режима.

topbar-caption-record-audio = Запись звука

topbar-stop-recording-after-min = Остановить запись через { $minutes } мин.

topbar-caption-play-back = Воспроизведение

topbar-recordings-chip = ▶ ЗАПИСИ…

topbar-recordings-hint = Воспроизвести запись обратно в декодеры — пропущенную картинку SSTV, слабый сигнал RTTY или CW, слот FT8 — с водопадом и журналом декодирования, как будто это в эфире. Пока она играет, радио отключается.

topbar-ant-label = ANT

topbar-pwr-label = PWR

topbar-drive-less = меньше

topbar-drive-more = больше

topbar-fill-hint = Залить цветом область под линией спектра, полным внизу и затухающим по мере подъёма к линии, чтобы сигнал выделялся массой, а не только контуром. Цвет выбирается в Настройки → UI → Линия спектра. На 3D-поверхности не рисуется.

topbar-group-spectrum = Спектр

topbar-group-waterfall = Водопад

topbar-freeze-on-tx-hint = Останавливать прокрутку водопада на время вашей собственной передачи и продолжать с того места, где остановились — чтобы картинка была непрерывной историей приёма, а не блоком строк с вашей несущей (или, на радио, которое глушит приёмник при передаче, строк шума, которого не было). Линия спектра не затрагивается. Полезно при работе в режиме, где вы часто передаёте, чтобы ваша передача не вытесняла с экрана передачу корреспондента.

topbar-fft-chip-hint = Уровни и контраст водопада, размер БПФ и направление прокрутки

topbar-caption-waterfall = Водопад

topbar-flatten-hint = Выровнять уровень шума по диапазону: участок тише соседей поднимается к их уровню, чтобы слабый сигнал был виден на загруженном диапазоне. Только поднимает — несущая никогда не опускается — и не более чем на 20 дБ. Трасса спектра выше остаётся ровно такой, как измерена.

topbar-log-swl-hover = Журнал приёма — услышанные станции с SINPO/SIO

topbar-programs-hint = Расписание вещания — что идёт, когда и где

topbar-listen-hint = Что есть в этой полосе, на карте — выберите станцию, настройтесь на неё, запишите, что услышали, и внесите в журнал

topbar-spots-hint = Живые споты — PSK Reporter (а вне режима SWL — DX-кластер, POTA и SOTA)

topbar-sigid-hint = Что на этой частоте? Справочник сигналов по частоте, виду излучения и ширине полосы

topbar-extras-hint = EXTRAS — DAB и другие приёмники в окне, которое можно отсоединить

topbar-fill-style-caption = линия

topbar-fill-style-hint = Как затеняется цвет под линией спектра. Сам цвет выбирается в Настройки → UI → Линия спектра.

topbar-fill-fixed = ФИКСИР.

topbar-fill-fixed-hint = Один цвет, одинаковый везде под линией

topbar-fill-gradient = ГРАДИЕНТ

topbar-fill-gradient-hint = Полный внизу спектра, затухающий к верху

topbar-fill-mixed = СМЕШАН.

topbar-fill-mixed-hint = Полный в нижней половине, затухающий в верхней

topbar-fill-switch-on = Включите FILL, чтобы задать это

topbar-nothing = ничего

topbar-olivia-unconfirmed = OLIVIA — приём работает, передача не проверена с другими программами. Этот декодер читает другие станции (подтверждено в эфире), так что вы можете принять QSO на Olivia. Наша собственная передача теперь проходит круговой тест через наш приёмник, но её не принимали fldigi или MultiPSK, и одно соглашение о знаке отличается от исходного кода fldigi — поэтому проверьте первую передачу, а не рассчитывайте на ответ.

topbar-dab-bandwidth = DAB / DAB+, проверено в эфире. Нужно 2,5 Msps полосы на входе, прежде чем появится звук: при 1,536 МГц, которые занимает ансамбль, приёмник на пределе своего АЦП и теряет отсчёты, а для декодирования DAB нужен непрерывный поток, поэтому ансамбль не захватывается и вы ничего не слышите. Сначала установите частоту дискретизации радио 2,500 Msps.

topbar-stanag-unproven = STANAG 4285 — только приём. Опознавание проверено в эфире: 4285 захватывается, измеряется его кадр 106,67 мс, а опорные блоки стягиваются, поэтому панель может сказать, что на частоте и с какой скоростью. Чтение его *текста* в эфире не проверено: код и перемежитель Annex E точно декодируют сгенерированный сигнал, но ни одна кодированная станция пока не принята. Появившийся кодированный текст реален; отсутствие текста означает, что станция некодированная или передаёт шифрованный трафик, что обычно и бывает.

topbar-cis11-unproven = CIS-11 (TORG-11) — только приём. Российский метеорологический телетайп: 2-FSK на 100 бод, около 500 Гц сдвига и 11-битный символ с битом нечётности. Проверено на реальных записях: приёмник сам находит свои тоны и читает холостую передачу с 121 из 123 символов, верных по чётности. Не решено, как десять битов данных делятся на символы, поэтому панель показывает и биты, и своё чтение по ITA-2 — а сам трафик зашифрован, что обычно и бывает.

topbar-thor-unproven = THOR — принимающая сторона пока не читает передачи fldigi. Декодер работает со своим модулятором, но ни одна эфирная или эталонная запись не принята, поэтому относитесь к появляющемуся как к предварительному.

topbar-mode-not-on-band = { $mode } не используется на { $band } — выберите диапазон, которому он принадлежит

topbar-radio-classic-hint = RADIO CLASSIC — классический вид радио для вещательного прослушивания. Его включение отдаёт ему приёмник: остальные режимы и их окна закрываются

topbar-ats-bands-caption = Диапазоны приёмника

topbar-ats-band-hover = { $name } — { $mhz } МГц, { $mode } по умолчанию. Переводит приёмник на этот диапазон.

topbar-caption-mode = Режим

topbar-caption-digital = Цифровые

topbar-ats-receive-only = Только приём — Si4732 демодулирует аппаратно, и манипулировать нечем.

topbar-band-forecast-derived =
    { $band }: ≈{ $verdict } ({ $time ->
            [day] день
           *[night] ночь
        }) — опубликованная оценка группы { $group }, ближайший заменитель; HAMQSL.com не оценивает сам { $band }. Прогноз, а не измерение вашей собственной трассы.

topbar-keypad-readout-khz = { $digits } кГц

topbar-caption-band-keypad = Клавиатура диапазонов

topbar-keypad-bands = Диапазоны

topbar-keypad-bands-hint = Клавиши называют диапазоны: нажмите одну, чтобы перейти туда, оставаясь в текущем режиме. Возврат из МГц очищает набранное.

topbar-keypad-mhz-hint = Наберите частоту на клавишах в килогерцах: 14074 — это 14,074 МГц. ENTER отправляет её, CLEAR очищает ввод.

topbar-keypad-key-stay = Остаться в { $mode } и настроиться на { $mhz } МГц — где с ним работают на { $band }.

topbar-keypad-key-go = Перейти на { $band }.

topbar-keypad-clear = Очистить

topbar-keypad-clear-typing-hint = Очистить набранную частоту. Клавиши остаются в МГц.

topbar-keypad-clear-bands-hint = Ничего не набрано — клавиши на диапазонах, где нажатие переходит на диапазон.

topbar-keypad-enter = Ввод

topbar-keypad-enter-empty-hint = Пока ничего не набрано. Клавиши в МГц — сначала нажмите их.

topbar-keypad-enter-ok-hint = Настроиться на { $mhz } МГц и вернуться к клавишам диапазонов.

topbar-keypad-enter-range-hint = { $mhz } МГц вне диапазона приёма этого радио ({ $ranges } МГц).

topbar-nothing-published = ничего не опубликовано

topbar-tab-listen = LISTEN

topbar-tab-operate = OPERATE

topbar-listen-tab-note = Эта вкладка может также скрывать элементы передачи: включите «скрывать TX на вкладке LISTEN» в Настройки → UI

topbar-caption-show-bands = Показывать диапазоны

topbar-band-filter-hint = Показывать в списке ниже только диапазоны { $filter }. Нажмите снова, чтобы показать все диапазоны.

topbar-show-every-band = показать все диапазоны

topbar-all-chip = ALL

topbar-all-chip-hint = Общее покрытие: снять диапазон и позволить настройке уходить куда угодно. Это не то же самое, что фильтр диапазонов, который лишь сужает этот список.

topbar-caption-sw-metre-bands = Метровые диапазоны KB

topbar-metre-band-hint = Настроиться на середину вещания { $name }

topbar-caption-wideband = Широкополосный

topbar-caption-cb = 11M/CB

topbar-caption-experimental = Экспериментальные (не проверены)

topbar-caption-receive-modes = Режимы приёма

topbar-wav-hint = То же аудио как несжатый 16-бит PCM на 48 кГц — около 11 МБ в минуту в стерео. Для редактора или вашего собственного декодера.

topbar-mode-offset-hint-side = Этот режим работает с сигналом { $side } настройки. Число — это настройка, потому что вы настраиваетесь по ней; сигнал находится на { $mhz } МГц.

levelslider-hover =
    Уровень водопада
    нижний { $floor } дБ · верхний { $ceil } дБ

    Перетащите или прокрутите, чтобы сделать ярче или темнее; shift — для контраста.
    Перемещение отключает автоподгонку (FIT).

smeter-audio-level-hover =
    Уровень звука: { $dbfs } dBFS{ $over }.

    У этого радио **нет собственного S-метра для отчёта**, поэтому это уровень звука, который оно отдаёт, — измеренный здесь, с уже применёнными АРУ и шумоподавителем самого трансивера. Он следует за звуком, а не за ВЧ: шум звуковой карты, беспроводная мышь рядом с трансивером или накачка АРУ сдвинут его, хотя сигнал не изменился.

smeter-audio-over = (S9 + { $over } дБ по шкале звука)

spectrum-offset-left = ◀ тоны { $hz } Гц

spectrum-offset-right = { $hz } Гц тоны ▶

worldmap-no-snr = нет SNR

solar-tune-the-radio-to-this-downlink = Настроить радио на эту линию вниз

qo100-width-label = ширина

qo100-park-label = парковка

qo100-led-carrier = несущая

qo100-led-sync = синхр.

scanner-stopped = остановлен

ism-band-label = диапазон

amp-none = нет

colormap-classic = Классическая

colormap-viridis = Viridis

colormap-gray = Серая

colormap-icom = Icom

colormap-neon = Неон

colormap-synthwave = Synthwave

colormap-matrix = Matrix

colormap-tron = Tron

colormap-amber = Янтарная

colormap-rainbow = Радуга

colormap-blue = Синяя

colormap-calmer = Спокойная

colormap-compline = Compline

colormap-lauds = Lauds

settings-cb-wide-calls-hint =
    Принимать более широкую форму позывного 11 м { $wide } — три буквы в середине, до четырёх цифр после — наряду с { $narrow } из WSJT-CB. Экспериментально: сам WSJT-CB отвергает более широкую форму, поэтому позывной, которому она нужна, может быть не понят станцией WSJT-CB; это расширяет то, что эта станция слышит, больше, чем то, что она может надёжно передать. В выключенном состоянии действует точная грамматика WSJT-CB.

    Этот переключатель меняет *форму*, а не *длину*: позывной 11 м по-прежнему состоит из 11 символов, включая любой суффикс /P, /MM или /QRP, так что 19DCG3733/P подходит, а 19TST1001/QRP — нет.

settings-idle-stop-disable = ОТКЛЮЧИТЬ

settings-xvtr-tx-off-hint =
    Выкл. при конвертации: только приём — строка является конвертером, а не трансвертером, и на этом диапазоне ничего не манипулируется.

    Через тот же конвертер: работает в обе стороны, и передача использует тот же сдвиг.

settings-xvtr-drive-hint =
    Предел мощности раскачки при передаче, пока выбран этот трансвертер, в процентах от полной. ПЧ-вход трансвертера принимает милливатты, а раскачка, подходящая для собственных диапазонов радио, его разрушит, поэтому это самое важное поле строки. 100% — без ограничения.

    Ваша настройка Drive удерживается ниже этого значения, а не изменяется, поэтому число, которым вы пользуетесь на КВ, возвращается, когда настройка покидает диапазон.

settings-brown-build-note = Сборка Brown sdroxide: программа оригинала плюс гражданский диапазон 11 м, коротковолновое прослушивание и декодирование. Работает как обычная любительская станция с добавленными диапазонами и инструментами.

settings-gain-by-band = Запоминать усиление входного каскада для каждого диапазона

settings-gain-by-band-hint =
    Хранить отдельное усиление приёмника для каждого диапазона и восстанавливать его, когда настройка переходит в этот диапазон. Задайте усиление для КВ один раз и для 11 м или УКВ один раз; радио следует за диапазоном.

    По умолчанию выключено. Пока выключено, одно усиление действует везде, как и раньше. Включение запоминает диапазон, на котором вы сейчас, чтобы ни один диапазон не остался без значения; диапазон, для которого вы ничего не задавали, сохраняет текущее усиление.

settings-callsign-grid-note = Ваш позывной и локатор, общие для FT8/FT4/FT2, заголовков изображений SSTV, цифровых режимов CB и журнала. Это идентификатор, который **передаёт** — позывной CB, если вы работаете на 11 м, иначе любительский позывной. Чем подписывается **рапорт о приёме**, смотрите в поле **Report as (SWL)** на вкладке Spots. Также редактируется в диалоге настройки FT8 / SSTV.

settings-cb-grammar = Грамматика позывных 11 м / WSJT-CB

settings-cb-wide-calls = ШИРОКИЕ ПОЗЫВНЫЕ CB — ЭКСПЕРИМЕНТАЛЬНО

settings-cb-wide-calls-note = Включите, если ваше сообщество на 11 м использует трёхбуквенные группы или четырёхзначные номера единиц. Применяется к декодированию и передаче цифровых режимов CB. В любом случае позывной 11 м не может превышать **11 символов**, включая суффикс — 19DC3733/P подходит, 19TST1001/QRP — нет.

settings-cb-plan-hint = В каком канальном плане читается настройка: планы гражданского диапазона 11 м и планы PMR446 на 446 МГц. Меняются только номера каналов и канал, на котором открывается диапазон, — границы диапазона остаются на месте, поэтому переключение никогда не сдвигает то, что принимает или передаёт. Номер канала рисуется на линии настройки везде, где настройка находится на одном из каналов плана.

settings-cb-allow-tx = Разрешить передачу на 11 м (CB)

settings-cb-allow-tx-hint = Блокировка любительских диапазонов отказывает в передаче на каждом диапазоне, не являющемся любительским распределением, а 11 м — таков: это отдельная радиослужба со своими правилами и своим сертифицированным оборудованием. Включение разрешает передачу там — и больше нигде; вещательные диапазоны остаются только на приём. При первом включении вас попросят подтвердить.

settings-cb-licence-note = CB не требует лицензии во многих странах, но не во всех, а разрешённые каналы, виды излучения и мощность различаются. Использование диапазона подчиняется правилам страны, в которой вы находитесь.

settings-radio-callsign-hint =
    Позывной, под которым это радио передаёт, заносит в журнал и даёт споты. Оставьте пустым, чтобы использовать позывной станции со вкладки «Общие» — по умолчанию и подходит станции с одной личностью.

    Задавайте на радио, которое принадлежит кому-то другому: позывной CB на радио 11 м, любительский позывной на КВ-трансивере, и каждый сохраняет свой. Позывной со вкладки «Общие» остаётся запасным для каждого радио, которое его не переопределяет.

    Рапорты о *приёме* по-прежнему могут подписываться номером SWL со вкладки Spots, который это не меняет.

    Вступает в силу после «Применить».

settings-radio-swap-iq-own = У этого интерфейса есть собственная инверсия I/Q в его разделе ниже на этой странице — используйте её.

settings-radio-swap-iq-hint =
    Инвертировать I/Q этого приёмника перед демодуляцией. Используйте, когда I и Q приёмника поменяны местами и SSB выходит не на той боковой полосе (LSB слышен как USB), а спектр зеркален относительно настройки.

    Свойство оборудования, а не того, кто вошёл.

    Вступает в силу после «Применить».

settings-radio-transmit-controls = Элементы передачи

settings-radio-hide-tx-all = скрыть все (режим SWL)

settings-radio-hide-tx-hint =
    Скрыть все элементы передачи для *этого* радио — PTT, CALL CQ, уровень TX, SEND, BEACON, всё — и заменить любительские дополнения полосы (споты, награды) на слушательские (SCHEDULE и SWL LOG, журнал приёма). Для каждого радио отдельно: приёмный донгл на длинном проводе может находиться в этом режиме, пока трансивер рядом сохраняет свой передатчик.

    Оборудование по-прежнему может передавать; это лишь скрывает элементы управления. `--swl` принудительно включает его для каждого радио на время запуска.

    Вступает в силу после «Применить».

settings-radio-auto-mode = Авто режим

settings-radio-auto-stop-label = остановка без ввода

settings-radio-auto-stop-hint-long =
    Как долго автономный секвенсор FT8/FT4/FT2 может работать без действий оператора, прежде чем отключится. Для каждого радио отдельно: один комплект можно оставить работать дольше другого.

    На PlutoSDR значение доходит до шести часов, а DISABLE отключает остановку совсем: авто режим тогда работает, пока вы его не остановите. Сторожевой таймер передачи и блокировка диапазона по-прежнему действуют.

    Вступает в силу после «Применить».

settings-radio-auto-stop-hint =
    Как долго автономный секвенсор FT8/FT4/FT2 может работать без действий оператора, прежде чем отключится. Для каждого радио отдельно: один комплект можно оставить работать дольше другого.

    Ограничено 45 минутами намеренно — авто режим рассчитан на перерыв на пару минут, а не на то, чтобы оставить станцию работать в контесте без человека за ней.

    Вступает в силу после «Применить».

settings-radio-smeter-calib = Калибровка S-метра

settings-tx-eq = Эквалайзер передачи

settings-tx-eq-hint = 3-полосный параметрический эквалайзер звука микрофона перед модулятором. Только голосовые режимы (SSB/AM/FM); цифровые режимы и CW несут синтезированный или манипулируемый звук, который до него не доходит. По умолчанию выключен и плоский при включении, поэтому включение ничего не меняет в эфире, пока полоса ниже действительно не сдвинута. Применяется сразу.

settings-tx-eq-low-hint = Срезает/поднимает гул и шум от прикосновений.

settings-tx-eq-mid = Пик средних

settings-tx-eq-mid-hint = Присутствие: узкий подъём или провал где-то в голосовой полосе.

settings-tx-eq-high-hint = Яркость/де-эссинг.

settings-cb-call-too-long = ⚠ Слишком длинный для 11 м — { $len } символов, а FT8 несёт 11 для позывного CB *включая* любой суффикс /P, /MM или /QRP. Этот позывной активации передать нельзя. 19DC3733/P (10) подходит; 19TST1001/QRP (13) — нет.

settings-cb-plan-note = 11 м (CB) — не один и тот же диапазон везде. Здесь выбирается, каналы какой страны читает настройка 11 м: { $plan }. Только каналы и канал, на котором открывается диапазон, — границы диапазона оставлены широкими, поэтому переключение планов никогда не меняет то, что принимает или передаёт. Применяется ко всем радио этой станции.

settings-cw-enter-mode = Войдите в CW (или любой цифровой режим) один раз, чтобы загрузить сохранённые значения.

settings-cw-where-from = Откуда:

settings-cw-straight-key-hint = Привязка `CW straight key` (по умолчанию Пробел) читается, пока включён KEY на панели CW. Один контакт, поэтому это прямой ключ — приложение декодирует собственный темп оператора.

settings-cw-speed = Скорость:

settings-cw-speed-note = длина точки и тире электронного ключа

settings-cw-key-tx = МАНИПУЛИРОВАТЬ ПЕРЕДАТЧИКОМ

settings-cw-key-tx-hint = Выкл.: ключ управляет только локальным тренажёром. Вкл.: он манипулирует радио через обычный путь ручного ключа, с блокировкой диапазона и сторожевым таймером вокруг.

settings-cw-key-self-note = Для трансивера, который манипулируется сам, установите Настройки → Радио → CW keying в `Sound card (MCW)`: ручная манипуляция на маршруте ключа трансивера отклоняется по замыслу.

settings-detachable-windows = Отсоединяемые окна

settings-detachable-windows-note = Вытянуть панель в отдельное окно ОС. Закрытие окна пристыковывает её обратно.

settings-tool-windows = Окна инструментов

settings-tool-windows-note = Панели, преобразованные в плавающие окна. Их состояние открытия запоминается.

settings-tool-window-scanner = СКАНЕР

settings-tool-window-memories = ПАМЯТЬ

settings-tool-window-playback = ВОСПРОИЗВЕДЕНИЕ

settings-tool-window-open = Открыто

settings-tool-window-closed = Закрыто

settings-detachable-native-only = Отсоединяемые окна — только в нативной версии.

settings-psk-reporter-band-hint = Лента активности диапазона: станции, слышимые на текущем диапазоне, рисуются цветными точками. Также включает запрос «кто меня слышал» за чипом HEARD ME на карте — тоже обновляется ежечасно.

settings-psk-upload-hint = Сообщать pskreporter.info, что слышит эта станция, чтобы она появилась там как приёмник. Используются позывной и локатор со вкладки «Общие» или номер SWL ниже, если он задан.

settings-swl-report-as = Report as (SWL)

settings-swl-report-as-hint = Собственный идентификатор слушателя для рапортов о приёме — зарегистрированный номер SWL, номер клуба, имя. Когда задан, он подписывает то, что эта станция *слышит* (выгрузки в PSK Reporter и WSPRnet и рапорт о приёме, который копирует журнал SWL), вместо позывного. Оставьте пустым, чтобы рапортовать как позывной. Он никогда не передаётся, никогда не манипулирует радио и никогда не называет QSO или спотера, поэтому слушатель без позывного тоже может рапортовать.

settings-wsjtcb-spot-server = Сервер спотов WSJT-CB

settings-wsjtcb-report = Сообщать мои декодирования на 11 м на сервер спотов WSJT-CB

settings-wsjtcb-report-hint = Отправлять каждый спот, декодированный на диапазоне 11 м, в службу спотов сообщества CB (xzgroup.net) — на тот же сервер, на который может отправлять собственный клиент WSJT-CB. По умолчанию выключено: это сторонняя служба, и ничего не отправляется, пока вы не попросите. Спотером является ваш позывной; PSK Reporter — отдельный переключатель выше.

settings-eqsl-qth-hint = В какой профиль QTH заносится выгрузка — нужно, только если в учётной записи больше одного, иначе оставьте пустым.

settings-log11dx-api-url = URL API

settings-log11dx-api-token = Токен API

settings-log11dx-note = Оба берутся со страницы вашего профиля LOG11DX. Его собственный мост WSJT-X не нужен: sdroxide заносит QSO в журнал и отправляет его прямо туда. Сайт маскирует токен, поэтому скопируйте его при создании — он хранится в net.json, как и остальные учётные данные.

settings-pan-attached-note =
    Его спектр, его водопад и всё, считываемое с них, находятся на вкладке того радио, поэтому у него нет своих. Настройки интерфейса выше по-прежнему принадлежат этому радио и по-прежнему действуют — нажмите Применить / переподключить после их изменения, и оба радио поднимутся вместе.

    Чтобы снова использовать его самостоятельно, очистите поле «Приёмник панадаптера» на странице того радио.

settings-pan-source-hint =
    Какое радио принимает для этого. Оно уходит из полосы вкладок, пока одолжено — его входной каскад принадлежит движку этого радио, — и сразу возвращается, когда здесь установлено «Нет».

    Вступает в силу после «Применить».

settings-pan-offset-hint =
    Насколько выше настройки этого радио находится приёмник, в герцах. 0 для приёмника на той же антенне; промежуточная частота для отвода ПЧ — 9000000 для ПЧ 9 МГц, 70455000 для 70,455 МГц.

    Это не поле «Конвертер» выше, которое перестраивает радио, и не смещение центра I/Q на вкладке CAT, относящееся к собственной звуковой карте этого радио. Ничто из введённого здесь никогда не отправляется ни на одно из радио.

    Вступает в силу после «Применить».

settings-cw-device = Устройство:

settings-cw-device-none = (не найдено)

settings-cw-device-auto = авто (первое с именем «key»)

settings-cw-reverse-hint = Поменять точку и тире местами, для переключателя на самом манипуляторе.

settings-cw-key-type = Тип ключа:

settings-cw-key-straight = ПРЯМОЙ

settings-cw-key-iambic-a = IAMBIC A

settings-cw-key-iambic-b = IAMBIC B

settings-cw-usb-note = Блок ключа, сообщающий о двух своих контактах (часто как кнопки мыши), работает; тот, что сам делает ямбику и отправляет готовые касания, — нет. Пока ключ активен, устройство захвачено монопольно, поэтому его контакты не могут щёлкать в других окнах.

settings-cw-usb-linux = USB-манипулятору нужен рабочий стол Linux.

settings-smeter-zero = 0 (как измерено)

settings-smeter-cal-hint =
    Корректирует S-метр этого радио, когда он показывает слишком много или слишком мало, в S-единицах по 6 дБ.

    Отрицательное значение уменьшает показание — для метра, который показывает слишком много, шум на S5, где должно быть S2. Положительное увеличивает. 0 — метр как измерено.

    Сдвигается только показание: стрелка, значения S и дБм, уровень, с которого начинается рапорт о приёме, и озвученный S-метр. Сигнал, шумоподавитель, запись и декодеры не затрагиваются.

    Сохраняется вместе с этим радио на компьютере, к которому оно подключено, и вступает в силу сразу.

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1 и 2 оба используют { $host }:{ $port } — каждый спот будет приходить дважды. Задайте им разные порты (7000 CW/RTTY, 7001 FT8/FT4) или выключите один.

settings-rx-site-station = На станции

settings-rx-site-elsewhere = В другом месте

settings-remote-tab-note =
    Станция появляется вкладкой рядом с уже открытыми здесь радио, а полоса вкладок вверху окна переключает между ними (⊞ ставит два рядом). Закройте её из списка вверху вкладки Радио — это завершает соединение; на сервере ничего не изменяется.

    Если сервер запрашивает имя пользователя и пароль, его экран входа появится в новой вкладке. Обычная ссылка ws:// ничего из этого не шифрует, поэтому в открытом интернете либо пропустите её через VPN или SSH-туннель, либо терминируйте HTTPS перед сервером и отметьте Secure выше.

settings-cw-source-keyboard = КЛАВИАТУРА

settings-cw-source-usb = USB-МАНИПУЛЯТОР

pane-frame = КАДР

pane-frames = КАДРЫ

pane-words = СЛОВА

antenna-feed-z0 = Z0 фидера:

antenna-feed-z0-hint = Волновое сопротивление фидера/системы для КСВ (обычно 50 Ом)

antenna-height-label = Высота:

antenna-height-hint = Высота точки питания/антенны над землёй (влияет на угол излучения/диаграмму)

antenna-m-agl = м над землёй

antenna-apply = Применить

antenna-apply-hint = Запустить решатель методом моментов на пресетах.

antenna-yagi-optimizer = Оптимизатор Яги

antenna-stack = Этажей:

antenna-objective = Цель:

antenna-optimize = Оптимизировать

antenna-swr-value = КСВ { $v }

antenna-swr-ref = КСВ { $v } ({ $z }Ω)

antenna-radiation-pattern = Диаграмма направленности

antenna-view = Вид

antenna-view-front = Спереди

antenna-view-side = Сбоку

antenna-view-top = Сверху

antenna-3d-hint = 3D: перетаскивание вращает • цвет = напряжённость поля

antenna-yagi-x = Яги ×{ $n }

settings-tool-window-session-qsos = QSO СЕАНСА

sessionqso-time = Время

sessionqso-call = Позывной

sessionqso-grid = Локатор

sessionqso-band = Диапазон

sessionqso-mode = Режим

sessionqso-rst-s = RST перед.

sessionqso-rst-r = RST прин.

sessionqso-country = Страна

sessionqso-dist = Расст.

sessionqso-final = Итог

sessionqso-count = { $n } QSO за этот сеанс

sessionqso-include-cw = Включая CW

decodes-live-log = ЖИВОЙ ЖУРНАЛ

decodes-setup = ⚙ НАСТРОЙКА

schedule-colour-all = Цвет

schedule-colour-all-hint = Цвет подписи на водопаде для всех станций — заменяет цвета, выбранные для отдельных станций

schedule-colour-station-hint = Цвет подписи этой станции на водопаде

schedule-colour-reset = ПО УМОЛЧАНИЮ

schedule-colour-reset-hint = Вернуть стандартный цвет для всех станций, сбросив выбранные

settings-language = Язык

settings-language-note = Язык, на котором отображается интерфейс. «Системный» следует языку операционной системы; выберите другой, чтобы переопределить его. Выбор сохраняется и запоминается.

aprs-all-chip = ВСЕ

aprs-all-chip-hint = Включено: все сообщения, услышанные на канале, кому бы они ни предназначались. Выключено: только адресованные вам или от вас, а также бюллетени.

aprs-composer-hidden-swl = Отправка скрыта: включён режим SWL / прослушивания. Выключите его, чтобы писать и отправлять сообщения.

aprs-compose-count-hint = Использовано символов из 67, которые может вместить сообщение APRS.

tx-bands-title = Диапазоны передачи

tx-bands-note = Переключайте TX на ДА только для диапазонов, на которых этот трансивер действительно передаёт. Эта настройка стоит перед всеми остальными настройками передачи; ДА не снимает остальные ограничения безопасности.

tx-bands-careful = ⚠ Неверный диапазон может вывести трансивер из строя: внимательно проверьте каждый пункт.

tx-bands-yes = ДА

tx-bands-no = НЕТ

tx-bands-confirmed = ✓ Диапазоны передачи для этого трансивера подтверждены

tx-bands-ack = Я проверил эти диапазоны по своему трансиверу

tx-bands-confirm = Подтвердить эти диапазоны

tx-bands-confirm-hint = Сначала поставьте галочку: подтверждение выполняется один раз для каждого трансивера.

tx-bands-check = Проверить диапазоны…

tx-bands-notice-new = Передача выключена на всех диапазонах. Выберите диапазоны, на которых передаёт этот трансивер, и подтвердите их один раз. Из-за риска для трансивера внимательно проверьте выбранные пункты.

tx-bands-notice-carried = Ваши диапазоны передачи перенесены из предыдущей версии. Передача остаётся заблокированной, пока вы не проверите и не подтвердите их один раз. Из-за риска для трансивера внимательно проверьте каждый пункт.


adsbmap-surface-asphalt = асфальт

adsbmap-surface-concrete = бетон

adsbmap-surface-grass = трава

adsbmap-surface-gravel = гравий

adsbmap-surface-dirt = грунт

adsbmap-surface-water = вода

adsbmap-airport-runway-lit = с огнями

adsbmap-airport-more = +ещё { $n }

adsbmap-airport-freq-head = Частоты, МГц

adsbmap-airport-data-date = данные о ВПП и частотах от { $date }
aircraft-online-switch = Онлайн

aircraft-online-switch-on-hover = Онлайн: тип, оператор, маршрут и фото берутся из adsbdb и Planespotters, фото аэропортов — из Wikipedia, а местное время и температура аэропорта — из Open-Meteo (передаётся только положение аэропорта); запросы выполняет ваша станция от вашего имени. Нажмите, чтобы выключить; станция запоминает ваш выбор. Больше ничего за пределы станции не уходит.

aircraft-online-switch-off-hover = Офлайн: ничего не покидает станцию. Нажмите, чтобы включить тип, оператора, маршрут и фото (adsbdb и Planespotters), фото аэропортов (Wikipedia) и погоду в аэропортах (Open-Meteo).

aircraft-switch-held = оператор станции удерживает его через SDROXIDE_AIRCRAFT_ONLINE

adsbmap-airport-photo-credit = Фото: Wikipedia

adsbmap-airport-local-time = Местное время { $time } ({ $zone })

adsbmap-airport-temperature = Температура { $temp }

adsbmap-airport-weather-credit = Погода: Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time } (местное время: загрузка…)

adsbmap-airport-utc-offline = UTC { $time } (местное время: офлайн)

schedule-listen = СЛУШАТЬ

schedule-listen-hint = Настроить приёмник на эту станцию

schedule-sort = Сортировка

schedule-sort-asc = По возрастанию

schedule-sort-desc = По убыванию

schedule-col-freq = Част.

schedule-col-band = Диапазон

schedule-col-name = Название

schedule-col-lang = Язык

schedule-col-target = Зона

schedule-col-site = Площадка

schedule-col-local = Местн.

tx-bands-off = Передача выключена на { $band }: этот диапазон не разрешён в ваших настройках, поэтому ваш передатчик не совместим с передачей на этом диапазоне. (Показывается не более { $max } раз за сеанс.)

tx-off-band = ПЕРЕДАЧА ЗАПРЕЩЕНА — { $freq } МГц ({ $band }) не является подтверждённым диапазоном передачи. Отметьте и подтвердите его в Настройки → Радио → Диапазоны передачи.

tx-off-unconfirmed = ПЕРЕДАЧА ЗАПРЕЩЕНА — диапазоны передачи этой радиостанции ещё не подтверждены. Проверьте и подтвердите их в Настройки → Радио → Диапазоны передачи.

aircraft-hover-lookup-retry = нет ответа — повтор через { $s } с

adsbmap-airport-navaids-head = Радионавигационные средства на аэродроме

adsbmap-airport-no-freq = Для этого аэропорта частоты не опубликованы

topbar-preset-right-click = Щёлкните правой кнопкой, чтобы изменить этот пресет

topbar-preset-edit = ИЗМЕНИТЬ ПРЕСЕТ

topbar-preset-label = Метка

topbar-preset-label-hint = Текст на кнопке. До 8 символов.

topbar-preset-lo = Низ

topbar-preset-hi = Верх

topbar-preset-use-current = Взять текущий

topbar-preset-use-current-hint = Взять границы фильтра, который сейчас стоит в приёмнике

topbar-preset-save = Сохранить

topbar-preset-reset = Сбросить

topbar-preset-reset-hint = Вернуть встроенный пресет этого режима

topbar-preset-cancel = Отмена
wefax-schedule = ГРАФИК
wefax-schedule-hint = Открывает в браузере расписание, которое публикует { $station }, — карты и время UTC.

tx-bands-check-again = ПРОВЕРИТЬ

tx-bands-check-again-hint = Отзывает подтверждение, чтобы список можно было проверить снова. Изменение любой полосы делает это само.
