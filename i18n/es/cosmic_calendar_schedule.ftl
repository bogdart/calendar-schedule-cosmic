app-title = Calendario y agenda
no-meetings = No hay reuniones próximas
no-meetings-panel = Sin reuniones
no-meetings-day = Sin eventos
loading-meetings = Cargando reuniones...
settings = Ajustes
open-calendar = Calendario
back = Volver
next-meeting = Próxima reunión
upcoming = Próximas
calendars-section = Calendarios
calendars-enabled = { $enabled }/{ $total } habilitados
calendars-none = No configurado
display-format-section = Hora de la próxima reunión
display-format-day-time = Día y hora
display-format-relative = Tiempo relativo
upcoming-events-section = Mostrar reuniones adicionales
join = Unirse
join-button-section = Botón de unirse
join-button-visibility = Visibilidad
join-button-description = Cuando está habilitado, aparecerá un botón "Unirse" para reuniones con una URL de videollamada detectada.
formatting-section = Formato
url-patterns-description = Patrones regex utilizados para detectar URLs de videollamadas en las descripciones y ubicaciones de reuniones. Estos patrones se aplican al botón Unirse tanto en el panel como en el desplegable.
panel-join-button = En el panel
popup-join-button = En el desplegable
join-hide = Ocultar
join-show = Mostrar siempre
join-show-same-day = Mostrar si es el mismo día
join-show-30m = Mostrar si faltan menos de 30m
join-show-15m = Mostrar si faltan menos de 15m
join-show-5m = Mostrar si faltan menos de 5m
url-patterns = Patrones de URL
add-pattern = Añadir patrón
location-section = Ubicación física
location-description = Cuando está habilitado, se mostrará la ubicación física de la reunión (por ejemplo, el nombre de una sala) si está disponible.
panel-location = En el panel
popup-location = En el desplegable
location-hide = Ocultar
location-show = Mostrar siempre (si está disponible)
location-show-same-day = Mostrar si es el mismo día
location-show-30m = Mostrar si faltan menos de 30m
location-show-15m = Mostrar si faltan menos de 15m
location-show-5m = Mostrar si faltan menos de 5m
status-off = Desactivado
status-panel = En el panel
status-popup = En el desplegable
status-both = En el panel y desplegable
time-now = ahora
time-in-days-hours = en { $days }d { $hours }h
time-in-days = en { $days }d
time-in-hours-minutes = en { $hours }h { $minutes }m
time-in-hours = en { $hours }h
time-in-minutes = en { $minutes }m
panel-time-location = ({ $time } en { $location })
panel-time = ({ $time })
calendar-indicator-section = Indicar calendario de origen
calendar-indicator-description = Cuando está habilitado, se mostrará un punto de color que indica de qué calendario proviene la reunión.
panel-indicator = En el panel
popup-indicator = En el desplegable
filter-events-section = Filtrar eventos
show-all-day-events = Mostrar eventos de todo el día
in-progress-section = Mostrar reuniones en curso
in-progress-off = Desactivado
in-progress-5m = <5m después del inicio
in-progress-10m = <10m después del inicio
in-progress-15m = <15m después del inicio
in-progress-30m = <30m después del inicio
panel-started = iniciada
time-until-section = Filtrar por tiempo restante
time-until-all = Todos los eventos
time-until-3h = Dentro de 3 horas
time-until-6h = Dentro de 6 horas
time-until-same-day = Mismo día
time-until-1d = Dentro de 1 día
time-until-2d = Dentro de 2 días
status-filter-section = Filtrar por estado
status-filter-all = Todos los eventos
status-filter-accepted = Solo aceptados
status-filter-accepted-tentative = Aceptados y tentativos
filter-summary-all = Mostrar todos los eventos
filter-summary-no-all-day = Sin eventos de todo el día
filter-summary-accepted = Aceptados
filter-summary-tentative = Aceptados y tentativos
filter-summary-combo = { $allday }, { $status }
additional-emails-section = Mis direcciones de correo
additional-emails-description = Añade las direcciones de correo que usas para invitaciones de calendario (por ejemplo, alias, correo del trabajo). Se usan para determinar tu estado de aceptación en reuniones, además del correo configurado para cada calendario.
additional-emails-summary = { $count ->
    [0] Ninguno configurado
    [one] { $count } correo
    *[other] { $count } correos
}
add-email = Añadir correo
filter-events-description = Filtra qué eventos del calendario se muestran según las propiedades del evento o tu estado de asistencia.
panel-display = Panel
dropdown-display = Desplegable
configure = Configurar...
about = Acerca de
version = Versión { $version }
author = por { $author }
website = Sitio web
report-bug = Reportar error
no-calendars = Sin calendarios
no-calendars-description = Configura una cuenta de calendario usando GNOME Online Accounts o una aplicación de calendario como Evolution.
refresh-section = Forzar sincronización remota
refresh-now = Forzar sincronización manualmente
refreshing = Sincronizando...
auto-refresh = Auto-forzar sincronización remota
refresh-interval = Intervalo
refresh-interval-5m = Cada 5 minutos
refresh-interval-10m = Cada 10 minutos
refresh-interval-15m = Cada 15 minutos
refresh-interval-30m = Cada 30 minutos
refresh-summary-on = Cada { $interval }m
refresh-summary-off = Solo manual
refresh-description =
    Este applet lee datos de calendario almacenados en caché de Evolution Data Server (EDS), que puede sincronizar con calendarios en línea. Aplicaciones de calendario como Evolution y GNOME Calendar activarán la sincronización de EDS.

    Si esta opción está habilitada, el applet también activará automáticamente la sincronización de EDS con la nube. Si usas otra solución, o si solo usas un calendario local, esto puede ser redundante.
