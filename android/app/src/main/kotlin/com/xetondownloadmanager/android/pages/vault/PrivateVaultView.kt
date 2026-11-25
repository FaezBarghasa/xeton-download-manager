package com.xetondownloadmanager.android.pages.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.xetondownloadmanager.shared.ui.widget.Text
import com.xetondownloadmanager.shared.util.ui.myColors

data class VaultFileItem(
    val name: String,
    val sizeBytes: Long,
    val encryptedPath: String,
)

/**
 * UI for the XChaCha20-Poly1305 Encrypted Download Vault.
 */
@Composable
fun PrivateVaultView(
    vaultFiles: List<VaultFileItem>,
    isUnlocked: Boolean,
    onUnlockRequested: () -> Unit,
    onFileSelected: (VaultFileItem) -> Unit,
    modifier: Modifier = Modifier,
) {
    Box(
        modifier = modifier
            .fillMaxSize()
            .background(myColors.background)
            .padding(16.dp)
    ) {
        if (!isUnlocked) {
            Column(
                modifier = Modifier.align(Alignment.Center),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center,
            ) {
                Text(
                    text = "Encrypted Private Vault",
                    fontSize = 20.sp,
                    fontWeight = FontWeight.Bold,
                    color = myColors.onBackground,
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Protected by XChaCha20-Poly1305 authenticated encryption.",
                    fontSize = 14.sp,
                    color = myColors.onBackground.copy(alpha = 0.7f),
                )
                Spacer(modifier = Modifier.height(24.dp))
                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(24.dp))
                        .background(myColors.primary)
                        .clickable { onUnlockRequested() }
                        .padding(horizontal = 24.dp, vertical = 12.dp)
                ) {
                    Text(
                        text = "Unlock with Biometrics",
                        color = myColors.onPrimary,
                        fontWeight = FontWeight.SemiBold,
                    )
                }
            }
        } else {
            Column(modifier = Modifier.fillMaxSize()) {
                Text(
                    text = "Secure Vault Files",
                    fontSize = 20.sp,
                    fontWeight = FontWeight.Bold,
                    color = myColors.onBackground,
                )
                Spacer(modifier = Modifier.height(16.dp))
                if (vaultFiles.isEmpty()) {
                    Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                        Text(
                            text = "No encrypted downloads stored yet.",
                            color = myColors.onBackground.copy(alpha = 0.6f),
                        )
                    }
                } else {
                    LazyColumn(
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                        modifier = Modifier.fillMaxSize()
                    ) {
                        items(vaultFiles) { file ->
                            Box(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .clip(RoundedCornerShape(12.dp))
                                    .background(myColors.surface)
                                    .clickable { onFileSelected(file) }
                                    .padding(16.dp)
                            ) {
                                Row(
                                    modifier = Modifier.fillMaxWidth(),
                                    verticalAlignment = Alignment.CenterVertically,
                                    horizontalArrangement = Arrangement.SpaceBetween,
                                ) {
                                    Column {
                                        Text(
                                            text = file.name,
                                            fontWeight = FontWeight.SemiBold,
                                            fontSize = 14.sp,
                                            color = myColors.onSurface,
                                        )
                                        Text(
                                            text = "${file.sizeBytes / 1024} KB (Encrypted)",
                                            fontSize = 12.sp,
                                            color = myColors.onSurface.copy(alpha = 0.7f),
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
