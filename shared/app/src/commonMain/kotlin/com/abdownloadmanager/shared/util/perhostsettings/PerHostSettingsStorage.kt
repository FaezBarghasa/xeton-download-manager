package com.xetondownloadmanager.shared.util.perhostsettings

import kotlinx.coroutines.flow.MutableStateFlow

interface IPerHostSettingsStorage {
    val perHostSettingsFlow: MutableStateFlow<List<PerHostSettingsItem>>
}
