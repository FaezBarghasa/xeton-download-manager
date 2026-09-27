package com.xetondownloadmanager.desktop.ui.configurable.comon.renderer

import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.desktop.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.desktop.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.ui.configurable.isConfigEnabled
import com.xetondownloadmanager.shared.ui.configurable.item.LongConfigurable
import com.xetondownloadmanager.shared.ui.widget.LongTextField

object LongConfigurableRenderer : ConfigurableRenderer<LongConfigurable> {
    @Composable
    override fun RenderConfigurable(configurable: LongConfigurable, configurableUiProps: ConfigurableUiProps) {
        RenderLongConfig(configurable, configurableUiProps)
    }


    private operator fun LongRange.get(index: Int): Long {
        return (start + index).also {
            if (it > last) {
                throw IndexOutOfBoundsException("$it bigger that $last")
            }
        }
    }

    @Composable
    private fun RenderLongConfig(cfg: LongConfigurable, configurableUiProps: ConfigurableUiProps) {
        val value by cfg.stateFlow.collectAsState()
        val setValue = cfg::set
        val enabled = isConfigEnabled()
        ConfigTemplate(
            modifier = configurableUiProps.modifier.padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(cfg, true)
            },
            value = {
                when (cfg.renderMode) {
                    LongConfigurable.RenderMode.TextField -> {
                        val interactionSource = remember { MutableInteractionSource() }
                        LongTextField(
                            value = value,
                            onValueChange = { v ->
                                setValue(v)
                            },
//                        colors = TextFieldDefaults.textFieldColors(
//                            backgroundColor = Color.Transparent
//                        ),
                            modifier = Modifier.width(200.dp),
                            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                            interactionSource = interactionSource,
                            range = cfg.range,
                            enabled = enabled,
                        )
                    }
                }
            })
    }

}
