app-title = Kalender & Termine
no-meetings = Keine anstehenden Meetings
no-meetings-panel = Keine Meetings
no-meetings-day = Keine Termine
loading-meetings = Meetings werden geladen...
settings = Einstellungen
open-calendar = Kalender
back = Zurück
next-meeting = Nächstes Meeting
upcoming = Anstehend
calendars-section = Kalender
calendars-enabled = { $enabled }/{ $total } aktiviert
calendars-none = Nicht konfiguriert
display-format-section = Uhrzeit des nächsten Meetings
display-format-day-time = Tag & Uhrzeit
display-format-relative = Relative Zeit
upcoming-events-section = Weitere Meetings anzeigen
join = Beitreten
join-button-section = Beitreten-Schaltfläche
join-button-visibility = Sichtbarkeit
join-button-description = Wenn aktiviert, wird eine „Beitreten"-Schaltfläche für Meetings mit erkannter Videoanruf-URL angezeigt.
formatting-section = Formatierung
url-patterns-description = Regex-Muster zur Erkennung von Videoanruf-URLs in Meetingbeschreibungen und -orten. Diese Muster gelten für die Beitreten-Schaltfläche im Panel und im Dropdown.
panel-join-button = Im Panel
popup-join-button = Im Dropdown
join-hide = Ausblenden
join-show = Immer anzeigen
join-show-same-day = Anzeigen wenn gleicher Tag
join-show-30m = Anzeigen wenn innerhalb von 30m
join-show-15m = Anzeigen wenn innerhalb von 15m
join-show-5m = Anzeigen wenn innerhalb von 5m
url-patterns = URL-Muster
add-pattern = Muster hinzufügen
location-section = Physischer Ort
location-description = Wenn aktiviert, wird der physische Ort des Meetings (z. B. ein Raumname) angezeigt, sofern verfügbar.
panel-location = Im Panel
popup-location = Im Dropdown
location-hide = Ausblenden
location-show = Immer anzeigen (wenn verfügbar)
location-show-same-day = Anzeigen wenn gleicher Tag
location-show-30m = Anzeigen wenn innerhalb von 30m
location-show-15m = Anzeigen wenn innerhalb von 15m
location-show-5m = Anzeigen wenn innerhalb von 5m
status-off = Aus
status-panel = Im Panel
status-popup = Im Dropdown
status-both = Im Panel & Dropdown
time-now = jetzt
time-in-days-hours = in { $days }T { $hours }Std
time-in-days = in { $days }T
time-in-hours-minutes = in { $hours }Std { $minutes }Min
time-in-hours = in { $hours }Std
time-in-minutes = in { $minutes }Min
panel-time-location = ({ $time } in { $location })
panel-time = ({ $time })
calendar-indicator-section = Quellkalender anzeigen
calendar-indicator-description = Wenn aktiviert, wird ein farbiger Punkt angezeigt, der angibt, von welchem Kalender das Meeting stammt.
panel-indicator = Im Panel
popup-indicator = Im Dropdown
filter-events-section = Termine filtern
show-all-day-events = Ganztägige Termine anzeigen
in-progress-section = Laufende Meetings anzeigen
in-progress-off = Aus
in-progress-5m = <5Min nach Beginn
in-progress-10m = <10Min nach Beginn
in-progress-15m = <15Min nach Beginn
in-progress-30m = <30Min nach Beginn
panel-started = gestartet
time-until-section = Nach Zeit filtern
time-until-all = Alle Termine
time-until-3h = Innerhalb von 3 Stunden
time-until-6h = Innerhalb von 6 Stunden
time-until-same-day = Gleicher Tag
time-until-1d = Innerhalb von 1 Tag
time-until-2d = Innerhalb von 2 Tagen
status-filter-section = Nach Status filtern
status-filter-all = Alle Termine
status-filter-accepted = Nur akzeptierte
status-filter-accepted-tentative = Akzeptierte und vorläufige
filter-summary-all = Alle Termine anzeigen
filter-summary-no-all-day = Ohne ganztägige
filter-summary-accepted = Akzeptierte
filter-summary-tentative = Akzeptierte und vorläufige
filter-summary-combo = { $allday }, { $status }
additional-emails-section = Meine E-Mail-Adressen
additional-emails-description = Fügen Sie E-Mail-Adressen hinzu, die Sie für Kalendereinladungen verwenden (z. B. Aliase, Arbeits-E-Mail). Diese werden zur Bestimmung Ihres Annahmestatus bei Meetings verwendet, zusätzlich zur für jeden Kalender konfigurierten E-Mail.
additional-emails-summary = { $count ->
    [0] Keine konfiguriert
    [one] { $count } E-Mail
    *[other] { $count } E-Mails
}
add-email = E-Mail hinzufügen
filter-events-description = Filtern Sie, welche Kalendertermine basierend auf Termin-Eigenschaften oder Ihrem Teilnahmestatus angezeigt werden.
panel-display = Panel
dropdown-display = Dropdown
configure = Konfigurieren...
about = Über
version = Version { $version }
author = von { $author }
website = Webseite
report-bug = Fehler melden
no-calendars = Keine Kalender
no-calendars-description = Konfigurieren Sie ein Kalenderkonto über GNOME Online Accounts oder eine Kalender-App wie Evolution.
refresh-section = Remote-Synchronisation erzwingen
refresh-now = Synchronisation manuell erzwingen
refreshing = Synchronisierung...
auto-refresh = Automatische Remote-Synchronisation
refresh-interval = Intervall
refresh-interval-5m = Alle 5 Minuten
refresh-interval-10m = Alle 10 Minuten
refresh-interval-15m = Alle 15 Minuten
refresh-interval-30m = Alle 30 Minuten
refresh-summary-on = Alle { $interval }Min
refresh-summary-off = Nur manuell
refresh-description =
    Dieses Applet liest zwischengespeicherte Kalenderdaten vom Evolution Data Server (EDS), der mit Online-Kalendern synchronisieren kann. Kalender-Apps wie Evolution und GNOME Calendar lösen die EDS-Synchronisation aus.

    Wenn diese Einstellung aktiviert ist, löst das Applet auch automatisch die EDS-Synchronisation mit der Cloud aus. Wenn Sie eine andere Lösung verwenden oder nur einen lokalen Kalender nutzen, kann dies überflüssig sein.
