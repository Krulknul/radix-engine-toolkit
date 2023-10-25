import org.gradle.jvm.tasks.Jar

plugins {
    kotlin("jvm") version "1.8.21"
    `java-library`
    `maven-publish`
    `signing`
}

repositories {
    mavenCentral()
}

dependencies {
   implementation("net.java.dev.jna:jna:5.13.0")
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(8))
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "com.radixdlt"
            artifactId = "radix-engine-toolkit-kotlin"
            version = providers.gradleProperty("ret-version").getOrNull()

            from(components["java"])
        }
    }

    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/radixdlt/radix-engine-toolkit")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }

        maven {
            url = uri("https://s01.oss.sonatype.org/content/repositories/snapshots/")
            credentials {
                username = project.findProperty("ossrhUsername") as String?
                password = project.findProperty("ossrhPassword") as String?
            }
        }
    }
}

signing {
    sign(publishing.publications["maven"])
}
