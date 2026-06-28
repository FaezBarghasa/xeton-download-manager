package com.xetondownloadmanager.shared.util

import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xeton.downloader.connection.UserAgentProvider

class UserAgentProviderFromSettings(
    private val appSettingsStorage: BaseAppSettingsStorage
) : UserAgentProvider {
    override fun getUserAgent(): String? {
        return appSettingsStorage.userAgent.value.takeIf { it.isNotBlank() }
    }
}
