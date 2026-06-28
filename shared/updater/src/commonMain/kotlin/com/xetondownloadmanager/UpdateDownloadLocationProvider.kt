package com.xetondownloadmanager

import java.io.File

fun interface UpdateDownloadLocationProvider {
    fun getSaveLocation(): File
}