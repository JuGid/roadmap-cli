# Référence des commandes

Documentation complète de toutes les commandes roadmap-cli.

---

## Commandes globales

### `roadmap init`

Initialise une roadmap dans le projet courant.

```bash
roadmap init
```

Crée :
- `.phases/config.yml` — Configuration du projet

---

### `roadmap report`

Affiche un rapport de progression.

```bash
roadmap report [--json]
```

| Option | Description |
|--------|-------------|
| `--json` | Sortie JSON pour parsing |

**Exemple :**
```bash
roadmap report
```

**Sortie JSON :**
```json
{
  "summary": {
    "total_phases": 6,
    "phases_done": 3,
    "phases_in_progress": 1,
    "progress_percent": 33.33
  },
  "phases_in_progress": [...],
  "next_tasks": [...],
  "optional_tasks": [...],
  "blocked": { "phases": [], "tasks": [] }
}
```

---

### `roadmap list`

Liste toutes les phases.

```bash
roadmap list [--table] [--json]
```

| Option | Description |
|--------|-------------|
| `--table` | Affichage tableau formaté |
| `--json` | Sortie JSON |

**Exemples :**
```bash
roadmap list           # Liste simple colorée
roadmap list --table   # Tableau ASCII
roadmap list --json    # JSON complet
```

---

### `roadmap tree`

Affiche l'arborescence complète.

```bash
roadmap tree [--json]
```

| Option | Description |
|--------|-------------|
| `--json` | Sortie JSON hiérarchique |

**Sortie JSON :**
```json
[
  {
    "id": "9",
    "name": "Observabilité",
    "type": "phase",
    "status": "in_progress",
    "children": [
      { "id": "9.1", "type": "task", "status": "done", "children": [...] }
    ]
  }
]
```

---

### `roadmap ui`

Lance l'interface interactive (TUI).

```bash
roadmap ui
```

**Raccourcis :**
| Touche | Action |
|--------|--------|
| `↑↓` ou `jk` | Navigation |
| `→` ou `l` | Aller aux tâches |
| `←` ou `h` | Retour aux phases |
| `Tab` / `Enter` | Basculer focus |
| `q` / `Esc` | Quitter |

---

### `roadmap export`

Génère le fichier Markdown.

```bash
roadmap export
```

Le chemin de sortie est défini dans `.phases/config.yml` → `export.roadmap_path`.

---

## Gestion des phases

### `roadmap add`

Crée une nouvelle phase.

```bash
roadmap add <id> <nom> [--parent <phase_id>]
```

| Argument | Description |
|----------|-------------|
| `<id>` | Identifiant de la phase (ex: 9, 10, 9a) |
| `<nom>` | Nom de la phase |
| `--parent` | Phase parente (pour sous-phases) |

**Exemples :**
```bash
roadmap add 9 "Observabilité"
roadmap add 9a "Logs Pino" --parent 9
```

---

### `roadmap show`

Affiche les détails d'une phase.

```bash
roadmap show <id> [--json]
```

| Option | Description |
|--------|-------------|
| `--json` | Sortie JSON complète |

**Exemple :**
```bash
roadmap show 9
```

---

### `roadmap edit`

Modifie une phase.

```bash
roadmap edit <id> [--name <nom>] [--description <desc>]
```

| Option | Description |
|--------|-------------|
| `--name` | Nouveau nom |
| `--description` | Nouvelle description |

**Exemple :**
```bash
roadmap edit 9 --name "Observabilité & Monitoring" --description "Logs, métriques, alerting"
```

---

### `roadmap priority`

Change la priorité d'une phase.

```bash
roadmap priority <id> --set <n>
```

| Option | Description |
|--------|-------------|
| `--set` | Nouvelle priorité (1 = haute) |

**Exemple :**
```bash
roadmap priority 9 --set 1
```

---

### `roadmap status`

Change le statut d'une phase.

```bash
roadmap status <id> --set <statut>
```

| Statut | Description |
|--------|-------------|
| `pending` | À faire |
| `in_progress` | En cours |
| `done` | Terminé |
| `blocked` | Bloqué |

**Exemple :**
```bash
roadmap status 9 --set in_progress
```

---

### `roadmap note`

Ajoute une note à une phase.

```bash
roadmap note <id> <contenu>
```

**Exemple :**
```bash
roadmap note 9 "Décision: utiliser Pino pour la performance"
```

---

## Gestion des tâches

### `roadmap task add`

Ajoute une tâche à une phase.

```bash
roadmap task add <phase_id> <nom> [--parent <task_id>] [--optional]
```

| Option | Description |
|--------|-------------|
| `--parent` | Tâche parente (pour sous-tâches) |
| `--optional` | Marquer comme optionnelle |

