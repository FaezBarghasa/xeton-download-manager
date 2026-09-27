package com.xetondownloadmanager.desktop.actions.onevennts

import com.xetondownloadmanager.shared.storage.IExtraDownloadSettingsStorage
import com.xetondownloadmanager.shared.util.ondownloadcompletion.OnDownloadCompletionAction
import com.xeton.downloader.downloaditem.IDownloadItem

class CleanExtraSettingsOnDownloadFinish(
    private val storage: IExtraDownloadSettingsStorage<*>
) : OnDownloadCompletionAction {
    override suspend fun onDownloadCompleted(downloadItem: IDownloadItem) {
        storage.deleteExtraDownloadItemSettings(downloadItem.id)
    }
}
