app-title = Календар і розклад
no-meetings = Немає запланованих зустрічей
no-meetings-panel = Немає зустрічей
no-meetings-day = Немає подій
loading-meetings = Завантаження зустрічей...
settings = Налаштування
open-calendar = Календар
back = Назад
next-meeting = Наступна зустріч
upcoming = Заплановані
calendars-section = Календарі
calendars-enabled = { $enabled }/{ $total } увімкнено
calendars-none = Не налаштовано
display-format-section = Час наступної зустрічі
display-format-day-time = День і час
display-format-relative = Відносний час
upcoming-events-section = Показати додаткові зустрічі
join = Приєднатися
join-button-section = Кнопка «Приєднатися»
join-button-visibility = Видимість
join-button-description = Якщо увімкнено, для зустрічей із виявленим URL відеодзвінка з'явиться кнопка «Приєднатися».
formatting-section = Форматування
url-patterns-description = Регулярні вирази для виявлення URL відеодзвінків в описах та місцях зустрічей. Ці шаблони застосовуються до кнопки «Приєднатися» як на панелі, так і у випадному меню.
panel-join-button = На панелі
popup-join-button = У випадному меню
join-hide = Приховати
join-show = Показувати завжди
join-show-same-day = Показувати в той самий день
join-show-30m = Показувати за 30 хв
join-show-15m = Показувати за 15 хв
join-show-5m = Показувати за 5 хв
url-patterns = Шаблони URL
add-pattern = Додати шаблон
location-section = Фізичне місцезнаходження
location-description = Якщо увімкнено, буде показано фізичне місцезнаходження зустрічі (наприклад, назва кімнати), якщо воно вказане.
panel-location = На панелі
popup-location = У випадному меню
location-hide = Приховати
location-show = Показувати завжди (якщо вказано)
location-show-same-day = Показувати в той самий день
location-show-30m = Показувати за 30 хв
location-show-15m = Показувати за 15 хв
location-show-5m = Показувати за 5 хв
status-off = Вимк
status-panel = На панелі
status-popup = У випадному меню
status-both = На панелі та в меню
time-now = зараз
time-in-days-hours = через { $days }д { $hours }год
time-in-days = через { $days }д
time-in-hours-minutes = через { $hours }год { $minutes }хв
time-in-hours = через { $hours }год
time-in-minutes = через { $minutes }хв
panel-time-location = ({ $time } у { $location })
panel-time = ({ $time })
calendar-indicator-section = Індикатор вихідного календаря
calendar-indicator-description = Якщо увімкнено, буде показано кольорову крапку, що вказує, з якого календаря зустріч.
panel-indicator = На панелі
popup-indicator = У випадному меню
filter-events-section = Фільтр подій
show-all-day-events = Показувати події на весь день
in-progress-section = Показувати поточні зустрічі
in-progress-off = Вимк
in-progress-5m = <5 хв після початку
in-progress-10m = <10 хв після початку
in-progress-15m = <15 хв після початку
in-progress-30m = <30 хв після початку
panel-started = розпочалася
time-until-section = Фільтр за часом
time-until-all = Усі події
time-until-3h = Протягом 3 годин
time-until-6h = Протягом 6 годин
time-until-same-day = Сьогодні
time-until-1d = Протягом 1 дня
time-until-2d = Протягом 2 днів
status-filter-section = Фільтр за статусом
status-filter-all = Усі події
status-filter-accepted = Лише прийняті
status-filter-accepted-tentative = Прийняті та попередні
filter-summary-all = Показати всі події
filter-summary-no-all-day = Без подій на весь день
filter-summary-accepted = Прийняті
filter-summary-tentative = Прийняті та попередні
filter-summary-combo = { $allday }, { $status }
additional-emails-section = Мої адреси електронної пошти
additional-emails-description = Додайте адреси електронної пошти, які ви використовуєте для запрошень календаря (наприклад, псевдоніми, робоча пошта). Вони використовуються для визначення вашого статусу прийняття зустрічей на додаток до пошти, налаштованої для кожного календаря.
additional-emails-summary = { $count ->
    [0] Не налаштовано
    [one] { $count } адреса
    [few] { $count } адреси
    *[other] { $count } адрес
}
add-email = Додати пошту
filter-events-description = Фільтруйте відображувані події календаря за властивостями подій або статусом участі.
panel-display = Панель
dropdown-display = Випадне меню
configure = Налаштувати...
about = Про застосунок
version = Версія { $version }
author = автор { $author }
website = Вебсайт
report-bug = Повідомити про помилку
no-calendars = Немає календарів
no-calendars-description = Налаштуйте обліковий запис календаря за допомогою GNOME Online Accounts або застосунку-календаря, наприклад Evolution.
refresh-section = Примусова віддалена синхронізація
refresh-now = Примусова синхронізація вручну
refreshing = Синхронізація...
auto-refresh = Автоматична примусова синхронізація
refresh-interval = Інтервал
refresh-interval-5m = Кожні 5 хвилин
refresh-interval-10m = Кожні 10 хвилин
refresh-interval-15m = Кожні 15 хвилин
refresh-interval-30m = Кожні 30 хвилин
refresh-summary-on = Кожні { $interval } хв
refresh-summary-off = Лише вручну
refresh-description =
    Цей аплет читає кешовані дані календаря з Evolution Data Server (EDS), який може синхронізуватися з онлайн-календарями. Застосунки-календарі, такі як Evolution та GNOME Calendar, запускають синхронізацію EDS.

    Якщо цю настройку увімкнено, аплет також автоматично запускатиме синхронізацію EDS з хмарою. Якщо ви використовуєте інше рішення або лише локальний календар, це може бути зайвим.
