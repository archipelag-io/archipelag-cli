# API Coverage

Which Archipelag.io API endpoints are exposed through the CLI.

## Covered

| Endpoint | CLI Command | Scope |
|----------|-------------|-------|
| `GET /api/v1/account` | `archipelag account` | read |
| `GET /api/v1/jobs` | `archipelag jobs list` | read |
| `GET /api/v1/jobs/:id` | `archipelag jobs get <id>` | read |
| `POST /api/v1/jobs` | `archipelag jobs submit` | write |
| `DELETE /api/v1/jobs/:id` | `archipelag jobs cancel <id>` | write |
| `GET /api/v1/jobs/:id/stream` | `archipelag jobs stream <id>` | read |
| `POST /api/v1/chat/completions` | `archipelag chat` | write |
| `GET /api/v1/workloads` | `archipelag workloads list` | read |
| `GET /api/v1/workloads/:slug` | `archipelag workloads get <slug>` | read |
| `GET /api/v1/hosts` | `archipelag hosts list` | read |
| `GET /api/v1/hosts/:id` | `archipelag hosts get <id>` | read |
| `GET /api/v1/api-keys` | `archipelag api-keys list` | read |
| `POST /api/v1/api-keys` | `archipelag api-keys create <name>` | write |
| `DELETE /api/v1/api-keys/:id` | `archipelag api-keys delete <id>` | write |
| `GET /api/v1/market/rates` | `archipelag market rates` | public |
| `GET /api/v1/market/rates/:slug` | `archipelag market rates <slug>` | public |
| `GET /api/v1/market/history/:slug` | `archipelag market history <slug>` | public |

## Not Yet Covered

These endpoints exist in the API but don't have CLI commands yet. Good candidates for contribution.

### High priority

| Endpoint | Description | Notes |
|----------|-------------|-------|
| `GET /api/v1/jobs/:id/output` | Get full job output | Non-streaming alternative to `jobs stream` |
| `GET /api/v1/hosts/:id/karma` | Host karma score | Could add as `hosts karma <id>` |
| `GET /api/v1/hosts/:id/karma/history` | Karma history | Could add as `hosts karma-history <id>` |
| `GET /api/v1/verification/status` | KYC verification status | Could add as `account verification` |
| `GET /api/v1/models` | List available models | Could add as `models list` |

### Marketplace

| Endpoint | Description |
|----------|-------------|
| `GET /api/v1/marketplace/` | Browse marketplace |
| `GET /api/v1/marketplace/categories` | List categories |
| `GET /api/v1/marketplace/:slug` | Workload details |
| `GET /api/v1/marketplace/:slug/reviews` | Workload reviews |
| `POST /api/v1/marketplace/submissions` | Create submission |
| `PUT /api/v1/marketplace/submissions/:id` | Edit submission |
| `POST /api/v1/marketplace/submissions/:id/submit` | Submit for review |

### Host management (for Island operators)

| Endpoint | Description |
|----------|-------------|
| `POST /api/v1/market/asking-prices` | Set asking prices |

### Admin (would need admin auth)

| Endpoint | Description |
|----------|-------------|
| `GET /api/v1/admin/hosts` | List all hosts |
| `POST /api/v1/admin/hosts/:id/approve` | Approve host |
| `POST /api/v1/admin/hosts/:id/suspend` | Suspend host |
| Various admin endpoints | KYC, marketplace review, etc. |

## NATS

The `archipelag nats subscribe` command provides raw access to any NATS subject. This covers all real-time messaging without needing individual commands for each subject pattern.
