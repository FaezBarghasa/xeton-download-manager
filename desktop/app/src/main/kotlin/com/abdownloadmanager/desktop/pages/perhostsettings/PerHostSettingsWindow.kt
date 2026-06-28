package com.xetondownloadmanager.desktop.pages.perhostsettings

import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.rememberWindowState
import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.desktop.window.custom.CustomWindow
import com.xetondownloadmanager.shared.pages.perhostsettings.BasePerHostSettingsComponent
import com.xetondownloadmanager.shared.util.mvi.HandleEffects
import com.xetondownloadmanager.shared.util.rememberChild

@Composable
fun PerHostSettingsWindow(
    appComponent: AppComponent
) {
    val component = appComponent.perHostSettingsSlot.rememberChild()
    if (component != null) {
        val windowState = rememberWindowState(
            size = DpSize(
                600.dp,
                400.dp,
            ),
            position = WindowPosition.Aligned(Alignment.Center)
        )
        CustomWindow(
            state = windowState,
            onCloseRequest = appComponent::closePerHostSettings,
        ) {
            HandleEffects(component) {
                when (it) {
                    is BasePerHostSettingsComponent.Effects.Platform -> {
                        when (it as DesktopPerHostSettingsComponent.Effects) {
                            DesktopPerHostSettingsComponent.Effects.BringToFront -> {
                                windowState.isMinimized = false
                                window.toFront()
                            }
                        }
                    }
                }
            }
            PerHostSettingsPage(component)
        }
    }
}
