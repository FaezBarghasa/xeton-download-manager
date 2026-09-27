package com.xetondownloadmanager.desktop.pages.addDownload.single

import com.xetondownloadmanager.shared.downloaderinui.DownloaderInUi
import com.xetondownloadmanager.shared.pagemanager.CategoryDialogManager
import com.xetondownloadmanager.shared.pagemanager.DownloadErrorDialogManager
import com.xetondownloadmanager.shared.pages.adddownload.AddDownloadCredentialsInUiProps
import com.xetondownloadmanager.shared.pages.adddownload.ImportOptions
import com.xetondownloadmanager.shared.pages.adddownload.single.BaseAddSingleDownloadComponent
import com.xetondownloadmanager.shared.pages.adddownload.single.OnRequestAddSingleItem
import com.xetondownloadmanager.shared.pages.adddownload.single.OnRequestDownloadSingleItem
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xetondownloadmanager.shared.storage.ILastSavedLocationsStorage
import com.xetondownloadmanager.shared.storage.ISelectQueueStorage
import com.xetondownloadmanager.shared.util.DownloadItemOpener
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.util.FileIconProvider
import com.xetondownloadmanager.shared.util.category.CategoryManager
import com.xetondownloadmanager.shared.util.perhostsettings.PerHostSettingsManager
import com.arkivanov.decompose.ComponentContext
import com.xeton.downloader.downloaditem.DownloadJobExtraConfig
import com.xeton.downloader.downloaditem.IDownloadCredentials
import com.xeton.downloader.queue.QueueManager
import kotlinx.coroutines.CoroutineScope

class DesktopAddSingleDownloadComponent(
    ctx: ComponentContext,
    onRequestClose: () -> Unit,
    onRequestDownload: OnRequestDownloadSingleItem,
    onRequestAddToQueue: OnRequestAddSingleItem,
    openExistingDownload: (Long) -> Unit,
    updateExistingDownloadCredentials: (Long, IDownloadCredentials, DownloadJobExtraConfig?) -> Unit,
    downloadItemOpener: DownloadItemOpener,
    lastSavedLocationsStorage: ILastSavedLocationsStorage,
    selectQueueStorage: ISelectQueueStorage,
    queueManager: QueueManager,
    categoryManager: CategoryManager,
    downloadSystem: DownloadSystem,
    appSettings: BaseAppSettingsStorage,
    iconProvider: FileIconProvider,
    appScope: CoroutineScope,
    appRepository: BaseAppRepository,
    perHostSettingsManager: PerHostSettingsManager,
    importOptions: ImportOptions,
    id: String,
    downloaderInUi: DownloaderInUi<IDownloadCredentials, *, *, *, *, *, *, *, *, *>,
    initialCredentials: AddDownloadCredentialsInUiProps,
    downloadErrorDialogManager: DownloadErrorDialogManager,
    private val categoryDialogManager: CategoryDialogManager,
) : BaseAddSingleDownloadComponent(
    ctx = ctx,
    onRequestClose = onRequestClose,
    onRequestDownload = onRequestDownload,
    onRequestAddToQueue = onRequestAddToQueue,
    openExistingDownload = openExistingDownload,
    updateExistingDownloadCredentials = updateExistingDownloadCredentials,
    downloadItemOpener = downloadItemOpener,
    lastSavedLocationsStorage = lastSavedLocationsStorage,
    selectQueueStorage = selectQueueStorage,
    importOptions = importOptions,
    id = id,
    downloaderInUi = downloaderInUi,
    initialCredentials = initialCredentials,
    queueManager = queueManager,
    categoryManager = categoryManager,
    downloadSystem = downloadSystem,
    appSettings = appSettings,
    iconProvider = iconProvider,
    downloadErrorDialogManager = downloadErrorDialogManager,
    appScope = appScope,
    appRepository = appRepository,
    perHostSettingsManager = perHostSettingsManager,
) {
    override fun getCategoryPageManager(): CategoryDialogManager {
        return categoryDialogManager
    }
}
