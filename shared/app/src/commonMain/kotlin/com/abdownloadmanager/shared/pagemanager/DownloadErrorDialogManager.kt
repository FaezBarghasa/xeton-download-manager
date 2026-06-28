package com.xetondownloadmanager.shared.pagemanager

import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorReason
import com.xeton.downloader.downloaditem.IDownloadItem

interface DownloadErrorDialogManager {
    fun openDownloadErrorDialog(downloadItem: IDownloadItem, reason: DownloadErrorReason)
    fun closeDownloadErrorDialog()
}
