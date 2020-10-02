cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target aarch64-linux-android --release

cp target/aarch64-linux-android/release/librlnandroid.so ~/AndroidStudioProjects/RLNBench3/app/src/main/jniLibs/arm64
cp target/armv7-linux-androideabi/release/librlnandroid.so ~/AndroidStudioProjects/RLNBench3/app/src/main/jniLibs/armeabi
cp target/i686-linux-android/release/librlnandroid.so ~/AndroidStudioProjects/RLNBench3/app/src/main/jniLibs/x86
