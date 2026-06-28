package com.xetondownloadmanager.android.ui.configurable.comon.renderer

import android.content.Context
import android.media.RingtoneManager
import android.net.Uri
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.xetondownloadmanager.android.ui.configurable.ConfigTemplate
import com.xetondownloadmanager.android.ui.configurable.NextIcon
import com.xetondownloadmanager.android.ui.configurable.SheetInput
import com.xetondownloadmanager.android.ui.configurable.TitleAndDescription
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableRenderer
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableUiProps
import com.xetondownloadmanager.shared.ui.configurable.item.SoundConfigurable
import com.xetondownloadmanager.shared.ui.widget.IconActionButton
import com.xetondownloadmanager.shared.ui.widget.Text
import com.xetondownloadmanager.shared.util.div
import com.xetondownloadmanager.shared.util.notification.INotificationSound
import com.xetondownloadmanager.shared.util.notification.platformNotificationSound
import com.xetondownloadmanager.shared.util.ui.WithContentAlpha
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.util.ui.myColors
import com.xetondownloadmanager.shared.util.ui.theme.mySpacings
import com.xetondownloadmanager.shared.util.ui.widget.MyIcon
import com.xeton.util.compose.asStringSource
import com.xeton.util.ifThen

object SoundConfigurableRenderer : ConfigurableRenderer<SoundConfigurable> {
    @Composable
    override fun RenderConfigurable(configurable: SoundConfigurable, configurableUiProps: ConfigurableUiProps) {
        RenderStringConfig(configurable, configurableUiProps)
    }

    @Composable
    fun RenderStringConfig(cfg: SoundConfigurable, configurableUiProps: ConfigurableUiProps) {
        val value by cfg.stateFlow.collectAsState()
        val setValue = cfg::set
        var isOpened by remember { mutableStateOf(false) }
        val onDismiss = {
            isOpened = false
        }
        ConfigTemplate(
            modifier = configurableUiProps.modifier
                .clickable { isOpened = true }
                .padding(configurableUiProps.itemPaddingValues),
            title = {
                TitleAndDescription(cfg, true)
            },
            value = {
                Row(
                    Modifier,
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.End,
                ) {
                    if (value != INotificationSound.DEFAULT_VALUE) {
                        IconActionButton(
                            MyIcons.undo,
                            Res.string.reset.asStringSource(),
                            onClick = {
                                setValue(INotificationSound.DEFAULT_VALUE)
                            }
                        )
                        Spacer(Modifier.width(mySpacings.smallSpace))
                    }
                    IconActionButton(
                        MyIcons.resume,
                        "".asStringSource(),
                        onClick = {
                            platformNotificationSound().actualPlay(value)
                        }
                    )
                }
            },
            nestedContent = {
            }
        )
        SheetInput(
            configurable = cfg,
            isOpened = isOpened,
            onDismiss = onDismiss,
            inputContent = { params ->
                val context = LocalContext.current
                val notifications = remember {
                    buildList {
                        add(Res.string.default.asStringSource() to null)
                        addAll(
                            loadNotificationSounds(context).map {
                                it.first.asStringSource() to it.second
                            }
                        )
                    }
                }
                LazyColumn(
                    modifier = params.modifier,
                ) {
                    items(
                        notifications
                    ) { (name, uri) ->
                        val isSelected = params.editingValue == uri?.toString().orEmpty()
                        Row(
                            Modifier
                                .fillMaxWidth()
                                .heightIn(min = mySpacings.thumbSize)
                                .ifThen(isSelected) {
                                    background(myColors.onBackground / 0.05f)
                                }
                                .clickable {
                                    val audioSource = uri?.toString().orEmpty()
                                    params.setEditingValue(audioSource)
                                    platformNotificationSound().actualPlay(audioSource)
                                },
                            verticalAlignment = Alignment.CenterVertically,
                        ) {
                            WithContentAlpha(
                                if (isSelected) 1f else 0.75f
                            ) {
                                Text(
                                    name.rememberString(),
                                    modifier = Modifier.weight(1f)
                                )
                                if (isSelected) {
                                    MyIcon(
                                        MyIcons.check,
                                        contentDescription = null,
                                        modifier = Modifier.size(mySpacings.iconSize)
                                    )
                                }
                            }
                        }
                    }
                }
            },
            onConfirm = {
                cfg.set(it)
                onDismiss()
            },
        )
    }
}

private fun loadNotificationSounds(context: Context): List<Pair<String, Uri>> {
    val manager = RingtoneManager(context).apply {
        setType(RingtoneManager.TYPE_NOTIFICATION)
    }

    val cursor = manager.cursor
    val list = mutableListOf<Pair<String, Uri>>()

    while (cursor.moveToNext()) {
        val title = cursor.getString(RingtoneManager.TITLE_COLUMN_INDEX)
        val uri = manager.getRingtoneUri(cursor.position)
        list.add(title to uri)
    }

    cursor.close()
    return list
}
