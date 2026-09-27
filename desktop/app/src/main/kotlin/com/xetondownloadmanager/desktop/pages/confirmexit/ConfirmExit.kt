package com.xetondownloadmanager.desktop.pages.confirmexit

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.desktop.ui.widget.ConfirmDialog
import com.xetondownloadmanager.desktop.ui.widget.ConfirmDialogType
import com.xetondownloadmanager.resources.Res
import com.xeton.util.compose.asStringSource

@Composable
fun ConfirmExit(appComponent: AppComponent) {
    val showExitDialog by appComponent.showConfirmExitDialog.collectAsState()
    if (showExitDialog) {
        ConfirmDialog(
            Res.string.confirm_exit.asStringSource(),
            Res.string.confirm_exit_description.asStringSource(),
            onCancel = appComponent::closeConfirmExit,
            onConfirm = appComponent::exitAppAsync,
            type = ConfirmDialogType.Warning,
        )
    }
}