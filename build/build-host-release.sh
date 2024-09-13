#!/bin/bash

BUILD_TARGET=""
BUILD_FEATURES=()
while getopts "t:f:" opt; do
  case $opt in
  t)
    BUILD_TARGET=$OPTARG
    ;;
  f)
    BUILD_FEATURES+=($OPTARG)
    ;;
  ?)
    echo "Usage: $(basename $0) [-t <target-triple>] [-f <feature>]"
    ;;
  esac
done

BUILD_FEATURES+=${EXTRA_FEATURES}

ROOT_DIR=$(cd $(dirname $0) && pwd)
VERSION=$(grep -E '^version' "${ROOT_DIR}/../Cargo.toml" | awk '{print $3}' | sed 's/"//g')
HOST_TRIPLE=$(rustc -Vv | grep 'host:' | awk '{print $2}')

echo "Started build release ${VERSION} for ${HOST_TRIPLE} (target: ${BUILD_TARGET}) with features \"${BUILD_FEATURES}\"..."

CARGO_RELEASE_OPTS="--release"
if [[ "${BUILD_TARGET}" != "" ]]; then
  CARGO_RELEASE_OPTS+=" --target ${BUILD_TARGET}"
  if [[ "${BUILD_FEATURES}" != "" ]]; then
    CARGO_RELEASE_OPTS+=" --features ${BUILD_FEATURES}"
  fi
fi

cargo build ${CARGO_RELEASE_OPTS}
if [[ "$?" != "0" ]]; then
  exit 1
fi

if [[ "${BUILD_TARGET}" == "" ]]; then
  BUILD_TARGET=$HOST_TRIPLE
fi

TARGET_SUFFIX=""
if [[ "${BUILD_TARGET}" == *"-windows-"* ]]; then
  TARGET_SUFFIX=".exe"
fi

TARGETS=("smmstools${TARGET_SUFFIX}")


RELEASE_FOLDER="${ROOT_DIR}/release"
RELEASE_PACKAGE_NAME="smmstools-v${VERSION}.${BUILD_TARGET}"

mkdir -p "${RELEASE_FOLDER}"

# Into release folder
if [[ "${BUILD_TARGET}" != "" ]]; then
  cd "${ROOT_DIR}/../target/${BUILD_TARGET}/release"
else
  cd "${ROOT_DIR}/../target/release"
fi

if [[ "${BUILD_TARGET}" == *"-windows-"* ]]; then
  # For Windows, use zip

  RELEASE_PACKAGE_FILE_NAME="${RELEASE_PACKAGE_NAME}.zip"
  RELEASE_PACKAGE_FILE_PATH="${RELEASE_FOLDER}/${RELEASE_PACKAGE_FILE_NAME}"
  7z a "${RELEASE_PACKAGE_FILE_PATH}" "${TARGETS[@]}"

  if [[ $? != "0" ]]; then
    exit 1
  fi

  # Checksum
  cd "${RELEASE_FOLDER}"
  sha256sum "${RELEASE_PACKAGE_FILE_NAME}" >"${RELEASE_PACKAGE_FILE_NAME}.sha256"
else
  # For others, Linux, OS X, uses tar.gz

  # For Darwin, .DS_Store and other related files should be ignored
  if [[ "$(uname -s)" == "Darwin" ]]; then
    export COPYFILE_DISABLE=1
  fi

  RELEASE_PACKAGE_FILE_NAME="${RELEASE_PACKAGE_NAME}.tar.gz"
  RELEASE_PACKAGE_FILE_PATH="${RELEASE_FOLDER}/${RELEASE_PACKAGE_FILE_NAME}"
  tar -czf "${RELEASE_PACKAGE_FILE_PATH}" "${TARGETS[@]}"

  if [[ $? != "0" ]]; then
    exit 1
  fi

  # Checksum
  cd "${RELEASE_FOLDER}"
  shasum -a 256 "${RELEASE_PACKAGE_FILE_NAME}" >"${RELEASE_PACKAGE_FILE_NAME}.sha256"
fi

echo "Finished build release ${RELEASE_PACKAGE_FILE_PATH}"