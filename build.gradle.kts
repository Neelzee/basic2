plugins {
    kotlin("jvm") version "2.0.21"
    id("antlr")
    id("com.strumenta.antlr-kotlin") version "1.0.2"
}

group = "com.example"
version = "1.0.2"

repositories {
    mavenCentral()
}

dependencies {
    antlr("org.antlr:antlr4:4.13.2")
    implementation("com.strumenta:antlr-kotlin-runtime:1.0.0")
    testImplementation(kotlin("test"))
}

sourceSets["main"].kotlin.srcDir("build/generated-src/")

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

