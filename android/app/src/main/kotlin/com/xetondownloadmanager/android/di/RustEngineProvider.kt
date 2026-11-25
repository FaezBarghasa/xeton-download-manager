package com.xetondownloadmanager.android.di

import android.content.Context
import com.xeton.downloader.DownloadSettings
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import uniffi.xeton_core.NewDownloadProps
import uniffi.xeton_core.XetonEngine
import uniffi.xeton_core.DownloadSettings as RustDownloadSettings
import uniffi.xeton_core.DownloadProtocol as RustDownloadProtocol
import uniffi.xeton_core.DuplicateStrategy as RustDuplicateStrategy
import java.io.File

/**
 * Singleton holder for the Rust XetonEngine instance inside the Android application.
 */
class RustEngineProvider(
    private val context: Context,
    private val scope: CoroutineScope = CoroutineScope(SupervisorJob() + Dispatchers.IO),
) {
    @Volatile
    private var engine: XetonEngine? = null

    init {
        // Load the compiled NDK native library
        try {
            System.loadLibrary("xeton_core")
        } catch (e: UnsatisfiedLinkError) {
            e.printStackTrace()
        }
    }

    fun getEngine(): XetonEngine {
        return engine ?: synchronized(this) {
            engine ?: buildEngine().also {
                engine = it
                scope.launch {
                    try {
                        it.boot()
                    } catch (e: Exception) {
                        e.printStackTrace()
                    }
                }
            }
        }
    }

    private fun buildEngine(): XetonEngine {
        val dataDir = File(context.filesDir, "xeton_surreal_kv").apply { mkdirs() }.absolutePath
        val settings = RustDownloadSettings(
            defaultThreadCount = 8u,
            dynamicPartCreation = true,
            useServerLastModified = true,
            globalSpeedLimit = 0L,
            minPartSize = 1024L * 1024L,
            maxRetryCount = 5u,
            appendExtensionToIncomplete = true,
        )
        return XetonEngine(dataDir, settings).also { engine ->
            scope.launch {
                migrateLegacyJsonIfNeeded(engine)
            }
        }
    }

    private suspend fun migrateLegacyJsonIfNeeded(engine: XetonEngine) {
        val legacyDir = File(context.filesDir, "download_list")
        if (!legacyDir.exists() || !legacyDir.isDirectory) return

        val jsonFiles = legacyDir.listFiles { f -> f.extension == "json" } ?: return
        if (jsonFiles.isEmpty()) return

        for (file in jsonFiles) {
            try {
                val content = file.readText()
                // If it contains valid link, import into Rust engine
                val linkRegex = Regex("\"link\":\\s*\"(.*?)\"")
                val nameRegex = Regex("\"name\":\\s*\"(.*?)\"")
                val linkMatch = linkRegex.find(content)?.groupValues?.get(1)
                val nameMatch = nameRegex.find(content)?.groupValues?.get(1) ?: file.nameWithoutExtension

                if (!linkMatch.isNullOrBlank()) {
                    engine.addDownload(
                        NewDownloadProps(
                            name = nameMatch,
                            folder = context.filesDir.resolve("downloads").absolutePath,
                            link = linkMatch,
                            protocol = RustDownloadProtocol.HTTP,
                            preferredConnections = 8u,
                            speedLimit = 0L,
                            onDuplicate = RustDuplicateStrategy.ADD_NUMBERED,
                        )
                    )
                }
            } catch (e: Exception) {
                // Ignore migration errors on legacy broken files
            }
        }
    }

    fun addDownload(
        name: String,
        folder: String,
        link: String,
        speedLimit: Long = 0L,
    ) {
        scope.launch {
            try {
                getEngine().addDownload(
                    NewDownloadProps(
                        name = name,
                        folder = folder,
                        link = link,
                        protocol = RustDownloadProtocol.HTTP,
                        preferredConnections = 8u,
                        speedLimit = speedLimit,
                        onDuplicate = RustDuplicateStrategy.ADD_NUMBERED,
                    )
                )
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    fun resume(id: Long) {
        scope.launch {
            try {
                getEngine().resume(id)
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    fun pause(id: Long) {
        scope.launch {
            try {
                getEngine().pause(id)
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    fun deleteDownload(id: Long, removeFile: Boolean) {
        scope.launch {
            try {
                getEngine().deleteDownload(id, removeFile)
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    suspend fun getDownloadList(): List<uniffi.xeton_core.DownloadItem> {
        return try {
            getEngine().getDownloadList()
        } catch (e: Exception) {
            emptyList()
        }
    }

    fun setGlobalSpeedLimit(bytesPerSecond: Long) {
        scope.launch {
            try {
                getEngine().setGlobalSpeedLimit(bytesPerSecond)
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }
}
