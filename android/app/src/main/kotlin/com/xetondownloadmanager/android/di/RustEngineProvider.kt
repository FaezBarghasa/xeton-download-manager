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
        return XetonEngine(dataDir, settings)
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
