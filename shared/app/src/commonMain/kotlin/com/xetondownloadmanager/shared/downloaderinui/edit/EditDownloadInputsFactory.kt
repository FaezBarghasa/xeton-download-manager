package com.xetondownloadmanager.shared.downloaderinui.edit

import com.xetondownloadmanager.shared.downloaderinui.CredentialAndItemMapper
import com.xetondownloadmanager.shared.downloaderinui.DownloadSize
import com.xetondownloadmanager.shared.downloaderinui.LinkChecker
import com.xeton.downloader.connection.IResponseInfo
import com.xeton.downloader.downloaditem.IDownloadCredentials
import com.xeton.downloader.downloaditem.IDownloadItem
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow

interface EditDownloadInputsFactory<
        TDownloadItem : IDownloadItem,
        TCredentials : IDownloadCredentials,
        TResponseInfo : IResponseInfo,
        TDownloadSize : DownloadSize,
        TLinkChecker : LinkChecker<TCredentials, TResponseInfo, TDownloadSize>,
        TCredentialsToItemMapper : CredentialAndItemMapper<TCredentials, TDownloadItem>,
        TEditDownloadInputs : EditDownloadInputs<TDownloadItem, TCredentials, TResponseInfo, TDownloadSize, TLinkChecker, TCredentialsToItemMapper>
        > {
    fun createEditDownloadInputs(
        currentDownloadItem: MutableStateFlow<TDownloadItem>,
        editedDownloadItem: MutableStateFlow<TDownloadItem>,
        conflictDetector: DownloadConflictDetector,
        scope: CoroutineScope,
    ): TEditDownloadInputs
}


