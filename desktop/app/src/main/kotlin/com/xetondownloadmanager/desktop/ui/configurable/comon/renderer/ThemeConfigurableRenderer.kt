package com.xetondownloadmanager.desktop.ui.configurable.comon.renderer

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.desktop.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.RenderSpinner
import com.xetondownloadmanager.desktop.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.ui.configurable.isConfigEnabled
import com.xetondownloadmanager.shared.ui.configurable.item.ThemeConfigurable
import com.xetondownloadmanager.shared.ui.widget.Text
import com.xetondownloadmanager.shared.util.ui.myColors
import com.xetondownloadmanager.shared.util.ui.theme.myTextSizes
import com.xeton.util.ifThen

object ThemeConfigurableRenderer : ConfigurableRenderer<ThemeConfigurable> {
    @Composable
    override fun RenderConfigurable(configurable: ThemeConfigurable, configurableUiProps: ConfigurableUiProps) {
        RenderThemeConfig(configurable, configurableUiProps)
    }

    @Composable
    private fun RenderThemeConfig(cfg: ThemeConfigurable, configurableUiProps: ConfigurableUiProps) {
        val value by cfg.stateFlow.collectAsState()
        val setValue = cfg::set
        val enabled = isConfigEnabled()
        ConfigTemplate(
            modifier = configurableUiProps.modifier.padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(cfg, true)
            },
            value = {
                RenderSpinner(
                    possibleValues = cfg.possibleValues, value = value, onSelect = {
                        setValue(it)
                    },
                    modifier = Modifier.widthIn(min = 160.dp),
                    enabled = enabled,
                    valueToString = cfg.valueToString,
                    render = {
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            modifier = Modifier.ifThen(!enabled) {
                                alpha(0.5f)
                            }
                        ) {
                            Spacer(
                                Modifier
                                    .clip(CircleShape)
                                    .border(
                                        1.dp,
                                        Brush.verticalGradient(myColors.primaryGradientColors),
                                        CircleShape
                                    )
                                    .padding(1.dp)
                                    .background(
                                        it.color,
                                    )
                                    .size(16.dp)
                            )
                            Spacer(Modifier.width(16.dp))
                            Text(cfg.describe(it).rememberString(), fontSize = myTextSizes.lg)
                        }
                    })
            }
        )
    }

}
