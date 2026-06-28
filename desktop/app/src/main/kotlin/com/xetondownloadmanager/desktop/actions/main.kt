package com.xetondownloadmanager.desktop.actions

import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.shared.util.SharedConstants
import com.xetondownloadmanager.desktop.di.Di
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.desktop.utils.AppInfo
import com.xetondownloadmanager.desktop.utils.DesktopEntryCreator
import com.xetondownloadmanager.desktop.utils.isAppInstalled
import com.xetondownloadmanager.desktop.window.Browser
import com.xeton.util.compose.action.MenuItem
import com.xeton.util.compose.action.buildMenu
import com.xeton.util.compose.action.simpleAction
import com.xetondownloadmanager.shared.util.getIcon
import com.xetondownloadmanager.shared.util.getName
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.action.createCheckForUpdateAction
import com.xetondownloadmanager.shared.action.createDownloadFromClipboardAction
import com.xetondownloadmanager.shared.action.createNewDownloadAction
import com.xetondownloadmanager.shared.action.createNewQueueAction
import com.xetondownloadmanager.shared.action.createOpenAboutPage
import com.xetondownloadmanager.shared.action.createOpenBatchDownloadAction
import com.xetondownloadmanager.shared.action.createOpenOpenSourceThirdPartyLibrariesPage
import com.xetondownloadmanager.shared.action.createOpenQueuesAction
import com.xetondownloadmanager.shared.action.createOpenSettingsAction
import com.xetondownloadmanager.shared.action.createOpenTranslatorsPageAction
import com.xetondownloadmanager.shared.action.createPerHostSettingsPage
import com.xetondownloadmanager.shared.action.createRequestExitAction
import com.xetondownloadmanager.shared.action.createStartQueueGroupAction
import com.xetondownloadmanager.shared.action.createStopQueueGroupAction
import com.xeton.downloader.queue.activeQueuesFlow
import com.xeton.util.URLOpener
import com.xeton.util.compose.asStringSource
import com.xeton.util.desktop.PlatformAppActivator
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.*
import org.koin.core.component.get

private val appComponent = Di.get<AppComponent>()
private val scope = Di.get<CoroutineScope>()
private val downloadSystem = appComponent.downloadSystem

private val activeQueuesFlow = downloadSystem
    .queueManager
    .activeQueuesFlow()
    .stateIn(
        scope,
        SharingStarted.WhileSubscribed(),
        emptyList()
    )

// desktop
val stopAllAction = createDesktopStopAllAction(scope, downloadSystem, appComponent, activeQueuesFlow)
val newDownloadAction = createNewDownloadAction(appComponent)
val newDownloadFromClipboardAction = createDownloadFromClipboardAction(appComponent)
val createDesktopEntryAction = simpleAction(
    Res.string.create_desktop_entry.asStringSource(),
    MyIcons.applicationFile,
    checkEnable = MutableStateFlow(AppInfo.isAppInstalled())
) {
    DesktopEntryCreator.createLinuxDesktopEntry()
}
val showDownloadList = simpleAction(
    Res.string.show_downloads.asStringSource(),
    MyIcons.download,
) {
    PlatformAppActivator.active()
    appComponent.openHome()
}
val browserIntegrations = MenuItem.SubMenu(
    title = Res.string.download_browser_integration.asStringSource(),
    icon = MyIcons.download,
    items = buildMenu {
        for (browserExtension in SharedConstants.browserIntegrations) {
            item(
                title = browserExtension.type.getName().asStringSource(),
                icon = browserExtension.type.getIcon(),
                onClick = {
                    val browser = Browser.getBrowserByType(browserExtension.type)
                    val success = browser?.openLink(browserExtension.url) == true
                    if (!success) {
                        URLOpener.openUrl(browserExtension.url)
                    }
                }
            )
        }
    }
)


// commonUsage but with desktop implementations
val newQueueAction = createNewQueueAction(scope, appComponent)
val openQueuesAction = createOpenQueuesAction(appComponent)
val openTranslators = createOpenTranslatorsPageAction(appComponent)
val openAboutAction = createOpenAboutPage(appComponent)
val checkForUpdateAction = createCheckForUpdateAction(appComponent.updater)
val gotoSettingsAction = createOpenSettingsAction(appComponent)
val perHostSettings = createPerHostSettingsPage(appComponent)
val requestExitAction = createRequestExitAction(scope, appComponent)
val startQueueGroupAction = createStartQueueGroupAction(scope, appComponent.downloadSystem.queueManager)
val stopQueueGroupAction = createStopQueueGroupAction(scope, activeQueuesFlow)
val batchDownloadAction = createOpenBatchDownloadAction(appComponent)
val openOpenSourceThirdPartyLibraries = createOpenOpenSourceThirdPartyLibrariesPage(appComponent)
