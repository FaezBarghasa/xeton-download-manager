package com.xetondownloadmanager.android.pages.batchdownload

import com.xetondownloadmanager.shared.pages.batchdownload.BaseBatchDownloadComponent
import com.arkivanov.decompose.ComponentContext

class AndroidBatchDownloadComponent(
    ctx: ComponentContext,
    onClose: () -> Unit,
    importLinks: (List<String>) -> Unit,
) : BaseBatchDownloadComponent(
    ctx = ctx,
    onClose = onClose,
    importLinks = importLinks
) {
    sealed interface Effects : BaseBatchDownloadComponent.Effects.PlatformEffects {
        // nothing for now
    }
}
