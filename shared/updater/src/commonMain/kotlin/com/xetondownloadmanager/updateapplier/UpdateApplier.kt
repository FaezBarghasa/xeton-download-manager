package com.xetondownloadmanager.updateapplier

import com.xetondownloadmanager.updatechecker.UpdateInfo

interface UpdateApplier {
    fun updateSupported(): Boolean
    suspend fun applyUpdate(updateInfo: UpdateInfo)
    suspend fun cleanup()
}