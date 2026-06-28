package com.xetondownloadmanager.shared.downloaderror

import com.xetondownloadmanager.shared.util.BaseComponent
import com.xetondownloadmanager.shared.util.ClipboardUtil
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorReason
import com.arkivanov.decompose.ComponentContext
import com.xeton.downloader.downloaditem.IDownloadItem
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject

class DownloadErrorComponent(
    ctx: ComponentContext,
    config: DownloadErrorConfig,
    val onClose: () -> Unit,
) : BaseComponent(ctx), KoinComponent {
    val json: Json by inject()
    val downloadItem = config.downloadItem
    val reason = config.errorReason

    @Serializable
    data class DownloadErrorConfig(
        val downloadItem: IDownloadItem,
        val errorReason: DownloadErrorReason,
    )

    fun onRequestCopyToClipboard() {
        ClipboardUtil.copy(
            json.encodeToString(
                ClipboardData(
                    link = downloadItem.link,
                    reason = reason,
                )
            )
        )
    }

    @Serializable
    private data class ClipboardData(
        val link: String,
        val reason: DownloadErrorReason,
    )
}
