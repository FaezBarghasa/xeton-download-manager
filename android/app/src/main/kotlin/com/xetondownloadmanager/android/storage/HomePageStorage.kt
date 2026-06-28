package com.xetondownloadmanager.android.storage

import androidx.datastore.core.DataStore
import com.xetondownloadmanager.android.pages.home.HomePageStateToPersist
import com.xetondownloadmanager.android.pages.home.sortBy
import com.xetondownloadmanager.shared.util.ConfigBaseSettingsByJson

class HomePageStorage(
    dataStore: DataStore<HomePageStateToPersist>,
) : ConfigBaseSettingsByJson<HomePageStateToPersist>(
    dataStore = dataStore,
) {
    val sortBy = from(HomePageStateToPersist.sortBy)
}
