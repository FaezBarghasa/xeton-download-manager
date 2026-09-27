package com.xetondownloadmanager.shared.pages.enterurl

import com.xetondownloadmanager.shared.downloaderinui.TADownloaderInUI

sealed interface DownloaderSelection {
    data object Auto : DownloaderSelection
    data class Fixed(
        val downloaderInUi: TADownloaderInUI,
    ) : DownloaderSelection
}
