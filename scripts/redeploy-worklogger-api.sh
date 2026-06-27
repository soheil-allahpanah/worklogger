#!/usr/bin/env bash
set -euo pipefail

# Rebuild worklogger-api, replace the running container, and prune the old image.
#
# Scenarios (DATABASE_URL host + optional flags):
#
#   Remote DB:
#     DATABASE_URL='postgres://USER:PASS@db.example.com:5432/mydb' ./scripts/redeploy-worklogger-api.sh
#
#   DB in another Docker container (same network; use the DB container name as host):
#     DOCKER_NETWORK=worklogger-net DATABASE_URL='postgres://USER:PASS@pg:5432/mydb' ./scripts/redeploy-worklogger-api.sh
#
#   DB on the host machine (Postgres not in Docker):
#     USE_HOST_DB=1 DATABASE_URL='postgres://USER:PASS@127.0.0.1:5432/mydb' ./scripts/redeploy-worklogger-api.sh
#     (127.0.0.1 / localhost in DATABASE_URL are rewritten to host.docker.internal)
#
#   Env file (app vars only; networking flags stay as shell env):
#     ENV_FILE=./api.env DOCKER_NETWORK=worklogger-net ./scripts/redeploy-worklogger-api.sh
#
# Required runtime vars: DATABASE_URL, JWT_SECRET (≥32 chars). Both can live in ENV_FILE or the shell.

# --- Image / container ---
IMAGE_NAME="${IMAGE_NAME:-worklogger-api}"
IMAGE_TAG="${IMAGE_TAG:-latest}"
CONTAINER_NAME="${CONTAINER_NAME:-worklogger-api}"
HOST_PORT="${HOST_PORT:-3000}"
CONTAINER_PORT="${CONTAINER_PORT:-3000}"

# --- Database / networking ---
# Path to a file with runtime env vars (e.g. DATABASE_URL, JWT_SECRET). Do not put DOCKER_NETWORK here.
ENV_FILE="${ENV_FILE:-}"
# Attach the API container to this Docker network (for a DB running in another container).
DOCKER_NETWORK="${DOCKER_NETWORK:-}"
# Create DOCKER_NETWORK if it does not exist (set to 1 to enable).
DOCKER_ENSURE_NETWORK="${DOCKER_ENSURE_NETWORK:-0}"
# Set to 1 when Postgres runs on the host: adds host.docker.internal and rewrites localhost in DATABASE_URL.
USE_HOST_DB="${USE_HOST_DB:-0}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

FULL_IMAGE="${IMAGE_NAME}:${IMAGE_TAG}"

read_env_value_from_file() {
  local file="$1"
  local want_key="$2"
  local line key value
  while IFS= read -r line || [[ -n "$line" ]]; do
    line="${line%%#*}"
    line="${line#"${line%%[![:space:]]*}"}"
    line="${line%"${line##*[![:space:]]}"}"
    [[ -z "$line" ]] && continue
    if [[ "$line" != *=* ]]; then
      continue
    fi
    key="${line%%=*}"
    value="${line#*=}"
    key="${key%"${key##*[![:space:]]}"}"
    if [[ "$key" == "$want_key" ]]; then
      # Strip optional surrounding quotes.
      value="${value#\"}"
      value="${value%\"}"
      value="${value#\'}"
      value="${value%\'}"
      printf '%s' "$value"
      return 0
    fi
  done < "$file"
  return 1
}

resolve_env_value() {
  local var_name="$1"
  local shell_value="${!var_name:-}"

  if [[ -n "$shell_value" ]]; then
    printf '%s' "$shell_value"
    return 0
  fi
  if [[ -n "${ENV_FILE}" ]]; then
    read_env_value_from_file "${ENV_FILE}" "$var_name"
    return $?
  fi
  return 1
}

resolve_database_url() {
  resolve_env_value DATABASE_URL
}

rewrite_database_url_for_host_db() {
  local url="$1"
  url="${url//127.0.0.1/host.docker.internal}"
  url="${url//localhost/host.docker.internal}"
  printf '%s' "$url"
}

is_truthy() {
  case "${1,,}" in
    1 | true | yes | on) return 0 ;;
    *) return 1 ;;
  esac
}

