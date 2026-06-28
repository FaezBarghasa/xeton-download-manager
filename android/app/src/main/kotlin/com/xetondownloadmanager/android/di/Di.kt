package com.xetondownloadmanager.android.di

import AndroidDirectLinkUpdateApplier
import android.app.Application
import android.content.Context
import com.xetondownloadmanager.github.GithubApi
import com.xetondownloadmanager.UpdateDownloadLocationProvider
import com.xetondownloadmanager.UpdateManager
import com.xetondownloadmanager.android.ABDMApp
import com.xetondownloadmanager.android.pages.home.HomePageStateToPersist
import com.xetondownloadmanager.android.pages.onboarding.permissions.ABDMPermissions
import com.xetondownloadmanager.android.pages.onboarding.permissions.PermissionManager
import com.xetondownloadmanager.android.receiver.StartOnBootBroadcastReceiver
import com.xetondownloadmanager.android.repository.AppRepository
import com.xetondownloadmanager.android.storage.AndroidExtraDownloadItemSettings
import com.xetondownloadmanager.android.storage.AndroidExtraQueueSettings
import com.xetondownloadmanager.android.storage.AndroidOnBoardingStorage
import com.xetondownloadmanager.android.storage.AppSettingsStorage
import com.xetondownloadmanager.android.storage.BrowserBookmarksStorage
import com.xetondownloadmanager.android.storage.HomePageStorage
import com.xetondownloadmanager.android.storage.OnBoardingData
import com.xetondownloadmanager.android.util.ABDMAppManager
import com.xetondownloadmanager.android.util.ABDMServiceNotificationManager
import com.xetondownloadmanager.android.util.AndroidDefinedPaths
import com.xetondownloadmanager.android.util.AndroidDownloadItemOpener
import com.xetondownloadmanager.android.util.AppInfo
import com.xetondownloadmanager.shared.util.SharedConstants
import com.xetondownloadmanager.shared.ui.theme.ThemeManager
import com.xeton.downloader.queue.QueueManager
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.util.ui.theme.ISystemThemeDetector
import com.xeton.downloader.DownloadManagerMinimalControl
import com.xeton.downloader.DownloadSettings
import com.xeton.downloader.connection.HttpDownloaderClient
import com.xeton.downloader.connection.OkHttpHttpDownloaderClient
import com.xeton.downloader.db.*
import com.xeton.downloader.monitor.DownloadMonitor
import com.xeton.downloader.utils.IDiskStat
import com.xetondownloadmanager.resources.ABDMLanguageResources
import com.xetondownloadmanager.shared.downloaderinui.DownloaderInUiRegistry
import com.xetondownloadmanager.shared.downloaderinui.hls.HLSDownloaderInUi
import com.xetondownloadmanager.shared.downloaderinui.http.HttpDownloaderInUi
import com.xetondownloadmanager.shared.repository.BaseAppRepository
import com.xetondownloadmanager.shared.storage.BaseAppSettingsStorage
import com.xetondownloadmanager.shared.storage.ExtraDownloadSettingsStorage
import com.xetondownloadmanager.shared.storage.ExtraQueueSettingsStorage
import com.xetondownloadmanager.shared.storage.IExtraDownloadSettingsStorage
import com.xetondownloadmanager.shared.storage.IExtraQueueSettingsStorage
import com.xetondownloadmanager.shared.storage.ILastSavedLocationsStorage
import com.xetondownloadmanager.shared.storage.ISelectQueueStorage
import com.xetondownloadmanager.shared.storage.PerHostSettingsDatastoreStorage
import com.xetondownloadmanager.shared.storage.ProxyDatastoreStorage
import com.xetondownloadmanager.shared.storage.SelectQueueSettings
import com.xetondownloadmanager.shared.storage.impl.LastSavedLocationStorage
import com.xetondownloadmanager.shared.storage.impl.SelectQueueStorage
import com.xetondownloadmanager.shared.ui.theme.ThemeSettingsStorage
import com.xetondownloadmanager.shared.ui.widget.NotificationManager
import com.xetondownloadmanager.shared.updater.UpdateDownloaderViaDownloadSystem
import com.xetondownloadmanager.shared.util.AndroidDiskStat
import com.xetondownloadmanager.shared.util.AndroidSystemThemeDetector
import com.xetondownloadmanager.shared.util.AppVersion
import com.xetondownloadmanager.shared.util.DefinedPaths
import com.xetondownloadmanager.shared.util.SizeAndSpeedUnitProvider
import com.xetondownloadmanager.shared.util.UserAgentProviderFromSettings
import com.xetondownloadmanager.shared.util.*
import com.xetondownloadmanager.updateapplier.UpdateApplier
import com.xeton.downloader.DownloadManager
import com.xeton.util.config.datastore.createMapConfigDatastore
import kotlinx.coroutines.*
import kotlinx.serialization.json.Json
import okhttp3.Dispatcher
import okhttp3.OkHttpClient
import org.koin.core.component.KoinComponent
import org.koin.core.context.startKoin
import org.koin.dsl.bind
import org.koin.dsl.module
import com.xetondownloadmanager.updatechecker.GithubUpdateChecker
import com.xetondownloadmanager.updatechecker.UpdateChecker
import com.xeton.util.AppVersionTracker
import com.xetondownloadmanager.shared.util.appinfo.PreviousVersion
import com.xetondownloadmanager.shared.util.autoremove.RemovedDownloadsFromDiskTracker
import com.xetondownloadmanager.shared.util.category.*
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorMapperRegistryFactory
import com.xetondownloadmanager.shared.util.downloaderror.faileddownloads.FailedDownloadErrorStorageInMemory
import com.xetondownloadmanager.shared.util.downloaderror.faileddownloads.FailedDownloads
import com.xetondownloadmanager.shared.util.downloaderror.faileddownloads.IFailedDownloadErrorStorage
import com.xetondownloadmanager.shared.util.keepawake.KeepAwakeManager
import com.xetondownloadmanager.shared.util.keepawake.platformKeepAwake
import com.xetondownloadmanager.shared.util.notification.INotificationSettingsStorage
import com.xetondownloadmanager.shared.util.ondownloadcompletion.NoOpOnDownloadCompletionActionProvider
import com.xetondownloadmanager.shared.util.ondownloadcompletion.OnDownloadCompletionActionProvider
import com.xetondownloadmanager.shared.util.ondownloadcompletion.OnDownloadCompletionActionRunner
import com.xetondownloadmanager.shared.util.onqueuecompletion.NoopOnQueueCompletionActionProvider
import com.xetondownloadmanager.shared.util.onqueuecompletion.OnQueueEventActionRunner
import com.xetondownloadmanager.shared.util.onqueuecompletion.OnQueueCompletionActionProvider
import com.xetondownloadmanager.shared.util.perhostsettings.IPerHostSettingsStorage
import com.xetondownloadmanager.shared.util.perhostsettings.PerHostSettingsItem
import com.xetondownloadmanager.shared.util.perhostsettings.PerHostSettingsManager
import com.xetondownloadmanager.shared.util.ui.IMyIcons
import com.xetondownloadmanager.shared.util.proxy.IProxyStorage
import com.xetondownloadmanager.shared.util.proxy.ProxyData
import com.xetondownloadmanager.shared.util.proxy.ProxyManager
import com.xeton.downloader.DownloaderRegistry
import com.xeton.downloader.connection.UserAgentProvider
import com.xeton.downloader.connection.proxy.AutoConfigurableProxyProvider
import com.xeton.downloader.connection.proxy.NoopSystemProxySelectorProvider
import com.xeton.downloader.connection.proxy.ProxyStrategyProvider
import com.xeton.downloader.connection.proxy.SystemProxySelectorProvider
import com.xeton.downloader.downloaditem.DownloadJob
import com.xeton.downloader.downloaditem.IDownloadCredentials
import com.xeton.downloader.downloaditem.IDownloadItem
import com.xeton.downloader.downloaditem.hls.HLSDownloader
import com.xeton.downloader.downloaditem.http.HttpDownloadCredentials
import com.xeton.downloader.downloaditem.http.HttpDownloadItem
import com.xeton.downloader.downloaditem.http.HttpDownloader
import com.xeton.downloader.monitor.DownloadItemStateFactory
import com.xeton.downloader.monitor.IDownloadMonitor
import com.xeton.downloader.queue.ManualDownloadQueue
import com.xeton.downloader.utils.EmptyFileCreator
import com.xeton.util.compose.IIconResolver
import com.xeton.util.compose.localizationmanager.LanguageManager
import com.xeton.util.compose.localizationmanager.LanguageSourceProvider
import com.xeton.util.compose.localizationmanager.LanguageStorage
import com.xeton.util.config.datastore.kotlinxSerializationDataStore
import com.xeton.util.startup.AbstractStartupManager
import com.xeton.util.startup.Startup
import kotlinx.serialization.modules.SerializersModule
import kotlinx.serialization.modules.polymorphic
import okhttp3.Protocol
import okhttp3.internal.tls.OkHostnameVerifier

