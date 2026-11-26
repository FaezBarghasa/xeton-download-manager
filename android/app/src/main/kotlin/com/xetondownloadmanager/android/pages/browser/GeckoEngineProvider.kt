package com.xetondownloadmanager.android.pages.browser

import android.content.Context
import org.mozilla.geckoview.ContentBlocking
import org.mozilla.geckoview.GeckoRuntime
import org.mozilla.geckoview.GeckoRuntimeSettings
import org.mozilla.geckoview.WebExtension

/**
 * Hardened GeckoEngineProvider with Strict Tracking Protection, HTTPS-Only mode,
 * and built-in Xeton WebExtension installation.
 */
object GeckoEngineProvider {

    @Volatile
    private var _runtime: GeckoRuntime? = null

    @Volatile
    var xetonExtension: WebExtension? = null
        private set

    fun getOrCreate(context: Context): GeckoRuntime {
        return _runtime ?: synchronized(this) {
            _runtime ?: buildRuntime(context.applicationContext).also {
                _runtime = it
                installBuiltInExtension(it)
            }
        }
    }

    val isInitialized: Boolean
        get() = _runtime != null

    private fun buildRuntime(appContext: Context): GeckoRuntime {
        val settings = GeckoRuntimeSettings.Builder()
            .remoteDebuggingEnabled(false)
            .consoleOutput(false)
            .doubleTapZoomingEnabled(false)
            .inputAutoZoomEnabled(false)
            // Privacy Hardening: enforce HTTPS-Only and strict tracking protection
            .allowInsecureConnections(GeckoRuntimeSettings.HTTPS_ONLY)
            .contentBlocking(
                ContentBlocking.Settings.Builder()
                    .enhancedTrackingProtectionLevel(ContentBlocking.EtpLevel.STRICT)
                    .antiTracking(ContentBlocking.AntiTracking.STRICT)
                    .cookieBehavior(ContentBlocking.CookieBehavior.ACCEPT_FIRST_PARTY_AND_ISOLATE_OTHERS)
                    .cookiePurging(true)
                    .strictSocialTrackingProtection(true)
                    .queryParameterStrippingEnabled(true)
                    .build()
            )
            .build()

        return GeckoRuntime.create(appContext, settings)
    }

    private fun installBuiltInExtension(runtime: GeckoRuntime) {
        runtime.webExtensionController
            .installBuiltIn("resource://android/assets/extensions/xeton/")
            .accept(
                { ext ->
                    xetonExtension = ext
                    ext?.let { XetonWebExtensionBridge.registerNativePort(runtime, it) }
                },
                { err ->
                    // Fallback to file asset URI if resource scheme differs across builds
                    runtime.webExtensionController
                        .installBuiltIn("assets/extensions/xeton/")
                        .accept({ xetonExtension = it }, {})
                }
            )
    }
}
