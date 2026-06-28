package com.xetondownloadmanager.shared.storage.impl

import androidx.datastore.core.DataStore
import com.xetondownloadmanager.shared.storage.ILastSavedLocationsStorage
import com.xetondownloadmanager.shared.storage.ISelectQueueStorage
import com.xetondownloadmanager.shared.storage.SelectQueueSettings
import com.xetondownloadmanager.shared.util.ConfigBaseSettingsByJson
import kotlinx.coroutines.flow.MutableStateFlow

class SelectQueueStorage(
    dataStore: DataStore<SelectQueueSettings>
) : ConfigBaseSettingsByJson<SelectQueueSettings>(dataStore), ISelectQueueStorage {
    override val selectQueueSettings: MutableStateFlow<SelectQueueSettings> = data
}