updated-unknown = Actualizado: desconocido
updated-just-now = Actualizado hace un momento
updated-minutes-ago = Actualizado hace { $minutes }m
updated-hours-ago = Actualizado hace { $hours }h
updated-days-ago = Actualizado hace { $days }d
vertical-panel-notice = Este applet funciona mejor en un panel horizontal donde la información de la reunión se puede mostrar en línea.
calendars-setup-tip = Recuerda configurar los calendarios en línea que quieras usar en GNOME Calendar/Online Accounts.
keyboard-shortcut = Atajo de teclado
keyboard-shortcut-description = Puedes configurar un atajo de teclado del sistema para unirte instantáneamente a tu próxima reunión.
keyboard-shortcut-instructions = Abre COSMIC Settings → Teclado → Atajos personalizados y añade un nuevo atajo con el siguiente comando:
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = Copiar
keyboard-shortcut-open-settings = Abrir ajustes

# Ajustes de visualización de hora
datetime-settings = Ajustes de fecha y hora
time-display-section = Visualización de hora
military-time = Formato 24 horas
show-seconds = Mostrar segundos
show-weekday = Mostrar día de la semana
first-day-of-week = Primer día de la semana
first-day-sunday = Domingo
first-day-monday = Lunes
custom-format = Formato personalizado
custom-format-description = Usa una cadena de formato strftime (deja vacío para el predeterminado)

# Navegación del calendario
previous-month = Mes anterior
next-month = Mes siguiente
today = Hoy

# Ajustes de visualización del panel
panel-show-weekday = Mostrar día de la semana
panel-show-year = Mostrar año

# Clima
weather-section = Clima
weather-not-configured = No configurado
weather-search-placeholder = Buscar ciudad...
weather-clear = Eliminar ciudad
weather-current = Clima actual

# Reloj mundial
world-clock-section = Reloj mundial
world-clock-not-configured = No configurado
world-clock-add = Añadir ciudad
world-clock-search-placeholder = Buscar ciudad...
world-clock-cities-count = { $count ->
    [one] { $count } ciudad
    *[other] { $count } ciudades
}
