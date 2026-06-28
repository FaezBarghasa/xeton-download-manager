package com.xetondownloadmanager.android.action

import com.xetondownloadmanager.android.util.pagemanager.IBrowserPageManager
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xeton.util.compose.action.AnAction
import com.xeton.util.compose.action.simpleAction
import com.xeton.util.compose.asStringSource

fun createOpenBrowserAction(
    browserPageManager: IBrowserPageManager,
): AnAction {
    return simpleAction(
        Res.string.browser.asStringSource(),
        MyIcons.earth,
    ) {
        browserPageManager.openBrowser(null)
    }
}
