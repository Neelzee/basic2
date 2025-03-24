plugins {
    kotlin("jvm")
    id("antlr")
    id("com.strumenta.antlr-kotlin")
    kotlin("plugin.serialization")
}

group = "no.nilsmf.compiler-frontend"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    antlr("org.antlr:antlr4:4.13.2")
    implementation("com.strumenta:antlr-kotlin-runtime:1.0.0")
    implementation("io.arrow-kt:arrow-core:2.0.1")
    implementation("io.arrow-kt:arrow-fx-coroutines:2.0.1")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.0")
    testImplementation(kotlin("test"))
}

tasks.register<com.strumenta.antlrkotlin.gradle.AntlrKotlinTask>("generateKotlinGrammarSource") {
    packageName = "no.nilsmf.antlr"
    arguments = listOf("-visitor")
    source = project.objects
        .sourceDirectorySet("antlr", "antlr")
        .srcDir("src/main/antlr").apply {
            include("*.g4")
        }
    outputDirectory = File("build/generated-src")
}

tasks.named("generateGrammarSource").configure {
    enabled = false
}

tasks.named("compileKotlin") {
    dependsOn(tasks.named("generateKotlinGrammarSource"))
}

tasks.named("compileTestKotlin") {
    dependsOn(tasks.named("generateKotlinGrammarSource"), tasks.named("generateTestGrammarSource"))
}

sourceSets["main"].kotlin.srcDir("build/generated-src/")

kotlin {
    compilerOptions {
        freeCompilerArgs.add("-Xwhen-guards")
    }
}