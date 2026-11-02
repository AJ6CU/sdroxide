# sdroxide — the interface's English text, the reference every translation
# is checked against. See crates/sdroxide-ui/src/i18n.rs for the rules.
#
# Ids are grouped by where they are shown, and named after it:
# settings-ui-* is the UI tab of the Settings window, and so on. A message
# shown in capitals is written in capitals here; the code does not upper-case
# translated text.

## The language's own name, as the language picker lists it.
language-name = 日本語

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = 自動
layout-mode-desktop = デスクトップ
layout-mode-tablet = タブレット
layout-mode-small = 小型画面
layout-mode-phone = スマートフォン

ui-theme-default = 標準
ui-theme-light = ライト
ui-theme-high-contrast = ハイコントラスト
ui-theme-green-phosphor = グリーン蛍光
ui-theme-amber-phosphor = アンバー蛍光
ui-theme-teal-orange = ティール / オレンジ
ui-theme-rainbow = レインボー
ui-theme-nord = Nord
ui-theme-nord-dark = Nord ダーク
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized ダーク
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin モカ
ui-theme-catppuccin-latte = Catppuccin ラテ
ui-theme-modern-minimalist = モダン・ミニマリスト

chrome-style-angled = 角張り
chrome-style-rectangular = 長方形
chrome-style-rounded = 角丸
chrome-style-gradient = グラデーション
chrome-style-bevel = 3D ベベル
chrome-style-terminal = ターミナル

font-size-small = 小
font-size-medium = 中
font-size-large = 大

bandplan-kind-ham = アマチュア
bandplan-kind-cw = CW
bandplan-kind-digi = デジタル
bandplan-kind-phone = 音声
bandplan-kind-beacon = ビーコン
bandplan-kind-broadcast = 放送
bandplan-kind-am = AM / LW / MW
bandplan-kind-cb = CB

speech-verbosity-terse = 簡潔
speech-verbosity-normal = 標準
speech-verbosity-full = 詳細

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = ダイヤル（14 point 074）
speech-freq-style-digits = 一桁ずつ
speech-freq-style-full = 単位付きダイヤル

speech-callsign-style-phonetic = フォネティック（kilo one alpha）
speech-callsign-style-letters = 文字（kay one ay）
speech-callsign-style-as-is = 書かれたとおり

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = 表示
settings-ui-moved-note = パナダプターの詳細、スペクトラムの反応、ウォーターフォールのスクロール速度は、表示帯の SPEC ボタンで設定します（狭いウィンドウでは DISP メニュー）。
settings-ui-language = 言語
    .hint = インターフェースの言語です。ここで何を選んでも、F1 のマニュアルと音声アナウンスは英語のままです。
settings-ui-layout = レイアウト
settings-ui-theme = テーマ
settings-ui-button-style = ボタンのスタイル
settings-ui-frame-rate = 画面の更新頻度
settings-ui-palette = ウォーターフォールのパレット
settings-ui-tuning-buttons = 同調ボタン
settings-ui-tuning-buttons-show = スマートフォンとタブレットで表示
    .hint = タッチ画面で、コントロール帯の下に並ぶ「下げる / ステップ / 上げる」ボタンの列です。数字をスクロールするホイールもダイヤルもないため、これがないと既知のステップで動かすには周波数全体を入力するしかありません。デスクトップでは表示されません。
settings-ui-tuning-step = ステップ: { $step }
settings-ui-tuning-snap = 最初の押下でステップに合わせる
    .hint = ステップの倍数の間に止まったダイヤルは、押した方向の次の倍数に移動します。1 kHz ステップなら、7 074 300 は上へ 7 075 000、下へ 7 074 000 になります。その後はボタンが通常どおりステップ単位で動きます。オフの場合、押すたびに正確にステップ分だけ動きます。
settings-ui-smoothing = ウォーターフォールの平滑化
settings-ui-smoothing-interpolate = 補間
    .hint = 各画素を周囲のビンや行とブレンドするため、表示がFFTより広い場合に信号が連続して見えます。矩形のウォーターフォール（ビンごとに1ブロック、行ごとに1ブロック）が必要な場合はオフにしてください。画像から信号の形を読み取るにはそれが必要で、補間された信号は本当に広い信号と区別できなくなります。より大きな FFT（FFT チップ）の設定も、その半分を担います。
settings-ui-background = スペクトラムの背景
settings-ui-background-gradient = グラデーション
settings-ui-background-top = 上
settings-ui-background-bottom = 下
settings-ui-spot-colours = スポットラベルの色
    .hint = 各スポット元に付く色合いです。ウォーターフォール下部のボックス、スポット一覧のバッジ、世界地図上の点に使われます。
settings-ui-spot-colour = { $kind } スポットの色
settings-ui-spot-colours-reset = すべてのスポットの色を既定に戻す
settings-ui-reset = リセット
settings-ui-bandplan-colours = バンドプランの色
    .hint = ウォーターフォール下部のバンドプラン帯に、各割り当ての種類をどの色で塗るかです。ブロックはウォーターフォールの上に半透明で描かれるため、ここの見本より暗く表示されます。
settings-ui-bandplan-colour = { $kind } の割り当ての色
settings-ui-bandplan-colours-reset = すべてのバンドプランの色を既定に戻す
settings-ui-skimmer-font = スキマーの文字サイズ
settings-ui-waterfall-font = ウォーターフォール / スペクトラムの文字サイズ
settings-ui-interface-font = インターフェースの文字サイズ
    .hint = インターフェース全体を拡大縮小します。メニュー、ダイアログ、ウィンドウ、無線機のタブ、上部バーとそのボタンが対象です。上の 2 つのサイズは、この値に対する相対です。
settings-ui-cities = 地図上の都市
    .hint =
        世界の都市を平面地図（FT8/WSPR、APRS、ADS-B、AIS）に描画します。各地点に点を打ち、余地があれば横に名前を表示します。

        They are most of what says *where* a dot is on a map of the whole world, and they are also the busiest thing on it. Turn them off if the names are getting in the way of the stations you are reading. The 3D globe is unaffected — its cities are night-side lights rather than markers, and nothing there is written across a contact.
settings-ui-cities-show = 都市と名前を表示
settings-ui-3d-view = 3D ビュー
settings-ui-clouds = 雲の描画
settings-ui-clouds-layered = レイヤー
settings-ui-clouds-volumetric = ボリュメトリック
settings-ui-clouds-note = 3D ビューの CLOUDS レイヤーが気象をどう描くかです。レイヤーは対流圏の断面を積み重ねるもので、軽い方式です。ボリュメトリックは、その中を光線が進むため、太陽が雲頂を下の面に落とし、雷が嵐の外側を明るくするだけでなく、嵐の内部から光を放ちます。ピクセルあたりのコストは数倍になります。

## Settings window, UI tab: voice announcements.

speech-title = 音声アナウンス
speech-enabled = 無線機の変化を読み上げる
    .hint = 何が変わったかを読み上げるため、無線機を見ずに操作できます
speech-voice = 音声
speech-voice-shipped = 同梱の音声
speech-speed = 速度
    .hint = 音声は自身のフレーズを伸縮させるため、ピッチは変わりません。約 2× を超えると、それ以上短くはなりません。
speech-volume = 音量
speech-output = 出力
speech-detail = 詳しさ
speech-duck = 受信音を下げる
speech-duck-while = 読み上げ中
speech-test = テスト
speech-what = 読み上げる内容
speech-cat-frequency = 周波数
speech-cat-mode-band = モードとバンド
speech-cat-vfo-split = VFO とスプリット
speech-cat-agc-gain = AGC とゲイン
speech-cat-levels = ドライブ、チューン、マイク
speech-cat-ptt = 送信と受信
speech-cat-rit-xit = RIT と XIT
speech-cat-memory-scan = メモリーとスキャン
speech-cat-band-edge = アマチュアバンドの端を出る
speech-cat-notices = 警告とメッセージ
speech-cat-filters = フィルター、スケルチ、ノイズリダクション
    .hint = 既定ではオフです。信号を追っている間、これらは絶えず変化するためです
speech-decoded = デコードされたメッセージ
speech-ft8-to-me = 私宛の FT8 呼び出し
speech-ft8-cq = 応答できる FT8 CQ
    .hint = 20 m の混雑した夜には、1 分に 100 件になります
speech-js8 = 私宛の JS8 メッセージ
speech-js8-allcall = JS8 @ALLCALL も
speech-fsq = 私宛の FSQ メッセージ
speech-include-snr = 報告値を含める
    .hint = メッセージ自体に報告値が含まれない場合のみ。すでに数値を報告しているデコードには、こちらの値は付けません
speech-ft8-qso = 自分の FT8 交信
    .hint = シーケンサーが次に送信する内容と、交信の終了
speech-reading-text = デコードされた文字を読み上げる
speech-text-cw = CW
speech-text-rtty-psk = RTTY、PSK、Olivia、THOR、FSQ
speech-text-cw-locked = デコーダーがロックしている間の CW のみ
    .hint = ロックしていないデコーダーの出力を読み上げるのは、無音より悪い結果になります
speech-text-note = どちらも既定ではオフです。デコーダーは音声の読み上げより速くテキストを生成するため、ライブ音声から遅れすぎたものはキューに入れず捨てられます。
speech-tuning-up = 同調中
speech-swr-while-tuning = TUNE を押している間、SWR を読み上げる
speech-swr-every = 毎回
speech-swr-summary = 離したとき、最も良い整合を報告する
speech-swr-alarm = どの送信中でも、高い SWR を警告する
speech-how-read = 読み上げ方
speech-frequencies = 周波数
speech-callsigns = コールサイン
speech-quiet-on-tx = 送信中は沈黙する
    .hint = 音声はスピーカーに出力されるため、マイクにも入ります。高 SWR の警告は引き続き届きます。
speech-device-note = アナウンスは専用のサウンドデバイスで再生されるため、録音されることも、リモートで聴いている人に送られることもありません。ステータスを読み上げるキー、直前のアナウンスを繰り返すキー、文の途中で止めるキーは、コントロールタブにあります。

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = ステーションのネットワーク設定を待っています…
settings-device-reports = デバイスの報告どおり
settings-transverters = トランスバーター
settings-one-row-per-box-while = 1 行につき 1 つのボックスです。ダイヤルがその行のバンド内にあるとき、無線機は ダイヤル + オフセット に合わせられ、その行の送信ルールと駆動制限が適用されます。どの行の外にあるときは、無線機は本来のバンドで動作します。行は上から順に判定されます。
settings-name = 名前
settings-band-low = バンド下限
settings-band-high = バンド上限
settings-offset = オフセット
settings-transmit = 送信
settings-max-drive = 最大駆動
settings-off-takes-transverter-out-line = オフにすると、行を残したままこのトランスバーターを経路から外します。バンドは、無線機が独自に届く範囲に戻ります。
settings-t-2-m = 2 m
settings-what-you-call-shown-log = 呼び名です。ダイヤルでこれが選ばれたときにログに表示されます。
settings-bottom-band-transverter-works-dial = このトランスバーターが扱うバンドの下端（ダイヤル上）。2 m なら 144 です。
settings-top-band-dial-148-2 = バンドの上端（ダイヤル上）。リージョン 2 と 3 では 2 m は 148、リージョン 1 では 146 です。
settings-radio-ends-up-dial-offset = 無線機はダイヤル + オフセットに合わせられるため、I.F. へ帯域を下ろすトランスバーターは負の値になります。28 MHz の I.F. に入れる 2 m は -116 です。144 - 116 = 28 だからです。水晶の発振がわずかにずれたオシレーターには、1 Hz ずつドラッグして微調整します。

    Through the same converter: it works both ways, and transmit takes the same offset.

    Your Drive setting is held under this rather than moved, so the number you use on HF comes back when the dial leaves the band.
settings-remove = 削除
settings-add-transverter = トランスバーターを追加
settings-takes-effect-apply-reconnect-like = 上のコンバーターのオフセットと同様、適用 / 再接続で有効になります。
settings-maximum-transmit-drive = 最大送信駆動
settings-hard-limit-drive-tune-controls = 駆動と TUNE の操作でも超えられない固定の上限です。バンドごとのキャリブレーションの後に適用されるため、何によっても駆動がこれを超えることはありません。無線機の定格出力に合わせると、駆動の操作全体が使えるようになります。自分でサンプルを変調する送信機（HPSDR、LimeSDR、PlutoSDR、HackRF）では、フル駆動は送信機が全開であることを意味し、高利得のアンプでは定格を大きく超えます。オフにすると、操作はフル駆動まで届きます。
settings-limit-drive = 駆動を次の値に制限
settings-per-cent-full-drive-key = フル駆動に対するパーセント。ダミーロードに送信し、アンプの定格出力をメーターが示すまでこの値を下げてください。
settings-applies-immediately-every-band-tune = すぐに適用され、すべてのバンドで、TUNE にも適用されます。
settings-transmit-drive-band = バンドごとの送信駆動
settings-every-amplifier-has-different-gain = アンプはバンドごとに利得が異なるため、同じ駆動設定でもバンドごとに出力が異なります。各バンドで出力を測定し、出力が高すぎるバンドを、すべて揃うまで下げてください。最も必要とするバンド（通常は最も高いバンド）に合わせて駆動を設定し、他のバンドをそれに合わせて下げます。音声、デジタル、TUNE のすべてに適用される出力のデシベル値です。すべてゼロ（既定値）では何も変わりません。
settings-other = その他
settings-clear-all = すべてクリア
settings-back-no-calibration-drive-setting = キャリブレーションなしに戻す：駆動設定がすべてのバンドにそのまま届きます。
settings-applies-immediately-band-you-would = すぐに適用され、送信するバンドに適用されます。トランスバーターの背後では、ダイヤル上のバンドが対象となり、トランスバーター自身の駆動制限は、ここで設定したものより常に優先されます。
settings-settings = 設定
settings-enter-server-s-address-first = まずサーバーのアドレスを入力してください
settings-enter-receiver-s-address-first = まず受信機のアドレスを入力するか、Public SDRs から選んでください
settings-no-radio-selected-press-discover = 無線機が選択されていません。検出を押すか、アドレスを入力してください
settings-enter-radio-s-address-first = まず無線機のアドレスを入力してください
settings-general = 全般
settings-radio = 無線機
settings-ui = UI
settings-alerts = アラート
settings-controls = コントロール
settings-spots = スポット
settings-freedv = FreeDV
settings-uploads = アップロード
settings-winlink = Winlink
settings-servers = サーバー
settings-t-r-switch = T/R スイッチ
settings-tle = TLE
settings-profiles = プロファイル
settings-station = ステーション
settings-enter-digital-mode-ft8-sstv = デジタルモード（FT8 / SSTV / …）を一度選ぶと、保存された値が読み込まれます。
settings-callsign = コールサイン
settings-grid-square = グリッドスクエア
settings-iaru-region = IARU リージョン
settings-which-radio-interface-server-uses = サーバーが使う無線インターフェースは、サーバーを動かしているマシンで設定します。
settings-radio-interface = 無線インターフェース
settings-waiting-machine-radio-attached-interface = 無線機が接続されているマシンを待っています。そのインターフェースの設定は下にあり、その間もここから変更できます。
settings-converter = コンバーター
settings-manual-type-offset-below = 手動：下にオフセットを入力
settings-what-box-reads-when-offset = オフセットがプリセットのいずれでもないとき、このボックスに表示される値です。下のオフセット欄に入力して設定します。ここで選ぶものはありません。
settings-frequency-converter-front-receiver-pick = 受信機の前に置く周波数コンバーターです。1 つ選ぶか、横にオフセットを入力してください。
settings-how-far-converter-moves-signal =
    コンバーターが信号を受信機へ運ぶ際にずらす量を Hz で示します。どのコンバーターの説明書や、他のすべての SDR ソフトでも、同じ数値と符号が使われます。無線機はダイヤル + オフセットに合わせられます。アップコンバーター（Ham It Up は 125000000）は正、衛星の LNB や、28 MHz の I.F. に 2 m を下ろすトランスバーター（28 − 144 = -116000000）のように、バンドを I.F. に下ろすものは負です。0 = コンバーターなし。

    Drag to trim it a hertz at a time, which is what a converter whose oscillator is slightly off wants.

    This is the receive path. What is in the transmit line is the row below.

    Takes effect on Apply.
settings-what-transmit-line-while-converter =
    コンバーターを設定している間の送信経路の内容です。

    Off while converting: nothing is transmitted — the safe default, and right for a receive-only accessory.

    Through the same converter: one box converts both ways, a transverter. Transmit takes the offset above and follows it when it is trimmed.

    Its own offset: the transmit line is different from the receive one. Nought is the common case — a converter (an LNB, a Ham It Up) on receive with the transmitter on its own antenna, which is the QO-100 station.

    The amateur-band check still applies, on the frequency you are transmitting on.

    Takes effect on Apply.
settings-how-far-transmit-line-moves =
    送信経路が信号を移動させる量を Hz で示します。符号規則は受信オフセットと同じで、無線機は ダイヤル + オフセット で送信します。

    0 = the transmitter is wired to its own antenna and works on the frequency the dial says — a QO-100 station's 2.4 GHz uplink.

    A transmit converter that takes an I.F. *up* is a negative number: the radio works below the dial.
settings-rx-range = 受信範囲
settings-which-frequencies-radio-receives-mhz =
    この無線機が受信する周波数を MHz で指定します。例：144-146, 430-440。空欄にすると、デバイスが自身について報告する値を使います。

    Band buttons outside the range are greyed out and the dial will not go there.

    These are dial frequencies. With a converter set, state the band you tune — 144-148 for a 2 m transverter — not the I.F. the radio is really on.

    Takes effect on Apply.
settings-tx-range = 送信範囲
settings-which-frequencies-radio-transmits-mhz =
    この無線機が送信する周波数を MHz で指定します。例：144-146, 430-440。空欄にすると、デバイスが報告する値を使います。何も報告しない場合は、ドライバーの申告が信頼され、どの周波数でも許可されます。

    Dial frequencies, like the receive range above.

    This is a limit you set, not a licence: transmitting outside the amateur bands is refused regardless unless you have turned that off in config.toml. Nor does it give a receive-only device a transmitter.

    Takes effect on Apply.
settings-antenna = アンテナの場所
settings-where-radio-listens-station-antenna =
    この無線機が受信する場所です。

    At the station: the antenna is yours, so your own locator says where it is. The default, and right for everything in the shack.

    Somewhere else: an online receiver, or your own set up on a hilltop. What it hears is reported to PSK Reporter, WSPRnet and FreeDV Reporter from the locator beside this, never from yours — and ADS-B places aircraft against it too.

    Picking a receiver under "Public SDRs" fills this in for you.

    Takes effect on Apply.
settings-receiver-s-maidenhead-locator-jn88ec =
    受信機のメイデンヘッド・ロケーターです（例：JN88ec、DO30db）。4 または 6 文字です。

    Leave it empty if you do not know where the receiver is: nothing is then reported at all, which is the only honest answer. Reporting from your own square would put somebody else's reception on the wrong continent.

    On top of cal_offset_db in config.toml, which is one figure for the whole station.
settings-antenna-somewhere-else-no-locator = アンテナは別の場所にあり、ロケーターが設定されていないため、この無線機が受信した内容は PSK Reporter、WSPRnet、FreeDV Reporter に報告されません。上に受信機のロケーターを入力すると、そこから報告されます。
settings-ranges-are-mhz-low-high = 範囲は MHz で、低-高の形式で、カンマ区切りで指定します：144-146, 430-440。これは 144000000-146000000 Hz と 430000000-440000000 Hz を意味します。これらはダイヤル周波数で、コンバーターのオフセットは移動させません。上のコンバーターのオフセットは Hz 単位の欄です。範囲を空欄にすると、デバイスが自身について報告する値を使います。何も報告しないデバイスは、その申告が信頼されます。
settings-careful-rtl-sdr-blog-v4 = RTL-SDR を使う場合は注意：Blog V4 は 28.8 MHz 未満で自身がアップコンバートするため、ハードウェアがそこに来る負のオフセットは二重にずれます。
settings-pick-radio-interface-above-configuration = 上で無線インターフェースを選んでください（この構成は削除された自動検出モードを使っていました）。
settings-configuration-used-removed-auto-detect = この構成は削除された自動検出モードを使っていました。無線機が接続されているマシンが応答したら、上でインターフェースを選んでください。
settings-radio-has-no-interface-yet = この無線機にはまだインターフェースがありません。上で選び、適用 / 再接続を押してください。
settings-radio-has-no-interface-yet-2 = この無線機にはまだインターフェースがありません。無線機が接続されているマシンが応答すると、上で選べます。
settings-apply-reconnect = 適用 / 再接続
settings-switch-interface-now-no-restart = このインターフェースに今すぐ切り替える（再起動は不要）
settings-switch-server-s-radio-interface = サーバーの無線機を、このインターフェースに今すぐ切り替えます（再起動は不要）。新しいものを開けない場合は、古いものが動き続けます。
settings-switches-live-radio-without-restarting = 再起動せずに、稼働中の無線機を切り替えます。
settings-everything-above-server-s-own = 上のすべてはサーバー自身の構成で、変更はそこに保存されます。ほとんどの設定は変更と同時に適用されます。デバイスを開いたときに固定される項目（サンプルレート、アドレス、インターフェースそのもの）には、このボタンが必要です。
settings-dx-cluster-telnet = DX クラスター（telnet）
settings-enabled = 有効
settings-host = ホスト
settings-port = ポート
settings-login-call = ログインコール
settings-commands = コマンド
settings-one-per-line-e-g = 1 行に 1 つ。例：SET/FT8
settings-one-per-line-e-g-2 = 1 行に 1 つ。例：set/filter cont=eu
settings-sent-after-login-place-narrow = ログイン後に送信されます。フィードを絞り込む場所です。フィルターがなければ、地球上のすべてのスキマーが対象になります。
settings-rbn-paths-are-placed-country = RBN の経路はロケーターではなく国の中心から配置されるため、小さな国では正確ですが、大きな国では大きくずれます。伝搬マップ上では独立したレイヤーで、そこでオフにできます。
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = POTA アクティベーターのスポット
settings-sota-spots = SOTA のスポット
settings-psk-reporter-current-band = PSK Reporter（現在のバンド）
settings-upload-my-ft8-ft4-ft2 = 自分の FT8/FT4/FT2/JS8 デコードをアップロード
settings-antenna-2 = アンテナ
settings-collector = コレクター
settings-t-4739-live-collector-14739-test = 4739 は本番のコレクター、14739 はテスト用です
settings-max-age-s = 最大経過時間（秒）
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = 自分の WSPR デコードをアップロード
settings-send-every-wspr-reception-wsprnet = すべての WSPR 受信を wsprnet.org に送信します。既定でオン：電波は発信しません。受信した内容を報告することで、WSPR 受信機は個人の趣味にとどまらず、ネットワークの一部になります。何もデコードしなかったスロットも報告されます。これにより、ネットワークは閉じたバンドと電源を切った受信機を区別できます。
settings-download-who-heard-me = 私を受信した局をダウンロード
settings-ask-wsprnet-org-which-stations = wsprnet.org に、どの局がこの局をデコードしたかを問い合わせます。WSPR には確認応答の仕組みがまったくないため、送信しているビーコンが自分の到達範囲について知る方法は、これだけです。
settings-ask-every = 問い合わせ間隔
settings-looking-back = 過去を遡って
settings-apply = {" "}適用{" "}
settings-persist-re-connect-feeds = フィードを保存して（再）接続
settings-broadcast-stations = 放送局
settings-winlink-account = Winlink アカウント
settings-password = パスワード
settings-winlink-account-password-not-gateway = Winlink アカウントのパスワードです。ゲートウェイのパスワードではありません。大文字と小文字を区別するので、発行されたとおりに正確に入力してください。
settings-locator = ロケーター
settings-how-connect = 接続方法
settings-route = 経路
settings-internet = インターネット
settings-forward-cms-over-internet = インターネット経由で CMS と転送
settings-radio-packet = 無線（パケット）
settings-call-rms-gateway-air-radio = 電波で RMS ゲートウェイを呼び出します。無線機は PACKET または PACKET-HF モードにする必要があります。
settings-gateway = ゲートウェイ
settings-via = 経由
settings-digipeaters-order-separated-spaces-usually = 中継局（デジピーター）を順番に、スペース区切りで指定します。通常は空欄です。直接聞こえるゲートウェイには、直接呼び出すべきです。
settings-speed = 速度
settings-most-rms-gateways-answer-1200 = ほとんどの RMS ゲートウェイは 1200 で応答します。1200 のゲートウェイを 9600 で呼ぶと、電源が落ちているゲートウェイとまったく同じ音になるため、推測せず、ゲートウェイの運用者が公表している値を設定してください。9600 は両端で無線機のデータポートも必要です。300 は HF です。
settings-frequency = 周波数
settings-clear = クリア
settings-zero-leaves-dial-alone-which = 0 にするとダイヤルはそのままになります。1 つのチャンネルに固定して使う場合に便利です。それ以外の値は、セッション開始時に無線機を同調させます。
settings-my-gateways = 自分のゲートウェイ
settings-winlink-s-published-gateway-list = Winlink の公開ゲートウェイ一覧には、sdroxide が持っていない API キーが必要なため、自分で管理してください。1 か所から到達できる 2〜3 のゲートウェイは、試して覚えるもので、ほとんど変わりません。
settings-use-2 = 使う
settings-call-one-next-connect = 次の接続でこれを呼び出す
settings-current-dial = 現在のダイヤル
settings-forget = 削除
settings-forget-gateway = このゲートウェイを削除
settings-add-gateway = + ゲートウェイを追加
settings-save-gateway-above-list = 上のゲートウェイをリストに保存
settings-internet-gateway = インターネットゲートウェイ
settings-cms-address = CMS アドレス
settings-client-name = クライアント名
settings-winlink-s-production-servers-only = Winlink の本番サーバーは既知のクライアント名しか受け付けず、未知の名前には「本番サーバーでは未知のクライアントタイプは許可されていません」と応答します。sdroxide が Winlink 開発チームに登録されるまでは、接続には認識される名前が必要です。
settings-automatic-connection = 自動接続
settings-connect-timer = タイマーで接続
settings-every = 間隔
settings-persist-winlink-account = Winlink アカウントを保存
settings-callsign-lookup = コールサイン検索
settings-provider = プロバイダー
settings-auto-fill-name-qth-grid = スポットをクリックしたときと QSO で、名前/QTH/グリッドを自動入力
settings-choose-provider-enter-login = ログインを入力するプロバイダーを選んでください。
settings-qrz-user = QRZ ユーザー
settings-qrz-pass = QRZ パスワード
settings-hamqth-user = HamQTH ユーザー
settings-hamqth-pass = HamQTH パスワード
settings-one-hamqth-account-does-both = HamQTH のアカウントは 1 つで両方に使えます。このログインは HamQTH アップロードタブでも使われます。
settings-upload = アップロード
settings-auto-upload-each-new-qso = 新しい QSO を自動アップロード
settings-master-switch-which-services-qso = 主スイッチです。QSO をどのサービスに送るかは、下の各タブで設定します。
settings-turn-auto-upload-each-new = 上の「新しい QSO を自動アップロード」を先にオンにしてください。ここでチェックしたサービスは、それまで送信されません。
settings-auto-upload-off-above-so = 上で自動アップロードがオフのため、サービスのチェックは無効になっており、まだ何も自動送信されません。ログブックの QSO ごとの UP ボタンは引き続き使えます。
settings-qrz-log-key = QRZ ログキー
settings-qrz-logbook-api-key-your = QRZ ログブックの設定にある API キーです。XML 検索のログインではありません。
settings-eqsl-user = eQSL ユーザー
settings-eqsl-pass = eQSL パスワード
settings-eqsl-qth = eQSL QTH
settings-same-login-hamqth-callsign-lookup = HamQTH のコールサイン検索と同じログインです。どちらか一方に入力すると、両方に入力されます。
settings-club-log-email = Club Log メール
settings-club-log-pass = Club Log パスワード
settings-club-log-key = Club Log キー
settings-wrl-api-key = WRL API キー
settings-developer-api-key-world-radio = World Radio League → Integrations → Developer API で発行される開発者 API キーです。生成時に一度だけ表示されるので、その時にコピーしてください。交信は既定のログブックに送られます。複数ある場合は、WRL で設定してください。
settings-confirmations-download = 確認（ダウンロード）
settings-lotw-user = LoTW ユーザー
settings-lotw-pass = LoTW パスワード
settings-lotw-upload-uses-tqsl-export = LoTW へのアップロードには TQSL を使います。ログブックから ADIF を書き出して署名します。LoTW/eQSL の確認はここでダウンロードされ、交信済みと確認済みを区別します。
settings-sync-confirmations = 確認を同期
settings-fetching-subscriptions = 購読を取得中…

    To use it on its own again, clear the Panadapter receiver box on that radio's page.
settings-panadapter = パナダプター
settings-give-radio-wideband-panadapter-borrowing = 別の無線機の受信機を借りて、この無線機に広帯域のパナダプターを与えます。同じアンテナ上の SDR、またはこの無線機の I.F. 出力から供給される受信機が対象です。スペクトラム、ウォーターフォール、サブ受信機、デジタルモード、スキマーはすべてその受信機から得られます。ダイヤル、モード、フィルター、送信機はここに残ります。
settings-receiver = 受信機
settings-none = なし

    Takes effect on Apply.
settings-connected = 接続先

    This is not the Converter field above, which retunes the radio, and not the CAT tab's I/Q centre offset, which is about this radio's own sound card. Nothing typed here is ever sent to either radio.

    Takes effect on Apply.
settings-audio-2 = 音声の入力元
settings-follow-dial = ダイヤルに追従
settings-keep-receiver-radio-s-dial = 受信機をこの無線機のダイヤルに合わせたままにし、パナダプターで同調するとダイヤルも動きます。オフにすると受信機はその場に留まり、無線機が別の場所へ行っても、1 つのセグメントを固定して監視します。
settings-invert-spectrum = スペクトラムを反転
settings-mirror-receiver-s-span-about = 受信機の幅を中心を軸に反転します。オシレーターが信号より上にあり、帯域が逆向きに渡されるタップ用です。ウォーターフォールがダイヤルの反対側にある信号で埋まる場合にのみオンにし、それ以外はオフのままにしてください。
settings-mute-transmit = 送信中はミュート
settings-silence-receive-audio-while-radio = この無線機が送信している間は受信音声を消します。既定でオンです。受信機が同じアンテナ上、またはこの無線機の I.F. にあるとき、聞こえるのは自分の送信機です。無線機本体で発信した送信（マイクの PTT、フットスイッチ、MOX）も、CAT 接続が無線機の送信状態を伝えた時点で対象になります。
settings-blank-transmit = 送信中は表示を停止
settings-stop-panadapter-waterfall-while-radio = この無線機が送信している間、パナダプターとウォーターフォールを止めます。既定でオンです。全幅に描かれた送信機は、その下の帯域を消してしまうためです。オフにすると、送信中もバンドや自分の信号を見られます。受信機が別のアンテナにあるときに便利です。対象は sdroxide が発信する送信のみです。無線機で発信されたものはそのまま受信されるため、この設定に関係なく表示は動き続けます。
settings-per-mode-offsets-radio-whose = 無線機の I.F. がモードとともに動く場合のための、モード別オフセットです。空欄のモードは上のオフセットを使います。
settings-computer = このコンピューター上
settings-add-radio-computer = このコンピューターに無線機を追加
settings-radio-s-settings-are-below = この無線機の設定は下にあります。ドラッグして帯に沿って移動できます。
settings-switch-radio-dialog-follows-drag = この無線機に切り替えます（ダイアログも追従します）。ドラッグして帯に沿って移動できます。
settings-tx = ● TX
settings-mute-radio-s-audio = この無線機の音声をミュート
settings-close-radio-configuration-kept = この無線機を閉じます（構成は保持されます）
settings-close-tab-close-radio-station = このタブを閉じるか、ステーションでこの無線機を閉じてください
settings-close-tab = このタブを閉じる
settings-transverters-full = { $max } は、この表が保持できる最大数です
settings-dialling = ダイヤル中… { $url }
settings-region-note = ステーションの所在地です。すべてのバンドプランに影響します。バンド端、ウォーターフォール帯の CW / データ / 電話のサブセグメント、スキマーの待ち受け位置、提案される呼び出し周波数、送信時にバンド外とみなされる範囲を決めます。リージョン { $region } では 70 cm は { $region ->
        [1] 430–440 MHz となり、446 はバンド外、40 m は 7.200 で止まります
        [2] 420–450 MHz となり、40 m は 7.300 まであります
       *[3] 430–450 MHz となり、80 m は 3.900 で止まります
    } すぐに有効になり、このステーションのすべての無線機に適用されます。
settings-rx-grid-reported = この無線機で受信した内容は、自分のロケーターではなく { $grid } から報告されます。
settings-rx-grid-invalid = { $grid } はメイデンヘッド・ロケーターではないため、この無線機が受信した内容は報告されません。4 または 6 文字で、2 文字の英字、2 桁の数字、必要に応じてさらに 2 文字の英字です。
settings-gateway-via = { " " }経由 { $via }
settings-auto-upload-to = 新しい QSO を { $service } へ自動アップロード
settings-tle-updated = { $n } 件の購読を更新しました：衛星 { $total } 件。
settings-tle-updated-failed = { $n } 件中 { $ok } 件を更新、{ $failed } 件が失敗 — 上の行を参照してください。
settings-panadapter-of = この無線機は { $radio } のパナダプターです
settings-radio-n = 無線機 { $n }
settings-on-station = { $station } 上
settings-add-radio-on-station = { $station } に無線機を追加 — ステーションがすぐに提供します
settings-panadapter-receiver-for = { $radio } のパナダプター受信機
settings-hang-up-stays-on = 切断します。無線機は { $station } に残ります。
settings-close-on-station = { $station } 上の { $radio } を閉じますか？ その構成はそのマシンに残ります。ここのタブは閉じられ、他の人のタブも同様に閉じられます。
settings-close-it-on = { $station } で閉じる
settings-locator-hint = ロケーター

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = 音によるアラート
settings-alerts-sound-alarm-when-decode-matters = 重要なデコードがあったとき警報音を鳴らす
settings-alerts-plays-over-own-audio-output = 専用の音声出力から再生するため、バンドがこの画面とは別のスピーカーにあっても聞こえます。
settings-alerts-volume = 音量
settings-alerts-output = 出力
settings-alerts-test = テスト
settings-alerts-what-sound = 鳴らす内容
settings-alerts-each-station-quiet-while-after = 各局はアラートの後しばらく静かになるため、混み合ったバンドでもすべての時間帯で鳴り続けることはありません。音声と「トーン + 音声」は、音声アナウンスの声で読み上げられます。設定 → UI でオンにしてください。

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = アンプ
settings-amp-waiting-station-s-amplifier-configuration = ステーションのアンプ構成を待っています…
settings-amp-hobbypcb-hardrock-500-hardrock-50 = USB シリアルポートに接続された HobbyPCB の Hardrock-500 または Hardrock-50 です。sdroxide は AMP ウィンドウからそのメーターを読み取り、バンド、モード、アンテナを切り替え、チューナーを動かします。アンプは従来どおり無線機の PTT 回線でキーされます。
settings-amp-enable = 有効
settings-amp-model = 機種
settings-amp-untested-panel-has-only-been = 未検証：このパネルは Hardrock-500 でのみ試験されています。この機種のメーターの上限は Hardrock-500 から換算しており、誤っている可能性があります。アンプ自身の表示を確認してください。
settings-amp-port = ポート
settings-amp-choose = — 選択 —
settings-amp-baud = ボーレート
settings-amp-must-match-amplifier-s-menu = アンプのメニューと一致させる必要があります。HobbyPCB の制御パネルは 115200 が必要です。
settings-amp-follow-radio-s-band = 無線機のバンドに追従
settings-amp-sends-transmit-frequency-amplifier-every = バンドを変えるたびに送信周波数をアンプへ送るため、アンプのフィルターは専用の CAT ケーブルなしで無線機に追従します。送信中には決して送られません。
settings-amp-show-power-swr-tx-meter = 出力と SWR を送信メーターに表示
settings-amp-only-where-radio-does-not = 無線機自身が測定しない場合のみで、アンプが回路に入っている間だけ表示されます。
settings-amp-off = オフ
settings-amp-connecting = 接続中…
settings-amp-status = 状態
settings-amp-apply = {" "}適用{" "}
settings-amp-save-re-open-amplifier-s = アンプのポートを保存して（再）オープン

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = キーボード
settings-controls-click-shortcut-rebind-then-press = ショートカットをクリックして割り当てを変更し、キーの組み合わせを押してください（Esc で取り消し）。テキスト欄に入力している間は、割り当ては無視されます。
settings-controls-shortcut = ショートカット
settings-controls-does = 動作
settings-controls-step-mode = ステップ / モード
settings-controls-accel = 修飾
settings-controls-on = 有効
settings-controls-press-key = キーを押してください…
settings-controls-remove-binding = この割り当てを削除
settings-controls-add-shortcut = + ショートカットを追加
settings-controls-restore-defaults = 既定に戻す
settings-controls-bind-hold-talk-space = 押している間送信をスペースキーに割り当てる
settings-controls-hold-space-transmit-releasing-losing = スペースキーを押している間送信します。離す — またはウィンドウのフォーカスを失う — と送信が解除されます
settings-controls-unkey-held-ptt-after = 押し続けた PTT を次の時間後に解除
settings-controls-backstop-against-stuck-key-controller = キーが引っかかったり、コントローラーが報告を止めたりした場合の安全策です。0 で無効になります。
settings-controls-panadapter-mouse = パナダプターのマウス
settings-controls-wheel = ホイール
settings-controls-wheel-shift = ホイール + Shift
settings-controls-tune-step = 同調ステップ
settings-controls-zoom-rate = ズーム速度
settings-controls-click-tune-rounding = クリック同調の丸め
settings-controls-invert-wheel-direction = ホイールの方向を反転
settings-controls-left-drag-tunes-well-pans = 左ドラッグで表示を移動するとともに同調する
settings-controls-off-makes-left-drag-pan = オフにすると、左ドラッグは右ドラッグと同様に表示の移動のみを行います。
settings-controls-scroll-digit-frequency-readout-tune = 周波数表示の桁をスクロールして同調する
settings-controls-both-wheel-actions-are-tune = ホイールの両方の動作が同調になっているため、ズームする方法が残っていません。
settings-controls-restore-mouse-defaults = マウスの既定に戻す
settings-controls-mouse-buttons = マウスボタン
settings-controls-left-right-buttons-are-reserved = 左右のボタンは同調と表示の移動に予約されています。中ボタンと拡張ボタンは自由に設定できます。PTT に割り当てた側面ボタンは、フットスイッチと同じように動作します。
settings-controls-add-mouse-button = + マウスボタンを追加
settings-controls-f1-always-opens-manual-even = F1 は入力中であっても常にこのマニュアルを開くため、割り当てを変更できません。
settings-controls-midi-controller = MIDI コントローラー
settings-controls-midi-controllers-need-native-app = MIDI コントローラーにはネイティブアプリが必要です。ブラウザ版クライアントは MIDI にアクセスできません。
settings-controls-any-class-compliant-midi-surface = クラス準拠の MIDI 機器なら何でも使えます。DJ コントローラーのジョグホイールは良い VFO つまみになり、そのパッドは PTT やバンドボタンに、フェーダーはゲインの操作になります。
settings-controls-enable = 有効
settings-controls-controller = コントローラー
settings-controls-feedback = フィードバック先
settings-controls-rescan-ports = ポートを再スキャン
settings-controls-not-connected = 接続されていません。
settings-controls-move-control-see-here = コントロールを動かすと、ここに表示されます。
settings-controls-control = コントロール
settings-controls-reads = 読み取り
settings-controls-led = LED
settings-controls-move = 動かす…
settings-controls-click-then-move-control-you = クリックしてから、割り当てたいコントロールを動かしてください
settings-controls-speed-sensitivity-spin-faster-tune = 速度感度：速く回すほど速く同調します
settings-controls-send-current-value-back-light = 現在の値を返して、LED を点灯させたりフェーダーを動かしたりします
settings-controls-add-midi-control = + MIDI コントロールを追加
settings-controls-clear-all = すべてクリア
settings-controls-endless-jog-encoders-send-relative = エンドレス（ジョグ）エンコーダーは、位置ではなく相対的なステップを送ります。小さな動きでは区別がつきにくい 3 つの方式のいずれかです。LEARN は時計回りの回転から推測します。つまみが逆向きに同調する場合は「rev」にチェックを入れてください。
settings-controls-none = — なし —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = システムの既定値
settings-general-remote-access = リモートアクセス
settings-general-what-remote-client-browser-page = リモートクライアント（ブラウザのページ、または --connect で起動した別の sdroxide）が、このステーションの操作を許可される前に提示する必要があるものです。サーバーモード（--server）で適用されます。変更は次回のサインイン時に反映され、再起動は不要です。
settings-general-username = ユーザー名
settings-general-password = パスワード
settings-general-clients-must-give-password-leaving = クライアントはパスワードを提示する必要があります。ユーザー名を空欄にしても構いません。
settings-general-clients-must-sign = クライアントはサインインする必要があります。
settings-general-empty-anyone-who-can-reach = ⚠ 空欄：サーバーのポートに到達できる人は誰でも、あなたのコールサインでこの無線機を操作できます。ポートを転送する前にパスワードを設定してください。
settings-general-stored-clear-config-toml-like = sdroxide が保持する他のパスワードと同様に、config.toml に平文で保存されます。
settings-general-receive-audio-gain = 受信音声のゲイン
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    音量つまみに加えて、この無線機がスピーカーへ送るすべての音声に追加されるゲインです。音量を最大にしても無線機の音が小さい場合を除き、0 dB のままにしてください。音量の上限は到着した音声そのものなので、無線機を小さくすることはできても、大きくすることはできません。また、一部のトランシーバーの USB 音声出力はフルスケールよりかなり低い位置にあります。

    Go up 6 dB at a time. Too much clips — the audio is limited at full scale rather than allowed to wrap round, so overdoing it sounds harsh rather than loud. Recordings are taken ahead of this and are not affected.
settings-general-reload-band-plan = バンドプランを再読み込み
settings-general-re-read-bandplan-json-machine = 無線機が接続されているマシンの bandplan.json を読み直して適用します。バンド端、サブセグメント、スキマーの待ち受け範囲を、再起動せずに反映します。
settings-general-running-station-s-own-band = ステーション独自のバンドプランで動作中です。
settings-general-running-built-iaru-tables = 内蔵の IARU 表で動作中です。
settings-general-settings-file = 設定ファイル
settings-general-settings-are-machine-radio-attached = 設定は無線機が接続されているマシン上にあります。そこで書き出してください。
settings-general-export = 書き出し…
settings-general-write-every-setting-station-radios = このステーションのすべての設定（無線機、モード、サーバー、メモリー、バンドプラン）を、別のインストールに持ち運べる 1 つのファイルに書き出します。ログブックと保存されたサーバーのパスワードはここに残ります。
settings-general-import = 読み込み…
settings-general-replace-station-s-settings-ones = このステーションの設定を、別のインストールから書き出されたファイルの設定で置き換えます。その後 sdroxide を再起動してください。
settings-general-import-overwrites-what-here-file = 読み込むと、ここにあるものがファイル単位で上書きされ、次回 sdroxide を起動したときに有効になります。メモリー上の設定がそのまま書き戻されてしまうためです。ファイルに記載のないものはそのまま残るため、無線機 1 台のステーションのバンドルで、ここにある 2 台目の無線機が削除されることはありません。ログブックはファイルに含まれません。それも移動したい場合は、LOG ウィンドウから ADIF で書き出してください。
settings-general-sdroxide-settings = sdroxide の設定
settings-general-swr-guard = SWR ガード
settings-general-stop-transmitting-high-swr = 高い SWR で送信を停止
settings-general-trip = 作動値
settings-general-stops-transmission-when-radio-reports =
    無線機がこの値以上の SWR を報告すると送信を停止し、確認するまで送信を禁止したままにします。アンテナの切断、給電線の故障、誤ったポートに残されたスイッチを検出します。

    Tuning is treated differently, because feeding a mismatch is the point of it: an antenna tuner gets double the limit and about five seconds before the guard applies at all. A manual tuner that takes longer than that wants the guard switched off for the session.

    Needs a rig that reports SWR over CAT. Ignores the first fifth of a second of each transmission, and does not wait for high power.
settings-general-per-mode-settings = モード別の設定
settings-general-reset-every-mode = すべてのモードをリセット
settings-general-forget-every-mode-s-remembered = すべてのモードが記憶している AGC、スケルチ、ノイズリダクション、ノッチ、ステレオのスイッチを忘れ、各モード自身の既定値に戻します。
settings-general-selecting-mode-lays-mode-s = モードを選ぶと、そのモード自身の開始値（AGC、スケルチ、ノイズリダクション、ノッチ、ステレオのスイッチ）が受信機に設定されます。そのいずれかを変更すると、そのモードだけについて記憶されます。受信機のフィルター / ノイズの行の末尾にある丸矢印のチップには、変更された項目が一覧され、現在のモードの値に戻すことができます。
settings-general-channels = チャンネル

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = 運用者
settings-net-callsign-grid-are-set-general = コールサインとグリッドは全般タブで設定します。
settings-net-set-your-callsign-grid-general = ⚠ コールサインとグリッドを全般タブで設定してください。
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = 有効
settings-net-connects-whenever-enabled-your-station = 有効にすると常に接続されます。あなたのステーションは、無線機が RADE モードのときだけ他の人に表示されます。それ以外のモードでは接続したまま非表示になります。
settings-net-station = ステーション
settings-net-message = メッセージ
settings-net-receive-only-i-cannot-transmit = 受信のみ（送信できない）
settings-net-server = サーバー
settings-net-host = ホスト
settings-net-port = ポート
settings-net-tls-wss = TLS（wss://）
settings-net-not-yet-implemented-freedv-gui = 未実装です。FreeDV GUI も同様に平文の ws:// を使います。
settings-net-reporting = 報告
settings-net-report-stations-i-decode = デコードした局を報告
settings-net-sends-rx-report-each-callsign = RADE の End-of-Over フレームから復元された各コールサインについて、rx_report を送信します。
settings-net-show-other-reporter-stations-spots = 他のレポーター局をスポットとして表示
settings-net-adds-them-panadapter-overlay-world = それらを FREEDV フィルターの下で、パナダプターのオーバーレイ、世界地図、スポットウィンドウに追加します。
settings-net-set-your-callsign-grid-general-2 = ⚠ コールサインとグリッドを全般タブで設定してください。両方がなければ接続は閲覧のみになります。他の局は見えますが、あなた自身は表示されません。
settings-net-apply = {" "}適用{" "}
settings-net-persist-re-connect = 保存して（再）接続
settings-net-season = シーズン
settings-net-downloading = ダウンロード中…
settings-net-using-built-copy = 内蔵のコピーを使用中
settings-net-your-own-stations-corrections-go = 自分の局や訂正はこのファイルに記入します。sdroxide がこのファイルに書き込むことはなく、ここの項目は、同じ名前と周波数のスケジュールの項目を置き換えます。
settings-net-reload = 再読み込み
settings-net-re-read-your-station-file = 編集後に自分の局のファイルを読み直す
settings-net-broadcast-stations-labelled-waterfall-come = ウォーターフォールに表示される放送局は、このビルドに組み込まれた一覧から取得されます。編集にはデスクトップアプリが必要です。
settings-net-ask-service-whether-these-credentials = このアカウントが有効かをサービスに問い合わせます。先に上の設定を適用しますが、何も記録せず、読み取るだけです。
settings-net-checking = 確認中…

## Settings window: messages with values filled in.

settings-amp-connected-to = { $model } に接続されています
settings-controls-last-midi = 最後のメッセージ: { $message }  値 { $value }
settings-general-band-plan-file = すべてのバンド端とサブセグメントは { $path } から取得されます。初回は内蔵の IARU 表から書き出され、その後は編集できます。バンドを自分の免許の範囲に狭めると、sdroxide はその外への送信を拒否します。周波数は MHz 単位です。既定値の新しいコピーを得るには、このファイルを削除してください。これは地域の割り当てであり、あなたの免許ではありません。個々の条件はより狭い場合があり、同じ地域内でも国によって計画が異なります。
settings-general-exported = { $note } を書き出しました。保存先を選んでください。
settings-general-export-failed = 書き出しに失敗しました: { $error }
settings-general-import-failed = 読み込みに失敗しました: { $error }
settings-general-imported = { $summary } — 使うには sdroxide を再起動してください。
settings-general-import-skipped = { $path } をスキップしました: { $why }
settings-general-swr-tripped = ⚠ 現在 { $swr }:1 で作動中です — 送信は禁止されています。
settings-net-call-grid-set-on-general = { $call } / { $grid } — 全般タブで設定
settings-net-reporting-as = { $call } / { $grid } として報告中 — SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ ダウンロードに失敗しました: { $error }。内蔵のスケジュールを使用します。
settings-net-test-service = { $service } をテスト

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = コンバーターを設定している間は送信がオフです。送信を再び有効にするには、上の送信経路の内容を指定してください。
settings-transmit-goes-through-same-converter = 送信は同じコンバーターを通るため、無線機は双方向で、ダイヤルから同じオフセット分ずれた周波数で動作します。
settings-transmit-not-converted-radio-transmits = 送信は変換されません。無線機はダイヤルが示す周波数で送信し、受信はコンバーターを通ります。
settings-transmit-takes-own-offset-separate = 送信は、受信とは別の独自のオフセットを使います。
settings-no-enabled-subscriptions-update = 更新する有効な購読がありません。
settings-add-radio-here-station-you = 無線機を追加 — ここ、または接続しているステーションで

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = すべてのバンド端とサブセグメントは、無線機が接続されているマシンの bandplan.json から取得されます。これは地域の割り当てであり、あなたの免許ではありません。個々の条件はより狭い場合があり、同じ地域内でも国によって計画が異なります。

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = 受信画像 → FTP
settings-picup-uploads-every-received-sstv-picture = 受信したすべての SSTV 画像（チェックすれば、すべての気象図も）を、保存されるたびにアップロードします。QSSTV と同じ配置で送るため、QSSTV 用に書かれた Web ページはそのまま動きます。
settings-picup-upload-received-pictures = 受信画像をアップロード
settings-picup-server = サーバー
settings-picup-port = ポート
settings-picup-security = セキュリティ
settings-picup-user = ユーザー
settings-picup-password = パスワード
settings-picup-certificate = 証明書
settings-picup-trusted-when-computer-s-operating = このコンピューターのオペレーティングシステムが信頼している場合に信頼されます（ここのブラウザと同じです）。プライベート CA は下に追加できます（PEM）。
settings-picup-extra-ca = 追加の CA
settings-picup-only-certificate-fingerprint-accepted-way = このフィンガープリントを持つ証明書だけが受け入れられます。自己署名のサーバーを信頼する方法です。TEST はサーバーのものを表示します。
settings-picup-any-certificate-accepted-link-encrypted = ⚠ どの証明書も受け入れられます。接続は暗号化されていますが、経路上の誰でもサーバーになりすまし、パスワードを読み取れます。
settings-picup-plain-ftp-sends-password-pictures = ⚠ 平文の FTP は、パスワードと画像を暗号化せずに送信します。
settings-picup-sstv = SSTV
settings-picup-sstv-dir = SSTV ディレクトリ
settings-picup-weather-fax = 気象ファクス
settings-picup-wefax-dir = WEFAX ディレクトリ
settings-picup-empty-login-directory = 空欄の場合はログインディレクトリです。
settings-picup-file-names = ファイル名
settings-picup-format = 形式
settings-picup-add-extension = 拡張子を付ける
settings-picup-image1-png-rather-than-qsstv = QSSTV の image1 ではなく image1.png
settings-picup-caption-bar = キャプションバー
settings-picup-time-frequency-mode-sender-s = QSSTV が描くように、時刻、周波数、モード、送信者の FSK ID を上部に表示します。アップロードされたコピーにのみ印が付きます。
settings-picup-fit-960-768 = 960 × 768 に収める
settings-picup-qsstv-s-heartbeat-tiny-picture = QSSTV のハートビート：高さが分を表す小さな画像で、各画像と並べてアップロードされ、プログラムを終了すると幅 2 ピクセルになります。
settings-picup-nat-workaround = NAT の回避策
settings-picup-ignore-private-address-server-s = サーバーのパッシブモードの応答にあるプライベートアドレスを無視し、サーバー自身のアドレスを使います。公開アドレスを知らない NAT の背後にあるサーバー用です。
settings-picup-apply = {" "}適用{" "}
settings-picup-test = テスト
settings-picup-connect-settings-above-applied-not = 上の設定（適用済みかどうかに関係なく）で接続し、サインインして、各アップロード先ディレクトリに小さなファイルを書き込んで削除します。
settings-picup-checking = 確認中…
settings-picup-trust-certificate = この証明書を信頼
settings-picup-pin-switch-pinned-fingerprint-one = 固定するには、この証明書で「固定フィンガープリント」に切り替えます。先にサーバーの値と照合してください。これはサーバーが言っている内容であり、相手が誰であるかの証明ではありません。その後で適用してください。
settings-picup-last-upload = 最後のアップロード

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = プロファイル：ステーションの保存された設定
settings-profiles-profile-captures-whole-working-setup = プロファイルは作業設定全体を保存し、ワンクリックで戻します。ダイヤルと VFO、モードとフィルター、ゲイン、駆動とアンテナ、コールサインとメッセージのテンプレート、バンドごとのレジスターが対象です。ハードウェア（バックエンド、音声デバイス、コンバーター）は意図的に含みません。プロファイルはステーションの運用方法であり、動かす無線機ではないからです。
settings-profiles-save-current-setup = 現在の設定を保存
settings-profiles-profile-name = プロファイル名
settings-profiles-save = 保存
settings-profiles-applies-whatever-sdroxide-attached-dials = sdroxide が接続しているものに適用されます。ダイヤルと VFO、モードとフィルター、レベル、ゲイン、駆動、アンテナ、デジタルの識別情報、バンドスタックです。
settings-profiles-no-profiles-saved-yet-name = まだ保存されたプロファイルはありません。上で名前を付けて保存を押してください。
settings-profiles-saved-profiles = 保存されたプロファイル
settings-profiles-apply = 適用

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = 送受信スイッチ
settings-relay-waiting-station-s-t-r = ステーションの T/R スイッチ構成を待っています…
settings-relay-closes-contact-while-station-transmits = このステーションが送信している間、接点を閉じます。SDR のアンテナ入力を切り離して接地し、順番にアンプや外付けの T/R リレーをキーするためのものです。安価な USB リレーボード（LCUS、KMtronic、Numato）、PTT の接点閉鎖を求めるインターフェースへのシリアル RTS/DTR 回線、そしてコマンドフックを経由するその他のものに対応します。
settings-relay-hardware = ハードウェア
settings-relay-board = ボード
settings-relay-lcus-four-byte-a0-command = LCUS は、十数種類の名前で売られている 4 バイトの A0 コマンドボードです。KMtronic と Numato は、現在の設定を問い合わせることもできます
settings-relay-port = ポート
settings-relay-choose = — 選択 —
settings-relay-device = デバイス
settings-relay-gpio-pin = GPIO ピン
settings-relay-pin-3-every-homebrew-plan = すべての自作プランとすべての市販ボードで、ピン 3 を使ってください。パッケージの端にあり、配線を付けやすいピンです。リレーがカチッと鳴らない場合、そのカードはこれらのピンをまったく実装していない互換品です。
settings-relay-gpio-chip = GPIO チップ
settings-relay-linux-only-gpiodetect-lists-chips = Linux のみ。`gpiodetect` がマシン上のチップを一覧表示します。
settings-relay-transmit = 送信時
settings-relay-receive = 受信時
settings-relay-nothing-switched = 何も切り替えません。
settings-relay-starting-program-costs-unpredictable-amount = プログラムの起動には予測できない時間がかかり、その時間はリードに含まれません。そのため、リレーボードの場合より余裕を大きく取ってください。接点表の数値はこの回線では意味がありません。コマンドはキーダウン時とキーアップ時に 1 回ずつ実行されます。
settings-relay-contacts = 接点
settings-relay-contact-1-rts-contact-2 = 接点 1 は RTS、接点 2 は DTR です。
settings-relay-no = 番号
settings-relay-gpio = GPIO
settings-relay-name = 名前
settings-relay-job = 役割
settings-relay-tx-closes = 送信時に閉じる
settings-relay-lead = リード
settings-relay-hold = ホールド
settings-relay-line-s-offset-chip-bcm = このチップ上の回線のオフセット。Raspberry Pi のヘッダーでは BCM 番号であり、物理ピン番号ではありません
settings-relay-sdr-antenna = SDR アンテナ
settings-relay-decides-default-timings-what-log = 既定のタイミングと、ログでの呼び方を決めます。バンドデコーダーの場合は、接点が下のバンドテーブルに従うかどうかも決めます
settings-relay-transmitting-energises-coil-off-transmitting =
    オン：送信時にコイルが励磁されます。オフ：送信時にコイルが解放されます。

    This is a wiring decision before it is a setting — see the note below.
settings-relay-how-long-before-rf-contact = RF より何時間前に、この接点がバンドの受信用の状態から送信用の状態へ動くか。再同調では即座に動きます。この値はキーダウン時のもので、送信専用フィルターが電力より先に回路に入っている必要があるためです。
settings-relay-closed-long-before-rf-small = RF より前にこの時間だけ閉じます。小型の同軸リレーは 5〜15 ms で切り替わります。送信はこれらの最長を待つため、大きな値は受信音声の途切れとして聞こえます。250 ms を超える値は無視されます。
settings-relay-how-long-after-rf-stops = RF が止まってから、この接点が送信用の状態から受信用の状態へ戻るまでの時間です。
settings-relay-opened-long-after-rf-stops = RF が止まってから、この時間だけ後に開きます。意図的にリードより長くしています。アンテナを少し遅れて戻すのは何の害もありませんが、早く戻すとフロントエンドを損ないます。0 も正しい答えで、アンプのキーラインではこれが適切です。
settings-relay-test = {" "}テスト{" "}
settings-relay-not-while-station-air = ステーションが電波を出している間は使用できません
settings-relay-close-contact-half-second-so = この接点を 0.5 秒間閉じるので、送信機が停止した状態でリレーの音を聞き、配線を確認できます
settings-relay-remove = 削除
settings-relay-add-contact = + 接点を追加
settings-relay-if-will-not-answer = 応答しない場合
settings-relay-switch-exists-protect-receiver-not = 受信機を守るためのスイッチは、黙って守らなくなるなら価値がありません。そのため、既定では拒否します。USB ケーブルの接触不良でコンテストが終わってしまうほうが悪い結果になる場合は、もう一方を選んでください。
settings-relay-order-timings-longest-lead-closes = 順序はタイミングで決まります。最も長いリードが最初に閉じ、最も長いホールドが最後に開きます。そのため、アンテナリレーに長いリードと長いホールドを、アンプには各々の短い方を与えてください。そうすると、アンプがキーされる前に必ずアンテナが切り替わり、アンテナが戻る前に必ずアンプのキーが解除されます。
settings-relay-transmit-sense-input = 送信検出入力
settings-relay-when-transceiver-keys-itself-own = トランシーバーが自分でキーされたとき（マイクのボタン、フットスイッチ、VOX、キーヤーなど）、sdroxide は CAT で問い合わせて知ります。その答えは、交信が始まってから数百ミリ秒後に届きます。無線機の SEND 回線を、フォトカプラーを介してこの同じポートのハンドシェイク入力に配線すると、数ミリ秒で検出できます。メーターと送信インターロックもそれに伴って速くなります。
settings-relay-line = 回線
settings-relay-transmitting = 送信中は
settings-relay-high-line = 高レベルの回線
settings-relay-most-opto-isolated-interfaces-pull = ほとんどのフォトアイソレーション・インターフェースは、無線機がキーされると回線を下げるため、通常はオフです
settings-relay-belongs-radio = 所属する無線機
settings-relay-which-radio-tab-sensed-transceiver = 検出されたトランシーバーがどの無線機タブかです。無線機が 1 台のステーションでは 1 で、他には影響しません。
settings-relay-needs-serial-port-handshake-lines = シリアルポートが必要です。ハンドシェイク回線がそこにあります。
settings-relay-wiring-only-fail-safe-survives = このプログラムが終了しても残る唯一のフェイルセーフは配線です。
settings-relay-choose-tx-closes-so-de = 「送信時に閉じる」を選び、ボードが抜かれたとき、コンピューターの電源が切れたとき、または sdroxide が動いていないときに、*励磁されていない*接点が望ましい状態になるようにしてください。励磁されていないときに接地される配線にすると、リレーが故障してもSDR は聞こえなくなります。迷惑ですが安全です。逆に配線すると、リレーの故障と 1 回の送信でフロントエンドが壊れます。背後に本物のアンプがある場合は、前者を選んでください。
settings-relay-cannot-protect-first-instant-over = これは、無線機で始めた交信の最初の瞬間を守ることはできません。sdroxide からキーすると、接点は必ず RF より先行します。上の検出入力を配線すると、数ミリ秒以内に追従します。本当に守る価値のあるフロントエンドには、RF で検出するハードウェアの T/R スイッチも併用してください。コンピューター上のプログラムだけでは、完全な答えにはなりません。
settings-relay-contacts-transmit = 送信中の接点
settings-relay-not-open = 開いていません。
settings-relay-status-unknown-press-apply = 状態不明です。適用を押してください。
settings-relay-station-air-apply-will-take = ステーションは電波を出しています。適用はこの交信の後に有効になります。
settings-relay-band-decoder-outputs = バンドデコーダーの出力
settings-relay-one-control-word-per-band = バンドごとに 1 つの制御ワードです。ビット 0 は接点 1、ビット 1 は接点 2 というように、上の番号と対応します。RX は受信ダイヤルのバンドに従います。TX は送信したほうの無線機のバンドに従います。各接点は RF の前にリードで切り替わり、ホールドの後に戻ります。RF の向きを気にしないフィルターには同じ値を、受信専用のプリアンプのバイパスや送信専用の LPF バンクには異なる値を与えてください。0 のままのバンドは何も出力しません。適用 / 再接続で有効になります。
settings-relay-band = バンド
settings-relay-rx = RX
settings-relay-tx = TX
settings-relay-outputs-asserted = 出力中
settings-relay-other = その他
settings-relay-everywhere-outside-amateur-bands-short = アマチュアバンドの外すべて。短波の受信や、トランスバーターのダイヤルがどのバンドにも属さない場合を含みます。
settings-relay-apply = {" "}適用{" "}
settings-relay-persist-re-open-switch = 保存して（再）オープン

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = サーバーに接続
settings-remote-drive-sdroxide-running-somewhere-else = 別の場所で動いている sdroxide（シャックのマシン、リモートの拠点など）をネットワーク越しに操作します。そちらで --server を付けて起動し、そのアドレスをここに入力してください。無線機、デコーダー、ログブックはそのマシンに残ります。この画面は波形と音声を受け取り、使うのはこちらのスピーカーとマイクです。
settings-remote-address = アドレス
settings-remote-host-name-ip = ホスト名または IP
settings-remote-machine-running-sdroxide-server-host = sdroxide --server を実行しているマシンです。ホスト名、IPv4 または IPv6 アドレス、もしくはサーバーがリバースプロキシの背後にある場合は完全な ws://（または wss://）URL を指定します。URL は入力どおりに使われ、ポートと下のスイッチは無視されます。
settings-remote-port = ポート
settings-remote-port-server-listens-server-port = そのサーバーが待ち受けるポートです。その config.toml の server_port です。--port を指定しない限り 4950 です。
settings-remote-secure = セキュア
settings-remote-wss-tls = wss://（TLS）
settings-remote-tick-where-server-sits-behind = サーバーの前段で HTTPS を終端している場合にチェックします。たとえばポート 443 のリバースプロキシで、インターネット越しにステーションへ接続する一般的な方法です。sdroxide 自身のサーバーは平文の ws:// を使うため、これはチェックしません。
settings-remote-connect = {" "}接続{" "}
settings-remote-open-station-radio-tab-own = このステーションを独自の無線機タブとして開く
settings-remote-client-cannot-hold-second-connection = このクライアントは 2 つ目の接続を保持できません。

    If the server asks for a username and password, its sign-in screen comes up in the new tab. A plain ws:// link carries none of this encrypted, so across the open internet either put it through a VPN or an SSH tunnel, or terminate HTTPS in front of the server and tick Secure above.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = WSJT-X の UDP ブロードキャスト
settings-servers-waiting-station-s-wsjt-x = ステーションの WSJT-X ブロードキャスト構成を待っています…
settings-servers-sends-decodes-station-status-logged = WSJT-X と同じ方法で、デコード、ステーションの状態、記録された QSO を送信します。そのため、GridTracker、JTAlert、N1MM+、Log4OM は sdroxide をそのまま使えます。出力のみで、このソケットから無線機を操作することはできません。
settings-servers-enable = 有効
settings-servers-send = 送信先
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 はこのマシン上のクライアントに届きます。LAN アドレスやマルチキャストグループ（224.0.0.1）は他のマシンに届きます
settings-servers-port = ポート
settings-servers-t-2237-port-every-client-defaults = 2237 は、すべてのクライアントが既定で使うポートです
settings-servers-identify = 名乗る名前
settings-servers-name-clients-see-some-loggers = クライアントに表示される名前です。一部のロガーは、WSJT-X として自己識別するトラフィックしか受け付けません。
settings-servers-n1mm-contactinfo-broadcast = N1MM+ contactinfo のブロードキャスト
settings-servers-same-news-n1mm-s-words = N1MM の形式で同じ情報を送ります。記録された交信ごとに 1 つの XML データグラムで、WSJT-X ではなく N1MM を待ち受けるロガー向けです。World Radio League のブリッジはどちらも受け付けます。交信のみで、このプロトコルにデコードのストリームはありません。ステーションが両方を必要とする場合があるため、独自のスイッチと独自のポートがあります。
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 はこのマシン上のクライアントに届きます。コンテストのネットワークについて N1MM 自身が勧めるのは、このサブネットのブロードキャストアドレスです。192.168.1.n のネットワークなら 192.168.1.255 で、すべての席に一度に届きます。
settings-servers-t-12060-port-n1mm-s-documentation = 12060 は、N1MM のドキュメントが推奨するポートです
settings-servers-station-name = ステーション名
settings-servers-what-n1mm-calls-stationname-name = N1MM が StationName と呼ぶもの：パケットを送ったコンピューターの名前です。ロガーは、どの運用席かを区別するためにこれを表示します。
settings-servers-cw-panel-s-copy-plain = CW パネルの受信文字を、確定するたびに 1 件の UDP データグラムずつ平文で送信します。ステーションのコントローラー、ロガーの CW ウィンドウ、スクリプト向けです。独自のプロトコルはなく、受信側は届いたものを追記するだけです（nc -ul 9999 などで試せます）。確定したテキストのみを送り、デコーダーがまだ書き直している末尾は送りません。無線機ごとに独自のスイッチがあります。
settings-servers-apply = {" "}適用{" "}
settings-servers-persist-re-open-broadcast-sockets = 保存して（再）オープン、ブロードキャストソケット
settings-servers-hamlib-rigctld-server = Hamlib rigctld サーバー
settings-servers-waiting-station-s-rigctld-configuration = ステーションの rigctld 構成を待っています…
settings-servers-listen = 待ち受けるアドレス
settings-servers-t-127-0-0-1-machine = 127.0.0.1（このマシン）
settings-servers-t-0-0-0-0-whole = 0.0.0.0（ネットワーク全体）
settings-servers-t-4532-port-every-rigctld-client = 4532 は、すべての rigctld クライアントが想定するポートです
settings-servers-rig-name = 無線機名
settings-servers-reported-clients = クライアントへ報告する名前
settings-servers-max-clients = 最大クライアント数
settings-servers-allow-clients-transmit = クライアントに送信を許可
settings-servers-off-refuses-every-key-request = オフにすると、すべてのキー要求を拒否し、送信範囲の通知も止めるため、Hamlib 自身がキーを拒否します。
settings-servers-rigctld-has-no-authentication-0 = ⚠ rigctld には認証がありません。0.0.0.0 では、このポートに到達できる人は誰でもあなたの送信機を同調させてキーできます。
settings-servers-t-1-client = クライアント 1 件
settings-servers-not-running = 停止中です。
settings-servers-status-unknown-press-apply = 状態不明です。適用を押してください。
settings-servers-persist-re-bind-server = 保存して（再）バインド、サーバー
settings-servers-lets-any-hamlib-capable-program = Hamlib に対応したどのプログラムからもこの無線機を操作できます。周波数、モード、PTT、スプリット、RIT、出力が対象です。WSJT-X、fldigi、CQRLOG では、無線機として「Hamlib NET rigctl」（モデル 2）を選び、このアドレスを指定します。GPredict と N1MM では、ホストとポートを直接入力します。TCI サーバーと異なり、制御のみで、音声や IQ は含みません。
settings-servers-antenna-rotator = アンテナローテーター
settings-servers-waiting-station-s-rotator-configuration = ステーションのローテーター構成を待っています…
settings-servers-host = ホスト
settings-servers-min-elevation = 最低仰角
settings-servers-below-rotator-parks-instead-tracking = これより低い場合、ローテーターは追跡せず停止します。地元の地平線や屋根の稜線に合わせてください
settings-servers-azimuth-offset = 方位オフセット
settings-servers-added-every-command-rotator-whose = すべてのコマンドに加算されます。北がずれているローテーター用です
settings-servers-min-movement = 最小移動量
settings-servers-steps-smaller-than-are-not = これより小さいステップは送信されません。0.1 度単位の追従はモーターを無駄に摩耗させます
settings-servers-park-position = 停止位置
settings-servers-drive-position-when-idle = 待機中は指定の位置へ動かす
settings-servers-off-leaves-antenna-wherever-last = オフにすると、アンテナは最後のパスが終わった位置に残ります
settings-servers-not-connected = 接続されていません。
settings-servers-waiting-station-s-tci-server = ステーションの TCI サーバー構成を待っています…
settings-servers-built-tci-server = 内蔵の TCI サーバー
settings-servers-device-name = デバイス名
settings-servers-off-leaves-control-receive-streams = オフにすると、制御と受信ストリームは動作しますが、すべてのキー要求は拒否されます。
settings-servers-tci-has-no-authentication-0 = ⚠ TCI には認証がありません。0.0.0.0 では、このポートに到達できる人は誰でもあなたの送信機を同調させてキーできます。

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = 衛星：軌道要素と周波数
settings-tle-waiting-station-s-satellite-configuration = ステーションの衛星構成を待っています…
settings-tle-tracker-already-fetches-celestrak-s = トラッカーは、CelesTrak のアマチュア衛星グループを自動で既に取得しています。これは、それ以外のもの向けです。NOAA の気象衛星、そのグループに入るには新しすぎる CubeSat、または届いたものより新しい軌道要素などです。リストは、無線エンジンが動いている場所で取得され、保存されます。そのため、アプリがどこで開かれていても、同じ衛星の集合になります。
settings-tle-subscriptions = 購読
settings-tle-listings-fetched-kept-current-same = アマチュア衛星の集合と同じ 6 時間ごとの周期で、取得し最新に保たれます。太陽ウィンドウが開いている間に更新され、ここの「今すぐ更新」でも更新されます。
settings-tle-fetch-track-listing = このリストを取得して追跡
settings-tle-remove-subscription = この購読を削除
settings-tle-orbits = 軌道
settings-tle-which-satellites-listing-get-orbit = このリストのどの衛星に、軌道のリングとラベルを付けるかです。グループ全体を指定する場合は「キュレーション済み」にしてください。90 のリングが同時に表示されると読めず、まったく表示しないと 90 個の無名の点が残るだけです。
settings-tle-nothing-listing-sdroxide-s-curated = このリストにあるものは、sdroxide のキュレーション済み一覧に入っていません。その一覧はアマチュア衛星 10 個のため、これは「なし」と全く同じ動作になります。
settings-tle-all-satellites = すべての衛星
settings-tle-catalogue-numbers-keep-comma-separated = 残すカタログ番号（カンマ区切り）。空欄の場合はリストに含まれるすべてを追跡します。
settings-tle-not-fetched-yet = まだ取得していません
settings-tle-subscription = + 購読
settings-tle-update-now = {" "}今すぐ更新{" "}
settings-tle-ask-radio-engine-fetch-every = 有効なすべての購読を今すぐ取得するよう無線エンジンに依頼します
settings-tle-celestrak-groups = CelesTrak グループ
settings-tle-pasted-element-sets = 貼り付けた軌道要素
settings-tle-one-off-these-do-not = 一度だけ使う場合向けです。これらは自動更新されず、SGP4 は軌道要素の元期から 2 週間を過ぎると伝播計算を止めます。維持したいものは、代わりに購読してください。
settings-tle-track-one = この 1 件を追跡
settings-tle-show-two-element-lines = 2 行の軌道要素を表示
settings-tle-remove = 削除
settings-tle-paste-two-three-line-element = 2 行または 3 行の軌道要素をここに貼り付けてください
settings-tle-add-pasted = + 貼り付けたものを追加
settings-tle-nothing-paste-box-looked-like = 貼り付け欄の内容は、どれも軌道要素には見えませんでした。
settings-tle-clear-box = 欄をクリア
settings-tle-frequencies = 周波数
settings-tle-shown-under-pass-table-solar = 太陽ウィンドウのパス表の下に表示されます。ここの項目は、そのカタログ番号について内蔵の項目を完全に置き換えるため、他を捨てるつもりでなければ、内蔵のコピーから始めてください。
settings-tle-remove-satellite-s-entry = この衛星の項目を削除
settings-tle-link = LINK
settings-tle-downlink = ダウンリンク
settings-tle-uplink = アップリンク
settings-tle-mode = モード
settings-tle-note = 備考
settings-tle-fm-repeater = FM レピーター
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67.0 Hz
settings-tle-link-2 = + リンク
settings-tle-copy-built = 内蔵のものをコピー
settings-tle-frequency-either-one-number-145 = 周波数は、1 つの数値（145.800）か、145.950-145.970 のように書かれたトランスポンダーの帯域のいずれかです。ビーコンの場合は、方向を空欄にしてください。
settings-tle-norad = NORAD
settings-tle-satellite = + 衛星
settings-tle-frequency-entry-needs-satellite-s = 周波数の項目には、衛星の NORAD カタログ番号が必要です。

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = アドレス
settings-wavelog-api-key = API キー
settings-wavelog-your-wavelog-s-front-page = Wavelog のフロントページのアドレスです。https://log.example.org や http://192.168.1.10/wavelog など。API ページから取得したキーを入力してください。アップロードには読み書き可能なキーが必要です。
settings-wavelog-plain-http-address-internet-api = インターネット上のアドレスへの平文の http:// は、API キーが暗号化されずに送られます。自分のネットワーク内なら問題ありません。インターネット越しには https:// を使ってください。
settings-wavelog-trusted-cert = 信頼された証明書
settings-wavelog-forget-certificate = この証明書を削除
settings-wavelog-go-back-ordinary-checking-against = 公開の認証局に対する通常の確認に戻します。適切な証明書を持つサーバー用です。
settings-wavelog-connection-check-runs-machine-radio = 接続確認は、無線機が接続されているマシンで実行されます。そこの設定でテストしてステーションの所在地を一覧できます。
settings-wavelog-server-presented-certificate-sha-256 = サーバーはこの証明書を提示しました（SHA-256）：
settings-wavelog-compare-your-server-s-e = サーバーの値と照合してください。例：`openssl x509 -noout -fingerprint -sha256 -in cert.pem`。一致する場合にのみ信頼してください。以後、この証明書だけが受け入れられ、それ以外は拒否されます。
settings-wavelog-trust-certificate = この証明書を信頼
settings-wavelog-station-location = ステーションの所在地
settings-wavelog-none = — なし —
settings-wavelog-press-test-above-list-station = 上のテストを押すと、Wavelog に定義されたステーションの所在地を一覧表示します。
settings-wavelog-default = 既定
settings-wavelog-where-qsos-go-when-operating = 運用プロファイルで場所が指定されていない場合に、交信を送る先です。
settings-wavelog-profile = このプロファイル
settings-wavelog-use-default = 既定を使う
settings-wavelog-not-set = 未設定
settings-wavelog-no-station-location-chosen-uploads = ステーションの所在地が選ばれていません。選ぶまでアップロードは失敗します。
settings-wavelog-qsos-not-wavelog = Wavelog にない交信
settings-wavelog-every-qso-logbook-wavelog = ログブックのすべての交信は Wavelog にあります。
settings-wavelog-everything-logbook-wavelog-has-not = Wavelog が確認していないログブックの内容です。自動アップロードより前の交信も含みます。既に Wavelog にあるものは重複として戻り、送信済みとして印が付くだけです。
settings-wavelog-set-address-key-station-location = 先にアドレス、キー、ステーションの所在地を設定してください。
settings-wavelog-yes-send-them = はい、送信します
settings-wavelog-cancel = キャンセル
settings-wavelog-download-into-logbook = このログブックにダウンロード
settings-wavelog-downloading-runs-where-radio-attached = ダウンロードは無線機が接続されているマシンで実行され、そのマシンのログブックに入ります。そちらで設定してください。
settings-wavelog-download-qsos-start-every = 交信のダウンロードを開始時と次の間隔で行う
settings-wavelog-downloaded-qsos-join-logbook-so = ダウンロードした交信はログブックに加わるため、FT8 とスポット一覧は、他のプログラムや無線機で交信済みの局を把握できます。各ダウンロードは新しいものだけを取得します。
settings-wavelog-press-test-above-choose-which = 上のテストを押すと、ダウンロードするステーションの所在地を選べます。
settings-wavelog-sync-now = 今すぐ同期
settings-wavelog-fetch-what-wavelog-has-added = 前回のダウンロード以降に Wavelog が追加したものを取得します。先に上の設定を適用します。
settings-wavelog-full-resync = 完全な再同期…
settings-wavelog-download-ticked-locations-whole-compare = チェックされた所在地を丸ごとダウンロードし、ログと比較します。Wavelog で編集または削除された交信や、チェックが外れた所在地からの交信は、一致するように更新されます。何かが起きる前に変更内容が表示されます。
settings-wavelog-full-resync-changes-make = 完全な再同期：行う変更
settings-wavelog-qsos-logged-here-imported-file = ここで記録された交信や、ファイルから読み込んだ交信は、決して変更も削除もされません。変更されるのは、Wavelog からダウンロードされたものだけです。
settings-wavelog-apply = 適用
settings-wavelog-discard = 破棄
settings-wavelog-live-radio-status = 無線機のライブ状態
settings-wavelog-send-frequency-mode-wavelog-they = 周波数とモードが変わるたびに Wavelog へ送信する
settings-wavelog-radio-name = 無線機名
settings-wavelog-pick-radio-wavelog-s-live = Wavelog のライブログで、この無線機を選ぶと、そのフォームがダイヤルに追従します。読み書き可能なキーが必要です。ステーションのメインの無線機からのみ有効です。

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = サーバー証明書の SHA-256：{ $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = ステーションを「{ $name }」に戻します。ダイヤル、VFO、モード、フィルター、ゲイン、駆動、アンテナ、識別情報、バンドスタックです。
settings-profiles-delete = 「{ $name }」を削除

## Settings window, T/R switch: messages with values

settings-relay-not-usable = まだ使用できません：{ $why }
settings-relay-receiving = 受信中
settings-relay-rx-tx-words = { $rx } / TX { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n } クライアント
settings-servers-listening = ● { $addr } で待ち受け中 — { $clients }
settings-servers-failed = 失敗：{ $error }
settings-servers-rotator-at = ● 接続済み — アンテナは方位 { $az }°、仰角 { $el }°

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } 衛星 · { $age } 経過
settings-tle-subscribed = { $name } を購読しました。取得するには今すぐ更新を押してください。
settings-tle-norad-too-stale = NORAD { $norad } · { $age } 経過 — 古すぎて伝播できません
settings-tle-norad-age = NORAD { $norad } · { $age } 経過
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = { $n } 件の軌道要素を追加しました。
settings-tle-added-refreshed = { $added } 件を追加し、{ $r } 件の軌道要素を更新しました。
settings-tle-links = { $n } 件のリンク
settings-tle-replace-links = これらのリンクを { $name } の内蔵のものに置き換える

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = 所在地「{ $location }」は { $station_call } ですが、あなたは { $call } で記録しています。Wavelog はこれらの交信を拒否します。
settings-wavelog-grid-mismatch = 所在地「{ $location }」は { $station_grid } にありますが、あなたのグリッドは { $grid } です。Wavelog はこれらの交信を拒否します。
settings-wavelog-location-n = 所在地 #{ $id }
settings-wavelog-saved-with-callsign = 設定 → プロファイルでコールサイン（{ $call }）とともに保存されるため、ポータブルなプロファイルはポータブルな所在地に記録できます。
settings-wavelog-owed = 自動アップロードをオンにしてから記録された { $n } 件の交信が、まだ送信されていません。5 分ごとに再試行されます。
settings-wavelog-send-unsent = Wavelog にまだない { $n } 件の交信を送信…
settings-wavelog-send-confirm = { $location } へ { $n } 件の交信を送信しますか？ 先に設定を適用します。
settings-wavelog-plan-add = • Wavelog にある { $n } 件の交信をログに追加
settings-wavelog-plan-update = • Wavelog で変更されたダウンロード済みの { $n } 件の交信を更新
settings-wavelog-plan-remove = • Wavelog にない（またはチェックが外れた所在地の）ダウンロード済みの { $n } 件の交信を削除
settings-wavelog-plan-mark = • 自分の交信のうち { $n } 件を Wavelog にあるものとして印を付ける

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = アンテナ
settings-radio-which-socket-back-radio-using =
    無線機が背面のどのソケットを使うか。無線機自身の ANT コマンドで、前面パネルの ANT ボタンと同じ設定です。

    Applies immediately, and it is the radio's setting rather than a copy kept here: the socket the radio is on is read back when the session opens.

    The choice is remembered per band. Switch to the beam on 2 m and the wire on 40, and each comes back the next time the dial crosses into that band — and the next time sdroxide starts. A memory channel stored here carries its socket too.
settings-radio-receive-antenna = 受信アンテナ
settings-radio-radio-s-separate-receiving-antenna =
    無線機の独立した受信アンテナ（背面の RX ANT）を、受信経路に入れるか外すかです。無線機自身の設定で、無線機の RX ANT ボタンと同じものです。

    The aerial on the main socket stays on transmit either way: this is an extra input for receiving, a loop or a beverage, not a choice of socket.

    Unlike the antenna above, this is *not* remembered here. The radio holds it per band itself, so sdroxide reads it back after every band change and shows what the radio says; ticking this box is the only thing that moves it.
settings-radio-receive-path = 受信経路に入れる
settings-radio-radio-power = 無線機の電源
settings-radio-switch-radio-itself-off-back =
    無線機自体の電源を、制御リンク経由で切り、再び入れます。これはプログラム内で唯一の本当の電源スイッチです。sdroxide 自身の LINK スイッチ（sdroxide 側を閉じ、無線機は動作したまま）とは異なります。

    For the switch back on to reach anything, the radio's control end has to stay awake while it is off. Over the network that is what Network Control does. Over a serial cable it is the radio's CI-V port, which stays powered from the mains supply on a set that is switched off at the front rather than unplugged; sdroxide sends the wake-up run Icom's own documentation asks for in front of the power-on.

    Switching off ends the audio and the meters — there is no radio behind them — and sdroxide keeps the control link open so the switch back on has somewhere to go.
settings-radio-on = オン
settings-radio-off = オフ
settings-radio-waiting-configuration-machine-radio-attached = 無線機が接続されているマシンの構成を待っています。
settings-radio-sound-format = 音声フォーマット
settings-radio-invert-spectrum = スペクトラムを反転
settings-radio-swap-i-q = I/Q を入れ替え
settings-radio-mirror-panadapter-about-tuned-frequency =
    同調周波数を軸にパナダプターを反転します。音声カードで I と Q が逆に入っている無線機用です。手がかりは、ウォーターフォールに信号が多数あるのに、すべてがダイヤルの反対側にあり、SSB が反対側の側波帯に出ることです。音声カードでケーブルの 2 本を入れ替えても同じように直ります。

    Receive only: transmit hands the radio one real audio signal, which has no sideband to invert.
settings-radio-i-q-sample-rate = I/Q サンプルレート
settings-radio-how-fast-radio-s-i =
    無線機の I/Q 音声カードをどの速さで動かすかです。これによりパナダプターに表示されるバンド幅も決まります。直交信号のストリームは、サンプルレート全体を占めるためです。48 kHz ではダイヤルの両側に ±24 kHz、192 kHz では ±96 kHz になります。

    The card has the final say. One that cannot do the rate picked here is opened at the nearest it can, and the panadapter is that much narrower — the log says so at startup when it happens.

    A faster card is also more work: the machine has proportionally less time to empty it, and one that cannot keep up drops samples. That shows up as audio breaking up while the waterfall still looks perfect, and it too is logged. If you see it, come back down a step.
settings-radio-t-48-khz-24-khz = 48 kHz（±24 kHz）
settings-radio-t-96-khz-48-khz = 96 kHz（±48 kHz）
settings-radio-t-192-khz-96-khz = 192 kHz（±96 kHz）
settings-radio-t-384-khz-192-khz = 384 kHz（±192 kHz）
settings-radio-i-q-centre-offset = I/Q の中心オフセット
settings-radio-how-far-radio-s-i =
    無線機の I/Q 出力が、自分のダイヤルより上のどれだけ中心に置かれるかです。受信の中間周波数をゼロからずらした無線機用です。Elecraft では、メニューの RX SHFT を NOR ではなく 8.0 に設定します。これによりダイヤルがミキサーの DC スパイクから外れ、近くにある強い SSB/AM 局が AM 検波されなくなります。

    Not a converter: the radio still displays and transmits on the real frequency, and nothing here is ever sent to it. This only says where the samples on the sound card sit, and the stream is shifted back onto the dial as it arrives.

    Leave at 0 unless you have turned such a menu entry on. If signals land twice the offset away, the sign is the other way round.
settings-radio-iq-correction = IQ 補正
settings-radio-cancel-mirror-image-every-signal =
    すべての信号のイメージ（鏡像）と、ウォーターフォール中央の DC スパイクを消去します。

    A radio's I/Q output is two analogue paths that are never quite equal in gain nor exactly 90° apart, and what that leaves is a copy of every signal reflected about the tuned frequency, usually 30-40 dB down — strong enough to look like a station that is not there, and to be decoded as one. Radios with front-panel balance trimmers are adjusting exactly this; here it is measured off the received noise and needs no setting.

    Leave it on. Turn it off if you are listening to AM tuned dead on the carrier — the carrier is DC, so it goes with the spike — or to check whether a signal is real by watching whether it survives.
settings-radio-cancel-mirror-images = イメージを消去
settings-radio-dc-notch = DC ノッチ
settings-radio-widen-hole-taken-out-middle =
    スパンの中央から取り除く穴を広げます。中央のスパイクが、その下のオフセットよりも広い無線機用です。

    0 leaves the ordinary blocker, which is a few tens of hertz wide and removes the offset without touching anything else. Wind it up and the bottom of the span goes with it: a first-order high-pass, 3 dB down at the figure set here and further in below it.

    It is centred where the radio's I/Q is centred, which is the dial unless the offset above has moved it — so anything tuned there goes too. A CW note at 600 Hz is inside a 600 Hz setting.
settings-radio-panadapter-bw = パナダプターの帯域幅
settings-radio-cat-family = CAT ファミリー
settings-radio-rigctld-address = rigctld のアドレス
settings-radio-host-port-running-hamlib-rigctld =
    実行中の Hamlib rigctld のホスト:ポートです。このマシン上では、既定の 127.0.0.1:4532 です。

    Start one with, for example, `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, where -m is the Hamlib model number for your radio (`rigctl -l` lists them).

    This is the catch-all: it reaches the frequency, mode, PTT, power, S-meter and SWR and nothing else. Where one of the native families above fits your radio it does more — keying from the rig's own text buffer, the receive filter, and per-model meter scales.
settings-radio-flrig-address = flrig のアドレス
settings-radio-host-port-running-flrig-127 =
    実行中の flrig のホスト:ポートです。このマシン上では、既定の 127.0.0.1:12345 です。flrig は実行中ならいつでも XML-RPC を提供します。ポートは flrig の Config → Setup → Server にあります。

    Like the Hamlib option this drives a daemon rather than the radio, but through flrig's own per-model driver — on a number of rigs its power and filter handling is the more faithful of the two. It reaches the frequency, mode, PTT, transmit power (in whole watts), the receive bandwidth, the S-meter, SWR and power-out. CW keys through flrig's own cwio port, which must be configured in flrig itself.
settings-radio-serial-port = シリアルポート
settings-radio-select = — 選択 —
settings-radio-baud = ボーレート
settings-radio-data-bits = データビット
settings-radio-parity = パリティ
settings-radio-stop-bits = ストップビット
settings-radio-force-rts = RTS を強制
settings-radio-force-dtr = DTR を強制
settings-radio-ptt-method = PTT 方式
settings-radio-how-transmit-keyed = 送信をどうキーするか。
settings-radio-how-transmit-keyed-network-link = 送信をどうキーするか。ネットワーク接続には制御線がないため、ここでは DTR と RTS ではキーできません。デーモンに無線機のキーを頼む CAT か、VOX を使ってください。
settings-radio-mode-control = モード制御
settings-radio-digimode-mode = デジタルモードのモード
settings-radio-what-put-radio-mode-sdroxide =
    sdroxide が音声カード経由で変調するモードのために、無線機をどのモードにするかです。「DIGI」は無線機の DATA/PKT の位置を選びます。これは USB または ACC 入力から送信音声を取り、マイク経路の音声処理を通しません。「USB」は通常の側波帯のままにします。SSB を DATA 入力から変調するよう設定済みの無線機、または DATA の位置がまったくない無線機用です。

    It covers every digital mode — FT8, FT4, PSK, RTTY, SSTV and the rest — and CW sent as "Sound card", and it overrides Mode control for them. SSTV is the one whose sideband follows the band, so there it means the DATA position on that sideband: USB-D above 40 m, LSB-D at and below it. SSTV-FM is not part of it — an FM carrier has no sideband to choose.
settings-radio-cw-keying = CW キーイング
settings-radio-how-cw-panel-s-keyer =
    CW パネルのキーヤーがどう送信するかです。「無線機のキーヤー」は、無線機を CW にして、送るテキストを無線機自身のキーヤーに渡します。パネルの WPM から設定されるリグのキーヤー速度を使い、Yaesu では、キーヤーメモリー 1 を経由して送信します。そこに保存されていた内容は上書きされます。

    It needs break-in on at the radio, which sdroxide turns on itself with each message on Icom, Yaesu and Kenwood — a rig with break-in off takes the message and transmits nothing. It is left on afterwards, and a full break-in is never turned down to semi.

    "Sound card" sends the keyed sidetone as audio instead (MCW), a tone at dial + pitch — and because a rig in CW would ignore its sound card entirely, selecting CW then follows the Digimode mode setting (USB, DATA, or Radio controlled) instead of switching the rig to CW. Pick "Radio controlled" there for rigs whose data position can't be commanded over CAT (a Xiegu's U-D) and park the rig on it yourself, as for FT8.
settings-radio-poll-rate = ポーリング頻度
settings-radio-how-often-radio-asked-what =
    無線機に、何をしているか（ダイヤル、モード、メーター）を問い合わせる頻度です。これは制御リンクのうち、無線機から sdroxide へ向かう半分です。リグ自身の VFO つまみを回したり、前面パネルでモードを変えたりすると、表示、バンド、パナダプターが 1 回のポーリング以内に追従します。

    It is also the whole of the control traffic this end generates, and on a modern Icom that traffic is not free: the CI-V port and the sound card sit behind one USB hub inside the radio, so frames asked for here are bus time the audio does not get — heard as dropouts that look like a DSP fault and are not one. The default of 2 Hz is half a second behind the knob and quiet enough to stay out of the audio's way. Raise it where the control port is its own device (a separate USB-serial adapter, a network rigctld); lower it if the radio's audio breaks up.

    An Icom with CI-V Transceive switched on in its own menu needs none of this for the dial: it reports the knob the instant it moves, and sdroxide stands the dial poll down as soon as it sees it do so — and picks it back up again if the radio ever moves without saying so.

    The rate is the dial's. The mode rides along with only every fourth poll, since it is a setting that changes a few times in an evening rather than one that follows a knob.
settings-radio-send-command = 送信コマンド
settings-radio-which-transceiver-generation-keys-rig =
    PTT 方式「CAT」で、どの世代のトランシーバーが無線機をキーするかです。2 つは TX パラメーターの意味が異なり、回線上にはそれを区別する情報がありません。お使いの無線機に合うものを選んでください。

    "TS-2000 style (TX;)" — TS-480, TS-570, TS-870, TS-2000: the ordinary send, on the main band.

    "TS-590 style (TX1;)" — TS-590S/SG, TS-890, TS-990: DATA SEND, which keys with the ACC2/USB audio input live. On these rigs the plain send selects the microphone instead and mutes the audio sdroxide transmits.

    Set wrong, a TS-590 transmits silence — but a TS-2000 transmits on the sub-band, which is another band entirely.
settings-radio-radio = 無線機
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    1 つのプロファイルでファミリー全体を扱います。KX3、KX2、K4 がすべて応答する K3 のコマンドセットです。ここで選ぶものはありません。ドライブのスライダーが何ワットにわたるか（素の KX2 や KX3 では 12、KPA3 や KXPA100 では 110）は、ポートを開いたときに無線機自身のオプションモジュールの問い合わせから読み取られます。

    Note the baud rates above: the K3, K3S, KX3 and KX2 go no faster than 38400, and a rig set below the rate chosen here answers nothing at all.
settings-radio-transmit-input = 送信入力
settings-radio-where-radio-takes-transmit-audio =
    無線機が送信音声をどこから取るかです。リグの TI コマンドで、前面パネルのメニュー 32「TX IN」にあたります。ポートを開いたときに設定されます。

    "USB audio" is what makes this interface work: the FDM-DUO transmits what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.

    "Auto" lets the rig choose — the microphone for a PTT press on the microphone, the USB port for a CAT or RTS key-down.

    "Leave as set on the radio" sends no TI at all, for an operator who sets this at the front panel.
settings-radio-which-two-sockets-back-receiver =
    受信機が背面の 2 つのソケットのどちらで受信するかです。リグの AN コマンドで、前面パネルのメニュー 31「ANTENNAS」と、ディスプレイの「ANT 1 2」表示にあたります。

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    Applies immediately, and is read back from the radio when the port opens: this is the rig's own setting rather than a copy kept here.
settings-radio-one-profile-covers-both-there =
    1 つのプロファイルで両方を扱います。ここで選ぶものはありません。ポートを開くと無線機が自分の名前を名乗ります。

    This is the CAT half only: it drives the dial, the mode, PTT, the S-meter, the SWR and the transmit power over the rig's CAT USB port, and takes audio from its USB Audio port. The FDM-DUO's third USB port carries wideband I/Q, which this interface does not read — select the ELAD interface above for that.

    Note the baud rate: menu 70 "CAT BAUD" on the radio must match, and it ships at 38400.

    CW is keyed by the radio's own key or paddle. The FDM-DUO has no command that accepts text, so the CW panel cannot key it over CAT.
settings-radio-one-profile-covers-range-qrp =
    1 つのプロファイルでシリーズ全体を扱います。QRP Labs 独自のコマンドセットで、Kenwood TS-480 のサブセットに多くが追加されたものです。ここで選ぶものはありません。無線機がポートを開いたときに自分の名前とファームウェアのバージョンを名乗り、そのバージョンによって SWR 保護の読み取りを求めるかどうかが決まります。

    Not the Kenwood profile: on a QMX the PC command is the power METER rather than the power control, so a radio driven as a Kenwood would have its meter read written to as if it were a setting, and MD8 — a Kenwood mode — is SWR Tune here.

    The transmit power is set at the radio. There is no CAT command for it, so the Drive slider only reaches the level of the audio going into the sound card.

    The receive filter is the radio's too: it reports the width its mode implies (3.2 kHz in Digi, 300 Hz in CW) and offers nothing to change it with.
settings-radio-baud-rate-ignored-port-usb = ボーレートは無視されます。ポートは USB です
settings-radio-qmx-serves-own-virtual-com =
    QMX は独自の仮想 COM ポートを USB で提供するため、上で設定したレートは両端に影響しません。最大 3 つまで提供されます。無線機が何も応答しない場合、最も可能性が高い理由は、誤ったポートを選んでいることです。

    One thing does matter about the port: a carriage return sent to it switches the radio into terminal mode for the rest of the session. sdroxide never sends one — but a terminal program left open on the same port will, and CAT stops working the moment it does.
settings-radio-i-q-mode-switched-radio = I/Q モードは無線機でオンに切り替えます
settings-radio-radio-s-sound-card-carries =
    無線機のサウンドカードは、復調された音声か、ADC が見ている生の I/Q のどちらかを運びます。sdroxide は、ポートを開いたときに、上の音声フォーマットが求めるほうを設定します（無線機の Q9 コマンド、「IQ mode」メニュー項目）。

    The centre offset and sample rate above have been set to what a QMX needs: its receiver is a superhet with a 12 kHz I.F., so the synthesiser sits 12 kHz below the dial and everything on the card is 12 kHz above the middle of the span. The card runs at 48 kHz, which makes the panadapter 48 kHz wide.

    In CW the radio moves that I.F. by a further ~700 Hz, which one figure cannot follow — add it by hand if you run the panadapter with the radio in CW. In Digi, where a radio used as an I/Q front end normally sits, there is nothing to add.

    QRP Labs note that I/Q mode is not suitable for WSJT-X and other programs that expect demodulated audio; here the demodulation happens on this side, so it is.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ（5 W HF トランシーバー）
settings-radio-hobbypcb-s-rs-hfiq-there =
    HobbyPCB の RS-HFIQ です。ここで選ぶものはありません。このプロファイルが無線機のコマンドセットのすべてであり、ポートを開くとファームウェアのバージョンが記録されます。

    The control link carries two things and no more — where to put the oscillator, and whether to transmit. There is no mode, power, filter, squelch or meter command, because the radio has none of them. Everything else is sdroxide's: what comes off the sound card is complex baseband centred on the dial, so the mode, the filter and the modulation all happen on this side.

    The frequency command covers 3–30 MHz. Ask for anything outside that and the radio refuses it and stays where it is; the log says so.

    CW is keyed as audio through the transmit chain — this radio has no keyer that takes text. (It has an internal CW generator; sdroxide never uses it, on the firmware's own advice.)
settings-radio-sound-format-baud-ptt-are = 音声フォーマット、ボーレート、PTT は無線機のもので、設定ではありません
settings-radio-all-three-have-been-filled =
    3 つすべてが入力されており、どれも好みの問題ではありません。

    • Sound format is I/Q (stereo) — the card carries complex baseband and nothing else.
    • 57600 baud, 8N1 — the firmware's one rate, with no menu to change it. Any other is a silent link, so it is pinned when the port opens.
    • PTT over CAT — the *X command is the only transmit switch the interface has.

    The centre offset has been set to zero: the synthesiser runs at four times the dial into a quadrature detector, so the middle of the span is the dial. Set the sample rate above to whatever your sound card is actually running at; that is what makes the panadapter as wide as it is.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    DL2MAN/PE1NNZ のポケット QRP トランシーバーと、それが発展した公開の uSDX ファームウェアです。Kenwood TS-480 をエミュレートしますが、サブセットは少ないです。応答するのは、ダイヤル、モード、PTT、RIT/XIT のクリアだけです。S メーター、SWR、出力制御、VFO B、スプリット、CAT キーヤーはないため、CAT ではドライブのスライダーとメーターは何もしません。音声レベルが、唯一の送信制御です。

    CW is keyed as audio (MCW) through the sideband, or with the key or paddle at the radio; there is no command that takes text.
settings-radio-audio = 音声
settings-radio-tr-usdx-has-no-sound =
    (tr)uSDX には独自のサウンドカードがなく、音を聞く方法が 2 つあります。どちらが適切かは、無線機ではなくお使いのシャックの状況で決まります。

    "One cable" takes the audio inside the CAT serial link itself — the radio's own 8-bit stream, over the same USB cable as the control, with nothing else to plug in. The catch is the firmware: it cannot take a CAT command while its stream is running (a command sent into the stream does not pause it, it kills it), so this mode never polls. Tuning and mode changes are sent as you make them, each pausing the stream for a moment and starting it again, so expect a brief gap on each change; nothing is sent while you are simply listening. The radio's own dial and mode are not followed either — there is no read to follow them with — so drive it from here.

    "USB sound card" takes the audio from an external sound card wired to the radio's 3.5 mm speaker/mic jack, and behaves as any other CAT rig: the dial poll runs, so the radio's own knob and mode are followed. Pick the card under Radio audio below.

    Both modes open the port with DTR held high: the serial adapter's DTR is wired to the radio's reset, so opening the port reboots it and the first second or two is quiet while it comes up. DTR is never used to key.
settings-radio-radio-model = 無線機の機種
settings-radio-which-icom-two-things-ci =
    どの Icom か。CI-V が機種によって異なる 2 つの点のためです。

    The transceiver address below is filled in from it: every model ships with a different one, and a frame sent to the wrong address is simply ignored — a radio that answers nothing, with no error anywhere to say why.

    It also decides whether sdroxide can select DATA mode. On CI-V, USB and USB-DATA are the same mode byte and a separate command tells them apart; without it a digital-mode over goes out through the microphone input, with the rig's speech processing and SSB filter in the path.

    "Other" leaves the address to you and sends no DATA-mode command at all.
settings-radio-radio-id-hex = 無線機 ID（16 進）
settings-radio-show-radio-s-spectrum-scope = 無線機のスペクトラムスコープを表示
settings-radio-scope-span = スコープのスパン
settings-radio-how-wide-sweep-radio-keeps = スコープを掃引する幅です。無線機は、自身の画面で最後に選ばれたスパン（多くは数 kHz）を保持します。ここでスパンを設定すると、スコープは中心モードになり、ダイヤルに追従します。これは無線機自身の表示も変えます。「無線機の設定どおり」にすると、そのままになります。
settings-radio-press-apply-reconnect-switch-without = 再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-radio-audio-sound-card = 無線機の音声（サウンドカード）
settings-radio-devices = デバイス
settings-radio-discover = 検出
settings-radio-no-devices-press-discover = デバイスがありません。検出を押してください
settings-radio-manual-ip = 手動の IP
settings-radio-optional-e-g-192-168 = 任意。例：192.168.1.50
settings-radio-sample-rate = サンプルレート
settings-radio-receiver-ddc = 受信機（DDC）
settings-radio-protocol-2-board-carries-several = Protocol 2 のボードは、1 つの接続上に独立して同調できる複数の受信機（DDC）を持ちます。この無線機を DDC1 で動かし、別の無線機を同じアドレスで DDC2 に載せることができます。送信機は DDC1 の無線機に属します。Protocol 1 のボードには DDC1 しかありません。
settings-radio-lna-gain = LNA ゲイン
settings-radio-front-end-gain-hermes-lite = Hermes-Lite 2 のフロントエンドのゲインです。即座に有効になり（再接続は不要）、無線機が開始するレベルとして記憶されます。高すぎると ADC がクリップし、バンド全体が歪んで見えます。低すぎると受信機が聞こえなくなります。
settings-radio-overload-protection = 過負荷保護
settings-radio-back-lna-gain-off-itself =
    ボードが ADC のオーバーフローを報告している間、LNA ゲインを自動的に下げ、オーバーフローが止まったら戻します。

    A Hermes-Lite 2 samples the whole of 0-38 MHz onto one 12-bit converter with no mixer and no preselector in front of it, so a broadcast station a band away can drive it into overflow while the band you are looking at shows nothing wrong at all — the noise floor climbs, everything intermodulates and the decoders stop. The board knows, and this is what acts on it.

    Off by default: it moves a control you set. Nothing happens while transmitting.
settings-radio-wind-gain-back-when-adc = ADC がオーバーフローしたときにゲインを戻す
settings-radio-applies-apply-reconnect-main-window = 適用 / 再接続で適用されます。メインウィンドウのゲインのレールは、ボードが実際に動かしているゲインを表示するため、ループに追従して、その動きを見られます。上のスライダーは無線機が開始するレベルで、ループはそれを書き換えません。
settings-radio-how-far-gain-moves-each = 1 回ごとにゲインが動く量（dB）です。1 は、ボード自身のゲインレジスターのステップ幅です。
settings-radio-attack-decay = {"  "}アタック / ディケイ
settings-radio-how-often-gain-may-come =
    コンバーターがオーバーフローしている間、ゲインを下げられる頻度と、止まった後に上げ戻せる頻度です。

    The two are deliberately a hundred times apart. Retreat immediately: every millisecond of overflow is a receiver full of intermodulation. Return slowly: whatever caused it — a neighbour keying, a broadcaster coming up at dusk — has usually not gone away, and a loop that recovered as fast as it retreated would spend the evening oscillating across the threshold. 100 ms and 10 s per decibel is what PowerSDR's Auto S-Att and N1GP's HermesIntf have both used.
settings-radio-range = {"  "}範囲
settings-radio-lowest-highest-gain-loop-may = ループが使ってよい最低と最高のゲイン（dB）です。上限は、ループが受信機の感度を勝手に決めないようにするためのもので、下限は、ある点より下のオーバーロードは他人の問題であり、答えは 20 dB 追加ではなくフィルターである、とあなたが決める位置です。
settings-radio-filter-board = フィルターボード
settings-radio-accessory-board-hermes-lite-2 = Hermes-Lite 2 の J16 ヘッダーにあるアクセサリーボード（または他の openHPSDR ボードの、オープンコレクター出力。Protocol 2 を含みます）。「N2ADR」は、バンドごとに 1 つのリレーを選びます。「Alex / Hermes バンドコード」は、バンド番号を出力 1〜4 に出します。ANAN の Alex ボード、Zeus SDR、HiQSDR、Quisk が期待するものです。「カスタム」を選ぶと、下に表が開き、受信時と送信時に、バンドごとの 7 つの出力を自分で指定できます。アンテナスイッチ、アンプのバンドデコーダー、その他どちらのプリセットにも合わないものに使えます。どちらのプリセットも、出発点として表に流し込めます。実際に何かが取り付けられていない限り「なし」のままにしてください。それらの 7 ピンは汎用のオープンコレクター出力で、運用者はアンプの PTT、アンテナリレー、トランスバーターの切り替えにも配線します。バンドのデータで駆動すると、接続されているものが動いてしまいます。適用 / 再接続で有効になります。
settings-radio-io-board-rx-input = IO ボードの RX 入力
settings-radio-where-n2adr-hl2ioboard-takes-receive = N2ADR HL2IOBoard が受信信号を取る場所です。ボード自体は自動的に見つかり、設定は不要です。この設定は、ボード自身の SMA ジャックを配線した運用者だけのためのものです。それ以外は「無線機自身の入力」のままにしてください。何も接続されていない IO ボードの J9 を選ぶと、受信機が聞こえなくなります。適用 / 再接続で有効になります。どちらかの J9 を選ぶと、入力は ANT コントロールにも表示され、バンドごとに記憶されます。
settings-radio-power-amplifier = パワーアンプ
settings-radio-use-hermes-lite-2-s = Hermes-Lite 2 の内蔵 PA を使う
settings-radio-default-what-you-want-unless = 既定でオンで、ボードの低出力 RF1 出力から外部アンプを駆動する場合を除き、こちらが望ましい設定です。オフでも無線機はキーされます。T/R リレーが切り替わり、アクセサリーボードも従いますが、アンテナのジャックは電力を出さず、リレーは意図的に受信に保たれます。Hermes-Lite 以外のボードでは無視されます。
settings-radio-mirror-board-s-spectrum-about = ボードのスペクトラムを、受信だけでなく送信時にも同調周波数を軸に反転します。既定でオンです。Hermes-Lite 2 にはこれが必要です。信号がダイヤルの反対側に現れ、何もデコードされない場合にのみオフにしてください。手がかりは、ウォーターフォールに説得力のある線が多数あるのに、SSB が反対側の側波帯に入り、FT8 がまったくデコードされないことです。
settings-radio-frequency-correction = 周波数補正
settings-radio-crystal-tcxo-error-ppm-applied = 水晶 / TCXO の誤差（ppm）。受信と送信に適用されます。即座に適用されます。
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = 送信バッファー
settings-radio-how-far-ahead-real-time = sdroxide がそれに合わせて速度を落とす前に、送信音声をネットワーク経由でボードへリアルタイムよりどれだけ先に送るかです。ボード自体はそのようなバッファーを持たず、これは sdroxide 自身の余裕を広げるだけです。直接の有線接続には適した低い既定値では、ジッターに対する余裕が足りず、送信音声や PTT が途切れる WiFi やVPN 接続では、値を上げてください。値を上げると送信の遅延が増えます。適用で有効になり、ボードへ再接続します。
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = 適応プリディストーション：実際に放射された内容のサンプルから送信機を直線化し、同じ出力で相互変調を 20 dB 強く減らします。受信機がフィードバック経路になります。ボードは送信中も受信を続けるため、方向性結合器と減衰器で、アンプの出力のサンプルを、送信時に T/R スイッチが取り去らない入力に入れる必要があります。Hermes-Lite 2 では、IO ボードの PureSignal ジャックを使い、上の受信入力をそれに合わせます。ない場合は、ANAN の RX ポート、または交信の間だけ結合器を入れる外部リレーを使います。読み取られるのはこの受信機です。2 つ目の ADC を持つボードは、ファームウェアがどう経路を組んでいても、RX2 をフィードバックのタップに使うことはできません。何も結合されていない場合、ループはロックせず、送信機はまったく元のままになります。適用 / 再接続で適用され、送信機を持つ無線機（DDC1）でのみ有効です。
settings-radio-correct-transmitter-receiver-s-own = 送信機を、受信機自身のフィードバックから補正する
settings-radio-how-finely-amplifier-s-curve = アンプの曲線をどれだけ細かくモデル化するかです。ステップが多いほど鋭い屈曲に従いますが、埋めるのに時間がかかります。32 から始めるのが妥当です。
settings-radio-how-fast-correction-follows-what = 補正がカプラーの報告にどれだけ速く追従するかです。遅くするのが適切です。これは、変化しないアンプの曲線を、ノイズの多いフィードバック経路から平均するものだからです。
settings-radio-hold = 保持
settings-radio-stop-adapting-keep-correction-where = 適応を止め、補正をそのままの位置に保ちます。測定用や、学習した内容に満足している運用者用です。
settings-radio-manual-ip-overrides-discovery-press = 手動の IP は検出より優先されます。再起動せずに切り替えるには「適用 / 再接続」を押してください。
settings-radio-open-collector-outputs-band = バンド別のオープンコレクター出力
settings-radio-one-control-word-per-band = ハードウェアのドキュメントどおり、バンドごとに 1 つの制御ワードです。ビット 0 は出力 1、ビット 6 は出力 7 です。RX はそのバンドで受信中にアサートされ、TX は送信機がキーされている間にアサートされます。フィルターには同じ値を、切り替えの片側だけに属するもの（アンプのキーライン、受信プリアンプのバイパス）には異なる値を与えてください。00 のままのバンドは何もアサートしません。適用 / 再接続で有効になります。
settings-radio-alex-band-code = ALEX バンドコード
settings-radio-replace-table-what-preset-would = 表を、このプリセットが各バンドで送るものに置き換え、編集の出発点にします。設定の他の部分は変わりません。
settings-radio-clear = クリア
settings-radio-every-output-off-every-band = すべてのバンドのすべての出力がオフになります。「なし」と同じです。
settings-radio-band = バンド
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = 出力中
settings-radio-other = その他
settings-radio-everywhere-outside-amateur-bands-short = アマチュアバンドの外すべて。短波の受信や、トランスバーターのダイヤルがどのバンドにも属さない場合を含みます。
settings-radio-dongle = ドングル
settings-radio-rescan = 再スキャン
settings-radio-re-list-usb-bus-no = USB バスを再度一覧表示します。デバイスは開かれないため、受信中に押しても安全です。
settings-radio-no-dongles-press-rescan = ドングルがありません。再スキャンを押してください
settings-radio-first-one-found = — 最初に見つかったもの —
settings-radio-rtl2832u-s-resampler-reaches-225 = RTL2832U のリサンプラーは 225–300 kHz と 900 kHz–3.2 MHz に届き、その間はありません。適用で有効になります。
settings-radio-often-drops-samples = {"  "}（サンプルが落ちることが多い）
settings-radio-agc = AGC
settings-radio-manual-setting-measurement-weak-signal = 手動は、測定や微弱信号のデジタルモード用の設定です。チューナーと復調器は、それぞれ独立した自動ループを持ちます。
settings-radio-tuner-gain = チューナーのゲイン
settings-radio-applies-immediately-no-reconnect-tuner = 即座に適用されます。再接続は不要です。チューナーには 29 段階の離散的なステップがあるため、値は生成できる最も近い段階に合わされます。チューナー AGC の動作中は無視されます。
settings-radio-crystal-error-parts-per-million = 水晶の誤差（百万分率）。RUST_LOG=sdroxide_rtlsdr=debug で実行すると、約 20 秒後にログが測定されたクロック誤差を出力します。これが入力すべき数値です。即座に適用されます。
settings-radio-hf-reception = HF 受信
settings-radio-tuner-itself-starts-24-mhz =
    チューナー自体は 24 MHz から始まります。RTL-SDR Blog V4 は、それより下をハードウェアでアップコンバートします。ほかのドングルは、V3 の HF ポートを通して ADC を直接サンプリングすることでしか HF に届きません。自動では、24 MHz より下のすべてを ADC に、それより上のすべてをチューナーに振り分けます。

    Direct sampling covers every HF band, 17 m and 15 m included — they arrive in the ADC's second Nyquist zone, the right way up. Nothing filters the ADC's input, though, so whatever is at 28.8 MHz minus the dial comes with them: 10.7 MHz under 17 m, 7.726 MHz under 15 m.

    Switching modes briefly interrupts the stream.
settings-radio-removes-dongle-s-own-dc =
    ドングル自身の DC スパイクを、スパンの中央から取り除きます。そして、その周りに反射される、すべての信号のイメージも取り除きます。サンプル自身の不均衡を測定して行うため、校正は不要で、即座に適用されます。チューナーにはオフセット同調のモードがないため、中央をクリアする唯一の方法です。

    An AM carrier tuned dead on the dial is at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-remove-centre-spike-mirror-image = 中央のスパイクとイメージを除去
settings-radio-bias-tee = バイアス T
settings-radio-feed-4-5-v-dc = 同軸ケーブルに約 4.5 V の DC を供給
settings-radio-bias-tee-never-connect-transceiver = バイアス T はオンです。これが有効な間は、トランシーバー、接地されたアンテナ、他から電源を得るプリアンプを接続しないでください。DC は給電線をそのまま流れます。
settings-radio-receive-only-dongle-sample-rate = 受信のみ。ドングルとサンプルレートは適用で有効になります。それ以外は変更と同時に適用されます。
settings-radio-server-address = サーバーのアドレス
settings-radio-where-rtl-tcp-listening-address =
    rtl_tcp が待ち受けているアドレスです。アドレス、またはアドレスとポートです。ポートの既定値は 1234 で、これは rtl_tcp 自身の既定値です。

    On the far end, start it as `rtl_tcp -a 0.0.0.0` — bound to 127.0.0.1, which is what it does with no -a, it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g = ホストまたはホスト:ポート。例：raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    サーバーに要求するものです。ローカルのドングルと同じリサンプラーの制限が適用されます。遠端でも同じシリコンだからです。

    The figure beside each rate is what it costs on the link: the samples are sent uncompressed, and a rate the network cannot carry makes rtl_tcp drop the connection rather than degrade. Takes effect on Apply.
settings-radio-wired-link = {"  "}（有線リンク）
settings-radio-runs-server-s-dongle-manual = サーバーのドングルで動作します。手動は、測定や微弱信号のデジタルモード用の設定です。
settings-radio-applies-immediately-no-reconnect-sent = 即座に適用されます。再接続は不要です。値は 0.1 dB 単位で送られ、サーバーがチューナーの持つ段階に合わせます。プロトコルには応答がないため、落ち着いた値は読み戻せず、このスライダーは要求した値を表示し続けます。チューナー AGC の動作中は無視されます。
settings-radio-crystal-error-server-s-dongle =
    サーバーの* ドングルの水晶の誤差（百万分率）です。その機器の性質なので、このマシンのドングルではなく、ここで設定します。即座に適用されます。

    The measured clock error the USB interface prints is not available here: over a network what that measurement sees is the buffering, not the crystal, and it is wrong by thousands of ppm. Calibrate the dongle on USB once and carry the number over, or tune a broadcast station of known frequency and adjust until it sits on the dial.
settings-radio-tuner-starts-24-mhz-below =
    チューナーは 24 MHz から始まり、それより下では遠端に助けが必要です。Blog V4 は自分でアップコンバートするため、自動ではそのまま放置します。プロトコルはチューナーのチップしか報告しないため、それができる唯一のことです。また、ワイヤー上では V4 は素の R828D と区別がつきません。

    On a V3 or any other dongle, Automatic switches the server to direct sampling below the tuner's own 24 MHz floor — which covers every HF band, 17 m and 15 m arriving in the ADC's second Nyquist zone with 28.8 MHz minus the dial folded on top. Choose Direct sampling explicitly for a plain R828D that hears nothing on HF. Switching briefly interrupts the stream.
settings-radio-removes-dongle-s-dc-spike =
    ここで、ドングルの DC スパイクとイメージを、届いたサンプルから取り除きます。それらはハードウェアによる副産物なので、ほかのものと一緒にネットワークを通ります。即座に適用されます。

    An AM carrier tuned dead on the dial sits at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-powers-preamp-far-end-s = 遠端のドングルからプリアンプに電源を供給します。古いサーバーはこのコマンドを実装しておらず、黙って無視します。プロトコルには「いいえ」を伝える方法がないためです。そのため、バイアス T がオンにならなくても、必ずしもこちら側の問題ではありません。
settings-radio-feed-4-5-v-dc-2 = リモートの同軸ケーブルに約 4.5 V の DC を供給
settings-radio-bias-tee-hardware-somewhere-else = バイアス T はオンで、その機器は別の場所にあります。その給電線の反対側にあるもの（トランシーバー、接地されたアンテナ、電源が入っているプリアンプ）は、確認のためにあなたの目の前にはありません。
settings-radio-sdrplay-server-rsp-tcp = SDRplay サーバー（rsp_tcp）
settings-radio-sdrplay-server-greets-exactly-like = SDRplay サーバーは、ドングルとまったく同じ挨拶をします。そのため、検出されたときではなく、常にこれらを表示します。通常の rtl_tcp サーバーに対しては、単に無視されます。プロトコルには応答がなく、知らないコマンドは捨てられるからです。サーバーが自分を名乗ると、デバイスのタブはそれを rsp_tcp と表示します。
settings-radio-input = 入力 A
settings-radio-input-b = 入力 B
settings-radio-hi-z = ハイ Z
settings-radio-lna-state = LNA の状態
settings-radio-step-index-not-db-figure = dB の値ではなく、段階の番号です。1 段階あたりの値は、RSP の機種とバンドによって異なります。0 が最大のゲインです。
settings-radio-if-gain-reduction = IF のゲイン減衰
settings-radio-reduction-so-more-less-signal = 減衰量なので、大きいほど信号は小さくなります。RSP の AGC がオフのときにのみ有効です。
settings-radio-enable = 有効
settings-radio-level-rsp-s-agc-aims = RSP の AGC が維持しようとするレベルです。
settings-radio-notches = ノッチ
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = 基準出力
settings-radio-t-24-mhz-clock-out = 24 MHz クロック出力
settings-radio-rsp2-rspduo-only = RSP2 と RSPduo のみ。
settings-radio-receive-only-address-sample-rate = 受信のみ。アドレスとサンプルレートは適用で有効になります。それ以外は変更と同時に適用されます。接続が切れた場合は自動で再試行されるため、再起動されたサーバーは、ここで何も触らなくても復帰します。
settings-radio-where-spyserver-listening-address-address =
    spyserver が待ち受けているアドレスです。アドレス、またはアドレスとポートです。ポートの既定値は 5555 で、これは spyserver 自身の既定値です。

    On the far end, check that its config file binds an address other machines can reach — bound to 127.0.0.1 it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-2 = ホストまたはホスト:ポート。例：raspberrypi.local:5555
settings-radio-i-q-bandwidth = I/Q 帯域幅
settings-radio-how-wide-slice-band-arrives =
    バンドのどれだけの幅が I/Q として届くかです。そのため、このリンクをどれだけ使うかも決まります。これは *復調された* 窓で、ダイヤルに追従します。それより広いものはすべて、パナダプターの上のストリップにあるサーバーの FFT です。

    Automatic aims at about 96 kHz, which carries every mode here including wide FM and still fits down a cellular uplink. Takes effect on Apply.
settings-radio-which-stage-server-s-own =
    サーバー自身のレートの段階のうち、どれを要求するかです。受信機ごとに段階が異なります。最大レートを段階ごとに半分にしたものだからです。そのため、ヘルツの値ではなく段階として保存されます。同じ設定は、別のサーバーに向けても意味を持ちます。

    Automatic aims at about 1 Msps. Press Test connection to see what the stages come to on this server. Takes effect on Apply.
settings-radio-automatic = 自動
settings-radio-sample-format = サンプルの形式
settings-radio-bits-per-component-wire-so =
    回線上の各成分のビット数で、リンクにかかるコストもこれで決まります。16 ビットは 8 ビットの 2 倍で、32 ビット浮動小数点はその 4 倍ですが、受信機の ADC が持っていた情報以上のものは増えません。

    8-bit is what makes a remote receiver work over a domestic uplink and is right for almost everything. A server may override this — some are configured to insist on one format — and says so in the log when it does. Takes effect on Apply.
settings-radio-gain = ゲイン
settings-radio-server-s-gain-stage-index =
    サーバーのゲインの段階です。デシベルの値ではなく、インデックスです。各インデックスが何を意味するかは、遠端の受信機に属し、バンドによって変わります。プロトコルはそれを示さないため、ここでは、作り話の数値を入れずに dB に変換することはできません。

    The real range is the server's and is only known once connected; an index past it is clamped. Applies immediately — no reconnect.
settings-radio-index = インデックス{" "}
settings-radio-digital-gain = デジタルゲイン
settings-radio-how-far-server-scales-samples =
    量子化する前に、サーバーがサンプルをどれだけ拡大してワイヤーに送るかです。自動では、実際に届いているものを監視し、ピークをフルスケールの半分に保ちます。これは、空いているバンドと混雑したバンドの両方で正しくなる唯一の方法です。両者は 30 dB 離れており、固定の値では両方に対応できません。

    It matters only at 8 bits, where a signal far below full scale loses its lower bits to the quantiser and one above it is clipped flat. Wider formats have room to spare and get the figure every other client computes, from the gain index and the decimation stage. Applies immediately.
settings-radio-full-band-strip = 全帯域のストリップ
settings-radio-server-s-own-fft-whole =
    バンド全体についてのサーバー自身の FFT で、パナダプターの上のストリップに描かれます。このインターフェースでは、これが唯一のバンド表示です。パナダプター自体は、受信中の I/Q と同じ幅しかないためです。

    Switching it off leaves a receiver with no way to see anything it is not already tuned to. Applies immediately, with a short break in the audio while the server changes what it is sending.
settings-radio-ask-server-low-rate-fft =
    I/Q と一緒に、バンド全体の低レートな FFT もサーバーに要求し、パナダプターの上のストリップに描きます。

    It costs almost nothing — a couple of kilobytes a frame, a dozen or so times a second — and it shows the whole receiver rather than the slice being demodulated. Worth switching off only on a link where every byte counts. Applies immediately, with a short break in the audio.
settings-radio-show = 表示
settings-radio-whole-band = 全バンド
settings-radio-how-much-receiver-strip-covers = ストリップが受信機のどれだけをカバーするかです。全バンドは最も広い表示です。これを狭めると、同じ数のビンがより狭いスペクトラムに配置されるため、短い区間をより細かく見られます。
settings-radio-strip-db-window = ストリップの dB 窓
settings-radio-server-quantises-fft-into-window =
    サーバーは、送信前に FFT をこの窓に量子化します。ビンあたり 1 バイトです。そのため、これらはストリップの描き方だけでなく、どれだけ細かく測定されるかも決めます。ストリップが *表示される* 下限と上限は、エンジン自身の自動レベル調整で設定されるもので、これとは別のものです。

    The default 150 dB is the whole protocol range and needs no attention. Narrowing it around the noise floor buys resolution on a receiver whose signals all sit in a small part of the scale. Applies immediately.
settings-radio-top = 上{" "}
settings-radio-range-2 = 範囲{" "}
settings-radio-i-q-correction = I/Q 補正
settings-radio-remove-dc-spike-mirror-image = DC スパイクとイメージを、こちら側の DSP で除去します。遠端の受信機がそれを必要とするかは、その機種によります。Airspy HF+ は不要で、RTL-SDR は必要です。プロトコルは、どの機種と通信しているかを示しません。即座に適用されます。
settings-radio-enabled = 有効
settings-radio-test-connection = 接続テスト
settings-radio-connect-read-what-server-says = 接続して、サーバーが受信機について言うことを読み取り、再び切断します。ストリームは開始しないため、他の人が使っているサーバーに対して押しても安全です。
settings-radio-receive-only-panadapter-narrow-i = 受信のみ。パナダプターは狭い I/Q の窓で、ダイヤルに追従します。バンド表示は、その上のストリップにあるサーバーの FFT です。再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-receive-only-wideband-i-q = 受信のみ。広帯域の I/Q で、どのローカルの SDR とも同じです。再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-receiver-address = 受信機のアドレス
settings-radio-where-receiver-serves-web-page =
    受信機がウェブページを提供している場所です。アドレス、またはアドレスとポートです。ポートの既定値は 8073 で、これは KiwiSDR 自身の既定値です。

    Note that a receiver reached through the project's proxy (something.proxy.kiwisdr.com — nearly half the public ones) answers on port 80 instead, so give it explicitly. Picking a receiver from "Public SDRs" fills this in correctly.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-3 = ホストまたはホスト:ポート。例：kiwi.example.org:8073
settings-radio-password = パスワード
settings-radio-receiver-s-user-password-where =
    受信機の *ユーザー* パスワードです。運用者が設定している場合のみ。ほとんどすべての公開受信機では空欄です。管理者パスワードではありません。

    Takes effect on Apply.
settings-radio-announce = 名乗る名前
settings-radio-name-connection-shows-up-under =
    この接続が表示される名前です。受信機の所有者と、それを聴いているすべての人が見られます。

    Left blank it is your station callsign, which is the network's own convention — some operators do block clients that will not identify. Takes effect on Apply.
settings-radio-blank-station-callsign = 空欄 = ステーションのコールサイン
settings-radio-band-view = バンド表示
settings-radio-ask-receiver-own-waterfall-well =
    I/Q と一緒に、受信機自身のウォーターフォールも要求し、パナダプターの上のストリップに描きます。

    Worth having: the I/Q is only about 12 kHz wide, so without this there is nothing to tune *by*. It is a second connection carrying roughly 20 kB/s at full speed, against the I/Q's 44 kB/s.

    Takes effect on Apply.
settings-radio-show-receiver-s-0-30 = 受信機の 0〜30 MHz のウォーターフォールを表示
settings-radio-band-view-speed = バンド表示の速度
settings-radio-how-often-receiver-sends-waterfall =
    受信機がウォーターフォールの行を送る頻度です。1（最も遅い）から 4 までです。ここで、動作中のリンクのコストを変える唯一の設定です。従量課金の接続では下げてください。

    Applies immediately.
settings-radio-band-view-span = バンド表示のスパン
settings-radio-how-much-band-receiver-s =
    受信機のウォーターフォールがカバーするバンドの量です。スライダーの左端は全体の 0〜30 MHz で、右へ 1 段階ごとに半分になります。

    The waterfall is always 1024 bins wide however wide the window is, so this is really a resolution control: the whole band is 29 kHz to a bin, which is a band *map* rather than a picture of anything, while a few hundred kilohertz shows individual stations in a broadcast band. Zoomed in, the window follows your dial.

    The whole band is the default and is what makes the strip a thing to tune by; the panadapter below it is the ~12 kHz of I/Q. Applies immediately.
settings-radio-receiver-agc = 受信機の AGC
settings-radio-receiver-s-agc-which-far =
    受信機の* AGC です。リンクの向こう側にあり、I/Q より前にあるため、sdroxide が何をするより先に作用します。

    On by default, unlike every local SDR here, because the measured alternative was worse: on a live receiver the manual gain was not monotonic across its range and its top end clipped the I/Q at full scale, while the AGC held about -24 dBFS with 7 dB of headroom.

    It does mean the sample amplitude is not a signal level, which is why the S-meter is read from the receiver's own figure instead. Applies immediately.
settings-radio-let-receiver-ride-own-gain = 受信機に自身のゲインで動かせる
settings-radio-manual-gain = 手動ゲイン
settings-radio-fixed-gain-when-receiver-s = 受信機の AGC がオフのときの固定ゲインです。独自の 0〜90 の目盛りです。何らかの値のデシベルではありません。プロトコルは manGain と呼ぶだけで、それ以上は何も言いません。また単調でないことが測定されているため、校正ではなく、良い位置を見つけるためのつまみとして扱ってください。即座に適用されます。
settings-radio-read-receiver-s-status-page = 受信機のステータスページを読み取ります。何であるか、何をカバーするか、空きチャンネル数、ブラウザ以外のアプリからの接続を運用者が許可しているかを確認します。そのチャンネルは使いません。
settings-radio-host-port-e-g-127 = ホスト:ポート。例：127.0.0.1:50001
settings-radio-iq-sample-rate = IQ サンプルレート
settings-radio-receiver = 受信機
settings-radio-rig-two-receivers-sunsdr2dx-can = 2 つの受信機を持つリグ（SunSDR2DX）は、1 つの接続から 2 つの無線機タブを提供できます。この無線機を RX1 で、別の無線機を RX2 で動かしてください。送信機は RX1 の無線機に属します。
settings-radio-stream-delay = ストリームの遅延
settings-radio-how-long-rig-s-iq = sdroxide が新しい中心へ移動させた後、リグの IQ が届くまでの時間です。これはネットワークではなく、リグ自身の DSP パイプラインによるものです。このマシン上のサーバーにも同様にあります。パナダプターを最大に縮小した状態でドラッグしているとき、この量を誤ると、最新のウォーターフォールの行が、ドラッグの速度と誤差の積だけ履歴から横にずれます。高すぎても低すぎても、ずれる量は同じで、方向が反対になります。`cargo run --release -p sdroxide-tci --example retune_latency` で測定してください。既定値は、ExpertSDR3 で 192 kHz の SunSDR2DX のものです。
settings-radio-wideband-iq-receive-audio-transmit = 広帯域 IQ の受信、音声の送信。再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-radio-address = 無線機のアドレス
settings-radio-address-shown-radio-under-set = 無線機の SET > Network に表示されるアドレスです。そこで Network Control をオンにしておく必要があり、無線機にネットワークのユーザー名とパスワードを設定しておく必要があります。
settings-radio-host-ip-e-g-192 = ホストまたは IP。例：192.168.1.50
settings-radio-control-port = 制御ポート
settings-radio-t-50001-unless-has-been-changed = 無線機で変更されていない限り 50001 です。
settings-radio-network-user = ネットワークのユーザー
settings-radio-stored-clear-radio-json-protocol = radio.json に平文で保存されます。プロトコルは回線上で可逆的に難読化しているだけなので、ここで秘密になるわけではありません。
settings-radio-receive = 受信元
settings-radio-audio-sample-rate = 音声サンプルレート
settings-radio-t-12-khz-if-needs-48000 = 12 kHz の IF には 48000 Hz のストリームが必要です。このレートでは、無線機の復調された音声が使われます。
settings-radio-if-spectrum = IF スペクトラム
settings-radio-which-way-round-radio-s = 無線機の 12 kHz IF が、どちら向きに動くかです。「自動」は、そのモデルが知られている動作を使います。IC-7760 では反転し、他のすべての Icom では通常です。SSB が誤った側波帯で出る場合は、手動で設定してください。手がかりは、バンドが LSB で動いているのに USB を選ぶ必要があり、しかも無線機自身のモード表示は sdroxide とずっと一致している場合です。
settings-radio-normal = 通常
settings-radio-mirrored = 反転
settings-radio-displayed-bandwidth = 表示される帯域幅
settings-radio-width-audio-band-panadapter-cat = CAT リグの場合と同様に、音声帯域のパナダプターの幅です。デジタルモードと、無線機のスコープがオフのときに使われます。それ以外の場合は、パナダプターがスコープであり、スコープのスパンがその幅を設定します。
settings-radio-how-cw-panel-s-keyer-2 = CW パネルのキーヤーがどう送信するかです。「無線機のキーヤー」は、無線機を CW にして、送るテキストを無線機自身のキーヤー（CI-V 0x17）に渡します。「サウンドカード」は、キーされたサイドトーンを代わりに LAN 経由で音声（MCW）として送ります。周波数は ダイヤル + ピッチ です。CW の無線機はそのモジュレーター入力を無視するため、CW を選んでも無線機は通常の USB のままになります。デジタルモードと同じモードです。
settings-radio-how-much-audio-radio-holds = 無線機が変調する前に保持する音声の量です。値を大きくすると、より悪いネットワークにも耐えられますが、送信の遅延が増えます。
settings-radio-streams-radio-s-own-475 = 無線機自身の 475 ビンのスイープを送信します。AF では、これがパナダプターです。無線機が送る音声は、そのフィルターを通過したものであり、バンドの画像ではありません。12 kHz IF では、上にある全帯域のウォーターフォールです。どちらの場合も、それは sdroxide の DSP ではなく無線機の画像です。それを計算する I/Q はありません。
settings-radio-how-wide-sweep-only-wide = どれだけ広くスイープするかです。これは Icom が持つ唯一の広い表示で、無線機は自身の画面で最後に選ばれたスパンを保持します。多くは数 kHz で、そのためストリップは、その下のパナダプターより広く表示されないことがあります。ここでスパンを設定すると、スコープは中心モードになり、ダイヤルに追従します。無線機自身の表示も変わります。「無線機の設定どおり」にすると、そのままになります。
settings-radio-switch-modulation-input-lan = 変調入力を LAN に切り替え
settings-radio-transmit-audio-only-heard-when = 送信音声は、無線機の MOD 入力が LAN に設定されている場合にのみ聞こえます。sdroxide は、メニュー番号を把握している機種では、これを書き込めます。それ以外の機種では、その旨を伝え、メニューには触れません。これは貸し出しです。そこに無線機が持っていたものは、最初に読み取られ、セッションが終わると戻されます。そのため、後で単独で使われるリグは、自分のマイクの音を聞けます。
settings-radio-copy-diagnostic-report = 診断レポートをコピー
settings-radio-control-audio-radio-s-own = 制御、音声、無線機自身のスコープを、1 つのネットワーク接続で行います。シリアルケーブルも、サウンドカードも不要です。再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-testing = テスト中…
settings-radio-was-only-check-press-apply = これは確認のみでした。この無線機の使用を開始するには、下の「適用 / 再接続」を押してください。
settings-radio-radios = 無線機
settings-radio-asks-network-iio-devices-also = ネットワーク上の IIO デバイスを問い合わせ、192.168.2.1 にも直接試みます。USB ケーブルの先の Pluto は、アドレスは有効でも、マルチキャストでは到達できないことがよくあるためです。
settings-radio-no-radios-press-discover = 無線機がありません。検出を押してください
settings-radio-address = アドレス
settings-radio-overrides-selection-above-usb-cable = 上の選択を上書きします。USB ケーブルは Pluto をシリアルポートではなくネットワークアダプターとして見せるため、無線機が机の上にあっても、これは IP アドレスです。
settings-radio-pluto-revision-c-pluto-unlocked = 2R2T にロックを解除された Pluto+ や改訂 C の Pluto は、1 台から 2 つの無線機タブを提供できます。この無線機を RX1 で、別の無線機を RX2 で、それぞれ独自のアンテナで動かします。2 つのチェーンは 1 つの発振器を共有するため、どちらの無線機を再同調しても両方が動きます。RX2 が提供するのは、同じスペクトラムの 2 つ目のアンテナ（ダイバーシティ）であり、2 つ目のバンドではありません。送信機は RX1 の無線機に属します。標準の 1R1T の Pluto は、接続時に RX2 を拒否します。
settings-radio-width-spectrum-sdroxide-receives-ad9361 = sdroxide が受信するスペクトラムの幅です。AD9361 は 61.44 Msps に達しますが、USB ネットワークリンクはそこまで届きません。このリストはそれに合わせて調整されています。最も低いレートには、AD9361 にフィルター構成を読み込む必要がありますが、sdroxide はそれを行いません。標準の Pluto は、代わりにそれらを約 2.084 Msps で動かし、接続時にその旨を伝えます。適用で有効になります。
settings-radio-more-than-usb-2-will = {"  "}（USB 2 では運べない量）
settings-radio-stock-pluto-runs-2-084 = {"  "}（標準の Pluto は 2.084 で動作）
settings-radio-analog-filter = アナログフィルター
settings-radio-ad9361-s-baseband-filter-leave = AD9361 のベースバンドフィルターです。自動の場合は 0 のままにしてください。自動では、サンプルレートの 9/10 まで開きます。これは意図的に広くしています。受信機は、DC スパイクから信号を遠ざけるために、発振器をスパンの 4 分の 1 だけダイヤルからずらして置くため、狭いフィルターでは、まさにそこへ移動させた部分が切れてしまうからです。適用で有効になります。
settings-radio-ad9361-has-four-modes-not = AD9361 はオン / オフのスイッチではなく、4 つのモードを持ちます。遅いアタックは SSB と CW に、速いアタックは断続的な信号に向きます。手動は、測定や微弱信号のデジタルモード用の設定です。即座に適用されます。
settings-radio-rx-gain = RX ゲイン
settings-radio-applies-immediately-no-reconnect-ignored = 即座に適用されます。再接続は不要です。AGC が手動に設定されていない限り無視されます。これは sdroxide の動作ではなく、AD9361 自身の動作です。
settings-radio-tx-gain = TX ゲイン
settings-radio-negative-because-ad9361-expresses-transmit = AD9361 は送信レベルを減衰量で表すため、負の値になります。0 dB が最大出力です。即座に適用されます。接続時にこの値を適用する前に、送信機は最も小さい値に設定されるため、前のプログラムが残したものが有効になることはありません。
settings-radio-reference-error-parts-per-million = 基準の誤差（百万分率）です。RUST_LOG=sdroxide_pluto=debug で実行すると、約 20 秒後にログが測定されたクロック誤差を出力します。これが入力すべき数値です。sdroxide が適用し、無線機自身の永続的なトリムには書き込みません。即座に適用されます。
settings-radio-full-duplex = フル デュプレックス
settings-radio-keep-receiving-while-transmitting = 送信中も受信を続ける
settings-radio-leave-off-pluto-reached-over =
    USB ケーブル経由で到達する Pluto では、これをオフのままにしてください。そのリンクは、両方向に同時に毎秒 1 メガサンプルを運べないためです。送信バッファーが枯渇すると、送信は途切れて出ます。そのため、既定では送信中は受信が止まります。

    Turn it on for a board on real Ethernet (a LibreSDR, a Pluto on a gigabit adapter), where there is room for both: you then hear the receiver through your own transmission, which is how a QO-100 station listens to its own downlink.

    An over needs twice the sample rate's worth of link — 2.5 Msps is 10 MB/s each way — so lower the rate if the log starts saying the link is not carrying it. The panadapter still shows the transmitted signal during an over; it is the audio that keeps coming.

    Takes effect on Apply.
settings-radio-duplex = デュプレックス
settings-radio-whether-ad9361-runs-both-directions =
    AD9361 が両方向を同時に動かすか（FDD。Pluto が起動するときの状態で、sdroxide が常にそのままにしてきたもの）、一度に一方向だけ動かすか（TDD）です。

    Leave this on FDD unless you want the PTT pins below — TDD is what those key from, and it rules out Full duplex above. Takes effect on Apply.
settings-radio-ptt-pins = PTT ピン
settings-radio-pluto-s-four-gpo-test =
    Pluto の 4 つの GPO テストポイントは、単独で外部のパワーアンプ、LNA、送受信スイッチをキーできます。2 本を選ぶと、受信中は一方のピンが常に High になり、送信中は他方が常に High になります。ホストソフトウェアは介在せず、配線する必要のあるシリアルの PTT 回線もありません。

    This puts the radio in TDD whatever the Duplex row says, because the pins follow the AD9361's enable lines and in FDD both of those are asserted the entire session. Full duplex goes off with it, so leave both at their defaults if you work satellites and need to hear your own downlink.

    Analog Devices' own note puts an external LNA on GPO0/GPO1, so use GPO2/GPO3 if your board is wired that way. The pins are about 1.3 V at a few milliamps: drive a transistor or an opto-isolator with them, never a relay coil.

    Takes effect on Apply.
settings-radio-buffer-size = バッファーサイズ
settings-radio-how-much-device-holds-before =
    デバイスが各転送の前に保持する量です。複素サンプル単位です。既定値の 32768 は 2 Msps で約 16 ms です。転送ごとの往復がボトルネックにならないほど長く、再同調が目に見えて遅れないほど短い値です。

    Halve it if the log reports the receive socket being replaced. A marginal link — a Pluto reached over its USB cable at a high sample rate is the usual one — stalls part-way through a transfer, and a smaller transfer is both less likely to be caught mid-flight and quicker to make good afterwards. Raise it to trade retune latency for fewer round trips.

    Takes effect on Apply.
settings-radio-samples = {" "}サンプル
settings-radio-rx-tx-port = RX / TX ポート
settings-radio-ad9361-s-rf-port-select = AD9361 の rf_port_select です。標準の Pluto は、それぞれ 1 つずつ配線されているため、そうでないボードを使う場合を除き、ここは空欄のままにしてください。適用で有効になります。
settings-radio-opens-radio-reads-what-says = 無線機を開き、自身について言うことを読み取り、この特定のボードが持つ同調範囲を報告します。ストリームは開始しません。
settings-radio-wideband-iq-receive-transmit-half = 広帯域 IQ の受信と送信。半二重です。USB ネットワークリンクは両方を同時に運べないため、送信中は受信が止まります。SoapySDR も libiio も不要です。
settings-radio-flexradio-announces-itself-local-network = FlexRadio は、ローカルネットワーク上で約 1 秒ごとに自分を告知します。ルーターや VPN 経由で到達する無線機は、あなたに向けて告知することはありません。その場合は、下にそのアドレスを入力してください。
settings-radio-another-gui-client-has-radio = 別の GUI クライアントがこの無線機を使用中で、multiFLEX は無効です。そのクライアントを切断するか、無線機で multiFLEX を有効にしてください。
settings-radio-overrides-selection-above-use-radio = 上の選択を上書きします。別のサブネット、VPN の背後、または標準的でないポートにある無線機にはこれを使います。
settings-radio-width-spectrum-sdroxide-receives-192 = sdroxide が受信するスペクトラムの幅です。192 kHz は DAX IQ ストリームの無線機の最大値で、このインターフェースが表示できる最も広いスパンです。
settings-radio-t-192-khz-widest-dax-iq = 192 kHz は FLEX が送る最も広い DAX IQ ストリームで、無線機はパナダプターのスパンをそれに結び付けます。つまりそれがスパン全体で、ここの設定でそれ以上広げることはできません。
settings-radio-dax-iq-channel = DAX IQ チャンネル
settings-radio-radio-has-four-change-only = 無線機には 4 つあります。ネットワーク上の別のものがすでにチャンネル 1 を使っている場合にのみ変更してください。無線機は、同じチャンネルを 2 回受け付けません。
settings-radio-station-name = ステーション名
settings-radio-shown-against-session-radio-s = 無線機のクライアント一覧で、このセッションと並んで表示されます。無線機もこの名前でクライアントを記憶するため、名前を変えると、無線機は sdroxide を新しいクライアントとして扱います。
settings-radio-gui-client-id = GUI クライアント ID
settings-radio-identity-radio-restores-session-s = 無線機がこのセッションのスライスとパナダプターを復元するための識別子です。空欄にすると、ステーション名から導かれます。安定していますが、既定の名前のままの sdroxide はどれも同じになり、無線機は重複を解決するために先のクライアントを切断します。sdroxide はそれをワイヤー上で検出し、1 セッション限りの識別子に切り替えます。その場合、復元はできなくなります。保持したい場合は、ここに自分の UUID を入力してください。
settings-radio-optional-e-g-uuid-your = 任意。例：自分の UUID
settings-radio-mirror-radio-s-i-q =
    無線機の I/Q を、パナダプターの中心を軸に反転します。既定ではオフです。FLEX-6600 はこの状態で検証されました。

    Try it if receive audio is unintelligible on USB *and* on LSB and nothing decodes, while the waterfall looks entirely convincing — that is what a mirrored stream looks like, and it is the one fault with no other symptom (issue #368). If it is not that, turning this on makes it obvious rather than subtle. Applies on Apply / reconnect.
settings-radio-network-mtu = ネットワーク MTU
settings-radio-largest-datagram-radio-may-send = 無線機が送ってよい最大のデータグラムです。1450 は SmartSDR 自身が要求する値です。より小さい MTU の経路（VPN やトンネル）では下げてください。フラグメントが落ちて、スペクトラムがまったく届かなくなるためです。
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = GUI クライアントとして登録せずに、無線機が応答するかを確認します。そのため、すでに実行中の SmartSDR のセッションを妨げません。
settings-radio-wideband-iq-receive-over-dax = DAX 経由の広帯域 IQ の受信。無線機が変調する音声の送信。再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-not-yet-verified-against-real-2 = 実機ではまだ検証されていません。不具合がある場合は、診断レポートを送ってください。無線機とやり取りされたすべてのプロトコル行が含まれています。
settings-radio-devices-soapysdr-can-see = SoapySDR が見つけられるデバイス
settings-radio-waiting-machine-radio-attached-where = モジュールがインストールされている、無線機が接続されているマシンを待っています。開くデバイスは、そのマシンの config.toml の device_args、または --device で指定されます。
settings-radio-ask-every-installed-soapysdr-module = インストールされているすべての SoapySDR モジュールにスキャンを依頼します。何も開かないため、受信中でも安全ですが、少し時間がかかることがあります。
settings-radio-not-enumerated-yet-press-rescan = まだ列挙されていません。再スキャンを押してください。
settings-radio-no-soapysdr-devices-found-check = SoapySDR デバイスが見つかりません。お使いの無線機のモジュールがインストールされていて、デバイスにアクセスできるか確認してください。
settings-radio-not-radio = 無線機ではありません
settings-radio-sound-card-listed-above-if = 上に、SDR であるかのようにサウンドカードが表示されています。どの周波数も受け付けて無視するため、それを開くと、バンドではなくサウンドカードの入力が表示されます。sdroxide はそれらを自動では選びませんが、それを指定する device_args の行は、依然として従われます。
settings-radio-which-one-opens-set-device = どれを開くかは、--device か config.toml の device_args で設定されます。どちらもない場合は、一覧の最初の無線機が選ばれます。
settings-radio-no-device = デバイスなし
settings-radio-rig-has-no-software-adjustable = このリグにはソフトウェアで調整できるゲインがありません。
settings-radio-rx-gains = RX ゲイン
settings-radio-tx-gains = TX ゲイン
settings-radio-antennas = アンテナ
settings-radio-remembered-next-start = 次回の起動のために記憶されます。
settings-radio-these-controls-appear-once-soapysdr = これらのコントロールは、SoapySDR デバイスが開かれると表示されます。デバイス自身のもので、デバイスに問い合わせる必要があるためです。
settings-radio-stream = ストリーム
settings-radio-follow-app-setting = アプリの設定に従う
settings-radio-msps = または Msps 単位で
settings-radio-baseband-filter = ベースバンドフィルター
settings-radio-let-driver-choose = ドライバーに選ばせる
settings-radio-changing-either-reopens-radio = どちらを変更しても無線機は開き直されます。
settings-radio-driver-publishes-no-settings-own = このドライバーは独自の設定を公開していません。
settings-radio-applied-once-remembered-next-start = すぐに適用され、次回の起動のために記憶されます。
settings-radio-no-rx-888-found-press = RX-888 が見つかりません。再スキャンを押してください
settings-radio-adc-clock = ADC クロック
settings-radio-t-129-6-msps-needs-superspeed = 129.6 Msps には SuperSpeed のリンクと高速なホストが必要です。64.8 が安全な既定値です。変更すると受信機が自動的に開き直されます。少し時間がかかりますが、再起動は不要です。
settings-radio-panadapter-width = パナダプターの幅
settings-radio-hf-only = {"  "}· HF のみ
settings-radio-usable-both-front-ends = 両方のフロントエンドで使用できます。
settings-radio-below-vhf-crossover-ordinary-width = VHF のクロスオーバーより下では、これは通常の幅です。それより上では、チューナーの 8 MHz の IF を、この幅の窓の中央に置くことができず、ライブのスペクトラムはパナダプターの片側に寄り、その横には何もなくなります。
settings-radio-dial-above-crossover-now-so = 現在、ダイヤルがクロスオーバーより上にあるため、この幅ではチューナーの 8 MHz が中央から外れ、その横に無信号のスペクトラムが表示されています。パナダプターを満たすように幅を狭めてください。
settings-radio-vga-gain = VGA ゲイン
settings-radio-ad8370-variable-gain-amplifier-ahead = ADC の前にある AD8370 可変ゲインアンプです。
settings-radio-attenuator = 減衰器
settings-radio-pe4304-step-attenuator-0-5 = PE4304 ステップ減衰器、0.5 dB 刻みです。
settings-radio-adc-range = ADC レンジ
settings-radio-wide-2-25-vp-p = 広い（2.25 Vp-p）
settings-radio-selects-adc-s-wider-input = ADC のより広い入力範囲を選びます。強い放送信号に対する余裕は増えますが、弱い信号に対する分解能は下がります。オフにすると、より高感度な 1.5 Vp-p の範囲になります。
settings-radio-dither = ディザー
settings-radio-adds-small-dither-signal-ahead = ADC の前に小さなディザー信号を加えます。ノイズフロアは少し上がりますが、スプリアスフリーのダイナミックレンジが得られます。
settings-radio-randomiser = ランダマイザー
settings-radio-adc-scrambles-output-so-digital = ADC は出力をスクランブルするため、デジタルバスがフロントエンドに放射しなくなります。ドライバーがそれを元に戻します。デバッグ中でない限り、これはオンのままにしてください。再接続時に適用されます。
settings-radio-dc-hf-antenna-port = HF アンテナポートの DC
settings-radio-powers-active-antenna-preamp-down = 同軸ケーブルを通してアクティブアンテナやプリアンプに電源を供給します。
settings-radio-vhf-tuner-gain = VHF チューナーのゲイン
settings-radio-r828d-rf-gain-used-above = R828D の RF ゲインで、自動の HF/VHF クロスオーバーより上で使われます。29 段階の離散値で、最も近いものが使われます。
settings-radio-auto = 自動
settings-radio-let-tuner-run-own-lna = チューナーに、独自の LNA とミキサーのループを動かさせます。
settings-radio-bias-tee-vhf = バイアス T（VHF）
settings-radio-dc-vhf-antenna-port = VHF アンテナポートの DC
settings-radio-clock-trim = クロックのトリム
settings-radio-corrects-reference-oscillator-applied-when = 基準発振器を補正します。値を離したときに適用されます。ドラッグのピクセルごとに開き直すと、受信機が何百回も再起動してしまうためです。
settings-radio-receive-only-below-adc-s = 受信のみ。ADC のナイキスト限界より下では、アンテナが直接サンプリングされ、ハードウェアのダウンコンバーターがないため、再同調は即座に行われます。ADC のストリーム全体がホストでベースバンドに変換されます。それより上では、受信機は自動的に R828D チューナーと VHF の SMA に切り替わるため、両方のアンテナを接続する必要があります。VHF では、チューナーの IF が収まるように ADC クロックが 32.4 Msps 以上必要です。また、クロックが 48 Msps 未満では、2 つの範囲の間に、どれも届かない隙間があります。ここのすべての設定は直ちに適用されます。押す適用ボタンはありません。
settings-radio-device = デバイス
settings-radio-no-devices-press-rescan = デバイスがありません。再スキャンを押してください
settings-radio-six-rates-are-six-different =
    6 つのレートは、6 つの異なる FPGA イメージであり、レジスターではありません。そのため、ELAD のプロトコルではそれらを選ぶ手段がなく、この設定が何を意味するかは機種によって異なります。

    FDM-S1 / FDM-S2: a command. Their FPGA is loaded by the computer at every power-up, and sdroxide runs ELAD's own elad-firmware loader to put this rate's image in. Without that loader installed a sampler sends no samples at all, and sdroxide says so on screen.

    FDM-DUO: not a command. The radio boots its own FPGA and arrives at whatever rate it powered up in or FDM-SW2 last left it in (192 kHz on a fresh one), and this only says how the stream is READ. Get it wrong and the panadapter is simply the wrong width, with every frequency inside it scaled to match.

    The stream's real rate is measured a couple of seconds after it starts, and a mismatch is reported on screen. Takes effect on Apply.
settings-radio-fdm-s-fpga-image-loaded = FDM-S：読み込まれた FPGA イメージ — FDM-DUO：ストリームの読み取り方法
settings-radio-receiver-s-input-pad-one = 受信機の入力パッドです。1 ステップ、入または切です。メインウィンドウのゲインスライダーと同じコントロールです。
settings-radio-pre-selection-filters = 事前選択フィルター
settings-radio-low-pass-bank-front-adc = ADC の前にあるローパスフィルターのバンクです。これを迂回すると最も広い表示になりますが、帯域外の強い信号の近くでは最悪の動作になります。数マイル離れた放送送信機が、バンド全体にイメージを生じさせます。フィルターされた範囲の外を意図的に聴くのでない限り、オンのままにしてください。
settings-radio-filters-circuit = 回路に入っているフィルター
settings-radio-rig-control-fdm-duo-only = リグの制御 — FDM-DUO のみ
settings-radio-transceiver-s-cat-usb-port = トランシーバーの CAT USB ポートです。受信用とは別のケーブルです。ポートを持たない FDM-S1 と FDM-S2 では未設定のままにしてください。FDM-DUO では、受信ケーブルだけで操作する方が良いでしょう。その場合、同調と送信はできますが、メーターは読めません。
settings-radio-port-set-radio-s-display = ポートを設定すると、無線機の表示とこのダイヤルは、双方向で同じ数値になります。FDM-DUO の受信窓は VFO を中心に構成されているため、ここで同調すると前面パネルが動き、前面のつまみを回すとここの表示が動きます。パナダプターは、ダイヤルに合わせて移動した中心へ再び中央に置かれます。ポートがない場合、または無線機が応答しないポートの場合は、指令すべき VFO がないため、同調は無線機がすでに送っている窓の中にとどまります。
settings-radio-none-usb-control-only = — なし（USB 制御のみ） —
settings-radio-must-match-menu-70-cat = 無線機のメニュー 70「CAT BAUD」と一致させる必要があります。出荷時は 38400 です。無線機にはこの 4 つのレートしかなく、それ以外はありません。別のレートで開かれたポートは、双方向に無音になります。無線機はダイヤルを無視し、キーを拒否しますが、その理由は示しません。
settings-radio-how-transmit-keyed-cat-sends = 送信をどうキーするか。「CAT」は無線機自身の TX コマンドを送り、リグ側の設定は不要です。RTS には、メニュー 54「PTT」を PTT+RTS に設定する必要があります。
settings-radio-where-radio-takes-transmit-audio-2 =
    無線機が送信音声をどこから取るかです。リグの TI コマンドで、前面パネルのメニュー 32「TX IN」にあたります。ポートを開いたときに設定されます。

    "USB audio" is what makes transmit work here: the FDM-DUO sends what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.
settings-radio-which-two-sockets-back-receiver-2 =
    受信機が背面の 2 つのソケットのどちらで受信するかです。リグの AN コマンドで、前面パネルのメニュー 31「ANTENNAS」と、ディスプレイの「ANT 1 2」表示にあたります。

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    It moves this whole receiver: the panadapter, the demodulators and the rig's own audio all come from the socket selected here.

    Applies immediately, and is read back from the radio when the control port opens — this is the rig's own setting, not a copy of it kept here.
settings-radio-how-often-radio-asked-what-2 = 無線機に、何をしているか（ダイヤル、モード、メーター）を問い合わせる頻度です。リグ自身の VFO つまみを回すと、表示は 1 回のポーリング以内に追従します。また、これはこちら側が生成する制御トラフィックのすべてでもあるため、無線機の音声が途切れる場合は下げてください。
settings-radio-transmit-audio = 送信音声
settings-radio-only-receiving-fdm-duo-has = FDM-DUO での受信のみが、これまでに実機で動作確認されています。何か不具合があれば、診断レポートをバグ報告に添付してください。
settings-radio-cw-keyed-radio-s-own = CW は無線機自身のキーまたはパドルでキーされます。FDM-DUO には文字列を受け付けるコマンドがないため、CW パネルではキーできません。
settings-radio-press-apply-reopen-radio = 無線機を開き直すには、適用を押してください。
settings-radio-no-receivers-press-rescan = 受信機がありません。再スキャンを押してください
settings-radio-which-rates-receiver-has-depends = 受信機がどのレートを持つかは、機種とファームウェアによって異なります。適用で有効になります。
settings-radio-every-rate-any-hf-model = どの HF+ 機種が提供するすべてのレート — 接続して、それぞれの一覧を確認してください
settings-radio-receiver-s-own-gain-control = 受信機自身のゲインコントロールです。一般の受信にはオンのままにしてください。測定のために減衰器を手動で設定する場合はオフにします。
settings-radio-agc-threshold = AGC のしきい値
settings-radio-high = 高
settings-radio-high-trades-little-sensitivity-headroom = 高では、感度を少し犠牲にして、強い隣接信号に対する余裕を得ます。混雑した夜のバンドに適した設定です。
settings-radio-front-end-attenuation-gain-0 = フロントエンドの減衰（ゲインとして表示）。0 dB は減衰なしです。AGC がオフのときにのみ有効です。
settings-radio-preamp = プリアンプ
settings-radio-hf-low-noise-amplifier-buys = HF の低雑音アンプです。感度を得る代わりに相互変調が増えるため、既定ではオフです。実際のアンテナでは、通常それが適切です。
settings-radio-lna = LNA オン
settings-radio-frequency-calibration = 周波数の校正
settings-radio-parts-per-billion-receiver-s = 十億分率（ppb）です。この受信機固有の単位で、RTL-SDR が使う ppm の 1000 倍細かい単位です。ここで設定したものが受信機のフラッシュに書き込まれることはありません。この値はセッションの間だけ、保存された値を上書きします。
settings-radio-use-receiver-s-stored-value = 受信機に保存された値を使う
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = 同軸ケーブルに DC を供給
settings-radio-not-every-hf-has-one = すべての HF+ にあるわけではありません。ない受信機では、これは何もしません。
settings-radio-host-dsp = ホスト DSP
settings-radio-image-balancer-zero-if-offset = イメージバランサー、ゼロ IF オフセット、微調整発振器です。生のハードウェアの出力を見るためにのみオフにしてください。オフの場合、ゼロ IF のレートでイメージが現れ、ダイヤルの精度は 1 kHz 単位までになります。
settings-radio-correct-image-fine-tune = イメージを補正して微調整
settings-radio-receive-only-no-soapysdr-no = 受信のみ。SoapySDR も libairspyhf も不要です。シンセサイザーの下限より下では、ホストの発振器が同調を行います。これにより、この受信機は VLF に届きます。受信機とサンプルレートは適用で有効になります。それ以外は変更と同時に適用されます。
settings-radio-not-yet-verified-against-real-3 = 実機ではまだ検証されていません。不具合がある場合は、診断レポートを送ってください。受信機とやり取りされたすべてのコマンドと、I/Q ペアとしてデコードされたサンプルストリームの最初のバイトが含まれています。
settings-radio-no-airspy-r2-mini-found = Airspy R2 または Mini が見つかりません。再スキャンを押してください
settings-radio-read-receiver = この受信機から読み取る
settings-radio-r2-mini-offer-different-rates = R2 と Mini は異なるレートを提供し、どちらかを開くまで区別できません
settings-radio-rate-you-get-receiver-s = これが得られるレートです。受信機の ADC はその 2 倍で動作します。実信号をデジタル化し、sdroxide がそこからホストで複素ベースバンドを作ります。
settings-radio-gain-curve = ゲイン曲線
settings-radio-step-along-curve-above-not = 上の曲線に沿った 1 ステップで、dB の値ではありません。チューナーの LNA、ミキサー、VGA は一緒に動き、各ステップの値は曲線とバンドによって異なります。0 が静かな側の端です。
settings-radio-tuner-agc = チューナー AGC
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = 曲線を上書きして、LNA をチューナー自身のループに渡します。
settings-radio-mixer = ミキサー
settings-radio-same-mixer-stage = ミキサー段でも同じです。
settings-radio-loop-running-gain-slider-no = ループが動いている間は、ゲインスライダーは、ループが所有する段を設定しなくなります。ループがそれを上書きするためです。
settings-radio-dc-antenna-port = アンテナポートの DC
settings-radio-t-12-bit-packing = 12 ビットのパッキング
settings-radio-third-less-usb-traffic-leave = USB のトラフィックが 3 分の 1 少なくなります。オンのままにしてください。これは USB 2.0 のデバイスで、最大レートはパッキングすると 30 MB/s、しないと 40 MB/s です。再接続時に適用されます。
settings-radio-dc-removal = DC 除去
settings-radio-remove-adc-s-offset = ADC のオフセットを除去します
settings-radio-turn-off-see-raw-hardware = 生のハードウェアの出力を見るにはオフにします。スプリアスがどこに落ちるかを知っておくとよいでしょう。信号は途中でサンプルレートの 4 分の 1 だけ移されるため、オフセットはスパンの中心ではなく、端に現れます。
settings-radio-every-command-exchanged-receiver-sample = 受信機とやり取りされたすべてのコマンド、サンプルレートの計算、I/Q としてデコードされた最初のサンプルです。
settings-radio-receive-only-24-1800-mhz = 受信のみ、24〜1800 MHz。SoapySDR も libairspy も不要です。受信機、サンプルレート、パッキングは適用で有効になります。それ以外は変更と同時に適用されます。
settings-radio-not-yet-verified-against-real-4 = 実機ではまだ検証されていません。不具合がある場合は、診断レポートを送ってください。受信機とやり取りされたすべてのコマンド、レートの計算、I/Q ペアとしてデコードされた最初のサンプルが含まれています。
settings-radio-no-hydrasdr-rfone-found-press = HydraSDR RFOne が見つかりません。再スキャンを押してください
settings-radio-these-are-rates-receiver-turned = これらは、この受信機が持っていることがわかったレートです
settings-radio-receiver-only-reports-three-these = 受信機が報告するのは、これらのうち 3 つだけです。残りはファームウェアの代替テーブルにあり、古いビルドでは持っていない場合があります
settings-radio-rf-input = RF 入力
settings-radio-antenna-sma-only-socket-bias = アンテナ SMA です。バイアス T が接続されている唯一のソケットです。
settings-radio-cable-socket-no-bias-tee = ケーブルのソケットです。バイアス T はここにはなく、ハードウェアは ANT だけに付けています。
settings-radio-powers-active-antenna-preamp-down-2 = 同軸ケーブルを通してアクティブアンテナやプリアンプに電源を供給します。電源を供給するのは ANT ソケットだけです。2 つのケーブルポートは単なる入力です。
settings-radio-third-less-usb-traffic-leave-2 = USB のトラフィックが 3 分の 1 少なくなります。オンのままにしてください。これは USB 2.0 のデバイスで、最大レートはパッキングすると 36 MB/s、しないと 48 MB/s です。再接続時に適用されます。
settings-radio-one-these-boards-1d50-60a1 = これらのボードの 1 つは 1d50:60a1 で、HydraSDR のプロトタイプが Airspy R2 と Mini と共有している USB ID です。sdroxide は開いた後にファームウェアを確認し、誤ったインターフェースが選ばれている場合は、どちら向きでもその旨を伝えます。
settings-radio-receive-only-24-1800-mhz-2 = 受信のみ、24〜1800 MHz。SoapySDR も libhydrasdr も不要です。受信機、サンプルレート、パッキングは適用で有効になります。それ以外は変更と同時に適用されます。
settings-radio-re-list-fobos-sdrs-no = Fobos SDR を再度一覧表示します。デバイスは開かれないため、受信中に押しても安全です。
settings-radio-no-fobos-sdr-found-press = Fobos SDR が見つかりません。再スキャンを押してください
settings-radio-input-2 = 入力
settings-radio-through-tuner-lna-vga-mixer = チューナー経由：LNA、VGA、およびミキサー独自の周波数範囲です。
settings-radio-direct-sampling-hf1-antenna-input = HF1 アンテナ入力の直接サンプリング。チューナー、LNA/VGA のゲインはなく、すべてソフトウェアで同調されます。
settings-radio-direct-sampling-hf2-antenna-input = HF2 アンテナ入力の直接サンプリング。HF1 と同じです。
settings-radio-both-real-adc-channels-once = 2 つの実 ADC チャンネルを同時に使い、下のダイバーシティフィルターで結合します。ノイズ源をヌルにするか、2 つのフェージング経路を結合します。
settings-radio-direct-sampling-tuner-so-lna = 直接サンプリング。チューナー（つまり LNA/VGA のゲイン）は経路に入りません。下のレートはソフトウェアのダウンコンバーターの目標値で、ハードウェアの設定ではないため、実際のレートはこれと異なる場合があります。
settings-radio-receiver-s-own-reported-rates = この受信機が報告するレート
settings-radio-measured-one-real-unit-connect = 実機 1 台で測定しました。この受信機を接続して再スキャンすると、その一覧が表示されます
settings-radio-clock-source = クロックの供給元
settings-radio-external-reference = 外部基準
settings-radio-diversity-taps = ダイバーシティのタップ
settings-radio-taps = {" "}タップ
settings-radio-one-tap-gain-phase-null = 1 つのタップはゲインと位相です。ある周波数でのヌルで、その両側では悪化していきます。アナログのフェーザーができるのは、これだけです。タップを増やすごとに、HF1 と HF2 の経路の差のうち、1 サンプル周期分を、フィルターが等化できるようになります。そのため、そのノッチが全域で静かな帯域になります。
settings-radio-receive-only-rf-port-tuner = 受信のみ。RF ポート（チューナー）、HF1/HF2（直接サンプリング、ソフトウェアでデコード）、HF1+HF2（同時に、ダイバーシティフィルターで結合）はすべて動作します。受信機、入力、サンプルレートは適用で有効になります。ゲイン、クロックの供給元、タップは変更と同時に適用されます。ダイバーシティフィルターのモード、適応レート、保持は、この無線機を選択すると、メインウィンドウ自身の DIV ボックスにあります。
settings-radio-no-hackrf-found-press-rescan = HackRF が見つかりません。再スキャンを押してください
settings-radio-front-end-amplifier-8-db = フロントエンドのアンプで、8 dB 刻みです。感度を変える段であり、実際のアンテナでは最初に過負荷になる段です。
settings-radio-baseband-amplifier-after-mixer-2 = ミキサーの後のベースバンドアンプで、2 dB 刻みです。LNA に手を伸ばす前に、弱い信号ではこれを上げてください。
settings-radio-rf-amp = RF アンプ
settings-radio-one-switch-shared-transmit-setting = 1 つのスイッチで、下の送信の設定と共有されます。無線機は、そのとき入る方向に属する方の設定を適用します。実際のアンテナでは、通常オフが適切です。
settings-radio-hackrf-pro-chooses-itself-three = HackRF Pro は、これを自分で選びます。サンプルレートの 4 分の 3 で、FPGA でフィルターされます。ホストが要求するものは何でも無視します。
settings-radio-leave-automatic-choosing-one-too = 自動のままにしてください。狭すぎるものを選ぶと、帯域端が柔らかくなるだけでなく、信号から DC スパイクを遠ざけている LO オフセットも取り去られてしまいます。
settings-radio-about-3-v-50-ma = 同軸ケーブルに約 3 V、50 mA です。アクティブアンテナやプリアンプ用です。HackRF One または Pro のみで、Jawbreaker と rad1o にはそのような回路はありません。
settings-radio-remove-dc-mirror-image = DC とイメージを除去
settings-radio-zero-if-radio-so-own = これはゼロ IF の無線機なので、その LO の漏れがスパンの中央に現れ、ミキサーの直交誤差がその上にイメージを生じさせます。これをオフにすると生のハードウェアの出力が見えます。これが、ドライバーの問題と DSP の問題を見分ける手早い方法です。
settings-radio-corrects-reference-oscillator = 基準発振器を補正します。
settings-radio-transmit = 送信
settings-radio-transmitter = 送信機
settings-radio-off-default-while-off-backend = 既定ではオフです。オフの間、バックエンドは送信チャンネルをまったく公開しないため、無線機をキーすることはできません。再接続時に適用されます。
settings-radio-tx-vga-gain = 送信 VGA ゲイン
settings-radio-transmit-driver-amplifier-drive-applied = 送信ドライバーアンプです。駆動はこの段の前にデジタルで適用されるため、駆動は高いままにして、出力レベルはここで設定してください。駆動を下げると、DAC がフルスケールのごく一部で動作し、相互変調が増えます。
settings-radio-same-switch-receive-setting-above = 上の受信の設定と同じスイッチで、無線機が方向を切り替えるときに適用されます。
settings-radio-transmit-armed-into-dummy-load = 送信は待機状態です。測定するまではダミーロードに送信してください。HackRF の高調波は強く、使用しているバンド用の外部ローパスフィルターが必要なほどです。また半二重のため、送信のたびに受信が止まります。
settings-radio-every-command-exchanged-radio-session = このセッションで無線機とやり取りされたすべてのコマンドを、順番どおりに示します。各キーダウンの前後の順序も含みます。
settings-radio-dc-7-25-ghz-half = DC〜7.25 GHz、半二重。SoapySDR、libusb、libhackrf は不要です。無線機とサンプルレートは適用で有効になります。それ以外は変更と同時に適用されます。帯域端はファームウェアのものです。HackRF は 1 MHz〜6 GHz（Pro では 100 kHz）に対応するように規定されており、その外側では大きく減衰しますが、そこにも同調はできます。
settings-radio-ask-sdrplay-api-service-device = SDRplay API サービスにデバイス一覧を問い合わせます。何も開かないため、受信中に押しても安全です。
settings-radio-no-rsps-press-rescan-needs = RSP がありません。再スキャンを押してください（SDRplay API サービスが必要です）
settings-radio-both-tuners-running-api-fixes = 両方のチューナーが動いている場合、API は ADC を 6 MHz に固定し、低い IF から 2 Msps を返します。そのため、2 Msps が最も広いスパンで、それより狭いものはその間引きされたものです。適用で有効になります。
settings-radio-rates-below-2-msps-run = 2 Msps より低いレートでは、ADC は 2 Msps で動作し、サービス内で間引きを行います。適用で有効になります。
settings-radio-reduced-adc-resolution = {"  "}（ADC の分解能が低下）
settings-radio-if-bandwidth = IF 帯域幅
settings-radio-tuner-s-analog-filter-auto = チューナーのアナログフィルターです。自動では、サンプルレートに収まる最も広いものを選びます。適用で有効になります。
settings-radio-rsp-s-own-if-gain = API サービスが動かす、RSP 自身の IF ゲインのループです。オフにすると、IF ゲインのスライダーがあなたの手に戻ります。測定や微弱信号のデジタルモード用の設定です。
settings-radio-agc-set-point = AGC の設定点
settings-radio-signal-level-loop-holds-adc = ループが ADC を保つ信号レベルです。低くすると、チャンネル外の信号に対する余裕が増えます。どこまで下げられるかはサンプルレートによります。8.064 Msps を超えると、コンバーターが与えられる余裕は減り、9.216 Msps を超えるとさらに減ります。
settings-radio-rsp-s-native-gain-unit = RSP のネイティブなゲインの単位です。20 dB が最大ゲイン（下の拡張範囲では 0）、59 dB が最小です。即座に適用されます。AGC の動作中は無視されます。その場合はループがこの値を所有し、S メーターはそれが落ち着いた値を示します。
settings-radio-extended-if-range = 拡張 IF 範囲
settings-radio-lets-if-gain-reduction-go = IF のゲイン減衰を 20 dB より下、0 まで下げられるようにします。これは、受信機が持つ最後の 20 dB のゲインです。オフは API の既定値で、通常の受信に適しています。範囲の下端は、RSP が最も過負荷になりやすい位置だからです。LNA がすでに 0 にある弱い信号には、オンにする価値があります。また、AGC の設定点を 0 dBFS まで上げられるようにもします。即座に適用されます。
settings-radio-allow-if-gain-reduction-below = IF のゲイン減衰を 20 dB より下にも許可
settings-radio-front-end-attenuation-steps-0 = フロントエンドの減衰を段階で指定します。0 が最大ゲインで、段階を上げるとより多くの減衰が入ります。バンドによっては段階が少ないものもあります。ドライバーは範囲内に収め、あなたの選択は、同調し直したときのために保持します。ここのスライダーは、無線機があるバンドが受け付ける上限で止まります。別のバンドで選んだより高い値は保持され、その横に表示されます。即座に適用されます。
settings-radio-t-0-max-gain = 0 = 最大ゲイン
settings-radio-reference-error-parts-per-million-2 = 基準の誤差（百万分率）で、デバイス自身が適用します。即座に適用されます。
settings-radio-radio-s-tuner = この無線機のチューナー
settings-radio-tuner = チューナー
settings-radio-which-rspduo-s-two-tuners = この無線機が聴く RSPduo の 2 つのチューナーのうちどちらかです。もう一方は、それに設定された無線機に属します。そちらには同じ受信機と、もう一方のチューナーを設定してください。適用で有効になります。
settings-radio-which-rspduo-s-two-tuners-2 = 聴いているアンテナを担当する、RSPduo の 2 つのチューナーのどちらかです。もう一方は、2 本目のアンテナを担当します。適用で有効になります。
settings-radio-which-rspduo-s-two-tuners-3 = RSPduo の 2 つのチューナーのどちらを動かすか（一度に 1 つ）です。適用で有効になります。
settings-radio-applies-immediately = 即座に適用されます。
settings-radio-dab-notch = DAB ノッチ
settings-radio-hardware-notch-over-165-230 = 165〜230 MHz の DAB 帯のハードウェアノッチです。即座に適用されます。
settings-radio-hdr-mode = HDR モード
settings-radio-enable-below-2-mhz = 2 MHz より下で有効
settings-radio-rspdx-s-high-dynamic-range =
    RSPdx の LF/MF 用の高ダイナミックレンジ経路です。即座に適用されます。

    It does not work yet: on the one RSPdx it has been tried against, switching it on silences the receiver — at every centre, filter, port, gain and rate tried. Whatever the mode needs is not on the documented API surface, and finding it is somebody's next job. Leave this off unless you are the one looking.

    What is known: it is not a mode that follows the dial. The path has a fixed analog filter, built only at the centres listed under the filter below, so tuning anywhere else could not work even once the rest does.
settings-radio-hdr-filter = HDR フィルター
settings-radio-feed-4-7-v-dc = 同軸ケーブルに約 4.7 V の DC を供給
settings-radio-second-tuner = 2 つ目のチューナー
settings-radio-rspduo-two-whole-tuners-one = RSPduo は 1 つの基板上の 2 つの完全なチューナーで、1 つの基準から同じクロックを受けます。両方を動かすと、同じクロックから同じ瞬間にスパンを聴くため、2 本のアンテナが干渉性を持ち、相対位相は偶然ではなく給電線によって決まり、2 つを組み合わせることができます。あるいは別々にしておけます。チューナーは個別に同調するため、もう一方を、独自のバンドの 2 つ目の無線機にすることもできます。
settings-radio-run-both-tuners = 両方のチューナーを動かす
settings-radio-puts-rspduo-api-s-dual = RSPduo を API のデュアルチューナーモードにします。このモードでは ADC は 6 MHz に固定され、最も広いスパンは 2 Msps です。モードはデバイスを開くときに選ばれるため、適用で有効になります。
settings-radio-used = 用途
settings-radio-what-other-tuner-takes-effect = もう一方のチューナーの用途です。適用で有効になります。両方を動かすこと自体と同様に、これは基板を開くときに選ばれます。
settings-radio-add-second-radio-settings-radio = 2 台目の無線機を追加します（設定 → 無線機 → +）。同じ受信機と、RSPduo のもう一方のチューナーを与え、そちらも両方のチューナーを動かすように設定してください。基板を開いた無線機がデュアルチューナーモードにするため、両方がそれを想定している必要があります。その後、2 つは独立して同調します。一方のタブで HF、もう一方のタブで VHF を、1 つの基板から扱えます。クロックは 1 つを共有するため、基板を開いた無線機が要求したサンプルレートで両方が動きます。どちらも送信はできません。
settings-radio-what-do = 用途
settings-radio-lna-state-2 = LNA の状態
settings-radio-second-tuner-s-own-front = 2 つ目のチューナー自身のフロントエンドの減衰です。両方のアンテナが、ほぼ同じノイズフロアを示すように設定してください。他のすべてがこの調整に依存します。合成は、2 つの枝をそのノイズで重み付けするため、過負荷に駆動された 2 つ目のフロントエンドは、干渉の歪んだコピーをフィルターに渡すことになり、それは歪みのない方からは差し引けないからです。即座に適用されます。
settings-radio-if-gain-reduction-2 = IF のゲイン減衰（2 つ目）
settings-radio-second-tuner-s-if-gain = 2 つ目のチューナーの IF ゲインで、RSP のネイティブな単位です。20 dB が最大ゲインです。AGC の動作中は無視されます。フィルターは安定したゲインを求めるため、保持したいヌルがある場合は、AGC をオフにする価値があります。
settings-radio-filter-length = フィルターの長さ
settings-radio-one-tap-gain-phase-null-2 = 1 つのタップはゲインと位相です。ある周波数でのヌルで、その両側では悪化していきます。アナログのフェーザーができるのは、これだけです。タップを増やすごとに、フィルターが等化できる 2 つのアンテナの経路差のうち、1 サンプル周期分が得られます。そのため、そのノッチが全域で静かな帯域になります。
settings-radio-adaptation-2 = 適応
settings-radio-slow-steady-left-converging-inside = 左では遅くて安定しており、1 秒の何分の一かの間に収束します。右では目に見えて探し回ります。最初は速くしてヌルを見つけ、その後で保持してください。
settings-radio-stop-filter-moving-reach-moment = フィルターの動きを止めます。ヌルが現れた瞬間にこれに手を伸ばしてください。適応したままのフィルターは、最も大きくなったものに自分を合わせ直します。静かなバンドでは、それがあなたが聴いている局になります。
settings-radio-restart = 再開
settings-radio-zero-filter-find-null-again = フィルターをゼロに戻し、ヌルをもう一度見つけます。
settings-radio-nothing-here-can-tell-wanted = ここでは、欲しい信号と欲しくない信号を区別できません。フィルターが知っているのは、2 つのアンテナが共通して持つものだけです。キャンセルでは、2 本目のアンテナはノイズ源を聴き、バンドはできるだけ聴かないことを望みます。さもないと、律儀に局まで打ち消してしまいます。合成では、両方が同じ局を聴くことを望みます。ヌルがどれだけ深くなっているかは、数秒ごとにログに出力されます。
settings-radio-not-yet-verified-against-rspduo = RSPduo ではまだ検証されていません。ここのデュアルチューナーの動作は、実機で測定したものではなく、SDRplay の API から書かれています。
settings-radio-no-receiver-listed-check-against = 照合するための受信機が一覧にありません。再スキャンを押すか、実際に開かれたものをログで確認してください。
settings-radio-receive-only-1-khz-2 = 受信のみ、1 kHz〜2 GHz。ベンダーの SDRplay API サービス（sdrplay.com/api）が必要です。オリジナルの RSP1 以降の RSP には、公開されたプロトコルがありません。デバイス、サンプルレート、帯域幅、RSPduo のチューナーは適用で有効になります。それ以外は変更と同時に適用されます。
settings-radio-board = 基板
settings-radio-first-one-found-2 = 最初に見つかったもの
settings-radio-receive-chain = 受信チェーン
settings-radio-both-chains-tune-together-they = 両方のチェーンは一緒に同調します（1 つのシンセサイザーを共有）が、別々のソケットにある独立したフロントエンドです。アンテナがつながっているものを選んでください。1 つのチェーンが HF のマッチング改造を受けており、もう一方が標準の場合に、これを設定します。
settings-radio-receive-gain = 受信ゲイン
settings-radio-one-combined-figure-which-limesuite = 1 つにまとめた値で、LimeSuite が LNA、TIA、PGA に自分で配分します。整数のデシベル単位のため、それより細かい値は切り捨てられます。
settings-radio-receive-port = 受信ポート
settings-radio-automatic-follows-frequency-lnal-low = 自動では周波数に従います。LNAL は低い周波数で、LNAH は高い周波数です。ただし、下に LimeRFE が接続されている場合を除きます。その場合は 1 本のケーブルを 1 つのソケットに接続するため、すべての周波数で LNAW になります。ソケットが別のものに配線されている場合は、そのソケットの名前を指定してください。そのソケットがどちらのチェーンに属するかは、上の選択で決まります。
settings-radio-t-0-follows-sample-rate-worth = 0 はサンプルレートに従います。そのままにしておく価値があります。スパンの 4 分の 1 より狭いフィルターは、ゼロ IF の LO オフセットを黙って取り去るため、LO の漏れが聴いている信号の上に戻ってしまいます。
settings-radio-following-rate = レートに従う
settings-radio-corrections = 補正
settings-radio-host-iq-dc-correction = ホストの IQ / DC 補正
settings-radio-adaptive-image-dc-removal-side = チップ自身の校正の上に、こちら側で適応的なイメージと DC の除去を行います。オフにするのは、ドライバーの問題と DSP の問題をワンクリックで見分ける方法です。
settings-radio-calibrate-automatically = 自動で校正
settings-radio-runs-chip-s-own-dc = 無線機を開いたときに、チップ自身の DC オフセットとイメージの校正を行い、ダイヤルが新しいバンドまたは別のソケットに落ち着いたら、再び行います。それらの数値は 1 つの周波数で測定されるため、ほかの場所では誤りです。スパンの中央にあるキャリアは、たいていそうです。毎回約 1 秒かかります。
settings-radio-calibrate-now = 今すぐ校正
settings-radio-stalls-receiver-better-part-second = 受信機を、およそ 1 秒間停止させます。
settings-radio-second-aerial = 2 本目のアンテナ
settings-radio-socket = ソケット
settings-radio-same-first = 1 本目と同じ
settings-radio-gain-2 = ゲイン
settings-radio-set-low-coupled-sample-your = これは低く設定してください。自分の送信機の結合されたサンプルは強い信号であり、圧縮に駆動されたフィードバックのチェーンは、アンプの曲線を誤って測定します。補正が自分の歪みを学習してしまうためです。下から始めて、カプラーの減衰器を使ってください。
settings-radio-set-so-both-aerials-show = 両方のアンテナが、ほぼ同じノイズフロアを示すように設定してください。これは全体が依存する調整です。合成は、2 つの枝をそのノイズで重み付けし、圧縮に駆動された 2 つ目のチェーンは、干渉の歪んだコピーをフィルターに渡すからです。それは歪みのないものからは差し引けません。
settings-radio-table-steps-2 = テーブルのステップ
settings-radio-how-finely-correction-follows-amplifier = 補正がアンプの曲線にどれだけ細かく従うかです。ステップが多いほど鋭い屈曲を追いますが、それぞれが、そこに落ちたサンプルから学習されなければなりません。また、音声の振幅ヒストグラムの頂点は薄いものです。HF アンプが実際に持つ滑らかな曲線には、32 で十分です。値を変えると、補正は最初からやり直されます。
settings-radio-how-hard-each-block-feedback = フィードバックの各ブロックが補正をどれだけ動かすかです。アンプの曲線は変わらないため、急ぐ必要はありません。中間の値は、数回の交信分のノイズを平均して取り除きます。
settings-radio-keep-correction-curve-learned-clean = 補正をそのままに保ちます。きれいな交信で学習された曲線は保持する価値があり、次の交信までにアンプが変わっていることはありません。
settings-radio-forget-correction-learn-again = 補正を忘れて、もう一度学習します。
settings-radio-directional-coupler-amplifier-s-output = アンプの出力にある方向性結合器が、このチェーンに入ります。送信機は、戻ってきたものと送ろうとしたものを比較し、その差の逆を送ります。そのため、アンプから出るものはまっすぐになります。他の人の交信の相互変調は 20 dB 強く減り、アンプは出力を保ちます。フィードバックが送信と揃うまで補正は 1 倍のままなので、接続されていないカプラーは何もコストをかけません。また、コンバーターにフルスケールを超えるものを要求することは決してできないため、無意味な値を読むフィードバック経路が何かを過駆動することもありません。送信中は、その進み具合がログに出力されます。
settings-radio-transmit-not-armed-so-there = 送信は待機状態ではないため、ここには補正するものがありません。
settings-radio-off-interface-publishes-no-transmit = これがオフの間、インターフェースは送信チャンネルをまったく公開しないため、無線機をキーすることはできません。
settings-radio-transmit-gain = 送信ゲイン
settings-radio-transmit-port = 送信ポート
settings-radio-board-s-two-transmit-sockets = 基板の 2 つの送信ソケットは、同じ回路に入る 2 つのジャックではなく、2 つの異なるマッチングネットワークです。BAND1（TX_1）は 30 MHz から 1.9 GHz を運び、BAND2（TX_2）は 13 cm のポートです。23 cm より低いアマチュアバンドを BAND2 からキーすると、ほとんど何も通さないマッチングネットワークに送信が入ります。そのとき、すべての設定は正しく表示され、駆動は設定したとおりで、出力メーターはゼロのままです。これを自動のままにすると、ダイヤルに従います。
settings-radio-limesdr-transmits-about-100-khz = LimeSDR は、独自のフィルターなしで約 100 kHz から 3.8 GHz まで送信します。ローパスフィルター、適切な LimeRFE のチャンネル、またはダミーロードを使ってください。
settings-radio-limerfe-front-end = LimeRFE フロントエンド
settings-radio-connected = 接続先
settings-radio-limerfe-s-own-micro-usb = 無線機のものではなく、LimeRFE 自身の micro-USB ポートです。9600 baud で、ファームウェアによって固定されています。
settings-radio-receive-connector = 受信コネクター
settings-radio-transmit-connector = 送信コネクター
settings-radio-follow-dial = ダイヤルに従う
settings-radio-switch-filters-match-operating-frequency = RF が現れる前に、使用周波数に合わせてフィルターを切り替えます。1 つのバンド内での同調は、制御リンクに何も送りません。
settings-radio-relays = リレー
settings-radio-receive-attenuator = 受信減衰器
settings-radio-notch-filter = ノッチフィルター
settings-radio-fan = ファン
settings-radio-worth-having-any-sustained-transmitting = 長時間の送信がある場合は、オンにしておく価値があります。
settings-radio-automatic-board-receives-switched-transmit = 自動では基板は受信しており、キーダウンで送信に、キーアップで受信に戻されます。どちらのケーブル配線でも同じです。そのアマチュアのチャンネルには、両側に送受信スイッチを挟んだ 1 つのフィルターがあるため、両方を同時に要求すると、そのスイッチが送信機側に入り、何も聞こえなくなります。30 MHz のチャンネルは J5 からしか届きません。これは両方向に使われる 1 つのコネクターです。その上では、J4 から送信すると、アンプが駆動しているコネクターから受信経路が外れたままになります。
settings-radio-limesdr-support-has-not-been = LimeSDR のサポートは実機ではまだ検証されていません。これは直前のセッションのトレースで、不具合報告用です。
settings-radio-gains-filters-corrections-antenna-sockets = ゲイン、フィルター、補正、アンテナソケット、およびすべての LimeRFE のコントロール（コネクター、バンド、リレー、減衰器、ノッチ、ファン）は即座に適用されます。基板、受信チェーン、サンプルレート、送信の待機状態、LimeRFE の接続は、適用で有効になります。

## Settings window, Radio tab: messages with values

settings-radio-fill-from = { $name } から入力
settings-radio-decimation-auto = 自動（最も近い { $khz } kHz）
settings-radio-decimation-stage-server = 段階 { $stage } — サーバーのレート ÷ { $divisor }
settings-radio-decimation-stage = 段階 { $stage } — レート ÷ { $divisor }
settings-radio-kiwi-receive-only = 受信のみ — これは他の人のアンテナです。パナダプターは、受信機が I/Q として送る約 12 kHz で、ダイヤルに従います。その上のバンド表示は受信機自身のウォーターフォールで、その上を同調すると受信機が同調し直されます。接続中は約 { $rate } kB/s です。再起動せずに切り替えるには、「適用 / 再接続」を押してください。
settings-radio-connected-detail = 接続中：{ $detail }
settings-radio-failed = 失敗：{ $error }
settings-radio-pluto-buffer = 約 { $ms } ms、転送ごとに { $kib } KiB
settings-radio-driver-settings = { $driver } の設定
settings-radio-rx888-coverage = 0〜{ $mhz } MHz をカバー、USB 経由で { $mbytes } MB/s
settings-radio-rx888-digitised = { $mhz } MHz のデジタル化範囲のうち
settings-radio-elad-16-bit = { $khz } kHz（16 ビットサンプル）
settings-radio-elad-pad = { $db } dB パッド挿入
settings-radio-hackrf-rx-preamp = 受信時 { $db } dB プリアンプ
settings-radio-hackrf-tx-preamp = 送信時 { $db } dB プリアンプ
settings-radio-device-not-found = { $device }（見つかりません）
settings-radio-lime-chain = チェーン { $n }（RX{ $n }_*）
settings-radio-lime-chain-tx = チェーン { $n }（RX{ $n }_* / TX{ $n }_*）
settings-radio-lime-second-chain = 基板のもう一方の受信チェーンで、{ $sockets } ソケットにあります。シンセサイザーを共有するため、1 つ目と同じスパンを同じ瞬間に聴きます。そのため、2 本目のアンテナや、自分の送信機のサンプルを運ぶことができます。
settings-radio-tx-blocked = 送信がブロックされています：{ $reason }
settings-radio-rfe-rx-fallback = { $port } で受信中、これらはフィルターなしの広帯域経路に切り替わります：{ $bands }。
settings-radio-rfe-tx-fallback = { $port } で送信中、これらは広帯域経路に切り替わります。バンドアンプもフィルターもありません：{ $bands }。

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = 音量、ゲイン、AGC、スケルチ、ノイズのコントロール、および FM 放送の RDS 表示
topbar-receiver = 受信機
topbar-vfo-b-split-rit-xit = VFO A/B、スプリット、RIT/XIT のオフセット
topbar-tuning = 同調
topbar-diversity-filter-which-way-combines = ダイバーシティフィルターです。2 本のアンテナをどちら向きに合成するか、どれだけ速く適応するか、そしてその位置で保持するかを設定します
topbar-diversity = ダイバーシティ
topbar-second-receiver-s-frequency-mode = 2 つ目の受信機の周波数、モード、フィルター、レベル
topbar-sub-receiver = サブ受信機
topbar-transceiver-s-own-aerial-socket = トランシーバー自身のアンテナソケット、および電源スイッチ
topbar-radio = 無線機
topbar-tune-voice-keyer-drive-mic = 同調、音声キーヤー、駆動とマイクのレベル
topbar-transmit = 送信
topbar-tune = {" "}同調{" "}
topbar-display = 表示
topbar-logbook-spots-awards-memories-scanner = ログブック、スポット、アワード、メモリー、スキャナー、設定、マニュアル
topbar-system = システム
topbar-linked = リンク中
topbar-not-linked = リンクしていません
topbar-bar-trace = バー / トレース
topbar-needle-bar-trace = 針 / バー / トレース
topbar-swap-vfos = VFO を入れ替え
topbar-copy-b = A を B にコピー
topbar-second-receiver-right-ear-tunes = 2 つ目の受信機で、右耳に聞こえます。A/B とは独立して同調します。そのコントロールは SUB モジュールに、そのパスバンドはウォーターフォールに表示されます。
topbar-band-plan = 、バンドプランから
topbar-repeater-shift-following-band-plan = レピーターシフト：バンドプランに従っています。この周波数にはシフトがないため、シンプレックスです
topbar-repeater-shift-transmit-above-below = レピーターシフト：レピーターと交信するために、ダイヤルより上または下で送信します。現在はシンプレックスです
topbar-repeater-shift = レピーターシフト
topbar-auto = 自動
topbar-take-shift-band-plan-dial = ダイヤルが動くとともに、バンドプランからシフトを取得します。レピーターの出力サブバンド内でのみ働きます。それ以外の場所では無線機はシンプレックスのままなので、呼び出しチャンネルはシンプレックスのままです。
topbar-offset = オフセット
topbar-repeater-tone-ctcss-dcs-goes = レピーターのトーン：音声の下で送られる CTCSS/DCS、1750 Hz のバースト、受信トーンスケルチです
topbar-transmit-tone = 送信トーン
topbar-set-here-sent-nfm-sub = ここで設定し、NFM で送信されます。可聴以下の信号はFM チャンネルのものです
topbar-normal = 通常
topbar-invert = 反転
topbar-bit-order-dcs-encoded-here = ここで DCS を符号化するビット順序は、規格から書き写したもので、レピーターで確認されたことはありません。開かない場合は、CTCSS を試してください
topbar-t-1750-hz-burst = 1750 Hz バースト
topbar-every-over = 交信ごと
topbar-open-burst-every-time-transmitter = 送信機がキーされるたびに、バーストで開始します
topbar-send = 送信
topbar-send-burst-now-keying-transmitter = 今すぐバーストを送信します。まだキーされていなければ、その長さの間、送信機をキーします
topbar-nfm-only-burst-fm-repeater = NFM のみ：バーストは FM レピーターのドアオープナーです
topbar-receive-tone-squelch = 受信トーンスケルチ
topbar-require-receive-what-station-transmits = この局が送信しているものを、受信でも要求します
topbar-hide-docked-band-selector-click = ドックされたバンド選択を隠します。もう一度クリックすると戻ります
topbar-show-docked-band-selector = ドックされたバンド選択を表示
topbar-band-mode = バンドとモード
topbar-hide-band-chip-brings-back = 隠す — バンドのチップで戻ります
topbar-dec-off = DEC オフ
topbar-vol = 音量
topbar-counted-steps-not-decibels-what =


    Counted in steps, not decibels: what one step is worth is the receiver's own business, and on an SDRplay it depends on the band as well. Right is still more gain.
topbar-gain = ゲイン
topbar-plutosdr-s-own-agc-setting =


    The PlutoSDR's own AGC is setting this gain, so moving the slider does nothing. Set Settings → Radio → AGC to Manual to control it here.
topbar-agc-hang-time-click-cycle = AGC のホールドタイム。クリックで切り替え：オフ / 遅い / 中 / 速い
topbar-man = 手動
topbar-manual-audio-gain-used-while = 手動の音声ゲイン。AGC がオフの間に使われます。AGC がオフにされたときに保持していたレベルから初期化されます。
topbar-radio-s-own-squelch-sent = 無線機自身のスケルチで、制御リンク経由で送られます。これが音声が実際に通るゲートです。ソフトウェアのものは、リグがすでに通したものに対してさらに閉じるだけです。
topbar-max-gain = 最大ゲイン
topbar-manual-gain = 手動ゲイン
topbar-noise-reduction = ノイズリダクション
topbar-impulse-noise-blanker = インパルスノイズブランカー
topbar-auto-notch-cancel-constant-tone = 自動ノッチ：一定のトーン成分（ヘテロダイン）を消去します
topbar-binaural-audio-not-while-sub = バイノーラル音声。サブ受信機が右耳を使っている間は使えません
topbar-binaural-audio-passband-spread-across = バイノーラル音声：パスバンドを両耳に広げるため、異なる音程の信号は異なる方向から聞こえ、同調している信号は、その間を漂います。クリックでモノラルに戻します
topbar-binaural-cw-spread-passband-across = バイノーラル CW：パスバンドを両耳に広げ、異なる音程の信号が異なる方向から聞こえるようにします。パイルアップは 1 つの混み合った音ではなく、いくつかの場所になり、局に同調すると、その音が移動します。ヘッドホンで最も効果的です
topbar-binaural-audio-spread-passband-across = バイノーラル音声：パスバンドを両耳に広げます。音声では、ノイズは全体の像に広がり、局は中央に留まります。そのため、1 時間聴くには楽になりますが、音声そのものは広がって聞こえます。ヘッドホンで最も効果的です
topbar-mute = ミュート
topbar-record-audio-raw-i-q = 音声、生の I/Q、またはその両方を録音
topbar-wfm-stereo-forced-off-click = WFM ステレオは強制オフです。クリックで自動ステレオ
topbar-wfm-stereo-pilot-locked-click = WFM ステレオ：パイロットがロックしています。クリックで強制モノラル
topbar-wfm-stereo-automatic-no-pilot = WFM ステレオ：自動、この局にはパイロットがありません
topbar-rds-data-arriving-click-see = RDS のデータが届いています。クリックして確認
topbar-rds-no-data-station-click = RDS — この局にはデータがありません。それでもクリックするとデコーダーの診断が見られます
topbar-drm-decoding-click-details = DRM をデコード中です。クリックで詳細
topbar-drm-click-decoder-s-state = DRM。クリックでデコーダーの状態
topbar-hd-radio-decoding-click-details = HD Radio をデコード中です。クリックで詳細
topbar-hd-radio-click-decoder-s = HD Radio。クリックでデコーダーの状態
topbar-presets = プリセット
topbar-width-per-sideband = 側波帯ごとの幅
topbar-width = 幅
topbar-passband-width-hertz-type-drag = パスバンドの幅（Hz）。入力するか、ドラッグしてください。欄をクリックすると、正確な値を入力できます。
topbar-edges = 端
topbar-low-edge-hz-carrier = 下端。搬送波からの Hz
topbar-high-edge-hz-carrier = 上端。搬送波からの Hz
topbar-noise-reduction-voice-off-click = ノイズリダクション（音声）、オフ。クリックしてエンジンを選んでください
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = 聞こえているものを MP3 で録音します。一方のチャンネルは受信機、もう一方は自分の送信です。交信の記録になります。
topbar-mono = モノラル
topbar-mixed-one-channel-click-two = 1 チャンネルにミックス済み。クリックで 2 チャンネル
topbar-two-channels-rx-left-tx = 2 チャンネル：受信は左、送信は右。クリックで 1 つのミックスされたチャンネル
topbar-quick-clip = クイッククリップ
topbar-stop-after = 次の後に停止
topbar-no-stop = 停止しない
topbar-auto-record = 自動録音
topbar-set-squelch-auto-record-needs = スケルチを設定してください。自動録音は、無音を判定するためにそれが必要です
topbar-auto-record-needs-sdr-front = 自動録音には、ソフトウェアのスケルチを持つ SDR のフロントエンドが必要です
topbar-stop-following-squelch = スケルチに従うのをやめる
topbar-waiting-signal = 信号を待っています
topbar-record-spectrum = スペクトラムを録音
topbar-i-q-wav = ● I/Q WAV
topbar-i-q-wav-2 = I/Q WAV
topbar-raw-spectrum-receiver-delivering-stereo = 受信機が送っている生のスペクトラムを、ステレオの 32 ビット浮動小数点 WAV（4 GB を超えると RF64）で保存します。SDR#、SDRuno、HDSDR、SDRangel で開け、sdroxide 自身も --file で再生できます。大きなファイルです。1 サンプルあたり 8 バイトです。
topbar-radio-hands-over-demodulated-audio = この無線機は復調された音声を渡すため、録音する I/Q はありません。
topbar-what-capture-costs-sample-rate = このサンプルレートでの録音のコスト
topbar-engine = エンジン
topbar-no-noise-reduction-decoders-never = ノイズリダクションなし。デコーダーは、どちらにしてもそれを見ることはありませんでした
topbar-strength = 強度
topbar-tone = トーン
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS：この信号には可聴以下のトーンはありません
topbar-tone-squelch = トーンスケルチ
topbar-carrier-squelch-open-any-signal = キャリアスケルチ：どの信号でも開きます
topbar-require-tone-currently-being-received = 現在受信しているトーンを要求します
topbar-ctcss = CTCSS
topbar-any-dcs = 任意の DCS
topbar-open-any-dcs-coded-signal = どの DCS 符号化信号でも開きます。それが運ぶ 104 の符号のどれかは、ここでは確実に読み取れないため、それ以上細かく選ぶものはありません
topbar-which-socket-back-radio-receiving =
    無線機が受信に使っている背面のソケットです。無線機自身の ANT コマンドで、前面パネルの ANT ボタンと同じ設定です。クリックすると次へ進みます。

    The choice is remembered per band, and put back the next time the dial crosses into that band.
topbar-radio-s-separate-receiving-antenna =
    無線機の独立した受信アンテナです。受信経路に入れるか外すかを切り替えます。無線機自身の RX ANT の設定です。メインのソケットのアンテナは、どちらの場合も送信のままです。

    The radio remembers this per band itself, so sdroxide reads it back after every band change rather than putting back what it last saw: clicking here is the only thing that moves it.
topbar-radio-s-own-power-switch =
    無線機自身の電源スイッチです。制御リンク経由で、プログラム内で唯一の本当の電源スイッチです。sdroxide 自身の LINK スイッチ（sdroxide 側を閉じ、無線機は動作したまま）とは異なります。

    For ON to reach anything the radio's control end has to stay awake while it is off: Network Control over the LAN, or a CI-V port still fed from the mains on a set switched off at the front.
topbar-switch-radio = 無線機の電源を入れる
topbar-switch-radio-off-audio-meters = 無線機の電源を切ります。音声とメーターもそれとともに止まります。
topbar-what-second-aerial-click-swap =
    2 本目のアンテナの用途です。クリックで入れ替えます。

    CANCEL subtracts it from the first, in the gain, phase and delay that make a local noise source line up on both: the DSP form of a noise-cancelling phaser. COMBINE adds the two in the phase that reinforces, weighted by how well each hears — diversity reception, which fills in fades.
topbar-hold = ホールド
topbar-stop-filter-moving-reach-moment = フィルターの動きを止めます。ヌルが現れた瞬間にこれに手を伸ばしてください。適応したままのフィルターは、最も大きくなったものに自分を合わせ直します。静かなバンドでは、それがあなたが聴いている局になります。
topbar-restart = 再開
topbar-zero-filter-find-null-again = フィルターをゼロに戻し、ヌルをもう一度見つけます。
topbar-adapt = 適応
topbar-how-fast-filter-chases-slow = フィルターの追従速度です。左では遅くて安定しており、1 秒の何分の一かの間に収束します。右では目に見えて探し回ります。最初は速くしてヌルを見つけ、その後で保持（ホールド）してください。
topbar-where-sub-receiver-listens-shift = サブ受信機が聴いている場所です。ウォーターフォールをシフトクリックするか、サブのパスバンド内をドラッグすると移動します。
topbar-dial = ←ダイヤル
topbar-move-sub-receiver-main-dial = サブ受信機をメインのダイヤルに移動
topbar-dial-2 = ダイヤル→
topbar-move-main-dial-sub-receiver = メインのダイヤルをサブ受信機に移動
topbar-filter = フィルター
topbar-sub-receiver-passband-edges-hz = サブ受信機のパスバンドの端（Hz）
topbar-sub-receiver-level-plays-right = サブ受信機のレベル（右耳で再生されます）
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = 音声キーヤー：保存されたメッセージを録音して送信します
topbar-drive = 駆動
topbar-tune-2 = 同調
topbar-mic = マイク
topbar-cessb = CESSB
topbar-microphone-gain = マイクゲイン
topbar-deviation-how-far-mode-s =
    偏移：このモードのバーストが、それを自分で変調する無線機をどれだけ振らせるかです。FM 送信機は音声レベルを周波数の振れに変換し、それを捕まえる ALC を持ちません。そのため、音声用に設定されたデータ入力にフルスケールを入れると、偏移が過大になります。それは聞き手には完全に普通に聞こえますが、誰のデコードにもなりません。

    Kept per mode, so a deviation set for 1200 baud packet never lands on FT8.
topbar-transmit-audio-how-hard-mode =
    送信音声：このモードが、送信するものを変調する無線機の変調器をどれだけ強く駆動するかです。サウンドカード経由の CAT リグ、FLEX、ネットワークポート経由の Icom などが対象です。リグの ALC がわずかしか動かないところまで下げ、出力は無線機で設定してください。一定の包絡線のデジタルモードで ALC が働くと、スプラッタの原因になります。

    Drive is not this control: on these radios Drive reaches the rig's power register and never touches its audio.

    Kept per mode — FT8, RTTY, PSK and MCW each keep their own.
topbar-tx-audio = 送信音声
topbar-cw-psk-rtty-skimmers-decode = CW / PSK / RTTY のスキマー。バンド全体の信号をデコードし、ウォーターフォールに印を付けます
topbar-skimmers = スキマー
topbar-run-skimmer = このスキマーを実行
topbar-minimum-snr-decoded-signal-needs = デコードされた信号がスポットされるために必要な最小 SNR
topbar-needs-wideband-iq-source = 広帯域 IQ のソースが必要です
topbar-cw-decoder = CW デコーダー
topbar-how-many-signals-model-reads = モデルが一度に読み取る信号の数です。残りは印を保ちますが、テキストは付きません。
topbar-spots = スポット
topbar-save-skimmer-s-spot-list = スキマーのスポット一覧をファイルに保存
topbar-show-hide-full-band-waterfall = パナダプターの上の、全バンドのウォーターフォールのストリップを表示／非表示にします。この受信機が一度に見られるすべてです
topbar-spectrum-waterfall-either-layer-both = スペクトラムとウォーターフォール。どちらか一方、両方、またはどちらもなし。両方が表示されている間は点灯します。
topbar-spectrum = スペクトラム
topbar-show-spectrum = スペクトラムを表示
topbar-draw-spectrum-line-across-top = パナダプターの上部にスペクトラムの線を描きます。オフにすると、ウォーターフォールが全高を使います。
topbar-peak-hold = ピークホールド
topbar-trace-highest-level-each-column = 各列が生の線の上で到達した最も高いレベルをトレースし、それが減衰して戻ります。目を離している間にバンドがどうだったかを示します
topbar-draw-spectrum-receding-surface-instead = スペクトラムを平らな線ではなく、後退する面として描きます。最新のスペクトラムが手前に、その前のものはあなたから遠ざかるように流れます。バンドの直近数秒が風景として見えます。出たり入ったりするキャリアは、ぴくぴく動く線ではなく、尾根になります。グリッドとピークホールドは平らな線に属するため、この表示には描かれません。
topbar-how-quickly-spectrum-line-follows = スペクトラムの線がバンドにどれだけ速く追従するかです。遅いほど、より多くのフレームが互いに平均されます。線が安定し、静止しているほど弱いキャリアも読めるようになります。ウォーターフォールには影響しません。その行は、どちらの場合も毎フレームを受け取ります。
topbar-show-waterfall = ウォーターフォールを表示
topbar-draw-scrolling-waterfall-below-spectrum = スペクトラムの下に、スクロールするウォーターフォールを描きます。オフにすると、スペクトラムの線が全高を使います。
topbar-how-fast-waterfall-scrolls-lines = ウォーターフォールが 1 秒あたり何行でスクロールするか：遅い 5、中 28、速い 56、より速い 112、最速 224。エンジンはこれらを自分で刻むため、最も速い 2 つは同じ行を 2 回描いたものではなく、本当の詳細です。ただし、受信機がそれを供給できる範囲に限ります。1 行は 1 回の変換以上を示すことはできず、狭いフロントエンドでは 1 秒に数十行しか作れません。これらは履歴を消費します。ウォーターフォールは固定の行数を保持するため、中では 73 秒、最速では 9 秒です。
topbar-decode-labels = デコードのラベル
topbar-mark-every-decoded-station-waterfall = デコードされたすべての局に、聞こえた周波数で、そのコールサインのラベルをウォーターフォールに付けます。FT8、FT4 などのスロット式のモードです。良い状況では、読んでいるトレースの上に、1 分間に 2 回、スパン全体に約 30 個が並びます。オフにするとウォーターフォールは単なるバンドになります。その横のデコード一覧には、依然としてすべてのコールサインがあります。
topbar-switch-3d-set = これを設定するには、3D をオンにしてください
topbar-how-3d-spectrum-drawn-only = 3D スペクトラムの描き方です。違うのは形だけです。どちらの描き方も同じスペクトラムを示し、どちらも背後にあるものを隠すため、前にある強い信号は、それが覆うバンドの上に立ちます。
topbar-lines = 線
topbar-one-trace-per-remembered-spectrum = 記憶された各スペクトラムにつき 1 本のトレースで、平らな線と同じ色です。それぞれが後ろのものを隠します。レベルを除いた形で、形こそ欲しいときに選ぶ表示です。描く面は同じで、その上の各頂をなぞるため、中塗りのものより少し多くのコストがかかります。
topbar-solid = 塗りつぶし
topbar-filled-surface-coloured-waterfall-s = ウォーターフォールのパレットで色付けされた塗りつぶしの面です。レベルは高さだけでなく色にも現れるため、パナダプターの 2 つの部分が、強い信号の見え方について一致します。パレットは、設定 › 表示で変更できます。
topbar-how-fast-3d-spectrum-flows = 3D スペクトラムがあなたから遠ざかるように流れる速さで、1 秒あたりの行数です。遅い 6、中 12、速い 24、より速い 48。面は固定の行数の深さがあるため、これは保持する時間の量でもあります。遅いほど記憶が長く、面がゆっくり這うようになります。速いほど記憶は短くなりますが、動きは速くなります。ウォーターフォールは、下にある独自のスクロール速度を保ちます。
topbar-how-many-columns-panadapter-waterfall = パナダプターとそのウォーターフォールを描く列の数です。自動では、このマシンのレンダラーとパナダプターの大きさを読み、運べる最大の数を選びます。4K の画面では 4096 が必要です。各列は、すべてのフレームのバイトなので、リモートの局に接続しているクライアントは、その詳細分をリンクで払います。60 fps の 4096 列は、毎秒約 0.25 MB で、標準の幅の 2 倍です。
topbar-read-machine-screen-take-most = このマシンとこの画面を読み取り、運べる最大のものを取り、それらが変われば追従します
topbar-width-force-right-now-whatever = 上の行が何を要求していても、今まさに有効な幅
topbar-keep-floor-ceiling-set-best = ウォーターフォールのコントラストが最も良くなるように設定された床 / 天井を保持します。バンドの変更後、パンやズームの後、レベルがずれたときには、徐々に元の位置に戻ります。すぐに合わせるにはオンにし、レベルを設定したところに保持するにはオフにしてください。
topbar-keep-tuned-frequency-middle-panadapter = 同調周波数をパナダプターの中央に保持します。端の外で同調したときに絵が 1 スパン分飛ぶのではなく、窓がダイヤルの下をスライドします。すぐ中央に合わせるにはオンにし、好きな場所にパンやズームするにはオフにしてください。
topbar-ceil = 天井{" "}
topbar-fft-size = FFT サイズ
topbar-scroll-waterfall-upwards-newest-row = ウォーターフォールを上にスクロールします。最新の行が下になります（V）
topbar-logbook-all-qsos-digital-manual = ログブック：すべての QSO（デジタル + 手動）
topbar-award-tracking-dxcc-was-waz = アワードの追跡：DXCC / WAS / WAZ / グリッド
topbar-band-conditions-published-forecast-beside = バンドの状況：公表された予報と、各バンドで実際に聞こえたものを並べて表示
topbar-satellite-qo-100-beacon-calibration = 衛星：QO-100 ビーコンの校正が実行中です
topbar-satellite-doppler-tracking-qo-100 = 衛星：ドップラー追跡と QO-100 ビーコンの校正
topbar-ism-band-devices-decoding-now = ISM バンドの機器：このウィンドウが開いているかどうかに関係なく、今デコード中です。ウィンドウ内の DECODING でオフにできます
topbar-ism-band-devices-weather-sensors = ISM バンドの機器：周囲で聞こえる気象センサー、メーター、ホームオートメーション
topbar-public-sdrs-internet-browse-kiwisdr = インターネット上の公開 SDR：KiwiSDR と SpyServer のディレクトリを閲覧し、どれかを無線機として開きます
topbar-winlink-radio-email = Winlink 無線メール
topbar-memory-channels = メモリーチャンネル
topbar-scanner-stopped-signal = スキャナー：信号で停止中
topbar-scanner-running = スキャナー：実行中
topbar-scan-memory-channels-frequency-range = メモリーチャンネルまたは周波数範囲をスキャンします
topbar-grid-tracker-worked-maidenhead-squares = グリッドトラッカー：交信したメイデンヘッドの区画を地図上に表示し、聞こえたものを並べます
topbar-settings-device-gains-antennas-audio = 設定：デバイスのゲイン、アンテナ、音声デバイス
topbar-user-manual-f1 = ユーザーマニュアル（F1）
topbar-range-was-typed-settings-radio = その範囲は、設定 ▸ 無線機 ▸ 受信範囲で入力されたもので、無線機が報告したものではありません。範囲を広げるか、欄を空にして、デバイスが自身について報告するものを使ってください。
topbar-what-device-reports-about-itself = これはデバイスが自身について報告するものです。無線機がこのバンドをカバーしている場合は、設定 ▸ 無線機 ▸ 受信範囲に、その実際の範囲を入力してください。
topbar-band = バンド
topbar-mode = モード

## Top bar: the System box's window chips.

topbar-chip-log = ログ
topbar-chip-awards = アワード
topbar-chip-bands = バンド
topbar-chip-sat = 衛星
topbar-chip-ism = ISM
topbar-chip-mail = メール
topbar-chip-mem = メモリー
topbar-chip-scan = スキャン
topbar-chip-hfdl = HFDL
topbar-chip-grid = グリッド

## Top bar: the mode-defaults chip.

topbar-changed-agc = AGC
topbar-changed-squelch = スケルチ
topbar-changed-auto-notch = 自動ノッチ
topbar-changed-stereo = ステレオ
topbar-changed-binaural = バイノーラル
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    { $mode } の既定値に戻します。既定値から変更された項目：{ $changed }。

    Click to put them back. { $mode }'s own values return, and what you set here is forgotten.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = この無線機には { $baud } baud の設定がありません。代わりに { $default } が使われます
settings-radio-icom-scope-hint =
    無線機自身のスコープの掃引を CI-V リンク経由で送り、パナダプターとして描きます。復調音声の無線機が与えられる、バンドの唯一の画像です。送られる音声は、すでにそのフィルターを通ったものだからです。

    Needs the radio set up for it: CI-V USB Baud Rate to 115200 and CI-V USB Port to "Unlink from {"["}REMOTE]" (both under SET > Connectors > CI-V), and the baud rate above set to match — below { $min_baud } the sweeps do not fit down the link and the scope stays off.

    The sweeps share the radio's internal USB bus with its own sound card. If received audio starts to drop out with the scope on, this box is the first thing to try turning off.
settings-radio-rx-tx-words = { $rx } / TX { $tx }
settings-radio-soapy-native = 以下のハードウェアは sdroxide が直接サポートしています：{ $names }。上でそのインターフェースを選ぶと、独自の設定が表示されます。SoapySDR では表現できないゲイン段、フィルター、ノッチです。SoapySDR モジュールは不要です。
settings-radio-rx888-width-hint = パナダプターが一度に表示する、デジタル化されたスペクトラムの量です。DSP チェーン全体がこの幅で動くため、広くするとそれに比例して CPU を使います。1/2 はウォーターフォールのバンド全体で、相当な量の演算になります。VHF のクロスオーバー（{ $crossover } MHz）より上では、パナダプターはアンテナではなくチューナーの IF を見ています。その IF は 8 MHz 幅で、{ $if_centre } MHz に置かれています。そのため、{ $widest } より広いものは、その上に中央を合わせることができず、余分な幅は信号のそばの無信号のスペクトラムになり、より多くの信号が見えるわけではありません。
settings-radio-sdrplay-serial-missing = シリアル { $serial } は、SDRplay サービスが報告する受信機の中にありません。一覧の受信機から 1 つ選ぶか、この受信機を差し直して、再スキャンを押してください。
settings-radio-hdr-filter-hint =
    HDR 経路の前にあるアナログフィルターです。受信機自身の帯域幅とは別のコントロールです。各設定は固定の中心の組で作られ、ほかの場所では何もできません。この設定は { $centres } で作られています。即座に適用されます。

    The path itself does not work yet — see the switch above — so this chooses a filter for a mode that is currently silent.
settings-radio-rspduo-only = 2 つ目のチューナーを持つのは RSPduo だけで、ここで選んだ受信機は { $model } なので、これは何もしません。
settings-radio-lime-tx-gain-low = 送信ゲインは { $db } dB で、その範囲の最低値です。基板からは数マイクロワットしか出ないため、その下流が何であれ、電力計では何も表示されません。武装した送信機に驚かされないための既定値です。キーする前に上げてください。

## Settings window, General: SWR

settings-general-swr-tune-limit = （同調：{ $limit }:1）

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Click to cycle meter face: { $faces }
topbar-step-down = 下げる { $step }
topbar-step-up = 上げる { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = レピーターシフト { $shift }。{ $rx } で受信、{ $tx } MHz で送信{ $auto }
topbar-tone-and-burst = 音声の下で { $tone } を送信し、交信の始めに毎回 { $ms } ms の 1750 Hz バーストを送ります
topbar-tone-under-voice = 音声の下で { $tone } を送信しています
topbar-burst-every-over = 交信の始めに毎回、{ $ms } ms の 1750 Hz バーストを送ります
topbar-match-tx = 送信に合わせる（{ $tone }）
topbar-decimation-on = フロントエンドの間引き：無線機が送信している { $msps } Msps のうち中央の 1/{ $n }（{ $span }）を受信機に渡し、残りは受信機に届く前に捨てられます。
topbar-decimation-off = フロントエンドの間引きはオフです。受信機は、この無線機が送信する { $span } 全体を見ています。
topbar-decimation-hint =
    { $what }

    A narrower span means finer waterfall resolution, a quieter noise floor (3 dB per halving) and less CPU — at the cost of the band either side of it. The dial still tunes anywhere; the radio moves its LO to follow.

    Click to cycle: off / 2 / 4 … { $max }.
topbar-rx-gain-hint = フロントエンドの RX ゲイン（{ $stage }）。大きすぎると受信機の ADC がクリップし、バンド全体に偽の信号が広がります。小さすぎると受信機が聞こえなくなります。
topbar-rx-gain-stages =
    {""}

    This rig has { $n } RX gain stages — the rest are in Settings → Device.
topbar-passband-now =
    {""}

    The passband is at { $dbfs } dBFS now.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    受信パスバンド内で、これより低い電力の音声をゲートします。単位は dBFS です。左はオープンです。{ $level } ノイズより上、信号より下に設定してください。

    A stream that arrives with the radio's own AGC already in it — an Icom's 12 kHz IF, for one — sits far higher on this scale than an SDR's raw baseband does, which is why the rail reaches full scale.

    Click to cycle the strength: Low / Med / High / Off. The engine is in the rows below.
topbar-recording-both = { $audio } と { $iq } を録音中
topbar-recording-audio = { $audio } に音声を録音中
topbar-recording-iq = { $iq } に I/Q を録音中
topbar-auto-record-armed = 自動録音が待機中：送信ごとに 1 ファイルで、{ $secs } 秒の無音の後に閉じます
topbar-rds-hover = RDS：{ $ps }。局のデータはクリックで表示
topbar-drm-hover = DRM：{ $service }。放送の詳細はクリックで表示
topbar-drm-no-codec = DRM：{ $summary } — ロックされていますが、その { $codec } 音声はここではデコードできません。不足しているものはクリックで表示
topbar-drm-state = DRM：{ $summary } — デコーダーの状態はクリックで表示
topbar-hd-hover = HD Radio：{ $station }。放送の詳細はクリックで表示
topbar-hd-state = HD Radio：{ $summary } — デコーダーの状態はクリックで表示
topbar-binaural-preset = 各耳に { $preset } — { $lo } … { $hi } Hz、両方の側波帯
topbar-nr-hover =
    ノイズリダクション：{ $strength } — { $engine }

    Click to change engine or strength
topbar-clipping = { $span } のクリップ中です。もう一度押すと、最初からやり直します
topbar-record-clip = { $span } を録音して停止します。受信報告に添付するクリップです
topbar-starting-clip = { $span } のクリップを開始中…
topbar-stops-in = { $time } 後に停止
topbar-auto-record-hint = 送信ごとに、それぞれのファイルへ録音し、{ $secs } 秒の無音の後に閉じます
topbar-recording-silence = 録音中 · { $secs } 秒の無音（{ $hold } に閉じます）
topbar-tone-open = { $tone } を受信しています。これはトーンスケルチで、音声は開いています
topbar-tone-closed = { $heard } を受信していますが、トーンスケルチは { $wanted } を求めています。音声は閉じたままです
topbar-tone-heard = CTCSS/DCS { $tone } を受信中
topbar-tone-nothing = トーンスケルチ { $tone }：それに一致するものは受信されていません
topbar-tone-use = { $tone } を使う
topbar-sockets = ソケット：{ $sockets }
topbar-voice-keyer-tx = { $slot } を送信中です。クリックで音声キーヤーを開きます
topbar-drive-uncal-gen = この周波数は、どのアマチュアバンドにもないため校正されていません。そのため、設定はそのまま送信機に届きます。
topbar-drive-uncal-band = { $band } は校正されていないため、設定はそのまま送信機に届きます。
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = この周波数は、どのアマチュアバンドにもないもので、{ $trim } dB に校正されています。そのため、同じ設定で、校正されていないバンドより、ここでは { $diff } dB { $more ->
        [less] 小さく
       *[more] 大きく
    } 送信されます。
topbar-drive-cal-band = { $band } は { $trim } dB に校正されているため、同じ設定で、校正されていないバンドより、ここでは { $diff } dB { $more ->
        [less] 小さく
       *[more] 大きく
    } 送信されます。
topbar-drive-hover =
    送信機をどれだけ強く駆動するかです。sdroxide 自身が変調する無線機では、変調されたサンプルを拡大縮小します。独自の出力制御を持つリグでは、リグに要求する定格出力の割合です。

    { $trim }

    Settings → Radio → Transmit drive by band.
topbar-cessb-hover =
    制御されたエンベロープの SSB：同じピークでも平均出力が増え、スプラッタはありません。相手側では見かけの音量が 3〜4 dB 増します。音声をプロセッサーにどれだけ駆動するかのデシベル値です。0 はオフ、6 は妥当な最初の試みです。駆動には影響しません。

    Now: { $now }
topbar-waterfall-rate = 1 秒あたり { $rows } 行 — バンドの { $seconds } 秒分
topbar-detail-auto-columns = { $n } 列。ここで自動が選ぶであろうもの
topbar-n-columns = { $n } 列
topbar-fft-bins-whole = ストリーム全体で { $n } ビン — 列ごとに 1 つ、またはそれ以上に細かく
topbar-fft-bins-pooled = { $n } ビンをパナダプターの { $cols } 列にまとめます。信号は鮮明になりますが、数が増えるわけではありません
topbar-sat-locked = 衛星：{ $name } にロック中
topbar-filter-edges = 搬送波から { $lo } … { $hi } Hz
topbar-filter-isb = 受信フィルター：各耳で { $width } Hz（{ $edges }）。ISB の 2 つの側波帯は別々の送信のため、幅は片方の幅です。
topbar-filter-width = 受信フィルター：幅 { $width } Hz（{ $edges }）。
topbar-filter-hint =
    { $what }

    Click for the width, the two edges and this mode's presets, as numbers — the panadapter's grips place a passband by eye, this is where an exact figure is typed.
topbar-band-outside-range =
    { $band } は、この無線機が受信する範囲（{ $ranges } MHz）の外です。

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band }：{ $verdict }（{ $time ->
        [day] 昼
       *[night] 夜
    }）。HAMQSL.com が太陽指数から予測したもので、あなた自身の経路の測定値ではありません。

## Top bar: short words and captions

topbar-squelch-open = 開
topbar-off = オフ
topbar-recording = 録音中
topbar-sql = SQL
topbar-stations = 局
topbar-caption-reaction = 反応
topbar-caption-scroll = スクロール
topbar-caption-surface = 面
topbar-caption-flow = 流れ
topbar-caption-detail = 詳細
topbar-floor = 床

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = 航空機
pane-chart = チャート
pane-chat = チャット
pane-decodes = デコード
pane-files = ファイル
pane-heard = 聞こえた局
pane-image = 画像
pane-map = 地図
pane-messages = メッセージ
pane-monitor = モニター
pane-panel = パネル
pane-qso = QSO
pane-reading = 読み取り
pane-receive = 受信
pane-saved = 保存済み
pane-send = 送信
pane-spots = スポット
pane-stations = 局
pane-status = 状態
pane-terminal = ターミナル
pane-text = テキスト
pane-traffic = トラフィック
pane-vessels = 船舶
pane-wfall = WFALL

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}保存{" "}
panels-nothing-decoded-save = 保存するデコードされたものはありません
panels-clear-rx = {" "}RX をクリア{" "}
panels-empty-receive-window-nothing-air = 受信ウィンドウを空にします。電波に出ているものは何も止まりません。
panels-nothing-received-clear = クリアする受信内容はありません
panels-save-what-panel-has-decoded = このパネルがデコードした内容をファイルに保存します
panels-prop = 伝搬
panels-shade-map-where-signals-are = 各バンドで、実際に信号が届いている場所によって地図に陰影を付けます。各受信は、遠方の局ではなく、その経路の中点、つまりそれを曲げた電離層の部分に置かれます。そのため、これはアマチュア無線家がどこにいるかの地図ではなく、空の地図です。このステーションが参加した経路から作られ、設定 → スポットで Reverse Beacon Network をオンにしている場合は、世界のスキマーが聞いているものも加わります。この無線機がいないバンドもカバーします。
panels-night = 夜
panels-shade-where-sun-down-twilight = 太陽が沈んでいる場所と、その間の薄明に陰影を付け、地図上に灰色の線が見えるようにします。低い周波数のバンドは夜側で遠くまで届き、高い周波数のバンドは近くで閉じます。その境界線そのものが DX のある場所です。
panels-all-bands = 全バンド
panels-every-band-once-one-hue = すべてのバンドを、それぞれ 1 色ずつ同時に。全体の状況です。
panels-one-band = 1 バンド
panels-one-band-cold-hot-blue = 1 つのバンドを冷たいものから熱いものへ：青、緑、黄、赤。
panels-frequency-you-are-actually-working = 実際に交信している周波数です。ダイヤルと、その横のトーンのオフセットを足したものです。これがログに記録される値です。ダイヤルだけでは、その分だけ低くなります。
panels-send-return = 戻ったら送信
panels-freq = ⇵ 周波数
panels-nothing-agreed-mode-save-your = このモードについて合意されたものはありません。下に自分のものを保存してください。
panels-forget-saved-frequency = この保存された周波数を削除
panels-dial-already-one-your-saved = このダイヤルは、すでに保存された周波数の 1 つです。
panels-dial-already-agreed-list-mode = このダイヤルは、このモードの合意された一覧にすでにあります。
panels-decode-squelch-raise-stop-decoding = デコードのスケルチ。ノイズをデコードしないように上げてください
panels-message-repeats-long-transmit-held = 送信を押している間、メッセージは繰り返されます。流星は、その通過中の部分を捕まえます。
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}送信オン{"  "}
panels-tx-2 = {"   "}送信{"   "}
panels-call-cq = {" "}CQ を呼ぶ{" "}
panels-nocall = NOCALL

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = 聞き取れた音声が少なすぎます。数え切るまで、普通に話して、もう一度試してください。
txaudio-tx-audio = TX 音声
txaudio-mode-not-voice-mode-so = このモードは音声モードではないため、送信するものはどれもチェーンを通りません。ここで行った設定は SSB、AM、FM で有効になります。
txaudio-bypass = バイパス
txaudio-switch-every-stage-out-once = すべての段を一度にオフにします。ノイズ除去も含むため、マイクは処理されないまま送信されます。もう一度クリックすると、オンだった段がちょうど元に戻ります。チェーンが何をしているかを比べる最も手早い方法です。
txaudio-denoise = ノイズ除去
txaudio-rnnoise-microphone-ahead-every-other = マイクに対して、他のすべての段より前に RNNoise を適用します。そのため、ゲートやコンプレッサーが部屋のノイズを持ち上げる前に、それが除去されます。約 10 ms の遅延が加わります。
txaudio-digital-voice = デジタル音声
txaudio-also-run-freedv-rade-digital = FreeDV / RADE のデジタル音声もチェーンに通します。既定ではオフです。ボコーダーは自然な音声で学習されているため、強く処理された音声からはデコードが悪くなる場合があります。頼る前に、電波に出して試してください。TCI の音声とデジタルモードは、決してチェーンを通りません。
txaudio-test = テスト
txaudio-rec = {" "}⏺ 録音{" "}
txaudio-play = {" "}▶ 再生{" "}
txaudio-play-test-phrase-through-chain = 今の設定でチェーンを通してテストフレーズを、スピーカーで再生します。再生中に調整すると、変化が聞こえます。再生中は受信音声に置き換わります。
txaudio-recording = 録音中…
txaudio-no-phrase-yet = まだフレーズがありません
txaudio-live = ライブ
txaudio-live-2 = {" "}🎧 ライブ{" "}
txaudio-hear-your-microphone-through-chain = 話している間、送信機をキーせずに、チェーンを通したマイクの音を聞きます。ヘッドホンを使ってください。スピーカーだとハウリングします。自分の声が数十ミリ秒遅れて聞こえるため、話しにくくなります。レベルはメーターで設定し、音は 録音 / 再生 で判断してください。実行中は受信音声に置き換わります。PTT を押すと停止します。
txaudio-meters = {" "}メーター{" "}
txaudio-run-your-microphone-through-chain = メーターだけのために、マイクをチェーンに通します。スピーカーには何も出ないため、話し合わせる遅延もなく、ハウリングもありません。マイクと入力のゲイン、ゲートのしきい値、コンプレッサーがどれだけ働くかを設定する方法です。受信音声は続きます。PTT を押すと停止します。
txaudio-monitoring-live-use-headphones-speakers = ライブでモニター中です。ヘッドホンを使ってください。さもないと、スピーカーの音がマイクにハウリングします。
txaudio-voice-modes-only-tci-clients = 音声モードのみです。TCI クライアントとデジタルモードはチェーンを迂回します。使用中は、無線機自身の TX EQ とプロセッサーをフラットにしてください。
txaudio-auto-set-stopped-microphone-was = 自動設定が停止しました。マイクが取られました。
txaudio-microphone-may-clipping-before-gets = {" "}マイクがここに届く前にクリップしている可能性があります。耳障りに聞こえる場合は、マイク自身のゲインを下げてください。
txaudio-level = レベル
txaudio-mic-control-same-one-transmit = マイクのコントロールです。送信バーのものと同じで、すべてのプロファイルで共有されます。中央で 0 dB、最大で +6 dB です。
txaudio-mic = マイク
txaudio-gain-top-mic-microphone-stored = このマイクのマイクの上に加えるゲインで、プロファイルとともに保存されます。プリセットのしきい値は、IN メーターで約 -10 dBFS にピークを持つ声を前提としています。自動設定はそこに合わせます。バイパスの下でも残り、デジタル音声にも適用されます。これは処理ではなくレベルだからです。
txaudio-input = 入力
txaudio-auto-set = 自動設定
txaudio-auto-set-cancelled = 自動設定はキャンセルされました。
txaudio-talk-now-you-would-air = 電波に出す時と同じように、今話してください…
txaudio-db = -∞ dB
txaudio-chain = チェーン
txaudio-order-audio-meets-them = 音声が通る順序
txaudio-switch-stage = この段をオンにする
txaudio-earlier-chain = チェーンの前の方
txaudio-later-chain = チェーンの後の方
txaudio-output-limiter = 出力リミッター
txaudio-output = 出力
txaudio-trim-limiter-always-last = トリムとリミッター — 常に最後
txaudio-profiles = プロファイル…
txaudio-gate-expander = ゲート / エキスパンダー
txaudio-pushes-room-noise-fan-rumble = 休止中の部屋のノイズ、ファンの低音、ヒスを押し下げます。しきい値は、ノイズフロアのすぐ上で、最も小さな音節の下に設定してください。
txaudio-gate = ゲート
txaudio-hard-gate-10-1-down = ハードゲート：10:1、-40 dB まで下げます
txaudio-expander = エキスパンダー
txaudio-gentle-expander-2-1-never = やわらかいエキスパンダー：2:1、-15 dB を超えて下げることはありません。部屋が無音になることはありません
txaudio-reset = リセット
txaudio-back-defaults = 既定値に戻す
txaudio-threshold = しきい値
txaudio-below-gate-starts-close = これより下で、ゲートが閉じ始めます。
txaudio-return = 戻り
txaudio-hysteresis-once-open-closes-only = ヒステリシス：一度開くと、しきい値よりこの分だけ下がったときに閉じるため、しきい値の付近にいる声でゲートがパタパタしません。
txaudio-hold = ホールド
txaudio-how-long-stays-open-after = 声が下がった後、どれだけ開いたままでいるか。語尾が切れないようにします。
txaudio-release = リリース
txaudio-how-fast-closes-once-hold = ホールドが終わってから、どれだけ速く閉じるか。ゲートがパンプするなら長くしてください。
txaudio-floor = 下限
txaudio-most-ever-turns-pauses-down = 休止を最大でどれだけ下げるか。
txaudio-ratio = 比率
txaudio-how-steeply-closes-below-threshold = しきい値より下で、どれだけ急に閉じるか。
txaudio-look-ahead = 先読み
txaudio-opens-much-before-word-starts = 語が始まるこれだけ前に開くため、最初の子音が切れません。送信するすべてに同じ遅延が加わります。
txaudio-high-pass-low-shelf-two = ハイパス、ローシェルフ、2 つのピーク、ハイシェルフ、ローパス。曲線上の点をドラッグするか、下で設定してください。ローパスは、送信フィルターの幅に合わせてください。
txaudio-flat = フラット
txaudio-every-band-back-0-db = すべてのバンドを 0 dB に戻し、両端をオフにします
txaudio-freq = 周波数
txaudio-gain = ゲイン
txaudio-q-slope = Q / 傾き
txaudio-high-pass = ハイパス
txaudio-removes-handling-noise-breath-pops = 角の周波数より下の、取り扱いノイズ、息のポップ音、ハムを除去します。
txaudio-low-shelf = ローシェルフ
txaudio-body-mud-cut-little-here = 厚み、またはこもり。箱っぽく聞こえる声では、ここを少し削ってください。
txaudio-peak-1 = ピーク 1
txaudio-bump-dip-voice-band-often = 音声帯域の盛り上がりまたは谷です。多くは 300〜500 Hz 付近の削りです。
txaudio-peak-2 = ピーク 2
txaudio-presence-lift-around-2-3 = 存在感：2〜3 kHz 付近の持ち上げは、SSB で抜けてくる音になります。
txaudio-high-shelf = ハイシェルフ
txaudio-brightness = 明るさ。
txaudio-low-pass = ローパス
txaudio-stops-energy-above-transmit-filter = 送信フィルターより上のエネルギーが、変調器で消費されないようにします。
txaudio-t-24-db-oct = 24 dB/oct
txaudio-t-12-db-oct = 12 dB/oct
txaudio-click-steeper-gentler-slope = クリックで、より急またはより緩やかな傾きに
txaudio-de-esser = ディエッサー
txaudio-turns-down-only-sibilant-band = 「s」と「sh」がしきい値を超えたときだけ、歯擦音の帯域を下げるため、声の残りはレベルを保ちます。「sister silver cellophane」と言いながら周波数をスイープし、ヒスが最も大きくなる位置に合わせて周波数を見つけてください。
txaudio-frequency = 周波数
txaudio-centre-sibilant-band = 歯擦音帯域の中心。
txaudio-width-band-higher-narrower = 帯域の幅。高いほど狭くなります。
txaudio-band-level-where-reduction-starts = 削減が始まる帯域のレベル。低すぎると常に作動し、声がこもります。
txaudio-amount = 量
txaudio-most-band-ever-turned-down = 帯域が最大でどれだけ下げられるか。-6 dB 程度から始めてください。
txaudio-attack = アタック
txaudio-how-fast-reacts = どれだけ速く反応するか。
txaudio-how-fast-lets-go = どれだけ速く解放するか。
txaudio-compressor = コンプレッサー
txaudio-brings-quiet-syllables-up-holds = 小さな音節を持ち上げ、大きな音節を抑えます。ピークで 3〜6 dB の削減を目指してください。10 dB を超えると明らかなコンプレッションになり、コンテストに向いています。
txaudio-level-above-which-starts-compress = これを超えるレベルで圧縮が始まります。
txaudio-how-hard-compresses-above-threshold = しきい値を超えて、どれだけ強く圧縮するか。
txaudio-how-fast-reacts-peak-10 = ピークにどれだけ速く反応するか。10〜25 ms で子音が際立ちます。
txaudio-how-fast-recovers = どれだけ速く回復するか。
txaudio-knee = ニー
txaudio-soft-knee-width-0-hard = ソフトニーの幅。0 はハードニーです。
txaudio-makeup = メイクアップ
txaudio-gain-after-compression-bring-level = 圧縮の後のゲインで、レベルを戻します。
txaudio-drive = 駆動
txaudio-gain-ahead-compressor-so-more = コンプレッサーより前のゲインで、声のより多くがしきい値を超えます。平均電力が上がります。出力リミッターと組み合わせてください。
txaudio-phase = 位相
txaudio-all-pass-phase-rotator-ahead = 検波器より前の全域通過の位相ローテーターです。声が持つ偏ったピークを均し、同じピークレベルでもより多くの平均電力を運べるようにします。4 は一般的な放送の設定で、0 はオフです。
txaudio-last-stage-trim-look-ahead = 最後の段：トリムと、先読みリミッターです。ハードクリップがもたらすスプラッタなしで、すべてのピークを天井以下に保ちます。既定ではオフなので、チェーンをオンにしても、すでに設定した駆動レベルは変わりません。
txaudio-limiter = リミッター
txaudio-on = オン
txaudio-ceiling = 天井
txaudio-no-peak-leaves-chain-above = これを超えるピークは、チェーンから出ません。
txaudio-trim = トリム
txaudio-output-gain-ahead-limiter = リミッターより前の出力ゲインです。
txaudio-off = {"  "}オフ
txaudio-meters-move-while-you-transmit = 音声を送信している間、テストフレーズを再生している間、またはライブかメーターを実行している間、メーターが動きます。
txaudio-profiles-2 = プロファイル
txaudio-profile-whole-chain-every-stage = プロファイルはチェーン全体です。すべての段の設定、どれがオンか、その順序、そしてあなたのマイクの入力ゲインを含みます。読み込むとチェーンが置き換わります。マイクのコントロールと駆動はそのままです。
txaudio-name = 名前
txaudio-save = 保存
txaudio-store-chain-now-under-name = 今のチェーンを、この名前で保存します。同じ名前のプロファイルは置き換えられます
txaudio-import = 読み込み…
txaudio-read-profile-json-file = .json ファイルからプロファイルを読み込みます
txaudio-tx-audio-profile = TX 音声のプロファイル
txaudio-no-profiles-yet-save-stores = まだプロファイルはありません。保存は、今設定したチェーンを保存します。
txaudio-load = 読み込み
txaudio-export = 書き出し…
txaudio-delete = 削除
txaudio-starting-points = 出発点
txaudio-set-your-level-auto-set = まず自動設定でレベルを設定してください。プリセットのしきい値はそれを前提としています。その後、読み込まれたままの状態で送信するのではなく、ここから調整してください。フレーズを録音し、変更するたびに聴いてください。プリセットは入力ゲインを保持します。
txaudio-profile-loaded = プロファイルを読み込みました。

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = 伝搬
solar-where-signals-are-actually-getting =
    信号が実際に届いている場所を、バンドごとに示します。このステーションが参加するすべての受信（WSPR、FT8/FT4、JS8、ログブック）は、遠方の局ではなく、その経路の中点、つまりそれを曲げた電離層の部分に置かれます。そのため、これはアマチュア無線家がどこにいるかの地図ではなく、空の地図です。

    It can only show paths this station has been one end of. Oceans light up because the midpoints of long paths fall there; Antarctica stays dark because nobody is transmitting from it. Off by default — it paints the whole planet.
solar-orbits = 軌道
solar-orbital-paths = 軌道の経路
solar-clouds = 雲
solar-cloud-cover-noaa-s-global = NOAA の全球静止衛星モザイクによる雲量です。表面に貼られた画像ではなく、空気の厚みとして描かれます。赤外線が各雲頂の冷たさを示し、それによってどれだけの高さにあるかがわかります。雷雨は、それが作る塔の内部で光ります。
solar-planets = 惑星
solar-other-seven-planets-eighteen-their = 他の 7 つの惑星、その 18 個の衛星、そして土星と天王星の環です。惑星の位置は JPL のケプラー軌道要素から、衛星は JPL Horizons に合わせた円軌道から得られます。
solar-coronal-mass-ejection-trajectory-cones = コロナ質量放出の軌道円錐
solar-sun-obs = 太陽観測
solar-solar-observations-sun-s-disk = 太陽の円盤上の太陽観測：黒点の活動領域と、フレアの発生源
solar-labels = ラベル
solar-body-region-labels = 天体と領域のラベル
solar-small-bodies = 小天体
solar-names-asteroids-comets-they-are = 小惑星と彗星に名前を付けます。どちらにしても描かれます。これは、35 の仮符号を一緒に描くかどうかだけです。どのように設定されていても、カメラが捉えている天体と、検索ボックスで一致したものには、名前が付いたままです。
solar-decoded-ft8-ft4-stations-path = デコードされた FT8/FT4 の局と、交信している局への経路
solar-sats = 衛星
solar-amateur-radio-satellites-their-orbits = アマチュア無線衛星、その軌道、そしてあなたの QTH からの仰角
solar-aurora = オーロラ
solar-auroral-oval-noaa-s-ovation = NOAA の OVATION モデルによるオーロラ帯です。実際の高度での発光の殻として描かれ、10 % の等値線の赤道側の縁が表面に示されています
solar-awards = アワード
solar-award-coverage-every-dxcc-entity = アワードの達成状況：すべての DXCC エンティティを、ログブックにまだ不足しているものによって塗ります。まだ交信していないエンティティはオレンジ、交信したが未確認の場合は琥珀色、QSL が返ってきた場合は暗い緑です。アワードウィンドウのバンドフィルターに従います。
solar-view = 表示
solar-which-body-camera-orbits-animated = カメラがどの天体の周りを回るか、そしてアニメーションのツアー
solar-layers = レイヤー
solar-what-drawn-orbits-clouds-cmes = 描かれるもの：軌道、雲、CME、ラベル、QSO、オーロラ
solar-which-sdo-channel-wraps-sun = 太陽を包む SDO のチャンネル、およびその古さ
solar-weather = 天気
solar-aurora-propagation-numbers-which-bands = オーロラ、伝搬の数値、どのバンドが開いているか
solar-space-weather = 宇宙天気
solar-nothing-measured-yet = まだ何も測定されていません
solar-scale = 縮尺
solar-body-moon-orbit-exaggeration = 天体と月の軌道の誇張
solar-time = 時間
solar-scrub-whole-scene-forward-back = シーン全体を前後にスクラブします
solar-activity = 活動
solar-replay-last-hour-decodes-globe = 地球儀上で、直前 1 時間のデコードを再生します
solar-what-propagation-heat-map-showing = 伝搬のヒートマップが示しているもの
solar-propagation = 伝搬
solar-auto = ▶ 自動
solar-fly-spline-through-set-framed = 枠で囲まれた視点の一連を、スプラインで飛び抜けます。マウスの操作はどれでもキャンセルします。
solar-moons-ride-circular-orbits-fitted = 衛星は JPL Horizons に合わせた円軌道に乗ります。実際の位置から 1〜2 度以内で、ミランダは最大 6 個です。その軌道面は、円では追えないほど速く振れるためです。
solar-fetch-everything-again-now = 今すぐすべてを再取得
solar-all-sats = すべての衛星
solar-show-every-satellite-element-set = 軌道要素のセットにあるすべての衛星を表示します。人気のあるものだけではありません。軌道のリングはキュレーションされた少数に留まります。一度に 90 個は読めないためです。
solar-starting = 開始中…
solar-body = 天体{" "}
solar-moon-orbit = 月の軌道{" "}
solar-stretch-earth-moon-distance-so = 地球から月までの距離を引き伸ばし、ペアが離れて見えるようにします
solar-sun = 太陽{" "}
solar-sun-radius-exaggeration-leave-1 = 太陽の半径の誇張です。CME の幾何を読みやすく保つため、1× のままにしてください。膨らんだ太陽は、すべての円錐の根元を飲み込みます。
solar-show = 表示
solar-paint-globe-what-getting-through = 届いているものによって地球儀を塗る
solar-paints-globe-what-getting-through = 各バンドで、届いているものによって地球儀を塗ります。各受信は、遠方の局ではなく、その経路の中点、つまり電離層が働いた場所に置かれます。このステーションが聞いたものから作られ、Reverse Beacon Network をオンにした場合は、世界のスキマーも加わります。
solar-display = 表示
solar-all-bands = 全バンド
solar-every-band-once-one-hue = すべてのバンドを、それぞれ 1 色ずつ同時に。2 つが重なる場所では混ざります。「20 m は開いているか」ではなく、「状況はどうか」を見るための表示です。
solar-one-band = 1 バンド
solar-one-band-cold-hot-blue = 1 つのバンドを冷たいものから熱いものへ：青、緑、黄、赤。
solar-band = バンド
solar-nothing-heard-yet = まだ何も聞こえていません
solar-bands = バンド
solar-sources = ソース
solar-contacts-logbook-path-was-open = ログブックの交信です。経路は開いていましたが、まともな信号レポートはありません。RST は SNR ではないため、これらはセルの混み具合には数えられますが、その余裕には決して数えられません。
solar-stations-reported-hearing-one-downloaded = この局を聞いたと報告した局で、WSPRnet からダウンロードされたものです。送信しているビーコンが受け取る唯一のフィードバックです。
solar-reverse-beacon-network-skimmers-what = Reverse Beacon Network のスキマーです。それ以外のみんなが、すべてのバンドで聞いているものです。この無線機がいないバンドも含みます。他より粗くなります。RBN の行にはロケーターがないため、両端はそれぞれの国の中心に置かれます。小さな国では近く、大きな国では大きくずれます。設定 → スポットでオンにしてください。
solar-decodes-station-made-each-measured = このステーションが作ったデコードです。それぞれは、それ自身のモードの下限に対して測定されるため、WSPR とFT4 の報告は比較できます。
solar-memory = 記憶
solar-half-life = 半減期{" "}
solar-how-long-reception-takes-count = 受信が半分の重みで数えられるまでの時間。電離層自身の記憶は短く、2 時間前の開きが、2 分前のものと議論すべきではありません。
solar-live = ライブ
solar-follow-band-happens = 起きているとおりにバンドを追います
solar-replay = ⏸ 再生
solar-replay-2 = ▶ 再生
solar-replay-last-hour-decodes-over = 直前 1 時間のデコードを、何度も繰り返し再生します
solar-min-ago = {" "}分前
solar-where-last-hour-globe-showing = 直前 1 時間のどこを地球儀が示しているか
solar-trail = 軌跡{" "}
solar-how-long-decode-s-arc = デコードの弧が、先頭の後ろに地球儀上にどれだけ残るか
solar-speed = 速度{" "}
solar-how-much-faster-than-real = 再生が実時間より何倍速いか
solar-qso-layer-off = QSO レイヤーはオフです
solar-no-decodes-yet = まだデコードはありません
solar-qth-not-set-enter-your = QTH が設定されていません。設定でグリッドを入力してください
solar-mode = モード
solar-path = 経路
solar-sent = 送信
solar-rcvd = 受信
solar-grid = グリッド
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = 静止軌道です。常にあなたの地平線の上にあります。
solar-no-passes-next-48-hours = あなたの QTH から、今後 48 時間にパスはありません。
solar-start = 開始
solar-max-el = 最大仰角
solar-aos-los-are-azimuths-horizon = AOS/LOS は地平線での方位角です。時刻は UTC です。
solar-locked-unlock = {" "}● ロック中 — 解除{" "}
solar-release-satellite-lock = 衛星のロックを解除
solar-lock = {" "}ロック{" "}
solar-track-satellite-main-window-doppler = この衛星をメインウィンドウで追跡します。ドップラー補正された RX と TX、トランスポンダーにマップされたアップリンク、ローテーターの操作
solar-no-frequencies-file-one-add = この衛星の周波数は登録されていません。設定 ▸ TLE で追加してください。
solar-frequencies-yours = 周波数 · 自分のもの
solar-frequencies = 周波数
solar-link = リンク
solar-downlink-mhz = ダウンリンク（MHz）
solar-uplink-mhz = アップリンク（MHz）
solar-doppler-shifts-these-few-khz = ドップラーシフトは、LEO のパスの間にこれらを数 kHz ずらします。
solar-satellite-planet-comet = 衛星、惑星、または彗星
solar-planet-asteroid-comet = 惑星、小惑星、または彗星
solar-clear-search = 検索をクリア
solar-no-match = 一致なし
solar-no-sounder = サウンダーなし
solar-heard = 聞こえた ≥
solar-kp = Kp / A
solar-x-ray = X 線
solar-observed-above-sounder-better-than = サウンダーの上で観測 — モデルより精度が高い
solar-power-n-s = 電力 N/S
solar-edge-n-s = 縁 N/S
solar-kp-peak-24-h = Kp の最大値（24 時間）
solar-observed-forecast = 観測 | 予報
solar-bands-open = 開いているバンド
solar-dxcc-coverage = DXCC の達成状況
solar-ir-vis = IR+VIS
solar-ir-only = IR のみ
solar-no-deep-convection = 深い対流なし
solar-t-1-storm = 1 件の嵐
solar-glancing = {" "}· かすめる
solar-direction-estimated = {" "}· 方向は推定

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = APRS の設定
setup-packet-setup = パケットの設定
setup-ft8-ft4-ft2-setup = FT8 / FT4 / FT2 の設定
setup-my-callsign = 自分のコールサイン
setup-my-grid = 自分のグリッド
setup-station-call = ステーションのコール
setup-ssid-oe3jjs-10-packet-station = SSID 付き — OE3JJS-10。パケットステーションは、慣例として運用者とは別の SSID を使います。これを設定するまでは何も送信されません。
setup-speed = 速度
setup-t-9600-needs-radio-s-data = 9600 には無線機のデータポートが必要です。マイクとスピーカーの経路では、両端で壊れてしまいます。これは sdroxide ではなく無線機の問題です。
setup-tx-delay = 送信の遅延
setup-flags-sent-before-frame-so = フレームの前に送られるフラグで、相手が私たちを聞き取り、そのクロックをロックできるようにします。CAT のリグでは、sdroxide だけで電波に出るまで 165〜240 ms かかり、リグ自身の送信準備時間がそれに加わります。
setup-packet-length = パケット長
setup-bytes = {" "}バイト
setup-most-single-frame-carries-shorter = 1 つのフレームが運ぶ最大量です。短いフレームは、ぎりぎりの経路でも生き残ります（当たったフレームだけが再送されます）が、オーバーヘッドは増えます。HF では 128、経路が安定している場合は 256 です。
setup-frames-sent-before-waiting-acknowledgement = 確認応答を待つ前に送るフレームの数です。多いほど良い経路を埋めますが、悪い経路では再送する量が増えます。
setup-extended = 拡張
setup-ask-mod-128 = mod-128 を要求
setup-extended-sequence-numbers-window-bigger = 7 より大きいウィンドウ用の拡張シーケンス番号です。多くのノードは、この要求を DM で拒否します。それは、あなたと話さない局とまったく同じ見え方をするため、相手がそれを求めていると知っている場合以外はオフにしておいてください。
setup-answer-calls = 呼び出しに応答
setup-accept-connections = 接続を受け付ける
setup-off-winlink-client-which-dials = Winlink クライアントの場合はオフにしてください。クライアントは外に発信するため、応答する理由がありません。ピアとして到達可能にするにはオンにします。呼び出しは端末ペインに届き、それを発信した相手と会話できます。リンクがすでに Winlink セッションで占有されているステーションは、それが終わるまで呼び出しを拒否します。
setup-connect-text = 接続時のテキスト
setup-what-caller-greeted = 呼び出し元に挨拶として送るもの
setup-sent-station-connects-us-once = リンクが確立すると、私たちに接続してきた局に送られます。空欄では何も送られません。それは呼んだ相手には壊れているように見えます。
setup-default-via = 既定の経由
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = 端末が開始するデジピーターの経路です。ローカルのノードへの経路は毎回同じで、それを打ち直すと中継が抜けてしまいます。
setup-beacon = ビーコン
setup-t-0-disables-timer = 0 でタイマーを無効にします
setup-beacon-text = ビーコンのテキスト
setup-kiss-server = KISS サーバー
setup-serve = 提供
setup-offer-modem-kiss-tnc-socket = このモデムを、ソケット上の KISS TNC として提供します。これにより、Pat、APRS クライアント、または Linux の AX.25 スタックが無線機を使えます。
setup-aprs-call = APRS コール
setup-callsign-station-beacons-under-ssid = このステーションがビーコンを送る際のコールサインで、SSID を付けます。車は OE3JJS-9、I-gate は -10、電話は -5 です。空欄にすると、上のコールサインを使います。
setup-no-callsign-station-will-not = コールサインがありません。このステーションは、ビーコン、メッセージ、確認応答のいずれも、まったく送信しません。
setup-symbol = シンボル
setup-two-characters-protocol-carries-them = プロトコルが運ぶ 2 文字です。テーブル（`/` または `\`）とシンボルコードです。最初の位置の数字または文字は、代替テーブルのアイコンの上に描かれるオーバーレイです。
setup-path = 経路
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = どこまで中継を要求するかです。これはチャンネルで最も影響の大きい単一の設定です。中継が 1 回増えるたびに、1 つの共有周波数上でネットワーク全体が行う送信が増えます。
setup-position = 位置
setup-my-grid-2 = 自分のグリッドから
setup-beacon-centre-locator-above-reported = 上のロケーターの中心をビーコンで送り、ロケーターが実際に持つあいまいさとともに報告します。6 文字のロケーターは幅が約 2 km あり、それを伝えるのが正直です。正確な座標を代わりに送る場合はチェックを外してください。
setup-latitude = 緯度
setup-longitude = 経度
setup-locator = ロケーターから
setup-not-coordinate = 座標ではありません
setup-comment = コメント
setup-sent-every-beacon = すべてのビーコンとともに送られます
setup-t-0-default-never-beacons-timer = 0 — 既定値 — はタイマーでビーコンを送りません。固定局では 30 分が慣例です。移動する局はより頻繁にビーコンを送りますが、すべてのビーコンは他の人のチャンネル時間を使います。
setup-compressed = 圧縮
setup-compressed-position-format-third-air = 圧縮された位置の形式です。電波に出る時間が 3 分の 1 で、より正確です。1990 年代以降のすべての受信機が読めます。
setup-first-goes-out-one-interval = 最初のものは、今すぐではなく、ここから 1 間隔後に送られます。
setup-messages = メッセージ
setup-acknowledge = 確認応答
setup-answer-messages-addressed-you-acknowledgement = あなた宛てのメッセージに応答します。確認応答は、このステーションが頼まれずに行う送信なので、受信専用の設定ではオフにしてください。そうすれば、ビーコンも到達可能であると主張しなくなります。
setup-flags-sent-ahead-every-frame = すべてのフレームの前に送られるフラグで、データが始まる前に、相手のモデムがキャリアを聞き取り、クロックをロックできるようにします。送信ボタンを押してから電波が出るまでのすべてを覆う必要があります。CAT のリグでは、sdroxide だけで 165〜240 ms を使い、ネットワーク経由で音声を受け取る無線機は、さらにバッファーを加えます。少なすぎると、相手はロックせず、送信はそこにあるのに何もデコードされません。
setup-shared-packet-mode = パケットモードと共有
setup-keep-stations = 局を保持
setup-how-long-station-stays-map = 最後に聞こえてから、局が地図に残る時間です。地図のフェードの基準にもなるため、短い窓では生きているものだけが表示されます。
setup-auto-reply = 自動応答
setup-answer-snr-grid-status = SNR？ / GRID？ / STATUS？ に応答
setup-answer-direct-question-addressed-you = あなた宛て、または @ALLCALL 宛ての直接の質問に、確認応答ではなく答えで応じます。他の局のトラフィックには決して応じず、自分自身にも決して応じません。
setup-heartbeat = ハートビート
setup-off = オフ
setup-turbo-does-not-beacon-local = Turbo はビーコンを送りません。ローカルと VHF の速度です。
setup-sends-your-callsign-grid-so = 相手があなたを受信可能だとわかるように、コールサインとグリッドを送ります。最初のものは、今すぐではなく、ここから 1 間隔後に送られます。
setup-beacon-frequency = ビーコンの周波数
setup-t-500-1000-hz = 500〜1000 Hz
setup-move-each-beacon-free-slot = JS8Call と同じように、各ビーコンをハートビートのサブバンド内の空いたスロットに移動します。ビーコンを探している局がそこを見ますし、無人の送信機が他人の交信を妨げるのを防ぎます。スロットは、ビーコンが実際に送られる時点で、デコードされている他のすべてから離れたものを選びます。
setup-working-freq = 交信周波数
setup-beacon-where-you-are-working = 代わりに、交信している周波数でビーコンを送ります。バンドの慣例とは異なりますが、送信するものすべてを 1 か所にまとめられます。
setup-heartbeat-reply = ハートビートへの応答
setup-answer-heartbeats-signal-report = ハートビートに信号レポートで応答
setup-tell-station-beaconed-how-well = ビーコンを送ってきた局に、あなたがその局をどれだけよく受信できたかを伝えます。既定ではオフです。混雑したバンドでは、すべてのスロットにハートビートが乗ります。それら全部に応答すると、まさにハートビートが静かに保とうとしているバンドを溢れさせてしまうためです。局ごとに 15 分に 1 回の応答に制限され、メッセージが届いている最中や、送る待ちの内容がある間は決して送りません。
setup-status-message = ステータスメッセージ
setup-tx-period = 送信の周期
setup-even = 偶数
setup-odd = 奇数
setup-auto-sequence = 自動シーケンス
setup-decode-depth = デコードの深さ
setup-fast = 速い
setup-normal = 標準
setup-deep = 深い

    Fast — one pass, no signal subtraction: quickest, fewest decodes.
    Normal — flat multi-pass subtraction: a little quicker than Deep, a little less thorough.
    Deep — the checkpointed multi-pass: the most decodes, and ~1.2 s on a busy slot.
setup-auto-tx-frequency = 自動送信周波数
setup-choose-transmit-frequency-automatically-quietest = 送信周波数を自動で選びます。送信する周期の中で最も静かな場所で、応答している局の周波数ではありません。オフにしても保持はされません。呼ばれている局の周波数で応答します。保持するには、送信周波数を保持を使ってください。
setup-hold-tx-frequency = 送信周波数を保持
setup-pin-transmit-tone-where-nothing = 送信トーンを今の位置に固定します。局に応答しても、呼び出しキューでも、CQ を呼んでも、デコードやウォーターフォールをクリックしても、何も動かしません。自動送信周波数より優先されます。バンドを変えることだけが例外です。新しいバンドで最後に設定したオフセットが、それとともに戻ります。免許がバンドプランより狭い場合に使ってください。たとえば英国の 60 m の 5357 kHz のダイヤルでは、割り当ては 5358.0 で終わるため、トーンは 1000 Hz より下に留めておく必要があります。
setup-tx-watchdog = 送信のウォッチドッグ
setup-stop-transmitting-after-long-no = 応答がなく、あなたからの操作もないまま、この時間が経過すると送信を停止します。0 で無効になります。
setup-give-up-after = 次の後にあきらめる
setup-calls = {" "}回の呼び出し
setup-contest = コンテスト
setup-everyday-exchange-grid-report-rr73 = 日常的なやり取り：グリッド、レポート、RR73。
setup-european-vhf-contests-send-signal = ヨーロッパの VHF コンテスト：信号レポート、シリアル番号、そして 6 文字のロケーターを、WSJT-X が EU VHF Contest と呼ぶメッセージ形式で送ります。CQ は CQ TEST として送られます。
setup-serial = シリアル
setup-serial-number-next-exchange-will = 次のやり取りが運ぶシリアル番号です。各交信がログされるたびに自動で進み、セッションをまたいで記憶されます。コンテストを開始するとき、または紙のログが進んだところから再開するときにだけ、手で設定してください。
setup-needs-6-character-grid = ⚠ 6 文字のグリッドが必要です
setup-eu-vhf-contest-messages-carry = EU VHF Contest のメッセージは、6 文字のロケーター（JN88 ではなく JN88DD）を運びます。自分のグリッドにそれがない間は、通常のやり取りが代わりに送られます。
setup-dxpedition = DXpedition
setup-ordinary-ft8-operation = 通常の FT8 の運用です。
setup-calling-dxpedition-running-fox-mode = Fox モードで運用している DXpedition を呼ぶ場合：1000 Hz より上から呼び、応答されたら Fox の上へ移動し、73 を送らずにその RR73 でログします。
setup-run-pile-up-several-signals = パイルアップを運用：一度に複数の信号、呼び出し元のキュー、最も強く最も珍しいものから交信します。CQ を呼ぶで開始、QSO を停止で中止します。
setup-fox-signals = Fox の信号
setup-once = {" "}同時に
setup-simultaneous-transmissions-spaced-60-hz = 同時送信で、60 Hz 間隔です。送信機の電力を分け合うため、信号が増えると、それぞれが弱くなります。
setup-tx-audio = 送信音声
setup-deviation-fm = 偏移（FM）
setup-drive-ssb = 駆動（SSB）
setup-grid = グリッド
setup-report = レポート
setup-r-report = R+レポート

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = 、10 W スケール、再スケール
amp-t-10-w-scale = 、10 W スケール
amp-t-2-w-scale-rescaled = 、2 W スケール、再スケール
amp-t-2-w-scale = 、2 W スケール
amp-t-100-w-scale-rescaled = 、100 W スケール、再スケール
amp-t-100-w-scale = 、100 W スケール
amp-t-20-w-scale-rescaled = 、20 W スケール、再スケール
amp-t-20-w-scale = 、20 W スケール
amp-rescaled = 、再スケール
amp-and = {" "}と{" "}
amp-not-while-transmitting-switching-relay = 送信中は不可：RF がかかったままリレーを切り替えると、破損するおそれがあります。先にキーを解除してください。
amp-no-amplifier-set-up-choose = アンプが設定されていません。設定 → アンプでそのポートを選んでください。
amp-settings = 設定
amp-dock = ドック
amp-put-amp-window-back-inside = AMP ウィンドウをメインウィンドウの中に戻します
amp-pop-out = ポップアウト
amp-give-amp-window-window-own = AMP ウィンドウに独自のウィンドウを与え、別の画面に移動できるようにします
amp-rig = 、リグから
amp-rig-flagged-reading = 、リグが読み値に印を付けました
amp-drv = 、DRV から
amp-rig-s-swr = 、リグの SWR から
amp-ptt-keys-amplifier-radio-off = PTT は無線機とともにアンプをキーします。OFF はバイパスのままにします。クリックで切り替えます。
amp-switch-antenna-jack = アンテナジャックを切り替え
amp-band-down = バンドを下げる
amp-band-up = バンドを上げる
amp-tuner = チューナー
amp-tuned = 同調済み
amp-tune = 同調
amp-start-tune-then-key-steady = 同調を開始し、その後で一定のキャリア（CW、FM、RTTY）をキーしてください。もう一度押すと停止します。
amp-put-tuner-line-bypass = チューナーを回路に入れるか、バイパスします
amp-tuning = 同調中…
amp-bottom = 下
amp-pick-any-reading-either-needle = どちらの針にも、任意の読み値を選べます。上の針は白、下の針は青です。
amp-forward-power-measured-radio-amplifier = 順方向電力。無線機が測定します。OFF ではアンプは報告しません
amp-forward-power-taken-drv-amplifier = 順方向電力。DRV から取得します。OFF ではアンプは報告しません。バイパス中は無線機の電力がそのまま通ります
amp-forward-power = 順方向電力
amp-reflected-power-measured-radio-amplifier = 反射電力。無線機が測定します。OFF ではアンプは報告しません
amp-reflected-power-worked-out-fwd = 反射電力。FWD と SWR から算出します。OFF ではアンプは報告しません
amp-reflected-power = 反射電力
amp-standing-wave-ratio-red-past = 定在波比。同調の上限（設定 → 全般）で赤になるか、それを超えます。アンテナを確認してください
amp-standing-wave-ratio-measured-radio = 定在波比。無線機が測定します。OFF ではアンプは報告しません
amp-standing-wave-ratio = 定在波比
amp-drive-radio = 無線機からの駆動
amp-drain-supply = ドレイン電圧
amp-drain-current = ドレイン電流
amp-temp = 温度
amp-heat-spreader-temperature-yellow-49 = ヒートスプレッダーの温度。49 °C（120 °F）から黄、65 °C（149 °F）を超えると赤、70 °C（158 °F）を超えると回線から外れます。クリックで °C/°F を切り替えます。
amp-c = °C
amp-f = °F
amp-performance = 性能
amp-numbers-hold-peaks-bars-are = 数値はピークを保持、バーはライブです
amp-input-power = 入力電力
amp-supply-volts-drain-amps = 電源電圧 × ドレイン電流
amp-efficiency = 効率
amp-output-power-input-power = 出力電力 ÷ 入力電力
amp-gain = 利得
amp-output-power-over-drive-decibels = 駆動に対する出力電力（デシベル）
amp-off-line-over-temperature = 回線から外れています · 温度超過
amp-connected-transmitting = 接続中 · 送信中
amp-connected = 接続中
amp-connecting = 接続中…
amp-atu-bypassed = ATU バイパス中
amp-no-inductance-needed = インダクタンスは不要
amp-no-capacitance-needed = キャパシタンスは不要
amp-c-out-antenna-side = C OUT · アンテナ側
amp-c-transmitter-side = C IN · 送信機側

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = 受信したチャンク（点灯）と、まだ欠けているチャンク（暗色）
sstv-sstv = SSTV
sstv-auto = 自動
sstv-signal = 信号
sstv-station-sending-mode-sdroxide-does = 局が sdroxide にないモードで送信しています。信号と同調は問題ありません。このモードのデコーダーがないだけです。あなた側で直すものはありません。
sstv-listening = 聞いています…
sstv-restart-rx = 受信を再開
sstv-abandon-picture-being-received-listen = 受信中の画像を中止し、次のヘッダーを待ちます。誤ったモードでデコードを始めた送信用です。そうでなければ、受信機はそのモードが終わるまで拘束されたままです。
sstv-callsign-last-station-sent-fsk = 最後の局が画像の後に FSK ID として送ったコールサインです。
sstv-tx-slant = 送信のスラント
sstv-transmit-clock-trim-ppm-remove = 相手側のデコーダーのスラントを取り除くための送信クロックのトリム（ppm）
sstv-reset-0-ppm = 0 ppm にリセット
sstv-fsk-id = FSK ID
sstv-send-your-callsign-tones-after = 各画像の後に、コールサインをトーンで送ります。SSTV のリピーターや他のプログラムが読む識別です。約 2.5 秒が加わり、コールサインを設定するまでは何も送りません。
sstv-tx-lead = 送信のリード
sstv-silence-sent-after-keying-before = キーした後、画像のリーダーと VIS コードの前に送られる無音です。そのヘッダーのどれかを逃すデコーダーは、画像をまったく表示しません。そのため、リグに PTT を頼んでから実際に電波が出るまでの隙間を埋めます。すぐにキーする SDR では 0 です。
sstv-live = ライブ
sstv-waiting-picture = 画像を待っています…
sstv-no-low-audio = 音声なし / 低い
sstv-received = 受信済み
sstv-reading-radio-s-pictures = 無線機の画像を読み取っています…
sstv-decoded-pictures-collect-here = デコードされた画像はここに集まります。
sstv-click-enlarge-right-click-delete = クリックで拡大 · 右クリックで削除
sstv-delete-picture = この画像を削除
sstv-loading = 読み込み中…
sstv-transmit = 送信
sstv-click-edit-slot-s-message = クリックでこのスロットのメッセージを編集 · ダブルクリックで画像を読み込み
sstv-change-image = 画像を変更…
sstv-load-image = 画像を読み込み…
sstv-clear = クリア
sstv-empty-slot-s-picture-message = このスロットの画像を空にします（メッセージは残ります）
sstv-banner = バナー…
sstv-what-printed-across-top-every = このステーションが送るすべての画像の上部に印刷される内容です
sstv-fetching-slot-s-picture = このスロットの画像を取得中…
sstv-load-image-into-slot = このスロットに画像を読み込む →
sstv-drawn-slot-s-image = このスロットの画像の上に描かれます
sstv-abort-tx = {" "}送信を中止{" "}
sstv-delete-picture-store = この画像をストアから削除
sstv-received-image = 受信した画像
sstv-no-longer-store = ストアにもうありません
sstv-loading-full-size = 原寸で読み込み中…
sstv-save-picture = 画像を保存…
sstv-delete-sure = 削除しますか？
sstv-click-again-delete-good = もう一度クリックすると、完全に削除されます
sstv-delete = 削除…
sstv-image-banner = 画像のバナー
sstv-drawn-into-every-picture-station = このステーションが送るすべての画像に、スロット自身のメッセージの上から描かれます。
sstv-draw-banner = バナーを描く
sstv-substituted-when-picture-composed = 画像を組み立てるときに置き換えられます：
sstv-top-left = 左上
sstv-top-right = 右上
sstv-colours = 色
sstv-strip-top-edge-fades-black = 帯の上端です。黒に、またはその下のグラデーションの色にフェードします
sstv-both-texts = 両方のテキスト
sstv-gradient = グラデーション
sstv-fade = フェード先
sstv-fade-strip-top-colour-second = 帯を、黒ではなく、上端の色から 2 つ目の色にフェードします。
sstv-text-outline = テキストの縁取り
sstv-draw-outline-around-banner-text = バナーのテキストの周りに縁取りを描き、混み合った帯や明るい帯の上でも読めるようにします。
sstv-text-gradient = テキストのグラデーション
sstv-fade-banner-text-colour-second = バナーのテキストを、画像の幅にわたって、その色から 2 つ目の色にフェードします。
sstv-rainbow-text = レインボーテキスト
sstv-all-text-overriding-colours-above = すべてのテキスト。上の色よりも優先されます
sstv-draw-every-text-picture-banner = 画像内のすべてのテキスト（バナーとメッセージ）を、上の色を無視して、水平のレインボーとして描きます。
sstv-message = メッセージ
sstv-slot-message-s-text-colour = スロットのメッセージのテキストの色
sstv-outline = 縁取りの色
sstv-height = 高さ
sstv-how-tall-strip-pixels-transmitted = 送信される画像のピクセルで、帯がどれだけ高いかです。テキストの大きさはこれから決まります。SSTV のフレームは幅がわずか 320 ピクセルなので、バナーを高くすると、相手側で読めるようになります。
sstv-reset = リセット
sstv-back-callsign-left-version-right = 左のコールサインと右のバージョンに戻します
sstv-waiting-station-s-digital-mode = 局のデジタルモードの設定が読み込まれるのを待っています…
sstv-rifp = RIFP
sstv-size = サイズ
sstv-calling-frequency-draft-names = 草案が挙げる呼び出し周波数
sstv-encode = エンコード
sstv-how-picture-encoded-into-object = RIFP が運ぶオブジェクトに、画像がどうエンコードされるかです。自動は各方式を試し、最も小さいものを送ります。
sstv-try-every-encoding-send-smallest = すべてのエンコードを試し、最も小さいものを送ります（非可逆にはなりません）
sstv-gray = グレー
sstv-grayscale-depth-rifp-s-raster = グレースケールの深さ。RIFP のラスターは定義上グレースケールで、マニフェストに色の入る余地はありません。
sstv-dither = ディザー
sstv-diffuse-quantisation-error-worth-below = 量子化誤差を拡散します。8 ビット未満では価値があります
sstv-caption = キャプション
sstv-sent-manifest-content-hint-shown = マニフェストのコンテンツヒントとして送られ、受信側は画像の下に表示します。テキストとして伝わり、画像の中には描かれません。
sstv-what-picture = この画像の内容
sstv-repeat-data = データを繰り返す
sstv-send-every-data-frame-many = 各データフレームを、この回数だけ送ります。RIFP は再送要求のない一方向なので、これが受信側が得られる唯一の回復手段です。
sstv-chunk = チャンク
sstv-payload-octets-per-data-frame = データフレームあたりのペイロードのオクテット数（プロファイルは 192 を推奨）
sstv-valid-frames-frames-failed-crc = 有効なフレーム、CRC に失敗したフレーム、そして検証済みの完全な画像
sstv-no-transfer-progress = 転送は進行していません
sstv-forget-incomplete-transfer = この未完了の転送を破棄

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = 呼び出し
windows-delete = 削除
windows-edit-name-frequency-mode-antenna = 名前、周波数、モード、アンテナ、およびレピーターの設定を編集
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = このチャンネルに保存されたレピーターシフト、トーン、1750 Hz バースト
windows-abandon-edit-escape = 編集を取り消す（Esc）
windows-save = 保存
windows-keep-changes-enter = 変更を保存（Enter）
windows-recall-channel-without-moving-antenna = アンテナを動かさずに、このチャンネルを呼び出します
windows-antenna-switch-when-channel-recalled = このチャンネルを呼び出したときに切り替えるアンテナです。— 無線機が現在使っているものはそのままにします。
windows-shift = シフト
windows-tone = トーン
windows-invert = 反転
windows-normal = 通常
windows-dcs-polarity = DCS の極性
windows-open-every-over-tone-burst = このチャンネルを呼び出している間、交信ごとにトーンバーストで開きます
windows-memories = 記憶
windows-memory-name = 記憶の名前
windows-store = 保存
windows-folder-name = フォルダー名
windows-new-folder = 新しいフォルダー
windows-import = 読み込み
windows-read-channel-list-chirp-csv =
    CHIRP の CSV ファイル（.csv）からチャンネル一覧を読み込みます。レピーターのディレクトリの書き出し、海上または PMR のチャンネル表、CHIRP が書き出せるその他のものです。

    Each channel brings its frequency, mode, repeater shift and CTCSS or DCS tone with it. Channels already on this list — same frequency, same mode — are skipped, so re-importing an updated directory adds what is new rather than doubling what is not.
windows-chirp-csv = CHIRP CSV
windows-export = 書き出し
windows-write-list-out-chirp-csv = この一覧を CHIRP の CSV ファイルとして書き出します。ハンディ機に読み込むか、バックアップとして保存します。
windows-no-memories-yet = まだ記憶がありません
windows-drag-memory-onto-folder-file = 記憶をフォルダーにドラッグすると格納されます。その下にドラッグすると格納を解除します
windows-drop-here-unfile = ここにドロップすると格納を解除します
windows-sort = 並べ替え
windows-order-they-were-stored = 保存された順
windows-name-ignoring-case = 名前順（大文字と小文字を区別しない）
windows-frequency = 周波数順
windows-band-then-frequency-inside-each = バンド順、そのなかで周波数順
windows-descending-click-ascending = 降順。クリックで昇順
windows-ascending-click-descending = 昇順。クリックで降順
windows-delete-folder-memories-move-top = フォルダーを削除します。その記憶は最上位に移動します
windows-rename-folder = フォルダー名を変更
windows-empty-drop-memories-here = 空です。ここに記憶をドロップしてください
windows-voice-keyer = ボイスキーヤー
windows-rec-records-your-microphone-play = REC はマイクから録音し、PLAY は録音した内容を聴けるようにし、TX は電波に出します。テンキーのキー、MIDI パッド、または rigctld の send_voice_mem でも同じです。
windows-stop-store = 停止して保存
windows-stop = 停止
windows-play = 再生
windows-stop-listening = 聴くのをやめる
windows-nothing-recorded-slot = このスロットには何も録音されていません
windows-not-while-transmitting = 送信中は不可
windows-listen-message-nothing-transmitted = このメッセージを聴きます。何も送信されません
windows-stop-transmitting = 送信を停止
windows-tune-active-switch-off-first = 同調が動作中です。先にオフにしてください
windows-transmit-message = このメッセージを送信
windows-erase-recording = この録音を消去
windows-rade-message-encoded-digital-voice = RADE：メッセージは、ライブの交信とまったく同じように、デジタル音声のコーデックでエンコードされます。

## TX Audio window: messages with values

txaudio-rec-hint = 送信機をキーせずに、マイクからテストフレーズを録音します。最大 { $max } 秒です。もう一度クリックすると停止します。処理されないまま保持されるため、再生ではあなたが行う変更のすべてに同じフレーズを通せます。
txaudio-phrase-length = { $secs } 秒のフレーズ
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = 入力ゲインを { $gain } dB に設定しました（音声のピークは { $loud } dBFS でした）。{ $clip }
txaudio-listening = 聞いています… { $secs } 秒
txaudio-auto-set-hint = { $secs } 秒の間、普段どおり話してください。電波に出すときと同じように話すと、声のピークが約 { $target } dBFS になるように入力ゲインが設定されます。もう一度クリックするとキャンセルします。
txaudio-eq-point = { $hz } Hz、{ $db } dB — ドラッグで移動
txaudio-not-a-profile = TX 音声のプロファイルではありません：{ $error }
txaudio-imported = 「{ $name }」を読み込みました。
txaudio-saved = 「{ $name }」を保存しました。
txaudio-deleted = 「{ $name }」を削除しました。
txaudio-loaded-preset = 「{ $name }」の出発点を読み込みました。

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · オフライン
solar-radius-exaggeration = 地球と月の半径の誇張（この月の軌道の縮尺では最大 { $max }×）
solar-brightest-cell = 最も明るいセルは約 { $peak } 経路 · { $bands } { $bands ->
        [one] バンド
       *[other] バンド
    }
solar-hits-in-hour = 1 時間に { $hits } 件
solar-passes-from = { $name } · { $grid } からのパス
solar-elements-age = 軌道要素 { $age } 経過 · SGP4
solar-point-at = 方位 { $az }°（{ $compass }）、仰角 { $el }° に向けます。
solar-published-as = { $name } として公開
solar-fly-there = { $name } — ↵ でそこへ飛びます
solar-sats-tracked = { $total } 件中 { $n } 件を追跡
solar-bodies-found = { $total } 個中 { $n } 個の天体
solar-sats-and-bodies = { $total } 件中 { $n } 件の衛星を追跡 · { $bodies_total } 個中 { $bodies } 個の天体
solar-confidence-on-band = { $band } で { $confidence }
solar-kp-now = 現在の Kp は { $kp }
solar-kp-in = { $hours } 時間後の Kp は { $kp }
solar-valid-age = 有効 { $valid } · { $age } 経過
solar-full-bar = 満タンのバー = 世界の { $share }
solar-own-paths-memory = このステーション自身の経路の行き先 · { $minutes } 分の記憶
solar-n-storms = { $n } 件の嵐
solar-clouds-legend = 雲 { $time } · { $age } 経過 · { $channels } · { $storms } · 雷はシミュレート
solar-cme-eta = 到着予想 { $eta }（+{ $hours } 時間）
solar-cme-arrived = 到着は { $eta }（{ $hours } 時間前）でした
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = 地球に向かう CME { $time } · { $speed } km/s · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = { $mode } の設定
setup-transmits-as = 送信時のコール：{ $call }
setup-decimal-degrees = 十進角度。{ $hemisphere } の場合は負の値です。地図からそのまま貼り付けられます。

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } はこの SWR では信頼できません
amp-lower-drive-first = 先に駆動を下げてください。{ $drive } W では、回路に入っているアンプを過駆動します（その上限は { $limit } W です）。
amp-live-value = ライブ { $value } { $unit }
amp-tripped = 作動{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = 上位約 { $n } 経路
panels-no-agreed-frequency = { $mode } について合意された周波数はありません。今いる周波数を保存するには、これを開いてください
panels-mode-frequencies-hint = { $mode } の { $n } 個の周波数です。1 つ選ぶとダイヤルが同調し、自分のものを保存できます
panels-mode-frequencies = { $mode } · 周波数
panels-global-convention = IARU リージョン { $region } のバンドプランでは、狭帯域のデータは置かれていない世界的な慣例です。ここで送信する前に、自分のバンドプランを確認してください。
panels-outside-data-segment = 琥珀色：リージョン { $region } のデータセグメントの外です。
panels-save-dial = ＋ { $mhz } MHz を保存
panels-save-dial-hint = このダイヤルを { $mode } の下に記憶し、今後ここに提示します。ステーションに保存されるため、この無線機に接続されたすべての画面で使えます。
panels-tr-period = { $period } 秒の T/R 周期
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = この周期：{ $slot } 秒のうち { $into } 秒。デコードは周期の終わりに届き、次の送信はそのときに始められます。{ $mark }
panels-burst-mark = {" "}マークは、バーストが終わる位置で、{ $secs } 秒目です。
panels-tr-period-burst = { $slot } 秒の T/R 周期、{ $burst } 秒のバースト

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } チャンネル
windows-slot-n = スロット { $n }
windows-record-from-mic = マイクから録音（最大 { $max } 秒）

## SSTV panel: messages with values

sstv-picture-too-big = その画像は { $size } MB です。上限は { $limit } MB です。
sstv-auto-mode = 自動（{ $mode }）
sstv-not-decoded = { $what } — デコードされていません
sstv-last = 直前：{ $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = デコードされた画像は { $place } に保存され、ここに集まります。
sstv-older-in-store = ストア内の古いものが { $n } 件あります
sstv-older-load-more = 古いものが { $n } 件 — さらに読み込む
sstv-delete-picture-from = この画像を { $place } から削除
sstv-rifp-profile = { $profile } · 約 { $khz } kHz を占有 · ダイヤルはチャンネルの中心です
sstv-rifp-too-wide = ⚠ { $profile } は約 { $khz } kHz を占有します — 狭帯域のセグメントには広すぎます
sstv-rifp-no-frequency = RIFP は周波数を割り当てません。sdroxide は、どこにダイヤルを合わせても送信します。{ $khz } kHz のチャンネルは、広帯域または FM の運用が許可されている場所（{ $segments }）にしか収まらず、狭帯域のセグメント、特に HF では収まりません。その中でも、あなた自身の免許の条件はより狭い場合があります。運用者はあなたです。ご自身の規則を確認してください。
sstv-rifp-tx-frame = ● 送信フレーム { $frame }/{ $frames } · 残り { $secs } 秒
sstv-rifp-sent-as = { $encoding } として送信 · { $octets } オクテット
sstv-rifp-rx-stats = フレーム { $frames } · 不良 { $bad } · 画像 { $pictures }
sstv-rifp-session-idle = セッション { $session } · 待機 { $secs } 秒
sstv-rifp-session-waiting = セッション { $session } · チャンクを保持、マニフェストを待機中 · 待機 { $secs } 秒
sstv-store-on-radio = 無線機上の { $dir } に
sstv-store-in = { $dir } に

## decodes (src/app/panels/decodes.rs)

decodes-decodes = デコード
decodes-adif = ADIF
decodes-sort = 並べ替え
decodes-single-list = 単一のリスト
decodes-every-decode-one-list-newest = 奇数と偶数の周期のブロックではなく、すべてのデコードを 1 つのリストにまとめ、最新の周期を先頭にします。並べ替えのチップは、周期ごとではなくリスト全体を並べ替えます。各行にはスロットの時刻が付きます。偶数スロットはシアン、奇数スロットは金色です。
decodes-cq-only = CQ のみ
decodes-only-stations-calling-cq-only = CQ を呼んでいる局だけです。応答してよい呼び出しだけを表示します。指定型の CQ（DX、EU、JA、POTA、TEST など）は、あなたを指名しているときに一覧され、別の人を指名しているときは隠されます。
decodes-new-only = 新規のみ
decodes-only-stations-would-new-entity = 新しくなる局だけです。エンティティ、バンドとスロット、グリッド、またはコールサインで判定します。
decodes-auto-tx-frq = 自動 TX 周波数
decodes-overridden-hold-tx-lift-hold = 送信周波数を保持によって上書きされています。送信周波数の動き方を選ぶには、保持を解除してください。
decodes-hold-tx = 送信周波数を保持
decodes-pin-transmit-tone-where-nothing =
    送信トーンを今の位置に固定します。局に応答しても、呼び出しキューでも、CQ を呼んでも、デコードやウォーターフォールをクリックしても、何も動かしません。移動させるにはオフにし、その後もう一度オンにしてください。

    Changing band is the one exception, and it is your own act: the offset you last set on the new band comes back with it, so a 60 m figure is not carried onto 20 m and 20 m's is not carried onto 60 m.

    For where the licence is narrower than the band plan — on a UK 60 m dial of 5357 kHz the allocation ends at 5358.0, so the tone has to stay under 1000 Hz, and either automatic mover will walk out of the band between one over and the next.
decodes-type-transmit-offset-hz-then = 送信オフセットを Hz で入力し、Enter を押してください。免許がバンドプランより狭い場合、端より下の範囲全体があなたが選べる範囲になるため、ここでは何も提案されません。
decodes-clear-rx = {" "}RX をクリア{" "}
decodes-empty-decode-list-nothing-air = デコードの一覧を空にします。電波に出ているものは何も止まりません。
decodes-even = 偶数
decodes-text = テキスト
decodes-dxcc = DXCC
decodes-band = バンド
decodes-grid = グリッド
decodes-dupe = 重複
decodes-reply = 応答
decodes-queued-click-remove = 待機中。クリックで削除
decodes-work-station-after-current-one = 現在の局の後に、この局と交信します
decodes-qsos-worked-since-sdroxide-was = sdroxide を起動してから交信した QSO
decodes-qso-complete = ✓ QSO 完了
decodes-tx = ● TX
decodes-dxpedition-mode-transmit-frequency-held = DXpedition モードです。Fox が応答するまで、送信周波数はパスバンドの Fox 側（1000 Hz より下）から外して保持されます。
decodes-watchdog = ウォッチドッグ
decodes-transmitting-stopped-no-reply-no = 送信を停止しました。ウォッチドッグの期間中、応答も操作もありませんでした。CQ を呼ぶか、メッセージを選ぶと再開します。
decodes-well-inside-tolerance = 許容範囲の内側にあります。
decodes-no-active-qso-pick-decode = 進行中の QSO はありません。応答するデコードを選ぶか、CQ を呼んでください
decodes-ft8-engine-idle = FT8 エンジンは待機中
decodes-being-worked = 交信中
decodes-clear = クリア
decodes-no-messages = — メッセージなし —
decodes-call-cq = {"  "}CQ を呼ぶ{"  "}
decodes-stop-qso = {" "}QSO を停止{" "}
decodes-stop-tx = {" "}送信を停止{" "}
decodes-free-text-13-chars = 自由テキスト（13 文字）
decodes-send = 送信

## sat (src/app/sat.rs)

sat-pass-profile = パスのプロファイル
sat-satellite = 衛星
sat-satellites = 衛星
sat-passes = パス
sat-set-your-grid-passes-settings = パスのためにグリッドを設定してください（設定 ▸ 全般）。
sat-next = 次
sat-peak = 最大仰角 ≥
sat-all-satellites = すべての衛星
sat-off-only-satellites-frequencies-file = オフ：周波数が登録されている衛星だけ（交信できるもの）と、自分で貼り付けたものです。オン：一覧にあるすべての衛星です。購読していると数千になることもあり、検索に時間がかかります。
sat-no-passes-window-lower-peak = この窓にはパスがありません。最大仰角を下げるか、窓を広げてください。
sat-length = 長さ
sat-peak-2 = 最大仰角
sat-from-to = 開始 → 終了
sat-up-now = ● 今、見えています
sat-unlock = ロック解除
sat-release-lock-dial-stays-where = ロックを解除します。ダイヤルはそのままの位置に残ります
sat-azimuth = 方位角
sat-elevation = 仰角
sat-range = 距離
sat-rate = 変化率
sat-downlink = ダウンリンク
sat-doppler-rx = ドップラー RX
sat-uplink = アップリンク
sat-doppler-tx = ドップラー TX
sat-no-pass-inside-next-48 = あなたの QTH から、今後 48 時間以内にパスはありません。
sat-elements-stale-corrections-suspended-until = ⚠ 軌道要素が古くなっています。TLE を更新するまで補正は停止中です
sat-doppler-correction-needs-iq-front = ドップラー補正には IQ のフロントエンドが必要です。CAT のリグは追跡しかできません
sat-rotator-not-connected = ローテーター：未接続
sat-doppler = ドップラー
sat-apply-computed-correction-rx-tx = 計算された補正を RX と TX に適用します
sat-rotator = ローテーター
sat-steer-antenna-through-rotctld-client = rotctld クライアントを通してアンテナを操作します（設定 ▸ サーバー）
sat-name-norad = 名前または NORAD
sat-set-your-grid-passes-settings-2 = パスのためにグリッドを設定してください（設定 ▸ 全般）
sat-norad = NORAD
sat-pass = パス
sat-always-visible = 常に見えます
sat-not-here = ここからは見えません
sat-no-elements = 軌道要素なし
sat-pick-satellite-see-links-lock = 衛星を選ぶと、そのリンクが表示され、ロックできます。
sat-no-frequencies-file-one-add = この衛星の周波数は登録されていません。設定 ▸ TLE で追加してください。
sat-link = リンク
sat-downlink-mhz = ダウンリンク（MHz）
sat-uplink-mhz = アップリンク（MHz）
sat-mode = モード
sat-tune = 同調
sat-set-dial-mode-link-no = ダイヤルとモードをこのリンクに設定します。ロックも補正もしません
sat-locked = {" "}ロック中{" "}
sat-lock = {" "}ロック{" "}
sat-track-satellite-doppler-correction-rx = この衛星を追跡します。RX と TX のドップラー補正、トランスポンダーにマップされたアップリンク、3D ビューもそれに従います

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ 停止{" "}
wefax-end-chart-now-save-what = 今、チャートを終了し、届いた内容を保存します
wefax-start = {" "}● 開始{" "}
wefax-start-chart-now-without-waiting = 開始トーンを待たずに、今チャートを開始します
wefax-phasing = 位相合わせ中…
wefax-subcarrier-offset-1900-hz-tune = 1900 Hz からのサブキャリアのオフセットです。おおよそゼロになるように同調してください。ファクスのキャリアは、黒が 1500 Hz、白が 2300 Hz です。受信機が数百 Hz ずれると、画像が真っ黒または真っ白にクリップされます。
wefax-auto-start = 自動開始
wefax-auto-stop = 自動停止
wefax-end-450-hz-stop-tone = 450 Hz の停止トーンで終了します。オフにすると、続けて複数のチャートを送る局の間も録音し続けます。
wefax-phase = 位相
wefax-slant = スラント
wefax-sample-clock-trim-if-chart = サンプルクロックのトリムです。チャートが左に傾いている場合は増やし、右に傾いている場合は減らしてください。100 ppm ずれたサウンドカードでは、15 分のチャートは行の大半が横にずれてしまいます。
wefax-view = 表示
wefax-scale-chart-panel-width = チャートをパネルの幅に合わせて拡大縮小
wefax-whole = 全体
wefax-shrink-chart-until-all-view = チャート全体が一度に見えるまで縮小します
wefax-half-size = 半分のサイズ
wefax-one-screen-pixel-per-fax = ファクスの 1 ピクセルにつき画面の 1 ピクセル。詳細を見るにはスクロールしてください
wefax-twice-size-scroll-move-around = 2 倍のサイズ。スクロールしてチャート内を移動します
wefax-height = 高さ
wefax-stretch-picture-vertically-chart-came = 画像を垂直方向に伸ばします。つぶれたり伸びたりして出てきたチャートは、たいてい誤った行速度でデコードされています。これで読めるようになり、LPM チップが正しく直します。値を入力するにはダブルクリックしてください。
wefax-reset = リセット
wefax-back-picture-s-own-proportions = 画像自身の比率に戻します
wefax-follow = 追従
wefax-keep-newest-lines-view-scrolling = 最新の行を表示し続けます。上にスクロールするとオフになり、届いた内容を読めます。最下部に戻るとオンになります。
wefax-saved = 保存済み
wefax-path = パス
wefax-reading-radio-s-charts = 無線機のチャートを読み取り中…
wefax-finished-charts-collect-here = 完了したチャートはここに集まります。
wefax-delete-chart = このチャートを削除
wefax-loading = 読み込み中…
wefax-stations = 局
wefax-broadcast-radiofax-schedules-picking-one = 放送のラジオファクスのスケジュール。1 つ選ぶとダイヤルが同調し、回線速度も設定されます
wefax-frequencies-are-published-carrier-dial = 周波数は公表されたキャリアです。ダイヤルはその 1.9 kHz 下になります。これは自動で行われます。スケジュールは変わり、局は停止するため、これは探し始める場所として扱ってください。時刻表ではありません。
wefax-delete-chart-store = このチャートをストアから削除
wefax-newer = ◀ 新しい
wefax-chart-received-after-one = このチャートの後に受信したチャート
wefax-older = 古い ▶
wefax-chart-received-before-one = このチャートの前に受信したチャート
wefax-no-longer-store = ストアにもうありません
wefax-loading-full-size = 原寸で読み込み中…
wefax-save = 保存
wefax-save-copy-chart-computer = このチャートのコピーを、このコンピューターに保存します
wefax-sure = よろしいですか？
wefax-click-again-delete-chart-good = もう一度クリックすると、このチャートを完全に削除します
wefax-delete = 削除
wefax-file-chart-was-saved = このチャートが保存されたファイル

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = APRS モデムを開始中…
aprs-aprs = APRS
aprs-no-callsign = コールサインなし
aprs-nothing-will-transmitted-no-beacon = このステーションにコールサインができるまで、何も送信されません。ビーコン、メッセージ、確認応答のいずれも送られません。設定 → 全般で設定するか、SSID 付きの APRS 専用のものを 設定 の下で設定してください。
aprs-no-position = 位置なし
aprs-beacon-needs-somewhere-report-fill = ビーコンには報告する場所が必要です。設定 → 全般でロケーターを入力するか、設定の下で座標を指定してください。
aprs-setup = 設定
aprs-callsign-symbol-digipeater-path-position = コールサイン、シンボル、デジピーターの経路、位置、ビーコン
aprs-how-often-beacon-your-position = 無人で、位置をどれくらいの頻度でビーコンするかです。`off` — 既定値 — では、決してビーコンを送りません。モードを選んだだけでステーションが電波に出ないようにするためです。固定局では 30 分が慣例で、移動する局はより頻繁に送ります。ただし、すべてのビーコンは他の人のチャンネル時間を使います。
aprs-beacon = {" "}ビーコン{" "}
aprs-station-has-no-callsign-see = このステーションにはコールサインがありません。左の警告を参照してください。
aprs-station-has-no-position-report = このステーションには報告する位置がありません。左の警告を参照してください。
aprs-send-one-position-report-now = 間隔を待たずに、位置報告を 1 回今送ります。ほかのすべてと同様に、チャンネルが空くのを待ちます。
aprs-stations = 局
aprs-nothing-heard-yet = まだ何も聞こえていません
aprs-heard-direct = 直接聞こえた
aprs-two-characters-station-sent-say = 局が送ってきた、それが何であるかを示す 2 文字です
aprs-killed = 取り消し
aprs-station-put-object-here-has = このオブジェクトをここに置いた局が、それを取り消しました
aprs-sender-deliberately-reported-square-rather = 送信者は、点ではなく区画を意図的に報告しました。地図は区画を描きます。
aprs-digipeaters-last-frame-came-through = 最後のフレームが通ったデジピーターです。実際に中継したものには `*` が付きます。
aprs-message = {" "}メッセージ{" "}
aprs-address-message-box-station = メッセージ欄の宛先をこの局にします
aprs-center = {" "}中央{" "}
aprs-put-station-middle-map-double = この局を地図の中央に置きます。地図をダブルクリックすると、表示は自動の合わせ込みに戻ります。
aprs-channel = チャンネル
aprs-messages = メッセージ
aprs-frames-arrived-failed-their-check = 届いたものの、チェックシーケンスに失敗したフレームです。衝突、フェード、または読むには弱すぎる信号が原因です。何もデコードされていないのに数が増える場合、モデムはチャンネルを *聞いています*。レベルメーターを確認し、無線機のスケルチを開いてください。
aprs-frames-read-cleanly-off-channel = チャンネルから正しく読み取れた、APRS ではないフレームです。他の人のパケットセッション、またはこのビルドがデコードしない形式です。
aprs-every-frame-channel-arrived-other = 届いたとおりの、チャンネル上のすべてのフレームです。他の人のトラフィックと、デコーダーが読めなかったものを含みます。
aprs-nothing-channel-yet = まだチャンネルに何もありません
aprs-no-messages = メッセージはありません。
aprs-waiting-channel = チャンネルを待っています
aprs-sent-retried-still-waiting-acknowledged = 送信して再試行しました。まだ確認応答を待っています
aprs-air-waiting-acknowledged = 電波に出ていて、確認応答を待っています
aprs-acknowledged-far-end = 相手が確認応答しました
aprs-refused-far-end-not-retried = 相手が拒否しました。再試行はしません
aprs-no-ack = 確認応答なし
aprs-no-answer-after-every-retry = すべての再試行の後も応答なし
aprs-send = {" "}送信{" "}
aprs-station-has-no-callsign-so = このステーションにはコールサインがないため、何も送信できません。設定 → 全般で設定するか、SSID 付きの APRS 専用のものを 設定 の下で設定してください。
aprs-needs-station-address-something-say = 宛先の局と、伝える内容が必要です。
aprs-send-keep-retrying-until-far = 送信し、相手が確認応答するまで再試行を続けます。メッセージは、このチャンネルで応答されるただ 1 つのものです。
aprs-busy = 使用中
aprs-stations-heard-still-inside-window = 聞こえた局のうち、APRS 設定で定めた窓の中にあるものと、今チャンネルに別の局がいるかどうかです。使用中の間は、何もキーされません。
aprs-tx = ● TX
aprs-over-progress-frames-waiting-channel = 進行中の交信、チャンネルが空くのを待つフレーム、または次の予定されたビーコンまでの時間です。
aprs-audio-reaching-modem-flat-means = モデムに届いている音声です。平坦であれば、何も届いていないことを意味します。sdroxide が聴いている無線機のデータ出力が正しいか確認してください。モデムがキャリアを聞いている間は緑に点灯します。

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = VDL2 デコーダーを開始中…
vdl2-aircraft-datalink = 航空機のデータリンク
vdl2-tune-middle-vdl2-group-decoder = VDL2 グループの中央に同調します。デコーダー自身の窓はそこからスライドし、受信機が届く範囲で 14 チャンネルのうちできるだけ多くを取り込みます。
vdl2-setup = 設定
vdl2-which-channels-listen-how-hard = どのチャンネルを聴くか、バーストがどれだけ強い必要があるか、そしてどれだけのログを保持するか
vdl2-save-vdl2-message-log-file = VDL2 のメッセージログをファイルに保存
vdl2-vdl2-channel = VDL2 チャンネル
vdl2-messages = メッセージ
vdl2-address-flight-registration-label = アドレス、便名、登録記号、またはラベル
vdl2-held = 保持中
vdl2-hold = 保持
vdl2-hold-message-log-where-so = メッセージログをその位置に保持します。次の送信によって画面の上へ押し上げられることなく、1 件を読めます。デコーダーは動き続け、カウンターも動き続けます。解除すると、その間に届いたものがすべて表示されます。
vdl2-nothing-decoded-yet = まだデコードされたものはありません
vdl2-nothing-matches-filter = フィルターに一致するものはありません
vdl2-stations = 局
vdl2-nobody-heard-yet = まだ誰も聞こえていません
vdl2-vdl2-setup = VDL2 の設定
vdl2-channels = チャンネル
vdl2-every-25-khz-slot-136 = 136.650 から 136.975 MHz までの 25 kHz ごとのスロットです。それぞれに 1 つのダウンコンバーターがあり、すべて同じ受信機の窓の中にあります。1 つをオフにすると、プロセッサーの時間がわずかに節約されます。ほかの受信感度が上がるわけではありません。
vdl2-burst-threshold = バーストのしきい値
vdl2-above-each-channel-s-own = 各チャンネル自身のノイズフロアより上
vdl2-lower-catches-weaker-transmissions-costs = 下げると、より弱い送信を捕まえられますが、ノイズでのプロセッサー時間が増えます。上げると、それらを取り逃します。フロアはチャンネルごとに学習され、ここでの変更がそれを捨てることはありません。
vdl2-keep-log = ログに保持
vdl2-track-most = 追跡する上限
vdl2-longest-silent-go-first = 最も長く沈黙している局から先に消えます
vdl2-forget-station-after = 局を忘れるまでの時間
vdl2-nothing-heard = その局から何も聞こえていない時間
vdl2-show-unread-payloads = 読み取れないペイロードを表示
vdl2-hex = 16 進で
vdl2-frames-carrying-x-25-clnp = X.25、CLNP、またはその上のデータリンクアプリケーションを運ぶフレームです。SDRoxide は、それらを読まずに名前を付けてバイトを表示します。隠すと、トラフィックのどれだけがそれなのかが隠れてしまうためです。
vdl2-registration = 登録記号
vdl2-flight = 便名
vdl2-label = ラベル
vdl2-block = ブロック
vdl2-sequence = シーケンス
vdl2-mode = モード
vdl2-more-blocks-follow = さらにブロックが続きます
vdl2-check = チェック
vdl2-message-crc-good = メッセージの CRC は正常
vdl2-message-crc-not-checked = メッセージの CRC は未確認
vdl2-parity = パリティ
vdl2-link-control-no-payload = リンク制御、ペイロードなし
vdl2-name = 名前
vdl2-addr = アドレス
vdl2-type = 種別
vdl2-msgs = メッセージ

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = 受信しているトーンと、送信するトーンです。CW では同じ周波数です。ウォーターフォールをクリックすると、信号の上に移動します。
cw-down-10-hz = 10 Hz 下げる
cw-up-10-hz = 10 Hz 上げる
cw-qrg-main-readout-tuning-line = QRG：メインの表示と同調線は、交信している周波数に合わせられています。クリックすると、代わりにダイヤルを表示します。その下に側音のピッチを加えた値で、多くの無線機が表示するものです。
cw-put-main-readout-tuning-line =
    メインの表示と同調線を、ダイヤルではなく信号に合わせます。そのため、表示される周波数は両方の運用者が言う値になり、同調線はパスバンドの中央に来ます。同調の仕方は変わりません。変わるのは数値だけです。

    Clicking a signal lands it on the cursor only as closely as the click step allows — Controls → click-to-tune rounding, 10 Hz by default. A coarse step leaves the signal off the pitch by up to half of it, and the readout will say so.
cw-sending-speed-read-off-signal = 信号から読み取られた送信速度
cw-signal-noise-500-hz-same = 500 Hz での信号対雑音比。レポートで引用される値と同じです
cw-how-far-off-cursor-signal = 信号が実際にカーソルからどれだけずれているか。デコーダーはそれに追従しますが、パスバンドは追従しないため、大きくなったらカーソルを微調整してください。
cw-listening = — 聞いています —
cw-tx = ● TX
cw-nothing-copied-yet = — まだ何も受信していません —
cw-key-send-characters-you-send = キーを押して送信します。送った文字がここに表示されます。
cw-type-line-return-sends = 1 行入力して、Return で送信…
cw-type-here-send = ここに入力して送信…
cw-key = {" "}キー ●{" "}
cw-key-2 = {" "}キー{" "}
cw-hold-key-bound-cw-straight = CW のストレートキーに割り当てられたキーを押したまま（既定ではスペース、設定 → コントロールで任意のキーに変更できます）、ストレートキーとして使います。押している間は下がり、離すと上がります。テキストを入力する代わりです。オンの間は送信ボックスがロックされ、キーヤー全体がキーに渡されます。待機していたテキストは破棄されます。

    To hand-key it, set CW keying to "Sound card (MCW)" in Settings → Radio. The rig is then held on a sideband and the keyer's own sidetone is transmitted as audio, which is the route the straight key drives.
cw-watchdog = ウォッチドッグ
cw-straight-key-was-held-down = ストレートキーが押し下げられたままになりすぎました。手ではなく、キーアップが失われたためです。そのためキャリアは落とされ、送信はオフになりました。続けるにはキーをもう一度押してください。
cw-tx-2 = {"  "}送信オン{"  "}
cw-tx-3 = {"   "}送信{"   "}
cw-send-what-box-now-without = Return を待たずに、ボックスにある内容を今送信します
cw-hold-key-down-between-characters = 文字の間もキーを押し続け、入力されたものが待たされないようにします
cw-call-cq = {" "}CQ を呼ぶ{" "}
cw-nocall = NOCALL
cw-clear = {" "}クリア{" "}
cw-stop-sending-drop-whatever-has = 送信を停止し、まだ送られていないものを破棄します。ストレートキーがオンの場合、キーした内容の読み戻しもそれとともに消えます。
cw-sidetone = 側音
cw-play-keyed-tone-through-computer = 送信とともに、キーされたトーンをこのコンピューターのスピーカーでも再生し、送っている内容を聞けるようにします。「サウンドカード (MCW)」では、トーンはリグにだけ出るため、これがないと無音で送信することになります。リグ自身のモニターや、電波を受信したコピーが同じ役目を果たしている場合はオフにしてください。
cw-hold-what-typed-until-return = 入力された内容を Return まで保持し、入力されるたびに文字をキーするのではなく、1 行をまとめて送信します。テキストからそれ自身をキーするトランシーバーで役立ちます。キーヤーへの引き渡しのたびに、送受信の 1 サイクルが増えるためです。
cw-cw-messages = CW のメッセージ
cw-clear-rx = {" "}RX をクリア{" "}
cw-empty-receive-window-straight-key = 受信ウィンドウと、ストレートキーの読み戻しを空にします。電波に出ているものは何も止まりません。
cw-lock = ロック
cw-decode-speed-set-here-instead = 信号から読み取る代わりに、ここで設定した速度でデコードします。速度探索では定まらないほど弱い信号に役立ちます。
cw-farnsworth-send-characters-full-speed = ファーンズワース：文字は全速で送り、文字の間の隙間だけを伸ばします。そのため、正しいリズムで聞こえますが、書き留められるほど遅く届きます。
cw-off-normal-spacing = オフ — 通常の間隔
cw-keying-speed = キーイングの速度
cw-idle-0 = 待機 0
cw-how-long-transmit-held-after = 最後の文字の後、またはストレートキーを離した後、キャリアが落ちるまで送信を保持する時間です。長いほど遅いタイピストの間を埋め、短いほど早く周波数から外れます。オフは、待機中のすべてが送られるとすぐに落とします。
cw-off-drop-once = オフ — 直ちに落とす

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = 10 Hz 下げる
textmodem-tune-up-10-hz = 10 Hz 上げる
textmodem-tx = ● TX
textmodem-listening = — 聞いています —
textmodem-type-line-return-sends = 1 行入力して、Return で送信…
textmodem-type-here-transmit = ここに入力して送信…
textmodem-tx-2 = {"  "}送信オン{"  "}
textmodem-tx-3 = {"   "}送信{"   "}
textmodem-call-cq = {" "}CQ を呼ぶ{" "}
textmodem-nocall = NOCALL
textmodem-clear = {" "}クリア{" "}
textmodem-hold-what-typed-until-return = 入力された内容を Return まで保持し、入力されるたびに文字を電波に流すのではなく、1 行をまとめて送信します。どれかが送られる前に、1 行を読み返して修正できます。
textmodem-messages = メッセージ
textmodem-hell = HELL
textmodem-contrast = コントラスト
textmodem-harder-softer-dots-redraws-whole = ドットを硬く、または柔らかくします。帯全体を描き直します
textmodem-width = 幅
textmodem-t-2row = {" "}2 行{" "}
textmodem-draw-every-column-twice-stacked = 各列を 2 回、上下に重ねて描きます。Hell には垂直同期がないため、位相に関係なく、テキストの完全な 1 つのコピーが読めるようになります。
textmodem-rev = {" "}反転{" "}
textmodem-reverse-video-light-dots-dark = 反転表示：暗い紙に明るいドット
textmodem-clear-rx = {" "}RX をクリア{" "}
textmodem-hold-channel-idle-sends-blank = チャンネルを保持します。待機中は白紙を送るため、帯はスクロールし続けます
textmodem-hold-what-typed-until-return-2 = 入力された内容を Return まで保持し、入力されるたびに各文字を帯に描くのではなく、1 行をまとめて送信します。ライブで入力された修正は、すでに紙の上にあります。Return の前に行った修正は、一度も紙に載っていません。
textmodem-shift = シフト
textmodem-shift-5-hz = シフト −5 Hz
textmodem-shift-5-hz-2 = シフト +5 Hz
textmodem-baud = ボーレート
textmodem-reverse-swap-mark-space-needed = 反転：マークとスペースを入れ替えます。送信された側波帯とは反対の側波帯で受信され、トーンが反転する場合に必要です。
textmodem-track-tuning-error-automatically-instead = カーソルに固定されたままにせず、同調誤差を自動で追跡します。
textmodem-tones = トーン
textmodem-mode = モード
textmodem-off-keyed = オン/オフキー

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = AIS デコーダーを開始中…
ais-tune-between-two-ais-channels = 2 つの AIS チャンネルの間に同調します。161.975（AIS 1、海上 87B）と 162.025（AIS 2、88B）です。そこには何も送信されないため、ダイヤルは中央に置かれます。そのためゼロ IF の受信機の DC スパイクは、どちらにも乗りません。
ais-setup = 設定
ais-channels-timeouts-trail-length-how = チャンネル、タイムアウト、航跡の長さ、ベクトルがどれだけ先まで届くか
ais-vessels = 船舶
ais-nothing-heard-yet = まだ何も聞こえていません
ais-virtual-nothing-there = 仮想 — そこには何もありません
ais-position-stale = 位置が古くなっています
ais-search-rescue-transmitter-man-overboard = 捜索救助用の送信機、人が落水したときのビーコン、または AIS EPIRB
ais-call-sign = 呼出符号
ais-station-time = 局の時刻
ais-dgnss = {" "}（DGNSS）
ais-first-heard = 初めて聞こえた時刻
ais-center = 中央
ais-put-vessel-middle-chart = この船舶を海図の中央に置きます
ais-last-message-station-nmea-sentences = この局からの最後のメッセージを、受信機が回線に流す NMEA の文として示します。他の AIS デコーダーに貼り付けると、sdroxide がそれをどう解釈したか確認できます。
ais-ais-setup = AIS の設定
ais-channels = チャンネル
ais-drop-chart-after = 海図から外すまでの時間
ais-without-position-report = 位置報告がない場合
ais-five-minutes-default-not-ads = 既定では 5 分です。ADS-B の 10 秒ではありません。停泊中の船舶は 3 分に 1 回報告するため、短い窓では、2 つの完全に正しい報告の間に港のほとんどが消えてしまいます。それを過ぎると、船は海図から外れ、その行はグレー表示になります。フェードではありません。古い位置にある薄い記号も、真の記号と同じインクで、船がどこにいるかという主張だからです。
ais-drop-list-after = 一覧から外すまでの時間
ais-nothing-heard-all = まったく何も聞こえていない場合
ais-message-carrying-ship-s-name = 船の名前を運ぶメッセージは 6 分ごとに回ってきます。そのため、短い一覧の窓では、船がその名前を名乗る直前に船舶を捨ててしまいます。
ais-trail-length = 航跡の長さ
ais-history-behind-each-target = 各目標の後ろの履歴
ais-minutes-rather-than-points-because = 点ではなく分で示します。AIS の報告頻度は 2 桁にわたるため、固定の点数では、フェリーでは 80 秒、停泊中のタンカーでは 2 時間分になり、同じように描かれてしまうためです。0 で航跡をオフにします。
ais-speed-vector = 速度ベクトル
ais-how-far-ahead-vector-reaches = ベクトルがどれだけ先まで届くか
ais-slot-threshold = スロットのしきい値
ais-above-channel-s-learned-noise = チャンネルの学習されたノイズフロアより上
ais-track-most = 追跡する上限
ais-vessels-2 = {" "}船舶
ais-ais-receive-only-here-always = AIS はここでは常に受信専用です。これは人命に関わる安全のサービスであり、そこに偽の船舶トラフィックを載せることは、免許が認めるものではありません。
ais-fill-my-grid-digimode-setup = 範囲と方位のために、デジタルモードの設定で自分のグリッドを入力してください。そうすると、海図の枠は、最も遠いものではなく、あなたのいる場所を中心に組まれます。
ais-name = 名前
ais-mmsi = MMSI
ais-type = 種類

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = この受信機は単独では 10489.750 MHz に届きません。先に LNB/コンバーターのオフセットを設定してください（設定 ▸ 無線機）
qo100-run-fast-spectral-tracker-every = 高速のスペクトルトラッカーを実行します。毎秒、待機窓の中でビーコンの左右対称な 2 つの葉を探し、キャリアがどこにあるかを報告します
qo100-telemetry = テレメトリー
qo100-also-run-ao-40-uncoded = AO-40 の符号化されていないフレームデコーダーも実行します。同期ワード、CRC、テレメトリーのテキストを扱い、各パスがどこまで進むかを、段階ごと（キャリア → 同期 → CRC）に表示します
qo100-auto = 自動
qo100-auto-correct-let-tracker-adjust = 自動補正：トラッカーに、コンバーター/LNB のオフセットを自分で調整させます。きれいで安定した推定の上で、LNB がずれるとビーコンを 10489.750 MHz に戻し、そこに保持する、ゆっくりした閉ループです。動作するたびにフロントエンドを開き直します。
qo100-narrower-search-smaller-slice = 狭く — より小さな範囲を探します
qo100-wider-when-beacon-isn-t = 広く — 現在の幅でビーコンが見つからない場合に
qo100-clear-mark = マークをクリア
qo100-drop-hand-placed-beacon-mark = 手で置いたビーコンのマークを外し、デコーダー自身の読み取りに戻します
qo100-lo = 下限 −
qo100-lo-2 = 下限 +
qo100-hi = 上限 −
qo100-hi-2 = 上限 +
qo100-radio-s-own-tuning-range = この無線機の同調範囲は 10489.750 MHz に届きません。このウィンドウが何かをするには、先に LNB/コンバーターのオフセット（設定 ▸ 無線機 ▸ コンバーター）が必要です。
qo100-open-settings-radio = 設定 ▸ 無線機を開く
qo100-tune-10489-750-mhz = 10489.750 MHz に同調
qo100-tip-if-you-can-see = ヒント：ストリップにビーコンが見えるのにロックしない場合は、それをダブルクリックしてください。10489.750 MHz としてマークされ、APPLY がそれに補正できるようになります
qo100-tracker = トラッカー
qo100-no-twin-lobe-shape-parking = 待機窓に、左右対称の 2 つの葉の形はありません
qo100-shape = 形
qo100-drift = ドリフト
qo100-need-longer-run-estimates = —（推定をもっと長く集める必要があります）
qo100-cycles = サイクル
qo100-receiver = 受信機
qo100-target = 目標
qo100-measured = 測定値
qo100-clicked = {"  "}（クリック）
qo100-tracker-2 = {"  "}（トラッカー）
qo100-not-locked-yet = まだロックしていません
qo100-converter-offset = コンバーターのオフセット
qo100-decode = デコード
qo100-decode-filling-first-frame-buffer = デコード — 最初のフレームのバッファーを埋めています…
qo100-decode-off-turn-telemetry-chip = デコード — オフ。キャリア → 同期 → CRC の表示のために、テレメトリーのチップ（上段）をオンにしてください。
qo100-apply-correction = {" "}補正を適用{" "}
qo100-write-converter-lnb-offset-puts = クリックした場所のビーコンを 10489.750 MHz に置くコンバーター/LNB のオフセットを書き込み、受信機を開き直します。設定 ▸ 無線機 ▸ 適用で起きるのと同じ短い中断です
qo100-write-converter-lnb-offset-puts-2 = トラッカーの推定値を 10489.750 MHz に置くコンバーター/LNB のオフセットを書き込み、受信機を開き直します。設定 ▸ 無線機 ▸ 適用で起きるのと同じ短い中断です
qo100-waiting-second-crc-valid-frame = これを書き込むことを提案する前に、CRC が正しい 2 つ目のフレームを待っています。1 回のロックだけでは、偶然の一致かもしれないためです
qo100-write-corrected-converter-lnb-offset = 補正されたコンバーター/LNB のオフセットを書き込み、受信機を開き直します。設定 ▸ 無線機 ▸ 適用で起きるのと同じ短い中断です
qo100-decoder-runs-receiving-station-readout = デコーダーは受信しているステーションで動作します。その表示はリモートのクライアントには送られません
qo100-starting = 開始中…
qo100-searching-first-window-fills-after = 検索中 — 最初の窓は約 24 秒後に埋まり、その後繰り返します

## js8 (src/app/panels/js8.rs)

js8-multi = マルチ
js8-decode-every-js8-speed-not = 送信している速度だけでなく、すべての JS8 の速度をデコードします。4 つの速度は同じサブバンドを共有し、4 つの異なる波形なので、これがないと別の速度の局は見えず、2 つの速度の間の交信はまったく起こりえません。受信の CPU は約 4 倍かかります。
js8-hb-auto = HB 自動
js8-turbo-does-not-beacon-local = Turbo はビーコンを送りません。ローカルと VHF の速度です
js8-beacon-your-callsign-grid-every = 15 分ごとに、コールサインとグリッドをビーコンで送ります
js8-until-next-heartbeat = 次のハートビートまで
js8-tx = ● TX
js8-heard = 聞こえた
js8-nothing-heard-yet = — まだ何も聞こえていません —
js8-dxcc = DXCC
js8-band = バンド
js8-grid = グリッド
js8-dupe = 重複
js8-reply = 応答
js8-no-messages = — メッセージなし —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}HB{" "}
js8-clear = {" "}宛先をクリア{" "}
js8-forget-selected-station-composer-goes = 選択した局を忘れます。作成欄は @ALLCALL に戻ります
js8-already-addressing-allcall = すでに @ALLCALL 宛てです
js8-stop = {" "}停止{" "}
js8-send = {" "}送信{" "}
js8-message = メッセージ…

## logbook (src/app/logbook.rs)

logbook-logbook = ログブック
logbook-new-entry = + 新規エントリー
logbook-adif = ADIF
logbook-import = 読み込み
logbook-import-qsos-adif-adi-adif = ADIF（.adi または .adif）ファイルから QSO を読み込みます
logbook-new-qso = 新規 QSO
logbook-edit-qso = QSO を編集
logbook-call = コール
logbook-lookup = 検索
logbook-look-up-name-qth-grid = 名前 / QTH / グリッドを検索
logbook-grid = グリッド
logbook-freq-mhz = 周波数 MHz
logbook-mode = モード
logbook-rst-sent = 送信 RST
logbook-rst-rcvd = 受信 RST
logbook-name = 名前
logbook-state = 州
logbook-country = 国
logbook-date-utc = 日付（UTC）
logbook-time = 時刻
logbook-pwr-w = 出力 W
logbook-contest = コンテスト
logbook-s-sent = 送信 S#
logbook-s-rcvd = 受信 S#
logbook-comment = コメント
logbook-save = {" "}保存{" "}
logbook-cancel = キャンセル
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = カード ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = このエントリーを削除
logbook-edit = 編集
logbook-upload-qso-configured-logs = この QSO を設定済みのログにアップロード
logbook-no-qsos-yet-run-ft8 = まだ QSO がありません。FT8/FT4 を実行するか、手動のエントリーを追加してください

## scanner (src/app/scanner.rs)

scanner-scanner = スキャナー
scanner-work-through-stored-memory-channels = 保存されたメモリーチャンネルを順に巡回します
scanner-work-through-slice-band-channel = チャンネルグリッド上で、バンドの一部を順に巡回します
scanner-stop = 停止
scanner-start = 開始
scanner-from = 開始
scanner-step = ステップ
scanner-mode = モード
scanner-range-empty-high-edge-has = その範囲は空です。上端は下端より少なくとも 1 チャンネル上である必要があります。
scanner-skip-while-holding-pass-over = 今後、そのチャンネルを通過するために保持している間はスキップします
scanner-skipping = スキップ中
scanner-clear = クリア
scanner-stop-passing-over-any-them = それらのどれも通過しないようにします
scanner-stop-passing-over-channel = このチャンネルを通過しないようにします
scanner-no-memory-channels-stored-yet = まだメモリーチャンネルが保存されていません。MEM ウィンドウで保存してください。
scanner-skip-channel-pass-over = チャンネルをスキップして通過します
scanner-skip = スキップ
scanner-look-all-channels-fall-inside = パナダプターが作られているのと同じ変換で、1 つの受信機の窓に収まるすべてのチャンネルを探し、何かが出ているものだけに同調します。そのため、1 つのバンドの一覧は、長さにかかわらず、チャンネルごとの整定時間ではなく、1 周につき 1 回の同調で済みます。スキャンは停止する前に、候補ごとにそこで聴いて判定するため、停止の条件は変わりません。ただし、スイープは受信機のフィルターではなく FFT を通して測定するため、何もないところで止まり始めた場合はしきい値を確認してください。
scanner-radio-hands-over-demodulated-audio = この無線機は復調された音声を渡し、検索すべき独自のスペクトラムを持たないため、そのメモリースキャンはどちらの場合もすべてのチャンネルを訪れます。
scanner-fast = 高速
scanner-read-list-off-spectrum-instead = すべてのチャンネルを訪れる代わりに、スペクトラムから一覧を読み取ります
scanner-folders = フォルダー
scanner-scan-every-folder-every-folder = すべてのフォルダー、およびこれ以降に作られるすべてのフォルダーをスキャンします。そのフォルダーだけをスキャンするには、代わりにフォルダーを選んでください。
scanner-unfiled = 未分類
scanner-nothing-filed-under-folders-you = 選んだフォルダーの下には、何も格納されていません。
scanner-stops = 停止条件
scanner-use-receiver-s-own-squelch = 受信機自身のスケルチを使うため、音声が開くまさにその場所でスキャンが止まります
scanner-squelch-off-scan-will-stop = スケルチがオフです。スキャンは、最初に調べたものでそのまま止まります
scanner-channel-power-signal-has-reach = スキャンを止めるために、信号が到達しなければならないチャンネルの電力です
scanner-listens = 聴く時間
scanner-how-long-stay-candidate-before = 候補を判定する前に、その候補にとどまる時間です。約 0.1 秒を下回ると、レベルメーターが整定せず、弱い信号を取り逃します
scanner-resumes = 再開
scanner-carry-once-signal-drops = 信号が下がったら続けます
scanner-carry-after-fixed-time-regardless = 固定の時間が経ったら、それに関係なく続けます
scanner-stay-until-you-press-next = 次を押すまでとどまります
scanner-how-long-stay = とどまる時間
scanner-how-long-wait-after-signal = 信号が下がった後に待つ時間
scanner-move-don-t-stop-channel = 次へ進み、このチャンネルでは再び止まりません
scanner-next = 次へ
scanner-move-now = 今すぐ次へ進みます

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = ADS-B デコーダーを開始中…
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = 1090 MHz Mode S
adsb-tune-receiver-1090-000-mhz = 受信機を、世界共通の ADS-B のダウンリンクである 1090.000 MHz に同調します。チャンネルは 1 つしかありません。
adsb-setup = 設定
adsb-timeouts-trail-length-how-far = タイムアウト、航跡の長さ、速度ベクトルがどれだけ先まで届くか
adsb-aircraft = 航空機
adsb-nothing-heard-yet = まだ何も聞こえていません
adsb-ground = 地上
adsb-position-stale = 位置が古くなっています
adsb-gnss-altitude = GNSS 高度
adsb-ground-speed = 対地速度
adsb-vertical-rate = 上昇率
adsb-first-heard = 初めて聞こえた時刻
adsb-center = 中央
adsb-put-aircraft-middle-map = この航空機を地図の中央に置きます
adsb-last-frame-accepted-aircraft = この航空機から最後に受け入れられたフレームです
adsb-ads-b-setup = ADS-B の設定
adsb-drop-map-after = 地図から外すまでの時間
adsb-without-position-report = 位置報告がない場合
adsb-past-aircraft-comes-off-map = これを過ぎると、航空機は地図から外れ、その行はグレー表示になります。フェードではありません。古い位置にある薄い四角も、真の記号と同じインクで、飛行機がどこにいるかという主張だからです。
adsb-drop-list-after = 一覧から外すまでの時間
adsb-nothing-heard-all = まったく何も聞こえていない場合
adsb-trail-length = 航跡の長さ
adsb-points = {" "}点
adsb-history-dots-behind-each-target = 各目標の後ろの履歴の点
adsb-speed-vector = 速度ベクトル
adsb-how-far-ahead-leader-line = 先導線がどれだけ先まで届くか
adsb-one-minute-usual-radar-convention = 1 分は通常のレーダーの慣例です。線はその時間に航空機が進む距離とちょうど同じ長さなので、ズームにかかわらず、等しい先導線は等しい速度になります。0 でオフになります。
adsb-track-most = 追跡する上限
adsb-aircraft-2 = {" "}機
adsb-aircraft-ground-are-placed-against = 地上の航空機は、ステーション自身の位置を基準に配置されます。地表スキッターには、それ自体には一意なデコードがないためです。近くの空港に何も表示されない場合は、デジタルモードの設定で自分のグリッドを入力してください。
adsb-call = コール
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = 受信機を待っています…
drm-not-drm-set-mode-drm = DRM ではありません。デジタルの短波放送で、モードを DRM に設定してください。ダイヤルはサイドバンドではなく、チャンネルの中心に合わせます。
drm-no-drm-signal-locked-decoder = DRM 信号はロックされていません。きれいなキャリアでも、デコーダーには数秒かかります。DRM は、読み取れるようになる前に、400 ms または 2 s にわたってインターリーブするためです。
drm-station-s-audio-codec-not = この局の音声コーデックは、ここではデコードできません。xHE-AAC には、システムに libfdk-aac が必要です。ライセンスがこのプログラムと両立しないため、組み込むことはできません。インストールし（Debian/Ubuntu：libfdk-aac2、Arch：libfdk-aac、macOS：brew install fdk-aac、Windows：sdroxide.exe の隣に libfdk-aac-2.dll）、再起動してください。この画面の他のすべては、正常にデコードされています。
drm-quality = 品質
drm-last-minute = 直近 1 分
drm-waiting-locked-signal = ロックされた信号を待っています
drm-audio-frames-decoding-but-station = 音声フレームをデコードしていますが、この局のコーデックはここではデコードできません
drm-audio-frames-decoding = 音声フレームをデコード中
drm-samples-reaching-decoder = デコーダーに届いているサンプル
drm-time = 時刻
drm-symbol-timing-recovered = シンボルのタイミングを回復しました
drm-frame = フレーム
drm-transmission-frames-found = 見つかった送信フレーム
drm-fast-access-channel-what-transmission = 高速アクセスチャンネル — 送信の内容
drm-service-description-channel-what-services = サービス記述チャンネル — サービスの内容
drm-audio = 音声
drm-constellation = コンステレーション
drm-waiting-decoded-symbols = デコードされたシンボルを待っています
drm-sampled-across-frame = フレーム全体にわたってサンプリング
drm-whole-frame = フレーム全体
drm-mode = モード
drm-robustness-mode-channel-width-ground = ロバストネスモードとチャンネル幅。A は地上波の経路用で、最も多くを運びます。D は激しく散乱する空間波の経路用で、最も少なく運びます。
drm-interleave = インターリーブ
drm-how-far-transmission-spreads-each = 送信が各フレームを時間的にどれだけ広げるかです。長くすると、より深いフェードに耐えられますが、その分、捕捉に時間がかかります。
drm-protection = 保護
drm-offset = オフセット
drm-residual-sample-clock-error-against = 送信機に対する残留サンプルクロック誤差です。大きく安定している場合は、放送ではなく受信機の基準がずれています。
drm-doppler = ドップラー
drm-delay = 遅延
drm-doppler-delay-spread-path-how = 経路のドップラーと遅延の広がりです。経路がどれだけ速く動いているか、そのエコーがどれだけ離れて届くかです。
drm-service = サービス
drm-decode-service-multiplex = マルチプレックスのこのサービスをデコードします

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = AtCHAT NET ステーションを開始中…
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET — 複数局のキーボードとファイル
atchat-net = {" "}NET に参加中{" "}
atchat-net-click-leave-state-kept = NET に参加中 — クリックで退出します（状態は保持されます。もう一度クリックすると再参加します）
atchat-rejoin = {" "}再参加{" "}
atchat-off-net-click-rejoin = NET から離脱中 — クリックで再参加します
atchat-off-net = NET 外
atchat-carrier = ● キャリア
atchat-another-station-transmitting-channel-busy = 別の局が送信中です。チャンネルは使用中です。
atchat-tx = ● TX
atchat-station-callsign-virtual-channel-address = ステーションのコールサインと仮想チャンネルのアドレス
atchat-virtual-channel = 仮想チャンネル
atchat-work-net-over-tcp-endpoint = 無線機の代わりに TCP のエンドポイント経由で NET を動かします。無線機のない開発用に、channel_server 互換のループバックです。オフにすると、ステーションが電波に出ます。
atchat-roster = 参加者一覧
atchat-nobody-heard-yet = まだ誰も聞こえていません
atchat-chat = {" "}チャット{" "}
atchat-common-channel-lines-here-go = 共通チャンネルです。ここの行は全員（ALL）に届きます
atchat-log = {" "}ログ{" "}
atchat-station-s-own-air-activity = ステーション自身の電波上の活動です。マスターの選出、参加者一覧の経過、ARQ の再試行
atchat-no-activity-logged-yet = まだ活動は記録されていません。
atchat-no-messages-yet = まだメッセージはありません。
atchat-message-everyone-all = 全員（ALL）へのメッセージ
atchat-send = {" "}送信{" "}
atchat-transfers = 転送
atchat-all = → 全員
atchat-sent-file-follows-active-chat = 送信したファイルは、アクティブなチャットのタブに従います
atchat-send-file = {" "}ファイルを送信{" "}
atchat-send-file-image-over-air = ファイルまたは画像を電波で送信します。ブロック CRC と ARQ を使用します
atchat-nothing-flight = 送信中のものはありません
atchat-images = 画像
atchat-no-images-received = 受信した画像はありません。


## rds (src/app/rds.rs)

rds-station = 局
rds-diagnostics = 診断
rds-which-programme-type-table-read = この局を、どの番組種別のテーブルで読むかです。RDS は北米以外のすべての場所で使われます。RBDS は、米国の局の識別コードがつづる呼出符号を加えます。自動では、局が送る拡張国コードに従います。それが届くまでは、呼出符号を綴りません。識別コードの範囲は、9 か国の RDS のコードとも共有されているためです。国コードを送らない局に名前を付けるには、RBDS を選んでください。
rds-waiting-receiver = 受信機を待っています…
rds-no-rds-station-not-every = この局に RDS はありません。すべての送信機が運ぶわけではなく、弱い送信機は、届かないまま運んでいる場合があります。
rds-rds-only-carried-wfm-broadcast = RDS は WFM の放送局でのみ運ばれます。
rds-holding = 保持中
rds-block-sync-has-dropped-last = ブロック同期が外れました。これは最後に読んだ局です。
rds-identity = 識別情報
rds-programme = 番組
rds-described = 説明
rds-content = 内容
rds-music = 音楽
rds-speech = 話し
rds-traffic = 交通
rds-announcement-air-now = 今、電波に出ている告知
rds-carried-station = この局が運んでいます
rds-not-carried = 運ばれていません
rds-radio-text = ラジオテキスト
rds-station-clock = 局の時計
rds-country-code = 国コード
rds-also = 同時放送
rds-also-tagged = 同時のタグ
rds-sync = 同期
rds-no-sync = 同期なし
rds-blocks-failed-their-check-plus = チェックに失敗したブロックと、誤り訂正器が修復しなければならなかったブロックです。訂正は、あえて誤りとして数えます。増えていく数値は、局がデコードをやめようとしている最初の兆候だからです。
rds-front-end-overloading = フロントエンドの過負荷
rds-converter-clipping-which-puts-distortion = コンバーターがクリップしており、マルチプレックス全体に歪みを生じさせています。57 kHz のデータサブキャリアも含みます。RF ゲインを下げるか、減衰器を入れてください。RDS は、何かが聞こえるより約 1 dB 手前で壊れ始めます。そのため、完璧に聞こえる局でも、そのデータが生き残るには強すぎることがあります。
rds-group-types = グループの種類
rds-recent-groups-newest-last = 最近のグループ — 最新が最後
rds-nothing-decoded-yet = まだデコードされたものはありません

## winlink (src/app/winlink.rs)

winlink-mail = メール
winlink-abort = 中止
winlink-stop-session-progress = 進行中のセッションを停止します
winlink-connect = 接続
winlink-compose = 作成
winlink-connecting = 接続中…
winlink-not-connected-yet = まだ接続していません
winlink-loading = 読み込み中…
winlink-date = 日付
winlink-from = 差出人
winlink-subject = 件名
winlink-no-subject = （件名なし）
winlink-delete-message = このメッセージを削除
winlink-arch = 保管
winlink-move-archive = 保管庫に移動
winlink-select-message = メッセージを選択
winlink-reply = 返信
winlink-delete = 削除
winlink-to = 宛先
winlink-cc = Cc
winlink-subject-2 = 件名
winlink-callsign-smtp-someone-example-org = コールサイン、またはインターネットメールの場合は SMTP:someone@example.org。複数の場合はカンマで区切ってください。
winlink-file-outbox = 送信箱に入れる
winlink-discard = 破棄
winlink-filed-messages-go-out-next = 格納されたメッセージは、次のセッションで送信されます
winlink-mail-log = メールログ
winlink-session-running = セッション実行中…
winlink-no-session-yet = まだセッションはありません

## wspr (src/app/panels/wspr.rs)

wspr-receptions = 受信
wspr-save-wspr-reception-list-csv = WSPR の受信一覧を CSV で保存
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = 微弱信号の伝搬ビーコン
wspr-time-left-two-minute-slot = この 2 分のスロットの残り時間です。終わると、録音がデコーダーに渡され、次の送信の判断が行われます。
wspr-tx = ● TX
wspr-decoding = デコード中…
wspr-band = バンド
wspr-last-slot = 直前のスロット
wspr-nothing-heard = 何も聞こえていません
wspr-receive-only = 受信のみ
wspr-next-slot = 次のスロット
wspr-waiting-radio-take-change = 無線機が変更を受け入れるのを待っています…
wspr-hop-next = 次へ移動
wspr-transmit = 送信
wspr-receive-only-2 = 受信のみ。
wspr-power = 出力
wspr-what-you-actually-radiate = 実際に放射する出力
wspr-goes-out-message-everyone-who = これはメッセージに含まれて送られ、あなたを聞いたすべての人がそれで経路を判断します。そのため、ここで楽観的な数値を入れると、相手の測定もあなたの測定も誤ります。
wspr-roam = ローミング
wspr-pick-different-offset-inside-200 = 送信のたびに、200 Hz の窓の中で別のオフセットを選びます。これが慣例です。全員が共有する 200 Hz は、誰もその中央に留まらない場合にだけ機能します。オフにすると、カーソルの位置に留まります。
wspr-band-hop = バンドホップ
wspr-move-dial-band-band-between = スロットの間にダイヤルをバンドからバンドへ動かすため、1 台の受信機がスペクトラムの一部ではなく全体をサンプリングできます。自分で VFO を回すと一時停止します。再開するにはこれを 2 回押してください。
wspr-upload = アップロード
wspr-send-what-station-decodes-wsprnet = このステーションがデコードした内容を wsprnet.org に送ります。聞こえたものを報告することで、WSPR 受信機はネットワークの一部になります。電波には何も出しません。
wspr-who-heard-me = 私を聞いた局
wspr-ask-wsprnet-org-every-few = 数分ごとに、どの局がこの局をデコードしたかを wsprnet.org に問い合わせます。WSPR には確認応答の仕組みがまったくないため、ビーコンが自分の到達範囲について知る方法は、これだけです。
wspr-bands = バンド
wspr-none-selected-hopping-has-nowhere = 何も選ばれていないため、ホップする先がありません
wspr-set-your-callsign-grid-general = 送信する前に、設定の全般タブでコールサインとグリッドを設定してください。

## packet (src/app/panels/packet.rs)

packet-monitor = モニター
packet-busy = 使用中
packet-frames-arrived-but-failed-their = 届いたものの、チェックシーケンスに失敗したフレームです。衝突、フェード、または読むには弱すぎる信号が原因です。
packet-station-callsign-speed-tx-delay = ステーションのコールサイン、速度、送信の遅延、デジピーターの経路、ビーコン、KISS サーバー
packet-starting-packet-modem = パケットモデムを開始中…
packet-nothing-heard-yet = まだ何も聞こえていません
packet-terminal = ターミナル
packet-connected = {" "}接続中{" "}
packet-calling = {" "}呼び出し中{" "}
packet-idle = {" "}待機{" "}
packet-extended-sequence-numbers-window-up = 拡張シーケンス番号。ウィンドウは最大 127 フレームです。
packet-frames-sent-not-yet-acknowledged = 送信したが、相手がまだ確認応答していないフレームです。
packet-retries-against-link-s-limit = このリンクの上限に対する再試行の回数です。確認応答されていないフレームが動かないまま数が増えていくのは、こちら側からは、フェードしている経路がそう見えるということです。
packet-mail-window-using-link = メールウィンドウがリンクを使用中です。
packet-one-radio-one-channel-one = 1 台の無線機、1 つのチャンネル、1 つのリンクです。Winlink のセッションとこのターミナルは、それを使う 2 つの方法で、先に要求したほうが使えます。
packet-digipeaters-nearest-first-separated-commas = デジピーターを、近いものから順に、カンマで区切って指定します（例：OE3XLR-1,OE3XMS-1）。直接聞こえる局の場合、またはパケット設定の既定の経路を使う場合は、空欄にしてください。
packet-disconnect = {" "}切断{" "}
packet-hang-up-properly-wait-far = きちんと切断し、相手が同意するのを待ちます。モードを変えると、誰にも伝えずにセッションが終わります。
packet-connect = {" "}接続{" "}
packet-mail-window-has-link-finish = メールウィンドウがリンクを持っています。先にそのセッションを終了するか停止してください。
packet-station-has-no-callsign-yet = このステーションにはまだコールサインがありません。モニターペインの上の 設定 の下で設定してください。設定されるまでは何も送信されません。
packet-call-station-connected-mode-node = この局を接続モードで呼び出します。ノード、BBS、または別の運用者です。
packet-needs-callsign-call = 呼び出すにはコールサインが必要です。
packet-clear = {" "}クリア{" "}
packet-empty-transcript-link-untouched-connected = 記録を空にします。リンクには触れません。接続中の局は接続されたままです。
packet-not-connected-type-callsign-above = 接続していません。上にコールサインを入力し、接続を押すと、ノードや BBS を呼び出せます。
packet-type-here = ここに入力
packet-not-connected = 接続していません
packet-send = {" "}送信{" "}
packet-send-line-node-bbs-reads = 行を送信します。ノードや BBS は一度に 1 行を読みます。

## fsq (src/app/panels/fsq.rs)

fsq-speed = 速度
fsq-call = コール
fsq-callsign-directed-fsqcall-messages-defaults = 指定型（FSQCALL）メッセージ用のコールサインです。既定では自分のコールサインになります
fsq-tx = ● TX
fsq-contacts = 交信相手
fsq-heard = 聞こえた
fsq-none = — なし —
fsq-allcall = ALLCALL
fsq-image = 画像
fsq-clear = クリア
fsq-forget-pictures-received-so-far = これまでに受信した画像を破棄します
fsq-send-image = 画像を送信…
fsq-right-click-remove = 右クリックで削除
fsq-remove-picture = この画像を削除
fsq-listening = — 聞いています —
fsq-message = メッセージ…
fsq-send = {" "}送信{" "}
fsq-call-cq = {" "}CQ を呼ぶ{" "}
fsq-heard-2 = {" "}? 聞こえた{" "}
fsq-clear-2 = {" "}クリア{" "}
fsq-fsq-contacts = FSQ の交信相手
fsq-add = 追加

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + タブ
publicsdr-open-receiver-new-radio-own = この受信機を、独自のタブで新しい無線機として開きます
publicsdr-point-radio-receiver-replacing-whatever = *この* 無線機を、現在どのインターフェースにあっても置き換えて、この受信機に向けます。無線機一台分の設定になるため、もう一度確認します
publicsdr-not-stated = 記載なし
publicsdr-replace = 置き換え
publicsdr-point-radio-you-are-receiver = 今使っている無線機を、この受信機に向けます
publicsdr-tab-instead = + タブで代わりに
publicsdr-open-receiver-another-radio-leave = 受信機を別の無線機として開き、この無線機はそのままにします
publicsdr-cancel = キャンセル
publicsdr-public-sdrs = 公開 SDR
publicsdr-available = 利用可能
publicsdr-hide-receivers-are-full-ones = 満員の受信機と、運用者がブラウザ以外のアプリ向けのチャンネルを開いていない受信機を隠します
publicsdr-band = 対象バンド
publicsdr-only-receivers-cover-current-dial = 現在のダイヤル周波数をカバーする受信機だけを表示します
publicsdr-low-bw = 低帯域
publicsdr-take-spyserver-low-bandwidth-shape = SpyServer を低帯域の形で受信します。ダイヤルに追従する狭い I/Q 窓と、サーバー自身のバンド表示を使い、広帯域の I/Q の数メガビットは使いません。KiwiSDR には影響しません。KiwiSDR の形はこの 1 つだけです。
publicsdr-refresh = ⟳ 更新
publicsdr-fetch-both-directories-again = 両方のディレクトリを再度取得します
publicsdr-name-place-antenna-band = 名前、場所、アンテナ、バンド
publicsdr-clear-search = 検索をクリア
publicsdr-fetching-receiver-lists = 受信機の一覧を取得中…
publicsdr-machine-radio-attached-does-not = この無線機が接続されているマシンはデバイスへの問い合わせに応答しないため、一覧も取得できません。
publicsdr-no-match = 一致なし
publicsdr-nothing-matches-try-turning-filter = 一致するものがありません。フィルターのチップを一つ戻してみてください
publicsdr-radio = この無線機
publicsdr-radio-s-configuration-has-not = この無線機の構成が、接続しているマシンからまだ届いていないため、受信機に向けるものがありません。代わりに + タブで受け取ってください。
publicsdr-position-not-published-so-nothing = {" "}その位置は公開されていないため、聞いたものは何も報告されません。

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = 受信機を待っています…
hd-not-hd-radio-set-mode = HD Radio ではありません。デジタルのサイドカーを運ぶ FM 放送で、モードを HD Radio に設定してください。ダイヤルは、隣ではなく、アナログのキャリアの中心に合わせます。
hd-no-hd-radio-lock-digital = HD Radio はロックされていません。デジタルのサイドバンドはアナログのキャリアより約 20 dB 低く送信されるため、聴ける局でも、デコードするにはノイズが多すぎることがあります。きれいな局でも、デコーダーには数秒かかります。
hd-sync = 同期
hd-ofdm-frame-timing-recovered = OFDM フレームのタイミングを回復しました
hd-audio = 音声
hd-audio-frames-decoding = 音声フレームをデコード中
hd-quality = 品質
hd-lower = 下側
hd-upper = 上側
hd-last-minute = 直近 1 分
hd-waiting-locked-signal = ロックされた信号を待っています
hd-mer-l = MER L
hd-modulation-error-ratio-lower-digital = 下側デジタルサイドバンドの変調誤差比です。
hd-mer-u = MER U
hd-modulation-error-ratio-upper-digital = 上側デジタルサイドバンドの変調誤差比です。
hd-cber = CBER
hd-channel-bit-error-ratio-after = 内側の符号の後のチャンネルのビット誤り率です。約 1e-3 より下では外側の符号が訂正できます。それを超えると、音声が途切れ始めます。
hd-offset = オフセット
hd-residual-carrier-frequency-offset-large = キャリア周波数の残留オフセットです。大きく安定している場合は、放送ではなく受信機の基準がずれています。
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = 一次サービスモード表示です。局が送信している FM ハイブリッドのサービスモードのどれかを示します。
hd-programme = 番組
hd-restricted-access = {" "}— アクセス制限あり

## bands (src/app/bands.rs)

bands-bands = バンド
bands-daylight = ☀ 昼
bands-night = ☾ 夜
bands-no-forecast-yet-fetching-hamqsl = · まだ予報はありません。HAMQSL.com から取得中
bands-band = バンド
bands-condx = 状況
bands-paths = 経路
bands-reach = 到達
bands-best = 最良
bands-ibp-beacons = IBP ビーコン
bands-meteor-showers = 流星群
bands-peak = ピーク

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = スポット
spots-longwave-shortwave-broadcast-stations-air = 今、電波に出ている長波と短波の放送局
spots-view = 表示中
spots-only-spots-inside-panadapter-span = パナダプターのスパン内にあるスポットだけ
spots-setup = ⚙ 設定
spots-feeds-lookup-upload-settings = フィード、検索、アップロードの設定
spots-call-station-site-frequency = コール、局、サイト、周波数
spots-clear-search = 検索をクリア
spots-no-match = 一致なし
spots-no-spots-enable-feed-setup = スポットがありません。⚙ 設定 でフィードを有効にしてください
spots-nothing-matches-search = 検索に一致するものはありません

## ism (src/app/ism.rs)

ism-ism-devices = ISM 機器
ism-decoding = デコード
ism-needs-wideband-iq-source-cat = 広帯域の IQ ソースが必要です。CAT のリグは音声しか送りません
ism-how-far-burst-must-stand = デコードされる前に、バーストがチャンネル自身のノイズフロアより、どれだけ上にある必要があるかです
ism-give-each-band-width-normally = 各バンドに、通常必要な幅を与えます。315、345、433 MHz の OOK バンドには 0.25 MHz、868 と 915 には完全な幅です。
ism-several-hundred-more-device-decoders = rtl_433 プロジェクトからの、数百の追加の機器デコーダーが組み込まれています。sdroxide 自身のデコーダーがまったく届かない 433 MHz の OOK 機器を読みます。両方が機器を読める場合は、rtl_433 の方が多くのバリエーションを知っているため、そちらが引き継ぎます。
ism-t-1-your-own = 1 件が自分のもの
ism-device-decoders-registered-how-many = 登録された機器デコーダーの数、そのうち rtl433_flex.conf から来たものの数、そして開始以降にデコードされた送信の数です。
ism-reload-decoders = デコーダーを再読み込み
ism-re-read-rtl433-flex-conf = 編集後に rtl433_flex.conf を読み直します。すでに聞こえた機器は一覧に残ります。
ism-rtl-433-knows-more-variants = rtl_433 は、sdroxide 自身のデコーダーよりこれらの多くのバリエーションを知っているため、ここで聴いている間はそれで読みます。別のバンドを選ぶと、sdroxide 自身のものが再び引き継ぎます。
ism-decoder-your-rtl433-flex-conf = rtl433_flex.conf のデコーダーの 1 つが拒否されました。残りは読み込まれています。その行を直してから、デコーダーを再読み込みを押してください。
ism-waiting-decoder = デコーダーを待っています…
ism-where-decoder-actually-listening-follows = デコーダーが実際に聴いている場所です。これは VFO ではなく、受信機のハードウェアの中心に従います。広帯域のフロントエンドでは両者は大きく離れることがあり、どのチャンネルに届くかを決めるのはこの窓です。
ism-transmissions-gate-opened-how-many = ゲートが開いた送信の数と、そのうち有効なフレームを生んだ数です。バーストが多いのにデコードがない場合は、sdroxide がまだ読めない機器でバンドが混み合っています。まったくない場合は、何もしきい値に届いていません。しきい値を下げるか、ダイヤルがバンドに合っているか確認してください。
ism-nothing-heard-yet = まだ何も聞こえていません
ism-click-tune-receiver =


    Click to tune the receiver to it

## app (src/app/mod.rs)

app-hpsdr = HPSDR
app-rtl-sdr = RTL-SDR
app-rtl-sdr-rtl-tcp = RTL-SDR（rtl_tcp）
app-spyserver-vfo = SpyServer（VFO）
app-airspy-hf = Airspy HF+
app-sdrplay = SDRplay
app-elad = ELAD
app-airspy = Airspy
app-sxceiver = SXceiver
app-usrp = USRP

## navtex (src/app/panels/navtex.rs)

navtex-messages = メッセージ
navtex-sync = 同期
navtex-character-phase-locked-signal-being = 文字の位相がロックされています。信号が読まれています。
navtex-characters-whose-first-copy-was = 最初の受信が壊れていて、5 スロット後の繰り返しから取られた文字と、両方が不良だった文字です。NAVTEX の文字は 2 回送られます。それがその誤り訂正のすべてです。
navtex-swap-mark-space-tones-signal = 反対の側波帯で受信された信号のために、マークとスペースのトーンを入れ替えます。オフは、チャンネル周波数の上側波帯です。これは、このサービスについて公開されているすべての同調の手順が示すものです。
navtex-starting-navtex-receiver = NAVTEX 受信機を開始中…
navtex-nothing-yet-station-transmits-ten = まだ何もありません。局は 4 時間ごとに 10 分間送信します
navtex-cut-short = 途中で打ち切られた
navtex-message = メッセージ
navtex-time-day-stated-message-utc = メッセージ内に記載された時刻（UTC）。表示のみです
navtex-receiving = 受信中
navtex-monitor = モニター
navtex-everything-decoded-message-not = メッセージかどうかに関係なく、デコードされたすべて

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = RF ペイント
rfpaint-scan-speed = スキャン速度
rfpaint-how-fast-text-image-scanned = テキストや画像をウォーターフォールにどれだけ速くスキャンするかです。低いほど遅く、読みやすくなります。100% は基本速度で、25%（中央）が既定値です。
rfpaint-abort = 中止
rfpaint-text-paint = テキストペイント
rfpaint-type-text-paint = ペイントするテキストを入力…
rfpaint-preview-waterfall = ウォーターフォールをプレビュー
rfpaint-type-text-preview = テキストを入力してプレビュー
rfpaint-transmit = {"  "}送信{"  "}
rfpaint-image-paint = 画像ペイント
rfpaint-no-image-loaded = 画像が読み込まれていません
rfpaint-load-image = 画像を読み込み…
rfpaint-load-image-preview = プレビューする画像を読み込んでください

## hfdl (src/app/hfdl.rs)

hfdl-listen = 受信
hfdl-switches-engine-s-hfdl-downconverter = エンジンの HFDL ダウンコンバーターとデコーダーをオンまたはオフに切り替えます。デコードには、窓が開いているかどうかに関係なく、24 kHz のレーンとワーカースレッドのコストがかかります。
hfdl-channel-assigned-frequency-listen-anything = 受信するチャンネル（割り当てられた周波数）です。ここを中心とする固定の 24 kHz のレーンに対して、バンド内のどれでも動作します。HFDL の局は 2.8〜22 MHz で送信します。
hfdl-save-hfdl-decode-log-file = HFDL のデコードログをファイルに保存
hfdl-riverhead-northern-atlantic-north-america = Riverhead（北大西洋 / 北米）です。既定値で、デコーダーが検証されたチャンネルです。
hfdl-hfdl-assigned-frequency = HFDL の割り当て周波数です。
hfdl-one-your-saved-channels-click = 保存したチャンネルの 1 つです。クリックで同調、右クリックで削除します。
hfdl-hfdl-selected-but-decoding-off = HFDL が選択されていますが、デコードはオフです。上の 受信 を押してください
hfdl-decodes = デコード
hfdl-nothing-decoded-yet-ground-station = まだデコードされたものはありません。チャンネルに入ると、地上局のスキッターは約 32 秒ごとに繰り返されます。
hfdl-no-decode-matches-filter = フィルターに一致するデコードはありません。
hfdl-decoding = デコード中
hfdl-decoding-off = デコードはオフ
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = FreeDV V1 デジタル音声
rade-tx = ● TX
rade-mute-analog = アナログをミュート
rade-mute-demodulated-audio-so-only = 復調された音声をミュートし、デコードされた音声だけが聞こえるようにします。モデムが再生するものがないときは、そうでなければ生の信号が通されます。そのヒスは、同期する前に交信を見つける手段なので、同調中はこれをオフにしておいてください。
rade-sync = 同期
rade-how-far-received-signal-sits = 受信信号が、モデムが予期している位置からどれだけずれているかです。値が大きくても、捕捉ループが追跡するためデコードはされますが、これをゼロに近づけるようにダイヤルを微調整すると、マージンが最も良くなります。
rade-snr = SNR —
rade-callsign-station-put-end-over =
    局がその End-of-Over フレームに入れたコールサインです。RADE の交信の中で、これが送られる唯一の点です。また、FreeDV Reporter にも報告されるため、聞いている局は、あなたが聞いたものとして qso.freedv.org に表示されます。

    Held for a minute after the signal goes, and dropped when you start an over of your own: it names the station on frequency, not every station the receiver has ever heard.
rade-stop-talking = 話すのを停止
rade-talk-ptt = 話す (PTT)
rade-open-close-rade-over-modem = RADE の交信を開始または終了します。モデムは、最初のフレームが出る前に約 120 ms の音声を必要とし、話し終えると終了フレームを送るため、送信はボタンより少し長く続きます。
rade-rade-v1-occupies-roughly-1060 = RADE V1 はパスバンドのおよそ 1060〜1880 Hz を占有します。上側波帯、または RADE が音声の練習に使い、sdroxide が切り替える 160、80、40 m では下側波帯です。ウォーターフォールの陰影が付いたバンド内に信号を置いてください。そこからはモデムが見つけます。

## frame (src/app/frame.rs)

frame-acknowledge = 確認
frame-dismiss = 閉じる
frame-reconnecting = 再接続中…
frame-not-connected = 接続されていません。
frame-reconnect-now = 今すぐ再接続
frame-transmit-lockout-disabled = ⚠ 送信ロックアウトは無効です
frame-engine-was-started-oob-tx = このエンジンは --oob-tx で起動されました。アマチュアバンドのロックアウトはオフです。ハードウェアがサポートする任意の周波数で送信機をキーします。
frame-transmitting-outside-your-licence-offence = 免許の範囲外で送信することは、免許を発行するすべての国で違法です。これからキーする周波数を使う許可がある場合にのみ続けてください。MARS/CAP または商用の免許、実験許可、またはダミーロードです。
frame-i-understand = {"  "}理解しました{"  "}
frame-restart-without-oob-tx-put = ロックアウトを元に戻すには、--oob-tx を付けずに再起動してください。

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC。送信機がどれだけ強く駆動されているかを、出力できる最大値に対する割合で示します。100 % はフルスケールであり、ワット数でのフルパワーではありません。アンテナでそれが何ワットに相当するかは、無線機と駆動の設定によります。
smeter-radio-reports-no-forward-power =


    This radio reports no forward power, so the figure beside ALC is the ALC reading again rather than watts.
smeter-no-swr-radio-has-no =


    No SWR: this radio has no bridge to measure it with, and sdroxide will not invent one — it can only show what the radio itself reports. A rig with an SWR meter (over CAT, TCI or a LAN link) and an HPSDR board both fill this in; a plain SDR transmitter cannot.
smeter-no-signal-report-yet-receiver = まだ信号レポートはありません。受信機がメーターの読み値をまだ送っていません。
smeter-s9-73-dbm-each-s =


    S9 is −73 dBm and each S-unit below it is 6 dB. The dBm figure is only as honest as the front end's calibration — an uncalibrated receiver reports dBFS with a dBm label.
smeter-ovld-converter-into-rails-so =


    OVLD: the converter is into its rails, so the level shown understates the signal. Wind the front-end gain back.
smeter-table-held = 、テーブルを保持
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● TX
multi-link = 電力
multi-mute-radio-s-audio = この無線機の音声をミュート
multi-close-radio-s-split-view = この無線機の分割表示を閉じます
multi-open-radio-split-view-own = この無線機を、独自の分割表示で開きます
multi-already-open-another-split-view = 別の分割表示ですでに開いています
multi-show-radio-here = この無線機をここに表示
multi-add-radio-computer-settings-radio = このコンピューターに無線機を追加します（ステーションに追加するには 設定 → 無線機）
multi-add-radio = 無線機を追加
multi-asked-station-another-radio = ステーションに別の無線機を依頼しました…
multi-client-cannot-open-connection = このクライアントは接続を開けません。

## net (src/app/net.rs)

net-fail = 失敗
net-downloading = ダウンロード中…
net-adif-import-failed-file-could = ADIF の読み込みに失敗しました。ファイルを解析できませんでした
net-channel-import-failed-file-could = チャンネルの読み込みに失敗しました。ファイルを読み取れませんでした
net-channel-import-nothing-read-chirp = チャンネルの読み込み：読み取るものがありません。CHIRP の CSV ファイルは、列の名前を示すヘッダー行で始まります。その列の 1 つは Frequency である必要があります

## widgets (src/app/panels/widgets.rs)

widgets-free-text = 自由テキスト
widgets-hashed-callsign-not-yet-heard = ハッシュ化されたコールサイン。まだ全体は聞こえていません
widgets-entity-unknown = エンティティ不明
widgets-band = このバンド
widgets-new-entity-never-worked-any = 新しいエンティティ — 交信したことがなく、どのバンドでも
widgets-new-grid-square = 新しいグリッドスクエア
widgets-not-worked-before = 以前に交信していません
widgets-worked-before-but-not-band = 以前に交信していますが、このバンドではありません
widgets-call-queue = 呼び出しキューにあります
widgets-image = 画像

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = 受信
pi4-save-pi4-reception-list-csv = PI4 の受信一覧を CSV で保存
pi4-next-generation-beacon = 次世代ビーコン
pi4-time-left-one-minute-beacon = この 1 分間のビーコンサイクルの残り時間です。PI4 のメッセージは、窓がそこで終わるはずだった位置を過ぎて埋まる時間ができてから検索されます。
pi4-decoding = デコード中…
pi4-band = バンド
pi4-last-cycle = 直前のサイクル
pi4-nothing-heard = 何も聞こえていません
pi4-receive-only-tune-so-beacon = 受信のみ。ビーコンの CW 識別が 800 Hz の音声に来るように同調してください。これはネットワーク自身の慣例です。4 つの PI4 のトーンは、このデコーダーが既定で探す位置に収まります。

## awards (src/app/awards.rs)

awards-awards = アワード
awards-band = バンド
awards-all = すべて
awards-dxcc = DXCC
awards-grids = グリッド
awards-worked-all-states = Worked All States
awards-cq-zones-waz = CQ ゾーン（WAZ）
awards-dxcc-entities = DXCC エンティティ

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = デコーダーのパスバンド内の音声です。
acars-blocks-framed-blocks-whose-check = フレーム化されたブロックと、チェックシーケンスが失敗したブロックです。
acars-starting-acars-receiver = ACARS 受信機を開始中…
acars-channel = チャンネル
acars-tune-dial-here = ここでダイヤルを同調してください。

acars-channel-out-of-range = この無線機の受信範囲外です。
acars-nothing-yet-acars-quiet-between = まだ何もありません。ACARS は航空機と航空機の間は静かです。チャンネルに合わせたままバーストを待ってください。
acars-check-failed = チェック失敗
acars-block-check-sequence-did-not = ブロックチェックのシーケンスが一致しませんでした。テキストが誤っている可能性があります。
acars-no-text = （テキストなし）

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = ユーザー名
login-password = パスワード
login-remember-device = このデバイスで記憶する
login-signs-without-asking-next-time = 次回は尋ねずにサインインし、3D ビューのタブも尋ねずに開きます。パスワードは平文で保存されるため、他の人も使うマシンではオフにしてください。
login-keeps-password-device-clear = このデバイスにパスワードを平文で保持します。
login-checking = {" "}確認中…{" "}
login-sign = {" "}サインイン{" "}
login-wrong-password-shuts-server-s = 間違ったパスワードは、サーバーの扉を数秒間閉ざすため、少し時間がかかることがあります。

## help (src/help.rs)

help-find = 検索
help-find-manual = マニュアル内を検索
help-previous-match-shift-enter-shift = 前の一致（Shift+Enter、Shift+F3）
help-next-match-enter-f3 = 次の一致（Enter、F3）
help-no-matches = 一致なし
help-clear-search-esc = 検索をクリア（Esc）
help-sdroxide-manual = SDROXIDE マニュアル
help-contents = 目次

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = 1 局が聞こえています。位置を報告するのを待っています
aismap-double-click-reframe = ダブルクリックで枠を合わせ直します
aismap-zoom-out-wheel-pinch-do = 縮小。ホイールやピンチでも同じことができます
aismap-zoom-about-middle-chart = 海図の中央を中心に拡大します
aismap-frame-everything-being-tracked-again = 追跡中のすべてを再度枠に収めます。ダブルクリックでも同じです

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = スペクトラムを待っています…
specview-hounds = HOUNDS

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = このウィンドウには GPU のレンダラーがないため、ウォーターフォールは標準の幅のままです。
spectrum-machine-drawing-without-gpu-every = このマシンは GPU なしで描画しています。ウォーターフォールのすべての列はプロセッサーの仕事であり、それは無線機とすでに分け合っています。
spectrum-window-renders-through-opengl-which = このウィンドウは OpenGL で描画します。これは sdroxide の互換性のための経路です。Raspberry Pi、古いグラフィックチップ、または WebGPU のないブラウザがこれにあたります。ここではフレームレートを犠牲にしてまで、幅の広いウォーターフォールにする価値はありません。
spectrum-wider-than-renderer-can-draw = このレンダラーが描ける幅より広いです。

## macros (src/app/panels/macros.rs)

macros-nocall = NOCALL
macros-remove-button = このボタンを削除

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = グリッドトラッカー
gridtracker-drag-pan-wheel-zoom-amber = ドラッグで移動、ホイールでズームします。琥珀色 = 交信済み、緑 = 確認済み、シアン = 聞こえた。

## remote (src/remote.rs)

remote-server-busy-another-client-connected = サーバーは使用中です。別のクライアントが接続しています
remote-transmitting-but-client-s-microphone = 送信していますが、このクライアントのマイクは音声をまったく生成していません。何も変調されていません。ブラウザでは、ページにマイクの許可があるか、他のアプリケーションがマイクを占有していないかを確認してください。
remote-connection-closed = 接続が閉じられました

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = 太陽系 3D ビュー — 太陽、地球、月、黒点、CME（別ウィンドウ）
solarbtn-solar-system-3d-view-sun-2 = 太陽系 3D ビュー — 太陽、地球、月、黒点、CME（新しいブラウザタブ）

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = 音声を読み込んでいます…
speech-browser-client-cannot-speak-yet = ブラウザのクライアントはまだ話せません

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = ダブルクリックで周波数を入力
freqdisplay-tap-type-frequency = タップして周波数を入力

## solarstate (src/solar3d/state.rs)

solarstate-home = ホーム
solarstate-dwarf-planets = 準惑星

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = ダブルクリックで枠を合わせ直します

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = ダブルクリックで枠を合わせ直します

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = 接続が閉じられました

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = アラートのスレッドを開始できませんでした

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = まだ位置の特定できた航空機はありません

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = ダブルクリックで枠を合わせ直します

## chrome (src/chrome.rs)



## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = FDM-DUO 自身の電源設定（メニュー 34 TX POWER）を設定します。その中で、駆動と同調が出力を設定し、SDRoxide は接続するたびにその設定を再送します。

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $month }月{ $day }日

## download (src/download.rs)

download-browser-could-not-read-file = ブラウザはそのファイルを読み取れませんでした

## morse (src/app/morse.rs)

morse-translate = 翻訳
morse-practice = 練習
morse-learn = 学習
morse-morse = モールス
morse-text-morse = テキスト → モールス
morse-morse-text = モールス → テキスト
morse-per-character-space-between-characters = 文字ごとに `.` と `-`、文字の間にスペース、単語の間に ` / `。ITU のアルファベット、数字、句読点、一般的なプロサインです。
morse-reference-translator-available-desktop-build = 参照用の翻訳機能は、デスクトップ版で利用できます。
morse-play-what-you-type-your = 入力した内容を、自分のスピーカーで再生します
morse-play = 再生
morse-stop = 停止
morse-every-character-learned-well-done = すべての文字を覚えました。よくできました。
morse-reveal = 表示
morse-reset = リセット
morse-replay = 再生し直す
morse-what-did-you-hear = 何が聞こえましたか？
morse-check = 確認
morse-press-new-hear-character = 「新規」を押すと文字が聞こえます。
morse-tone = トーン
morse-speed = 速度
morse-spacing = 間隔
morse-characters-go-out-speed-above = 文字は上の速度で送られます。これは *間隔* が送られる速度なので、初心者は、その速さで送れるようになる前に、文字全体を聞き取れます。
morse-playback-available-desktop-build = 再生は、デスクトップ版で利用できます。

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = 高度
adsb-row-track = 進路
adsb-row-turning = 旋回
adsb-row-squawk = スクォーク
adsb-row-range = 距離
adsb-row-position = 位置
adsb-row-signal = 信号
adsb-row-frames = フレーム
rade-transmitting = 送信中
decodes-odd = 奇数
decodes-new = 新規
js8-new = 新規

## AIS panel: vessel detail rows.

ais-row-status = 状態
ais-row-speed = 速度
ais-row-course = 針路
ais-row-heading = 船首方位
ais-row-turning = 旋回
ais-row-altitude = 高度
ais-row-size = 大きさ
ais-row-draught = 喫水
ais-row-destination = 目的地
ais-row-range = 距離
ais-row-signal = 信号
ais-row-type = 種類
ais-row-position = 位置
ais-row-messages = メッセージ
ism-off = オフ
textmodem-frequency-shifted = 周波数偏移
winlink-to-heading = 宛先

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = { $heard } 件を受信 — { $drop_map_s } 秒より新しい位置はありません
aismap-stations-heard-waiting-position-report = { $heard } 局を受信 — 位置報告を待っています
aismap-km-across = { $lon_span }° · 横幅 約 { $km } km

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = { $device } で警報中
alerts-alarm-device-unavailable = 警報デバイスが利用できません：{ $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = 交信 { $w }
awards-confirmed = 確認 { $value }

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · HAMQSL.com の予報、{ $value } 経過
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · 次のビーコンまで { $now } 秒
bands-radiants-placed = 輻射点を緯度 { $lat }°、経度 { $lon }° に配置
bands-zhr = ZHR { $zhr }
bands-radiant = 輻射点 { $alt_deg }° { $az_deg }
bands-radiant-down = 輻射点は地平線の下（{ $alt_deg }°）

    A radiant above the horizon means the trails can reach you; a fast shower leaves longer-lived ionised trails for meteor scatter on 6 m and 2 m, and the brief 10 m openings.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } はデコードされていません
drm-symbols = { $value } シンボル · { $frame }
drm-not-decodable = { $value } — デコードできません
drm-broadcaster-s-clock-utc = 放送局の時計 { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    MAX：FDM-DUO の最大出力で、約 8 W です。HR500 を回路に入れている場合に使う設定です。

    { $base }
duopower-t-5-w-hr500-line-will =
    5 W。HR500 を回路に入れている場合、この設定ではほとんどのバンドで最大出力に達しません。MAX を使い、駆動で下げてください。

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } W。アンプなしの運用や QRP 用です。

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = { $swr }:1 の後に送信を再び有効にします。まずアンテナを確認してください。障害が残っている場合、次の送信も停止します。
frame-reconnecting-s = { $value } 秒後に再接続します…

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = 交信 { $worked }
gridtracker-confirmed = 確認 { $confirmed }
gridtracker-heard = 受信 { $heard }

## hd: messages with values (src/app/hd.rs)

hd-decode = { $value } をデコード（{ $codec_label }）

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } kHz にはすでにチップがあります
hfdl-save-current-channel-khz-one = 現在のチャンネル（{ $cur_khz } kHz）を、自分のチップの 1 つとして保存します
hfdl-aircraft = 航空機 { $aircraft }
hfdl-bursts = バースト { $bursts }
hfdl-decodes-2 = デコード { $decodes }
hfdl-fixed = 修正 { $fec }

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint } — まだデコードされていません
ism-listen-tunes-radio-there-one = { $value } で受信 — そこに無線機を同調させます。一度に 1 つのバンドです。これらは離れすぎているため、どの受信機も 2 つを同時にカバーできません。
ism-watch-hz-around-band-centre = バンドが何を要求していても、バンド中心の周囲 { $value }Hz を監視します。狭いと、バンド自身の幅を出せない受信機に合います。広いと、中心から離れた位置にある機器まで届きます。受信機は、これより約 3 分の 1 多く出力する必要があります。
ism-rtl-433 = rtl_433 — { $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } kHz
ism-your-own = 自分のもの { $flex } 件
ism-decoders-decoded = デコーダー { $decoders } 件（{ $flex }）、デコード済み { $decodes }
ism-handled-rtl-433 = rtl_433 が処理：{ $names }
ism-flex = flex：{ $error }
ism-window-mhz-mhz-wide = 窓 { $mhz } MHz、幅 { $e62 } MHz
ism-bursts-decoded = バースト { $bursts }、デコード { $decodes }
ism-devices = 機器 { $ism_reports } 件
ism-id-mhz-db-heard-first =
    { $protocol } ID { $device }
    { $e6 } MHz { $snr_db } dB 受信 { $value }×
    初めて聞こえたのは { $first_at } 前

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = 確認：{ $recs } 件をダウンロード、新たに { $matched } 件が確認されました
logbook-qso = { $qso_log } QSO
logbook-worked-before = ⚠ 以前に交信済み（{ $dupe_band }）
logbook-utc-qso = { $oldest }〜{ $newest } UTC · QSO { $value } 件

## app: messages with values (src/app/mod.rs)

app-radio = 無線機 { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = モールスの音声スレッドを開始できませんでした：{ $error }
morse-no-audio-output = 音声出力がありません：{ $error }
morse-about-s = 約 { $farnsworth_wpm } 秒
morse-characters-play = 出題中の文字：{ $set }
morse-score-run = スコア：{ $correct } / { $total }。連続：{ $value } のうち { $streak }。
morse-correct = 正解 — { $value }
morse-was = 正解は { $value } でした

## net: messages with values (src/app/net.rs)

net-adif-import-failed = ADIF の読み込みに失敗しました：{ $error }
net-not-unicode-read = {" "}（Unicode ではありません。{ $enc } として読み取りました）
net-received-reports-swl-left-out = 、受信レポート（SWL）{ $n } 件は除外
net-adif-import-added-duplicates-skipped = ADIF の読み込み：追加 { $added } 件、重複をスキップ { $skipped } 件{ $reports }{ $assumed }
net-channel-import-failed = チャンネルの読み込みに失敗しました：{ $error }
net-line-s-skipped = 、{ $n } 行をスキップ
net-not-unicode-read-2 = {" "}（Unicode ではありません。{ $enc } として読み取りました）
net-channel-import-channel-s-read = チャンネルの読み込み：{ $n } チャンネルを読み取りました{ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = フレーム { $frames } · 不良 { $bad }
acars-mode = モード { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = 航空機 { $aircraft } 機
adsb-frames = フレーム { $frames }
adsb-preambles = プリアンブル { $preambles }
adsb-bad-crc = 不良な CRC { $bad_crc }
adsb-tune = 同調 { $mhz }
adsb-ft-barometric = 気圧高度 { $ft } ft

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = 船舶 { $vessels } 隻
ais-messages = メッセージ { $messages }
ais-slots = スロット { $bursts }
ais-bad-fcs = 不良な FCS { $bad_fcs }
ais-ais-mhz = AIS { $value }（{ $mhz } MHz）— { $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value }（{ $mhz } MHz）— スロット { $bursts }、メッセージ { $messages }、フロア { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = 同調 { $mhz }
ais-ships-heard-are-khz-off = 聞こえている船の周波数は { $khz } kHz ずれています（162 MHz で { $mhz } ppm）。フロントエンドの周波数補正を設定してください。さもないと、約 5 kHz を超えるずれの送信はデコードされなくなります
ais-over-ground = 対地 { $value }°
ais-dbfs-db-snr = { $rssi_dbfs } dBFS、SNR { $snr_db } dB
ais-last-type-ais = { $messages }（最後の種類 { $last_type }、AIS { $channel }）
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } MHz — 海上チャンネル { $ch }。船は 2 つを交互に使うため、片方をオフにすると、すべての船が聞こえる頻度が半分になります。

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = { $reported_by } からのオブジェクト
aprs-via = 経由 { $via }
aprs-object-reported = オブジェクトを報告したのは { $reported_by }
aprs-digit-s-blanked = ± { $ambiguity } 桁を空欄
aprs-wind-m-s = 風 { $value }° { $value2 } m/s
aprs-gust = 突風 { $value }
aprs-rain-mm-h = 雨 { $value } mm/h
aprs-frame-s-last-ago = フレーム { $packets } 件、最後は { $last_heard } 前
aprs-via-2 = 経由 { $via }
aprs-bad = 不良 { $bad_frames }
aprs-other = その他 { $non_aprs }
aprs-sent = 送信 ×{ $tries }
aprs-stn = 局 { $stations }
aprs-queued = 待機 { $tx_queue }

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = マスター { $value }
atchat-last-heard-s-ago-click = { $status } — 最後に聞こえたのは { $age_s } 秒前 · クリックで個人メッセージのタブを開きます
atchat-close-tab = { $peer } のタブを閉じる
atchat-direct-message = { $value } への個人メッセージ
atchat-cannot-display = { $path } を表示できません

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Click for the { $neural } decoder. The neural one copies further down and reads hand-sent CW a timing fit will not accept; the timing one is the only one that copies the accented letters — Ä, Ö, Å, Ü, É — because the model has no output class for them.
cw-idle-s = 待機 { $idle } 秒

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = 一覧にある { $n } 件のデコードを ADIF で保存します。受信レポートであり、交信ではありません。送信者を示さないデコードは除外されます。
decodes-save-decodes-list-csv-one = 一覧にある { $n } 件のデコードを、1 件につき 1 行の CSV で保存します
decodes-session-qso = セッション：QSO { $session } 件
decodes-save-whole-logbook-qso-adif = ログブック全体（QSO { $logged } 件）を ADIF で保存します
decodes-save-whole-logbook-qso-text = ログブック全体（QSO { $logged } 件）をテキストで保存します
decodes-hz-slots = { $audio_hz } Hz · { $odd } スロット
decodes-your-slot-timing-against-stations =
    聞いている局に対するあなたのスロットのタイミングです。
    { $after }

    It covers the whole receive path, so a slow audio or network chain counts the same as a wrong clock. Under 0.5 s is comfortable.
decodes-pile-up = パイルアップ { $fox_queue }
decodes-queue = キュー { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } dB · { $audio_hz } Hz{ $unwrap_or_default }
    クリックで削除

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = すべての速度をデコードしています。▸ は { $speed } を示します。これは送信している速度です
js8-every-speed-being-decoded-click = すべての速度をデコードしています。代わりに { $speed } で送信するにはクリックしてください
js8-transmit-decode = { $speed } で送信とデコード
js8-beaconing-every-min-click-stop = { $hb_min } 分ごとにビーコン送信中 — クリックで停止
js8-last-beacon-went-out-hz = 最後のビーコンは { $hz } Hz で送信されました。ハートビートのサブバンド { $value }〜{ $value2 } Hz の空いたスロットで、デコードされている信号から離れた位置になるように選ばれています。
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } フレーム · 残り { $left } 秒
js8-decoded = { $speed } でデコード
js8-frames = …（{ $frames } フレーム）
js8-reply-2 = { $value } への返信：「{ $value2 }」
js8-address-composer = 作成欄の宛先を { $value } にします

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i }：「{ $text }」を送信
macros-sends = 「{ $text }」を送信
macros-most = { $value } が上限です

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = 修復 { $repaired } · 失われた { $lost }
navtex-characters-lost = { $lost } 文字が失われました

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = 不良 { $bad_frames }
packet-link-layer = リンク層は { $link } にあります
packet-via = {" "}経由 { $via }
packet-unacked = 未確認 { $unacked }
packet-retry = 再試行 { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] ビーコン
       *[other] ビーコン
    }
pi4-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } 前

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = SNR { $snr_db } dB
rade-offset-hz = オフセット { $freq_offset_hz } Hz
rade-heard = 受信 { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ 受信機とデコーダーの間で { $dropped } サンプルが失われました。このマシンはニューラルデコードに追いついていません。

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } 文字/秒
textmodem-char-s-hz-wide = { $value } — { $chars_per_sec } 文字/秒、幅 { $bandwidth_hz } Hz、{ $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = フレーム { $frames }
vdl2-bursts = バースト { $bursts }
vdl2-sync = 同期 { $syncs }
vdl2-hdlc-bad = HDLC 不良 { $hdlc_bad }
vdl2-bad-fcs = 不良な FCS { $fcs_bad }
vdl2-mhz-bursts-frames-noise-floor = { $mhz } MHz、{ $what } — バースト { $bursts }、フレーム { $frames }、ノイズフロア { $floor_dbfs } dBFS
vdl2-tune = 同調 { $mhz }
vdl2-held-2 = 保持 +{ $since }
vdl2-use-worldwide-where-every-link = { $what } — 世界中で使われ、すべてのリンクが始まる場所です。1 つだけ残すなら、残すべきものです。
vdl2-assigned-which-channels-carry-anything = { $what } に割り当てられています。どのチャンネルが何を運ぶかは、あなたがいる場所によります。特に知らない限り、すべてオンのままにしてください。
vdl2-mhz-db-snr-dbfs-evm = { $mhz } MHz · SNR { $snr_db } dB · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Hz · 修正された RS シンボル { $rs_corrected } 個
vdl2-characters = { $parity_errors } 文字
vdl2-parameter-sdroxide-does-not-read = { $unknown } 個のパラメーター。SDRoxide は読み取りません。上に識別子とバイトとして表示されています

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = { $live_size } 行を保持
wefax-pixels-per-line = 1 行あたり { $width } ピクセル
wefax-shift-picture-pixels = 画像を { $px } ピクセルずらします
wefax-charts-are-saved-click-copy =
    チャートは { $place } に保存されます

    Click to copy the path
wefax-finished-charts-are-saved-collect = 完了したチャートは { $place } に保存され、ここに集まります。
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } ピクセル
    { $value }

    Right-click to delete
wefax-older-store = ストア内の古いものが { $older } 件あります
wefax-older-load-more = 古いものが { $older } 件 — さらに読み込む
wefax-published-carrier-khz-dial-khz = 公表されたキャリア { $f } kHz → ダイヤル { $value } kHz USB
wefax-delete-chart-2 = このチャートを { $dir } から削除

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · CQ ゾーン { $cq_zone } · ITU ゾーン { $itu_zone }
widgets-new-entity = { $band_label } で新しいエンティティ
widgets-worked-before = { $band_label } では以前に交信済み
widgets-calling-cq-includes-you = CQ { $target } を呼んでいます — あなたを含みます
widgets-calling-cq-not-aimed-you = CQ { $target } を呼んでいます — あなたを対象としていません

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = 受信 { $heard_us } · 私を聞いた { $heard_us2 }
wspr-beacon = { $last_slot_spots } { $last_slot_spots ->
        [one] ビーコン
       *[other] ビーコン
    }
wspr-beaconing-duty = ビーコン送信中 — { $wspr_power_dbm }、デューティ { $wspr_tx_percent }%
wspr-listening-duty = 受信中（デューティ { $wspr_tx_percent }%）
wspr-beacon-one-two-minute-slot = { $pct } ごとの 2 分のスロットのうち 1 つでビーコンを送ります。これは { $pct2 }% で、均等に間隔を空けます。出力は { $wspr_power_dbm } です。サイクルがどのスロットで始まるかはコールサインから決まるため、このプログラムを動かす 2 つのステーションが同時に始まることはありません。
wspr-transmitting-set-general-tab-settings = { $sent } で { $call } として送信中です。設定の全般タブで設定されています。
wspr-utc-ago = { $t }:{ $t2 } UTC — { $slot_utc } 前

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    アンテナ：{ $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = 受信機 { $entries } 件 · { $per_net } · 取得 { $age } 前
publicsdr-showing-search-narrow = { $n } 件中 { $shown } 件を表示中 — 検索で絞り込めます
publicsdr-match = { $n } 件一致
publicsdr-replace-taking-receiver-points-same =
    { $this } を { $entry } に置き換えますか？

    { $this2 } is on { $iface }. Taking this receiver points the same tab at { $address } instead, renames it “{ $entry2 }”, and holds the dial to the { $range_label } the receiver publishes — nothing here transmits.

    The { $iface2 } settings stay where they are, and so do the ranges stated for them: switching the interface back in Settings → Radio brings the radio back as it was.
publicsdr-receptions-will-reported = {" "}受信は { $value } から報告されます。
publicsdr-pointing-radio = この無線機を { $entry } に向けています…{ $site }
publicsdr-opening-another-radio = { $entry } を別の無線機として開いています…

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } kHz
qo100-before-switching-tune-so-beacon = オンにする前に、ビーコンの 2 つの葉がストリップの +{ $park_lo_hz } と +{ $park_hi_hz } kHz の間（陰影のあるレーン）に来るように同調してください。
qo100-receiver-currently-listening-around-mhz = 受信機は現在 { $mhz } MHz 付近を聴いており、10489.750 MHz のビーコンからは遠く離れているため、ここに描くものはありません。デコーダーは、それに関係なく探し続けます。
qo100-receiver-currently-covers-only-khz = 受信機は現在ここで ±{ $khz } kHz しかカバーしていません。ストリップの残りは空白のままです
qo100-null-db-sym-snr-db = ヌル { $est_null_depth_db } dB、対称 { $est_symmetry }、SNR { $est_snr_db } dB
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Hz/s、{ $est_drift_accel_hz_s2 } Hz/s²（デコード前に回転を戻します）
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Hz/s（デコード前に回転を戻します）
qo100-found-empty = 検出 { $est_updates } 件、空 { $est_misses } 件
qo100-last-s-ago = {"   "}最後 { $auto_last_hz }（{ $auto_last_unix } 秒前）
qo100-over-correction = { $auto_applies } 回の補正で合計 { $auto_total_hz }{ $last }
qo100-sync-32-err = 同期 { $sync_bit_errors } / 32 誤り ×{ $sync_matches }
qo100-frame-buffer = フレームバッファー { $frame_fill }%
qo100-blocks-tried-decoded = 試行ブロック { $blocks_tried } 件、デコード { $blocks_locked } 件
qo100-last-applied-s-ago-hz = 最後に適用 { $ago } 秒前：{ $old } → { $new } Hz（{ $old2 } Hz）
qo100-locked-block-tried-locked = ロック済み — 試行 { $blocks_tried } ブロック、ロック { $blocks_locked } 件
qo100-searching-block-tried-none-locked = 検索中 — 試行 { $blocks_tried } ブロック、まだロックなし

## rds: messages with values (src/app/rds.rs)

rds-auto = 自動（{ $standard }）
rds-type = 種別 { $class }
rds-groups = · グループ { $groups }
rds-blocks-error = · ブロックの { $ber } % に誤り
rds-clean-corrected-lost = 正常 { $blocks_ok } · 訂正 { $blocks_corrected } · 失われた { $blocks_bad }

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC（現地 { $offset }）
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } 地平線の上
sat-min = { $value } 分後
sat-h-min = { $value } 時間 { $value2 } 分後
sat-searching = 検索中… { $entries } 件中 { $next }
sat-locked-2 = ● ロック中 — { $t }
sat-pass-until-max = { $set_unix } までのパス · 最大 { $max_el }°
sat-next-pass-rises-max = 次のパス { $rise_unix } { $rise_unix2 }（{ $now }）· 昇る方位 { $rise_az }° { $rise_az2 } · 最大 { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ rigctld のクライアントが { $clients } 件接続中です。そこでダイヤルを操作する衛星ソフトウェアは、ドップラーを二重に補正してしまいます
sat-rotator-antenna-az-el = ローテーター：アンテナは方位 { $az }°、仰角 { $el }°
sat-rotator-2 = ローテーター：{ $error }
sat-up-now-2 = ● 今、見えています、{ $el }°
sat-inv = { $mode } · 反転
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = { $here } MHz で保持中
scanner-scanning-mhz = スキャン中 · { $here } MHz

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = このレンダラーは、{ $max_texture_dim } ピクセルより広いテクスチャを保持できません。

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = 音声：{ $voice }
speech-speech-unavailable = 音声は利用できません：{ $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = { $stations } 件の送信

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = 前回のセッションは失敗しました — { $err }
winlink-last-session-received-sent = 前回のセッション：受信 { $last_received } 件、送信 { $last_sent } 件
winlink-empty = { $to_lowercase } は空です
winlink-showing = { $total } 件中 { $entries } 件を表示
winlink-fetching = { $mid } を取得中…
winlink-from-2 = 差出人 { $msg } { $date }
winlink-attachment-bytes = 添付：{ $att }（{ $data } バイト）

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    { $reported_by } からのオブジェクト
aprsmap-heard-ago =
    {""}
    { $age } 前に受信

## download: messages with values (src/download.rs)

download-reading = { $path } の読み取り：{ $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}画像がありません：{ $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } MHz
hfdlmap-fix =
    {""}
    { $fixes } 件の位置{ $es }
hfdlmap-heard-ago =
    {""}
    { $last_at } 前に受信

## multi: messages with values (src/multi.rs)

multi-radio = 無線機 { $n }
multi-could-not-add-radio = 無線機を追加できませんでした：{ $error }
multi-was-closed-station = { $value } はステーションで閉じられました。
multi-could-not-open = { $peer } を開けませんでした：{ $error }
multi-has-tab-own-now = { $value } は独自のタブを持つようになりました。

## remote: messages with values (src/remote.rs)

remote-protocol-error = プロトコルエラー：{ $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = 不正なメッセージ：{ $error }
solarnet-solar-protocol-mismatch-server-client = 太陽プロトコルの不一致：サーバー { $proto }、クライアント { $value }

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
smeter-received-signal-strength-s9-db = 受信信号強度：S9 + { $over } dB（パスバンド内で { $s_dbm } dBm）。
smeter-received-signal-strength-s-dbm = 受信信号強度：S{ $value }（パスバンド内で { $s_dbm } dBm）。
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

decodes-you-transmit-s-everyone-else = あなたの送信は他の全員より { $off } 秒 { $after } に行われています。これは、呼び出しに応答がない通常の理由です。コンピューターのクロックを同期してください。

## publicsdr: nested messages

publicsdr-noise-floor-score = ノイズフロアのスコア { $snr }
publicsdr-up-khz-i-q = I/Q を最大 { $khz } kHz

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = GR ゲート { $gate_gr_db } ディエッサー { $deess_gr_db } コンプ { $comp_gr_db } リミッター { $limiter_gr_db }

## amp: more messages

amp-h-step = 約 { $l_step } µH · ステップ { $l_step2 }
amp-pf-step = 約 { $c_step } pF · ステップ { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel } ビット · { $media_type } / { $content_encoding } · { $chunk_count } 個のチャンクに { $encoded_size } オクテット（最初のパスで { $chunks_first_pass }）· セッション { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value } — { $symbol_rate } baud CPFSK、±{ $deviation_hz } Hz、占有帯域幅 { $bandwidth_hz } kHz

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · 現地 { $local }
sat-in-seconds = { $s } 秒後
winlink-to-list = 宛先 { $to }
winlink-cc-list = Cc { $cc }
spots-n-of-total = { $total } 件中 { $n }
wefax-n-of-total = { $total } 件中 { $n }
ais-range-bearing = { $km } km、方位 { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = オフ（チューナーのみ、24 MHz 以上）
rtl-sdr-hf-mode-auto = 自動
rtl-sdr-hf-mode-direct-q = 直接サンプリング（Q 分岐）
rtl-sdr-agc-manual = 手動（AGC なし）
rtl-sdr-agc-tuner = チューナー AGC
rtl-sdr-agc-rtl = RTL デジタル AGC
rtl-sdr-agc-both = チューナー + RTL AGC
mode-control-cat = CAT
mode-control-radio = 無線機が制御
line-state-none = なし
line-state-high = High
line-state-low = Low
elad-tx-input-usb-audio = USB 音声
elad-tx-input-mic = マイク
elad-tx-input-auto = 自動
elad-tx-input-leave = 無線機の設定どおり
tr-usdx-audio-one-cable = ワンケーブル（CAT ストリーム内の音声）
tr-usdx-audio-sound-card = USB サウンドカード（3.5 mm ジャック）
spy-server-format-uint8 = 8 ビット（最も低い帯域）
spy-server-format-int16 = 16 ビット
spy-server-format-float32 = 32 ビット浮動小数点
sound-format-iq = IQ（ステレオ）
sound-format-demod-audio = 復調音声
sdr-play-duo-tuner-tuner1 = チューナー 1（50 Ω / ハイ Z）
sdr-play-duo-tuner-tuner2 = チューナー 2（50 Ω）
sdr-play-duo-role-diversity = 2 本目のアンテナ（ダイバーシティ / QRM 抑圧）
sdr-play-duo-role-second-radio = 2 台目の無線機（独自の周波数）
sdr-play-agc-off = オフ（手動の IF ゲイン）
sdr-play-agc-hz5 = 5 Hz
sdr-play-agc-hz50 = 50 Hz
sdr-play-agc-hz100 = 100 Hz
pluto-ptt-off = オフ
pluto-ptt-gpo01 = GPO0 = RX、GPO1 = TX
pluto-ptt-gpo23 = GPO2 = RX、GPO3 = TX
pluto-duplex-fdd = FDD（同時に両方向）
pluto-duplex-tdd = TDD（一度に一方向）
pluto-agc-manual = 手動
pluto-agc-slow-attack = 遅いアタック
pluto-agc-fast-attack = 速いアタック
pluto-agc-hybrid = ハイブリッド
parity-none = なし
parity-even = 偶数
parity-odd = 奇数
panadapter-tap-antenna = アンテナ（無線機と共有）
panadapter-tap-if-output = 無線機の I.F. 出力
panadapter-audio-attached = 接続された受信機
panadapter-audio-transceiver = トランシーバー
kenwood-send-ts2000 = TS-2000 形式（TX;）
kenwood-send-ts590 = TS-590 形式（TX1;）
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = 無線機が制御
cw-keying-cat = リグのキーヤー（CAT）
cw-keying-audio = サウンドカード（MCW）
alert-sound-ding = ディン
alert-sound-two-tone = ツートーン
alert-sound-triplet = トリプレット
alert-sound-warble = ワーブル
alert-sound-digital = デジタル
alert-reply-tone = トーン
alert-reply-voice = 音声
alert-reply-both = トーン + 音声
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
icom-model-other = その他（アドレスを手で設定）
backend-auto = 自動検出（SoapySDR / CAT）
backend-soapy = SoapySDR
backend-cat = CAT / 音声
backend-hpsdr = HPSDR（ネットワーク）
backend-tci = TCI（ネットワーク）
backend-icom-net = Icom LAN（ネットワーク）
backend-smart-sdr = SmartSDR / FlexRadio（ネットワーク）
backend-pluto = PlutoSDR（ネットワーク）
backend-rtl-sdr = RTL-SDR（USB）
backend-rtl-tcp = rtl_tcp による RTL-SDR（ネットワーク）
backend-spy-server = SpyServer（ネットワーク）
backend-spy-server-vfo = SpyServer VFO+FFT、低帯域（ネットワーク）
backend-kiwi-sdr = KiwiSDR / Web-888（ネットワーク）
backend-rx888 = RX-888（USB）
backend-airspy-hf = Airspy HF+（USB）
backend-airspy = Airspy R2 / Mini（USB）
backend-hack-rf = HackRF One / Pro（USB）
backend-sdr-play = SDRplay RSP（USB）
backend-elad = ELAD FDM-DUO / FDM-S（USB）
backend-lime = LimeSDR + LimeRFE（LimeSuite）
backend-hydra-sdr = HydraSDR RFOne（USB）
backend-fobos = RigExpert Fobos SDR（USB）
backend-usb-audio = USB 音声の無線機（サウンドカード）
backend-none = 未設定
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld（ネットワーク）
cat-family-flrig = flrig（ネットワーク）
publicsdr-its-interface = そのインターフェース

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = オフ
agc-mode-slow = 遅い
agc-mode-med = 中
agc-mode-fast = 速い
airspy-gain-linearity = 直線性（強い信号の扱いが最良）
airspy-gain-sensitivity = 感度（弱い信号に最良）
ais-kind-class-a = クラス A
ais-kind-class-b = クラス B
ais-kind-base-station = 基地局
ais-kind-sar-aircraft = SAR 航空機
ais-kind-aid-to-navigation = 航路標識
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = 関連する船舶
alert-event-called = 局が私を呼んでいます
alert-event-cq = 応答できる CQ
alert-event-new-dxcc = 新しい DXCC エンティティ
alert-event-new-dxcc-band = このバンドで新しいエンティティ
alert-event-new-grid = 新しいグリッドスクエア
audio-routing-both = 左 + 右
audio-routing-left = 左のみ
audio-routing-right = 右のみ
button-mode-momentary = 押している間
button-mode-toggle = 切り替え
contest-mode-none = なし
contest-mode-eu-vhf = EU VHF Contest
converter-tx-off = 変換中はオフ
converter-tx-transverter = 同じコンバーターを通す
converter-tx-own = 独自のオフセット
cw-engine-neural = ニューラル
cw-engine-timing = タイミング
decode-sort-none = なし
decode-sort-signal = SNR
decode-sort-distance = 距離
decode-sort-country = 国
diversity-mode-cancel = キャンセル — ノイズ源をヌルにする
diversity-mode-combine = 合成 — ダイバーシティ受信
dxped-mode-normal = 通常
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = 送信を拒否
fail-safe-warn-only = それでも送信し、警告
ftp-security-plain = FTP（暗号化なし）
ftp-security-explicit-tls = FTPS 明示的（AUTH TLS）
ftp-security-implicit-tls = FTPS 暗黙的
hpsdr-filter-board-none = なし — 出力はオフのまま
hpsdr-filter-board-n2adr = N2ADR フィルターボード
hpsdr-filter-board-alex = Alex / Hermes バンドコード（Zeus、HiQSDR、Quisk）
hpsdr-filter-board-custom = カスタム — 下の自分の表
hpsdr-io-rx-input-radio = 無線機自身の入力
hpsdr-io-rx-input-io-board = IO ボード J9
hpsdr-io-rx-input-io-board-pure-signal = IO ボード J9、送信時は PureSignal
hydra-sdr-gain-linearity = 直線性（強い信号の扱いが最良）
hydra-sdr-gain-sensitivity = 感度（弱い信号に最良）
icom-rx-source-af = AF — 無線機が復調
icom-rx-source-if12k = 12 kHz IF — sdroxide が復調
icom-scope-span-radio = 無線機の設定どおり
icom-scope-span-khz2-5 = 5 kHz（±2.5k）
icom-scope-span-khz5 = 10 kHz（±5k）
icom-scope-span-khz10 = 20 kHz（±10k）
icom-scope-span-khz25 = 50 kHz（±25k）
icom-scope-span-khz50 = 100 kHz（±50k）
icom-scope-span-khz100 = 200 kHz（±100k）
icom-scope-span-khz250 = 500 kHz（±250k）
icom-scope-span-khz500 = 1 MHz（±500k）
ism-family-weather = 気象
ism-family-meter = メーター
ism-family-home-auto = 家庭
ism-family-lo-ra = LORA
ism-family-unidentified = 不明
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = 未識別
js8-speed-normal = 標準
js8-speed-fast = 速い
js8-speed-turbo = ターボ
js8-speed-slow = 遅い
lime-aux-role-off = 使用しない
lime-aux-role-diversity = 2 本目のアンテナ（ダイバーシティ / QRM 抑圧）
lime-aux-role-pure-signal = 送信フィードバック（PureSignal プリディストーション）
lookup-provider-none = オフ
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = 受信箱
mail-folder-outbox = 送信箱
mail-folder-sent = 送信済み
mail-folder-archive = 保管
memory-sort-name = 名前
memory-sort-freq = 周波数
memory-sort-band = バンド
memory-sort-stored = 保存順
mouse-button-middle = 中
mouse-button-extra1 = 戻る（拡張 1）
mouse-button-extra2 = 進む（拡張 2）
nr-level-off = オフ
nr-level-low = 低
nr-level-medium = 中
nr-level-high = 高
nr-level-rnn-low = RNN 低
nr-level-rnn-med = RNN 中
nr-level-rnn-high = RNN 高
nr-level-spec-low = SPEC 低
nr-level-spec-med = SPEC 中
nr-level-spec-high = SPEC 高
nr-level-nr2-low = NR2 低
nr-level-nr2-med = NR2 中
nr-level-nr2-high = NR2 高
nr-level-df-low = DFNR 低
nr-level-df-med = DFNR 中
nr-level-df-high = DFNR 高
nr-strength-low = 低
nr-strength-med = 中
nr-strength-high = 高
orbit-rings-none = なし
orbit-rings-curated = キュレーション済み
orbit-rings-all = すべて
qso-step-idle = 待機
qso-step-wait-cq = CQ を待機
qso-step-calling-cq = CQ を呼んでいます
qso-step-tx-grid = 送信：グリッド
qso-step-tx-report = 送信：レポート
qso-step-tx-r-report = 送信：R+レポート
qso-step-tx-rr73 = 送信：RR73
qso-step-tx73 = 送信：73
qso-step-confirming = 確認中
rds-standard-auto = 自動
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = リージョン 1 — ヨーロッパ、アフリカ、中東、北アジア
region-r2 = リージョン 2 — 南北アメリカ
region-r3 = リージョン 3 — 南アジアと東アジア、オーストラリア、太平洋
relative-mode-absolute = 絶対（フェーダー）
relative-mode-sign-magnitude = 相対：符号 / 大きさ
relative-mode-twos-complement = 相対：2 の補数
relative-mode-binary-offset = 相対：64 中心
relay-link-off = なし
relay-link-serial = USB リレーボード（シリアル）
relay-link-serial-lines = シリアル RTS/DTR 回線
relay-link-hid = USB リレーボード（HID）
relay-link-cm108 = CM108/CM119 サウンドカードの GPIO
relay-link-gpio = Linux の GPIO 回線
relay-link-command = 外部コマンド
relay-role-unused = 使用しない
relay-role-sdr-antenna = SDR アンテナを接地
relay-role-amplifier = アンプ / T-R リレーをキー
relay-role-aux = 補助
relay-role-band-decoder = バンドデコーダー（フィルター / トランスバーター）
rfe-channel-wb1000 = 広帯域 1〜1000 MHz
rfe-channel-wb4000 = 広帯域 1〜4 GHz
rfe-channel-ham0030 = HF、30 MHz まで
rfe-channel-ham0070 = 6 m / 4 m（50〜70 MHz）
rfe-channel-ham0145 = 2 m（140〜150 MHz）
rfe-channel-ham0220 = 1.25 m（220〜225 MHz）
rfe-channel-ham0435 = 70 cm（400〜450 MHz）
rfe-channel-ham0920 = 33 cm（902〜928 MHz）
rfe-channel-ham1280 = 23 cm（1220〜1420 MHz）
rfe-channel-ham2400 = 13 cm（2.3〜2.5 GHz）
rfe-channel-ham3500 = 9 cm（3.3〜3.7 GHz）
rfe-channel-cell-band01 = セルラーバンド 1
rfe-channel-cell-band02 = セルラーバンド 2
rfe-channel-cell-band03 = セルラーバンド 3
rfe-channel-cell-band07 = セルラーバンド 7
rfe-channel-cell-band38 = セルラーバンド 38
rfe-channel-not-selected = 未選択
rfe-link-off = 未接続
rfe-link-serial = 独自の USB ケーブル（シリアル）
rfe-link-board = LimeSDR 経由（GPIO / I²C）
rfe-mode-control-auto = 自動（配線に従う）
rfe-mode-control-rx = 常に受信
rfe-mode-control-tx = 常に送信
rfe-mode-control-tx-rx = 常に両方（分割コネクターのみ）
rifp-encoding-auto = 自動
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = 生
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = MEM
scan-kind-range = 範囲
scan-resume-carrier = キャリア
scan-resume-timed = 時間指定
scan-resume-manual = 手動
sense-line-off = 配線なし
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD（キャリア検出）
shift-simplex = シンプレックス
shift-minus = −
shift-plus = +
speed-slow = 遅い
speed-medium = 中
speed-fast = 速い
speed-faster = より速い
speed-fastest = 最速
tls-verify-system = システムの信頼ストア
tls-verify-pinned = 固定されたフィンガープリント
tls-verify-off = 検証しない
tone-mode-off = オフ
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN（巡回）
upload-naming-original = 元のファイル名
wheel-action-zoom = ズーム
wheel-action-tune = 同調
wheel-action-none = なし

settings-alerts-say = 読み上げ

settings-alerts-say-hint = このアラートを音声で聞く

settings-alerts-say-need-voice = 聞くには「音声」または「トーン＋音声」を選んでください

settings-alerts-say-voice-off = 設定 → UI → 音声案内 で音声をオンにしてください

settings-alerts-speaks = 読み上げ: { $text }

settings-picup-keep = 保持

jtty-starting-receiver = JTTY受信機を起動中…

jtty-receive-audio-level = 受信音声レベル

jtty-sending = {" "}送信中{" "}

jtty-sent-once-note = メッセージはTXを押したときに一度だけ送信されます。JTTYは非同期なので、待つ周期はなく、繰り返し送信もされません。

signalid-title = 信号識別

signalid-tuned-now = 現在の同調

signalid-passband = 通過帯域 { $bw }

signalid-band = { $band } バンド

signalid-find = 検索

signalid-find-hint = FT8、流星、気象、航空機…

signalid-candidates = このダイヤルの候補 { $n } 件

signalid-matches = 「{ $query }」に一致 { $n } 件

signalid-nothing-fits = このダイヤル、モード、通過帯域に合うものはカタログにありません。

signalid-no-match = 一致するカタログ項目はありません。

signalid-open-url = { $url } を開く

windows-chip-del = 削除

windows-chip-edt = 編集

windows-chip-ren = 名変

windows-chip-rec = 録音

windows-name-hint = 名前

windows-waterfall-marks = ウォーターフォール

windows-stacked = 積み重ね

jtty-listening-note = 受信待機中。JTTYの送信はいつでも始まります。受信するとメッセージが少し遅れてここに表示されます。

extras-title = 追加機能

extras-receivers = 受信機

extras-dab-hint = DAB / DAB+ デジタルラジオ — プレーヤーはウォーターフォールの下に開きます

extras-aircraft-watch-hint = 航空機 — ウォッチリストの航空機が電波を出しています。ウォッチビューを開いてください

extras-aircraft-hint = 航空機 — 無線機が受信している航空機とラジオゾンデの地図。別ウィンドウに切り離せます

extras-radio-hint = RADIO CLASSIC — 放送受信向けのクラシックなラジオ画面。バンド、メモリー、局名、RadioTextを表示します。音声モードではウォーターフォールの下にドッキングします。設定 → 切り離し でウィンドウにできます

extras-antenna = アンテナ

extras-antenna-hint = アンテナ（実験的/ベータ）— 一般的なワイヤー/垂直/八木アンテナの切断長、給電点インピーダンス、パターン

recjobs-title = 録音

recjobs-finished = 「{ $name }」が完了

recjobs-skipped = 「{ $name }」をスキップ — すでに録音中

recjobs-recording-name = 「{ $name }」を録音中

recjobs-new-job = + 新規ジョブ

recjobs-recording = ● 録音中

recjobs-none = 予約録音はありません。+ 新規ジョブ を押して追加してください。

recjobs-done = 完了

recjobs-edit = 編集

recjobs-del = 削除

recjobs-station = 放送局

recjobs-station-hint = BBC World Service

recjobs-kind = 種類

recjobs-start-utc = 開始 (UTC)

recjobs-for = 長さ

recjobs-save = 保存

recjobs-kind-audio = 音声

recjobs-kind-iq = I/Q

recjobs-kind-both = 両方

panels-sensitivity = 感度

navtex-hunting = 探索中

navtex-hunting-hint =
    まだ何も受信できていません。

    ほとんどの場合、同調が原因です。案内されている数字（518）を周波数欄に入力しないでください。NAVTEXはチャンネルとして示されており、本プログラムはその少し下で受信します。代わりに上の518ボタンを押してください。正しいダイヤル（516.300）に設定されます。

    強い信号があるのに文字が意味不明な場合は、REVを押してください。信号が逆向きに届いています。

    また、NAVTEXは会話ではなく時刻表で運用されます。海岸局は4時間ごとに10分間送信するので、ほとんどの時間は無音で正常です。局の送信時刻に合わせて受信する必要があります。さらに518 kHzは夜間にしか開けません。昼間に静かなのはデコーダーの故障ではありません。

navtex-sensitivity-hint =
    感度 — デコーダーが信号にどれだけ捉えやすく、保持しやすいか。
navtex-auto-hint = 基準しきい値（0）に戻します。有効な間は強調表示されます。

navtex-tune-dial-usb-below-channel = ここにダイヤルを合わせます（USB、チャンネルより1.710 kHz下）。

settings-audio-your-audio = お使いのオーディオ

settings-audio-your-audio-why = 聴くためのスピーカーと、音声を送り出すマイクです。これらは無線機ではなくこのマシンに属するので、どの無線機を使っていても同じです。

settings-audio-browser-note = ここではサウンドデバイスをこのウィンドウではなくブラウザーで選びます。ページがブラウザーにスピーカーとマイクを問い合わせます。このページの下にある局側のサウンドカード設定も引き続き有効です。

settings-audio-speakers = スピーカー

settings-audio-channels-hint = 受信音声をどのスピーカーチャンネルに出すか。左または右にすると反対側が無音になり、片側だけで聴けます。

settings-audio-microphone-note = マイクは、無線機が音声（MCW）またはVOXでキーイングされるときに音声を送り出します。無線機がCATで自らキーイングする場合、マイクは送信にまったく使われず、この行を変更しても何も変わりません。

settings-audio-stream-out = 音声をストリーム出力

settings-audio-stream-out-why = この無線機が受信した音声を、サウンドカードを介さずにネットワーク経由で別のプログラム（デコーダー、レコーダー、別のマシン）へ送ります。

settings-audio-send-to-hint =
    `host:port`。モノラル、16ビット、受信レートで、音声ブロックごとに1データグラムです。**録音タップ**の音声なので、AFボリューム、ミュート、送信状態の影響を受けません。外部デコーダーが必要とするのはヘッドホンの音量ではなく無線機の音だからです。

    ホスト名ではなく数値アドレスで指定します。空欄はオフです。

settings-audio-send-to-note = `127.0.0.1:7355` はこのコンピューター上の別のプログラムへ送ります。このマシン自身のアドレスならネットワーク経由で送ります。

settings-audio-radio-audio = この無線機のオーディオ

settings-audio-radio-audio-why = この無線機とコンピューターの間のサウンドカードです。マシンではなく無線機に属します。ここにある2台の無線機が同じコンピューターを共有しても、別のカードが必要な場合があるため、この設定は設定を開いた無線機ごとに保持されます。

settings-audio-no-cards-yet = このマシンはまだサウンドカードを報告していません。名前は下に入力できます。デバイスの問い合わせが済むと一覧が表示されます。

settings-audio-rescan = サウンドカードを再スキャン

settings-audio-rescan-hint =
    マシンにサウンドカードを再度問い合わせます。何も開かないので、受信中に押しても安全です。

    コーデックを接続した後に使ってください。無線機は再接続されません（それは無線機タブの適用/再接続です）。また、サウンドサーバーが名前を変えたカードは修復できません。その場合は無線機側で報告されます。

settings-audio-from-radio = 無線機から

settings-audio-to-radio = 無線機へ

settings-audio-radio-note = { "*" }*無線機から**は、リグが出力する音声です（USBサウンド出力、または受信機のヘッドホン端子からサウンドカードのライン入力）。**無線機へ**は、リグが音声として聞き取る入力で、プログラムが音声でリグをキーイングする経路でもあります。空欄にすると、リグ自身のマイクが使われます（あれば）。

settings-audio-recording = 録音

settings-audio-recording-why = 音声録音、予約録音、生のI/Qキャプチャーの保存先です。

enigma-need-more = もっと暗号文が必要です

enigma-title = ENIGMA

enigma-no-cables = ケーブルなし

enigma-thin = 少なめ

enigma-rotor = ローター

enigma-window-hint = Grundstellung — クリックでローターを進めます

enigma-input-hint = 入力するか、上のキーを使います

enigma-clear = クリア

enigma-copy = コピー ⇄

enigma-solve-note = LÖSEN — 暗号文を貼り付けてください。ケーブル接続ありの文面には、メッセージに含まれると思われるクリブ単語を入力します。なしの場合は、配線なしの機械を解きます。

enigma-crib = クリブ

enigma-solve = 解読

enigma-flaw = 文字が自分自身に暗号化されることはありません。反射板がそれを防ぎます。これは実機の弱点であり、解読器はそれを使って枝刈りします。

settings-audio-takes-over-speakers = { "*" }*このスピーカーはサウンドカードではなくサウンドサーバーのデバイスです。** 使用すると、このコンピューター上の*他のすべて*の再生デバイスにもなり、他のプログラムもここから音が出ます。PipeWireでは、プログラムごとに再生先を選ぶ方法がないため、これは避けられません。変更するには、サウンドカードを選ぶか、マシンの既定を戻して（`wpctl set-default <name>`）再スキャンしてください。

settings-audio-takes-over-microphone = { "*" }*このマイクはサウンドカードではなくサウンドサーバーのデバイスです。** 使用すると、このコンピューター上の*他のすべて*の録音デバイスにもなり、他のプログラムもここから録音します。PipeWireでは、プログラムごとに録音元を選ぶ方法がないため、これは避けられません。変更するには、サウンドカードを選ぶか、マシンの既定を戻して（`wpctl set-default <name>`）再スキャンしてください。

settings-audio-currently-chosen = ⚠  現在の選択: { $device }

knowncalls-no-table = このモードにはコールサイン表がありません。コールサインをハッシュ化するFT8/FT4デコーダーのみが対象で、それ以外には表示する内容がありません。

knowncalls-not-asked = まだ問い合わせていません。更新を押してください。

knowncalls-nobody = まだ誰もいません。局は、CQ、双方のコールが平文の信号レポート、または11 mの短いコールなど、コールサインを綴って送信するのが聞こえた時点で追加されます。

knowncalls-title = 既知の局

knowncalls-intro = この受信機でハッシュ化された <...> が現在名指しできるコールサインすべて。ハッシュは逆引きできません。局はコールサインを綴って送信するのが聞こえて初めてここに現れます。11 mの短いコールはそのためのものです。

knowncalls-find-hint = コールサイン、国、プレフィックス…

knowncalls-refresh = 更新

knowncalls-capped = { $total } 件中の最新 { $shown } 件 — 更新で再取得、一覧はスナップショットです

knowncalls-known = 既知 { $shown } 局

knowncalls-matches =
    { $n } { $n ->
        [one] 件一致
       *[other] 件一致
    }

knowncalls-no-match = 一致する既知の局はありません。

knowncalls-no-country = 国なし — この表にないコールサインです

classic-controls = 操作

classic-fold-the-column-away-to-a-rail-on = 列を左のレールに折りたたみます。表示とメーターはバーに残り、元に戻す操作はレール上にあります。

classic-click-to-open-or-close-this = クリックでこのセクションを開閉します。他のセクションへドラッグすると移動できます。順序は記憶されます。

classic-bring-the-control-column-back = 操作列を元に戻します。

classic-no-radio = 無線機なし

classic-rate = レート

classic-dial = ダイヤル

classic-receive = 受信

classic-listening-screen = 受信画面

classic-radio-settings = 無線機設定

classic-the-backend-the-sample-rate-the = バックエンド、サンプルレート、アンテナ、コンバーター、この無線機固有の校正 — 従来どおり 設定 → 無線機 にあります。この列は操作の複製ではなく、現在の設定値を表示します。

classic-counted-in-steps-not-decibels-what = デシベルではなくステップ単位です。1ステップの値は受信機次第で、SDRplayではバンドによっても変わります。右が高ゲインなのは変わりません。

classic-the-plutosdr-s-own-agc-is-setting = PlutoSDR自身のAGCがこのゲインを設定しているため、スライダーを動かしても何も起きません。ここで制御するには、設定 → 無線機 → AGC を手動にしてください。

classic-squelch = スケルチ

classic-gate-the-audio-below-this-power-in = 受信通過帯域内でこの電力（dBFS）を下回る音声を遮断します。左で開放です。ノイズより上、信号より下に設定してください。

classic-this-radio-has-not-reported-what = この無線機はまだ機能を報告していません。リンクが閉じているか、応答がありません。

classic-open-the-link-in-the-radio-strip = 無線機ストリップでリンクを開くと、送信コントロールがここに表示されます。

classic-this-device-receives-only-it-has = このデバイスは受信専用です。本プログラムがキーイングする送信機はありません。

classic-transmit-is-offered-by-a-rig-with = 送信は、制御経路または音声経路を持つリグで利用できます。設定 → 無線機 を参照してください。

classic-transmit-is-off-for-this-radio-s = この無線機の画面では送信がオフです。

classic-settings-radio-show-transmit = 設定 → 無線機 →「送信コントロールを表示」で元に戻ります。

classic-key = キー

classic-controlled-envelope-ssb-how-much = コントロールド・エンベロープSSB: 音声処理をどれだけかけるか。上げると音は大きく平坦になりますが、両側のスプラッターが少し増えます。

classic-holding-the-squelch-open-click-to = スケルチを開いたままにしています。クリックで元のゲートに戻します。

classic-click-to-hold-the-squelch-open-to = クリックでスケルチを開いたままにして、何か信号があるか聴けます。もう一度クリックすると元の状態に戻ります。

panels-received = 受信

panels-text = テキスト

panels-hex = 16進

panels-nothing-received-yet = まだ何も受信していません。

panels-payload = ペイロード

panels-blocks = ブロック

panels-sequence = シーケンス

uvpacket-starting-receiver = UVPacket受信機を起動中…

uvpacket-frames-count = { $n } フレーム

uvpacket-frames-count-hint = 受信機の起動後にデコードされたフレーム数。

uvpacket-intro = 限定VHF/UHFグループ向けのパケットモード。ダイヤルをグループのチャンネルに合わせてください。サブモードはフレームごとに検出されます。

uvpacket-nothing-yet = まだありません。UVPacketは送信の合間は静かです。チャンネルに合わせたままバーストを待ってください。

uvpacket-app-seq = アプリ { $app } · シーケンス { $seq }

uvpacket-click-read-frame = クリックでフレーム全体を表示します。

uvpacket-bps-net = 実効 { $bps } bps

uvpacket-sub-mode = サブモード

uvpacket-app-type = アプリ種別

uvpacket-payload-bytes = { $n } バイト

dsc-starting-receiver = DSC受信機を起動中…

dsc-seq-tone = { $n } シーケンス · トーン { $pct }%

dsc-seq-tone-hint = 終了マーカーまで届いたシーケンス数と、2つのトーンがどれだけきれいに分離できているか。

dsc-nothing-yet = まだありません。DSCは船舶の送信の合間は静かです。チャンネルに合わせたままバーストを待ってください。

panels-marginal = 境界

dsc-marginal-hint = 文字がBCHチェックに失敗しました。表示されている項目は誤りの可能性があります。

dsc-click-read-sequence = クリックでシーケンス全体を表示します。

panels-marginal-caps = 境界

dsc-marginal-caps-hint = 少なくとも1文字がBCHチェックに失敗しました。不完全に届いた遇難警報でも読む価値があるため、シーケンスを表示しています。

dsc-category = カテゴリー

dsc-nature = 種別

dsc-caller-mmsi = 発信MMSI

dsc-target-mmsi = 宛先MMSI

dsc-alert-time = 警報時刻

dsc-working-ch = 運用ch

dsc-raw-symbols = 生シンボル

dsc-none = （なし）

settings-general-device-type-name = …または名前を入力 — 空欄はシステム既定

settings-general-signed-in-station = サインイン中の局

settings-general-sign-out-hint = このデバイスのサインインを忘れます。次回の接続時に局から再度認証を求められます。この接続は閉じるまで維持されます。

settings-general-recordings-remote = 録音は無線機が接続されているマシン上に保存されます。フォルダーはそちらで設定してください。

settings-general-folder = フォルダー

settings-general-choose = 選択…

settings-general-choose-hint = 録音の保存先フォルダーを選びます

settings-general-default = 既定

settings-general-default-hint = 既定に戻します — ミュージックフォルダー、システムにない場合は設定ディレクトリ

settings-general-recordings-dir-note = 音声録音、予約録音、生のI/Qキャプチャーの保存先。既定: ミュージックフォルダー内のsdroxideサブフォルダー。システムにミュージックフォルダーがない場合は、設定ディレクトリのrecordingsフォルダー。

setup-suffix-cq = { " " }cq

setup-suffix-tries = { " " }回

setup-window = ウィンドウ

setup-south = 南

setup-west = 西

setup-message-templates = メッセージテンプレート  { $tokens }

setup-ft8-decoder-effort-hint =
    FT8デコーダーが弱い信号にどれだけ力を入れるか。

    高速 — 1パス、信号減算なし: 最速で、デコード数は最少。
    標準 — 平坦な多パス減算: 混雑したスロットで約0.4秒、ディープよりデコードが数件少なめ。
    ディープ — WSJT-Xのチェックポイント方式パス: 約1.2秒で最多デコード（既定）。

    対象は2回目のデコード群のみです。通常パスは常に先に出力されるので、返信が遅れることはありません。

setup-auto-cq-repeats-hint = 1回の運用で終了するまでに出すCQの回数。0は独自の上限なしで、送信ウォッチドッグと無操作停止が引き続き運用を制限します。

setup-auto-cq-then-listen-hint = それらのCQの後、完全に停止せず、自動CQから自動RXに切り替えて受信を続けます。オフの場合は運用が停止します。

setup-auto-call-retries-hint = 1つの局を呼ぶ回数。超えるとその局を諦めてキューの次へ移ります。

setup-auto-session-hint = 無人運用が何をしていても続けられる時間。0は独自の上限なしです。

setup-serial-contests-hint = シリアルコンテスト — ARRL RTTY RoundupとFT8によるCQ WPX: 信号レポートとシリアル番号（または米国の州）で、ロケーターはなし。CQはCQ RUで送出されます。

setup-tx-level-fm-hint =
    このモードのバーストを、自ら変調する無線機へ渡す大きさです。FMではこれが偏移になります。FM送信機は音声レベルを周波数偏移に変え、それを抑えるALCがないためです。1200ボーパケットは約3 kHz、音声は5 kHzを求めるので、音声用に設定されたデータ入力へフルスケールを入れると過偏移になり、聞いた限りでは普通なのに誰にもデコードされません。他局から応答が来るまで下げるか、無線機側でレベルを設定してください。

    このレベルはこのモード専用です。まだ設定していないモードはFMの値から、サイドバンドのモードはそれぞれ自分の値から始まるので、1200ボー用に設定した偏移がFT8に影響することはありません。

setup-tx-level-ssb-hint =
    このモードの送信を、自ら変調する無線機（サウンドカード経由のCATリグ、FLEX、ネットワークポート経由のIcom）へ渡す大きさです。サイドバンドでは変調器へのドライブになります。リグのALCがほとんど動かなくなるまで下げ、電力は無線機側で設定してください。定包絡線モードでALCが働くとスプラッターの原因になります。ここでのドライブはリグの音声ではなく電力レジスターに届くので、これがそのレベルです。

    このレベルはこのモード専用です。FT8、RTTY、PSK、MCWはそれぞれ別に保持します。変調器への負荷のかかり方が異なるためです。同じスライダーが送信ストリップにもあり、送信中でも操作できます。

setup-repeats-hint-base = 1つの局への無応答の呼び出しが、次へ移るまでの回数。CQは対象外です。

setup-repeats-hint-duration = { " " }{ $mode } のスロット長（{ $slot } 秒）では約 { $mins } 分です。

setup-repeats-hint-disables = { " " }0で無効になります。

setup-repeats-hint-11m = { " " }11 mではこれがCQ運用を終わらせる条件にもなります。CQに応答する局は約30秒以内に来るはずで、時計では新しい運用と古い運用を区別できないため、送信ウォッチドッグは切断しません。

setup-symbol-home-station = 自宅局

setup-symbol-car = 自動車

setup-symbol-truck = トラック

setup-symbol-van = バン

setup-symbol-motorcycle = オートバイ

setup-symbol-bicycle = 自転車

setup-symbol-person-on-foot = 徒歩の人

setup-symbol-motorhome = キャンピングカー

setup-symbol-boat = ボート

setup-symbol-yacht = ヨット

setup-symbol-light-aircraft = 軽飛行機

setup-symbol-balloon = 気球

setup-symbol-digipeater = デジピーター

setup-symbol-i-gate = Iゲート

setup-symbol-repeater = リピーター

setup-symbol-weather-station = 気象局

setup-symbol-hospital = 病院

setup-symbol-portable-campsite = 移動運用 / キャンプ場

settings-general-sign-out = サインアウト

settings-general-fallback-password = { $stations } のパスワードをこのブラウザーに記憶しています。サインインCookieが返されなかったためです。古い局ではこうなりますが、同じものではありません。

settings-general-device-hint-speakers = スピーカーを開くときの正確な名前です。一覧にあれば一覧から選び、なければ入力してください。サウンドサーバーが、ホストAPIが列挙しないデバイスを提供することがあります。空欄はシステム既定を使います。

settings-general-device-hint-microphone = マイクを開くときの正確な名前です。一覧にあれば一覧から選び、なければ入力してください。サウンドサーバーが、ホストAPIが列挙しないデバイスを提供することがあります。空欄はシステム既定を使います。

settings-general-device-hint-radio-rx = この無線機の受信音声を開くときの正確な名前です。一覧にあれば一覧から選び、なければ入力してください。サウンドサーバーが、ホストAPIが列挙しないデバイスを提供することがあります。空欄はシステム既定を使います。

settings-general-device-hint-radio-tx = この無線機の送信音声を開くときの正確な名前です。一覧にあれば一覧から選び、なければ入力してください。サウンドサーバーが、ホストAPIが列挙しないデバイスを提供することがあります。空欄はシステム既定を使います。

settings-general-device-hint-other = デバイスを開くときの正確な名前です。一覧にあれば一覧から選び、なければ入力してください。サウンドサーバーが、ホストAPIが列挙しないデバイスを提供することがあります。空欄はシステム既定を使います。

settings-general-cb-world = 世界 / フリーバンド — 80ch

settings-general-cb-cept = CEPT / EU — 40ch

settings-general-cb-germany = ドイツ — 80ch

settings-general-cb-uk = 英国 — 27/81、40ch

settings-general-cb-us = 米国 — 40ch

settings-general-cb-australia = オーストラリア — 40ch

settings-general-cb-pmr446 = PMR446 — 16ch (446 MHz)

settings-general-cb-dpmr446 = dPMR446 — 32ch (446 MHz)

wefax-receiving-progress = 受信中 { $min } 分 · { $lines } ライン

wefax-listening = 受信待機

wefax-status-hint =
    天気図を待っています。

    WEFAXは会話ではなく時刻表で運用されます。局は決まった時刻に天気図を送信するため、選んだ周波数は一日の大半は静かで、それは正常です。局ボタンで局を選び、時刻を確認してください。

    信号はあるのに画像が単一の色調（全面グレー、全面黒、全面白）になる場合は、同調か速度が原因です。+0 Hz表示とIOC/速度の設定を使い、局の項目が実際の送信内容と一致しているか確認してください。

wefax-centre = 中心

wefax-centre-hint = ダイヤルを { $sign }{ $off } Hz ずらして、副搬送波を再び1900 Hzに合わせます。受信機が { $off } Hz ずれており、これが天気図が白紙になる原因です。

wefax-auto-start-hint = 300 Hz（IOC 576）または675 Hzのスタートトーンを検出したとき、あるいはトーンがフェージングやキャリアで失われた場合は位相信号が数秒届いたときに天気図を開始します。次の送信が来たら、ストップトーンの有無にかかわらず、進行中のものを終了して新しいものの位相合わせを行います。チャンネル上のキャリアだけでは何も開始しません。

wefax-fit = フィット

wefax-save-chart-hint = 画面上の天気図を、届いた分までこのコンピューターに保存します

wefax-waiting-for-chart = USBでファクスの時刻表に合わせ、天気図の開始（スタートトーンまたは位相信号）を待つか、開始を押して途中から始めてください。

wefax-carrier-dial-note = キャリア { $carrier } · ダイヤル { $dial } kHz (USB −1.9k)

wefax-carrier-note-hint = 局チップには公表されたキャリアが表示されます。ラジオファクスの副搬送波はその1900 Hz上にあるため、ダイヤルは意図的に1.9 kHz低くなります。ダイヤルをキャリア自体に合わせると、信号は通過帯域の外に出ます。

contest-title = コンテスト

contest-intro = コンテストを選び、送信する内容を入力します。ロガーはどのモードでも使えます。CWやSSBではエクスチェンジを入力し、FT8側はコンテストにFT8の形式があれば自動で入力されます。

contest-you-send = { "  " }送信: { $exchange }

contest-they-send = { "  " }相手の送信: { $exchange }

contest-cb-note = CB / 11 m運用: レポートと自由記述のエクスチェンジ。運用で使うチャンネル、名前、地域を入れてください。

contest-your-exchange = 自局のエクスチェンジ

contest-start = 開始

contest-score-line = QSO { $qsos } · { $points } 点 · マルチ { $mults } · 得点 { $total }

contest-rate = 10分で { $r10 } · { $r60 }/時

contest-stop = 停止

contest-estimate = 得点は概算です。最終判定は主催者が行います

contest-call = コール

contest-dupe = 重複

contest-sent = 送信

contest-rcvd = 受信

contest-sending = 送信中 { $rst }

contest-sending-ours = 送信中 { $rst } { $ours }

contest-log = ログ

playback-radio-n = { " " }· 無線機 { $n }

playback-no-stamp = 名前にタイムスタンプなし — 現在のモードで再生します

playback-title = 再生 — 録音音声

playback-find-hint = モード、MHz、日付、名前…

playback-refresh-hint = 録音フォルダーを再読み込み

playback-asking = 局に録音を問い合わせ中…

playback-folder = フォルダー: { $dir }

playback-none = まだ録音がありません。電波が出ている間に REC → MP3 または WAV を押すと、時刻、周波数、モードを含む名前のファイルがここに表示されます。

playback-on = ▶ 再生中

playback-play = ▶ 再生

playback-iq-hint = I/Qキャプチャーで、音声ではありません。再生はレコーダーが書き出す音声を扱います。I/Qキャプチャーは --file で再生します。

playback-not-while-tx = 送信中は不可

playback-play-hint = この録音を最初からデコーダーへ再生します

playback-busy-playing = 再生中です。削除する前に停止してください

playback-busy-recording = 録音中です。削除する前にRECを停止してください

playback-delete-ask = 削除？

playback-delete-hint-armed = もう一度押すとこのファイルを完全に削除します。元に戻せません。

playback-delete-hint = この録音をフォルダーから削除します（先にもう一度確認します）

playback-strip-label = ▶ 再生

playback-not-live = ライブではありません — 無線機は待機中です

playback-carry-on = 再生を続ける

playback-pause = ⏸ 一時停止

playback-pause-hint = 一時停止 — 再開するまでデコーダーには無音が聞こえます

playback-again = ↺ もう一度

playback-again-hint = 最初に戻ります — 画像やデコードをもう一度試せます

playback-loop = ループ

playback-loop-hint = ファイルが終わったら最初からやり直します

playback-stop = ■ 停止

playback-stop-hint = 再生を終了し、無線機を元の状態に戻します

playback-files = ファイル

playback-end-of-file = ファイル終端

playback-paused = 一時停止中

playback-recorded-on-mode = { $mhz } MHz、{ $mode } で録音 — ダイヤルとモードはファイル名から設定されました。

playback-recorded-on = { $mhz } MHz で録音 — ダイヤルとモードはファイル名から設定されました。

playback-no-dial-mode = 名前にダイヤルもモードも記録されていないため、現在のモードで再生します。モードメニューからデコーダーを選んでください。

playback-format-stereo = { $rate } Hz、ステレオ — 左（受信）チャンネルを再生します。

playback-format-mono = { $rate } Hz、モノラル — その1チャンネルを再生します。

playback-stopped = 停止: { $fault }

playback-intro = 録音をデコーダー（SSTV、CW、RTTY、PSK、FT8、その他音声からデコードするすべてのモード）へ再生します。再生中は無線機が待機し、ダイヤルとモードは録音時の状態になり、停止で無線機が元に戻ります。再生中にモードを変えると別のデコーダーを試せます。

packet-callsign-hint = コールサイン

packet-via-hint = 経由

aprs-filter-hint = フィルター

aprs-msg-sent = 送信済

aprs-msg-ack = 確認済

aprs-msg-rejected = 拒否

aprs-reply-chip = 返信

aprs-to-hint = 宛先

aprs-message-hint = メッセージ

aprs-raw-chip = RAW

fsq-to-button = 宛先

fsq-del-chip = 削除

macros-column-key = キー

macros-column-button = ボタン

macros-column-sends = 送信内容

macros-label-hint = ラベル

macros-add = 追加

macros-note-placeholders = { $mycall } と { $mygrid } は送信時に自動で入ります。

macros-buttons-note = 各ボタンは全文を一度に送信します。画面上でキーボードが使われていなければ、F1〜F9で最初の9個を押せます。



widgets-worked-before-any = 交信済み

vdl2-col-sig = 信号

vdl2-col-age = 経過

vdl2-command = コマンド

vdl2-response = 応答

vdl2-frame-hint = フレーム

vdl2-suffix-messages = { " " }メッセージ

vdl2-suffix-stations = { " " }局

ale-words-count = { $n } ワード

ale-waiting-audio = 音声を待っています…

ale-no-words-yet = まだワードがありません。ALEチャンネル（例: 8992 または 11175 kHz USB）に合わせて、呼び出しを待ってください。

atchat-role-master = マスター

atchat-role-backup = バックアップ

atchat-me = 自分

dab-sub-channel = { " " }· サブ { $n }

dab-starting-receiver = DAB受信機を起動中…

dab-no-ensemble-yet = アンサンブル名がまだありません。このチャンネルでDAB送信はありますか？

dab-window-stats = { $mhz } MHz · { $frames } フレーム · { $fibs } FIB

dab-volume-meter-hint = 聴いているとおりの局の音（左右）と音量です。バーに沿ってドラッグまたはクリックするか、ホイールを使います。クリック後は矢印キーで刻めます。南京錠でロックした画面でも使えます。

rade-searching = 探索中

rfpaint-swl-note = RF Paintは送信専用モードです。SWLで表示するものはありません。

cw-hand-key-arm-hint = キーヤーを有効にしてUSBパドルを使います。接点はエンジンに送られ、エンジンが送信速度で短点と長点を作ります。パネルではなくここで行うため、タイミングがフレーム単位に丸められません。有効な間は送信ボックスがロックされ、キュー内のテキストは破棄されます。同じボックスでストレートキーも使えます。ストレートキー型を選んでください。

cw-own-keyer-hint =
    この無線機は内蔵キーヤーから送信します。テキストは制御ポート経由で送られ、リグが要素のタイミングを取るため、手と電波の間にキーで駆動できるものはありません。パドルもストレートキーも同様です。

    手でキーイングするには、設定 → 無線機 でCWキーイングを「サウンドカード (MCW)」にしてください。リグはサイドバンドに保持され、キーヤー自身のサイドトーンが音声として送信されます。ストレートキーが駆動するのはこの経路です。

cw-no-paddle-found = パドルが見つかりません — 設定 → CW で選んでください

cw-msg-hint = 自分用のメッセージボタン — コンテストのエクスチェンジ、名前とQTHの返信、TNX 73 GLなど。各ボタンは全文を一度に送信し、F1〜F9で最初の9個を押せます。局の設定と一緒に保存されるので、リモートクライアントでも使えます。

wspr-clear-rx = RXクリア

wspr-rx-count = 受信 { $n }

wspr-sort-dist = 距離

wspr-sort-country = 国

wspr-tx-enable = TX有効

wspr-stop-tx = TX停止

wspr-listening-as = { $call } として { $sent } で受信中 — 設定の一般タブで設定します。

wspr-set-callsign-grid-label-spots = 設定の一般タブでコールサインとグリッドを設定すると、スポットにラベルが付きます。

schedule-source-mine = 自分の

schedule-all-of-them = すべて

schedule-title = 番組

schedule-at = 時刻

schedule-now-chip = 現在

schedule-set-time = 時刻設定

schedule-find-hint = BBC、6185、15.4 MHz…

schedule-target = ターゲット

schedule-any = 任意

schedule-source = ソース

schedule-all-hint = ダウンロード済みのすべての時刻表を1つのストアにまとめたもの。2つの時刻表に載っている局は2回表示されます。このウィンドウはその違いを見るためのもので、パナダプターには名前が1つだけ表示されます。

schedule-favs = ★ お気に入り

schedule-favs-hint = 星を付けた局のみ

schedule-all-favs = ⭐ 全部

schedule-all-favs-hint = すべての時刻表の局をお気に入りにし、オンの間はウォーターフォールにそのラベルを表示します。もう一度クリックするとお気に入りをすべて解除します。

schedule-solar-time = 現地時刻

schedule-solar-time-hint = 各行の送信所での現地時刻を表示します。経度から1度あたり4分で求めた太陽時であり、標準時ではないため、夏時間や時間帯の境界は考慮されません。

schedule-map = 地図を表示

schedule-map-hint = LISTENを開きます — この時刻表を地図で表示。地図上で局を選び、聞こえた内容を書いてログに記録します。選択した局が引き継がれます

schedule-on-air = { $time } UTC に { $count } 局が放送中

schedule-now = 現在

schedule-solar-hover = { $site } の現地太陽時 { $local }、{ $lon }°

schedule-favourite-hint = この局をお気に入りにする

rc-radio-classic = RADIO CLASSIC

rc-bands = バンド

rc-mem-programs = メモリー · 番組

rc-recall-band-mode-frequency-and = 呼び出し: バンド、モード、周波数、フィルターを一度に

rc-no-memories-yet-store-one-with-mem = メモリーはまだありません。上部バーのMEMで保存してください

rc-broadcast-schedule-what-is-on-when = 放送時刻表 — 何がいつどこで放送されているか

rc-bw-this-mode-has-one-fixed = BW — このモードの通過帯域は固定です

rc-bw-click-next-filter-right-click = BW — クリック: 次のフィルター · 右クリック: 前へ · ホイール: ステップ

rc-step = ステップ

rc-step-click-next-tuning-step-right = ステップ — クリック: 次の同調ステップ · 右クリック: 前へ · ホイール: ステップ

rc-agc-click-next-speed-right-click = AGC — クリック: 次の速度 · 右クリック: 前へ · ホイール: ステップ

rc-open = 開放

rc-sql-click-open-back-to-the-last = SQL — クリック: 開放 / 前回のしきい値に戻す · ホイール: ゲートを上下

rc-s-meter = Sメーター

rc-wheel-on-a-digit-click-upper-lower = 桁の上でホイール · 上半分/下半分をクリックで ± · 右クリックでゼロ · ダブルクリックで入力

rc-music = 音楽 ♪

rc-speech = 音声

rc-stereo-the-19-khz-pilot-is-locked = ステレオ — 19 kHzパイロットをロック中 — クリックでモノラル

rc-mono-the-broadcast-stereo-decode = モノラル — 放送ステレオのデコードはオフです — クリックでオン

rc-mono-noise-reduction-is-on-and = モノラル — ノイズリダクションがオンで、ステレオはそちらが優先されます

rc-mono-no-19-khz-pilot-the-station = モノラル — 19 kHzパイロットなし: 局がモノラル送信か、信号が弱すぎます

rc-stereo = ステレオ

rc-group-broadcast = 放送

rc-group-ham = アマチュア

rc-group-air = 航空

rc-group-marine = 海上

rc-cap-lw-mw-fm = LW / MW · FM

rc-cap-shortwave = SW · 短波

rc-cap-amateur-bands = アマチュアバンド

rc-cap-airband = 航空バンド

rc-cap-marine = 海上VHF · MF

rc-cap-cb = 市民バンド

rc-preset-fm-ext = FM拡張 64–87.5

rc-mem-n = MEM { $id }

controls-action-tune = VFO同調

controls-action-rit-offset = RITオフセット

controls-action-xit-offset = XITオフセット

controls-action-volume = ボリューム

controls-action-sub-volume = サブRXボリューム

controls-action-squelch = スケルチ

controls-action-filter-width = フィルター幅

controls-action-filter-shift = フィルターシフト

controls-action-agc-max-gain = AGC最大ゲイン

controls-action-manual-gain = 手動ゲイン（AGCオフ）

controls-action-tx-drive = TXドライブ

controls-action-tune-drive = チューンドライブ

controls-action-mic-gain = マイクゲイン

controls-action-digi-audio-freq = デジタル音声オフセット

controls-action-spectrum-zoom = スペクトラムズーム

controls-action-spectrum-pan = スペクトラムパン

controls-action-spectrum-floor = スペクトラム下限

controls-action-spectrum-ceiling = スペクトラム上限

controls-action-ptt = PTT

controls-action-tune-carrier = TUNEキャリア

controls-action-mute = ミュート

controls-action-noise-blanker = ノイズブランカー

controls-action-nr-cycle = ノイズリダクション(切替)

controls-action-auto-notch = オートノッチ

controls-action-binaural = バイノーラル音声

controls-action-agc-cycle = AGC(切替)

controls-action-sub-rx = サブ受信機

controls-action-split = スプリット

controls-action-rit-enable = RIT オン/オフ

controls-action-xit-enable = XIT オン/オフ

controls-action-rit-clear = RIT クリア

controls-action-xit-clear = XIT クリア

controls-action-vfo-toggle = VFO A/B 切替

controls-action-swap-vfos = A↔B 入替

controls-action-copy-a-to-b = A→B コピー

controls-action-band-up = バンドアップ

controls-action-band-down = バンドダウン

controls-action-mode-next = 次のモード

controls-action-mode-prev = 前のモード

controls-action-record-toggle = 録音 オン/オフ

controls-action-abort-tx = 送信中止

controls-action-cw-straight = CW ストレートキー

controls-action-voice-stop = ボイスキーヤー停止

controls-action-tone-burst = 1750 Hz トーンバースト

controls-action-fit-span = スパンに合わせる

controls-action-zoom-in = ズームイン

controls-action-zoom-out = ズームアウト

controls-action-peak-hold = ピークホールド

controls-action-spectrum-collapse = スペクトラムを折りたたむ

controls-action-waterfall-collapse = ウォーターフォールを折りたたむ

controls-action-waterfall-flip = ウォーターフォール反転

controls-action-toggle-help = マニュアルウィンドウ

controls-action-toggle-settings = 設定ウィンドウ

controls-action-toggle-logbook = ログブックウィンドウ

controls-action-toggle-spots = スポットウィンドウ

controls-action-toggle-memories = メモリーウィンドウ

controls-action-toggle-voice = ボイスキーヤーウィンドウ

controls-action-workspaces = ワークスペースウィンドウ

controls-action-speak-status = 状態を読み上げ

controls-action-speak-repeat = 直前のアナウンスを繰り返す

controls-action-speech-silence = 読み上げ停止

controls-action-speech-toggle = アナウンス オン/オフ

controls-action-broadcast-labels = 放送局ラベル(同調中 / バンド全体)

controls-action-select-vfo = VFO { $vfo } を選択

controls-action-band = バンド { $band }

controls-action-mode = モード { $mode }

controls-action-recall-memory = メモリー { $n } を呼出

controls-action-voice-slot = ボイスキーヤー スロット { $n }

controls-group-tuning = 同調

controls-group-receive = 受信

controls-group-transmit = 送信

controls-group-display = 表示

controls-group-windows = ウィンドウ

controls-group-speech = 音声案内

controls-midi-note = ノート

controls-midi-cc = CC

controls-midi-pitch-bend = ピッチベンド

controls-midi-program = プログラム

controls-midi-any = 任意

controls-midi-msg = { $kind } { $data }  ch { $ch }

controls-midi-msg-pitch-bend = ピッチベンド  ch { $ch }

settings-controls-down = ダウン

settings-controls-rev = 逆

antenna-antenna = アンテナ

antenna-cutting-lengths = 切断長

antenna-the-frequency-to-cut-for = 切断する対象の周波数

antenna-optimizing = 最適化中…

antenna-tune-the-yagi-s-element-lengths = 選択した目的に対して、八木アンテナのエレメント長と間隔を調整します。バックグラウンドスレッドで山登り法による局所探索を行うため、実行中もウィンドウは使えます。

antenna-solved = 解決済み

antenna-optimized-yagi = 最適化した八木

antenna-obstacles-approximate = 障害物(概算)

antenna-trees = 樹木

antenna-in-leaf = 葉あり

antenna-ridge = 尾根

dab-channel-received-mhz = 受信チャンネル { $channel } { $mhz }

dab-no-channel-received = 受信チャンネルなし

dab-channel-received = 受信チャンネル { $channel }

dab-slideshow = スライドショー

dab-no-picture-yet = 局からまだ画像が送られていません

dab-no-station-chosen = 局が選択されていません

dab-no-logo-yet = ロゴはまだありません

dab-playing = ▶ 再生中

dab-chosen = 選択中

dab-no-text-yet = 局からまだテキストが送られていません

dab-clear-channel-hint = このチャンネルとその局を消去します。SCAN でまた見つかります。

dab-clear-all-channels = × 全チャンネルを消去

dab-clear-all-channels-hint = すべてのチャンネルとその局を忘れます。SCAN でまた見つかります。スターは残ります。

dab-n-stations = { $n } 局

dab-scanning-progress = スキャン中 { $channel } · { $at }/{ $n } · { $found } 件検出

dab-clear-station-hint = この局をリストから消去します。アンサンブルにまだ含まれていれば、次の SCAN で戻ります。

dab-prev = 前へ

dab-prev-hint = リストの前の局。

dab-preset = プリセット

dab-preset-hint = この局にスターを付ける、または外します。

dab-next = 次へ

dab-next-hint = リストの次の局。

dab-pty-hint = リストに各局の番組タイプを表示します。

dab-stop-sweep-hint = ここでスイープを停止します。それまでに見つかった分は残ります。

dab-scan-hint = Band III の全ブロックを調べ、局があるものを残します。

swl-title = SWL ログ

swl-new = + 新規

swl-jobs = ジョブ

swl-jobs-hint = 予約録音 — 指定した時刻にバンドを録音します

swl-sigid = 信号識別

swl-sigid-hint = このダイヤルにいるのは何? 周波数・モード・帯域幅から信号を調べるガイド

swl-replay = リプレイ

swl-replay-hint = ライブではなく直前の2分間を再生 — 聞き逃した局のIDを確認できます

swl-report = レポート

swl-report-hint = この受信内容を局へ送るレポートとしてコピーします

swl-stats-heard = { $n } 受信

swl-stats-reported = { $n } 報告済み

swl-stats-qsl = { $n } QSL

swl-stats-pirates = { $n } 海賊局

swl-stats-hint = ログ全体 — 「表示」の絞り込みでは変わりません

swl-antenna = アンテナ

swl-antenna-hint = ロングワイヤー 20 m、MLA-30 ループ、ミニウィップ …

swl-antenna-tip = 受信レポートの Antenna: 行に入ります

swl-tone = トーン

swl-tone-on = オン

swl-tone-bass = 低音

swl-tone-mid = 中音

swl-tone-treble = 高音

swl-report-copied = レポートをクリップボードにコピーしました — メールソフトに貼り付けてください。メールハンドラを開けませんでした。

swl-show = 表示

swl-find-hint = 局、言語、サイト …

swl-find-tip = 局、言語、送信所、メモに一致します

swl-all-bands = 全バンド

swl-all-days = 全日

swl-pirates-only = 海賊局のみ

swl-pirates-only-tip = 無免許の局のみ

swl-adif-tip = ログ全体を ADIF で保存 — 交信ではなく受信記録です

swl-csv-tip = ログ全体を CSV で保存、受信1件につき1行

swl-shown-of = { $shown } / { $n }

swl-empty = まだ記録がありません — 局に同調して + 新規 を押してください。

swl-transmitted-from = 送信所: { $site }

swl-click-to-copy = (クリックでコピー)

swl-addr-btn = 住所

swl-mail-btn = メール

swl-mark-sent = 送信済

swl-recall = 呼出

swl-recall-tip = この周波数とモードに戻す — 局が戻ってきたか確認します

swl-edit = 編集

swl-del = 削除

swl-no-match = 絞り込みに一致する受信がありません — CLEAR で全て表示します。

swl-form-new = 新規受信

swl-form-edit = 受信を編集

swl-station-hint = Radio Taiwan International

swl-language-hint = 英語

swl-received = 受信地

swl-received-hint = グリッド

swl-email = メール

swl-email-hint = 受信レポート用

swl-email-tip = レポートの Send to: 行に入ります

swl-postal = 郵送先

swl-postal-hint = 番地、市、国

swl-postal-tip = メールがない場合の代替レポート送付先

swl-heard = 受信日時

swl-smeter = Sメーター

swl-report-label = レポート

swl-judged = 判定

swl-report-sent = レポート送信

swl-not-yet = 未

swl-qsl-received = QSL 受領

swl-awaiting = 待機中

swl-notes = メモ

swl-notes-hint = 番組メモ

swl-pirate = 海賊局

swl-pirate-tip = 無免許放送 — 割り当て外で送信している局。ログでは海賊旗で示されます。

swl-mail-why-no-address =
    この局にはメールアドレスがありません — SCHEDULE ウィンドウで調べるか、ここに入力すると使えるようになります。

    受信レポートは局が読んで QSL カードで返信するための礼儀です。宛先がなければ送る先がありません。

swl-mail-why-bad-address = そのメールアドレスは単一のアドレスではないため使用されません — 局を受信していない他人にレポートが届くなら、送らないほうがましです。

swl-mail-why-ok =
    受信レポートを作成し、メールクライアントで開きます。

    画像と音声クリップはそちらで添付してください — このプログラムはメッセージにファイルを添付できません。

swl-mail-report = レポートをメール…

antenna-feed-ohms = 給電点 ≈ { $r } Ω

antenna-takeoff = 打ち出し角 { $deg }°

antenna-director = 導波器 { $n }

antenna-reflector = 反射器

antenna-driven = 放射器

antenna-gap = 間隔 { $v } m

antenna-az = 方位

antenna-depth = 奥行

antenna-height = 高さ

antenna-attenuation = 0°方向の減衰: { $att } dB

morse-no-paddle-found-connect-one-then = パドルが見つかりません — 接続してからこのペインを開き直してください

morse-send-with-a-paddle-played-here = パドルで送信 — ここで再生され、電波には出ません

morse-no-paddle-found-in-dev-input-by-id = /dev/input/by-id にパドルが見つかりません。接続してこのペインを開き直してください。2つの接点を報告するキーヤーボックス(多くはマウスのボタンとして)は使えます。自前でイアンビックキーイングを行いタッチを送るものは使えません。

morse-reverse = 反転

morse-swap-dit-and-dah-the-switch-on = ツーとトンを入れ替えます。多くのパドルでは本体のスイッチで行えます。これはスイッチが反対の位置にあるときのためのものです。

morse-stop-key = キー停止

morse-start-key = キー開始

morse-key-the-character-above-the-keyer = 上の文字をキーイングします。キーヤーがトンとツーをきれいに作ります — どのパドルか、間をどれだけ取るかはあなたが決めます。タップで1エレメント、長押しで連続します。

morse-the-sending-drill-needs-a-linux = 送信練習には Linux デスクトップとパドルが必要です。

panels-float-chip-hint = フロート — このパネルをフローティングパネルとして切り離し、ウォーターフォールに場所を戻します。パネル上のドックチップで元に戻せます

panels-map-style-radar = レーダー — 暗い地面、陸地は塗りつぶし。航空状況を読むための地図

panels-map-style-terrain = 地形 — 陸地を持ち上げて暖色にし、地面らしく見せ、国境は控えめに

panels-map-style-chart = チャート — 淡い陸地、際立つ国境と経緯線、航空図の見た目

panels-heard-me-chip = 受信された

panels-heard-me-hint = 現在のバンドでこのコールサインを受信したと報告した局を、PSK Reporter から取得し、報告者自身のグリッドに丸で表示します。PSK スポットの逆で、あちらはこの無線機が受信している局です。1時間ごとに更新され、直近1時間を対象とします。設定 → スポット で PSK フィードをオンにし、一般タブでコールサインを設定しておく必要があります。

panels-period-label = 周期

panels-sub-mode-label = サブモード

panels-front-end-overloading = ⚠ フロントエンド過負荷 — RFゲイン(LNA/IF)を下げるか、アンテナを移動してください

panels-front-end-overloading-hint = 受信機がフルスケールを超えて駆動されているため、音声は大きいものの信号が含まれず、この状態ではデコードできません。RF/LNA ゲインを下げ(減衰を増やす)、IF ゲインを下げるか、AGC を有効にしてください。近くの強力な送信機や、同じバンドの別の無線機が通常の原因です。

panels-enigma-hint = エニグマ機 — 戦時中のパネルでメッセージを暗号化し、電波で受信した暗号文を解読します

panels-slot-bar-heard-hint = スピーカーの音に追従します。フレームが聞こえている間は白、止まると緑、次のフレームが始まると最初に戻ります。時間はモードごとのもので、短いフレームでもバーは満たされます。


morse-send-correct = 正解: { $want }

morse-send-heard = 受信 { $got }、正解 { $want }

morse-send-received = 受信: { $text }

amp-title-model = AMP · { $model }

amp-state-off = オフ

amp-state-on = オン

amp-relay-dn = DN

amp-relay-up = UP

amp-band-unknown = 不明

amp-needle-top = TOP

decodes-not-uploaded = ⚠ { $services } に未アップロード

decodes-not-uploaded-hint = 交信はローカルログにありますが、アップロードに失敗しました。設定 → スポット でトークンと接続を確認し、ログブックからもう一度アップロードしてください。

decodes-logged-to = ↑ { $services } に記録済み

decodes-logged-to-hint = 新しい交信をすべて受け付けるよう設定したオンラインログブックにもアップロードされました。

decodes-known = 既知

decodes-known-hint = ハッシュ化された <...> が現在解決できるすべてのコールサインと国。ハッシュは逆引きできません — 局がコールサインを綴って送信するのを受信して初めて表示されます。11 m の短縮コールはそのためのものです。

decodes-ignored-clear = { $n } 無視中 · クリア

decodes-auto-tx-frequency-hint =
    送信周波数を自動で選びます。応答相手の周波数ではなく、こちらが送信する周期で最も静かな場所です。相手は逆の周期で送信するため、相手の周波数はこちらが送信するときにそこに誰がいるかを示しません。オフにしても周波数は固定されません。呼んでいる局の周波数で応答します。固定するには Hold TX を使ってください。

    1つのバンドは例外です。11 m では WSJT-CB の慣例として CQ に対して呼び出し側のトーンで応答し、交信中ずっとそこに留まるため、探索は行われず、CQ への応答は受信した周波数のままになります(オン/オフに関係なく)。

decodes-muted-caps = ミュート

decodes-muted-hint = このセッションではミュート中 — クリックでまた聞こえます

decodes-ignore-hint = セッションの残りの間この局を無視します。デコードリストから消え、アラートも鳴らず、自動モードの対象にもなりません。この局のために何も送信・アップロードされません。sdroxide を再起動すると戻ります。

decodes-qso-complete-hint = 交信は完了し、ローカルログに記録済みです。相手が最後のメッセージを繰り返した場合に再送できるよう、数分間ここに保持されます — それ以上の義務はありません。

decodes-slot-even = 偶数

decodes-slot-odd = 奇数

decodes-before = 前

decodes-after = 後

decodes-beam-hint = アンテナをこの局のロケーターに向けます

decodes-queue-next = 次

decodes-queue-waiting = 待機


decodes-setup-hint = セットアップ — これらの QSO モードが共有する局の識別情報とテンプレート

decodes-auto-armed-hint =
    { $role } は待機中です。クリックで停止します。

    { $note }

    エンジンの段階: { $step } — CQ に応答する局は、交信中でない CallingCq のときだけ受け付けられます。

decodes-auto-rx-hint = 新しい局の CQ に無人で応答し、自分では CQ を出しません。ログにないコールサインにのみ応答します。送信ウォッチドッグがペースを管理し、無操作停止が解除します。

decodes-auto-continuous-hint = ワークフロー全体を無人で実行します。QSO を完了し、呼んできた局と交信し、CQ を出している局に応答し、他に何もないときだけ CQ を出します。停止するまで実行されます{ $until }。{ $bound }

decodes-auto-or-minutes-up = { " " }または { $n } 分経過するまで

decodes-auto-cq-run-unlimited = 各 CQ 連続送信には独自の呼出回数制限がありません。

decodes-auto-cq-run-bounded = 各 CQ 連続送信は独自の呼出回数で区切られます。

decodes-auto-no-call-limit = 独自の呼出回数制限なし — ウォッチドッグと無操作停止で終了します。

decodes-auto-after-calls-listen = { $repeats } 回呼んだ後、AUTO RX に切り替わり受信を続けます。

decodes-auto-after-calls-stop = { $repeats } 回呼んだ後、停止します。

decodes-auto-cq-hint = 無人で CQ を出し、応答した局と交信します。{ $bound } 短すぎると静かなバンドでは聞こえず、長すぎると誰も応答しない周波数で送信機が占有されます。

decodes-auto-stopped-by-operator = { $role } はオペレーターにより停止されました

decodes-cont = 継続

settings-servers-decoded-text-udp = UDP でデコードテキスト出力

settings-servers-decoded-text-udp-note = デコードごとに1行の JSON を出力します。自作スクリプト向けで、FT8 系、WSPR、各キーボードモード(CW、RTTY、PSK、Olivia、THOR、MFSK、DominoEX、Hell)のデコード行に、受信したダイヤル周波数が付きます。上の WSJT-X ブロードキャストはロガー向けのプロトコルで、これらのモード向けのメッセージがないため、別に用意されています。

settings-servers-decoded-text-udp-hint = host:port、UDP。空欄はオフ(既定)。出力専用で、このソケットから無線機を操作することはできません。名前は APPLY を押したときに解決されます。

settings-servers-rotator-note = ロックしている衛星、または ROTATOR ウィンドウや地図から手動で、電動アンテナを向けます。既定は Hamlib の rotctld デーモンです(GS-232B ローテーターなら例えば「rotctld -m 603 -r /dev/ttyUSB0」、ハードウェアなしで試すなら「rotctld -m 1」)。1つのデーモンで Hamlib が扱うすべてのローテーターに対応します。シリアルポートのコントローラーも直接制御できます。

settings-servers-connection = 接続

settings-servers-rotctld-port-hint = rotctld の既定は 4533

settings-servers-baud-hint = ほとんどのコントローラーは 9600

settings-servers-pstrotator-note = PSTRotator 独自の UDP ポートです。その Setup で UDP Control をオンにしてください。このポートでコマンドを受け、次のポートで応答します。

settings-servers-el = 仰角

settings-servers-persist-re-connect-rotator = 保存してローテーターに(再)接続

settings-servers-tci-note = TCI 対応プログラムからこの無線機を操作できます。周波数・モード制御、広帯域 IQ、受信音声、送信音声に対応します。WSJT-X ではリグタイプに SunSDR (TCI) を選び、このアドレスを指定し、PTT を TCI に設定してください。

settings-servers-decoded-text-udp-format = 各行は1データグラムで、改行で終わります: { $example }

settings-servers-transport-rotctld = Hamlib rotctld (TCP)

settings-servers-transport-easycomm = EasyComm II (シリアル)

settings-servers-transport-gs232 = Yaesu GS-232 (シリアル)

settings-servers-transport-pstrotator = PSTRotator (UDP)

spots-active = アクティブ

spots-closing = クローズ間近

spots-none-left-in-the-window = ウィンドウ内に残りなし

spots-openings = オープニング

spots-band-opening-detections-paths = バンドオープニング検出: 直近の活動が自身の3時間ベースラインを超えて急増した経路

spots-save-the-spots-you-can-see-as-a = 表示中のスポットを KML ファイルとして保存します — Google Earth や KML を読めるその他の地図ツールで使えます。オンのフィルターに該当し、かつ位置情報を持つスポットのみです。座標のないピンは世界の果てに立ってしまうためです。

spots-no-spots-to-save-the-filters-are = 保存するスポットがありません — フィルターがすべてオフです

spots-bc-all = BC 全て

spots-bc-tuned = BC 同調中

spots-which-broadcasters-the-waterfall =
    { "**" }ウォーターフォール**に名前を表示する放送局。

    { "**" }BC 同調中**は既定で、bebepip の規則です。同調中の局の名前だけを表示します。バンド全体にラベルを付けると、読みたい通過帯域の上が名前の壁になってしまいます。

    { "**" }BC 全て**は、スケジュール上で現在放送中のすべての局を場所を問わず表示します。1つの局を聴くのではなくバンドを見て回るときに向いています。上の BC チップが放送局を表示するかどうかには影響しません。

app-rx-only-hint = この無線機は受信専用です — キーイングする送信機がありません。

spots-n-calls = { $n } コール

spots-opening-hover = { $from } から { $to } への { $band } バンド経路: 直近15分のウィンドウが経路の3時間ベースラインを超えて急増しました — 最近のコール: { $calls }


spots-download-unsupported = このプラットフォームでは { $source } をダウンロードできませんでした

spots-kml-partial = { $total } 件中 { $placed } 件を保存 — 残りは位置情報がなくピン留めできません

spots-net-log = ネットログ ({ $n })

decodes-beam = ビーム

settings-radio-no-answer-yet = 無線機が接続されているマシンの応答待ちです。これらはそのハードウェアとネットワークを問い合わせるもので、応答は1件ずつ返ります。

settings-radio-elad-no-baud = この無線機には { $baud } ボーの設定がありません — 代わりに { $default } を使用します

settings-radio-elad-antenna-rx-only = 受信専用

settings-radio-rx-only-n = RX{ $n } のみ

settings-radio-ng-level = nG レベル

settings-radio-ng-level-hint =
    nG は 2.00x ファームウェアにない2つのコマンドを追加し、設定できるのはここだけです。nG はどちらにも応答せず保存もしないため、sdroxide はリンクが開くたびに再送します。

    ボリュームは無線機が USB ストリームに送り出すレベルです。nG はこの音声を VOL と AGC の後で取り込むため、デコーダーの入力が小さすぎる・大きすぎる場合の調整はコンピューターのミキサーではなくここで行います。

    ゲインコントロールは無線機自身の AGC 設定です。Auto ではデジタルモードのとき DIGI、それ以外ではオンにします。nG の FT8 向けの注意書きに沿ったもので、相手局の送信の合間にゲインが上がってしまわず保持されます。

settings-radio-ng-volume-hint = 0–31、`AG0nn;` として送信。無線機がコンピューターへストリーミングするレベル。

settings-radio-gain-control = ゲインコントロール

settings-radio-speaker = スピーカー

settings-radio-on-while-streaming = ストリーミング中はオン

settings-radio-ng-speaker-hint = オフにすると、ストリーミング中は無線機自身のスピーカーを切ります(`UA2;`)。音声は sdroxide でのみ聞こえます。2.00x にはこの形式がありませんでした。

settings-radio-ng-agc-auto = Auto (データモードでは DIGI)

settings-radio-ng-agc-off = オフ

settings-radio-ng-agc-on = オン (高速)

settings-radio-sound-card-note = この無線機の音声を扱うサウンドカードは、お使いのスピーカーやマイクと同じ **オーディオ** にあります。どの無線機を使っても同じ組の設定なので、1ページにまとまっています。

settings-radio-no-control-cable-audio-page = この無線機には制御ケーブルがありません — VOX で自動的にキーイングするため、操作する PTT はありません。送信は無線機のマイクへの音声入力で行うため、送信音声の設定はここではなくオーディオページにあります。

settings-radio-sound-card-note-short = 音声を扱うサウンドカードは **オーディオ** にあります。

settings-radio-control-link = 制御リンク

settings-radio-atsmini-host-hint =
    受信機のアドホック TCP サーバー: `atsmini.local` (mDNS) またはその IP。無線機側で Settings → TCP Port → Ad hoc、Wi-Fi は Connect、AP+Connect、AP Only のいずれかにします。

    `atsmini.local` にはこのマシンに mDNS リゾルバー(Avahi/nss-mdns)が必要です。制御リンクが確立しない場合は、無線機の Wi-Fi 画面に IP が表示されるので、それを入力してください。

settings-radio-atsmini-find-hint = このマシンのネットワークで受信機の制御ポートを検索します

settings-radio-tcp-port = TCP ポート

settings-radio-tcp-port-hint = ファームウェアの既定は 60000 です。

settings-radio-atsmini-receive-only = 受信専用: Si4732 はハードウェアで復調するため、ここで選んだモードは無線機に指示として送られ、それ以外(検波、デコード、ログ)はすべてこのコンピューターで行われます。

settings-radio-atsmini-tuning-note =
    同調は受信機自身のバンド切替に従うため、スクロールに対してダイヤルが少し遅れます。周波数が別のバンドに移るときが最も遅れます。落ち着くのを待ってからスクロールを続ければ、必ず指定した周波数に着地します。

    受信機のチューニングステップは受信機固有のもの(上の STEP コントロール)です。それより細かい変更は無線機側で丸められ、ダイヤルは実際に着地したステップに戻ります。

settings-radio-live-controls = ライブコントロール

settings-radio-atsmini-vol = VOL

settings-radio-atsmini-step = STEP

settings-radio-atsmini-sent-at-once = すぐに無線機へ送信されます。VOL は受信機自身のボリューム(そのつまみ、および Si4732 のもの)で、sdroxide の AF スライダーではありません。

settings-radio-refresh = 更新

settings-radio-read-99-slots = 受信機から99スロットを読み込みます

settings-radio-tune-radio-slot = この無線機をスロットに同調

settings-radio-no-slots-stored = 受信機にスロットが保存されていません。

settings-radio-press-refresh-slots = 更新を押して受信機のメモリースロットを読み込みます。

settings-radio-icomnet-receive-hint = AF: 無線機が復調し、sdroxide は音声帯域を表示します。12 kHz IF: 無線機が DRM の中間周波数を送り、sdroxide が復調します。これにより sdroxide 独自のフィルター、ノイズリダクション、デコーダーが約 ±12 kHz にわたって使えます。どちらの場合も広いウォーターフォールは無線機自身のスコープで、Icom は I/Q を出力しません。

settings-radio-copy-icom-session-hint = この無線機の直近のセッション — ハンドシェイクと CI-V トレース — をクリップボードにコピーします(ページからのコピーを許可しないブラウザではファイルに保存)。バグ報告用です。ステーション全体ではなくこの無線機のもので、LAN 上に Icom が2台ある場合は各タブが自身のアドレスについて答えます。

settings-radio-copy-protocol-trace-hint = 直近のセッションのプロトコルトレースをクリップボードにコピーします(ページからのコピーを許可しないブラウザではファイルに保存)。バグ報告用です。

settings-radio-copy-trace-device-hint = 直近のセッションのトレースをクリップボードにコピーします(ページからのコピーを許可しないブラウザではファイルに保存)。バグ報告用で、デバイスとやり取りしたすべてのコマンドとサンプルストリームの先頭バイトを含みます。

settings-radio-copy-trace-receiver-hint = 直近のセッションのトレースをクリップボードにコピーします(ページからのコピーを許可しないブラウザではファイルに保存)。バグ報告用で、受信機とやり取りしたすべてのコマンドとサンプルストリームの先頭バイトを含みます。

settings-radio-pluto-choose-receiver = 受信機を選択してください

settings-radio-dab-rate-hint = DAB/DAB+ をデコードするには 2.500 Msps を選択してください

settings-radio-rx888-dab-rate-hint = DAB/DAB+ をデコードするには 80.0 Msps と 2.500 MHz 幅 (1/32) を選択してください

settings-radio-pluto-both-chains-hint = 両方のアンテナで聴くには、同じアドレスで2台目の無線機を追加して RX2 に設定してください。両方の無線機は同じ周波数を共有します。

settings-radio-duplex-tdd-hint =
    AD9361 が両方向を同時に動かす(FDD、Pluto の起動時の状態で sdroxide が常に維持してきた設定)か、一方向ずつ動かす(TDD)かを選びます。

    下の PTT ピンを使いたい場合を除き、FDD のままにしてください。PTT ピンは TDD でキーイングされ、上の全二重とは両立しません。Apply で反映されます。

    PlutoSDR: 全二重はオフ — TDD は一度に一方向のみを有効にし、GPO の送受信ピンはそれでキーイングされます。ターンの間も聴くには、PTT ピンをオフ、デュプレックスを FDD にしてください。

settings-radio-fm-mw-notch = FM/MW ノッチ

settings-radio-fm-mw-notch-hint = FM 放送(88–108 MHz)と中波帯に対するハードウェアノッチ。近くの送信所が他のすべてを過負荷にするときに使います。すぐに反映されます。

scanner-broadcast-band = 放送バンド

scanner-all = 全て

scanner-stop-only-on-channels-that-sound = 誰かが話しているように聞こえるチャンネルでのみ停止します。音声は周期的で、音声帯域を満たし、毎秒数回の強弱があり、キャリア、バーディー、データバーストにはそれがありません。各候補は少なくとも0.5秒聴きます。音節を聞き取るのに必要な時間だからです。

scanner-nothing-in-this-scan-is-in-a-mode = このスキャンには音声を扱うモードがありません — ここで音声探索を行ってもどこにも止まらないため、あるまでは選択できません。FM、AM、サイドバンドは音声を扱い、デジタルモードは扱いません。

scanner-voice = 音声

scanner-stop-only-where-somebody-is = 誰かが話している所でのみ停止

scanner-no-voice-to-look-for-in-these = これらのモードでは探す音声がありません

scanner-sql = SQL

scanner-how-long-to-stay-on-a-candidate = 判定する前に候補に留まる時間。音声判定にはこの長さが最低限必要です。音声は音節のリズムで、0.5秒でその2つ分になります

scanner-listening-for-a-voice = 音声を探して聴取中…

scanner-scan-band-hover = { $name } をスキャン ({ $lo }-{ $hi } kHz)

scanner-range-to = ～

scanner-mode-no-speech = { $mode } は音声を扱いません — これらのチャンネルを飛ばします

scanner-voice-holding = 音声 { $score } · 誰かが話しています

scanner-voice-passed = 通過 { $mhz } MHz · 音声 { $score }

aircraft-hfdl-aircraft = HFDL 航空機 { $id }

aircraft-no-icao = ICAO アドレスなし (HFDL)

aircraft-provenance = { $source }、{ $age } 前

aircraft-label-flight = 便名

aircraft-label-registration = 登録記号

aircraft-label-position = 位置

aircraft-label-altitude = 高度

aircraft-label-speed = 速度

aircraft-label-track = 航跡

aircraft-label-last-message = 最後のメッセージ

aircraft-label-type = 機種

aircraft-label-operator = 運航者

aircraft-label-origin = 出発地

aircraft-label-destination = 目的地

aircraft-label-online = オンライン

aircraft-missing-flight = 未受信 — ADS-B または VDL2 で得られます

aircraft-missing-registration = 未受信 — VDL2 でのみ得られます

aircraft-missing-position = 未受信 — ADS-B、または VDL2 か HFDL の位置報告

aircraft-missing-adsb = 未受信 — ADS-B で得られます

aircraft-missing-message = まだなし — VDL2、または VDL2 が機体名を判明させた後の ACARS 音声

aircraft-online-unavailable-disabled = オンライン利用不可 — オンラインスイッチがオフです

aircraft-online-unavailable-no-station = オンライン利用不可 — 問い合わせ先のステーションがなく、無線データのみです

aircraft-online-unavailable-unauthorized = オンライン利用不可 — ステーションにサインインしていません

aircraft-online-unavailable-retry = オンライン照会に失敗 — まもなく再試行します

aircraft-switch-no-answer = ステーションが応答しませんでした

aircraft-switch-not-signed-in = ステーションにサインインしていません

aircraft-switch-refused = ステーションがリクエストを拒否しました

aircraft-switch-too-old = ステーションにこのスイッチがありません(バージョンが古い?)

aircraft-switch-failed = ステーションが変更できませんでした

aircraft-online-source = adsbdb(ステーション経由)

aircraft-asking-station = ステーションに問い合わせ中

aircraft-provenance-radio = 無線

aircraft-provenance-radio-online = 無線 + オンライン

aircraft-provenance-online = オンライン

aircraft-photo-credit = 写真: { $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = アンテナローテーターが設定されていません。

rotor-set-one-up-in-settings-servers-a = 設定 ▸ サーバー で設定してください — Hamlib rotctld デーモン、またはシリアルポートのコントローラーです。その後、このダイヤル、コールサイン、または地図からビームを向けられます。

rotor-halt-the-antenna-where-it-is = アンテナをその場で停止

rotor-park = パーク

rotor-hand-the-antenna-back-to-the = アンテナを衛星ロックに戻します — 衛星を追尾し、衛星がないときはパークします。

rotor-point = 向ける

rotor-dx-country = DX 国

rotor-set-your-grid-locator-settings = 国に向けるには、グリッドロケーター(設定 ▸ 一般)を設定してください。

rotor-connected = 接続済み — アンテナ { $az }°

rotor-not-connected-why = 未接続: { $e }

rotor-beam = ビーム 方位 { $az }° 仰角 { $el }°

rotor-target = 目標 { $az }° / { $el }°

rotor-park-hover = パーク方位 ({ $az }° / { $el }°) へ回す

rotor-country-bearing = { $name } — 方位 { $az }°

aircraft-switch-no-station = 切り替えるステーションがありません

aircraft-title = 航空機

aircraft-mode-adsb-hover = ADS-B、1090 MHz — 位置、高度、速度付きの航空機

aircraft-mode-hfdl-hover = HFDL、HF データリンク — 洋上や極地上空の航空機を位置報告から

aircraft-mode-vdl2-hover = VDL2、VHF データリンク — 航空機からのメッセージ。位置を含むものは少数

aircraft-mode-acars-hover = ACARS — 航空機と地上間の短いメッセージ

aircraft-badge-online = オンライン

aircraft-badge-history = 履歴

aircraft-stats-total = { $n } 機

aircraft-stats-most-first = 多い順

aircraft-stats-empty = まだ航空機がありません。表は ADS-B、VDL2、HFDL、ACARS から埋まります。ここではインターネットは不要です。

aircraft-stats-relay-note = 機種、運航者、経路は、リレーが回答した航空機のみを数えます — Cards でカードを開いて照会してください。

aircraft-stats-known-kinds = { $known } 既知 · { $kinds } 種類

aircraft-and-more = ほか { $more } 件

aircraft-show = 表示

aircraft-aviation = 航空

aircraft-cards-chip = カード { $n }

aircraft-cards-hover = ADS-B、VDL2、HFDL、ACARS で受信したすべての航空機を1機1枚のカードで表示 — 現在の無線機のモードに関係なく10分間保持されます

aircraft-watch-chip = ウォッチ { $n }{ $suffix }

aircraft-watch-hover = コールサインまたは登録記号で通知を依頼した航空機 — リストはステーションが保持し、該当機が電波に出るとアラートが表示されます

aircraft-stats-chip = 統計

aircraft-stats-hover = このステーションが把握している航空機の、最も多いカテゴリー、航空会社、機種、運航者、経路 — まず無線から集計し、リレーのカードで補います

aircraft-export = エクスポート

aircraft-export-csv-hover = 航空機をスプレッドシート用の CSV 表として保存します。履歴があればステーションの直近24時間、なければ現在受信中のもの。位置は含みません。

aircraft-export-json-hover = 航空機を JSON として保存します。履歴があればステーションの直近24時間、なければ現在受信中のもの。位置は含みません。

aircraft-watch-intro = 通知を受けたいコールサインと登録記号。リストはステーションが保持するため、どの画面でも同じです。該当機が現れたとき、または10分間電波から消えた後に戻ったときにアラートが出ます。

aircraft-watch-needs-station = ウォッチリストは接続中のステーションが保持します。使うにはステーションに接続してください。

aircraft-watch-reading = リストを読み込み中…

aircraft-watch-cannot-read = リストを読み込めません: { $why }。

aircraft-watch-input-hint = KLM1234 または PH-BXA

aircraft-watch-add = 追加

aircraft-watch-sending = 送信中…

aircraft-watch-empty = リストはまだ空です。

aircraft-watch-on-air = 現在電波に出ています

aircraft-watch-remove = 削除

aircraft-watch-alerts = アラート

aircraft-watch-clear = クリア

aircraft-watch-none-yet = まだありません。

aircraft-cards-empty = まだ航空機を受信していません。カードは ADS-B、VDL2、HFDL、ACARS から埋まります — どれかを押して聴いてください。ここではインターネットは不要です。

aircraft-cards-no-match = 「{ $query }」に一致する航空機はありません。

aircraft-history-hover = ステーション自身の記録から、この航空機の直近1日の軌跡を表示します。問い合わせ先はステーションで、インターネットではありません。

aircraft-copy-summary = 概要をコピー

aircraft-copy-summary-hover = このカードをテキストとしてコピーします。判明している内容と各行の出典を含みます。位置と最後のメッセージは含みません。

aircraft-asking-station-dots = ステーションに問い合わせ中…

aircraft-not-changed = 変更されませんでした: { $why }。

aircraft-loading-photo = 写真を読み込み中…

aircraft-radio-is-on = 無線機は { $mode } です — 聴くには { $label } を押してください。これは { $label } が以前に受信した内容です。

aircraft-no-adsb = まだ ADS-B を受信していません。

aircraft-hfdl-off = HFDL デコーダーはオフです — HFDL を押してオンにしてください。

aircraft-vdl2-starting = VDL2 デコーダーを起動中…

aircraft-no-view = { $label } にはここで表示できるビューがありません。

textmodem-waiting-modem = モデム待機中…

textmodem-locked = ロック

textmodem-no-frame = フレームなし

textmodem-locked-hint = フレームを追跡中です。同期相関を12フレームにわたって積算した値がそれ自身のヌル値より明確に高く、かつフレーム周期の自己相関でフレームの存在が確認できた場合にのみ受理されます。

textmodem-sync-readout = 同期 { $sync }（ヌル { $null }）

textmodem-sync-hint = 全フレーム位相で積算した差動同期相関と、その位相間の中央値（超えるべきヌル値）です。実際の電波での測定値：4285では0.85、他モードの強いPSK搬送波では0.14でした。

textmodem-frame-readout = フレーム { $acf }

textmodem-frame-hint = 1フレーム後にフレームのどれだけが繰り返されるかを示します。4285は80個の同期シンボルが繰り返されるため0.3〜0.6、他モードの搬送波は約0.03です。

textmodem-refs-readout = 参照 { $a }/{ $b }/{ $c }

textmodem-refs-hint = デスクランブル後の3つの参照ブロックの集中度です。スクランブル前はすべてシンボル0なので、正しいフレームでは各ブロックが1点に集まり0.7以上になります。16シンボル分のノイズでは0.25です。

textmodem-rate-hint = コンスタレーションから判別できる内容です。符号化なしの各レートは符号化ありのレートとコンスタレーションを共有するため、区別するにはAnnex Eが必要で、ペイロードの読み取りにもAnnex EのFECが必要です。

textmodem-offset-frames = { $offset } Hz · { $n } フレーム

textmodem-offset-frames-hint = 参照ブロックが一致して示す搬送波オフセットと、このセッションで復調したフレーム数です。

textmodem-som-readout = SOM { $best }/32 · { $n } 通

textmodem-som-hint = Annex Eのメッセージ開始パターンと、デコーダ出力がそれにどこまで近づいたかを示します。ノイズでは32中16付近、符号化された送信ではフェージングで一致しなくても20台に達し、一致すると符号化レートが特定されメッセージが始まります。符号とインターリーバは生成した信号を正確にデコードできますが、符号化局はまだ実際の電波から受信できていないため、受信できたときはまずここで一致が見られるはずです。

textmodem-rx-only = 受信専用

textmodem-rx-only-hint = このモードのデコーダは移植済みで録音を読み取れますが、変調器はまだ書かれていないため、後から拒否するのではなく送信側をあらかじめ無効にしています。

textmodem-msg-hint = 自分用のメッセージボタンです。運用状況、天気、定型の返信などを登録できます。各ボタンは全文を一度に送信し、F1〜F9で最初の9個を押せます。局の設定と一緒に保存されるので、リモートクライアントでも使えます。

textmodem-weather-chip = {" "}天気 { $n }{" "}

textmodem-weather-hint =
    このバンドで受信したSYNOPおよびSHIP報を復号したものです。

    数値だけでは意味がなく、数時間前のある位置の天気こそ後で見返したい情報なので、デコード一覧ではなくここに保存します。

    RTTYテキストの受信中に検出するため、局のシフトとボーレートに合わせて同調する必要があります。陸上局は位置を送らないので、それらの行には局番号と数値のみで座標はありません。船舶報は自身の位置を含みます。

textmodem-olivia-reverse-hint = トーン順序を送受信の両方向で反転します。逆順で送信している局は、これをオンにするまでノイズとして受信されます。

textmodem-olivia-tones-raised = （Olivia の { $n } → 4）

textmodem-ifk-reverse-hint = トーンのステップを反転します。IFK+はデータを連続するトーン間のステップで運ぶため、反対側のサイドバンドで受信した局はすべてのステップが負になり、これをオンにするまでノイズとして受信されます。

textmodem-domino-hint = { $name } — { $baud } ボー、18トーン、前方誤り訂正なしのインクリメンタル周波数キーイング

textmodem-mt63-long-hint = MT63-{ $label }、帯域幅 { $bw } Hz — 長いインターリーブによりバーストが { $chars } 文字に分散されます

textmodem-mt63-short-hint = MT63-{ $label }、帯域幅 { $bw } Hz、短いインターリーブ — 短いインターリーブによりバーストが { $chars } 文字に分散されます

textmodem-mt63-not-offered = 1Kと500Lは未対応

textmodem-mt63-not-offered-hint = MT63-1Kと500 Hz長インターリーブは、バッチデコーダでは読み取れますが、ライブ受信機ではまだ対応していません。ロックはシンボル位相と極性を探索しますが帯域中心は探索せず、それを探索すると一方の変種と引き換えに別の変種が犠牲になることが分かりました。docs/internal/MODE-PROGRESS.md を参照してください。

textmodem-mfsk-caveat-mfsk8 = { " " }— コーパスの録音はテキストの一部が欠落します。デコーダは他の全変種と同じものです

textmodem-mfsk-caveat-mfsk11 = { " " }— コーパスの録音はリサンプラの起動のため先頭グループが欠落します

textmodem-mfsk-hint = { $label } — { $baud } ボー、{ $tones } トーン、帯域幅 { $bw } Hz{ $caveat }

textmodem-mfsk-reverse-hint = トーン順序を反転します。送信時とは反対のサイドバンドで受信した局は中心を軸に鏡像になって届き、これをオンにするまでノイズとして受信されます。

textmodem-psk-family = ファミリー

textmodem-psk-family-plain-hint = PSK31本来のファミリー：差動搬送波1本、前方誤り訂正なし

textmodem-psk-family-bpsk63f-hint = 62.5ボーのロバスト符号でインターリーバなし — fldigiがインターリーバなしで送信する唯一の符号化モードです

textmodem-psk-family-robust-hint = ロバスト符号と2x2xNインターリーバを使う単一搬送波

textmodem-psk-family-multi-hint = 複数搬送波、PSK31のバリコードで符号化は**なし** — 広帯域で高速ですが、きれいな信号が必要です

textmodem-psk-family-robust-multi-hint = 複数搬送波**と**ロバスト符号：PSKモードの中で最も広帯域で最もロバストです

textmodem-psk-baud-fast-hint = { $baud } ボー — 約1秒後から読み取れます。このレートでは受信機のロック時間がメッセージの3分の1に相当します

textmodem-psk-baud-hint = { $baud } ボー

textmodem-psk-modulation = 変調

textmodem-psk-bpsk-hint = 1シンボル1ビット、差動検波

textmodem-psk-qpsk-hint = 1シンボル2ビットでレート1/2符号を使うため、実質1シンボル1データビットです。同じ速度でBPSKと同じテキストレートになり、誤りは半分です

textmodem-psk-8psk-hint = 1シンボル3ビット、符号なし：同じ速度では3つの中で最速で、最もロバスト性が低くなります。受信専用です（8相の送信機はありません）。合成信号で検証済みで、テストコーパスの8PSK録音はきれいなコンスタレーションではありません

textmodem-psk-coded = 符号化あり

textmodem-psk-no-code = 符号化なし

textmodem-psk-coded-hint =
    { $baud } ボー、{ $n } { $n ->
            [one] 搬送波
           *[other] 搬送波
        }、{ $code }、帯域幅 { $bw } Hz — 受信専用：符号化PSKの送信機はまだ書かれていません

textmodem-psk-wide-bank-hint = { "*" }*この広帯域バンクはまだデコードできません。** 搬送波同士が近すぎて、入力レートの受信フィルタでは分離できません。より狭いバンクは自身の録音を読み取れます。ここで聞こえるものが失われるわけではなく、問題は信号ではなくフロントエンドにあります。

textmodem-hell-slant = スラント

textmodem-hell-slant-hint = 受信サンプルクロックの補正です。ドットが0.4 %長く届くキャプチャ（普通のサウンドカードや、SkySweepのオフエアHell録音での測定値）では約-4000が必要です。文字が直立して見えるまで調整してください。

settings-net-rbn-enabled-hint = 世界中のスキマーを読み取り、伝搬マップに反映します。これにより、このラジオが受信していないバンドもマップに表示されます。電波は出さず、アカウントも不要で、一般タブのコールサインを使用します。RBNのスポットはスポット一覧には表示されません。招待ではなく測定値だからです。ホストはreversebeacon.netに限定されず、同じtelnetフィードを提供するノードなら使えます。

settings-net-rbn-port-hint = reversebeacon.netでは、7000がCW/RTTYフィード、7001がFT8/FT4フィードです

settings-net-wavelog-intro = Wavelogは**あなた自身**が指定したアドレスで運用するログブックです。ここで記録した交信は選択した局ロケーションへアップロードされ、Wavelogが既に保持している交信を取り込むこともできるため、他のプログラム、他のリグ、またはWavelog自身のWebフォームで行ったQSOも「交信済み」判定に反映されます。

settings-net-wavelog-address-hint = Wavelogのトップページの場所：https://log.example.org またはLANアドレス。/index.php は付けても付けなくても構いません。

settings-net-wavelog-key-hint = WavelogのAPIページで発行したAPIキー。アップロードには読み書きキーが必要で、読み取り専用キーではダウンロードのみ可能です。

settings-net-pick-one = 選択

settings-net-test-to-list-locations = 接続テストでロケーション一覧を取得します

settings-net-trust = 信頼

settings-net-trust-hint = このサーバーに限り、**この**証明書のみを受け入れます。「何でも受け入れる」ではなくピン留めなので、証明書が変わった場合は接続が拒否されてその旨が表示され、そのアドレスで応答する相手に黙って接続されることはありません。

settings-net-test-wavelog = Wavelogをテスト

settings-net-test-wavelog-hint = このアドレスとキーが使えるかサーバーに問い合わせ、局ロケーションを一覧表示します。先に設定を適用し、ログは記録しません。

settings-net-wavelog-download = Wavelogから交信をダウンロード

settings-net-every-min = 間隔（分）

settings-net-locations = ロケーション

settings-net-download-now = 今すぐダウンロード

settings-net-download-now-hint = 前回のダウンロード以降にWavelogへ追加された分を取り込み、既に保持している交信をアップロード済みとしてマークします。ログブック内の既存データは変更されません。

settings-net-wavelog-upload-note = 自動でアップロードされるのは**ここで**記録した交信のみです。Wavelog側の既存分はログブックでQSOにチェックを入れて送信するため、あなたが選んでいないものがこのマシンから出ることはありません。

settings-net-broadcast-note = ウォーターフォールに表示される長波・短波の放送局です。以下の各ソースは現在の放送シーズン分がダウンロードされ、シーズンが替わると再ダウンロードされます。すべて**1つのストア**に入るため、SCHEDULEウィンドウのソースフィルタで1つだけでも全部でも表示できます。

settings-net-downloaded = ダウンロード済み

settings-net-schedules = スケジュール

settings-net-schedule-yours = 自分用

settings-net-download = ダウンロード

settings-net-hfcc-hint = HFCCは局名を別ファイルで公開しているため、両方が必要です。それが揃うまでは、コールサインだけのスケジュールを読み込まず、その旨を表示します。

settings-net-refetch-hint = 今シーズンのコピーを再取得し、キャッシュ済みのものを置き換えます

settings-net-from-file = ファイルから…

settings-net-aoki-file-hint = Aokiのスケジュールを、自分で保存したファイルから読み込みます。Aokiが公開している.zip、またはその中の.txtです。サイトはシーズンごとに移動するため、この項目があります。

settings-net-eibi-file-hint = EiBiのスケジュールを、このマシンに保存したsked .csvから読み込みます

settings-net-filter-aoki = Aokiスケジュール

settings-net-filter-eibi = EiBiスケジュール

settings-net-download-all = 今すぐすべてダウンロード

settings-net-download-all-hint = 取得可能なすべてのソースを取得します。既に正常なものは保持されます

settings-net-own-list-note = 自分用リストは**スケジュール**、つまり聞こえると予想するものであり、実際に聞こえたものを記録する受信ログではありません。下でインポートするか、ファイルを直接編集してください。

settings-net-import-list = 自分の局リストをインポート…

settings-net-import-list-hint = CSVまたは単純な局リストを読み込み、上のファイルにマージします。周波数はkHzでもMHzでも構いません。単位は数値から判別します。

settings-net-filter-station-lists = 局リスト

settings-net-dx-status = DX — { $body }

settings-net-pin-own = 独自 — { $pin }…

settings-net-schedule-rows = { $name } { $n } 送信

settings-net-schedule-not-downloaded = { $name }     未ダウンロード

settings-net-schedule-imported = { $source } { $season } — { $n } 送信、ストアに保存済み

settings-net-stations-imported =
    { $n ->
        [one] リスト内の { $n } 局をストアに保存しました
       *[other] リスト内の { $n } 局をストアに保存しました
    }

txaudio-natural = ナチュラル

txaudio-ragchews-and-local-contacts-the = ラグチューや近距離の交信向け：ノイズ除去、ポーズ中の穏やかなエクスパンダー、軽いコンプレッション。部屋の音を除いた、あなたの声に近い音になります。

txaudio-ssb-everyday = SSB標準

txaudio-everyday-ssb-100-hz-2-9-khz-a = 標準的なSSB：100 Hz〜2.9 kHz、2 kHz付近に少しプレゼンス、中程度のコンプレッション。

txaudio-ssb-punch-dx = SSBパンチ / DX

txaudio-weak-signals-and-pileups-a-narrow = 弱い信号やパイルアップ向け：300 Hz〜2.7 kHzの狭い音声帯域に強いプレゼンス強調をかけ、位相ローテーターと高速コンプレッサーで強く駆動し、送信機が出せる最大のトークパワーを得ます。ハイファイではありません。

txaudio-ssb-punch-for-hours-in-a-noisy = ノイズの多い局で長時間使うSSBパンチ：ノイズ除去と強いゲートでファンの音や隣のオペレーターの声をポーズ中に入れず、コンプレッサーはさらに強く働きます。

txaudio-essb-hi-fi = ESSB / ハイファイ

txaudio-wide-ssb-or-am-50-hz-4-khz-with = ワイドSSBまたはAM：低域を残した50 Hz〜4 kHz、穏やかなコンプレッションとディエッサー。それだけ広い送信フィルターと静かなバンドが必要です。

txaudio-fm-repeater = FM / レピータ

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM：急峻な300 Hzハイパスで音声をCTCSSトーンから遠ざけ、上限はチャンネルに合わせて3 kHzで止め、リミッターが偏移を抑えます。

txaudio-mobile-noisy-shack = モービル / 騒がしいシャック

txaudio-fans-engine-traffic-the-denoiser-a = ファン、エンジン、交通音向け：ノイズ除去、150 Hzの急峻なハイパス、強いゲートに加え、残った音の上に出るだけのコンプレッションをかけます。

txaudio-digital-voice-2 = デジタルボイス

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE：ノイズ除去と穏やかなレベリングのみでEQなし。ボコーダは自然な音声を好みます。デジタルボイスをオンにします。

txaudio-imported-2 = インポート済み

frame-no-contrast-fit = コントラストなし — クリックでフィット

frame-levels-hidden-hint = 表示の下限/上限が画面上のすべての信号を外れているため、ウォーターフォールが単色になっています。クリックするとラジオが受信している内容に合わせます。

frame-bw-waterfall = BW ウォーターフォール · { $span }

frame-bw-waterfall-unknown = BW ウォーターフォール · —

frame-every-module-own-window = すべてのモジュールが個別のウィンドウにあります

frame-window-chip-hint = ウィンドウ — このウィンドウを独立したOSウィンドウにして別のモニターに置きます

frame-dock-panel-pane-hint = ドック — このパネルをウォーターフォール下のペインに戻します

frame-dock-panel-hint = ドック — このパネルをウォーターフォールの下に戻します

frame-dock-spectrum-hint = ドック — このスペクトラムをメインウィンドウに戻します。閉じるボタンを探さずに閉じるのと同じです。

frame-dock-band-menu-hint = ドック — バンド/モード選択をメインウィンドウに戻します。閉じるボタンを探さずに閉じるのと同じです。

frame-dock-rotator-hint = ドック — ローテーターをメインウィンドウに戻します。閉じるボタンを探さずに閉じるのと同じです。

frame-module-panadapter = パンアダプター

frame-module-panel = パネル

frame-module-controls = コントロール

frame-module-aux-panadapter = 補助パンアダプター

frame-module-band-mode = バンドとモード

frame-module-rotator = アンテナローテーター

frame-detached-title = { $flavor } — { $name }

frame-workspace-missing = ワークスペース { $n } は保存されていません。設定 › UI › ワークスペースで保存すると、このキーで呼び出せます。

frame-no-operating-panel = { $mode } には操作パネルがありません。ドック解除されており、このモードにはパネルがありません。

frame-rx-only-nudge = このラジオは受信専用で送信機がありません。送信コントロールを非表示にしてリスニング画面を使いますか？

frame-listening-controls = リスニング用コントロール

frame-listening-controls-hint = 送信コントロールを非表示にして、このラジオをリスニング画面に切り替えます（設定 → ラジオ）。再同調はされず、デコーダは動作し続けます。

frame-update-available = SDR Oxide Brown { $tag } が利用可能です。現在のバージョンは { $version } です。

frame-download = ダウンロード

frame-download-hint = ブラウザでリリースページを開きます

frame-tuning-catching-up = 同調中 — ラジオが追従しています

frame-roll-up-hint = 巻き上げ — タイトルバーだけにして、ウィンドウ領域をウォーターフォールに返します

frame-unroll-hint = 巻き戻し — パネルを再表示します

frame-window-panel-hint = ウィンドウ — このパネルを独立したウィンドウにして別のモニターに置きます

frame-panel-window-title = { $mode } — パネル

frame-auto-stopped-stop-control = 自動停止：停止コントロールが使われました

frame-dont-show-again = 今後表示しない

frame-dont-show-again-hint = この画面でこの確認を記憶します。エンジンが --oob-tx で動作している間はロックアウトは解除されたままで、これは警告を再表示しないだけです。

frame-cb-window-title = ⚠  11 m（CB）— アマチュアバンドではありません

frame-cb-about-to-allow = 11 mバンドでの送信を許可しようとしています。

frame-cb-not-amateur = 11 mはアマチュアバンドではありません。独自のチャンネル、モード、電力制限、技術基準適合機器を持つ別の無線業務であり、それらは国ごとに異なります。そこでの送信は、運用している国の規則に従います。

frame-cb-only-continue = 自国で27 MHzに何が許されているかを理解し、その範囲内で運用する場合にのみ続行してください。送信の責任はあなたにあります。

frame-cancel = キャンセル

frame-saved-to-profile = プロファイル { $name } に保存しました

frame-saved-as-default = この局の既定として保存しました（サーバーにパスワードがないため、すべてのクライアントがこれを受け取ります）

frame-bindings-offer-title = このプロファイルにはキーボード割り当てがあります

frame-bindings-offer-profile = サインインしたプロファイル（{ $name }）はキーボードとマウスの割り当てをサーバーに保存していますが、このブラウザでは使用していません。これが安全な既定です。他の人と共用する局ではキーボードも共用になり、別のログインのPTTやスペースキーが引き継がれるのは実際に危険です。

frame-bindings-offer-default = この局の既定プロファイルはキーボードとマウスの割り当てをサーバーに保存していますが、このブラウザでは使用していません。これが安全な既定です。他の人と共用する局ではキーボードも共用になり、別のログインのPTTやスペースキーが引き継がれるのは実際に危険です。

frame-bindings-offer-choice = ここで使うか、使わないかを選んでください。どちらの場合も結果が通知され、あなたが選ぶまで何も変更されません。

frame-bindings-use = このプロファイルの割り当てを使う

frame-bindings-keep-mine = 自分のものを維持（今後は確認しない）

frame-bindings-using-profile = このプロファイルに保存されたキーボード割り当てを使用中

frame-bindings-keeping-own = このデバイス自身のキーボード割り当てを維持します。今後は確認しません

frame-look-back-to-profile = このプロファイルに保存された外観に戻しました

frame-look-none-stored = このプロファイルには保存された外観がまだありません。先に保存してください

frame-antenna-rotator-caption = アンテナローテーター

bands-condx-global-forecast-rest-what = CONDXは全世界の予報です。WSPRは世界のWSPRネットワークが受信した内容（wspr.live）、PATHS、REACH、BESTはこの局とRBNが受信した内容です。空の行は何もデコードされなかったことを示しますが、バンドが閉じていたのか、単に誰も運用していなかったのかは分かりません。

bands-km-s-parent-peak-rate =
    { $name }（{ $code }）— { $velocity_kms } km/s、母天体 { $parent }。極大時のZHR { $zhr }、極大は { $peak } 頃。

    輻射点が地平線の上にあれば流星の軌跡があなたのところまで届く可能性があります。速い流星群は電離した軌跡が長く残るため、6 mと2 mのミーティアスキャッターや、10 m/11 mの短時間のオープンに有利です。

bands-hearing-means-path-open-each = { $location } の { $callsign } — 受信できれば { $band } のパスが開いています。10m（28.200 MHz）は11 mの状況に最も近いアマチュアバンドの代用です。各ビーコンは10秒ごとに1バンド上がるため、この行はスロットごとに変わります。

bands-wspr-activity-old = · WSPR活動 { $age } 前（wspr.live）

bands-psk-activity-old = · PSK活動 { $age } 前

bands-activity-hover =
    過去15分間の { $band } での全世界の活動（出典：{ $source }）：
    受信レポート { $paths } 件 · 送信局 { $tx } · 受信局 { $rx }

    { $note }

    バンドが混んでいるほど明るく表示されます。これはバンドがどこかで受信されていることを示すもので、あなたに対して開いているとは限りません。

bands-derived-hover =
    { $band } はHAMQSL.comが評価するバンドではありません。これは公表されている { $group } グループの判定を、最も近い代用として読み取ったものです。

    予報であり、あなた自身のパスの測定ではありません。

bands-wspr-note = 世界のWSPRネットワーク（微弱信号ビーコン）。

bands-psk-note = アクティビティモードからの受信レポート：FT8、FT4、およびWSPRのビーコンがカバーしないCW/RTTYのレポート。

aircraft-history-reason-disabled = 局が履歴をオンにしていません

aircraft-history-reason-unauthorized = 局がサインインを求めています

aircraft-history-reason-no-station = 履歴は接続先の局が保持します

aircraft-history-reason-unknown = 局の応答を理解できませんでした

aircraft-history-show = 履歴を表示

aircraft-history-hide = 履歴を隠す

aircraft-label-history = 履歴

aircraft-label-first-heard = 初受信

aircraft-label-last-heard = 最終受信

aircraft-label-track-points = 航跡点

aircraft-label-highest = 最高高度

aircraft-label-callsigns = コールサイン

aircraft-label-registrations = 登録記号

aircraft-history-nothing-kept = この機体の保存データはまだありません

aircraft-history-by-station = この局による

aircraft-history-kept-24h = 過去24時間分を保存

aircraft-history-of-kept = 保存された点のうち

aircraft-history-used-24h = 過去24時間で使用

aircraft-history-seen-24h = 過去24時間で確認

aircraft-history-looked-up = { $age } 前に照会

aircraft-alert-on-air-as = { $entry } が { $callsign } として運用中です

aircraft-alert-on-air = { $entry } が運用中です

aircraft-watch-already = 既にリストにあります。

aircraft-watch-absent = リストにありません。

aircraft-watch-invalid = コールサインでも登録記号でもありません：英数字3〜10文字。

aircraft-watch-full = リストがいっぱいです：100機まで。

multi-module-windows = モジュールウィンドウ

multi-in-its-own-window = 独立ウィンドウ

multi-docked = ドック済み

multi-dock-all-windows = すべてドック

multi-bring-every-module-window-back = すべてのモジュールウィンドウをメインウィンドウに戻します

multi-undock-all-windows = すべてドック解除

multi-put-every-module-in-a-window-of = すべてのモジュールを個別のウィンドウにします。メインウィンドウにはスペクトラムとデコーダが残ります

multi-two-radios-side-by-side-need-a = 2台のラジオを並べるにはより広いウィンドウが必要です。この画面では1台ずつ表示します

multi-the-station-s-windows-some-modules = 局のウィンドウ — 一部のモジュールは独立したOSウィンドウにあります。変更するにはここを開いてください。

multi-put-a-module-in-its-own-window-sp1 = モジュールを独立ウィンドウにします。SP1やAUX SPを別のモニターに、RXコントロールをその横に置けます。同じ項目は設定 → UI → ドック解除にもあります。

multi-module-sp1 = SP1 — スペクトラム

multi-module-sp2 = SP2（AUX）— 2つ目のスペクトラム

multi-module-panel = 操作パネル

multi-module-controls = RXコントロール

multi-module-band-mode = バンドとモード

multi-module-rotor = アンテナローテーター

multi-bring-back = { $name } をメインウィンドウに戻します

multi-put-own-window = { $name } を独立したOSウィンドウにします。独自のapp-idを持つため、ウィンドウルールでモニターに固定できます

multi-windows-chip = ウィンドウ

aircraft-stats-category = カテゴリ

aircraft-stats-airline = 航空会社

aircraft-stats-model = 機種

aircraft-stats-operator = 運航者

aircraft-stats-route = 経路

aircraft-stats-src-no-station = 現在受信中の機体を集計しています。接続先の局は直近1日分を保持します。

aircraft-stats-src-asking = 局に直近1日分を問い合わせ中…

aircraft-stats-src-ready = 局が保持する過去24時間分と、現在受信中の機体です。

aircraft-stats-src-disabled = 局が履歴をオンにしていません。現在受信中の機体を集計しています。

aircraft-stats-src-reason = { $reason } — 現在受信中の機体を集計しています。

aircraft-export-saved-station = { $name } を保存しました：局の過去24時間分。

aircraft-export-saved-local = { $reason }：代わりに過去10分間に受信した { $n } 機を保存しました。

login-remember-this-station = この局を記憶する

login-remember-me = ログイン情報を記憶

login-this-station-keeps-your-sign-in = サインイン情報はこのページではなく局が保持します。他のラジオで入力する必要はなく、離れるときに消去するものもありません。

login-not-this-time-this-browser-forgets = 今回は記憶しません：このブラウザは閉じると忘れるため、次回のセッションで再度確認され、このサインインに関する情報はデバイスに残りません。

aircraft-label-range = 距離

aircraft-label-vertical = 垂直速度

aircraft-label-emergency = 緊急

aircraft-from-your-station = 自局から

aircraft-footer-disabled = オンライン照会不可 — 局側でリレーがオフ

aircraft-footer-no-station = オンライン照会不可 — 問い合わせ先の局がありません

aircraft-footer-unauthorized = オンライン照会不可 — 未サインイン

aircraft-hover-close = 閉じる（Esc）

aircraft-hover-lookup = オンラインで照会

aircraft-hover-lookup-hint = この局のリレーに、この機体の機種、運航者、経路、写真を問い合わせます。ボタンを押すまで何も問い合わせません。

aircraft-hover-lookup-waiting = 別の照会を実行中です。一度に1件のみ

aircraft-hover-history-hint = この機体が過去1日にどこにいたかを、自局の記録から表示します。問い合わせ先は自局のみで、インターネットには接続しません。

auto-stopped-reason = 自動停止：{ $reason }

auto-stopped-idle = 自動停止：{ $mins } 分間操作がありませんでした

auto-answering = { $role }：{ $call } に応答中

auto-caller-went-away = 自動：呼び出し局がいなくなりました

auto-nobody-left = 自動：応答する相手がいなくなりました

auto-calling-cq = { $role }：CQ送信中（#{ $n }）

auto-cq-no-answer = 自動CQ：{ $limit } 回呼び出しても応答なし — 受信待機（自動RX）

auto-listening = { $role }：受信待機中

auto-role-stopped = { $role } 停止：{ $sentence }

auto-rx-listening = 自動RX：受信待機中

auto-cq-calling = 自動CQ：CQ送信中

auto-cont-running = 自動継続：実行中

aircraft-search-missing-online = { $total } 機中 { $missing } 機にオンラインデータがありません — カードを開いてください

aircraft-search-shown-of = { $shown } / { $total }

aircraft-search-help =
    受信済みの機体を検索します：アドレス（ICAO）、コールサイン、登録記号、機種、運航者、カテゴリ。複数の語はすべて一致する必要があります。語を特定の項目に限定するには icao: call: reg: type: op: を使います。例：  reg:PH op:KLM
    機種と運航者は局のオンライン照会から得られるため、カードを照会済みの機体でのみ見つかります。検索自体は何も問い合わせません。

aircraft-search-hint = 検索

aircraft-search-clear = 検索をクリア

gridtracker-shade-squares-heard-live-decode = ログにあるものだけでなく、ライブのデコード一覧で受信したものも塗り分けます。リスナー向けのマップで、いま電波に出ているものを交信済みマークの下にシアンで表示します。

gridtracker-grid-ham = GRID/HAM

gridtracker-maidenhead-squares-what-a-ham-log = メイデンヘッドのグリッド — アマチュア無線ログが持つ情報です。CBのコールはここには表示されません。11 mの交信にはロケーターではなく国が含まれるためです。

gridtracker-country-cb = COUNTRY/CB

gridtracker-dxcc-countries-at-their-nominal = DXCCエンティティを名目上の中心位置に表示します — 11 mの交信にはロケーターがないため、CBログが持つ情報です。アマチュアのコールもここには表示されません。ここはグリッドではなく国の表示です。

colouredit-paste-a-colour-code-rrggbb-or-r-g = カラーコード（#RRGGBB または r, g, b）を貼り付けるか、スウォッチ上で Cmd/Ctrl+V

cwkey-thread-failed = CWキースレッドを開始できませんでした：{ $error }

cwkey-grab-failed = パドルを排他的に取得できませんでした。接点が他のウィンドウでもクリックとして反応する可能性があります（別のプログラムが使用中ではありませんか？）

ignore-muted-hover =
    このセッション中ミュート：{ $names }

    ミュートした局はデコード一覧に表示されず、アラートも鳴らさず、読み上げられず、自動モードの応答候補にもなりません。その局のために送信やアップロードが行われることはなく、リストはsdroxideを閉じると破棄されます。保存されたコピーはないため、一晩聴いた後に何かが隠れたまま残ることはありません。

    クリックするとすべてのミュートを解除します。または、該当する行の−ボタンを押してください。

js8-setup-hint = セットアップ — コールサイン、グループ、自動返信、ビーコン間隔、ステータスメッセージ

logbook-contest-logger-a-session-dupes-a = コンテストロガー — セッション、重複チェック、ライブスコア、Cabrillo

logbook-up-chip = UP

net-wavelog-stopped = 停止：{ $error }

net-wavelog-download-stopped = Wavelogのダウンロードが停止しました：{ $error }

net-wavelog-new-already-here = 新規 { $added } 件、既存 { $matched } 件

net-wavelog-fill-in-first = 先にアドレスとAPIキーを入力してください

net-wavelog-no-station-location = ダウンロード元の局ロケーションがありません。先に接続テストをしてください

publicsdr-could-not-connect = このセッションでは接続できませんでした

publicsdr-use-chip = 使用

weather-title = 気象通報

weather-intro = 船舶と陸上局からの符号化された気象通報で、RTTYの4583、7646、10100.8 kHzで送信されます。SHIPは02:00と07:35 UTC頃、SYNOPは00:35と12:10頃です。数値だけでは意味がないため、通報はデコード一覧ではなくここに保存されます。

weather-nothing-decoded = まだデコードされたものはありません。上記の時刻のいずれかにRTTYを動作させたままにしておくだけです。バンドが最も静かな時間帯です。

weather-head-call-day-hour = { $call } — { $day }日 { $hour }:00 UTC

weather-head-day = { $day }:00 UTC

weather-head-report = 通報

weather-not-reported = { $label }：報告なし

weather-in-their-words = 本文：{ $remark }

weather-ago-now = たった今

weather-ago-minute = 1分前

weather-ago-minutes = { $m } 分前

weather-ago-hours =
    { $h } { $h ->
            [one] 時間
           *[other] 時間
        }前

weather-ago-days = { $d } 日前

sat-off-only-satellites-with = オフ：周波数が登録されている衛星（交信可能なもの）と、自分で貼り付けた衛星のみ。オン：局が追跡するすべての衛星で、その大半は誰にも聞こえないものです。

adsbmap-tune-hint = ダブルクリックで同調 · Shiftでもう一方のダイヤル

adsbmap-airport-large = 大型空港 · 標高 { $elev }

adsbmap-airport-medium = 中型空港 · 標高 { $elev }

adsbmap-from-station = 自局から { $dist }  { $bear }°

adsbmap-aircraft-within =
    { $dist } 以内に { $n } { $n ->
        [one] 機
       *[other] 機
    }

adsbmap-navaid-mw = { $khz } kHz（中波）

adsbmap-click-to-close = マップをクリックして閉じる

settings-ui-broadcast-stations-hint =
    LISTENウィンドウとウォーターフォールのラベルに表示してよい放送局を選びます。

    { "**" }このスパン内**は、この選択肢ができる前の全画面の動作で、ダイヤルの両側にスパンの半分ずつです。そのウォーターフォールのラベルが対象とすべき範囲です。**バンド全体**は、スケジュールにある位置付きのものをすべて表示します。バンドを見て回るリスナーが探しているのはこれですが、マップは混み合います。

settings-ui-waterfall-labels = ウォーターフォールのラベル

settings-ui-tuned-only = 同調中のみ

settings-ui-tuned-only-hint = 同調している局の名前だけを表示します。パスバンド上に名前が並ぶのではなく、聴いているものの名前1つだけです。

settings-ui-whole-band = バンド全体

settings-ui-whole-band-hint = スケジュール上で現在運用中のすべての放送局を、位置に関係なく表示します。1局を聴くのではなくバンドを見て回るときに向いていますが、その分混み合います。SPOTSウィンドウにもBC ALLとしてあります。

settings-ui-classic-ui = クラシックUI

settings-ui-classic-ui-hint =
    すべてのコントロールを左側の1列に並べ、グループを折りたためるようにします。SDR++やSDR++Brownと同じ配置で、残りのスペースはすべて画面表示に使います。コントロールも設定も同じで、多くの短波リスナーが慣れている配置にしただけです。

    ウォーターフォールの取り分を保つため十分な幅のウィンドウが必要です。それより狭い場合は列は省かれ、ストリップが通常どおり描画されます。パネルは右端をドラッグして幅を変えられます。オフにすると、このプログラム独自のストリップがこれまでどおり使われます。

settings-ui-classic-ui-check = コントロールを左の列に配置

settings-ui-updates = アップデート

settings-ui-updates-check = 起動時に新しいリリースを確認

settings-ui-updates-hint = 起動時に1回、フォークのGitHub Releasesに最新リリースを問い合わせ、このビルドより新しければバナーを表示します。正式リリースのみが対象で（ナイトリーはプレリリース）、オフにすると外部への通信は一切行いません。

settings-ui-undocked = ドック解除 — 独立ウィンドウ

settings-ui-docked = このウィンドウにドック

settings-ui-dock-all-hint = ドック解除したすべてのウィンドウをこのウィンドウに戻します。

settings-ui-undock-all-hint = すべてのモジュールを個別のウィンドウにします。メインウィンドウにはスペクトラムとデコーダが残ります。音声モードの操作パネルのように表示するものがないモジュールは、ウィンドウを開きません。

settings-ui-spectrum-line = スペクトラム線

settings-ui-spectrum-line-hint = スペクトラム線の下の色とその陰影です。固定は全体が単色、グラデーションは下が最も濃く上に向かって薄くなり、混合は下半分が濃く上半分で薄くなります。線自体はシアンのままです。SPECポップアップのFILLチップで塗りを切り替え、その線の行で陰影を選びます。

settings-ui-fill = 塗り

settings-ui-fill-colour = 色

settings-ui-simple-ui = シンプルUI

settings-ui-simple-ui-hint =
    トップストリップから高度な追加機能を隠します：3Dビュー、スキマー、スペクトラム/ウォーターフォールのレイヤー切り替え、アワード管理、衛星、ISMデコード、無線メール。CBオペレーターや短波リスナーが使うコントロール（同調、モード、音量、スケルチ、帯域幅、ウォーターフォール、メモリー、スキャン）はすべて残ります。

    何かが無効になるわけではなく、隠れるだけです。オフに戻すとチップが再び表示されます。

settings-ui-simple-ui-check = 高度なチップを隠す

settings-ui-wake-lock = ウェイクロック

settings-ui-wake-lock-hint =
    このページが前面にある間は画面をオンにしておき、ウォーターフォールを表示したままのスマートフォンやタブレットが、1分間触れないだけで暗くなってロックされるのを防ぎます。受信、音声、デコードには不要でどちらでも動作し続けます。画面を読める状態に保つためのものです。

    スマートフォンとタブレットのストリップにあるパドロックと組み合わせます。指が触れても誤動作しないよう画面をロックしつつ、点灯したままにできます。

    ページが非表示になるたびに（別のタブや端末の電源ボタンなど）ブラウザはロックを解除し、ページが戻るとすぐに再度要求されます。これに対応したブラウザと安全なページ（httpsまたはlocalhost）が必要で、利用できない場合や拒否された場合はチェックボックスの横の行に表示されます。バッテリーを消費するため、頼まない限りオフです。コンピュータにインストールしたプログラムは、代わりにシステム自身の電源設定に従います。

settings-ui-wake-lock-check = 画面をオンにしておく

settings-ui-sound-welcome = サウンドの案内

settings-ui-sound-welcome-hint =
    Webページを開いたとき（新しいセッションやF5での再読み込み）、ページに触れるまでブラウザは音を出しません。これはブラウザの規則であり、画面に何も表示がないと、新しいセッションは動作していないように見えます。

    オンにすると、画面中央に🔇 画面のどこかをタップ、クリック、またはキーを押すと音が始まります、というメッセージが表示されます。マウス、タッチパッド、指、キーボードのいずれでも、最初のクリック、タップ、キーでメッセージが閉じて音が始まり、それ以外は何もしません。同調も、ボタン操作も、ショートカットの実行もしません。表示されるのはページを開いたときだけで、その後のセッション中は音に何が起きても二度と表示されません。

    やり方が分かったらオフにしてください。それでも音は最初の操作を待ち、その操作は通常どおり動作します。サーバー上のプロファイルに保存されます。既定ではオンです。

settings-ui-sound-welcome-check = サウンドの案内メッセージ

settings-ui-sound-welcome-browser-only = ブラウザのみ — インストール版プログラムは操作なしで音が出ます

settings-ui-profile-screen = 画面をプロファイルに追従

settings-ui-profile-screen-hint =
    `sdroxide --server` のリモートクライアントとして接続している場合、この画面の*外観*（テーマ、レイアウト、ウォーターフォールとスペクトラム、フォント、シンプルUI、マップレイヤー、パンアダプター自身の表示）は、サインインしたプロファイルに保存されるため、どのマシンでも既定値から始まらずに元の状態が復元されます。

    { "**" }サーバーに保存され、サインイン中は操作のたびに保存されます**ので、押すボタンはありません。これは意図的なものです。ブラウザのストレージは当てにできず（`persist()` の動作がブラウザごとに違う）、そこに保存されたセッションはそのデバイスの前にいない人にはリセットできません。サーバー上のプロファイルは、局のオペレーターが元に戻せるファイルです。

    { "**" }パスワードのない**サーバーではすべてのクライアントが同じプロファイルになるため、下のボタンを押さない限り何も書き込まれません。あるオペレーターのテーマが次のオペレーターのものになってはいけないからです。マシン固有のものは一切保存されず、ローカルのラジオには影響しません。

settings-ui-save-to-profile = プロファイルに保存

settings-ui-back-to-profile = プロファイルに戻す

settings-ui-carry-bindings = 操作割り当てを持ち運ぶ

settings-ui-carry-bindings-hint =
    キーボードとマウスの割り当ても、サーバー上のプロファイルと一緒に保存し、ブラウザやデバイスが変わってもログインに付いてくるようにします。上の画面設定と同じ考え方を、キーに適用したものです。

    { "**" }非推奨で、既定ではオフです。**複数人で共用する局ではキーボードも共用されます。割り当てを持つプロファイルは、サインインした瞬間に別のオペレーターのPTT、スペースキー、同調キーを再割り当てしてしまう可能性があり、オンにすると今は分からない別の影響が出ることもあります。サーバーがあなた専用で、自分の複数のデバイスから接続する場合にのみオンにしてください。

    フォーク独自：上流の機能ではありません。上の画面設定は常に引き継がれ、ここがこの項目で唯一のオプトインです。

settings-ui-carry-bindings-on = オン — 非推奨

settings-ui-carry-bindings-off = 持ち運ぶ（非推奨）

settings-ui-carry-bindings-title = 操作割り当てをサーバーに持ち運びますか？

settings-ui-carry-bindings-body =
    キーボードとマウスの割り当てをサーバー上のプロファイルに保存し、ブラウザやデバイスが変わってもログインに付いてくるようにします。

    他の人も使う局ではキーボードも共用されます。割り当てを持つプロファイルは、サインインするだけで別のオペレーターのPTT、スペースキー、同調キーを再割り当てできてしまい、これを有効にすると今は分からない別の影響が出ることもあります。

    そのため非推奨です。サーバーがあなた専用の場合にのみオンにしてください。

settings-ui-carry-bindings-confirm = 理解しました — それでも持ち運ぶ

settings-ui-start-swl = SWLモードで開始

settings-ui-start-swl-hint =
    毎回のセッションをSWLモードがオンの状態で開き、プログラムがリスナー向け画面で起動するようにします。すべてのラジオが対象で、ラジオごとではなく画面の設定です。

    ラジオごとの切り替えは設定 → ラジオ → 送信コントロールにあります。受信専用ラジオ1台と送受信機1台がある局では、受信専用のほうをそこで設定し、これはオフのままにします。そのスイッチを使うと、このセッションではこの初期設定はオフになります。`--swl` は常に優先されます。

settings-ui-start-swl-check = SWLモードをオンで開始

settings-ui-listen-hides-tx = LISTENタブで送信を隠す

settings-ui-listen-hides-tx-hint =
    バンド/モードメニューで**LISTEN**を選ぶとそのタブのすべての送信コントロールが隠れ、**OPERATE**を選ぶとこのラジオ自身の設定どおりに戻ります。

    送信機をオンにすることはできません。受信専用に設定されたラジオは受信専用のままで、バンドロックアウトと帯域外警告には影響しません。

    バンドメニューは最後に開いていたタブを記憶するため、受信状態で放置した局は受信状態で戻ってきます。次回の起動でも送信コントロールは隠れたままで、OPERATEを選ぶと戻ります。タブの意味が変わるため既定ではオフで、最初に一度確認します。

settings-ui-listen-hides-tx-check = LISTENタブでTXを隠す

settings-ui-listen-hides-tx-title = LISTENタブは送信コントロールを隠します

settings-ui-listen-hides-tx-body =
    LISTENタブでは送信コントロールが消え、OPERATEではこのラジオ自身の設定どおりに戻ります。

    何かを有効にするものではありません。同じ局の別の見え方なので、バンドロックアウト、帯域外警告、送信のガードには影響しません。

    タブは記憶されます。LISTENで放置した局はLISTENで始まり、OPERATEを選ぶまで送信コントロールは隠れたままです。

    知っておくべきこと：LISTENのまま放置した局は、タブが同調した周波数で受信し続けます。また、隠れているコントロールはオフのコントロールではありません。送信がオフなのは設定がそうなっているからで、ボタンが見えないからではありません。

settings-ui-listen-hides-tx-confirm = 理解しました — LISTENで隠す

settings-ui-swl-mode = SWLモード

settings-ui-swl-mode-hint =
    このラジオのすべての送信コントロール（PTT、CALL CQ、TXレベル、SEND、BEACONなどすべて）を隠し、ストリップのアマチュア向け追加機能（スポット、アワード）をリスナー向けのもの（SCHEDULEとSWL LOG、受信ログ）に置き換えます。設定 → ラジオ → 送信コントロールと同じスイッチで、ラジオごとに設定できます。受信専用のドングルがこのモードのまま、隣の送受信機は送信機を維持することもできます。

    ハードウェアは送信できる状態のままで、隠れるのはコントロールだけです。`--swl` は、その実行中すべてのラジオで強制的にオンにします。

    適用で有効になります。

settings-ui-swl-mode-check = すべての送信コントロールを隠す

settings-ui-report-picture = レポート用画像

settings-ui-report-picture-hint =
    受信レポートを放送局にメールで送るときに添付する画像です。自分のカード、写真など、局に見てほしいものを指定します。局向けの画像は1枚で、すべてのレポートに使われます。

    未設定のままならレポートはプレーンテキストで送られ、それでも十分なレポートです。

settings-ui-report-picture-none = なし — レポートはテキストで送信

settings-ui-choose = 選択…

settings-ui-report-picture-pick = このコンピュータから画像を選択

settings-ui-report-note = レポートのメモ

settings-ui-report-note-hint = すべての受信レポートに自動入力される定型メモです。自分が誰か、何で聴いているか、手紙ごとではなく一度だけ局に伝えたいことなどを書きます。プレーンファイルのプレーンテキストで、秘密ではありません。

settings-ui-report-note-hint-text = 任意 — 例：「詳細はご請求ください」

sstv-keep-dial = ダイヤル維持

sstv-keep-dial-on-hint = オン：SSTVを選んでもダイヤルは同調した位置のままになり、どの周波数でもデコーダが動作します。公表されているSSTV周波数は⇵ピッカーに残ります。クリックすると、バンドのSSTV周波数に移動する動作に戻ります。

sstv-keep-dial-off-hint = オフ：SSTVを選ぶと、SSTV周波数から3 kHzを超えて離れている場合にダイヤルがそのバンドのSSTV周波数に移動します。クリックすると、同調した位置のままにしてどの周波数でもSSTVをデコードします。SSTVを選んだ後の同調はどちらの場合も元に戻されません。

sstv-clear-rx = 受信をクリア

sstv-clear-rx-hint = LIVEビューに現在表示されている画像をクリアします。デコーダは動作し続けます。受信途中の画像を破棄して次のヘッダーを待つには、受信を再開を使ってください。

sstv-audio-present-no-header = 音声あり · SSTVヘッダーはまだなし

sstv-editor = エディタ…

sstv-editor-hint = 画像そのものを作成します：移動可能なテキスト、任意の書体、フレームの横または縦に渡すストリップ

sstv-save-image-as = 名前を付けて画像を保存…

sstv-save-copy-asks-file = このコンピュータにコピーを保存します。保存先のファイルを尋ねます

sstv-picture-still-loading = 画像を読み込み中です

sstv-share-hint-direct = この画像を誰かに送ります — WhatsApp、Telegram、このデバイス上の他のアプリなど

sstv-share-hint-save = この画像を保存し、送信用のメッセージを開きます

sstv-share-picture = 画像を共有…

sstv-reupload-hint = この画像を再送信します — 送信スロット { $slot } に読み込みます

sstv-radio-cannot-transmit = このラジオは送信できません

sstv-reupload = 再アップロード

sstv-unidentified = 識別不能

sstv-send-picture = この画像を送信

sstv-qsl-to = 宛先

sstv-the-card = カード

sstv-the-card-hint = これらは画像の横に印刷されるカードに載ります。空欄にした項目は描画されません。誰も出していないレポートがカードに載ることはありません。

sstv-date-utc = 日付（UTC）

sstv-rst-sent = RST送信

sstv-rst-received = RST受信

sstv-qsl-note = メモ

sstv-add-address = 送信するにはアドレスを追加してください。

sstv-saves-opens-mail-hint = 画像を保存してメールクライアントを開きます。添付はそこでワンクリックです。

sstv-browser-cannot-attach-hint = ブラウザのタブはメッセージにファイルを添付できないため、画像を保存してメッセージを開きます。添付は手動で行ってください。

sstv-save-open-mail = 画像を保存してメールを開く

sstv-mail-not-opened = 画像を保存し、メッセージをコピーしました。メールプログラムに貼り付けてください。メールハンドラを開けませんでした。

sstv-qsl-saved-where-chosen = { $name }（選択した場所に保存）

sstv-qsl-downloaded = { $name } — このブラウザのダウンロードフォルダにダウンロードしました

sstv-picture-editor = 画像エディタ

sstv-slot-n = スロット { $n }

sstv-slot-layers-hint = この送信スロット自身のレイヤー

sstv-station-overlay = 局オーバーレイ

sstv-station-overlay-hint = すべてのスロットが自身のレイヤーの下に描画する1組のレイヤーです。好きな隅にコールサインを置けます。一度設定すれば、各スロット自身のテキストと重ねて合成され、置き換えられることはありません。

sstv-draw-under-slot = このスロットの下に描画

sstv-draw-under-slot-hint = 局オーバーレイをこのスロット自身のレイヤーの下に描画します。再送信するモザイクなど、既に識別情報を含む画像ではオフにして、二重に押されないようにします。

sstv-overlay-note = 局オーバーレイ：すべてのスロットが、自身のレイヤーの下にこれらのレイヤーを描画します。画像上で行をドラッグして移動します。

sstv-slot-text-note = このスロット自身のテキストで、置いた場所に描画されます。画像上で行をドラッグして移動します。行のストリップも一緒に動きます。

sstv-no-picture-in-slot = このスロットに画像がありません。下から読み込んでください

sstv-base = ベース

sstv-base-hint = これらのレイヤーをどのスロットの画像の上に描画するか

sstv-compose-over-hint = このスロットの画像の上に合成します

sstv-slot-empty = このスロットは空です

sstv-picture-btn = 画像…

sstv-load-picture-hint = このスロットに画像を読み込みます

sstv-use-received = 受信画像を使用

sstv-use-received-hint = 届いたばかりの画像をこれらのレイヤーの下に置き、返信に相手の画像と自分のコールを載せます

sstv-received-now-base = 受信画像がベースになりました

sstv-add-text = ＋ テキスト

sstv-add-strip = ＋ ストリップ

sstv-n-lines =
    { $n } { $n ->
            [one] 行
           *[other] 行
        }

sstv-draw-this-line = この行を描画

sstv-strip-paren = （ストリップ）

sstv-draw-later-hint = この行を、上の行より後に描画します

sstv-draw-earlier-hint = この行を先に描画します

sstv-copy = コピー

sstv-face = 書体

sstv-colour = 色

sstv-line-text-hint = 行の内容

sstv-the-letters = 文字

sstv-composer-outline = 縁取り

sstv-the-edge = 文字の縁

sstv-placement = 配置

sstv-down-frame = フレームの縦方向

sstv-down-frame-hint = 行を90度回転させて下向きに読むようにします。画像の側面に沿ったコールサインなどに使います

sstv-strip = ストリップ

sstv-behind = 背面

sstv-behind-hint = 行の背後に、画像を横切る、または縦に渡る色の帯を、行を中心に置きます

sstv-strip-first-edge = ストリップの第1の縁

sstv-thickness = 太さ

adsbpanel-paths = PATHS

adsbpanel-paths-hint = 各機の航跡を1本の線で描きます。通ってきた場所、現在地、向かう先までをつなぎます。オフにすると、航跡は点の列で描かれます。

adsbpanel-nav = NAV

adsbpanel-nav-hint = 内蔵テーブルの空港と無線航法援助施設を画像の下に表示します。大きな空港は常に、中規模の空港はより近い範囲で、航法施設は局の位置が分かるほど拡大したときに表示されます。受信できる中波ビーコンは専用の色で描かれます。

adsbpanel-units = 単位

adsbpanel-units-hint = 航空機ビューでの高度、速度、上昇率、距離の表記方法です。デコーダーとエクスポートには影響しません。

adsbpanel-age = 経過

touchlock-locked-hover = 誤タッチ防止のため画面をロック中です。ここを3秒間長押しで解除します。

touchlock-unlocked-hover = 誤タッチ防止のため画面をロックします。周波数、バンド、モード、フィルター、ウォーターフォールは指に反応しなくなります。受信と音声は継続します。3秒間長押しで解除します。解除後10秒間は南京錠が休止し、タップしても再ロックされないため、解除した指でそのまま再ロックすることはありません。

touchlock-hold = 長押し { $s } 秒…

touchlock-unlocked = ロック解除

touchlock-locked-hold = ロック中 · 3秒長押し

touchlock-unlocked-relock = ロック解除 · { $s } 秒後にロック可

wake-held = 画面をオンに維持中

wake-asking = ブラウザーに要求中…

wake-unsupported = ここでは利用できません。対応ブラウザーと https ページが必要です

wake-refused = ブラウザーに拒否されました(省電力設定?)

wake-native = ブラウザー専用です。インストール版はシステムの電源設定に従います

tooltipcopy-copied = ✔ コピーしました

chrome-hide-message-hover = このメッセージを今後表示しません。設定は何も変わらず、アドバイスはマニュアルに載っています。

chrome-close-message-hover = このメッセージを閉じます。他は何も変わりません。

settings-ui-window-label = { $module } ウィンドウ

settings-ui-undock-hint =
    ドッキング解除すると、このモジュールは専用ウィンドウに移り、メインウィンドウは空いた場所を残りの部分に譲ります。ウォーターフォールを2台目のモニターに、操作部を3台目に置くこともできます。ウィンドウを閉じると再びドッキングします。

    Wayland ではサイズは復元されますが、ウィンドウの位置はコンポジターが決めます。フロート表示にし、app-id "{ $app_id }" に一致するウィンドウルールでモニターに固定してください。

settings-ui-workspace-restore-out = この配置に戻す — { $n } 個のモジュールを個別ウィンドウに

settings-ui-profile-in-use-named = 使用中: プロファイル { $name }

settings-ui-profile-in-use-default = 使用中: この局のデフォルト

settings-ui-profile-not-in-use = 未使用 — このログインには何も保存されていません

settings-ui-module-panadapter = パンアダプター

settings-ui-module-controls = コントロール

settings-ui-module-aux = AUX SP (第2スペクトラム)

settings-ui-module-band-menu = バンド・モード選択

settings-ui-broadcast-scope-in-view = このスパン内

settings-ui-fill-fixed = 固定

settings-ui-fill-gradient = グラデーション

settings-ui-fill-mixed = 混合

settings-ui-spot-kind-heard-me = 受信された

settings-ui-wake-held = 画面をオンに維持中

settings-ui-wake-asking = ブラウザーに要求中…

settings-ui-wake-unsupported = ここでは利用できません。対応ブラウザーと https ページが必要です

settings-ui-wake-refused = ブラウザーに拒否されました(省電力設定?)

settings-ui-wake-native = ブラウザー専用です。インストール版はシステムの電源設定に従います

settings-ui-workspaces = ワークスペース

settings-ui-workspaces-note = 名前付きのウィンドウ配置です。どのモジュールを個別ウィンドウにし、それをどこに置くかを保存します。現在の配置を保存するか、保存済みの配置を呼び出せます。最大10個。

settings-ui-workspaces-none = 保存されたワークスペースはまだありません。

settings-ui-workspace-restore-home = この配置に戻す — すべてのモジュールをメインウィンドウに

settings-ui-workspace-forget = このワークスペースを削除

settings-ui-workspace-name = この配置の名前

settings-ui-workspace-save = 現在の配置を保存

settings-ui-workspace-full = ワークスペースは SDRuno と同じく最大10個です。先に1つ削除してください

settings-ui-workspace-save-hint = 現在の配置をこの名前で保存します

settings-ui-workspace-name-first = 先に名前を入力してください

sstv-composer-fade = フェード

sstv-layouts = レイアウト

sstv-name-layout-hint = このレイアウトの名前

sstv-keep-layout-hint = この名前でレイアウトを保存します

sstv-layout-renamed = "{ $name }" に名前を変更しました

sstv-layout-saved-as = "{ $name }" として保存しました

sstv-rename-cancelled = 名前の変更を取り消しました

sstv-no-layouts-yet = 保存されたレイアウトはまだありません。上で名前を付けてください。

sstv-load-layout-hint = このレイアウトを編集中の内容に読み込みます

sstv-layout-loaded = "{ $name }" を読み込みました

sstv-rename-layout-hint = "{ $name }" の名前を変更

sstv-layout-renaming = "{ $name }" の名前を変更中

sstv-duplicate-layout-hint = "{ $name }" を複製

sstv-layout-copy-name = { $name } のコピー

sstv-layout-duplicated = "{ $name }" として複製しました

sstv-forget-layout-hint = レイアウト "{ $name }" を削除

sstv-layout-forgot = "{ $name }" を削除しました

sstv-mosaic-header = 受信画像からモザイクを作成

sstv-mosaic-nothing = まだ何も受信していません。モザイクはギャラリーの画像から作られます。

sstv-mosaic-picked = { $max } 枚中 { $n } 枚を選択、この順に配置します。順番がグリッド上の位置になります。各画像は元の形を保ち、すき間は黒です。

sstv-mosaic-earlier-hint = この画像を1つ前のセルに移動

sstv-mosaic-later-hint = この画像を1つ後のセルに移動

sstv-mosaic-max = モザイクは最大 { $n } 枚です

sstv-mosaic-compose = このスロットに合成

sstv-mosaic-compose-hint = モザイクを作成し、このスロットの画像として読み込みます。受信した元画像はそのまま残ります

sstv-mosaic-composed = { $n } 枚の画像をスロット { $slot } に合成しました

sstv-clear-picks = 選択をクリア

sstv-composed-saved = 保存済み

sstv-send = 送信

sstv-send-now-hint = この合成画像を今すぐ送信します

sstv-composed-sending = 送信中

sstv-forget = 破棄

sstv-forget-hint = このプリセットのドキュメントを空にします。画像はバナー帯と通常のキャプションの設定に戻ります

sstv-forget-done = クリアしました。バナーとキャプションに戻りました

sstv-copy-to-slot = スロットにコピー

sstv-copy-to-slot-hint = この画像とテキストを別のスロットに渡し、そこから送信できるようにします

sstv-copied-into-slot = スロット { $n } にコピーしました

sstv-word-strip = 帯

sstv-word-text = テキスト

sstv-text-size = 文字サイズ

sstv-text-size-hint = バナーの文字の大きさを、帯自体の大きさに対する割合で指定します。100% が従来のサイズです。QSO ではノイズの多いデコード後でもコールが読める必要があり、これは帯を高くせずに文字だけを拡大縮小します。

sstv-placement-hint = 帯を描く辺を選びます。メッセージのオーバーレイは反対側から始まるため、バナーを下にすると画像の上側がテキスト用に空きます。

sstv-top = 上

sstv-bottom = 下

topbar-ptt-hold-hint = 押している間送信します。クリックでラッチされ、次に押すと解除されます


topbar-above-the-dial = 上

topbar-below-the-dial = 下

topbar-step-chip-hint = 1回の操作でダイヤルが動く幅です。タップで次のステップに切り替えます: 10 Hz、100 Hz、500 Hz、1、2.5、5、6.25、9、10、12.5、25 kHz。この行は 設定 › UI でオフにできます。

topbar-mem-chip-hint = メモリーチャンネル — 保存した周波数を呼び出します

topbar-disp-menu-hint = パンアダプター(2つのレイヤー、速度と詳細、ピークホールド)に加え、ウォーターフォールのレベル、表示オプション、スキマー

topbar-phone-menu-hint = フォン用ストリップに表示されないすべて: バンド、モード、受信機と送信機、デコードウィンドウ、設定、マニュアル

topbar-group-band = バンド

topbar-group-system = システム

topbar-caption-diversity = ダイバーシティ

topbar-group-decode-windows = デコードウィンドウ

topbar-group-extras = エクストラ

topbar-grid-tracker-hint = グリッドトラッカー — 交信したメイデンヘッド・スクエアを地図上に表示

topbar-settings-chip-hint = 設定 — デバイスのゲイン、アンテナ、オーディオデバイス

topbar-windows-chip-hint =
    ▣  左クリック: すべてのモジュールウィンドウを戻す
    右クリック: モジュールウィンドウの一覧

topbar-manual-chip-hint = ユーザーマニュアル (F1)

topbar-module-windows-title = モジュールウィンドウ

topbar-wname-panadapter = SP1 — スペクトラム

topbar-wname-aux = SP2 (AUX) — 第2スペクトラム

topbar-wname-panel = オペレーションパネル

topbar-wname-controls = RX コントロール

topbar-wname-band-menu = バンド・モード

topbar-wname-rotor = アンテナローテーター

topbar-wstate-put-back = { $name } をこのウィンドウに戻す

topbar-wstate-floating-panel-only = フロート表示はオペレーションパネル専用の状態です。スペクトラムやコンソールはこのウィンドウ内の1か所に固定されます

topbar-wstate-float = { $name } をペインから外し、このウィンドウの上にフロート表示する

topbar-wstate-no-panel = { $mode } にはオペレーションパネルがありません。パネルのあるモードで適用されます

topbar-wstate-own-window = { $name } を専用ウィンドウにして、任意のモニターに置く

topbar-wstate-here-now = { $name } は現在ここにあります

topbar-dock-all = すべてドック

topbar-dock-all-hint = すべてのモジュールウィンドウをこのウィンドウに戻します

topbar-undock-all = すべて分離

topbar-undock-all-hint = すべてのモジュールを専用ウィンドウにします。このウィンドウにはスペクトラムとデコーダーだけが残ります

topbar-rx-tx-freq = RX { $rx }   TX { $tx } MHz

topbar-band-dock-hint = ドック — このポップアップを毎回閉じる代わりに、バンド・モード選択をウォーターフォールの横に開いたままにします

topbar-band-window-hint = ウィンドウ — バンド・モード選択を専用ウィンドウで開きます。好きな幅にでき、ウォーターフォール横の任意のモニターに置けます

topbar-band-return-popup-hint = ドック — 選択をトップバーのポップアップに戻します。選択のたびに閉じます

topbar-band-window2-hint = ウィンドウ — 選択を専用ウィンドウで開きます。好きな幅にでき、任意のモニターに置けます

topbar-squelch-held-hint = スケルチを開いたまま保持中です。クリックで元の設定に戻します。

topbar-squelch-hold-hint = クリックでスケルチを開いたまま保持し、何か出ていないか聴けます。もう一度クリックで元の状態に戻ります。

topbar-changed-tone = トーン

topbar-changed-loudness = ラウドネス

topbar-nr-chip-hint =
    ノイズリダクション: { $strength } — { $engine }

    { $advice }

    クリックで強度を切り替えます: 弱 / 中 / 強 / オフ。エンジンはこの下の行で選びます。

topbar-nr-off-narrow-hint = ノイズリダクションはオフです。この下の行に5種類のデノイザー(RNNoise、DeepFilterNet3、spectral-bleach、NR2、従来のスペクトラル)があり、ここをクリックすると強度を選んでオンになります。音声にはまず RNNoise をお試しください。

topbar-eq-tone-hover = トーン: 低音 { $bass } dB、中音 { $mid } dB、高音 { $treble } dB

topbar-eq-closed-hover = トーン — 受信音声用の3バンドイコライザー。クリックで開きます

topbar-eq-loudness-on = ラウドネス オン

topbar-stereo-yields-hint = ノイズリダクションまたはオートノッチの動作中は WFM ステレオがオフになります。左右のチャンネルがにじむためです。ステレオで聴くにはそれらをオフにしてください

topbar-stereo-playback-hint = ステレオ: 録音を収録どおり両耳で再生中です。クリックでモノラルに固定します

topbar-stereo-mono-recording-hint = この録音はモノラルのため、再生する第2チャンネルがありません

topbar-preset-edges = { $lo } … { $hi } Hz

topbar-caption-tone = トーン

topbar-eq-on = オン

topbar-eq-bass = 低音

topbar-eq-mid = 中音

topbar-eq-treble = 高音

topbar-eq-remembered = { $mode } 用に記憶されました。モードごとに別々に保持されます。

topbar-loudness-hint = 音量を下げるとき低音(と少しの高音)を持ち上げ、小さな音でも豊かに聞こえるようにします。トーンの有無にかかわらず動作します。最大音量では何もせず、最大音量時より大きくなることもありません。デフォルトはオフで、現在のモードごとに記憶されます。

topbar-caption-record-audio = 音声を録音

topbar-stop-recording-after-min = { $minutes } 分後に録音を停止

topbar-caption-play-back = 再生

topbar-recordings-chip = ▶ 録音…

topbar-recordings-hint = 録音をデコーダーに再生します。取りこぼした SSTV 画像、弱い RTTY や CW 信号、FT8 のスロットなどを、ウォーターフォールとデコードログとともに、実際の電波のように扱えます。再生中は無線機は待機します。

topbar-ant-label = ANT

topbar-pwr-label = PWR

topbar-drive-less = 小

topbar-drive-more = 大

topbar-fill-hint = スペクトラム線の下の領域を、下端で濃く、線に向かって薄れるように塗ります。信号が輪郭だけでなく面積でも目立ちます。色は 設定 → UI → スペクトラム線 で選びます。3Dサーフェスでは描画されません。

topbar-group-spectrum = スペクトラム

topbar-group-waterfall = ウォーターフォール

topbar-freeze-on-tx-hint = 自局の送信中はウォーターフォールのスクロールを止め、終了後に続きから再開します。キャリアの行(受信機を送信中ミュートする無線機では、実際には存在しなかったノイズの行)が並ぶのではなく、連続した受信履歴の画像になります。スペクトラム線には影響しません。頻繁に送信するモードの監視に便利で、相手局のターンが自分の送信で画面から押し出されません。

topbar-fft-chip-hint = ウォーターフォールのレベルとコントラスト、FFT サイズ、スクロール方向

topbar-caption-waterfall = ウォーターフォール

topbar-flatten-hint = バンド全体でノイズフロアを平坦にします。周囲より静かな区間は隣のフロアに近づくよう持ち上げられ、混雑したバンドでも弱い信号が見えます。持ち上げるだけでキャリアが下げられることはなく、上げ幅は最大 20 dB です。上のスペクトラム波形は測定値のままです。

topbar-log-swl-hover = 受信ログ — 受信した局を SINPO/SIO とともに記録

topbar-programs-hint = 放送スケジュール — いつ、どこで、何が放送されているか

topbar-listen-hint = このスパンで聞こえるものを地図上に表示します。局を選んで同調し、聞いた内容を書き込み、ログに記録できます

topbar-spots-hint = ライブスポット — PSK Reporter(SWL モード以外では DX クラスター、POTA、SOTA も)

topbar-sigid-hint = このダイヤルで聞こえているのは何か? 周波数、モード、帯域幅から信号を調べるガイド

topbar-extras-hint = エクストラ — DAB などの受信機を、分離できるウィンドウで

topbar-fill-style-caption = 線

topbar-fill-style-hint = スペクトラム線の下の色の濃淡のつけ方です。色自体は 設定 → UI → スペクトラム線 で選びます。

topbar-fill-fixed = 固定

topbar-fill-fixed-hint = 線の下のどこでも同じ1色

topbar-fill-gradient = グラデーション

topbar-fill-gradient-hint = スペクトラムの下端で濃く、上に向かって薄れる

topbar-fill-mixed = 混合

topbar-fill-mixed-hint = 下半分は濃く、上半分で薄れる

topbar-fill-switch-on = これを設定するには塗りつぶしをオンにしてください

topbar-nothing = なし

topbar-olivia-unconfirmed = OLIVIA — 受信は動作しますが、送信は他のプログラムとの照合が未実施です。このデコーダーは他局を読み取れる(実電波で確認済み)ので、Olivia の QSO をコピーできます。自局の送信は自局の受信機を通した往復では成功しますが、fldigi や MultiPSK ではコピーされておらず、符号の取り決めが fldigi のソースと1か所異なります。そのため、返信を期待せず、最初のオーバーを確認してください。

topbar-dab-bandwidth = DAB / DAB+、実電波で確認済み。音声が出るにはフロントエンドで 2.5 Msps の帯域が必要です。アンサンブルが占める 1.536 MHz では受信機は ADC の下限にありサンプルを取りこぼし、DAB のデコードには連続したストリームが必要なため、アンサンブルがロックせず何も聞こえません。先に無線機のサンプルレートを 2.500 Msps に設定してください。

topbar-stanag-unproven = STANAG 4285 — 受信のみ。識別は実電波で確認済みです。4285 はロックし、106.67 ms のフレームが測定され、参照ブロックが収束するため、パネルは周波数上の信号とそのレートを示せます。*テキスト*の読み取りは実電波では未確認です。Annex E の符号とインターリーバーは生成した信号を正確にデコードしますが、符号化された局はまだコピーできていません。表示された符号化テキストは本物です。テキストがまったく出ない場合、その局は非符号化か暗号化トラフィックを送っており、これが普通です。

topbar-cis11-unproven = CIS-11 (TORG-11) — 受信のみ。ロシアの気象用テレプリンターで、100 ボーの 2-FSK、シフト約 500 Hz、奇数パリティビット付きの 11 ビット文字です。実際の録音で測定して確認できたのは、受信機が自らトーンを見つけ、アイドル送信を 123 文字中 121 文字のパリティ正常で読み取ることです。未解決なのは、10 個のデータビットがどう文字に区切られるかで、そのためパネルは ITA-2 としての読みに加えてビットも表示します。また、トラフィック自体は暗号化されており、これが普通です。

topbar-thor-unproven = THOR — 受信側はまだ fldigi の送信を読み取れません。デコーダーは自身の変調器に対しては動作しますが、実電波や参照録音のコピーは未実施のため、表示内容は暫定的なものとして扱ってください。

topbar-mode-not-on-band = { $mode } は { $band } では使われません。対応するバンドを選んでください

topbar-radio-classic-hint = RADIO CLASSIC — 放送受信向けのクラシックなラジオ表示。オンにすると受信機を占有し、他のモードとそのウィンドウは閉じられます

topbar-ats-bands-caption = 受信機が対応するバンド

topbar-ats-band-hover = { $name } — { $mhz } MHz、{ $mode } のデフォルト設定。受信機をこのバンドに切り替えます。

topbar-caption-mode = モード

topbar-caption-digital = デジタル

topbar-ats-receive-only = 受信専用 — Si4732 はハードウェアで復調するため、キーイングするものはありません。

topbar-band-forecast-derived =
    { $band }: ≈{ $verdict } ({ $time ->
            [day] 昼間
           *[night] 夜間
        }) — 公表されている { $group } グループの評価で、最も近い代用です。HAMQSL.com は { $band } 自体は評価していません。予報であり、自分の伝搬路の測定ではありません。

topbar-keypad-readout-khz = { $digits } kHz

topbar-caption-band-keypad = バンドキーパッド

topbar-keypad-bands = バンド

topbar-keypad-bands-hint = キーはバンド名です。押すとそのバンドに移動し、現在のモードのままです。MHz から戻ると入力中の内容は消えます。

topbar-keypad-mhz-hint = キーで周波数を kHz 単位で入力します。14074 は 14.074 MHz です。ENTER で送信、CLEAR で入力を消去します。

topbar-keypad-key-stay = { $mode } のまま { $mhz } MHz に同調します — { $band } で運用される周波数です。

topbar-keypad-key-go = { $band } へ移動します。

topbar-keypad-clear = クリア

topbar-keypad-clear-typing-hint = 入力した周波数を消去します。キーは MHz のままです。

topbar-keypad-clear-bands-hint = 何も入力されていません。キーはバンド選択になっており、押すとバンドに移動します。

topbar-keypad-enter = 決定

topbar-keypad-enter-empty-hint = まだ何も入力されていません。キーは MHz です。先に入力してください。

topbar-keypad-enter-ok-hint = { $mhz } MHz に同調し、バンドキーに戻ります。

topbar-keypad-enter-range-hint = { $mhz } MHz はこの無線機の受信範囲外です({ $ranges } MHz)。

topbar-nothing-published = 公開なし

topbar-tab-listen = 受信

topbar-tab-operate = 運用

topbar-listen-tab-note = このタブでは送信コントロールも隠せます。設定 → UI で「受信タブで TX を隠す」をオンにしてください

topbar-caption-show-bands = 表示するバンド

topbar-band-filter-hint = 下の一覧に { $filter } バンドのみを表示します。もう一度クリックですべてのバンドを表示します。

topbar-show-every-band = すべてのバンドを表示

topbar-all-chip = ALL

topbar-all-chip-hint = ゼネラルカバレッジ: バンドを解除し、ダイヤルを自由に動かせます。この一覧を絞り込むだけのバンドフィルターとは別物です。

topbar-caption-sw-metre-bands = SW メートルバンド

topbar-metre-band-hint = { $name } 放送の中央に同調します

topbar-caption-wideband = ワイドバンド

topbar-caption-cb = 11M/CB

topbar-caption-experimental = 実験的(未検証)

topbar-caption-receive-modes = 受信モード

topbar-wav-hint = 同じ音声を非圧縮の 16 ビット PCM、48 kHz で保存します。ステレオで1分あたり約 11 MB。エディターや自作のデコーダー向けです。

topbar-mode-offset-hint-side = このモードでは信号はダイヤルの{ $side }にあります。表示される数値は同調するダイヤルの値で、信号は { $mhz } MHz にあります。

levelslider-hover =
    ウォーターフォールのレベル
    下限 { $floor } dB · 上限 { $ceil } dB

    ドラッグまたはスクロールで明るさを調整、Shift でコントラスト。
    動かすとオートフィット (FIT) がオフになります。

smeter-audio-level-hover =
    音声レベル: { $dbfs } dBFS{ $over }。

    この無線機は**報告できる S メーターを持っていない**ため、これは無線機が送ってくる音声のレベルで、無線機自身の AGC とスケルチが適用済みの状態でここで測定したものです。RF ではなく音声に追従するため、サウンドカードのノイズ、無線機の近くのワイヤレスマウス、AGC のポンピングなどで、信号が変わらなくても動きます。

smeter-audio-over = (音声スケールで S9 + { $over } dB)

spectrum-offset-left = ◀ トーン { $hz } Hz

spectrum-offset-right = { $hz } Hz トーン ▶

worldmap-no-snr = SNR なし

solar-tune-the-radio-to-this-downlink = 無線機をこのダウンリンクに同調

qo100-width-label = 幅

qo100-park-label = パーク

qo100-led-carrier = キャリア

qo100-led-sync = 同期

scanner-stopped = 停止

ism-band-label = バンド

amp-none = なし

colormap-classic = クラシック

colormap-viridis = Viridis

colormap-gray = グレー

colormap-icom = Icom

colormap-neon = ネオン

colormap-synthwave = シンセウェイブ

colormap-matrix = マトリックス

colormap-tron = Tron

colormap-amber = アンバー

colormap-rainbow = レインボー

colormap-blue = ブルー

colormap-calmer = 落ち着き

colormap-compline = Compline

colormap-lauds = Lauds

settings-cb-wide-calls-hint =
    WSJT-CB の { $narrow } に加えて、より広い 11 m コールサイン形式 { $wide } (中央に3文字、後ろに最大4桁)を受け付けます。実験的機能: WSJT-CB 自体はこの広い形式を拒否するため、これが必要なコールは WSJT-CB 局では理解されない場合があります。確実に送れる範囲より、この局が受信できる範囲を広げる設定です。オフにすると WSJT-CB の厳密な文法のままです。

    この切り替えが変えるのは*形式*であって*長さ*ではありません。11 m のコールは /P、/MM、/QRP などの接尾辞を含めて 11 文字までで、19DCG3733/P は収まりますが 19TST1001/QRP は収まりません。

settings-idle-stop-disable = 無効

settings-xvtr-tx-off-hint =
    変換中はオフ: 受信のみです。この行はトランスバーターではなくコンバーターのため、このバンドでキーイングされるものはありません。

    同じコンバーターを通す場合: 双方向で動作し、送信にも同じオフセットが適用されます。

settings-xvtr-drive-hint =
    このトランスバーターを選択中の送信ドライブの上限を、フルに対する割合で指定します。トランスバーターの I.F. 入力はミリワット単位で、無線機自身のバンドに適したドライブでは破損するため、この行で最も重要な項目です。100% は制限なしです。

    ドライブ設定は移動されずにこの上限の下に保持されるため、ダイヤルがこのバンドを離れると HF で使っている値に戻ります。

settings-brown-build-note = sdroxide の Brown ビルド: 上流のプログラムに、11 m 市民バンド、短波受信、デコードを加えたものです。これらのバンドとツールが加わった通常のアマチュア局として動作します。

settings-gain-by-band = バンドごとにフロントエンドゲインを記憶

settings-gain-by-band-hint =
    バンドごとに別々の受信ゲインを保持し、ダイヤルがそのバンドに入ると復元します。HF のゲインと 11 m または VHF のゲインを一度設定すれば、無線機がバンドに追従します。

    デフォルトはオフです。オフの間は従来どおり、すべてのバンドで1つのゲインが適用されます。オンにすると現在のバンドが記録され、ゲインのないバンドは残りません。一度も設定していないバンドは現在のゲインを引き継ぎます。

settings-callsign-grid-note = コールサインとグリッドは、FT8/FT4/FT2、SSTV 画像ヘッダー、CB デジタルモード、ログブックで共有されます。これは**送信する**ときの識別で、11 m を運用するなら CB コールサイン、それ以外ならアマチュアのコールサインです。**受信レポート**に署名する名前については、スポットタブの **レポート名 (SWL)** 欄を参照してください。FT8 / SSTV の設定ダイアログからも編集できます。

settings-cb-grammar = 11 m / WSJT-CB コールサイン文法

settings-cb-wide-calls = ワイド CB コールサイン — 実験的

settings-cb-wide-calls-note = 11 m のコミュニティが3文字のグループや4桁のユニット番号を使う場合にオンにします。CB デジタルモードのデコードと送信に適用されます。どちらの場合も、11 m のコールは接尾辞を含めて**11 文字**を超えられません。19DC3733/P は収まりますが 19TST1001/QRP は収まりません。

settings-cb-plan-hint = ダイヤルが読み取るチャンネルプランを選びます。11 m 市民バンドのプランと、446 MHz の PMR446 のプランです。変わるのはチャンネル番号とバンドを開いたときのチャンネルだけで、バンド端はそのままなので、切り替えても受信や送信の範囲は動きません。ダイヤルがプランのチャンネル上にあるとき、チャンネル番号が同調ラインに描かれます。

settings-cb-allow-tx = 11 m (CB) での送信を許可

settings-cb-allow-tx-hint = アマチュアバンドのロックアウトは、アマチュア割当でないすべてのバンドで送信を拒否し、11 m もその1つです。独自の規則と型式認定機器を持つ別の無線業務だからです。これをオンにすると 11 m に限って送信が許可され、他は変わりません。放送バンドは受信専用のままです。初回は確認が求められます。

settings-cb-licence-note = CB は多くの国で免許不要ですが、すべての国ではなく、許可されるチャンネル、モード、出力も異なります。バンドの使用はお住まいの国の規則に従います。

settings-radio-callsign-hint =
    この無線機が送信、ログ記録、スポットに使うコールサインです。空欄にすると 一般 タブの局コールサインを使います。これがデフォルトで、識別が1つの局に適しています。

    別人の無線機に設定します。11 m の機器には CB コールサイン、HF 機にはアマチュアコールサインというように、互いに別のものを保持できます。一般タブのコールサインは、上書きしないすべての無線機のフォールバックです。

    受信*レポート*は、これを変更しなくても、スポットタブの SWL 番号で署名できます。

    適用で有効になります。

settings-radio-swap-iq-own = このインターフェースには、このページの下のセクションに独自の I/Q 反転があります。そちらを使ってください。

settings-radio-swap-iq-hint =
    復調の前にこの受信機の I/Q を反転します。受信機の I と Q が逆で、SSB が逆のサイドバンドに出る(LSB が USB として聞こえる)うえ、スペクトラムがダイヤルを中心に鏡像になる場合に使います。

    ハードウェアの特性であり、サインインしているユーザーとは無関係です。

    適用で有効になります。

settings-radio-transmit-controls = 送信コントロール

settings-radio-hide-tx-all = すべて隠す (SWL モード)

settings-radio-hide-tx-hint =
    { "*" }この*無線機のすべての送信コントロール(PTT、CALL CQ、TX レベル、SEND、BEACON など全部)を隠し、ストリップのアマチュア向け機能(スポット、アワード)を受信者向け(スケジュールと SWL ログ、受信ログ)に置き換えます。無線機ごとの設定です。ロングワイヤーにつないだ受信専用ドングルをこのモードにしても、隣のトランシーバーは送信機能を保てます。

    ハードウェアは送信できる状態のままで、隠れるのはコントロールだけです。`--swl` を付けると、その実行中はすべての無線機で強制的にオンになります。

    適用で有効になります。

settings-radio-auto-mode = オートモード

settings-radio-auto-stop-label = 無操作で停止

settings-radio-auto-stop-hint-long =
    無人の FT8/FT4/FT2 シーケンサーが、オペレーターの入力なしで解除されるまで動作できる時間です。無線機ごとの設定で、ある機器を別の機器より長く動かしたままにできます。

    PlutoSDR では最大6時間までで、無効にすると停止が完全にオフになり、オートモードはあなたが止めるまで動き続けます。送信ウォッチドッグとバンドロックアウトは引き続き適用されます。

    適用で有効になります。

settings-radio-auto-stop-hint =
    無人の FT8/FT4/FT2 シーケンサーが、オペレーターの入力なしで解除されるまで動作できる時間です。無線機ごとの設定で、ある機器を別の機器より長く動かしたままにできます。

    意図的に45分が上限です。オートモードは少し席を外すためのもので、誰もいない局でコンテストを運用させるためのものではありません。

    適用で有効になります。

settings-radio-smeter-calib = S メーター校正

settings-tx-eq = 送信 EQ

settings-tx-eq-hint = マイク音声に、変調器の前でかける3バンドのパラメトリック EQ です。音声モード(SSB/AM/FM)のみ有効で、デジタルモードと CW は合成音声やキーイングの音声のため、これを通りません。デフォルトはオフで、オンにしてもフラットなので、下のバンドを実際に動かすまで電波には何も変わりません。即時に反映されます。

settings-tx-eq-low-hint = ランブルとハンドリングノイズをカット/ブーストします。

settings-tx-eq-mid = 中域ピーク

settings-tx-eq-mid-hint = プレゼンス: 音声帯域内の狭いピークまたはディップ。

settings-tx-eq-high-hint = 明るさ/ディエッサー。

settings-cb-call-too-long = ⚠ 11 m には長すぎます — { $len } 文字で、FT8 が CB コールに使えるのは /P、/MM、/QRP の接尾辞を*含めて* 11 文字です。このアクティベーションのコールサインは送信できません。19DC3733/P (10) は収まりますが、19TST1001/QRP (13) は収まりません。

settings-cb-plan-note = 11 m (CB) は国によって同じバンドではありません。ここでは 11 m ダイヤルがどの国のチャンネルを読み取るかを選びます: { $plan }。変わるのはチャンネルとバンドを開いたときのチャンネルだけで、バンド端は広く取ってあるため、プランを切り替えても受信や送信の範囲は変わりません。この局のすべての無線機に適用されます。

settings-cw-enter-mode = 保存された値を読み込むには、一度 CW(または任意のデジタルモード)に入ってください。

settings-cw-where-from = 入力元:

settings-cw-straight-key-hint = `CW straight key` の割り当て(デフォルトはスペース)は、CW パネルの KEY がオンの間に読み取られます。接点は1つなのでストレートキーとして扱われ、アプリはオペレーター自身のタイミングをデコードします。

settings-cw-speed = 速度:

settings-cw-speed-note = キーヤーのトン・ツーの長さ

settings-cw-key-tx = 送信機をキーイング

settings-cw-key-tx-hint = オフ: キーはローカルのトレーナーだけを動かします。オン: 通常の手動キーの経路で無線機をキーイングし、バンドロックアウトとウォッチドッグが働きます。

settings-cw-key-self-note = 自身でキーイングする無線機では、設定 → 無線機 → CW キーイング を `Sound card (MCW)` にしてください。リグのキーヤー経路での手動キーイングは設計上拒否されます。

settings-detachable-windows = 分離可能なウィンドウ

settings-detachable-windows-note = パネルを独立した OS ウィンドウに引き出します。ウィンドウを閉じると再びドッキングします。

settings-tool-windows = ツールウィンドウ

settings-tool-windows-note = フローティングウィンドウに変換されたパネルです。開閉の状態は記憶されます。

settings-tool-window-scanner = スキャナー

settings-tool-window-memories = メモリー

settings-tool-window-playback = 再生

settings-tool-window-open = 開く

settings-tool-window-closed = 閉じる

settings-detachable-native-only = 分離可能なウィンドウはネイティブ版のみです。

settings-psk-reporter-band-hint = バンドアクティビティのフィード: 現在のバンドで受信されている局を色付きの点で表示します。地図の「受信された」チップの背後にある「誰が私を受信したか」の照会も有効にし、1時間ごとに再取得します。

settings-psk-upload-hint = この局が受信した内容を pskreporter.info に報告し、受信局として表示されるようにします。一般タブのコールサインとグリッド、または設定されていれば下の SWL 番号を使います。

settings-swl-report-as = レポート名 (SWL)

settings-swl-report-as-hint = 受信レポートに使うリスナー自身の識別です。登録済みの SWL 番号、クラブ番号、名前などです。設定すると、コールサインの代わりに、この局が*受信した*内容(PSK Reporter と WSPRnet へのアップロード、SWL ログが書き出す受信レポート)に署名します。空欄ならコールサインで報告します。送信されることも、無線機をキーイングすることも、QSO やスポッターの名前になることもないため、コールサインのないリスナーも報告できます。

settings-wsjtcb-spot-server = WSJT-CB スポットサーバー

settings-wsjtcb-report = 自分の 11 m デコードを WSJT-CB スポットサーバーに報告

settings-wsjtcb-report-hint = 11 m バンドでデコードした各スポットを CB コミュニティのスポッティングサービス(xzgroup.net)に送信します。WSJT-CB 自身のクライアントが投稿できるのと同じサーバーです。デフォルトはオフで、サードパーティのサービスのため、あなたが求めるまで何も送信されません。スポッターはあなたのコールサインになります。PSK Reporter は上の別のスイッチです。

settings-eqsl-qth-hint = アップロードをどの QTH プロファイルに登録するかです。アカウントが複数持っている場合のみ必要で、そうでなければ空欄のままにします。

settings-log11dx-api-url = API URL

settings-log11dx-api-token = API トークン

settings-log11dx-note = どちらも LOG11DX のプロファイルページから取得します。専用の WSJT-X ブリッジは不要で、sdroxide が QSO を記録してそのまま投稿します。サイトはトークンを伏せ字にするため、生成時にコピーしてください。他の認証情報と同じく net.json に保存されます。

settings-pan-attached-note =
    この無線機のスペクトラム、ウォーターフォール、およびそれらから読み取るものはすべて、その無線機のタブにあり、これ自体には存在しません。上のインターフェース設定は引き続きこの無線機のもので有効です。変更後は 適用 / 再接続 を押すと、両方の無線機が一緒に立ち上がります。

    再び単独で使うには、その無線機のページで パンアダプター受信機 の欄をクリアしてください。

settings-pan-source-hint =
    この無線機のために受信する無線機です。貸し出し中はタブ一覧から消えます(フロントエンドがこの無線機のエンジンに属するため)。なしに設定するとすぐに戻ります。

    適用で有効になります。

settings-pan-offset-hint =
    受信機がこの無線機のダイヤルよりどれだけ上にあるかを、ヘルツで指定します。同じアンテナの受信機なら 0、I.F. タップなら中間周波数です。9 MHz の I.F. なら 9000000、70.455 MHz なら 70455000。

    これは無線機を再同調する上の コンバーター 欄とは別物で、この無線機自身のサウンドカードに関する CAT タブの I/Q 中心オフセットとも別物です。ここに入力した内容はどちらの無線機にも送信されません。

    適用で有効になります。

settings-cw-device = デバイス:

settings-cw-device-none = (見つかりません)

settings-cw-device-auto = 自動 (最初の "key" という名前)

settings-cw-reverse-hint = トン・ツーを入れ替えます。パドル本体のスイッチ用です。

settings-cw-key-type = キーの種類:

settings-cw-key-straight = ストレート

settings-cw-key-iambic-a = アイアンビック A

settings-cw-key-iambic-b = アイアンビック B

settings-cw-usb-note = 2つの接点を(多くはマウスのボタンとして)報告するキーヤーボックスは使えます。独自にアイアンビックを処理して完成したタッチを送るものは使えません。キーが有効な間、デバイスは占有されるため、その接点が他のウィンドウでクリックとして働くことはありません。

settings-cw-usb-linux = USB パドルには Linux デスクトップが必要です。

settings-smeter-zero = 0 (測定値のまま)

settings-smeter-cal-hint =
    この無線機の S メーターが高すぎる、または低すぎる場合に、6 dB の S ユニット単位で補正します。

    負の値は表示を下げます。ノイズフロアが S2 のはずなのに S5 と表示されるように、表示が多すぎるメーター向けです。正の値は上げます。0 は測定値のままです。

    動くのは表示だけです。針、S と dBm の数値、受信レポートの出発点のレベル、音声での S メーター読み上げが対象で、信号、スケルチ、録音、デコーダーには影響しません。

    接続しているマシン上でこの無線機とともに保存され、すぐに有効になります。

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1 と 2 はどちらも { $host }:{ $port } を使っています。すべてのスポットが2回届きます。別々のポート(7000 CW/RTTY、7001 FT8/FT4)にするか、片方をオフにしてください。

settings-rx-site-station = 局内

settings-rx-site-elsewhere = 別の場所

settings-remote-tab-note =
    局は、ここで既に開いている無線機の横のタブとして表示され、ウィンドウ上部のタブ一覧で切り替えます(⊞ で2つを並べて表示)。閉じるには 無線機 タブ上部の一覧から行います。これは切断するだけで、サーバー側は何も変更されません。

    サーバーがユーザー名とパスワードを求める場合は、新しいタブにサインイン画面が表示されます。通常の ws:// リンクはこれらを暗号化せずに運ぶため、インターネット越しでは VPN か SSH トンネルを通すか、サーバーの前段で HTTPS を終端し、上の セキュア にチェックを入れてください。

settings-cw-source-keyboard = キーボード

settings-cw-source-usb = USB パドル

pane-frame = フレーム

pane-frames = フレーム

pane-words = ワード

antenna-feed-z0 = 給電 Z0:

antenna-feed-z0-hint = SWR 計算用の給電線/システムのインピーダンス(通常 50 Ω)

antenna-height-label = 高さ:

antenna-height-hint = 給電点/アンテナの地上高(打ち出し角やパターンに影響)

antenna-m-agl = m 地上高

antenna-apply = 適用

antenna-apply-hint = プリセットに対してモーメント法ソルバーを実行します。

antenna-yagi-optimizer = 八木オプティマイザー

antenna-stack = スタック:

antenna-objective = 目的:

antenna-optimize = 最適化

antenna-swr-value = SWR { $v }

antenna-swr-ref = SWR { $v } ({ $z }Ω)

antenna-radiation-pattern = 放射パターン

antenna-view = 表示

antenna-view-front = 正面

antenna-view-side = 側面

antenna-view-top = 上面

antenna-3d-hint = 3D ドラッグで回転 • 色 = 電界強度

antenna-yagi-x = 八木 ×{ $n }

settings-tool-window-session-qsos = セッション QSO

sessionqso-time = 時刻

sessionqso-call = コール

sessionqso-grid = グリッド

sessionqso-band = バンド

sessionqso-mode = モード

sessionqso-rst-s = RST 送

sessionqso-rst-r = RST 受

sessionqso-country = 国

sessionqso-dist = 距離

sessionqso-final = 最終

sessionqso-count = このセッションで { $n } QSO

sessionqso-include-cw = CW を含める

decodes-live-log = ライブログ

decodes-setup = ⚙ 設定

schedule-colour-all = 色

schedule-colour-all-hint = すべての局のウォーターフォールラベルの色です。局ごとに選んだ色は置き換えられます

schedule-colour-station-hint = この局のウォーターフォールラベルの色

schedule-colour-reset = デフォルト

schedule-colour-reset-hint = すべての局を標準の色に戻し、個別に選んだ色を破棄します

settings-language = 言語

settings-language-note = インターフェースを表示する言語です。システムのデフォルトはオペレーティングシステムに従い、他を選ぶと上書きされます。選択は保存され、記憶されます。

aprs-all-chip = ALL

aprs-all-chip-hint = オン: 宛先に関係なく、チャンネルで聞こえたすべてのメッセージ。オフ: 自分宛てと自分発、およびブレティンのみ。

aprs-composer-hidden-swl = 送信は非表示です。SWL / 受信モードがオンになっています。メッセージを書いて送るにはオフにしてください。

aprs-compose-count-hint = APRS メッセージが運べる 67 文字のうち、使用した文字数です。

tx-bands-title = 送信バンド

tx-bands-note = この無線機が実際に送信するバンドに限り、TX を YES にしてください。これは他のすべての送信設定より優先され、YES にしても他の安全制限は解除されません。

tx-bands-careful = ⚠ バンドを誤ると無線機が破損するおそれがあります。すべての項目を慎重に確認してください。

tx-bands-yes = YES

tx-bands-no = NO

tx-bands-confirmed = ✓ この無線機の送信バンドは確認済みです

tx-bands-ack = これらのバンドを自分の無線機に照らして確認しました

tx-bands-confirm = これらのバンドを確定

tx-bands-confirm-hint = 先にチェックボックスにチェックしてください。確認は無線機ごとに1回です。

tx-bands-check = バンドを確認…

tx-bands-notice-new = すべてのバンドで送信はオフです。この無線機が送信するバンドを選び、一度確認してください。無線機に危険が及ぶため、選んだ項目を慎重に確認してください。

tx-bands-notice-carried = 送信バンドは以前のバージョンから引き継がれました。一度確認して確定するまで、送信はブロックされたままです。無線機に危険が及ぶため、すべての項目を慎重に確認してください。


adsbmap-surface-asphalt = アスファルト

adsbmap-surface-concrete = コンクリート

adsbmap-surface-grass = 芝

adsbmap-surface-gravel = 砂利

adsbmap-surface-dirt = 土

adsbmap-surface-water = 水面

adsbmap-airport-runway-lit = 灯火あり

adsbmap-airport-more = ほか { $n } 件

adsbmap-airport-freq-head = 周波数、MHz

adsbmap-airport-data-date = 滑走路と周波数のデータは { $date } 時点
aircraft-online-switch = オンライン

aircraft-online-switch-on-hover = オンライン: 機種、運航者、経路、写真は adsbdb と Planespotters から、空港の写真は Wikipedia から、空港の現地時刻と気温は Open-Meteo から、あなたの局があなたに代わって取得します(送られるのは空港の位置のみ)。クリックでオフにでき、局はこの選択を保持します。それ以外は何も局の外に出ません。

aircraft-online-switch-off-hover = オフライン: 何も局の外に出ません。クリックで機種、運航者、経路、写真(adsbdb と Planespotters)、空港の写真(Wikipedia)、空港の天気(Open-Meteo)をオンにします。

aircraft-switch-held = 局のオペレーターが SDROXIDE_AIRCRAFT_ONLINE で固定しています

adsbmap-airport-photo-credit = 写真: Wikipedia

adsbmap-airport-local-time = 現地時刻 { $time } ({ $zone })

adsbmap-airport-temperature = 気温 { $temp }

adsbmap-airport-weather-credit = 天気: Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time } (現地時刻: 読み込み中…)

adsbmap-airport-utc-offline = UTC { $time } (現地時刻: オフライン)

schedule-listen = 受信

schedule-listen-hint = 受信機をこの局に同調します

schedule-sort = 並べ替え

schedule-sort-asc = 昇順

schedule-sort-desc = 降順

schedule-col-freq = 周波数

schedule-col-band = バンド

schedule-col-name = 名称

schedule-col-lang = 言語

schedule-col-target = 対象地域

schedule-col-site = 送信所

schedule-col-local = 現地

tx-bands-off = { $band } では送信がオフです。このバンドは設定で許可されていないため、あなたの送信機はこのバンドでの送信に対応していません。(1セッションにつき最大 { $max } 回表示されます。)

tx-off-band = 送信禁止 — { $freq } MHz（{ $band }）は確認済みの送信バンドではありません。設定 → 無線機 → 送信バンドでチェックして確認してください。

tx-off-unconfirmed = 送信禁止 — この無線機の送信バンドはまだ確認されていません。設定 → 無線機 → 送信バンドで確認してください。

aircraft-hover-lookup-retry = 応答なし。{ $s } 秒後に再試行します

adsbmap-airport-navaids-head = 場内の無線航行援助

adsbmap-airport-no-freq = この空港の周波数は公開されていません

topbar-preset-right-click = 右クリックでこのプリセットを変更

topbar-preset-edit = プリセットを編集

topbar-preset-label = ラベル

topbar-preset-label-hint = チップに表示される文字。最大8文字。

topbar-preset-lo = 下限

topbar-preset-hi = 上限

topbar-preset-use-current = 現在値を使用

topbar-preset-use-current-hint = 受信機の現在のフィルターの端を取り込む

topbar-preset-save = 保存

topbar-preset-reset = リセット

topbar-preset-reset-hint = このモードの組み込みプリセットに戻す

topbar-preset-cancel = キャンセル
wefax-schedule = スケジュール
wefax-schedule-hint = { $station } が公開しているスケジュール（図と UTC 時刻）をブラウザーで開きます。

tx-bands-check-again = 再確認

tx-bands-check-again-hint = 確認を取り消してリストを見直します。バンドを変更すると自動的にそうなります。
