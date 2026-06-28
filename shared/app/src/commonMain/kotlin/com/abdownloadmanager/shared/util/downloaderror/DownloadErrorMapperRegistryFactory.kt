package com.xetondownloadmanager.shared.util.downloaderror

import com.xetondownloadmanager.shared.util.downloaderror.definederrors.*

class DownloadErrorMapperRegistryFactory {
    private fun getAvailableMappers(): List<DownloadErrorMapper> {
        return listOf(
            HttpStatusDownloadErrorMapper,
            UnknownHostErrorMapper,
            ChangedToWebPageDownloadErrorMapper,
            ConnectionResetDownloadErrorMapper,
            DestinationExceptionDownloadErrorMapper,
            EtagChangedDownloadErrorMapper,
            SizeChangedDownloadErrorMapper,
            ResumeSupportChangedDownloadErrorMapper,
            TimeoutErrorMapper,
            SSLNotTrustedErrorMapper,
            NotEnoughStorageAvailableExceptionMapper(),
            // at last
            DefaultDownloadErrorMapper,
        )
    }


    fun createRegistry(): IDownloadErrorMapperRegistry {
        return DownloadErrorMapperRegistry(
            getAvailableMappers()
        )
    }
}
