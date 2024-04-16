import java.io.IOException
import java.time.Instant

object Versioning {
    val name: String by lazy {
        try {
            val androidEnv = env("ANDROID_ENV") ?: "STAGING"
            val isRelease = androidEnv == "RELEASE"
            val gitTagProcess = Runtime.getRuntime().exec("git describe --tags --abbrev=0")
            val bufferedReader = gitTagProcess.inputStream.bufferedReader()
            var tag = bufferedReader.readLine().removePrefix("v")
            gitTagProcess.waitFor()

            if (!isRelease) {
                val commitHashProcess = Runtime.getRuntime().exec("git rev-parse --short HEAD")
                val bufferReader = commitHashProcess.inputStream.bufferedReader()
                val commitHash = bufferReader.readLine()

                commitHashProcess.waitFor()

                tag = "$tag-snapshot-($commitHash)"
            }

            tag
        } catch (e: IOException) {
            "1.0.0"
        }
    }
    val code: Int by lazy {
        Instant.now().epochSecond.toInt()
    }
}