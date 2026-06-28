package com.xetondownloadmanager.desktop.storage

import com.xetondownloadmanager.shared.util.DefinedPaths
import okio.Path
import java.io.File

class DesktopDefinedPaths(
    dataDir: Path
) : DefinedPaths(
    dataDir
) {
    val pageStatesStorageFile: Path = configDir.resolve("pageStatesStorage.json")
    val renderApiFile: Path = optionsDir.resolve("renderApi.txt")
}
