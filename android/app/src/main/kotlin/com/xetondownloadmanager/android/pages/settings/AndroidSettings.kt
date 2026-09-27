package com.xetondownloadmanager.android.pages.settings

import com.xetondownloadmanager.android.pages.onboarding.permissions.ABDMPermissions
import com.xetondownloadmanager.android.storage.AppSettingsStorage
import com.xetondownloadmanager.android.ui.configurable.android.item.PermissionConfigurable
import com.xetondownloadmanager.android.util.pagemanager.PermissionsPageManager
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.ui.configurable.item.BooleanConfigurable
import com.xetondownloadmanager.shared.ui.configurable.item.NavigatableConfigurable
import com.xeton.util.compose.asStringSource
import kotlinx.coroutines.flow.MutableStateFlow


object AndroidSettings {
    fun permissionSettings(
        permissionsPageManager: PermissionsPageManager
    ): NavigatableConfigurable {
        return NavigatableConfigurable(
            title = Res.string.permissions.asStringSource(),
            description = "".asStringSource(),
            onRequestNavigate = {
                permissionsPageManager.openPermissionsPage(false)
            },
        )
    }

    fun ignoreBatteryOptimizations(): PermissionConfigurable {
        val permission = ABDMPermissions.BatteryOptimizationPermission
        return PermissionConfigurable(
            title = permission.title,
            description = permission.description,
            backedBy = MutableStateFlow(permission),
        )
    }

    fun browserIconInLauncher(
        appSettingsStorage: AppSettingsStorage
    ): BooleanConfigurable {
        return BooleanConfigurable(
            title = Res.string.settings_browser_in_launcher.asStringSource(),
            description = Res.string.settings_browser_in_launcher_description.asStringSource(),
            backedBy = appSettingsStorage.browserIconInLauncher,
            describe = {
                if (it) {
                    Res.string.enabled
                } else {
                    Res.string.disabled
                }.asStringSource()
            }
        )
    }
}
