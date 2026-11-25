package com.xetondownloadmanager.android.pages.browser

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.xetondownloadmanager.shared.ui.widget.Text
import com.xetondownloadmanager.shared.util.ui.myColors
import kotlinx.coroutines.delay

/**
 * Floating media download pill displayed when GeckoView / WebExtension sniffs
 * downloadable media (video stream, audio track, direct file link).
 */
@Composable
fun DownloadSuggestionPill(
    onDownloadRequested: (SniffedMedia) -> Unit,
    modifier: Modifier = Modifier,
) {
    var currentMedia by remember { mutableStateOf<SniffedMedia?>(null) }
    var visible by remember { mutableStateOf(false) }

    LaunchedEffect(Unit) {
        XetonWebExtensionBridge.mediaSuggestions.collect { sniffed ->
            currentMedia = sniffed
            visible = true
            // Auto dismiss after 12 seconds if not interacted with
            delay(12000L)
            visible = false
        }
    }

    AnimatedVisibility(
        visible = visible && currentMedia != null,
        enter = slideInVertically(initialOffsetY = { it }) + fadeIn(),
        exit = slideOutVertically(targetOffsetY = { it }) + fadeOut(),
        modifier = modifier
    ) {
        val media = currentMedia ?: return@AnimatedVisibility

        Box(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 8.dp)
                .clip(RoundedCornerShape(28.dp))
                .background(myColors.surface)
                .clickable {
                    visible = false
                    onDownloadRequested(media)
                }
                .padding(horizontal = 16.dp, vertical = 12.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Column(
                    modifier = Modifier.weight(1f)
                ) {
                    Text(
                        text = "Download Detected Media",
                        fontWeight = FontWeight.Bold,
                        fontSize = 14.sp,
                        color = myColors.onSurface,
                    )
                    Text(
                        text = media.title ?: media.url,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        fontSize = 12.sp,
                        color = myColors.onSurface.copy(alpha = 0.8f),
                    )
                }

                Spacer(modifier = Modifier.width(8.dp))

                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(20.dp))
                        .background(myColors.primary)
                        .padding(horizontal = 12.dp, vertical = 6.dp)
                ) {
                    Text(
                        text = "Grab",
                        color = myColors.onPrimary,
                        fontWeight = FontWeight.SemiBold,
                        fontSize = 12.sp,
                    )
                }
            }
        }
    }
}
