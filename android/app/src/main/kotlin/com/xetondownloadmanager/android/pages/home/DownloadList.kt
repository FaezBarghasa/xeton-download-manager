package com.xetondownloadmanager.android.pages.home

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.util.ui.widget.MyIcon
import com.xetondownloadmanager.shared.ui.widget.Text
import com.xetondownloadmanager.shared.util.FileIconProvider
import com.xetondownloadmanager.shared.util.div
import com.xetondownloadmanager.shared.util.downloaderror.DownloadErrorReason
import com.xetondownloadmanager.shared.util.ui.WithContentAlpha
import com.xetondownloadmanager.shared.util.ui.icon.MyIcons
import com.xetondownloadmanager.shared.util.ui.myColors
import com.xetondownloadmanager.shared.util.ui.theme.myTextSizes
import com.xeton.downloader.monitor.IDownloadItemState
import com.xeton.util.compose.resources.myStringResource
import com.xeton.util.ifThen


@Composable
fun DownloadList(
    downloadList: List<IDownloadItemState>,
    selectionList: List<Long>,
    onItemSelectionChange: (Long, Boolean) -> Unit,
    onItemClicked: (IDownloadItemState) -> Unit,
    fileIconProvider: FileIconProvider,
    onNewSelection: (List<Long>) -> Unit,
    lazyListState: LazyListState,
    modifier: Modifier,
    contentPadding: PaddingValues,
    downloadErrorReasons: Map<Long, DownloadErrorReason>,
) {

    fun newSelection(ids: List<Long>, isSelected: Boolean) {
        onNewSelection(ids.filter { isSelected })
    }

    fun changeAllSelection(isSelected: Boolean) {
        newSelection(downloadList.map { it.id }, isSelected)
    }

    val isInSelectMode = selectionList.isNotEmpty()
    BackHandler(
        isInSelectMode
    ) {
        changeAllSelection(false)
    }
    val dividerColor = myColors.onBackground / 0.5f
    Box {
        LazyColumn(
            state = lazyListState,
            modifier = modifier,
            contentPadding = contentPadding
        ) {
            itemsIndexed(
                items = downloadList,
                key = { _, item -> item.id }
            ) { index, item ->
                val isFirstItem = index == 0
                Column(
                    modifier = Modifier.animateItem()
                ) {
                    RenderDownloadItem(
                        downloadItem = item,
                        errorReason = downloadErrorReasons[item.id],
                        checked = if (isInSelectMode) {
                            item.id in selectionList
                        } else {
                            null
                        },
                        onClick = {
                            if (isInSelectMode) {
                                val wasInSelections = item.id in selectionList
                                onItemSelectionChange(item.id, !wasInSelections)
                            } else {
                                onItemClicked(item)
                            }
                        },
                        onLongClick = {
                            val wasInSelections = item.id in selectionList
                            onItemSelectionChange(item.id, !wasInSelections)
                        },
                        fileIconProvider = fileIconProvider,
                        modifier = Modifier.ifThen(!isFirstItem) {
                            drawBehind {
                                drawLine(
                                    brush = Brush.horizontalGradient(
                                        listOf(
                                            Color.Transparent,
                                            dividerColor,
                                            Color.Transparent,
                                        )
                                    ),
                                    start = Offset.Zero,
                                    end = Offset(size.width, 0f)
                                )
                            }
                        },
                    )
                }
            }
        }
        if (downloadList.isEmpty()) {
            Box(
                Modifier
                    .padding()
                    .fillMaxSize()
            ) {
                Column(
                    modifier = Modifier.align(Alignment.Center),
                    horizontalAlignment = Alignment.CenterHorizontally,
                ) {
                    MyIcon(
                        MyIcons.download,
                        contentDescription = null,
                        modifier = Modifier.size(56.dp),
                        tint = myColors.onBackground / 0.3f,
                    )
                    Spacer(Modifier.height(16.dp))
                    Text(
                        myStringResource(Res.string.list_is_empty),
                        fontSize = myTextSizes.base,
                        color = myColors.onBackground / 0.85f,
                        maxLines = 1,
                    )
                    Spacer(Modifier.height(4.dp))
                    Text(
                        "Tap + below to start downloading",
                        fontSize = myTextSizes.xs,
                        color = myColors.onBackground / 0.45f,
                        maxLines = 1,
                    )
                }
            }
        }
    }

}
