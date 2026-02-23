app-title = Calendário e agenda
no-meetings = Nenhuma reunião próxima
no-meetings-panel = Sem reuniões
no-meetings-day = Sem eventos
loading-meetings = Carregando reuniões...
settings = Configurações
open-calendar = Calendário
back = Voltar
next-meeting = Próxima reunião
upcoming = Próximas
calendars-section = Calendários
calendars-enabled = { $enabled }/{ $total } habilitados
calendars-none = Não configurado
display-format-section = Horário da próxima reunião
display-format-day-time = Dia e horário
display-format-relative = Tempo relativo
upcoming-events-section = Mostrar reuniões adicionais
join = Entrar
join-button-section = Botão de entrar
join-button-visibility = Visibilidade
join-button-description = Quando habilitado, um botão "Entrar" aparecerá para reuniões com uma URL de videochamada detectada.
formatting-section = Formatação
url-patterns-description = Padrões regex usados para detectar URLs de videochamadas nas descrições e locais das reuniões. Esses padrões se aplicam ao botão Entrar tanto no painel quanto no menu suspenso.
panel-join-button = No painel
popup-join-button = No menu suspenso
join-hide = Ocultar
join-show = Mostrar sempre
join-show-same-day = Mostrar se for no mesmo dia
join-show-30m = Mostrar se dentro de 30m
join-show-15m = Mostrar se dentro de 15m
join-show-5m = Mostrar se dentro de 5m
url-patterns = Padrões de URL
add-pattern = Adicionar padrão
location-section = Local físico
location-description = Quando habilitado, o local físico da reunião (ex.: nome de uma sala) será exibido, se disponível.
panel-location = No painel
popup-location = No menu suspenso
location-hide = Ocultar
location-show = Mostrar sempre (se disponível)
location-show-same-day = Mostrar se for no mesmo dia
location-show-30m = Mostrar se dentro de 30m
location-show-15m = Mostrar se dentro de 15m
location-show-5m = Mostrar se dentro de 5m
status-off = Desativado
status-panel = No painel
status-popup = No menu suspenso
status-both = No painel e menu suspenso
time-now = agora
time-in-days-hours = em { $days }d { $hours }h
time-in-days = em { $days }d
time-in-hours-minutes = em { $hours }h { $minutes }m
time-in-hours = em { $hours }h
time-in-minutes = em { $minutes }m
panel-time-location = ({ $time } em { $location })
panel-time = ({ $time })
calendar-indicator-section = Indicar calendário de origem
calendar-indicator-description = Quando habilitado, um ponto colorido indicando de qual calendário a reunião pertence será exibido.
panel-indicator = No painel
popup-indicator = No menu suspenso
filter-events-section = Filtrar eventos
show-all-day-events = Mostrar eventos de dia inteiro
in-progress-section = Mostrar reuniões em andamento
in-progress-off = Desativado
in-progress-5m = <5m após o início
in-progress-10m = <10m após o início
in-progress-15m = <15m após o início
in-progress-30m = <30m após o início
panel-started = iniciada
time-until-section = Filtrar por tempo restante
time-until-all = Todos os eventos
time-until-3h = Dentro de 3 horas
time-until-6h = Dentro de 6 horas
time-until-same-day = Mesmo dia
time-until-1d = Dentro de 1 dia
time-until-2d = Dentro de 2 dias
status-filter-section = Filtrar por status
status-filter-all = Todos os eventos
status-filter-accepted = Somente aceitos
status-filter-accepted-tentative = Aceitos e provisórios
filter-summary-all = Mostrar todos os eventos
filter-summary-no-all-day = Sem eventos de dia inteiro
filter-summary-accepted = Aceitos
filter-summary-tentative = Aceitos e provisórios
filter-summary-combo = { $allday }, { $status }
additional-emails-section = Meus endereços de e-mail
additional-emails-description = Adicione os endereços de e-mail que você usa para convites de calendário (ex.: aliases, e-mail do trabalho). Eles são usados para determinar seu status de aceitação em reuniões, além do e-mail configurado para cada calendário.
additional-emails-summary = { $count ->
    [0] Nenhum configurado
    [one] { $count } e-mail
    *[other] { $count } e-mails
}
add-email = Adicionar e-mail
filter-events-description = Filtre quais eventos do calendário são exibidos com base nas propriedades do evento ou no seu status de participação.
panel-display = Painel
dropdown-display = Menu suspenso
configure = Configurar...
about = Sobre
version = Versão { $version }
author = por { $author }
website = Site
report-bug = Reportar bug
no-calendars = Sem calendários
no-calendars-description = Configure uma conta de calendário usando GNOME Online Accounts ou um aplicativo de calendário como Evolution.
refresh-section = Forçar sincronização remota
refresh-now = Forçar sincronização manualmente
refreshing = Sincronizando...
auto-refresh = Auto-forçar sincronização remota
refresh-interval = Intervalo
refresh-interval-5m = A cada 5 minutos
refresh-interval-10m = A cada 10 minutos
refresh-interval-15m = A cada 15 minutos
refresh-interval-30m = A cada 30 minutos
refresh-summary-on = A cada { $interval }m
refresh-summary-off = Somente manual
refresh-description =
    Este applet lê dados de calendário em cache do Evolution Data Server (EDS), que pode sincronizar com calendários online. Aplicativos de calendário como Evolution e GNOME Calendar acionarão a sincronização do EDS.

    Se esta configuração estiver habilitada, o applet também acionará automaticamente a sincronização do EDS com a nuvem. Se você usa outra solução, ou se usa apenas um calendário local, isso pode ser redundante.
