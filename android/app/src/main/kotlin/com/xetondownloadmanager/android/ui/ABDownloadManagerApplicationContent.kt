package com.xetondownloadmanager.android.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import com.xetondownloadmanager.android.ui.configurable.comon.CommonConfigurableRenderersForAndroid
import com.xetondownloadmanager.android.ui.configurable.comon.ConfigurableRenderersForAndroid
import com.xetondownloadmanager.android.util.AppInfo
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xetondownloadmanager.shared.ui.ProvideCommonSettings
import com.xetondownloadmanager.shared.ui.ProvideSizeUnits
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRendererRegistry
import com.xetondownloadmanager.shared.ui.theme.ABDownloaderTheme
import com.xetondownloadmanager.shared.ui.theme.ThemeManager
import com.xetondownloadmanager.shared.ui.widget.NotificationManager
import com.xetondownloadmanager.shared.ui.widget.ProvideLanguageManager
import com.xetondownloadmanager.shared.ui.widget.ProvideNotificationManager
import com.xetondownloadmanager.shared.util.PopUpContainer
import com.xetondownloadmanager.shared.util.ResponsiveBox
import com.xetondownloadmanager.shared.util.ui.ProvideDebugInfo
import com.xeton.util.compose.IIconResolver
import com.xeton.util.compose.localizationmanager.LanguageManager
import kotlin.collections.component1
import kotlin.collections.component2

@Composable
fun XetonDownloadManagerApplicationContent(
    languageManager: LanguageManager,
    themeManager: ThemeManager,
    appSettingsStorage: BaseAppSettingsStorage,
    iconResolver: IIconResolver,
    appRepository: BaseAppRepository,
    notificationManager: NotificationManager,
    content: @Composable () -> Unit,
) {
    val configurableRendererRegistry = remember {
        ConfigurableRendererRegistry {
            listOf(
                CommonConfigurableRenderersForAndroid,
                ConfigurableRenderersForAndroid
            ).forEach {
                it.getAllRenderers().forEach { (key, renderer) ->
                    this.register(key, renderer)
                }
            }
        }
    }
    ProvideDebugInfo(AppInfo.isInDebugMode) {
        ProvideLanguageManager(languageManager) {
            ProvideCommonSettings(
                appSettings = appSettingsStorage,
                iconProvider = iconResolver,
                configurableRendererRegistry = configurableRendererRegistry,
            ) {
                ProvideNotificationManager(notificationManager) {
                    val myColors by themeManager.currentThemeColor.collectAsState()
                    val uiScale by appSettingsStorage.uiScale.collectAsState()
                    ABDownloaderTheme(
                        myColors = myColors,
                        fontFamily = null,
                        uiScale = uiScale,
                    ) {
                        ResponsiveBox {
                            ProvideSizeUnits(
                                appRepository
                            ) {
                                PopUpContainer {
                                    content()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
