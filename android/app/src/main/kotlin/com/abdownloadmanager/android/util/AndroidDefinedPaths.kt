package com.xetondownloadmanager.android.util

import com.xetondownloadmanager.shared.util.DefinedPaths
import okio.Path

class AndroidDefinedPaths(
    dataDir: Path,
) : DefinedPaths(
    dataDir = dataDir
) {
    val onboardingFile = pagesStateDir.resolve("onboarding.json")
    val homePageFile = pagesStateDir.resolve("home.json")
    val browserBookmarksFile = pagesStateDir.resolve("browser_bookmarks.json")
}
