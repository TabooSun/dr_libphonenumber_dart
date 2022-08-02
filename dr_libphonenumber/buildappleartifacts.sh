#!/bin/sh
# https://github.com/rust-lang/rust/issues/79408
set -x

FINAL_FRAMEWORK_NAME="DrLibPhonenumber"

IOS_ARTIFACTS_INSTALLATION_DIR=../../ios
MACOS_ARTIFACTS_INSTALLATION_DIR=../../macos

STATIC_LIBRARY_NAME="libdr_libphonenumber.a"

IOS_ARTIFACTS_INSTALLATION_PATH=${IOS_ARTIFACTS_INSTALLATION_DIR}/${FINAL_FRAMEWORK_NAME}.xcframework
rm -rf ${IOS_ARTIFACTS_INSTALLATION_PATH}
MACOS_ARTIFACTS_INSTALLATION_PATH=${MACOS_ARTIFACTS_INSTALLATION_DIR}/${FINAL_FRAMEWORK_NAME}.xcframework
rm -rf ${MACOS_ARTIFACTS_INSTALLATION_PATH}
 
# Create ios xcframework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/$STATIC_LIBRARY_NAME \
  -headers includes/ \
  -library target/lipo/ios-sim/$STATIC_LIBRARY_NAME \
  -headers includes/ \
  -output ${IOS_ARTIFACTS_INSTALLATION_PATH}
  
# Create macos xcframework
xcodebuild -create-xcframework \
  -library target/lipo/macos/$STATIC_LIBRARY_NAME \
  -headers includes/ \
  -output ${MACOS_ARTIFACTS_INSTALLATION_PATH}
  
