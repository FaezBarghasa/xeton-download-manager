package com.xetondownloadmanager.shared.util.downloaderror

interface IDownloadErrorMapperRegistry {
    fun getReason(throwable: Throwable): DownloadErrorReason?
}
