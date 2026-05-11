//! API Coverage command

use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use colored::Colorize;
use regex::Regex;

#[derive(Debug, Clone, serde::Serialize)]
struct ApiRoute {
    method: String,
    path: String,
    file: String,
    line: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ApiCall {
    method: String,
    path: String,
    file: String,
    line: usize,
}

fn normalize_path(path: &str) -> String {
    let path = path.split('?').next().unwrap_or(path);
    let re_colon = Regex::new(r"/:([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let re_bracket = Regex::new(r"/\[([a-zA-Z_][a-zA-Z0-9_]*)\]").unwrap();
    let re_brace = Regex::new(r"/\{([a-zA-Z_][a-zA-Z0-9_]*)\}").unwrap();

    let path = re_colon.replace_all(path, "/:param");
    let path = re_bracket.replace_all(&path, "/:param");
    let path = re_brace.replace_all(&path, "/:param");

    path.to_string()
}

/// Extract route path from Next.js App Router file path
/// e.g., src/app/api/users/[id]/route.ts -> /api/users/:param
fn extract_nextjs_route_path(file_path: &str) -> Option<String> {
    // Find the api/ part in the path
    let api_start = file_path.find("/api/")?;
    let route_part = &file_path[api_start..];

    // Remove route.ts or route.js suffix
    let route_part = route_part
        .strip_suffix("/route.ts")
        .or_else(|| route_part.strip_suffix("/route.tsx"))
        .or_else(|| route_part.strip_suffix("/route.js"))
        .or_else(|| route_part.strip_suffix("/route.jsx"))?;

    Some(normalize_path(route_part))
}

/// Scan for Next.js App Router route handlers
fn scan_nextjs_routes(pattern: &str) -> Vec<ApiRoute> {
    let mut routes = Vec::new();

    // Pattern for exported HTTP method handlers in Next.js
    let method_pattern = Regex::new(
        r"export\s+(?:async\s+)?function\s+(GET|POST|PUT|PATCH|DELETE|HEAD|OPTIONS)"
    ).unwrap();

    let glob_pattern = if pattern.starts_with('/') || pattern.starts_with('.') {
        pattern.to_string()
    } else {
        format!("./{}", pattern)
    };

    if let Ok(entries) = glob::glob(&glob_pattern) {
        for entry in entries.flatten() {
            if !entry.is_file() {
                continue;
            }

            let file_path = entry.to_string_lossy().to_string();

            // Extract route path from file location
            let route_path = match extract_nextjs_route_path(&file_path) {
                Some(p) => p,
                None => continue,
            };

            // Scan file for exported methods
            if let Ok(content) = fs::read_to_string(&entry) {
                for (line_num, line) in content.lines().enumerate() {
                    for cap in method_pattern.captures_iter(line) {
                        if let Some(method) = cap.get(1) {
                            routes.push(ApiRoute {
                                method: method.as_str().to_uppercase(),
                                path: route_path.clone(),
                                file: file_path.clone(),
                                line: line_num + 1,
                            });
                        }
                    }
                }
            }
        }
    }

    routes
}

/// Scan for API calls (fetch, axios, etc.)
fn scan_api_calls(pattern: &str, api_prefix: &str) -> Vec<ApiCall> {
    let mut calls = Vec::new();

    let call_patterns = [
        Regex::new(r#"fetch\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(),
        Regex::new(r#"fetch\s*\(\s*`[^`]*(/api/[^`]+)`"#).unwrap(),
        Regex::new(r#"axios\.(get|post|put|patch|delete|head|options)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(),
        Regex::new(r#"axios\s*\(\s*\{[^}]*url:\s*['"`]([^'"`]+)['"`][^}]*method:\s*['"`](\w+)['"`]"#).unwrap(),
        Regex::new(r#"\$http\.(get|post|put|patch|delete)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(),
        Regex::new(r#"['"`](/api/[a-zA-Z0-9_/\-:]+)['"`]"#).unwrap(),
        Regex::new(r#"ky\.(get|post|put|patch|delete)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(),
    ];

    let glob_pattern = if pattern.starts_with('/') || pattern.starts_with('.') {
        pattern.to_string()
    } else {
        format!("./{}", pattern)
    };

    if let Ok(entries) = glob::glob(&glob_pattern) {
        for entry in entries.flatten() {
            if !entry.is_file() {
                continue;
            }

            if let Ok(file) = fs::File::open(&entry) {
                let reader = BufReader::new(file);
                for (line_num, line_result) in reader.lines().enumerate() {
                    if let Ok(line) = line_result {
                        for pattern in &call_patterns {
                            for cap in pattern.captures_iter(&line) {
                                let (method, path) = if cap.len() == 3 {
                                    (cap.get(1).map(|m| m.as_str().to_uppercase()).unwrap_or_else(|| "GET".to_string()),
                                     cap.get(2).map(|m| m.as_str()).unwrap_or_default())
                                } else if cap.len() == 2 {
                                    ("GET".to_string(), cap.get(1).map(|m| m.as_str()).unwrap_or_default())
                                } else {
                                    continue;
                                };

                                if path.starts_with(api_prefix) {
                                    calls.push(ApiCall {
                                        method: method.to_uppercase(),
                                        path: normalize_path(path),
                                        file: entry.to_string_lossy().to_string(),
                                        line: line_num + 1,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    calls
}

/// Scan for backend route definitions
fn scan_backend_routes(pattern: &str, api_prefix: &str) -> Vec<ApiRoute> {
    let mut routes = Vec::new();

    let route_patterns: Vec<(Regex, bool)> = vec![
        // Express.js / Node.js
        (Regex::new(r#"(?:app|router|server)\.(get|post|put|patch|delete|head|options)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(), false),
        // Fastify
        (Regex::new(r#"fastify\.(get|post|put|patch|delete|head|options)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(), false),
        // NestJS
        (Regex::new(r#"@(Get|Post|Put|Patch|Delete|Head|Options)\s*\(\s*['"`]([^'"`]*)['"`]\s*\)"#).unwrap(), false),
        // Hono
        (Regex::new(r#"\.(?:get|post|put|patch|delete|head|options)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(), false),
        // Python Flask/FastAPI
        (Regex::new(r#"@(?:app|router|blueprint)\.(route|get|post|put|patch|delete)\s*\(\s*['"]([^'"]+)['"]\s*(?:,|\))"#).unwrap(), false),
        // Go Gin/Chi
        (Regex::new(r#"(?:r|router|mux|e|app)\.(GET|POST|PUT|PATCH|DELETE|Get|Post|Put|Patch|Delete)\s*\(\s*"([^"]+)""#).unwrap(), false),
        // Rust Axum
        (Regex::new(r#"\.route\s*\(\s*"([^"]+)"\s*,\s*(get|post|put|patch|delete|head|options)"#).unwrap(), true),
        (Regex::new(r#"\.route\s*\(\s*"([^"]+)"\s*,\s*(?:get|post|put|patch|delete)\s*\(\s*\w+\s*\)\s*\.(get|post|put|patch|delete)"#).unwrap(), true),
        // Rust Actix-web
        (Regex::new(r#"\.route\s*\(\s*"([^"]+)"\s*,\s*web::(get|post|put|patch|delete)\s*\(\s*\)"#).unwrap(), true),
        (Regex::new(r#"web::resource\s*\(\s*"([^"]+)"\s*\).*web::(get|post|put|patch|delete)"#).unwrap(), true),
        (Regex::new(r#"\.service\s*\(\s*web::resource\s*\(\s*"([^"]+)"\s*\)"#).unwrap(), true),
    ];

    let glob_pattern = if pattern.starts_with('/') || pattern.starts_with('.') {
        pattern.to_string()
    } else {
        format!("./{}", pattern)
    };

    if let Ok(entries) = glob::glob(&glob_pattern) {
        for entry in entries.flatten() {
            if !entry.is_file() {
                continue;
            }

            if let Ok(file) = fs::File::open(&entry) {
                let reader = BufReader::new(file);
                for (line_num, line_result) in reader.lines().enumerate() {
                    if let Ok(line) = line_result {
                        for (pattern, reversed) in &route_patterns {
                            for cap in pattern.captures_iter(&line) {
                                let (method, path) = if cap.len() == 3 {
                                    if *reversed {
                                        (cap.get(2).map(|m| m.as_str().to_uppercase()).unwrap_or_else(|| "GET".to_string()),
                                         cap.get(1).map(|m| m.as_str()).unwrap_or_default())
                                    } else {
                                        (cap.get(1).map(|m| m.as_str().to_uppercase()).unwrap_or_default(),
                                         cap.get(2).map(|m| m.as_str()).unwrap_or_default())
                                    }
                                } else if cap.len() == 2 {
                                    ("GET".to_string(), cap.get(1).map(|m| m.as_str()).unwrap_or_default())
                                } else {
                                    continue;
                                };

                                if path.starts_with(api_prefix) || api_prefix == "/" || path.starts_with('/') {
                                    routes.push(ApiRoute {
                                        method: method.to_uppercase(),
                                        path: normalize_path(path),
                                        file: entry.to_string_lossy().to_string(),
                                        line: line_num + 1,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    routes
}

/// Scan BFF layer for outgoing API calls (to backend)
fn scan_bff_outgoing_calls(pattern: &str, backend_prefix: &str) -> Vec<ApiCall> {
    let mut calls = Vec::new();

    // Patterns for backend API calls from BFF layer
    let call_patterns = [
        // fetch with full URL or relative path
        Regex::new(r#"fetch\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(),
        Regex::new(r#"fetch\s*\(\s*`([^`]+)`"#).unwrap(),
        // axios
        Regex::new(r#"axios\.(get|post|put|patch|delete|head|options)\s*\(\s*['"`]([^'"`]+)['"`]"#).unwrap(),
        // Generic URL patterns for backend calls
        Regex::new(r#"['"`](https?://[^'"`]+/api/[^'"`]+)['"`]"#).unwrap(),
        // Environment variable based URLs
        Regex::new(r#"\$\{?(?:process\.env\.)?(?:BACKEND_URL|API_URL|INTERNAL_API)[^}]*\}?[`]?(/[a-zA-Z0-9_/\-:]+)"#).unwrap(),
    ];

    let glob_pattern = if pattern.starts_with('/') || pattern.starts_with('.') {
        pattern.to_string()
    } else {
        format!("./{}", pattern)
    };

    if let Ok(entries) = glob::glob(&glob_pattern) {
        for entry in entries.flatten() {
            if !entry.is_file() {
                continue;
            }

            if let Ok(file) = fs::File::open(&entry) {
                let reader = BufReader::new(file);
                for (line_num, line_result) in reader.lines().enumerate() {
                    if let Ok(line) = line_result {
                        for pattern in &call_patterns {
                            for cap in pattern.captures_iter(&line) {
                                let (method, path) = if cap.len() == 3 {
                                    (cap.get(1).map(|m| m.as_str().to_uppercase()).unwrap_or_else(|| "GET".to_string()),
                                     cap.get(2).map(|m| m.as_str()).unwrap_or_default())
                                } else if cap.len() == 2 {
                                    ("GET".to_string(), cap.get(1).map(|m| m.as_str()).unwrap_or_default())
                                } else {
                                    continue;
                                };

                                // Extract path from full URL if needed
                                let path = if path.starts_with("http") {
                                    // Extract path from URL
                                    if let Some(idx) = path.find("/api/") {
                                        &path[idx..]
                                    } else if let Some(idx) = path.find("://") {
                                        let after_proto = &path[idx + 3..];
                                        if let Some(slash_idx) = after_proto.find('/') {
                                            &after_proto[slash_idx..]
                                        } else {
                                            continue;
                                        }
                                    } else {
                                        continue;
                                    }
                                } else {
                                    path
                                };

                                if path.starts_with(backend_prefix) || path.starts_with("/api/") {
                                    calls.push(ApiCall {
                                        method: method.to_uppercase(),
                                        path: normalize_path(path),
                                        file: entry.to_string_lossy().to_string(),
                                        line: line_num + 1,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    calls
}

pub fn cmd_coverage(
    backend_pattern: String,
    frontend_pattern: String,
    bff_pattern: Option<String>,
    api_prefix: String,
    backend_prefix: Option<String>,
    json_output: bool,
) {
    let backend_prefix = backend_prefix.unwrap_or_else(|| api_prefix.clone());

    // Scan backend routes
    let backend_routes = scan_backend_routes(&backend_pattern, &backend_prefix);

    // Scan frontend calls
    let frontend_calls = scan_api_calls(&frontend_pattern, &api_prefix);

    // If BFF layer is specified, use 3-layer analysis
    if let Some(ref bff) = bff_pattern {
        let bff_routes = scan_nextjs_routes(bff);
        let bff_calls = scan_bff_outgoing_calls(bff, &backend_prefix);

        output_3layer_report(
            &frontend_calls,
            &bff_routes,
            &bff_calls,
            &backend_routes,
            json_output,
        );
    } else {
        // Traditional 2-layer analysis
        output_2layer_report(&frontend_calls, &backend_routes, json_output);
    }
}

fn output_2layer_report(calls: &[ApiCall], routes: &[ApiRoute], json_output: bool) {
    let route_set: HashSet<String> = routes.iter()
        .map(|r| format!("{} {}", r.method, r.path))
        .collect();

    let call_set: HashSet<String> = calls.iter()
        .map(|c| format!("{} {}", c.method, c.path))
        .collect();

    let unused: Vec<&ApiRoute> = routes.iter()
        .filter(|r| !call_set.contains(&format!("{} {}", r.method, r.path)))
        .collect();

    let missing: Vec<&ApiCall> = calls.iter()
        .filter(|c| !route_set.contains(&format!("{} {}", c.method, c.path)))
        .collect();

    let unique_unused: HashMap<String, &ApiRoute> = unused.iter()
        .map(|r| (format!("{} {}", r.method, r.path), *r))
        .collect();

    let unique_missing: HashMap<String, &ApiCall> = missing.iter()
        .map(|c| (format!("{} {}", c.method, c.path), *c))
        .collect();

    if json_output {
        #[derive(serde::Serialize)]
        struct CoverageReport {
            mode: String,
            summary: CoverageSummary,
            routes: Vec<ApiRoute>,
            calls: Vec<ApiCall>,
            unused_routes: Vec<ApiRoute>,
            missing_routes: Vec<ApiCall>,
        }

        #[derive(serde::Serialize)]
        struct CoverageSummary {
            total_routes: usize,
            total_calls: usize,
            unused_count: usize,
            missing_count: usize,
            coverage_percent: f64,
        }

        let coverage = if routes.is_empty() {
            0.0
        } else {
            let used = routes.len() - unique_unused.len();
            (used as f64 / routes.len() as f64) * 100.0
        };

        let report = CoverageReport {
            mode: "2-layer".to_string(),
            summary: CoverageSummary {
                total_routes: routes.len(),
                total_calls: calls.len(),
                unused_count: unique_unused.len(),
                missing_count: unique_missing.len(),
                coverage_percent: (coverage * 100.0).round() / 100.0,
            },
            routes: routes.to_vec(),
            calls: calls.to_vec(),
            unused_routes: unique_unused.values().cloned().cloned().collect(),
            missing_routes: unique_missing.values().cloned().cloned().collect(),
        };

        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return;
    }

    println!();
    println!("{}", "📊 API Coverage Report (2-layer)".bold());
    println!();

    let coverage = if routes.is_empty() {
        0.0
    } else {
        let used = routes.len() - unique_unused.len();
        (used as f64 / routes.len() as f64) * 100.0
    };

    println!("  {} routes backend trouvées", routes.len().to_string().cyan());
    println!("  {} appels frontend trouvés", calls.len().to_string().cyan());
    println!("  {} couverture API", format!("{:.1}%", coverage).green().bold());
    println!();

    if !unique_unused.is_empty() {
        println!("{}", "⚠️  Routes non utilisées (backend sans appels frontend)".yellow().bold());
        println!();
        for (endpoint, route) in &unique_unused {
            let short_file = route.file.strip_prefix("./").unwrap_or(&route.file);
            println!("   {} {}", endpoint.red(), format!("({}:{})", short_file, route.line).dimmed());
        }
        println!();
    }

    if !unique_missing.is_empty() {
        println!("{}", "❌ Routes manquantes (appels frontend sans backend)".red().bold());
        println!();
        for (endpoint, call) in &unique_missing {
            let short_file = call.file.strip_prefix("./").unwrap_or(&call.file);
            println!("   {} {}", endpoint.yellow(), format!("({}:{})", short_file, call.line).dimmed());
        }
    }
}

fn output_3layer_report(
    frontend_calls: &[ApiCall],
    bff_routes: &[ApiRoute],
    bff_calls: &[ApiCall],
    backend_routes: &[ApiRoute],
    json_output: bool,
) {
    // Layer 1: Frontend -> BFF
    let bff_route_set: HashSet<String> = bff_routes.iter()
        .map(|r| format!("{} {}", r.method, r.path))
        .collect();

    let frontend_call_set: HashSet<String> = frontend_calls.iter()
        .map(|c| format!("{} {}", c.method, c.path))
        .collect();

    let unused_bff: Vec<&ApiRoute> = bff_routes.iter()
        .filter(|r| !frontend_call_set.contains(&format!("{} {}", r.method, r.path)))
        .collect();

    let missing_bff: Vec<&ApiCall> = frontend_calls.iter()
        .filter(|c| !bff_route_set.contains(&format!("{} {}", c.method, c.path)))
        .collect();

    // Layer 2: BFF -> Backend
    let backend_route_set: HashSet<String> = backend_routes.iter()
        .map(|r| format!("{} {}", r.method, r.path))
        .collect();

    let bff_call_set: HashSet<String> = bff_calls.iter()
        .map(|c| format!("{} {}", c.method, c.path))
        .collect();

    let unused_backend: Vec<&ApiRoute> = backend_routes.iter()
        .filter(|r| !bff_call_set.contains(&format!("{} {}", r.method, r.path)))
        .collect();

    let missing_backend: Vec<&ApiCall> = bff_calls.iter()
        .filter(|c| !backend_route_set.contains(&format!("{} {}", c.method, c.path)))
        .collect();

    // Deduplicate
    let unique_unused_bff: HashMap<String, &ApiRoute> = unused_bff.iter()
        .map(|r| (format!("{} {}", r.method, r.path), *r))
        .collect();

    let unique_missing_bff: HashMap<String, &ApiCall> = missing_bff.iter()
        .map(|c| (format!("{} {}", c.method, c.path), *c))
        .collect();

    let unique_unused_backend: HashMap<String, &ApiRoute> = unused_backend.iter()
        .map(|r| (format!("{} {}", r.method, r.path), *r))
        .collect();

    let unique_missing_backend: HashMap<String, &ApiCall> = missing_backend.iter()
        .map(|c| (format!("{} {}", c.method, c.path), *c))
        .collect();

    // Calculate coverage
    let bff_coverage = if bff_routes.is_empty() {
        0.0
    } else {
        let used = bff_routes.len() - unique_unused_bff.len();
        (used as f64 / bff_routes.len() as f64) * 100.0
    };

    let backend_coverage = if backend_routes.is_empty() {
        0.0
    } else {
        let used = backend_routes.len() - unique_unused_backend.len();
        (used as f64 / backend_routes.len() as f64) * 100.0
    };

    if json_output {
        #[derive(serde::Serialize)]
        struct ThreeLayerReport {
            mode: String,
            frontend_to_bff: LayerReport,
            bff_to_backend: LayerReport,
        }

        #[derive(serde::Serialize)]
        struct LayerReport {
            source_calls: Vec<ApiCall>,
            target_routes: Vec<ApiRoute>,
            unused_routes: Vec<ApiRoute>,
            missing_routes: Vec<ApiCall>,
            coverage_percent: f64,
        }

        let report = ThreeLayerReport {
            mode: "3-layer".to_string(),
            frontend_to_bff: LayerReport {
                source_calls: frontend_calls.to_vec(),
                target_routes: bff_routes.to_vec(),
                unused_routes: unique_unused_bff.values().cloned().cloned().collect(),
                missing_routes: unique_missing_bff.values().cloned().cloned().collect(),
                coverage_percent: (bff_coverage * 100.0).round() / 100.0,
            },
            bff_to_backend: LayerReport {
                source_calls: bff_calls.to_vec(),
                target_routes: backend_routes.to_vec(),
                unused_routes: unique_unused_backend.values().cloned().cloned().collect(),
                missing_routes: unique_missing_backend.values().cloned().cloned().collect(),
                coverage_percent: (backend_coverage * 100.0).round() / 100.0,
            },
        };

        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return;
    }

    // Console output
    println!();
    println!("{}", "📊 API Coverage Report (3-layer architecture)".bold());
    println!();

    // Layer 1: Frontend -> BFF
    println!("{}", "━━━ Layer 1: Frontend → BFF ━━━".cyan().bold());
    println!();
    println!("  {} appels frontend", frontend_calls.len().to_string().cyan());
    println!("  {} routes BFF (Next.js)", bff_routes.len().to_string().cyan());
    println!("  {} couverture BFF", format!("{:.1}%", bff_coverage).green().bold());
    println!();

    if !unique_unused_bff.is_empty() {
        println!("  {} Routes BFF non utilisées:", "⚠️".yellow());
        for (endpoint, route) in &unique_unused_bff {
            let short_file = route.file.strip_prefix("./").unwrap_or(&route.file);
            println!("     {} {}", endpoint.red(), format!("({}:{})", short_file, route.line).dimmed());
        }
        println!();
    }

    if !unique_missing_bff.is_empty() {
        println!("  {} Routes BFF manquantes:", "❌".red());
        for (endpoint, call) in &unique_missing_bff {
            let short_file = call.file.strip_prefix("./").unwrap_or(&call.file);
            println!("     {} {}", endpoint.yellow(), format!("({}:{})", short_file, call.line).dimmed());
        }
        println!();
    }

    // Layer 2: BFF -> Backend
    println!("{}", "━━━ Layer 2: BFF → Backend ━━━".cyan().bold());
    println!();
    println!("  {} appels BFF vers backend", bff_calls.len().to_string().cyan());
    println!("  {} routes backend", backend_routes.len().to_string().cyan());
    println!("  {} couverture backend", format!("{:.1}%", backend_coverage).green().bold());
    println!();

    if !unique_unused_backend.is_empty() {
        println!("  {} Routes backend non utilisées:", "⚠️".yellow());
        for (endpoint, route) in &unique_unused_backend {
            let short_file = route.file.strip_prefix("./").unwrap_or(&route.file);
            println!("     {} {}", endpoint.red(), format!("({}:{})", short_file, route.line).dimmed());
        }
        println!();
    }

    if !unique_missing_backend.is_empty() {
        println!("  {} Routes backend manquantes:", "❌".red());
        for (endpoint, call) in &unique_missing_backend {
            let short_file = call.file.strip_prefix("./").unwrap_or(&call.file);
            println!("     {} {}", endpoint.yellow(), format!("({}:{})", short_file, call.line).dimmed());
        }
    }

    // Overall summary
    println!();
    println!("{}", "━━━ Résumé ━━━".cyan().bold());
    println!();
    let total_issues = unique_unused_bff.len() + unique_missing_bff.len()
                     + unique_unused_backend.len() + unique_missing_backend.len();
    if total_issues == 0 {
        println!("  {} Toutes les couches sont correctement connectées!", "✅".green());
    } else {
        println!("  {} {} problèmes détectés au total", "⚠️".yellow(), total_issues);
    }
}
