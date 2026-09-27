package com.xetondownloadmanager.shared.util.category

interface ICategoryItemProvider {
    suspend fun getAll(): List<CategoryItemWithId>
}
