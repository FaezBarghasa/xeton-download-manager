package com.xetondownloadmanager.shared.downloaderinui.http.add

import com.xetondownloadmanager.shared.downloaderinui.DownloadSize
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.shared.downloaderinui.add.NewDownloadUiChecker
import com.xetondownloadmanager.shared.downloaderinui.LinkCheckerFactory
import com.xeton.downloader.connection.response.HttpResponseInfo
import com.xeton.downloader.downloaditem.http.HttpDownloadCredentials
import kotlinx.coroutines.CoroutineScope

class HttpNewDownloadUiChecker(
    initialCredentials: HttpDownloadCredentials = HttpDownloadCredentials.Companion.empty(),
    linkCheckerFactory: LinkCheckerFactory<HttpDownloadCredentials, HttpResponseInfo, DownloadSize.Bytes, HttpLinkChecker>,
    initialFolder: String,
    initialName: String = "",
    downloadSystem: DownloadSystem,
    scope: CoroutineScope,
) : NewDownloadUiChecker<HttpDownloadCredentials, HttpResponseInfo, DownloadSize.Bytes, HttpLinkChecker>(
    initialCredentials, linkCheckerFactory, initialFolder, initialName, downloadSystem, scope
) {
}
