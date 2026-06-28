package com.xetondownloadmanager.desktop.actions

import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.desktop.di.Di
import com.xetondownloadmanager.desktop.pages.poweractionalert.PowerActionComponent
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.ui.widget.MessageDialogType
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.action.createDummyExceptionAction
import com.xetondownloadmanager.shared.action.createDummyMessageAction
import com.xeton.util.compose.action.AnAction
import com.xeton.util.compose.action.MenuItem
import com.xeton.util.compose.action.simpleAction
import com.xeton.util.compose.asStringSource
import com.xeton.util.desktop.poweraction.PowerActionConfig
import org.koin.core.component.get

private val appComponent = Di.get<AppComponent>()
val dummyMessage = createDummyMessageAction(appComponent)
val dummyException = createDummyExceptionAction()
val shutdown = simpleAction(
    Res.string.shutdown_now.asStringSource(),
    MyIcons.exit,
) {
    appComponent.initiatePowerAction(
        PowerActionConfig(PowerActionConfig.Type.Shutdown, false),
        PowerActionComponent.PowerActionReason.Unknown
    )
}
