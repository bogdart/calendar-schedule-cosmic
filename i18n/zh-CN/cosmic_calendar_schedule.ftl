app-title = 日历与日程
no-meetings = 没有即将到来的会议
no-meetings-panel = 无会议
no-meetings-day = 无事件
loading-meetings = 正在加载会议...
settings = 设置
open-calendar = 日历
back = 返回
next-meeting = 下一个会议
upcoming = 即将到来
calendars-section = 日历
calendars-enabled = 已启用 { $enabled }/{ $total }
calendars-none = 未配置
display-format-section = 下一个会议时间
display-format-day-time = 日期和时间
display-format-relative = 相对时间
upcoming-events-section = 显示更多会议
join = 加入
join-button-section = 加入按钮
join-button-visibility = 可见性
join-button-description = 启用后，检测到视频通话链接的会议将显示"加入"按钮。
formatting-section = 格式化
url-patterns-description = 用于在会议描述和位置中检测视频通话链接的正则表达式模式。这些模式适用于面板和下拉菜单中的加入按钮。
panel-join-button = 面板中
popup-join-button = 下拉菜单中
join-hide = 隐藏
join-show = 始终显示
join-show-same-day = 当天显示
join-show-30m = 30分钟内显示
join-show-15m = 15分钟内显示
join-show-5m = 5分钟内显示
url-patterns = URL 模式
add-pattern = 添加模式
location-section = 实际地点
location-description = 启用后，如果提供了会议的实际地点（如会议室名称），将会显示。
panel-location = 面板中
popup-location = 下拉菜单中
location-hide = 隐藏
location-show = 始终显示（如果提供）
location-show-same-day = 当天显示
location-show-30m = 30分钟内显示
location-show-15m = 15分钟内显示
location-show-5m = 5分钟内显示
status-off = 关闭
status-panel = 面板中
status-popup = 下拉菜单中
status-both = 面板和下拉菜单中
time-now = 现在
time-in-days-hours = { $days }天{ $hours }小时后
time-in-days = { $days }天后
time-in-hours-minutes = { $hours }小时{ $minutes }分钟后
time-in-hours = { $hours }小时后
time-in-minutes = { $minutes }分钟后
panel-time-location = （{ $time }，{ $location }）
panel-time = （{ $time }）
calendar-indicator-section = 指示来源日历
calendar-indicator-description = 启用后，将显示一个彩色圆点，指示会议来自哪个日历。
panel-indicator = 面板中
popup-indicator = 下拉菜单中
filter-events-section = 筛选事件
show-all-day-events = 显示全天事件
in-progress-section = 显示进行中的会议
in-progress-off = 关闭
in-progress-5m = 开始后<5分钟
in-progress-10m = 开始后<10分钟
in-progress-15m = 开始后<15分钟
in-progress-30m = 开始后<30分钟
panel-started = 已开始
time-until-section = 按时间筛选
time-until-all = 所有事件
time-until-3h = 3小时内
time-until-6h = 6小时内
time-until-same-day = 当天
time-until-1d = 1天内
time-until-2d = 2天内
status-filter-section = 按状态筛选
status-filter-all = 所有事件
status-filter-accepted = 仅已接受
status-filter-accepted-tentative = 已接受和暂定
filter-summary-all = 显示所有事件
filter-summary-no-all-day = 不含全天事件
filter-summary-accepted = 已接受
filter-summary-tentative = 已接受和暂定
filter-summary-combo = { $allday }，{ $status }
additional-emails-section = 我的电子邮箱地址
additional-emails-description = 添加您用于日历邀请的电子邮箱地址（如别名、工作邮箱）。这些用于确定您的会议接受状态，作为为每个日历配置的邮箱的补充。
additional-emails-summary = { $count ->
    [0] 未配置
    [one] { $count } 个邮箱
    *[other] { $count } 个邮箱
}
add-email = 添加邮箱
filter-events-description = 根据事件属性或您的参与状态筛选显示的日历事件。
panel-display = 面板
dropdown-display = 下拉菜单
configure = 配置...
about = 关于
version = 版本 { $version }
author = 作者 { $author }
website = 网站
report-bug = 报告错误
no-calendars = 无日历
no-calendars-description = 使用 GNOME Online Accounts 或 Evolution 等日历应用配置日历账户。
refresh-section = 强制远程同步
refresh-now = 手动强制同步
refreshing = 同步中...
auto-refresh = 自动强制远程同步
refresh-interval = 间隔
refresh-interval-5m = 每5分钟
refresh-interval-10m = 每10分钟
refresh-interval-15m = 每15分钟
refresh-interval-30m = 每30分钟
refresh-summary-on = 每{ $interval }分钟
refresh-summary-off = 仅手动
refresh-description =
    此小程序从 Evolution Data Server（EDS）读取缓存的日历数据，EDS 可以与在线日历同步。Evolution 和 GNOME Calendar 等日历应用会触发 EDS 同步。

    如果启用此设置，小程序也会自动触发 EDS 与云端同步。如果您使用其他解决方案，或仅使用本地日历，这可能是多余的。
updated-unknown = 更新时间：未知
updated-just-now = 刚刚更新
updated-minutes-ago = { $minutes }分钟前更新
updated-hours-ago = { $hours }小时前更新
updated-days-ago = { $days }天前更新
vertical-panel-notice = 此小程序在水平面板中效果最佳，可以内联显示会议信息。
calendars-setup-tip = 请记得在 GNOME Calendar/Online Accounts 中设置您要使用的在线日历。
keyboard-shortcut = 键盘快捷键
keyboard-shortcut-description = 您可以设置系统键盘快捷键来即时加入下一个会议。
keyboard-shortcut-instructions = 打开 COSMIC Settings → 键盘 → 自定义快捷键，然后使用以下命令添加新快捷键：
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = 复制
keyboard-shortcut-open-settings = 打开设置

# 时间显示设置
datetime-settings = 日期与时间设置
time-display-section = 时间显示
military-time = 24小时制
show-seconds = 显示秒
show-weekday = 显示星期
first-day-of-week = 每周第一天
first-day-sunday = 星期日
first-day-monday = 星期一
custom-format = 自定义格式
custom-format-description = 使用 strftime 格式字符串（留空使用默认格式）

# 日历导航
previous-month = 上个月
next-month = 下个月
today = 今天

# 面板显示设置
panel-show-weekday = 显示星期
panel-show-year = 显示年份

# 天气
weather-section = 天气
weather-not-configured = 未配置
weather-search-placeholder = 搜索城市...
weather-clear = 移除城市
weather-current = 当前天气

# 世界时钟
world-clock-section = 世界时钟
world-clock-not-configured = 未配置
world-clock-add = 添加城市
world-clock-search-placeholder = 搜索城市...
world-clock-cities-count = { $count ->
    [one] { $count } 个城市
    *[other] { $count } 个城市
}
