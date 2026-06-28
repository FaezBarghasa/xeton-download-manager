package com.xetondownloadmanager.shared.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRendererRegistry
import com.xetondownloadmanager.shared.ui.configurable.LocalConfigurationRendererRegistry
import com.xetondownloadmanager.shared.util.LocalUseRelativeDateTime
import com.xetondownloadmanager.shared.util.ProvideSizeAndSpeedUnit
import com.xeton.util.compose.IIconResolver
import com.xeton.util.compose.LocalIconFromUriResolver


@Composable
fun ProvideCommonSettings(
    appSettings: BaseAppSettingsStorage,
    iconProvider: IIconResolver,
    configurableRendererRegistry: ConfigurableRendererRegistry,
    content: @Composable () -> Unit,
) {
    val useNativeDateTime by appSettings.useRelativeDateTime.collectAsState()
    CompositionLocalProvider(
        LocalUseRelativeDateTime provides useNativeDateTime,
        LocalIconFromUriResolver provides iconProvider,
        LocalConfigurationRendererRegistry provides configurableRendererRegistry,
    ) {
        content()
    }
}

@Composable
fun ProvideSizeUnits(
    appRepository: BaseAppRepository,
    content: @Composable () -> Unit,
) {
    ProvideSizeAndSpeedUnit(
        sizeUnitConfig = appRepository.sizeUnit.collectAsState().value,
        speedUnitConfig = appRepository.speedUnit.collectAsState().value,
        content = content
    )
}
