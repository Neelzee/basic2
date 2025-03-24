plugins {
    kotlin("multiplatform") version "2.1.20" apply false
    kotlin("jvm") version "2.1.20" apply false
    id("com.strumenta.antlr-kotlin") version "1.0.2" apply false
    kotlin("plugin.serialization") version "2.1.0" apply false
}

rootProject.name = "basic2"

include("compiler-backend")

include("compiler-frontend")