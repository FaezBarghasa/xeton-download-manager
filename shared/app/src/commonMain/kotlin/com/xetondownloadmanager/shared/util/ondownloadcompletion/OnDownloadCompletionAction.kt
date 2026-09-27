package com.xetondownloadmanager.shared.util.ondownloadcompletion

import com.xeton.downloader.downloaditem.IDownloadItem

interface OnDownloadCompletionAction {
    suspend fun onDownloadCompleted(downloadItem: IDownloadItem)
}
