package com.xetondownloadmanager.shared.storage

import androidx.datastore.core.DataStore
import com.xetondownloadmanager.shared.util.ConfigBaseSettingsByJson
import com.xetondownloadmanager.shared.util.perhostsettings.IPerHostSettingsStorage
import com.xetondownloadmanager.shared.util.perhostsettings.PerHostSettingsItem
import kotlinx.coroutines.flow.MutableStateFlow

class PerHostSettingsDatastoreStorage(
    dataStore: DataStore<List<PerHostSettingsItem>>,
) : IPerHostSettingsStorage, ConfigBaseSettingsByJson<List<PerHostSettingsItem>>(dataStore) {
    override val perHostSettingsFlow: MutableStateFlow<List<PerHostSettingsItem>> = data
}
