# roadmap-cli - Roadmap

CLI pour gérer les roadmaps projet

---

## Phases

| Phase | Nom | Statut | Priorité | Progression |
|-------|-----|--------|----------|-------------|
| 1 | MVP | ✅ | 1 | 4/4 |
| 2 | Interface & Formats | ✅ | 2 | 5/5 |
| 3 | Documentation | ✅ | 3 | 6/6 |
| 4 | Distribution | ✅ | 4 | 6/7 |
| 5 | Améliorations | ⬜ | 5 | 0/5 |
| 7 | Interface Web | ⬜ | 6 | 0/12 |
| 6 | Intégrations | ⬜ | 10 | 0/4 |

---

## Phase 1 — MVP ✅

### Tâches

- ✅ **1.1** — Commandes de base (init, add, list, show)
- ✅ **1.2** — Gestion des tâches (add, done, start)
- ✅ **1.3** — Export Markdown
- ✅ **1.4** — Sous-phases et sous-tâches

### Notes

- **2026-02-23** : MVP développé en une session avec Claude

---

## Phase 2 — Interface & Formats ✅

### Tâches

- ✅ **2.1** — TUI interactif (ratatui)
- ✅ **2.2** — Sortie tableau (tabled)
- ✅ **2.3** — Sortie JSON
- ✅ **2.4** — Vue arborescente (tree)
- ✅ **2.5** — Rapport de progression

### Notes

- **2026-02-23** : TUI avec ratatui, tables avec tabled

---

## Phase 3 — Documentation ✅

### Tâches

- ✅ **3.1** — README.md
- ✅ **3.2** — CLAUDE.md (instructions AI)
- ✅ **3.3** — Guide de démarrage
- ✅ **3.4** — Référence des commandes
- ✅ **3.5** — Format des données
- ✅ **3.6** — Exemples

---

## Phase 4 — Distribution ✅

### Tâches

- ✅ **4.1** — Build release
- ✅ **4.2** — Script d'installation
- ✅ **4.3** — GitHub Actions (CI/CD)
- ✅ **4.4** — Homebrew formula
- ✅ **4.5** — Créer repo GitHub
- ✅ **4.6** — Première release (v0.1.0)
- ⬜ **4.7** — Créer homebrew-tap

### Notes

- **2026-02-23** : Binaire 2.3 MB, installé localement

---

## Phase 5 — Améliorations ⬜

### Tâches

- ⬜ **5.1** — Recherche (search)
- ⬜ **5.2** — Filtres (filter)
- ⬜ **5.3** — Assignation (--assignee)
- ⬜ **5.4** — Dépendances (--blocks)
- ⬜ **5.5** — Due dates (--due)

---

## Phase 7 — Interface Web ⬜

### Tâches

- ⬜ **7.1** — Setup serveur axum
- ⬜ **7.2** — Intégration rust-embed
- ⬜ **7.3** — Templates Askama
- ⬜ **7.4** — Vue Kanban
- ⬜ **7.5** — Vue phases
- ⬜ **7.6** — Page détail phase
- ⬜ **7.7** — Commande roadmap serve
- ⬜ **7.8** — Drag & drop HTMX
- ⬜ **7.9** — Édition inline *(optionnel)*
- ⬜ **7.10** — Création tâches *(optionnel)*
- ⬜ **7.11** — Mode lecture seule *(optionnel)*
- ⬜ **7.12** — Auto-refresh WebSocket *(optionnel)*

### Notes

- **2026-03-05** : Stack: axum + HTMX + Askama + rust-embed

---

## Phase 6 — Intégrations ⬜

### Tâches

- ⬜ **6.1** — Import ROADMAP.md existant *(optionnel)*
- ⬜ **6.2** — Sync GitHub Issues *(optionnel)*
- ⬜ **6.3** — Git hooks (auto-export) *(optionnel)*
- ⬜ **6.4** — Templates de phase *(optionnel)*

---

