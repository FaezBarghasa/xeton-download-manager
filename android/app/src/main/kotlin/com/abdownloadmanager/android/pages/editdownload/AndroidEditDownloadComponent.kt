package com.xetondownloadmanager.android.pages.editdownload

import com.xetondownloadmanager.shared.downloaderinui.DownloaderInUiRegistry
import com.xetondownloadmanager.shared.pagemanager.DownloadErrorDialogManager
import com.xetondownloadmanager.shared.pages.editdownload.BaseEditDownloadComponent
import com.xetondownloadmanager.shared.util.mvi.ContainsEffects
import com.xetondownloadmanager.shared.util.mvi.supportEffects
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.util.FileIconProvider
import com.arkivanov.decompose.ComponentContext
import com.xeton.downloader.downloaditem.DownloadJobExtraConfig
import com.xeton.downloader.downloaditem.IDownloadItem
import kotlinx.coroutines.flow.*

class AndroidEditDownloadComponent(
    ctx: ComponentContext,
    onRequestClose: () -> Unit,
    downloadId: Long,
    acceptEdit: StateFlow<Boolean>,
    onEdited: ((IDownloadItem) -> Unit, DownloadJobExtraConfig?) -> Unit,
    downloadSystem: DownloadSystem,
    downloaderInUiRegistry: DownloaderInUiRegistry,
    iconProvider: FileIconProvider,
    downloadErrorDialogManager: DownloadErrorDialogManager,
) : BaseEditDownloadComponent(
    ctx = ctx,
    downloadSystem = downloadSystem,
    downloaderInUiRegistry = downloaderInUiRegistry,
    iconProvider = iconProvider,
    onEdited = onEdited,
    onRequestClose = onRequestClose,
    downloadId = downloadId,
    acceptEdit = acceptEdit,
    downloadErrorDialogManager = downloadErrorDialogManager,
),
    ContainsEffects<AndroidEditDownloadComponent.Effects> by supportEffects() {
    sealed interface Effects {
    }
}
