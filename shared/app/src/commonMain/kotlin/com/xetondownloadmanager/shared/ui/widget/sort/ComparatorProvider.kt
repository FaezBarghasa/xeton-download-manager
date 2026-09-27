package com.xetondownloadmanager.shared.ui.widget.sort

interface ComparatorProvider<T> {
    fun comparator(): Comparator<T>
}
