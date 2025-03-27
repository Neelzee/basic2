plugins {
    kotlin("jvm")
}

group = "no.nilsmf.compiler-backend"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.bytedeco:llvm-platform:17.0.6-1.5.10")
    implementation("org.bytedeco:llvm:17.0.6-1.5.10")
}