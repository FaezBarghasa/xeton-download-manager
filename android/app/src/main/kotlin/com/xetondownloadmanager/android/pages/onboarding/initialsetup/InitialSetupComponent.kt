package com.xetondownloadmanager.android.pages.onboarding.initialsetup

import com.xetondownloadmanager.shared.settings.CommonSettings
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableGroup
import com.xetondownloadmanager.shared.ui.theme.ThemeManager
import com.xetondownloadmanager.shared.util.BaseComponent
import com.arkivanov.decompose.ComponentContext
import com.xeton.util.compose.localizationmanager.LanguageManager

class InitialSetupComponent(
    ctx: ComponentContext,
    private val languageManager: LanguageManager,
    private val themeManager: ThemeManager,
    private val onFinish: () -> Unit
) : BaseComponent(ctx) {
    val configurables = listOf(
            CommonSettings.languageConfig(languageManager, scope),
            CommonSettings.themeConfig(themeManager, scope),
        )

    fun onUserPressFinish() {
        onFinish()
    }
}
