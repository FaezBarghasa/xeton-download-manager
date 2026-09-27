package com.xetondownloadmanager.android.repository

import com.xetondownloadmanager.android.pages.browser.BrowserActivity
import com.xetondownloadmanager.android.storage.AppSettingsStorage
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.util.autoremove.RemovedDownloadsFromDiskTracker
import com.xetondownloadmanager.shared.util.category.CategoryManager
import com.xetondownloadmanager.shared.util.proxy.ProxyManager
import com.xeton.downloader.DownloadSettings
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

class AppRepository(
    scope: CoroutineScope,
    appSettings: AppSettingsStorage,
    proxyManager: ProxyManager,
    downloadSystem: DownloadSystem,
    downloadSettings: DownloadSettings,
    removedDownloadsFromDiskTracker: RemovedDownloadsFromDiskTracker,
    categoryManager: CategoryManager,
) : BaseAppRepository(
    scope = scope,
    appSettings = appSettings,
    proxyManager = proxyManager,
    downloadSystem = downloadSystem,
    downloadSettings = downloadSettings,
    removedDownloadsFromDiskTracker = removedDownloadsFromDiskTracker,
    categoryManager = categoryManager,
) {
    init {
        appSettings.browserIconInLauncher
            .debounce(500)
            .distinctUntilChanged()
            .onEach { enabled ->
                BrowserActivity.Companion.Launcher.setEnabled(enabled)
            }.launchIn(scope)
    }
}
