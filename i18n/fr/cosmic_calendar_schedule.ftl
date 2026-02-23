app-title = Calendrier et agenda
no-meetings = Aucune réunion à venir
no-meetings-panel = Aucune réunion
no-meetings-day = Aucun événement
loading-meetings = Chargement des réunions...
settings = Paramètres
open-calendar = Calendrier
back = Retour
next-meeting = Prochaine réunion
upcoming = À venir
calendars-section = Calendriers
calendars-enabled = { $enabled }/{ $total } activés
calendars-none = Non configuré
display-format-section = Heure de la prochaine réunion
display-format-day-time = Jour et heure
display-format-relative = Temps relatif
upcoming-events-section = Afficher les réunions supplémentaires
join = Rejoindre
join-button-section = Bouton rejoindre
join-button-visibility = Visibilité
join-button-description = Lorsqu'il est activé, un bouton « Rejoindre » apparaîtra pour les réunions avec une URL de visioconférence détectée.
formatting-section = Mise en forme
url-patterns-description = Expressions régulières utilisées pour détecter les URL de visioconférence dans les descriptions et emplacements des réunions. Ces motifs s'appliquent au bouton Rejoindre dans le panneau et le menu déroulant.
panel-join-button = Dans le panneau
popup-join-button = Dans le menu déroulant
join-hide = Masquer
join-show = Toujours afficher
join-show-same-day = Afficher si même jour
join-show-30m = Afficher si dans moins de 30m
join-show-15m = Afficher si dans moins de 15m
join-show-5m = Afficher si dans moins de 5m
url-patterns = Motifs d'URL
add-pattern = Ajouter un motif
location-section = Lieu physique
location-description = Lorsqu'il est activé, le lieu physique de la réunion (par ex. le nom d'une salle) sera affiché s'il est disponible.
panel-location = Dans le panneau
popup-location = Dans le menu déroulant
location-hide = Masquer
location-show = Toujours afficher (si disponible)
location-show-same-day = Afficher si même jour
location-show-30m = Afficher si dans moins de 30m
location-show-15m = Afficher si dans moins de 15m
location-show-5m = Afficher si dans moins de 5m
status-off = Désactivé
status-panel = Dans le panneau
status-popup = Dans le menu déroulant
status-both = Dans le panneau et le menu déroulant
time-now = maintenant
time-in-days-hours = dans { $days }j { $hours }h
time-in-days = dans { $days }j
time-in-hours-minutes = dans { $hours }h { $minutes }m
time-in-hours = dans { $hours }h
time-in-minutes = dans { $minutes }m
panel-time-location = ({ $time } à { $location })
panel-time = ({ $time })
calendar-indicator-section = Indiquer le calendrier source
calendar-indicator-description = Lorsqu'il est activé, un point coloré indiquant de quel calendrier provient la réunion sera affiché.
panel-indicator = Dans le panneau
popup-indicator = Dans le menu déroulant
filter-events-section = Filtrer les événements
show-all-day-events = Afficher les événements de toute la journée
in-progress-section = Afficher les réunions en cours
in-progress-off = Désactivé
in-progress-5m = <5m après le début
in-progress-10m = <10m après le début
in-progress-15m = <15m après le début
in-progress-30m = <30m après le début
panel-started = commencée
time-until-section = Filtrer par temps restant
time-until-all = Tous les événements
time-until-3h = Dans les 3 prochaines heures
time-until-6h = Dans les 6 prochaines heures
time-until-same-day = Même jour
time-until-1d = Dans les prochaines 24h
time-until-2d = Dans les 2 prochains jours
status-filter-section = Filtrer par statut
status-filter-all = Tous les événements
status-filter-accepted = Acceptés uniquement
status-filter-accepted-tentative = Acceptés et provisoires
filter-summary-all = Afficher tous les événements
filter-summary-no-all-day = Sans événements journée entière
filter-summary-accepted = Acceptés
filter-summary-tentative = Acceptés et provisoires
filter-summary-combo = { $allday }, { $status }
additional-emails-section = Mes adresses e-mail
additional-emails-description = Ajoutez les adresses e-mail que vous utilisez pour les invitations de calendrier (par ex. alias, e-mail professionnel). Elles sont utilisées pour déterminer votre statut d'acceptation des réunions, en plus de l'e-mail configuré pour chaque calendrier.
additional-emails-summary = { $count ->
    [0] Aucun configuré
    [one] { $count } e-mail
    *[other] { $count } e-mails
}
add-email = Ajouter un e-mail
filter-events-description = Filtrez les événements de calendrier affichés selon les propriétés de l'événement ou votre statut de participation.
panel-display = Panneau
dropdown-display = Menu déroulant
configure = Configurer...
about = À propos
version = Version { $version }
author = par { $author }
website = Site web
report-bug = Signaler un bug
no-calendars = Aucun calendrier
no-calendars-description = Configurez un compte de calendrier avec GNOME Online Accounts ou une application de calendrier comme Evolution.
refresh-section = Forcer la synchronisation distante
refresh-now = Forcer la synchronisation manuellement
refreshing = Synchronisation...
auto-refresh = Auto-forcer la synchronisation distante
refresh-interval = Intervalle
refresh-interval-5m = Toutes les 5 minutes
refresh-interval-10m = Toutes les 10 minutes
refresh-interval-15m = Toutes les 15 minutes
refresh-interval-30m = Toutes les 30 minutes
refresh-summary-on = Toutes les { $interval }m
refresh-summary-off = Manuel uniquement
refresh-description =
    Cet applet lit les données de calendrier en cache d'Evolution Data Server (EDS), qui peut se synchroniser avec des calendriers en ligne. Les applications de calendrier comme Evolution et GNOME Calendar déclencheront la synchronisation d'EDS.

    Si ce paramètre est activé, l'applet déclenchera aussi automatiquement la synchronisation d'EDS avec le cloud. Si vous utilisez une autre solution, ou si vous n'utilisez qu'un calendrier local, cela peut être redondant.
