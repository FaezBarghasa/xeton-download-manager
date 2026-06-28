package com.xetondownloadmanager.desktop.integration

import com.xetondownloadmanager.desktop.AppComponent
import com.xetondownloadmanager.shared.pages.adddownload.AddDownloadCredentialsInUiProps
import com.xetondownloadmanager.shared.pages.adddownload.ImportOptions
import com.xetondownloadmanager.shared.pages.adddownload.SilentImportOptions
import com.xetondownloadmanager.desktop.repository.AppRepository
import com.xetondownloadmanager.shared.util.DownloadSystem
import com.xetondownloadmanager.integration.IntegrationHandler
import com.xetondownloadmanager.integration.HttpDownloadCredentialsFromIntegration
import com.xetondownloadmanager.integration.NewDownloadTask
import com.xetondownloadmanager.integration.ApiQueueModel
import com.xetondownloadmanager.integration.AddDownloadOptionsFromIntegration
import com.xetondownloadmanager.integration.HLSDownloadCredentialsFromIntegration
import com.xetondownloadmanager.integration.IDownloadCredentialsFromIntegration
import com.xetondownloadmanager.shared.downloaderinui.BasicDownloadItem
import com.xetondownloadmanager.shared.downloaderinui.DownloaderInUiRegistry
import com.xeton.downloader.downloaditem.hls.HLSDownloadCredentials
import com.xeton.downloader.NewDownloadItemProps
import com.xeton.downloader.downloaditem.EmptyContext
import com.xeton.downloader.downloaditem.http.HttpDownloadCredentials
import com.xeton.downloader.queue.QueueManager
import com.xeton.downloader.utils.OnDuplicateStrategy
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject

class IntegrationHandlerImp : IntegrationHandler, KoinComponent {
    val appComponent by inject<AppComponent>()
    val downloadSystem by inject<DownloadSystem>()
    val queueManager by inject<QueueManager>()
    val appSettings by inject<AppRepository>()
    private val downloaderInUiRegistry by inject<DownloaderInUiRegistry>()

    override suspend fun addDownload(
        list: List<IDownloadCredentialsFromIntegration>,
        options: AddDownloadOptionsFromIntegration,
    ) {
        appComponent.externalCredentialComingIntoApp(
            list.map {
                convertToDownloadSystemCredentials(it)
            },
            options = ImportOptions(
                silentImport = if (options.silentAdd) {
                    SilentImportOptions(
                        silentDownload = options.silentStart
                    )
                } else null
            )
        )
    }

    override fun listQueues(): List<ApiQueueModel> {
        return queueManager.getAll().map { downloadQueue ->
            val queueModel = downloadQueue.getQueueModel()
            ApiQueueModel(id = queueModel.id, name = queueModel.name)
        }
    }
    override suspend fun addDownloadTask(task: NewDownloadTask) {
        val addDownloaderInUiProps = convertToDownloadSystemCredentials(task.downloadSource)
        val downloaderInUi = downloaderInUiRegistry.getDownloaderOf(
            addDownloaderInUiProps.credentials
        ) ?: error("Downloader for ${addDownloaderInUiProps.credentials::class.qualifiedName} not found")
        val downloadItem = downloaderInUi.createBareDownloadItem(
            addDownloaderInUiProps.credentials,
            basicDownloadItem = BasicDownloadItem(
                folder = task.folder ?: appSettings.saveLocation.value,
                name = task.name ?: addDownloaderInUiProps.extraConfig.suggestedName
                ?: task.downloadSource.link.substringAfterLast("/"),
            ),
        )
        val id =
            downloadSystem.addDownload(
                newDownload = NewDownloadItemProps(
                    downloadItem = downloadItem,
                    onDuplicateStrategy = OnDuplicateStrategy.default(),
                    extraConfig = null,
                    context = EmptyContext,
                ),
                queueId = task.queueId,
                categoryId = null
            )
        if (task.queueId != null) {
            val queue = queueManager.getQueue(task.queueId!!)
            queue.start()
        } else {
            downloadSystem.userManualResume(id)
        }
    }

    companion object {
        private fun convertToDownloadSystemCredentials(it: IDownloadCredentialsFromIntegration): AddDownloadCredentialsInUiProps {
            val credentials = when (it) {
                is HttpDownloadCredentialsFromIntegration -> {
                    HttpDownloadCredentials(
                        link = it.link,
                        headers = it.headers,
                        downloadPage = it.downloadPage,
                    )
                }

                is HLSDownloadCredentialsFromIntegration -> {
                    HLSDownloadCredentials(
                        link = it.link,
                        headers = it.headers,
                        downloadPage = it.downloadPage,
                    )
                }
            }
            return AddDownloadCredentialsInUiProps(
                credentials = credentials,
                extraConfig = AddDownloadCredentialsInUiProps.Configs(
                    suggestedName = it.suggestedName,
                )
            )
        }
    }
}
