# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_roadmap_global_optspecs
	string join \n h/help V/version
end

function __fish_roadmap_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_roadmap_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_roadmap_using_subcommand
	set -l cmd (__fish_roadmap_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c roadmap -n "__fish_roadmap_needs_command" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_needs_command" -s V -l version -d 'Print version'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "init" -d 'Initialiser une roadmap dans le projet courant'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "add" -d 'Ajouter une nouvelle phase'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "edit" -d 'Modifier une phase'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "list" -d 'Lister toutes les phases'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "tree" -d 'Afficher l\'arborescence complète'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "show" -d 'Afficher les détails d\'une phase'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "task" -d 'Gérer les tâches'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "priority" -d 'Changer la priorité d\'une phase'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "status" -d 'Changer le statut d\'une phase'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "note" -d 'Ajouter une note à une phase'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "export" -d 'Générer les fichiers Markdown'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "report" -d 'Rapport de progression (vue planning)'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "workflow" -d 'Gérer le workflow d\'une tâche'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "ui" -d 'Interface interactive (TUI)'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "serve" -d 'Lancer le serveur web'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "update" -d 'Mettre à jour roadmap-cli'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "next" -d 'Afficher la prochaine tâche à faire'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "context" -d 'Contexte complet pour LLM (optimisé pour AI assistants)'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "scan" -d 'Scanner le code pour trouver les TODO/FIXME'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "coverage" -d 'Analyser la couverture API (routes backend vs appels frontend)'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "changelog" -d 'Générer un changelog depuis les commits git'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "generate" -d 'Générer les fichiers man et completions shell'
complete -c roadmap -n "__fish_roadmap_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c roadmap -n "__fish_roadmap_using_subcommand init" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand add" -l parent -d 'Phase parente (pour les sous-phases)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand add" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand edit" -l name -d 'Nouveau nom' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand edit" -l description -d 'Nouvelle description' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand edit" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand list" -l table -d 'Afficher en tableau formaté'
complete -c roadmap -n "__fish_roadmap_using_subcommand list" -l json -d 'Sortie JSON'
complete -c roadmap -n "__fish_roadmap_using_subcommand list" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand tree" -l json -d 'Sortie JSON'
complete -c roadmap -n "__fish_roadmap_using_subcommand tree" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand show" -l json -d 'Sortie JSON'
complete -c roadmap -n "__fish_roadmap_using_subcommand show" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "add" -d 'Ajouter une tâche à une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "done" -d 'Marquer une tâche comme terminée'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "start" -d 'Marquer une tâche comme en cours'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "edit" -d 'Modifier une tâche'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "move" -d 'Déplacer une tâche vers une autre phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "blocks" -d 'Définir qu\'une tâche en bloque une autre'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "unblocks" -d 'Retirer une dépendance'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and not __fish_seen_subcommand_from add done start edit move blocks unblocks help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from add" -l parent -d 'Tâche parente (pour les sous-tâches)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from add" -l files -d 'Fichiers liés à cette tâche' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from add" -l optional -d 'Marquer comme optionnelle'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from done" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from start" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from edit" -l name -d 'Nouveau nom' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from edit" -l description -d 'Nouvelle description' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from edit" -l optional -d 'Marquer comme optionnelle' -r -f -a "true\t''
false\t''"
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from edit" -l files -d 'Fichiers liés à cette tâche (remplace les existants)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from edit" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from move" -l to -d 'Phase de destination' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from move" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from blocks" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from unblocks" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "add" -d 'Ajouter une tâche à une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "done" -d 'Marquer une tâche comme terminée'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "start" -d 'Marquer une tâche comme en cours'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "edit" -d 'Modifier une tâche'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "move" -d 'Déplacer une tâche vers une autre phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "blocks" -d 'Définir qu\'une tâche en bloque une autre'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "unblocks" -d 'Retirer une dépendance'
complete -c roadmap -n "__fish_roadmap_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c roadmap -n "__fish_roadmap_using_subcommand priority" -l set -d 'Nouvelle priorité (1 = haute)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand priority" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand status" -l set -d 'Nouveau statut (pending, in_progress, done, blocked)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand status" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand note" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand export" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand report" -l json -d 'Sortie JSON'
complete -c roadmap -n "__fish_roadmap_using_subcommand report" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand workflow" -l set -d 'Forcer une étape spécifique' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand workflow" -l advance -d 'Avancer à l\'étape suivante'
complete -c roadmap -n "__fish_roadmap_using_subcommand workflow" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand ui" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand serve" -s p -l port -d 'Port du serveur (défaut: 7878)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand serve" -l open -d 'Ouvrir le navigateur automatiquement'
complete -c roadmap -n "__fish_roadmap_using_subcommand serve" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand update" -l check -d 'Vérifier seulement, sans installer'
complete -c roadmap -n "__fish_roadmap_using_subcommand update" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand next" -l json -d 'Sortie JSON'
complete -c roadmap -n "__fish_roadmap_using_subcommand next" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand context" -l include-done -d 'Inclure les tâches terminées'
complete -c roadmap -n "__fish_roadmap_using_subcommand context" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand scan" -l glob -d 'Pattern glob pour filtrer les fichiers (défaut: **/*.rs)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand scan" -l phase -d 'Phase cible pour les tâches créées' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand scan" -l create -d 'Créer des tâches automatiquement'
complete -c roadmap -n "__fish_roadmap_using_subcommand scan" -l hidden -d 'Inclure les répertoires cachés'
complete -c roadmap -n "__fish_roadmap_using_subcommand scan" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand coverage" -s b -l backend -d 'Pattern glob pour les fichiers backend (ex: src/api/**/*.ts)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand coverage" -s f -l frontend -d 'Pattern glob pour les fichiers frontend (ex: src/app/**/*.tsx)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand coverage" -l prefix -d 'Préfixe des routes API (défaut: /api)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand coverage" -l json -d 'Sortie JSON'
complete -c roadmap -n "__fish_roadmap_using_subcommand coverage" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand changelog" -l limit -d 'Nombre de commits à inclure (défaut: 50)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand changelog" -l from -d 'Tag de départ (ex: v0.1.0)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand changelog" -l to -d 'Tag de fin (défaut: HEAD)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand changelog" -l format -d 'Format de sortie (markdown, json)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand changelog" -s h -l help -d 'Print help'
complete -c roadmap -n "__fish_roadmap_using_subcommand generate" -s o -l output -d 'Répertoire de sortie (défaut: ./generated)' -r
complete -c roadmap -n "__fish_roadmap_using_subcommand generate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "init" -d 'Initialiser une roadmap dans le projet courant'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "add" -d 'Ajouter une nouvelle phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "edit" -d 'Modifier une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "list" -d 'Lister toutes les phases'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "tree" -d 'Afficher l\'arborescence complète'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "show" -d 'Afficher les détails d\'une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "task" -d 'Gérer les tâches'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "priority" -d 'Changer la priorité d\'une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "status" -d 'Changer le statut d\'une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "note" -d 'Ajouter une note à une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "export" -d 'Générer les fichiers Markdown'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "report" -d 'Rapport de progression (vue planning)'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "workflow" -d 'Gérer le workflow d\'une tâche'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "ui" -d 'Interface interactive (TUI)'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "serve" -d 'Lancer le serveur web'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "update" -d 'Mettre à jour roadmap-cli'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "next" -d 'Afficher la prochaine tâche à faire'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "context" -d 'Contexte complet pour LLM (optimisé pour AI assistants)'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "scan" -d 'Scanner le code pour trouver les TODO/FIXME'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "coverage" -d 'Analyser la couverture API (routes backend vs appels frontend)'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "changelog" -d 'Générer un changelog depuis les commits git'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "generate" -d 'Générer les fichiers man et completions shell'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and not __fish_seen_subcommand_from init add edit list tree show task priority status note export report workflow ui serve update next context scan coverage changelog generate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "add" -d 'Ajouter une tâche à une phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "done" -d 'Marquer une tâche comme terminée'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "start" -d 'Marquer une tâche comme en cours'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "edit" -d 'Modifier une tâche'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "move" -d 'Déplacer une tâche vers une autre phase'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "blocks" -d 'Définir qu\'une tâche en bloque une autre'
complete -c roadmap -n "__fish_roadmap_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "unblocks" -d 'Retirer une dépendance'
