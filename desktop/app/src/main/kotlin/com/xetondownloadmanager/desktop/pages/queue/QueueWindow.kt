package com.xetondownloadmanager.desktop.pages.queue

import com.xetondownloadmanager.desktop.window.custom.CustomWindow
import com.xetondownloadmanager.shared.util.mvi.HandleEffects
import androidx.compose.runtime.Composable
import androidx.compose.ui.window.rememberWindowState

@Composable
fun QueuesWindow(queuesComponent: QueuesComponent) {
    val state = rememberWindowState()
    CustomWindow(
        state = state,
        onCloseRequest = queuesComponent.close
    ) {
        HandleEffects(queuesComponent) {
            if (it == QueuesComponentEffects.ToFront) {
                state.isMinimized = false
                window.toFront()
            }
        }
        QueuePage(queuesComponent)
    }
}
