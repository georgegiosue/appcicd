import org.jetbrains.kotlin.gradle.plugin.mpp.pm20.util.archivesName
import org.gradle.language.nativeplatform.internal.BuildType

plugins {
    alias(libs.plugins.androidApplication)
    alias(libs.plugins.jetbrainsKotlinAndroid)
}

android {
    namespace = "xyz.ggeorge.kotlindsl_cicd"
    compileSdk = 34

    defaultConfig {
        applicationId = "xyz.ggeorge.kotlindsl_cicd"
        minSdk = 24
        targetSdk = 34
        versionCode = Versioning.code
        versionName = Versioning.name
        archivesName = "${rootProject.name}-${versionName}"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    signingConfigs {
        named(BuildType.DEBUG.name) {
            storeFile = rootProject.file("secrets/debug-keystore.jks")
            storePassword = env("DEBUG_KEYSTORE_PASSWORD")
            keyAlias = "debug"
            keyPassword = env("DEBUG_KEY_PASSWORD")
        }
        register(BuildType.RELEASE.name) {
            storeFile = rootProject.file("secrets/release-keystore.jks")
            storePassword = env("RELEASE_KEYSTORE_PASSWORD")
            keyAlias = "release"
            keyPassword = env("RELEASE_KEY_PASSWORD")
        }
    }

    buildTypes {
        named(BuildType.DEBUG.name) {
            signingConfig = signingConfigs.getByName(BuildType.DEBUG.name)
            isDebuggable = true
        }
        named(BuildType.RELEASE.name) {
            if (rootProject.file("secrets/release-keystore.jks").exists()) {
                signingConfig = signingConfigs.getByName(BuildType.RELEASE.name)
            }
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        compose = true
    }
    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.1"
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {

    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.activity.compose)
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.ui)
    implementation(libs.androidx.ui.graphics)
    implementation(libs.androidx.ui.tooling.preview)
    implementation(libs.androidx.material3)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.ui.test.junit4)
    debugImplementation(libs.androidx.ui.tooling)
    debugImplementation(libs.androidx.ui.test.manifest)
}