val downloaderModule = module {
    single<IDownloadQueueDatabase> {
        val definedPaths = get<DefinedPaths>()

        DownloadQueueFileStorageDatabase(
            queueFolder = get<DownloadFoldersRegistry>().registerAndGet(
                definedPaths.queuesDir
            ),
            fileSaver = get(),
        )
    }
    single<IDownloadListDb> {
        val definedPaths = get<DefinedPaths>()
        DownloadListFileStorage(
            downloadListFolder = get<DownloadFoldersRegistry>().registerAndGet(
                definedPaths.downloadListDir
            ),
            fileSaver = get(),
        )
    }
    single {
        TransactionalFileSaver(get())
    }
    single<IDownloadPartListDb> {
        val definedPaths = get<DefinedPaths>()
        PartListFileStorage(
            get<DownloadFoldersRegistry>().registerAndGet(
                definedPaths.partsDir
            ),
            get()
        )
    }
    single<IDiskStat> {
        AndroidDiskStat()
    }
    single<ISystemThemeDetector> {
        AndroidSystemThemeDetector(get())
    }
    single {
        QueueManager(get(), get())
    }
    single {
        DownloadFoldersRegistry()
    }
    single {
        DownloadSettings(
            8,
        )
    }
    single {
        ProxyManager(
            get()
        )
    }.bind<ProxyStrategyProvider>()
    single<SystemProxySelectorProvider> {
        NoopSystemProxySelectorProvider()
    }
    single<AutoConfigurableProxyProvider> {
        AutoConfigurableProxyProvider.NoOp()
    }
    single<UserAgentProvider> {
        UserAgentProviderFromSettings(get())
    }
    single<HttpDownloaderClient> {
        OkHttpHttpDownloaderClient(
            get(),
            get(),
            get(),
            get(),
            get(),
        )
    }
    single {
        val downloadSettings: DownloadSettings = get()
        EmptyFileCreator(
            diskStat = get(),
            useSparseFile = { downloadSettings.useSparseFileAllocation }
        )
    }
    single {
        HLSDownloader(inject())
    }
    single {
        HLSDownloaderInUi(get(), get())
    }
    single {
        HttpDownloader(inject())
    }
    single {
        HttpDownloaderInUi(get(), get())
    }
    single {
        DownloaderInUiRegistry().apply {
            add(get<HttpDownloaderInUi>())
            add(get<HLSDownloaderInUi>())
        }
    }.bind<DownloadItemStateFactory<IDownloadItem, DownloadJob>>()
    single {
        DownloaderRegistry().apply {
            add(get<HttpDownloader>())
            add(get<HLSDownloader>())
        }
    }
    single {
        val definedPaths = get<DefinedPaths>()
        DownloadManager(
            get(),
            get(),
            get(),
            get(),
            get(),
            get<DownloadFoldersRegistry>().registerAndGet(
                definedPaths.downloadDataDir
            )
        )
    }.bind(DownloadManagerMinimalControl::class)
    single {
        ManualDownloadQueue(get(), get())
    }
    single<IDownloadMonitor> {
        DownloadMonitor(
            downloadManager = get(),
            manualDownloadQueue = get(),
            downloadItemStateFactory = inject(),
        )
    }
}
val downloadSystemModule = module {
    single {
        val definedPaths = get<DefinedPaths>()
        get<DownloadFoldersRegistry>().registerAndGet(definedPaths.categoriesDir)
        CategoryFileStorage(
            file = definedPaths.categoriesFile.toFile(),
            fileSaver = get()
        )
    }.bind<CategoryStorage>()
    single {
        FileIconProviderUsingCategoryIcons(
            get(),
            get(),
            get(),
            get(),
        )
    }.bind<FileIconProvider>()
    single {
        DefaultCategories(
            icons = get(),
            getDefaultDownloadFolder = {
                get<BaseAppSettingsStorage>().defaultDownloadFolder.value
            }
        )
    }
    single {
        DownloadManagerCategoryItemProvider(get())
    }.bind<ICategoryItemProvider>()
    single {
        CategoryManager(
            categoryStorage = get(),
            scope = get(),
            defaultCategoriesFactory = get(),
            categoryItemProvider = get(),
        )
    }

    single {
        DownloadSystem(
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
        )
    }
    single {
        val definedPaths = get<DefinedPaths>()
        val extraDownloadSettingsStorageFolder = get<DownloadFoldersRegistry>().registerAndGet(
            definedPaths.extraDownloadSettings
        )
        ExtraDownloadSettingsStorage(
            extraDownloadSettingsStorageFolder,
            get(),
            AndroidExtraDownloadItemSettings
        )
    }.bind<IExtraDownloadSettingsStorage<*>>()
    single {
        val definedPaths = get<DefinedPaths>()
        val extraQueueSettingsStorageFolder = get<DownloadFoldersRegistry>().registerAndGet(
            definedPaths.extraQueueSettings
        )
        ExtraQueueSettingsStorage(
            extraQueueSettingsStorageFolder,
            get(),
            AndroidExtraQueueSettings
        )
    }.apply {
        bind<IExtraQueueSettingsStorage<*>>()
    }
    single<OnDownloadCompletionActionProvider> {
        NoOpOnDownloadCompletionActionProvider()
    }
    single<OnQueueCompletionActionProvider> {
        NoopOnQueueCompletionActionProvider()
    }
    single {
        OnDownloadCompletionActionRunner(
            downloadManagerMinimalControl = get(),
            scope = get(),
            onDownloadCompletionActionProvider = get(),
        )
    }
    single {
        OnQueueEventActionRunner(
            queueManager = get(),
            scope = get(),
            onQueueCompletionActionProvider = get(),
        )
    }
    single {
        PermissionManager(
            ABDMPermissions.importantPermissions,
            get(),
        )
    }
}
val coroutineModule = module {
    single {
        CoroutineScope(SupervisorJob())
    }
}
val jsonModule = module {
    single {
        val downloaderRegistry: DownloaderRegistry by inject()
        Json {
            this.encodeDefaults = true
            this.prettyPrint = true
            this.ignoreUnknownKeys = true
            this.serializersModule = SerializersModule {
                polymorphic(IDownloadItem::class) {
                    downloaderRegistry.getAll().forEach {
                        subclass(it.downloadItemClass, it.downloadItemSerializer)
                    }
                    defaultDeserializer {
                        HttpDownloadItem.serializer()
                    }
                }
                polymorphic(IDownloadCredentials::class) {
                    downloaderRegistry.getAll().forEach {
                        subclass(it.downloadCredentialsClass, it.downloadCredentialsSerializer)
                    }
                    defaultDeserializer {
                        HttpDownloadCredentials.serializer()
                    }
                }
            }
        }
    }
}
val updaterModule = module {
    single {
        val definedPaths = get<DefinedPaths>()
        UpdateDownloadLocationProvider {
            definedPaths.updateDownloadLocation.toFile()
        }
    }
    single<UpdateApplier> {
        val definedPaths = get<DefinedPaths>()
        definedPaths.updateDownloadLocation
        AndroidDirectLinkUpdateApplier(
            updateDownloader = UpdateDownloaderViaDownloadSystem(
                get(),
                get(),
            ),
        )
    }
    single<UpdateChecker> {
        GithubUpdateChecker(
            AppVersion.get(),
            githubApi = GithubApi(
                owner = SharedConstants.projectGithubOwner,
                repo = SharedConstants.projectGithubRepo,
                client = OkHttpClient
                    .Builder()
                    .build()
            )
        )
    }
    single {
        UpdateManager(
            updateChecker = get(),
            updateApplier = get(),
            appVersionTracker = get(),
        )
    }
}
val startUpModule = module {
    single {
        Startup.getStartUpManager(get(), StartOnBootBroadcastReceiver::class.java)
    }.apply {
        bind<AbstractStartupManager>()
    }
}