if ! RESOLVED_DATABASE_URL="$(resolve_database_url)"; then
  echo "Error: set DATABASE_URL or point ENV_FILE at a file containing it." >&2
  echo "  export DATABASE_URL='postgres://USER:PASSWORD@db-host:5432/worklog'" >&2
  echo "  or: ENV_FILE=/path/to/api.env $0" >&2
  exit 1
fi

if ! RESOLVED_JWT_SECRET="$(resolve_env_value JWT_SECRET)"; then
  echo "Error: set JWT_SECRET or point ENV_FILE at a file containing it." >&2
  echo "  export JWT_SECRET='your-long-random-secret-at-least-32-chars'" >&2
  echo "  or: ENV_FILE=/path/to/api.env $0" >&2
  exit 1
fi

if ((${#RESOLVED_JWT_SECRET} < 32)); then
  echo "Error: JWT_SECRET must be at least 32 characters." >&2
  exit 1
fi

if [[ -n "${ENV_FILE}" && ! -f "${ENV_FILE}" ]]; then
  echo "Error: ENV_FILE not found: ${ENV_FILE}" >&2
  exit 1
fi

if is_truthy "${USE_HOST_DB}"; then
  RESOLVED_DATABASE_URL="$(rewrite_database_url_for_host_db "${RESOLVED_DATABASE_URL}")"
fi

if is_truthy "${DOCKER_ENSURE_NETWORK}" && [[ -n "${DOCKER_NETWORK}" ]]; then
  if ! docker network inspect "${DOCKER_NETWORK}" &>/dev/null; then
    echo "==> Creating Docker network ${DOCKER_NETWORK}..."
    docker network create "${DOCKER_NETWORK}"
  fi
fi

OLD_IMAGE_ID=""
if docker image inspect "${FULL_IMAGE}" &>/dev/null; then
  OLD_IMAGE_ID="$(docker image inspect -f '{{.Id}}' "${FULL_IMAGE}")"
fi

echo "==> Building ${FULL_IMAGE}..."
docker build -f api/Dockerfile -t "${FULL_IMAGE}" .

NEW_IMAGE_ID="$(docker image inspect -f '{{.Id}}' "${FULL_IMAGE}")"

echo "==> Stopping and removing old container (if any)..."
docker stop "${CONTAINER_NAME}" 2>/dev/null || true
docker rm "${CONTAINER_NAME}" 2>/dev/null || true

echo "==> Starting new container..."
RUN_ARGS=(
  -d
  --name "${CONTAINER_NAME}"
  --restart unless-stopped
  -p "${HOST_PORT}:${CONTAINER_PORT}"
)

if [[ -n "${ENV_FILE}" ]]; then
  RUN_ARGS+=(--env-file "${ENV_FILE}")
fi

# After --env-file so rewritten / shell values win over the file when both are set.
RUN_ARGS+=(-e "DATABASE_URL=${RESOLVED_DATABASE_URL}")
RUN_ARGS+=(-e "JWT_SECRET=${RESOLVED_JWT_SECRET}")

if [[ -n "${DOCKER_NETWORK}" ]]; then
  RUN_ARGS+=(--network "${DOCKER_NETWORK}")
fi

if is_truthy "${USE_HOST_DB}"; then
  RUN_ARGS+=(--add-host=host.docker.internal:host-gateway)
fi

docker run "${RUN_ARGS[@]}" "${FULL_IMAGE}"

if [[ -n "${OLD_IMAGE_ID}" && "${OLD_IMAGE_ID}" != "${NEW_IMAGE_ID}" ]]; then
  echo "==> Removing previous image..."
  docker rmi "${OLD_IMAGE_ID}" || true
fi

docker image prune -f >/dev/null || true

echo "==> Done. Container status:"
docker ps --filter "name=${CONTAINER_NAME}"
