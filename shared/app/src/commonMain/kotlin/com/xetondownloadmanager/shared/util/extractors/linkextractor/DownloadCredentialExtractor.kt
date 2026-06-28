package com.xetondownloadmanager.shared.util.extractors.linkextractor

import com.xetondownloadmanager.shared.util.extractors.Extractor
import com.xeton.downloader.downloaditem.IDownloadCredentials


interface DownloadCredentialExtractor<T> : Extractor<T, List<IDownloadCredentials>> {
    override fun extract(input: T): List<IDownloadCredentials>
}

