package com.xetondownloadmanager.android.pages.queue

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.android.ui.SheetHeader
import com.xetondownloadmanager.android.ui.SheetTitle
import com.xetondownloadmanager.android.ui.SheetUI
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.ui.configurable.ConfigurableGroup
import com.xetondownloadmanager.shared.ui.configurable.RenderConfigurableGroup
import com.xetondownloadmanager.shared.util.OnFullyDismissed
import com.xetondownloadmanager.shared.util.ResponsiveDialog
import com.xetondownloadmanager.shared.util.ResponsiveDialogScope
import com.xetondownloadmanager.shared.util.rememberResponsiveDialogState
import com.xeton.util.compose.resources.myStringResource

@Composable
fun QueueConfigSheet(
    queuesConfigurationComponent: QueueConfigurationComponent?,
    onDismiss: () -> Unit,
) {
    val state = rememberResponsiveDialogState(false)
    LaunchedEffect(
        queuesConfigurationComponent
    ) {
        if (queuesConfigurationComponent != null) {
            state.show()
        } else {
            state.hide()
        }
    }
    state.OnFullyDismissed(onDismiss)
    ResponsiveDialog(state, onDismiss = state::hide) {
        queuesConfigurationComponent?.let {
            QueueConfig(
                name = it.downloadQueue.queueModel.collectAsState().value.name,
                groups = it.configurations,
                onDismissRequest = state::hide,
            )
        }
    }
}

@Composable
private fun ResponsiveDialogScope.QueueConfig(
    name: String,
    groups: List<ConfigurableGroup>,
    onDismissRequest: () -> Unit,
) {
    SheetUI(
        header = {
            SheetHeader(
                headerTitle = {
                    val queues = myStringResource(Res.string.queues)
                    SheetTitle("${queues}: $name")
                }
            )
        }
    ) {
        Column(
            Modifier
                .verticalScroll(rememberScrollState())
        ) {
            for (group in groups) {
                RenderConfigurableGroup(
                    modifier = Modifier,
                    group = group,
                    itemPadding = PaddingValues(8.dp)
                )
            }
        }
    }
}
