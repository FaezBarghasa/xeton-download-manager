package com.xetondownloadmanager.desktop.pages.addDownload.multiple

import com.xetondownloadmanager.shared.ui.widget.table.customtable.TableState
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.desktop.repository.AppRepository
import com.xetondownloadmanager.shared.downloaderinui.DownloaderInUiRegistry
import com.xetondownloadmanager.shared.pagemanager.CategoryDialogManager
import com.xetondownloadmanager.shared.pages.adddownload.multiple.BaseAddMultiDownloadComponent
import com.xetondownloadmanager.shared.pages.adddownload.multiple.OnRequestAddMultipleItem
import com.xetondownloadmanager.shared.pages.adddownload.multiple.OnRequestDownloadMultipleItem
import com.xetondownloadmanager.shared.storage.ILastSavedLocationsStorage
import com.xetondownloadmanager.shared.storage.ISelectQueueStorage
import com.xetondownloadmanager.shared.util.FileIconProvider
import com.xetondownloadmanager.shared.util.category.CategoryManager
import com.xetondownloadmanager.shared.util.perhostsettings.PerHostSettingsManager
import com.arkivanov.decompose.ComponentContext
import com.xeton.downloader.queue.QueueManager

class DesktopAddMultiDownloadComponent(
    ctx: ComponentContext,
    id: String,
    onRequestClose: () -> Unit,
    onRequestAddMultipleItem: OnRequestAddMultipleItem,
    onRequestDownloadMultipleItem: OnRequestDownloadMultipleItem,
    private val categoryDialogManager: CategoryDialogManager,
    lastSavedLocationsStorage: ILastSavedLocationsStorage,
    selectQueueStorage: ISelectQueueStorage,
    perHostSettingsManager: PerHostSettingsManager, downloadSystem: DownloadSystem,
    fileIconProvider: FileIconProvider,
    appRepository: AppRepository,
    downloaderInUiRegistry: DownloaderInUiRegistry,
    queueManager: QueueManager,
    categoryManager: CategoryManager,
) : BaseAddMultiDownloadComponent(
    ctx = ctx,
    id = id,
    lastSavedLocationsStorage = lastSavedLocationsStorage,
    selectQueueStorage = selectQueueStorage,
    onRequestAddMultipleItem = onRequestAddMultipleItem,
    onRequestDownloadMultipleItem = onRequestDownloadMultipleItem,
    onRequestClose = onRequestClose,
    perHostSettingsManager = perHostSettingsManager,
    downloadSystem = downloadSystem,
    appRepository = appRepository,
    fileIconProvider = fileIconProvider,
    downloaderInUiRegistry = downloaderInUiRegistry,
    queueManager = queueManager,
    categoryManager = categoryManager,
) {
    override fun getCategoryPageManager(): CategoryDialogManager {
        return categoryDialogManager
    }
    val tableState = TableState(
        cells = AddMultiItemTableCells.all(),
        forceVisibleCells = listOf(
            AddMultiItemTableCells.Check,
            AddMultiItemTableCells.Name,
        )
    )
}

