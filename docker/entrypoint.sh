#!/bin/sh
set -euo pipefail

DEFAULT_CONFIG="/app/config.example.json"
CONFIG_PATH="${CONFIG_PATH:-/app/config.json}"
DATA_DIR="${DATA_DIR:-/app/data/blockchain}"
USER_NAME="blockchain"
GROUP_NAME="blockchain"

# Ensure config exists for first run
if [ ! -f "${CONFIG_PATH}" ]; then
    if [ -f "${DEFAULT_CONFIG}" ]; then
        echo "[*] Config file not found at ${CONFIG_PATH}, seeding from template."
        cp "${DEFAULT_CONFIG}" "${CONFIG_PATH}"
    else
        echo "[!] No config file found and no template available; creating empty config.json"
        touch "${CONFIG_PATH}"
    fi
fi

# Ensure data directory exists
mkdir -p "${DATA_DIR}"

# Adjust ownership (ignore errors on read-only volumes)
chown -R "${USER_NAME}:${GROUP_NAME}" "${DATA_DIR}" 2>/dev/null || true
chown "${USER_NAME}:${GROUP_NAME}" "${CONFIG_PATH}" 2>/dev/null || true

# Support running arbitrary commands
CMD="${1:-}"

if [ "${CMD#-}" != "${CMD}" ] || [ -z "${CMD}" ]; then
    set -- blockchain-grpc "$@"
    CMD="${1}"
fi

if [ "${CMD}" = "blockchain-grpc" ]; then
    shift
    exec gosu "${USER_NAME}:${GROUP_NAME}" /usr/local/bin/blockchain-grpc "$@"
fi

exec "$@"
