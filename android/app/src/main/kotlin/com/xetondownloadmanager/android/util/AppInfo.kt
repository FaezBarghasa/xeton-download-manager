package com.xetondownloadmanager.android.util

import android.app.Application
import com.xetondownloadmanager.android.BuildConfig
import com.xetondownloadmanager.shared.util.AppVersion
import com.xetondownloadmanager.shared.util.SharedConstants
import com.xeton.util.platform.Platform
import okio.Path.Companion.toOkioPath

object AppInfo {
    val isInDebugMode: Boolean = BuildConfig.DEBUG
    lateinit var context: Application
    fun init(context: Application) {
        this.context = context
    }

    val platform = Platform.Android
    val version = AppVersion.get()

    val definedPaths by lazy {
        AndroidDefinedPaths(
            dataDir = context.filesDir.resolve(
                SharedConstants.dataDirName
            ).toOkioPath()
        )
    }
}
