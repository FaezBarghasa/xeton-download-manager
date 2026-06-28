package com.xetondownloadmanager.desktop.pages.addDownload

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.rememberWindowState
import com.xetondownloadmanager.desktop.DesktopAddDownloadDialogManager
import com.xetondownloadmanager.desktop.pages.addDownload.multiple.DesktopAddMultiDownloadComponent
import com.xetondownloadmanager.desktop.pages.addDownload.multiple.AddMultiItemPage
import com.xetondownloadmanager.desktop.pages.addDownload.single.AddDownloadPage
import com.xetondownloadmanager.shared.pages.adddownload.single.BaseAddSingleDownloadComponent
import com.xetondownloadmanager.desktop.window.custom.CustomWindow
import com.xetondownloadmanager.desktop.window.custom.WindowIcon
import com.xetondownloadmanager.desktop.window.custom.WindowTitle
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.pages.adddownload.AddDownloadComponent
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.util.ui.theme.LocalUiScale
import com.xeton.util.compose.resources.myStringResource
import com.xeton.util.desktop.PlatformAppActivator
import com.xeton.util.desktop.screen.applyUiScale
import java.awt.Dimension

@Composable
fun ShowAddDownloadDialogs(component: DesktopAddDownloadDialogManager) {
    val openedAddDownloadDialogs = component.openedAddDownloadDialogs.collectAsState().value
    for (addDownloadComponent in openedAddDownloadDialogs) {
        key(addDownloadComponent.id) {
            AddDownloadWindow(
                addDownloadComponent = addDownloadComponent,
                onRequestClose = {
                    component.closeAddDownloadDialog(addDownloadComponent.id)
                }
            )
        }
    }
}

@Composable
private fun AddDownloadWindow(
    addDownloadComponent: AddDownloadComponent,
    onRequestClose: () -> Unit,
) {
    val shouldShowWindow by addDownloadComponent.shouldShowWindow.collectAsState()
    if (!shouldShowWindow) return
    val uiScale = LocalUiScale.current
    when (addDownloadComponent) {
        is BaseAddSingleDownloadComponent -> {
            val h = 265.applyUiScale(uiScale)
            val w = 500.applyUiScale(uiScale)
            val size = remember {
                DpSize(
                    height = h.dp,
                    width = w.dp,
                )
            }

            val state = rememberWindowState(
                size = size,
                position = WindowPosition(Alignment.Center)
            )
            CustomWindow(
                state = state,
                onCloseRequest = onRequestClose,
                alwaysOnTop = true,
            ) {
                LaunchedEffect(Unit) {
                    window.minimumSize = Dimension(w, h)
                    PlatformAppActivator.active()
                }
//                    BringToFront()
                WindowTitle(myStringResource(Res.string.add_download))
                WindowIcon(MyIcons.appIcon)
                AddDownloadPage(addDownloadComponent)
            }
        }

        is DesktopAddMultiDownloadComponent -> {
            val h = 450
            val w = 800
            val state = rememberWindowState(
                height = h.dp,
                width = w.dp,
                position = WindowPosition(Alignment.Center)
            )
            CustomWindow(
                state = state,
                onCloseRequest = onRequestClose,
                alwaysOnTop = true,
            ) {
                LaunchedEffect(Unit) {
                    window.minimumSize = Dimension(w, h)
                    PlatformAppActivator.active()
                }
//                    BringToFront()
                WindowTitle(myStringResource(Res.string.add_download))
                WindowIcon(MyIcons.appIcon)
                AddMultiItemPage(addDownloadComponent)
            }
        }
    }
}

//it seems not affect at all
//@Composable
//private fun WindowScope.BringToFront() {
//    LaunchedEffect(Unit) {
//        window.toFront()
//        window.requestFocus()
//    }
//}
