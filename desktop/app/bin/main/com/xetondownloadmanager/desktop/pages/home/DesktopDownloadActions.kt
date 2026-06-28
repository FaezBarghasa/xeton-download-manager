package com.xetondownloadmanager.desktop.pages.home

import com.xetondownloadmanager.shared.pagemanager.EditDownloadDialogManager
import com.xetondownloadmanager.shared.pagemanager.FileChecksumDialogManager
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.pagemanager.DownloadDialogManager
import com.xetondownloadmanager.shared.pages.home.AbstractDownloadActions
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.util.category.CategoryManager
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xeton.downloader.monitor.IDownloadItemState
import com.xeton.downloader.queue.QueueManager
import com.xeton.util.compose.action.MenuItem
import com.xeton.util.compose.action.buildMenu
import com.xeton.util.compose.action.simpleAction
import com.xeton.util.compose.asStringSource
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

class DesktopDownloadActions(
    scope: CoroutineScope,
    downloadSystem: DownloadSystem,
    downloadDialogManager: DownloadDialogManager,
    editDownloadDialogManager: EditDownloadDialogManager,
    fileChecksumDialogManager: FileChecksumDialogManager,
    selections: StateFlow<List<IDownloadItemState>>,
    queueManager: QueueManager,
    categoryManager: CategoryManager,
    openFile: (Long) -> Unit,
    requestDelete: (List<Long>) -> Unit,
    mainItem: StateFlow<Long?>,
    private val openFolder: (Long) -> Unit,
) : AbstractDownloadActions(
    scope = scope,
    downloadSystem = downloadSystem,
    downloadDialogManager = downloadDialogManager,
    editDownloadDialogManager = editDownloadDialogManager,
    fileChecksumDialogManager = fileChecksumDialogManager,
    selections = selections,
    queueManager = queueManager,
    categoryManager = categoryManager,
    openFile = openFile,
    requestDelete = requestDelete,
    mainItem = mainItem,
) {
    val openFolderAction = simpleAction(
        title = Res.string.open_folder.asStringSource(),
        icon = MyIcons.folderOpen,
        onActionPerformed = {
            scope.launch {
                val d = defaultItem.value ?: return@launch
                openFolder(d.id)
            }
        }
    )

    val menu: List<MenuItem> = buildMenu {
        +openFileAction
        +openFolderAction
        +(resumeAction)
        +pauseAction
        separator()
        +(deleteAction)
        +(reDownloadAction)
        separator()
        +moveToQueueItems
        +moveToCategoryAction
        separator()
        subMenu(Res.string.copy.asStringSource(), MyIcons.copy) {
            +(copyDownloadLinkAction)
            +(copyDownloadCredentialsAsJsonAction)
            +(copyDownloadCredentialsAsCurlAction)
        }
        +editDownloadAction
        +fileChecksumAction
        +(openDownloadDialogAction)
    }
}
