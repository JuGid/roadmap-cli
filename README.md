# roadmap-cli

**CLI-first project management.** Git for your roadmap.

Gerez vos roadmaps projet en fichiers YAML versionnables, directement dans votre repo.

## Pourquoi roadmap-cli ?

- **Versionnable** -- Les donnees sont des fichiers YAML dans `.phases/`, committes avec votre code
- **CLI-first** -- Pas besoin de quitter le terminal : `roadmap task done 5.1` en 2 secondes
- **Integrable** -- Sortie JSON, contexte LLM, scan TODO/FIXME, sync code / roadmap
- **Leger** -- Binaire unique Rust, pas de runtime, pas de base de donnees requise
- **SaaS optionnel** -- Dashboard web pour les PO/managers, sync cloud pour les equipes

## Installation

```bash
# Script d'installation (macOS/Linux)
curl -fsSL https://raw.githubusercontent.com/Siovos/roadmap-cli/main/install.sh | bash

# Cargo (si Rust installe)
cargo install --git https://github.com/Siovos/roadmap-cli

# Depuis les sources
git clone https://github.com/Siovos/roadmap-cli.git
cd roadmap-cli
cargo build --release
cp target/release/roadmap-cli /usr/local/bin/roadmap
```

## Demarrage rapide

```bash
# Initialiser dans un projet
roadmap init

# Creer des phases et taches
roadmap add 1 "MVP Auth"
roadmap task add 1 "Login page" --assignee sarah --due 2026-06-01
roadmap task add 1 "JWT middleware" --tag backend,security
roadmap task done 1.1

# Voir l'etat
roadmap report
roadmap tree
```

## Apercu

```
Roadmap

+-- [done] Phase 1 -- MVP (P1)
|   +-- [done] 1.1 -- Login page @sarah
|   +-- [in_progress] 1.2 -- JWT middleware
+-- [pending] Phase 2 -- Dashboard (P2)
|   +-- [pending] 2.1 -- Composants UI
|   +-- [pending] 2.2 -- API endpoints
+-- [pending] Phase 3 -- Deploiement (P3)
```

## Commandes

### Gestion de projet

| Commande | Description |
|----------|-------------|
| `roadmap init` | Initialiser une roadmap |
| `roadmap add <id> <nom>` | Creer une phase |
| `roadmap edit <id> --name "..."` | Modifier une phase |
| `roadmap status <id> --set done` | Changer le statut |
| `roadmap priority <id> --set 1` | Changer la priorite |
| `roadmap note <id> "..."` | Ajouter une note |

### Taches

| Commande | Description |
|----------|-------------|
| `roadmap task add <phase> <nom>` | Ajouter une tache |
| `roadmap task done 5.1 5.2 5.3` | Terminer (batch) |
| `roadmap task start 5.1` | Marquer en cours |
| `roadmap task edit 5.1 --desc "..."` | Modifier (+ --tag, --assignee, --due) |
| `roadmap task move 5.1 --to 6` | Deplacer vers une autre phase |

### Vues et rapports

| Commande | Description |
|----------|-------------|
| `roadmap report` | Rapport de progression |
| `roadmap tree` | Vue arborescente |
| `roadmap list --table` | Liste en tableau |
| `roadmap list --tag backend` | Filtrer par tag |
| `roadmap list --status done` | Filtrer par statut |
| `roadmap list --assignee tim` | Filtrer par assignee |
| `roadmap list --overdue` | Taches en retard |
| `roadmap show <id>` | Detail d'une phase |
| `roadmap next` | Prochaine tache intelligente |
| `roadmap search "auth"` | Rechercher partout |
| `roadmap log` | Historique des changements |

### Bugs et features (cross-projet)

| Commande | Description |
|----------|-------------|
| `roadmap bug add "titre" --severity blocking` | Declarer un bug |
| `roadmap bug add "..." --target /path/to/projet` | Bug cross-projet |
| `roadmap bug list --severity blocking` | Lister les bugs |
| `roadmap bug resolve 1 --commit abc123` | Resoudre |
| `roadmap feature add "titre" --priority high` | Demander une feature |
| `roadmap feature add "..." --target /path/to/projet` | Feature cross-projet |
| `roadmap feature implement 1` | Marquer implementee |

### Outils

| Commande | Description |
|----------|-------------|
| `roadmap doctor` | Verifier l'integrite YAML |
| `roadmap sync` | Coherence code / roadmap |
| `roadmap scan --glob "**/*.ts"` | Scanner TODO/FIXME |
| `roadmap export` | Generer ROADMAP.md |
| `roadmap context --phase 5` | Contexte pour LLM |
| `roadmap template feature 10` | Creer depuis un template |
| `roadmap hooks --install` | Git hook auto-export |
| `roadmap changelog` | Changelog depuis git |

### SaaS (optionnel)

| Commande | Description |
|----------|-------------|
| `roadmap login` | Se connecter (ouvre le navigateur) |
| `roadmap push` | Synchroniser vers le cloud |
| `roadmap serve` | Serveur web local (Kanban) |

## Sortie JSON

Toutes les commandes supportent `--json` pour l'integration avec d'autres outils :

```bash
roadmap report --json | jq '.next_tasks'
roadmap list --json | jq '.[] | select(.status == "in_progress")'
roadmap bug list --json
```

## Structure des fichiers

```
projet/
  .phases/
    config.yml       # Configuration projet
    phase-1.yml      # Phase 1
    phase-2.yml      # Phase 2
    bugs.yml         # Bugs/incidents
    features.yml     # Feature requests
  ROADMAP.md         # Genere par roadmap export
```

## License

MIT
