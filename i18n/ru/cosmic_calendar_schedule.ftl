app-title = Календарь и расписание
no-meetings = Нет предстоящих встреч
no-meetings-panel = Нет встреч
no-meetings-day = Нет событий
loading-meetings = Загрузка встреч...
settings = Настройки
open-calendar = Календарь
back = Назад
next-meeting = Следующая встреча
upcoming = Предстоящие
calendars-section = Календари
calendars-enabled = { $enabled }/{ $total } включено
calendars-none = Не настроено
display-format-section = Время следующей встречи
display-format-day-time = День и время
display-format-relative = Относительное время
upcoming-events-section = Показать дополнительные встречи
join = Присоединиться
join-button-section = Кнопка «Присоединиться»
join-button-visibility = Видимость
join-button-description = При включении для встреч с обнаруженным URL видеозвонка будет отображаться кнопка «Присоединиться».
formatting-section = Форматирование
url-patterns-description = Регулярные выражения для обнаружения URL видеозвонков в описаниях и местах встреч. Эти шаблоны применяются к кнопке «Присоединиться» как на панели, так и в выпадающем меню.
panel-join-button = На панели
popup-join-button = В выпадающем меню
join-hide = Скрыть
join-show = Показывать всегда
join-show-same-day = Показывать в тот же день
join-show-30m = Показывать за 30 мин
join-show-15m = Показывать за 15 мин
join-show-5m = Показывать за 5 мин
url-patterns = Шаблоны URL
add-pattern = Добавить шаблон
location-section = Физическое местоположение
location-description = При включении будет отображаться физическое местоположение встречи (например, название комнаты), если оно указано.
panel-location = На панели
popup-location = В выпадающем меню
location-hide = Скрыть
location-show = Показывать всегда (если указано)
location-show-same-day = Показывать в тот же день
location-show-30m = Показывать за 30 мин
location-show-15m = Показывать за 15 мин
location-show-5m = Показывать за 5 мин
status-off = Выкл
status-panel = На панели
status-popup = В выпадающем меню
status-both = На панели и в меню
time-now = сейчас
time-in-days-hours = через { $days }д { $hours }ч
time-in-days = через { $days }д
time-in-hours-minutes = через { $hours }ч { $minutes }мин
time-in-hours = через { $hours }ч
time-in-minutes = через { $minutes }мин
panel-time-location = ({ $time } в { $location })
panel-time = ({ $time })
calendar-indicator-section = Индикатор исходного календаря
calendar-indicator-description = При включении будет отображаться цветная точка, указывающая, из какого календаря взята встреча.
panel-indicator = На панели
popup-indicator = В выпадающем меню
filter-events-section = Фильтр событий
show-all-day-events = Показывать события на весь день
in-progress-section = Показывать текущие встречи
in-progress-off = Выкл
in-progress-5m = <5 мин после начала
in-progress-10m = <10 мин после начала
in-progress-15m = <15 мин после начала
in-progress-30m = <30 мин после начала
panel-started = началась
time-until-section = Фильтр по времени
time-until-all = Все события
time-until-3h = В течение 3 часов
time-until-6h = В течение 6 часов
time-until-same-day = Сегодня
time-until-1d = В течение 1 дня
time-until-2d = В течение 2 дней
status-filter-section = Фильтр по статусу
status-filter-all = Все события
status-filter-accepted = Только принятые
status-filter-accepted-tentative = Принятые и предварительные
filter-summary-all = Показать все события
filter-summary-no-all-day = Без событий на весь день
filter-summary-accepted = Принятые
filter-summary-tentative = Принятые и предварительные
filter-summary-combo = { $allday }, { $status }
additional-emails-section = Мои адреса электронной почты
additional-emails-description = Добавьте адреса электронной почты, используемые для приглашений календаря (например, псевдонимы, рабочая почта). Они используются для определения вашего статуса принятия встреч в дополнение к почте, настроенной для каждого календаря.
additional-emails-summary = { $count ->
    [0] Не настроено
    [one] { $count } адрес
    [few] { $count } адреса
    *[other] { $count } адресов
}
add-email = Добавить почту
filter-events-description = Фильтруйте отображаемые события календаря по свойствам события или статусу участия.
panel-display = Панель
dropdown-display = Выпадающее меню
configure = Настроить...
about = О приложении
version = Версия { $version }
author = автор { $author }
website = Веб-сайт
report-bug = Сообщить об ошибке
no-calendars = Нет календарей
no-calendars-description = Настройте учётную запись календаря с помощью GNOME Online Accounts или приложения-календаря, например Evolution.
refresh-section = Принудительная удалённая синхронизация
refresh-now = Принудительная синхронизация вручную
refreshing = Синхронизация...
auto-refresh = Автоматическая принудительная синхронизация
refresh-interval = Интервал
refresh-interval-5m = Каждые 5 минут
refresh-interval-10m = Каждые 10 минут
refresh-interval-15m = Каждые 15 минут
refresh-interval-30m = Каждые 30 минут
refresh-summary-on = Каждые { $interval } мин
refresh-summary-off = Только вручную
refresh-description =
    Этот апплет читает кэшированные данные календаря из Evolution Data Server (EDS), который может синхронизироваться с онлайн-календарями. Приложения-календари, такие как Evolution и GNOME Calendar, запускают синхронизацию EDS.

    Если эта настройка включена, апплет также автоматически запускает синхронизацию EDS с облаком. Если вы используете другое решение или только локальный календарь, это может быть избыточным.
updated-unknown = Обновлено: неизвестно
updated-just-now = Обновлено только что
updated-minutes-ago = Обновлено { $minutes } мин назад
updated-hours-ago = Обновлено { $hours } ч назад
updated-days-ago = Обновлено { $days } д назад
vertical-panel-notice = Этот апплет лучше работает на горизонтальной панели, где информация о встрече может отображаться в строке.
calendars-setup-tip = Не забудьте настроить онлайн-календари, которые вы хотите использовать, в GNOME Calendar/Online Accounts.
keyboard-shortcut = Сочетание клавиш
keyboard-shortcut-description = Вы можете настроить системное сочетание клавиш для мгновенного присоединения к следующей встрече.
keyboard-shortcut-instructions = Откройте COSMIC Settings → Клавиатура → Пользовательские сочетания клавиш и добавьте новое сочетание со следующей командой:
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = Копировать
keyboard-shortcut-open-settings = Открыть настройки

# Настройки отображения времени
datetime-settings = Настройки даты и времени
time-display-section = Отображение времени
military-time = 24-часовой формат
show-seconds = Показывать секунды
show-weekday = Показывать день недели
first-day-of-week = Первый день недели
first-day-sunday = Воскресенье
first-day-monday = Понедельник
custom-format = Пользовательский формат
custom-format-description = Используйте строку формата strftime (оставьте пустым для значения по умолчанию)

# Навигация по календарю
previous-month = Предыдущий месяц
next-month = Следующий месяц
today = Сегодня

# Настройки отображения панели
panel-show-weekday = Показывать день недели
panel-show-year = Показывать год

# Погода
weather-section = Погода
weather-not-configured = Не настроено
weather-search-placeholder = Поиск города...
weather-clear = Удалить город
weather-current = Текущая погода

# Мировые часы
world-clock-section = Мировые часы
world-clock-not-configured = Не настроено
world-clock-add = Добавить город
world-clock-search-placeholder = Поиск города...
world-clock-cities-count = { $count ->
    [one] { $count } город
    [few] { $count } города
    *[other] { $count } городов
}
