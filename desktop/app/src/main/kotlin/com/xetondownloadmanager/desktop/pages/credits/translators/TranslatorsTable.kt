package com.xetondownloadmanager.desktop.pages.credits.translators

import com.xetondownloadmanager.shared.ui.widget.table.customtable.CellSize
import com.xetondownloadmanager.shared.ui.widget.table.customtable.SortableCell
import com.xetondownloadmanager.shared.ui.widget.table.customtable.TableCell
import androidx.compose.ui.unit.dp
import com.xetondownloadmanager.resources.Res
import com.xetondownloadmanager.shared.pages.credits.translators.LanguageTranslationInfo
import com.xeton.util.compose.StringSource
import com.xeton.util.compose.asStringSource

sealed interface TranslatorsCells : TableCell<LanguageTranslationInfo> {
    data object LanguageName : TranslatorsCells,
        SortableCell<LanguageTranslationInfo> {
        override fun comparator(): Comparator<LanguageTranslationInfo> = compareBy { it.locale }
        override val id: String = "language"
        override val name: StringSource = Res.string.language.asStringSource()
        override val size: CellSize = CellSize.Resizeable(100.dp..1000.dp, 200.dp)
    }

    data object Translators : TranslatorsCells {
        override val id: String = "translators"
        override val name: StringSource = Res.string.translators.asStringSource()
        override val size: CellSize = CellSize.Resizeable(100.dp..1000.dp, 350.dp)
    }

    companion object {
        fun all() = listOf(
            LanguageName,
            Translators,
        )
    }
}
