package com.xetondownloadmanager.android.pages.vault

import android.app.KeyguardManager
import android.content.Context
import android.content.Intent

object BiometricVaultAuth {
    /**
     * Checks if device lock (biometric, PIN, pattern, password) is secured.
     */
    fun isDeviceSecure(context: Context): Boolean {
        val keyguardManager = context.getSystemService(Context.KEYGUARD_SERVICE) as? KeyguardManager
        return keyguardManager?.isDeviceSecure == true
    }

    /**
     * Creates an intent for device credential verification (PIN/Pattern/Biometrics).
     */
    fun createConfirmDeviceCredentialIntent(context: Context): Intent? {
        val keyguardManager = context.getSystemService(Context.KEYGUARD_SERVICE) as? KeyguardManager
        return keyguardManager?.createConfirmDeviceCredentialIntent(
            "Unlock Private Vault",
            "Authenticate to access your encrypted downloads"
        )
    }
}
