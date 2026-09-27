package com.xetondownloadmanager.shared.util.downloadlocation

import com.xetondownloadmanager.shared.util.SystemDownloadLocationProvider
import com.xeton.util.platform.Platform
import com.xeton.util.platform.asDesktop

actual fun getPlatformDownloadLocationProvider(): SystemDownloadLocationProvider {
    return when (Platform.asDesktop()) {
        Platform.Desktop.Windows -> WindowsDownloadLocationProvider()
        Platform.Desktop.Linux -> LinuxDownloadLocationProvider()
        Platform.Desktop.MacOS -> MacDownloadLocationProvider()
    }
}