updated-unknown = Aktualisiert: unbekannt
updated-just-now = Gerade aktualisiert
updated-minutes-ago = Vor { $minutes }Min aktualisiert
updated-hours-ago = Vor { $hours }Std aktualisiert
updated-days-ago = Vor { $days }T aktualisiert
vertical-panel-notice = Dieses Applet funktioniert am besten in einem horizontalen Panel, in dem Meeting-Informationen inline angezeigt werden können.
calendars-setup-tip = Denken Sie daran, alle Online-Kalender, die Sie verwenden möchten, in GNOME Calendar/Online Accounts einzurichten.
keyboard-shortcut = Tastenkürzel
keyboard-shortcut-description = Sie können ein System-Tastenkürzel einrichten, um sofort Ihrem nächsten Meeting beizutreten.
keyboard-shortcut-instructions = Öffnen Sie COSMIC Settings → Tastatur → Benutzerdefinierte Tastenkürzel und fügen Sie ein neues Tastenkürzel mit folgendem Befehl hinzu:
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = Kopieren
keyboard-shortcut-open-settings = Einstellungen öffnen

# Zeitanzeigeeinstellungen
datetime-settings = Datum- & Zeiteinstellungen
time-display-section = Zeitanzeige
military-time = 24-Stunden-Format
show-seconds = Sekunden anzeigen
show-weekday = Wochentag anzeigen
first-day-of-week = Erster Wochentag
first-day-sunday = Sonntag
first-day-monday = Montag
custom-format = Benutzerdefiniertes Format
custom-format-description = Verwenden Sie eine strftime-Formatzeichenkette (leer lassen für Standard)

# Kalendernavigation
previous-month = Vorheriger Monat
next-month = Nächster Monat
today = Heute

# Panel-Anzeigeeinstellungen
panel-show-weekday = Wochentag anzeigen
panel-show-year = Jahr anzeigen

# Wetter
weather-section = Wetter
weather-not-configured = Nicht konfiguriert
weather-search-placeholder = Stadt suchen...
weather-clear = Stadt entfernen
weather-current = Aktuelles Wetter

# Weltzeituhr
world-clock-section = Weltzeituhr
world-clock-not-configured = Nicht konfiguriert
world-clock-add = Stadt hinzufügen
world-clock-search-placeholder = Stadt suchen...
world-clock-cities-count = { $count ->
    [one] { $count } Stadt
    *[other] { $count } Städte
}
