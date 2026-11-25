// Xeton Mobile Background Script — Intercepts headers and media links
const port = browser.runtime.connectNative("xeton_bridge");

browser.webRequest.onBeforeSendHeaders.addListener(
  (details) => {
    const headers = {};
    for (const h of details.requestHeaders || []) {
      headers[h.name.toLowerCase()] = h.value;
    }

    port.postMessage({
      type: "REQUEST_HEADERS",
      url: details.url,
      tabId: details.tabId,
      headers: headers
    });

    return { requestHeaders: details.requestHeaders };
  },
  { urls: ["<all_urls>"] },
  ["blocking", "requestHeaders"]
);

// Listen for content script messages
browser.runtime.onMessage.addListener((message, sender) => {
  if (message.type === "MEDIA_DETECTED") {
    port.postMessage({
      type: "MEDIA_SNIFFED",
      url: message.url,
      pageUrl: sender.url,
      tabId: sender.tab ? sender.tab.id : null,
      mimeType: message.mimeType,
      title: message.title
    });
  }
});
