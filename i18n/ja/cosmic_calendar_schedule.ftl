app-title = カレンダーとスケジュール
no-meetings = 予定されている会議はありません
no-meetings-panel = 会議なし
no-meetings-day = イベントなし
loading-meetings = 会議を読み込み中...
settings = 設定
open-calendar = カレンダー
back = 戻る
next-meeting = 次の会議
upcoming = 今後の予定
calendars-section = カレンダー
calendars-enabled = { $enabled }/{ $total } 有効
calendars-none = 未設定
display-format-section = 次の会議時刻
display-format-day-time = 日付と時刻
display-format-relative = 相対時間
upcoming-events-section = 追加の会議を表示
join = 参加
join-button-section = 参加ボタン
join-button-visibility = 表示設定
join-button-description = 有効にすると、ビデオ通話URLが検出された会議に「参加」ボタンが表示されます。
formatting-section = フォーマット
url-patterns-description = 会議の説明や場所からビデオ通話URLを検出するための正規表現パターン。これらのパターンはパネルとドロップダウンの参加ボタンに適用されます。
panel-join-button = パネル内
popup-join-button = ドロップダウン内
join-hide = 非表示
join-show = 常に表示
join-show-same-day = 当日のみ表示
join-show-30m = 30分以内に表示
join-show-15m = 15分以内に表示
join-show-5m = 5分以内に表示
url-patterns = URLパターン
add-pattern = パターンを追加
location-section = 場所
location-description = 有効にすると、会議の物理的な場所（会議室名など）が提供されている場合に表示されます。
panel-location = パネル内
popup-location = ドロップダウン内
location-hide = 非表示
location-show = 常に表示（提供されている場合）
location-show-same-day = 当日のみ表示
location-show-30m = 30分以内に表示
location-show-15m = 15分以内に表示
location-show-5m = 5分以内に表示
status-off = オフ
status-panel = パネル内
status-popup = ドロップダウン内
status-both = パネルとドロップダウン
time-now = 今
time-in-days-hours = { $days }日{ $hours }時間後
time-in-days = { $days }日後
time-in-hours-minutes = { $hours }時間{ $minutes }分後
time-in-hours = { $hours }時間後
time-in-minutes = { $minutes }分後
panel-time-location = （{ $time }、{ $location }）
panel-time = （{ $time }）
calendar-indicator-section = カレンダーの出所を表示
calendar-indicator-description = 有効にすると、会議がどのカレンダーからのものかを示す色付きの点が表示されます。
panel-indicator = パネル内
popup-indicator = ドロップダウン内
filter-events-section = イベントをフィルター
show-all-day-events = 終日イベントを表示
in-progress-section = 進行中の会議を表示
in-progress-off = オフ
in-progress-5m = 開始後5分未満
in-progress-10m = 開始後10分未満
in-progress-15m = 開始後15分未満
in-progress-30m = 開始後30分未満
panel-started = 開始済み
time-until-section = 時間でフィルター
time-until-all = すべてのイベント
time-until-3h = 3時間以内
time-until-6h = 6時間以内
time-until-same-day = 当日
time-until-1d = 1日以内
time-until-2d = 2日以内
status-filter-section = ステータスでフィルター
status-filter-all = すべてのイベント
status-filter-accepted = 承諾済みのみ
status-filter-accepted-tentative = 承諾済みと仮承諾
filter-summary-all = すべてのイベントを表示
filter-summary-no-all-day = 終日イベントなし
filter-summary-accepted = 承諾済み
filter-summary-tentative = 承諾済みと仮承諾
filter-summary-combo = { $allday }、{ $status }
additional-emails-section = メールアドレス
additional-emails-description = カレンダーの招待に使用するメールアドレスを追加します（エイリアス、仕事用メールなど）。各カレンダーに設定されたメールに加えて、会議への参加状態を判断するために使用されます。
additional-emails-summary = { $count ->
    [0] 未設定
    [one] { $count } 件のメール
    *[other] { $count } 件のメール
}
add-email = メールを追加
filter-events-description = イベントのプロパティや参加状態に基づいて、表示するカレンダーイベントをフィルターします。
panel-display = パネル
dropdown-display = ドロップダウン
configure = 設定...
about = このアプリについて
version = バージョン { $version }
author = 作成者 { $author }
website = ウェブサイト
report-bug = バグを報告
no-calendars = カレンダーなし
no-calendars-description = GNOME Online Accounts またはEvolutionなどのカレンダーアプリでカレンダーアカウントを設定してください。
refresh-section = リモート同期を強制
refresh-now = 手動で同期を強制
refreshing = 同期中...
auto-refresh = 自動リモート同期を強制
refresh-interval = 間隔
refresh-interval-5m = 5分ごと
refresh-interval-10m = 10分ごと
refresh-interval-15m = 15分ごと
refresh-interval-30m = 30分ごと
refresh-summary-on = { $interval }分ごと
refresh-summary-off = 手動のみ
refresh-description =
    このアプレットはEvolution Data Server（EDS）からキャッシュされたカレンダーデータを読み取ります。EDSはオンラインカレンダーと同期できます。EvolutionやGNOME CalendarなどのカレンダーアプリがEDSの同期をトリガーします。

    この設定が有効な場合、アプレットもEDSのクラウドとの同期を自動的にトリガーします。別のソリューションを使用している場合、またはローカルカレンダーのみを使用している場合、これは冗長になる可能性があります。
updated-unknown = 更新日時：不明
updated-just-now = たった今更新
updated-minutes-ago = { $minutes }分前に更新
updated-hours-ago = { $hours }時間前に更新
updated-days-ago = { $days }日前に更新
vertical-panel-notice = このアプレットは、会議情報をインラインで表示できる水平パネルで最適に動作します。
calendars-setup-tip = 使用したいオンラインカレンダーをGNOME Calendar/Online Accountsで設定することを忘れないでください。
keyboard-shortcut = キーボードショートカット
keyboard-shortcut-description = システムキーボードショートカットを設定して、次の会議に即座に参加できます。
keyboard-shortcut-instructions = COSMIC Settings → キーボード → カスタムショートカットを開き、以下のコマンドで新しいショートカットを追加します：
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = コピー
keyboard-shortcut-open-settings = 設定を開く

# 時刻表示設定
datetime-settings = 日付と時刻の設定
time-display-section = 時刻表示
military-time = 24時間表示
show-seconds = 秒を表示
show-weekday = 曜日を表示
first-day-of-week = 週の始まり
first-day-sunday = 日曜日
first-day-monday = 月曜日
custom-format = カスタムフォーマット
custom-format-description = strftime形式の文字列を使用（空白でデフォルト）

# カレンダーナビゲーション
previous-month = 前月
next-month = 翌月
today = 今日

# パネル表示設定
panel-show-weekday = 曜日を表示
panel-show-year = 年を表示

# 天気
weather-section = 天気
weather-not-configured = 未設定
weather-search-placeholder = 都市を検索...
weather-clear = 都市を削除
weather-current = 現在の天気

# 世界時計
world-clock-section = 世界時計
world-clock-not-configured = 未設定
world-clock-add = 都市を追加
world-clock-search-placeholder = 都市を検索...
world-clock-cities-count = { $count ->
    [one] { $count } 都市
    *[other] { $count } 都市
}
