use super::base_styles::BASE_STYLES;

pub fn get_template() -> String {
    TEMPLATE_RAW.replace("BASE_STYLES_PLACEHOLDER", BASE_STYLES)
}

const TEMPLATE_RAW: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NanoLink - URL Shortener</title>
    <style>
        BASE_STYLES_PLACEHOLDER

        .container {
            max-width: 600px;
            margin: 0 auto;
            padding: 4rem 2rem;
        }

        .hero {
            text-align: center;
            margin-bottom: 3rem;
        }

        .hero h1 {
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 0.75rem;
            color: #f4f4f5;
        }

        .hero p {
            font-size: 1rem;
            color: #a1a1aa;
            margin-bottom: 2rem;
        }

        .url-form {
            background-color: #1A1A1D;
            padding: 2rem;
            border-radius: 5px;
            border: 1px solid #222831;
        }

        .form-note {
            font-size: 0.75rem;
            color: #71717a;
            margin-top: 0.25rem;
        }

        .result {
            margin-top: 1.5rem;
            padding: 1rem;
            border-radius: 5px;
            border: 1px solid #222831;
            background-color: #0f0f0f;
        }

        .result.success {
            border-color: #22c55e;
            background-color: rgba(34, 197, 94, 0.1);
        }

        .result.error {
            border-color: #ef4444;
            background-color: rgba(239, 68, 68, 0.1);
        }

        .result h4 {
            margin-bottom: 0.5rem;
            color: #f4f4f5;
            font-size: 0.875rem;
        }

        .result a {
            color: #F2613F;
            text-decoration: none;
            font-weight: 500;
        }

        .result a:hover {
            text-decoration: underline;
        }

        .auth-required {
            background-color: rgba(99, 102, 241, 0.1);
            border: 1px solid #6366f1;
            padding: 1rem;
            border-radius: 5px;
            margin-bottom: 1rem;
            text-align: center;
        }

        .auth-required p {
            margin-bottom: 0.75rem;
            color: #e4e4e7;
            font-size: 0.875rem;
        }

        @media (max-width: 768px) {
            .hero h1 {
                font-size: 2rem;
            }
            
            .container {
                padding: 2rem 1rem;
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
            <div class="nav-buttons" id="navButtons">
                <a href="/signin" class="btn btn-secondary">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                        <polyline points="10,17 15,12 10,7"/>
                        <line x1="15" y1="12" x2="3" y2="12"/>
                    </svg>
                    Sign In
                </a>
                <a href="/signup" class="btn btn-primary">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
                        <circle cx="9" cy="7" r="4"/>
                        <line x1="19" y1="8" x2="19" y2="14"/>
                        <line x1="22" y1="11" x2="16" y2="11"/>
                    </svg>
                    Sign Up
                </a>
            </div>
        </div>
    </nav>

    <div class="container">
        <div class="hero">
            <h1>Shorten Your URLs</h1>
            <p>Create short, memorable links in seconds. Track clicks and manage your URLs with ease.</p>
        </div>

        <div class="url-form">
            <div id="authRequired" class="auth-required hidden">
                <p>
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.25rem;">
                        <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                        <circle cx="12" cy="16" r="1"/>
                        <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                    </svg>
                    Custom short codes require authentication
                </p>
                <a href="/signin" class="btn btn-primary">Sign In to Continue</a>
            </div>

            <form id="shortenForm">
                <div class="form-group">
                    <label for="originalUrl">
                        <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.25rem;">
                            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
                            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
                        </svg>
                        Original URL
                    </label>
                    <input type="url" id="originalUrl" class="form-input" placeholder="https://example.com" required>
                </div>
                
                <div class="form-group">
                    <label for="shortCode">
                        Custom Short Code (Optional)
                    </label>
                    <input type="text" id="shortCode" class="form-input" placeholder="my-custom-code">
                    <div class="form-note">Leave empty for a random code. Custom codes require sign in.</div>
                </div>
                
                <button type="submit" class="btn btn-primary" style="width: 100%; justify-content: center; padding: 0.75rem;">
                    Shorten URL
                </button>
            </form>

            <div id="result" class="result hidden"></div>
        </div>
    </div>

    <script>
        let authToken = localStorage.getItem('authToken');
        
        function updateNavbar() {
            const navButtons = document.getElementById('navButtons');
            
            if (authToken) {
                const payload = JSON.parse(atob(authToken.split('.')[1]));
                navButtons.innerHTML = `
                    <a href="/profile" class="btn profile-btn">
                        <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                            <circle cx="12" cy="7" r="4"/>
                        </svg>
                        ${payload.username}
                    </a>
                    <button onclick="logout()" class="btn btn-secondary">
                        <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                            <polyline points="16,17 21,12 16,7"/>
                            <line x1="21" y1="12" x2="9" y2="12"/>
                        </svg>
                        Logout
                    </button>
                `;
            } else {
                navButtons.innerHTML = `
                    <a href="/signin" class="btn btn-secondary">
                        <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                            <polyline points="10,17 15,12 10,7"/>
                            <line x1="15" y1="12" x2="3" y2="12"/>
                        </svg>
                        Sign In
                    </a>
                    <a href="/signup" class="btn btn-primary">
                        <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
                            <circle cx="9" cy="7" r="4"/>
                            <line x1="19" y1="8" x2="19" y2="14"/>
                            <line x1="22" y1="11" x2="16" y2="11"/>
                        </svg>
                        Sign Up
                    </a>
                `;
            }
        }

        function logout() {
            localStorage.removeItem('authToken');
            authToken = null;
            updateNavbar();
            location.reload();
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

        document.getElementById('shortenForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const originalUrl = document.getElementById('originalUrl').value;
            const shortCode = document.getElementById('shortCode').value;
            const authRequired = document.getElementById('authRequired');
            
            if (shortCode && !authToken) {
                authRequired.classList.remove('hidden');
                return;
            } else {
                authRequired.classList.add('hidden');
            }
            
            try {
                const response = await makeRequest('/shorten', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        original_url: originalUrl,
                        short_code: shortCode || null
                    })
                });
                
                const data = await response.json();
                const resultDiv = document.getElementById('result');
                
                if (response.ok) {
                    resultDiv.className = 'result success';
                    resultDiv.innerHTML = `
                        <h4>✅ Success!</h4>
                        <p>Your short URL: <a href="${data.short_url}" target="_blank">${data.short_url}</a></p>
                    `;
                    resultDiv.classList.remove('hidden');
                    
                    document.getElementById('originalUrl').value = '';
                    document.getElementById('shortCode').value = '';
                } else {
                    resultDiv.className = 'result error';
                    resultDiv.innerHTML = `<h4>❌ Error</h4><p>${data.error}</p>`;
                    resultDiv.classList.remove('hidden');
                }
            } catch (error) {
                const resultDiv = document.getElementById('result');
                resultDiv.className = 'result error';
                resultDiv.innerHTML = `<h4>❌ Error</h4><p>${error.message}</p>`;
                resultDiv.classList.remove('hidden');
            }
        });

        updateNavbar();
    </script>
</body>
</html>
"#;
