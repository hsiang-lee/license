#! /bin/bash

set -e

mkdir Release

cp build/src/libsmlicense.a build/app/checker/license_check \
    build/app/key_gen/key_gen build/app/license_gen/license_gen \
    build/app/machine_id/machine_id Release

zip -r Release.zip ./Release ./include

REPO="http://192.168.100.12/artifactory/CI-Releases/"
PROJ="smlicense"
VERSION=$DRONE_TAG

USER="ciuploader"
PWD="ciuploader"

SOURCE="Release.zip"
URI="${REPO}/${PROJ}/${VERSION}/linux_x86_64.zip"

curl -u "${USER}:${PWD}" -X PUT "${REPO}/${PROJ}/${VERSION}/linux_x86_64.zip" -T Release.zip