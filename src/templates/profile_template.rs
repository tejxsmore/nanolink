use std::env;
use super::base_styles::BASE_STYLES;

pub fn get_template() -> String {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    TEMPLATE_RAW
        .replace("BASE_STYLES_PLACEHOLDER", BASE_STYLES)
        .replace("BASE_URL_PLACEHOLDER", &base_url)
}

const TEMPLATE_RAW: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Profile - NanoLink</title>
    <style>
        BASE_STYLES_PLACEHOLDER

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }

        .profile-header {
            background-color: #1A1A1D;
            padding: 2rem;
            border-radius: 5px;
            border: 1px solid #222831;
            margin-bottom: 2rem;
            text-align: center;
        }

        .profile-header h1 {
            font-size: 2rem;
            color: #f4f4f5;
            margin-bottom: 0.5rem;
            font-weight: 600;
        }

        .profile-header p {
            color: #a1a1aa;
            font-size: 0.875rem;
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 2rem;
            margin-bottom: 2rem;
        }

        .stat-card {
            background-color: #1A1A1D;
            padding: 1.5rem;
            border-radius: 5px;
            border: 1px solid #222831;
            text-align: center;
        }

        .stat-number {
            font-size: 1.75rem;
            font-weight: 700;
            color: #F2613F;
            margin-bottom: 0.25rem;
        }

        .stat-label {
            color: #a1a1aa;
            font-size: 0.875rem;
        }

        .urls-section {
            background-color: #1A1A1D;
            padding: 2rem;
            border-radius: 5px;
            border: 1px solid #222831;
        }

        .section-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.5rem;
        }

        .section-header h2 {
            color: #f4f4f5;
            font-size: 1.25rem;
            font-weight: 600;
        }

        .url-grid {
            display: grid;
            gap: 1rem;
        }

        .url-card {
            background-color: #0c0c0c;
            padding: 1.25rem;
            border-radius: 5px;
            border: 1px solid #222831;
            transition: border-color 0.2s ease;
        }

        .url-card:hover {
            border-color: #3f3f46;
        }

        .url-code {
            font-size: 1rem;
            font-weight: 600;
            color: #F2613F;
            margin-bottom: 0.5rem;
        }

        .url-original {
            color: #e4e4e7;
            word-break: break-all;
            margin-bottom: 1rem;
            font-size: 0.875rem;
        }

        .url-meta {
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 0.75rem;
            color: #71717a;
            margin-bottom: 1rem;
        }

        .url-link {
            color: #F2613F;
            text-decoration: none;
            font-weight: 500;
            font-size: 0.875rem;
            margin-bottom: 0.75rem;
            display: inline-flex;
            align-items: center;
            gap: 0.25rem;
        }

        .url-link:hover {
            text-decoration: underline;
        }

        .url-actions {
            display: flex;
            gap: 0.75rem;
            flex-wrap: wrap;
        }

        .action-btn {
            padding: 0.5rem 1rem;
            background-color: #1A1A1D;
            border: 1px solid #222831;
            border-radius: 5px;
            color: #F0ECE5;
            cursor: pointer;
            font-size: 0.875rem;
            font-weight: 500;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            transition: all 0.2s ease;
            text-decoration: none;
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .action-btn:hover {
            color: white;
            background-color: #0C0C0C;
        }

        .action-btn.copied {
            background-color: #F2613F;
            color: white;
            border-color: #9B3922;
        }

        .empty-state {
            text-align: center;
            padding: 3rem;
            color: #71717a;
        }

        .empty-state h3 {
            margin-bottom: 0.75rem;
            color: #a1a1aa;
            font-weight: 500;
        }

        .loading {
            text-align: center;
            padding: 2rem;
            color: #71717a;
        }

        /* Modal Styles */
        .modal {
            display: none;
            position: fixed;
            z-index: 1000;
            left: 0;
            top: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.7);
            backdrop-filter: blur(8px);
        }

        .modal-content {
            background-color: #1A1A1D;
            margin: 5% auto;
            padding: 2rem;
            border: 1px solid #222831;
            border-radius: 5px;
            width: 90%;
            max-width: 500px;
            position: relative;
        }

        .modal-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.5rem;
        }

        .modal-header h3 {
            color: #F0ECE5;
            margin: 0;
            font-size: 1.25rem;
            font-weight: 600;
        }

        .close {
            color: #F0ECE5;
            font-size: 2rem;
            font-weight: bold;
            cursor: pointer;
            line-height: 1;
            transition: color 0.2s ease;
        }

        .close:hover {
            color: #F2613F;
        }

        .qr-container {
            text-align: center;
            margin-bottom: 1.5rem;
        }

        .qr-code {
            background-color: white;
            padding: 1rem;
            border-radius: 5px;
            display: inline-block;
            margin-bottom: 1rem;
        }

        .modal-url {
            background-color: #0C0C0C;
            padding: 0.75rem;
            border-radius: 5px;
            border: 1px solid #222831;
            color: #F0ECE5;
            font-family: 'Inter', monospace;
            font-size: 0.875rem;
            word-break: break-all;
            margin-bottom: 1rem;
        }

        .modal-actions {
            display: flex;
            gap: 0.75rem;
            justify-content: center;
        }

        .btn-modal {
            padding: 0.5rem 1rem;
            border: 1px solid #222831;
            border-radius: 5px;
            cursor: pointer;
            font-size: 0.875rem;
            font-weight: 500;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            transition: all 0.2s ease;
            background-color: transparent;
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .btn-modal.primary {
            background-color: #F2613F;
            color: white;
            border-color: #9B3922;
        }

        .btn-modal.primary:hover {
            background-color: #ff6f4b;
            border-color: #9B3922;
        }

        .btn-modal.secondary {
            color: #F0ECE5;
            border-color: #222831;
        }

        .btn-modal.secondary:hover {
            background-color: #0c0c0c;
        }

        /* Confirmation Dialog */
        .confirm-dialog {
            display: none;
            position: fixed;
            z-index: 1001;
            left: 0;
            top: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.8);
            backdrop-filter: blur(8px);
        }

        .confirm-content {
            background-color: #1A1A1D;
            margin: 15% auto;
            padding: 2rem;
            border: 1px solid #222831;
            border-radius: 5px;
            width: 90%;
            max-width: 400px;
            text-align: center;
        }

        .confirm-content h3 {
            color: #F0ECE5;
            margin-bottom: 1rem;
            font-weight: 600;
            font-size: 1.25rem;
        }

        .confirm-content p {
            color: #F0ECE5;
            margin-bottom: 1.5rem;
            line-height: 1.6;
        }

        .confirm-actions {
            display: flex;
            gap: 0.75rem;
            justify-content: center;
        }

        .btn-danger {
            background-color: #F2613F;
            color: white;
            border-color: #9B3922;
        }

        .btn-danger:hover {
            background-color: #ff6f4b;
            border-color: #9B3922;
        }

        @media (max-width: 768px) {
            .container {
                padding: 1rem;
            }
            
            .profile-header h1 {
                font-size: 1.5rem;
            }

            .profile-header {
                margin-bottom: 1rem;
            }
            
            .stats-grid {
                grid-template-columns: repeat(2, 1fr);
                margin-bottom: 1rem;
                gap: 1rem;
            }
            
            .section-header {
                flex-direction: column;
                gap: 1rem;
                align-items: stretch;
            }

            .url-actions {
                justify-content: start;
            }

            .modal-content {
                margin: 10% auto;
                padding: 1.5rem;
            }
        }
    </style>
