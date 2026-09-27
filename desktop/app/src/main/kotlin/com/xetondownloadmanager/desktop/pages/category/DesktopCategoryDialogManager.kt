package com.xetondownloadmanager.desktop.pages.category

import com.xetondownloadmanager.shared.pagemanager.CategoryDialogManager
import com.xetondownloadmanager.shared.pages.category.CategoryComponent
import kotlinx.coroutines.flow.StateFlow

interface DesktopCategoryDialogManager : CategoryDialogManager {
    val openedCategoryDialogs: StateFlow<List<CategoryComponent>>
    fun closeCategoryDialog(categoryId: Long)
}
