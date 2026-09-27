package com.xetondownloadmanager.android.ui.configurable.comon.renderer

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.android.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.android.ui.configurable.NextIcon
import com.xetondownloadmanager.android.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.ui.configurable.item.NavigatableConfigurable
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.util.ui.widget.MyIcon

object NavigatableConfigurableRenderer : ConfigurableRenderer<NavigatableConfigurable> {
    @Composable
    override fun RenderConfigurable(
        configurable: NavigatableConfigurable,
        configurableUiProps: ConfigurableUiProps
    ) {
        RenderPerHostSettingsConfigurable(
            cfg = configurable,
            configurableUiProps = configurableUiProps,
            onRequestOpenConfigWindow = configurable.onRequestNavigate,
        )
    }

    @Composable
    private fun RenderPerHostSettingsConfigurable(
        cfg: NavigatableConfigurable,
        configurableUiProps: ConfigurableUiProps,
        onRequestOpenConfigWindow: () -> Unit
    ) {
//    val value by cfg.stateFlow.collectAsState()
//    val setValue = cfg::set
//    val enabled = isConfigEnabled()

        ConfigTemplate(
            modifier = configurableUiProps.modifier
                .clickable {
                    onRequestOpenConfigWindow()
                }
                .padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(cfg, true)
            },
            value = {
                NextIcon()
            },
        )
    }

}