updated-unknown = Оновлено: невідомо
updated-just-now = Щойно оновлено
updated-minutes-ago = Оновлено { $minutes } хв тому
updated-hours-ago = Оновлено { $hours } год тому
updated-days-ago = Оновлено { $days } д тому
vertical-panel-notice = Цей аплет найкраще працює на горизонтальній панелі, де інформація про зустріч може відображатися в рядку.
calendars-setup-tip = Не забудьте налаштувати онлайн-календарі, які ви хочете використовувати, у GNOME Calendar/Online Accounts.
keyboard-shortcut = Комбінація клавіш
keyboard-shortcut-description = Ви можете налаштувати системну комбінацію клавіш для миттєвого приєднання до наступної зустрічі.
keyboard-shortcut-instructions = Відкрийте COSMIC Settings → Клавіатура → Користувацькі комбінації клавіш і додайте нову комбінацію з наступною командою:
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = Копіювати
keyboard-shortcut-open-settings = Відкрити налаштування

# Налаштування відображення часу
datetime-settings = Налаштування дати та часу
time-display-section = Відображення часу
military-time = 24-годинний формат
show-seconds = Показувати секунди
show-weekday = Показувати день тижня
first-day-of-week = Перший день тижня
first-day-sunday = Неділя
first-day-monday = Понеділок
custom-format = Користувацький формат
custom-format-description = Використовуйте рядок формату strftime (залиште порожнім для стандартного)

# Навігація по календарю
previous-month = Попередній місяць
next-month = Наступний місяць
today = Сьогодні

# Налаштування відображення панелі
panel-show-weekday = Показувати день тижня
panel-show-year = Показувати рік

# Погода
weather-section = Погода
weather-not-configured = Не налаштовано
weather-search-placeholder = Пошук міста...
weather-clear = Видалити місто
weather-current = Поточна погода

# Світовий годинник
world-clock-section = Світовий годинник
world-clock-not-configured = Не налаштовано
world-clock-add = Додати місто
world-clock-search-placeholder = Пошук міста...
world-clock-cities-count = { $count ->
    [one] { $count } місто
    [few] { $count } міста
    *[other] { $count } міст
}
