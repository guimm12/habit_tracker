const CACHE_NAME = "habit-tracker-cache-v1";
const BASE_PATH = "test-pwa-wasm-rust";

let urls = ["/", "/index.html", "/pkg/habit_tracker_bg.wasm", "/pkg/habit_tracker.js", "/manifest.json"];
const urlsToCache = urls.map((url) => BASE_PATH + url);

// Install the service worker and cache resources
self.addEventListener("install", (event) => {
	event.waitUntil(
		caches.open(CACHE_NAME).then((cache) => {
			return cache.addAll(urlsToCache);
		})
	);
});

// Respond with cached resources
self.addEventListener("fetch", (event) => {
	event.respondWith(
		caches.match(event.request).then((response) => {
			return response || fetch(event.request);
		})
	);
});

// Update the service worker
self.addEventListener("activate", (event) => {
	const cacheWhitelist = [CACHE_NAME];
	event.waitUntil(
		caches.keys().then((cacheNames) => {
			return Promise.all(
				cacheNames.map((cacheName) => {
					if (!cacheWhitelist.includes(cacheName)) {
						return caches.delete(cacheName);
					}
				})
			);
		})
	);
});
