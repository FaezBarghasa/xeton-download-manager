package com.xetondownloadmanager.shared.storage

import androidx.datastore.core.DataStore
import com.xetondownloadmanager.shared.util.ConfigBaseSettingsByJson
import com.xetondownloadmanager.shared.util.proxy.IProxyStorage
import com.xetondownloadmanager.shared.util.proxy.ProxyData

class ProxyDatastoreStorage(
    dataStore: DataStore<ProxyData>,
) : IProxyStorage, ConfigBaseSettingsByJson<ProxyData>(dataStore) {
    override val proxyDataFlow = data
}
