package com.xetondownloadmanager.android.ui.configurable

import androidx.compose.foundation.layout.RowScope
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import com.xetondownloadmanager.android.ui.SheetHeader
import com.xetondownloadmanager.android.ui.SheetTitle
import com.xetondownloadmanager.android.ui.SheetUI
import com.xetondownloadmanager.shared.ui.widget.Text
import com.xetondownloadmanager.shared.util.OnFullyDismissed
import com.xetondownloadmanager.shared.util.ResponsiveDialog
import com.xetondownloadmanager.shared.util.rememberResponsiveDialogState
import com.xeton.util.compose.StringSource

@Composable
fun ConfigurableSheet(
    title: StringSource,
    isOpened: Boolean,
    onDismiss: () -> Unit,
    headerActions: @Composable RowScope.() -> Unit = {},
    content: @Composable () -> Unit,
) {
    val dialogState = rememberResponsiveDialogState(isOpened)
    LaunchedEffect(isOpened) {
        when (isOpened) {
            true -> dialogState.show()
            false -> dialogState.hide()
        }
    }
    dialogState.OnFullyDismissed {
        onDismiss()
    }
    ResponsiveDialog(
        state = dialogState,
        onDismiss = dialogState::hide,
    ) {
        SheetUI(
            header = {
                SheetHeader(
                    headerTitle = {
                        SheetTitle(
                            title.rememberString()
                        )
                    },
                    headerActions = headerActions,
                )
            }
        ) {
            content()
        }
    }
}
