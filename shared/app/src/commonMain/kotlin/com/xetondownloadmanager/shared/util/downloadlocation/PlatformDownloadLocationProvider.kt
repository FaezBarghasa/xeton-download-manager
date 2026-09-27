package com.xetondownloadmanager.shared.util.downloadlocation

import com.xetondownloadmanager.shared.util.SystemDownloadLocationProvider

object PlatformDownloadLocationProvider {
    val instance: SystemDownloadLocationProvider by lazy {
        getPlatformDownloadLocationProvider()
    }
}

expect fun getPlatformDownloadLocationProvider(): SystemDownloadLocationProvider

