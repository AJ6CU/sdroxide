# sdroxide — the interface's English text, the reference every translation
# is checked against. See crates/sdroxide-ui/src/i18n.rs for the rules.
#
# Ids are grouped by where they are shown, and named after it:
# settings-ui-* is the UI tab of the Settings window, and so on. A message
# shown in capitals is written in capitals here; the code does not upper-case
# translated text.

## The language's own name, as the language picker lists it.
language-name = 简体中文

## Values from sdroxide-types, as the screen names them (src/i18n/labels.rs).

layout-mode-auto = 自动
layout-mode-desktop = 桌面
layout-mode-tablet = 平板
layout-mode-small = 小屏幕
layout-mode-phone = 手机

ui-theme-default = 默认
ui-theme-light = 浅色
ui-theme-high-contrast = 高对比度
ui-theme-green-phosphor = 绿色荧光
ui-theme-amber-phosphor = 琥珀色荧光
ui-theme-teal-orange = 青色 / 橙色
ui-theme-rainbow = 彩虹
ui-theme-nord = Nord
ui-theme-nord-dark = Nord 深色
ui-theme-gruvbox = Gruvbox
ui-theme-everforest = Everforest
ui-theme-solarized-dark = Solarized 深色
ui-theme-solarized = Solarized
ui-theme-dracula = Dracula
ui-theme-catppuccin-mocha = Catppuccin 摩卡
ui-theme-catppuccin-latte = Catppuccin 拿铁
ui-theme-modern-minimalist = 现代极简

chrome-style-angled = 斜角
chrome-style-rectangular = 直角
chrome-style-rounded = 圆角
chrome-style-gradient = 渐变
chrome-style-bevel = 3D 浮雕
chrome-style-terminal = 终端

font-size-small = 小
font-size-medium = 中
font-size-large = 大

bandplan-kind-ham = 业余
bandplan-kind-cw = CW
bandplan-kind-digi = 数字
bandplan-kind-phone = 话音
bandplan-kind-beacon = 信标
bandplan-kind-broadcast = 广播
bandplan-kind-am = AM / 长波 / 中波
bandplan-kind-cb = CB

speech-verbosity-terse = 简洁
speech-verbosity-normal = 正常
speech-verbosity-full = 详细

# The examples in brackets are what the (English) voice says.
speech-freq-style-dial = 读表盘（14 点 074）
speech-freq-style-digits = 逐位读出
speech-freq-style-full = 表盘读法带单位

speech-callsign-style-phonetic = 音标（kilo one alpha）
speech-callsign-style-letters = 字母（kay one ay）
speech-callsign-style-as-is = 按原样

## Settings window, UI tab (src/app/settings/ui_tab.rs).

settings-ui-display = 显示
settings-ui-moved-note = 全景图的细节、频谱的反应速度和瀑布图的滚动速度，在显示条上的 SPEC 按钮中设置；窄窗口下在 DISP 菜单中设置。
settings-ui-language = 语言
    .hint = 界面的语言。无论选择哪种语言，F1 手册和语音播报都保持英文。
settings-ui-layout = 布局
settings-ui-theme = 主题
settings-ui-button-style = 按钮样式
settings-ui-frame-rate = 屏幕刷新率
settings-ui-palette = 瀑布图调色板
settings-ui-tuning-buttons = 调谐按钮
settings-ui-tuning-buttons-show = 在手机和平板上显示
    .hint = 触摸屏上控制条下方的一排按钮：降低步进 / 步进 / 增加步进。没有可滚动数字的滚轮，也没有旋钮，因此如果没有这些按钮，要移动一个已知步进，只能把整个频率输入进去。在桌面上从不显示。
settings-ui-tuning-step = 步进：{ $step }
settings-ui-tuning-snap = 首次按下时对齐到步进
    .hint = 表盘停在两个步进倍数之间时，按下的方向决定跳到哪一个。例如以 1 kHz 步进时，7 074 300 向上会到 7 075 000，向下会到 7 074 000。之后按钮照常按步进移动。关闭时，每次按下都只移动一个步进。
settings-ui-smoothing = 瀑布图平滑
settings-ui-smoothing-interpolate = 插值
    .hint = 当显示宽度大于变换的分辨率时，把每个屏幕像素与其周围的频点和行混合，使信号看起来是连续的。若要用矩形瀑布图（每个频点一个方块、每行一个方块）从图像上读取信号形状，则应关闭它：插值后的信号与真正更宽的信号无法区分。更大的 FFT（FFT 芯片）是另一半的关键。
settings-ui-background = 频谱背景
settings-ui-background-gradient = 渐变
settings-ui-background-top = 顶部
settings-ui-background-bottom = 底部
settings-ui-spot-colours = 信号标注颜色
    .hint = 每个信号来源的着色：瀑布图底部的方框、SPOTS 列表中的标签，以及世界地图上的圆点。
settings-ui-spot-colour = { $kind } 信号的颜色
settings-ui-spot-colours-reset = 把所有信号颜色恢复为默认值
settings-ui-reset = 重置
settings-ui-bandplan-colours = 频段计划颜色
    .hint = 瀑布图底部频段计划条上，每类频率分配所使用的底色。这些色块以半透明方式叠加在瀑布图上，因此在瀑布图中看起来比这里的色样更深。
settings-ui-bandplan-colour = { $kind } 分配的颜色
settings-ui-bandplan-colours-reset = 把所有频段计划颜色恢复为默认值
settings-ui-skimmer-font = Skimmer 字号
settings-ui-waterfall-font = 瀑布图 / 频谱字号
settings-ui-interface-font = 界面字号
    .hint = 缩放整个界面：菜单、对话框、窗口、电台选项卡、顶部栏及其按钮。上面两个字号都以此为基准。
settings-ui-cities = 地图上的城市
    .hint =
        在平面地图（FT8/WSPR、APRS、ADS-B 和 AIS）上绘制世界城市：每个地点一个圆点，有空间时在旁边标注名称。

        They are most of what says *where* a dot is on a map of the whole world, and they are also the busiest thing on it. Turn them off if the names are getting in the way of the stations you are reading. The 3D globe is unaffected — its cities are night-side lights rather than markers, and nothing there is written across a contact.
settings-ui-cities-show = 显示城市及其名称
settings-ui-3d-view = 3D 视图
settings-ui-clouds = 云层渲染
settings-ui-clouds-layered = 分层
settings-ui-clouds-volumetric = 体积
settings-ui-clouds-note = 3D 视图中的云层图层如何绘制天气。分层会在对流层中叠加若干切片，开销较小。体积则让光线穿过云层，因此太阳会把云顶的投影投到下方的云层上，雷暴内部的闪电会从中透出光芒，而不仅仅是照亮外壳，代价是每个像素的开销高出数倍。

## Settings window, UI tab: voice announcements.

speech-title = 语音播报
speech-enabled = 播报电台的变化
    .hint = 朗读发生的变化，以便不看电台也能操作
speech-voice = 语音
speech-voice-shipped = 内置语音
speech-speed = 语速
    .hint = 语音会拉伸或压缩自己的节奏，因此音调不会改变。超过约 2 倍后，语速不再加快。
speech-volume = 音量
speech-output = 输出
speech-detail = 详细程度
speech-duck = 压低接收音量
speech-duck-while = 播报期间
speech-test = 测试
speech-what = 播报哪些内容
speech-cat-frequency = 频率
speech-cat-mode-band = 模式和频段
speech-cat-vfo-split = VFO 和异频
speech-cat-agc-gain = AGC 和增益
speech-cat-levels = 驱动、调谐和话筒
speech-cat-ptt = 发射和接收
speech-cat-rit-xit = RIT 和 XIT
speech-cat-memory-scan = 记忆信道和扫描
speech-cat-band-edge = 离开业余频段
speech-cat-notices = 警告和消息
speech-cat-filters = 滤波器、静噪和降噪
    .hint = 默认关闭：追踪信号时它们会频繁变化
speech-decoded = 解码的消息
speech-ft8-to-me = 呼叫我的 FT8 消息
speech-ft8-cq = 我能回应的 FT8 CQ
    .hint = 20 米波段的繁忙时段，每分钟可能有上百条
speech-js8 = 发给我的 JS8 消息
speech-js8-allcall = 也包括 JS8 @ALLCALL
speech-fsq = 发给我的 FSQ 消息
speech-include-snr = 包括信号报告
    .hint = 仅当消息本身不含报告时才加入——已经报告了数值的解码结果，不再额外加上我们的报告
speech-ft8-qso = 我自己的 FT8 通联
    .hint = 序列器即将发送的内容，以及通联结束
speech-reading-text = 朗读解码的文字
speech-text-cw = CW
speech-text-rtty-psk = RTTY、PSK、Olivia、THOR、FSQ
speech-text-cw-locked = 仅当解码器锁定时朗读 CW
    .hint = 朗读未锁定解码器的输出，还不如保持安静
speech-text-note = 两者默认都关闭。解码器生成文字的速度快于语音朗读，因此任何落后于实时音频太多的内容都会被丢弃，而不是排队等待。
speech-tuning-up = 调谐时
speech-swr-while-tuning = 按住 TUNE 时读出驻波比
speech-swr-every = 每隔
speech-swr-summary = 松开时报告最佳匹配
speech-swr-alarm = 任何发射期间出现高驻波比时发出警告
speech-how-read = 读法
speech-frequencies = 频率
speech-callsigns = 呼号
speech-quiet-on-tx = 发射时保持安静
    .hint = 语音从你的扬声器播出，因此会被你的话筒拾取。高驻波比警告仍会播报。
speech-device-note = 播报使用其独立的声音设备，因此永远不会被录制，也不会发送给远程收听者。播报状态、重复上一条播报以及句中停止的按键，设在“控制”选项卡中。

## Settings window (src/app/settings/mod.rs)

settings-waiting-station-s-network-configuration = 等待电台的网络配置……
settings-device-reports = 按设备报告的值
settings-transverters = 变频器
settings-one-row-per-box-while = 每个方框一行。当表盘位于某行的频段内时，电台会调谐到“表盘 + 偏移”，并应用该行的发射规则和驱动上限；在所有行之外，电台使用其自身的频段。各行按从上到下的顺序判断。
settings-name = 名称
settings-band-low = 频段下限
settings-band-high = 频段上限
settings-offset = 偏移
settings-transmit = 发射
settings-max-drive = 最大驱动
settings-off-takes-transverter-out-line = 关闭后，该变频器退出线路但保留此行——频段恢复为电台自身能覆盖的范围。
settings-t-2-m = 2 米
settings-what-you-call-shown-log = 你给它起的名字。表盘选中它时显示在日志中。
settings-bottom-band-transverter-works-dial = 此变频器工作的频段下限，以表盘读数表示。2 米为 144。
settings-top-band-dial-148-2 = 频段上限，以表盘读数表示。2 米在第 2 和第 3 区为 148，第 1 区为 146。
settings-radio-ends-up-dial-offset = 电台最终调谐到“表盘 + 偏移”，因此把频段下变到中频的变频器为负值：2 米变到 28 MHz 中频时为 -116，因为 144 - 116 = 28。对于略有偏差的本振，可拖动以每次微调一赫兹。

    Through the same converter: it works both ways, and transmit takes the same offset.

    Your Drive setting is held under this rather than moved, so the number you use on HF comes back when the dial leaves the band.
settings-remove = 删除
settings-add-transverter = 添加变频器
settings-takes-effect-apply-reconnect-like = 在“应用”或重新连接后生效，与上面的变频器偏移相同。
settings-maximum-transmit-drive = 最大发射驱动
settings-hard-limit-drive-tune-controls = 驱动和 TUNE 控件无法超越的硬性上限，在下方的频段校准之后应用，因此任何设置都不能把驱动推回其上。请将其设在电台的额定功率处，这样整个驱动控件都能有效使用——对于自行调制采样的发射机（HPSDR、LimeSDR、PlutoSDR、HackRF），满驱动就是发射机全开，对于高增益功放而言，远超其末级的额定值。关闭则控件可达满驱动。
settings-limit-drive = 驱动限制为
settings-per-cent-full-drive-key = 满驱动的百分比。将电台接入假负载并按下发射，然后把此值调低，直到功率计读数达到功放的额定功率。
settings-applies-immediately-every-band-tune = 立即生效，适用于每个频段，也包括 TUNE。
settings-transmit-drive-band = 分频段的发射驱动
settings-every-amplifier-has-different-gain = 每台功放在每个频段上的增益都不同，因此同一个驱动设置在各频段产生的功率也不同。请在每个频段上测量输出，然后把输出偏高的频段调低，直到全部一致：以需要最多的频段（通常是最高频段）设定驱动，其余频段则降到与之相同的水平。单位为输出功率的分贝，同时适用于话音、数字和 TUNE。全部为零（默认值）则不做任何改变。
settings-other = 其他
settings-clear-all = 全部清除
settings-back-no-calibration-drive-setting = 恢复为无校准：驱动设置完整地作用于每个频段。
settings-applies-immediately-band-you-would = 立即生效，并作用于你将要发射的频段——通过变频器时，即表盘所在的频段，而变频器自身的驱动上限仍优先于此处的任何设置。
settings-settings = 设置
settings-enter-server-s-address-first = 请先输入服务器地址
settings-enter-receiver-s-address-first = 请先输入接收机地址，或从公共 SDR 中选择一个
settings-no-radio-selected-press-discover = 未选择电台——请按“搜索”，或输入地址
settings-enter-radio-s-address-first = 请先输入电台地址
settings-general = 常规
settings-radio = 电台
settings-ui = 界面
settings-alerts = 提醒
settings-controls = 控制
settings-spots = 信号标注
settings-freedv = FreeDV
settings-uploads = 上传
settings-winlink = Winlink
settings-servers = 服务器
settings-t-r-switch = 收发开关
settings-tle = TLE
settings-profiles = 配置文件
settings-station = 电台站点
settings-enter-digital-mode-ft8-sstv = 输入一次数字模式（FT8 / SSTV / …）即可加载已保存的值。
settings-callsign = 呼号
settings-grid-square = 网格定位
settings-iaru-region = IARU 区域
settings-which-radio-interface-server-uses = 服务器使用哪种电台接口，在其所在的机器上设置。
settings-radio-interface = 电台接口
settings-waiting-machine-radio-attached-interface = 等待电台所连接的机器。其接口设置在下方，在此期间可以在此处修改。
settings-converter = 变频器
settings-manual-type-offset-below = 手动——在下方输入偏移
settings-what-box-reads-when-offset = 当偏移不是预设值之一时，此框显示的内容。请在下方的“偏移”字段中输入来设置；此处无法选择。
settings-frequency-converter-front-receiver-pick = 接收机前端的频率变换器。请选择一个，或在旁边输入偏移。
settings-how-far-converter-moves-signal =
    变频器把信号移向接收机的距离，单位为赫兹——与所有变频器的说明文档和其他 SDR 程序所用的数值及符号相同。电台最终调谐到“表盘 + 偏移”：上变频器为正（Ham It Up 为 125000000），把频段下变到中频的设备为负，例如卫星 LNB，或 28 MHz 中频上的 2 米变频器，即 -116000000（28 − 144）。0 表示无变频器。

    Drag to trim it a hertz at a time, which is what a converter whose oscillator is slightly off wants.

    This is the receive path. What is in the transmit line is the row below.

    Takes effect on Apply.
settings-what-transmit-line-while-converter =
    设置了变频器时，发射线路中的内容。

    Off while converting: nothing is transmitted — the safe default, and right for a receive-only accessory.

    Through the same converter: one box converts both ways, a transverter. Transmit takes the offset above and follows it when it is trimmed.

    Its own offset: the transmit line is different from the receive one. Nought is the common case — a converter (an LNB, a Ham It Up) on receive with the transmitter on its own antenna, which is the QO-100 station.

    The amateur-band check still applies, on the frequency you are transmitting on.

    Takes effect on Apply.
settings-how-far-transmit-line-moves =
    发射线路移动信号的距离，单位为赫兹，符号规则与接收偏移相同：电台以“表盘 + 偏移”发射。

    0 = the transmitter is wired to its own antenna and works on the frequency the dial says — a QO-100 station's 2.4 GHz uplink.

    A transmit converter that takes an I.F. *up* is a negative number: the radio works below the dial.
settings-rx-range = 接收范围
settings-which-frequencies-radio-receives-mhz =
    此电台可接收的频率，单位 MHz：144-146、430-440。留空则使用设备自行报告的范围。

    Band buttons outside the range are greyed out and the dial will not go there.

    These are dial frequencies. With a converter set, state the band you tune — 144-148 for a 2 m transverter — not the I.F. the radio is really on.

    Takes effect on Apply.
settings-tx-range = 发射范围
settings-which-frequencies-radio-transmits-mhz =
    此电台可以发射的频率，单位 MHz：144-146、430-440。留空则使用设备报告的范围——如果设备没有报告，则以驱动程序所言为准，允许任何频率。

    Dial frequencies, like the receive range above.

    This is a limit you set, not a licence: transmitting outside the amateur bands is refused regardless unless you have turned that off in config.toml. Nor does it give a receive-only device a transmitter.

    Takes effect on Apply.
settings-antenna = 天线位于
settings-where-radio-listens-station-antenna =
    此电台从何处收听。

    At the station: the antenna is yours, so your own locator says where it is. The default, and right for everything in the shack.

    Somewhere else: an online receiver, or your own set up on a hilltop. What it hears is reported to PSK Reporter, WSPRnet and FreeDV Reporter from the locator beside this, never from yours — and ADS-B places aircraft against it too.

    Picking a receiver under "Public SDRs" fills this in for you.

    Takes effect on Apply.
settings-receiver-s-maidenhead-locator-jn88ec =
    接收机的梅登海德网格定位——例如 JN88ec、DO30db。四个或六个字符。

    Leave it empty if you do not know where the receiver is: nothing is then reported at all, which is the only honest answer. Reporting from your own square would put somebody else's reception on the wrong continent.

    On top of cal_offset_db in config.toml, which is one figure for the whole station.
settings-antenna-somewhere-else-no-locator = 天线位于别处且未设置网格定位，因此此电台收听到的内容不会报告给 PSK Reporter、WSPRnet 或 FreeDV Reporter。请在上面输入接收机的网格定位，即可以它的位置报告。
settings-ranges-are-mhz-low-high = 范围以 MHz 为单位，写作“低-高”，用逗号分隔：144-146、430-440——即 144000000-146000000 Hz 和 430000000-440000000 Hz。它们是表盘频率：变频器偏移不会改变它们。上面的变频器偏移是以赫兹为单位的字段。留空某个范围则使用设备自行报告的范围；设备若不报告任何内容，则以其所言为准。
settings-careful-rtl-sdr-blog-v4 = 使用 RTL-SDR 时要小心：Blog V4 自己会在 28.8 MHz 以下做上变频，因此若负偏移把硬件调到那里，频率会被移动两次。
settings-pick-radio-interface-above-configuration = 请在上方选择一个电台接口（此配置使用的是已移除的自动检测模式）。
settings-configuration-used-removed-auto-detect = 此配置使用的是已移除的自动检测模式。待电台所连接的机器作出应答后，请在上方选择一个接口。
settings-radio-has-no-interface-yet = 此电台尚无接口——请在上方选择一个，然后按“应用 / 重新连接”。
settings-radio-has-no-interface-yet-2 = 此电台尚无接口。待电台所连接的机器作出应答后，可在上方选择一个。
settings-apply-reconnect = 应用 / 重新连接
settings-switch-interface-now-no-restart = 立即切换到此接口——无需重启
settings-switch-server-s-radio-interface = 立即将服务器的电台切换到此接口——无需重启。若无法打开新接口，则旧接口继续运行。
settings-switches-live-radio-without-restarting = 在不重启的情况下切换正在运行的电台。
settings-everything-above-server-s-own = 以上所有内容都是服务器自身的配置，更改会保存在那里。大多数设置在你更改时即时生效；而在打开设备时才确定的项目——采样率、地址、接口本身——需要使用此按钮。
settings-dx-cluster-telnet = DX 集群（telnet）
settings-enabled = 启用
settings-host = 主机
settings-port = 端口
settings-login-call = 登录呼号
settings-commands = 命令
settings-one-per-line-e-g = 每行一条，例如 SET/FT8
settings-one-per-line-e-g-2 = 每行一条，例如 set/filter cont=eu
settings-sent-after-login-place-narrow = 登录后发送。用于缩小数据范围的地方——若不加过滤器，这就是地球上的每一台 skimmer。
settings-rbn-paths-are-placed-country = RBN 路径是按国家中心而非网格定位标注的——对小国较准确，对大国则偏差很大。它们是传播地图上的一个独立图层，可以在那里关闭。
settings-pota-sota-psk-reporter = POTA / SOTA / PSK Reporter
settings-pota-activator-spots = POTA 活动台信号
settings-sota-spots = SOTA 信号
settings-psk-reporter-current-band = PSK Reporter（当前频段）
settings-upload-my-ft8-ft4-ft2 = 上传我的 FT8/FT4/FT2/JS8 解码结果
settings-antenna-2 = 天线
settings-collector = 收集器
settings-t-4739-live-collector-14739-test = 4739 为正式收集器，14739 为测试收集器
settings-max-age-s = 最长保留时间（秒）
settings-wsprnet = WSPRnet
settings-upload-my-wspr-decodes = 上传我的 WSPR 解码结果
settings-send-every-wspr-reception-wsprnet = 将每一次 WSPR 接收结果发送到 wsprnet.org。默认开启：它不向空中发射任何内容，而报告你所听到的内容，正是使 WSPR 接收机成为网络的一部分、而非私人玩具的原因。没有解码到任何内容的时段也会被报告，网络借此区分频段关闭与接收机已关机。
settings-download-who-heard-me = 下载收听过我的电台
settings-ask-wsprnet-org-which-stations = 询问 wsprnet.org 哪些电台解码了此电台的信号。WSPR 没有任何形式的确认机制，因此这是发射信标了解自身覆盖范围的唯一途径。
settings-ask-every = 每隔
settings-looking-back = 回溯查询
settings-apply = {" "}应用{" "}
settings-persist-re-connect-feeds = 保存并（重新）连接数据源
settings-broadcast-stations = 广播电台
settings-winlink-account = Winlink 账户
settings-password = 密码
settings-winlink-account-password-not-gateway = 是 Winlink 账户的密码，而不是网关的密码。区分大小写——请与签发时完全一致地输入。
settings-locator = 网格定位
settings-how-connect = 连接方式
settings-route = 路由
settings-internet = 互联网
settings-forward-cms-over-internet = 通过互联网经 CMS 转发
settings-radio-packet = 电台（分组）
settings-call-rms-gateway-air-radio = 通过无线电呼叫 RMS 网关。电台必须处于 PACKET 或 PACKET-HF 模式。
settings-gateway = 网关
settings-via = 经由
settings-digipeaters-order-separated-spaces-usually = 数字中继，按顺序排列，以空格分隔。通常留空——你能直接收听到的网关，就应该直接呼叫。
settings-speed = 速率
settings-most-rms-gateways-answer-1200 = 大多数 RMS 网关以 1200 应答。以 9600 呼叫 1200 网关，听起来与网关已离线完全相同，因此请以网关所有者公布的数值为准，而不要猜测。9600 还要求两端的电台都使用数据端口。300 适用于 HF。
settings-frequency = 频率
settings-clear = 清除
settings-zero-leaves-dial-alone-which = 填零则不改动表盘，这正是你停留在某一个信道上时所需要的。其他任何值都会在会话开始时调谐电台。
settings-my-gateways = 我的网关
settings-winlink-s-published-gateway-list = Winlink 公布的网关列表需要 sdroxide 没有的 API 密钥，因此请保留你自己的列表。从某一地点能够连到的两三个网关是通过尝试得知的，它们很少变动。
settings-use-2 = 使用
settings-call-one-next-connect = 下次连接时呼叫此网关
settings-current-dial = 当前表盘
settings-forget = 删除
settings-forget-gateway = 删除此网关
settings-add-gateway = + 添加网关
settings-save-gateway-above-list = 将上面的网关保存到列表中
settings-internet-gateway = 互联网网关
settings-cms-address = CMS 地址
settings-client-name = 客户端名称
settings-winlink-s-production-servers-only = Winlink 的正式服务器只接受其认可的客户端名称，对未知名称会回复“正式服务器不允许未知客户端类型”。在 sdroxide 向 Winlink 开发团队注册之前，连接需要使用他们认可的名称。
settings-automatic-connection = 自动连接
settings-connect-timer = 定时连接
settings-every = 每隔
settings-persist-winlink-account = 保存 Winlink 账户
settings-callsign-lookup = 呼号查询
settings-provider = 服务商
settings-auto-fill-name-qth-grid = 点击信号或通联时自动填入姓名 / QTH / 网格
settings-choose-provider-enter-login = 请选择一个服务商以输入其登录信息。
settings-qrz-user = QRZ 用户名
settings-qrz-pass = QRZ 密码
settings-hamqth-user = HamQTH 用户名
settings-hamqth-pass = HamQTH 密码
settings-one-hamqth-account-does-both = 一个 HamQTH 账户可兼作两用——此处的登录信息即 HamQTH 上传选项卡所使用的那一个。
settings-upload = 上传
settings-auto-upload-each-new-qso = 自动上传每一次新通联
settings-master-switch-which-services-qso = 总开关。通联上传到哪些服务，在下面各个选项卡中设置。
settings-turn-auto-upload-each-new = 请先打开上面的“自动上传每一次新通联”——在此勾选的服务，在此之前不会被推送。
settings-auto-upload-off-above-so = 上面的自动上传已关闭，因此各服务的勾选已被禁用，目前不会自动推送任何内容——日志中每次通联的“上传”按钮仍然可用。
settings-qrz-log-key = QRZ 日志密钥
settings-qrz-logbook-api-key-your = 来自你 QRZ 日志设置的 QRZ 日志 API 密钥——不是 XML 查询的登录信息。
settings-eqsl-user = eQSL 用户名
settings-eqsl-pass = eQSL 密码
settings-eqsl-qth = eQSL QTH
settings-same-login-hamqth-callsign-lookup = 与 HamQTH 呼号查询相同的登录信息——填写其中任何一个，两者都会填上。
settings-club-log-email = Club Log 邮箱
settings-club-log-pass = Club Log 密码
settings-club-log-key = Club Log 密钥
settings-wrl-api-key = WRL API 密钥
settings-developer-api-key-world-radio = 来自 World Radio League → Integrations → Developer API 的开发者 API 密钥。它在生成时只显示一次，请当场复制。通联会进入你的默认日志——如果你保留了多个日志，请在 WRL 中设定一个默认值。
settings-confirmations-download = 确认信息（下载）
settings-lotw-user = LoTW 用户名
settings-lotw-pass = LoTW 密码
settings-lotw-upload-uses-tqsl-export = LoTW 上传使用 TQSL——从日志导出 ADIF 并签名。此处下载 LoTW/eQSL 的确认信息，用于标记已通联与已确认。
settings-sync-confirmations = 同步确认信息
settings-fetching-subscriptions = 正在获取订阅……

    To use it on its own again, clear the Panadapter receiver box on that radio's page.
settings-panadapter = 全景图
settings-give-radio-wideband-panadapter-borrowing = 通过借用另一台电台的接收机，为此电台提供宽带全景图——例如同一天线上的 SDR，或由此电台的中频输出馈送的接收机。频谱、瀑布图、副接收机、数字模式和 skimmer 都来自该接收机；表盘、模式、滤波器和发射机仍保留在此。
settings-receiver = 接收机
settings-none = 无

    Takes effect on Apply.
settings-connected = 连接至

    This is not the Converter field above, which retunes the radio, and not the CAT tab's I/Q centre offset, which is about this radio's own sound card. Nothing typed here is ever sent to either radio.

    Takes effect on Apply.
settings-audio-2 = 音频来自
settings-follow-dial = 跟随表盘
settings-keep-receiver-radio-s-dial = 让接收机跟随此电台的表盘，并在全景图上调谐时移动表盘。关闭时接收机停在原处——电台去了别处，而接收机固定监视某一段。
settings-invert-spectrum = 反转频谱
settings-mirror-receiver-s-span-about = 以接收机的中心为轴镜像其跨度，适用于本振位于信号上方、把频段倒着送过来的抽头。除非瀑布图上充满了全都位于表盘另一侧的信号，否则请保持关闭。
settings-mute-transmit = 发射时静音
settings-silence-receive-audio-while-radio = 此电台发射时，静音接收音频。默认开启：接收机与发射机共用同一天线，或接在此电台的中频上时，它在通话期间听到的就是你自己的发射机。在电台本身按下的通话——话筒的 PTT、脚踏开关、MOX——同样计入，只要 CAT 连接报告电台已转为发射。
settings-blank-transmit = 发射时清空显示
settings-stop-panadapter-waterfall-while-radio = 此电台发射时，停止全景图和瀑布图。默认开启：发射机会把整个跨度都涂满，从而抹掉其背后的频段。关闭后，你可以在通话期间观察频段——或你自己的信号——这在接收机使用独立天线时很有价值。仅适用于由 sdroxide 发射的通话；在电台上按下发射的通话会被完整接收，因此无论此项如何设置，画面都会持续刷新。
settings-per-mode-offsets-radio-whose = 按模式设置偏移，适用于中频随模式变化的电台。留空的模式使用上面的偏移。
settings-computer = 本计算机上
settings-add-radio-computer = 在本计算机上添加电台
settings-radio-s-settings-are-below = 此电台的设置在下方。拖动可沿条带移动它。
settings-switch-radio-dialog-follows-drag = 切换到此电台（对话框随之切换）。拖动可沿条带移动它。
settings-tx = ● 发射
settings-mute-radio-s-audio = 静音此电台的音频
settings-close-radio-configuration-kept = 关闭此电台（保留其配置）
settings-close-tab-close-radio-station = 关闭此选项卡，或在其所在的电台站点上关闭电台
settings-close-tab = 关闭此选项卡
settings-transverters-full = { $max } 是此表最多可容纳的数量
settings-dialling = 正在拨号 { $url }……
settings-region-note = 电台所在的区域。它决定所有频段计划：频段边界、瀑布图条带上的 CW / 数据 / 话音子频段、skimmer 的收听频率、提供的呼叫频率，以及发射时哪些算作频段外。在第 { $region } 区，这意味着 70 厘米 { $region ->
        [1] 为 430–440 MHz——因此 446 属于频段外，40 米止于 7.200
        [2] 为 420–450 MHz，40 米延伸至 7.300
       *[3] 为 430–450 MHz，80 米止于 3.900
    }。立即生效，并适用于此电台站点上的每一台电台。
settings-rx-grid-reported = 经由此电台的接收结果，以 { $grid } 为位置报告，而不是以你自己的网格定位报告。
settings-rx-grid-invalid = { $grid } 不是梅登海德网格定位，因此此电台收听到的内容不会被报告。应为四个或六个字符：两个字母、两个数字，以及可选的另外两个字母。
settings-gateway-via = { " " }经由 { $via }
settings-auto-upload-to = 将每一次新通联自动上传到 { $service }
settings-tle-updated = 已更新 { $n } 个订阅：共 { $total } 颗卫星。
settings-tle-updated-failed = 已更新 { $ok } / { $n }；{ $failed } 个失败——请见上方各行。
settings-panadapter-of = 此电台是 { $radio } 的全景图
settings-radio-n = 电台 { $n }
settings-on-station = 位于 { $station }
settings-add-radio-on-station = 在 { $station } 上添加电台——电台站点会立即为其提供服务
settings-panadapter-receiver-for = { $radio } 的全景接收机
settings-hang-up-stays-on = 挂断。电台仍保留在 { $station } 上。
settings-close-on-station = 要在 { $station } 上关闭 { $radio } 吗？其配置仍保留在该机器上；此处的选项卡随之关闭，其他人的也一样。
settings-close-it-on = 在 { $station } 上关闭它
settings-locator-hint = 网格定位

## Settings window, Alerts (src/app/settings/alerts.rs)

settings-alerts-audible-alerts = 声音提醒
settings-alerts-sound-alarm-when-decode-matters = 当解码结果重要时发出警报声
settings-alerts-plays-over-own-audio-output = 通过其独立的音频输出播放，因此即使频段所用的扬声器与此屏幕不同，也能听到。
settings-alerts-volume = 音量
settings-alerts-output = 输出
settings-alerts-test = 测试
settings-alerts-what-sound = 发出哪些声音
settings-alerts-each-station-quiet-while-after = 每个电台在一次提醒之后会安静一段时间，因此繁忙的频段不会在每个时隙都响起。“语音”和“音调 + 语音”由语音播报朗读——在“设置 → UI”中开启它。

## Settings window, Amplifier (src/app/settings/amp.rs)

settings-amp-amplifier = 功放
settings-amp-waiting-station-s-amplifier-configuration = 正在等待电台站点的功放配置……
settings-amp-hobbypcb-hardrock-500-hardrock-50 = 通过其 USB 串口连接的 HobbyPCB Hardrock-500 或 Hardrock-50。SDRoxide 在 AMP 窗口中读取其仪表、切换其频段、模式和天线，并操作其调谐器。功放仍像以前一样由电台的 PTT 线路控制。
settings-amp-enable = 启用
settings-amp-model = 型号
settings-amp-untested-panel-has-only-been = 未经测试：此面板仅在 Hardrock-500 上测试过。此型号的表头限值由 Hardrock-500 的数值换算而来，可能有误。请留意功放自身的显示。
settings-amp-port = 端口
settings-amp-choose = — 请选择 —
settings-amp-baud = 波特率
settings-amp-must-match-amplifier-s-menu = 必须与功放的菜单设置一致。HobbyPCB 自己的控制面板需要 115200。
settings-amp-follow-radio-s-band = 跟随电台的频段
settings-amp-sends-transmit-frequency-amplifier-every = 每次频段变化时，都会把发射频率发送给功放，使其滤波器跟随电台，而无需单独的 CAT 线缆。发射期间从不发送。
settings-amp-show-power-swr-tx-meter = 在发射表上显示其功率和驻波比
settings-amp-only-where-radio-does-not = 仅在电台自身不测量它们时显示，并且仅在功放接入线路时显示。
settings-amp-off = 关闭
settings-amp-connecting = 正在连接……
settings-amp-status = 状态
settings-amp-apply = {" "}应用{" "}
settings-amp-save-re-open-amplifier-s = 保存并（重新）打开功放的端口

## Settings window, Controls (src/app/settings/controls.rs)

settings-controls-keyboard = 键盘
settings-controls-click-shortcut-rebind-then-press = 点击某个快捷键即可重新绑定，然后按下组合键（Esc 取消）。在文本框中输入时，快捷键不生效。
settings-controls-shortcut = 快捷键
settings-controls-does = 功能
settings-controls-step-mode = 步进 / 模式
settings-controls-accel = 加速
settings-controls-on = 开
settings-controls-press-key = 按下一个键……
settings-controls-remove-binding = 删除此绑定
settings-controls-add-shortcut = + 添加快捷键
settings-controls-restore-defaults = 恢复默认值
settings-controls-bind-hold-talk-space = 将按住通话绑定到空格键
settings-controls-hold-space-transmit-releasing-losing = 按住空格键发射；松开它——或窗口失去焦点——即停止发射
settings-controls-unkey-held-ptt-after = 在以下时间后自动松开按住的 PTT
settings-controls-backstop-against-stuck-key-controller = 防止按键卡住或控制器停止上报的后备机制。0 表示禁用。
settings-controls-panadapter-mouse = 全景图鼠标
settings-controls-wheel = 滚轮
settings-controls-wheel-shift = 滚轮 + Shift
settings-controls-tune-step = 调谐步进
settings-controls-zoom-rate = 缩放速率
settings-controls-click-tune-rounding = 点击调谐取整
settings-controls-invert-wheel-direction = 反转滚轮方向
settings-controls-left-drag-tunes-well-pans = 左键拖动同时调谐和平移
settings-controls-off-makes-left-drag-pan = 关闭后，左键拖动仅平移视图，与右键拖动相同。
settings-controls-scroll-digit-frequency-readout-tune = 滚动频率读数上的数字即可调谐
settings-controls-both-wheel-actions-are-tune = 两种滚轮操作都是调谐——此时已没有缩放的方式。
settings-controls-restore-mouse-defaults = 恢复鼠标默认值
settings-controls-mouse-buttons = 鼠标按键
settings-controls-left-right-buttons-are-reserved = 左键和右键保留用于调谐和平移；中键和额外按键可自由设置。为 PTT 按住的侧键，其作用与脚踏开关相同。
settings-controls-add-mouse-button = + 添加鼠标按键
settings-controls-f1-always-opens-manual-even = F1 始终打开此手册，即使在输入文字时也是如此，因此无法重新绑定。
settings-controls-midi-controller = MIDI 控制器
settings-controls-midi-controllers-need-native-app = MIDI 控制器需要原生应用——浏览器客户端无法访问 MIDI。
settings-controls-any-class-compliant-midi-surface = 任何符合 class 标准的 MIDI 设备都可以使用：DJ 控制器的转轮可以做很好的 VFO 旋钮，其按键可做 PTT 和频段按钮，其推子可做增益控制。
settings-controls-enable = 启用
settings-controls-controller = 控制器
settings-controls-feedback = 反馈至
settings-controls-rescan-ports = 重新扫描端口
settings-controls-not-connected = 未连接。
settings-controls-move-control-see-here = 移动一个控件即可在此看到它。
settings-controls-control = 控件
settings-controls-reads = 读取为
settings-controls-led = LED
settings-controls-move = 移动它……
settings-controls-click-then-move-control-you = 点击，然后移动你想要绑定的控件
settings-controls-speed-sensitivity-spin-faster-tune = 速度灵敏度：转得越快，调谐越快
settings-controls-send-current-value-back-light = 将当前值回传，用于点亮 LED 或移动推子
settings-controls-add-midi-control = + 添加 MIDI 控件
settings-controls-clear-all = 全部清除
settings-controls-endless-jog-encoders-send-relative = 无限旋转（转轮）编码器发送的是相对步进而非位置，有三种编码方式，在小幅移动时看起来很相似。“学习”会根据顺时针旋转进行推测；如果旋钮随后调谐的方向相反，请勾选“反向”。
settings-controls-none = — 无 —

## Settings window, General (src/app/settings/general.rs)

settings-general-system-default = 系统默认
settings-general-remote-access = 远程访问
settings-general-what-remote-client-browser-page = 远程客户端（浏览器页面，或以 --connect 启动的另一个 sdroxide）在此电台站点允许其操作之前，必须提供的凭据。在服务器模式（--server）下生效；下次登录时即会应用更改，无需重启。
settings-general-username = 用户名
settings-general-password = 密码
settings-general-clients-must-give-password-leaving = 客户端必须提供密码。用户名留空也可以。
settings-general-clients-must-sign = 客户端必须登录。
settings-general-empty-anyone-who-can-reach = ⚠ 留空：任何能访问服务器端口的人都可以以你的呼号操作此电台。转发端口之前，请先设置密码。
settings-general-stored-clear-config-toml-like = 以明文保存在 config.toml 中，与 sdroxide 保存的其他密码相同。
settings-general-receive-audio-gain = 接收音频增益
settings-general-t-0-db = 0 dB
settings-general-extra-gain-everything-radio-sends =
    在音量控制之上，对此电台发送到扬声器的所有内容额外增益。除非电台在满音量下仍然很小，否则请保持 0 dB：音量滑块的顶端是原始到达的音频，因此它只能把电台调小，而不能调大；而且某些收发信机的 USB 声音输出远低于满量程。

    Go up 6 dB at a time. Too much clips — the audio is limited at full scale rather than allowed to wrap round, so overdoing it sounds harsh rather than loud. Recordings are taken ahead of this and are not affected.
settings-general-reload-band-plan = 重新加载频段计划
settings-general-re-read-bandplan-json-machine = 在电台所连接的机器上重新读取 bandplan.json 并应用它——频段边界、子频段和 skimmer 窗口——无需重启。
settings-general-running-station-s-own-band = 正在使用电台站点自己的频段计划。
settings-general-running-built-iaru-tables = 正在使用内置的 IARU 表。
settings-general-settings-file = 设置文件
settings-general-settings-are-machine-radio-attached = 设置保存在电台所连接的机器上。请在那里导出。
settings-general-export = 导出……
settings-general-write-every-setting-station-radios = 将此电台站点的所有设置——电台、模式、服务器、记忆信道、频段计划——写入一个文件，可以带到另一套安装中使用。你的日志和任何已保存的服务器密码仍保留在此。
settings-general-import = 导入……
settings-general-replace-station-s-settings-ones = 用从另一套安装导出的文件中的设置，替换此电台站点的设置。之后请重启 sdroxide。
settings-general-import-overwrites-what-here-file = 导入会按文件逐个覆盖此处的内容，并在 sdroxide 下次启动时生效——否则内存中的现有设置会把它再原样写回。文件中未提及的内容保持不变，因此来自单电台站点的配置包不会删除此处的第二台电台。你的日志永远不在该文件中：如果也要迁移日志，请在日志窗口中以 ADIF 格式导出。
settings-general-sdroxide-settings = sdroxide 设置
settings-general-swr-guard = 驻波比保护
settings-general-stop-transmitting-high-swr = 驻波比过高时停止发射
settings-general-trip = 触发阈值
settings-general-stops-transmission-when-radio-reports =
    当电台报告的驻波比达到或超过此数值时，停止发射，并在你确认之前锁定发射功能。可捕获天线断开、馈线故障或开关误接到错误端口等情况。

    Tuning is treated differently, because feeding a mismatch is the point of it: an antenna tuner gets double the limit and about five seconds before the guard applies at all. A manual tuner that takes longer than that wants the guard switched off for the session.

    Needs a rig that reports SWR over CAT. Ignores the first fifth of a second of each transmission, and does not wait for high power.
settings-general-per-mode-settings = 按模式设置
settings-general-reset-every-mode = 重置所有模式
settings-general-forget-every-mode-s-remembered = 清除每种模式记住的 AGC、静噪、降噪、陷波和立体声开关，并把每种模式自己的默认值恢复回来。
settings-general-selecting-mode-lays-mode-s = 选择某种模式时，接收机会应用该模式自己的 AGC、静噪、降噪、陷波和立体声开关的起始值；更改其中一项，则只为该模式记住。接收机滤波器/降噪行末尾的圆形箭头按钮会列出已更改的项目，并把当前模式恢复为默认值。
settings-general-channels = 声道

## Settings window, network services (src/app/settings/net.rs)

settings-net-operator = 操作员
settings-net-callsign-grid-are-set-general = 呼号和网格定位在“常规”选项卡中设置。
settings-net-set-your-callsign-grid-general = ⚠ 请在“常规”选项卡中设置你的呼号和网格定位。
settings-net-freedv-reporter = FreeDV Reporter
settings-net-enable = 启用
settings-net-connects-whenever-enabled-your-station = 启用时随时连接。只有当电台处于 RADE 模式时，你的电台才会显示给其他人；在其他任何模式下，你保持连接但处于隐藏状态。
settings-net-station = 电台
settings-net-message = 消息
settings-net-receive-only-i-cannot-transmit = 仅接收（我无法发射）
settings-net-server = 服务器
settings-net-host = 主机
settings-net-port = 端口
settings-net-tls-wss = TLS（wss://）
settings-net-not-yet-implemented-freedv-gui = 尚未实现——FreeDV 图形界面也使用普通的 ws://。
settings-net-reporting = 报告
settings-net-report-stations-i-decode = 报告我解码的电台
settings-net-sends-rx-report-each-callsign = 对从 RADE 帧尾（End-of-Over）中恢复的每个呼号，发送一条 rx_report。
settings-net-show-other-reporter-stations-spots = 将其他报告站显示为信号
settings-net-adds-them-panadapter-overlay-world = 在 FREEDV 过滤器下，把它们加入全景图叠加、世界地图和 SPOTS 窗口。
settings-net-set-your-callsign-grid-general-2 = ⚠ 请在“常规”选项卡中设置你的呼号和网格定位。两者缺一，连接就只能查看：你能看到其他电台，但自己不会出现在列表中。
settings-net-apply = {" "}应用{" "}
settings-net-persist-re-connect = 保存并（重新）连接
settings-net-season = 季节
settings-net-downloading = 正在下载……
settings-net-using-built-copy = 正在使用内置副本
settings-net-your-own-stations-corrections-go = 你自己的电台和修正内容写在此文件中。sdroxide 从不写入它，此处的条目会替换同名且同频率的预定条目。
settings-net-reload = 重新加载
settings-net-re-read-your-station-file = 编辑后重新读取你的电台文件
settings-net-broadcast-stations-labelled-waterfall-come = 瀑布图上标注的广播电台来自本版本内置的列表。编辑它们需要使用桌面应用。
settings-net-ask-service-whether-these-credentials = 询问服务这些凭据是否有效。会先应用上面的设置，并且不记录任何内容：它只读取。
settings-net-checking = 正在检查……

## Settings window: messages with values filled in.

settings-amp-connected-to = 已连接到 { $model }
settings-controls-last-midi = 最后一条消息：{ $message }  数值 { $value }
settings-general-band-plan-file = 每个频段边界和子频段都来自 { $path }。该文件首次运行时由内置的 IARU 表写入，之后即归你编辑——把某个频段缩窄到你的执照范围，sdroxide 就会拒绝在其之外发射。频率以 MHz 为单位；删除该文件可重新获得默认值的副本。这是区域分配，而不是你的执照：你自己的条件可能更窄，而且同一区域内的各国计划也有所不同。
settings-general-exported = 已导出 { $note }——请选择保存位置。
settings-general-export-failed = 导出失败：{ $error }
settings-general-import-failed = 导入失败：{ $error }
settings-general-imported = { $summary }——请重启 sdroxide 以使用它们。
settings-general-import-skipped = 已跳过 { $path }：{ $why }
settings-general-swr-tripped = ⚠ 当前已在 { $swr }:1 时触发——发射已被锁定。
settings-net-call-grid-set-on-general = { $call } / { $grid } ——在“常规”选项卡中设置
settings-net-reporting-as = 以 { $call } / { $grid } 身份报告——SDRoxide { $version }
settings-net-schedule-download-failed = ⚠ 下载失败：{ $error }。正在使用内置时刻表。
settings-net-test-service = 测试 { $service }

## Settings window (src/app/settings/mod.rs), continued

settings-transmit-off-while-converter-set = 设置了变频器时发射已关闭——请说明上面发射线路中的内容，以重新开启发射。
settings-transmit-goes-through-same-converter = 发射经过同一变频器，因此电台在两个方向上都以距表盘相同的偏移工作。
settings-transmit-not-converted-radio-transmits = 发射未经转换：电台在表盘所示频率上发射，而接收则经过变频器。
settings-transmit-takes-own-offset-separate = 发射使用其自己的偏移，与接收偏移分开。
settings-no-enabled-subscriptions-update = 没有可更新的已启用订阅。
settings-add-radio-here-station-you = 添加电台——在此处，或在你已连接的某个电台站点上

## Settings window, General, continued

settings-general-every-band-edge-sub-segment = 每个频段边界和子频段都来自电台所连接机器上的 bandplan.json。这是区域分配，而不是你的执照：你自己的条件可能更窄，而且同一区域内的各国计划也有所不同。

## Settings window, picture upload (src/app/settings/picup.rs)

settings-picup-received-pictures-ftp = 接收的图片 → FTP
settings-picup-uploads-every-received-sstv-picture = 在每张接收到的 SSTV 图片保存时将其上传（如勾选，也包括每张天气图），布局与 QSSTV 相同，因此为 QSSTV 编写的网页无需修改即可使用。
settings-picup-upload-received-pictures = 上传接收的图片
settings-picup-server = 服务器
settings-picup-port = 端口
settings-picup-security = 安全性
settings-picup-user = 用户
settings-picup-password = 密码
settings-picup-certificate = 证书
settings-picup-trusted-when-computer-s-operating = 当本计算机的操作系统信任它时即被信任，与此处的浏览器相同。可在下方添加私有 CA（PEM）。
settings-picup-extra-ca = 额外 CA
settings-picup-only-certificate-fingerprint-accepted-way = 只接受具有此指纹的证书——这是信任自签名服务器的方式。“测试”会显示服务器的指纹。
settings-picup-any-certificate-accepted-link-encrypted = ⚠ 接受任何证书。连接是加密的，但路径上的任何人都可以冒充服务器并读取密码。
settings-picup-plain-ftp-sends-password-pictures = ⚠ 普通 FTP 以未加密方式发送密码和图片。
settings-picup-sstv = SSTV
settings-picup-sstv-dir = SSTV 目录
settings-picup-weather-fax = 天气传真
settings-picup-wefax-dir = WEFAX 目录
settings-picup-empty-login-directory = 留空则为登录目录。
settings-picup-file-names = 文件名
settings-picup-format = 格式
settings-picup-add-extension = 添加扩展名
settings-picup-image1-png-rather-than-qsstv = 使用 image1.png，而不是 QSSTV 的无扩展名 image1
settings-picup-caption-bar = 标题栏
settings-picup-time-frequency-mode-sender-s = 顶部显示时间、频率、模式以及发送方的 FSK ID，与 QSSTV 的绘制方式相同。仅已上传的副本带有标记。
settings-picup-fit-960-768 = 适配为 960 × 768
settings-picup-qsstv-s-heartbeat-tiny-picture = QSSTV 的心跳：一张小图片，其高度表示当前分钟，每张图片旁边都会上传一张，程序关闭时宽度变为两个像素。
settings-picup-nat-workaround = NAT 变通
settings-picup-ignore-private-address-server-s = 忽略服务器被动模式回复中的私有地址，改用服务器自己的地址——适用于位于 NAT 之后、不知道自己公网地址的服务器。
settings-picup-apply = {" "}应用{" "}
settings-picup-test = 测试
settings-picup-connect-settings-above-applied-not = 使用上面的设置连接（无论是否已应用）、登录，并在每个上传目录中写入和删除一个小文件。
settings-picup-checking = 正在检查……
settings-picup-trust-certificate = 信任此证书
settings-picup-pin-switch-pinned-fingerprint-one = 固定它：将开关切换为“固定指纹”并填入此指纹。请先与服务器核对——这是服务器所报告的内容，并不能证明其身份。然后点击“应用”。
settings-picup-last-upload = 最近一次上传

## Settings window, Profiles (src/app/settings/profiles.rs)

settings-profiles-profiles-station-s-saved-setups = 配置文件：电台站点保存的设置
settings-profiles-profile-captures-whole-working-setup = 配置文件会记录整套工作设置——表盘和 VFO、模式和滤波器、增益、驱动和天线、你的呼号和消息模板，以及分频段的寄存器——并可一键恢复。硬件（后端、音频设备、变频器）则被有意排除在外：配置文件记录的是你操作电台的方式，而不是电台本身。
settings-profiles-save-current-setup = 保存当前设置
settings-profiles-profile-name = 配置文件名称
settings-profiles-save = 保存
settings-profiles-applies-whatever-sdroxide-attached-dials = 适用于 sdroxide 所连接的任何电台：表盘和 VFO、模式和滤波器、电平、增益、驱动、天线、数字身份以及频段堆栈。
settings-profiles-no-profiles-saved-yet-name = 尚未保存任何配置文件——请在上面命名一个，然后点击“保存”。
settings-profiles-saved-profiles = 已保存的配置文件
settings-profiles-apply = 应用

## Settings window, T/R switch (src/app/settings/relay.rs)

settings-relay-transmit-receive-switch = 收发开关
settings-relay-waiting-station-s-t-r = 正在等待电台站点的收发开关配置……
settings-relay-closes-contact-while-station-transmits = 在此电台站点发射期间闭合一个触点——用于断开并接地 SDR 的天线输入，并与之按顺序驱动功放或外置收发继电器。适用于廉价的 USB 继电器板（LCUS、KMtronic、Numato），通过串口 RTS/DTR 线路可用于任何需要 PTT 闭合的接口，也可通过命令钩子用于其他任何设备。
settings-relay-hardware = 硬件
settings-relay-board = 板卡
settings-relay-lcus-four-byte-a0-command = LCUS 是以十几个名称出售的四字节 A0 命令板；KMtronic 和 Numato 也可以查询其当前设置
settings-relay-port = 端口
settings-relay-choose = — 请选择 —
settings-relay-device = 设备
settings-relay-gpio-pin = GPIO 引脚
settings-relay-pin-3-every-homebrew-plan = 第 3 脚见于所有自制方案和所有商用板卡——它位于封装的末端，便于焊接导线。如果继电器没有咔哒声，说明该卡是不支持这些引脚的仿制品。
settings-relay-gpio-chip = GPIO 芯片
settings-relay-linux-only-gpiodetect-lists-chips = 仅限 Linux。`gpiodetect` 可列出本机上的芯片。
settings-relay-transmit = 发射时
settings-relay-receive = 接收时
settings-relay-nothing-switched = 未切换任何内容。
settings-relay-starting-program-costs-unpredictable-amount = 启动一个程序所需的时间不可预测，而且这段时间都不计入提前量——因此在此处应比对继电器板预留更宽的余量。触点表中的数值对此链路没有意义：你的命令会在按下时运行一次，松开时再运行一次。
settings-relay-contacts = 触点
settings-relay-contact-1-rts-contact-2 = 触点 1 为 RTS，触点 2 为 DTR。
settings-relay-no = 编号
settings-relay-gpio = GPIO
settings-relay-name = 名称
settings-relay-job = 功能
settings-relay-tx-closes = 发射时闭合
settings-relay-lead = 提前
settings-relay-hold = 保持
settings-relay-line-s-offset-chip-bcm = 此芯片上的线路偏移——在树莓派针脚排针上即为 BCM 编号，而非物理针脚号
settings-relay-sdr-antenna = SDR 天线
settings-relay-decides-default-timings-what-log = 决定默认时序，以及日志如何称呼它——对于波段解码器，还决定该触点是否跟随下方的频段表
settings-relay-transmitting-energises-coil-off-transmitting =
    开：发射时线圈通电。关：发射时线圈释放。

    This is a wiring decision before it is a setting — see the note below.
settings-relay-how-long-before-rf-contact = 在射频信号出现之前多久，此触点从其频段的接收字切换到发射字。重新调谐时会立即切换；此项用于按下发射的时刻，因为仅发射滤波器必须在功率到达之前接入电路。
settings-relay-closed-long-before-rf-small = 在射频之前闭合的时长。小型同轴继电器需要 5 至 15 毫秒吸合。发射会等待其中最长的时间，因此数值较大会在接收音频中产生可听见的间隙；超过 250 毫秒的数值将被忽略。
settings-relay-how-long-after-rf-stops = 射频停止之后多久，此触点从其发射字切回接收字。
settings-relay-opened-long-after-rf-stops = 射频停止之后的这段时间后断开。有意比提前量更长：天线稍晚切回没有代价，而过早切回则会损坏前端。零是一个有效的答案，也是功放按键线路的正确设置。
settings-relay-test = {" "}测试{" "}
settings-relay-not-while-station-air = 电台在空中时不可用
settings-relay-close-contact-half-second-so = 将此触点闭合半秒，以便你听到继电器动作，并在发射机断电的情况下检查接线
settings-relay-remove = 删除
settings-relay-add-contact = + 添加触点
settings-relay-if-will-not-answer = 如果它不响应
settings-relay-switch-exists-protect-receiver-not = 一个用来保护接收机的开关，如果它悄悄停止了保护，就没有存在的价值——因此默认是拒绝。若一根松动的 USB 线缆结束了你的比赛，那属于更糟的结果，则可以选择另一项。
settings-relay-order-timings-longest-lead-closes = 顺序由时序决定：提前量最长的先闭合，保持时间最长的最后断开。因此天线继电器应设为较长的提前量和较长的保持时间，而功放则各取较短的值——这样天线总是先于功放按键被切换，而功放总是先于天线复位而解除按键。
settings-relay-transmit-sense-input = 发射检测输入
settings-relay-when-transceiver-keys-itself-own = 当收发信机自行按下发射时——例如其自带的话筒按钮、脚踏开关、VOX 或键控器——sdroxide 会通过 CAT 询问得知，而该回答要在通话开始后几百毫秒才到达。若将电台的 SEND 线路经光耦接入同一端口的握手输入，则几毫秒内即可察觉。这样一来，表头和发射互锁都会更快。
settings-relay-line = 线路
settings-relay-transmitting = 发射时为
settings-relay-high-line = 高电平
settings-relay-most-opto-isolated-interfaces-pull = 大多数光隔离接口会在电台按键时把线路拉低，因此通常应关闭此项
settings-relay-belongs-radio = 所属电台
settings-relay-which-radio-tab-sensed-transceiver = 被检测的收发信机属于哪个电台选项卡。在只有一台电台的站点上，此项为 1，且不影响任何内容。
settings-relay-needs-serial-port-handshake-lines = 需要串口——握手线路位于其上。
settings-relay-wiring-only-fail-safe-survives = 只有接线才是能够在本程序失效后仍然有效的故障保护。
settings-relay-choose-tx-closes-so-de = 选择“发射时闭合”，使得*失电*的触点恰是你所需的状态：当板卡被拔掉、计算机关机或 sdroxide 未运行时。若接线使失电即接地，则继电器失效时接收机会失聪——令人烦恼，但安全。若接线方式相反，继电器失效加上一次发射，前端就会损坏。若其后接的是真正的功放，请选择前一种。
settings-relay-cannot-protect-first-instant-over = 对于你在电台上开始的通话，它无法保护最初的瞬间。若从 sdroxide 按下发射，触点总是先于射频动作；若接入上面的检测输入，它们则会在几毫秒内跟随。若前端确实值得保护，还应另外使用由射频检测的硬件收发开关——计算机上的任何程序都不能单独解决全部问题。
settings-relay-contacts-transmit = 发射时的触点
settings-relay-not-open = 未断开。
settings-relay-status-unknown-press-apply = 状态未知——请按“应用”。
settings-relay-station-air-apply-will-take = 电台在空中——“应用”将在本次通话结束后生效。
settings-relay-band-decoder-outputs = 频段解码器输出
settings-relay-one-control-word-per-band = 每个频段一个控制字：位 0 为触点 1，位 1 为触点 2，依此类推，与上面的编号一致。接收跟随接收表盘所在的频段；发射则跟随按下发射的那台电台的频段——在射频之前经每个触点的提前量接入，并在保持时间之后切回。对于不关心射频方向的滤波器，两者设为相同的值；对于仅接收前置放大器的旁路或仅发射低通滤波器组，则设为不同的值。保持为 0 的频段不输出任何信号。在“应用 / 重新连接”后生效。
settings-relay-band = 频段
settings-relay-rx = 接收
settings-relay-tx = 发射
settings-relay-outputs-asserted = 已输出的信号
settings-relay-other = 其他
settings-relay-everywhere-outside-amateur-bands-short = 业余频段之外的所有位置——短波收听，以及变频器表盘落到任何频段都未覆盖之处。
settings-relay-apply = {" "}应用{" "}
settings-relay-persist-re-open-switch = 保存并（重新）打开开关

## Settings window, Remote (src/app/settings/remote.rs)

settings-remote-connect-server = 连接服务器
settings-remote-drive-sdroxide-running-somewhere-else = 通过网络操作在别处运行的 sdroxide——机房计算机、远程站点。请在那里以 --server 启动它，并在此输入其地址。电台、解码器和日志仍保留在该机器上；此屏幕获取瀑布图和音频，并使用你在此处的扬声器和话筒。
settings-remote-address = 地址
settings-remote-host-name-ip = 主机名或 IP
settings-remote-machine-running-sdroxide-server-host = 运行 sdroxide --server 的机器：可以是主机名、IPv4 或 IPv6 地址，或者在服务器位于反向代理之后时使用完整的 ws://（或 wss://）URL——URL 会按原样使用，忽略下面的端口和开关。
settings-remote-port = 端口
settings-remote-port-server-listens-server-port = 该服务器监听的端口——即其 config.toml 中的 server_port。除非指定了 --port，否则为 4950。
settings-remote-secure = 安全
settings-remote-wss-tls = wss://（TLS）
settings-remote-tick-where-server-sits-behind = 当服务器位于一个为其终止 HTTPS 的组件之后时勾选此项——例如端口 443 上的反向代理，这是通过公共互联网访问电台的常见方式。sdroxide 自身的服务器使用普通的 ws://，不需要勾选此项。
settings-remote-connect = {" "}连接{" "}
settings-remote-open-station-radio-tab-own = 将此电台站点打开为其自己的电台选项卡
settings-remote-client-cannot-hold-second-connection = 此客户端无法保持第二个连接。

    If the server asks for a username and password, its sign-in screen comes up in the new tab. A plain ws:// link carries none of this encrypted, so across the open internet either put it through a VPN or an SSH tunnel, or terminate HTTPS in front of the server and tick Secure above.

## Settings window, Servers (src/app/settings/servers.rs)

settings-servers-wsjt-x-udp-broadcast = WSJT-X UDP 广播
settings-servers-waiting-station-s-wsjt-x = 正在等待电台站点的 WSJT-X 广播配置……
settings-servers-sends-decodes-station-status-logged = 以 WSJT-X 的方式发送解码结果、电台状态和已记录的通联，因此 GridTracker、JTAlert、N1MM+ 和 Log4OM 无需修改即可与 sdroxide 配合使用。仅用于输出——此套接字上的任何内容都无法控制电台。
settings-servers-enable = 启用
settings-servers-send = 发送至
settings-servers-t-127-0-0-1-reaches = 127.0.0.1 可到达本机上的客户端；局域网地址或组播组（224.0.0.1）可到达其他设备
settings-servers-port = 端口
settings-servers-t-2237-port-every-client-defaults = 2237 是每个客户端的默认端口
settings-servers-identify = 标识为
settings-servers-name-clients-see-some-loggers = 客户端看到的名称。某些日志软件只接受自称为 WSJT-X 的流量。
settings-servers-n1mm-contactinfo-broadcast = N1MM+ contactinfo 广播
settings-servers-same-news-n1mm-s-words = 以 N1MM 的格式发布相同的信息：每条已记录的通联一个 XML 数据报，供那些监听 N1MM 而非 WSJT-X 的日志软件使用——世界无线电联盟（World Radio League）的桥接程序两者都支持。仅包含通联；此协议中没有解码数据流。它有自己的开关和自己的端口，因为一个电台站点可能两者都需要。
settings-servers-t-127-0-0-1-reaches-2 = 127.0.0.1 可到达本机上的客户端。N1MM 对比赛网络的建议是该子网的广播地址——对于 192.168.1.n 网络即为 192.168.1.255——它可以同时到达每个操作位置。
settings-servers-t-12060-port-n1mm-s-documentation = 12060 是 N1MM 文档推荐的端口
settings-servers-station-name = 电台站点名称
settings-servers-what-n1mm-calls-stationname-name = N1MM 所称的 StationName：发送该数据包的计算机名称。日志软件借此区分不同的操作位置。
settings-servers-cw-panel-s-copy-plain = 将 CW 面板的文字以纯文本形式发送，每段稳定后作为一个 UDP 数据报发送——用于电台站点控制器、日志软件的 CW 窗口或脚本。没有协议：监听方只需追加接收到的内容（可尝试 nc -ul 9999）。只发送已确定的文字，从不发送解码器仍在修正的尾部。每台电台有各自的开关。
settings-servers-apply = {" "}应用{" "}
settings-servers-persist-re-open-broadcast-sockets = 保存并（重新）打开广播套接字
settings-servers-hamlib-rigctld-server = Hamlib rigctld 服务器
settings-servers-waiting-station-s-rigctld-configuration = 正在等待电台站点的 rigctld 配置……
settings-servers-listen = 监听地址
settings-servers-t-127-0-0-1-machine = 127.0.0.1（本机）
settings-servers-t-0-0-0-0-whole = 0.0.0.0（整个网络）
settings-servers-t-4532-port-every-rigctld-client = 4532 是每个 rigctld 客户端默认采用的端口
settings-servers-rig-name = 电台名称
settings-servers-reported-clients = 报告给客户端的名称
settings-servers-max-clients = 最大客户端数
settings-servers-allow-clients-transmit = 允许客户端发射
settings-servers-off-refuses-every-key-request = 关闭后会拒绝每一个按键请求，并停止宣告发射范围，因此 Hamlib 本身会拒绝按键。
settings-servers-rigctld-has-no-authentication-0 = ⚠ rigctld 没有身份验证。若监听 0.0.0.0，任何能访问该端口的人都可以调谐并按下你的发射机。
settings-servers-t-1-client = 1 个客户端
settings-servers-not-running = 未运行。
settings-servers-status-unknown-press-apply = 状态未知——请按“应用”。
settings-servers-persist-re-bind-server = 保存并（重新）绑定服务器
settings-servers-lets-any-hamlib-capable-program = 允许任何支持 Hamlib 的程序操作此电台：频率、模式、PTT、异频、RIT 和功率。在 WSJT-X、fldigi 或 CQRLOG 中，选择电台“Hamlib NET rigctl”（型号 2）并将其指向此地址；在 GPredict 和 N1MM 中，直接输入主机和端口。与 TCI 服务器不同，它只传输控制信息——没有音频，也没有 IQ。
settings-servers-antenna-rotator = 天线旋转器
settings-servers-waiting-station-s-rotator-configuration = 正在等待电台站点的旋转器配置……
settings-servers-host = 主机
settings-servers-min-elevation = 最低仰角
settings-servers-below-rotator-parks-instead-tracking = 低于此仰角时，旋转器会停靠而不是跟踪——请将其设为你所在地的地平线或屋顶线
settings-servers-azimuth-offset = 方位偏移
settings-servers-added-every-command-rotator-whose = 加入每条命令中，适用于北向有偏差的旋转器
settings-servers-min-movement = 最小移动量
settings-servers-steps-smaller-than-are-not = 小于此值的步进不会发送——追逐十分之一度的移动只会白白磨损电机
settings-servers-park-position = 停靠位置
settings-servers-drive-position-when-idle = 空闲时转到一个位置
settings-servers-off-leaves-antenna-wherever-last = 关闭后，天线停留在上一次过境结束时的位置
settings-servers-not-connected = 未连接。
settings-servers-waiting-station-s-tci-server = 正在等待电台站点的 TCI 服务器配置……
settings-servers-built-tci-server = 内置 TCI 服务器
settings-servers-device-name = 设备名称
settings-servers-off-leaves-control-receive-streams = 关闭后，控制和接收数据流仍然工作，但所有按键请求都会被拒绝。
settings-servers-tci-has-no-authentication-0 = ⚠ TCI 没有身份验证。若监听 0.0.0.0，任何能访问该端口的人都可以调谐并按下你的发射机。

## Settings window, TLE (src/app/settings/tle.rs)

settings-tle-satellites-element-sets-frequencies = 卫星：轨道根数与频率
settings-tle-waiting-station-s-satellite-configuration = 正在等待电台站点的卫星配置……
settings-tle-tracker-already-fetches-celestrak-s = 跟踪器本身已会获取 CelesTrak 的业余卫星组。此处用于其他一切：NOAA 气象卫星、尚未进入该组的新立方星，或比已收到的版本更新的轨道根数。这些列表在电台引擎运行的地方获取并保存，因此无论在哪里打开此应用，卫星集合都相同。
settings-tle-subscriptions = 订阅
settings-tle-listings-fetched-kept-current-same = 列表会获取并保持更新，更新周期与业余卫星集相同，即每六小时一次。太阳窗口打开时以及在此处点击“立即更新”时刷新。
settings-tle-fetch-track-listing = 获取并跟踪此列表
settings-tle-remove-subscription = 删除此订阅
settings-tle-orbits = 轨道
settings-tle-which-satellites-listing-get-orbit = 此列表中哪些卫星会获得轨道环和标签。整组卫星需要选择“精选”：同时显示九十个轨道环会无法辨认，而一个都不显示则只会留下九十个无名圆点。
settings-tle-nothing-listing-sdroxide-s-curated = 此列表中的任何卫星都不在 sdroxide 的精选列表中——该列表只有十颗业余卫星，因此这里的行为将与“无”完全相同。
settings-tle-all-satellites = 所有卫星
settings-tle-catalogue-numbers-keep-comma-separated = 要保留的编目号，用逗号分隔。留空则跟踪列表中包含的全部卫星。
settings-tle-not-fetched-yet = 尚未获取
settings-tle-subscription = + 订阅
settings-tle-update-now = {" "}立即更新{" "}
settings-tle-ask-radio-engine-fetch-every = 要求电台引擎立即获取每个已启用的订阅
settings-tle-celestrak-groups = CelesTrak 分组
settings-tle-pasted-element-sets = 粘贴的轨道根数
settings-tle-one-off-these-do-not = 用于一次性使用。这些不会自动更新，而且 SGP4 在轨道根数超过其历元两周后就会停止推算——凡是要长期使用的，请改为订阅。
settings-tle-track-one = 跟踪此项
settings-tle-show-two-element-lines = 显示两行轨道根数
settings-tle-remove = 删除
settings-tle-paste-two-three-line-element = 在此粘贴两行或三行的轨道根数
settings-tle-add-pasted = + 添加粘贴内容
settings-tle-nothing-paste-box-looked-like = 粘贴框中的内容都不像是轨道根数。
settings-tle-clear-box = 清空文本框
settings-tle-frequencies = 频率
settings-tle-shown-under-pass-table-solar = 显示在太阳窗口的过境表下方。此处的条目会完全替换该编目号内置的条目，因此除非你打算丢弃其余内容，否则请从它的副本开始修改。
settings-tle-remove-satellite-s-entry = 删除此卫星的条目
settings-tle-link = 链路
settings-tle-downlink = 下行
settings-tle-uplink = 上行
settings-tle-mode = 模式
settings-tle-note = 备注
settings-tle-fm-repeater = FM 中继
settings-tle-fm = FM
settings-tle-ctcss-67-0-hz = CTCSS 67.0 Hz
settings-tle-link-2 = + 链路
settings-tle-copy-built = 复制内置项
settings-tle-frequency-either-one-number-145 = 频率可以是一个数字（145.800），也可以是写作 145.950-145.970 的转发器通带。信标的方向留空即可。
settings-tle-norad = NORAD
settings-tle-satellite = + 卫星
settings-tle-frequency-entry-needs-satellite-s = 频率条目需要卫星的 NORAD 编目号。

## Settings window, Wavelog (src/app/settings/wavelog.rs)

settings-wavelog-address = 地址
settings-wavelog-api-key = API 密钥
settings-wavelog-your-wavelog-s-front-page = 你的 Wavelog 首页地址——https://log.example.org 或 http://192.168.1.10/wavelog——以及来自其 API 页面的密钥。上传需要读写密钥。
settings-wavelog-plain-http-address-internet-api = 互联网上某地址的普通 http:// 连接：API 密钥会以未加密方式传输。在你自己的网络内没有问题；跨互联网时请使用 https://。
settings-wavelog-trusted-cert = 受信任的证书
settings-wavelog-forget-certificate = 忘记此证书
settings-wavelog-go-back-ordinary-checking-against = 恢复为依据公共证书颁发机构的常规检查——适用于现在已拥有正式证书的服务器。
settings-wavelog-connection-check-runs-machine-radio = 连接检查在电台所连接的机器上运行——请在那里的设置中测试，并列出电台站点的位置。
settings-wavelog-server-presented-certificate-sha-256 = 服务器提供的证书（SHA-256）：
settings-wavelog-compare-your-server-s-e = 将其与你服务器的证书进行比较——例如 `openssl x509 -noout -fingerprint -sha256 -in cert.pem`——只有两者一致时才信任它。此后将只接受这个确切的证书，其他任何证书都会被拒绝。
settings-wavelog-trust-certificate = 信任此证书
settings-wavelog-station-location = 电台站点位置
settings-wavelog-none = — 无 —
settings-wavelog-press-test-above-list-station = 点击上面的“测试”，列出你的 Wavelog 中定义的电台站点位置。
settings-wavelog-default = 默认
settings-wavelog-where-qsos-go-when-operating = 当操作配置文件未指定位置时，通联记录到哪里。
settings-wavelog-profile = 此配置文件
settings-wavelog-use-default = 使用默认值
settings-wavelog-not-set = 未设置
settings-wavelog-no-station-location-chosen-uploads = 未选择电台站点位置——在选择之前，上传都会失败。
settings-wavelog-qsos-not-wavelog = 不在 Wavelog 中的通联
settings-wavelog-every-qso-logbook-wavelog = 日志中的每一次通联都已在 Wavelog 中。
settings-wavelog-everything-logbook-wavelog-has-not = 日志中所有 Wavelog 尚未确认的内容——包括自动上传开启之前的通联。Wavelog 已有的那些会作为重复项返回，并仅被标记为已发送。
settings-wavelog-set-address-key-station-location = 请先设置地址、密钥和电台站点位置。
settings-wavelog-yes-send-them = 是，发送它们
settings-wavelog-cancel = 取消
settings-wavelog-download-into-logbook = 下载到此日志
settings-wavelog-downloading-runs-where-radio-attached = 下载在电台所连接的机器上运行，并写入该机器的日志——请在那里设置。
settings-wavelog-download-qsos-start-every = 启动时以及每隔多久下载通联
settings-wavelog-downloaded-qsos-join-logbook-so = 下载的通联会加入日志，因此 FT8 和信号列表能知道你已通过其他程序和电台通联过哪些。每次下载只获取新增的内容。
settings-wavelog-press-test-above-choose-which = 点击上面的“测试”，选择要下载哪些电台站点位置。
settings-wavelog-sync-now = 立即同步
settings-wavelog-fetch-what-wavelog-has-added = 获取自上次下载以来 Wavelog 新增的内容。会先应用上面的设置。
settings-wavelog-full-resync = 完全重新同步……
settings-wavelog-download-ticked-locations-whole-compare = 完整下载已勾选的位置，并与日志比较：在 Wavelog 中编辑或删除的通联，或来自已取消勾选位置的通联，都会被同步一致。在任何操作发生之前，你会先看到这些更改。
settings-wavelog-full-resync-changes-make = 完全重新同步——将要进行的更改
settings-wavelog-qsos-logged-here-imported-file = 在此记录或从文件导入的通联永远不会被更改或删除——只有从 Wavelog 下载的通联会如此。
settings-wavelog-apply = 应用
settings-wavelog-discard = 放弃
settings-wavelog-live-radio-status = 电台实时状态
settings-wavelog-send-frequency-mode-wavelog-they = 频率和模式变化时即发送到 Wavelog
settings-wavelog-radio-name = 电台名称
settings-wavelog-pick-radio-wavelog-s-live = 在 Wavelog 的实时记录中选择此电台，其表单会跟随表盘。需要读写密钥。仅限电台站点的主电台。

## Settings window, picture upload: messages with values

settings-picup-server-fingerprint = 服务器证书 SHA-256：{ $fingerprint }

## Settings window, Profiles: messages with values

settings-profiles-apply-hint = 将电台站点恢复为“{ $name }”：表盘、VFO、模式、滤波器、增益、驱动、天线、身份和频段堆栈。
settings-profiles-delete = 删除“{ $name }”

## Settings window, T/R switch: messages with values

settings-relay-not-usable = 尚不可用：{ $why }
settings-relay-receiving = 接收中
settings-relay-rx-tx-words = { $rx } / 发射 { $tx }

## Settings window, Servers: messages with values

settings-servers-n-clients = { $n } 个客户端
settings-servers-listening = ● 正在监听 { $addr } — { $clients }
settings-servers-failed = 失败：{ $error }
settings-servers-rotator-at = ● 已连接——天线方位 { $az }°，仰角 { $el }°

## Settings window, TLE: messages with values

settings-tle-subscription-age = { $count } 颗卫星 · 已过 { $age }
settings-tle-subscribed = 已订阅 { $name }。请按“立即更新”获取。
settings-tle-norad-too-stale = NORAD { $norad } · 已过 { $age } — 过旧，无法推算
settings-tle-norad-age = NORAD { $norad } · 已过 { $age }
settings-tle-norad-n = NORAD { $norad }
settings-tle-added = 已添加 { $n } 组轨道根数。
settings-tle-added-refreshed = 已添加 { $added } 组，并刷新了 { $r } 组轨道根数。
settings-tle-links = { $n } 条链路
settings-tle-replace-links = 用 { $name } 的内置链路替换这些链路

## Settings window, Wavelog: messages with values

settings-wavelog-call-mismatch = 位置“{ $location }”的呼号为 { $station_call }，但你以 { $call } 记录通联——Wavelog 会拒绝这些通联。
settings-wavelog-grid-mismatch = 位置“{ $location }”位于 { $station_grid }，但你的网格定位为 { $grid } ——Wavelog 会拒绝这些通联。
settings-wavelog-location-n = 位置 #{ $id }
settings-wavelog-saved-with-callsign = 已与设置 → 配置文件中的呼号（{ $call }）一同保存，因此便携配置文件可以记录到便携位置。
settings-wavelog-owed = 自开启自动上传以来记录的 { $n } 条通联仍待发送——每 5 分钟重试一次。
settings-wavelog-send-unsent = 发送 { $n } 条尚未在 Wavelog 中的通联……
settings-wavelog-send-confirm = 将 { $n } 条通联发送到 { $location }？会先应用设置。
settings-wavelog-plan-add = • 将有 { $n } 条通联在 Wavelog 中被添加到日志
settings-wavelog-plan-update = • 有 { $n } 条已下载的通联在 Wavelog 中被更改，需要更新
settings-wavelog-plan-remove = • 有 { $n } 条已下载的通联不再在 Wavelog 中（或来自已取消勾选的位置），将被删除
settings-wavelog-plan-mark = • 将有 { $n } 条你自己的通联被标记为已在 Wavelog 中

## Settings window, Radio tab (src/app/settings/radio.rs)

settings-radio-antenna = 天线
settings-radio-which-socket-back-radio-using =
    电台使用背面的哪个插座——即其自身的 ANT 命令，与前面板上 ANT 按钮的设置相同。

    Applies immediately, and it is the radio's setting rather than a copy kept here: the socket the radio is on is read back when the session opens.

    The choice is remembered per band. Switch to the beam on 2 m and the wire on 40, and each comes back the next time the dial crosses into that band — and the next time sdroxide starts. A memory channel stored here carries its socket too.
settings-radio-receive-antenna = 接收天线
settings-radio-radio-s-separate-receiving-antenna =
    电台独立的接收天线——背面的 RX ANT——切入或切出接收通路。这是它自己的设置，与电台上 RX ANT 按钮的设置相同。

    The aerial on the main socket stays on transmit either way: this is an extra input for receiving, a loop or a beverage, not a choice of socket.

    Unlike the antenna above, this is *not* remembered here. The radio holds it per band itself, so sdroxide reads it back after every band change and shows what the radio says; ticking this box is the only thing that moves it.
settings-radio-receive-path = 位于接收通路中
settings-radio-radio-power = 电台电源
settings-radio-switch-radio-itself-off-back =
    通过控制链路，关闭电台本身再重新打开。这是本程序中唯一真正的开关——而非 sdroxide 自己的“连接”开关，后者只关闭 sdroxide 这一端，电台仍在运行。

    For the switch back on to reach anything, the radio's control end has to stay awake while it is off. Over the network that is what Network Control does. Over a serial cable it is the radio's CI-V port, which stays powered from the mains supply on a set that is switched off at the front rather than unplugged; sdroxide sends the wake-up run Icom's own documentation asks for in front of the power-on.

    Switching off ends the audio and the meters — there is no radio behind them — and sdroxide keeps the control link open so the switch back on has somewhere to go.
settings-radio-on = 开
settings-radio-off = 关
settings-radio-waiting-configuration-machine-radio-attached = 正在等待电台所连接机器的配置。
settings-radio-sound-format = 声音格式
settings-radio-invert-spectrum = 反转频谱
settings-radio-swap-i-q = 交换 I/Q
settings-radio-mirror-panadapter-about-tuned-frequency =
    以调谐频率为轴镜像全景图，适用于声卡上 I 和 Q 相反的电台。明显的迹象是：瀑布图上充满了逼真的信号，但全都位于表盘的错误一侧，SSB 则出现在相反的边带上——在声卡上互换两根线缆同样可以解决此问题。

    Receive only: transmit hands the radio one real audio signal, which has no sideband to invert.
settings-radio-i-q-sample-rate = I/Q 采样率
settings-radio-how-fast-radio-s-i =
    电台的 I/Q 声卡以多快的速率运行——这也决定全景图显示多宽的频段，因为正交数据流覆盖其整个采样率：48 kHz 在表盘两侧各为 ±24 kHz，192 kHz 则为 ±96。

    The card has the final say. One that cannot do the rate picked here is opened at the nearest it can, and the panadapter is that much narrower — the log says so at startup when it happens.

    A faster card is also more work: the machine has proportionally less time to empty it, and one that cannot keep up drops samples. That shows up as audio breaking up while the waterfall still looks perfect, and it too is logged. If you see it, come back down a step.
settings-radio-t-48-khz-24-khz = 48 kHz（±24 kHz）
settings-radio-t-96-khz-48-khz = 96 kHz（±48 kHz）
settings-radio-t-192-khz-96-khz = 192 kHz（±96 kHz）
settings-radio-t-384-khz-192-khz = 384 kHz（±192 kHz）
settings-radio-i-q-centre-offset = I/Q 中心偏移
settings-radio-how-far-radio-s-i =
    电台的 I/Q 输出在其自身表盘之上居中的距离，适用于接收中频已偏离零点的电台——例如在 Elecraft 上，将 MENU:RX SHFT 设为 8.0 而非 NOR，这会使表盘避开混频器的直流尖峰（并避免附近强大的 SSB/AM 电台被误作 AM 解调）。

    Not a converter: the radio still displays and transmits on the real frequency, and nothing here is ever sent to it. This only says where the samples on the sound card sit, and the stream is shifted back onto the dial as it arrives.

    Leave at 0 unless you have turned such a menu entry on. If signals land twice the offset away, the sign is the other way round.
settings-radio-iq-correction = IQ 校正
settings-radio-cancel-mirror-image-every-signal =
    消除每个信号的镜像，以及瀑布图中间的直流尖峰。

    A radio's I/Q output is two analogue paths that are never quite equal in gain nor exactly 90° apart, and what that leaves is a copy of every signal reflected about the tuned frequency, usually 30-40 dB down — strong enough to look like a station that is not there, and to be decoded as one. Radios with front-panel balance trimmers are adjusting exactly this; here it is measured off the received noise and needs no setting.

    Leave it on. Turn it off if you are listening to AM tuned dead on the carrier — the carrier is DC, so it goes with the spike — or to check whether a signal is real by watching whether it survives.
settings-radio-cancel-mirror-images = 消除镜像
settings-radio-dc-notch = 直流陷波
settings-radio-widen-hole-taken-out-middle =
    加宽从跨度中间挖去的凹口，适用于中心尖峰比其下方偏移更宽的电台。

    0 leaves the ordinary blocker, which is a few tens of hertz wide and removes the offset without touching anything else. Wind it up and the bottom of the span goes with it: a first-order high-pass, 3 dB down at the figure set here and further in below it.

    It is centred where the radio's I/Q is centred, which is the dial unless the offset above has moved it — so anything tuned there goes too. A CW note at 600 Hz is inside a 600 Hz setting.
settings-radio-panadapter-bw = 全景图带宽
settings-radio-cat-family = CAT 系列
settings-radio-rigctld-address = rigctld 地址
settings-radio-host-port-running-hamlib-rigctld =
    正在运行的 Hamlib rigctld 的 主机:端口——127.0.0.1:4532 是本机上它自己的默认值。

    Start one with, for example, `rigctld -m 2028 -r /dev/ttyUSB0 -s 38400`, where -m is the Hamlib model number for your radio (`rigctl -l` lists them).

    This is the catch-all: it reaches the frequency, mode, PTT, power, S-meter and SWR and nothing else. Where one of the native families above fits your radio it does more — keying from the rig's own text buffer, the receive filter, and per-model meter scales.
settings-radio-flrig-address = flrig 地址
settings-radio-host-port-running-flrig-127 =
    正在运行的 flrig 的 主机:端口——127.0.0.1:12345 是本机上它自己的默认值。flrig 在运行时始终提供 XML-RPC；端口位于其 Config → Setup → Server 中。

    Like the Hamlib option this drives a daemon rather than the radio, but through flrig's own per-model driver — on a number of rigs its power and filter handling is the more faithful of the two. It reaches the frequency, mode, PTT, transmit power (in whole watts), the receive bandwidth, the S-meter, SWR and power-out. CW keys through flrig's own cwio port, which must be configured in flrig itself.
settings-radio-serial-port = 串口
settings-radio-select = — 请选择 —
settings-radio-baud = 波特率
settings-radio-data-bits = 数据位
settings-radio-parity = 校验位
settings-radio-stop-bits = 停止位
settings-radio-force-rts = 强制 RTS
settings-radio-force-dtr = 强制 DTR
settings-radio-ptt-method = PTT 方式
settings-radio-how-transmit-keyed = 发射是如何被按下的。
settings-radio-how-transmit-keyed-network-link = 发射是如何被按下的。网络链路没有控制线，因此 DTR 和 RTS 在此都不起作用——请使用 CAT（它要求守护进程按下电台）或 VOX。
settings-radio-mode-control = 模式控制
settings-radio-digimode-mode = 数字模式下的模式
settings-radio-what-put-radio-mode-sdroxide =
    对于 sdroxide 通过其声卡调制的模式，应将电台设为何种模式。“DIGI”选择电台的 DATA/PKT 位置，该位置从 USB 或 ACC 输入获取发射音频，并且不经过话筒通路的语音处理；“USB”则保持普通的边带，适用于已设为从其数据输入调制 SSB 的电台，或完全没有 DATA 位置的电台。

    It covers every digital mode — FT8, FT4, PSK, RTTY, SSTV and the rest — and CW sent as "Sound card", and it overrides Mode control for them. SSTV is the one whose sideband follows the band, so there it means the DATA position on that sideband: USB-D above 40 m, LSB-D at and below it. SSTV-FM is not part of it — an FM carrier has no sideband to choose.
settings-radio-cw-keying = CW 键控
settings-radio-how-cw-panel-s-keyer =
    CW 面板的键控器如何发射。“电台键控器”将电台设为 CW，并把要发送的文字交给其自己的键控器。它使用电台的键控速度（由面板上的 WPM 设定）；在 Yaesu 上，它通过键控器记忆 1 发送，会覆盖其中原有的内容。

    It needs break-in on at the radio, which sdroxide turns on itself with each message on Icom, Yaesu and Kenwood — a rig with break-in off takes the message and transmits nothing. It is left on afterwards, and a full break-in is never turned down to semi.

    "Sound card" sends the keyed sidetone as audio instead (MCW), a tone at dial + pitch — and because a rig in CW would ignore its sound card entirely, selecting CW then follows the Digimode mode setting (USB, DATA, or Radio controlled) instead of switching the rig to CW. Pick "Radio controlled" there for rigs whose data position can't be commanded over CAT (a Xiegu's U-D) and park the rig on it yourself, as for FT8.
settings-radio-poll-rate = 轮询速率
settings-radio-how-often-radio-asked-what =
    多久询问一次电台正在做什么——它的表盘、模式和表头。这是控制链路中从电台返回 sdroxide 的那一半：转动电台自己的 VFO 旋钮或在前面板更改模式，读数、频段和全景图会在一次轮询之内跟随。

    It is also the whole of the control traffic this end generates, and on a modern Icom that traffic is not free: the CI-V port and the sound card sit behind one USB hub inside the radio, so frames asked for here are bus time the audio does not get — heard as dropouts that look like a DSP fault and are not one. The default of 2 Hz is half a second behind the knob and quiet enough to stay out of the audio's way. Raise it where the control port is its own device (a separate USB-serial adapter, a network rigctld); lower it if the radio's audio breaks up.

    An Icom with CI-V Transceive switched on in its own menu needs none of this for the dial: it reports the knob the instant it moves, and sdroxide stands the dial poll down as soon as it sees it do so — and picks it back up again if the radio ever moves without saying so.

    The rate is the dial's. The mode rides along with only every fourth poll, since it is a setting that changes a few times in an evening rather than one that follows a knob.
settings-radio-send-command = 发送命令
settings-radio-which-transceiver-generation-keys-rig =
    对于 PTT 方式“CAT”，选择哪一代收发信机来按下电台。两者对 TX 参数的含义各不相同，而且线路上没有任何信息能区分它们，因此请选择与你的电台相符的一项。

    "TS-2000 style (TX;)" — TS-480, TS-570, TS-870, TS-2000: the ordinary send, on the main band.

    "TS-590 style (TX1;)" — TS-590S/SG, TS-890, TS-990: DATA SEND, which keys with the ACC2/USB audio input live. On these rigs the plain send selects the microphone instead and mutes the audio sdroxide transmits.

    Set wrong, a TS-590 transmits silence — but a TS-2000 transmits on the sub-band, which is another band entirely.
settings-radio-radio = 电台
settings-radio-k3-k3s-kx3-kx2-k4 = K3 · K3S · KX3 · KX2 · K4
settings-radio-one-profile-covers-family-k3 =
    一个配置文件涵盖整个系列：K3 命令集，KX3、KX2 和 K4 都支持它。此处无需选择——驱动滑块的范围（裸机 KX2 或 KX3 为 12 瓦，搭配 KPA3 或 KXPA100 则为 110 瓦）是在端口打开时，从电台自身的选件模块查询中读取的。

    Note the baud rates above: the K3, K3S, KX3 and KX2 go no faster than 38400, and a rig set below the rate chosen here answers nothing at all.
settings-radio-transmit-input = 发射输入
settings-radio-where-radio-takes-transmit-audio =
    电台从何处获取其发射音频——即电台的 TI 命令，它在前面板的菜单 32“TX IN”中。端口打开时设置。

    "USB audio" is what makes this interface work: the FDM-DUO transmits what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.

    "Auto" lets the rig choose — the microphone for a PTT press on the microphone, the USB port for a CAT or RTS key-down.

    "Leave as set on the radio" sends no TI at all, for an operator who sets this at the front panel.
settings-radio-which-two-sockets-back-receiver =
    接收机监听背面两个插座中的哪一个——即电台的 AN 命令，在前面板的菜单 31“ANTENNAS”中，并在其显示屏上以“ANT 1 2”指示。

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    Applies immediately, and is read back from the radio when the port opens: this is the rig's own setting rather than a copy kept here.
settings-radio-one-profile-covers-both-there =
    一个配置文件涵盖两者。此处无需选择——端口打开时电台会自报名称。

    This is the CAT half only: it drives the dial, the mode, PTT, the S-meter, the SWR and the transmit power over the rig's CAT USB port, and takes audio from its USB Audio port. The FDM-DUO's third USB port carries wideband I/Q, which this interface does not read — select the ELAD interface above for that.

    Note the baud rate: menu 70 "CAT BAUD" on the radio must match, and it ships at 38400.

    CW is keyed by the radio's own key or paddle. The FDM-DUO has no command that accepts text, so the CW panel cannot key it over CAT.
settings-radio-one-profile-covers-range-qrp =
    一个配置文件涵盖整个系列：QRP Labs 自己的命令集，它是 Kenwood TS-480 命令集的子集，并增加了很多内容。此处无需选择——端口打开时电台会自报名称及其固件版本，而该版本决定是否要请求读取驻波比保护。

    Not the Kenwood profile: on a QMX the PC command is the power METER rather than the power control, so a radio driven as a Kenwood would have its meter read written to as if it were a setting, and MD8 — a Kenwood mode — is SWR Tune here.

    The transmit power is set at the radio. There is no CAT command for it, so the Drive slider only reaches the level of the audio going into the sound card.

    The receive filter is the radio's too: it reports the width its mode implies (3.2 kHz in Digi, 300 Hz in CW) and offers nothing to change it with.
settings-radio-baud-rate-ignored-port-usb = 波特率被忽略——该端口为 USB
settings-radio-qmx-serves-own-virtual-com =
    QMX 通过 USB 提供它自己的虚拟 COM 端口，因此上面设置的速率对两端都没有影响。它最多提供三个此类端口；如果电台无应答，最可能的原因是选错了端口。

    One thing does matter about the port: a carriage return sent to it switches the radio into terminal mode for the rest of the session. sdroxide never sends one — but a terminal program left open on the same port will, and CAT stops working the moment it does.
settings-radio-i-q-mode-switched-radio = I/Q 模式在电台上开启
settings-radio-radio-s-sound-card-carries =
    电台的声卡承载的，要么是解调后的音频，要么是其 ADC 所见的原始 I/Q；端口打开时，sdroxide 会按上面“音频格式”的要求设置其中之一（即电台的 Q9 命令，即菜单中的“IQ mode”项）。

    The centre offset and sample rate above have been set to what a QMX needs: its receiver is a superhet with a 12 kHz I.F., so the synthesiser sits 12 kHz below the dial and everything on the card is 12 kHz above the middle of the span. The card runs at 48 kHz, which makes the panadapter 48 kHz wide.

    In CW the radio moves that I.F. by a further ~700 Hz, which one figure cannot follow — add it by hand if you run the panadapter with the radio in CW. In Digi, where a radio used as an I/Q front end normally sits, there is nothing to add.

    QRP Labs note that I/Q mode is not suitable for WSJT-X and other programs that expect demodulated audio; here the demodulation happens on this side, so it is.
settings-radio-rs-hfiq-5-w-hf = RS-HFIQ（5 瓦 HF 收发信机）
settings-radio-hobbypcb-s-rs-hfiq-there =
    HobbyPCB 的 RS-HFIQ。此处无需选择：该配置文件即为电台命令集的全部，端口打开时会记录其固件版本。

    The control link carries two things and no more — where to put the oscillator, and whether to transmit. There is no mode, power, filter, squelch or meter command, because the radio has none of them. Everything else is sdroxide's: what comes off the sound card is complex baseband centred on the dial, so the mode, the filter and the modulation all happen on this side.

    The frequency command covers 3–30 MHz. Ask for anything outside that and the radio refuses it and stays where it is; the log says so.

    CW is keyed as audio through the transmit chain — this radio has no keyer that takes text. (It has an internal CW generator; sdroxide never uses it, on the firmware's own advice.)
settings-radio-sound-format-baud-ptt-are = 音频格式、波特率和 PTT 由电台决定，而非设置项
settings-radio-all-three-have-been-filled =
    这三项都已填写，且没有一项是偏好设置：

    • Sound format is I/Q (stereo) — the card carries complex baseband and nothing else.
    • 57600 baud, 8N1 — the firmware's one rate, with no menu to change it. Any other is a silent link, so it is pinned when the port opens.
    • PTT over CAT — the *X command is the only transmit switch the interface has.

    The centre offset has been set to zero: the synthesiser runs at four times the dial into a quadrature detector, so the middle of the span is the dial. Set the sample rate above to whatever your sound card is actually running at; that is what makes the panadapter as wide as it is.
settings-radio-dl2man-pe1nnz-s-pocket-qrp =
    DL2MAN/PE1NNZ 的袖珍 QRP 收发信机，以及由它发展而来的开源 uSDX 固件。它模拟 Kenwood TS-480，但子集很薄：它只应答表盘、模式、PTT 以及 RIT/XIT 清除。没有 S 表、驻波比、功率控制、VFO B、异频和 CAT 键控器，因此驱动滑块和计量在 CAT 上不起任何作用——音频电平是唯一的发射控制。

    CW is keyed as audio (MCW) through the sideband, or with the key or paddle at the radio; there is no command that takes text.
settings-radio-audio = 音频
settings-radio-tr-usdx-has-no-sound =
    (tr)uSDX 没有自己的声卡，只有两种方式可以被听到，哪一种正确取决于你的操作环境，而非电台本身。

    "One cable" takes the audio inside the CAT serial link itself — the radio's own 8-bit stream, over the same USB cable as the control, with nothing else to plug in. The catch is the firmware: it cannot take a CAT command while its stream is running (a command sent into the stream does not pause it, it kills it), so this mode never polls. Tuning and mode changes are sent as you make them, each pausing the stream for a moment and starting it again, so expect a brief gap on each change; nothing is sent while you are simply listening. The radio's own dial and mode are not followed either — there is no read to follow them with — so drive it from here.

    "USB sound card" takes the audio from an external sound card wired to the radio's 3.5 mm speaker/mic jack, and behaves as any other CAT rig: the dial poll runs, so the radio's own knob and mode are followed. Pick the card under Radio audio below.

    Both modes open the port with DTR held high: the serial adapter's DTR is wired to the radio's reset, so opening the port reboots it and the first second or two is quiet while it comes up. DTR is never used to key.
settings-radio-radio-model = 电台型号
settings-radio-which-icom-two-things-ci =
    选择哪一款 Icom，因为 CI-V 在其中部分型号上的两件事情的做法不同。

    The transceiver address below is filled in from it: every model ships with a different one, and a frame sent to the wrong address is simply ignored — a radio that answers nothing, with no error anywhere to say why.

    It also decides whether sdroxide can select DATA mode. On CI-V, USB and USB-DATA are the same mode byte and a separate command tells them apart; without it a digital-mode over goes out through the microphone input, with the rig's speech processing and SSB filter in the path.

    "Other" leaves the address to you and sends no DATA-mode command at all.
settings-radio-radio-id-hex = 电台 ID（十六进制）
settings-radio-show-radio-s-spectrum-scope = 显示电台的频谱示波器
settings-radio-scope-span = 示波器跨度
settings-radio-how-wide-sweep-radio-keeps = 扫描的宽度。电台会保留其自身屏幕上最后选择的跨度——通常为几 kHz。在此设置跨度还会使示波器进入中心模式，从而跟随表盘。它也会改变电台自身的显示；“保持电台上的设置”则不作改动。
settings-radio-press-apply-reconnect-switch-without = 按“应用 / 重新连接”即可无需重启地切换。
settings-radio-radio-audio-sound-card = 电台音频（声卡）
settings-radio-devices = 设备
settings-radio-discover = 搜索
settings-radio-no-devices-press-discover = 没有设备——请按“搜索”
settings-radio-manual-ip = 手动 IP
settings-radio-optional-e-g-192-168 = 可选，例如 192.168.1.50
settings-radio-sample-rate = 采样率
settings-radio-receiver-ddc = 接收机（DDC）
settings-radio-protocol-2-board-carries-several = Protocol 2 板卡可在一条连接上承载多个可独立调谐的接收机（DDC）——例如在 DDC1 上运行此电台，在 DDC2 上运行另一台电台（地址相同）。发射机属于 DDC1 的电台。Protocol 1 板卡只有 DDC1。
settings-radio-lna-gain = LNA 增益
settings-radio-front-end-gain-hermes-lite = Hermes-Lite 2 的前端增益。立即生效——无需重新连接——并会被记住为电台启动时的电平。过高会使 ADC 削波，整个频段看起来失真；过低则接收机会变聋。
settings-radio-overload-protection = 过载保护
settings-radio-back-lna-gain-off-itself =
    当板卡报告其 ADC 过载时，自动回退 LNA 增益，待过载停止后再将其恢复。

    A Hermes-Lite 2 samples the whole of 0-38 MHz onto one 12-bit converter with no mixer and no preselector in front of it, so a broadcast station a band away can drive it into overflow while the band you are looking at shows nothing wrong at all — the noise floor climbs, everything intermodulates and the decoders stop. The board knows, and this is what acts on it.

    Off by default: it moves a control you set. Nothing happens while transmitting.
settings-radio-wind-gain-back-when-adc = ADC 过载时回调增益
settings-radio-applies-apply-reconnect-main-window = 在“应用 / 重新连接”时生效。主窗口的增益滑轨读取板卡实际运行的增益，因此会跟随此循环，你可以观察它的动作；上面的滑块是电台启动时的电平，循环不会改写它。
settings-radio-how-far-gain-moves-each = 每次增益移动的距离，单位为分贝。一分贝即为板卡自身增益寄存器的步进。
settings-radio-attack-decay = {"  "}攻击 / 衰减
settings-radio-how-often-gain-may-come =
    当变换器过载时，增益可以多久下调一次；过载停止后，又可以多久上调一次。

    The two are deliberately a hundred times apart. Retreat immediately: every millisecond of overflow is a receiver full of intermodulation. Return slowly: whatever caused it — a neighbour keying, a broadcaster coming up at dusk — has usually not gone away, and a loop that recovered as fast as it retreated would spend the evening oscillating across the threshold. 100 ms and 10 s per decibel is what PowerSDR's Auto S-Att and N1GP's HermesIntf have both used.
settings-radio-range = {"  "}范围
settings-radio-lowest-highest-gain-loop-may = 循环可以使用的最低和最高增益，单位为分贝。上限的作用是防止它替你决定接收机的灵敏度；下限则是你设定的界限：低于某一点时，过载就成了别人的问题，解决办法是加滤波器，而不是再多加二十分贝。
settings-radio-filter-board = 滤波器板
settings-radio-accessory-board-hermes-lite-2 = Hermes-Lite 2 的 J16 接口上的附件板（或任何其他 openHPSDR 板卡的开集电极输出——包括 Protocol 2）。“N2ADR”为每个频段选择一个继电器；“Alex / Hermes 频段代码”将频段编号放在输出 1–4 上，这是 ANAN 的 Alex 板、Zeus SDR、HiQSDR 和 Quisk 所期望的。“自定义”会打开下方的一个表格，由你逐频段、分别为接收和发射指定七个输出——用于天线开关、功放的频段译码器或其他两种预设都不适用的情况；任一预设都可以先复制进去作为起点。除非确实安装了相应设备，请保持为“无”：这七个引脚是通用的开集电极输出，操作员也会把它们接到功放 PTT、天线继电器和变频器切换上。若根据频段数据驱动它们，就会开始操作所连接的任何设备。在“应用 / 重新连接”后生效。
settings-radio-io-board-rx-input = IO 板 RX 输入
settings-radio-where-n2adr-hl2ioboard-takes-receive = N2ADR HL2IOBoard 从何处获取其接收信号。板卡本身会被自动识别，无需设置；此项仅供已将其自身 SMA 插座接线的操作员使用。否则请保持为“电台自身的输入”——若选择 IO 板的 J9 但其上未接任何东西，接收机就会变聋。在“应用 / 重新连接”后生效。设为任一 J9 选项后，这些输入也会出现在 ANT 控件中，并按频段记忆。
settings-radio-power-amplifier = 功率放大器
settings-radio-use-hermes-lite-2-s = 使用 Hermes-Lite 2 的板载功放
settings-radio-default-what-you-want-unless = 默认开启，除非外置功放由板卡的低功率 RF1 输出驱动，否则这正是你需要的设置。关闭时电台仍会键控——收发继电器会动作，任何附件板也会跟随——但天线插孔完全不输出功率，并且继电器被有意保持在接收状态。仅对 Hermes-Lite 板卡有效，其他板卡忽略此项。
settings-radio-mirror-board-s-spectrum-about = 以调谐频率为轴镜像板卡的频谱，发射和接收都适用。默认开启：Hermes-Lite 2 需要它。只有当信号出现在表盘的错误一侧而且无法解码时才应关闭——明显的迹象是：瀑布图上充满了逼真的轨迹，而 SSB 落在了错误的边带上，FT8 则完全没有解码结果。
settings-radio-frequency-correction = 频率校正
settings-radio-crystal-tcxo-error-ppm-applied = 晶体/TCXO 误差，单位为 ppm，同时应用于接收和发射。立即生效。
settings-radio-ppm = {" "}ppm
settings-radio-transmit-buffer = 发射缓冲
settings-radio-how-far-ahead-real-time = 发射音频通过网络被送入板卡时，领先实时多远，之后 sdroxide 才会放慢速度以与之匹配。板卡本身不保存此类缓冲——这仅会扩大 sdroxide 自身的余量。在 WiFi 链路或 VPN 上应提高它：对于直连有线连接而言正确的较低默认值，不足以抵御抖动，发射音频或 PTT 会出现卡顿；数值越高，发射延迟越大。在“应用”后生效，此操作会重新连接板卡。
settings-radio-puresignal = PureSignal
settings-radio-adaptive-predistortion-linearise-transmitter-sample = 自适应预失真：根据发射机实际输出的一份样本使其线性化，在相同功率下可减少二十余分贝的互调。接收机是反馈通路——板卡在通话期间持续接收——因此定向耦合器和衰减器必须把功放输出的一份样本送入一个发射时不会被收发开关切断的输入。在 Hermes-Lite 2 上，这意味着使用 IO 板的 PureSignal 插孔，并将上面的接收输入设为与之相符；在没有该插孔的板卡上，则使用 ANAN 的 RX 端口，或在通话期间切入耦合器的外部继电器。被读取的是这台接收机——带有第二个 ADC 的板卡无法将 RX2 用作反馈抽头，无论其自身固件如何路由它。若没有耦合任何信号，循环将永远无法锁定，发射机则保持在原本的状态。在“应用 / 重新连接”后生效，并且仅对拥有发射机的电台（DDC1）有效。
settings-radio-correct-transmitter-receiver-s-own = 利用接收机自身的反馈校正发射机
settings-radio-how-finely-amplifier-s-curve = 功放的曲线建模的精细程度。步数越多，越能贴合陡峭的拐点，但填充所需时间也越长；32 是一个合理的起点。
settings-radio-how-fast-correction-follows-what = 校正跟随耦合器报告内容的速度。慢速是正确的选择——这是从一条带有噪声的反馈通路中，对一条不会移动的功放曲线做平均。
settings-radio-hold = 保持
settings-radio-stop-adapting-keep-correction-where = 停止自适应，并将校正保持在当前位置——用于测量，以及对其已学习的结果感到满意的操作员。
settings-radio-manual-ip-overrides-discovery-press = 手动 IP 会覆盖搜索结果。按“应用 / 重新连接”即可无需重启地切换。
settings-radio-open-collector-outputs-band = 按频段的开集电极输出
settings-radio-one-control-word-per-band = 每个频段一个控制字，与你的硬件文档所述一致：位 0 为输出 1，位 6 为输出 7。在该频段接收时 RX 有效，发射机按下时 TX 有效——对于滤波器，两者设为相同的值；对于只应属于切换某一侧的设备（功放的按键线路、接收前置放大器的旁路），则设为不同的值。保持为 00 的频段不输出任何信号。在“应用 / 重新连接”后生效。
settings-radio-alex-band-code = ALEX 频段代码
settings-radio-replace-table-what-preset-would = 用此预设在每个频段上会发送的内容替换该表格，作为可供编辑的起点。配置中的其他内容不会改变。
settings-radio-clear = 清除
settings-radio-every-output-off-every-band = 每个频段上的所有输出都关闭，这与“无”的效果相同。
settings-radio-band = 频段
settings-radio-rx = RX
settings-radio-tx = TX
settings-radio-outputs-asserted = 已输出的信号
settings-radio-other = 其他
settings-radio-everywhere-outside-amateur-bands-short = 业余频段之外的所有位置——短波收听，以及变频器表盘落到任何频段都未覆盖之处。
settings-radio-dongle = 加密狗
settings-radio-rescan = 重新扫描
settings-radio-re-list-usb-bus-no = 重新列出 USB 总线。不会打开任何设备，因此在接收时按下也是安全的。
settings-radio-no-dongles-press-rescan = 没有加密狗——请按“重新扫描”
settings-radio-first-one-found = — 找到的第一个 —
settings-radio-rtl2832u-s-resampler-reaches-225 = RTL2832U 的重采样器覆盖 225–300 kHz 和 900 kHz–3.2 MHz，两者之间没有其他范围。在“应用”后生效。
settings-radio-often-drops-samples = {"  "}（常常丢失采样）
settings-radio-agc = AGC
settings-radio-manual-setting-measurement-weak-signal = 手动是用于测量和弱信号数字模式的设置。调谐器与解调器各有独立的自动环路。
settings-radio-tuner-gain = 调谐器增益
settings-radio-applies-immediately-no-reconnect-tuner = 立即生效——无需重新连接。调谐器有 29 个离散步进，因此数值会贴合到其能够产生的最接近的一档。调谐器 AGC 运行时忽略此项。
settings-radio-crystal-error-parts-per-million = 晶体误差，单位为百万分之一。以 RUST_LOG=sdroxide_rtlsdr=debug 运行，日志会在约 20 秒后打印测得的时钟误差——这就是应当输入的数值。立即生效。
settings-radio-hf-reception = HF 接收
settings-radio-tuner-itself-starts-24-mhz =
    调谐器本身从 24 MHz 开始。RTL-SDR Blog V4 在硬件上对其以下的频率做上变频；其他加密狗只能通过直接对 ADC 采样才能接收 HF，即经由 V3 的 HF 端口。“自动”会把 24 MHz 以下的全部交给 ADC，而把以上的交给调谐器。

    Direct sampling covers every HF band, 17 m and 15 m included — they arrive in the ADC's second Nyquist zone, the right way up. Nothing filters the ADC's input, though, so whatever is at 28.8 MHz minus the dial comes with them: 10.7 MHz under 17 m, 7.726 MHz under 15 m.

    Switching modes briefly interrupts the stream.
settings-radio-removes-dongle-s-own-dc =
    通过测量采样本身的不平衡，从跨度中心去除加密狗自身的直流尖峰，以及每个信号围绕它留下的镜像——无需校准，并且立即生效。调谐器没有偏移调谐模式，因此这是清除中心的唯一方法。

    An AM carrier tuned dead on the dial is at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-remove-centre-spike-mirror-image = 去除中心尖峰和镜像
settings-radio-bias-tee = 偏置三通
settings-radio-feed-4-5-v-dc = 通过同轴电缆供给约 4.5 V 直流
settings-radio-bias-tee-never-connect-transceiver = 偏置三通已开启。此功能启用期间，切勿连接收发信机、接地天线或由别处供电的前置放大器——直流会直接沿馈线进入。
settings-radio-receive-only-dongle-sample-rate = 仅接收。加密狗和采样率在“应用”后生效；其他设置在你更改时即生效。
settings-radio-server-address = 服务器地址
settings-radio-where-rtl-tcp-listening-address =
    rtl_tcp 正在监听的位置：一个地址，或一个地址加端口。端口默认为 1234，即 rtl_tcp 自身的默认值。

    On the far end, start it as `rtl_tcp -a 0.0.0.0` — bound to 127.0.0.1, which is what it does with no -a, it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g = 主机或 主机:端口，例如 raspberrypi.local:1234
settings-radio-requested-server-same-resampler-limits =
    向服务器请求，其重采样器限制与本地加密狗相同——远端使用的是同一块芯片。

    The figure beside each rate is what it costs on the link: the samples are sent uncompressed, and a rate the network cannot carry makes rtl_tcp drop the connection rather than degrade. Takes effect on Apply.
settings-radio-wired-link = {"  "}（有线链路）
settings-radio-runs-server-s-dongle-manual = 在服务器的加密狗上运行。手动是用于测量和弱信号数字模式的设置。
settings-radio-applies-immediately-no-reconnect-sent = 立即生效——无需重新连接。以十分之一分贝为单位发送，并由服务器贴合到其调谐器所具有的步进；该协议没有回复，因此无法读回它最终设定的数值，此滑块将继续显示所请求的数值。调谐器 AGC 运行时忽略此项。
settings-radio-crystal-error-server-s-dongle =
    服务器加密狗的晶体误差，单位为百万分之一——它是该硬件的属性，因此在此设置，而非在本机的加密狗上设置。立即生效。

    The measured clock error the USB interface prints is not available here: over a network what that measurement sees is the buffering, not the crystal, and it is wrong by thousands of ppm. Calibrate the dongle on USB once and carry the number over, or tune a broadcast station of known frequency and adjust until it sits on the dial.
settings-radio-tuner-starts-24-mhz-below =
    调谐器从 24 MHz 开始；低于此频率时，远端需要帮助。Blog V4 会自行上变频，而“自动”会让它保持原样——这是它唯一能做的事，因为协议只报告调谐器芯片，而没有其他信息，而且 V4 在线路上与普通的 R828D 无法区分。

    On a V3 or any other dongle, Automatic switches the server to direct sampling below the tuner's own 24 MHz floor — which covers every HF band, 17 m and 15 m arriving in the ADC's second Nyquist zone with 28.8 MHz minus the dial folded on top. Choose Direct sampling explicitly for a plain R828D that hears nothing on HF. Switching briefly interrupts the stream.
settings-radio-removes-dongle-s-dc-spike =
    在此处从采样到达时去除加密狗的直流尖峰和镜像——它们是硬件产生的伪影，因此会与其他一切一同通过网络传输。立即生效。

    An AM carrier tuned dead on the dial sits at DC too, so it goes with the spike: tune a kilohertz off it, or switch this off.
settings-radio-powers-preamp-far-end-s = 通过远端的加密狗为前置放大器供电。较旧的服务器没有实现此命令，并会静默地忽略它——协议没有表示“否”的方式——因此偏置三通没有开启，并不一定是这一端的问题。
settings-radio-feed-4-5-v-dc-2 = 通过远程同轴电缆供给约 4.5 V 直流
settings-radio-bias-tee-hardware-somewhere-else = 偏置三通已开启，而其硬件位于别处。该馈线另一端的任何东西——收发信机、接地天线、已经供电的前置放大器——都不在你面前，无法查看。
settings-radio-sdrplay-server-rsp-tcp = SDRplay 服务器（rsp_tcp）
settings-radio-sdrplay-server-greets-exactly-like = SDRplay 服务器的问候方式与加密狗完全相同，因此这些选项始终显示，而不是在检测到设备时才显示。对于普通的 rtl_tcp 服务器，它们会被直接忽略——该协议没有回复，并会丢弃它不认识的命令。服务器一旦自我识别，设备选项卡便会将其标为 rsp_tcp。
settings-radio-input = 输入 A
settings-radio-input-b = 输入 B
settings-radio-hi-z = 高阻
settings-radio-lna-state = LNA 状态
settings-radio-step-index-not-db-figure = 步进索引，而非分贝数：每一步的价值取决于 RSP 型号和频段。0 为最大增益。
settings-radio-if-gain-reduction = 中频增益衰减
settings-radio-reduction-so-more-less-signal = 衰减量，因此数值越大，信号越弱。仅在 RSP 的 AGC 关闭时生效。
settings-radio-enable = 启用
settings-radio-level-rsp-s-agc-aims = RSP 的 AGC 力图维持的电平。
settings-radio-notches = 陷波
settings-radio-am = AM
settings-radio-fm = FM
settings-radio-dab = DAB
settings-radio-rf = RF
settings-radio-reference-out = 参考输出
settings-radio-t-24-mhz-clock-out = 24 MHz 时钟输出
settings-radio-rsp2-rspduo-only = 仅 RSP2 和 RSPduo。
settings-radio-receive-only-address-sample-rate = 仅接收。地址和采样率在“应用”后生效；其他设置在你更改时即生效。连接断开后会自动重试，因此服务器重新启动后，无需在此处做任何操作即可恢复。
settings-radio-where-spyserver-listening-address-address =
    spyserver 正在监听的位置：一个地址，或一个地址加端口。端口默认为 5555，即 spyserver 自身的默认值。

    On the far end, check that its config file binds an address other machines can reach — bound to 127.0.0.1 it only accepts connections from that same machine.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-2 = 主机或 主机:端口，例如 raspberrypi.local:5555
settings-radio-i-q-bandwidth = I/Q 带宽
settings-radio-how-wide-slice-band-arrives =
    频段中有多宽的一段以 I/Q 形式到达——因此它占用了多少链路。这是*解调*窗口：它跟随表盘，比它更宽的部分则是服务器在全景图上方的条带中的 FFT。

    Automatic aims at about 96 kHz, which carries every mode here including wide FM and still fits down a cellular uplink. Takes effect on Apply.
settings-radio-which-stage-server-s-own = 向服务器的自身速率阶梯请求哪一级。每个接收机的阶梯各不相同——它是其最大速率逐级减半——因此这里保存的是级别而非赫兹数，同一设置指向另一台服务器时仍然有意义。

    Automatic aims at about 1 Msps. Press Test connection to see what the stages come to on this server. Takes effect on Apply.
settings-radio-automatic = 自动
settings-radio-sample-format = 采样格式
settings-radio-bits-per-component-wire-so =
    线路上每个分量的位数，以及由此在链路上的开销：16 位是 8 位的两倍，而 32 位浮点则是它的四倍，但所携带的信息并不比接收机的 ADC 更多。

    8-bit is what makes a remote receiver work over a domestic uplink and is right for almost everything. A server may override this — some are configured to insist on one format — and says so in the log when it does. Takes effect on Apply.
settings-radio-gain = 增益
settings-radio-server-s-gain-stage-index =
    服务器的增益级，以索引表示——而非分贝数。每个索引的含义取决于远端的接收机，并随频段而变化，而协议从不说明，因此此处无法在不凭空捏造数值的情况下将其换算为分贝。

    The real range is the server's and is only known once connected; an index past it is clamped. Applies immediately — no reconnect.
settings-radio-index = 索引{" "}
settings-radio-digital-gain = 数字增益
settings-radio-how-far-server-scales-samples =
    服务器在将采样量化以用于线路传输之前，向上缩放多少。“自动”会监视实际到达的信号，并将峰值保持在满量程的一半，这是唯一能同时适用于空旷频段和拥挤频段的做法——二者相差三十分贝，没有任何固定数值能同时适用于两者。

    It matters only at 8 bits, where a signal far below full scale loses its lower bits to the quantiser and one above it is clipped flat. Wider formats have room to spare and get the figure every other client computes, from the gain index and the decimation stage. Applies immediately.
settings-radio-full-band-strip = 全频段条带
settings-radio-server-s-own-fft-whole =
    服务器对整个频段进行的 FFT，绘制在全景图上方的条带中。在本界面中，它是唯一的频段视图——全景图本身只与所接收的 I/Q 一样宽。

    Switching it off leaves a receiver with no way to see anything it is not already tuned to. Applies immediately, with a short break in the audio while the server changes what it is sending.
settings-radio-ask-server-low-rate-fft =
    同时向服务器请求整个频段的低速率 FFT 和 I/Q，并将其绘制在全景图上方的条带中。

    It costs almost nothing — a couple of kilobytes a frame, a dozen or so times a second — and it shows the whole receiver rather than the slice being demodulated. Worth switching off only on a link where every byte counts. Applies immediately, with a short break in the audio.
settings-radio-show = 显示
settings-radio-whole-band = 整个频段
settings-radio-how-much-receiver-strip-covers = 条带覆盖接收机的多大范围。整个频段是可用的最宽视图；缩窄它会把相同数量的频点分布在更少的频谱上，即在较小的范围内获得更精细的细节。
settings-radio-strip-db-window = 条带 dB 窗口
settings-radio-server-quantises-fft-into-window =
    服务器在发送之前，把其 FFT 量化到此窗口内，每个频点一个字节——因此这些设置决定的是条带的测量精细程度，而不仅仅是它的绘制方式。条带*显示*所用的底噪和顶端，由引擎自己的自动电平调整决定，是另一回事。

    The default 150 dB is the whole protocol range and needs no attention. Narrowing it around the noise floor buys resolution on a receiver whose signals all sit in a small part of the scale. Applies immediately.
settings-radio-top = 顶部{" "}
settings-radio-range-2 = 范围{" "}
settings-radio-i-q-correction = I/Q 校正
settings-radio-remove-dc-spike-mirror-image = 在本端通过 DSP 去除直流尖峰和镜像。远端的接收机是否需要它取决于其类型——Airspy HF+ 不需要，RTL-SDR 则需要——而协议并不说明它在与哪一种接收机通信。立即生效。
settings-radio-enabled = 启用
settings-radio-test-connection = 测试连接
settings-radio-connect-read-what-server-says = 连接，读取服务器对其接收机的说明，然后再断开——不启动数据流，因此对别人正在使用的服务器按下也是安全的。
settings-radio-receive-only-panadapter-narrow-i = 仅接收。全景图是跟随表盘的窄幅 I/Q 窗口；频段视图则是其上方条带中的服务器 FFT。按下“应用 / 重新连接”即可无需重启地切换。
settings-radio-receive-only-wideband-i-q = 仅接收。宽带 I/Q，与任何本地 SDR 相同。按下“应用 / 重新连接”即可无需重启地切换。
settings-radio-receiver-address = 接收机地址
settings-radio-where-receiver-serves-web-page =
    接收机提供其网页的位置：一个地址，或一个地址加端口。端口默认为 8073，即 KiwiSDR 自身的默认端口。

    Note that a receiver reached through the project's proxy (something.proxy.kiwisdr.com — nearly half the public ones) answers on port 80 instead, so give it explicitly. Picking a receiver from "Public SDRs" fills this in correctly.

    Takes effect on Apply.
settings-radio-host-host-port-e-g-3 = 主机或 主机:端口，例如 kiwi.example.org:8073
settings-radio-password = 密码
settings-radio-receiver-s-user-password-where =
    接收机的*用户*密码，在其操作员设置了密码的情况下。几乎所有公共接收机都留空。不是管理员密码。

    Takes effect on Apply.
settings-radio-announce = 显示名称
settings-radio-name-connection-shows-up-under =
    此连接显示的名称，接收机的所有者以及收听它的其他人都可以看到。

    Left blank it is your station callsign, which is the network's own convention — some operators do block clients that will not identify. Takes effect on Apply.
settings-radio-blank-station-callsign = 留空 = 电台呼号
settings-radio-band-view = 频段视图
settings-radio-ask-receiver-own-waterfall-well =
    同时向接收机请求其自身的瀑布图和 I/Q，并将其绘制在全景图上方的条带中。

    Worth having: the I/Q is only about 12 kHz wide, so without this there is nothing to tune *by*. It is a second connection carrying roughly 20 kB/s at full speed, against the I/Q's 44 kB/s.

    Takes effect on Apply.
settings-radio-show-receiver-s-0-30 = 显示接收机的 0–30 MHz 瀑布图
settings-radio-band-view-speed = 频段视图速率
settings-radio-how-often-receiver-sends-waterfall =
    接收机发送瀑布图行的频率，从 1（最慢）到 4。这是此处唯一会改变运行时链路开销的设置，因此在按流量计费的连接上请调低它。

    Applies immediately.
settings-radio-band-view-span = 频段视图跨度
settings-radio-how-much-band-receiver-s =
    接收机的瀑布图覆盖多大频段。滑块最左端为完整的 0–30 MHz，向右每一步减半。

    The waterfall is always 1024 bins wide however wide the window is, so this is really a resolution control: the whole band is 29 kHz to a bin, which is a band *map* rather than a picture of anything, while a few hundred kilohertz shows individual stations in a broadcast band. Zoomed in, the window follows your dial.

    The whole band is the default and is what makes the strip a thing to tune by; the panadapter below it is the ~12 kHz of I/Q. Applies immediately.
settings-radio-receiver-agc = 接收机 AGC
settings-radio-receiver-s-agc-which-far =
    接收机的 AGC，它位于链路另一端，且在 I/Q 之前——因此它先于 sdroxide 所做的任何处理而动作。

    On by default, unlike every local SDR here, because the measured alternative was worse: on a live receiver the manual gain was not monotonic across its range and its top end clipped the I/Q at full scale, while the AGC held about -24 dBFS with 7 dB of headroom.

    It does mean the sample amplitude is not a signal level, which is why the S-meter is read from the receiver's own figure instead. Applies immediately.
settings-radio-let-receiver-ride-own-gain = 让接收机自行控制增益
settings-radio-manual-gain = 手动增益
settings-radio-fixed-gain-when-receiver-s = 接收机 AGC 关闭时的固定增益，使用其自身的 0–90 刻度。这不是任何已说明的分贝数——协议称之为 manGain，并不多作说明，而且经测量发现它并非单调变化，因此应将其视为用来寻找良好位置的旋钮，而非校准值。立即生效。
settings-radio-read-receiver-s-status-page = 读取接收机的状态页面：它是什么、覆盖哪些范围、还有多少空闲通道，以及其操作员是否允许浏览器以外的应用连接。不占用它的任何通道。
settings-radio-host-port-e-g-127 = 主机:端口，例如 127.0.0.1:50001
settings-radio-iq-sample-rate = IQ 采样率
settings-radio-receiver = 接收机
settings-radio-rig-two-receivers-sunsdr2dx-can = 带两台接收机的电台（SunSDR2DX）可通过一条连接为两个电台选项卡提供服务——例如在 RX1 上运行此电台，在 RX2 上运行另一台。发射机属于 RX1 的电台。
settings-radio-stream-delay = 数据流延迟
settings-radio-how-long-rig-s-iq = 在 sdroxide 将电台的 IQ 移至新的中心频率后，它需要多长时间才能到达。这是电台自身的 DSP 流水线，而非网络——本机上的服务器同样有这一流水线——而当全景图被完全缩小拖动时，偏差达到这一量级，会使最新的瀑布图行相对于历史记录横向错位，错位量等于误差乘以拖动速度。设得过高或过低，错位的距离相同，方向相反。可用 `cargo run --release -p sdroxide-tci --example retune_latency` 测量它；默认值针对的是在 ExpertSDR3 上以 192 kHz 运行的 SunSDR2DX。
settings-radio-wideband-iq-receive-audio-transmit = 宽带 IQ 接收，音频发射。按下“应用 / 重新连接”即可无需重启地切换。
settings-radio-radio-address = 电台地址
settings-radio-address-shown-radio-under-set = 电台在 SET > Network 下显示的地址。那里必须开启“Network Control”，并且电台需要设置网络用户名和密码。
settings-radio-host-ip-e-g-192 = 主机或 IP，例如 192.168.1.50
settings-radio-control-port = 控制端口
settings-radio-t-50001-unless-has-been-changed = 除非在电台上被更改，否则为 50001。
settings-radio-network-user = 网络用户
settings-radio-stored-clear-radio-json-protocol = 以明文保存在 radio.json 中。协议在线路上以可逆方式对其进行混淆，因此在此处做任何处理都不会使它成为秘密。
settings-radio-receive = 接收来源
settings-radio-audio-sample-rate = 音频采样率
settings-radio-t-12-khz-if-needs-48000 = 12 kHz IF 需要 48000 Hz 的数据流——在此速率下，使用的是电台解调后的音频。
settings-radio-if-spectrum = IF 频谱
settings-radio-which-way-round-radio-s = 电台的 12 kHz IF 以哪个方向运行。“自动”会采用已知该型号的做法——在 IC-7760 上为镜像，在其他所有 Icom 上为正常。若 SSB 出现在错误的边带上，请手动设置：其明显迹象是，在频段运行 LSB 的地方你必须选择 USB，同时电台自己的模式显示始终与 sdroxide 一致。
settings-radio-normal = 正常
settings-radio-mirrored = 镜像
settings-radio-displayed-bandwidth = 显示带宽
settings-radio-width-audio-band-panadapter-cat = 音频频段全景图的宽度，与 CAT 电台相同。用于数字模式以及电台示波器关闭时；否则全景图本身就是示波器，其宽度由“示波器跨度”设定。
settings-radio-how-cw-panel-s-keyer-2 = CW 面板的键控器如何发射。“电台键控器”将电台设为 CW，并把要发送的文字交给其自身的键控器（CI-V 0x17）。“声卡”则改为通过局域网以音频方式发送键控的侧音（MCW），音调为表盘 + 音调偏移——由于 CW 模式下的电台会忽略其调制器输入，因此选择 CW 后会让电台保持在普通的 USB 模式，即数字模式所使用的模式。
settings-radio-how-much-audio-radio-holds = 电台在调制之前保留多少音频。网络越差，保留得越多，代价是发射延迟越大。
settings-radio-streams-radio-s-own-475 = 传输电台自身的 475 频点扫描。在 AF 下，它就是全景图——电台发送的音频是经过其滤波器的结果，而非频段的画面；在 12 kHz IF 下，它则是其上方的全频段瀑布图。无论哪种方式，它都是电台的画面，而非 sdroxide 的 DSP 生成的：没有 I/Q 可供计算它。
settings-radio-how-wide-sweep-only-wide = 扫描的宽度。这是 Icom 唯一的宽视图，而电台会保留其自身屏幕上最后选择的跨度——通常为几 kHz，这就是条带有时无法比其下方的全景图更宽的原因。在此设置跨度还会使示波器进入中心模式，从而跟随表盘。它也会改变电台自身的显示；“保持电台上的设置”则不作改动。
settings-radio-switch-modulation-input-lan = 将调制输入切换为 LAN
settings-radio-transmit-audio-only-heard-when = 只有当电台的 MOD 输入设为 LAN 时，发射音频才会被听到。若 sdroxide 认识该型号的菜单编号，它可以替你写入该设置；对于其他型号，它会说明情况并保持菜单不变。这是临时借用：会话开始时先读取电台在此处保存的内容，会话结束时再将其放回，因此之后单独使用的电台仍会听到它自己的话筒。
settings-radio-copy-diagnostic-report = 复制诊断报告
settings-radio-control-audio-radio-s-own = 控制、音频以及电台自身的示波器，都通过一条网络连接完成——无需串口线缆，也无需声卡。按下“应用 / 重新连接”即可无需重启地切换。
settings-radio-testing = 测试中……
settings-radio-was-only-check-press-apply = 那只是一次检查——请按下方的“应用 / 重新连接”，开始使用此电台。
settings-radio-radios = 电台
settings-radio-asks-network-iio-devices-also = 向网络查询 IIO 设备，并直接尝试 192.168.2.1——通过 USB 线缆连接的 Pluto，即使该地址可以访问，也常常无法通过组播发现。
settings-radio-no-radios-press-discover = 没有电台——请按“搜索”
settings-radio-address = 地址
settings-radio-overrides-selection-above-usb-cable = 覆盖上面的选择。USB 线缆将 Pluto 呈现为网络适配器，而非串口，因此即使电台就在你的桌上，这里填写的也是 IP 地址。
settings-radio-pluto-revision-c-pluto-unlocked = Pluto+ 或已解锁为 2R2T 的 C 版 Pluto，可以从一台设备为两个电台选项卡提供服务——此电台使用 RX1，另一台使用 RX2，各自接在自己的天线上。两条通道共用同一个振荡器，因此重新调谐任一电台都会同时移动两者；RX2 带来的是同一频谱上的第二根天线（分集），而不是第二个频段。发射机属于 RX1 的电台。标准的 1R1T Pluto 在连接时会拒绝 RX2。
settings-radio-width-spectrum-sdroxide-receives-ad9361 = sdroxide 所接收频谱的宽度。AD9361 可达 61.44 Msps；USB 网络链路则达不到，此列表即按此进行缩放。最低的速率需要加载到 AD9361 中的滤波器配置，而 sdroxide 不做这件事——标准的 Pluto 会改为以约 2.084 Msps 运行，并在连接时说明这一点。在“应用”后生效。
settings-radio-more-than-usb-2-will = {"  "}（超过 USB 2 所能承载的范围）
settings-radio-stock-pluto-runs-2-084 = {"  "}（标准 Pluto 运行于 2.084）
settings-radio-analog-filter = 模拟滤波器
settings-radio-ad9361-s-baseband-filter-leave = AD9361 的基带滤波器。保持为 0 即为自动，此时它会打开到采样率的九成——之所以刻意设得很宽，是因为接收机会把其振荡器偏离表盘四分之一个跨度，以使信号远离直流尖峰；窄滤波器会恰好切掉它被移到的那一部分。在“应用”后生效。
settings-radio-ad9361-has-four-modes-not = AD9361 有四种模式，而不是开/关开关。慢速起振适合 SSB 和 CW；快速起振适合突发信号；手动则是用于测量和弱信号数字模式的设置。立即生效。
settings-radio-rx-gain = RX 增益
settings-radio-applies-immediately-no-reconnect-ignored = 立即生效——无需重新连接。除非 AGC 设为手动，否则忽略此项，这是 AD9361 自身的行为，而非 sdroxide 的。
settings-radio-tx-gain = TX 增益
settings-radio-negative-because-ad9361-expresses-transmit = 为负值，因为 AD9361 以衰减表示发射电平：0 dB 即为满功率输出。立即生效。在连接时应用此数值之前，发射机会先被设为最小，因此上一个程序留下的任何东西都不会处于激活状态。
settings-radio-reference-error-parts-per-million = 参考误差，单位为百万分之一。以 RUST_LOG=sdroxide_pluto=debug 运行，日志会在约 20 秒后打印测得的时钟误差——这就是应当输入的数值。由 sdroxide 应用，而不写入电台自身的持久微调。立即生效。
settings-radio-full-duplex = 全双工
settings-radio-keep-receiving-while-transmitting = 发射时继续接收
settings-radio-leave-off-pluto-reached-over =
    通过 USB 线缆连接的 Pluto 请保持关闭。该链路无法同时以每秒一兆采样的速率双向传输，而饿死发射缓冲的通话会被切断发出——因此默认情况下，在通话期间接收会停止。

    Turn it on for a board on real Ethernet (a LibreSDR, a Pluto on a gigabit adapter), where there is room for both: you then hear the receiver through your own transmission, which is how a QO-100 station listens to its own downlink.

    An over needs twice the sample rate's worth of link — 2.5 Msps is 10 MB/s each way — so lower the rate if the log starts saying the link is not carrying it. The panadapter still shows the transmitted signal during an over; it is the audio that keeps coming.

    Takes effect on Apply.
settings-radio-duplex = 双工
settings-radio-whether-ad9361-runs-both-directions =
    AD9361 是同时双向运行（FDD，即 Pluto 启动时的方式，也是 sdroxide 一直保持的状态），还是一次只运行一个方向（TDD）。

    Leave this on FDD unless you want the PTT pins below — TDD is what those key from, and it rules out Full duplex above. Takes effect on Apply.
settings-radio-ptt-pins = PTT 引脚
settings-radio-pluto-s-four-gpo-test =
    Pluto 的四个 GPO 测试点本身即可驱动外部功率放大器、LNA 或收发开关。选择一对引脚后，其中一个在接收期间始终为高电平，另一个在发射期间始终为高电平——无需主机软件参与，也无需接线的串口 PTT 线。

    This puts the radio in TDD whatever the Duplex row says, because the pins follow the AD9361's enable lines and in FDD both of those are asserted the entire session. Full duplex goes off with it, so leave both at their defaults if you work satellites and need to hear your own downlink.

    Analog Devices' own note puts an external LNA on GPO0/GPO1, so use GPO2/GPO3 if your board is wired that way. The pins are about 1.3 V at a few milliamps: drive a transistor or an opto-isolator with them, never a relay coil.

    Takes effect on Apply.
settings-radio-buffer-size = 缓冲大小
settings-radio-how-much-device-holds-before =
    设备在每次传输前保留多少复数采样。默认的 32768 在 2 Msps 下约为 16 毫秒：足够长，使每次传输的往返时间不会成为瓶颈；又足够短，使重新调谐不会明显迟滞。

    Halve it if the log reports the receive socket being replaced. A marginal link — a Pluto reached over its USB cable at a high sample rate is the usual one — stalls part-way through a transfer, and a smaller transfer is both less likely to be caught mid-flight and quicker to make good afterwards. Raise it to trade retune latency for fewer round trips.

    Takes effect on Apply.
settings-radio-samples = {" "}采样
settings-radio-rx-tx-port = RX / TX 端口
settings-radio-ad9361-s-rf-port-select = AD9361 的 rf_port_select。标准的 Pluto 每个方向各接好一个，因此除非你使用的板卡没有这样接线，否则请留空。在“应用”后生效。
settings-radio-opens-radio-reads-what-says = 打开电台，读取它对自身的说明，并报告此特定板卡的调谐范围。不启动数据流。
settings-radio-wideband-iq-receive-transmit-half = 宽带 IQ 接收与发射。半双工——在通话期间接收会停止，因为 USB 网络链路无法同时承载两者。无需 SoapySDR，也无需 libiio。
settings-radio-flexradio-announces-itself-local-network = FlexRadio 大约每秒在本地网络上宣告一次自身。通过路由器或 VPN 连接的电台永远不会向你广播——请改为在下方输入其地址。
settings-radio-another-gui-client-has-radio = 另一个图形界面客户端正在使用此电台，并且已禁用 multiFLEX。请断开该客户端的连接，或在电台上启用 multiFLEX。
settings-radio-overrides-selection-above-use-radio = 覆盖上面的选择。适用于位于另一子网、VPN 之后或使用非标准端口的电台。
settings-radio-width-spectrum-sdroxide-receives-192 = sdroxide 所接收频谱的宽度。192 kHz 是电台 DAX IQ 数据流的最大值，因此也是此界面能够显示的最宽跨度。
settings-radio-t-192-khz-widest-dax-iq = 192 kHz 是 FLEX 能够发送的最宽 DAX IQ 数据流，而电台会将全景图的跨度与之绑定——因此这就是整个跨度，此处没有任何设置能使它更宽。
settings-radio-dax-iq-channel = DAX IQ 通道
settings-radio-radio-has-four-change-only = 电台有四个通道。只有当网络上的其他设备已在使用通道 1 时才需更改——电台会两次拒绝同一通道。
settings-radio-station-name = 电台站点名称
settings-radio-shown-against-session-radio-s = 在电台的客户端列表中，与此会话对应显示。电台也以此名称记住客户端，因此重命名会使电台将 sdroxide 视为一个新的客户端。
settings-radio-gui-client-id = 图形界面客户端 ID
settings-radio-identity-radio-restores-session-s = 电台用来恢复此会话的切片和全景图的身份。留空时，它由电台站点名称派生——稳定，但在每个保留默认名称的 sdroxide 上都相同，而电台会通过踢掉较早的客户端来处理重复项。sdroxide 能在线路上察觉这一点，并回退到单次会话的身份，这会失去恢复功能；若要保留它，请在此处填入你自己的 UUID。
settings-radio-optional-e-g-uuid-your = 可选，例如你自己的 UUID
settings-radio-mirror-radio-s-i-q =
    以全景图的中心为轴镜像电台的 I/Q。默认关闭，FLEX-6600 即是以此方式验证的。

    Try it if receive audio is unintelligible on USB *and* on LSB and nothing decodes, while the waterfall looks entirely convincing — that is what a mirrored stream looks like, and it is the one fault with no other symptom (issue #368). If it is not that, turning this on makes it obvious rather than subtle. Applies on Apply / reconnect.
settings-radio-network-mtu = 网络 MTU
settings-radio-largest-datagram-radio-may-send = 电台可以发送的最大数据报。1450 是 SmartSDR 自身请求的数值。在 MTU 较小的路径上——VPN 或隧道——请将其调低，否则分片会被丢弃，根本收不到任何频谱。
settings-radio-b = {" "}B
settings-radio-checks-radio-answers-without-registering = 检查电台是否应答，但不注册为图形界面客户端——因此不会干扰已在运行的 SmartSDR 会话。
settings-radio-wideband-iq-receive-over-dax = 通过 DAX 进行宽带 IQ 接收，电台调制的音频发射。按下“应用 / 重新连接”即可无需重启地切换。
settings-radio-not-yet-verified-against-real-2 = 尚未在实际硬件上验证。若出现异常，请发送诊断报告——它包含与电台交换的每一行协议内容。
settings-radio-devices-soapysdr-can-see = SoapySDR 可识别的设备
settings-radio-waiting-machine-radio-attached-where = 正在等待电台所连接的机器，模块安装在那里。它打开哪个设备，取决于该机器 config.toml 中的 `device_args`，或其 --device 参数。
settings-radio-ask-every-installed-soapysdr-module = 要求每个已安装的 SoapySDR 模块进行扫描。不会打开任何设备，因此在接收时也是安全的——但可能需要一点时间。
settings-radio-not-enumerated-yet-press-rescan = 尚未枚举——请按“重新扫描”。
settings-radio-no-soapysdr-devices-found-check = 未找到 SoapySDR 设备。请检查你电台的模块是否已安装，以及你是否有权访问该设备。
settings-radio-not-radio = 不是电台
settings-radio-sound-card-listed-above-if = 上面有一个声卡被列为 SDR。它接受任何频率并忽略它，因此打开它时显示的是声卡的输入，而非频段。sdroxide 不会自动选择它们——但在 device_args 中指名它的那一行仍会被执行。
settings-radio-which-one-opens-set-device = 打开哪一个由 config.toml 中的 --device 或 device_args 设定；两者都没有时，列表中第一台电台生效。
settings-radio-no-device = 无设备
settings-radio-rig-has-no-software-adjustable = 此电台没有可由软件调节的增益。
settings-radio-rx-gains = RX 增益
settings-radio-tx-gains = TX 增益
settings-radio-antennas = 天线
settings-radio-remembered-next-start = 下次启动时记住。
settings-radio-these-controls-appear-once-soapysdr = 这些控件在 SoapySDR 设备打开后才会出现——它们是设备自身的控件，必须向设备查询。
settings-radio-stream = 数据流
settings-radio-follow-app-setting = 跟随应用设置
settings-radio-msps = 或以 Msps 计
settings-radio-baseband-filter = 基带滤波器
settings-radio-let-driver-choose = 由驱动程序选择
settings-radio-changing-either-reopens-radio = 更改其中任何一项都会重新打开电台。
settings-radio-driver-publishes-no-settings-own = 此驱动程序没有公布任何自身设置。
settings-radio-applied-once-remembered-next-start = 立即应用，并在下次启动时记住。
settings-radio-no-rx-888-found-press = 未找到 RX-888——请按“重新扫描”
settings-radio-adc-clock = ADC 时钟
settings-radio-t-129-6-msps-needs-superspeed = 129.6 Msps 需要 SuperSpeed 链路和高速主机；64.8 为安全的默认值。更改它会自动重新打开接收机，这需要一点时间，但无需重启。
settings-radio-panadapter-width = 全景图宽度
settings-radio-hf-only = {"  "}· 仅 HF
settings-radio-usable-both-front-ends = 两种前端均可使用。
settings-radio-below-vhf-crossover-ordinary-width = 低于 VHF 分界点时，这是普通的宽度。高于它时，调谐器的 8 MHz 中频无法居中于这样宽的窗口中，因此实时频谱会偏到全景图的一侧，而其旁边什么都没有。
settings-radio-dial-above-crossover-now-so = 表盘现在位于分界点之上，因此此宽度会将调谐器的 8 MHz 显示为偏离中心，而其旁边是空白的频谱。请缩窄宽度以填满全景图。
settings-radio-vga-gain = VGA 增益
settings-radio-ad8370-variable-gain-amplifier-ahead = ADC 之前的 AD8370 可变增益放大器。
settings-radio-attenuator = 衰减器
settings-radio-pe4304-step-attenuator-0-5 = PE4304 步进衰减器，以 0.5 dB 为步进。
settings-radio-adc-range = ADC 范围
settings-radio-wide-2-25-vp-p = 宽（2.25 Vp-p）
settings-radio-selects-adc-s-wider-input = 选择 ADC 更宽的输入范围：为强广播信号提供更多余量，但弱信号的计数更少。关闭则选择更灵敏的 1.5 Vp-p 范围。
settings-radio-dither = 抖动
settings-radio-adds-small-dither-signal-ahead = 在 ADC 之前加入一个小的抖动信号：以少量底噪为代价，换取无杂散动态范围。
settings-radio-randomiser = 随机化
settings-radio-adc-scrambles-output-so-digital = ADC 会打乱其输出，使数字总线不再向前端辐射；驱动程序会将其还原。除非在调试，否则请保持开启。在重新连接后生效。
settings-radio-dc-hf-antenna-port = HF 天线端口的直流
settings-radio-powers-active-antenna-preamp-down = 通过同轴电缆为有源天线或前置放大器供电。
settings-radio-vhf-tuner-gain = VHF 调谐器增益
settings-radio-r828d-rf-gain-used-above = R828D 射频增益，在自动 HF/VHF 分界点之上使用。29 个离散步进；取最接近的一档。
settings-radio-auto = 自动
settings-radio-let-tuner-run-own-lna = 让调谐器运行其自身的 LNA 和混频器环路。
settings-radio-bias-tee-vhf = 偏置三通（VHF）
settings-radio-dc-vhf-antenna-port = VHF 天线端口的直流
settings-radio-clock-trim = 时钟微调
settings-radio-corrects-reference-oscillator-applied-when = 校正参考振荡器。在你松开数值时应用——若在拖动的每个像素上都重新打开，接收机就会被重启数百次。
settings-radio-receive-only-below-adc-s = 仅接收。在 ADC 的奈奎斯特极限以下，天线被直接采样，重新调谐是即时的，因为没有硬件下变频器——完整的 ADC 数据流由主机转换为基带。在其以上，接收机会自动切换到其 R828D 调谐器和 VHF SMA，因此两根天线都需要接好。VHF 需要 32.4 Msps 或更高的 ADC 时钟，才能使调谐器的中频放得下；在低于 48 Msps 的时钟下，两个范围之间存在无法覆盖的间隙。此处的每项设置都会立即生效——没有需要按下的“应用”按钮。
settings-radio-device = 设备
settings-radio-no-devices-press-rescan = 没有设备——请按“重新扫描”
settings-radio-six-rates-are-six-different =
    这六种速率是六种不同的 FPGA 镜像，而不是一个寄存器——这就是为什么 ELAD 的协议中没有任何东西可以在它们之间进行选择，也是为什么此设置的含义取决于型号。

    FDM-S1 / FDM-S2: a command. Their FPGA is loaded by the computer at every power-up, and sdroxide runs ELAD's own elad-firmware loader to put this rate's image in. Without that loader installed a sampler sends no samples at all, and sdroxide says so on screen.

    FDM-DUO: not a command. The radio boots its own FPGA and arrives at whatever rate it powered up in or FDM-SW2 last left it in (192 kHz on a fresh one), and this only says how the stream is READ. Get it wrong and the panadapter is simply the wrong width, with every frequency inside it scaled to match.

    The stream's real rate is measured a couple of seconds after it starts, and a mismatch is reported on screen. Takes effect on Apply.
settings-radio-fdm-s-fpga-image-loaded = FDM-S：加载的 FPGA 镜像——FDM-DUO：数据流的读取方式
settings-radio-receiver-s-input-pad-one = 接收机的输入衰减器——一步，加入或取消。与主窗口的增益滑块为同一控件。
settings-radio-pre-selection-filters = 预选滤波器
settings-radio-low-pass-bank-front-adc = ADC 之前的低通滤波器组。将其旁路可获得最宽的视野，但在强带外信号附近表现最差——几英里外的广播发射机会在频段上产生镜像。除非你有意收听滤波范围之外的内容，否则请保持开启。
settings-radio-filters-circuit = 电路中的滤波器
settings-radio-rig-control-fdm-duo-only = 电台控制——仅 FDM-DUO
settings-radio-transceiver-s-cat-usb-port = 收发信机的 CAT USB 端口，它与接收线缆是分开的线缆。在 FDM-S1 或 FDM-S2 上请保持端口为未设置——它们没有此端口；而在 FDM-DUO 上，你宁可仅通过其接收线缆进行操控：此时仍可调谐和键控，但无法读取任何计量。
settings-radio-port-set-radio-s-display = 设置了端口后，电台的显示与此处的表盘在两个方向上都是相同的数字：FDM-DUO 的接收窗口是围绕其 VFO 构建的，因此在此调谐会移动前面板，而转动前面板旋钮则会移动此处的读数。全景图会随表盘移动而重新居中。若没有端口——或电台不应答的端口——则没有可供命令的 VFO，调谐会停留在电台已在发送的窗口之内。
settings-radio-none-usb-control-only = — 无（仅 USB 控制） —
settings-radio-must-match-menu-70-cat = 必须与电台上的菜单 70“CAT BAUD”一致，出厂设置为 38400。电台只有这四种速率，没有其他，而以任何其他速率打开的端口在两个方向上都是静默的——电台忽略表盘并拒绝键控，且不会说明原因。
settings-radio-how-transmit-keyed-cat-sends = 发射是如何被按下的。“CAT”发送电台自身的 TX 命令，无需在电台上进行任何设置；RTS 则需要将菜单 54“PTT”设为 PTT+RTS。
settings-radio-where-radio-takes-transmit-audio-2 =
    电台从何处获取其发射音频——即电台的 TI 命令，它在前面板的菜单 32“TX IN”中。端口打开时设置。

    "USB audio" is what makes transmit work here: the FDM-DUO sends what sdroxide puts into its USB sound card. A radio left on "Microphone" sends the room instead, and nothing on screen says so.
settings-radio-which-two-sockets-back-receiver-2 =
    接收机监听背面两个插座中的哪一个——即电台的 AN 命令，在前面板的菜单 31“ANTENNAS”中，并在其显示屏上以“ANT 1 2”指示。

    "RTX" is one antenna doing both jobs, on the M-type socket that also carries transmit. "RX only" moves receive to the second socket and leaves transmit on RTX — a receiving antenna, a loop or a beverage, with the beam still on the transmitter.

    It moves this whole receiver: the panadapter, the demodulators and the rig's own audio all come from the socket selected here.

    Applies immediately, and is read back from the radio when the control port opens — this is the rig's own setting, not a copy of it kept here.
settings-radio-how-often-radio-asked-what-2 = 多久询问一次电台正在做什么——它的表盘、模式和表头。转动电台自己的 VFO 旋钮，读数会在一次轮询之内跟随。这也是此端产生的全部控制通信量，因此如果电台的音频断断续续，请调低它。
settings-radio-transmit-audio = 发射音频
settings-radio-only-receiving-fdm-duo-has = 迄今为止，只有 FDM-DUO 上的接收功能在实际硬件上运行过。若出现任何异常，请将诊断报告附在错误报告中。
settings-radio-cw-keyed-radio-s-own = CW 由电台自己的键或电键按下：FDM-DUO 没有接受文字的命令，因此 CW 面板无法对其键控。
settings-radio-press-apply-reopen-radio = 按“应用”以重新打开电台。
settings-radio-no-receivers-press-rescan = 没有接收机——请按“重新扫描”
settings-radio-which-rates-receiver-has-depends = 接收机具有哪些速率取决于型号和固件。在“应用”后生效。
settings-radio-every-rate-any-hf-model = 任何 HF+ 型号提供的每一种速率——连接一台即可查看其自身的速率
settings-radio-receiver-s-own-gain-control = 接收机自身的增益控制。普通收听时请保持开启；若要为测量而手动设置衰减器，请将其关闭。
settings-radio-agc-threshold = AGC 阈值
settings-radio-high = 高
settings-radio-high-trades-little-sensitivity-headroom = 高阈值会以少量灵敏度换取对强邻台的余量——在夜间拥挤的频段上，这是正确的设置。
settings-radio-front-end-attenuation-gain-0 = 前端衰减，以增益表示——0 dB 即无衰减。仅在 AGC 关闭时生效。
settings-radio-preamp = 前置放大器
settings-radio-hf-low-noise-amplifier-buys = HF 低噪声放大器。以互调为代价换取灵敏度，因此默认关闭——在真实天线上通常是正确的。
settings-radio-lna = LNA 开
settings-radio-frequency-calibration = 频率校准
settings-radio-parts-per-billion-receiver-s = 十亿分之一——这是此接收机自己的单位，比 RTL-SDR 使用的 ppm 精细一千倍。此处的任何内容都永远不会写入接收机的闪存：它只在本次会话中覆盖已存储的数值。
settings-radio-use-receiver-s-stored-value = 使用接收机已存储的数值
settings-radio-ppb = {" "}ppb
settings-radio-feed-dc-up-coax = 通过同轴电缆供直流
settings-radio-not-every-hf-has-one = 并非每台 HF+ 都有此功能；在没有它的接收机上，此项不起任何作用。
settings-radio-host-dsp = 主机 DSP
settings-radio-image-balancer-zero-if-offset = 镜像平衡器、零中频偏移以及微调振荡器。只有在需要查看原始硬件输出时才将其关闭——关闭后，镜像会在零中频速率上出现，而表盘仅能精确到最接近的千赫兹。
settings-radio-correct-image-fine-tune = 校正镜像并微调
settings-radio-receive-only-no-soapysdr-no = 仅接收。无需 SoapySDR，也无需 libairspyhf。在合成器的下限以下，由主机振荡器完成调谐，这正是此接收机能够到达 VLF 的方式。接收机和采样率在“应用”后生效；其他设置在你更改时即生效。
settings-radio-not-yet-verified-against-real-3 = 尚未在实际硬件上验证。若出现异常，请发送诊断报告——它包含与接收机交换的每一条命令，以及采样数据流前几个字节解码为 I/Q 对的结果。
settings-radio-no-airspy-r2-mini-found = 未找到 Airspy R2 或 Mini——请按“重新扫描”
settings-radio-read-receiver = 从此接收机读取
settings-radio-r2-mini-offer-different-rates = R2 和 Mini 提供的速率不同，在打开其中一台之前无法区分
settings-radio-rate-you-get-receiver-s = 这是你将得到的速率。接收机的 ADC 以其两倍的速率运行——它对真实信号进行数字化，然后由 sdroxide 在主机上从中生成复数基带。
settings-radio-gain-curve = 增益曲线
settings-radio-step-along-curve-above-not = 沿上面曲线的一个步进，而不是分贝数——调谐器的 LNA、混频器和 VGA 会一起移动，每一步的价值取决于曲线和频段。0 为最安静的一端。
settings-radio-tuner-agc = 调谐器 AGC
settings-radio-lna-2 = LNA
settings-radio-hands-lna-tuner-s-own = 将 LNA 交给调谐器自身的环路控制，覆盖曲线设置。
settings-radio-mixer = 混频器
settings-radio-same-mixer-stage = 混频级同理。
settings-radio-loop-running-gain-slider-no = 环路运行时，增益滑块不再设置它所控制的级——环路会覆盖它。
settings-radio-dc-antenna-port = 天线端口的直流
settings-radio-t-12-bit-packing = 12 位打包
settings-radio-third-less-usb-traffic-leave = USB 流量减少三分之一。请保持开启：这是一台 USB 2.0 设备，最高速率为打包时 30 MB/s、未打包时 40 MB/s。在重新连接后生效。
settings-radio-dc-removal = 直流去除
settings-radio-remove-adc-s-offset = 去除 ADC 的偏移
settings-radio-turn-off-see-raw-hardware = 关闭它即可看到原始硬件输出。值得了解杂散的位置：偏移落在跨度的边缘而非中心，因为信号在途中被平移了采样率的四分之一。
settings-radio-every-command-exchanged-receiver-sample = 与接收机交换的每一条命令、采样率的计算过程，以及解码为 I/Q 的前几个采样。
settings-radio-receive-only-24-1800-mhz = 仅接收，24–1800 MHz。无需 SoapySDR，也无需 libairspy。接收机、采样率和打包在“应用”后生效；其他设置在你更改时即生效。
settings-radio-not-yet-verified-against-real-4 = 尚未在实际硬件上验证。若出现异常，请发送诊断报告——它包含与接收机交换的每一条命令、速率计算过程，以及解码为 I/Q 对的前几个采样。
settings-radio-no-hydrasdr-rfone-found-press = 未找到 HydraSDR RFOne——请按“重新扫描”
settings-radio-these-are-rates-receiver-turned = 这些就是该接收机结果所具有的速率
settings-radio-receiver-only-reports-three-these = 接收机只报告其中三种；其余的位于固件的备用表中，较旧的版本可能不包含它们
settings-radio-rf-input = 射频输入
settings-radio-antenna-sma-only-socket-bias = 天线 SMA——唯一后接偏置三通的插座。
settings-radio-cable-socket-no-bias-tee = 线缆插座。这里没有偏置三通；硬件只把它接在 ANT 上。
settings-radio-powers-active-antenna-preamp-down-2 = 通过同轴电缆为有源天线或前置放大器供电。只有 ANT 插座具有此功能——两个线缆端口是普通输入。
settings-radio-third-less-usb-traffic-leave-2 = USB 流量减少三分之一。请保持开启：这是一台 USB 2.0 设备，最高速率为打包时 36 MB/s、未打包时 48 MB/s。在重新连接后生效。
settings-radio-one-these-boards-1d50-60a1 = 其中一种板卡使用 1d50:60a1，这是 HydraSDR 原型机与 Airspy R2 和 Mini 共用的 USB 标识。sdroxide 会在打开后检查固件，若选错了接口（无论哪个方向），都会提示。
settings-radio-receive-only-24-1800-mhz-2 = 仅接收，24–1800 MHz。无需 SoapySDR，也无需 libhydrasdr。接收机、采样率和打包在“应用”后生效；其他设置在你更改时即生效。
settings-radio-re-list-fobos-sdrs-no = 重新列出 Fobos SDR。不会打开任何设备，因此在接收时按下也是安全的。
settings-radio-no-fobos-sdr-found-press = 未找到 Fobos SDR——请按“重新扫描”
settings-radio-input-2 = 输入
settings-radio-through-tuner-lna-vga-mixer = 经由调谐器：LNA、VGA，以及混频器自身的频率范围。
settings-radio-direct-sampling-hf1-antenna-input = 在 HF1 天线输入上进行直接采样——无调谐器，无 LNA/VGA 增益，完全由软件调谐。
settings-radio-direct-sampling-hf2-antenna-input = 在 HF2 天线输入上进行直接采样，与 HF1 相同。
settings-radio-both-real-adc-channels-once = 两个真实 ADC 通道同时工作，由下面的分集滤波器合并——用于将噪声源置零，或合并两条衰落路径。
settings-radio-direct-sampling-tuner-so-lna = 直接采样——调谐器（因此 LNA/VGA 增益）不在通路中。下面的速率是软件下变频器的目标，而非硬件设置，因此实际达到的速率可能与之不同。
settings-radio-receiver-s-own-reported-rates = 此接收机自己报告的速率
settings-radio-measured-one-real-unit-connect = 在一台实际设备上测得——连接此接收机并按“重新扫描”即可查看它自己的列表
settings-radio-clock-source = 时钟源
settings-radio-external-reference = 外部参考
settings-radio-diversity-taps = 分集抽头
settings-radio-taps = {" "}抽头
settings-radio-one-tap-gain-phase-null = 一个抽头是一个增益和一个相位——在一个频率上的零点，其两侧都会变差，这正是模拟移相器所能做到的全部。每增加一个抽头，滤波器就能均衡 HF1 与 HF2 之间路径差的一个采样周期，正是它把那个陷波变成一整段都安静的频段。
settings-radio-receive-only-rf-port-tuner = 仅接收。射频端口（调谐器）、HF1/HF2（直接采样，在软件中解码）以及 HF1+HF2（两者同时工作，由分集滤波器合并）均可使用。接收机、输入和采样率在“应用”后生效；增益、时钟源和抽头在你更改时即生效——选中此电台后，分集滤波器的模式、适应速率和保持设置位于主窗口自己的 DIV 框中。
settings-radio-no-hackrf-found-press-rescan = 未找到 HackRF——请按“重新扫描”
settings-radio-front-end-amplifier-8-db = 前端放大器，以 8 dB 为步进。这是改变灵敏度的一级，也是在真实天线上最先过载的一级。
settings-radio-baseband-amplifier-after-mixer-2 = 混频器之后的基带放大器，以 2 dB 为步进。遇到弱信号时，先调高它，再去动 LNA。
settings-radio-rf-amp = 射频放大
settings-radio-one-switch-shared-transmit-setting = 一个开关，与下面的发射设置共用——电台会应用其进入的方向所对应的那一项。在真实天线上，通常应保持关闭。
settings-radio-hackrf-pro-chooses-itself-three = HackRF Pro 会自行选择此项——即采样率的四分之三，并在 FPGA 中滤波——并忽略主机请求的任何设置。
settings-radio-leave-automatic-choosing-one-too = 保持为“自动”。选择过窄的一项不仅会使频段边缘变软——还会撤销使直流尖峰远离你的信号的本振偏移。
settings-radio-about-3-v-50-ma = 通过同轴电缆供给约 3 V、50 mA，用于有源天线或前置放大器。仅限 HackRF One 或 Pro；Jawbreaker 和 rad1o 没有此电路。
settings-radio-remove-dc-mirror-image = 去除直流和镜像
settings-radio-zero-if-radio-so-own = 这是一台零中频电台，因此其自身的本振泄漏位于跨度中部，而混频器的正交误差会在其上产生镜像。关闭此项即可看到原始硬件输出，这是区分驱动程序问题与 DSP 问题的快捷方法。
settings-radio-corrects-reference-oscillator = 校正参考振荡器。
settings-radio-transmit = 发射
settings-radio-transmitter = 发射机
settings-radio-off-default-while-off-backend = 默认关闭。在此关闭期间，后端不会发布任何发射通道，因此无法键控电台。在重新连接后生效。
settings-radio-tx-vga-gain = 发射 VGA 增益
settings-radio-transmit-driver-amplifier-drive-applied = 发射驱动放大器。驱动在此级之前以数字方式施加，因此请保持驱动较高，并在此处设置输出电平——反之，将驱动调低会使 DAC 以满量程的一小部分运行，并增加互调。
settings-radio-same-switch-receive-setting-above = 与上面接收设置相同的开关，在电台切换方向时应用。
settings-radio-transmit-armed-into-dummy-load = 发射已就绪。在完成测量之前，请接入假负载：HackRF 的谐波很强，需要为你所在的频段配备外部低通滤波器，而且它是半双工，因此每次通话期间接收都会停止。
settings-radio-every-command-exchanged-radio-session = 本次会话中与电台交换的每一条命令，按顺序排列，包括每次按下发射前后的序列。
settings-radio-dc-7-25-ghz-half = 直流 – 7.25 GHz，半双工。无需 SoapySDR、libusb 或 libhackrf。电台和采样率在“应用”后生效；其他设置在你更改时即生效。频段边缘由固件决定：HackRF 的规格为 1 MHz – 6 GHz（Pro 为 100 kHz），在此范围之外会被大幅衰减，但它确实可以调谐到那里。
settings-radio-ask-sdrplay-api-service-device = 向 SDRplay API 服务查询其设备列表。不会打开任何设备，因此在接收时按下也是安全的。
settings-radio-no-rsps-press-rescan-needs = 没有 RSP——请按“重新扫描”（需要 SDRplay API 服务）
settings-radio-both-tuners-running-api-fixes = 当两个调谐器都运行时，API 将 ADC 固定在 6 MHz，并从低中频返回 2 Msps——因此 2 Msps 是最宽的跨度，较窄的跨度则是对它进行抽取得到的。在“应用”后生效。
settings-radio-rates-below-2-msps-run = 低于 2 Msps 的速率会让 ADC 以 2 Msps 运行，并在服务中进行抽取。在“应用”后生效。
settings-radio-reduced-adc-resolution = {"  "}（降低的 ADC 分辨率）
settings-radio-if-bandwidth = 中频带宽
settings-radio-tuner-s-analog-filter-auto = 调谐器的模拟滤波器。“自动”会选择适合该采样率的最宽的一个。在“应用”后生效。
settings-radio-rsp-s-own-if-gain = RSP 自身的中频增益环路，由 API 服务运行。关闭则把中频增益滑块交还给你——这是用于测量和弱信号数字模式的设置。
settings-radio-agc-set-point = AGC 设定点
settings-radio-signal-level-loop-holds-adc = 环路将 ADC 保持在的信号电平。调低会为频道外的信号留出更多余量。它能降到多低取决于采样率——高于 8.064 Msps 时，变换器能提供的余量更少，高于 9.216 Msps 则更少。
settings-radio-rsp-s-native-gain-unit = RSP 的原生增益单位：20 dB 为最大增益（使用下面的扩展范围时为 0），59 dB 为最小。立即生效。AGC 运行时忽略此项——此时该数值由环路掌控，S 表显示的是它最终稳定的结果。
settings-radio-extended-if-range = 扩展中频范围
settings-radio-lets-if-gain-reduction-go = 允许中频增益衰减低于 20 dB，直至 0——即接收机最后的 20 dB 增益。关闭为 API 自身的默认值，也是普通收听的正确选择，因为该范围的底端正是 RSP 最容易过载的地方。在 LNA 已经为 0 的弱信号情况下，值得开启它。它还允许 AGC 设定点升至 0 dBFS。立即生效。
settings-radio-allow-if-gain-reduction-below = 允许中频增益衰减低于 20 dB
settings-radio-front-end-attenuation-steps-0 = 分步的前端衰减：0 为最大增益，每向上一步就多切入一级衰减。某些频段的步进较少——驱动程序会将其钳位，并保留你的选择，供你调回时使用；此处的滑轨止于电台所在频段所能接受的最大值。在另一个频段上做出的更高选择会被保留，并仍显示在其旁边。立即生效。
settings-radio-t-0-max-gain = 0 = 最大增益
settings-radio-reference-error-parts-per-million-2 = 参考误差，单位为百万分之一，由设备自身应用。立即生效。
settings-radio-radio-s-tuner = 此电台的调谐器
settings-radio-tuner = 调谐器
settings-radio-which-rspduo-s-two-tuners = 此电台收听 RSPduo 两个调谐器中的哪一个。另一个属于为它配置的那台电台——请给那一台设置相同的接收机和另一个调谐器。在“应用”后生效。
settings-radio-which-rspduo-s-two-tuners-2 = RSPduo 两个调谐器中，哪一个承载你正在收听的天线。另一个承载第二根天线。在“应用”后生效。
settings-radio-which-rspduo-s-two-tuners-3 = 运行 RSPduo 两个调谐器中的哪一个（一次一个）。在“应用”后生效。
settings-radio-applies-immediately = 立即生效。
settings-radio-dab-notch = DAB 陷波
settings-radio-hardware-notch-over-165-230 = 针对 165–230 MHz DAB 频段的硬件陷波。立即生效。
settings-radio-hdr-mode = HDR 模式
settings-radio-enable-below-2-mhz = 在 2 MHz 以下启用
settings-radio-rspdx-s-high-dynamic-range =
    RSPdx 用于 LF/MF 的高动态范围通路。立即生效。

    It does not work yet: on the one RSPdx it has been tried against, switching it on silences the receiver — at every centre, filter, port, gain and rate tried. Whatever the mode needs is not on the documented API surface, and finding it is somebody's next job. Leave this off unless you are the one looking.

    What is known: it is not a mode that follows the dial. The path has a fixed analog filter, built only at the centres listed under the filter below, so tuning anywhere else could not work even once the rest does.
settings-radio-hdr-filter = HDR 滤波器
settings-radio-feed-4-7-v-dc = 通过同轴电缆供给约 4.7 V 直流
settings-radio-second-tuner = 第二个调谐器
settings-radio-rspduo-two-whole-tuners-one = RSPduo 是同一块板卡上的两个完整调谐器，由同一个参考时钟驱动。两者同时运行时，它们会在同一时刻、以同一时钟收听各自的跨度——这使两根天线之间保持相干，其相对相位由馈线决定而非偶然，并使这一对能够被合并。或者让它们各自独立：两个调谐器分别调谐，因此另一个可以在它自己的频段上作为第二台电台。
settings-radio-run-both-tuners = 运行两个调谐器
settings-radio-puts-rspduo-api-s-dual = 将 RSPduo 置于 API 的双调谐器模式，此时 ADC 固定在 6 MHz，最宽跨度为 2 Msps。在“应用”后生效，因为该模式是在设备打开时选定的。
settings-radio-used = 用于
settings-radio-what-other-tuner-takes-effect = 另一个调谐器用于什么。在“应用”后生效——与同时运行两者一样，这是在板卡打开时选定的。
settings-radio-add-second-radio-settings-radio = 添加第二台电台（设置 → 电台 → +），赋予它这同一个接收机以及 RSPduo 的另一个调谐器，并同样将其设为运行两个调谐器——无论哪台电台打开该板卡，都会将其置于双调谐器模式，因此两者都必须预期这一点。随后两者可以独立调谐：一个选项卡处理 HF，另一个处理 VHF，均来自同一块板卡。它们共用一个 ADC 时钟，因此无论打开板卡的是哪台电台所请求的采样率，两者都以该采样率运行，而且两者都不能发射。
settings-radio-what-do = 用途
settings-radio-lna-state-2 = 其 LNA 状态
settings-radio-second-tuner-s-own-front = 第二个调谐器自己的前端衰减。设置为使两根天线显示大致相同的底噪：整个系统都依赖于此项调整，因为合并时会按各分支的噪声为其加权；若第二个前端被驱动至过载，就会给滤波器一份失真的干扰副本——而失真的副本无法从未失真的副本中减去。立即生效。
settings-radio-if-gain-reduction-2 = 其中频增益衰减
settings-radio-second-tuner-s-if-gain = 第二个调谐器的中频增益，以 RSP 的原生单位表示：20 dB 为最大增益。AGC 运行时忽略此项——而滤波器需要的是稳定的增益，因此若要保持一个陷波，关闭 AGC 是值得的。
settings-radio-filter-length = 滤波器长度
settings-radio-one-tap-gain-phase-null-2 = 一个抽头是一个增益和一个相位——在一个频率上的零点，其两侧都会变差，这正是模拟移相器所能做到的全部。每增加一个抽头，滤波器就能均衡两根天线之间路径差的一个采样周期，正是它把那个陷波变成一整段都安静的频段。
settings-radio-adaptation-2 = 适应
settings-radio-slow-steady-left-converging-inside = 在左侧缓慢而稳定，在不到一秒内收敛；在右侧则明显地来回搜寻。先快速找到零点，再将其保持。
settings-radio-stop-filter-moving-reach-moment = 让滤波器停止移动。一旦出现零点就应使用它：若滤波器持续自适应，它会重新对准变得最响的任何东西，而在安静的频段上，那往往就是你正在收听的电台。
settings-radio-restart = 重新开始
settings-radio-zero-filter-find-null-again = 将滤波器归零，并重新找到零点。
settings-radio-nothing-here-can-tell-wanted = 这里没有任何东西能区分所需信号与不需要的信号——滤波器只知道两根天线的共同之处。在“消除”模式下，第二根天线应当只听到噪声源，并尽可能少地听到频段中的其他内容，否则它会尽职地把电台也消除掉。在“合并”模式下，两者都应听到同一个电台。零点有多深，每隔几秒便会写入日志。
settings-radio-not-yet-verified-against-rspduo = 尚未在 RSPduo 上验证：此处的双调谐器操作是依据 SDRplay 的 API 编写的，而非在硬件上测量的。
settings-radio-no-receiver-listed-check-against = 没有列出可用于核对的接收机——请按“重新扫描”，或阅读日志以了解实际打开的是哪一台。
settings-radio-receive-only-1-khz-2 = 仅接收，1 kHz–2 GHz。需要厂商的 SDRplay API 服务（sdrplay.com/api）——原始 RSP1 之后的 RSP 没有公开协议。设备、采样率、带宽和 RSPduo 调谐器在“应用”后生效；其他设置在你更改时即生效。
settings-radio-board = 板卡
settings-radio-first-one-found-2 = 找到的第一个
settings-radio-receive-chain = 接收通道
settings-radio-both-chains-tune-together-they = 两个通道一同调谐——它们共用一个合成器——但它们是位于不同插座上的独立前端。请选择你的天线所接的那一个；当其中一个通道做过 HF 匹配改装而另一个仍为原装时，这就是需要用到的设置。
settings-radio-receive-gain = 接收增益
settings-radio-one-combined-figure-which-limesuite = 一个合并的数值，由 LimeSuite 自己分配到 LNA、TIA 和 PGA 上。它只接受整数分贝，更细的数值会被截断。
settings-radio-receive-port = 接收端口
settings-radio-automatic-follows-frequency-lnal-low = “自动”会跟随频率：LNAL 在低频、LNAH 在高频——除非下方连接了 LimeRFE，它通过一根线缆接入一个插座，此时在所有频率上都是 LNAW。若你的插座不是这一个，请指明它接线所用的插座。该插座属于哪一个通道，由上面的选择器决定。
settings-radio-t-0-follows-sample-rate-worth = 0 跟随采样率。最好保持在那里：比跨度四分之一更窄的滤波器会悄悄撤销零中频本振偏移，使本振泄漏重新出现在你正在收听的信号之上。
settings-radio-following-rate = 跟随速率
settings-radio-corrections = 校正
settings-radio-host-iq-dc-correction = 主机 IQ / 直流校正
settings-radio-adaptive-image-dc-removal-side = 在本端进行自适应镜像和直流去除，叠加在芯片自身的校准之上。关闭它是一键区分驱动程序问题与 DSP 问题的方法。
settings-radio-calibrate-automatically = 自动校准
settings-radio-runs-chip-s-own-dc = 在电台打开时运行芯片自身的直流偏移和镜像校准，并在表盘于新频段或不同插座上稳定后再次运行。这些数值是在一个频率上测得的，在其他地方并不准确——而位于跨度中部的载波通常就是这种情况。每次约需一秒。
settings-radio-calibrate-now = 立即校准
settings-radio-stalls-receiver-better-part-second = 使接收机停顿将近一秒。
settings-radio-second-aerial = 第二根天线
settings-radio-socket = 其插座
settings-radio-same-first = 与第一个相同
settings-radio-gain-2 = 其增益
settings-radio-set-low-coupled-sample-your = 请将此项设为低。你自己发射机的耦合采样是一个强信号，而被驱动至压缩的反馈通道会错误地测量功放曲线——它会让校正学习到它自己的失真。请从最低处开始，并使用耦合器的衰减器。
settings-radio-set-so-both-aerials-show = 设置为使两根天线显示大致相同的底噪。整个系统都依赖于此调整：合并时会按各分支的噪声为其加权；若第二个通道被驱动至压缩，就会给滤波器一份失真的干扰副本——而失真的副本无法从未失真的副本中减去。
settings-radio-table-steps-2 = 表格步数
settings-radio-how-finely-correction-follows-amplifier = 校正对功放曲线的跟随精细程度。步数越多越能追踪更陡的拐点，但每一步都必须从落入其中的采样中学习——而语音幅度直方图的顶端很稀疏。对于 HF 功放实际具有的平滑曲线，三十二步已足够。更改它会重新开始校正。
settings-radio-how-hard-each-block-feedback = 每一块反馈对校正的推动力度。功放的曲线不会改变，因此无需急于求成——取中间值可把若干次通话的噪声平均掉。
settings-radio-keep-correction-curve-learned-clean = 保持校正不变。在一次干净的通话中学到的曲线值得保留，而到下一次通话时功放也不会有所改变。
settings-radio-forget-correction-learn-again = 忘记校正，重新学习。
settings-radio-directional-coupler-amplifier-s-output = 功放输出端的定向耦合器接入此通道，发射机会比较返回的信号与其本意发送的信号，然后发送该差值的反相——因此离开功放的信号是纯净的。对他人的通联，互调减少二十余分贝，而功放的功率保持不变。在反馈与发射对齐之前，校正保持为单位增益，因此未连接的耦合器不会造成任何损失；并且它永远不会要求变换器超过满量程，因此读数异常的反馈通路也不会过驱动任何东西。它的进展情况会在你发射时写入日志。
settings-radio-transmit-not-armed-so-there = 发射未就绪，因此此处没有需要校正的内容。
settings-radio-off-interface-publishes-no-transmit = 关闭时，接口不会发布任何发射通道，因此无法键控电台。
settings-radio-transmit-gain = 发射增益
settings-radio-transmit-port = 发射端口
settings-radio-board-s-two-transmit-sockets = 板卡的两个发射插座是两个不同的匹配网络，而不是接到同一网络上的两个插孔：BAND1（TX_1）覆盖 30 MHz 至 1.9 GHz，BAND2（TX_2）是 13 cm 端口。若从 BAND2 发射 23 cm 以下的业余频段，通话会进入一个几乎不让任何能量通过的匹配网络——每个设置都显示正常，驱动为你所设的值，而功率计则保持为零。将此项保持为“自动”，它便会跟随表盘。
settings-radio-limesdr-transmits-about-100-khz = LimeSDR 从约 100 kHz 发射至 3.8 GHz，自身不做任何滤波。请使用低通滤波器、合适的 LimeRFE 通道或假负载。
settings-radio-limerfe-front-end = LimeRFE 前端
settings-radio-connected = 连接方式
settings-radio-limerfe-s-own-micro-usb = LimeRFE 自身的 micro-USB 端口，而非电台的。9600 波特，由其固件固定。
settings-radio-receive-connector = 接收连接器
settings-radio-transmit-connector = 发射连接器
settings-radio-follow-dial = 跟随表盘
settings-radio-switch-filters-match-operating-frequency = 在任何射频出现之前，将滤波器切换为与工作频率相匹配。在同一频段内调谐，不会在控制链路上发送任何内容。
settings-radio-relays = 继电器
settings-radio-receive-attenuator = 接收衰减器
settings-radio-notch-filter = 陷波滤波器
settings-radio-fan = 风扇
settings-radio-worth-having-any-sustained-transmitting = 任何持续发射时都值得开启。
settings-radio-automatic-board-receives-switched-transmit = 在“自动”下，板卡处于接收状态，并在按下发射时切换为发射，松开时切回——无论采用哪种接线。其业余频道在两侧各有一个收发开关，夹着一个滤波器，因此若同时要求板卡两个方向，该开关会被接到发射机上，而接收将收不到任何东西。30 MHz 频道只能通过 J5 访问，这是一个同时用于两个方向的连接器；在它以上，从 J4 发射可使接收通路远离功放正在驱动的连接器。
settings-radio-limesdr-support-has-not-been = LimeSDR 支持尚未在硬件上验证。这是上一次会话的跟踪记录，用于问题报告。
settings-radio-gains-filters-corrections-antenna-sockets = 增益、滤波器、校正、天线插座以及每一项 LimeRFE 控件——其连接器、频段、继电器、衰减器、陷波和风扇——均立即生效。板卡、接收通道、采样率、发射就绪以及 LimeRFE 的连接在“应用”后生效。

## Settings window, Radio tab: messages with values

settings-radio-fill-from = 从 { $name } 填充
settings-radio-decimation-auto = 自动（最接近的 { $khz } kHz）
settings-radio-decimation-stage-server = 第 { $stage } 级——服务器速率 ÷ { $divisor }
settings-radio-decimation-stage = 第 { $stage } 级——速率 ÷ { $divisor }
settings-radio-kiwi-receive-only = 仅接收——这是别人的天线。全景图是接收机以 I/Q 发送的约 12 kHz，它跟随表盘；其上方的频段视图是它自己的瀑布图，在其上调谐即会重新调谐接收机。连接期间约 { $rate } kB/s。按下“应用 / 重新连接”即可无需重启地切换。
settings-radio-connected-detail = 已连接：{ $detail }
settings-radio-failed = 失败：{ $error }
settings-radio-pluto-buffer = ≈ { $ms } ms，每次传输 { $kib } KiB
settings-radio-driver-settings = { $driver } 设置
settings-radio-rx888-coverage = 0–{ $mhz } MHz 覆盖范围，通过 USB 为 { $mbytes } MB/s
settings-radio-rx888-digitised = 在 { $mhz } MHz 数字化范围中
settings-radio-elad-16-bit = { $khz } kHz（16 位采样）
settings-radio-elad-pad = 输入 { $db } dB 衰减
settings-radio-hackrf-rx-preamp = 接收时 { $db } dB 前置放大
settings-radio-hackrf-tx-preamp = 发射时 { $db } dB 前置放大
settings-radio-device-not-found = { $device }（未找到）
settings-radio-lime-chain = 通道 { $n }（RX{ $n }_*）
settings-radio-lime-chain-tx = 通道 { $n }（RX{ $n }_* / TX{ $n }_*）
settings-radio-lime-second-chain = 板卡的另一条接收通道，位于 { $sockets } 插座上。它与第一条共用合成器，因此在同一瞬间收听相同的跨度——这正是它能承载第二根天线，或你自己发射机采样的原因。
settings-radio-tx-blocked = 发射被阻止：{ $reason }
settings-radio-rfe-rx-fallback = 在 { $port } 上接收时，这些频段会回退到未滤波的宽带通路：{ $bands }。
settings-radio-rfe-tx-fallback = 在 { $port } 上发射时，这些频段会回退到宽带通路——无频段功放，也无滤波：{ $bands }。

## Top bar and control strip (src/app/top_bar.rs)

topbar-volume-gain-agc-squelch-noise = 音量、增益、AGC、静噪、噪声控件，以及调频广播上的 RDS 读数
topbar-receiver = 接收机
topbar-vfo-b-split-rit-xit = VFO A/B、异频，以及 RIT/XIT 偏移
topbar-tuning = 调谐
topbar-diversity-filter-which-way-combines = 分集滤波器：以何种方式合并两根天线、适应速度，以及将其保持在当前位置
topbar-diversity = 分集
topbar-second-receiver-s-frequency-mode = 第二台接收机的频率、模式、滤波器和电平
topbar-sub-receiver = 副接收机
topbar-transceiver-s-own-aerial-socket = 收发信机自身的天线插座及其电源开关
topbar-radio = 电台
topbar-tune-voice-keyer-drive-mic = 调谐、语音键控器，以及驱动和话筒电平
topbar-transmit = 发射
topbar-tune = {" "}调谐{" "}
topbar-display = 显示
topbar-logbook-spots-awards-memories-scanner = 日志、信号、奖项、记忆信道、扫描器、设置和手册
topbar-system = 系统
topbar-linked = 已连接
topbar-not-linked = 未连接
topbar-bar-trace = 条形 / 轨迹
topbar-needle-bar-trace = 指针 / 条形 / 轨迹
topbar-swap-vfos = 交换 VFO
topbar-copy-b = 将 A 复制到 B
topbar-second-receiver-right-ear-tunes = 第二台接收机，在右耳中收听。它独立于 A/B 进行调谐——其控件出现在 SUB 模块中，其通带显示在瀑布图上。
topbar-band-plan = ，来自频段计划
topbar-repeater-shift-following-band-plan = 中继偏移：跟随频段计划，而该频率在频段计划中没有偏移——即单工
topbar-repeater-shift-transmit-above-below = 中继偏移：在表盘之上或之下发射，以使用中继台。现已为单工
topbar-repeater-shift = 中继偏移
topbar-auto = 自动
topbar-take-shift-band-plan-dial = 随表盘移动，从频段计划中获取偏移。它只在中继输出子频段内生效——在其他任何地方，它都会使电台保持单工，因此呼叫信道保持单工。
topbar-offset = 偏移
topbar-repeater-tone-ctcss-dcs-goes = 中继音：在语音下发出的 CTCSS/DCS、1750 Hz 脉冲，以及接收音静噪
topbar-transmit-tone = 发射音
topbar-set-here-sent-nfm-sub = 在此设置，以 NFM 发送——亚音频信令属于 FM 信道
topbar-normal = 正常
topbar-invert = 反相
topbar-bit-order-dcs-encoded-here = 此处 DCS 编码所用的位序是从标准转录而来的，从未在中继台上核对过——若无法打开，请尝试 CTCSS
topbar-t-1750-hz-burst = 1750 HZ 脉冲
topbar-every-over = 每次通话
topbar-open-burst-every-time-transmitter = 每当发射机按下时，都以脉冲开启
topbar-send = 发送
topbar-send-burst-now-keying-transmitter = 立即发送脉冲——若发射机尚未按下，则按下它以持续该脉冲的时长
topbar-nfm-only-burst-fm-repeater = 仅 NFM：脉冲即 FM 中继台的开门信号
topbar-receive-tone-squelch = 接收音静噪
topbar-require-receive-what-station-transmits = 在接收时要求本台所发射的音调
topbar-hide-docked-band-selector-click = 隐藏已停靠的频段选择器——再次点击即可重新显示
topbar-show-docked-band-selector = 显示已停靠的频段选择器
topbar-band-mode = 频段与模式
topbar-hide-band-chip-brings-back = 隐藏——频段标签可将其重新带回
topbar-dec-off = DEC 关
topbar-vol = 音量
topbar-counted-steps-not-decibels-what =


    Counted in steps, not decibels: what one step is worth is the receiver's own business, and on an SDRplay it depends on the band as well. Right is still more gain.
topbar-gain = 增益
topbar-plutosdr-s-own-agc-setting =


    The PlutoSDR's own AGC is setting this gain, so moving the slider does nothing. Set Settings → Radio → AGC to Manual to control it here.
topbar-agc-hang-time-click-cycle = AGC 保持时间——点击循环切换：关 / 慢 / 中 / 快
topbar-man = 手动
topbar-manual-audio-gain-used-while = 手动音频增益，在 AGC 关闭时使用。以关闭 AGC 时它所保持的电平为初始值。
topbar-radio-s-own-squelch-sent = 电台自身的静噪，通过控制链路发送。这是音频实际通过的闸门——软件静噪只会在电台已放行的内容上进一步关闭。
topbar-max-gain = 最大增益
topbar-manual-gain = 手动增益
topbar-noise-reduction = 降噪
topbar-impulse-noise-blanker = 脉冲噪声消隐器
topbar-auto-notch-cancel-constant-tone = 自动陷波：消除恒定音调成分（差拍干扰）
topbar-binaural-audio-not-while-sub = 双耳音频——副接收机占用右耳时不可用
topbar-binaural-audio-passband-spread-across = 双耳音频：通带分布在两只耳朵上，因此不同音调的信号来自不同方向，你所调谐的那一个会在两耳之间漂移。点击可恢复单声道
topbar-binaural-cw-spread-passband-across = 双耳 CW：将通带分布在两只耳朵上，使不同音调的信号来自不同方向——拥挤的通联不再只是一个挤在一起的音，而是分散在多个位置，调谐某个电台时它会在两耳之间漂移。在耳机上效果最佳
topbar-binaural-audio-spread-passband-across = 双耳音频：将通带分布在两只耳朵上。对于话音，噪声会分散在整个声像中，而电台则保持在中间，这样听上一个小时会更轻松——代价是话音本身也被分散开。在耳机上效果最佳
topbar-mute = 静音
topbar-record-audio-raw-i-q = 录制音频、原始 I/Q，或两者
topbar-wfm-stereo-forced-off-click = 调频立体声已强制关闭——点击以恢复自动立体声
topbar-wfm-stereo-pilot-locked-click = 调频立体声：导频已锁定。点击可强制单声道
topbar-wfm-stereo-automatic-no-pilot = 调频立体声：自动，此电台没有导频
topbar-rds-data-arriving-click-see = 正在接收 RDS 数据——点击查看
topbar-rds-no-data-station-click = RDS——此电台没有数据。仍可点击查看解码器的诊断信息
topbar-drm-decoding-click-details = 正在解码 DRM——点击查看详情
topbar-drm-click-decoder-s-state = DRM——点击查看解码器的状态
topbar-hd-radio-decoding-click-details = 正在解码 HD Radio——点击查看详情
topbar-hd-radio-click-decoder-s = HD Radio——点击查看解码器的状态
topbar-presets = 预设
topbar-width-per-sideband = 每个边带的宽度
topbar-width = 宽度
topbar-passband-width-hertz-type-drag = 通带宽度，单位为赫兹——可直接输入，也可拖动。点击输入框即可输入精确数值。
topbar-edges = 边缘
topbar-low-edge-hz-carrier = 低端边缘，距载波的赫兹数
topbar-high-edge-hz-carrier = 高端边缘，距载波的赫兹数
topbar-noise-reduction-voice-off-click = 降噪（话音），关闭——点击以选择引擎
topbar-mp3 = ● MP3
topbar-what-you-hear-mp3-receiver = 你听到的内容，以 MP3 格式保存：接收机在一个声道，你自己的发射在另一个声道。这就是一次通联的录音。
topbar-mono = 单声道
topbar-mixed-one-channel-click-two = 混合为一个声道——点击切换为两个声道
topbar-two-channels-rx-left-tx = 两个声道：接收在左，发射在右——点击切换为混合的一个声道
topbar-quick-clip = 快速剪辑
topbar-stop-after = 之后停止
topbar-no-stop = 不停止
topbar-auto-record = 自动录制
topbar-set-squelch-auto-record-needs = 请设置静噪——自动录制需要它来判断静音
topbar-auto-record-needs-sdr-front = 自动录制需要带有软件静噪的 SDR 前端
topbar-stop-following-squelch = 停止跟随静噪
topbar-waiting-signal = 等待信号
topbar-record-spectrum = 录制频谱
topbar-i-q-wav = ● I/Q WAV
topbar-i-q-wav-2 = I/Q WAV
topbar-raw-spectrum-receiver-delivering-stereo = 接收机输出的原始频谱，保存为立体声 32 位浮点 WAV（超过 4 GB 时为 RF64），SDR#、SDRuno、HDSDR 和 SDRangel 都能打开它——sdroxide 自身也能用 --file 回放它。文件很大：每个采样 8 字节。
topbar-radio-hands-over-demodulated-audio = 此电台输出的是解调后的音频，因此没有 I/Q 可供录制。
topbar-what-capture-costs-sample-rate = 在此采样率下录制的开销
topbar-engine = 引擎
topbar-no-noise-reduction-decoders-never = 无降噪——解码器本来就从未看到它
topbar-strength = 强度
topbar-tone = 音调
topbar-ctcss-dcs-no-sub-audible = CTCSS / DCS——此信号上没有亚音频音调
topbar-tone-squelch = 音调静噪
topbar-carrier-squelch-open-any-signal = 载波静噪：任何信号都会打开
topbar-require-tone-currently-being-received = 要求当前正在接收的音调
topbar-ctcss = CTCSS
topbar-any-dcs = 任意 DCS
topbar-open-any-dcs-coded-signal = 任何带 DCS 编码的信号都会打开。它所携带的 104 个编码中具体是哪一个，无法在此可靠读取，因此没有更细的选项可选
topbar-which-socket-back-radio-receiving =
    电台在背面的哪个插座上接收——即其自身的 ANT 命令，与前面板上 ANT 按钮的设置相同。点击可切换到下一个。

    The choice is remembered per band, and put back the next time the dial crosses into that band.
topbar-radio-s-separate-receiving-antenna =
    电台独立的接收天线，切入或切出接收通路——即其自身的 RX ANT 设置。无论如何，主插座上的天线都保留用于发射。

    The radio remembers this per band itself, so sdroxide reads it back after every band change rather than putting back what it last saw: clicking here is the only thing that moves it.
topbar-radio-s-own-power-switch =
    电台自身的电源开关，通过控制链路操作，也是本程序中唯一真正的开关——而非 sdroxide 的“连接”开关，后者只关闭 sdroxide 这一端，电台仍在运行。

    For ON to reach anything the radio's control end has to stay awake while it is off: Network Control over the LAN, or a CI-V port still fed from the mains on a set switched off at the front.
topbar-switch-radio = 开启电台
topbar-switch-radio-off-audio-meters = 关闭电台。音频和表头随之停止。
topbar-what-second-aerial-click-swap =
    第二根天线的用途——点击即可切换。

    CANCEL subtracts it from the first, in the gain, phase and delay that make a local noise source line up on both: the DSP form of a noise-cancelling phaser. COMBINE adds the two in the phase that reinforces, weighted by how well each hears — diversity reception, which fills in fades.
topbar-hold = 保持
topbar-stop-filter-moving-reach-moment = 让滤波器停止移动。一旦出现零点就应使用它：若滤波器持续自适应，它会重新对准变得最响的任何东西，而在安静的频段上，那往往就是你正在收听的电台。
topbar-restart = 重新开始
topbar-zero-filter-find-null-again = 将滤波器归零，并重新找到零点。
topbar-adapt = 自适应
topbar-how-fast-filter-chases-slow = 滤波器追踪的速度：左侧缓慢而稳定，在不到一秒内收敛；右侧则明显地来回搜寻。先快速找到零点，然后将其保持。
topbar-where-sub-receiver-listens-shift = 副接收机收听的位置。按住 Shift 点击瀑布图，或在副接收机的通带内拖动，即可移动它。
topbar-dial = ←表盘
topbar-move-sub-receiver-main-dial = 将副接收机移动到主表盘
topbar-dial-2 = 表盘→
topbar-move-main-dial-sub-receiver = 将主表盘移动到副接收机
topbar-filter = 滤波器
topbar-sub-receiver-passband-edges-hz = 副接收机通带边缘，单位为赫兹
topbar-sub-receiver-level-plays-right = 副接收机电平（它在右耳播放）
topbar-ptt = {" "}PTT{" "}
topbar-voice-keyer-record-transmit-stored = 语音键控器：录制并发射已存储的消息
topbar-drive = 驱动
topbar-tune-2 = 调谐
topbar-mic = 话筒
topbar-cessb = CESSB
topbar-microphone-gain = 话筒增益
topbar-deviation-how-far-mode-s =
    频偏：此模式的脉冲使自行调制它的电台摆动多远。FM 发射机把音频电平转换为频率摆幅，而且没有 ALC 来制约它，因此把满量程送入一个设为话音的数据输入，会造成过度频偏——在收听者听来完全正常，却不能为任何人解码。

    Kept per mode, so a deviation set for 1200 baud packet never lands on FT8.
topbar-transmit-audio-how-hard-mode =
    发射音频：此模式以多大力度驱动自行调制我们所发送内容的电台的调制器——例如声卡上的 CAT 电台、FLEX，或网络端口上的 Icom。将其调低，直到电台的 ALC 几乎不动，然后在电台上设置功率；恒包络数字模式下 ALC 过度工作，正是造成频谱扩散的原因。

    Drive is not this control: on these radios Drive reaches the rig's power register and never touches its audio.

    Kept per mode — FT8, RTTY, PSK and MCW each keep their own.
topbar-tx-audio = 发射音频
topbar-cw-psk-rtty-skimmers-decode = CW / PSK / RTTY skimmer——在整个频段上解码信号，并在瀑布图上标注它们
topbar-skimmers = SKIMMER
topbar-run-skimmer = 运行此 skimmer
topbar-minimum-snr-decoded-signal-needs = 解码信号被标注所需的最低信噪比
topbar-needs-wideband-iq-source = 需要宽带 IQ 信源
topbar-cw-decoder = CW 解码器
topbar-how-many-signals-model-reads = 模型一次读取多少个信号。其余信号保留标记，但不附带文字。
topbar-spots = 信号列表
topbar-save-skimmer-s-spot-list = 将 skimmer 的信号列表保存到文件
topbar-show-hide-full-band-waterfall = 显示/隐藏全景图上方的全频段瀑布图条带——即此接收机能一次看到的全部内容
topbar-spectrum-waterfall-either-layer-both = 频谱和瀑布图——任选一层、两层或都不选。两者都显示时亮起。
topbar-spectrum = 频谱
topbar-show-spectrum = 显示频谱
topbar-draw-spectrum-line-across-top = 在全景图顶部绘制频谱线。关闭后，瀑布图占据全部高度。
topbar-peak-hold = 峰值保持
topbar-trace-highest-level-each-column = 在实时曲线之上追踪每一列所达到的最高电平，然后逐渐回落——即你看向别处时频段曾出现过的情况
topbar-draw-spectrum-receding-surface-instead = 以后退的曲面而非平面线绘制频谱：最新的频谱位于前排，之前的频谱则向远处流去。频段的最近几秒呈现为一幅景观——时有时无的载波是一道山脊，而不是抽动的线。网格和峰值保持属于平面线，不会绘制在其上。
topbar-how-quickly-spectrum-line-follows = 频谱线跟随频段的速度。越慢，相邻帧之间的平均越多：线条更稳定，而静止足够久的弱载波也能读出来。瀑布图不受其影响——无论如何，这些行都会获得每一帧。
topbar-show-waterfall = 显示瀑布图
topbar-draw-scrolling-waterfall-below-spectrum = 在频谱下方绘制滚动的瀑布图。关闭后，频谱线占据全部高度。
topbar-how-fast-waterfall-scrolls-lines = 瀑布图每秒滚动的行数：慢速 5、中速 28、快速 56、更快 112、最快 224。引擎自己计时，因此两个最快档是真正的细节，而不是同一行画了两遍——只要接收机能够供给：一行永远只能显示一次变换，而较窄的前端每秒只能产生几十行。它们会占用历史记录，因为瀑布图保存固定数量的行——中速时为 73 秒，最快时为 9 秒。
topbar-decode-labels = 解码标签
topbar-mark-every-decoded-station-waterfall = 在瀑布图上，以其被收听到的频率标记每一个已解码的电台及其呼号——适用于 FT8、FT4 和其他分时隙模式。良好的开端是每两分钟在整个跨度上放置三十个标签，覆盖在你正在阅读的轨迹之上；关闭它们，瀑布图就只是频段本身。旁边的解码列表仍然包含每一个呼号。
topbar-switch-3d-set = 请先开启 3D 以设置此项
topbar-how-3d-spectrum-drawn-only = 3D 频谱如何绘制。只有形状不同——两种呈现方式显示的都是相同的频谱，而且都会遮挡其后方的内容，因此前方的强信号会矗立在它所覆盖的频段之上。
topbar-lines = 线条
topbar-one-trace-per-remembered-spectrum = 每个记忆的频谱一条轨迹，颜色与平面线相同，并遮挡其后的轨迹。只显示形状而不显示电平——当你需要的是形状时，应选择此读法。它比实心模式的开销略大而非更小，因为它先绘制相同的曲面，然后在其上描绘每一个峰顶。
topbar-solid = 实心
topbar-filled-surface-coloured-waterfall-s = 一个由瀑布图调色板着色的填充曲面，因此电平既体现在颜色上，也体现在高度上，全景图的两半对强信号的样子看法一致。可在“设置 › 显示”中更改调色板。
topbar-how-fast-3d-spectrum-flows = 3D 频谱离你远去的速度，单位为每秒行数：慢速 6、中速 12、快速 24、更快 48。曲面的深度为固定的行数，因此这也决定了它保存的时间长度。越慢记忆越长，曲面则缓慢爬行；越快则记忆越短，但移动得更快。瀑布图有其自己的滚动速率，见下文。
topbar-how-many-columns-panadapter-waterfall = 全景图及其瀑布图绘制的列数。“自动”会读取此计算机的渲染器和全景图的大小，并选择它能承载的最大值——4K 屏幕需要 4096。每一列在每一帧中都占一个字节，因此连接到远程电台站点的客户端需要为其链路上的细节付出代价：4096 列在 60 fps 下约为每秒四分之一兆字节，是标准宽度的两倍。
topbar-read-machine-screen-take-most = 读取此计算机和此屏幕，取它们能承载的最大值，并在它们变化时随之调整
topbar-width-force-right-now-whatever = 当前生效的宽度，无论上面一行如何要求
topbar-keep-floor-ceiling-set-best = 保持为获得最佳瀑布图对比度而设置的底噪/顶端——在频段变化、平移或缩放之后，以及电平漂移时，会缓慢回到位置。开启它可立即适配；关闭它则使电平保持在你设置的位置。
topbar-keep-tuned-frequency-middle-panadapter = 将调谐频率保持在全景图中央：窗口会在表盘下滑动，而不是在你调谐到边缘之外时图像整段跳动。开启它可立即居中；关闭它则可随意平移和缩放。
topbar-ceil = 顶端{" "}
topbar-fft-size = FFT 大小
topbar-scroll-waterfall-upwards-newest-row = 使瀑布图向上滚动——最新的行在底部（V）
topbar-logbook-all-qsos-digital-manual = 日志——所有通联（数字 + 手动）
topbar-award-tracking-dxcc-was-waz = 奖项追踪——DXCC / WAS / WAZ / 网格
topbar-band-conditions-published-forecast-beside = 频段条件——公布的预报，旁边是每个频段上实际听到的情况
topbar-satellite-qo-100-beacon-calibration = 卫星——QO-100 信标校准正在运行
topbar-satellite-doppler-tracking-qo-100 = 卫星——多普勒跟踪，以及 QO-100 信标校准
topbar-ism-band-devices-decoding-now = ISM 频段设备——正在解码，无论此窗口是否打开。在窗口内使用 DECODING 将其关闭。
topbar-ism-band-devices-weather-sensors = ISM 频段设备——你周围听到的气象传感器、仪表和家庭自动化设备
topbar-public-sdrs-internet-browse-kiwisdr = 互联网上的公共 SDR——浏览 KiwiSDR 和 SpyServer 目录，并将其中之一作为电台打开
topbar-winlink-radio-email = Winlink 无线电邮件
topbar-memory-channels = 记忆信道
topbar-scanner-stopped-signal = 扫描器——停在一个信号上
topbar-scanner-running = 扫描器——运行中
topbar-scan-memory-channels-frequency-range = 扫描记忆信道或频率范围
topbar-grid-tracker-worked-maidenhead-squares = 网格追踪器——地图上已通联的梅登海德方格，以及已听到的内容
topbar-settings-device-gains-antennas-audio = 设置——设备增益、天线、音频设备
topbar-user-manual-f1 = 用户手册（F1）
topbar-range-was-typed-settings-radio = 该范围是在“设置 ▸ 电台 ▸ RX 范围”中输入的，而不是由电台报告的。请将其加宽，或清空输入框以使用设备自行报告的范围。
topbar-what-device-reports-about-itself = 这是设备自行报告的内容。如果电台确实覆盖此频段，请在“设置 ▸ 电台 ▸ RX 范围”中填写其实际范围。
topbar-band = 频段
topbar-mode = 模式

## Top bar: the System box's window chips.

topbar-chip-log = 日志
topbar-chip-awards = 奖项
topbar-chip-bands = 频段
topbar-chip-sat = 卫星
topbar-chip-ism = ISM
topbar-chip-mail = 邮件
topbar-chip-mem = 记忆
topbar-chip-scan = 扫描
topbar-chip-hfdl = HFDL
topbar-chip-grid = 网格

## Top bar: the mode-defaults chip.

topbar-changed-agc = AGC
topbar-changed-squelch = 静噪
topbar-changed-auto-notch = 自动陷波
topbar-changed-stereo = 立体声
topbar-changed-binaural = 双耳
# $mode is a mode name (USB, CW…); $changed lists what differs from its defaults.
topbar-mode-defaults =
    返回 { $mode } 的默认值。已更改的项目：{ $changed }。

    Click to put them back. { $mode }'s own values return, and what you set here is forgotten.

## Settings window, Radio tab: longer messages with values

settings-radio-no-such-baud = 此电台没有 { $baud } 波特的设置——将改用 { $default }
settings-radio-icom-scope-hint =
    通过 CI-V 链路传输电台自身的示波器扫描，并将其绘制为全景图——对于一台解调音频的电台，这是它能提供的唯一频段画面，因为它发送的音频本身就是已经过其滤波器的结果。

    Needs the radio set up for it: CI-V USB Baud Rate to 115200 and CI-V USB Port to "Unlink from {"["}REMOTE]" (both under SET > Connectors > CI-V), and the baud rate above set to match — below { $min_baud } the sweeps do not fit down the link and the scope stays off.

    The sweeps share the radio's internal USB bus with its own sound card. If received audio starts to drop out with the scope on, this box is the first thing to try turning off.
settings-radio-rx-tx-words = { $rx } / 发射 { $tx }
settings-radio-soapy-native = sdroxide 直接支持上面的硬件：{ $names }。选择上面的该接口可获得其自身的设置——SoapySDR 无法表达的增益级、滤波器和陷波——并且无需 SoapySDR 模块。
settings-radio-rx888-width-hint = 全景图一次显示数字化频谱的多少。整个 DSP 链路都以此宽度运行，因此越宽则 CPU 开销成比例增加——1/2 即瀑布图中的整个频段，计算量相当可观。在 VHF 分界点（{ $crossover } MHz）以上，全景图看的不是天线，而是调谐器的中频，它宽 8 MHz，固定在 { $if_centre } MHz——因此任何宽于 { $widest } 的设置都无法将其居中，多出的宽度只是信号旁边的空白频谱，而并非更多的信号。
settings-radio-sdrplay-serial-missing = 序列号 { $serial } 不在 SDRplay 服务报告的接收机之中。请选择列出的某一台接收机，或重新插拔此设备后按“重新扫描”。
settings-radio-hdr-filter-hint =
    位于 HDR 通路之前的模拟滤波器——与接收机自身的带宽是不同的控件。每个设置都只在固定的一组中心频率上构建，在其他地方则不起作用；此处的滤波器是在 { $centres } 上构建的。立即生效。

    The path itself does not work yet — see the switch above — so this chooses a filter for a mode that is currently silent.
settings-radio-rspduo-only = 只有 RSPduo 具有第二个调谐器，而此处所选的接收机是 { $model }，因此此项不会起任何作用。
settings-radio-lime-tx-gain-low = 发射增益为 { $db } dB，处于其范围的最低端——板卡只输出几微瓦，无论其后接的是什么，功率计上都会显示为零。这是默认值，以使已就绪的发射机不会让你措手不及；在按下发射之前请将其调高。

## Settings window, General: SWR

settings-general-swr-tune-limit = （调谐：{ $limit }:1）

## Top bar and control strip: messages with values

# $reading is the meter's own hover text; $faces lists the faces a click cycles through.
topbar-smeter-hover =
    { $reading }

    Click to cycle meter face: { $faces }
topbar-step-down = 降低 { $step }
topbar-step-up = 增加 { $step }
# $auto is topbar-band-plan or empty.
topbar-repeater-shift-hover = 中继偏移 { $shift } — 在 { $rx } 上接收，在 { $tx } MHz 上发射{ $auto }
topbar-tone-and-burst = 在语音下发射 { $tone }，并在每次通话开始时发送一个 { $ms } ms 的 1750 Hz 脉冲
topbar-tone-under-voice = 在语音下发射 { $tone }
topbar-burst-every-over = 每次通话开始时发送一个 { $ms } ms 的 1750 Hz 脉冲
topbar-match-tx = 与发射匹配（{ $tone }）
topbar-decimation-on = 前端抽取：接收机只获得此电台所传输的 { $msps } Msps 中央的 1/{ $n }——{ $span }——其余部分在到达接收机之前即被丢弃。
topbar-decimation-off = 前端抽取，关闭：接收机看到的是此电台所传输的整个 { $span }。
topbar-decimation-hint =
    { $what }

    A narrower span means finer waterfall resolution, a quieter noise floor (3 dB per halving) and less CPU — at the cost of the band either side of it. The dial still tunes anywhere; the radio moves its LO to follow.

    Click to cycle: off / 2 / 4 … { $max }.
topbar-rx-gain-hint = 前端接收增益（{ $stage }）。过大会使接收机的 ADC 削波，并把杂散信号涂抹到整个频段上；过小则接收机会变聋。
topbar-rx-gain-stages =
    {""}

    This rig has { $n } RX gain stages — the rest are in Settings → Device.
topbar-passband-now =
    {""}

    The passband is at { $dbfs } dBFS now.
# $level is topbar-passband-now or empty.
topbar-squelch-hint =
    以 dBFS 为单位，在接收通带中将低于此功率的音频关闭。向左为打开。{ $level } 请将其设在噪声之上、信号之下。

    A stream that arrives with the radio's own AGC already in it — an Icom's 12 kHz IF, for one — sits far higher on this scale than an SDR's raw baseband does, which is why the rail reaches full scale.

    Click to cycle the strength: Low / Med / High / Off. The engine is in the rows below.
topbar-recording-both = 正在录制 { $audio } 和 { $iq }
topbar-recording-audio = 正在将音频录制到 { $audio }
topbar-recording-iq = 正在将 I/Q 录制到 { $iq }
topbar-auto-record-armed = 自动录制已就绪：每次发射一个文件，静音 { $secs } 秒后关闭
topbar-rds-hover = RDS：{ $ps }。点击查看电台的数据
topbar-drm-hover = DRM：{ $service }。点击查看此广播的详细信息
topbar-drm-no-codec = DRM：{ $summary } — 已锁定，但其 { $codec } 音频无法在此解码。点击查看缺少什么
topbar-drm-state = DRM：{ $summary } — 点击查看解码器的状态
topbar-hd-hover = HD Radio：{ $station }。点击查看此广播的详细信息
topbar-hd-state = HD Radio：{ $summary } — 点击查看解码器的状态
topbar-binaural-preset = 每只耳朵 { $preset } — { $lo } … { $hi } Hz，两个边带
topbar-nr-hover =
    降噪：{ $strength } — { $engine }

    Click to change engine or strength
topbar-clipping = 剪辑 { $span } — 再次按下即可重新开始
topbar-record-clip = 录制 { $span } 并停止——作为接收报告附件的剪辑
topbar-starting-clip = 正在开始 { $span } 的剪辑……
topbar-stops-in = 将在 { $time } 后停止
topbar-auto-record-hint = 将每次发射录制到其自己的文件中，静音 { $secs } 秒后关闭
topbar-recording-silence = 正在录制 · 静音 { $secs } 秒（在 { $hold } 时关闭）
topbar-tone-open = 正在接收 { $tone }，即音调静噪——音频已打开
topbar-tone-closed = 正在接收 { $heard }，但音调静噪需要 { $wanted }——音频保持关闭
topbar-tone-heard = 正在接收 CTCSS/DCS { $tone }
topbar-tone-nothing = 音调静噪 { $tone }：没有接收到与之匹配的内容
topbar-tone-use = 使用 { $tone }
topbar-sockets = 插座：{ $sockets }
topbar-voice-keyer-tx = 正在发射 { $slot } — 点击打开语音键控器
topbar-drive-uncal-gen = 此频率不属于任何业余频段，因此未经校准，设置会完整地作用于发射机。
topbar-drive-uncal-band = { $band } 未经校准，因此设置会完整地作用于发射机。
# $trim is signed (+1.5); $diff is its size; $more is "less" or "more".
topbar-drive-cal-gen = 此频率不属于任何业余频段，经校准为 { $trim } dB，因此同样的设置在此处比未经校准的频段{ $more ->
        [less] 低
       *[more] 高
    } { $diff } dB 发射出去。
topbar-drive-cal-band = { $band } 经校准为 { $trim } dB，因此同样的设置在此处比未经校准的频段{ $more ->
        [less] 低
       *[more] 高
    } { $diff } dB 发射出去。
topbar-drive-hover =
    发射机被驱动的力度。在 sdroxide 自行调制的电台上，它缩放的是已调制的采样；在带有自身功率控制的电台上，它是请求电台输出的额定功率的比例。

    { $trim }

    Settings → Radio → Transmit drive by band.
topbar-cessb-hover =
    受控包络 SSB：同样的峰值下平均功率更大，且没有频谱扩散——在对方听来响度增加约三到四分贝。这是话音被推入处理器的分贝数；0 为关闭，6 是合理的首次尝试。它不影响驱动。

    Now: { $now }
topbar-waterfall-rate = 每秒 { $rows } 行——{ $seconds } 秒的频段
topbar-detail-auto-columns = { $n } 列，即“自动”在此处会选择的数值
topbar-n-columns = { $n } 列
topbar-fft-bins-whole = 整个数据流上 { $n } 个频点——每列一个，或更细
topbar-fft-bins-pooled = { $n } 个频点汇集到全景图的 { $cols } 列中：信号更锐利，但并非更多信号
topbar-sat-locked = 卫星——已锁定 { $name }
topbar-filter-edges = 距载波 { $lo } … { $hi } Hz
topbar-filter-isb = 接收滤波器：每只耳朵 { $width } Hz（{ $edges }）。ISB 的两个边带是各自独立的发射，因此此宽度是其中一个边带的宽度。
topbar-filter-width = 接收滤波器：宽 { $width } Hz（{ $edges }）。
topbar-filter-hint =
    { $what }

    Click for the width, the two edges and this mode's presets, as numbers — the panadapter's grips place a passband by eye, this is where an exact figure is typed.
topbar-band-outside-range =
    { $band } 超出此电台的接收范围（{ $ranges } MHz）。

    { $why }
# $time is "day" or "night".
topbar-band-forecast = { $band }：{ $verdict }（{ $time ->
        [day] 白天
       *[night] 夜间
    }）——由 HAMQSL.com 根据太阳指数预报，而非你自己路径的测量。

## Top bar: short words and captions

topbar-squelch-open = 打开
topbar-off = 关
topbar-recording = 录制中
topbar-sql = SQL
topbar-stations = 电台
topbar-caption-reaction = 响应
topbar-caption-scroll = 滚动
topbar-caption-surface = 曲面
topbar-caption-flow = 流动
topbar-caption-detail = 细节
topbar-floor = 底噪

## Digital-mode panel tabs (src/app/panels/mod.rs, pane_label).

pane-aircraft = 飞机
pane-chart = 图表
pane-chat = 聊天
pane-decodes = 解码
pane-files = 文件
pane-heard = 收听
pane-image = 图像
pane-map = 地图
pane-messages = 消息
pane-monitor = 监视
pane-panel = 面板
pane-qso = 通联
pane-reading = 阅读
pane-receive = 接收
pane-saved = 已保存
pane-send = 发送
pane-spots = 信号
pane-stations = 电台
pane-status = 状态
pane-terminal = 终端
pane-text = 文本
pane-traffic = 流量
pane-vessels = 船舶
pane-wfall = 瀑布

## Digital-mode panels, shared (src/app/panels/mod.rs)

panels-save = {" "}保存{" "}
panels-nothing-decoded-save = 没有可保存的解码内容
panels-clear-rx = {" "}清空接收{" "}
panels-empty-receive-window-nothing-air = 清空接收窗口。空中的任何内容都不会停止。
panels-nothing-received-clear = 没有可清空的接收内容
panels-save-what-panel-has-decoded = 将此面板解码的内容保存到文件
panels-prop = 传播
panels-shade-map-where-signals-are = 按信号在每个频段上实际抵达的位置为地图着色。每一次接收都标在其路径的中点——即使之弯曲的那片电离层——而不是对方电台的位置，因此这张图反映的是天空，而非业余无线电爱好者所在的地方。它由本电台参与过的路径构成；若在“设置 → 信号”中开启了反向信标网络（RBN），还包括全球 skimmer 所听到的内容，这涵盖了此电台并未收听的频段。
panels-night = 夜间
panels-shade-where-sun-down-twilight = 在太阳落下的地方以及其间的暮光区域着色，使灰色分界线显示在地图上。在其夜侧，低频段延伸得更远，高频段则收缩；而分界线本身正是 DX 所在之处。
panels-all-bands = 所有频段
panels-every-band-once-one-hue = 所有频段同时显示，每个频段一种色调——总体条件。
panels-one-band = 单个频段
panels-one-band-cold-hot-blue = 单个频段，由冷到热：蓝、绿、黄、红。
panels-frequency-you-are-actually-working = 你实际工作的频率：表盘加上其旁边的音调偏移。这才是应记入日志的内容——仅表盘频率会低这么多。
panels-send-return = 返回时发送
panels-freq = ⇵ 频率
panels-nothing-agreed-mode-save-your = 此模式下没有约定的频率——请在下方保存你自己的频率。
panels-forget-saved-frequency = 删除此已保存的频率
panels-dial-already-one-your-saved = 此表盘频率已是你保存的频率之一。
panels-dial-already-agreed-list-mode = 此表盘频率已在此模式的约定列表中。
panels-decode-squelch-raise-stop-decoding = 解码静噪——调高以停止解码噪声
panels-message-repeats-long-transmit-held = 只要按住发射，消息就会重复——流星会捕捉到正在经过的那一部分。
panels-w1abc-w9xyz-fn42 = W1ABC W9XYZ FN42
panels-tx = {"  "}发射开{"  "}
panels-tx-2 = {"   "}发射{"   "}
panels-call-cq = {" "}呼叫 CQ{" "}
panels-nocall = 无呼号

## TX Audio window (src/app/tx_audio.rs)

txaudio-heard-too-little-speech-talk = 听到的话音太少——请在整个计数期间正常说话，然后重试。
txaudio-tx-audio = 发射音频
txaudio-mode-not-voice-mode-so = 此模式不是话音模式，因此它发送的任何内容都不经过此处理链。在此所做的设置对 SSB、AM 和 FM 生效。
txaudio-bypass = 旁路
txaudio-switch-every-stage-out-once = 一次性将每一级（包括降噪器）全部切出，使话筒以未处理的方式发出。再次点击可精确恢复原先开启的各级——这是对比处理链作用的最快方法。
txaudio-denoise = 降噪
txaudio-rnnoise-microphone-ahead-every-other = 对话筒使用 RNNoise，位于所有其他级之前，因此在噪声门或压缩器将室内噪声提升之前，它已被去除。会增加约 10 ms 的延迟。
txaudio-digital-voice = 数字话音
txaudio-also-run-freedv-rade-digital = 同时让 FreeDV / RADE 数字话音经过此处理链。默认关闭：声码器是在自然语音上训练的，对于经过大量处理的声音，其解码效果可能更差，因此在依赖它之前，请先在空中试用。TCI 音频和数字模式从不经过此处理链。
txaudio-test = 测试
txaudio-rec = {" "}⏺ 录制{" "}
txaudio-play = {" "}▶ 播放{" "}
txaudio-play-test-phrase-through-chain = 按当前设置，将测试短语经过处理链播放到你的扬声器。在播放时进行调整，即可听到变化。播放期间，接收音频会被替换。
txaudio-recording = 录制中……
txaudio-no-phrase-yet = 尚无短语
txaudio-live = 实时
txaudio-live-2 = {" "}🎧 实时{" "}
txaudio-hear-your-microphone-through-chain = 在你说话时，通过处理链听到自己的话筒，而无需按下发射机。请使用耳机：通过扬声器会产生反馈。你会听到自己晚几十毫秒的声音，这会使说话变得别扭——请用表头设置电平，并用“录制 / 播放”判断声音效果。运行期间，接收音频会被替换；按下 PTT 即可停止。
txaudio-meters = {" "}表头{" "}
txaudio-run-your-microphone-through-chain = 仅为了表头而让话筒经过处理链：扬声器上没有任何声音，因此没有需要与之对话的延迟，也不会产生反馈。这是设置话筒和输入增益、噪声门阈值以及压缩器工作强度的方法。接收音频继续；按下 PTT 即可停止。
txaudio-monitoring-live-use-headphones-speakers = 正在实时监听——请使用耳机，否则扬声器会反馈到话筒中。
txaudio-voice-modes-only-tci-clients = 仅限话音模式。TCI 客户端和数字模式会绕过此处理链；使用它时，请将电台自身的发射均衡器和处理器保持为平直。
txaudio-auto-set-stopped-microphone-was = 自动设置已停止：话筒已被占用。
txaudio-microphone-may-clipping-before-gets = {" "}话筒在到达此处之前可能已削波——若听起来刺耳，请调低它自身的增益。
txaudio-level = 电平
txaudio-mic-control-same-one-transmit = 话筒控件——与发射栏上的是同一个，由所有配置文件共用。中间为 0 dB，顶端为 +6 dB。
txaudio-mic = 话筒
txaudio-gain-top-mic-microphone-stored = 在话筒控件之上，针对此话筒的增益，与配置文件一同保存。预设的阈值假定话音在 IN 表头上的峰值约为 -10 dBFS；“自动设置”会将其设到此处。在“旁路”下仍然生效，并且也适用于数字话音：它是电平，而非处理。
txaudio-input = 输入
txaudio-auto-set = 自动设置
txaudio-auto-set-cancelled = 自动设置已取消。
txaudio-talk-now-you-would-air = 现在请像在空中那样说话……
txaudio-db = -∞ dB
txaudio-chain = 处理链
txaudio-order-audio-meets-them = 音频经过它们的顺序
txaudio-switch-stage = 开启此级
txaudio-earlier-chain = 处理链中的前面
txaudio-later-chain = 处理链中的后面
txaudio-output-limiter = 输出限幅器
txaudio-output = 输出
txaudio-trim-limiter-always-last = 修整和限幅器——始终位于最后
txaudio-profiles = 配置文件……
txaudio-gate-expander = 噪声门 / 扩展器
txaudio-pushes-room-noise-fan-rumble = 在停顿时把室内噪声、风扇隆隆声和嘶嘶声压低。将阈值设在底噪之上、最轻的音节之下。
txaudio-gate = 噪声门
txaudio-hard-gate-10-1-down = 硬噪声门：10:1，最多降至 -40 dB
txaudio-expander = 扩展器
txaudio-gentle-expander-2-1-never = 柔和扩展器：2:1，最多不超过 -15 dB——室内声音不会变得死寂
txaudio-reset = 复位
txaudio-back-defaults = 恢复默认值
txaudio-threshold = 阈值
txaudio-below-gate-starts-close = 低于此值时，噪声门开始关闭。
txaudio-return = 回差
txaudio-hysteresis-once-open-closes-only = 迟滞：一旦打开，它只在阈值以下这么多时才关闭，因此在阈值附近徘徊的话音不会使噪声门抖动。
txaudio-hold = 保持
txaudio-how-long-stays-open-after = 话音下降后，它会保持打开多久，使词尾不被切掉。
txaudio-release = 释放
txaudio-how-fast-closes-once-hold = 保持时间结束后，它关闭的速度。如果噪声门出现抽动，请加长此时间。
txaudio-floor = 下限
txaudio-most-ever-turns-pauses-down = 它在停顿时最多能将声音压低的量。
txaudio-ratio = 比率
txaudio-how-steeply-closes-below-threshold = 它在阈值以下关闭的陡峭程度。
txaudio-look-ahead = 预读
txaudio-opens-much-before-word-starts = 在一个词开始之前提前这么多打开，使其首个辅音不被切掉。会为所有发射内容增加相同的延迟。
txaudio-high-pass-low-shelf-two = 高通、低搁架、两个峰值、高搁架和低通。拖动曲线上的一个点，或在下方设置。将低通设为你的发射滤波器的宽度。
txaudio-flat = 平直
txaudio-every-band-back-0-db = 所有频段恢复到 0 dB，两端均关闭
txaudio-freq = 频率
txaudio-gain = 增益
txaudio-q-slope = Q / 斜率
txaudio-high-pass = 高通
txaudio-removes-handling-noise-breath-pops = 去除转角以下的手持噪声、呼吸爆破音和嗡嗡声。
txaudio-low-shelf = 低搁架
txaudio-body-mud-cut-little-here = 厚度，即浑浊感：若话音听起来像关在箱子里，请在此处稍微削减。
txaudio-peak-1 = 峰值 1
txaudio-bump-dip-voice-band-often = 话音频段中的凸起或凹陷——通常是 300–500 Hz 附近的削减。
txaudio-peak-2 = 峰值 2
txaudio-presence-lift-around-2-3 = 临场感：2–3 kHz 附近的提升，是 SSB 上穿透力的来源。
txaudio-high-shelf = 高搁架
txaudio-brightness = 明亮度。
txaudio-low-pass = 低通
txaudio-stops-energy-above-transmit-filter = 防止超出发射滤波器的能量被浪费在调制器中。
txaudio-t-24-db-oct = 24 dB/倍频程
txaudio-t-12-db-oct = 12 dB/倍频程
txaudio-click-steeper-gentler-slope = 点击以切换更陡或更缓的斜率
txaudio-de-esser = 去齿音器
txaudio-turns-down-only-sibilant-band = 仅当“s”和“sh”音超过阈值时才压低齿音频段，因此话音的其余部分保持其电平。可在说“sister silver cellophane”的同时扫描频率，并将其停在嘶嘶声最响之处来找到该频率。
txaudio-frequency = 频率
txaudio-centre-sibilant-band = 齿音频段的中心。
txaudio-width-band-higher-narrower = 频段的宽度——越高则越窄。
txaudio-band-level-where-reduction-starts = 频段电平，超过此值时开始压低。过低则会一直在工作，使话音变得发闷。
txaudio-amount = 量
txaudio-most-band-ever-turned-down = 频段最多被压低的量。从 -6 dB 左右开始。
txaudio-attack = 起振
txaudio-how-fast-reacts = 它反应的速度。
txaudio-how-fast-lets-go = 它松开的速度。
txaudio-compressor = 压缩器
txaudio-brings-quiet-syllables-up-holds = 将轻柔的音节提起，并压制响亮的音节。峰值处的压缩量以 3–6 dB 为目标；超过 10 dB 则是明显的压缩，更适合比赛。
txaudio-level-above-which-starts-compress = 超过此电平即开始压缩。
txaudio-how-hard-compresses-above-threshold = 超过阈值后压缩的力度。
txaudio-how-fast-reacts-peak-10 = 它对峰值反应的速度。10–25 ms 可使辅音保持清晰。
txaudio-how-fast-recovers = 它恢复的速度。
txaudio-knee = 拐点
txaudio-soft-knee-width-0-hard = 软拐点宽度；0 为硬拐点。
txaudio-makeup = 补偿
txaudio-gain-after-compression-bring-level = 压缩之后的增益，用于将电平重新提起。
txaudio-drive = 驱动
txaudio-gain-ahead-compressor-so-more = 压缩器之前的增益，使更多的话音越过阈值——提高平均功率。应与输出限幅器配合使用。
txaudio-phase = 相位
txaudio-all-pass-phase-rotator-ahead = 检波器之前的全通相位旋转器。它可以抚平话音中不对称的峰值，使相同的峰值电平携带更多的平均功率。4 是常见的广播设置；0 为关闭。
txaudio-last-stage-trim-look-ahead = 最后一级：修整，以及一个预读限幅器，它把每个峰值都限制在上限之下，而不会产生硬削波所造成的频谱扩散。默认关闭，因此开启处理链不会改变你已经设定的驱动电平。
txaudio-limiter = 限幅器
txaudio-on = 开
txaudio-ceiling = 上限
txaudio-no-peak-leaves-chain-above = 没有任何峰值会以高于此值的电平离开处理链。
txaudio-trim = 修整
txaudio-output-gain-ahead-limiter = 输出增益，位于限幅器之前。
txaudio-off = {"  "}关
txaudio-meters-move-while-you-transmit = 当你发射话音、播放测试短语或运行“实时”或“表头”时，表头会移动。
txaudio-profiles-2 = 配置文件
txaudio-profile-whole-chain-every-stage = 配置文件即整个处理链：每一级的设置、哪些级已开启、它们的顺序，以及你的话筒的输入增益。加载一个配置会替换整条处理链；话筒控件和驱动保持不变。
txaudio-name = 名称
txaudio-save = 保存
txaudio-store-chain-now-under-name = 以此名称保存当前的处理链，并替换同名的配置文件
txaudio-import = 导入……
txaudio-read-profile-json-file = 从 .json 文件读取配置文件
txaudio-tx-audio-profile = 发射音频配置文件
txaudio-no-profiles-yet-save-stores = 尚无配置文件。“保存”会存储你现在设置好的处理链。
txaudio-load = 加载
txaudio-export = 导出……
txaudio-delete = 删除
txaudio-starting-points = 起始点
txaudio-set-your-level-auto-set = 请先用“自动设置”设定你的电平——预设的阈值以此为前提——然后从此处进行调整，而不要按加载时的设置直接发射：录制一段短语，并在每次更改后收听。预设会保留你的输入增益。
txaudio-profile-loaded = 配置文件已加载。

## Solar-system 3D view overlay (src/solar3d/overlay.rs)

solar-prop = 传播
solar-where-signals-are-actually-getting =
    信号在每个频段上实际抵达的位置。本电台参与的每一次接收——WSPR、FT8/FT4、JS8 以及日志——都标在其路径的中点，即使之弯曲的那片电离层，而不是对方电台的位置。因此这张图反映的是天空，而非业余无线电爱好者所在的地方。

    It can only show paths this station has been one end of. Oceans light up because the midpoints of long paths fall there; Antarctica stays dark because nobody is transmitting from it. Off by default — it paints the whole planet.
solar-orbits = 轨道
solar-orbital-paths = 轨道路径
solar-clouds = 云层
solar-cloud-cover-noaa-s-global = 云层覆盖来自 NOAA 的全球静止卫星拼图，以一层空气的厚度绘制，而不是贴在表面上的图片：红外数据表明每个云顶有多冷，由此决定它有多高。雷暴会在它们所形成的塔状结构内闪烁。
solar-planets = 行星
solar-other-seven-planets-eighteen-their = 其余七颗行星、它们的十八颗卫星，以及土星和天王星的环。行星位置来自 JPL 的开普勒轨道根数集；卫星则是拟合到 JPL Horizons 的圆形轨道。
solar-coronal-mass-ejection-trajectory-cones = 日冕物质抛射轨迹锥
solar-sun-obs = 太阳观测
solar-solar-observations-sun-s-disk = 太阳圆面上的太阳观测：太阳黑子活动区，以及耀斑的来源
solar-labels = 标签
solar-body-region-labels = 天体和区域标签
solar-small-bodies = 小天体
solar-names-asteroids-comets-they-are = 为小行星和彗星标注名称。无论如何它们都会被绘制——这里只决定是否同时绘制三十五个编号。无论此项如何设置，相机所对准的天体以及查找框匹配的任何东西，都会保持标注名称。
solar-decoded-ft8-ft4-stations-path = 已解码的 FT8/FT4 电台，以及通往所通联电台的路径
solar-sats = 卫星
solar-amateur-radio-satellites-their-orbits = 业余无线电卫星、它们的轨道，以及从你的 QTH 看到的仰角
solar-aurora = 极光
solar-auroral-oval-noaa-s-ovation = 来自 NOAA OVATION 模型的极光椭圆，以其真实高度绘制为发光壳层，并在表面上标出 10% 等值线的赤道侧边缘
solar-awards = 奖项
solar-award-coverage-every-dxcc-entity = 奖项覆盖：每个 DXCC 实体都按你的日志仍然缺少的内容着色。橙色表示你从未与该实体通联过，琥珀色表示你通联过但未获确认，暗绿色表示已收到 QSL 卡。跟随“奖项”窗口中的频段过滤器。
solar-view = 视图
solar-which-body-camera-orbits-animated = 相机绕行哪个天体，以及动画导览
solar-layers = 图层
solar-what-drawn-orbits-clouds-cmes = 绘制的内容：轨道、云层、日冕物质抛射、标签、通联、极光
solar-which-sdo-channel-wraps-sun = 哪个 SDO 通道环绕太阳，以及它有多旧
solar-weather = 天气
solar-aurora-propagation-numbers-which-bands = 极光、传播数值，以及哪些频段开放
solar-space-weather = 空间天气
solar-nothing-measured-yet = 尚无测量数据
solar-scale = 比例
solar-body-moon-orbit-exaggeration = 天体和月球轨道的夸大
solar-time = 时间
solar-scrub-whole-scene-forward-back = 前后拖动整个场景
solar-activity = 活动
solar-replay-last-hour-decodes-globe = 在地球上回放过去一小时的解码
solar-what-propagation-heat-map-showing = 传播热力图显示的内容
solar-propagation = 传播
solar-auto = ▶ 自动
solar-fly-spline-through-set-framed = 沿一组取景视点飞行样条曲线。任何鼠标操作都会将其取消。
solar-moons-ride-circular-orbits-fitted = 卫星沿拟合到 JPL Horizons 的圆形轨道运行：与它们真实位置相差一到两度以内，而对于米兰达（Miranda）则最多相差六度，因为其轨道平面摆动太快，圆形无法跟随。
solar-fetch-everything-again-now = 立即重新获取所有内容
solar-all-sats = 所有卫星
solar-show-every-satellite-element-set = 显示轨道根数集中的每一颗卫星，而不仅是热门的几颗。轨道环仍只保留精选的少数——同时显示九十个会无法辨认。
solar-starting = 正在启动……
solar-body = 天体{" "}
solar-moon-orbit = 月球轨道{" "}
solar-stretch-earth-moon-distance-so = 拉伸地球到月球的距离，使这一对天体能够被分开看到
solar-sun = 太阳{" "}
solar-sun-radius-exaggeration-leave-1 = 太阳半径的夸大。保持为 1× 以使日冕物质抛射的几何关系清晰可读——膨胀的太阳会吞没每个锥体的底部。
solar-show = 显示
solar-paint-globe-what-getting-through = 按信号抵达情况为地球着色
solar-paints-globe-what-getting-through = 在每个频段上，按信号实际抵达的情况为地球着色。每一次接收都标在其路径的中点——即电离层发挥作用之处——而不是对方电台的位置。它由本电台所听到的内容构成，若开启了反向信标网络，还包括全球 skimmer 的数据。
solar-display = 显示
solar-all-bands = 所有频段
solar-every-band-once-one-hue = 所有频段同时显示，每个频段一种色调，并在两者重叠之处混合颜色。这是回答“条件如何”的视图，而不是“二十米开放了吗”。
solar-one-band = 单个频段
solar-one-band-cold-hot-blue = 单个频段，由冷到热：蓝、绿、黄、红。
solar-band = 频段
solar-nothing-heard-yet = 尚未听到任何内容
solar-bands = 频段
solar-sources = 来源
solar-contacts-logbook-path-was-open = 日志中的通联。路径是开放的，但没有值得一提的信号报告——RST 不是 SNR，因此这些通联只计入某个小区的繁忙程度，从不计入其余量。
solar-stations-reported-hearing-one-downloaded = 报告收听到本电台的电台，从 WSPRnet 下载。这是发射信标唯一能得到的反馈。
solar-reverse-beacon-network-skimmers-what = 反向信标网络的 skimmer：每个频段上其他所有人正在听到的内容，包括本电台并未收听的频段。它比其他数据粗略——RBN 的记录不带网格定位，因此双方都被放在其所在国家的中心，对小国而言相当接近，对大国则偏差很大。在“设置 → 信号”下开启。
solar-decodes-station-made-each-measured = 本电台做出的解码。每一项都按其自身模式的底噪进行测量，因此 WSPR 报告与 FT4 报告可以相互比较。
solar-memory = 记忆
solar-half-life = 半衰期{" "}
solar-how-long-reception-takes-count = 一次接收需要多长时间，其计入的权重才会减半。电离层自身的记忆很短——两小时前的开放不应与两分钟前的开放争论不休。
solar-live = 实时
solar-follow-band-happens = 随频段的变化实时跟随
solar-replay = ⏸ 回放
solar-replay-2 = ▶ 回放
solar-replay-last-hour-decodes-over = 反复回放过去一小时的解码
solar-min-ago = {" "}分钟前
solar-where-last-hour-globe-showing = 地球当前显示的是过去一小时中的哪个时刻
solar-trail = 轨迹{" "}
solar-how-long-decode-s-arc = 一次解码的弧线在磁头身后停留在地球上的时长
solar-speed = 速度{" "}
solar-how-much-faster-than-real = 回放比实时快多少
solar-qso-layer-off = 通联图层已关闭
solar-no-decodes-yet = 尚无解码
solar-qth-not-set-enter-your = 未设置 QTH——请在“设置”中输入你的网格定位
solar-mode = 模式
solar-path = 路径
solar-sent = 发送
solar-rcvd = 接收
solar-grid = 网格
solar-dxcc = DXCC
solar-geostationary-always-above-your-horizon = 静止轨道——始终位于你的地平线之上。
solar-no-passes-next-48-hours = 从你的 QTH 看，未来 48 小时内没有过境。
solar-start = 开始
solar-max-el = 最大仰角
solar-aos-los-are-azimuths-horizon = AOS/LOS 是地平线上的方位角。时间为 UTC。
solar-locked-unlock = {" "}● 已锁定 — 解锁{" "}
solar-release-satellite-lock = 释放卫星锁定
solar-lock = {" "}锁定{" "}
solar-track-satellite-main-window-doppler = 在主窗口中跟踪此卫星：经多普勒校正的收发、按转发器映射的上行，以及旋转器转向
solar-no-frequencies-file-one-add = 此卫星没有已存档的频率——请在“设置 ▸ TLE”中添加。
solar-frequencies-yours = 频率 · 你的
solar-frequencies = 频率
solar-link = 链路
solar-downlink-mhz = 下行（MHz）
solar-uplink-mhz = 上行（MHz）
solar-doppler-shifts-these-few-khz = 在一次 LEO 过境期间，多普勒频移会使它们偏移几 kHz。
solar-satellite-planet-comet = 卫星、行星或彗星
solar-planet-asteroid-comet = 行星、小行星或彗星
solar-clear-search = 清除搜索
solar-no-match = 无匹配
solar-no-sounder = 无探测器数据
solar-heard = 收听 ≥
solar-kp = Kp / A
solar-x-ray = X 射线
solar-observed-above-sounder-better-than = 探测器观测值之上——优于模型
solar-power-n-s = 功率 N/S
solar-edge-n-s = 边缘 N/S
solar-kp-peak-24-h = 24 小时 Kp 峰值
solar-observed-forecast = 观测 | 预报
solar-bands-open = 开放的频段
solar-dxcc-coverage = DXCC 覆盖
solar-ir-vis = 红外+可见光
solar-ir-only = 仅红外
solar-no-deep-convection = 无深对流
solar-t-1-storm = 1 次风暴
solar-glancing = {" "}· 擦过
solar-direction-estimated = {" "}· 方向为估算值

## Digital-mode setup dialogs (src/app/panels/setup.rs)

setup-aprs-setup = APRS 设置
setup-packet-setup = 分组设置
setup-ft8-ft4-ft2-setup = FT8 / FT4 / FT2 设置
setup-my-callsign = 我的呼号
setup-my-grid = 我的网格
setup-station-call = 电台呼号
setup-ssid-oe3jjs-10-packet-station = 带 SSID——OE3JJS-10。分组电台通常使用与操作员不同的 SSID，而且在此设置之前不会发射任何内容。
setup-speed = 速率
setup-t-9600-needs-radio-s-data = 9600 需要电台的数据端口。麦克风和扬声器通路会在两端破坏它——这是电台的问题，而非 sdroxide 的。
setup-tx-delay = 发射延迟
setup-flags-sent-before-frame-so = 在一帧之前发送标志，使对方能够听到我们并锁定其时钟。在 CAT 电台上，仅 sdroxide 自身就要花 165–240 ms 才能发射，电台自身的发射就绪时间还要再加上去。
setup-packet-length = 分组长度
setup-bytes = {" "}字节
setup-most-single-frame-carries-shorter = 单帧所能承载的最大长度。较短的帧能在勉强可用的路径上存活——只需重发被击中的那一帧——但开销更大。HF 上为 128，路径可靠时为 256。
setup-frames-sent-before-waiting-acknowledgement = 等待确认之前发送的帧数。越多越能充分利用良好的路径；在糟糕的路径上，则意味着更多需要重发的内容。
setup-extended = 扩展
setup-ask-mod-128 = 请求 mod-128
setup-extended-sequence-numbers-window-bigger = 扩展序列号，用于窗口大于七的情况。许多节点会以 DM 拒绝该请求，这看起来与一个不愿与你通话的电台完全相同——因此除非你确知对方需要它，否则请保持关闭。
setup-answer-calls = 应答呼叫
setup-accept-connections = 接受连接
setup-off-winlink-client-which-dials = 对于 Winlink 客户端应保持关闭，因为它是拨出的，没有理由应答。开启后则可作为对等方被访问：呼叫会出现在终端面板中，你即与发起呼叫的一方通话。若某个电台的链路已在进行 Winlink 会话，它会在会话结束前拒绝呼叫。
setup-connect-text = 连接文本
setup-what-caller-greeted = 呼叫者所收到的问候语
setup-sent-station-connects-us-once = 链路建立后，发送给连接到我们的电台。留空则什么也不发送——这会让呼叫者以为出了故障。
setup-default-via = 默认经由
setup-oe3xlr-1-oe3xms-1 = OE3XLR-1,OE3XMS-1
setup-digipeater-path-terminal-starts-path = 终端开始时使用的数字中继路径。通往你本地节点的路径每次都相同，而重新输入它正是某一跳被遗漏的原因。
setup-beacon = 信标
setup-t-0-disables-timer = 0 表示禁用计时器
setup-beacon-text = 信标文本
setup-kiss-server = KISS 服务器
setup-serve = 服务
setup-offer-modem-kiss-tnc-socket = 将此调制解调器作为 KISS TNC 通过套接字提供，使 Pat、APRS 客户端或 Linux AX.25 协议栈都能使用电台。
setup-aprs-call = APRS 呼号
setup-callsign-station-beacons-under-ssid = 此电台以其发射信标所用的呼号，连同其 SSID——汽车用 OE3JJS-9，I-gate 用 -10，手机用 -5。留空则使用上面的呼号。
setup-no-callsign-station-will-not = 没有呼号：此电台将完全不发射——既不发射信标，也不发送消息，也不发送确认。
setup-symbol = 符号
setup-two-characters-protocol-carries-them = 协议所携带的两个字符：一个表（`/` 或 `\`）和一个符号代码。第一位的数字或字母是叠加层，会绘制在备用表图标之上。
setup-path = 路径
setup-wide1-1-wide2-1 = WIDE1-1,WIDE2-1
setup-how-far-you-ask-repeated = 你要求被中继的距离。这是该信道上影响最大的单一设置：每一跳都会使整个网络在同一个共享频率上的发射次数成倍增加。
setup-position = 位置
setup-my-grid-2 = 来自我的网格
setup-beacon-centre-locator-above-reported = 以上面网格定位的中心为信标位置，并附上网格定位实际具有的模糊度来报告——六位字符的网格定位跨度约为两公里，如实说明这一点是诚实的做法。取消勾选则改为给出精确坐标。
setup-latitude = 纬度
setup-longitude = 经度
setup-locator = 来自网格定位
setup-not-coordinate = 不是坐标
setup-comment = 备注
setup-sent-every-beacon = 每次信标都会一同发送
setup-t-0-default-never-beacons-timer = 0——默认值——从不按计时器发送信标。三十分钟是固定电台的惯例；移动电台发送得更频繁，但每一次信标都占用别人的信道时间。
setup-compressed = 压缩
setup-compressed-position-format-third-air = 压缩位置格式：空中时间只需三分之一，而且更精确。自 1990 年代以来的每一台接收机都能读取它。
setup-first-goes-out-one-interval = 第一次信标会在从现在起一个间隔后发出，而不是立即发出。
setup-messages = 消息
setup-acknowledge = 确认
setup-answer-messages-addressed-you-acknowledgement = 应答发给你的消息。确认是本电台在无人请求时发出的发射，因此仅接收的设置应将其关闭——这样信标也就不再声称可以被联系到。
setup-flags-sent-ahead-every-frame = 在每一帧之前发送的标志，使对方的调制解调器能在数据开始之前听到载波并锁定其时钟。它必须长于从按下发射到辐射出去之间的全部时间：仅 sdroxide 自身在 CAT 电台上就要占用其中的 165–240 ms，而通过网络接收音频的电台还会在此之上再加缓冲。设得太短，对方永远无法锁定——发射确实存在，却没有任何东西能将其解码。
setup-shared-packet-mode = 与分组模式共用
setup-keep-stations = 保留电台
setup-how-long-station-stays-map = 电台最后一次被听到之后，会在地图上保留多久。这也是地图淡出所依据的时间，因此较短的窗口只显示当前活跃的内容。
setup-auto-reply = 自动应答
setup-answer-snr-grid-status = 应答 SNR? / GRID? / STATUS?
setup-answer-direct-question-addressed-you = 应答发给你或发给 @ALLCALL 的直接问题，以答复代替确认。从不应答其他电台的通信，也从不应答它自己。
setup-heartbeat = 心跳
setup-off = 关
setup-turbo-does-not-beacon-local = Turbo 不发送信标——它是本地和 VHF 的速率。
setup-sends-your-callsign-grid-so = 发送你的呼号和网格，使他人知道你可以被接收。第一次会在从现在起一个间隔后发出，而不是立即发出。
setup-beacon-frequency = 信标频率
setup-t-500-1000-hz = 500–1000 Hz
setup-move-each-beacon-free-slot = 与 JS8Call 的做法一样，将每个信标移至心跳子频段中的空闲时隙：监听信标的电台正是在那里寻找它，而且这能使无人值守的发射机不干扰别人的通联。时隙在信标真正发出时选定，并避开所有正在解码的内容。
setup-working-freq = 工作频率
setup-beacon-where-you-are-working = 改为在你正在工作的频率上发送信标。这与频段惯例相悖，但能使你所发射的一切都集中在一处。
setup-heartbeat-reply = 心跳应答
setup-answer-heartbeats-signal-report = 以信号报告应答心跳
setup-tell-station-beaconed-how-well = 告诉发送信标的电台你接收它的效果如何。默认关闭：繁忙的频段每个时隙都会有心跳，而全部应答会淹没正是心跳所用来保持安静的频段。每个电台每 15 分钟只应答一次，并且在仍有消息到达时，或你有待发送的内容时，绝不应答。
setup-status-message = 状态消息
setup-tx-period = 发射时段
setup-even = 偶数
setup-odd = 奇数
setup-auto-sequence = 自动序列
setup-decode-depth = 解码深度
setup-fast = 快速
setup-normal = 标准
setup-deep = 深度

    Fast — one pass, no signal subtraction: quickest, fewest decodes.
    Normal — flat multi-pass subtraction: a little quicker than Deep, a little less thorough.
    Deep — the checkpointed multi-pass: the most decodes, and ~1.2 s on a busy slot.
setup-auto-tx-frequency = 自动发射频率
setup-choose-transmit-frequency-automatically-quietest = 自动选择发射频率：在你发射的时段中最安静的位置，而不是你所应答的电台的频率。关闭时并不会保持该频率——它会在被呼叫电台的频率上应答。若要保持，请使用“保持发射频率”。
setup-hold-tx-frequency = 保持发射频率
setup-pin-transmit-tone-where-nothing = 将发射音调固定在当前位置。没有任何东西会移动它：应答电台、呼叫队列、呼叫 CQ、点击解码或瀑布图都不会。它会覆盖“自动发射频率”。更换频段是唯一的例外：你在新频段上最后设定的偏移会随之恢复。若你的执照比频段计划更窄——例如在英国 60 米的 5357 kHz 表盘上，分配在 5358.0 结束，因此音调必须保持在 1000 Hz 以下。
setup-tx-watchdog = 发射看门狗
setup-stop-transmitting-after-long-no = 在这么长时间没有回复且你也没有任何操作之后，停止发射。0 表示禁用。
setup-give-up-after = 在以下次数后放弃
setup-calls = {" "}次呼叫
setup-contest = 比赛
setup-everyday-exchange-grid-report-rr73 = 日常交换：网格、报告、RR73。
setup-european-vhf-contests-send-signal = 欧洲 VHF 比赛：发送信号报告、序列号以及你的六位网格定位，采用 WSJT-X 称为 EU VHF Contest 的消息格式。CQ 发送为 CQ TEST。
setup-serial = 序列号
setup-serial-number-next-exchange-will = 下一次交换将携带的序列号。它会随着每次通联被记录而自行递增，并在会话之间保留——只有在开始一场比赛，或要从纸质日志停下的地方接着记录时，才需手动设置。
setup-needs-6-character-grid = ⚠ 需要六位网格定位
setup-eu-vhf-contest-messages-carry = 欧洲 VHF 比赛消息携带的是六位网格定位（JN88DD，而不是 JN88）。在“我的网格”中设置好之前，将改为发送普通的交换内容。
setup-dxpedition = DXpedition
setup-ordinary-ft8-operation = 普通 FT8 操作。
setup-calling-dxpedition-running-fox-mode = 呼叫运行 Fox 模式的 DXpedition：在 1000 Hz 以上呼叫，当它应答时移至 Fox 的频率，并在它的 RR73 上记录通联，而不发送 73。
setup-run-pile-up-several-signals = 运行大批呼叫：同时发送多个信号，按队列接收呼叫者，最强和最稀有的优先通联。“呼叫 CQ”开始它，“停止通联”则使其停止。
setup-fox-signals = Fox 信号
setup-once = {" "}同时
setup-simultaneous-transmissions-spaced-60-hz = 同时发射，彼此相隔 60 Hz。它们共用发射机的功率，因此信号越多，每个信号就越弱。
setup-tx-audio = 发射音频
setup-deviation-fm = 频偏，FM
setup-drive-ssb = 驱动，SSB
setup-grid = 网格
setup-report = 报告
setup-r-report = R+报告

## AMP window (src/app/amp.rs)

amp-t-10-w-scale-rescaled = ，10 W 刻度，已重新缩放
amp-t-10-w-scale = ，10 W 刻度
amp-t-2-w-scale-rescaled = ，2 W 刻度，已重新缩放
amp-t-2-w-scale = ，2 W 刻度
amp-t-100-w-scale-rescaled = ，100 W 刻度，已重新缩放
amp-t-100-w-scale = ，100 W 刻度
amp-t-20-w-scale-rescaled = ，20 W 刻度，已重新缩放
amp-t-20-w-scale = ，20 W 刻度
amp-rescaled = ，已重新缩放
amp-and = {" "}与{" "}
amp-not-while-transmitting-switching-relay = 发射期间不可用：在有射频的情况下切换继电器可能会损坏它。请先松开发射。
amp-no-amplifier-set-up-choose = 尚未设置功放。请在“设置 → 功放”中选择其端口。
amp-settings = 设置
amp-dock = 停靠
amp-put-amp-window-back-inside = 将功放窗口放回主窗口内部
amp-pop-out = 弹出
amp-give-amp-window-window-own = 为功放窗口提供其自己的窗口，以便移至另一块屏幕
amp-rig = ，来自电台
amp-rig-flagged-reading = ，电台标记了此读数
amp-drv = ，来自 DRV
amp-rig-s-swr = ，来自电台的驻波比
amp-ptt-keys-amplifier-radio-off = PTT 随电台一起按下功放；OFF 则使其保持旁路。点击即可切换。
amp-switch-antenna-jack = 切换天线插孔
amp-band-down = 降低频段
amp-band-up = 提高频段
amp-tuner = 调谐器
amp-tuned = 已调谐
amp-tune = 调谐
amp-start-tune-then-key-steady = 开始调谐——然后按下稳定的载波（CW、FM 或 RTTY）。再次按下即停止。
amp-put-tuner-line-bypass = 将调谐器接入线路，或将其旁路
amp-tuning = 正在调谐……
amp-bottom = 底部
amp-pick-any-reading-either-needle = 为任一指针选择任意读数。上指针为白色，下指针为蓝色。
amp-forward-power-measured-radio-amplifier = 正向功率，由电台测量：功放在 OFF 状态下不报告此项
amp-forward-power-taken-drv-amplifier = 正向功率，取自 DRV：功放在 OFF 状态下不报告此项，而在旁路状态下，电台的功率直接通过
amp-forward-power = 正向功率
amp-reflected-power-measured-radio-amplifier = 反射功率，由电台测量：功放在 OFF 状态下不报告此项
amp-reflected-power-worked-out-fwd = 反射功率，根据 FWD 和驻波比计算：功放在 OFF 状态下不报告此项
amp-reflected-power = 反射功率
amp-standing-wave-ratio-red-past = 驻波比：达到或超过调谐上限（“设置 → 常规”）时显示为红色。请检查天线
amp-standing-wave-ratio-measured-radio = 驻波比，由电台测量：功放在 OFF 状态下不报告此项
amp-standing-wave-ratio = 驻波比
amp-drive-radio = 来自电台的驱动
amp-drain-supply = 漏极电源
amp-drain-current = 漏极电流
amp-temp = 温度
amp-heat-spreader-temperature-yellow-49 = 散热片温度：49 °C（120 °F）起为黄色，超过 65 °C（149 °F）为红色，高于 70 °C（158 °F）则脱离线路。点击可切换 °C/°F。
amp-c = °C
amp-f = °F
amp-performance = 性能
amp-numbers-hold-peaks-bars-are = 数字保持峰值 · 条形为实时
amp-input-power = 输入功率
amp-supply-volts-drain-amps = 电源电压 × 漏极电流
amp-efficiency = 效率
amp-output-power-input-power = 输出功率 ÷ 输入功率
amp-gain = 增益
amp-output-power-over-drive-decibels = 输出功率相对于驱动的增益，单位为分贝
amp-off-line-over-temperature = 已脱离线路 · 温度过高
amp-connected-transmitting = 已连接 · 发射中
amp-connected = 已连接
amp-connecting = 正在连接……
amp-atu-bypassed = ATU 已旁路
amp-no-inductance-needed = 无需电感
amp-no-capacitance-needed = 无需电容
amp-c-out-antenna-side = C OUT · 天线侧
amp-c-transmitter-side = C IN · 发射机侧

## SSTV panel (src/app/panels/sstv.rs)

sstv-chunks-received-lit-still-missing = 已接收的数据块（亮）和仍缺少的数据块（暗）
sstv-sstv = SSTV
sstv-auto = 自动
sstv-signal = 信号
sstv-station-sending-mode-sdroxide-does = 有电台正在发送一种 sdroxide 不支持的模式。信号和调谐都正常——只是没有针对此模式的解码器。你这一端无需做任何处理。
sstv-listening = 正在收听……
sstv-restart-rx = 重新开始接收
sstv-abandon-picture-being-received-listen = 放弃正在接收的图片，并收听下一个帧头。适用于以错误模式开始解码的发射——否则接收机会一直锁定在该模式上，直到它结束。
sstv-callsign-last-station-sent-fsk = 上一个电台在图片之后以 FSK ID 发送的呼号。
sstv-tx-slant = 发射倾斜
sstv-transmit-clock-trim-ppm-remove = 发射时钟微调（ppm），用于消除对方解码器上的倾斜
sstv-reset-0-ppm = 重置为 0 ppm
sstv-fsk-id = FSK ID
sstv-send-your-callsign-tones-after = 在每张图片之后以音调发送你的呼号——即 SSTV 中继台和其他程序读取的识别信息。会增加约 2.5 秒，并且在你设置呼号之前什么也不发送。
sstv-tx-lead = 发射提前量
sstv-silence-sent-after-keying-before = 按下发射之后、图片的引导信号和 VIS 码之前所发送的静音。若解码器漏掉此帧头中的任何部分，就根本显示不出图片，因此它覆盖了向电台请求 PTT 与电台真正发射之间的间隔。对于即时键控的 SDR，设为 0。
sstv-live = 实时
sstv-waiting-picture = 等待图片……
sstv-no-low-audio = 无音频 / 音频过低
sstv-received = 已接收
sstv-reading-radio-s-pictures = 正在读取电台的图片……
sstv-decoded-pictures-collect-here = 解码的图片会收集在这里。
sstv-click-enlarge-right-click-delete = 点击放大 · 右键删除
sstv-delete-picture = 删除此图片
sstv-loading = 加载中……
sstv-transmit = 发射
sstv-click-edit-slot-s-message = 点击编辑此时隙的消息 · 双击加载图片
sstv-change-image = 更换图片……
sstv-load-image = 加载图片……
sstv-clear = 清除
sstv-empty-slot-s-picture-message = 清空此时隙的图片（消息保留）
sstv-banner = 横幅……
sstv-what-printed-across-top-every = 本电台发送的每张图片顶部所印的内容
sstv-fetching-slot-s-picture = 正在获取此时隙的图片……
sstv-load-image-into-slot = 将图片加载到此时隙 →
sstv-drawn-slot-s-image = 绘制在此时隙的图片上
sstv-abort-tx = {" "}中止发射{" "}
sstv-delete-picture-store = 从存储中删除此图片
sstv-received-image = 接收的图片
sstv-no-longer-store = 已不在存储中
sstv-loading-full-size = 正在加载原始尺寸……
sstv-save-picture = 保存图片……
sstv-delete-sure = 删除——确定吗？
sstv-click-again-delete-good = 再次点击即永久删除
sstv-delete = 删除……
sstv-image-banner = 图片横幅
sstv-drawn-into-every-picture-station = 绘制在本电台发送的每张图片中，位于此时隙自身消息之上。
sstv-draw-banner = 绘制横幅
sstv-substituted-when-picture-composed = 图片合成时替换：
sstv-top-left = 左上
sstv-top-right = 右上
sstv-colours = 颜色
sstv-strip-top-edge-fades-black = 条带，在其顶边——它渐变为黑色，或渐变为其下方的渐变色
sstv-both-texts = 两段文字
sstv-gradient = 渐变
sstv-fade = 渐变至
sstv-fade-strip-top-colour-second = 将条带从其顶部颜色渐变为第二种颜色，而不是渐变为黑色。
sstv-text-outline = 文字描边
sstv-draw-outline-around-banner-text = 为横幅文字绘制描边，使其在繁杂或浅色的条带上仍然清晰可读。
sstv-text-gradient = 文字渐变
sstv-fade-banner-text-colour-second = 在图片宽度范围内，将横幅文字从其颜色渐变为第二种颜色。
sstv-rainbow-text = 彩虹文字
sstv-all-text-overriding-colours-above = 所有文字，覆盖上面的颜色
sstv-draw-every-text-picture-banner = 将图片中的每段文字——横幅和消息——绘制为水平彩虹色，忽略上面的颜色。
sstv-message = 消息
sstv-slot-message-s-text-colour = 时隙消息的文字颜色
sstv-outline = 描边颜色
sstv-height = 高度
sstv-how-tall-strip-pixels-transmitted = 条带的高度，以所发射图片的像素计。文字的大小由它决定——SSTV 帧只有 320 像素宽，因此更高的横幅才能使对方读清楚。
sstv-reset = 重置
sstv-back-callsign-left-version-right = 恢复为左侧的呼号和右侧的版本
sstv-waiting-station-s-digital-mode = 正在等待电台的数字模式设置加载……
sstv-rifp = RIFP
sstv-size = 大小
sstv-calling-frequency-draft-names = 草稿中所指定的呼叫频率
sstv-encode = 编码
sstv-how-picture-encoded-into-object = 图片如何编码进 RIFP 所携带的对象中。“自动”会逐一尝试，并发送最小的一个。
sstv-try-every-encoding-send-smallest = 尝试每一种编码，发送最小的一个（从不有损）
sstv-gray = 灰度
sstv-grayscale-depth-rifp-s-raster = 灰度位深。按定义，RIFP 的光栅是灰度的——其清单中没有颜色的位置。
sstv-dither = 抖动
sstv-diffuse-quantisation-error-worth-below = 扩散量化误差——在低于 8 位时值得使用
sstv-caption = 说明
sstv-sent-manifest-content-hint-shown = 作为内容提示发送到清单中，并由接收方显示在图片下方。以文本形式传输——它不会绘制在图像中。
sstv-what-picture = 这张图片是什么
sstv-repeat-data = 重复数据
sstv-send-every-data-frame-many = 每个数据帧发送这么多次。RIFP 是单向的，没有修复请求，因此这是接收方唯一的恢复手段。
sstv-chunk = 数据块
sstv-payload-octets-per-data-frame = 每个数据帧的有效载荷字节数（该配置建议为 192）
sstv-valid-frames-frames-failed-crc = 有效帧、CRC 校验失败的帧，以及已完整验证的图片
sstv-no-transfer-progress = 没有正在进行的传输
sstv-forget-incomplete-transfer = 放弃此未完成的传输

## Memories and voice keyer windows (src/app/windows.rs)

windows-recall = 调用
windows-delete = 删除
windows-edit-name-frequency-mode-antenna = 编辑名称、频率、模式、天线和中继设置
windows-r = {" "}R
windows-repeater-shift-tone-1750-hz = 与此信道一同保存的中继偏移、音调和 1750 Hz 脉冲
windows-abandon-edit-escape = 放弃编辑（Esc）
windows-save = 保存
windows-keep-changes-enter = 保留更改（Enter）
windows-recall-channel-without-moving-antenna = 调用此信道，而不移动天线
windows-antenna-switch-when-channel-recalled = 调用此信道时要切换到的天线。— 保持电台当前所在的天线不变。
windows-shift = 偏移
windows-tone = 音调
windows-invert = 反相
windows-normal = 正常
windows-dcs-polarity = DCS 极性
windows-open-every-over-tone-burst = 调用此信道期间，每次通话都以音调脉冲打开中继
windows-memories = 记忆信道
windows-memory-name = 记忆名称
windows-store = 存储
windows-folder-name = 文件夹名称
windows-new-folder = 新建文件夹
windows-import = 导入
windows-read-channel-list-chirp-csv =
    从 CHIRP CSV 文件（.csv）读取信道列表——中继台目录导出、海事或 PMR 信道表，或 CHIRP 能够写出的任何其他内容。

    Each channel brings its frequency, mode, repeater shift and CTCSS or DCS tone with it. Channels already on this list — same frequency, same mode — are skipped, so re-importing an updated directory adds what is new rather than doubling what is not.
windows-chirp-csv = CHIRP CSV
windows-export = 导出
windows-write-list-out-chirp-csv = 将此列表写出为 CHIRP CSV 文件，用于载入手持机或作为备份保存。
windows-no-memories-yet = 尚无记忆信道
windows-drag-memory-onto-folder-file = 将记忆信道拖到文件夹上即可归档，拖到它们下方即可取消归档
windows-drop-here-unfile = 放在此处即取消归档
windows-sort = 排序
windows-order-they-were-stored = 按存储顺序
windows-name-ignoring-case = 按名称（不区分大小写）
windows-frequency = 按频率
windows-band-then-frequency-inside-each = 按频段，每个频段内再按频率
windows-descending-click-ascending = 降序——点击切换为升序
windows-ascending-click-descending = 升序——点击切换为降序
windows-delete-folder-memories-move-top = 删除文件夹——其中的记忆信道将移至顶层
windows-rename-folder = 重命名文件夹
windows-empty-drop-memories-here = 为空——将记忆信道拖到此处
windows-voice-keyer = 语音键控器
windows-rec-records-your-microphone-play = 录制从你的话筒录音，播放让你听到所录的内容，发射则将其送上空中——数字小键盘按键、MIDI 打击垫或 rigctld 的 send_voice_mem 同样可以触发。
windows-stop-store = 停止并存储
windows-stop = 停止
windows-play = 播放
windows-stop-listening = 停止收听
windows-nothing-recorded-slot = 此时隙中没有录音
windows-not-while-transmitting = 发射期间不可用
windows-listen-message-nothing-transmitted = 收听此消息——不会发射任何内容
windows-stop-transmitting = 停止发射
windows-tune-active-switch-off-first = 调谐处于激活状态——请先将其关闭
windows-transmit-message = 发射此消息
windows-erase-recording = 擦除此录音
windows-rade-message-encoded-digital-voice = RADE：消息由数字话音编解码器编码，与实时通话完全相同。

## TX Audio window: messages with values

txaudio-rec-hint = 从话筒录制一段测试短语——最长 { $max } 秒，无需按下发射机。再次点击即停止。它保持未处理状态，因此“播放”可以让你每次更改后都用同一段短语进行比较。
txaudio-phrase-length = { $secs } 秒短语
# $clip is txaudio-microphone-may-clipping-before-gets or empty.
txaudio-auto-set-done = 输入增益已设为 { $gain } dB（话音峰值为 { $loud } dBFS）。{ $clip }
txaudio-listening = 正在收听…… { $secs } 秒
txaudio-auto-set-hint = 请像在空中那样正常说话 { $secs } 秒，输入增益将被设定为使你的话音峰值约为 { $target } dBFS。再次点击即取消。
txaudio-eq-point = { $hz } Hz，{ $db } dB——拖动可移动
txaudio-not-a-profile = 不是发射音频配置文件：{ $error }
txaudio-imported = 已导入“{ $name }”。
txaudio-saved = 已保存“{ $name }”。
txaudio-deleted = 已删除“{ $name }”。
txaudio-loaded-preset = 已加载“{ $name }”起始预设。

## Solar-system 3D view: messages with values

solar-age-offline = { $age } · 离线
solar-radius-exaggeration = 地球/月球半径夸大（在此月球轨道比例下最大为 { $max }×）
solar-brightest-cell = 最亮的方格约 { $peak } 条路径 · { $bands } 个频段
solar-hits-in-hour = 一小时内 { $hits } 次
solar-passes-from = { $name } · 来自 { $grid } 的过境
solar-elements-age = 轨道根数已过 { $age } · SGP4
solar-point-at = 指向方位 { $az }°（{ $compass }），仰角 { $el }°。
solar-published-as = 发布为 { $name }
solar-fly-there = { $name } — 按 ↵ 飞往该处
solar-sats-tracked = 已跟踪 { $total } 颗中的 { $n } 颗
solar-bodies-found = 已找到 { $total } 个天体中的 { $n } 个
solar-sats-and-bodies = 已跟踪 { $total } 颗中的 { $n } 颗 · { $bodies_total } 个天体中的 { $bodies } 个
solar-confidence-on-band = { $band } 上的 { $confidence }
solar-kp-now = 当前 Kp { $kp }
solar-kp-in = { $hours } 小时后 Kp { $kp }
solar-valid-age = 有效期 { $valid } · 已过 { $age }
solar-full-bar = 满条 = 全球的 { $share }
solar-own-paths-memory = 本电台自身路径的走向 · { $minutes } 分钟记忆
solar-n-storms = { $n } 次风暴
solar-clouds-legend = 云层 { $time } · 已过 { $age } · { $channels } · { $storms } · 闪电为模拟
solar-cme-eta = 预计 { $eta }（+{ $hours } 小时）
solar-cme-arrived = 已于 { $eta } 到达（{ $hours } 小时前）
# $glancing and $estimated are solar-glancing / solar-direction-estimated or empty.
solar-cme-earth-directed = 指向地球的日冕物质抛射 { $time } · { $speed } km/s · { $when }{ $glancing }{ $estimated }

## Digital-mode setup dialogs: messages with values

setup-mode-setup = { $mode } 设置
setup-transmits-as = 以 { $call } 发射
setup-decimal-degrees = 十进制度，{ $hemisphere } 为负。可直接从地图上粘贴。

## AMP window: messages with values

amp-unreliable-at-swr = { $meters } 在此驻波比下不可靠
amp-lower-drive-first = 请先降低驱动：{ $drive } W 会使接入线路的功放过驱动（其限值为 { $limit } W）。
amp-live-value = 实时 { $value } { $unit }
amp-tripped = 已触发{ $why }

## Digital-mode panels, shared: messages with values

panels-top-paths = 最多约 { $n } 条路径
panels-no-agreed-frequency = { $mode } 没有约定的频率——打开此面板以保存你当前所在的频率
panels-mode-frequencies-hint = { $mode } 的 { $n } 个频率——选择其中一个即可调谐表盘，你也可以保存自己的频率
panels-mode-frequencies = { $mode } · 频率
panels-global-convention = 一项全球惯例，但 IARU { $region } 区域的频段计划并未将窄带数据放在此处——在此发射前请核对你自己的频段计划。
panels-outside-data-segment = 琥珀色：位于 { $region } 区数据子频段之外。
panels-save-dial = ＋ 保存 { $mhz } MHz
panels-save-dial-hint = 以 { $mode } 记住此表盘频率，并从现在起在此处提供它。保存在电台站点上，因此连接到此电台的每个屏幕都有它。
panels-tr-period = { $period } 秒收发周期
# $mark is panels-burst-mark while transmitting, otherwise empty.
panels-this-turn = 本轮：{ $slot } 秒时隙中的 { $into } 秒。解码结果在其末尾出现，而那正是下一次发射可以开始的时候。{ $mark }
panels-burst-mark = {" "}标记是脉冲停止的位置，位于 { $secs } 秒处。
panels-tr-period-burst = { $slot } 秒收发周期，{ $burst } 秒脉冲

## Memories and voice keyer windows: messages with values

windows-n-channels = { $n } 个信道
windows-slot-n = 时隙 { $n }
windows-record-from-mic = 从话筒录制（最长 { $max } 秒）

## SSTV panel: messages with values

sstv-picture-too-big = 该图片为 { $size } MB——限制为 { $limit } MB。
sstv-auto-mode = 自动（{ $mode }）
sstv-not-decoded = { $what }——未解码
sstv-last = 上一个：{ $mode }
# $place is sstv-store-on-radio or sstv-store-in.
sstv-saved-where = 解码的图片保存{ $place }，并收集在此处。
sstv-older-in-store = 存储中另有 { $n } 张更早的图片
sstv-older-load-more = 另有 { $n } 张更早的图片——加载更多
sstv-delete-picture-from = 从{ $place }删除此图片
sstv-rifp-profile = { $profile } · 占用约 { $khz } kHz · 表盘为信道中心
sstv-rifp-too-wide = ⚠ { $profile } 占用约 { $khz } kHz——对于窄带段来说太宽
sstv-rifp-no-frequency = RIFP 不指定频率，sdroxide 会将其在你调谐到的任何地方发射。一个 { $khz } kHz 的信道只适用于允许宽带或 FM 操作的地方——{ $segments }——而不适用于窄带段，在 HF 上尤其如此。即使在这些地方，你自己的执照条件也可能更窄。你是操作员；请核对你自己的规则。
sstv-rifp-tx-frame = ● 发射帧 { $frame }/{ $frames } · 剩余 { $secs } 秒
sstv-rifp-sent-as = 以 { $encoding } 发送 · { $octets } 字节
sstv-rifp-rx-stats = 帧 { $frames } · 错误 { $bad } · 图片 { $pictures }
sstv-rifp-session-idle = 会话 { $session } · 空闲 { $secs } 秒
sstv-rifp-session-waiting = 会话 { $session } · 已保留数据块，仍在等待清单 · 空闲 { $secs } 秒
sstv-store-on-radio = 在电台上，位于 { $dir }
sstv-store-in = 位于 { $dir }

## decodes (src/app/panels/decodes.rs)

decodes-decodes = 解码
decodes-adif = ADIF
decodes-sort = 排序
decodes-single-list = 单一列表
decodes-every-decode-one-list-newest = 所有解码放在一个列表中，最新的时段在前，而不是按奇偶时段分块。此时“排序”按钮排列的是整个列表而非每个时段，并且每一行都带有其时隙时间——偶数时隙为青色，奇数时隙为金色。
decodes-cq-only = 仅 CQ
decodes-only-stations-calling-cq-only = 仅显示呼叫 CQ 的电台——并且只显示你可以应答的呼叫：定向 CQ（DX、EU、JA、POTA、TEST 等）在指名你时才会列出，指名他人时则会隐藏。
decodes-new-only = 仅新的
decodes-only-stations-would-new-entity = 仅显示会成为新内容的电台：实体、频段时隙、网格或呼号
decodes-auto-tx-frq = 自动发射频率
decodes-overridden-hold-tx-lift-hold = 已被“保持发射”覆盖。请取消保持，以选择发射频率的移动方向。
decodes-hold-tx = 保持发射
decodes-pin-transmit-tone-where-nothing =
    将发射音调固定在当前位置。没有任何东西会移动它：应答电台、呼叫队列、呼叫 CQ、点击解码或瀑布图都不会。若要移动，请先将其关闭，然后再开启。

    Changing band is the one exception, and it is your own act: the offset you last set on the new band comes back with it, so a 60 m figure is not carried onto 20 m and 20 m's is not carried onto 60 m.

    For where the licence is narrower than the band plan — on a UK 60 m dial of 5357 kHz the allocation ends at 5358.0, so the tone has to stay under 1000 Hz, and either automatic mover will walk out of the band between one over and the next.
decodes-type-transmit-offset-hz-then = 以赫兹输入发射偏移，然后按 Enter。若你的执照比频段计划更窄，则边缘以下的整个范围都由你自己选择，因此此处不提供任何建议。
decodes-clear-rx = {" "}清空接收{" "}
decodes-empty-decode-list-nothing-air = 清空解码列表。空中的任何内容都不会停止。
decodes-even = 偶数
decodes-text = 文本
decodes-dxcc = DXCC
decodes-band = 频段
decodes-grid = 网格
decodes-dupe = 重复
decodes-reply = 应答
decodes-queued-click-remove = 已排队——点击即可移除
decodes-work-station-after-current-one = 在当前电台之后与此电台通联
decodes-qsos-worked-since-sdroxide-was = 自 sdroxide 启动以来完成的通联
decodes-qso-complete = ✓ 通联完成
decodes-tx = ● 发射
decodes-dxpedition-mode-transmit-frequency-held = DXpedition 模式。发射频率被保持在通带中 Fox 所占的那一半之外（1000 Hz 以下），直到 Fox 应答为止。
decodes-watchdog = 看门狗
decodes-transmitting-stopped-no-reply-no = 发射已停止：在看门狗周期内既无回复也无操作。请呼叫 CQ 或选择一条消息以恢复。
decodes-well-inside-tolerance = 完全在容差范围之内。
decodes-no-active-qso-pick-decode = 没有进行中的通联——请选择一条解码以应答，或呼叫 CQ
decodes-ft8-engine-idle = FT8 引擎空闲
decodes-being-worked = 正在通联
decodes-clear = 清除
decodes-no-messages = — 无消息 —
decodes-call-cq = {"  "}呼叫 CQ{"  "}
decodes-stop-qso = {" "}停止通联{" "}
decodes-stop-tx = {" "}停止发射{" "}
decodes-free-text-13-chars = 自由文本（13 个字符）
decodes-send = 发送

## sat (src/app/sat.rs)

sat-pass-profile = 过境配置
sat-satellite = 卫星
sat-satellites = 卫星
sat-passes = 过境
sat-set-your-grid-passes-settings = 请设置你的网格以计算过境（设置 ▸ 常规）。
sat-next = 下一次
sat-peak = 峰值 ≥
sat-all-satellites = 所有卫星
sat-off-only-satellites-frequencies-file = 关闭：仅包含已存档频率的卫星——即你能够通联的那些——以及你自己粘贴的任何卫星。开启：列表中的每一颗卫星，若有订阅，可能多达数千颗，搜索需要一些时间。
sat-no-passes-window-lower-peak = 此窗口内没有过境。请降低峰值，或加宽窗口。
sat-length = 长度
sat-peak-2 = 峰值
sat-from-to = 自 → 至
sat-up-now = ● 当前在空
sat-unlock = 解锁
sat-release-lock-dial-stays-where = 释放锁定；表盘保持在原位
sat-azimuth = 方位角
sat-elevation = 仰角
sat-range = 距离
sat-rate = 速率
sat-downlink = 下行
sat-doppler-rx = 接收多普勒
sat-uplink = 上行
sat-doppler-tx = 发射多普勒
sat-no-pass-inside-next-48 = 从你的 QTH 看，未来 48 小时内没有过境。
sat-elements-stale-corrections-suspended-until = ⚠ 轨道根数已过时——在 TLE 刷新之前，校正已暂停
sat-doppler-correction-needs-iq-front = 多普勒校正需要 IQ 前端——CAT 电台只能跟踪
sat-rotator-not-connected = 旋转器：未连接
sat-doppler = 多普勒
sat-apply-computed-correction-rx-tx = 将计算出的校正应用于接收和发射
sat-rotator = 旋转器
sat-steer-antenna-through-rotctld-client = 通过 rotctld 客户端操控天线（设置 ▸ 服务器）
sat-name-norad = 名称或 NORAD
sat-set-your-grid-passes-settings-2 = 设置你的网格以计算过境（设置 ▸ 常规）
sat-norad = NORAD
sat-pass = 过境
sat-always-visible = 始终可见
sat-not-here = 此处不可见
sat-no-elements = 无轨道根数
sat-pick-satellite-see-links-lock = 选择一颗卫星，以查看其链路并锁定。
sat-no-frequencies-file-one-add = 此卫星没有已存档的频率——请在“设置 ▸ TLE”中添加。
sat-link = 链路
sat-downlink-mhz = 下行（MHz）
sat-uplink-mhz = 上行（MHz）
sat-mode = 模式
sat-tune = 调谐
sat-set-dial-mode-link-no = 将表盘和模式设为此链路——不锁定，不校正
sat-locked = {" "}已锁定{" "}
sat-lock = {" "}锁定{" "}
sat-track-satellite-doppler-correction-rx = 跟踪此卫星：对接收和发射进行多普勒校正，上行按转发器映射，三维视图随之跟随

## wefax (src/app/panels/wefax.rs)

wefax-wefax = WEFAX
wefax-stop = {" "}■ 停止{" "}
wefax-end-chart-now-save-what = 立即结束图表，并保存已接收的内容
wefax-start = {" "}● 开始{" "}
wefax-start-chart-now-without-waiting = 立即开始一张图表，无需等待开始音
wefax-phasing = 对相……
wefax-subcarrier-offset-1900-hz-tune = 相对于 1900 Hz 的副载波偏移。请调谐至大约为零：传真载波为 1500 Hz（黑）至 2300 Hz（白），若接收机偏差数百赫兹，图片就会被削成全黑或全白。
wefax-auto-start = 自动开始
wefax-auto-stop = 自动停止
wefax-end-450-hz-stop-tone = 在 450 Hz 停止音时结束。关闭后，对于连续发送多张图表的电台，会一直录制下去。
wefax-phase = 相位
wefax-slant = 倾斜
wefax-sample-clock-trim-if-chart = 采样时钟微调。若图表向左倾斜，请调高此值；若向右倾斜，则调低。时钟偏差一百 ppm 的声卡，在一张十五分钟的图表上会使大部分行发生横向偏移。
wefax-view = 视图
wefax-scale-chart-panel-width = 将图表缩放到面板宽度
wefax-whole = 整体
wefax-shrink-chart-until-all-view = 缩小图表，直至其全部同时可见
wefax-half-size = 半尺寸
wefax-one-screen-pixel-per-fax = 每个传真像素对应一个屏幕像素——滚动以查看细节
wefax-twice-size-scroll-move-around = 两倍尺寸；滚动以在图表中移动
wefax-height = 高度
wefax-stretch-picture-vertically-chart-came = 垂直拉伸图片。若图表被压扁或拉长，通常是因为解码时使用的行速率有误——拉伸可使其可读，而 LPM 芯片则能从根本上解决此问题。双击可输入数值。
wefax-reset = 重置
wefax-back-picture-s-own-proportions = 恢复为图片自身的比例
wefax-follow = 跟随
wefax-keep-newest-lines-view-scrolling = 使最新的行保持在视图中。向上滚动会关闭此项，以便你阅读已经接收到的内容；滚动回底部则会再次开启。
wefax-saved = 已保存
wefax-path = 路径
wefax-reading-radio-s-charts = 正在读取电台的图表……
wefax-finished-charts-collect-here = 完成的图表会收集在这里。
wefax-delete-chart = 删除此图表
wefax-loading = 加载中……
wefax-stations = 电台
wefax-broadcast-radiofax-schedules-picking-one = 广播无线电传真时刻表——选择其中一项即可调谐表盘并设定行速
wefax-frequencies-are-published-carrier-dial = 频率为公布的载波；表盘位于其下方 1.9 kHz，此操作已为你完成。时刻表会变化，电台也会停播——请将其作为开始查找的地方，而不是时间表。
wefax-delete-chart-store = 从存储中删除此图表
wefax-newer = ◀ 较新
wefax-chart-received-after-one = 在此之后接收的图表
wefax-older = 较早 ▶
wefax-chart-received-before-one = 在此之前接收的图表
wefax-no-longer-store = 已不在存储中
wefax-loading-full-size = 正在加载原始尺寸……
wefax-save = 保存
wefax-save-copy-chart-computer = 在本计算机上保存此图表的副本
wefax-sure = 确定？
wefax-click-again-delete-chart-good = 再次点击即永久删除此图表
wefax-delete = 删除
wefax-file-chart-was-saved = 保存此图表所用的文件

## aprs (src/app/panels/aprs.rs)

aprs-starting-aprs-modem = 正在启动 APRS 调制解调器……
aprs-aprs = APRS
aprs-no-callsign = 无呼号
aprs-nothing-will-transmitted-no-beacon = 在此电台拥有呼号之前，不会发射任何内容——不发信标，不发消息，甚至不发确认。请在“设置 → 常规”中设置呼号，或在“设置”中为 APRS 单独设置带 SSID 的呼号。
aprs-no-position = 无位置
aprs-beacon-needs-somewhere-report-fill = 信标需要一个报告位置。请在“设置 → 常规”中填入你的网格定位，或在“设置”中给出坐标。
aprs-setup = 设置
aprs-callsign-symbol-digipeater-path-position = 呼号、符号、数字中继路径、位置和信标
aprs-how-often-beacon-your-position = 无人值守时，多久发送一次你的位置信标。`off`——默认值——从不发送信标：选择某种模式不得使电台上空。三十分钟是固定电台的惯例，移动电台则更频繁，而且每一次信标都占用别人的信道时间。
aprs-beacon = {" "}信标{" "}
aprs-station-has-no-callsign-see = 此电台没有呼号——请参见左侧的警告。
aprs-station-has-no-position-report = 此电台没有可报告的位置——请参见左侧的警告。
aprs-send-one-position-report-now = 立即发送一次位置报告，无需等待间隔。与其他所有内容一样，它会等待信道空闲。
aprs-stations = 电台
aprs-nothing-heard-yet = 尚未听到任何内容
aprs-heard-direct = 直接收听到
aprs-two-characters-station-sent-say = 电台发送的两个字符，用于说明它是什么
aprs-killed = 已撤销
aprs-station-put-object-here-has = 在此放置该对象的电台已将其撤销
aprs-sender-deliberately-reported-square-rather = 发送方特意报告的是一个方格，而非一个点。地图绘制的是方格。
aprs-digipeaters-last-frame-came-through = 最后一帧经过的数字中继。带有 `*` 的表示它确实转发了该帧。
aprs-message = {" "}消息{" "}
aprs-address-message-box-station = 将消息框的收信人设为此电台
aprs-center = {" "}居中{" "}
aprs-put-station-middle-map-double = 将此电台置于地图中央。双击地图即可将视图交还给自动适配。
aprs-channel = 信道
aprs-messages = 消息
aprs-frames-arrived-failed-their-check = 已到达但未通过校验序列的帧——可能是碰撞、衰落，或信号太弱而无法读取。计数上升而没有解码结果，说明调制解调器*确实*在收听该信道：请检查电平表，并打开电台的静噪。
aprs-frames-read-cleanly-off-channel = 从信道上干净读取、但并非 APRS 的帧——可能是他人的分组通信，或本版本无法解码的格式。
aprs-every-frame-channel-arrived-other = 信道上到达的每一帧，按其到达的原样显示——包括他人的通信，以及解码器无法读取的任何内容。
aprs-nothing-channel-yet = 信道上尚无任何内容
aprs-no-messages = 没有消息。
aprs-waiting-channel = 等待信道
aprs-sent-retried-still-waiting-acknowledged = 已发送并重试，仍在等待确认
aprs-air-waiting-acknowledged = 已在空中，等待确认
aprs-acknowledged-far-end = 已被对方确认
aprs-refused-far-end-not-retried = 被对方拒绝——不再重试
aprs-no-ack = 无确认
aprs-no-answer-after-every-retry = 每次重试后均无应答
aprs-send = {" "}发送{" "}
aprs-station-has-no-callsign-so = 此电台没有呼号，因此无法发射任何内容。请在“设置 → 常规”中设置呼号，或在“设置”中为 APRS 单独设置带 SSID 的呼号。
aprs-needs-station-address-something-say = 需要一个收信电台和要说的内容。
aprs-send-keep-retrying-until-far = 发送它，并持续重试，直到对方确认。消息是此信道上唯一会被应答的内容。
aprs-busy = 忙
aprs-stations-heard-still-inside-window = 在 APRS 设置中设定的窗口内听到的电台，以及此刻是否有其他电台在信道上。忙碌时将不会键控任何内容。
aprs-tx = ● 发射
aprs-over-progress-frames-waiting-channel = 正在进行的通话、等待信道空闲的帧，或距离下一次计划信标的时间。
aprs-audio-reaching-modem-flat-means = 到达调制解调器的音频。平直表示完全没有任何东西到达——请检查电台的数据输出是否就是 sdroxide 所收听的那一路。在调制解调器听到载波时，它会亮起绿色。

## vdl2 (src/app/panels/vdl2.rs)

vdl2-starting-vdl2-decoder = 正在启动 VDL2 解码器……
vdl2-aircraft-datalink = 飞机数据链
vdl2-tune-middle-vdl2-group-decoder = 调谐到 VDL2 组的中心。解码器自己的窗口会从那里滑动，以尽可能多地纳入接收机能够触及的十四个信道。
vdl2-setup = 设置
vdl2-which-channels-listen-how-hard = 收听哪些信道、突发信号需要多强，以及保留多少日志
vdl2-save-vdl2-message-log-file = 将 VDL2 消息日志保存到文件
vdl2-vdl2-channel = VDL2 信道
vdl2-messages = 消息
vdl2-address-flight-registration-label = 地址、航班、注册号或标签
vdl2-held = 已暂停
vdl2-hold = 暂停
vdl2-hold-message-log-where-so = 将消息日志保持在原位，以便在下一次发射将其推上屏幕之前阅读其中一条。解码器继续运行，计数器也继续变化；松开后即可显示此期间到达的所有内容。
vdl2-nothing-decoded-yet = 尚无任何解码内容
vdl2-nothing-matches-filter = 没有任何内容符合过滤条件
vdl2-stations = 电台
vdl2-nobody-heard-yet = 尚未听到任何电台
vdl2-vdl2-setup = VDL2 设置
vdl2-channels = 信道
vdl2-every-25-khz-slot-136 = 从 136.650 到 136.975 MHz，每个 25 kHz 的时隙。每个信道各有一个下变频器，全部位于同一个接收机窗口之内。关闭某一个会节省一点处理器时间；它并不会使其他信道更灵敏。
vdl2-burst-threshold = 突发阈值
vdl2-above-each-channel-s-own = 高于每个信道各自的底噪
vdl2-lower-catches-weaker-transmissions-costs = 更低的阈值能捕获更弱的发射，但会在噪声上耗费处理器时间；更高则会漏掉它们。底噪是按信道逐一学习的，在此处的更改不会将其丢弃。
vdl2-keep-log = 保留在日志中
vdl2-track-most = 最多跟踪
vdl2-longest-silent-go-first = 最长时间保持沉默的先移除
vdl2-forget-station-after = 在以下情况下忘记一个电台
vdl2-nothing-heard = 且未从其收到任何内容
vdl2-show-unread-payloads = 显示未读载荷
vdl2-hex = 以十六进制显示
vdl2-frames-carrying-x-25-clnp = 承载 X.25、CLNP 或其之上的数据链应用的帧。SDRoxide 会为它们命名并显示字节，而不是读取它们；若将其隐藏，就会隐藏这些流量所占的比例。
vdl2-registration = 注册号
vdl2-flight = 航班
vdl2-label = 标签
vdl2-block = 块
vdl2-sequence = 序号
vdl2-mode = 模式
vdl2-more-blocks-follow = 后续还有更多块
vdl2-check = 校验
vdl2-message-crc-good = 消息 CRC 正确
vdl2-message-crc-not-checked = 未检查消息 CRC
vdl2-parity = 奇偶校验
vdl2-link-control-no-payload = 链路控制，无载荷
vdl2-name = 名称
vdl2-addr = 地址
vdl2-type = 类型
vdl2-msgs = 消息数

## cw (src/app/panels/cw.rs)

cw-tone-being-copied-tone-transmitted = 被抄收的音调和所发射的音调——在 CW 中它们是同一频率。点击瀑布图，可将其移到一个信号上。
cw-down-10-hz = 降低 10 Hz
cw-up-10-hz = 提高 10 Hz
cw-qrg-main-readout-tuning-line = QRG：主读数和调谐线位于正在通联的频率上。点击则改为显示表盘——即低于它的侧音音高，这是大多数电台的显示方式。
cw-put-main-readout-tuning-line =
    将主读数和调谐线置于信号上，而不是表盘上，因此所显示的频率就是双方操作员都会报出的频率，而调谐线位于通带中央。调谐方式不变；只是数字发生了移动。

    Clicking a signal lands it on the cursor only as closely as the click step allows — Controls → click-to-tune rounding, 10 Hz by default. A coarse step leaves the signal off the pitch by up to half of it, and the readout will say so.
cw-sending-speed-read-off-signal = 从信号读取的发送速度
cw-signal-noise-500-hz-same = 500 Hz 带宽内的信噪比——与报告中所引用的数值相同
cw-how-far-off-cursor-signal = 信号实际偏离光标多远。解码器会跟随它；通带则不会，因此若偏差增大，请轻推光标。
cw-listening = — 收听中 —
cw-tx = ● 发射
cw-nothing-copied-yet = — 尚未抄收任何内容 —
cw-key-send-characters-you-send = 按键发送——你发送的字符会出现在此处。
cw-type-line-return-sends = 输入一行，按回车发送……
cw-type-here-send = 在此输入以发送……
cw-key = {" "}键 ●{" "}
cw-key-2 = {" "}键{" "}
cw-hold-key-bound-cw-straight = 将绑定到 CW 直键的按键——默认为空格键，你也可以在“设置 → 控制”中选择任何按键——作为直键按住：按住时按下，松开时抬起，而不是输入文字。开启时发射框被锁定，整个键控器交给直键使用：排队中的任何文字都会被丢弃。

    To hand-key it, set CW keying to "Sound card (MCW)" in Settings → Radio. The rig is then held on a sideband and the keyer's own sidetone is transmitted as audio, which is the route the straight key drives.
cw-watchdog = 看门狗
cw-straight-key-was-held-down = 直键被按住的时间过长——是抬起信号丢失，而不是手在操作——因此载波已被切断，发射也已关闭。再次按下键即可继续。
cw-tx-2 = {"  "}发射开{"  "}
cw-tx-3 = {"   "}发射{"   "}
cw-send-what-box-now-without = 立即发送输入框中的内容，无需等待回车
cw-hold-key-down-between-characters = 在字符之间保持按键按下，这样键入的内容都无需等待
cw-call-cq = {" "}呼叫 CQ{" "}
cw-nocall = 无呼号
cw-clear = {" "}清除{" "}
cw-stop-sending-drop-whatever-has = 停止发送，并丢弃尚未发出的内容。开启直键时，对你所键入内容的回读也会一并停止。
cw-sidetone = 侧音
cw-play-keyed-tone-through-computer = 在发送的同时，也通过本计算机的扬声器播放所键控的音调，以便你听到自己正在发送的内容。在“声卡（MCW）”下，音调只输出到电台，别处听不到，因此没有此项你就是在无声地发送。若电台自身的监听或离线抄收已能完成此任务，则请关闭。
cw-hold-what-typed-until-return = 将键入的内容保留到按下回车，然后整行一次性发送，而不是在键入每个字符时就逐个键控到空中。这在能够由文字自行键控的收发信机上值得开启，因为每一次交给其键控器的操作，都是一次完整的收发循环。
cw-cw-messages = CW 消息
cw-clear-rx = {" "}清空接收{" "}
cw-empty-receive-window-straight-key = 清空接收窗口以及直键的回读内容。空中的任何内容都不会停止。
cw-lock = 锁定
cw-decode-speed-set-here-instead = 以此处设置的速度解码，而不是从信号中读取速度。有助于处理太弱、以至于速度搜索无法稳定下来的信号。
cw-farnsworth-send-characters-full-speed = Farnsworth：以全速发送字符，只拉长它们之间的间隔，使其以正确的节奏被听到，同时又慢到足以记下。
cw-off-normal-spacing = 关闭——正常间距
cw-keying-speed = 键控速度
cw-idle-0 = 空闲 0
cw-how-long-transmit-held-after = 在最后一个字符之后，或松开直键之后，发射会保持多久才切断载波。更长可以容纳慢速打字者的停顿；更短则能更快地离开该频率；关闭则在排队的所有内容发出后立即切断。
cw-off-drop-once = 关闭——立即切断

## textmodem (src/app/panels/text_modem.rs)

textmodem-tune-down-10-hz = 向下调谐 10 Hz
textmodem-tune-up-10-hz = 向上调谐 10 Hz
textmodem-tx = ● 发射
textmodem-listening = — 收听中 —
textmodem-type-line-return-sends = 输入一行，按回车发送……
textmodem-type-here-transmit = 在此输入以发射……
textmodem-tx-2 = {"  "}发射开{"  "}
textmodem-tx-3 = {"   "}发射{"   "}
textmodem-call-cq = {" "}呼叫 CQ{" "}
textmodem-nocall = 无呼号
textmodem-clear = {" "}清除{" "}
textmodem-hold-what-typed-until-return = 将键入的内容保留到按下回车，然后整行一次性发送，而不是在键入每个字符时就将其逐一推送到空中。这使得一行内容可以在任何部分发出之前被回读并更正。
textmodem-messages = 消息
textmodem-hell = HELL
textmodem-contrast = 对比度
textmodem-harder-softer-dots-redraws-whole = 更硬或更软的点——重绘整个条带
textmodem-width = 宽度
textmodem-t-2row = {" "}2 行{" "}
textmodem-draw-every-column-twice-stacked = 将每一列绘制两次，上下叠放。Hell 没有垂直同步，因此无论相位如何，这都能保持一份完整的文字副本可读。
textmodem-rev = {" "}反色{" "}
textmodem-reverse-video-light-dots-dark = 反色显示——深色纸上的浅色点
textmodem-clear-rx = {" "}清空接收{" "}
textmodem-hold-channel-idle-sends-blank = 保持信道：空闲时发送空白纸，使条带继续滚动
textmodem-hold-what-typed-until-return-2 = 将键入的内容保留到按下回车，然后整行一次性发送，而不是在键入每个字符时就将其逐一绘制到条带上。实时键入的更正已经在纸上；而在回车之前所做的更正则从未出现在纸上。
textmodem-shift = 频移
textmodem-shift-5-hz = 频移 −5 Hz
textmodem-shift-5-hz-2 = 频移 +5 Hz
textmodem-baud = 波特率
textmodem-reverse-swap-mark-space-needed = 反向：交换标记和空号。当信号在与发送时相反的边带上被接收、从而使音调颠倒时，需要使用此项。
textmodem-track-tuning-error-automatically-instead = 自动跟踪调谐误差，而不是固定在光标上。
textmodem-tones = 音调
textmodem-mode = 模式
textmodem-off-keyed = 开/关键控

## ais (src/app/panels/ais.rs)

ais-starting-ais-decoder = 正在启动 AIS 解码器……
ais-tune-between-two-ais-channels = 调谐在两个 AIS 信道之间——161.975（AIS 1，海事 87B）和 162.025（AIS 2，88B）。表盘位于中间，因为那里没有任何发射，因此零中频接收机的直流尖峰不会落在任何一个信道上。
ais-setup = 设置
ais-channels-timeouts-trail-length-how = 信道、超时、轨迹长度，以及矢量延伸的距离
ais-vessels = 船舶
ais-nothing-heard-yet = 尚未听到任何内容
ais-virtual-nothing-there = 虚拟——那里并无实物
ais-position-stale = 位置已过时
ais-search-rescue-transmitter-man-overboard = 搜救发射机、落水信标或 AIS EPIRB
ais-call-sign = 呼号
ais-station-time = 电台时间
ais-dgnss = {" "}（DGNSS）
ais-first-heard = 首次收听
ais-center = 居中
ais-put-vessel-middle-chart = 将此船舶置于海图中央
ais-last-message-station-nmea-sentences = 此电台最后一条消息，以接收机会发送到线路上的 NMEA 语句形式显示。可将其粘贴到任何其他 AIS 解码器中，以核对 sdroxide 对它的解读。
ais-ais-setup = AIS 设置
ais-channels = 信道
ais-drop-chart-after = 超过以下时间后从海图中移除
ais-without-position-report = 没有位置报告
ais-five-minutes-default-not-ads = 默认为五分钟，而非 ADS-B 的十秒：锚泊中的船只每三分钟报告一次，而较短的窗口会使港口的大部分区域在两条完全有效的报告之间变为空白。超过此时间后，船舶会从海图上移除，其行会变灰——它不是淡出，因为位置过时的暗淡符号仍然是关于船舶位置的一种声明，与真实的符号使用相同的墨色。
ais-drop-list-after = 超过以下时间后从列表中移除
ais-nothing-heard-all = 完全没有听到任何内容
ais-message-carrying-ship-s-name = 携带船名的消息每六分钟才会出现一次，因此较短的列表窗口会在船舶刚要说出其名字之前就把它们丢弃。
ais-trail-length = 轨迹长度
ais-history-behind-each-target = 每个目标身后的历史轨迹
ais-minutes-rather-than-points-because = 以分钟而非点数计算，因为 AIS 的报告速率跨越两个数量级：固定的点数对渡轮而言是八十秒，对锚泊的油轮却是两小时，而绘制出来完全相同。设为零则关闭轨迹。
ais-speed-vector = 速度矢量
ais-how-far-ahead-vector-reaches = 矢量延伸的距离
ais-slot-threshold = 时隙阈值
ais-above-channel-s-learned-noise = 高于该信道已学习到的底噪
ais-track-most = 最多跟踪
ais-vessels-2 = {" "}艘船舶
ais-ais-receive-only-here-always = 此处的 AIS 仅用于接收，并且将始终如此：它是一项保障生命安全的服务，而执照并不允许在其上发送虚假的船舶通信。
ais-fill-my-grid-digimode-setup = 请在数字模式设置中填写“我的网格”，以便计算距离和方位，并使海图以你所在的位置为中心，而不是以最远的目标为中心。
ais-name = 名称
ais-mmsi = MMSI
ais-type = 类型

## qo100 (src/app/qo100.rs)

qo100-receiver-cannot-reach-10489-750 = 此接收机自身无法到达 10489.750 MHz——请先设置 LNB/变频器偏移（设置 ▸ 电台）
qo100-run-fast-spectral-tracker-every = 运行快速频谱跟踪器：每秒在停靠窗口中查找信标的两个对称波瓣，并报告载波所在的位置
qo100-telemetry = 遥测
qo100-also-run-ao-40-uncoded = 同时运行 AO-40 未编码帧解码器——同步字、CRC 和遥测文本——并通过逐步显示（载波 → 同步 → CRC）读出每次过境的进展程度
qo100-auto = 自动
qo100-auto-correct-let-tracker-adjust = 自动校正：让跟踪器自行调整变频器/LNB 偏移——这是一个缓慢的闭环，在估计值干净且稳定时，它会将信标轻推回 10489.750 MHz，并在 LNB 漂移时保持其在那里。每次动作时都会重新打开前端。
qo100-narrower-search-smaller-slice = 更窄——搜索较小的片段
qo100-wider-when-beacon-isn-t = 更宽——用于当前宽度下找不到信标时
qo100-clear-mark = 清除标记
qo100-drop-hand-placed-beacon-mark = 删除手动放置的信标标记，并恢复为解码器自己的读数
qo100-lo = 低 −
qo100-lo-2 = 低 +
qo100-hi = 高 −
qo100-hi-2 = 高 +
qo100-radio-s-own-tuning-range = 此电台自身的调谐范围无法到达 10489.750 MHz——在此窗口能做任何事情之前，它需要 LNB/变频器偏移（设置 ▸ 电台 ▸ 变频器）。
qo100-open-settings-radio = 打开设置 ▸ 电台
qo100-tune-10489-750-mhz = 调谐到 10489.750 MHz
qo100-tip-if-you-can-see = 提示：如果你能在条带中看到信标但无法锁定，请双击它——这会将其标记为 10489.750 MHz，并让“应用”校正到该频率
qo100-tracker = 跟踪器
qo100-no-twin-lobe-shape-parking = 停靠窗口中没有双波瓣形状
qo100-shape = 形状
qo100-drift = 漂移
qo100-need-longer-run-estimates = —（需要更长的估计序列）
qo100-cycles = 周期
qo100-receiver = 接收机
qo100-target = 目标
qo100-measured = 测得
qo100-clicked = {"  "}（已点击）
qo100-tracker-2 = {"  "}（跟踪器）
qo100-not-locked-yet = 尚未锁定
qo100-converter-offset = 变频器偏移
qo100-decode = 解码
qo100-decode-filling-first-frame-buffer = 解码——正在填充第一个帧缓冲……
qo100-decode-off-turn-telemetry-chip = 解码——关闭。请开启遥测芯片（顶行）以查看载波 → 同步 → CRC 的读数。
qo100-apply-correction = {" "}应用校正{" "}
qo100-write-converter-lnb-offset-puts = 写入变频器/LNB 偏移，使信标位于你所点击的位置并对应到 10489.750 MHz，然后重新打开接收机——与“设置 ▸ 电台 ▸ 应用”所造成的短暂中断相同
qo100-write-converter-lnb-offset-puts-2 = 写入变频器/LNB 偏移，使跟踪器的估计值对应到 10489.750 MHz，然后重新打开接收机——与“设置 ▸ 电台 ▸ 应用”所造成的短暂中断相同
qo100-waiting-second-crc-valid-frame = 等待第二个 CRC 校验通过的帧，然后才提供写入此项——仅一次锁定可能只是偶然匹配
qo100-write-corrected-converter-lnb-offset = 写入校正后的变频器/LNB 偏移并重新打开接收机——短暂中断，与“设置 ▸ 电台 ▸ 应用”所造成的相同
qo100-decoder-runs-receiving-station-readout = 解码器在接收电台上运行——其读数不会发送给远程客户端
qo100-starting = 正在启动……
qo100-searching-first-window-fills-after = 正在搜索——第一个窗口约 24 秒后填满，然后重复

## js8 (src/app/panels/js8.rs)

js8-multi = 多速率
js8-decode-every-js8-speed-not = 解码所有 JS8 速率，而不仅是你发射所用的那一种。四种速率共用子频段，且是四种不同的波形，因此若不开启此项，以其他速率发送的电台将不可见——而两种速率之间的通信则根本无法发生。大约会使接收 CPU 开销增加四倍。
js8-hb-auto = 心跳自动
js8-turbo-does-not-beacon-local = Turbo 不发送信标——它是本地和 VHF 的速率
js8-beacon-your-callsign-grid-every = 每 15 分钟发送一次你的呼号和网格信标
js8-until-next-heartbeat = 直到下一次心跳
js8-tx = ● 发射
js8-heard = 收听
js8-nothing-heard-yet = — 尚未听到任何内容 —
js8-dxcc = DXCC
js8-band = 频段
js8-grid = 网格
js8-dupe = 重复
js8-reply = 应答
js8-no-messages = — 无消息 —
js8-cq = {" "}CQ{" "}
js8-hb = {" "}心跳{" "}
js8-clear = {" "}清空至{" "}
js8-forget-selected-station-composer-goes = 忘记所选电台——编写框将回到 @ALLCALL
js8-already-addressing-allcall = 已在发送给 @ALLCALL
js8-stop = {" "}停止{" "}
js8-send = {" "}发送{" "}
js8-message = 消息……

## logbook (src/app/logbook.rs)

logbook-logbook = 日志
logbook-new-entry = + 新建条目
logbook-adif = ADIF
logbook-import = 导入
logbook-import-qsos-adif-adi-adif = 从 ADIF（.adi 或 .adif）文件导入通联
logbook-new-qso = 新建通联
logbook-edit-qso = 编辑通联
logbook-call = 呼号
logbook-lookup = 查询
logbook-look-up-name-qth-grid = 查询姓名 / QTH / 网格
logbook-grid = 网格
logbook-freq-mhz = 频率 MHz
logbook-mode = 模式
logbook-rst-sent = 发送 RST
logbook-rst-rcvd = 接收 RST
logbook-name = 姓名
logbook-state = 州 / 省
logbook-country = 国家
logbook-date-utc = 日期 UTC
logbook-time = 时间
logbook-pwr-w = 功率 W
logbook-contest = 比赛
logbook-s-sent = 发送序号
logbook-s-rcvd = 接收序号
logbook-comment = 备注
logbook-save = {" "}保存{" "}
logbook-cancel = 取消
logbook-lotw = LoTW ✓
logbook-eqsl = eQSL ✓
logbook-card = QSL 卡 ✓
logbook-lotw-2 = LoTW ↑
logbook-eqsl-2 = eQSL ↑
logbook-qrz = QRZ ↑
logbook-hamqth = HamQTH ↑
logbook-club-log = Club Log ↑
logbook-wavelog = Wavelog ↑
logbook-delete-entry = 删除此条目
logbook-edit = 编辑
logbook-upload-qso-configured-logs = 将此通联上传到已配置的日志
logbook-no-qsos-yet-run-ft8 = 尚无通联——请运行 FT8/FT4，或添加手动条目

## scanner (src/app/scanner.rs)

scanner-scanner = 扫描器
scanner-work-through-stored-memory-channels = 依次扫描已存储的记忆信道
scanner-work-through-slice-band-channel = 按信道网格依次扫描频段的一个片段
scanner-stop = 停止
scanner-start = 开始
scanner-from = 起始
scanner-step = 步进
scanner-mode = 模式
scanner-range-empty-high-edge-has = 该范围为空——高端必须至少比低端高出一个信道。
scanner-skip-while-holding-pass-over = 跳过——此后它会保持跳过该信道
scanner-skipping = 跳过中
scanner-clear = 清除
scanner-stop-passing-over-any-them = 停止跳过其中任何一个
scanner-stop-passing-over-channel = 停止跳过此信道
scanner-no-memory-channels-stored-yet = 尚未存储任何记忆信道——请在 MEM 窗口中存储一些。
scanner-skip-channel-pass-over = 跳过信道以越过它
scanner-skip = 跳过
scanner-look-all-channels-fall-inside = 在全景图所用的同一变换上，查找落在同一接收机窗口内的所有信道，并且只调谐到有信号的那些。这样，一个频段上的列表无论多长，每一轮只需调谐一次，而不是每个信道都要等待稳定的时间。扫描在停止之前仍会在每个候选信道上收听，因此停止的条件不变——但扫描是通过 FFT 而非接收机的滤波器进行测量的，因此如果它开始在空无一物处停下，请检查阈值。
scanner-radio-hands-over-demodulated-audio = 此电台输出的是解调后的音频，且自身没有可供搜索的频谱，因此无论如何，它的记忆扫描都会访问每一个信道。
scanner-fast = 快速
scanner-read-list-off-spectrum-instead = 从频谱中读取列表，而不是访问每一个信道
scanner-folders = 文件夹
scanner-scan-every-folder-every-folder = 扫描每一个文件夹，以及从现在起创建的每一个文件夹。若只想扫描特定文件夹，请改为选择它们。
scanner-unfiled = 未归档
scanner-nothing-filed-under-folders-you = 你所选文件夹中没有任何归档内容。
scanner-stops = 停止条件
scanner-use-receiver-s-own-squelch = 使用接收机自己的静噪，使扫描恰好在音频会打开的地方停止
scanner-squelch-off-scan-will-stop = 静噪已关闭——扫描会在它检查的第一个内容上停止
scanner-channel-power-signal-has-reach = 信号必须达到的信道功率，才能使扫描停止
scanner-listens = 收听时间
scanner-how-long-stay-candidate-before = 在判断候选信道之前在其上停留多久。低于约十分之一秒时，电平表尚未稳定，弱信号会被漏掉
scanner-resumes = 恢复
scanner-carry-once-signal-drops = 信号消失后继续
scanner-carry-after-fixed-time-regardless = 无论如何，经过固定时间后继续
scanner-stay-until-you-press-next = 停留直到你按下“下一个”
scanner-how-long-stay = 停留时长
scanner-how-long-wait-after-signal = 信号消失后等待多久
scanner-move-don-t-stop-channel = 继续前进，并且不再在此信道上停止
scanner-next = 下一个
scanner-move-now = 立即继续

## adsb (src/app/panels/adsb.rs)

adsb-starting-ads-b-decoder = 正在启动 ADS-B 解码器……
adsb-ads-b = ADS-B
adsb-t-1090-mhz-mode-s = 1090 MHz Mode S
adsb-tune-receiver-1090-000-mhz = 将接收机调谐到 1090.000 MHz，即全球通用的 ADS-B 下行频率。只有一个信道。
adsb-setup = 设置
adsb-timeouts-trail-length-how-far = 超时、轨迹长度，以及速度矢量延伸的距离
adsb-aircraft = 飞机
adsb-nothing-heard-yet = 尚未听到任何内容
adsb-ground = 在地面
adsb-position-stale = 位置已过时
adsb-gnss-altitude = GNSS 高度
adsb-ground-speed = 地速
adsb-vertical-rate = 垂直速率
adsb-first-heard = 首次收听
adsb-center = 居中
adsb-put-aircraft-middle-map = 将此飞机置于地图中央
adsb-last-frame-accepted-aircraft = 从此飞机接受的最后一帧
adsb-ads-b-setup = ADS-B 设置
adsb-drop-map-after = 超过以下时间后从地图上移除
adsb-without-position-report = 没有位置报告
adsb-past-aircraft-comes-off-map = 超过此时间后，飞机会从地图上移除，其行会变灰。它不是淡出：位置过时的暗淡方块仍然是关于飞机位置的一种声明，与真实的符号使用相同的墨色。
adsb-drop-list-after = 超过以下时间后从列表中移除
adsb-nothing-heard-all = 完全没有听到任何内容
adsb-trail-length = 轨迹长度
adsb-points = {" "}个点
adsb-history-dots-behind-each-target = 每个目标身后的历史轨迹点
adsb-speed-vector = 速度矢量
adsb-how-far-ahead-leader-line = 引导线延伸的距离
adsb-one-minute-usual-radar-convention = 一分钟是雷达的惯例：该线段的长度恰好等于飞机在该时间内飞行的距离，因此在任何缩放比例下，两条等长的引导线都代表相同的速度。设为零则关闭它们。
adsb-track-most = 最多跟踪
adsb-aircraft-2 = {" "}架飞机
adsb-aircraft-ground-are-placed-against = 在地面上的飞机以电台自身的位置为基准进行放置——表面应答器本身没有明确的解码结果——因此，若附近的机场没有显示任何内容，请在数字模式设置中填写“我的网格”。
adsb-call = 呼号
adsb-icao = ICAO

## drm (src/app/drm.rs)

drm-waiting-receiver = 正在等待接收机……
drm-not-drm-set-mode-drm = 不在 DRM 模式。请在数字短波广播上将模式设为 DRM——表盘应位于信道中心，而不是边带上。
drm-no-drm-signal-locked-decoder = 尚未锁定 DRM 信号。在干净的载波上，解码器需要几秒钟：DRM 在能够读取任何内容之前，会进行 400 ms 或 2 s 的交织。
drm-station-s-audio-codec-not = 此电台的音频编解码器在此无法解码。xHE-AAC 需要系统中的 libfdk-aac——它无法内置，因为其许可证与本程序的许可证不兼容。请安装它（Debian/Ubuntu：libfdk-aac2，Arch：libfdk-aac，macOS：brew install fdk-aac，Windows：与 sdroxide.exe 放在同一目录中的 libfdk-aac-2.dll），然后重新启动。此屏幕上的其他内容均可正常解码。
drm-quality = 质量
drm-last-minute = 最近一分钟
drm-waiting-locked-signal = 等待锁定的信号
drm-audio-frames-decoding-but-station = 音频帧正在解码——但此电台的编解码器在此无法解码
drm-audio-frames-decoding = 音频帧正在解码
drm-samples-reaching-decoder = 采样正在到达解码器
drm-time = 时间
drm-symbol-timing-recovered = 符号时序已恢复
drm-frame = 帧
drm-transmission-frames-found = 已找到的传输帧
drm-fast-access-channel-what-transmission = 快速访问信道——即该传输的内容
drm-service-description-channel-what-services = 服务描述信道——即这些服务是什么
drm-audio = 音频
drm-constellation = 星座图
drm-waiting-decoded-symbols = 等待解码出的符号
drm-sampled-across-frame = 在整个帧上采样
drm-whole-frame = 整帧
drm-mode = 模式
drm-robustness-mode-channel-width-ground = 稳健模式和信道宽度。A 用于地波路径，承载量最大；D 用于严重散射的天波路径，承载量最小。
drm-interleave = 交织
drm-how-far-transmission-spreads-each = 传输在时间上将每一帧分散得多远。长交织可抵御更深的衰落，但相应地获取所需的时间也更长。
drm-protection = 保护
drm-offset = 偏移
drm-residual-sample-clock-error-against = 相对于发射机的残余采样时钟误差。数值大且稳定，说明是接收机的参考有偏差，而非广播本身。
drm-doppler = 多普勒
drm-delay = 时延
drm-doppler-delay-spread-path-how = 路径的多普勒和时延扩展——即它移动得多快，以及其回波到达的时间相隔多远。
drm-service = 服务
drm-decode-service-multiplex = 解码复用中的此服务

## atchat (src/app/panels/atchat.rs)

atchat-starting-atchat-net-station = 正在启动 AtCHAT NET 电台……
atchat-atchat = ATCHAT
atchat-net-multi-station-keyboard-file = NET——多电台键盘与文件
atchat-net = {" "}在网{" "}
atchat-net-click-leave-state-kept = 在网上——点击即可离开（状态会保留；再次点击即可重新加入）
atchat-rejoin = {" "}重新加入{" "}
atchat-off-net-click-rejoin = 不在网上——点击即可重新加入
atchat-off-net = 不在网上
atchat-carrier = ● 载波
atchat-another-station-transmitting-channel-busy = 另一电台正在发射——信道忙。
atchat-tx = ● 发射
atchat-station-callsign-virtual-channel-address = 电台呼号和虚拟信道地址
atchat-virtual-channel = 虚拟信道
atchat-work-net-over-tcp-endpoint = 通过 TCP 端点而非电台运行网络——一个兼容 channel_server 的回环，用于没有电台时的开发。关闭则使电台上空。
atchat-roster = 成员名单
atchat-nobody-heard-yet = 尚未听到任何电台
atchat-chat = {" "}聊天{" "}
atchat-common-channel-lines-here-go = 公共信道——此处的消息发送给所有人（ALL）
atchat-log = {" "}日志{" "}
atchat-station-s-own-air-activity = 电台自身的空中活动——主节点选举、成员名单老化、ARQ 重试
atchat-no-activity-logged-yet = 尚未记录任何活动。
atchat-no-messages-yet = 尚无消息。
atchat-message-everyone-all = 发给所有人的消息（ALL）
atchat-send = {" "}发送{" "}
atchat-transfers = 传输
atchat-all = → 全部
atchat-sent-file-follows-active-chat = 发送的文件会跟随当前活动的聊天标签页
atchat-send-file = {" "}发送文件{" "}
atchat-send-file-image-over-air = 通过空中发送文件或图片，采用分块 CRC-ARQ
atchat-nothing-flight = 没有正在传输的内容
atchat-images = 图片
atchat-no-images-received = 尚未收到图片。


## rds (src/app/rds.rs)

rds-station = 电台
rds-diagnostics = 诊断
rds-which-programme-type-table-read = 依据哪一张节目类型表来解读此电台。RDS 用于北美以外的所有地区；RBDS 则增加了美国电台身份码所拼写出的呼号。“自动”会跟随电台发送的扩展国家代码；在收到它之前，它不会拼写出呼号——因为它所读取的身份范围与九个国家的 RDS 代码共用，因此若电台不发送国家代码，请选择 RBDS 来为其命名。
rds-waiting-receiver = 正在等待接收机……
rds-no-rds-station-not-every = 此电台没有 RDS。并非每座发射机都带有它，较弱的发射机可能携带了它，却未能收到。
rds-rds-only-carried-wfm-broadcast = RDS 仅由调频广播电台携带。
rds-holding = 保持
rds-block-sync-has-dropped-last = 数据块同步已丢失——这是最后一次读取的电台。
rds-identity = 身份
rds-programme = 节目
rds-described = 描述为
rds-content = 内容
rds-music = 音乐
rds-speech = 语音
rds-traffic = 交通
rds-announcement-air-now = 当前正在播出的公告
rds-carried-station = 由此电台携带
rds-not-carried = 未携带
rds-radio-text = 电台文本
rds-station-clock = 电台时钟
rds-country-code = 国家代码
rds-also = 同时在
rds-also-tagged = 同时标记为
rds-sync = 同步
rds-no-sync = 无同步
rds-blocks-failed-their-check-plus = 未通过校验的数据块，以及纠错器不得不修复的数据块。纠正也算作错误，这是有意为之：数字上升是电台即将停止解码的第一个迹象。
rds-front-end-overloading = 前端过载
rds-converter-clipping-which-puts-distortion = 变换器正在削波，这会使失真遍布整个复合信号——包括 57 kHz 数据副载波。请调低射频增益，或接入衰减器：RDS 在任何内容可以听到之前大约就会受损一分贝，因此听起来完美的电台，其强度仍可能远超出其数据能够承受的程度。
rds-group-types = 组类型
rds-recent-groups-newest-last = 最近的组——最新的在最后
rds-nothing-decoded-yet = 尚无任何解码内容

## winlink (src/app/winlink.rs)

winlink-mail = 邮件
winlink-abort = 中止
winlink-stop-session-progress = 停止正在进行的会话
winlink-connect = 连接
winlink-compose = 撰写
winlink-connecting = 正在连接……
winlink-not-connected-yet = 尚未连接
winlink-loading = 加载中……
winlink-date = 日期
winlink-from = 发件人
winlink-subject = 主题
winlink-no-subject = （无主题）
winlink-delete-message = 删除此消息
winlink-arch = 存档
winlink-move-archive = 移至存档
winlink-select-message = 选择一条消息
winlink-reply = 回复
winlink-delete = 删除
winlink-to = 收件人
winlink-cc = 抄送
winlink-subject-2 = 主题
winlink-callsign-smtp-someone-example-org = 一个呼号，或用于互联网邮件的 SMTP:someone@example.org。多个之间用逗号分隔。
winlink-file-outbox = 存入发件箱
winlink-discard = 丢弃
winlink-filed-messages-go-out-next = 已存入的消息将在下一次会话中发出
winlink-mail-log = 邮件日志
winlink-session-running = 会话运行中……
winlink-no-session-yet = 尚无会话

## wspr (src/app/panels/wspr.rs)

wspr-receptions = 接收记录
wspr-save-wspr-reception-list-csv = 将 WSPR 接收列表保存为 CSV
wspr-wspr = WSPR
wspr-weak-signal-propagation-beacon = 弱信号传播信标
wspr-time-left-two-minute-slot = 此两分钟时隙的剩余时间。时隙结束时，录音会交给解码器，下一次发射决定随即生效。
wspr-tx = ● 发射
wspr-decoding = 解码中……
wspr-band = 频段
wspr-last-slot = 上一时隙
wspr-nothing-heard = 未听到任何内容
wspr-receive-only = 仅接收
wspr-next-slot = 下一时隙
wspr-waiting-radio-take-change = 正在等待电台接受更改……
wspr-hop-next = 下一跳
wspr-transmit = 发射
wspr-receive-only-2 = 仅接收。
wspr-power = 功率
wspr-what-you-actually-radiate = 你实际辐射的功率
wspr-goes-out-message-everyone-who = 此数值会包含在消息中，每个收听到你的人都会据此判断路径——因此这里若给出乐观的数值，他们的测量结果和你的都会出错。
wspr-roam = 漫游
wspr-pick-different-offset-inside-200 = 每次发射都在 200 Hz 窗口内选择不同的偏移。这是惯例：200 赫兹由所有人共用，只有在没有人停在其中央时才行得通。关闭则保持在光标所在的位置。
wspr-band-hop = 跨频段跳转
wspr-move-dial-band-band-between = 在时隙之间于各频段之间移动表盘，使一台接收机采样整个频谱，而不是其中的一个片段。若你自己转动 VFO，跳转会暂停——再次按下此项即可恢复。
wspr-upload = 上传
wspr-send-what-station-decodes-wsprnet = 将此电台解码的内容发送到 wsprnet.org。报告你所听到的内容，正是使 WSPR 接收机成为网络一部分的原因；它不会向空中发射任何内容。
wspr-who-heard-me = 谁收听到了我
wspr-ask-wsprnet-org-every-few = 每隔几分钟询问 wsprnet.org 哪些电台解码了此电台。WSPR 没有任何形式的确认，因此这是信标了解自身覆盖范围的唯一途径。
wspr-bands = 频段
wspr-none-selected-hopping-has-nowhere = 未选择任何频段——跳转无处可去
wspr-set-your-callsign-grid-general = 发射前，请在“设置”的“常规”选项卡中设置你的呼号和网格。

## packet (src/app/panels/packet.rs)

packet-monitor = 监视
packet-busy = 忙
packet-frames-arrived-but-failed-their = 已到达但未通过校验序列的帧——可能是碰撞、衰落，或信号太弱而无法读取。
packet-station-callsign-speed-tx-delay = 电台呼号、速率、发射延迟、数字中继路径、信标和 KISS 服务器
packet-starting-packet-modem = 正在启动分组调制解调器……
packet-nothing-heard-yet = 尚未听到任何内容
packet-terminal = 终端
packet-connected = {" "}已连接{" "}
packet-calling = {" "}呼叫中{" "}
packet-idle = {" "}空闲{" "}
packet-extended-sequence-numbers-window-up = 扩展序列号——窗口最多可达 127 帧。
packet-frames-sent-not-yet-acknowledged = 已发送但尚未被对方确认的帧。
packet-retries-against-link-s-limit = 针对此链路上限的重试次数。当未确认的帧保持不动而计数不断上升时，从此端看来，这就是衰落路径的样子。
packet-mail-window-using-link = 邮件窗口正在使用此链路。
packet-one-radio-one-channel-one = 一台电台、一个信道、一条链路。Winlink 会话和此终端是使用它的两种方式，谁先请求，谁就能使用。
packet-digipeaters-nearest-first-separated-commas = 数字中继，最近的在前，以逗号分隔——OE3XLR-1,OE3XMS-1。对于你能直接收听到的电台，或要使用分组设置中的默认路径，请将其留空。
packet-disconnect = {" "}断开{" "}
packet-hang-up-properly-wait-far = 正常挂断，并等待对方确认。改为切换模式则会结束会话，而不会告知任何人。
packet-connect = {" "}连接{" "}
packet-mail-window-has-link-finish = 邮件窗口占用着此链路。请先完成或停止该会话。
packet-station-has-no-callsign-yet = 此电台尚无呼号——请在监视面板上方的“设置”中设置一个。在设置之前，不会发射任何内容。
packet-call-station-connected-mode-node = 以连接模式呼叫此电台——节点、BBS 或另一位操作员。
packet-needs-callsign-call = 需要一个要呼叫的呼号。
packet-clear = {" "}清除{" "}
packet-empty-transcript-link-untouched-connected = 清空记录。链路不受影响——已连接的电台保持连接。
packet-not-connected-type-callsign-above = 未连接。请在上方输入一个呼号，然后按“连接”以呼叫节点或 BBS。
packet-type-here = 在此输入
packet-not-connected = 未连接
packet-send = {" "}发送{" "}
packet-send-line-node-bbs-reads = 发送此行。节点或 BBS 每次读取一行。

## fsq (src/app/panels/fsq.rs)

fsq-speed = 速率
fsq-call = 呼叫
fsq-callsign-directed-fsqcall-messages-defaults = 用于定向（FSQCALL）消息的呼号；默认为你的呼号
fsq-tx = ● 发射
fsq-contacts = 通联
fsq-heard = 收听
fsq-none = — 无 —
fsq-allcall = ALLCALL
fsq-image = 图片
fsq-clear = 清除
fsq-forget-pictures-received-so-far = 忘记迄今为止接收到的图片
fsq-send-image = 发送图片……
fsq-right-click-remove = 右键点击即可移除
fsq-remove-picture = 移除此图片
fsq-listening = — 收听中 —
fsq-message = 消息……
fsq-send = {" "}发送{" "}
fsq-call-cq = {" "}呼叫 CQ{" "}
fsq-heard-2 = {" "}？收听{" "}
fsq-clear-2 = {" "}清除{" "}
fsq-fsq-contacts = FSQ 通联
fsq-add = 添加

## publicsdr (src/app/publicsdr.rs)

publicsdr-tab = + 选项卡
publicsdr-open-receiver-new-radio-own = 将此接收机作为新电台打开，并位于其自己的选项卡中
publicsdr-point-radio-receiver-replacing-whatever = 将*此*电台指向该接收机，替换其当前所在的任何接口——会再次询问，因为那相当于整台电台的设置
publicsdr-not-stated = 未说明
publicsdr-replace = 替换
publicsdr-point-radio-you-are-receiver = 将你所在的电台指向此接收机
publicsdr-tab-instead = + 改为选项卡
publicsdr-open-receiver-another-radio-leave = 将接收机作为另一台电台打开，并保持此电台不变
publicsdr-cancel = 取消
publicsdr-public-sdrs = 公共 SDR
publicsdr-available = 可用
publicsdr-hide-receivers-are-full-ones = 隐藏已满的接收机，以及其操作员未向浏览器以外的应用开放任何通道的接收机
publicsdr-band = 在频段内
publicsdr-only-receivers-cover-current-dial = 仅显示覆盖当前表盘频率的接收机
publicsdr-low-bw = 低带宽
publicsdr-take-spyserver-low-bandwidth-shape = 以低带宽形式接入 SpyServer：一个跟随表盘的窄幅 I/Q 窗口，加上服务器自己的频段视图，而不是数兆比特的宽带 I/Q。对 KiwiSDR 无影响，因为它只有这一种形式。
publicsdr-refresh = ⟳ 刷新
publicsdr-fetch-both-directories-again = 重新获取两个目录
publicsdr-name-place-antenna-band = 名称、地点、天线、频段
publicsdr-clear-search = 清除搜索
publicsdr-fetching-receiver-lists = 正在获取接收机列表……
publicsdr-machine-radio-attached-does-not = 此电台所连接的机器不应答设备查询，因此它也无法获取列表。
publicsdr-no-match = 无匹配
publicsdr-nothing-matches-try-turning-filter = 没有任何内容匹配——请尝试重新打开某个过滤标签
publicsdr-radio = 此电台
publicsdr-radio-s-configuration-has-not = 此电台的配置尚未从它所在的机器送达，因此没有可指向接收机的目标——请改用“+ 选项卡”添加。
publicsdr-position-not-published-so-nothing = {" "}其位置未公布，因此它所听到的任何内容都不会被报告。

## hd (src/app/hd.rs)

hd-hd-radio = HD Radio
hd-waiting-receiver = 正在等待接收机……
hd-not-hd-radio-set-mode = 不在 HD Radio 模式。请在携带数字边带的调频广播上将模式设为 HD Radio——表盘应位于模拟载波的中心，而不是其旁边。
hd-no-hd-radio-lock-digital = 未锁定 HD Radio。数字边带的发射功率比模拟载波低约 20 dB，因此一个可以听到的电台，仍可能噪声太大而无法解码：在干净的信号上，解码器需要几秒钟。
hd-sync = 同步
hd-ofdm-frame-timing-recovered = OFDM 帧时序已恢复
hd-audio = 音频
hd-audio-frames-decoding = 音频帧正在解码
hd-quality = 质量
hd-lower = 下边带
hd-upper = 上边带
hd-last-minute = 最近一分钟
hd-waiting-locked-signal = 等待锁定的信号
hd-mer-l = MER L
hd-modulation-error-ratio-lower-digital = 下数字边带的调制误差比。
hd-mer-u = MER U
hd-modulation-error-ratio-upper-digital = 上数字边带的调制误差比。
hd-cber = CBER
hd-channel-bit-error-ratio-after = 内码之后的信道误码率。低于约 1e-3 时，外码可以纠正它；高于此值，音频就会开始断续。
hd-offset = 偏移
hd-residual-carrier-frequency-offset-large = 残余载波频率偏移。数值大且稳定，说明是接收机的参考有偏差，而非广播本身。
hd-psmi = PSMI
hd-primary-service-mode-indicator-which = 主服务模式指示器：电台正在发射的是哪一种调频混合服务模式。
hd-programme = 节目
hd-restricted-access = {" "}——受限访问

## bands (src/app/bands.rs)

bands-bands = 频段
bands-daylight = ☀ 白天
bands-night = ☾ 夜间
bands-no-forecast-yet-fetching-hamqsl = · 尚无预报——正在从 HAMQSL.com 获取
bands-band = 频段
bands-condx = CONDX
bands-paths = 路径
bands-reach = 覆盖
bands-best = 最佳
bands-ibp-beacons = IBP 信标
bands-meteor-showers = 流星雨
bands-peak = 峰值

## spots (src/app/spots.rs)

spots-pota = POTA
spots-sota = SOTA
spots-freedv = FREEDV
spots-spots = 信号
spots-longwave-shortwave-broadcast-stations-air = 当前正在播出的长波和短波广播电台
spots-view = 视图内
spots-only-spots-inside-panadapter-span = 仅显示全景图跨度之内的信号
spots-setup = ⚙ 设置
spots-feeds-lookup-upload-settings = 数据源、查询和上传设置
spots-call-station-site-frequency = 呼号、电台、站点、频率
spots-clear-search = 清除搜索
spots-no-match = 无匹配
spots-no-spots-enable-feed-setup = 没有信号——请在 ⚙ 设置 中启用一个数据源
spots-nothing-matches-search = 没有任何内容匹配搜索

## ism (src/app/ism.rs)

ism-ism-devices = ISM 设备
ism-decoding = 解码中
ism-needs-wideband-iq-source-cat = 需要宽带 IQ 信源；CAT 电台只发送音频
ism-how-far-burst-must-stand = 突发信号在被解码之前，必须高出该信道自身底噪多少
ism-give-each-band-width-normally = 为每个频段提供其通常所需的宽度——315、345 和 433 MHz 的 OOK 频段需要四分之一兆赫，868 和 915 则需要一整个兆赫。
ism-several-hundred-more-device-decoders = 另有数百个设备解码器，来自 rtl_433 项目，已内置。可读取 sdroxide 自身解码器完全无法触及的 433 MHz OOK 设备。凡是两者都能读取的设备，rtl_433 知道的变体要多得多，并会接管。
ism-t-1-your-own = 1 个你自己的
ism-device-decoders-registered-how-many = 已注册的设备解码器、其中来自你的 rtl433_flex.conf 的数量，以及自启动以来已解码的发射数。
ism-reload-decoders = 重新加载解码器
ism-re-read-rtl433-flex-conf = 编辑后重新读取 rtl433_flex.conf。已收听到的设备仍会保留在列表中。
ism-rtl-433-knows-more-variants = rtl_433 比 sdroxide 自身的解码器了解这些设备的更多变体，因此在此收听时由它读取。选择另一个频段，sdroxide 自身的解码器便会重新接管。
ism-decoder-your-rtl433-flex-conf = 你的 rtl433_flex.conf 中的一个解码器被拒绝。其余的已正常加载。请修正该行，然后按“重新加载解码器”。
ism-waiting-decoder = 正在等待解码器……
ism-where-decoder-actually-listening-follows = 解码器实际在收听的位置。它跟随接收机的硬件中心，而不是 VFO——在宽幅前端上，两者可能相距甚远，而决定哪些信道可以到达的正是这个窗口。
ism-transmissions-gate-opened-how-many = 闸门打开过的发射次数，以及其中产生有效帧的次数。突发很多而解码很少，说明频段上充斥着 sdroxide 尚无法读取的设备。完全没有，则说明没有任何东西达到阈值——请调低阈值，或检查表盘是否在该频段上。
ism-nothing-heard-yet = 尚未听到任何内容
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

navtex-messages = 消息
navtex-sync = 同步
navtex-character-phase-locked-signal-being = 字符相位已锁定——正在读取信号。
navtex-characters-whose-first-copy-was = 首次副本已损坏、取自五个时隙之后的重复副本的字符，以及两份副本都已损坏的字符。NAVTEX 的每个字符都会发送两次；这就是它全部的纠错手段。
navtex-swap-mark-space-tones-signal = 交换标记音和空号音，适用于在另一个边带上接收到的信号。关闭即为信道频率上的上边带，这也是该服务所有已发布的调谐说明所采用的方式。
navtex-starting-navtex-receiver = 正在启动 NAVTEX 接收机……
navtex-nothing-yet-station-transmits-ten = 尚无内容——电台每四小时发射十分钟
navtex-cut-short = 被提前切断
navtex-message = 消息
navtex-time-day-stated-message-utc = 消息中所述的时刻（UTC）：仅供显示
navtex-receiving = 接收中
navtex-monitor = 监视
navtex-everything-decoded-message-not = 所有解码内容，无论是否为消息

## rfpaint (src/app/panels/rf_paint.rs)

rfpaint-rf-paint = 射频绘图
rfpaint-scan-speed = 扫描速度
rfpaint-how-fast-text-image-scanned = 文字或图像被扫描到瀑布图上的速度。越低越慢、越清晰；100% = 基准速率，25%（中间）为默认值。
rfpaint-abort = 中止
rfpaint-text-paint = 文字绘图
rfpaint-type-text-paint = 输入要绘制的文字……
rfpaint-preview-waterfall = 预览瀑布图
rfpaint-type-text-preview = 输入文字以预览
rfpaint-transmit = {"  "}发射{"  "}
rfpaint-image-paint = 图像绘图
rfpaint-no-image-loaded = 未加载图像
rfpaint-load-image = 加载图像……
rfpaint-load-image-preview = 加载一张图像以预览

## hfdl (src/app/hfdl.rs)

hfdl-listen = 收听
hfdl-switches-engine-s-hfdl-downconverter = 开启或关闭引擎的 HFDL 下变频器和解码器。无论窗口是否打开，解码都会占用一个 24 kHz 的通道和一个工作线程。
hfdl-channel-assigned-frequency-listen-anything = 要收听的信道（指定频率）。频段内的任何频率都可以——HFDL 电台在 2.8 至 22 MHz 发射——但会对准这里居中的固定 24 kHz 通道。
hfdl-save-hfdl-decode-log-file = 将 HFDL 解码日志保存到文件
hfdl-riverhead-northern-atlantic-north-america = Riverhead（北大西洋 / 北美）——默认值，也是解码器验证时所用的信道。
hfdl-hfdl-assigned-frequency = 一个 HFDL 指定频率。
hfdl-one-your-saved-channels-click = 你已保存的信道之一——点击即可调谐，右键点击即可移除。
hfdl-hfdl-selected-but-decoding-off = 已选择 HFDL，但解码已关闭——请按上方的“收听”
hfdl-decodes = 解码
hfdl-nothing-decoded-yet-ground-station = 尚无解码内容——一旦信道中出现地面站，其信标每约 32 秒重复一次。
hfdl-no-decode-matches-filter = 没有解码内容符合过滤条件。
hfdl-decoding = 解码中
hfdl-decoding-off = 解码已关闭
hfdl-dbfs = — dBFS

## rade (src/app/panels/rade.rs)

rade-rade = RADE
rade-freedv-v1-digital-voice = FreeDV V1 数字话音
rade-tx = ● 发射
rade-mute-analog = 静音模拟信号
rade-mute-demodulated-audio-so-only = 静音解调音频，使只能听到解码出的语音。在其他情况下，每当调制解调器没有可播放的内容时，原始信号都会被直接传出——正是这种嘶嘶声让你能在同步之前找到一次发射，因此调谐时请保持此项关闭。
rade-sync = 同步
rade-how-far-received-signal-sits = 接收到的信号与调制解调器所期望的位置相差多远。数值大的仍能解码——捕获环路会跟踪它们——但稍微移动表盘使其接近零，可获得最佳余量。
rade-snr = 信噪比 —
rade-callsign-station-put-end-over =
    电台在其通话结束（End-of-Over）帧中放入的呼号——这是 RADE 通话中唯一发送它的时机。它也是报告给 FreeDV Reporter 的内容，因此你所收听到的电台会在 qso.freedv.org 上显示为由你收听到。

    Held for a minute after the signal goes, and dropped when you start an over of your own: it names the station on frequency, not every station the receiver has ever heard.
rade-stop-talking = 停止说话
rade-talk-ptt = 说话（PTT）
rade-open-close-rade-over-modem = 开始或结束一次 RADE 通话。调制解调器在第一帧发出之前需要约 120 ms 的语音，并在你停止时发送一个通话结束帧，因此发射会在按钮松开后再持续一小段时间。
rade-rade-v1-occupies-roughly-1060 = RADE V1 占用通带中约 1060–1880 Hz——上边带，或在 160、80 和 40 米上为下边带，RADE 在这些频段上保留了电话练习，sdroxide 会为你切换。将信号置于瀑布图上的阴影频带之内；调制解调器便会从那里找到它。

## frame (src/app/frame.rs)

frame-acknowledge = 确认
frame-dismiss = 关闭
frame-reconnecting = 正在重新连接……
frame-not-connected = 未连接。
frame-reconnect-now = 立即重新连接
frame-transmit-lockout-disabled = ⚠ 发射锁定已禁用
frame-engine-was-started-oob-tx = 此引擎是以 --oob-tx 启动的。业余频段锁定已关闭：它将在硬件支持的任何频率上键控发射机。
frame-transmitting-outside-your-licence-offence = 在执照范围之外发射，在每个颁发执照的国家都属于违法行为。只有当你获得授权使用即将键控的频率时才应继续——例如 MARS/CAP 或商业执照、实验许可证，或假负载。
frame-i-understand = {"  "}我已了解{"  "}
frame-restart-without-oob-tx-put = 不带 --oob-tx 重新启动，以恢复锁定。

## smeter (src/widgets/smeter.rs)

smeter-alc-how-hard-transmitter-being = ALC——发射机被驱动的力度，以其最大输出的百分比表示。100% 是满量程，而不是以瓦计的满功率：它在天线上相当于多少，取决于电台和驱动设置。
smeter-radio-reports-no-forward-power =


    This radio reports no forward power, so the figure beside ALC is the ALC reading again rather than watts.
smeter-no-swr-radio-has-no =


    No SWR: this radio has no bridge to measure it with, and sdroxide will not invent one — it can only show what the radio itself reports. A rig with an SWR meter (over CAT, TCI or a LAN link) and an HPSDR board both fill this in; a plain SDR transmitter cannot.
smeter-no-signal-report-yet-receiver = 尚无信号报告——接收机尚未提供表头读数。
smeter-s9-73-dbm-each-s =


    S9 is −73 dBm and each S-unit below it is 6 dB. The dBm figure is only as honest as the front end's calibration — an uncalibrated receiver reports dBFS with a dBm label.
smeter-ovld-converter-into-rails-so =


    OVLD: the converter is into its rails, so the level shown understates the signal. Wind the front-end gain back.
smeter-table-held = ，表格已保持
smeter-ps = PS --

## multi (src/multi.rs)

multi-tx = ● 发射
multi-link = 功率
multi-mute-radio-s-audio = 静音此电台的音频
multi-close-radio-s-split-view = 关闭此电台的分屏视图
multi-open-radio-split-view-own = 在其自己的分屏视图中打开此电台
multi-already-open-another-split-view = 已在另一个分屏视图中打开
multi-show-radio-here = 在此处显示此电台
multi-add-radio-computer-settings-radio = 在本计算机上添加电台（若要在电台站点上添加，请使用“设置 → 电台”）
multi-add-radio = 添加电台
multi-asked-station-another-radio = 已向电台站点请求另一台电台……
multi-client-cannot-open-connection = 此客户端无法打开连接。

## net (src/app/net.rs)

net-fail = 失败
net-downloading = 正在下载……
net-adif-import-failed-file-could = ADIF 导入失败：文件无法解析
net-channel-import-failed-file-could = 信道导入失败：文件无法读取
net-channel-import-nothing-read-chirp = 信道导入：没有可读取的内容——CHIRP CSV 文件以一行表头开始，表头列出其列名，其中必须包含“Frequency”

## widgets (src/app/panels/widgets.rs)

widgets-free-text = 自由文本
widgets-hashed-callsign-not-yet-heard = 哈希呼号，尚未完整听到
widgets-entity-unknown = 实体未知
widgets-band = 此频段
widgets-new-entity-never-worked-any = 新实体——从未通联过，任何频段均无
widgets-new-grid-square = 新网格方格
widgets-not-worked-before = 此前未通联
widgets-worked-before-but-not-band = 此前已通联，但不在此频段
widgets-call-queue = 在呼叫队列中
widgets-image = 图像

## pi4 (src/app/panels/pi4.rs)

pi4-receptions = 接收记录
pi4-save-pi4-reception-list-csv = 将 PI4 接收列表保存为 CSV
pi4-next-generation-beacon = 下一代信标
pi4-time-left-one-minute-beacon = 此一分钟信标周期的剩余时间。在窗口有足够时间填充到其结束点之后，才会搜索 PI4 消息。
pi4-decoding = 解码中……
pi4-band = 频段
pi4-last-cycle = 上一周期
pi4-nothing-heard = 未听到任何内容
pi4-receive-only-tune-so-beacon = 仅接收。请调谐，使信标的 CW 识别位于 800 Hz 音频处——即该网络自身的惯例——并使四个 PI4 音调落在此解码器默认搜索它们的位置。

## awards (src/app/awards.rs)

awards-awards = 奖项
awards-band = 频段
awards-all = 全部
awards-dxcc = DXCC
awards-grids = 网格
awards-worked-all-states = 通联全部州
awards-cq-zones-waz = CQ 区（WAZ）
awards-dxcc-entities = DXCC 实体

## acars (src/app/panels/acars.rs)

acars-acars = ACARS
acars-audio-decoder-s-passband = 解码器通带内的音频。
acars-blocks-framed-blocks-whose-check = 已成帧的数据块，以及校验序列失败的数据块。
acars-starting-acars-receiver = 正在启动 ACARS 接收机……
acars-channel = 信道
acars-tune-dial-here = 在此调谐表盘。

acars-channel-out-of-range = 超出本电台的接收范围。
acars-nothing-yet-acars-quiet-between = 尚无内容。ACARS 在飞机之间保持安静——将其留在一个信道上，等待突发信号。
acars-check-failed = 校验失败
acars-block-check-sequence-did-not = 数据块校验序列不匹配——文字可能有误。
acars-no-text = （无文字）

## login (src/login.rs)

login-sdr-oxide = SDR OXIDE
login-username = 用户名
login-password = 密码
login-remember-device = 在此设备上记住
login-signs-without-asking-next-time = 下次无需询问即可登录，并且 3D 视图的选项卡也无需询问即可进入。密码以明文保存，因此在他人也会使用的机器上请关闭此项。
login-keeps-password-device-clear = 将密码以明文保存在此设备上。
login-checking = {" "}正在检查……{" "}
login-sign = {" "}登录{" "}
login-wrong-password-shuts-server-s = 密码错误会使服务器的大门关闭数秒，因此可能需要一点时间。

## help (src/help.rs)

help-find = 查找
help-find-manual = 在手册中查找
help-previous-match-shift-enter-shift = 上一个匹配项（Shift+Enter、Shift+F3）
help-next-match-enter-f3 = 下一个匹配项（Enter、F3）
help-no-matches = 无匹配
help-clear-search-esc = 清除搜索（Esc）
help-sdroxide-manual = SDROXIDE 手册
help-contents = 目录

## aismap (src/ais_map.rs)

aismap-t-1-station-heard-waiting-report = 已听到 1 个电台——正在等待其报告位置
aismap-double-click-reframe = 双击重新取景
aismap-zoom-out-wheel-pinch-do = 缩小——滚轮或双指捏合同样可以
aismap-zoom-about-middle-chart = 以海图中央为中心放大
aismap-frame-everything-being-tracked-again = 重新框取所有正在跟踪的内容——双击也可以

## specview (src/widgets/spectrum_view.rs)

specview-waiting-spectrum = 等待频谱……
specview-hounds = HOUNDS

## spectrum (src/app/spectrum.rs)

spectrum-window-has-no-gpu-renderer = 此窗口没有 GPU 渲染器，因此瀑布图保持标准宽度。
spectrum-machine-drawing-without-gpu-every = 此机器在没有 GPU 的情况下绘图——瀑布图的每一列都由处理器完成，而处理器已经与电台共享。
spectrum-window-renders-through-opengl-which = 此窗口通过 OpenGL 渲染，这是 sdroxide 的兼容路径——例如 Raspberry Pi、较旧的显卡，或不支持 WebGPU 的浏览器。在那里，更宽的瀑布图不值得牺牲帧率。
spectrum-wider-than-renderer-can-draw = 比此渲染器所能绘制的更宽。

## macros (src/app/panels/macros.rs)

macros-nocall = 无呼号
macros-remove-button = 删除此按钮

## gridtracker (src/app/grid_tracker.rs)

gridtracker-grid-tracker = 网格追踪器
gridtracker-drag-pan-wheel-zoom-amber = 拖动以平移，滚轮以缩放。琥珀色 = 已通联，绿色 = 已确认，青色 = 已收听。

## remote (src/remote.rs)

remote-server-busy-another-client-connected = 服务器忙——另一个客户端已连接
remote-transmitting-but-client-s-microphone = 正在发射，但此客户端的话筒完全没有产生任何音频——因此没有任何东西被调制。在浏览器中，请检查该页面是否获得了话筒权限，以及是否有其他应用占用着它。
remote-connection-closed = 连接已关闭

## solarwin (src/solar3d/mod.rs)


## solarbtn (src/app/solar.rs)

solarbtn-solar-system-3d-view-sun = 太阳系三维视图——太阳、地球、月球、太阳黑子和日冕物质抛射（独立窗口）
solarbtn-solar-system-3d-view-sun-2 = 太阳系三维视图——太阳、地球、月球、太阳黑子和日冕物质抛射（新的浏览器标签页）

## widespec (src/widgets/wide_spectrum.rs)


## speech (src/app/speech.rs)

speech-loading-voice = 正在加载语音……
speech-browser-client-cannot-speak-yet = 浏览器客户端尚无法发声

## freqdisplay (src/widgets/freq_display.rs)

freqdisplay-double-click-type-frequency = 双击以输入频率
freqdisplay-tap-type-frequency = 点按以输入频率

## solarstate (src/solar3d/state.rs)

solarstate-home = 首页
solarstate-dwarf-planets = 矮行星

## adsbmap (src/adsb_map.rs)

adsbmap-double-click-reframe = 双击重新取景

## aprsmap (src/aprs_map.rs)

aprsmap-double-click-reframe = 双击重新取景

## solarnet (src/solar3d/net.rs)

solarnet-connection-closed = 连接已关闭

## alerts (src/app/alerts.rs)

alerts-could-not-start-alert-thread = 无法启动警报线程

## hfdlmap (src/hfdl_map.rs)

hfdlmap-no-aircraft-located-yet = 尚未定位到任何飞机

## worldmap (src/widgets/worldmap.rs)

worldmap-double-click-reframe = 双击重新取景

## chrome (src/chrome.rs)



## duopower (src/app/duo_power.rs)

duopower-sets-fdm-duo-s-own = 设置 FDM-DUO 自身的功率（菜单 34 TX POWER）。驱动和调谐随后在此范围内设置功率，而 SDRoxide 每次连接时都会重新发送此设置。

## Band conditions window: a peak date.

# $month is 1–12.
bands-peak-date = { $month } 月 { $day } 日

## download (src/download.rs)

download-browser-could-not-read-file = 浏览器无法读取该文件

## morse (src/app/morse.rs)

morse-translate = 翻译
morse-practice = 练习
morse-learn = 学习
morse-morse = 摩尔斯
morse-text-morse = 文字 → 摩尔斯
morse-morse-text = 摩尔斯 → 文字
morse-per-character-space-between-characters = 每个字符用 `.` 和 `-` 表示，字符之间有一个空格，单词之间为 ` / `。采用国际电信联盟字母表、数字、标点以及常用的程序符号。
morse-reference-translator-available-desktop-build = 桌面版本提供参考翻译器。
morse-play-what-you-type-your = 通过你自己的扬声器播放你所键入的内容
morse-play = 播放
morse-stop = 停止
morse-every-character-learned-well-done = 所有字符都已学会——做得好。
morse-reveal = 显示
morse-reset = 重置
morse-replay = 重播
morse-what-did-you-hear = 你听到了什么？
morse-check = 检查
morse-press-new-hear-character = 按“新建”以听到一个字符。
morse-tone = 音调
morse-speed = 速度
morse-spacing = 间距
morse-characters-go-out-speed-above = 字符以上方的速度发出；而*间隔*则以这一速度发送，这样初学者在能够以那么快的速度发送之前，就能先把字母作为一个整体听到。
morse-playback-available-desktop-build = 桌面版本提供回放功能。

## ADS-B panel: aircraft detail rows.

adsb-row-altitude = 高度
adsb-row-track = 航迹
adsb-row-turning = 转弯
adsb-row-squawk = 应答码
adsb-row-range = 距离
adsb-row-position = 位置
adsb-row-signal = 信号
adsb-row-frames = 帧
rade-transmitting = 发射中
decodes-odd = 奇数
decodes-new = 新
js8-new = 新

## AIS panel: vessel detail rows.

ais-row-status = 状态
ais-row-speed = 速度
ais-row-course = 航向
ais-row-heading = 船首向
ais-row-turning = 转向
ais-row-altitude = 高度
ais-row-size = 尺寸
ais-row-draught = 吃水
ais-row-destination = 目的地
ais-row-range = 距离
ais-row-signal = 信号
ais-row-type = 类型
ais-row-position = 位置
ais-row-messages = 消息
ism-off = 关
textmodem-frequency-shifted = 频移
winlink-to-heading = 收件人

## adsbmap: messages with values (src/adsb_map.rs)


## aismap: messages with values (src/ais_map.rs)

aismap-heard-no-position-newer-than = 已听到 { $heard } 个——没有比 { $drop_map_s } 秒更新的位置
aismap-stations-heard-waiting-position-report = 已听到 { $heard } 个电台——正在等待位置报告
aismap-km-across = { $lon_span }° · 横跨约 { $km } km

## alerts: messages with values (src/app/alerts.rs)

alerts-alarming = 正在通过 { $device } 发出警报
alerts-alarm-device-unavailable = 警报设备不可用：{ $error }

## awards: messages with values (src/app/awards.rs)

awards-worked = 已通联 { $w }
awards-confirmed = 已确认 { $value }

## bands: messages with values (src/app/bands.rs)

bands-forecast-hamqsl-com-old = · 预报来自 HAMQSL.com，已过 { $value }
bands-ncdxf-iaru-next-beacon-s = NCDXF/IARU · 下一个信标在 { $now } 秒后
bands-radiants-placed = 已为 { $lat }°、{ $lon }° 放置辐射点
bands-zhr = ZHR { $zhr }
bands-radiant = 辐射点 { $alt_deg }° { $az_deg }
bands-radiant-down = 辐射点在下方（{ $alt_deg }°）

    A radiant above the horizon means the trails can reach you; a fast shower leaves longer-lived ionised trails for meteor scatter on 6 m and 2 m, and the brief 10 m openings.

## drm: messages with values (src/app/drm.rs)

drm-not-decoding = { $channel } 未在解码
drm-symbols = { $value } 个符号 · { $frame }
drm-not-decodable = { $value }——无法解码
drm-broadcaster-s-clock-utc = 广播方时钟 { $year }-{ $month }-{ $day } { $hour }:{ $minute } UTC

## duopower: messages with values (src/app/duo_power.rs)

duopower-max-fdm-duo-s-full =
    最大：FDM-DUO 的满功率输出，约 8 W。在接入 HR500 时应使用的设置。

    { $base }
duopower-t-5-w-hr500-line-will =
    5 W。在接入 HR500 的情况下，此设置在大多数频段上都不会达到满输出：请使用“最大”，并通过驱动将其调低。

    { $base }
duopower-w-running-barefoot-qrp =
    { $code } W，适用于不接功放的运行或 QRP。

    { $base }

## frame: messages with values (src/app/frame.rs)

frame-re-enables-transmit-after-1 = 在 { $swr }:1 之后重新启用发射。请先检查天线：若故障仍然存在，下一次发射同样会被停止。
frame-reconnecting-s = { $value } 秒后重新连接……

## gridtracker: messages with values (src/app/grid_tracker.rs)

gridtracker-worked = 已通联 { $worked }
gridtracker-confirmed = 已确认 { $confirmed }
gridtracker-heard = 已收听 { $heard }

## hd: messages with values (src/app/hd.rs)

hd-decode = 解码 { $value }（{ $codec_label }）

## hfdl: messages with values (src/app/hfdl.rs)

hfdl-khz-already-has-chip = { $cur_khz } kHz 已有一个芯片
hfdl-save-current-channel-khz-one = 将当前信道（{ $cur_khz } kHz）保存为你自己的一个芯片
hfdl-aircraft = { $aircraft } 架飞机
hfdl-bursts = { $bursts } 次突发
hfdl-decodes-2 = { $decodes } 次解码
hfdl-fixed = 已纠正 { $fec }

## ism: messages with values (src/app/ism.rs)

ism-not-decoded-yet = { $hint }——尚未解码
ism-listen-tunes-radio-there-one = 收听 { $value }——会将电台调谐到那里。一次一个频段：它们相距太远，任何接收机都无法同时覆盖两个。
ism-watch-hz-around-band-centre = 监视频段中心两侧 { $value }Hz 的范围，无论该频段原本会要求多少。较窄的范围适合无法提供该频段自身宽度的接收机；较宽的范围则能覆盖位于中心更远处的设备。你的接收机必须多提供大约三分之一的带宽。
ism-rtl-433 = rtl_433——{ $why }
ism-rtl-433-khz = rtl_433 { $value } · { $khz } kHz
ism-your-own = 你自己的 { $flex } 个
ism-decoders-decoded = { $decoders } 个解码器（{ $flex }），已解码 { $decodes }
ism-handled-rtl-433 = 由 rtl_433 处理：{ $names }
ism-flex = flex：{ $error }
ism-window-mhz-mhz-wide = 窗口 { $mhz } MHz，宽 { $e62 } MHz
ism-bursts-decoded = { $bursts } 次突发，已解码 { $decodes }
ism-devices = { $ism_reports } 个设备
ism-id-mhz-db-heard-first =
    { $protocol } 标识 { $device }
    { $e6 } MHz { $snr_db } dB 已收听 { $value } 次
    首次收听于 { $first_at } 前

## logbook: messages with values (src/app/logbook.rs)

logbook-confirmations-downloaded-newly-confirmed = 确认信息：已下载 { $recs } 条，新确认 { $matched } 条
logbook-qso = { $qso_log } 次通联
logbook-worked-before = ⚠ 此前已通联（{ $dupe_band }）
logbook-utc-qso = { $oldest }–{ $newest } UTC · { $value } 次通联

## app: messages with values (src/app/mod.rs)

app-radio = 电台 { $id }

## morse: messages with values (src/app/morse.rs)

morse-could-not-start-morse-audio = 无法启动摩尔斯音频线程：{ $error }
morse-no-audio-output = 没有音频输出：{ $error }
morse-about-s = 约 { $farnsworth_wpm } 秒
morse-characters-play = 正在练习的字符：{ $set }
morse-score-run = 得分：{ $correct } / { $total }。连续正确：{ $streak }（共 { $value }）。
morse-correct = 正确——{ $value }
morse-was = 那是 { $value }

## net: messages with values (src/app/net.rs)

net-adif-import-failed = ADIF 导入失败：{ $error }
net-not-unicode-read = {" "}（非 Unicode；按 { $enc } 读取）
net-received-reports-swl-left-out = ，已排除 { $n } 条接收报告（SWL）
net-adif-import-added-duplicates-skipped = ADIF 导入：新增 { $added } 条，跳过 { $skipped } 条重复项{ $reports }{ $assumed }
net-channel-import-failed = 信道导入失败：{ $error }
net-line-s-skipped = ，已跳过 { $n } 行
net-not-unicode-read-2 = {" "}（非 Unicode；按 { $enc } 读取）
net-channel-import-channel-s-read = 信道导入：已读取 { $n } 个信道{ $skipped }{ $assumed }

## acars: messages with values (src/app/panels/acars.rs)

acars-frames-bad = { $frames } 帧 · { $bad } 个错误
acars-mode = 模式 { $mode }

## adsb: messages with values (src/app/panels/adsb.rs)

adsb-aircraft-3 = { $aircraft } 架飞机
adsb-frames = { $frames } 帧
adsb-preambles = { $preambles } 个前导码
adsb-bad-crc = { $bad_crc } 个 CRC 错误
adsb-tune = 调谐 { $mhz }
adsb-ft-barometric = { $ft } 英尺（气压高度）

## ais: messages with values (src/app/panels/ais.rs)

ais-vessels-3 = { $vessels } 艘船舶
ais-messages = { $messages } 条消息
ais-slots = { $bursts } 个时隙
ais-bad-fcs = { $bad_fcs } 个 FCS 错误
ais-ais-mhz = AIS { $value }（{ $mhz } MHz）——{ $value2 }
ais-ais-mhz-slots-messages-floor = AIS { $value }（{ $mhz } MHz）——{ $bursts } 个时隙，{ $messages } 条消息，底噪 { $floor_dbfs } dBFS
ais-ais = AIS { $value }
ais-tune = 调谐 { $mhz }
ais-ships-heard-are-khz-off = 所听到的船舶偏离频率 { $khz } kHz（162 MHz 下为 { $mhz } ppm）——请设置前端的频率校正，否则超过约 5 kHz 的发射将无法解码
ais-over-ground = 对地 { $value }°
ais-dbfs-db-snr = { $rssi_dbfs } dBFS，信噪比 { $snr_db } dB
ais-last-type-ais = { $messages }（最后类型 { $last_type }，AIS { $channel }）
ais-ais-2 = AIS { $ch }
ais-mhz-marine-channel-ship-alternates = { $mhz } MHz——海事信道 { $ch }。船舶在两者之间交替发射，因此关闭其中一个会使每艘船被听到的频率减半。

## aprs: messages with values (src/app/panels/aprs.rs)

aprs-object = 来自 { $reported_by } 的对象
aprs-via = 经由 { $via }
aprs-object-reported = 对象由 { $reported_by } 报告
aprs-digit-s-blanked = ± { $ambiguity } 位数字被隐去
aprs-wind-m-s = 风 { $value }° { $value2 } m/s
aprs-gust = 阵风 { $value }
aprs-rain-mm-h = 降雨 { $value } mm/h
aprs-frame-s-last-ago = { $packets } 帧，最近一帧距今 { $last_heard }
aprs-via-2 = 经由 { $via }
aprs-bad = { $bad_frames } 个错误
aprs-other = { $non_aprs } 个其他
aprs-sent = 已发送 ×{ $tries }
aprs-stn = { $stations } 个电台
aprs-queued = { $tx_queue } 个排队中

## atchat: messages with values (src/app/panels/atchat.rs)

atchat-master = 主节点 { $value }
atchat-last-heard-s-ago-click = { $status }——最近一次收听于 { $age_s } 秒前 · 点击打开一个私信标签页
atchat-close-tab = 关闭 { $peer } 标签页
atchat-direct-message = 发给 { $value } 的私信
atchat-cannot-display = 无法显示 { $path }

## cw: messages with values (src/app/panels/cw.rs)

cw-click-decoder-neural-one-copies =
    { $hint }

    Click for the { $neural } decoder. The neural one copies further down and reads hand-sent CW a timing fit will not accept; the timing one is the only one that copies the accented letters — Ä, Ö, Å, Ü, É — because the model has no output class for them.
cw-idle-s = 空闲 { $idle } 秒

## decodes: messages with values (src/app/panels/decodes.rs)

decodes-save-decodes-list-adif-received = 将列表中的 { $n } 条解码保存为 ADIF——即接收报告，而非通联。未指明发送方的解码将被排除。
decodes-save-decodes-list-csv-one = 将列表中的 { $n } 条解码保存为 CSV，每条一行
decodes-session-qso = 本次会话：{ $session } 次通联
decodes-save-whole-logbook-qso-adif = 将整个日志（{ $logged } 次通联）保存为 ADIF
decodes-save-whole-logbook-qso-text = 将整个日志（{ $logged } 次通联）保存为文本
decodes-hz-slots = { $audio_hz } Hz · { $odd } 个时隙
decodes-your-slot-timing-against-stations =
    你的时隙计时与你所收听的电台之间的对比。
    { $after }

    It covers the whole receive path, so a slow audio or network chain counts the same as a wrong clock. Under 0.5 s is comfortable.
decodes-pile-up = 大批呼叫 { $fox_queue }
decodes-queue = 队列 { $call_queue }
decodes-db-hz-click-remove =
    { $waiting } · { $snr_db } dB · { $audio_hz } Hz{ $unwrap_or_default }
    点击以移除

## js8: messages with values (src/app/panels/js8.rs)

js8-every-speed-being-decoded-marks = 所有速率都在解码；▸ 标记 { $speed }，即你发射所用的速率
js8-every-speed-being-decoded-click = 所有速率都在解码。点击改为以 { $speed } 发射
js8-transmit-decode = 以 { $speed } 发射并解码
js8-beaconing-every-min-click-stop = 每 { $hb_min } 分钟发送一次信标——点击即可停止
js8-last-beacon-went-out-hz = 最近一次信标在 { $hz } Hz 发出——位于 { $value }–{ $value2 } Hz 心跳子频段中的一个空闲时隙，之所以选择它，是为了使其避开正在被解码的信号。
js8-frames-s = { $tx_frames_pending }/{ $tx_frames_total } 帧 · { $left } 秒
js8-decoded = 以 { $speed } 解码
js8-frames = …（{ $frames } 帧）
js8-reply-2 = 回复 { $value }：“{ $value2 }”
js8-address-composer = 将编写框的收件人设为 { $value }

## macros: messages with values (src/app/panels/macros.rs)

macros-f-sends = F{ $i }：发送“{ $text }”
macros-sends = 发送“{ $text }”
macros-most = { $value } 为最多

## navtex: messages with values (src/app/panels/navtex.rs)

navtex-repaired-lost = 已修复 { $repaired } · 丢失 { $lost }
navtex-characters-lost = 丢失 { $lost } 个字符

## packet: messages with values (src/app/panels/packet.rs)

packet-bad = { $bad_frames } 个错误
packet-link-layer = 链路层处于 { $link }
packet-via = {" "}经由 { $via }
packet-unacked = { $unacked } 个未确认
packet-retry = 重试 { $retries }

## pi4: messages with values (src/app/panels/pi4.rs)

pi4-beacon = { $last_slot_spots } 个信标
pi4-utc-ago = { $t }:{ $t2 } UTC——{ $slot_utc } 前

## rade: messages with values (src/app/panels/rade.rs)

rade-snr-db = 信噪比 { $snr_db } dB
rade-offset-hz = 偏移 { $freq_offset_hz } Hz
rade-heard = 收听到 { $call }
rade-samples-dropped-between-receiver-decoder = ⚠ 接收机与解码器之间丢失了 { $dropped } 个采样——此机器跟不上神经网络解码的速度。

## textmodem: messages with values (src/app/panels/text_modem.rs)

textmodem-char-s = { $chars_per_sec } 字符/秒
textmodem-char-s-hz-wide = { $value }——{ $chars_per_sec } 字符/秒，宽 { $bandwidth_hz } Hz，{ $keyed }

## vdl2: messages with values (src/app/panels/vdl2.rs)

vdl2-frames = { $frames } 帧
vdl2-bursts = { $bursts } 次突发
vdl2-sync = { $syncs } 次同步
vdl2-hdlc-bad = HDLC 错误 { $hdlc_bad }
vdl2-bad-fcs = { $fcs_bad } 个 FCS 错误
vdl2-mhz-bursts-frames-noise-floor = { $mhz } MHz，{ $what }——{ $bursts } 次突发，{ $frames } 帧，底噪 { $floor_dbfs } dBFS
vdl2-tune = 调谐 { $mhz }
vdl2-held-2 = 已暂停 +{ $since }
vdl2-use-worldwide-where-every-link = { $what }——全球通用，也是每条链路开始的地方。若只保留一个，请保留这个。
vdl2-assigned-which-channels-carry-anything = 分配给 { $what }。哪些信道承载内容取决于你所在的位置：除非另有所知，请全部保持开启。
vdl2-mhz-db-snr-dbfs-evm = { $mhz } MHz · 信噪比 { $snr_db } dB · { $rssi_dbfs } dBFS · EVM { $evm_deg }° · { $freq_err_hz } Hz · 已纠正 { $rs_corrected } 个 RS 符号
vdl2-characters = { $parity_errors } 个字符
vdl2-parameter-sdroxide-does-not-read = { $unknown } 个 SDRoxide 不读取的参数——见上方，以标识符和字节显示

## wefax: messages with values (src/app/panels/wefax.rs)

wefax-lines-held = 已保留 { $live_size } 行
wefax-pixels-per-line = 每行 { $width } 个像素
wefax-shift-picture-pixels = 将图片平移 { $px } 个像素
wefax-charts-are-saved-click-copy =
    图表保存在 { $place }

    Click to copy the path
wefax-finished-charts-are-saved-collect = 完成的图表保存在 { $place }，并收集在此处。
wefax-pixels-right-click-delete =
    { $when_full }
    { $size } × { $size2 } 像素
    { $value }

    Right-click to delete
wefax-older-store = 存储中另有 { $older } 张更早的图表
wefax-older-load-more = 另有 { $older } 张更早的图表——加载更多
wefax-published-carrier-khz-dial-khz = 公布的载波 { $f } kHz → 表盘 USB { $value } kHz
wefax-delete-chart-2 = 删除此图表 { $dir }

## widgets: messages with values (src/app/panels/widgets.rs)

widgets-cq-zone-itu-zone = { $continent } · CQ 区 { $cq_zone } · ITU 区 { $itu_zone }
widgets-new-entity = { $band_label } 上的新实体
widgets-worked-before = 此前已在 { $band_label } 上通联过
widgets-calling-cq-includes-you = 呼叫 CQ { $target }——包括你
widgets-calling-cq-not-aimed-you = 呼叫 CQ { $target }——并非针对你

## wspr: messages with values (src/app/panels/wspr.rs)

wspr-rx-heard-us = { $heard_us } 次接收 · { $heard_us2 } 个收听到了我们
wspr-beacon = { $last_slot_spots } 个信标
wspr-beaconing-duty = 发送信标——{ $wspr_power_dbm }，{ $wspr_tx_percent }% 占空比
wspr-listening-duty = 收听中（{ $wspr_tx_percent }% 占空比）
wspr-beacon-one-two-minute-slot = 每 { $pct } 个两分钟时隙中有一个用于发送信标——占其中的 { $pct2 }%，均匀分布——功率 { $wspr_power_dbm }。周期从哪个时隙开始由你的呼号决定，因此两台运行本程序的电台不会同时开始。
wspr-transmitting-set-general-tab-settings = 以 { $call } 在 { $sent } 发射——请在“设置”的“常规”选项卡中设置。
wspr-utc-ago = { $t }:{ $t2 } UTC——{ $slot_utc } 前

## publicsdr: messages with values (src/app/publicsdr.rs)

publicsdr-antenna =
    { $address }
    { $device }
    天线：{ $antenna }
    { $e3 }
publicsdr-receivers-fetched-ago = { $entries } 台接收机 · { $per_net } · 获取于 { $age } 前
publicsdr-showing-search-narrow = 显示 { $n } 个中的 { $shown } 个——搜索以缩小范围
publicsdr-match = { $n } 个匹配
publicsdr-replace-taking-receiver-points-same =
    用 { $entry } 替换 { $this }？

    { $this2 } is on { $iface }. Taking this receiver points the same tab at { $address } instead, renames it “{ $entry2 }”, and holds the dial to the { $range_label } the receiver publishes — nothing here transmits.

    The { $iface2 } settings stay where they are, and so do the ranges stated for them: switching the interface back in Settings → Radio brings the radio back as it was.
publicsdr-receptions-will-reported = {" "}接收结果将以 { $value } 为位置报告。
publicsdr-pointing-radio = 正在将此电台指向 { $entry }……{ $site }
publicsdr-opening-another-radio = 正在将 { $entry } 作为另一台电台打开……

## qo100: messages with values (src/app/qo100.rs)

qo100-khz-3 = +{ $park_lo_hz } … +{ $park_hi_hz } kHz
qo100-before-switching-tune-so-beacon = 开启之前：请调谐，使信标的两个波瓣位于条带中的 +{ $park_lo_hz } 与 +{ $park_hi_hz } kHz 之间（阴影通道）。
qo100-receiver-currently-listening-around-mhz = 接收机目前收听的是 { $mhz } MHz 附近——离 10489.750 MHz 信标很远，因此这里没有可绘制的内容。无论如何，解码器都会继续搜索。
qo100-receiver-currently-covers-only-khz = 接收机在此处仅覆盖 ±{ $khz } kHz——条带的其余部分保持空白
qo100-null-db-sym-snr-db = 零点 { $est_null_depth_db } dB 对称 { $est_symmetry } 信噪比 { $est_snr_db } dB
qo100-hz-s-hz-s-de = { $est_drift_hz_s } Hz/s { $est_drift_accel_hz_s2 } Hz/s²（解码前已去旋）
qo100-hz-s-de-rotated-before = { $est_drift_hz_s } Hz/s（解码前已去旋）
qo100-found-empty = 找到 { $est_updates } 次，空 { $est_misses } 次
qo100-last-s-ago = {"   "}上次 { $auto_last_hz }（{ $auto_last_unix } 秒前）
qo100-over-correction = 累计 { $auto_total_hz }，共 { $auto_applies } 次校正{ $last }
qo100-sync-32-err = 同步 { $sync_bit_errors } / 32 个错误 ×{ $sync_matches }
qo100-frame-buffer = 帧缓冲 { $frame_fill }%
qo100-blocks-tried-decoded = 已尝试 { $blocks_tried } 个数据块，已解码 { $blocks_locked } 个
qo100-last-applied-s-ago-hz = 上次应用于 { $ago } 秒前：{ $old } → { $new } Hz（{ $old2 } Hz）
qo100-locked-block-tried-locked = 已锁定——已尝试 { $blocks_tried } 个数据块，锁定 { $blocks_locked } 个
qo100-searching-block-tried-none-locked = 搜索中——已尝试 { $blocks_tried } 个数据块，尚未锁定任何一个

## rds: messages with values (src/app/rds.rs)

rds-auto = 自动（{ $standard }）
rds-type = 类型 { $class }
rds-groups = · { $groups } 个组
rds-blocks-error = · { $ber }% 的数据块出错
rds-clean-corrected-lost = { $blocks_ok } 个正确 · { $blocks_corrected } 个已纠正 · { $blocks_bad } 个丢失

## sat: messages with values (src/app/sat.rs)

sat-utc-local = { $unix } UTC（本地 { $offset }）
sat-aos = AOS { $rise_unix } { $rise_unix2 }
sat-los = LOS { $set_unix } { $set_unix2 }
sat-above-horizon = { $rise_unix } 在地平线以上
sat-min = { $value } 分钟后
sat-h-min = { $value } 小时 { $value2 } 分钟后
sat-searching = 搜索中…… { $next } / { $entries }
sat-locked-2 = ● 已锁定 — { $t }
sat-pass-until-max = 过境至 { $set_unix } · 最大仰角 { $max_el }°
sat-next-pass-rises-max = 下一次过境 { $rise_unix } { $rise_unix2 }（{ $now }）· 升起于方位 { $rise_az }° { $rise_az2 } · 最大仰角 { $max_el }°
sat-rigctld-client-connected-satellite-software = ⚠ 已连接 { $clients } 个 rigctld 客户端——卫星软件若在那里操控表盘，会对多普勒进行两次校正
sat-rotator-antenna-az-el = 旋转器：天线位于方位 { $az }°、仰角 { $el }°
sat-rotator-2 = 旋转器：{ $error }
sat-up-now-2 = ● 当前在空，仰角 { $el }°
sat-inv = { $mode } · 反向
sat-norad-2 = NORAD { $norad_id }

## scanner: messages with values (src/app/scanner.rs)

scanner-holding-mhz = 停留在 { $here } MHz
scanner-scanning-mhz = 扫描中 · { $here } MHz

## spectrum: messages with values (src/app/spectrum.rs)

spectrum-renderer-will-not-hold-texture = 此渲染器无法保存宽度超过 { $max_texture_dim } 像素的纹理。

## speech: messages with values (src/app/speech.rs)

speech-voice-2 = 语音：{ $voice }
speech-speech-unavailable = 语音不可用：{ $error }

## spots: messages with values (src/app/spots.rs)

spots-transmissions = { $stations } 次发射

## winlink: messages with values (src/app/winlink.rs)

winlink-last-session-failed = 上次会话失败——{ $err }
winlink-last-session-received-sent = 上次会话：接收 { $last_received } 条，发送 { $last_sent } 条
winlink-empty = { $to_lowercase } 为空
winlink-showing = 显示 { $total } 条中的 { $entries } 条
winlink-fetching = 正在获取 { $mid }……
winlink-from-2 = 来自 { $msg } { $date }
winlink-attachment-bytes = 附件：{ $att }（{ $data } 字节）

## aprsmap: messages with values (src/aprs_map.rs)

aprsmap-object =
    {""}
    来自 { $reported_by } 的对象
aprsmap-heard-ago =
    {""}
    { $age } 前收听到

## download: messages with values (src/download.rs)

download-reading = 正在读取 { $path }：{ $error }

## help: messages with values (src/help.rs)

help-missing-image = {"["}缺少图像：{ $path }]

## hfdlmap: messages with values (src/hfdl_map.rs)

hfdlmap-mhz =
    {""}
    { $e3 } MHz
hfdlmap-fix =
    {""}
    { $fixes } 次修正
hfdlmap-heard-ago =
    {""}
    { $last_at } 前收听到

## multi: messages with values (src/multi.rs)

multi-radio = 电台 { $n }
multi-could-not-add-radio = 无法添加电台：{ $error }
multi-was-closed-station = { $value } 已在电台站点上关闭。
multi-could-not-open = 无法打开 { $peer }：{ $error }
multi-has-tab-own-now = { $value } 现在拥有其自己的选项卡。

## remote: messages with values (src/remote.rs)

remote-protocol-error = 协议错误：{ $error }

## solarnet: messages with values (src/solar3d/net.rs)

solarnet-bad-message = 错误的消息：{ $error }
solarnet-solar-protocol-mismatch-server-client = 太阳协议不匹配：服务器 { $proto }，客户端 { $value }

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
smeter-received-signal-strength-s9-db = 接收信号强度：S9 + { $over } dB（通带内 { $s_dbm } dBm）。
smeter-received-signal-strength-s-dbm = 接收信号强度：S{ $value }（通带内 { $s_dbm } dBm）。
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

decodes-you-transmit-s-everyone-else = 你的发射比其他所有人 { $off } 秒 { $after }——这是呼叫常常得不到应答的常见原因。请同步你计算机的时钟。

## publicsdr: nested messages

publicsdr-noise-floor-score = 底噪得分 { $snr }
publicsdr-up-khz-i-q = I/Q 最高 { $khz } kHz

## txaudio: more messages

txaudio-gr-gate-de-ess-comp = 增益衰减 噪声门 { $gate_gr_db } 去齿音 { $deess_gr_db } 压缩 { $comp_gr_db } 限幅 { $limiter_gr_db }

## amp: more messages

amp-h-step = ~{ $l_step } µH · 步进 { $l_step2 }
amp-pf-step = ~{ $c_step } pF · 步进 { $c_step2 }

## sstv: more messages

sstv-bit-octets-chunks-first-pass = { $from } · { $filename } · { $width }×{ $height } { $bits_per_pixel } 位 · { $media_type } / { $content_encoding } · { $encoded_size } 字节，分为 { $chunk_count } 个数据块（首轮 { $chunks_first_pass }）· 会话 { $session }
sstv-baud-cpfsk-hz-khz-occupied = { $value }——{ $symbol_rate } 波特 CPFSK，±{ $deviation_hz } Hz，占用带宽 { $bandwidth_hz } kHz

## Clock, and a few short labels with values.

time-utc-and-local = { $utc } UTC · 本地 { $local }
sat-in-seconds = { $s } 秒后
winlink-to-list = 收件人 { $to }
winlink-cc-list = 抄送 { $cc }
spots-n-of-total = { $n } / { $total }
wefax-n-of-total = { $n } / { $total }
ais-range-bearing = { $km } km，方位 { $bearing }°

## More values from sdroxide-types, as pickers name them (src/i18n/labels.rs).
rtl-sdr-hf-mode-off = 关闭（仅调谐器，24 MHz 以上）
rtl-sdr-hf-mode-auto = 自动
rtl-sdr-hf-mode-direct-q = 直接采样（Q 支路）
rtl-sdr-agc-manual = 手动（无 AGC）
rtl-sdr-agc-tuner = 调谐器 AGC
rtl-sdr-agc-rtl = RTL 数字 AGC
rtl-sdr-agc-both = 调谐器 + RTL AGC
mode-control-cat = CAT
mode-control-radio = 由电台控制
line-state-none = 无
line-state-high = 高
line-state-low = 低
elad-tx-input-usb-audio = USB 音频
elad-tx-input-mic = 话筒
elad-tx-input-auto = 自动
elad-tx-input-leave = 保持电台上的设置
tr-usdx-audio-one-cable = 单根线缆（音频在 CAT 数据流中）
tr-usdx-audio-sound-card = USB 声卡（3.5 mm 插孔）
spy-server-format-uint8 = 8 位（最低带宽）
spy-server-format-int16 = 16 位
spy-server-format-float32 = 32 位浮点
sound-format-iq = IQ（立体声）
sound-format-demod-audio = 解调音频
sdr-play-duo-tuner-tuner1 = 调谐器 1（50 Ohm / Hi-Z）
sdr-play-duo-tuner-tuner2 = 调谐器 2（50 Ohm）
sdr-play-duo-role-diversity = 第二根天线（分集 / 抑制干扰）
sdr-play-duo-role-second-radio = 第二台电台，使用其自己的频率
sdr-play-agc-off = 关闭（手动中频增益）
sdr-play-agc-hz5 = 5 Hz
sdr-play-agc-hz50 = 50 Hz
sdr-play-agc-hz100 = 100 Hz
pluto-ptt-off = 关闭
pluto-ptt-gpo01 = GPO0 = 接收，GPO1 = 发射
pluto-ptt-gpo23 = GPO2 = 接收，GPO3 = 发射
pluto-duplex-fdd = FDD（同时双向）
pluto-duplex-tdd = TDD（一次一个方向）
pluto-agc-manual = 手动
pluto-agc-slow-attack = 慢速起振
pluto-agc-fast-attack = 快速起振
pluto-agc-hybrid = 混合
parity-none = 无
parity-even = 偶
parity-odd = 奇
panadapter-tap-antenna = 天线（与电台共用）
panadapter-tap-if-output = 电台的中频输出
panadapter-audio-attached = 所连接的接收机
panadapter-audio-transceiver = 收发信机
kenwood-send-ts2000 = TS-2000 风格（TX;）
kenwood-send-ts590 = TS-590 风格（TX1;）
digi-mode-usb = USB
digi-mode-data = DIGI
digi-mode-radio = 由电台控制
cw-keying-cat = 电台键控器（CAT）
cw-keying-audio = 声卡（MCW）
alert-sound-ding = 叮声
alert-sound-two-tone = 双音
alert-sound-triplet = 三连音
alert-sound-warble = 颤音
alert-sound-digital = 数字
alert-reply-tone = 音调
alert-reply-voice = 语音
alert-reply-both = 音调 + 语音
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
icom-model-other = 其他（手动设置地址）
backend-auto = 自动检测（SoapySDR / CAT）
backend-soapy = SoapySDR
backend-cat = CAT / 音频
backend-hpsdr = HPSDR（网络）
backend-tci = TCI（网络）
backend-icom-net = Icom LAN（网络）
backend-smart-sdr = SmartSDR / FlexRadio（网络）
backend-pluto = PlutoSDR（网络）
backend-rtl-sdr = RTL-SDR（USB）
backend-rtl-tcp = 通过 rtl_tcp 的 RTL-SDR（网络）
backend-spy-server = SpyServer（网络）
backend-spy-server-vfo = SpyServer VFO+FFT，低带宽（网络）
backend-kiwi-sdr = KiwiSDR / Web-888（网络）
backend-rx888 = RX-888（USB）
backend-airspy-hf = Airspy HF+（USB）
backend-airspy = Airspy R2 / Mini（USB）
backend-hack-rf = HackRF One / Pro（USB）
backend-sdr-play = SDRplay RSP（USB）
backend-elad = ELAD FDM-DUO / FDM-S（USB）
backend-lime = LimeSDR + LimeRFE（LimeSuite）
backend-hydra-sdr = HydraSDR RFOne（USB）
backend-fobos = RigExpert Fobos SDR（USB）
backend-usb-audio = USB 音频电台（声卡）
backend-none = 未配置
cat-family-xiegu = Xiegu
cat-family-icom = Icom
cat-family-yaesu = Yaesu
cat-family-kenwood = Kenwood
cat-family-elecraft = Elecraft
cat-family-elad = ELAD
cat-family-qrp-labs = QRP Labs
cat-family-rs-hfiq = RS-HFIQ
cat-family-tr-usdx = (tr)uSDX
cat-family-rigctld = Hamlib rigctld（网络）
cat-family-flrig = flrig（网络）
publicsdr-its-interface = 其接口

## Values from sdroxide-types drawn on chips, menus and lists (src/i18n/labels.rs).
agc-mode-off = 关
agc-mode-slow = 慢
agc-mode-med = 中
agc-mode-fast = 快
airspy-gain-linearity = 线性（最佳强信号处理）
airspy-gain-sensitivity = 灵敏度（最佳弱信号）
ais-kind-class-a = A 类
ais-kind-class-b = B 类
ais-kind-base-station = 基站
ais-kind-sar-aircraft = 搜救飞机
ais-kind-aid-to-navigation = 助航设备
ais-kind-sart = SART / MOB / EPIRB
ais-kind-craft = 关联船只
alert-event-called = 有电台在呼叫我
alert-event-cq = 我能应答的 CQ
alert-event-new-dxcc = 新的 DXCC 实体
alert-event-new-dxcc-band = 此频段上的新实体
alert-event-new-grid = 新的网格方格
audio-routing-both = 左 + 右
audio-routing-left = 仅左
audio-routing-right = 仅右
button-mode-momentary = 按住
button-mode-toggle = 切换
contest-mode-none = 无
contest-mode-eu-vhf = EU VHF 比赛
converter-tx-off = 转换时关闭
converter-tx-transverter = 通过同一变频器
converter-tx-own = 独立偏移
cw-engine-neural = 神经网络
cw-engine-timing = 时序
decode-sort-none = 无
decode-sort-signal = 信噪比
decode-sort-distance = 距离
decode-sort-country = 国家
diversity-mode-cancel = 消除——使噪声源置零
diversity-mode-combine = 合并——分集接收
dxped-mode-normal = 普通
dxped-mode-hound = Hound
dxped-mode-fox = Fox
fail-safe-refuse-tx = 拒绝发射
fail-safe-warn-only = 仍然发射，但发出警告
ftp-security-plain = FTP（无加密）
ftp-security-explicit-tls = FTPS 显式（AUTH TLS）
ftp-security-implicit-tls = FTPS 隐式
hpsdr-filter-board-none = 无——输出保持关闭
hpsdr-filter-board-n2adr = N2ADR 滤波器板
hpsdr-filter-board-alex = Alex / Hermes 频段代码（Zeus、HiQSDR、Quisk）
hpsdr-filter-board-custom = 自定义——我在下方自己的表格
hpsdr-io-rx-input-radio = 电台自身的输入
hpsdr-io-rx-input-io-board = IO 板 J9
hpsdr-io-rx-input-io-board-pure-signal = IO 板 J9，发射时使用 PureSignal
hydra-sdr-gain-linearity = 线性（最佳强信号处理）
hydra-sdr-gain-sensitivity = 灵敏度（最佳弱信号）
icom-rx-source-af = AF——电台解调
icom-rx-source-if12k = 12 kHz 中频——sdroxide 解调
icom-scope-span-radio = 保持电台上的设置
icom-scope-span-khz2-5 = 5 kHz（±2.5k）
icom-scope-span-khz5 = 10 kHz（±5k）
icom-scope-span-khz10 = 20 kHz（±10k）
icom-scope-span-khz25 = 50 kHz（±25k）
icom-scope-span-khz50 = 100 kHz（±50k）
icom-scope-span-khz100 = 200 kHz（±100k）
icom-scope-span-khz250 = 500 kHz（±250k）
icom-scope-span-khz500 = 1 MHz（±500k）
ism-family-weather = 气象
ism-family-meter = 仪表
ism-family-home-auto = 家居
ism-family-lo-ra = LORA
ism-family-unidentified = 未知
ism-protocol-la-crosse-it = LaCrosse IT+
ism-protocol-fine-offset = Fine Offset
ism-protocol-bresser = Bresser
ism-protocol-z-wave = Z-Wave
ism-protocol-homematic = Homematic
ism-protocol-rtl433 = rtl_433
ism-protocol-unidentified = 未识别
js8-speed-normal = 普通
js8-speed-fast = 快速
js8-speed-turbo = Turbo
js8-speed-slow = 慢速
lime-aux-role-off = 未使用
lime-aux-role-diversity = 第二根天线（分集 / 抑制干扰）
lime-aux-role-pure-signal = 发射反馈（PureSignal 预失真）
lookup-provider-none = 关闭
lookup-provider-qrz = QRZ.com
lookup-provider-ham-qth = HamQTH
mail-folder-inbox = 收件箱
mail-folder-outbox = 发件箱
mail-folder-sent = 已发送
mail-folder-archive = 存档
memory-sort-name = 名称
memory-sort-freq = 频率
memory-sort-band = 频段
memory-sort-stored = 存储顺序
mouse-button-middle = 中键
mouse-button-extra1 = 后退（额外 1）
mouse-button-extra2 = 前进（额外 2）
nr-level-off = 关
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
orbit-rings-none = 无
orbit-rings-curated = 精选
orbit-rings-all = 全部
qso-step-idle = 空闲
qso-step-wait-cq = 等待 CQ
qso-step-calling-cq = 呼叫 CQ
qso-step-tx-grid = 发送网格
qso-step-tx-report = 发送报告
qso-step-tx-r-report = 发送 R+报告
qso-step-tx-rr73 = 发送 RR73
qso-step-tx73 = 发送 73
qso-step-confirming = 确认中
rds-standard-auto = 自动
rds-standard-rds = RDS
rds-standard-rbds = RBDS
region-r1 = 第 1 区——欧洲、非洲、中东、北亚
region-r2 = 第 2 区——美洲
region-r3 = 第 3 区——南亚和东亚、大洋洲、太平洋
relative-mode-absolute = 绝对（推子）
relative-mode-sign-magnitude = 相对：符号/幅值
relative-mode-twos-complement = 相对：二进制补码
relative-mode-binary-offset = 相对：以 64 为中心
relay-link-off = 无
relay-link-serial = USB 继电器板（串口）
relay-link-serial-lines = 串口 RTS/DTR 线路
relay-link-hid = USB 继电器板（HID）
relay-link-cm108 = CM108/CM119 声卡 GPIO
relay-link-gpio = Linux GPIO 线路
relay-link-command = 外部命令
relay-role-unused = 未使用
relay-role-sdr-antenna = 接地 SDR 天线
relay-role-amplifier = 按下功放 / 收发继电器
relay-role-aux = 辅助
relay-role-band-decoder = 频段解码器（滤波器 / 变频器）
rfe-channel-wb1000 = 宽带 1 – 1000 MHz
rfe-channel-wb4000 = 宽带 1 – 4 GHz
rfe-channel-ham0030 = HF，最高 30 MHz
rfe-channel-ham0070 = 6 m / 4 m（50 – 70 MHz）
rfe-channel-ham0145 = 2 m（140 – 150 MHz）
rfe-channel-ham0220 = 1.25 m（220 – 225 MHz）
rfe-channel-ham0435 = 70 cm（400 – 450 MHz）
rfe-channel-ham0920 = 33 cm（902 – 928 MHz）
rfe-channel-ham1280 = 23 cm（1220 – 1420 MHz）
rfe-channel-ham2400 = 13 cm（2.3 – 2.5 GHz）
rfe-channel-ham3500 = 9 cm（3.3 – 3.7 GHz）
rfe-channel-cell-band01 = 蜂窝频段 1
rfe-channel-cell-band02 = 蜂窝频段 2
rfe-channel-cell-band03 = 蜂窝频段 3
rfe-channel-cell-band07 = 蜂窝频段 7
rfe-channel-cell-band38 = 蜂窝频段 38
rfe-channel-not-selected = 未选择
rfe-link-off = 未连接
rfe-link-serial = 其自己的 USB 线缆（串口）
rfe-link-board = 通过 LimeSDR（GPIO / I²C）
rfe-mode-control-auto = 自动（跟随接线）
rfe-mode-control-rx = 始终接收
rfe-mode-control-tx = 始终发射
rfe-mode-control-tx-rx = 始终两者（仅限分离式连接器）
rifp-encoding-auto = 自动
rifp-encoding-png = PNG
rifp-encoding-jpeg = JPEG
rifp-encoding-raw = 原始
rifp-encoding-rle8 = RLE8
rifp-encoding-zlib = Zlib
rifp-encoding-group4 = G4
rifp-encoding-group3 = G3
scan-kind-memories = 记忆
scan-kind-range = 范围
scan-resume-carrier = 载波
scan-resume-timed = 定时
scan-resume-manual = 手动
sense-line-off = 未接线
sense-line-cts = CTS
sense-line-dsr = DSR
sense-line-dcd = DCD（载波检测）
shift-simplex = 单工
shift-minus = −
shift-plus = +
speed-slow = 慢
speed-medium = 中
speed-fast = 快
speed-faster = 更快
speed-fastest = 最快
tls-verify-system = 系统信任库
tls-verify-pinned = 固定指纹
tls-verify-off = 不验证
tone-mode-off = 关
tone-mode-ctcss = CTCSS
tone-mode-dcs = DCS
upload-naming-sequence = image1 … imageN（循环）
upload-naming-original = 原始文件名
wheel-action-zoom = 缩放
wheel-action-tune = 调谐
wheel-action-none = 无

settings-alerts-say = 朗读

settings-alerts-say-hint = 听取此提醒的语音播报

settings-alerts-say-need-voice = 请选择“语音”或“提示音 + 语音”以收听

settings-alerts-say-voice-off = 请在 设置 → 界面 → 语音播报 中开启语音

settings-alerts-speaks = 播报：{ $text }

settings-picup-keep = 保留

jtty-starting-receiver = 正在启动 JTTY 接收机…

jtty-receive-audio-level = 接收音频电平

jtty-sending = {" "}发送中{" "}

jtty-sent-once-note = 按下 TX 后消息只发送一次。JTTY 是异步的，无需等待周期，也不会重复发送。

signalid-title = 信号识别

signalid-tuned-now = 当前调谐

signalid-passband = 通带 { $bw }

signalid-band = { $band } 波段

signalid-find = 查找

signalid-find-hint = FT8、流星、天气、飞机…

signalid-candidates = 此频率有 { $n } 个候选项

signalid-matches = “{ $query }”有 { $n } 个匹配

signalid-nothing-fits = 目录中没有与当前频率、模式和通带相符的条目。

signalid-no-match = 目录中没有匹配的条目。

signalid-open-url = 打开 { $url }

windows-chip-del = 删除

windows-chip-edt = 编辑

windows-chip-ren = 改名

windows-chip-rec = 录制

windows-name-hint = 名称

windows-waterfall-marks = 瀑布图

windows-stacked = 堆叠

jtty-listening-note = 监听中。JTTY 信号随时可能出现，收到后消息会稍后显示在这里。

extras-title = 扩展

extras-receivers = 接收机

extras-dab-hint = DAB / DAB+ 数字广播，播放器在瀑布图下方打开

extras-aircraft-watch-hint = 飞机，关注列表中的飞机正在空中，请打开关注视图

extras-aircraft-hint = 飞机，电台所收到内容的地图：飞机和无线电探空仪，可在独立窗口中显示

extras-radio-hint = 经典收音机，用于收听广播的经典面板：波段、记忆、电台名称和 RadioText。在语音模式下停靠在瀑布图下方；设置 → 可分离 可将其放入独立窗口

extras-antenna = 天线

extras-antenna-hint = 天线（实验/测试版），常见线状、垂直和八木天线的截取长度、馈点阻抗和方向图

recjobs-title = 录音

recjobs-finished = 已完成“{ $name }”

recjobs-skipped = 已跳过“{ $name }”，正在录音

recjobs-recording-name = 正在录制“{ $name }”

recjobs-new-job = + 新建任务

recjobs-recording = ● 录制中

recjobs-none = 没有定时录音，请按 + 新建任务 添加。

recjobs-done = 完成

recjobs-edit = 编辑

recjobs-del = 删除

recjobs-station = 电台

recjobs-station-hint = BBC World Service

recjobs-kind = 类型

recjobs-start-utc = 开始 (UTC)

recjobs-for = 时长

recjobs-save = 保存

recjobs-kind-audio = 音频

recjobs-kind-iq = I/Q

recjobs-kind-both = 两者

panels-sensitivity = 灵敏度

navtex-hunting = 搜索中

navtex-hunting-hint =
    尚未读取到任何内容。

    几乎总是调谐的问题。请不要在频率框中输入标称的数字 (518)：NAVTEX 按信道标称，本程序在其略下方收听。请改为点击上方的 518 按钮，它会为您设置正确的拨盘频率 (516.300)。

    如果明显有强信号但文字仍是乱码，请按 REV：信号是反向到达的。

    还请记住，NAVTEX 是按时间表发送的，而不是对话：海岸电台每四小时发送十分钟，所以大部分时间安静是正常的。您必须在电台的发送时刻收听。而且 518 kHz 只在夜间才活跃，白天波段安静并不意味着解码器坏了。

navtex-sensitivity-hint =
    灵敏度：解码器锁定并保持信号的难易程度。
navtex-auto-hint = 恢复为参考阈值 (0)。启用时高亮显示。

navtex-tune-dial-usb-below-channel = 在此调谐拨盘（USB，比信道低 1.710 kHz）。

settings-audio-your-audio = 您的音频

settings-audio-your-audio-why = 您收听用的扬声器和发送语音用的麦克风。它们属于这台计算机，而不属于任何电台，因此无论使用哪台电台都是一样的。

settings-audio-browser-note = 此处的声音设备在浏览器中选择，而不是在本窗口中：页面会向浏览器请求扬声器和麦克风。本页下方您电台自己的声卡设置仍然有效。

settings-audio-speakers = 扬声器

settings-audio-channels-hint = 接收音频送到哪个扬声器声道。选左或右会使另一侧静音，以便只在一侧收听。

settings-audio-microphone-note = 当电台由声音 (MCW) 或 VOX 键控时，麦克风负责送出您的语音。如果您的电台改由 CAT 键控，则发射时完全不使用麦克风，此行不起任何作用。

settings-audio-stream-out = 输出音频流

settings-audio-stream-out-why = 通过网络将本电台接收的内容发送给其他程序（解码器、录音机、另一台计算机），无需经过声卡。

settings-audio-send-to-hint =
    `host:port`。单声道，16 位，采用接收采样率，每个音频块一个数据报，其内容是**录音抽头**的音频：AF 音量、静音和发射都不会影响它，因为外部解码器需要的是电台音频，而不是您的耳机音量。

    请填数字地址，不能填主机名。留空表示关闭。

settings-audio-send-to-note = `127.0.0.1:7355` 会将其发送到本机上的另一个程序；本机自己的地址则通过网络发送。

settings-audio-radio-audio = 本电台的音频

settings-audio-radio-audio-why = 本电台与本计算机之间的声卡。它属于电台而不属于这台机器：两台电台可以共用一台计算机，却仍需要不同的声卡，所以这些设置跟随您打开设置时所用的电台。

settings-audio-no-cards-yet = 本机尚未报告其声卡。仍可在下方输入名称；设备被查询后会出现列表。

settings-audio-rescan = 重新扫描声卡

settings-audio-rescan-hint =
    再次向本机询问它有哪些声卡。不会打开任何设备，因此在接收时按下是安全的。

    插入编解码器后使用。它不会重新打开电台（那是电台选项卡上的 应用 / 重新连接），也无法修复被声音服务器改名的声卡，这类情况会在电台上报告。

settings-audio-from-radio = 来自电台

settings-audio-to-radio = 送往电台

settings-audio-radio-note = { "*" }*来自电台**是您的电台送出的音频：USB 声音输出，或接收机的耳机插孔接入声卡的线路输入。**送往电台**是电台用来接收您语音的音频，也是程序通过声音键控电台的方式：留空则使用电台自带的麦克风（如果有的话）。

settings-audio-recording = 录音

settings-audio-recording-why = 音频录音、定时录音和原始 I/Q 捕获的写入位置。

enigma-need-more = 需要更多密文才能进行解算

enigma-title = ENIGMA

enigma-no-cables = 无连线

enigma-thin = 稀少

enigma-rotor = 转子

enigma-window-hint = 初始位置，点击使转轮前进

enigma-input-hint = 输入，或使用上方的按键

enigma-clear = 清除

enigma-copy = 复制

enigma-solve-note = 解算：粘贴密文。若为接线的文本，请给出您预期出现在消息中的词作为猜测词；没有的话，解算器只处理未接线的机器。

enigma-crib = 猜测词

enigma-solve = 解算

enigma-flaw = 一个字母绝不会加密成它自己，这是反射器造成的。这是真实机器的缺陷，解算器正是据此剪枝。

settings-audio-takes-over-speakers = { "*" }*这些扬声器是声音服务器设备，而不是声卡。**使用它们也会使其成为本计算机上*其他一切*程序的播放设备，其他程序也会通过它们播放。PipeWire 在这里无法让某个程序自选播放目标，所以这是不可避免的。要更改：请选择一块声卡，或将本机默认设备改回（`wpctl set-default <name>`）并重新扫描。

settings-audio-takes-over-microphone = { "*" }*这个麦克风是声音服务器设备，而不是声卡。**使用它也会使其成为本计算机上*其他一切*程序的录音设备，其他程序也会从它录音。PipeWire 在这里无法让某个程序自选采集源，所以这是不可避免的。要更改：请选择一块声卡，或将本机默认设备改回（`wpctl set-default <name>`）并重新扫描。

settings-audio-currently-chosen = 当前选择：{ $device }

knowncalls-no-table = 此模式不保存呼号表。它是对呼号做哈希的 FT8/FT4 解码器，其他模式在这里没有可显示的内容。

knowncalls-not-asked = 尚未查询，请按 刷新。

knowncalls-nobody = 还没有电台。一旦听到某电台完整报出呼号（CQ、双方呼号都明文的报告，或简短的 11 m 呼叫），就会将其加入。

knowncalls-title = 已知电台

knowncalls-intro = 本接收机上哈希形式 <...> 当前能解析出的每个呼号。哈希无法反推，只有听到电台完整报出呼号后它才会出现在这里，简短的 11 m 呼叫正是为此。

knowncalls-find-hint = 呼号、国家、前缀…

knowncalls-refresh = 刷新

knowncalls-capped = 最新 { $shown } 条，共 { $total } 条，按 刷新 重新查询，列表只是快照

knowncalls-known = 已知 { $shown } 个

knowncalls-matches =
    { $n } { $n ->
        [one] 个匹配
       *[other] 个匹配
    }

knowncalls-no-match = 没有匹配的已知电台。

knowncalls-no-country = 无国家，此表不认识该呼号

classic-controls = 控制

classic-fold-the-column-away-to-a-rail-on = 将此栏收起到左侧的导轨。读数和电平表仍留在栏上，恢复此栏的控件在导轨上。

classic-click-to-open-or-close-this = 点击以展开或收起此区段。将其拖到另一区段上可移动，顺序会被记住。

classic-bring-the-control-column-back = 恢复控制栏。

classic-no-radio = 无电台

classic-rate = 速率

classic-dial = 拨盘

classic-receive = 接收

classic-listening-screen = 收听屏幕

classic-radio-settings = 电台设置

classic-the-backend-the-sample-rate-the = 后端、采样率、天线、变频器和本电台自己的校准，位于 设置 → 电台，一直都在那里。此栏显示它们的当前设置，而不是控件的第二份副本。

classic-counted-in-steps-not-decibels-what = 以步进计数，而非分贝：一步的价值由接收机自己决定，在 SDRplay 上还取决于波段。向右仍然是增益更大。

classic-the-plutosdr-s-own-agc-is-setting = PlutoSDR 自带的 AGC 正在设置此增益，因此移动滑块不起作用。请将 设置 → 电台 → AGC 设为手动，以便在此控制。

classic-squelch = 静噪

classic-gate-the-audio-below-this-power-in = 对接收通带内低于此功率（单位 dBFS）的音频进行门限。向左为开启。请设在噪声之上、信号之下。

classic-this-radio-has-not-reported-what = 本电台尚未报告其功能，其链路已关闭，或尚未应答。

classic-open-the-link-in-the-radio-strip = 请在电台栏中打开链路，发射控件就会出现在这里。

classic-this-device-receives-only-it-has = 此设备仅能接收：没有可供本程序键控的发射机。

classic-transmit-is-offered-by-a-rig-with = 具有控制或音频通路的电台才提供发射，见 设置 → 电台。

classic-transmit-is-off-for-this-radio-s = 本电台的屏幕已关闭发射。

classic-settings-radio-show-transmit = 设置 → 电台 → “显示发射控件”可将其恢复。

classic-key = 键控

classic-controlled-envelope-ssb-how-much = 受控包络 SSB：应用多少语音处理。越高声音越响、越平，代价是两侧会有一些溅射。

classic-holding-the-squelch-open-click-to = 正保持静噪开启。点击以恢复原来的门限。

classic-click-to-hold-the-squelch-open-to = 点击以保持静噪开启（听听是否有信号），再次点击则完全恢复原状。

panels-received = 已接收

panels-text = 文本

panels-hex = 十六进制

panels-nothing-received-yet = 尚未收到任何内容。

panels-payload = 载荷

panels-blocks = 数据块

panels-sequence = 序列

uvpacket-starting-receiver = 正在启动 UVPacket 接收机…

uvpacket-frames-count = { $n } 帧

uvpacket-frames-count-hint = 接收机启动以来已解码的帧数。

uvpacket-intro = 用于私人 VHF/UHF 小组的分组模式。将拨盘调到小组的信道；子模式由每一帧自动检测。

uvpacket-nothing-yet = 暂无内容。UVPacket 在两次发射之间是安静的，请停在某个信道上等待突发信号。

uvpacket-app-seq = 应用 { $app } · 序号 { $seq }

uvpacket-click-read-frame = 点击以查看整个帧。

uvpacket-bps-net = 净速率 { $bps } bps

uvpacket-sub-mode = 子模式

uvpacket-app-type = 应用类型

uvpacket-payload-bytes = { $n } 字节

dsc-starting-receiver = 正在启动 DSC 接收机…

dsc-seq-tone = { $n } 序列 · 音调 { $pct }%

dsc-seq-tone-hint = 到达结束标记的序列数，以及两个音调的分离清晰程度。

dsc-nothing-yet = 暂无内容。DSC 在船舶之间是安静的，请停在某个信道上等待突发信号。

panels-marginal = 边缘

dsc-marginal-hint = 有字符未通过 BCH 校验，所显示的字段可能有误。

dsc-click-read-sequence = 点击以查看整个序列。

panels-marginal-caps = 边缘

dsc-marginal-caps-hint = 至少有一个字符未通过 BCH 校验。仍显示该序列，因为不完整到达的遇险报警依然值得一读。

dsc-category = 类别

dsc-nature = 性质

dsc-caller-mmsi = 呼叫方 MMSI

dsc-target-mmsi = 目标 MMSI

dsc-alert-time = 报警时间

dsc-working-ch = 工作信道

dsc-raw-symbols = 原始符号

dsc-none = （无）

settings-general-device-type-name = …或输入名称，留空为系统默认

settings-general-signed-in-station = 已登录的电台

settings-general-sign-out-hint = 清除此设备的登录信息。下次连接时电台需要重新询问；当前连接会保持到您关闭为止。

settings-general-recordings-remote = 录音写在电台所连接的机器上。请在那里设置文件夹。

settings-general-folder = 文件夹

settings-general-choose = 选择…

settings-general-choose-hint = 选择录音写入的文件夹

settings-general-default = 默认

settings-general-default-hint = 恢复默认：您的音乐文件夹，系统没有音乐文件夹时为配置目录

settings-general-recordings-dir-note = 音频录音、定时录音和原始 I/Q 捕获的写入位置。默认：您的音乐文件夹下的 sdroxide 子文件夹；系统没有音乐文件夹时，为配置目录下的 recordings 文件夹。

setup-suffix-cq = { " " }cq

setup-suffix-tries = { " " }次尝试

setup-window = 窗口

setup-south = 南

setup-west = 西

setup-message-templates = 消息模板  { $tokens }

setup-ft8-decoder-effort-hint =
    FT8 解码器为弱信号付出多大努力。

    快速：单遍，无信号减除，最快，解码最少。
    普通：平坦多遍减除，繁忙时隙约 0.4 s，比深度少几个解码。
    深度：WSJT-X 的检查点遍，约 1.2 s，解码最多（默认）。

    只有第二批解码受此控制；普通遍总是最先输出，因此回复不会被它拖延。

setup-auto-cq-repeats-hint = 一次运行在结束前发出多少次 CQ 呼叫。0 表示自身不设上限：发射看门狗和无活动停止仍会限制运行。

setup-auto-cq-then-listen-hint = 这些 CQ 呼叫之后，将自动 CQ 切换为自动 RX 并继续监听，而不是完全停止。关闭时，运行就此停止。

setup-auto-call-retries-hint = 对一个电台呼叫多少次后放弃，并转向队列中的下一个。

setup-auto-session-hint = 无人值守运行最长可持续多久，无论它在处理什么。0 表示自身不设上限。

setup-serial-contests-hint = 序号比赛，如 ARRL RTTY Roundup 和 FT8 上的 CQ WPX：信号报告加序号（或美国州名），不带网格。CQ 以 CQ RU 发出。

setup-tx-level-fm-hint =
    交给自行调制的电台的本模式突发信号有多响。在 FM 上这就是频偏，因为 FM 发射机把音频电平变为频率摆动，且没有 ALC 来限制。1200 波特分组需要约 3 kHz，而语音需要 5 kHz，所以满幅送入按语音设定的数据输入会过偏，听者听起来完全正常，却没有人能解码。请降低到其他电台报告收到您为止，或在电台上设置电平。

    此电平只属于本模式。从未设置过的模式从 FM 的数值开始，边带模式从它们自己的数值开始，因此为 1200 波特设置的频偏不会用到 FT8 上。

setup-tx-level-ssb-hint =
    交给自行调制的电台的本模式信号有多响，例如通过声卡的 CAT 电台、FLEX、通过网络端口的 Icom。在边带上这是进入调制器的驱动：降低到电台 ALC 几乎不动，并在电台上设置功率，因为恒包络模式上 ALC 起作用正是造成溅射的原因。驱动在这里到达电台的功率寄存器，而不是其音频，所以这就是电平。

    此电平只属于本模式：FT8、RTTY、PSK 和 MCW 各自保留自己的值，因为它们对调制器的负载方式不同。发射栏上有同样的滑轨，发射时也可以调节。

setup-repeats-hint-base = 对一个电台的未应答呼叫次数，超过后转向下一个。呼叫 CQ 不受限制。

setup-repeats-hint-duration = { " " }按 { $mode } 的时隙长度（{ $slot } s），约为 { $mins } 分钟。

setup-repeats-hint-disables = { " " }0 表示禁用。

setup-repeats-hint-11m = { " " }在 11 m 上，这也是结束 CQ 运行的条件：发射看门狗不会中断它，因为应答 CQ 的电台预计在约 30 s 内出现，而时钟无法区分新运行和过期运行。

setup-symbol-home-station = 家庭电台

setup-symbol-car = 汽车

setup-symbol-truck = 卡车

setup-symbol-van = 面包车

setup-symbol-motorcycle = 摩托车

setup-symbol-bicycle = 自行车

setup-symbol-person-on-foot = 步行者

setup-symbol-motorhome = 房车

setup-symbol-boat = 船

setup-symbol-yacht = 游艇

setup-symbol-light-aircraft = 轻型飞机

setup-symbol-balloon = 气球

setup-symbol-digipeater = 数字中继

setup-symbol-i-gate = I-gate

setup-symbol-repeater = 中继台

setup-symbol-weather-station = 气象站

setup-symbol-hospital = 医院

setup-symbol-portable-campsite = 便携 / 营地

settings-general-sign-out = 退出登录

settings-general-fallback-password = 正在此浏览器中记住 { $stations } 的密码，因为它没有返回登录 cookie。较旧的电台会这样；这与 cookie 不是一回事。

settings-general-device-hint-speakers = 打开扬声器所用的确切名称。列表中有时请从列表选择；列表中没有时请手动输入，声音服务器可能提供主机 API 未枚举的设备。留空则使用系统默认。

settings-general-device-hint-microphone = 打开麦克风所用的确切名称。列表中有时请从列表选择；列表中没有时请手动输入，声音服务器可能提供主机 API 未枚举的设备。留空则使用系统默认。

settings-general-device-hint-radio-rx = 打开本电台接收音频所用的确切名称。列表中有时请从列表选择；列表中没有时请手动输入，声音服务器可能提供主机 API 未枚举的设备。留空则使用系统默认。

settings-general-device-hint-radio-tx = 打开本电台发射音频所用的确切名称。列表中有时请从列表选择；列表中没有时请手动输入，声音服务器可能提供主机 API 未枚举的设备。留空则使用系统默认。

settings-general-device-hint-other = 打开该设备所用的确切名称。列表中有时请从列表选择；列表中没有时请手动输入，声音服务器可能提供主机 API 未枚举的设备。留空则使用系统默认。

settings-general-cb-world = 全球 / 自由频段，80 个信道

settings-general-cb-cept = CEPT / 欧盟，40 个信道

settings-general-cb-germany = 德国，80 个信道

settings-general-cb-uk = 英国，27/81，40 个信道

settings-general-cb-us = 美国，40 个信道

settings-general-cb-australia = 澳大利亚，40 个信道

settings-general-cb-pmr446 = PMR446，16 个信道 (446 MHz)

settings-general-cb-dpmr446 = dPMR446，32 个信道 (446 MHz)

wefax-receiving-progress = 已接收 { $min } 分钟 · { $lines } 行

wefax-listening = 监听中

wefax-status-hint =
    等待图表。

    WEFAX 是按时间表发送的，而不是对话：电台在固定时间发送图表，所以选定的频率在一天中的大部分时间是安静的，这是正常的。请用 电台 按钮选择一个并核对时间。

    如果有信号但图像只是单一灰度（全灰、全黑或全白），则是调谐或速度问题：请使用 +0 Hz 读数和 IOC/速度控件，并核对 电台 条目与所发送的内容一致。

wefax-centre = 居中

wefax-centre-hint = 将拨盘微调 { $sign }{ $off } Hz，使副载波重新回到 1900 Hz，接收机偏了 { $off } Hz，这会使图表变成白页。

wefax-auto-start-hint = 听到 300 Hz (IOC 576) 或 675 Hz 起始音时开始一幅图表；如果起始音在衰落中或载波下丢失，则在收到几秒相位信号时开始。下一次发射到来时，无论是否先有停止音，都会结束正在进行的图表并为新图表定相。信道上的载波不会触发任何开始。

wefax-fit = 适应

wefax-save-chart-hint = 将屏幕上的图表按已到达的部分保存到本计算机

wefax-waiting-for-chart = 在 USB 下调谐到传真时间表并等待图表开始（起始音或相位信号），或按 开始 从图表中途开始。

wefax-carrier-dial-note = 载波 { $carrier } · 拨盘 { $dial } kHz (USB -1.9k)

wefax-carrier-note-hint = 电台标签给出公布的载波；传真副载波位于其上方 1900 Hz，所以拨盘特意低 1.9 kHz。若将拨盘调到载波本身，信号就会落在通带之外。

contest-title = 比赛

contest-intro = 选择一项比赛并输入您发送的内容。日志适用于任何模式：CW 或 SSB 请输入交换信息；比赛有 FT8 格式时，FT8 一侧会自动填写。

contest-you-send = { "  " }您发送：{ $exchange }

contest-they-send = { "  " }对方发送：{ $exchange }

contest-cb-note = CB / 11 m 活动：一份报告和自由文本交换，请在其中填写活动使用的信道、姓名或地区。

contest-your-exchange = 您的交换信息

contest-start = 开始

contest-score-line = QSO { $qsos } · { $points } 分 · { $mults } 倍增 · 得分 { $total }

contest-rate = 10 分钟 { $r10 } 个 · { $r60 }/小时

contest-stop = 停止

contest-estimate = 得分只是估计，以主办方裁定为准

contest-call = 呼号

contest-dupe = 重复

contest-sent = 已发

contest-rcvd = 已收

contest-sending = 发送 { $rst }

contest-sending-ours = 发送 { $rst } { $ours }

contest-log = 日志

playback-radio-n = { " " }· 电台 { $n }

playback-no-stamp = 名称中没有时间戳，将以当前模式播放

playback-title = 回放：已录音频

playback-find-hint = 模式、MHz、日期、名称…

playback-refresh-hint = 重新读取录音文件夹

playback-asking = 正在向电台询问其录音…

playback-folder = 文件夹：{ $dir }

playback-none = 还没有录音。在有信号时按 录制 → MP3 或 WAV；文件会出现在这里，以其时间、频率和模式命名。

playback-on = 播放中

playback-play = 播放

playback-iq-hint = 这是 I/Q 捕获，不是音频。回放使用录音机写入的音频；I/Q 捕获请用 --file 播放。

playback-not-while-tx = 发射时不可用

playback-play-hint = 从头将此录音播放到解码器中

playback-busy-playing = 正在回放，请先停止再删除

playback-busy-recording = 仍在录制，请先停止 录制 再删除

playback-delete-ask = 删除？

playback-delete-hint-armed = 再按一次将永久删除此文件，无法撤销。

playback-delete-hint = 从文件夹中删除此录音（会再确认一次）

playback-strip-label = 回放

playback-not-live = 非实时，电台已让出

playback-carry-on = 继续播放

playback-pause = 暂停

playback-pause-hint = 暂停，解码器在您继续之前听到的是静音

playback-again = 重来

playback-again-hint = 回到开头，再试一次图像或解码

playback-loop = 循环

playback-loop-hint = 文件结束时从头重新开始

playback-stop = 停止

playback-stop-hint = 结束回放并让电台恢复原状

playback-files = 文件

playback-end-of-file = 文件结束

playback-paused = 已暂停

playback-recorded-on-mode = 录制于 { $mhz } MHz，{ $mode } 模式，拨盘和模式已根据文件名设置。

playback-recorded-on = 录制于 { $mhz } MHz，拨盘和模式已根据文件名设置。

playback-no-dial-mode = 文件名中没有记录拨盘或模式，因此以当前模式播放，请从模式菜单选择解码器。

playback-format-stereo = { $rate } Hz，立体声，播放左（接收）声道。

playback-format-mono = { $rate } Hz，单声道，播放该唯一声道。

playback-stopped = 已停止：{ $fault }

playback-intro = 将录音回放到解码器中，包括 SSTV、CW、RTTY、PSK、FT8 以及其他所有从音频解码的模式。播放期间电台让出，拨盘和模式会调到录音时的设置，停止 会让电台恢复。播放时更改模式可尝试另一个解码器。

packet-callsign-hint = 呼号

packet-via-hint = 经由

aprs-filter-hint = 过滤

aprs-msg-sent = 已发送

aprs-msg-ack = 已确认

aprs-msg-rejected = 被拒绝

aprs-reply-chip = 回复

aprs-to-hint = 收件人

aprs-message-hint = 消息

aprs-raw-chip = 原始

fsq-to-button = 发给

fsq-del-chip = 删除

macros-column-key = 按键

macros-column-button = 按钮

macros-column-sends = 发送内容

macros-label-hint = 标签

macros-add = 添加

macros-note-placeholders = 消息发出时会填入 { $mycall } 和 { $mygrid }。

macros-buttons-note = 每个按钮一次发送其全部文本。只要屏幕上没有其他控件占用键盘，F1-F9 就对应前九个按钮。



widgets-worked-before-any = 已联络过

vdl2-col-sig = 信号

vdl2-col-age = 时长

vdl2-command = 命令

vdl2-response = 响应

vdl2-frame-hint = 帧

vdl2-suffix-messages = { " " }条消息

vdl2-suffix-stations = { " " }个电台

ale-words-count = { $n } 个词

ale-waiting-audio = 等待音频…

ale-no-words-yet = 还没有词。请调谐到 ALE 信道（例如 8992 或 11175 kHz USB）并等待呼叫。

atchat-role-master = 主

atchat-role-backup = 备

atchat-me = 我

dab-sub-channel = { " " }· 子信道 { $n }

dab-starting-receiver = 正在启动 DAB 接收机…

dab-no-ensemble-yet = 尚未识别出复用名称，此信道上有 DAB 发射吗？

dab-window-stats = { $mhz } MHz · { $frames } 帧 · { $fibs } fib

dab-volume-meter-hint = 您听到的电台，左右声道及音量：沿电平条拖动或点击，或使用滚轮。点击后可用方向键步进。在用挂锁锁定的屏幕上也可使用。

rade-searching = 搜索中

rfpaint-swl-note = RF Paint 是仅发射模式，在 SWL 中无内容可显示。

cw-hand-key-arm-hint = 启用键控器并使用 USB 拨键。触点送到引擎，由引擎按发射速度生成点和划，在这里而不是面板中，因此定时不会被取整到帧。启用时发射框被锁定，排队的文本会被丢弃。同一框中的直键也可用：请选择直键类型。

cw-own-keyer-hint =
    本电台使用自己的键控器发送：文本经控制端口发出，由电台决定元素时序，因此手与空中之间没有可供按键驱动的环节，拨键和直键都一样。

    要手动键控，请在 设置 → 电台 中将 CW 键控设为“声卡 (MCW)”。电台随后保持在边带上，键控器自己的侧音作为音频发射，这正是直键所驱动的路径。

cw-no-paddle-found = 未找到拨键，请在 设置 → CW 中选择

cw-msg-hint = 您自己的消息按钮：比赛交换、姓名和 QTH 回复、TNX 73 GL。每个按钮一次发送其全部文本，F1-F9 对应前九个。它们随电台配置保存，因此远程客户端也有。

wspr-clear-rx = 清除 RX

wspr-rx-count = { $n } 个 rx

wspr-sort-dist = 距离

wspr-sort-country = 国家

wspr-tx-enable = 启用 TX

wspr-stop-tx = 停止 TX

wspr-listening-as = 以 { $call } 在 { $sent } 监听，在 设置 的常规选项卡中设置。

wspr-set-callsign-grid-label-spots = 请在 设置 的常规选项卡中设置您的呼号和网格，以便标注您的 spot。

schedule-source-mine = 我的

schedule-all-of-them = 全部

schedule-title = 节目

schedule-at = 时间

schedule-now-chip = 现在

schedule-set-time = 设置时间

schedule-find-hint = BBC、6185、15.4 MHz…

schedule-target = 目标

schedule-any = 任意

schedule-source = 来源

schedule-all-hint = 同一存储中所有已下载的时间表。被两个时间表同时列出的电台会出现两次，这正是本窗口要显示的细节，而全景频谱只显示一个名称。

schedule-favs = ★ 收藏

schedule-favs-hint = 只显示您加星标的电台

schedule-all-favs = ★ 全部

schedule-all-favs-hint = 收藏所有时间表中的电台，并在保持开启期间在瀑布图上显示其标签。再次点击可清除全部收藏。

schedule-solar-time = 当地时间

schedule-solar-time-hint = 显示每行在发射台的当地时间。这是太阳钟，按经度每度四分钟计算，而不是民用时区，因此不考虑夏令时或时区边界。

schedule-map = 显示地图

schedule-map-hint = 打开 LISTEN，即此时间表的地图。在其中选一个电台，写下您听到的内容并记录。所选电台会一并带过去

schedule-on-air = { $time } UTC 有 { $count } 个在播

schedule-now = 现在

schedule-solar-hover = { $site } 的当地太阳时 { $local }，{ $lon }°

schedule-favourite-hint = 收藏此电台

rc-radio-classic = 经典收音机

rc-bands = 波段

rc-mem-programs = 记忆 · 节目

rc-recall-band-mode-frequency-and = 调用：一次性恢复波段、模式、频率和滤波器

rc-no-memories-yet-store-one-with-mem = 还没有记忆，请用顶栏中的 MEM 存储一个

rc-broadcast-schedule-what-is-on-when = 广播时间表：何时、何地播什么

rc-bw-this-mode-has-one-fixed = BW：此模式只有一个固定通带

rc-bw-click-next-filter-right-click = BW：点击：下一个滤波器 · 右键：上一个 · 滚轮：步进

rc-step = 步进

rc-step-click-next-tuning-step-right = 步进：点击：下一个调谐步进 · 右键：上一个 · 滚轮：步进

rc-agc-click-next-speed-right-click = AGC：点击：下一档速度 · 右键：上一档 · 滚轮：步进

rc-open = 开启

rc-sql-click-open-back-to-the-last = SQL：点击：开启 / 回到上次的阈值 · 滚轮：升高或降低门限

rc-s-meter = S 表

rc-wheel-on-a-digit-click-upper-lower = 在数字上滚轮 · 点击上/下半部 ± · 右键清零 · 双击输入

rc-music = 音乐

rc-speech = 语音

rc-stereo-the-19-khz-pilot-is-locked = 立体声：19 kHz 导频已锁定，点击切换为单声道

rc-mono-the-broadcast-stereo-decode = 单声道：广播立体声解码已关闭，点击开启

rc-mono-noise-reduction-is-on-and = 单声道：降噪已开启，立体声让位于降噪

rc-mono-no-19-khz-pilot-the-station = 单声道：没有 19 kHz 导频，电台发送单声道，或信号太弱

rc-stereo = 立体声

rc-group-broadcast = 广播

rc-group-ham = 业余

rc-group-air = 航空

rc-group-marine = 海事

rc-cap-lw-mw-fm = 长波 / 中波 · FM

rc-cap-shortwave = SW · 短波

rc-cap-amateur-bands = 业余波段

rc-cap-airband = 航空波段

rc-cap-marine = 海事 VHF · MF

rc-cap-cb = 民用波段

rc-preset-fm-ext = FM 扩展 64-87.5

rc-mem-n = 记忆 { $id }

controls-action-tune = 调谐 VFO

controls-action-rit-offset = RIT 偏移

controls-action-xit-offset = XIT 偏移

controls-action-volume = 音量

controls-action-sub-volume = 副接收音量

controls-action-squelch = 静噪

controls-action-filter-width = 滤波器宽度

controls-action-filter-shift = 滤波器偏移

controls-action-agc-max-gain = AGC 最大增益

controls-action-manual-gain = 手动增益（AGC 关）

controls-action-tx-drive = TX 驱动

controls-action-tune-drive = 调谐驱动

controls-action-mic-gain = 麦克风增益

controls-action-digi-audio-freq = 数字音频偏移

controls-action-spectrum-zoom = 频谱缩放

controls-action-spectrum-pan = 频谱平移

controls-action-spectrum-floor = 频谱下限

controls-action-spectrum-ceiling = 频谱上限

controls-action-ptt = PTT

controls-action-tune-carrier = TUNE 载波

controls-action-mute = 静音

controls-action-noise-blanker = 噪声抑制器

controls-action-nr-cycle = 降噪（循环）

controls-action-auto-notch = 自动陷波

controls-action-binaural = 双耳音频

controls-action-agc-cycle = AGC（循环）

controls-action-sub-rx = 副接收机

controls-action-split = 异频

controls-action-rit-enable = RIT 开/关

controls-action-xit-enable = XIT 开/关

controls-action-rit-clear = RIT 清零

controls-action-xit-clear = XIT 清零

controls-action-vfo-toggle = VFO A/B 切换

controls-action-swap-vfos = 交换 A 与 B

controls-action-copy-a-to-b = 复制 A→B

controls-action-band-up = 上一波段

controls-action-band-down = 下一波段

controls-action-mode-next = 下一模式

controls-action-mode-prev = 上一模式

controls-action-record-toggle = 录音 开/关

controls-action-abort-tx = 中止发射

controls-action-cw-straight = CW 直键

controls-action-voice-stop = 语音键控器停止

controls-action-tone-burst = 1750 Hz 音频脉冲

controls-action-fit-span = 适配带宽

controls-action-zoom-in = 放大

controls-action-zoom-out = 缩小

controls-action-peak-hold = 峰值保持

controls-action-spectrum-collapse = 折叠频谱

controls-action-waterfall-collapse = 折叠瀑布图

controls-action-waterfall-flip = 翻转瀑布图

controls-action-toggle-help = 手册窗口

controls-action-toggle-settings = 设置窗口

controls-action-toggle-logbook = 日志窗口

controls-action-toggle-spots = 监测点窗口

controls-action-toggle-memories = 存储窗口

controls-action-toggle-voice = 语音键控器窗口

controls-action-workspaces = 工作区窗口

controls-action-speak-status = 语音播报状态

controls-action-speak-repeat = 重复上一条播报

controls-action-speech-silence = 停止播报

controls-action-speech-toggle = 播报 开/关

controls-action-broadcast-labels = 广播标签（已调谐/整个波段）

controls-action-select-vfo = 选择 VFO { $vfo }

controls-action-band = 波段 { $band }

controls-action-mode = 模式 { $mode }

controls-action-recall-memory = 调用存储 { $n }

controls-action-voice-slot = 语音键控器槽位 { $n }

controls-group-tuning = 调谐

controls-group-receive = 接收

controls-group-transmit = 发射

controls-group-display = 显示

controls-group-windows = 窗口

controls-group-speech = 语音

controls-midi-note = 音符

controls-midi-cc = CC

controls-midi-pitch-bend = 弯音

controls-midi-program = 程序

controls-midi-any = 任意

controls-midi-msg = { $kind } { $data }  通道 { $ch }

controls-midi-msg-pitch-bend = 弯音  通道 { $ch }

settings-controls-down = 下

settings-controls-rev = 反

antenna-antenna = 天线

antenna-cutting-lengths = 裁切长度

antenna-the-frequency-to-cut-for = 裁切所针对的频率

antenna-optimizing = 正在优化…

antenna-tune-the-yagi-s-element-lengths = 针对所选目标调整八木天线的振子长度和间距。在后台线程中进行局部爬山搜索，运行期间窗口仍可使用。

antenna-solved = 已求解

antenna-optimized-yagi = 优化后的八木天线

antenna-obstacles-approximate = 障碍物（近似）

antenna-trees = 树木

antenna-in-leaf = 有叶期

antenna-ridge = 山脊

dab-channel-received-mhz = 已接收频道 { $channel } { $mhz }

dab-no-channel-received = 未接收到频道

dab-channel-received = 已接收频道 { $channel }

dab-slideshow = 幻灯片

dab-no-picture-yet = 电台尚未发送图片

dab-no-station-chosen = 未选择电台

dab-no-logo-yet = 暂无台标

dab-playing = 播放中

dab-chosen = 已选

dab-no-text-yet = 电台尚未发送文字

dab-clear-channel-hint = 清除此频道及其电台。扫描会再次找到它。

dab-clear-all-channels = × 清除所有频道

dab-clear-all-channels-hint = 忘记所有频道及其上的电台。扫描会再次找到它们，星标保留。

dab-n-stations = { $n } 个电台

dab-scanning-progress = 正在扫描 { $channel } · { $at }/{ $n } · 已找到 { $found }

dab-clear-station-hint = 从列表中清除此电台。如果该复用仍包含它，下次扫描会将其找回。

dab-prev = 上一个

dab-prev-hint = 列表中的上一个电台。

dab-preset = 预设

dab-preset-hint = 给此电台加星标，或取消星标。

dab-next = 下一个

dab-next-hint = 列表中的下一个电台。

dab-pty-hint = 在列表中显示每个电台的节目类型。

dab-stop-sweep-hint = 在此停止扫描。已找到的内容保留。

dab-scan-hint = 遍历所有 Band III 频块，保留含有电台的频块。

swl-title = SWL 日志

swl-new = + 新建

swl-jobs = 任务

swl-jobs-hint = 定时录音——在设定的时间录制某个波段

swl-sigid = 信号识别

swl-sigid-hint = 这个频率上是什么？按频率、模式和带宽查找信号的指南

swl-replay = 回放

swl-replay-hint = 播放最近两分钟而非实时——找回刚刚错过的电台呼号

swl-report = 报告

swl-report-hint = 将此次接收复制为报告，以便发送给电台

swl-stats-heard = 已收听 { $n }

swl-stats-reported = 已报告 { $n }

swl-stats-qsl = QSL { $n }

swl-stats-pirates = 海盗台 { $n }

swl-stats-hint = 完整日志——“显示”筛选不会改变它

swl-antenna = 天线

swl-antenna-hint = 20 米长线、MLA-30 环形天线、mini-whip …

swl-antenna-tip = 将写入接收报告的“Antenna:”行

swl-tone = 音调

swl-tone-on = 开

swl-tone-bass = 低音

swl-tone-mid = 中音

swl-tone-treble = 高音

swl-report-copied = 报告已复制到剪贴板——请粘贴到您的邮件程序中。无法打开邮件处理程序。

swl-show = 显示

swl-find-hint = 电台、语言、台址 …

swl-find-tip = 匹配电台、语言、发射台址和备注

swl-all-bands = 所有波段

swl-all-days = 所有日期

swl-pirates-only = 仅海盗台

swl-pirates-only-tip = 仅显示无执照的电台

swl-adif-tip = 将完整日志保存为 ADIF——是接收记录，不是通联

swl-csv-tip = 将完整日志保存为 CSV，每次接收一行

swl-shown-of = { $shown } / { $n }

swl-empty = 尚无记录——调谐到一个电台并按 + 新建。

swl-transmitted-from = 发射自 { $site }

swl-click-to-copy = （点击复制）

swl-addr-btn = 地址

swl-mail-btn = 邮件

swl-mark-sent = 已发送

swl-recall = 调回

swl-recall-tip = 调回此频率和模式——看看电台是否又回来了

swl-edit = 编辑

swl-del = 删除

swl-no-match = 没有符合筛选条件的接收记录——点击清除可显示全部。

swl-form-new = 新建接收记录

swl-form-edit = 编辑接收记录

swl-station-hint = 台湾国际广播电台

swl-language-hint = 英语

swl-received = 接收地

swl-received-hint = 您的网格

swl-email = 电子邮件

swl-email-hint = 接收报告

swl-email-tip = 将写入报告的“Send to:”行

swl-postal = 邮寄地址

swl-postal-hint = 街道、城市、国家

swl-postal-tip = 没有电子邮件时的备用报告投递地址

swl-heard = 收听时间

swl-smeter = S 表

swl-report-label = 报告

swl-judged = 评估

swl-report-sent = 报告已发送

swl-not-yet = 尚未

swl-qsl-received = 已收到 QSL

swl-awaiting = 等待中

swl-notes = 备注

swl-notes-hint = 节目备注

swl-pirate = 海盗台

swl-pirate-tip = 无执照广播——在任何频率分配之外发射的电台。日志会用海盗旗标记它。

swl-mail-why-no-address =
    此电台没有电子邮件地址——请在“节目表”窗口中查找，或在此输入一个，即可使用此功能。

    接收报告是一种礼节，电台会阅读并以 QSL 卡回复；没有地址就无处可发。

swl-mail-why-bad-address = 该邮件地址不是一个有效的单一地址，因此不予使用——把报告发给从未收听过该电台的陌生人，不如不发。

swl-mail-why-ok =
    撰写接收报告并在您的邮件客户端中打开。

    图片和音频片段需要您在邮件客户端中自行添加——本程序无法把文件放进邮件。

swl-mail-report = 邮件报告…

antenna-feed-ohms = 馈电 ≈ { $r } Ω

antenna-takeoff = 仰角 { $deg }°

antenna-director = 引向器 { $n }

antenna-reflector = 反射器

antenna-driven = 有源振子

antenna-gap = 间距 { $v } m

antenna-az = 方位

antenna-depth = 深度

antenna-height = 高度

antenna-attenuation = 0° 方向上的衰减：{ $att } dB

morse-no-paddle-found-connect-one-then = 未找到拍键——请连接一个，然后重新打开此面板

morse-send-with-a-paddle-played-here = 用拍键发送——仅在此播放，绝不发射

morse-no-paddle-found-in-dev-input-by-id = 在 /dev/input/by-id 中未找到拍键。请连接一个并重新打开此面板。能报告两个触点的键控盒（常表现为鼠标按键）可以使用；自带迭代键控并发送触摸信号的则不行。

morse-reverse = 反转

morse-swap-dit-and-dah-the-switch-on = 交换点和划。许多拍键上的开关可在键上直接完成此操作；此选项用于开关处于另一位置的情况。

morse-stop-key = 停止键控

morse-start-key = 开始键控

morse-key-the-character-above-the-keyer = 键控上方的字符。键控器会把点和划做得干净利落——您来选择用哪个拍键以及停顿多久。轻点为一个单元；按住则重复。

morse-the-sending-drill-needs-a-linux = 发送练习需要 Linux 桌面和拍键。

panels-float-chip-hint = 浮动——将此面板拆出为浮动面板，把空间还给瀑布图；其上的停靠标签可将其放回

panels-map-style-radar = 雷达——深色底图，陆地填充。适合阅读空中态势

panels-map-style-terrain = 地形——陆地抬升并加暖色，呈现为地面，国界变暗

panels-map-style-chart = 海图——浅色陆地、突出的国界和经纬网，航空图风格

panels-heard-me-chip = 听到我

panels-heard-me-hint = 在当前波段上，用圆圈标出每个报告收到此呼号的电台，数据来自 PSK Reporter，位置为报告者自己的网格。与 PSK 监测点相反：那些是本电台收到的电台。每小时刷新，涵盖最近一小时；需要在设置 → 监测点中打开 PSK 数据源，并在“常规”页设置呼号。

panels-period-label = 周期

panels-sub-mode-label = 子模式

panels-front-end-overloading = 警告：前端过载——请降低射频增益（LNA/IF），或移开天线

panels-front-end-overloading-hint = 接收机被驱动到超过满量程，因此音频很响但不含信号——在此期间无法解码任何内容。请调低射频/LNA 增益（增加衰减），降低 IF 增益，或启用 AGC。通常是附近的强发射台或同一波段上的另一台电台造成的。

panels-enigma-hint = 恩尼格玛密码机——在战时面板上加密消息，并破解您从空中抄收的密文

panels-slot-bar-heard-hint = 跟随扬声器中的声音：听到帧时为白色，停止后变为绿色，下一帧开始时回到起点。时间取自该模式，因此较短的帧仍会填满进度条。


morse-send-correct = 正确：{ $want }

morse-send-heard = 听到 { $got }，应为 { $want }

morse-send-received = 收到：{ $text }

amp-title-model = 功放 · { $model }

amp-state-off = 关

amp-state-on = 开

amp-relay-dn = 下

amp-relay-up = 上

amp-band-unknown = 未知

amp-needle-top = 顶部

decodes-not-uploaded = 警告：{ $services } 未上传

decodes-not-uploaded-hint = 此通联已在本地日志中，但上传失败。请在设置 → 监测点中检查令牌和连接，然后从日志中再次上传。

decodes-logged-to = ↑ 已记录到 { $services }

decodes-logged-to-hint = 同时已上传到设为接收每个新通联的在线日志。

decodes-known = 已知

decodes-known-hint = 此处带哈希的 <...> 当前可解析出的每个呼号及其国家。哈希无法反推——只有听到电台拼出自己的呼号后才会出现，这正是 11 m 波段裸呼号的用途。

decodes-ignored-clear = 已忽略 { $n } · 清除

decodes-auto-tx-frequency-hint =
    自动选择我们的发射频率：选择我们发射的时间段里最安静的位置，而不是我们所回应电台的频率——对方在另一个时间段发射，因此其频率无法说明我们按下发射时那里有谁。关闭并不会保持频率：它会在被呼叫电台的频率上应答。如需保持，请使用“保持发射频率”。

    有一个波段例外：在 11 m 上，WSJT-CB 的惯例是在呼叫者自己的音调上回应 CQ 并在整个通联期间停留在那里，因此会跳过寻找步骤，无论此项开或关，对 CQ 的回复都停留在听到它的频率上。

decodes-muted-caps = 已静音

decodes-muted-hint = 本次会话已静音——点击恢复收听

decodes-ignore-hint = 在本次会话剩余时间内忽略此电台：它会离开解码列表，不再触发提示音，也不会被提供给自动模式。不会代表它发送或上传任何内容，重启 sdroxide 后它会恢复。

decodes-qso-complete-hint = 此通联已完成并在本地日志中。它在此保留几分钟，以便对方重复其消息时可重发最后一条消息——无需再做其他事。

decodes-slot-even = 偶数

decodes-slot-odd = 奇数

decodes-before = 之前

decodes-after = 之后

decodes-beam-hint = 将天线指向此电台的网格定位

decodes-queue-next = 下一个

decodes-queue-waiting = 等待中


decodes-setup-hint = 设置——这些 QSO 模式共用的电台身份和模板

decodes-auto-armed-hint =
    { $role } 已启用。点击停止。

    { $note }

    引擎步骤：{ $step }——仅当处于 CallingCq 且没有进行中的通联时，才会接受应答 CQ 的电台。

decodes-auto-rx-hint = 无人值守地应答新电台的 CQ，自己绝不呼叫 CQ。只应答日志中尚没有的呼号。发射看门狗控制节奏，无活动停止功能使其解除。

decodes-auto-continuous-hint = 完整的无人值守流程：完成 QSO，与呼叫您的电台通联，应答呼叫 CQ 的电台，仅在没有其他事可做时才呼叫 CQ。持续运行直到您停止{ $until }。{ $bound }

decodes-auto-or-minutes-up = { " " }或 { $n } 分钟时间到

decodes-auto-cq-run-unlimited = 每轮 CQ 本身没有呼叫次数限制。

decodes-auto-cq-run-bounded = 每轮 CQ 以自身的呼叫次数为限。

decodes-auto-no-call-limit = 本身没有呼叫次数限制——由看门狗和无活动停止来结束运行。

decodes-auto-after-calls-listen = 呼叫 { $repeats } 次后切换到自动接收并持续收听。

decodes-auto-after-calls-stop = 呼叫 { $repeats } 次后停止。

decodes-auto-cq-hint = 无人值守地呼叫 CQ 并与应答者通联。{ $bound } 运行太短，安静的波段听不到您；太长，则发射机会停在一个无人应答的频率上。

decodes-auto-stopped-by-operator = { $role } 已被操作员停止

decodes-cont = 续

settings-servers-decoded-text-udp = 通过 UDP 输出解码文本

settings-servers-decoded-text-udp-note = 每次解码输出一行 JSON，供您自己的脚本使用：FT8 系列、WSPR 以及所有键盘模式的解码行（CW、RTTY、PSK、Olivia、THOR、MFSK、DominoEX、Hell），每行都带有听到时的频率。上面的 WSJT-X 广播是日志程序的协议，没有针对这些模式的消息，因此才单独提供此功能。

settings-servers-decoded-text-udp-hint = 主机:端口，经 UDP。留空为关闭——默认值。仅输出：此套接字上的任何内容都无法控制电台。名称会在您按下应用时解析。

settings-servers-rotator-note = 将带电机的天线指向您锁定的卫星，或通过旋转器窗口和地图手动指向。默认使用 Hamlib rotctld 守护进程（例如运行“rotctld -m 603 -r /dev/ttyUSB0”用于 GS-232B 旋转器，或运行“rotctld -m 1”在没有硬件时试用）——一个守护进程即可涵盖 Hamlib 支持的所有旋转器。也可以直接驱动串口上的控制器。

settings-servers-connection = 连接

settings-servers-rotctld-port-hint = rotctld 默认为 4533

settings-servers-baud-hint = 大多数控制器使用 9600

settings-servers-pstrotator-note = PSTRotator 自己的 UDP 端口。请在其设置中打开 UDP 控制；它在此端口上接收命令，并在下一个端口上回复。

settings-servers-el = 仰角

settings-servers-persist-re-connect-rotator = 保存并（重新）连接旋转器

settings-servers-tci-note = 允许支持 TCI 的程序控制此电台：频率和模式控制、宽带 IQ、接收音频和发射音频。在 WSJT-X 中选择 SunSDR (TCI) 电台类型，指向此地址，并将 PTT 设为 TCI。

settings-servers-decoded-text-udp-format = 每行为一个数据报，以换行符结尾：{ $example }

settings-servers-transport-rotctld = Hamlib rotctld（TCP）

settings-servers-transport-easycomm = EasyComm II（串口）

settings-servers-transport-gs232 = Yaesu GS-232（串口）

settings-servers-transport-pstrotator = PSTRotator（UDP）

spots-active = 活跃

spots-closing = 即将关闭

spots-none-left-in-the-window = 窗口内已无剩余

spots-openings = 开通

spots-band-opening-detections-paths = 波段开通检测：近期活动量超过其自身 3 小时基线的路径

spots-save-the-spots-you-can-see-as-a = 将您看到的监测点保存为 KML 文件——可用于 Google Earth 以及任何其他能读取它的地图工具。只包含已打开的筛选项，且只包含带位置的监测点，因为没有坐标的标记点会落在世界的另一端。

spots-no-spots-to-save-the-filters-are = 没有可保存的监测点——筛选项全部关闭

spots-bc-all = 全部广播

spots-bc-tuned = 已调谐广播

spots-which-broadcasters-the-waterfall =
    瀑布图标注哪些广播电台的名称。

    { "**" }已调谐广播**为默认设置，遵循 bebepip 的规则：只标出您所调谐电台的名称，不标其他——整个波段全部标上名称，就会在您想读取的通带上形成一堵名称墙。

    { "**" }全部广播**会标出节目表中当前所有在播电台的名称，无论其位置，适合浏览波段而非收听某个电台时使用。它与上方的广播标签是否显示广播电台无关。

app-rx-only-hint = 此电台只能接收——没有可键控的发射机。

spots-n-calls = { $n } 个呼叫

spots-opening-hover = { $band } 波段从 { $from } 到 { $to } 的路径：最近 15 分钟窗口的活动量超过该路径的 3 小时基线——近期呼叫者：{ $calls }


spots-download-unsupported = 在此平台上无法下载 { $source }

spots-kml-partial = 已保存 { $placed } / { $total }——其余没有位置，无法标记

spots-net-log = 联网日志（{ $n }）

decodes-beam = 波束

settings-radio-no-answer-yet = 正在等待电台所连接的机器：这些查询涉及其硬件和网络，它一次只回答一个。

settings-radio-elad-no-baud = 电台没有 { $baud } 波特率设置——将改用 { $default }

settings-radio-elad-antenna-rx-only = 仅接收

settings-radio-rx-only-n = RX{ $n } 仅接收

settings-radio-ng-level = nG 电平

settings-radio-ng-level-hint =
    nG 增加了 2.00x 固件所没有的两个命令，而这里是唯一可以设置它们的地方：nG 既不应答也不存储它们，因此每当链路打开时 sdroxide 都会重新发送。

    音量是电台通过 USB 流发送的电平。nG 取的是经过 VOL 和 AGC 之后的音频，因此要修正解码器收到的信号过小或过大，应在这里调整，而不是电脑的混音器。

    增益控制是电台自己的 AGC 设置。自动会在任何数字模式下设为 DIGI，其他情况下设为开，这正是 nG 的说明对 FT8 所要求的：这样增益会在对方电台发射间隙中保持，而不会向上调节进入间隙。

settings-radio-ng-volume-hint = 0-31，格式为 `AG0nn;`。电台流向电脑的电平。

settings-radio-gain-control = 增益控制

settings-radio-speaker = 扬声器

settings-radio-on-while-streaming = 传输时开启

settings-radio-ng-speaker-hint = 关闭会在传输期间关闭电台自己的扬声器（`UA2;`），这样声音只在 sdroxide 中听到。2.00x 没有这种形式。

settings-radio-ng-agc-auto = 自动（数据模式下为 DIGI）

settings-radio-ng-agc-off = 关

settings-radio-ng-agc-on = 开（快）

settings-radio-sound-card-note = 哪块声卡承载此电台的音频，在**音频**中与您自己的扬声器和麦克风一起设置——无论您使用哪台电台，这都是同一组设置，所以只有一页。

settings-radio-no-control-cable-audio-page = 此电台没有控制线——它通过 VOX 自行触发发射，因此没有可下达命令的 PTT。发射就是向电台麦克风输入音频，所以电台的发射音频在音频页而不在这里。

settings-radio-sound-card-note-short = 哪块声卡承载音频，在**音频**中设置。

settings-radio-control-link = 控制链路

settings-radio-atsmini-host-hint =
    接收机的临时 TCP 服务器：`atsmini.local`（mDNS）或其 IP。在电台上，设置 → TCP 端口 → 临时，并将 Wi-Fi 设为连接、AP+连接或仅 AP。

    `atsmini.local` 需要本机上有 mDNS 解析器（Avahi/nss-mdns）。如果控制链路无法建立，其 IP 会显示在电台的 Wi-Fi 屏幕上——请改为输入该 IP。

settings-radio-atsmini-find-hint = 扫描本机网络以查找接收机的控制端口

settings-radio-tcp-port = TCP 端口

settings-radio-tcp-port-hint = 固件默认为 60000。

settings-radio-atsmini-receive-only = 仅接收：Si4732 在硬件中解调，因此这里选择的模式只是命令电台切换，其余一切——检测、解码、日志——都在本电脑上完成。

settings-radio-atsmini-tuning-note =
    调谐跟随接收机自身的波段循环，因此刻度盘会比您的滚动滞后片刻——跨入另一个波段时滞后最明显。请等它稳定后再继续滚动，它总会落在您所要求的频率上。

    接收机的调谐步进由其自身决定（上方的 STEP 控件）：比该步进更细的变化会在电台上被舍去，刻度盘会回到它实际落到的步进上。

settings-radio-live-controls = 实时控制

settings-radio-atsmini-vol = 音量

settings-radio-atsmini-step = 步进

settings-radio-atsmini-sent-at-once = 立即发送到电台。音量是接收机自己的音量（它的旋钮，也是 Si4732 的），不是 sdroxide 的 AF 滑块。

settings-radio-refresh = 刷新

settings-radio-read-99-slots = 从接收机读取 99 个槽位

settings-radio-tune-radio-slot = 将此电台调谐到该槽位

settings-radio-no-slots-stored = 接收机上没有存储任何槽位。

settings-radio-press-refresh-slots = 按刷新读取接收机的存储槽位。

settings-radio-icomnet-receive-hint = AF：电台负责解调，sdroxide 显示音频频带。12 kHz IF：电台改为发送其 DRM 中频，由 sdroxide 解调，从而在约 ±12 kHz 范围内使用 sdroxide 自己的滤波器、降噪和解码器。无论哪种方式，宽带瀑布图都是电台自己的频谱显示——Icom 不输出 I/Q。

settings-radio-copy-icom-session-hint = 将此电台最近一次会话——其握手和 CI-V 跟踪记录——复制到剪贴板（在不允许网页复制的浏览器中则保存为文件），用于提交错误报告。仅限此电台，而非整个电台站：局域网中有两台 Icom 时，每个标签页只回答其自己地址的情况。

settings-radio-copy-protocol-trace-hint = 将最近一次会话的协议跟踪记录复制到剪贴板（在不允许网页复制的浏览器中则保存为文件），用于提交错误报告。

settings-radio-copy-trace-device-hint = 将最近一次会话的跟踪记录复制到剪贴板（在不允许网页复制的浏览器中则保存为文件），用于提交错误报告：包括与设备交换的每条命令，以及采样流的前几个字节。

settings-radio-copy-trace-receiver-hint = 将最近一次会话的跟踪记录复制到剪贴板（在不允许网页复制的浏览器中则保存为文件），用于提交错误报告：包括与接收机交换的每条命令，以及采样流的前几个字节。

settings-radio-pluto-choose-receiver = 请选择您的接收机

settings-radio-dab-rate-hint = 如需解码 DAB/DAB+，请选择 2.500 Msps

settings-radio-rx888-dab-rate-hint = 如需解码 DAB/DAB+，请选择 80.0 Msps 和 2.500 MHz 带宽（1/32）

settings-radio-pluto-both-chains-hint = 要同时收听两根天线：在同一地址上添加第二台电台并设为 RX2。两台电台共用同一频率。

settings-radio-duplex-tdd-hint =
    AD9361 是同时运行两个方向（FDD，Pluto 的启动方式，也是 sdroxide 一贯保持的状态），还是一次只运行一个方向（TDD）。

    除非您需要下面的 PTT 引脚，否则请保持 FDD——这些引脚由 TDD 触发，且它与上面的全双工互斥。应用后生效。

    PlutoSDR：全双工已关闭——TDD 一次只启用一个方向，GPO 收发引脚正是据此触发。要在一次发射间隙中收听，请将 PTT 引脚设为关闭并将双工设为 FDD。

settings-radio-fm-mw-notch = FM/MW 陷波

settings-radio-fm-mw-notch-hint = 针对 FM 广播（88-108 MHz）和中波波段的硬件陷波，用于本地发射台使其他一切信号过载的情况。立即生效。

scanner-broadcast-band = 广播波段

scanner-all = 全部

scanner-stop-only-on-channels-that-sound = 只在听起来有人说话的信道上停止：人声是周期性的，充满语音频带，并且每秒起伏几次，而载波、鸟鸣干扰和数据脉冲则没有这些特征。每个候选信道至少收听半秒，因为听出一个音节至少需要这么久。

scanner-nothing-in-this-scan-is-in-a-mode = 此次扫描中没有任何信道处于能承载语音的模式——在这里搜索语音会一直运行而不会在任何位置停止，因此在出现这样的模式之前不提供此功能。FM、AM 和单边带能承载语音；数字模式则不能。

scanner-voice = 语音

scanner-stop-only-where-somebody-is = 只在有人说话处停止

scanner-no-voice-to-look-for-in-these = 这些模式中没有可查找的语音

scanner-sql = 静噪

scanner-how-long-to-stay-on-a-candidate = 在判断之前在候选信道上停留多久。语音检测至少需要这么久：人声是音节的节奏，半秒就是两个音节

scanner-listening-for-a-voice = 正在收听语音…

scanner-scan-band-hover = 扫描 { $name }（{ $lo }-{ $hi } kHz）

scanner-range-to = 至

scanner-mode-no-speech = { $mode } 不承载语音——跳过这些信道

scanner-voice-holding = 语音 { $score } · 有人在说话

scanner-voice-passed = 已跳过 { $mhz } MHz · 语音 { $score }

aircraft-hfdl-aircraft = HFDL 飞机 { $id }

aircraft-no-icao = 无 ICAO 地址（HFDL）

aircraft-provenance = { $source }，{ $age }前

aircraft-label-flight = 航班

aircraft-label-registration = 注册号

aircraft-label-position = 位置

aircraft-label-altitude = 高度

aircraft-label-speed = 速度

aircraft-label-track = 航迹

aircraft-label-last-message = 最后一条消息

aircraft-label-type = 机型

aircraft-label-operator = 运营方

aircraft-label-origin = 出发地

aircraft-label-destination = 目的地

aircraft-label-online = 在线

aircraft-missing-flight = 尚未收到——ADS-B 或 VDL2 会提供

aircraft-missing-registration = 尚未收到——只有 VDL2 会提供

aircraft-missing-position = 尚未收到——ADS-B，或 VDL2 或 HFDL 的位置报告

aircraft-missing-adsb = 尚未收到——ADS-B 会提供

aircraft-missing-message = 暂无——VDL2，或在 VDL2 识别出该飞机后的 ACARS 音频

aircraft-online-unavailable-disabled = 在线功能不可用——在线开关已关闭

aircraft-online-unavailable-no-station = 在线功能不可用——没有可查询的台站；只有电台数据

aircraft-online-unavailable-unauthorized = 在线功能不可用——未登录台站

aircraft-online-unavailable-retry = 在线查询失败——稍后将重试

aircraft-switch-no-answer = 台站没有应答

aircraft-switch-not-signed-in = 未登录台站

aircraft-switch-refused = 台站拒绝了该请求

aircraft-switch-too-old = 台站不提供此开关（版本太旧？）

aircraft-switch-failed = 台站无法更改

aircraft-online-source = adsbdb，经由台站

aircraft-asking-station = 正在询问台站

aircraft-provenance-radio = 电台

aircraft-provenance-radio-online = 电台 + 在线

aircraft-provenance-online = 在线

aircraft-photo-credit = 照片：{ $photographer } / Planespotters

rotor-no-antenna-rotator-is-configured = 尚未配置天线旋转器。

rotor-set-one-up-in-settings-servers-a = 请在设置 > 服务器中设置一个——Hamlib rotctld 守护进程，或串口上的控制器。然后即可从此刻度盘、呼号或地图指向波束。

rotor-halt-the-antenna-where-it-is = 让天线停在当前位置

rotor-park = 归位

rotor-hand-the-antenna-back-to-the = 把天线交还给卫星锁定——它会跟踪卫星，没有卫星时则归位。

rotor-point = 指向

rotor-dx-country = DX 国家

rotor-set-your-grid-locator-settings = 请设置您的网格定位（设置 > 常规），以便指向某个国家。

rotor-connected = 已连接——天线位于 { $az }°

rotor-not-connected-why = 未连接：{ $e }

rotor-beam = 波束 方位 { $az }° 仰角 { $el }°

rotor-target = 目标 { $az }° / { $el }°

rotor-park-hover = 转到归位方向（{ $az }° / { $el }°）

rotor-country-bearing = { $name }——方位 { $az }°

aircraft-switch-no-station = 没有可切换的台站

aircraft-title = 飞机

aircraft-mode-adsb-hover = ADS-B，1090 MHz——带有位置、高度和速度的飞机

aircraft-mode-hfdl-hover = HFDL，高频数据链——海洋和极地上空的飞机，来自其位置报告

aircraft-mode-vdl2-hover = VDL2，甚高频数据链——来自飞机的消息；很少带位置

aircraft-mode-acars-hover = ACARS——飞机与地面之间的短消息

aircraft-badge-online = 在线

aircraft-badge-history = 历史

aircraft-stats-total = { $n } 架飞机

aircraft-stats-most-first = 最多优先

aircraft-stats-empty = 尚无飞机。表格由 ADS-B、VDL2、HFDL 和 ACARS 填充；这里的一切都不需要互联网。

aircraft-stats-relay-note = 机型、运营方和航线只统计中继已回答过的飞机——请在卡片中打开一张卡片进行查询。

aircraft-stats-known-kinds = 已知 { $known } · { $kinds } 种

aircraft-and-more = 以及另外 { $more } 个

aircraft-show = 显示

aircraft-aviation = 航空

aircraft-cards-chip = 卡片 { $n }

aircraft-cards-hover = 通过 ADS-B、VDL2、HFDL 和 ACARS 收到的每架飞机，每架一张卡片——保留十分钟，无论电台当前处于什么模式

aircraft-watch-chip = 关注 { $n }{ $suffix }

aircraft-watch-hover = 您要求提醒的飞机，按呼号或注册号——列表由台站保存，当其中一架升空联络时会显示提醒

aircraft-stats-chip = 统计

aircraft-stats-hover = 此台站已知的飞机中最常见的类别、航空公司、机型、运营方和航线——优先按电台数据统计，并以中继的卡片作为补充

aircraft-export = 导出

aircraft-export-csv-hover = 将飞机保存为 CSV 表格，供电子表格使用：若台站保留历史，则为其最近 24 小时，否则为当前收到的。不含位置。

aircraft-export-json-hover = 将飞机保存为 JSON：若台站保留历史，则为其最近 24 小时，否则为当前收到的。不含位置。

aircraft-watch-intro = 您希望得到提醒的呼号和注册号。列表由台站保存，因此在每个屏幕上都相同；当其中一个出现，或在离开空中十分钟后重新出现时，会发出提醒。

aircraft-watch-needs-station = 关注列表由您所连接的台站保存。请连接一个台站以使用它。

aircraft-watch-reading = 正在读取列表…

aircraft-watch-cannot-read = 无法读取列表：{ $why }。

aircraft-watch-input-hint = KLM1234 或 PH-BXA

aircraft-watch-add = 添加

aircraft-watch-sending = 正在发送…

aircraft-watch-empty = 列表中暂无内容。

aircraft-watch-on-air = 当前在空中

aircraft-watch-remove = 移除

aircraft-watch-alerts = 提醒

aircraft-watch-clear = 清除

aircraft-watch-none-yet = 暂无。

aircraft-cards-empty = 尚未收到飞机。卡片由 ADS-B、VDL2、HFDL 和 ACARS 填充——请按其中一个并收听。这里的一切都不需要互联网。

aircraft-cards-no-match = 没有飞机匹配“{ $query }”。

aircraft-history-hover = 根据台站自己的记录，显示这架飞机最近一天去过的地方。它只询问您的台站，绝不访问互联网。

aircraft-copy-summary = 复制摘要

aircraft-copy-summary-hover = 将此卡片复制为文本：已知内容及每一行的来源。位置和最后一条消息不包含在内。

aircraft-asking-station-dots = 正在询问台站…

aircraft-not-changed = 未更改：{ $why }。

aircraft-loading-photo = 正在加载照片…

aircraft-radio-is-on = 电台处于 { $mode }——按 { $label } 开始收听。这是 { $label } 之前收到的内容。

aircraft-no-adsb = 尚未收到 ADS-B。

aircraft-hfdl-off = HFDL 解码器已关闭——按 HFDL 将其打开。

aircraft-vdl2-starting = 正在启动 VDL2 解码器…

aircraft-no-view = { $label } 在此处没有视图。

textmodem-waiting-modem = 等待调制解调器…

textmodem-locked = 已锁定

textmodem-no-frame = 无帧

textmodem-locked-hint = 正在跟踪一帧。只有当十二帧叠加后的同步相关值明显高于其自身的噪底，且帧周期自相关表明存在帧时，才会接受该帧。

textmodem-sync-readout = 同步 { $sync }（噪底 { $null }）

textmodem-sync-hint = 在每个帧相位上叠加的差分同步相关值，以及这些相位上的中值，即必须超过的噪底。实测空中信号：4285 为 0.85，而另一种模式的强 PSK 载波为 0.14。

textmodem-frame-readout = 帧 { $acf }

textmodem-frame-hint = 一帧的内容在下一帧重复的程度。4285 会重复其 80 个同步符号，因此为 0.3-0.6；另一种模式的载波约为 0.03。

textmodem-refs-readout = 参考 { $a }/{ $b }/{ $c }

textmodem-refs-hint = 解扰后三个参考块的集中度。它们在加扰前都是符号 0，所以正确的帧会使每个参考块收敛到一点，即 0.7 以上。十六个符号的噪声只有 0.25。

textmodem-rate-hint = 星座图所能表明的信息。每个未编码速率与一个编码速率共用星座图，因此要区分编码速率需要附录 E，读取其载荷也需要附录 E 的前向纠错。

textmodem-offset-frames = { $offset } Hz · { $n } 帧

textmodem-offset-frames-hint = 各参考块一致认定的载波偏移，以及本次会话已解调的帧数。

textmodem-som-readout = SOM { $best }/32 · { $n } 条消息

textmodem-som-hint = 附录 E 的消息起始码型，以及解码器输出与它的接近程度。噪声约在 32 位中的 16 位；编码传输即使因衰落而未能匹配，也能达到二十几位，而匹配即可确定编码速率并开始一条消息。编码和交织器能准确解码生成的信号，但尚未从空中读到任何编码电台，因此出现电台时，这里的匹配是最先应出现的现象。

textmodem-rx-only = 仅接收

textmodem-rx-only-hint = 此模式的解码器已移植，可读取其录音；其调制器尚未编写，因此发送部分被关闭，而不是事后才拒绝。

textmodem-msg-hint = 你自己的消息按钮：工作条件、天气、标准回复。每个按钮一次发送其全部文字，F1-F9 对应前九个。它们随电台配置一起保存，所以远程客户端也能使用。

textmodem-weather-chip = {" "}天气 { $n }{" "}

textmodem-weather-hint =
    在此波段收到并解码的 SYNOP 和 SHIP 报告。

    它们保存在这里而不在解码列表中，因为单独的数字没有意义，而你往往想回头查看三小时前某位置的天气。

    从到达的 RTTY 文本中找出，因此必须把模式调到电台的频移和波特率。陆地站不发送位置，所以这些行只有站号和数据而没有坐标；船舶报告则带有自己的位置。

textmodem-olivia-reverse-hint = 双向反转音调顺序。在开启之前，反向发送的电台读出来只是噪声。

textmodem-olivia-tones-raised = （Olivia 的 { $n } → 4）

textmodem-ifk-reverse-hint = 反转音调步进。IFK+ 的数据承载于相邻音调之间的步进，因此在相反边带上收到的电台每个步进都是负的，在开启之前读出来只是噪声。

textmodem-domino-hint = { $name }，{ $baud } 波特，18 个音调，增量频移键控，无前向纠错

textmodem-mt63-long-hint = MT63-{ $label }，带宽 { $bw } Hz，长交织将突发错误分散到 { $chars } 个字符上

textmodem-mt63-short-hint = MT63-{ $label }，带宽 { $bw } Hz，短交织，短交织将突发错误分散到 { $chars } 个字符上

textmodem-mt63-not-offered = 暂不提供 1K 和 500L

textmodem-mt63-not-offered-hint = MT63-1K 和 500 Hz 长交织可用批处理解码器读取，但尚不能通过实时接收机读取：锁定会搜索符号相位和极性，但不搜索频带中心，而且发现加入这种搜索只是以一种变体换另一种变体。参见 docs/internal/MODE-PROGRESS.md。

textmodem-mfsk-caveat-mfsk8 = { " " }，语料库录音丢失了一段文字；解码器与其他所有变体使用的相同

textmodem-mfsk-caveat-mfsk11 = { " " }，语料库录音因重采样器启动而丢失了开头的一组

textmodem-mfsk-hint = { $label }，{ $baud } 波特，{ $tones } 个音调，带宽 { $bw } Hz{ $caveat }

textmodem-mfsk-reverse-hint = 反转音调顺序。在与发送时相反的边带上接收的电台会以中心为轴镜像，在开启之前读出来只是噪声。

textmodem-psk-family = 系列

textmodem-psk-family-plain-hint = PSK31 自己的系列：单个差分载波，无前向纠错

textmodem-psk-family-bpsk63f-hint = 62.5 波特的稳健编码，无交织器，是 fldigi 唯一不带交织器发送的编码模式

textmodem-psk-family-robust-hint = 单载波，带稳健编码和 2x2xN 交织器

textmodem-psk-family-multi-hint = 多个载波，PSK31 的变长码，**无**编码：更宽更快，需要干净的信号

textmodem-psk-family-robust-multi-hint = 多个载波**加上**稳健编码：PSK 模式中最宽也最稳健的一种

textmodem-psk-baud-fast-hint = { $baud } 波特，约一秒后即可读出：在此速率下，接收机的锁定时间是一条消息的三分之一

textmodem-psk-baud-hint = { $baud } 波特

textmodem-psk-modulation = 调制

textmodem-psk-bpsk-hint = 每符号一比特，差分检测

textmodem-psk-qpsk-hint = 每符号两比特，带 1/2 码率编码，因此每符号一个数据比特：相同速度下文本速率与 BPSK 相同，而错误减半

textmodem-psk-8psk-hint = 每符号三比特，无编码：三者中同等速度下最快，也最不稳健。仅接收，没有八相发射机。已在合成信号上验证：测试语料库中的 8PSK 录音并不是干净的星座图

textmodem-psk-coded = 编码

textmodem-psk-no-code = 无编码

textmodem-psk-coded-hint =
    { $baud } 波特，{ $n } { $n ->
            [one] 个载波
           *[other] 个载波
        }，{ $code }，带宽 { $bw } Hz，仅接收：编码 PSK 发射机尚未编写

textmodem-psk-wide-bank-hint = { "*" }*这个宽频组暂时无法解码。** 它的载波靠得太近，接收机在输入速率下的滤波器无法将它们分开；较窄的频组可以读取各自的录音。你在这里听到的内容不会丢失，问题出在前端而不在信号。

textmodem-hell-slant = 倾斜

textmodem-hell-slant-hint = 接收采样时钟微调。若捕获的点长了 0.4 %（普通声卡就是如此，SkySweep 的空中 Hell 录音实测也是这样），需要约 -4000。调整到字形直立为止。

settings-net-rbn-enabled-hint = 读取全球的 CW 解码站，并用它们为传播地图提供数据。这使地图能显示本电台没有在收听的波段。它不会向空中发射任何内容，无需账号，并使用“常规”选项卡中的呼号。RBN 的发现不会出现在发现列表中，因为它们是测量数据，而不是邀请。主机并不限于 reversebeacon.net：任何使用相同 telnet 数据流的节点都可以。

settings-net-rbn-port-hint = 在 reversebeacon.net 上，7000 是 CW/RTTY 数据流，7001 是 FT8/FT4 数据流

settings-net-wavelog-intro = Wavelog 是由**你**自己运行的日志，地址由你指定。这里记录的通联会上传到你选择的电台位置，它已有的通联也可以拉回来，这样“已通联过”就能知道在其他程序、其他电台或它自己的网页表单中做的 QSO。

settings-net-wavelog-address-hint = Wavelog 首页所在的位置：https://log.example.org，或其局域网地址。带不带 /index.php 都可以。

settings-net-wavelog-key-hint = 来自 Wavelog 自己 API 页面的 API 密钥。上传需要读写密钥；只读密钥只能下载。

settings-net-pick-one = 请选择

settings-net-test-to-list-locations = 测试连接以列出位置

settings-net-trust = 信任

settings-net-trust-hint = 仅对此服务器接受**这个**证书，不接受其他证书。这是固定证书而不是“接受一切”：一旦证书变化，连接会被拒绝并给出提示，而不是悄悄连到该地址上任何应答的对象。

settings-net-test-wavelog = 测试 Wavelog

settings-net-test-wavelog-hint = 询问服务器此地址和密钥是否可用，并列出其电台位置。会先应用设置；不记录任何内容。

settings-net-wavelog-download = 从 Wavelog 下载通联

settings-net-every-min = 每隔（分钟）

settings-net-locations = 位置

settings-net-download-now = 立即下载

settings-net-download-now-hint = 取回自上次下载以来 Wavelog 新增的内容，并把它已有的通联标记为已上传。日志中已有的内容不会被更改。

settings-net-wavelog-upload-note = 只有在**本机**记录的通联会自动上传；Wavelog 自己的积压记录需要在日志中勾选 QSO 才会发送，因此没有你未选择的内容会离开本机。

settings-net-broadcast-note = 在瀑布图上标注的长波和短波电台。下面的每个来源都会按当前广播季下载，季节更替时重新下载。它们都进入**同一个库**，因此“节目表”窗口的来源筛选可以只显示其中一个或全部。

settings-net-downloaded = 已下载

settings-net-schedules = 节目表

settings-net-schedule-yours = 自己的

settings-net-download = 下载

settings-net-hfcc-hint = HFCC 把电台名称放在另一个文件中，所以两个都需要。在此之前，它会如实说明，而不是加载只有呼号的节目表。

settings-net-refetch-hint = 重新获取本季的副本，替换已缓存的副本

settings-net-from-file = 从文件…

settings-net-aoki-file-hint = 从你自己保存的文件读取 Aoki 的节目表：可以是 Aoki 发布的 .zip，或其中的 .txt。它的网站会随季节变动，所以才有这个选项。

settings-net-eibi-file-hint = 从保存在本机的 EiBi sked .csv 读取其节目表

settings-net-filter-aoki = Aoki 节目表

settings-net-filter-eibi = EiBi 节目表

settings-net-download-all = 全部立即下载

settings-net-download-all-hint = 获取所有可获取的来源，保留已经可用的来源

settings-net-own-list-note = 你自己的列表是一份**节目表**，即你期望听到的内容，而不是记录你实际听到内容的接收日志。请在下面导入，或直接编辑该文件。

settings-net-import-list = 导入我的电台列表…

settings-net-import-list-hint = 读取 CSV 或纯文本电台列表，并合并到上面的文件中。频率可用 kHz 或 MHz，单位根据数字判断。

settings-net-filter-station-lists = 电台列表

settings-net-dx-status = DX：{ $body }

settings-net-pin-own = 自有：{ $pin }…

settings-net-schedule-rows = { $name } { $n } 次发射

settings-net-schedule-not-downloaded = { $name }     未下载

settings-net-schedule-imported = { $source } { $season }：{ $n } 次发射，已存入库中

settings-net-stations-imported =
    { $n ->
        [one] 你的列表中有 { $n } 个电台，已存入库中
       *[other] 你的列表中有 { $n } 个电台，已存入库中
    }

txaudio-natural = 自然

txaudio-ragchews-and-local-contacts-the = 闲聊和本地通联：降噪、停顿时轻柔的扩展器、轻度压缩。听起来是你本人，只是去掉了房间的声音。

txaudio-ssb-everyday = SSB 日常

txaudio-everyday-ssb-100-hz-2-9-khz-a = 日常 SSB：100 Hz - 2.9 kHz，2 kHz 附近略加临场感，中度压缩。

txaudio-ssb-punch-dx = SSB 强劲 / DX

txaudio-weak-signals-and-pileups-a-narrow = 弱信号和 pileup：300 Hz - 2.7 kHz 的窄带语音，带强烈的临场感提升，送入带相位旋转器的快速压缩器，以获得发射机能给出的最大话音功率。不是高保真。

txaudio-ssb-punch-for-hours-in-a-noisy = 嘈杂电台长时间使用的 SSB 强劲预设：降噪和强力噪声门把风扇声和旁边的操作员挡在你的停顿之外，压缩器工作得更用力。

txaudio-essb-hi-fi = ESSB / 高保真

txaudio-wide-ssb-or-am-50-hz-4-khz-with = 宽带 SSB 或 AM：50 Hz - 4 kHz，保留低频，轻度压缩并带去齿音器。需要这么宽的发射滤波器，并且波段要安静。

txaudio-fm-repeater = FM / 中继

txaudio-fm-a-steep-300-hz-high-pass-keeps = FM：陡峭的 300 Hz 高通使语音远离 CTCSS 亚音，高端在 3 kHz 截止以适应信道，限幅器控制频偏。

txaudio-mobile-noisy-shack = 车载 / 嘈杂电台

txaudio-fans-engine-traffic-the-denoiser-a = 风扇、发动机、车流：降噪、150 Hz 的陡峭高通和强力噪声门，并有足够的压缩，使声音高于剩余的噪声。

txaudio-digital-voice-2 = 数字语音

txaudio-freedv-rade-the-denoiser-and = FreeDV / RADE：仅降噪和轻柔的电平调整，不做均衡，因为声码器更喜欢自然语音。会打开数字语音。

txaudio-imported-2 = 已导入

frame-no-contrast-fit = 无对比度，点击以适配

frame-levels-hidden-hint = 显示的下限和上限都超出了屏幕上的所有内容，所以瀑布图是一片单色。点击可使其适配电台当前听到的内容。

frame-bw-waterfall = 带宽瀑布图 · { $span }

frame-bw-waterfall-unknown = 带宽瀑布图 · 未知

frame-every-module-own-window = 每个模块都在各自的窗口中

frame-window-chip-hint = 窗口：把此窗口移到独立的系统窗口，以便放到另一个显示器上

frame-dock-panel-pane-hint = 停靠：把此面板放回瀑布图下方的窗格

frame-dock-panel-hint = 停靠：把此面板放回瀑布图下方

frame-dock-spectrum-hint = 停靠：把此频谱放回主窗口，效果与关闭此窗口相同，无需寻找关闭按钮。

frame-dock-band-menu-hint = 停靠：把波段和模式选择器放回主窗口，效果与关闭此窗口相同，无需寻找关闭按钮。

frame-dock-rotator-hint = 停靠：把转台放回主窗口，效果与关闭此窗口相同，无需寻找关闭按钮。

frame-module-panadapter = 全景显示

frame-module-panel = 面板

frame-module-controls = 控制

frame-module-aux-panadapter = 辅助全景显示

frame-module-band-mode = 波段与模式

frame-module-rotator = 天线转台

frame-detached-title = { $flavor }：{ $name }

frame-workspace-missing = 未保存工作区 { $n }。请在“设置 → 界面 → 工作区”中保存一个，此按键就会调出它。

frame-no-operating-panel = { $mode } 没有操作面板：它已取消停靠，而且此模式没有操作面板。

frame-rx-only-nudge = 这台电台只能接收，没有发射机。要隐藏发射控制并使用收听界面吗？

frame-listening-controls = 收听控制

frame-listening-controls-hint = 隐藏发射控制，并把这台电台切换到收听界面（设置 → 电台）。不会重新调谐，解码器继续运行。

frame-update-available = SDR Oxide Brown { $tag } 已发布，你当前使用的是 { $version }。

frame-download = 下载

frame-download-hint = 在浏览器中打开发布页面

frame-tuning-catching-up = 调谐中，电台正在跟上

frame-roll-up-hint = 卷起：离开标题栏，把窗口还给瀑布图

frame-unroll-hint = 展开：重新显示面板

frame-window-panel-hint = 窗口：给此面板一个独立窗口，以便放到另一个显示器上

frame-panel-window-title = { $mode }：面板

frame-auto-stopped-stop-control = 已自动停止：使用了停止控制

frame-dont-show-again = 不再显示

frame-dont-show-again-hint = 在此屏幕上记住这次确认。只要引擎以 --oob-tx 运行，锁定仍然是关闭的，这只是不再显示警告。

frame-cb-window-title = 11 m（CB）：不是业余波段

frame-cb-about-to-allow = 你即将允许在 11 m 波段发射。

frame-cb-not-amateur = 11 m 不是业余波段。它是另一种无线电业务，有自己的信道、模式、功率限制和型号核准设备，而且各国规定不同。在那里发射须遵守你所在国家的规定。

frame-cb-only-continue = 只有在你清楚本国允许在 27 MHz 做什么并且在规定范围内操作时才继续，发射的责任仍由你承担。

frame-cancel = 取消

frame-saved-to-profile = 已保存到配置文件 { $name }

frame-saved-as-default = 已保存为本电台的默认值（服务器没有密码，所以每个客户端得到的都是这个）

frame-bindings-offer-title = 此配置文件带有键盘绑定

frame-bindings-offer-profile = 你登录所用的配置文件（{ $name }）在服务器上保存了键盘和鼠标绑定，而此浏览器没有使用它们。这是安全的默认做法：在多人共用的电台上，键盘也是共用的，一个登录采用另一个人的 PTT 或空格键是真正的隐患。

frame-bindings-offer-default = 本电台的默认配置文件在服务器上保存了键盘和鼠标绑定，而此浏览器没有使用它们。这是安全的默认做法：在多人共用的电台上，键盘也是共用的，一个登录采用另一个人的 PTT 或空格键是真正的隐患。

frame-bindings-offer-choice = 可以在这里使用它们，也可以不用。无论哪种，都会告知你结果，未经你选择不会更改任何内容。

frame-bindings-use = 使用此配置文件的绑定

frame-bindings-keep-mine = 保留我的（并不再询问）

frame-bindings-using-profile = 正在使用为此配置文件保存的键盘绑定

frame-bindings-keeping-own = 保留本设备自己的键盘绑定；不再询问

frame-look-back-to-profile = 已恢复为此配置文件保存的外观

frame-look-none-stored = 此配置文件尚无保存的外观，请先保存一个

frame-antenna-rotator-caption = 天线转台

bands-condx-global-forecast-rest-what = CONDX 是全球预报。WSPR 是全球 WSPR 网络听到的内容（wspr.live）；PATHS、REACH 和 BEST 是本电台和 RBN 听到的内容。空行表示没有解码到任何内容，可能是波段关闭了，也可能只是没有人在上面。

bands-km-s-parent-peak-rate =
    { $name }（{ $code }），{ $velocity_kms } km/s，母体 { $parent }。峰值速率 ZHR { $zhr }，出现在 { $peak } 前后。

    辐射点在地平线以上意味着流星余迹可以到达你这里；快速流星雨会留下存在时间更长的电离余迹，可用于 6 m 和 2 m 的流星余迹散射，以及短暂的 10 m/11 m 开通。

bands-hearing-means-path-open-each = { $callsign } 位于 { $location }：听到它意味着 { $band } 路径已开通。10m（28.200 MHz）是最接近 11 m 条件的业余波段替代。每个信标每 10 秒升高一个波段，所以这一行每个时隙都会变化。

bands-wspr-activity-old = · WSPR 活动数据已有 { $age }（wspr.live）

bands-psk-activity-old = · PSK 活动数据已有 { $age }

bands-activity-hover =
    过去 15 分钟内 { $band } 的全球活动，来源：{ $source }：
    { $paths } 份接收报告 · { $tx } 个发射台 · { $rx } 个接收台

    { $note }

    波段越繁忙，显示越亮；它表示该波段在某处被听到，而不表示对你开通。

bands-derived-hover =
    { $band } 不是 HAMQSL.com 评级的波段之一；这是已发布的 { $group } 组的判断，视为最接近的替代。

    这是预报，不是对你自己路径的测量。

bands-wspr-note = 全球 WSPR 网络：弱信号信标。

bands-psk-note = 来自活动模式的接收报告：FT8、FT4，以及 WSPR 信标未涵盖的 CW/RTTY 报告。

aircraft-history-reason-disabled = 电台没有开启历史记录

aircraft-history-reason-unauthorized = 电台需要登录

aircraft-history-reason-no-station = 历史记录由你所连接的电台保存

aircraft-history-reason-unknown = 无法理解电台的应答

aircraft-history-show = 显示历史

aircraft-history-hide = 隐藏历史

aircraft-label-history = 历史

aircraft-label-first-heard = 首次收到

aircraft-label-last-heard = 最后收到

aircraft-label-track-points = 轨迹点

aircraft-label-highest = 最高

aircraft-label-callsigns = 呼号

aircraft-label-registrations = 注册号

aircraft-history-nothing-kept = 尚未保存此飞机的任何记录

aircraft-history-by-station = 来自本电台

aircraft-history-kept-24h = 保存最近 24 小时

aircraft-history-of-kept = 占已保存的点

aircraft-history-used-24h = 最近 24 小时使用

aircraft-history-seen-24h = 最近 24 小时见到

aircraft-history-looked-up = { $age }前查询

aircraft-alert-on-air-as = { $entry } 正在空中，呼号为 { $callsign }

aircraft-alert-on-air = { $entry } 正在空中

aircraft-watch-already = 已在列表中。

aircraft-watch-absent = 不在列表中。

aircraft-watch-invalid = 不是呼号或注册号：应为 3 到 10 个字母或数字。

aircraft-watch-full = 列表已满：100 架飞机。

multi-module-windows = 模块窗口

multi-in-its-own-window = 独立窗口

multi-docked = 已停靠

multi-dock-all-windows = 全部停靠

multi-bring-every-module-window-back = 把所有模块窗口放回主窗口

multi-undock-all-windows = 全部取消停靠

multi-put-every-module-in-a-window-of = 让每个模块都使用自己的窗口：主窗口只留下频谱和解码器

multi-two-radios-side-by-side-need-a = 并排显示两台电台需要更宽的窗口，此屏幕一次只显示一台

multi-the-station-s-windows-some-modules = 电台的窗口：有些模块在各自的系统窗口中。打开此项可更改。

multi-put-a-module-in-its-own-window-sp1 = 把模块放进独立窗口：SP1 或 AUX SP 放到另一个显示器上，接收控制放在它们旁边。同样的行也在“设置 → 界面 → 已取消停靠”中。

multi-module-sp1 = SP1：频谱

multi-module-sp2 = SP2（AUX）：第二个频谱

multi-module-panel = 操作面板

multi-module-controls = 接收控制

multi-module-band-mode = 波段与模式

multi-module-rotor = 天线转台

multi-bring-back = 把 { $name } 放回主窗口

multi-put-own-window = 把 { $name } 放进独立的系统窗口：它有自己的应用标识，因此窗口规则可以把它固定到某个显示器

multi-windows-chip = 窗口

aircraft-stats-category = 类别

aircraft-stats-airline = 航空公司

aircraft-stats-model = 型号

aircraft-stats-operator = 运营方

aircraft-stats-route = 航线

aircraft-stats-src-no-station = 统计当前收到的飞机。你所连接的电台会保存最近一天的数据。

aircraft-stats-src-asking = 正在向电台询问最近一天的数据…

aircraft-stats-src-ready = 电台保存的最近 24 小时数据，加上当前收到的飞机。

aircraft-stats-src-disabled = 电台没有开启历史记录，正在统计当前收到的飞机。

aircraft-stats-src-reason = { $reason }，正在统计当前收到的飞机。

aircraft-export-saved-station = 已保存 { $name }：电台最近 24 小时的数据。

aircraft-export-saved-local = { $reason }：改为保存最近十分钟内收到的 { $n } 架飞机。

login-remember-this-station = 记住此电台

login-remember-me = 记住我

login-this-station-keeps-your-sign-in = 登录信息由此电台保存，而不是此页面：其他电台上无需再填写，离开时也无需清除。

login-not-this-time-this-browser-forgets = 这次不记：此浏览器关闭时会忘记登录信息，所以下次会话会再次询问，设备上不会留下任何与此次登录有关的内容。

aircraft-label-range = 距离

aircraft-label-vertical = 垂直

aircraft-label-emergency = 紧急

aircraft-from-your-station = 来自你的电台

aircraft-footer-disabled = 在线查询不可用：电台上的中继已关闭

aircraft-footer-no-station = 在线查询不可用：没有可询问的电台

aircraft-footer-unauthorized = 在线查询不可用：未登录

aircraft-hover-close = 关闭（Esc）

aircraft-hover-lookup = 在线查询

aircraft-hover-lookup-hint = 向此电台的中继询问这架飞机：型号、运营方、航线和照片。在你按下之前不会询问任何内容。

aircraft-hover-lookup-waiting = 另一个查询正在进行，一次只能查询一个

aircraft-hover-history-hint = 这架飞机在最近一天去过哪里，来自你自己电台的记录。它只询问你的电台，绝不访问互联网。

auto-stopped-reason = 已自动停止：{ $reason }

auto-stopped-idle = 已自动停止：{ $mins } 分钟没有操作员输入

auto-answering = { $role }：正在应答 { $call }

auto-caller-went-away = 自动：呼叫方离开了

auto-nobody-left = 自动：没有可应答的对象了

auto-calling-cq = { $role }：正在呼叫 CQ（#{ $n }）

auto-cq-no-answer = 自动 CQ：呼叫 { $limit } 次后无人应答，转入收听（自动接收）

auto-listening = { $role }：正在收听

auto-role-stopped = { $role } 已停止：{ $sentence }

auto-rx-listening = 自动接收：正在收听

auto-cq-calling = 自动 CQ：正在呼叫 CQ

auto-cont-running = 自动连续：运行中

aircraft-search-missing-online = { $total } 架飞机中有 { $missing } 架没有在线数据，请打开它们的卡片

aircraft-search-shown-of = { $shown } / { $total }

aircraft-search-help =
    搜索已收到的飞机：地址（ICAO）、呼号、注册号、型号、运营方或类别。多个词必须全部匹配。可用 icao: call: reg: type: op: 把某个词限定在一个字段中，例如  reg:PH op:KLM。
    型号和运营方来自电台的在线查询，所以只有已查询过卡片的飞机才能搜到。搜索本身不会请求任何内容。

aircraft-search-hint = 搜索

aircraft-search-clear = 清除搜索

gridtracker-shade-squares-heard-live-decode = 对实时解码列表中听到的内容着色，而不只是日志中的内容。这是收听者版本的地图：当前空中的内容，以青色显示在已通联标记之下。

gridtracker-grid-ham = 网格/业余

gridtracker-maidenhead-squares-what-a-ham-log = Maidenhead 网格，即业余无线电日志所记录的内容。CB 呼号不会出现在这里：11 m 的交换只带国家，不带网格定位。

gridtracker-country-cb = 国家/CB

gridtracker-dxcc-countries-at-their-nominal = DXCC 国家，位于其名义中心，即 CB 日志所记录的内容，因为 11 m 的交换中没有网格定位。这里同样不会出现业余呼号：这里是国家，不是网格。

colouredit-paste-a-colour-code-rrggbb-or-r-g = 粘贴颜色代码（#RRGGBB 或 r, g, b），或在色块上按 Cmd/Ctrl+V

cwkey-thread-failed = 无法启动 CW 键线程：{ $error }

cwkey-grab-failed = 无法独占电键桨，其触点可能也会在其他窗口中产生点击（是否有其他程序在使用它？）

ignore-muted-hover =
    本次会话已静音：{ $names }。

    被静音的电台不在解码列表中，不触发提醒，不被朗读，也绝不会被自动模式当作应答对象。不会代表它发射或上传任何内容，该列表在 sdroxide 关闭时即被清除，没有存储副本，所以一晚收听之后不会有任何内容保持隐藏。

    点击可全部取消静音，或按其任一行上的 - 按钮。

js8-setup-hint = 设置：呼号、群组、自动回复、信标间隔和状态消息

logbook-contest-logger-a-session-dupes-a = 竞赛记录：一次会话、重复检查、实时得分和 Cabrillo

logbook-up-chip = 上传

net-wavelog-stopped = 已停止：{ $error }

net-wavelog-download-stopped = Wavelog 下载已停止：{ $error }

net-wavelog-new-already-here = 新增 { $added }，已有 { $matched }

net-wavelog-fill-in-first = 必须先填写地址和 API 密钥

net-wavelog-no-station-location = 没有可供下载的电台位置，请先测试连接

publicsdr-could-not-connect = 本次会话无法连接

publicsdr-use-chip = 使用

weather-title = 天气报告

weather-intro = 来自船舶和陆地站的编码天气报告，使用 RTTY，频率为 4583、7646 和 10100.8 kHz。SHIP 约在 02:00 和 07:35 UTC 发出，SYNOP 约在 00:35 和 12:10。报告保存在这里而不在解码列表中，因为单独的数字没有意义。

weather-nothing-decoded = 尚未解码到任何内容。只需在上述时间之一让 RTTY 保持运行，这些是波段上最安静的时段。

weather-head-call-day-hour = { $call }：{ $day } 日 { $hour }:00 UTC

weather-head-day = { $day }:00 UTC

weather-head-report = 报告

weather-not-reported = { $label }：未报告

weather-in-their-words = 原话：{ $remark }

weather-ago-now = 刚刚

weather-ago-minute = 一分钟前

weather-ago-minutes = { $m } 分钟前

weather-ago-hours =
    { $h } { $h ->
            [one] 小时
           *[other] 小时
        }前

weather-ago-days = { $d } 天前

sat-off-only-satellites-with = 关：只显示有频率记录的卫星（你能通联的那些），以及你自己粘贴的卫星。开：电台跟踪的所有卫星，其中大多数是没人能听到的。

adsbmap-tune-hint = 双击调谐 · 按住 Shift 使用另一个调谐盘

adsbmap-airport-large = 大型机场 · 标高 { $elev }

adsbmap-airport-medium = 中型机场 · 标高 { $elev }

adsbmap-from-station = 距你的电台 { $dist }  { $bear }°

adsbmap-aircraft-within =
    { $dist } 内有 { $n } { $n ->
        [one] 架飞机
       *[other] 架飞机
    }

adsbmap-navaid-mw = { $khz } kHz（中波）

adsbmap-click-to-close = 点击地图关闭

settings-ui-broadcast-stations-hint =
    “收听”窗口和瀑布图标注可以显示哪些广播电台。

    { "**" }本范围内**是在有这个选项之前所有屏幕的做法：拨盘两侧各半个频宽，这正是该瀑布图上的标注应当覆盖的范围。**整个波段**显示节目表中所有带位置的电台，这正是浏览波段的收听者想要的，代价是地图更拥挤。

settings-ui-waterfall-labels = 瀑布图标注

settings-ui-tuned-only = 仅已调谐

settings-ui-tuned-only-hint = 只标出你所调谐的电台，不标其他：只有一个名字，对应你正在收听的对象，而不是通带上一堵名字墙。

settings-ui-whole-band = 整个波段

settings-ui-whole-band-hint = 标出节目表中当前在播的每个广播电台，无论它在哪里。适合浏览波段而不是收听某个电台，因此也更拥挤。在“发现”窗口中也有，名为 BC ALL。

settings-ui-classic-ui = 经典界面

settings-ui-classic-ui-hint =
    把所有控件放在左侧的一列中，各组可以折叠，就像 SDR++ 和 SDR++Brown 的布局，其余空间全部留给画面。控件相同，设置相同，只是排列成大多数短波收听者早已习惯的样子。

    它需要足够宽的窗口以保证瀑布图的份额：窗口更窄时会省略这一列，按常规绘制工具条，并且可以从面板右边缘拖动调整宽度。关闭时，完全像以前一样使用本程序自己的工具条。

settings-ui-classic-ui-check = 控件放在左侧一列

settings-ui-updates = 更新

settings-ui-updates-check = 启动时检查新版本

settings-ui-updates-hint = 启动时向此分支的 GitHub Releases 查询一次最新版本，如果比当前构建更新，则显示横幅。只计正式发布版，夜间版属于预发布，关闭则完全不进行外部请求。

settings-ui-undocked = 已取消停靠：独立窗口

settings-ui-docked = 停靠在此窗口中

settings-ui-dock-all-hint = 把所有已取消停靠的窗口放回此窗口。

settings-ui-undock-all-hint = 让每个模块都使用自己的窗口：主窗口只留下频谱和解码器。没有内容可显示的模块，例如语音模式下的操作面板，不会打开窗口。

settings-ui-spectrum-line = 频谱线

settings-ui-spectrum-line-hint = 频谱线下方的颜色及其着色方式：“固定”全程单一颜色，“渐变”在底部最浓并向顶部淡出，“混合”下半部分最浓并在上半部分淡出。线本身保持青色。频谱弹出窗口中的“填充”按钮切换填充，其线条行选择着色方式。

settings-ui-fill = 填充

settings-ui-fill-colour = 颜色

settings-ui-simple-ui = 简洁界面

settings-ui-simple-ui-hint =
    从顶部工具条隐藏高级附加功能：3D 视图、CW 解码站、频谱/瀑布图图层开关、奖状跟踪、卫星、ISM 解码和无线电邮件。CB 操作员或短波收听者常用的控件：调谐、模式、音量、静噪、带宽、瀑布图、记忆、扫描，都会保留。

    没有任何功能被关闭，只是被隐藏：重新打开此项即可把这些按钮带回来。

settings-ui-simple-ui-check = 隐藏高级按钮

settings-ui-wake-lock = 屏幕常亮

settings-ui-wake-lock-hint =
    此页面在前台时保持屏幕常亮，这样停在瀑布图上的手机或平板不会在一分钟没有触摸后变暗并锁屏。接收、音频和解码并不需要它，无论如何都会继续进行，它只是为了让屏幕保持可读。

    与手机和平板工具条上的挂锁配合：锁定屏幕以防误触，同时让它保持亮着。

    只要页面被隐藏，浏览器就会收回此锁，例如切换到其他标签页或按手机自己的电源键，页面回来后会立即再次申请。它需要支持此功能的浏览器和安全页面（https 或 localhost）；复选框旁边的文字会说明它何时不可用或被拒绝。它会耗电，所以在你要求之前保持关闭。电脑上安装的程序则遵循系统自己的电源设置。

settings-ui-wake-lock-check = 保持屏幕常亮

settings-ui-sound-welcome = 声音欢迎提示

settings-ui-sound-welcome-hint =
    网页打开时（新会话，或用 F5 重新加载），在你触摸页面之前浏览器不会播放任何声音。这是浏览器的规则，而且屏幕上没有任何提示时，新会话看起来像是坏了。

    开启后，屏幕中央会出现一条消息：点击、轻触或按任意键以开始播放声音。第一次点击、轻触或按键（任何位置，鼠标、触摸板、手指或键盘均可）会关闭它并开始播放声音，除此之外什么也不做：不会调谐，不会按下按钮，也不会运行快捷键。它只在页面打开时显示，会话期间无论声音之后发生什么都不会再显示。

    知道该怎么做之后可以关闭：声音仍然要等你第一次触摸，而那次触摸照常起作用。随你的配置文件保存在服务器上。默认开启。

settings-ui-sound-welcome-check = 声音欢迎消息

settings-ui-sound-welcome-browser-only = 仅限浏览器，安装的程序无需触摸即可播放声音

settings-ui-profile-screen = 屏幕跟随你的配置文件

settings-ui-profile-screen-hint =
    当你是 `sdroxide --server` 的远程客户端时，此屏幕的*外观*（主题、布局、瀑布图和频谱、字体、简洁界面、地图图层、全景显示自己的视图）会与你登录所用的配置文件一起保存，因此在任何机器上都能恢复，而不是从默认值开始。

    { "**" }保存在服务器上，并且在你登录期间随时自动保存**，无需按任何按钮。这是有意为之：浏览器存储不可依赖，因为 `persist()` 在每个浏览器中的行为都不同，而且保存在那里的会话无法由不在该设备前的人重置。服务器上的配置文件是电台操作员可以恢复的文件。

    在**没有密码**的服务器上，所有客户端都是同一个配置文件，所以除非你按下面的按钮，否则不会写入任何内容，一个操作员的主题不应变成下一个人的主题。属于机器本身的内容绝不会被保存，本地电台不受影响。

settings-ui-save-to-profile = 保存到配置文件

settings-ui-back-to-profile = 恢复配置文件

settings-ui-carry-bindings = 携带控制绑定

settings-ui-carry-bindings-hint =
    同时把你的键盘和鼠标绑定保存在服务器上的配置文件中，让它们随登录在浏览器和设备之间跟随你，思路与上面的屏幕设置相同，只是针对按键。

    { "**" }不推荐，默认关闭。**在多人共用的电台上，键盘也是共用的：携带绑定的配置文件在登录的瞬间就可能重新绑定另一位操作员的 PTT、空格键或调谐键，开启它还可能产生目前尚不明显的其他影响。只有在服务器仅归你一人所有、且你从自己的多台设备访问时才开启。

    仅限此分支：这不是上游功能。上面的屏幕设置总会随之携带，这里是唯一需要选择开启的。

settings-ui-carry-bindings-on = 已开启，不推荐

settings-ui-carry-bindings-off = 携带（不推荐）

settings-ui-carry-bindings-title = 在服务器上携带控制绑定？

settings-ui-carry-bindings-body =
    这会把你的键盘和鼠标绑定与你的配置文件一起保存在服务器上，使它们随登录在浏览器和设备之间跟随你。

    在其他人也使用的电台上，键盘是共用的。携带绑定的配置文件只要登录就可能重新绑定另一位操作员的 PTT、空格键或调谐键，而且启用它还可能产生目前尚不明显的其他影响。

    这就是不推荐的原因。只有服务器仅归你一人所有时才开启。

settings-ui-carry-bindings-confirm = 我明白，仍要携带

settings-ui-start-swl = 以 SWL 模式启动

settings-ui-start-swl-hint =
    每次会话打开时都已开启 SWL 模式，使程序启动后就是收听者的界面，对每台电台都是如此，这是屏幕的偏好而不是某台电台的偏好。

    每台电台的开关在“设置 → 电台 → 发射控制”，因此有一台收听电台和一台收发信机的电台，应在那里设置收听电台并关闭此项。使用那个开关会在本次会话中关闭这个初始设置。`--swl` 始终优先。

settings-ui-start-swl-check = 启动时开启 SWL 模式

settings-ui-listen-hides-tx = “收听”选项卡隐藏发射

settings-ui-listen-hides-tx-hint =
    在波段/模式菜单中选择**收听**会隐藏该选项卡上的所有发射控制，选择**操作**则恢复这台电台自身设置所规定的状态。

    它不能打开发射机。设置为仅接收的电台仍然只能接收，波段锁定和带外警告不受影响。

    波段菜单会记住上次停留的选项卡，所以停在收听状态的电台重新打开时仍是收听状态：下次启动时发射控制同样被隐藏，选择“操作”即可恢复。默认关闭，因为它改变了选项卡的含义；它只询问一次。

settings-ui-listen-hides-tx-check = 在“收听”选项卡隐藏发射

settings-ui-listen-hides-tx-title = “收听”选项卡将隐藏发射控制

settings-ui-listen-hides-tx-body =
    在“收听”选项卡上发射控制会消失；在“操作”选项卡上它们会完全按这台电台自身设置的样子回来。

    它不会启用任何功能。这只是同一电台的一种视图，所以波段锁定、带外警告和发射保护都不受影响。

    选项卡会被记住：停在“收听”的电台启动时仍在“收听”，发射控制仍然隐藏，直到选择“操作”。

    值得注意：停在“收听”的电台仍会在选项卡所调谐的频率上接收，而被隐藏的控件并不等于被关闭的控件，发射之所以关闭是因为设置如此，而不是因为按钮看不见。

settings-ui-listen-hides-tx-confirm = 我明白，在“收听”时隐藏

settings-ui-swl-mode = SWL 模式

settings-ui-swl-mode-hint =
    隐藏这台电台的所有发射控制，包括 PTT、呼叫 CQ、发射电平、发送、信标等全部，并把工具条上的业余附加功能（发现、奖状）换成收听者的功能（节目表和 SWL 日志，即接收日志）。与“设置 → 电台 → 发射控制”是同一个开关，并且按电台设置：收听用的接收器可以处于此模式，而旁边的收发信机仍保留其发射机。

    硬件仍然可以发射，这只是隐藏控件。`--swl` 会在本次运行中对所有电台强制开启。

    点击“应用”后生效。

settings-ui-swl-mode-check = 隐藏所有发射控制

settings-ui-report-picture = 报告图片

settings-ui-report-picture-hint =
    把接收报告邮寄给广播电台时随报告附上的图片：你自己的卡片、一张照片，或任何你想让电台看到的东西。给电台一张图片，用于每份报告。

    不设置时，报告以纯文本发送，这也是一份完全合格的报告。

settings-ui-report-picture-none = 无，报告以文本发送

settings-ui-choose = 选择…

settings-ui-report-picture-pick = 选择本机上的图片

settings-ui-report-note = 报告备注

settings-ui-report-note-hint = 你的固定备注，预先填入每份接收报告：你是谁、用什么收听、电台只问一次而不是每封信都问的任何信息。是纯文件中的纯文本，不是机密。

settings-ui-report-note-hint-text = 可选，例如“详情可应要求提供”

sstv-keep-dial = 保持拨盘

sstv-keep-dial-on-hint = 开：选择 SSTV 时拨盘停在你调谐的位置，所以解码器可在任何频率上工作。已发布的 SSTV 频率仍在频率选择器中。点击可改回落在该波段的 SSTV 频率。

sstv-keep-dial-off-hint = 关：选择 SSTV 时，若你距离任一 SSTV 频率超过 3 kHz，拨盘会移到该波段的 SSTV 频率上。点击可改为让拨盘保持在你调谐的位置，并在任何频率上解码 SSTV。选择 SSTV 之后的调谐无论哪种情况都不会被撤销。

sstv-clear-rx = 清除接收

sstv-clear-rx-hint = 清除“实时”视图中当前显示的图片。解码器继续运行；要放弃正在接收的图片并等待下一个帧头，请使用“重启接收”。

sstv-audio-present-no-header = 有音频 · 尚无 SSTV 帧头

sstv-editor = 编辑器…

sstv-editor-hint = 编排图片本身：可移动的文字、任意字体、横穿或纵贯画面的色带

sstv-save-image-as = 图片另存为…

sstv-save-copy-asks-file = 在本机保存一份副本，会询问保存到哪个文件

sstv-picture-still-loading = 图片仍在加载

sstv-share-hint-direct = 把这张图片发给别人：WhatsApp、Telegram，或此设备上的任何其他应用

sstv-share-hint-save = 保存这张图片并打开一封消息以发送它

sstv-share-picture = 分享图片…

sstv-reupload-hint = 把这张图片重新发出：加载到发射槽 { $slot }

sstv-radio-cannot-transmit = 这台电台无法发射

sstv-reupload = 重新上传

sstv-unidentified = 未识别

sstv-send-picture = 发送这张图片

sstv-qsl-to = 收件人

sstv-the-card = 卡片

sstv-the-card-hint = 这些内容会印在图片旁边的卡片上。任何一项留空就不会绘制，卡片绝不会写出没有人给过的报告。

sstv-date-utc = 日期（UTC）

sstv-rst-sent = 发出 RST

sstv-rst-received = 收到 RST

sstv-qsl-note = 备注

sstv-add-address = 请添加一个地址以发送。

sstv-saves-opens-mail-hint = 保存图片并打开你的邮件客户端，在那里添加附件只需点一下。

sstv-browser-cannot-attach-hint = 浏览器标签页无法给消息附加文件，所以这会保存图片并打开消息，请手动添加附件。

sstv-save-open-mail = 保存图片并打开邮件

sstv-mail-not-opened = 图片已保存，消息已复制，请粘贴到你的邮件程序中。无法打开任何邮件处理程序。

sstv-qsl-saved-where-chosen = { $name }（已保存到你选择的位置）

sstv-qsl-downloaded = { $name }：已下载到此浏览器的下载文件夹

sstv-picture-editor = 图片编辑器

sstv-slot-n = 槽 { $n }

sstv-slot-layers-hint = 此发射槽自己的图层

sstv-station-overlay = 电台叠加层

sstv-station-overlay-hint = 一组图层，每个槽都把它画在自己的内容之下：呼号，放在你喜欢的角落。设置一次，与每个槽自己的文字合成，而不是取代它。

sstv-draw-under-slot = 画在此槽之下

sstv-draw-under-slot-hint = 把电台叠加层画在此槽自己的图层之下。对已经带有识别信息的图片（通常是转发的拼图）请关闭，以免被盖两次章。

sstv-overlay-note = 电台叠加层：每个槽都把这些图层画在自己的图层之下。在图片上拖动一行可移动它。

sstv-slot-text-note = 此槽自己的文字，画在你放置的位置。在图片上拖动一行可移动它；该行的色带随之移动。

sstv-no-picture-in-slot = 此槽中没有图片，请在下面加载一张

sstv-base = 底图

sstv-base-hint = 这些图层画在哪个槽的图片之上

sstv-compose-over-hint = 在此槽的图片上合成

sstv-slot-empty = 此槽为空

sstv-picture-btn = 图片…

sstv-load-picture-hint = 把图片加载到此槽

sstv-use-received = 使用收到的

sstv-use-received-hint = 把刚收到的图片放到这些图层之下，这样回复中就带有对方的图片和你的呼号

sstv-received-now-base = 收到的图片现在是底图

sstv-add-text = ＋ 文字

sstv-add-strip = ＋ 色带

sstv-n-lines =
    { $n } { $n ->
            [one] 行
           *[other] 行
        }

sstv-draw-this-line = 绘制此行

sstv-strip-paren = （色带）

sstv-draw-later-hint = 比上面的行更晚绘制此行

sstv-draw-earlier-hint = 更早绘制此行

sstv-copy = 复制

sstv-face = 字体

sstv-colour = 颜色

sstv-line-text-hint = 这一行的内容

sstv-the-letters = 字母

sstv-composer-outline = 描边

sstv-the-edge = 字母周围的边缘

sstv-placement = 位置

sstv-down-frame = 沿画面向下

sstv-down-frame-hint = 把这一行旋转四分之一圈，使其向下阅读，例如沿图片侧边写呼号

sstv-strip = 色带

sstv-behind = 在后面

sstv-behind-hint = 在这一行后面放一条色带，横穿图片或纵贯图片，以这一行为中心

sstv-strip-first-edge = 色带，第一条边

sstv-thickness = 粗细

adsbpanel-paths = 路径

adsbpanel-paths-hint = 将每架航班的航迹绘成一条线：从来处，经现处，到去处。关闭后，航迹改用点状轨迹绘制。

adsbpanel-nav = 导航

adsbpanel-nav-hint = 在图像下方显示内置数据表中的机场和无线电导航设施：大型机场始终显示，中型机场在较近视图中显示，视图放大到足以让一个台站成为一个地点时再显示导航台。可以收听到的中波信标以单独的颜色绘制。

adsbpanel-units = 单位

adsbpanel-units-hint = 飞机视图中高度、速度、爬升率和距离的显示方式。不影响解码器和导出文件。

adsbpanel-age = 时效

touchlock-locked-hover = 屏幕已锁定，防止误触。在此按住3秒解锁。

touchlock-unlocked-hover = 锁定屏幕以防误触：频率、波段、模式、滤波器和瀑布图将不再响应手指操作，接收和音频照常工作。按住3秒解锁。解锁后挂锁会静止10秒，之后轻点才能再次锁定，因此刚解锁的那根手指不会立刻把它锁回去。

touchlock-hold = 按住 { $s } 秒…

touchlock-unlocked = 已解锁

touchlock-locked-hold = 已锁定 · 按住3秒

touchlock-unlocked-relock = 已解锁 · { $s } 秒后锁定

wake-held = 屏幕保持常亮

wake-asking = 正在询问浏览器…

wake-unsupported = 此处不可用：需要支持该功能的浏览器和https页面

wake-refused = 被浏览器拒绝（省电模式？）

wake-native = 仅限浏览器：已安装的程序遵循系统的电源设置

tooltipcopy-copied = ✔ 已复制

chrome-hide-message-hover = 永久隐藏此消息。您的设置不会有任何变化，相关建议见手册。

chrome-close-message-hover = 关闭此消息。其他内容不变。

settings-ui-window-label = { $module }窗口

settings-ui-undock-hint =
    取消停靠后，此模块将移到独立窗口，主窗口把空出的空间让给其余部分：可以把瀑布图放到第二块显示器，把控制面板放到第三块。关闭其窗口即可重新停靠。

    在Wayland下大小可以恢复，但窗口位置由合成器决定：请让它浮动，并用匹配app-id "{ $app_id }" 的窗口规则把它固定到某块显示器。

settings-ui-workspace-restore-out = 恢复此布局：{ $n } 个模块位于独立窗口

settings-ui-profile-in-use-named = 使用中：配置文件 { $name }

settings-ui-profile-in-use-default = 使用中：本电台的默认配置

settings-ui-profile-not-in-use = 尚未使用：此登录没有存储任何内容

settings-ui-module-panadapter = 全景频谱

settings-ui-module-controls = 控制

settings-ui-module-aux = 辅助频谱（第二频谱）

settings-ui-module-band-menu = 波段和模式选择器

settings-ui-broadcast-scope-in-view = 当前频段内

settings-ui-fill-fixed = 固定

settings-ui-fill-gradient = 渐变

settings-ui-fill-mixed = 混合

settings-ui-spot-kind-heard-me = 听到我

settings-ui-wake-held = 屏幕保持常亮

settings-ui-wake-asking = 正在询问浏览器…

settings-ui-wake-unsupported = 此处不可用：需要支持该功能的浏览器和https页面

settings-ui-wake-refused = 被浏览器拒绝（省电模式？）

settings-ui-wake-native = 仅限浏览器：已安装的程序遵循系统的电源设置

settings-ui-workspaces = 工作区

settings-ui-workspaces-note = 一个有名称的窗口布局：哪些模块在独立窗口中，以及这些窗口的位置。可以保存当前布局，或恢复已保存的布局。最多十个。

settings-ui-workspaces-none = 尚未保存任何工作区。

settings-ui-workspace-restore-home = 恢复此布局：所有模块都在主窗口中

settings-ui-workspace-forget = 删除此工作区

settings-ui-workspace-name = 为此布局命名

settings-ui-workspace-save = 保存当前布局

settings-ui-workspace-full = SDRuno最多保留十个工作区，请先删除一个

settings-ui-workspace-save-hint = 以此名称保存当前布局

settings-ui-workspace-name-first = 请先输入名称

sstv-composer-fade = 淡化

sstv-layouts = 版式

sstv-name-layout-hint = 为此版式命名

sstv-keep-layout-hint = 以该名称保存此版式

sstv-layout-renamed = 已重命名为“{ $name }”

sstv-layout-saved-as = 已保存为“{ $name }”

sstv-rename-cancelled = 已取消重命名

sstv-no-layouts-yet = 尚无已保存的版式：请在上方命名一个。

sstv-load-layout-hint = 将此版式载入当前编辑的内容

sstv-layout-loaded = 已载入“{ $name }”

sstv-rename-layout-hint = 重命名“{ $name }”

sstv-layout-renaming = 正在重命名“{ $name }”

sstv-duplicate-layout-hint = 复制“{ $name }”

sstv-layout-copy-name = { $name } 副本

sstv-layout-duplicated = 已复制为“{ $name }”

sstv-forget-layout-hint = 删除版式“{ $name }”

sstv-layout-forgot = 已删除“{ $name }”

sstv-mosaic-header = 用已接收图片制作拼图

sstv-mosaic-nothing = 尚未接收任何图片：拼图由图库中的图片制作。

sstv-mosaic-picked = 已选 { $n } 张，最多 { $max } 张，按此顺序排版：顺序即网格位置。每张图片保持原有形状，空隙为黑色。

sstv-mosaic-earlier-hint = 将此图片前移一格

sstv-mosaic-later-hint = 将此图片后移一格

sstv-mosaic-max = 拼图最多容纳 { $n } 张

sstv-mosaic-compose = 合成到此槽位

sstv-mosaic-compose-hint = 制作拼图并载入此槽位作为其图片；已接收的原图不受影响

sstv-mosaic-composed = 已将 { $n } 张图片合成到槽位 { $slot }

sstv-clear-picks = 清除所选

sstv-composed-saved = 已保存

sstv-send = 发送

sstv-send-now-hint = 立即发射此合成图

sstv-composed-sending = 正在发送

sstv-forget = 清除

sstv-forget-hint = 清空此预设的文档：其图片重新由横幅条和普通标题负责

sstv-forget-done = 已清除：横幅和标题已恢复

sstv-copy-to-slot = 复制到槽位

sstv-copy-to-slot-hint = 让另一个槽位使用此图片和此文字，以便从那里发送

sstv-copied-into-slot = 已复制到槽位 { $n }

sstv-word-strip = 条

sstv-word-text = 文字

sstv-text-size = 文字大小

sstv-text-size-hint = 横幅文字相对于横幅条自身大小的百分比。100%与以往一致；QSO中需要在噪声较大的解码之后仍能看清呼号，此项可放大字体而不增加横幅条的高度。

sstv-placement-hint = 横幅条沿哪条边绘制。消息叠加层从相反的一端开始，因此横幅在底部时，图片顶部留给文字。

sstv-top = 顶部

sstv-bottom = 底部

topbar-ptt-hold-hint = 按住发射：点击一次锁定为开，再按一次松开


topbar-above-the-dial = 高于

topbar-below-the-dial = 低于

topbar-step-chip-hint = 按一次频率移动多远。轻点切换到下一步进：10 Hz、100 Hz、500 Hz、1、2.5、5、6.25、9、10、12.5 和 25 kHz。可在设置 › 界面中关闭此行。

topbar-mem-chip-hint = 记忆信道：调出已存储的频率

topbar-disp-menu-hint = 全景频谱：其两个图层、速度和细节、峰值保持，以及瀑布图电平、视图选项和Skimmer

topbar-phone-menu-hint = 手机条未显示的所有内容：波段、模式、接收机和发射机、解码窗口、设置和手册

topbar-group-band = 波段

topbar-group-system = 系统

topbar-caption-diversity = 分集

topbar-group-decode-windows = 解码窗口

topbar-group-extras = 扩展

topbar-grid-tracker-hint = 网格追踪：在地图上显示已通联的梅登黑德方格

topbar-settings-chip-hint = 设置：设备增益、天线、音频设备

topbar-windows-chip-hint =
    ▣  左键：将所有模块窗口收回
    右键：模块窗口列表

topbar-manual-chip-hint = 用户手册（F1）

topbar-module-windows-title = 模块窗口

topbar-wname-panadapter = SP1：频谱

topbar-wname-aux = SP2（AUX）：第二频谱

topbar-wname-panel = 操作面板

topbar-wname-controls = 接收控制

topbar-wname-band-menu = 波段和模式

topbar-wname-rotor = 天线转台

topbar-wstate-put-back = 将 { $name } 放回此窗口

topbar-wstate-floating-panel-only = 浮动是操作面板自身的状态：频谱或控制台在此窗口中只有一个位置

topbar-wstate-float = 将 { $name } 从面板中提出，浮动在此窗口之上

topbar-wstate-no-panel = { $mode } 中没有操作面板：在有操作面板的模式下才适用

topbar-wstate-own-window = 为 { $name } 提供独立窗口，可放在任意显示器上

topbar-wstate-here-now = { $name } 现在就在这里

topbar-dock-all = 全部停靠

topbar-dock-all-hint = 将所有模块窗口收回此窗口

topbar-undock-all = 全部取消停靠

topbar-undock-all-hint = 让每个模块都有独立窗口：此窗口只剩频谱和解码器

topbar-rx-tx-freq = 接收 { $rx }   发射 { $tx } MHz

topbar-band-dock-hint = 停靠：让波段和模式选择器在瀑布图旁保持打开，而不是每次都关闭此弹出窗口

topbar-band-window-hint = 窗口：在独立窗口中打开波段和模式选择器，宽度随意，可放在瀑布图旁的任意显示器上

topbar-band-return-popup-hint = 停靠：将选择器放回顶栏弹出窗口，每次选择后都会关闭

topbar-band-window2-hint = 窗口：在独立窗口中打开选择器，宽度随意，可放在任意显示器上

topbar-squelch-held-hint = 静噪被强制打开。点击以恢复原来的门限。

topbar-squelch-hold-hint = 点击以强制打开静噪，听听有没有信号；再次点击则原样恢复。

topbar-changed-tone = 音调

topbar-changed-loudness = 响度

topbar-nr-chip-hint =
    降噪：{ $strength } — { $engine }

    { $advice }

    点击循环切换强度：低 / 中 / 高 / 关。引擎在此行下方的一行中选择。

topbar-nr-off-narrow-hint = 降噪已关闭。下一行提供五种降噪器：RNNoise、DeepFilterNet3、spectral-bleach、NR2 和原始频谱降噪；点击此处即可按某个强度开启。语音请先用RNNoise。

topbar-eq-tone-hover = 音调：低音 { $bass } dB，中音 { $mid } dB，高音 { $treble } dB

topbar-eq-closed-hover = 音调：接收音频上的三段均衡器。点击展开

topbar-eq-loudness-on = 响度已开

topbar-stereo-yields-hint = 降噪或自动陷波运行时，WFM立体声关闭，因为它们会模糊两个声道。关闭它们即可收听立体声

topbar-stereo-playback-hint = 立体声：录音按录制时的样子在双耳中回放。点击强制为单声道

topbar-stereo-mono-recording-hint = 此录音为单声道：没有第二个声道可播放

topbar-preset-edges = { $lo } … { $hi } Hz

topbar-caption-tone = 音调

topbar-eq-on = 开

topbar-eq-bass = 低音

topbar-eq-mid = 中音

topbar-eq-treble = 高音

topbar-eq-remembered = 已为 { $mode } 记住：每种模式各自保存。

topbar-loudness-hint = 调低音量时提升低音（和少许高音），使小音量的扬声器听起来依然饱满。可与音调同时使用或单独使用；满音量时不起作用，也绝不会比满音量更响。默认关闭，并按当前模式记住。

topbar-caption-record-audio = 录音

topbar-stop-recording-after-min = { $minutes } 分钟后停止录音

topbar-caption-play-back = 回放

topbar-recordings-chip = ▶ 录音…

topbar-recordings-hint = 将录音回放到解码器中：错过的SSTV图片、微弱的RTTY或CW信号、一个FT8时隙，配合瀑布图和解码日志，如同正在空中接收。回放期间电台暂停工作。

topbar-ant-label = 天线

topbar-pwr-label = 功率

topbar-drive-less = 小

topbar-drive-more = 大

topbar-fill-hint = 为频谱线下方的区域着色，底部最浓，向上逐渐淡出至谱线，使信号既有轮廓又有分量而更为突出。颜色在设置 → 界面 → 频谱线中选择。不绘制在3D曲面上。

topbar-group-spectrum = 频谱

topbar-group-waterfall = 瀑布图

topbar-freeze-on-tx-hint = 在自己发射期间停止瀑布图滚动，并在发射结束后从中断处继续，这样画面是连续的接收历史，而不是一块显示自己载波的行（或者在发射时静音接收机的电台上，一块根本不曾发生的噪声行）。频谱线不受影响。适用于经常发射的模式，这样对方的发射就不会被自己的发射挤出屏幕。

topbar-fft-chip-hint = 瀑布图电平和对比度、FFT大小以及滚动方向

topbar-caption-waterfall = 瀑布图

topbar-flatten-hint = 拉平整个波段的底噪：比邻近区域更安静的一段会被抬高到接近它们的底噪，使弱信号在繁忙的波段上也能显示。它只会抬高，绝不会压低载波，且抬高幅度不超过20 dB。上方的频谱曲线保持测量原样。

topbar-log-swl-hover = 收听日志：听到的电台，附SINPO/SIO

topbar-programs-hint = 广播节目表：什么时间、什么频率、播什么

topbar-listen-hint = 此频段内有什么，显示在地图上：选一个电台，调谐过去，记下所听内容并记入日志

topbar-spots-hint = 实时报点：PSK Reporter（非SWL模式下还有DX Cluster、POTA和SOTA）

topbar-sigid-hint = 这个频率上是什么？按频率、模式和带宽查询信号的指南

topbar-extras-hint = 扩展：DAB和其他接收机，位于可分离的窗口中

topbar-fill-style-caption = 线

topbar-fill-style-hint = 频谱线下方颜色的着色方式。颜色本身在设置 → 界面 → 频谱线中选择。

topbar-fill-fixed = 固定

topbar-fill-fixed-hint = 单一颜色，谱线下方处处相同

topbar-fill-gradient = 渐变

topbar-fill-gradient-hint = 频谱底部最浓，向顶部逐渐淡出

topbar-fill-mixed = 混合

topbar-fill-mixed-hint = 下半部最浓，在上半部逐渐淡出

topbar-fill-switch-on = 请先打开填充再设置

topbar-nothing = 无

topbar-olivia-unconfirmed = OLIVIA：接收可用，发射尚未与其他程序对照测试。此解码器可读取其他电台（已通过实际空中信号确认），因此可以抄收Olivia QSO。我们自己的发射现在可通过自己的接收机往返解码，但尚未被fldigi或MultiPSK抄收，且有一处符号约定与fldigi源码不同，所以请检查第一次回复，不要指望对方一定应答。

topbar-dab-bandwidth = DAB / DAB+，已在空中验证。前端需要2.5 Msps带宽才有音频：在复用包占用的1.536 MHz下，接收机处于其ADC下限并会丢样，而DAB解码需要连续的数据流，因此复用包永远无法锁定，您什么也听不到。请先把电台的采样率设为2.500 Msps。

topbar-stanag-unproven = STANAG 4285：仅接收。识别已通过空中信号验证：4285可以锁定，其106.67 ms帧长得到测量，参考块也能收敛，因此面板可以说明频率上是什么信号以及速率多少。读取其*文本*尚未经空中验证：Annex E编码和交织器能准确解码生成的信号，但尚未抄收到任何编码电台。若出现编码文本则是真实的；完全没有文本则说明该电台未编码或在传送加密通信，这是最常见的情况。

topbar-cis11-unproven = CIS-11（TORG-11）：仅接收。俄罗斯气象电传机：100波特的2-FSK，频移约500 Hz，11位字符带一个奇校验位。已证明的内容来自真实录音的测量：接收机能自行找到音调，并读取空闲传输，123个字符中有121个校验有效。尚未确定的是十个数据位如何划分为字符，因此面板既显示这些位，也显示其ITA-2解读；而且通信内容本身是加密的，这是最常见的情况。

topbar-thor-unproven = THOR：接收端尚不能读取fldigi的发射。解码器可配合自身调制器工作，但尚未抄收任何空中或参考录音，因此请把显示的内容视为暂定。

topbar-mode-not-on-band = { $mode } 不用于 { $band }：请选择它所属的波段

topbar-radio-classic-hint = 经典收音机：用于广播收听的经典收音机界面。开启后它将独占接收机：其他模式及其窗口会被关闭

topbar-ats-bands-caption = 接收机提供的波段

topbar-ats-band-hover = { $name }：{ $mhz } MHz，{ $mode }为其默认模式。将接收机步进到此波段。

topbar-caption-mode = 模式

topbar-caption-digital = 数字

topbar-ats-receive-only = 仅接收：Si4732由硬件解调，没有可键控的部分。

topbar-band-forecast-derived =
    { $band }：≈{ $verdict }（{ $time ->
            [day] 白天
           *[night] 夜间
        }）：这是已发布的 { $group } 组的结论，是最接近的替代；HAMQSL.com没有对 { $band } 本身评级。这是预报，不是对您自身路径的测量。

topbar-keypad-readout-khz = { $digits } kHz

topbar-caption-band-keypad = 波段键盘

topbar-keypad-bands = 波段

topbar-keypad-bands-hint = 按键即波段名：按一个即可跳转，并保持当前模式。从MHz返回时会清空已输入的内容。

topbar-keypad-mhz-hint = 在按键上以千赫输入频率：14074即14.074 MHz。ENTER发送，CLEAR清空输入。

topbar-keypad-key-stay = 保持 { $mode }，调谐到 { $mhz } MHz：这是 { $band } 上的通联频率。

topbar-keypad-key-go = 前往 { $band }。

topbar-keypad-clear = 清除

topbar-keypad-clear-typing-hint = 清空已输入的频率。按键仍为MHz。

topbar-keypad-clear-bands-hint = 尚未输入任何内容：按键处于波段状态，按一下即前往某个波段。

topbar-keypad-enter = 确认

topbar-keypad-enter-empty-hint = 尚未输入任何内容。按键为MHz：请先按键。

topbar-keypad-enter-ok-hint = 调谐到 { $mhz } MHz，并返回波段键。

topbar-keypad-enter-range-hint = { $mhz } MHz 超出此电台的接收范围（{ $ranges } MHz）。

topbar-nothing-published = 未发布任何内容

topbar-tab-listen = 收听

topbar-tab-operate = 操作

topbar-listen-tab-note = 此标签页也可以隐藏发射控制：在设置 → 界面中打开“在收听标签页隐藏发射”

topbar-caption-show-bands = 显示波段

topbar-band-filter-hint = 在下方列表中只显示 { $filter } 波段。再次点击则显示所有波段。

topbar-show-every-band = 显示所有波段

topbar-all-chip = 全部

topbar-all-chip-hint = 通用覆盖：清除波段，让频率可以去任何地方。与波段过滤不同，后者只会缩小此列表。

topbar-caption-sw-metre-bands = 短波米波段

topbar-metre-band-hint = 调谐到 { $name } 广播的中间位置

topbar-caption-wideband = 宽带

topbar-caption-cb = 11M/CB

topbar-caption-experimental = 实验性（未验证）

topbar-caption-receive-modes = 接收模式

topbar-wav-hint = 相同的音频，以未压缩的48 kHz 16位PCM保存：立体声约每分钟11 MB。适合用于编辑器或您自己的解码器。

topbar-mode-offset-hint-side = 此模式的信号在频率{ $side }。数字是频率，因为您调谐的是它；信号位于 { $mhz } MHz。

levelslider-hover =
    瀑布图电平
    底限 { $floor } dB · 上限 { $ceil } dB

    拖动或滚动可调亮或调暗；按住Shift调节对比度。
    移动它会关闭自动适配（FIT）。

smeter-audio-level-hover =
    音频电平：{ $dbfs } dBFS{ $over }。

    这台电台**没有自己的S表可供读取**，所以这是它送出的音频电平：在此测量，已包含电台自身的AGC和静噪的作用。它跟随的是音频，不是射频：声卡上的噪声、电台附近的无线鼠标或AGC的抽吸都会使它变动，而信号并没有变化。

smeter-audio-over = （音频刻度上的S9 + { $over } dB）

spectrum-offset-left = ◀ 音调 { $hz } Hz

spectrum-offset-right = { $hz } Hz 音调 ▶

worldmap-no-snr = 无SNR

solar-tune-the-radio-to-this-downlink = 将电台调谐到此下行频率

qo100-width-label = 宽度

qo100-park-label = 停放

qo100-led-carrier = 载波

qo100-led-sync = 同步

scanner-stopped = 已停止

ism-band-label = 波段

amp-none = 无

colormap-classic = 经典

colormap-viridis = Viridis

colormap-gray = 灰色

colormap-icom = Icom

colormap-neon = 霓虹

colormap-synthwave = Synthwave

colormap-matrix = 黑客帝国

colormap-tron = Tron

colormap-amber = 琥珀

colormap-rainbow = 彩虹

colormap-blue = 蓝色

colormap-calmer = 柔和

colormap-compline = Compline

colormap-lauds = Lauds

settings-cb-wide-calls-hint =
    除WSJT-CB的 { $narrow } 之外，同时接受更宽的11 m呼号形式 { $wide }：中间三个字母，后面最多四位数字。实验性：WSJT-CB自身拒绝这种较宽的形式，因此需要它的呼号可能不被WSJT-CB电台理解；它扩大的是本电台能听到的范围，而不是能可靠发送的范围。关闭则保持WSJT-CB的严格语法。

    此开关改变的是*形式*，而不是*长度*：11 m呼号包括任何 /P、/MM 或 /QRP 后缀在内仍为11个字符，因此 19DCG3733/P 可以，19TST1001/QRP 则不行。

settings-idle-stop-disable = 禁用

settings-xvtr-tx-off-hint =
    变频期间关闭：仅接收，此行是变频器（converter）而不是收发变频器（transverter），此波段上不会键控任何发射。

    通过同一变频器：双向工作，发射使用相同的偏移。

settings-xvtr-drive-hint =
    选中此收发变频器时对发射驱动的上限，以满值的百分比表示。收发变频器的中频输入只能承受毫瓦级功率，适合电台自身波段的驱动会将其损坏，因此这是此行最重要的字段。100%表示不限制。

    您的驱动设置保持在此值之下而不被改动，因此当频率离开该波段时，您在HF上使用的数值会恢复。

settings-brown-build-note = sdroxide的Brown版本：上游程序，外加11 m民用波段、短波收听和解码。作为普通业余电台运行，并增加了这些波段和工具。

settings-gain-by-band = 按波段记住前端增益

settings-gain-by-band-hint =
    为每个波段分别保存接收机增益，并在频率进入该波段时恢复。HF增益设置一次，11 m或VHF增益设置一次；电台会跟随波段。

    默认关闭。关闭时，一个增益适用于所有波段，与以前完全相同。开启时会记录您当前所在的波段，因此不会有波段没有增益；从未设置过的波段沿用当前增益。

settings-callsign-grid-note = 您的呼号和网格，由FT8/FT4/FT2、SSTV图像头、CB数字模式和日志本共用。这是**发射**时使用的身份：通联11 m时用CB呼号，否则用业余呼号。**接收报告**的签名使用什么，请见“频谱”标签页上的**报告身份（SWL）**框。也可以在FT8 / SSTV设置对话框中编辑。

settings-cb-grammar = 11 m / WSJT-CB呼号语法

settings-cb-wide-calls = 宽CB呼号：实验性

settings-cb-wide-calls-note = 如果您的11 m社群使用三字母组或四位数字的单位编号，请打开。适用于CB数字模式的解码和发射。无论如何，11 m呼号包括后缀在内不得超过**11个字符**：19DC3733/P 可以，19TST1001/QRP 不行。

settings-cb-plan-hint = 频率读数所依据的信道方案：11 m民用波段方案和446 MHz的PMR446方案。只有信道编号和波段打开时所在的信道会变化，波段边界保持不变，因此切换绝不会改变接收或发射的范围。只要频率位于方案的某个信道上，信道编号就会绘制在调谐线上。

settings-cb-allow-tx = 允许在11 m（CB）发射

settings-cb-allow-tx-hint = 业余波段锁定会拒绝在所有非业余划分的波段上发射，11 m就是其中之一：它是独立的无线电业务，有自己的规则和经型号认证的设备。打开此项即允许在该波段发射，且仅限该波段；广播波段仍保持仅接收。首次使用时会要求您确认。

settings-cb-licence-note = CB在许多国家无需执照，但并非所有国家，且允许的信道、模式和功率各不相同。使用该波段须遵守您所在国家的规定。

settings-radio-callsign-hint =
    此电台发射、记录和报点时使用的呼号。留空则使用“常规”标签页上的电台呼号：这是默认设置，适合只有一个身份的电台。

    在属于“另一个人”的电台上设置：11 m电台上用CB呼号，HF电台上用业余呼号，各自保持自己的呼号。“常规”标签页的呼号仍是所有未覆盖它的电台的后备。

    接收*报告*仍可使用“频谱”标签页中的SWL编号签名，此项不会改变它。

    点击应用后生效。

settings-radio-swap-iq-own = 此接口在本页下方的相应部分有自己的I/Q反转：请使用那一个。

settings-radio-swap-iq-hint =
    在解调之前反转此接收机的I/Q。当接收机的I和Q接反，导致SSB出现在错误的边带上（LSB听成USB），且频谱以频率为中心镜像时使用。

    这是硬件的属性，与登录的用户无关。

    点击应用后生效。

settings-radio-transmit-controls = 发射控制

settings-radio-hide-tx-all = 全部隐藏（SWL模式）

settings-radio-hide-tx-hint =
    隐藏*此*电台的所有发射控件：PTT、呼叫CQ、发射电平、发送、信标，全部隐藏，并把操作条上的业余扩展（报点、奖状）换成收听者的扩展（节目表和SWL日志，即收听日志）。按电台设置：接长线的收听用接收器可以处于此模式，而旁边的收发信机仍保留其发射机。

    硬件仍然可以发射，这只是隐藏控件。`--swl` 会在本次运行中对所有电台强制开启。

    点击应用后生效。

settings-radio-auto-mode = 自动模式

settings-radio-auto-stop-label = 无输入后停止

settings-radio-auto-stop-hint-long =
    无人值守的FT8/FT4/FT2序列器在没有操作员输入的情况下最长可运行多久，之后自动解除待命。按电台设置：一台可以比另一台运行更久。

    在PlutoSDR上最长可达六小时，禁用则完全关闭停止功能：自动模式将一直运行到您停止它。发射看门狗和波段锁定仍然有效。

    点击应用后生效。

settings-radio-auto-stop-hint =
    无人值守的FT8/FT4/FT2序列器在没有操作员输入的情况下最长可运行多久，之后自动解除待命。按电台设置：一台可以比另一台运行更久。

    特意限制在45分钟以内：自动模式是为上个洗手间准备的，不是让电台无人看守地去打比赛。

    点击应用后生效。

settings-radio-smeter-calib = S表校准

settings-tx-eq = 发射均衡器

settings-tx-eq-hint = 作用于话筒音频的3段参量均衡器，位于调制器之前。仅限语音模式（SSB/AM/FM）；数字模式和CW使用合成或键控的音频，不会经过它。默认关闭，开启时为平直，因此在下方某个频段真正被调动之前，开启它不会改变空中的任何东西。立即生效。

settings-tx-eq-low-hint = 削减或提升隆隆声和手持噪声。

settings-tx-eq-mid = 中频峰

settings-tx-eq-mid-hint = 临场感：在语音频带某处的窄峰或窄谷。

settings-tx-eq-high-hint = 明亮度/去齿音。

settings-cb-call-too-long = ⚠ 对11 m来说太长：{ $len } 个字符，而FT8对CB呼号只能承载11个，*包括*任何 /P、/MM 或 /QRP 后缀。此激活呼号无法发送。19DC3733/P（10）可以；19TST1001/QRP（13）不行。

settings-cb-plan-note = 11 m（CB）在各地并不是同一个波段。此项选择11 m频率读数所依据的国家信道：{ $plan }。只有信道和波段打开时所在的信道会变，波段边界保持放宽，因此切换方案绝不会改变接收或发射的范围。适用于本电台的所有电台设备。

settings-cw-enter-mode = 进入一次CW（或任何数字模式）以载入已保存的数值。

settings-cw-where-from = 来源：

settings-cw-straight-key-hint = 当CW面板的KEY开启时，会读取“CW直键”绑定（默认为空格键）。只有一个触点，因此是直键：应用会解码操作员自己的节奏。

settings-cw-speed = 速度：

settings-cw-speed-note = 电键的点和划的长度

settings-cw-key-tx = 键控发射机

settings-cw-key-tx-hint = 关：按键只驱动本地练习器。开：通过普通的手动键控路径键控电台，并受波段锁定和看门狗的约束。

settings-cw-key-self-note = 对于自己键控的电台，请将设置 → 电台 → CW键控设为`声卡（MCW）`：设计上，在电台电键路径上不允许手动键控。

settings-detachable-windows = 可分离窗口

settings-detachable-windows-note = 将面板拉出为独立的系统窗口。关闭窗口即可重新停靠。

settings-tool-windows = 工具窗口

settings-tool-windows-note = 已转换为浮动窗口的面板。其打开状态会被记住。

settings-tool-window-scanner = 扫描器

settings-tool-window-memories = 记忆

settings-tool-window-playback = 回放

settings-tool-window-open = 打开

settings-tool-window-closed = 关闭

settings-detachable-native-only = 可分离窗口仅限本机版本。

settings-psk-reporter-band-hint = 波段活动馈送：当前波段上被听到的电台，以彩色圆点绘制。同时启用地图“听到我”标签背后的“谁听到了我”查询，每小时也会重新获取。

settings-psk-upload-hint = 将本电台听到的内容报告给pskreporter.info，使其作为接收站出现在那里。使用“常规”标签页的呼号和网格，或在设置了下方SWL编号时使用该编号。

settings-swl-report-as = 报告身份（SWL）

settings-swl-report-as-hint = 收听者用于接收报告的自身身份：注册的SWL编号、俱乐部编号或姓名。设置后，它会取代呼号，为本电台*听到*的内容签名（PSK Reporter和WSPRnet上传，以及SWL日志导出的接收报告）。留空则以呼号报告。它绝不会被发射，绝不会键控电台，也绝不会作为QSO或报点者的名称，因此没有呼号的收听者也可以报告。

settings-wsjtcb-spot-server = WSJT-CB报点服务器

settings-wsjtcb-report = 将我的11 m解码报告到WSJT-CB报点服务器

settings-wsjtcb-report-hint = 将11 m波段上解码出的每个报点发送到CB社群的报点服务（xzgroup.net），这与WSJT-CB自己的客户端可以发布的服务器相同。默认关闭：这是第三方服务，您不要求就不会发送任何内容。您的呼号即为报点者；PSK Reporter是上方的另一个独立开关。

settings-eqsl-qth-hint = 上传归入哪个QTH配置：仅当账户拥有多个时才需要，否则留空。

settings-log11dx-api-url = API地址

settings-log11dx-api-token = API令牌

settings-log11dx-note = 两者都来自您的LOG11DX个人资料页面。不需要它自己的WSJT-X桥接：sdroxide记录QSO并直接提交到那里。该网站会遮蔽令牌，因此生成时请复制下来：它和其他凭据一样存储在net.json中。

settings-pan-attached-note =
    它的频谱、瀑布图以及由它们读取的所有内容都在该电台的标签页上，所以它自己没有这些。上方的接口设置仍属于此电台并继续有效：更改后请按应用 / 重新连接，两台电台将一起重新启动。

    若要让它重新独立使用，请清除该电台页面上的全景频谱接收机框。

settings-pan-source-hint =
    由哪台电台为这一台接收。借出期间它会从标签栏消失，因为其前端属于这台电台的引擎；设为“无”后它会立刻回来。

    点击应用后生效。

settings-pan-offset-hint =
    接收机位于此电台频率之上多少，单位为赫兹。与此电台共用同一天线的接收机填0；中频抽头则填中频：9 MHz中频填9000000，70.455 MHz中频填70455000。

    这不是上方的变频器字段（它会重新调谐电台），也不是CAT标签页的I/Q中心偏移（它与此电台自身的声卡有关）。在此输入的任何内容都不会发送给任何一台电台。

    点击应用后生效。

settings-cw-device = 设备：

settings-cw-device-none = （未找到）

settings-cw-device-auto = 自动（第一个名为“key”的）

settings-cw-reverse-hint = 交换点和划，适用于拍键自身上的开关。

settings-cw-key-type = 电键类型：

settings-cw-key-straight = 直键

settings-cw-key-iambic-a = IAMBIC A

settings-cw-key-iambic-b = IAMBIC B

settings-cw-usb-note = 能报告其两个触点（常表现为鼠标按键）的电键盒可以使用；自带iambic功能并发送成品触发的则不行。电键处于待命状态时，设备被独占，因此其触点不会在其他窗口中产生点击。

settings-cw-usb-linux = USB拍键需要Linux桌面。

settings-smeter-zero = 0（按测量值）

settings-smeter-cal-hint =
    当此电台的S表读数偏高或偏低时对其进行校正，单位为S单位，每个6 dB。

    负值会降低读数：适用于显示过高的表，例如本应是S2的底噪显示为S5。正值则抬高读数。0即为测量所得的表读数。

    只有读数会变动：指针、S和dBm数值、接收报告的起始电平以及语音报S表。信号、静噪、录音和解码器不受影响。

    与此电台一起保存在它所连接的机器上，立即生效。

settings-rbn-n = Reverse Beacon Network { $n }

settings-rbn-same-feed = Reverse Beacon Network 1和2都使用 { $host }:{ $port }：每个报点都会到达两次。请给它们不同的端口（7000用于CW/RTTY，7001用于FT8/FT4），或关闭其中一个。

settings-rx-site-station = 在电台处

settings-rx-site-elsewhere = 在别处

settings-remote-tab-note =
    该电台会作为一个标签页出现在此处已打开的电台旁边，窗口顶部的标签栏可以在它们之间切换（⊞可以并排显示两个）。从电台标签页顶部的名单中关闭它：这只是挂断，服务器上不会有任何改动。

    如果服务器要求用户名和密码，其登录界面会在新标签页中出现。普通的ws://链接不会对这些内容加密，所以经由开放的互联网时，请使用VPN或SSH隧道，或者在服务器前端终结HTTPS，并勾选上方的安全。

settings-cw-source-keyboard = 键盘

settings-cw-source-usb = USB拍键

pane-frame = 帧

pane-frames = 帧

pane-words = 词

antenna-feed-z0 = 馈电Z0：

antenna-feed-z0-hint = 用于SWR的馈线/系统阻抗（通常为50 Ω）

antenna-height-label = 高度：

antenna-height-hint = 馈电点/天线距地面的高度（影响仰角/方向图）

antenna-m-agl = m（离地）

antenna-apply = 应用

antenna-apply-hint = 对预设运行矩量法求解器。

antenna-yagi-optimizer = 八木优化器

antenna-stack = 叠层：

antenna-objective = 目标：

antenna-optimize = 优化

antenna-swr-value = SWR { $v }

antenna-swr-ref = SWR { $v }（{ $z }Ω）

antenna-radiation-pattern = 辐射方向图

antenna-view = 视图

antenna-view-front = 正面

antenna-view-side = 侧面

antenna-view-top = 顶面

antenna-3d-hint = 3D：拖动旋转 • 热度 = 场强

antenna-yagi-x = 八木 ×{ $n }

settings-tool-window-session-qsos = 本次QSO

sessionqso-time = 时间

sessionqso-call = 呼号

sessionqso-grid = 网格

sessionqso-band = 波段

sessionqso-mode = 模式

sessionqso-rst-s = RST 发

sessionqso-rst-r = RST 收

sessionqso-country = 国家

sessionqso-dist = 距离

sessionqso-final = 最终

sessionqso-count = 本次共 { $n } 个QSO

sessionqso-include-cw = 包含CW

decodes-live-log = 实时日志

decodes-setup = ⚙ 设置

schedule-colour-all = 颜色

schedule-colour-all-hint = 所有电台的瀑布图标签颜色：取代为各电台单独选的颜色

schedule-colour-station-hint = 此电台的瀑布图标签颜色

schedule-colour-reset = 默认

schedule-colour-reset-hint = 所有电台恢复为默认颜色，并清除已单独选的颜色

settings-language = 语言

settings-language-note = 界面使用的语言。系统默认跟随操作系统；选择其他语言可覆盖它。所作选择会被保存并记住。

aprs-all-chip = 全部

aprs-all-chip-hint = 亮起：信道上听到的每条消息，不论发给谁。关闭：只显示发给您或来自您的消息，以及公告。

aprs-composer-hidden-swl = 发送已隐藏：SWL / 收听模式已开启。关闭它才能撰写和发送消息。

aprs-compose-count-hint = 已使用的字符数，APRS消息最多可承载67个。

tx-bands-title = 发射波段

tx-bands-note = 只在此电台确实可以发射的波段上把发射设为是。此项先于其他所有发射设置；选择是绝不会解除其他安全限制。

tx-bands-careful = ⚠ 选错波段可能损坏电台：请仔细检查每个选项。

tx-bands-yes = 是

tx-bands-no = 否

tx-bands-confirmed = ✓ 此电台的发射波段已确认

tx-bands-ack = 我已对照我的电台检查过这些波段

tx-bands-confirm = 确认这些波段

tx-bands-confirm-hint = 请先勾选方框：每台电台只需确认一次。

tx-bands-check = 检查波段…

tx-bands-notice-new = 所有波段的发射均已关闭。请选择此电台发射的波段，并确认一次。由于存在损坏电台的风险，请仔细检查您所选的选项。

tx-bands-notice-carried = 您的发射波段是从上一版本沿用的。在您检查并确认一次之前，发射仍被禁止。由于存在损坏电台的风险，请仔细检查每个选项。


adsbmap-surface-asphalt = 沥青

adsbmap-surface-concrete = 混凝土

adsbmap-surface-grass = 草地

adsbmap-surface-gravel = 砾石

adsbmap-surface-dirt = 泥土

adsbmap-surface-water = 水面

adsbmap-airport-runway-lit = 有灯光

adsbmap-airport-more = 另有 { $n } 个

adsbmap-airport-freq-head = 频率，MHz

adsbmap-airport-data-date = 跑道和频率数据截至 { $date }
aircraft-online-switch = 在线

aircraft-online-switch-on-hover = 在线：机型、运营商、航线和照片来自adsbdb和Planespotters，机场图片来自Wikipedia，机场的本地时间和气温来自Open-Meteo（只发送机场的位置），由您的电台代您请求。点击关闭；电台会记住您的选择。除此之外没有任何内容离开电台。

aircraft-online-switch-off-hover = 离线：没有任何内容离开电台。点击开启机型、运营商、航线和照片（adsbdb和Planespotters）、机场图片（Wikipedia）以及机场天气（Open-Meteo）。

aircraft-switch-held = 电台的操作员通过SDROXIDE_AIRCRAFT_ONLINE控制此项

adsbmap-airport-photo-credit = 照片：Wikipedia

adsbmap-airport-local-time = 当地时间 { $time }（{ $zone }）

adsbmap-airport-temperature = 气温 { $temp }

adsbmap-airport-weather-credit = 天气：Open-Meteo.com

adsbmap-airport-utc-wait = UTC { $time }（当地时间：加载中…）

adsbmap-airport-utc-offline = UTC { $time }（当地时间：离线）

schedule-listen = 收听

schedule-listen-hint = 将接收机调谐到此电台

schedule-sort = 排序

schedule-sort-asc = 升序

schedule-sort-desc = 降序

schedule-col-freq = 频率

schedule-col-band = 波段

schedule-col-name = 名称

schedule-col-lang = 语言

schedule-col-target = 目标

schedule-col-site = 发射站

schedule-col-local = 本地

tx-bands-off = { $band } 上的发射已关闭：您的设置不允许此波段，因此您的发射机不适合在此波段上发射。（每次会话最多显示 { $max } 次。）

tx-off-band = 禁止发射 — { $freq } MHz（{ $band }）不是已确认的发射频段。请在 设置 → 电台 → 发射频段 中勾选并确认。

tx-off-unconfirmed = 禁止发射 — 此电台的发射频段尚未确认。请在 设置 → 电台 → 发射频段 中检查并确认。

aircraft-hover-lookup-retry = 无响应，{ $s } 秒后重试

adsbmap-airport-navaids-head = 机场内的无线电导航设施

adsbmap-airport-no-freq = 该机场未公布频率

topbar-preset-right-click = 右键单击可修改此预设

topbar-preset-edit = 编辑预设

topbar-preset-label = 标签

topbar-preset-label-hint = 按钮上显示的文字，最多 8 个字符。

topbar-preset-lo = 下限

topbar-preset-hi = 上限

topbar-preset-use-current = 使用当前值

topbar-preset-use-current-hint = 采用接收机当前滤波器的边沿

topbar-preset-save = 保存

topbar-preset-reset = 重置

topbar-preset-reset-hint = 恢复此模式的内置预设

topbar-preset-cancel = 取消
wefax-schedule = 时刻表
wefax-schedule-hint = 在浏览器中打开 { $station } 发布的时刻表，含图卡与 UTC 时间。

tx-bands-check-again = 重新确认

tx-bands-check-again-hint = 撤回确认以便重新检查列表。更改任何频段都会自动这样做。
