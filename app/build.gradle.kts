plugins {
    id("org.jetbrains.kotlin.jvm") version "2.0.0"
    id("antlr")
}

group = "com.example"
version = "1.0"

repositories {
    mavenCentral()
}

dependencies {
    antlr("org.antlr:antlr4:4.13.1")
    implementation("io.arrow-kt:arrow-core:2.0.1")
    implementation("io.arrow-kt:arrow-fx-coroutines:2.0.1")

    implementation("org.jetbrains.kotlin:kotlin-stdlib")
    implementation("org.antlr:antlr4-runtime:4.13.1")

    testImplementation("org.jetbrains.kotlin:kotlin-test")
}

tasks.generateGrammarSource {
    arguments = listOf("-visitor", "-listener")
    outputDirectory = file("build/generated-src/antlr/main")
}

sourceSets.main {
    java.srcDirs("build/generated-src/antlr/main")
}

tasks.compileKotlin {
    dependsOn(tasks.generateGrammarSource)
}
