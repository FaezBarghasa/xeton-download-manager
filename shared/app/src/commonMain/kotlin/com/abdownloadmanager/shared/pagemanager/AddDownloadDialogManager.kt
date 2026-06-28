package com.xetondownloadmanager.shared.pagemanager

import com.xetondownloadmanager.shared.pages.adddownload.AddDownloadCredentialsInUiProps
import com.xetondownloadmanager.shared.pages.adddownload.ImportOptions

interface AddDownloadDialogManager {
    fun closeAddDownloadDialog()
    fun openAddDownloadDialog(
        links: List<AddDownloadCredentialsInUiProps>,
        importOptions: ImportOptions = ImportOptions(),
    )
}
