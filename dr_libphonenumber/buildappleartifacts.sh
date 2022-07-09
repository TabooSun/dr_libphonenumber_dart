#!/bin/sh
# https://github.com/rust-lang/rust/issues/79408
set -x

FINAL_FRAMEWORK_NAME="DrLibPhonenumber"

IOS_ARTIFACTS_INSTALLATION_DIR=../../ios
MACOS_ARTIFACTS_INSTALLATION_DIR=../../macos

STATIC_LIBRARY_NAME="libdr_libphonenumber.a"

XCFRAMEWORK_ARGS=""
for ARCH in "aarch64-apple-ios" "aarch64-apple-ios-sim"
do
  XCFRAMEWORK_ARGS="${XCFRAMEWORK_ARGS} -library target/$ARCH/release/$STATIC_LIBRARY_NAME"
  XCFRAMEWORK_ARGS="${XCFRAMEWORK_ARGS} -headers target/bindings.h"
  
#  swift_module_map > "$tmpdir/$ARCH/release/headers/module.modulemap"
done

IOS_ARTIFACTS_INSTALLATION_PATH=${IOS_ARTIFACTS_INSTALLATION_DIR}/${FINAL_FRAMEWORK_NAME}.xcframework
rm -rf ${IOS_ARTIFACTS_INSTALLATION_PATH}
MACOS_ARTIFACTS_INSTALLATION_PATH=${MACOS_ARTIFACTS_INSTALLATION_DIR}/${FINAL_FRAMEWORK_NAME}.xcframework
rm -rf ${MACOS_ARTIFACTS_INSTALLATION_PATH}
 
# Create ios xcframework
xcodebuild -create-xcframework \
  $XCFRAMEWORK_ARGS \
  -library target/lipo/macos/$STATIC_LIBRARY_NAME \
  -headers target/bindings.h \
  -output ${IOS_ARTIFACTS_INSTALLATION_PATH}
  
# Create macos xcframework
xcodebuild -create-xcframework \
  -library target/lipo/macos/$STATIC_LIBRARY_NAME \
  -headers target/bindings.h \
  -output ${MACOS_ARTIFACTS_INSTALLATION_PATH}
  
