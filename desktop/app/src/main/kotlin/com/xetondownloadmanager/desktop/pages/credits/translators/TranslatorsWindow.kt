package com.xetondownloadmanager.desktop.pages.credits.translators

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.rememberWindowState
import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.desktop.window.custom.CustomWindow
import com.xetondownloadmanager.desktop.window.custom.WindowTitle
import com.xetondownloadmanager.resources.Res
import com.xeton.util.compose.resources.myStringResource


@Composable
fun ShowTranslators(
    appComponent: AppComponent,
) {
    TranslatorsWindow(
        isVisible = appComponent.showTranslators.collectAsState().value,
        onRequestClose = {
            appComponent.closeTranslatorsPage()
        }
    )
}

@Composable
private fun TranslatorsWindow(
    isVisible: Boolean,
    onRequestClose: () -> Unit,
) {
    if (!isVisible) return
    CustomWindow(
        onCloseRequest = onRequestClose,
        state = rememberWindowState(
            size = DpSize(650.dp, 500.dp)
        )
    ) {
        WindowTitle(myStringResource(Res.string.meet_the_translators))
        Translators(
            modifier = Modifier.fillMaxSize(),
        )
    }
}