fun getAppModule(context: ABDMApp) = module {
    includes(downloaderModule)
    includes(downloadSystemModule)
    includes(coroutineModule)
    includes(jsonModule)
    includes(updaterModule)
    includes(startUpModule)
//    single {
//        NetworkChecker(get())
//    }
    single {
        AppInfo.definedPaths
    }.apply {
        bind<DefinedPaths>()
        bind<AndroidDefinedPaths>()
    }
    single {
        AppRepository(
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
            get(),
        )
    }.apply {
        bind<BaseAppRepository>()
        bind<SizeAndSpeedUnitProvider>()
    }
    single {
        ThemeManager(get(), get(), get())
    }
//    single {
//        FontManager(get())
//    }
    single {
        LanguageManager(
            get(),
            LanguageSourceProvider(
                ABDMLanguageResources.defaultLanguageResource,
                ABDMLanguageResources.languages,
            )
        )
    }
    single {
        MyIcons
    }.apply {
        bind<IMyIcons>()
        bind<IIconResolver>()
    }
    single {
        val definedPaths = get<DefinedPaths>()
        ProxyDatastoreStorage(
            kotlinxSerializationDataStore(
                definedPaths.proxySettingsFile.toFile(),
                get(),
                ProxyData::default,
            )
        )
    }.bind<IProxyStorage>()
    single {
        val definedPaths = get<DefinedPaths>()
        AppSettingsStorage(
            createMapConfigDatastore(
                definedPaths.appSettingsFile.toFile(),
                get(),
            )
        )
    }.apply {
        bind<BaseAppSettingsStorage>()
        bind<LanguageStorage>()
        bind<ThemeSettingsStorage>()
        bind<INotificationSettingsStorage>()
    }
    single {
        RemovedDownloadsFromDiskTracker(
            get(), get(), get(),
        )
    }
    single {
        val definedPaths = get<DefinedPaths>()
        PreviousVersion(
            systemPath = definedPaths.systemDir.toFile(),
            currentVersion = AppVersion.get(),
        )
    }
    single {
        AppVersionTracker(
            previousVersion = {
                // it MUST be booted first
                get<PreviousVersion>().get()
            },
            currentVersion = AppVersion.get(),
        )
    }

    single {
        val appSettingsStorage: BaseAppSettingsStorage = get()
        AppSSLFactoryProvider(
            ignoreSSLCertificates = appSettingsStorage.ignoreSSLCertificates
        )
    }
    single {
        val appSettingsStorage: BaseAppSettingsStorage = get()
        AppHostNameVerifier(
            delegateHostnameVerifier = OkHostnameVerifier,
            ignoreHostNameVerification = appSettingsStorage.ignoreSSLCertificates
        )
    }
    single<OkHttpClient> {
        val appSSLFactoryProvider: AppSSLFactoryProvider = get()
        val appHostNameVerifier: AppHostNameVerifier = get()
        OkHttpClient
            .Builder()
            .protocols(listOf(Protocol.HTTP_1_1))
            .dispatcher(Dispatcher().apply {
                //bypass limit on concurrent connections!
                maxRequests = Int.MAX_VALUE
                maxRequestsPerHost = Int.MAX_VALUE
            })
            .sslSocketFactory(
                appSSLFactoryProvider.createSSLSocketFactory(),
                appSSLFactoryProvider.trustManager,
            )
            .hostnameVerifier(appHostNameVerifier)
            .build()
    }
    single<ILastSavedLocationsStorage> {
        val definedPaths = get<AndroidDefinedPaths>()
        LastSavedLocationStorage(
            kotlinxSerializationDataStore<List<String>>(
                definedPaths.lastSavedLocationFile.toFile(),
                get(),
                ::emptyList,
            )
        )
    }
    single<ISelectQueueStorage> {
        val definedPaths = get<AndroidDefinedPaths>()
        SelectQueueStorage(
            kotlinxSerializationDataStore<SelectQueueSettings>(
                definedPaths.selectQueueSettingsFile.toFile(),
                get(),
                ::SelectQueueSettings,
            )
        )
    }
    single {
        KeepAwakeManager(
            platformKeepAwake(),
            get(),
            get(),
        )
    }
    single<IPerHostSettingsStorage> {
        val definedPaths = get<DefinedPaths>()
        PerHostSettingsDatastoreStorage(
            kotlinxSerializationDataStore<List<PerHostSettingsItem>>(
                definedPaths.perHostSettingsFile.toFile(),
                get(),
                ::emptyList,
            )
        )
    }
    single {
        PerHostSettingsManager(get())
    }
    single {
        DownloadErrorMapperRegistryFactory().createRegistry()
    }
    single<IFailedDownloadErrorStorage> {
        FailedDownloadErrorStorageInMemory()
    }
    single {
        FailedDownloads(
            get(),
            get(),
            get(),
            get(),
        )
    }
    single { context }.apply {
        bind<ABDMApp>()
        bind<Application>()
        bind<Context>()
    }
    single {
        ABDMAppManager(get(), get(), get(), get(), get(), get(), get(), get())
    }
    single {
        ABDMServiceNotificationManager(get(), get(), get(), get(), get(), get())
    }
    single {
        AndroidDownloadItemOpener(get())
    }.apply {
        bind<DownloadItemOpener>()
    }
    single { NotificationManager() }
    single {
        val paths = get<AndroidDefinedPaths>()
        AndroidOnBoardingStorage(
            kotlinxSerializationDataStore(
                paths.onboardingFile.toFile(),
                get(),
                ::OnBoardingData,
            )
        )
    }
    single {
        val paths = get<AndroidDefinedPaths>()
        HomePageStorage(
            kotlinxSerializationDataStore(
                paths.homePageFile.toFile(),
                get(),
                ::HomePageStateToPersist,
            )
        )
    }
    single {
        val paths = get<AndroidDefinedPaths>()
        BrowserBookmarksStorage(
            kotlinxSerializationDataStore(
                paths.browserBookmarksFile.toFile(),
                get(),
                ::emptyList,
            )
        )
    }
}


object Di : KoinComponent {
    fun boot(applicationContext: ABDMApp) {
        startKoin {
            modules(getAppModule(applicationContext))
        }
    }
}
