package com.xetondownloadmanager.desktop.ui.configurable.comon.renderer

import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import com.xetondownloadmanager.desktop.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.desktop.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.ui.configurable.isConfigEnabled
import com.xetondownloadmanager.shared.ui.configurable.item.BooleanConfigurable
import com.xetondownloadmanager.shared.ui.widget.CheckBox
import com.xetondownloadmanager.shared.ui.widget.Switch

object BooleanConfigurableRenderer : ConfigurableRenderer<BooleanConfigurable> {
    @Composable
    override fun RenderConfigurable(configurable: BooleanConfigurable, configurableUiProps: ConfigurableUiProps) {
        RenderBooleanConfig(configurable, configurableUiProps)
    }

    @Composable
    private fun RenderBooleanConfig(
        cfg: BooleanConfigurable,
        configurableUiProps: ConfigurableUiProps,
    ) {
        val checked = cfg.stateFlow.collectAsState().value
        val setValue = cfg::set
        val enabled = isConfigEnabled()
        ConfigTemplate(
            modifier = configurableUiProps.modifier.padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(cfg, true)
            },
            value = {
                when (cfg.renderMode) {
                    BooleanConfigurable.RenderMode.Checkbox -> {
                        CheckBox(
                            value = checked,
                            enabled = enabled,
                            onValueChange = {
                                setValue(it)
                            }
                        )
                    }

                    BooleanConfigurable.RenderMode.Switch -> {
                        Switch(
                            checked = checked,
                            enabled = enabled,
                            onCheckedChange = {
                                setValue(it)
                            }
                        )
                    }
                }
            })
    }
}
