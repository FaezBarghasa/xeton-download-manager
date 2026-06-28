package com.xetondownloadmanager.shared.pages.adddownload.multiple

import com.xetondownloadmanager.shared.util.category.CategorySelectionMode
import com.xeton.downloader.NewDownloadItemProps
import kotlinx.coroutines.Deferred

fun interface OnRequestAddMultipleItem {
    operator fun invoke(
        items: List<NewDownloadItemProps>,
        queueId: Long?,
        categorySelectionMode: CategorySelectionMode?,
    ): Deferred<List<Long>>
}
