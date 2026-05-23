plugins {
    id("java")
    id("com.rustyjavac") version "0.1.0"
}

repositories {
    mavenCentral()
}

rustyJavaC {
    command.set(listOf("cargo", "run", "--manifest-path", "../../Cargo.toml", "-p", "compiler-example", "--"))
    javaVersion.set(21)
}

tasks.register("runDemo", JavaExec::class) {
    dependsOn("rustyJavaCJar")
    classpath = files(tasks.named("rustyJavaCJar"))
    mainClass.set("com.example.HelloWorld")
    group = "rustyjavac"
    description = "Runs the demo using RustyJavaC-compiled JAR"
}
