package com.xetondownloadmanager.desktop.ui.configurable.comon.renderer

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.PointerIcon
import androidx.compose.ui.input.pointer.pointerHoverIcon
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.desktop.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.desktop.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.ui.configurable.item.FolderConfigurable
import com.xetondownloadmanager.desktop.ui.util.rememberMyDirectoryPickerLauncher
import com.xetondownloadmanager.shared.ui.widget.MyTextField
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.util.ui.theme.myShapes
import com.xetondownloadmanager.shared.util.ui.widget.MyIcon
import java.io.File

object FolderConfigurableRenderer : ConfigurableRenderer<FolderConfigurable> {
    @Composable
    override fun RenderConfigurable(configurable: FolderConfigurable, configurableUiProps: ConfigurableUiProps) {
        RenderFolderConfig(configurable, configurableUiProps)
    }

    @Composable
    private fun RenderFolderConfig(cfg: FolderConfigurable, configurableUiProps: ConfigurableUiProps) {
        val value by cfg.stateFlow.collectAsState()
        val setValue = cfg::set

        val pickFolderLauncher = rememberMyDirectoryPickerLauncher(
            title = cfg.title.rememberString(),
            initialDirectory = remember(value) {
                runCatching {
                    File(value).canonicalPath
                }.getOrNull()
            },
        ) { directory ->
            directory?.let(setValue)
        }


        ConfigTemplate(
            modifier = configurableUiProps.modifier.padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(cfg, true)
            },
            value = {
                MyTextField(
                    modifier = Modifier.fillMaxWidth(),
                    text = value,
                    onTextChange = {
                        setValue(it)
                    },
                    shape = myShapes.defaultRounded,
                    textPadding = PaddingValues(4.dp),
                    placeholder = cfg.title.rememberString(),
                    end = {
                        MyIcon(
                            icon = MyIcons.folder,
                            contentDescription = null,
                            modifier = Modifier
                                .pointerHoverIcon(PointerIcon.Default)
                                .fillMaxHeight()
                                .clickable { pickFolderLauncher.launch() }
                                .wrapContentHeight()
                                .padding(horizontal = 8.dp)
                                .size(16.dp))
                    }
                )
            }
        )
    }
}
