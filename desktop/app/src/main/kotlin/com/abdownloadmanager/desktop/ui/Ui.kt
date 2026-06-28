package com.xetondownloadmanager.desktop.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.window.ApplicationScope
import androidx.compose.ui.window.application
import com.xetondownloadmanager.desktop.AppArguments
import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.desktop.AppEffects
import com.xetondownloadmanager.desktop.actions.gotoSettingsAction
import com.xetondownloadmanager.desktop.actions.requestExitAction
import com.xetondownloadmanager.desktop.actions.showDownloadList
import com.xetondownloadmanager.desktop.pages.about.ShowAboutDialog
import com.xetondownloadmanager.desktop.pages.addDownload.ShowAddDownloadDialogs
import com.xetondownloadmanager.desktop.pages.batchdownload.BatchDownloadWindow
import com.xetondownloadmanager.desktop.pages.category.ShowCategoryDialogs
import com.xetondownloadmanager.desktop.pages.confirmexit.ConfirmExit
import com.xetondownloadmanager.desktop.pages.credits.translators.ShowTranslators
import com.xetondownloadmanager.desktop.pages.editdownload.EditDownloadWindow
import com.xetondownloadmanager.desktop.pages.enterurl.EnterNewDownloadWindow
import com.xetondownloadmanager.desktop.pages.extenallibs.ShowOpenSourceLibraries
import com.xetondownloadmanager.desktop.pages.checksum.FileChecksumWindow
import com.xetondownloadmanager.desktop.pages.downloaderror.DownloadErrorDialog
import com.xetondownloadmanager.desktop.pages.home.HomeWindow
import com.xetondownloadmanager.desktop.pages.newQueue.NewQueueDialog
import com.xetondownloadmanager.desktop.pages.perhostsettings.PerHostSettingsWindow
import com.xetondownloadmanager.desktop.pages.queue.QueuesWindow
import com.xetondownloadmanager.desktop.pages.settings.FontManager
import com.xetondownloadmanager.desktop.pages.settings.SettingWindow
import com.xetondownloadmanager.shared.ui.theme.ThemeManager
import com.xetondownloadmanager.desktop.pages.poweractionalert.PowerActionAlert
import com.xetondownloadmanager.desktop.pages.singleDownloadPage.ShowDownloadDialogs
import com.xetondownloadmanager.desktop.pages.updater.ShowUpdaterDialog
import com.xetondownloadmanager.desktop.ui.configurable.comon.CommonConfigurableRenderersForDesktop
import com.xetondownloadmanager.desktop.ui.configurable.platform.PlatformConfigurableRenderersForDesktop
import com.xetondownloadmanager.desktop.ui.widget.Tray
import com.xetondownloadmanager.desktop.ui.widget.ShowMessageDialogs
import com.xetondownloadmanager.desktop.utils.AppInfo
import com.xetondownloadmanager.desktop.utils.GlobalAppExceptionHandler
import com.xetondownloadmanager.desktop.utils.ProvideGlobalExceptionHandler
import com.xetondownloadmanager.desktop.utils.isInDebugMode
import com.xetondownloadmanager.shared.ui.ProvideCommonSettings
import com.xetondownloadmanager.shared.ui.ProvideSizeUnits
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRendererRegistry
import com.xetondownloadmanager.shared.ui.theme.ABDownloaderTheme
import com.xetondownloadmanager.shared.ui.widget.NotificationManager
import com.xetondownloadmanager.shared.ui.widget.ProvideLanguageManager
import com.xetondownloadmanager.shared.ui.widget.ProvideNotificationManager
import com.xetondownloadmanager.shared.ui.widget.useNotification
import com.xetondownloadmanager.shared.util.mvi.HandleEffects
import com.xetondownloadmanager.shared.util.ui.ProvideDebugInfo
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xeton.util.compose.action.buildMenu
import com.xeton.util.compose.localizationmanager.LanguageManager
import com.xeton.util.desktop.PlatformDockToggler
import com.xeton.util.desktop.mac.event.MacEventHandler
import com.xeton.util.platform.Platform
import com.xeton.util.platform.isMac
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withTimeout
import org.koin.core.component.KoinComponent
import org.koin.core.component.get
import org.koin.core.component.inject

