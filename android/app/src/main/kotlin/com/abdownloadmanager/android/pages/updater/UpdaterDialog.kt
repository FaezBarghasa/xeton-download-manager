package com.xetondownloadmanager.android.pages.updater

import androidx.compose.runtime.*
import com.xetondownloadmanager.shared.pages.updater.RenderUpdateNotifications
import com.xetondownloadmanager.shared.pages.updater.UpdateComponent
import com.xetondownloadmanager.shared.util.OnFullyDismissed
import com.xetondownloadmanager.shared.util.ResponsiveDialog
import com.xetondownloadmanager.shared.util.rememberResponsiveDialogState

@Composable
fun UpdaterSheet(
    updaterComponent: UpdateComponent,
) {
    ShowUpdaterDialog(updaterComponent)
}

@Composable
private fun ShowUpdaterDialog(updaterComponent: UpdateComponent) {
    val showUpdate = updaterComponent.showNewUpdate.collectAsState().value
    val newVersion = updaterComponent.newVersionData.collectAsState().value
    val closeUpdatePage = {
        updaterComponent.requestClose()
    }
    RenderUpdateNotifications(updaterComponent)
    val isOpened = showUpdate && newVersion != null
    val state = rememberResponsiveDialogState(false)
    LaunchedEffect(isOpened) {
        if (isOpened) {
            state.show()
        } else {
            state.hide()
        }
    }
    state.OnFullyDismissed(closeUpdatePage)
    ResponsiveDialog(
        state, state::hide
    ) {
        newVersion?.let {
            NewUpdatePage(
                newVersionInfo = newVersion,
                currentVersion = updaterComponent.currentVersion,
                cancel = closeUpdatePage,
                update = {
                    updaterComponent.performUpdate()
                    closeUpdatePage()
                }
            )
        }
    }
}
