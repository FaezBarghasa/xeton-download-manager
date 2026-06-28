package com.xetondownloadmanager.android.ui.configurable.android

import com.xetondownloadmanager.android.ui.configurable.android.item.PermissionConfigurable
import com.xetondownloadmanager.shared.ui.configurable.Configurable
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.ContainsConfigurableRenderers

data class AndroidConfigurableRenderers(
    val permissionConfigurableRenderers: ConfigurableRenderer<PermissionConfigurable>,
) : ContainsConfigurableRenderers {
    override fun getAllRenderers(): Map<Configurable.Key, ConfigurableRenderer<*>> {
        return mapOf(
            PermissionConfigurable.Key to permissionConfigurableRenderers,
        )
    }
}
