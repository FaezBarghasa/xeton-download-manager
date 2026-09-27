package com.xetondownloadmanager.android.pages.add.single

import com.xetondownloadmanager.shared.action.createNewQueueAction
import com.xetondownloadmanager.shared.downloaderinui.DownloaderInUi
import com.xetondownloadmanager.shared.pagemanager.CategoryDialogManager
import com.xetondownloadmanager.shared.pagemanager.DownloadErrorDialogManager
import com.xetondownloadmanager.shared.pagemanager.NewQueuePageManager
import com.xetondownloadmanager.shared.pages.adddownload.AddDownloadCredentialsInUiProps
import com.xetondownloadmanager.shared.pages.adddownload.ImportOptions
import com.xetondownloadmanager.shared.pages.adddownload.single.BaseAddSingleDownloadComponent
import com.xetondownloadmanager.shared.pages.adddownload.single.OnRequestAddSingleItem
import com.xetondownloadmanager.shared.pages.adddownload.single.OnRequestDownloadSingleItem
import com.xetondownloadmanager.shared.pages.category.CategoryComponent
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xetondownloadmanager.shared.storage.ILastSavedLocationsStorage
import com.xetondownloadmanager.shared.storage.ISelectQueueStorage
import com.xetondownloadmanager.shared.util.DownloadItemOpener
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.util.FileIconProvider
import com.xetondownloadmanager.shared.util.category.CategoryManager
import com.xetondownloadmanager.shared.util.perhostsettings.PerHostSettingsManager
import com.xetondownloadmanager.shared.util.subscribeAsStateFlow
import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.router.slot.SlotNavigation
import com.arkivanov.decompose.router.slot.activate
import com.arkivanov.decompose.router.slot.childSlot
import com.arkivanov.decompose.router.slot.dismiss
import com.xeton.downloader.downloaditem.DownloadJobExtraConfig
import com.xeton.downloader.downloaditem.IDownloadCredentials
import com.xeton.downloader.queue.QueueManager
import com.xeton.util.flow.mapStateFlow
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.serialization.builtins.serializer

class AndroidAddSingleDownloadComponent(
    ctx: ComponentContext,
    onRequestClose: () -> Unit,
    onRequestDownload: OnRequestDownloadSingleItem,
    onRequestAddToQueue: OnRequestAddSingleItem,
    openExistingDownload: (Long) -> Unit,
    updateExistingDownloadCredentials: (Long, IDownloadCredentials, DownloadJobExtraConfig?) -> Unit,
    downloadItemOpener: DownloadItemOpener,
    downloadErrorDialogManager: DownloadErrorDialogManager,
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
) : BaseAddSingleDownloadComponent(
    ctx = ctx,
    onRequestClose = onRequestClose,
    onRequestDownload = onRequestDownload,
    onRequestAddToQueue = onRequestAddToQueue,
    openExistingDownload = openExistingDownload,
    updateExistingDownloadCredentials = updateExistingDownloadCredentials,
    downloadItemOpener = downloadItemOpener,
    downloadErrorDialogManager = downloadErrorDialogManager,
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
    appScope = appScope,
    appRepository = appRepository,
    perHostSettingsManager = perHostSettingsManager,
), CategoryDialogManager, NewQueuePageManager {
    val categoryComponentNavigation = SlotNavigation<Long>()
    val categorySlot = childSlot(
        source = categoryComponentNavigation,
        childFactory = { config, ctx ->
            CategoryComponent(
                ctx = ctx,
                id = config,
                close = ::closeCategoryDialog,
                submit = { submittedCategory ->
                    if (submittedCategory.id < 0) {
                        categoryManager.addCustomCategory(submittedCategory)
                    } else {
                        categoryManager.updateCategory(
                            submittedCategory.id
                        ) {
                            submittedCategory.copy(
                                items = it.items
                            )
                        }
                    }
                    closeCategoryDialog()
                },
            )
        },
        serializer = Long.serializer(),
    ).subscribeAsStateFlow()
    val newQueuesAction = createNewQueueAction(
        appScope,
        this,
    )

    override fun openCategoryDialog(categoryId: Long) {
        scope.launch {
            categoryComponentNavigation.activate(categoryId)
        }
    }

    override fun closeCategoryDialog() {
        scope.launch {
            categoryComponentNavigation.dismiss()
        }
    }

    override fun getCategoryPageManager(): CategoryDialogManager {
        return this
    }

    private val _showMoreInputs = MutableStateFlow(false)
    val showMoreInputs = _showMoreInputs.asStateFlow()
    fun setShowMoreInputs(value: Boolean) {
        _showMoreInputs.value = value
    }

    private val _showAddQueue = MutableStateFlow(false)
    val showAddQueue = _showAddQueue.asStateFlow()
    fun setShowAddQueue(value: Boolean) {
        _showAddQueue.value = value
    }

    val isWebPage = downloadChecker
        .responseInfo
        .mapStateFlow { it?.isWebPage ?: false }

    fun createQueueWithName(name: String) {
        scope.launch { queueManager.addQueue(name) }
        setShowAddQueue(false)
    }

    override fun closeNewQueueDialog() {
        setShowAddQueue(false)
    }

    override fun openNewQueueDialog() {
        setShowAddQueue(true)
    }

    fun onRequestOpenLinkInBrowser() {
        sendEffect(
            Effects.OpenInBrowser(
                downloadChecker.credentials.value.link
            )
        )
    }

    sealed interface Effects : BaseAddSingleDownloadComponent.Effects.Platform {
        data class OpenInBrowser(val link: String) : Effects
    }
}
