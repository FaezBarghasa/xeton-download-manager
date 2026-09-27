package com.xetondownloadmanager.desktop.ui.configurable.platform

import com.xetondownloadmanager.desktop.ui.configurable.platform.item.FontConfigurable
import com.xetondownloadmanager.shared.ui.configurable.item.ProxyConfigurable
import com.xetondownloadmanager.desktop.ui.configurable.platform.renderer.FontConfigurableRenderer
import com.xetondownloadmanager.desktop.ui.configurable.comon.renderer.ProxyConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.Configurable
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.ContainsConfigurableRenderers

data class DesktopConfigurableRenderers(
    val fontConfigurableRenderer: ConfigurableRenderer<FontConfigurable>,
) : ContainsConfigurableRenderers {
    override fun getAllRenderers(): Map<Configurable.Key, ConfigurableRenderer<*>> {
        return mapOf(
            FontConfigurable.Key to fontConfigurableRenderer,
        )
    }
}

val PlatformConfigurableRenderersForDesktop = DesktopConfigurableRenderers(
    fontConfigurableRenderer = FontConfigurableRenderer,
)
