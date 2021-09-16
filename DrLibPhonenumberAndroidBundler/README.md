### DrLibPhonenumberAndroidBundler
This project is experimental and has no effect.

Copy the following to the root project of the dr_libphonenumber.
```markdown
### Android
1. Change working directory to `android/`
2. Uncomment all `mavenLocal()` in project level `build.gradle`.
3. Change working directory to `native/DrLibPhonenumberAndroidBundler`.
4. Run `./gradlew publishReleasePublicationToMavenLocal`.
```
```markdown

## Publish (For author reference only)
Update the version in `native/DrLibPhonenumberAndroidBundler/dr_lib_phonenumber_android_bundler/build.gradle`.
As in:
```groovy
publishing {
    publications {
        release(MavenPublication) {
            ...
            version = "1.0"
        }
    }
}
```

```
