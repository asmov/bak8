#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(realpath "$(dirname "${BASH_SOURCE[0]}" )")"
MOCK_FS_DIR="$(realpath "${SCRIPT_DIR}/../testing/fixtures/integration/testlib/mock-fs")"

sources=(source-1 source-2)
for source in "${sources[@]}"; do
    cd "${MOCK_FS_DIR}/${source}/home/testusr"
    find . -type d | sort > "${MOCK_FS_DIR}/${source}.manifest"
    find . -type f -exec sha256sum {} \; | sort -k 2 >> "${MOCK_FS_DIR}/${source}.manifest"
done


