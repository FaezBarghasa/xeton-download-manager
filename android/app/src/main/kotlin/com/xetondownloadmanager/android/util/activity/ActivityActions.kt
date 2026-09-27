package com.xetondownloadmanager.android.util.activity

import android.content.Intent
import com.xetondownloadmanager.shared.util.mvi.ContainsEffects

interface ActivityActions {
    fun startActivityAction(intent: Intent)
    fun finishActivityAction()
}