**Exemples :**
```bash
roadmap task add 9 "Intégrer Pino"
roadmap task add 9 "Configurer transport" --parent 9.1
roadmap task add 9 "GlitchTip" --optional
```

---

### `roadmap task edit`

Modifie une tâche.

```bash
roadmap task edit <task_id> [--name <nom>] [--description <desc>] [--optional <bool>]
```

| Option | Description |
|--------|-------------|
| `--name` | Nouveau nom |
| `--description` | Nouvelle description |
| `--optional` | `true` ou `false` |

**Exemple :**
```bash
roadmap task edit 9.1 --name "Logs structurés Pino" --description "Remplacer console.error"
```

---

### `roadmap task start`

Marque une tâche comme en cours.

```bash
roadmap task start <task_id>
```

---

### `roadmap task done`

Marque une tâche comme terminée.

```bash
roadmap task done <task_id>
```

---

### `roadmap task move`

Déplace une tâche vers une autre phase.

```bash
roadmap task move <task_id> --to <phase_id>
```

**Exemple :**
```bash
roadmap task move 9.3 --to 10
# ✓ Tâche 9.3 → 10 (nouvel ID: 10.1)
```

---

## Gestion du workflow

### `roadmap workflow`

Gère l'étape workflow d'une tâche.

```bash
roadmap workflow <task_id> --advance
roadmap workflow <task_id> --set <stage>
```

| Option | Description |
|--------|-------------|
| `--advance` | Passe à l'étape suivante |
| `--set` | Force une étape spécifique |

**Étapes par défaut :**
`analysis` → `design` → `implementation` → `testing` → `documentation`

**Exemples :**
```bash
roadmap workflow 9.1 --advance
# → Tâche 9.1 : analysis → design

roadmap workflow 9.1 --set testing
# ✓ Tâche 9.1 : design → testing
```

---

### `roadmap task blocks`

Déclare qu'une tâche bloque une autre tâche.

```bash
roadmap task blocks <task_id> <blocked_task_id>
```

**Exemple :**
```bash
roadmap task blocks 9.1 9.2
# ✓ 9.1 bloque maintenant 9.2
```

La tâche bloquée apparaîtra avec ses dépendances dans `roadmap next` et `roadmap report`.

---

### `roadmap task unblocks`

Retire une dépendance de blocage.

```bash
roadmap task unblocks <task_id> <blocked_task_id>
```

**Exemple :**
```bash
roadmap task unblocks 9.1 9.2
# ✓ 9.1 ne bloque plus 9.2
```

---

## Serveur web

### `roadmap serve`

Lance un serveur web avec interface Kanban.

```bash
roadmap serve [--port <port>] [--open]
```

| Option | Description |
|--------|-------------|
| `--port`, `-p` | Port d'écoute (défaut: 7878) |
| `--open` | Ouvrir dans le navigateur |

**Exemples :**
```bash
roadmap serve                  # Démarre sur http://localhost:7878
roadmap serve --port 8080      # Port personnalisé
roadmap serve --open           # Ouvre automatiquement le navigateur
```

**Routes disponibles :**
| Route | Description |
|-------|-------------|
| `/` | Vue Kanban (4 colonnes par statut) |
| `/phases` | Liste des phases |
| `/phases/{id}` | Détail d'une phase |
| `/api/phases` | API JSON - toutes les phases |
| `/api/phases/{id}` | API JSON - une phase |
| `/api/report` | API JSON - rapport complet |

---

## Mise à jour

### `roadmap update`

Vérifie et installe les mises à jour depuis GitHub Releases.

```bash
roadmap update [--check]
```

| Option | Description |
|--------|-------------|
| `--check` | Vérifier seulement (sans installer) |

**Exemples :**
```bash
roadmap update          # Télécharge et installe la dernière version
roadmap update --check  # Affiche si une mise à jour est disponible
```

---

## Outils LLM

### `roadmap next`

Affiche les prochaines tâches à faire (filtre automatiquement les tâches bloquées).

```bash
roadmap next [--json]
```

| Option | Description |
|--------|-------------|
| `--json` | Sortie JSON |

**Sortie JSON :**
```json
[
  {
    "task_id": "9.2",
    "task_name": "Métriques Prometheus",
    "phase_id": "9",
    "phase_name": "Observabilité",
    "priority": 1,
    "optional": false
  }
]
```

---

### `roadmap context`

Génère un contexte formaté pour les LLM (prompt système).

```bash
roadmap context [--include-done]
```

| Option | Description |
|--------|-------------|
| `--include-done` | Inclure les tâches terminées |

