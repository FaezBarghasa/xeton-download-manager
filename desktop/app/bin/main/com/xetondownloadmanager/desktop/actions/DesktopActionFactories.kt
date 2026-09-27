package com.xetondownloadmanager.desktop.actions

import com.xetondownloadmanager.desktop.DesktopDownloadDialogManager
import com.xetondownloadmanager.shared.action.createStopAllAction
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xeton.downloader.queue.DownloadQueue
import com.xeton.util.compose.action.AnAction
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.StateFlow


fun createDesktopStopAllAction(
    scope: CoroutineScope,
    downloadSystem: DownloadSystem,
    desktopDownloadDialogManager: DesktopDownloadDialogManager,
    activeQueuesFlow: StateFlow<List<DownloadQueue>>
): AnAction {
    return createStopAllAction(
        scope = scope,
        downloadSystem = downloadSystem,
        activeQueuesFlow = activeQueuesFlow,
        extraJobs = {
            val activeDownloadIds = downloadSystem.downloadMonitor.activeDownloadListFlow.value.map { it.id }
            desktopDownloadDialogManager.closeDownloadDialog(activeDownloadIds)
        }
    )
}