updated-unknown = Mis à jour : inconnu
updated-just-now = Mis à jour à l'instant
updated-minutes-ago = Mis à jour il y a { $minutes }m
updated-hours-ago = Mis à jour il y a { $hours }h
updated-days-ago = Mis à jour il y a { $days }j
vertical-panel-notice = Cet applet fonctionne mieux dans un panneau horizontal où les informations de réunion peuvent être affichées en ligne.
calendars-setup-tip = N'oubliez pas de configurer les calendriers en ligne que vous souhaitez utiliser dans GNOME Calendar/Online Accounts.
keyboard-shortcut = Raccourci clavier
keyboard-shortcut-description = Vous pouvez configurer un raccourci clavier système pour rejoindre instantanément votre prochaine réunion.
keyboard-shortcut-instructions = Ouvrez COSMIC Settings → Clavier → Raccourcis personnalisés et ajoutez un nouveau raccourci avec la commande suivante :
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = Copier
keyboard-shortcut-open-settings = Ouvrir les paramètres

# Paramètres d'affichage de l'heure
datetime-settings = Paramètres de date et heure
time-display-section = Affichage de l'heure
military-time = Format 24 heures
show-seconds = Afficher les secondes
show-weekday = Afficher le jour de la semaine
first-day-of-week = Premier jour de la semaine
first-day-sunday = Dimanche
first-day-monday = Lundi
custom-format = Format personnalisé
custom-format-description = Utilisez une chaîne de format strftime (laissez vide pour le format par défaut)

# Navigation du calendrier
previous-month = Mois précédent
next-month = Mois suivant
today = Aujourd'hui

# Paramètres d'affichage du panneau
panel-show-weekday = Afficher le jour de la semaine
panel-show-year = Afficher l'année

# Météo
weather-section = Météo
weather-not-configured = Non configuré
weather-search-placeholder = Rechercher une ville...
weather-clear = Supprimer la ville
weather-current = Météo actuelle

# Horloge mondiale
world-clock-section = Horloge mondiale
world-clock-not-configured = Non configuré
world-clock-add = Ajouter une ville
world-clock-search-placeholder = Rechercher une ville...
world-clock-cities-count = { $count ->
    [one] { $count } ville
    *[other] { $count } villes
}