**Exemple de sortie :**
```
# Contexte Roadmap

## État actuel
- Phases: 6 (3 done, 1 in_progress, 2 pending)
- Progression: 33%

## Phase en cours: Observabilité (P1)
- ✅ 9.1 Logs structurés Pino
- ⬜ 9.2 Métriques Prometheus
- ⬜ 9.3 Mode debug utilisateur

## Prochaines tâches
1. [9.2] Métriques Prometheus
2. [9.3] Mode debug utilisateur
```

---

## Intégrations code

### `roadmap scan`

Scanne le code source pour trouver les TODO, FIXME, HACK, etc.

```bash
roadmap scan [--glob <pattern>] [--create] [--phase <id>] [--hidden]
```

| Option | Description |
|--------|-------------|
| `--glob` | Pattern de fichiers (défaut: `**/*.rs`) |
| `--create` | Créer des tâches automatiquement |
| `--phase` | Phase cible pour les tâches créées |
| `--hidden` | Inclure les fichiers/dossiers cachés |

**Exemples :**
```bash
roadmap scan --glob "**/*.ts"                    # Scanner TypeScript
roadmap scan --glob "src/**/*.js" --create --phase 9  # Créer des tâches
```

**Marqueurs détectés :**
`TODO`, `FIXME`, `HACK`, `BUG`, `XXX`, `OPTIMIZE`, `REFACTOR`

---

### `roadmap coverage`

Analyse la couverture API (routes backend vs appels frontend).

```bash
roadmap coverage --backend <pattern> --frontend <pattern> [--prefix <prefix>] [--json]
```

| Option | Description |
|--------|-------------|
| `--backend`, `-b` | Pattern glob fichiers backend |
| `--frontend`, `-f` | Pattern glob fichiers frontend |
| `--prefix` | Préfixe des routes API (défaut: `/api`) |
| `--json` | Sortie JSON |

**Exemples :**
```bash
# Next.js
roadmap coverage -b "src/pages/api/**/*.ts" -f "src/**/*.tsx"

# Express + React
roadmap coverage -b "server/**/*.js" -f "client/src/**/*.js"

# Sortie JSON
roadmap coverage -b "api/**/*.ts" -f "app/**/*.tsx" --json
```

**Frameworks supportés :**
- **Backend** : Express, Fastify, NestJS, Hono, Flask, FastAPI, Gin, Chi, Axum
- **Frontend** : fetch, axios, $http, ky

---

### `roadmap changelog`

Génère un changelog depuis les commits git.

```bash
roadmap changelog [--limit <n>] [--from <tag>] [--to <tag>] [--format <format>]
```

| Option | Description |
|--------|-------------|
| `--limit` | Nombre de commits (défaut: 50) |
| `--from` | Tag de départ (ex: v0.1.0) |
| `--to` | Tag de fin (défaut: HEAD) |
| `--format` | `markdown` ou `json` |

**Exemples :**
```bash
roadmap changelog                           # 50 derniers commits
roadmap changelog --from v0.1.0             # Depuis un tag
roadmap changelog --from v0.1.0 --to v0.2.0 # Entre deux tags
roadmap changelog --format json             # Sortie JSON
```

**Support Conventional Commits :**
Les commits suivant le format `type(scope): message` sont automatiquement parsés et groupés par type (feat, fix, docs, etc.).

---

## Génération de documentation

### `roadmap generate`

Génère les pages man et les completions shell.

```bash
roadmap generate <TYPE> [--output <dir>]
```

| Argument | Description |
|----------|-------------|
| `man` | Générer les pages man uniquement |
| `completions` | Générer les completions shell uniquement |
| `all` | Générer tout |

| Option | Description |
|--------|-------------|
| `--output`, `-o` | Répertoire de sortie (défaut: `./generated`) |

**Exemples :**
```bash
roadmap generate all                    # Tout générer dans ./generated
roadmap generate man -o ./docs          # Man pages dans ./docs/man
roadmap generate completions            # Completions shell
```

**Installation des man pages :**
```bash
sudo cp generated/man/*.1 /usr/local/share/man/man1/
sudo mandb   # Linux
```

**Installation des completions :**
```bash
# Bash
sudo cp generated/completions/roadmap.bash /etc/bash_completion.d/

# Zsh (ajouter à .zshrc: fpath=(~/.zsh/completions $fpath))
mkdir -p ~/.zsh/completions
cp generated/completions/_roadmap ~/.zsh/completions/

# Fish
cp generated/completions/roadmap.fish ~/.config/fish/completions/
```

---

## Options globales

| Option | Description |
|--------|-------------|
| `--help` | Affiche l'aide |
| `--version` | Affiche la version |

```bash
roadmap --help
roadmap task --help
roadmap task add --help
```
