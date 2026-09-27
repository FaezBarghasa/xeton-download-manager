package com.xetondownloadmanager.shared.downloaderinui.hls.add

import com.xetondownloadmanager.shared.downloaderinui.DownloadSize
import com.xetondownloadmanager.shared.downloaderinui.add.NewDownloadUiChecker
import com.xetondownloadmanager.shared.downloaderinui.LinkCheckerFactory
import com.xeton.downloader.downloaditem.hls.HLSDownloadCredentials
import com.xetondownloadmanager.shared.downloaderinui.hls.HLSLinkChecker
import com.xeton.downloader.downloaditem.hls.HLSResponseInfo
import com.xetondownloadmanager.shared.util.DownloadSystem
import kotlinx.coroutines.CoroutineScope

class HLSNewDownloadUIChecker(
    initCredentials: HLSDownloadCredentials,
    linkCheckerFactory: LinkCheckerFactory<HLSDownloadCredentials, HLSResponseInfo, DownloadSize.Duration, HLSLinkChecker>,
    initialFolder: String,
    initialName: String,
    downloadSystem: DownloadSystem,
    scope: CoroutineScope,
) : NewDownloadUiChecker<HLSDownloadCredentials, HLSResponseInfo, DownloadSize.Duration, HLSLinkChecker>(
    initialCredentials = initCredentials,
    linkCheckerFactory = linkCheckerFactory,
    initialFolder = initialFolder,
    initialName = initialName,
    downloadSystem = downloadSystem,
    scope = scope,
) {
}
