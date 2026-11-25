// Xeton Download Manager - In-Page Webpage Link Scanner & Batch Capture Modal
// Equivalent to and exceeding Persepolis Link Grabber / DownThemAll batch capture.
// Supports sorting (Name, URL, Extension), regex filtering, inverted selection,
// category tabs, duplicate detection, and batch dispatch with cookies & referrers.

(() => {
  if (window.__XETON_SCANNER_INJECTED__) {
    window.__XETON_OPEN_SCANNER__?.();
    return;
  }
  window.__XETON_SCANNER_INJECTED__ = true;

  const EXT_CATEGORIES = {
    archives: ['zip', 'rar', '7z', 'tar', 'gz', 'bz2', 'xz', 'iso', 'dmg', 'apk', 'zst', 'tgz'],
    media: ['mp4', 'mkv', 'webm', 'avi', 'mov', 'flv', 'mp3', 'flac', 'wav', 'm4a', 'aac', 'ogg', 'opus', 'm3u8', 'mpd'],
    documents: ['pdf', 'epub', 'mobi', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt', 'csv', 'md'],
    programs: ['exe', 'msi', 'deb', 'rpm', 'appimage', 'bin', 'sh', 'apk', 'jar']
  };

  function getExtension(url) {
    try {
      const pathname = new URL(url, location.href).pathname;
      const lastDot = pathname.lastIndexOf('.');
      if (lastDot !== -1 && lastDot < pathname.length - 1) {
        return pathname.slice(lastDot + 1).toLowerCase();
      }
    } catch (_) {}
    return '';
  }

  function getCategory(ext) {
    for (const [cat, list] of Object.entries(EXT_CATEGORIES)) {
      if (list.includes(ext)) return cat;
    }
    return 'other';
  }

  function scanPageLinks() {
    const urls = new Map();

    // 1. Anchors
    for (const a of document.querySelectorAll('a[href]')) {
      const href = a.href;
      if (!href || href.startsWith('javascript:') || href.startsWith('mailto:') || href.startsWith('#') || href.startsWith('tel:')) continue;
      const text = (a.innerText || a.getAttribute('title') || a.getAttribute('download') || '').trim();
      const ext = getExtension(href);
      if (!urls.has(href)) {
        urls.set(href, {
          url: href,
          title: text || href.split('/').pop() || 'Download',
          ext: ext || 'html',
          category: getCategory(ext)
        });
      }
    }

    // 2. Media elements (video, audio, source)
    for (const m of document.querySelectorAll('video, audio, source')) {
      const src = m.src || m.currentSrc;
      if (src && src.startsWith('http')) {
        const ext = getExtension(src) || 'mp4';
        if (!urls.has(src)) {
          urls.set(src, {
            url: src,
            title: document.title || 'Media Stream',
            ext: ext,
            category: 'media'
          });
        }
      }
    }

    // 3. Images with downloadable formats
    for (const img of document.querySelectorAll('img[src]')) {
      const src = img.src;
      if (!src || !src.startsWith('http')) continue;
      const ext = getExtension(src);
      if (['jpg', 'jpeg', 'png', 'webp', 'gif', 'svg'].includes(ext)) {
        if (!urls.has(src)) {
          urls.set(src, {
            url: src,
            title: img.getAttribute('alt') || src.split('/').pop() || 'Image',
            ext: ext,
            category: 'media'
          });
        }
      }
    }

    return Array.from(urls.values());
  }

  let scannerModal = null;
  let scannedItems = [];
  let selectedUrls = new Set();
  let currentCategory = 'all';
  let currentSearch = '';
  let sortBy = 'title'; // 'title', 'url', 'ext'
  let sortAsc = true;

  function createScannerUI() {
    const container = document.createElement('div');
    container.id = 'xeton-scanner-overlay';
    container.style.cssText = `
      position: fixed; inset: 0; z-index: 2147483647;
      background: rgba(10, 11, 14, 0.78); backdrop-filter: blur(8px);
      display: flex; align-items: center; justify-content: center;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      color: #F0F2F5;
    `;

    container.innerHTML = `
      <div style="
        width: 860px; max-width: 95vw; height: 640px; max-height: 92vh;
        background: #17181C; border: 1px solid rgba(255,255,255,0.12);
        border-radius: 12px; display: flex; flex-direction: column;
        box-shadow: 0 24px 64px rgba(0,0,0,0.65); overflow: hidden;
      ">
        <!-- Header -->
        <div style="
          padding: 14px 20px; display: flex; align-items: center; justify-content: space-between;
          border-bottom: 1px solid rgba(255,255,255,0.08); background: #1C1E24;
        ">
          <div style="display: flex; align-items: center; gap: 10px;">
            <div style="width: 10px; height: 10px; border-radius: 50%; background: #6366F1; box-shadow: 0 0 12px #6366F1;"></div>
            <h2 style="margin: 0; font-size: 15px; font-weight: 700; letter-spacing: -0.01em;">Xeton Batch Link Capture</h2>
            <span id="xeton-scan-count" style="font-size: 11px; color: #9BA3AF; background: rgba(255,255,255,0.06); padding: 2px 8px; border-radius: 10px;">0 links found</span>
          </div>
          <button id="xeton-close-modal" style="
            background: transparent; border: none; color: #9BA3AF; font-size: 20px;
            cursor: pointer; padding: 2px 8px; border-radius: 6px;
          ">&times;</button>
        </div>

        <!-- Filter & Sorting Bar -->
        <div style="
          padding: 10px 20px; display: flex; align-items: center; justify-content: space-between; gap: 12px;
          border-bottom: 1px solid rgba(255,255,255,0.06); background: #15161A; flex-wrap: wrap;
        ">
          <!-- Category Tabs -->
          <div id="xeton-cat-pills" style="display: flex; gap: 6px; overflow-x: auto;">
            <button data-cat="all" class="xeton-pill xeton-pill-active">All</button>
            <button data-cat="media" class="xeton-pill">Media</button>
            <button data-cat="archives" class="xeton-pill">Archives</button>
            <button data-cat="documents" class="xeton-pill">Documents</button>
            <button data-cat="programs" class="xeton-pill">Programs</button>
          </div>

          <!-- Controls: Sort & Filter -->
          <div style="display: flex; align-items: center; gap: 8px;">
            <select id="xeton-sort-select" style="
              background: #202229; border: 1px solid rgba(255,255,255,0.1); border-radius: 6px;
              padding: 5px 8px; color: #FFF; font-size: 11px; outline: none; cursor: pointer;
            ">
              <option value="title_asc">Sort: Name (A-Z)</option>
              <option value="title_desc">Sort: Name (Z-A)</option>
              <option value="ext_asc">Sort: Extension</option>
              <option value="url_asc">Sort: URL</option>
            </select>

            <input id="xeton-search-input" type="text" placeholder="Regex or text filter..." style="
              background: #202229; border: 1px solid rgba(255,255,255,0.1); border-radius: 6px;
              padding: 5px 10px; color: #FFF; font-size: 11px; width: 190px; outline: none;
            " />
          </div>
        </div>

        <!-- Links List Header -->
        <div style="
          padding: 6px 20px; display: flex; align-items: center; gap: 10px;
          background: rgba(255,255,255,0.02); border-bottom: 1px solid rgba(255,255,255,0.04);
          font-size: 10px; font-weight: 700; color: #6B7280; text-transform: uppercase; letter-spacing: 0.05em;
        ">
          <span style="width: 20px;">✓</span>
          <span style="width: 44px;">Type</span>
          <span style="flex: 1;">Title / Target URL</span>
        </div>

        <!-- Links List Table -->
        <div id="xeton-links-table" style="
          flex: 1; overflow-y: auto; padding: 6px 16px; display: flex; flex-direction: column; gap: 3px;
        "></div>

        <!-- Footer / Action Bar -->
        <div style="
          padding: 12px 20px; display: flex; align-items: center; justify-content: space-between;
          border-top: 1px solid rgba(255,255,255,0.08); background: #1C1E24;
        ">
          <div style="display: flex; align-items: center; gap: 8px;">
            <button id="xeton-select-all" class="xeton-sub-btn">Select All</button>
            <button id="xeton-invert-selection" class="xeton-sub-btn">Invert</button>
            <button id="xeton-deselect-all" class="xeton-sub-btn">Deselect All</button>
            <span id="xeton-selected-count" style="font-size: 12px; color: #818CF8; font-weight: 600; margin-left: 6px;">0 selected</span>
          </div>
          <button id="xeton-send-btn" style="
            background: #6366F1; color: white; border: none; border-radius: 6px;
            padding: 8px 22px; font-size: 13px; font-weight: 600; cursor: pointer;
            box-shadow: 0 2px 10px rgba(99,102,241,0.4); transition: transform 120ms ease;
          ">Download Selected with Xeton</button>
        </div>
      </div>
    `;

    // Inject styles
    const style = document.createElement('style');
    style.textContent = `
      .xeton-pill {
        background: rgba(255,255,255,0.05); color: #9BA3AF; border: none; border-radius: 12px;
        padding: 4px 11px; font-size: 11px; font-weight: 600; cursor: pointer; transition: 120ms ease;
      }
      .xeton-pill:hover { background: rgba(255,255,255,0.1); color: #FFF; }
      .xeton-pill-active { background: #6366F1 !important; color: #FFF !important; }
      .xeton-sub-btn {
        background: transparent; color: #9BA3AF; border: 1px solid rgba(255,255,255,0.1);
        border-radius: 4px; padding: 4px 10px; font-size: 11px; cursor: pointer; transition: 120ms ease;
      }
      .xeton-sub-btn:hover { color: #FFF; border-color: rgba(255,255,255,0.2); }
      .xeton-link-row {
        display: flex; align-items: center; gap: 10px; padding: 6px 10px; border-radius: 6px;
        background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.03); cursor: pointer;
        transition: background 120ms ease;
      }
      .xeton-link-row:hover { background: rgba(255,255,255,0.06); }
      .xeton-ext-chip {
        font-size: 10px; font-weight: 700; text-transform: uppercase; padding: 2px 5px;
        border-radius: 4px; background: rgba(99,102,241,0.18); color: #818CF8; min-width: 44px; text-align: center;
      }
    `;
    container.appendChild(style);

    document.body.appendChild(container);
    return container;
  }

  function renderList() {
    const listContainer = document.getElementById('xeton-links-table');
    if (!listContainer) return;
    listContainer.innerHTML = '';

    // Filter by category and search/regex
    let filtered = scannedItems.filter(item => {
      const matchCat = currentCategory === 'all' || item.category === currentCategory;
      if (!matchCat) return false;

      if (!currentSearch) return true;

      try {
        const regex = new RegExp(currentSearch, 'i');
        return regex.test(item.title) || regex.test(item.url) || regex.test(item.ext);
      } catch (_) {
        return item.title.toLowerCase().includes(currentSearch) || item.url.toLowerCase().includes(currentSearch) || item.ext.toLowerCase().includes(currentSearch);
      }
    });

    // Sorting
    filtered.sort((a, b) => {
      let cmp = 0;
      if (sortBy === 'title') cmp = a.title.localeCompare(b.title);
      else if (sortBy === 'ext') cmp = a.ext.localeCompare(b.ext);
      else if (sortBy === 'url') cmp = a.url.localeCompare(b.url);
      return sortAsc ? cmp : -cmp;
    });

    document.getElementById('xeton-scan-count').textContent = `${filtered.length} of ${scannedItems.length} links`;
    document.getElementById('xeton-selected-count').textContent = `${selectedUrls.size} selected`;

    if (filtered.length === 0) {
      listContainer.innerHTML = `<div style="text-align: center; padding: 40px; color: #6B7280; font-size: 13px;">No matching links found.</div>`;
      return;
    }

    for (const item of filtered) {
      const row = document.createElement('div');
      row.className = 'xeton-link-row';
      const isChecked = selectedUrls.has(item.url);

      row.innerHTML = `
        <input type="checkbox" style="cursor: pointer;" ${isChecked ? 'checked' : ''} />
        <span class="xeton-ext-chip">${item.ext}</span>
        <div style="flex: 1; overflow: hidden;">
          <div style="font-size: 12px; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: #F0F2F5;">${item.title}</div>
          <div style="font-size: 11px; color: #6B7280; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${item.url}</div>
        </div>
      `;

      const checkbox = row.querySelector('input');
      row.addEventListener('click', (e) => {
        if (e.target !== checkbox) checkbox.checked = !checkbox.checked;
        if (checkbox.checked) selectedUrls.add(item.url);
        else selectedUrls.delete(item.url);
        document.getElementById('xeton-selected-count').textContent = `${selectedUrls.size} selected`;
      });

      listContainer.appendChild(row);
    }
  }

  function openScanner() {
    if (!scannerModal) {
      scannerModal = createScannerUI();

      // Close handler
      document.getElementById('xeton-close-modal').onclick = () => {
        scannerModal.style.display = 'none';
      };

      // Category pills
      document.getElementById('xeton-cat-pills').onclick = (e) => {
        const btn = e.target.closest('.xeton-pill');
        if (!btn) return;
        document.querySelectorAll('.xeton-pill').forEach(p => p.classList.remove('xeton-pill-active'));
        btn.classList.add('xeton-pill-active');
        currentCategory = btn.dataset.cat;
        renderList();
      };

      // Sorting select
      document.getElementById('xeton-sort-select').onchange = (e) => {
        const val = e.target.value;
        if (val === 'title_asc') { sortBy = 'title'; sortAsc = true; }
        else if (val === 'title_desc') { sortBy = 'title'; sortAsc = false; }
        else if (val === 'ext_asc') { sortBy = 'ext'; sortAsc = true; }
        else if (val === 'url_asc') { sortBy = 'url'; sortAsc = true; }
        renderList();
      };

      // Search
      document.getElementById('xeton-search-input').oninput = (e) => {
        currentSearch = e.target.value.trim();
        renderList();
      };

      // Selection buttons
      document.getElementById('xeton-select-all').onclick = () => {
        for (const item of scannedItems) {
          if (currentCategory === 'all' || item.category === currentCategory) {
            selectedUrls.add(item.url);
          }
        }
        renderList();
      };

      document.getElementById('xeton-invert-selection').onclick = () => {
        for (const item of scannedItems) {
          if (currentCategory === 'all' || item.category === currentCategory) {
            if (selectedUrls.has(item.url)) selectedUrls.delete(item.url);
            else selectedUrls.add(item.url);
          }
        }
        renderList();
      };

      document.getElementById('xeton-deselect-all').onclick = () => {
        selectedUrls.clear();
        renderList();
      };

      // Send batch
      document.getElementById('xeton-send-btn').onclick = () => {
        if (selectedUrls.size === 0) {
          alert('Please select at least one download.');
          return;
        }

        const batch = Array.from(selectedUrls).map(url => {
          const item = scannedItems.find(i => i.url === url);
          return {
            url: url,
            file_name: item ? item.title : url.split('/').pop() || 'download'
          };
        });

        chrome.runtime.sendMessage({
          action: "send_batch_to_xeton",
          batch: batch,
          referrer: location.href
        }, () => {
          scannerModal.style.display = 'none';
        });
      };
    }

    scannedItems = scanPageLinks();
    selectedUrls.clear();
    // Default select downloads that look like files/media/archives
    for (const item of scannedItems) {
      if (['archives', 'media', 'documents', 'programs'].includes(item.category)) {
        selectedUrls.add(item.url);
      }
    }

    scannerModal.style.display = 'flex';
    renderList();
  }

  window.__XETON_OPEN_SCANNER__ = openScanner;
  openScanner();
})();
