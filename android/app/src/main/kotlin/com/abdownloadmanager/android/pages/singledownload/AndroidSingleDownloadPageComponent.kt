package com.xetondownloadmanager.android.pages.singledownload

import com.xetondownloadmanager.android.storage.AndroidExtraDownloadItemSettings
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.pagemanager.DownloadErrorDialogManager
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.singledownloadpage.BaseSingleDownloadComponent
import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xetondownloadmanager.shared.storage.ExtraDownloadSettingsStorage
import com.xetondownloadmanager.shared.ui.configurable.item.BooleanConfigurable
import com.xetondownloadmanager.shared.util.DownloadItemOpener
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.util.FileIconProvider
import com.arkivanov.decompose.ComponentContext
import com.xeton.util.compose.asStringSource
import com.xeton.util.flow.mapTwoWayStateFlow
import kotlinx.coroutines.CoroutineScope

class AndroidSingleDownloadComponent(
    ctx: ComponentContext,
    downloadItemOpener: DownloadItemOpener,
    onDismiss: () -> Unit,
    downloadId: Long,
    extraDownloadSettingsStorage: ExtraDownloadSettingsStorage<AndroidExtraDownloadItemSettings>,
    downloadSystem: DownloadSystem,
    appSettings: BaseAppSettingsStorage,
    appRepository: BaseAppRepository,
    applicationScope: CoroutineScope,
    fileIconProvider: FileIconProvider,
    val comesFromExternalApplication: Boolean,
    downloadErrorDialogManager: DownloadErrorDialogManager,
) : BaseSingleDownloadComponent<AndroidExtraDownloadItemSettings>(
    ctx = ctx,
    downloadItemOpener = downloadItemOpener,
    onDismiss = onDismiss,
    downloadId = downloadId,
    extraDownloadSettingsStorage = extraDownloadSettingsStorage,
    downloadSystem = downloadSystem,
    appSettings = appSettings,
    appRepository = appRepository,
    applicationScope = applicationScope,
    fileIconProvider = fileIconProvider,
    downloadErrorDialogManager = downloadErrorDialogManager,
) {
    override val defaultShowPartInfo: Boolean = false
//    private val singleDownloadPageStateToPersist by lazy {
//        get<PageStatesStorage>().singleDownloadPageState
//    }
//    override fun setShowPartInfo(value: Boolean) {
//        super.setShowPartInfo(value)
//        singleDownloadPageStateToPersist.update {
//            it.copy {
//                SingleDownloadPageStateToPersist.showPartInfo.set(value)
//            }
//        }
//    }

    sealed interface Effects : BaseSingleDownloadComponent.Effects.Platform

    val onCompletion by lazy {
        listOf(
            BooleanConfigurable(
                title = Res.string.download_item_settings_show_download_completion_dialog.asStringSource(),
                description = Res.string.download_item_settings_show_download_completion_dialog_description.asStringSource(),
                backedBy = itemShouldShowCompletionDialog.mapTwoWayStateFlow(
                    map = {
                        it ?: globalShowCompletionDialog.value
                    },
                    unMap = { it }
                ),
                describe = {
                    when (it) {
                        true -> Res.string.enabled
                        false -> Res.string.disabled
                    }.asStringSource()
                },
            ),
        )
    }

    data class Config(
        override val id: Long
    ) : BaseSingleDownloadComponent.Config
}


