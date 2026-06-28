package com.xetondownloadmanager.shared.pagemanager

import com.xetondownloadmanager.shared.ui.widget.MessageDialogType
import com.xetondownloadmanager.shared.ui.widget.NotificationType
import com.xeton.util.compose.StringSource

interface NotificationSender {
    fun sendDialogNotification(title: StringSource, description: StringSource, type: MessageDialogType)
    fun sendNotification(tag: Any, title: StringSource, description: StringSource, type: NotificationType)
}
