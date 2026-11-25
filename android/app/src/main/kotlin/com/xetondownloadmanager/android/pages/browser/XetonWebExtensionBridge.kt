package com.xetondownloadmanager.android.pages.browser

import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import org.json.JSONObject
import org.mozilla.geckoview.GeckoRuntime
import org.mozilla.geckoview.WebExtension
import java.util.Collections
import java.util.LinkedHashMap

data class SniffedMedia(
    val url: String,
    val pageUrl: String?,
    val mimeType: String?,
    val title: String?,
    val headers: Map<String, String>,
)

object XetonWebExtensionBridge {
    // Thread-safe bounded LRU cache for headers (max 256 entries to prevent memory leaks)
    private val headersByUrl: MutableMap<String, Map<String, String>> = Collections.synchronizedMap(
        object : LinkedHashMap<String, Map<String, String>>(256, 0.75f, true) {
            override fun removeEldestEntry(eldest: MutableMap.MutableEntry<String, Map<String, String>>?): Boolean {
                return size > 256
            }
        }
    )
    private val _mediaSuggestions = MutableSharedFlow<SniffedMedia>(extraBufferCapacity = 64)
    val mediaSuggestions: SharedFlow<SniffedMedia> = _mediaSuggestions.asSharedFlow()

    fun getHeadersFor(url: String): Map<String, String> {
        return headersByUrl[url] ?: emptyMap()
    }

    fun registerNativePort(runtime: GeckoRuntime, extension: WebExtension) {
        extension.setMessageDelegate(
            object : WebExtension.MessageDelegate {
                override fun onConnect(port: WebExtension.Port) {
                    port.setDelegate(object : WebExtension.PortDelegate {
                        override fun onPortMessage(message: Any, port: WebExtension.Port) {
                            if (message !is JSONObject) return

                            when (message.optString("type")) {
                                "REQUEST_HEADERS" -> {
                                    val url = message.optString("url")
                                    val headersObj = message.optJSONObject("headers")
                                    if (url.isNotEmpty() && headersObj != null) {
                                        val map = mutableMapOf<String, String>()
                                        val keys = headersObj.keys()
                                        while (keys.hasNext()) {
                                            val key = keys.next()
                                            map[key] = headersObj.optString(key)
                                        }
                                        headersByUrl[url] = map
                                    }
                                }
                                "MEDIA_SNIFFED" -> {
                                    val url = message.optString("url")
                                    if (url.isNotEmpty()) {
                                        val pageUrl = message.optString("pageUrl").takeIf { it.isNotEmpty() }
                                        val mimeType = message.optString("mimeType").takeIf { it.isNotEmpty() }
                                        val title = message.optString("title").takeIf { it.isNotEmpty() }
                                        val headers = headersByUrl[url] ?: emptyMap()
                                        _mediaSuggestions.tryEmit(
                                            SniffedMedia(
                                                url = url,
                                                pageUrl = pageUrl,
                                                mimeType = mimeType,
                                                title = title,
                                                headers = headers,
                                            )
                                        )
                                    }
                                }
                            }
                        }

                        override fun onDisconnect(port: WebExtension.Port) {}
                    })
                }
            },
            "xeton_bridge"
        )
    }
}