</head>
<body>
    <nav class="navbar">
        <div class="nav-container">
            <a href="/" class="logo">
                <svg style="width: 1.5rem; height: 1.5rem;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.042 21.672 13.684 16.6m0 0-2.51 2.225.569-9.47 5.227 7.917-3.286-.672Zm-7.518-.267A8.25 8.25 0 1 1 20.25 10.5M8.288 14.212A5.25 5.25 0 1 1 17.25 10.5" />
                </svg>
                NanoLink
            </a>
            <div class="nav-buttons">
                <span class="btn btn-secondary" id="profileBtn">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                        <circle cx="12" cy="7" r="4"/>
                    </svg>
                    Loading...
                </span>
                <button onclick="logout()" class="btn btn-secondary">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                        <polyline points="16,17 21,12 16,7"/>
                        <line x1="21" y1="12" x2="9" y2="12"/>
                    </svg>
                    Logout
                </button>
            </div>
        </div>
    </nav>

    <div class="container">
        <div class="profile-header">
            <h1 id="welcomeMessage">Welcome!</h1>
            <p>Manage your shortened URLs and track their performance</p>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-number" id="totalUrls">0</div>
                <div class="stat-label">Total URLs</div>
            </div>
            <div class="stat-card">
                <div class="stat-number" id="totalClicks">0</div>
                <div class="stat-label">Total Clicks</div>
            </div>
            <div class="stat-card">
                <div class="stat-number" id="avgClicks">0</div>
                <div class="stat-label">Avg Clicks per URL</div>
            </div>
        </div>

        <div class="urls-section">
            <div class="section-header">
                <h2>
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.5rem;">
                        <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
                        <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
                    </svg>
                    Your URLs
                </h2>
                <a href="/" class="btn btn-primary">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="12" y1="5" x2="12" y2="19"/>
                        <line x1="5" y1="12" x2="19" y2="12"/>
                    </svg>
                    Create New URL
                </a>
            </div>
            
            <div id="urlsContainer">
                <div class="loading">Loading your URLs...</div>
            </div>
        </div>
    </div>

    <!-- QR Code Modal -->
    <div id="qrModal" class="modal">
        <div class="modal-content">
            <div class="modal-header">
                <h3>QR Code</h3>
                <span class="close" onclick="closeQRModal()">&times;</span>
            </div>
            <div class="qr-container">
                <div id="qrcode" class="qr-code"></div>
                <div id="qrUrl" class="modal-url"></div>
            </div>
            <div class="modal-actions">
                <button onclick="downloadQR()" class="btn-modal primary">
                    <svg style="width: 1rem; height: 1rem;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="7,10 12,15 17,10"/>
                        <line x1="12" y1="15" x2="12" y2="3"/>
                    </svg>
                    Download QR
                </button>
                <button onclick="closeQRModal()" class="btn-modal secondary">Close</button>
            </div>
        </div>
    </div>

    <!-- Confirmation Dialog -->
    <div id="confirmDialog" class="confirm-dialog">
        <div class="confirm-content">
            <h3>Delete URL</h3>
            <p>Are you sure you want to delete this shortened URL? This action cannot be undone.</p>
            <div class="confirm-actions">
                <button onclick="confirmDelete()" class="btn-modal btn-danger">Delete</button>
                <button onclick="closeConfirmDialog()" class="btn-modal secondary">Cancel</button>
            </div>
        </div>
    </div>

    <script src="https://cdnjs.cloudflare.com/ajax/libs/qrcode/1.5.3/qrcode.min.js"></script>
    <script>
        let authToken = localStorage.getItem('authToken');
        let urlToDelete = null;
        
        if (!authToken) {
            window.location.href = '/signin';
        }

        function logout() {
            localStorage.removeItem('authToken');
            window.location.href = '/';
        }

        async function makeRequest(url, options = {}) {
            if (authToken) {
                options.headers = {
                    ...options.headers,
                    'Authorization': `Bearer ${authToken}`
                };
            }
            return fetch(url, options);
        }

        function formatDate(dateString) {
            return new Date(dateString).toLocaleDateString('en-US', {
                year: 'numeric',
                month: 'short',
                day: 'numeric',
                hour: '2-digit',
                minute: '2-digit'
            });
        }

        async function copyToClipboard(text, buttonElement) {
            try {
                await navigator.clipboard.writeText(text);
                const originalText = buttonElement.innerHTML;
                buttonElement.innerHTML = `
                    <svg style="width: 1rem; height: 1rem;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20,6 9,17 4,12"/>
                    </svg>
                    Copied!
                `;
                buttonElement.classList.add('copied');
                
                setTimeout(() => {
                    buttonElement.innerHTML = originalText;
                    buttonElement.classList.remove('copied');
                }, 2000);
            } catch (err) {
                console.error('Failed to copy: ', err);
                // Fallback for older browsers
                const textArea = document.createElement('textarea');
                textArea.value = text;
                document.body.appendChild(textArea);
                textArea.select();
                document.execCommand('copy');
                document.body.removeChild(textArea);
                alert('Link copied to clipboard!');
            }
        }

        function showQRModal(url, shortCode) {
            const modal = document.getElementById('qrModal');
            const qrContainer = document.getElementById('qrcode');
            const urlElement = document.getElementById('qrUrl');
            
            // Clear previous QR code
            qrContainer.innerHTML = '';
            urlElement.textContent = url;
            
            // Generate QR code
            QRCode.toCanvas(qrContainer, url, {
                width: 200,
                margin: 2,
                color: {
                    dark: '#000000',
                    light: '#FFFFFF'
                }
            }, function (error) {
                if (error) console.error(error);
            });
            
            modal.style.display = 'block';
        }

        function closeQRModal() {
            document.getElementById('qrModal').style.display = 'none';
        }

        function downloadQR() {
            const canvas = document.querySelector('#qrcode canvas');
            if (canvas) {
                const link = document.createElement('a');
                link.download = 'qr-code.png';
                link.href = canvas.toDataURL();
                link.click();
            }
        }

        function showDeleteConfirm(shortCode) {
            urlToDelete = shortCode;
            document.getElementById('confirmDialog').style.display = 'block';
        }

        function closeConfirmDialog() {
            document.getElementById('confirmDialog').style.display = 'none';
            urlToDelete = null;
        }

        async function confirmDelete() {
            if (!urlToDelete) return;
            
            try {
                const response = await makeRequest(`/delete-url/${urlToDelete}`, {
                    method: 'DELETE'
                });
                
                if (response.ok) {
                    closeConfirmDialog();
                    loadProfile(); // Reload the profile to update the list
                } else {
                    alert('Failed to delete URL. Please try again.');
                }
            } catch (error) {
                console.error('Error deleting URL:', error);
                alert('Failed to delete URL. Please try again.');
            }
        }

        // Close modals when clicking outside
        window.onclick = function(event) {
            const qrModal = document.getElementById('qrModal');
            const confirmDialog = document.getElementById('confirmDialog');
            
            if (event.target === qrModal) {
                closeQRModal();
            }
            if (event.target === confirmDialog) {
                closeConfirmDialog();
            }
        }

        async function loadProfile() {
            try {
                const payload = JSON.parse(atob(authToken.split('.')[1]));
                document.getElementById('welcomeMessage').textContent = `Welcome, ${payload.username}!`;
                document.getElementById('profileBtn').innerHTML = `
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                        <circle cx="12" cy="7" r="4"/>
                    </svg>
                    ${payload.username}
                `;
                
                const response = await makeRequest('/my-urls');
                const urls = await response.json();
                
                const totalUrls = urls.length;
                const totalClicks = urls.reduce((sum, url) => sum + url.click_count, 0);
                const avgClicks = totalUrls > 0 ? Math.round(totalClicks / totalUrls) : 0;
                
                document.getElementById('totalUrls').textContent = totalUrls;
                document.getElementById('totalClicks').textContent = totalClicks;
                document.getElementById('avgClicks').textContent = avgClicks;
                
                const urlsContainer = document.getElementById('urlsContainer');
                
                if (urls.length === 0) {
                    urlsContainer.innerHTML = `
                        <div class="empty-state">
                            <h3>No URLs yet</h3>
                            <p>Create your first shortened URL to get started!</p>
                            <a href="/" class="btn btn-primary" style="margin-top: 1rem; display: inline-flex;">
                                <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <line x1="12" y1="5" x2="12" y2="19"/>
                                    <line x1="5" y1="12" x2="19" y2="12"/>
                                </svg>
                                Create URL
                            </a>
                        </div>
                    `;
                } else {
                    urlsContainer.innerHTML = `
                        <div class="url-grid">
                            ${urls.map(url => {
                                const fullUrl = `${"BASE_URL_PLACEHOLDER"}/${url.short_code}`;
                                return `
                                    <div class="url-card">
                                        <div class="url-code">${url.short_code}</div>
                                        <div class="url-original">${url.original_url}</div>
                                        <div class="url-meta">
                                            <span style="display: flex; align-items: center;">
                                                <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.5rem;">
                                                    <circle cx="12" cy="12" r="10"/>
                                                    <polyline points="12,6 12,12 16,14"/>
                                                </svg>
                                                ${formatDate(url.created_at)}
                                            </span>
                                            <span style="display: flex; align-items: center;">
                                                <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.5rem;">
                                                    <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/>
                                                    <circle cx="12" cy="12" r="3"/>
                                                </svg>
                                                ${url.click_count} clicks
                                            </span>
                                        </div>
                                        <a href="${fullUrl}" target="_blank" class="url-link" style="gap: 0.5rem;">
                                            ${fullUrl}
                                            <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
                                                <polyline points="15,3 21,3 21,9"/>
                                                <line x1="10" y1="14" x2="21" y2="3"/>
                                            </svg>
                                        </a>
                                        <div class="url-actions">
                                            <button onclick="copyToClipboard('${fullUrl}', this)" class="action-btn copy">
                                                <svg style="width: 0.75rem; height: 0.75rem;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                                                </svg>
                                                Copy Link
                                            </button>
                                            <button onclick="showQRModal('${fullUrl}', '${url.short_code}')" class="action-btn qr">
                                                <svg class="icon" style="width: 0.75rem; height: 0.75rem;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                    <rect x="3" y="3" width="5" height="5"/>
                                                    <rect x="16" y="3" width="5" height="5"/>
                                                    <rect x="3" y="16" width="5" height="5"/>
                                                    <path d="M21 16h-3a2 2 0 0 0-2 2v3"/>
                                                    <path d="M21 21v.01"/>
                                                    <path d="M12 7v3a2 2 0 0 1-2 2H7"/>
                                                    <path d="M3 12h.01"/>
                                                    <path d="M12 3h.01"/>
                                                    <path d="M12 16v.01"/>
                                                    <path d="M16 12h1"/>
                                                    <path d="M21 12v.01"/>
                                                    <path d="M12 21v-1"/>
                                                </svg>
                                                QR Code
                                            </button>
                                            <button onclick="showDeleteConfirm('${url.short_code}')" class="action-btn delete">
                                                <svg class="icon" style="width: 0.75rem; height: 0.75rem;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                    <polyline points="3,6 5,6 21,6"/>
                                                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                                                    <line x1="10" y1="11" x2="10" y2="17"/>
                                                    <line x1="14" y1="11" x2="14" y2="17"/>
                                                </svg>
                                                Delete
                                            </button>
                                        </div>
                                    </div>
                                `;
                            }).join('')}
                        </div>
                    `;
                }
            } catch (error) {
                console.error('Failed to load profile:', error);
                document.getElementById('urlsContainer').innerHTML = `
                    <div class="empty-state">
                        <h3>Error loading data</h3>
                        <p>Please try refreshing the page.</p>
                    </div>
                `;
            }
        }

        loadProfile();
    </script>
</body>
</html>
"#;