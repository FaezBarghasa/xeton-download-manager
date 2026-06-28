package com.xetondownloadmanager.android.util.pagemanager

import android.content.Context
import android.content.Intent
import com.xetondownloadmanager.android.pages.downloaderror.DownloadErrorActivity
import com.xetondownloadmanager.shared.downloaderror.DownloadErrorComponent
import com.xetondownloadmanager.shared.pagemanager.DownloadErrorDialogManager
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorReason
import com.xeton.downloader.downloaditem.IDownloadItem
import kotlinx.serialization.json.Json

class AndroidDownloadErrorPageManager(
    private val openIntent: (Intent) -> Unit,
    private val context: Context,
    private val json: Json,
) : DownloadErrorDialogManager {
    override fun openDownloadErrorDialog(
        downloadItem: IDownloadItem,
        reason: DownloadErrorReason
    ) {
        openIntent(
            DownloadErrorActivity.createIntent(
                context,
                DownloadErrorComponent.DownloadErrorConfig(
                    downloadItem = downloadItem,
                    errorReason = reason,
                ),
                json = json,
            )
        )
    }

    override fun closeDownloadErrorDialog() {
        TODO("Not yet implemented")
    }
}