updated-unknown = Atualizado: desconhecido
updated-just-now = Atualizado agora mesmo
updated-minutes-ago = Atualizado há { $minutes }m
updated-hours-ago = Atualizado há { $hours }h
updated-days-ago = Atualizado há { $days }d
vertical-panel-notice = Este applet funciona melhor em um painel horizontal onde as informações da reunião podem ser exibidas em linha.
calendars-setup-tip = Lembre-se de configurar os calendários online que você deseja usar no GNOME Calendar/Online Accounts.
keyboard-shortcut = Atalho de teclado
keyboard-shortcut-description = Você pode configurar um atalho de teclado do sistema para entrar instantaneamente na sua próxima reunião.
keyboard-shortcut-instructions = Abra COSMIC Settings → Teclado → Atalhos personalizados e adicione um novo atalho com o seguinte comando:
keyboard-shortcut-command = cosmic-calendar-schedule --join-next
keyboard-shortcut-copy = Copiar
keyboard-shortcut-open-settings = Abrir configurações

# Configurações de exibição de hora
datetime-settings = Configurações de data e hora
time-display-section = Exibição de hora
military-time = Formato 24 horas
show-seconds = Mostrar segundos
show-weekday = Mostrar dia da semana
first-day-of-week = Primeiro dia da semana
first-day-sunday = Domingo
first-day-monday = Segunda-feira
custom-format = Formato personalizado
custom-format-description = Use uma string de formato strftime (deixe vazio para o padrão)

# Navegação do calendário
previous-month = Mês anterior
next-month = Próximo mês
today = Hoje

# Configurações de exibição do painel
panel-show-weekday = Mostrar dia da semana
panel-show-year = Mostrar ano

# Clima
weather-section = Clima
weather-not-configured = Não configurado
weather-search-placeholder = Buscar cidade...
weather-clear = Remover cidade
weather-current = Clima atual

# Relógio mundial
world-clock-section = Relógio mundial
world-clock-not-configured = Não configurado
world-clock-add = Adicionar cidade
world-clock-search-placeholder = Buscar cidade...
world-clock-cities-count = { $count ->
    [one] { $count } cidade
    *[other] { $count } cidades
}
