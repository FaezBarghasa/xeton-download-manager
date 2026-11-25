package com.xetondownloadmanager.android.pages.vault

import android.app.KeyguardManager
import android.content.Context
import android.content.Intent
import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

object BiometricVaultAuth {
    private const val ANDROID_KEYSTORE = "AndroidKeyStore"
    private const val VAULT_MASTER_KEY_ALIAS = "xeton_vault_master_key"

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

    /**
     * Retrieves or generates a hardware-backed AES-256 master key in Android KeyStore.
     */
    fun getOrCreateHardwareMasterKey(): SecretKey {
        val keyStore = KeyStore.getInstance(ANDROID_KEYSTORE).apply { load(null) }
        if (keyStore.containsAlias(VAULT_MASTER_KEY_ALIAS)) {
            val entry = keyStore.getEntry(VAULT_MASTER_KEY_ALIAS, null) as? KeyStore.SecretKeyEntry
            if (entry != null) return entry.secretKey
        }

        val keyGenerator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, ANDROID_KEYSTORE)
        val builder = KeyGenParameterSpec.Builder(
            VAULT_MASTER_KEY_ALIAS,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setKeySize(256)
            .setUserAuthenticationRequired(true)

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            builder.setUserAuthenticationParameters(
                0, // 0 = requires auth for every crypto operation
                KeyProperties.AUTH_BIOMETRIC_STRONG or KeyProperties.AUTH_DEVICE_CREDENTIAL
            )
        }

        keyGenerator.init(builder.build())
        return keyGenerator.generateKey()
    }

    /**
     * Initializes a cipher instance for hardware-authenticated vault wrapping.
     */
    fun createVaultCipher(mode: Int, iv: ByteArray? = null): Cipher {
        val key = getOrCreateHardwareMasterKey()
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        if (iv != null) {
            val spec = GCMParameterSpec(128, iv)
            cipher.init(mode, key, spec)
        } else {
            cipher.init(mode, key)
        }
        return cipher
    }
}
