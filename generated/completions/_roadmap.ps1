
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'roadmap' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'roadmap'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'roadmap' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialiser une roadmap dans le projet courant')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Ajouter une nouvelle phase')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Modifier une phase')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'Lister toutes les phases')
            [CompletionResult]::new('tree', 'tree', [CompletionResultType]::ParameterValue, 'Afficher l''arborescence complète')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Afficher les détails d''une phase')
            [CompletionResult]::new('task', 'task', [CompletionResultType]::ParameterValue, 'Gérer les tâches')
            [CompletionResult]::new('priority', 'priority', [CompletionResultType]::ParameterValue, 'Changer la priorité d''une phase')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Changer le statut d''une phase')
            [CompletionResult]::new('note', 'note', [CompletionResultType]::ParameterValue, 'Ajouter une note à une phase')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Générer les fichiers Markdown')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Rapport de progression (vue planning)')
            [CompletionResult]::new('workflow', 'workflow', [CompletionResultType]::ParameterValue, 'Gérer le workflow d''une tâche')
            [CompletionResult]::new('ui', 'ui', [CompletionResultType]::ParameterValue, 'Interface interactive (TUI)')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Lancer le serveur web')
            [CompletionResult]::new('update', 'update', [CompletionResultType]::ParameterValue, 'Mettre à jour roadmap-cli')
            [CompletionResult]::new('next', 'next', [CompletionResultType]::ParameterValue, 'Afficher la prochaine tâche à faire')
            [CompletionResult]::new('context', 'context', [CompletionResultType]::ParameterValue, 'Contexte complet pour LLM (optimisé pour AI assistants)')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scanner le code pour trouver les TODO/FIXME')
            [CompletionResult]::new('coverage', 'coverage', [CompletionResultType]::ParameterValue, 'Analyser la couverture API (routes backend vs appels frontend)')
            [CompletionResult]::new('changelog', 'changelog', [CompletionResultType]::ParameterValue, 'Générer un changelog depuis les commits git')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Générer les fichiers man et completions shell')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'roadmap;init' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;add' {
            [CompletionResult]::new('--parent', '--parent', [CompletionResultType]::ParameterName, 'Phase parente (pour les sous-phases)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;edit' {
            [CompletionResult]::new('--name', '--name', [CompletionResultType]::ParameterName, 'Nouveau nom')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'Nouvelle description')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;list' {
            [CompletionResult]::new('--table', '--table', [CompletionResultType]::ParameterName, 'Afficher en tableau formaté')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Sortie JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;tree' {
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Sortie JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;show' {
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Sortie JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Ajouter une tâche à une phase')
            [CompletionResult]::new('done', 'done', [CompletionResultType]::ParameterValue, 'Marquer une tâche comme terminée')
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Marquer une tâche comme en cours')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Modifier une tâche')
            [CompletionResult]::new('move', 'move', [CompletionResultType]::ParameterValue, 'Déplacer une tâche vers une autre phase')
            [CompletionResult]::new('blocks', 'blocks', [CompletionResultType]::ParameterValue, 'Définir qu''une tâche en bloque une autre')
            [CompletionResult]::new('unblocks', 'unblocks', [CompletionResultType]::ParameterValue, 'Retirer une dépendance')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'roadmap;task;add' {
            [CompletionResult]::new('--parent', '--parent', [CompletionResultType]::ParameterName, 'Tâche parente (pour les sous-tâches)')
            [CompletionResult]::new('--files', '--files', [CompletionResultType]::ParameterName, 'Fichiers liés à cette tâche')
            [CompletionResult]::new('--optional', '--optional', [CompletionResultType]::ParameterName, 'Marquer comme optionnelle')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;done' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;start' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;edit' {
            [CompletionResult]::new('--name', '--name', [CompletionResultType]::ParameterName, 'Nouveau nom')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'Nouvelle description')
            [CompletionResult]::new('--optional', '--optional', [CompletionResultType]::ParameterName, 'Marquer comme optionnelle')
            [CompletionResult]::new('--files', '--files', [CompletionResultType]::ParameterName, 'Fichiers liés à cette tâche (remplace les existants)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;move' {
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'Phase de destination')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;blocks' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;unblocks' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;task;help' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Ajouter une tâche à une phase')
            [CompletionResult]::new('done', 'done', [CompletionResultType]::ParameterValue, 'Marquer une tâche comme terminée')
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Marquer une tâche comme en cours')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Modifier une tâche')
            [CompletionResult]::new('move', 'move', [CompletionResultType]::ParameterValue, 'Déplacer une tâche vers une autre phase')
            [CompletionResult]::new('blocks', 'blocks', [CompletionResultType]::ParameterValue, 'Définir qu''une tâche en bloque une autre')
            [CompletionResult]::new('unblocks', 'unblocks', [CompletionResultType]::ParameterValue, 'Retirer une dépendance')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'roadmap;task;help;add' {
            break
        }
        'roadmap;task;help;done' {
            break
        }
        'roadmap;task;help;start' {
            break
        }
        'roadmap;task;help;edit' {
            break
        }
        'roadmap;task;help;move' {
            break
        }
        'roadmap;task;help;blocks' {
            break
        }
        'roadmap;task;help;unblocks' {
            break
        }
        'roadmap;task;help;help' {
            break
        }
        'roadmap;priority' {
            [CompletionResult]::new('--set', '--set', [CompletionResultType]::ParameterName, 'Nouvelle priorité (1 = haute)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;status' {
            [CompletionResult]::new('--set', '--set', [CompletionResultType]::ParameterName, 'Nouveau statut (pending, in_progress, done, blocked)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;note' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;export' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;report' {
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Sortie JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;workflow' {
            [CompletionResult]::new('--set', '--set', [CompletionResultType]::ParameterName, 'Forcer une étape spécifique')
            [CompletionResult]::new('--advance', '--advance', [CompletionResultType]::ParameterName, 'Avancer à l''étape suivante')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;ui' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;serve' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Port du serveur (défaut: 7878)')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'Port du serveur (défaut: 7878)')
            [CompletionResult]::new('--open', '--open', [CompletionResultType]::ParameterName, 'Ouvrir le navigateur automatiquement')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;update' {
            [CompletionResult]::new('--check', '--check', [CompletionResultType]::ParameterName, 'Vérifier seulement, sans installer')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;next' {
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Sortie JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;context' {
            [CompletionResult]::new('--include-done', '--include-done', [CompletionResultType]::ParameterName, 'Inclure les tâches terminées')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;scan' {
            [CompletionResult]::new('--glob', '--glob', [CompletionResultType]::ParameterName, 'Pattern glob pour filtrer les fichiers (défaut: **/*.rs)')
            [CompletionResult]::new('--phase', '--phase', [CompletionResultType]::ParameterName, 'Phase cible pour les tâches créées')
            [CompletionResult]::new('--create', '--create', [CompletionResultType]::ParameterName, 'Créer des tâches automatiquement')
            [CompletionResult]::new('--hidden', '--hidden', [CompletionResultType]::ParameterName, 'Inclure les répertoires cachés')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;coverage' {
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Pattern glob pour les fichiers backend (ex: src/api/**/*.ts)')
            [CompletionResult]::new('--backend', '--backend', [CompletionResultType]::ParameterName, 'Pattern glob pour les fichiers backend (ex: src/api/**/*.ts)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Pattern glob pour les fichiers frontend (ex: src/app/**/*.tsx)')
            [CompletionResult]::new('--frontend', '--frontend', [CompletionResultType]::ParameterName, 'Pattern glob pour les fichiers frontend (ex: src/app/**/*.tsx)')
            [CompletionResult]::new('--prefix', '--prefix', [CompletionResultType]::ParameterName, 'Préfixe des routes API (défaut: /api)')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'Sortie JSON')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;changelog' {
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Nombre de commits à inclure (défaut: 50)')
            [CompletionResult]::new('--from', '--from', [CompletionResultType]::ParameterName, 'Tag de départ (ex: v0.1.0)')
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'Tag de fin (défaut: HEAD)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Format de sortie (markdown, json)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'roadmap;generate' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Répertoire de sortie (défaut: ./generated)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Répertoire de sortie (défaut: ./generated)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'roadmap;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialiser une roadmap dans le projet courant')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Ajouter une nouvelle phase')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Modifier une phase')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'Lister toutes les phases')
            [CompletionResult]::new('tree', 'tree', [CompletionResultType]::ParameterValue, 'Afficher l''arborescence complète')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Afficher les détails d''une phase')
            [CompletionResult]::new('task', 'task', [CompletionResultType]::ParameterValue, 'Gérer les tâches')
            [CompletionResult]::new('priority', 'priority', [CompletionResultType]::ParameterValue, 'Changer la priorité d''une phase')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Changer le statut d''une phase')
            [CompletionResult]::new('note', 'note', [CompletionResultType]::ParameterValue, 'Ajouter une note à une phase')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Générer les fichiers Markdown')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Rapport de progression (vue planning)')
            [CompletionResult]::new('workflow', 'workflow', [CompletionResultType]::ParameterValue, 'Gérer le workflow d''une tâche')
            [CompletionResult]::new('ui', 'ui', [CompletionResultType]::ParameterValue, 'Interface interactive (TUI)')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Lancer le serveur web')
            [CompletionResult]::new('update', 'update', [CompletionResultType]::ParameterValue, 'Mettre à jour roadmap-cli')
            [CompletionResult]::new('next', 'next', [CompletionResultType]::ParameterValue, 'Afficher la prochaine tâche à faire')
            [CompletionResult]::new('context', 'context', [CompletionResultType]::ParameterValue, 'Contexte complet pour LLM (optimisé pour AI assistants)')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scanner le code pour trouver les TODO/FIXME')
            [CompletionResult]::new('coverage', 'coverage', [CompletionResultType]::ParameterValue, 'Analyser la couverture API (routes backend vs appels frontend)')
            [CompletionResult]::new('changelog', 'changelog', [CompletionResultType]::ParameterValue, 'Générer un changelog depuis les commits git')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Générer les fichiers man et completions shell')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'roadmap;help;init' {
            break
        }
        'roadmap;help;add' {
            break
        }
        'roadmap;help;edit' {
            break
        }
        'roadmap;help;list' {
            break
        }
        'roadmap;help;tree' {
            break
        }
        'roadmap;help;show' {
            break
        }
        'roadmap;help;task' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Ajouter une tâche à une phase')
            [CompletionResult]::new('done', 'done', [CompletionResultType]::ParameterValue, 'Marquer une tâche comme terminée')
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Marquer une tâche comme en cours')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Modifier une tâche')
            [CompletionResult]::new('move', 'move', [CompletionResultType]::ParameterValue, 'Déplacer une tâche vers une autre phase')
            [CompletionResult]::new('blocks', 'blocks', [CompletionResultType]::ParameterValue, 'Définir qu''une tâche en bloque une autre')
            [CompletionResult]::new('unblocks', 'unblocks', [CompletionResultType]::ParameterValue, 'Retirer une dépendance')
            break
        }
        'roadmap;help;task;add' {
            break
        }
        'roadmap;help;task;done' {
            break
        }
        'roadmap;help;task;start' {
            break
        }
        'roadmap;help;task;edit' {
            break
        }
        'roadmap;help;task;move' {
            break
        }
        'roadmap;help;task;blocks' {
            break
        }
        'roadmap;help;task;unblocks' {
            break
        }
        'roadmap;help;priority' {
            break
        }
        'roadmap;help;status' {
            break
        }
        'roadmap;help;note' {
            break
        }
        'roadmap;help;export' {
            break
        }
        'roadmap;help;report' {
            break
        }
        'roadmap;help;workflow' {
            break
        }
        'roadmap;help;ui' {
            break
        }
        'roadmap;help;serve' {
            break
        }
        'roadmap;help;update' {
            break
        }
        'roadmap;help;next' {
            break
        }
        'roadmap;help;context' {
            break
        }
        'roadmap;help;scan' {
            break
        }
        'roadmap;help;coverage' {
            break
        }
        'roadmap;help;changelog' {
            break
        }
        'roadmap;help;generate' {
            break
        }
        'roadmap;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