object Ui : KoinComponent {
    val scope: CoroutineScope by inject()
    fun boot(
        appArguments: AppArguments,
        globalAppExceptionHandler: GlobalAppExceptionHandler,
    ) {
        val appComponent: AppComponent = get()
        val themeManager: ThemeManager = get()
        val fontManager: FontManager = get()
        val languageManager: LanguageManager = get()
        val notificationManager: NotificationManager = get()
        themeManager.boot()
        fontManager.boot()
        languageManager.boot()
        if (!appArguments.startSilent) {
            appComponent.openHome()
        }
        if (Platform.isMac()) {
            MacEventHandler.configure(
                onClickIcon = appComponent::activateHomeIfNotOpen,
                onAboutClick = {
                    appComponent.showAboutPage.value = true
                },
                onSettingsClick = appComponent::openSettings,
                onQuit = {
                    scope.launch { appComponent.requestExitApp() }
                }
            )
        }
        application {
            ProvideLocalProviders(
                languageManager = languageManager,
                appComponent = appComponent,
                themeManager = themeManager,
                fontManager = fontManager,
                globalAppExceptionHandler = globalAppExceptionHandler,
                notificationManager = notificationManager,
            ) {
                HandleEffectsForApp(appComponent)
                SystemTray(appComponent)
                val showHomeSlot =
                    appComponent.showHomeSlot.collectAsState().value
                showHomeSlot.child?.instance?.let {
                    HomeWindow(it, appComponent::closeHome)
                }
                val showSettingSlot =
                    appComponent.showSettingSlot.collectAsState().value
                showSettingSlot.child?.instance?.let {
                    SettingWindow(it, appComponent::closeSettings)
                }
                val showQueuesSlot =
                    appComponent.showQueuesSlot.collectAsState().value
                showQueuesSlot.child?.instance?.let {
                    QueuesWindow(it)
                }
                val batchDownloadSlot =
                    appComponent.batchDownloadSlot.collectAsState().value
                batchDownloadSlot.child?.instance?.let {
                    BatchDownloadWindow(it)
                }
                val editDownloadSlot =
                    appComponent.editDownloadSlot.collectAsState().value
                editDownloadSlot.child?.instance?.let {
                    EditDownloadWindow(it)
                }
                EnterNewDownloadWindow(appComponent)
                ShowAddDownloadDialogs(appComponent)
                ShowDownloadDialogs(appComponent)
                ShowCategoryDialogs(appComponent)
                FileChecksumWindow(appComponent)
                ShowUpdaterDialog(appComponent.updater)
                ShowAboutDialog(appComponent)
                NewQueueDialog(appComponent)
                ShowMessageDialogs(appComponent)
                ShowOpenSourceLibraries(appComponent)
                ShowTranslators(appComponent)
                ConfirmExit(appComponent)
                PowerActionAlert(appComponent)
                PerHostSettingsWindow(appComponent)
                DownloadErrorDialog(appComponent)
            }
        }
    }
}

@Composable
private fun ProvideLocalProviders(
    languageManager: LanguageManager,
    themeManager: ThemeManager,
    fontManager: FontManager,
    appComponent: AppComponent,
    notificationManager: NotificationManager,
    globalAppExceptionHandler: GlobalAppExceptionHandler,
    content: @Composable () -> Unit
) {
    val theme by themeManager.currentThemeColor.collectAsState()
    val fontFamily by fontManager.currentFontFamily.collectAsState()
    val configurableRendererRegistry = remember {
        ConfigurableRendererRegistry {
            listOf(
                PlatformConfigurableRenderersForDesktop,
                CommonConfigurableRenderersForDesktop,
            ).forEach {
                it.getAllRenderers().forEach { (key, renderer) ->
                    this.register(key, renderer)
                }
            }
        }
    }
    ProvideDebugInfo(AppInfo.isInDebugMode()) {
        ProvideLanguageManager(languageManager) {
            ProvideCommonSettings(
                appSettings = appComponent.appSettings,
                configurableRendererRegistry = configurableRendererRegistry,
                iconProvider = appComponent.iconFromUriResolver
            ) {
                ProvideNotificationManager(notificationManager) {
                    ABDownloaderTheme(
                        myColors = theme,
                        fontFamily = fontFamily,
                        uiScale = appComponent.uiScale.collectAsState().value
                    ) {
                        ProvideGlobalExceptionHandler(globalAppExceptionHandler) {
                            ProvideSizeUnits(appComponent.appRepository) {
                                content()
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun HandleEffectsForApp(appComponent: AppComponent) {
    val notificationManager = useNotification()
    val scope = rememberCoroutineScope()
    HandleEffects(appComponent) {
        when (it) {
            is AppEffects.SimpleNotificationNotification -> {
                scope.launch {
                    withTimeout(5000) {
                        notificationManager.showNotification(it.notificationModel)
                    }
                }
            }
        }
    }
}

@Composable
private fun ApplicationScope.SystemTray(
    component: AppComponent,
) {
    val useSystemTray by component.useSystemTray.collectAsState()
    if (useSystemTray) {
        LaunchedEffect(Unit) { PlatformDockToggler.hide() }
        val menu = remember {
            buildMenu {
                +showDownloadList
                +gotoSettingsAction
                +requestExitAction
            }
        }
        Tray(
            icon = MyIcons.appIcon,
            tooltip = AppInfo.displayName,
            primaryAction = { showDownloadList.onClick() },
            menu = menu,
        )
    } else {
        LaunchedEffect(Unit) { PlatformDockToggler.show() }
    }
}
