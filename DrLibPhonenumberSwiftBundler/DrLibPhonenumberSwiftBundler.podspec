#
# Be sure to run `pod lib lint DrLibPhonenumberSwiftBundler.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'DrLibPhonenumberSwiftBundler'
  s.version          = '0.1.0'
  s.summary          = 'The underlying implementation of dr_libphonenumber.'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
This library is the underlying implementation of dr_libphonenumber.
                       DESC

  s.homepage         = 'https://github.com/TabooSun/dr_libphonenumber_rust.git'
  # s.screenshots     = 'www.example.com/screenshots_1', 'www.example.com/screenshots_2'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'TabooSun' => 'taboosun1996@gmail.com' }
  s.source           = { :git => 'https://github.com/TabooSun/dr_libphonenumber_rust.git', :tag => s.version.to_s }
  s.public_header_files = 'DrLibPhonenumberSwiftBundler/Classes/**/*.h'
  s.source_files = 'DrLibPhonenumberSwiftBundler/Classes/**/*'
  s.static_framework = true
  s.vendored_libraries = "DrLibPhonenumberSwiftBundler/*.a"

  s.platform = :ios, '10.0'

  s.swift_version = '5.0'

  s.user_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
  s.pod_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }

  # s.resource_bundles = {
  #   'DrLibPhonenumberSwiftBundler' => ['DrLibPhonenumberSwiftBundler/Assets/*.png']
  # }

  # s.public_header_files = 'Pod/Classes/**/*.h'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end
